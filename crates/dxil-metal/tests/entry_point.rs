//! Entry-point resolution for graphics/compute DXIL without a `PSV0` entry name (runtime info
//! v0-v2, validator < 1.8). Regression test for "entry point 'main' not found in DXIL" on game
//! shaders whose entry is e.g. `mainVS`.

use dxil_metal::*;

const FIXTURE: &[u8] = include_bytes!("fixtures/entry_v2.vs.dxil");
const HULL_V2_FIXTURE: &[u8] = include_bytes!("fixtures/entry_hull_v2.hs.dxil");
const HULL_PASSTHROUGH_FIXTURE: &[u8] = include_bytes!("fixtures/entry_hull_passthrough.hs.dxil");

/// Fixtures are written unsigned; sign them like real DXIL containers are.
fn signed_bytes(raw: &[u8]) -> Vec<u8> {
    let mut b = raw.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

fn signed() -> Vec<u8> {
    signed_bytes(FIXTURE)
}

#[test]
fn entry_comes_from_bitcode_when_psv0_has_none() {
    let dxil = signed();
    let c = dxbc::Container::parse(&dxil).unwrap();
    assert_eq!(c.psv_entry_point(), None, "fixture must be built without a PSV0 entry name");
    assert_eq!(c.dxil_entry_points().unwrap(), ["mainVS"]);
    assert_eq!(c.defined_functions().unwrap(), ["mainVS"]);
    assert_eq!(c.entry_point().as_deref(), Some("mainVS"));
}

#[test]
fn hull_entry_comes_from_dx_entrypoints_metadata() {
    // Non-passthrough hs_6_6 (PSV0 v2, 2 defined functions: patch constant + control point).
    let custom = signed_bytes(HULL_V2_FIXTURE);
    let c = dxbc::Container::parse(&custom).unwrap();
    assert_eq!(c.psv_entry_point(), None, "hs_6_6 fixture must have PSV0 v2 (no entry name)");
    assert_eq!(c.defined_functions().unwrap().len(), 2, "expected patch-constant + control-point functions");
    assert_eq!(c.dxil_entry_points().unwrap(), ["HS_Custom"]);
    assert_eq!(c.entry_point().as_deref(), Some("HS_Custom"));

    // Pass-through-style hs_6_6 (PSV0 v2, trivial copy control-point phase + patch constant).
    let passthrough = signed_bytes(HULL_PASSTHROUGH_FIXTURE);
    let cp = dxbc::Container::parse(&passthrough).unwrap();
    assert_eq!(cp.psv_entry_point(), None, "passthrough hs_6_6 fixture must have PSV0 v2");
    assert_eq!(cp.defined_functions().unwrap().len(), 2, "expected patch-constant + control-point functions");
    assert_eq!(cp.dxil_entry_points().unwrap(), ["HS_PassThrough"]);
    assert_eq!(cp.entry_point().as_deref(), Some("HS_PassThrough"));
}

#[test]
fn converts_hull_fixtures_without_explicit_entry_point() {
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return;
    }
    let mut opts = CompileOptions::default();
    opts.geometry_tessellation_emulation = true;
    opts.input_topology = Some(InputTopology::Patch);

    for (label, raw) in [("HS_Custom", HULL_V2_FIXTURE), ("HS_PassThrough", HULL_PASSTHROUGH_FIXTURE)] {
        let dxil = signed_bytes(raw);
        let s = compile_uncached(&CompileRequest::new(&dxil, None, &opts).stage(ShaderStage::Hull))
            .unwrap_or_else(|e| panic!("convert {label}: {e}"));
        assert_eq!(s.stage, ShaderStage::Hull);
        assert!(!s.metallib.is_empty());
        assert!(matches!(s.reflection.info, StageInfo::Hull { .. }), "{label}: {:?}", s.reflection.info);

        // Also confirm default options (used by precompile-shaders) succeed without an explicit entry point.
        let default_opts = CompileOptions::default();
        let s_def = compile_uncached(&CompileRequest::new(&dxil, None, &default_opts).stage(ShaderStage::Hull))
            .unwrap_or_else(|e| panic!("convert {label} (default opts): {e}"));
        assert_eq!(s_def.stage, ShaderStage::Hull);
        assert!(!s_def.metallib.is_empty());

        // Passing the wrong name ("main") still fails cleanly.
        let err = compile_uncached(&CompileRequest::new(&dxil, None, &opts).entry_point("main")).unwrap_err();
        assert!(err.to_string().contains("entry point"), "{label}: {err}");
    }
}

#[test]
fn converts_without_an_explicit_entry_point() {
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return;
    }
    let dxil = signed();
    let opts = CompileOptions::default();
    let s = compile_uncached(&CompileRequest::new(&dxil, None, &opts).stage(ShaderStage::Vertex)).expect("convert");
    assert_eq!(s.stage, ShaderStage::Vertex);
    assert!(!s.metallib.is_empty());
    // An explicit wrong name still fails cleanly.
    let e = compile_uncached(&CompileRequest::new(&dxil, None, &opts).entry_point("main")).unwrap_err();
    assert!(e.to_string().contains("entry point"), "{e}");
}
