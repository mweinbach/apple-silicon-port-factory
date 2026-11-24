//! Shared test harness for the headless Windows x86-64 guest corpus
//! (`os_guest_corpus.rs` and `d3d12_e2e.rs`).
//!
//! Compiles self-checking Windows x86-64 PE executables on the fly with
//! `zig cc` / `zig c++` (`-target x86_64-windows-gnu`), runs each guest once on
//! the live JIT (`PORT_AOT=0`) and once from a prewarmed `.aot` cache
//! (`--prewarm-aot` followed by `PORT_AOT=1`), and asserts exit status,
//! documented markers, AOT cache activation, and zero unexpected runtime/stub
//! errors.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

pub struct TempDir(PathBuf);

impl TempDir {
    pub fn new(tag: &str) -> Self {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "port-guest-corpus-{tag}-{}-{id}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp corpus dir");
        Self(dir)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn find_zig() -> Option<PathBuf> {
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

pub fn has_dxilconv() -> bool {
    if let Some(p) = std::env::var_os("AW2_DXILCONV").filter(|v| !v.is_empty()) {
        if Path::new(&p).is_file() {
            return true;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        if PathBuf::from(home).join(".local/lib/libdxilconv.dylib").is_file() {
            return true;
        }
    }
    Path::new("/opt/homebrew/lib/libdxilconv.dylib").is_file()
        || Path::new("/usr/local/lib/libdxilconv.dylib").is_file()
}

pub fn has_metal_irconverter() -> bool {
    if let Some(p) = std::env::var_os("METAL_IRCONVERTER_DYLIB").filter(|v| !v.is_empty()) {
        if Path::new(&p).is_file() {
            return true;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        if PathBuf::from(home)
            .join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib")
            .is_file()
        {
            return true;
        }
    }
    Path::new("/usr/local/lib/libmetalirconverter.dylib").is_file()
        || Path::new("/opt/metal-shaderconverter/lib/libmetalirconverter.dylib").is_file()
}

pub fn compile_c_guest(
    zig: &Path,
    work_dir: &Path,
    name: &str,
    src: &str,
    libs: &[&str],
) -> PathBuf {
    let src_path = work_dir.join(format!("{name}.c"));
    let exe_path = work_dir.join(format!("{name}.exe"));
    std::fs::write(&src_path, src).expect("write guest C source");
    let zig_cache = work_dir.join("zig-cache");
    let mut cmd = Command::new(zig);
    cmd.args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path);
    for lib in libs {
        cmd.arg(format!("-l{lib}"));
    }
    cmd.env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
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

pub fn compile_cpp_guest(
    zig: &Path,
    work_dir: &Path,
    name: &str,
    src: &str,
    libs: &[&str],
) -> PathBuf {
    let src_path = work_dir.join(format!("{name}.cpp"));
    let exe_path = work_dir.join(format!("{name}.exe"));
    std::fs::write(&src_path, src).expect("write guest C++ source");
    let zig_cache = work_dir.join("zig-cache");
    let mut cmd = Command::new(zig);
    cmd.args([
        "c++",
        "-target",
        "x86_64-windows-gnu",
        "-O2",
        "-static",
        "-Wall",
        "-Wextra",
    ])
    .arg(&src_path)
    .arg("-o")
    .arg(&exe_path);
    for lib in libs {
        cmd.arg(format!("-l{lib}"));
    }
    cmd.env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
    let output = cmd.output().expect("spawn zig c++");
    assert!(
        output.status.success(),
        "zig c++ failed for {name} (status={:?}):\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(exe_path.is_file(), "compiled PE missing at {}", exe_path.display());
    exe_path
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecMode {
    LiveJit,
    PrewarmedAot,
}

impl ExecMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::LiveJit => "live-jit",
            Self::PrewarmedAot => "prewarmed-aot",
        }
    }
}

pub struct RunResult {
    pub exit_code: Option<i32>,
    pub log: String,
    pub stdout: String,
    pub stderr: String,
}

fn wait_with_timeout(mut child: std::process::Child, timeout: Duration) -> RunResult {
    use std::io::Read;
    let out_pipe = child.stdout.take();
    let err_pipe = child.stderr.take();
    let out_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut p) = out_pipe {
            let _ = p.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    });
    let err_reader = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut p) = err_pipe {
            let _ = p.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    });

    let deadline = Instant::now() + timeout;
    let code = loop {
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
    let stdout = out_reader.join().unwrap_or_default();
    let stderr = err_reader.join().unwrap_or_default();
    let mut combined = String::new();
    if !stdout.is_empty() {
        combined.push_str("\n--- stdout ---\n");
        combined.push_str(&stdout);
    }
    if !stderr.is_empty() {
        combined.push_str("\n--- stderr ---\n");
        combined.push_str(&stderr);
    }
    RunResult {
        exit_code: code,
        log: combined,
        stdout,
        stderr,
    }
}

pub fn prewarm_aot(work_dir: &Path, exe_path: &Path, aot_dir: &Path) -> String {
    let log_dir = work_dir.join("logs-prewarm");
    let drive_c = work_dir.join("drive_c-prewarm");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(aot_dir).unwrap();

    let child = Command::new(env!("CARGO_BIN_EXE_port-runner"))
        .arg("--pe")
        .arg(exe_path)
        .arg("--prewarm-aot")
        .arg("--aot-dir")
        .arg(aot_dir)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", aot_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn port-runner --prewarm-aot");

    let mut res = wait_with_timeout(child, Duration::from_secs(60));
    let log_file = log_dir.join("port-runner.log");
    if let Ok(file_log) = std::fs::read_to_string(&log_file) {
        res.log = format!("{file_log}{}", res.log);
    }
    assert_eq!(
        res.exit_code,
        Some(0),
        "prewarm-aot failed for {} (exit={:?}):\n{}",
        exe_path.display(),
        res.exit_code,
        res.log
    );

    let has_aot = std::fs::read_dir(aot_dir)
        .expect("read aot_dir")
        .flatten()
        .any(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("aot"))
        });
    assert!(
        has_aot,
        "prewarm-aot produced no .aot cache file in {}:\n{}",
        aot_dir.display(),
        res.log
    );
    res.log
}

pub fn run_guest(
    work_dir: &Path,
    exe_path: &Path,
    aot_dir: &Path,
    mode: ExecMode,
    timeout: Duration,
) -> RunResult {
    let tag = mode.label();
    let log_dir = work_dir.join(format!("logs-{tag}"));
    let drive_c = work_dir.join(format!("drive_c-{tag}"));
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(aot_dir).unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-runner"));
    cmd.arg("--pe")
        .arg(exe_path)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", aot_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("PORT_AOT_GUARD", "0")
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match mode {
        ExecMode::LiveJit => {
            cmd.env("PORT_AOT", "0").env("AW2_DLL_AOT", "0");
        }
        ExecMode::PrewarmedAot => {
            cmd.env("PORT_AOT", "1").env("AW2_DLL_AOT", "1");
        }
    }

    let child = cmd.spawn().expect("spawn port-runner");
    let mut res = wait_with_timeout(child, timeout);
    let log_file = log_dir.join("port-runner.log");
    if let Ok(file_log) = std::fs::read_to_string(&log_file) {
        res.log = format!("{file_log}{}", res.log);
    }
    res
}

pub fn assert_clean_run(
    name: &str,
    mode: ExecMode,
    res: &RunResult,
    required_markers: &[&str],
) {
    let label = mode.label();
    assert_eq!(
        res.exit_code,
        Some(0),
        "{name} ({label}) failed (exit={:?}):\n{}",
        res.exit_code,
        res.log
    );
    if mode == ExecMode::PrewarmedAot {
        assert!(
            res.log.contains("emu aot:") && res.log.contains("blocks live"),
            "{name} ({label}) did not load prewarmed .aot blocks:\n{}",
            res.log
        );
    }
    for marker in required_markers {
        assert!(
            res.log.contains(marker),
            "{name} ({label}) missing marker {marker:?}:\n{}",
            res.log
        );
    }
    for forbidden in [
        "FAIL:",
        "PANIC",
        "UNIMPLEMENTED",
        "failed assertion",
    ] {
        assert!(
            !res.log.contains(forbidden),
            "{name} ({label}) hit forbidden marker {forbidden:?}:\n{}",
            res.log
        );
    }
}

pub fn run_guest_both_modes(
    name: &str,
    exe_path: &Path,
    work_dir: &Path,
    required_markers: &[&str],
    timeout: Duration,
) {
    let aot_dir = work_dir.join("aot-cache");

    let jit_res = run_guest(work_dir, exe_path, &aot_dir, ExecMode::LiveJit, timeout);
    assert_clean_run(name, ExecMode::LiveJit, &jit_res, required_markers);

    let _prewarm_log = prewarm_aot(work_dir, exe_path, &aot_dir);

    let aot_res = run_guest(work_dir, exe_path, &aot_dir, ExecMode::PrewarmedAot, timeout);
    assert_clean_run(name, ExecMode::PrewarmedAot, &aot_res, required_markers);
}
