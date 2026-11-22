//! SM4/SM5 (FXC) bytecode goes through libdxilconv, then the Metal Shader Converter. The
//! shaders are hand-assembled here (no fxc on macOS), and the test skips when either library
//! is missing.

use dxil_metal::*;

fn words(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// An empty input/output signature.
const EMPTY_SIG: [u8; 8] = [0, 0, 0, 0, 8, 0, 0, 0];

/// ```text
/// cs_5_0
/// dcl_globalFlags refactoringAllowed
/// dcl_constantbuffer cb0[1], immediateIndexed
/// dcl_uav_typed_buffer (float,float,float,float) u0
/// dcl_input vThreadID.x
/// dcl_thread_group 64, 1, 1
/// store_uav_typed u0.xyzw, vThreadID.xxxx, cb0[0].xyzw
/// ret
/// ```
fn sm5_compute() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = words(&[
        0x0005_0050, 25,
        0x0100_086a,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0400_089c, 0x0011_e000, 0, 0x5555,
        0x0200_005f, 0x0002_0012,
        0x0400_009b, 64, 1, 1,
        0x0700_00a4, 0x0011_e0f2, 0, 0x0002_0006, 0x0020_8e46, 0, 0,
        0x0100_003e,
    ]);
    dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &EMPTY_SIG), (*b"SHEX", &shex)])
}

/// ```text
/// ps_5_0
/// dcl_output o0.xyzw
/// mov o0.xyzw, l(1.0, 0, 0, 1.0)
/// ret
/// ```
fn sm5_pixel() -> Vec<u8> {
    #[rustfmt::skip]
    let shex = words(&[
        0x0000_0050, 14,
        0x0300_0065, 0x0010_20f2, 0,
        0x0800_0036, 0x0010_20f2, 0, 0x0000_4002, 0x3f80_0000, 0, 0, 0x3f80_0000,
        0x0100_003e,
    ]);
    // One SV_Target0 element: name offset, semantic index, system value, component type
    // (float), register, mask, rw mask.
    let mut osgn = words(&[1, 8, 32, 0, 0, 3, 0]);
    osgn.extend_from_slice(&[0x0f, 0, 0, 0]);
    osgn.extend_from_slice(b"SV_Target\0\0\0");
    dxbc::build_container(&[(*b"ISGN", &EMPTY_SIG), (*b"OSGN", &osgn), (*b"SHEX", &shex)])
}

#[test]
fn detects_sm5_bytecode() {
    let c = sm5_compute();
    let c = dxbc::Container::parse(&c).unwrap();
    assert!(c.is_sm4_bytecode());
    assert!(c.part(*b"DXIL").is_none());
    let p = c.program().expect("SM5 SHEX program header");
    assert_eq!((p.kind, p.major, p.minor), (dxbc::ProgramKind::Compute, 5, 0));
}

fn ready() -> bool {
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return false;
    }
    if !dxilconv::available() {
        eprintln!("skipping: libdxilconv.dylib not found (crates/dxil-metal/tools/dxilconv/build.sh)");
        return false;
    }
    true
}

#[test]
fn dxilconv_produces_dxil() {
    if !ready() {
        return;
    }
    let dxil = dxilconv::convert_signed(&sm5_compute()).expect("dxilconv");
    assert!(dxbc::verify_checksum(&dxil));
    let c = dxbc::Container::parse(&dxil).unwrap();
    let p = c.program().expect("DXIL program");
    assert_eq!((p.kind, p.major, p.minor), (dxbc::ProgramKind::Compute, 6, 0));
    assert_eq!(c.entry_point().as_deref(), Some("main"));
    // Garbage is an error, not a crash.
    let bad = dxbc::build_container(&[(*b"SHEX", &words(&[0x0005_0050, 3, 0xdead_beef]))]);
    assert!(dxilconv::convert(&bad).is_err());
}

#[test]
fn sm5_compiles_to_metal() {
    if !ready() {
        return;
    }
    let opts = CompileOptions::default();
    let cs = sm5_compute();
    let s = compile_uncached(&CompileRequest::new(&cs, None, &opts).stage(ShaderStage::Compute)).expect("compute");
    assert_eq!(s.reflection.info, reflection::StageInfo::Compute { threadgroup_size: [64, 1, 1] });
    assert!(!s.metallib.is_empty());
    // The key is that of the original DXBC, not of the intermediate DXIL.
    assert_eq!(s.key, CompileRequest::new(&cs, None, &opts).cache_key(&ffi::api().unwrap().fingerprint));

    let ps = sm5_pixel();
    let s = compile_uncached(&CompileRequest::new(&ps, None, &opts).stage(ShaderStage::Fragment)).expect("pixel");
    assert!(matches!(s.reflection.info, reflection::StageInfo::Fragment { num_render_targets: 1, .. }), "{:?}", s.reflection.info);

    // Through the cached pipeline too.
    let p = ShaderPipeline::new(CacheConfig::memory_only()).unwrap();
    let s = p.compile_stage(ShaderStage::Compute, &cs, None, &opts).expect("cached compute");
    assert_eq!(s.stage, ShaderStage::Compute);
}
