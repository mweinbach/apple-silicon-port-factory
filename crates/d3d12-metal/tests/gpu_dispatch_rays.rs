//! GPU integration tests for DXR Tier 1.0 state objects (`COLLECTION`, `RAYTRACING_PIPELINE`,
//! `EXISTING_COLLECTION`, `AddToStateObject`), shader identifiers, Shader Binding Tables (SBT),
//! local root signatures, and `CommandList::dispatch_rays` on Metal 4.

use std::ptr::NonNull;
use std::sync::Arc;

use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList};
use d3d12_metal::metal::queue::ListType;
use d3d12_metal::rt::accel::BuildArgs;
use d3d12_metal::rt::d3d::{build_flags, Geometry, Inputs, InstanceDesc, TrianglesDesc};
use d3d12_metal::rt::pipeline::{
    subobject_type, DispatchRaysDesc, HitGroupDesc, PipelineConfig, RawDxilLibraryDesc,
    RawExistingCollectionDesc, RawExportDesc, RawHitGroupDesc, RawStateObjectDesc,
    RawStateSubobject, RawSubobjectToExportsAssociation, ShaderConfig, StateObject,
    StateObjectDesc, VaRange, VaRangeAndStride, HIT_GROUP_TYPE_TRIANGLES,
    STATE_OBJECT_TYPE_COLLECTION, STATE_OBJECT_TYPE_RAYTRACING_PIPELINE,
};
use dxil_metal::dxbc;
use dxil_metal::root_signature::{
    RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
    RootSignatureDesc, RootSignatureVersion,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

const LIB_DXIL: &[u8] = include_bytes!("fixtures/dispatch_rays.lib.dxil");

type Buffer = Retained<ProtocolObject<dyn MTLBuffer>>;

/// Ray origins (x, y); rays travel along +z from z = -1.
const RAYS: [[f32; 2]; 8] = [
    [0.25, 0.25], // 0: hits inst 0, prim 0
    [0.75, 0.75], // 1: hits inst 0, prim 1
    [1.50, 0.50], // 2: miss
    [2.25, 0.25], // 3: hits inst 1, prim 0
    [2.75, 0.75], // 4: hits inst 1, prim 1
    [-0.5, 0.50], // 5: miss
    [0.25, 0.60], // 6: hits inst 0, prim 0
    [2.50, 0.40], // 7: hits inst 1, prim 0
];

fn signed(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    dxbc::sign_container(&mut v).unwrap();
    v
}

fn global_rs() -> Arc<RootSignature> {
    let srv = |reg: u32| RootParameter {
        kind: RootParameterKind::Descriptor(
            RootDescriptorKind::Srv,
            RootDescriptor {
                shader_register: reg,
                register_space: 0,
                flags: 0,
            },
        ),
        visibility: 0,
    };
    let uav = |reg: u32| RootParameter {
        kind: RootParameterKind::Descriptor(
            RootDescriptorKind::Uav,
            RootDescriptor {
                shader_register: reg,
                register_space: 0,
                flags: 0,
            },
        ),
        visibility: 0,
    };
    let consts = |reg: u32, n: u32| RootParameter {
        kind: RootParameterKind::Constants {
            shader_register: reg,
            register_space: 0,
            num_32bit_values: n,
        },
        visibility: 0,
    };
    RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![srv(0), uav(0), srv(1), consts(0, 4)],
        static_samplers: Vec::new(),
        flags: 0,
    })
    .expect("global root signature")
}

fn ready() -> bool {
    if !dxil_metal::converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return false;
    }
    let Some(d) = d3d12_metal::metal::device() else {
        eprintln!("skipping: no Metal 4 device");
        return false;
    };
    if !d.dev.supportsRaytracing() {
        eprintln!("skipping: device has no ray tracing support");
        return false;
    }
    true
}

fn shared(bytes: &[u8]) -> Buffer {
    let d = d3d12_metal::metal::device().unwrap();
    let b = d
        .dev
        .newBufferWithLength_options(bytes.len().max(256), MTLResourceOptions::StorageModeShared)
        .expect("buffer");
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), b.contents().as_ptr() as *mut u8, bytes.len());
    }
    d.add_resident(ProtocolObject::from_ref(&*b));
    b
}

fn bytes_of<T: Copy>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn submit_and_wait(list: &mut CommandList) {
    list.close().expect("close command list");
    let d = d3d12_metal::metal::device().unwrap();
    d.commit_residency();
    let queue = d.dev.newMTL4CommandQueue().expect("queue");
    queue.addResidencySet(&d.residency);
    let ev = d.dev.newSharedEvent().expect("event");
    let mut cbs = [NonNull::from(&*list.cb)];
    unsafe { queue.commit_count(NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
    queue.signalEvent_value(ProtocolObject::from_ref(&*ev), 1);
    assert!(ev.waitUntilSignaledValue_timeoutMS(1, 10_000), "GPU timeout");
}

struct TestScene {
    tlas_va: u64,
    _keep: Vec<Buffer>,
}

fn build_test_scene(alloc: &CommandAllocator, list: &mut CommandList) -> TestScene {
    let verts: [[f32; 3]; 4] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ];
    let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let vb = shared(bytes_of(&verts));
    let ib = shared(bytes_of(&indices));

    // Non-opaque triangles (flags = 0) so AnyHit shaders are invoked unless HitGroup or RayFlags
    // force opaque.
    let blas_inputs = Inputs::Bottom {
        flags: build_flags::PREFER_FAST_TRACE,
        geometries: vec![Geometry::Triangles {
            flags: 0,
            desc: TrianglesDesc {
                Transform3x4: 0,
                IndexFormat: 57, // DXGI_FORMAT_R16_UINT
                VertexFormat: 6, // DXGI_FORMAT_R32G32B32_FLOAT
                IndexCount: 6,
                VertexCount: 4,
                IndexBuffer: vb_ib_addr(&ib),
                VertexBufferStart: vb_ib_addr(&vb),
                VertexBufferStride: 12,
            },
        }],
    };
    let blas_buf = shared(&[0u8; 512]);
    let tlas_buf = shared(&[0u8; 512]);
    let scratch_buf = shared(&[0u8; 65536]);

    let blas_va = blas_buf.gpuAddress().next_multiple_of(256);
    let tlas_va = tlas_buf.gpuAddress().next_multiple_of(256);
    let scratch_va = scratch_buf.gpuAddress().next_multiple_of(256);

    // Instance 0: ID 10, mask 0x01, contribution 0 (HitGroup record 0)
    // Instance 1: ID 20, mask 0x02, contribution 1 (HitGroup record 1), translated by (2, 0, 1)
    let inst_descs = [
        InstanceDesc::new(
            [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            10,
            0x01,
            0,
            0,
            blas_va,
        ),
        InstanceDesc::new(
            [[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]],
            20,
            0x02,
            1,
            0,
            blas_va,
        ),
    ];
    let inst_buf = shared(bytes_of(&inst_descs));
    let tlas_inputs = Inputs::Top {
        flags: 0,
        count: 2,
        instances: inst_buf.gpuAddress(),
        pointers: false,
    };

    list.begin(alloc).expect("begin");
    list.build_accel(
        &BuildArgs {
            dest: blas_va,
            source: 0,
            scratch: scratch_va,
            inputs: blas_inputs,
            owner: 1,
        },
        &[],
    )
    .expect("build BLAS");
    list.barrier();
    list.build_accel(
        &BuildArgs {
            dest: tlas_va,
            source: 0,
            scratch: scratch_va,
            inputs: tlas_inputs,
            owner: 2,
        },
        &[],
    )
    .expect("build TLAS");
    submit_and_wait(list);

    TestScene {
        tlas_va,
        _keep: vec![vb, ib, blas_buf, tlas_buf, scratch_buf, inst_buf],
    }
}

fn vb_ib_addr(b: &Buffer) -> u64 {
    b.gpuAddress()
}

#[test]
fn dispatch_rays_basic_and_anyhit() {
    if !ready() {
        return;
    }
    let dxil = signed(LIB_DXIL);
    let grs = global_rs();

    let mut desc = StateObjectDesc {
        ty: STATE_OBJECT_TYPE_RAYTRACING_PIPELINE,
        ..Default::default()
    };
    desc.global_rs = Some(grs.clone());
    desc.add_dxil_library(&d3d12_metal::rt::pipeline::DxilLibraryDesc {
        dxil: dxil.into(),
        exports: Vec::new(),
    })
    .expect("add DXIL library");
    desc.hit_groups.insert(
        "HitGroupOpaque".to_string(),
        HitGroupDesc {
            export: "HitGroupOpaque".to_string(),
            hit_group_type: HIT_GROUP_TYPE_TRIANGLES,
            any_hit: None,
            closest_hit: Some("ClosestHitMain".to_string()),
            intersection: None,
        },
    );
    desc.hit_groups.insert(
        "HitGroupAlpha".to_string(),
        HitGroupDesc {
            export: "HitGroupAlpha".to_string(),
            hit_group_type: HIT_GROUP_TYPE_TRIANGLES,
            any_hit: Some("AnyHitRejectPrim1".to_string()),
            closest_hit: Some("ClosestHitMain".to_string()),
            intersection: None,
        },
    );
    desc.merge_shader_config(ShaderConfig {
        MaxPayloadSizeInBytes: 32,
        MaxAttributeSizeInBytes: 8,
    });
    desc.merge_pipeline_config(PipelineConfig {
        MaxTraceRecursionDepth: 1,
        Flags: 0,
    });

    let so = StateObject::new(desc).expect("compile raytracing StateObject");
    let rg_id = *so.shader_identifier("RayGenMain").expect("RayGenMain id");
    let miss0_id = *so.shader_identifier("MissMain").expect("MissMain id");
    let miss1_id = *so.shader_identifier("MissAlt").expect("MissAlt id");
    let hg_opaque_id = *so.shader_identifier("HitGroupOpaque").expect("HitGroupOpaque id");
    let hg_alpha_id = *so.shader_identifier("HitGroupAlpha").expect("HitGroupAlpha id");

    assert!(rg_id.shader_handle > 0);
    assert!(miss0_id.shader_handle > 0);
    assert!(miss1_id.shader_handle > 0);
    assert!(hg_opaque_id.shader_handle > 0);
    assert_eq!(hg_opaque_id.intersection_shader_handle, 0);
    assert!(hg_alpha_id.shader_handle > 0);
    assert!(hg_alpha_id.intersection_shader_handle > 0);

    let alloc = CommandAllocator::new(ListType::Direct).expect("allocator");
    let mut list = CommandList::new(ListType::Direct).expect("command list");
    let scene = build_test_scene(&alloc, &mut list);
    let rays_buf = shared(bytes_of(&RAYS));

    let run_pass = |list: &mut CommandList,
                    hg0: [u8; 32],
                    hg1: [u8; 32],
                    ray_flags: u32,
                    mask: u32,
                    miss_idx: u32|
     -> Vec<[u32; 4]> {
        // SBT layout (256B aligned sections):
        //   [0..32]:    RayGenMain
        //   [256..320]: MissMain, MissAlt (stride 32)
        //   [512..576]: HitGroup 0, HitGroup 1 (stride 32)
        let mut sbt_bytes = vec![0u8; 768];
        sbt_bytes[0..32].copy_from_slice(rg_id.as_bytes());
        sbt_bytes[256..288].copy_from_slice(miss0_id.as_bytes());
        sbt_bytes[288..320].copy_from_slice(miss1_id.as_bytes());
        sbt_bytes[512..544].copy_from_slice(&hg0);
        sbt_bytes[544..576].copy_from_slice(&hg1);
        let sbt_buf = shared(&sbt_bytes);
        let sbt_base = sbt_buf.gpuAddress();

        let out_buf = shared(&[0xCCu8; 16 * 16]);

        list.begin(&alloc).expect("begin");
        list.set_raytracing_pipeline(so.clone());
        list.compute_root
            .set_root_signature(grs.clone())
            .expect("set compute root sig");
        list.compute_root
            .set_descriptor(0, scene.tlas_va)
            .expect("set Scene SRV");
        list.compute_root
            .set_descriptor(1, out_buf.gpuAddress())
            .expect("set Out UAV");
        list.compute_root
            .set_descriptor(2, rays_buf.gpuAddress())
            .expect("set Rays SRV");
        list.compute_root
            .set_constants(3, 0, &[ray_flags, mask, miss_idx, 1])
            .expect("set GlobalParams");

        let dr = DispatchRaysDesc {
            RayGenerationShaderRecord: VaRange {
                StartAddress: sbt_base,
                SizeInBytes: 32,
            },
            MissShaderTable: VaRangeAndStride {
                StartAddress: sbt_base + 256,
                SizeInBytes: 64,
                StrideInBytes: 32,
            },
            HitGroupTable: VaRangeAndStride {
                StartAddress: sbt_base + 512,
                SizeInBytes: 64,
                StrideInBytes: 32,
            },
            CallableShaderTable: VaRangeAndStride::default(),
            Width: RAYS.len() as u32,
            Height: 1,
            Depth: 1,
            _pad: 0,
        };
        list.dispatch_rays(&dr).expect("dispatch_rays");
        submit_and_wait(list);

        unsafe { std::slice::from_raw_parts(out_buf.contents().as_ptr() as *const [u32; 4], 16) }
            .to_vec()
    };

    // Pass A: both instances use HitGroupOpaque, miss shader 0 (MissMain).
    let out_a = run_pass(
        &mut list,
        *hg_opaque_id.as_bytes(),
        *hg_opaque_id.as_bytes(),
        0,
        0xFF,
        0,
    );
    // Ray 0 ([0.25, 0.25]): hits inst 0 (ID 10), prim 0, t = 1.0, back-face HitKind = 255 (0xFE/0xFF).
    assert_eq!(out_a[0][0], 1, "ray 0 hit status");
    assert_eq!(out_a[0][1], 10, "ray 0 InstanceID");
    assert_eq!(out_a[0][2], 0, "ray 0 InstanceIndex");
    assert_eq!(out_a[0][3], 0, "ray 0 PrimitiveIndex");
    assert!((f32::from_bits(out_a[1][0]) - 1.0).abs() < 1e-4, "ray 0 t");
    assert!((f32::from_bits(out_a[1][1]) - 0.25).abs() < 1e-4, "ray 0 bary.x");
    assert!((f32::from_bits(out_a[1][2]) - 0.25).abs() < 1e-4, "ray 0 bary.y");
    assert_eq!(out_a[1][3], 0xFF, "ray 0 HitKind (back face = 255)");

    // Ray 1 ([0.75, 0.75]): hits inst 0 (ID 10), prim 1, t = 1.0.
    assert_eq!(out_a[2], [1, 10, 0, 1], "ray 1 hit");

    // Ray 2 ([1.50, 0.50]): miss -> MissMain (0xDEAD0001, ray index 2, dim 8, extra 0x1111).
    assert_eq!(out_a[4], [0, 0xDEAD_0001, 2, 8], "ray 2 MissMain");
    assert_eq!(out_a[5][3], 0x1111, "ray 2 MissMain extra");

    // Ray 3 ([2.25, 0.25]): hits inst 1 (ID 20), prim 0, t = 2.0.
    assert_eq!(out_a[6], [1, 20, 1, 0], "ray 3 hit inst 1 prim 0");
    assert!((f32::from_bits(out_a[7][0]) - 2.0).abs() < 1e-4, "ray 3 t");

    // Ray 4 ([2.75, 0.75]): hits inst 1 (ID 20), prim 1, t = 2.0.
    assert_eq!(out_a[8], [1, 20, 1, 1], "ray 4 hit inst 1 prim 1");

    // Pass B: miss shader 1 (MissAlt) + instance mask 0x01 (culls instance 1).
    let out_b = run_pass(
        &mut list,
        *hg_opaque_id.as_bytes(),
        *hg_opaque_id.as_bytes(),
        0,
        0x01,
        1,
    );
    assert_eq!(out_b[0], [1, 10, 0, 0], "ray 0 still hits inst 0");
    assert_eq!(out_b[6], [0, 0xDEAD_0002, 3, 8], "ray 3 culled by mask -> MissAlt");
    assert_eq!(out_b[7][3], 0x2222, "ray 3 MissAlt extra");

    // Pass C: Instance 0 uses HitGroupOpaque; Instance 1 uses HitGroupAlpha (AnyHitRejectPrim1).
    // On Instance 1:
    //   - Ray 3 hits prim 0 -> AnyHitRejectPrim1 sets extra = 0xA000 and accepts -> ClosestHitMain ORs 0xFF -> 0xA0FF.
    //   - Ray 4 hits prim 1 -> AnyHitRejectPrim1 calls IgnoreHit() -> MissMain!
    let out_c = run_pass(
        &mut list,
        *hg_opaque_id.as_bytes(),
        *hg_alpha_id.as_bytes(),
        0,
        0xFF,
        0,
    );
    assert_eq!(out_c[6], [1, 20, 1, 0], "ray 3 accepted by AnyHitRejectPrim1");
    assert_eq!(out_c[7][3], 0xA0FF, "ray 3 payload modified by AnyHit + ClosestHit");
    assert_eq!(out_c[8], [0, 0xDEAD_0001, 4, 8], "ray 4 rejected by AnyHitRejectPrim1 -> MissMain");
}

#[test]
fn dispatch_rays_collection_add_to_state_object_and_local_rs() {
    if !ready() {
        return;
    }
    let dxil = signed(LIB_DXIL);
    let grs = global_rs();

    // Local root signature: 2 root constants at register(b0, space1).
    let lrs_desc = RootSignatureDesc {
        version: dxil_metal::RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 1,
                num_32bit_values: 2,
            },
            visibility: 0, // D3D12_SHADER_VISIBILITY_ALL
        }],
        static_samplers: Vec::new(),
        flags: dxil_metal::root_signature::flags::LOCAL_ROOT_SIGNATURE,
    };
    let lrs = RootSignature::new(lrs_desc).expect("local root signature");

    // 1. Build a `COLLECTION` state object from raw D3D12 subobjects containing the library
    //    exports (`RayGenMain`, `MissMain`, `ClosestHitLocal`).
    let exp_rg = wide("RayGenMain");
    let exp_miss = wide("MissMain");
    let exp_ch_local = wide("ClosestHitLocal");
    let raw_exports = [
        RawExportDesc {
            Name: exp_rg.as_ptr(),
            ExportToRename: std::ptr::null(),
            Flags: 0,
            _pad: 0,
        },
        RawExportDesc {
            Name: exp_miss.as_ptr(),
            ExportToRename: std::ptr::null(),
            Flags: 0,
            _pad: 0,
        },
        RawExportDesc {
            Name: exp_ch_local.as_ptr(),
            ExportToRename: std::ptr::null(),
            Flags: 0,
            _pad: 0,
        },
    ];
    let raw_lib = RawDxilLibraryDesc {
        pShaderBytecode: dxil.as_ptr(),
        BytecodeLength: dxil.len(),
        NumExports: raw_exports.len() as u32,
        _pad: 0,
        pExports: raw_exports.as_ptr(),
    };
    let col_subs = [RawStateSubobject {
        Type: subobject_type::DXIL_LIBRARY,
        _pad: 0,
        pDesc: (&raw_lib as *const RawDxilLibraryDesc).cast(),
    }];
    let col_raw = RawStateObjectDesc {
        Type: STATE_OBJECT_TYPE_COLLECTION,
        NumSubobjects: col_subs.len() as u32,
        pSubobjects: col_subs.as_ptr(),
    };
    let col_desc = unsafe { StateObjectDesc::from_raw(&col_raw, None, |_| None, |_| None) }
        .expect("parse COLLECTION");
    let col_so = StateObject::new(col_desc).expect("create COLLECTION StateObject");
    assert!(col_so.pipeline.is_none());

    // 2. Build a base `RAYTRACING_PIPELINE` importing `col_so` via `EXISTING_COLLECTION`.
    let col_ptr = Arc::as_ptr(&col_so) as *mut std::ffi::c_void;
    let grs_ptr = 0x1001usize as *mut std::ffi::c_void;
    let lrs_ptr = 0x1002usize as *mut std::ffi::c_void;

    let raw_ec = RawExistingCollectionDesc {
        pExistingCollection: col_ptr,
        NumExports: 0,
        _pad: 0,
        pExports: std::ptr::null(),
    };
    let sc_raw = ShaderConfig {
        MaxPayloadSizeInBytes: 32,
        MaxAttributeSizeInBytes: 8,
    };
    let pc_raw = PipelineConfig {
        MaxTraceRecursionDepth: 1,
        Flags: 0,
    };
    let base_subs = [
        RawStateSubobject {
            Type: subobject_type::EXISTING_COLLECTION,
            _pad: 0,
            pDesc: (&raw_ec as *const RawExistingCollectionDesc).cast(),
        },
        RawStateSubobject {
            Type: subobject_type::GLOBAL_ROOT_SIGNATURE,
            _pad: 0,
            pDesc: (&grs_ptr as *const *mut std::ffi::c_void).cast(),
        },
        RawStateSubobject {
            Type: subobject_type::RAYTRACING_SHADER_CONFIG,
            _pad: 0,
            pDesc: (&sc_raw as *const ShaderConfig).cast(),
        },
        RawStateSubobject {
            Type: subobject_type::RAYTRACING_PIPELINE_CONFIG,
            _pad: 0,
            pDesc: (&pc_raw.MaxTraceRecursionDepth as *const u32).cast(),
        },
    ];
    let base_raw = RawStateObjectDesc {
        Type: STATE_OBJECT_TYPE_RAYTRACING_PIPELINE,
        NumSubobjects: base_subs.len() as u32,
        pSubobjects: base_subs.as_ptr(),
    };
    let base_desc = unsafe {
        StateObjectDesc::from_raw(
            &base_raw,
            None,
            |p| (p == grs_ptr).then(|| grs.clone()),
            |p| (p == col_ptr).then(|| col_so.clone()),
        )
    }
    .expect("parse base pipeline");
    let base_so = StateObject::new(base_desc).expect("compile base pipeline");

    // 3. Grow `base_so` via `AddToStateObject` (`StateObjectDesc::from_raw` with `Some(&base_so.desc)`),
    //    adding `HitGroupLocal` and associating `lrs` with it.
    let hg_local_name = wide("HitGroupLocal");
    let raw_hg = RawHitGroupDesc {
        HitGroupExport: hg_local_name.as_ptr(),
        Type: HIT_GROUP_TYPE_TRIANGLES,
        _pad: 0,
        AnyHitShaderImport: std::ptr::null(),
        ClosestHitShaderImport: exp_ch_local.as_ptr(),
        IntersectionShaderImport: std::ptr::null(),
    };
    let assoc_exports = [hg_local_name.as_ptr()];
    let mut add_subs = [
        RawStateSubobject {
            Type: subobject_type::HIT_GROUP,
            _pad: 0,
            pDesc: (&raw_hg as *const RawHitGroupDesc).cast(),
        },
        RawStateSubobject {
            Type: subobject_type::LOCAL_ROOT_SIGNATURE,
            _pad: 0,
            pDesc: (&lrs_ptr as *const *mut std::ffi::c_void).cast(),
        },
        RawStateSubobject {
            Type: subobject_type::SUBOBJECT_TO_EXPORTS_ASSOCIATION,
            _pad: 0,
            pDesc: std::ptr::null(),
        },
    ];
    let raw_assoc = RawSubobjectToExportsAssociation {
        pSubobjectToAssociate: &add_subs[1],
        NumExports: 1,
        _pad: 0,
        pExports: assoc_exports.as_ptr(),
    };
    add_subs[2].pDesc = (&raw_assoc as *const RawSubobjectToExportsAssociation).cast();
    let add_raw = RawStateObjectDesc {
        Type: STATE_OBJECT_TYPE_RAYTRACING_PIPELINE,
        NumSubobjects: add_subs.len() as u32,
        pSubobjects: add_subs.as_ptr(),
    };
    let grown_desc = unsafe {
        StateObjectDesc::from_raw(
            &add_raw,
            Some(&base_so.desc),
            |p| (p == lrs_ptr).then(|| lrs.clone()),
            |_| None,
        )
    }
    .expect("parse AddToStateObject");
    let grown_so = StateObject::new(grown_desc).expect("compile grown StateObject");

    let rg_id = *grown_so.shader_identifier("RayGenMain").unwrap();
    let miss_id = *grown_so.shader_identifier("MissMain").unwrap();
    let hg_local_id = *grown_so.shader_identifier("HitGroupLocal").unwrap();

    // 4. Build SBT with 64-byte HitGroup records carrying per-instance local root constants:
    //    Record 0 (Instance 0): ShaderIdentifier(HitGroupLocal) + [localTag = 0xCAFE_0001, localBias = 100]
    //    Record 1 (Instance 1): ShaderIdentifier(HitGroupLocal) + [localTag = 0xCAFE_0002, localBias = 200]
    let alloc = CommandAllocator::new(ListType::Direct).expect("allocator");
    let mut list = CommandList::new(ListType::Direct).expect("command list");
    let scene = build_test_scene(&alloc, &mut list);
    let rays_buf = shared(bytes_of(&RAYS));

    let mut sbt_bytes = vec![0u8; 768];
    sbt_bytes[0..32].copy_from_slice(rg_id.as_bytes());
    sbt_bytes[256..288].copy_from_slice(miss_id.as_bytes());
    // HitGroup record 0 at 512 (64B stride)
    sbt_bytes[512..544].copy_from_slice(hg_local_id.as_bytes());
    sbt_bytes[544..548].copy_from_slice(&0xCAFE_0001u32.to_le_bytes());
    sbt_bytes[548..552].copy_from_slice(&100u32.to_le_bytes());
    // HitGroup record 1 at 576 (64B stride)
    sbt_bytes[576..608].copy_from_slice(hg_local_id.as_bytes());
    sbt_bytes[608..612].copy_from_slice(&0xCAFE_0002u32.to_le_bytes());
    sbt_bytes[612..616].copy_from_slice(&200u32.to_le_bytes());

    let sbt_buf = shared(&sbt_bytes);
    let sbt_base = sbt_buf.gpuAddress();
    let out_buf = shared(&[0xCCu8; 16 * 16]);

    list.begin(&alloc).expect("begin");
    list.set_raytracing_pipeline(grown_so);
    list.compute_root.set_root_signature(grs).unwrap();
    list.compute_root.set_descriptor(0, scene.tlas_va).unwrap();
    list.compute_root.set_descriptor(1, out_buf.gpuAddress()).unwrap();
    list.compute_root.set_descriptor(2, rays_buf.gpuAddress()).unwrap();
    list.compute_root.set_constants(3, 0, &[0, 0xFF, 0, 1]).unwrap();

    let dr = DispatchRaysDesc {
        RayGenerationShaderRecord: VaRange {
            StartAddress: sbt_base,
            SizeInBytes: 32,
        },
        MissShaderTable: VaRangeAndStride {
            StartAddress: sbt_base + 256,
            SizeInBytes: 32,
            StrideInBytes: 32,
        },
        HitGroupTable: VaRangeAndStride {
            StartAddress: sbt_base + 512,
            SizeInBytes: 128,
            StrideInBytes: 64,
        },
        CallableShaderTable: VaRangeAndStride::default(),
        Width: RAYS.len() as u32,
        Height: 1,
        Depth: 1,
        _pad: 0,
    };
    list.dispatch_rays(&dr).expect("dispatch_rays with local root signature");
    submit_and_wait(&mut list);

    let out = unsafe { std::slice::from_raw_parts(out_buf.contents().as_ptr() as *const [u32; 4], 16) };
    // Ray 0 hits Instance 0 (InstanceID 10 + localBias 100 = 110, localTag = 0xCAFE_0001).
    assert_eq!(out[0], [1, 110, 0, 0], "ray 0 InstanceID + localBias");
    assert_eq!(out[1][3], 0xCAFE_0001, "ray 0 localTag from SBT record 0");
    // Ray 3 hits Instance 1 (InstanceID 20 + localBias 200 = 220, localTag = 0xCAFE_0002).
    assert_eq!(out[6], [1, 220, 1, 0], "ray 3 InstanceID + localBias");
    assert_eq!(out[7][3], 0xCAFE_0002, "ray 3 localTag from SBT record 1");
}

#[test]
fn shipped_game_dxr_state_objects_and_rayquery_psos() {
    if !ready() {
        return;
    }
    let Some(game) = std::env::var("AW2_GAME_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(aw2_data::discover_game_dir)
    else {
        eprintln!("skip: Alan Wake 2 game directory not found");
        return;
    };
    let dir = game.join("data/shaders/build/pc_dx12");
    let Ok(rd) = std::fs::read_dir(&dir) else {
        eprintln!("skip: {} not found", dir.display());
        return;
    };
    let mut rt_bundles: Vec<_> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "binrfx")
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("rt_"))
        })
        .collect();
    rt_bundles.sort();
    assert!(!rt_bundles.is_empty(), "expected rt_*.binrfx bundles in {}", dir.display());

    let mut lib_state_objects = 0usize;
    let mut total_identifiers = 0usize;
    let mut rayquery_compute_psos = 0usize;

    for path in &rt_bundles {
        let rfx = aw2_data::BinRfx::open(path).expect("parse rt_*.binrfx");
        // Filter out SM 6.9 (`lib_6_9` NVIDIA SER / HitObject) permutations; the runtime reports
        // SM 6.6 + DXR Tier 1.1 so the game selects the standard `lib_6_5`/`lib_6_6` libraries.
        for prog in rfx
            .programs
            .iter()
            .filter(|p| {
                p.entry_point.is_empty()
                    && dxil_metal::dxbc::summarize(&p.dxbc_bytes)
                        .ok()
                        .and_then(|s| s.program)
                        .is_some_and(|pr| pr.major == 6 && pr.minor <= 6)
            })
            .take(2)
        {
            let grs_desc = RootSignatureDesc::covering_dxil(&prog.dxbc_bytes)
                .unwrap_or_else(|| panic!("{} library covering_dxil returned None", rfx.file_name));
            let grs = RootSignature::new(grs_desc).expect("covering global root signature");

            let mut desc = StateObjectDesc {
                ty: STATE_OBJECT_TYPE_RAYTRACING_PIPELINE,
                global_rs: Some(grs),
                shader_config: Some(ShaderConfig {
                    MaxPayloadSizeInBytes: 256,
                    MaxAttributeSizeInBytes: 32,
                }),
                pipeline_config: Some(PipelineConfig {
                    MaxTraceRecursionDepth: 4,
                    Flags: 0,
                }),
                ..StateObjectDesc::default()
            };
            desc.add_dxil_library(&d3d12_metal::rt::pipeline::DxilLibraryDesc {
                dxil: Arc::from(prog.dxbc_bytes.as_slice()),
                exports: Vec::new(),
            })
            .unwrap_or_else(|e| panic!("{} add_dxil_library: {e}", rfx.file_name));

            let ch_names: Vec<String> = desc
                .exports
                .values()
                .filter(|e| e.kind == dxbc::ProgramKind::ClosestHit)
                .map(|e| e.export_name.clone())
                .collect();
            let ah_name: Option<String> = desc
                .exports
                .values()
                .find(|e| e.kind == dxbc::ProgramKind::AnyHit)
                .map(|e| e.export_name.clone());
            for (i, ch) in ch_names.iter().enumerate() {
                let hg_name = format!("hg_{i}_{ch}");
                desc.hit_groups.insert(
                    hg_name.clone(),
                    HitGroupDesc {
                        export: hg_name,
                        hit_group_type: HIT_GROUP_TYPE_TRIANGLES,
                        any_hit: ah_name.clone(),
                        closest_hit: Some(ch.clone()),
                        intersection: None,
                    },
                );
            }

            let so = StateObject::new(desc)
                .unwrap_or_else(|e| panic!("{} StateObject::new: {e}", rfx.file_name));
            assert!(so.pipeline.is_some());
            for hg in so.desc.hit_groups.keys() {
                let id = so.shader_identifier(hg).expect("hit group identifier");
                assert!(id.shader_handle != 0, "{} {hg}: zero closest-hit handle", rfx.file_name);
                if ah_name.is_some() {
                    assert!(
                        id.intersection_shader_handle != 0,
                        "{} {hg}: zero any-hit handle",
                        rfx.file_name
                    );
                }
            }
            total_identifiers += so.identifiers.len();
            lib_state_objects += 1;
        }

        for prog in rfx.programs.iter().filter(|p| !p.entry_point.is_empty()).take(2) {
            let summary = dxil_metal::dxbc::summarize(&prog.dxbc_bytes).expect("summarize");
            if summary.program.map(|p| p.kind) != Some(dxbc::ProgramKind::Compute) {
                continue;
            }
            if summary.program.is_some_and(|p| p.minor > 6) {
                continue;
            }
            let grs_desc = RootSignatureDesc::covering_dxil(&prog.dxbc_bytes).expect("covering_dxil");
            let grs = RootSignature::new(grs_desc).expect("root signature");
            let _pso = d3d12_metal::metal::pipeline::ComputePipeline::new(
                grs,
                &prog.dxbc_bytes,
                &prog.entry_point,
            )
            .unwrap_or_else(|e| panic!("{} {}: {e}", rfx.file_name, prog.entry_point));
            rayquery_compute_psos += 1;
        }
    }

    eprintln!(
        "shipped AW2 RT bundles ({}): built {} DXR StateObjects ({} shader identifiers) and {} RayQuery/RT compute PSOs on Metal 4",
        rt_bundles.len(),
        lib_state_objects,
        total_identifiers,
        rayquery_compute_psos
    );
    assert!(lib_state_objects > 0, "expected at least one lib_6_* DXR StateObject");
}
