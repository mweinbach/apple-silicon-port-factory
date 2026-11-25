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
    NSApplicationPresentationOptions, NSApplicationTerminateReply, NSBackingStoreType, NSColor, NSCursor, NSEventMask,
    NSFont, NSImage, NSMenu, NSMenuItem, NSPasteboard, NSPasteboardTypeString, NSProgressIndicator,
    NSProgressIndicatorStyle, NSScreen, NSTextField, NSView, NSWindow, NSWindowCollectionBehavior, NSWindowDelegate,
    NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString,
};
use objc2_quartz_core::CAMetalLayer;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

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

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(p: *const c_void);
    fn CFDataCreate(alloc: *const c_void, bytes: *const u8, len: isize) -> *const c_void;
    fn CFURLCreateFromFileSystemRepresentation(alloc: *const c_void, buf: *const u8, len: isize, is_dir: bool) -> *const c_void;
    fn CFStringCreateWithCString(alloc: *const c_void, s: *const std::ffi::c_char, enc: u32) -> *const c_void;
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
    static LAST_CURSOR: RefCell<CursorState> = const {
        RefCell::new(CursorState {
            hidden: false,
            shape: Shape::Arrow,
            relative: false,
        })
    };
}

pub(super) fn cursor_update(s: CursorState) {
    LAST_CURSOR.with(|last| *last.borrow_mut() = s.clone());
    apply_cursor(s);
}

/// Called from `events::locate` when the pointer enters or leaves the client area
/// so moving onto the macOS title bar or outside the window immediately restores
/// the visible arrow cursor.
pub(super) fn refresh_cursor_for_client(inside: bool) {
    let prev = super::mouse_in_client();
    if prev != inside {
        super::set_mouse_in_client(inside);
        let s = LAST_CURSOR.with(|last| last.borrow().clone());
        apply_cursor(s);
    }
}

fn custom_ns_cursor(img: &CustomCursorImage) -> Option<Retained<NSCursor>> {
    let w = img.width as usize;
    let h = img.height as usize;
    if w == 0 || h == 0 || img.rgba.len() < w.saturating_mul(h).saturating_mul(4) {
        return None;
    }
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
        let size = NSSize::new(img.width as f64, img.height as f64);
        let Some(cls) = objc2::runtime::AnyClass::get(c"NSImage") else {
            CGImageRelease(cg_img);
            return None;
        };
        let alloc: *mut AnyObject = msg_send![cls, alloc];
        let ns_img_raw: *mut AnyObject = msg_send![alloc, initWithCGImage: cg_img, size: size];
        CGImageRelease(cg_img);
        if ns_img_raw.is_null() {
            return None;
        }
        let ns_img = Retained::<NSImage>::from_raw(ns_img_raw.cast())?;
        let hot = NSPoint::new(
            (img.hotspot_x as f64).clamp(0.0, img.width.saturating_sub(1) as f64),
            (img.hotspot_y as f64).clamp(0.0, img.height.saturating_sub(1) as f64),
        );
        Some(NSCursor::initWithImage_hotSpot(NSCursor::alloc(), &ns_img, hot))
    }
}

fn apply_cursor(s: CursorState) {
    let mtm = mtm();
    let in_client = super::fullscreen_takeover() || super::mouse_in_client();
    let relative = s.relative && super::lock_cursor_allowed() && in_client;
    let hidden = s.hidden && in_client;

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
            Shape::Custom(img) => custom_ns_cursor(img).unwrap_or_else(NSCursor::arrowCursor),
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
// Alert
// ---------------------------------------------------------------------------

pub(super) fn alert(caption: &str, text: &str, buttons: &[String], critical: bool) -> usize {
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    activate_app(&app);
    let a = NSAlert::new(mtm);
    a.setMessageText(&NSString::from_str(caption));
    a.setInformativeText(&NSString::from_str(text));
    a.setAlertStyle(if critical { NSAlertStyle::Critical } else { NSAlertStyle::Warning });
    for b in buttons {
        a.addButtonWithTitle(&NSString::from_str(b));
    }
    let r = a.runModal();
    // NSAlertFirstButtonReturn = 1000
    (r - 1000).clamp(0, buttons.len().saturating_sub(1) as isize) as usize
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
}
