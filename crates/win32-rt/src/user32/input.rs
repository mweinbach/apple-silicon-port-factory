//! Input: activation/focus/capture, keyboard state, translation of native
//! (AppKit) input into queued Win32 input messages, and raw input.
//!
//! The `native_*` entry points run on the AppKit main thread: they only
//! touch atomics/mutexes and queue messages — no TEB, no guest code.

use super::keymap::{self, *};
use super::message::{self, InputMsg};
use super::window;
use super::*;
use crate::types::*;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Activation / focus / capture (process-wide; Windows tracks these per
// input context, which for a single-window game is the same thing)
// ---------------------------------------------------------------------------

static ACTIVE: AtomicUsize = AtomicUsize::new(0);
static FOCUS: AtomicUsize = AtomicUsize::new(0);
static CAPTURE: AtomicUsize = AtomicUsize::new(0);
static APP_ACTIVE: AtomicBool = AtomicBool::new(false);

const WA_INACTIVE: usize = 0;
const WA_ACTIVE: usize = 1;

pub fn active_window() -> usize {
    ACTIVE.load(Ordering::Acquire)
}

pub fn focus_window() -> usize {
    FOCUS.load(Ordering::Acquire)
}

pub fn app_active() -> bool {
    APP_ACTIVE.load(Ordering::Acquire)
}

/// Make `hwnd` the active window (runs on its owner thread). Sends
/// WM_NCACTIVATE/WM_ACTIVATE to the old window, WM_ACTIVATEAPP when the app
/// becomes active, WM_NCACTIVATE/WM_ACTIVATE (-> DefWindowProc focus) to
/// the new one. `from_native`: AppKit already made the window key.
pub fn set_active_window(hwnd: usize, from_native: bool) {
    let old = ACTIVE.load(Ordering::Acquire);
    if old == hwnd {
        if from_native && !APP_ACTIVE.swap(true, Ordering::AcqRel) {
            message::send_message(hwnd, wm::ACTIVATEAPP, 1, 0);
            super::misc::dispatch_win_event(0x0003 /* EVENT_SYSTEM_FOREGROUND */, hwnd, 0, 0);
            super::cursor::update_state();
        }
        return;
    }
    ACTIVE.store(hwnd, Ordering::Release);
    log::debug!("activate {hwnd:#x} (old {old:#x}, native={from_native})");
    if old != 0 && window::exists(old) {
        message::send_message(old, wm::NCACTIVATE, 0, 0);
        message::send_message(old, wm::ACTIVATE, WA_INACTIVE, hwnd as LPARAM);
    }
    if !APP_ACTIVE.swap(true, Ordering::AcqRel) {
        message::send_message(hwnd, wm::ACTIVATEAPP, 1, 0);
    }
    message::send_message(hwnd, wm::NCACTIVATE, 1, 0);
    message::send_message(hwnd, wm::ACTIVATE, WA_ACTIVE, old as LPARAM);
    super::misc::dispatch_win_event(0x0003 /* EVENT_SYSTEM_FOREGROUND */, hwnd, 0, 0);
    if FOCUS.load(Ordering::Acquire) == 0 || !belongs_to(FOCUS.load(Ordering::Acquire), hwnd) {
        set_focus(hwnd);
    }
    if !from_native {
        if let Some(w) = window::get(hwnd) {
            if w.data().native {
                crate::macos::window_activate(hwnd);
            }
        }
    }
    super::cursor::update_state();
}

fn belongs_to(h: usize, top: usize) -> bool {
    h == top || window::is_child_of(h, top)
}

/// AppKit says `hwnd` stopped being key (another app or window took over).
pub fn native_deactivated(hwnd: usize) {
    if ACTIVE.load(Ordering::Acquire) != hwnd {
        return;
    }
    ACTIVE.store(0, Ordering::Release);
    log::debug!("deactivate {hwnd:#x}");
    message::send_message(hwnd, wm::NCACTIVATE, 0, 0);
    message::send_message(hwnd, wm::ACTIVATE, WA_INACTIVE, 0);
    if APP_ACTIVE.swap(false, Ordering::AcqRel) {
        message::send_message(hwnd, wm::ACTIVATEAPP, 0, 0);
    }
    set_focus(0);
    release_all_keys();
    super::cursor::update_state();
}

/// Change the keyboard focus; returns the previous focus window.
pub fn set_focus(hwnd: usize) -> usize {
    let old = FOCUS.swap(hwnd, Ordering::AcqRel);
    if old == hwnd {
        return old;
    }
    if old != 0 && window::exists(old) {
        message::send_message(old, wm::KILLFOCUS, hwnd, 0);
    }
    if hwnd != 0 && FOCUS.load(Ordering::Acquire) == hwnd {
        message::send_message(hwnd, wm::SETFOCUS, old, 0);
        super::misc::dispatch_win_event(0x8005 /* EVENT_OBJECT_FOCUS */, hwnd, 0, 0);
    }
    old
}

/// Non-blocking activation/focus helper for `AW2_AUTO_PLAY` (safe to call from
/// the render/present thread without blocking on the window owner thread).
pub fn force_focus_for_autoplay(hwnd: usize) {
    if hwnd == 0 {
        return;
    }
    let was_app = APP_ACTIVE.swap(true, Ordering::AcqRel);
    let old_act = ACTIVE.swap(hwnd, Ordering::AcqRel);
    let old_foc = FOCUS.swap(hwnd, Ordering::AcqRel);
    if !was_app {
        post_input_to(hwnd, wm::ACTIVATEAPP, 1, 0, Vec::new());
    }
    if old_act != hwnd {
        post_input_to(hwnd, wm::NCACTIVATE, 1, 0, Vec::new());
        post_input_to(hwnd, wm::ACTIVATE, WA_ACTIVE, 0, Vec::new());
    }
    if old_foc != hwnd {
        post_input_to(hwnd, wm::SETFOCUS, 0, 0, Vec::new());
    }
}

/// A window is being destroyed.
pub fn window_going_away(hwnd: usize) {
    if CAPTURE.load(Ordering::Acquire) == hwnd {
        CAPTURE.store(0, Ordering::Release);
    }
    if FOCUS.load(Ordering::Acquire) == hwnd || window::is_child_of(FOCUS.load(Ordering::Acquire), hwnd) {
        FOCUS.store(0, Ordering::Release);
    }
    if ACTIVE.load(Ordering::Acquire) == hwnd {
        ACTIVE.store(0, Ordering::Release);
        super::cursor::update_state();
    }
}

/// A window was hidden or minimized: it loses activation.
pub fn window_hidden(hwnd: usize) {
    if ACTIVE.load(Ordering::Acquire) == hwnd {
        ACTIVE.store(0, Ordering::Release);
        message::send_message(hwnd, wm::NCACTIVATE, 0, 0);
        message::send_message(hwnd, wm::ACTIVATE, WA_INACTIVE, 0);
        set_focus(0);
        super::cursor::update_state();
    }
}

/// Forget a freed window everywhere.
pub fn forget_window(hwnd: usize) {
    for a in [&ACTIVE, &FOCUS, &CAPTURE] {
        let _ = a.compare_exchange(hwnd, 0, Ordering::AcqRel, Ordering::Acquire);
    }
    RAW_REGS.lock().unwrap().retain(|r| r.target != hwnd);
}

pub unsafe extern "win64" fn GetActiveWindow() -> HWND {
    ACTIVE.load(Ordering::Acquire) as HWND
}

pub unsafe extern "win64" fn GetForegroundWindow() -> HWND {
    if APP_ACTIVE.load(Ordering::Acquire) {
        ACTIVE.load(Ordering::Acquire) as HWND
    } else {
        // Another application is in front.
        std::ptr::null_mut()
    }
}

pub unsafe extern "win64" fn GetFocus() -> HWND {
    FOCUS.load(Ordering::Acquire) as HWND
}

fn top_level_of(hwnd: usize) -> usize {
    let mut cur = hwnd;
    for _ in 0..64 {
        let Some(w) = window::get(cur) else { return 0 };
        let d = w.data();
        if d.style & ws::CHILD == 0 {
            return cur;
        }
        cur = d.parent;
    }
    0
}

/// Run `f` on the owner thread of `hwnd` (directly if that is us).
fn on_owner(hwnd: usize, f: impl FnOnce() + Send + 'static) -> bool {
    let Some(w) = window::get(hwnd) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return false;
    };
    if w.tid == crate::teb::thread_id() {
        f();
        return true;
    }
    match message::queue_of(w.tid) {
        Some(q) => {
            q.call(f);
            true
        }
        None => false,
    }
}

pub unsafe extern "win64" fn SetActiveWindow(hwnd: HWND) -> HWND {
    let old = ACTIVE.load(Ordering::Acquire);
    let h = hwnd as usize;
    if h == 0 {
        return old as HWND;
    }
    if on_owner(h, move || set_active_window(h, false)) {
        old as HWND
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe extern "win64" fn SetForegroundWindow(hwnd: HWND) -> BOOL {
    let h = top_level_of(hwnd as usize);
    if h == 0 {
        return FALSE;
    }
    on_owner(h, move || set_active_window(h, false)) as BOOL
}

pub unsafe extern "win64" fn SetFocus(hwnd: HWND) -> HWND {
    let h = hwnd as usize;
    let old = FOCUS.load(Ordering::Acquire);
    if h == 0 {
        set_focus(0);
        return old as HWND;
    }
    if !window::exists(h) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return std::ptr::null_mut();
    }
    let top = top_level_of(h);
    on_owner(h, move || {
        if ACTIVE.load(Ordering::Acquire) != top {
            set_active_window(top, false);
        }
        set_focus(h);
    });
    old as HWND
}

pub unsafe extern "win64" fn GetCapture() -> HWND {
    CAPTURE.load(Ordering::Acquire) as HWND
}

pub unsafe extern "win64" fn SetCapture(hwnd: HWND) -> HWND {
    let old = CAPTURE.swap(hwnd as usize, Ordering::AcqRel);
    if old != 0 && old != hwnd as usize && window::exists(old) {
        message::send_message(old, wm::CAPTURECHANGED, 0, hwnd as LPARAM);
    }
    old as HWND
}

pub unsafe extern "win64" fn ReleaseCapture() -> BOOL {
    let old = CAPTURE.swap(0, Ordering::AcqRel);
    if old != 0 && window::exists(old) {
        message::send_message(old, wm::CAPTURECHANGED, 0, 0);
    }
    TRUE
}

#[repr(C)]
pub struct GUITHREADINFO {
    pub cb_size: u32,
    pub flags: u32,
    pub active: usize,
    pub focus: usize,
    pub capture: usize,
    pub menu_owner: usize,
    pub move_size: usize,
    pub caret: usize,
    pub rc_caret: RECT,
}
const _: () = assert!(std::mem::size_of::<GUITHREADINFO>() == 72);

pub unsafe extern "win64" fn GetGUIThreadInfo(tid: u32, gti: *mut GUITHREADINFO) -> BOOL {
    if gti.is_null() || (*gti).cb_size as usize != std::mem::size_of::<GUITHREADINFO>() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let owned = |h: usize| -> usize {
        if h == 0 {
            return 0;
        }
        match window::get(h) {
            Some(w) if tid == 0 || w.tid == tid => h,
            _ => 0,
        }
    };
    let g = &mut *gti;
    g.flags = 0;
    g.active = owned(ACTIVE.load(Ordering::Acquire));
    g.focus = owned(FOCUS.load(Ordering::Acquire));
    g.capture = owned(CAPTURE.load(Ordering::Acquire));
    g.menu_owner = 0;
    g.move_size = 0;
    g.caret = 0;
    g.rc_caret = RECT::default();
    TRUE
}

// ---------------------------------------------------------------------------
// Keyboard state
// ---------------------------------------------------------------------------

/// Live ("async") key state: 0x80 = down, 0x01 = toggled.
static ASYNC: [AtomicU8; 256] = [const { AtomicU8::new(0) }; 256];
/// State as of the last input message retrieved (GetKeyState).
static SYNC: Mutex<[u8; 256]> = Mutex::new([0; 256]);
/// Set when the async state changed since GetAsyncKeyState last looked (bit 0).
static PRESSED_SINCE: [AtomicU8; 256] = [const { AtomicU8::new(0) }; 256];

fn async_down(vk: u8) -> bool {
    ASYNC[vk as usize].load(Ordering::Relaxed) & 0x80 != 0
}

fn set_async(vk: u8, down: bool) {
    let a = &ASYNC[vk as usize];
    if down {
        let prev = a.fetch_or(0x80, Ordering::Relaxed);
        if prev & 0x80 == 0 {
            a.fetch_xor(0x01, Ordering::Relaxed);
            PRESSED_SINCE[vk as usize].store(1, Ordering::Relaxed);
        }
    } else {
        a.fetch_and(!0x80, Ordering::Relaxed);
    }
}

fn set_async_key(k: &keymap::Key, down: bool) {
    set_async(k.vk_lr, down);
    if k.vk != k.vk_lr {
        // Generic modifier = either side down.
        let (l, r) = match k.vk {
            VK_SHIFT => (VK_LSHIFT, VK_RSHIFT),
            VK_CONTROL => (VK_LCONTROL, VK_RCONTROL),
            _ => (VK_LMENU, VK_RMENU),
        };
        set_async(k.vk, async_down(l) || async_down(r));
    }
}

pub fn shift_down() -> bool {
    async_down(VK_SHIFT)
}

pub fn caps_on() -> bool {
    ASYNC[VK_CAPITAL as usize].load(Ordering::Relaxed) & 1 != 0
}

pub(crate) fn release_all_keys() {
    for (i, a) in ASYNC.iter().enumerate() {
        if i != VK_CAPITAL as usize && i != VK_NUMLOCK as usize {
            a.fetch_and(!0x80, Ordering::Relaxed);
        }
    }
    let mut s = SYNC.lock().unwrap();
    for (i, v) in s.iter_mut().enumerate() {
        if i != VK_CAPITAL as usize && i != VK_NUMLOCK as usize {
            *v &= !0x80;
        }
    }
}

/// Called on the Win32 thread when it removes an input message.
pub fn on_input_retrieved(m: &MSG) {
    match m.message {
        wm::KEYDOWN | wm::SYSKEYDOWN | wm::KEYUP | wm::SYSKEYUP => {
            let down = matches!(m.message, wm::KEYDOWN | wm::SYSKEYDOWN);
            let scan = ((m.lparam >> 16) & 0xFF) as u32;
            let ext = (m.lparam >> 24) & 1 != 0;
            let vk = m.wparam as u8;
            let lr = keymap::map_virtual_key(if ext { 0xE000 | scan } else { scan }, MAPVK_VSC_TO_VK_EX) as u8;
            let mut s = SYNC.lock().unwrap();
            for v in [vk, lr] {
                if v == 0 {
                    continue;
                }
                let e = &mut s[v as usize];
                if down {
                    if *e & 0x80 == 0 {
                        *e ^= 1;
                    }
                    *e |= 0x80;
                } else {
                    *e &= !0x80;
                }
            }
        }
        m2 if (wm::MOUSEFIRST..=wm::MOUSELAST).contains(&m2) => {
            {
                let mut s = SYNC.lock().unwrap();
                for (vk, bit) in [(VK_LBUTTON, 1usize), (VK_RBUTTON, 2), (VK_MBUTTON, 0x10), (VK_XBUTTON1, 0x20), (VK_XBUTTON2, 0x40)] {
                    if m.wparam & bit != 0 {
                        s[vk as usize] |= 0x80;
                    } else {
                        s[vk as usize] &= !0x80;
                    }
                }
            }
            if CAPTURE.load(Ordering::Acquire) == 0 && m.hwnd != 0 && m2 != wm::MOUSEWHEEL && m2 != wm::MOUSEHWHEEL {
                message::send_message(m.hwnd, wm::SETCURSOR, m.hwnd, make_long(super::defproc::HTCLIENT as i32, m2 as i32));
            }
        }
        _ => {}
    }
}

pub unsafe extern "win64" fn GetKeyState(vk: i32) -> i16 {
    let s = SYNC.lock().unwrap();
    let v = s[(vk & 0xFF) as usize];
    (if v & 0x80 != 0 { 0x8000u16 as i16 } else { 0 }) | (v & 1) as i16
}

pub unsafe extern "win64" fn GetKeyboardState(out: *mut u8) -> BOOL {
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let s = SYNC.lock().unwrap();
    std::ptr::copy_nonoverlapping(s.as_ptr(), out, 256);
    TRUE
}

pub unsafe extern "win64" fn GetAsyncKeyState(vk: i32) -> i16 {
    let i = (vk & 0xFF) as usize;
    let down = ASYNC[i].load(Ordering::Relaxed) & 0x80 != 0;
    let pressed = PRESSED_SINCE[i].swap(0, Ordering::Relaxed) != 0;
    (if down { 0x8000u16 as i16 } else { 0 }) | pressed as i16
}

pub unsafe extern "win64" fn MapVirtualKeyA(code: u32, ty: u32) -> u32 {
    keymap::map_virtual_key(code, ty)
}

pub unsafe extern "win64" fn MapVirtualKeyW(code: u32, ty: u32) -> u32 {
    keymap::map_virtual_key(code, ty)
}

pub unsafe extern "win64" fn MapVirtualKeyExW(code: u32, ty: u32, _hkl: usize) -> u32 {
    keymap::map_virtual_key(code, ty)
}

/// Persistent dead-key state across `ToUnicode` / `ToUnicodeEx` calls (cleared
/// on composition or reset; preserved when `wFlags & 0x4` is set).
static TO_UNICODE_DEAD_KEY: Mutex<u32> = Mutex::new(0);

unsafe fn write_utf16_result(chars: &[u16], buf: *mut u16, cap: i32) -> i32 {
    if cap < 1 || buf.is_null() || chars.is_empty() {
        return 0;
    }
    let n = chars.len().min(cap as usize);
    for (i, &u) in chars[..n].iter().enumerate() {
        buf.add(i).write_unaligned(u);
    }
    if n < cap as usize {
        buf.add(n).write_unaligned(0);
    }
    n as i32
}

fn normalize_macos_chars(vk: u32, chars: Vec<u16>) -> Vec<u16> {
    chars
        .into_iter()
        .filter_map(|c| match c {
            0xF700..=0xF8FF => None, // Apple function-key private use
            0x03 => Some(0x0D),      // keypad Enter -> CR
            0x19 => Some(0x09),      // Shift+Tab -> Tab
            0x7F if vk == VK_BACK as u32 => Some(0x08),
            c => Some(c),
        })
        .collect()
}

/// `ToUnicode` backed by the active macOS keyboard layout (`UCKeyTranslate`
/// over the cached `'uchr'` table), falling back to the static US layout table
/// when headless/unavailable or for unmapped keys.
pub unsafe extern "win64" fn ToUnicode(vk: u32, scan: u32, state: *const u8, buf: *mut u16, cap: i32, flags: u32) -> i32 {
    // High bit of scan code signals key-up (no translation on Windows).
    if scan & 0x8000 != 0 || !keymap::is_character_vk(vk) {
        return 0;
    }
    let (shift, caps, ctrl, alt) = if state.is_null() {
        (false, false, false, false)
    } else {
        let s = std::slice::from_raw_parts(state, 256);
        let shift = (s[VK_SHIFT as usize] | s[VK_LSHIFT as usize] | s[VK_RSHIFT as usize]) & 0x80 != 0;
        let caps = s[VK_CAPITAL as usize] & 1 != 0;
        let ctrl = (s[VK_CONTROL as usize] | s[VK_LCONTROL as usize] | s[VK_RCONTROL as usize]) & 0x80 != 0;
        let alt = (s[VK_MENU as usize] | s[VK_LMENU as usize] | s[VK_RMENU as usize]) & 0x80 != 0;
        (shift, caps, ctrl, alt)
    };
    // Windows 10 1607+ flag: do not modify keyboard / dead-key state.
    let preserve_state = flags & 0x0004 != 0;
    // Windows AltGr (Ctrl+Alt) and macOS Option both map to macOS optionKey;
    // avoid passing controlKey alongside AltGr so third-level glyphs resolve.
    let option = alt;
    let effective_ctrl = ctrl && !alt;

    if let Some(mac) = keymap::vk_to_mac(vk, scan) {
        let mut dead_guard = TO_UNICODE_DEAD_KEY.lock().unwrap();
        let outcome = crate::macos::keyboard::translate_key(
            mac,
            shift,
            caps,
            option,
            effective_ctrl,
            preserve_state,
            &mut dead_guard,
        );
        drop(dead_guard);
        match outcome {
            crate::macos::keyboard::TranslateOutcome::Chars(raw) => {
                let chars = normalize_macos_chars(vk, raw);
                return write_utf16_result(&chars, buf, cap);
            }
            crate::macos::keyboard::TranslateOutcome::DeadKey(raw) => {
                let chars = normalize_macos_chars(vk, raw);
                let _ = write_utf16_result(&chars, buf, cap);
                return -1;
            }
            crate::macos::keyboard::TranslateOutcome::None => {
                return 0;
            }
            crate::macos::keyboard::TranslateOutcome::Unavailable => {
                // Fall through to the static US layout table below.
            }
        }
    }

    let c = match keymap::us_char(vk, shift, caps) {
        Some(c) if effective_ctrl && (b'a' as u16..=b'z' as u16).contains(&(c | 0x20)) => (c | 0x20) - b'a' as u16 + 1,
        Some(c) => c,
        None => return 0,
    };
    write_utf16_result(&[c], buf, cap)
}

pub unsafe extern "win64" fn ToUnicodeEx(vk: u32, scan: u32, state: *const u8, buf: *mut u16, cap: i32, flags: u32, _hkl: usize) -> i32 {
    ToUnicode(vk, scan, state, buf, cap, flags)
}

pub unsafe extern "win64" fn GetKeyboardLayout(_tid: u32) -> usize {
    0x0409_0409 // en-US
}

pub unsafe extern "win64" fn GetKeyboardLayoutList(n_buff: i32, list: *mut usize) -> i32 {
    if n_buff == 0 || list.is_null() {
        return 1;
    }
    if n_buff < 0 {
        return 0;
    }
    std::ptr::write_unaligned(list, 0x0409_0409);
    1
}

pub unsafe extern "win64" fn GetKeyboardLayoutNameW(buf: *mut u16) -> BOOL {
    if buf.is_null() {
        return FALSE;
    }
    crate::strings::write_wide_buf("00000409", buf, 9);
    TRUE
}

pub unsafe extern "win64" fn GetKeyboardLayoutNameA(buf: *mut u8) -> BOOL {
    if buf.is_null() {
        return FALSE;
    }
    crate::strings::write_ansi_buf("00000409", buf, 9);
    TRUE
}

pub unsafe extern "win64" fn LoadKeyboardLayoutW(_name: LPCWSTR, _flags: u32) -> usize {
    0x0409_0409
}

pub unsafe extern "win64" fn LoadKeyboardLayoutA(_name: LPCSTR, _flags: u32) -> usize {
    0x0409_0409
}

pub unsafe extern "win64" fn ActivateKeyboardLayout(_hkl: usize, _flags: u32) -> usize {
    0x0409_0409
}

pub unsafe extern "win64" fn UnloadKeyboardLayout(_hkl: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn VkKeyScanW(ch: u16) -> i16 {
    let c = ch as u8;
    match c {
        b'a'..=b'z' => (c - b'a' + b'A') as i16,
        b'A'..=b'Z' => (c as i16) | 0x0100,
        b'0'..=b'9' | b' ' | b'\r' | b'\t' | b'\x08' => c as i16,
        _ => -1,
    }
}

pub unsafe extern "win64" fn VkKeyScanExW(ch: u16, _hkl: usize) -> i16 {
    VkKeyScanW(ch)
}

pub unsafe extern "win64" fn VkKeyScanA(ch: u8) -> i16 {
    VkKeyScanW(ch as u16)
}

pub unsafe extern "win64" fn VkKeyScanExA(ch: u8, _hkl: usize) -> i16 {
    VkKeyScanW(ch as u16)
}

pub unsafe extern "win64" fn GetKeyNameTextA(lparam: i32, buf: *mut u8, cap: i32) -> i32 {
    if buf.is_null() || cap <= 0 {
        return 0;
    }
    let mut w = [0u16; 64];
    let n = GetKeyNameTextW(lparam, w.as_mut_ptr(), w.len() as i32);
    if n <= 0 {
        return 0;
    }
    let s = String::from_utf16_lossy(&w[..n as usize]);
    crate::strings::write_ansi_buf(&s, buf, cap as usize);
    s.len().min((cap as usize).saturating_sub(1)) as i32
}

pub unsafe extern "win64" fn GetKeyNameTextW(lparam: i32, buf: *mut u16, cap: i32) -> i32 {
    let scan = ((lparam >> 16) & 0xFF) as u32;
    let ext = (lparam >> 24) & 1 != 0;
    let vk = keymap::map_virtual_key(if ext { 0xE000 | scan } else { scan }, MAPVK_VSC_TO_VK_EX);
    let name = match keymap::us_char(vk, true, false) {
        Some(c) if c > 0x20 => String::from_utf16_lossy(&[c]),
        _ => format!("VK_{vk:02X}"),
    };
    if buf.is_null() || cap <= 0 {
        return 0;
    }
    crate::strings::write_wide_trunc(&name, buf, cap as usize).1 as i32
}

// ---------------------------------------------------------------------------
// Native input (main thread)
// ---------------------------------------------------------------------------

/// A key event from AppKit.
#[derive(Clone, Debug)]
pub struct NativeKey {
    pub mac: u16,
    pub down: bool,
    pub repeat: bool,
    /// UTF-16 text produced by the key (layout-correct), for WM_CHAR.
    pub chars: Vec<u16>,
}

fn cursor_screen() -> POINT {
    super::cursor::cursor_pos()
}

fn post_input_to(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM, chars: Vec<u16>) {
    let _ = post_input_to_checked(hwnd, msg, wparam, lparam, chars, false);
}

fn post_input_to_checked(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM, chars: Vec<u16>, ll_checked: bool) -> bool {
    let Some(w) = window::get(hwnd) else { return false };
    let Some(q) = message::queue_of(w.tid) else { return false };
    q.post_input(InputMsg {
        msg: MSG { hwnd, message: msg, wparam, lparam, time: tick_ms(), pt: cursor_screen(), l_private: 0 },
        chars,
        ll_checked,
        seq: 0,
    });
    true
}

/// Queue a deferred `WH_KEYBOARD_LL` callback onto the hook-owning Win32
/// thread's message queue when an event arrives on a non-Win32 thread (such as
/// the AppKit main thread) and no legacy keyboard message is queued (e.g.
/// `RIDEV_NOLEGACY`, hotkey interception, or unfocused input).
#[allow(clippy::too_many_arguments)]
fn defer_keyboard_ll_hook(
    fallback_hwnd: usize,
    msg: u32,
    vk: u32,
    scan: u32,
    ext: bool,
    alt: bool,
    up: bool,
    time: u32,
) {
    if !super::misc::has_hooks(super::misc::WH_KEYBOARD_LL) {
        return;
    }
    let q = super::misc::ll_hook_owner_tid(super::misc::WH_KEYBOARD_LL)
        .map(message::queue_for)
        .or_else(|| window::get(fallback_hwnd).and_then(|w| message::queue_of(w.tid)));
    if let Some(q) = q {
        q.call(move || {
            let _ = dispatch_keyboard_ll_hook(msg, vk, scan, ext, alt, up, time);
        });
    }
}

/// Queue a deferred `WH_MOUSE_LL` callback onto the hook-owning Win32 thread's
/// message queue when an event arrives on a non-Win32 thread and no legacy
/// mouse message is queued (e.g. `RIDEV_NOLEGACY`, relative mouse mode, or
/// unfocused input).
fn defer_mouse_ll_hook(
    fallback_hwnd: usize,
    msg: u32,
    pt: POINT,
    mouse_data: u32,
    time: u32,
) {
    if !super::misc::has_hooks(super::misc::WH_MOUSE_LL) {
        return;
    }
    let q = super::misc::ll_hook_owner_tid(super::misc::WH_MOUSE_LL)
        .map(message::queue_for)
        .or_else(|| window::get(fallback_hwnd).and_then(|w| message::queue_of(w.tid)));
    if let Some(q) = q {
        q.call(move || {
            let _ = dispatch_mouse_ll_hook(msg, pt, mouse_data, time);
        });
    }
}

fn is_keyboard_msg(msg: u32) -> bool {
    matches!(msg, wm::KEYDOWN | wm::KEYUP | wm::SYSKEYDOWN | wm::SYSKEYUP)
}

fn is_mouse_msg(msg: u32) -> bool {
    (wm::MOUSEFIRST..=wm::MOUSELAST).contains(&msg)
}

fn mouse_msg_data(m: &MSG) -> u32 {
    if matches!(
        m.message,
        wm::MOUSEWHEEL | wm::MOUSEHWHEEL | wm::XBUTTONDOWN | wm::XBUTTONUP | wm::XBUTTONDBLCLK
    ) {
        (m.wparam & 0xFFFF_0000) as u32
    } else {
        0
    }
}

fn ll_mouse_msg(msg: u32) -> u32 {
    match msg {
        wm::LBUTTONDBLCLK => wm::LBUTTONDOWN,
        wm::RBUTTONDBLCLK => wm::RBUTTONDOWN,
        wm::MBUTTONDBLCLK => wm::MBUTTONDOWN,
        wm::XBUTTONDBLCLK => wm::XBUTTONDOWN,
        other => other,
    }
}

/// Dispatch `WH_KEYBOARD_LL` hooks. Returns `true` if a hook suppressed the event.
pub(crate) fn dispatch_keyboard_ll_hook(
    msg: u32,
    vk: u32,
    scan: u32,
    ext: bool,
    alt: bool,
    up: bool,
    time: u32,
) -> bool {
    if !super::misc::has_hooks(super::misc::WH_KEYBOARD_LL)
        || !super::misc::can_invoke_guest_hooks_on_this_thread()
    {
        return false;
    }
    let mut flags = 0u32;
    if ext {
        flags |= 0x01; // LLKHF_EXTENDED
    }
    if alt {
        flags |= 0x20; // LLKHF_ALTDOWN
    }
    if up {
        flags |= 0x80; // LLKHF_UP
    }
    let ks = super::misc::KBDLLHOOKSTRUCT {
        vk_code: vk,
        scan_code: scan,
        flags,
        time,
        dw_extra_info: 0,
    };
    let tid = crate::teb::thread_id();
    super::misc::dispatch_hook(
        super::misc::WH_KEYBOARD_LL,
        tid,
        super::misc::HC_ACTION,
        msg as WPARAM,
        &ks as *const super::misc::KBDLLHOOKSTRUCT as LPARAM,
    ) != 0
}

/// Dispatch `WH_MOUSE_LL` hooks. Returns `true` if a hook suppressed the event.
pub(crate) fn dispatch_mouse_ll_hook(msg: u32, pt: POINT, mouse_data: u32, time: u32) -> bool {
    if !super::misc::has_hooks(super::misc::WH_MOUSE_LL)
        || !super::misc::can_invoke_guest_hooks_on_this_thread()
    {
        return false;
    }
    let ms = super::misc::MSLLHOOKSTRUCT {
        pt,
        mouse_data,
        flags: 0,
        time,
        _pad: 0,
        dw_extra_info: 0,
    };
    let tid = crate::teb::thread_id();
    super::misc::dispatch_hook(
        super::misc::WH_MOUSE_LL,
        tid,
        super::misc::HC_ACTION,
        msg as WPARAM,
        &ms as *const super::misc::MSLLHOOKSTRUCT as LPARAM,
    ) != 0
}

/// Dispatch `WH_KEYBOARD` hooks. Returns `true` if a hook suppressed the event.
pub(crate) fn dispatch_keyboard_hook(m: &MSG, remove: bool) -> bool {
    if !super::misc::has_hooks(super::misc::WH_KEYBOARD)
        || !super::misc::can_invoke_guest_hooks_on_this_thread()
    {
        return false;
    }
    let code = if remove { super::misc::HC_ACTION } else { super::misc::HC_NOREMOVE };
    let tid = crate::teb::thread_id();
    super::misc::dispatch_hook(super::misc::WH_KEYBOARD, tid, code, m.wparam, m.lparam) != 0
}

/// Dispatch `WH_MOUSE` hooks. Returns `true` if a hook suppressed the event.
pub(crate) fn dispatch_mouse_hook(m: &MSG, remove: bool) -> bool {
    if !super::misc::has_hooks(super::misc::WH_MOUSE)
        || !super::misc::can_invoke_guest_hooks_on_this_thread()
    {
        return false;
    }
    let code = if remove { super::misc::HC_ACTION } else { super::misc::HC_NOREMOVE };
    let ms = super::misc::MOUSEHOOKSTRUCTEX {
        pt: m.pt,
        hwnd: m.hwnd,
        w_hit_test_code: super::defproc::HTCLIENT as u32,
        _pad0: 0,
        dw_extra_info: 0,
        mouse_data: mouse_msg_data(m),
        _pad1: 0,
    };
    let tid = crate::teb::thread_id();
    super::misc::dispatch_hook(
        super::misc::WH_MOUSE,
        tid,
        code,
        m.message as WPARAM,
        &ms as *const super::misc::MOUSEHOOKSTRUCTEX as LPARAM,
    ) != 0
}

/// Run keyboard/mouse hooks for a queued hardware-input message on the Win32
/// thread. Returns `true` if the event was suppressed and should be discarded.
pub(crate) fn filter_input_msg(m: &MSG, remove: bool, ll_already_checked: bool) -> bool {
    if is_keyboard_msg(m.message) {
        if !ll_already_checked {
            let vk = m.wparam as u32;
            let scan = ((m.lparam >> 16) & 0xFF) as u32;
            let ext = (m.lparam >> 24) & 1 != 0;
            let alt = (m.lparam >> 29) & 1 != 0;
            let up = matches!(m.message, wm::KEYUP | wm::SYSKEYUP) || ((m.lparam >> 31) & 1 != 0);
            if dispatch_keyboard_ll_hook(m.message, vk, scan, ext, alt, up, m.time) {
                return true;
            }
        }
        return dispatch_keyboard_hook(m, remove);
    }
    if is_mouse_msg(m.message) {
        if !ll_already_checked
            && dispatch_mouse_ll_hook(ll_mouse_msg(m.message), m.pt, mouse_msg_data(m), m.time)
        {
            return true;
        }
        return dispatch_mouse_hook(m, remove);
    }
    false
}

/// Run `WH_KEYBOARD` / `WH_MOUSE` hooks for a message posted via `PostMessage`.
/// Returns `true` if the event was suppressed and should be discarded.
pub(crate) fn filter_posted_msg(m: &MSG, remove: bool) -> bool {
    if is_keyboard_msg(m.message) {
        return dispatch_keyboard_hook(m, remove);
    }
    if is_mouse_msg(m.message) {
        return dispatch_mouse_hook(m, remove);
    }
    false
}

pub fn native_key(k: NativeKey) {
    let Some(key) = keymap::from_mac(k.mac) else {
        log::trace!("unmapped mac key code {:#x}", k.mac);
        return;
    };
    let was_down = async_down(key.vk_lr);
    if key.vk == VK_CAPITAL || key.mac == 0x47 {
        // Lock keys report press+release per toggle.
    }
    let alt_after = if key.vk == VK_MENU { k.down } else { async_down(VK_MENU) };
    let ctrl_after = if key.vk == VK_CONTROL { k.down } else { async_down(VK_CONTROL) };
    let sys = (alt_after && !ctrl_after) || key.vk == VK_F10 || (key.vk == VK_MENU && !ctrl_after);
    let msg = match (k.down, sys) {
        (true, false) => wm::KEYDOWN,
        (true, true) => wm::SYSKEYDOWN,
        (false, false) => wm::KEYUP,
        (false, true) => wm::SYSKEYUP,
    };
    let time = tick_ms();
    let ll_checked = if super::misc::can_invoke_guest_hooks_on_this_thread() {
        if dispatch_keyboard_ll_hook(msg, key.vk as u32, key.scan as u32, key.ext, alt_after, !k.down, time) {
            return;
        }
        true
    } else {
        false
    };
    set_async_key(key, k.down);
    let alt = async_down(VK_MENU);
    // Registered hotkeys (RegisterHotKey) intercept keydown before WM_KEYDOWN,
    // posting WM_HOTKEY to the registering window/thread — unless the foreground
    // raw-keyboard registration opted out via RIDEV_NOHOTKEYS.
    if k.down && !raw_nohotkeys_active() {
        let mut mods = 0u32;
        if alt {
            mods |= 0x0001; // MOD_ALT
        }
        if async_down(VK_CONTROL) {
            mods |= 0x0002; // MOD_CONTROL
        }
        if async_down(VK_SHIFT) {
            mods |= 0x0004; // MOD_SHIFT
        }
        if async_down(0x5B) || async_down(0x5C) {
            mods |= 0x0008; // MOD_WIN (VK_LWIN / VK_RWIN)
        }
        if super::misc::try_dispatch_hotkey(mods, key.vk as u32, was_down || k.repeat) {
            if !ll_checked {
                defer_keyboard_ll_hook(0, msg, key.vk as u32, key.scan as u32, key.ext, alt_after, !k.down, time);
            }
            return;
        }
    }
    let target = FOCUS.load(Ordering::Acquire);
    if target == 0 {
        if !ll_checked {
            defer_keyboard_ll_hook(0, msg, key.vk as u32, key.scan as u32, key.ext, alt_after, !k.down, time);
        }
        return;
    }
    // Raw input first (Windows delivers WM_INPUT ahead of the legacy message).
    raw_keyboard(key, k.down, msg);
    if raw_nolegacy_active(1, 6) {
        if !ll_checked {
            defer_keyboard_ll_hook(target, msg, key.vk as u32, key.scan as u32, key.ext, alt_after, !k.down, time);
        }
        return;
    }
    let mut lp: u32 = 1 | (key.scan as u32) << 16;
    if key.ext {
        lp |= 1 << 24;
    }
    if alt {
        lp |= 1 << 29;
    }
    if k.down {
        if was_down || k.repeat {
            lp |= 1 << 30;
        }
    } else {
        lp |= 1 << 30 | 1 << 31;
    }
    let chars: Vec<u16> = if k.down {
        k.chars
            .into_iter()
            .filter_map(|c| match c {
                0xF700..=0xF8FF => None, // AppKit function-key private use
                0x7F => Some(0x08),      // Backspace
                0x03 => Some(0x0D),      // keypad Enter
                0x19 => Some(0x09),      // Shift+Tab
                c => Some(c),
            })
            .collect()
    } else {
        Vec::new()
    };
    if !post_input_to_checked(target, msg, key.vk as usize, lp as i32 as isize, chars, ll_checked) && !ll_checked {
        defer_keyboard_ll_hook(target, msg, key.vk as u32, key.scan as u32, key.ext, alt_after, !k.down, time);
    }
}

/// Modifier key transitions from `flagsChanged:` are delivered as NativeKey too.
pub fn native_caps_toggle(mac: u16) {
    native_key(NativeKey { mac, down: true, repeat: false, chars: Vec::new() });
    native_key(NativeKey { mac, down: false, repeat: false, chars: Vec::new() });
}

fn ascii_char_to_mac(ch: char) -> Option<u16> {
    let b = ch as u32;
    if b > 0x7F {
        return None;
    }
    match b as u8 {
        b'\r' | b'\n' => Some(0x24),
        b'\t' => Some(0x30),
        0x08 | 0x7F => Some(0x33),
        0x1B => Some(0x35),
        b' ' => Some(0x31),
        _ => {
            for k in keymap::KEYS {
                for shift in [false, true] {
                    if keymap::us_char(k.vk as u32, shift, false) == Some(b as u16) {
                        return Some(k.mac);
                    }
                }
            }
            None
        }
    }
}

/// Deliver Unicode text entered via a native software keyboard (e.g. iOS `UIKeyInput` /
/// IME / dictation / autocorrect) into the focused (or active) Win32 window.
///
/// ASCII characters with a known physical key emit `WM_KEYDOWN` (with `chars` for
/// `WM_CHAR`) followed by `WM_KEYUP` so games polling `WM_KEYDOWN`, `GetAsyncKeyState`,
/// or `WM_CHAR` all receive the keystroke; non-ASCII UTF-16 code units post `WM_CHAR`
/// directly.
pub fn native_text(text: &str) {
    for ch in text.chars() {
        let normalized = if ch == '\n' { '\r' } else { ch };
        if let Some(mac) = ascii_char_to_mac(normalized) {
            let unit = normalized as u16;
            native_key(NativeKey { mac, down: true, repeat: false, chars: vec![unit] });
            native_key(NativeKey { mac, down: false, repeat: false, chars: Vec::new() });
        } else {
            let target = {
                let f = FOCUS.load(Ordering::Acquire);
                if f != 0 { f } else { ACTIVE.load(Ordering::Acquire) }
            };
            if target == 0 {
                continue;
            }
            let mut buf = [0u16; 2];
            for unit in normalized.encode_utf16(&mut buf).iter().copied() {
                post_input_to(target, wm::CHAR, unit as usize, 1, Vec::new());
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

#[derive(Clone, Copy, Debug)]
pub enum NativeMouse {
    /// `hwnd`/`client`: window under the event (0/None if not ours).
    /// `screen`: Win32 screen pixels. `dx`/`dy`: relative motion (points).
    Move { hwnd: usize, client: Option<POINT>, screen: POINT, dx: f64, dy: f64 },
    Button { hwnd: usize, client: Option<POINT>, screen: POINT, button: MouseButton, down: bool, clicks: i64 },
    /// `delta` in WHEEL_DELTA units (120 per notch).
    Wheel { screen: POINT, delta: i32, horizontal: bool },
}

fn mk_state() -> usize {
    let mut w = 0;
    for (vk, bit) in [(VK_LBUTTON, 1usize), (VK_RBUTTON, 2), (VK_SHIFT, 4), (VK_CONTROL, 8), (VK_MBUTTON, 0x10), (VK_XBUTTON1, 0x20), (VK_XBUTTON2, 0x40)] {
        if async_down(vk) {
            w |= bit;
        }
    }
    w
}

fn client_of(hwnd: usize, screen: POINT) -> Option<POINT> {
    let o = window::client_origin_screen(hwnd)?;
    Some(POINT { x: screen.x - o.x, y: screen.y - o.y })
}

static RAW_ACC: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
static RAW_ACC_GC: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
/// `tick_ms()` of the last GCMouse delta (0 = never).
static GC_RAW_LAST: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Where `RIM_TYPEMOUSE` deltas come from (`AW2_RAW_MOUSE`):
/// * `auto` (default): GCMouse (unaccelerated device counts, like Windows
///   raw input) while it is delivering, NSEvent deltas (accelerated
///   points) otherwise. Absolute pointers, such as KVMs and tablets,
///   produce no GCMouse deltas.
/// * `gc`: GCMouse only.
/// * `nsevent`: NSEvent only.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RawSource {
    Auto,
    Gc,
    NsEvent,
}

fn raw_source() -> RawSource {
    static S: std::sync::OnceLock<RawSource> = std::sync::OnceLock::new();
    *S.get_or_init(|| match std::env::var("AW2_RAW_MOUSE").as_deref() {
        Ok("gc") => RawSource::Gc,
        Ok("nsevent") => RawSource::NsEvent,
        _ => RawSource::Auto,
    })
}

/// NSEvent deltas feed raw input unless GCMouse delivered within 500 ms.
fn nsevent_raw_active(now: u32) -> bool {
    match raw_source() {
        RawSource::NsEvent => true,
        RawSource::Gc => false,
        RawSource::Auto => {
            let last = GC_RAW_LAST.load(Ordering::Acquire);
            last == 0 || now.wrapping_sub(last) > 500
        }
    }
}

fn accumulate(acc: &Mutex<(f64, f64)>, dx: f64, dy: f64) -> Option<(i32, i32)> {
    let mut a = acc.lock().unwrap();
    a.0 += dx;
    a.1 += dy;
    let (ix, iy) = (a.0.trunc(), a.1.trunc());
    a.0 -= ix;
    a.1 -= iy;
    (ix != 0.0 || iy != 0.0).then_some((ix as i32, iy as i32))
}

/// Raw mouse motion from GCMouse (main thread), +Y down.
pub fn native_raw_mouse(dx: f64, dy: f64) {
    if raw_source() == RawSource::NsEvent {
        return;
    }
    GC_RAW_LAST.store(tick_ms().max(1), Ordering::Release);
    if let Some((x, y)) = accumulate(&RAW_ACC_GC, dx, dy) {
        raw_mouse(x, y, 0, 0);
    }
}

pub fn native_mouse(ev: NativeMouse) {
    match ev {
        NativeMouse::Move { hwnd, client, screen, dx, dy } => {
            let time = tick_ms();
            let ll_checked = if super::misc::can_invoke_guest_hooks_on_this_thread() {
                if dispatch_mouse_ll_hook(wm::MOUSEMOVE, screen, 0, time) {
                    return;
                }
                true
            } else {
                false
            };
            let relative = super::cursor::relative_mode();
            if let Some(c) = super::cursor::clip_violation(screen) {
                crate::macos::cursor_warp(c);
            }
            if !relative {
                super::cursor::set_cached_pos(screen);
            }
            if nsevent_raw_active(time) {
                if let Some((x, y)) = accumulate(&RAW_ACC, dx, dy) {
                    raw_mouse(x, y, 0, 0);
                }
            }
            if relative || raw_nolegacy_active(1, 2) {
                if !ll_checked {
                    defer_mouse_ll_hook(hwnd, wm::MOUSEMOVE, screen, 0, time);
                }
                return;
            }
            let cap = CAPTURE.load(Ordering::Acquire);
            let (target, pos) = if cap != 0 {
                (cap, client_of(cap, screen))
            } else {
                (hwnd, client)
            };
            if target == 0 {
                if !ll_checked {
                    defer_mouse_ll_hook(0, wm::MOUSEMOVE, screen, 0, time);
                }
                return;
            }
            let posted = if let Some(p) = pos {
                post_input_to_checked(target, wm::MOUSEMOVE, mk_state(), make_long(p.x, p.y), Vec::new(), ll_checked)
            } else {
                false
            };
            if !posted && !ll_checked {
                defer_mouse_ll_hook(target, wm::MOUSEMOVE, screen, 0, time);
            }
        }
        NativeMouse::Button { hwnd, client, screen, button, down, clicks } => {
            let (vk, raw_down, raw_up, ll_msg, ll_xbtn) = match (button, down) {
                (MouseButton::Left, true) => (VK_LBUTTON, 0x1u16, 0x2u16, wm::LBUTTONDOWN, 0usize),
                (MouseButton::Left, false) => (VK_LBUTTON, 0x1, 0x2, wm::LBUTTONUP, 0),
                (MouseButton::Right, true) => (VK_RBUTTON, 0x4, 0x8, wm::RBUTTONDOWN, 0),
                (MouseButton::Right, false) => (VK_RBUTTON, 0x4, 0x8, wm::RBUTTONUP, 0),
                (MouseButton::Middle, true) => (VK_MBUTTON, 0x10, 0x20, wm::MBUTTONDOWN, 0),
                (MouseButton::Middle, false) => (VK_MBUTTON, 0x10, 0x20, wm::MBUTTONUP, 0),
                (MouseButton::X1, true) => (VK_XBUTTON1, 0x40, 0x80, wm::XBUTTONDOWN, 1),
                (MouseButton::X1, false) => (VK_XBUTTON1, 0x40, 0x80, wm::XBUTTONUP, 1),
                (MouseButton::X2, true) => (VK_XBUTTON2, 0x100, 0x200, wm::XBUTTONDOWN, 2),
                (MouseButton::X2, false) => (VK_XBUTTON2, 0x100, 0x200, wm::XBUTTONUP, 2),
            };
            let time = tick_ms();
            let ll_mouse_data = (ll_xbtn << 16) as u32;
            let ll_checked = if super::misc::can_invoke_guest_hooks_on_this_thread() {
                if dispatch_mouse_ll_hook(ll_msg, screen, ll_mouse_data, time) {
                    return;
                }
                true
            } else {
                false
            };
            set_async(vk, down);
            raw_mouse(0, 0, if down { raw_down } else { raw_up }, 0);
            if !super::cursor::relative_mode() {
                super::cursor::set_cached_pos(screen);
            }
            if raw_nolegacy_active(1, 2) {
                if !ll_checked {
                    defer_mouse_ll_hook(hwnd, ll_msg, screen, ll_mouse_data, time);
                }
                return;
            }
            let cap = CAPTURE.load(Ordering::Acquire);
            let (target, pos) = if cap != 0 {
                (cap, client_of(cap, screen))
            } else {
                (hwnd, client)
            };
            if target == 0 {
                if !ll_checked {
                    defer_mouse_ll_hook(0, ll_msg, screen, ll_mouse_data, time);
                }
                return;
            }
            let pos = if super::cursor::relative_mode() { client_of(target, super::cursor::cursor_pos()) } else { pos };
            let Some(p) = pos else {
                if !ll_checked {
                    defer_mouse_ll_hook(target, ll_msg, screen, ll_mouse_data, time);
                }
                return;
            };
            let dbl = down && clicks >= 2 && clicks % 2 == 0 && window::get(target).is_some_and(|w| w.class.style.load(Ordering::Relaxed) & super::class::CS_DBLCLKS != 0);
            let (msg, xbtn) = match (button, down, dbl) {
                (MouseButton::Left, true, false) => (wm::LBUTTONDOWN, 0),
                (MouseButton::Left, true, true) => (wm::LBUTTONDBLCLK, 0),
                (MouseButton::Left, false, _) => (wm::LBUTTONUP, 0),
                (MouseButton::Right, true, false) => (wm::RBUTTONDOWN, 0),
                (MouseButton::Right, true, true) => (wm::RBUTTONDBLCLK, 0),
                (MouseButton::Right, false, _) => (wm::RBUTTONUP, 0),
                (MouseButton::Middle, true, false) => (wm::MBUTTONDOWN, 0),
                (MouseButton::Middle, true, true) => (wm::MBUTTONDBLCLK, 0),
                (MouseButton::Middle, false, _) => (wm::MBUTTONUP, 0),
                (MouseButton::X1 | MouseButton::X2, d, dd) => {
                    let x = if matches!(button, MouseButton::X1) { 1usize } else { 2 };
                    (if !d { wm::XBUTTONUP } else if dd { wm::XBUTTONDBLCLK } else { wm::XBUTTONDOWN }, x)
                }
            };
            if !post_input_to_checked(target, msg, mk_state() | (xbtn << 16), make_long(p.x, p.y), Vec::new(), ll_checked) && !ll_checked {
                defer_mouse_ll_hook(target, ll_msg, screen, ll_mouse_data, time);
            }
        }
        NativeMouse::Wheel { screen, delta, horizontal } => {
            if delta == 0 {
                return;
            }
            let time = tick_ms();
            let msg = if horizontal { wm::MOUSEHWHEEL } else { wm::MOUSEWHEEL };
            let pos = if super::cursor::relative_mode() { super::cursor::cursor_pos() } else { screen };
            let mouse_data = ((delta as i16 as u16) as u32) << 16;
            let ll_checked = if super::misc::can_invoke_guest_hooks_on_this_thread() {
                if dispatch_mouse_ll_hook(msg, pos, mouse_data, time) {
                    return;
                }
                true
            } else {
                false
            };
            raw_mouse(0, 0, if horizontal { 0x800 } else { 0x400 }, delta as i16 as u16);
            let target = FOCUS.load(Ordering::Acquire);
            if raw_nolegacy_active(1, 2) {
                if !ll_checked {
                    defer_mouse_ll_hook(target, msg, pos, mouse_data, time);
                }
                return;
            }
            if target == 0 {
                if !ll_checked {
                    defer_mouse_ll_hook(0, msg, pos, mouse_data, time);
                }
                return;
            }
            if !post_input_to_checked(target, msg, mk_state() | ((delta as i16 as u16 as usize) << 16), make_long(pos.x, pos.y), Vec::new(), ll_checked) && !ll_checked {
                defer_mouse_ll_hook(target, msg, pos, mouse_data, time);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Raw input
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RAWINPUTDEVICE {
    pub usage_page: u16,
    pub usage: u16,
    pub flags: u32,
    pub target: usize,
}
const _: () = assert!(std::mem::size_of::<RAWINPUTDEVICE>() == 16);

const RIDEV_REMOVE: u32 = 0x0000_0001;
const RIDEV_NOLEGACY: u32 = 0x0000_0030;
const RIDEV_INPUTSINK: u32 = 0x0000_0100;
const RIDEV_NOHOTKEYS: u32 = 0x0000_0200;
const RIM_TYPEMOUSE: u32 = 0;
const RIM_TYPEKEYBOARD: u32 = 1;
const RID_INPUT: u32 = 0x1000_0003;
const RID_HEADER: u32 = 0x1000_0005;
const HEADER_SIZE: usize = 24;
const FAKE_MOUSE_DEVICE: usize = 0x0001_0001;
const FAKE_KEYBOARD_DEVICE: usize = 0x0001_0002;

#[derive(Clone, Copy, Debug)]
struct RawReg {
    page: u16,
    usage: u16,
    flags: u32,
    target: usize,
    #[allow(dead_code)] // registering thread (Windows scopes registrations per thread)
    tid: u32,
}

static RAW_REGS: Mutex<Vec<RawReg>> = Mutex::new(Vec::new());

const RING: usize = 4096;
struct RawRing {
    seq: u32,
    slots: Vec<(usize, Vec<u8>)>,
}
static RAW_RING: Mutex<RawRing> = Mutex::new(RawRing { seq: 0, slots: Vec::new() });

fn raw_store(data: Vec<u8>) -> usize {
    let mut r = RAW_RING.lock().unwrap();
    if r.slots.is_empty() {
        r.slots = vec![(0, Vec::new()); RING];
    }
    r.seq = r.seq.wrapping_add(1).max(1);
    let h = 0x2000_0000usize | (r.seq as usize & 0x0FFF_FFFF);
    let i = r.seq as usize % RING;
    r.slots[i] = (h, data);
    h
}

fn raw_fetch(h: usize) -> Option<Vec<u8>> {
    let r = RAW_RING.lock().unwrap();
    if r.slots.is_empty() {
        return None;
    }
    let i = (h & 0x0FFF_FFFF) % RING;
    let (sh, d) = &r.slots[i];
    (*sh == h).then(|| d.clone())
}

pub fn raw_mouse_registered() -> bool {
    RAW_REGS.lock().unwrap().iter().any(|r| r.page == 1 && r.usage == 2)
}

/// True when the given legacy HID device (`(1, 2)` mouse or `(1, 6)` keyboard)
/// is registered with `RIDEV_NOLEGACY`, suppressing legacy `WM_MOUSE*` / `WM_KEY*`
/// messages while still delivering `WM_INPUT`.
fn raw_nolegacy_active(page: u16, usage: u16) -> bool {
    RAW_REGS
        .lock()
        .unwrap()
        .iter()
        .any(|r| r.page == page && r.usage == usage && (r.flags & RIDEV_NOLEGACY) == RIDEV_NOLEGACY)
}

/// True when the foreground application has an active raw keyboard (`(1, 6)`)
/// registration with `RIDEV_NOHOTKEYS`, disabling application-defined `RegisterHotKey`
/// interception so keydowns flow through to raw input and normal dispatch.
fn raw_nohotkeys_active() -> bool {
    let reg = RAW_REGS
        .lock()
        .unwrap()
        .iter()
        .find(|r| r.page == 1 && r.usage == 6 && (r.flags & RIDEV_NOHOTKEYS) != 0)
        .copied();
    let Some(reg) = reg else {
        return false;
    };
    if !APP_ACTIVE.load(Ordering::Acquire) {
        return false;
    }
    if reg.target != 0 {
        let act = ACTIVE.load(Ordering::Acquire);
        return act == reg.target || window::is_child_of(reg.target, act);
    }
    FOCUS.load(Ordering::Acquire) != 0 || ACTIVE.load(Ordering::Acquire) != 0
}

/// Deliver a raw input record to the registered target (if any).
fn raw_deliver(page: u16, usage: u16, build: impl FnOnce(usize) -> Vec<u8>) {
    let reg = RAW_REGS.lock().unwrap().iter().find(|r| r.page == page && r.usage == usage).copied();
    let Some(reg) = reg else { return };
    let fg = APP_ACTIVE.load(Ordering::Acquire);
    if !fg && reg.flags & RIDEV_INPUTSINK == 0 {
        return;
    }
    let target = if reg.target != 0 { reg.target } else { FOCUS.load(Ordering::Acquire) };
    if target == 0 {
        return;
    }
    if reg.target != 0 && reg.flags & RIDEV_INPUTSINK == 0 {
        // Without INPUTSINK, only while the target is foreground.
        let act = ACTIVE.load(Ordering::Acquire);
        if !(act == reg.target || window::is_child_of(reg.target, act)) {
            return;
        }
    }
    let device = if page == 1 && usage == 2 { FAKE_MOUSE_DEVICE } else { FAKE_KEYBOARD_DEVICE };
    let h = raw_store(build(device));
    let wparam = if fg { 0 } else { 1 }; // RIM_INPUT / RIM_INPUTSINK
    post_input_to(target, wm::INPUT, wparam, h as isize, Vec::new());
}

fn header(ty: u32, size: u32, device: usize, wparam: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);
    v.extend_from_slice(&ty.to_le_bytes());
    v.extend_from_slice(&size.to_le_bytes());
    v.extend_from_slice(&(device as u64).to_le_bytes());
    v.extend_from_slice(&(wparam as u64).to_le_bytes());
    v
}

fn raw_mouse(dx: i32, dy: i32, button_flags: u16, button_data: u16) {
    raw_deliver(1, 2, |dev| {
        let wp = if APP_ACTIVE.load(Ordering::Acquire) { 0 } else { 1 };
        let mut v = header(RIM_TYPEMOUSE, 48, dev, wp);
        v.extend_from_slice(&0u16.to_le_bytes()); // usFlags = MOUSE_MOVE_RELATIVE
        v.extend_from_slice(&0u16.to_le_bytes()); // padding
        v.extend_from_slice(&button_flags.to_le_bytes());
        v.extend_from_slice(&button_data.to_le_bytes());
        v.extend_from_slice(&0u32.to_le_bytes()); // ulRawButtons
        v.extend_from_slice(&dx.to_le_bytes());
        v.extend_from_slice(&dy.to_le_bytes());
        v.extend_from_slice(&0u32.to_le_bytes()); // ulExtraInformation
        v
    });
}

fn raw_keyboard(k: &keymap::Key, down: bool, msg: u32) {
    raw_deliver(1, 6, |dev| {
        let wp = if APP_ACTIVE.load(Ordering::Acquire) { 0 } else { 1 };
        let mut v = header(RIM_TYPEKEYBOARD, 40, dev, wp);
        let flags: u16 = (!down) as u16 | if k.ext { 2 } else { 0 };
        v.extend_from_slice(&(k.scan as u16).to_le_bytes());
        v.extend_from_slice(&flags.to_le_bytes());
        v.extend_from_slice(&0u16.to_le_bytes());
        v.extend_from_slice(&(k.vk as u16).to_le_bytes());
        v.extend_from_slice(&msg.to_le_bytes());
        v.extend_from_slice(&0u32.to_le_bytes());
        v
    });
}

pub unsafe extern "win64" fn RegisterRawInputDevices(devs: *const RAWINPUTDEVICE, n: u32, size: u32) -> BOOL {
    if devs.is_null() || n == 0 || size as usize != std::mem::size_of::<RAWINPUTDEVICE>() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let tid = crate::teb::thread_id();
    let devs = std::slice::from_raw_parts(devs, n as usize);
    let mut regs = RAW_REGS.lock().unwrap();
    for d in devs {
        if d.flags & RIDEV_REMOVE != 0 {
            if d.target != 0 {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            regs.retain(|r| !(r.page == d.usage_page && r.usage == d.usage));
            log::info!("RegisterRawInputDevices: removed ({},{})", d.usage_page, d.usage);
            continue;
        }
        if d.target != 0 && !window::exists(d.target) {
            set_error(e::INVALID_WINDOW_HANDLE);
            return FALSE;
        }
        // Per Microsoft Learn, RIDEV_NOLEGACY (0x30 = RIDEV_EXCLUDE | RIDEV_PAGEONLY) is only
        // valid for legacy keyboard (1, 6) and mouse (1, 2) top-level collections. Test the full
        // composite mask so standalone RIDEV_PAGEONLY (0x20) / RIDEV_EXCLUDE (0x10) are not rejected.
        if (d.flags & RIDEV_NOLEGACY) == RIDEV_NOLEGACY
            && !(d.usage_page == 1 && matches!(d.usage, 2 | 6))
        {
            set_error(e::INVALID_PARAMETER);
            return FALSE;
        }
        regs.retain(|r| !(r.page == d.usage_page && r.usage == d.usage));
        regs.push(RawReg { page: d.usage_page, usage: d.usage, flags: d.flags, target: d.target, tid });
        log::info!("RegisterRawInputDevices: ({},{}) flags={:#x} target={:#x}", d.usage_page, d.usage, d.flags, d.target);
    }
    drop(regs);
    super::cursor::update_state();
    TRUE
}

pub unsafe extern "win64" fn GetRegisteredRawInputDevices(out: *mut RAWINPUTDEVICE, n: *mut u32, size: u32) -> u32 {
    if n.is_null() || size as usize != std::mem::size_of::<RAWINPUTDEVICE>() {
        set_error(e::INVALID_PARAMETER);
        return u32::MAX;
    }
    let regs = RAW_REGS.lock().unwrap();
    if out.is_null() || (*n as usize) < regs.len() {
        *n = regs.len() as u32;
        if !out.is_null() {
            set_error(e::INSUFFICIENT_BUFFER);
            return u32::MAX;
        }
        return 0;
    }
    for (i, r) in regs.iter().enumerate() {
        *out.add(i) = RAWINPUTDEVICE { usage_page: r.page, usage: r.usage, flags: r.flags, target: r.target };
    }
    regs.len() as u32
}

pub unsafe extern "win64" fn GetRawInputData(h: usize, cmd: u32, data: *mut u8, size: *mut u32, header_size: u32) -> u32 {
    if header_size as usize != HEADER_SIZE || size.is_null() || (cmd != RID_INPUT && cmd != RID_HEADER) {
        set_error(e::INVALID_PARAMETER);
        return u32::MAX;
    }
    let Some(rec) = raw_fetch(h) else {
        set_error(e::INVALID_PARAMETER);
        return u32::MAX;
    };
    let need = if cmd == RID_HEADER { HEADER_SIZE } else { rec.len() };
    if data.is_null() {
        *size = need as u32;
        return 0;
    }
    if (*size as usize) < need {
        set_error(e::INSUFFICIENT_BUFFER);
        return u32::MAX;
    }
    std::ptr::copy_nonoverlapping(rec.as_ptr(), data, need);
    need as u32
}

#[repr(C)]
pub struct RAWINPUTDEVICELIST {
    pub h_device: usize,
    pub dw_type: u32,
}

pub unsafe extern "win64" fn GetRawInputDeviceList(
    list: *mut RAWINPUTDEVICELIST,
    num_devices: *mut u32,
    cb_size: u32,
) -> u32 {
    if num_devices.is_null() || cb_size as usize != std::mem::size_of::<RAWINPUTDEVICELIST>() {
        set_error(e::INVALID_PARAMETER);
        return u32::MAX;
    }
    let devs = [
        RAWINPUTDEVICELIST { h_device: FAKE_MOUSE_DEVICE, dw_type: RIM_TYPEMOUSE },
        RAWINPUTDEVICELIST { h_device: FAKE_KEYBOARD_DEVICE, dw_type: RIM_TYPEKEYBOARD },
    ];
    let cap = std::ptr::read_unaligned(num_devices) as usize;
    if list.is_null() {
        std::ptr::write_unaligned(num_devices, devs.len() as u32);
        return 0;
    }
    if cap < devs.len() {
        std::ptr::write_unaligned(num_devices, devs.len() as u32);
        set_error(e::INSUFFICIENT_BUFFER);
        return u32::MAX;
    }
    for (i, d) in devs.iter().enumerate() {
        std::ptr::write_unaligned(list.add(i), RAWINPUTDEVICELIST { h_device: d.h_device, dw_type: d.dw_type });
    }
    devs.len() as u32
}

unsafe fn raw_device_info_impl(h_device: usize, cmd: u32, data: *mut u8, pcb_size: *mut u32, wide: bool) -> u32 {
    const RIDI_PREPARSEDDATA: u32 = 0x2000_0005;
    const RIDI_DEVICENAME: u32 = 0x2000_0007;
    const RIDI_DEVICEINFO: u32 = 0x2000_000B;
    if pcb_size.is_null() {
        set_error(e::INVALID_PARAMETER);
        return u32::MAX;
    }
    let is_mouse = h_device == FAKE_MOUSE_DEVICE;
    let is_kbd = h_device == FAKE_KEYBOARD_DEVICE;
    if !is_mouse && !is_kbd {
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        return u32::MAX;
    }
    match cmd {
        RIDI_DEVICENAME => {
            let name = if is_mouse {
                "\\\\?\\HID#VID_05AC&PID_0001#Mouse#{378de44c-56ef-11d1-bc8c-00a0c91405dd}"
            } else {
                "\\\\?\\HID#VID_05AC&PID_0002#Keyboard#{884b96c3-56ef-11d1-bc8c-00a0c91405dd}"
            };
            let need = if wide { name.encode_utf16().count() + 1 } else { name.len() + 1 };
            let cap = std::ptr::read_unaligned(pcb_size) as usize;
            if data.is_null() {
                std::ptr::write_unaligned(pcb_size, need as u32);
                return 0;
            }
            if cap < need {
                std::ptr::write_unaligned(pcb_size, need as u32);
                set_error(e::INSUFFICIENT_BUFFER);
                return u32::MAX;
            }
            if wide {
                crate::strings::write_wide_buf(name, data as *mut u16, cap);
            } else {
                crate::strings::write_ansi_buf(name, data, cap);
            }
            need as u32
        }
        RIDI_DEVICEINFO => {
            // sizeof(RID_DEVICE_INFO) == 32
            const SIZE: usize = 32;
            let cap = std::ptr::read_unaligned(pcb_size) as usize;
            if data.is_null() {
                std::ptr::write_unaligned(pcb_size, SIZE as u32);
                return 0;
            }
            if cap < SIZE {
                std::ptr::write_unaligned(pcb_size, SIZE as u32);
                set_error(e::INSUFFICIENT_BUFFER);
                return u32::MAX;
            }
            std::ptr::write_bytes(data, 0, SIZE);
            std::ptr::write_unaligned(data as *mut u32, SIZE as u32);
            if is_mouse {
                std::ptr::write_unaligned(data.add(4) as *mut u32, RIM_TYPEMOUSE);
                std::ptr::write_unaligned(data.add(8) as *mut u32, 1); // dwId
                std::ptr::write_unaligned(data.add(12) as *mut u32, 5); // dwNumberOfButtons
                std::ptr::write_unaligned(data.add(16) as *mut u32, 1000); // dwSampleRate
                std::ptr::write_unaligned(data.add(20) as *mut u32, 1); // fHasHorizontalWheel
            } else {
                std::ptr::write_unaligned(data.add(4) as *mut u32, RIM_TYPEKEYBOARD);
                std::ptr::write_unaligned(data.add(8) as *mut u32, 4); // dwType (enhanced)
                std::ptr::write_unaligned(data.add(16) as *mut u32, 1); // dwKeyboardMode
                std::ptr::write_unaligned(data.add(20) as *mut u32, 12); // dwNumberOfFunctionKeys
                std::ptr::write_unaligned(data.add(24) as *mut u32, 3); // dwNumberOfIndicators
                std::ptr::write_unaligned(data.add(28) as *mut u32, 104); // dwNumberOfKeysTotal
            }
            SIZE as u32
        }
        RIDI_PREPARSEDDATA => {
            std::ptr::write_unaligned(pcb_size, 0);
            0
        }
        _ => {
            set_error(e::INVALID_PARAMETER);
            u32::MAX
        }
    }
}

pub unsafe extern "win64" fn GetRawInputDeviceInfoA(
    h_device: usize,
    cmd: u32,
    data: *mut u8,
    pcb_size: *mut u32,
) -> u32 {
    raw_device_info_impl(h_device, cmd, data, pcb_size, false)
}

pub unsafe extern "win64" fn GetRawInputDeviceInfoW(
    h_device: usize,
    cmd: u32,
    data: *mut u8,
    pcb_size: *mut u32,
) -> u32 {
    raw_device_info_impl(h_device, cmd, data, pcb_size, true)
}

pub unsafe extern "win64" fn GetRawInputBuffer(
    _data: *mut u8,
    pcb_size: *mut u32,
    _header_size: u32,
) -> u32 {
    if !pcb_size.is_null() {
        std::ptr::write_unaligned(pcb_size, 0);
    }
    0
}

pub unsafe extern "win64" fn DefRawInputProc(_raw: *mut *mut u8, _n: i32, _header_size: u32) -> LRESULT {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_input_ring_and_contract() {
        let h = raw_store(header(RIM_TYPEMOUSE, 48, FAKE_MOUSE_DEVICE, 0).into_iter().chain([0u8; 24]).collect());
        unsafe {
            let mut sz = 0u32;
            assert_eq!(GetRawInputData(h, RID_INPUT, std::ptr::null_mut(), &mut sz, 24), 0);
            assert_eq!(sz, 48);
            let mut buf = [0u8; 64];
            let mut cb = 64u32;
            assert_eq!(GetRawInputData(h, RID_INPUT, buf.as_mut_ptr(), &mut cb, 24), 48);
            assert_eq!(u32::from_le_bytes(buf[4..8].try_into().unwrap()), 48);
            let mut small = 16u32;
            assert_eq!(GetRawInputData(h, RID_INPUT, buf.as_mut_ptr(), &mut small, 24), u32::MAX);
            assert_eq!(GetRawInputData(h, RID_INPUT, buf.as_mut_ptr(), &mut cb, 16), u32::MAX);
            assert_eq!(GetRawInputData(h ^ 0x0100_0000, RID_INPUT, buf.as_mut_ptr(), &mut cb, 24), u32::MAX);
        }
    }

    #[test]
    fn key_lparam_and_state() {
        // Synthesize the sync state update path.
        let m = MSG { message: wm::KEYDOWN, wparam: b'A' as usize, lparam: 1 | (0x1E << 16), ..Default::default() };
        on_input_retrieved(&m);
        unsafe { assert!(GetKeyState(b'A' as i32) < 0) };
        let m = MSG { message: wm::KEYUP, wparam: b'A' as usize, lparam: 1 | (0x1E << 16) | (3 << 30), ..Default::default() };
        on_input_retrieved(&m);
        unsafe { assert!(GetKeyState(b'A' as i32) >= 0) };
    }

    #[test]
    fn to_unicode_non_us_layout_and_fallback() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        *TO_UNICODE_DEAD_KEY.lock().unwrap() = 0;

        // 1. Non-US synthetic layout (AZERTY + German umlaut + AltGr Euro).
        let azerty = crate::macos::keyboard::build_test_uchr(
            &[
                (0x0C, b'a' as u16), // physical Q -> 'a'
                (0x00, b'q' as u16), // physical A -> 'q'
                (0x12, b'&' as u16), // physical 1 -> '&'
                (0x21, 0x00FC),      // physical [ -> 'ü'
            ],
            &[
                (0x0C, b'A' as u16),
                (0x00, b'Q' as u16),
                (0x12, b'1' as u16),
                (0x21, 0x00DC),
            ],
            &[(0x0E, 0x20AC)], // Option/AltGr + E -> '€'
        );
        crate::macos::keyboard::with_test_layout(Some(azerty), || unsafe {
            let mut ks = [0u8; 256];
            let mut buf = [0u16; 8];

            // Unshifted physical Q (VK 'Q') -> 'a'.
            assert_eq!(ToUnicode(b'Q' as u32, 0x10, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], b'a' as u16);

            // Shifted physical Q -> 'A'.
            ks[VK_SHIFT as usize] = 0x80;
            assert_eq!(ToUnicodeEx(b'Q' as u32, 0x10, ks.as_ptr(), buf.as_mut_ptr(), 8, 0, 0), 1);
            assert_eq!(buf[0], b'A' as u16);

            // Number row: Shift+1 -> '1', unshifted 1 -> '&'.
            assert_eq!(ToUnicode(b'1' as u32, 0x02, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], b'1' as u16);
            ks[VK_SHIFT as usize] = 0;
            assert_eq!(ToUnicode(b'1' as u32, 0x02, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], b'&' as u16);

            // Non-ASCII character 'ü' (U+00FC) and CapsLock -> 'Ü' (U+00DC).
            assert_eq!(ToUnicode(0xDB, 0x1A, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], 0x00FC);
            ks[VK_CAPITAL as usize] = 0x01;
            assert_eq!(ToUnicode(0xDB, 0x1A, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], 0x00DC);
            ks[VK_CAPITAL as usize] = 0;

            // AltGr (Ctrl+Alt) + E -> '€' (U+20AC).
            ks[VK_CONTROL as usize] = 0x80;
            ks[VK_MENU as usize] = 0x80;
            assert_eq!(ToUnicodeEx(b'E' as u32, 0x12, ks.as_ptr(), buf.as_mut_ptr(), 8, 0, 0), 1);
            assert_eq!(buf[0], 0x20AC);
            ks[VK_CONTROL as usize] = 0;
            ks[VK_MENU as usize] = 0;

            // Non-character keys (F1, Left arrow, key-up high bit) return 0.
            assert_eq!(ToUnicode(0x70, 0x3B, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 0);
            assert_eq!(ToUnicode(0x25, 0x4B, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 0);
            assert_eq!(ToUnicode(b'Q' as u32, 0x8010, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 0);
        });

        // 2. Fallback to US layout table when macOS layout is unavailable.
        crate::macos::keyboard::with_test_layout(None, || unsafe {
            let mut ks = [0u8; 256];
            let mut buf = [0u16; 8];
            assert_eq!(ToUnicode(b'A' as u32, 0x1E, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], b'a' as u16);
            ks[VK_SHIFT as usize] = 0x80;
            assert_eq!(ToUnicode(b'1' as u32, 0x02, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], b'!' as u16);
            ks[VK_SHIFT as usize] = 0;
            ks[VK_CONTROL as usize] = 0x80;
            assert_eq!(ToUnicode(b'C' as u32, 0x2E, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 1);
            assert_eq!(buf[0], 0x03);
        });
    }

    #[test]
    fn to_unicode_dead_key_state_hygiene() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        *TO_UNICODE_DEAD_KEY.lock().unwrap() = 0;
        unsafe {
            let mut ks = [0u8; 256];
            let mut buf = [0u16; 8];

            // Probe with wFlags = 0x4 (do not change keyboard state): if the host
            // layout has a dead key on Option+E (standard US/ABC/etc.), verify
            // that wFlags=0x4 leaves TO_UNICODE_DEAD_KEY == 0 while wFlags=0
            // sets it and composes cleanly.
            ks[VK_MENU as usize] = 0x80;
            let peek = ToUnicode(b'E' as u32, 0x12, ks.as_ptr(), buf.as_mut_ptr(), 8, 0x0004);
            assert_eq!(*TO_UNICODE_DEAD_KEY.lock().unwrap(), 0, "wFlags=0x4 must not mutate dead-key state");

            if peek == -1 {
                // Real dead key (e.g., acute accent on US/ABC layout).
                assert_eq!(ToUnicode(b'E' as u32, 0x12, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), -1);
                assert_ne!(*TO_UNICODE_DEAD_KEY.lock().unwrap(), 0);
                let saved = *TO_UNICODE_DEAD_KEY.lock().unwrap();

                // Pressing Shift alone must not clobber dead-key state.
                ks[VK_MENU as usize] = 0;
                ks[VK_SHIFT as usize] = 0x80;
                assert_eq!(ToUnicode(VK_SHIFT as u32, 0x2A, ks.as_ptr(), buf.as_mut_ptr(), 8, 0), 0);
                assert_eq!(*TO_UNICODE_DEAD_KEY.lock().unwrap(), saved);

                // Peeking 'E' with wFlags=0x4 must not consume dead-key state.
                let n_peek = ToUnicode(b'E' as u32, 0x12, ks.as_ptr(), buf.as_mut_ptr(), 8, 0x0004);
                assert_eq!(n_peek, 1);
                assert_eq!(*TO_UNICODE_DEAD_KEY.lock().unwrap(), saved);

                // Actually translating Shift+'E' consumes dead-key state and composes ('É').
                let n = ToUnicode(b'E' as u32, 0x12, ks.as_ptr(), buf.as_mut_ptr(), 8, 0);
                assert_eq!(n, 1);
                assert_eq!(buf[0], 'É' as u16);
                assert_eq!(*TO_UNICODE_DEAD_KEY.lock().unwrap(), 0);
            }
            *TO_UNICODE_DEAD_KEY.lock().unwrap() = 0;
        }
    }

    #[test]
    fn low_level_keyboard_and_mouse_hooks_suppress_and_deliver() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        static SEEN_KBD: Mutex<Vec<(WPARAM, u32, u32, u32)>> = Mutex::new(Vec::new());
        static SEEN_MOUSE: Mutex<Vec<(WPARAM, POINT, u32)>> = Mutex::new(Vec::new());

        unsafe extern "win64" fn kbd_ll(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            let ks = (lparam as *const super::misc::KBDLLHOOKSTRUCT).read_unaligned();
            SEEN_KBD.lock().unwrap().push((wparam, ks.vk_code, ks.scan_code, ks.flags));
            if code >= 0 && ks.vk_code == b'X' as u32 {
                return 1; // suppress 'X'
            }
            super::misc::CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn mouse_ll(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            let ms = (lparam as *const super::misc::MSLLHOOKSTRUCT).read_unaligned();
            SEEN_MOUSE.lock().unwrap().push((wparam, ms.pt, ms.mouse_data));
            if code >= 0 && wparam as u32 == wm::RBUTTONDOWN {
                return 1; // suppress right button down
            }
            super::misc::CallNextHookEx(0, code, wparam, lparam)
        }

        std::thread::spawn(|| unsafe {
            SEEN_KBD.lock().unwrap().clear();
            SEEN_MOUSE.lock().unwrap().clear();

            let cls = crate::strings::to_wide_z("LLHookInputTestCls");
            let wc = super::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<super::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::WNDPROC_HANDLE_DEF_WINDOW_W,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
                icon_sm: 0,
            };
            let _ = super::class::RegisterClassExW(&wc);
            let hwnd = window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                0,
                0,
                0,
                200,
                200,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            set_focus(hwnd as usize);

            let h_k = super::misc::SetWindowsHookExW(
                super::misc::WH_KEYBOARD_LL,
                kbd_ll as *const () as usize,
                std::ptr::null_mut(),
                0,
            );
            let h_m = super::misc::SetWindowsHookExW(
                super::misc::WH_MOUSE_LL,
                mouse_ll as *const () as usize,
                std::ptr::null_mut(),
                0,
            );
            assert_ne!(h_k, 0);
            assert_ne!(h_m, 0);

            // 'X' (mac 0x07) is suppressed; 'A' (mac 0x00) is delivered.
            native_key(NativeKey { mac: 0x07, down: true, repeat: false, chars: vec![b'x' as u16] });
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            let mut m = MSG::default();
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::KEYFIRST, wm::KEYLAST, message::PM_REMOVE), TRUE);
            assert_eq!(m.message, wm::KEYDOWN);
            assert_eq!(m.wparam, b'A' as usize);
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::KEYFIRST, wm::KEYLAST, message::PM_REMOVE), FALSE);
            {
                let ks = SEEN_KBD.lock().unwrap().clone();
                assert_eq!(ks.len(), 2);
                assert_eq!(ks[0], (wm::KEYDOWN as WPARAM, b'X' as u32, 0x2D, 0));
                assert_eq!(ks[1], (wm::KEYDOWN as WPARAM, b'A' as u32, 0x1E, 0));
            }

            // Right button down is suppressed; left button down is delivered.
            native_mouse(NativeMouse::Button {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 10, y: 20 }),
                screen: POINT { x: 110, y: 120 },
                button: MouseButton::Right,
                down: true,
                clicks: 1,
            });
            native_mouse(NativeMouse::Button {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 15, y: 25 }),
                screen: POINT { x: 115, y: 125 },
                button: MouseButton::Left,
                down: true,
                clicks: 1,
            });
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::MOUSEFIRST, wm::MOUSELAST, message::PM_REMOVE), TRUE);
            assert_eq!(m.message, wm::LBUTTONDOWN);
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::MOUSEFIRST, wm::MOUSELAST, message::PM_REMOVE), FALSE);
            {
                let ms = SEEN_MOUSE.lock().unwrap().clone();
                assert_eq!(ms.len(), 2);
                assert_eq!(ms[0], (wm::RBUTTONDOWN as WPARAM, POINT { x: 110, y: 120 }, 0));
                assert_eq!(ms[1], (wm::LBUTTONDOWN as WPARAM, POINT { x: 115, y: 125 }, 0));
            }

            assert_eq!(super::misc::UnhookWindowsHookEx(h_k), TRUE);
            assert_eq!(super::misc::UnhookWindowsHookEx(h_m), TRUE);
            release_all_keys();
            let _ = window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn raw_input_nolegacy_suppresses_legacy_messages_and_keeps_wm_input() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let cls = crate::strings::to_wide_z("RawNoLegacyTestWnd");
            let wc = super::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<super::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::WNDPROC_HANDLE_DEF_WINDOW_W,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
                icon_sm: 0,
            };
            let _ = super::class::RegisterClassExW(&wc);
            let hwnd = window::CreateWindowExW(
                0, cls.as_ptr(), std::ptr::null(), ws::POPUP | ws::VISIBLE,
                0, 0, 200, 200, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            APP_ACTIVE.store(true, Ordering::Release);
            SetFocus(hwnd);
            ACTIVE.store(hwnd as usize, Ordering::Release);

            // RIDEV_NOLEGACY (0x30) on a non-legacy usage (e.g. gamepad 1,5) must fail with INVALID_PARAMETER.
            let bad = RAWINPUTDEVICE { usage_page: 1, usage: 5, flags: RIDEV_NOLEGACY, target: hwnd as usize };
            assert_eq!(RegisterRawInputDevices(&bad, 1, 16), FALSE);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);

            // Standalone RIDEV_PAGEONLY (0x20) and RIDEV_EXCLUDE (0x10) overlap RIDEV_NOLEGACY's
            // bits and must NOT be rejected as RIDEV_NOLEGACY or suppress legacy messages.
            const RIDEV_EXCLUDE: u32 = 0x0000_0010;
            const RIDEV_PAGEONLY: u32 = 0x0000_0020;
            let page_only_batch = [
                RAWINPUTDEVICE { usage_page: 1, usage: 0, flags: RIDEV_PAGEONLY, target: hwnd as usize },
                RAWINPUTDEVICE { usage_page: 1, usage: 5, flags: RIDEV_EXCLUDE, target: 0 },
            ];
            assert_eq!(RegisterRawInputDevices(page_only_batch.as_ptr(), 2, 16), TRUE);
            assert!(!raw_nolegacy_active(1, 2));
            assert!(!raw_nolegacy_active(1, 6));
            let rem_page = [
                RAWINPUTDEVICE { usage_page: 1, usage: 0, flags: RIDEV_REMOVE, target: 0 },
                RAWINPUTDEVICE { usage_page: 1, usage: 5, flags: RIDEV_REMOVE, target: 0 },
            ];
            assert_eq!(RegisterRawInputDevices(rem_page.as_ptr(), 2, 16), TRUE);

            // Register mouse (1,2) and keyboard (1,6) with RIDEV_NOLEGACY.
            let devs = [
                RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: RIDEV_NOLEGACY, target: hwnd as usize },
                RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: RIDEV_NOLEGACY, target: hwnd as usize },
            ];
            assert_eq!(RegisterRawInputDevices(devs.as_ptr(), 2, 16), TRUE);

            // Key press: WM_INPUT delivered, WM_KEYDOWN suppressed, async state updated.
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            let mut m = MSG::default();
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::INPUT, wm::INPUT, message::PM_REMOVE),
                TRUE,
                "WM_INPUT must still arrive under RIDEV_NOLEGACY"
            );
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::KEYFIRST, wm::KEYLAST, message::PM_REMOVE),
                FALSE,
                "WM_KEYDOWN must be suppressed under RIDEV_NOLEGACY"
            );
            assert!(GetAsyncKeyState(b'A' as i32) < 0);

            // Mouse button: WM_INPUT delivered, WM_LBUTTONDOWN suppressed.
            native_mouse(NativeMouse::Button {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 10, y: 10 }),
                screen: POINT { x: 10, y: 10 },
                button: MouseButton::Left,
                down: true,
                clicks: 1,
            });
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::INPUT, wm::INPUT, message::PM_REMOVE),
                TRUE,
                "mouse WM_INPUT must still arrive under RIDEV_NOLEGACY"
            );
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::MOUSEFIRST, wm::MOUSELAST, message::PM_REMOVE),
                FALSE,
                "WM_LBUTTONDOWN must be suppressed under RIDEV_NOLEGACY"
            );

            // Remove registrations -> legacy messages flow again.
            let rem = [
                RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: RIDEV_REMOVE, target: 0 },
                RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: RIDEV_REMOVE, target: 0 },
            ];
            assert_eq!(RegisterRawInputDevices(rem.as_ptr(), 2, 16), TRUE);
            release_all_keys();
            let _ = window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn deferred_low_level_hooks_fire_under_ridev_nolegacy_and_once_without_it() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            static KB_LL_EVENTS: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
            static MS_LL_EVENTS: Mutex<Vec<(u32, i32, i32, u32)>> = Mutex::new(Vec::new());

            unsafe extern "win64" fn kb_ll_proc(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
                if code >= 0 && lp != 0 {
                    let ks = (lp as *const super::super::misc::KBDLLHOOKSTRUCT).read_unaligned();
                    KB_LL_EVENTS.lock().unwrap().push((wp as u32, ks.vk_code));
                }
                super::super::misc::CallNextHookEx(0, code, wp, lp)
            }
            unsafe extern "win64" fn ms_ll_proc(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
                if code >= 0 && lp != 0 {
                    let ms = (lp as *const super::super::misc::MSLLHOOKSTRUCT).read_unaligned();
                    MS_LL_EVENTS
                        .lock()
                        .unwrap()
                        .push((wp as u32, ms.pt.x, ms.pt.y, ms.mouse_data));
                }
                super::super::misc::CallNextHookEx(0, code, wp, lp)
            }

            KB_LL_EVENTS.lock().unwrap().clear();
            MS_LL_EVENTS.lock().unwrap().clear();

            let cls = crate::strings::to_wide_z("DeferredLlNoLegacyWnd");
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: register_typed_host_wndproc(defproc::DefWindowProcW),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = class::RegisterClassW(&wc);
            let hwnd = window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                ws::POPUP,
                0,
                0,
                200,
                200,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            set_active_window(hwnd as usize, false);
            set_focus(hwnd as usize);

            let h_kb = super::super::misc::SetWindowsHookExW(
                super::super::misc::WH_KEYBOARD_LL,
                kb_ll_proc as *const () as usize,
                std::ptr::null_mut(),
                0,
            );
            let h_ms = super::super::misc::SetWindowsHookExW(
                super::super::misc::WH_MOUSE_LL,
                ms_ll_proc as *const () as usize,
                std::ptr::null_mut(),
                0,
            );
            assert_ne!(h_kb, 0);
            assert_ne!(h_ms, 0);

            // Register both mouse and keyboard with RIDEV_NOLEGACY.
            let devs = [
                RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: RIDEV_NOLEGACY, target: hwnd as usize },
                RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: RIDEV_NOLEGACY, target: hwnd as usize },
            ];
            assert_eq!(RegisterRawInputDevices(devs.as_ptr(), 2, 16), TRUE);

            // Simulate AppKit main thread where guest hooks cannot execute inline.
            super::super::misc::set_force_defer_guest_hooks(true);
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            native_key(NativeKey { mac: 0x00, down: false, repeat: false, chars: Vec::new() });
            native_mouse(NativeMouse::Move {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 12, y: 34 }),
                screen: POINT { x: 12, y: 34 },
                dx: 5.0,
                dy: -3.0,
            });
            native_mouse(NativeMouse::Button {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 12, y: 34 }),
                screen: POINT { x: 12, y: 34 },
                button: MouseButton::Left,
                down: true,
                clicks: 1,
            });
            native_mouse(NativeMouse::Wheel {
                screen: POINT { x: 12, y: 34 },
                delta: 120,
                horizontal: false,
            });
            // Hooks must NOT have fired synchronously on the simulated AppKit thread.
            assert!(KB_LL_EVENTS.lock().unwrap().is_empty());
            assert!(MS_LL_EVENTS.lock().unwrap().is_empty());
            super::super::misc::set_force_defer_guest_hooks(false);

            // Pump on the Win32 thread: WM_INPUT arrives, legacy messages stay suppressed,
            // and all deferred WH_KEYBOARD_LL and WH_MOUSE_LL callbacks fire.
            let mut m = MSG::default();
            let mut raw_count = 0usize;
            while message::PeekMessageW(&mut m, hwnd, wm::INPUT, wm::INPUT, message::PM_REMOVE) != 0 {
                raw_count += 1;
            }
            assert!(raw_count >= 4, "expected WM_INPUT messages under RIDEV_NOLEGACY, got {raw_count}");
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::KEYFIRST, wm::KEYLAST, message::PM_REMOVE), FALSE);
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::MOUSEFIRST, wm::MOUSELAST, message::PM_REMOVE), FALSE);

            assert_eq!(
                KB_LL_EVENTS.lock().unwrap().clone(),
                vec![(wm::KEYDOWN, b'A' as u32), (wm::KEYUP, b'A' as u32)],
                "deferred WH_KEYBOARD_LL must fire under RIDEV_NOLEGACY"
            );
            assert_eq!(
                MS_LL_EVENTS.lock().unwrap().clone(),
                vec![
                    (wm::MOUSEMOVE, 12, 34, 0),
                    (wm::LBUTTONDOWN, 12, 34, 0),
                    (wm::MOUSEWHEEL, 12, 34, (120u16 as u32) << 16),
                ],
                "deferred WH_MOUSE_LL must fire under RIDEV_NOLEGACY"
            );

            // Without RIDEV_NOLEGACY, deferred hooks on the legacy path must fire exactly once (no duplicates).
            let rem = [
                RAWINPUTDEVICE { usage_page: 1, usage: 2, flags: RIDEV_REMOVE, target: 0 },
                RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: RIDEV_REMOVE, target: 0 },
            ];
            assert_eq!(RegisterRawInputDevices(rem.as_ptr(), 2, 16), TRUE);
            KB_LL_EVENTS.lock().unwrap().clear();
            MS_LL_EVENTS.lock().unwrap().clear();
            super::super::misc::set_force_defer_guest_hooks(true);
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            native_mouse(NativeMouse::Button {
                hwnd: hwnd as usize,
                client: Some(POINT { x: 20, y: 25 }),
                screen: POINT { x: 20, y: 25 },
                button: MouseButton::Right,
                down: true,
                clicks: 1,
            });
            super::super::misc::set_force_defer_guest_hooks(false);
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::KEYFIRST, wm::KEYLAST, message::PM_REMOVE), TRUE);
            assert_eq!(message::PeekMessageW(&mut m, hwnd, wm::MOUSEFIRST, wm::MOUSELAST, message::PM_REMOVE), TRUE);
            assert_eq!(KB_LL_EVENTS.lock().unwrap().clone(), vec![(wm::KEYDOWN, b'A' as u32)]);
            assert_eq!(MS_LL_EVENTS.lock().unwrap().clone(), vec![(wm::RBUTTONDOWN, 20, 25, 0)]);

            assert_eq!(super::super::misc::UnhookWindowsHookEx(h_kb), TRUE);
            assert_eq!(super::super::misc::UnhookWindowsHookEx(h_ms), TRUE);
            release_all_keys();
            let _ = window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn ridev_nohotkeys_disables_hotkey_interception_and_delivers_raw_keydown() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let cls = crate::strings::to_wide_z("RawNoHotkeysTestWnd");
            let wc = class::WNDCLASS {
                style: 0,
                wndproc: register_typed_host_wndproc(defproc::DefWindowProcW),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = class::RegisterClassW(&wc);
            let hwnd = window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                ws::POPUP,
                0,
                0,
                200,
                200,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            set_active_window(hwnd as usize, false);
            set_focus(hwnd as usize);

            // Register Ctrl+A as a hotkey.
            assert_eq!(
                super::super::misc::RegisterHotKey(hwnd, 55, 0x0002 /* MOD_CONTROL */, b'A' as u32),
                TRUE
            );

            // Register raw keyboard with RIDEV_NOHOTKEYS | RIDEV_NOLEGACY.
            let dev = RAWINPUTDEVICE {
                usage_page: 1,
                usage: 6,
                flags: RIDEV_NOHOTKEYS | RIDEV_NOLEGACY,
                target: hwnd as usize,
            };
            assert_eq!(RegisterRawInputDevices(&dev, 1, 16), TRUE);

            // Press and release Ctrl+A: hotkey must NOT fire, and raw input must receive BOTH
            // keydown and keyup for 'A' (not just an orphaned keyup).
            native_key(NativeKey { mac: 0x3B, down: true, repeat: false, chars: Vec::new() });
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            native_key(NativeKey { mac: 0x00, down: false, repeat: false, chars: Vec::new() });
            native_key(NativeKey { mac: 0x3B, down: false, repeat: false, chars: Vec::new() });
            release_all_keys();

            let mut m = MSG::default();
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::HOTKEY, wm::HOTKEY, message::PM_REMOVE),
                FALSE,
                "RIDEV_NOHOTKEYS must disable WM_HOTKEY interception"
            );

            // Inspect WM_INPUT records for 'A' (VK 0x41): both make (flags & 1 == 0) and break (flags & 1 == 1).
            let mut a_events: Vec<(u16, u32)> = Vec::new();
            while message::PeekMessageW(&mut m, hwnd, wm::INPUT, wm::INPUT, message::PM_REMOVE) != 0 {
                let mut buf = [0u8; 64];
                let mut sz = buf.len() as u32;
                let got = GetRawInputData(
                    m.lparam as usize,
                    RID_INPUT,
                    buf.as_mut_ptr(),
                    &mut sz,
                    HEADER_SIZE as u32,
                );
                assert_eq!(got, 40);
                let flags = u16::from_le_bytes([buf[26], buf[27]]);
                let vk = u16::from_le_bytes([buf[30], buf[31]]);
                let msg = u32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]);
                if vk == b'A' as u16 {
                    a_events.push((flags, msg));
                }
            }
            assert_eq!(
                a_events,
                vec![(0, wm::KEYDOWN), (1, wm::KEYUP)],
                "raw keyboard consumer must receive both keydown and keyup under RIDEV_NOHOTKEYS"
            );

            // Removing the raw keyboard registration restores normal hotkey interception.
            let rem = RAWINPUTDEVICE { usage_page: 1, usage: 6, flags: RIDEV_REMOVE, target: 0 };
            assert_eq!(RegisterRawInputDevices(&rem, 1, 16), TRUE);
            native_key(NativeKey { mac: 0x3B, down: true, repeat: false, chars: Vec::new() });
            native_key(NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            release_all_keys();
            assert_eq!(
                message::PeekMessageW(&mut m, hwnd, wm::HOTKEY, wm::HOTKEY, message::PM_REMOVE),
                TRUE,
                "removing RIDEV_NOHOTKEYS registration must restore WM_HOTKEY"
            );
            assert_eq!(m.wparam, 55);

            assert_eq!(super::super::misc::UnregisterHotKey(hwnd, 55), TRUE);
            let _ = window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }
}
