//! NSEvent -> user32 input translation (main thread).
//!
//! Keys are never forwarded to AppKit except Command combinations (so
//! Cmd+Q / Cmd+H reach the menu); mouse events are forwarded too so window
//! dragging, resizing and activation clicks keep working.

use super::appkit::{content_rect, hwnd_for_window};
use super::display;
use crate::user32::input::{self, MouseButton, NativeKey, NativeMouse};
use crate::user32::POINT;
use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
use objc2_foundation::MainThreadMarker;
use std::cell::Cell;

thread_local! {
    static WHEEL_ACC: Cell<(f64, f64)> = const { Cell::new((0.0, 0.0)) };
}

pub(super) fn handle(app: &NSApplication, ev: &NSEvent, mtm: MainThreadMarker) {
    let ty = ev.r#type();
    match ty {
        NSEventType::KeyDown | NSEventType::KeyUp | NSEventType::FlagsChanged => {
            if let Some((edit_hwnd, wants_return)) = super::appkit::focused_native_edit_info(ev, mtm) {
                let has_cmd = ev.modifierFlags().contains(NSEventModifierFlags::Command);
                match classify_edit_key(ty, ev.keyCode(), has_cmd, wants_return) {
                    EditKeyRoute::DialogToWin32 => match translate_key(ev) {
                        KeyAction::Forward => app.sendEvent(ev),
                        KeyAction::Key(k) => input::native_key_for_hwnd(edit_hwnd, k),
                        KeyAction::CapsToggle(mac) => input::native_caps_toggle(mac),
                        KeyAction::Ignore => {}
                    },
                    EditKeyRoute::ModifierBoth => {
                        match translate_key(ev) {
                            KeyAction::Key(k) => input::native_key_for_hwnd(edit_hwnd, k),
                            KeyAction::CapsToggle(mac) => input::native_caps_toggle(mac),
                            KeyAction::Forward | KeyAction::Ignore => {}
                        }
                        app.sendEvent(ev);
                    }
                    EditKeyRoute::CommandToAppKit => {
                        app.sendEvent(ev);
                        super::appkit::sync_focused_edit_selection(ev, mtm);
                    }
                    EditKeyRoute::AppKitEditWithWin32State => {
                        if let KeyAction::Key(k) = translate_key(ev) {
                            input::native_key_state_only(k);
                        }
                        app.sendEvent(ev);
                        super::appkit::sync_focused_edit_selection(ev, mtm);
                    }
                }
                return;
            }
            match translate_key(ev) {
                KeyAction::Forward => app.sendEvent(ev),
                KeyAction::Key(k) => input::native_key(k),
                KeyAction::CapsToggle(mac) => input::native_caps_toggle(mac),
                KeyAction::Ignore => {}
            }
        }
        NSEventType::MouseMoved
        | NSEventType::LeftMouseDragged
        | NSEventType::RightMouseDragged
        | NSEventType::OtherMouseDragged => {
            let (hwnd, client, screen) = locate(ev, mtm);
            let (dx, dy) = move_delta(ev);
            input::native_mouse(NativeMouse::Move { hwnd, client, screen, dx, dy });
            app.sendEvent(ev);
        }
        NSEventType::LeftMouseDown
        | NSEventType::LeftMouseUp
        | NSEventType::RightMouseDown
        | NSEventType::RightMouseUp
        | NSEventType::OtherMouseDown
        | NSEventType::OtherMouseUp => {
            let (hwnd, client, screen) = locate(ev, mtm);
            let down = matches!(ty, NSEventType::LeftMouseDown | NSEventType::RightMouseDown | NSEventType::OtherMouseDown);
            let button = match ev.buttonNumber() {
                0 => MouseButton::Left,
                1 => MouseButton::Right,
                2 => MouseButton::Middle,
                3 => MouseButton::X1,
                4 => MouseButton::X2,
                _ => {
                    app.sendEvent(ev);
                    return;
                }
            };
            input::native_mouse(NativeMouse::Button { hwnd, client, screen, button, down, clicks: ev.clickCount() as i64 });
            if down && matches!(button, MouseButton::Left) {
                super::appkit::notify_static_click_if_hit(ev, mtm);
            }
            // In relative mode the frozen pointer may sit on a title bar;
            // don't let AppKit start a window drag from game clicks.
            if !crate::user32::cursor::relative_mode() || hwnd == 0 {
                app.sendEvent(ev);
                if matches!(button, MouseButton::Left) {
                    super::appkit::sync_focused_edit_selection(ev, mtm);
                }
            }
        }
        NSEventType::ScrollWheel => {
            if super::appkit::window_has_native_controls(ev, mtm) {
                app.sendEvent(ev);
                return;
            }
            let (_, _, screen) = locate(ev, mtm);
            let (dy, dx) = wheel_notches(ev);
            let (ay, ax) = WHEEL_ACC.with(|a| {
                let (mut ay, mut ax) = a.get();
                ay += dy;
                ax += dx;
                let (ny, nx) = (ay.trunc(), ax.trunc());
                a.set((ay - ny, ax - nx));
                (ny, nx)
            });
            if ay != 0.0 {
                input::native_mouse(NativeMouse::Wheel { screen, delta: (ay * 120.0) as i32, horizontal: false });
            }
            if ax != 0.0 {
                input::native_mouse(NativeMouse::Wheel { screen, delta: (ax * 120.0) as i32, horizontal: true });
            }
        }
        _ => app.sendEvent(ev),
    }
}

/// Window under the event (if it is ours and the point is in its content
/// area), client position, and Win32 screen position.
fn locate(ev: &NSEvent, mtm: MainThreadMarker) -> (usize, Option<POINT>, POINT) {
    let loc = ev.locationInWindow();
    let win = ev.window(mtm);
    let (sx, sy) = match &win {
        Some(w) => {
            let f = w.frame();
            (f.origin.x + loc.x, f.origin.y + loc.y)
        }
        None => (loc.x, loc.y),
    };
    let screen = display::cocoa_point_to_px(sx, sy);
    let Some(hwnd) = win.as_deref().and_then(hwnd_for_window) else {
        super::appkit::refresh_cursor_for_client(false);
        return (0, None, screen);
    };
    let Some(cr) = content_rect(hwnd) else {
        super::appkit::refresh_cursor_for_client(false);
        return (0, None, screen);
    };
    let inside = sx >= cr.origin.x && sx < cr.origin.x + cr.size.width && sy > cr.origin.y && sy <= cr.origin.y + cr.size.height;
    super::appkit::refresh_cursor_for_client(inside);
    if !inside {
        return (0, None, screen);
    }
    let s = display::scale_for_cocoa_point(sx, sy);
    let client = POINT { x: ((sx - cr.origin.x) * s).floor() as i32, y: ((cr.origin.y + cr.size.height - sy) * s).floor() as i32 };
    (hwnd, Some(client), screen)
}

/// Device-dependent modifier bits (IOKit NX_DEVICE*KEYMASK) by mac key code.
fn modifier_mask(mac: u16) -> Option<usize> {
    Some(match mac {
        0x3B => 0x0001, // left control
        0x38 => 0x0002, // left shift
        0x3C => 0x0004, // right shift
        0x3A => 0x0020, // left option
        0x3D => 0x0040, // right option
        0x3E => 0x2000, // right control
        _ => return None,
    })
}

/// How a keyboard event should be routed when a native AppKit `EDIT` control
/// holds first-responder focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EditKeyRoute {
    /// Escape, Tab, or Return without `ES_WANTRETURN`: route to Win32 so
    /// `IsDialogMessage` handles dialog cancel/navigation/default-button logic.
    DialogToWin32,
    /// Modifier key transitions (`FlagsChanged`): update Win32 key state and
    /// forward to AppKit so both `Shift+Tab` and AppKit text editing stay in sync.
    ModifierBoth,
    /// Command shortcut (Cmd+C/V/X/A/Z/Q/H): forward directly to AppKit.
    CommandToAppKit,
    /// Regular editing keys (typing, Backspace, Delete, arrows, or Return with
    /// `ES_WANTRETURN`): update Win32 key state without double-posting `WM_CHAR`,
    /// and forward to AppKit.
    AppKitEditWithWin32State,
}

pub(super) fn classify_edit_key(
    ty: NSEventType,
    mac_keycode: u16,
    has_command: bool,
    wants_return: bool,
) -> EditKeyRoute {
    if ty == NSEventType::FlagsChanged {
        return EditKeyRoute::ModifierBoth;
    }
    if has_command {
        return EditKeyRoute::CommandToAppKit;
    }
    match mac_keycode {
        0x35 | 0x30 => EditKeyRoute::DialogToWin32, // kVK_Escape, kVK_Tab
        0x24 | 0x4C if !wants_return => EditKeyRoute::DialogToWin32, // kVK_Return, kVK_ANSI_KeypadEnter
        _ => EditKeyRoute::AppKitEditWithWin32State,
    }
}

/// What a key-class NSEvent (`keyDown:` / `keyUp:` / `flagsChanged:`)
/// becomes.
#[derive(Debug)]
pub(super) enum KeyAction {
    /// Command shortcut: hand to AppKit (menu), not to the game.
    Forward,
    Key(NativeKey),
    /// Caps Lock: one press+release per toggle.
    CapsToggle(u16),
    /// Command / Fn modifiers (deliberately unmapped).
    Ignore,
}

pub(super) fn translate_key(ev: &NSEvent) -> KeyAction {
    let ty = ev.r#type();
    if ty == NSEventType::FlagsChanged {
        let mac = ev.keyCode();
        if mac == 0x39 {
            return KeyAction::CapsToggle(mac);
        }
        let Some(mask) = modifier_mask(mac) else { return KeyAction::Ignore };
        let down = ev.modifierFlags().0 & mask != 0;
        return KeyAction::Key(NativeKey { mac, down, repeat: false, chars: Vec::new() });
    }
    if ev.modifierFlags().contains(NSEventModifierFlags::Command) {
        return KeyAction::Forward;
    }
    let down = ty == NSEventType::KeyDown;
    let chars = if down {
        ev.characters().map(|s| s.to_string().encode_utf16().collect()).unwrap_or_default()
    } else {
        Vec::new()
    };
    KeyAction::Key(NativeKey { mac: ev.keyCode(), down, repeat: down && ev.isARepeat(), chars })
}

/// Relative motion of a mouse-move/drag event, in points, +Y down. Includes
/// motion while the cursor is disassociated (relative mode).
pub(super) fn move_delta(ev: &NSEvent) -> (f64, f64) {
    (ev.deltaX(), ev.deltaY())
}

/// Scroll event -> (vertical, horizontal) wheel notches, Windows sign
/// convention: +vertical = away from the user, +horizontal = right.
pub(super) fn wheel_notches(ev: &NSEvent) -> (f64, f64) {
    let inv = if ev.isDirectionInvertedFromDevice() { -1.0 } else { 1.0 };
    // Lines -> notches; trackpads report pixels (~30 px per notch).
    let k = if ev.hasPreciseScrollingDeltas() { 1.0 / 30.0 } else { 1.0 };
    (ev.scrollingDeltaY() * inv * k, -ev.scrollingDeltaX() * inv * k)
}

/// Synthesized NSEvents (created off the main thread, never dispatched)
/// through the translation layer into the Win32 queues: legacy messages
/// plus `WM_INPUT` records read back with `GetRawInputData`.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HWND;
    use crate::user32::{class, input as inp, message, window, wm, MSG};
    use objc2::encode::{Encoding, RefEncode};
    use objc2::rc::Retained;
    use objc2::{msg_send, ClassType};
    use objc2_foundation::{NSPoint, NSString};
    use std::ffi::c_void;

    #[repr(C)]
    struct CGEventT {
        _p: [u8; 0],
    }
    unsafe impl RefEncode for CGEventT {
        const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("__CGEvent", &[]));
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }
    extern "C" {
        fn CGEventCreateMouseEvent(src: *const c_void, ty: u32, pt: CGPoint, button: u32) -> *mut CGEventT;
        fn CGEventCreateScrollWheelEvent2(src: *const c_void, units: u32, count: u32, w1: i32, w2: i32, w3: i32) -> *mut CGEventT;
        fn CGEventSetIntegerValueField(e: *mut CGEventT, field: u32, v: i64);
        fn CFRelease(p: *const c_void);
    }
    const KCG_EVENT_MOUSE_MOVED: u32 = 5;
    const KCG_MOUSE_EVENT_DELTA_X: u32 = 4;
    const KCG_MOUSE_EVENT_DELTA_Y: u32 = 5;
    const KCG_SCROLL_EVENT_UNIT_LINE: u32 = 1;

    fn from_cg(e: *mut CGEventT) -> Retained<NSEvent> {
        let ev: Option<Retained<NSEvent>> = unsafe { msg_send![NSEvent::class(), eventWithCGEvent: e] };
        unsafe { CFRelease(e as *const c_void) };
        ev.expect("eventWithCGEvent")
    }

    fn key_event(down: bool, mac: u16, chars: &str, flags: NSEventModifierFlags) -> Retained<NSEvent> {
        let s = NSString::from_str(chars);
        NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            if down { NSEventType::KeyDown } else { NSEventType::KeyUp },
            NSPoint::new(0.0, 0.0),
            flags,
            0.0,
            0,
            None,
            &s,
            &s,
            false,
            mac,
        )
        .expect("keyEvent")
    }

    fn flags_event(mac: u16, flags: NSEventModifierFlags) -> Retained<NSEvent> {
        let s = NSString::from_str("");
        NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            NSEventType::FlagsChanged,
            NSPoint::new(0.0, 0.0),
            flags,
            0.0,
            0,
            None,
            &s,
            &s,
            false,
            mac,
        )
        .expect("flagsChanged event")
    }

    fn feed_key(ev: &NSEvent) {
        match translate_key(ev) {
            KeyAction::Key(k) => inp::native_key(k),
            KeyAction::CapsToggle(m) => inp::native_caps_toggle(m),
            other => panic!("unexpected {other:?}"),
        }
    }

    /// Drain the thread's queue: (message, wparam, lparam, raw record).
    fn drain(h: HWND) -> Vec<(u32, usize, isize, Vec<u8>)> {
        let mut out = Vec::new();
        let mut m = MSG::default();
        unsafe {
            while message::PeekMessageW(&mut m, h, 0, 0, message::PM_REMOVE) != 0 {
                let mut raw = Vec::new();
                if m.message == wm::INPUT {
                    let mut sz = 0u32;
                    assert_eq!(inp::GetRawInputData(m.lparam as usize, 0x1000_0003, std::ptr::null_mut(), &mut sz, 24), 0);
                    raw = vec![0u8; sz as usize];
                    assert_eq!(inp::GetRawInputData(m.lparam as usize, 0x1000_0003, raw.as_mut_ptr(), &mut sz, 24), sz);
                }
                if m.message != wm::PAINT {
                    out.push((m.message, m.wparam, m.lparam, raw));
                }
                message::TranslateMessage(&m);
                message::DispatchMessageW(&m);
            }
        }
        out
    }

    fn u16_at(b: &[u8], o: usize) -> u16 {
        u16::from_le_bytes(b[o..o + 2].try_into().unwrap())
    }
    fn i32_at(b: &[u8], o: usize) -> i32 {
        i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
    }

    #[test]
    fn nsevents_to_legacy_and_raw_input() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let name = crate::strings::to_wide_z("Aw2TestNsEvents");
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: crate::user32::WNDPROC_HANDLE_DEF_WINDOW_W,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: name.as_ptr() as usize,
            };
            assert_ne!(class::RegisterClassW(&wc), 0);
            let h = window::CreateWindowExW(0, name.as_ptr(), std::ptr::null(), 0x8000_0000, 0, 0, 640, 480, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!h.is_null());
            window::ShowWindow(h, window::SW_SHOW);
            assert_eq!(inp::GetFocus() as usize, h as usize);
            let devs = [
                inp::RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: 0, target: h as usize },
                inp::RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: 0, target: h as usize },
            ];
            assert_eq!(inp::RegisterRawInputDevices(devs.as_ptr(), 2, 16), 1);
            drain(h);

            // 'A' (kVK_ANSI_A = 0x00): scan 0x1E, VK 'A', WM_CHAR 'a'.
            feed_key(&key_event(true, 0x00, "a", NSEventModifierFlags::empty()));
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).expect("WM_INPUT").3;
            assert_eq!(u32::from_le_bytes(raw[0..4].try_into().unwrap()), 1, "RIM_TYPEKEYBOARD");
            assert_eq!((u16_at(raw, 24), u16_at(raw, 26), u16_at(raw, 30)), (0x1E, 0, 0x41), "MakeCode/Flags/VKey");
            assert_eq!(i32_at(raw, 32) as u32, wm::KEYDOWN);
            let kd = m.iter().find(|x| x.0 == wm::KEYDOWN).expect("WM_KEYDOWN");
            assert_eq!((kd.1, (kd.2 >> 16) & 0x1FF), (0x41, 0x1E));
            assert!(m.iter().any(|x| x.0 == wm::CHAR && x.1 == 'a' as usize), "WM_CHAR via TranslateMessage: {m:?}");
            feed_key(&key_event(false, 0x00, "a", NSEventModifierFlags::empty()));
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).unwrap().3;
            assert_eq!(u16_at(raw, 26), 1, "RI_KEY_BREAK");

            // Right arrow (0x7C): extended scan 0x4D, RI_KEY_E0, VK_RIGHT.
            feed_key(&key_event(true, 0x7C, "\u{F703}", NSEventModifierFlags::empty()));
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).unwrap().3;
            assert_eq!((u16_at(raw, 24), u16_at(raw, 26), u16_at(raw, 30)), (0x4D, 2, 0x27));
            let kd = m.iter().find(|x| x.0 == wm::KEYDOWN).unwrap();
            assert_ne!(kd.2 & (1 << 24), 0, "extended bit");
            assert!(!m.iter().any(|x| x.0 == wm::CHAR), "no WM_CHAR for function keys");
            feed_key(&key_event(false, 0x7C, "", NSEventModifierFlags::empty()));
            drain(h);

            // Right shift via flagsChanged (device mask 0x4): scan 0x36, VK_SHIFT.
            feed_key(&flags_event(0x3C, NSEventModifierFlags(NSEventModifierFlags::Shift.0 | 0x4)));
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).unwrap().3;
            assert_eq!((u16_at(raw, 24), u16_at(raw, 26), u16_at(raw, 30)), (0x36, 0, 0x10));
            assert!(inp::GetAsyncKeyState(0xA1) < 0, "VK_RSHIFT down");
            feed_key(&flags_event(0x3C, NSEventModifierFlags::empty()));
            drain(h);
            assert!(inp::GetAsyncKeyState(0xA1) >= 0);

            // Cmd+Q stays with AppKit.
            assert!(matches!(translate_key(&key_event(true, 0x0C, "q", NSEventModifierFlags::Command)), KeyAction::Forward));

            // Mouse motion: CGEvent deltas -> NSEvent deltaX/Y -> RIM_TYPEMOUSE.
            let cg = CGEventCreateMouseEvent(std::ptr::null(), KCG_EVENT_MOUSE_MOVED, CGPoint { x: 100.0, y: 100.0 }, 0);
            CGEventSetIntegerValueField(cg, KCG_MOUSE_EVENT_DELTA_X, 7);
            CGEventSetIntegerValueField(cg, KCG_MOUSE_EVENT_DELTA_Y, -3);
            let ev = from_cg(cg);
            assert_eq!(move_delta(&ev), (7.0, -3.0));
            let (dx, dy) = move_delta(&ev);
            inp::native_mouse(NativeMouse::Move { hwnd: h as usize, client: Some(POINT { x: 10, y: 10 }), screen: POINT { x: 10, y: 10 }, dx, dy });
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).expect("raw mouse").3;
            assert_eq!(u32::from_le_bytes(raw[0..4].try_into().unwrap()), 0, "RIM_TYPEMOUSE");
            assert_eq!((u16_at(raw, 24), i32_at(raw, 36), i32_at(raw, 40)), (0, 7, -3), "relative lLastX/lLastY");
            assert!(m.iter().any(|x| x.0 == wm::MOUSEMOVE));

            // Wheel: one line up -> WM_MOUSEWHEEL +120 and RI_MOUSE_WHEEL.
            let ev = from_cg(CGEventCreateScrollWheelEvent2(std::ptr::null(), KCG_SCROLL_EVENT_UNIT_LINE, 1, 1, 0, 0));
            let (ny, _) = wheel_notches(&ev);
            assert!(ny > 0.0, "scroll up -> positive notches ({ny})");
            inp::native_mouse(NativeMouse::Wheel { screen: POINT { x: 10, y: 10 }, delta: 120, horizontal: false });
            let m = drain(h);
            let raw = &m.iter().find(|x| x.0 == wm::INPUT).unwrap().3;
            assert_eq!((u16_at(raw, 28), u16_at(raw, 30) as i16), (0x400, 120));
            let w = m.iter().find(|x| x.0 == wm::MOUSEWHEEL).unwrap();
            assert_eq!((w.1 >> 16) as i16, 120);

            // GCMouse deltas (unaccelerated) take over raw input from NSEvent.
            inp::native_raw_mouse(2.5, 1.5);
            inp::native_raw_mouse(0.5, 0.5);
            let m = drain(h);
            let d: Vec<(i32, i32)> = m.iter().filter(|x| x.0 == wm::INPUT).map(|x| (i32_at(&x.3, 36), i32_at(&x.3, 40))).collect();
            assert_eq!(d, vec![(2, 1), (1, 1)], "fractional GCMouse deltas accumulate");
            inp::native_mouse(NativeMouse::Move { hwnd: h as usize, client: Some(POINT { x: 12, y: 12 }), screen: POINT { x: 12, y: 12 }, dx: 5.0, dy: 5.0 });
            let m = drain(h);
            assert!(!m.iter().any(|x| x.0 == wm::INPUT), "NSEvent raw deltas suppressed while GCMouse delivers");
            assert!(m.iter().any(|x| x.0 == wm::MOUSEMOVE), "legacy WM_MOUSEMOVE still sent");

            let rm = [
                inp::RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: 1, target: 0 },
                inp::RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: 1, target: 0 },
            ];
            assert_eq!(inp::RegisterRawInputDevices(rm.as_ptr(), 2, 16), 1);
            window::DestroyWindow(h);
            class::UnregisterClassW(name.as_ptr(), std::ptr::null_mut());
        })
        .join()
        .unwrap();
    }

    #[test]
    fn focused_edit_key_routing_preserves_dialog_keys_and_key_state() {
        // 1. Escape & Tab always route to Win32 dialog handling regardless of ES_WANTRETURN.
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x35, false, false), EditKeyRoute::DialogToWin32);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x35, false, true), EditKeyRoute::DialogToWin32);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x30, false, false), EditKeyRoute::DialogToWin32);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x30, false, true), EditKeyRoute::DialogToWin32);

        // 2. Return / Keypad Enter route to Win32 unless ES_WANTRETURN is set.
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x24, false, false), EditKeyRoute::DialogToWin32);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x4C, false, false), EditKeyRoute::DialogToWin32);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x24, false, true), EditKeyRoute::AppKitEditWithWin32State);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x4C, false, true), EditKeyRoute::AppKitEditWithWin32State);

        // 3. Modifiers update both Win32 & AppKit; Command shortcuts go to AppKit; normal keys update Win32 state + AppKit.
        assert_eq!(classify_edit_key(NSEventType::FlagsChanged, 0x38, false, false), EditKeyRoute::ModifierBoth);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x00, true, false), EditKeyRoute::CommandToAppKit);
        assert_eq!(classify_edit_key(NSEventType::KeyDown, 0x00, false, false), EditKeyRoute::AppKitEditWithWin32State);

        // 4. End-to-end Win32 check: native_key_state_only updates key state without posting WM_CHAR,
        //    while native_key_for_hwnd(edit_hwnd, Escape) triggers IsDialogMessageW -> IDCANCEL.
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static LAST_CMD_ID: AtomicUsize = AtomicUsize::new(0);
            LAST_CMD_ID.store(0, Ordering::SeqCst);

            unsafe extern "win64" fn dlg_cmd_proc(
                h: HWND,
                m: u32,
                w: usize,
                l: isize,
            ) -> isize {
                if m == wm::COMMAND {
                    LAST_CMD_ID.store(w & 0xFFFF, Ordering::SeqCst);
                    return 0;
                }
                crate::user32::dialog::DefDlgProcW(h, m, w, l)
            }

            let cls = crate::strings::to_wide_z("TestEditDialogKeys");
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: crate::user32::register_typed_host_wndproc(dlg_cmd_proc),
                cls_extra: 0,
                wnd_extra: 30, // DLGWINDOWEXTRA
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = class::RegisterClassW(&wc);
            let hdlg = window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                crate::user32::ws::CAPTION | crate::user32::ws::VISIBLE,
                0,
                0,
                320,
                160,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_edit = window::CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("").as_ptr(),
                crate::user32::ws::CHILD | crate::user32::ws::VISIBLE | crate::user32::ws::TABSTOP,
                10,
                10,
                200,
                24,
                hdlg,
                101,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_cancel = window::CreateWindowExW(
                0,
                crate::strings::to_wide_z("BUTTON").as_ptr(),
                crate::strings::to_wide_z("Cancel").as_ptr(),
                crate::user32::ws::CHILD | crate::user32::ws::VISIBLE | crate::user32::ws::TABSTOP,
                10,
                50,
                80,
                28,
                hdlg,
                2, // IDCANCEL
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hdlg.is_null() && !h_edit.is_null() && !h_cancel.is_null());
            inp::SetFocus(h_edit);
            drain(hdlg);

            // Typing key via native_key_state_only updates GetAsyncKeyState & GetKeyState without queueing WM_KEYDOWN.
            inp::native_key_state_only(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            assert!(inp::GetAsyncKeyState(0x41) < 0);
            assert!(inp::GetKeyState(0x41) < 0);
            let mut m = MSG::default();
            assert_eq!(message::PeekMessageW(&mut m, std::ptr::null_mut(), wm::KEYDOWN, wm::CHAR, message::PM_REMOVE), 0);
            inp::native_key_state_only(NativeKey { mac: 0x00, down: false, repeat: false, chars: Vec::new() });
            assert!(inp::GetAsyncKeyState(0x41) >= 0);

            // Dialog Escape routed via native_key_for_hwnd(h_edit, Escape) is consumed by IsDialogMessageW -> IDCANCEL (2).
            inp::native_key_for_hwnd(h_edit as usize, NativeKey { mac: 0x35, down: true, repeat: false, chars: vec![0x1B] });
            assert_eq!(message::PeekMessageW(&mut m, std::ptr::null_mut(), wm::KEYDOWN, wm::KEYDOWN, message::PM_REMOVE), 1);
            assert_eq!(m.hwnd, h_edit as usize);
            assert_eq!(crate::user32::dialog::IsDialogMessageW(hdlg, &mut m), 1);
            assert_eq!(LAST_CMD_ID.load(Ordering::SeqCst), 2, "Escape must trigger IDCANCEL");
            inp::native_key_for_hwnd(h_edit as usize, NativeKey { mac: 0x35, down: false, repeat: false, chars: Vec::new() });

            // Dialog Tab routed via native_key_for_hwnd(h_edit, Tab) moves focus from h_edit to h_cancel.
            inp::native_key_for_hwnd(h_edit as usize, NativeKey { mac: 0x30, down: true, repeat: false, chars: vec![0x09] });
            assert_eq!(message::PeekMessageW(&mut m, std::ptr::null_mut(), wm::KEYDOWN, wm::KEYDOWN, message::PM_REMOVE), 1);
            assert_eq!(crate::user32::dialog::IsDialogMessageW(hdlg, &mut m), 1);
            assert_eq!(inp::GetFocus(), h_cancel, "Tab must move dialog focus to next tab stop");
            inp::native_key_for_hwnd(h_edit as usize, NativeKey { mac: 0x30, down: false, repeat: false, chars: Vec::new() });

            window::DestroyWindow(hdlg);
            class::UnregisterClassW(cls.as_ptr(), std::ptr::null_mut());
        })
        .join()
        .unwrap();
    }
}
