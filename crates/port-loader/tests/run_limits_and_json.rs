//! End-to-end tests for `port-runner`:
//! - `--no-run` exit code (`0` when all imports resolve, `3` when unresolved imports exist)
//! - `--json` Schema v1 envelopes across `--no-run`, `--ops-report`, `--list-exports`,
//!   live runs, usage errors (`2`), runtime errors (`1`), and timeouts (`124`)
//! - `--timeout-secs` wall-clock termination with exit code `124`
//! - `--max-frames` flag validation and live run summary

#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use port_factory::json_output::{exit_codes, parse_json, JsonValue, SCHEMA_VERSION};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!(
        "pr_limits_test_{tag}_{}_{}",
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

fn emit_call_rip(code: &mut Vec<u8>, text_rva: u32, target_rva: u32) {
    code.extend_from_slice(&[0xFF, 0x15]);
    let next_rva = text_rva + code.len() as u32 + 4;
    let disp = (target_rva as i64 - next_rva as i64) as i32;
    code.extend_from_slice(&disp.to_le_bytes());
}

#[derive(Clone, Copy)]
enum TestPeKind {
    CleanExitZero,
    UnresolvedImport,
    SleepLong,
}

/// Synthesize a minimal valid PE32+ x86-64 executable.
fn build_test_pe(kind: TestPeKind) -> Vec<u8> {
    let mut img = vec![0u8; 0xC00];

    img[0..2].copy_from_slice(b"MZ");
    let pe_off = 0x80usize;
    put_u32(&mut img, 0x3C, pe_off as u32);

    img[pe_off..pe_off + 4].copy_from_slice(b"PE\0\0");
    let coff = pe_off + 4;
    put_u16(&mut img, coff, 0x8664); // AMD64
    put_u16(&mut img, coff + 2, 2); // .text, .rdata
    put_u16(&mut img, coff + 16, 0xF0);
    put_u16(&mut img, coff + 18, 0x0022);

    let opt = coff + 20;
    put_u16(&mut img, opt, 0x020B);
    put_u32(&mut img, opt + 16, 0x1000); // EntryPoint
    put_u32(&mut img, opt + 20, 0x1000); // BaseOfCode
    put_u64(&mut img, opt + 24, 0x0000_0001_4000_0000);
    put_u32(&mut img, opt + 32, 0x1000);
    put_u32(&mut img, opt + 36, 0x0200);
    put_u16(&mut img, opt + 40, 6);
    put_u16(&mut img, opt + 48, 6);
    put_u32(&mut img, opt + 56, 0x3000);
    put_u32(&mut img, opt + 60, 0x0200);
    put_u16(&mut img, opt + 68, 3); // CUI
    put_u16(&mut img, opt + 70, 0x8160);
    put_u64(&mut img, opt + 72, 0x10_0000);
    put_u64(&mut img, opt + 80, 0x1000);
    put_u64(&mut img, opt + 88, 0x10_0000);
    put_u64(&mut img, opt + 96, 0x1000);
    put_u32(&mut img, opt + 108, 16);

    let imports: Vec<(&str, Vec<&str>)> = match kind {
        TestPeKind::CleanExitZero => vec![("kernel32.dll", vec!["ExitProcess"])],
        TestPeKind::UnresolvedImport => vec![
            ("kernel32.dll", vec!["ExitProcess"]),
            ("kernel32.dll", vec!["NonExistentWin32ExportForTest"]),
        ],
        TestPeKind::SleepLong => vec![("kernel32.dll", vec!["Sleep", "ExitProcess"])],
    };

    let desc_size = ((imports.len() + 1) * 20) as u32;
    put_u32(&mut img, opt + 112 + 8, 0x2000);
    put_u32(&mut img, opt + 112 + 12, desc_size);

    let sec = opt + 0xF0;
    img[sec..sec + 5].copy_from_slice(b".text");
    put_u32(&mut img, sec + 8, 0x200);
    put_u32(&mut img, sec + 12, 0x1000);
    put_u32(&mut img, sec + 16, 0x200);
    put_u32(&mut img, sec + 20, 0x200);
    put_u32(&mut img, sec + 36, 0x6000_0020);

    let sec2 = sec + 40;
    img[sec2..sec2 + 6].copy_from_slice(b".rdata");
    put_u32(&mut img, sec2 + 8, 0x800);
    put_u32(&mut img, sec2 + 12, 0x2000);
    put_u32(&mut img, sec2 + 16, 0x800);
    put_u32(&mut img, sec2 + 20, 0x400);
    put_u32(&mut img, sec2 + 36, 0xC000_0040); // READ | WRITE for IAT patching

    // Populate import directory at file offset 0x400 (RVA 0x2000)
    let rdata_off = 0x400usize;
    let rdata_rva = 0x2000u32;
    let mut cursor = (rdata_off + (imports.len() + 1) * 20 + 7) & !7;
    let mut iat_rvas: Vec<u32> = Vec::new();

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
            iat_rvas.push(iat_rva + (si * 8) as u32);
        }
        cursor = (cursor + 7) & !7;

        let d_off = rdata_off + idx * 20;
        put_u32(&mut img, d_off, ilt_rva);
        put_u32(&mut img, d_off + 12, dll_rva);
        put_u32(&mut img, d_off + 16, iat_rva);
    }

    // Build .text machine code
    let mut code = Vec::new();
    // sub rsp, 0x28
    code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);
    match kind {
        TestPeKind::CleanExitZero | TestPeKind::UnresolvedImport => {
            // xor ecx, ecx
            code.extend_from_slice(&[0x31, 0xC9]);
            // call [rip + ExitProcess]
            emit_call_rip(&mut code, 0x1000, iat_rvas[0]);
            code.push(0xC3);
        }
        TestPeKind::SleepLong => {
            // mov ecx, 60000
            code.push(0xB9);
            code.extend_from_slice(&60_000u32.to_le_bytes());
            // call [rip + Sleep]
            emit_call_rip(&mut code, 0x1000, iat_rvas[0]);
            // xor ecx, ecx
            code.extend_from_slice(&[0x31, 0xC9]);
            // call [rip + ExitProcess]
            emit_call_rip(&mut code, 0x1000, iat_rvas[1]);
            code.push(0xC3);
        }
    }
    img[0x200..0x200 + code.len()].copy_from_slice(&code);
    img
}

fn run_runner(drive_c: &PathBuf, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_port-runner"))
        .args(args)
        .env("TRANSLATOR_HOME", drive_c)
        .env("PORT_HEADLESS", "1")
        .env("PORT_AOT", "0")
        .env("PORT_NO_BASE_RETRY", "1")
        .output()
        .expect("spawn port-runner");
    (
        out.status.code().expect("exit code"),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn parse_and_check_envelope(code: i32, stdout: &str, stderr: &str, args: &[&str]) -> JsonValue {
    let val = parse_json(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "port-runner {args:?} (exit {code}) emitted invalid JSON: {e}\nstdout:\n{stdout}\nstderr:\n{stderr}"
        )
    });
    assert_eq!(
        val.get("schema_version").and_then(JsonValue::as_u64),
        Some(SCHEMA_VERSION as u64)
    );
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("port-runner")
    );
    assert_eq!(
        val.get("exit_code").and_then(JsonValue::as_i64),
        Some(code as i64)
    );
    assert_eq!(
        val.get("ok").and_then(JsonValue::as_bool),
        Some(code == exit_codes::OK)
    );
    val
}

#[test]
fn port_runner_no_run_json_and_exit_codes() {
    let dir = unique_temp_dir("no_run");
    let drive_c = dir.join("drive_c");
    std::fs::create_dir_all(&drive_c).unwrap();

    let clean_pe = dir.join("clean.exe");
    let unresolved_pe = dir.join("unresolved.exe");
    std::fs::write(&clean_pe, build_test_pe(TestPeKind::CleanExitZero)).unwrap();
    std::fs::write(&unresolved_pe, build_test_pe(TestPeKind::UnresolvedImport)).unwrap();

    let clean_s = clean_pe.to_str().unwrap();
    let unresolved_s = unresolved_pe.to_str().unwrap();

    // 1. --no-run with 0 unresolved imports -> exit 0 (text & JSON + --ops-report)
    let (code, _out, err) = run_runner(&drive_c, &["--pe", clean_s, "--no-run"]);
    assert_eq!(code, exit_codes::OK, "clean --no-run stderr:\n{err}");

    let args = ["--pe", clean_s, "--no-run", "--ops-report", "--json"];
    let (code, out, err) = run_runner(&drive_c, &args);
    assert_eq!(code, exit_codes::OK, "clean --no-run --json stderr:\n{err}");
    let val = parse_and_check_envelope(code, &out, &err, &args);
    let data = val.get("data").unwrap();
    assert_eq!(
        data.get("unresolved_count").and_then(JsonValue::as_u64),
        Some(0)
    );
    assert!(data.get("ops_report").unwrap().as_object().is_some());

    // 2. --no-run with unresolved imports -> exit 3 (text & JSON)
    let (code, _out, err) = run_runner(&drive_c, &["--pe", unresolved_s, "--no-run"]);
    assert_eq!(
        code,
        exit_codes::UNRESOLVED_IMPORTS,
        "unresolved --no-run must exit 3, stderr:\n{err}"
    );

    let args = ["--pe", unresolved_s, "--no-run", "--json"];
    let (code, out, err) = run_runner(&drive_c, &args);
    assert_eq!(code, exit_codes::UNRESOLVED_IMPORTS);
    let val = parse_and_check_envelope(code, &out, &err, &args);
    let data = val.get("data").unwrap();
    assert_eq!(
        data.get("unresolved_count").and_then(JsonValue::as_u64),
        Some(1)
    );
    let k32_missing = data
        .get("unresolved_by_dll")
        .unwrap()
        .get("kernel32.dll")
        .and_then(JsonValue::as_array)
        .unwrap();
    assert_eq!(k32_missing.len(), 1);
    assert_eq!(
        k32_missing[0].as_str(),
        Some("NonExistentWin32ExportForTest")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    // 3. --list-exports --json -> exit 0
    let args = ["--pe", clean_s, "--list-exports", "--json"];
    let (code, out, err) = run_runner(&drive_c, &args);
    assert_eq!(code, exit_codes::OK);
    let val = parse_and_check_envelope(code, &out, &err, &args);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("exports_count")
            .and_then(JsonValue::as_u64),
        Some(0)
    );

    // 4. Runtime error (missing PE) with --json -> exit 1
    let missing_s = dir.join("missing.exe");
    let args = ["--pe", missing_s.to_str().unwrap(), "--no-run", "--json"];
    let (code, out, err) = run_runner(&drive_c, &args);
    assert_eq!(code, exit_codes::RUNTIME_ERROR);
    let val = parse_and_check_envelope(code, &out, &err, &args);
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    // 5. Usage errors with --json -> exit 2
    for bad_args in [
        vec!["--max-frames", "0", "--json"],
        vec!["--timeout-secs", "0", "--json"],
        vec!["--timeout-secs", "-2", "--json"],
        vec!["--unknown-flag", "--json"],
    ] {
        let (code, out, err) = run_runner(&drive_c, &bad_args);
        assert_eq!(code, exit_codes::USAGE_ERROR, "args={bad_args:?}");
        let val = parse_and_check_envelope(code, &out, &err, &bad_args);
        assert!(val.get("error").and_then(JsonValue::as_str).is_some());
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn port_runner_timeout_secs_and_live_json_run() {
    let dir = unique_temp_dir("limits");
    let drive_c = dir.join("drive_c");
    std::fs::create_dir_all(&drive_c).unwrap();

    let clean_pe = dir.join("clean_exit.exe");
    let sleep_pe = dir.join("sleep_long.exe");
    std::fs::write(&clean_pe, build_test_pe(TestPeKind::CleanExitZero)).unwrap();
    std::fs::write(&sleep_pe, build_test_pe(TestPeKind::SleepLong)).unwrap();

    // 1. Clean guest exit with --max-frames and --timeout-secs -> exit 0, stop_reason == "guest_exit"
    let args = [
        "--pe",
        clean_pe.to_str().unwrap(),
        "--max-frames",
        "10",
        "--timeout-secs",
        "15",
        "--json",
    ];
    let (code, out, err) = run_runner(&drive_c, &args);
    assert_eq!(code, exit_codes::OK, "stderr:\n{err}");
    let val = parse_and_check_envelope(code, &out, &err, &args);
    let data = val.get("data").unwrap();
    assert_eq!(
        data.get("stop_reason").and_then(JsonValue::as_str),
        Some("guest_exit")
    );
    assert_eq!(
        data.get("guest_exit_code").and_then(JsonValue::as_u64),
        Some(0)
    );
    assert_eq!(data.get("max_frames").and_then(JsonValue::as_u64), Some(10));

    // 2. Sleeping guest bounded by --timeout-secs 0.35 -> exit 124 (TIMEOUT) well under 5s
    let args = [
        "--pe",
        sleep_pe.to_str().unwrap(),
        "--timeout-secs",
        "0.35",
        "--json",
    ];
    let t0 = Instant::now();
    let (code, out, err) = run_runner(&drive_c, &args);
    let elapsed = t0.elapsed();
    assert_eq!(
        code,
        exit_codes::TIMEOUT,
        "expected exit 124 on timeout, got {code}\nstdout:\n{out}\nstderr:\n{err}"
    );
    assert!(
        elapsed.as_secs_f64() < 5.0,
        "timeout took too long: {elapsed:?}"
    );
    let val = parse_and_check_envelope(code, &out, &err, &args);
    let data = val.get("data").unwrap();
    assert_eq!(
        data.get("stop_reason").and_then(JsonValue::as_str),
        Some("timeout")
    );
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let _ = std::fs::remove_dir_all(&dir);
}
