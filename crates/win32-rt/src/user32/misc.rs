//! user32 odds and ends: rectangles, system metrics, SystemParametersInfo,
//! window stations, device/hotkey registration, clipboard, dialogs,
//! DPI-awareness switches, MessageBox.

use super::*;
use crate::types::*;
use crate::va::win64_varargs_thunk;
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
/// ones are Windows 10 defaults at 96 DPI. Per Microsoft Learn, unknown
/// `SM_*` indices return 0 without setting last error.
pub unsafe extern "win64" fn GetSystemMetrics(index: i32) -> i32 {
    let p = super::monitor::primary_rect();
    let work = super::monitor::primary_work_area();
    let v = virtual_screen();
    let cap = window::nc_insets(ws::CAPTION, 0).1;
    let is_ios = cfg!(target_os = "ios");
    match index {
        0 => p.width(),                                                   // SM_CXSCREEN
        1 => p.height(),                                                  // SM_CYSCREEN
        2 | 3 => 17,                                                      // SM_CXVSCROLL, SM_CYHSCROLL
        4 => cap,                                                         // SM_CYCAPTION
        5 | 6 => 1,                                                       // SM_CXBORDER, SM_CYBORDER
        7 | 8 => 3,                                                       // SM_CXDLGFRAME / SM_CXFIXEDFRAME
        9 | 10 => 17,                                                     // SM_CYVTHUMB, SM_CXHTHUMB
        11 | 12 => 32,                                                    // SM_CXICON, SM_CYICON
        13 | 14 => 32,                                                    // SM_CXCURSOR, SM_CYCURSOR
        15 => 20,                                                         // SM_CYMENU
        16 => work.width(),                                               // SM_CXFULLSCREEN
        17 => work.height() - cap,                                        // SM_CYFULLSCREEN
        18 => 0,                                                          // SM_CYKANJIWINDOW
        19 => if is_ios { 0 } else { 1 },                                 // SM_MOUSEPRESENT
        20 | 21 => 17,                                                    // SM_CYVSCROLL, SM_CXHSCROLL
        22 => 0,                                                          // SM_DEBUG
        23 => 0,                                                          // SM_SWAPBUTTON
        24..=27 => 0,                                                     // reserved
        28 | 29 => 136,                                                   // SM_CXMIN, SM_CYMIN
        30 => 36,                                                         // SM_CXSIZE
        31 => cap,                                                        // SM_CYSIZE
        32 | 33 => 4,                                                     // SM_CXFRAME / SM_CXSIZEFRAME
        34 => 136,                                                        // SM_CXMINTRACK
        35 => 39,                                                         // SM_CYMINTRACK
        36 | 37 => 4,                                                     // SM_CXDOUBLECLK, SM_CYDOUBLECLK
        38 | 39 => 75,                                                    // SM_CXICONSPACING, SM_CYICONSPACING
        40 => 0,                                                          // SM_MENUDROPALIGNMENT
        41 => 0,                                                          // SM_PENWINDOWS
        42 => 1,                                                          // SM_DBCSENABLED
        43 => if is_ios { 0 } else { 5 },                                 // SM_CMOUSEBUTTONS
        44 => 0,                                                          // SM_SECURE
        45 | 46 => 2,                                                     // SM_CXEDGE, SM_CYEDGE
        47 => 160,                                                        // SM_CXMINSPACING
        48 => 24,                                                         // SM_CYMINSPACING
        49 | 50 => 16,                                                    // SM_CXSMICON, SM_CYSMICON
        51 => 22,                                                         // SM_CYSMCAPTION
        52 | 53 => 22,                                                    // SM_CXSMSIZE, SM_CYSMSIZE
        54 | 55 => 19,                                                    // SM_CXMENUSIZE, SM_CYMENUSIZE
        56 => 8,                                                          // SM_ARRANGE (ARW_HIDE)
        57 => 160,                                                        // SM_CXMINIMIZED
        58 => 24,                                                         // SM_CYMINIMIZED
        59 => v.width() + 16,                                             // SM_CXMAXTRACK
        60 => v.height() + 16,                                            // SM_CYMAXTRACK
        61 => work.width(),                                               // SM_CXMAXIMIZED
        62 => work.height(),                                              // SM_CYMAXIMIZED
        63 => 0x1,                                                        // SM_NETWORK
        64..=66 => 0,                                                     // reserved
        67 => 0,                                                          // SM_CLEANBOOT
        68 | 69 => 4,                                                     // SM_CXDRAG, SM_CYDRAG
        70 => 0,                                                          // SM_SHOWSOUNDS
        71 | 72 => 15,                                                    // SM_CXMENUCHECK, SM_CYMENUCHECK
        73 => 0,                                                          // SM_SLOWMACHINE
        74 => 0,                                                          // SM_MIDEASTENABLED
        75 => if is_ios { 0 } else { 1 },                                 // SM_MOUSEWHEELPRESENT
        76 => v.left,                                                     // SM_XVIRTUALSCREEN
        77 => v.top,                                                      // SM_YVIRTUALSCREEN
        78 => v.width(),                                                  // SM_CXVIRTUALSCREEN
        79 => v.height(),                                                 // SM_CYVIRTUALSCREEN
        80 => crate::macos::display::monitors().len() as i32,             // SM_CMONITORS
        81 => 1,                                                          // SM_SAMEDISPLAYFORMAT
        82 => 1,                                                          // SM_IMMENABLED
        83 | 84 => 1,                                                     // SM_CXFOCUSBORDER, SM_CYFOCUSBORDER
        86 => if is_ios { 1 } else { 0 },                                 // SM_TABLETPC
        87 => 0,                                                          // SM_MEDIACENTER
        88 => 0,                                                          // SM_STARTER
        89 => 0,                                                          // SM_SERVERR2
        91 => if is_ios { 0 } else { 1 },                                 // SM_MOUSEHORIZONTALWHEELPRESENT
        92 => 0,                                                          // SM_CXPADDEDBORDER
        94 => if is_ios { 0xC1 } else { 0 },                              // SM_DIGITIZER
        95 => if is_ios { 10 } else { 0 },                                // SM_MAXIMUMTOUCHES
        0x1000 => 0,                                                      // SM_REMOTESESSION
        0x2000 => 0,                                                      // SM_SHUTTINGDOWN
        0x2001 => 0,                                                      // SM_REMOTECONTROL
        0x2003 => if is_ios { 0 } else { 1 },                             // SM_CONVERTIBLESLATEMODE (1 = not slate)
        0x2004 => 0,                                                      // SM_SYSTEMDOCKED
        _ => 0,                                                           // Undefined index: Windows returns 0
    }
}

pub unsafe extern "win64" fn GetSystemMetricsForDpi(index: i32, dpi: u32) -> i32 {
    let base = GetSystemMetrics(index);
    if dpi < 48 || dpi == 96 {
        return base;
    }
    // Scale UI element dimensions linearly with DPI; screen pixel metrics stay unscaled.
    match index {
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 20 | 21 | 28 | 29 | 30
        | 31 | 32 | 33 | 34 | 35 | 38 | 39 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54
        | 55 | 57 | 58 | 71 | 72 | 83 | 84 | 92 => {
            let scaled = (base as i64 * dpi as i64 + 48) / 96;
            scaled as i32
        }
        _ => base,
    }
}

// ---------------------------------------------------------------------------
// SystemParametersInfo
// ---------------------------------------------------------------------------

const SPI_GETBEEP: u32 = 0x0001;
const SPI_SETBEEP: u32 = 0x0002;
const SPI_GETMOUSE: u32 = 0x0003;
const SPI_SETMOUSE: u32 = 0x0004;
const SPI_GETBORDER: u32 = 0x0005;
const SPI_SETBORDER: u32 = 0x0006;
const SPI_GETKEYBOARDSPEED: u32 = 0x000A;
const SPI_SETKEYBOARDSPEED: u32 = 0x000B;
const SPI_GETSCREENSAVETIMEOUT: u32 = 0x000E;
const SPI_SETSCREENSAVETIMEOUT: u32 = 0x000F;
const SPI_GETSCREENSAVEACTIVE: u32 = 0x0010;
const SPI_SETSCREENSAVEACTIVE: u32 = 0x0011;
const SPI_GETKEYBOARDDELAY: u32 = 0x0016;
const SPI_SETKEYBOARDDELAY: u32 = 0x0017;
const SPI_GETICONTITLEWRAP: u32 = 0x0019;
const SPI_GETMENUDROPALIGNMENT: u32 = 0x001B;
const SPI_GETICONTITLELOGFONT: u32 = 0x001F;
const SPI_GETFASTTASKSWITCH: u32 = 0x0023;
const SPI_SETDRAGFULLWINDOWS: u32 = 0x0025;
const SPI_GETDRAGFULLWINDOWS: u32 = 0x0026;
const SPI_GETNONCLIENTMETRICS: u32 = 0x0029;
const SPI_GETMINIMIZEDMETRICS: u32 = 0x002B;
const SPI_GETICONMETRICS: u32 = 0x002D;
const SPI_GETWORKAREA: u32 = 0x0030;
const SPI_GETFILTERKEYS: u32 = 0x0032;
const SPI_SETFILTERKEYS: u32 = 0x0033;
const SPI_GETTOGGLEKEYS: u32 = 0x0034;
const SPI_SETTOGGLEKEYS: u32 = 0x0035;
const SPI_GETMOUSEKEYS: u32 = 0x0036;
const SPI_SETMOUSEKEYS: u32 = 0x0037;
const SPI_GETSHOWSOUNDS: u32 = 0x0038;
const SPI_GETSTICKYKEYS: u32 = 0x003A;
const SPI_SETSTICKYKEYS: u32 = 0x003B;
const SPI_GETSOUNDSENTRY: u32 = 0x0040;
const SPI_GETHIGHCONTRAST: u32 = 0x0042;
const SPI_GETSCREENREADER: u32 = 0x0046;
const SPI_GETANIMATION: u32 = 0x0048;
const SPI_GETFONTSMOOTHING: u32 = 0x004A;
const SPI_GETSNAPTODEFBUTTON: u32 = 0x005F;
const SPI_GETMOUSEHOVERWIDTH: u32 = 0x0062;
const SPI_GETMOUSEHOVERHEIGHT: u32 = 0x0064;
const SPI_GETMOUSEHOVERTIME: u32 = 0x0066;
const SPI_GETWHEELSCROLLLINES: u32 = 0x0068;
const SPI_SETWHEELSCROLLLINES: u32 = 0x0069;
const SPI_GETWHEELSCROLLCHARS: u32 = 0x006C;
const SPI_SETWHEELSCROLLCHARS: u32 = 0x006D;
const SPI_GETMOUSESPEED: u32 = 0x0070;
const SPI_SETMOUSESPEED: u32 = 0x0071;
const SPI_GETSCREENSAVERRUNNING: u32 = 0x0072;
const SPI_GETDESKWALLPAPER: u32 = 0x0073;
const SPI_GETWINARRANGING: u32 = 0x0082;
const SPI_GETDOCKMOVING: u32 = 0x0090;
const SPI_GETKEYBOARDCUES: u32 = 0x100A;
const SPI_GETMOUSEVANISH: u32 = 0x1020;
const SPI_GETFLATMENU: u32 = 0x1022;
const SPI_GETDROPSHADOW: u32 = 0x1024;
const SPI_GETUIEFFECTS: u32 = 0x103E;
const SPI_GETCLIENTAREAANIMATION: u32 = 0x1042;
const SPI_GETFOREGROUNDLOCKTIMEOUT: u32 = 0x2000;
const SPI_SETFOREGROUNDLOCKTIMEOUT: u32 = 0x2001;
const SPI_GETACTIVEWNDTRKTIMEOUT: u32 = 0x2002;
const SPI_GETFOREGROUNDFLASHCOUNT: u32 = 0x2004;
const SPI_SETFOREGROUNDFLASHCOUNT: u32 = 0x2005;
const SPI_GETCARETWIDTH: u32 = 0x2006;
const SPI_SETCARETWIDTH: u32 = 0x2007;
const SPI_GETFONTSMOOTHINGTYPE: u32 = 0x200A;
const SPI_GETFONTSMOOTHINGCONTRAST: u32 = 0x200C;
const SPI_GETFONTSMOOTHINGORIENTATION: u32 = 0x2012;
const SPI_GETMESSAGEDURATION: u32 = 0x2016;

struct SpiState {
    beep: u32,
    mouse: [i32; 3],
    border: u32,
    kbd_speed: u32,
    kbd_delay: u32,
    screensave_timeout: u32,
    screensave_active: u32,
    drag_full_windows: u32,
    mouse_speed: u32,
    wheel_lines: u32,
    wheel_chars: u32,
    fg_lock_timeout: u32,
    fg_flash_count: u32,
    caret_width: u32,
    sticky_flags: u32,
    toggle_flags: u32,
    filter_keys: [u32; 5],
    mouse_keys: [u32; 6],
}

fn spi_state() -> &'static std::sync::Mutex<SpiState> {
    static S: std::sync::OnceLock<std::sync::Mutex<SpiState>> = std::sync::OnceLock::new();
    S.get_or_init(|| {
        std::sync::Mutex::new(SpiState {
            beep: 1,
            mouse: [6, 10, 1],
            border: 1,
            kbd_speed: 31,
            kbd_delay: 1,
            screensave_timeout: 600,
            screensave_active: 0,
            drag_full_windows: 1,
            mouse_speed: 10,
            wheel_lines: 3,
            wheel_chars: 3,
            fg_lock_timeout: 200_000,
            fg_flash_count: 7,
            caret_width: 1,
            sticky_flags: 0,
            toggle_flags: 0,
            filter_keys: [0; 5],
            mouse_keys: [0; 6],
        })
    })
}

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

const _: () = assert!(std::mem::size_of::<LogFontA>() == 60);
const _: () = assert!(std::mem::size_of::<LogFontW>() == 92);
const _: () = assert!(std::mem::size_of::<NonClientMetricsA>() == 344);
const _: () = assert!(std::mem::size_of::<NonClientMetricsW>() == 504);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, lf_caption_font) == 24);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, i_sm_caption_width) == 84);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, lf_sm_caption_font) == 92);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, i_menu_width) == 152);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, lf_menu_font) == 160);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, lf_status_font) == 220);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, lf_message_font) == 280);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsA, i_padded_border_width) == 340);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, lf_caption_font) == 24);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, i_sm_caption_width) == 116);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, lf_sm_caption_font) == 124);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, i_menu_width) == 216);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, lf_menu_font) == 224);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, lf_status_font) == 316);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, lf_message_font) == 408);
const _: () = assert!(std::mem::offset_of!(NonClientMetricsW, i_padded_border_width) == 500);

#[inline]
fn scale_dpi(val: i32, dpi: u32) -> i32 {
    if dpi < 48 || dpi == 96 {
        val
    } else if val >= 0 {
        ((val as i64 * dpi as i64 + 48) / 96) as i32
    } else {
        -((((-val) as i64 * dpi as i64 + 48) / 96) as i32)
    }
}

unsafe fn spi(action: u32, uparam: u32, pv: *mut u8, wide: bool, dpi: u32) -> BOOL {
    let put_u32 = |v: u32| {
        if pv.is_null() {
            set_error(e::INVALID_PARAMETER);
            return FALSE;
        }
        std::ptr::write_unaligned(pv as *mut u32, v);
        TRUE
    };
    match action {
        SPI_GETBEEP => put_u32(spi_state().lock().unwrap().beep),
        SPI_SETBEEP => {
            spi_state().lock().unwrap().beep = (uparam != 0) as u32;
            TRUE
        }
        SPI_GETWORKAREA => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv as *mut RECT, super::monitor::primary_work_area());
            TRUE
        }
        // Accessibility structs: { UINT cbSize; DWORD dwFlags; ... }.
        SPI_GETSTICKYKEYS | SPI_GETTOGGLEKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let st = spi_state().lock().unwrap();
            let flags = if action == SPI_GETSTICKYKEYS { st.sticky_flags } else { st.toggle_flags };
            std::ptr::write_unaligned(pv as *mut u32, 8);
            std::ptr::write_unaligned(pv.add(4) as *mut u32, flags);
            TRUE
        }
        SPI_SETSTICKYKEYS | SPI_SETTOGGLEKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let flags = std::ptr::read_unaligned(pv.add(4) as *const u32);
            let mut st = spi_state().lock().unwrap();
            if action == SPI_SETSTICKYKEYS {
                st.sticky_flags = flags;
            } else {
                st.toggle_flags = flags;
            }
            TRUE
        }
        SPI_GETFILTERKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let fk = spi_state().lock().unwrap().filter_keys;
            std::ptr::write_unaligned(pv as *mut u32, 24);
            for (i, &w) in fk.iter().enumerate() {
                std::ptr::write_unaligned(pv.add(4 + i * 4) as *mut u32, w);
            }
            TRUE
        }
        SPI_SETFILTERKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let mut fk = [0u32; 5];
            for (i, slot) in fk.iter_mut().enumerate() {
                *slot = std::ptr::read_unaligned(pv.add(4 + i * 4) as *const u32);
            }
            spi_state().lock().unwrap().filter_keys = fk;
            TRUE
        }
        SPI_GETMOUSEKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let mk = spi_state().lock().unwrap().mouse_keys;
            std::ptr::write_unaligned(pv as *mut u32, 28);
            for (i, &w) in mk.iter().enumerate() {
                std::ptr::write_unaligned(pv.add(4 + i * 4) as *mut u32, w);
            }
            TRUE
        }
        SPI_SETMOUSEKEYS => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let mut mk = [0u32; 6];
            for (i, slot) in mk.iter_mut().enumerate() {
                *slot = std::ptr::read_unaligned(pv.add(4 + i * 4) as *const u32);
            }
            spi_state().lock().unwrap().mouse_keys = mk;
            TRUE
        }
        SPI_GETBORDER => put_u32(spi_state().lock().unwrap().border),
        SPI_SETBORDER => {
            spi_state().lock().unwrap().border = uparam.max(1);
            TRUE
        }
        SPI_GETSCREENSAVETIMEOUT => put_u32(spi_state().lock().unwrap().screensave_timeout),
        SPI_SETSCREENSAVETIMEOUT => {
            spi_state().lock().unwrap().screensave_timeout = uparam;
            TRUE
        }
        SPI_GETSCREENSAVEACTIVE => put_u32(spi_state().lock().unwrap().screensave_active),
        SPI_SETSCREENSAVEACTIVE => {
            spi_state().lock().unwrap().screensave_active = (uparam != 0) as u32;
            TRUE
        }
        SPI_GETSCREENSAVERRUNNING => put_u32(0),
        SPI_GETKEYBOARDDELAY => put_u32(spi_state().lock().unwrap().kbd_delay),
        SPI_SETKEYBOARDDELAY => {
            spi_state().lock().unwrap().kbd_delay = uparam.min(3);
            TRUE
        }
        SPI_GETKEYBOARDSPEED => put_u32(spi_state().lock().unwrap().kbd_speed),
        SPI_SETKEYBOARDSPEED => {
            spi_state().lock().unwrap().kbd_speed = uparam.min(31);
            TRUE
        }
        SPI_GETMOUSE => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let m = spi_state().lock().unwrap().mouse;
            for (i, &val) in m.iter().enumerate() {
                std::ptr::write_unaligned((pv as *mut i32).add(i), val);
            }
            TRUE
        }
        SPI_SETMOUSE => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let mut m = [0i32; 3];
            for (i, slot) in m.iter_mut().enumerate() {
                *slot = std::ptr::read_unaligned((pv as *const i32).add(i));
            }
            spi_state().lock().unwrap().mouse = m;
            TRUE
        }
        SPI_GETMOUSESPEED => put_u32(spi_state().lock().unwrap().mouse_speed),
        SPI_SETMOUSESPEED => {
            let v = pv as usize as u32;
            if !(1..=20).contains(&v) {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            spi_state().lock().unwrap().mouse_speed = v;
            TRUE
        }
        SPI_GETWHEELSCROLLLINES => put_u32(spi_state().lock().unwrap().wheel_lines),
        SPI_SETWHEELSCROLLLINES => {
            spi_state().lock().unwrap().wheel_lines = uparam;
            TRUE
        }
        SPI_GETWHEELSCROLLCHARS => put_u32(spi_state().lock().unwrap().wheel_chars),
        SPI_SETWHEELSCROLLCHARS => {
            spi_state().lock().unwrap().wheel_chars = uparam;
            TRUE
        }
        SPI_GETICONTITLEWRAP => put_u32(1),
        SPI_GETMENUDROPALIGNMENT => put_u32(0),
        SPI_GETFASTTASKSWITCH => put_u32(1),
        SPI_GETDRAGFULLWINDOWS => put_u32(spi_state().lock().unwrap().drag_full_windows),
        SPI_SETDRAGFULLWINDOWS => {
            spi_state().lock().unwrap().drag_full_windows = (uparam != 0) as u32;
            TRUE
        }
        SPI_GETFONTSMOOTHING => put_u32(1),
        SPI_GETFONTSMOOTHINGTYPE => put_u32(2),        // FE_FONTSMOOTHINGCLEARTYPE
        SPI_GETFONTSMOOTHINGCONTRAST => put_u32(1200), // standard gamma 1.2
        SPI_GETFONTSMOOTHINGORIENTATION => put_u32(1), // FE_FONTSMOOTHINGORIENTATIONRGB
        SPI_GETSCREENREADER => put_u32(0),
        SPI_GETSHOWSOUNDS => put_u32(0),
        SPI_GETSNAPTODEFBUTTON => put_u32(0),
        SPI_GETMOUSEHOVERWIDTH | SPI_GETMOUSEHOVERHEIGHT => put_u32(4),
        SPI_GETMOUSEHOVERTIME => put_u32(400),
        SPI_GETWINARRANGING | SPI_GETDOCKMOVING => put_u32(1),
        SPI_GETKEYBOARDCUES => put_u32(0),
        SPI_GETMOUSEVANISH => put_u32(0),
        SPI_GETFLATMENU | SPI_GETDROPSHADOW | SPI_GETUIEFFECTS | SPI_GETCLIENTAREAANIMATION => put_u32(1),
        SPI_GETFOREGROUNDLOCKTIMEOUT => put_u32(spi_state().lock().unwrap().fg_lock_timeout),
        SPI_SETFOREGROUNDLOCKTIMEOUT => {
            spi_state().lock().unwrap().fg_lock_timeout = pv as usize as u32;
            TRUE
        }
        SPI_GETACTIVEWNDTRKTIMEOUT => put_u32(0),
        SPI_GETFOREGROUNDFLASHCOUNT => put_u32(spi_state().lock().unwrap().fg_flash_count),
        SPI_SETFOREGROUNDFLASHCOUNT => {
            spi_state().lock().unwrap().fg_flash_count = pv as usize as u32;
            TRUE
        }
        SPI_GETCARETWIDTH => put_u32(spi_state().lock().unwrap().caret_width),
        SPI_SETCARETWIDTH => {
            let v = pv as usize as u32;
            if v == 0 {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            spi_state().lock().unwrap().caret_width = v;
            TRUE
        }
        SPI_GETMESSAGEDURATION => put_u32(5),
        SPI_GETDESKWALLPAPER => {
            if pv.is_null() || uparam == 0 {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            if wide {
                std::ptr::write_unaligned(pv as *mut u16, 0);
            } else {
                std::ptr::write_unaligned(pv, 0);
            }
            TRUE
        }
        SPI_GETHIGHCONTRAST => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            // HIGHCONTRAST { UINT cbSize; DWORD dwFlags; LPTSTR lpszDefaultScheme; } (16 bytes on x64)
            std::ptr::write_unaligned(pv.add(4) as *mut u32, 0);
            std::ptr::write_unaligned(pv.add(8) as *mut usize, 0);
            TRUE
        }
        SPI_GETSOUNDSENTRY => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            let cb = std::ptr::read_unaligned(pv as *const u32) as usize;
            if cb < 48 {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            std::ptr::write_bytes(pv.add(4), 0, cb - 4);
            TRUE
        }
        SPI_GETANIMATION => {
            if pv.is_null() {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv.add(4) as *mut i32, 1);
            TRUE
        }
        SPI_GETMINIMIZEDMETRICS => {
            // MINIMIZEDMETRICS { UINT cbSize; int iWidth, iHorzGap, iVertGap, iArrange; } (20 bytes)
            if pv.is_null() || std::ptr::read_unaligned(pv as *const u32) < 20 {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv.add(4) as *mut i32, scale_dpi(154, dpi));
            std::ptr::write_unaligned(pv.add(8) as *mut i32, 0);
            std::ptr::write_unaligned(pv.add(12) as *mut i32, 0);
            std::ptr::write_unaligned(pv.add(16) as *mut i32, 8); // ARW_HIDE
            TRUE
        }
        SPI_GETICONTITLELOGFONT => {
            let need = if wide { std::mem::size_of::<LogFontW>() } else { std::mem::size_of::<LogFontA>() };
            if pv.is_null() || (uparam != 0 && (uparam as usize) < need) {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            if wide {
                let mut lf = LogFontW::segoe_ui_9pt();
                lf.lf_height = scale_dpi(lf.lf_height, dpi);
                std::ptr::write_unaligned(pv as *mut LogFontW, lf);
            } else {
                let mut lf = LogFontA::segoe_ui_9pt();
                lf.lf_height = scale_dpi(lf.lf_height, dpi);
                std::ptr::write_unaligned(pv as *mut LogFontA, lf);
            }
            TRUE
        }
        SPI_GETICONMETRICS => {
            // ICONMETRICSA = 16 + 60 = 76 bytes; ICONMETRICSW = 16 + 92 = 108 bytes.
            let need = if wide { 108usize } else { 76usize };
            if pv.is_null() || (std::ptr::read_unaligned(pv as *const u32) as usize) < need {
                set_error(e::INVALID_PARAMETER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv.add(4) as *mut i32, scale_dpi(75, dpi)); // iHorzSpacing
            std::ptr::write_unaligned(pv.add(8) as *mut i32, scale_dpi(75, dpi)); // iVertSpacing
            std::ptr::write_unaligned(pv.add(12) as *mut i32, 1); // iTitleWrap
            if wide {
                let mut lf = LogFontW::segoe_ui_9pt();
                lf.lf_height = scale_dpi(lf.lf_height, dpi);
                std::ptr::write_unaligned(pv.add(16) as *mut LogFontW, lf);
            } else {
                let mut lf = LogFontA::segoe_ui_9pt();
                lf.lf_height = scale_dpi(lf.lf_height, dpi);
                std::ptr::write_unaligned(pv.add(16) as *mut LogFontA, lf);
            }
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
                let mut ncm = NonClientMetricsW::windows_default(cb as u32, cap);
                if dpi >= 48 && dpi != 96 {
                    ncm.i_border_width = scale_dpi(ncm.i_border_width, dpi);
                    ncm.i_scroll_width = scale_dpi(ncm.i_scroll_width, dpi);
                    ncm.i_scroll_height = scale_dpi(ncm.i_scroll_height, dpi);
                    ncm.i_caption_width = scale_dpi(ncm.i_caption_width, dpi);
                    ncm.i_caption_height = scale_dpi(ncm.i_caption_height, dpi);
                    ncm.i_sm_caption_width = scale_dpi(ncm.i_sm_caption_width, dpi);
                    ncm.i_sm_caption_height = scale_dpi(ncm.i_sm_caption_height, dpi);
                    ncm.i_menu_width = scale_dpi(ncm.i_menu_width, dpi);
                    ncm.i_menu_height = scale_dpi(ncm.i_menu_height, dpi);
                    ncm.i_padded_border_width = scale_dpi(ncm.i_padded_border_width, dpi);
                    let fh = scale_dpi(ncm.lf_caption_font.lf_height, dpi);
                    ncm.lf_caption_font.lf_height = fh;
                    ncm.lf_sm_caption_font.lf_height = fh;
                    ncm.lf_menu_font.lf_height = fh;
                    ncm.lf_status_font.lf_height = fh;
                    ncm.lf_message_font.lf_height = fh;
                }
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
            // Per Microsoft Learn SystemParametersInfo: invalid/unsupported uiAction
            // returns FALSE with ERROR_INVALID_PARAMETER.
            set_error(e::INVALID_PARAMETER);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SystemParametersInfoA(action: u32, uparam: u32, pv: *mut u8, _winini: u32) -> BOOL {
    spi(action, uparam, pv, false, 96)
}

pub unsafe extern "win64" fn SystemParametersInfoW(action: u32, uparam: u32, pv: *mut u8, _winini: u32) -> BOOL {
    spi(action, uparam, pv, true, 96)
}

pub unsafe extern "win64" fn SystemParametersInfoForDpi(action: u32, uparam: u32, pv: *mut u8, _winini: u32, dpi: u32) -> BOOL {
    spi(action, uparam, pv, true, dpi)
}

// ---------------------------------------------------------------------------
// Window station & Desktop
// ---------------------------------------------------------------------------

const WINSTA_HANDLE: usize = 0x0000_00A0;
const DESKTOP_HANDLE: usize = 0x0000_00A4;
const UOI_FLAGS: i32 = 1;
const UOI_NAME: i32 = 2;
const UOI_TYPE: i32 = 3;
const UOI_USER_SID: i32 = 4;
const UOI_HEAPSIZE: i32 = 5;
const UOI_IO: i32 = 6;
const WSF_VISIBLE: u32 = 1;

pub unsafe extern "win64" fn GetProcessWindowStation() -> usize {
    WINSTA_HANDLE
}

pub unsafe extern "win64" fn CloseWindowStation(h: usize) -> BOOL {
    if h == WINSTA_HANDLE {
        TRUE
    } else {
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

pub unsafe extern "win64" fn GetThreadDesktop(_tid: u32) -> usize {
    DESKTOP_HANDLE
}

pub unsafe extern "win64" fn OpenInputDesktop(_flags: u32, _inherit: BOOL, _access: u32) -> usize {
    DESKTOP_HANDLE
}

pub unsafe extern "win64" fn OpenDesktopW(name: LPCWSTR, _flags: u32, _inherit: BOOL, _access: u32) -> usize {
    if name.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let s = crate::strings::from_wide(name);
    if s.eq_ignore_ascii_case("Default") {
        DESKTOP_HANDLE
    } else {
        set_error(crate::types::err::ERROR_FILE_NOT_FOUND);
        0
    }
}

pub unsafe extern "win64" fn OpenDesktopA(name: LPCSTR, flags: u32, inherit: BOOL, access: u32) -> usize {
    if name.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let w = crate::strings::to_wide_z(&crate::strings::from_ansi(name));
    OpenDesktopW(w.as_ptr(), flags, inherit, access)
}

pub unsafe extern "win64" fn CloseDesktop(h: usize) -> BOOL {
    if h == DESKTOP_HANDLE {
        TRUE
    } else {
        set_error(crate::types::err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

unsafe fn user_object_info(h: usize, index: i32, pv: *mut u8, len: u32, needed: *mut u32, wide: bool) -> BOOL {
    let is_desktop = match h {
        WINSTA_HANDLE => false,
        DESKTOP_HANDLE => true,
        _ => {
            set_error(crate::types::err::ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    let s = match index {
        UOI_FLAGS => {
            // USEROBJECTFLAGS { BOOL fInherit; BOOL fReserved; DWORD dwFlags; }
            if !needed.is_null() {
                std::ptr::write_unaligned(needed, 12);
            }
            if len < 12 || pv.is_null() {
                set_error(e::INSUFFICIENT_BUFFER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv as *mut u32, 0);
            std::ptr::write_unaligned(pv.add(4) as *mut u32, 0);
            std::ptr::write_unaligned(pv.add(8) as *mut u32, if is_desktop { 0 } else { WSF_VISIBLE });
            return TRUE;
        }
        UOI_NAME => if is_desktop { "Default" } else { "WinSta0" },
        UOI_TYPE => if is_desktop { "Desktop" } else { "WindowStation" },
        UOI_USER_SID => {
            // Per Microsoft Learn: if no SID is associated with the window station or desktop,
            // GetUserObjectInformation sets *lpnLengthNeeded = 0 and returns TRUE.
            if !needed.is_null() {
                std::ptr::write_unaligned(needed, 0);
            }
            return TRUE;
        }
        UOI_HEAPSIZE => {
            if !needed.is_null() {
                std::ptr::write_unaligned(needed, 4);
            }
            if len < 4 || pv.is_null() {
                set_error(e::INSUFFICIENT_BUFFER);
                return FALSE;
            }
            // Standard interactive desktop heap size (20 MB in KB).
            std::ptr::write_unaligned(pv as *mut u32, if is_desktop { 20_480 } else { 0 });
            return TRUE;
        }
        UOI_IO => {
            if !needed.is_null() {
                std::ptr::write_unaligned(needed, 4);
            }
            if len < 4 || pv.is_null() {
                set_error(e::INSUFFICIENT_BUFFER);
                return FALSE;
            }
            std::ptr::write_unaligned(pv as *mut BOOL, TRUE);
            return TRUE;
        }
        _ => {
            set_error(e::INVALID_PARAMETER);
            return FALSE;
        }
    };
    let bytes = if wide { (s.len() + 1) * 2 } else { s.len() + 1 } as u32;
    if !needed.is_null() {
        std::ptr::write_unaligned(needed, bytes);
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

const MOD_MASK: u32 = 0x000F; // MOD_ALT | MOD_CONTROL | MOD_SHIFT | MOD_WIN
const MOD_NOREPEAT: u32 = 0x4000;

#[derive(Clone, Copy, Debug)]
struct HotKeyEntry {
    hwnd: usize,
    tid: u32,
    id: i32,
    mods: u32,
    norepeat: bool,
    vk: u32,
}

static HOTKEYS: std::sync::Mutex<Vec<HotKeyEntry>> = std::sync::Mutex::new(Vec::new());

struct HotKeyThreadGuard(u32);
impl Drop for HotKeyThreadGuard {
    fn drop(&mut self) {
        forget_thread_hotkeys(self.0);
    }
}

thread_local! {
    static HOTKEY_THREAD_GUARD: std::cell::RefCell<Option<HotKeyThreadGuard>> =
        const { std::cell::RefCell::new(None) };
}

fn arm_hotkey_thread_cleanup(tid: u32) {
    HOTKEY_THREAD_GUARD.with(|cell| {
        let mut guard = cell.borrow_mut();
        if guard.as_ref().map_or(true, |g| g.0 != tid) {
            *guard = Some(HotKeyThreadGuard(tid));
        }
    });
}

fn prune_dead_hotkeys(list: &mut Vec<HotKeyEntry>) {
    list.retain(|e| {
        if e.hwnd != 0 && !super::window::exists(e.hwnd) {
            return false;
        }
        if let Some(t) = crate::thread::find_by_tid(e.tid) {
            if t.exit_code().is_some() {
                return false;
            }
        }
        true
    });
}

/// Remove any registered hotkeys belonging to a destroyed window.
pub(crate) fn forget_hotkeys(hwnd: usize) {
    if hwnd == 0 {
        return;
    }
    HOTKEYS.lock().unwrap().retain(|h| h.hwnd != hwnd);
}

/// Remove any registered hotkeys belonging to an exiting thread (including
/// `hwnd == NULL` thread-targeted registrations).
pub(crate) fn forget_thread_hotkeys(tid: u32) {
    if tid == 0 {
        return;
    }
    HOTKEYS.lock().unwrap().retain(|h| h.tid != tid);
}

/// Check if an incoming keydown matches a registered hotkey; if so, post
/// `WM_HOTKEY` (0x0312) to the target window (or thread queue if `hwnd == 0`)
/// and return `true` to consume the keystroke.
pub(crate) fn try_dispatch_hotkey(mods: u32, vk: u32, is_repeat: bool) -> bool {
    let norm_mods = mods & MOD_MASK;
    let matched = {
        let mut list = HOTKEYS.lock().unwrap();
        prune_dead_hotkeys(&mut list);
        list.iter().find(|h| h.mods == norm_mods && h.vk == vk).copied()
    };
    let Some(entry) = matched else {
        return false;
    };
    if is_repeat && entry.norepeat {
        return true;
    }
    let lparam = ((entry.mods & 0xFFFF) | ((entry.vk & 0xFFFF) << 16)) as u32 as isize;
    if entry.hwnd != 0 {
        super::message::post_message(entry.hwnd, super::wm::HOTKEY, entry.id as usize, lparam);
    } else {
        let q = super::message::queue_for(entry.tid);
        q.post(MSG {
            hwnd: 0,
            message: super::wm::HOTKEY,
            wparam: entry.id as usize,
            lparam,
            time: tick_ms(),
            pt: super::cursor::cursor_pos(),
            l_private: 0,
        });
    }
    true
}

/// Register a hotkey (`WM_HOTKEY`) for `hwnd` (or the calling thread when `hwnd == NULL`).
/// Fires whenever the registered `(mods, vk)` combination is pressed while the app receives
/// keyboard events (note: macOS does not permit background global key capture without
/// user-granted Accessibility permissions).
pub unsafe extern "win64" fn RegisterHotKey(hwnd: HWND, id: i32, mods: u32, vk: u32) -> BOOL {
    if vk == 0 || (mods & !(MOD_MASK | MOD_NOREPEAT)) != 0 {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let h = hwnd as usize;
    let tid = crate::teb::thread_id();
    if h != 0 {
        match super::window::get(h) {
            Some(w) if w.tid == tid => {}
            Some(_) => {
                set_error(e::WINDOW_OF_OTHER_THREAD);
                return FALSE;
            }
            None => {
                set_error(e::INVALID_WINDOW_HANDLE);
                return FALSE;
            }
        }
    } else {
        // Ensure a thread message queue exists immediately so a hotkey arriving
        // before the thread's first PeekMessage/GetMessage is queued rather than dropped.
        let _ = super::message::queue_for(tid);
    }
    let norm_mods = mods & MOD_MASK;
    let norepeat = (mods & MOD_NOREPEAT) != 0;
    let mut list = HOTKEYS.lock().unwrap();
    prune_dead_hotkeys(&mut list);
    // Reject if another registration already owns this (mods, vk) pair.
    if list
        .iter()
        .any(|e| e.mods == norm_mods && e.vk == vk && !(e.hwnd == h && e.tid == tid && e.id == id))
    {
        set_error(e::HOTKEY_ALREADY_REGISTERED);
        return FALSE;
    }
    // Updating an existing (hwnd, tid, id) replaces its key binding in place.
    if let Some(existing) = list.iter_mut().find(|e| e.hwnd == h && e.tid == tid && e.id == id) {
        existing.mods = norm_mods;
        existing.norepeat = norepeat;
        existing.vk = vk;
    } else {
        list.push(HotKeyEntry { hwnd: h, tid, id, mods: norm_mods, norepeat, vk });
    }
    drop(list);
    arm_hotkey_thread_cleanup(tid);
    log::debug!("user32: RegisterHotKey(hwnd={h:#x}, id={id}, mods={mods:#x}, vk={vk:#x})");
    TRUE
}

pub unsafe extern "win64" fn UnregisterHotKey(hwnd: HWND, id: i32) -> BOOL {
    let h = hwnd as usize;
    let tid = crate::teb::thread_id();
    let mut list = HOTKEYS.lock().unwrap();
    let before = list.len();
    list.retain(|e| !(e.hwnd == h && e.tid == tid && e.id == id));
    if list.len() == before {
        set_error(e::HOTKEY_NOT_REGISTERED);
        return FALSE;
    }
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

/// Win32 `wsprintfA` / `wsprintfW` output buffer limit (1024 characters including NUL).
const WSPRINTF_MAX_CHARS: usize = 1023;

unsafe extern "win64" fn wsprintf_a_impl(args: *const u64) -> i32 {
    let mut va = crate::va::VaList::new(args as *const u8);
    let out = va.next_ptr::<u8>();
    let fmt = va.next_ptr::<u8>() as *const u8;
    if out.is_null() || fmt.is_null() {
        return 0;
    }
    let Some(bytes) = crate::ucrt::printf::format_narrow_va(fmt, va.as_ptr()) else {
        std::ptr::write_unaligned(out, 0);
        return 0;
    };
    let n = bytes.len().min(WSPRINTF_MAX_CHARS);
    if n > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, n);
    }
    std::ptr::write_unaligned(out.add(n), 0);
    n as i32
}

unsafe extern "win64" fn wsprintf_w_impl(args: *const u64) -> i32 {
    let mut va = crate::va::VaList::new(args as *const u8);
    let out = va.next_ptr::<u16>();
    let fmt = va.next_ptr::<u16>() as *const u16;
    if out.is_null() || fmt.is_null() {
        return 0;
    }
    let Some(units) = crate::ucrt::printf::format_wide_va(fmt, va.as_ptr()) else {
        std::ptr::write_unaligned(out, 0);
        return 0;
    };
    let n = units.len().min(WSPRINTF_MAX_CHARS);
    if n > 0 {
        std::ptr::copy_nonoverlapping(units.as_ptr() as *const u8, out as *mut u8, n * 2);
    }
    std::ptr::write_unaligned(out.add(n), 0);
    n as i32
}

win64_varargs_thunk!(wsprintfA => wsprintf_a_impl);
win64_varargs_thunk!(wsprintfW => wsprintf_w_impl);

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
            // Native-unsupported in headless/non-interactive mode (`PORT_HEADLESS=1` / no GUI
            // session): interactive macOS displays a real `NSAlert` above. When there is no
            // interactive desktop, Windows itself fails multi-choice `MessageBox` with 0 and
            // `ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION` (1459; Microsoft Learn `MessageBox`).
            log::warn!(target: "messagebox", "MessageBox type {ty:#x}: non-interactive headless session -> 0 (ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION)");
            set_error(e::REQUIRES_INTERACTIVE_WINDOWSTATION);
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

    #[test]
    fn wsprintf_a_and_w_format_varargs_and_cap_at_1023() {
        unsafe {
            let mut abuf = [0u8; 1040];
            let fmt_a = c"val=%d hex=%x str=%s".as_ptr() as u64;
            let s_arg = c"ok".as_ptr() as u64;
            let n = wsprintfA(abuf.as_mut_ptr() as u64, fmt_a, 42, 0xAB, s_arg, 0, 0, 0, 0, 0, 0, 0);
            assert_eq!(n, 20);
            assert_eq!(crate::strings::from_ansi(abuf.as_ptr()), "val=42 hex=ab str=ok");

            // 1024-char cap (max 1023 formatted chars + NUL).
            let long_str = format!("{}\0", "X".repeat(1200));
            let n_cap = wsprintfA(
                abuf.as_mut_ptr() as u64,
                c"%s".as_ptr() as u64,
                long_str.as_ptr() as u64,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            );
            assert_eq!(n_cap, 1023);
            assert_eq!(abuf[1023], 0);

            let mut wbuf = [0u16; 1040];
            let fmt_w = crate::strings::to_wide_z("val=%d hex=%X str=%s");
            let ws_arg = crate::strings::to_wide_z("wide");
            let wn = wsprintfW(
                wbuf.as_mut_ptr() as u64,
                fmt_w.as_ptr() as u64,
                42,
                0xAB,
                ws_arg.as_ptr() as u64,
                0, 0, 0, 0, 0, 0, 0,
            );
            assert_eq!(wn, 22);
            assert_eq!(read_wide_unaligned(wbuf.as_ptr()), "val=42 hex=AB str=wide");

            let long_w = crate::strings::to_wide_z(&"Y".repeat(1200));
            let fmt_ws = crate::strings::to_wide_z("%s");
            let wn_cap = wsprintfW(
                wbuf.as_mut_ptr() as u64,
                fmt_ws.as_ptr() as u64,
                long_w.as_ptr() as u64,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            );
            assert_eq!(wn_cap, 1023);
            assert_eq!(wbuf[1023], 0);

            // Odd-address (unaligned) UTF-16 output buffer must work without UB/alignment panic.
            let mut raw = vec![0xA5u8; 128];
            let odd_out = raw.as_mut_ptr().add(1) as *mut u16;
            assert!(!odd_out.is_aligned(), "expected odd-address unaligned u16 pointer");
            let wn_odd = wsprintfW(
                odd_out as u64,
                fmt_w.as_ptr() as u64,
                7,
                0x2A,
                ws_arg.as_ptr() as u64,
                0, 0, 0, 0, 0, 0, 0,
            );
            assert_eq!(wn_odd, 21);
            assert_eq!(read_wide_unaligned(odd_out), "val=7 hex=2A str=wide");
            assert_eq!(std::ptr::read_unaligned(odd_out.add(21)), 0);
            assert_eq!(raw[0], 0xA5);

            // %c / %C with 0 argument must emit an embedded NUL, count it in the return length,
            // and continue formatting subsequent characters rather than terminating early.
            let mut nul_a = [0xFFu8; 16];
            let n_c = wsprintfA(nul_a.as_mut_ptr() as u64, c"a%cb".as_ptr() as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            assert_eq!(n_c, 3);
            assert_eq!(&nul_a[..4], &[b'a', 0, b'b', 0]);

            let mut nul_a_upper = [0xFFu8; 16];
            let n_cu = wsprintfA(nul_a_upper.as_mut_ptr() as u64, c"a%Cb".as_ptr() as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            assert_eq!(n_cu, 3);
            assert_eq!(&nul_a_upper[..4], &[b'a', 0, b'b', 0]);

            let mut nul_w = [0xFFFFu16; 16];
            let fmt_wc0 = crate::strings::to_wide_z("a%cb");
            let nw_c = wsprintfW(nul_w.as_mut_ptr() as u64, fmt_wc0.as_ptr() as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            assert_eq!(nw_c, 3);
            assert_eq!(&nul_w[..4], &[b'a' as u16, 0, b'b' as u16, 0]);

            let mut nul_w_upper = [0xFFFFu16; 16];
            let fmt_wcu0 = crate::strings::to_wide_z("a%Cb");
            let nw_cu = wsprintfW(nul_w_upper.as_mut_ptr() as u64, fmt_wcu0.as_ptr() as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            assert_eq!(nw_cu, 3);
            assert_eq!(&nul_w_upper[..4], &[b'a' as u16, 0, b'b' as u16, 0]);

            // Precision must not limit %c / %C output.
            let mut prec_a = [0u8; 32];
            let n_prec_a = wsprintfA(
                prec_a.as_mut_ptr() as u64,
                c"%.0c|%.0C|%4.0c|%-4.0C".as_ptr() as u64,
                b'X' as u64,
                b'Y' as u64,
                b'Z' as u64,
                b'W' as u64,
                0, 0, 0, 0, 0, 0,
            );
            assert_eq!(n_prec_a, 13);
            assert_eq!(crate::strings::from_ansi(prec_a.as_ptr()), "X|Y|   Z|W   ");

            let mut prec_w = [0u16; 32];
            let fmt_wprec = crate::strings::to_wide_z("%.0c|%.0C|%4.0c|%-4.0C");
            let n_prec_w = wsprintfW(
                prec_w.as_mut_ptr() as u64,
                fmt_wprec.as_ptr() as u64,
                b'X' as u64,
                b'Y' as u64,
                b'Z' as u64,
                b'W' as u64,
                0, 0, 0, 0, 0, 0,
            );
            assert_eq!(n_prec_w, 13);
            assert_eq!(read_wide_unaligned(prec_w.as_ptr()), "X|Y|   Z|W   ");

            // On ARM64, guest calls with >12 total Win64 arguments must read all remaining
            // variadic slots from the caller's guest Win64 stack via `incoming_rsp()`.
            #[cfg(target_arch = "aarch64")]
            {
                use std::sync::atomic::{AtomicU64, Ordering};
                static TEST_WS_RSP_LO: AtomicU64 = AtomicU64::new(0);
                static TEST_WS_RSP_HI: AtomicU64 = AtomicU64::new(0);
                fn test_ws_rsp() -> (u64, u64) {
                    (TEST_WS_RSP_LO.load(Ordering::Acquire), TEST_WS_RSP_HI.load(Ordering::Acquire))
                }
                let s11 = c"eleven".as_ptr() as u64;
                let s13 = c"thirteen".as_ptr() as u64;
                let mut big_a = [0u8; 128];
                let fmt_many_a = c"%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%s,%d,%s".as_ptr() as u64;
                // Win64 stack frame: [0..4] shadow space, [4..15] args 4..14 (15 total args).
                let mut stack = [0u64; 32];
                stack[4] = 3;
                stack[5] = 4;
                stack[6] = 5;
                stack[7] = 6;
                stack[8] = 7;
                stack[9] = 8;
                stack[10] = 9;
                stack[11] = 10;
                stack[12] = s11;
                stack[13] = 12;
                stack[14] = s13;
                let lo = stack.as_ptr() as u64;
                TEST_WS_RSP_LO.store(lo, Ordering::Release);
                TEST_WS_RSP_HI.store(lo + 256, Ordering::Release);
                crate::set_incoming_rsp_reader(test_ws_rsp);
                let nm = wsprintfA(big_a.as_mut_ptr() as u64, fmt_many_a, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
                TEST_WS_RSP_LO.store(0, Ordering::Release);
                TEST_WS_RSP_HI.store(0, Ordering::Release);
                assert_eq!(nm, 39);
                assert_eq!(
                    crate::strings::from_ansi(big_a.as_ptr()),
                    "1,2,3,4,5,6,7,8,9,10,eleven,12,thirteen"
                );
            }
        }
    }

    #[test]
    fn system_metrics_spi_user_object_info_hotkeys_and_headless_messagebox() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            // 1. GetSystemMetrics covers newly added SM_* indices and returns 0 for unknown.
            assert_eq!(GetSystemMetrics(42), 1); // SM_DBCSENABLED
            assert_eq!(GetSystemMetrics(45), 2); // SM_CXEDGE
            assert_eq!(GetSystemMetrics(82), 1); // SM_IMMENABLED
            assert_eq!(GetSystemMetrics(9999), 0); // undefined -> 0
            assert_eq!(GetSystemMetricsForDpi(45, 192), 4); // 2 * 192/96

            // 2. SystemParametersInfo GET/SET persistence + DPI-scaled NONCLIENTMETRICS.
            let mut lines = 0u32;
            assert_eq!(SystemParametersInfoW(SPI_SETWHEELSCROLLLINES, 7, std::ptr::null_mut(), 0), TRUE);
            assert_eq!(SystemParametersInfoW(SPI_GETWHEELSCROLLLINES, 0, &mut lines as *mut u32 as *mut u8, 0), TRUE);
            assert_eq!(lines, 7);
            assert_eq!(SystemParametersInfoW(SPI_SETWHEELSCROLLLINES, 3, std::ptr::null_mut(), 0), TRUE);

            let mut ncm = std::mem::zeroed::<NonClientMetricsW>();
            ncm.cb_size = std::mem::size_of::<NonClientMetricsW>() as u32;
            assert_eq!(
                SystemParametersInfoForDpi(SPI_GETNONCLIENTMETRICS, ncm.cb_size, &mut ncm as *mut _ as *mut u8, 0, 192),
                TRUE
            );
            assert_eq!(ncm.i_scroll_width, 34); // 17 * 2
            assert_eq!(ncm.lf_caption_font.lf_height, -24); // -12 * 2

            // Unknown SPI action returns FALSE + ERROR_INVALID_PARAMETER.
            assert_eq!(SystemParametersInfoW(0xDEAD, 0, std::ptr::null_mut(), 0), FALSE);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);

            // 3. GetUserObjectInformation on WindowStation AND Desktop handles for all UOI_* indices.
            let winsta = GetProcessWindowStation();
            let desk = GetThreadDesktop(0);
            let mut needed = 0u32;
            let mut name_buf = [0u16; 32];
            assert_eq!(GetUserObjectInformationW(desk, UOI_NAME, name_buf.as_mut_ptr() as *mut u8, 64, &mut needed), TRUE);
            assert_eq!(read_wide_unaligned(name_buf.as_ptr()), "Default");
            assert_eq!(GetUserObjectInformationW(desk, UOI_TYPE, name_buf.as_mut_ptr() as *mut u8, 64, &mut needed), TRUE);
            assert_eq!(read_wide_unaligned(name_buf.as_ptr()), "Desktop");
            let mut heap_kb = 0u32;
            assert_eq!(GetUserObjectInformationW(desk, UOI_HEAPSIZE, &mut heap_kb as *mut u32 as *mut u8, 4, &mut needed), TRUE);
            assert!(heap_kb > 0);
            let mut io_flag: BOOL = 0;
            assert_eq!(GetUserObjectInformationW(winsta, UOI_IO, &mut io_flag as *mut BOOL as *mut u8, 4, &mut needed), TRUE);
            assert_eq!(io_flag, TRUE);
            assert_eq!(GetUserObjectInformationW(winsta, UOI_USER_SID, std::ptr::null_mut(), 0, &mut needed), TRUE);
            assert_eq!(needed, 0);
            assert_eq!(GetUserObjectInformationW(0x1234, UOI_NAME, std::ptr::null_mut(), 0, &mut needed), FALSE);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_HANDLE);
            assert_eq!(GetUserObjectInformationW(winsta, 99, std::ptr::null_mut(), 0, &mut needed), FALSE);
            assert_eq!(crate::teb::last_error(), e::INVALID_PARAMETER);

            // 4. Headless multi-button MessageBox returns 0 + ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION.
            let title = crate::strings::to_wide_z("Confirm");
            let text = crate::strings::to_wide_z("Proceed?");
            assert_eq!(MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 4 /* MB_YESNO */), 0);
            assert_eq!(crate::teb::last_error(), e::REQUIRES_INTERACTIVE_WINDOWSTATION);
            assert_eq!(MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), 0 /* MB_OK */), IDOK);
        }

        // 5. RegisterHotKey -> native_key -> WM_HOTKEY -> UnregisterHotKey.
        std::thread::spawn(|| unsafe {
            let cls = crate::strings::to_wide_z("HotKeyTestWnd");
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
                0, cls.as_ptr(), std::ptr::null(), ws::POPUP,
                0, 0, 50, 50, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());
            assert_eq!(RegisterHotKey(hwnd, 7, 0x0002 /* MOD_CONTROL */ | MOD_NOREPEAT, b'A' as u32), TRUE);
            // Duplicate registration with different ID fails with HOTKEY_ALREADY_REGISTERED.
            assert_eq!(RegisterHotKey(hwnd, 8, 0x0002, b'A' as u32), FALSE);
            assert_eq!(crate::teb::last_error(), e::HOTKEY_ALREADY_REGISTERED);

            // Press Ctrl + 'A' (mac 0x3B = LCtrl, 0x00 = 'A').
            super::super::input::native_key(super::super::input::NativeKey { mac: 0x3B, down: true, repeat: false, chars: Vec::new() });
            super::super::input::native_key(super::super::input::NativeKey { mac: 0x00, down: true, repeat: false, chars: vec![b'a' as u16] });
            // Repeat must be suppressed by MOD_NOREPEAT.
            super::super::input::native_key(super::super::input::NativeKey { mac: 0x00, down: true, repeat: true, chars: vec![b'a' as u16] });
            super::super::input::release_all_keys();

            let mut m = MSG::default();
            assert_eq!(super::super::message::PeekMessageW(&mut m, hwnd, wm::HOTKEY, wm::HOTKEY, super::super::message::PM_REMOVE), TRUE);
            assert_eq!(m.wparam, 7);
            assert_eq!(m.lparam, ((0x0002u32) | ((b'A' as u32) << 16)) as isize);
            assert_eq!(super::super::message::PeekMessageW(&mut m, hwnd, wm::HOTKEY, wm::HOTKEY, super::super::message::PM_REMOVE), FALSE);

            assert_eq!(UnregisterHotKey(hwnd, 7), TRUE);
            assert_eq!(UnregisterHotKey(hwnd, 7), FALSE);
            assert_eq!(crate::teb::last_error(), e::HOTKEY_NOT_REGISTERED);
            let _ = super::super::window::DestroyWindow(hwnd);
        })
        .join()
        .unwrap();

        // 6. Thread-targeted RegisterHotKey(NULL, ...) creates its queue before first pump
        //    AND automatically releases its hotkey binding when the owning thread exits.
        std::thread::spawn(|| unsafe {
            // Register Ctrl+B on a brand-new thread with no window and no prior message pump.
            assert_eq!(RegisterHotKey(std::ptr::null_mut(), 42, 0x0002, b'B' as u32), TRUE);
            // Fire Ctrl+B before this thread ever calls PeekMessageW.
            super::super::input::native_key(super::super::input::NativeKey {
                mac: 0x3B, down: true, repeat: false, chars: Vec::new(),
            });
            super::super::input::native_key(super::super::input::NativeKey {
                mac: 0x0B, down: true, repeat: false, chars: vec![b'b' as u16],
            });
            super::super::input::release_all_keys();

            let mut m = MSG::default();
            assert_eq!(
                super::super::message::PeekMessageW(
                    &mut m,
                    std::ptr::null_mut(),
                    wm::HOTKEY,
                    wm::HOTKEY,
                    super::super::message::PM_REMOVE,
                ),
                TRUE,
                "hotkey arriving before first pump must be delivered to thread queue"
            );
            assert_eq!(m.hwnd, 0);
            assert_eq!(m.wparam, 42);
            assert_eq!(m.lparam, ((0x0002u32) | ((b'B' as u32) << 16)) as isize);
            // Intentionally exit WITHOUT calling UnregisterHotKey to verify teardown cleanup.
        })
        .join()
        .unwrap();

        // A replacement thread must now be able to register Ctrl+B cleanly (no stale conflict).
        std::thread::spawn(|| unsafe {
            assert_eq!(RegisterHotKey(std::ptr::null_mut(), 43, 0x0002, b'B' as u32), TRUE);
            assert_eq!(UnregisterHotKey(std::ptr::null_mut(), 43), TRUE);
        })
        .join()
        .unwrap();
    }
}
