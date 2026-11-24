//! End-to-end integration tests for pre-first-frame watchdog & heartbeat behavior.
//!
//! Compiles real Windows x86-64 PE executables on the fly with
//! `zig cc -target x86_64-windows-gnu` and runs them through `port-runner`:
//! 1. A guest that blocks forever before presenting any frame triggers a
//!    pre-first-frame `[HITCH:STALL]` watchdog report with live thread activity
//!    (`HandleWait`) and captured backtraces.
//! 2. A guest that actively pumps the Win32 message loop (`PeekMessageW`) before
//!    presenting any frame emits heartbeats that suppress false stall alarms.
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
            "port-runner-pre-present-hang-{tag}-{}-{id}",
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

fn compile_guest(zig: &Path, work_dir: &Path, name: &str, src: &str, libs: &[&str]) -> PathBuf {
    let src_path = work_dir.join(format!("{name}.c"));
    let exe_path = work_dir.join(format!("{name}.exe"));
    let zig_cache = work_dir.join("zig-cache");
    std::fs::write(&src_path, src).expect("write guest C source");
    let mut cmd = Command::new(zig);
    cmd.args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path)
        .env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
    for lib in libs {
        cmd.arg(lib);
    }
    let out = cmd.output().expect("spawn zig cc");
    assert!(
        out.status.success(),
        "zig cc failed for {name} (status={:?}):\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(exe_path.is_file(), "compiled PE missing at {}", exe_path.display());
    exe_path
}

const HANG_GUEST_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>

int main(void) {
    HANDLE ev = CreateEventW(NULL, TRUE, FALSE, NULL);
    if (!ev) {
        fprintf(stderr, "FAIL: CreateEventW\n");
        ExitProcess(1);
    }
    printf("PRE_PRESENT_HANG: entering infinite wait\n");
    fflush(stdout);
    /* Block before any graphics device or Present is ever created. */
    WaitForSingleObject(ev, 10000);
    CloseHandle(ev);
    ExitProcess(0);
    return 0;
}
"##;

const PUMPING_GUEST_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>
#include <string.h>

static LRESULT CALLBACK wnd_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    return DefWindowProcW(hwnd, msg, wp, lp);
}

int main(void) {
    HINSTANCE hinst = GetModuleHandleW(NULL);
    WNDCLASSEXW wc;
    memset(&wc, 0, sizeof(wc));
    wc.cbSize = sizeof(wc);
    wc.lpfnWndProc = wnd_proc;
    wc.hInstance = hinst;
    wc.lpszClassName = L"PortRunnerPumpHeartbeatClass";
    if (!RegisterClassExW(&wc)) {
        fprintf(stderr, "FAIL: RegisterClassExW\n");
        ExitProcess(1);
    }
    HWND hwnd = CreateWindowExW(
        0, wc.lpszClassName, L"Pump Heartbeat Test", WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, 160, 120, NULL, NULL, hinst, NULL
    );
    if (!hwnd) {
        fprintf(stderr, "FAIL: CreateWindowExW\n");
        ExitProcess(2);
    }
    /* Pump messages every 40 ms for ~1.2 s (3x PORT_STALL_MS=400) without presenting. */
    for (int i = 0; i < 30; ++i) {
        MSG msg;
        while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE)) {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        Sleep(40);
    }
    DestroyWindow(hwnd);
    printf("PRE_PRESENT_PUMP: PASS\n");
    fflush(stdout);
    ExitProcess(0);
    return 0;
}
"##;

#[test]
fn pre_first_frame_deadlock_triggers_watchdog_report() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping pre_first_frame_deadlock_triggers_watchdog_report: zig not found");
        return;
    };
    let tmp = TempDir::new("deadlock");
    let exe = compile_guest(&zig, tmp.path(), "hang_guest", HANG_GUEST_C, &["-lkernel32"]);

    let log_dir = tmp.path().join("logs");
    let drive_c = tmp.path().join("drive_c");
    let aot_dir = tmp.path().join("aot");
    let hitch_log = tmp.path().join("hitch.log");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(&aot_dir).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_port-runner"))
        .arg("--pe")
        .arg(&exe)
        .env("PORT_HEADLESS", "1")
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("PORT_STALL_MS", "400")
        .env("PORT_HITCH_LOG", &hitch_log)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn port-runner");

    let deadline = Instant::now() + Duration::from_secs(15);
    let mut report = String::new();
    while Instant::now() < deadline {
        if let Ok(contents) = std::fs::read_to_string(&hitch_log) {
            if contents.contains("HandleWait")
                && contents.contains("==================== end of stall report ====================")
            {
                report = contents;
                break;
            }
        }
        if child.try_wait().expect("try_wait").is_some() {
            report = std::fs::read_to_string(&hitch_log).unwrap_or_default();
            break;
        }
        std::thread::sleep(Duration::from_millis(25));
    }

    let _ = child.kill();
    let out = child.wait_with_output().expect("collect output");
    let runner_log = std::fs::read_to_string(log_dir.join("port-runner.log")).unwrap_or_default();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        report.contains("[HITCH:STALL]") && report.contains("Pre-first-frame stall"),
        "expected pre-first-frame [HITCH:STALL] report in {}:\nreport:\n{report}\nstdout:\n{stdout}\nstderr:\n{stderr}\nrunner_log:\n{runner_log}",
        hitch_log.display()
    );
    assert!(
        report.contains("HandleWait"),
        "expected stall report to show guest thread blocked in HandleWait:\n{report}"
    );
    assert!(
        report.contains("hang_guest.exe+") && report.contains("WaitForSingleObject"),
        "expected stall report to include mixed guest+host backtrace for hung guest thread:\n{report}"
    );
    assert!(
        report.contains("==================== end of stall report ===================="),
        "expected complete stall report trailer:\n{report}"
    );
}

#[test]
fn pre_first_frame_message_pump_heartbeats_suppress_false_stall() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping pre_first_frame_message_pump_heartbeats_suppress_false_stall: zig not found");
        return;
    };
    let tmp = TempDir::new("pump-alive");
    let exe = compile_guest(
        &zig,
        tmp.path(),
        "pumping_guest",
        PUMPING_GUEST_C,
        &["-luser32", "-lkernel32"],
    );

    let log_dir = tmp.path().join("logs");
    let drive_c = tmp.path().join("drive_c");
    let aot_dir = tmp.path().join("aot");
    let hitch_log = tmp.path().join("hitch.log");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(&aot_dir).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_port-runner"))
        .arg("--pe")
        .arg(&exe)
        .env("PORT_HEADLESS", "1")
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("PORT_STALL_MS", "400")
        .env("PORT_HITCH_LOG", &hitch_log)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn port-runner");

    let deadline = Instant::now() + Duration::from_secs(30);
    let code = loop {
        if let Some(st) = child.try_wait().expect("try_wait") {
            break st.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(25));
    };

    let out = child.wait_with_output().expect("collect output");
    let mut combined = std::fs::read_to_string(log_dir.join("port-runner.log")).unwrap_or_default();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    combined.push_str("\n--- stdout ---\n");
    combined.push_str(&stdout);
    combined.push_str("\n--- stderr ---\n");
    combined.push_str(&stderr);
    let report = std::fs::read_to_string(&hitch_log).unwrap_or_default();

    assert_eq!(
        code,
        Some(0),
        "pumping guest failed (code={code:?}):\n{combined}"
    );
    assert!(
        combined.contains("PRE_PRESENT_PUMP: PASS"),
        "pumping guest missing PASS marker:\n{combined}"
    );
    assert!(
        !report.contains("[HITCH:STALL]"),
        "active message pump should not trigger false pre-first-frame stall:\n{report}"
    );
}
