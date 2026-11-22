//! The `dxil-metald` helper backend: results identical to in-process conversion, errors
//! propagate with their original variant, concurrent compiles use a bounded worker pool, and a
//! worker crash costs one request (the next request gets a fresh worker).
//! Skips when the game or the converter isn't installed.
use std::path::Path;
use std::sync::Arc;

use aw2_data::{discover_game_dir, BinRfx};
use dxil_metal::*;

const HELPER: &str = env!("CARGO_BIN_EXE_dxil-metald");

fn game_programs(bundles: &[&str]) -> Option<Vec<(String, Vec<u8>)>> {
    let game = std::env::var("AW2_GAME_DIR").ok().map(Into::into).or_else(discover_game_dir)?;
    if !converter_available() {
        return None;
    }
    let dir = game.join("data/shaders/build/pc_dx12");
    let mut out = Vec::new();
    for b in bundles {
        for p in BinRfx::open(dir.join(b)).expect("parse binrfx").programs {
            // Libraries need export names; this test sticks to graphics/compute programs.
            if !p.entry_point.is_empty() {
                out.push((p.entry_point, p.dxbc_bytes));
            }
        }
    }
    Some(out)
}

#[test]
fn helper_matches_in_process_and_recovers_from_crashes() {
    let Some(progs) = game_programs(&["ambientconvolution.binrfx", "eye.binrfx", "billboard.binrfx"]) else {
        eprintln!("skip: game or converter not installed");
        return;
    };
    let local = ShaderPipeline::with_backend(CacheConfig::memory_only(), BackendKind::InProcess).unwrap();
    let remote = ShaderPipeline::with_helper(CacheConfig::memory_only(), Path::new(HELPER), 4).unwrap();
    assert_eq!(remote.backend(), BackendKind::Helper);
    // Same converter build -> same fingerprint -> same cache keys.
    assert_eq!(remote.converter_fingerprint(), local.converter_fingerprint());
    let opts = CompileOptions::default();

    for (entry, dxil) in &progs {
        let blob = RootSignatureDesc::covering_dxil(dxil).unwrap().to_blob();
        let (rs_l, rs_r) = (local.root_signature(&blob).unwrap(), remote.root_signature(&blob).unwrap());
        let a = local.compile(&CompileRequest::new(dxil, Some(&rs_l), &opts)).unwrap();
        let b = remote.compile(&CompileRequest::new(dxil, Some(&rs_r), &opts)).unwrap_or_else(|e| panic!("{entry}: {e}"));
        assert_eq!(a.key, b.key, "{entry}");
        assert_eq!(a.function_name, b.function_name);
        // The converter's JSON is not byte-deterministic (4.0.1 emits an uninitialized
        // "ResourceCount"), so compare the decoded reflection without it.
        let strip = |r: &ShaderReflection| ShaderReflection { json: String::new(), ..r.clone() };
        assert_eq!(strip(&a.reflection), strip(&b.reflection), "{entry}: reflection");
        assert_eq!(a.metallib, b.metallib, "{entry}: metallib");
        // Errors keep their variant across the process boundary.
        let wrong = if a.stage == ShaderStage::Compute { ShaderStage::Vertex } else { ShaderStage::Compute };
        let el = local.compile(&CompileRequest::new(dxil, Some(&rs_l), &opts).stage(wrong)).unwrap_err();
        let er = remote.compile(&CompileRequest::new(dxil, Some(&rs_r), &opts).stage(wrong)).unwrap_err();
        assert_eq!(el, er);
        assert!(matches!(er, Error::StageMismatch { .. }));
    }
    let bad = remote.compile(&CompileRequest::new(b"not dxil", None, &opts)).unwrap_err();
    assert!(matches!(bad, Error::InvalidInput(_)), "{bad:?}");

    // A missing resource binding is a converter error, not a helper failure.
    let (_, dxil) = progs
        .iter()
        .find(|(_, d)| dxbc::Container::parse(d).unwrap().resource_bindings().is_some_and(|v| !v.is_empty()))
        .unwrap();
    let empty = remote.root_signature(&RootSignatureDesc::covering(&[]).to_blob()).unwrap();
    let e = remote.compile(&CompileRequest::new(dxil, Some(&empty), &opts)).unwrap_err();
    assert!(matches!(e, Error::Convert { .. }), "{e:?}");

    // Crash isolation: the request fails with Error::Helper, the pool replaces the worker.
    let client = remote.helper().unwrap();
    let before = client.workers();
    assert!(matches!(client.crash_one_worker(), Error::Helper(_)));
    assert_eq!(client.workers(), before - 1);
    remote.cache().clear_memory();
    let (entry, dxil) = &progs[0];
    let blob = RootSignatureDesc::covering_dxil(dxil).unwrap().to_blob();
    let rs = remote.root_signature(&blob).unwrap();
    remote.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap_or_else(|e| panic!("{entry} after crash: {e}"));

    // Timeout: a hung worker is killed after the deadline and replaced on demand.
    assert_eq!(client.timeout(), Some(helper::DEFAULT_TIMEOUT));
    client.sleep_one_worker(10).unwrap();
    client.set_timeout(Some(std::time::Duration::from_millis(300)));
    let before = client.workers();
    let t = std::time::Instant::now();
    let e = client.sleep_one_worker(20_000).unwrap_err();
    assert!(t.elapsed() < std::time::Duration::from_secs(5), "{:?}", t.elapsed());
    assert!(matches!(&e, Error::Helper(m) if m.contains("timed out")), "{e:?}");
    assert_eq!(client.workers(), before - 1);
    remote.cache().clear_memory();
    remote.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap_or_else(|e| panic!("{entry} after timeout: {e}"));
}

#[test]
fn helper_pool_is_bounded_and_dedupes() {
    let Some(progs) = game_programs(&["cluster_operations.binrfx", "ambientconvolution.binrfx"]) else {
        eprintln!("skip: game or converter not installed");
        return;
    };
    let remote = Arc::new(ShaderPipeline::with_helper(CacheConfig::memory_only(), Path::new(HELPER), 3).unwrap());
    let progs = Arc::new(progs);
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let (remote, progs) = (remote.clone(), progs.clone());
            std::thread::spawn(move || {
                let opts = CompileOptions::default();
                for (entry, dxil) in progs.iter() {
                    let rs = remote.root_signature(&RootSignatureDesc::covering_dxil(dxil).unwrap().to_blob()).unwrap();
                    remote.compile(&CompileRequest::new(dxil, Some(&rs), &opts)).unwrap_or_else(|e| panic!("{entry}: {e}"));
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    let s = remote.stats();
    let unique = progs.iter().map(|(_, d)| d.clone()).collect::<std::collections::HashSet<_>>().len() as u64;
    assert_eq!(s.compiles, unique, "{s:?}");
    assert_eq!(s.compiles + s.memory_hits + s.dedup_waits, 8 * progs.len() as u64, "{s:?}");
    let workers = remote.helper().unwrap().workers();
    assert!((1..=3).contains(&workers), "{workers} workers");
}
