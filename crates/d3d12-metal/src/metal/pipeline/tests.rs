use super::dxil_hk::psv_resource_handle_kinds;
use super::*;
use dxil_metal::root_signature::{
    flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
    RootParameter, RootParameterKind, RootSignatureDesc, RootSignatureVersion,
};

/// Builds a minimal valid DXBC container with an `SFI0` feature-flags part (`flags`).
fn fake_dxbc_with_sfi0(flags: u64) -> Vec<u8> {
    // Header (32 bytes): "DXBC" (4) + digest (16) + ver (4) + total_size (4) + part_count (4)
    // Part offset table (4 bytes): [36]
    // Part 0 (16 bytes): "SFI0" (4) + size=8 (4) + flags (8)
    let total_size: u32 = 32 + 4 + 16;
    let mut b = Vec::with_capacity(total_size as usize);
    b.extend_from_slice(b"DXBC");
    b.extend_from_slice(&[0u8; 16]);
    b.extend_from_slice(&1u32.to_le_bytes());
    b.extend_from_slice(&total_size.to_le_bytes());
    b.extend_from_slice(&1u32.to_le_bytes());
    b.extend_from_slice(&36u32.to_le_bytes());
    b.extend_from_slice(b"SFI0");
    b.extend_from_slice(&8u32.to_le_bytes());
    b.extend_from_slice(&flags.to_le_bytes());
    let sum = dxil_metal::hash::dxbc_checksum(&b[20..]);
    b[4..20].copy_from_slice(&sum);
    b
}

fn northlight_like_root_sig_with(with_static_sampler: bool) -> Arc<RootSignature> {
    // Mirrors AW2's compute GRS shape (`fillLightBufferDirectLightCS` / `transformCS`):
    // param 0: table [CBV b0..b9 (0..10), SRV t0..t50 (10..61), UAV u0..unbounded (61..)]
    // param 1: 32-bit inline root constants (b10)
    // param 2: root CBV (b11)
    // param 3: unbounded SRV table space1 (t0..unbounded)
    RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    DescriptorRange {
                        range_type: DescriptorRangeType::Cbv,
                        num_descriptors: 10,
                        base_shader_register: 0,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 0,
                    },
                    DescriptorRange {
                        range_type: DescriptorRangeType::Srv,
                        num_descriptors: 51,
                        base_shader_register: 0,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 10,
                    },
                    DescriptorRange {
                        range_type: DescriptorRangeType::Uav,
                        num_descriptors: rs_flags::UNBOUNDED,
                        base_shader_register: 0,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 61,
                    },
                ]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Constants {
                    shader_register: 10,
                    register_space: 0,
                    num_32bit_values: 4,
                },
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor {
                        shader_register: 11,
                        register_space: 0,
                        flags: 0,
                    },
                ),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: rs_flags::UNBOUNDED,
                    base_shader_register: 0,
                    register_space: 1,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
        ],
        static_samplers: if with_static_sampler {
            vec![dxil_metal::root_signature::StaticSampler::default()]
        } else {
            Vec::new()
        },
        flags: 0,
    })
    .unwrap()
}

#[test]
fn msc_compute_access_resolves_bounded_tables_and_enforces_heap_and_cbv_guards() {
    if std::env::var_os("AW2_MSC_COMPUTE_ACCESS").is_none() {
        assert!(!msc_compute_access(), "AW2_MSC_COMPUTE_ACCESS must default OFF");
    }
    let rs = northlight_like_root_sig_with(false);
    let dxil_no_heap = fake_dxbc_with_sfi0(0);

    // 1) `fillLightBufferDirectLightCS`-shaped shader: unbounded UAV/SRV tables in RS,
    //    all `"UsedResources"` entries have `tableLength: 1` (`ResourceUsages = 26`).
    let json_fill = r#"{
        "Features": 0,
        "ResourceUsages": 26,
        "UsedResources": [
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 10},
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 62},
            {"bindingIndex": 1, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
            {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
            {"bindingIndex": 3, "isInGRS": true, "tableLength": 1, "tableStartIndex": 6}
        ]
    }"#;
    let parsed = msc_used_resources(json_fill).unwrap();
    assert_eq!(parsed.len(), 6);
    let acc_fill = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_fill,
        Some(26),
    );
    assert!(acc_fill.msc_refined);
    assert_eq!(
        acc_fill.uav_slots,
        Some(vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 },
            ComputeBind::TableEntry { param: 0, byte_offset: 62 * 24 },
        ])
    );
    assert_eq!(
        acc_fill.read_slots,
        Some(vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 10 * 24 },
            ComputeBind::RootVa { param: 2 },
            ComputeBind::TableEntry { param: 3, byte_offset: 6 * 24 },
        ])
    );
    assert_eq!(acc_fill.uav_classes, READS_TEX);
    assert_eq!(acc_fill.read_classes, READS_TEX | READS_BUF);
    assert!(!acc_fill.heap_indexed_or_unknown);

    // 2) `transformCS`-shaped shader: bounded UAV (`61`) + unbounded buffer SRV (`param 3`), `RU = 6`.
    let json_skin = r#"{
        "Features": 0,
        "ResourceUsages": 6,
        "UsedResources": [
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 13},
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
            {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295},
            {"bindingIndex": 3, "isInGRS": true, "tableLength": 4294967295, "tableStartIndex": 0}
        ]
    }"#;
    let acc_skin = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_skin,
        Some(6),
    );
    assert!(acc_skin.msc_refined);
    assert_eq!(
        acc_skin.uav_slots,
        Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
    );
    assert_eq!(acc_skin.read_slots, None);
    assert_eq!(
        acc_skin.bounded_read_slots,
        vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 13 * 24 },
            ComputeBind::RootVa { param: 2 },
        ]
    );
    assert_eq!(acc_skin.unbounded_srv_classes, READS_BUF);
    assert!(!acc_skin.touches_textures);
    assert_eq!(acc_skin.uav_classes, READS_BUF);
    assert_eq!(acc_skin.read_classes, READS_BUF);

    // 3) SM 6.6 `RESOURCE_HEAP_INDEXING` (`1 << 25`): MUST reject Fix 1 (`uav_slots == None`
    //    when UAVs exist) while still refining `uav_classes` via Fix 2 (`RU = 16` -> `READS_TEX`).
    let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
    let acc_heap = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_heap,
        &rs,
        json_fill,
        Some(16),
    );
    assert!(!acc_heap.msc_refined);
    assert_eq!(acc_heap.uav_slots, None);
    assert_eq!(acc_heap.read_slots, None);
    assert_eq!(acc_heap.uav_classes, READS_TEX);
    assert_eq!(acc_heap.read_classes, READS_TEX | READS_BUF);

    // 4) `isInGRS == false`: rejects Fix 1 (`read_slots == None`), while `RU = 8` (`TextureRead` only,
    //    zero UAVs) proves `uav_slots == Some(vec![])` and `uav_classes == 0`.
    let json_non_grs = r#"{
        "ResourceUsages": 8,
        "UsedResources": [
            {"bindingIndex": 0, "isInGRS": false, "tableLength": 1, "tableStartIndex": 10}
        ]
    }"#;
    let acc_non_grs = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_non_grs,
        Some(8),
    );
    assert_eq!(acc_non_grs.read_slots, None);
    assert_eq!(acc_non_grs.uav_slots, Some(Vec::new()));
    assert_eq!(acc_non_grs.uav_classes, 0);

    // 5) CBV-only `ResourceUsages == 0` soundness: root CBV (`param 2`) keeps `READS_BUF` in
    //    `read_classes`, whereas inline 32-bit root constants (`param 1`) alone have `read_classes == 0`.
    let json_root_cbv = r#"{
        "ResourceUsages": 0,
        "UsedResources": [
            {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
        ]
    }"#;
    let acc_cbv = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_root_cbv,
        Some(0),
    );
    assert_eq!(acc_cbv.uav_classes, 0);
    assert_eq!(acc_cbv.read_classes, READS_BUF, "root CBV resource must retain READS_BUF");

    let json_inline_const = r#"{
        "ResourceUsages": 0,
        "UsedResources": [
            {"bindingIndex": 1, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
        ]
    }"#;
    let acc_const = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_inline_const,
        Some(0),
    );
    assert_eq!(acc_const.uav_classes, 0);
    assert_eq!(acc_const.read_classes, 0);

    // 6) Static-sampler TLAB slot (`bindingIndex == parameters.len() == 4`):
    //    rejected when `static_samplers` is empty, accepted as stateless sampler table when non-empty.
    let json_static_smp = r#"{
        "ResourceUsages": 12,
        "UsedResources": [
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
            {"bindingIndex": 4, "isInGRS": true, "tableLength": 1, "tableStartIndex": 0}
        ]
    }"#;
    let acc_no_smp = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs,
        json_static_smp,
        Some(12),
    );
    assert!(!acc_no_smp.msc_refined);
    assert_eq!(acc_no_smp.uav_slots, None);

    let rs_with_smp = northlight_like_root_sig_with(true);
    let acc_with_smp = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &dxil_no_heap,
        &rs_with_smp,
        json_static_smp,
        Some(12),
    );
    assert!(acc_with_smp.msc_refined);
    assert_eq!(
        acc_with_smp.uav_slots,
        Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
    );
    assert_eq!(acc_with_smp.read_slots, Some(Vec::new()));
}

#[test]
fn g40_air_no_res_heap_and_stage_usage_refinement() {
    if std::env::var_os("AW2_MSC_STAGE_USAGE").is_none() {
        assert!(!msc_stage_usage(), "AW2_MSC_STAGE_USAGE must default OFF");
    }
    if std::env::var_os("AW2_AIR_NO_RES_HEAP").is_none() {
        assert!(!air_no_res_heap(), "AW2_AIR_NO_RES_HEAP must default OFF");
    }

    let build_mock_metallib = |magic: &[u8; 4],
                               include_rbuf: bool,
                               unused_byte: u8,
                               loc_idx: u32,
                               dup_name: bool|
     -> Vec<u8> {
        let mut m = Vec::new();
        m.extend_from_slice(magic);
        m.extend_from_slice(&[0u8; 32]);
        if include_rbuf {
            m.extend_from_slice(b"RBUF");
            m.extend_from_slice(&[0u8; 16]);
        }
        m.extend_from_slice(&[0x01, 0x02, unused_byte]);
        m.extend_from_slice(&loc_idx.to_le_bytes());
        m.extend_from_slice(&1u32.to_le_bytes());
        m.extend_from_slice(&0x1234_5678u32.to_le_bytes()); // vtable soffset_t
        m.extend_from_slice(&8u32.to_le_bytes());
        m.extend_from_slice(&4u32.to_le_bytes());
        m.extend_from_slice(&23u32.to_le_bytes());
        m.extend_from_slice(b"struct.res_desc_heap_ab\0");
        if dup_name {
            m.extend_from_slice(b"struct.res_desc_heap_ab\0");
        }
        m
    };

    assert!(metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 0, false)));
    assert!(
        !metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 0, 0, false)),
        "unused==0 (active ResourceDescriptorHeap) MUST fail closed"
    );
    assert!(
        !metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 2, false)),
        "unexpected location_index MUST fail closed"
    );
    assert!(
        !metallib_res_desc_heap_unused(&build_mock_metallib(b"XXXX", true, 1, 0, false)),
        "non-MTLB magic MUST fail closed"
    );
    assert!(
        !metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", false, 1, 0, false)),
        "missing RBUF section MUST fail closed"
    );
    assert!(
        !metallib_res_desc_heap_unused(&build_mock_metallib(b"MTLB", true, 1, 0, true)),
        "duplicate res_desc_heap_ab symbol MUST fail closed"
    );

    let rs = northlight_like_root_sig_with(false);
    let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
    let json_tex_cs = r#"{
        "Features": 0,
        "ResourceUsages": 24,
        "UsedResources": [
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 10},
            {"bindingIndex": 0, "isInGRS": true, "tableLength": 1, "tableStartIndex": 61},
            {"bindingIndex": 2, "isInGRS": true, "tableLength": 1, "tableStartIndex": 4294967295}
        ]
    }"#;

    // Stage B (`enable_used_resources = false`): class masks narrow (`uav_srv_classes == READS_TEX`)
    // while `read_classes` preserves `READS_BUF` for root CBV and `uav_slots` stays `None` on SM 6.6.
    let acc_stage_b = ComputeAccess::refine_with_msc_full(
        ComputeAccess::opaque(),
        &dxil_heap,
        &rs,
        json_tex_cs,
        Some(24),
        false,
        false,
    );
    assert_eq!(acc_stage_b.uav_slots, None);
    assert_eq!(acc_stage_b.uav_classes, READS_TEX);
    assert_eq!(
        acc_stage_b.uav_srv_classes, READS_TEX,
        "uav_srv_classes must exclude CBV-only READS_BUF"
    );
    assert_eq!(
        acc_stage_b.read_classes,
        READS_TEX | READS_BUF,
        "read_classes must retain READS_BUF for CBV"
    );

    // Stage D (`enable_used_resources = true, ignore_heap_flag = true`): SM 6.6 `SFI0`
    // `RESOURCE_HEAP_INDEXING` is bypassed when AIR proves `res_desc_heap_ab` is unused!
    let acc_stage_d = ComputeAccess::refine_with_msc_full(
        ComputeAccess::opaque(),
        &dxil_heap,
        &rs,
        json_tex_cs,
        Some(24),
        true,
        true,
    );
    assert!(acc_stage_d.msc_refined);
    assert_eq!(
        acc_stage_d.uav_slots,
        Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 61 * 24 }])
    );
    assert_eq!(
        acc_stage_d.read_slots,
        Some(vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 10 * 24 },
            ComputeBind::RootVa { param: 2 },
        ])
    );
    assert_eq!(acc_stage_d.uav_classes, READS_TEX);
    assert_eq!(acc_stage_d.uav_srv_classes, READS_TEX);
    assert_eq!(acc_stage_d.read_classes, READS_TEX | READS_BUF);
    assert_eq!(acc_stage_d.msc_exact_classes, READS_TEX | READS_BUF);
}

#[test]
fn g42_msc_gfx_cbv_and_cs_msc_exact_classes_distinguish_inline_constants_from_buffer_cbvs() {
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };

    // Root signature with inline constants at b0 (param 0 -> bindingIndex 0),
    // root CBV at b1 (param 1 -> bindingIndex 1), and SRV/UAV table at param 2 (bindingIndex 2).
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Constants {
                    shader_register: 0,
                    register_space: 0,
                    num_32bit_values: 4,
                },
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor {
                        shader_register: 1,
                        register_space: 0,
                        flags: 0,
                    },
                ),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    DescriptorRange {
                        range_type: DescriptorRangeType::Srv,
                        num_descriptors: rs_flags::UNBOUNDED,
                        base_shader_register: 0,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 0,
                    },
                    DescriptorRange {
                        range_type: DescriptorRangeType::Cbv,
                        num_descriptors: 2,
                        base_shader_register: 2,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start: 16,
                    },
                ]),
                visibility: 0,
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED,
    })
    .unwrap();

    let make_mtlb = |unused: u8| -> Vec<u8> {
        let mut m = Vec::new();
        m.extend_from_slice(b"MTLB");
        m.extend_from_slice(&[0u8; 32]);
        m.extend_from_slice(b"RBUF");
        m.extend_from_slice(&[0u8; 16]);
        m.extend_from_slice(&[0x01, 0x02, unused]);
        m.extend_from_slice(&0u32.to_le_bytes());
        m.extend_from_slice(&1u32.to_le_bytes());
        m.extend_from_slice(&0x1234_5678u32.to_le_bytes());
        m.extend_from_slice(&8u32.to_le_bytes());
        m.extend_from_slice(&4u32.to_le_bytes());
        m.extend_from_slice(&23u32.to_le_bytes());
        m.extend_from_slice(b"struct.res_desc_heap_ab\0");
        m
    };
    let mtlb_unused = make_mtlb(1);
    let mtlb_active = make_mtlb(0);

    // 1) SM 6.6 shader touching ONLY inline constants at `bindingIndex: 0` + texture SRV (`tableStartIndex: 5`):
    //    `msc_stage_may_read_cbv_from_reflection` MUST return `Some(false)` when `res_desc_heap_ab` is unused
    //    (and `None` when active!), and `msc_exact_classes` MUST be `READS_TEX`!
    let json_inline_const_tex = r#"{"ResourceUsages":8,"UsedResources":[{"bindingIndex":0,"isInGRS":true,"tableLength":1,"tableStartIndex":0},{"bindingIndex":2,"isInGRS":true,"tableLength":1,"tableStartIndex":5}]}"#;
    assert_eq!(
        super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_inline_const_tex, &rs),
        Some(false),
        "inline root constants + texture SRV table must not report CBV buffer read"
    );
    assert_eq!(
        super::msc_stage_may_read_cbv_from_reflection(&mtlb_active, json_inline_const_tex, &rs),
        None,
        "active ResourceDescriptorHeap must fail closed (None)"
    );
    let dxil_heap = fake_dxbc_with_sfi0(1u64 << 25);
    assert_eq!(
        ComputeAccess::exact_classes_from_metallib(
            &mtlb_unused,
            json_inline_const_tex,
            8,
            true,
            &rs
        ),
        READS_TEX,
        "unused res_desc_heap_ab + inline constants must narrow msc_exact_classes to READS_TEX"
    );
    assert_eq!(
        ComputeAccess::exact_classes_from_metallib(
            &mtlb_active,
            json_inline_const_tex,
            8,
            true,
            &rs
        ),
        READS_TEX | READS_BUF,
        "active res_desc_heap_ab must fail closed and retain READS_BUF"
    );
    let mut init_tex = ComputeAccess::opaque();
    init_tex.msc_exact_classes = ComputeAccess::exact_classes_from_metallib(
        &mtlb_unused,
        json_inline_const_tex,
        8,
        true,
        &rs,
    );
    let acc_tex_only = ComputeAccess::refine_with_msc_full(
        init_tex,
        &dxil_heap,
        &rs,
        json_inline_const_tex,
        Some(8),
        false,
        false,
    );
    // Even with `enable_used_resources = false` (zero salt!), `msc_exact_classes` stays `READS_TEX` (`!READS_BUF`).
    assert_eq!(acc_tex_only.read_classes, READS_TEX | READS_BUF);
    assert_eq!(acc_tex_only.msc_exact_classes, READS_TEX);

    // 2) Shader touching root CBV at `bindingIndex: 1` -> `Some(true)` & `msc_exact_classes` includes `READS_BUF`.
    let json_root_cbv = r#"{"ResourceUsages":0,"UsedResources":[{"bindingIndex":1,"isInGRS":true,"tableLength":1,"tableStartIndex":4294967295}]}"#;
    assert_eq!(
        super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_root_cbv, &rs),
        Some(true)
    );
    let mut init_cbv = ComputeAccess::opaque();
    init_cbv.msc_exact_classes =
        ComputeAccess::exact_classes_from_metallib(&mtlb_unused, json_root_cbv, 0, true, &rs);
    let acc_root_cbv = ComputeAccess::refine_with_msc_full(
        init_cbv,
        &dxil_heap,
        &rs,
        json_root_cbv,
        Some(0),
        false,
        false,
    );
    assert_eq!(acc_root_cbv.msc_exact_classes, READS_BUF);

    // 3) Shader touching CBV descriptor table entry (`bindingIndex: 2`, `tableStartIndex: 17` inside `[16, 18)`) -> `Some(true)`.
    let json_tbl_cbv = r#"{"ResourceUsages":0,"UsedResources":[{"bindingIndex":2,"isInGRS":true,"tableLength":1,"tableStartIndex":17}]}"#;
    assert_eq!(
        super::msc_stage_may_read_cbv_from_reflection(&mtlb_unused, json_tbl_cbv, &rs),
        Some(true)
    );
}

fn build_synth_dxil_with_annotate(props: &[(u32, u32)], fn_names: &[&str]) -> Vec<u8> {
    #[derive(Default)]
    struct Tw {
        out: Vec<u8>,
        bit: usize,
    }
    impl Tw {
        fn put(&mut self, v: u64, n: u32) {
            for i in 0..n {
                if self.bit.is_multiple_of(8) {
                    self.out.push(0);
                }
                if (v >> i) & 1 == 1 {
                    *self.out.last_mut().unwrap() |= 1 << (self.bit % 8);
                }
                self.bit += 1;
            }
        }
        fn vbr(&mut self, mut v: u64, n: u32) {
            let hi = 1u64 << (n - 1);
            loop {
                if v < hi {
                    self.put(v, n);
                    return;
                }
                self.put((v & (hi - 1)) | hi, n);
                v >>= n - 1;
            }
        }
        fn align(&mut self) {
            while !self.bit.is_multiple_of(32) {
                self.put(0, 1);
            }
        }
        fn enter(&mut self, w: u32, id: u64, new_w: u32) -> usize {
            self.put(1, w);
            self.vbr(id, 8);
            self.vbr(new_w as u64, 4);
            self.align();
            let at = self.out.len();
            self.put(0, 32);
            at
        }
        fn end(&mut self, w: u32, len_at: usize) {
            self.put(0, w);
            self.align();
            let words = ((self.out.len() - len_at - 4) / 4) as u32;
            self.out[len_at..len_at + 4].copy_from_slice(&words.to_le_bytes());
        }
        fn rec(&mut self, w: u32, code: u64, ops: &[u64]) {
            self.put(3, w);
            self.vbr(code, 6);
            self.vbr(ops.len() as u64, 6);
            for &o in ops {
                self.vbr(o, 6);
            }
        }
    }

    let mut w = Tw::default();
    w.out.extend_from_slice(b"BC\xC0\xDE");
    w.bit = 32;
    let m_at = w.enter(2, 8, 4);

    // TYPE_BLOCK_NEW (17):
    // tid 0: void (2)
    // tid 1: i32 (7, [32])
    // tid 2: %dx.types.Handle (20, [0, ...])
    // tid 3: %dx.types.ResourceProperties (20, [0, 1, 1])
    // tid 4: void() (21, [0, 0])
    // tid 5: Handle(i32, Handle, ResourceProperties) (21, [0, 2, 1, 2, 3])
    let t_at = w.enter(4, 17, 4);
    w.rec(4, 1, &[6]);
    w.rec(4, 2, &[]);
    w.rec(4, 7, &[32]);
    let h_nm: Vec<u64> = b"dx.types.Handle".iter().map(|&c| c as u64).collect();
    w.rec(4, 19, &h_nm);
    w.rec(4, 20, &[0, 1]);
    let rp_nm: Vec<u64> = b"dx.types.ResourceProperties".iter().map(|&c| c as u64).collect();
    w.rec(4, 19, &rp_nm);
    w.rec(4, 20, &[0, 1, 1]);
    w.rec(4, 21, &[0, 0]);
    w.rec(4, 21, &[0, 2, 1, 2, 3]);
    w.end(4, t_at);

    // vid 0: entry `main` (isproto = 0, ty = 4)
    w.rec(4, 8, &[4, 0, 0, 0]);
    // vid 1..: declared handle-returning functions (isproto = 1, ty = 5)
    for _ in fn_names {
        w.rec(4, 8, &[5, 0, 1, 0]);
    }

    // VALUE_SYMTAB_BLOCK (14)
    let v_at = w.enter(4, 14, 4);
    let mut main_ops = vec![0u64];
    main_ops.extend(b"main".iter().map(|&c| c as u64));
    w.rec(4, 1, &main_ops);
    for (i, nm) in fn_names.iter().enumerate() {
        let mut ops = vec![(i + 1) as u64];
        ops.extend(nm.bytes().map(|c| c as u64));
        w.rec(4, 1, &ops);
    }
    w.end(4, v_at);

    // FUNCTION_BLOCK (12) for `main` with function-local CONSTANTS_BLOCK (11)
    let f_at = w.enter(4, 12, 4);
    w.rec(4, 1, &[1]); // DECLAREBLOCKS
    let c_at = w.enter(4, 11, 4);
    let base_vid = 1 + fn_names.len() as u64;
    w.rec(4, 1, &[1]); // SETTYPE i32
    for &(d0, d1) in props {
        w.rec(4, 4, &[(d0 as u64) << 1]);
        w.rec(4, 4, &[(d1 as u64) << 1]);
    }
    w.rec(4, 1, &[3]); // SETTYPE %dx.types.ResourceProperties
    for (k, _) in props.iter().enumerate() {
        let v0 = base_vid + (k as u64) * 2;
        let v1 = v0 + 1;
        w.rec(4, 7, &[v0, v1]); // CST_CODE_AGGREGATE
    }
    w.end(4, c_at);
    w.rec(4, 10, &[]); // RET void
    w.end(4, f_at);

    w.end(2, m_at);

    // Wrap bitcode in a DXIL part + DXBC container
    let bc = w.out;
    let mut dxil_part = Vec::with_capacity(24 + bc.len());
    dxil_part.extend_from_slice(&0x0005_0066u32.to_le_bytes()); // version (CS 6.6)
    dxil_part.extend_from_slice(&(((24 + bc.len()) / 4) as u32).to_le_bytes());
    dxil_part.extend_from_slice(b"DXIL");
    dxil_part.extend_from_slice(&0x0000_0106u32.to_le_bytes());
    dxil_part.extend_from_slice(&16u32.to_le_bytes()); // off from +8 -> 24
    dxil_part.extend_from_slice(&(bc.len() as u32).to_le_bytes());
    dxil_part.extend_from_slice(&bc);

    let total_size = (32 + 4 + 8 + dxil_part.len()) as u32;
    let mut dxbc = Vec::with_capacity(total_size as usize);
    dxbc.extend_from_slice(b"DXBC");
    dxbc.extend_from_slice(&[0u8; 16]);
    dxbc.extend_from_slice(&1u32.to_le_bytes());
    dxbc.extend_from_slice(&total_size.to_le_bytes());
    dxbc.extend_from_slice(&1u32.to_le_bytes());
    dxbc.extend_from_slice(&36u32.to_le_bytes());
    dxbc.extend_from_slice(b"DXIL");
    dxbc.extend_from_slice(&(dxil_part.len() as u32).to_le_bytes());
    dxbc.extend_from_slice(&dxil_part);
    let sum = dxil_metal::hash::dxbc_checksum(&dxbc[20..]);
    dxbc[4..20].copy_from_slice(&sum);
    dxbc
}

#[test]
fn g50_dxil_annotate_handle_kinds_flt_vs_int_and_fail_closed() {
    use super::super::resource::*;

    // 1) SM 6.6 shader with Texture2D<float4> SRV (kind=2, comp=F32=9),
    //    StructuredBuffer SRV (kind=12), and RWTexture3D<float4> UAV (kind=4 | (1<<12), comp=F32=9).
    let dxil_flt = build_synth_dxil_with_annotate(
        &[(2, 9), (12, 16), (4 | (1 << 12), 9)],
        &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
    );
    let (srv_f, uav_f) = dxil_annotate_handle_kinds(&dxil_flt);
    assert_eq!(srv_f, HK_TEX2D_FLT | HK_STRUCT_BUF);
    assert_eq!(uav_f, HK_TEX3D_FLT);
    assert_eq!(srv_f & HK_TEX2D_INT, 0, "float Texture2D must not match uint Texture2D");
    assert_eq!(srv_f & HK_RAW_BUF, 0, "StructuredBuffer must not match RawBuffer");

    // 2) SM 6.6 shader with Texture2D<uint4> SRV (kind=2, comp=U32=5) and RWByteAddressBuffer UAV (kind=11 | (1<<12)).
    let dxil_int = build_synth_dxil_with_annotate(
        &[(2, 5), (11 | (1 << 12), 0)],
        &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
    );
    let (srv_i, uav_i) = dxil_annotate_handle_kinds(&dxil_int);
    assert_eq!(srv_i, HK_TEX2D_INT);
    assert_eq!(uav_i, HK_RAW_BUF);
    assert_eq!(srv_i & HK_TEX2D_FLT, 0, "uint Texture2D must not match float Texture2D");

    // 3) Unknown handle-producing function fails closed to (HK_ALL, HK_ALL & !HK_CBV_BUF).
    let dxil_unknown = build_synth_dxil_with_annotate(
        &[(2, 9)],
        &["dx.op.annotateHandle", "dx.op.unknownMakeHandle"],
    );
    assert_eq!(
        dxil_annotate_handle_kinds(&dxil_unknown),
        (HK_ALL, HK_ALL & !HK_CBV_BUF),
        "unknown handle-returning intrinsic must fail closed"
    );
}

#[test]
fn g51_dxil_cst_data_undef_and_cache_hit_parity() {
    use super::super::resource::*;

    let mut raw = dxil_metal::dxbc::PsvResource {
        res_type: 4,
        space: 0,
        lower_bound: 0,
        upper_bound: u32::MAX,
        kind: 0,
    };
    assert_eq!(psv_resource_handle_kinds(&[raw]), (HK_RAW_BUF | HK_ACCEL_STRUCT, 0));
    raw.kind = 16;
    assert_eq!(psv_resource_handle_kinds(&[raw]), (HK_ACCEL_STRUCT, 0));

    // 1) `createHandleFromHeap` WITHOUT `annotateHandle` MUST fail closed (`None`).
    let dxil_unannotated = build_synth_dxil_with_annotate(&[], &["dx.op.createHandleFromHeap"]);
    assert_eq!(
        dxil_annotate_handle_kinds_opt(&dxil_unannotated),
        None,
        "SM 6.6 createHandleFromHeap without annotateHandle must fail closed"
    );

    assert_eq!(
        dxil_annotate_handle_kinds_opt(&build_synth_dxil_with_annotate(
            &[],
            &["dx.op.createHandleFromHeap", "dx.op.annotateHandle"],
        )),
        None,
        "an annotateHandle declaration alone cannot prove the set of resource kinds",
    );

    // 2) `CST_CODE_DATA = 22` + `VST_CODE_FNENTRY = 3` + multiple module `CONSTANTS_BLOCK`s
    //    must parse cleanly, whereas `CST_CODE_UNDEF = 3` on `%dx.types.ResourceProperties`
    //    must fail closed (`None`).
    let build_variant = |use_undef: bool| -> Vec<u8> {
        #[derive(Default)]
        struct Tw {
            out: Vec<u8>,
            bit: usize,
        }
        impl Tw {
            fn put(&mut self, v: u64, n: u32) {
                for i in 0..n {
                    if self.bit.is_multiple_of(8) {
                        self.out.push(0);
                    }
                    if (v >> i) & 1 == 1 {
                        *self.out.last_mut().unwrap() |= 1 << (self.bit % 8);
                    }
                    self.bit += 1;
                }
            }
            fn vbr(&mut self, mut v: u64, n: u32) {
                let hi = 1u64 << (n - 1);
                loop {
                    if v < hi {
                        self.put(v, n);
                        return;
                    }
                    self.put((v & (hi - 1)) | hi, n);
                    v >>= n - 1;
                }
            }
            fn align(&mut self) {
                while !self.bit.is_multiple_of(32) {
                    self.put(0, 1);
                }
            }
            fn enter(&mut self, w: u32, id: u64, new_w: u32) -> usize {
                self.put(1, w);
                self.vbr(id, 8);
                self.vbr(new_w as u64, 4);
                self.align();
                let at = self.out.len();
                self.put(0, 32);
                at
            }
            fn end(&mut self, w: u32, len_at: usize) {
                self.put(0, w);
                self.align();
                let words = ((self.out.len() - len_at - 4) / 4) as u32;
                self.out[len_at..len_at + 4].copy_from_slice(&words.to_le_bytes());
            }
            fn rec(&mut self, w: u32, code: u64, ops: &[u64]) {
                self.put(3, w);
                self.vbr(code, 6);
                self.vbr(ops.len() as u64, 6);
                for &o in ops {
                    self.vbr(o, 6);
                }
            }
        }

        let mut w = Tw::default();
        w.out.extend_from_slice(b"BC\xC0\xDE");
        w.bit = 32;
        let m_at = w.enter(2, 8, 4);

        let t_at = w.enter(4, 17, 4);
        w.rec(4, 1, &[6]);
        w.rec(4, 2, &[]);
        w.rec(4, 7, &[32]);
        let h_nm: Vec<u64> = b"dx.types.Handle".iter().map(|&c| c as u64).collect();
        w.rec(4, 19, &h_nm);
        w.rec(4, 20, &[0, 1]);
        let rp_nm: Vec<u64> =
            b"dx.types.ResourceProperties".iter().map(|&c| c as u64).collect();
        w.rec(4, 19, &rp_nm);
        w.rec(4, 20, &[0, 1, 1]);
        w.rec(4, 21, &[0, 0]);
        w.rec(4, 21, &[0, 2, 1, 2, 3]);
        w.end(4, t_at);

        w.rec(4, 8, &[4, 0, 0, 0]); // vid 0: main
        w.rec(4, 8, &[5, 0, 1, 0]); // vid 1: createHandleFromHeap
        w.rec(4, 8, &[5, 0, 1, 0]); // vid 2: annotateHandle

        let v_at = w.enter(4, 14, 4);
        w.rec(4, 2, &[1, b'b' as u64, b'b' as u64]); // BB ID 1 is independent of value ID 1.
        for (vid, nm) in [
            (0u64, "main"),
            (1, "dx.op.createHandleFromHeap"),
            (2, "dx.op.annotateHandle"),
        ] {
            let code = if vid == 0 { 3 } else { 1 };
            let mut ops = if vid == 0 { vec![vid, 0] } else { vec![vid] };
            ops.extend(nm.bytes().map(|c| c as u64));
            w.rec(4, code, &ops);
        }
        w.end(4, v_at);

        // Two module CONSTANTS_BLOCKs: first holds i32, second holds CST_CODE_DATA=22 (or UNDEF=3).
        let mc1 = w.enter(4, 11, 4);
        w.rec(4, 1, &[1]);
        w.rec(4, 4, &[42 << 1]);
        w.end(4, mc1);

        let mc2 = w.enter(4, 11, 4);
        w.rec(4, 1, &[3]); // SETTYPE %dx.types.ResourceProperties
        if use_undef {
            w.rec(4, 3, &[]); // CST_CODE_UNDEF = 3 -> MUST fail closed
        } else {
            // CST_CODE_DATA = 22: Texture3D<float4> SRV (kind=4, comp=9)
            w.rec(4, 22, &[4, 9]);
        }
        w.end(4, mc2);

        let f_at = w.enter(4, 12, 4);
        w.rec(4, 1, &[1]);
        let fc = w.enter(4, 11, 4);
        w.rec(4, 1, &[3]);
        // Function-local CST_CODE_DATA = 22: RWStructuredBuffer UAV (kind=12 | (1<<12))
        w.rec(4, 22, &[12 | (1 << 12), 0]);
        w.end(4, fc);
        w.rec(4, 10, &[]);
        w.end(4, f_at);
        w.end(2, m_at);

        let bc = w.out;
        let mut dxil_part = Vec::with_capacity(24 + bc.len());
        dxil_part.extend_from_slice(&0x0005_0066u32.to_le_bytes());
        dxil_part.extend_from_slice(&(((24 + bc.len()) / 4) as u32).to_le_bytes());
        dxil_part.extend_from_slice(b"DXIL");
        dxil_part.extend_from_slice(&0x0000_0106u32.to_le_bytes());
        dxil_part.extend_from_slice(&16u32.to_le_bytes());
        dxil_part.extend_from_slice(&(bc.len() as u32).to_le_bytes());
        dxil_part.extend_from_slice(&bc);
        let total_size = (32 + 4 + 8 + dxil_part.len()) as u32;
        let mut dxbc = Vec::with_capacity(total_size as usize);
        dxbc.extend_from_slice(b"DXBC");
        dxbc.extend_from_slice(&[0u8; 16]);
        dxbc.extend_from_slice(&1u32.to_le_bytes());
        dxbc.extend_from_slice(&total_size.to_le_bytes());
        dxbc.extend_from_slice(&1u32.to_le_bytes());
        dxbc.extend_from_slice(&36u32.to_le_bytes());
        dxbc.extend_from_slice(b"DXIL");
        dxbc.extend_from_slice(&(dxil_part.len() as u32).to_le_bytes());
        dxbc.extend_from_slice(&dxil_part);
        let sum = dxil_metal::hash::dxbc_checksum(&dxbc[20..]);
        dxbc[4..20].copy_from_slice(&sum);
        dxbc
    };

    assert_eq!(
        dxil_annotate_handle_kinds_opt(&build_variant(false)),
        Some((HK_TEX3D_FLT, HK_STRUCT_BUF))
    );
    assert_eq!(
        dxil_annotate_handle_kinds_opt(&build_variant(true)),
        None,
        "CST_CODE_UNDEF on ResourceProperties must fail closed"
    );

    // 3) `.aw2s` cold-compile MISS vs warm-cache HIT `HandleKindProfile` parity on a real
    //    compiled shader fixture.
    if super::super::device().is_some() && dxil_metal::converter_available() {
        let raw_cs = include_bytes!("../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");
        let mut cs_dxil = raw_cs.to_vec();
        let sum = dxil_metal::hash::dxbc_checksum(&cs_dxil[20..]);
        cs_dxil[4..20].copy_from_slice(&sum);
        let cs_rs = RootSignature::new(RootSignatureDesc::covering_dxil(&cs_dxil).unwrap()).unwrap();
        let tmp_dir =
            std::env::temp_dir().join(format!("aw2_g51_hk_cache_parity_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        let pipe = dxil_metal::ShaderPipeline::new(dxil_metal::CacheConfig {
            disk_dir: Some(tmp_dir.clone()),
            memory_limit_bytes: 16 * 1024 * 1024,
        })
        .unwrap();
        let sh_miss = pipe
            .compile_stage(
                dxil_metal::ShaderStage::Compute,
                &cs_dxil,
                Some(&cs_rs),
                &dxil_metal::CompileOptions::default(),
            )
            .unwrap();
        let pipe_warm = dxil_metal::ShaderPipeline::new(dxil_metal::CacheConfig {
            disk_dir: Some(tmp_dir.clone()),
            memory_limit_bytes: 16 * 1024 * 1024,
        })
        .unwrap();
        let sh_hit = pipe_warm
            .compile_stage(
                dxil_metal::ShaderStage::Compute,
                &cs_dxil,
                Some(&cs_rs),
                &dxil_metal::CompileOptions::default(),
            )
            .unwrap();
        assert_eq!(
            pipe_warm.cache().stats().disk_hits,
            1,
            "second pipeline must hit .aw2s disk cache"
        );
        let acc_miss = ComputeAccess::from_dxil_and_shader(&cs_dxil, &cs_rs, &sh_miss);
        let prof_miss = build_handle_kind_profile(
            &cs_dxil,
            &cs_rs,
            &sh_miss,
            msc_resource_usages(&sh_miss),
            &acc_miss,
        );
        let acc_hit = ComputeAccess::from_dxil_and_shader(&cs_dxil, &cs_rs, &sh_hit);
        let prof_hit = build_handle_kind_profile(
            &cs_dxil,
            &cs_rs,
            &sh_hit,
            msc_resource_usages(&sh_hit),
            &acc_hit,
        );
        assert_eq!(
            prof_miss, prof_hit,
            ".aw2s disk cache HIT and MISS must produce identical HandleKindProfile"
        );
        assert_eq!(
            prof_hit.ro_srv_kinds, HK_TEX2D_FLT,
            "static_samplers.cs.dxil Texture2D<float4> SRV must narrow ro_srv_kinds to HK_TEX2D_FLT (not HK_TEX2D_INT)"
        );
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}

#[test]
fn g51_real_northlight_binrfx_and_pe_dxil_handle_kinds() {
    use super::super::resource::*;
    use std::path::PathBuf;

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        std::env::var_os("AW2_GAME_DIR").map(PathBuf::from),
        Some(manifest.join("../../Alan Wake 2.app/Contents/Resources/game")),
        Some(manifest.join("../../../../Alan Wake 2.app/Contents/Resources/game")),
        Some(manifest.join("../../Alan Wake 2.app/Contents/Resources/Alan Wake 2")),
        Some(manifest.join("../../../../Alan Wake 2.app/Contents/Resources/Alan Wake 2")),
    ];
    let Some(game_dir) = candidates
        .into_iter()
        .flatten()
        .find(|p| p.join("data/shaders/build/pc_dx12").is_dir())
    else {
        assert!(
            std::env::var_os("AW2_GAME_DIR").is_none(),
            "AW2_GAME_DIR has no Northlight shader directory"
        );
        eprintln!("skipping real DXIL corpus: no local game installation");
        return;
    };

    let rfx_dir = game_dir.join("data/shaders/build/pc_dx12");
    let mut rfx_total = 0;
    let mut rfx_proven = 0;
    let mut pe_total = 0;
    let mut pe_proven = 0;
    let mut sm66_proven = 0;
    let mut skin = false;
    let mut depth = false;
    let mut framegen = false;
    for path in dxil_metal::rfx::list_rfx_bundles(&rfx_dir, None).unwrap() {
        let bundle = dxil_metal::rfx::RfxBundle::open(path).unwrap();
        for prog in bundle.programs {
            rfx_total += 1;
            let detail = dxil_handle_kinds_detail_opt(&prog.dxbc_bytes);
            rfx_proven += usize::from(detail.as_ref().is_some_and(|d| d.srv_mask != HK_ALL));
            // Expected masks from independent LLVM disassembly of the shipped shader.
            match (bundle.file_name.as_str(), prog.entry_point.as_str()) {
                ("skinning.binrfx", "transformCS") if !skin => {
                    let d = detail.unwrap();
                    assert_eq!(d.srv_mask, HK_RAW_STRUCT_BUF | HK_CBV_BUF);
                    assert_eq!(d.uav_mask, HK_RAW_BUF);
                    assert!(d.srv_ranges.contains(&DxilSrvRangeHk {
                        space: 2,
                        lower_bound: 0,
                        upper_bound: u32::MAX,
                        hk: HK_RAW_BUF,
                    }));
                    skin = true;
                }
                ("converttolineardepth.binrfx", "convert_to_linear") if !depth => {
                    let d = detail.unwrap();
                    assert_eq!(d.srv_mask, HK_TEX2D_FLT | HK_CBV_BUF);
                    assert_eq!(d.uav_mask, 0);
                    depth = true;
                }
                _ => {}
            }
        }
    }
    for path in dxil_metal::pe::list_pe_binaries(&game_dir, None).unwrap() {
        let bytes = std::fs::read(&path).unwrap();
        let name = path.file_name().unwrap().to_string_lossy();
        for emb in dxil_metal::pe::extract_pe_shaders(name.as_ref(), &bytes) {
            if emb.is_sm4_5 {
                continue;
            }
            let c = dxil_metal::dxbc::Container::parse(&emb.dxbc_bytes).unwrap();
            if c.part(dxil_metal::dxbc::FOURCC_DXIL).is_none() {
                continue;
            }
            pe_total += 1;
            let profile = dxil_annotate_handle_kinds_opt(&emb.dxbc_bytes);
            let precise = profile.is_some_and(|(srv, _)| srv != HK_ALL);
            pe_proven += usize::from(precise);
            sm66_proven +=
                usize::from(precise && c.program().is_some_and(|p| (p.major, p.minor) >= (6, 6)));
            if name == "amd_fidelityfx_framegeneration_dx12.dll" && emb.file_offset == 0x2384af0 {
                // SM6.6 annotateHandle constants have dword1 = 265/521 (SRV F32)
                // and 261 (UAV U32). The same shader also binds a CBV and a sampler.
                assert_eq!(profile, Some((HK_TEX2D_FLT | HK_CBV_BUF, HK_TEX2D_INT)));
                framegen = true;
            }
        }
    }
    eprintln!(
        "g51 DXIL corpus: {rfx_proven}/{rfx_total} bundle programs and {pe_proven}/{pe_total} PE programs narrowed SRV masks ({sm66_proven} PE SM6.6+)"
    );
    assert!(skin && depth && framegen, "missing expected Alan Wake 2 v1.2.10 shader fixtures");
}

#[test]
fn metal41_compute_scheduling_options_build_valid_psos() {
    if crate::metal::device().is_none() {
        return;
    }
    use crate::metal::m41::{ComputeSchedulingOpts, ContentionRelief, ForwardProgressUsage};
    let src = "
#include <metal_stdlib>
using namespace metal;
kernel void m41_sched_noop(device uint* dst [[buffer(0)]], uint id [[thread_position_in_grid]]) {
    dst[id] = id + 1u;
}
";
    let lib = crate::metal::pipeline::library_from_source_with_opts("m41_sched_test", src, false)
        .expect("compile m41_sched_test");
    let cases = [
        ComputeSchedulingOpts {
            forward_progress: Some(ForwardProgressUsage::Weak),
            contention_relief: Some(ContentionRelief::Automatic),
            optimize_for_persistent_kernel: true, // must be auto-suppressed when Weak
        },
        ComputeSchedulingOpts {
            forward_progress: Some(ForwardProgressUsage::SimdGroupParallel),
            contention_relief: Some(ContentionRelief::None),
            optimize_for_persistent_kernel: true,
        },
    ];
    for (idx, sched) in cases.iter().enumerate() {
        let desc = MTL4ComputePipelineDescriptor::new();
        desc.setComputeFunctionDescriptor(Some(&function_descriptor(&lib, "m41_sched_noop")));
        apply_compute_scheduling_options(&desc, sched);
        let pso = pso_cache()
            .expect("pso_cache")
            .compute_pipeline(None, &desc)
            .unwrap_or_else(|e| panic!("scheduling case {idx} failed: {e}"));
        assert!(pso.maxTotalThreadsPerThreadgroup() >= 32);
    }
}
