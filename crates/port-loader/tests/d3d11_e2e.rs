//! End-to-end integration tests that compile a real Windows x86-64 Direct3D 11
//! PE executable on the fly with `zig cc -target x86_64-windows-gnu` and run it
//! through `port-runner` on Apple Silicon (SM 5.0 DXBC -> `libdxilconv.dylib`
//! -> `libmetalirconverter.dylib` -> Metal 4), verifying:
//! - Win32 `HWND` creation (`RegisterClassExW` + `CreateWindowExW`) and message pump
//! - `D3D11CreateDeviceAndSwapChain` + COM `QueryInterface` chain (`ID3D11Device1`,
//!   `IDXGIDevice1`/`IDXGIDevice3`, `IDXGIAdapter`, `IDXGIFactory2`, `IDXGISurface2`)
//! - Dynamic VB (`Map(WRITE_DISCARD)`), CB (`UpdateSubresource`), depth-stencil
//!   state (`LESS`) + `ClearDepthStencilView` (`f32` Win64 ABI bridge), blend &
//!   rasterizer states, SM 5.0 VS+PS draw, `Present(0, 0)` & `Present(1, 0)`
//! - Backbuffer staging copy + CPU `Map(READ)` asserting exact RGBA8 rendered pixels
//! - Window resize (`SetWindowPos`) + `IDXGISwapChain::ResizeBuffers` (both explicit
//!   `320x200` and auto `0x0` -> `192x160`), re-fetching backbuffer + pixel readback
//! - SM 5.0 compute shader `Dispatch` + staging readback
//! - Both `PORT_HEADLESS=1` offscreen mode and real `NSWindow` + `CAMetalLayer` mode.
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
            "port-runner-d3d11-e2e-{tag}-{}-{id}",
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

fn has_dxilconv() -> bool {
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

fn has_metal_irconverter() -> bool {
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

fn check_prerequisites() -> Option<PathBuf> {
    let Some(zig) = find_zig() else {
        eprintln!("skipping d3d11_e2e: zig not found (install zig or set ZIG)");
        return None;
    };
    if !has_dxilconv() {
        eprintln!("skipping d3d11_e2e: libdxilconv.dylib not found (set AW2_DXILCONV or install to ~/.local/lib/)");
        return None;
    }
    if !has_metal_irconverter() {
        eprintln!("skipping d3d11_e2e: libmetalirconverter.dylib not found (set METAL_IRCONVERTER_DYLIB)");
        return None;
    }
    Some(zig)
}

fn compile_c_source(zig: &Path, work_dir: &Path, stem: &str, src: &str) -> PathBuf {
    let src_path = work_dir.join(format!("{stem}.c"));
    let exe_path = work_dir.join(format!("{stem}.exe"));
    std::fs::write(&src_path, src).expect("write guest C source");
    let zig_cache = work_dir.join("zig-cache");
    let output = Command::new(zig)
        .args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path)
        .args(["-ld3d11", "-ldxgi", "-luser32", "-lkernel32", "-luuid"])
        .env("ZIG_LOCAL_CACHE_DIR", &zig_cache)
        .output()
        .expect("spawn zig cc");
    assert!(
        output.status.success(),
        "zig cc failed (status={:?}):\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(exe_path.is_file(), "compiled PE missing at {}", exe_path.display());
    exe_path
}

fn compile_guest_exe(zig: &Path, work_dir: &Path) -> PathBuf {
    compile_c_source(zig, work_dir, "d3d11_e2e_guest", GUEST_C_SOURCE)
}

fn run_guest_in_port_runner(work_dir: &Path, exe_path: &Path, headless: bool) -> (Option<i32>, String, PathBuf) {
    let log_dir = work_dir.join("logs");
    let drive_c = work_dir.join("drive_c");
    let aot_dir = work_dir.join("aot");
    let ctl_dir = work_dir.join("ctl");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(&aot_dir).unwrap();
    std::fs::create_dir_all(&ctl_dir).unwrap();
    std::fs::write(ctl_dir.join("cmd.txt"), "shot d3d11_frame.png\n").expect("write ctl cmd.txt");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-runner"));
    cmd.arg("--pe")
        .arg(exe_path)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("PORT_CONTROL_DIR", &ctl_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if headless {
        cmd.env("PORT_HEADLESS", "1").env("AW2_HEADLESS", "1");
    } else {
        cmd.env_remove("PORT_HEADLESS").env_remove("AW2_HEADLESS");
    }

    let mut child = cmd.spawn().expect("spawn port-runner");
    let deadline = Instant::now() + Duration::from_secs(60);
    let code = loop {
        if let Some(st) = child.try_wait().expect("try_wait port-runner") {
            break st.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(25));
    };

    let out = child.wait_with_output().expect("collect child output");
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
    (code, combined, ctl_dir)
}

fn assert_control_channel_answered(ctl_dir: &Path, mode: &str, log: &str) {
    assert!(
        !ctl_dir.join("cmd.txt").exists(),
        "d3d11_e2e ({mode}): PORT_CONTROL_DIR cmd.txt was not consumed by PresentHook:\n{log}"
    );
    let status_path = ctl_dir.join("status.json");
    let status = std::fs::read_to_string(&status_path)
        .unwrap_or_else(|e| panic!("d3d11_e2e ({mode}): missing {}: {e}\n{log}", status_path.display()));
    assert!(
        status.contains("\"api\":\"d3d11\""),
        "d3d11_e2e ({mode}): status.json missing api=d3d11: {status}"
    );
    assert!(
        status.contains("\"cmds\":1"),
        "d3d11_e2e ({mode}): status.json did not record shot command: {status}"
    );
    let shot_path = ctl_dir.join("d3d11_frame.png");
    let png = std::fs::read(&shot_path)
        .unwrap_or_else(|e| panic!("d3d11_e2e ({mode}): missing screenshot {}: {e}\n{log}", shot_path.display()));
    assert!(
        png.len() > 24 && &png[..8] == b"\x89PNG\r\n\x1a\n" && &png[12..16] == b"IHDR",
        "d3d11_e2e ({mode}): invalid PNG header in {} ({} bytes)",
        shot_path.display(),
        png.len()
    );
    let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
    let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
    assert_eq!(
        (w, h),
        (256, 256),
        "d3d11_e2e ({mode}): unexpected PNG dimensions {w}x{h} (expected 256x256)"
    );
}

fn assert_guest_output_clean(code: Option<i32>, log: &str, mode: &str) {
    assert_eq!(code, Some(0), "d3d11_e2e ({mode}) failed (exit={code:?}):\n{log}");
    for marker in [
        "D3D11_E2E: window created",
        "D3D11_E2E: device+swapchain created",
        "D3D11_E2E: QI chain verified",
        "D3D11_E2E: 256x256 frame1 corner=(0,64,255,255) center=(0,255,0,255)",
        "D3D11_E2E: 256x256 frame2 corner=(0,64,255,255) center=(0,255,0,255)",
        "D3D11_E2E: 320x200 corner=(128,0,128,255) center=(255,255,0,255) far=(128,0,128,255)",
        "D3D11_E2E: auto-resize 192x160 verified",
        "D3D11_E2E: compute dispatch verified (64/64 float4 exact match)",
        "D3D11_E2E: PASS ALL",
    ] {
        assert!(
            log.contains(marker),
            "d3d11_e2e ({mode}) missing expected marker {marker:?}:\n{log}"
        );
    }
    for forbidden in [
        "FAIL:",
        "PANIC",
        "UNIMPLEMENTED",
        "format conversion not implemented",
        "MTLDebugComputeCommandEncoder",
        "MTLDebugRenderCommandEncoder",
        "failed assertion",
    ] {
        assert!(
            !log.contains(forbidden),
            "d3d11_e2e ({mode}) reported error {forbidden:?}:\n{log}"
        );
    }
}

#[test]
fn d3d11_e2e_headless_offscreen() {
    let Some(zig) = check_prerequisites() else {
        return;
    };
    let tmp = TempDir::new("headless");
    let exe = compile_guest_exe(&zig, tmp.path());
    let (code, log, ctl_dir) = run_guest_in_port_runner(tmp.path(), &exe, true);
    assert_guest_output_clean(code, &log, "headless");
    assert_control_channel_answered(&ctl_dir, "headless", &log);
    assert!(
        log.contains("swapchain: 256x256 DXGI_FORMAT_R8G8B8A8_UNORM") && log.contains("offscreen"),
        "expected offscreen swapchain log:\n{log}"
    );
}

#[test]
fn d3d11_e2e_windowed_nswindow_swapchain() {
    let Some(zig) = check_prerequisites() else {
        return;
    };
    let tmp = TempDir::new("windowed");
    let exe = compile_guest_exe(&zig, tmp.path());
    let (code, log, ctl_dir) = run_guest_in_port_runner(tmp.path(), &exe, false);
    assert_guest_output_clean(code, &log, "windowed");
    assert_control_channel_answered(&ctl_dir, "windowed", &log);
    assert!(
        log.contains("NSWindow for hwnd"),
        "windowed run did not create a real NSWindow:\n{log}"
    );
    assert!(
        log.contains("swapchain: 256x256 DXGI_FORMAT_R8G8B8A8_UNORM")
            && !log.contains("2 buffers, offscreen"),
        "windowed run unexpectedly fell back to offscreen swapchain:\n{log}"
    );
}

fn parse_u64_kv(line: &str, key: &str) -> Option<u64> {
    let prefix = format!("{key}=");
    line.split_whitespace()
        .find_map(|tok| tok.strip_prefix(&prefix)?.parse::<u64>().ok())
}

#[test]
fn d3d11_e2e_stress_4096_draws_map_discard() {
    let Some(zig) = check_prerequisites() else {
        return;
    };
    let tmp = TempDir::new("stress4096");
    let exe = compile_c_source(&zig, tmp.path(), "d3d11_stress_4096", STRESS_GUEST_C_SOURCE);
    let (code, log, _ctl_dir) = run_guest_in_port_runner(tmp.path(), &exe, true);
    let mut stress_frames = 0usize;
    for line in log.lines() {
        if line.contains("D3D11_STRESS:") || line.contains("d3d11 frame stats:") {
            eprintln!("{line}");
        }
        if line.contains("d3d11 frame stats:") {
            let draws = parse_u64_kv(line, "draws").unwrap_or(0);
            if draws == 4096 {
                stress_frames += 1;
                let flushes = parse_u64_kv(line, "flushes").expect("flushes kv");
                let mid_flushes = parse_u64_kv(line, "mid_frame_flushes").expect("mid_frame_flushes kv");
                let mid_stalls = parse_u64_kv(line, "mid_frame_stalls").expect("mid_frame_stalls kv");
                let samp_slots = parse_u64_kv(line, "samp_slots").expect("samp_slots kv");
                assert!(
                    flushes <= 1,
                    "4096-draw frame must flush at most once, got flushes={flushes}: {line}"
                );
                assert_eq!(
                    mid_flushes, 0,
                    "4096-draw frame must not mid-frame flush: {line}"
                );
                assert_eq!(
                    mid_stalls, 0,
                    "4096-draw frame must have 0 mid-frame wait_cpu stalls: {line}"
                );
                assert!(
                    samp_slots <= 16,
                    "4096-draw frame with static sampler should reuse sampler table, got samp_slots={samp_slots}: {line}"
                );
            }
        }
    }
    assert_eq!(code, Some(0), "d3d11_stress_4096 failed (exit={code:?}):\n{log}");
    assert!(
        log.contains("D3D11_STRESS: PASS"),
        "d3d11_stress_4096 missing PASS marker:\n{log}"
    );
    assert_eq!(
        stress_frames, 3,
        "expected 3 measured 4096-draw frame stat lines:\n{log}"
    );
    for forbidden in [
        "FAIL:",
        "PANIC",
        "UNIMPLEMENTED",
        "MTLDebugComputeCommandEncoder",
        "MTLDebugRenderCommandEncoder",
        "failed assertion",
    ] {
        assert!(
            !log.contains(forbidden),
            "d3d11_stress_4096 reported error {forbidden:?}:\n{log}"
        );
    }
}

const STRESS_GUEST_C_SOURCE: &str = r##"
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <d3d11.h>
#include <dxgi.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static const GUID MY_IID_ID3D11Texture2D =
    {0x6f15aaf2, 0xd208, 0x4e89, {0x9a, 0xb4, 0x48, 0x95, 0x35, 0xd3, 0x4f, 0x9c}};

/* vs_5_0: passthrough POSITION (v0.xyzw) -> SV_Position (o0.xyzw) */
static const uint8_t kVsPassthroughDxbc[216] = {
    0x44, 0x58, 0x42, 0x43, 0x9d, 0x3b, 0x28, 0xb4, 0xb7, 0xe9, 0x69, 0xf4,
    0x0e, 0x49, 0x0a, 0xe5, 0xae, 0x5e, 0xa2, 0x25, 0x01, 0x00, 0x00, 0x00,
    0xd8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x60, 0x00, 0x00, 0x00, 0x94, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00,
    0x50, 0x4f, 0x53, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x00, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x00, 0x53, 0x48, 0x45, 0x58, 0x3c, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x01, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x03,
    0xf2, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x00, 0x04,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x36, 0x00, 0x00, 0x05, 0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x46, 0x1e, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

/* ps_5_0: samples Texture2D t0 with sampler s0 at UV = cb0[0].xy -> SV_Target0 */
static const uint8_t kPsSampleTex2dCb0Dxbc[232] = {
    0x44, 0x58, 0x42, 0x43, 0xcd, 0xed, 0x6c, 0x7f, 0xbe, 0x8e, 0x5f, 0x53,
    0xd1, 0x0e, 0x5f, 0xd9, 0x1c, 0xd1, 0x10, 0x9e, 0x01, 0x00, 0x00, 0x00,
    0xe8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x70, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x03, 0x00, 0x60, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x58, 0x18, 0x00, 0x04, 0x00, 0x70, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0x00, 0x00, 0x65, 0x00, 0x00, 0x03,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x00, 0x00, 0x0a,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x8e, 0x20, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x7e, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x3e, 0x00, 0x00, 0x01,
};

#define CHECK_HR(hr, msg) do { \
    HRESULT _h = (hr); \
    if (FAILED(_h)) { \
        fprintf(stderr, "FAIL: %s (hr=0x%08lx)\n", (msg), (unsigned long)_h); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

#define CHECK_COND(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s\n", (msg)); \
        fflush(stderr); \
        ExitProcess(2); \
    } \
} while (0)

int main(void) {
    DXGI_SWAP_CHAIN_DESC scd;
    memset(&scd, 0, sizeof(scd));
    scd.BufferDesc.Width = 256;
    scd.BufferDesc.Height = 256;
    scd.BufferDesc.RefreshRate.Numerator = 60;
    scd.BufferDesc.RefreshRate.Denominator = 1;
    scd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    scd.SampleDesc.Count = 1;
    scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    scd.BufferCount = 2;
    scd.OutputWindow = NULL;
    scd.Windowed = TRUE;
    scd.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;

    IDXGISwapChain *sc = NULL;
    ID3D11Device *dev = NULL;
    ID3D11DeviceContext *ctx = NULL;
    D3D_FEATURE_LEVEL fl = 0;
    CHECK_HR(
        D3D11CreateDeviceAndSwapChain(
            NULL, D3D_DRIVER_TYPE_HARDWARE, NULL, 0, NULL, 0,
            D3D11_SDK_VERSION, &scd, &sc, &dev, &fl, &ctx
        ),
        "D3D11CreateDeviceAndSwapChain"
    );

    ID3D11Texture2D *backbuf = NULL;
    CHECK_HR(IDXGISwapChain_GetBuffer(sc, 0, &MY_IID_ID3D11Texture2D, (void **)&backbuf), "GetBuffer(0)");
    ID3D11RenderTargetView *rtv = NULL;
    CHECK_HR(ID3D11Device_CreateRenderTargetView(dev, (ID3D11Resource *)backbuf, NULL, &rtv), "CreateRTV");

    D3D11_TEXTURE2D_DESC stg_desc;
    memset(&stg_desc, 0, sizeof(stg_desc));
    stg_desc.Width = 256;
    stg_desc.Height = 256;
    stg_desc.MipLevels = 1;
    stg_desc.ArraySize = 1;
    stg_desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    stg_desc.SampleDesc.Count = 1;
    stg_desc.Usage = D3D11_USAGE_STAGING;
    stg_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    ID3D11Texture2D *staging = NULL;
    CHECK_HR(ID3D11Device_CreateTexture2D(dev, &stg_desc, NULL, &staging), "CreateTexture2D(staging)");

    /* 2x2 RGBA8 immutable SRV texture: (0,0)=Red, (1,0)=Green, (0,1)=Blue, (1,1)=Cyan */
    static const uint32_t kTexPixels[4] = {
        0xFF0000FFu, 0xFF00FF00u,
        0xFFFF0000u, 0xFFFFFF00u,
    };
    D3D11_TEXTURE2D_DESC tex_desc;
    memset(&tex_desc, 0, sizeof(tex_desc));
    tex_desc.Width = 2;
    tex_desc.Height = 2;
    tex_desc.MipLevels = 1;
    tex_desc.ArraySize = 1;
    tex_desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    tex_desc.SampleDesc.Count = 1;
    tex_desc.Usage = D3D11_USAGE_IMMUTABLE;
    tex_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
    D3D11_SUBRESOURCE_DATA tex_init = {kTexPixels, 8, 16};
    ID3D11Texture2D *srv_tex = NULL;
    CHECK_HR(ID3D11Device_CreateTexture2D(dev, &tex_desc, &tex_init, &srv_tex), "CreateTexture2D(srv)");
    ID3D11ShaderResourceView *srv = NULL;
    CHECK_HR(ID3D11Device_CreateShaderResourceView(dev, (ID3D11Resource *)srv_tex, NULL, &srv), "CreateSRV");

    D3D11_SAMPLER_DESC samp_desc;
    memset(&samp_desc, 0, sizeof(samp_desc));
    samp_desc.Filter = D3D11_FILTER_MIN_MAG_MIP_POINT;
    samp_desc.AddressU = D3D11_TEXTURE_ADDRESS_CLAMP;
    samp_desc.AddressV = D3D11_TEXTURE_ADDRESS_CLAMP;
    samp_desc.AddressW = D3D11_TEXTURE_ADDRESS_CLAMP;
    samp_desc.ComparisonFunc = D3D11_COMPARISON_NEVER;
    samp_desc.MaxLOD = 3.402823466e+38f;
    ID3D11SamplerState *samp = NULL;
    CHECK_HR(ID3D11Device_CreateSamplerState(dev, &samp_desc, &samp), "CreateSamplerState");

    D3D11_RASTERIZER_DESC rs_desc;
    memset(&rs_desc, 0, sizeof(rs_desc));
    rs_desc.FillMode = D3D11_FILL_SOLID;
    rs_desc.CullMode = D3D11_CULL_NONE;
    rs_desc.DepthClipEnable = TRUE;
    ID3D11RasterizerState *rs = NULL;
    CHECK_HR(ID3D11Device_CreateRasterizerState(dev, &rs_desc, &rs), "CreateRasterizerState");

    ID3D11VertexShader *vs = NULL;
    ID3D11PixelShader *ps = NULL;
    CHECK_HR(ID3D11Device_CreateVertexShader(dev, kVsPassthroughDxbc, sizeof(kVsPassthroughDxbc), NULL, &vs), "CreateVS");
    CHECK_HR(ID3D11Device_CreatePixelShader(dev, kPsSampleTex2dCb0Dxbc, sizeof(kPsSampleTex2dCb0Dxbc), NULL, &ps), "CreatePS");

    D3D11_INPUT_ELEMENT_DESC ied;
    memset(&ied, 0, sizeof(ied));
    ied.SemanticName = "POSITION";
    ied.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    ID3D11InputLayout *il = NULL;
    CHECK_HR(ID3D11Device_CreateInputLayout(dev, &ied, 1, kVsPassthroughDxbc, sizeof(kVsPassthroughDxbc), &il), "CreateIL");

    static const float kQuad[6][4] = {
        {-0.5f, -0.5f, 0.25f, 1.0f}, {-0.5f,  0.5f, 0.25f, 1.0f}, { 0.5f,  0.5f, 0.25f, 1.0f},
        {-0.5f, -0.5f, 0.25f, 1.0f}, { 0.5f,  0.5f, 0.25f, 1.0f}, { 0.5f, -0.5f, 0.25f, 1.0f},
    };
    D3D11_BUFFER_DESC vb_desc;
    memset(&vb_desc, 0, sizeof(vb_desc));
    vb_desc.ByteWidth = sizeof(kQuad);
    vb_desc.Usage = D3D11_USAGE_IMMUTABLE;
    vb_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    D3D11_SUBRESOURCE_DATA vb_init = {kQuad, sizeof(kQuad), sizeof(kQuad)};
    ID3D11Buffer *vb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &vb_desc, &vb_init, &vb), "CreateBuffer(VB)");

    D3D11_BUFFER_DESC cb_desc;
    memset(&cb_desc, 0, sizeof(cb_desc));
    cb_desc.ByteWidth = 16;
    cb_desc.Usage = D3D11_USAGE_DYNAMIC;
    cb_desc.BindFlags = D3D11_BIND_CONSTANT_BUFFER;
    cb_desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;
    ID3D11Buffer *dyn_cb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &cb_desc, NULL, &dyn_cb), "CreateBuffer(dyn CB)");

    const float clear_black[4] = {0.0f, 0.0f, 0.0f, 1.0f};
    const float uvs[4][4] = {
        {0.25f, 0.25f, 0.0f, 0.0f}, /* Red */
        {0.75f, 0.25f, 0.0f, 0.0f}, /* Green */
        {0.25f, 0.75f, 0.0f, 0.0f}, /* Blue */
        {0.75f, 0.75f, 0.0f, 0.0f}, /* Cyan */
    };
    UINT stride = 16, offset = 0;
    D3D11_VIEWPORT vp = {0.0f, 0.0f, 256.0f, 256.0f, 0.0f, 1.0f};

    LARGE_INTEGER freq, t0, t1, t2;
    QueryPerformanceFrequency(&freq);

    /* Frame 0: warmup (4 draws); Frames 1..3: 4096 draws each with Map(WRITE_DISCARD) per draw */
    for (int frame = 0; frame < 4; ++frame) {
        int ndraws = (frame == 0) ? 4 : 4096;
        ID3D11DeviceContext_ClearRenderTargetView(ctx, rtv, clear_black);
        ID3D11DeviceContext_OMSetRenderTargets(ctx, 1, &rtv, NULL);
        ID3D11DeviceContext_RSSetState(ctx, rs);
        ID3D11DeviceContext_RSSetViewports(ctx, 1, &vp);
        ID3D11DeviceContext_IASetInputLayout(ctx, il);
        ID3D11DeviceContext_IASetVertexBuffers(ctx, 0, 1, &vb, &stride, &offset);
        ID3D11DeviceContext_IASetPrimitiveTopology(ctx, D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        ID3D11DeviceContext_VSSetShader(ctx, vs, NULL, 0);
        ID3D11DeviceContext_PSSetShader(ctx, ps, NULL, 0);
        ID3D11DeviceContext_PSSetShaderResources(ctx, 0, 1, &srv);
        ID3D11DeviceContext_PSSetSamplers(ctx, 0, 1, &samp);
        ID3D11DeviceContext_PSSetConstantBuffers(ctx, 0, 1, &dyn_cb);

        QueryPerformanceCounter(&t0);
        for (int i = 0; i < ndraws; ++i) {
            D3D11_MAPPED_SUBRESOURCE mapped;
            CHECK_HR(
                ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)dyn_cb, 0, D3D11_MAP_WRITE_DISCARD, 0, &mapped),
                "Map(dyn_cb WRITE_DISCARD)"
            );
            int idx = (i == ndraws - 1) ? 3 : (i % 3);
            memcpy(mapped.pData, uvs[idx], 16);
            ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)dyn_cb, 0);
            ID3D11DeviceContext_Draw(ctx, 6, 0);
        }
        QueryPerformanceCounter(&t1);

        if (frame == 3) {
            ID3D11DeviceContext_CopyResource(ctx, (ID3D11Resource *)staging, (ID3D11Resource *)backbuf);
            D3D11_MAPPED_SUBRESOURCE rb;
            memset(&rb, 0, sizeof(rb));
            CHECK_HR(ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)staging, 0, D3D11_MAP_READ, 0, &rb), "Map(staging)");
            const uint8_t *px = (const uint8_t *)rb.pData + (size_t)128 * rb.RowPitch + (size_t)128 * 4;
            uint8_t r = px[0], g = px[1], b = px[2], a = px[3];
            ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)staging, 0);
            printf("D3D11_STRESS: center=(%u,%u,%u,%u)\n", r, g, b, a);
            CHECK_COND(r == 0 && g == 255 && b == 255 && a == 255, "final draw #4095 must render Cyan (0,255,255,255)");
        }

        HRESULT phr = IDXGISwapChain_Present(sc, 0, 0);
        CHECK_COND(phr == S_OK || phr == DXGI_STATUS_OCCLUDED, "Present");
        QueryPerformanceCounter(&t2);

        if (frame > 0) {
            double draw_ms = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
            double total_ms = (double)(t2.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
            double dpm = (draw_ms > 0.0) ? ((double)ndraws / draw_ms) : 0.0;
            printf("D3D11_STRESS: frame=%d draws=%d draw_ms=%.2f total_ms=%.2f draws_per_ms=%.2f\n",
                   frame, ndraws, draw_ms, total_ms, dpm);
            fflush(stdout);
        }
    }

    ID3D11DeviceContext_ClearState(ctx);
    ID3D11DeviceContext_Flush(ctx);
    ID3D11Buffer_Release(dyn_cb);
    ID3D11Buffer_Release(vb);
    ID3D11InputLayout_Release(il);
    ID3D11PixelShader_Release(ps);
    ID3D11VertexShader_Release(vs);
    ID3D11RasterizerState_Release(rs);
    ID3D11SamplerState_Release(samp);
    ID3D11ShaderResourceView_Release(srv);
    ID3D11Texture2D_Release(srv_tex);
    ID3D11Texture2D_Release(staging);
    ID3D11RenderTargetView_Release(rtv);
    ID3D11Texture2D_Release(backbuf);
    IDXGISwapChain_Release(sc);
    ID3D11DeviceContext_Release(ctx);
    ID3D11Device_Release(dev);

    printf("D3D11_STRESS: PASS\n");
    fflush(stdout);
    ExitProcess(0);
    return 0;
}
"##;

const GUEST_C_SOURCE: &str = r##"
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <d3d11.h>
#include <d3d11_1.h>
#include <dxgi.h>
#include <dxgi1_2.h>
#include <dxgi1_3.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* Explicit IIDs so linking never depends on missing mingw GUID symbols. */
static const GUID MY_IID_ID3D11Device =
    {0xdb6f6ddb, 0xac77, 0x4e88, {0x82, 0x53, 0x81, 0x9d, 0xf9, 0xbb, 0xf1, 0x40}};
static const GUID MY_IID_ID3D11Device1 =
    {0xa04bfb29, 0x08ef, 0x43d6, {0xa4, 0x9c, 0xa9, 0xbd, 0xbd, 0xcb, 0xe6, 0x86}};
static const GUID MY_IID_ID3D11Texture2D =
    {0x6f15aaf2, 0xd208, 0x4e89, {0x9a, 0xb4, 0x48, 0x95, 0x35, 0xd3, 0x4f, 0x9c}};
static const GUID MY_IID_IDXGIDevice1 =
    {0x77db970f, 0x6276, 0x48ba, {0xba, 0x28, 0x07, 0x01, 0x43, 0xb4, 0x39, 0x2c}};
static const GUID MY_IID_IDXGIDevice3 =
    {0x6007896c, 0x3244, 0x4afd, {0xbf, 0x18, 0xa6, 0xd3, 0xbe, 0xda, 0x50, 0x23}};
static const GUID MY_IID_IDXGIFactory2 =
    {0x50c83a1c, 0xe072, 0x4c48, {0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0}};
static const GUID MY_IID_IDXGISurface2 =
    {0xaba496dd, 0xb617, 0x4cb8, {0xa8, 0x66, 0xbc, 0x44, 0xd7, 0xeb, 0x1f, 0xa2}};

/* Valid signed SM 5.0 DXBC containers (matching crates/win32-rt/src/d3d11/mod.rs):
 * 1) vs_5_0: passthrough POSITION (v0.xyzw) -> SV_Position (o0.xyzw)
 * 2) ps_5_0: outputs cb0[0].xyzw -> SV_Target0
 * 3) cs_5_0: [numthreads(64,1,1)] writes u0[vThreadID.x] = cb0[0].xyzw
 */
static const uint8_t kVsPassthroughDxbc[216] = {
    0x44, 0x58, 0x42, 0x43, 0x9d, 0x3b, 0x28, 0xb4, 0xb7, 0xe9, 0x69, 0xf4,
    0x0e, 0x49, 0x0a, 0xe5, 0xae, 0x5e, 0xa2, 0x25, 0x01, 0x00, 0x00, 0x00,
    0xd8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x60, 0x00, 0x00, 0x00, 0x94, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00,
    0x50, 0x4f, 0x53, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x00, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x00, 0x53, 0x48, 0x45, 0x58, 0x3c, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x01, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x03,
    0xf2, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x00, 0x04,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x36, 0x00, 0x00, 0x05, 0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x46, 0x1e, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

static const uint8_t kPsCb0ColorDxbc[188] = {
    0x44, 0x58, 0x42, 0x43, 0xf4, 0x68, 0x60, 0xac, 0x53, 0x58, 0x0a, 0x02,
    0xf3, 0x2f, 0xb6, 0x13, 0xd5, 0x77, 0x32, 0x24, 0x01, 0x00, 0x00, 0x00,
    0xbc, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x44, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x03, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x06, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

static const uint8_t kCsWriteCb0Dxbc[184] = {
    0x44, 0x58, 0x42, 0x43, 0x3d, 0xf7, 0x0a, 0x03, 0xa5, 0xda, 0x5c, 0xa0,
    0x07, 0x84, 0xc9, 0x7e, 0x27, 0x46, 0xe9, 0x80, 0x01, 0x00, 0x00, 0x00,
    0xb8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x64, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x05, 0x00, 0x19, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x9c, 0x08, 0x00, 0x04, 0x00, 0xe0, 0x11, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x02,
    0x12, 0x00, 0x02, 0x00, 0x9b, 0x00, 0x00, 0x04, 0x40, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xa4, 0x00, 0x00, 0x07,
    0xf2, 0xe0, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00,
    0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x3e, 0x00, 0x00, 0x01,
};

#define CHECK_HR(hr, msg) do { \
    HRESULT _h = (hr); \
    if (FAILED(_h)) { \
        fprintf(stderr, "FAIL: %s (hr=0x%08lx)\n", (msg), (unsigned long)_h); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

#define CHECK_COND(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s\n", (msg)); \
        fflush(stderr); \
        ExitProcess(2); \
    } \
} while (0)

static int near_u8(uint8_t got, uint8_t expect, int tol) {
    int d = (int)got - (int)expect;
    if (d < 0) d = -d;
    return d <= tol;
}

static LRESULT CALLBACK wnd_proc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam) {
    if (msg == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcW(hwnd, msg, wparam, lparam);
}

static void pump_messages(void) {
    MSG msg;
    while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE)) {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

static void read_pixel(const D3D11_MAPPED_SUBRESOURCE *mapped, UINT x, UINT y,
                       uint8_t *r, uint8_t *g, uint8_t *b, uint8_t *a) {
    const uint8_t *row = (const uint8_t *)mapped->pData + (size_t)y * mapped->RowPitch;
    const uint8_t *px = row + (size_t)x * 4;
    *r = px[0];
    *g = px[1];
    *b = px[2];
    *a = px[3];
}

static void create_depth_and_staging(
    ID3D11Device *dev,
    UINT w,
    UINT h,
    ID3D11Texture2D **out_ds_tex,
    ID3D11DepthStencilView **out_dsv,
    ID3D11Texture2D **out_staging
) {
    D3D11_TEXTURE2D_DESC ds_desc;
    memset(&ds_desc, 0, sizeof(ds_desc));
    ds_desc.Width = w;
    ds_desc.Height = h;
    ds_desc.MipLevels = 1;
    ds_desc.ArraySize = 1;
    ds_desc.Format = DXGI_FORMAT_D32_FLOAT;
    ds_desc.SampleDesc.Count = 1;
    ds_desc.SampleDesc.Quality = 0;
    ds_desc.Usage = D3D11_USAGE_DEFAULT;
    ds_desc.BindFlags = D3D11_BIND_DEPTH_STENCIL;
    CHECK_HR(ID3D11Device_CreateTexture2D(dev, &ds_desc, NULL, out_ds_tex), "CreateTexture2D(depth)");
    CHECK_HR(ID3D11Device_CreateDepthStencilView(dev, (ID3D11Resource *)*out_ds_tex, NULL, out_dsv),
             "CreateDepthStencilView");

    D3D11_TEXTURE2D_DESC stg_desc;
    memset(&stg_desc, 0, sizeof(stg_desc));
    stg_desc.Width = w;
    stg_desc.Height = h;
    stg_desc.MipLevels = 1;
    stg_desc.ArraySize = 1;
    stg_desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    stg_desc.SampleDesc.Count = 1;
    stg_desc.SampleDesc.Quality = 0;
    stg_desc.Usage = D3D11_USAGE_STAGING;
    stg_desc.BindFlags = 0;
    stg_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    CHECK_HR(ID3D11Device_CreateTexture2D(dev, &stg_desc, NULL, out_staging), "CreateTexture2D(staging)");
}

int main(void) {
    HINSTANCE hinst = GetModuleHandleW(NULL);
    WNDCLASSEXW wc;
    memset(&wc, 0, sizeof(wc));
    wc.cbSize = sizeof(wc);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = wnd_proc;
    wc.hInstance = hinst;
    wc.lpszClassName = L"PortRunnerD3D11E2EClass";
    ATOM cls = RegisterClassExW(&wc);
    CHECK_COND(cls != 0, "RegisterClassExW");

    RECT rc = {0, 0, 256, 256};
    DWORD style = WS_OVERLAPPEDWINDOW;
    AdjustWindowRectEx(&rc, style, FALSE, 0);
    HWND hwnd = CreateWindowExW(
        0,
        wc.lpszClassName,
        L"D3D11 E2E Test",
        style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        rc.right - rc.left,
        rc.bottom - rc.top,
        NULL,
        NULL,
        hinst,
        NULL
    );
    CHECK_COND(hwnd != NULL, "CreateWindowExW");
    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);
    pump_messages();
    printf("D3D11_E2E: window created hwnd=%p\n", (void *)hwnd);
    fflush(stdout);

    /* 1. D3D11CreateDeviceAndSwapChain */
    DXGI_SWAP_CHAIN_DESC scd;
    memset(&scd, 0, sizeof(scd));
    scd.BufferDesc.Width = 256;
    scd.BufferDesc.Height = 256;
    scd.BufferDesc.RefreshRate.Numerator = 60;
    scd.BufferDesc.RefreshRate.Denominator = 1;
    scd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    scd.SampleDesc.Count = 1;
    scd.SampleDesc.Quality = 0;
    scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    scd.BufferCount = 2;
    scd.OutputWindow = hwnd;
    scd.Windowed = TRUE;
    scd.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
    scd.Flags = 0;

    IDXGISwapChain *sc = NULL;
    ID3D11Device *dev = NULL;
    ID3D11DeviceContext *ctx = NULL;
    D3D_FEATURE_LEVEL fl = 0;
    CHECK_HR(
        D3D11CreateDeviceAndSwapChain(
            NULL,
            D3D_DRIVER_TYPE_HARDWARE,
            NULL,
            0,
            NULL,
            0,
            D3D11_SDK_VERSION,
            &scd,
            &sc,
            &dev,
            &fl,
            &ctx
        ),
        "D3D11CreateDeviceAndSwapChain"
    );
    CHECK_COND(sc && dev && ctx, "device/swapchain/context non-null");
    CHECK_COND(fl >= D3D_FEATURE_LEVEL_11_0, "feature level >= 11_0");
    CHECK_COND(ID3D11Device_GetFeatureLevel(dev) == fl, "GetFeatureLevel matches");
    printf("D3D11_E2E: device+swapchain created FL=0x%x\n", (unsigned)fl);
    fflush(stdout);

    /* 2. QueryInterface chain: ID3D11Device1, IDXGIDevice1/3, IDXGIAdapter, IDXGIFactory2 */
    ID3D11Device1 *dev1 = NULL;
    CHECK_HR(ID3D11Device_QueryInterface(dev, &MY_IID_ID3D11Device1, (void **)&dev1), "QI ID3D11Device1");
    ID3D11DeviceContext1 *ctx1 = NULL;
    ID3D11Device1_GetImmediateContext1(dev1, &ctx1);
    CHECK_COND(ctx1 != NULL, "GetImmediateContext1");
    ID3D11DeviceContext1_Release(ctx1);
    ID3D11Device1_Release(dev1);

    IDXGIDevice1 *dxgi_dev = NULL;
    CHECK_HR(ID3D11Device_QueryInterface(dev, &MY_IID_IDXGIDevice1, (void **)&dxgi_dev), "QI IDXGIDevice1");
    CHECK_HR(IDXGIDevice1_SetMaximumFrameLatency(dxgi_dev, 2), "SetMaximumFrameLatency");
    UINT got_lat = 0;
    CHECK_HR(IDXGIDevice1_GetMaximumFrameLatency(dxgi_dev, &got_lat), "GetMaximumFrameLatency");
    CHECK_COND(got_lat == 2, "GetMaximumFrameLatency == 2");

    IDXGIDevice3 *dxgi_dev3 = NULL;
    CHECK_HR(ID3D11Device_QueryInterface(dev, &MY_IID_IDXGIDevice3, (void **)&dxgi_dev3), "QI IDXGIDevice3");
    CHECK_HR(IDXGIDevice3_OfferResources(dxgi_dev3, 0, NULL, DXGI_OFFER_RESOURCE_PRIORITY_NORMAL), "OfferResources");
    BOOL discarded = TRUE;
    CHECK_HR(IDXGIDevice3_ReclaimResources(dxgi_dev3, 1, NULL, &discarded), "ReclaimResources");
    CHECK_COND(discarded == FALSE, "ReclaimResources discarded == FALSE");
    HANDLE gpu_ev = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK_COND(gpu_ev != NULL, "CreateEventW");
    CHECK_HR(IDXGIDevice3_EnqueueSetEvent(dxgi_dev3, gpu_ev), "EnqueueSetEvent");
    CHECK_COND(WaitForSingleObject(gpu_ev, 5000) == WAIT_OBJECT_0, "EnqueueSetEvent signaled");
    CloseHandle(gpu_ev);
    IDXGIDevice3_Trim(dxgi_dev3);
    IDXGIDevice3_Release(dxgi_dev3);

    ID3D11Device *dev_roundtrip = NULL;
    CHECK_HR(IDXGIDevice1_QueryInterface(dxgi_dev, &MY_IID_ID3D11Device, (void **)&dev_roundtrip),
             "IDXGIDevice1 QI back to ID3D11Device");
    CHECK_COND(dev_roundtrip == dev, "COM identity preserved across IDXGIDevice1 -> ID3D11Device");
    ID3D11Device_Release(dev_roundtrip);

    IDXGIAdapter *adapter = NULL;
    CHECK_HR(IDXGIDevice1_GetAdapter(dxgi_dev, &adapter), "IDXGIDevice1::GetAdapter");
    DXGI_ADAPTER_DESC adp_desc;
    memset(&adp_desc, 0, sizeof(adp_desc));
    CHECK_HR(IDXGIAdapter_GetDesc(adapter, &adp_desc), "IDXGIAdapter::GetDesc");
    CHECK_COND(adp_desc.VendorId != 0, "IDXGIAdapter VendorId != 0");

    IDXGIFactory2 *factory2 = NULL;
    CHECK_HR(IDXGIAdapter_GetParent(adapter, &MY_IID_IDXGIFactory2, (void **)&factory2),
             "IDXGIAdapter::GetParent(IDXGIFactory2)");
    CHECK_COND(IDXGIFactory2_IsCurrent(factory2) == TRUE, "IDXGIFactory2::IsCurrent");
    IDXGIFactory2_Release(factory2);
    IDXGIAdapter_Release(adapter);
    IDXGIDevice1_Release(dxgi_dev);
    printf("D3D11_E2E: QI chain verified (VendorId=0x%04x)\n", adp_desc.VendorId);
    fflush(stdout);

    /* 3. Backbuffer + IDXGISurface2 check + RTV + DSV + Staging at 256x256 */
    ID3D11Texture2D *backbuf = NULL;
    CHECK_HR(IDXGISwapChain_GetBuffer(sc, 0, &MY_IID_ID3D11Texture2D, (void **)&backbuf), "GetBuffer(0)");
    D3D11_TEXTURE2D_DESC bb_desc;
    ID3D11Texture2D_GetDesc(backbuf, &bb_desc);
    CHECK_COND(bb_desc.Width == 256 && bb_desc.Height == 256 && bb_desc.Format == DXGI_FORMAT_R8G8B8A8_UNORM,
               "backbuffer 256x256 RGBA8");

    IDXGISurface2 *surf2 = NULL;
    CHECK_HR(ID3D11Texture2D_QueryInterface(backbuf, &MY_IID_IDXGISurface2, (void **)&surf2), "QI IDXGISurface2");
    DXGI_SURFACE_DESC sdesc;
    memset(&sdesc, 0, sizeof(sdesc));
    CHECK_HR(IDXGISurface2_GetDesc(surf2, &sdesc), "IDXGISurface2::GetDesc");
    CHECK_COND(sdesc.Width == 256 && sdesc.Height == 256 && sdesc.Format == DXGI_FORMAT_R8G8B8A8_UNORM,
               "IDXGISurface2 desc 256x256 RGBA8");
    ID3D11Texture2D *surf_parent = NULL;
    UINT surf_sub = 99;
    CHECK_HR(IDXGISurface2_GetResource(surf2, &MY_IID_ID3D11Texture2D, (void **)&surf_parent, &surf_sub),
             "IDXGISurface2::GetResource");
    CHECK_COND(surf_parent == backbuf && surf_sub == 0, "IDXGISurface2::GetResource identity");
    ID3D11Texture2D_Release(surf_parent);
    IDXGISurface2_Release(surf2);

    ID3D11RenderTargetView *rtv = NULL;
    CHECK_HR(ID3D11Device_CreateRenderTargetView(dev, (ID3D11Resource *)backbuf, NULL, &rtv),
             "CreateRenderTargetView");

    ID3D11Texture2D *ds_tex = NULL;
    ID3D11DepthStencilView *dsv = NULL;
    ID3D11Texture2D *staging = NULL;
    create_depth_and_staging(dev, 256, 256, &ds_tex, &dsv, &staging);

    /* 4. Pipeline state objects & shaders */
    D3D11_DEPTH_STENCIL_DESC dss_desc;
    memset(&dss_desc, 0, sizeof(dss_desc));
    dss_desc.DepthEnable = TRUE;
    dss_desc.DepthWriteMask = D3D11_DEPTH_WRITE_MASK_ALL;
    dss_desc.DepthFunc = D3D11_COMPARISON_LESS;
    dss_desc.StencilEnable = FALSE;
    dss_desc.StencilReadMask = 0xFF;
    dss_desc.StencilWriteMask = 0xFF;
    ID3D11DepthStencilState *dss = NULL;
    CHECK_HR(ID3D11Device_CreateDepthStencilState(dev, &dss_desc, &dss), "CreateDepthStencilState");

    D3D11_RASTERIZER_DESC rs_desc;
    memset(&rs_desc, 0, sizeof(rs_desc));
    rs_desc.FillMode = D3D11_FILL_SOLID;
    rs_desc.CullMode = D3D11_CULL_NONE;
    rs_desc.DepthClipEnable = TRUE;
    ID3D11RasterizerState *rs = NULL;
    CHECK_HR(ID3D11Device_CreateRasterizerState(dev, &rs_desc, &rs), "CreateRasterizerState");

    D3D11_BLEND_DESC bs_desc;
    memset(&bs_desc, 0, sizeof(bs_desc));
    bs_desc.RenderTarget[0].BlendEnable = FALSE;
    bs_desc.RenderTarget[0].SrcBlend = D3D11_BLEND_ONE;
    bs_desc.RenderTarget[0].DestBlend = D3D11_BLEND_ZERO;
    bs_desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    bs_desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
    bs_desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
    bs_desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    bs_desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL;
    ID3D11BlendState *bs = NULL;
    CHECK_HR(ID3D11Device_CreateBlendState(dev, &bs_desc, &bs), "CreateBlendState");

    ID3D11VertexShader *vs = NULL;
    ID3D11PixelShader *ps = NULL;
    CHECK_HR(ID3D11Device_CreateVertexShader(dev, kVsPassthroughDxbc, sizeof(kVsPassthroughDxbc), NULL, &vs),
             "CreateVertexShader");
    CHECK_HR(ID3D11Device_CreatePixelShader(dev, kPsCb0ColorDxbc, sizeof(kPsCb0ColorDxbc), NULL, &ps),
             "CreatePixelShader");

    D3D11_INPUT_ELEMENT_DESC ied;
    memset(&ied, 0, sizeof(ied));
    ied.SemanticName = "POSITION";
    ied.SemanticIndex = 0;
    ied.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    ied.InputSlot = 0;
    ied.AlignedByteOffset = 0;
    ied.InputSlotClass = D3D11_INPUT_PER_VERTEX_DATA;
    ied.InstanceDataStepRate = 0;
    ID3D11InputLayout *il = NULL;
    CHECK_HR(ID3D11Device_CreateInputLayout(dev, &ied, 1, kVsPassthroughDxbc, sizeof(kVsPassthroughDxbc), &il),
             "CreateInputLayout");

    /* Dynamic vertex buffer (Map WRITE_DISCARD + Unmap):
     * Vertices 0..5: center quad [-0.5..0.5]^2 at z = 0.25 (near)
     * Vertices 6..11: center quad [-0.5..0.5]^2 at z = 0.75 (far, must fail LESS depth test)
     */
    static const float kVerts[12][4] = {
        {-0.5f, -0.5f, 0.25f, 1.0f}, {-0.5f,  0.5f, 0.25f, 1.0f}, { 0.5f,  0.5f, 0.25f, 1.0f},
        {-0.5f, -0.5f, 0.25f, 1.0f}, { 0.5f,  0.5f, 0.25f, 1.0f}, { 0.5f, -0.5f, 0.25f, 1.0f},
        {-0.5f, -0.5f, 0.75f, 1.0f}, {-0.5f,  0.5f, 0.75f, 1.0f}, { 0.5f,  0.5f, 0.75f, 1.0f},
        {-0.5f, -0.5f, 0.75f, 1.0f}, { 0.5f,  0.5f, 0.75f, 1.0f}, { 0.5f, -0.5f, 0.75f, 1.0f},
    };
    D3D11_BUFFER_DESC vb_desc;
    memset(&vb_desc, 0, sizeof(vb_desc));
    vb_desc.ByteWidth = sizeof(kVerts);
    vb_desc.Usage = D3D11_USAGE_DYNAMIC;
    vb_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    vb_desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;
    ID3D11Buffer *vb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &vb_desc, NULL, &vb), "CreateBuffer(dynamic VB)");

    D3D11_MAPPED_SUBRESOURCE vb_map;
    memset(&vb_map, 0, sizeof(vb_map));
    CHECK_HR(ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)vb, 0, D3D11_MAP_WRITE_DISCARD, 0, &vb_map),
             "Map(VB WRITE_DISCARD)");
    CHECK_COND(vb_map.pData != NULL, "VB mapped ptr non-null");
    memcpy(vb_map.pData, kVerts, sizeof(kVerts));
    ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)vb, 0);

    D3D11_BUFFER_DESC cb_desc;
    memset(&cb_desc, 0, sizeof(cb_desc));
    cb_desc.ByteWidth = 16;
    cb_desc.Usage = D3D11_USAGE_DEFAULT;
    cb_desc.BindFlags = D3D11_BIND_CONSTANT_BUFFER;
    ID3D11Buffer *cb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &cb_desc, NULL, &cb), "CreateBuffer(CB)");

    /* 5. Render 3 frames at 256x256 (verifying BOTH odd and even rotated backbuffers) */
    const float clear_blue[4] = {0.0f, 0.25f, 1.0f, 1.0f};
    const float color_green[4] = {0.0f, 1.0f, 0.0f, 1.0f};
    const float color_red[4] = {1.0f, 0.0f, 0.0f, 1.0f};
    const float blend_factor[4] = {1.0f, 1.0f, 1.0f, 1.0f};
    UINT stride = 16, offset = 0;

    for (int frame = 0; frame < 3; ++frame) {
        pump_messages();
        ID3D11DeviceContext_ClearRenderTargetView(ctx, rtv, clear_blue);
        ID3D11DeviceContext_ClearDepthStencilView(ctx, dsv, D3D11_CLEAR_DEPTH, 1.0f, 0);
        ID3D11DeviceContext_OMSetRenderTargets(ctx, 1, &rtv, dsv);
        ID3D11DeviceContext_OMSetDepthStencilState(ctx, dss, 0);
        ID3D11DeviceContext_OMSetBlendState(ctx, bs, blend_factor, 0xFFFFFFFF);
        ID3D11DeviceContext_RSSetState(ctx, rs);
        D3D11_VIEWPORT vp = {0.0f, 0.0f, 256.0f, 256.0f, 0.0f, 1.0f};
        ID3D11DeviceContext_RSSetViewports(ctx, 1, &vp);
        ID3D11DeviceContext_IASetInputLayout(ctx, il);
        ID3D11DeviceContext_IASetVertexBuffers(ctx, 0, 1, &vb, &stride, &offset);
        ID3D11DeviceContext_IASetPrimitiveTopology(ctx, D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        ID3D11DeviceContext_VSSetShader(ctx, vs, NULL, 0);
        ID3D11DeviceContext_PSSetShader(ctx, ps, NULL, 0);
        ID3D11DeviceContext_PSSetConstantBuffers(ctx, 0, 1, &cb);

        /* Near quad (z=0.25) in green (also tests UpdateSubresource with 0,0 pitches) */
        ID3D11DeviceContext_UpdateSubresource(ctx, (ID3D11Resource *)cb, 0, NULL, color_green, 0, 0);
        ID3D11DeviceContext_Draw(ctx, 6, 0);

        /* Far quad (z=0.75) in red -> rejected by LESS depth test */
        ID3D11DeviceContext_UpdateSubresource(ctx, (ID3D11Resource *)cb, 0, NULL, color_red, 0, 0);
        ID3D11DeviceContext_Draw(ctx, 6, 6);

        if (frame == 1 || frame == 2) {
            ID3D11DeviceContext_CopyResource(ctx, (ID3D11Resource *)staging, (ID3D11Resource *)backbuf);
            D3D11_MAPPED_SUBRESOURCE rb_map;
            memset(&rb_map, 0, sizeof(rb_map));
            CHECK_HR(ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)staging, 0, D3D11_MAP_READ, 0, &rb_map),
                     "Map(staging256 READ)");
            CHECK_COND(rb_map.RowPitch >= 256 * 4, "staging256 RowPitch >= 1024");
            uint8_t br, bg, bb, ba, cr, cg, cb_px, ca;
            read_pixel(&rb_map, 8, 8, &br, &bg, &bb, &ba);
            read_pixel(&rb_map, 128, 128, &cr, &cg, &cb_px, &ca);
            ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)staging, 0);

            printf("D3D11_E2E: 256x256 frame%d corner=(%u,%u,%u,%u) center=(%u,%u,%u,%u)\n",
                   frame, br, bg, bb, ba, cr, cg, cb_px, ca);
            fflush(stdout);
            CHECK_COND(br == 0 && near_u8(bg, 64, 1) && bb == 255 && ba == 255,
                       "256x256 corner pixel == clear blue");
            CHECK_COND(cr == 0 && cg == 255 && cb_px == 0 && ca == 255,
                       "256x256 center pixel == near green (depth test rejected far red)");
        }

        /* Exercise both sync=0 async present and sync=1 vsync present */
        UINT sync_interval = (frame == 2) ? 1 : 0;
        HRESULT phr = IDXGISwapChain_Present(sc, sync_interval, 0);
        CHECK_COND(phr == S_OK || phr == DXGI_STATUS_OCCLUDED, "Present(256x256)");
    }

    /* 6. Window resize + ResizeBuffers to 320x200 */
    ID3D11DeviceContext_OMSetRenderTargets(ctx, 0, NULL, NULL);
    ID3D11RenderTargetView_Release(rtv); rtv = NULL;
    ID3D11DepthStencilView_Release(dsv); dsv = NULL;
    ID3D11Texture2D_Release(ds_tex); ds_tex = NULL;
    ID3D11Texture2D_Release(staging); staging = NULL;
    ID3D11Texture2D_Release(backbuf); backbuf = NULL;
    ID3D11DeviceContext_Flush(ctx);

    RECT rc2 = {0, 0, 320, 200};
    AdjustWindowRectEx(&rc2, style, FALSE, 0);
    SetWindowPos(hwnd, NULL, 0, 0, rc2.right - rc2.left, rc2.bottom - rc2.top,
                 SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE);
    pump_messages();

    CHECK_HR(IDXGISwapChain_ResizeBuffers(sc, 0, 320, 200, DXGI_FORMAT_UNKNOWN, 0),
             "ResizeBuffers(0, 320, 200, UNKNOWN, 0)");

    DXGI_SWAP_CHAIN_DESC scd_after;
    memset(&scd_after, 0, sizeof(scd_after));
    CHECK_HR(IDXGISwapChain_GetDesc(sc, &scd_after), "GetDesc after ResizeBuffers");
    CHECK_COND(scd_after.BufferDesc.Width == 320 && scd_after.BufferDesc.Height == 200
                   && scd_after.BufferDesc.Format == DXGI_FORMAT_R8G8B8A8_UNORM,
               "SwapChain GetDesc reports 320x200 RGBA8");

    CHECK_HR(IDXGISwapChain_GetBuffer(sc, 0, &MY_IID_ID3D11Texture2D, (void **)&backbuf),
             "GetBuffer(0) after ResizeBuffers");
    ID3D11Texture2D_GetDesc(backbuf, &bb_desc);
    CHECK_COND(bb_desc.Width == 320 && bb_desc.Height == 200 && bb_desc.Format == DXGI_FORMAT_R8G8B8A8_UNORM,
               "resized backbuffer GetDesc reports 320x200 RGBA8");

    CHECK_HR(ID3D11Device_CreateRenderTargetView(dev, (ID3D11Resource *)backbuf, NULL, &rtv),
             "CreateRenderTargetView(320x200)");
    create_depth_and_staging(dev, 320, 200, &ds_tex, &dsv, &staging);

    const float clear_purple[4] = {0.5f, 0.0f, 0.5f, 1.0f};
    const float color_yellow[4] = {1.0f, 1.0f, 0.0f, 1.0f};

    for (int frame = 0; frame < 3; ++frame) {
        pump_messages();
        ID3D11DeviceContext_ClearRenderTargetView(ctx, rtv, clear_purple);
        /* Clear depth to 0.5f to also verify non-1.0 float depth clear over Win64 ABI */
        ID3D11DeviceContext_ClearDepthStencilView(ctx, dsv, D3D11_CLEAR_DEPTH, 0.5f, 0);
        ID3D11DeviceContext_OMSetRenderTargets(ctx, 1, &rtv, dsv);
        ID3D11DeviceContext_OMSetDepthStencilState(ctx, dss, 0);
        ID3D11DeviceContext_OMSetBlendState(ctx, bs, blend_factor, 0xFFFFFFFF);
        ID3D11DeviceContext_RSSetState(ctx, rs);
        D3D11_VIEWPORT vp = {0.0f, 0.0f, 320.0f, 200.0f, 0.0f, 1.0f};
        ID3D11DeviceContext_RSSetViewports(ctx, 1, &vp);
        ID3D11DeviceContext_IASetInputLayout(ctx, il);
        ID3D11DeviceContext_IASetVertexBuffers(ctx, 0, 1, &vb, &stride, &offset);
        ID3D11DeviceContext_IASetPrimitiveTopology(ctx, D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        ID3D11DeviceContext_VSSetShader(ctx, vs, NULL, 0);
        ID3D11DeviceContext_PSSetShader(ctx, ps, NULL, 0);
        ID3D11DeviceContext_PSSetConstantBuffers(ctx, 0, 1, &cb);

        /* Near quad (z=0.25 < 0.5) passes -> yellow */
        ID3D11DeviceContext_UpdateSubresource(ctx, (ID3D11Resource *)cb, 0, NULL, color_yellow, 0, 0);
        ID3D11DeviceContext_Draw(ctx, 6, 0);
        /* Far quad (z=0.75 > 0.25) rejected */
        ID3D11DeviceContext_UpdateSubresource(ctx, (ID3D11Resource *)cb, 0, NULL, color_red, 0, 0);
        ID3D11DeviceContext_Draw(ctx, 6, 6);

        if (frame == 2) {
            ID3D11DeviceContext_CopyResource(ctx, (ID3D11Resource *)staging, (ID3D11Resource *)backbuf);
            D3D11_MAPPED_SUBRESOURCE rb_map;
            memset(&rb_map, 0, sizeof(rb_map));
            CHECK_HR(ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)staging, 0, D3D11_MAP_READ, 0, &rb_map),
                     "Map(staging320 READ)");
            CHECK_COND(rb_map.RowPitch >= 320 * 4, "staging320 RowPitch >= 1280");
            uint8_t br, bg, bb, ba, cr, cg, cb_px, ca, fr, fg, fb, fa;
            read_pixel(&rb_map, 8, 8, &br, &bg, &bb, &ba);
            read_pixel(&rb_map, 160, 100, &cr, &cg, &cb_px, &ca);
            read_pixel(&rb_map, 310, 190, &fr, &fg, &fb, &fa);
            ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)staging, 0);

            printf("D3D11_E2E: 320x200 corner=(%u,%u,%u,%u) center=(%u,%u,%u,%u) far=(%u,%u,%u,%u)\n",
                   br, bg, bb, ba, cr, cg, cb_px, ca, fr, fg, fb, fa);
            fflush(stdout);
            CHECK_COND(near_u8(br, 128, 1) && bg == 0 && near_u8(bb, 128, 1) && ba == 255,
                       "320x200 corner pixel == clear purple");
            CHECK_COND(cr == 255 && cg == 255 && cb_px == 0 && ca == 255,
                       "320x200 center pixel == near yellow");
            CHECK_COND(near_u8(fr, 128, 1) && fg == 0 && near_u8(fb, 128, 1) && fa == 255,
                       "320x200 far-right pixel (310,190) == clear purple");
        }

        HRESULT phr = IDXGISwapChain_Present(sc, 0, 0);
        CHECK_COND(phr == S_OK || phr == DXGI_STATUS_OCCLUDED, "Present(320x200)");
    }

    /* 6b. Auto-size ResizeBuffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0) tracks SetWindowPos client size */
    ID3D11DeviceContext_OMSetRenderTargets(ctx, 0, NULL, NULL);
    ID3D11RenderTargetView_Release(rtv); rtv = NULL;
    ID3D11DepthStencilView_Release(dsv); dsv = NULL;
    ID3D11Texture2D_Release(ds_tex); ds_tex = NULL;
    ID3D11Texture2D_Release(staging); staging = NULL;
    ID3D11Texture2D_Release(backbuf); backbuf = NULL;
    ID3D11DeviceContext_Flush(ctx);

    RECT rc3 = {0, 0, 192, 160};
    AdjustWindowRectEx(&rc3, style, FALSE, 0);
    SetWindowPos(hwnd, NULL, 0, 0, rc3.right - rc3.left, rc3.bottom - rc3.top,
                 SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE);
    pump_messages();

    CHECK_HR(IDXGISwapChain_ResizeBuffers(sc, 0, 0, 0, DXGI_FORMAT_UNKNOWN, 0),
             "ResizeBuffers(0, 0, 0, UNKNOWN, 0)");
    memset(&scd_after, 0, sizeof(scd_after));
    CHECK_HR(IDXGISwapChain_GetDesc(sc, &scd_after), "GetDesc after auto ResizeBuffers");
    CHECK_COND(scd_after.BufferDesc.Width == 192 && scd_after.BufferDesc.Height == 160,
               "auto ResizeBuffers(0, 0, 0) picked up 192x160 HWND client size");
    printf("D3D11_E2E: auto-resize 192x160 verified\n");
    fflush(stdout);

    /* 7. SM 5.0 Compute shader dispatch + staging readback */
    ID3D11ComputeShader *cs = NULL;
    CHECK_HR(ID3D11Device_CreateComputeShader(dev, kCsWriteCb0Dxbc, sizeof(kCsWriteCb0Dxbc), NULL, &cs),
             "CreateComputeShader");

    const float cs_vals[4] = {0.25f, 0.5f, 0.75f, 1.0f};
    D3D11_SUBRESOURCE_DATA cs_cb_init = {cs_vals, 16, 16};
    ID3D11Buffer *cs_cb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &cb_desc, &cs_cb_init, &cs_cb), "CreateBuffer(CS CB)");

    D3D11_BUFFER_DESC uav_buf_desc;
    memset(&uav_buf_desc, 0, sizeof(uav_buf_desc));
    uav_buf_desc.ByteWidth = 64 * 16;
    uav_buf_desc.Usage = D3D11_USAGE_DEFAULT;
    uav_buf_desc.BindFlags = D3D11_BIND_UNORDERED_ACCESS;
    ID3D11Buffer *cs_out = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &uav_buf_desc, NULL, &cs_out), "CreateBuffer(CS UAV out)");

    D3D11_BUFFER_DESC rb_buf_desc;
    memset(&rb_buf_desc, 0, sizeof(rb_buf_desc));
    rb_buf_desc.ByteWidth = 64 * 16;
    rb_buf_desc.Usage = D3D11_USAGE_STAGING;
    rb_buf_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
    ID3D11Buffer *cs_rb = NULL;
    CHECK_HR(ID3D11Device_CreateBuffer(dev, &rb_buf_desc, NULL, &cs_rb), "CreateBuffer(CS staging)");

    D3D11_UNORDERED_ACCESS_VIEW_DESC uav_desc;
    memset(&uav_desc, 0, sizeof(uav_desc));
    uav_desc.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    uav_desc.ViewDimension = D3D11_UAV_DIMENSION_BUFFER;
    uav_desc.Buffer.FirstElement = 0;
    uav_desc.Buffer.NumElements = 64;
    ID3D11UnorderedAccessView *cs_uav = NULL;
    CHECK_HR(ID3D11Device_CreateUnorderedAccessView(dev, (ID3D11Resource *)cs_out, &uav_desc, &cs_uav),
             "CreateUnorderedAccessView");

    ID3D11DeviceContext_CSSetShader(ctx, cs, NULL, 0);
    ID3D11DeviceContext_CSSetConstantBuffers(ctx, 0, 1, &cs_cb);
    ID3D11DeviceContext_CSSetUnorderedAccessViews(ctx, 0, 1, &cs_uav, NULL);
    ID3D11DeviceContext_Dispatch(ctx, 1, 1, 1);

    ID3D11DeviceContext_CopyResource(ctx, (ID3D11Resource *)cs_rb, (ID3D11Resource *)cs_out);
    D3D11_MAPPED_SUBRESOURCE cs_map;
    memset(&cs_map, 0, sizeof(cs_map));
    CHECK_HR(ID3D11DeviceContext_Map(ctx, (ID3D11Resource *)cs_rb, 0, D3D11_MAP_READ, 0, &cs_map),
             "Map(CS staging READ)");
    const float *got_f = (const float *)cs_map.pData;
    for (int i = 0; i < 64; ++i) {
        CHECK_COND(got_f[i * 4 + 0] == cs_vals[0]
                       && got_f[i * 4 + 1] == cs_vals[1]
                       && got_f[i * 4 + 2] == cs_vals[2]
                       && got_f[i * 4 + 3] == cs_vals[3],
                   "CS output float4 matches cb0[0]");
    }
    ID3D11DeviceContext_Unmap(ctx, (ID3D11Resource *)cs_rb, 0);
    printf("D3D11_E2E: compute dispatch verified (64/64 float4 exact match)\n");
    fflush(stdout);

    /* 8. Clean COM Release() teardown */
    ID3D11DeviceContext_ClearState(ctx);
    ID3D11DeviceContext_Flush(ctx);

    ID3D11UnorderedAccessView_Release(cs_uav);
    ID3D11Buffer_Release(cs_rb);
    ID3D11Buffer_Release(cs_out);
    ID3D11Buffer_Release(cs_cb);
    ID3D11ComputeShader_Release(cs);

    ID3D11Buffer_Release(cb);
    ID3D11Buffer_Release(vb);
    ID3D11InputLayout_Release(il);
    ID3D11PixelShader_Release(ps);
    ID3D11VertexShader_Release(vs);
    ID3D11BlendState_Release(bs);
    ID3D11RasterizerState_Release(rs);
    ID3D11DepthStencilState_Release(dss);

    IDXGISwapChain_Release(sc);
    ID3D11DeviceContext_Release(ctx);
    ID3D11Device_Release(dev);

    DestroyWindow(hwnd);
    pump_messages();

    printf("D3D11_E2E: PASS ALL\n");
    fflush(stdout);
    ExitProcess(0);
    return 0;
}
"##;
