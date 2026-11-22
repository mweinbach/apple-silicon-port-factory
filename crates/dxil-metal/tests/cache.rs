//! Cache behavior: misses/hits across memory and disk, concurrent de-duplication, negative
//! caching, corruption recovery, and cache-key sensitivity. Uses synthetic entries, so it runs
//! without the converter or the game.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::time::Duration;

use dxil_metal::*;

fn tmpdir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("dxil-metal-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

fn fake(key: u128, n: u8) -> CompiledShader {
    CompiledShader {
        metallib: vec![b'M', b'T', b'L', b'B', n, n, n],
        function_name: format!("fn{n}"),
        stage: ShaderStage::Compute,
        reflection: ShaderReflection {
            stage: ShaderStage::Compute,
            function_name: format!("fn{n}"),
            resources: vec![],
            needs_function_constants: false,
            function_constants: vec![],
            info: StageInfo::None,
            json: String::new(), // empty JSON: decoding doesn't need the converter
        },
        key,
    }
}

#[test]
fn memory_then_disk_hits() {
    let dir = tmpdir("hits");
    let cfg = CacheConfig { disk_dir: Some(dir.clone()), memory_limit_bytes: 1 << 20 };
    let calls = AtomicUsize::new(0);
    let c = ShaderCache::new(cfg.clone(), "fp1");
    let a = c.get_or_compile(42, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Ok(fake(42, 1))
    });
    assert_eq!(a.unwrap().function_name, "fn1");
    let b = c.get_or_compile(42, || panic!("must not recompile")).unwrap();
    assert_eq!(b.function_name, "fn1");
    let s = c.stats();
    assert_eq!((s.compiles, s.memory_hits, s.disk_hits), (1, 1, 0));
    assert!(s.disk_bytes_written > 0);
    assert!(c.entry_path(42).unwrap().exists());

    // New process-equivalent: served from disk.
    let c2 = ShaderCache::new(cfg.clone(), "fp1");
    let d = c2.get_or_compile(42, || panic!("must hit disk")).unwrap();
    assert_eq!(*d, fake(42, 1));
    assert_eq!(c2.stats().disk_hits, 1);

    // Different converter fingerprint = different namespace = miss.
    let c3 = ShaderCache::new(cfg, "fp2");
    c3.get_or_compile(42, || Ok(fake(42, 2))).unwrap();
    assert_eq!(c3.stats().compiles, 1);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn concurrent_requests_compile_once() {
    let c = Arc::new(ShaderCache::new(CacheConfig::memory_only(), "fp"));
    let calls = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(16));
    let threads: Vec<_> = (0..16)
        .map(|_| {
            let (c, calls, barrier) = (c.clone(), calls.clone(), barrier.clone());
            std::thread::spawn(move || {
                barrier.wait();
                c.get_or_compile(7, || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(100));
                    Ok(fake(7, 3))
                })
                .unwrap()
            })
        })
        .collect();
    for t in threads {
        assert_eq!(t.join().unwrap().function_name, "fn3");
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let s = c.stats();
    assert_eq!(s.compiles, 1);
    assert_eq!(s.dedup_waits + s.memory_hits, 15);
}

#[test]
fn failures_are_remembered() {
    let c = ShaderCache::new(CacheConfig::memory_only(), "fp");
    let e = Error::Convert { code: ErrorCode::UnsupportedInstruction, message: "nope".into() };
    assert_eq!(c.get_or_compile(9, || Err(e.clone())).unwrap_err(), e);
    assert_eq!(c.get_or_compile(9, || panic!("must not retry")).unwrap_err(), e);
    let s = c.stats();
    assert_eq!((s.compiles, s.compile_failures, s.negative_hits), (1, 1, 1));
    c.clear_memory();
    assert!(c.get_or_compile(9, || Ok(fake(9, 1))).is_ok());
}

#[test]
fn transient_failures_retry_once_then_cache() {
    let c = ShaderCache::new(CacheConfig::memory_only(), "fp");
    let e = Error::Helper("worker crashed".into());
    assert_eq!(c.get_or_compile(5, || Err(e.clone())).unwrap_err(), e);
    assert_eq!(c.get_or_compile(5, || Err(e.clone())).unwrap_err(), e); // the one retry
    assert_eq!(c.get_or_compile(5, || panic!("must not retry twice")).unwrap_err(), e);
    let s = c.stats();
    assert_eq!((s.compiles, s.compile_failures, s.negative_hits), (2, 2, 1));
    // A retry that succeeds is cached normally.
    assert_eq!(c.get_or_compile(6, || Err(Error::Io("disk full".into()))).unwrap_err(), Error::Io("disk full".into()));
    assert!(c.get_or_compile(6, || Ok(fake(6, 1))).is_ok());
    assert!(c.get_or_compile(6, || panic!("cached")).is_ok());
}

#[test]
fn deterministic_failures_persist_on_disk() {
    let dir = tmpdir("failures");
    let cfg = CacheConfig { disk_dir: Some(dir.clone()), memory_limit_bytes: 1 << 20 };
    let e = Error::Convert { code: ErrorCode::UnsupportedInstruction, message: "nope".into() };
    let transient = Error::Helper("worker died".into());
    {
        let c = ShaderCache::new(cfg.clone(), "fp");
        assert_eq!(c.get_or_compile(1, || Err(e.clone())).unwrap_err(), e);
        assert_eq!(c.get_or_compile(2, || Err(transient.clone())).unwrap_err(), transient);
    }
    // A fresh process sees the converter failure without recompiling; the transient one retries.
    let c = ShaderCache::new(cfg, "fp");
    assert_eq!(c.get_or_compile(1, || panic!("must not retry")).unwrap_err(), e);
    assert!(c.get_or_compile(2, || Ok(fake(2, 1))).is_ok());
    let s = c.stats();
    assert_eq!((s.negative_hits, s.compiles), (1, 1));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn corrupt_entries_are_recompiled() {
    let dir = tmpdir("corrupt");
    let cfg = CacheConfig { disk_dir: Some(dir.clone()), memory_limit_bytes: 1 << 20 };
    let c = ShaderCache::new(cfg.clone(), "fp");
    c.get_or_compile(5, || Ok(fake(5, 1))).unwrap();
    let p = c.entry_path(5).unwrap();
    let mut b = std::fs::read(&p).unwrap();
    let n = b.len();
    b[n / 2] ^= 0xFF;
    std::fs::write(&p, b).unwrap();
    let c2 = ShaderCache::new(cfg, "fp");
    let r = c2.get_or_compile(5, || Ok(fake(5, 2))).unwrap();
    assert_eq!(r.function_name, "fn2");
    let s = c2.stats();
    assert_eq!((s.disk_errors, s.compiles, s.disk_hits), (1, 1, 0));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn memory_limit_spills_to_disk() {
    let dir = tmpdir("limit");
    let c = ShaderCache::new(CacheConfig { disk_dir: Some(dir.clone()), memory_limit_bytes: 10 }, "fp");
    c.get_or_compile(1, || Ok(fake(1, 1))).unwrap(); // 7 bytes: kept
    c.get_or_compile(2, || Ok(fake(2, 2))).unwrap(); // would exceed: disk only
    c.get_or_compile(2, || panic!()).unwrap();
    let s = c.stats();
    assert_eq!((s.compiles, s.disk_hits), (2, 1));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn cache_key_sensitivity() {
    let dxil_a = b"DXBC-fake-a".to_vec();
    let dxil_b = b"DXBC-fake-b".to_vec();
    let o = CompileOptions::default();
    let o2 = CompileOptions { compatibility_flags: compat::BOUNDS_CHECK, ..Default::default() };
    let k = |d: &[u8], o: &CompileOptions, e: Option<&str>, fp: &str| {
        let mut r = CompileRequest::new(d, None, o);
        r.entry_point = e;
        r.cache_key(fp)
    };
    let base = k(&dxil_a, &o, None, "fp");
    assert_eq!(base, k(&dxil_a, &o, None, "fp"));
    assert_ne!(base, k(&dxil_b, &o, None, "fp"));
    assert_ne!(base, k(&dxil_a, &o2, None, "fp"));
    assert_ne!(base, k(&dxil_a, &o, Some("main"), "fp"));
    assert_ne!(base, k(&dxil_a, &o, None, "fp2"));
}

#[test]
fn env_config() {
    // Only checks parsing of the documented forms; doesn't touch the real cache.
    std::env::remove_var("PORT_SHADER_CACHE");
    std::env::remove_var("TRANSLATOR_HOME");
    std::env::remove_var("AW2_DRIVE_C");
    std::env::set_var("AW2_SHADER_CACHE", "off");
    assert_eq!(CacheConfig::from_env().disk_dir, None);
    std::env::set_var("AW2_SHADER_CACHE", "/tmp/aw2-x");
    assert_eq!(CacheConfig::from_env().disk_dir, Some("/tmp/aw2-x".into()));
    std::env::set_var("PORT_SHADER_CACHE", "/tmp/port-x");
    assert_eq!(CacheConfig::from_env().disk_dir, Some("/tmp/port-x".into()));
    std::env::remove_var("PORT_SHADER_CACHE");
    std::env::remove_var("AW2_SHADER_CACHE");
    assert!(CacheConfig::from_env().disk_dir.unwrap().ends_with("Translator/ShaderCache"));
}

#[test]
fn fallback_dir_hits_across_different_converter_suffix() {
    // Simulates a prewarmed bundled `Contents/Resources/ShaderCache/ir4.0.1-60039344-1791220727/`
    // (legacy size+mtime suffix) loaded on a new Mac where the ad-hoc-signed bundled converter
    // reports an immutable UUID fingerprint `ir4.0.1-uuid-0102030405060708090a0b0c0d0e0f10` and a
    // fresh empty writable user cache.
    let root = tmpdir("fallback-cross-mac");
    let bundled_root = root.join("bundled");
    let user_root = root.join("user");
    let old_fp = "ir4.0.1-60039344-1791220727";
    let new_fp = "ir4.0.1-uuid-0102030405060708090a0b0c0d0e0f10";

    // Prewarm into bundled root under `old_fp`.
    let seed = ShaderCache::new(CacheConfig { disk_dir: Some(bundled_root.clone()), memory_limit_bytes: 1 << 20 }, old_fp);
    seed.get_or_compile(99, || Ok(fake(99, 7))).unwrap();
    assert!(seed.entry_path(99).unwrap().is_file());

    // Open fresh writable `user_root` with `PORT_SHADER_CACHE_FALLBACK = bundled_root` under `new_fp`.
    let prev_fb = std::env::var_os("PORT_SHADER_CACHE_FALLBACK");
    std::env::set_var("PORT_SHADER_CACHE_FALLBACK", &bundled_root);
    let user_cache = ShaderCache::new(CacheConfig { disk_dir: Some(user_root.clone()), memory_limit_bytes: 1 << 20 }, new_fp);
    match prev_fb {
        Some(v) => std::env::set_var("PORT_SHADER_CACHE_FALLBACK", v),
        None => std::env::remove_var("PORT_SHADER_CACHE_FALLBACK"),
    }

    assert_eq!(user_cache.namespace(), old_fp, "should resolve namespace to bundled prefix match");
    assert!(user_cache.has_cached_entry(99), "has_cached_entry must detect fallback entries");
    let hit = user_cache.get_or_compile(99, || panic!("must hit bundled fallback without recompiling")).unwrap();
    assert_eq!(hit.function_name, "fn7");
    assert_eq!((user_cache.stats().disk_hits, user_cache.stats().compiles), (1, 0));
    assert!(user_cache.entry_path(99).unwrap().is_file(), "hit should hydrate user cache copy");

    let _ = std::fs::remove_dir_all(root);
}
