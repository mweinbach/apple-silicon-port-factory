//! macOS window-system backend for `user32` (AppKit, on the main thread).
//!
//! # Threading model
//!
//! AppKit must run on the process main thread, but all Windows code runs on
//! loader-created threads (the game's WinMain thread, its message-pump
//! thread, ...). So:
//!
//! * The loader calls [`init_main_thread`] on the main thread before it
//!   starts the initial Windows thread, then hands the main thread to
//!   [`run_main_loop`], which pumps `NSEvent`s (and, through the run loop,
//!   the libdispatch main queue) until the Windows process exits.
//! * Windows threads never touch AppKit, CoreGraphics or CoreFoundation
//!   directly (Apple frameworks use pthread TSD slots that alias our TEB
//!   fields, see `teb.rs`). Every native operation is a closure marshalled
//!   onto the main queue with [`run_sync`] (`dispatch_sync_f`, used only
//!   where a result is needed: window creation) or [`run_async`]
//!   (`dispatch_async_f`; FIFO, so operations stay ordered).
//! * The main thread **never waits on a Windows thread**. Native events are
//!   translated there and *queued* to the owning Win32 thread
//!   (`user32::input::native_*` posts input messages,
//!   `user32::window::native_event` queues internal calls). Hence a
//!   `run_sync` from a Windows thread cannot deadlock, provided the caller
//!   holds no user32 lock across it (rule for callers).
//! * Code on the main thread has no TEB: it must not call `crate::teb::*`
//!   (directly or via `set_last_error`, `wait_for`, exported Win32
//!   functions, ...). `user32::set_error` is a no-op there.
//! * Display information (monitors, scale, title-bar height) is cached by
//!   the main thread ([`display`]) and only read by Windows threads.
//!
//! Without a main loop (unit tests, `--no-run`, `AW2_HEADLESS=1`) the
//! backend is inactive: native operations are no-ops, windows exist only
//! in the Rust window table, and [`display`] reports a 1920x1080 monitor.
//!
//! # Per-window objects
//!
//! Each top-level HWND gets an `NSWindow` whose content view (`AW2View`)
//! is backed by a `CAMetalLayer`. [`metal_layer_for_hwnd`] /
//! [`window_shared`] expose the layer and live client size to other
//! modules (the future dxgi swap chain) from any thread.

pub mod display;
pub mod gamepad;
pub mod hidmgr;
pub mod keyboard;
pub mod uikit;

#[cfg(target_os = "macos")]
mod appkit;
#[cfg(target_os = "macos")]
mod events;

use crate::user32::cursor::Shape;
use crate::user32::{POINT, RECT};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

pub use display::titlebar_height_pt;
pub use uikit::{
    attach_host_surface, detach_host_surface, dispatch_host_rumble, has_rumble_callback,
    key_event, keyboard_state, mouse_button_event, resize_host_surface, scroll_wheel_event,
    set_active_swapchain_size, set_keyboard_state, set_rumble_callback, text_input, touch_event,
    virtual_gamepad_state, KeyboardState, RumbleCallback, MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT,
    TOUCH_BEGAN, TOUCH_CANCELLED, TOUCH_ENDED, TOUCH_MOVED,
};

// ---------------------------------------------------------------------------
// Main-thread marshalling
// ---------------------------------------------------------------------------

#[repr(C)]
struct DispatchQueue {
    _p: [u8; 0],
}

extern "C" {
    static _dispatch_main_q: DispatchQueue;
    fn dispatch_sync_f(q: *const DispatchQueue, ctx: *mut c_void, f: extern "C" fn(*mut c_void));
    fn dispatch_async_f(q: *const DispatchQueue, ctx: *mut c_void, f: extern "C" fn(*mut c_void));
    fn pthread_main_np() -> i32;
}

static ACTIVE: AtomicBool = AtomicBool::new(false);
static MOUSE_IN_CLIENT: AtomicBool = AtomicBool::new(true);

/// True once [`init_main_thread`] (or [`attach_host_surface`] on iOS) ran:
/// the platform main thread is active and services the main queue.
pub fn active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

#[cfg(target_os = "ios")]
pub(super) fn mark_active(v: bool) {
    ACTIVE.store(v, Ordering::Release);
}

#[cfg(test)]
static TEST_SINGLE_SURFACE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Run `f` with [`single_surface_mode`] forced on (`Some(true)`), off (`Some(false)`),
/// or restored to platform default (`None`). Used by unit tests on macOS.
#[cfg(test)]
pub(crate) fn with_single_surface_mode<R>(forced: bool, f: impl FnOnce() -> R) -> R {
    let prev = TEST_SINGLE_SURFACE.swap(if forced { 2 } else { 1 }, Ordering::AcqRel);
    struct Restore(u8);
    impl Drop for Restore {
        fn drop(&mut self) {
            TEST_SINGLE_SURFACE.store(self.0, Ordering::Release);
        }
    }
    let _r = Restore(prev);
    f()
}

/// True when the host owns a single locked display surface (always `true` on iOS,
/// or on macOS when `PORT_LOCK_DISPLAY_MODE` is enabled).
///
/// In single-surface mode, Windowed, Borderless, and Fullscreen all map identically
/// onto the single fullscreen `CAMetalLayer` with zero non-client titlebar insets,
/// identical window/client geometry, `S_OK` from `IDXGISwapChain::SetFullscreenState`,
/// and a single consistent display resolution in `IDXGIOutput::GetDisplayModeList`.
pub fn single_surface_mode() -> bool {
    #[cfg(test)]
    {
        match TEST_SINGLE_SURFACE.load(Ordering::Acquire) {
            1 => return false,
            2 => return true,
            _ => {}
        }
    }
    static LOCKED: OnceLock<bool> = OnceLock::new();
    *LOCKED.get_or_init(|| {
        if let Ok(v) = std::env::var("PORT_LOCK_DISPLAY_MODE") {
            let s = v.trim();
            return !s.is_empty()
                && !matches!(s.to_ascii_lowercase().as_str(), "0" | "false" | "off" | "none");
        }
        cfg!(target_os = "ios")
    })
}

/// True when borderless fullscreen takeover is active (always `true` on iOS,
/// or when `AW2_FULLSCREEN=1` / `PORT_LOCK_DISPLAY_MODE=fullscreen|borderless|1`).
/// By default on macOS (`false`), the game runs inside a standard framed `NSWindow`.
pub fn fullscreen_takeover() -> bool {
    if cfg!(target_os = "ios") {
        return true;
    }
    static FS: OnceLock<bool> = OnceLock::new();
    *FS.get_or_init(|| {
        if let Ok(v) = std::env::var("PORT_LOCK_DISPLAY_MODE") {
            let s = v.trim().to_ascii_lowercase();
            if matches!(s.as_str(), "1" | "true" | "borderless" | "fullscreen") {
                return true;
            }
            if s == "windowed" {
                return false;
            }
        }
        std::env::var("AW2_FULLSCREEN")
            .ok()
            .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    })
}

/// True when the native AppKit backend is running in standard macOS framed window mode.
/// Always `false` on iOS (UIKit has no desktop window chrome / titlebar).
pub fn framed_mode() -> bool {
    if cfg!(target_os = "ios") || single_surface_mode() && fullscreen_takeover() {
        return false;
    }
    active() && !fullscreen_takeover()
}

/// Whether physical cursor warping (`CGWarpMouseCursorPosition` on `ClipCursor`/`SetCursorPos`)
/// is permitted. Disabled in framed window mode unless `AW2_LOCK_CURSOR=1` so the user can freely
/// move the pointer to the title bar, Dock, menu bar, or other apps.
pub fn lock_cursor_allowed() -> bool {
    static LOCK: OnceLock<bool> = OnceLock::new();
    fullscreen_takeover()
        || *LOCK.get_or_init(|| {
            std::env::var("AW2_LOCK_CURSOR")
                .ok()
                .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        })
}

/// True while the pointer is inside the active window's content area (not on the title bar or desktop).
pub fn mouse_in_client() -> bool {
    MOUSE_IN_CLIENT.load(Ordering::Acquire)
}

pub fn set_mouse_in_client(inside: bool) {
    MOUSE_IN_CLIENT.store(inside, Ordering::Release);
}

/// Optional `AW2_WINDOW_SIZE=WxH` client size override in Win32 pixels.
pub fn configured_window_size() -> Option<(i32, i32)> {
    static SZ: OnceLock<Option<(i32, i32)>> = OnceLock::new();
    *SZ.get_or_init(|| {
        let s = std::env::var("AW2_WINDOW_SIZE").ok()?;
        let (w, h) = s.trim().split_once(['x', 'X'])?;
        let w = w.trim().parse::<i32>().ok()?.max(320);
        let h = h.trim().parse::<i32>().ok()?.max(200);
        Some((w, h))
    })
}

pub fn is_main_thread() -> bool {
    unsafe { pthread_main_np() == 1 }
}

/// Run `f` on the main thread and wait for its result. `None` when the
/// backend is inactive (headless).
pub fn run_sync<R: Send, F: FnOnce() -> R + Send>(f: F) -> Option<R> {
    if is_main_thread() {
        return active().then(f);
    }
    if !active() {
        return None;
    }
    struct Ctx<F, R> {
        f: Option<F>,
        r: Option<R>,
    }
    extern "C" fn tramp<F: FnOnce() -> R, R>(p: *mut c_void) {
        let c = unsafe { &mut *(p as *mut Ctx<F, R>) };
        let f = c.f.take().unwrap();
        // A panic must not unwind into libdispatch.
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| autorelease(f))) {
            Ok(r) => c.r = Some(r),
            Err(_) => log::error!("macos: panic in main-thread operation"),
        }
    }
    let mut c = Ctx { f: Some(f), r: None };
    unsafe { dispatch_sync_f(&_dispatch_main_q, &mut c as *mut _ as *mut c_void, tramp::<F, R>) };
    c.r
}

/// Queue `f` on the main thread (FIFO with [`run_sync`]). Dropped when the
/// backend is inactive.
pub fn run_async<F: FnOnce() + Send + 'static>(f: F) {
    if !active() {
        return;
    }
    extern "C" fn tramp<F: FnOnce()>(p: *mut c_void) {
        let f = unsafe { Box::from_raw(p as *mut F) };
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| autorelease(*f))).is_err() {
            log::error!("macos: panic in main-thread operation");
        }
    }
    let b = Box::into_raw(Box::new(f));
    unsafe { dispatch_async_f(&_dispatch_main_q, b as *mut c_void, tramp::<F>) };
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn autorelease<R>(f: impl FnOnce() -> R) -> R {
    objc2::rc::autoreleasepool(|_| f())
}
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn autorelease<R>(f: impl FnOnce() -> R) -> R {
    f()
}

/// Set up NSApplication (macOS) or UIKit display/controller state (iOS). Call on the
/// main thread before any Windows thread starts. Returns false (and stays headless)
/// when not on the main thread or when `PORT_HEADLESS=1` / `AW2_HEADLESS=1`.
pub fn init_main_thread() -> bool {
    if std::env::var("PORT_HEADLESS")
        .or_else(|_| std::env::var("AW2_HEADLESS"))
        .is_ok_and(|v| !v.trim().is_empty() && v.trim() != "0")
    {
        if is_main_thread() {
            keyboard::refresh();
        }
        log::info!("platform: PORT_HEADLESS/AW2_HEADLESS set; no native windows");
        return false;
    }
    if !is_main_thread() {
        log::warn!("platform: init_main_thread called off the main thread; headless");
        return false;
    }
    #[cfg(target_os = "macos")]
    {
        appkit::init();
        ACTIVE.store(true, Ordering::Release);
        true
    }
    #[cfg(target_os = "ios")]
    {
        uikit::init();
        ACTIVE.store(true, Ordering::Release);
        true
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    false
}

/// Pump AppKit events on the main thread until `exited()` returns an exit
/// code (the Windows process ended), then return it. Without
/// [`init_main_thread`] this just polls `exited` every 50 ms.
pub fn run_main_loop(exited: &dyn Fn() -> Option<u32>) -> u32 {
    #[cfg(target_os = "macos")]
    if active() && is_main_thread() {
        return appkit::run(exited);
    }
    loop {
        if let Some(c) = exited() {
            return c;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

// ---------------------------------------------------------------------------
// Window operations (called by user32 on the window's owner thread)
// ---------------------------------------------------------------------------

/// What the backend needs to know about a top-level window.
#[derive(Clone, Debug)]
pub struct NativeDesc {
    /// Client rect in Win32 screen pixels.
    pub client: RECT,
    pub style: u32,
    pub exstyle: u32,
    pub title: String,
    pub topmost: bool,
    pub dblclks: bool,
}

/// State shared between the main thread and any other thread (dxgi).
pub struct WindowShared {
    /// `CAMetalLayer*` backing the content view (retained by the view,
    /// valid until the window is destroyed).
    pub layer: usize,
    /// Client size in pixels (`w | h << 32`), updated on every resize.
    pub client_px: AtomicU64,
    /// Backing scale of the window's screen (bits of an f64).
    pub scale: AtomicU64,
    pub occluded: AtomicBool,
}

impl WindowShared {
    pub fn client_size(&self) -> (u32, u32) {
        let v = self.client_px.load(Ordering::Acquire);
        (v as u32, (v >> 32) as u32)
    }
    pub fn set_client_size(&self, w: i32, h: i32) {
        self.client_px.store((w.max(0) as u32 as u64) | ((h.max(0) as u32 as u64) << 32), Ordering::Release);
    }
}

fn shared_map() -> &'static Mutex<HashMap<usize, Arc<WindowShared>>> {
    static M: OnceLock<Mutex<HashMap<usize, Arc<WindowShared>>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Shared native state of a top-level window (any thread).
pub fn window_shared(hwnd: usize) -> Option<Arc<WindowShared>> {
    shared_map().lock().unwrap().get(&hwnd).cloned()
}

/// The `CAMetalLayer*` of a top-level window, for the swap chain.
pub fn metal_layer_for_hwnd(hwnd: usize) -> Option<*mut c_void> {
    window_shared(hwnd).map(|s| s.layer as *mut c_void).filter(|p| !p.is_null())
}

/// Create the NSWindow (macOS) or bind to the attached host surface (iOS).
/// Synchronous; false when headless.
pub fn window_create(hwnd: usize, desc: NativeDesc) -> bool {
    #[cfg(target_os = "macos")]
    {
        run_sync(move || appkit::window_create(hwnd, desc)).unwrap_or(false)
    }
    #[cfg(target_os = "ios")]
    {
        uikit::window_create(hwnd, desc)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = (hwnd, desc);
        false
    }
}

macro_rules! async_op {
    ($name:ident ( $($a:ident : $t:ty),* ) => $imp:ident) => {
        pub fn $name(hwnd: usize, $($a: $t),*) {
            #[cfg(target_os = "macos")]
            run_async(move || appkit::$imp(hwnd, $($a),*));
            #[cfg(target_os = "ios")]
            uikit::$imp(hwnd, $($a),*);
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            let _ = (hwnd, $($a),*);
        }
    };
}

async_op!(window_destroy() => window_destroy);
async_op!(window_set_frame(client: RECT) => window_set_frame);
async_op!(window_set_style(desc: NativeDesc) => window_set_style);
async_op!(window_show(show: bool, activate: bool) => window_show);
async_op!(window_minimize(minimize: bool) => window_minimize);
async_op!(window_set_title(title: String) => window_set_title);
async_op!(window_activate() => window_activate);

// ---------------------------------------------------------------------------
// Native child controls (BUTTON / EDIT / STATIC) — all FIFO dispatch_async
// ---------------------------------------------------------------------------

/// Built-in USER32 control class mapped to a native AppKit view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeControlKind {
    Button,
    Edit,
    Static,
}

/// Snapshot of a Win32 child control's initial properties for native creation.
#[derive(Clone, Debug)]
pub struct NativeControlDesc {
    pub kind: NativeControlKind,
    /// Control rect in the root top-level window's client pixel coordinates.
    pub rect: RECT,
    pub style: u32,
    pub exstyle: u32,
    pub text: String,
    pub enabled: bool,
    pub visible: bool,
}

macro_rules! async_ctrl_op {
    ($name:ident ( $($a:ident : $t:ty),* ) => $imp:ident) => {
        pub fn $name(hwnd: usize, $($a: $t),*) {
            #[cfg(target_os = "macos")]
            run_async(move || appkit::$imp(hwnd, $($a),*));
            #[cfg(not(target_os = "macos"))]
            let _ = (hwnd, $($a),*);
        }
    };
}

async_ctrl_op!(control_create(parent_hwnd: usize, desc: NativeControlDesc) => control_create);
async_ctrl_op!(control_destroy() => control_destroy);
async_ctrl_op!(control_set_frame(rect: RECT) => control_set_frame);
async_ctrl_op!(control_set_text(text: String) => control_set_text);
async_ctrl_op!(control_set_visible(visible: bool) => control_set_visible);
async_ctrl_op!(control_set_enabled(enabled: bool) => control_set_enabled);
async_ctrl_op!(control_set_readonly(readonly: bool) => control_set_readonly);
async_ctrl_op!(control_set_check(state: u32) => control_set_check);
async_ctrl_op!(control_set_style(style: u32) => control_set_style);
async_ctrl_op!(control_set_icon(payload: Option<crate::user32::cursor::IconPayload>) => control_set_icon);

pub(crate) fn register_shared(hwnd: usize, s: Arc<WindowShared>) {
    shared_map().lock().unwrap().insert(hwnd, s);
}

pub(crate) fn unregister_shared(hwnd: usize) {
    shared_map().lock().unwrap().remove(&hwnd);
}

// ---------------------------------------------------------------------------
// Cursor
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CursorState {
    /// Hide the pointer (only while our window is active).
    pub hidden: bool,
    pub shape: Shape,
    /// Detach the pointer from the mouse (mouselook); raw deltas only.
    pub relative: bool,
}

/// Apply visibility / shape / relative mode (async, main thread).
pub fn cursor_update(s: CursorState) {
    #[cfg(target_os = "macos")]
    run_async(move || appkit::cursor_update(s));
    #[cfg(not(target_os = "macos"))]
    let _ = s;
}

/// Move the pointer to a Win32 screen position (async, main thread).
pub fn cursor_warp(p: POINT) {
    #[cfg(target_os = "macos")]
    run_async(move || appkit::cursor_warp(p));
    #[cfg(not(target_os = "macos"))]
    let _ = p;
}

// ---------------------------------------------------------------------------
// Modal alert (MessageBox)
// ---------------------------------------------------------------------------

/// Show a modal alert with the given buttons; returns the index of the
/// button pressed, or `None` when headless / on iOS.
pub fn alert(caption: String, text: String, buttons: Vec<String>, critical: bool) -> Option<usize> {
    #[cfg(target_os = "macos")]
    {
        run_sync(move || appkit::alert(&caption, &text, &buttons, critical))
    }
    #[cfg(target_os = "ios")]
    {
        uikit::alert(&caption, &text, &buttons, critical)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = (caption, text, buttons, critical);
        None
    }
}

// ---------------------------------------------------------------------------
// Clipboard (NSPasteboard on macOS, UIPasteboard on iOS)
// ---------------------------------------------------------------------------

/// True when Win32 clipboard operations should synchronize with the host
/// pasteboard (`NSPasteboard` / `UIPasteboard`). Disabled when headless, in
/// unit tests (so tests never clobber the desktop clipboard), or when
/// `PORT_SYNC_CLIPBOARD=0` / `AW2_SYNC_CLIPBOARD=0`.
pub fn clipboard_sync_enabled() -> bool {
    if cfg!(test) || !active() {
        return false;
    }
    let disabled = std::env::var("PORT_SYNC_CLIPBOARD")
        .or_else(|_| std::env::var("AW2_SYNC_CLIPBOARD"))
        .ok()
        .is_some_and(|v| matches!(v.trim(), "0" | "false" | "off"));
    !disabled
}

/// Read UTF-8 text from the host pasteboard, or `None` when headless / sync
/// disabled / no string on pasteboard.
pub fn clipboard_get_text() -> Option<String> {
    if !clipboard_sync_enabled() {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        run_sync(appkit::clipboard_get_text).flatten()
    }
    #[cfg(target_os = "ios")]
    {
        run_sync(uikit::clipboard_get_text).flatten()
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    None
}

/// Write UTF-8 text to the host pasteboard, returning the new `changeCount` on
/// success.
pub fn clipboard_set_text(text: String) -> Option<isize> {
    if !clipboard_sync_enabled() {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        run_sync(move || appkit::clipboard_set_text(&text)).flatten()
    }
    #[cfg(target_os = "ios")]
    {
        run_sync(move || uikit::clipboard_set_text(&text)).flatten()
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = text;
        None
    }
}

/// Clear the host pasteboard, returning the new `changeCount`.
pub fn clipboard_clear() -> Option<isize> {
    if !clipboard_sync_enabled() {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        run_sync(appkit::clipboard_clear)
    }
    #[cfg(target_os = "ios")]
    {
        run_sync(uikit::clipboard_clear)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    None
}

/// Read the host pasteboard's `changeCount`.
pub fn clipboard_change_count() -> Option<isize> {
    if !clipboard_sync_enabled() {
        return None;
    }
    #[cfg(target_os = "macos")]
    {
        run_sync(appkit::clipboard_change_count)
    }
    #[cfg(target_os = "ios")]
    {
        run_sync(uikit::clipboard_change_count)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    None
}

/// Show a native centered macOS progress window (no-op in headless mode).
pub fn progress_window_open(title: String, detail: String, total: usize) {
    #[cfg(target_os = "macos")]
    if active() {
        let _ = run_sync(move || appkit::progress_window_open(&title, &detail, total));
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (title, detail, total);
}

/// Update the native macOS progress window (non-blocking from worker threads; no-op when headless).
pub fn progress_window_update(detail: String, done: usize, total: usize) {
    #[cfg(target_os = "macos")]
    if active() {
        run_async(move || appkit::progress_window_update(&detail, done, total));
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (detail, done, total);
}

/// Close the native macOS progress window if open (no-op in headless mode).
pub fn progress_window_close() {
    #[cfg(target_os = "macos")]
    if active() {
        let _ = run_sync(appkit::progress_window_close);
    }
}
