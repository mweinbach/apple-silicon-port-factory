//! End-to-end integration tests for fail-loudly guarantees, compiling real
//! Windows x86-64 PE executables on the fly with `zig cc -target x86_64-windows-gnu`
//! and running them through `port-runner`:
//!
//! 1. Direct x86-64 `syscall` (`0F 05`):
//!    - Unknown SSN returns `STATUS_NOT_IMPLEMENTED` (`0xC0000002`) and logs a
//!      warning once per distinct SSN.
//!    - `NtReleaseSemaphore` (`0x000A`), `NtWaitForMultipleObjects` (`0x005B`),
//!      and `NtRemoveIoCompletion` (`0x0009`) execute real handle/wait/IOCP
//!      operations with NT semantics and status codes.
//! 2. Unimplemented x86-64 instructions (`rdpkru` = `0F 01 EE`):
//!    - Fail loudly by default on CLI bring-up runs (and with `--fail-on-unimpl-op`
//!      / `PORT_FAIL_ON_UNIMPL_OP=1` / `live_probe.py`) with documented exit code
//!      `86` (`EXIT_UNIMPL_OP`) and a crash report.
//!    - Continue as no-op when opted out with `--no-fail-on-unimpl-op` or
//!      `PORT_FAIL_ON_UNIMPL_OP=0`.
//! 3. Missing COM methods + silent lookup failures:
//!    - Failed `GetProcAddress`, unknown CLSIDs in `CoCreateInstance`, and failed
//!      `QueryInterface` each log a warning once per distinct failure.
//!    - Calling an unimplemented COM method on ARM64 hits a numbered trap entry
//!      that names the exact `IFoo::Method`, writes a crash report, and exits
//!      with `STATUS_ENTRYPOINT_NOT_FOUND` (`0xC0000139`, POSIX `0x39` = 57).
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "port-runner-fail-loudly-{tag}-{}-{id}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp test dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn find_zig() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("ZIG").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Some(pb);
        }
    }
    if let Some(path_env) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let cand = dir.join("zig");
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    for fixed in ["/opt/homebrew/bin/zig", "/usr/local/bin/zig"] {
        let pb = PathBuf::from(fixed);
        if pb.is_file() {
            return Some(pb);
        }
    }
    None
}

fn compile_guest_exe(
    zig: &Path,
    work_dir: &Path,
    name: &str,
    source: &str,
    libs: &[&str],
) -> PathBuf {
    let src_path = work_dir.join(format!("{name}.c"));
    let exe_path = work_dir.join(format!("{name}.exe"));
    std::fs::write(&src_path, source).expect("write guest C source");
    let zig_cache = work_dir.join("zig-cache");
    let mut cmd = Command::new(zig);
    cmd.args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path)
        .args(libs)
        .env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
    let output = cmd.output().expect("spawn zig cc");
    assert!(
        output.status.success(),
        "zig cc failed for {name} (status={:?}):\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(exe_path.is_file(), "compiled PE missing at {}", exe_path.display());
    exe_path
}

struct RunResult {
    exit_code: Option<i32>,
    log: String,
    crash_reports: Vec<(PathBuf, String)>,
}

fn run_in_port_runner(
    work_dir: &Path,
    exe_path: &Path,
    extra_args: &[&str],
    extra_env: &[(&str, Option<&str>)],
) -> RunResult {
    let log_dir = work_dir.join("logs");
    let drive_c = work_dir.join("drive_c");
    let aot_dir = work_dir.join("aot");
    let crash_dir = work_dir.join("crash");
    let _ = std::fs::remove_dir_all(&log_dir);
    let _ = std::fs::remove_dir_all(&crash_dir);
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(&aot_dir).unwrap();
    std::fs::create_dir_all(&crash_dir).unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-runner"));
    cmd.arg("--pe")
        .arg(exe_path)
        .args(extra_args)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("PORT_CRASH_DIR", &crash_dir)
        .env("AW2_CRASH_DIR", &crash_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("RUST_LOG", "info")
        .env_remove("PORT_FAIL_ON_UNIMPL_OP")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for &(k, v) in extra_env {
        match v {
            Some(val) => {
                cmd.env(k, val);
            }
            None => {
                cmd.env_remove(k);
            }
        }
    }

    let mut child = cmd.spawn().expect("spawn port-runner");
    let deadline = Instant::now() + Duration::from_secs(30);
    let exit_code = loop {
        if let Some(st) = child.try_wait().expect("try_wait port-runner") {
            break st.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(20));
    };

    let out = child.wait_with_output().expect("collect output");
    let mut combined = String::new();
    let log_file = log_dir.join("port-runner.log");
    if let Ok(file_log) = std::fs::read_to_string(&log_file) {
        combined.push_str(&file_log);
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !stdout.is_empty() {
        combined.push_str("\n--- stdout ---\n");
        combined.push_str(&stdout);
    }
    if !stderr.is_empty() {
        combined.push_str("\n--- stderr ---\n");
        combined.push_str(&stderr);
    }

    let mut crash_reports = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&crash_dir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Ok(body) = std::fs::read_to_string(&p) {
                    crash_reports.push((p, body));
                }
            }
        }
    }
    RunResult { exit_code, log: combined, crash_reports }
}

fn count_lines_containing(haystack: &str, needle: &str) -> usize {
    haystack.lines().filter(|l| l.contains(needle)).count()
}

#[test]
fn guest_direct_nt_syscalls_and_unknown_ssn() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping guest_direct_nt_syscalls_and_unknown_ssn: zig not found");
        return;
    };
    let tmp = TempDir::new("ntsyscall");
    let exe = compile_guest_exe(
        &zig,
        tmp.path(),
        "ntsyscall_guest",
        GUEST_NTSYSCALL_C,
        &["-lkernel32"],
    );
    let res = run_in_port_runner(tmp.path(), &exe, &[], &[]);
    assert_eq!(
        res.exit_code,
        Some(0),
        "ntsyscall_guest failed (exit={:?}):\n{}",
        res.exit_code,
        res.log
    );
    assert!(
        res.log.contains("NTSYSCALL_E2E: PASS ALL"),
        "missing pass marker:\n{}",
        res.log
    );
    // Guest invokes unknown SSN 0x0FFE twice; port-runner.log must emit the warning exactly once.
    let log_only = std::fs::read_to_string(tmp.path().join("logs/port-runner.log"))
        .unwrap_or_else(|_| res.log.clone());
    let warn_count = count_lines_containing(
        &log_only,
        "ntsyscall: unhandled direct syscall unknown (SSN=0x0ffe",
    );
    assert_eq!(
        warn_count, 1,
        "expected exactly 1 warning for repeated unknown SSN 0x0ffe, got {warn_count}:\n{log_only}"
    );
}

#[test]
fn guest_unimpl_op_fails_loudly_and_supports_opt_out() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping guest_unimpl_op_fails_loudly_and_supports_opt_out: zig not found");
        return;
    };
    let tmp = TempDir::new("unimpl-op");
    let exe = compile_guest_exe(
        &zig,
        tmp.path(),
        "unimpl_op_guest",
        GUEST_UNIMPL_OP_C,
        &["-lkernel32"],
    );

    // 1. Default CLI bring-up run (no flag, no env): fail-on-unimpl-op is ON by default -> exit 86.
    let res_default = run_in_port_runner(tmp.path(), &exe, &[], &[]);
    assert_eq!(
        res_default.exit_code,
        Some(x86_runtime::emu::EXIT_UNIMPL_OP as i32),
        "expected exit 86 in default bring-up mode:\n{}",
        res_default.log
    );
    assert!(
        res_default.log.contains("UNIMPLEMENTED mnemonic Rdpkru"),
        "expected Rdpkru unimpl-op diagnostic:\n{}",
        res_default.log
    );
    assert!(
        !res_default.log.contains("SURVIVED_UNIMPL_OP"),
        "guest should have aborted before printing survival marker:\n{}",
        res_default.log
    );
    assert!(
        !res_default.crash_reports.is_empty(),
        "expected crash report for unimplemented x86 instruction:\n{}",
        res_default.log
    );
    assert!(
        res_default
            .crash_reports
            .iter()
            .any(|(_, body)| body.contains("unimplemented x86 instruction Rdpkru")),
        "crash report missing unimplemented instruction title: {:?}",
        res_default.crash_reports
    );

    // 2. Explicit --fail-on-unimpl-op overrides PORT_FAIL_ON_UNIMPL_OP=0 -> exit 86.
    let res_cli_on = run_in_port_runner(
        tmp.path(),
        &exe,
        &["--fail-on-unimpl-op"],
        &[("PORT_FAIL_ON_UNIMPL_OP", Some("0"))],
    );
    assert_eq!(
        res_cli_on.exit_code,
        Some(x86_runtime::emu::EXIT_UNIMPL_OP as i32),
        "expected --fail-on-unimpl-op to override env=0:\n{}",
        res_cli_on.log
    );

    // 3. Explicit --no-fail-on-unimpl-op continues as no-op -> exit 0.
    let res_cli_off = run_in_port_runner(tmp.path(), &exe, &["--no-fail-on-unimpl-op"], &[]);
    assert_eq!(
        res_cli_off.exit_code,
        Some(0),
        "expected exit 0 with --no-fail-on-unimpl-op:\n{}",
        res_cli_off.log
    );
    assert!(
        res_cli_off.log.contains("SURVIVED_UNIMPL_OP"),
        "guest did not continue past unimpl op with --no-fail-on-unimpl-op:\n{}",
        res_cli_off.log
    );

    // 4. PORT_FAIL_ON_UNIMPL_OP=0 (without CLI flag) continues as no-op -> exit 0.
    let res_env_off = run_in_port_runner(
        tmp.path(),
        &exe,
        &[],
        &[("PORT_FAIL_ON_UNIMPL_OP", Some("0"))],
    );
    assert_eq!(
        res_env_off.exit_code,
        Some(0),
        "expected exit 0 with PORT_FAIL_ON_UNIMPL_OP=0:\n{}",
        res_env_off.log
    );
    assert!(
        res_env_off.log.contains("SURVIVED_UNIMPL_OP"),
        "guest did not continue past unimpl op with PORT_FAIL_ON_UNIMPL_OP=0:\n{}",
        res_env_off.log
    );

    // 5. live_probe.py turns --fail-on-unimpl-op ON by default and classifies exit 86 as BUG (rc=1).
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let live_probe = repo_root.join("tools/debug/live_probe.py");
    let probe_log = tmp.path().join("logs/port-runner.log");
    let probe_out = Command::new("python3")
        .arg(&live_probe)
        .args([
            "--pe",
            exe.to_str().unwrap(),
            "--runner",
            env!("CARGO_BIN_EXE_port-runner"),
            "--log-file",
            probe_log.to_str().unwrap(),
            "--ready-regex",
            "Present #10",
            "--timeout-boot",
            "15",
            "--extra-env",
            "PORT_HEADLESS=1",
            "--extra-env",
            &format!("PORT_LOG_DIR={}", tmp.path().join("logs").display()),
            "--extra-env",
            &format!("PORT_CRASH_DIR={}", tmp.path().join("crash").display()),
        ])
        .output()
        .expect("run live_probe.py");
    let probe_stdout = String::from_utf8_lossy(&probe_out.stdout);
    let probe_stderr = String::from_utf8_lossy(&probe_out.stderr);
    assert_eq!(
        probe_out.status.code(),
        Some(1),
        "expected live_probe.py to exit 1 (BUG) on exit 86:\nstdout:\n{probe_stdout}\nstderr:\n{probe_stderr}"
    );
    assert!(
        probe_stdout.contains("unimplemented x86 instruction, exit code 86"),
        "expected live_probe.py to report exit 86:\nstdout:\n{probe_stdout}\nstderr:\n{probe_stderr}"
    );
}

#[test]
fn guest_missing_com_method_and_lookup_warnings() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping guest_missing_com_method_and_lookup_warnings: zig not found");
        return;
    };
    let tmp = TempDir::new("com-trap");
    let exe = compile_guest_exe(
        &zig,
        tmp.path(),
        "com_trap_guest",
        GUEST_COM_TRAP_C,
        &["-ldxgi", "-lole32", "-lkernel32", "-luuid"],
    );
    let res = run_in_port_runner(tmp.path(), &exe, &[], &[]);
    // STATUS_ENTRYPOINT_NOT_FOUND = 0xC0000139 -> low 8 bits on POSIX = 0x39 (57)
    assert_eq!(
        res.exit_code,
        Some(0x39),
        "expected COM trap exit code 0x39 (0xC0000139):\n{}",
        res.log
    );

    let log_only = std::fs::read_to_string(tmp.path().join("logs/port-runner.log"))
        .unwrap_or_else(|_| res.log.clone());

    // 1. Failed GetProcAddress logged once at WARN level
    assert_eq!(
        count_lines_containing(&log_only, "MissingExport_FailLoudly_Test"),
        1,
        "expected exactly 1 warning for repeated failed GetProcAddress:\n{log_only}"
    );
    assert!(
        log_only.contains("WARN") && log_only.contains("MissingExport_FailLoudly_Test"),
        "expected WARN log for failed GetProcAddress:\n{log_only}"
    );

    // 2. Unknown CLSID in CoCreateInstance logged once at WARN level
    assert_eq!(
        count_lines_containing(&log_only, "DEADBEEF-1111-2222-3333-444455556666"),
        1,
        "expected exactly 1 warning for repeated unknown CLSID:\n{log_only}"
    );

    // 3. Failed QueryInterface logged once at WARN level
    assert_eq!(
        count_lines_containing(&log_only, "CAFEBABE-1111-2222-3333-444455556666"),
        1,
        "expected exactly 1 warning for repeated failed QueryInterface:\n{log_only}"
    );

    // 4. Numbered ARM64 COM trap names the exact interface + method and writes a crash report
    assert!(
        res.log.contains("UNIMPLEMENTED IDXGIOutput6::GetGammaControlCapabilities"),
        "COM trap must name exact interface and method:\n{}",
        res.log
    );
    assert!(
        !res.log.contains("UNIMPLEMENTED COM method called on aarch64"),
        "old generic COM trap string must not appear:\n{}",
        res.log
    );
    assert!(
        !res.crash_reports.is_empty(),
        "expected crash report for missing COM method:\n{}",
        res.log
    );
    assert!(
        res.crash_reports.iter().any(|(_, body)| {
            body.contains("unimplemented COM method IDXGIOutput6::GetGammaControlCapabilities")
        }),
        "crash report missing COM method name: {:?}",
        res.crash_reports
    );
}

const GUEST_NTSYSCALL_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

typedef int32_t NTSTATUS;
typedef struct {
    union {
        NTSTATUS Status;
        void *Pointer;
    };
    uintptr_t Information;
} MY_IO_STATUS_BLOCK;

static __attribute__((noinline)) NTSTATUS raw_syscall5(
    uint32_t ssn, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5
) {
    register uint64_t r10 __asm__("r10") = a1;
    register uint64_t rdx __asm__("rdx") = a2;
    register uint64_t r8 __asm__("r8") = a3;
    register uint64_t r9 __asm__("r9") = a4;
    uint64_t rax = ssn;
    /* Place a5 at [rsp + 0x28] across the syscall per Windows x64 syscall stack ABI. */
    __asm__ __volatile__(
        "subq $0x38, %%rsp\n\t"
        "movq %[a5], 0x28(%%rsp)\n\t"
        "syscall\n\t"
        "addq $0x38, %%rsp\n\t"
        : "+a"(rax), "+r"(r10), "+r"(rdx), "+r"(r8), "+r"(r9)
        : [a5] "r"(a5)
        : "rcx", "r11", "memory", "cc"
    );
    return (NTSTATUS)rax;
}

#define CHECK(cond, msg) do { if (!(cond)) { fprintf(stderr, "FAIL: %s\n", msg); fflush(stderr); return 1; } } while (0)

int main(void) {
    /* 1. Unknown direct NT syscall SSN 0x0FFE called twice -> STATUS_NOT_IMPLEMENTED (0xC0000002). */
    NTSTATUS u1 = raw_syscall5(0x0FFE, 1, 2, 3, 4, 0);
    NTSTATUS u2 = raw_syscall5(0x0FFE, 5, 6, 7, 8, 0);
    CHECK((uint32_t)u1 == 0xC0000002u, "unknown SSN #1 != STATUS_NOT_IMPLEMENTED");
    CHECK((uint32_t)u2 == 0xC0000002u, "unknown SSN #2 != STATUS_NOT_IMPLEMENTED");

    /* 2. NtReleaseSemaphore (0x000A) */
    HANDLE sem = CreateSemaphoreW(NULL, 0, 2, NULL);
    CHECK(sem != NULL, "CreateSemaphoreW");
    LONG prev = -1;
    NTSTATUS s1 = raw_syscall5(0x000A, (uint64_t)(uintptr_t)sem, 2, (uint64_t)(uintptr_t)&prev, 0, 0);
    CHECK(s1 == 0 && prev == 0, "NtReleaseSemaphore valid release");
    NTSTATUS s2 = raw_syscall5(0x000A, (uint64_t)(uintptr_t)sem, 1, (uint64_t)(uintptr_t)&prev, 0, 0);
    CHECK((uint32_t)s2 == 0xC0000047u, "NtReleaseSemaphore limit exceeded");

    /* 3. NtWaitForMultipleObjects (0x005B): WaitAny=1, WaitAll=0 */
    HANDLE ev0 = CreateEventW(NULL, TRUE, FALSE, NULL);
    HANDLE ev1 = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK(ev0 != NULL && ev1 != NULL, "CreateEventW");
    HANDLE hs[2] = { ev0, ev1 };
    int64_t zero = 0;

    NTSTATUS w0 = raw_syscall5(0x005B, 2, (uint64_t)(uintptr_t)hs, 1 /* WaitAny */, 0, (uint64_t)(uintptr_t)&zero);
    CHECK(w0 == 0x00000102, "NtWaitForMultipleObjects WaitAny timeout");

    SetEvent(ev1);
    NTSTATUS w1 = raw_syscall5(0x005B, 2, (uint64_t)(uintptr_t)hs, 1 /* WaitAny */, 0, (uint64_t)(uintptr_t)&zero);
    CHECK(w1 == 1, "NtWaitForMultipleObjects WaitAny -> STATUS_WAIT_0 + 1");

    NTSTATUS w2 = raw_syscall5(0x005B, 2, (uint64_t)(uintptr_t)hs, 0 /* WaitAll */, 0, (uint64_t)(uintptr_t)&zero);
    CHECK(w2 == 0x00000102, "NtWaitForMultipleObjects WaitAll partial timeout");

    SetEvent(ev0);
    NTSTATUS w3 = raw_syscall5(0x005B, 2, (uint64_t)(uintptr_t)hs, 0 /* WaitAll */, 0, (uint64_t)(uintptr_t)&zero);
    CHECK(w3 == 0, "NtWaitForMultipleObjects WaitAll -> STATUS_WAIT_0");

    /* Wrong handle type on NtReleaseSemaphore -> STATUS_OBJECT_TYPE_MISMATCH (0xC0000024) */
    NTSTATUS s_bad = raw_syscall5(0x000A, (uint64_t)(uintptr_t)ev0, 1, 0, 0, 0);
    CHECK((uint32_t)s_bad == 0xC0000024u, "NtReleaseSemaphore wrong handle type");

    /* 4. NtRemoveIoCompletion (0x0009) */
    HANDLE iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 0);
    CHECK(iocp != NULL, "CreateIoCompletionPort");
    uintptr_t key = 0;
    void *apc = NULL;
    MY_IO_STATUS_BLOCK iosb = {0};

    NTSTATUS i0 = raw_syscall5(
        0x0009,
        (uint64_t)(uintptr_t)iocp,
        (uint64_t)(uintptr_t)&key,
        (uint64_t)(uintptr_t)&apc,
        (uint64_t)(uintptr_t)&iosb,
        (uint64_t)(uintptr_t)&zero
    );
    CHECK(i0 == 0x00000102, "NtRemoveIoCompletion empty timeout");

    CHECK(PostQueuedCompletionStatus(iocp, 0x333, 0x11223344, (LPOVERLAPPED)(uintptr_t)0x55667788), "PostQueuedCompletionStatus");
    NTSTATUS i1 = raw_syscall5(
        0x0009,
        (uint64_t)(uintptr_t)iocp,
        (uint64_t)(uintptr_t)&key,
        (uint64_t)(uintptr_t)&apc,
        (uint64_t)(uintptr_t)&iosb,
        (uint64_t)(uintptr_t)&zero
    );
    CHECK(i1 == 0, "NtRemoveIoCompletion dequeue status");
    CHECK(key == 0x11223344, "NtRemoveIoCompletion key mismatch");
    CHECK((uintptr_t)apc == 0x55667788, "NtRemoveIoCompletion apc mismatch");
    CHECK(iosb.Status == 0 && iosb.Information == 0x333, "NtRemoveIoCompletion iosb mismatch");

    CloseHandle(iocp);
    CloseHandle(ev0);
    CloseHandle(ev1);
    CloseHandle(sem);

    /* 5. Direct NtContinue (0x0043) syscall outside any exception handler (NTCONTINUE_STACK empty):
     *    captures a live CONTEXT, rewrites Rip/Rbx/R12/Rax, invokes raw `syscall`, and verifies
     *    execution resumes at the target label with the restored registers instead of crashing. */
    NTSTATUS nc_null = raw_syscall5(0x0043, 0, 0, 0, 0, 0);
    CHECK((uint32_t)nc_null == 0xC000000Du, "NtContinue(NULL) != STATUS_INVALID_PARAMETER");

    __attribute__((aligned(16))) CONTEXT cont_ctx;
    RtlCaptureContext(&cont_ctx);
    CONTEXT *volatile ctx_ptr = &cont_ctx;
    uint64_t restored_rbx = 0, restored_r12 = 0, restored_rax = 0;
    __asm__ __volatile__(
        "movq %[ctxp], %%r10\n\t"
        "leaq 1f(%%rip), %%rax\n\t"
        "movq %%rax, %c[rip_off](%%r10)\n\t"
        "movq %%rsp, %c[rsp_off](%%r10)\n\t"
        "movq %%rbp, %c[rbp_off](%%r10)\n\t"
        "movabsq $0x1122334455667788, %%rax\n\t"
        "movq %%rax, %c[rax_off](%%r10)\n\t"
        "movabsq $0xA1B2C3D4E5F60718, %%rax\n\t"
        "movq %%rax, %c[rbx_off](%%r10)\n\t"
        "movabsq $0x0F1E2D3C4B5A6978, %%rax\n\t"
        "movq %%rax, %c[r12_off](%%r10)\n\t"
        "xorl %%edx, %%edx\n\t"
        "movl $0x0043, %%eax\n\t"
        "syscall\n\t"
        "ud2\n\t"
        "1:\n\t"
        "movq %%rax, %[out_rax]\n\t"
        "movq %%rbx, %[out_rbx]\n\t"
        "movq %%r12, %[out_r12]\n\t"
        : [out_rax] "=m"(restored_rax),
          [out_rbx] "=m"(restored_rbx),
          [out_r12] "=m"(restored_r12)
        : [ctxp] "m"(ctx_ptr),
          [rip_off] "i"(offsetof(CONTEXT, Rip)),
          [rsp_off] "i"(offsetof(CONTEXT, Rsp)),
          [rbp_off] "i"(offsetof(CONTEXT, Rbp)),
          [rax_off] "i"(offsetof(CONTEXT, Rax)),
          [rbx_off] "i"(offsetof(CONTEXT, Rbx)),
          [r12_off] "i"(offsetof(CONTEXT, R12))
        : "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "memory", "cc"
    );
    CHECK(restored_rax == 0x1122334455667788ULL, "NtContinue direct syscall rax mismatch");
    CHECK(restored_rbx == 0xA1B2C3D4E5F60718ULL, "NtContinue direct syscall rbx mismatch");
    CHECK(restored_r12 == 0x0F1E2D3C4B5A6978ULL, "NtContinue direct syscall r12 mismatch");

    printf("NTSYSCALL_E2E: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

const GUEST_UNIMPL_OP_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>

static __attribute__((noinline)) void run_rdpkru(void) {
    /* 0F 01 EE = rdpkru (unimplemented in x86-runtime generic interpreter). */
    __asm__ __volatile__(".byte 0x0f, 0x01, 0xee" : : "c"(0) : "eax", "edx");
}

int main(void) {
    run_rdpkru();
    printf("SURVIVED_UNIMPL_OP\n");
    fflush(stdout);
    return 0;
}
"##;

const GUEST_COM_TRAP_C: &str = r##"
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <objbase.h>
#include <dxgi.h>
#include <stdio.h>

static const GUID MY_IID_IDXGIFactory1 =
    {0x770aae78, 0xf26f, 0x4dba, {0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87}};
static const GUID UNKNOWN_CLSID =
    {0xdeadbeef, 0x1111, 0x2222, {0x33, 0x33, 0x44, 0x44, 0x55, 0x55, 0x66, 0x66}};
static const GUID BOGUS_IID =
    {0xcafebabe, 0x1111, 0x2222, {0x33, 0x33, 0x44, 0x44, 0x55, 0x55, 0x66, 0x66}};

#define CHECK(cond, msg) do { if (!(cond)) { fprintf(stderr, "FAIL: %s\n", msg); fflush(stderr); return 1; } } while (0)

int main(void) {
    /* 1. Failed GetProcAddress called twice -> warn once. */
    HMODULE k32 = GetModuleHandleA("kernel32.dll");
    CHECK(k32 != NULL, "GetModuleHandleA(kernel32)");
    CHECK(GetProcAddress(k32, "MissingExport_FailLoudly_Test") == NULL, "GetProcAddress #1");
    CHECK(GetLastError() == ERROR_PROC_NOT_FOUND, "GetLastError == ERROR_PROC_NOT_FOUND");
    CHECK(GetProcAddress(k32, "MissingExport_FailLoudly_Test") == NULL, "GetProcAddress #2");

    /* 2. Unknown CLSID in CoCreateInstance called twice -> warn once. */
    CoInitializeEx(NULL, COINIT_MULTITHREADED);
    void *obj = NULL;
    HRESULT hr1 = CoCreateInstance(&UNKNOWN_CLSID, NULL, CLSCTX_INPROC_SERVER, &MY_IID_IDXGIFactory1, &obj);
    HRESULT hr2 = CoCreateInstance(&UNKNOWN_CLSID, NULL, CLSCTX_INPROC_SERVER, &MY_IID_IDXGIFactory1, &obj);
    CHECK(hr1 == REGDB_E_CLASSNOTREG && hr2 == REGDB_E_CLASSNOTREG, "CoCreateInstance unknown CLSID");

    /* 3. Failed QueryInterface on IDXGIFactory7 called twice -> warn once. */
    IDXGIFactory1 *fac = NULL;
    CHECK(SUCCEEDED(CreateDXGIFactory1(&MY_IID_IDXGIFactory1, (void **)&fac)) && fac != NULL, "CreateDXGIFactory1");
    void *bad = NULL;
    HRESULT q1 = IDXGIFactory1_QueryInterface(fac, &BOGUS_IID, &bad);
    HRESULT q2 = IDXGIFactory1_QueryInterface(fac, &BOGUS_IID, &bad);
    CHECK(q1 == E_NOINTERFACE && q2 == E_NOINTERFACE, "QueryInterface bogus IID");

    /* 4. Invoke an unimplemented COM method on IDXGIOutput6 -> numbered ARM64 trap + crash report. */
    IDXGIAdapter *adapter = NULL;
    CHECK(SUCCEEDED(IDXGIFactory1_EnumAdapters(fac, 0, &adapter)) && adapter != NULL, "EnumAdapters(0)");
    IDXGIOutput *output = NULL;
    CHECK(SUCCEEDED(IDXGIAdapter_EnumOutputs(adapter, 0, &output)) && output != NULL, "EnumOutputs(0)");
    DXGI_GAMMA_CONTROL_CAPABILITIES caps;
    IDXGIOutput_GetGammaControlCapabilities(output, &caps);

    /* Never reached. */
    fprintf(stderr, "FAIL: COM trap returned unexpectedly\n");
    return 2;
}
"##;
