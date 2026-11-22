//! Smoke tests against a user-supplied local game install. They are skipped
//! (pass with a note) when `PORT_GAME_DIR` / `AW2_GAME_DIR` is unset or missing,
//! so CI without game files stays green.
#![cfg(target_arch = "x86_64")]

use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn game_dir() -> Option<PathBuf> {
    let Some(d) = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
    else {
        eprintln!("skipping: set PORT_GAME_DIR or AW2_GAME_DIR to run game smoke tests");
        return None;
    };
    let has_exe = std::fs::read_dir(&d)
        .ok()?
        .filter_map(|e| e.ok())
        .any(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case("AlanWake2.exe"));
    if has_exe {
        Some(d)
    } else {
        eprintln!("skipping: no AlanWake2.exe in {}", d.display());
        None
    }
}

/// Run the loader with a private drive_c, killing it after `limit`.
fn run(dir: &PathBuf, extra: &[&str], limit: Duration) -> (Option<i32>, String) {
    let drive_c = std::env::temp_dir().join(format!("aw2-loader-test-{}", std::process::id()));
    std::fs::create_dir_all(&drive_c).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_aw2-loader"))
        .arg("--game-dir")
        .arg(dir)
        .args(extra)
        .env("AW2_DRIVE_C", &drive_c)
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn aw2-loader");
    let mut err = child.stderr.take().unwrap();
    let reader = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = err.read_to_string(&mut s);
        s
    });
    let t0 = Instant::now();
    let code = loop {
        if let Some(st) = child.try_wait().unwrap() {
            break st.code();
        }
        if t0.elapsed() > limit {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let log = reader.join().unwrap();
    let _ = std::fs::remove_dir_all(&drive_c);
    (code, log)
}

#[test]
fn maps_and_resolves_everything() {
    let Some(dir) = game_dir() else { return };
    let (code, log) = run(&dir, &["--no-run"], Duration::from_secs(60));
    assert_eq!(code, Some(0), "loader failed:\n{log}");
    assert!(log.contains("alanwake2.exe") || log.contains("AlanWake2.exe"), "{log}");
    assert!(log.contains("--no-run: not executing"), "{log}");
    // Summary line: "loaded N modules (P PE, B builtin) ...". A static import
    // of a missing PE DLL would abort the load before this point.
    assert!(log.contains("loaded ") && log.contains(" PE, "), "{log}");
}

/// Runs DLL init, TLS callbacks and the CRT startup. The process is expected
/// to stop at the first unimplemented import (currently user32 window
/// creation inside WinMain) or be killed by the timeout once it gets further.
#[test]
fn reaches_exe_entry_point() {
    let Some(dir) = game_dir() else { return };
    let (_code, log) = run(&dir, &[], Duration::from_secs(90));
    assert!(log.contains("calling exe entry point"), "{log}");
    assert!(!log.contains("FATAL"), "{log}");
}
