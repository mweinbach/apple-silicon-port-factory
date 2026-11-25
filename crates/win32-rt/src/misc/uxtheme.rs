//! uxtheme.dll / comctl32.dll: Desktop Visual Styles (honest unthemed fallback),
//! common controls initialization, and `_TrackMouseEvent` per-HWND state tracking.

use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (17, "InitCommonControls"),
    (345, "TaskDialogIndirect"),
    (410, "SetWindowSubclass"),
    (411, "GetWindowSubclass"),
    (412, "RemoveWindowSubclass"),
    (413, "DefSubclassProc"),
];

const E_HANDLE: HRESULT = 0x8007_0006u32 as i32;

// TRACKMOUSEEVENT.dwFlags
const TME_HOVER: u32 = 0x0000_0001;
const TME_LEAVE: u32 = 0x0000_0002;
const TME_NONCLIENT: u32 = 0x0000_0010;
const TME_QUERY: u32 = 0x4000_0000;
const TME_CANCEL: u32 = 0x8000_0000;
const TME_VALID_TRACK_MASK: u32 = TME_HOVER | TME_LEAVE | TME_NONCLIENT;
const HOVER_DEFAULT: u32 = 0xFFFF_FFFF;
const DEFAULT_HOVER_MS: u32 = 400;

#[derive(Clone, Copy, Debug, Default)]
struct MouseTrackEntry {
    flags: u32,
    hover_time: u32,
}

#[derive(Default)]
struct MouseTrackState {
    per_hwnd: HashMap<usize, MouseTrackEntry>,
    last_hwnd: usize,
}

fn mouse_track() -> &'static Mutex<MouseTrackState> {
    static S: OnceLock<Mutex<MouseTrackState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(MouseTrackState::default()))
}

pub unsafe extern "win64" fn IsAppThemed() -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn IsThemeActive() -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn OpenThemeData(_hwnd: HWND, _class_list: LPCWSTR) -> usize {
    0
}

pub unsafe extern "win64" fn OpenThemeDataEx(_hwnd: HWND, _class_list: LPCWSTR, _flags: u32) -> usize {
    0
}

pub unsafe extern "win64" fn OpenThemeDataForDpi(_hwnd: HWND, _class_list: LPCWSTR, _dpi: u32) -> usize {
    0
}

pub unsafe extern "win64" fn CloseThemeData(htheme: usize) -> HRESULT {
    if htheme == 0 {
        E_HANDLE
    } else {
        S_OK
    }
}

pub unsafe extern "win64" fn SetWindowTheme(_hwnd: HWND, _sub_app: LPCWSTR, _sub_id: LPCWSTR) -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn GetThemeColor(
    htheme: usize,
    _part: i32,
    _state: i32,
    _prop: i32,
    color: *mut u32,
) -> HRESULT {
    if htheme == 0 {
        return E_HANDLE;
    }
    if color.is_null() {
        return E_POINTER;
    }
    color.write_unaligned(0x0020_2020);
    S_OK
}

pub unsafe extern "win64" fn GetThemeSysColor(_htheme: usize, color_id: i32) -> u32 {
    crate::user32::misc::GetSysColor(color_id)
}

pub unsafe extern "win64" fn DrawThemeBackground(
    htheme: usize,
    hdc: usize,
    _part: i32,
    _state: i32,
    rect: *const u8,
    _clip: *const u8,
) -> HRESULT {
    // Visual styles are disabled (`IsThemeActive() == FALSE`, `OpenThemeData*`
    // returns NULL), so no open HTHEME handles exist in the process handle table.
    // On Windows `uxtheme!DrawThemeBackground` validates `hTheme` first via
    // `ValidateHandle` and returns `E_HANDLE` for any handle that is not an open
    // theme handle, matching Windows Classic / unthemed desktop behavior.
    let _ = (htheme, hdc, rect);
    E_HANDLE
}

pub unsafe extern "win64" fn InitCommonControls() {}

pub unsafe extern "win64" fn InitCommonControlsEx(icce: *const u8) -> BOOL {
    if icce.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb_size = (icce as *const u32).read_unaligned();
    if cb_size < 8 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    TRUE
}

/// `TRACKMOUSEEVENT` (x64, 24 bytes): `cbSize` @0, `dwFlags` @4, `hwndTrack` @8, `dwHoverTime` @16.
pub unsafe extern "win64" fn _TrackMouseEvent(tme: *mut u8) -> BOOL {
    if tme.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb_size = (tme as *const u32).read_unaligned();
    if cb_size < 24 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let flags = (tme.add(4) as *const u32).read_unaligned();
    let hwnd = (tme.add(8) as *const usize).read_unaligned();
    let hover_time = (tme.add(16) as *const u32).read_unaligned();

    let mut st = mouse_track().lock().unwrap();
    if (flags & TME_QUERY) != 0 {
        let query_hwnd = if hwnd != 0 { hwnd } else { st.last_hwnd };
        if let Some(&entry) = st.per_hwnd.get(&query_hwnd) {
            (tme.add(4) as *mut u32).write_unaligned(entry.flags);
            (tme.add(8) as *mut usize).write_unaligned(query_hwnd);
            (tme.add(16) as *mut u32).write_unaligned(entry.hover_time);
        } else {
            (tme.add(4) as *mut u32).write_unaligned(0);
            (tme.add(16) as *mut u32).write_unaligned(0);
        }
        return TRUE;
    }

    if hwnd == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }

    let track_bits = flags & TME_VALID_TRACK_MASK;
    if (flags & TME_CANCEL) != 0 {
        if let Some(entry) = st.per_hwnd.get_mut(&hwnd) {
            entry.flags &= !track_bits;
            if entry.flags == 0 {
                st.per_hwnd.remove(&hwnd);
                if st.last_hwnd == hwnd {
                    st.last_hwnd = 0;
                }
            }
        }
        return TRUE;
    }

    let eff_hover = if hover_time == HOVER_DEFAULT { DEFAULT_HOVER_MS } else { hover_time };
    let entry = st.per_hwnd.entry(hwnd).or_default();
    entry.flags |= track_bits;
    if (track_bits & TME_HOVER) != 0 {
        entry.hover_time = eff_hover;
    }
    st.last_hwnd = hwnd;
    TRUE
}

// `TASKDIALOGCONFIG` and `TASKDIALOG_BUTTON` use `#include <pshpack1.h>` in commctrl.h.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct TaskDialogButton {
    button_id: i32,
    button_text: LPCWSTR,
}
const _: () = assert!(std::mem::size_of::<TaskDialogButton>() == 12);

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct TaskDialogConfig {
    cb_size: u32,
    hwnd_parent: HWND,
    h_instance: HINSTANCE,
    dw_flags: u32,
    dw_common_buttons: u32,
    psz_window_title: LPCWSTR,
    main_icon: usize,
    psz_main_instruction: LPCWSTR,
    psz_content: LPCWSTR,
    c_buttons: u32,
    p_buttons: *const TaskDialogButton,
    n_default_button: i32,
    c_radio_buttons: u32,
    p_radio_buttons: *const TaskDialogButton,
    n_default_radio_button: i32,
    psz_verification_text: LPCWSTR,
    psz_expanded_information: LPCWSTR,
    psz_expanded_control_text: LPCWSTR,
    psz_collapsed_control_text: LPCWSTR,
    footer_icon: usize,
    psz_footer: LPCWSTR,
    pf_callback: usize,
    lp_callback_data: isize,
    cx_width: u32,
}
const _: () = assert!(std::mem::size_of::<TaskDialogConfig>() == 160);

const TDCBF_OK_BUTTON: u32 = 0x0001;
const TDCBF_YES_BUTTON: u32 = 0x0002;
const TDCBF_NO_BUTTON: u32 = 0x0004;
const TDCBF_CANCEL_BUTTON: u32 = 0x0008;
const TDCBF_RETRY_BUTTON: u32 = 0x0010;
const TDCBF_CLOSE_BUTTON: u32 = 0x0020;

const TDF_VERIFICATION_FLAG_CHECKED: u32 = 0x0100;
const TDF_CALLBACK_TIMER: u32 = 0x0800;
const TDF_NO_DEFAULT_RADIO_BUTTON: u32 = 0x4000;

const TDN_CREATED: u32 = 0;
const TDN_BUTTON_CLICKED: u32 = 2;
const TDN_TIMER: u32 = 4;
const TDN_DESTROYED: u32 = 5;
const TDN_RADIO_BUTTON_CLICKED: u32 = 6;
const TDN_DIALOG_CONSTRUCTED: u32 = 7;
const TDN_VERIFICATION_CLICKED: u32 = 8;

/// Standard Windows TaskDialog `TDF_CALLBACK_TIMER` notification period (~200 ms).
const TASK_DIALOG_TIMER_INTERVAL: std::time::Duration = std::time::Duration::from_millis(200);

const TDM_CLICK_BUTTON: u32 = crate::user32::wm::USER + 102;
const TDM_SET_ELEMENT_TEXT: u32 = crate::user32::wm::USER + 108;
const TDM_CLICK_RADIO_BUTTON: u32 = crate::user32::wm::USER + 110;
const TDM_ENABLE_BUTTON: u32 = crate::user32::wm::USER + 111;
const TDM_ENABLE_RADIO_BUTTON: u32 = crate::user32::wm::USER + 112;
const TDM_CLICK_VERIFICATION: u32 = crate::user32::wm::USER + 113;
const TDM_UPDATE_ELEMENT_TEXT: u32 = crate::user32::wm::USER + 114;

const TDE_CONTENT: usize = 0;
const TDE_EXPANDED_INFORMATION: usize = 1;
const TDE_FOOTER: usize = 2;
const TDE_MAIN_INSTRUCTION: usize = 3;

const S_FALSE_HR: HRESULT = 1;
const HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION: HRESULT = 0x8007_05B3u32 as i32;

static NEXT_TASK_DIALOG_SESSION: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(1);

struct ActiveTaskDialog {
    session_id: u64,
    modal_open: bool,
    hinst: usize,
    pf_callback: usize,
    lp_callback_data: isize,
    caption: String,
    main_inst: String,
    content: String,
    expanded: String,
    footer: String,
    buttons: Vec<(i32, String)>,
    button_enabled: Vec<bool>,
    radios: Vec<(i32, String)>,
    radio_enabled: Vec<bool>,
    selected_radio_idx: Option<usize>,
    has_verification: bool,
    verified: bool,
    pending_click_button: Option<i32>,
}

impl ActiveTaskDialog {
    fn compose_body(&self) -> String {
        let mut body_parts: Vec<&str> = Vec::new();
        for part in [&self.main_inst, &self.content, &self.expanded, &self.footer] {
            if !part.is_empty() {
                body_parts.push(part.as_str());
            }
        }
        body_parts.join("\n\n")
    }

    fn live_update(&self, click_button_idx: Option<usize>) -> Option<(u64, crate::macos::TaskDialogLiveUpdate)> {
        self.modal_open.then(|| {
            (
                self.session_id,
                crate::macos::TaskDialogLiveUpdate {
                    caption: self.caption.clone(),
                    text: self.compose_body(),
                    button_enabled: self.button_enabled.clone(),
                    radio_enabled: self.radio_enabled.clone(),
                    selected_radio_idx: self.selected_radio_idx,
                    verification_checked: self.has_verification.then_some(self.verified),
                    click_button_idx,
                },
            )
        })
    }
}

fn active_task_dialogs() -> &'static Mutex<HashMap<usize, ActiveTaskDialog>> {
    static T: OnceLock<Mutex<HashMap<usize, ActiveTaskDialog>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

fn task_dialog_wndproc_va() -> usize {
    crate::user32::register_typed_host_wndproc(task_dialog_wndproc)
}

unsafe extern "win64" fn task_dialog_wndproc(
    hwnd: HWND,
    u_msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    let h = hwnd as usize;
    match u_msg {
        crate::user32::wm::SETTEXT => {
            let res = crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param);
            let new_title = if l_param == 0 {
                String::new()
            } else {
                crate::strings::from_wide(l_param as LPCWSTR)
            };
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    dlg.caption = new_title;
                    dlg.live_update(None)
                })
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            res
        }
        TDM_SET_ELEMENT_TEXT | TDM_UPDATE_ELEMENT_TEXT => {
            let hinst = active_task_dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(&h)
                .map(|d| d.hinst as HINSTANCE)
                .unwrap_or(std::ptr::null_mut());
            let new_text = resolve_task_dialog_str(hinst, l_param as LPCWSTR);
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    match w_param {
                        TDE_CONTENT => dlg.content = new_text,
                        TDE_EXPANDED_INFORMATION => dlg.expanded = new_text,
                        TDE_FOOTER => dlg.footer = new_text,
                        TDE_MAIN_INSTRUCTION => dlg.main_inst = new_text,
                        _ => {}
                    }
                    dlg.live_update(None)
                })
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            0
        }
        TDM_CLICK_BUTTON => {
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    let bid = w_param as i32;
                    dlg.pending_click_button = Some(bid);
                    let bidx = dlg.buttons.iter().position(|(id, _)| *id == bid);
                    dlg.live_update(bidx)
                })
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            0
        }
        TDM_CLICK_RADIO_BUTTON => {
            let (push, cb_notify) = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h)
                    .map(|dlg| {
                        let matched = dlg.radios.iter().position(|(id, _)| *id == w_param as i32);
                        let notify = if let Some(idx) = matched {
                            dlg.selected_radio_idx = Some(idx);
                            (dlg.pf_callback != 0).then_some((
                                dlg.pf_callback,
                                dlg.lp_callback_data,
                                w_param as u64,
                            ))
                        } else {
                            None
                        };
                        (dlg.live_update(None), notify)
                    })
                    .unwrap_or((None, None))
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            if let Some((cb, ref_data, rid)) = cb_notify {
                let _ = crate::call_guest8(
                    cb,
                    h as u64,
                    TDN_RADIO_BUTTON_CLICKED as u64,
                    rid,
                    0,
                    ref_data as u64,
                    0,
                    0,
                    0,
                );
            }
            0
        }
        TDM_ENABLE_BUTTON => {
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    let bid = w_param as i32;
                    let enabled = l_param != 0;
                    if let Some(idx) = dlg.buttons.iter().position(|(id, _)| *id == bid) {
                        if let Some(slot) = dlg.button_enabled.get_mut(idx) {
                            *slot = enabled;
                        }
                        dlg.live_update(None)
                    } else {
                        None
                    }
                })
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            0
        }
        TDM_ENABLE_RADIO_BUTTON => {
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    let rid = w_param as i32;
                    let enabled = l_param != 0;
                    if let Some(idx) = dlg.radios.iter().position(|(id, _)| *id == rid) {
                        if let Some(slot) = dlg.radio_enabled.get_mut(idx) {
                            *slot = enabled;
                        }
                        dlg.live_update(None)
                    } else {
                        None
                    }
                })
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            0
        }
        TDM_CLICK_VERIFICATION => {
            let (push, cb_notify) = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h)
                    .map(|dlg| {
                        let notify = if dlg.has_verification {
                            let checked = w_param != 0;
                            dlg.verified = checked;
                            (dlg.pf_callback != 0).then_some((
                                dlg.pf_callback,
                                dlg.lp_callback_data,
                                if checked { 1u64 } else { 0u64 },
                            ))
                        } else {
                            None
                        };
                        (dlg.live_update(None), notify)
                    })
                    .unwrap_or((None, None))
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            if let Some((cb, ref_data, checked_u64)) = cb_notify {
                let _ = crate::call_guest8(
                    cb,
                    h as u64,
                    TDN_VERIFICATION_CLICKED as u64,
                    checked_u64,
                    0,
                    ref_data as u64,
                    0,
                    0,
                    0,
                );
            }
            0
        }
        _ => crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
}

struct TaskDialogHwndGuard {
    hwnd: usize,
}

impl Drop for TaskDialogHwndGuard {
    fn drop(&mut self) {
        active_task_dialogs()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&self.hwnd);
        unsafe {
            let _ = crate::user32::window::DestroyWindow(self.hwnd as HWND);
        }
    }
}

unsafe fn resolve_task_dialog_str(hinst: HINSTANCE, ptr: LPCWSTR) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let raw = ptr as usize;
    if raw <= 0xFFFF {
        let mut buf = [0u16; 512];
        let n = crate::user32::misc::LoadStringW(hinst, raw as u32, buf.as_mut_ptr(), buf.len() as i32);
        if n > 0 {
            String::from_utf16_lossy(&buf[..n as usize])
        } else {
            String::new()
        }
    } else {
        crate::strings::from_wide(ptr)
    }
}

fn strip_accelerator_ampersands(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                out.push('&');
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub unsafe extern "win64" fn TaskDialogIndirect(
    config: *const u8,
    button: *mut i32,
    radio_button: *mut i32,
    verification_flag: *mut BOOL,
) -> HRESULT {
    if !button.is_null() {
        button.write_unaligned(0);
    }
    if !radio_button.is_null() {
        radio_button.write_unaligned(0);
    }
    if !verification_flag.is_null() {
        verification_flag.write_unaligned(FALSE);
    }
    if config.is_null() {
        return E_INVALIDARG;
    }
    let cfg = (config as *const TaskDialogConfig).read_unaligned();
    if cfg.cb_size != std::mem::size_of::<TaskDialogConfig>() as u32 {
        return E_INVALIDARG;
    }
    if (cfg.c_buttons > 0 && cfg.p_buttons.is_null())
        || (cfg.c_radio_buttons > 0 && cfg.p_radio_buttons.is_null())
    {
        return E_INVALIDARG;
    }

    let hinst = cfg.h_instance;
    let title = resolve_task_dialog_str(hinst, cfg.psz_window_title);
    let main_inst = resolve_task_dialog_str(hinst, cfg.psz_main_instruction);
    let content = resolve_task_dialog_str(hinst, cfg.psz_content);
    let expanded = resolve_task_dialog_str(hinst, cfg.psz_expanded_information);
    let footer = resolve_task_dialog_str(hinst, cfg.psz_footer);

    // Collect custom buttons followed by common buttons in Windows dialog order.
    let mut buttons: Vec<(i32, String)> = Vec::new();
    for i in 0..cfg.c_buttons as usize {
        let b = cfg.p_buttons.add(i).read_unaligned();
        let bid = b.button_id;
        let label = strip_accelerator_ampersands(&resolve_task_dialog_str(hinst, b.button_text));
        buttons.push((bid, if label.is_empty() { format!("Button {bid}") } else { label }));
    }
    let common = cfg.dw_common_buttons;
    if (common & TDCBF_OK_BUTTON) != 0 {
        buttons.push((1, "OK".into()));
    }
    if (common & TDCBF_YES_BUTTON) != 0 {
        buttons.push((6, "Yes".into()));
    }
    if (common & TDCBF_NO_BUTTON) != 0 {
        buttons.push((7, "No".into()));
    }
    if (common & TDCBF_RETRY_BUTTON) != 0 {
        buttons.push((4, "Retry".into()));
    }
    if (common & TDCBF_CANCEL_BUTTON) != 0 {
        buttons.push((2, "Cancel".into()));
    }
    if (common & TDCBF_CLOSE_BUTTON) != 0 {
        buttons.push((8, "Close".into()));
    }
    if buttons.is_empty() {
        buttons.push((1, "OK".into()));
    }

    // Collect radio buttons and resolve initial default selection.
    let mut radios: Vec<(i32, String)> = Vec::with_capacity(cfg.c_radio_buttons as usize);
    for i in 0..cfg.c_radio_buttons as usize {
        let rb = cfg.p_radio_buttons.add(i).read_unaligned();
        let rid = rb.button_id;
        let label = strip_accelerator_ampersands(&resolve_task_dialog_str(hinst, rb.button_text));
        radios.push((rid, if label.is_empty() { format!("Option {rid}") } else { label }));
    }
    let selected_radio_idx: Option<usize> =
        if !radios.is_empty() && (cfg.dw_flags & TDF_NO_DEFAULT_RADIO_BUTTON) == 0 {
            if cfg.n_default_radio_button != 0 {
                radios
                    .iter()
                    .position(|(id, _)| *id == cfg.n_default_radio_button)
                    .or(Some(0))
            } else {
                Some(0)
            }
        } else {
            None
        };
    let verification_text: Option<String> = if !cfg.psz_verification_text.is_null() {
        let s = strip_accelerator_ampersands(&resolve_task_dialog_str(hinst, cfg.psz_verification_text));
        Some(if s.is_empty() { "Confirm".to_string() } else { s })
    } else {
        None
    };
    let verified: bool =
        verification_text.is_some() && (cfg.dw_flags & TDF_VERIFICATION_FLAG_CHECKED) != 0;

    // TD_ERROR_ICON == MAKEINTRESOURCEW(-2) == 0xFFFE.
    let critical = (cfg.main_icon & 0xFFFF) == 0xFFFE && cfg.main_icon <= 0xFFFF;
    let caption = if title.is_empty() {
        if critical { "Error".to_string() } else { "Notice".to_string() }
    } else {
        title
    };
    let labels: Vec<String> = buttons.iter().map(|(_, l)| l.clone()).collect();
    let default_button_idx: Option<usize> = if cfg.n_default_button != 0 {
        buttons.iter().position(|(id, _)| *id == cfg.n_default_button)
    } else {
        None
    };
    let radio_labels: Vec<String> = radios.iter().map(|(_, l)| l.clone()).collect();
    let has_selection_controls = !radios.is_empty() || verification_text.is_some();

    // Allocate a dialog-owned Win32 HWND (distinct from cfg.hwnd_parent, non-NULL even for
    // unparented dialogs) so callbacks using SetWindowText, SetWindowSubclass, or TDM_*
    // messages operate on the TaskDialog window rather than mutating the parent window.
    let session_id = NEXT_TASK_DIALOG_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dialog_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
        "#32770_TaskDialog",
        &caption,
        cfg.hwnd_parent as usize,
        task_dialog_wndproc_va(),
    );
    active_task_dialogs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(
            dialog_hwnd,
            ActiveTaskDialog {
                session_id,
                modal_open: false,
                hinst: hinst as usize,
                pf_callback: cfg.pf_callback,
                lp_callback_data: cfg.lp_callback_data,
                caption,
                main_inst,
                content,
                expanded,
                footer,
                buttons: buttons.clone(),
                button_enabled: vec![true; buttons.len()],
                radios: radios.clone(),
                radio_enabled: vec![true; radios.len()],
                selected_radio_idx,
                has_verification: verification_text.is_some(),
                verified,
                pending_click_button: None,
            },
        );
    let _dialog_guard = TaskDialogHwndGuard { hwnd: dialog_hwnd };

    let notify = |msg: u32, wp: u64| -> HRESULT {
        if cfg.pf_callback != 0 {
            crate::call_guest8(
                cfg.pf_callback,
                dialog_hwnd as u64,
                msg as u64,
                wp,
                0,
                cfg.lp_callback_data as u64,
                0,
                0,
                0,
            ) as HRESULT
        } else {
            S_OK
        }
    };

    let callback_timer = (cfg.dw_flags & TDF_CALLBACK_TIMER) != 0 && cfg.pf_callback != 0;
    let timer_interval = callback_timer.then_some(TASK_DIALOG_TIMER_INTERVAL);
    let mut timer_origin = std::time::Instant::now();

    let _ = notify(TDN_DIALOG_CONSTRUCTED, 0);
    let _ = notify(TDN_CREATED, 0);

    let chosen_id = loop {
        let (cur_caption, cur_body, cur_button_enabled, cur_radio_enabled, cur_radio_idx, cur_verified, programmatic_click) = {
            let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
            let dlg = map.get_mut(&dialog_hwnd).expect("active task dialog missing");
            let snap = (
                dlg.caption.clone(),
                dlg.compose_body(),
                dlg.button_enabled.clone(),
                dlg.radio_enabled.clone(),
                dlg.selected_radio_idx,
                dlg.verified,
                dlg.pending_click_button.take(),
            );
            if snap.6.is_none() {
                dlg.modal_open = true;
            }
            snap
        };

        if let Some(bid) = programmatic_click {
            let cb_hr = notify(TDN_BUTTON_CLICKED, bid as u64);
            if cb_hr != S_FALSE_HR {
                break bid;
            }
            continue;
        }

        let presented = crate::macos::task_dialog_alert(
            session_id,
            dialog_hwnd,
            cur_caption,
            cur_body,
            labels.clone(),
            cur_button_enabled.clone(),
            default_button_idx,
            radio_labels.clone(),
            cur_radio_enabled,
            cur_radio_idx,
            verification_text.clone(),
            cur_verified,
            critical,
            &mut timer_origin,
            timer_interval,
            |ev| match ev {
                crate::macos::TaskDialogEvent::RadioSelected(ridx) => {
                    let rid_opt = {
                        let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                        map.get_mut(&dialog_hwnd).and_then(|dlg| {
                            dlg.selected_radio_idx = Some(ridx);
                            radios.get(ridx).map(|(rid, _)| *rid)
                        })
                    };
                    if let Some(rid) = rid_opt {
                        let _ = notify(TDN_RADIO_BUTTON_CLICKED, rid as u64);
                    }
                }
                crate::macos::TaskDialogEvent::VerificationToggled(checked) => {
                    let has_ver = {
                        let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                        map.get_mut(&dialog_hwnd).is_some_and(|dlg| {
                            if dlg.has_verification {
                                dlg.verified = checked;
                                true
                            } else {
                                false
                            }
                        })
                    };
                    if has_ver {
                        let _ = notify(TDN_VERIFICATION_CLICKED, if checked { 1 } else { 0 });
                    }
                }
                crate::macos::TaskDialogEvent::ButtonClicked(_) => {}
            },
            |elapsed_ms| notify(TDN_TIMER, elapsed_ms as u64) == S_FALSE_HR,
        );
        // Consume any programmatic TDM_CLICK_BUTTON triggered while the native modal was open
        // so it is delivered exactly once and never replayed if TDN_BUTTON_CLICKED returns S_FALSE.
        let consumed_prog_click = {
            let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
            map.get_mut(&dialog_hwnd).and_then(|dlg| {
                dlg.modal_open = false;
                if let crate::macos::TaskDialogOutcome::Completed(ref res) = presented {
                    dlg.selected_radio_idx = res.radio_index;
                    if dlg.has_verification {
                        dlg.verified = res.verified;
                    }
                }
                dlg.pending_click_button.take()
            })
        };
        let (bid, interactive) = match presented {
            crate::macos::TaskDialogOutcome::Completed(res) => {
                let bid = consumed_prog_click.unwrap_or_else(|| {
                    buttons
                        .get(res.button_index)
                        .map(|(id, _)| *id)
                        .unwrap_or(buttons[0].0)
                });
                (bid, true)
            }
            crate::macos::TaskDialogOutcome::Quit => {
                // WM_QUIT arrived on the owning thread: exit the modal loop immediately
                // without synthesizing a button click, leaving the re-posted WM_QUIT
                // intact for the application's outer message loop.
                break 0;
            }
            crate::macos::TaskDialogOutcome::Unavailable => {
                let single_enabled = cur_button_enabled.first().copied().unwrap_or(true);
                if buttons.len() == 1 && !has_selection_controls && !callback_timer && single_enabled {
                    // Pure single-button informational dialog with an enabled button and no
                    // selection controls or timer can dismiss in headless mode (matching MessageBox MB_OK).
                    (buttons[0].0, false)
                } else {
                    // Multi-button prompts or dialogs with radio buttons / verification checkboxes /
                    // TDF_CALLBACK_TIMER require an interactive desktop so we never fabricate user
                    // selections or silently skip timer callbacks.
                    if callback_timer {
                        log::warn!("uxtheme: TaskDialogIndirect with TDF_CALLBACK_TIMER requires interactive desktop");
                    }
                    let _ = notify(TDN_DESTROYED, 0);
                    return HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION;
                }
            }
        };

        // Per PFTASKDIALOGCALLBACK: returning S_FALSE from TDN_BUTTON_CLICKED keeps the
        // dialog open instead of dismissing it.
        let cb_hr = notify(TDN_BUTTON_CLICKED, bid as u64);
        if cb_hr == S_FALSE_HR {
            if interactive {
                continue;
            }
            let _ = notify(TDN_DESTROYED, 0);
            return HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION;
        }
        break bid;
    };

    let (final_radio_idx, final_verified) = {
        let map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&dialog_hwnd)
            .map(|d| (d.selected_radio_idx, d.verified))
            .unwrap_or((selected_radio_idx, verified))
    };

    let _ = notify(TDN_DESTROYED, 0);

    let selected_radio = final_radio_idx
        .and_then(|idx| radios.get(idx))
        .map(|(id, _)| *id)
        .unwrap_or(0);
    if !button.is_null() {
        button.write_unaligned(chosen_id);
    }
    if !radio_button.is_null() {
        radio_button.write_unaligned(selected_radio);
    }
    if !verification_flag.is_null() {
        verification_flag.write_unaligned(if final_verified { TRUE } else { FALSE });
    }
    S_OK
}

// ---------------------------------------------------------------------------
// comctl32 window subclassing (ordinals 410..=413)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct SubclassNode {
    seq: u64,
    pfn: usize,
    id: usize,
    ref_data: usize,
}

struct SubclassHeader {
    orig_wndproc: usize,
    next_seq: u64,
    /// Ordered oldest (bottom) -> newest (top), strictly increasing `seq`.
    nodes: Vec<SubclassNode>,
    active_calls: u32,
}

#[derive(Clone, Copy, Debug)]
struct ActiveSubclassFrame {
    hwnd: usize,
    /// Upper bound for the next node to dispatch (`u64::MAX` before entering top).
    last_seq: u64,
    /// Snapshot of the original base WNDPROC captured when entering the master proc.
    orig_wndproc: usize,
}

fn subclass_table() -> &'static Mutex<HashMap<usize, SubclassHeader>> {
    static T: OnceLock<Mutex<HashMap<usize, SubclassHeader>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

thread_local! {
    static ACTIVE_SUBCLASS_CALLS: std::cell::RefCell<Vec<ActiveSubclassFrame>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

fn master_wndproc_va() -> usize {
    crate::user32::register_typed_host_wndproc(master_subclass_wndproc)
}

/// Remove any remaining subclass state for a destroyed window (called from
/// `user32::window::free_window` as well as after `WM_NCDESTROY`).
pub(crate) fn forget_window_subclasses(hwnd: usize) {
    subclass_table().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
}

fn try_detach_header_locked(
    map: &mut HashMap<usize, SubclassHeader>,
    hwnd: usize,
) -> Option<usize> {
    let should_remove = map
        .get(&hwnd)
        .is_some_and(|h| h.nodes.is_empty() && h.active_calls == 0);
    if should_remove {
        map.remove(&hwnd).map(|h| h.orig_wndproc)
    } else {
        None
    }
}

unsafe fn restore_orig_wndproc_if_master(hwnd: usize, orig_wndproc: usize) {
    if orig_wndproc == 0 || !crate::user32::window::exists(hwnd) {
        return;
    }
    let master = master_wndproc_va();
    if let Some(w) = crate::user32::window::get(hwnd) {
        let mut d = w.data();
        if d.wndproc == master {
            d.wndproc = orig_wndproc;
        }
    }
}

unsafe extern "win64" fn master_subclass_wndproc(
    hwnd: HWND,
    u_msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    let h = hwnd as usize;
    let orig_wndproc = {
        let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        match map.get_mut(&h) {
            Some(hdr) => {
                hdr.active_calls = hdr.active_calls.saturating_add(1);
                hdr.orig_wndproc
            }
            None => {
                drop(map);
                return crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param);
            }
        }
    };

    ACTIVE_SUBCLASS_CALLS.with(|stack| {
        stack.borrow_mut().push(ActiveSubclassFrame {
            hwnd: h,
            last_seq: u64::MAX,
            orig_wndproc,
        });
    });

    struct MasterExitGuard {
        hwnd: usize,
        is_ncdestroy: bool,
    }
    impl Drop for MasterExitGuard {
        fn drop(&mut self) {
            ACTIVE_SUBCLASS_CALLS.with(|stack| {
                stack.borrow_mut().pop();
            });
            let restore = {
                let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
                if self.is_ncdestroy {
                    map.remove(&self.hwnd);
                    None
                } else if let Some(hdr) = map.get_mut(&self.hwnd) {
                    hdr.active_calls = hdr.active_calls.saturating_sub(1);
                    try_detach_header_locked(&mut map, self.hwnd)
                } else {
                    None
                }
            };
            if let Some(orig) = restore {
                unsafe { restore_orig_wndproc_if_master(self.hwnd, orig) };
            }
        }
    }

    let _guard = MasterExitGuard {
        hwnd: h,
        is_ncdestroy: u_msg == crate::user32::wm::NCDESTROY,
    };

    DefSubclassProc(hwnd, u_msg, w_param, l_param)
}

pub unsafe extern "win64" fn SetWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
    dw_ref_data: usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let master = master_wndproc_va();
    let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
    let hdr = map.entry(h).or_insert_with(|| {
        let orig = crate::user32::window::get(h)
            .map(|w| {
                let mut d = w.data();
                let prev = d.wndproc;
                d.wndproc = master;
                prev
            })
            .unwrap_or(0);
        SubclassHeader {
            orig_wndproc: orig,
            next_seq: 1,
            nodes: Vec::new(),
            active_calls: 0,
        }
    });

    // Updating an existing (pfn, id) pair updates dwRefData in place without changing order.
    if let Some(existing) = hdr
        .nodes
        .iter_mut()
        .find(|n| n.pfn == pfn_subclass && n.id == u_id_subclass)
    {
        existing.ref_data = dw_ref_data;
        return TRUE;
    }

    let seq = hdr.next_seq;
    hdr.next_seq = hdr.next_seq.saturating_add(1);
    hdr.nodes.push(SubclassNode {
        seq,
        pfn: pfn_subclass,
        id: u_id_subclass,
        ref_data: dw_ref_data,
    });
    TRUE
}

pub unsafe extern "win64" fn GetWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
    pdw_ref_data: *mut usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(node) = map
        .get(&h)
        .and_then(|hdr| hdr.nodes.iter().find(|n| n.pfn == pfn_subclass && n.id == u_id_subclass))
    {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(node.ref_data);
        }
        TRUE
    } else {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn RemoveWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let (removed, restore_orig) = {
        let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        let Some(hdr) = map.get_mut(&h) else {
            return FALSE;
        };
        let Some(pos) = hdr
            .nodes
            .iter()
            .position(|n| n.pfn == pfn_subclass && n.id == u_id_subclass)
        else {
            return FALSE;
        };
        hdr.nodes.remove(pos);
        let restore = try_detach_header_locked(&mut map, h);
        (true, restore)
    };

    if let Some(orig) = restore_orig {
        restore_orig_wndproc_if_master(h, orig);
    }
    removed as BOOL
}

pub unsafe extern "win64" fn DefSubclassProc(
    hwnd: HWND,
    u_msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    let h = hwnd as usize;
    let frame_info: Option<(usize, u64, usize)> = ACTIVE_SUBCLASS_CALLS.with(|stack| {
        let s = stack.borrow();
        s.iter()
            .enumerate()
            .rev()
            .find(|(_, f)| f.hwnd == h)
            .map(|(idx, f)| (idx, f.last_seq, f.orig_wndproc))
    });

    let Some((frame_idx, saved_last_seq, frame_orig)) = frame_info else {
        return crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param);
    };

    // Find the highest-seq subclass strictly below `saved_last_seq`.
    let next_node: Option<SubclassNode> = {
        let map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&h).and_then(|hdr| {
            hdr.nodes
                .iter()
                .rev()
                .find(|n| n.seq < saved_last_seq)
                .copied()
        })
    };

    struct RestoreSeqGuard {
        frame_idx: usize,
        saved_last_seq: u64,
    }
    impl Drop for RestoreSeqGuard {
        fn drop(&mut self) {
            ACTIVE_SUBCLASS_CALLS.with(|stack| {
                let mut s = stack.borrow_mut();
                if let Some(frame) = s.get_mut(self.frame_idx) {
                    frame.last_seq = self.saved_last_seq;
                }
            });
        }
    }

    let next_seq = next_node.map(|n| n.seq).unwrap_or(0);
    ACTIVE_SUBCLASS_CALLS.with(|stack| {
        if let Some(frame) = stack.borrow_mut().get_mut(frame_idx) {
            frame.last_seq = next_seq;
        }
    });
    let _restore = RestoreSeqGuard {
        frame_idx,
        saved_last_seq,
    };

    if let Some(node) = next_node {
        // All locks are dropped before invoking guest SUBCLASSPROC (6 args).
        crate::call_guest8(
            node.pfn,
            h as u64,
            u_msg as u64,
            w_param as u64,
            l_param as u64,
            node.id as u64,
            node.ref_data as u64,
            0,
            0,
        ) as isize
    } else {
        let master = master_wndproc_va();
        if frame_orig != 0 && frame_orig != master {
            crate::user32::message::CallWindowProcW(frame_orig, hwnd, u_msg, w_param, l_param)
        } else {
            crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tme(flags: u32, hwnd: usize, hover: u32) -> [u8; 24] {
        let mut buf = [0u8; 24];
        buf[0..4].copy_from_slice(&24u32.to_le_bytes());
        buf[4..8].copy_from_slice(&flags.to_le_bytes());
        buf[8..16].copy_from_slice(&hwnd.to_le_bytes());
        buf[16..20].copy_from_slice(&hover.to_le_bytes());
        buf
    }

    #[test]
    fn honest_unthemed_fallback_and_task_dialog() {
        unsafe {
            assert_eq!(IsThemeActive(), FALSE);
            assert_eq!(IsAppThemed(), FALSE);
            assert_eq!(OpenThemeData(std::ptr::null_mut(), std::ptr::null()), 0);
            assert_eq!(OpenThemeDataEx(std::ptr::null_mut(), std::ptr::null(), 0), 0);
            assert_eq!(OpenThemeDataForDpi(std::ptr::null_mut(), std::ptr::null(), 96), 0);
            assert_eq!(CloseThemeData(0), E_HANDLE);
            assert_eq!(DrawThemeBackground(0, 0, 0, 0, std::ptr::null(), std::ptr::null()), E_HANDLE);
            // Non-zero unopened HTHEME must also fail with E_HANDLE when visual styles are off.
            assert_eq!(DrawThemeBackground(0x1234, 1, 1, 1, std::ptr::null(), std::ptr::null()), E_HANDLE);

            // Null config or wrong cbSize returns E_INVALIDARG and clears out params.
            let mut btn = 99;
            let mut radio = 99;
            let mut verified = TRUE;
            let hr = TaskDialogIndirect(std::ptr::null(), &mut btn, &mut radio, &mut verified);
            assert_eq!(hr, E_INVALIDARG);
            assert_eq!((btn, radio, verified), (0, 0, FALSE));

            // 1. Single-button informational TaskDialogIndirect (no selection controls)
            //    succeeds in headless mode and dispatches full callback lifecycle.
            static CB_MSGS: Mutex<Vec<(u32, u64)>> = Mutex::new(Vec::new());
            unsafe extern "win64" fn td_cb(
                _hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                CB_MSGS.lock().unwrap().push((msg, wp as u64));
                S_OK
            }
            let title_w = crate::strings::to_wide_z("Notice");
            let radio_lbl1 = crate::strings::to_wide_z("Option &1");
            let radio_lbl2 = crate::strings::to_wide_z("Option &2");
            let ver_lbl = crate::strings::to_wide_z("Remember choice");
            let radios = [
                TaskDialogButton { button_id: 201, button_text: radio_lbl1.as_ptr() },
                TaskDialogButton { button_id: 202, button_text: radio_lbl2.as_ptr() },
            ];
            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.dw_common_buttons = TDCBF_CLOSE_BUTTON;
            cfg.psz_window_title = title_w.as_ptr();
            cfg.pf_callback = td_cb as *const () as usize;

            CB_MSGS.lock().unwrap().clear();
            let hr = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            assert_eq!(hr, S_OK);
            assert_eq!(btn, 8); // IDCLOSE
            assert_eq!(radio, 0);
            assert_eq!(verified, FALSE);
            let msgs = CB_MSGS.lock().unwrap().clone();
            assert_eq!(
                msgs,
                vec![(TDN_DIALOG_CONSTRUCTED, 0), (TDN_CREATED, 0), (TDN_BUTTON_CLICKED, 8), (TDN_DESTROYED, 0)]
            );

            // 2. Headless dialog with radio buttons or verification checkbox must NOT fabricate
            //    selections — returns HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION with zeroed outputs.
            cfg.dw_flags = TDF_VERIFICATION_FLAG_CHECKED;
            cfg.c_radio_buttons = radios.len() as u32;
            cfg.p_radio_buttons = radios.as_ptr();
            cfg.n_default_radio_button = 202;
            cfg.psz_verification_text = ver_lbl.as_ptr();
            btn = 99;
            radio = 99;
            verified = TRUE;
            let hr_sel_headless = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            assert_eq!(hr_sel_headless, HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION);
            assert_eq!((btn, radio, verified), (0, 0, FALSE));

            // 3. Interactive TaskDialogIndirect renders radio + verification controls, honors
            //    S_FALSE from TDN_BUTTON_CLICKED to keep the dialog open, and returns the user's
            //    actual final selections once the callback allows dismissal.
            static REJECT_ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
            unsafe extern "win64" fn td_cb_keep_open(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                CB_MSGS.lock().unwrap().push((msg, wp as u64));
                if msg == TDN_RADIO_BUTTON_CLICKED && wp == 201 {
                    let live_body = crate::strings::to_wide_z("Live Radio 201 Body");
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_ELEMENT_TEXT,
                        TDE_CONTENT,
                        live_body.as_ptr() as isize,
                    );
                }
                if msg == TDN_BUTTON_CLICKED && REJECT_ONCE.swap(false, std::sync::atomic::Ordering::SeqCst) {
                    return S_FALSE_HR;
                }
                S_OK
            }
            cfg.dw_common_buttons = TDCBF_YES_BUTTON | TDCBF_NO_BUTTON;
            cfg.n_default_button = 7; // IDNO -> index 1
            cfg.pf_callback = td_cb_keep_open as *const () as usize;
            CB_MSGS.lock().unwrap().clear();
            REJECT_ONCE.store(true, std::sync::atomic::Ordering::SeqCst);

            let live_updates: std::sync::Arc<Mutex<Vec<String>>> =
                std::sync::Arc::new(Mutex::new(Vec::new()));
            let live_updates_hook = live_updates.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |_sid, upd| {
                live_updates_hook.lock().unwrap().push(upd.text.clone());
            })));

            let mut step = 0usize;
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, body, btns, _btn_en, def_btn, rads, _rad_en, def_rad, ver_txt, ver_chk, emit| {
                    step += 1;
                    assert_eq!(btns, &["Yes", "No"]);
                    assert_eq!(def_btn, Some(1)); // n_default_button == IDNO (7)
                    assert_eq!(rads, &["Option 1", "Option 2"]);
                    assert_eq!(ver_txt, Some("Remember choice"));
                    if step == 1 {
                        // Initial defaults: radio index 1 (ID 202), verified = true.
                        assert_eq!(def_rad, Some(1));
                        assert!(ver_chk);
                        // Emit intermediate live clicks while the modal dialog is open (201 -> 202 -> 201,
                        // plus verification unchecked) and verify callbacks run BEFORE dismissal.
                        emit(crate::macos::TaskDialogEvent::RadioSelected(0));
                        emit(crate::macos::TaskDialogEvent::RadioSelected(1));
                        emit(crate::macos::TaskDialogEvent::RadioSelected(0));
                        emit(crate::macos::TaskDialogEvent::VerificationToggled(false));
                        assert!(
                            CB_MSGS.lock().unwrap().contains(&(TDN_RADIO_BUTTON_CLICKED, 202)),
                            "intermediate radio click must be delivered live before modal dismissal"
                        );
                        // User clicks "Yes" (idx 0); callback returns S_FALSE so the dialog remains open.
                        crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                            button_index: 0,
                            radio_index: Some(0),
                            verified: false,
                        })
                    } else {
                        // Second presentation carries forward both updated selection state and live content.
                        assert_eq!(body, "Live Radio 201 Body");
                        assert_eq!(def_rad, Some(0));
                        assert!(!ver_chk);
                        // User clicks "No" (idx 1) with radio 0 (ID 201) and verified = false.
                        crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                            button_index: 1,
                            radio_index: Some(0),
                            verified: false,
                        })
                    }
                },
            )));
            let hr_interactive = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            crate::macos::set_test_task_dialog_updater(None);
            assert_eq!(hr_interactive, S_OK);
            assert_eq!(btn, 7); // IDNO (second click after first S_FALSE kept dialog open)
            assert_eq!(radio, 201); // updated radio choice, not default 202
            assert_eq!(verified, FALSE); // updated checkbox state, not default TRUE
            assert!(
                live_updates.lock().unwrap().iter().any(|t| t == "Live Radio 201 Body"),
                "live update from TDN_RADIO_BUTTON_CLICKED must be pushed while modal is open"
            );
            assert_eq!(
                CB_MSGS.lock().unwrap().clone(),
                vec![
                    (TDN_DIALOG_CONSTRUCTED, 0),
                    (TDN_CREATED, 0),
                    (TDN_RADIO_BUTTON_CLICKED, 201),
                    (TDN_RADIO_BUTTON_CLICKED, 202),
                    (TDN_RADIO_BUTTON_CLICKED, 201),
                    (TDN_VERIFICATION_CLICKED, 0),
                    (TDN_BUTTON_CLICKED, 6), // IDYES rejected with S_FALSE
                    (TDN_BUTTON_CLICKED, 7), // IDNO accepted with S_OK
                    (TDN_DESTROYED, 0),
                ]
            );

            // 4. Headless single-button dialog whose callback returns S_FALSE (refusing dismissal)
            //    must NOT silently return S_OK.
            cfg.c_radio_buttons = 0;
            cfg.p_radio_buttons = std::ptr::null();
            cfg.psz_verification_text = std::ptr::null();
            cfg.dw_common_buttons = TDCBF_CLOSE_BUTTON;
            REJECT_ONCE.store(true, std::sync::atomic::Ordering::SeqCst);
            let hr_headless_reject = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            assert_eq!(hr_headless_reject, HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION);

            // 5. Multi-button prompt in headless mode returns HRESULT_FROM_WIN32(ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION).
            cfg.dw_common_buttons = TDCBF_YES_BUTTON | TDCBF_NO_BUTTON;
            cfg.pf_callback = 0;
            btn = 99;
            let hr_multi = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            assert_eq!(hr_multi, HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION);
            assert_eq!(btn, 0);
        }
    }

    #[test]
    fn task_dialog_callback_receives_dialog_owned_hwnd_and_routes_operations() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            // Create a real parent window whose title and subclass chain must remain untouched.
            let cls = crate::strings::to_wide_z("TaskDialogParentTestWnd");
            let parent_title = crate::strings::to_wide_z("Parent Window");
            let wc = crate::user32::class::WNDCLASS {
                style: 0,
                wndproc: crate::user32::register_typed_host_wndproc(crate::user32::defproc::DefWindowProcW),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = crate::user32::class::RegisterClassW(&wc);
            let parent_hwnd = crate::user32::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                parent_title.as_ptr(),
                crate::user32::ws::POPUP,
                0,
                0,
                200,
                200,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!parent_hwnd.is_null());

            const WM_TD_PING: u32 = crate::user32::wm::USER + 42;
            static SEEN_HWNDS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
            static SEEN_NOTIFS: Mutex<Vec<(u32, usize)>> = Mutex::new(Vec::new());
            static SUB_PINGS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

            unsafe extern "win64" fn td_subclass(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                lp: isize,
                uid: usize,
                ref_data: usize,
            ) -> isize {
                if msg == WM_TD_PING && uid == 1 && ref_data == 0xCAFE {
                    SUB_PINGS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    return 777;
                }
                DefSubclassProc(hwnd, msg, wp, lp)
            }

            unsafe extern "win64" fn td_init_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                assert!(!hwnd.is_null(), "callback must always receive a non-NULL dialog HWND");
                assert_eq!(crate::user32::window::IsWindow(hwnd), TRUE);
                SEEN_HWNDS.lock().unwrap().push(hwnd as usize);
                SEEN_NOTIFS.lock().unwrap().push((msg, wp));
                if msg == TDN_CREATED {
                    let new_title = crate::strings::to_wide_z("Callback Title");
                    assert_eq!(crate::user32::window::SetWindowTextW(hwnd, new_title.as_ptr()), TRUE);
                    let new_body = crate::strings::to_wide_z("Initial Callback Body");
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_ELEMENT_TEXT,
                        TDE_CONTENT,
                        new_body.as_ptr() as isize,
                    );
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_CLICK_RADIO_BUTTON, 201, 0);
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_CLICK_VERIFICATION, 0, 0);
                    // Unknown radio ID must not trigger TDN_RADIO_BUTTON_CLICKED.
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_CLICK_RADIO_BUTTON, 999, 0);
                    assert_eq!(SetWindowSubclass(hwnd, td_subclass as *const () as usize, 1, 0xCAFE), TRUE);
                    assert_eq!(crate::user32::message::SendMessageW(hwnd, WM_TD_PING, 0, 0), 777);
                } else if msg == TDN_RADIO_BUTTON_CLICKED && wp == 201 {
                    // Callback invoked outside mutex can mutate dialog content reentrantly.
                    let new_body = crate::strings::to_wide_z("Updated Body");
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_ELEMENT_TEXT,
                        TDE_CONTENT,
                        new_body.as_ptr() as isize,
                    );
                }
                S_OK
            }

            // 1. Parented dialog: callback gets a distinct dialog-owned HWND, mutates dialog
            //    title/content/radio/verification & subclasses it without affecting parent_hwnd.
            let orig_title = crate::strings::to_wide_z("Initial Title");
            let orig_body = crate::strings::to_wide_z("Initial Body");
            let radio_lbl1 = crate::strings::to_wide_z("Alpha");
            let radio_lbl2 = crate::strings::to_wide_z("Beta");
            let ver_lbl = crate::strings::to_wide_z("Check me");
            let radios = [
                TaskDialogButton { button_id: 201, button_text: radio_lbl1.as_ptr() },
                TaskDialogButton { button_id: 202, button_text: radio_lbl2.as_ptr() },
            ];
            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.hwnd_parent = parent_hwnd;
            cfg.dw_flags = TDF_VERIFICATION_FLAG_CHECKED;
            cfg.dw_common_buttons = TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON;
            cfg.psz_window_title = orig_title.as_ptr();
            cfg.psz_content = orig_body.as_ptr();
            cfg.c_radio_buttons = radios.len() as u32;
            cfg.p_radio_buttons = radios.as_ptr();
            cfg.n_default_radio_button = 202;
            cfg.psz_verification_text = ver_lbl.as_ptr();
            cfg.pf_callback = td_init_cb as *const () as usize;

            SEEN_HWNDS.lock().unwrap().clear();
            SEEN_NOTIFS.lock().unwrap().clear();
            SUB_PINGS.store(0, std::sync::atomic::Ordering::SeqCst);
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                |cap, body, _btns, _btn_en, _def_btn, _rads, _rad_en, def_rad, _ver_txt, ver_chk, _emit| {
                    assert_eq!(cap, "Callback Title");
                    assert_eq!(body, "Updated Body");
                    assert_eq!(def_rad, Some(0)); // updated via TDM_CLICK_RADIO_BUTTON(201)
                    assert!(!ver_chk); // updated via TDM_CLICK_VERIFICATION(0)
                    crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                        button_index: 0,
                        radio_index: def_rad,
                        verified: ver_chk,
                    })
                },
            )));

            let mut btn = 0;
            let mut radio = 0;
            let mut verified = TRUE;
            let hr = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            assert_eq!(hr, S_OK);
            assert_eq!((btn, radio, verified), (1, 201, FALSE));
            assert_eq!(SUB_PINGS.load(std::sync::atomic::Ordering::SeqCst), 1);
            let notifs = SEEN_NOTIFS.lock().unwrap().clone();
            assert!(
                notifs.contains(&(TDN_RADIO_BUTTON_CLICKED, 201)),
                "TDM_CLICK_RADIO_BUTTON must invoke TDN_RADIO_BUTTON_CLICKED: {notifs:?}"
            );
            assert!(
                !notifs.iter().any(|&(m, wp)| m == TDN_RADIO_BUTTON_CLICKED && wp == 999),
                "unknown radio ID must not trigger TDN_RADIO_BUTTON_CLICKED: {notifs:?}"
            );
            assert!(
                notifs.contains(&(TDN_VERIFICATION_CLICKED, 0)),
                "TDM_CLICK_VERIFICATION must invoke TDN_VERIFICATION_CLICKED: {notifs:?}"
            );

            let seen = SEEN_HWNDS.lock().unwrap().clone();
            assert!(!seen.is_empty());
            let dlg_hwnd = seen[0];
            assert!(seen.iter().all(|&h| h == dlg_hwnd), "all notifications must share one dialog HWND");
            assert_ne!(dlg_hwnd, 0);
            assert_ne!(dlg_hwnd, parent_hwnd as usize);
            // Dialog HWND and its subclass entry must be destroyed on return.
            assert_eq!(crate::user32::window::IsWindow(dlg_hwnd as HWND), FALSE);
            assert!(!subclass_table().lock().unwrap().contains_key(&dlg_hwnd));
            // Parent window title and subclass table must be untouched.
            let mut pbuf = [0u16; 64];
            let plen = crate::user32::window::GetWindowTextW(parent_hwnd, pbuf.as_mut_ptr(), pbuf.len() as i32);
            assert_eq!(String::from_utf16_lossy(&pbuf[..plen as usize]), "Parent Window");
            assert!(!subclass_table().lock().unwrap().contains_key(&(parent_hwnd as usize)));

            // 2. Unparented dialog (hwnd_parent == NULL) must still pass a real, non-NULL HWND.
            let mut cfg_unparented: TaskDialogConfig = std::mem::zeroed();
            cfg_unparented.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg_unparented.hwnd_parent = std::ptr::null_mut();
            cfg_unparented.dw_common_buttons = TDCBF_CLOSE_BUTTON;
            cfg_unparented.psz_window_title = orig_title.as_ptr();
            cfg_unparented.pf_callback = td_init_cb as *const () as usize;

            SEEN_HWNDS.lock().unwrap().clear();
            let hr_unp = TaskDialogIndirect(
                &cfg_unparented as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            assert_eq!(hr_unp, S_OK);
            let unp_seen = SEEN_HWNDS.lock().unwrap().clone();
            assert!(!unp_seen.is_empty());
            assert_ne!(unp_seen[0], 0);
            assert_eq!(crate::user32::window::IsWindow(unp_seen[0] as HWND), FALSE);

            let _ = crate::user32::window::DestroyWindow(parent_hwnd);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn task_dialog_consumes_live_programmatic_click_once_and_pumps_cross_thread_messages() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            // 1. Live TDM_CLICK_BUTTON from TDN_RADIO_BUTTON_CLICKED whose TDN_BUTTON_CLICKED
            //    returns S_FALSE must NOT be replayed on the next presentation loop iteration.
            static BUTTON_CLICKS: Mutex<Vec<i32>> = Mutex::new(Vec::new());
            static VETO_FIRST: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
            static LIVE_DIALOG_HWND: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

            unsafe extern "win64" fn prog_click_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                if msg == TDN_CREATED {
                    LIVE_DIALOG_HWND.store(hwnd as usize, std::sync::atomic::Ordering::Release);
                } else if msg == TDN_RADIO_BUTTON_CLICKED && wp == 201 {
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_CLICK_BUTTON, 6, 0);
                } else if msg == TDN_BUTTON_CLICKED {
                    BUTTON_CLICKS.lock().unwrap().push(wp as i32);
                    if VETO_FIRST.swap(false, std::sync::atomic::Ordering::SeqCst) {
                        return S_FALSE_HR;
                    }
                }
                S_OK
            }

            let title_w = crate::strings::to_wide_z("ProgClick");
            let r1 = crate::strings::to_wide_z("R1");
            let r2 = crate::strings::to_wide_z("R2");
            let radios = [
                TaskDialogButton { button_id: 201, button_text: r1.as_ptr() },
                TaskDialogButton { button_id: 202, button_text: r2.as_ptr() },
            ];
            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.dw_common_buttons = TDCBF_YES_BUTTON | TDCBF_NO_BUTTON;
            cfg.psz_window_title = title_w.as_ptr();
            cfg.c_radio_buttons = radios.len() as u32;
            cfg.p_radio_buttons = radios.as_ptr();
            cfg.n_default_radio_button = 202;
            cfg.pf_callback = prog_click_cb as *const () as usize;

            BUTTON_CLICKS.lock().unwrap().clear();
            VETO_FIRST.store(true, std::sync::atomic::Ordering::SeqCst);
            let mut step = 0usize;
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, _body, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, emit| {
                    step += 1;
                    if step == 1 {
                        // Triggers TDM_CLICK_BUTTON(6) while modal_open == true.
                        emit(crate::macos::TaskDialogEvent::RadioSelected(0));
                        // Native modal exits with button_index 0 (IDYES = 6) due to stopModalWithCode.
                        crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                            button_index: 0,
                            radio_index: Some(0),
                            verified: false,
                        })
                    } else {
                        // Second presentation must be reached (not bypassed by a stale pending click!).
                        crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                            button_index: 1, // IDNO = 7
                            radio_index: Some(0),
                            verified: false,
                        })
                    }
                },
            )));
            let mut btn = 0;
            let mut radio = 0;
            let mut verified = FALSE;
            assert_eq!(
                TaskDialogIndirect(
                    &cfg as *const TaskDialogConfig as *const u8,
                    &mut btn,
                    &mut radio,
                    &mut verified,
                ),
                S_OK
            );
            crate::macos::set_test_task_dialog_presenter(None);
            assert_eq!(btn, 7);
            assert_eq!(BUTTON_CLICKS.lock().unwrap().clone(), vec![6, 7]);

            // 2. Cross-thread SendMessageW to an open dialog HWND while the owner thread waits
            //    in pump_task_dialog_wait_loop must be processed without deadlocking and push
            //    the live content update + programmatic dismissal.
            let dlg_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
                "#32770_TaskDialog",
                "WaitTitle",
                0,
                task_dialog_wndproc_va(),
            );
            let sid = NEXT_TASK_DIALOG_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            active_task_dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(
                    dlg_hwnd,
                    ActiveTaskDialog {
                        session_id: sid,
                        modal_open: true,
                        hinst: 0,
                        pf_callback: 0,
                        lp_callback_data: 0,
                        caption: "WaitTitle".to_string(),
                        main_inst: String::new(),
                        content: "Before".to_string(),
                        expanded: String::new(),
                        footer: String::new(),
                        buttons: vec![(1, "OK".to_string())],
                        button_enabled: vec![true],
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        has_verification: false,
                        verified: false,
                        pending_click_button: None,
                    },
                );
            let _guard = TaskDialogHwndGuard { hwnd: dlg_hwnd };
            let (tx, rx) = std::sync::mpsc::channel::<crate::macos::TaskDialogEvent>();
            let tx_upd = tx.clone();
            let saw_text = std::sync::Arc::new(Mutex::new(String::new()));
            let saw_text_hook = saw_text.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |u_sid, upd| {
                if u_sid == sid {
                    *saw_text_hook.lock().unwrap() = upd.text.clone();
                    if let Some(bidx) = upd.click_button_idx {
                        let _ = tx_upd.send(crate::macos::TaskDialogEvent::ButtonClicked(
                            crate::macos::TaskDialogAlertResult {
                                button_index: bidx,
                                radio_index: None,
                                verified: false,
                            },
                        ));
                    }
                }
            })));
            let worker = std::thread::spawn(move || {
                let updated = crate::strings::to_wide_z("From Worker Thread");
                let _ = crate::user32::message::SendMessageW(
                    dlg_hwnd as HWND,
                    TDM_SET_ELEMENT_TEXT,
                    TDE_CONTENT,
                    updated.as_ptr() as isize,
                );
                let _ = crate::user32::message::SendMessageW(dlg_hwnd as HWND, TDM_CLICK_BUTTON, 1, 0);
            });
            let mut noop = |_ev: crate::macos::TaskDialogEvent| {};
            let mut noop_timer = |_ms: u32| false;
            let mut origin = std::time::Instant::now();
            let res = crate::macos::pump_task_dialog_wait_loop(
                dlg_hwnd,
                rx,
                &mut origin,
                None,
                &mut noop,
                &mut noop_timer,
                || {},
            );
            worker.join().unwrap();
            crate::macos::set_test_task_dialog_updater(None);
            assert_eq!(
                res,
                crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                    button_index: 0,
                    radio_index: None,
                    verified: false,
                })
            );
            assert_eq!(saw_text.lock().unwrap().as_str(), "From Worker Thread");

            // 3. TDF_CALLBACK_TIMER: pump_task_dialog_wait_loop delivers periodic TDN_TIMER ticks
            //    with elapsed ms, resets tick origin when callback returns S_FALSE, and allows the
            //    timer callback to dismiss the dialog live via TDM_CLICK_BUTTON.
            let timer_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
                "#32770_TaskDialog",
                "TimerTitle",
                0,
                task_dialog_wndproc_va(),
            );
            let timer_sid = NEXT_TASK_DIALOG_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            active_task_dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(
                    timer_hwnd,
                    ActiveTaskDialog {
                        session_id: timer_sid,
                        modal_open: true,
                        hinst: 0,
                        pf_callback: 0,
                        lp_callback_data: 0,
                        caption: "TimerTitle".to_string(),
                        main_inst: String::new(),
                        content: "Waiting".to_string(),
                        expanded: String::new(),
                        footer: String::new(),
                        buttons: vec![(1, "OK".to_string())],
                        button_enabled: vec![true],
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        has_verification: false,
                        verified: false,
                        pending_click_button: None,
                    },
                );
            let _timer_guard = TaskDialogHwndGuard { hwnd: timer_hwnd };
            let (ttx, trx) = std::sync::mpsc::channel::<crate::macos::TaskDialogEvent>();
            let ttx_upd = ttx.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |u_sid, upd| {
                if u_sid == timer_sid {
                    if let Some(bidx) = upd.click_button_idx {
                        let _ = ttx_upd.send(crate::macos::TaskDialogEvent::ButtonClicked(
                            crate::macos::TaskDialogAlertResult {
                                button_index: bidx,
                                radio_index: None,
                                verified: false,
                            },
                        ));
                    }
                }
            })));

            let mut ticks: Vec<u32> = Vec::new();
            let mut t_origin = std::time::Instant::now() - std::time::Duration::from_millis(50);
            let mut on_timer = |elapsed_ms: u32| -> bool {
                ticks.push(elapsed_ms);
                match ticks.len() {
                    1 => false, // S_OK: keep accumulating elapsed ms
                    2 => true,  // S_FALSE: reset tick origin
                    _ => {
                        let _ = crate::user32::message::SendMessageW(
                            timer_hwnd as HWND,
                            TDM_CLICK_BUTTON,
                            1,
                            0,
                        );
                        false
                    }
                }
            };
            let timer_res = crate::macos::pump_task_dialog_wait_loop(
                timer_hwnd,
                trx,
                &mut t_origin,
                Some(std::time::Duration::from_millis(20)),
                &mut noop,
                &mut on_timer,
                || {},
            );
            crate::macos::set_test_task_dialog_updater(None);
            assert_eq!(
                timer_res,
                crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                    button_index: 0,
                    radio_index: None,
                    verified: false,
                })
            );
            assert_eq!(ticks.len(), 3, "expected 3 timer ticks before dismissal: {ticks:?}");
            assert!(ticks[0] >= 50, "first tick should include initial elapsed time: {ticks:?}");
            assert!(ticks[1] > ticks[0], "second tick should increase without reset: {ticks:?}");
            assert!(
                ticks[2] < ticks[1],
                "third tick should reset after S_FALSE on second tick: {ticks:?}"
            );

            // 4. Headless TaskDialogIndirect with TDF_CALLBACK_TIMER must refuse honestly
            //    rather than silently dismissing without running the timer.
            unsafe extern "win64" fn noop_td_cb(
                _hwnd: HWND,
                _msg: u32,
                _wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                S_OK
            }
            let mut timer_cfg: TaskDialogConfig = std::mem::zeroed();
            timer_cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            timer_cfg.dw_flags = TDF_CALLBACK_TIMER;
            timer_cfg.dw_common_buttons = TDCBF_OK_BUTTON;
            timer_cfg.pf_callback = noop_td_cb as *const () as usize;
            assert_eq!(
                TaskDialogIndirect(
                    &timer_cfg as *const TaskDialogConfig as *const u8,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION
            );

            // 5. WM_QUIT arriving during pump_task_dialog_wait_loop must dismiss/cancel
            //    the modal session and preserve the exit code for the outer message loop.
            let quit_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
                "#32770_TaskDialog",
                "QuitWaitTitle",
                0,
                task_dialog_wndproc_va(),
            );
            let _quit_guard = TaskDialogHwndGuard { hwnd: quit_hwnd };
            let (_qtx, qrx) = std::sync::mpsc::channel::<crate::macos::TaskDialogEvent>();
            let mut cancelled = false;
            let mut q_origin = std::time::Instant::now();
            let mut quit_timer = |_ms: u32| -> bool {
                crate::user32::message::PostQuitMessage(-7);
                false
            };
            let quit_outcome = crate::macos::pump_task_dialog_wait_loop(
                quit_hwnd,
                qrx,
                &mut q_origin,
                Some(std::time::Duration::from_millis(5)),
                &mut noop,
                &mut quit_timer,
                || {
                    cancelled = true;
                },
            );
            assert_eq!(quit_outcome, crate::macos::TaskDialogOutcome::Quit);
            assert!(cancelled, "WM_QUIT must invoke on_cancel to unwind the native modal");
            let mut outer_msg = crate::user32::MSG::default();
            assert_eq!(
                crate::user32::message::GetMessageW(&mut outer_msg, std::ptr::null_mut(), 0, 0),
                FALSE,
                "outer message loop must still receive WM_QUIT"
            );
            assert_eq!(outer_msg.message, crate::user32::wm::QUIT);
            assert_eq!(outer_msg.wparam as i32, -7);

            // 6. End-to-end TaskDialogIndirect unwinds on WM_QUIT without invoking
            //    TDN_BUTTON_CLICKED (even if a callback would veto button clicks),
            //    delivers TDN_DESTROYED, and preserves WM_QUIT for the outer loop.
            static QUIT_CB_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
            unsafe extern "win64" fn quit_td_cb(
                _hwnd: HWND,
                msg: u32,
                _wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                QUIT_CB_LOG.lock().unwrap().push(msg);
                if msg == TDN_BUTTON_CLICKED {
                    return S_FALSE_HR;
                }
                S_OK
            }
            QUIT_CB_LOG.lock().unwrap().clear();
            let mut quit_cfg: TaskDialogConfig = std::mem::zeroed();
            quit_cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            quit_cfg.dw_flags = TDF_CALLBACK_TIMER;
            quit_cfg.dw_common_buttons = TDCBF_YES_BUTTON | TDCBF_NO_BUTTON;
            quit_cfg.psz_window_title = title_w.as_ptr();
            quit_cfg.pf_callback = quit_td_cb as *const () as usize;

            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                |_cap, _body, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, _emit| {
                    crate::user32::message::PostQuitMessage(77);
                    crate::macos::TaskDialogOutcome::Quit
                },
            )));
            let mut qbtn = 99;
            let mut qradio = 99;
            let mut qver = TRUE;
            let qhr = TaskDialogIndirect(
                &quit_cfg as *const TaskDialogConfig as *const u8,
                &mut qbtn,
                &mut qradio,
                &mut qver,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            assert_eq!(qhr, S_OK);
            assert_eq!(qbtn, 0);
            let qlog = QUIT_CB_LOG.lock().unwrap().clone();
            assert!(!qlog.contains(&TDN_BUTTON_CLICKED), "WM_QUIT must not synthesize TDN_BUTTON_CLICKED: {qlog:?}");
            assert!(qlog.contains(&TDN_DESTROYED), "WM_QUIT unwind must deliver TDN_DESTROYED: {qlog:?}");
            let mut outer_msg2 = crate::user32::MSG::default();
            assert_eq!(
                crate::user32::message::GetMessageW(&mut outer_msg2, std::ptr::null_mut(), 0, 0),
                FALSE
            );
            assert_eq!(outer_msg2.message, crate::user32::wm::QUIT);
            assert_eq!(outer_msg2.wparam as i32, 77);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn task_dialog_enable_and_disable_buttons_before_and_during_presentation() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            // Callback disables OK (ID 1) and radio 202 during TDN_CREATED, then re-enables
            // OK (and radio 202) live when the user checks the verification checkbox.
            unsafe extern "win64" fn gate_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                if msg == TDN_CREATED {
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_ENABLE_BUTTON, 1, 0);
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_ENABLE_RADIO_BUTTON, 202, 0);
                } else if msg == TDN_VERIFICATION_CLICKED {
                    let en: isize = if wp != 0 { 1 } else { 0 };
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_ENABLE_BUTTON, 1, en);
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_ENABLE_RADIO_BUTTON, 202, en);
                }
                S_OK
            }

            let title_w = crate::strings::to_wide_z("Gate");
            let ver_w = crate::strings::to_wide_z("I accept");
            let r1 = crate::strings::to_wide_z("R1");
            let r2 = crate::strings::to_wide_z("R2");
            let radios = [
                TaskDialogButton { button_id: 201, button_text: r1.as_ptr() },
                TaskDialogButton { button_id: 202, button_text: r2.as_ptr() },
            ];

            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.dw_common_buttons = TDCBF_OK_BUTTON | TDCBF_CANCEL_BUTTON;
            cfg.psz_window_title = title_w.as_ptr();
            cfg.c_radio_buttons = radios.len() as u32;
            cfg.p_radio_buttons = radios.as_ptr();
            cfg.psz_verification_text = ver_w.as_ptr();
            cfg.pf_callback = gate_cb as *const () as usize;

            let live_btn_states: std::sync::Arc<Mutex<Vec<(Vec<bool>, Vec<bool>)>>> =
                std::sync::Arc::new(Mutex::new(Vec::new()));
            let live_hook = live_btn_states.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |_sid, upd| {
                live_hook
                    .lock()
                    .unwrap()
                    .push((upd.button_enabled.clone(), upd.radio_enabled.clone()));
            })));

            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, _body, btns, btn_en, _db, rads, rad_en, _dr, _vt, _vc, emit| {
                    assert_eq!(btns, &["OK", "Cancel"]);
                    assert_eq!(rads, &["R1", "R2"]);
                    // Pre-presentation TDM_ENABLE_BUTTON(1, FALSE) and TDM_ENABLE_RADIO_BUTTON(202, FALSE)
                    // sent in TDN_CREATED must already be reflected at initial presentation.
                    assert_eq!(btn_en, &[false, true]);
                    assert_eq!(rad_en, &[true, false]);

                    // User checks verification box -> callback sends live TDM_ENABLE_BUTTON(1, TRUE).
                    emit(crate::macos::TaskDialogEvent::VerificationToggled(true));

                    crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                        button_index: 0,
                        radio_index: Some(1),
                        verified: true,
                    })
                },
            )));

            let mut btn = 0;
            let mut radio = 0;
            let mut verified = FALSE;
            let hr = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut radio,
                &mut verified,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            crate::macos::set_test_task_dialog_updater(None);

            assert_eq!(hr, S_OK);
            assert_eq!((btn, radio, verified), (1, 202, TRUE));
            let updates = live_btn_states.lock().unwrap().clone();
            assert!(
                updates.iter().any(|(be, re)| be == &[true, true] && re == &[true, true]),
                "live TDM_ENABLE_BUTTON/TDM_ENABLE_RADIO_BUTTON must push updated enabled states: {updates:?}"
            );

            // Headless single-button dialog whose sole button is disabled in TDN_CREATED
            // must NOT auto-dismiss — it returns HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION.
            unsafe extern "win64" fn disable_only_ok_cb(
                hwnd: HWND,
                msg: u32,
                _wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                if msg == TDN_CREATED {
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_ENABLE_BUTTON, 1, 0);
                }
                S_OK
            }
            let mut single_cfg: TaskDialogConfig = std::mem::zeroed();
            single_cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            single_cfg.dw_common_buttons = TDCBF_OK_BUTTON;
            single_cfg.psz_window_title = title_w.as_ptr();
            single_cfg.pf_callback = disable_only_ok_cb as *const () as usize;
            let mut sbtn = 99;
            assert_eq!(
                TaskDialogIndirect(
                    &single_cfg as *const TaskDialogConfig as *const u8,
                    &mut sbtn,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION
            );
            assert_eq!(sbtn, 0);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn task_dialog_wait_loop_pumps_owner_window_posted_messages_and_timers() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            const WM_WORK_DONE: u32 = crate::user32::wm::USER + 77;
            static OWNER_DIALOG_HWND: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);
            static OWNER_TIMER_HITS: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);

            unsafe extern "win64" fn owner_wndproc(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                lp: isize,
            ) -> isize {
                if msg == crate::user32::wm::TIMER {
                    OWNER_TIMER_HITS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    return 0;
                }
                if msg == WM_WORK_DONE {
                    let dlg = OWNER_DIALOG_HWND.load(std::sync::atomic::Ordering::Acquire);
                    if dlg != 0 {
                        let done_txt = crate::strings::to_wide_z("Owner Completed");
                        let _ = crate::user32::message::SendMessageW(
                            dlg as HWND,
                            TDM_SET_ELEMENT_TEXT,
                            TDE_CONTENT,
                            done_txt.as_ptr() as isize,
                        );
                        let _ = crate::user32::message::SendMessageW(
                            dlg as HWND,
                            TDM_CLICK_BUTTON,
                            1,
                            0,
                        );
                    }
                    return 0;
                }
                crate::user32::defproc::DefWindowProcW(hwnd, msg, wp, lp)
            }

            let cls = crate::strings::to_wide_z("TaskDialogOwnerPumpWnd");
            let wc = crate::user32::class::WNDCLASS {
                style: 0,
                wndproc: crate::user32::register_typed_host_wndproc(owner_wndproc),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = crate::user32::class::RegisterClassW(&wc);
            let owner_hwnd = crate::user32::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                crate::user32::ws::POPUP,
                0,
                0,
                100,
                100,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!owner_hwnd.is_null());

            let dlg_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
                "#32770_TaskDialog",
                "Progress",
                owner_hwnd as usize,
                task_dialog_wndproc_va(),
            );
            let sid = NEXT_TASK_DIALOG_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            active_task_dialogs()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(
                    dlg_hwnd,
                    ActiveTaskDialog {
                        session_id: sid,
                        modal_open: true,
                        hinst: 0,
                        pf_callback: 0,
                        lp_callback_data: 0,
                        caption: "Progress".to_string(),
                        main_inst: String::new(),
                        content: "Working...".to_string(),
                        expanded: String::new(),
                        footer: String::new(),
                        buttons: vec![(1, "OK".to_string())],
                        button_enabled: vec![true],
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        has_verification: false,
                        verified: false,
                        pending_click_button: None,
                    },
                );
            let _dlg_guard = TaskDialogHwndGuard { hwnd: dlg_hwnd };
            OWNER_DIALOG_HWND.store(dlg_hwnd, std::sync::atomic::Ordering::Release);
            OWNER_TIMER_HITS.store(0, std::sync::atomic::Ordering::Release);

            // Start a fast owner-window timer to verify WM_TIMER fires while modal is open.
            assert_ne!(crate::user32::message::SetTimer(owner_hwnd, 1, 10, 0), 0);

            let (tx, rx) = std::sync::mpsc::channel::<crate::macos::TaskDialogEvent>();
            let tx_upd = tx.clone();
            let final_text = std::sync::Arc::new(Mutex::new(String::new()));
            let final_text_hook = final_text.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |u_sid, upd| {
                if u_sid == sid {
                    *final_text_hook.lock().unwrap() = upd.text.clone();
                    if let Some(bidx) = upd.click_button_idx {
                        let _ = tx_upd.send(crate::macos::TaskDialogEvent::ButtonClicked(
                            crate::macos::TaskDialogAlertResult {
                                button_index: bidx,
                                radio_index: None,
                                verified: false,
                            },
                        ));
                    }
                }
            })));

            // Background worker posts WM_WORK_DONE to the OWNER HWND (not dialog_hwnd).
            let owner_addr = owner_hwnd as usize;
            let worker = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(30));
                assert_eq!(
                    crate::user32::message::PostMessageW(owner_addr as HWND, WM_WORK_DONE, 0, 0),
                    TRUE
                );
            });

            let mut noop = |_ev: crate::macos::TaskDialogEvent| {};
            let mut noop_timer = |_ms: u32| false;
            let mut origin = std::time::Instant::now();
            let outcome = crate::macos::pump_task_dialog_wait_loop(
                dlg_hwnd,
                rx,
                &mut origin,
                None,
                &mut noop,
                &mut noop_timer,
                || {},
            );
            worker.join().unwrap();
            crate::macos::set_test_task_dialog_updater(None);
            OWNER_DIALOG_HWND.store(0, std::sync::atomic::Ordering::Release);
            let _ = crate::user32::message::KillTimer(owner_hwnd, 1);
            let _ = crate::user32::window::DestroyWindow(owner_hwnd);

            assert_eq!(
                outcome,
                crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                    button_index: 0,
                    radio_index: None,
                    verified: false,
                })
            );
            assert_eq!(final_text.lock().unwrap().as_str(), "Owner Completed");
            assert!(
                OWNER_TIMER_HITS.load(std::sync::atomic::Ordering::Acquire) >= 1,
                "owner-window WM_TIMER must be dispatched while modal TaskDialog waits"
            );
        })
        .join()
        .unwrap();
    }

    #[test]
    fn track_mouse_event_stores_queries_and_cancels_per_hwnd() {
        unsafe {
            let hwnd = 0x00AB_0010usize;

            // Register HOVER + LEAVE with HOVER_DEFAULT.
            let mut reg = make_tme(TME_HOVER | TME_LEAVE, hwnd, HOVER_DEFAULT);
            assert_eq!(_TrackMouseEvent(reg.as_mut_ptr()), TRUE);

            // TME_QUERY returns stored flags and resolved default hover time (400 ms).
            let mut q = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q[4..8].try_into().unwrap()), TME_HOVER | TME_LEAVE);
            assert_eq!(u32::from_le_bytes(q[16..20].try_into().unwrap()), DEFAULT_HOVER_MS);

            // Cancel TME_HOVER only; TME_LEAVE remains active.
            let mut cancel_hover = make_tme(TME_CANCEL | TME_HOVER, hwnd, 0);
            assert_eq!(_TrackMouseEvent(cancel_hover.as_mut_ptr()), TRUE);

            let mut q2 = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q2.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q2[4..8].try_into().unwrap()), TME_LEAVE);

            // Cancel remaining TME_LEAVE -> flags become 0.
            let mut cancel_leave = make_tme(TME_CANCEL | TME_LEAVE, hwnd, 0);
            assert_eq!(_TrackMouseEvent(cancel_leave.as_mut_ptr()), TRUE);

            let mut q3 = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q3.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q3[4..8].try_into().unwrap()), 0);

            // Undersized cbSize must fail.
            let mut bad = [0u8; 24];
            bad[0..4].copy_from_slice(&12u32.to_le_bytes());
            assert_eq!(_TrackMouseEvent(bad.as_mut_ptr()), FALSE);
        }
    }

    #[test]
    fn comctl32_subclass_ordinals_chaining_reentrancy_and_cleanup() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");

        // Verify ordinal resolution 410..=413 via comctl32.dll -> uxtheme.dll alias.
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 410).unwrap() as usize,
            SetWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 411).unwrap() as usize,
            GetWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 412).unwrap() as usize,
            RemoveWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 413).unwrap() as usize,
            DefSubclassProc as *const () as usize
        );

        static TRACE: Mutex<Vec<String>> = Mutex::new(Vec::new());
        const WM_PING: u32 = crate::user32::wm::USER + 1;
        const WM_REENTRANT: u32 = crate::user32::wm::USER + 2;
        const WM_REMOVE_SELF: u32 = crate::user32::wm::USER + 3;
        const WM_REMOVE_NEIGHBOR: u32 = crate::user32::wm::USER + 4;

        unsafe extern "win64" fn base_wndproc(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("base:{msg:#x}:{wp:#x}:{lp:#x}"));
            }
            if msg == WM_PING {
                return 100;
            }
            crate::user32::defproc::DefWindowProcW(hwnd, msg, wp, lp)
        }

        unsafe extern "win64" fn sub_a(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("A:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 1 } else { r }
        }

        unsafe extern "win64" fn sub_b(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("B:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            if msg == WM_REMOVE_SELF {
                assert_eq!(RemoveWindowSubclass(hwnd, sub_b as *const () as usize, uid), TRUE);
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 10 } else { r }
        }

        unsafe extern "win64" fn sub_c(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("C:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            if msg == WM_REENTRANT {
                // Nested SendMessage while inside SUBCLASSPROC C.
                let nested = crate::user32::message::SendMessageW(hwnd, WM_PING, 7, 9);
                assert_eq!(nested, 1111);
            }
            if msg == WM_REMOVE_NEIGHBOR {
                // Remove B while C is dispatching; DefSubclassProc must skip B and hit A.
                assert_eq!(RemoveWindowSubclass(hwnd, sub_b as *const () as usize, 20), TRUE);
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 1000 } else { r }
        }

        std::thread::spawn(|| unsafe {
            TRACE.lock().unwrap().clear();

            // Invalid / null HWND must fail with ERROR_INVALID_WINDOW_HANDLE.
            assert_eq!(SetWindowSubclass(std::ptr::null_mut(), sub_a as *const () as usize, 1, 0), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);
            assert_eq!(RemoveWindowSubclass(0xDEAD_BEEFusize as HWND, sub_a as *const () as usize, 1), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);
            let mut out_ref = 0x55usize;
            assert_eq!(GetWindowSubclass(std::ptr::null_mut(), sub_a as *const () as usize, 1, &mut out_ref), FALSE);
            assert_eq!(out_ref, 0);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);

            // Create a headless window.
            let cls = crate::strings::to_wide_z("ComctlSubclassTestWnd");
            let wc = crate::user32::class::WNDCLASS {
                style: 0,
                wndproc: crate::user32::register_typed_host_wndproc(base_wndproc),
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = crate::user32::class::RegisterClassW(&wc);
            let hwnd = crate::user32::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                crate::user32::ws::POPUP,
                0,
                0,
                100,
                100,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());

            // Stack A -> B -> C (LIFO: C runs first, then B, then A, then base_wndproc).
            assert_eq!(SetWindowSubclass(hwnd, sub_a as *const () as usize, 10, 0xAAA1), TRUE);
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB1), TRUE);
            assert_eq!(SetWindowSubclass(hwnd, sub_c as *const () as usize, 30, 0xCCC1), TRUE);

            // Update B's dwRefData in place without changing LIFO order.
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB2), TRUE);

            // GetWindowSubclass (including unaligned output pointer and null pointer check).
            let mut unaligned_buf = [0u8; 16];
            let unaligned_ptr = unaligned_buf.as_mut_ptr().add(3) as *mut usize;
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, unaligned_ptr), TRUE);
            assert_eq!(unaligned_ptr.read_unaligned(), 0xBBB2);
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), TRUE);
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 999, unaligned_ptr), FALSE);
            assert_eq!(unaligned_ptr.read_unaligned(), 0);

            // 1. LIFO chaining + 6-arg delivery + base_wndproc fallback.
            TRACE.lock().unwrap().clear();
            let res = crate::user32::message::SendMessageW(hwnd, WM_PING, 1, 2);
            assert_eq!(res, 1111); // 100 + 1 + 10 + 1000
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x401:id=30:ref=0xccc1",
                    "B:msg=0x401:id=20:ref=0xbbb2",
                    "A:msg=0x401:id=10:ref=0xaaa1",
                    "base:0x401:0x1:0x2",
                ]
            );

            // 2. Re-entrant SendMessage inside C.
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REENTRANT, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x402:id=30:ref=0xccc1",
                    "C:msg=0x401:id=30:ref=0xccc1",
                    "B:msg=0x401:id=20:ref=0xbbb2",
                    "A:msg=0x401:id=10:ref=0xaaa1",
                    "base:0x401:0x7:0x9",
                    "B:msg=0x402:id=20:ref=0xbbb2",
                    "A:msg=0x402:id=10:ref=0xaaa1",
                    "base:0x402:0x0:0x0",
                ]
            );

            // 3. Neighbor removal during dispatch (C removes B, then calls DefSubclassProc -> A).
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REMOVE_NEIGHBOR, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x404:id=30:ref=0xccc1",
                    "A:msg=0x404:id=10:ref=0xaaa1",
                    "base:0x404:0x0:0x0",
                ]
            );
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), FALSE);

            // 4. Re-add B and test self-removal during dispatch (B removes itself, then calls DefSubclassProc -> A).
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB3), TRUE);
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REMOVE_SELF, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "B:msg=0x403:id=20:ref=0xbbb3",
                    "C:msg=0x403:id=30:ref=0xccc1",
                    "A:msg=0x403:id=10:ref=0xaaa1",
                    "base:0x403:0x0:0x0",
                ]
            );
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), FALSE);

            // 5. WM_NCDESTROY is delivered through remaining chain (C -> A -> base) and cleans up table.
            TRACE.lock().unwrap().clear();
            assert_eq!(crate::user32::window::DestroyWindow(hwnd), TRUE);
            let tr = TRACE.lock().unwrap().clone();
            assert!(tr.iter().any(|s| s == "C:msg=0x82:id=30:ref=0xccc1"));
            assert!(tr.iter().any(|s| s == "A:msg=0x82:id=10:ref=0xaaa1"));
            assert!(tr.iter().any(|s| s == "base:0x82:0x0:0x0"));
            assert!(!subclass_table().lock().unwrap().contains_key(&(hwnd as usize)));
        })
        .join()
        .unwrap();
    }
}
