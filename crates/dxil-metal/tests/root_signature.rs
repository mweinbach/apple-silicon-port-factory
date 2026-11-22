//! Root signature tests: hand-built D3D12 serialized blobs (per the RTS0 layout), round trips,
//! converter acceptance, and our argument-buffer layout vs the converter's.
use dxil_metal::root_signature::flags;
use dxil_metal::*;

/// Writes an RTS0 payload by hand, independent of `RootSignatureDesc::to_rts0`, following the
/// D3D12 serialized layout (header, parameter array, payloads, samplers). Version 1.0.
fn handmade_v10_blob() -> Vec<u8> {
    let mut b: Vec<u32> = Vec::new();
    // header: version, numParams, paramsOffset, numSamplers, samplersOffset, flags
    b.extend([1, 3, 24, 1, 0, flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT]);
    // param array at 24: 3 x {type, visibility, payloadOffset}
    let params_at = b.len();
    b.extend([0; 9]);
    // param 0: table with 2 ranges
    let p0 = b.len() * 4;
    b.extend([2, 0]);
    let ranges_at = b.len() * 4;
    b[p0 / 4 + 1] = ranges_at as u32;
    b.extend([0, 4, 0, 0, 0xFFFF_FFFF]); // SRV t0-t3, append
    b.extend([2, 1, 0, 0, 0xFFFF_FFFF]); // CBV b0, append
    // param 1: constants b1, 4 values
    let p1 = b.len() * 4;
    b.extend([1, 0, 4]);
    // param 2: root UAV u0 space 1
    let p2 = b.len() * 4;
    b.extend([0, 1]);
    // sampler
    let s_at = b.len() * 4;
    b[4] = s_at as u32;
    b.extend([0x15, 3, 3, 3, 0f32.to_bits(), 16, 1, 0, 0f32.to_bits(), 1000f32.to_bits(), 0, 0, 5]);
    for (i, (ty, vis, off)) in [(0, 0, p0), (1, 5, p1), (4, 0, p2)].into_iter().enumerate() {
        b[params_at + i * 3] = ty;
        b[params_at + i * 3 + 1] = vis;
        b[params_at + i * 3 + 2] = off as u32;
    }
    b.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn complex_desc(version: RootSignatureVersion) -> RootSignatureDesc {
    let v11 = version >= RootSignatureVersion::V1_1;
    let r = |t, n, reg, space, off| DescriptorRange {
        range_type: t,
        num_descriptors: n,
        base_shader_register: reg,
        register_space: space,
        flags: if v11 { 0x8 } else { 0 },
        offset_in_descriptors_from_table_start: off,
    };
    RootSignatureDesc {
        version,
        flags: flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Constants { shader_register: 0, register_space: 0, num_32bit_values: 3 },
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor { shader_register: 1, register_space: 0, flags: if v11 { 0x8 } else { 0 } },
                ),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    r(DescriptorRangeType::Srv, 8, 0, 0, flags::DESCRIPTOR_RANGE_OFFSET_APPEND),
                    r(DescriptorRangeType::Uav, 2, 0, 0, 10),
                    r(DescriptorRangeType::Cbv, 1, 2, 0, flags::DESCRIPTOR_RANGE_OFFSET_APPEND),
                    r(DescriptorRangeType::Srv, flags::UNBOUNDED, 0, 5, flags::DESCRIPTOR_RANGE_OFFSET_APPEND),
                ]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![r(DescriptorRangeType::Sampler, 4, 0, 0, 0)]),
                visibility: 5,
            },
            RootParameter {
                kind: RootParameterKind::Constants { shader_register: 3, register_space: 0, num_32bit_values: 1 },
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Srv,
                    RootDescriptor { shader_register: 9, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![StaticSampler {
            shader_register: 10,
            border_color: if version == RootSignatureVersion::V1_2 { 4 } else { 2 },
            flags: if version == RootSignatureVersion::V1_2 { flags::SAMPLER_FLAG_UINT_BORDER_COLOR } else { 0 },
            ..Default::default()
        }],
    }
}

#[test]
fn parses_handmade_v10_blob() {
    let rts0 = handmade_v10_blob();
    let d = RootSignatureDesc::from_blob(&rts0).unwrap();
    assert_eq!(d.version, RootSignatureVersion::V1_0);
    assert_eq!(d.parameters.len(), 3);
    let RootParameterKind::DescriptorTable(ranges) = &d.parameters[0].kind else { panic!() };
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[1].range_type, DescriptorRangeType::Cbv);
    assert_eq!(
        d.parameters[1].kind,
        RootParameterKind::Constants { shader_register: 1, register_space: 0, num_32bit_values: 4 }
    );
    assert_eq!(d.parameters[1].visibility, 5);
    assert!(matches!(d.parameters[2].kind, RootParameterKind::Descriptor(RootDescriptorKind::Uav, RootDescriptor { register_space: 1, .. })));
    assert_eq!(d.static_samplers[0].max_lod, 1000.0);
    assert_eq!(d.static_samplers[0].visibility, 5);
    // Re-serialization reproduces the same bytes (our layout matches the hand layout).
    assert_eq!(d.to_rts0(), rts0);
    // Wrapped in a DXBC container it parses identically.
    let blob = d.to_blob();
    assert!(dxbc::verify_checksum(&blob));
    assert_eq!(RootSignatureDesc::from_blob(&blob).unwrap(), d);
}

#[test]
fn round_trips_all_versions() {
    for v in [RootSignatureVersion::V1_0, RootSignatureVersion::V1_1, RootSignatureVersion::V1_2] {
        let d = complex_desc(v);
        let blob = d.to_blob();
        let back = RootSignatureDesc::from_blob(&blob).unwrap();
        assert_eq!(back, d, "{v:?}");
        assert_eq!(back.canonical_hash(), d.canonical_hash());
    }
    assert_ne!(
        complex_desc(RootSignatureVersion::V1_0).canonical_hash(),
        complex_desc(RootSignatureVersion::V1_1).canonical_hash()
    );
}

#[test]
fn rejects_garbage() {
    assert!(RootSignatureDesc::from_blob(&[1, 2, 3]).is_err());
    let mut b = complex_desc(RootSignatureVersion::V1_1).to_rts0();
    b[0] = 9; // bad version
    assert!(RootSignatureDesc::from_rts0(&b).is_err());
    let b = complex_desc(RootSignatureVersion::V1_1).to_rts0();
    assert!(RootSignatureDesc::from_rts0(&b[..b.len() - 8]).is_err());
}

#[test]
fn layout_rules() {
    let l = complex_desc(RootSignatureVersion::V1_1).layout();
    let offs: Vec<(u32, u32)> = l.parameters.iter().map(|p| (p.offset, p.size)).collect();
    // constants(3) @0..12, cbv aligned to 16, table @24, sampler table @32, const(1) @40, srv @48
    assert_eq!(offs, vec![(0, 12), (16, 8), (24, 8), (32, 8), (40, 4), (48, 8)]);
    // + static sampler table pointer @56
    assert_eq!(l.static_samplers, Some(StaticSamplerTableLayout { offset: 56, count: 1 }));
    assert_eq!(l.size, 64);
    let RootParameterLayoutKind::DescriptorTable { ranges, sampler } = &l.parameters[2].kind else { panic!() };
    assert!(!sampler);
    let o: Vec<u32> = ranges.iter().map(|r| r.descriptor_offset).collect();
    assert_eq!(o, vec![0, 10, 12, 13]);
    assert_eq!(ranges[3].num_descriptors, None);
    assert_eq!(ranges[1].byte_offset, 240);

    let mut ab = l.new_argument_buffer();
    ab.set_constants(&l, 0, 1, &[0xAABBCCDD, 7]);
    ab.set_root_descriptor(&l, 1, 0x1122_3344_5566_7788);
    ab.set_descriptor_table(&l, 2, 0xDEAD_0000);
    assert_eq!(&ab.bytes[4..8], &0xAABBCCDDu32.to_le_bytes());
    assert_eq!(&ab.bytes[8..12], &7u32.to_le_bytes());
    assert_eq!(&ab.bytes[16..24], &0x1122_3344_5566_7788u64.to_le_bytes());
    assert_eq!(&ab.bytes[24..32], &0xDEAD_0000u64.to_le_bytes());
}

#[test]
fn converter_accepts_and_layout_matches() {
    if !converter_available() {
        eprintln!("skip: converter unavailable");
        return;
    }
    for v in [RootSignatureVersion::V1_0, RootSignatureVersion::V1_1, RootSignatureVersion::V1_2] {
        let d = complex_desc(v);
        let rs = RootSignature::new(d.clone()).expect("converter rejected root signature");
        let locs = rs.converter_resource_locations().unwrap();
        // Every top-level resource the converter reports must sit at our parameter offset;
        // the trailing (-1, -1) table is the static sampler table.
        let sst = rs.layout().static_samplers.expect("static sampler table");
        assert_eq!(locs.len(), rs.layout().parameters.len() + 1);
        let last = locs.last().unwrap();
        assert_eq!((last.resource_type, last.space, last.slot, last.top_level_offset), (ResourceType::Table, u32::MAX, u32::MAX, sst.offset));
        assert_eq!(sst.count, 1);
        assert_eq!(rs.layout().size, sst.offset + 8);
        for loc in &locs[..locs.len() - 1] {
            let p = rs
                .layout()
                .parameters
                .iter()
                .find(|p| p.offset == loc.top_level_offset)
                .unwrap_or_else(|| panic!("{v:?}: converter offset {} not in our layout {:?}", loc.top_level_offset, rs.layout()));
            assert_eq!(loc.size_bytes, p.size as u64);
            match loc.resource_type {
                ResourceType::Constant => assert!(matches!(p.kind, RootParameterLayoutKind::Constants { .. })),
                ResourceType::Table => assert!(matches!(p.kind, RootParameterLayoutKind::DescriptorTable { .. })),
                ResourceType::Cbv => assert!(matches!(p.kind, RootParameterLayoutKind::Descriptor { kind: RootDescriptorKind::Cbv, .. })),
                ResourceType::Srv => assert!(matches!(p.kind, RootParameterLayoutKind::Descriptor { kind: RootDescriptorKind::Srv, .. })),
                ResourceType::Uav => assert!(matches!(p.kind, RootParameterLayoutKind::Descriptor { kind: RootDescriptorKind::Uav, .. })),
                t => panic!("unexpected {t:?}"),
            }
        }
        // The converter's own deserializer reads our blob too.
        let json = root_signature::converter_blob_to_json(&d.to_blob()).unwrap();
        assert!(json.contains("RootParameters") || json.len() > 10, "{json}");
    }
    let rs = RootSignature::from_blob(&handmade_v10_blob()).unwrap();
    assert_eq!(rs.desc().parameters.len(), 3);
}
