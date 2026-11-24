//! API-neutral presentation hooks and per-frame runtime notifications.
//!
//! D3D12 (`d3d12::swapchain`), D3D11 (`d3d11::swapchain`), and Vulkan
//! (`misc::vulkan_1::vkQueuePresentKHR`) all call [`on_present`] once a frame
//! completes presentation so hitch/stall tracking, `PORT_CONTROL_DIR` /
//! `live-probe`, scripted input (`PORT_INPUT_SCRIPT`), and registered
//! [`PresentHook`] callbacks work identically across graphics APIs.

use std::sync::atomic::{AtomicU32, AtomicU8, AtomicUsize, Ordering};
use std::sync::RwLock;

/// Graphics API that performed a swapchain presentation.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentApi {
    Unknown = 0,
    D3d12 = 1,
    D3d11 = 2,
    Vulkan = 3,
}

impl PresentApi {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::D3d12,
            2 => Self::D3d11,
            3 => Self::Vulkan,
            _ => Self::Unknown,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::D3d12 => "d3d12",
            Self::D3d11 => "d3d11",
            Self::Vulkan => "vulkan",
        }
    }
}

/// Callback invoked after every successful swapchain presentation (`(hwnd, frame_number)`).
pub type PresentHook = fn(usize, u32);

static PRESENT_HOOKS: RwLock<Vec<PresentHook>> = RwLock::new(Vec::new());
static PRESENT_COUNT: AtomicU32 = AtomicU32::new(0);
static LAST_PRESENT_API: AtomicU8 = AtomicU8::new(0);
static LAST_PRESENT_HWND: AtomicUsize = AtomicUsize::new(0);

/// Register a per-Present hook (e.g. `live_ctl`, automated test drivers, or telemetry).
pub fn register_present_hook(hook: PresentHook) {
    PRESENT_HOOKS.write().unwrap().push(hook);
}

/// Total number of successful presents recorded across all APIs in this process.
pub fn present_count() -> u32 {
    PRESENT_COUNT.load(Ordering::Acquire)
}

/// Graphics API of the most recent successful present (`PresentApi::Unknown` before frame 1).
pub fn last_present_api() -> PresentApi {
    PresentApi::from_u8(LAST_PRESENT_API.load(Ordering::Acquire))
}

/// Window handle (`HWND`) associated with the most recent successful present.
pub fn last_present_hwnd() -> usize {
    LAST_PRESENT_HWND.load(Ordering::Acquire)
}

/// Notify the runtime that a swapchain frame finished presenting.
///
/// Advances the process-wide monotonic frame counter, updates hitch/stall
/// accounting via [`crate::hitch::on_present`], runs any `PORT_INPUT_SCRIPT`
/// step, and invokes all registered [`PresentHook`] callbacks.
///
/// Returns the 1-based process-wide frame number assigned to this present.
pub fn on_present(api: PresentApi, hwnd: usize, present_ns: u64) -> u32 {
    let frame = PRESENT_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
    LAST_PRESENT_API.store(api as u8, Ordering::Release);
    LAST_PRESENT_HWND.store(hwnd, Ordering::Release);

    if frame == 1 && cfg!(target_os = "ios") {
        crate::thread::sched::promote_current_thread_interactive();
    }

    crate::hitch::on_present(frame, present_ns);

    if let Some(script) = std::env::var_os("PORT_INPUT_SCRIPT")
        .or_else(|| std::env::var_os("AW2_AUTO_PLAY_SCRIPT"))
        .and_then(|v| v.into_string().ok())
    {
        run_input_script_step(hwnd, frame, &script);
    }

    let hooks: Vec<PresentHook> = PRESENT_HOOKS
        .read()
        .map(|h| h.clone())
        .unwrap_or_default();
    for hook in hooks {
        hook(hwnd, frame);
    }

    frame
}

/// Generic scripted input runner (`PORT_INPUT_SCRIPT="900:down,960:enter"`).
fn run_input_script_step(hwnd: usize, frame: u32, script: &str) {
    use crate::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad};
    use crate::user32::input::{self, NativeKey};
    input::force_focus_for_autoplay(hwnd);
    if frame == 30 {
        xinput1_4::backend_connect(
            0,
            DeviceInfo {
                name: "Virtual Controller".into(),
                wireless: false,
                rumble: false,
                battery: None,
            },
        );
    }
    for item in script.split(',') {
        let Some((f, action)) = item.trim().split_once(':') else {
            continue;
        };
        let Ok(f) = f.trim().parse::<u32>() else {
            continue;
        };
        let down = if frame == f {
            true
        } else if frame == f + 6 {
            false
        } else {
            continue;
        };
        let (mac, chars, pad): (u16, Vec<u16>, u16) = match action.trim() {
            "enter" => (0x24, vec![0x0D], btn::A),
            "space" => (0x31, vec![0x20], btn::A),
            "a" => (0x24, vec![0x0D], btn::A),
            "esc" | "b" => (0x35, vec![0x1B], btn::B),
            "up" => (0x7E, Vec::new(), btn::DPAD_UP),
            "down" => (0x7D, Vec::new(), btn::DPAD_DOWN),
            "left" => (0x7B, Vec::new(), btn::DPAD_LEFT),
            "right" => (0x7C, Vec::new(), btn::DPAD_RIGHT),
            _ => continue,
        };
        log::info!(
            target: "d3d",
            "input script: frame {frame} {action} {}",
            if down { "down" } else { "up" }
        );
        input::native_key(NativeKey {
            mac,
            down,
            repeat: false,
            chars: if down { chars } else { Vec::new() },
        });
        xinput1_4::backend_update(
            0,
            Gamepad {
                buttons: if down { pad } else { 0 },
                ..Default::default()
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static HOOK_LOG: Mutex<Vec<(usize, u32)>> = Mutex::new(Vec::new());

    fn record_hook(hwnd: usize, frame: u32) {
        HOOK_LOG.lock().unwrap().push((hwnd, frame));
    }

    #[test]
    fn on_present_dispatches_hooks_and_tracks_api_metadata() {
        HOOK_LOG.lock().unwrap().clear();
        register_present_hook(record_hook);
        let base = present_count();

        let f1 = on_present(PresentApi::D3d11, 0x1111, 500_000);
        assert_eq!(f1, base + 1);
        assert_eq!(last_present_api(), PresentApi::D3d11);
        assert_eq!(last_present_hwnd(), 0x1111);

        let f2 = on_present(PresentApi::Vulkan, 0x2222, 600_000);
        assert_eq!(f2, base + 2);
        assert_eq!(last_present_api(), PresentApi::Vulkan);
        assert_eq!(last_present_hwnd(), 0x2222);

        let f3 = on_present(PresentApi::D3d12, 0x3333, 700_000);
        assert_eq!(f3, base + 3);
        assert_eq!(last_present_api(), PresentApi::D3d12);
        assert_eq!(last_present_hwnd(), 0x3333);

        let entries = HOOK_LOG.lock().unwrap();
        assert!(entries.contains(&(0x1111, f1)));
        assert!(entries.contains(&(0x2222, f2)));
        assert!(entries.contains(&(0x3333, f3)));
    }
}
