//! Windows: the HWND table, CreateWindowEx/DestroyWindow, geometry,
//! show/position, window longs, text, painting.
//!
//! Geometry is stored in Win32 pixels (see the Retina policy in
//! [`super`]): `rect` is the window rectangle and `client` the client
//! rectangle, both in screen coordinates for top-level windows and in
//! parent-client coordinates for child windows. Non-client insets mirror
//! the macOS frame actually used: a captioned window has a title bar of
//! the real AppKit height and no side borders; everything else is
//! borderless (see [`nc_insets`]).

use super::class::{self, Class, ClassName};
use super::message::{self, send_message};
use super::*;
use crate::types::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock};

pub const GWLP_WNDPROC: i32 = -4;
pub const GWLP_HINSTANCE: i32 = -6;
pub const GWLP_HWNDPARENT: i32 = -8;
pub const GWLP_ID: i32 = -12;
pub const GWL_STYLE: i32 = -16;
pub const GWL_EXSTYLE: i32 = -20;
pub const GWLP_USERDATA: i32 = -21;

pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOMOVE: u32 = 0x0002;
pub const SWP_NOZORDER: u32 = 0x0004;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_FRAMECHANGED: u32 = 0x0020;
pub const SWP_SHOWWINDOW: u32 = 0x0040;
pub const SWP_HIDEWINDOW: u32 = 0x0080;
pub const SWP_NOOWNERZORDER: u32 = 0x0200;
pub const SWP_NOSENDCHANGING: u32 = 0x0400;
pub const SWP_STATECHANGED: u32 = 0x8000;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOWNORMAL: i32 = 1;
pub const SW_SHOWMINIMIZED: i32 = 2;
pub const SW_MAXIMIZE: i32 = 3;
pub const SW_SHOWNOACTIVATE: i32 = 4;
pub const SW_SHOW: i32 = 5;
pub const SW_MINIMIZE: i32 = 6;
pub const SW_SHOWMINNOACTIVE: i32 = 7;
pub const SW_SHOWNA: i32 = 8;
pub const SW_RESTORE: i32 = 9;
pub const SW_SHOWDEFAULT: i32 = 10;
pub const SW_FORCEMINIMIZE: i32 = 11;

pub const SIZE_RESTORED: usize = 0;
pub const SIZE_MINIMIZED: usize = 1;
pub const SIZE_MAXIMIZED: usize = 2;

pub const HWND_TOP: usize = 0;
pub const HWND_BOTTOM: usize = 1;
pub const HWND_TOPMOST: usize = usize::MAX; // -1
pub const HWND_NOTOPMOST: usize = usize::MAX - 1; // -2
pub const HWND_MESSAGE: usize = usize::MAX - 2; // -3

const CW_USEDEFAULT: i32 = 0x8000_0000u32 as i32;
const MINIMIZED_POS: i32 = -32000;

/// The desktop window (GetDesktopWindow).
pub const DESKTOP_HWND: usize = 0x0001_0010;

pub struct WinData {
    pub wndproc: usize,
    pub style: u32,
    pub exstyle: u32,
    pub parent: usize,
    pub owner: usize,
    pub id: usize,
    pub instance: usize,
    pub userdata: usize,
    pub extra: Vec<u8>,
    pub text: Vec<u16>,
    pub rect: RECT,
    pub client: RECT,
    /// Window rect to return to from minimized/maximized.
    pub normal: RECT,
    pub visible: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub enabled: bool,
    pub topmost: bool,
    pub native: bool,
    pub destroying: bool,
    pub erase_pending: bool,
}

pub struct Window {
    pub hwnd: usize,
    pub tid: u32,
    pub class: Arc<Class>,
    d: Mutex<WinData>,
}

impl Window {
    pub fn data(&self) -> MutexGuard<'_, WinData> {
        self.d.lock().unwrap_or_else(|e| e.into_inner())
    }
}

struct Table {
    map: HashMap<usize, Arc<Window>>,
    /// Top-level windows, front to back.
    zorder: Vec<usize>,
}

fn table() -> &'static RwLock<Table> {
    static T: OnceLock<RwLock<Table>> = OnceLock::new();
    T.get_or_init(|| RwLock::new(Table { map: HashMap::new(), zorder: Vec::new() }))
}

static NEXT_HWND: AtomicUsize = AtomicUsize::new(0x0001_0040);

pub fn get(hwnd: usize) -> Option<Arc<Window>> {
    table().read().unwrap().map.get(&hwnd).cloned()
}

pub fn exists(hwnd: usize) -> bool {
    table().read().unwrap().map.contains_key(&hwnd)
}

pub fn any_window_of_class(atom: ATOM) -> bool {
    table().read().unwrap().map.values().any(|w| w.class.atom == atom)
}

pub fn top_level_windows() -> Vec<usize> {
    table().read().unwrap().zorder.clone()
}

pub fn windows_of_thread(tid: u32) -> Vec<usize> {
    table().read().unwrap().map.values().filter(|w| w.tid == tid).map(|w| w.hwnd).collect()
}

fn bring_to_front(hwnd: usize) {
    let mut t = table().write().unwrap();
    if let Some(i) = t.zorder.iter().position(|&h| h == hwnd) {
        t.zorder.remove(i);
        t.zorder.insert(0, hwnd);
    }
}

/// Is `h` a (direct or indirect) child of `parent`?
pub fn is_child_of(h: usize, parent: usize) -> bool {
    let mut cur = h;
    for _ in 0..64 {
        let Some(w) = get(cur) else { return false };
        let d = w.data();
        if d.style & ws::CHILD == 0 {
            return false;
        }
        if d.parent == parent {
            return true;
        }
        cur = d.parent;
    }
    false
}

fn is_top_level(d: &WinData) -> bool {
    d.style & ws::CHILD == 0
}

fn require(hwnd: usize) -> Option<Arc<Window>> {
    let w = get(hwnd);
    if w.is_none() {
        set_error(e::INVALID_WINDOW_HANDLE);
    }
    w
}

/// Call the window's current window procedure (on the calling thread).
pub fn call_window_proc(hwnd: usize, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let Some(w) = get(hwnd) else { return 0 };
    let p = w.data().wndproc;
    if p == 0 {
        return 0;
    }
    unsafe { super::dispatch_wndproc(p, hwnd as HWND, msg, wparam, lparam) }
}

// ---------------------------------------------------------------------------
// Non-client metrics
// ---------------------------------------------------------------------------

/// Win32 pixels per macOS point on the primary display.
pub fn scale() -> f64 {
    crate::macos::display::scale()
}

/// (left, top, right, bottom) non-client insets in pixels for a style at a
/// specific display scale factor.
pub fn nc_insets_for_scale(style: u32, _exstyle: u32, s: f64) -> (i32, i32, i32, i32) {
    if crate::macos::single_surface_mode() {
        return (0, 0, 0, 0);
    }
    if style & ws::CHILD == 0 && style & ws::CAPTION == ws::CAPTION {
        let h = (crate::macos::titlebar_height_pt() * s.max(0.5)).round() as i32;
        (0, h, 0, 0)
    } else {
        (0, 0, 0, 0)
    }
}

/// (left, top, right, bottom) non-client insets in pixels for a style.
pub fn nc_insets(style: u32, exstyle: u32) -> (i32, i32, i32, i32) {
    nc_insets_for_scale(style, exstyle, scale())
}

pub fn client_from_window_for_scale(r: &RECT, style: u32, exstyle: u32, s: f64) -> RECT {
    let (l, t, rr, b) = nc_insets_for_scale(style, exstyle, s);
    let mut c = RECT::new(r.left + l, r.top + t, r.right - rr, r.bottom - b);
    if c.right < c.left {
        c.right = c.left;
    }
    if c.bottom < c.top {
        c.bottom = c.top;
    }
    c
}

pub fn client_from_window(r: &RECT, style: u32, exstyle: u32) -> RECT {
    client_from_window_for_scale(r, style, exstyle, scale())
}

pub fn window_from_client_for_scale(c: &RECT, style: u32, exstyle: u32, s: f64) -> RECT {
    let (l, t, r, b) = nc_insets_for_scale(style, exstyle, s);
    RECT::new(c.left - l, c.top - t, c.right + r, c.bottom + b)
}

pub fn window_from_client(c: &RECT, style: u32, exstyle: u32) -> RECT {
    window_from_client_for_scale(c, style, exstyle, scale())
}

unsafe fn adjust(r: *mut RECT, style: u32, exstyle: u32, s: f64) -> BOOL {
    if r.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let cur = r.read_unaligned();
    r.write_unaligned(window_from_client_for_scale(&cur, style, exstyle, s));
    TRUE
}

pub unsafe extern "win64" fn AdjustWindowRectEx(r: *mut RECT, style: u32, _menu: BOOL, exstyle: u32) -> BOOL {
    adjust(r, style, exstyle, scale())
}

pub unsafe extern "win64" fn AdjustWindowRect(r: *mut RECT, style: u32, _menu: BOOL) -> BOOL {
    adjust(r, style, 0, scale())
}

pub unsafe extern "win64" fn AdjustWindowRectExForDpi(r: *mut RECT, style: u32, _menu: BOOL, exstyle: u32, dpi_arg: u32) -> BOOL {
    let s = if dpi_arg >= 48 { dpi_arg as f64 / 96.0 } else { scale() };
    adjust(r, style, exstyle, s)
}

// ---------------------------------------------------------------------------
// Native backend glue
// ---------------------------------------------------------------------------

fn native_desc(w: &Window, d: &WinData) -> crate::macos::NativeDesc {
    crate::macos::NativeDesc {
        client: d.client,
        style: d.style,
        exstyle: d.exstyle,
        title: String::from_utf16_lossy(&d.text),
        topmost: d.topmost,
        dblclks: w.class.style & class::CS_DBLCLKS != 0,
    }
}

/// Events from the AppKit backend (main thread). They are re-dispatched as
/// internal calls on the window's owner thread.
#[derive(Clone, Copy, Debug)]
pub enum NativeEvent {
    /// The native content rect changed (user resize/move, zoom, screen change).
    Frame { client: RECT },
    /// The window moved to a monitor with a different backing scale / DPI.
    DpiChanged { dpi: u32, client: RECT },
    CloseRequested,
    Activated(bool),
    Minimized(bool),
    EnterSizeMove,
    ExitSizeMove,
}

pub fn native_event(hwnd: usize, ev: NativeEvent) {
    let Some(w) = get(hwnd) else { return };
    let Some(q) = message::queue_of(w.tid) else { return };
    log::trace!("native event {hwnd:#x}: {ev:?}");
    q.call(move || unsafe { handle_native_event(hwnd, ev) });
}

unsafe fn handle_native_event(hwnd: usize, ev: NativeEvent) {
    let Some(w) = get(hwnd) else { return };
    match ev {
        NativeEvent::Frame { client } => {
            let (style, exstyle, minimized) = {
                let d = w.data();
                (d.style, d.exstyle, d.minimized)
            };
            if minimized {
                return;
            }
            let s = crate::macos::display::scale_for_rect(&client);
            let r = window_from_client_for_scale(&client, style, exstyle, s);
            let cur = w.data().rect;
            if cur == r {
                return;
            }
            set_window_pos_impl(&w, 0, r.left, r.top, r.width(), r.height(), SWP_NOZORDER | SWP_NOACTIVATE, false);
        }
        NativeEvent::DpiChanged { dpi: new_dpi, client } => {
            let (style, exstyle, minimized) = {
                let d = w.data();
                (d.style, d.exstyle, d.minimized)
            };
            if minimized {
                return;
            }
            let s = (new_dpi.max(48) as f64) / 96.0;
            let mut suggested = window_from_client_for_scale(&client, style, exstyle, s);
            let wp = ((new_dpi & 0xFFFF) | ((new_dpi & 0xFFFF) << 16)) as WPARAM;
            send_message(hwnd, wm::DPICHANGED, wp, &mut suggested as *mut RECT as LPARAM);
            let cur = w.data().rect;
            if cur != suggested {
                set_window_pos_impl(
                    &w,
                    0,
                    suggested.left,
                    suggested.top,
                    suggested.width(),
                    suggested.height(),
                    SWP_NOZORDER | SWP_NOACTIVATE,
                    false,
                );
            }
        }
        NativeEvent::CloseRequested => {
            send_message(hwnd, wm::SYSCOMMAND, super::defproc::SC_CLOSE, 0);
        }
        NativeEvent::Activated(true) => super::input::set_active_window(hwnd, true),
        NativeEvent::Activated(false) => super::input::native_deactivated(hwnd),
        NativeEvent::Minimized(m) => {
            let cur = w.data().minimized;
            if cur != m {
                show_window_impl(&w, if m { SW_SHOWMINNOACTIVE } else { SW_RESTORE }, false);
            }
        }
        NativeEvent::EnterSizeMove => {
            send_message(hwnd, wm::ENTERSIZEMOVE, 0, 0);
        }
        NativeEvent::ExitSizeMove => {
            send_message(hwnd, wm::EXITSIZEMOVE, 0, 0);
        }
    }
}

// ---------------------------------------------------------------------------
// CreateWindowEx / DestroyWindow
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct CREATESTRUCT {
    pub create_params: usize,
    pub instance: usize,
    pub menu: usize,
    pub parent: usize,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub name: usize,
    pub class: usize,
    pub exstyle: u32,
}
const _: () = assert!(std::mem::size_of::<CREATESTRUCT>() == 80);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WINDOWPOS {
    pub hwnd: usize,
    pub insert_after: usize,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MINMAXINFO {
    pub reserved: POINT,
    pub max_size: POINT,
    pub max_position: POINT,
    pub min_track: POINT,
    pub max_track: POINT,
}

pub struct CreateArgs {
    pub exstyle: u32,
    pub class: ClassName,
    pub name: Vec<u16>,
    pub style: u32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub parent: usize,
    pub menu: usize,
    pub instance: usize,
    pub param: usize,
    pub class_ptr: usize,
    pub name_ptr: usize,
}

fn default_rect(style: u32, a: &CreateArgs) -> (i32, i32, i32, i32) {
    let work = super::monitor::primary_work_area();
    let (mut x, mut y, mut w, mut h) = (a.x, a.y, a.w, a.h);
    if x == CW_USEDEFAULT {
        x = work.left + work.width() / 8;
        y = work.top + work.height() / 8;
    }
    if w == CW_USEDEFAULT {
        w = work.width() * 3 / 4;
        h = work.height() * 3 / 4;
    }
    if style & ws::CHILD != 0 && (a.x == CW_USEDEFAULT || a.w == CW_USEDEFAULT) {
        (0, 0, 0, 0)
    } else {
        (x, y, w.max(0), h.max(0))
    }
}

fn single_surface_top_level_rect(style: u32) -> Option<RECT> {
    if crate::macos::single_surface_mode() && style & ws::CHILD == 0 {
        Some(super::monitor::primary_rect())
    } else {
        None
    }
}

fn framed_top_level_style(style: u32, exstyle: u32) -> (u32, u32) {
    if crate::macos::single_surface_mode() && style & ws::CHILD == 0 {
        // Locked single-surface mode (iOS or PORT_LOCK_DISPLAY_MODE): preserve
        // game-visible style flags without forcing desktop window chrome, and
        // never let EX_TOPMOST/caption change presentation geometry.
        return (style, exstyle & !ws::EX_TOPMOST);
    }
    if crate::macos::framed_mode() && style & ws::CHILD == 0 {
        let s = (style & !ws::POPUP)
            | ws::CAPTION
            | ws::SYSMENU
            | ws::MINIMIZEBOX
            | ws::MAXIMIZEBOX
            | ws::THICKFRAME;
        let ex = exstyle & !ws::EX_TOPMOST;
        (s, ex)
    } else {
        (style, exstyle)
    }
}

fn fit_framed_window_rect(rect: RECT, style: u32, exstyle: u32) -> RECT {
    if !crate::macos::framed_mode() || style & ws::CHILD != 0 {
        return rect;
    }
    let work = super::monitor::primary_work_area();
    let mon = super::monitor::primary_rect();
    let (_, top_inset, _, _) = nc_insets(style, exstyle);
    let max_cw = work.width().max(320);
    let max_ch = (work.height() - top_inset).max(200);

    let full_takeover_req = rect.width() >= mon.width() * 9 / 10
        || rect.height() >= mon.height() * 9 / 10
        || rect.width() > work.width()
        || rect.height() > work.height();

    if let Some((cfg_w, cfg_h)) = crate::macos::configured_window_size() {
        let cw = cfg_w.min(max_cw);
        let ch = cfg_h.min(max_ch);
        let win_h = ch + top_inset;
        let wx = work.left + (work.width() - cw).max(0) / 2;
        let wy = work.top + (work.height() - win_h).max(0) / 2;
        return RECT::new(wx, wy, wx + cw, wy + win_h);
    }

    if full_takeover_req {
        let mut cw = (max_cw * 85 / 100) & !1;
        let mut ch = (cw * 9 / 16) & !1;
        if ch > max_ch * 88 / 100 {
            ch = (max_ch * 85 / 100) & !1;
            cw = ((ch * 16 / 9) & !1).min(max_cw);
        }
        let win_h = ch + top_inset;
        let wx = work.left + (work.width() - cw).max(0) / 2;
        let wy = work.top + (work.height() - win_h).max(0) / 2;
        return RECT::new(wx, wy, wx + cw, wy + win_h);
    }

    // Smaller explicit window: keep its size, just ensure its title bar and frame stay inside work area.
    let w = rect.width().min(work.width());
    let h = rect.height().min(work.height());
    let x = rect.left.clamp(work.left, (work.right - w).max(work.left));
    let y = rect.top.clamp(work.top, (work.bottom - h).max(work.top));
    RECT::new(x, y, x + w, y + h)
}

pub unsafe fn create_window(a: CreateArgs) -> usize {
    let Some(cls) = class::find(&a.class) else {
        log::warn!("CreateWindowEx: class {} not registered", a.class);
        set_error(e::CANNOT_FIND_WND_CLASS);
        return 0;
    };
    let mut style = a.style;
    let mut exstyle = a.exstyle;
    let parent = if a.parent == HWND_MESSAGE { 0 } else { a.parent };
    if style & ws::CHILD != 0 && (parent == 0 || !exists(parent)) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    }
    // Overlapped windows always get a caption (Windows forces WS_CLIPSIBLINGS|WS_CAPTION).
    if style & (ws::CHILD | ws::POPUP) == 0 {
        style |= ws::CAPTION;
    }
    (style, exstyle) = framed_top_level_style(style, exstyle);
    let (x, y, w, h) = default_rect(style, &a);
    let tid = crate::teb::thread_id();
    let _ = message::current_queue();
    let hwnd = NEXT_HWND.fetch_add(4, Ordering::Relaxed);
    let mut rect = RECT::new(x, y, x + w, y + h);
    if let Some(locked) = single_surface_top_level_rect(style) {
        rect = locked;
    } else if style & ws::CHILD != 0 {
        // Child coordinates are relative to the parent's client area.
        rect = RECT::new(x, y, x + w, y + h);
    } else {
        rect = fit_framed_window_rect(rect, style, exstyle);
    }
    let (x, y, w, h) = (rect.left, rect.top, rect.width(), rect.height());
    let client = client_from_window(&rect, style, exstyle);
    let topmost = exstyle & ws::EX_TOPMOST != 0;
    let win = Arc::new(Window {
        hwnd,
        tid,
        class: cls.clone(),
        d: Mutex::new(WinData {
            wndproc: cls.wndproc,
            style: style & !ws::VISIBLE,
            exstyle,
            parent: if style & ws::CHILD != 0 { parent } else { 0 },
            owner: if style & ws::CHILD == 0 { parent } else { 0 },
            id: if style & ws::CHILD != 0 { a.menu } else { 0 },
            instance: a.instance,
            userdata: 0,
            extra: vec![0; cls.wnd_extra as usize],
            text: Vec::new(),
            rect,
            client,
            normal: rect,
            visible: false,
            minimized: false,
            maximized: false,
            enabled: style & ws::DISABLED == 0,
            topmost,
            native: false,
            destroying: false,
            erase_pending: true,
        }),
    });
    {
        let mut t = table().write().unwrap();
        t.map.insert(hwnd, win.clone());
        if style & ws::CHILD == 0 {
            t.zorder.insert(0, hwnd);
        }
    }
    log::info!(
        "CreateWindowEx hwnd={hwnd:#x} class={} title={:?} style={style:#x} ex={exstyle:#x} rect=({x},{y} {w}x{h}) thread={tid:#x}",
        cls.name,
        String::from_utf16_lossy(&a.name),
    );

    let mut cs = CREATESTRUCT {
        create_params: a.param,
        instance: a.instance,
        menu: a.menu,
        parent: a.parent,
        cy: h,
        cx: w,
        y,
        x,
        style: style as i32,
        name: a.name_ptr,
        class: a.class_ptr,
        exstyle,
    };
    let name_z: Vec<u16> = a.name.iter().copied().chain(std::iter::once(0)).collect();
    if cs.name != 0 {
        cs.name = name_z.as_ptr() as usize;
    }
    let csp = &mut cs as *mut CREATESTRUCT as LPARAM;

    if style & ws::CHILD == 0 && style & (ws::CAPTION | ws::THICKFRAME) != 0 {
        let mut mmi = default_minmax(style, a.exstyle);
        send_message(hwnd, wm::GETMINMAXINFO, 0, &mut mmi as *mut MINMAXINFO as LPARAM);
    }
    if send_message(hwnd, wm::NCCREATE, 0, csp) == 0 {
        log::warn!("CreateWindowEx {hwnd:#x}: WM_NCCREATE returned FALSE");
        free_window(hwnd);
        return 0;
    }
    // WM_NCCALCSIZE: let the window compute its client area.
    let mut r = rect;
    send_message(hwnd, wm::NCCALCSIZE, 0, &mut r as *mut RECT as LPARAM);
    win.data().client = r;

    if style & ws::CHILD == 0 {
        let desc = native_desc(&win, &win.data());
        if crate::macos::window_create(hwnd, desc) {
            win.data().native = true;
        }
    }

    if send_message(hwnd, wm::CREATE, 0, csp) == -1 {
        log::warn!("CreateWindowEx {hwnd:#x}: WM_CREATE returned -1");
        DestroyWindow(hwnd as HWND);
        return 0;
    }
    if !exists(hwnd) {
        return 0;
    }
    // Size/move notifications (CreateWindow sends these after WM_CREATE).
    let (c, wr) = {
        let d = win.data();
        (d.client, d.rect)
    };
    let origin = client_origin_in_parent(&win, &c, &wr);
    send_message(hwnd, wm::SIZE, SIZE_RESTORED, make_long(c.width(), c.height()));
    send_message(hwnd, wm::MOVE, 0, make_long(origin.x, origin.y));

    if style & ws::VISIBLE != 0 {
        let cmd = if style & ws::MINIMIZE != 0 {
            SW_SHOWMINIMIZED
        } else if style & ws::MAXIMIZE != 0 {
            SW_MAXIMIZE
        } else {
            SW_SHOW
        };
        show_window_impl(&win, cmd, true);
    }
    hwnd
}

fn client_origin_in_parent(_w: &Window, c: &RECT, _r: &RECT) -> POINT {
    POINT { x: c.left, y: c.top }
}

fn default_minmax(style: u32, exstyle: u32) -> MINMAXINFO {
    let work = super::monitor::primary_work_area();
    let full = window_from_client(&work, style, exstyle);
    MINMAXINFO {
        reserved: POINT::default(),
        max_size: POINT { x: full.width(), y: full.height() },
        max_position: POINT { x: full.left, y: full.top },
        min_track: POINT { x: 136, y: 39 },
        max_track: POINT { x: super::misc::virtual_screen().width() + 16, y: super::misc::virtual_screen().height() + 16 },
    }
}

fn free_window(hwnd: usize) {
    let w = {
        let mut t = table().write().unwrap();
        t.zorder.retain(|&h| h != hwnd);
        t.map.remove(&hwnd)
    };
    if let Some(w) = w {
        if let Some(q) = message::queue_of(w.tid) {
            q.purge_window(hwnd);
        }
        if w.data().native {
            crate::macos::window_destroy(hwnd);
        }
    }
    super::input::forget_window(hwnd);
    super::misc::forget_device_notifications(hwnd);
    crate::misc::uxtheme::forget_window_subclasses(hwnd);
    super::dialog::forget_dialog(hwnd);
}

pub unsafe fn create_window_w(
    exstyle: u32,
    class: LPCWSTR,
    name: LPCWSTR,
    style: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: HWND,
    menu: usize,
    inst: HINSTANCE,
    param: LPVOID,
) -> HWND {
    if class.is_null() {
        set_error(e::CANNOT_FIND_WND_CLASS);
        return std::ptr::null_mut();
    }
    let a = CreateArgs {
        exstyle,
        class: ClassName::from_w(class),
        name: if name.is_null() { Vec::new() } else { crate::strings::wide_slice(name).to_vec() },
        style,
        x,
        y,
        w,
        h,
        parent: parent as usize,
        menu,
        instance: inst as usize,
        param: param as usize,
        class_ptr: class as usize,
        name_ptr: name as usize,
    };
    create_window(a) as HWND
}

pub unsafe extern "win64" fn CreateWindowExW(
    exstyle: u32,
    class: LPCWSTR,
    name: LPCWSTR,
    style: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: HWND,
    menu: usize,
    inst: HINSTANCE,
    param: LPVOID,
) -> HWND {
    create_window_w(exstyle, class, name, style, x, y, w, h, parent, menu, inst, param)
}

pub unsafe extern "win64" fn CreateWindowExA(
    exstyle: u32,
    class: LPCSTR,
    name: LPCSTR,
    style: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: HWND,
    menu: usize,
    inst: HINSTANCE,
    param: LPVOID,
) -> HWND {
    let cls_w;
    let cls_ptr = if is_int_resource(class as *const u16) {
        class as *const u16
    } else {
        cls_w = crate::strings::to_wide_z(&crate::strings::from_ansi(class));
        cls_w.as_ptr()
    };
    let name_w = crate::strings::from_ansi_opt(name).map(|s| crate::strings::to_wide_z(&s));
    let name_ptr = name_w.as_ref().map(|v| v.as_ptr()).unwrap_or(std::ptr::null());
    create_window_w(exstyle, cls_ptr, name_ptr, style, x, y, w, h, parent, menu, inst, param)
}

pub unsafe extern "win64" fn DestroyWindow(hwnd: HWND) -> BOOL {
    let hwnd = hwnd as usize;
    let Some(w) = require(hwnd) else { return FALSE };
    if w.tid != crate::teb::thread_id() {
        set_error(5); // ERROR_ACCESS_DENIED
        return FALSE;
    }
    {
        let mut d = w.data();
        if d.destroying {
            return TRUE;
        }
        d.destroying = true;
    }
    log::info!("DestroyWindow {hwnd:#x}");
    // Hide first (sends WINDOWPOSCHANGING/CHANGED), then deactivate.
    if w.data().visible {
        set_window_pos_impl(&w, 0, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE, true);
    }
    super::input::window_going_away(hwnd);
    send_message(hwnd, wm::DESTROY, 0, 0);
    // Children (and owned windows) go first-to-last.
    let children: Vec<usize> = {
        let t = table().read().unwrap();
        t.map
            .values()
            .filter(|c| {
                let d = c.data();
                (d.style & ws::CHILD != 0 && d.parent == hwnd) || (d.style & ws::CHILD == 0 && d.owner == hwnd)
            })
            .map(|c| c.hwnd)
            .collect()
    };
    for c in children {
        DestroyWindow(c as HWND);
    }
    send_message(hwnd, wm::NCDESTROY, 0, 0);
    free_window(hwnd);
    TRUE
}

pub unsafe extern "win64" fn IsWindow(hwnd: HWND) -> BOOL {
    exists(hwnd as usize) as BOOL
}

pub unsafe extern "win64" fn IsWindowVisible(hwnd: HWND) -> BOOL {
    let mut cur = hwnd as usize;
    for _ in 0..64 {
        let Some(w) = get(cur) else { return FALSE };
        let d = w.data();
        if !d.visible {
            return FALSE;
        }
        if d.style & ws::CHILD == 0 {
            return TRUE;
        }
        cur = d.parent;
    }
    FALSE
}

pub unsafe extern "win64" fn IsIconic(hwnd: HWND) -> BOOL {
    get(hwnd as usize).map(|w| w.data().minimized as BOOL).unwrap_or(FALSE)
}

pub unsafe extern "win64" fn IsZoomed(hwnd: HWND) -> BOOL {
    get(hwnd as usize).map(|w| w.data().maximized as BOOL).unwrap_or(FALSE)
}

pub unsafe extern "win64" fn IsWindowEnabled(hwnd: HWND) -> BOOL {
    get(hwnd as usize).map(|w| w.data().enabled as BOOL).unwrap_or(FALSE)
}

pub unsafe extern "win64" fn EnableWindow(hwnd: HWND, enable: BOOL) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    let was_disabled = {
        let mut d = w.data();
        let was = !d.enabled;
        d.enabled = enable != 0;
        if enable != 0 {
            d.style &= !ws::DISABLED;
        } else {
            d.style |= ws::DISABLED;
        }
        was
    };
    if was_disabled == (enable != 0) {
        send_message(hwnd as usize, wm::ENABLE, (enable != 0) as usize, 0);
    }
    was_disabled as BOOL
}

// ---------------------------------------------------------------------------
// Position / size / visibility
// ---------------------------------------------------------------------------

/// SetWindowPos core. `internal` suppresses activation side effects used by
/// ShowWindow/DestroyWindow.
pub unsafe fn set_window_pos_impl(w: &Arc<Window>, after: usize, x: i32, y: i32, cx: i32, cy: i32, mut flags: u32, internal: bool) -> bool {
    let hwnd = w.hwnd;
    let (old_rect, old_client, style, exstyle, was_visible) = {
        let d = w.data();
        (d.rect, d.client, d.style, d.exstyle, d.visible)
    };
    let mut wp = WINDOWPOS {
        hwnd,
        insert_after: after,
        x: if flags & SWP_NOMOVE != 0 { old_rect.left } else { x },
        y: if flags & SWP_NOMOVE != 0 { old_rect.top } else { y },
        cx: if flags & SWP_NOSIZE != 0 { old_rect.width() } else { cx.max(0) },
        cy: if flags & SWP_NOSIZE != 0 { old_rect.height() } else { cy.max(0) },
        flags,
    };
    if wp.x == old_rect.left && wp.y == old_rect.top {
        wp.flags |= SWP_NOMOVE;
    }
    if wp.cx == old_rect.width() && wp.cy == old_rect.height() {
        wp.flags |= SWP_NOSIZE;
    }
    if flags & SWP_SHOWWINDOW != 0 && was_visible {
        wp.flags &= !SWP_SHOWWINDOW;
    }
    if flags & SWP_HIDEWINDOW != 0 && !was_visible {
        wp.flags &= !SWP_HIDEWINDOW;
    }
    if wp.flags & SWP_NOSENDCHANGING == 0 {
        send_message(hwnd, wm::WINDOWPOSCHANGING, 0, &mut wp as *mut WINDOWPOS as LPARAM);
    }
    flags = wp.flags;
    let new_rect = RECT::new(wp.x, wp.y, wp.x + wp.cx, wp.y + wp.cy);
    let mut new_client = old_client;
    if new_rect != old_rect || flags & SWP_FRAMECHANGED != 0 {
        let mut r = new_rect;
        send_message(hwnd, wm::NCCALCSIZE, 0, &mut r as *mut RECT as LPARAM);
        new_client = r;
    }
    let (native, minimized) = {
        let mut d = w.data();
        d.rect = new_rect;
        d.client = new_client;
        if flags & SWP_SHOWWINDOW != 0 {
            d.visible = true;
            d.style |= ws::VISIBLE;
        }
        if flags & SWP_HIDEWINDOW != 0 {
            d.visible = false;
            d.style &= !ws::VISIBLE;
        }
        if flags & SWP_NOZORDER == 0 {
            match after {
                HWND_TOPMOST => d.topmost = true,
                HWND_NOTOPMOST => d.topmost = false,
                _ => {}
            }
        }
        (d.native, d.minimized)
    };
    let _ = (style, exstyle);
    if flags & SWP_NOZORDER == 0 && (after == HWND_TOP || after == HWND_TOPMOST) {
        bring_to_front(hwnd);
    }
    if native {
        let d = w.data();
        if flags & SWP_FRAMECHANGED != 0 || flags & SWP_NOZORDER == 0 {
            crate::macos::window_set_style(hwnd, native_desc(w, &d));
        }
        if (new_client != old_client || flags & SWP_FRAMECHANGED != 0) && !minimized {
            crate::macos::window_set_frame(hwnd, d.client);
        }
        drop(d);
        if flags & SWP_HIDEWINDOW != 0 {
            crate::macos::window_show(hwnd, false, false);
        }
        if flags & SWP_SHOWWINDOW != 0 {
            crate::macos::window_show(hwnd, true, flags & SWP_NOACTIVATE == 0);
        }
    }
    if flags & SWP_SHOWWINDOW != 0 || (new_client != old_client && w.data().visible) {
        invalidate(w);
    }
    if flags & SWP_HIDEWINDOW != 0 {
        if let Some(q) = message::queue_of(w.tid) {
            q.validate(hwnd);
        }
        super::input::window_hidden(hwnd);
    }
    if !internal && flags & SWP_NOACTIVATE == 0 && w.data().visible && is_top_level(&w.data()) {
        super::input::set_active_window(hwnd, false);
    }
    send_message(hwnd, wm::WINDOWPOSCHANGED, 0, &mut wp as *mut WINDOWPOS as LPARAM);
    true
}

fn invalidate(w: &Window) {
    w.data().erase_pending = true;
    if let Some(q) = message::queue_of(w.tid) {
        q.invalidate(w.hwnd);
    }
}

pub unsafe extern "win64" fn SetWindowPos(hwnd: HWND, after: HWND, mut x: i32, mut y: i32, mut cx: i32, mut cy: i32, mut flags: u32) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    let mut a = after as usize;
    if crate::macos::single_surface_mode() {
        let (top_level, style) = {
            let d = w.data();
            (is_top_level(&d), d.style)
        };
        if top_level {
            if a == HWND_TOPMOST {
                a = HWND_TOP;
            }
            if let Some(locked) = single_surface_top_level_rect(style) {
                if flags & (SWP_NOMOVE | SWP_NOSIZE) != (SWP_NOMOVE | SWP_NOSIZE) {
                    x = locked.left;
                    y = locked.top;
                    cx = locked.width();
                    cy = locked.height();
                    flags &= !(SWP_NOMOVE | SWP_NOSIZE);
                }
            }
        }
    } else if crate::macos::framed_mode() {
        let (top_level, old_rect, style, exstyle) = {
            let d = w.data();
            (is_top_level(&d), d.rect, d.style, d.exstyle)
        };
        if top_level {
            if a == HWND_TOPMOST {
                a = HWND_TOP;
            }
            if flags & (SWP_NOMOVE | SWP_NOSIZE) != (SWP_NOMOVE | SWP_NOSIZE) {
                let rx = if flags & SWP_NOMOVE != 0 { old_rect.left } else { x };
                let ry = if flags & SWP_NOMOVE != 0 { old_rect.top } else { y };
                let rw = if flags & SWP_NOSIZE != 0 { old_rect.width() } else { cx.max(0) };
                let rh = if flags & SWP_NOSIZE != 0 { old_rect.height() } else { cy.max(0) };
                let req = RECT::new(rx, ry, rx + rw, ry + rh);
                let fitted = fit_framed_window_rect(req, style, exstyle);
                if fitted != req {
                    x = fitted.left;
                    y = fitted.top;
                    cx = fitted.width();
                    cy = fitted.height();
                    flags &= !(SWP_NOMOVE | SWP_NOSIZE);
                }
            }
        }
    }
    log::debug!("SetWindowPos {:#x} after={a:#x} ({x},{y} {cx}x{cy}) flags={flags:#x}", hwnd as usize);
    if w.tid != crate::teb::thread_id() {
        // Cross-thread: performed by the owner thread, like Windows does
        // (synchronously unless SWP_ASYNCWINDOWPOS).
        let h = hwnd as usize;
        if let Some(q) = message::queue_of(w.tid) {
            q.call(move || {
                if let Some(w) = get(h) {
                    set_window_pos_impl(&w, a, x, y, cx, cy, flags, false);
                }
            });
        }
        return TRUE;
    }
    set_window_pos_impl(&w, a, x, y, cx, cy, flags, false) as BOOL
}

pub unsafe extern "win64" fn MoveWindow(hwnd: HWND, x: i32, y: i32, w: i32, h: i32, _repaint: BOOL) -> BOOL {
    SetWindowPos(hwnd, std::ptr::null_mut(), x, y, w, h, SWP_NOZORDER | SWP_NOACTIVATE)
}

pub unsafe extern "win64" fn BringWindowToTop(hwnd: HWND) -> BOOL {
    SetWindowPos(hwnd, std::ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)
}

pub unsafe extern "win64" fn SetLayeredWindowAttributes(
    hwnd: HWND,
    _key: u32,
    _alpha: u8,
    _flags: u32,
) -> BOOL {
    if require(hwnd as usize).is_none() {
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetLayeredWindowAttributes(
    hwnd: HWND,
    key: *mut u32,
    alpha: *mut u8,
    flags: *mut u32,
) -> BOOL {
    if require(hwnd as usize).is_none() {
        return FALSE;
    }
    if !key.is_null() {
        std::ptr::write_unaligned(key, 0);
    }
    if !alpha.is_null() {
        std::ptr::write_unaligned(alpha, 255);
    }
    if !flags.is_null() {
        std::ptr::write_unaligned(flags, 0x2); // LWA_ALPHA
    }
    TRUE
}

pub unsafe extern "win64" fn FlashWindow(_hwnd: HWND, _invert: BOOL) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn FlashWindowEx(_info: *const u8) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn SetWindowRgn(hwnd: HWND, _hrgn: usize, _redraw: BOOL) -> i32 {
    if require(hwnd as usize).is_none() { 0 } else { 1 }
}

pub unsafe extern "win64" fn TrackMouseEvent(tme: *mut u8) -> BOOL {
    if tme.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetDoubleClickTime() -> u32 {
    500
}

/// ShowWindow core; returns the previous visibility.
pub unsafe fn show_window_impl(w: &Arc<Window>, cmd: i32, from_guest: bool) -> bool {
    let hwnd = w.hwnd;
    let (was_visible, minimized, maximized, native) = {
        let d = w.data();
        (d.visible, d.minimized, d.maximized, d.native)
    };
    log::debug!("ShowWindow {hwnd:#x} cmd={cmd} (visible={was_visible} min={minimized} max={maximized})");
    let activate = !matches!(cmd, SW_SHOWNOACTIVATE | SW_SHOWNA | SW_SHOWMINNOACTIVE | SW_HIDE | SW_MINIMIZE | SW_FORCEMINIMIZE);
    match cmd {
        SW_HIDE => {
            if !was_visible {
                return false;
            }
            send_message(hwnd, wm::SHOWWINDOW, 0, 0);
            set_window_pos_impl(w, 0, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE, true);
            return true;
        }
        SW_MINIMIZE | SW_SHOWMINIMIZED | SW_SHOWMINNOACTIVE | SW_FORCEMINIMIZE => {
            if !was_visible {
                send_message(hwnd, wm::SHOWWINDOW, 1, 0);
            }
            if !minimized {
                let r = {
                    let mut d = w.data();
                    if !d.maximized {
                        d.normal = d.rect;
                    }
                    d.minimized = true;
                    d.style |= ws::MINIMIZE;
                    d.rect
                };
                let _ = r;
                if native && from_guest {
                    crate::macos::window_minimize(hwnd, true);
                }
                let (mw, mh) = (160, (crate::macos::titlebar_height_pt() * scale()) as i32);
                set_window_pos_impl(
                    w,
                    0,
                    MINIMIZED_POS,
                    MINIMIZED_POS,
                    mw,
                    mh,
                    SWP_NOZORDER | SWP_NOACTIVATE | SWP_STATECHANGED | if was_visible { 0 } else { SWP_SHOWWINDOW },
                    true,
                );
            }
            if super::input::active_window() == hwnd {
                super::input::window_hidden(hwnd);
            }
            return was_visible;
        }
        SW_MAXIMIZE => {
            if !was_visible {
                send_message(hwnd, wm::SHOWWINDOW, 1, 0);
            }
            let (style, exstyle) = {
                let mut d = w.data();
                if !d.maximized && !d.minimized {
                    d.normal = d.rect;
                }
                d.maximized = true;
                d.minimized = false;
                d.style = (d.style | ws::MAXIMIZE) & !ws::MINIMIZE;
                (d.style, d.exstyle)
            };
            if minimized && native {
                crate::macos::window_minimize(hwnd, false);
            }
            let work = super::monitor::work_area_for_window(hwnd);
            let s = super::monitor::scale_for_window(hwnd);
            let r = window_from_client_for_scale(&work, style, exstyle, s);
            set_window_pos_impl(
                w,
                0,
                r.left,
                r.top,
                r.width(),
                r.height(),
                SWP_NOZORDER | SWP_FRAMECHANGED | SWP_STATECHANGED | SWP_SHOWWINDOW | if activate { 0 } else { SWP_NOACTIVATE },
                true,
            );
        }
        _ => {
            // SW_SHOWNORMAL / SW_SHOW / SW_SHOWNA / SW_SHOWNOACTIVATE / SW_RESTORE / SW_SHOWDEFAULT
            if !was_visible {
                send_message(hwnd, wm::SHOWWINDOW, 1, 0);
            }
            let restore = matches!(cmd, SW_SHOWNORMAL | SW_RESTORE | SW_SHOWDEFAULT | SW_SHOWNOACTIVATE) && (minimized || maximized);
            let restore = restore || (cmd == SW_SHOW && minimized && false);
            if restore {
                let n = {
                    let mut d = w.data();
                    d.minimized = false;
                    d.maximized = false;
                    d.style &= !(ws::MINIMIZE | ws::MAXIMIZE);
                    d.normal
                };
                if minimized && native && from_guest {
                    crate::macos::window_minimize(hwnd, false);
                }
                set_window_pos_impl(
                    w,
                    0,
                    n.left,
                    n.top,
                    n.width(),
                    n.height(),
                    SWP_NOZORDER | SWP_STATECHANGED | SWP_FRAMECHANGED | SWP_SHOWWINDOW | if activate { 0 } else { SWP_NOACTIVATE },
                    true,
                );
            } else {
                set_window_pos_impl(
                    w,
                    0,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOMOVE | SWP_SHOWWINDOW | if activate { 0 } else { SWP_NOACTIVATE | SWP_NOZORDER },
                    true,
                );
            }
        }
    }
    if activate && w.data().visible && is_top_level(&w.data()) {
        super::input::set_active_window(hwnd, false);
    }
    was_visible
}

pub unsafe extern "win64" fn ShowWindow(hwnd: HWND, cmd: i32) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if w.tid != crate::teb::thread_id() {
        let h = hwnd as usize;
        let was = w.data().visible;
        if let Some(q) = message::queue_of(w.tid) {
            q.call(move || {
                if let Some(w) = get(h) {
                    show_window_impl(&w, cmd, true);
                }
            });
        }
        return was as BOOL;
    }
    show_window_impl(&w, cmd, true) as BOOL
}

pub unsafe extern "win64" fn ShowWindowAsync(hwnd: HWND, cmd: i32) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    let h = hwnd as usize;
    if let Some(q) = message::queue_of(w.tid) {
        q.call(move || {
            if let Some(w) = get(h) {
                show_window_impl(&w, cmd, true);
            }
        });
    }
    TRUE
}

pub unsafe extern "win64" fn UpdateWindow(hwnd: HWND) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    let pending = message::queue_of(w.tid).is_some() && w.data().visible;
    if pending && w.tid == crate::teb::thread_id() {
        // Send WM_PAINT directly if the window is invalid.
        let invalid = {
            let mut m = MSG::default();
            message::retrieve(
                message::Filter { hwnd: hwnd as usize, min: wm::PAINT, max: wm::PAINT, kinds: message::QS_PAINT },
                false,
                false,
            )
            .map(|x| {
                m = x;
                true
            })
            .unwrap_or(false)
        };
        if invalid {
            send_message(hwnd as usize, wm::PAINT, 0, 0);
        }
    }
    TRUE
}

// ---------------------------------------------------------------------------
// Geometry queries
// ---------------------------------------------------------------------------

/// Screen offset of a window's coordinate space (0 for top-level).
fn parent_client_origin(d: &WinData) -> POINT {
    if d.style & ws::CHILD == 0 {
        return POINT::default();
    }
    match get(d.parent) {
        Some(p) => {
            let pd = p.data();
            let o = parent_client_origin(&pd);
            POINT { x: o.x + pd.client.left, y: o.y + pd.client.top }
        }
        None => POINT::default(),
    }
}

/// Screen coordinates of the client-area origin.
pub fn client_origin_screen(hwnd: usize) -> Option<POINT> {
    if hwnd == DESKTOP_HWND {
        return Some(POINT::default());
    }
    let w = get(hwnd)?;
    let d = w.data();
    let o = parent_client_origin(&d);
    Some(POINT { x: o.x + d.client.left, y: o.y + d.client.top })
}

pub fn client_size(hwnd: usize) -> Option<(i32, i32)> {
    if hwnd == DESKTOP_HWND {
        let v = super::misc::virtual_screen();
        return Some((v.width(), v.height()));
    }
    let w = get(hwnd)?;
    let d = w.data();
    if d.minimized {
        return Some((0, 0));
    }
    Some((d.client.width(), d.client.height()))
}

pub fn window_rect_screen(hwnd: usize) -> Option<RECT> {
    if hwnd == DESKTOP_HWND {
        return Some(super::misc::virtual_screen());
    }
    let w = get(hwnd)?;
    let d = w.data();
    let o = parent_client_origin(&d);
    Some(d.rect.offset(o.x, o.y))
}

pub unsafe extern "win64" fn GetWindowRect(hwnd: HWND, out: *mut RECT) -> BOOL {
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    match window_rect_screen(hwnd as usize) {
        Some(r) => {
            *out = r;
            TRUE
        }
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetClientRect(hwnd: HWND, out: *mut RECT) -> BOOL {
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    match client_size(hwnd as usize) {
        Some((w, h)) => {
            *out = RECT::new(0, 0, w, h);
            TRUE
        }
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn ClientToScreen(hwnd: HWND, p: *mut POINT) -> BOOL {
    if p.is_null() {
        return FALSE;
    }
    match client_origin_screen(hwnd as usize) {
        Some(o) => {
            (*p).x += o.x;
            (*p).y += o.y;
            TRUE
        }
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn ScreenToClient(hwnd: HWND, p: *mut POINT) -> BOOL {
    if p.is_null() {
        return FALSE;
    }
    match client_origin_screen(hwnd as usize) {
        Some(o) => {
            (*p).x -= o.x;
            (*p).y -= o.y;
            TRUE
        }
        None => {
            set_error(e::INVALID_WINDOW_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn MapWindowPoints(from: HWND, to: HWND, pts: *mut POINT, n: u32) -> i32 {
    let o1 = if from.is_null() { Some(POINT::default()) } else { client_origin_screen(from as usize) };
    let o2 = if to.is_null() { Some(POINT::default()) } else { client_origin_screen(to as usize) };
    let (Some(a), Some(b)) = (o1, o2) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    };
    let (dx, dy) = (a.x - b.x, a.y - b.y);
    for i in 0..n as usize {
        let p = &mut *pts.add(i);
        p.x += dx;
        p.y += dy;
    }
    set_error(0);
    make_long(dx, dy) as i32
}

#[repr(C)]
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: u32,
    pub show_cmd: u32,
    pub min_pos: POINT,
    pub max_pos: POINT,
    pub normal: RECT,
}

pub unsafe extern "win64" fn GetWindowPlacement(hwnd: HWND, out: *mut WINDOWPLACEMENT) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let d = w.data();
    let work = super::monitor::primary_work_area();
    let normal = if d.minimized || d.maximized { d.normal } else { d.rect };
    *out = WINDOWPLACEMENT {
        length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
        flags: 0,
        show_cmd: if d.minimized {
            SW_SHOWMINIMIZED as u32
        } else if d.maximized {
            SW_MAXIMIZE as u32
        } else {
            SW_SHOWNORMAL as u32
        },
        min_pos: POINT { x: -1, y: -1 },
        max_pos: POINT { x: -1, y: -1 },
        // Workspace coordinates: relative to the work area origin.
        normal: normal.offset(-work.left, -work.top),
    };
    TRUE
}

pub unsafe extern "win64" fn SetWindowPlacement(hwnd: HWND, p: *const WINDOWPLACEMENT) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if p.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let work = super::monitor::primary_work_area();
    let style = w.data().style;
    let n = single_surface_top_level_rect(style)
        .unwrap_or_else(|| (*p).normal.offset(work.left, work.top));
    w.data().normal = n;
    let cmd = (*p).show_cmd as i32;
    if !w.data().minimized && !w.data().maximized {
        set_window_pos_impl(&w, 0, n.left, n.top, n.width(), n.height(), SWP_NOZORDER | SWP_NOACTIVATE, true);
    }
    show_window_impl(&w, cmd, true);
    TRUE
}

#[repr(C)]
pub struct WINDOWINFO {
    pub cb_size: u32,
    pub rc_window: RECT,
    pub rc_client: RECT,
    pub dw_style: u32,
    pub dw_ex_style: u32,
    pub dw_window_status: u32,
    pub cx_window_borders: u32,
    pub cy_window_borders: u32,
    pub atom_window_type: u16,
    pub w_creator_version: u16,
}

pub unsafe extern "win64" fn GetWindowInfo(hwnd: HWND, out: *mut WINDOWINFO) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let d = w.data();
    let active = super::input::active_window() == hwnd as usize;
    let cx = ((d.client.left - d.rect.left).max(0)) as u32;
    let cy = ((d.rect.bottom - d.client.bottom).max(0)) as u32;
    std::ptr::write_unaligned(
        out,
        WINDOWINFO {
            cb_size: std::mem::size_of::<WINDOWINFO>() as u32,
            rc_window: d.rect,
            rc_client: d.client,
            dw_style: d.style,
            dw_ex_style: d.exstyle,
            dw_window_status: if active { 1 } else { 0 },
            cx_window_borders: cx,
            cy_window_borders: cy,
            atom_window_type: 0,
            w_creator_version: 0x0A00,
        },
    );
    TRUE
}

pub unsafe extern "win64" fn SetWindowDisplayAffinity(hwnd: HWND, _affinity: u32) -> BOOL {
    if require(hwnd as usize).is_none() {
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetWindowDisplayAffinity(hwnd: HWND, affinity: *mut u32) -> BOOL {
    if require(hwnd as usize).is_none() {
        return FALSE;
    }
    if !affinity.is_null() {
        std::ptr::write_unaligned(affinity, 0);
    }
    TRUE
}

pub unsafe extern "win64" fn WindowFromPoint(p: POINT) -> HWND {
    for h in top_level_windows() {
        let Some(w) = get(h) else { continue };
        let d = w.data();
        if d.visible && !d.minimized && d.rect.contains(p.x, p.y) {
            return h as HWND;
        }
    }
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn GetDesktopWindow() -> HWND {
    DESKTOP_HWND as HWND
}

pub unsafe extern "win64" fn GetShellWindow() -> HWND {
    DESKTOP_HWND as HWND
}

pub unsafe extern "win64" fn GetWindowThreadProcessId(hwnd: HWND, pid: *mut u32) -> u32 {
    let Some(w) = require(hwnd as usize) else { return 0 };
    if !pid.is_null() {
        *pid = libc::getpid() as u32;
    }
    w.tid
}

pub unsafe extern "win64" fn GetParent(hwnd: HWND) -> HWND {
    let Some(w) = require(hwnd as usize) else { return std::ptr::null_mut() };
    let d = w.data();
    (if d.style & ws::CHILD != 0 { d.parent } else { d.owner }) as HWND
}

pub unsafe extern "win64" fn IsChild(parent: HWND, hwnd: HWND) -> BOOL {
    is_child_of(hwnd as usize, parent as usize) as BOOL
}

pub unsafe extern "win64" fn GetAncestor(hwnd: HWND, flags: u32) -> HWND {
    let Some(w) = require(hwnd as usize) else { return std::ptr::null_mut() };
    match flags {
        1 => {
            // GA_PARENT
            let d = w.data();
            (if d.style & ws::CHILD != 0 { d.parent } else { DESKTOP_HWND }) as HWND
        }
        _ => {
            // GA_ROOT / GA_ROOTOWNER
            let mut cur = hwnd as usize;
            for _ in 0..64 {
                let Some(w) = get(cur) else { break };
                let d = w.data();
                if d.style & ws::CHILD != 0 {
                    cur = d.parent;
                } else if flags == 3 && d.owner != 0 {
                    cur = d.owner;
                } else {
                    break;
                }
            }
            cur as HWND
        }
    }
}

const GW_HWNDFIRST: u32 = 0;
const GW_HWNDLAST: u32 = 1;
const GW_HWNDNEXT: u32 = 2;
const GW_HWNDPREV: u32 = 3;
const GW_OWNER: u32 = 4;
const GW_CHILD: u32 = 5;

fn siblings_of(hwnd: usize) -> Vec<usize> {
    let Some(w) = get(hwnd) else { return Vec::new() };
    let d = w.data();
    if d.style & ws::CHILD == 0 {
        return top_level_windows();
    }
    let parent = d.parent;
    drop(d);
    children_of(parent)
}

pub(crate) fn children_of(parent: usize) -> Vec<usize> {
    let t = table().read().unwrap();
    let mut v: Vec<usize> = t
        .map
        .values()
        .filter(|c| {
            let d = c.data();
            d.style & ws::CHILD != 0 && d.parent == parent
        })
        .map(|c| c.hwnd)
        .collect();
    v.sort();
    v
}

pub unsafe extern "win64" fn GetWindow(hwnd: HWND, cmd: u32) -> HWND {
    let h = hwnd as usize;
    if h != DESKTOP_HWND && !exists(h) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return std::ptr::null_mut();
    }
    let r = match cmd {
        GW_CHILD => {
            if h == DESKTOP_HWND {
                top_level_windows().first().copied()
            } else {
                children_of(h).first().copied()
            }
        }
        GW_OWNER => get(h).map(|w| w.data().owner).filter(|&o| o != 0),
        GW_HWNDFIRST => siblings_of(h).first().copied(),
        GW_HWNDLAST => siblings_of(h).last().copied(),
        GW_HWNDNEXT | GW_HWNDPREV => {
            let s = siblings_of(h);
            s.iter().position(|&x| x == h).and_then(|i| {
                if cmd == GW_HWNDNEXT {
                    s.get(i + 1).copied()
                } else if i > 0 {
                    s.get(i - 1).copied()
                } else {
                    None
                }
            })
        }
        _ => {
            set_error(e::INVALID_PARAMETER);
            None
        }
    };
    r.unwrap_or(0) as HWND
}

pub unsafe extern "win64" fn GetTopWindow(hwnd: HWND) -> HWND {
    if hwnd.is_null() || hwnd as usize == DESKTOP_HWND {
        return top_level_windows().first().copied().unwrap_or(0) as HWND;
    }
    children_of(hwnd as usize).first().copied().unwrap_or(0) as HWND
}

pub unsafe extern "win64" fn EnumWindows(cb: usize, lparam: LPARAM) -> BOOL {
    if cb == 0 {
        return FALSE;
    }
    for h in top_level_windows() {
        if crate::call_guest8(cb, h as u64, lparam as u64, 0, 0, 0, 0, 0, 0) as BOOL == 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn EnumThreadWindows(tid: u32, cb: usize, lparam: LPARAM) -> BOOL {
    if cb == 0 {
        return FALSE;
    }
    for h in top_level_windows() {
        if get(h).map(|w| w.tid) == Some(tid)
            && crate::call_guest8(cb, h as u64, lparam as u64, 0, 0, 0, 0, 0, 0) as BOOL == 0
        {
            break;
        }
    }
    TRUE
}

unsafe fn find_window_ex(parent: usize, child_after: usize, class: Option<ClassName>, title: Option<String>) -> usize {
    let list = if parent == 0 || parent == DESKTOP_HWND {
        top_level_windows()
    } else if parent == HWND_MESSAGE {
        Vec::new()
    } else if exists(parent) {
        children_of(parent)
    } else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    };
    let start = if child_after == 0 {
        0
    } else {
        match list.iter().position(|&x| x == child_after) {
            Some(idx) => idx + 1,
            None => return 0,
        }
    };
    for &h in &list[start..] {
        let Some(w) = get(h) else { continue };
        if let Some(c) = &class {
            let ok = match c {
                ClassName::Atom(a) => w.class.atom == *a,
                ClassName::Name(n) => w.class.name.eq_ignore_ascii_case(n),
            };
            if !ok {
                continue;
            }
        }
        if let Some(t) = &title {
            if String::from_utf16_lossy(&w.data().text) != *t {
                continue;
            }
        }
        return h;
    }
    0
}

pub unsafe extern "win64" fn FindWindowW(class: LPCWSTR, title: LPCWSTR) -> HWND {
    let c = (!class.is_null()).then(|| ClassName::from_w(class));
    find_window_ex(0, 0, c, crate::strings::from_wide_opt(title)) as HWND
}

pub unsafe extern "win64" fn FindWindowA(class: LPCSTR, title: LPCSTR) -> HWND {
    let c = (!class.is_null()).then(|| ClassName::from_a(class));
    find_window_ex(0, 0, c, crate::strings::from_ansi_opt(title)) as HWND
}

pub unsafe extern "win64" fn FindWindowExW(parent: HWND, child_after: HWND, class: LPCWSTR, title: LPCWSTR) -> HWND {
    let c = (!class.is_null()).then(|| ClassName::from_w(class));
    find_window_ex(parent as usize, child_after as usize, c, crate::strings::from_wide_opt(title)) as HWND
}

pub unsafe extern "win64" fn FindWindowExA(parent: HWND, child_after: HWND, class: LPCSTR, title: LPCSTR) -> HWND {
    let c = (!class.is_null()).then(|| ClassName::from_a(class));
    find_window_ex(parent as usize, child_after as usize, c, crate::strings::from_ansi_opt(title)) as HWND
}

// ---------------------------------------------------------------------------
// Window longs
// ---------------------------------------------------------------------------

fn get_long(hwnd: usize, idx: i32, size: usize) -> isize {
    let Some(w) = require(hwnd) else { return 0 };
    let d = w.data();
    match idx {
        GWLP_WNDPROC => d.wndproc as isize,
        GWLP_HINSTANCE => d.instance as isize,
        GWLP_HWNDPARENT => (if d.style & ws::CHILD != 0 { d.parent } else { d.owner }) as isize,
        GWLP_ID => d.id as isize,
        GWL_STYLE => d.style as i32 as isize,
        GWL_EXSTYLE => d.exstyle as i32 as isize,
        GWLP_USERDATA => d.userdata as isize,
        i if i >= 0 && (i as usize) + size <= d.extra.len() => {
            let mut b = [0u8; 8];
            b[..size].copy_from_slice(&d.extra[i as usize..i as usize + size]);
            if size == 4 {
                i32::from_le_bytes(b[..4].try_into().unwrap()) as isize
            } else {
                i64::from_le_bytes(b) as isize
            }
        }
        _ => {
            drop(d);
            set_error(e::INVALID_INDEX);
            0
        }
    }
}

#[repr(C)]
struct STYLESTRUCT {
    old: u32,
    new: u32,
}

fn set_long(hwnd: usize, idx: i32, val: isize, size: usize) -> isize {
    let Some(w) = require(hwnd) else { return 0 };
    match idx {
        GWL_STYLE | GWL_EXSTYLE => {
            let old = {
                let d = w.data();
                if idx == GWL_STYLE {
                    d.style
                } else {
                    d.exstyle
                }
            };
            let mut ss = STYLESTRUCT { old, new: val as u32 };
            let which = if idx == GWL_STYLE { -16isize } else { -20isize } as usize;
            send_message(hwnd, 0x007C, which, &mut ss as *mut STYLESTRUCT as LPARAM); // WM_STYLECHANGING
            {
                let mut d = w.data();
                if idx == GWL_STYLE {
                    let (s, _) = framed_top_level_style(ss.new, d.exstyle);
                    ss.new = s;
                    // WS_VISIBLE changes the visibility state directly (no messages).
                    d.visible = ss.new & ws::VISIBLE != 0;
                    d.style = ss.new;
                } else {
                    let (_, ex) = framed_top_level_style(d.style, ss.new);
                    ss.new = ex;
                    d.exstyle = ss.new;
                    d.topmost = ss.new & ws::EX_TOPMOST != 0;
                }
            }
            log::debug!("SetWindowLong {hwnd:#x} {} {old:#x} -> {:#x}", if idx == GWL_STYLE { "GWL_STYLE" } else { "GWL_EXSTYLE" }, ss.new);
            send_message(hwnd, 0x007D, which, &mut ss as *mut STYLESTRUCT as LPARAM); // WM_STYLECHANGED
            old as i32 as isize
        }
        _ => {
            let mut d = w.data();
            let old;
            match idx {
                GWLP_WNDPROC => {
                    old = d.wndproc as isize;
                    d.wndproc = val as usize;
                }
                GWLP_HINSTANCE => {
                    old = d.instance as isize;
                    d.instance = val as usize;
                }
                GWLP_HWNDPARENT => {
                    old = d.owner as isize;
                    d.owner = val as usize;
                }
                GWLP_ID => {
                    old = d.id as isize;
                    d.id = val as usize;
                }
                GWLP_USERDATA => {
                    old = d.userdata as isize;
                    d.userdata = val as usize;
                }
                i if i >= 0 && (i as usize) + size <= d.extra.len() => {
                    let i = i as usize;
                    let mut b = [0u8; 8];
                    b[..size].copy_from_slice(&d.extra[i..i + size]);
                    old = if size == 4 { i32::from_le_bytes(b[..4].try_into().unwrap()) as isize } else { i64::from_le_bytes(b) as isize };
                    d.extra[i..i + size].copy_from_slice(&(val as i64).to_le_bytes()[..size]);
                }
                _ => {
                    drop(d);
                    set_error(e::INVALID_INDEX);
                    return 0;
                }
            }
            drop(d);
            set_error(0);
            old
        }
    }
}

pub unsafe extern "win64" fn GetWindowLongPtrW(hwnd: HWND, idx: i32) -> isize {
    get_long(hwnd as usize, idx, 8)
}
pub unsafe extern "win64" fn GetWindowLongPtrA(hwnd: HWND, idx: i32) -> isize {
    get_long(hwnd as usize, idx, 8)
}
pub unsafe extern "win64" fn GetWindowLongW(hwnd: HWND, idx: i32) -> i32 {
    get_long(hwnd as usize, idx, 4) as i32
}
pub unsafe extern "win64" fn GetWindowLongA(hwnd: HWND, idx: i32) -> i32 {
    get_long(hwnd as usize, idx, 4) as i32
}
pub unsafe extern "win64" fn SetWindowLongPtrW(hwnd: HWND, idx: i32, v: isize) -> isize {
    set_long(hwnd as usize, idx, v, 8)
}
pub unsafe extern "win64" fn SetWindowLongPtrA(hwnd: HWND, idx: i32, v: isize) -> isize {
    set_long(hwnd as usize, idx, v, 8)
}
pub unsafe extern "win64" fn SetWindowLongW(hwnd: HWND, idx: i32, v: i32) -> i32 {
    set_long(hwnd as usize, idx, v as isize, 4) as i32
}
pub unsafe extern "win64" fn SetWindowLongA(hwnd: HWND, idx: i32, v: i32) -> i32 {
    set_long(hwnd as usize, idx, v as isize, 4) as i32
}

const GCLP_HCURSOR: i32 = -12;
const GCLP_HICON: i32 = -14;
const GCLP_WNDPROC: i32 = -24;
const GCL_STYLE: i32 = -26;
const GCW_ATOM: i32 = -32;

pub unsafe extern "win64" fn GetClassLongPtrW(hwnd: HWND, idx: i32) -> usize {
    let Some(w) = require(hwnd as usize) else { return 0 };
    match idx {
        GCLP_HCURSOR => *w.class.cursor.lock().unwrap(),
        GCLP_HICON => w.class.icon,
        GCLP_WNDPROC => w.class.wndproc,
        GCL_STYLE => w.class.style as usize,
        GCW_ATOM => w.class.atom as usize,
        _ => {
            stub!("GetClassLongPtrW(index {idx}) -> 0");
            0
        }
    }
}

pub unsafe extern "win64" fn SetClassLongPtrW(hwnd: HWND, idx: i32, v: usize) -> usize {
    let Some(w) = require(hwnd as usize) else { return 0 };
    match idx {
        GCLP_HCURSOR => std::mem::replace(&mut *w.class.cursor.lock().unwrap(), v),
        _ => {
            stub!("SetClassLongPtrW(index {idx}) ignored");
            0
        }
    }
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

pub fn internal_text(hwnd: usize) -> Option<Vec<u16>> {
    get(hwnd).map(|w| w.data().text.clone())
}

pub fn set_internal_text(hwnd: usize, text: Vec<u16>) {
    if let Some(w) = get(hwnd) {
        let native = {
            let mut d = w.data();
            d.text = text.clone();
            d.native
        };
        if native {
            crate::macos::window_set_title(hwnd, String::from_utf16_lossy(&text));
        }
    }
}

pub unsafe extern "win64" fn SetWindowTextW(hwnd: HWND, text: LPCWSTR) -> BOOL {
    if require(hwnd as usize).is_none() {
        return FALSE;
    }
    let empty = [0u16];
    let p = if text.is_null() { empty.as_ptr() } else { text };
    send_message(hwnd as usize, wm::SETTEXT, 0, p as LPARAM) as BOOL
}

pub unsafe extern "win64" fn SetWindowTextA(hwnd: HWND, text: LPCSTR) -> BOOL {
    let w = crate::strings::to_wide_z(&crate::strings::from_ansi_opt(text).unwrap_or_default());
    SetWindowTextW(hwnd, w.as_ptr())
}

pub unsafe extern "win64" fn GetWindowTextW(hwnd: HWND, buf: LPWSTR, cap: i32) -> i32 {
    let Some(w) = require(hwnd as usize) else { return 0 };
    if buf.is_null() || cap <= 0 {
        return 0;
    }
    if w.tid == crate::teb::thread_id() {
        return send_message(hwnd as usize, wm::GETTEXT, cap as usize, buf as LPARAM) as i32;
    }
    let t = w.data().text.clone();
    let n = t.len().min(cap as usize - 1);
    std::ptr::copy_nonoverlapping(t.as_ptr(), buf, n);
    *buf.add(n) = 0;
    n as i32
}

pub unsafe extern "win64" fn GetWindowTextA(hwnd: HWND, buf: LPSTR, cap: i32) -> i32 {
    if buf.is_null() || cap <= 0 {
        return 0;
    }
    let Some(t) = internal_text(hwnd as usize) else {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    };
    crate::strings::write_ansi_trunc(&String::from_utf16_lossy(&t), buf, cap as usize).1 as i32
}

pub unsafe extern "win64" fn GetWindowTextLengthW(hwnd: HWND) -> i32 {
    let Some(w) = require(hwnd as usize) else { return 0 };
    if w.tid == crate::teb::thread_id() {
        return send_message(hwnd as usize, wm::GETTEXTLENGTH, 0, 0) as i32;
    }
    let n = w.data().text.len() as i32;
    n
}

// ---------------------------------------------------------------------------
// Painting
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: usize,
    pub erase: BOOL,
    pub rc_paint: RECT,
    pub restore: BOOL,
    pub inc_update: BOOL,
    pub reserved: [u8; 32],
}
const _: () = assert!(std::mem::size_of::<PAINTSTRUCT>() == 72);

pub unsafe extern "win64" fn InvalidateRect(hwnd: HWND, _r: *const RECT, erase: BOOL) -> BOOL {
    if hwnd.is_null() {
        // Whole desktop: invalidate all our windows.
        for h in top_level_windows() {
            if let Some(w) = get(h) {
                invalidate(&w);
            }
        }
        return TRUE;
    }
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if w.data().visible {
        invalidate(&w);
        if erase == 0 {
            w.data().erase_pending = false;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn ValidateRect(hwnd: HWND, _r: *const RECT) -> BOOL {
    let Some(w) = require(hwnd as usize) else { return FALSE };
    if let Some(q) = message::queue_of(w.tid) {
        q.validate(w.hwnd);
    }
    TRUE
}

pub unsafe extern "win64" fn RedrawWindow(hwnd: HWND, _r: *const RECT, _rgn: usize, flags: u32) -> BOOL {
    const RDW_INVALIDATE: u32 = 1;
    const RDW_VALIDATE: u32 = 8;
    const RDW_UPDATENOW: u32 = 0x100;
    if flags & RDW_INVALIDATE != 0 {
        InvalidateRect(hwnd, std::ptr::null(), TRUE);
    }
    if flags & RDW_VALIDATE != 0 {
        ValidateRect(hwnd, std::ptr::null());
    }
    if flags & RDW_UPDATENOW != 0 && !hwnd.is_null() {
        UpdateWindow(hwnd);
    }
    TRUE
}

pub unsafe extern "win64" fn GetUpdateRect(hwnd: HWND, r: *mut RECT, _erase: BOOL) -> BOOL {
    let Some((w, h)) = client_size(hwnd as usize) else { return FALSE };
    if !r.is_null() {
        *r = RECT::new(0, 0, w, h);
    }
    TRUE
}

pub unsafe extern "win64" fn BeginPaint(hwnd: HWND, ps: *mut PAINTSTRUCT) -> usize {
    let Some(w) = require(hwnd as usize) else { return 0 };
    if let Some(q) = message::queue_of(w.tid) {
        q.validate(w.hwnd);
    }
    let erase = std::mem::replace(&mut w.data().erase_pending, false);
    let hdc = crate::misc::gdi32::window_dc(hwnd as usize);
    let erased = if erase { send_message(hwnd as usize, wm::ERASEBKGND, hdc, 0) != 0 } else { true };
    let (cw, ch) = client_size(hwnd as usize).unwrap_or((0, 0));
    if !ps.is_null() {
        std::ptr::write_bytes(ps, 0, 1);
        (*ps).hdc = hdc;
        (*ps).erase = (!erased) as BOOL;
        (*ps).rc_paint = RECT::new(0, 0, cw, ch);
    }
    hdc
}

pub unsafe extern "win64" fn EndPaint(_hwnd: HWND, _ps: *const PAINTSTRUCT) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetDC(hwnd: HWND) -> usize {
    crate::misc::gdi32::window_dc(hwnd as usize)
}

pub unsafe extern "win64" fn GetWindowDC(hwnd: HWND) -> usize {
    crate::misc::gdi32::window_dc(hwnd as usize)
}

pub unsafe extern "win64" fn GetDCEx(hwnd: HWND, _clip: usize, _flags: u32) -> usize {
    crate::misc::gdi32::window_dc(hwnd as usize)
}

pub unsafe extern "win64" fn ReleaseDC(_hwnd: HWND, _hdc: usize) -> i32 {
    1
}

// ---------------------------------------------------------------------------
// DPI
// ---------------------------------------------------------------------------

/// Reported system (primary monitor) DPI (`scale * 96`, e.g. 192 on 2x Retina,
/// 96 on 1x external / headless). `PORT_DPI=<n>` / `AW2_DPI=<n>` overrides.
pub fn dpi() -> u32 {
    super::monitor::primary_dpi()
}

pub unsafe extern "win64" fn GetDpiForWindow(hwnd: HWND) -> u32 {
    let h = hwnd as usize;
    if h == DESKTOP_HWND {
        return dpi();
    }
    if !exists(h) {
        set_error(e::INVALID_WINDOW_HANDLE);
        return 0;
    }
    super::monitor::dpi_for_window(h)
}

pub unsafe extern "win64" fn GetDpiForSystem() -> u32 {
    dpi()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    static LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static SIZES: AtomicU32 = AtomicU32::new(0);

    unsafe extern "win64" fn test_proc(h: HWND, m: u32, w: WPARAM, l: LPARAM) -> LRESULT {
        LOG.lock().unwrap().push(m);
        if m == wm::SIZE {
            SIZES.store(l as u32, Ordering::SeqCst);
        }
        super::super::defproc::DefWindowProcW(h, m, w, l)
    }

    fn test_proc_handle() -> usize {
        super::super::register_typed_host_wndproc(test_proc)
    }

    fn register(name: &str) -> Vec<u16> {
        let n = crate::strings::to_wide_z(name);
        let wc = class::WNDCLASS {
            style: 0,
            wndproc: test_proc_handle(),
            cls_extra: 0,
            wnd_extra: 16,
            instance: 0,
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: 0,
            class_name: n.as_ptr() as usize,
        };
        unsafe { assert_ne!(class::RegisterClassW(&wc), 0) };
        n
    }

    /// The message log and activation state are global: run the window
    /// tests one at a time (shared with other user32-state tests).
    use super::super::TEST_SERIAL as SERIAL;

    #[test]
    fn create_show_destroy_sequence() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| unsafe {
            let cls = register("Aw2TestWinSeq");
            let title = crate::strings::to_wide_z("hello");
            LOG.lock().unwrap().clear();
            let style = ws::CAPTION | ws::SYSMENU | ws::THICKFRAME;
            let h = CreateWindowExW(0, cls.as_ptr(), title.as_ptr(), style, 10, 20, 640, 480, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!h.is_null());
            let log = LOG.lock().unwrap().clone();
            assert_eq!(&log[..], &[wm::GETMINMAXINFO, wm::NCCREATE, wm::NCCALCSIZE, wm::CREATE, wm::SIZE, wm::MOVE]);
            // Title set by DefWindowProc(WM_NCCREATE).
            let mut buf = [0u16; 16];
            assert_eq!(GetWindowTextW(h, buf.as_mut_ptr(), 16), 5);
            // Client = window minus title bar.
            let mut r = RECT::default();
            GetClientRect(h, &mut r);
            let (_, t, _, _) = nc_insets(style, 0);
            assert_eq!((r.right, r.bottom), (640, 480 - t));
            let mut wr = RECT::default();
            GetWindowRect(h, &mut wr);
            assert_eq!(wr, RECT::new(10, 20, 650, 500));
            let mut p = POINT { x: 0, y: 0 };
            ClientToScreen(h, &mut p);
            assert_eq!((p.x, p.y), (10, 20 + t));
            // Window longs.
            SetWindowLongPtrW(h, GWLP_USERDATA, 0x1234);
            assert_eq!(GetWindowLongPtrW(h, GWLP_USERDATA), 0x1234);
            SetWindowLongPtrW(h, 8, 0x55);
            assert_eq!(GetWindowLongPtrW(h, 8), 0x55);
            assert_eq!(GetWindowLongW(h, GWL_STYLE) as u32 & ws::CAPTION, ws::CAPTION);
            // Show: activation + focus.
            LOG.lock().unwrap().clear();
            assert_eq!(ShowWindow(h, SW_SHOW), FALSE);
            let log = LOG.lock().unwrap().clone();
            assert_eq!(log[0], wm::SHOWWINDOW);
            assert_eq!(log[1], wm::WINDOWPOSCHANGING);
            assert!(log.contains(&wm::ACTIVATE));
            assert!(log.contains(&wm::SETFOCUS));
            assert_eq!(super::super::input::GetFocus() as usize, h as usize);
            assert_eq!(IsWindowVisible(h), TRUE);
            // Paint is pending after show and cleared by BeginPaint (DefWindowProc).
            let mut m = MSG::default();
            assert_eq!(message::PeekMessageW(&mut m, h, wm::PAINT, wm::PAINT, 0), TRUE);
            message::DispatchMessageW(&m);
            assert_eq!(message::PeekMessageW(&mut m, h, wm::PAINT, wm::PAINT, 0), FALSE);
            // Resize -> WM_SIZE through DefWindowProc(WM_WINDOWPOSCHANGED).
            SetWindowPos(h, std::ptr::null_mut(), 0, 0, 800, 600, SWP_NOMOVE | SWP_NOZORDER);
            assert_eq!(SIZES.load(Ordering::SeqCst), make_long(800, 600 - t) as u32);
            // Close -> DestroyWindow via DefWindowProc.
            LOG.lock().unwrap().clear();
            message::SendMessageW(h, wm::CLOSE, 0, 0);
            assert_eq!(IsWindow(h), FALSE);
            let log = LOG.lock().unwrap().clone();
            assert!(log.contains(&wm::DESTROY) && log.ends_with(&[wm::NCDESTROY]));
            assert_eq!(class::UnregisterClassW(cls.as_ptr(), std::ptr::null_mut()), TRUE);
        })
        .join()
        .unwrap();
    }

    static OLD_PROC: AtomicUsize = AtomicUsize::new(0);
    static SUB_HITS: AtomicU32 = AtomicU32::new(0);

    unsafe extern "win64" fn sub_proc(h: HWND, m: u32, w: WPARAM, l: LPARAM) -> LRESULT {
        if m == 0x0400 {
            SUB_HITS.fetch_add(1, Ordering::SeqCst);
        }
        message::CallWindowProcW(OLD_PROC.load(Ordering::SeqCst), h, m, w, l)
    }

    /// Close a window owned by a thread running a GetMessage loop and wait
    /// for that thread to exit. WM_QUIT is posted after WM_CLOSE, and posted
    /// messages are retrieved first, so the owner has dispatched WM_CLOSE (and
    /// DefWindowProc has destroyed the window) by the time it sees WM_QUIT.
    /// No timing assumptions: `tid` must be captured while the window exists.
    unsafe fn close_and_join(h: usize, tid: u32, owner: std::thread::JoinHandle<()>) {
        message::PostMessageW(h as HWND, wm::CLOSE, 0, 0);
        message::PostThreadMessageW(tid, wm::QUIT, 0, 0);
        owner.join().unwrap();
        assert!(!exists(h), "WM_CLOSE did not destroy {h:#x}");
    }

    /// Owner thread: create a window, report (hwnd, owner tid), pump messages
    /// until WM_QUIT.
    fn spawn_owner(class: &'static str, style: u32, foreground: bool) -> (usize, u32, std::thread::JoinHandle<()>) {
        let (tx, rx) = std::sync::mpsc::channel();
        let owner = std::thread::spawn(move || unsafe {
            let cls = register(class);
            let h = CreateWindowExW(0, cls.as_ptr(), std::ptr::null(), style, 0, 0, 10, 10, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!h.is_null());
            if foreground {
                super::super::input::SetForegroundWindow(h);
            }
            tx.send((h as usize, get(h as usize).unwrap().tid)).unwrap();
            let mut m = MSG::default();
            while message::GetMessageW(&mut m, std::ptr::null_mut(), 0, 0) > 0 {
                message::DispatchMessageW(&m);
            }
        });
        let (h, tid) = rx.recv().unwrap();
        (h, tid, owner)
    }

    /// Wwise (0x14152d2b0) subclasses the game's window from its own thread:
    /// GetForegroundWindow, Get/SetWindowLongPtrW(GWLP_WNDPROC), CallWindowProcW.
    #[test]
    fn subclass_from_other_thread() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (hu, tid, owner) = spawn_owner("Aw2TestWinSub", ws::POPUP | ws::VISIBLE, true);
        let h = hu as HWND;
        unsafe {
            assert_eq!(super::super::input::GetForegroundWindow() as usize, hu);
            let old = GetWindowLongPtrW(h, GWLP_WNDPROC);
            assert_eq!(old as usize, test_proc_handle());
            OLD_PROC.store(old as usize, Ordering::SeqCst);
            let sub_handle = super::super::register_typed_host_wndproc(sub_proc);
            assert_eq!(SetWindowLongPtrW(h, GWLP_WNDPROC, sub_handle as isize), old);
            assert_eq!(GetWindowLongPtrW(h, GWLP_WNDPROC) as usize, sub_handle);
            LOG.lock().unwrap().clear();
            // Cross-thread SendMessage returns only after the owner processed it.
            message::SendMessageW(h, 0x0400, 0, 0);
            assert_eq!(SUB_HITS.load(Ordering::SeqCst), 1);
            // Chained to the original procedure.
            assert!(LOG.lock().unwrap().contains(&0x0400));
            close_and_join(hu, tid, owner);
        }
    }

    #[test]
    fn cross_thread_send_message() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (h, tid, owner) = spawn_owner("Aw2TestWinXT", ws::POPUP, false);
        let title = crate::strings::to_wide_z("xt");
        unsafe {
            assert_eq!(message::SendMessageW(h as HWND, wm::SETTEXT, 0, title.as_ptr() as LPARAM), 1);
        }
        assert_eq!(String::from_utf16_lossy(&internal_text(h).unwrap()), "xt");
        // Window is destroyed by DefWindowProc(WM_CLOSE) on the owner thread.
        unsafe { close_and_join(h, tid, owner) };
    }

    #[test]
    fn per_monitor_dpi_for_window_and_adjust_rect() {
        let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = crate::macos::display::build_displays(
            &[
                crate::macos::display::ScreenDesc {
                    id: 951,
                    frame_pt: (0.0, 0.0, 1512.0, 982.0),
                    visible_pt: (0.0, 0.0, 1512.0, 950.0),
                    backing_scale: 2.0,
                    refresh_hz: 120,
                    name: "Retina".into(),
                },
                crate::macos::display::ScreenDesc {
                    id: 952,
                    frame_pt: (1512.0, -98.0, 1920.0, 1080.0),
                    visible_pt: (1512.0, -98.0, 1920.0, 1055.0),
                    backing_scale: 1.0,
                    refresh_hz: 60,
                    name: "External".into(),
                },
            ],
            28.0,
            true,
        );
        crate::macos::display::with_test_displays(d, || {
            std::thread::spawn(|| unsafe {
                let cls = register("Aw2TestWinMixedDpi");
                let style = ws::CAPTION | ws::SYSMENU | ws::THICKFRAME;
                // Create on primary Retina monitor (0..3024, 192 DPI).
                let h = CreateWindowExW(
                    0,
                    cls.as_ptr(),
                    std::ptr::null(),
                    style,
                    100,
                    100,
                    800,
                    600,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                assert!(!h.is_null());
                assert_eq!(GetDpiForSystem(), 192);
                assert_eq!(GetDpiForWindow(h), 192);

                // Client inset on 2x Retina monitor should be 28*2 = 56 px.
                let mut rc_ret = RECT::default();
                GetClientRect(h, &mut rc_ret);
                assert_eq!(rc_ret.height(), 600 - 56);

                // Move window onto 1x external monitor (3024..4944, 96 DPI).
                SetWindowPos(h, std::ptr::null_mut(), 3200, 100, 800, 600, SWP_NOZORDER | SWP_NOACTIVATE);
                assert_eq!(GetDpiForWindow(h), 96);
                let mut rc_ext = RECT::default();
                GetClientRect(h, &mut rc_ext);
                assert_eq!(rc_ext.height(), 600 - 28);

                // AdjustWindowRectExForDpi respects passed DPI.
                let mut r96 = RECT::new(0, 0, 800, 600);
                let mut r192 = RECT::new(0, 0, 800, 600);
                assert_eq!(AdjustWindowRectExForDpi(&mut r96, style, FALSE, 0, 96), TRUE);
                assert_eq!(AdjustWindowRectExForDpi(&mut r192, style, FALSE, 0, 192), TRUE);
                assert_eq!(r96.height(), 600 + 28);
                assert_eq!(r192.height(), 600 + 56);

                DestroyWindow(h);
                class::UnregisterClassW(cls.as_ptr(), std::ptr::null_mut());
            })
            .join()
            .unwrap();
        });
    }

    #[test]
    fn single_surface_mode_locks_windowed_borderless_and_fullscreen_identically() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = crate::macos::display::build_displays(
            &[crate::macos::display::ScreenDesc {
                id: 960,
                frame_pt: (0.0, 0.0, 956.0, 440.0),
                visible_pt: (0.0, 0.0, 956.0, 440.0),
                backing_scale: 1600.0 / 956.0,
                refresh_hz: 120,
                name: "iOS Locked".into(),
            }],
            0.0,
            true,
        );
        crate::macos::display::with_test_displays(d, || {
            crate::macos::with_single_surface_mode(true, || {
                std::thread::spawn(|| unsafe {
                    let cls = register("Aw2TestSingleSurface");
                    // Windowed creation with WS_OVERLAPPEDWINDOW + smaller size (1280x720)
                    let windowed_style = ws::CAPTION | ws::SYSMENU | ws::THICKFRAME | ws::MINIMIZEBOX | ws::MAXIMIZEBOX;
                    let h = CreateWindowExW(
                        0,
                        cls.as_ptr(),
                        std::ptr::null(),
                        windowed_style,
                        100,
                        50,
                        1280,
                        720,
                        std::ptr::null_mut(),
                        0,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );
                    assert!(!h.is_null());

                    let mut wr = RECT::default();
                    let mut cr = RECT::default();
                    GetWindowRect(h, &mut wr);
                    GetClientRect(h, &mut cr);
                    assert_eq!(wr, RECT::new(0, 0, 1600, 736));
                    assert_eq!(cr, RECT::new(0, 0, 1600, 736));

                    // AdjustWindowRectEx adds zero non-client insets in single-surface mode
                    let mut adj = RECT::new(0, 0, 1600, 736);
                    assert_eq!(AdjustWindowRectEx(&mut adj, windowed_style, FALSE, 0), TRUE);
                    assert_eq!(adj, RECT::new(0, 0, 1600, 736));

                    // Switching to Borderless (WS_POPUP) + SetWindowPos to an arbitrary rect stays locked
                    SetWindowLongW(h, GWL_STYLE, (ws::POPUP | ws::VISIBLE) as i32);
                    SetWindowPos(h, std::ptr::null_mut(), 200, 120, 800, 600, SWP_FRAMECHANGED | SWP_NOZORDER);
                    GetWindowRect(h, &mut wr);
                    GetClientRect(h, &mut cr);
                    assert_eq!(wr, RECT::new(0, 0, 1600, 736));
                    assert_eq!(cr, RECT::new(0, 0, 1600, 736));

                    // Switching back to Windowed + MoveWindow stays locked to the exact same rect
                    SetWindowLongW(h, GWL_STYLE, (windowed_style | ws::VISIBLE) as i32);
                    MoveWindow(h, 50, 50, 1024, 576, TRUE);
                    GetWindowRect(h, &mut wr);
                    GetClientRect(h, &mut cr);
                    assert_eq!(wr, RECT::new(0, 0, 1600, 736));
                    assert_eq!(cr, RECT::new(0, 0, 1600, 736));

                    DestroyWindow(h);
                    class::UnregisterClassW(cls.as_ptr(), std::ptr::null_mut());
                })
                .join()
                .unwrap();
            });
        });
    }
}
