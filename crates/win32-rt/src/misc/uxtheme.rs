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

const TDF_ENABLE_HYPERLINKS: u32 = 0x0001;
const TDF_VERIFICATION_FLAG_CHECKED: u32 = 0x0100;
const TDF_SHOW_PROGRESS_BAR: u32 = 0x0200;
const TDF_SHOW_MARQUEE_PROGRESS_BAR: u32 = 0x0400;
const TDF_CALLBACK_TIMER: u32 = 0x0800;
const TDF_NO_DEFAULT_RADIO_BUTTON: u32 = 0x4000;

const TDN_CREATED: u32 = 0;
const TDN_NAVIGATED: u32 = 1;
const TDN_BUTTON_CLICKED: u32 = 2;
const TDN_HYPERLINK_CLICKED: u32 = 3;
const TDN_TIMER: u32 = 4;
const TDN_DESTROYED: u32 = 5;
const TDN_RADIO_BUTTON_CLICKED: u32 = 6;
const TDN_DIALOG_CONSTRUCTED: u32 = 7;
const TDN_VERIFICATION_CLICKED: u32 = 8;

/// Standard Windows TaskDialog `TDF_CALLBACK_TIMER` notification period (~200 ms).
const TASK_DIALOG_TIMER_INTERVAL: std::time::Duration = std::time::Duration::from_millis(200);

const TDM_NAVIGATE_PAGE: u32 = crate::user32::wm::USER + 101;
const TDM_CLICK_BUTTON: u32 = crate::user32::wm::USER + 102;
const TDM_SET_MARQUEE_PROGRESS_BAR: u32 = crate::user32::wm::USER + 103;
const TDM_SET_PROGRESS_BAR_STATE: u32 = crate::user32::wm::USER + 104;
const TDM_SET_PROGRESS_BAR_RANGE: u32 = crate::user32::wm::USER + 105;
const TDM_SET_PROGRESS_BAR_POS: u32 = crate::user32::wm::USER + 106;
const TDM_SET_PROGRESS_BAR_MARQUEE: u32 = crate::user32::wm::USER + 107;
const TDM_SET_ELEMENT_TEXT: u32 = crate::user32::wm::USER + 108;
const TDM_CLICK_RADIO_BUTTON: u32 = crate::user32::wm::USER + 110;
const TDM_ENABLE_BUTTON: u32 = crate::user32::wm::USER + 111;
const TDM_ENABLE_RADIO_BUTTON: u32 = crate::user32::wm::USER + 112;
const TDM_CLICK_VERIFICATION: u32 = crate::user32::wm::USER + 113;
const TDM_UPDATE_ELEMENT_TEXT: u32 = crate::user32::wm::USER + 114;

const PBST_NORMAL: u32 = 1;
const PBST_ERROR: u32 = 2;
const PBST_PAUSED: u32 = 3;

const TDE_CONTENT: usize = 0;
const TDE_EXPANDED_INFORMATION: usize = 1;
const TDE_FOOTER: usize = 2;
const TDE_MAIN_INSTRUCTION: usize = 3;

const S_FALSE_HR: HRESULT = 1;
const HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION: HRESULT = 0x8007_05B3u32 as i32;

static NEXT_TASK_DIALOG_SESSION: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(1);

struct ParsedTaskDialogPage {
    hwnd_parent: HWND,
    hinst: HINSTANCE,
    pf_callback: usize,
    lp_callback_data: isize,
    enable_hyperlinks: bool,
    callback_timer: bool,
    progress: Option<crate::macos::TaskDialogProgressState>,
    critical: bool,
    caption: String,
    main_inst: String,
    content: String,
    expanded: String,
    footer: String,
    buttons: Vec<(i32, String)>,
    default_button_idx: Option<usize>,
    radios: Vec<(i32, String)>,
    selected_radio_idx: Option<usize>,
    verification_text: Option<String>,
    verified: bool,
}

unsafe fn parse_task_dialog_config(config: *const u8) -> Result<ParsedTaskDialogPage, HRESULT> {
    if config.is_null() {
        return Err(E_INVALIDARG);
    }
    let cfg = (config as *const TaskDialogConfig).read_unaligned();
    if cfg.cb_size != std::mem::size_of::<TaskDialogConfig>() as u32 {
        return Err(E_INVALIDARG);
    }
    if (cfg.c_buttons > 0 && cfg.p_buttons.is_null())
        || (cfg.c_radio_buttons > 0 && cfg.p_radio_buttons.is_null())
    {
        return Err(E_INVALIDARG);
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
    let default_button_idx: Option<usize> = if cfg.n_default_button != 0 {
        buttons.iter().position(|(id, _)| *id == cfg.n_default_button)
    } else {
        None
    };
    let enable_hyperlinks = (cfg.dw_flags & TDF_ENABLE_HYPERLINKS) != 0;
    let callback_timer = (cfg.dw_flags & TDF_CALLBACK_TIMER) != 0 && cfg.pf_callback != 0;
    let show_progress =
        (cfg.dw_flags & (TDF_SHOW_PROGRESS_BAR | TDF_SHOW_MARQUEE_PROGRESS_BAR)) != 0;
    let progress = show_progress.then(|| crate::macos::TaskDialogProgressState {
        marquee: (cfg.dw_flags & TDF_SHOW_MARQUEE_PROGRESS_BAR) != 0,
        marquee_running: false,
        min: 0,
        max: 100,
        pos: 0,
        state: PBST_NORMAL,
    });

    Ok(ParsedTaskDialogPage {
        hwnd_parent: cfg.hwnd_parent,
        hinst,
        pf_callback: cfg.pf_callback,
        lp_callback_data: cfg.lp_callback_data,
        enable_hyperlinks,
        callback_timer,
        progress,
        critical,
        caption,
        main_inst,
        content,
        expanded,
        footer,
        buttons,
        default_button_idx,
        radios,
        selected_radio_idx,
        verification_text,
        verified,
    })
}

struct ActiveTaskDialog {
    session_id: u64,
    page_generation: u64,
    modal_open: bool,
    enable_hyperlinks: bool,
    callback_timer: bool,
    reset_timer_on_present: bool,
    critical: bool,
    progress: Option<crate::macos::TaskDialogProgressState>,
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
    default_button_idx: Option<usize>,
    radios: Vec<(i32, String)>,
    radio_enabled: Vec<bool>,
    selected_radio_idx: Option<usize>,
    verification_text: Option<String>,
    has_verification: bool,
    verified: bool,
    pending_click_button: Option<i32>,
}

/// Extract Windows TaskDialog `<A HREF="...">label</A>` anchor markup from a text
/// element, returning the stripped visible text and `(label, href)` pairs in order.
fn parse_task_dialog_hyperlinks(input: &str) -> (String, Vec<(String, String)>) {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut visible = String::with_capacity(len);
    let mut links: Vec<(String, String)> = Vec::new();
    let mut i = 0usize;

    while i < len {
        if bytes[i] == b'<'
            && i + 2 < len
            && bytes[i + 1].eq_ignore_ascii_case(&b'a')
            && (bytes[i + 2].is_ascii_whitespace() || bytes[i + 2] == b'>')
        {
            // Find the end of the opening `<a ...>` tag while respecting quoted attributes.
            let mut j = i + 2;
            let mut quote: u8 = 0;
            while j < len {
                let b = bytes[j];
                if quote != 0 {
                    if b == quote {
                        quote = 0;
                    }
                } else if b == b'"' || b == b'\'' {
                    quote = b;
                } else if b == b'>' {
                    break;
                }
                j += 1;
            }
            if j < len && bytes[j] == b'>' {
                let attrs = &input[i + 2..j];
                // Find the matching `</a>` closing tag (case-insensitive).
                let content_start = j + 1;
                let mut k = content_start;
                let mut close_end: Option<usize> = None;
                while k + 3 < len {
                    if bytes[k] == b'<'
                        && bytes[k + 1] == b'/'
                        && bytes[k + 2].eq_ignore_ascii_case(&b'a')
                    {
                        let mut m = k + 3;
                        while m < len && bytes[m].is_ascii_whitespace() {
                            m += 1;
                        }
                        if m < len && bytes[m] == b'>' {
                            close_end = Some(m + 1);
                            break;
                        }
                    }
                    k += 1;
                }
                if let Some(end_idx) = close_end {
                    let href = extract_href_attribute(attrs);
                    let label = &input[content_start..k];
                    visible.push_str(label);
                    let display_label = if label.trim().is_empty() {
                        href.clone()
                    } else {
                        label.to_string()
                    };
                    links.push((display_label, href));
                    i = end_idx;
                    continue;
                }
            }
        }
        let ch = input[i..].chars().next().unwrap();
        visible.push(ch);
        i += ch.len_utf8();
    }

    (visible, links)
}

fn extract_href_attribute(attrs: &str) -> String {
    let bytes = attrs.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    while i < len {
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let name_start = i;
        while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b'=' {
            i += 1;
        }
        let name = &attrs[name_start..i];
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < len && bytes[i] == b'=' {
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            let val = if i < len && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let q = bytes[i];
                i += 1;
                let val_start = i;
                while i < len && bytes[i] != q {
                    i += 1;
                }
                let s = &attrs[val_start..i];
                if i < len {
                    i += 1;
                }
                s
            } else {
                let val_start = i;
                while i < len && !bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                &attrs[val_start..i]
            };
            if name.eq_ignore_ascii_case("href") {
                return val.to_string();
            }
        }
    }
    String::new()
}

impl ActiveTaskDialog {
    fn from_page(session_id: u64, page: ParsedTaskDialogPage) -> Self {
        let button_enabled = vec![true; page.buttons.len()];
        let radio_enabled = vec![true; page.radios.len()];
        let has_verification = page.verification_text.is_some();
        Self {
            session_id,
            page_generation: 0,
            modal_open: false,
            enable_hyperlinks: page.enable_hyperlinks,
            callback_timer: page.callback_timer,
            reset_timer_on_present: false,
            critical: page.critical,
            progress: page.progress,
            hinst: page.hinst as usize,
            pf_callback: page.pf_callback,
            lp_callback_data: page.lp_callback_data,
            caption: page.caption,
            main_inst: page.main_inst,
            content: page.content,
            expanded: page.expanded,
            footer: page.footer,
            buttons: page.buttons,
            button_enabled,
            default_button_idx: page.default_button_idx,
            radios: page.radios,
            radio_enabled,
            selected_radio_idx: page.selected_radio_idx,
            verification_text: page.verification_text,
            has_verification,
            verified: page.verified,
            pending_click_button: None,
        }
    }

    fn apply_page(&mut self, page: ParsedTaskDialogPage) {
        self.page_generation = self.page_generation.wrapping_add(1);
        self.enable_hyperlinks = page.enable_hyperlinks;
        self.callback_timer = page.callback_timer;
        self.reset_timer_on_present = true;
        self.critical = page.critical;
        self.progress = page.progress;
        self.hinst = page.hinst as usize;
        self.pf_callback = page.pf_callback;
        self.lp_callback_data = page.lp_callback_data;
        self.caption = page.caption;
        self.main_inst = page.main_inst;
        self.content = page.content;
        self.expanded = page.expanded;
        self.footer = page.footer;
        self.button_enabled = vec![true; page.buttons.len()];
        self.buttons = page.buttons;
        self.default_button_idx = page.default_button_idx;
        self.radio_enabled = vec![true; page.radios.len()];
        self.radios = page.radios;
        self.selected_radio_idx = page.selected_radio_idx;
        self.has_verification = page.verification_text.is_some();
        self.verification_text = page.verification_text;
        self.verified = page.verified;
        self.pending_click_button = None;
    }

    fn compose_body_and_links(&self) -> (String, Vec<(String, String)>) {
        let mut body_parts: Vec<String> = Vec::new();
        let mut links: Vec<(String, String)> = Vec::new();
        if !self.main_inst.is_empty() {
            body_parts.push(self.main_inst.clone());
        }
        for part in [&self.content, &self.expanded, &self.footer] {
            if part.is_empty() {
                continue;
            }
            if self.enable_hyperlinks {
                let (plain, mut part_links) = parse_task_dialog_hyperlinks(part);
                links.append(&mut part_links);
                if !plain.is_empty() {
                    body_parts.push(plain);
                }
            } else {
                body_parts.push(part.clone());
            }
        }
        (body_parts.join("\n\n"), links)
    }

    fn live_update(&self, click_button_idx: Option<usize>) -> Option<(u64, crate::macos::TaskDialogLiveUpdate)> {
        self.modal_open.then(|| {
            let (text, links) = self.compose_body_and_links();
            (
                self.session_id,
                crate::macos::TaskDialogLiveUpdate {
                    caption: self.caption.clone(),
                    text,
                    links,
                    progress: self.progress,
                    button_enabled: self.button_enabled.clone(),
                    radio_enabled: self.radio_enabled.clone(),
                    selected_radio_idx: self.selected_radio_idx,
                    verification_checked: self.has_verification.then_some(self.verified),
                    click_button_idx,
                    navigate_page: false,
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
        TDM_NAVIGATE_PAGE => {
            let page = match parse_task_dialog_config(l_param as *const u8) {
                Ok(p) => p,
                Err(hr) => {
                    log::warn!(
                        "uxtheme: TDM_NAVIGATE_PAGE rejected invalid TASKDIALOGCONFIG (hr={hr:#x})"
                    );
                    return 0;
                }
            };
            let nav_action = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).map(|dlg| {
                    let old_sid = dlg.session_id;
                    let was_modal_open = dlg.modal_open;
                    let stop_old = if was_modal_open {
                        // Rotate session_id and close the live-update gate so any TDM_*
                        // messages sent during TDN_NAVIGATED mutate the new page state
                        // in memory rather than updating the dying NSAlert.
                        dlg.session_id = NEXT_TASK_DIALOG_SESSION
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        dlg.modal_open = false;
                        Some((
                            old_sid,
                            crate::macos::TaskDialogLiveUpdate {
                                caption: page.caption.clone(),
                                text: String::new(),
                                links: Vec::new(),
                                progress: None,
                                button_enabled: Vec::new(),
                                radio_enabled: Vec::new(),
                                selected_radio_idx: None,
                                verification_checked: None,
                                click_button_idx: None,
                                navigate_page: true,
                            },
                        ))
                    } else {
                        None
                    };
                    dlg.apply_page(page);
                    (dlg.caption.clone(), dlg.pf_callback, dlg.lp_callback_data, stop_old)
                })
            };
            if let Some((new_caption, new_cb, new_ref_data, stop_old)) = nav_action {
                let wide_cap = crate::strings::to_wide_z(&new_caption);
                let _ = crate::user32::defproc::DefWindowProcW(
                    hwnd,
                    crate::user32::wm::SETTEXT,
                    0,
                    wide_cap.as_ptr() as isize,
                );
                if let Some((old_sid, upd)) = stop_old {
                    crate::macos::task_dialog_alert_update(old_sid, upd);
                }
                if new_cb != 0 {
                    let _ = crate::call_guest8(
                        new_cb,
                        h as u64,
                        TDN_NAVIGATED as u64,
                        0,
                        0,
                        new_ref_data as u64,
                        0,
                        0,
                        0,
                    );
                }
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
        TDM_SET_MARQUEE_PROGRESS_BAR => {
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    if let Some(ref mut p) = dlg.progress {
                        p.marquee = w_param != 0;
                        if !p.marquee {
                            p.marquee_running = false;
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
        TDM_SET_PROGRESS_BAR_STATE => {
            let (prev, push) = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h)
                    .map(|dlg| {
                        if let Some(ref mut p) = dlg.progress {
                            let st = w_param as u32;
                            if matches!(st, PBST_NORMAL | PBST_ERROR | PBST_PAUSED) {
                                let old = p.state as isize;
                                p.state = st;
                                (old, dlg.live_update(None))
                            } else {
                                (0, None)
                            }
                        } else {
                            (0, None)
                        }
                    })
                    .unwrap_or((0, None))
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            prev
        }
        TDM_SET_PROGRESS_BAR_RANGE => {
            let (prev, push) = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h)
                    .map(|dlg| {
                        if let Some(ref mut p) = dlg.progress {
                            let old = ((p.min as usize) | ((p.max as usize) << 16)) as isize;
                            let raw = l_param as usize;
                            let lo = (raw & 0xFFFF) as u16;
                            let hi = ((raw >> 16) & 0xFFFF) as u16;
                            p.min = lo;
                            p.max = hi;
                            p.pos = p.pos.clamp(lo as i32, hi.max(lo) as i32);
                            (old, dlg.live_update(None))
                        } else {
                            (0, None)
                        }
                    })
                    .unwrap_or((0, None))
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            prev
        }
        TDM_SET_PROGRESS_BAR_POS => {
            let (prev, push) = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h)
                    .map(|dlg| {
                        if let Some(ref mut p) = dlg.progress {
                            let old = p.pos as isize;
                            let hi = p.max.max(p.min) as i32;
                            p.pos = (w_param as i32).clamp(p.min as i32, hi);
                            (old, dlg.live_update(None))
                        } else {
                            (0, None)
                        }
                    })
                    .unwrap_or((0, None))
            };
            if let Some((sid, upd)) = push {
                crate::macos::task_dialog_alert_update(sid, upd);
            }
            prev
        }
        TDM_SET_PROGRESS_BAR_MARQUEE => {
            let push = {
                let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                map.get_mut(&h).and_then(|dlg| {
                    if let Some(ref mut p) = dlg.progress {
                        p.marquee = true;
                        p.marquee_running = w_param != 0;
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
    let initial_page = match parse_task_dialog_config(config) {
        Ok(p) => p,
        Err(hr) => return hr,
    };

    // Allocate a dialog-owned Win32 HWND (distinct from cfg.hwnd_parent, non-NULL even for
    // unparented dialogs) so callbacks using SetWindowText, SetWindowSubclass, or TDM_*
    // messages operate on the TaskDialog window rather than mutating the parent window.
    let session_id = NEXT_TASK_DIALOG_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dialog_hwnd = crate::user32::window::create_virtual_dialog_hwnd(
        "#32770_TaskDialog",
        &initial_page.caption,
        initial_page.hwnd_parent as usize,
        task_dialog_wndproc_va(),
    );
    active_task_dialogs()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(dialog_hwnd, ActiveTaskDialog::from_page(session_id, initial_page));
    let _dialog_guard = TaskDialogHwndGuard { hwnd: dialog_hwnd };

    let notify_lp = |msg: u32, wp: u64, lp: u64| -> HRESULT {
        let cb_info = {
            let map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
            map.get(&dialog_hwnd)
                .map(|d| (d.pf_callback, d.lp_callback_data))
                .unwrap_or((0, 0))
        };
        if cb_info.0 != 0 {
            crate::call_guest8(
                cb_info.0,
                dialog_hwnd as u64,
                msg as u64,
                wp,
                lp,
                cb_info.1 as u64,
                0,
                0,
                0,
            ) as HRESULT
        } else {
            S_OK
        }
    };
    let notify = |msg: u32, wp: u64| -> HRESULT { notify_lp(msg, wp, 0) };

    let mut timer_origin = std::time::Instant::now();

    let _ = notify(TDN_DIALOG_CONSTRUCTED, 0);
    let _ = notify(TDN_CREATED, 0);

    let chosen_id = loop {
        struct PageSnapshot {
            session_id: u64,
            page_generation: u64,
            caption: String,
            body: String,
            links: Vec<(String, String)>,
            progress: Option<crate::macos::TaskDialogProgressState>,
            buttons: Vec<(i32, String)>,
            button_enabled: Vec<bool>,
            default_button_idx: Option<usize>,
            radio_labels: Vec<String>,
            radio_enabled: Vec<bool>,
            selected_radio_idx: Option<usize>,
            verification_text: Option<String>,
            verified: bool,
            critical: bool,
            callback_timer: bool,
            reset_timer: bool,
            programmatic_click: Option<i32>,
        }
        let snap = {
            let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
            let dlg = map.get_mut(&dialog_hwnd).expect("active task dialog missing");
            let (body, links) = dlg.compose_body_and_links();
            let reset_timer = std::mem::take(&mut dlg.reset_timer_on_present);
            let programmatic_click = dlg.pending_click_button.take();
            if programmatic_click.is_none() {
                dlg.modal_open = true;
            }
            PageSnapshot {
                session_id: dlg.session_id,
                page_generation: dlg.page_generation,
                caption: dlg.caption.clone(),
                body,
                links,
                progress: dlg.progress,
                buttons: dlg.buttons.clone(),
                button_enabled: dlg.button_enabled.clone(),
                default_button_idx: dlg.default_button_idx,
                radio_labels: dlg.radios.iter().map(|(_, l)| l.clone()).collect(),
                radio_enabled: dlg.radio_enabled.clone(),
                selected_radio_idx: dlg.selected_radio_idx,
                verification_text: dlg.verification_text.clone(),
                verified: dlg.verified,
                critical: dlg.critical,
                callback_timer: dlg.callback_timer,
                reset_timer,
                programmatic_click,
            }
        };

        if snap.reset_timer {
            timer_origin = std::time::Instant::now();
        }

        if let Some(bid) = snap.programmatic_click {
            let cb_hr = notify(TDN_BUTTON_CLICKED, bid as u64);
            if cb_hr != S_FALSE_HR {
                break bid;
            }
            continue;
        }

        let labels: Vec<String> = snap.buttons.iter().map(|(_, l)| l.clone()).collect();
        let has_selection_controls =
            !snap.radio_labels.is_empty() || snap.verification_text.is_some();
        let timer_interval = snap.callback_timer.then_some(TASK_DIALOG_TIMER_INTERVAL);
        let presented_gen = snap.page_generation;

        let presented = crate::macos::task_dialog_alert(
            snap.session_id,
            dialog_hwnd,
            snap.caption,
            snap.body,
            snap.links.clone(),
            snap.progress,
            labels,
            snap.button_enabled.clone(),
            snap.default_button_idx,
            snap.radio_labels,
            snap.radio_enabled,
            snap.selected_radio_idx,
            snap.verification_text,
            snap.verified,
            snap.critical,
            &mut timer_origin,
            timer_interval,
            |ev| match ev {
                crate::macos::TaskDialogEvent::RadioSelected(ridx) => {
                    let rid_opt = {
                        let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                        map.get_mut(&dialog_hwnd).and_then(|dlg| {
                            if dlg.page_generation != presented_gen {
                                return None;
                            }
                            dlg.selected_radio_idx = Some(ridx);
                            dlg.radios.get(ridx).map(|(rid, _)| *rid)
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
                            if dlg.page_generation != presented_gen || !dlg.has_verification {
                                return false;
                            }
                            dlg.verified = checked;
                            true
                        })
                    };
                    if has_ver {
                        let _ = notify(TDN_VERIFICATION_CLICKED, if checked { 1 } else { 0 });
                    }
                }
                crate::macos::TaskDialogEvent::HyperlinkClicked(href) => {
                    let same_page = {
                        let map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
                        map.get(&dialog_hwnd)
                            .is_some_and(|dlg| dlg.page_generation == presented_gen)
                    };
                    if same_page {
                        let wide = crate::strings::to_wide_z(&href);
                        let _ = notify_lp(TDN_HYPERLINK_CLICKED, 0, wide.as_ptr() as u64);
                    }
                }
                crate::macos::TaskDialogEvent::ButtonClicked(_)
                | crate::macos::TaskDialogEvent::Navigated => {}
            },
            |elapsed_ms| notify(TDN_TIMER, elapsed_ms as u64) == S_FALSE_HR,
        );
        // Consume any programmatic TDM_CLICK_BUTTON triggered while the native modal was open
        // so it is delivered exactly once and never replayed if TDN_BUTTON_CLICKED returns S_FALSE.
        // If TDM_NAVIGATE_PAGE superseded this page while the modal was open, preserve the new
        // page's state and loop immediately to present it.
        let (navigated_while_open, consumed_prog_click) = {
            let mut map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
            map.get_mut(&dialog_hwnd)
                .map(|dlg| {
                    let navigated = dlg.page_generation != presented_gen;
                    dlg.modal_open = false;
                    if !navigated {
                        if let crate::macos::TaskDialogOutcome::Completed(ref res) = presented {
                            dlg.selected_radio_idx = res.radio_index;
                            if dlg.has_verification {
                                dlg.verified = res.verified;
                            }
                        }
                        (false, dlg.pending_click_button.take())
                    } else {
                        (true, None)
                    }
                })
                .unwrap_or((false, None))
        };
        if matches!(presented, crate::macos::TaskDialogOutcome::Quit) {
            // WM_QUIT arrived on the owning thread: exit the modal loop immediately
            // without synthesizing a button click, leaving the re-posted WM_QUIT
            // intact for the application's outer message loop.
            break 0;
        }
        if matches!(presented, crate::macos::TaskDialogOutcome::Navigated) || navigated_while_open {
            continue;
        }
        let (bid, interactive) = match presented {
            crate::macos::TaskDialogOutcome::Completed(res) => {
                let bid = consumed_prog_click.unwrap_or_else(|| {
                    snap.buttons
                        .get(res.button_index)
                        .map(|(id, _)| *id)
                        .unwrap_or(snap.buttons[0].0)
                });
                (bid, true)
            }
            crate::macos::TaskDialogOutcome::Navigated | crate::macos::TaskDialogOutcome::Quit => {
                unreachable!()
            }
            crate::macos::TaskDialogOutcome::Unavailable => {
                let single_enabled = snap.button_enabled.first().copied().unwrap_or(true);
                if snap.buttons.len() == 1
                    && !has_selection_controls
                    && !snap.callback_timer
                    && snap.links.is_empty()
                    && snap.progress.is_none()
                    && single_enabled
                {
                    // Pure single-button informational dialog with an enabled button and no
                    // selection controls, hyperlinks, progress bar, or timer can dismiss in
                    // headless mode (matching MessageBox MB_OK).
                    (snap.buttons[0].0, false)
                } else {
                    // Multi-button prompts or dialogs with radio buttons / verification checkboxes /
                    // clickable hyperlinks / progress bars / TDF_CALLBACK_TIMER require an interactive
                    // desktop so we never fabricate user selections or silently skip live UI updates.
                    if snap.callback_timer {
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

    let (selected_radio, final_verified) = {
        let map = active_task_dialogs().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&dialog_hwnd)
            .map(|d| {
                let rid = d
                    .selected_radio_idx
                    .and_then(|idx| d.radios.get(idx))
                    .map(|(id, _)| *id)
                    .unwrap_or(0);
                (rid, d.verified)
            })
            .unwrap_or((0, false))
    };

    let _ = notify(TDN_DESTROYED, 0);

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
                move |_cap, body, _links, _prog, btns, _btn_en, def_btn, rads, _rad_en, def_rad, ver_txt, ver_chk, emit| {
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
                |cap, body, _links, _prog, _btns, _btn_en, _def_btn, _rads, _rad_en, def_rad, _ver_txt, ver_chk, _emit| {
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
                move |_cap, _body, _links, _prog, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, emit| {
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
                        page_generation: 0,
                        modal_open: true,
                        enable_hyperlinks: false,
                        callback_timer: false,
                        reset_timer_on_present: false,
                        critical: false,
                        progress: None,
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
                        default_button_idx: None,
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        verification_text: None,
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
                        page_generation: 0,
                        modal_open: true,
                        enable_hyperlinks: false,
                        callback_timer: false,
                        reset_timer_on_present: false,
                        critical: false,
                        progress: None,
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
                        default_button_idx: None,
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        verification_text: None,
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
                |_cap, _body, _links, _prog, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, _emit| {
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
                move |_cap, _body, _links, _prog, btns, btn_en, _db, rads, rad_en, _dr, _vt, _vc, emit| {
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
    fn task_dialog_hyperlinks_strip_markup_and_dispatch_tdn_hyperlink_clicked() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());

        // 1. Parser handles double-quoted, single-quoted, extra-attribute, mixed-case,
        //    empty-label, and multiple anchor tags while preserving malformed tags literally.
        let (vis, parsed) = parse_task_dialog_hyperlinks(
            "See <A ID=\"help\" HREF=\"https://example.com/docs\">the docs</A> or <a href='mailto:a@b.c'>email us</a> and <a href=\"https://bare.test\"></a> (<abbr>ok</abbr>).",
        );
        assert_eq!(vis, "See the docs or email us and  (<abbr>ok</abbr>).");
        assert_eq!(
            parsed,
            vec![
                ("the docs".to_string(), "https://example.com/docs".to_string()),
                ("email us".to_string(), "mailto:a@b.c".to_string()),
                ("https://bare.test".to_string(), "https://bare.test".to_string()),
            ]
        );

        std::thread::spawn(|| unsafe {
            static CLICKED_HREFS: Mutex<Vec<String>> = Mutex::new(Vec::new());
            unsafe extern "win64" fn link_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                if msg == TDN_HYPERLINK_CLICKED {
                    assert_eq!(wp, 0);
                    let href = if lp != 0 {
                        crate::strings::from_wide(lp as LPCWSTR)
                    } else {
                        String::new()
                    };
                    CLICKED_HREFS.lock().unwrap().push(href.clone());
                    if href == "https://example.com/docs" {
                        let next_footer = crate::strings::to_wide_z(
                            "Updated <a href=\"https://example.com/changelog\">Changelog</a>",
                        );
                        let _ = crate::user32::message::SendMessageW(
                            hwnd,
                            TDM_SET_ELEMENT_TEXT,
                            TDE_FOOTER,
                            next_footer.as_ptr() as isize,
                        );
                    }
                }
                S_OK
            }

            let title_w = crate::strings::to_wide_z("Help");
            let main_w = crate::strings::to_wide_z("Heading <a href=\"ignored\">Raw</a>");
            let content_w = crate::strings::to_wide_z(
                "Read <A HREF=\"https://example.com/docs\">Documentation</A>.",
            );
            let expanded_w = crate::strings::to_wide_z(
                "Extra <a href='https://example.com/faq'>FAQ</a>.",
            );
            let footer_w = crate::strings::to_wide_z(
                "Support: <a href=\"https://example.com/support\">Contact</a>",
            );

            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.dw_flags = TDF_ENABLE_HYPERLINKS;
            cfg.dw_common_buttons = TDCBF_CLOSE_BUTTON;
            cfg.psz_window_title = title_w.as_ptr();
            cfg.psz_main_instruction = main_w.as_ptr();
            cfg.psz_content = content_w.as_ptr();
            cfg.psz_expanded_information = expanded_w.as_ptr();
            cfg.psz_footer = footer_w.as_ptr();
            cfg.pf_callback = link_cb as *const () as usize;

            // 2. Interactive dialog strips markup in content/expanded/footer (leaving main_instruction
            //    verbatim), surfaces extracted links, delivers TDN_HYPERLINK_CLICKED with UTF-16 href,
            //    and pushes live updated links on TDM_SET_ELEMENT_TEXT.
            CLICKED_HREFS.lock().unwrap().clear();
            let live_links: std::sync::Arc<Mutex<Vec<Vec<(String, String)>>>> =
                std::sync::Arc::new(Mutex::new(Vec::new()));
            let live_links_hook = live_links.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |_sid, upd| {
                live_links_hook.lock().unwrap().push(upd.links.clone());
            })));
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, body, links, _prog, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, emit| {
                    assert_eq!(
                        body,
                        "Heading <a href=\"ignored\">Raw</a>\n\nRead Documentation.\n\nExtra FAQ.\n\nSupport: Contact"
                    );
                    assert_eq!(
                        links,
                        &[
                            ("Documentation".to_string(), "https://example.com/docs".to_string()),
                            ("FAQ".to_string(), "https://example.com/faq".to_string()),
                            ("Contact".to_string(), "https://example.com/support".to_string()),
                        ]
                    );
                    emit(crate::macos::TaskDialogEvent::HyperlinkClicked(
                        "https://example.com/docs".to_string(),
                    ));
                    emit(crate::macos::TaskDialogEvent::HyperlinkClicked(
                        "https://example.com/changelog".to_string(),
                    ));
                    crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                        button_index: 0,
                        radio_index: None,
                        verified: false,
                    })
                },
            )));

            let mut btn = 0;
            let hr = TaskDialogIndirect(
                &cfg as *const TaskDialogConfig as *const u8,
                &mut btn,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            crate::macos::set_test_task_dialog_presenter(None);
            crate::macos::set_test_task_dialog_updater(None);
            assert_eq!(hr, S_OK);
            assert_eq!(btn, 8);
            assert_eq!(
                CLICKED_HREFS.lock().unwrap().clone(),
                vec![
                    "https://example.com/docs".to_string(),
                    "https://example.com/changelog".to_string(),
                ]
            );
            let pushed = live_links.lock().unwrap().clone();
            assert!(
                pushed.iter().any(|ls| ls.iter().any(|(l, u)| l == "Changelog" && u == "https://example.com/changelog")),
                "TDM_SET_ELEMENT_TEXT on a hyperlink-enabled dialog must push updated links: {pushed:?}"
            );

            // 3. Headless single-button dialog with TDF_ENABLE_HYPERLINKS and active links must
            //    refuse auto-dismissal rather than dropping clickable links.
            let mut headless_btn = 99;
            assert_eq!(
                TaskDialogIndirect(
                    &cfg as *const TaskDialogConfig as *const u8,
                    &mut headless_btn,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION
            );
            assert_eq!(headless_btn, 0);

            // 4. Without TDF_ENABLE_HYPERLINKS, anchor tags remain literal and no links are extracted.
            cfg.dw_flags = 0;
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, body, links, _prog, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, _emit| {
                    assert!(body.contains("<A HREF=\"https://example.com/docs\">Documentation</A>"));
                    assert!(links.is_empty());
                    crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                        button_index: 0,
                        radio_index: None,
                        verified: false,
                    })
                },
            )));
            assert_eq!(
                TaskDialogIndirect(
                    &cfg as *const TaskDialogConfig as *const u8,
                    &mut btn,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                S_OK
            );
            crate::macos::set_test_task_dialog_presenter(None);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn task_dialog_progress_bar_updates_and_headless_refusal() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            // 1. Determinate progress bar: TDN_CREATED sets range/pos/state (verifying previous
            //    return values), and live interaction switches to marquee and animates it.
            unsafe extern "win64" fn prog_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                _ref_data: isize,
            ) -> HRESULT {
                if msg == TDN_CREATED {
                    let prev_range = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_PROGRESS_BAR_RANGE,
                        0,
                        ((10usize) | (200usize << 16)) as isize,
                    );
                    assert_eq!(prev_range as usize, 100usize << 16); // default [0, 100]
                    let prev_pos = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_PROGRESS_BAR_POS,
                        75,
                        0,
                    );
                    assert_eq!(prev_pos, 10); // clamped from 0 -> 10 when range changed to [10, 200]
                    let prev_state = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_PROGRESS_BAR_STATE,
                        PBST_PAUSED as usize,
                        0,
                    );
                    assert_eq!(prev_state, PBST_NORMAL as isize);
                } else if msg == TDN_VERIFICATION_CLICKED && wp != 0 {
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_SET_MARQUEE_PROGRESS_BAR, 1, 0);
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_SET_PROGRESS_BAR_MARQUEE, 1, 30);
                }
                S_OK
            }

            let title_w = crate::strings::to_wide_z("Downloading");
            let ver_w = crate::strings::to_wide_z("Switch to marquee");
            let mut cfg: TaskDialogConfig = std::mem::zeroed();
            cfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg.dw_flags = TDF_SHOW_PROGRESS_BAR;
            cfg.dw_common_buttons = TDCBF_CANCEL_BUTTON;
            cfg.psz_window_title = title_w.as_ptr();
            cfg.psz_verification_text = ver_w.as_ptr();
            cfg.pf_callback = prog_cb as *const () as usize;

            let live_progs: std::sync::Arc<Mutex<Vec<Option<crate::macos::TaskDialogProgressState>>>> =
                std::sync::Arc::new(Mutex::new(Vec::new()));
            let live_progs_hook = live_progs.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |_sid, upd| {
                live_progs_hook.lock().unwrap().push(upd.progress);
            })));
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |_cap, _body, _links, prog, _btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, emit| {
                    assert_eq!(
                        prog,
                        Some(crate::macos::TaskDialogProgressState {
                            marquee: false,
                            marquee_running: false,
                            min: 10,
                            max: 200,
                            pos: 75,
                            state: PBST_PAUSED,
                        })
                    );
                    emit(crate::macos::TaskDialogEvent::VerificationToggled(true));
                    crate::macos::TaskDialogOutcome::Completed(crate::macos::TaskDialogAlertResult {
                        button_index: 0,
                        radio_index: None,
                        verified: true,
                    })
                },
            )));

            let mut btn = 0;
            let mut ver = FALSE;
            assert_eq!(
                TaskDialogIndirect(
                    &cfg as *const TaskDialogConfig as *const u8,
                    &mut btn,
                    std::ptr::null_mut(),
                    &mut ver,
                ),
                S_OK
            );
            crate::macos::set_test_task_dialog_presenter(None);
            crate::macos::set_test_task_dialog_updater(None);
            assert_eq!(btn, 2); // IDCANCEL
            assert_eq!(ver, TRUE);
            let pushed = live_progs.lock().unwrap().clone();
            assert!(
                pushed.iter().any(|p| *p == Some(crate::macos::TaskDialogProgressState {
                    marquee: true,
                    marquee_running: true,
                    min: 10,
                    max: 200,
                    pos: 75,
                    state: PBST_PAUSED,
                })),
                "live TDM_SET_MARQUEE_PROGRESS_BAR/TDM_SET_PROGRESS_BAR_MARQUEE must push marquee state: {pushed:?}"
            );

            // 2. Headless single-button dialog with TDF_SHOW_PROGRESS_BAR or
            //    TDF_SHOW_MARQUEE_PROGRESS_BAR (without TDF_CALLBACK_TIMER) must NOT
            //    auto-dismiss — returns HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION.
            for flag in [TDF_SHOW_PROGRESS_BAR, TDF_SHOW_MARQUEE_PROGRESS_BAR] {
                let mut hcfg: TaskDialogConfig = std::mem::zeroed();
                hcfg.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
                hcfg.dw_flags = flag;
                hcfg.dw_common_buttons = TDCBF_CANCEL_BUTTON;
                hcfg.psz_window_title = title_w.as_ptr();
                let mut hbtn = 99;
                assert_eq!(
                    TaskDialogIndirect(
                        &hcfg as *const TaskDialogConfig as *const u8,
                        &mut hbtn,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    ),
                    HRESULT_REQUIRES_INTERACTIVE_WINDOWSTATION
                );
                assert_eq!(hbtn, 0);
            }
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
                        page_generation: 0,
                        modal_open: true,
                        enable_hyperlinks: false,
                        callback_timer: false,
                        reset_timer_on_present: false,
                        critical: false,
                        progress: None,
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
                        default_button_idx: None,
                        radios: Vec::new(),
                        radio_enabled: Vec::new(),
                        selected_radio_idx: None,
                        verification_text: None,
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
    fn task_dialog_navigate_page_rebuilds_wizard_and_mid_modal_pages() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            static EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
            static PAGE2_CFG_PTR: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);
            static PAGE3_CFG_PTR: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);

            unsafe extern "win64" fn page1_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                ref_data: isize,
            ) -> HRESULT {
                EVENTS.lock().unwrap().push(format!("p1:{msg}:{wp}:{ref_data:#x}"));
                if msg == TDN_CREATED {
                    // Invalid configs (null / wrong cbSize) must be rejected without
                    // firing TDN_NAVIGATED or corrupting the current page.
                    let _ = crate::user32::message::SendMessageW(hwnd, TDM_NAVIGATE_PAGE, 0, 0);
                    let mut bad: TaskDialogConfig = std::mem::zeroed();
                    bad.cb_size = 16;
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_NAVIGATE_PAGE,
                        0,
                        &bad as *const TaskDialogConfig as isize,
                    );
                } else if msg == TDN_BUTTON_CLICKED && wp == 1001 {
                    let p2 = PAGE2_CFG_PTR.load(std::sync::atomic::Ordering::Acquire);
                    assert_ne!(p2, 0);
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_NAVIGATE_PAGE,
                        0,
                        p2 as isize,
                    );
                    return S_FALSE_HR;
                }
                S_OK
            }

            unsafe extern "win64" fn page2_cb(
                hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                ref_data: isize,
            ) -> HRESULT {
                EVENTS.lock().unwrap().push(format!("p2:{msg}:{wp}:{ref_data:#x}"));
                if msg == TDN_NAVIGATED {
                    let mut title_buf = [0u16; 64];
                    let n = crate::user32::window::GetWindowTextW(
                        hwnd,
                        title_buf.as_mut_ptr(),
                        title_buf.len() as i32,
                    );
                    assert_eq!(String::from_utf16_lossy(&title_buf[..n as usize]), "Step 2");
                    let footer = crate::strings::to_wide_z("Navigated Footer");
                    let _ = crate::user32::message::SendMessageW(
                        hwnd,
                        TDM_SET_ELEMENT_TEXT,
                        TDE_FOOTER,
                        footer.as_ptr() as isize,
                    );
                } else if msg == TDN_HYPERLINK_CLICKED {
                    let p3 = PAGE3_CFG_PTR.load(std::sync::atomic::Ordering::Acquire);
                    if p3 != 0 {
                        let _ = crate::user32::message::SendMessageW(
                            hwnd,
                            TDM_NAVIGATE_PAGE,
                            0,
                            p3 as isize,
                        );
                    }
                }
                S_OK
            }

            unsafe extern "win64" fn page3_cb(
                _hwnd: HWND,
                msg: u32,
                wp: usize,
                _lp: isize,
                ref_data: isize,
            ) -> HRESULT {
                EVENTS.lock().unwrap().push(format!("p3:{msg}:{wp}:{ref_data:#x}"));
                S_OK
            }

            let title1 = crate::strings::to_wide_z("Step 1");
            let content1 = crate::strings::to_wide_z("Welcome");
            let next_lbl = crate::strings::to_wide_z("&Next");
            let btns1 = [TaskDialogButton {
                button_id: 1001,
                button_text: next_lbl.as_ptr(),
            }];

            let title2 = crate::strings::to_wide_z("Step 2");
            let content2 = crate::strings::to_wide_z(
                "Details <a href=\"https://example.test/jump\">Jump</a>",
            );
            let finish_lbl = crate::strings::to_wide_z("&Finish");
            let btns2 = [TaskDialogButton {
                button_id: 2001,
                button_text: finish_lbl.as_ptr(),
            }];
            let rad_a = crate::strings::to_wide_z("Alpha");
            let rad_b = crate::strings::to_wide_z("Beta");
            let radios2 = [
                TaskDialogButton { button_id: 301, button_text: rad_a.as_ptr() },
                TaskDialogButton { button_id: 302, button_text: rad_b.as_ptr() },
            ];
            let ver2 = crate::strings::to_wide_z("Launch now");

            let title3 = crate::strings::to_wide_z("Step 3");
            let content3 = crate::strings::to_wide_z("Final page");
            let done_lbl = crate::strings::to_wide_z("Done");
            let btns3 = [TaskDialogButton {
                button_id: 3001,
                button_text: done_lbl.as_ptr(),
            }];

            let mut cfg2: TaskDialogConfig = std::mem::zeroed();
            cfg2.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg2.dw_flags = TDF_ENABLE_HYPERLINKS;
            cfg2.dw_common_buttons = TDCBF_CLOSE_BUTTON;
            cfg2.psz_window_title = title2.as_ptr();
            cfg2.psz_content = content2.as_ptr();
            cfg2.c_buttons = btns2.len() as u32;
            cfg2.p_buttons = btns2.as_ptr();
            cfg2.n_default_button = 2001;
            cfg2.c_radio_buttons = radios2.len() as u32;
            cfg2.p_radio_buttons = radios2.as_ptr();
            cfg2.n_default_radio_button = 302;
            cfg2.psz_verification_text = ver2.as_ptr();
            cfg2.pf_callback = page2_cb as *const () as usize;
            cfg2.lp_callback_data = 0x2222;

            let mut cfg3: TaskDialogConfig = std::mem::zeroed();
            cfg3.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg3.psz_window_title = title3.as_ptr();
            cfg3.psz_content = content3.as_ptr();
            cfg3.c_buttons = btns3.len() as u32;
            cfg3.p_buttons = btns3.as_ptr();
            cfg3.pf_callback = page3_cb as *const () as usize;
            cfg3.lp_callback_data = 0x3333;

            let mut cfg1: TaskDialogConfig = std::mem::zeroed();
            cfg1.cb_size = std::mem::size_of::<TaskDialogConfig>() as u32;
            cfg1.dw_common_buttons = TDCBF_CANCEL_BUTTON;
            cfg1.psz_window_title = title1.as_ptr();
            cfg1.psz_content = content1.as_ptr();
            cfg1.c_buttons = btns1.len() as u32;
            cfg1.p_buttons = btns1.as_ptr();
            cfg1.pf_callback = page1_cb as *const () as usize;
            cfg1.lp_callback_data = 0x1111;

            // 1. Standard wizard transition from TDN_BUTTON_CLICKED (Page 1 -> Page 2).
            EVENTS.lock().unwrap().clear();
            PAGE2_CFG_PTR.store(
                &cfg2 as *const TaskDialogConfig as usize,
                std::sync::atomic::Ordering::Release,
            );
            PAGE3_CFG_PTR.store(0, std::sync::atomic::Ordering::Release);

            let stage = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let stage_hook = stage.clone();
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |cap, body, links, _prog, btns, _btn_en, def_btn, rads, _rad_en, def_rad, vt, vc, emit| {
                    let cur = stage_hook.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    if cur == 0 {
                        assert_eq!(cap, "Step 1");
                        assert_eq!(body, "Welcome");
                        assert_eq!(btns, &["Next".to_string(), "Cancel".to_string()]);
                        assert!(rads.is_empty());
                        assert_eq!(vt, None);
                        crate::macos::TaskDialogOutcome::Completed(
                            crate::macos::TaskDialogAlertResult {
                                button_index: 0, // ID 1001 ("Next")
                                radio_index: None,
                                verified: false,
                            },
                        )
                    } else {
                        assert_eq!(cur, 1);
                        assert_eq!(cap, "Step 2");
                        assert_eq!(body, "Details Jump\n\nNavigated Footer");
                        assert_eq!(
                            links,
                            &[("Jump".to_string(), "https://example.test/jump".to_string())]
                        );
                        assert_eq!(btns, &["Finish".to_string(), "Close".to_string()]);
                        assert_eq!(def_btn, Some(0));
                        assert_eq!(rads, &["Alpha".to_string(), "Beta".to_string()]);
                        assert_eq!(def_rad, Some(1));
                        assert_eq!(vt, Some("Launch now"));
                        assert!(!vc);
                        emit(crate::macos::TaskDialogEvent::RadioSelected(0));
                        emit(crate::macos::TaskDialogEvent::VerificationToggled(true));
                        crate::macos::TaskDialogOutcome::Completed(
                            crate::macos::TaskDialogAlertResult {
                                button_index: 0, // ID 2001 ("Finish")
                                radio_index: Some(0),
                                verified: true,
                            },
                        )
                    }
                },
            )));

            let mut btn = 0;
            let mut rad = 0;
            let mut ver = FALSE;
            let hr = TaskDialogIndirect(
                &cfg1 as *const TaskDialogConfig as *const u8,
                &mut btn,
                &mut rad,
                &mut ver,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            assert_eq!(hr, S_OK);
            assert_eq!(btn, 2001);
            assert_eq!(rad, 301);
            assert_eq!(ver, TRUE);
            assert_eq!(
                EVENTS.lock().unwrap().clone(),
                vec![
                    format!("p1:{TDN_DIALOG_CONSTRUCTED}:0:0x1111"),
                    format!("p1:{TDN_CREATED}:0:0x1111"),
                    format!("p1:{TDN_BUTTON_CLICKED}:1001:0x1111"),
                    format!("p2:{TDN_NAVIGATED}:0:0x2222"),
                    format!("p2:{TDN_RADIO_BUTTON_CLICKED}:301:0x2222"),
                    format!("p2:{TDN_VERIFICATION_CLICKED}:1:0x2222"),
                    format!("p2:{TDN_BUTTON_CLICKED}:2001:0x2222"),
                    format!("p2:{TDN_DESTROYED}:0:0x2222"),
                ]
            );

            // 2. Mid-modal navigation (TDN_HYPERLINK_CLICKED triggers TDM_NAVIGATE_PAGE
            //    while Page 2's modal is open): must emit navigate_page live update and
            //    immediately re-present Page 3.
            EVENTS.lock().unwrap().clear();
            PAGE3_CFG_PTR.store(
                &cfg3 as *const TaskDialogConfig as usize,
                std::sync::atomic::Ordering::Release,
            );
            let saw_nav_update = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let saw_nav_hook = saw_nav_update.clone();
            crate::macos::set_test_task_dialog_updater(Some(Box::new(move |_sid, upd| {
                if upd.navigate_page {
                    saw_nav_hook.store(true, std::sync::atomic::Ordering::Release);
                }
            })));
            let stage2 = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let stage2_hook = stage2.clone();
            crate::macos::set_test_task_dialog_presenter(Some(Box::new(
                move |cap, body, _links, _prog, btns, _btn_en, _db, _rads, _rad_en, _dr, _vt, _vc, emit| {
                    let cur = stage2_hook.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    match cur {
                        0 => {
                            assert_eq!(cap, "Step 1");
                            crate::macos::TaskDialogOutcome::Completed(
                                crate::macos::TaskDialogAlertResult {
                                    button_index: 0,
                                    radio_index: None,
                                    verified: false,
                                },
                            )
                        }
                        1 => {
                            assert_eq!(cap, "Step 2");
                            emit(crate::macos::TaskDialogEvent::HyperlinkClicked(
                                "https://example.test/jump".to_string(),
                            ));
                            crate::macos::TaskDialogOutcome::Navigated
                        }
                        _ => {
                            assert_eq!(cur, 2);
                            assert_eq!(cap, "Step 3");
                            assert_eq!(body, "Final page");
                            assert_eq!(btns, &["Done".to_string()]);
                            crate::macos::TaskDialogOutcome::Completed(
                                crate::macos::TaskDialogAlertResult {
                                    button_index: 0, // ID 3001 ("Done")
                                    radio_index: None,
                                    verified: false,
                                },
                            )
                        }
                    }
                },
            )));

            let mut btn3 = 0;
            let mut rad3 = 99;
            let mut ver3 = TRUE;
            let hr2 = TaskDialogIndirect(
                &cfg1 as *const TaskDialogConfig as *const u8,
                &mut btn3,
                &mut rad3,
                &mut ver3,
            );
            crate::macos::set_test_task_dialog_presenter(None);
            crate::macos::set_test_task_dialog_updater(None);
            PAGE2_CFG_PTR.store(0, std::sync::atomic::Ordering::Release);
            PAGE3_CFG_PTR.store(0, std::sync::atomic::Ordering::Release);
            assert_eq!(hr2, S_OK);
            assert!(
                saw_nav_update.load(std::sync::atomic::Ordering::Acquire),
                "mid-modal TDM_NAVIGATE_PAGE must signal navigate_page to tear down the active alert"
            );
            assert_eq!(btn3, 3001);
            assert_eq!(rad3, 0);
            assert_eq!(ver3, FALSE);
            assert_eq!(
                EVENTS.lock().unwrap().clone(),
                vec![
                    format!("p1:{TDN_DIALOG_CONSTRUCTED}:0:0x1111"),
                    format!("p1:{TDN_CREATED}:0:0x1111"),
                    format!("p1:{TDN_BUTTON_CLICKED}:1001:0x1111"),
                    format!("p2:{TDN_NAVIGATED}:0:0x2222"),
                    format!("p2:{TDN_HYPERLINK_CLICKED}:0:0x2222"),
                    format!("p3:{TDN_NAVIGATED}:0:0x3333"),
                    format!("p3:{TDN_BUTTON_CLICKED}:3001:0x3333"),
                    format!("p3:{TDN_DESTROYED}:0:0x3333"),
                ]
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
