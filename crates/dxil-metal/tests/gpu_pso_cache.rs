//! Metal 4 PSO cache: compile through the cache's MTL4Compiler, then flush. On reopen, the
//! pipelines come back as archive hits with no compilation. Also covers in-memory dedup,
//! incremental session archives, and compaction. Skips without a Metal device or the converter.
#![cfg(all(target_os = "macos", feature = "metal"))]

use std::path::{Path, PathBuf};

use dxil_metal::pso_cache::{PipelineCache, PipelineCacheConfig};
use dxil_metal::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSString, NSURL};
use objc2_metal::*;

const FIXTURE: &[u8] = include_bytes!("fixtures/static_samplers.cs.dxil");

fn fixture_dxil() -> Vec<u8> {
    let mut b = FIXTURE.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

fn tmpdir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("dxil-metal-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn range(range_type: DescriptorRangeType, reg: u32, n: u32) -> DescriptorRange {
    DescriptorRange {
        range_type,
        num_descriptors: n,
        base_shader_register: reg,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
    }
}

/// Two root-signature layouts give two different metallibs for the same fixture. (Static
/// samplers vs a sampler table at the same offset would not: the converter emits identical code.)
fn root_signature(swap: bool) -> std::sync::Arc<RootSignature> {
    let mut parameters = vec![
        RootParameter { kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Srv, 0, 1)]), visibility: 0 },
        RootParameter {
            kind: RootParameterKind::Descriptor(RootDescriptorKind::Uav, RootDescriptor { shader_register: 0, register_space: 0, flags: 0 }),
            visibility: 0,
        },
    ];
    if swap {
        parameters.swap(0, 1);
    }
    parameters.push(RootParameter {
        kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, 0, 2)]),
        visibility: 0,
    });
    RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters,
        static_samplers: vec![],
        flags: 0,
    })
    .unwrap()
}

fn load_library(device: &ProtocolObject<dyn MTLDevice>, dir: &Path, shader: &CompiledShader) -> Retained<ProtocolObject<dyn MTLLibrary>> {
    let path = dir.join(format!("{:032x}.metallib", shader.key));
    std::fs::write(&path, &shader.metallib).unwrap();
    let url = NSURL::fileURLWithPath(&NSString::from_str(path.to_str().unwrap()));
    device.newLibraryWithURL_error(&url).expect("newLibraryWithURL")
}

fn descriptor(lib: &ProtocolObject<dyn MTLLibrary>, name: &str) -> Retained<MTL4ComputePipelineDescriptor> {
    let f = MTL4LibraryFunctionDescriptor::new();
    f.setName(Some(&NSString::from_str(name)));
    f.setLibrary(Some(lib));
    let d = MTL4ComputePipelineDescriptor::new();
    d.setComputeFunctionDescriptor(Some(&f));
    d
}

fn archives(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "mtl4a"))
        .collect();
    v.sort();
    v
}

#[test]
fn pipelines_round_trip_through_archives() {
    let Some(device) = MTLCreateSystemDefaultDevice() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return;
    }
    let root = tmpdir("pso");
    let libdir = root.join("libs");
    std::fs::create_dir_all(&libdir).unwrap();
    let dxil = fixture_dxil();
    let opts = CompileOptions::default();
    let shaders: Vec<CompiledShader> = [false, true]
        .into_iter()
        .map(|s| {
            let rs = root_signature(s);
            compile_uncached(&CompileRequest::new(&dxil, Some(&rs), &opts).stage(ShaderStage::Compute)).unwrap()
        })
        .collect();
    assert_ne!(shaders[0].metallib, shaders[1].metallib);
    let libs: Vec<_> = shaders.iter().map(|s| load_library(&device, &libdir, s)).collect();
    let descs: Vec<_> = libs.iter().zip(&shaders).map(|(l, s)| descriptor(l, &s.function_name)).collect();

    let cfg = |max_archives| PipelineCacheConfig { dir: Some(root.join("pso")), max_archives, flush_on_drop: false };
    let check = |p: &ProtocolObject<dyn MTLComputePipelineState>| {
        assert!(p.maxTotalThreadsPerThreadgroup() >= 16 && p.threadExecutionWidth() > 0);
    };

    // Session 1: a cold compile of A. Dedup by key; one archive written; the second flush is a no-op.
    let c = PipelineCache::open(&device, &cfg(16)).unwrap();
    assert!(c.is_cold_device(), "brand-new device cache directory must report is_cold_device=true");
    assert!(!c.device_identity_str().is_empty());
    let dir = c.dir().unwrap().to_path_buf();
    assert!(dir.join("device.txt").exists());
    let a1 = c.compute_pipeline(Some(1), &descs[0]).unwrap();
    let a2 = c.compute_pipeline(Some(1), &descs[0]).unwrap();
    check(&a1);
    assert!(std::ptr::eq(&*a1, &*a2), "keyed requests must share one pipeline object");
    let s = c.stats();
    assert_eq!((s.compiles, s.memory_hits, s.archive_hits, s.archives_loaded), (1, 1, 0, 0));
    let cold = s.compile_nanos;
    assert!(c.flush().unwrap().is_some());
    assert_eq!(c.flush().unwrap(), None);
    drop(c);
    assert_eq!(archives(&dir).len(), 1);

    // Session 2: A is an archive hit. B is a miss and goes into a second archive.
    let c = PipelineCache::open(&device, &cfg(16)).unwrap();
    assert!(!c.is_cold_device(), "device cache with loaded archive must report is_cold_device=false");
    check(&c.compute_pipeline(None, &descs[0]).unwrap());
    let s = c.stats();
    assert_eq!((s.archives_loaded, s.archive_hits, s.compiles), (1, 1, 0), "{s:?}");
    let warm = s.lookup_nanos;
    check(&c.compute_pipeline(None, &descs[1]).unwrap());
    assert_eq!(c.stats().compiles, 1);
    let second = c.flush().unwrap().expect("second archive");
    drop(c);
    assert_eq!(archives(&dir).len(), 2);
    eprintln!("pso cache: cold compile {:.2} ms, archive lookup {:.2} ms", cold as f64 / 1e6, warm as f64 / 1e6);

    // Second archive holds only B (A came from the first archive, so it was not recompiled).
    let only_b = device
        .newArchiveWithURL_error(&NSURL::fileURLWithPath(&NSString::from_str(second.to_str().unwrap())))
        .unwrap();
    assert!(only_b.newComputePipelineStateWithDescriptor_error(&descs[1]).is_ok());
    let a_in_second = only_b.newComputePipelineStateWithDescriptor_error(&descs[0]).is_ok();
    eprintln!("pso cache: session archive also contains earlier pipelines: {a_in_second}");

    // Session 3: max 1 archive means compaction. Both pipelines are rebuilt through lookupArchives,
    // then one archive replaces the two.
    let c = PipelineCache::open(&device, &cfg(1)).unwrap();
    assert!(c.is_compacting());
    check(&c.compute_pipeline(None, &descs[0]).unwrap());
    check(&c.compute_pipeline(None, &descs[1]).unwrap());
    let s = c.stats();
    assert_eq!((s.archive_hits, s.compiles), (0, 2));
    eprintln!("pso cache: compaction rebuild of 2 pipelines {:.2} ms", s.compile_nanos as f64 / 1e6);
    let compact = c.flush().unwrap().expect("compacted archive");
    drop(c);
    assert_eq!(archives(&dir), vec![compact]);

    // Session 4: both are hits from the compacted archive.
    let c = PipelineCache::open(&device, &cfg(16)).unwrap();
    check(&c.compute_pipeline(None, &descs[0]).unwrap());
    check(&c.compute_pipeline(None, &descs[1]).unwrap());
    let s = c.stats();
    assert_eq!((s.archives_loaded, s.archive_hits, s.compiles), (1, 2, 0), "{s:?}");
    assert_eq!(c.flush().unwrap(), None);
    drop(c);

    // Corrupt or truncated archives are dropped at open, not fatal. (Handing junk to Metal
    // would raise inside newArchiveWithURL.)
    let good = std::fs::read(&archives(&dir)[0]).unwrap();
    std::fs::write(dir.join("zz-corrupt.mtl4a"), b"not an archive").unwrap();
    std::fs::write(dir.join("zz-truncated.mtl4a"), &good[..good.len() / 2]).unwrap();
    let c = PipelineCache::open(&device, &cfg(16)).unwrap();
    let s = c.stats();
    assert_eq!((s.archives_loaded, s.archives_rejected), (1, 2));
    assert!(!dir.join("zz-corrupt.mtl4a").exists() && !dir.join("zz-truncated.mtl4a").exists());
    check(&c.compute_pipeline(None, &descs[1]).unwrap());
    assert_eq!(c.stats().archive_hits, 1);
    drop(c);

    // No directory: a plain compiler with no serializer, so flush does nothing.
    let c = PipelineCache::open(&device, &PipelineCacheConfig { dir: None, max_archives: 16, flush_on_drop: true }).unwrap();
    check(&c.compute_pipeline(None, &descs[0]).unwrap());
    assert_eq!(c.flush().unwrap(), None);

    // Metal 4.1 MSL source compilation (`MTLLanguageVersion4_1` / `__METAL_VERSION__ == 410`)
    // and compute pipeline build through `MTL4Compiler`.
    let msl_4_1 = r#"
        #include <metal_stdlib>
        using namespace metal;
        static_assert(__METAL_VERSION__ >= 410, "expected Metal 4.1 language version");
        kernel void msl41_noop(device uint* out [[buffer(0)]], uint tid [[thread_position_in_grid]]) {
            if (tid == 0) out[0] = __METAL_VERSION__;
        }
    "#;
    let lib41 = c.library_from_source("msl41_test", msl_4_1).expect("Metal 4.1 MSL library");
    let pso41 = c.compute_pipeline_from_function(Some(41), &lib41, "msl41_noop").expect("Metal 4.1 compute pipeline");
    check(&pso41);

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn port_pso_cache_env_precedence() {
    let prev_port = std::env::var_os("PORT_PSO_CACHE");
    let prev_aw2 = std::env::var_os("AW2_PSO_CACHE");
    let prev_max_p = std::env::var_os("PORT_PSO_MAX_ARCHIVES");
    let prev_max_a = std::env::var_os("AW2_PSO_MAX_ARCHIVES");

    std::env::remove_var("PORT_PSO_CACHE");
    std::env::set_var("AW2_PSO_CACHE", "/tmp/aw2-pso-legacy");
    assert_eq!(PipelineCacheConfig::from_env().dir, Some(PathBuf::from("/tmp/aw2-pso-legacy")));
    std::env::set_var("PORT_PSO_CACHE", "/tmp/port-pso-primary");
    assert_eq!(PipelineCacheConfig::from_env().dir, Some(PathBuf::from("/tmp/port-pso-primary")));
    std::env::set_var("PORT_PSO_CACHE", "off");
    assert_eq!(PipelineCacheConfig::from_env().dir, None);

    std::env::remove_var("PORT_PSO_MAX_ARCHIVES");
    std::env::set_var("AW2_PSO_MAX_ARCHIVES", "12");
    assert_eq!(PipelineCacheConfig::from_env().max_archives, 12);
    std::env::set_var("PORT_PSO_MAX_ARCHIVES", "24");
    assert_eq!(PipelineCacheConfig::from_env().max_archives, 24);

    match prev_port {
        Some(v) => std::env::set_var("PORT_PSO_CACHE", v),
        None => std::env::remove_var("PORT_PSO_CACHE"),
    }
    match prev_aw2 {
        Some(v) => std::env::set_var("AW2_PSO_CACHE", v),
        None => std::env::remove_var("AW2_PSO_CACHE"),
    }
    match prev_max_p {
        Some(v) => std::env::set_var("PORT_PSO_MAX_ARCHIVES", v),
        None => std::env::remove_var("PORT_PSO_MAX_ARCHIVES"),
    }
    match prev_max_a {
        Some(v) => std::env::set_var("AW2_PSO_MAX_ARCHIVES", v),
        None => std::env::remove_var("AW2_PSO_MAX_ARCHIVES"),
    }
}
