//! AppKit side of the backend. Everything here runs on the main thread
//! (marshalled from [`super`]); nothing here calls guest code or the TEB.

use super::display;
use super::{CursorState, NativeDesc, WindowShared};
use crate::user32::cursor::{CustomCursorImage, Shape};
use crate::user32::window::{native_event, NativeEvent};
use crate::user32::{ws, POINT, RECT};
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, sel, AnyThread, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSAlert, NSAlertStyle, NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
    NSApplicationPresentationOptions, NSApplicationTerminateReply, NSBackingStoreType, NSButton,
    NSColor, NSCursor, NSEvent, NSEventMask, NSFont, NSImage, NSMenu, NSMenuItem, NSPasteboard,
    NSPasteboardTypeString, NSProgressIndicator, NSProgressIndicatorStyle, NSScreen, NSTextField,
    NSView, NSWindow, NSWindowCollectionBehavior, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString,
};
use objc2_quartz_core::CAMetalLayer;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

#[repr(C)]
#[derive(Clone, Copy)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CGRect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[repr(C)]
struct CGImageOpaque {
    _private: [u8; 0],
}

unsafe impl objc2::RefEncode for CGImageOpaque {
    const ENCODING_REF: objc2::Encoding =
        objc2::Encoding::Pointer(&objc2::Encoding::Struct("CGImage", &[]));
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWarpMouseCursorPosition(p: CGPoint) -> i32;
    fn CGAssociateMouseAndMouseCursorPosition(connected: i32) -> i32;
    fn CGSetLocalEventsSuppressionInterval(seconds: f64) -> i32;
    fn CGWindowListCreateImage(bounds: CGRect, list_option: u32, window: u32, image_option: u32) -> *mut c_void;
    fn CGImageGetWidth(img: *mut c_void) -> usize;
    fn CGImageGetHeight(img: *mut c_void) -> usize;
    fn CGColorSpaceCreateDeviceRGB() -> *mut c_void;
    fn CGColorSpaceRelease(cs: *mut c_void);
    fn CGDataProviderCreateWithCFData(data: *const c_void) -> *mut c_void;
    fn CGDataProviderRelease(provider: *mut c_void);
    fn CGImageCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: *mut c_void,
        bitmap_info: u32,
        provider: *mut c_void,
        decode: *const f64,
        should_interpolate: bool,
        intent: i32,
    ) -> *mut c_void;
    fn CGImageRelease(img: *mut c_void);
}

#[link(name = "ImageIO", kind = "framework")]
extern "C" {
    fn CGImageDestinationCreateWithURL(url: *const c_void, ty: *const c_void, count: usize, opts: *const c_void) -> *mut c_void;
    fn CGImageDestinationAddImage(dest: *mut c_void, img: *mut c_void, props: *const c_void);
    fn CGImageDestinationFinalize(dest: *mut c_void) -> bool;
}

#[repr(C)]
struct CFRunLoopSourceContext {
    version: isize,
    info: *mut c_void,
    retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
    release: Option<extern "C" fn(*const c_void)>,
    copy_description: Option<extern "C" fn(*const c_void) -> *const c_void>,
    equal: Option<extern "C" fn(*const c_void, *const c_void) -> u8>,
    hash: Option<extern "C" fn(*const c_void) -> usize>,
    schedule: Option<extern "C" fn(*mut c_void, *mut c_void, *const c_void)>,
    cancel: Option<extern "C" fn(*mut c_void, *mut c_void, *const c_void)>,
    perform: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
struct CFRunLoopTimerContext {
    version: isize,
    info: *mut c_void,
    retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
    release: Option<extern "C" fn(*const c_void)>,
    copy_description: Option<extern "C" fn(*const c_void) -> *const c_void>,
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopCommonModes: *const c_void;
    fn CFRelease(p: *const c_void);
    fn CFRetain(p: *const c_void) -> *const c_void;
    fn CFDataCreate(alloc: *const c_void, bytes: *const u8, len: isize) -> *const c_void;
    fn CFURLCreateFromFileSystemRepresentation(alloc: *const c_void, buf: *const u8, len: isize, is_dir: bool) -> *const c_void;
    fn CFStringCreateWithCString(alloc: *const c_void, s: *const std::ffi::c_char, enc: u32) -> *const c_void;
    fn CFAbsoluteTimeGetCurrent() -> f64;
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopSourceCreate(alloc: *const c_void, order: isize, ctx: *mut CFRunLoopSourceContext) -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, src: *mut c_void, mode: *const c_void);
    fn CFRunLoopSourceSignal(src: *mut c_void);
    fn CFRunLoopWakeUp(rl: *mut c_void);
    fn CFRunLoopTimerCreate(
        alloc: *const c_void,
        fire_date: f64,
        interval: f64,
        flags: usize,
        order: isize,
        callout: extern "C" fn(*mut c_void, *mut c_void),
        ctx: *mut CFRunLoopTimerContext,
    ) -> *mut c_void;
    fn CFRunLoopAddTimer(rl: *mut c_void, timer: *mut c_void, mode: *const c_void);
    fn CFRunLoopTimerInvalidate(timer: *mut c_void);
}

/// Capture one of our windows (or, with `whole_screen`, every on-screen
/// window composited over the desktop) into a PNG. Own windows need no
/// Screen Recording permission. Debug aid behind `AW2_SCREENSHOT`.
pub(super) fn capture_png(hwnd: usize, path: &str, whole_screen: bool) -> Result<(usize, usize), String> {
    let num = with_win(hwnd, |w| w.window.windowNumber()).ok_or("no such window")?;
    unsafe {
        let img = if whole_screen {
            let ph = display::primary_height_pt();
            let pw = display::monitors()
                .iter()
                .map(|m| m.cocoa_frame.0 + m.cocoa_frame.2)
                .fold(0.0f64, f64::max)
                .max(1920.0);
            // kCGWindowListOptionOnScreenOnly
            CGWindowListCreateImage(CGRect { x: 0.0, y: 0.0, w: pw, h: ph }, 1, 0, 0)
        } else {
            // kCGWindowListOptionIncludingWindow, kCGWindowImageBoundsIgnoreFraming
            CGWindowListCreateImage(CGRect { x: f64::INFINITY, y: f64::INFINITY, w: 0.0, h: 0.0 }, 8, num as u32, 1)
        };
        if img.is_null() {
            return Err("CGWindowListCreateImage returned NULL".into());
        }
        let (w, h) = (CGImageGetWidth(img), CGImageGetHeight(img));
        let url = CFURLCreateFromFileSystemRepresentation(std::ptr::null(), path.as_ptr(), path.len() as isize, false);
        let ty = CFStringCreateWithCString(std::ptr::null(), c"public.png".as_ptr(), 0x0800_0100);
        let dest = CGImageDestinationCreateWithURL(url, ty, 1, std::ptr::null());
        let ok = !dest.is_null() && {
            CGImageDestinationAddImage(dest, img, std::ptr::null());
            CGImageDestinationFinalize(dest)
        };
        if !dest.is_null() {
            CFRelease(dest);
        }
        CFRelease(ty);
        CFRelease(url);
        CFRelease(img);
        if ok {
            Ok((w, h))
        } else {
            Err(format!("could not write {path}"))
        }
    }
}

fn mtm() -> MainThreadMarker {
    MainThreadMarker::new().expect("AppKit backend used off the main thread")
}

// ---------------------------------------------------------------------------
// Classes
// ---------------------------------------------------------------------------

pub struct HwndIvar {
    hwnd: Cell<usize>,
}

define_class!(
    /// Borderless windows must still become key (game windows are
    /// borderless popups).
    #[unsafe(super(NSWindow))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2Window"]
    pub struct AW2Window;

    impl AW2Window {
        #[unsafe(method(canBecomeKeyWindow))]
        fn can_become_key(&self) -> bool {
            true
        }
        #[unsafe(method(canBecomeMainWindow))]
        fn can_become_main(&self) -> bool {
            true
        }
    }
);

define_class!(
    /// Content view: layer-hosting view with a `CAMetalLayer`.
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2View"]
    pub struct AW2View;

    impl AW2View {
        #[unsafe(method(acceptsFirstResponder))]
        fn accepts_first_responder(&self) -> bool {
            true
        }
        #[unsafe(method(acceptsFirstMouse:))]
        fn accepts_first_mouse(&self, _e: Option<&AnyObject>) -> bool {
            true
        }
        #[unsafe(method(isOpaque))]
        fn is_opaque(&self) -> bool {
            true
        }
        // Keys are translated in the event loop and never forwarded; these
        // keep AppKit from beeping if one slips through.
        #[unsafe(method(keyDown:))]
        fn key_down(&self, _e: &AnyObject) {}
        #[unsafe(method(keyUp:))]
        fn key_up(&self, _e: &AnyObject) {}
        #[unsafe(method(flagsChanged:))]
        fn flags_changed(&self, _e: &AnyObject) {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2WindowDelegate"]
    #[ivars = HwndIvar]
    pub struct AW2WindowDelegate;

    unsafe impl NSObjectProtocol for AW2WindowDelegate {}

    unsafe impl NSWindowDelegate for AW2WindowDelegate {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            // The game decides (WM_SYSCOMMAND/SC_CLOSE -> WM_CLOSE).
            native_event(self.hwnd(), NativeEvent::CloseRequested);
            false
        }
        #[unsafe(method(windowDidResize:))]
        fn did_resize(&self, _n: &NSNotification) {
            frame_changed(self.hwnd());
        }
        #[unsafe(method(windowDidMove:))]
        fn did_move(&self, _n: &NSNotification) {
            frame_changed(self.hwnd());
        }
        #[unsafe(method(windowDidBecomeKey:))]
        fn did_become_key(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::Activated(true));
            update_presentation();
        }
        #[unsafe(method(windowDidResignKey:))]
        fn did_resign_key(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::Activated(false));
            update_presentation();
        }
        #[unsafe(method(windowDidMiniaturize:))]
        fn did_miniaturize(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::Minimized(true));
        }
        #[unsafe(method(windowDidDeminiaturize:))]
        fn did_deminiaturize(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::Minimized(false));
        }
        #[unsafe(method(windowWillStartLiveResize:))]
        fn will_start_live_resize(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::EnterSizeMove);
        }
        #[unsafe(method(windowDidEndLiveResize:))]
        fn did_end_live_resize(&self, _n: &NSNotification) {
            native_event(self.hwnd(), NativeEvent::ExitSizeMove);
        }
        #[unsafe(method(windowDidChangeBackingProperties:))]
        fn did_change_backing(&self, _n: &NSNotification) {
            let hwnd = self.hwnd();
            let changed = with_win(hwnd, |w| {
                let s = if display::retina_enabled() {
                    w.window.backingScaleFactor().max(1.0)
                } else {
                    1.0
                };
                w.layer.setContentsScale(s);
                w.shared.scale.store(s.to_bits(), Ordering::Release);
                let cr = w.window.contentRectForFrameRect(w.window.frame());
                let client = display::cocoa_to_px(cr.origin.x, cr.origin.y, cr.size.width, cr.size.height);
                w.shared.set_client_size(client.width(), client.height());
                let dpi = (s * 96.0).round().max(48.0) as u32;
                log::info!("hwnd {hwnd:#x}: backing scale now {s} ({dpi} DPI)");
                (dpi, client, w.programmatic.get() || w.window.isMiniaturized())
            });
            if let Some((dpi, client, programmatic)) = changed {
                if !programmatic {
                    native_event(hwnd, NativeEvent::DpiChanged { dpi, client });
                }
            }
        }
        #[unsafe(method(windowDidChangeOcclusionState:))]
        fn did_change_occlusion(&self, _n: &NSNotification) {
            with_win(self.hwnd(), |w| {
                // NSWindowOcclusionStateVisible = 1 << 1
                let st: usize = unsafe { msg_send![&*w.window, occlusionState] };
                w.shared.occluded.store(st & 2 == 0, Ordering::Release);
            });
        }
    }
);

impl AW2WindowDelegate {
    fn new(mtm: MainThreadMarker, hwnd: usize) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(HwndIvar { hwnd: Cell::new(hwnd) });
        unsafe { msg_send![super(this), init] }
    }
    fn hwnd(&self) -> usize {
        self.ivars().hwnd.get()
    }
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2AppDelegate"]
    pub struct AW2AppDelegate;

    unsafe impl NSObjectProtocol for AW2AppDelegate {}

    unsafe impl NSApplicationDelegate for AW2AppDelegate {
        #[unsafe(method(applicationShouldTerminate:))]
        fn should_terminate(&self, _app: &NSApplication) -> NSApplicationTerminateReply {
            // Cmd+Q / Quit menu: ask the game to close its window instead.
            let target = WINDOWS.with(|m| m.borrow().keys().min().copied());
            match target {
                Some(h) => {
                    log::info!("Quit requested: sending close to hwnd {h:#x}");
                    native_event(h, NativeEvent::CloseRequested);
                }
                None => {
                    log::info!("Quit requested with no window: exiting");
                    crate::shadow::note_exit("AppKit quit (no window)", 0);
                    unsafe { libc::_exit(0) };
                }
            }
            NSApplicationTerminateReply::TerminateCancel
        }
        #[unsafe(method(applicationDidChangeScreenParameters:))]
        fn screens_changed(&self, _n: &NSNotification) {
            display::refresh(mtm());
            update_presentation();
        }
    }
);

impl AW2AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

// ---------------------------------------------------------------------------
// Per-window state (main thread only)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct SavedWindowed {
    frame: NSRect,
    style_mask: NSWindowStyleMask,
    level: isize,
    collection_behavior: NSWindowCollectionBehavior,
}

struct NativeWin {
    window: Retained<AW2Window>,
    view: Retained<AW2View>,
    _delegate: Retained<AW2WindowDelegate>,
    layer: Retained<CAMetalLayer>,
    shared: Arc<WindowShared>,
    /// Inside a frame change we initiated (suppresses Frame events).
    programmatic: Cell<bool>,
    desc: RefCell<NativeDesc>,
    shown_once: Cell<bool>,
    /// Saved windowed `NSWindow` state while in DXGI borderless fullscreen.
    dxgi_fullscreen_saved: RefCell<Option<SavedWindowed>>,
}

thread_local! {
    static WINDOWS: RefCell<HashMap<usize, NativeWin>> = RefCell::new(HashMap::new());
    static APP_DELEGATE: RefCell<Option<Retained<AW2AppDelegate>>> = const { RefCell::new(None) };
    static ACTIVATED_ONCE: Cell<bool> = const { Cell::new(false) };
}

fn with_win<R>(hwnd: usize, f: impl FnOnce(&NativeWin) -> R) -> Option<R> {
    WINDOWS.with(|m| m.borrow().get(&hwnd).map(f))
}

/// HWND of one of our NSWindows (by object identity).
pub(super) fn hwnd_for_window(w: &NSWindow) -> Option<usize> {
    let p = w as *const NSWindow as usize;
    WINDOWS.with(|m| m.borrow().iter().find(|(_, n)| Retained::as_ptr(&n.window) as usize == p).map(|(h, _)| *h))
}

/// Content rect of a window in Cocoa global points.
pub(super) fn content_rect(hwnd: usize) -> Option<NSRect> {
    with_win(hwnd, |w| w.window.contentRectForFrameRect(w.window.frame()))
}

fn frame_changed(hwnd: usize) {
    let Some((client, programmatic)) = with_win(hwnd, |w| {
        let cr = w.window.contentRectForFrameRect(w.window.frame());
        let client = display::cocoa_to_px(cr.origin.x, cr.origin.y, cr.size.width, cr.size.height);
        w.shared.set_client_size(client.width(), client.height());
        (client, w.programmatic.get() || w.window.isMiniaturized())
    }) else {
        return;
    };
    if !programmatic {
        native_event(hwnd, NativeEvent::Frame { client });
    }
    update_presentation();
}

fn style_mask(d: &NativeDesc) -> NSWindowStyleMask {
    if !super::fullscreen_takeover() && d.style & ws::CHILD == 0 {
        return NSWindowStyleMask::Titled
            | NSWindowStyleMask::Closable
            | NSWindowStyleMask::Miniaturizable
            | NSWindowStyleMask::Resizable;
    }
    if d.style & ws::CHILD == 0 && d.style & ws::CAPTION == ws::CAPTION {
        let mut m = NSWindowStyleMask::Titled | NSWindowStyleMask::Closable;
        if d.style & ws::MINIMIZEBOX != 0 {
            m |= NSWindowStyleMask::Miniaturizable;
        }
        if d.style & ws::THICKFRAME != 0 {
            m |= NSWindowStyleMask::Resizable;
        }
        m
    } else {
        NSWindowStyleMask::Borderless
    }
}

fn cocoa_rect(r: &RECT) -> NSRect {
    let (x, y, w, h) = display::px_to_cocoa(r);
    NSRect::new(NSPoint::new(x, y), NSSize::new(w.max(1.0), h.max(1.0)))
}

/// NSWindowLevel values.
const NORMAL_LEVEL: isize = 0;
const FLOATING_LEVEL: isize = 3;

// ---------------------------------------------------------------------------
// Init and main loop
// ---------------------------------------------------------------------------

fn restore_cursor_on_exit() {
    unsafe {
        CGAssociateMouseAndMouseCursorPosition(1);
    }
}

pub(super) fn init() {
    let mtm = mtm();
    crate::process::register_exit_hook(restore_cursor_on_exit);
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    let delegate = AW2AppDelegate::new(mtm);
    app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    APP_DELEGATE.with(|d| *d.borrow_mut() = Some(delegate));
    build_menu(mtm, &app);
    app.finishLaunching();
    unsafe {
        // SetCursorPos must not freeze input for the default 250 ms.
        CGSetLocalEventsSuppressionInterval(0.0);
    }
    objc2_app_kit::NSEvent::setMouseCoalescingEnabled(false);
    display::refresh(mtm);
    super::keyboard::init_main_thread();
    init_task_dialog_dispatcher();
    crate::dxgi::adapter::register_fullscreen_toggle(window_set_dxgi_fullscreen);
    log::info!("macos: NSApplication ready (main thread owns AppKit)");
}

fn build_menu(mtm: MainThreadMarker, app: &NSApplication) {
    unsafe {
        let bar = NSMenu::new(mtm);
        let app_item = NSMenuItem::new(mtm);
        bar.addItem(&app_item);
        let menu = NSMenu::new(mtm);
        let app_name = crate::config::app_config().app_name;
        let hide = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str(&format!("Hide {app_name}")),
            Some(sel!(hide:)),
            &NSString::from_str("h"),
        );
        menu.addItem(&hide);
        let quit = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str(&format!("Quit {app_name}")),
            Some(sel!(terminate:)),
            &NSString::from_str("q"),
        );
        menu.addItem(&quit);
        app_item.setSubmenu(Some(&menu));

        let win_item = NSMenuItem::new(mtm);
        bar.addItem(&win_item);
        let win_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), &NSString::from_str("Window"));
        let min_item = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str("Minimize"),
            Some(sel!(performMiniaturize:)),
            &NSString::from_str("m"),
        );
        win_menu.addItem(&min_item);
        let zoom_item = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str("Zoom"),
            Some(sel!(performZoom:)),
            &NSString::from_str(""),
        );
        win_menu.addItem(&zoom_item);
        let fs_item = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str("Toggle Full Screen"),
            Some(sel!(toggleFullScreen:)),
            &NSString::from_str("f"),
        );
        win_menu.addItem(&fs_item);
        win_item.setSubmenu(Some(&win_menu));

        app.setMainMenu(Some(&bar));
    }
}

pub(super) fn run(exited: &dyn Fn() -> Option<u32>) -> u32 {
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    log::info!("macos: main loop running");
    loop {
        if let Some(code) = exited() {
            return code;
        }
        autoreleasepool(|_| unsafe {
            let until = NSDate::dateWithTimeIntervalSinceNow(0.05);
            let mut ev = app.nextEventMatchingMask_untilDate_inMode_dequeue(
                NSEventMask::Any,
                Some(&until),
                NSDefaultRunLoopMode,
                true,
            );
            let mut n = 0;
            while let Some(e) = ev {
                super::events::handle(&app, &e, mtm);
                n += 1;
                if n >= 512 {
                    break;
                }
                ev = app.nextEventMatchingMask_untilDate_inMode_dequeue(
                    NSEventMask::Any,
                    Some(&NSDate::distantPast()),
                    NSDefaultRunLoopMode,
                    true,
                );
            }
            super::gamepad::tick();
        });
    }
}

// ---------------------------------------------------------------------------
// Window operations
// ---------------------------------------------------------------------------

fn display_title(title: &str) -> String {
    if title.trim().is_empty() {
        crate::config::app_config().app_name
    } else {
        title.to_string()
    }
}

pub(super) fn window_create(hwnd: usize, desc: NativeDesc) -> bool {
    let mtm = mtm();
    let content = cocoa_rect(&desc.client);
    let mask = style_mask(&desc);
    let window: Retained<AW2Window> = unsafe {
        let this = AW2Window::alloc(mtm).set_ivars(());
        msg_send![super(this), initWithContentRect: content, styleMask: mask, backing: NSBackingStoreType::Buffered, defer: false]
    };
    unsafe { window.setReleasedWhenClosed(false) };
    let title = display_title(&desc.title);
    window.setTitle(&NSString::from_str(&title));
    window.setAcceptsMouseMovedEvents(true);
    window.setBackgroundColor(Some(&NSColor::blackColor()));
    if super::fullscreen_takeover() {
        window.setCollectionBehavior(NSWindowCollectionBehavior::FullScreenNone);
        window.setLevel(if desc.topmost { FLOATING_LEVEL } else { NORMAL_LEVEL });
    } else {
        window.setCollectionBehavior(NSWindowCollectionBehavior::FullScreenPrimary);
        window.setLevel(NORMAL_LEVEL);
    }

    let view: Retained<AW2View> = unsafe {
        let this = AW2View::alloc(mtm).set_ivars(());
        msg_send![super(this), initWithFrame: NSRect::new(NSPoint::new(0.0, 0.0), content.size)]
    };
    // NSViewWidthSizable (2) | NSViewHeightSizable (16) = 18 so live window resize
    // and green-button fullscreen resize the content view automatically.
    let _: () = unsafe { msg_send![&*view, setAutoresizingMask: 18usize] };
    let layer = CAMetalLayer::new();
    let win_scale = display::scale_for_rect(&desc.client);
    layer.setContentsScale(win_scale);
    layer.setOpaque(true);
    let _: () = unsafe { msg_send![&*layer, setContentsGravity: &*NSString::from_str("resizeAspect")] };
    view.setLayer(Some(&layer));
    view.setWantsLayer(true);
    window.setContentView(Some(&view));
    window.makeFirstResponder(Some(&view));

    let delegate = AW2WindowDelegate::new(mtm, hwnd);
    window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

    let shared = Arc::new(WindowShared {
        layer: Retained::as_ptr(&layer) as usize,
        client_px: AtomicU64::new(0),
        scale: AtomicU64::new(window.backingScaleFactor().to_bits()),
        occluded: AtomicBool::new(false),
    });
    shared.set_client_size(desc.client.width(), desc.client.height());
    super::register_shared(hwnd, shared.clone());
    log::info!(
        "NSWindow for hwnd {hwnd:#x}: {:?} content {}x{} px ({}x{} pt) at ({},{}) style {:?}",
        title,
        desc.client.width(),
        desc.client.height(),
        content.size.width,
        content.size.height,
        desc.client.left,
        desc.client.top,
        mask
    );
    WINDOWS.with(|m| {
        m.borrow_mut().insert(
            hwnd,
            NativeWin {
                window,
                view,
                _delegate: delegate,
                layer,
                shared,
                programmatic: Cell::new(false),
                desc: RefCell::new(desc),
                shown_once: Cell::new(false),
                dxgi_fullscreen_saved: RefCell::new(None),
            },
        )
    });
    true
}

pub(super) fn window_destroy(hwnd: usize) {
    let w = WINDOWS.with(|m| m.borrow_mut().remove(&hwnd));
    super::unregister_shared(hwnd);
    if let Some(w) = w {
        w.window.setDelegate(None);
        w.window.orderOut(None);
        w.window.close();
        let _ = (&w.view, &w.layer);
        log::info!("NSWindow for hwnd {hwnd:#x} destroyed");
    }
    update_presentation();
}

pub(super) fn window_set_frame(hwnd: usize, client: RECT) {
    with_win(hwnd, |w| {
        if w.dxgi_fullscreen_saved.borrow().is_some() {
            return;
        }
        let content = cocoa_rect(&client);
        let frame = w.window.frameRectForContentRect(content);
        w.programmatic.set(true);
        w.window.setFrame_display(frame, true);
        w.programmatic.set(false);
        w.desc.borrow_mut().client = client;
        w.shared.set_client_size(client.width(), client.height());
    });
    update_presentation();
}

pub(super) fn window_set_style(hwnd: usize, desc: NativeDesc) {
    with_win(hwnd, |w| {
        if w.dxgi_fullscreen_saved.borrow().is_some() {
            let mut cur = w.desc.borrow_mut();
            cur.title = desc.title;
            cur.style = desc.style;
            cur.exstyle = desc.exstyle;
            cur.topmost = desc.topmost;
            cur.dblclks = desc.dblclks;
            return;
        }
        let mask = style_mask(&desc);
        w.programmatic.set(true);
        if w.window.styleMask() != mask {
            log::debug!("hwnd {hwnd:#x}: style mask -> {mask:?}");
            w.window.setStyleMask(mask);
            w.window.makeFirstResponder(Some(&w.view));
        }
        let level = if super::fullscreen_takeover() && desc.topmost {
            FLOATING_LEVEL
        } else {
            NORMAL_LEVEL
        };
        w.window.setLevel(level);
        w.programmatic.set(false);
        *w.desc.borrow_mut() = desc;
    });
    update_presentation();
}

fn monitor_cocoa_frame(w: &NativeWin, target_rect: Option<RECT>) -> NSRect {
    if let Some(r) = target_rect {
        if let Some(m) = display::monitors().into_iter().find(|m| m.rect == r) {
            let (x, y, ww, hh) = m.cocoa_frame;
            return NSRect::new(NSPoint::new(x, y), NSSize::new(ww.max(1.0), hh.max(1.0)));
        }
        return cocoa_rect(&r);
    }
    if let Some(screen) = w.window.screen().or_else(|| NSScreen::mainScreen(mtm())) {
        return screen.frame();
    }
    if let Some(m) = display::monitors().into_iter().next() {
        let (x, y, ww, hh) = m.cocoa_frame;
        NSRect::new(NSPoint::new(x, y), NSSize::new(ww.max(1.0), hh.max(1.0)))
    } else {
        w.window.frame()
    }
}

fn window_set_dxgi_fullscreen(hwnd: usize, fullscreen: bool, target_rect: Option<RECT>) -> Option<RECT> {
    let result = with_win(hwnd, |w| {
        if fullscreen {
            let fs_frame = monitor_cocoa_frame(w, target_rect);
            if w.dxgi_fullscreen_saved.borrow().is_none() {
                *w.dxgi_fullscreen_saved.borrow_mut() = Some(SavedWindowed {
                    frame: w.window.frame(),
                    style_mask: w.window.styleMask(),
                    level: w.window.level(),
                    collection_behavior: w.window.collectionBehavior(),
                });
            }
            w.programmatic.set(true);
            if !w.window.styleMask().contains(NSWindowStyleMask::FullScreen) {
                w.window.setCollectionBehavior(NSWindowCollectionBehavior::FullScreenNone);
                w.window.setStyleMask(NSWindowStyleMask::Borderless);
                w.window.setLevel(NORMAL_LEVEL);
                w.window.setFrame_display(fs_frame, true);
                w.window.makeFirstResponder(Some(&w.view));
            }
            w.programmatic.set(false);
            let cr = w.window.contentRectForFrameRect(w.window.frame());
            let client = display::cocoa_to_px(cr.origin.x, cr.origin.y, cr.size.width, cr.size.height);
            w.desc.borrow_mut().client = client;
            w.shared.set_client_size(client.width(), client.height());
            log::info!(
                "NSWindow for hwnd {hwnd:#x}: entered DXGI borderless fullscreen ({}x{} px)",
                client.width(),
                client.height()
            );
            Some(client)
        } else {
            let saved = w.dxgi_fullscreen_saved.borrow_mut().take()?;
            w.programmatic.set(true);
            if !w.window.styleMask().contains(NSWindowStyleMask::FullScreen) {
                w.window.setStyleMask(saved.style_mask);
                w.window.setLevel(saved.level);
                w.window.setCollectionBehavior(saved.collection_behavior);
                w.window.setFrame_display(saved.frame, true);
                w.window.makeFirstResponder(Some(&w.view));
                let title = display_title(&w.desc.borrow().title);
                w.window.setTitle(&NSString::from_str(&title));
            }
            w.programmatic.set(false);
            let cr = w.window.contentRectForFrameRect(w.window.frame());
            let client = display::cocoa_to_px(cr.origin.x, cr.origin.y, cr.size.width, cr.size.height);
            w.desc.borrow_mut().client = client;
            w.shared.set_client_size(client.width(), client.height());
            log::info!(
                "NSWindow for hwnd {hwnd:#x}: restored windowed frame ({}x{} px)",
                client.width(),
                client.height()
            );
            Some(client)
        }
    })
    .flatten();
    if result.is_some() {
        update_presentation();
    }
    result
}

fn should_suppress_focus_steal() -> bool {
    if std::env::var("AW2_NO_ACTIVATE").ok().is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true")) {
        return true;
    }
    if std::env::var("AW2_ACTIVATE").ok().is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true")) {
        return false;
    }
    std::env::var_os("CLAUDECODE").is_some() || std::env::var_os("CONTROLLER_THREAD_ID").is_some()
}

fn activate_app(app: &NSApplication) {
    if should_suppress_focus_steal() {
        return;
    }
    if !app.isActive() || !ACTIVATED_ONCE.with(|c| c.get()) {
        ACTIVATED_ONCE.with(|c| c.set(true));
        #[allow(deprecated)]
        app.activateIgnoringOtherApps(true);
    }
}

pub(super) fn window_show(hwnd: usize, show: bool, activate: bool) {
    let mtm = mtm();
    with_win(hwnd, |w| {
        if show {
            if activate {
                activate_app(&NSApplication::sharedApplication(mtm));
                w.window.makeKeyAndOrderFront(None);
            } else {
                w.window.orderFront(None);
            }
            log::info!("NSWindow for hwnd {hwnd:#x} shown (activate={activate}, window number {})", w.window.windowNumber());
            if !w.shown_once.replace(true) {
                schedule_screenshot(hwnd);
            }
        } else {
            w.window.orderOut(None);
        }
    });
    update_presentation();
}

/// `AW2_SCREENSHOT=<file.png>`: some time after a window is first shown
/// (`AW2_SCREENSHOT_DELAY_MS`, default 3000), save the window and the
/// whole screen (`<file>_screen.png`).
fn schedule_screenshot(hwnd: usize) {
    let Ok(path) = std::env::var("AW2_SCREENSHOT") else { return };
    let delay = std::env::var("AW2_SCREENSHOT_DELAY_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(3000u64);
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(delay));
        super::run_async(move || {
            let screen = match path.strip_suffix(".png") {
                Some(stem) => format!("{stem}_screen.png"),
                None => format!("{path}_screen.png"),
            };
            for (p, whole) in [(path.as_str(), false), (screen.as_str(), true)] {
                match capture_png(hwnd, p, whole) {
                    Ok((w, h)) => log::info!("screenshot of hwnd {hwnd:#x} ({}) -> {p} ({w}x{h})", if whole { "screen" } else { "window" }),
                    Err(e) => log::warn!("screenshot {p} failed: {e}"),
                }
            }
        });
    });
}

pub(super) fn window_minimize(hwnd: usize, minimize: bool) {
    with_win(hwnd, |w| {
        if minimize && !w.window.isMiniaturized() {
            w.window.miniaturize(None);
        } else if !minimize && w.window.isMiniaturized() {
            w.window.deminiaturize(None);
        }
    });
}

pub(super) fn window_set_title(hwnd: usize, title: String) {
    with_win(hwnd, |w| w.window.setTitle(&NSString::from_str(&display_title(&title))));
}

pub(super) fn window_activate(hwnd: usize) {
    let mtm = mtm();
    with_win(hwnd, |w| {
        if !w.window.isVisible() {
            return;
        }
        activate_app(&NSApplication::sharedApplication(mtm));
        if !w.window.isKeyWindow() {
            w.window.makeKeyAndOrderFront(None);
        }
    });
}

/// Auto-hide the menu bar and Dock when a key window is in DXGI borderless
/// fullscreen or when `AW2_FULLSCREEN=1` explicitly opts into borderless takeover.
fn update_presentation() {
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    let fullscreen = WINDOWS.with(|m| {
        let monitors = display::monitors();
        m.borrow().values().any(|w| {
            if !w.window.isKeyWindow() {
                return false;
            }
            if w.dxgi_fullscreen_saved.borrow().is_some() {
                return true;
            }
            if !super::fullscreen_takeover() {
                return false;
            }
            let d = w.desc.borrow();
            style_mask(&d) == NSWindowStyleMask::Borderless
                && monitors.iter().any(|mon| {
                    d.client.left <= mon.rect.left
                        && d.client.top <= mon.rect.top
                        && d.client.right >= mon.rect.right
                        && d.client.bottom >= mon.rect.bottom
                })
        })
    });
    let want = if fullscreen {
        NSApplicationPresentationOptions::AutoHideMenuBar | NSApplicationPresentationOptions::AutoHideDock
    } else {
        NSApplicationPresentationOptions::Default
    };
    if app.presentationOptions() != want {
        log::info!("presentation options -> {want:?}");
        app.setPresentationOptions(want);
    }
}

// ---------------------------------------------------------------------------
// Cursor
// ---------------------------------------------------------------------------

thread_local! {
    static CURSOR_HIDDEN: Cell<bool> = const { Cell::new(false) };
    static CURSOR_RELATIVE: Cell<bool> = const { Cell::new(false) };
    static LAST_CURSOR_SCALE: Cell<f64> = const { Cell::new(1.0) };
    static LAST_CURSOR: RefCell<CursorState> = const {
        RefCell::new(CursorState {
            hidden: false,
            shape: Shape::Arrow,
            relative: false,
        })
    };
}

fn active_cursor_scale() -> f64 {
    let loc = NSEvent::mouseLocation();
    let s = display::scale_for_cocoa_point(loc.x, loc.y);
    if s.is_finite() && s > 0.0 {
        s
    } else {
        display::scale().max(1.0)
    }
}

pub(super) fn cursor_update(s: CursorState) {
    LAST_CURSOR.with(|last| *last.borrow_mut() = s.clone());
    apply_cursor(s, active_cursor_scale());
}

/// Called from `events::locate` when the pointer moves so moving onto the macOS
/// title bar or outside the window immediately restores the visible arrow cursor,
/// and moving a custom cursor across mixed-DPI screens updates its point size.
pub(super) fn refresh_cursor_for_client(inside: bool, scale: f64) {
    let prev = super::mouse_in_client();
    let scale = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let scale_changed = LAST_CURSOR_SCALE.with(|c| (c.get() - scale).abs() > 1e-6);
    if prev != inside {
        super::set_mouse_in_client(inside);
        let s = LAST_CURSOR.with(|last| last.borrow().clone());
        apply_cursor(s, scale);
    } else if inside && scale_changed {
        let s = LAST_CURSOR.with(|last| last.borrow().clone());
        if matches!(s.shape, Shape::Custom(_)) {
            apply_cursor(s, scale);
        } else {
            LAST_CURSOR_SCALE.with(|c| c.set(scale));
        }
    }
}

/// Convert Win32 pixel dimensions and hotspot of a custom cursor into Cocoa
/// logical points for a display with backing scale `scale`, while keeping the
/// underlying CGImage at full native pixel resolution.
fn custom_cursor_metrics(img: &CustomCursorImage, scale: f64) -> (NSSize, NSPoint) {
    let s = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let size = NSSize::new((img.width as f64) / s, (img.height as f64) / s);
    let hot_x_px = (img.hotspot_x as f64).clamp(0.0, img.width.saturating_sub(1) as f64);
    let hot_y_px = (img.hotspot_y as f64).clamp(0.0, img.height.saturating_sub(1) as f64);
    let hot = NSPoint::new(hot_x_px / s, hot_y_px / s);
    (size, hot)
}

fn custom_ns_cursor(img: &CustomCursorImage, scale: f64) -> Option<Retained<NSCursor>> {
    let w = img.width as usize;
    let h = img.height as usize;
    if w == 0 || h == 0 || img.rgba.len() < w.saturating_mul(h).saturating_mul(4) {
        return None;
    }
    let (size, hot) = custom_cursor_metrics(img, scale);
    unsafe {
        let cf_data = CFDataCreate(std::ptr::null(), img.rgba.as_ptr(), (w * h * 4) as isize);
        if cf_data.is_null() {
            return None;
        }
        let provider = CGDataProviderCreateWithCFData(cf_data);
        CFRelease(cf_data);
        if provider.is_null() {
            return None;
        }
        let cs = CGColorSpaceCreateDeviceRGB();
        if cs.is_null() {
            CGDataProviderRelease(provider);
            return None;
        }
        // kCGBitmapByteOrderDefault (0) | kCGImageAlphaPremultipliedLast (1)
        let cg_img = CGImageCreate(w, h, 8, 32, w * 4, cs, 1, provider, std::ptr::null(), false, 0);
        CGColorSpaceRelease(cs);
        CGDataProviderRelease(provider);
        if cg_img.is_null() {
            return None;
        }
        let Some(cls) = objc2::runtime::AnyClass::get(c"NSImage") else {
            CGImageRelease(cg_img);
            return None;
        };
        let alloc: *mut AnyObject = msg_send![cls, alloc];
        let ns_img_raw: *mut AnyObject =
            msg_send![alloc, initWithCGImage: cg_img.cast::<CGImageOpaque>(), size: size];
        CGImageRelease(cg_img);
        if ns_img_raw.is_null() {
            return None;
        }
        let ns_img = Retained::<NSImage>::from_raw(ns_img_raw.cast())?;
        Some(NSCursor::initWithImage_hotSpot(NSCursor::alloc(), &ns_img, hot))
    }
}

fn apply_cursor(s: CursorState, scale: f64) {
    let mtm = mtm();
    let in_client = super::fullscreen_takeover() || super::mouse_in_client();
    let relative = s.relative && super::lock_cursor_allowed() && in_client;
    let hidden = s.hidden && in_client;
    LAST_CURSOR_SCALE.with(|c| c.set(scale));

    if CURSOR_RELATIVE.with(|c| c.replace(relative)) != relative {
        unsafe { CGAssociateMouseAndMouseCursorPosition(!relative as i32) };
        if relative {
            // Park the (frozen, hidden) pointer over our window so clicks
            // can't land in another app.
            let key = WINDOWS.with(|m| m.borrow().iter().find(|(_, w)| w.window.isKeyWindow()).map(|(h, _)| *h));
            if let Some(cr) = key.and_then(content_rect) {
                let ph = display::primary_height_pt();
                let c = CGPoint { x: cr.origin.x + cr.size.width / 2.0, y: ph - (cr.origin.y + cr.size.height / 2.0) };
                unsafe { CGWarpMouseCursorPosition(c) };
            }
        }
        log::debug!("native cursor relative={relative}");
    }
    if CURSOR_HIDDEN.with(|c| c.replace(hidden)) != hidden {
        if hidden {
            NSCursor::hide();
        } else {
            NSCursor::unhide();
        }
        log::debug!("native cursor hidden={hidden}");
    }
    if !hidden {
        #[allow(deprecated)]
        let c = match &s.shape {
            Shape::IBeam => NSCursor::IBeamCursor(),
            Shape::Cross => NSCursor::crosshairCursor(),
            Shape::Hand => NSCursor::pointingHandCursor(),
            Shape::SizeWE => NSCursor::resizeLeftRightCursor(),
            Shape::SizeNS => NSCursor::resizeUpDownCursor(),
            Shape::SizeAll => NSCursor::openHandCursor(),
            Shape::No => NSCursor::operationNotAllowedCursor(),
            Shape::Custom(img) => custom_ns_cursor(img, scale).unwrap_or_else(NSCursor::arrowCursor),
            _ => NSCursor::arrowCursor(),
        };
        c.set();
    }
    let _ = mtm;
}

pub(super) fn cursor_warp(p: POINT) {
    let (cx, cy) = display::px_point_to_cg(p);
    unsafe { CGWarpMouseCursorPosition(CGPoint { x: cx, y: cy }) };
}

// ---------------------------------------------------------------------------
// Clipboard (NSPasteboard)
// ---------------------------------------------------------------------------

fn pasteboard_get_text(pb: &NSPasteboard) -> Option<String> {
    let ty = unsafe { NSPasteboardTypeString };
    pb.stringForType(ty).map(|s| s.to_string())
}

fn pasteboard_set_text(pb: &NSPasteboard, text: &str) -> Option<isize> {
    let ty = unsafe { NSPasteboardTypeString };
    pb.clearContents();
    if pb.setString_forType(&NSString::from_str(text), ty) {
        Some(pb.changeCount())
    } else {
        None
    }
}

pub(super) fn clipboard_get_text() -> Option<String> {
    let pb = NSPasteboard::generalPasteboard();
    pasteboard_get_text(&pb)
}

pub(super) fn clipboard_set_text(text: &str) -> Option<isize> {
    let pb = NSPasteboard::generalPasteboard();
    pasteboard_set_text(&pb, text)
}

pub(super) fn clipboard_clear() -> isize {
    let pb = NSPasteboard::generalPasteboard();
    pb.clearContents()
}

pub(super) fn clipboard_change_count() -> isize {
    NSPasteboard::generalPasteboard().changeCount()
}

// ---------------------------------------------------------------------------
// Alert (presented & updated via CFRunLoopSource, outside _dispatch_main_q)
// ---------------------------------------------------------------------------

type MainRunLoopTask = Box<dyn FnOnce() + Send>;

struct RunLoopTaskQueue {
    tasks: Mutex<VecDeque<MainRunLoopTask>>,
    source: std::sync::atomic::AtomicUsize,
}

/// Dual-source CoreFoundation run-loop dispatcher for modal TaskDialogs.
///
/// Presenting `NSAlert::runModal()` via a `CFRunLoopSource` registered in
/// `kCFRunLoopDefaultMode` keeps `_dispatch_main_q` completely unoccupied while
/// the modal session runs (libdispatch refuses recursive main-queue draining).
/// Live updates (`TDM_CLICK_BUTTON`, `SetWindowTextW`, selection updates) are
/// signaled on a second `CFRunLoopSource` registered in both
/// `kCFRunLoopCommonModes` and `NSModalPanelRunLoopMode`, so they execute
/// immediately while `NSAlert::runModal()` is open.
struct RunLoopDispatcher {
    run_loop: usize,
    present_src: usize,
    update_src: usize,
    present_q: *mut RunLoopTaskQueue,
    update_q: *mut RunLoopTaskQueue,
}

// SAFETY: `CFRunLoopSourceSignal` and `CFRunLoopWakeUp` are documented
// thread-safe by Apple CoreFoundation; queues are protected by `Mutex`.
unsafe impl Send for RunLoopDispatcher {}
unsafe impl Sync for RunLoopDispatcher {}

extern "C" fn perform_one_runloop_task(info: *mut c_void) {
    if info.is_null() {
        return;
    }
    let q = unsafe { &*(info as *const RunLoopTaskQueue) };
    let (task, more) = {
        let mut guard = q.tasks.lock().unwrap();
        let t = guard.pop_front();
        (t, !guard.is_empty())
    };
    if more {
        let src = q.source.load(Ordering::Acquire) as *mut c_void;
        if !src.is_null() {
            unsafe { CFRunLoopSourceSignal(src) };
        }
    }
    if let Some(f) = task {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| autoreleasepool(|_| f()))).is_err() {
            log::error!("macos: panic in task-dialog run-loop presentation");
        }
    }
}

extern "C" fn perform_all_runloop_tasks(info: *mut c_void) {
    if info.is_null() {
        return;
    }
    let q = unsafe { &*(info as *const RunLoopTaskQueue) };
    loop {
        let task = q.tasks.lock().unwrap().pop_front();
        let Some(f) = task else { break };
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| autoreleasepool(|_| f()))).is_err() {
            log::error!("macos: panic in task-dialog run-loop update");
        }
    }
}

impl RunLoopDispatcher {
    fn new_for_current_runloop() -> Option<Self> {
        unsafe {
            let rl = CFRunLoopGetCurrent();
            if rl.is_null() {
                return None;
            }
            CFRetain(rl as *const c_void);

            let present_q = Box::into_raw(Box::new(RunLoopTaskQueue {
                tasks: Mutex::new(VecDeque::new()),
                source: std::sync::atomic::AtomicUsize::new(0),
            }));
            let update_q = Box::into_raw(Box::new(RunLoopTaskQueue {
                tasks: Mutex::new(VecDeque::new()),
                source: std::sync::atomic::AtomicUsize::new(0),
            }));

            let mut present_ctx = CFRunLoopSourceContext {
                version: 0,
                info: present_q as *mut c_void,
                retain: None,
                release: None,
                copy_description: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(perform_one_runloop_task),
            };
            let present_src = CFRunLoopSourceCreate(std::ptr::null(), 0, &mut present_ctx);

            let mut update_ctx = CFRunLoopSourceContext {
                version: 0,
                info: update_q as *mut c_void,
                retain: None,
                release: None,
                copy_description: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(perform_all_runloop_tasks),
            };
            let update_src = CFRunLoopSourceCreate(std::ptr::null(), 0, &mut update_ctx);

            if present_src.is_null() || update_src.is_null() {
                if !present_src.is_null() {
                    CFRelease(present_src as *const c_void);
                }
                if !update_src.is_null() {
                    CFRelease(update_src as *const c_void);
                }
                CFRelease(rl as *const c_void);
                drop(Box::from_raw(present_q));
                drop(Box::from_raw(update_q));
                return None;
            }

            (*present_q).source.store(present_src as usize, Ordering::Release);
            (*update_q).source.store(update_src as usize, Ordering::Release);

            // Register BOTH presentation and update sources in common + modal-panel modes:
            // - Running outside `_dispatch_main_q` keeps libdispatch unblocked during `runModal()`.
            // - Registering `present_src` in modal mode as well allows callbacks from an active
            //   TaskDialog (e.g. radio/verification notifications) to open nested TaskDialogs
            //   immediately instead of deadlocking behind the outer modal loop.
            CFRunLoopAddSource(rl, present_src, kCFRunLoopCommonModes);
            CFRunLoopAddSource(rl, update_src, kCFRunLoopCommonModes);
            let modal_mode = CFStringCreateWithCString(
                std::ptr::null(),
                c"NSModalPanelRunLoopMode".as_ptr(),
                0x0800_0100, // kCFStringEncodingUTF8
            );
            if !modal_mode.is_null() {
                CFRunLoopAddSource(rl, present_src, modal_mode);
                CFRunLoopAddSource(rl, update_src, modal_mode);
                CFRelease(modal_mode);
            }

            Some(Self {
                run_loop: rl as usize,
                present_src: present_src as usize,
                update_src: update_src as usize,
                present_q,
                update_q,
            })
        }
    }

    fn schedule_present(&self, f: impl FnOnce() + Send + 'static) {
        unsafe {
            (*self.present_q).tasks.lock().unwrap().push_back(Box::new(f));
            CFRunLoopSourceSignal(self.present_src as *mut c_void);
            CFRunLoopWakeUp(self.run_loop as *mut c_void);
        }
    }

    fn schedule_update(&self, f: impl FnOnce() + Send + 'static) {
        unsafe {
            (*self.update_q).tasks.lock().unwrap().push_back(Box::new(f));
            CFRunLoopSourceSignal(self.update_src as *mut c_void);
            CFRunLoopWakeUp(self.run_loop as *mut c_void);
        }
    }
}

static TASK_DIALOG_DISPATCHER: OnceLock<RunLoopDispatcher> = OnceLock::new();

fn init_task_dialog_dispatcher() {
    if let Some(d) = RunLoopDispatcher::new_for_current_runloop() {
        let _ = TASK_DIALOG_DISPATCHER.set(d);
    }
}

pub(super) fn schedule_task_dialog_present(f: impl FnOnce() + Send + 'static) {
    if let Some(d) = TASK_DIALOG_DISPATCHER.get() {
        d.schedule_present(f);
    } else {
        super::run_async(f);
    }
}

pub(super) fn schedule_task_dialog_update(f: impl FnOnce() + Send + 'static) {
    if let Some(d) = TASK_DIALOG_DISPATCHER.get() {
        d.schedule_update(f);
    } else {
        super::run_async(f);
    }
}

const NS_CONTROL_STATE_OFF: isize = 0;
const NS_CONTROL_STATE_ON: isize = 1;
/// `NSModalResponseAbort`: cleanly terminates `NSAlert::runModal()` when the
/// owning Win32 thread receives `WM_QUIT` without synthesizing a button click.
const TASK_DIALOG_ABORT_CODE: isize = -1001;

#[derive(Clone, Copy, Debug)]
enum DeferredModalStop {
    Click(usize),
    Abort,
}

#[derive(Clone)]
enum TaskDialogEventSink {
    Channel(std::sync::mpsc::Sender<super::TaskDialogEvent>),
    Inline(*mut dyn FnMut(super::TaskDialogEvent)),
}

impl TaskDialogEventSink {
    fn emit(&self, ev: super::TaskDialogEvent) {
        match self {
            TaskDialogEventSink::Channel(tx) => {
                let _ = tx.send(ev);
            }
            TaskDialogEventSink::Inline(cb) => unsafe { (**cb)(ev) },
        }
    }
}

struct ActiveTaskDialogAlert {
    session_id: u64,
    alert: Retained<NSAlert>,
    target: Retained<TaskDialogRadioTarget>,
    action_buttons: Vec<Retained<NSButton>>,
    radio_buttons: Vec<Retained<NSButton>>,
    link_buttons: Vec<Retained<NSButton>>,
    links: Vec<(String, String)>,
    sink: TaskDialogEventSink,
    /// Deferred programmatic dismissal or `WM_QUIT` abort received while a nested
    /// modal alert was on top.
    deferred_stop: Option<DeferredModalStop>,
}

thread_local! {
    static ACTIVE_TASK_DIALOG_ALERTS: RefCell<Vec<ActiveTaskDialogAlert>> = const { RefCell::new(Vec::new()) };
    static PENDING_TASK_DIALOG_UPDATES: RefCell<HashMap<u64, Vec<super::TaskDialogLiveUpdate>>> = RefCell::new(HashMap::new());
}

struct SessionIvar {
    session_id: Cell<u64>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "PortTaskDialogRadioTarget"]
    #[ivars = SessionIvar]
    struct TaskDialogRadioTarget;

    impl TaskDialogRadioTarget {
        #[unsafe(method(radioSelected:))]
        fn radio_selected(&self, sender: &AnyObject) {
            let tag: isize = unsafe { msg_send![sender, tag] };
            if tag >= 0 {
                let sid = self.ivars().session_id.get();
                // Extract sink and release RefCell borrow before invoking so nested
                // modal dialogs opened from inline callbacks never double-borrow.
                let sink = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                    let s = stack.borrow();
                    s.iter()
                        .rev()
                        .find(|a| sid == 0 || a.session_id == sid)
                        .map(|a| a.sink.clone())
                });
                if let Some(sink) = sink {
                    let is_inline = matches!(sink, TaskDialogEventSink::Inline(_));
                    sink.emit(super::TaskDialogEvent::RadioSelected(tag as usize));
                    if is_inline && sid != 0 && crate::user32::message::has_pending_quit() {
                        task_dialog_alert_cancel(sid);
                    }
                }
            }
        }

        #[unsafe(method(verificationToggled:))]
        fn verification_toggled(&self, sender: &AnyObject) {
            let st: isize = unsafe { msg_send![sender, state] };
            let checked = st == NS_CONTROL_STATE_ON;
            let sid = self.ivars().session_id.get();
            let sink = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                let s = stack.borrow();
                s.iter()
                    .rev()
                    .find(|a| sid == 0 || a.session_id == sid)
                    .map(|a| a.sink.clone())
            });
            if let Some(sink) = sink {
                let is_inline = matches!(sink, TaskDialogEventSink::Inline(_));
                sink.emit(super::TaskDialogEvent::VerificationToggled(checked));
                if is_inline && sid != 0 && crate::user32::message::has_pending_quit() {
                    task_dialog_alert_cancel(sid);
                }
            }
        }

        #[unsafe(method(hyperlinkClicked:))]
        fn hyperlink_clicked(&self, sender: &AnyObject) {
            let tag: isize = unsafe { msg_send![sender, tag] };
            if tag < 0 {
                return;
            }
            let sid = self.ivars().session_id.get();
            let payload = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                let s = stack.borrow();
                s.iter()
                    .rev()
                    .find(|a| sid == 0 || a.session_id == sid)
                    .and_then(|a| {
                        let href = a.links.get(tag as usize)?.1.clone();
                        Some((a.sink.clone(), href))
                    })
            });
            if let Some((sink, href)) = payload {
                let is_inline = matches!(sink, TaskDialogEventSink::Inline(_));
                sink.emit(super::TaskDialogEvent::HyperlinkClicked(href));
                if is_inline && sid != 0 && crate::user32::message::has_pending_quit() {
                    task_dialog_alert_cancel(sid);
                }
            }
        }
    }
);

impl TaskDialogRadioTarget {
    fn new(mtm: MainThreadMarker, session_id: u64) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(SessionIvar { session_id: Cell::new(session_id) });
        unsafe { msg_send![super(this), init] }
    }
}

pub(super) fn alert(caption: &str, text: &str, buttons: &[String], critical: bool) -> usize {
    let mut noop = |_ev: super::TaskDialogEvent| {};
    let mut noop_timer = |_ms: u32| false;
    let mut origin = std::time::Instant::now();
    task_dialog_alert_inline(
        0,
        caption,
        text,
        &[],
        buttons,
        &[],
        None,
        &[],
        &[],
        None,
        None,
        false,
        critical,
        &mut origin,
        None,
        &mut noop,
        &mut noop_timer,
    )
    .map(|r| r.button_index)
    .unwrap_or(0)
}

fn rebuild_task_dialog_accessory_view(
    mtm: MainThreadMarker,
    alert: &NSAlert,
    target: &TaskDialogRadioTarget,
    radio_buttons: &[Retained<NSButton>],
    link_buttons: &mut Vec<Retained<NSButton>>,
    links: &[(String, String)],
) {
    for b in link_buttons.drain(..) {
        b.removeFromSuperview();
    }
    for b in radio_buttons {
        b.removeFromSuperview();
    }
    let total_rows = radio_buttons.len() + links.len();
    if total_rows == 0 {
        alert.setAccessoryView(None);
        return;
    }
    let row_h = 24.0f64;
    let width = 320.0f64;
    let gap = if !radio_buttons.is_empty() && !links.is_empty() { 6.0f64 } else { 0.0f64 };
    let total_h = row_h * (total_rows as f64) + gap + 4.0;
    let container = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(width, total_h)),
    );
    for (idx, btn) in radio_buttons.iter().enumerate() {
        // Position top-to-bottom in AppKit's bottom-up coordinate system.
        let y = total_h - ((idx + 1) as f64) * row_h;
        btn.setFrame(NSRect::new(NSPoint::new(0.0, y), NSSize::new(width, row_h)));
        container.addSubview(btn);
    }
    let link_top = total_h - (radio_buttons.len() as f64) * row_h - gap;
    for (idx, (label, _href)) in links.iter().enumerate() {
        let btn = unsafe {
            NSButton::buttonWithTitle_target_action(
                &NSString::from_str(label),
                Some(target),
                Some(sel!(hyperlinkClicked:)),
                mtm,
            )
        };
        btn.setBordered(false);
        btn.setContentTintColor(Some(&NSColor::linkColor()));
        let _: () = unsafe { msg_send![&*btn, setAlignment: 0isize] };
        let _: () = unsafe { msg_send![&*btn, setTag: idx as isize] };
        let y = link_top - ((idx + 1) as f64) * row_h;
        btn.setFrame(NSRect::new(NSPoint::new(0.0, y), NSSize::new(width, row_h)));
        container.addSubview(&btn);
        link_buttons.push(btn);
    }
    alert.setAccessoryView(Some(&container));
}

fn apply_live_update_to_alert(
    mtm: MainThreadMarker,
    alert: &NSAlert,
    target: &TaskDialogRadioTarget,
    action_buttons: &[Retained<NSButton>],
    radio_buttons: &[Retained<NSButton>],
    link_buttons: &mut Vec<Retained<NSButton>>,
    links_slot: &mut Vec<(String, String)>,
    update: &super::TaskDialogLiveUpdate,
) {
    alert.setMessageText(&NSString::from_str(&update.caption));
    alert.setInformativeText(&NSString::from_str(&update.text));
    *links_slot = update.links.clone();
    if update.links.len() == link_buttons.len() {
        for (i, b) in link_buttons.iter().enumerate() {
            if let Some((label, _)) = update.links.get(i) {
                b.setTitle(&NSString::from_str(label));
            }
        }
    } else {
        rebuild_task_dialog_accessory_view(
            mtm,
            alert,
            target,
            radio_buttons,
            link_buttons,
            &update.links,
        );
        let _: () = unsafe { msg_send![alert, layout] };
    }
    for (i, b) in action_buttons.iter().enumerate() {
        if let Some(&en) = update.button_enabled.get(i) {
            b.setEnabled(en);
        }
    }
    for (i, b) in radio_buttons.iter().enumerate() {
        if let Some(&en) = update.radio_enabled.get(i) {
            b.setEnabled(en);
        }
    }
    if let Some(ridx) = update.selected_radio_idx {
        for (i, b) in radio_buttons.iter().enumerate() {
            let st = if i == ridx { NS_CONTROL_STATE_ON } else { NS_CONTROL_STATE_OFF };
            let _: () = unsafe { msg_send![&**b, setState: st] };
        }
    }
    if let Some(checked) = update.verification_checked {
        if let Some(cb) = alert.suppressionButton() {
            let st = if checked { NS_CONTROL_STATE_ON } else { NS_CONTROL_STATE_OFF };
            let _: () = unsafe { msg_send![&*cb, setState: st] };
        }
    }
}

pub(super) fn task_dialog_alert_update(session_id: u64, update: &super::TaskDialogLiveUpdate) {
    let mtm = mtm();
    let (applied, stop_code) = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
        let mut s = stack.borrow_mut();
        let Some(pos) = s.iter().rposition(|a| a.session_id == session_id) else {
            return (false, None);
        };
        let is_top = pos + 1 == s.len();
        let active = &mut s[pos];
        apply_live_update_to_alert(
            mtm,
            &active.alert,
            &active.target,
            &active.action_buttons,
            &active.radio_buttons,
            &mut active.link_buttons,
            &mut active.links,
            update,
        );
        let stop = if let Some(bidx) = update.click_button_idx {
            if is_top {
                Some(1000 + bidx as isize)
            } else {
                active.deferred_stop = Some(DeferredModalStop::Click(bidx));
                None
            }
        } else {
            None
        };
        (true, stop)
    });
    if let Some(code) = stop_code {
        let app = NSApplication::sharedApplication(mtm);
        app.stopModalWithCode(code);
    }
    if !applied && session_id != 0 {
        PENDING_TASK_DIALOG_UPDATES.with(|map| {
            let mut m = map.borrow_mut();
            let q = m.entry(session_id).or_default();
            if q.len() < 64 {
                q.push(update.clone());
            }
        });
    }
}

/// Abort/unwind an active native modal TaskDialog session (e.g. when the
/// owning Win32 thread receives `WM_QUIT`).
pub(super) fn task_dialog_alert_cancel(session_id: u64) {
    let mtm = mtm();
    let should_stop = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
        let mut s = stack.borrow_mut();
        let Some(pos) = s.iter().rposition(|a| a.session_id == session_id) else {
            return false;
        };
        let is_top = pos + 1 == s.len();
        if is_top {
            true
        } else {
            s[pos].deferred_stop = Some(DeferredModalStop::Abort);
            false
        }
    });
    if should_stop {
        let app = NSApplication::sharedApplication(mtm);
        app.stopModalWithCode(TASK_DIALOG_ABORT_CODE);
    }
}

#[allow(clippy::too_many_arguments)]
fn run_task_dialog_alert(
    session_id: u64,
    caption: &str,
    text: &str,
    links: &[(String, String)],
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    critical: bool,
    sink: TaskDialogEventSink,
    cancelled: Option<&std::sync::atomic::AtomicBool>,
) -> Option<super::TaskDialogAlertResult> {
    if cancelled.is_some_and(|c| c.load(Ordering::Acquire)) {
        if session_id != 0 {
            PENDING_TASK_DIALOG_UPDATES.with(|map| {
                map.borrow_mut().remove(&session_id);
            });
        }
        return None;
    }
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    activate_app(&app);
    let a = NSAlert::new(mtm);
    a.setMessageText(&NSString::from_str(caption));
    a.setInformativeText(&NSString::from_str(text));
    a.setAlertStyle(if critical { NSAlertStyle::Critical } else { NSAlertStyle::Warning });
    let custom_default = default_button_idx.filter(|&idx| idx < buttons.len());
    let mut action_buttons: Vec<Retained<NSButton>> = Vec::with_capacity(buttons.len());
    for (idx, b) in buttons.iter().enumerate() {
        let btn = a.addButtonWithTitle(&NSString::from_str(b));
        if let Some(def_idx) = custom_default {
            let eq = NSString::from_str(if idx == def_idx { "\r" } else { "" });
            let _: () = unsafe { msg_send![&*btn, setKeyEquivalent: &*eq] };
        }
        btn.setEnabled(button_enabled.get(idx).copied().unwrap_or(true));
        action_buttons.push(btn);
    }

    let target = TaskDialogRadioTarget::new(mtm, session_id);

    if let Some(vtext) = verification_text {
        a.setShowsSuppressionButton(true);
        if let Some(cb) = a.suppressionButton() {
            cb.setTitle(&NSString::from_str(vtext));
            let state = if verification_checked { NS_CONTROL_STATE_ON } else { NS_CONTROL_STATE_OFF };
            let _: () = unsafe { msg_send![&*cb, setState: state] };
            let _: () = unsafe { msg_send![&*cb, setTarget: &*target] };
            let _: () = unsafe { msg_send![&*cb, setAction: sel!(verificationToggled:)] };
        }
    }

    let mut radio_buttons: Vec<Retained<NSButton>> = Vec::with_capacity(radios.len());
    for (idx, label) in radios.iter().enumerate() {
        let btn = unsafe {
            NSButton::radioButtonWithTitle_target_action(
                &NSString::from_str(label),
                Some(&target),
                Some(sel!(radioSelected:)),
                mtm,
            )
        };
        let _: () = unsafe { msg_send![&*btn, setTag: idx as isize] };
        let state = if default_radio_idx == Some(idx) {
            NS_CONTROL_STATE_ON
        } else {
            NS_CONTROL_STATE_OFF
        };
        let _: () = unsafe { msg_send![&*btn, setState: state] };
        btn.setEnabled(radio_enabled.get(idx).copied().unwrap_or(true));
        radio_buttons.push(btn);
    }
    let mut link_buttons: Vec<Retained<NSButton>> = Vec::with_capacity(links.len());
    rebuild_task_dialog_accessory_view(
        mtm,
        &a,
        &target,
        &radio_buttons,
        &mut link_buttons,
        links,
    );

    struct ActiveAlertGuard {
        session_id: u64,
        mtm: MainThreadMarker,
    }
    impl Drop for ActiveAlertGuard {
        fn drop(&mut self) {
            let deferred_parent_stop = ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                let mut s = stack.borrow_mut();
                if let Some(pos) = s.iter().rposition(|a| a.session_id == self.session_id) {
                    s.remove(pos);
                }
                s.last_mut()
                    .and_then(|parent| parent.deferred_stop.take())
            });
            if self.session_id != 0 {
                PENDING_TASK_DIALOG_UPDATES.with(|map| {
                    map.borrow_mut().remove(&self.session_id);
                });
            }
            if let Some(stop) = deferred_parent_stop {
                let app = NSApplication::sharedApplication(self.mtm);
                let code = match stop {
                    DeferredModalStop::Click(bidx) => 1000 + bidx as isize,
                    DeferredModalStop::Abort => TASK_DIALOG_ABORT_CODE,
                };
                app.stopModalWithCode(code);
            }
        }
    }
    ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
        stack.borrow_mut().push(ActiveTaskDialogAlert {
            session_id,
            alert: a.clone(),
            target: target.clone(),
            action_buttons: action_buttons.clone(),
            radio_buttons: radio_buttons.clone(),
            link_buttons,
            links: links.to_vec(),
            sink,
            deferred_stop: None,
        });
    });
    let _active_guard = ActiveAlertGuard { session_id, mtm };

    // Apply any live updates that arrived before `run_task_dialog_alert` began.
    let mut pre_clicked: Option<usize> = None;
    if session_id != 0 {
        let queued = PENDING_TASK_DIALOG_UPDATES.with(|map| map.borrow_mut().remove(&session_id));
        if let Some(updates) = queued {
            ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                if let Some(entry) = stack.borrow_mut().iter_mut().rev().find(|e| e.session_id == session_id) {
                    for upd in updates {
                        apply_live_update_to_alert(
                            mtm,
                            &entry.alert,
                            &entry.target,
                            &entry.action_buttons,
                            &entry.radio_buttons,
                            &mut entry.link_buttons,
                            &mut entry.links,
                            &upd,
                        );
                        if let Some(bidx) = upd.click_button_idx {
                            pre_clicked = Some(bidx);
                        }
                    }
                }
            });
        }
    }

    if cancelled.is_some_and(|c| c.load(Ordering::Acquire)) {
        return None;
    }

    let r = if let Some(bidx) = pre_clicked {
        1000 + bidx as isize
    } else {
        a.runModal()
    };
    if r == TASK_DIALOG_ABORT_CODE || cancelled.is_some_and(|c| c.load(Ordering::Acquire)) {
        return None;
    }
    // NSAlertFirstButtonReturn = 1000
    let button_index = (r - 1000).clamp(0, buttons.len().saturating_sub(1) as isize) as usize;
    let radio_index = if radio_buttons.is_empty() {
        None
    } else {
        radio_buttons.iter().position(|b| {
            let st: isize = unsafe { msg_send![&**b, state] };
            st == NS_CONTROL_STATE_ON
        })
    };
    let verified = if verification_text.is_some() {
        a.suppressionButton().is_some_and(|cb| {
            let st: isize = unsafe { msg_send![&*cb, state] };
            st == NS_CONTROL_STATE_ON
        })
    } else {
        false
    };
    let _keep_target_alive = target;
    Some(super::TaskDialogAlertResult {
        button_index,
        radio_index,
        verified,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn task_dialog_alert_channel(
    session_id: u64,
    caption: &str,
    text: &str,
    links: &[(String, String)],
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    critical: bool,
    tx: std::sync::mpsc::Sender<super::TaskDialogEvent>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    if let Some(res) = run_task_dialog_alert(
        session_id,
        caption,
        text,
        links,
        buttons,
        button_enabled,
        default_button_idx,
        radios,
        radio_enabled,
        default_radio_idx,
        verification_text,
        verification_checked,
        critical,
        TaskDialogEventSink::Channel(tx.clone()),
        Some(&cancelled),
    ) {
        let _ = tx.send(super::TaskDialogEvent::ButtonClicked(res));
    }
}

struct InlineTimerCtx {
    session_id: u64,
    origin: *mut std::time::Instant,
    cb: *mut dyn FnMut(u32) -> bool,
}

extern "C" fn inline_task_dialog_timer_cb(_timer: *mut c_void, info: *mut c_void) {
    if info.is_null() {
        return;
    }
    let ctx = unsafe { &mut *(info as *mut InlineTimerCtx) };
    let now = std::time::Instant::now();
    let elapsed_ms = unsafe {
        now.saturating_duration_since(*ctx.origin)
            .as_millis()
            .min(u32::MAX as u128) as u32
    };
    let reset = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        (*ctx.cb)(elapsed_ms)
    }))
    .unwrap_or(false);
    if reset {
        unsafe { *ctx.origin = std::time::Instant::now() };
    }
    if ctx.session_id != 0 && crate::user32::message::has_pending_quit() {
        task_dialog_alert_cancel(ctx.session_id);
    }
}

struct InlineTimerGuard {
    timer: *mut c_void,
    ctx: *mut InlineTimerCtx,
}

impl Drop for InlineTimerGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.timer.is_null() {
                CFRunLoopTimerInvalidate(self.timer);
                CFRelease(self.timer as *const c_void);
            }
            if !self.ctx.is_null() {
                drop(Box::from_raw(self.ctx));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn task_dialog_alert_inline(
    session_id: u64,
    caption: &str,
    text: &str,
    links: &[(String, String)],
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    critical: bool,
    timer_origin: &mut std::time::Instant,
    timer_interval: Option<std::time::Duration>,
    on_event: &mut dyn FnMut(super::TaskDialogEvent),
    on_timer: &mut dyn FnMut(u32) -> bool,
) -> Option<super::TaskDialogAlertResult> {
    // SAFETY: ACTIVE_TASK_DIALOG_ALERTS and InlineTimerGuard are both torn down before this returns.
    let cb_ptr: *mut dyn FnMut(super::TaskDialogEvent) =
        unsafe { std::mem::transmute::<&mut dyn FnMut(super::TaskDialogEvent), *mut dyn FnMut(super::TaskDialogEvent)>(on_event) };
    let _timer_guard = timer_interval.and_then(|iv| unsafe {
        let secs = iv.as_secs_f64().max(0.005);
        let timer_cb_ptr: *mut dyn FnMut(u32) -> bool =
            std::mem::transmute::<&mut dyn FnMut(u32) -> bool, *mut dyn FnMut(u32) -> bool>(on_timer);
        let ctx_box = Box::into_raw(Box::new(InlineTimerCtx {
            session_id,
            origin: timer_origin as *mut std::time::Instant,
            cb: timer_cb_ptr,
        }));
        let mut cf_ctx = CFRunLoopTimerContext {
            version: 0,
            info: ctx_box as *mut c_void,
            retain: None,
            release: None,
            copy_description: None,
        };
        let timer = CFRunLoopTimerCreate(
            std::ptr::null(),
            CFAbsoluteTimeGetCurrent() + secs,
            secs,
            0,
            0,
            inline_task_dialog_timer_cb,
            &mut cf_ctx,
        );
        if timer.is_null() {
            drop(Box::from_raw(ctx_box));
            return None;
        }
        let rl = CFRunLoopGetCurrent();
        CFRunLoopAddTimer(rl, timer, kCFRunLoopCommonModes);
        let modal_mode = CFStringCreateWithCString(
            std::ptr::null(),
            c"NSModalPanelRunLoopMode".as_ptr(),
            0x0800_0100,
        );
        if !modal_mode.is_null() {
            CFRunLoopAddTimer(rl, timer, modal_mode);
            CFRelease(modal_mode);
        }
        Some(InlineTimerGuard { timer, ctx: ctx_box })
    });
    run_task_dialog_alert(
        session_id,
        caption,
        text,
        links,
        buttons,
        button_enabled,
        default_button_idx,
        radios,
        radio_enabled,
        default_radio_idx,
        verification_text,
        verification_checked,
        critical,
        TaskDialogEventSink::Inline(cb_ptr),
        None,
    )
}

// ---------------------------------------------------------------------------
// Native first-launch shader/PSO prewarm progress window
// ---------------------------------------------------------------------------

struct ProgressWindow {
    window: Retained<NSWindow>,
    detail: Retained<NSTextField>,
    bar: Retained<NSProgressIndicator>,
}

thread_local! {
    static PROGRESS_WIN: RefCell<Option<ProgressWindow>> = const { RefCell::new(None) };
}

pub(super) fn progress_window_open(title: &str, detail: &str, total: usize) {
    progress_window_close();
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    activate_app(&app);

    let rect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(460.0, 126.0));
    let style = NSWindowStyleMask::Titled | NSWindowStyleMask::Miniaturizable;
    let window = unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            rect,
            style,
            NSBackingStoreType::Buffered,
            false,
        )
    };
    window.setTitle(&NSString::from_str(title));
    unsafe { window.setReleasedWhenClosed(false) };
    window.center();

    let heading = NSTextField::labelWithString(&NSString::from_str(title), mtm);
    heading.setFrame(NSRect::new(NSPoint::new(20.0, 84.0), NSSize::new(420.0, 22.0)));
    heading.setFont(Some(&NSFont::boldSystemFontOfSize(14.0)));

    let detail_label = NSTextField::labelWithString(&NSString::from_str(detail), mtm);
    detail_label.setFrame(NSRect::new(NSPoint::new(20.0, 56.0), NSSize::new(420.0, 20.0)));
    detail_label.setFont(Some(&NSFont::systemFontOfSize(12.0)));

    let bar = NSProgressIndicator::initWithFrame(
        NSProgressIndicator::alloc(mtm),
        NSRect::new(NSPoint::new(20.0, 24.0), NSSize::new(420.0, 18.0)),
    );
    bar.setStyle(NSProgressIndicatorStyle::Bar);
    bar.setMinValue(0.0);
    bar.setMaxValue(total.max(1) as f64);
    bar.setDoubleValue(0.0);
    bar.setIndeterminate(total == 0);
    if total == 0 {
        unsafe { bar.startAnimation(None) };
    }

    if let Some(cv) = window.contentView() {
        cv.addSubview(&heading);
        cv.addSubview(&detail_label);
        cv.addSubview(&bar);
    }
    window.makeKeyAndOrderFront(None);

    PROGRESS_WIN.with(|slot| {
        *slot.borrow_mut() = Some(ProgressWindow {
            window,
            detail: detail_label,
            bar,
        });
    });
}

pub(super) fn progress_window_update(detail: &str, done: usize, total: usize) {
    PROGRESS_WIN.with(|slot| {
        let borrow = slot.borrow();
        let Some(pw) = borrow.as_ref() else {
            return;
        };
        pw.detail.setStringValue(&NSString::from_str(detail));
        if total == 0 {
            pw.bar.setIndeterminate(true);
            unsafe { pw.bar.startAnimation(None) };
        } else {
            if pw.bar.isIndeterminate() {
                unsafe { pw.bar.stopAnimation(None) };
                pw.bar.setIndeterminate(false);
            }
            pw.bar.setMaxValue(total as f64);
            pw.bar.setDoubleValue(done.min(total) as f64);
        }
    });
}

pub(super) fn progress_window_close() {
    PROGRESS_WIN.with(|slot| {
        if let Some(pw) = slot.borrow_mut().take() {
            unsafe { pw.bar.stopAnimation(None) };
            pw.window.orderOut(None);
            pw.window.close();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFRunLoopDefaultMode: *const c_void;
        fn CFRunLoopRunInMode(mode: *const c_void, secs: f64, ret_after_source: u8) -> i32;
        fn CFRunLoopStop(rl: *mut c_void);
        fn CFRunLoopRemoveSource(rl: *mut c_void, src: *mut c_void, mode: *const c_void);
    }

    #[test]
    fn unique_nspasteboard_text_roundtrip() {
        // Uses an isolated pasteboard name so the host user's generalPasteboard is untouched.
        let pb = NSPasteboard::pasteboardWithUniqueName();
        let c0 = pb.changeCount();
        let text = "Hello — 世界\nLine 2";
        let c1 = pasteboard_set_text(&pb, text).expect("setString_forType");
        assert!(c1 > c0);
        assert_eq!(pasteboard_get_text(&pb).as_deref(), Some(text));
        pb.clearContents();
        assert_eq!(pasteboard_get_text(&pb), None);
    }

    #[test]
    fn task_dialog_runloop_dispatcher_delivers_updates_inside_modal_mode() {
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        let (disp_tx, disp_rx) = mpsc::channel::<Arc<RunLoopDispatcher>>();
        let (modal_entered_tx, modal_entered_rx) = mpsc::channel::<()>();
        let (modal_exited_tx, modal_exited_rx) = mpsc::channel::<bool>();
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
        let update_ran = Arc::new(AtomicBool::new(false));
        let update_ran_modal = Arc::clone(&update_ran);

        let rl_thread = std::thread::spawn(move || {
            let disp = Arc::new(
                RunLoopDispatcher::new_for_current_runloop().expect("CFRunLoop dispatcher"),
            );
            disp_tx.send(Arc::clone(&disp)).unwrap();

            // Pump default mode until our presentation task runs and returns.
            let deadline = Instant::now() + Duration::from_secs(3);
            while Instant::now() < deadline && !update_ran_modal.load(Ordering::Acquire) {
                unsafe {
                    CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.05, 1);
                }
            }
            let _ = shutdown_rx.recv_timeout(Duration::from_secs(2));

            // Clean up sources created on this test thread's run loop.
            unsafe {
                let rl = disp.run_loop as *mut c_void;
                let psrc = disp.present_src as *mut c_void;
                let usrc = disp.update_src as *mut c_void;
                CFRunLoopRemoveSource(rl, psrc, kCFRunLoopCommonModes);
                CFRunLoopRemoveSource(rl, usrc, kCFRunLoopCommonModes);
                let modal_mode = CFStringCreateWithCString(
                    std::ptr::null(),
                    c"NSModalPanelRunLoopMode".as_ptr(),
                    0x0800_0100,
                );
                if !modal_mode.is_null() {
                    CFRunLoopRemoveSource(rl, psrc, modal_mode);
                    CFRunLoopRemoveSource(rl, usrc, modal_mode);
                    CFRelease(modal_mode);
                }
                CFRelease(psrc as *const c_void);
                CFRelease(usrc as *const c_void);
                CFRelease(rl as *const c_void);
                drop(Box::from_raw(disp.present_q));
                drop(Box::from_raw(disp.update_q));
            }
        });

        let disp = disp_rx.recv_timeout(Duration::from_secs(2)).expect("dispatcher");
        let update_ran_check = Arc::clone(&update_ran);
        disp.schedule_present(move || {
            // Simulate NSAlert::runModal() entering a nested NSModalPanelRunLoopMode loop
            // inside the presentation callback.
            let _ = modal_entered_tx.send(());
            unsafe {
                let modal_mode = CFStringCreateWithCString(
                    std::ptr::null(),
                    c"NSModalPanelRunLoopMode".as_ptr(),
                    0x0800_0100,
                );
                assert!(!modal_mode.is_null());
                CFRunLoopRunInMode(modal_mode, 3.0, 0);
                CFRelease(modal_mode);
            }
            let _ = modal_exited_tx.send(update_ran_check.load(Ordering::Acquire));
        });

        // Wait until the outer modal run-loop mode is active, then verify a nested
        // presentation runs live inside the outer modal loop AND is dismissed by a live update.
        modal_entered_rx.recv_timeout(Duration::from_secs(2)).expect("outer modal entered");

        let (nested_entered_tx, nested_entered_rx) = mpsc::channel::<()>();
        let (nested_exited_tx, nested_exited_rx) = mpsc::channel::<bool>();
        let nested_dismissed = Arc::new(AtomicBool::new(false));
        let nested_dismissed_check = Arc::clone(&nested_dismissed);
        disp.schedule_present(move || {
            let _ = nested_entered_tx.send(());
            unsafe {
                let modal_mode = CFStringCreateWithCString(
                    std::ptr::null(),
                    c"NSModalPanelRunLoopMode".as_ptr(),
                    0x0800_0100,
                );
                assert!(!modal_mode.is_null());
                CFRunLoopRunInMode(modal_mode, 3.0, 0);
                CFRelease(modal_mode);
            }
            let _ = nested_exited_tx.send(nested_dismissed_check.load(Ordering::Acquire));
        });

        nested_entered_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("nested modal must present inside outer NSModalPanelRunLoopMode");

        let nested_dismissed_set = Arc::clone(&nested_dismissed);
        disp.schedule_update(move || {
            nested_dismissed_set.store(true, Ordering::Release);
            unsafe {
                CFRunLoopStop(CFRunLoopGetCurrent());
            }
        });
        assert!(
            nested_exited_rx
                .recv_timeout(Duration::from_secs(2))
                .expect("nested modal exited"),
            "nested modal must be dismissed by its live update while outer modal remains open"
        );

        let t0 = Instant::now();
        let update_ran_set = Arc::clone(&update_ran);
        disp.schedule_update(move || {
            update_ran_set.store(true, Ordering::Release);
            unsafe {
                CFRunLoopStop(CFRunLoopGetCurrent());
            }
        });

        let saw_update_before_exit = modal_exited_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("outer modal exited via live update");
        assert!(
            saw_update_before_exit,
            "live update must execute inside nested NSModalPanelRunLoopMode and stop it"
        );
        assert!(
            t0.elapsed() < Duration::from_secs(2),
            "modal loop must be dismissed immediately by live update, not timeout"
        );
        drop(disp);
        let _ = shutdown_tx.send(());
        rl_thread.join().unwrap();
    }

    #[test]
    fn custom_cursor_scales_logical_size_and_hotspot_preserving_bitmap_pixels() {
        let img = CustomCursorImage {
            width: 32,
            height: 32,
            hotspot_x: 10,
            hotspot_y: 6,
            rgba: vec![0xFF; 32 * 32 * 4].into(),
        };

        // 1x display (or PORT_RETINA=0): 1 Win32 pixel == 1 Cocoa point.
        let (sz1, hot1) = custom_cursor_metrics(&img, 1.0);
        assert_eq!((sz1.width, sz1.height), (32.0, 32.0));
        assert_eq!((hot1.x, hot1.y), (10.0, 6.0));

        // 2x Retina display: 32x32 Win32 pixels -> 16x16 Cocoa points, hotspot halved.
        let (sz2, hot2) = custom_cursor_metrics(&img, 2.0);
        assert_eq!((sz2.width, sz2.height), (16.0, 16.0));
        assert_eq!((hot2.x, hot2.y), (5.0, 3.0));

        // Out-of-range hotspot clamps in pixel space before scaling to points.
        let clamped_img = CustomCursorImage {
            width: 32,
            height: 16,
            hotspot_x: 100,
            hotspot_y: 50,
            rgba: vec![0xFF; 32 * 16 * 4].into(),
        };
        let (sz_c, hot_c) = custom_cursor_metrics(&clamped_img, 2.0);
        assert_eq!((sz_c.width, sz_c.height), (16.0, 8.0));
        assert_eq!((hot_c.x, hot_c.y), (15.5, 7.5));

        // Full NSCursor + NSImage construction on 2x Retina keeps the 32x32 backing
        // bitmap resolution while exposing 16x16 pt logical size and (5, 3) pt hotspot.
        let cursor = custom_ns_cursor(&img, 2.0).expect("custom NSCursor at 2x");
        let cur_img = cursor.image();
        assert_eq!(cur_img.size(), NSSize::new(16.0, 16.0));
        assert_eq!(cursor.hotSpot(), NSPoint::new(5.0, 3.0));
        unsafe {
            let cg: *mut CGImageOpaque = msg_send![
                &*cur_img,
                CGImageForProposedRect: std::ptr::null_mut::<NSRect>(),
                context: std::ptr::null::<AnyObject>(),
                hints: std::ptr::null::<AnyObject>()
            ];
            assert!(!cg.is_null(), "expected backing CGImage on custom cursor NSImage");
            assert_eq!(CGImageGetWidth(cg.cast()), 32);
            assert_eq!(CGImageGetHeight(cg.cast()), 32);
        }
    }
}
