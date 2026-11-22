//! Real game shaders: conversion against root signatures synthesized from each shader's own
//! binding tables (the game's shaders carry no RTS0; the game creates root signatures at
//! runtime), reflection sanity, DXR library exports, and pipeline cache round trips.
//! Skips when the game or the converter isn't installed.
use aw2_data::{discover_game_dir, BinRfx};
use dxil_metal::*;

fn game_programs(bundles: &[&str]) -> Option<Vec<(String, String, Vec<u8>)>> {
    let game = std::env::var("AW2_GAME_DIR").ok().map(Into::into).or_else(discover_game_dir)?;
    if !converter_available() {
        eprintln!("skip: converter unavailable");
        return None;
    }
    let dir = game.join("data/shaders/build/pc_dx12");
    let mut out = Vec::new();
    for b in bundles {
        let rfx = BinRfx::open(dir.join(b)).expect("parse binrfx");
        for p in rfx.programs {
            out.push((b.to_string(), p.entry_point, p.dxbc_bytes));
        }
    }
    Some(out)
}

#[test]
fn converts_graphics_and_compute_with_root_signatures() {
    let Some(progs) = game_programs(&["ambientconvolution.binrfx", "cluster_operations.binrfx", "eye.binrfx", "billboard.binrfx"])
    else {
        eprintln!("skip: game not installed");
        return;
    };
    let opts = CompileOptions::default();
    let mut stages = std::collections::BTreeMap::new();
    for (bundle, entry, dxil) in &progs {
        // Our DXBC checksum implementation matches what the shader compiler wrote.
        assert!(dxbc::verify_checksum(dxil), "{bundle} {entry}: checksum");
        let summary = dxbc::summarize(dxil).unwrap();
        assert!(!summary.has_root_signature, "game shaders are not expected to embed RTS0");
        // Graphics/compute PSOs pass no entry name: it must come from PSV0.
        assert_eq!(summary.entry_point.as_deref(), Some(entry.as_str()), "{bundle}");
        let desc = RootSignatureDesc::covering_dxil(dxil).expect("PSV0 bindings");
        // The synthesized signature survives the D3D12 blob round trip.
        let rs = RootSignature::from_blob(&desc.to_blob()).unwrap();
        let c = compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap_or_else(|e| panic!("{bundle} {entry}: {e}"));
        assert_eq!(&c.metallib[..4], b"MTLB");
        assert_eq!(c.function_name, *entry);
        // With a root signature, every reflected resource lives at a root parameter offset.
        for r in &c.reflection.resources {
            assert!(
                rs.layout().parameters.iter().any(|p| p.offset == r.top_level_offset),
                "{bundle} {entry}: resource {r:?} not at a root parameter ({:?})",
                rs.layout()
            );
        }
        match &c.reflection.info {
            StageInfo::Compute { threadgroup_size } | StageInfo::Mesh { threadgroup_size, .. } => {
                assert!(threadgroup_size.iter().all(|&n| n >= 1))
            }
            StageInfo::Vertex { .. } | StageInfo::Fragment { .. } => {}
            other => panic!("{bundle} {entry}: unexpected info {other:?}"),
        }
        // Expected-stage check is enforced.
        let wrong = if c.stage == ShaderStage::Compute { ShaderStage::Vertex } else { ShaderStage::Compute };
        assert!(matches!(
            compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts).stage(wrong)),
            Err(Error::StageMismatch { .. })
        ));
        *stages.entry(format!("{:?}", c.stage)).or_insert(0) += 1;
    }
    eprintln!("converted {} programs: {stages:?}", progs.len());
    for s in ["Vertex", "Fragment", "Compute", "Mesh"] {
        assert!(stages.contains_key(s), "no {s} shader exercised: {stages:?}");
    }
}

#[test]
fn converts_dxr_library_exports() {
    let Some(progs) = game_programs(&["rt_diffusegi.binrfx"]) else {
        eprintln!("skip: game not installed");
        return;
    };
    let opts = CompileOptions { ray_tracing: Some(RayTracingConfig::default()), ..Default::default() };
    let mut kinds = std::collections::BTreeSet::new();
    for (_, entry, dxil) in &progs {
        assert!(entry.is_empty(), "library programs have no bundle entry name");
        let c = dxbc::Container::parse(dxil).unwrap();
        let funcs = c.library_functions();
        assert!(!funcs.is_empty());
        let rs = RootSignature::new(RootSignatureDesc::covering_dxil(dxil).expect("RDAT bindings")).unwrap();
        for f in funcs {
            let r = compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts).entry_point(&f.unmangled_name))
                .unwrap_or_else(|e| panic!("{}: {e}", f.unmangled_name));
            assert!(r.stage.is_ray_tracing(), "{:?}", r.stage);
            kinds.insert(format!("{:?}", r.stage));
        }
        // Empty / missing entry names are rejected instead of crashing the converter.
        assert!(matches!(compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts).entry_point("")), Err(Error::InvalidInput(_))));
        assert!(compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts)).is_err());
        // Unknown export names crash converter 4.0.1, so they're rejected up front.
        assert!(matches!(
            compile_uncached(&CompileRequest::new(dxil, Some(&rs), &opts).entry_point("noSuchExport")),
            Err(Error::InvalidInput(_))
        ));
    }
    assert_eq!(kinds.len(), 4, "{kinds:?}");
}

#[test]
fn pipeline_cache_round_trip_and_stage_in() {
    let Some(progs) = game_programs(&["ambientconvolution.binrfx"]) else {
        eprintln!("skip: game not installed");
        return;
    };
    let dir = std::env::temp_dir().join(format!("dxil-metal-game-cache-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let cfg = CacheConfig { disk_dir: Some(dir.clone()), memory_limit_bytes: 64 << 20 };
    let opts = CompileOptions::default();
    let p1 = ShaderPipeline::new(cfg.clone()).unwrap();
    let mut firsts = Vec::new();
    for (_, _, dxil) in &progs {
        let rs = p1.root_signature(&RootSignatureDesc::covering_dxil(dxil).unwrap().to_blob()).unwrap();
        let a = p1.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap();
        let b = p1.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap();
        assert!(std::sync::Arc::ptr_eq(&a, &b));
        firsts.push(a);
    }
    assert_eq!(p1.stats().compiles as usize, progs.len());

    // Fresh pipeline (= next launch): everything from disk, identical results incl. reflection.
    let p2 = ShaderPipeline::new(cfg).unwrap();
    for ((_, _, dxil), first) in progs.iter().zip(&firsts) {
        let rs = p2.root_signature(&RootSignatureDesc::covering_dxil(dxil).unwrap().to_blob()).unwrap();
        let c = p2.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap();
        assert_eq!(*c, **first);
    }
    let s = p2.stats();
    assert_eq!((s.compiles, s.disk_hits as usize), (0, progs.len()));

    // Separate stage-in function for the vertex shader.
    let (_, _, vs) = progs.iter().find(|(_, e, _)| e.ends_with("VS")).expect("a vertex shader");
    let o2 = CompileOptions { stage_in_mode: StageInMode::SeparateFunction, ..Default::default() };
    let rs = p2.root_signature(&RootSignatureDesc::covering_dxil(vs).unwrap().to_blob()).unwrap();
    let v = p2.compile(&CompileRequest::new(vs, Some(&rs), &o2)).unwrap();
    let StageInfo::Vertex { inputs, .. } = &v.reflection.info else { panic!("not a vertex shader") };
    assert!(!inputs.is_empty());
    let layout: Vec<InputElement> = inputs
        .iter()
        .enumerate()
        .map(|(i, a)| InputElement {
            semantic_name: a.name.trim_end_matches(|c: char| c.is_ascii_digit()).to_string(),
            semantic_index: 0,
            format: 2, // DXGI_FORMAT_R32G32B32A32_FLOAT
            input_slot: 0,
            aligned_byte_offset: 16 * i as u32,
            input_slot_class: 0,
            instance_data_step_rate: 0,
        })
        .collect();
    let si = p2.stage_in(&v, &layout, &o2).unwrap();
    assert_eq!(&si.metallib[..4], b"MTLB");
    let text = String::from_utf8_lossy(&si.metallib);
    assert!(text.contains(compiler::STAGE_IN_FUNCTION_NAME), "stage-in function name changed");
    let si2 = p2.stage_in(&v, &layout, &o2).unwrap();
    assert!(std::sync::Arc::ptr_eq(&si, &si2));
    let _ = std::fs::remove_dir_all(dir);
}

/// Wider smoke test: first programs of many bundles, linear layout and covering root signature.
#[test]
fn converts_first_programs_of_many_bundles() {
    let Some(game) = std::env::var("AW2_GAME_DIR").ok().map(Into::into).or_else(discover_game_dir) else {
        eprintln!("skip: game not installed");
        return;
    };
    if !converter_available() {
        eprintln!("skip: converter unavailable");
        return;
    }
    let dir = game.join("data/shaders/build/pc_dx12");
    let mut bundles: Vec<_> = std::fs::read_dir(&dir)
        .expect("shader dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "binrfx"))
        .collect();
    bundles.sort();
    let opts = CompileOptions::default();
    let (mut ok, mut tried) = (0, 0);
    for path in bundles.iter().take(40) {
        let rfx = BinRfx::open(path).expect("parse binrfx");
        for prog in rfx.programs.iter().filter(|p| !p.entry_point.is_empty()).take(2) {
            tried += 1;
            let rs = RootSignatureDesc::covering_dxil(&prog.dxbc_bytes).map(|d| RootSignature::new(d).unwrap());
            match compile_uncached(&CompileRequest::new(&prog.dxbc_bytes, rs.as_deref(), &opts)) {
                Ok(m) => {
                    assert_eq!(&m.metallib[..4], b"MTLB");
                    ok += 1;
                }
                Err(e) => eprintln!("{} {}: {e}", rfx.file_name, prog.entry_point),
            }
        }
    }
    eprintln!("converted {ok}/{tried}");
    assert!(tried > 0 && ok == tried, "conversion failures: {ok}/{tried}");
}
