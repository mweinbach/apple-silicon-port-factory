//! Entry-point resolution for graphics/compute DXIL without a `PSV0` entry name (runtime info
//! v0-v2, validator < 1.8). Regression test for "entry point 'main' not found in DXIL" on game
//! shaders whose entry is e.g. `mainVS`.

use dxil_metal::*;

const FIXTURE: &[u8] = include_bytes!("fixtures/entry_v2.vs.dxil");

/// The fixture is written unsigned; sign it like the game's shaders are.
fn signed() -> Vec<u8> {
    let mut b = FIXTURE.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

#[test]
fn entry_comes_from_bitcode_when_psv0_has_none() {
    let dxil = signed();
    let c = dxbc::Container::parse(&dxil).unwrap();
    assert_eq!(c.psv_entry_point(), None, "fixture must be built without a PSV0 entry name");
    assert_eq!(c.defined_functions().unwrap(), ["mainVS"]);
    assert_eq!(c.entry_point().as_deref(), Some("mainVS"));
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
