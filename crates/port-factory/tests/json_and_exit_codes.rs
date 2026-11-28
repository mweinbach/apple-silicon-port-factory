//! Integration tests verifying `--json` Schema v1 envelopes and canonical exit codes
//! (`0`, `2`, `3`, `4`, `5`) across `port-factory` audit & verification commands.

use port_factory::json_output::{exit_codes, parse_json, JsonValue, SCHEMA_VERSION};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!(
        "pf_json_test_{tag}_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn put_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}
fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
fn put_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

/// Build a minimal valid x86-64 PE32+ executable importing `(dll, &[fn_names])` pairs.
fn build_minimal_pe(imports: &[(&str, &[&str])]) -> Vec<u8> {
    // Layout:
    //   0x000..0x200 : DOS + PE headers + 2 section headers (.text, .rdata)
    //   0x200..0x400 : .text  (RVA 0x1000) -> `xor eax, eax; ret`
    //   0x400..0xC00 : .rdata (RVA 0x2000) -> Import descriptors, thunks, strings
    let mut img = vec![0u8; 0xC00];

    img[0..2].copy_from_slice(b"MZ");
    let pe_off = 0x80usize;
    put_u32(&mut img, 0x3C, pe_off as u32);

    img[pe_off..pe_off + 4].copy_from_slice(b"PE\0\0");
    let coff = pe_off + 4;
    put_u16(&mut img, coff, 0x8664); // AMD64
    put_u16(&mut img, coff + 2, 2); // 2 sections
    put_u16(&mut img, coff + 16, 0xF0); // SizeOfOptionalHeader
    put_u16(&mut img, coff + 18, 0x0022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

    let opt = coff + 20;
    put_u16(&mut img, opt, 0x020B); // PE32+
    put_u32(&mut img, opt + 16, 0x1000); // EntryPoint
    put_u32(&mut img, opt + 20, 0x1000); // BaseOfCode
    put_u64(&mut img, opt + 24, 0x0000_0001_4000_0000); // ImageBase
    put_u32(&mut img, opt + 32, 0x1000); // SectionAlignment
    put_u32(&mut img, opt + 36, 0x0200); // FileAlignment
    put_u16(&mut img, opt + 40, 6);
    put_u16(&mut img, opt + 48, 6);
    put_u32(&mut img, opt + 56, 0x3000); // SizeOfImage
    put_u32(&mut img, opt + 60, 0x0200); // SizeOfHeaders
    put_u16(&mut img, opt + 68, 3); // CUI
    put_u16(&mut img, opt + 70, 0x8160);
    put_u64(&mut img, opt + 72, 0x10_0000);
    put_u64(&mut img, opt + 80, 0x1000);
    put_u64(&mut img, opt + 88, 0x10_0000);
    put_u64(&mut img, opt + 96, 0x1000);
    put_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    let desc_size = ((imports.len() + 1) * 20) as u32;
    if !imports.is_empty() {
        put_u32(&mut img, opt + 112 + 8, 0x2000);
        put_u32(&mut img, opt + 112 + 12, desc_size);
    }

    // Section headers at opt + 0xF0
    let sec = opt + 0xF0;
    // .text
    img[sec..sec + 5].copy_from_slice(b".text");
    put_u32(&mut img, sec + 8, 0x200);
    put_u32(&mut img, sec + 12, 0x1000);
    put_u32(&mut img, sec + 16, 0x200);
    put_u32(&mut img, sec + 20, 0x200);
    put_u32(&mut img, sec + 36, 0x6000_0020); // CODE | EXEC | READ

    // .rdata
    let sec2 = sec + 40;
    img[sec2..sec2 + 6].copy_from_slice(b".rdata");
    put_u32(&mut img, sec2 + 8, 0x800);
    put_u32(&mut img, sec2 + 12, 0x2000);
    put_u32(&mut img, sec2 + 16, 0x800);
    put_u32(&mut img, sec2 + 20, 0x400);
    put_u32(&mut img, sec2 + 36, 0x4000_0040); // INITIALIZED_DATA | READ

    // .text code: xor eax, eax; ret
    img[0x200..0x203].copy_from_slice(&[0x31, 0xC0, 0xC3]);

    // Populate import directory at file offset 0x400 (RVA 0x2000)
    let rdata_off = 0x400usize;
    let rdata_rva = 0x2000u32;
    let mut cursor = rdata_off + (imports.len() + 1) * 20;
    // Align cursor to 8
    cursor = (cursor + 7) & !7;

    for (idx, (dll, syms)) in imports.iter().enumerate() {
        let ilt_off = cursor;
        let ilt_rva = rdata_rva + (ilt_off - rdata_off) as u32;
        cursor += (syms.len() + 1) * 8;

        let iat_off = cursor;
        let iat_rva = rdata_rva + (iat_off - rdata_off) as u32;
        cursor += (syms.len() + 1) * 8;

        let dll_off = cursor;
        let dll_rva = rdata_rva + (dll_off - rdata_off) as u32;
        img[dll_off..dll_off + dll.len()].copy_from_slice(dll.as_bytes());
        cursor += dll.len() + 1;
        cursor = (cursor + 1) & !1;

        for (si, sym) in syms.iter().enumerate() {
            let hn_off = cursor;
            let hn_rva = rdata_rva + (hn_off - rdata_off) as u32;
            put_u16(&mut img, hn_off, 0);
            img[hn_off + 2..hn_off + 2 + sym.len()].copy_from_slice(sym.as_bytes());
            cursor += 2 + sym.len() + 1;
            cursor = (cursor + 1) & !1;

            put_u64(&mut img, ilt_off + si * 8, hn_rva as u64);
            put_u64(&mut img, iat_off + si * 8, hn_rva as u64);
        }
        cursor = (cursor + 7) & !7;

        let d_off = rdata_off + idx * 20;
        put_u32(&mut img, d_off, ilt_rva); // OriginalFirstThunk
        put_u32(&mut img, d_off + 12, dll_rva); // Name
        put_u32(&mut img, d_off + 16, iat_rva); // FirstThunk
    }

    img
}

fn run_pf(args: &[&str]) -> (i32, JsonValue) {
    run_pf_with_env(args, &[])
}

fn run_pf_with_env(args: &[&str], extra_env: &[(&str, &str)]) -> (i32, JsonValue) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-factory"));
    cmd.args(args);
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("spawn port-factory");
    let code = out.status.code().expect("exit code");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let parsed = parse_json(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "port-factory {:?} (exit {code}) produced invalid JSON: {e}\nstdout:\n{stdout}\nstderr:\n{stderr}",
            args
        )
    });
    assert_eq!(
        parsed.get("schema_version").and_then(JsonValue::as_u64),
        Some(SCHEMA_VERSION as u64),
        "missing/wrong schema_version for {args:?}: {stdout}"
    );
    assert_eq!(
        parsed.get("exit_code").and_then(JsonValue::as_i64),
        Some(code as i64),
        "envelope exit_code mismatch for {args:?}: {stdout}"
    );
    assert_eq!(
        parsed.get("ok").and_then(JsonValue::as_bool),
        Some(code == exit_codes::OK),
        "envelope ok flag mismatch for {args:?}: {stdout}"
    );
    assert!(
        parsed.get("data").is_some() && parsed.get("error").is_some(),
        "envelope missing data/error fields for {args:?}: {stdout}"
    );
    (code, parsed)
}

#[test]
fn audit_and_verify_commands_emit_valid_json_and_canonical_exit_codes() {
    let dir = unique_temp_dir("suite");
    let clean_pe = dir.join("clean.exe");
    let unresolved_pe = dir.join("unresolved.exe");
    let kernel_pe = dir.join("kernel_ac.exe");
    let bad_app = dir.join("Broken.app");
    let bad_ios_app = dir.join("BrokenIos.app");

    std::fs::write(&clean_pe, build_minimal_pe(&[("kernel32.dll", &["ExitProcess"])])).unwrap();
    std::fs::write(
        &unresolved_pe,
        build_minimal_pe(&[
            ("kernel32.dll", &["ExitProcess"]),
            ("nonexistent_test_lib.dll", &["MissingSymbolAlpha"]),
        ]),
    )
    .unwrap();
    std::fs::write(
        &kernel_pe,
        build_minimal_pe(&[("advapi32.dll", &["CreateServiceW"])]),
    )
    .unwrap();
    std::fs::create_dir_all(&bad_app).unwrap();
    std::fs::create_dir_all(&bad_ios_app).unwrap();

    let clean_s = clean_pe.to_str().unwrap();
    let unresolved_s = unresolved_pe.to_str().unwrap();
    let kernel_s = kernel_pe.to_str().unwrap();
    let bad_app_s = bad_app.to_str().unwrap();
    let bad_ios_s = bad_ios_app.to_str().unwrap();

    // 1. scan --json -> exit 0
    let (code, val) = run_pf(&["scan", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(val.get("command").and_then(JsonValue::as_str), Some("scan"));
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("binaries_scanned")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 2. win32-audit --json (clean -> exit 0; unresolved -> exit 3)
    let (code, val) = run_pf(&["win32-audit", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("win32-audit")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("unresolved_imports")
            .and_then(JsonValue::as_u64),
        Some(0)
    );

    let (code, val) = run_pf(&["win32-audit", unresolved_s, "--json"]);
    assert_eq!(code, exit_codes::UNRESOLVED_IMPORTS);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("unresolved_imports")
            .and_then(JsonValue::as_u64),
        Some(1)
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    // 3. anticheat-audit --json (clean -> exit 0; Ring-0 kernel driver -> exit 5)
    let (code, val) = run_pf(&["anticheat-audit", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("anticheat-audit")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("kernel_driver_blockers")
            .and_then(JsonValue::as_u64),
        Some(0)
    );

    let (code, val) = run_pf(&["anticheat-audit", kernel_s, "--json"]);
    assert_eq!(code, exit_codes::INCOMPATIBLE);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("kernel_driver_blockers")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 4. runtime-audit --json -> exit 0
    let (code, val) = run_pf(&["runtime-audit", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("runtime-audit")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("binaries_audited")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 5. app-audit --json -> exit 0
    let (code, val) = run_pf(&["app-audit", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("app-audit")
    );

    // 6. dotnet-audit --json -> exit 0
    let (code, val) = run_pf(&["dotnet-audit", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("dotnet-audit")
    );

    // 7. shader-inspect --json -> exit 0
    let (code, val) = run_pf(&["shader-inspect", clean_s, "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("shader-inspect")
    );

    // 8. stats / win32-stats / shader-formats / game-profiles / vfs-resolve / bundles --json -> exit 0
    for (sub, extra) in [
        ("stats", vec!["--json"]),
        ("win32-stats", vec!["--json"]),
        ("shader-formats", vec!["--json"]),
        ("game-profiles", vec!["--json"]),
        ("vfs-resolve", vec!["C:\\Games\\Test", "--json"]),
        ("bundles", vec!["--json"]),
    ] {
        let mut argv = vec![sub];
        argv.extend(extra);
        let (code, val) = run_pf(&argv);
        assert_eq!(code, exit_codes::OK, "{sub} failed");
        assert_eq!(val.get("command").and_then(JsonValue::as_str), Some(sub));
    }

    // 9. bundle-verify --json on broken .app -> exit 4 (VERIFICATION_FAILED)
    let (code, val) = run_pf(&["bundle-verify", bad_app_s, "--json"]);
    assert_eq!(code, exit_codes::VERIFICATION_FAILED);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("bundle-verify")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("failed_bundles")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 10. ios-verify --json on broken iOS .app -> exit 4 (VERIFICATION_FAILED)
    let (code, val) = run_pf(&["ios-verify", bad_ios_s, "--json"]);
    assert_eq!(code, exit_codes::VERIFICATION_FAILED);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("ios-verify")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("passed")
            .and_then(JsonValue::as_bool),
        Some(false)
    );

    // 11. doctor --json on real workspace -> exit 0
    let (code, val) = run_pf(&["doctor", "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("doctor")
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("db_clean")
            .and_then(JsonValue::as_bool),
        Some(true)
    );

    // 12. doctor --json with drifted db -> exit 4 (VERIFICATION_FAILED)
    let fake_ws = unique_temp_dir("drifted_ws");
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    std::os::unix::fs::symlink(repo_root.join("crates"), fake_ws.join("crates")).unwrap();
    std::os::unix::fs::symlink(repo_root.join("docs"), fake_ws.join("docs")).unwrap();
    std::fs::create_dir_all(fake_ws.join("db")).unwrap();
    for tsv in [
        "ops_x86_to_arm64.tsv",
        "win32_apis.tsv",
        "shader_formats.tsv",
        "game_profiles.tsv",
        "stub_inventory.tsv",
        "isa_coverage.tsv",
        "a64_emitter_matches.tsv",
        "a64_encodings.tsv",
    ] {
        std::fs::copy(repo_root.join("db").join(tsv), fake_ws.join("db").join(tsv)).unwrap();
    }
    // Introduce drift into ops_x86_to_arm64.tsv
    let mut drifted = std::fs::read_to_string(fake_ws.join("db/ops_x86_to_arm64.tsv")).unwrap();
    drifted.push_str("# intentional drift line\n");
    std::fs::write(fake_ws.join("db/ops_x86_to_arm64.tsv"), drifted).unwrap();

    let (code, val) = run_pf_with_env(
        &["doctor", "--json"],
        &[("PORT_FACTORY_WORKSPACE", fake_ws.to_str().unwrap())],
    );
    assert_eq!(code, exit_codes::VERIFICATION_FAILED);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("db_clean")
            .and_then(JsonValue::as_bool),
        Some(false)
    );
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("drifted_count")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 13. Usage errors (exit 2) emit valid JSON error envelopes
    let (code, val) = run_pf(&["scan", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert!(val.get("data").unwrap().is_null());
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&["doctor", "--bogus-flag", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&["not-a-real-cmd", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    // 14. verify-x86 --json (when x86-verify binary is already built in target/)
    let x86_verify_candidates = [
        repo_root.join("target/aarch64-apple-darwin/release/x86-verify"),
        repo_root.join("target/release/x86-verify"),
        repo_root.join("target/aarch64-apple-darwin/debug/x86-verify"),
        repo_root.join("target/debug/x86-verify"),
    ];
    if x86_verify_candidates.iter().any(|p| p.is_file()) {
        let (code, val) = run_pf(&[
            "verify-x86",
            clean_s,
            "--max-blocks",
            "4",
            "--trials",
            "1",
            "--json",
        ]);
        assert_eq!(code, exit_codes::OK);
        assert_eq!(
            val.get("command").and_then(JsonValue::as_str),
            Some("verify-x86")
        );
        assert_eq!(
            val.get("data")
                .unwrap()
                .get("critical_mismatches")
                .and_then(JsonValue::as_u64),
            Some(0)
        );

        let (code, val) = run_pf(&["verify-x86", "--unknown-flag", "--json"]);
        assert_eq!(code, exit_codes::USAGE_ERROR);
        assert_eq!(
            val.get("command").and_then(JsonValue::as_str),
            Some("verify-x86")
        );
        assert!(val.get("error").and_then(JsonValue::as_str).is_some());
    }

    // 15. Python diagnostic wrappers emit valid Schema v1 JSON error envelopes on failure
    let (code, val) = run_pf(&["oracle", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(val.get("command").and_then(JsonValue::as_str), Some("oracle"));
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&["oracle", "/nonexistent/port_factory_snaps_dir", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(val.get("command").and_then(JsonValue::as_str), Some("oracle"));
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&["live-probe", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("live-probe")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&[
        "live-probe",
        "--pe",
        clean_s,
        "--runner",
        "/nonexistent/port-runner-bin",
        "--json",
    ]);
    assert_eq!(code, exit_codes::RUNTIME_ERROR);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("live-probe")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&["bisect-runtime", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("bisect-runtime")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let (code, val) = run_pf(&[
        "bisect-runtime",
        "--cmd",
        "true",
        "--extra-env",
        "BAD_NO_EQUALS",
        "--json",
    ]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("bisect-runtime")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&fake_ws);
}
