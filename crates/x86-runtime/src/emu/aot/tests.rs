//! Unit tests for the AOT translation cache (`emu::aot`).

use super::bg::{
    bg_job, clean_exit_code, delete_other_keys, evict, pid_alive, profile_merge, session_check,
    session_lock, touch, Session,
};
use super::builder::{
    build_image, build_image_with_threads, build_image_with_threads_and_limits, WorkerCaches,
};
use super::cfg_discover::{discover, discover_pe_roots, discover_with_threads, FuncMap};
use super::format::{
    codegen_hash, decode, encode, encode_with_version, module_key, read_cache_file,
    read_cache_file_in, read_cached_preferred_base_in, set_flags, AotKey, Image, BUILD_VBASE,
    FLAG_VERIFY_PENDING, FORMAT_VERSION_V5, FORMAT_VERSION_V6, FORMAT_VERSION_V7, MAX_CODE,
};
use super::runtime::{
    install_image, load_module, load_verify, Verify, DIR, FIRST_LOAD_FULL_MAX,
    FIRST_LOAD_SAMPLE_PCT, MAPPED, REVALIDATE,
};
use super::*;
use crate::emu::jit::{self, AOT_STUBS_BYTES};
use crate::emu::BlockCache;
use std::collections::HashMap;
use std::path::PathBuf;

/// `.pdata` map: chained (`UNW_FLAG_CHAININFO`, odd unwind-code count padding, two hops)
/// and indirect entries merge into their root; cyclic / out-of-image chains stay their own
/// root; gaps (leaf code) have no function.
#[test]
fn funcmap_merges_chained_unwind_entries() {
    let mut img = vec![0u8; 0x2000];
    let w16 = |img: &mut Vec<u8>, o: usize, v: u16| img[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |img: &mut Vec<u8>, o: usize, v: u32| img[o..o + 4].copy_from_slice(&v.to_le_bytes());
    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 108, 16);
    let rf = |img: &mut Vec<u8>, o: usize, b: u32, e: u32, u: u32| {
        w32(img, o, b);
        w32(img, o + 4, e);
        w32(img, o + 8, u);
    };
    let pdata = [
        (0x100, 0x180, 0x1800),         // root
        (0x400, 0x420, 0x1810),         // chained -> 0x100
        (0x180, 0x1A0, 0x1820),         // chained (1 code) -> 0x400 -> 0x100
        (0x200, 0x240, 0x1000 | 1),     // indirect -> entry 0 (0x100)
        (0x300, 0x340, 0x1840),         // cyclic chain
        (0x500, 0x520, 0x00FF_FFF0),    // unwind info out of the image
        (0x600, 0x640, 0x1850),         // root (EH handler flag only)
    ];
    for (i, &(b, e, u)) in pdata.iter().enumerate() {
        rf(&mut img, 0x1000 + 12 * i, b, e, u);
    }
    w32(&mut img, opt + 112 + 3 * 8, 0x1000);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 12 * pdata.len() as u32);
    img[0x1800] = 0x01;
    img[0x1810] = 0x01 | 4 << 3;
    rf(&mut img, 0x1814, 0x100, 0x180, 0x1800);
    img[0x1820] = 0x01 | 4 << 3;
    img[0x1822] = 1;
    rf(&mut img, 0x1828, 0x400, 0x420, 0x1810);
    img[0x1840] = 0x01 | 4 << 3;
    rf(&mut img, 0x1844, 0x300, 0x340, 0x1840);
    img[0x1850] = 0x01 | 1 << 3;
    let fm = unsafe { FuncMap::read(img.as_ptr() as usize, img.len()) };
    assert_eq!((fm.ranges.len(), fm.chained, fm.broken, fm.functions()), (7, 3, 2, 4));
    for (rva, want) in [
        (0x100, Some(0x100)),
        (0x17F, Some(0x100)),
        (0x180, Some(0x100)),
        (0x410, Some(0x100)),
        (0x210, Some(0x100)),
        (0x310, Some(0x300)),
        (0x510, Some(0x500)),
        (0x610, Some(0x600)),
        (0x260, None),
        (0x420, None),
        (0x50, None),
    ] {
        assert_eq!(fm.root_of(rva), want, "rva {rva:#x}");
    }
}

/// Fresh scratch directory for the cache-maintenance tests.
#[cfg(test)]
fn test_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("aw2-aot-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// Crash-loop guard: a run whose lock survives (no clean exit) is a strike; the second
/// strike in a row skips AOT for one run (no lock) and starts over; live processes' locks
/// are kept (concurrent instances); a lock with the starting pid is stale (pid reuse).
#[test]
fn aot_session_guard_skips_one_run_after_two_unclean_exits() {
    let d = test_dir("session");
    let lock = |p: u32| session_lock(&d, p).exists();
    let dead = |_: u32| false;
    assert_eq!(session_check(&d, 100, dead), Session::Go { strikes: 0, stale: 0 });
    assert!(lock(100));
    // 100 crashed (lock left): first strike.
    assert_eq!(session_check(&d, 101, dead), Session::Go { strikes: 1, stale: 1 });
    assert!(!lock(100) && lock(101));
    // 101 crashed too: this run goes without AOT and creates no lock.
    assert_eq!(session_check(&d, 102, dead), Session::Skip { stale: 1 });
    assert!(!lock(101) && !lock(102));
    // The run after the skipped one starts over.
    assert_eq!(session_check(&d, 103, dead), Session::Go { strikes: 0, stale: 0 });
    // 103 exits cleanly (session_end removes its lock): no strike.
    std::fs::remove_file(session_lock(&d, 103)).unwrap();
    assert_eq!(session_check(&d, 104, dead), Session::Go { strikes: 0, stale: 0 });
    // 104 still running: a concurrent start leaves its lock alone.
    assert_eq!(session_check(&d, 105, |p| p == 104), Session::Go { strikes: 0, stale: 0 });
    assert!(lock(104) && lock(105));
    // Both crash; a later start whose pid was reused from one of them counts both.
    assert_eq!(session_check(&d, 105, dead), Session::Go { strikes: 1, stale: 2 });
    assert!(lock(105) && !lock(104));
    // Unrelated files are ignored.
    std::fs::write(d.join("session-x.lock"), "").unwrap();
    std::fs::remove_file(session_lock(&d, 105)).unwrap();
    assert_eq!(session_check(&d, 106, dead), Session::Go { strikes: 0, stale: 0 });
    assert!(d.join("session-x.lock").exists());
    assert!(clean_exit_code(0) && clean_exit_code(1) && !clean_exit_code(3) && !clean_exit_code(255));
    assert!(!clean_exit_code(0xC000_0005) && !clean_exit_code(0x8000_0003));
    assert!(pid_alive(std::process::id()) && !pid_alive(0));
    let _ = std::fs::remove_dir_all(&d);
}

/// Sweep: files unused for longer than the age limit go first, then the least recently
/// used until the rest fits the cap; only cache files (`*.aot`, `profile/*.tsv`, leftover
/// `*.tmp<pid>`) are candidates.
#[test]
fn aot_evict_removes_old_then_least_recently_used_over_cap() {
    let d = test_dir("evict");
    std::fs::create_dir_all(d.join("profile")).unwrap();
    let now = std::time::SystemTime::now();
    let day = std::time::Duration::from_secs(86400);
    let put = |rel: &str, len: usize, age: std::time::Duration| {
        let p = d.join(rel);
        std::fs::write(&p, vec![0u8; len]).unwrap();
        std::fs::File::options().write(true).open(&p).unwrap().set_modified(now - age).unwrap();
    };
    put("a.dll-0000000000000001.aot", 100, day * 40);
    put("b.dll-0000000000000002.aot", 200, day * 2);
    put("c.dll-0000000000000003.aot", 300, day);
    put("profile/c.dll-0000000000000003.tsv", 50, day / 24);
    put("c.dll-0000000000000004.tmp4242", 10, day * 3);
    put("session-1.lock", 5, day * 90);
    put("session.strikes", 5, day * 90);
    put("notes.txt", 5, day * 90);
    let r = evict(&d, now, day * 30, 400);
    // a: too old; then over the cap (560): the tmp file (3 days), b (2 days) -> 350 bytes.
    assert_eq!(r, [3, 310, 2, 350]);
    for (rel, there) in [
        ("a.dll-0000000000000001.aot", false),
        ("b.dll-0000000000000002.aot", false),
        ("c.dll-0000000000000004.tmp4242", false),
        ("c.dll-0000000000000003.aot", true),
        ("profile/c.dll-0000000000000003.tsv", true),
        ("session-1.lock", true),
        ("session.strikes", true),
        ("notes.txt", true),
    ] {
        assert_eq!(d.join(rel).exists(), there, "{rel}");
    }
    // Within limits: nothing to do.
    assert_eq!(evict(&d, now, day * 30, 400), [0, 0, 2, 350]);
    // touch() makes a file the most recently used.
    touch(&d.join("c.dll-0000000000000003.aot"));
    assert_eq!(evict(&d, now + std::time::Duration::from_secs(1), day * 30, 300), [1, 50, 1, 300]);
    assert!(d.join("c.dll-0000000000000003.aot").exists());
    let _ = std::fs::remove_dir_all(&d);
}

/// After a write, only the same module's caches with other keys are removed.
#[test]
fn aot_delete_other_keys_only_touches_the_module() {
    let d = test_dir("keys");
    let names = [
        "m.dll-0123456789abcdef.aot",
        "m.dll-fedcba9876543210.aot",
        "m.dll-0123.aot",
        "m.dll-0123456789abcdeg.aot",
        "m.dll.x-0123456789abcdef.aot",
        "n.dll-0123456789abcdef.aot",
        "m.dll-0123456789abcdef.tsv",
    ];
    for n in names {
        std::fs::write(d.join(n), "x").unwrap();
    }
    let keep = d.join("m.dll-fedcba9876543210.aot");
    assert_eq!(delete_other_keys(&d, "m.dll", &keep), 1);
    for n in names {
        assert_eq!(d.join(n).exists(), n != "m.dll-0123456789abcdef.aot", "{n}");
    }
    assert_eq!(delete_other_keys(&d, "m.dll", &keep), 0);
    let _ = std::fs::remove_dir_all(&d);
}

/// Header flags round-trip and can be rewritten in place (checksum follows); the
/// first-load verify selection is full below the block limit, a ~2% sample above it.
#[test]
fn aot_verify_pending_flag_and_first_load_selection() {
    let key = AotKey { image_base: 0x1_4000_0000, image_size: 0x1000, codegen_hash: 7, ..AotKey::default() };
    let mut b = encode("m.dll", &key, &Image::default(), FLAG_VERIFY_PENDING);
    let (n, k, _, f) = decode(&b).expect("decodes");
    assert!(n == "m.dll" && k == key && f == FLAG_VERIFY_PENDING);
    assert!(set_flags(&mut b, 0));
    let (n, k, _, f) = decode(&b).expect("decodes after the flag rewrite");
    assert!(n == "m.dll" && k == key && f == 0);
    b[20] ^= 1;
    assert!(decode(&b).is_none(), "checksum must cover the header");
    assert!(!set_flags(&mut [0u8; 8], 0));
    assert_eq!(load_verify(false, false, 10, 1), Verify::None);
    assert_eq!(load_verify(true, false, 1 << 20, 1), Verify::All);
    assert_eq!(load_verify(false, true, FIRST_LOAD_FULL_MAX - 1, 1), Verify::All);
    let s = load_verify(false, true, FIRST_LOAD_FULL_MAX, 1);
    assert_eq!(s, Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed: 1 });
    let picked = (0..100_000u64).filter(|&i| s.picks(0x1_4000_0000 + i * 16)).count();
    assert!((1_500..=2_500).contains(&picked), "{picked} of 100000 sampled");
    let other = Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed: 2 };
    assert!((0..100_000u64).any(|i| s.picks(i * 16) != other.picks(i * 16)), "seed must change the sample");
}

/// Profile merge adds counts and halves them (rounding up, once per merge) past the cap.
#[test]
fn aot_profile_merge_halves_past_the_cap() {
    let mut all: HashMap<u32, u64> = [(1, 5), (2, 1)].into_iter().collect();
    assert!(!profile_merge(&mut all, [(2, 2), (3, 1)]));
    assert_eq!((all[&1], all[&2], all[&3]), (5, 3, 1));
    // Total 2^32 + 9: halved to 2^31 + 3, 2, 1.
    assert!(profile_merge(&mut all, [(1, 1 << 32)]));
    assert_eq!((all[&1], all[&2], all[&3]), ((1u64 << 31) + 3, 2, 1));
    assert!(!profile_merge(&mut all, []));
    // Still over the cap after one halving: halved again at the next merge.
    assert!(profile_merge(&mut all, [(2, 3 << 32)]));
    assert!(profile_merge(&mut all, []));
    assert!(!profile_merge(&mut all, []));
}

/// Profile-free offline prewarm on an in-memory synthetic PE32+ image:
/// discovers roots from `.pdata`, `AddressOfEntryPoint`, and Export Directory (skipping
/// forwarders and non-code data exports), expands static CFG closure, writes the `.aot`
/// cache file to `PORT_AOT_DIR`, and loads it back cleanly via `activate()`.
#[test]
fn aot_prewarm_synthetic_pe_writes_and_loads_via_activate() {
    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            struct EnvGuard {
                port_dir: Option<std::ffi::OsString>,
                aw2_dir: Option<std::ffi::OsString>,
            }
            impl Drop for EnvGuard {
                fn drop(&mut self) {
                    match &self.port_dir {
                        Some(v) => std::env::set_var("PORT_AOT_DIR", v),
                        None => std::env::remove_var("PORT_AOT_DIR"),
                    }
                    match &self.aw2_dir {
                        Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
                        None => std::env::remove_var("AW2_DLL_AOT_DIR"),
                    }
                }
            }
            let _env_guard = EnvGuard {
                port_dir: std::env::var_os("PORT_AOT_DIR"),
                aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
            };

            let d = test_dir("prewarm");
            std::env::set_var("PORT_AOT_DIR", &d);
            std::env::remove_var("AW2_DLL_AOT_DIR");

            // Synthetic PE32+ image (16 KiB):
            //   .text  @ RVA 0x1000..0x2000 (code + trailing padding for BlockCache::fill)
            //   .rdata @ RVA 0x2000..0x3000 (Export Directory @ 0x2000, .pdata @ 0x2200)
            let mut img = vec![0u8; 0x4000];
            let w16 = |buf: &mut [u8], o: usize, v: u16| buf[o..o + 2].copy_from_slice(&v.to_le_bytes());
            let w32 = |buf: &mut [u8], o: usize, v: u32| buf[o..o + 4].copy_from_slice(&v.to_le_bytes());

            w16(&mut img, 0, 0x5A4D); // MZ
            w32(&mut img, 0x3C, 0x40); // e_lfanew
            w32(&mut img, 0x40, 0x0000_4550); // PE\0\0
            w16(&mut img, 0x44, 0x8664); // AMD64
            w16(&mut img, 0x46, 2); // NumberOfSections
            w16(&mut img, 0x54, 240); // SizeOfOptionalHeader

            let opt = 0x58;
            w16(&mut img, opt, 0x20B); // PE32+
            w32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint
            w32(&mut img, opt + 56, 0x4000); // SizeOfImage
            w32(&mut img, opt + 60, 0x400); // SizeOfHeaders
            w32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes
            // DataDirectory[0]: Export Directory @ 0x2000 (size 0x80)
            w32(&mut img, opt + 112, 0x2000);
            w32(&mut img, opt + 112 + 4, 0x80);
            // DataDirectory[3]: Exception Directory (.pdata) @ 0x2200 (2 entries * 12 = 24 bytes)
            w32(&mut img, opt + 112 + 3 * 8, 0x2200);
            w32(&mut img, opt + 112 + 3 * 8 + 4, 24);

            // Section table at opt + 240 = 0x148
            let sec0 = opt + 240;
            img[sec0..sec0 + 5].copy_from_slice(b".text");
            w32(&mut img, sec0 + 8, 0x1000); // VirtualSize
            w32(&mut img, sec0 + 12, 0x1000); // VirtualAddress
            w32(&mut img, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

            let sec1 = sec0 + 40;
            img[sec1..sec1 + 6].copy_from_slice(b".rdata");
            w32(&mut img, sec1 + 8, 0x1000);
            w32(&mut img, sec1 + 12, 0x2000);
            w32(&mut img, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA

            // Code in .text:
            //   0x1000 (EntryPoint + .pdata[0]): test ecx, ecx; jz +6 (0x100a); mov eax, 1; ret; mov eax, 2; ret
            let fn0: &[u8] = &[0x85, 0xC9, 0x74, 0x06, 0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3, 0xB8, 0x02, 0x00, 0x00, 0x00, 0xC3];
            img[0x1000..0x1000 + fn0.len()].copy_from_slice(fn0);
            //   0x1040 (Export Directory ONLY): lea eax, [rcx + rdx]; ret
            let fn1: &[u8] = &[0x8D, 0x04, 0x11, 0xC3];
            img[0x1040..0x1040 + fn1.len()].copy_from_slice(fn1);
            //   0x1080 (.pdata[1] ONLY): add rcx, 7; mov rax, rcx; ret
            let fn2: &[u8] = &[0x48, 0x83, 0xC1, 0x07, 0x48, 0x89, 0xC8, 0xC3];
            img[0x1080..0x1080 + fn2.len()].copy_from_slice(fn2);

            // Export Directory @ 0x2000:
            w32(&mut img, 0x2000 + 20, 3); // NumberOfFunctions
            w32(&mut img, 0x2000 + 28, 0x2030); // AddressOfFunctions
            w32(&mut img, 0x2030, 0x1040); // exported code function
            w32(&mut img, 0x2034, 0x2060); // forwarder inside export directory (must be skipped)
            w32(&mut img, 0x2038, 0x2400); // data export in .rdata (must be skipped)

            // .pdata @ 0x2200:
            w32(&mut img, 0x2200, 0x1000);
            w32(&mut img, 0x2204, 0x1020);
            w32(&mut img, 0x2208, 0x2300);
            w32(&mut img, 0x220C, 0x1080);
            w32(&mut img, 0x2210, 0x10A0);
            w32(&mut img, 0x2214, 0x2300);
            img[0x2300] = 0x01; // UNWIND_INFO version 1, flags 0

            let base = img.as_ptr() as usize;
            let size = img.len();
            assert!((0x1_0000..0x8000_0000_0000).contains(&base), "unexpected test buffer address {base:#x}");

            // Root discovery covers .pdata + EntryPoint + Exports (excluding forwarders & data)
            // and respects deterministic max_roots capping.
            assert_eq!(unsafe { discover_pe_roots(base, size, 0) }, vec![0x1000, 0x1040, 0x1080]);
            assert_eq!(unsafe { discover_pe_roots(base, size, 2) }, vec![0x1000, 0x1040]);

            let mod_name = "synth_prewarm_test.dll";
            module_mapped(mod_name, base, size);

            let reports = aot_prewarm_all_mapped(0);
            let rep = reports
                .iter()
                .find(|r| r.module == mod_name)
                .expect("synthetic module should appear in prewarm report");
            assert_eq!(rep.roots, 3, "expected 3 unique code roots");
            assert_eq!(rep.blocks, 5, "expected 5 translated blocks (3 roots + 2 CFG successors)");
            assert!(rep.code_bytes > 0);
            assert!(rep.skipped_reason.is_none(), "unexpected skip: {:?}", rep.skipped_reason);
            let cache_path = rep.cache_path.as_ref().expect("cache file path");
            assert!(cache_path.is_file(), "expected .aot cache file at {}", cache_path.display());
            assert!(cache_path.starts_with(&d), "cache file should live inside PORT_AOT_DIR");

            // Load back via activate() and confirm all 5 blocks are live in the AOT directory.
            TEST_AOT.with(|c| c.set(true));
            activate();
            for rva in [0x1000u64, 0x1004, 0x100a, 0x1040, 0x1080] {
                assert!(
                    find(base as u64 + rva).is_some(),
                    "prewarmed block at RVA {rva:#x} was not loaded by activate()"
                );
            }

            // Tear down global test registrations so the synthetic buffer can be dropped safely.
            TEST_AOT.with(|c| c.set(false));
            if let Ok(mut v) = MAPPED.lock() {
                v.retain(|m| m.name != mod_name);
            }
            {
                let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
                let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
                if let Some(dir) = g.as_mut() {
                    let gone: Vec<u32> = (0..dir.modules.len() as u32)
                        .filter(|&i| dir.modules[i as usize].name == mod_name && !dir.modules[i as usize].gone)
                        .collect();
                    for &i in &gone {
                        dir.modules[i as usize].gone = true;
                    }
                    let ips: Vec<u64> = dir.map.iter().filter(|(_, &(m, _))| gone.contains(&m)).map(|(&ip, _)| ip).collect();
                    dir.map.retain(|_, &mut (m, _)| !gone.contains(&m));
                    for ip in ips {
                        unsafe { dir.probe_sync(ip) };
                    }
                    for v in dir.incoming.values_mut() {
                        v.retain(|&(m, _)| !gone.contains(&m));
                    }
                }
            }
            let _ = std::fs::remove_dir_all(&d);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Default AOT cache directory precedence: `PORT_AOT_DIR` > `AW2_DLL_AOT_DIR` >
/// `~/Library/Caches/port-factory/aot`, with legacy `~/Library/Caches/aw2-port/aot` fallback
/// only when neither override is set.
#[test]
fn aot_cache_dir_env_precedence_and_legacy_fallback() {
    struct EnvGuard {
        port_dir: Option<std::ffi::OsString>,
        aw2_dir: Option<std::ffi::OsString>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.port_dir {
                Some(v) => std::env::set_var("PORT_AOT_DIR", v),
                None => std::env::remove_var("PORT_AOT_DIR"),
            }
            match &self.aw2_dir {
                Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
                None => std::env::remove_var("AW2_DLL_AOT_DIR"),
            }
        }
    }
    let _guard = EnvGuard {
        port_dir: std::env::var_os("PORT_AOT_DIR"),
        aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
    };

    std::env::remove_var("PORT_AOT_DIR");
    std::env::remove_var("AW2_DLL_AOT_DIR");
    assert!(cache_dir().ends_with("Library/Caches/port-factory/aot"));
    assert!(legacy_cache_dir().is_some_and(|p| p.ends_with("Library/Caches/aw2-port/aot")));

    std::env::set_var("AW2_DLL_AOT_DIR", "/tmp/legacy-aot-override");
    assert_eq!(cache_dir(), PathBuf::from("/tmp/legacy-aot-override"));
    assert!(legacy_cache_dir().is_none());

    std::env::set_var("PORT_AOT_DIR", "/tmp/port-aot-override");
    assert_eq!(cache_dir(), PathBuf::from("/tmp/port-aot-override"));
    assert!(legacy_cache_dir().is_none());

    // Verify graceful read fallback from primary dir to legacy dir.
    let root = test_dir("legacy-fallback");
    let primary = root.join("port-factory-aot");
    let legacy = root.join("aw2-port-aot");
    std::fs::create_dir_all(&primary).unwrap();
    std::fs::create_dir_all(&legacy).unwrap();
    let key = AotKey { image_base: 0x1_8000_0000, image_size: 0x2000, codegen_hash: 42, ..AotKey::default() };
    let fname = format!("sample.dll-{:016x}.aot", key.id());
    std::fs::write(legacy.join(&fname), b"legacy-payload").unwrap();

    let (hit_path, hit) = read_cache_file_in(&primary, std::slice::from_ref(&legacy), "sample.dll", &key);
    assert_eq!(hit_path, legacy.join(&fname));
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"legacy-payload"[..], true)));

    // Primary takes precedence once present.
    std::fs::write(primary.join(&fname), b"primary-payload").unwrap();
    let (hit_path, hit) = read_cache_file_in(&primary, std::slice::from_ref(&legacy), "sample.dll", &key);
    assert_eq!(hit_path, primary.join(&fname));
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"primary-payload"[..], false)));

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn aot_read_cache_file_loads_from_readonly_fallback_without_writing() {
    use std::os::unix::fs::PermissionsExt;

    struct EnvGuard {
        port_dir: Option<std::ffi::OsString>,
        aw2_dir: Option<std::ffi::OsString>,
        fb_dir: Option<std::ffi::OsString>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, v) in [
                ("PORT_AOT_DIR", &self.port_dir),
                ("AW2_DLL_AOT_DIR", &self.aw2_dir),
                ("PORT_AOT_FALLBACK_DIR", &self.fb_dir),
            ] {
                match v {
                    Some(val) => std::env::set_var(k, val),
                    None => std::env::remove_var(k),
                }
            }
        }
    }
    struct PermsRestore(PathBuf, PathBuf);
    impl Drop for PermsRestore {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(0o755));
            let _ = std::fs::set_permissions(&self.1, std::fs::Permissions::from_mode(0o644));
        }
    }

    let root = test_dir("readonly-fallback");
    let primary = root.join("writable-user");
    let fallback = root.join("readonly-bundle");
    std::fs::create_dir_all(&primary).unwrap();
    std::fs::create_dir_all(&fallback).unwrap();

    let key = AotKey { image_base: 0x1_9000_0000, image_size: 0x1000, codegen_hash: 99, ..AotKey::default() };
    let fname = format!("bundled.dll-{:016x}.aot", key.id());
    let fb_file = fallback.join(&fname);
    std::fs::write(&fb_file, b"bundled-aot-bytes").unwrap();
    let mtime_before = std::fs::metadata(&fb_file).unwrap().modified().unwrap();

    // Lock the fallback file + directory read-only so any accidental write/touch fails visibly.
    std::fs::set_permissions(&fb_file, std::fs::Permissions::from_mode(0o444)).unwrap();
    std::fs::set_permissions(&fallback, std::fs::Permissions::from_mode(0o555)).unwrap();
    let _restore = PermsRestore(fallback.clone(), fb_file.clone());

    let _env = EnvGuard {
        port_dir: std::env::var_os("PORT_AOT_DIR"),
        aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
        fb_dir: std::env::var_os("PORT_AOT_FALLBACK_DIR"),
    };
    std::env::set_var("PORT_AOT_DIR", &primary);
    std::env::remove_var("AW2_DLL_AOT_DIR");
    std::env::set_var("PORT_AOT_FALLBACK_DIR", &fallback);

    let (hit_path, hit) = read_cache_file("bundled.dll", &key);
    assert_eq!(hit_path, fb_file);
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"bundled-aot-bytes"[..], true)));

    // Verify zero mutation inside the read-only fallback directory.
    let entries: Vec<_> = std::fs::read_dir(&fallback).unwrap().filter_map(|e| e.ok()).collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(std::fs::metadata(&fb_file).unwrap().modified().unwrap(), mtime_before);

    drop(_env);
    drop(_restore);
    let _ = std::fs::remove_dir_all(&root);
}

// ---------------------------------------------------------------------------------------------
// Helpers & tests for parallel prewarm, jump-table CFG discovery, and multi-module verification
// ---------------------------------------------------------------------------------------------

#[cfg(test)]
struct TestEnvGuard {
    saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

#[cfg(test)]
impl TestEnvGuard {
    fn new(vars: &[&'static str]) -> Self {
        let saved = vars.iter().map(|&k| (k, std::env::var_os(k))).collect();
        Self { saved }
    }
}

#[cfg(test)]
impl Drop for TestEnvGuard {
    fn drop(&mut self) {
        for (k, v) in &self.saved {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
    }
}

#[cfg(test)]
struct TestMappedBuf {
    ptr: *mut u8,
    len: usize,
}

#[cfg(test)]
impl TestMappedBuf {
    /// Allocate an anonymous RW page-aligned buffer at a realistic 64-bit guest VA
    /// (`>= 0x1_0000_0000`, `< 1 << 47`) suitable for synthetic PE images and IAT slots.
    fn alloc(size: usize, hint: usize) -> Self {
        extern "C" {
            fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
            fn munmap(addr: *mut u8, len: usize) -> i32;
        }
        let _ = munmap;
        let len = (size + 0x3FFF) & !0x3FFF;
        // MAP_PRIVATE (0x2) | MAP_ANON (0x1000), PROT_READ | PROT_WRITE (3).
        let p = unsafe { mmap(hint as *mut u8, len, 3, 0x2 | 0x1000, -1, 0) };
        assert!(!p.is_null() && p as isize != -1, "test mmap({len:#x}) failed");
        let addr = p as usize;
        assert!(
            (0x1_0000_0000..1usize << 47).contains(&addr),
            "test mmap landed outside expected 64-bit user range: {addr:#x}"
        );
        Self { ptr: p, len }
    }

    fn base(&self) -> usize {
        self.ptr as usize
    }

    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

#[cfg(test)]
impl Drop for TestMappedBuf {
    fn drop(&mut self) {
        extern "C" {
            fn munmap(addr: *mut u8, len: usize) -> i32;
        }
        unsafe {
            munmap(self.ptr, self.len);
        }
    }
}

/// Write a minimal valid PE32+ header into `buf` with two sections:
/// `.text` at `[text_rva, text_rva + text_size)` and `.rdata` at `[rdata_rva, rdata_rva + rdata_size)`.
#[cfg(test)]
fn init_synth_pe(
    buf: &mut [u8],
    preferred_base: u64,
    entry_rva: u32,
    text_rva: u32,
    text_size: u32,
    rdata_rva: u32,
    rdata_size: u32,
    export_dir: Option<(u32, u32)>,
    pdata_dir: Option<(u32, u32)>,
) {
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(buf, 0, 0x5A4D); // MZ
    w32(buf, 0x3C, 0x40); // e_lfanew
    w32(buf, 0x40, 0x0000_4550); // PE\0\0
    w16(buf, 0x44, 0x8664); // AMD64
    w16(buf, 0x46, 2); // NumberOfSections
    w16(buf, 0x54, 240); // SizeOfOptionalHeader

    let opt = 0x58;
    w16(buf, opt, 0x20B); // PE32+
    w32(buf, opt + 16, entry_rva); // AddressOfEntryPoint
    w64(buf, opt + 24, preferred_base); // ImageBase
    w32(buf, opt + 56, buf.len() as u32); // SizeOfImage
    w32(buf, opt + 60, 0x400); // SizeOfHeaders
    w32(buf, opt + 108, 16); // NumberOfRvaAndSizes
    if let Some((rva, len)) = export_dir {
        w32(buf, opt + 112, rva);
        w32(buf, opt + 112 + 4, len);
    }
    if let Some((rva, len)) = pdata_dir {
        w32(buf, opt + 112 + 3 * 8, rva);
        w32(buf, opt + 112 + 3 * 8 + 4, len);
    }

    let sec0 = opt + 240;
    buf[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(buf, sec0 + 8, text_size);
    w32(buf, sec0 + 12, text_rva);
    w32(buf, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

    let sec1 = sec0 + 40;
    buf[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(buf, sec1 + 8, rdata_size);
    w32(buf, sec1 + 12, rdata_rva);
    w32(buf, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA
}

#[cfg(test)]
fn teardown_test_modules(names: &[&str]) {
    if let Ok(mut v) = MAPPED.lock() {
        v.retain(|m| !names.iter().any(|&n| m.name == n));
    }
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    if let Some(dir) = g.as_mut() {
        let gone: Vec<u32> = (0..dir.modules.len() as u32)
            .filter(|&i| names.iter().any(|&n| dir.modules[i as usize].name == n) && !dir.modules[i as usize].gone)
            .collect();
        for &i in &gone {
            dir.modules[i as usize].gone = true;
        }
        let ips: Vec<u64> = dir.map.iter().filter(|(_, &(m, _))| gone.contains(&m)).map(|(&ip, _)| ip).collect();
        dir.map.retain(|_, &mut (m, _)| !gone.contains(&m));
        for ip in ips {
            unsafe { dir.probe_sync(ip) };
        }
        for v in dir.incoming.values_mut() {
            v.retain(|&(m, _)| !gone.contains(&m));
        }
    }
}

/// Helper: encode a 7-byte RIP-relative `lea reg, [rip + disp32]` pointing to `target_va`.
#[cfg(test)]
fn emit_lea_rip(code: &mut Vec<u8>, code_base_va: u64, reg: u8, target_va: u64) {
    let rex = 0x48 | ((reg >> 3) << 2); // REX.W + REX.R
    let modrm = ((reg & 7) << 3) | 0x05; // mod=00, rm=101 (RIP-relative)
    let next_ip = code_base_va + code.len() as u64 + 7;
    let disp = (target_va as i64 - next_ip as i64) as i32;
    code.push(rex);
    code.push(0x8D);
    code.push(modrm);
    code.extend_from_slice(&disp.to_le_bytes());
}

/// Helper: encode a 6-byte RIP-relative `call qword ptr [rip + disp32]` targeting `slot_va`.
#[cfg(test)]
fn emit_call_rip_slot(code: &mut Vec<u8>, code_base_va: u64, slot_va: u64) {
    let next_ip = code_base_va + code.len() as u64 + 6;
    let disp = (slot_va as i64 - next_ip as i64) as i32;
    code.extend_from_slice(&[0xFF, 0x15]);
    code.extend_from_slice(&disp.to_le_bytes());
}

/// Pattern 1 (MSVC / Clang `rva32` image-base-relative switch tables):
/// - 1A: `cmp ecx, 3; lea rdx, [rip + __ImageBase]; ja default; mov eax, [rdx + rcx*4 + rva_tbl]; add rax, rdx; jmp rax`
/// - 1B: separate `lea rBase, [rip + __ImageBase]` + `lea r8, [rip + tbl]` + `movsxd rax, [r8 + rcx*4]` + `add rax, rBase` + `jmp rax`
/// - 1C: MSVC hoisted `__ImageBase` (`r11` undefined in dispatch block, `[r11 + rcx*4 + rva_tbl]` + `add rax, r11; jmp rax`)
#[test]
fn aot_jump_table_discovery_msvc_rva32() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3000_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 36)));

    // --- Function 1A @ 0x1000 (MSVC classic with intervening `lea` between `cmp` and `ja`) ---
    let f1a_cases: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let f1a_default: u32 = 0x10C0;
    let tbl_1a_rva: u32 = 0x4100;
    for (i, &rva) in f1a_cases.iter().enumerate() {
        buf[tbl_1a_rva as usize + i * 4..tbl_1a_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    // Trailing garbage word (must NOT be followed because `cmp ecx, 3; ja` caps at 4).
    buf[tbl_1a_rva as usize + 16..tbl_1a_rva as usize + 20].copy_from_slice(&0x10D0u32.to_le_bytes());

    let mut c1a = Vec::new();
    // mov eax, ecx; cmp eax, 3; lea rdx, [rip + __ImageBase]; ja default
    c1a.extend_from_slice(&[0x89, 0xC8]); // mov eax, ecx
    c1a.extend_from_slice(&[0x83, 0xF8, 0x03]); // cmp eax, 3
    emit_lea_rip(&mut c1a, base64 + 0x1000, 2 /* rdx */, base64);
    let ja_next = 0x1000 + c1a.len() as u32 + 6;
    c1a.extend_from_slice(&[0x0F, 0x87]);
    c1a.extend_from_slice(&((f1a_default as i32) - (ja_next as i32)).to_le_bytes());
    // Dispatch block: mov eax, dword ptr [rdx + rax*4 + tbl_1a_rva]; add rax, rdx; jmp rax
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + tbl_1a_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    let d1a = a.assemble(base64 + 0x1000 + c1a.len() as u64).unwrap();
    c1a.extend_from_slice(&d1a);
    buf[0x1000..0x1000 + c1a.len()].copy_from_slice(&c1a);
    for (i, &rva) in f1a_cases.iter().enumerate() {
        let b = [0xB8, (10 + i as u8), 0, 0, 0, 0xC3]; // mov eax, 10+i; ret
        buf[rva as usize..rva as usize + b.len()].copy_from_slice(&b);
    }
    buf[f1a_default as usize..f1a_default as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);
    buf[0x10D0..0x10D6].copy_from_slice(&[0xB8, 77, 0, 0, 0, 0xC3]); // out-of-bound decoy

    // --- Function 1B @ 0x1200 (Separate `lea rBase` and `lea rTbl`, `movsxd`) ---
    let f1b_cases: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let f1b_default: u32 = 0x12C0;
    let tbl_1b_rva: u32 = 0x4140;
    for (i, &rva) in f1b_cases.iter().enumerate() {
        buf[tbl_1b_rva as usize + i * 4..tbl_1b_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c1b = Vec::new();
    c1b.extend_from_slice(&[0x83, 0xF9, 0x04]); // cmp ecx, 4
    let jae_next = 0x1200 + c1b.len() as u32 + 6;
    c1b.extend_from_slice(&[0x0F, 0x83]); // jae default
    c1b.extend_from_slice(&((f1b_default as i32) - (jae_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c1b, base64 + 0x1200, 2 /* rdx */, base64);
    emit_lea_rip(&mut c1b, base64 + 0x1200, 8 /* r8 */, base64 + tbl_1b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c1b.extend_from_slice(&a.assemble(base64 + 0x1200 + c1b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c1b.len()].copy_from_slice(&c1b);
    for (i, &rva) in f1b_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[f1b_default as usize..f1b_default as usize + 6].copy_from_slice(&[0xB8, 98, 0, 0, 0, 0xC3]);

    // --- Function 1C @ 0x1400 (MSVC hoisted `__ImageBase` in r11, undefined in this block) ---
    let f1c_cases: [u32; 3] = [0x1480, 0x1490, 0x14A0];
    let f1c_default: u32 = 0x14B0;
    let tbl_1c_rva: u32 = 0x4180;
    for (i, &rva) in f1c_cases.iter().enumerate() {
        buf[tbl_1c_rva as usize + i * 4..tbl_1c_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c1c = Vec::new();
    c1c.extend_from_slice(&[0x83, 0xF9, 0x02]); // cmp ecx, 2
    let ja_next = 0x1400 + c1c.len() as u32 + 6;
    c1c.extend_from_slice(&[0x0F, 0x87]); // ja default
    c1c.extend_from_slice(&((f1c_default as i32) - (ja_next as i32)).to_le_bytes());
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, dword_ptr(r11 + rcx * 4 + tbl_1c_rva as i32)).unwrap();
    a.add(rax, r11).unwrap();
    a.jmp(rax).unwrap();
    c1c.extend_from_slice(&a.assemble(base64 + 0x1400 + c1c.len() as u64).unwrap());
    buf[0x1400..0x1400 + c1c.len()].copy_from_slice(&c1c);
    for (i, &rva) in f1c_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (30 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[f1c_default as usize..f1c_default as usize + 6].copy_from_slice(&[0xB8, 97, 0, 0, 0, 0xC3]);

    // .pdata entries for the 3 functions.
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300), (0x1400, 0x1500)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01; // UNWIND_INFO v1

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200, 0x1400], 2) };
    assert_eq!(clos.jt_tables, 3, "expected 3 MSVC rva32 jump tables discovered");
    assert_eq!(clos.jt_targets, 11, "expected 4 + 4 + 3 = 11 unique jump-table targets");
    for &rva in f1a_cases.iter().chain(f1b_cases.iter()).chain(f1c_cases.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing switch case RVA {rva:#x}");
    }
    assert!(
        !ips.contains(&(base64 + 0x10D0)),
        "out-of-bounds 5th entry (past `cmp eax, 3; ja`) must NOT be followed"
    );
}

/// Pattern 2 (direct 64-bit pointer tables):
/// - 2A: `cmp ecx, 3; ja default; lea r8, [rip + tbl]; jmp qword ptr [r8 + rcx*8]`
/// - 2B: unbounded `lea r8, [rip + tbl]; mov rax, qword ptr [r8 + rcx*8]; jmp rax` (stops at 0)
#[test]
fn aot_jump_table_discovery_direct_ptr64() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3100_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 24)));

    // --- 2A @ 0x1000: bounded `jmp qword ptr [r8 + rcx*8]` ---
    let f2a_cases: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let f2a_default: u32 = 0x10C0;
    let tbl_2a_rva: u32 = 0x4100;
    for (i, &rva) in f2a_cases.iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_2a_rva as usize + i * 8..tbl_2a_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    let mut c2a = Vec::new();
    c2a.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
    let ja_next = 0x1000 + c2a.len() as u32 + 6;
    c2a.extend_from_slice(&[0x0F, 0x87]);
    c2a.extend_from_slice(&((f2a_default as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c2a, base64 + 0x1000, 8 /* r8 */, base64 + tbl_2a_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.jmp(qword_ptr(r8 + rcx * 8)).unwrap();
    c2a.extend_from_slice(&a.assemble(base64 + 0x1000 + c2a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c2a.len()].copy_from_slice(&c2a);
    for (i, &rva) in f2a_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[f2a_default as usize..f2a_default as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 2B @ 0x1200: unbounded `mov rax, qword ptr [r8 + rcx*8]; jmp rax` (stops at zero entry) ---
    let f2b_cases: [u32; 3] = [0x1280, 0x1290, 0x12A0];
    let tbl_2b_rva: u32 = 0x4180;
    for (i, &rva) in f2b_cases.iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_2b_rva as usize + i * 8..tbl_2b_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    // Entry 3 is 0 (terminates conservative scan); entry 4 is a decoy that must not be reached.
    let decoy = base64 + 0x12B0;
    buf[tbl_2b_rva as usize + 32..tbl_2b_rva as usize + 40].copy_from_slice(&decoy.to_le_bytes());
    let mut c2b = Vec::new();
    emit_lea_rip(&mut c2b, base64 + 0x1200, 8 /* r8 */, base64 + tbl_2b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rax, qword_ptr(r8 + rcx * 8)).unwrap();
    a.jmp(rax).unwrap();
    c2b.extend_from_slice(&a.assemble(base64 + 0x1200 + c2b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c2b.len()].copy_from_slice(&c2b);
    for (i, &rva) in f2b_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (10 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[0x12B0..0x12B6].copy_from_slice(&[0xB8, 77, 0, 0, 0, 0xC3]);

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 2);
    assert_eq!(clos.jt_targets, 7);
    for &rva in f2a_cases.iter().chain(f2b_cases.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing ptr64 switch case RVA {rva:#x}");
    }
    assert!(!ips.contains(&(base64 + 0x12B0)), "scan must stop at zero terminator");
}

/// Pattern 3 (compact 8-bit and 16-bit index indirection tables -> 32-bit RVA jump table).
#[test]
fn aot_jump_table_discovery_compact_indirection_u8_u16() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3200_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 24)));

    // --- 3A @ 0x1000: 8-bit indirection table (8 cases -> 3 unique handlers) ---
    let h3a: [u32; 3] = [0x1080, 0x1090, 0x10A0];
    let def3a: u32 = 0x10B0;
    let idx8_rva: u32 = 0x4100;
    let jmp3a_rva: u32 = 0x4120;
    let map8: [u8; 8] = [0, 1, 2, 1, 0, 2, 2, 1];
    buf[idx8_rva as usize..idx8_rva as usize + 8].copy_from_slice(&map8);
    for (i, &rva) in h3a.iter().enumerate() {
        buf[jmp3a_rva as usize + i * 4..jmp3a_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c3a = Vec::new();
    c3a.extend_from_slice(&[0x83, 0xF9, 0x07]); // cmp ecx, 7
    let ja_next = 0x1000 + c3a.len() as u32 + 6;
    c3a.extend_from_slice(&[0x0F, 0x87]);
    c3a.extend_from_slice(&((def3a as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3a, base64 + 0x1000, 2 /* rdx */, base64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, byte_ptr(rdx + rcx + idx8_rva as i32)).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + jmp3a_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c3a.extend_from_slice(&a.assemble(base64 + 0x1000 + c3a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c3a.len()].copy_from_slice(&c3a);
    for (i, &rva) in h3a.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[def3a as usize..def3a as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 3B @ 0x1200: 16-bit indirection table (6 cases -> 4 unique handlers) ---
    let h3b: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let def3b: u32 = 0x12C0;
    let idx16_rva: u32 = 0x4160;
    let jmp3b_rva: u32 = 0x4180;
    let map16: [u16; 6] = [3, 0, 2, 1, 3, 2];
    for (i, &v) in map16.iter().enumerate() {
        buf[idx16_rva as usize + i * 2..idx16_rva as usize + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    for (i, &rva) in h3b.iter().enumerate() {
        buf[jmp3b_rva as usize + i * 4..jmp3b_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c3b = Vec::new();
    c3b.extend_from_slice(&[0x83, 0xF9, 0x05]); // cmp ecx, 5
    let ja_next = 0x1200 + c3b.len() as u32 + 6;
    c3b.extend_from_slice(&[0x0F, 0x87]);
    c3b.extend_from_slice(&((def3b as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3b, base64 + 0x1200, 2 /* rdx */, base64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, word_ptr(rdx + rcx * 2 + idx16_rva as i32)).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + jmp3b_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c3b.extend_from_slice(&a.assemble(base64 + 0x1200 + c3b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c3b.len()].copy_from_slice(&c3b);
    for (i, &rva) in h3b.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[def3b as usize..def3b as usize + 6].copy_from_slice(&[0xB8, 98, 0, 0, 0, 0xC3]);

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 2);
    assert_eq!(clos.jt_targets, 7, "expected 3 + 4 unique deduplicated handlers");
    for &rva in h3a.iter().chain(h3b.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing indirection handler RVA {rva:#x}");
    }
}

/// Pattern 4 (Clang/GCC PIC signed 32-bit table-base-relative offsets):
/// - 4A: `lea rcx, [rip + tbl]; movsxd rax, [rcx + rdx*4]; add rax, rcx; jmp rax` with NEGATIVE offsets (handlers before `.rdata` table)
/// - 4B: symmetric `add rcx, rax; jmp rcx` with `and edx, 3` mask bound
/// - 4C: `lea rax, [rcx + rax]; jmp rax`
#[test]
fn aot_jump_table_discovery_clang_pic_signed32() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3300_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 36)));

    // --- 4A @ 0x1000: negative signed-32 offsets from `.rdata` table at 0x4100 back into `.text` ---
    let h4a: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let def4a: u32 = 0x10C0;
    let tbl_4a_rva: u32 = 0x4100;
    for (i, &rva) in h4a.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4a_rva as i32); // negative!
        assert!(rel < 0);
        buf[tbl_4a_rva as usize + i * 4..tbl_4a_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4a = Vec::new();
    c4a.extend_from_slice(&[0x83, 0xFA, 0x03]); // cmp edx, 3
    let ja_next = 0x1000 + c4a.len() as u32 + 6;
    c4a.extend_from_slice(&[0x0F, 0x87]);
    c4a.extend_from_slice(&((def4a as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c4a, base64 + 0x1000, 1 /* rcx */, base64 + tbl_4a_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.add(rax, rcx).unwrap();
    a.jmp(rax).unwrap();
    c4a.extend_from_slice(&a.assemble(base64 + 0x1000 + c4a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c4a.len()].copy_from_slice(&c4a);
    for (i, &rva) in h4a.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[def4a as usize..def4a as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 4B @ 0x1200: `and edx, 3` mask + symmetric `add rcx, rax; jmp rcx` ---
    let h4b: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let tbl_4b_rva: u32 = 0x4140;
    for (i, &rva) in h4b.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4b_rva as i32);
        buf[tbl_4b_rva as usize + i * 4..tbl_4b_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4b = Vec::new();
    c4b.extend_from_slice(&[0x83, 0xE2, 0x03]); // and edx, 3
    emit_lea_rip(&mut c4b, base64 + 0x1200, 1 /* rcx */, base64 + tbl_4b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.add(rcx, rax).unwrap();
    a.jmp(rcx).unwrap();
    c4b.extend_from_slice(&a.assemble(base64 + 0x1200 + c4b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c4b.len()].copy_from_slice(&c4b);
    for (i, &rva) in h4b.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (10 + i as u8), 0, 0, 0, 0xC3]);
    }

    // --- 4C @ 0x1400: `lea rax, [rcx + rax]; jmp rax` with `jbe` taken-edge bound ---
    let h4c: [u32; 3] = [0x1480, 0x1490, 0x14A0];
    let tbl_4c_rva: u32 = 0x4180;
    let disp_4c: u32 = 0x1430;
    for (i, &rva) in h4c.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4c_rva as i32);
        buf[tbl_4c_rva as usize + i * 4..tbl_4c_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4c = Vec::new();
    c4c.extend_from_slice(&[0x83, 0xFA, 0x02]); // cmp edx, 2
    let jbe_next = 0x1400 + c4c.len() as u32 + 6;
    c4c.extend_from_slice(&[0x0F, 0x86]); // jbe disp_4c
    c4c.extend_from_slice(&((disp_4c as i32) - (jbe_next as i32)).to_le_bytes());
    c4c.extend_from_slice(&[0xB8, 97, 0, 0, 0, 0xC3]); // default fallthrough
    buf[0x1400..0x1400 + c4c.len()].copy_from_slice(&c4c);

    let mut d4c = Vec::new();
    emit_lea_rip(&mut d4c, base64 + disp_4c as u64, 1 /* rcx */, base64 + tbl_4c_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.lea(rax, ptr(rcx + rax)).unwrap();
    a.jmp(rax).unwrap();
    d4c.extend_from_slice(&a.assemble(base64 + disp_4c as u64 + d4c.len() as u64).unwrap());
    buf[disp_4c as usize..disp_4c as usize + d4c.len()].copy_from_slice(&d4c);
    for (i, &rva) in h4c.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300), (0x1400, 0x1500)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200, 0x1400], 2) };
    assert_eq!(clos.jt_tables, 3);
    assert_eq!(clos.jt_targets, 11);
    for &rva in h4a.iter().chain(h4b.iter()).chain(h4c.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing Clang PIC case RVA {rva:#x}");
    }
}

/// Soundness / negative tests for jump-table discovery:
/// - IAT jump thunk (`jmp qword ptr [rip + slot]`) must NOT be mistaken for a jump table;
/// - Virtual-call dispatch (`mov rax, [rcx]; jmp qword ptr [rax + 0x10]`) must NOT trigger;
/// - A jump-table entry pointing into another `.pdata` function stops the scan cleanly.
#[test]
fn aot_jump_table_discovery_ignores_non_switch_indirect_branches() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3400_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 48)));

    // 1. IAT jump thunk @ 0x1000: `jmp qword ptr [rip + 0x4100]`
    let iat_slot = base64 + 0x4100;
    let target_in_text = base64 + 0x1080;
    buf[0x4100..0x4108].copy_from_slice(&target_in_text.to_le_bytes());
    let disp = (iat_slot as i64 - (base64 + 0x1006) as i64) as i32;
    buf[0x1000..0x1002].copy_from_slice(&[0xFF, 0x25]);
    buf[0x1002..0x1006].copy_from_slice(&disp.to_le_bytes());
    buf[0x1080..0x1086].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);

    // 2. Vtable jump @ 0x1100: `mov rax, [rcx]; jmp qword ptr [rax + 0x10]`
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.jmp(qword_ptr(rax + 0x10)).unwrap();
    let vtbl = a.assemble(base64 + 0x1100).unwrap();
    buf[0x1100..0x1100 + vtbl.len()].copy_from_slice(&vtbl);

    // 3. Switch @ 0x1200 whose 3rd entry crosses into another `.pdata` function @ 0x1300.
    let tbl_rva: u32 = 0x4200;
    for (i, &rva) in [0x1280u32, 0x1290, 0x1300, 0x12A0].iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_rva as usize + i * 8..tbl_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    let mut c3 = Vec::new();
    emit_lea_rip(&mut c3, base64 + 0x1200, 8 /* r8 */, base64 + tbl_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.jmp(qword_ptr(r8 + rcx * 8)).unwrap();
    c3.extend_from_slice(&a.assemble(base64 + 0x1200 + c3.len() as u64).unwrap());
    buf[0x1200..0x1200 + c3.len()].copy_from_slice(&c3);
    for &rva in &[0x1280u32, 0x1290, 0x12A0, 0x1300] {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, 5, 0, 0, 0, 0xC3]);
    }

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x10C0u32), (0x1100, 0x1140), (0x1200, 0x12E0), (0x1300, 0x1340)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1100, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 1, "only the real switch table should be discovered");
    assert_eq!(clos.jt_targets, 2, "scan must stop at the cross-function entry 0x1300");
    assert!(!ips.contains(&target_in_text), "IAT thunk must not trigger jump-table discovery");
    assert!(ips.contains(&(base64 + 0x1280)) && ips.contains(&(base64 + 0x1290)));
    assert!(!ips.contains(&(base64 + 0x1300)), "cross-function entry must be rejected");
    assert!(!ips.contains(&(base64 + 0x12A0)), "scan must stop at first invalid/cross-function entry");
}

/// Regression test: `cmp reg, -1` (`imm == u64::MAX`) followed by `ja`/`jbe` must never overflow
/// when computing jump-table bounds during offline CFG discovery.
#[test]
fn aot_jump_table_discovery_handles_cmp_u64_max_without_overflow() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x4000, 0x2_3450_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x1000, 0x2000, 0x1000, None, None);

    let mut a = CodeAssembler::new(64).unwrap();
    let mut lbl_out = a.create_label();
    a.cmp(rax, -1i32).unwrap();
    a.ja(lbl_out).unwrap();
    a.mov(eax, 1).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_out).unwrap();
    a.mov(eax, 2).unwrap();
    a.ret().unwrap();
    let code = a.assemble(base64 + 0x1000).unwrap();
    buf[0x1000..0x1000 + code.len()].copy_from_slice(&code);

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], 2) };
    assert!(ips.contains(&(base64 + 0x1000)));
    assert_eq!(clos.jt_tables, 0);
}

/// Parallel `.aot` prewarming produces bit-for-bit identical `.aot` images across 1, 2, 4,
/// and 8 worker threads, passes `Verify::All` with 0 mismatches, and benchmarks wall-clock
/// speedup on a multi-thousand-block synthetic PE image.
#[test]
fn aot_parallel_prewarm_deterministic_and_verified() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            // Synthesize a realistic 640-function PE (~1,920 basic blocks) with branches,
            // calls, literal loads, and a host-builtin IAT call so every stub branch kind,
            // link site, and literal pool relocation is exercised under parallel translation.
            const NFUNCS: usize = 2048;
            const FUNC_STRIDE: usize = 0x40;
            const TEXT_RVA: u32 = 0x1000;
            const TEXT_SIZE: u32 = 0x2_1000; // > 2048 * 0x40 = 0x2_0000 + padding
            const RDATA_RVA: u32 = 0x2_2000;
            const RDATA_SIZE: u32 = 0xA000;
            const IMG_SIZE: usize = 0x2_C000;

            let mut pe = TestMappedBuf::alloc(IMG_SIZE, 0x2_3500_0000);
            let base = pe.base();
            let base64 = base as u64;
            let buf = pe.as_slice_mut();
            let pdata_rva: u32 = RDATA_RVA + 0x100;
            init_synth_pe(
                buf,
                base64,
                TEXT_RVA,
                TEXT_RVA,
                TEXT_SIZE,
                RDATA_RVA,
                RDATA_SIZE,
                None,
                Some((pdata_rva, (NFUNCS * 12) as u32)),
            );

            // Register a plain builtin VA for IAT calls inside the synthetic functions.
            let h_va = base64 + RDATA_RVA as u64 + 0x80;
            crate::exe::register_native_va(h_va, win32_rt::msvcrt::memset as *const () as usize);
            let iat_slot = base64 + RDATA_RVA as u64;
            buf[RDATA_RVA as usize..RDATA_RVA as usize + 8].copy_from_slice(&h_va.to_le_bytes());

            let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
            let unwind_rva: u32 = RDATA_RVA + 0x7000;
            buf[unwind_rva as usize] = 0x01;

            for i in 0..NFUNCS {
                let rva = TEXT_RVA + (i * FUNC_STRIDE) as u32;
                let va = base64 + rva as u64;
                let mut code = Vec::new();
                if i % 8 == 0 {
                    // Includes an indirect IAT call to the host builtin + conditional branch.
                    code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]); // sub rsp, 0x28
                    code.extend_from_slice(&[0x85, 0xC9]); // test ecx, ecx
                    code.extend_from_slice(&[0x74, 0x0B]); // jz +11
                    emit_call_rip_slot(&mut code, va, iat_slot); // 6 bytes
                    code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]); // add rsp, 0x28; ret
                    // else branch:
                    code.extend_from_slice(&[0xB8]);
                    code.extend_from_slice(&(i as u32).to_le_bytes());
                    code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                } else {
                    // Conditional diamond + local arithmetic (3 basic blocks per function).
                    let mut a = CodeAssembler::new(64).unwrap();
                    let mut l_alt = a.create_label();
                    a.cmp(ecx, (i & 15) as i32).unwrap();
                    a.ja(l_alt).unwrap();
                    a.lea(eax, ptr(rcx + rdx + (i as i32))).unwrap();
                    a.imul_3(eax, eax, (i as i32) | 1).unwrap();
                    a.ret().unwrap();
                    a.set_label(&mut l_alt).unwrap();
                    a.mov(eax, edx).unwrap();
                    a.xor(eax, (i as i32) ^ 0x5A).unwrap();
                    a.ret().unwrap();
                    code = a.assemble(va).unwrap();
                }
                assert!(code.len() <= FUNC_STRIDE, "fn {i} exceeded stride");
                buf[rva as usize..rva as usize + code.len()].copy_from_slice(&code);
                let pe_off = pdata_rva as usize + i * 12;
                w32(buf, pe_off, rva);
                w32(buf, pe_off + 4, rva + FUNC_STRIDE as u32);
                w32(buf, pe_off + 8, unwind_rva);
            }

            let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
            assert_eq!(roots.len(), NFUNCS);

            let key = unsafe { module_key(base, pe.len) };
            let mut pool_1 = WorkerCaches::new();

            let t_seq = std::time::Instant::now();
            let (ips_1, _) = unsafe { discover_with_threads(&mut pool_1, base, pe.len, &roots, 2, 1) };
            let img_1 = unsafe { build_image_with_threads(&mut pool_1, &ips_1, BUILD_VBASE, 1) }
                .expect("1-thread build");
            let dt_seq = t_seq.elapsed();
            let bytes_1 = encode("synth_par.dll", &key, &img_1, 0);
            assert!(ips_1.len() >= NFUNCS * 2, "expected multi-block closure, got {}", ips_1.len());
            assert!(img_1.blocks.len() >= NFUNCS * 2);
            assert!(!img_1.links.is_empty());

            let mut timings = Vec::new();
            for threads in [2usize, 4, 8] {
                let mut pool_p = WorkerCaches::new();
                let t_par = std::time::Instant::now();
                let (ips_p, _) = unsafe { discover_with_threads(&mut pool_p, base, pe.len, &roots, 2, threads) };
                let img_p = unsafe { build_image_with_threads(&mut pool_p, &ips_p, BUILD_VBASE, threads) }
                    .expect("parallel build");
                let dt_par = t_par.elapsed();
                timings.push((threads, dt_par));
                assert_eq!(ips_p, ips_1, "discovered IPs diverged at {threads} threads");
                let bytes_p = encode("synth_par.dll", &key, &img_p, 0);
                assert_eq!(
                    bytes_p, bytes_1,
                    "encoded .aot output differed between 1 thread and {threads} threads"
                );
            }

            // Verify every single block of the parallel-built image via `install_image(..., Verify::All)`.
            let (_, _, img_dec, _) = decode(&bytes_1).expect("decode parallel .aot");
            let total_blocks = img_dec.blocks.len();
            let inst = unsafe { install_image("synth_par.dll", base, pe.len, img_dec, Verify::All, true) }
                .expect("install parallel .aot");
            assert_eq!(inst.mismatches, 0, "parallel .aot had verify mismatches");
            assert_eq!(inst.verified, total_blocks, "expected all blocks verified");
            assert_eq!(inst.live, total_blocks, "expected all blocks live");

            let summary: Vec<String> = timings
                .iter()
                .map(|&(t, dt)| {
                    format!(
                        "{t}t={:.2} ms ({:.2}x)",
                        dt.as_secs_f64() * 1e3,
                        dt_seq.as_secs_f64() / dt.as_secs_f64().max(1e-9)
                    )
                })
                .collect();
            eprintln!(
                "aot parallel benchmark ({total_blocks} blocks): 1t={:.2} ms, {}",
                dt_seq.as_secs_f64() * 1e3,
                summary.join(", "),
            );

            test_uninstall();
            crate::exe::unregister_native_va(h_va);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Multi-module `.aot` prewarm + reload verification test under `PORT_AOT_VERIFY=1` semantics:
/// - Maps TWO synthetic PE32+ modules (`synth_app_v.exe` and `synth_helper_v.dll`) at
///   relocated/shifted bases (`!= ImageBase`) with PE DIR64 base relocations applied;
/// - Exercises cross-module indirect IAT calls, host-builtin IAT relocations, MSVC rva32 &
///   Clang PIC signed-32 switch jump tables, and intra-module AOT direct links;
/// - Prewarms both modules with `aot_prewarm_all_mapped`, moves host-builtin IAT slot VAs,
///   reloads both `.aot` caches with full block verification (`verify_mode() == 1`), verifies
///   0 verify mismatches and non-zero rebased relocations, and executes every switch case
///   comparing AOT output against the pure interpreter oracle;
/// - Then shifts `synth_helper_v.dll` to a SECOND relocated base address (`dll_base_2 != dll_base_1`),
///   confirms cache-key isolation on the un-prewarmed shifted base, updates the `.exe`'s
///   cross-module IAT slots, re-prewarms and reloads under full verification (0 mismatches),
///   and re-executes every switch case to confirm exact return values.
#[test]
fn aot_multi_module_prewarm_reload_verify_and_execute() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            let _env = TestEnvGuard::new(&[
                "PORT_AOT_DIR",
                "AW2_DLL_AOT_DIR",
                "PORT_AOT_THREADS",
                "PORT_AOT_CLOSURE",
            ]);
            let d = test_dir("multi-verify");
            std::env::set_var("PORT_AOT_DIR", &d);
            std::env::remove_var("AW2_DLL_AOT_DIR");
            std::env::set_var("PORT_AOT_THREADS", "4");
            std::env::set_var("PORT_AOT_CLOSURE", "3");

            let exe_name = "synth_app_v.exe";
            let dll_name = "synth_helper_v.dll";

            // Scratch guest buffer for host `memset` calls + 3 distinct host-builtin VAs so
            // we can move the IAT slots across reloads and exercise `rebase_block`.
            let scratch_mem = TestMappedBuf::alloc(0x4000, 0x2_3800_0000);
            let scratch_va = scratch_mem.base() as u64;
            let h_memset_1 = scratch_va + 0x1000;
            let h_memset_2 = scratch_va + 0x1020;
            let h_memset_3 = scratch_va + 0x1040;
            let memset_ptr = win32_rt::msvcrt::memset as *const () as usize;
            crate::exe::register_native_va(h_memset_1, memset_ptr);
            crate::exe::register_native_va(h_memset_2, memset_ptr);
            crate::exe::register_native_va(h_memset_3, memset_ptr);

            // Helper: build `synth_helper_v.dll` at a given shifted buffer with PE DIR64
            // relocations applied from preferred `ImageBase = 0x1_8000_0000`.
            // Exports:
            //   - 0x1000 (`compute_a(rcx)`): returns `rcx * 3 + 7`
            //   - 0x1040 (`switch_b(rcx = case, rdx = val)`): Clang PIC signed-32 switch table:
            //       0 -> val + 100
            //       1 -> val * 2 + 11
            //       2 -> reads PE-relocated 64-bit constant pointer -> returns `*ptr + val`
            //       3 -> calls host `memset(scratch, 0x2A, 16)` via IAT, returns `*(u32*)scratch + val`
            //       default -> 0xBEEF
            let build_helper_dll = |pe: &mut TestMappedBuf, host_va: u64, work_buf: u64| {
                const PREF: u64 = 0x1_8000_0000;
                let base = pe.base() as u64;
                let delta = base.wrapping_sub(PREF);
                let buf = pe.as_slice_mut();
                buf.fill(0);
                // Export dir @ 0x4000, .pdata @ 0x4200, IAT slot @ 0x4080,
                // Clang signed32 switch table @ 0x4100, PE-relocated 64-bit ptr @ 0x4180 -> 0x4190 (u64=500).
                init_synth_pe(
                    buf,
                    PREF,
                    0x1000,
                    0x1000,
                    0x3000,
                    0x4000,
                    0x3000,
                    Some((0x4000, 0x60)),
                    Some((0x4200, 24)),
                );
                let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
                let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

                // Export directory: 2 exports (0x1000, 0x1040).
                w32(buf, 0x4000 + 20, 2);
                w32(buf, 0x4000 + 28, 0x4030);
                w32(buf, 0x4030, 0x1000);
                w32(buf, 0x4034, 0x1040);

                // IAT slot @ 0x4080 -> host memset VA.
                w64(buf, 0x4080, host_va);
                // PE-relocated 64-bit pointer @ 0x4180 initially holding `PREF + 0x4190`, relocated by `delta`.
                w64(buf, 0x4180, PREF + 0x4190);
                w64(buf, 0x4190, 500);
                // Apply DIR64 base relocation to 0x4180 and update OptionalHeader.ImageBase.
                let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
                w64(buf, 0x4180, orig.wrapping_add(delta));
                w64(buf, 0x58 + 24, base);

                // Export 0 @ 0x1000 (`compute_a`): lea rax, [rcx + rcx*2 + 7]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rcx + rcx * 2 + 7)).unwrap();
                a.ret().unwrap();
                let fn0 = a.assemble(base + 0x1000).unwrap();
                buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

                // Export 1 @ 0x1040 (`switch_b`): Clang PIC signed-32 switch table at 0x4100.
                let cases: [u32; 4] = [0x10C0, 0x10E0, 0x1100, 0x1140];
                let def_rva: u32 = 0x11A0;
                for (i, &rva) in cases.iter().enumerate() {
                    let rel = (rva as i32) - 0x4100i32;
                    w32(buf, 0x4100 + i * 4, rel as u32);
                }
                let mut sw = Vec::new();
                sw.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
                let ja_next = 0x1040 + sw.len() as u32 + 6;
                sw.extend_from_slice(&[0x0F, 0x87]);
                sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
                emit_lea_rip(&mut sw, base + 0x1040, 8 /* r8 */, base + 0x4100);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
                a.add(rax, r8).unwrap();
                a.jmp(rax).unwrap();
                sw.extend_from_slice(&a.assemble(base + 0x1040 + sw.len() as u64).unwrap());
                buf[0x1040..0x1040 + sw.len()].copy_from_slice(&sw);

                // Case 0 @ 0x10C0: lea rax, [rdx + 100]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rdx + 100)).unwrap();
                a.ret().unwrap();
                let b0 = a.assemble(base + 0x10C0).unwrap();
                buf[0x10C0..0x10C0 + b0.len()].copy_from_slice(&b0);

                // Case 1 @ 0x10E0: lea rax, [rdx*2 + 11]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rdx * 2 + 11)).unwrap();
                a.ret().unwrap();
                let b1 = a.assemble(base + 0x10E0).unwrap();
                buf[0x10E0..0x10E0 + b1.len()].copy_from_slice(&b1);

                // Case 2 @ 0x1100: lea r8, [rip + 0x4180]; mov r8, [r8]; mov rax, [r8]; add rax, rdx; ret
                let mut b2 = Vec::new();
                emit_lea_rip(&mut b2, base + 0x1100, 8 /* r8 */, base + 0x4180);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(r8, qword_ptr(r8)).unwrap();
                a.mov(rax, qword_ptr(r8)).unwrap();
                a.add(rax, rdx).unwrap();
                a.ret().unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
                buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

                // Case 3 @ 0x1140: call host memset(work_buf, 0x2A, 16) via IAT @ 0x4080; return *(u8*)work_buf + rdx
                let mut b3 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbx).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(rbx, rdx).unwrap();
                a.mov(rcx, work_buf).unwrap();
                a.mov(edx, 0x2A).unwrap();
                a.mov(r8d, 16).unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1140).unwrap());
                emit_call_rip_slot(&mut b3, base + 0x1140, base + 0x4080);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movzx(eax, byte_ptr(rax)).unwrap();
                a.add(rax, rbx).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(rbx).unwrap();
                a.ret().unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1140 + b3.len() as u64).unwrap());
                buf[0x1140..0x1140 + b3.len()].copy_from_slice(&b3);

                // Default @ 0x11A0: mov eax, 0xBEEF; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, 0xBEEF).unwrap();
                a.ret().unwrap();
                let bd = a.assemble(base + 0x11A0).unwrap();
                buf[0x11A0..0x11A0 + bd.len()].copy_from_slice(&bd);

                // .pdata
                w32(buf, 0x4200, 0x1000);
                w32(buf, 0x4204, 0x1030);
                w32(buf, 0x4208, 0x4300);
                w32(buf, 0x420C, 0x1040);
                w32(buf, 0x4210, 0x11E0);
                w32(buf, 0x4214, 0x4300);
                buf[0x4300] = 0x01;
            };

            // Helper: build `synth_app_v.exe` at a shifted base (from preferred `0x1_4000_0000`)
            // with PE DIR64 base relocations, MSVC rva32 switch table, cross-module IAT calls
            // to `synth_helper_v.dll`, host-builtin IAT call, and intra-module local helper call.
            // Entry @ 0x1000(rcx = outer_case, rdx = arg1, r8 = arg2):
            //   0 -> calls dll!compute_a(arg1) via IAT @ 0x4080
            //   1 -> calls dll!switch_b(arg1, arg2) via IAT @ 0x4088
            //   2 -> calls host memset(work_buf + 0x100, 0x33, 8) via IAT @ 0x4090, returns *(u8*)ptr + arg1
            //   3 -> direct call to local helper @ 0x1300(arg1) + *(u64*)*(u64*)(reloc_ptr @ 0x4180)
            //   default -> 0xDEAD_0000
            let build_app_exe = |pe: &mut TestMappedBuf,
                                 dll_base: u64,
                                 host_va: u64,
                                 work_buf: u64| {
                const PREF: u64 = 0x1_4000_0000;
                let base = pe.base() as u64;
                let delta = base.wrapping_sub(PREF);
                let buf = pe.as_slice_mut();
                buf.fill(0);
                init_synth_pe(
                    buf,
                    PREF,
                    0x1000,
                    0x1000,
                    0x3000,
                    0x4000,
                    0x3000,
                    None,
                    Some((0x4200, 24)),
                );
                let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
                let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

                // IAT slots @ 0x4080..0x4098:
                w64(buf, 0x4080, dll_base + 0x1000); // dll!compute_a
                w64(buf, 0x4088, dll_base + 0x1040); // dll!switch_b
                w64(buf, 0x4090, host_va); // host memset

                // PE-relocated 64-bit constant pointer @ 0x4180 -> 0x4190 (u64=1000).
                w64(buf, 0x4180, PREF + 0x4190);
                w64(buf, 0x4190, 1000);
                let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
                w64(buf, 0x4180, orig.wrapping_add(delta));
                w64(buf, 0x58 + 24, base);

                // MSVC rva32 switch table @ 0x4100:
                let cases: [u32; 4] = [0x1080, 0x10C0, 0x1100, 0x1180];
                let def_rva: u32 = 0x1200;
                for (i, &rva) in cases.iter().enumerate() {
                    w32(buf, 0x4100 + i * 4, rva);
                }

                // Entry @ 0x1000: cmp ecx, 3; ja def; lea r10, [rip + __ImageBase]; mov eax, [r10 + rcx*4 + 0x4100]; add rax, r10; jmp rax
                let mut sw = Vec::new();
                sw.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
                let ja_next = 0x1000 + sw.len() as u32 + 6;
                sw.extend_from_slice(&[0x0F, 0x87]);
                sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
                emit_lea_rip(&mut sw, base + 0x1000, 10 /* r10 */, base);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, dword_ptr(r10 + rcx * 4 + 0x4100)).unwrap();
                a.add(rax, r10).unwrap();
                a.jmp(rax).unwrap();
                sw.extend_from_slice(&a.assemble(base + 0x1000 + sw.len() as u64).unwrap());
                buf[0x1000..0x1000 + sw.len()].copy_from_slice(&sw);

                // Case 0 @ 0x1080: sub rsp, 0x28; mov rcx, rdx; call [iat_compute_a]; add rsp, 0x28; ret
                let mut b0 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                b0.extend_from_slice(&a.assemble(base + 0x1080).unwrap());
                emit_call_rip_slot(&mut b0, base + 0x1080, base + 0x4080);
                b0.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                buf[0x1080..0x1080 + b0.len()].copy_from_slice(&b0);

                // Case 1 @ 0x10C0: sub rsp, 0x28; mov rcx, rdx; mov rdx, r8; call [iat_switch_b]; add rsp, 0x28; ret
                let mut b1 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                a.mov(rdx, r8).unwrap();
                b1.extend_from_slice(&a.assemble(base + 0x10C0).unwrap());
                emit_call_rip_slot(&mut b1, base + 0x10C0, base + 0x4088);
                b1.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                buf[0x10C0..0x10C0 + b1.len()].copy_from_slice(&b1);

                // Case 2 @ 0x1100: call host memset(work_buf + 0x100, 0x33, 8) via IAT @ 0x4090
                let mut b2 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbx).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(rbx, rdx).unwrap();
                a.mov(rcx, work_buf + 0x100).unwrap();
                a.mov(edx, 0x33).unwrap();
                a.mov(r8d, 8).unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100).unwrap());
                emit_call_rip_slot(&mut b2, base + 0x1100, base + 0x4090);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movzx(eax, byte_ptr(rax)).unwrap();
                a.add(rax, rbx).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(rbx).unwrap();
                a.ret().unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
                buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

                // Case 3 @ 0x1180: direct call local helper @ 0x1300 + read PE-relocated ptr @ 0x4180
                let mut b3 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                a.call(base + 0x1300).unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1180).unwrap());
                emit_lea_rip(&mut b3, base + 0x1180, 9 /* r9 */, base + 0x4180);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(r9, qword_ptr(r9)).unwrap();
                a.add(rax, qword_ptr(r9)).unwrap();
                a.add(rsp, 0x28).unwrap();
                a.ret().unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1180 + b3.len() as u64).unwrap());
                buf[0x1180..0x1180 + b3.len()].copy_from_slice(&b3);

                // Default @ 0x1200: mov eax, 0xDEAD0000; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, 0xDEAD_0000u32).unwrap();
                a.ret().unwrap();
                let bd = a.assemble(base + 0x1200).unwrap();
                buf[0x1200..0x1200 + bd.len()].copy_from_slice(&bd);

                // Local helper @ 0x1300: lea rax, [rcx * 4 + 5]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rcx * 4 + 5)).unwrap();
                a.ret().unwrap();
                let hl = a.assemble(base + 0x1300).unwrap();
                buf[0x1300..0x1300 + hl.len()].copy_from_slice(&hl);

                // .pdata
                w32(buf, 0x4200, 0x1000);
                w32(buf, 0x4204, 0x1240);
                w32(buf, 0x4208, 0x4300);
                w32(buf, 0x420C, 0x1300);
                w32(buf, 0x4210, 0x1320);
                w32(buf, 0x4214, 0x4300);
                buf[0x4300] = 0x01;
            };

            let run_all_cases = |exe_entry: u64, mode: u8| -> Vec<u64> {
                let inputs: [(u64, u64, u64); 8] = [
                    (0, 10, 0),    // compute_a(10) = 37
                    (1, 0, 25),    // switch_b(0, 25) = 125
                    (1, 1, 25),    // switch_b(1, 25) = 61
                    (1, 2, 25),    // switch_b(2, 25) = 525
                    (1, 3, 25),    // switch_b(3, 25) = 0x2A + 25 = 67
                    (1, 9, 25),    // switch_b(9, 25) = 0xBEEF
                    (2, 7, 0),     // exe host memset + 7 = 0x33 + 7 = 58
                    (3, 12, 0),    // local helper(12)=53 + 1000 = 1053
                ];
                jit::force(mode);
                let mut out = Vec::with_capacity(inputs.len() + 1);
                for (a0, a1, a2) in inputs {
                    let (r, _) = unsafe { crate::emu::emu_run_full(exe_entry, [a0, a1, a2, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                    out.push(r);
                }
                let (def_r, _) = unsafe { crate::emu::emu_run_full(exe_entry, [42, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                out.push(def_r);
                jit::force(0);
                out
            };

            let expected = vec![37, 125, 61, 525, 67, 0xBEEF, 58, 1053, 0xDEAD_0000];

            // --- Phase 1: Map both modules at shifted/relocated bases & prewarm ---
            let mut dll1 = TestMappedBuf::alloc(0x8000, 0x2_3900_0000);
            let mut exe1 = TestMappedBuf::alloc(0x8000, 0x2_3A00_0000);
            build_helper_dll(&mut dll1, h_memset_1, scratch_va);
            build_app_exe(&mut exe1, dll1.base() as u64, h_memset_1, scratch_va);

            // Confirm interpreter oracle matches expected values.
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 2), expected);

            module_mapped(exe_name, exe1.base(), exe1.len);
            module_mapped(dll_name, dll1.base(), dll1.len);
            let reps = aot_prewarm_all_mapped(0);
            let exe_rep = reps.iter().find(|r| r.module == exe_name).expect("exe report");
            let dll_rep = reps.iter().find(|r| r.module == dll_name).expect("dll report");
            assert!(exe_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(dll_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(exe_rep.blocks >= 8, "expected exe roots + all 4 switch cases + default, got {}", exe_rep.blocks);
            assert!(dll_rep.blocks >= 8, "expected dll exports + all 4 switch cases + default, got {}", dll_rep.blocks);

            // Move host-builtin IAT slots in BOTH modules to `h_memset_2` before reload to exercise
            // AOT relocation rebasing (`RELOC_HOST_FN` + `RELOC_SLOT_VAL`).
            exe1.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_2.to_le_bytes());
            dll1.as_slice_mut()[0x4080..0x4088].copy_from_slice(&h_memset_2.to_le_bytes());

            // Reload both modules with full `PORT_AOT_VERIFY=1` verification.
            let v0 = TEST_VERIFIED.with(|c| c.get());
            let m0 = TEST_VERIFY_MISMATCHES.with(|c| c.get());
            let r0 = TEST_REBASED.with(|c| c.get());
            TEST_AOT.with(|c| c.set(true));
            TEST_VERIFY.with(|c| c.set(Some(1)));
            activate();
            let verified1 = TEST_VERIFIED.with(|c| c.get()) - v0;
            let mismatches1 = TEST_VERIFY_MISMATCHES.with(|c| c.get()) - m0;
            let rebased1 = TEST_REBASED.with(|c| c.get()) - r0;
            assert_eq!(mismatches1, 0, "expected 0 verify mismatches on phase-1 reload");
            assert_eq!(
                verified1,
                exe_rep.blocks + dll_rep.blocks,
                "expected every block of both modules verified"
            );
            assert!(
                !jit::plaincall_enabled() || rebased1 >= 2,
                "expected host-builtin IAT relocations rebased in both modules, got {rebased1}"
            );

            // Confirm switch-case blocks in BOTH modules are live in the AOT directory.
            for rva in [0x1000u64, 0x1080, 0x10C0, 0x1100, 0x1180, 0x1200, 0x1300] {
                assert!(find(exe1.base() as u64 + rva).is_some(), "exe block {rva:#x} missing");
            }
            for rva in [0x1000u64, 0x1040, 0x10C0, 0x10E0, 0x1100, 0x1140, 0x11A0] {
                assert!(find(dll1.base() as u64 + rva).is_some(), "dll block {rva:#x} missing");
            }

            // Execute all switch cases via loaded AOT blocks and verify exact results & AOT hits.
            let installs0 = jit::test_aot_installs();
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 1), expected);
            assert!(
                jit::test_aot_installs() - installs0 > 0,
                "expected loaded AOT blocks to execute"
            );

            // --- Phase 2: Shift BOTH modules to new relocated base addresses ---
            teardown_test_modules(&[exe_name, dll_name]);
            let mut dll2 = TestMappedBuf::alloc(0x8000, 0x2_3C00_0000);
            let mut exe2 = TestMappedBuf::alloc(0x8000, 0x2_3D00_0000);
            assert_ne!(dll2.base(), dll1.base());
            assert_ne!(exe2.base(), exe1.base());
            build_helper_dll(&mut dll2, h_memset_2, scratch_va);
            build_app_exe(&mut exe2, dll2.base() as u64, h_memset_2, scratch_va);

            // Cache-key safety: loading shifted bases before re-prewarming must cleanly miss
            // the base-1 `.aot` files (no stale wrong-base blocks loaded).
            let key_exe2 = unsafe { module_key(exe2.base(), exe2.len) };
            let key_dll2 = unsafe { module_key(dll2.base(), dll2.len) };
            assert!(
                unsafe { load_module(exe_name, exe2.base(), exe2.len, &key_exe2) }.is_none(),
                "shifted exe base must not reuse old-base cache"
            );
            assert!(
                unsafe { load_module(dll_name, dll2.base(), dll2.len, &key_dll2) }.is_none(),
                "shifted dll base must not reuse old-base cache"
            );

            // Cross-module base shift: rebind `exe1`'s IAT slots to `dll2` (shifted DLL base!)
            // while keeping `exe1` on its existing base-1 `.aot` cache and moving host IAT to `h_memset_3`.
            exe1.as_slice_mut()[0x4080..0x4088].copy_from_slice(&(dll2.base() as u64 + 0x1000).to_le_bytes());
            exe1.as_slice_mut()[0x4088..0x4090].copy_from_slice(&(dll2.base() as u64 + 0x1040).to_le_bytes());
            exe1.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_3.to_le_bytes());

            // Prewarm shifted `dll2` (and shifted `exe2`), move host IAT slots to `h_memset_3`,
            // reload under `PORT_AOT_VERIFY=1`, verify 0 mismatches, and execute!
            module_mapped(exe_name, exe2.base(), exe2.len);
            module_mapped(dll_name, dll2.base(), dll2.len);
            let reps2 = aot_prewarm_all_mapped(0);
            let exe2_rep = reps2.iter().find(|r| r.module == exe_name).expect("exe2 report");
            let dll2_rep = reps2.iter().find(|r| r.module == dll_name).expect("dll2 report");
            assert_eq!(exe2_rep.blocks, exe_rep.blocks);
            assert_eq!(dll2_rep.blocks, dll_rep.blocks);

            exe2.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_3.to_le_bytes());
            dll2.as_slice_mut()[0x4080..0x4088].copy_from_slice(&h_memset_3.to_le_bytes());

            let v1 = TEST_VERIFIED.with(|c| c.get());
            let m1 = TEST_VERIFY_MISMATCHES.with(|c| c.get());
            activate();
            assert_eq!(TEST_VERIFY_MISMATCHES.with(|c| c.get()) - m1, 0, "0 mismatches on shifted reload");
            assert_eq!(
                TEST_VERIFIED.with(|c| c.get()) - v1,
                exe2_rep.blocks + dll2_rep.blocks,
                "all shifted blocks verified"
            );

            let installs1 = jit::test_aot_installs();
            assert_eq!(run_all_cases(exe2.base() as u64 + 0x1000, 1), expected);
            assert!(jit::test_aot_installs() - installs1 > 0);
            // Also execute `exe1` calling across to shifted `dll2`!
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 1), expected);

            // Clean teardown.
            TEST_VERIFY.with(|c| c.set(None));
            TEST_AOT.with(|c| c.set(false));
            teardown_test_modules(&[exe_name, dll_name]);
            crate::exe::unregister_native_va(h_memset_1);
            crate::exe::unregister_native_va(h_memset_2);
            crate::exe::unregister_native_va(h_memset_3);
            let _ = std::fs::remove_dir_all(&d);
            drop(scratch_mem);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn bg_job_and_load_skip_runtime_modified_code_regions() {
    // Use an isolated high VA far from any other test's mapped code buffers (SLACK = 16 MiB).
    let base: usize = 0x6_8000_0000;
    let size: usize = 0x10000;
    crate::emu::mem::note_code_region(base, base + size);
    assert!(crate::emu::mem::jit_overlaps(base, base + size));
    let key = AotKey { image_hash: [0x11, 0x22], image_base: base as u64, image_size: size as u64, codegen_hash: 0x33, knobs: 0 };
    assert!(bg_job("runtime_mod.exe", base, size, key, None).is_none());
    assert!(unsafe { load_module("runtime_mod.exe", base, size, &key) }.is_none());
}

/// Multi-bank AOT image test (using a small test `bank_span` = 16 KiB):
/// - Verifies multiple stub banks emit cleanly with 16-byte alignment and no block overlaps;
/// - Verifies every stub branch in every bank targets its OWN bank's stub offsets within range;
/// - Verifies current and legacy stub tables and bank directories round-trip;
/// - Verifies single- vs multi-threaded multi-bank builds are bit-for-bit deterministic;
/// - Installs the multi-bank image with `Verify::All` (0 mismatches) and executes functions
///   in every bank against the interpreter oracle.
#[test]
fn aot_multibank_chunk_assembler_roundtrip_determinism_and_exec() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            const NFUNCS: usize = 160;
            const FUNC_STRIDE: usize = 0x40;
            const TEXT_RVA: u32 = 0x1000;
            const TEXT_SIZE: u32 = 0x4000;
            const RDATA_RVA: u32 = 0x5000;
            const RDATA_SIZE: u32 = 0x2000;
            const IMG_SIZE: usize = 0x8000;
            const TEST_BANK_SPAN: usize = 16 << 10; // 16 KiB per bank -> forces >= 3 banks!

            let mut pe = TestMappedBuf::alloc(IMG_SIZE, 0x2_3E00_0000);
            let base = pe.base();
            let base64 = base as u64;
            let buf = pe.as_slice_mut();
            let pdata_rva: u32 = RDATA_RVA + 0x100;
            init_synth_pe(
                buf,
                base64,
                TEXT_RVA,
                TEXT_RVA,
                TEXT_SIZE,
                RDATA_RVA,
                RDATA_SIZE,
                None,
                Some((pdata_rva, (NFUNCS * 12) as u32)),
            );

            let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
            let unwind_rva: u32 = RDATA_RVA + 0x1800;
            buf[unwind_rva as usize] = 0x01;

            for i in 0..NFUNCS {
                let rva = TEXT_RVA + (i * FUNC_STRIDE) as u32;
                let va = base64 + rva as u64;
                let mut a = CodeAssembler::new(64).unwrap();
                let mut l_alt = a.create_label();
                a.cmp(ecx, (i & 7) as i32).unwrap();
                a.ja(l_alt).unwrap();
                a.lea(eax, ptr(rcx + rdx + (i as i32))).unwrap();
                a.imul_3(eax, eax, ((i as i32) * 2) | 1).unwrap();
                a.ret().unwrap();
                a.set_label(&mut l_alt).unwrap();
                a.mov(r10d, edx).unwrap();
                a.rdtsc().unwrap();
                a.add(r10d, eax).unwrap();
                a.rdtscp().unwrap();
                a.add(eax, r10d).unwrap();
                a.add(eax, (i as i32) + 100).unwrap();
                a.ret().unwrap();
                let code = a.assemble(va).unwrap();
                assert!(code.len() <= FUNC_STRIDE);
                buf[rva as usize..rva as usize + code.len()].copy_from_slice(&code);
                let pe_off = pdata_rva as usize + i * 12;
                w32(buf, pe_off, rva);
                w32(buf, pe_off + 4, rva + FUNC_STRIDE as u32);
                w32(buf, pe_off + 8, unwind_rva);
            }

            let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
            assert_eq!(roots.len(), NFUNCS);
            let mut pool1 = WorkerCaches::new();
            let (ips, _) = unsafe { discover_with_threads(&mut pool1, base, pe.len, &roots, DEFAULT_AOT_CLOSURE, 1) };

            let img_1 = unsafe {
                build_image_with_threads_and_limits(&mut pool1, &ips, BUILD_VBASE, 1, TEST_BANK_SPAN, MAX_CODE)
            }
            .expect("1-thread multi-bank build");
            assert!(
                img_1.stub_banks.len() >= 3,
                "expected >=3 stub banks at 16 KiB span, got {}",
                img_1.stub_banks.len()
            );
            assert!(img_1.valid(), "multi-bank image must pass structural validation");
            assert_eq!(img_1.stub_banks[0], 0);
            for w in img_1.stub_banks.windows(2) {
                assert!(w[1] > w[0] && w[1] % 16 == 0);
                assert!((w[1] - w[0]) as usize <= TEST_BANK_SPAN);
            }

            // Verify every block's stub branches target its OWN stub bank within `TEST_BANK_SPAN`.
            let stubs_words = (AOT_STUBS_BYTES / 4) as i64;
            let mut blocks_per_bank = vec![0usize; img_1.stub_banks.len()];
            let mut stub_branches_checked = 0usize;
            let mut timestamp_calls = vec![[false; 2]; img_1.stub_banks.len()];
            for b in &img_1.blocks {
                let bi = img_1.bank_index_of(b.code_off);
                blocks_per_bank[bi] += 1;
                let bank_off = img_1.stub_banks[bi] as i64;
                let bank_w = bank_off / 4;
                // Check hot words and cold words (up to any literal slots; scan all B/BL targeting a stub range).
                let mut check_slice = |words: &[u32], base_off: u32, count: &mut usize| {
                    let base_w = (base_off as i64) / 4;
                    for (i, &w) in words.iter().enumerate() {
                        if (w & 0x7C00_0000) == 0x1400_0000 {
                            let imm26 = ((((w & 0x03FF_FFFF) as i32) << 6) >> 6) as i64;
                            let target_w = base_w + i as i64 + imm26;
                            if target_w >= bank_w && target_w < bank_w + stubs_words {
                                let stub_w = (target_w - bank_w) as u32;
                                assert!(
                                    img_1.stubs.contains(&stub_w),
                                    "branch into bank {bi} stub region hit non-stub word {stub_w}"
                                );
                                assert!(
                                    imm26.abs() * 4 <= TEST_BANK_SPAN as i64,
                                    "stub branch exceeded bank span: {}",
                                    imm26 * 4
                                );
                                if let Some(op) = img_1.stubs[10..12].iter().position(|&s| s == stub_w) {
                                    timestamp_calls[bi][op] = true;
                                }
                                *count += 1;
                            } else {
                                // Must NEVER target a different bank's stub area!
                                for (obi, &obo) in img_1.stub_banks.iter().enumerate() {
                                    if obi != bi {
                                        let obw = (obo as i64) / 4;
                                        assert!(
                                            !(target_w >= obw && target_w < obw + stubs_words),
                                            "block in bank {bi} branched into bank {obi}'s stubs!"
                                        );
                                    }
                                }
                            }
                        }
                    }
                };
                check_slice(img_1.hot_words_of(b), b.code_off, &mut stub_branches_checked);
                check_slice(img_1.cold_words_of(b), b.cold_off, &mut stub_branches_checked);
            }
            assert!(
                blocks_per_bank.iter().all(|&c| c > 0),
                "every bank should hold blocks: {blocks_per_bank:?}"
            );
            assert!(stub_branches_checked > 0, "expected stub branches verified across banks");

            assert!(timestamp_calls.iter().filter(|&&calls| calls == [true; 2]).count() >= 2);

            // Thread count must not affect the emitted image.
            let key = unsafe { module_key(base, pe.len) };
            let bytes_1 = encode("multibank.dll", &key, &img_1, 0);
            for threads in [2usize, 4] {
                let mut pool_p = WorkerCaches::new();
                let img_p = unsafe {
                    build_image_with_threads_and_limits(&mut pool_p, &ips, BUILD_VBASE, threads, TEST_BANK_SPAN, MAX_CODE)
                }
                .expect("parallel multi-bank build");
                assert_eq!(img_p.stub_banks, img_1.stub_banks);
                assert_eq!(
                    encode("multibank.dll", &key, &img_p, 0),
                    bytes_1,
                    "multi-bank .aot differed at {threads} threads"
                );
            }

            // Preserve both timestamp entries in the current format.
            let (name_dec, key_dec, img_dec, flags_dec) = decode(&bytes_1).expect("decode multi-bank cache");
            assert_eq!(name_dec, "multibank.dll");
            assert_eq!(key_dec, key);
            assert_eq!(flags_dec, 0);
            assert_eq!(img_dec.stub_banks, img_1.stub_banks);
            assert_eq!(img_dec.blocks, img_1.blocks);
            assert_eq!(img_dec.stubs, img_1.stubs);
            assert_eq!(img_dec.code, img_1.code);

            let v6_bytes = encode_with_version("legacy_v6.dll", &key, &img_1, 0, FORMAT_VERSION_V6);
            let (_, _, v6_img, _) = decode(&v6_bytes).expect("decode legacy multi-bank header");
            assert_eq!(v6_img.stub_banks, img_1.stub_banks);
            assert_eq!(&v6_img.stubs[..10], &img_1.stubs[..10]);
            assert_eq!(&v6_img.stubs[10..], &[u32::MAX; 6]);

            let v7_bytes = encode_with_version("legacy_v7.dll", &key, &img_1, 0, FORMAT_VERSION_V7);
            let (_, _, v7_img, _) = decode(&v7_bytes).expect("decode legacy v7 header");
            assert_eq!(v7_img.stub_banks, img_1.stub_banks);
            assert_eq!(&v7_img.stubs[..12], &img_1.stubs[..12]);
            assert_eq!(&v7_img.stubs[12..], &[u32::MAX; 4]);

            let single_img = unsafe { build_image(pool1.get_one(), &ips[..4], BUILD_VBASE) }.expect("single-bank");
            let v5_bytes = encode_with_version("legacy_v5.dll", &key, &single_img, FLAG_VERIFY_PENDING, FORMAT_VERSION_V5);
            let (v5_name, v5_key, v5_img, v5_flags) = decode(&v5_bytes).expect("decode legacy v5");
            assert_eq!(v5_name, "legacy_v5.dll");
            assert_eq!(v5_key, key);
            assert_eq!(v5_flags, FLAG_VERIFY_PENDING);
            assert_eq!(v5_img.stub_banks, vec![0]);
            assert_eq!(v5_img.blocks, single_img.blocks);

            // Install the multi-bank image with Verify::All, then execute functions from Bank 0,
            // Bank 1, and the last bank comparing AOT output against the interpreter oracle!
            TEST_AOT.with(|c| c.set(true));
            let total_blocks = img_dec.blocks.len();
            let inst = unsafe { install_image("multibank.dll", base, pe.len, img_dec, Verify::All, true) }
                .expect("install multi-bank image");
            assert_eq!(inst.mismatches, 0, "multi-bank Verify::All had mismatches");
            assert_eq!(inst.verified, total_blocks);
            assert_eq!(inst.live, total_blocks);

            // Pick one function entry IP from each bank and execute both AOT and interpreter.
            let sample_funcs = [0usize, NFUNCS / 2, NFUNCS - 1];
            for fi in sample_funcs {
                let entry = base64 + TEXT_RVA as u64 + (fi * FUNC_STRIDE) as u64;
                assert!(find(entry).is_some(), "func {fi} missing from AOT dir");
                for (a0, a1) in [(2u64, 15u64), (20u64, 33u64)] {
                    jit::force(2);
                    let run = || crate::emu::with_frozen_tsc(|| unsafe {
                        crate::emu::emu_run_full(entry, [a0, a1, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8])
                    });
                    let (exp, _) = run();
                    jit::force(1);
                    let (got, _) = run();
                    jit::force(0);
                    assert_eq!(got, exp, "multi-bank func {fi} mismatch for args ({a0}, {a1})");
                }
            }

            test_uninstall();
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Deep CFG closure + `.pdata`-less leaf callees (direct call & tail-jump) + TLS/CFG Load Config
/// root harvesting + switch/jump-table dispatch re-analysis & register const/sub/shl hints.
#[test]
fn aot_deep_closure_leaf_callees_tls_cfg_roots_and_jt_reanalysis() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0xA000, 0x2_3F00_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    // .text @ 0x1000..0x5000, .rdata @ 0x5000..0x9000, .pdata @ 0x6000 (3 entries = 36 bytes)
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x4000, 0x5000, 0x4000, None, Some((0x6000, 36)));
    let opt = 0x58;
    // DataDirectory[9]: TLS Directory @ 0x6100 (40 bytes)
    w32(buf, opt + 112 + 9 * 8, 0x6100);
    w32(buf, opt + 112 + 9 * 8 + 4, 40);
    // DataDirectory[10]: Load Config Directory @ 0x6200 (DataDirectory.Size=0x40 like link.exe, struct Size=0xC0)
    w32(buf, opt + 112 + 10 * 8, 0x6200);
    w32(buf, opt + 112 + 10 * 8 + 4, 0x40);

    // --- 1. Deep `.pdata` function @ 0x1000: 8-hop chain calling `.pdata`-less leaf @ 0x3000
    //        and tail-jumping to `.pdata`-less leaf @ 0x3100 ---
    let mut a = CodeAssembler::new(64).unwrap();
    let mut hops: Vec<CodeLabel> = (0..8).map(|_| a.create_label()).collect();
    for i in 0..8 {
        a.cmp(ecx, i as i32).unwrap();
        let lbl = hops[i];
        a.je(lbl).unwrap();
    }
    a.call(base64 + 0x3000).unwrap();
    a.jmp(base64 + 0x3100).unwrap();
    for (i, lbl) in hops.iter_mut().enumerate() {
        a.set_label(lbl).unwrap();
        a.mov(eax, (100 + i) as u32).unwrap();
        a.ret().unwrap();
    }
    let fn0 = a.assemble(base64 + 0x1000).unwrap();
    buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

    // `.pdata`-less leaf callee @ 0x3000 (calls nested `.pdata`-less leaf @ 0x3040):
    let mut a = CodeAssembler::new(64).unwrap();
    a.call(base64 + 0x3040).unwrap();
    a.add(eax, 1).unwrap();
    a.ret().unwrap();
    let leaf0 = a.assemble(base64 + 0x3000).unwrap();
    buf[0x3000..0x3000 + leaf0.len()].copy_from_slice(&leaf0);
    buf[0x3040..0x3046].copy_from_slice(&[0xB8, 7, 0, 0, 0, 0xC3]); // mov eax, 7; ret
    // `.pdata`-less tail-call leaf @ 0x3100:
    buf[0x3100..0x3106].copy_from_slice(&[0xB8, 9, 0, 0, 0, 0xC3]);

    // --- 2. Switch dispatch re-analysis @ 0x1800 ---
    //    Block 0x1800 (guard): cmp ecx, 2; lea r8, [rip + tbl_re]; jbe 0x1840; ret
    //    Block 0x1840 (dispatch): movsxd rax, [r8 + rcx*4]; add rax, r8; jmp rax
    //    We seed BOTH 0x1840 (unhinted first!) and 0x1800 to force re-analysis of 0x1840!
    let re_cases: [u32; 3] = [0x1880, 0x1890, 0x18A0];
    let tbl_re_rva: u32 = 0x5100;
    for (i, &rva) in re_cases.iter().enumerate() {
        let rel = (rva as i32) - (tbl_re_rva as i32);
        w32(buf, tbl_re_rva as usize + i * 4, rel as u32);
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (40 + i as u8), 0, 0, 0, 0xC3]);
    }
    let mut c2 = Vec::new();
    c2.extend_from_slice(&[0x83, 0xF9, 0x02]); // cmp ecx, 2
    emit_lea_rip(&mut c2, base64 + 0x1800, 8 /* r8 */, base64 + tbl_re_rva as u64);
    let jbe_next = 0x1800 + c2.len() as u32 + 6;
    c2.extend_from_slice(&[0x0F, 0x86]); // jbe 0x1840
    c2.extend_from_slice(&((0x1840i32) - (jbe_next as i32)).to_le_bytes());
    c2.push(0xC3); // ret
    buf[0x1800..0x1800 + c2.len()].copy_from_slice(&c2);

    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
    a.add(rax, r8).unwrap();
    a.jmp(rax).unwrap();
    let disp2 = a.assemble(base64 + 0x1840).unwrap();
    buf[0x1840..0x1840 + disp2.len()].copy_from_slice(&disp2);

    // --- 3. Switch with `sub` + const-register `cmp` + `shl rcx, 2` + `sub r8, rax` ---
    let sub_cases: [u32; 3] = [0x2080, 0x2090, 0x20A0];
    let tbl_sub_rva: u32 = 0x5200;
    for (i, &rva) in sub_cases.iter().enumerate() {
        // Positive backward distance from tbl_sub_rva: target = tbl_va - entry!
        let dist = tbl_sub_rva - rva;
        w32(buf, tbl_sub_rva as usize + i * 4, dist);
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (50 + i as u8), 0, 0, 0, 0xC3]);
    }
    // Decoy 4th entry (must NOT be followed because `mov edx, 2; cmp ecx, edx; ja default` caps at 3).
    w32(buf, tbl_sub_rva as usize + 12, tbl_sub_rva - 0x20B0);
    buf[0x20B0..0x20B6].copy_from_slice(&[0xB8, 0xEE, 0, 0, 0, 0xC3]);
    buf[0x20C0..0x20C6].copy_from_slice(&[0xB8, 0xFF, 0, 0, 0, 0xC3]); // default

    let mut c3 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(ecx, 10).unwrap();
    a.mov(edx, 2).unwrap();
    a.cmp(ecx, edx).unwrap(); // register const cmp!
    c3.extend_from_slice(&a.assemble(base64 + 0x2000).unwrap());
    let ja_next = 0x2000 + c3.len() as u32 + 6;
    c3.extend_from_slice(&[0x0F, 0x87]); // ja 0x20C0
    c3.extend_from_slice(&((0x20C0i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3, base64 + 0x2000, 8 /* r8 */, base64 + tbl_sub_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.shl(rcx, 2).unwrap(); // pre-scaled index!
    a.mov(eax, dword_ptr(r8 + rcx)).unwrap();
    a.sub(r8, rax).unwrap(); // sub_from_base!
    a.jmp(r8).unwrap();
    c3.extend_from_slice(&a.assemble(base64 + 0x2000 + c3.len() as u64).unwrap());
    buf[0x2000..0x2000 + c3.len()].copy_from_slice(&c3);

    // --- 4. TLS callbacks (0x3200, 0x3240) & CFG GuardCF/LongJump targets (0x3300, 0x3340, 0x3380) ---
    for rva in [0x3200usize, 0x3240, 0x3300, 0x3340, 0x3380] {
        buf[rva..rva + 6].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);
    }
    // TLS Directory @ 0x6100 -> callback array @ 0x6140
    w64(buf, 0x6100 + 24, base64 + 0x6140);
    w64(buf, 0x6140, base64 + 0x3200);
    w64(buf, 0x6148, base64 + 0x3240);
    w64(buf, 0x6150, 0);

    // Load Config @ 0x6200: GuardCFFunctionTable @ 0x6300 with 5-byte entries (extra=1) +
    // GuardLongJumpTargetTable @ 0x6340.
    w32(buf, 0x6200, 0xC0); // Size
    w64(buf, 0x6200 + 0x80, base64 + 0x6300);
    w64(buf, 0x6200 + 0x88, 2);
    w32(buf, 0x6200 + 0x90, 0x0000_0400 | 0x0001_0000 | (1 << 28)); // CF_TABLE | LONGJUMP | extra=1
    w64(buf, 0x6200 + 0xB0, base64 + 0x6340);
    w64(buf, 0x6200 + 0xB8, 1);
    w32(buf, 0x6300, 0x3300);
    buf[0x6304] = 0x01;
    w32(buf, 0x6305, 0x3340); // intentionally unaligned 5-byte stride!
    buf[0x6309] = 0x02;
    w32(buf, 0x6340, 0x3380);

    // .pdata entries: cover ONLY functions 0x1000..0x1400, 0x1800..0x1900, 0x2000..0x2100.
    for (i, (s, e)) in [(0x1000u32, 0x1400u32), (0x1800, 0x1900), (0x2000, 0x2100)].iter().enumerate() {
        w32(buf, 0x6000 + i * 12, *s);
        w32(buf, 0x6000 + i * 12 + 4, *e);
        w32(buf, 0x6000 + i * 12 + 8, 0x6080);
    }
    buf[0x6080] = 0x01;

    // 1. Verify `discover_pe_roots` harvests `.pdata` + TLS callbacks + CFG function/longjmp tables!
    let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        roots,
        vec![0x1000, 0x1800, 0x2000, 0x3200, 0x3240, 0x3300, 0x3340, 0x3380],
        "expected .pdata + TLS + CFG roots"
    );

    // 2. Verify shallow depth 2 misses the deep chain's leaf callees, whereas default depth 128
    //    discovers all leaf callees (direct call, nested leaf call, and tail-jump leaf).
    let mut bc = Box::new(BlockCache::new());
    let (shallow_ips, shallow_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], 2) };
    assert!(!shallow_ips.contains(&(base64 + 0x3000)), "depth 2 should not reach 8-hop leaf callee");
    assert_eq!(shallow_cl.leaf_callees, 0);

    let (deep_ips, deep_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], DEFAULT_AOT_CLOSURE) };
    assert!(deep_ips.contains(&(base64 + 0x3000)), "deep closure must reach direct `.pdata`-less leaf callee");
    assert!(deep_ips.contains(&(base64 + 0x3040)), "deep closure must follow nested `.pdata`-less leaf callee");
    assert!(deep_ips.contains(&(base64 + 0x3100)), "deep closure must reach tail-jump `.pdata`-less leaf");
    assert!(deep_cl.leaf_callees >= 2, "expected >=2 leaf callees, got {}", deep_cl.leaf_callees);

    // 3. Verify switch dispatch re-analysis when `0x1840` is seeded unhinted BEFORE `0x1800`
    //    reaches it with `r8 = Const(tbl_re)` + `rcx = Index { bound: Some(3) }`!
    let (re_ips, re_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1840, 0x1800], DEFAULT_AOT_CLOSURE) };
    assert_eq!(re_cl.jt_tables, 1, "re-analyzed switch dispatch should be discovered once");
    assert_eq!(re_cl.jt_targets, 3);
    for &rva in &re_cases {
        assert!(re_ips.contains(&(base64 + rva as u64)), "missing re-analyzed switch case {rva:#x}");
    }

    // 4. Verify `sub` + const-register `cmp` + `shl` scaled-index + `sub rBase, rOff` jump table!
    let (sub_ips, sub_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x2000], DEFAULT_AOT_CLOSURE) };
    assert_eq!(sub_cl.jt_tables, 1, "expected sub/shl/const-cmp switch table discovered");
    assert_eq!(sub_cl.jt_targets, 3);
    for &rva in &sub_cases {
        assert!(sub_ips.contains(&(base64 + rva as u64)), "missing sub/shl switch case {rva:#x}");
    }
    assert!(!sub_ips.contains(&(base64 + 0x20B0)), "out-of-bounds decoy past const-reg cmp must NOT be followed");
}

/// PE Base Relocation (`DataDirectory[5]`, `IMAGE_REL_BASED_DIR64`) vtable/function-pointer
/// root discovery, `PORT_AOT_RELOC_ROOTS=0` gating, rejection of non-code / padding / invalid
/// targets, and stripped-`.reloc` fallback scan of initialized data sections.
#[test]
fn aot_base_reloc_dir64_vtable_roots_and_stripped_fallback() {
    let _env = TestEnvGuard::new(&["PORT_AOT_RELOC_ROOTS"]);
    std::env::remove_var("PORT_AOT_RELOC_ROOTS");

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_4000_0000);
    let base = pe.base();
    let base64 = base as u64;
    let opt = 0x58usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    {
        let buf = pe.as_slice_mut();

        // .text @ 0x1000..0x4000, .rdata @ 0x4000..0x7000, .pdata @ 0x5000 (1 entry = 12 bytes).
        init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x5000, 12)));
        w32(buf, 0x5000, 0x1000);
        w32(buf, 0x5004, 0x1040);
        w32(buf, 0x5008, 0x5040);
        buf[0x5040] = 0x01; // UNWIND_INFO v1

        // Entry function @ 0x1000 (in .pdata).
        buf[0x1000..0x1006].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]); // mov eax, 1; ret
        // `.pdata`-less leaf virtual method @ 0x1200: mov al, 1; ret
        buf[0x1200..0x1203].copy_from_slice(&[0xB0, 0x01, 0xC3]);
        // `.pdata`-less adjustor thunk @ 0x1203 (immediately after `ret`, not 16-byte aligned):
        // sub rcx, 8; ret
        buf[0x1203..0x1208].copy_from_slice(&[0x48, 0x83, 0xE9, 0x08, 0xC3]);
        // Padding / invalid targets in `.text` that must be rejected:
        buf[0x1240..0x1244].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]); // int3 padding
        // 0x1260..0x1268 left zeroed (`0x00, 0x00` padding)
        buf[0x1280..0x1284].copy_from_slice(&[0x06, 0x07, 0x0E, 0x16]); // invalid 64-bit opcode (PUSH ES)
        // Additional valid 16-byte-aligned leaf @ 0x12C0 (pointed to by non-DIR64 relocs only):
        buf[0x12C0..0x12C3].copy_from_slice(&[0xB0, 0x02, 0xC3]);
        // Unaligned non-boundary code @ 0x1305 (preceded by 0x42, NOT 0xCC/0x90/0xC3):
        buf[0x1304] = 0x42;
        buf[0x1305..0x130B].copy_from_slice(&[0xB8, 3, 0, 0, 0, 0xC3]);

        // Vtable / pointer slots in `.rdata` @ 0x4100:
        w64(buf, 0x4100, base64 + 0x1200); // valid leaf
        w64(buf, 0x4108, base64 + 0x1203); // valid adjustor thunk
        w64(buf, 0x4110, base64 + 0x4800); // non-code `.rdata` pointer
        w64(buf, 0x4118, base64 + 0x1240); // `0xCC` padding
        w64(buf, 0x4120, base64 + 0x1260); // `[0x00, 0x00]` padding
        w64(buf, 0x4128, base64 + 0x1280); // invalid opcode
        w64(buf, 0x4130, base64 + 0x12C0); // valid leaf, only referenced by ABSOLUTE/HIGHLOW relocs

        // Base Relocation Directory (`DataDirectory[5]`) @ 0x6000:
        // Block 0: page_rva = 0x4000, 8 u16 entries (block_size = 24), followed by zero block (8B).
        w32(buf, opt + 112 + 5 * 8, 0x6000);
        w32(buf, opt + 112 + 5 * 8 + 4, 32);
        w32(buf, 0x6000, 0x4000); // page_rva
        w32(buf, 0x6004, 24); // block_size
        let relocs: [u16; 8] = [
            (10 << 12) | 0x100, // DIR64 -> 0x1200 (accept)
            (10 << 12) | 0x108, // DIR64 -> 0x1203 (accept)
            (10 << 12) | 0x110, // DIR64 -> .rdata (reject)
            (10 << 12) | 0x118, // DIR64 -> 0xCC (reject)
            (10 << 12) | 0x120, // DIR64 -> 0x00,0x00 (reject)
            (10 << 12) | 0x128, // DIR64 -> invalid opcode (reject)
            (0 << 12) | 0x130,  // ABSOLUTE padding -> ignore
            (3 << 12) | 0x130,  // HIGHLOW -> ignore
        ];
        for (i, &ent) in relocs.iter().enumerate() {
            w16(buf, 0x6008 + i * 2, ent);
        }
        // Trailing zero block (`block_size = 0`) at 0x6018 must terminate cleanly without looping.
    }

    let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        roots,
        vec![0x1000, 0x1200, 0x1203],
        "expected .pdata root + DIR64 valid code targets only"
    );

    // Gating with `PORT_AOT_RELOC_ROOTS=0` disables `.reloc` root harvesting.
    std::env::set_var("PORT_AOT_RELOC_ROOTS", "0");
    let gated = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(gated, vec![0x1000], "PORT_AOT_RELOC_ROOTS=0 must disable reloc root discovery");
    std::env::remove_var("PORT_AOT_RELOC_ROOTS");

    // Stripped `.reloc` fallback: clear DataDirectory[5] and add a pointer to unaligned non-boundary
    // code (`0x1305`); fallback must recover clean entries (0x1200, 0x1203, 0x12C0) and reject 0x1305.
    {
        let buf = pe.as_slice_mut();
        w32(buf, opt + 112 + 5 * 8, 0);
        w32(buf, opt + 112 + 5 * 8 + 4, 0);
        w64(buf, 0x4138, base64 + 0x1305);
    }
    let fallback_roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        fallback_roots,
        vec![0x1000, 0x1200, 0x1203, 0x12C0],
        "stripped-.reloc fallback must find clean-boundary code pointers and reject unaligned/padding targets"
    );
}

/// `LEA reg, [rip + disp32]` discovery of `.pdata`-less leaf callbacks/methods via
/// [`EdgeKind::LeaLeaf`] (both with `jt_on = true` and `jt_on = false`), and rejection of
/// `.pdata`-covered targets, non-code sections, padding, invalid encodings, and unaligned
/// non-boundary data constants embedded in `.text`.
#[test]
fn aot_lea_rip_leaf_discovery_and_text_data_rejection() {
    let _env = TestEnvGuard::new(&["PORT_AOT_JUMPTABLES", "AW2_DLL_AOT_JUMPTABLES"]);
    std::env::remove_var("PORT_AOT_JUMPTABLES");
    std::env::remove_var("AW2_DLL_AOT_JUMPTABLES");

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_4100_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());

    // .text @ 0x1000..0x4000, .rdata @ 0x4000..0x7000, .pdata @ 0x5000 (2 entries = 24 bytes).
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x5000, 24)));
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1240)].iter().enumerate() {
        w32(buf, 0x5000 + i * 12, *s);
        w32(buf, 0x5000 + i * 12 + 4, *e);
        w32(buf, 0x5000 + i * 12 + 8, 0x5080);
    }
    buf[0x5080] = 0x01;

    // 1. Valid `.pdata`-less 16-byte-aligned leaf @ 0x2000: `mov al, 1; ret`
    buf[0x2000..0x2003].copy_from_slice(&[0xB0, 0x01, 0xC3]);
    // 2. Valid `.pdata`-less leaf @ 0x2045 (not 16-byte aligned, preceded by `0xCC`): `xor eax, eax; ret`
    buf[0x2044] = 0xCC;
    buf[0x2045..0x2048].copy_from_slice(&[0x31, 0xC0, 0xC3]);
    // 3. `.pdata`-covered function @ 0x1200 (must NOT be added via LeaLeaf when seeded from 0x1000):
    buf[0x1200..0x1206].copy_from_slice(&[0xB8, 7, 0, 0, 0, 0xC3]);
    // 4. `.text` data constants that MUST be rejected:
    buf[0x2100..0x2104].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]); // `0xCC` padding
    // 0x2120 left zeroed (`[0x00, 0x00]` constant)
    buf[0x2140..0x2144].copy_from_slice(&[0x06, 0x07, 0x0E, 0x16]); // invalid 64-bit opcode
    buf[0x2162] = 0x55; // non-boundary preceding byte (not 0xCC/0x90/0xC3)
    buf[0x2163..0x2166].copy_from_slice(&[0xB0, 0x01, 0xC3]); // unaligned data constant in `.text`
    // 5. `.rdata` bytes @ 0x4200 (outside `.text`):
    buf[0x4200..0x4203].copy_from_slice(&[0xB0, 0x01, 0xC3]);

    // Caller @ 0x1000 loads addresses via RIP-relative `LEA` without calling any of them.
    let mut c = Vec::new();
    emit_lea_rip(&mut c, base64 + 0x1000, 0 /* rax */, base64 + 0x2000);
    emit_lea_rip(&mut c, base64 + 0x1000, 1 /* rcx */, base64 + 0x2045);
    emit_lea_rip(&mut c, base64 + 0x1000, 2 /* rdx */, base64 + 0x1200);
    emit_lea_rip(&mut c, base64 + 0x1000, 8 /* r8  */, base64 + 0x2100);
    emit_lea_rip(&mut c, base64 + 0x1000, 9 /* r9  */, base64 + 0x2120);
    emit_lea_rip(&mut c, base64 + 0x1000, 10 /* r10 */, base64 + 0x2140);
    emit_lea_rip(&mut c, base64 + 0x1000, 11 /* r11 */, base64 + 0x2163);
    emit_lea_rip(&mut c, base64 + 0x1000, 0 /* rax */, base64 + 0x4200);
    c.push(0xC3); // ret
    buf[0x1000..0x1000 + c.len()].copy_from_slice(&c);

    let mut bc = Box::new(BlockCache::new());
    for jt_flag in ["1", "0"] {
        std::env::set_var("PORT_AOT_JUMPTABLES", jt_flag);
        let (ips, cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], DEFAULT_AOT_CLOSURE) };
        assert_eq!(cl.lea_leaves, 2, "jt={jt_flag}: expected exactly 2 LEA leaf functions");
        assert!(ips.contains(&(base64 + 0x2000)), "jt={jt_flag}: missing 16B-aligned LEA leaf");
        assert!(ips.contains(&(base64 + 0x2045)), "jt={jt_flag}: missing post-0xCC LEA leaf");
        for rejected in [0x1200u64, 0x2100, 0x2120, 0x2140, 0x2163, 0x4200] {
            assert!(
                !ips.contains(&(base64 + rejected)),
                "jt={jt_flag}: target {rejected:#x} must NOT be falsely discovered"
            );
        }
    }
}

#[test]
fn cached_preferred_base_reads_matching_header_only() {
    let dir = std::env::temp_dir().join(format!("port-aot-prefbase-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let primary = dir.join("primary");
    let fallback = dir.join("fallback");
    std::fs::create_dir_all(&primary).unwrap();
    std::fs::create_dir_all(&fallback).unwrap();

    let cg = codegen_hash();
    let fb_key = AotKey {
        image_hash: [1, 2],
        image_base: 0x1_4000_0000,
        image_size: 0x0487_6000,
        codegen_hash: cg,
        knobs: 0,
    };
    let fb_path = fallback.join(format!("overwatch.exe-{:016x}.aot", fb_key.id()));
    std::fs::write(&fb_path, encode("overwatch.exe", &fb_key, &Image::default(), 0)).unwrap();

    // Fallback hit when primary is empty
    assert_eq!(
        read_cached_preferred_base_in(&primary, std::slice::from_ref(&fallback), "overwatch.exe", 0x0487_6000, cg),
        Some(0x1_4000_0000)
    );
    // Wrong size or wrong codegen hash must not match
    assert_eq!(
        read_cached_preferred_base_in(&primary, std::slice::from_ref(&fallback), "overwatch.exe", 0x0400_0000, cg),
        None
    );
    assert_eq!(
        read_cached_preferred_base_in(&primary, std::slice::from_ref(&fallback), "overwatch.exe", 0x0487_6000, cg ^ 1),
        None
    );

    // Primary overrides fallback
    let pri_key = AotKey {
        image_hash: [3, 4],
        image_base: 0x1_5000_0000,
        image_size: 0x0487_6000,
        codegen_hash: cg,
        knobs: 0,
    };
    let pri_path = primary.join(format!("overwatch.exe-{:016x}.aot", pri_key.id()));
    std::fs::write(&pri_path, encode("overwatch.exe", &pri_key, &Image::default(), 0)).unwrap();
    assert_eq!(
        read_cached_preferred_base_in(&primary, std::slice::from_ref(&fallback), "overwatch.exe", 0x0487_6000, cg),
        Some(0x1_5000_0000)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Build a synthetic mapped PE32+ plugin image at `buf` (relocated to `buf.as_ptr()`) covering:
/// - Exported `run_register` (ord 1, @ 0x1000) calling:
///   - `kernel32.dll!ReadFileEx` with 5th stack arg `[rsp + 0x20]` -> `0x1200` (APC)
///   - `kernel32.dll!CreateThread` with 3rd arg `r8` -> `0x1220` (Worker)
///   - `kernel32.dll!FlsAlloc` with 1st arg `rcx` -> `0x1240` (FLS)
///   - `comctl32.dll!#410` (ordinal 410 = SetWindowSubclass) with 2nd arg `rdx` -> `0x1260` (WndProc)
/// - `.CRT$XCA` ctor pointer -> `0x1280` (CrtCtorDtor)
/// - `.pdata` `UNW_FLAG_EHANDLER` handler -> `0x12A0` (UnwindHandler) + `C_SCOPE_TABLE` JumpTarget -> `0x10B0` (EhContinuation)
/// - Load Config `GuardEHContinuationTable` -> `0x10B0` (EhContinuation)
/// - Static COM sink vtable in `.rdata` @ `0x2600` (3 methods: `0x12C0`, `0x12E0`, `0x1300`) stored into `[rcx]`
/// - Unreferenced hidden callback @ `0x1340` (NOT in exports, `.pdata`, `.reloc`, or static reg sites;
///   only discovered at runtime and replayed across base-address relocation via ImageFingerprint!)
/// - Non-executable `.rdata` sentinel @ `0x2700` (must NEVER be recorded or replayed: code vs data!)
fn fill_callback_plugin_pe(buf: &mut [u8], timestamp: u32, extra_text_byte: u8) {
    buf.fill(0);
    let base = buf.as_ptr() as u64;
    let opt_base: u64 = 0x1_8000_0000;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(buf, 0, 0x5A4D);
    w32(buf, 0x3C, 0x80);
    w32(buf, 0x80, 0x0000_4550);
    w16(buf, 0x84, 0x8664);
    w16(buf, 0x86, 4); // .text, .rdata, .CRT$XCA, .reloc
    w32(buf, 0x88, timestamp); // TimeDateStamp
    w16(buf, 0x94, 240);
    w16(buf, 0x96, 0x2022);
    let opt = 0x98;
    w16(buf, opt, 0x20B);
    w32(buf, opt + 16, 0x1000);
    w64(buf, opt + 24, opt_base);
    w32(buf, opt + 32, 0x1000);
    w32(buf, opt + 36, 0x1000);
    w32(buf, opt + 56, buf.len() as u32);
    w32(buf, opt + 60, 0x400);
    w32(buf, opt + 108, 16);

    // Data directories:
    // [0] Export @ 0x2000 (0x100)
    w32(buf, opt + 112, 0x2000);
    w32(buf, opt + 116, 0x100);
    // [1] Import @ 0x2100 (0x100)
    w32(buf, opt + 112 + 8, 0x2100);
    w32(buf, opt + 112 + 12, 0x100);
    // [3] .pdata @ 0x2300 (12)
    w32(buf, opt + 112 + 3 * 8, 0x2300);
    w32(buf, opt + 112 + 3 * 8 + 4, 12);
    // [5] .reloc @ 0x3000 (16)
    w32(buf, opt + 112 + 5 * 8, 0x3000);
    w32(buf, opt + 112 + 5 * 8 + 4, 16);
    // [10] Load Config @ 0x2400 (0x140)
    w32(buf, opt + 112 + 10 * 8, 0x2400);
    w32(buf, opt + 112 + 10 * 8 + 4, 0x140);

    let sec = opt + 240;
    let put_sec = |b: &mut [u8], idx: usize, name: &[u8; 8], rva: u32, sz: u32, ch: u32| {
        let o = sec + idx * 40;
        b[o..o + 8].copy_from_slice(name);
        w32(b, o + 8, sz);
        w32(b, o + 12, rva);
        w32(b, o + 16, sz);
        w32(b, o + 20, rva);
        w32(b, o + 36, ch);
    };
    put_sec(buf, 0, b".text\0\0\0", 0x1000, 0x1000, 0x6000_0020);
    put_sec(buf, 1, b".rdata\0\0", 0x2000, 0x0800, 0x4000_0040);
    put_sec(buf, 2, b".CRT$XCA", 0x2800, 0x0200, 0x4000_0040);
    put_sec(buf, 3, b".reloc\0\0", 0x3000, 0x0200, 0x4200_0040);

    // Exports @ 0x2000: ord 1 -> run_register @ 0x1000
    w32(buf, 0x2000 + 16, 1);
    w32(buf, 0x2000 + 20, 1);
    w32(buf, 0x2000 + 24, 1);
    w32(buf, 0x2000 + 28, 0x2040);
    w32(buf, 0x2000 + 32, 0x2050);
    w32(buf, 0x2000 + 36, 0x2060);
    w32(buf, 0x2040, 0x1000);
    w32(buf, 0x2050, 0x2070);
    w16(buf, 0x2060, 0);
    buf[0x2070..0x2070 + 13].copy_from_slice(b"run_register\0");

    // Imports @ 0x2100:
    // Desc 0: kernel32.dll -> ReadFileEx (0x2200), CreateThread (0x2208), FlsAlloc (0x2210)
    // Desc 1: comctl32.dll -> #410 (0x2220)
    w32(buf, 0x2100, 0x2160);
    w32(buf, 0x2100 + 12, 0x21A0);
    w32(buf, 0x2100 + 16, 0x2200);
    w32(buf, 0x2114, 0x2180);
    w32(buf, 0x2114 + 12, 0x21B0);
    w32(buf, 0x2114 + 16, 0x2220);
    w64(buf, 0x2160, 0x21C0);
    w64(buf, 0x2168, 0x21D0);
    w64(buf, 0x2170, 0x21E4);
    w64(buf, 0x2180, (1u64 << 63) | 410); // ordinal 410 = SetWindowSubclass
    buf[0x21A0..0x21A0 + 13].copy_from_slice(b"kernel32.dll\0");
    buf[0x21B0..0x21B0 + 13].copy_from_slice(b"comctl32.dll\0");
    buf[0x21C2..0x21C2 + 11].copy_from_slice(b"ReadFileEx\0");
    buf[0x21D2..0x21D2 + 13].copy_from_slice(b"CreateThread\0");
    buf[0x21E6..0x21E6 + 9].copy_from_slice(b"FlsAlloc\0");

    // .pdata @ 0x2300: 0x1000..0x10D0 with UNW_FLAG_EHANDLER -> 0x12A0 + JumpTarget -> 0x10B0
    w32(buf, 0x2300, 0x1000);
    w32(buf, 0x2304, 0x10D0);
    w32(buf, 0x2308, 0x2320);
    buf[0x2320] = 0x01 | (1 << 3);
    w32(buf, 0x2324, 0x12A0);
    w32(buf, 0x2328, 1);
    w32(buf, 0x232C, 0x1004);
    w32(buf, 0x2330, 0x10A0);
    w32(buf, 0x2334, 1);
    w32(buf, 0x2338, 0x10B0);

    // Load Config @ 0x2400 with GuardEHContinuationTable @ 0x2560 (relocated VA)
    w32(buf, 0x2400, 0x140);
    w32(buf, 0x2400 + 0x90, 0x0040_0000);
    w64(buf, 0x2400 + 0x110, base + 0x2560);
    w64(buf, 0x2400 + 0x118, 1);
    w32(buf, 0x2560, 0x10B0);

    // Static COM vtable @ 0x2600 (3 methods: QI=0x12C0, AddRef=0x12E0, Release=0x1300)
    w64(buf, 0x2600, base + 0x12C0);
    w64(buf, 0x2608, base + 0x12E0);
    w64(buf, 0x2610, base + 0x1300);

    // .CRT$XCA @ 0x2800: ctor -> 0x1280
    w64(buf, 0x2800, base + 0x1280);

    // .text @ 0x1000: static registration sites + COM vtable store + DIR64 relocated imm64 @ 0x1080
    let mut c = Vec::new();
    let emit_lea_rip = |v: &mut Vec<u8>, reg_code: u8, target_rva: u32| {
        let next_rva = 0x1000u32 + v.len() as u32 + 7;
        let rex = 0x48 | ((reg_code >> 3) << 2);
        let modrm = 0x05 | ((reg_code & 7) << 3);
        let disp = (target_rva as i32 - next_rva as i32).to_le_bytes();
        v.extend_from_slice(&[rex, 0x8D, modrm, disp[0], disp[1], disp[2], disp[3]]);
    };
    let emit_call_iat = |v: &mut Vec<u8>, iat_rva: u32| {
        let next_rva = 0x1000u32 + v.len() as u32 + 6;
        let disp = (iat_rva as i32 - next_rva as i32).to_le_bytes();
        v.extend_from_slice(&[0xFF, 0x15, disp[0], disp[1], disp[2], disp[3]]);
    };
    // sub rsp, 0x38
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]);
    // 1) ReadFileEx with 5th arg [rsp + 0x20] = 0x1200
    emit_lea_rip(&mut c, 0 /* rax */, 0x1200);
    c.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x20]); // mov [rsp+0x20], rax
    emit_call_iat(&mut c, 0x2200);
    // 2) CreateThread with r8 = 0x1220
    emit_lea_rip(&mut c, 8 /* r8 */, 0x1220);
    emit_call_iat(&mut c, 0x2208);
    // 3) FlsAlloc with rcx = 0x1240
    emit_lea_rip(&mut c, 1 /* rcx */, 0x1240);
    emit_call_iat(&mut c, 0x2210);
    // 4) comctl32!#410 (SetWindowSubclass) with rdx = 0x1260
    emit_lea_rip(&mut c, 2 /* rdx */, 0x1260);
    emit_call_iat(&mut c, 0x2220);
    // 5) Store COM vtable 0x2600 into [rcx]
    emit_lea_rip(&mut c, 0 /* rax */, 0x2600);
    c.extend_from_slice(&[0x48, 0x89, 0x01]); // mov [rcx], rax
    // add rsp, 0x38; ret
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38, 0xC3]);
    assert!(c.len() <= 0x80);
    buf[0x1000..0x1000 + c.len()].copy_from_slice(&c);

    // Relocated 64-bit immediate in .text @ 0x1080 (`mov rax, base + 0x2700; ret`) with a DIR64 reloc
    // at 0x1082 so `image_fingerprint` exercises DIR64 un-relocation inside `.text`!
    buf[0x1080] = 0x48;
    buf[0x1081] = 0xB8;
    w64(buf, 0x1082, base + 0x2700);
    buf[0x108A] = 0xC3;
    // .reloc block @ 0x3000: page 0x1000, size 12 (+2 pad -> 16), 1 DIR64 entry @ 0x082
    w32(buf, 0x3000, 0x1000);
    w32(buf, 0x3004, 12);
    w16(buf, 0x3008, (10 << 12) | 0x082);

    // EH continuation landing pad @ 0x10B0
    buf[0x10B0..0x10B6].copy_from_slice(&[0xB8, 0xE4, 0x00, 0x00, 0x00, 0xC3]);

    // Callback bodies:
    let put_ret_imm = |b: &mut [u8], rva: usize, imm: u8| {
        b[rva..rva + 6].copy_from_slice(&[0xB8, imm, 0x00, 0x00, 0x00, 0xC3]);
    };
    put_ret_imm(buf, 0x1200, 0x11); // APC
    put_ret_imm(buf, 0x1220, 0x22); // Worker
    put_ret_imm(buf, 0x1240, 0x33); // FLS
    put_ret_imm(buf, 0x1260, 0x44); // WndProc (SetWindowSubclass #410)
    put_ret_imm(buf, 0x1280, 0x55); // CRT ctor
    put_ret_imm(buf, 0x12A0, 0x66); // Unwind handler
    put_ret_imm(buf, 0x12C0, 0x71); // COM QI
    put_ret_imm(buf, 0x12E0, 0x72); // COM AddRef
    put_ret_imm(buf, 0x1300, 0x73); // COM Release
    // Runtime-only hidden callback @ 0x1340: returns rcx + extra_text_byte
    // mov rax, rcx; add rax, imm8; ret
    buf[0x1340..0x1348].copy_from_slice(&[0x48, 0x89, 0xC8, 0x48, 0x83, 0xC0, extra_text_byte, 0xC3]);
}

#[test]
fn callback_roots_fingerprint_relocation_replay_and_stale_invalidation() {
    use super::callbacks::{
        clear_runtime_callbacks_for_test, discover_all_callback_roots, flush_runtime_callbacks,
        image_fingerprint, load_replayed_callback_roots, record_runtime_callback, CallbackKind,
        CallbackRoot,
    };

    let dir = test_dir("cb-replay");
    let prev_aot_dir = std::env::var_os("PORT_AOT_DIR");
    let prev_aw2_dir = std::env::var_os("AW2_DLL_AOT_DIR");
    std::env::set_var("PORT_AOT_DIR", &dir);
    std::env::remove_var("AW2_DLL_AOT_DIR");
    clear_runtime_callbacks_for_test();

    // Two distinct mapped addresses simulating relocation across runs.
    let mut img1 = vec![0u8; 0x4000];
    let mut img2 = vec![0u8; 0x4000];
    let mut img_stale = vec![0u8; 0x4000];
    fill_callback_plugin_pe(&mut img1, 0x6700_0001, 7);
    fill_callback_plugin_pe(&mut img2, 0x6700_0001, 7);
    fill_callback_plugin_pe(&mut img_stale, 0x6700_0001, 9); // 1-byte code change -> stale!

    let b1 = img1.as_ptr() as usize;
    let b2 = img2.as_ptr() as usize;
    let bs = img_stale.as_ptr() as usize;
    assert_ne!(b1, b2);

    // 1. Fingerprint is relocation-invariant (even with a DIR64 reloc in `.text`) and
    //    changes immediately on any code modification (stale image invalidation).
    let fp1 = unsafe { image_fingerprint(b1, img1.len()) }.expect("fp1");
    let fp2 = unsafe { image_fingerprint(b2, img2.len()) }.expect("fp2");
    let fps = unsafe { image_fingerprint(bs, img_stale.len()) }.expect("fps");
    assert_eq!(fp1, fp2, "identical PE relocated to another base must share fingerprint");
    assert_ne!(fp1, fps, "modified PE must invalidate image fingerprint");

    // 2. Static callback discovery covers APC (ReadFileEx arg5), Worker, FLS, WndProc (#410),
    //    CRT ctor, UnwindHandler, EhContinuation (distinct context), and COM sink vtable.
    let static_roots = unsafe { discover_all_callback_roots("cb_plugin.dll", b1, img1.len()) };
    let has = |rva: u32, kind: CallbackKind| {
        static_roots.contains(&CallbackRoot { rva, kind })
    };
    assert!(has(0x1200, CallbackKind::Apc), "ReadFileEx 5th-arg stack callback: {static_roots:?}");
    assert!(has(0x1220, CallbackKind::Worker), "CreateThread callback");
    assert!(has(0x1240, CallbackKind::Fls), "FlsAlloc callback");
    assert!(has(0x1260, CallbackKind::WndProc), "SetWindowSubclass (#410) callback");
    assert!(has(0x1280, CallbackKind::CrtCtorDtor), ".CRT$XCA ctor");
    assert!(has(0x12A0, CallbackKind::UnwindHandler), ".pdata unwind handler");
    assert!(has(0x10B0, CallbackKind::EhContinuation), "EH continuation (distinct context)");
    assert!(CallbackKind::EhContinuation.is_eh_continuation());
    assert!(!CallbackKind::WndProc.is_eh_continuation());
    assert!(has(0x12C0, CallbackKind::ComSink) && has(0x12E0, CallbackKind::ComSink) && has(0x1300, CallbackKind::ComSink), "COM sink vtable");
    // Hidden callback 0x1340 is NOT yet known statically; data sentinel 0x2700 must NEVER appear.
    assert!(!static_roots.iter().any(|c| c.rva == 0x1340));
    assert!(!static_roots.iter().any(|c| c.rva == 0x2700));

    // 3. Runtime callback recording enforces guest-vs-host and code-vs-data discipline,
    //    persists only module-relative RVAs + fingerprint, and replays after relocation!
    module_mapped("cb_plugin.dll", b1, img1.len());
    // Reject null, data-section pointer, and host-native VA.
    assert!(!record_runtime_callback(0, CallbackKind::GenericCallback));
    assert!(!record_runtime_callback(b1 as u64 + 0x2700, CallbackKind::GenericCallback));
    let fake_host_va = b1 as u64 + 0x1340;
    // Registering a native VA temporarily proves host-callable pointers are rejected.
    crate::exe::register_native_va(fake_host_va, 0x1234_5678);
    assert!(!record_runtime_callback(fake_host_va, CallbackKind::GenericCallback));
    crate::exe::unregister_native_va(fake_host_va);

    // Record the real runtime-discovered hidden guest callback @ base1 + 0x1340 and flush.
    assert!(record_runtime_callback(b1 as u64 + 0x1340, CallbackKind::Timer));
    flush_runtime_callbacks();

    // Verify persisted file contains NO raw host pointers or raw base1 VA.
    let cb_file = dir.join("callbacks").join(format!("cb_plugin.dll-{}.tsv", fp1.hex()));
    let persisted = std::fs::read_to_string(&cb_file).expect("read persisted callbacks");
    assert!(persisted.contains("0x00001340\ttimer"), "expected RVA entry in:\n{persisted}");
    assert!(!persisted.contains(&format!("{b1:x}")), "must not persist raw base VA");

    // 4. Relocated image at `b2` replays `0x1340` cleanly; stale image at `bs` rejects it!
    let replayed_b2 = unsafe { load_replayed_callback_roots("cb_plugin.dll", b2, img2.len()) };
    assert!(
        replayed_b2.contains(&CallbackRoot { rva: 0x1340, kind: CallbackKind::Timer }),
        "relocated image must replay runtime-discovered callback RVA: {replayed_b2:?}"
    );
    let replayed_stale =
        unsafe { load_replayed_callback_roots("cb_plugin.dll", bs, img_stale.len()) };
    assert!(
        replayed_stale.is_empty(),
        "stale modified image must NOT replay old callback RVAs: {replayed_stale:?}"
    );

    // 5. Cold prewarm of the relocated image at `b2` includes the replayed callback `0x1340`,
    //    and activating AOT hits `b2 + 0x1340` on the VERY FIRST callback hit after relocation!
    module_mapped("cb_plugin.dll", b2, img2.len());
    let reports = aot_prewarm_all_mapped(0);
    let rep_b2 = reports.iter().rev().find(|r| r.module == "cb_plugin.dll").expect("b2 prewarm");
    assert!(rep_b2.cache_path.as_ref().is_some_and(|p| p.is_file()));

    let key_b2 = unsafe { module_key(b2, img2.len()) };
    TEST_AOT.with(|c| c.set(true));
    let loaded_ips = unsafe { load_module("cb_plugin.dll", b2, img2.len(), &key_b2) }
        .expect("load relocated prewarmed cache");
    assert!(
        loaded_ips.contains(&(b2 as u64 + 0x1340)),
        "prewarmed relocated cache must contain replayed callback block 0x1340"
    );
    assert!(
        find(b2 as u64 + 0x1340).is_some(),
        "first callback hit after relocation must hit live AOT directory directly"
    );
    let res = unsafe { crate::emu::emu_run(b2 as u64 + 0x1340, [40, 0, 0, 0, 0, 0, 0, 0]) };
    assert_eq!(res, 47, "callback(40) = 40 + 7");
    TEST_AOT.with(|c| c.set(false));

    // Clean up test env & MAPPED entries.
    if let Ok(mut m) = MAPPED.lock() {
        m.retain(|x| x.name != "cb_plugin.dll");
    }
    clear_runtime_callbacks_for_test();
    match prev_aot_dir {
        Some(v) => std::env::set_var("PORT_AOT_DIR", v),
        None => std::env::remove_var("PORT_AOT_DIR"),
    }
    match prev_aw2_dir {
        Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
        None => std::env::remove_var("AW2_DLL_AOT_DIR"),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// Regression for PR #2 review finding: `Loader::load_pe_with_opts` rewrites
/// `OptionalHeader.ImageBase` to the actual mapped base after applying DIR64 relocations.
/// `image_fingerprint` must normalize relocated executable-section immediates against the
/// mapped base (module-relative offset) so distinct mappings of the same PE produce the
/// exact same fingerprint and replay persisted callback roots, while still rejecting both
/// modified non-relocated code bytes and modified DIR64 target RVAs.
#[test]
fn image_fingerprint_dir64_normalizes_with_rewritten_optional_header_image_base() {
    use super::callbacks::{
        clear_runtime_callbacks_for_test, flush_runtime_callbacks, image_fingerprint,
        load_replayed_callback_roots, record_runtime_callback, CallbackKind, CallbackRoot,
    };

    let dir = test_dir("cb-fp-header-rewrite");
    let prev_aot_dir = std::env::var_os("PORT_AOT_DIR");
    let prev_aw2_dir = std::env::var_os("AW2_DLL_AOT_DIR");
    std::env::set_var("PORT_AOT_DIR", &dir);
    std::env::remove_var("AW2_DLL_AOT_DIR");
    clear_runtime_callbacks_for_test();

    // Helper: fill synthetic PE with a DIR64 relocation in `.text` (0x1082 -> target_rva)
    // and optionally rewrite OptionalHeader.ImageBase (opt + 24 = 0x98 + 24) to the mapped base,
    // matching `Loader::load_pe_with_opts` lines 1048-1055.
    let build_mapped = |buf: &mut [u8], extra_text_byte: u8, dir64_target_rva: u64, rewrite_opt_base: bool| {
        fill_callback_plugin_pe(buf, 0x6700_0042, extra_text_byte);
        let mapped_base = buf.as_ptr() as u64;
        // Overwrite the DIR64 relocated slot @ 0x1082 with `mapped_base + dir64_target_rva`.
        buf[0x1082..0x108A].copy_from_slice(&mapped_base.wrapping_add(dir64_target_rva).to_le_bytes());
        if rewrite_opt_base {
            let e_lfanew = u32::from_le_bytes(buf[0x3C..0x40].try_into().unwrap()) as usize;
            buf[e_lfanew + 24 + 24..e_lfanew + 24 + 32].copy_from_slice(&mapped_base.to_le_bytes());
        }
    };

    let mut img_a = vec![0u8; 0x4000];
    let mut img_b = vec![0u8; 0x4000];
    let mut img_unrewritten = vec![0u8; 0x4000];
    let mut img_stale_code = vec![0u8; 0x4000];
    let mut img_stale_reloc_target = vec![0u8; 0x4000];

    build_mapped(&mut img_a, 7, 0x2700, true);
    build_mapped(&mut img_b, 7, 0x2700, true);
    build_mapped(&mut img_unrewritten, 7, 0x2700, false);
    build_mapped(&mut img_stale_code, 9, 0x2700, true);
    build_mapped(&mut img_stale_reloc_target, 7, 0x2708, true);

    let ba = img_a.as_ptr() as usize;
    let bb = img_b.as_ptr() as usize;
    let bu = img_unrewritten.as_ptr() as usize;
    let bsc = img_stale_code.as_ptr() as usize;
    let bsr = img_stale_reloc_target.as_ptr() as usize;
    assert_ne!(ba, bb);

    // Confirm OptionalHeader.ImageBase really was rewritten to each distinct mapped base.
    let read_opt_base = |buf: &[u8]| -> u64 {
        let e_lfanew = u32::from_le_bytes(buf[0x3C..0x40].try_into().unwrap()) as usize;
        u64::from_le_bytes(buf[e_lfanew + 24 + 24..e_lfanew + 24 + 32].try_into().unwrap())
    };
    assert_eq!(read_opt_base(&img_a), ba as u64);
    assert_eq!(read_opt_base(&img_b), bb as u64);
    assert_ne!(read_opt_base(&img_a), read_opt_base(&img_b));

    let fp_a = unsafe { image_fingerprint(ba, img_a.len()) }.expect("fp_a");
    let fp_b = unsafe { image_fingerprint(bb, img_b.len()) }.expect("fp_b");
    let fp_u = unsafe { image_fingerprint(bu, img_unrewritten.len()) }.expect("fp_u");
    let fp_sc = unsafe { image_fingerprint(bsc, img_stale_code.len()) }.expect("fp_sc");
    let fp_sr = unsafe { image_fingerprint(bsr, img_stale_reloc_target.len()) }.expect("fp_sr");

    assert_eq!(
        fp_a, fp_b,
        "distinct mappings with rewritten OptionalHeader.ImageBase and DIR64 code reloc must share fingerprint"
    );
    assert_eq!(
        fp_a, fp_u,
        "rewritten and unrewritten OptionalHeader.ImageBase must normalize to the same module-relative fingerprint"
    );
    assert_ne!(fp_a, fp_sc, "non-relocated code change must invalidate fingerprint");
    assert_ne!(fp_a, fp_sr, "DIR64 relocated target RVA change must invalidate fingerprint");

    // Record runtime callback at `ba` (with rewritten OptionalHeader.ImageBase) and replay at `bb`.
    let mod_name = "cb_hdr_rewrite_test.dll";
    module_mapped(mod_name, ba, img_a.len());
    assert!(record_runtime_callback(ba as u64 + 0x1340, CallbackKind::Worker));
    flush_runtime_callbacks();

    let replayed_b = unsafe { load_replayed_callback_roots(mod_name, bb, img_b.len()) };
    assert!(
        replayed_b.contains(&CallbackRoot { rva: 0x1340, kind: CallbackKind::Worker }),
        "recorded runtime root must replay at second mapping with rewritten ImageBase: {replayed_b:?}"
    );
    assert!(
        unsafe { load_replayed_callback_roots(mod_name, bsc, img_stale_code.len()) }.is_empty(),
        "stale code bytes must reject replayed roots"
    );
    assert!(
        unsafe { load_replayed_callback_roots(mod_name, bsr, img_stale_reloc_target.len()) }.is_empty(),
        "stale DIR64 relocated target RVA must reject replayed roots"
    );

    if let Ok(mut m) = MAPPED.lock() {
        m.retain(|x| x.name != mod_name);
    }
    clear_runtime_callbacks_for_test();
    match prev_aot_dir {
        Some(v) => std::env::set_var("PORT_AOT_DIR", v),
        None => std::env::remove_var("PORT_AOT_DIR"),
    }
    match prev_aw2_dir {
        Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
        None => std::env::remove_var("AW2_DLL_AOT_DIR"),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn closure_discovers_mid_block_resume_after_non_jit_op() {
    let mut img = vec![0u8; 0x3000];
    let base = img.as_ptr() as usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x0000_4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x46, 2);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 16, 0x1000);
    w64(&mut img, opt + 24, base as u64);
    w32(&mut img, opt + 56, 0x3000);
    w32(&mut img, opt + 108, 16);
    // .pdata @ 0x2000 (2 functions)
    w32(&mut img, opt + 112 + 3 * 8, 0x2000);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 24);
    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut img, sec0 + 8, 0x1000);
    w32(&mut img, sec0 + 12, 0x1000);
    w32(&mut img, sec0 + 16, 0x1000);
    w32(&mut img, sec0 + 20, 0x1000);
    w32(&mut img, sec0 + 36, 0x6000_0020);
    let sec1 = sec0 + 40;
    img[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut img, sec1 + 8, 0x1000);
    w32(&mut img, sec1 + 12, 0x2000);
    w32(&mut img, sec1 + 16, 0x1000);
    w32(&mut img, sec1 + 20, 0x2000);
    w32(&mut img, sec1 + 36, 0x4000_0040);

    // Function 1 @ 0x1000: mov eax, 1 (5B); shld dword ptr [rax], ecx, 4 (4B: 0F A4 08 04, generic); add eax, 2 (3B); ret (1B)
    let f1: [u8; 13] = [
        0xB8, 1, 0, 0, 0, // +0x1000: mov eax, 1
        0x0F, 0xA4, 0x08, 0x04, // +0x1005: shld dword ptr [rax], ecx, 4 (generic interpreter)
        0x83, 0xC0, 0x02, // +0x1009: add eax, 2 (resume IP!)
        0xC3, // +0x100C: ret
    ];
    img[0x1000..0x1000 + f1.len()].copy_from_slice(&f1);

    // Function 2 @ 0x1040: xor ecx, ecx (2B); mov rax, fs:[rax] (4B: 64 48 8B 00, generic FS); add eax, 7 (3B); ret (1B)
    let f2: [u8; 10] = [
        0x31, 0xC9, // +0x1040: xor ecx, ecx
        0x64, 0x48, 0x8B, 0x00, // +0x1042: mov rax, fs:[rax] (generic interpreter)
        0x83, 0xC0, 0x07, // +0x1046: add eax, 7 (resume IP!)
        0xC3, // +0x1049: ret
    ];
    img[0x1040..0x1040 + f2.len()].copy_from_slice(&f2);

    w32(&mut img, 0x2000, 0x1000);
    w32(&mut img, 0x2004, 0x1020);
    w32(&mut img, 0x2008, 0x2100);
    w32(&mut img, 0x200C, 0x1040);
    w32(&mut img, 0x2010, 0x1060);
    w32(&mut img, 0x2014, 0x2100);
    img[0x2100] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, _) = unsafe { discover(&mut *bc, base, img.len(), &[0x1000, 0x1040], DEFAULT_AOT_CLOSURE) };
    assert!(
        ips.contains(&(base as u64 + 0x1009)),
        "expected post-shld-mem resume IP +0x1009 in discovered blocks: {ips:#x?}"
    );
    assert!(
        ips.contains(&(base as u64 + 0x1046)),
        "expected post-fs-mov resume IP +0x1046 in discovered blocks: {ips:#x?}"
    );
}

#[test]
fn closure_discovers_deep_intra_pdata_chain_past_128_hops() {
    let mut img = vec![0u8; 0x5000];
    let base = img.as_ptr() as usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x0000_4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x46, 2);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 16, 0x1000);
    w64(&mut img, opt + 24, base as u64);
    w32(&mut img, opt + 56, 0x5000);
    w32(&mut img, opt + 108, 16);
    // .pdata @ 0x4000 (2 functions: main chain + callee)
    w32(&mut img, opt + 112 + 3 * 8, 0x4000);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 24);
    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut img, sec0 + 8, 0x3000);
    w32(&mut img, sec0 + 12, 0x1000);
    w32(&mut img, sec0 + 16, 0x3000);
    w32(&mut img, sec0 + 20, 0x1000);
    w32(&mut img, sec0 + 36, 0x6000_0020);
    let sec1 = sec0 + 40;
    img[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut img, sec1 + 8, 0x1000);
    w32(&mut img, sec1 + 12, 0x4000);
    w32(&mut img, sec1 + 16, 0x1000);
    w32(&mut img, sec1 + 20, 0x4000);
    w32(&mut img, sec1 + 36, 0x4000_0040);

    // Helper callee @ 0x3800: ret
    img[0x3800] = 0xC3;

    // 150 sequential `call 0x3800` blocks inside one `.pdata` function @ 0x1000..0x2000.
    let mut cur = 0x1000usize;
    for _ in 0..150 {
        let rel = (0x3800i32) - ((cur + 5) as i32);
        img[cur] = 0xE8;
        img[cur + 1..cur + 5].copy_from_slice(&rel.to_le_bytes());
        cur += 5;
    }
    let tail_rva = cur as u32;
    img[cur] = 0xC3; // ret at hop 150

    w32(&mut img, 0x4000, 0x1000);
    w32(&mut img, 0x4004, 0x2000);
    w32(&mut img, 0x4008, 0x4100);
    w32(&mut img, 0x400C, 0x3800);
    w32(&mut img, 0x4010, 0x3810);
    w32(&mut img, 0x4014, 0x4100);
    img[0x4100] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    // Explicit shallow depth (10) still truncates cleanly.
    let (shallow, _) = unsafe { discover(&mut *bc, base, img.len(), &[0x1000], 10) };
    assert!(!shallow.contains(&(base as u64 + tail_rva as u64)));

    // Default closure depth expands the full intra-.pdata chain past 128 hops!
    let (deep, _) = unsafe { discover(&mut *bc, base, img.len(), &[0x1000], DEFAULT_AOT_CLOSURE) };
    assert!(
        deep.contains(&(base as u64 + tail_rva as u64)),
        "expected intra-.pdata block at hop 150 (+{tail_rva:#x}) to be discovered"
    );
}

#[test]
fn closure_discovers_byte_loaded_switch_jump_table() {
    let mut img = vec![0u8; 0x3000];
    let base = img.as_ptr() as usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x0000_4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x46, 2);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 16, 0x1000);
    w64(&mut img, opt + 24, base as u64);
    w32(&mut img, opt + 56, 0x3000);
    w32(&mut img, opt + 108, 16);
    w32(&mut img, opt + 112 + 3 * 8, 0x2000);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 12);
    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut img, sec0 + 8, 0x1000);
    w32(&mut img, sec0 + 12, 0x1000);
    w32(&mut img, sec0 + 16, 0x1000);
    w32(&mut img, sec0 + 20, 0x1000);
    w32(&mut img, sec0 + 36, 0x6000_0020);
    let sec1 = sec0 + 40;
    img[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut img, sec1 + 8, 0x1000);
    w32(&mut img, sec1 + 12, 0x2000);
    w32(&mut img, sec1 + 16, 0x1000);
    w32(&mut img, sec1 + 20, 0x2000);
    w32(&mut img, sec1 + 36, 0x4000_0040);

    // 4-entry RVA32 jump table @ 0x2200 pointing to cases @ 0x1100, 0x1110, 0x1120, 0x1130.
    let cases = [0x1100u32, 0x1110, 0x1120, 0x1130];
    for (i, &c) in cases.iter().enumerate() {
        w32(&mut img, 0x2200 + i * 4, c);
        img[c as usize..c as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }

    // Dispatch @ 0x1000 (NO prior `cmp` bound; relies on `movzx eax, byte ptr [rsi]`):
    //   lea r9, [rip - 0x1007]              ; r9 = image base
    //   movzx eax, byte ptr [rsi]           ; 0F B6 06
    //   mov eax, dword ptr [r9 + rax*4 + 0x2200] ; 41 8B 84 81 00 22 00 00
    //   add rax, r9                         ; 4C 01 C8
    //   jmp rax                             ; FF E0
    let mut code = Vec::new();
    let lea_disp: i32 = -0x1007;
    code.extend_from_slice(&[0x4C, 0x8D, 0x0D]);
    code.extend_from_slice(&lea_disp.to_le_bytes());
    code.extend_from_slice(&[0x0F, 0xB6, 0x06]);
    code.extend_from_slice(&[0x41, 0x8B, 0x84, 0x81, 0x00, 0x22, 0x00, 0x00]);
    code.extend_from_slice(&[0x4C, 0x01, 0xC8]);
    code.extend_from_slice(&[0xFF, 0xE0]);
    img[0x1000..0x1000 + code.len()].copy_from_slice(&code);

    // .pdata: 0x1000..0x1200
    w32(&mut img, 0x2000, 0x1000);
    w32(&mut img, 0x2004, 0x1200);
    w32(&mut img, 0x2008, 0x2100);
    img[0x2100] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, cl) = unsafe { discover(&mut *bc, base, img.len(), &[0x1000], DEFAULT_AOT_CLOSURE) };
    assert_eq!(cl.jt_tables, 1, "expected byte-loaded jump table resolved");
    for c in cases {
        assert!(
            ips.contains(&(base as u64 + c as u64)),
            "missing byte-loaded switch case +{c:#x} in {ips:#x?}"
        );
    }
}

#[test]
fn runtime_callback_replay_triggers_bg_rebuild() {
    use super::callbacks::clear_runtime_callbacks_for_test;
    let dir = std::env::temp_dir().join(format!("port_aot_cb_bg_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let prev_aot_dir = std::env::var_os("PORT_AOT_DIR");
    let prev_aw2_dir = std::env::var_os("AW2_DLL_AOT_DIR");
    std::env::set_var("PORT_AOT_DIR", &dir);
    std::env::remove_var("AW2_DLL_AOT_DIR");
    clear_runtime_callbacks_for_test();

    let mut img = vec![0u8; 0x3000];
    let base = img.as_ptr() as usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());
    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x0000_4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x46, 1);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 16, 0x1000);
    w64(&mut img, opt + 24, base as u64);
    w32(&mut img, opt + 56, 0x3000);
    w32(&mut img, opt + 108, 16);
    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut img, sec0 + 8, 0x1000);
    w32(&mut img, sec0 + 12, 0x1000);
    w32(&mut img, sec0 + 16, 0x1000);
    w32(&mut img, sec0 + 20, 0x1000);
    w32(&mut img, sec0 + 36, 0x6000_0020);

    // Entry @ 0x1000 and opaque indirect target @ 0x1200 (not referenced statically).
    img[0x1000..0x1006].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);
    img[0x1200..0x1206].copy_from_slice(&[0xB8, 2, 0, 0, 0, 0xC3]);

    let mod_name = "cb_bg_rebuild_test.dll";
    module_mapped(mod_name, base, img.len());
    let key = unsafe { module_key(base, img.len()) };

    // Before recording any runtime callback or profile, bg_job has nothing to build.
    assert!(bg_job(mod_name, base, img.len(), key, Some(&[base as u64 + 0x1000])).is_none());

    // Record runtime callback @ +0x1200 and flush; bg_job must now schedule a rebuild because
    // the loaded cache only has +0x1000 (100% of replayed roots missing).
    assert!(record_runtime_callback(base as u64 + 0x1200, CallbackKind::GenericCallback));
    flush_runtime_callbacks();
    let job = bg_job(mod_name, base, img.len(), key, Some(&[base as u64 + 0x1000]));
    assert!(job.is_some(), "expected bg_job to schedule rebuild for missing replayed callback");

    if let Ok(mut m) = MAPPED.lock() {
        m.retain(|x| x.name != mod_name);
    }
    clear_runtime_callbacks_for_test();
    match prev_aot_dir {
        Some(v) => std::env::set_var("PORT_AOT_DIR", v),
        None => std::env::remove_var("PORT_AOT_DIR"),
    }
    match prev_aw2_dir {
        Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
        None => std::env::remove_var("AW2_DLL_AOT_DIR"),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A large `.pdata` function whose intra-function CFG extends past `DEFAULT_AOT_CLOSURE`
/// (128 hops) must still admit direct calls and tail-jumps to `.pdata`-less leaf helpers.
#[test]
fn aot_deep_pdata_function_discovers_leaf_callees_past_default_depth() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3F10_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());

    // .text @ 0x1000..0x5000, .rdata @ 0x5000..0x7000, .pdata @ 0x6000 (1 entry = 12 bytes)
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x4000, 0x5000, 0x2000, None, Some((0x6000, 12)));

    // 140-hop chain (> DEFAULT_AOT_CLOSURE = 128) ending in call 0x4000; jmp 0x4100.
    let mut a = CodeAssembler::new(64).unwrap();
    let mut exit_lbl = a.create_label();
    for i in 0..140 {
        a.cmp(ecx, i as i32).unwrap();
        a.je(exit_lbl).unwrap();
    }
    a.call(base64 + 0x4000).unwrap();
    a.jmp(base64 + 0x4100).unwrap();
    a.set_label(&mut exit_lbl).unwrap();
    a.xor(eax, eax).unwrap();
    a.ret().unwrap();
    let fn0 = a.assemble(base64 + 0x1000).unwrap();
    assert!(0x1000 + fn0.len() < 0x3F00);
    buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

    // .pdata entry covering only [0x1000, 0x1000 + fn0.len())
    w32(buf, 0x6000, 0x1000);
    w32(buf, 0x6004, 0x1000 + fn0.len() as u32);
    w32(buf, 0x6008, 0);

    // Two `.pdata`-less leaf helpers at 0x4000 (direct call) and 0x4100 (tail jump).
    buf[0x4000..0x4006].copy_from_slice(&[0xB8, 0x11, 0, 0, 0, 0xC3]);
    buf[0x4100..0x4106].copy_from_slice(&[0xB8, 0x22, 0, 0, 0, 0xC3]);

    let mut bc = Box::new(BlockCache::new());
    let (ips, closure) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], DEFAULT_AOT_CLOSURE) };
    assert!(
        ips.contains(&(base64 + 0x4000)),
        "deep .pdata function (>128 hops) must discover direct-called .pdata-less leaf"
    );
    assert!(
        ips.contains(&(base64 + 0x4100)),
        "deep .pdata function (>128 hops) must discover tail-jumped .pdata-less leaf"
    );
    assert!(
        closure.leaf_callees >= 2,
        "expected >=2 leaf callees, got {}",
        closure.leaf_callees
    );
}

/// Far-branch islands (`ldr x16, .+8; br x16; .quad dst`) must chain both direct `b` tail jumps
/// and `bl` calls (preserving `x30` / RAS return landing pads), deduplicate repeated targets,
/// and match interpreter execution cleanly.
#[test]
fn aot_far_branch_island_chains_b_and_bl_preserving_ras() {
    use iced_x86::code_asm::*;
    use std::sync::atomic::Ordering;

    struct ResetFar;
    impl Drop for ResetFar {
        fn drop(&mut self) {
            super::runtime::TEST_FORCE_FAR_LINK.store(false, Ordering::Relaxed);
            super::runtime::test_uninstall();
        }
    }

    let mut code = TestMappedBuf::alloc(0x4000, 0x2_3F20_0000);
    let base = code.base() as u64;
    let buf = code.as_slice_mut();

    let fn_entry = base + 0x100;
    let fn_callee = base + 0x200;
    let fn_tail = base + 0x300;
    let fn_mid = base + 0x180;

    // entry: xor eax, eax; call callee; add eax, 100; jmp tail (and a second call to callee to test dedup)
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, ecx).unwrap();
    a.call(fn_callee).unwrap();
    a.call(fn_callee).unwrap();
    a.jmp(fn_mid).unwrap();
    let b_entry = a.assemble(fn_entry).unwrap();
    buf[0x100..0x100 + b_entry.len()].copy_from_slice(&b_entry);

    let mut a = CodeAssembler::new(64).unwrap();
    a.add(eax, 100).unwrap();
    a.jmp(fn_tail).unwrap();
    let b_mid = a.assemble(fn_mid).unwrap();
    buf[0x180..0x180 + b_mid.len()].copy_from_slice(&b_mid);

    let mut a = CodeAssembler::new(64).unwrap();
    a.add(eax, 7).unwrap();
    a.ret().unwrap();
    let b_callee = a.assemble(fn_callee).unwrap();
    buf[0x200..0x200 + b_callee.len()].copy_from_slice(&b_callee);

    let mut a = CodeAssembler::new(64).unwrap();
    a.imul_2(eax, eax).unwrap();
    a.ret().unwrap();
    let b_tail = a.assemble(fn_tail).unwrap();
    buf[0x300..0x300 + b_tail.len()].copy_from_slice(&b_tail);

    let call1 = fn_entry + 7; // after first `call fn_callee`
    let call2 = fn_entry + 12; // after second `call fn_callee`
    let ips = [fn_entry, call1, call2, fn_mid, fn_callee, fn_tail];

    let _reset = ResetFar;
    super::runtime::TEST_FORCE_FAR_LINK.store(true, Ordering::Relaxed);
    let (live, _, links) = unsafe { super::runtime::test_install(&ips, || {}) };
    assert!(live >= 4 && links >= 3, "expected live blocks and links, got live={live} links={links}");

    // Run through the installed AOT far-branch islands and compare with exact arithmetic:
    // ((5 + 7 + 7) + 100)^2 = 119^2 = 14161.
    crate::emu::jit::force(1);
    let got = unsafe { crate::emu::call_va8(fn_entry, 5, 0, 0, 0, 0, 0, 0, 0) };
    crate::emu::jit::force(0);
    assert_eq!(got, 119 * 119);
}
