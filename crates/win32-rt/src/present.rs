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

/// Default hold duration (in presented frames) for `PORT_INPUT_SCRIPT` entries
/// without an explicit `:hold_frames` third field.
const DEFAULT_SCRIPT_HOLD_FRAMES: u32 = 6;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScriptAction {
    start: u32,
    end: u32,
    name: String,
    mac_key: Option<(u16, Option<u16>)>,
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    thumb_lx: i16,
    thumb_ly: i16,
}

fn parse_input_script(script: &str) -> Vec<ScriptAction> {
    use crate::misc::xinput1_4::btn;

    let mut out = Vec::new();
    for item in script.split(',') {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.split(':');
        let Some(start_s) = parts.next() else {
            continue;
        };
        let Some(action_s) = parts.next() else {
            continue;
        };
        let hold_s = parts.next();
        if parts.next().is_some() {
            continue;
        }
        let Ok(start) = start_s.trim().parse::<u32>() else {
            continue;
        };
        let hold = match hold_s {
            Some(h) => match h.trim().parse::<u32>() {
                Ok(n) => n.max(1),
                Err(_) => continue,
            },
            None => DEFAULT_SCRIPT_HOLD_FRAMES,
        };
        let name = action_s.trim().to_ascii_lowercase();
        let (mac_key, buttons, left_trigger, right_trigger, thumb_lx, thumb_ly) =
            match name.as_str() {
                "enter" => (Some((0x24, Some(0x0D))), btn::A, 0, 0, 0, 0),
                "space" => (Some((0x31, Some(0x20))), btn::A, 0, 0, 0, 0),
                "a" => (Some((0x24, Some(0x0D))), btn::A, 0, 0, 0, 0),
                "esc" | "b" => (Some((0x35, Some(0x1B))), btn::B, 0, 0, 0, 0),
                "x" => (Some((0x07, Some(b'x' as u16))), btn::X, 0, 0, 0, 0),
                "y" => (Some((0x10, Some(b'y' as u16))), btn::Y, 0, 0, 0, 0),
                "start" => (Some((0x24, Some(0x0D))), btn::START, 0, 0, 0, 0),
                "back" | "select" | "tab" => (Some((0x30, Some(0x09))), btn::BACK, 0, 0, 0, 0),
                "lb" => (Some((0x0C, Some(b'q' as u16))), btn::LEFT_SHOULDER, 0, 0, 0, 0),
                "rb" => (Some((0x0E, Some(b'e' as u16))), btn::RIGHT_SHOULDER, 0, 0, 0, 0),
                "lt" => (None, 0, 255, 0, 0, 0),
                "rt" => (None, 0, 0, 255, 0, 0),
                "up" => (Some((0x7E, None)), btn::DPAD_UP, 0, 0, 0, 0),
                "down" => (Some((0x7D, None)), btn::DPAD_DOWN, 0, 0, 0, 0),
                "left" => (Some((0x7B, None)), btn::DPAD_LEFT, 0, 0, 0, 0),
                "right" => (Some((0x7C, None)), btn::DPAD_RIGHT, 0, 0, 0, 0),
                "ls_up" => (Some((0x0D, None)), 0, 0, 0, 0, i16::MAX),
                "ls_down" => (Some((0x01, None)), 0, 0, 0, 0, i16::MIN),
                "ls_left" => (Some((0x00, None)), 0, 0, 0, i16::MIN, 0),
                "ls_right" => (Some((0x02, None)), 0, 0, 0, i16::MAX, 0),
                _ => continue,
            };
        out.push(ScriptAction {
            start,
            end: start.saturating_add(hold),
            name,
            mac_key,
            buttons,
            left_trigger,
            right_trigger,
            thumb_lx,
            thumb_ly,
        });
    }
    out
}

fn combined_gamepad_at(actions: &[ScriptAction], frame: u32) -> crate::misc::xinput1_4::Gamepad {
    let mut gp = crate::misc::xinput1_4::Gamepad::default();
    let mut lx: i32 = 0;
    let mut ly: i32 = 0;
    for a in actions {
        if a.start <= frame && frame < a.end {
            gp.buttons |= a.buttons;
            gp.left_trigger = gp.left_trigger.max(a.left_trigger);
            gp.right_trigger = gp.right_trigger.max(a.right_trigger);
            lx += a.thumb_lx as i32;
            ly += a.thumb_ly as i32;
        }
    }
    gp.thumb_lx = lx.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    gp.thumb_ly = ly.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    gp
}

/// Generic scripted input runner (`PORT_INPUT_SCRIPT="900:down,960:enter:60"`).
fn run_input_script_step(hwnd: usize, frame: u32, script: &str) {
    use crate::misc::xinput1_4::{self, DeviceInfo};
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
    let actions = parse_input_script(script);
    let mut gamepad_changed = false;
    for a in &actions {
        if frame == a.start {
            gamepad_changed = true;
            log::info!(
                target: "d3d",
                "input script: frame {frame} {} down (hold={})",
                a.name,
                a.end.saturating_sub(a.start)
            );
            if let Some((mac, ch)) = a.mac_key {
                input::native_key(NativeKey {
                    mac,
                    down: true,
                    repeat: false,
                    chars: ch.into_iter().collect(),
                });
            }
        } else if frame == a.end {
            gamepad_changed = true;
            log::info!(target: "d3d", "input script: frame {frame} {} up", a.name);
            if let Some((mac, _)) = a.mac_key {
                let still_held = actions.iter().any(|other| {
                    other.start <= frame
                        && frame < other.end
                        && other.mac_key.is_some_and(|(m, _)| m == mac)
                });
                if !still_held {
                    input::native_key(NativeKey {
                        mac,
                        down: false,
                        repeat: false,
                        chars: Vec::new(),
                    });
                }
            }
        }
    }
    if gamepad_changed {
        xinput1_4::backend_update(0, combined_gamepad_at(&actions, frame));
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

    #[test]
    fn input_script_parses_default_and_custom_hold_and_combines_overlapping_gamepad_state() {
        use crate::misc::xinput1_4::btn;

        let actions = parse_input_script(
            "100:enter, 200:space:60, 210:ls_up:30, 220:rt:15, 300:start:0, bad:entry, 400:unknown:5",
        );
        assert_eq!(actions.len(), 5);
        assert_eq!((actions[0].start, actions[0].end, actions[0].buttons), (100, 106, btn::A));
        assert_eq!((actions[1].start, actions[1].end, actions[1].buttons), (200, 260, btn::A));
        assert_eq!((actions[2].start, actions[2].end, actions[2].thumb_ly), (210, 240, i16::MAX));
        assert_eq!((actions[3].start, actions[3].end, actions[3].right_trigger), (220, 235, 255));
        // Hold duration 0 clamps to 1 frame.
        assert_eq!((actions[4].start, actions[4].end, actions[4].buttons), (300, 301, btn::START));

        // At frame 225, space (btn::A), ls_up (thumb_ly=MAX), and rt (right_trigger=255) are all active.
        let gp225 = combined_gamepad_at(&actions, 225);
        assert_eq!(gp225.buttons, btn::A);
        assert_eq!(gp225.thumb_ly, i16::MAX);
        assert_eq!(gp225.right_trigger, 255);

        // At frame 245, ls_up & rt released, while space:60 remains continuously held until 260.
        let gp245 = combined_gamepad_at(&actions, 245);
        assert_eq!(gp245.buttons, btn::A);
        assert_eq!(gp245.thumb_ly, 0);
        assert_eq!(gp245.right_trigger, 0);

        // At frame 260, everything is released.
        let gp260 = combined_gamepad_at(&actions, 260);
        assert_eq!(gp260, crate::misc::xinput1_4::Gamepad::default());
    }
}
