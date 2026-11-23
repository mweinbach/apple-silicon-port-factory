//! Universal live input & frame-capture controller (`PORT_CONTROL_DIR`).
//!
//! When `PORT_CONTROL_DIR=/path/to/dir` is set, registers a `PresentHook` that:
//! - polls `<dir>/cmd.txt` on every presented frame (atomically reading and removing it),
//! - drives real Win32 keyboard, raw mouse, and XInput gamepad inputs across frames,
//! - triggers on-demand Metal 4 back-buffer PNG captures via `d3d12_metal::metal::capture`,
//! - writes `<dir>/status.json` periodically so external tools/agents can play or test any
//!   Windows game running on `port-runner` / `aw2-loader` interactively.
//!
//! Supported command lines in `<dir>/cmd.txt` (multiple lines per file allowed):
//! - `shot <path.png>` — capture the next presented back buffer atomically to `<path.png>`
//! - `key <name> <frames>` — hold key `<name>` for `<frames>` frames (default 8)
//! - `mouse <dx> <dy> <frames>` — inject smooth relative mouse deltas across `<frames>` frames
//! - `moveto <x> <y>` — move cursor to client coordinates `(x, y)`
//! - `click <left|right|middle> <frames>` — press & hold mouse button for `<frames>` frames
//! - `wheel <delta> [frames]` — send vertical `WM_MOUSEWHEEL` (`delta` in 120-per-notch units)
//! - `pad <lx> <ly> <rx> <ry> <lt> <rt> <buttons_csv_or_none> <frames>` — hold XInput gamepad state
//! - `release` — release all held keys, mouse buttons, and gamepad sticks/buttons
//! - `quit` — terminate the guest process cleanly (`ExitProcess(0)`)

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use win32_rt as w;
use win32_rt::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad};
use win32_rt::user32::input::{self, MouseButton, NativeKey, NativeMouse};
use win32_rt::user32::POINT;

static CTL_DIR: OnceLock<PathBuf> = OnceLock::new();
static STATE: OnceLock<Mutex<LiveState>> = OnceLock::new();

struct MouseRamp {
    dx_per_frame: f64,
    dy_per_frame: f64,
    rem_frames: u32,
}

struct LiveState {
    started: Instant,
    last_status_t: Instant,
    last_status_frame: u32,
    pad_connected: bool,
    held_keys: HashMap<u16, (u32, Vec<u16>)>, // mac_code -> (remaining_frames, chars)
    held_buttons: HashMap<u8, u32>,           // button_id (0=L,1=R,2=M) -> remaining_frames
    cursor_pt: POINT,
    mouse_ramp: Option<MouseRamp>,
    wheel_hold: Option<(i32, u32)>,
    pad_state: Option<(Gamepad, u32)>,
    last_cmd_count: u64,
}

impl LiveState {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            started: now,
            last_status_t: now,
            last_status_frame: 0,
            pad_connected: false,
            held_keys: HashMap::new(),
            held_buttons: HashMap::new(),
            cursor_pt: POINT { x: 640, y: 360 },
            mouse_ramp: None,
            wheel_hold: None,
            pad_state: None,
            last_cmd_count: 0,
        }
    }
}

/// Enable the universal live controller if `PORT_CONTROL_DIR` is set.
pub fn install() {
    let Ok(dir) = std::env::var("PORT_CONTROL_DIR") else {
        return;
    };
    let dir = dir.trim();
    if dir.is_empty() {
        return;
    }
    let path = PathBuf::from(dir);
    let _ = std::fs::create_dir_all(&path);
    let _ = CTL_DIR.set(path.clone());
    let _ = STATE.set(Mutex::new(LiveState::new()));
    w::d3d12::swapchain::register_present_hook(on_present);
    log::info!("live_ctl: enabled at {}", path.display());
}

fn key_spec(name: &str) -> Option<(u16, Vec<u16>)> {
    let lower = name.trim().to_ascii_lowercase();
    let (mac, ch): (u16, Option<u16>) = match lower.as_str() {
        "a" => (0x00, Some(b'a' as u16)),
        "s" => (0x01, Some(b's' as u16)),
        "d" => (0x02, Some(b'd' as u16)),
        "f" => (0x03, Some(b'f' as u16)),
        "h" => (0x04, Some(b'h' as u16)),
        "g" => (0x05, Some(b'g' as u16)),
        "z" => (0x06, Some(b'z' as u16)),
        "x" => (0x07, Some(b'x' as u16)),
        "c" => (0x08, Some(b'c' as u16)),
        "v" => (0x09, Some(b'v' as u16)),
        "b" => (0x0B, Some(b'b' as u16)),
        "q" => (0x0C, Some(b'q' as u16)),
        "w" => (0x0D, Some(b'w' as u16)),
        "e" => (0x0E, Some(b'e' as u16)),
        "r" => (0x0F, Some(b'r' as u16)),
        "y" => (0x10, Some(b'y' as u16)),
        "t" => (0x11, Some(b't' as u16)),
        "1" => (0x12, Some(b'1' as u16)),
        "2" => (0x13, Some(b'2' as u16)),
        "3" => (0x14, Some(b'3' as u16)),
        "4" => (0x15, Some(b'4' as u16)),
        "6" => (0x16, Some(b'6' as u16)),
        "5" => (0x17, Some(b'5' as u16)),
        "9" => (0x19, Some(b'9' as u16)),
        "7" => (0x1A, Some(b'7' as u16)),
        "8" => (0x1C, Some(b'8' as u16)),
        "0" => (0x1D, Some(b'0' as u16)),
        "o" => (0x1F, Some(b'o' as u16)),
        "u" => (0x20, Some(b'u' as u16)),
        "i" => (0x22, Some(b'i' as u16)),
        "p" => (0x23, Some(b'p' as u16)),
        "enter" | "return" => (0x24, Some(0x0D)),
        "l" => (0x25, Some(b'l' as u16)),
        "j" => (0x26, Some(b'j' as u16)),
        "k" => (0x28, Some(b'k' as u16)),
        "n" => (0x2D, Some(b'n' as u16)),
        "m" => (0x2E, Some(b'm' as u16)),
        "tab" => (0x30, Some(0x09)),
        "space" => (0x31, Some(0x20)),
        "backspace" | "back" => (0x33, Some(0x08)),
        "esc" | "escape" => (0x35, Some(0x1B)),
        "shift" | "lshift" => (0x38, None),
        "alt" | "lalt" => (0x3A, None),
        "ctrl" | "lctrl" => (0x3B, None),
        "left" => (0x7B, None),
        "right" => (0x7C, None),
        "down" => (0x7D, None),
        "up" => (0x7E, None),
        _ => return None,
    };
    Some((mac, ch.into_iter().collect()))
}

fn parse_pad_buttons(spec: &str) -> u16 {
    let mut mask = 0u16;
    for tok in spec.split([',', '+', '|']) {
        match tok.trim().to_ascii_lowercase().as_str() {
            "" | "none" | "0" => {}
            "a" => mask |= btn::A,
            "b" => mask |= btn::B,
            "x" => mask |= btn::X,
            "y" => mask |= btn::Y,
            "lb" | "l1" => mask |= btn::LEFT_SHOULDER,
            "rb" | "r1" => mask |= btn::RIGHT_SHOULDER,
            "start" => mask |= btn::START,
            "back" | "select" | "view" => mask |= btn::BACK,
            "ls" | "l3" => mask |= btn::LEFT_THUMB,
            "rs" | "r3" => mask |= btn::RIGHT_THUMB,
            "up" | "dup" => mask |= btn::DPAD_UP,
            "down" | "ddown" => mask |= btn::DPAD_DOWN,
            "left" | "dleft" => mask |= btn::DPAD_LEFT,
            "right" | "dright" => mask |= btn::DPAD_RIGHT,
            _ => {}
        }
    }
    mask
}

fn mouse_btn_from_id(id: u8) -> MouseButton {
    match id {
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        _ => MouseButton::Left,
    }
}

fn release_all(st: &mut LiveState, hwnd: usize) {
    for (mac, _) in st.held_keys.drain() {
        input::native_key(NativeKey {
            mac,
            down: false,
            repeat: false,
            chars: Vec::new(),
        });
    }
    for (bid, _) in st.held_buttons.drain() {
        if hwnd != 0 {
            input::native_mouse(NativeMouse::Button {
                hwnd,
                client: Some(st.cursor_pt),
                screen: st.cursor_pt,
                button: mouse_btn_from_id(bid),
                down: false,
                clicks: 1,
            });
        }
    }
    st.mouse_ramp = None;
    st.wheel_hold = None;
    if st.pad_state.take().is_some() {
        xinput1_4::backend_update(0, Gamepad::default());
    }
}

fn apply_line(st: &mut LiveState, hwnd: usize, dir: &PathBuf, line: &str) {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return;
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }
    st.last_cmd_count += 1;
    match parts[0].to_ascii_lowercase().as_str() {
        "shot" | "screenshot" => {
            let target = if parts.len() >= 2 {
                let raw = parts[1..].join(" ");
                let p = PathBuf::from(&raw);
                if p.is_absolute() {
                    raw
                } else {
                    dir.join(p).to_string_lossy().into_owned()
                }
            } else {
                dir.join("frame.png").to_string_lossy().into_owned()
            };
            d3d12_metal::metal::capture::request_next(target);
        }
        "key" if parts.len() >= 2 => {
            if let Some((mac, chars)) = key_spec(parts[1]) {
                let frames = parts
                    .get(2)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(8)
                    .max(1);
                let already = st.held_keys.contains_key(&mac);
                st.held_keys.insert(mac, (frames, chars.clone()));
                if !already {
                    input::native_key(NativeKey {
                        mac,
                        down: true,
                        repeat: false,
                        chars,
                    });
                }
            }
        }
        "mouse" if parts.len() >= 3 => {
            let dx: f64 = parts[1].parse().unwrap_or(0.0);
            let dy: f64 = parts[2].parse().unwrap_or(0.0);
            let frames = parts
                .get(3)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1)
                .max(1);
            st.mouse_ramp = Some(MouseRamp {
                dx_per_frame: dx / frames as f64,
                dy_per_frame: dy / frames as f64,
                rem_frames: frames,
            });
        }
        "moveto" if parts.len() >= 3 => {
            let x: i32 = parts[1].parse().unwrap_or(640);
            let y: i32 = parts[2].parse().unwrap_or(360);
            st.cursor_pt = POINT { x, y };
            if hwnd != 0 {
                input::native_mouse(NativeMouse::Move {
                    hwnd,
                    client: Some(st.cursor_pt),
                    screen: st.cursor_pt,
                    dx: 0.0,
                    dy: 0.0,
                });
            }
        }
        "click" if parts.len() >= 2 => {
            let bid: u8 = match parts[1].to_ascii_lowercase().as_str() {
                "right" | "r" => 1,
                "middle" | "m" => 2,
                _ => 0,
            };
            let frames = parts
                .get(2)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(6)
                .max(1);
            let already = st.held_buttons.contains_key(&bid);
            st.held_buttons.insert(bid, frames);
            if !already && hwnd != 0 {
                input::native_mouse(NativeMouse::Button {
                    hwnd,
                    client: Some(st.cursor_pt),
                    screen: st.cursor_pt,
                    button: mouse_btn_from_id(bid),
                    down: true,
                    clicks: 1,
                });
            }
        }
        "wheel" if parts.len() >= 2 => {
            let delta: i32 = parts[1].parse().unwrap_or(-120);
            let frames = parts
                .get(2)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(1)
                .max(1);
            st.wheel_hold = Some((delta, frames));
        }
        "pad" if parts.len() >= 8 => {
            let lx: i16 = parts[1].parse().unwrap_or(0);
            let ly: i16 = parts[2].parse().unwrap_or(0);
            let rx: i16 = parts[3].parse().unwrap_or(0);
            let ry: i16 = parts[4].parse().unwrap_or(0);
            let lt: u8 = parts[5].parse().unwrap_or(0);
            let rt: u8 = parts[6].parse().unwrap_or(0);
            let buttons = parse_pad_buttons(parts[7]);
            let frames = parts
                .get(8)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(15)
                .max(1);
            let gp = Gamepad {
                buttons,
                left_trigger: lt,
                right_trigger: rt,
                thumb_lx: lx,
                thumb_ly: ly,
                thumb_rx: rx,
                thumb_ry: ry,
            };
            st.pad_state = Some((gp, frames));
            xinput1_4::backend_update(0, gp);
        }
        "release" => release_all(st, hwnd),
        "gpunan" => {
            w::d3d12::mtl::arm_gpu_nan_scan();
        }
        "nantrace" if parts.len() >= 2 => {
            match parts[1].to_ascii_lowercase().as_str() {
                "on" | "1" | "start" => {
                    x86_runtime::diag::set_nan_trace_active(true);
                }
                "off" | "0" | "stop" => {
                    x86_runtime::diag::set_nan_trace_active(false);
                }
                "range" if parts.len() >= 3 => {
                    let ranges: Vec<_> = parts[2..]
                        .join(",")
                        .split(',')
                        .filter_map(x86_runtime::diag::VaRange::parse)
                        .collect();
                    x86_runtime::diag::set_nan_trace_ranges(ranges);
                }
                "watch" if parts.len() >= 4 => {
                    let parse_hex_or_dec = |s: &str| -> Option<u64> {
                        let s = s.trim();
                        if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                            u64::from_str_radix(h, 16).ok()
                        } else {
                            s.parse::<u64>().ok()
                        }
                    };
                    if let (Some(addr), Some(len)) = (parse_hex_or_dec(parts[2]), parse_hex_or_dec(parts[3])) {
                        x86_runtime::diag::set_nan_watch_mem(addr, len);
                    }
                }
                "stores" if parts.len() >= 3 => {
                    let on = parts[2] != "0" && !parts[2].eq_ignore_ascii_case("off") && !parts[2].eq_ignore_ascii_case("false");
                    x86_runtime::diag::set_nan_trace_stores(on);
                }
                "clear" => {
                    x86_runtime::diag::clear_nan_trace_filters();
                }
                _ => {
                    log::debug!("live_ctl: unknown nantrace subcommand {trimmed:?}");
                }
            }
        }
        "quit" | "exit" => {
            log::info!("live_ctl: quit command received -> ExitProcess(0)");
            unsafe {
                w::kernel32::process::ExitProcess(0);
            }
        }
        _ => {
            log::debug!("live_ctl: unknown command {trimmed:?}");
        }
    }
}

fn write_status(dir: &PathBuf, st: &mut LiveState, hwnd: usize, frame: u32) {
    let now = Instant::now();
    let dt = now.duration_since(st.last_status_t).as_secs_f64();
    let fps = if dt > 0.001 {
        frame.saturating_sub(st.last_status_frame) as f64 / dt
    } else {
        0.0
    };
    st.last_status_t = now;
    st.last_status_frame = frame;
    let uptime = now.duration_since(st.started).as_secs_f64();
    let active_keys = st.held_keys.len();
    let pad_rem = st.pad_state.map_or(0, |(_, r)| r);
    let mouse_rem = st.mouse_ramp.as_ref().map_or(0, |m| m.rem_frames);
    let (nan_births, host_nans, mem_watches) = x86_runtime::diag::nan_trace_stats();
    let json = format!(
        "{{\"frame\":{frame},\"fps\":{fps:.1},\"uptime_s\":{uptime:.2},\"hwnd\":{hwnd},\"cmds\":{},\"active_keys\":{active_keys},\"mouse_rem\":{mouse_rem},\"pad_rem\":{pad_rem},\"nan_active\":{},\"nan_births\":{nan_births},\"host_events\":{host_nans},\"mem_watches\":{mem_watches}}}\n",
        st.last_cmd_count,
        x86_runtime::diag::nan_trace_active()
    );
    let tmp = dir.join("status.json.tmp");
    let dst = dir.join("status.json");
    if std::fs::write(&tmp, json).is_ok() {
        let _ = std::fs::rename(&tmp, &dst);
    }
}

fn on_present(hwnd: usize, frame: u32) {
    let (Some(dir), Some(lock)) = (CTL_DIR.get(), STATE.get()) else {
        return;
    };
    let mut st = lock.lock().unwrap();
    if !st.pad_connected && frame >= 10 {
        xinput1_4::backend_connect(
            0,
            DeviceInfo {
                name: "Virtual Live Controller".into(),
                wireless: false,
                rumble: false,
                battery: None,
            },
        );
        st.pad_connected = true;
    }

    let mut had_cmd = false;
    let cmd_path = dir.join("cmd.txt");
    if let Ok(contents) = std::fs::read_to_string(&cmd_path) {
        let _ = std::fs::remove_file(&cmd_path);
        had_cmd = true;
        input::force_focus_for_autoplay(hwnd);
        for line in contents.lines() {
            apply_line(&mut st, hwnd, dir, line);
        }
    }

    let has_active = !st.held_keys.is_empty()
        || !st.held_buttons.is_empty()
        || st.mouse_ramp.is_some()
        || st.wheel_hold.is_some()
        || st.pad_state.is_some();
    if has_active {
        input::force_focus_for_autoplay(hwnd);
    }

    // Advance keyboard holds
    let mut expired_keys = Vec::new();
    for (&mac, (rem, _)) in st.held_keys.iter_mut() {
        if *rem <= 1 {
            expired_keys.push(mac);
        } else {
            *rem -= 1;
        }
    }
    for mac in expired_keys {
        st.held_keys.remove(&mac);
        input::native_key(NativeKey {
            mac,
            down: false,
            repeat: false,
            chars: Vec::new(),
        });
    }

    // Advance mouse button holds
    let mut expired_btns = Vec::new();
    for (&bid, rem) in st.held_buttons.iter_mut() {
        if *rem <= 1 {
            expired_btns.push(bid);
        } else {
            *rem -= 1;
        }
    }
    for bid in expired_btns {
        st.held_buttons.remove(&bid);
        if hwnd != 0 {
            input::native_mouse(NativeMouse::Button {
                hwnd,
                client: Some(st.cursor_pt),
                screen: st.cursor_pt,
                button: mouse_btn_from_id(bid),
                down: false,
                clicks: 1,
            });
        }
    }

    // Advance smooth relative mouse movement
    let cur_pt = st.cursor_pt;
    if let Some(ref mut ramp) = st.mouse_ramp {
        if hwnd != 0 {
            input::native_mouse(NativeMouse::Move {
                hwnd,
                client: Some(cur_pt),
                screen: cur_pt,
                dx: ramp.dx_per_frame,
                dy: ramp.dy_per_frame,
            });
        }
        if ramp.rem_frames <= 1 {
            st.mouse_ramp = None;
        } else {
            ramp.rem_frames -= 1;
        }
    }

    // Advance wheel scroll holds
    if let Some((delta, rem)) = st.wheel_hold {
        if hwnd != 0 {
            input::native_mouse(NativeMouse::Wheel {
                screen: cur_pt,
                delta,
                horizontal: false,
            });
        }
        if rem <= 1 {
            st.wheel_hold = None;
        } else {
            st.wheel_hold = Some((delta, rem - 1));
        }
    }

    // Advance XInput gamepad hold
    if let Some((gp, rem)) = st.pad_state {
        if rem <= 1 {
            st.pad_state = None;
            xinput1_4::backend_update(0, Gamepad::default());
        } else {
            st.pad_state = Some((gp, rem - 1));
            xinput1_4::backend_update(0, gp);
        }
    }

    if had_cmd || frame <= 5 || frame % 15 == 0 {
        write_status(dir, &mut st, hwnd, frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_and_gamepad_parsers_cover_standard_controls() {
        assert_eq!(key_spec("w"), Some((0x0D, vec![b'w' as u16])));
        assert_eq!(key_spec("SHIFT"), Some((0x38, vec![])));
        assert_eq!(key_spec("enter"), Some((0x24, vec![0x0D])));
        assert_eq!(key_spec("esc"), Some((0x35, vec![0x1B])));
        assert_eq!(key_spec("up"), Some((0x7E, vec![])));
        assert_eq!(parse_pad_buttons("a+rb,start"), btn::A | btn::RIGHT_SHOULDER | btn::START);
        assert_eq!(parse_pad_buttons("none"), 0);
    }
}
