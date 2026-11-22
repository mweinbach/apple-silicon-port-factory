//! Manifest recording / loading (no converter or game needed).
use dxil_metal::manifest::{Manifest, ManifestWriter};
use dxil_metal::*;

#[test]
fn record_dedupe_reload_and_torn_tail() {
    let dir = std::env::temp_dir().join(format!("aw2-manifest-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let rs = RootSignature::new_deferred(RootSignatureDesc::covering_bindings([(DescriptorRangeType::Srv, 0, 0, 4)]));
    let opts = CompileOptions::default();
    let fake_a = b"DXBC-fake-a".to_vec();
    let fake_b = b"DXBC-fake-b".to_vec();
    {
        let w = ManifestWriter::open(&dir).unwrap();
        assert!(w.record(&CompileRequest::new(&fake_a, Some(&rs), &opts).stage(ShaderStage::Vertex)).unwrap());
        assert!(!w.record(&CompileRequest::new(&fake_a, Some(&rs), &opts).stage(ShaderStage::Vertex)).unwrap());
        assert!(w.record(&CompileRequest::new(&fake_b, None, &opts).entry_point("Export")).unwrap());
        assert_eq!(w.len(), 2);
    }
    // Dedupe survives a reopen; a torn tail (crash mid-append) is ignored.
    {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(dir.join("records.aw2m")).unwrap();
        f.write_all(&[200, 0, 0, 0, 1, 2, 3]).unwrap();
    }
    let w = ManifestWriter::open(&dir).unwrap();
    assert_eq!(w.len(), 2);
    assert!(!w.record(&CompileRequest::new(&fake_b, None, &opts).entry_point("Export")).unwrap());

    let m = Manifest::load(&dir).unwrap();
    assert_eq!(m.entries.len(), 2);
    let a = &m.entries[0];
    assert_eq!(a.expected_stage, Some(ShaderStage::Vertex));
    assert_eq!(a.root_signature, Some(rs.hash()));
    assert_eq!(a.options, opts);
    assert_eq!(m.dxil(a).unwrap(), fake_a);
    // Replay rebuilds the same root signature -> the same cache key as the original request.
    let blob = m.root_signature_blob(a.root_signature.unwrap()).unwrap();
    let rs2 = RootSignature::new_deferred(RootSignatureDesc::from_blob(&blob).unwrap());
    assert_eq!(rs2.hash(), rs.hash());
    let mut replay = CompileRequest::new(&fake_a, Some(&rs2), &a.options);
    replay.expected_stage = a.expected_stage;
    assert_eq!(replay.cache_key("fp"), CompileRequest::new(&fake_a, Some(&rs), &opts).cache_key("fp"));
    let b = &m.entries[1];
    assert_eq!((b.entry_point.as_deref(), b.root_signature), (Some("Export"), None));
    let _ = std::fs::remove_dir_all(&dir);
}
