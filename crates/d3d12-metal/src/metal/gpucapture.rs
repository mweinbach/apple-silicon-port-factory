//! Programmatic Metal GPU capture of DXGI frames (diagnostics only).
//!
//! `AW2_GPU_CAPTURE=/abs/path/out.gputrace` captures `AW2_GPU_CAPTURE_FRAMES`
//! (default 1) frames starting at DXGI present number `AW2_GPU_CAPTURE_AT`
//! (default 600) into a `.gputrace` document (open it in Xcode). Device
//! scope, so every queue (render, compute, present) is included. One-shot.
//! Off (one `OnceLock` read per present) when unset.

use objc2::runtime::AnyObject;
use objc2_foundation::{NSString, NSURL};
use objc2_metal::*;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

struct Config {
    path: String,
    at: u64,
    frames: u64,
}

fn config() -> Option<&'static Config> {
    static C: OnceLock<Option<Config>> = OnceLock::new();
    C.get_or_init(|| {
        let path = std::env::var("AW2_GPU_CAPTURE").ok().filter(|p| !p.is_empty())?;
        let num = |k: &str, d: u64| std::env::var(k).ok().and_then(|v| v.trim().parse().ok()).unwrap_or(d);
        Some(Config { path, at: num("AW2_GPU_CAPTURE_AT", 600).max(2), frames: num("AW2_GPU_CAPTURE_FRAMES", 1).max(1) })
    })
    .as_ref()
}

/// True when a capture is configured (pipelines then carry shader-key labels).
pub fn enabled() -> bool {
    config().is_some()
}

/// Call before the first `MTLCreateSystemDefaultDevice`: outside Xcode,
/// Metal only allows `GPUTraceDocument` captures with `MTL_CAPTURE_ENABLED=1`.
pub fn pre_device_init() {
    if enabled() && std::env::var_os("MTL_CAPTURE_ENABLED").is_none() {
        std::env::set_var("MTL_CAPTURE_ENABLED", "1");
    }
}

const IDLE: u8 = 0;
const RUNNING: u8 = 1;
const DONE: u8 = 2;
static STATE: AtomicU8 = AtomicU8::new(IDLE);

/// Called after DXGI present number `n` (1-based) has been committed. The
/// capture starts after present `at - 1` so frame `at`'s command buffers are
/// all created and committed inside it, and stops after present
/// `at + frames - 1` (Metal writes the document on stop).
pub fn on_present(n: u64) {
    let Some(c) = config() else { return };
    match STATE.load(Ordering::Relaxed) {
        IDLE if n == c.at - 1 => {
            let ok = start(c);
            STATE.store(if ok { RUNNING } else { DONE }, Ordering::Release);
        }
        IDLE if n >= c.at => {
            STATE.store(DONE, Ordering::Release);
            log::warn!(target: "d3d", "gpu capture: present #{n} is past AW2_GPU_CAPTURE_AT={}; not capturing", c.at);
        }
        RUNNING if n >= c.at + c.frames - 1 => {
            STATE.store(DONE, Ordering::Release);
            objc2::rc::autoreleasepool(|_| {
                let m = unsafe { MTLCaptureManager::sharedCaptureManager() };
                let t = std::time::Instant::now();
                m.stopCapture();
                log::info!(target: "d3d", "gpu capture: stopped after present #{n} ({} frame(s)) in {:.1}s -> {}", c.frames, t.elapsed().as_secs_f64(), c.path);
            });
        }
        _ => {}
    }
}

fn start(c: &Config) -> bool {
    let Some(d) = super::device() else {
        log::warn!(target: "d3d", "gpu capture: no Metal device");
        return false;
    };
    objc2::rc::autoreleasepool(|_| {
        let m = unsafe { MTLCaptureManager::sharedCaptureManager() };
        if !m.supportsDestination(MTLCaptureDestination::GPUTraceDocument) {
            log::warn!(
                target: "d3d",
                "gpu capture: GPUTraceDocument unsupported (launch with MTL_CAPTURE_ENABLED=1, or MetalCaptureEnabled=true in the app Info.plist)"
            );
            return false;
        }
        if std::path::Path::new(&c.path).exists() {
            log::warn!(target: "d3d", "gpu capture: {} already exists; Metal will not overwrite it", c.path);
            return false;
        }
        let desc = MTLCaptureDescriptor::new();
        let obj: &AnyObject = (*d.dev).as_ref();
        unsafe { desc.setCaptureObject(Some(obj)) };
        desc.setDestination(MTLCaptureDestination::GPUTraceDocument);
        let url = NSURL::fileURLWithPath(&NSString::from_str(&c.path));
        desc.setOutputURL(Some(&url));
        match m.startCaptureWithDescriptor_error(&desc) {
            Ok(()) => {
                log::info!(target: "d3d", "gpu capture: started before frame #{} ({} frame(s)) -> {}", c.at, c.frames, c.path);
                true
            }
            Err(e) => {
                log::warn!(target: "d3d", "gpu capture: start failed: {}", e.localizedDescription());
                false
            }
        }
    })
}

/// Label for a pipeline when a capture is configured: the pass-timing shader
/// key (`passtime shader` log column), so Xcode's per-pipeline costs map back
/// to `ptshader.py` rows. `None` when capture is off or a label was given.
pub fn pipeline_label(given: &str, kind: &str, key: u64) -> Option<String> {
    (given.is_empty() && enabled()).then(|| format!("{kind} {key:016x}"))
}
