//! user32 odds and ends: rectangles, system metrics, SystemParametersInfo,
//! window stations, device/hotkey registration, clipboard, dialogs,
//! DPI-awareness switches, MessageBox.

use super::*;
use crate::types::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Rectangles
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn SetRect(r: *mut RECT, l: i32, t: i32, rt: i32, b: i32) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    *r = RECT { left: l, top: t, right: rt, bottom: b };
    TRUE
}

pub unsafe extern "win64" fn SetRectEmpty(r: *mut RECT) -> BOOL {
    SetRect(r, 0, 0, 0, 0)
}

pub unsafe extern "win64" fn IsRectEmpty(r: *const RECT) -> BOOL {
    if r.is_null() {
        return TRUE;
    }
    let r = &*r;
    (r.right <= r.left || r.bottom <= r.top) as BOOL
}

pub unsafe extern "win64" fn CopyRect(d: *mut RECT, s: *const RECT) -> BOOL {
    if d.is_null() || s.is_null() {
        return FALSE;
    }
    *d = *s;
    TRUE
}

pub unsafe extern "win64" fn OffsetRect(r: *mut RECT, dx: i32, dy: i32) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    *r = (*r).offset(dx, dy);
    TRUE
}

pub unsafe extern "win64" fn InflateRect(r: *mut RECT, dx: i32, dy: i32) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    let v = &mut *r;
    v.left -= dx;
    v.top -= dy;
    v.right += dx;
    v.bottom += dy;
    TRUE
}

pub unsafe extern "win64" fn IntersectRect(d: *mut RECT, a: *const RECT, b: *const RECT) -> BOOL {
    if d.is_null() || a.is_null() || b.is_null() {
        return FALSE;
    }
    match (*a).intersect(&*b) {
        Some(r) => {
            *d = r;
            TRUE
        }
        None => {
            *d = RECT::default();
            FALSE
        }
    }
}

pub unsafe extern "win64" fn UnionRect(d: *mut RECT, a: *const RECT, b: *const RECT) -> BOOL {
    if d.is_null() || a.is_null() || b.is_null() {
        return FALSE;
    }
    let (a, b) = (*a, *b);
    let ea = IsRectEmpty(&a) != 0;
    let eb = IsRectEmpty(&b) != 0;
    *d = match (ea, eb) {
        (true, true) => RECT::default(),
        (true, false) => b,
        (false, true) => a,
        (false, false) => RECT::new(a.left.min(b.left), a.top.min(b.top), a.right.max(b.right), a.bottom.max(b.bottom)),
    };
    (IsRectEmpty(d) == 0) as BOOL
}

pub unsafe extern "win64" fn EqualRect(a: *const RECT, b: *const RECT) -> BOOL {
    if a.is_null() || b.is_null() {
        return FALSE;
    }
    (*a == *b) as BOOL
}

/// `POINT` passed by value in one register.
pub unsafe extern "win64" fn PtInRect(r: *const RECT, p: u64) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    (*r).contains(p as u32 as i32, (p >> 32) as u32 as i32) as BOOL
}

pub unsafe extern "win64" fn FillRect(_hdc: usize, r: *const RECT, _hbr: usize) -> i32 {
    if r.is_null() {
        return 0;
    }
    1
}

pub unsafe extern "win64" fn DrawFocusRect(_hdc: usize, r: *const RECT) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetSysColor(index: i32) -> u32 {
    match index {
        0 => 0x00C8_C8C8,  // COLOR_SCROLLBAR
        1 => 0x0000_0000,  // COLOR_BACKGROUND
        5 => 0x00FF_FFFF,  // COLOR_WINDOW
        8 => 0x0000_0000,  // COLOR_WINDOWTEXT
        15 => 0x00F0_F0F0, // COLOR_BTNFACE
        18 => 0x0000_0000, // COLOR_BTNTEXT
        _ => 0,
    }
}

pub unsafe extern "win64" fn GetSysColorBrush(index: i32) -> usize {
    if !(0..=30).contains(&index) {
        return 0;
    }
    // WHITE_BRUSH (0) from gdi32 stock objects
    crate::misc::gdi32::GetStockObject(0)
}

pub unsafe extern "win64" fn RegisterWindowMessageW(lp_string: LPCWSTR) -> u32 {
    if lp_string.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    crate::kernel32::process::GlobalAddAtomW(lp_string) as u32
}

pub unsafe extern "win64" fn RegisterWindowMessageA(lp_string: LPCSTR) -> u32 {
    if lp_string.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    crate::kernel32::process::GlobalAddAtomA(lp_string) as u32
}

pub unsafe extern "win64" fn SendInput(c_inputs: u32, p_inputs: *const u8, cb_size: i32) -> u32 {
    // sizeof(INPUT) on x64 is 40 bytes.
    if c_inputs == 0 || p_inputs.is_null() || cb_size != 40 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    log::debug!("SendInput({c_inputs} inputs): accepted");
    c_inputs
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Bounding rect of all monitors.
pub fn virtual_screen() -> RECT {
    let ms = crate::macos::display::monitors();
    let mut r = ms[0].rect;
    for m in &ms[1..] {
        r = RECT::new(r.left.min(m.rect.left), r.top.min(m.rect.top), r.right.max(m.rect.right), r.bottom.max(m.rect.bottom));
    }
    r
}

/// Screen metrics come from the display cache (Win32 pixels); the fixed
/// ones are Windows 10 defaults at 96 DPI. Unknown indices return 0 (as
/// Windows does) and are logged as STUB.
pub unsafe extern "win64" fn GetSystemMetrics(index: i32) -> i32 {
    let p = super::monitor::primary_rect();
    let work = super::monitor::primary_work_area();
    let v = virtual_screen();
    let cap = window::nc_insets(ws::CAPTION, 0).1;
    match index {
        0 => p.width(),                                                   // SM_CXSCREEN
        1 => p.height(),                                                  // SM_CYSCREEN
        16 => work.width(),                                               // SM_CXFULLSCREEN
        17 => work.height() - cap,                                        // SM_CYFULLSCREEN
        61 => work.width(),                                               // SM_CXMAXIMIZED
        62 => work.height(),                                              // SM_CYMAXIMIZED
        59 | 60 => if index == 59 { v.width() + 16 } else { v.height() + 16 }, // SM_CXMAXTRACK
        76 => v.left,                                                     // SM_XVIRTUALSCREEN
        77 => v.top,                                                      // SM_YVIRTUALSCREEN
        78 => v.width(),                                                  // SM_CXVIRTUALSCREEN
        79 => v.height(),                                                 // SM_CYVIRTUALSCREEN
        80 => crate::macos::display::monitors().len() as i32,             // SM_CMONITORS
        81 => 1,                                                          // SM_SAMEDISPLAYFORMAT
        2 | 3 => 17,                                                      // SM_CXVSCROLL, SM_CYHSCROLL
        4 => cap,                                                         // SM_CYCAPTION
        5 | 6 => 1,                                                       // SM_CXBORDER, SM_CYBORDER
        7 | 8 => 3,                                                       // SM_CXDLGFRAME, SM_CYDLGFRAME
        11 | 12 => 32,                                                    // SM_CXICON, SM_CYICON
        13 | 14 => 32,                                                    // SM_CXCURSOR, SM_CYCURSOR
        15 => 20,                                                         // SM_CYMENU
        19 => 1,                                                          // SM_MOUSEPRESENT
        23 => 0,                                                          // SM_SWAPBUTTON
        28 | 29 => 136,                                                   // SM_CXMIN, SM_CYMIN
        32 | 33 => 4,                                                     // SM_CXFRAME, SM_CYFRAME
        34 => 136,                                                        // SM_CXMINTRACK
        35 => 39,                                                         // SM_CYMINTRACK
        36 | 37 => 4,                                                     // SM_CXDOUBLECLK, SM_CYDOUBLECLK
        43 => 5,                                                          // SM_CMOUSEBUTTONS
        49 | 50 => 16,                                                    // SM_CXSMICON, SM_CYSMICON
        63 => 0x1,                                                        // SM_NETWORK
        67 => 0,                                                          // SM_CLEANBOOT
        68 | 69 => 4,                                                     // SM_CXDRAG, SM_CYDRAG
        75 => 1,                                                          // SM_MOUSEWHEELPRESENT
        86 => 0,                                                          // SM_TABLETPC
        87 => 0,                                                          // SM_MEDIACENTER
        92 => 0,                                                          // SM_CXPADDEDBORDER
        94 => 0,                                                          // SM_DIGITIZER
        95 => 0,                                                          // SM_MAXIMUMTOUCHES
        0x1000 => 0,                                                      // SM_REMOTESESSION
        0x2001 => 0,                                                      // SM_REMOTECONTROL
        0x2003 => 1,                                                      // SM_CONVERTIBLESLATEMODE (1 = not slate)
        0x2004 => 0,                                                      // SM_SYSTEMDOCKED
        _ => {
            stub!("GetSystemMetrics({index}) -> 0");
            0
        }
    }
}

pub unsafe extern "win64" fn GetSystemMetricsForDpi(index: i32, dpi: u32) -> i32 {
    let base = GetSystemMetrics(index);
    if dpi < 48 || dpi == 96 {
        return base;
    }
    // Scale UI element dimensions linearly with DPI; screen pixel metrics stay unscaled.
    match index {
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 11 | 12 | 13 | 14 | 15 | 28 | 29 | 32 | 33 | 34 | 35 | 49 | 50 => {
            let scaled = (base as i64 * dpi as i64 + 48) / 96;
            scaled as i32
        }
        _ => base,
    }
}

// ---------------------------------------------------------------------------
// SystemParametersInfo
// ---------------------------------------------------------------------------

const SPI_GETWORKAREA: u32 = 0x0030;
const SPI_GETFILTERKEYS: u32 = 0x0032;
const SPI_SETFILTERKEYS: u32 = 0x0033;
const SPI_GETTOGGLEKEYS: u32 = 0x0034;
const SPI_SETTOGGLEKEYS: u32 = 0x0035;
const SPI_GETSTICKYKEYS: u32 = 0x003A;
const SPI_SETSTICKYKEYS: u32 = 0x003B;
const SPI_GETSCREENSAVEACTIVE: u32 = 0x0010;
const SPI_SETSCREENSAVEACTIVE: u32 = 0x0011;
const SPI_GETKEYBOARDDELAY: u32 = 0x0016;
const SPI_GETKEYBOARDSPEED: u32 = 0x000A;
const SPI_GETMOUSE: u32 = 0x0003;
const SPI_GETMOUSESPEED: u32 = 0x0070;
const SPI_GETWHEELSCROLLLINES: u32 = 0x0068;
const SPI_GETWHEELSCROLLCHARS: u32 = 0x006C;
const SPI_GETHIGHCONTRAST: u32 = 0x0042;
const SPI_GETNONCLIENTMETRICS: u32 = 0x0029;
const SPI_GETFONTSMOOTHING: u32 = 0x004A;
const SPI_GETSCREENREADER: u32 = 0x0046;
const SPI_GETCLIENTAREAANIMATION: u32 = 0x1042;
const SPI_GETANIMATION: u32 = 0x0048;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LogFontA {
    pub lf_height: i32,
    pub lf_width: i32,
    pub lf_escapement: i32,
    pub lf_orientation: i32,
    pub lf_weight: i32,
    pub lf_italic: u8,
    pub lf_underline: u8,
    pub lf_strike_out: u8,
    pub lf_char_set: u8,
    pub lf_out_precision: u8,
    pub lf_clip_precision: u8,
    pub lf_quality: u8,
    pub lf_pitch_and_family: u8,
    pub lf_face_name: [u8; 32],
}

impl LogFontA {
    fn segoe_ui_9pt() -> Self {
        let mut lf_face_name = [0u8; 32];
        let name = b"Segoe UI\0";
        lf_face_name[..name.len()].copy_from_slice(name);
        Self {
            lf_height: -12,
            lf_width: 0,
            lf_escapement: 0,
            lf_orientation: 0,
            lf_weight: 400,
            lf_italic: 0,
            lf_underline: 0,
            lf_strike_out: 0,
            lf_char_set: 1, // DEFAULT_CHARSET
            lf_out_precision: 0,
            lf_clip_precision: 0,
            lf_quality: 0,
            lf_pitch_and_family: 0,
            lf_face_name,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct LogFontW {
    pub lf_height: i32,
    pub lf_width: i32,
    pub lf_escapement: i32,
    pub lf_orientation: i32,
    pub lf_weight: i32,
    pub lf_italic: u8,
    pub lf_underline: u8,
    pub lf_strike_out: u8,
    pub lf_char_set: u8,
    pub lf_out_precision: u8,
    pub lf_clip_precision: u8,
    pub lf_quality: u8,
    pub lf_pitch_and_family: u8,
    pub lf_face_name: [u16; 32],
}

impl LogFontW {
    fn segoe_ui_9pt() -> Self {
        let mut lf_face_name = [0u16; 32];
        for (i, u) in "Segoe UI".encode_utf16().enumerate() {
            lf_face_name[i] = u;
        }
        Self {
            lf_height: -12,
            lf_width: 0,
            lf_escapement: 0,
            lf_orientation: 0,
            lf_weight: 400,
            lf_italic: 0,
            lf_underline: 0,
            lf_strike_out: 0,
            lf_char_set: 1, // DEFAULT_CHARSET
            lf_out_precision: 0,
            lf_clip_precision: 0,
            lf_quality: 0,
            lf_pitch_and_family: 0,
            lf_face_name,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct NonClientMetricsA {
    pub cb_size: u32,
    pub i_border_width: i32,
    pub i_scroll_width: i32,
    pub i_scroll_height: i32,
    pub i_caption_width: i32,
    pub i_caption_height: i32,
    pub lf_caption_font: LogFontA,
    pub i_sm_caption_width: i32,
    pub i_sm_caption_height: i32,
    pub lf_sm_caption_font: LogFontA,
    pub i_menu_width: i32,
    pub i_menu_height: i32,
    pub lf_menu_font: LogFontA,
    pub lf_status_font: LogFontA,
    pub lf_message_font: LogFontA,
    pub i_padded_border_width: i32,
}

impl NonClientMetricsA {
    fn windows_default(cb_size: u32, caption_height: i32) -> Self {
        let font = LogFontA::segoe_ui_9pt();
        Self {
            cb_size,
            i_border_width: 1,
            i_scroll_width: 17,
            i_scroll_height: 17,
            i_caption_width: 36,
            i_caption_height: caption_height,
            lf_caption_font: font,
            i_sm_caption_width: 22,
            i_sm_caption_height: 22,
            lf_sm_caption_font: font,
            i_menu_width: 19,
            i_menu_height: 19,
            lf_menu_font: font,
            lf_status_font: font,
            lf_message_font: font,
            i_padded_border_width: 4,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct NonClientMetricsW {
    pub cb_size: u32,
    pub i_border_width: i32,
    pub i_scroll_width: i32,
    pub i_scroll_height: i32,
    pub i_caption_width: i32,
    pub i_caption_height: i32,
    pub lf_caption_font: LogFontW,
    pub i_sm_caption_width: i32,
    pub i_sm_caption_height: i32,
    pub lf_sm_caption_font: LogFontW,
    pub i_menu_width: i32,
    pub i_menu_height: i32,
    pub lf_menu_font: LogFontW,
    pub lf_status_font: LogFontW,
    pub lf_message_font: LogFontW,
    pub i_padded_border_width: i32,
}

impl NonClientMetricsW {
    fn windows_default(cb_size: u32, caption_height: i32) -> Self {
        let font = LogFontW::segoe_ui_9pt();
        Self {
            cb_size,
            i_border_width: 1,
            i_scroll_width: 17,
            i_scroll_height: 17,
            i_caption_width: 36,
            i_caption_height: caption_height,
            lf_caption_font: font,
            i_sm_caption_width: 22,
            i_sm_caption_height: 22,
            lf_sm_caption_font: font,
            i_menu_width: 19,
            i_menu_height: 19,
            lf_menu_font: font,
            lf_status_font: font,
            lf_message_font: font,
            i_padded_border_width: 4,
        }
    }
}

unsafe fn spi(action: u32, uparam: u32, pv: *mut u8, wide: bool) -> BOOL {
    let put_u32 = |v: u32| {
        if pv.is_null() {
            return FALSE;
        }
        *(pv as *mut u32) = v;
        TRUE
    };
    match action {
        SPI_GETWORKAREA => {
            if pv.is_null() {
                return FALSE;
            }
            *(pv as *mut RECT) = super::monitor::primary_work_area();
            TRUE
        }
        // Accessibility structs: { UINT cbSize; DWORD dwFlags; ... }. Report
        // all features off; accept SET (games disable the shortcuts while
        // running and restore them on exit).
        SPI_GETSTICKYKEYS | SPI_GETTOGGLEKEYS | SPI_GETFILTERKEYS => {
            if pv.is_null() {
                return FALSE;
            }
            let size = match action {
                SPI_GETFILTERKEYS => 24,
                _ => 8,
            };
            std::ptr::write_bytes(pv.add(4), 0, size - 4);
            *(pv as *mut u32) = size as u32;
            TRUE
        }
        SPI_SETSTICKYKEYS | SPI_SETTOGGLEKEYS | SPI_SETFILTERKEYS => {
            log::debug!("SystemParametersInfo set accessibility {action:#x}: accepted (no system effect)");
            TRUE
        }
        SPI_GETSCREENSAVEACTIVE => put_u32(0),
        SPI_SETSCREENSAVEACTIVE => TRUE,
        SPI_GETKEYBOARDDELAY => put_u32(1),
        SPI_GETKEYBOARDSPEED => put_u32(31),
        SPI_GETMOUSE => {
            if pv.is_null() {
                return FALSE;
            }
            // thresholds 6/10, acceleration 1 (Windows default)
            let v = pv as *mut i32;
            *v = 6;
            *v.add(1) = 10;
            *v.add(2) = 1;
            TRUE
        }
        SPI_GETMOUSESPEED => put_u32(10),
        SPI_GETWHEELSCROLLLINES => put_u32(3),
        SPI_GETWHEELSCROLLCHARS => put_u32(3),
        SPI_GETFONTSMOOTHING => put_u32(1),
        SPI_GETSCREENREADER => put_u32(0),
        SPI_GETCLIENTAREAANIMATION => put_u32(1),
        SPI_GETHIGHCONTRAST => {
            if pv.is_null() {
                return FALSE;
            }
            // HIGHCONTRAST { UINT cbSize; DWORD dwFlags; LPTSTR lpszDefaultScheme; }
            *(pv.add(4) as *mut u32) = 0;
            *(pv.add(8) as *mut usize) = 0;
            TRUE
        }
        SPI_GETANIMATION => {
            if pv.is_null() {
                return FALSE;
            }
            *(pv.add(4) as *mut i32) = 1;
            TRUE
        }
        SPI_GETNONCLIENTMETRICS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            // Documented Win32 layouts:
            //   NONCLIENTMETRICSA: 340 bytes (344 with iPaddedBorderWidth)
            //   NONCLIENTMETRICSW: 500 bytes (504 with iPaddedBorderWidth)
            let base_sz: usize = if wide { 500 } else { 340 };
            let padded_sz = base_sz + 4;
            let cb = std::ptr::read_unaligned(pv as *const u32) as usize;
            if cb < base_sz {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let write_len = cb.min(padded_sz);
            let cap = super::window::nc_insets(ws::CAPTION, 0).1;
            if wide {
                let ncm = NonClientMetricsW::windows_default(cb as u32, cap);
                std::ptr::copy_nonoverlapping(
                    &ncm as *const NonClientMetricsW as *const u8,
                    pv,
                    write_len,
                );
            } else {
                let ncm = NonClientMetricsA::windows_default(cb as u32, cap);
                std::ptr::copy_nonoverlapping(
                    &ncm as *const NonClientMetricsA as *const u8,
                    pv,
                    write_len,
                );
            }
            TRUE
        }
        _ => {
            stub!("SystemParametersInfo{}({action:#x}, {uparam}) not supported -> FALSE", if wide { "W" } else { "A" });
            set_error(e::CALL_NOT_IMPLEMENTED);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SystemParametersInfoA(action: u32, uparam: u32, pv: *mut u8, _winini: u32) -> BOOL {
    spi(action, uparam, pv, false)
}

pub unsafe extern "win64" fn SystemParametersInfoW(action: u32, uparam: u32, pv: *mut u8, _winini: u32) -> BOOL {
    spi(action, uparam, pv, true)
}

pub unsafe extern "win64" fn SystemParametersInfoForDpi(action: u32, uparam: u32, pv: *mut u8, _winini: u32, _dpi: u32) -> BOOL {
    spi(action, uparam, pv, true)
}

// ---------------------------------------------------------------------------
// Window station
// ---------------------------------------------------------------------------

const WINSTA_HANDLE: usize = 0x0000_00A0;
const UOI_FLAGS: i32 = 1;
const UOI_NAME: i32 = 2;
const UOI_TYPE: i32 = 3;
const WSF_VISIBLE: u32 = 1;

pub unsafe extern "win64" fn GetProcessWindowStation() -> usize {
    WINSTA_HANDLE
}

unsafe fn user_object_info(h: usize, index: i32, pv: *mut u8, len: u32, needed: *mut u32, wide: bool) -> BOOL {
    if h != WINSTA_HANDLE {
        stub!("GetUserObjectInformation on handle {h:#x} (only the window station is known)");
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let s = match index {
        UOI_FLAGS => {
            // USEROBJECTFLAGS { BOOL fInherit; BOOL fReserved; DWORD dwFlags; }
            if !needed.is_null() {
                *needed = 12;
            }
            if len < 12 || pv.is_null() {
                set_error(e::INSUFFICIENT_BUFFER);
                return FALSE;
            }
            *(pv as *mut u32) = 0;
            *(pv.add(4) as *mut u32) = 0;
            *(pv.add(8) as *mut u32) = WSF_VISIBLE;
            return TRUE;
        }
        UOI_NAME => "WinSta0",
        UOI_TYPE => "WindowStation",
        _ => {
            stub!("GetUserObjectInformation index {index} not supported");
            set_error(e::INVALID_PARAMETER);
            return FALSE;
        }
    };
    let bytes = if wide { (s.len() + 1) * 2 } else { s.len() + 1 } as u32;
    if !needed.is_null() {
        *needed = bytes;
    }
    if len < bytes || pv.is_null() {
        set_error(e::INSUFFICIENT_BUFFER);
        return FALSE;
    }
    if wide {
        crate::strings::write_wide_trunc(s, pv as *mut u16, (len / 2) as usize);
    } else {
        crate::strings::write_ansi_trunc(s, pv, len as usize);
    }
    TRUE
}

pub unsafe extern "win64" fn GetUserObjectInformationW(h: usize, index: i32, pv: *mut u8, len: u32, needed: *mut u32) -> BOOL {
    user_object_info(h, index, pv, len, needed, true)
}

pub unsafe extern "win64" fn GetUserObjectInformationA(h: usize, index: i32, pv: *mut u8, len: u32, needed: *mut u32) -> BOOL {
    user_object_info(h, index, pv, len, needed, false)
}

// ---------------------------------------------------------------------------
// Device notifications, hotkeys
// ---------------------------------------------------------------------------

pub const DBT_DEVNODES_CHANGED: usize = 0x0007;
pub const DBT_DEVICEARRIVAL: usize = 0x8000;
pub const DBT_DEVICEREMOVECOMPLETE: usize = 0x8004;
const DBT_DEVTYP_DEVICEINTERFACE: u32 = 5;
const DEVICE_NOTIFY_SERVICE_HANDLE: u32 = 0x0000_0001;

#[repr(C)]
struct DevBroadcastDeviceInterfaceW {
    dbcc_size: u32,
    dbcc_devicetype: u32,
    dbcc_reserved: u32,
    dbcc_classguid: [u8; 16],
    dbcc_name: [u16; 1],
}

/// Permanently readable `DEV_BROADCAST_DEVICEINTERFACE_W` (`GUID_DEVINTERFACE_HID`)
/// passed as `lParam` for asynchronous `WM_DEVICECHANGE` arrival/removal posts.
static HID_BROADCAST_IFACE: DevBroadcastDeviceInterfaceW = DevBroadcastDeviceInterfaceW {
    dbcc_size: std::mem::size_of::<DevBroadcastDeviceInterfaceW>() as u32,
    dbcc_devicetype: DBT_DEVTYP_DEVICEINTERFACE,
    dbcc_reserved: 0,
    // {4D1E55B2-F16F-11CF-88CB-001111000030} (GUID_DEVINTERFACE_HID)
    dbcc_classguid: [
        0xB2, 0x55, 0x1E, 0x4D, 0x6F, 0xF1, 0xCF, 0x11, 0x88, 0xCB, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30,
    ],
    dbcc_name: [0],
};

#[derive(Clone, Copy)]
struct DevNotifyReg {
    handle: usize,
    hwnd: usize,
}

static NEXT_DEVNOTIFY: AtomicUsize = AtomicUsize::new(0x0000_D000);
static DEV_NOTIFIES: std::sync::Mutex<Vec<DevNotifyReg>> = std::sync::Mutex::new(Vec::new());

/// Remove any `RegisterDeviceNotification` entries for a destroyed window.
pub(crate) fn forget_device_notifications(hwnd: usize) {
    DEV_NOTIFIES.lock().unwrap().retain(|r| r.hwnd != hwnd);
}

/// Broadcast `WM_DEVICECHANGE` when a HID or XInput controller connects or
/// disconnects. Safe to call from any thread (including the macOS/iOS main
/// thread — never touches the Win32 TEB).
pub fn notify_device_change(arrived: bool) {
    let registered_hwnds: Vec<usize> = {
        let regs = DEV_NOTIFIES.lock().unwrap();
        let mut out = Vec::with_capacity(regs.len());
        for r in regs.iter() {
            if !out.contains(&r.hwnd) {
                out.push(r.hwnd);
            }
        }
        out
    };
    let top_hwnds = super::window::top_level_windows();
    if registered_hwnds.is_empty() && top_hwnds.is_empty() {
        return;
    }

    let iface_evt = if arrived { DBT_DEVICEARRIVAL } else { DBT_DEVICEREMOVECOMPLETE };
    let iface_ptr = &HID_BROADCAST_IFACE as *const DevBroadcastDeviceInterfaceW as LPARAM;
    for hwnd in registered_hwnds {
        super::message::post_message(hwnd, super::wm::DEVICECHANGE, iface_evt, iface_ptr);
    }
    for hwnd in top_hwnds {
        super::message::post_message(hwnd, super::wm::DEVICECHANGE, DBT_DEVNODES_CHANGED, 0);
    }
}

pub unsafe extern "win64" fn RegisterDeviceNotificationW(recipient: usize, filter: *const u32, flags: u32) -> usize {
    let ty = if filter.is_null() { 0 } else { std::ptr::read_unaligned(filter.add(1)) };
    let handle = NEXT_DEVNOTIFY.fetch_add(4, Ordering::Relaxed);
    if recipient != 0 && (flags & DEVICE_NOTIFY_SERVICE_HANDLE) == 0 {
        DEV_NOTIFIES.lock().unwrap().push(DevNotifyReg { handle, hwnd: recipient });
        log::info!(
            "user32: RegisterDeviceNotificationW(hwnd={recipient:#x}, devtype={ty}, flags={flags:#x}) -> {handle:#x}"
        );
    } else {
        log::debug!(
            "user32: RegisterDeviceNotificationW(recipient={recipient:#x}, devtype={ty}, flags={flags:#x}) -> {handle:#x}"
        );
    }
    handle
}

pub unsafe extern "win64" fn RegisterDeviceNotificationA(recipient: usize, filter: *const u32, flags: u32) -> usize {
    RegisterDeviceNotificationW(recipient, filter, flags)
}

pub unsafe extern "win64" fn UnregisterDeviceNotification(h: usize) -> BOOL {
    let mut regs = DEV_NOTIFIES.lock().unwrap();
    let before = regs.len();
    regs.retain(|r| r.handle != h);
    if regs.len() != before {
        log::debug!("user32: UnregisterDeviceNotification({h:#x})");
    }
    TRUE
}

/// Global hotkeys are not delivered (macOS has no equivalent without
/// accessibility permissions); registration succeeds so callers proceed.
pub unsafe extern "win64" fn RegisterHotKey(hwnd: HWND, id: i32, mods: u32, vk: u32) -> BOOL {
    stub!("RegisterHotKey(hwnd {:#x}, id {id}, mods {mods:#x}, vk {vk:#x}): never fires", hwnd as usize);
    TRUE
}

pub unsafe extern "win64" fn UnregisterHotKey(_hwnd: HWND, _id: i32) -> BOOL {
    TRUE
}

// ---------------------------------------------------------------------------
// Windows hooks (SetWindowsHookEx / UnhookWindowsHookEx / CallNextHookEx)
// and WinEvent hooks (SetWinEventHook / UnhookWinEvent)
// ---------------------------------------------------------------------------

pub const WH_KEYBOARD: i32 = 2;
pub const WH_GETMESSAGE: i32 = 3;
pub const WH_CALLWNDPROC: i32 = 4;
pub const WH_MOUSE: i32 = 7;
pub const WH_KEYBOARD_LL: i32 = 13;
pub const WH_MOUSE_LL: i32 = 14;

pub const HC_ACTION: i32 = 0;
pub const HC_NOREMOVE: i32 = 3;
pub const PM_NOREMOVE_HOOK: WPARAM = 0;
pub const PM_REMOVE_HOOK: WPARAM = 1;

const ERROR_INVALID_HOOK_HANDLE: u32 = 1404;
const ERROR_INVALID_HOOK_FILTER: u32 = 1426;
const ERROR_INVALID_FILTER_PROC: u32 = 1427;

/// `CWPSTRUCT` (Win64: 32 bytes) passed as `lParam` to `WH_CALLWNDPROC` hooks.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CWPSTRUCT {
    pub lparam: LPARAM,
    pub wparam: WPARAM,
    pub message: u32,
    pub _pad: u32,
    pub hwnd: usize,
}
const _: () = assert!(std::mem::size_of::<CWPSTRUCT>() == 32);

/// `MOUSEHOOKSTRUCTEX` (Win64: 40 bytes; first 32 bytes are `MOUSEHOOKSTRUCT`)
/// passed as `lParam` to `WH_MOUSE` hooks.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MOUSEHOOKSTRUCTEX {
    pub pt: POINT,
    pub hwnd: usize,
    pub w_hit_test_code: u32,
    pub _pad0: u32,
    pub dw_extra_info: usize,
    pub mouse_data: u32,
    pub _pad1: u32,
}
const _: () = assert!(std::mem::size_of::<MOUSEHOOKSTRUCTEX>() == 40);

/// `KBDLLHOOKSTRUCT` (Win64: 24 bytes) passed as `lParam` to `WH_KEYBOARD_LL` hooks.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
    pub scan_code: u32,
    pub flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}
const _: () = assert!(std::mem::size_of::<KBDLLHOOKSTRUCT>() == 24);

/// `MSLLHOOKSTRUCT` (Win64: 32 bytes) passed as `lParam` to `WH_MOUSE_LL` hooks.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouse_data: u32,
    pub flags: u32,
    pub time: u32,
    pub _pad: u32,
    pub dw_extra_info: usize,
}
const _: () = assert!(std::mem::size_of::<MSLLHOOKSTRUCT>() == 32);

#[derive(Clone, Copy, Debug)]
struct HookEntry {
    handle: usize,
    id_hook: i32,
    lpfn: usize,
    tid: u32,
}

static NEXT_HOOK: AtomicUsize = AtomicUsize::new(0x4000_0010);
static HOOKS: std::sync::Mutex<Vec<HookEntry>> = std::sync::Mutex::new(Vec::new());
/// Per-`id_hook` fast-path counters indexed by `(id_hook + 1)` for `-1..=14`.
static HOOK_COUNTS: [std::sync::atomic::AtomicU32; 16] = [const { std::sync::atomic::AtomicU32::new(0) }; 16];

struct ActiveHookFrame {
    /// Snapshot of matching `(handle, lpfn)` in LIFO dispatch order.
    chain: Vec<(usize, usize)>,
    /// Index of the next hook in `chain` to invoke on `CallNextHookEx`.
    next_index: usize,
}

thread_local! {
    static HOOK_STACK: std::cell::RefCell<Vec<ActiveHookFrame>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[inline]
fn hook_slot(id_hook: i32) -> Option<usize> {
    if (-1..=14).contains(&id_hook) && id_hook != 8 {
        Some((id_hook + 1) as usize)
    } else {
        None
    }
}

#[inline]
pub(crate) fn has_hooks(id_hook: i32) -> bool {
    hook_slot(id_hook).is_some_and(|slot| HOOK_COUNTS[slot].load(Ordering::Acquire) != 0)
}

#[inline]
pub(crate) fn can_invoke_guest_hooks_on_this_thread() -> bool {
    // When AppKit/UIKit is active, the platform main thread has no Win32 TEB
    // and must never run guest code. In headless / unit-test runs, `active()`
    // is false so tests can run hooks on any thread.
    !(crate::macos::active() && crate::macos::is_main_thread())
}

fn is_hook_registered(handle: usize) -> bool {
    HOOKS.lock().unwrap().iter().any(|e| e.handle == handle)
}

/// Dispatch all registered hooks of type `id_hook` matching `target_tid` (or
/// global `tid == 0`) in LIFO order. Tracks the active chain in a thread-local
/// stack so `CallNextHookEx` advances cleanly through the remaining hooks.
pub(crate) fn dispatch_hook(
    id_hook: i32,
    target_tid: u32,
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if !has_hooks(id_hook) || !can_invoke_guest_hooks_on_this_thread() {
        return 0;
    }
    let chain: Vec<(usize, usize)> = {
        let hooks = HOOKS.lock().unwrap();
        hooks
            .iter()
            .rev()
            .filter(|e| e.id_hook == id_hook && (e.tid == 0 || e.tid == target_tid))
            .map(|e| (e.handle, e.lpfn))
            .collect()
    };
    if chain.is_empty() {
        return 0;
    }
    let first_lpfn = chain[0].1;
    HOOK_STACK.with(|stack| {
        stack.borrow_mut().push(ActiveHookFrame { chain, next_index: 1 });
    });
    struct PopGuard;
    impl Drop for PopGuard {
        fn drop(&mut self) {
            HOOK_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }
    let _pop = PopGuard;
    unsafe {
        crate::call_guest8(
            first_lpfn,
            code as i64 as u64,
            wparam as u64,
            lparam as u64,
            0,
            0,
            0,
            0,
            0,
        ) as LRESULT
    }
}

/// Dispatch `WH_GETMESSAGE` hooks for a message retrieved by `GetMessage` or
/// `PeekMessage`. Hooks may inspect or modify `msg` in place.
pub(crate) fn dispatch_getmessage_hook(msg: &mut MSG, remove: bool) {
    if !has_hooks(WH_GETMESSAGE) || !can_invoke_guest_hooks_on_this_thread() {
        return;
    }
    let tid = crate::teb::thread_id();
    let wp = if remove { PM_REMOVE_HOOK } else { PM_NOREMOVE_HOOK };
    let _ = dispatch_hook(WH_GETMESSAGE, tid, HC_ACTION, wp, msg as *mut MSG as LPARAM);
}

/// Dispatch `WH_CALLWNDPROC` hooks before `SendMessage` invokes a window proc.
pub(crate) fn dispatch_callwndproc_hook(
    hwnd: usize,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    from_current_thread: bool,
) {
    if !has_hooks(WH_CALLWNDPROC) || !can_invoke_guest_hooks_on_this_thread() {
        return;
    }
    let cwp = CWPSTRUCT { lparam, wparam, message, _pad: 0, hwnd };
    let tid = crate::teb::thread_id();
    let wp = if from_current_thread { 1usize } else { 0usize };
    let _ = dispatch_hook(WH_CALLWNDPROC, tid, HC_ACTION, wp, &cwp as *const CWPSTRUCT as LPARAM);
}

pub unsafe extern "win64" fn SetWindowsHookExW(
    id_hook: i32,
    lpfn: usize,
    _hmod: HINSTANCE,
    tid: u32,
) -> usize {
    let Some(slot) = hook_slot(id_hook) else {
        set_error(ERROR_INVALID_HOOK_FILTER);
        return 0;
    };
    if lpfn == 0 {
        set_error(ERROR_INVALID_FILTER_PROC);
        return 0;
    }
    let h = NEXT_HOOK.fetch_add(4, Ordering::Relaxed);
    {
        let mut hooks = HOOKS.lock().unwrap();
        hooks.push(HookEntry { handle: h, id_hook, lpfn, tid });
        HOOK_COUNTS[slot].fetch_add(1, Ordering::Release);
    }
    log::debug!("user32: SetWindowsHookExW(id={id_hook}, lpfn={lpfn:#x}, tid={tid}) -> {h:#x}");
    h
}

pub unsafe extern "win64" fn SetWindowsHookExA(
    id_hook: i32,
    lpfn: usize,
    hmod: HINSTANCE,
    tid: u32,
) -> usize {
    SetWindowsHookExW(id_hook, lpfn, hmod, tid)
}

pub unsafe extern "win64" fn UnhookWindowsHookEx(hhk: usize) -> BOOL {
    if hhk == 0 {
        set_error(ERROR_INVALID_HOOK_HANDLE);
        return FALSE;
    }
    let removed = {
        let mut hooks = HOOKS.lock().unwrap();
        if let Some(pos) = hooks.iter().position(|e| e.handle == hhk) {
            let entry = hooks.remove(pos);
            if let Some(slot) = hook_slot(entry.id_hook) {
                HOOK_COUNTS[slot].fetch_sub(1, Ordering::Release);
            }
            Some(entry)
        } else {
            None
        }
    };
    match removed {
        Some(entry) => {
            log::debug!("user32: UnhookWindowsHookEx({hhk:#x}, id={})", entry.id_hook);
            TRUE
        }
        None => {
            set_error(ERROR_INVALID_HOOK_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CallNextHookEx(
    _hhk: usize,
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Windows ignores the `hhk` parameter and uses the current thread's active
    // hook chain to advance to the next registered hook procedure.
    let next: Option<(usize, usize)> = HOOK_STACK.with(|stack| {
        let mut s = stack.borrow_mut();
        let frame = s.last_mut()?;
        let saved = frame.next_index;
        let mut idx = saved;
        while idx < frame.chain.len() {
            let (cand_h, cand_fn) = frame.chain[idx];
            idx += 1;
            if is_hook_registered(cand_h) {
                frame.next_index = idx;
                return Some((saved, cand_fn));
            }
        }
        None
    });
    let Some((saved_next_index, next_lpfn)) = next else {
        return 0;
    };
    struct RestoreIndexGuard(usize);
    impl Drop for RestoreIndexGuard {
        fn drop(&mut self) {
            HOOK_STACK.with(|stack| {
                if let Some(frame) = stack.borrow_mut().last_mut() {
                    frame.next_index = self.0;
                }
            });
        }
    }
    let _restore = RestoreIndexGuard(saved_next_index);
    crate::call_guest8(
        next_lpfn,
        code as i64 as u64,
        wparam as u64,
        lparam as u64,
        0,
        0,
        0,
        0,
        0,
    ) as LRESULT
}

#[derive(Clone, Copy, Debug)]
struct WinEventHookEntry {
    handle: usize,
    event_min: u32,
    event_max: u32,
    proc_: usize,
    pid: u32,
    tid: u32,
}

static WIN_EVENT_HOOKS: std::sync::Mutex<Vec<WinEventHookEntry>> = std::sync::Mutex::new(Vec::new());
static WIN_EVENT_HOOK_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Dispatch active `SetWinEventHook` callbacks matching `event`, current PID,
/// and current TID.
pub(crate) fn dispatch_win_event(event: u32, hwnd: usize, id_object: i32, id_child: i32) {
    if WIN_EVENT_HOOK_COUNT.load(Ordering::Acquire) == 0 || !can_invoke_guest_hooks_on_this_thread() {
        return;
    }
    let cur_pid = unsafe { crate::kernel32::process::GetCurrentProcessId() };
    let cur_tid = crate::teb::thread_id();
    let matched: Vec<(usize, usize)> = {
        let hooks = WIN_EVENT_HOOKS.lock().unwrap();
        hooks
            .iter()
            .filter(|e| {
                event >= e.event_min
                    && event <= e.event_max
                    && (e.pid == 0 || e.pid == cur_pid)
                    && (e.tid == 0 || e.tid == cur_tid)
            })
            .map(|e| (e.handle, e.proc_))
            .collect()
    };
    if matched.is_empty() {
        return;
    }
    let time = tick_ms();
    for (handle, proc_) in matched {
        unsafe {
            crate::call_guest8(
                proc_,
                handle as u64,
                event as u64,
                hwnd as u64,
                id_object as i64 as u64,
                id_child as i64 as u64,
                cur_tid as u64,
                time as u64,
                0,
            );
        }
    }
}

pub unsafe extern "win64" fn SetWinEventHook(
    event_min: u32,
    event_max: u32,
    _hmod: HINSTANCE,
    proc_: usize,
    pid: u32,
    tid: u32,
    flags: u32,
) -> usize {
    if proc_ == 0 || event_min > event_max {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let h = NEXT_HOOK.fetch_add(4, Ordering::Relaxed);
    {
        let mut hooks = WIN_EVENT_HOOKS.lock().unwrap();
        hooks.push(WinEventHookEntry { handle: h, event_min, event_max, proc_, pid, tid });
        WIN_EVENT_HOOK_COUNT.fetch_add(1, Ordering::Release);
    }
    log::debug!(
        "user32: SetWinEventHook({event_min:#x}..={event_max:#x}, proc={proc_:#x}, pid={pid}, tid={tid}, flags={flags:#x}) -> {h:#x}"
    );
    h
}

pub unsafe extern "win64" fn UnhookWinEvent(h: usize) -> BOOL {
    if h == 0 {
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let removed = {
        let mut hooks = WIN_EVENT_HOOKS.lock().unwrap();
        if let Some(pos) = hooks.iter().position(|e| e.handle == h) {
            hooks.remove(pos);
            WIN_EVENT_HOOK_COUNT.fetch_sub(1, Ordering::Release);
            true
        } else {
            false
        }
    };
    if removed {
        log::debug!("user32: UnhookWinEvent({h:#x})");
        TRUE
    } else {
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Menus (minimal handle tracking; no native menu bar rendering)
// ---------------------------------------------------------------------------

static NEXT_MENU: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x5000_0010);

fn window_menus() -> &'static std::sync::Mutex<std::collections::HashMap<usize, usize>> {
    static M: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, usize>>> = std::sync::OnceLock::new();
    M.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

pub unsafe extern "win64" fn CreateMenu() -> usize {
    NEXT_MENU.fetch_add(4, Ordering::Relaxed)
}

pub unsafe extern "win64" fn CreatePopupMenu() -> usize {
    NEXT_MENU.fetch_add(4, Ordering::Relaxed)
}

pub unsafe extern "win64" fn DestroyMenu(hmenu: usize) -> BOOL {
    if hmenu == 0 { FALSE } else { TRUE }
}

pub unsafe extern "win64" fn GetMenu(hwnd: HWND) -> usize {
    window_menus().lock().unwrap().get(&(hwnd as usize)).copied().unwrap_or(0)
}

pub unsafe extern "win64" fn SetMenu(hwnd: HWND, hmenu: usize) -> BOOL {
    if window::get(hwnd as usize).is_none() {
        set_error(e::INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    let mut m = window_menus().lock().unwrap();
    if hmenu == 0 {
        m.remove(&(hwnd as usize));
    } else {
        m.insert(hwnd as usize, hmenu);
    }
    TRUE
}

pub unsafe extern "win64" fn GetSystemMenu(hwnd: HWND, revert: BOOL) -> usize {
    if window::get(hwnd as usize).is_none() {
        return 0;
    }
    let _ = revert;
    0x5000_0004
}

pub unsafe extern "win64" fn DrawMenuBar(hwnd: HWND) -> BOOL {
    if window::get(hwnd as usize).is_none() { FALSE } else { TRUE }
}

pub unsafe extern "win64" fn GetSubMenu(_hmenu: usize, _pos: i32) -> usize {
    0
}

pub unsafe extern "win64" fn GetMenuItemCount(_hmenu: usize) -> i32 {
    0
}

pub unsafe extern "win64" fn AppendMenuW(_hmenu: usize, _flags: u32, _id: usize, _item: LPCWSTR) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn AppendMenuA(_hmenu: usize, _flags: u32, _id: usize, _item: LPCSTR) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn InsertMenuW(_hmenu: usize, _pos: u32, _flags: u32, _id: usize, _item: LPCWSTR) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn InsertMenuA(_hmenu: usize, _pos: u32, _flags: u32, _id: usize, _item: LPCSTR) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn InsertMenuItemW(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *const u8) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn InsertMenuItemA(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *const u8) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn RemoveMenu(_hmenu: usize, _pos: u32, _flags: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn DeleteMenu(_hmenu: usize, _pos: u32, _flags: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn EnableMenuItem(_hmenu: usize, _id: u32, _enable: u32) -> BOOL {
    0 // MF_ENABLED (previous state)
}

pub unsafe extern "win64" fn CheckMenuItem(_hmenu: usize, _id: u32, _check: u32) -> u32 {
    0 // MF_UNCHECKED
}

pub unsafe extern "win64" fn GetMenuItemInfoW(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *mut u8) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn GetMenuItemInfoA(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *mut u8) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn SetMenuItemInfoW(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *const u8) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetMenuItemInfoA(_hmenu: usize, _item: u32, _by_pos: BOOL, _info: *const u8) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn TrackPopupMenu(
    _hmenu: usize,
    _flags: u32,
    _x: i32,
    _y: i32,
    _reserved: i32,
    _hwnd: HWND,
    _rect: *const RECT,
) -> BOOL {
    0
}

pub unsafe extern "win64" fn TrackPopupMenuEx(
    _hmenu: usize,
    _flags: u32,
    _x: i32,
    _y: i32,
    _hwnd: HWND,
    _params: *const u8,
) -> BOOL {
    0
}

// ---------------------------------------------------------------------------
// Clipboard (in-process GlobalAlloc handles + CF_UNICODETEXT <-> CF_TEXT +
// custom registered formats + real macOS NSPasteboard sync)
// ---------------------------------------------------------------------------

static CLIPBOARD_OPEN: AtomicBool = AtomicBool::new(false);
static CLIPBOARD_SEQ: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
const ERROR_CLIPBOARD_NOT_OPEN: u32 = 1418;
const CF_TEXT: u32 = 1;
const CF_OEMTEXT: u32 = 7;
const CF_UNICODETEXT: u32 = 13;

struct ClipboardState {
    formats: std::collections::BTreeMap<u32, usize>,
    /// Handles allocated internally by us (freed on EmptyClipboard / overwrite).
    owned_handles: std::collections::HashSet<usize>,
    custom_names: std::collections::BTreeMap<u32, String>,
    next_custom_id: u32,
    /// Last observed `NSPasteboard.generalPasteboard.changeCount`.
    last_macos_change: Option<isize>,
}

fn clipboard_state() -> &'static std::sync::Mutex<ClipboardState> {
    static S: std::sync::OnceLock<std::sync::Mutex<ClipboardState>> = std::sync::OnceLock::new();
    S.get_or_init(|| {
        std::sync::Mutex::new(ClipboardState {
            formats: std::collections::BTreeMap::new(),
            owned_handles: std::collections::HashSet::new(),
            custom_names: std::collections::BTreeMap::new(),
            next_custom_id: 0xC000,
            last_macos_change: None,
        })
    })
}

/// Convert Windows CRLF line endings to macOS LF line endings.
pub(crate) fn win_to_macos_newlines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}

/// Convert macOS LF (or lone CR) line endings to Windows CRLF without doubling
/// existing CRLF sequences.
pub(crate) fn macos_to_win_newlines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push_str("\r\n");
            }
            '\n' => out.push_str("\r\n"),
            _ => out.push(c),
        }
    }
    out
}

unsafe fn read_wide_unaligned(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut units = Vec::new();
    let mut i = 0usize;
    loop {
        let u = p.add(i).read_unaligned();
        if u == 0 {
            break;
        }
        units.push(u);
        i += 1;
    }
    String::from_utf16_lossy(&units)
}

unsafe fn alloc_clipboard_bytes(bytes: &[u8]) -> usize {
    let p = crate::heap::alloc(bytes.len().max(1), true) as *mut u8;
    if !p.is_null() && !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
    }
    p as usize
}

unsafe fn insert_owned_format(st: &mut ClipboardState, fmt: u32, h: usize) {
    if let Some(old) = st.formats.insert(fmt, h) {
        if old != h && st.owned_handles.remove(&old) && !st.formats.values().any(|&v| v == old) {
            let _ = crate::heap::free(old as *mut std::ffi::c_void);
        }
    }
    if h != 0 {
        st.owned_handles.insert(h);
    }
}

unsafe fn clear_all_formats(st: &mut ClipboardState) {
    st.formats.clear();
    for h in st.owned_handles.drain() {
        let _ = crate::heap::free(h as *mut std::ffi::c_void);
    }
}

unsafe fn remove_text_formats(st: &mut ClipboardState) {
    for fmt in [CF_TEXT, CF_OEMTEXT, CF_UNICODETEXT] {
        if let Some(old) = st.formats.remove(&fmt) {
            if st.owned_handles.contains(&old) && !st.formats.values().any(|&v| v == old) {
                st.owned_handles.remove(&old);
                let _ = crate::heap::free(old as *mut std::ffi::c_void);
            }
        }
    }
}

unsafe fn populate_text_formats_from_win_str(st: &mut ClipboardState, win_text: &str) {
    let mut ansi = win_text.as_bytes().to_vec();
    ansi.push(0);
    let mut wide: Vec<u8> = Vec::with_capacity((win_text.len() + 1) * 2);
    for u in win_text.encode_utf16().chain(std::iter::once(0u16)) {
        wide.extend_from_slice(&u.to_le_bytes());
    }
    let a_handle = alloc_clipboard_bytes(&ansi);
    let w_handle = alloc_clipboard_bytes(&wide);
    insert_owned_format(st, CF_TEXT, a_handle);
    insert_owned_format(st, CF_OEMTEXT, a_handle);
    insert_owned_format(st, CF_UNICODETEXT, w_handle);
}

unsafe fn ingest_macos_text(st: &mut ClipboardState, mac_text: Option<&str>) {
    remove_text_formats(st);
    if let Some(text) = mac_text {
        let win_text = macos_to_win_newlines(text);
        populate_text_formats_from_win_str(st, &win_text);
    }
    CLIPBOARD_SEQ.fetch_add(1, Ordering::Relaxed);
}

unsafe fn sync_from_macos_if_changed(st: &mut ClipboardState) {
    let Some(cc) = crate::macos::clipboard_change_count() else {
        return;
    };
    if st.last_macos_change == Some(cc) {
        return;
    }
    st.last_macos_change = Some(cc);
    let text = crate::macos::clipboard_get_text();
    ingest_macos_text(st, text.as_deref());
}

/// High-level helper to put UTF-8 text onto the Win32 clipboard (and macOS
/// `NSPasteboard` when active), converting line endings cleanly.
pub fn clipboard_set_text(text: &str) {
    unsafe {
        let mut st = clipboard_state().lock().unwrap();
        clear_all_formats(&mut st);
        let win_text = macos_to_win_newlines(text);
        populate_text_formats_from_win_str(&mut st, &win_text);
        let mac_text = win_to_macos_newlines(&win_text);
        if let Some(cc) = crate::macos::clipboard_set_text(mac_text) {
            st.last_macos_change = Some(cc);
        }
        CLIPBOARD_SEQ.fetch_add(1, Ordering::Relaxed);
    }
}

/// High-level helper to read text from the Win32 clipboard (pulling from macOS
/// `NSPasteboard` first if it changed externally).
pub fn clipboard_get_text() -> Option<String> {
    unsafe {
        let mut st = clipboard_state().lock().unwrap();
        sync_from_macos_if_changed(&mut st);
        if let Some(&h) = st.formats.get(&CF_UNICODETEXT) {
            let p = crate::kernel32::memory::GlobalLock(h as *mut std::ffi::c_void) as *const u16;
            if !p.is_null() {
                return Some(read_wide_unaligned(p));
            }
        }
        if let Some(&h) = st.formats.get(&CF_TEXT) {
            let p = crate::kernel32::memory::GlobalLock(h as *mut std::ffi::c_void) as *const u8;
            if !p.is_null() {
                return Some(crate::strings::from_ansi(p));
            }
        }
        None
    }
}

pub unsafe extern "win64" fn OpenClipboard(_hwnd: HWND) -> BOOL {
    if CLIPBOARD_OPEN.swap(true, Ordering::AcqRel) {
        set_error(5); // ERROR_ACCESS_DENIED
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn CloseClipboard() -> BOOL {
    if !CLIPBOARD_OPEN.swap(false, Ordering::AcqRel) {
        set_error(ERROR_CLIPBOARD_NOT_OPEN);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn EmptyClipboard() -> BOOL {
    if !CLIPBOARD_OPEN.load(Ordering::Acquire) {
        set_error(ERROR_CLIPBOARD_NOT_OPEN);
        return FALSE;
    }
    let mut st = clipboard_state().lock().unwrap();
    clear_all_formats(&mut st);
    if let Some(cc) = crate::macos::clipboard_clear() {
        st.last_macos_change = Some(cc);
    }
    CLIPBOARD_SEQ.fetch_add(1, Ordering::Relaxed);
    TRUE
}

pub unsafe extern "win64" fn GetClipboardSequenceNumber() -> u32 {
    let mut st = clipboard_state().lock().unwrap();
    sync_from_macos_if_changed(&mut st);
    CLIPBOARD_SEQ.load(Ordering::Relaxed)
}

pub unsafe extern "win64" fn CountClipboardFormats() -> i32 {
    let mut st = clipboard_state().lock().unwrap();
    sync_from_macos_if_changed(&mut st);
    st.formats.len() as i32
}

pub unsafe extern "win64" fn EnumClipboardFormats(prev: u32) -> u32 {
    if !CLIPBOARD_OPEN.load(Ordering::Acquire) {
        set_error(ERROR_CLIPBOARD_NOT_OPEN);
        return 0;
    }
    let mut st = clipboard_state().lock().unwrap();
    if prev == 0 {
        sync_from_macos_if_changed(&mut st);
        st.formats.keys().next().copied().unwrap_or(0)
    } else {
        st.formats.range((prev + 1)..).next().map(|(k, _)| *k).unwrap_or(0)
    }
}

pub unsafe extern "win64" fn IsClipboardFormatAvailable(fmt: u32) -> BOOL {
    let mut st = clipboard_state().lock().unwrap();
    if matches!(fmt, CF_TEXT | CF_OEMTEXT | CF_UNICODETEXT) {
        sync_from_macos_if_changed(&mut st);
    }
    st.formats.contains_key(&fmt) as BOOL
}

pub unsafe extern "win64" fn GetClipboardData(fmt: u32) -> usize {
    if !CLIPBOARD_OPEN.load(Ordering::Acquire) {
        set_error(ERROR_CLIPBOARD_NOT_OPEN);
        return 0;
    }
    let mut st = clipboard_state().lock().unwrap();
    if matches!(fmt, CF_TEXT | CF_OEMTEXT | CF_UNICODETEXT) {
        sync_from_macos_if_changed(&mut st);
    }
    match st.formats.get(&fmt).copied() {
        Some(h) => h,
        None => {
            set_error(1168); // ERROR_NOT_FOUND
            0
        }
    }
}

pub unsafe extern "win64" fn SetClipboardData(fmt: u32, h: usize) -> usize {
    if !CLIPBOARD_OPEN.load(Ordering::Acquire) {
        set_error(ERROR_CLIPBOARD_NOT_OPEN);
        return 0;
    }
    if h == 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let mut st = clipboard_state().lock().unwrap();
    if let Some(old) = st.formats.insert(fmt, h) {
        if old != h && st.owned_handles.contains(&old) && !st.formats.values().any(|&v| v == old) {
            st.owned_handles.remove(&old);
            let _ = crate::heap::free(old as *mut std::ffi::c_void);
        }
    }
    if fmt == CF_UNICODETEXT {
        let p = crate::kernel32::memory::GlobalLock(h as *mut std::ffi::c_void) as *const u16;
        let s = read_wide_unaligned(p);
        let _ = crate::kernel32::memory::GlobalUnlock(h as *mut std::ffi::c_void);
        let mut ansi = s.as_bytes().to_vec();
        ansi.push(0);
        let a_handle = alloc_clipboard_bytes(&ansi);
        insert_owned_format(&mut st, CF_TEXT, a_handle);
        insert_owned_format(&mut st, CF_OEMTEXT, a_handle);
        if let Some(cc) = crate::macos::clipboard_set_text(win_to_macos_newlines(&s)) {
            st.last_macos_change = Some(cc);
        }
    } else if fmt == CF_TEXT || fmt == CF_OEMTEXT {
        let p = crate::kernel32::memory::GlobalLock(h as *mut std::ffi::c_void) as *const u8;
        let s = crate::strings::from_ansi(p);
        let _ = crate::kernel32::memory::GlobalUnlock(h as *mut std::ffi::c_void);
        let mut wide = Vec::with_capacity((s.len() + 1) * 2);
        for u in s.encode_utf16().chain(std::iter::once(0u16)) {
            wide.extend_from_slice(&u.to_le_bytes());
        }
        let w_handle = alloc_clipboard_bytes(&wide);
        insert_owned_format(&mut st, CF_UNICODETEXT, w_handle);
        let other = if fmt == CF_TEXT { CF_OEMTEXT } else { CF_TEXT };
        let mut ansi = s.as_bytes().to_vec();
        ansi.push(0);
        let a_handle = alloc_clipboard_bytes(&ansi);
        insert_owned_format(&mut st, other, a_handle);
        if let Some(cc) = crate::macos::clipboard_set_text(win_to_macos_newlines(&s)) {
            st.last_macos_change = Some(cc);
        }
    }
    CLIPBOARD_SEQ.fetch_add(1, Ordering::Relaxed);
    h
}

pub unsafe extern "win64" fn RegisterClipboardFormatW(name: LPCWSTR) -> u32 {
    if name.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let s = crate::strings::from_wide(name);
    let mut st = clipboard_state().lock().unwrap();
    for (&id, existing) in &st.custom_names {
        if existing.eq_ignore_ascii_case(&s) {
            return id;
        }
    }
    let id = st.next_custom_id;
    st.next_custom_id = (st.next_custom_id + 1).min(0xFFFF);
    st.custom_names.insert(id, s);
    id
}

pub unsafe extern "win64" fn RegisterClipboardFormatA(name: LPCSTR) -> u32 {
    if name.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let w = crate::strings::to_wide_z(&crate::strings::from_ansi(name));
    RegisterClipboardFormatW(w.as_ptr())
}

pub unsafe extern "win64" fn GetClipboardFormatNameW(fmt: u32, buf: LPWSTR, max_count: i32) -> i32 {
    if buf.is_null() || max_count <= 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let st = clipboard_state().lock().unwrap();
    let Some(name) = st.custom_names.get(&fmt) else {
        set_error(e::INVALID_PARAMETER);
        return 0;
    };
    crate::strings::write_wide_trunc(name, buf, max_count as usize);
    name.encode_utf16().count().min(max_count as usize - 1) as i32
}

pub unsafe extern "win64" fn GetClipboardFormatNameA(fmt: u32, buf: LPSTR, max_count: i32) -> i32 {
    if buf.is_null() || max_count <= 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let st = clipboard_state().lock().unwrap();
    let Some(name) = st.custom_names.get(&fmt) else {
        set_error(e::INVALID_PARAMETER);
        return 0;
    };
    crate::strings::write_ansi_trunc(name, buf, max_count as usize);
    name.len().min(max_count as usize - 1) as i32
}

// ---------------------------------------------------------------------------
// DPI awareness (always per-monitor aware v2; see the Retina policy)
// ---------------------------------------------------------------------------

const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: isize = -4;

pub unsafe extern "win64" fn SetProcessDPIAware() -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn IsProcessDPIAware() -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetProcessDpiAwarenessContext(_ctx: isize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetThreadDpiAwarenessContext(_ctx: isize) -> isize {
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
}

pub unsafe extern "win64" fn GetThreadDpiAwarenessContext() -> isize {
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
}

pub unsafe extern "win64" fn GetWindowDpiAwarenessContext(_hwnd: HWND) -> isize {
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
}

pub unsafe extern "win64" fn AreDpiAwarenessContextsEqual(a: isize, b: isize) -> BOOL {
    (a == b) as BOOL
}

pub unsafe extern "win64" fn EnableNonClientDpiScaling(_hwnd: HWND) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetSystemDpiForProcess(_hprocess: HANDLE) -> u32 {
    window::dpi()
}

pub unsafe extern "win64" fn GetDpiForMonitor(hmonitor: usize, dpi_type: i32, dpi_x: *mut u32, dpi_y: *mut u32) -> HRESULT {
    if dpi_x.is_null() || dpi_y.is_null() || !(0..=2).contains(&dpi_type) || hmonitor == 0 {
        return E_INVALIDARG;
    }
    let Some(d) = super::monitor::dpi_for_monitor(hmonitor) else {
        return E_INVALIDARG;
    };
    dpi_x.write_unaligned(d);
    dpi_y.write_unaligned(d);
    S_OK
}

pub unsafe extern "win64" fn GetProcessDpiAwareness(_hprocess: HANDLE, value: *mut i32) -> HRESULT {
    if value.is_null() {
        return E_INVALIDARG;
    }
    *value = 2; // PROCESS_PER_MONITOR_DPI_AWARE
    S_OK
}

pub unsafe extern "win64" fn EnumChildWindows(
    _hwnd_parent: HWND,
    _enum_func: usize,
    _lparam: LPARAM,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn AllowSetForegroundWindow(_pid: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetWindowTextLengthA(hwnd: HWND) -> i32 {
    crate::user32::window::GetWindowTextLengthW(hwnd)
}

pub unsafe extern "win64" fn CharLowerBuffW(lpsz: LPWSTR, cch_length: u32) -> u32 {
    if lpsz.is_null() || cch_length == 0 {
        return 0;
    }
    for i in 0..cch_length as usize {
        let ch = *lpsz.add(i);
        if (b'A' as u16..=b'Z' as u16).contains(&ch) {
            *lpsz.add(i) = ch + 32;
        }
    }
    cch_length
}

pub unsafe extern "win64" fn wsprintfA(
    out: LPSTR,
    fmt: LPCSTR,
    a1: usize,
    a2: usize,
) -> i32 {
    let _ = (a1, a2);
    if out.is_null() {
        return 0;
    }
    let s = crate::strings::from_ansi_opt(fmt).unwrap_or_default();
    crate::strings::write_ansi_trunc(&s, out, 1024);
    s.len() as i32
}

pub unsafe extern "win64" fn SetProcessDpiAwareness(_value: i32) -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn GetWindowDpiHostingBehavior(_hwnd: HWND) -> i32 {
    0 // DPI_HOSTING_BEHAVIOR_DEFAULT
}

pub unsafe extern "win64" fn GetThreadDpiHostingBehavior() -> i32 {
    0 // DPI_HOSTING_BEHAVIOR_DEFAULT
}

pub unsafe extern "win64" fn GetLastActivePopup(hwnd: HWND) -> HWND {
    hwnd
}

// ---------------------------------------------------------------------------
// MessageBox
// ---------------------------------------------------------------------------

const MB_TYPEMASK: u32 = 0xF;
const MB_ICONMASK: u32 = 0xF0;
const MB_ICONHAND: u32 = 0x10;
const IDOK: i32 = 1;
const IDCANCEL: i32 = 2;
const IDABORT: i32 = 3;
const IDRETRY: i32 = 4;
const IDIGNORE: i32 = 5;
const IDYES: i32 = 6;
const IDNO: i32 = 7;
const IDTRYAGAIN: i32 = 10;
const IDCONTINUE: i32 = 11;

fn message_box(text: Option<String>, caption: Option<String>, ty: u32) -> i32 {
    let caption = caption.unwrap_or_else(|| "Error".into());
    let text = text.unwrap_or_default();
    log::error!(target: "messagebox", "MessageBox [{caption}] {text}");
    let buttons: &[(&str, i32)] = match ty & MB_TYPEMASK {
        0 => &[("OK", IDOK)],
        1 => &[("OK", IDOK), ("Cancel", IDCANCEL)],
        2 => &[("Abort", IDABORT), ("Retry", IDRETRY), ("Ignore", IDIGNORE)],
        3 => &[("Yes", IDYES), ("No", IDNO), ("Cancel", IDCANCEL)],
        4 => &[("Yes", IDYES), ("No", IDNO)],
        5 => &[("Retry", IDRETRY), ("Cancel", IDCANCEL)],
        6 => &[("Cancel", IDCANCEL), ("Try Again", IDTRYAGAIN), ("Continue", IDCONTINUE)],
        _ => &[("OK", IDOK)],
    };
    let names = buttons.iter().map(|b| b.0.to_string()).collect();
    match crate::macos::alert(caption, text, names, ty & MB_ICONMASK == MB_ICONHAND) {
        Some(i) => {
            let r = buttons[i.min(buttons.len() - 1)].1;
            log::info!(target: "messagebox", "MessageBox answer: {}", buttons[i.min(buttons.len() - 1)].0);
            r
        }
        None if buttons.len() == 1 => IDOK,
        None => {
            stub!("MessageBox type {ty:#x}: headless, cannot ask the user -> 0 (failure)");
            set_error(e::CALL_NOT_IMPLEMENTED);
            0
        }
    }
}

pub unsafe extern "win64" fn MessageBoxW(_hwnd: HWND, text: LPCWSTR, caption: LPCWSTR, ty: u32) -> i32 {
    message_box(crate::strings::from_wide_opt(text), crate::strings::from_wide_opt(caption), ty)
}

pub unsafe extern "win64" fn MessageBoxA(_hwnd: HWND, text: LPCSTR, caption: LPCSTR, ty: u32) -> i32 {
    message_box(crate::strings::from_ansi_opt(text), crate::strings::from_ansi_opt(caption), ty)
}

const RT_STRING: usize = 6;

/// Load a string resource from `hinstance`'s `RT_STRING` table (16 strings per block).
///
/// Follows the Win32 `LoadStringW` contract:
/// * `block_id = (u_id >> 4) + 1`, `index = u_id & 0x0F`.
/// * When `cch_buffer_max == 0`, writes a read-only `*const u16` pointer directly into
///   the resource block at `*(lp_buffer as *mut *const u16)` and returns the UTF-16 code-unit
///   length (without NUL termination).
/// * When `cch_buffer_max > 0`, copies up to `cch_buffer_max - 1` UTF-16 code units, writes
///   a trailing NUL terminator, and returns the number of copied code units (excluding NUL).
pub unsafe extern "win64" fn LoadStringW(
    hinstance: HINSTANCE,
    u_id: u32,
    lp_buffer: LPWSTR,
    cch_buffer_max: i32,
) -> i32 {
    if lp_buffer.is_null() || cch_buffer_max < 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let block_id = ((u_id >> 4) as usize).wrapping_add(1);
    let index = (u_id & 0x0F) as usize;
    if block_id == 0 || block_id > 0xFFFF {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        set_error(e::RESOURCE_NAME_NOT_FOUND);
        return 0;
    }
    if hinstance.is_null() && crate::try_loader().is_none() {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        set_error(e::RESOURCE_TYPE_NOT_FOUND);
        return 0;
    }
    let hres = crate::kernel32::module::FindResourceW(
        hinstance,
        block_id as LPCWSTR,
        RT_STRING as LPCWSTR,
    );
    if hres.is_null() {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        return 0;
    }
    let byte_size = crate::kernel32::module::SizeofResource(hinstance, hres) as usize;
    let hglob = crate::kernel32::module::LoadResource(hinstance, hres);
    if hglob.is_null() {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        set_error(crate::types::err::ERROR_RESOURCE_DATA_NOT_FOUND);
        return 0;
    }
    let ptr = crate::kernel32::module::LockResource(hglob) as *const u8;
    if ptr.is_null() || byte_size < 2 {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        set_error(crate::types::err::ERROR_RESOURCE_DATA_NOT_FOUND);
        return 0;
    }

    let mut off = 0usize;
    let mut str_ptr: *const u16 = std::ptr::null();
    let mut str_len = 0usize;
    for i in 0..=index {
        let Some(after_len) = off.checked_add(2) else {
            set_error(e::INVALID_DATA);
            return 0;
        };
        if after_len > byte_size {
            if cch_buffer_max > 0 {
                std::ptr::write_unaligned(lp_buffer, 0);
            } else {
                std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
            }
            set_error(crate::types::err::ERROR_RESOURCE_DATA_NOT_FOUND);
            return 0;
        }
        let entry_len = std::ptr::read_unaligned(ptr.add(off) as *const u16) as usize;
        off = after_len;
        let Some(entry_bytes) = entry_len.checked_mul(2) else {
            set_error(e::INVALID_DATA);
            return 0;
        };
        let Some(after_str) = off.checked_add(entry_bytes) else {
            set_error(e::INVALID_DATA);
            return 0;
        };
        if after_str > byte_size {
            if cch_buffer_max > 0 {
                std::ptr::write_unaligned(lp_buffer, 0);
            } else {
                std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
            }
            set_error(crate::types::err::ERROR_RESOURCE_DATA_NOT_FOUND);
            return 0;
        }
        if i == index {
            str_ptr = ptr.add(off) as *const u16;
            str_len = entry_len;
            break;
        }
        off = after_str;
    }

    if str_len == 0 {
        if cch_buffer_max > 0 {
            std::ptr::write_unaligned(lp_buffer, 0);
        } else {
            std::ptr::write_unaligned(lp_buffer as *mut *const u16, std::ptr::null());
        }
        set_error(e::RESOURCE_NAME_NOT_FOUND);
        return 0;
    }

    if cch_buffer_max == 0 {
        std::ptr::write_unaligned(lp_buffer as *mut *const u16, str_ptr);
        set_error(0);
        return str_len.min(i32::MAX as usize) as i32;
    }

    let copy_len = str_len.min((cch_buffer_max - 1) as usize);
    for i in 0..copy_len {
        let ch = std::ptr::read_unaligned(str_ptr.add(i));
        std::ptr::write_unaligned(lp_buffer.add(i), ch);
    }
    std::ptr::write_unaligned(lp_buffer.add(copy_len), 0);
    set_error(0);
    copy_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rects_and_metrics() {
        unsafe {
            let mut r = RECT::default();
            assert_eq!(IsRectEmpty(&r), TRUE);
            SetRect(&mut r, 0, 0, 10, 5);
            assert_eq!(IsRectEmpty(&r), FALSE);
            let mut i = RECT::default();
            let o = RECT::new(5, 2, 20, 20);
            assert_eq!(IntersectRect(&mut i, &r, &o), TRUE);
            assert_eq!(i, RECT::new(5, 2, 10, 5));
            let mut u = RECT::default();
            UnionRect(&mut u, &r, &o);
            assert_eq!(u, RECT::new(0, 0, 20, 20));
            assert_eq!(PtInRect(&r, 3 | (4u64 << 32)), TRUE);
            assert_eq!(PtInRect(&r, 10), FALSE);
            assert!(GetSystemMetrics(0) > 0);
            assert!(GetSystemMetrics(80) >= 1);
        }
    }

    #[test]
    fn spi_accessibility() {
        unsafe {
            let mut sk = [0xAAu8; 8];
            assert_eq!(SystemParametersInfoA(SPI_GETSTICKYKEYS, 8, sk.as_mut_ptr(), 0), TRUE);
            assert_eq!(&sk, &[8, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(SystemParametersInfoA(SPI_SETSTICKYKEYS, 8, sk.as_mut_ptr(), 0), TRUE);
        }
    }

    #[test]
    fn clipboard_unicode_and_ansi_roundtrip() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::clipboard_sync_enabled(), "unit tests must not touch host desktop clipboard");

        // Verify CRLF <-> LF helpers.
        assert_eq!(win_to_macos_newlines("a\r\nb\rc\nd"), "a\nb\nc\nd");
        assert_eq!(macos_to_win_newlines("a\nb\r\nc\rd"), "a\r\nb\r\nc\r\nd");

        unsafe {
            // 1. Win32 CF_UNICODETEXT -> GlobalLock -> exact UTF-16 & synthesized CF_TEXT roundtrip.
            let sample = "Héllo — 世界\r\nLine 2";
            let wide_z = crate::strings::to_wide_z(sample);
            let h_mem = crate::kernel32::memory::GlobalAlloc(0x0042, wide_z.len() * 2);
            assert!(!h_mem.is_null());
            let locked = crate::kernel32::memory::GlobalLock(h_mem) as *mut u16;
            std::ptr::copy_nonoverlapping(wide_z.as_ptr(), locked, wide_z.len());
            crate::kernel32::memory::GlobalUnlock(h_mem);

            assert_eq!(OpenClipboard(std::ptr::null_mut()), TRUE);
            assert_eq!(EmptyClipboard(), TRUE);
            let seq0 = GetClipboardSequenceNumber();
            assert_eq!(SetClipboardData(CF_UNICODETEXT, h_mem as usize), h_mem as usize);
            assert_eq!(GetClipboardSequenceNumber(), seq0 + 1);
            assert_eq!(CloseClipboard(), TRUE);

            assert_eq!(OpenClipboard(std::ptr::null_mut()), TRUE);
            assert_eq!(IsClipboardFormatAvailable(CF_UNICODETEXT), TRUE);
            assert_eq!(IsClipboardFormatAvailable(CF_TEXT), TRUE);
            assert_eq!(IsClipboardFormatAvailable(CF_OEMTEXT), TRUE);

            // EnumClipboardFormats walks CF_TEXT (1), CF_OEMTEXT (7), CF_UNICODETEXT (13).
            let mut fmts = Vec::new();
            let mut cur = 0u32;
            loop {
                cur = EnumClipboardFormats(cur);
                if cur == 0 {
                    break;
                }
                fmts.push(cur);
            }
            assert_eq!(fmts, vec![CF_TEXT, CF_OEMTEXT, CF_UNICODETEXT]);

            let h_w = GetClipboardData(CF_UNICODETEXT);
            assert_ne!(h_w, 0);
            let p_w = crate::kernel32::memory::GlobalLock(h_w as *mut std::ffi::c_void) as *const u16;
            assert_eq!(read_wide_unaligned(p_w), sample);

            let h_a = GetClipboardData(CF_TEXT);
            assert_ne!(h_a, 0);
            let p_a = crate::kernel32::memory::GlobalLock(h_a as *mut std::ffi::c_void) as *const u8;
            assert_eq!(crate::strings::from_ansi(p_a), sample);

            // 2. Win32 CF_TEXT -> synthesized CF_UNICODETEXT roundtrip.
            let ansi_sample = b"Plain ASCII\r\nRow 2\0";
            let h_ansi = crate::kernel32::memory::GlobalAlloc(0x0042, ansi_sample.len());
            let locked_a = crate::kernel32::memory::GlobalLock(h_ansi) as *mut u8;
            std::ptr::copy_nonoverlapping(ansi_sample.as_ptr(), locked_a, ansi_sample.len());
            assert_eq!(EmptyClipboard(), TRUE);
            assert_eq!(SetClipboardData(CF_TEXT, h_ansi as usize), h_ansi as usize);
            let h_w2 = GetClipboardData(CF_UNICODETEXT);
            let p_w2 = crate::kernel32::memory::GlobalLock(h_w2 as *mut std::ffi::c_void) as *const u16;
            assert_eq!(read_wide_unaligned(p_w2), "Plain ASCII\r\nRow 2");

            // 3. Simulated macOS pasteboard text import converts LF -> CRLF.
            {
                let mut st = clipboard_state().lock().unwrap();
                ingest_macos_text(&mut st, Some("from macOS\nsecond line"));
            }
            let h_w3 = GetClipboardData(CF_UNICODETEXT);
            let p_w3 = crate::kernel32::memory::GlobalLock(h_w3 as *mut std::ffi::c_void) as *const u16;
            assert_eq!(read_wide_unaligned(p_w3), "from macOS\r\nsecond line");

            assert_eq!(EmptyClipboard(), TRUE);
            assert_eq!(IsClipboardFormatAvailable(CF_UNICODETEXT), FALSE);
            assert_eq!(CloseClipboard(), TRUE);
            let _ = crate::kernel32::memory::GlobalFree(h_mem);
            let _ = crate::kernel32::memory::GlobalFree(h_ansi);
        }

        // 4. High-level clipboard_set_text / clipboard_get_text helper roundtrip.
        clipboard_set_text("Helper\nUnicode: 日本語");
        assert_eq!(clipboard_get_text().as_deref(), Some("Helper\r\nUnicode: 日本語"));
        unsafe {
            assert_eq!(OpenClipboard(std::ptr::null_mut()), TRUE);
            assert_eq!(EmptyClipboard(), TRUE);
            assert_eq!(CloseClipboard(), TRUE);
        }
    }

    #[test]
    fn device_notification_posts_wm_devicechange_on_hotplug() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| {
            let cls = crate::strings::to_wide_z("DevNotifyTestWnd");
            let wc = super::super::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<super::super::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::super::WNDPROC_HANDLE_DEF_WINDOW_W,
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
            unsafe {
                let _ = super::super::class::RegisterClassExW(&wc);
                let hwnd = super::super::window::CreateWindowExW(
                    0,
                    cls.as_ptr(),
                    std::ptr::null(),
                    0,
                    0,
                    0,
                    320,
                    200,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                assert!(!hwnd.is_null());

                let filter = [32u32, DBT_DEVTYP_DEVICEINTERFACE, 0, 0, 0, 0, 0, 0];
                let hdev = RegisterDeviceNotificationW(hwnd as usize, filter.as_ptr(), 0);
                assert_ne!(hdev, 0);

                notify_device_change(true);
                notify_device_change(false);

                let mut events = Vec::new();
                let mut msg = super::super::MSG::default();
                for _ in 0..16 {
                    if super::super::message::PeekMessageW(
                        &mut msg,
                        hwnd,
                        super::super::wm::DEVICECHANGE,
                        super::super::wm::DEVICECHANGE,
                        1,
                    ) == 0
                    {
                        break;
                    }
                    events.push((msg.wparam, msg.lparam));
                }
                assert_eq!(events.len(), 4);
                assert_eq!(events[0].0, DBT_DEVICEARRIVAL);
                assert_ne!(events[0].1, 0);
                assert_eq!(events[1], (DBT_DEVNODES_CHANGED, 0));
                assert_eq!(events[2].0, DBT_DEVICEREMOVECOMPLETE);
                assert_ne!(events[2].1, 0);
                assert_eq!(events[3], (DBT_DEVNODES_CHANGED, 0));

                assert_eq!(UnregisterDeviceNotification(hdev), TRUE);

                notify_device_change(true);
                let mut after = Vec::new();
                for _ in 0..16 {
                    if super::super::message::PeekMessageW(
                        &mut msg,
                        hwnd,
                        super::super::wm::DEVICECHANGE,
                        super::super::wm::DEVICECHANGE,
                        1,
                    ) == 0
                    {
                        break;
                    }
                    after.push((msg.wparam, msg.lparam));
                }
                // After unregistering, only the top-level DBT_DEVNODES_CHANGED broadcast arrives.
                assert_eq!(after, vec![(DBT_DEVNODES_CHANGED, 0)]);

                let _ = super::super::window::DestroyWindow(hwnd);
            }
        })
        .join()
        .unwrap();
    }

    #[test]
    fn windows_hooks_chaining_suppression_getmessage_callwndproc_and_unhook() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        static EVENTS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
        static SUPPRESS_VK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

        unsafe extern "win64" fn kbd_hook_outer(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            EVENTS.lock().unwrap().push(format!("kbd_outer:{code}:{wparam:#x}"));
            if code >= 0 && wparam == SUPPRESS_VK.load(Ordering::Acquire) {
                return 1; // suppress without calling next hook
            }
            CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn kbd_hook_inner(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            EVENTS.lock().unwrap().push(format!("kbd_inner:{code}:{wparam:#x}"));
            CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn getmsg_hook_outer(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            let m = (lparam as *const MSG).read_unaligned();
            EVENTS.lock().unwrap().push(format!("getmsg_outer:{code}:{wparam}:{:#x}", m.message));
            // Mutate wparam on WM_USER + 10 to prove in-place MSG modification.
            if m.message == super::super::wm::USER + 10 {
                let mut mutated = m;
                mutated.wparam = 0xCAFE;
                (lparam as *mut MSG).write_unaligned(mutated);
            }
            CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn getmsg_hook_inner(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            let m = (lparam as *const MSG).read_unaligned();
            EVENTS.lock().unwrap().push(format!("getmsg_inner:{code}:{wparam}:{:#x}:{:#x}", m.message, m.wparam));
            CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn cwp_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
            let cwp = (lparam as *const CWPSTRUCT).read_unaligned();
            EVENTS
                .lock()
                .unwrap()
                .push(format!("cwp:{code}:{wparam}:{:#x}:{:#x}", cwp.message, cwp.wparam));
            CallNextHookEx(0, code, wparam, lparam)
        }

        unsafe extern "win64" fn win_event_cb(
            _h: usize,
            event: u32,
            hwnd: usize,
            _id_obj: i32,
            _id_child: i32,
            _tid: u32,
            _time: u32,
        ) {
            EVENTS.lock().unwrap().push(format!("winevt:{event:#x}:{hwnd:#x}"));
        }

        std::thread::spawn(|| unsafe {
            EVENTS.lock().unwrap().clear();
            SUPPRESS_VK.store(0, Ordering::Release);
            let tid = crate::teb::thread_id();

            // Validation errors.
            assert_eq!(SetWindowsHookExW(99, kbd_hook_outer as *const () as usize, std::ptr::null_mut(), tid), 0);
            assert_eq!(SetWindowsHookExW(WH_KEYBOARD, 0, std::ptr::null_mut(), tid), 0);
            assert_eq!(UnhookWindowsHookEx(0), FALSE);
            assert_eq!(UnhookWindowsHookEx(0xDEAD_BEEF), FALSE);

            // Install inner first, then outer -> LIFO dispatch calls outer -> CallNextHookEx -> inner.
            let h_kbd_inner = SetWindowsHookExW(
                WH_KEYBOARD,
                kbd_hook_inner as *const () as usize,
                std::ptr::null_mut(),
                tid,
            );
            let h_kbd_outer = SetWindowsHookExA(
                WH_KEYBOARD,
                kbd_hook_outer as *const () as usize,
                std::ptr::null_mut(),
                tid,
            );
            // A hook registered for a different thread must NOT fire on this thread.
            let h_kbd_other = SetWindowsHookExW(
                WH_KEYBOARD,
                kbd_hook_inner as *const () as usize,
                std::ptr::null_mut(),
                tid.wrapping_add(12345),
            );
            let h_gm_inner = SetWindowsHookExW(
                WH_GETMESSAGE,
                getmsg_hook_inner as *const () as usize,
                std::ptr::null_mut(),
                tid,
            );
            let h_gm_outer = SetWindowsHookExW(
                WH_GETMESSAGE,
                getmsg_hook_outer as *const () as usize,
                std::ptr::null_mut(),
                0, // global tid == 0
            );
            let h_cwp = SetWindowsHookExW(
                WH_CALLWNDPROC,
                cwp_hook as *const () as usize,
                std::ptr::null_mut(),
                tid,
            );
            let h_wevt = SetWinEventHook(0x0003, 0x8005, std::ptr::null_mut(), win_event_cb as *const () as usize, 0, tid, 0);
            assert_ne!(h_kbd_inner, 0);
            assert_ne!(h_kbd_outer, 0);
            assert_ne!(h_kbd_other, 0);
            assert_ne!(h_gm_inner, 0);
            assert_ne!(h_gm_outer, 0);
            assert_ne!(h_cwp, 0);
            assert_ne!(h_wevt, 0);

            // 1. Allowed keydown traverses outer -> inner -> WH_GETMESSAGE (outer -> inner) and is returned.
            super::super::message::PostMessageW(std::ptr::null_mut(), super::super::wm::KEYDOWN, b'A' as usize, 1 | (0x1E << 16));
            let mut m = MSG::default();
            assert_eq!(super::super::message::PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, super::super::message::PM_REMOVE), TRUE);
            assert_eq!(m.message, super::super::wm::KEYDOWN);
            assert_eq!(m.wparam, b'A' as usize);
            {
                let evs = EVENTS.lock().unwrap().clone();
                assert_eq!(
                    evs,
                    vec![
                        "kbd_outer:0:0x41",
                        "kbd_inner:0:0x41",
                        "getmsg_outer:0:1:0x100",
                        "getmsg_inner:0:1:0x100:0x41",
                    ]
                );
            }

            // 2. Suppressed keydown (VK 'B') stops at outer, does NOT reach inner or WH_GETMESSAGE,
            //    and is discarded so PeekMessage retrieves the subsequent WM_USER+10 message (mutated by WH_GETMESSAGE).
            EVENTS.lock().unwrap().clear();
            SUPPRESS_VK.store(b'B' as usize, Ordering::Release);
            super::super::message::PostMessageW(std::ptr::null_mut(), super::super::wm::KEYDOWN, b'B' as usize, 1 | (0x30 << 16));
            super::super::message::PostMessageW(std::ptr::null_mut(), super::super::wm::USER + 10, 0x1111, 0);
            assert_eq!(super::super::message::PeekMessageW(&mut m, std::ptr::null_mut(), 0, 0, super::super::message::PM_REMOVE), TRUE);
            assert_eq!(m.message, super::super::wm::USER + 10);
            assert_eq!(m.wparam, 0xCAFE);
            {
                let evs = EVENTS.lock().unwrap().clone();
                assert_eq!(
                    evs,
                    vec![
                        "kbd_outer:0:0x42",
                        "getmsg_outer:0:1:0x40a",
                        "getmsg_inner:0:1:0x40a:0xcafe",
                    ]
                );
            }

            // 3. Unhook outer keyboard hook -> VK 'B' is no longer suppressed and reaches inner hook directly.
            EVENTS.lock().unwrap().clear();
            assert_eq!(UnhookWindowsHookEx(h_kbd_outer), TRUE);
            assert_eq!(UnhookWindowsHookEx(h_kbd_outer), FALSE);
            super::super::message::PostMessageW(std::ptr::null_mut(), super::super::wm::KEYDOWN, b'B' as usize, 1 | (0x30 << 16));
            assert_eq!(super::super::message::GetMessageW(&mut m, std::ptr::null_mut(), 0, 0), TRUE);
            assert_eq!(m.message, super::super::wm::KEYDOWN);
            assert_eq!(m.wparam, b'B' as usize);
            {
                let evs = EVENTS.lock().unwrap().clone();
                assert_eq!(
                    evs,
                    vec![
                        "kbd_inner:0:0x42",
                        "getmsg_outer:0:1:0x100",
                        "getmsg_inner:0:1:0x100:0x42",
                    ]
                );
            }

            // 4. WH_CALLWNDPROC fires on SendMessageW with exact CWPSTRUCT fields.
            let cls = crate::strings::to_wide_z("HookTestWndCls");
            let wc = super::super::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<super::super::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: super::super::WNDPROC_HANDLE_DEF_WINDOW_W,
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
            let _ = super::super::class::RegisterClassExW(&wc);
            let hwnd = super::super::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                0,
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
            EVENTS.lock().unwrap().clear();
            let _ = super::super::message::SendMessageW(hwnd, super::super::wm::USER + 77, 0x4242, 0);
            assert!(EVENTS.lock().unwrap().iter().any(|s| s == "cwp:0:1:0x44d:0x4242"));

            // 5. WinEvent hook fires and cleanly unhooks.
            EVENTS.lock().unwrap().clear();
            dispatch_win_event(0x8005, hwnd as usize, 0, 0);
            assert_eq!(EVENTS.lock().unwrap().clone(), vec![format!("winevt:0x8005:{:#x}", hwnd as usize)]);
            assert_eq!(UnhookWinEvent(h_wevt), TRUE);
            assert_eq!(UnhookWinEvent(h_wevt), FALSE);
            EVENTS.lock().unwrap().clear();
            dispatch_win_event(0x8005, hwnd as usize, 0, 0);
            assert!(EVENTS.lock().unwrap().is_empty());

            // Cleanup all remaining hooks + window.
            assert_eq!(UnhookWindowsHookEx(h_kbd_inner), TRUE);
            assert_eq!(UnhookWindowsHookEx(h_kbd_other), TRUE);
            assert_eq!(UnhookWindowsHookEx(h_gm_inner), TRUE);
            assert_eq!(UnhookWindowsHookEx(h_gm_outer), TRUE);
            assert_eq!(UnhookWindowsHookEx(h_cwp), TRUE);
            let _ = super::super::window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();
    }

    /// Build a minimal mapped PE32+ image containing an `.rsrc` `RT_STRING` directory
    /// with the supplied `(block_id, [16 strings])` blocks.
    fn build_pe_with_string_blocks(blocks: &[(u16, [&str; 16])]) -> Vec<u8> {
        const NT_OFF: usize = 0x80;
        const RSRC_RVA: usize = 0x1000;
        let mut img = vec![0u8; 0x4000];
        let wr16 = |buf: &mut [u8], off: usize, v: u16| {
            buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
        };
        let wr32 = |buf: &mut [u8], off: usize, v: u32| {
            buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
        };

        // DOS + NT PE32+ headers sufficient for `kernel32::module::FindResourceW`.
        wr16(&mut img, 0, 0x5A4D);
        wr32(&mut img, 0x3C, NT_OFF as u32);
        wr32(&mut img, NT_OFF, 0x0000_4550);
        let opt = NT_OFF + 0x18;
        wr16(&mut img, opt, 0x020B);
        let img_size = img.len() as u32;
        wr32(&mut img, opt + 56, img_size); // SizeOfImage
        wr32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes
        wr32(&mut img, opt + 112 + 2 * 8, RSRC_RVA as u32); // DataDirectory[2].RVA
        wr32(&mut img, opt + 112 + 2 * 8 + 4, 0x2000); // DataDirectory[2].Size

        // Resource tree layout inside RSRC_RVA:
        // [0x000] Root dir (1 entry: RT_STRING = 6 -> type_dir)
        // [0x020] Type dir (blocks.len() entries: block_id -> lang_dir[i])
        // [lang_base + i*0x20] Lang dir (1 entry: 0x0409 -> data_entry[i])
        // [data_entry_base + i*0x10] IMAGE_RESOURCE_DATA_ENTRY (rva, size, 0, 0)
        // [payload_cursor..] Packed RT_STRING blocks
        let type_dir_off = 0x20usize;
        let type_dir_size = 16 + blocks.len() * 8;
        let lang_base = (type_dir_off + type_dir_size + 15) & !15;
        let data_entry_base = lang_base + blocks.len() * 0x20;
        let mut payload_off = (data_entry_base + blocks.len() * 0x10 + 15) & !15;

        // Root directory: 1 integer ID entry (RT_STRING = 6).
        wr16(&mut img, RSRC_RVA + 14, 1);
        wr32(&mut img, RSRC_RVA + 16, RT_STRING as u32);
        wr32(&mut img, RSRC_RVA + 20, 0x8000_0000 | type_dir_off as u32);

        // Type directory: one entry per block_id.
        wr16(&mut img, RSRC_RVA + type_dir_off + 14, blocks.len() as u16);
        for (i, (block_id, entries)) in blocks.iter().enumerate() {
            let lang_off = lang_base + i * 0x20;
            let data_ent_off = data_entry_base + i * 0x10;

            wr32(&mut img, RSRC_RVA + type_dir_off + 16 + i * 8, *block_id as u32);
            wr32(&mut img, RSRC_RVA + type_dir_off + 20 + i * 8, 0x8000_0000 | lang_off as u32);

            // Lang directory: 1 entry (en-US 0x0409 -> leaf data entry).
            wr16(&mut img, RSRC_RVA + lang_off + 14, 1);
            wr32(&mut img, RSRC_RVA + lang_off + 16, 0x0409);
            wr32(&mut img, RSRC_RVA + lang_off + 20, data_ent_off as u32);

            let mut block_bytes = Vec::new();
            for s in entries {
                let units: Vec<u16> = s.encode_utf16().collect();
                block_bytes.extend_from_slice(&(units.len() as u16).to_le_bytes());
                for u in units {
                    block_bytes.extend_from_slice(&u.to_le_bytes());
                }
            }
            let data_rva = RSRC_RVA + payload_off;
            img[data_rva..data_rva + block_bytes.len()].copy_from_slice(&block_bytes);
            wr32(&mut img, RSRC_RVA + data_ent_off, data_rva as u32);
            wr32(&mut img, RSRC_RVA + data_ent_off + 4, block_bytes.len() as u32);
            payload_off = (payload_off + block_bytes.len() + 15) & !15;
        }
        img
    }

    #[test]
    fn load_string_w_blocks_truncation_and_zero_buffer_pointer() {
        let mut blk1 = [""; 16];
        blk1[0] = "ZeroId";
        blk1[5] = "Hello, Win32!";
        blk1[15] = "LastInBlock1";

        let mut blk2 = [""; 16];
        blk2[0] = "BlockTwoFirst";
        blk2[3] = "Héllo — 世界";

        let img = build_pe_with_string_blocks(&[(1, blk1), (2, blk2)]);

        std::thread::spawn(move || unsafe {
            let hinst = img.as_ptr() as HINSTANCE;
            let img_len = img.len();
            // 1. Block 1 entries (IDs 0, 5, 15).
            let mut buf = [0xBEEFu16; 64];
            assert_eq!(LoadStringW(hinst, 0, buf.as_mut_ptr(), buf.len() as i32), 6);
            assert_eq!(read_wide_unaligned(buf.as_ptr()), "ZeroId");

            assert_eq!(LoadStringW(hinst, 5, buf.as_mut_ptr(), buf.len() as i32), 13);
            assert_eq!(read_wide_unaligned(buf.as_ptr()), "Hello, Win32!");

            assert_eq!(LoadStringW(hinst, 15, buf.as_mut_ptr(), buf.len() as i32), 12);
            assert_eq!(read_wide_unaligned(buf.as_ptr()), "LastInBlock1");

            // 2. Block 2 entries (ID 16 -> block 2 index 0; ID 19 -> block 2 index 3).
            assert_eq!(LoadStringW(hinst, 16, buf.as_mut_ptr(), buf.len() as i32), 13);
            assert_eq!(read_wide_unaligned(buf.as_ptr()), "BlockTwoFirst");

            let unicode_expected: Vec<u16> = "Héllo — 世界".encode_utf16().collect();
            assert_eq!(
                LoadStringW(hinst, 19, buf.as_mut_ptr(), buf.len() as i32),
                unicode_expected.len() as i32
            );
            assert_eq!(read_wide_unaligned(buf.as_ptr()), "Héllo — 世界");

            // 3. Truncation: cch_buffer_max = 6 copies 5 chars + NUL; cch_buffer_max = 1 writes only NUL.
            let mut small = [0xAAAAu16; 8];
            assert_eq!(LoadStringW(hinst, 5, small.as_mut_ptr(), 6), 5);
            assert_eq!(read_wide_unaligned(small.as_ptr()), "Hello");
            assert_eq!(small[5], 0);
            assert_eq!(small[6], 0xAAAA);

            let mut one = [0xAAAAu16; 2];
            assert_eq!(LoadStringW(hinst, 5, one.as_mut_ptr(), 1), 0);
            assert_eq!(one[0], 0);
            assert_eq!(one[1], 0xAAAA);

            // 4. Zero-buffer read-only pointer mode (`cch_buffer_max == 0`):
            //    writes `*const u16` into `lp_buffer` pointing directly inside the resource block.
            let mut ro_ptr: *const u16 = std::ptr::null();
            let ro_len = LoadStringW(
                hinst,
                19,
                &mut ro_ptr as *mut *const u16 as LPWSTR,
                0,
            );
            assert_eq!(ro_len, unicode_expected.len() as i32);
            assert!(!ro_ptr.is_null());
            let img_lo = hinst as usize;
            let img_hi = img_lo + img_len;
            assert!(
                (ro_ptr as usize) >= img_lo && (ro_ptr as usize) < img_hi,
                "zero-buffer pointer must point into the PE resource block"
            );
            let ro_slice: Vec<u16> = (0..ro_len as usize)
                .map(|i| std::ptr::read_unaligned(ro_ptr.add(i)))
                .collect();
            assert_eq!(String::from_utf16(&ro_slice).unwrap(), "Héllo — 世界");

            // 5. Empty slot in existing block and missing block return 0 with resource error.
            let mut empty_out: *const u16 = 0x1234usize as *const u16;
            assert_eq!(
                LoadStringW(hinst, 2, &mut empty_out as *mut *const u16 as LPWSTR, 0),
                0
            );
            assert!(empty_out.is_null());
            assert_eq!(crate::teb::last_error(), e::RESOURCE_NAME_NOT_FOUND);

            buf[0] = b'X' as u16;
            assert_eq!(LoadStringW(hinst, 2, buf.as_mut_ptr(), 16), 0);
            assert_eq!(buf[0], 0);
            assert_eq!(crate::teb::last_error(), e::RESOURCE_NAME_NOT_FOUND);

            assert_eq!(LoadStringW(hinst, 999, buf.as_mut_ptr(), 16), 0);
            assert_eq!(crate::teb::last_error(), e::RESOURCE_NAME_NOT_FOUND);

            // 6. Invalid parameters: null buffer or negative capacity.
            assert_eq!(LoadStringW(hinst, 5, std::ptr::null_mut(), 16), 0);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
            assert_eq!(LoadStringW(hinst, 5, buf.as_mut_ptr(), -1), 0);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);
        })
        .join()
        .unwrap();
    }
}
