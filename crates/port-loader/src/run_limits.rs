//! Bounded-run controller (`--timeout-secs`, `--max-frames`) and structured
//! exit-code / JSON summary reporting for `port-runner`.
//!
//! ## API-Neutral Present Hook Seam
//! Frame counting for `--max-frames` is funneled through a single function:
//! [`on_frame_presented`]. Today [`install`] registers it on
//! `win32_rt::d3d12::swapchain::register_present_hook`. Once
//! `toolkit/api-neutral-present-hooks` adds a unified D3D12/D3D11/Vulkan present
//! hook registry, replacing that single registration line (or forwarding from the
//! API-neutral present dispatch directly into [`on_frame_presented`]) enables
//! `--max-frames` across all graphics APIs with zero changes to run-limit logic.

use std::io::Write as _;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// Canonical exit-code table (kept identical to `port_factory::json_output::exit_codes`
/// so `port-loader` compiles cleanly even with `--no-default-features` on iOS).
pub mod exit_codes {
    pub const OK: i32 = 0;
    pub const RUNTIME_ERROR: i32 = 1;
    pub const USAGE_ERROR: i32 = 2;
    pub const UNRESOLVED_IMPORTS: i32 = 3;
    pub const VERIFICATION_FAILED: i32 = 4;
    pub const INCOMPATIBLE: i32 = 5;
    pub const TIMEOUT: i32 = 124;
}

/// Configuration for a bounded or JSON-reporting `port-runner` execution.
#[derive(Debug, Clone)]
pub struct RunLimitsConfig {
    pub target_exe: String,
    pub max_frames: Option<u64>,
    pub timeout_secs: Option<f64>,
    pub json_mode: bool,
}

/// Pure, lock-free frame-limit tracker (tested directly in unit tests and used by the
/// process-global [`on_frame_presented`] hook).
pub struct FrameLimitTracker {
    frames_presented: AtomicU64,
    max_frames: AtomicU64, // 0 = unlimited
    stop_triggered: AtomicBool,
}

impl FrameLimitTracker {
    pub const fn new() -> Self {
        Self {
            frames_presented: AtomicU64::new(0),
            max_frames: AtomicU64::new(0),
            stop_triggered: AtomicBool::new(false),
        }
    }

    pub fn configure(&self, max_frames: Option<u64>) {
        self.frames_presented.store(0, Ordering::Release);
        self.max_frames
            .store(max_frames.unwrap_or(0), Ordering::Release);
        self.stop_triggered.store(false, Ordering::Release);
    }

    /// Record one presented frame and return `(new_total_frames, should_stop_now)`.
    ///
    /// Guarantees `should_stop_now` is `true` at most once across concurrent callers.
    pub fn record_frame(&self) -> (u64, bool) {
        let count = self.frames_presented.fetch_add(1, Ordering::AcqRel) + 1;
        let limit = self.max_frames.load(Ordering::Acquire);
        if limit > 0 && count >= limit {
            let won = self
                .stop_triggered
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok();
            (count, won)
        } else {
            (count, false)
        }
    }

    /// Claim the one-shot stop token for a non-frame stop reason (timeout or guest exit).
    pub fn claim_stop(&self) -> bool {
        self.stop_triggered
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub fn is_stopped(&self) -> bool {
        self.stop_triggered.load(Ordering::Acquire)
    }

    pub fn frames_presented(&self) -> u64 {
        self.frames_presented.load(Ordering::Acquire)
    }

    pub fn max_frames(&self) -> Option<u64> {
        let v = self.max_frames.load(Ordering::Acquire);
        (v > 0).then_some(v)
    }
}

struct RuntimeState {
    target_exe: String,
    max_frames: Option<u64>,
    timeout_secs: Option<f64>,
    json_mode: bool,
    started: Instant,
}

static TRACKER: FrameLimitTracker = FrameLimitTracker::new();
static STATE: OnceLock<RuntimeState> = OnceLock::new();

/// Install `--max-frames` / `--timeout-secs` / `--json` live-run tracking.
///
/// Must be called before starting the guest thread.
pub fn install(cfg: RunLimitsConfig) {
    TRACKER.configure(cfg.max_frames);
    let need_present_hook = cfg.max_frames.is_some() || cfg.json_mode;
    let need_exit_hook = cfg.json_mode;
    let _ = STATE.set(RuntimeState {
        target_exe: cfg.target_exe,
        max_frames: cfg.max_frames,
        timeout_secs: cfg.timeout_secs,
        json_mode: cfg.json_mode,
        started: Instant::now(),
    });
    if need_present_hook {
        // SEAM for `toolkit/api-neutral-present-hooks`:
        // Replace this single registration with the API-neutral present hook registry
        // once D3D12, D3D11, and Vulkan share one unified present dispatch.
        win32_rt::d3d12::swapchain::register_present_hook(on_frame_presented);
    }
    if need_exit_hook {
        win32_rt::process::register_exit_hook(on_guest_process_exit);
    }
}

/// Start the `--timeout-secs` wall-clock watchdog thread if configured.
pub fn start_timeout_watchdog() {
    let Some(st) = STATE.get() else {
        return;
    };
    let Some(timeout_secs) = st.timeout_secs else {
        return;
    };
    let deadline = Duration::from_secs_f64(timeout_secs);
    let _ = std::thread::Builder::new()
        .name("port-run-timeout".to_string())
        .spawn(move || {
            let poll = Duration::from_millis(20);
            loop {
                if TRACKER.is_stopped() {
                    return;
                }
                let elapsed = st.started.elapsed();
                if elapsed >= deadline {
                    if TRACKER.claim_stop() {
                        let frames = TRACKER.frames_presented();
                        let elapsed_ms = elapsed.as_millis() as u64;
                        log::warn!(
                            "[run-limit] timed out after {timeout_secs}s ({elapsed_ms} ms, frames_presented={frames}); exiting with code {}",
                            exit_codes::TIMEOUT
                        );
                        if st.json_mode {
                            emit_run_json(
                                st,
                                "timeout",
                                exit_codes::TIMEOUT,
                                frames,
                                elapsed_ms,
                                None,
                                Some(format!(
                                    "execution timed out after {timeout_secs}s ({frames} frame(s) presented)"
                                )),
                            );
                        } else {
                            eprintln!(
                                "port-runner: timed out after {timeout_secs}s ({frames} frame(s) presented); exiting with code {}",
                                exit_codes::TIMEOUT
                            );
                            let _ = std::io::stderr().flush();
                        }
                        let _ = std::io::stdout().flush();
                        // Terminate immediately via `_exit` so a deadlocked guest mutex
                        // cannot block the timeout watchdog.
                        unsafe { libc::_exit(exit_codes::TIMEOUT) }
                    }
                    return;
                }
                let remaining = deadline.saturating_sub(elapsed);
                std::thread::sleep(remaining.min(poll));
            }
        });
}

/// Single API-neutral present callback seam for `--max-frames` and live frame counting.
///
/// Signature matches `win32_rt::d3d12::swapchain::PresentHook` (`fn(usize, u32)`).
/// Every graphics backend (D3D12, D3D11, Vulkan) should funnel presented frames here
/// once the API-neutral present hook registry lands.
pub fn on_frame_presented(_hwnd: usize, _api_frame: u32) {
    let (count, should_stop) = TRACKER.record_frame();
    if !should_stop {
        return;
    }
    let Some(st) = STATE.get() else {
        return;
    };
    let elapsed_ms = st.started.elapsed().as_millis() as u64;
    let limit = st.max_frames.unwrap_or(count);
    log::info!(
        "[run-limit] reached --max-frames {limit} (frames_presented={count}, elapsed={elapsed_ms}ms); exiting cleanly"
    );
    if st.json_mode {
        emit_run_json(
            st,
            "max_frames",
            exit_codes::OK,
            count,
            elapsed_ms,
            Some(0),
            None,
        );
    }
    win32_rt::process::exit_process(exit_codes::OK as u32);
}

/// Return the number of frames counted via [`on_frame_presented`] in this process.
pub fn frames_presented() -> u64 {
    TRACKER.frames_presented()
}

fn on_guest_process_exit() {
    let Some(st) = STATE.get() else {
        return;
    };
    if !st.json_mode || !TRACKER.claim_stop() {
        return;
    }
    let guest_code = win32_rt::process::exit_code().unwrap_or(0);
    let frames = TRACKER.frames_presented();
    let elapsed_ms = st.started.elapsed().as_millis() as u64;
    let exit_i32 = guest_code as i32;
    let err_msg = (guest_code != 0).then(|| format!("guest process exited with code {guest_code}"));
    emit_run_json(
        st,
        "guest_exit",
        exit_i32,
        frames,
        elapsed_ms,
        Some(guest_code),
        err_msg,
    );
}

fn emit_run_json(
    st: &RuntimeState,
    stop_reason: &str,
    exit_code: i32,
    frames: u64,
    elapsed_ms: u64,
    guest_exit_code: Option<u32>,
    error: Option<String>,
) {
    let text = format_run_json(
        &st.target_exe,
        stop_reason,
        exit_code,
        frames,
        elapsed_ms,
        st.max_frames,
        st.timeout_secs,
        guest_exit_code,
        error,
    );
    println!("{text}");
    let _ = std::io::stdout().flush();
}

#[allow(clippy::too_many_arguments)]
pub fn format_run_json(
    target_exe: &str,
    stop_reason: &str,
    exit_code: i32,
    frames: u64,
    elapsed_ms: u64,
    max_frames: Option<u64>,
    timeout_secs: Option<f64>,
    guest_exit_code: Option<u32>,
    error: Option<String>,
) -> String {
    #[cfg(feature = "cli")]
    {
        use port_factory::json_output::{port_runner_run_data_json, JsonEnvelope};
        let data = port_runner_run_data_json(
            target_exe,
            stop_reason,
            frames,
            elapsed_ms,
            max_frames,
            timeout_secs,
            guest_exit_code,
        );
        JsonEnvelope::with_status("port-runner", exit_code, data, error).to_json_pretty()
    }
    #[cfg(not(feature = "cli"))]
    {
        let _ = (
            target_exe,
            stop_reason,
            exit_code,
            frames,
            elapsed_ms,
            max_frames,
            timeout_secs,
            guest_exit_code,
            error,
        );
        "{}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_limit_tracker_counts_and_triggers_once() {
        let t = FrameLimitTracker::new();
        t.configure(Some(3));
        assert_eq!(t.record_frame(), (1, false));
        assert_eq!(t.record_frame(), (2, false));
        assert_eq!(t.record_frame(), (3, true));
        // Subsequent frames increment count but never trigger a second stop.
        assert_eq!(t.record_frame(), (4, false));
        assert!(t.is_stopped());
        assert!(!t.claim_stop());
        assert_eq!(t.frames_presented(), 4);
    }

    #[test]
    fn frame_limit_tracker_unlimited_never_triggers() {
        let t = FrameLimitTracker::new();
        t.configure(None);
        for expected in 1..=10u64 {
            assert_eq!(t.record_frame(), (expected, false));
        }
        assert!(!t.is_stopped());
        assert!(t.claim_stop());
        assert!(!t.claim_stop());
    }

    #[cfg(feature = "cli")]
    #[test]
    fn exit_codes_match_port_factory_table() {
        use port_factory::json_output::exit_codes as pf;
        assert_eq!(exit_codes::OK, pf::OK);
        assert_eq!(exit_codes::RUNTIME_ERROR, pf::RUNTIME_ERROR);
        assert_eq!(exit_codes::USAGE_ERROR, pf::USAGE_ERROR);
        assert_eq!(exit_codes::UNRESOLVED_IMPORTS, pf::UNRESOLVED_IMPORTS);
        assert_eq!(exit_codes::VERIFICATION_FAILED, pf::VERIFICATION_FAILED);
        assert_eq!(exit_codes::INCOMPATIBLE, pf::INCOMPATIBLE);
        assert_eq!(exit_codes::TIMEOUT, pf::TIMEOUT);
    }
}
