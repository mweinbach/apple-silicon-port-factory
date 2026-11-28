//! End-to-end GPU test of pure in-register FP64 (`double`) DXIL lowering to FP32 (`float`).
//!
//! Apple Silicon GPUs have no FP64 hardware, and Metal Shader Converter 4.0.1 leaves raw LLVM
//! `double` instructions (`phi`/`select`/`fmul`/`fadd`/`fptrunc`) in emitted AIR, causing
//! Apple's `AGXCompilerCore` (`MTLCompilerService`) to abort with `report_fatal_error` during
//! compute pipeline compilation. This test compiles our DXC-built `fixtures/fp64_in_register.cs.dxil`
//! fixture through `dxil_metal::compile_uncached`, builds a Metal 4 compute PSO via
//! `PipelineCache::compute_pipeline`, dispatches it on the GPU, and checks the readback values.
#![cfg(all(target_os = "macos", feature = "metal"))]

use dxil_metal::pso_cache::{PipelineCache, PipelineCacheConfig};
use dxil_metal::runtime::ARGUMENT_BUFFER_BIND_POINT;
use dxil_metal::*;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSString, NSURL};
use objc2_metal::*;

const FP64_FIXTURE: &[u8] = include_bytes!("fixtures/fp64_in_register.cs.dxil");
const STATIC_SAMPLERS_FIXTURE: &[u8] = include_bytes!("fixtures/static_samplers.cs.dxil");

fn signed_dxil(raw: &[u8]) -> Vec<u8> {
    let mut b = raw.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

#[test]
fn fp64_in_register_dxil_lowers_compiles_and_executes_on_metal4_gpu() {
    let dxil = signed_dxil(FP64_FIXTURE);
    assert!(
        bitcode::needs_fp64_lowering(&dxil),
        "DXC fp64 fixture should be detected as eligible for FP64->FP32 lowering"
    );
    let non_fp64 = signed_dxil(STATIC_SAMPLERS_FIXTURE);
    assert!(
        !bitcode::needs_fp64_lowering(&non_fp64),
        "non-FP64 shader must not trigger FP64 lowering or cache-key salt"
    );

    // Lowered container must be valid DXBC with Doubles SFI0 bit cleared and no remaining double type.
    let lowered = bitcode::lower_dxil_container(&dxil).expect("lower_dxil_container should lower fp64 fixture");
    assert!(dxbc::verify_checksum(&lowered), "lowered container checksum must verify");
    let parsed = dxbc::Container::parse(&lowered).expect("parse lowered container");
    assert_eq!(parsed.feature_flags() & 1, 0, "SFI0 Doubles bit must be cleared after lowering");
    assert!(!bitcode::needs_fp64_lowering(&lowered), "lowering must be idempotent");

    if !converter_available() {
        eprintln!("skipping GPU conversion: Metal Shader Converter not available");
        return;
    }
    let Some(device) = MTLCreateSystemDefaultDevice() else {
        eprintln!("skipping GPU execution: no Metal device");
        return;
    };
    if !device.supportsFamily(MTLGPUFamily::Metal4) {
        eprintln!("skipping GPU execution: device does not support Metal 4");
        return;
    }

    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Uav,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        }],
        static_samplers: vec![],
        flags: 0,
    })
    .expect("root signature");

    let opts = CompileOptions::default();
    let shader = compile_uncached(&CompileRequest::new(&dxil, Some(&rs), &opts).stage(ShaderStage::Compute))
        .expect("compile_uncached should succeed for lowered fp64 DXIL");
    assert_eq!(shader.function_name, "CS_Fp64Lowering");

    let tmp_path = std::env::temp_dir().join(format!(
        "dxil-metal-fp64-{}-{:032x}.metallib",
        std::process::id(),
        shader.key
    ));
    std::fs::write(&tmp_path, &shader.metallib).expect("write metallib");
    let url = NSURL::fileURLWithPath(&NSString::from_str(tmp_path.to_str().unwrap()));
    let lib = device.newLibraryWithURL_error(&url).expect("newLibraryWithURL");
    let _ = std::fs::remove_file(&tmp_path);

    // Build Metal 4 compute PSO via PipelineCache (this is what previously crashed AGXCompilerCore
    // with XPC_ERROR_CONNECTION_INTERRUPTED / report_fatal_error when raw `double` leaked into AIR).
    let cache = PipelineCache::open(
        &device,
        &PipelineCacheConfig { dir: None, max_archives: 8, flush_on_drop: false },
    )
    .expect("open memory-only PipelineCache");
    let fn_desc = MTL4LibraryFunctionDescriptor::new();
    fn_desc.setLibrary(Some(&lib));
    fn_desc.setName(Some(&NSString::from_str(&shader.function_name)));
    let pso_desc = MTL4ComputePipelineDescriptor::new();
    pso_desc.setComputeFunctionDescriptor(Some(&fn_desc));
    pso_desc.setMaxTotalThreadsPerThreadgroup(4);
    pso_desc.setRequiredThreadsPerThreadgroup(MTLSize { width: 4, height: 1, depth: 1 });
    let pso = cache
        .compute_pipeline(Some(shader.key), &pso_desc)
        .expect("MTL4 compute PSO compilation must succeed without AGXCompilerCore abort");

    // Dispatch the lowered compute shader on the GPU and verify exact float results:
    //   gid.x = 0: 38.0 * 0.05 + 0.00 = 1.90
    //   gid.x = 1: 38.0 * 1.00 + 0.25 = 38.25
    //   gid.x = 2: 32.0 * 0.05 + 0.50 = 2.10
    //   gid.x = 3: 32.0 * 1.00 + 0.75 = 32.75
    let out_buf = device
        .newBufferWithLength_options(16, MTLResourceOptions::StorageModeShared)
        .expect("output buffer");
    let mut tlab = rs.layout().new_argument_buffer();
    tlab.set_root_descriptor(rs.layout(), 0, out_buf.gpuAddress());
    let tlab_buf = device
        .newBufferWithLength_options(tlab.bytes.len().max(16), MTLResourceOptions::StorageModeShared)
        .expect("tlab buffer");
    unsafe {
        std::ptr::copy_nonoverlapping(
            tlab.bytes.as_ptr(),
            tlab_buf.contents().as_ptr() as *mut u8,
            tlab.bytes.len(),
        );
    }

    let queue = device.newCommandQueue().expect("command queue");
    let cb = queue.commandBuffer().expect("command buffer");
    let enc = cb.computeCommandEncoder().expect("compute encoder");
    enc.setComputePipelineState(&pso);
    unsafe { enc.setBuffer_offset_atIndex(Some(&tlab_buf), 0, ARGUMENT_BUFFER_BIND_POINT as usize) };
    enc.useResource_usage(ProtocolObject::from_ref(&*out_buf), MTLResourceUsage::Read | MTLResourceUsage::Write);
    enc.dispatchThreadgroups_threadsPerThreadgroup(
        MTLSize { width: 1, height: 1, depth: 1 },
        MTLSize { width: 4, height: 1, depth: 1 },
    );
    enc.endEncoding();
    cb.commit();
    cb.waitUntilCompleted();
    assert_eq!(cb.status(), MTLCommandBufferStatus::Completed, "GPU error: {:?}", cb.error());

    let got: [f32; 4] = unsafe { *(out_buf.contents().as_ptr() as *const [f32; 4]) };
    let expected = [
        38.0f32 * 0.05f32 + 0.00f32,
        38.0f32 * 1.00f32 + 0.25f32,
        32.0f32 * 0.05f32 + 0.50f32,
        32.0f32 * 1.00f32 + 0.75f32,
    ];
    for (i, (&g, &e)) in got.iter().zip(&expected).enumerate() {
        assert!((g - e).abs() < 1e-5, "lane {i}: got {g}, expected {e}");
    }
}
