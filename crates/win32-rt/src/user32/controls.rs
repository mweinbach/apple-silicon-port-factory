//! Built-in USER32 control classes: `BUTTON`, `EDIT`, and `STATIC`.
//!
//! Provides default Win32 window procedures (`button_wndproc`, `edit_wndproc`,
//! `static_wndproc`, `default_control_wndproc`) and main-thread-safe native
//! event bridges (`native_button_clicked`, `native_edit_text_changed`,
//! `native_static_clicked`) that queue Win32 `WM_COMMAND` notifications onto
//! the control's owning thread without ever invoking guest code on the AppKit
//! main thread.

use super::cursor;
use super::defproc::DefWindowProcW;
use super::input;
use super::message::{self, send_message};
use super::window;
use super::*;
use crate::macos::NativeControlKind;
use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Button styles, states, and notifications
// ---------------------------------------------------------------------------

pub const BS_TYPEMASK: u32 = 0x0000_000F;
pub const BS_PUSHBUTTON: u32 = 0x0000_0000;
pub const BS_DEFPUSHBUTTON: u32 = 0x0000_0001;
pub const BS_CHECKBOX: u32 = 0x0000_0002;
pub const BS_AUTOCHECKBOX: u32 = 0x0000_0003;
pub const BS_RADIOBUTTON: u32 = 0x0000_0004;
pub const BS_3STATE: u32 = 0x0000_0005;
pub const BS_AUTO3STATE: u32 = 0x0000_0006;
pub const BS_GROUPBOX: u32 = 0x0000_0007;
pub const BS_USERBUTTON: u32 = 0x0000_0008;
pub const BS_AUTORADIOBUTTON: u32 = 0x0000_0009;
pub const BS_SPLITBUTTON: u32 = 0x0000_000C;
pub const BS_DEFSPLITBUTTON: u32 = 0x0000_000D;
pub const BS_COMMANDLINK: u32 = 0x0000_000E;
pub const BS_DEFCOMMANDLINK: u32 = 0x0000_000F;

pub const BST_UNCHECKED: u32 = 0x0000;
pub const BST_CHECKED: u32 = 0x0001;
pub const BST_INDETERMINATE: u32 = 0x0002;
pub const BST_PUSHED: u32 = 0x0004;
pub const BST_FOCUS: u32 = 0x0008;

pub const BN_CLICKED: usize = 0;

// ---------------------------------------------------------------------------
// Edit styles and notifications
// ---------------------------------------------------------------------------

pub const ES_LEFT: u32 = 0x0000;
pub const ES_CENTER: u32 = 0x0001;
pub const ES_RIGHT: u32 = 0x0002;
pub const ES_MULTILINE: u32 = 0x0004;
pub const ES_UPPERCASE: u32 = 0x0008;
pub const ES_LOWERCASE: u32 = 0x0010;
pub const ES_PASSWORD: u32 = 0x0020;
pub const ES_AUTOVSCROLL: u32 = 0x0040;
pub const ES_AUTOHSCROLL: u32 = 0x0080;
pub const ES_NOHIDESEL: u32 = 0x0100;
pub const ES_READONLY: u32 = 0x0800;
pub const ES_WANTRETURN: u32 = 0x1000;
pub const ES_NUMBER: u32 = 0x2000;

pub const EN_SETFOCUS: usize = 0x0100;
pub const EN_KILLFOCUS: usize = 0x0200;
pub const EN_CHANGE: usize = 0x0300;
pub const EN_UPDATE: usize = 0x0400;

pub const DEFAULT_EDIT_LIMIT: usize = 30_000;

/// Truncate UTF-16 code units to `limit` without splitting a UTF-16 surrogate pair.
/// Returns `true` if truncation occurred.
pub(crate) fn truncate_utf16_at_limit(units: &mut Vec<u16>, limit: usize) -> bool {
    if limit == 0 || units.len() <= limit {
        return false;
    }
    let mut new_len = limit;
    if new_len > 0 && (0xD800..=0xDBFF).contains(&units[new_len - 1]) {
        new_len -= 1;
    }
    units.truncate(new_len);
    true
}

/// Result of applying Win32 `EDIT` input constraints (`ES_NUMBER`, `ES_UPPERCASE`,
/// `ES_LOWERCASE`, and range-preserving `EM_LIMITTEXT`) to a native edit transition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NormalizedEdit {
    pub units: Vec<u16>,
    pub sel_start: usize,
    pub sel_end: usize,
    /// True if `units` differs from `prev`.
    pub text_changed: bool,
    /// True if normalization altered `next` (so the native AppKit view must be reconciled).
    pub reconcile_native: bool,
}

/// Enforce `ES_NUMBER`, `ES_UPPERCASE`, `ES_LOWERCASE`, and `EM_LIMITTEXT` on the
/// inserted/replaced range between `prev` and `next` without deleting existing text
/// outside that range.
pub(crate) fn apply_native_edit_constraints(
    prev: &[u16],
    next: &[u16],
    sel_start: usize,
    sel_end: usize,
    style: u32,
    limit: usize,
) -> NormalizedEdit {
    let lo_in = sel_start.min(sel_end).min(next.len());
    let hi_in = sel_start.max(sel_end).min(next.len());

    // Find the minimal changed span [prefix .. len - suffix], anchored around the
    // post-edit selection so inserting duplicate characters preserves the suffix.
    let max_prefix = prev.len().min(lo_in);
    let mut prefix = 0usize;
    while prefix < max_prefix && prev[prefix] == next[prefix] {
        prefix += 1;
    }
    let max_suffix = (prev.len() - prefix).min(next.len() - hi_in);
    let mut suffix = 0usize;
    while suffix < max_suffix && prev[prev.len() - 1 - suffix] == next[next.len() - 1 - suffix] {
        suffix += 1;
    }

    let raw_inserted = &next[prefix..next.len() - suffix];
    let mut inserted: Vec<u16> = Vec::with_capacity(raw_inserted.len());

    if (style & ES_NUMBER) != 0 {
        for &u in raw_inserted {
            if (b'0' as u16..=b'9' as u16).contains(&u) {
                inserted.push(u);
            }
        }
    } else if (style & ES_UPPERCASE) != 0 {
        let s = String::from_utf16_lossy(raw_inserted);
        inserted.extend(s.to_uppercase().encode_utf16());
    } else if (style & ES_LOWERCASE) != 0 {
        let s = String::from_utf16_lossy(raw_inserted);
        inserted.extend(s.to_lowercase().encode_utf16());
    } else {
        inserted.extend_from_slice(raw_inserted);
    }

    // Enforce EM_LIMITTEXT strictly against the newly inserted slice so existing
    // prefix/suffix text outside the edited range is never deleted.
    if limit > 0 && !inserted.is_empty() {
        let base_len = prefix + suffix;
        let avail = limit.saturating_sub(base_len);
        if avail == 0 {
            inserted.clear();
        } else {
            truncate_utf16_at_limit(&mut inserted, avail);
        }
    }

    let mut out = Vec::with_capacity(prefix + inserted.len() + suffix);
    out.extend_from_slice(&prev[..prefix]);
    out.extend_from_slice(&inserted);
    out.extend_from_slice(&prev[prev.len() - suffix..]);

    let reconcile_native = out != next;
    let text_changed = out != prev;
    let (out_lo, out_hi) = if reconcile_native {
        let caret = (prefix + inserted.len()).min(out.len());
        (caret, caret)
    } else {
        (lo_in.min(out.len()), hi_in.min(out.len()))
    };

    NormalizedEdit {
        units: out,
        sel_start: out_lo,
        sel_end: out_hi,
        text_changed,
        reconcile_native,
    }
}

// ---------------------------------------------------------------------------
// Static styles and notifications
// ---------------------------------------------------------------------------

pub const SS_TYPEMASK: u32 = 0x0000_001F;
pub const SS_LEFT: u32 = 0x0000;
pub const SS_CENTER: u32 = 0x0001;
pub const SS_RIGHT: u32 = 0x0002;
pub const SS_ICON: u32 = 0x0003;
pub const SS_BLACKRECT: u32 = 0x0004;
pub const SS_GRAYRECT: u32 = 0x0005;
pub const SS_WHITERECT: u32 = 0x0006;
pub const SS_BLACKFRAME: u32 = 0x0007;
pub const SS_GRAYFRAME: u32 = 0x0008;
pub const SS_WHITEFRAME: u32 = 0x0009;
pub const SS_SIMPLE: u32 = 0x000B;
pub const SS_LEFTNOWORDWRAP: u32 = 0x000C;
pub const SS_BITMAP: u32 = 0x000E;
pub const SS_ETCHEDHORZ: u32 = 0x0010;
pub const SS_ETCHEDVERT: u32 = 0x0011;
pub const SS_ETCHEDFRAME: u32 = 0x0012;
pub const SS_NOPREFIX: u32 = 0x0080;
pub const SS_NOTIFY: u32 = 0x0100;
pub const SS_CENTERIMAGE: u32 = 0x0200;

pub const STN_CLICKED: usize = 0;
pub const STN_DBLCLK: usize = 1;

pub const IMAGE_BITMAP: u32 = 0;
pub const IMAGE_ICON: u32 = 1;

// ---------------------------------------------------------------------------
// Dialog code bits
// ---------------------------------------------------------------------------

pub const DLGC_WANTARROWS: isize = 0x0001;
pub const DLGC_WANTMESSAGE: isize = 0x0004;
pub const DLGC_DEFPUSHBUTTON: isize = 0x0010;
pub const DLGC_UNDEFPUSHBUTTON: isize = 0x0020;
pub const DLGC_RADIOBUTTON: isize = 0x0040;
pub const DLGC_WANTCHARS: isize = 0x0080;
pub const DLGC_STATIC: isize = 0x0100;
pub const DLGC_BUTTON: isize = 0x2000;

const VK_RETURN: usize = 0x0D;
const VK_SPACE: usize = 0x20;
const VK_DELETE: usize = 0x2E;

// ---------------------------------------------------------------------------
// Helpers & per-control state tables
// ---------------------------------------------------------------------------

/// Identify whether a registered window class maps to a built-in native AppKit control.
pub fn control_kind_for_class(name: &str) -> Option<NativeControlKind> {
    if name.eq_ignore_ascii_case("Button") {
        Some(NativeControlKind::Button)
    } else if name.eq_ignore_ascii_case("Edit") {
        Some(NativeControlKind::Edit)
    } else if name.eq_ignore_ascii_case("Static") {
        Some(NativeControlKind::Static)
    } else {
        None
    }
}

/// Strip Win32 `&` accelerator mnemonics for AppKit display (`"&OK"` -> `"OK"`, `"A && B"` -> `"A & B"`).
pub fn strip_win32_mnemonic(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '&' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[derive(Default, Clone, Debug)]
struct ButtonState {
    check: u32,
    pushed: bool,
    image: usize,
    image_type: u32,
    font: usize,
}

#[derive(Clone, Debug)]
struct EditState {
    sel_start: usize,
    sel_end: usize,
    limit: usize,
    modified: bool,
    password_char: u16,
    font: usize,
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            sel_start: 0,
            sel_end: 0,
            limit: DEFAULT_EDIT_LIMIT,
            modified: false,
            password_char: 0,
            font: 0,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct StaticState {
    image: usize,
    image_type: u32,
    font: usize,
}

fn button_states() -> &'static Mutex<HashMap<usize, ButtonState>> {
    static B: OnceLock<Mutex<HashMap<usize, ButtonState>>> = OnceLock::new();
    B.get_or_init(|| Mutex::new(HashMap::new()))
}

fn edit_states() -> &'static Mutex<HashMap<usize, EditState>> {
    static E: OnceLock<Mutex<HashMap<usize, EditState>>> = OnceLock::new();
    E.get_or_init(|| Mutex::new(HashMap::new()))
}

fn static_states() -> &'static Mutex<HashMap<usize, StaticState>> {
    static S: OnceLock<Mutex<HashMap<usize, StaticState>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Remove per-control state when an HWND is freed.
pub(crate) fn forget_control(hwnd: usize) {
    button_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
    edit_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
    static_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
}

fn notify_parent_command(hwnd: usize, code: usize) {
    let Some((parent, id)) = window::get(hwnd).map(|w| {
        let d = w.data();
        (d.parent, d.id as u16)
    }) else {
        return;
    };
    if parent != 0 && window::exists(parent) {
        send_message(
            parent,
            wm::COMMAND,
            ((code & 0xFFFF) << 16) | (id as usize),
            hwnd as LPARAM,
        );
    }
}

fn is_native_control(hwnd: usize) -> bool {
    window::get(hwnd).is_some_and(|w| w.data().native_control)
}

fn uncheck_sibling_auto_radios(hwnd: usize, parent: usize) {
    if parent == 0 {
        return;
    }
    let siblings = window::children_of(parent);
    let Some(pos) = siblings.iter().position(|&c| c == hwnd) else {
        return;
    };
    // Walk backward to the start of the WS_GROUP, and forward until the next WS_GROUP.
    let mut start = pos;
    while start > 0 {
        if window::get(siblings[start]).is_some_and(|w| (w.data().style & ws::GROUP) != 0) {
            break;
        }
        start -= 1;
    }
    let mut end = pos + 1;
    while end < siblings.len() {
        if window::get(siblings[end]).is_some_and(|w| (w.data().style & ws::GROUP) != 0) {
            break;
        }
        end += 1;
    }
    for &sib in &siblings[start..end] {
        if sib == hwnd {
            continue;
        }
        let is_auto_radio = window::get(sib).is_some_and(|w| {
            w.class.name.eq_ignore_ascii_case("Button")
                && (w.data().style & BS_TYPEMASK) == BS_AUTORADIOBUTTON
        });
        if is_auto_radio {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(sib)
                .or_default()
                .check = BST_UNCHECKED;
            if is_native_control(sib) {
                crate::macos::control_set_check(sib, BST_UNCHECKED);
            }
        }
    }
}

fn activate_button_click(hwnd: usize, native_check_override: Option<u32>) {
    let Some((btn_type, parent, native)) = window::get(hwnd).map(|w| {
        let d = w.data();
        (d.style & BS_TYPEMASK, d.parent, d.native_control)
    }) else {
        return;
    };

    match btn_type {
        BS_AUTOCHECKBOX => {
            let next = {
                let mut map = button_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(hwnd).or_default();
                st.check = native_check_override
                    .map(|c| c & 0x3)
                    .unwrap_or(if st.check == BST_CHECKED { BST_UNCHECKED } else { BST_CHECKED });
                st.check
            };
            if native && native_check_override.is_none() {
                crate::macos::control_set_check(hwnd, next);
            }
        }
        BS_AUTO3STATE => {
            let next = {
                let mut map = button_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(hwnd).or_default();
                st.check = native_check_override
                    .map(|c| c & 0x3)
                    .unwrap_or((st.check + 1) % 3);
                st.check
            };
            if native && native_check_override.is_none() {
                crate::macos::control_set_check(hwnd, next);
            }
        }
        BS_AUTORADIOBUTTON => {
            {
                let mut map = button_states().lock().unwrap_or_else(|e| e.into_inner());
                map.entry(hwnd).or_default().check = BST_CHECKED;
            }
            if native && native_check_override.is_none() {
                crate::macos::control_set_check(hwnd, BST_CHECKED);
            }
            uncheck_sibling_auto_radios(hwnd, parent);
        }
        BS_CHECKBOX | BS_3STATE | BS_RADIOBUTTON => {
            // Manual check/radio styles are controlled solely by the application via
            // BM_SETCHECK; if AppKit visually toggled the NSButton on click, restore
            // the stored Win32 check state before notifying the parent.
            if native && native_check_override.is_some() {
                let cur = button_states()
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .get(&hwnd)
                    .map(|s| s.check)
                    .unwrap_or(BST_UNCHECKED);
                crate::macos::control_set_check(hwnd, cur);
            }
        }
        _ => {}
    }

    notify_parent_command(hwnd, BN_CLICKED);
}

// ---------------------------------------------------------------------------
// Native AppKit -> Win32 event bridges (called on the AppKit main thread)
// ---------------------------------------------------------------------------

/// Queue a native AppKit `NSButton` click onto the button's owning Win32 thread.
pub fn native_button_clicked(hwnd: usize, native_check_state: Option<u32>) {
    let Some(w) = window::get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    q.call(move || {
        let enabled = window::get(hwnd).is_some_and(|w| w.data().enabled);
        if enabled {
            activate_button_click(hwnd, native_check_state);
        }
    });
}

/// Queue a native AppKit `NSTextField` / `NSTextView` text edit and its live
/// UTF-16 selection range onto the owning Win32 thread.
///
/// Applies `ES_NUMBER`, `ES_UPPERCASE`, `ES_LOWERCASE`, and range-preserving
/// `EM_LIMITTEXT` against the edited span; whenever normalization alters the
/// text, the reconciled text and selection are pushed back to AppKit so
/// displayed text and `GetWindowText`/`EN_CHANGE` always agree.
pub fn native_edit_text_changed(hwnd: usize, text: String, sel_start: usize, sel_end: usize) {
    let Some(w) = window::get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    q.call(move || {
        let Some(w) = window::get(hwnd) else { return };
        let (style, native, prev_units) = {
            let d = w.data();
            (d.style, d.native_control, d.text.clone())
        };
        if (style & ES_READONLY) != 0 {
            return;
        }
        let next_units: Vec<u16> = text.encode_utf16().collect();
        let limit = edit_states()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&hwnd)
            .map(|s| s.limit)
            .unwrap_or(DEFAULT_EDIT_LIMIT);
        let norm = apply_native_edit_constraints(
            &prev_units,
            &next_units,
            sel_start,
            sel_end,
            style,
            limit,
        );
        if norm.reconcile_native {
            // Reconcile the displayed native AppKit text + selection with the normalized Win32 text.
            window::set_internal_text(hwnd, norm.units);
            if native {
                crate::macos::control_set_selection(hwnd, norm.sel_start, norm.sel_end);
            }
        } else {
            window::set_internal_text_from_native(hwnd, norm.units);
        }
        {
            let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
            let st = map.entry(hwnd).or_default();
            if norm.text_changed {
                st.modified = true;
            }
            st.sel_start = norm.sel_start;
            st.sel_end = norm.sel_end;
        }
        if norm.text_changed {
            notify_parent_command(hwnd, EN_UPDATE);
            notify_parent_command(hwnd, EN_CHANGE);
        }
    });
}

/// Queue a native AppKit caret/selection change (without text modification) onto
/// the owning Win32 thread so `EM_GETSEL` and `EM_REPLACESEL` track the native caret.
pub fn native_edit_selection_changed(hwnd: usize, sel_start: usize, sel_end: usize) {
    let Some(w) = window::get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    q.call(move || {
        let Some(len) = window::internal_text(hwnd).map(|t| t.len()) else { return };
        let lo = sel_start.min(sel_end).min(len);
        let hi = sel_start.max(sel_end).min(len);
        let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
        let st = map.entry(hwnd).or_default();
        st.sel_start = lo;
        st.sel_end = hi;
    });
}

/// Queue a native AppKit focus gain on a child control onto the owning Win32 thread
/// so `GetFocus()` and `IsDialogMessage` target the focused control.
pub fn native_control_focused(hwnd: usize) {
    let Some(w) = window::get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    q.call(move || {
        if window::exists(hwnd) && input::focus_window() != hwnd {
            input::set_focus(hwnd);
        }
    });
}

/// Queue a native AppKit click on a `STATIC` control (delivers `STN_CLICKED` if `SS_NOTIFY`).
pub fn native_static_clicked(hwnd: usize) {
    let Some(w) = window::get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    q.call(move || {
        let should_notify = window::get(hwnd).is_some_and(|w| {
            let d = w.data();
            d.enabled && (d.style & SS_NOTIFY) != 0
        });
        if should_notify {
            notify_parent_command(hwnd, STN_CLICKED);
        }
    });
}

// ---------------------------------------------------------------------------
// BUTTON window procedure
// ---------------------------------------------------------------------------

pub(crate) unsafe extern "win64" fn button_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let h = hwnd as usize;
    match msg {
        wm::CREATE => {
            button_states().lock().unwrap_or_else(|e| e.into_inner()).entry(h).or_default();
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        wm::BM_GETCHECK => {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.check)
                .unwrap_or(BST_UNCHECKED) as LRESULT
        }
        wm::BM_SETCHECK => {
            let check = (wparam as u32) & 0x3;
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .check = check;
            if is_native_control(h) {
                crate::macos::control_set_check(h, check);
            }
            0
        }
        wm::BM_GETSTATE => {
            let (check, pushed) = button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| (s.check, s.pushed))
                .unwrap_or((BST_UNCHECKED, false));
            let mut st = check & 0x3;
            if pushed {
                st |= BST_PUSHED;
            }
            if input::focus_window() == h {
                st |= BST_FOCUS;
            }
            st as LRESULT
        }
        wm::BM_SETSTATE => {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .pushed = wparam != 0;
            0
        }
        wm::BM_SETSTYLE => {
            let new_style = window::get(h).map(|w| {
                let mut d = w.data();
                d.style = (d.style & !0xFFFF) | ((wparam as u32) & 0xFFFF);
                (d.style, d.native_control)
            });
            if let Some((style, true)) = new_style {
                crate::macos::control_set_style(h, style);
            }
            0
        }
        wm::BM_CLICK => {
            activate_button_click(h, None);
            0
        }
        wm::BM_GETIMAGE => {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .filter(|s| s.image_type == wparam as u32)
                .map(|s| s.image as LRESULT)
                .unwrap_or(0)
        }
        wm::BM_SETIMAGE => {
            let mut map = button_states().lock().unwrap_or_else(|e| e.into_inner());
            let st = map.entry(h).or_default();
            let prev = st.image;
            st.image_type = wparam as u32;
            st.image = lparam as usize;
            prev as LRESULT
        }
        wm::LBUTTONDOWN => {
            let enabled = window::get(h).is_some_and(|w| w.data().enabled);
            if enabled {
                input::set_focus(h);
                input::SetCapture(hwnd);
                button_states()
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .entry(h)
                    .or_default()
                    .pushed = true;
            }
            0
        }
        wm::LBUTTONUP => {
            let was_pushed = {
                let mut map = button_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                let was = st.pushed;
                st.pushed = false;
                was
            };
            if input::GetCapture() == hwnd {
                input::ReleaseCapture();
            }
            if was_pushed {
                let x = (lparam & 0xFFFF) as i16 as i32;
                let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
                let inside = window::client_size(h)
                    .is_some_and(|(cw, ch)| x >= 0 && y >= 0 && x < cw && y < ch);
                let enabled = window::get(h).is_some_and(|w| w.data().enabled);
                if inside && enabled {
                    activate_button_click(h, None);
                }
            }
            0
        }
        wm::CAPTURECHANGED => {
            if let Some(st) = button_states().lock().unwrap_or_else(|e| e.into_inner()).get_mut(&h) {
                st.pushed = false;
            }
            0
        }
        wm::KEYUP if wparam == VK_SPACE => {
            if window::get(h).is_some_and(|w| w.data().enabled) {
                activate_button_click(h, None);
            }
            0
        }
        wm::SETFONT => {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .font = wparam;
            0
        }
        wm::GETFONT => {
            button_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.font as LRESULT)
                .unwrap_or(0)
        }
        wm::GETDLGCODE => {
            let btn_type = window::get(h).map(|w| w.data().style & BS_TYPEMASK).unwrap_or(BS_PUSHBUTTON);
            match btn_type {
                BS_DEFPUSHBUTTON | BS_DEFSPLITBUTTON | BS_DEFCOMMANDLINK => {
                    DLGC_BUTTON | DLGC_DEFPUSHBUTTON
                }
                BS_PUSHBUTTON | BS_USERBUTTON | BS_SPLITBUTTON | BS_COMMANDLINK => {
                    DLGC_BUTTON | DLGC_UNDEFPUSHBUTTON
                }
                BS_RADIOBUTTON | BS_AUTORADIOBUTTON => DLGC_BUTTON | DLGC_RADIOBUTTON,
                BS_GROUPBOX => DLGC_STATIC,
                _ => DLGC_BUTTON,
            }
        }
        wm::NCDESTROY => {
            button_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&h);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ---------------------------------------------------------------------------
// EDIT window procedure
// ---------------------------------------------------------------------------

/// Compute line start/end ranges (excluding `\r` and `\n`) in UTF-16 code units.
fn edit_line_ranges(text: &[u16]) -> Vec<(usize, usize)> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < text.len() {
        if text[i] == b'\r' as u16 {
            lines.push((start, i));
            i += 1;
            if i < text.len() && text[i] == b'\n' as u16 {
                i += 1;
            }
            start = i;
        } else if text[i] == b'\n' as u16 {
            lines.push((start, i));
            i += 1;
            start = i;
        } else {
            i += 1;
        }
    }
    lines.push((start, text.len()));
    lines
}

fn edit_replace_selection(hwnd: usize, insert: &[u16], respect_limit: bool) {
    let mut text = window::internal_text(hwnd).unwrap_or_default();
    let (sel_start, sel_end, limit) = {
        let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
        let st = map.entry(hwnd).or_default();
        let lo = st.sel_start.min(st.sel_end).min(text.len());
        let hi = st.sel_start.max(st.sel_end).min(text.len());
        (lo, hi, st.limit)
    };
    let mut to_insert = insert;
    if respect_limit && limit > 0 {
        let base_len = text.len() - (sel_end - sel_start);
        let avail = limit.saturating_sub(base_len);
        if to_insert.len() > avail {
            to_insert = &to_insert[..avail];
        }
    }
    text.splice(sel_start..sel_end, to_insert.iter().copied());
    let new_caret = sel_start + to_insert.len();
    window::set_internal_text(hwnd, text);
    {
        let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
        let st = map.entry(hwnd).or_default();
        st.sel_start = new_caret;
        st.sel_end = new_caret;
        st.modified = true;
    }
    if is_native_control(hwnd) {
        crate::macos::control_set_selection(hwnd, new_caret, new_caret);
    }
    notify_parent_command(hwnd, EN_UPDATE);
    notify_parent_command(hwnd, EN_CHANGE);
}

pub(crate) unsafe extern "win64" fn edit_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let h = hwnd as usize;
    match msg {
        wm::CREATE => {
            let style = window::get(h).map(|w| w.data().style).unwrap_or(0);
            let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
            let st = map.entry(h).or_default();
            if (style & ES_PASSWORD) != 0 {
                st.password_char = b'*' as u16;
            }
            drop(map);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        wm::SETTEXT => {
            let res = DefWindowProcW(hwnd, msg, wparam, lparam);
            {
                let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                st.sel_start = 0;
                st.sel_end = 0;
                st.modified = false;
            }
            if is_native_control(h) {
                crate::macos::control_set_selection(h, 0, 0);
            }
            notify_parent_command(h, EN_UPDATE);
            notify_parent_command(h, EN_CHANGE);
            res
        }
        wm::CHAR => {
            let style = window::get(h).map(|w| w.data().style).unwrap_or(0);
            if (style & ES_READONLY) != 0 {
                return 0;
            }
            let ch = (wparam & 0xFFFF) as u16;
            match ch {
                0x08 => {
                    // Backspace: delete selection, or one code unit before caret.
                    let mut text = window::internal_text(h).unwrap_or_default();
                    let (mut lo, hi) = {
                        let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                        let st = map.entry(h).or_default();
                        (
                            st.sel_start.min(st.sel_end).min(text.len()),
                            st.sel_start.max(st.sel_end).min(text.len()),
                        )
                    };
                    if lo == hi {
                        if lo == 0 {
                            return 0;
                        }
                        lo -= 1;
                    }
                    text.drain(lo..hi);
                    window::set_internal_text(h, text);
                    {
                        let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                        let st = map.entry(h).or_default();
                        st.sel_start = lo;
                        st.sel_end = lo;
                        st.modified = true;
                    }
                    if is_native_control(h) {
                        crate::macos::control_set_selection(h, lo, lo);
                    }
                    notify_parent_command(h, EN_UPDATE);
                    notify_parent_command(h, EN_CHANGE);
                }
                0x0D => {
                    if (style & ES_MULTILINE) != 0 {
                        edit_replace_selection(h, &[b'\r' as u16, b'\n' as u16], true);
                    }
                }
                c if c >= 0x20 => {
                    if (style & ES_NUMBER) != 0 && !(b'0' as u16..=b'9' as u16).contains(&c) {
                        return 0;
                    }
                    let mapped = if (style & ES_UPPERCASE) != 0 && (b'a' as u16..=b'z' as u16).contains(&c) {
                        c - 32
                    } else if (style & ES_LOWERCASE) != 0 && (b'A' as u16..=b'Z' as u16).contains(&c) {
                        c + 32
                    } else {
                        c
                    };
                    edit_replace_selection(h, &[mapped], true);
                }
                _ => {}
            }
            0
        }
        wm::KEYDOWN if wparam == VK_DELETE => {
            let style = window::get(h).map(|w| w.data().style).unwrap_or(0);
            if (style & ES_READONLY) != 0 {
                return 0;
            }
            let mut text = window::internal_text(h).unwrap_or_default();
            let (lo, mut hi) = {
                let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                (
                    st.sel_start.min(st.sel_end).min(text.len()),
                    st.sel_start.max(st.sel_end).min(text.len()),
                )
            };
            if lo == hi {
                if hi >= text.len() {
                    return 0;
                }
                hi += 1;
            }
            text.drain(lo..hi);
            window::set_internal_text(h, text);
            {
                let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                st.sel_start = lo;
                st.sel_end = lo;
                st.modified = true;
            }
            if is_native_control(h) {
                crate::macos::control_set_selection(h, lo, lo);
            }
            notify_parent_command(h, EN_UPDATE);
            notify_parent_command(h, EN_CHANGE);
            0
        }
        wm::SETFOCUS => {
            notify_parent_command(h, EN_SETFOCUS);
            0
        }
        wm::KILLFOCUS => {
            notify_parent_command(h, EN_KILLFOCUS);
            0
        }
        wm::EM_GETSEL => {
            let len = window::internal_text(h).map(|t| t.len()).unwrap_or(0);
            let (start, end) = {
                let map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                map.get(&h)
                    .map(|s| (s.sel_start.min(len), s.sel_end.min(len)))
                    .unwrap_or((0, 0))
            };
            if wparam != 0 {
                (wparam as *mut u32).write_unaligned(start as u32);
            }
            if lparam != 0 {
                (lparam as *mut u32).write_unaligned(end as u32);
            }
            make_long((start as u32 & 0xFFFF) as i32, (end as u32 & 0xFFFF) as i32)
        }
        wm::EM_SETSEL => {
            let len = window::internal_text(h).map(|t| t.len()).unwrap_or(0);
            let (sel_start, sel_end) = {
                let mut map = edit_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                let wp = wparam as isize;
                if wp == -1 {
                    // Remove selection, keep caret at previous end.
                    let caret = st.sel_end.min(len);
                    st.sel_start = caret;
                    st.sel_end = caret;
                } else {
                    let start = (wp.max(0) as usize).min(len);
                    let end = if lparam == -1 {
                        len
                    } else {
                        (lparam.max(0) as usize).min(len)
                    };
                    st.sel_start = start.min(end);
                    st.sel_end = start.max(end);
                }
                (st.sel_start, st.sel_end)
            };
            if is_native_control(h) {
                crate::macos::control_set_selection(h, sel_start, sel_end);
            }
            0
        }
        wm::EM_SCROLLCARET => 1,
        wm::EM_GETMODIFY => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.modified as LRESULT)
                .unwrap_or(0)
        }
        wm::EM_SETMODIFY => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .modified = wparam != 0;
            0
        }
        wm::EM_GETLINECOUNT => {
            let text = window::internal_text(h).unwrap_or_default();
            edit_line_ranges(&text).len() as LRESULT
        }
        wm::EM_LINEINDEX => {
            let text = window::internal_text(h).unwrap_or_default();
            let lines = edit_line_ranges(&text);
            let wp = wparam as isize;
            if wp == -1 {
                let caret = edit_states()
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .get(&h)
                    .map(|s| s.sel_start.min(text.len()))
                    .unwrap_or(0);
                for &(start, end) in &lines {
                    if caret >= start && caret <= end {
                        return start as LRESULT;
                    }
                }
                lines.last().map(|l| l.0 as LRESULT).unwrap_or(0)
            } else if wp < 0 || (wp as usize) >= lines.len() {
                -1
            } else {
                lines[wp as usize].0 as LRESULT
            }
        }
        wm::EM_LINELENGTH => {
            let text = window::internal_text(h).unwrap_or_default();
            let lines = edit_line_ranges(&text);
            let idx = if (wparam as isize) == -1 {
                edit_states()
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .get(&h)
                    .map(|s| s.sel_start.min(text.len()))
                    .unwrap_or(0)
            } else {
                wparam.min(text.len())
            };
            for &(start, end) in &lines {
                if idx >= start && idx <= end {
                    return (end - start) as LRESULT;
                }
            }
            0
        }
        wm::EM_REPLACESEL => {
            let insert = if lparam == 0 {
                Vec::new()
            } else {
                crate::strings::wide_slice(lparam as *const u16).to_vec()
            };
            edit_replace_selection(h, &insert, false);
            0
        }
        wm::EM_GETLINE => {
            if lparam == 0 {
                return 0;
            }
            let max_chars = (lparam as *const u16).read_unaligned() as usize;
            if max_chars == 0 {
                return 0;
            }
            let text = window::internal_text(h).unwrap_or_default();
            let lines = edit_line_ranges(&text);
            let Some(&(start, end)) = lines.get(wparam) else {
                return 0;
            };
            let line = &text[start..end];
            let n = line.len().min(max_chars);
            let dst = lparam as *mut u16;
            for (i, &u) in line[..n].iter().enumerate() {
                dst.add(i).write_unaligned(u);
            }
            n as LRESULT
        }
        wm::EM_LIMITTEXT => {
            let limit = if wparam == 0 { DEFAULT_EDIT_LIMIT } else { wparam };
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .limit = limit;
            if is_native_control(h) {
                crate::macos::control_set_limit(h, limit);
            }
            0
        }
        wm::EM_GETLIMITTEXT => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.limit)
                .unwrap_or(DEFAULT_EDIT_LIMIT) as LRESULT
        }
        wm::EM_SETREADONLY => {
            let readonly = wparam != 0;
            let native = window::get(h).map(|w| {
                let mut d = w.data();
                if readonly {
                    d.style |= ES_READONLY;
                } else {
                    d.style &= !ES_READONLY;
                }
                d.native_control
            });
            if native == Some(true) {
                crate::macos::control_set_readonly(h, readonly);
            }
            1
        }
        wm::EM_SETPASSWORDCHAR => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .password_char = (wparam & 0xFFFF) as u16;
            0
        }
        wm::EM_GETPASSWORDCHAR => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.password_char as LRESULT)
                .unwrap_or(0)
        }
        wm::SETFONT => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .font = wparam;
            0
        }
        wm::GETFONT => {
            edit_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.font as LRESULT)
                .unwrap_or(0)
        }
        wm::GETDLGCODE => {
            let mut code = DLGC_WANTARROWS | DLGC_WANTCHARS;
            if h != 0 && lparam != 0 {
                let style = window::get(h).map(|w| w.data().style).unwrap_or(0);
                if (style & ES_WANTRETURN) != 0 {
                    let m = (lparam as *const MSG).read_unaligned();
                    if m.message == wm::KEYDOWN && m.wparam == VK_RETURN {
                        code |= DLGC_WANTMESSAGE;
                    }
                }
            }
            code
        }
        wm::NCDESTROY => {
            edit_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&h);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ---------------------------------------------------------------------------
// STATIC window procedure
// ---------------------------------------------------------------------------

pub(crate) unsafe extern "win64" fn static_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let h = hwnd as usize;
    match msg {
        wm::CREATE => {
            static_states().lock().unwrap_or_else(|e| e.into_inner()).entry(h).or_default();
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        wm::STM_SETICON => {
            let prev = {
                let mut map = static_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                let prev = st.image;
                st.image_type = IMAGE_ICON;
                st.image = wparam;
                prev
            };
            if is_native_control(h) {
                crate::macos::control_set_icon(h, cursor::icon_payload(wparam));
            }
            prev as LRESULT
        }
        wm::STM_GETICON => {
            static_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .filter(|s| s.image_type == IMAGE_ICON)
                .map(|s| s.image as LRESULT)
                .unwrap_or(0)
        }
        wm::STM_SETIMAGE => {
            let ty = wparam as u32;
            let handle = lparam as usize;
            let prev = {
                let mut map = static_states().lock().unwrap_or_else(|e| e.into_inner());
                let st = map.entry(h).or_default();
                let prev = st.image;
                st.image_type = ty;
                st.image = handle;
                prev
            };
            if ty == IMAGE_ICON && is_native_control(h) {
                crate::macos::control_set_icon(h, cursor::icon_payload(handle));
            }
            prev as LRESULT
        }
        wm::STM_GETIMAGE => {
            static_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .filter(|s| s.image_type == wparam as u32)
                .map(|s| s.image as LRESULT)
                .unwrap_or(0)
        }
        wm::LBUTTONDOWN => {
            let should_notify = window::get(h).is_some_and(|w| {
                let d = w.data();
                d.enabled && (d.style & SS_NOTIFY) != 0
            });
            if should_notify {
                notify_parent_command(h, STN_CLICKED);
            }
            0
        }
        wm::LBUTTONDBLCLK => {
            let should_notify = window::get(h).is_some_and(|w| {
                let d = w.data();
                d.enabled && (d.style & SS_NOTIFY) != 0
            });
            if should_notify {
                notify_parent_command(h, STN_DBLCLK);
            }
            0
        }
        wm::SETFONT => {
            static_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .entry(h)
                .or_default()
                .font = wparam;
            0
        }
        wm::GETFONT => {
            static_states()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|s| s.font as LRESULT)
                .unwrap_or(0)
        }
        wm::GETDLGCODE => DLGC_STATIC,
        wm::NCDESTROY => {
            static_states().lock().unwrap_or_else(|e| e.into_inner()).remove(&h);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ---------------------------------------------------------------------------
// Generic default control window procedure (ListBox, ScrollBar, ComboBox)
// ---------------------------------------------------------------------------

pub(crate) unsafe extern "win64" fn default_control_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::class::{
        GetClassInfoExA, GetClassInfoExW, GetClassInfoW, RegisterClassExW, UnregisterClassW,
        WNDCLASS, WNDCLASSEX,
    };
    use super::super::dialog::{GetDlgItem, GetDlgItemTextA};
    use super::super::message::{PeekMessageW, SendMessageA, SendMessageW, PM_REMOVE};
    use super::super::window::{
        CreateWindowExA, CreateWindowExW, DestroyWindow, EnableWindow, GetWindowLongPtrW,
        GetWindowTextA, GetWindowTextLengthW, GetWindowTextW, IsWindow, IsWindowEnabled,
        IsWindowVisible, SetWindowTextA, SetWindowTextW, ShowWindow, GWLP_WNDPROC, SW_HIDE,
        SW_SHOW,
    };

    static COMMAND_LOG: Mutex<Vec<(u16, u16, usize)>> = Mutex::new(Vec::new());

    unsafe extern "win64" fn parent_capture_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == wm::COMMAND {
            let id = (wparam & 0xFFFF) as u16;
            let code = ((wparam >> 16) & 0xFFFF) as u16;
            COMMAND_LOG.lock().unwrap_or_else(|e| e.into_inner()).push((id, code, lparam as usize));
            return 0;
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    unsafe fn make_parent(class_name: &str) -> (HWND, Vec<u16>) {
        let cls_w = crate::strings::to_wide_z(class_name);
        let proc_handle = super::super::register_typed_host_wndproc(parent_capture_proc);
        let wc = WNDCLASSEX {
            cb_size: std::mem::size_of::<WNDCLASSEX>() as u32,
            style: 0,
            wndproc: proc_handle,
            cls_extra: 0,
            wnd_extra: 0,
            instance: 0,
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: 0,
            class_name: cls_w.as_ptr() as usize,
            icon_sm: 0,
        };
        let _ = RegisterClassExW(&wc);
        let title = crate::strings::to_wide_z("Parent");
        let h = CreateWindowExW(
            0,
            cls_w.as_ptr(),
            title.as_ptr(),
            ws::CAPTION | ws::SYSMENU | ws::VISIBLE,
            20,
            20,
            500,
            360,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert!(!h.is_null(), "parent window creation failed");
        (h, cls_w)
    }

    #[test]
    fn builtin_control_classes_registered_for_unicode_and_ansi() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            let (parent, parent_cls) = make_parent("TestBuiltinClassesParent");

            // Query GetClassInfoW / GetClassInfoExW / GetClassInfoExA directly without
            // ever calling DialogBox* or CreateDialog*.
            for (name_w, name_a, expected_proc) in [
                ("EDIT", c"EDIT", super::super::WNDPROC_HANDLE_EDIT),
                ("Edit", c"edit", super::super::WNDPROC_HANDLE_EDIT),
                ("BUTTON", c"BUTTON", super::super::WNDPROC_HANDLE_BUTTON),
                ("Button", c"button", super::super::WNDPROC_HANDLE_BUTTON),
                ("STATIC", c"STATIC", super::super::WNDPROC_HANDLE_STATIC),
                ("Static", c"static", super::super::WNDPROC_HANDLE_STATIC),
                ("#32770", c"#32770", super::super::WNDPROC_HANDLE_DEF_DLG_W),
            ] {
                let w = crate::strings::to_wide_z(name_w);
                let mut wc: WNDCLASS = std::mem::zeroed();
                let atom_w = GetClassInfoW(std::ptr::null_mut(), w.as_ptr(), &mut wc);
                assert_ne!(atom_w, 0, "GetClassInfoW failed for {name_w}");
                assert_eq!(wc.wndproc, expected_proc);

                let mut wcex_w: WNDCLASSEX = std::mem::zeroed();
                wcex_w.cb_size = std::mem::size_of::<WNDCLASSEX>() as u32;
                let atom_ex_w = GetClassInfoExW(std::ptr::null_mut(), w.as_ptr(), &mut wcex_w);
                assert_eq!(atom_ex_w, atom_w);
                assert_eq!(wcex_w.wndproc, expected_proc);

                let mut wcex_a: WNDCLASSEX = std::mem::zeroed();
                wcex_a.cb_size = std::mem::size_of::<WNDCLASSEX>() as u32;
                let atom_ex_a = GetClassInfoExA(std::ptr::null_mut(), name_a.as_ptr() as LPCSTR, &mut wcex_a);
                assert_eq!(atom_ex_a, atom_w);
                assert_eq!(wcex_a.wndproc, expected_proc);
            }

            // Built-in system classes cannot be unregistered by an app.
            let edit_w = crate::strings::to_wide_z("EDIT");
            assert_eq!(UnregisterClassW(edit_w.as_ptr(), std::ptr::null_mut()), FALSE);

            // CreateWindowExW + CreateWindowExA succeed for EDIT, BUTTON, STATIC and atom lookups.
            let h_edit_w = CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("Wide edit").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                10,
                10,
                200,
                24,
                parent,
                10,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_edit_w.is_null());
            assert_eq!(GetWindowLongPtrW(h_edit_w, GWLP_WNDPROC) as usize, super::super::WNDPROC_HANDLE_EDIT);

            let h_btn_a = CreateWindowExA(
                0,
                c"BUTTON".as_ptr() as LPCSTR,
                c"&OK".as_ptr() as LPCSTR,
                ws::CHILD | ws::VISIBLE | BS_DEFPUSHBUTTON,
                10,
                40,
                80,
                28,
                parent,
                11,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_btn_a.is_null());
            assert_eq!(GetWindowLongPtrW(h_btn_a, GWLP_WNDPROC) as usize, super::super::WNDPROC_HANDLE_BUTTON);

            let h_static_a = CreateWindowExA(
                0,
                c"STATIC".as_ptr() as LPCSTR,
                c"Status label".as_ptr() as LPCSTR,
                ws::CHILD | ws::VISIBLE | SS_LEFT,
                10,
                74,
                200,
                20,
                parent,
                12,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_static_a.is_null());
            assert_eq!(GetWindowLongPtrW(h_static_a, GWLP_WNDPROC) as usize, super::super::WNDPROC_HANDLE_STATIC);

            // Creating via MAKEINTATOM returned by GetClassInfoW also works.
            let mut wc_btn: WNDCLASS = std::mem::zeroed();
            let btn_atom = GetClassInfoW(
                std::ptr::null_mut(),
                crate::strings::to_wide_z("Button").as_ptr(),
                &mut wc_btn,
            ) as u16;
            let h_btn_atom = CreateWindowExW(
                0,
                btn_atom as usize as LPCWSTR,
                crate::strings::to_wide_z("AtomBtn").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                100,
                40,
                80,
                28,
                parent,
                13,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_btn_atom.is_null());

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(IsWindow(h_edit_w), FALSE);
            assert_eq!(IsWindow(h_btn_a), FALSE);
            assert_eq!(IsWindow(h_static_a), FALSE);
            assert_eq!(IsWindow(h_btn_atom), FALSE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn button_text_click_check_radio_enable_and_show() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            COMMAND_LOG.lock().unwrap().clear();
            let (parent, parent_cls) = make_parent("TestButtonBehaviorParent");

            let btn_cls = crate::strings::to_wide_z("BUTTON");
            let h_ok = CreateWindowExW(
                0,
                btn_cls.as_ptr(),
                crate::strings::to_wide_z("&OK").as_ptr(),
                ws::CHILD | ws::VISIBLE | BS_PUSHBUTTON,
                10,
                10,
                90,
                30,
                parent,
                100,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_chk = CreateWindowExW(
                0,
                btn_cls.as_ptr(),
                crate::strings::to_wide_z("Send crash log").as_ptr(),
                ws::CHILD | ws::VISIBLE | BS_AUTOCHECKBOX,
                10,
                50,
                150,
                24,
                parent,
                101,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_tri = CreateWindowExW(
                0,
                btn_cls.as_ptr(),
                crate::strings::to_wide_z("Tri").as_ptr(),
                ws::CHILD | ws::VISIBLE | BS_AUTO3STATE,
                10,
                80,
                100,
                24,
                parent,
                102,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_r1 = CreateWindowExW(
                0,
                btn_cls.as_ptr(),
                crate::strings::to_wide_z("Opt 1").as_ptr(),
                ws::CHILD | ws::VISIBLE | ws::GROUP | BS_AUTORADIOBUTTON,
                10,
                110,
                100,
                24,
                parent,
                103,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_r2 = CreateWindowExW(
                0,
                btn_cls.as_ptr(),
                crate::strings::to_wide_z("Opt 2").as_ptr(),
                ws::CHILD | ws::VISIBLE | BS_AUTORADIOBUTTON,
                120,
                110,
                100,
                24,
                parent,
                104,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            // WM_SETTEXT / WM_GETTEXT (W & A)
            let mut buf_w = [0u16; 32];
            assert_eq!(GetWindowTextW(h_ok, buf_w.as_mut_ptr(), 32), 3);
            assert_eq!(String::from_utf16_lossy(&buf_w[..3]), "&OK");
            assert_eq!(SetWindowTextA(h_ok, c"Continue".as_ptr() as LPCSTR), TRUE);
            let mut buf_a = [0u8; 32];
            assert_eq!(GetWindowTextA(h_ok, buf_a.as_mut_ptr(), 32), 8);
            assert_eq!(&buf_a[..8], b"Continue");

            // BM_CLICK sends BN_CLICKED to parent.
            SendMessageW(h_ok, wm::BM_CLICK, 0, 0);
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(100, BN_CLICKED as u16, h_ok as usize)]);

            // Mouse press + release inside button client rect clicks it; outside cancels click.
            COMMAND_LOG.lock().unwrap().clear();
            SendMessageW(h_ok, wm::LBUTTONDOWN, 1, make_long(15, 10));
            assert_ne!(SendMessageW(h_ok, wm::BM_GETSTATE, 0, 0) as u32 & BST_PUSHED, 0);
            SendMessageW(h_ok, wm::LBUTTONUP, 0, make_long(15, 10));
            assert_eq!(SendMessageW(h_ok, wm::BM_GETSTATE, 0, 0) as u32 & BST_PUSHED, 0);
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(100, BN_CLICKED as u16, h_ok as usize)]);

            COMMAND_LOG.lock().unwrap().clear();
            SendMessageW(h_ok, wm::LBUTTONDOWN, 1, make_long(15, 10));
            SendMessageW(h_ok, wm::LBUTTONUP, 0, make_long(500, 500));
            assert!(COMMAND_LOG.lock().unwrap().is_empty(), "release outside must not fire BN_CLICKED");

            // EnableWindow(FALSE) blocks mouse and keyboard activation.
            assert_eq!(EnableWindow(h_ok, FALSE), FALSE);
            assert_eq!(IsWindowEnabled(h_ok), FALSE);
            SendMessageW(h_ok, wm::LBUTTONDOWN, 1, make_long(15, 10));
            SendMessageW(h_ok, wm::LBUTTONUP, 0, make_long(15, 10));
            SendMessageW(h_ok, wm::KEYUP, VK_SPACE, 0);
            assert!(COMMAND_LOG.lock().unwrap().is_empty());
            assert_eq!(EnableWindow(h_ok, TRUE), TRUE);
            assert_eq!(IsWindowEnabled(h_ok), TRUE);
            SendMessageW(h_ok, wm::KEYUP, VK_SPACE, 0);
            assert_eq!(COMMAND_LOG.lock().unwrap().len(), 1);

            // ShowWindow toggles visibility.
            assert_eq!(IsWindowVisible(h_ok), TRUE);
            assert_eq!(ShowWindow(h_ok, SW_HIDE), TRUE);
            assert_eq!(IsWindowVisible(h_ok), FALSE);
            assert_eq!(ShowWindow(h_ok, SW_SHOW), FALSE);
            assert_eq!(IsWindowVisible(h_ok), TRUE);

            // Auto-checkbox toggles 0 -> 1 -> 0; auto-3state cycles 0 -> 1 -> 2 -> 0.
            assert_eq!(SendMessageW(h_chk, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);
            SendMessageW(h_chk, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_chk, wm::BM_GETCHECK, 0, 0), BST_CHECKED as LRESULT);
            SendMessageW(h_chk, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_chk, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);

            SendMessageW(h_tri, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_tri, wm::BM_GETCHECK, 0, 0), BST_CHECKED as LRESULT);
            SendMessageW(h_tri, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_tri, wm::BM_GETCHECK, 0, 0), BST_INDETERMINATE as LRESULT);
            SendMessageW(h_tri, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_tri, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);

            // Auto-radio group mutual exclusion.
            SendMessageW(h_r1, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_r1, wm::BM_GETCHECK, 0, 0), BST_CHECKED as LRESULT);
            assert_eq!(SendMessageW(h_r2, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);
            SendMessageW(h_r2, wm::BM_CLICK, 0, 0);
            assert_eq!(SendMessageW(h_r1, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);
            assert_eq!(SendMessageW(h_r2, wm::BM_GETCHECK, 0, 0), BST_CHECKED as LRESULT);

            // Native AppKit callback bridge delivers BN_CLICKED on the owning thread's queue poll.
            COMMAND_LOG.lock().unwrap().clear();
            native_button_clicked(h_ok as usize, None);
            let mut msg = MSG::default();
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(100, BN_CLICKED as u16, h_ok as usize)]);

            // BM_SETSTYLE + BM_SETIMAGE / BM_GETIMAGE + WM_SETFONT / WM_GETFONT.
            SendMessageW(h_ok, wm::BM_SETSTYLE, BS_DEFPUSHBUTTON as usize, 1);
            assert_eq!(SendMessageW(h_ok, wm::GETDLGCODE, 0, 0), DLGC_BUTTON | DLGC_DEFPUSHBUTTON);
            assert_eq!(SendMessageW(h_ok, wm::BM_SETIMAGE, IMAGE_ICON as usize, 0x1234), 0);
            assert_eq!(SendMessageW(h_ok, wm::BM_GETIMAGE, IMAGE_ICON as usize, 0), 0x1234);
            SendMessageW(h_ok, wm::SETFONT, 0xABCD, 1);
            assert_eq!(SendMessageW(h_ok, wm::GETFONT, 0, 0), 0xABCD);

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn edit_text_selection_replacesel_multiline_readonly_and_native_bridge() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            COMMAND_LOG.lock().unwrap().clear();
            let (parent, parent_cls) = make_parent("TestEditBehaviorParent");

            let h_edit = CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("Line 1\r\nLine 2").as_ptr(),
                ws::CHILD | ws::VISIBLE | ws::VSCROLL | ES_MULTILINE,
                10,
                10,
                300,
                120,
                parent,
                200,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_edit.is_null());

            // Line queries on multiline text.
            assert_eq!(SendMessageW(h_edit, wm::EM_GETLINECOUNT, 0, 0), 2);
            assert_eq!(SendMessageW(h_edit, wm::EM_LINEINDEX, 0, 0), 0);
            assert_eq!(SendMessageW(h_edit, wm::EM_LINEINDEX, 1, 0), 8);
            assert_eq!(SendMessageW(h_edit, wm::EM_LINEINDEX, 2, 0), -1);
            assert_eq!(SendMessageW(h_edit, wm::EM_LINELENGTH, 0, 0), 6);
            assert_eq!(SendMessageW(h_edit, wm::EM_LINELENGTH, 8, 0), 6);

            let mut line_w = [0u16; 16];
            line_w[0] = 16;
            let got_w = SendMessageW(h_edit, wm::EM_GETLINE, 1, line_w.as_mut_ptr() as LPARAM);
            assert_eq!(got_w, 6);
            assert_eq!(String::from_utf16_lossy(&line_w[..6]), "Line 2");

            let mut line_a = [0u8; 16];
            (line_a.as_mut_ptr() as *mut u16).write_unaligned(16);
            let got_a = SendMessageA(h_edit, wm::EM_GETLINE, 0, line_a.as_mut_ptr() as LPARAM);
            assert_eq!(got_a, 6);
            assert_eq!(&line_a[..6], b"Line 1");

            // EM_SETSEL + EM_GETSEL + EM_REPLACESEL (both W and A) + modify flag.
            assert_eq!(SendMessageW(h_edit, wm::EM_GETMODIFY, 0, 0), 0);
            let end = GetWindowTextLengthW(h_edit) as usize;
            SendMessageW(h_edit, wm::EM_SETSEL, end, end as LPARAM);
            let mut s0 = 0u32;
            let mut s1 = 0u32;
            SendMessageW(h_edit, wm::EM_GETSEL, &mut s0 as *mut u32 as usize, &mut s1 as *mut u32 as LPARAM);
            assert_eq!((s0 as usize, s1 as usize), (end, end));

            COMMAND_LOG.lock().unwrap().clear();
            SendMessageA(h_edit, wm::EM_REPLACESEL, 0, c"\r\nLine 3".as_ptr() as LPARAM);
            assert_eq!(SendMessageW(h_edit, wm::EM_GETMODIFY, 0, 0), 1);
            assert_eq!(SendMessageW(h_edit, wm::EM_GETLINECOUNT, 0, 0), 3);
            assert!(COMMAND_LOG.lock().unwrap().contains(&(200, EN_UPDATE as u16, h_edit as usize)));
            assert!(COMMAND_LOG.lock().unwrap().contains(&(200, EN_CHANGE as u16, h_edit as usize)));

            // WM_CHAR typing + backspace, then EM_SETREADONLY blocks edits.
            SendMessageW(h_edit, wm::CHAR, b'!' as usize, 0);
            let mut txt = [0u16; 64];
            let len = GetWindowTextW(h_edit, txt.as_mut_ptr(), 64) as usize;
            assert_eq!(String::from_utf16_lossy(&txt[..len]), "Line 1\r\nLine 2\r\nLine 3!");
            SendMessageW(h_edit, wm::CHAR, 0x08, 0); // Backspace
            let len2 = GetWindowTextW(h_edit, txt.as_mut_ptr(), 64) as usize;
            assert_eq!(String::from_utf16_lossy(&txt[..len2]), "Line 1\r\nLine 2\r\nLine 3");

            assert_eq!(SendMessageW(h_edit, wm::EM_SETREADONLY, 1, 0), 1);
            SendMessageW(h_edit, wm::CHAR, b'X' as usize, 0);
            let len3 = GetWindowTextW(h_edit, txt.as_mut_ptr(), 64) as usize;
            assert_eq!(len3, len2, "read-only EDIT must ignore WM_CHAR");
            assert_eq!(SendMessageW(h_edit, wm::EM_SETREADONLY, 0, 0), 1);

            // Native AppKit text-change bridge updates Win32 text, selection, and sends EN_CHANGE on queue poll.
            COMMAND_LOG.lock().unwrap().clear();
            native_edit_text_changed(h_edit as usize, "Typed via AppKit".to_string(), 16, 16);
            let mut msg = MSG::default();
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let len4 = GetWindowTextW(h_edit, txt.as_mut_ptr(), 64) as usize;
            assert_eq!(String::from_utf16_lossy(&txt[..len4]), "Typed via AppKit");
            assert!(COMMAND_LOG.lock().unwrap().contains(&(200, EN_CHANGE as u16, h_edit as usize)));
            let mut ns0 = 0u32;
            let mut ns1 = 0u32;
            SendMessageW(h_edit, wm::EM_GETSEL, &mut ns0 as *mut u32 as usize, &mut ns1 as *mut u32 as LPARAM);
            assert_eq!((ns0, ns1), (16, 16));

            // WM_SETTEXT clears modified flag.
            assert_eq!(SetWindowTextW(h_edit, crate::strings::to_wide_z("Reset").as_ptr()), TRUE);
            assert_eq!(SendMessageW(h_edit, wm::EM_GETMODIFY, 0, 0), 0);

            // Limit text & password char.
            SendMessageW(h_edit, wm::EM_LIMITTEXT, 64, 0);
            assert_eq!(SendMessageW(h_edit, wm::EM_GETLIMITTEXT, 0, 0), 64);
            SendMessageW(h_edit, wm::EM_SETPASSWORDCHAR, b'*' as usize, 0);
            assert_eq!(SendMessageW(h_edit, wm::EM_GETPASSWORDCHAR, 0, 0), b'*' as LRESULT);

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn static_text_icon_notify_and_mnemonic_stripping() {
        assert_eq!(strip_win32_mnemonic("&OK"), "OK");
        assert_eq!(strip_win32_mnemonic("Save && &Exit"), "Save & Exit");
        assert_eq!(strip_win32_mnemonic("Plain"), "Plain");

        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            COMMAND_LOG.lock().unwrap().clear();
            let (parent, parent_cls) = make_parent("TestStaticBehaviorParent");

            let h_lbl = CreateWindowExW(
                0,
                crate::strings::to_wide_z("STATIC").as_ptr(),
                crate::strings::to_wide_z("Initial label").as_ptr(),
                ws::CHILD | ws::VISIBLE | SS_LEFT | SS_NOTIFY,
                10,
                10,
                250,
                24,
                parent,
                300,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_ico = CreateWindowExW(
                0,
                crate::strings::to_wide_z("STATIC").as_ptr(),
                std::ptr::null(),
                ws::CHILD | ws::VISIBLE | SS_ICON,
                10,
                40,
                32,
                32,
                parent,
                301,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_lbl.is_null() && !h_ico.is_null());
            assert_eq!(SendMessageW(h_lbl, wm::GETDLGCODE, 0, 0), DLGC_STATIC);

            // STM_SETICON / STM_GETICON + STM_SETIMAGE / STM_GETIMAGE.
            let hicon_err = cursor::LoadIconW(std::ptr::null_mut(), 32513 as LPCWSTR); // IDI_ERROR
            assert_ne!(hicon_err, 0);
            assert_eq!(SendMessageW(h_ico, wm::STM_SETICON, hicon_err, 0), 0);
            assert_eq!(SendMessageW(h_ico, wm::STM_GETICON, 0, 0) as usize, hicon_err);
            assert_eq!(SendMessageW(h_ico, wm::STM_GETIMAGE, IMAGE_ICON as usize, 0) as usize, hicon_err);

            // SS_NOTIFY click delivers STN_CLICKED when enabled, suppressed when disabled.
            SendMessageW(h_lbl, wm::LBUTTONDOWN, 1, make_long(5, 5));
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(300, STN_CLICKED as u16, h_lbl as usize)]);
            EnableWindow(h_lbl, FALSE);
            COMMAND_LOG.lock().unwrap().clear();
            SendMessageW(h_lbl, wm::LBUTTONDOWN, 1, make_long(5, 5));
            assert!(COMMAND_LOG.lock().unwrap().is_empty());

            // Native static click bridge when re-enabled.
            EnableWindow(h_lbl, TRUE);
            native_static_clicked(h_lbl as usize);
            let mut msg = MSG::default();
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(300, STN_CLICKED as u16, h_lbl as usize)]);

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn error_dialog_fixture_createwindowex_renders_controls_and_handles_dismiss() {
        // Reproduces a standard Win32 ErrorDialog created directly via CreateWindowEx
        // with STATIC, EDIT, and BUTTON child controls (previously failing with
        // `CreateWindowEx: class "EDIT" not registered` / `"BUTTON"` not registered).
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");
        std::thread::spawn(|| unsafe {
            const ID_STATIC_HEADER: usize = 1000;
            const ID_EDIT_DETAILS: usize = 1001;
            const ID_BTN_COPY: usize = 1002;
            const ID_BTN_CLOSE: usize = 1; // IDOK

            static COPIED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            static CLOSED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            COPIED.store(false, std::sync::atomic::Ordering::SeqCst);
            CLOSED.store(false, std::sync::atomic::Ordering::SeqCst);

            unsafe extern "win64" fn error_dialog_proc(
                hwnd: HWND,
                msg: u32,
                wparam: WPARAM,
                lparam: LPARAM,
            ) -> LRESULT {
                if msg == wm::COMMAND {
                    let id = (wparam & 0xFFFF) as usize;
                    let code = (wparam >> 16) & 0xFFFF;
                    if code == BN_CLICKED {
                        if id == ID_BTN_COPY {
                            COPIED.store(true, std::sync::atomic::Ordering::SeqCst);
                            return 0;
                        }
                        if id == ID_BTN_CLOSE {
                            CLOSED.store(true, std::sync::atomic::Ordering::SeqCst);
                            DestroyWindow(hwnd);
                            return 0;
                        }
                    }
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }

            let dlg_cls = crate::strings::to_wide_z("GenericErrorDialogFixture");
            let wc = WNDCLASSEX {
                cb_size: std::mem::size_of::<WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::super::register_typed_host_wndproc(error_dialog_proc),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: dlg_cls.as_ptr() as usize,
                icon_sm: 0,
            };
            assert_ne!(RegisterClassExW(&wc), 0);

            let hdlg = CreateWindowExW(
                0,
                dlg_cls.as_ptr(),
                crate::strings::to_wide_z("Application Error").as_ptr(),
                ws::CAPTION | ws::SYSMENU,
                100,
                100,
                480,
                320,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hdlg.is_null());

            let h_header = CreateWindowExW(
                0,
                crate::strings::to_wide_z("STATIC").as_ptr(),
                crate::strings::to_wide_z("A fatal error has occurred.").as_ptr(),
                ws::CHILD | ws::VISIBLE | SS_LEFT,
                16,
                12,
                448,
                24,
                hdlg,
                ID_STATIC_HEADER,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_details = CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("Error code: 0x80070057").as_ptr(),
                ws::CHILD | ws::VISIBLE | ws::VSCROLL | ES_MULTILINE | ES_READONLY,
                16,
                44,
                448,
                190,
                hdlg,
                ID_EDIT_DETAILS,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_copy = CreateWindowExA(
                0,
                c"BUTTON".as_ptr() as LPCSTR,
                c"&Copy Details".as_ptr() as LPCSTR,
                ws::CHILD | ws::VISIBLE | ws::TABSTOP | BS_PUSHBUTTON,
                260,
                248,
                100,
                28,
                hdlg,
                ID_BTN_COPY,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_close = CreateWindowExW(
                0,
                crate::strings::to_wide_z("BUTTON").as_ptr(),
                crate::strings::to_wide_z("&Close").as_ptr(),
                ws::CHILD | ws::VISIBLE | ws::TABSTOP | BS_DEFPUSHBUTTON,
                372,
                248,
                92,
                28,
                hdlg,
                ID_BTN_CLOSE,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_header.is_null(), "STATIC control must be created");
            assert!(!h_details.is_null(), "EDIT control must be created");
            assert!(!h_copy.is_null(), "ANSI BUTTON control must be created");
            assert!(!h_close.is_null(), "Unicode BUTTON control must be created");

            // Append additional diagnostic line to the read-only EDIT via EM_SETSEL + EM_REPLACESEL.
            let cur_len = GetWindowTextLengthW(h_details) as usize;
            SendMessageW(h_details, wm::EM_SETSEL, cur_len, cur_len as LPARAM);
            let extra = crate::strings::to_wide_z("\r\nModule: renderer.dll");
            SendMessageW(h_details, wm::EM_REPLACESEL, 0, extra.as_ptr() as LPARAM);

            ShowWindow(hdlg, SW_SHOW);
            assert_eq!(IsWindowVisible(hdlg), TRUE);
            assert_eq!(IsWindowVisible(h_details), TRUE);
            assert_eq!(IsWindowVisible(h_close), TRUE);

            // Verify GetDlgItem + GetDlgItemTextA work seamlessly on CreateWindowEx-created controls.
            assert_eq!(GetDlgItem(hdlg, ID_EDIT_DETAILS as i32), h_details);
            let mut details_a = [0u8; 128];
            let n = GetDlgItemTextA(hdlg, ID_EDIT_DETAILS as i32, details_a.as_mut_ptr(), 128) as usize;
            assert_eq!(&details_a[..n], b"Error code: 0x80070057\r\nModule: renderer.dll");

            // Click Copy, then click Close to dismiss the ErrorDialog.
            SendMessageW(h_copy, wm::BM_CLICK, 0, 0);
            assert!(COPIED.load(std::sync::atomic::Ordering::SeqCst));
            assert!(!CLOSED.load(std::sync::atomic::Ordering::SeqCst));
            assert_eq!(IsWindow(hdlg), TRUE);

            SendMessageW(h_close, wm::BM_CLICK, 0, 0);
            assert!(CLOSED.load(std::sync::atomic::Ordering::SeqCst));
            assert_eq!(IsWindow(hdlg), FALSE);
            assert_eq!(IsWindow(h_details), FALSE);
            assert_eq!(IsWindow(h_copy), FALSE);
            assert_eq!(IsWindow(h_close), FALSE);

            assert_eq!(UnregisterClassW(dlg_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn native_edit_selection_sync_and_limit_reconciliation() {
        // Surrogate-pair-safe UTF-16 truncation.
        let mut emoji_units: Vec<u16> = "a😀b".encode_utf16().collect(); // ['a', 0xD83D, 0xDE00, 'b']
        assert!(truncate_utf16_at_limit(&mut emoji_units, 2));
        assert_eq!(String::from_utf16_lossy(&emoji_units), "a");

        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            COMMAND_LOG.lock().unwrap().clear();
            let (parent, parent_cls) = make_parent("TestEditSelLimitParent");

            let h_edit = CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                10,
                10,
                240,
                24,
                parent,
                250,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_edit.is_null());

            // 1. User types "abc" in AppKit (caret moves to offset 3); subsequent EM_REPLACESEL
            //    must append at offset 3 ("abcdef"), not insert at offset 0 ("defabc").
            native_edit_text_changed(h_edit as usize, "abc".to_string(), 3, 3);
            let mut msg = MSG::default();
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let (mut s0, mut s1) = (0u32, 0u32);
            SendMessageW(h_edit, wm::EM_GETSEL, &mut s0 as *mut u32 as usize, &mut s1 as *mut u32 as LPARAM);
            assert_eq!((s0, s1), (3, 3));

            let suffix = crate::strings::to_wide_z("def");
            SendMessageW(h_edit, wm::EM_REPLACESEL, 0, suffix.as_ptr() as LPARAM);
            let mut buf = [0u16; 32];
            let n = GetWindowTextW(h_edit, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n]), "abcdef");
            SendMessageW(h_edit, wm::EM_GETSEL, &mut s0 as *mut u32 as usize, &mut s1 as *mut u32 as LPARAM);
            assert_eq!((s0, s1), (6, 6));

            // 2. Native caret/selection move without text edit syncs to EM_GETSEL & EM_REPLACESEL.
            native_edit_selection_changed(h_edit as usize, 2, 5);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            SendMessageW(h_edit, wm::EM_GETSEL, &mut s0 as *mut u32 as usize, &mut s1 as *mut u32 as LPARAM);
            assert_eq!((s0, s1), (2, 5));
            let repl = crate::strings::to_wide_z("X");
            SendMessageW(h_edit, wm::EM_REPLACESEL, 0, repl.as_ptr() as LPARAM);
            let n2 = GetWindowTextW(h_edit, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n2]), "abXf");

            // 3. EM_LIMITTEXT truncates overlong native edits and clamps selection cleanly.
            SendMessageW(h_edit, wm::EM_LIMITTEXT, 5, 0);
            native_edit_text_changed(h_edit as usize, "123456789".to_string(), 9, 9);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let n3 = GetWindowTextW(h_edit, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n3]), "12345");
            SendMessageW(h_edit, wm::EM_GETSEL, &mut s0 as *mut u32 as usize, &mut s1 as *mut u32 as LPARAM);
            assert_eq!((s0, s1), (5, 5));

            // 4. Native focus bridge updates Win32 GetFocus().
            native_control_focused(h_edit as usize);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            assert_eq!(input::GetFocus(), h_edit);

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn native_edit_preserves_suffix_at_limit_and_enforces_char_styles() {
        let u = |s: &str| -> Vec<u16> { s.encode_utf16().collect() };

        // 1. At limit=5 with "abcde", typing 'X' at index 0 must NOT delete suffix 'e'.
        let r1 = apply_native_edit_constraints(&u("abcde"), &u("Xabcde"), 1, 1, 0, 5);
        assert_eq!(String::from_utf16_lossy(&r1.units), "abcde");
        assert_eq!((r1.sel_start, r1.sel_end), (0, 0));
        assert!(!r1.text_changed);
        assert!(r1.reconcile_native);

        // 2. With limit=5 and "abc", pasting "XYZ" at index 1 keeps suffix "bc" -> "aXYbc".
        let r2 = apply_native_edit_constraints(&u("abc"), &u("aXYZbc"), 4, 4, 0, 5);
        assert_eq!(String::from_utf16_lossy(&r2.units), "aXYbc");
        assert_eq!((r2.sel_start, r2.sel_end), (3, 3));
        assert!(r2.text_changed);
        assert!(r2.reconcile_native);

        // 3. Replacing "bc" inside "abcde" with "WXYZ" at limit=5 -> "aWXde" (preserves "de").
        let r3 = apply_native_edit_constraints(&u("abcde"), &u("aWXYZde"), 5, 5, 0, 5);
        assert_eq!(String::from_utf16_lossy(&r3.units), "aWXde");
        assert_eq!((r3.sel_start, r3.sel_end), (3, 3));

        // 4. Live EDIT tests: ES_NUMBER, ES_UPPERCASE, ES_LOWERCASE, and range-preserving limit.
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let (parent, parent_cls) = make_parent("TestEditStyleConstraintsParent");
            let edit_cls = crate::strings::to_wide_z("EDIT");

            let h_lim = CreateWindowExW(
                0,
                edit_cls.as_ptr(),
                crate::strings::to_wide_z("abcde").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                10,
                10,
                200,
                24,
                parent,
                260,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_num = CreateWindowExW(
                0,
                edit_cls.as_ptr(),
                crate::strings::to_wide_z("").as_ptr(),
                ws::CHILD | ws::VISIBLE | ES_NUMBER,
                10,
                40,
                200,
                24,
                parent,
                261,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_up = CreateWindowExW(
                0,
                edit_cls.as_ptr(),
                crate::strings::to_wide_z("").as_ptr(),
                ws::CHILD | ws::VISIBLE | ES_UPPERCASE,
                10,
                70,
                200,
                24,
                parent,
                262,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let h_lo = CreateWindowExW(
                0,
                edit_cls.as_ptr(),
                crate::strings::to_wide_z("").as_ptr(),
                ws::CHILD | ws::VISIBLE | ES_LOWERCASE,
                10,
                100,
                200,
                24,
                parent,
                263,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            let mut msg = MSG::default();
            let mut buf = [0u16; 32];

            // Rejecting excess insertion at start preserves "abcde" and suppresses spurious EN_CHANGE.
            SendMessageW(h_lim, wm::EM_LIMITTEXT, 5, 0);
            COMMAND_LOG.lock().unwrap().clear();
            native_edit_text_changed(h_lim as usize, "Xabcde".to_string(), 1, 1);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let n = GetWindowTextW(h_lim, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n]), "abcde");
            assert!(COMMAND_LOG.lock().unwrap().is_empty(), "rejected excess insert must not emit EN_CHANGE");

            // ES_NUMBER strips non-digits and rejects pure-letter typing.
            native_edit_text_changed(h_num as usize, "12a3b".to_string(), 5, 5);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let n = GetWindowTextW(h_num, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n]), "123");
            COMMAND_LOG.lock().unwrap().clear();
            native_edit_text_changed(h_num as usize, "12x3".to_string(), 3, 3);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let n = GetWindowTextW(h_num, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n]), "123");
            assert!(COMMAND_LOG.lock().unwrap().is_empty());

            // ES_UPPERCASE / ES_LOWERCASE convert native typed text.
            native_edit_text_changed(h_up as usize, "Hello".to_string(), 5, 5);
            native_edit_text_changed(h_lo as usize, "HeLLo".to_string(), 5, 5);
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            let n_up = GetWindowTextW(h_up, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n_up]), "HELLO");
            let n_lo = GetWindowTextW(h_lo, buf.as_mut_ptr(), 32) as usize;
            assert_eq!(String::from_utf16_lossy(&buf[..n_lo]), "hello");

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn nested_controls_reflect_child_container_visibility() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let (parent, parent_cls) = make_parent("TestNestedContainerVisParent");

            // Create an intermediate child container (e.g. a dialog tab page) with nested controls.
            let page = CreateWindowExW(
                0,
                parent_cls.as_ptr(),
                std::ptr::null(),
                ws::CHILD | ws::VISIBLE,
                10,
                10,
                300,
                200,
                parent,
                400,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let nested_edit = CreateWindowExW(
                0,
                crate::strings::to_wide_z("EDIT").as_ptr(),
                crate::strings::to_wide_z("Inside page").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                8,
                8,
                200,
                24,
                page,
                401,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let nested_btn = CreateWindowExW(
                0,
                crate::strings::to_wide_z("BUTTON").as_ptr(),
                crate::strings::to_wide_z("Apply").as_ptr(),
                ws::CHILD | ws::VISIBLE,
                8,
                40,
                80,
                28,
                page,
                402,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!page.is_null() && !nested_edit.is_null() && !nested_btn.is_null());

            assert!(window::effective_child_visible(nested_edit as usize));
            assert!(window::effective_child_visible(nested_btn as usize));

            // Hiding the child container must make descendant controls effectively hidden.
            assert_eq!(ShowWindow(page, SW_HIDE), TRUE);
            assert!(!window::effective_child_visible(nested_edit as usize));
            assert!(!window::effective_child_visible(nested_btn as usize));
            assert_eq!(IsWindowVisible(nested_edit), FALSE);
            assert_eq!(IsWindowVisible(nested_btn), FALSE);

            // Showing the child container restores descendant effective visibility.
            assert_eq!(ShowWindow(page, SW_SHOW), FALSE);
            assert!(window::effective_child_visible(nested_edit as usize));
            assert!(window::effective_child_visible(nested_btn as usize));
            assert_eq!(IsWindowVisible(nested_edit), TRUE);

            // Toggling WS_VISIBLE via SetWindowLongW on the container also propagates.
            let cur_style = super::super::window::GetWindowLongW(page, super::super::window::GWL_STYLE) as u32;
            super::super::window::SetWindowLongW(page, super::super::window::GWL_STYLE, (cur_style & !ws::VISIBLE) as i32);
            assert!(!window::effective_child_visible(nested_edit as usize));
            super::super::window::SetWindowLongW(page, super::super::window::GWL_STYLE, (cur_style | ws::VISIBLE) as i32);
            assert!(window::effective_child_visible(nested_edit as usize));

            // Moving the child container updates descendant root-relative rects.
            assert_eq!(
                window::root_top_level_and_rect(nested_edit as usize),
                Some((parent as usize, RECT::new(18, 18, 218, 42)))
            );
            assert_eq!(super::super::window::MoveWindow(page, 50, 70, 300, 200, TRUE), TRUE);
            assert_eq!(
                window::root_top_level_and_rect(nested_edit as usize),
                Some((parent as usize, RECT::new(58, 78, 258, 102)))
            );

            // Manual BS_CHECKBOX / BS_3STATE / BS_RADIOBUTTON native clicks keep
            // application-controlled state unchanged when parent ignores BN_CLICKED.
            COMMAND_LOG.lock().unwrap().clear();
            let h_manual_chk = CreateWindowExW(
                0,
                crate::strings::to_wide_z("BUTTON").as_ptr(),
                crate::strings::to_wide_z("Manual").as_ptr(),
                ws::CHILD | ws::VISIBLE | BS_CHECKBOX,
                8,
                80,
                120,
                24,
                parent,
                403,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_manual_chk.is_null());
            assert_eq!(SendMessageW(h_manual_chk, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);
            native_button_clicked(h_manual_chk as usize, Some(BST_CHECKED));
            let mut msg = MSG::default();
            let _ = PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            assert_eq!(SendMessageW(h_manual_chk, wm::BM_GETCHECK, 0, 0), BST_UNCHECKED as LRESULT);
            assert_eq!(*COMMAND_LOG.lock().unwrap(), vec![(403, BN_CLICKED as u16, h_manual_chk as usize)]);

            assert_eq!(DestroyWindow(parent), TRUE);
            assert_eq!(UnregisterClassW(parent_cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }
}
