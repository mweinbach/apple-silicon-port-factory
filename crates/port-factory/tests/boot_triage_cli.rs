//! End-to-end CLI integration tests for `port-factory boot-triage` using synthetic fixture
//! logs (zero copyrighted game files) and manifest validation for `games/spider-man-2/game.toml`.

use port_factory::json_output::{exit_codes, parse_json, JsonValue, SCHEMA_VERSION};
use port_manifest::GameManifest;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!(
        "pf_boot_triage_cli_{tag}_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn run_boot_triage_cli(args: &[&str]) -> (i32, JsonValue, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_port-factory"))
        .arg("boot-triage")
        .args(args)
        .output()
        .expect("spawn port-factory boot-triage");
    let code = out.status.code().expect("exit code");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let parsed = parse_json(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "boot-triage {:?} (exit {code}) emitted invalid JSON on stdout: {e}\nstdout:\n{stdout}\nstderr:\n{stderr}",
            args
        )
    });
    assert_eq!(
        parsed.get("schema_version").and_then(JsonValue::as_u64),
        Some(SCHEMA_VERSION as u64)
    );
    assert_eq!(
        parsed.get("command").and_then(JsonValue::as_str),
        Some("boot-triage")
    );
    assert_eq!(
        parsed.get("exit_code").and_then(JsonValue::as_i64),
        Some(code as i64)
    );
    assert_eq!(
        parsed.get("ok").and_then(JsonValue::as_bool),
        Some(code == exit_codes::OK)
    );
    (code, parsed, stderr)
}

#[test]
fn cli_classifies_all_seven_blocker_classes_from_synthetic_fixture_logs() {
    let dir = unique_temp_dir("classes");

    struct Case<'a> {
        name: &'a str,
        log: &'a str,
        extra_args: &'a [&'a str],
        expected_code: i32,
        expected_class: &'a str,
        expected_owner_substr: &'a str,
        expected_summary_substr: &'a str,
    }

    let cases = [
        Case {
            name: "unresolved_static_import",
            log: "\
[2026-11-28T05:00:00.100Z INFO  port_loader] loaded 12 modules (2 PE, 10 builtin)
[2026-11-28T05:00:00.200Z ERROR port_loader::thunks] UNIMPLEMENTED secur32.dll!AcquireCredentialsHandleW called from synth.exe+0x2040
",
            extra_args: &["--exit-code", "57"],
            expected_code: exit_codes::UNRESOLVED_IMPORTS,
            expected_class: "unresolved_import",
            expected_owner_substr: "crates/win32-rt",
            expected_summary_substr: "secur32.dll!AcquireCredentialsHandleW",
        },
        Case {
            name: "delay_load_0xc06d007e",
            log: "\
[2026-11-28T05:00:00.100Z WARN  port_loader::loader] LoadLibrary(\"missing_ext.dll\", 0x0) failed (err=126): not found
[2026-11-28T05:00:00.110Z ERROR win32_rt::unwind] UNHANDLED exception 0xc06d007e DELAYLOAD module not found flags=0x0 at synth.exe+0x3100 params=[0x140090000]
",
            extra_args: &["--exit-code", "126"],
            expected_code: exit_codes::UNRESOLVED_IMPORTS,
            expected_class: "unresolved_import",
            expected_owner_substr: "crates/win32-rt",
            expected_summary_substr: "missing_ext.dll",
        },
        Case {
            name: "delay_load_0xc06d007f",
            log: "\
[2026-11-28T05:00:00.100Z WARN  win32_rt::kernel32::module] GetProcAddress(kernel32.dll, FutureWinApi) -> NULL (ERROR_PROC_NOT_FOUND)
[2026-11-28T05:00:00.110Z ERROR win32_rt::unwind] UNHANDLED exception 0xc06d007f DELAYLOAD proc not found flags=0x0 at synth.exe+0x3200 params=[0x140090000]
",
            extra_args: &["--exit-code", "127"],
            expected_code: exit_codes::UNRESOLVED_IMPORTS,
            expected_class: "unresolved_import",
            expected_owner_substr: "crates/win32-rt",
            expected_summary_substr: "kernel32.dll!FutureWinApi",
        },
        Case {
            name: "stub_failing_hit",
            log: "\
[2026-11-28T05:00:00.100Z WARN  stub] STUB QueryOptionalDelayLoadedAPI: QueryOptionalDelayLoadedAPI -> 0 (last error 120)
[2026-11-28T05:00:00.200Z INFO  port_loader] entry point returned 1
",
            extra_args: &["--exit-code", "1"],
            expected_code: exit_codes::RUNTIME_ERROR,
            expected_class: "stub_hit",
            expected_owner_substr: "crates/win32-rt",
            expected_summary_substr: "QueryOptionalDelayLoadedAPI",
        },
        Case {
            name: "guest_crash_with_rva_and_insn",
            log: "\
[2026-11-28T05:00:00.100Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T05:00:00.101Z ERROR win32_rt::crash] exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at synth.exe+0x4480 params=[0x1, 0x0]
[2026-11-28T05:00:00.102Z ERROR win32_rt::crash] access violation: write to 0x0
[2026-11-28T05:00:00.103Z ERROR win32_rt::crash] disassembly @rip:
[2026-11-28T05:00:00.104Z ERROR win32_rt::crash]   => 140004480: c700deadbeef             mov dword ptr [rax],0xefbeadde
[2026-11-28T05:00:00.105Z ERROR win32_rt::crash] ==================== end of crash report (0.5ms) ====================
",
            extra_args: &["--exit-code", "5"],
            expected_code: exit_codes::RUNTIME_ERROR,
            expected_class: "guest_crash",
            expected_owner_substr: "crates/x86-runtime",
            expected_summary_substr: "synth.exe+0x4480",
        },
        Case {
            name: "rust_panic_with_backtrace",
            log: "\
thread 'win32-main' panicked at crates/win32-rt/src/d3d12/heap.rs:45:9:
assertion failed: size > 0
stack backtrace:
   0: std::panicking::begin_panic_handler
   1: win32_rt::d3d12::heap::create_heap
             at crates/win32-rt/src/d3d12/heap.rs:45:9
",
            extra_args: &["--exit-code", "101"],
            expected_code: exit_codes::RUNTIME_ERROR,
            expected_class: "rust_panic_or_abort",
            expected_owner_substr: "crates/win32-rt",
            expected_summary_substr: "RUST_PANIC_OR_ABORT",
        },
        Case {
            name: "metal_validation_error",
            log: "\
[2026-11-28T05:00:00.100Z INFO  d3d] Metal device: Apple M3 Ultra
-[MTLDebugComputeCommandEncoder dispatchThreads:threadsPerThreadgroup:]:912: failed assertion `threadsPerThreadgroup must be non-zero'
Abort trap: 6
",
            extra_args: &["--signal", "6"],
            expected_code: exit_codes::RUNTIME_ERROR,
            expected_class: "metal_validation_error",
            expected_owner_substr: "crates/d3d12-metal",
            expected_summary_substr: "METAL_VALIDATION_ERROR",
        },
        Case {
            name: "d3d12_dxgi_hresult",
            log: "\
[2026-11-28T05:00:00.100Z WARN  d3d] CreateSwapChain(desc): invalid format -> 0x887a0001
[2026-11-28T05:00:00.200Z INFO  port_loader] entry point returned 1
",
            extra_args: &["--exit-code", "1"],
            expected_code: exit_codes::RUNTIME_ERROR,
            expected_class: "d3d12_dxgi_hresult",
            expected_owner_substr: "d3d12-metal",
            expected_summary_substr: "DXGI_ERROR_INVALID_CALL",
        },
        Case {
            name: "hang_timeout_with_last_activity",
            log: "\
[2026-11-28T05:00:00.100Z INFO  port_loader] calling exe entry point synth.exe+0x1000
[2026-11-28T05:00:01.000Z INFO  dxil_metal::pso_cache] compiling pipeline shader hash=0xdeadbeef
[2026-11-28T05:00:30.100Z WARN  port_loader::run_limits] [run-limit] timed out after 30s (30000 ms, frames_presented=0); exiting with code 124
",
            extra_args: &["--exit-code", "124", "--timed-out"],
            expected_code: exit_codes::TIMEOUT,
            expected_class: "hang_timeout",
            expected_owner_substr: "crates/dxil-metal",
            expected_summary_substr: "compiling pipeline shader",
        },
        Case {
            name: "clean_boot",
            log: "\
[2026-11-28T05:00:00.100Z INFO  port_loader] calling exe entry point synth.exe+0x1000
[2026-11-28T05:00:01.500Z INFO  port_loader::run_limits] [run-limit] reached --max-frames 5 (frames_presented=5, elapsed=1400ms); exiting cleanly
",
            extra_args: &["--exit-code", "0"],
            expected_code: exit_codes::OK,
            expected_class: "none",
            expected_owner_substr: "",
            expected_summary_substr: "CLEAN_BOOT",
        },
    ];

    for c in cases {
        let lf = dir.join(format!("{}.log", c.name));
        std::fs::write(&lf, c.log).unwrap();
        let lf_str = lf.to_string_lossy().into_owned();
        let mut args = vec!["--log-file", &lf_str, "--exe", "synth.exe"];
        args.extend_from_slice(c.extra_args);

        // Default mode emits JSON on stdout + short human summary on stderr.
        let (code, parsed, stderr) = run_boot_triage_cli(&args);
        assert_eq!(code, c.expected_code, "exit code mismatch for {}", c.name);
        let data = parsed.get("data").expect("data");
        let blocker = data.get("blocker").expect("blocker");
        assert_eq!(
            blocker.get("class").and_then(JsonValue::as_str),
            Some(c.expected_class),
            "class mismatch for {}",
            c.name
        );
        if !c.expected_owner_substr.is_empty() {
            let owner_crate = data
                .get("owner")
                .and_then(|o| o.get("crate"))
                .and_then(JsonValue::as_str)
                .unwrap_or("");
            assert!(
                owner_crate.contains(c.expected_owner_substr),
                "owner mismatch for {}: got {owner_crate:?}",
                c.name
            );
            assert!(
                stderr.contains(c.expected_owner_substr),
                "stderr human summary for {} should mention owner {:?}, got:\n{stderr}",
                c.name,
                c.expected_owner_substr
            );
        }
        assert!(
            stderr.contains(c.expected_summary_substr),
            "stderr human summary for {} missing {:?}, got:\n{stderr}",
            c.name,
            c.expected_summary_substr
        );
        if c.name == "guest_crash_with_rva_and_insn" {
            assert_eq!(
                blocker.get("location").and_then(JsonValue::as_str),
                Some("synth.exe+0x4480")
            );
            assert_eq!(
                blocker.get("faulting_instruction").and_then(JsonValue::as_str),
                Some("mov dword ptr [rax],0xefbeadde")
            );
        }
    }

    // Verify `--json` suppresses stderr human summary and `--json-out` writes valid JSON.
    let clean_lf = dir.join("clean_boot.log");
    let json_out = dir.join("triage.json");
    let (code, _, stderr) = run_boot_triage_cli(&[
        "--log-file",
        clean_lf.to_str().unwrap(),
        "--exit-code",
        "0",
        "--json",
        "--json-out",
        json_out.to_str().unwrap(),
    ]);
    assert_eq!(code, exit_codes::OK);
    assert!(stderr.trim().is_empty(), "--json mode should keep stderr clean, got: {stderr}");
    let saved = std::fs::read_to_string(&json_out).unwrap();
    let saved_json = parse_json(saved.trim()).expect("valid saved JSON");
    assert_eq!(
        saved_json.get("command").and_then(JsonValue::as_str),
        Some("boot-triage")
    );

    // Verify usage error returns exit code 2 with Schema v1 error envelope under --json.
    let (code, parsed, _) = run_boot_triage_cli(&["--timeout", "0", "--json"]);
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert_eq!(parsed.get("ok").and_then(JsonValue::as_bool), Some(false));
    assert!(parsed.get("data").unwrap().is_null());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn spider_man_2_manifest_parses_cleanly_without_personal_paths() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../games/spider-man-2/game.toml");
    assert!(
        manifest_path.is_file(),
        "missing manifest at {}",
        manifest_path.display()
    );
    let raw = std::fs::read_to_string(&manifest_path).unwrap();
    assert!(
        !raw.contains("/Users/") && !raw.contains("/Volumes/"),
        "manifest must not contain personal filesystem paths"
    );
    let m = GameManifest::parse(&raw);
    assert_eq!(m.id.as_deref(), Some("spider-man-2"));
    assert_eq!(m.exe.as_deref(), Some("Spider-Man2.exe"));
    assert_eq!(m.v8_jitless, Some(true));
    assert_eq!(m.preferred_base_retry, Some(false));
    assert_eq!(m.legacy_remedy_migration, Some(false));
    assert!(m.middleware.contains_key("eos"));
    assert!(m.middleware.contains_key("dstorage"));
}
