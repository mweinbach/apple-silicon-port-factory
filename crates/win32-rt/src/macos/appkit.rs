//! AppKit side of the backend. Everything here runs on the main thread
//! (marshalled from [`super`]); nothing here calls guest code or the TEB.

use super::display;
use super::{CursorState, NativeControlDesc, NativeControlKind, NativeDesc, WindowShared};
use crate::user32::controls;
use crate::user32::cursor::{CustomCursorImage, IconPayload, Shape};
use crate::user32::window::{native_event, NativeEvent};
use crate::user32::{ws, POINT, RECT};
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, sel, AnyThread, DefinedClass, MainThreadOnly};
use objc2_app_kit::{
    NSAlert, NSAlertStyle, NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
    NSApplicationPresentationOptions, NSApplicationTerminateReply, NSBackingStoreType, NSBezelStyle,
    NSBitmapImageRep, NSBorderType, NSBox, NSBoxType, NSButton, NSButtonType,
    NSCalibratedRGBColorSpace, NSColor, NSControlStateValueMixed, NSControlStateValueOff,
    NSControlStateValueOn, NSControlTextEditingDelegate, NSCursor, NSEvent, NSEventMask,
    NSEventModifierFlags, NSFont, NSImage, NSImageScaling, NSImageView, NSMenu, NSMenuItem,
    NSPasteboard, NSPasteboardTypeString, NSProgressIndicator, NSProgressIndicatorStyle, NSScreen,
    NSScrollView, NSSecureTextField, NSTextAlignment, NSTextDelegate, NSTextField,
    NSTextFieldDelegate, NSTextView, NSTextViewDelegate, NSView, NSWindow,
    NSWindowCollectionBehavior, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{
    MainThreadMarker, NSData, NSDate, NSDefaultRunLoopMode, NSNotification, NSObject, NSObjectProtocol,
    NSPoint, NSRange, NSRect, NSSize, NSString,
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
            let changed = with_win(self.hwnd(), |w| {
                // NSWindowOcclusionStateVisible = 1 << 1
                let st: usize = unsafe { msg_send![&*w.window, occlusionState] };
                let now_occluded = st & 2 == 0;
                w.shared.occluded.swap(now_occluded, Ordering::AcqRel) != now_occluded
            })
            .unwrap_or(false);
            if changed {
                crate::dxgi::notify_occlusion_status_changed();
            }
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
            crate::dxgi::notify_adapters_changed();
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
    _metal_view: Retained<NSView>,
    _delegate: Retained<AW2WindowDelegate>,
    layer: Retained<CAMetalLayer>,
    shared: Arc<WindowShared>,
    /// Inside a frame change we initiated (suppresses Frame events).
    programmatic: Cell<bool>,
    desc: RefCell<NativeDesc>,
    shown_once: Cell<bool>,
    has_controls: Cell<bool>,
    /// Saved windowed `NSWindow` state while in DXGI borderless fullscreen.
    dxgi_fullscreen_saved: RefCell<Option<SavedWindowed>>,
}

thread_local! {
    static WINDOWS: RefCell<HashMap<usize, NativeWin>> = RefCell::new(HashMap::new());
    static CONTROLS: RefCell<HashMap<usize, NativeControl>> = RefCell::new(HashMap::new());
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
    reposition_child_controls(hwnd);
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

    let bounds = NSRect::new(NSPoint::new(0.0, 0.0), content.size);
    let view: Retained<AW2View> = unsafe {
        let this = AW2View::alloc(mtm).set_ivars(());
        msg_send![super(this), initWithFrame: bounds]
    };
    // NSViewWidthSizable (2) | NSViewHeightSizable (16) = 18 so live window resize
    // and green-button fullscreen resize the content view automatically.
    let _: () = unsafe { msg_send![&*view, setAutoresizingMask: 18usize] };
    view.setWantsLayer(true);

    // Host CAMetalLayer on a dedicated full-bleed background subview so native
    // AppKit child controls added to `view` composite cleanly above it.
    let metal_view = NSView::initWithFrame(NSView::alloc(mtm), bounds);
    let _: () = unsafe { msg_send![&*metal_view, setAutoresizingMask: 18usize] };
    let layer = CAMetalLayer::new();
    let win_scale = display::scale_for_rect(&desc.client);
    layer.setContentsScale(win_scale);
    layer.setOpaque(true);
    let _: () = unsafe { msg_send![&*layer, setContentsGravity: &*NSString::from_str("resizeAspect")] };
    metal_view.setLayer(Some(&layer));
    metal_view.setWantsLayer(true);
    view.addSubview(&metal_view);

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
                _metal_view: metal_view,
                _delegate: delegate,
                layer,
                shared,
                programmatic: Cell::new(false),
                desc: RefCell::new(desc),
                shown_once: Cell::new(false),
                has_controls: Cell::new(false),
                dxgi_fullscreen_saved: RefCell::new(None),
            },
        )
    });
    true
}

pub(super) fn window_destroy(hwnd: usize) {
    let orphan_ctrls: Vec<usize> = CONTROLS.with(|m| {
        m.borrow()
            .iter()
            .filter(|(_, c)| c.parent_hwnd == hwnd)
            .map(|(h, _)| *h)
            .collect()
    });
    for ch in orphan_ctrls {
        control_destroy(ch);
    }
    let w = WINDOWS.with(|m| m.borrow_mut().remove(&hwnd));
    super::unregister_shared(hwnd);
    if let Some(w) = w {
        w.window.setDelegate(None);
        w.window.orderOut(None);
        w.window.close();
        let _ = (&w.view, &w._metal_view, &w.layer);
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
    reposition_child_controls(hwnd);
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
    let (dxgi_fs, fullscreen) = WINDOWS.with(|m| {
        let monitors = display::monitors();
        let mut any_dxgi_fs = false;
        let mut any_fs = false;
        for w in m.borrow().values() {
            if !w.window.isKeyWindow() {
                continue;
            }
            if w.dxgi_fullscreen_saved.borrow().is_some() {
                any_dxgi_fs = true;
                any_fs = true;
                break;
            }
            if !super::fullscreen_takeover() {
                continue;
            }
            let d = w.desc.borrow();
            if style_mask(&d) == NSWindowStyleMask::Borderless
                && monitors.iter().any(|mon| {
                    d.client.left <= mon.rect.left
                        && d.client.top <= mon.rect.top
                        && d.client.right >= mon.rect.right
                        && d.client.bottom >= mon.rect.bottom
                })
            {
                any_fs = true;
            }
        }
        (any_dxgi_fs, any_fs)
    });
    if super::set_dxgi_fullscreen_active(dxgi_fs) {
        crate::user32::cursor::update_state();
    }
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
        crate::user32::cursor::update_state();
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
    let in_client = super::fullscreen_takeover()
        || super::dxgi_fullscreen_active()
        || super::mouse_in_client();
    let relative = s.relative && !super::cursor_capture_disallowed() && in_client;
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
/// Sentinel modal return code used when `TDM_NAVIGATE_PAGE` replaces the active
/// TaskDialog page while its native `NSAlert` is open so the caller immediately
/// rebuilds and re-presents the dialog with the new configuration.
const TASK_DIALOG_NAVIGATE_CODE: isize = -2001;
/// Sentinel modal return code used when the user cancels a `TDF_ALLOW_DIALOG_CANCELLATION`
/// dialog via Escape, Cmd+W, or the window close button (yielding `IDCANCEL`).
const TASK_DIALOG_CANCEL_CODE: isize = -2002;

#[derive(Clone, Copy, Debug)]
enum DeferredModalStop {
    Click(usize),
    Navigate,
    Abort,
}

/// Compute the replacement `NSButton` key equivalent for a TaskDialog action button
/// based on explicit default and `IDCANCEL` button indices (never by inspecting caption text).
///
/// - Assigns Return (`"\r"`) to `custom_default` and strips AppKit's default Return from
///   non-default buttons.
/// - Assigns Escape (`"\u{1b}"`) to `cancel_idx` (unless `cancel_idx` is the effective default
///   button) regardless of its localized caption.
/// - Strips AppKit's title-based Escape binding from any button that is not `cancel_idx`.
fn task_dialog_button_key_equivalent(
    idx: usize,
    existing_key_equiv: &str,
    custom_default: Option<usize>,
    cancel_idx: Option<usize>,
) -> Option<&'static str> {
    let effective_default = custom_default.unwrap_or(0);
    if custom_default == Some(idx) {
        return Some("\r");
    }
    if cancel_idx == Some(idx) && idx != effective_default {
        return if existing_key_equiv == "\u{1b}" { None } else { Some("\u{1b}") };
    }
    if custom_default.is_some() && existing_key_equiv == "\r" {
        return Some("");
    }
    if existing_key_equiv == "\u{1b}" {
        return Some("");
    }
    None
}

/// True when `run_task_dialog_alert` must attach a hidden Escape (`"\u{1b}"`) shortcut
/// button because no visible action button carries Escape, yet cancellation is permitted
/// either by `TDF_ALLOW_DIALOG_CANCELLATION` or by an `IDCANCEL` button that is also the
/// effective default button (holding Return `"\r"`).
fn task_dialog_needs_hidden_escape_shortcut(
    allow_cancellation: bool,
    cancel_idx: Option<usize>,
    has_escape_button: bool,
) -> bool {
    !has_escape_button && (allow_cancellation || cancel_idx.is_some())
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
    progress_bar: Option<Retained<NSProgressIndicator>>,
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

    unsafe impl NSObjectProtocol for TaskDialogRadioTarget {}

    unsafe impl NSWindowDelegate for TaskDialogRadioTarget {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            let app = NSApplication::sharedApplication(mtm());
            app.stopModalWithCode(TASK_DIALOG_CANCEL_CODE);
            false
        }
    }

    impl TaskDialogRadioTarget {
        #[unsafe(method(cancelDialog:))]
        fn cancel_dialog(&self, _sender: &AnyObject) {
            let app = NSApplication::sharedApplication(mtm());
            app.stopModalWithCode(TASK_DIALOG_CANCEL_CODE);
        }

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
    let cancel_idx = buttons.iter().position(|b| b.eq_ignore_ascii_case("cancel"));
    match task_dialog_alert_inline(
        0,
        caption,
        text,
        &[],
        None,
        buttons,
        &[],
        None,
        cancel_idx,
        false,
        &[],
        &[],
        None,
        None,
        false,
        false,
        critical,
        &mut origin,
        None,
        &mut noop,
        &mut noop_timer,
    ) {
        super::TaskDialogOutcome::Completed(r) => r.button_index,
        _ => 0,
    }
}

fn apply_progress_state_to_indicator(
    bar: &NSProgressIndicator,
    progress: super::TaskDialogProgressState,
) {
    bar.setIndeterminate(progress.marquee);
    if progress.marquee {
        if progress.marquee_running {
            unsafe { bar.startAnimation(None) };
        } else {
            unsafe { bar.stopAnimation(None) };
        }
    } else {
        unsafe { bar.stopAnimation(None) };
        let lo = progress.min as f64;
        let hi = (progress.max as f64).max(lo);
        bar.setMinValue(lo);
        bar.setMaxValue(hi);
        bar.setDoubleValue((progress.pos as f64).clamp(lo, hi));
    }
}

fn rebuild_task_dialog_accessory_view(
    mtm: MainThreadMarker,
    alert: &NSAlert,
    target: &TaskDialogRadioTarget,
    progress_bar: Option<&Retained<NSProgressIndicator>>,
    radio_buttons: &[Retained<NSButton>],
    link_buttons: &mut Vec<Retained<NSButton>>,
    links: &[(String, String)],
) {
    if let Some(bar) = progress_bar {
        bar.removeFromSuperview();
    }
    for b in link_buttons.drain(..) {
        b.removeFromSuperview();
    }
    for b in radio_buttons {
        b.removeFromSuperview();
    }
    let total_rows = radio_buttons.len() + links.len();
    if progress_bar.is_none() && total_rows == 0 {
        alert.setAccessoryView(None);
        return;
    }
    let bar_h = if progress_bar.is_some() { 18.0f64 } else { 0.0f64 };
    let bar_gap = if progress_bar.is_some() && total_rows > 0 { 8.0f64 } else { 0.0f64 };
    let row_h = 24.0f64;
    let width = 320.0f64;
    let link_gap = if !radio_buttons.is_empty() && !links.is_empty() { 6.0f64 } else { 0.0f64 };
    let total_h = bar_h + bar_gap + row_h * (total_rows as f64) + link_gap + 4.0;
    let container = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(width, total_h)),
    );
    let mut cursor_top = total_h;
    if let Some(bar) = progress_bar {
        cursor_top -= bar_h;
        bar.setFrame(NSRect::new(NSPoint::new(0.0, cursor_top), NSSize::new(width, bar_h)));
        container.addSubview(bar);
        cursor_top -= bar_gap;
    }
    for btn in radio_buttons {
        // Position top-to-bottom in AppKit's bottom-up coordinate system.
        cursor_top -= row_h;
        btn.setFrame(NSRect::new(NSPoint::new(0.0, cursor_top), NSSize::new(width, row_h)));
        container.addSubview(btn);
    }
    cursor_top -= link_gap;
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
        cursor_top -= row_h;
        btn.setFrame(NSRect::new(NSPoint::new(0.0, cursor_top), NSSize::new(width, row_h)));
        container.addSubview(&btn);
        link_buttons.push(btn);
    }
    alert.setAccessoryView(Some(&container));
}

#[allow(clippy::too_many_arguments)]
fn apply_live_update_to_alert(
    mtm: MainThreadMarker,
    alert: &NSAlert,
    target: &TaskDialogRadioTarget,
    progress_bar: Option<&Retained<NSProgressIndicator>>,
    action_buttons: &[Retained<NSButton>],
    radio_buttons: &[Retained<NSButton>],
    link_buttons: &mut Vec<Retained<NSButton>>,
    links_slot: &mut Vec<(String, String)>,
    update: &super::TaskDialogLiveUpdate,
) {
    alert.setMessageText(&NSString::from_str(&update.caption));
    alert.setInformativeText(&NSString::from_str(&update.text));
    if let (Some(bar), Some(progress)) = (progress_bar, update.progress) {
        apply_progress_state_to_indicator(bar, progress);
    }
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
            progress_bar,
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
        if update.navigate_page {
            let stop = if is_top {
                Some(TASK_DIALOG_NAVIGATE_CODE)
            } else {
                active.deferred_stop = Some(DeferredModalStop::Navigate);
                None
            };
            return (true, stop);
        }
        apply_live_update_to_alert(
            mtm,
            &active.alert,
            &active.target,
            active.progress_bar.as_ref(),
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
    progress: Option<super::TaskDialogProgressState>,
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    cancel_button_idx: Option<usize>,
    allow_cancellation: bool,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    verification_enabled: bool,
    critical: bool,
    sink: TaskDialogEventSink,
    cancelled: Option<&std::sync::atomic::AtomicBool>,
) -> super::TaskDialogOutcome {
    if cancelled.is_some_and(|c| c.load(Ordering::Acquire)) {
        if session_id != 0 {
            PENDING_TASK_DIALOG_UPDATES.with(|map| {
                map.borrow_mut().remove(&session_id);
            });
        }
        return super::TaskDialogOutcome::Quit;
    }
    let mtm = mtm();
    let app = NSApplication::sharedApplication(mtm);
    activate_app(&app);
    let a = NSAlert::new(mtm);
    a.setMessageText(&NSString::from_str(caption));
    a.setInformativeText(&NSString::from_str(text));
    a.setAlertStyle(if critical { NSAlertStyle::Critical } else { NSAlertStyle::Warning });
    let custom_default = default_button_idx.filter(|&idx| idx < buttons.len());
    let cancel_idx = cancel_button_idx.filter(|&idx| idx < buttons.len());
    let mut action_buttons: Vec<Retained<NSButton>> = Vec::with_capacity(buttons.len());
    let mut has_escape_button = false;
    for (idx, b) in buttons.iter().enumerate() {
        let btn = a.addButtonWithTitle(&NSString::from_str(b));
        let existing_eq = btn.keyEquivalent().to_string();
        if let Some(new_eq) =
            task_dialog_button_key_equivalent(idx, &existing_eq, custom_default, cancel_idx)
        {
            let eq = NSString::from_str(new_eq);
            let _: () = unsafe { msg_send![&*btn, setKeyEquivalent: &*eq] };
        }
        if btn.keyEquivalent().to_string() == "\u{1b}" {
            has_escape_button = true;
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
            cb.setEnabled(verification_enabled);
            if verification_enabled {
                let _: () = unsafe { msg_send![&*cb, setTarget: &*target] };
                let _: () = unsafe { msg_send![&*cb, setAction: sel!(verificationToggled:)] };
            }
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
    let progress_bar: Option<Retained<NSProgressIndicator>> = progress.map(|p| {
        let bar = NSProgressIndicator::initWithFrame(
            NSProgressIndicator::alloc(mtm),
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(320.0, 18.0)),
        );
        bar.setStyle(NSProgressIndicatorStyle::Bar);
        apply_progress_state_to_indicator(&bar, p);
        bar
    });
    let mut link_buttons: Vec<Retained<NSButton>> = Vec::with_capacity(links.len());
    rebuild_task_dialog_accessory_view(
        mtm,
        &a,
        &target,
        progress_bar.as_ref(),
        &radio_buttons,
        &mut link_buttons,
        links,
    );

    // Under `TDF_ALLOW_DIALOG_CANCELLATION`, allow Escape, Cmd+W, and the window's
    // title-bar close button to cancel the dialog with `IDCANCEL` even when no
    // `TDCBF_CANCEL_BUTTON` / Cancel button is configured. Also, when an `IDCANCEL`
    // button is the effective default button (so it carries Return `"\r"` instead of
    // Escape), attach a separate hidden Escape shortcut targeting that Cancel button so
    // Escape remains bound to `IDCANCEL` even without `TDF_ALLOW_DIALOG_CANCELLATION`.
    let mut cancel_shortcut_buttons: Vec<Retained<NSButton>> = Vec::new();
    let mut attached_close_delegate_win: Option<Retained<NSWindow>> = None;
    let need_hidden_escape =
        task_dialog_needs_hidden_escape_shortcut(allow_cancellation, cancel_idx, has_escape_button);
    if allow_cancellation || need_hidden_escape {
        let _: () = unsafe { msg_send![&*a, layout] };
        let win = a.window();
        if allow_cancellation {
            win.setStyleMask(win.styleMask() | NSWindowStyleMask::Closable);
            if win.delegate().is_none() {
                win.setDelegate(Some(ProtocolObject::from_ref(&*target)));
                attached_close_delegate_win = Some(win.clone());
            }
        }
        if let Some(cv) = win.contentView() {
            let add_hidden_shortcut = |key: &str,
                                       mods: Option<NSEventModifierFlags>,
                                       btn_target: &AnyObject,
                                       action: objc2::runtime::Sel| {
                let btn = unsafe {
                    NSButton::buttonWithTitle_target_action(
                        &NSString::from_str(""),
                        Some(btn_target),
                        Some(action),
                        mtm,
                    )
                };
                btn.setBordered(false);
                btn.setTransparent(true);
                btn.setFrame(NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(0.0, 0.0)));
                let eq = NSString::from_str(key);
                let _: () = unsafe { msg_send![&*btn, setKeyEquivalent: &*eq] };
                if let Some(m) = mods {
                    btn.setKeyEquivalentModifierMask(m);
                }
                cv.addSubview(&btn);
                btn
            };
            if need_hidden_escape {
                if allow_cancellation {
                    cancel_shortcut_buttons.push(add_hidden_shortcut(
                        "\u{1b}",
                        None,
                        &*target,
                        sel!(cancelDialog:),
                    ));
                } else if let Some(cidx) = cancel_idx {
                    if let Some(cancel_btn) = action_buttons.get(cidx) {
                        cancel_shortcut_buttons.push(add_hidden_shortcut(
                            "\u{1b}",
                            None,
                            &**cancel_btn,
                            sel!(performClick:),
                        ));
                    }
                }
            }
            if allow_cancellation {
                cancel_shortcut_buttons.push(add_hidden_shortcut(
                    "w",
                    Some(NSEventModifierFlags::Command),
                    &*target,
                    sel!(cancelDialog:),
                ));
            }
        }
    }

    struct ActiveAlertGuard {
        session_id: u64,
        mtm: MainThreadMarker,
        attached_delegate_win: Option<Retained<NSWindow>>,
    }
    impl Drop for ActiveAlertGuard {
        fn drop(&mut self) {
            if let Some(ref win) = self.attached_delegate_win {
                win.setDelegate(None);
            }
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
                    DeferredModalStop::Navigate => TASK_DIALOG_NAVIGATE_CODE,
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
            progress_bar,
            links: links.to_vec(),
            sink,
            deferred_stop: None,
        });
    });
    let _active_guard = ActiveAlertGuard {
        session_id,
        mtm,
        attached_delegate_win: attached_close_delegate_win,
    };

    // Apply any live updates that arrived before `run_task_dialog_alert` began.
    let mut pre_clicked: Option<usize> = None;
    let mut pre_navigated = false;
    if session_id != 0 {
        let queued = PENDING_TASK_DIALOG_UPDATES.with(|map| map.borrow_mut().remove(&session_id));
        if let Some(updates) = queued {
            ACTIVE_TASK_DIALOG_ALERTS.with(|stack| {
                if let Some(entry) = stack.borrow_mut().iter_mut().rev().find(|e| e.session_id == session_id) {
                    for upd in updates {
                        if upd.navigate_page {
                            pre_navigated = true;
                            break;
                        }
                        apply_live_update_to_alert(
                            mtm,
                            &entry.alert,
                            &entry.target,
                            entry.progress_bar.as_ref(),
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
        return super::TaskDialogOutcome::Quit;
    }
    if pre_navigated {
        return super::TaskDialogOutcome::Navigated;
    }

    let r = if let Some(bidx) = pre_clicked {
        1000 + bidx as isize
    } else {
        a.runModal()
    };
    let quit_requested = cancelled.is_some_and(|c| c.load(Ordering::Acquire))
        || crate::user32::message::has_pending_quit();
    if quit_requested || (r == TASK_DIALOG_ABORT_CODE && !allow_cancellation) {
        return super::TaskDialogOutcome::Quit;
    }
    if r == TASK_DIALOG_NAVIGATE_CODE {
        return super::TaskDialogOutcome::Navigated;
    }
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
    let _keep_cancel_shortcuts_alive = cancel_shortcut_buttons;
    let _keep_target_alive = target;
    if r == TASK_DIALOG_CANCEL_CODE || (r == TASK_DIALOG_ABORT_CODE && allow_cancellation) {
        return super::TaskDialogOutcome::Cancelled { radio_index, verified };
    }
    // NSAlertFirstButtonReturn = 1000
    let button_index = (r - 1000).clamp(0, buttons.len().saturating_sub(1) as isize) as usize;
    super::TaskDialogOutcome::Completed(super::TaskDialogAlertResult {
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
    progress: Option<super::TaskDialogProgressState>,
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    cancel_button_idx: Option<usize>,
    allow_cancellation: bool,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    verification_enabled: bool,
    critical: bool,
    tx: std::sync::mpsc::Sender<super::TaskDialogEvent>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    match run_task_dialog_alert(
        session_id,
        caption,
        text,
        links,
        progress,
        buttons,
        button_enabled,
        default_button_idx,
        cancel_button_idx,
        allow_cancellation,
        radios,
        radio_enabled,
        default_radio_idx,
        verification_text,
        verification_checked,
        verification_enabled,
        critical,
        TaskDialogEventSink::Channel(tx.clone()),
        Some(&cancelled),
    ) {
        super::TaskDialogOutcome::Completed(res) => {
            let _ = tx.send(super::TaskDialogEvent::ButtonClicked(res));
        }
        super::TaskDialogOutcome::Cancelled { radio_index, verified } => {
            let _ = tx.send(super::TaskDialogEvent::Cancelled { radio_index, verified });
        }
        super::TaskDialogOutcome::Navigated => {
            let _ = tx.send(super::TaskDialogEvent::Navigated);
        }
        super::TaskDialogOutcome::Quit | super::TaskDialogOutcome::Unavailable => {}
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
    progress: Option<super::TaskDialogProgressState>,
    buttons: &[String],
    button_enabled: &[bool],
    default_button_idx: Option<usize>,
    cancel_button_idx: Option<usize>,
    allow_cancellation: bool,
    radios: &[String],
    radio_enabled: &[bool],
    default_radio_idx: Option<usize>,
    verification_text: Option<&str>,
    verification_checked: bool,
    verification_enabled: bool,
    critical: bool,
    timer_origin: &mut std::time::Instant,
    timer_interval: Option<std::time::Duration>,
    on_event: &mut dyn FnMut(super::TaskDialogEvent),
    on_timer: &mut dyn FnMut(u32) -> bool,
) -> super::TaskDialogOutcome {
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
        progress,
        buttons,
        button_enabled,
        default_button_idx,
        cancel_button_idx,
        allow_cancellation,
        radios,
        radio_enabled,
        default_radio_idx,
        verification_text,
        verification_checked,
        verification_enabled,
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

// ---------------------------------------------------------------------------
// Native child controls (BUTTON / EDIT / STATIC)
// ---------------------------------------------------------------------------

struct ControlTargetIvar {
    hwnd: Cell<usize>,
    is_toggle: Cell<bool>,
    is_manual_toggle: Cell<bool>,
    stored_check: Cell<u32>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2ControlTarget"]
    #[ivars = ControlTargetIvar]
    struct AW2ControlTarget;

    unsafe impl NSObjectProtocol for AW2ControlTarget {}

    impl AW2ControlTarget {
        #[unsafe(method(buttonClicked:))]
        fn button_clicked(&self, sender: &NSButton) {
            let hwnd = self.ivars().hwnd.get();
            if self.ivars().is_manual_toggle.get() {
                // Restore the application-controlled Win32 state immediately so
                // clicking a manual BS_CHECKBOX / BS_3STATE / BS_RADIOBUTTON never
                // leaves the native button out of sync if the parent ignores BN_CLICKED.
                let cur = self.ivars().stored_check.get();
                sender.setState(ns_state_from_check(cur));
                controls::native_button_clicked(hwnd, Some(cur));
                return;
            }
            let check = if self.ivars().is_toggle.get() {
                let st = sender.state();
                let v = if st == NSControlStateValueOn {
                    controls::BST_CHECKED
                } else if st == NSControlStateValueMixed {
                    controls::BST_INDETERMINATE
                } else {
                    controls::BST_UNCHECKED
                };
                self.ivars().stored_check.set(v);
                Some(v)
            } else {
                None
            };
            controls::native_button_clicked(hwnd, check);
        }
    }
);

fn ns_state_from_check(state: u32) -> isize {
    match state & 0x3 {
        controls::BST_CHECKED => NSControlStateValueOn,
        controls::BST_INDETERMINATE => NSControlStateValueMixed,
        _ => NSControlStateValueOff,
    }
}

fn button_toggle_flags(style: u32) -> (bool, bool) {
    let btn_type = style & controls::BS_TYPEMASK;
    let is_auto = matches!(
        btn_type,
        controls::BS_AUTOCHECKBOX | controls::BS_AUTO3STATE | controls::BS_AUTORADIOBUTTON
    );
    let is_manual = matches!(
        btn_type,
        controls::BS_CHECKBOX | controls::BS_3STATE | controls::BS_RADIOBUTTON
    );
    (is_auto || is_manual, is_manual)
}

impl AW2ControlTarget {
    fn new(mtm: MainThreadMarker, hwnd: usize, style: u32) -> Retained<Self> {
        let (is_toggle, is_manual_toggle) = button_toggle_flags(style);
        let this = Self::alloc(mtm).set_ivars(ControlTargetIvar {
            hwnd: Cell::new(hwnd),
            is_toggle: Cell::new(is_toggle),
            is_manual_toggle: Cell::new(is_manual_toggle),
            stored_check: Cell::new(controls::BST_UNCHECKED),
        });
        unsafe { msg_send![super(this), init] }
    }
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AW2TextDelegate"]
    #[ivars = HwndIvar]
    struct AW2TextDelegate;

    unsafe impl NSObjectProtocol for AW2TextDelegate {}

    unsafe impl NSControlTextEditingDelegate for AW2TextDelegate {
        #[unsafe(method(controlTextDidBeginEditing:))]
        fn control_text_did_begin_editing(&self, _notification: &NSNotification) {
            let hwnd = self.ivars().hwnd.get();
            controls::native_control_focused(hwnd);
            let sel = CONTROLS.with(|m| {
                let map = m.borrow();
                let ctrl = map.get(&hwnd)?;
                if let NativeControlView::SingleLineEdit {
                    field,
                    programmatic,
                    last_sel,
                    pending_sel,
                    ..
                } = &ctrl.view
                {
                    let ed = field.currentEditor()?;
                    let len = ed.string().length();
                    if let Some((ps, pe)) = pending_sel.take() {
                        let lo = ps.min(pe).min(len);
                        let hi = ps.max(pe).min(len);
                        programmatic.set(true);
                        ed.setSelectedRange(NSRange::new(lo, hi - lo));
                        programmatic.set(false);
                        last_sel.set((lo, hi));
                        return Some((lo, hi));
                    }
                    let r = ed.selectedRange();
                    let lo = r.location.min(len);
                    let hi = r.location.saturating_add(r.length).min(len);
                    last_sel.set((lo, hi));
                    return Some((lo, hi));
                }
                None
            });
            if let Some((lo, hi)) = sel {
                controls::native_edit_selection_changed(hwnd, lo, hi);
            }
        }

        #[unsafe(method(controlTextDidChange:))]
        fn control_text_did_change(&self, _notification: &NSNotification) {
            enum EditChangeNotify {
                Text(String, usize, usize),
                Selection(usize, usize),
            }
            let hwnd = self.ivars().hwnd.get();
            let update = CONTROLS.with(|m| {
                let map = m.borrow();
                let ctrl = map.get(&hwnd)?;
                if let NativeControlView::SingleLineEdit {
                    field,
                    programmatic,
                    edit_style,
                    max_len,
                    last_text,
                    last_sel,
                    pending_sel,
                    ..
                } = &ctrl.view
                {
                    if programmatic.get() {
                        return None;
                    }
                    pending_sel.set(None);
                    let raw = field.stringValue().to_string();
                    let next_units: Vec<u16> = raw.encode_utf16().collect();
                    let ed = field.currentEditor();
                    let (lo, hi) = if let Some(ref e) = ed {
                        let r = e.selectedRange();
                        (r.location, r.location.saturating_add(r.length))
                    } else {
                        (next_units.len(), next_units.len())
                    };
                    let norm = controls::apply_native_edit_constraints(
                        &last_text.borrow(),
                        &next_units,
                        lo,
                        hi,
                        edit_style.get(),
                        max_len.get(),
                    );
                    let final_text = if norm.reconcile_native {
                        let s = String::from_utf16_lossy(&norm.units);
                        programmatic.set(true);
                        field.setStringValue(&NSString::from_str(&s));
                        if let Some(ref e) = ed {
                            e.setSelectedRange(NSRange::new(
                                norm.sel_start,
                                norm.sel_end.saturating_sub(norm.sel_start),
                            ));
                        }
                        programmatic.set(false);
                        s
                    } else {
                        raw
                    };
                    *last_text.borrow_mut() = norm.units;
                    let prev_sel = last_sel.replace((norm.sel_start, norm.sel_end));
                    if norm.text_changed {
                        return Some(EditChangeNotify::Text(final_text, norm.sel_start, norm.sel_end));
                    } else if prev_sel != (norm.sel_start, norm.sel_end) {
                        return Some(EditChangeNotify::Selection(norm.sel_start, norm.sel_end));
                    }
                }
                None
            });
            match update {
                Some(EditChangeNotify::Text(t, lo, hi)) => {
                    controls::native_edit_text_changed(hwnd, t, lo, hi);
                }
                Some(EditChangeNotify::Selection(lo, hi)) => {
                    controls::native_edit_selection_changed(hwnd, lo, hi);
                }
                None => {}
            }
        }
    }

    unsafe impl NSTextFieldDelegate for AW2TextDelegate {}

    unsafe impl NSTextDelegate for AW2TextDelegate {
        #[unsafe(method(textDidBeginEditing:))]
        fn text_did_begin_editing(&self, _notification: &NSNotification) {
            controls::native_control_focused(self.ivars().hwnd.get());
        }

        #[unsafe(method(textDidChange:))]
        fn text_did_change(&self, _notification: &NSNotification) {
            enum EditChangeNotify {
                Text(String, usize, usize),
                Selection(usize, usize),
            }
            let hwnd = self.ivars().hwnd.get();
            let update = CONTROLS.with(|m| {
                let map = m.borrow();
                let ctrl = map.get(&hwnd)?;
                if let NativeControlView::MultiLineEdit {
                    text_view,
                    programmatic,
                    edit_style,
                    max_len,
                    last_text,
                    last_sel,
                    ..
                } = &ctrl.view
                {
                    if programmatic.get() {
                        return None;
                    }
                    let raw = text_view.string().to_string();
                    let native_units: Vec<u16> = raw.encode_utf16().collect();
                    let r = text_view.selectedRange();
                    let (win32_next, win32_lo, win32_hi) = controls::native_newlines_to_win32(
                        &native_units,
                        r.location,
                        r.location.saturating_add(r.length),
                    );
                    let norm = controls::apply_native_edit_constraints(
                        &last_text.borrow(),
                        &win32_next,
                        win32_lo,
                        win32_hi,
                        edit_style.get(),
                        max_len.get(),
                    );
                    if norm.reconcile_native {
                        let (nat_units, nat_lo, nat_hi) = controls::win32_newlines_to_native(
                            &norm.units,
                            norm.sel_start,
                            norm.sel_end,
                        );
                        let nat_str = String::from_utf16_lossy(&nat_units);
                        programmatic.set(true);
                        text_view.setString(&NSString::from_str(&nat_str));
                        text_view.setSelectedRange(NSRange::new(
                            nat_lo,
                            nat_hi.saturating_sub(nat_lo),
                        ));
                        programmatic.set(false);
                    }
                    let final_win32 = if norm.text_changed {
                        Some(String::from_utf16_lossy(&norm.units))
                    } else {
                        None
                    };
                    *last_text.borrow_mut() = norm.units;
                    let prev_sel = last_sel.replace((norm.sel_start, norm.sel_end));
                    if let Some(win32_text) = final_win32 {
                        return Some(EditChangeNotify::Text(win32_text, norm.sel_start, norm.sel_end));
                    } else if prev_sel != (norm.sel_start, norm.sel_end) {
                        return Some(EditChangeNotify::Selection(norm.sel_start, norm.sel_end));
                    }
                }
                None
            });
            match update {
                Some(EditChangeNotify::Text(t, lo, hi)) => {
                    controls::native_edit_text_changed(hwnd, t, lo, hi);
                }
                Some(EditChangeNotify::Selection(lo, hi)) => {
                    controls::native_edit_selection_changed(hwnd, lo, hi);
                }
                None => {}
            }
        }
    }

    unsafe impl NSTextViewDelegate for AW2TextDelegate {
        #[unsafe(method(textViewDidChangeSelection:))]
        fn text_view_did_change_selection(&self, _notification: &NSNotification) {
            let hwnd = self.ivars().hwnd.get();
            let sel = CONTROLS.with(|m| {
                let map = m.borrow();
                let ctrl = map.get(&hwnd)?;
                if let NativeControlView::MultiLineEdit {
                    text_view,
                    programmatic,
                    last_text,
                    last_sel,
                    ..
                } = &ctrl.view
                {
                    if programmatic.get() {
                        return None;
                    }
                    let len = text_view.string().length();
                    let r = text_view.selectedRange();
                    let nat_lo = r.location.min(len);
                    let nat_hi = r.location.saturating_add(r.length).min(len);
                    let win32_units = last_text.borrow();
                    let lo = controls::native_offset_to_win32(&win32_units, nat_lo);
                    let hi = controls::native_offset_to_win32(&win32_units, nat_hi);
                    if last_sel.replace((lo, hi)) != (lo, hi) {
                        return Some((lo, hi));
                    }
                }
                None
            });
            if let Some((lo, hi)) = sel {
                controls::native_edit_selection_changed(hwnd, lo, hi);
            }
        }
    }
);

impl AW2TextDelegate {
    fn new(mtm: MainThreadMarker, hwnd: usize) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(HwndIvar { hwnd: Cell::new(hwnd) });
        unsafe { msg_send![super(this), init] }
    }
}

enum NativeControlView {
    Button {
        button: Retained<NSButton>,
        target: Retained<AW2ControlTarget>,
    },
    GroupBox {
        box_view: Retained<NSBox>,
    },
    SingleLineEdit {
        field: Retained<NSTextField>,
        _delegate: Retained<AW2TextDelegate>,
        programmatic: Cell<bool>,
        masked: Cell<bool>,
        edit_style: Cell<u32>,
        max_len: Cell<usize>,
        last_text: RefCell<Vec<u16>>,
        last_sel: Cell<(usize, usize)>,
        pending_sel: Cell<Option<(usize, usize)>>,
    },
    MultiLineEdit {
        scroll: Retained<NSScrollView>,
        text_view: Retained<NSTextView>,
        _delegate: Retained<AW2TextDelegate>,
        programmatic: Cell<bool>,
        readonly: Cell<bool>,
        enabled: Cell<bool>,
        edit_style: Cell<u32>,
        wants_return: Cell<bool>,
        max_len: Cell<usize>,
        last_text: RefCell<Vec<u16>>,
        last_sel: Cell<(usize, usize)>,
    },
    StaticLabel {
        label: Retained<NSTextField>,
        no_prefix: bool,
    },
    StaticIcon {
        image_view: Retained<NSImageView>,
    },
    StaticBox {
        box_view: Retained<NSBox>,
    },
}

impl NativeControlView {
    fn root_view(&self) -> &NSView {
        match self {
            Self::Button { button, .. } => button,
            Self::GroupBox { box_view } | Self::StaticBox { box_view } => box_view,
            Self::SingleLineEdit { field, .. } => field,
            Self::MultiLineEdit { scroll, .. } => scroll,
            Self::StaticLabel { label, .. } => label,
            Self::StaticIcon { image_view } => image_view,
        }
    }
}

struct NativeControl {
    parent_hwnd: usize,
    kind: NativeControlKind,
    notify_click: Cell<bool>,
    rect_px: Cell<RECT>,
    view: NativeControlView,
}

fn child_cocoa_rect(parent: &NativeWin, rect: &RECT) -> NSRect {
    let s = f64::from_bits(parent.shared.scale.load(Ordering::Acquire)).max(1.0);
    let cv_h = parent.view.bounds().size.height;
    let x = rect.left as f64 / s;
    let w = rect.width().max(0) as f64 / s;
    let h = rect.height().max(0) as f64 / s;
    let y = cv_h - (rect.top as f64 / s) - h;
    NSRect::new(NSPoint::new(x, y), NSSize::new(w, h))
}

fn reposition_child_controls(parent_hwnd: usize) {
    let items: Vec<(usize, RECT)> = CONTROLS.with(|m| {
        m.borrow()
            .iter()
            .filter(|(_, c)| c.parent_hwnd == parent_hwnd)
            .map(|(h, c)| (*h, c.rect_px.get()))
            .collect()
    });
    if items.is_empty() {
        return;
    }
    with_win(parent_hwnd, |parent| {
        CONTROLS.with(|m| {
            let map = m.borrow();
            for (ch, rect) in items {
                if let Some(ctrl) = map.get(&ch) {
                    let frame = child_cocoa_rect(parent, &rect);
                    ctrl.view.root_view().setFrame(frame);
                }
            }
        });
    });
}

fn apply_button_style(btn: &NSButton, style: u32) {
    let btn_type = style & controls::BS_TYPEMASK;
    match btn_type {
        controls::BS_CHECKBOX | controls::BS_AUTOCHECKBOX => {
            btn.setButtonType(NSButtonType::Switch);
            btn.setAllowsMixedState(false);
            btn.setKeyEquivalent(&NSString::from_str(""));
        }
        controls::BS_3STATE | controls::BS_AUTO3STATE => {
            btn.setButtonType(NSButtonType::Switch);
            btn.setAllowsMixedState(true);
            btn.setKeyEquivalent(&NSString::from_str(""));
        }
        controls::BS_RADIOBUTTON | controls::BS_AUTORADIOBUTTON => {
            btn.setButtonType(NSButtonType::Radio);
            btn.setAllowsMixedState(false);
            btn.setKeyEquivalent(&NSString::from_str(""));
        }
        _ => {
            btn.setButtonType(NSButtonType::MomentaryPushIn);
            btn.setBezelStyle(NSBezelStyle::Push);
            btn.setAllowsMixedState(false);
            if matches!(
                btn_type,
                controls::BS_DEFPUSHBUTTON | controls::BS_DEFSPLITBUTTON | controls::BS_DEFCOMMANDLINK
            ) {
                btn.setKeyEquivalent(&NSString::from_str("\r"));
            } else {
                btn.setKeyEquivalent(&NSString::from_str(""));
            }
        }
    }
}

fn ns_alignment_from_edit(style: u32) -> NSTextAlignment {
    match style & 0x3 {
        controls::ES_CENTER => NSTextAlignment::Center,
        controls::ES_RIGHT => NSTextAlignment::Right,
        _ => NSTextAlignment::Left,
    }
}

fn ns_alignment_from_static(style: u32) -> NSTextAlignment {
    match style & controls::SS_TYPEMASK {
        controls::SS_CENTER => NSTextAlignment::Center,
        controls::SS_RIGHT => NSTextAlignment::Right,
        _ => NSTextAlignment::Left,
    }
}

fn nsimage_from_icon_payload(payload: &IconPayload) -> Option<Retained<NSImage>> {
    match payload {
        IconPayload::Stock(id) => {
            let name = match *id {
                32513 | 32515 | 32518 => "NSCaution",       // IDI_ERROR / IDI_WARNING / IDI_SHIELD
                32514 | 32516 => "NSInfo",                  // IDI_QUESTION / IDI_INFORMATION
                _ => "NSApplicationIcon",                   // IDI_APPLICATION
            };
            NSImage::imageNamed(&NSString::from_str(name))
        }
        IconPayload::Png(bytes) => {
            let data = NSData::with_bytes(bytes);
            NSImage::initWithData(NSImage::alloc(), &data)
        }
        IconPayload::Bgra { width, height, pixels } => {
            let w = *width as usize;
            let h = *height as usize;
            if w == 0 || h == 0 || pixels.len() < w * h * 4 {
                return None;
            }
            let color_space = unsafe { NSCalibratedRGBColorSpace };
            let rep = unsafe {
                NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
                    NSBitmapImageRep::alloc(),
                    std::ptr::null_mut(),
                    w as isize,
                    h as isize,
                    8,
                    4,
                    true,
                    false,
                    color_space,
                    (w * 4) as isize,
                    32,
                )
            }?;
            let dst = rep.bitmapData();
            if dst.is_null() {
                return None;
            }
            // Convert BGRA -> RGBA into the bitmap backing store.
            unsafe {
                let dst_slice = std::slice::from_raw_parts_mut(dst, w * h * 4);
                for (d, s) in dst_slice.chunks_exact_mut(4).zip(pixels.chunks_exact(4)) {
                    d[0] = s[2];
                    d[1] = s[1];
                    d[2] = s[0];
                    d[3] = s[3];
                }
            }
            let img = NSImage::initWithSize(NSImage::alloc(), NSSize::new(w as f64, h as f64));
            img.addRepresentation(&rep);
            Some(img)
        }
    }
}

fn build_single_line_edit_field(
    mtm: MainThreadMarker,
    frame: NSRect,
    masked: bool,
    text: &str,
    style: u32,
    enabled: bool,
    visible: bool,
    delegate: &AW2TextDelegate,
) -> Retained<NSTextField> {
    let field: Retained<NSTextField> = if masked {
        let secure = NSSecureTextField::initWithFrame(NSSecureTextField::alloc(mtm), frame);
        Retained::into_super(secure)
    } else {
        NSTextField::initWithFrame(NSTextField::alloc(mtm), frame)
    };
    field.setStringValue(&NSString::from_str(text));
    field.setEditable((style & controls::ES_READONLY) == 0);
    field.setSelectable(true);
    field.setBordered(true);
    field.setBezeled(true);
    field.setDrawsBackground(true);
    field.setEnabled(enabled);
    field.setFont(Some(&NSFont::systemFontOfSize(13.0)));
    field.setAlignment(ns_alignment_from_edit(style));
    field.setWantsLayer(true);
    field.setHidden(!visible);
    unsafe { field.setDelegate(Some(ProtocolObject::from_ref(delegate))) };
    field
}

fn is_control_first_responder(ctrl: &NativeControl, win: &NSWindow) -> bool {
    let Some(cur) = win.firstResponder() else { return false };
    let cur_ptr = (&*cur as *const _) as usize;
    match &ctrl.view {
        NativeControlView::SingleLineEdit { field, .. } => {
            Retained::as_ptr(field) as usize == cur_ptr
                || field
                    .currentEditor()
                    .is_some_and(|ed| Retained::as_ptr(&ed) as usize == cur_ptr)
        }
        NativeControlView::MultiLineEdit { text_view, .. } => {
            Retained::as_ptr(text_view) as usize == cur_ptr
        }
        NativeControlView::Button { button, .. } => {
            Retained::as_ptr(button) as usize == cur_ptr
        }
        _ => false,
    }
}

pub(super) fn control_create(hwnd: usize, parent_hwnd: usize, desc: NativeControlDesc) {
    let mtm = mtm();
    let frame = match with_win(parent_hwnd, |p| {
        if !p.has_controls.replace(true) {
            p.layer.setOpaque(false);
            p.window.setBackgroundColor(Some(&NSColor::windowBackgroundColor()));
        }
        child_cocoa_rect(p, &desc.rect)
    }) {
        Some(f) => f,
        None => return,
    };

    let view = match desc.kind {
        NativeControlKind::Button => {
            let btn_type = desc.style & controls::BS_TYPEMASK;
            let title = controls::strip_win32_mnemonic(&desc.text);
            if btn_type == controls::BS_GROUPBOX {
                let box_view = NSBox::initWithFrame(NSBox::alloc(mtm), frame);
                box_view.setTitle(&NSString::from_str(&title));
                NativeControlView::GroupBox { box_view }
            } else {
                let button = NSButton::initWithFrame(NSButton::alloc(mtm), frame);
                apply_button_style(&button, desc.style);
                button.setTitle(&NSString::from_str(&title));
                button.setFont(Some(&NSFont::systemFontOfSize(13.0)));
                button.setEnabled(desc.enabled);
                let target = AW2ControlTarget::new(mtm, hwnd, desc.style);
                unsafe {
                    button.setTarget(Some(&*target));
                    button.setAction(Some(sel!(buttonClicked:)));
                }
                NativeControlView::Button { button, target }
            }
        }
        NativeControlKind::Edit => {
            let readonly = (desc.style & controls::ES_READONLY) != 0;
            let delegate = AW2TextDelegate::new(mtm, hwnd);
            if (desc.style & controls::ES_MULTILINE) != 0 {
                let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), frame);
                scroll.setHasVerticalScroller(
                    (desc.style & (ws::VSCROLL | controls::ES_AUTOVSCROLL)) != 0,
                );
                scroll.setHasHorizontalScroller((desc.style & ws::HSCROLL) != 0);
                scroll.setBorderType(NSBorderType::BezelBorder);

                let content_sz = scroll.contentSize();
                let tv_frame = NSRect::new(NSPoint::new(0.0, 0.0), content_sz);
                let text_view = NSTextView::initWithFrame(NSTextView::alloc(mtm), tv_frame);
                text_view.setEditable(!readonly && desc.enabled);
                text_view.setSelectable(true);
                text_view.setRichText(false);
                text_view.setFont(Some(&NSFont::systemFontOfSize(12.0)));
                text_view.setAlignment(ns_alignment_from_edit(desc.style));
                text_view.setVerticallyResizable(true);
                text_view.setHorizontallyResizable((desc.style & ws::HSCROLL) != 0);
                let win32_units: Vec<u16> = desc.text.encode_utf16().collect();
                let (nat_units, _, _) = controls::win32_newlines_to_native(&win32_units, 0, 0);
                text_view.setString(&NSString::from_str(&String::from_utf16_lossy(&nat_units)));
                text_view.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
                scroll.setDocumentView(Some(&text_view));
                NativeControlView::MultiLineEdit {
                    scroll,
                    text_view,
                    _delegate: delegate,
                    programmatic: Cell::new(false),
                    readonly: Cell::new(readonly),
                    enabled: Cell::new(desc.enabled),
                    edit_style: Cell::new(desc.style),
                    wants_return: Cell::new((desc.style & controls::ES_WANTRETURN) != 0),
                    max_len: Cell::new(controls::DEFAULT_EDIT_LIMIT),
                    last_text: RefCell::new(win32_units),
                    last_sel: Cell::new((0, 0)),
                }
            } else {
                let masked = (desc.style & controls::ES_PASSWORD) != 0;
                let field = build_single_line_edit_field(
                    mtm,
                    frame,
                    masked,
                    &desc.text,
                    desc.style,
                    desc.enabled,
                    desc.visible,
                    &delegate,
                );
                NativeControlView::SingleLineEdit {
                    field,
                    _delegate: delegate,
                    programmatic: Cell::new(false),
                    masked: Cell::new(masked),
                    edit_style: Cell::new(desc.style),
                    max_len: Cell::new(controls::DEFAULT_EDIT_LIMIT),
                    last_text: RefCell::new(desc.text.encode_utf16().collect()),
                    last_sel: Cell::new((0, 0)),
                    pending_sel: Cell::new(None),
                }
            }
        }
        NativeControlKind::Static => {
            let st_type = desc.style & controls::SS_TYPEMASK;
            match st_type {
                controls::SS_ICON | controls::SS_BITMAP => {
                    let image_view = NSImageView::initWithFrame(NSImageView::alloc(mtm), frame);
                    image_view.setImageScaling(NSImageScaling::ScaleProportionallyUpOrDown);
                    NativeControlView::StaticIcon { image_view }
                }
                controls::SS_ETCHEDHORZ | controls::SS_ETCHEDVERT | controls::SS_ETCHEDFRAME => {
                    let box_view = NSBox::initWithFrame(NSBox::alloc(mtm), frame);
                    box_view.setBoxType(NSBoxType::Separator);
                    NativeControlView::StaticBox { box_view }
                }
                _ => {
                    let no_prefix = (desc.style & controls::SS_NOPREFIX) != 0;
                    let text = if no_prefix {
                        desc.text.clone()
                    } else {
                        controls::strip_win32_mnemonic(&desc.text)
                    };
                    let label = if matches!(st_type, controls::SS_SIMPLE | controls::SS_LEFTNOWORDWRAP) {
                        NSTextField::labelWithString(&NSString::from_str(&text), mtm)
                    } else {
                        NSTextField::wrappingLabelWithString(&NSString::from_str(&text), mtm)
                    };
                    label.setFrame(frame);
                    label.setAlignment(ns_alignment_from_static(desc.style));
                    label.setFont(Some(&NSFont::systemFontOfSize(13.0)));
                    label.setEnabled(desc.enabled);
                    NativeControlView::StaticLabel { label, no_prefix }
                }
            }
        }
    };

    let root = view.root_view();
    root.setWantsLayer(true);
    root.setHidden(!desc.visible);
    with_win(parent_hwnd, |p| {
        p.view.addSubview(root);
    });

    let notify_click = desc.kind == NativeControlKind::Static && (desc.style & controls::SS_NOTIFY) != 0;
    CONTROLS.with(|m| {
        m.borrow_mut().insert(
            hwnd,
            NativeControl {
                parent_hwnd,
                kind: desc.kind,
                notify_click: Cell::new(notify_click),
                rect_px: Cell::new(desc.rect),
                view,
            },
        );
    });
}

pub(super) fn control_destroy(hwnd: usize) {
    let ctrl = CONTROLS.with(|m| m.borrow_mut().remove(&hwnd));
    if let Some(ctrl) = ctrl {
        match &ctrl.view {
            NativeControlView::Button { button, .. } => unsafe {
                button.setTarget(None);
                button.setAction(None);
            },
            NativeControlView::SingleLineEdit { field, .. } => unsafe {
                field.setDelegate(None);
            },
            NativeControlView::MultiLineEdit { text_view, .. } => {
                text_view.setDelegate(None);
            }
            _ => {}
        }
        with_win(ctrl.parent_hwnd, |p| {
            if is_control_first_responder(&ctrl, &p.window) {
                p.window.makeFirstResponder(Some(&p.view));
            }
            let any_left = CONTROLS.with(|m| m.borrow().values().any(|c| c.parent_hwnd == ctrl.parent_hwnd));
            if !any_left {
                p.has_controls.set(false);
            }
        });
        ctrl.view.root_view().removeFromSuperview();
    }
}

pub(super) fn control_set_frame(hwnd: usize, rect: RECT) {
    let parent_hwnd = CONTROLS.with(|m| {
        let map = m.borrow();
        let ctrl = map.get(&hwnd)?;
        ctrl.rect_px.set(rect);
        Some(ctrl.parent_hwnd)
    });
    if let Some(ph) = parent_hwnd {
        with_win(ph, |p| {
            let frame = child_cocoa_rect(p, &rect);
            CONTROLS.with(|m| {
                if let Some(ctrl) = m.borrow().get(&hwnd) {
                    ctrl.view.root_view().setFrame(frame);
                }
            });
        });
    }
}

pub(super) fn control_set_text(hwnd: usize, text: String) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::Button { button, .. } => {
                let s = controls::strip_win32_mnemonic(&text);
                button.setTitle(&NSString::from_str(&s));
            }
            NativeControlView::GroupBox { box_view } => {
                let s = controls::strip_win32_mnemonic(&text);
                box_view.setTitle(&NSString::from_str(&s));
            }
            NativeControlView::SingleLineEdit { field, programmatic, last_text, .. } => {
                *last_text.borrow_mut() = text.encode_utf16().collect();
                programmatic.set(true);
                field.setStringValue(&NSString::from_str(&text));
                programmatic.set(false);
            }
            NativeControlView::MultiLineEdit { text_view, programmatic, last_text, .. } => {
                let win32_units: Vec<u16> = text.encode_utf16().collect();
                let (nat_units, _, _) = controls::win32_newlines_to_native(&win32_units, 0, 0);
                *last_text.borrow_mut() = win32_units;
                programmatic.set(true);
                text_view.setString(&NSString::from_str(&String::from_utf16_lossy(&nat_units)));
                programmatic.set(false);
            }
            NativeControlView::StaticLabel { label, no_prefix } => {
                let s = if *no_prefix { text } else { controls::strip_win32_mnemonic(&text) };
                label.setStringValue(&NSString::from_str(&s));
            }
            NativeControlView::StaticIcon { .. } | NativeControlView::StaticBox { .. } => {}
        }
    });
}

pub(super) fn control_set_visible(hwnd: usize, visible: bool) {
    if !visible {
        control_unfocus(hwnd);
    }
    CONTROLS.with(|m| {
        if let Some(ctrl) = m.borrow().get(&hwnd) {
            ctrl.view.root_view().setHidden(!visible);
        }
    });
}

pub(super) fn control_set_enabled(hwnd: usize, enabled: bool) {
    if !enabled {
        control_unfocus(hwnd);
    }
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::Button { button, .. } => button.setEnabled(enabled),
            NativeControlView::SingleLineEdit { field, .. } => field.setEnabled(enabled),
            NativeControlView::MultiLineEdit { text_view, readonly, enabled: en_cell, .. } => {
                en_cell.set(enabled);
                text_view.setEditable(enabled && !readonly.get());
            }
            NativeControlView::StaticLabel { label, .. } => label.setEnabled(enabled),
            _ => {}
        }
    });
}

pub(super) fn control_set_readonly(hwnd: usize, readonly: bool) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::SingleLineEdit { field, edit_style, .. } => {
                let mut st = edit_style.get();
                if readonly {
                    st |= controls::ES_READONLY;
                } else {
                    st &= !controls::ES_READONLY;
                }
                edit_style.set(st);
                field.setEditable(!readonly);
            }
            NativeControlView::MultiLineEdit {
                text_view,
                readonly: ro_cell,
                enabled,
                edit_style,
                ..
            } => {
                let mut st = edit_style.get();
                if readonly {
                    st |= controls::ES_READONLY;
                } else {
                    st &= !controls::ES_READONLY;
                }
                edit_style.set(st);
                ro_cell.set(readonly);
                text_view.setEditable(enabled.get() && !readonly);
            }
            _ => {}
        }
    });
}

pub(super) fn control_set_selection(hwnd: usize, start: usize, end: usize) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::SingleLineEdit {
                field,
                programmatic,
                last_sel,
                pending_sel,
                ..
            } => {
                let len = field.stringValue().length();
                let lo = start.min(end).min(len);
                let hi = start.max(end).min(len);
                last_sel.set((lo, hi));
                if let Some(ed) = field.currentEditor() {
                    pending_sel.set(None);
                    programmatic.set(true);
                    ed.setSelectedRange(NSRange::new(lo, hi - lo));
                    programmatic.set(false);
                } else {
                    pending_sel.set(Some((lo, hi)));
                }
            }
            NativeControlView::MultiLineEdit {
                text_view,
                programmatic,
                last_text,
                last_sel,
                ..
            } => {
                let win32_units = last_text.borrow();
                let win32_len = win32_units.len();
                let lo = start.min(end).min(win32_len);
                let hi = start.max(end).min(win32_len);
                last_sel.set((lo, hi));
                let nat_len = text_view.string().length();
                let nat_lo = controls::win32_offset_to_native(&win32_units, lo).min(nat_len);
                let nat_hi = controls::win32_offset_to_native(&win32_units, hi).min(nat_len);
                programmatic.set(true);
                text_view.setSelectedRange(NSRange::new(nat_lo, nat_hi.saturating_sub(nat_lo)));
                programmatic.set(false);
            }
            _ => {}
        }
    });
}

pub(super) fn control_set_limit(hwnd: usize, limit: usize) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::SingleLineEdit { max_len, .. }
            | NativeControlView::MultiLineEdit { max_len, .. } => {
                max_len.set(limit);
            }
            _ => {}
        }
    });
}

pub(super) fn control_focus(hwnd: usize) {
    enum FocusTarget {
        SingleLine(Retained<NSTextField>),
        MultiLine(Retained<NSTextView>),
        Button(Retained<NSButton>),
        ResignWindow,
    }
    let parent_hwnd = CONTROLS.with(|m| m.borrow().get(&hwnd).map(|c| c.parent_hwnd));
    let Some(ph) = parent_hwnd else { return };
    with_win(ph, |p| {
        let target = CONTROLS.with(|m| {
            let map = m.borrow();
            let ctrl = map.get(&hwnd)?;
            if is_control_first_responder(ctrl, &p.window) {
                return None;
            }
            match &ctrl.view {
                NativeControlView::SingleLineEdit { field, .. } => {
                    Some(FocusTarget::SingleLine(field.clone()))
                }
                NativeControlView::MultiLineEdit { text_view, .. } => {
                    Some(FocusTarget::MultiLine(text_view.clone()))
                }
                NativeControlView::Button { button, .. } => {
                    Some(FocusTarget::Button(button.clone()))
                }
                _ => {
                    // Focusing a non-editable native control (e.g. STATIC or GroupBox)
                    // should resign any previously focused native editor in this window.
                    let has_focused = map.values().any(|c| {
                        c.parent_hwnd == ph && is_control_first_responder(c, &p.window)
                    });
                    has_focused.then_some(FocusTarget::ResignWindow)
                }
            }
        });
        match target {
            Some(FocusTarget::SingleLine(field)) => {
                p.window.makeFirstResponder(Some(&field));
                if let Some(ed) = field.currentEditor() {
                    CONTROLS.with(|m| {
                        if let Some(ctrl) = m.borrow().get(&hwnd) {
                            if let NativeControlView::SingleLineEdit {
                                programmatic,
                                last_sel,
                                pending_sel,
                                ..
                            } = &ctrl.view
                            {
                                if let Some((ps, pe)) = pending_sel.take() {
                                    let len = ed.string().length();
                                    let lo = ps.min(pe).min(len);
                                    let hi = ps.max(pe).min(len);
                                    last_sel.set((lo, hi));
                                    programmatic.set(true);
                                    ed.setSelectedRange(NSRange::new(lo, hi - lo));
                                    programmatic.set(false);
                                }
                            }
                        }
                    });
                }
            }
            Some(FocusTarget::MultiLine(text_view)) => {
                p.window.makeFirstResponder(Some(&text_view));
            }
            Some(FocusTarget::Button(button)) => {
                p.window.makeFirstResponder(Some(&button));
            }
            Some(FocusTarget::ResignWindow) => {
                p.window.makeFirstResponder(Some(&p.view));
            }
            None => {}
        }
    });
}

pub(super) fn control_unfocus(hwnd: usize) {
    let parent_hwnd = CONTROLS.with(|m| m.borrow().get(&hwnd).map(|c| c.parent_hwnd));
    let Some(ph) = parent_hwnd else { return };
    with_win(ph, |p| {
        let is_focused = CONTROLS.with(|m| {
            m.borrow()
                .get(&hwnd)
                .is_some_and(|ctrl| is_control_first_responder(ctrl, &p.window))
        });
        if is_focused {
            p.window.makeFirstResponder(Some(&p.view));
        }
    });
}

pub(super) fn window_clear_control_focus(top_hwnd: usize) {
    with_win(top_hwnd, |p| {
        let has_focused_ctrl = CONTROLS.with(|m| {
            m.borrow()
                .values()
                .any(|ctrl| ctrl.parent_hwnd == top_hwnd && is_control_first_responder(ctrl, &p.window))
        });
        if has_focused_ctrl {
            p.window.makeFirstResponder(Some(&p.view));
        }
    });
}

pub(super) fn control_set_password_mode(hwnd: usize, masked: bool) {
    let mtm = mtm();
    struct SwapSnapshot {
        parent_hwnd: usize,
        frame: NSRect,
        text: String,
        text_units: Vec<u16>,
        sel: (usize, usize),
        style: u32,
        enabled: bool,
        visible: bool,
        delegate: Retained<AW2TextDelegate>,
        old_field: Retained<NSTextField>,
    }

    let snap = CONTROLS.with(|m| {
        let map = m.borrow();
        let ctrl = map.get(&hwnd)?;
        let NativeControlView::SingleLineEdit {
            field,
            _delegate,
            masked: cur_masked,
            edit_style,
            last_sel,
            pending_sel,
            ..
        } = &ctrl.view
        else {
            return None;
        };
        if cur_masked.get() == masked {
            return None;
        }
        let ed = field.currentEditor();
        let text = ed
            .as_ref()
            .map(|e| e.string().to_string())
            .unwrap_or_else(|| field.stringValue().to_string());
        let text_units: Vec<u16> = text.encode_utf16().collect();
        let len = text_units.len();
        let (raw_lo, raw_hi) = if let Some(ref e) = ed {
            let r = e.selectedRange();
            (r.location, r.location.saturating_add(r.length))
        } else if let Some((ps, pe)) = pending_sel.get() {
            (ps, pe)
        } else {
            last_sel.get()
        };
        let lo = raw_lo.min(raw_hi).min(len);
        let hi = raw_lo.max(raw_hi).min(len);
        let mut style = edit_style.get();
        if masked {
            style |= controls::ES_PASSWORD;
        } else {
            style &= !controls::ES_PASSWORD;
        }
        if field.isEditable() {
            style &= !controls::ES_READONLY;
        } else {
            style |= controls::ES_READONLY;
        }
        Some(SwapSnapshot {
            parent_hwnd: ctrl.parent_hwnd,
            frame: field.frame(),
            text,
            text_units,
            sel: (lo, hi),
            style,
            enabled: field.isEnabled(),
            visible: !field.isHidden(),
            delegate: _delegate.clone(),
            old_field: field.clone(),
        })
    });

    let Some(snap) = snap else { return };

    let was_focused = with_win(snap.parent_hwnd, |p| {
        CONTROLS.with(|m| {
            m.borrow()
                .get(&hwnd)
                .is_some_and(|ctrl| is_control_first_responder(ctrl, &p.window))
        })
    })
    .unwrap_or(false);

    // Detach delegate from old field before removing it so ending editing does
    // not emit stale notifications.
    unsafe { snap.old_field.setDelegate(None) };

    let new_field = build_single_line_edit_field(
        mtm,
        snap.frame,
        masked,
        &snap.text,
        snap.style,
        snap.enabled,
        snap.visible,
        &snap.delegate,
    );

    with_win(snap.parent_hwnd, |p| {
        let attached = unsafe { snap.old_field.superview().is_some() };
        if attached {
            p.view.replaceSubview_with(&snap.old_field, &new_field);
        } else {
            p.view.addSubview(&new_field);
        }
    });

    CONTROLS.with(|m| {
        if let Some(ctrl) = m.borrow_mut().get_mut(&hwnd) {
            if let NativeControlView::SingleLineEdit {
                field,
                masked: cur_masked,
                edit_style,
                last_text,
                last_sel,
                pending_sel,
                ..
            } = &mut ctrl.view
            {
                *field = new_field.clone();
                cur_masked.set(masked);
                edit_style.set(snap.style);
                *last_text.borrow_mut() = snap.text_units;
                last_sel.set(snap.sel);
                pending_sel.set(Some(snap.sel));
            }
        }
    });

    if was_focused {
        with_win(snap.parent_hwnd, |p| {
            if snap.visible && snap.enabled {
                p.window.makeFirstResponder(Some(&new_field));
            } else {
                p.window.makeFirstResponder(Some(&p.view));
            }
        });
        if snap.visible && snap.enabled {
            if let Some(ed) = new_field.currentEditor() {
                let (lo, hi) = snap.sel;
                CONTROLS.with(|m| {
                    if let Some(ctrl) = m.borrow().get(&hwnd) {
                        if let NativeControlView::SingleLineEdit {
                            programmatic,
                            last_sel,
                            pending_sel,
                            ..
                        } = &ctrl.view
                        {
                            pending_sel.set(None);
                            last_sel.set((lo, hi));
                            programmatic.set(true);
                            ed.setSelectedRange(NSRange::new(lo, hi.saturating_sub(lo)));
                            programmatic.set(false);
                        }
                    }
                });
            }
        }
    }
}

pub(super) fn control_set_check(hwnd: usize, state: u32) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        if let NativeControlView::Button { button, target } = &ctrl.view {
            let clamped = state & 0x3;
            target.ivars().stored_check.set(clamped);
            button.setState(ns_state_from_check(clamped));
        }
    });
}

pub(super) fn control_set_style(hwnd: usize, style: u32) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        match &ctrl.view {
            NativeControlView::Button { button, target } => {
                let (is_toggle, is_manual) = button_toggle_flags(style);
                target.ivars().is_toggle.set(is_toggle);
                target.ivars().is_manual_toggle.set(is_manual);
                apply_button_style(button, style);
            }
            NativeControlView::SingleLineEdit { field, edit_style, .. } => {
                edit_style.set(style);
                field.setEditable((style & controls::ES_READONLY) == 0);
                field.setAlignment(ns_alignment_from_edit(style));
            }
            NativeControlView::MultiLineEdit {
                text_view,
                readonly,
                enabled,
                edit_style,
                wants_return,
                ..
            } => {
                let ro = (style & controls::ES_READONLY) != 0;
                readonly.set(ro);
                edit_style.set(style);
                wants_return.set((style & controls::ES_WANTRETURN) != 0);
                text_view.setEditable(enabled.get() && !ro);
                text_view.setAlignment(ns_alignment_from_edit(style));
            }
            NativeControlView::StaticLabel { label, .. } => {
                ctrl.notify_click.set((style & controls::SS_NOTIFY) != 0);
                label.setAlignment(ns_alignment_from_static(style));
            }
            NativeControlView::StaticIcon { .. } | NativeControlView::StaticBox { .. } => {
                ctrl.notify_click.set((style & controls::SS_NOTIFY) != 0);
            }
            _ => {}
        }
    });
}

pub(super) fn control_set_icon(hwnd: usize, payload: Option<IconPayload>) {
    CONTROLS.with(|m| {
        let map = m.borrow();
        let Some(ctrl) = map.get(&hwnd) else { return };
        if let NativeControlView::StaticIcon { image_view } = &ctrl.view {
            let img = payload.as_ref().and_then(nsimage_from_icon_payload);
            image_view.setImage(img.as_deref());
        }
    });
}

/// If the event targets one of our windows whose AppKit `firstResponder` is a
/// native `EDIT` control (an `NSTextView` or `NSTextField` field editor),
/// returns `Some((edit_hwnd, wants_return))`.
pub(super) fn focused_native_edit_info(ev: &NSEvent, mtm: MainThreadMarker) -> Option<(usize, bool)> {
    let win = ev
        .window(mtm)
        .or_else(|| NSApplication::sharedApplication(mtm).keyWindow())?;
    let parent_hwnd = hwnd_for_window(&win)?;
    let resp = win.firstResponder()?;
    let resp_ptr = (&*resp as *const _) as usize;
    CONTROLS.with(|m| {
        for (&ch, ctrl) in m.borrow().iter() {
            if ctrl.parent_hwnd != parent_hwnd || ctrl.kind != NativeControlKind::Edit {
                continue;
            }
            match &ctrl.view {
                NativeControlView::MultiLineEdit { text_view, wants_return, .. } => {
                    if Retained::as_ptr(text_view) as usize == resp_ptr {
                        return Some((ch, wants_return.get()));
                    }
                }
                NativeControlView::SingleLineEdit { field, .. } => {
                    if Retained::as_ptr(field) as usize == resp_ptr
                        || field
                            .currentEditor()
                            .is_some_and(|ed| Retained::as_ptr(&ed) as usize == resp_ptr)
                    {
                        return Some((ch, false));
                    }
                }
                _ => {}
            }
        }
        None
    })
}

/// After forwarding a key/mouse event to AppKit, synchronize the focused native
/// `EDIT` control's live UTF-16 selection range back to Win32 if it changed.
pub(super) fn sync_focused_edit_selection(ev: &NSEvent, mtm: MainThreadMarker) {
    let Some((edit_hwnd, _)) = focused_native_edit_info(ev, mtm) else {
        return;
    };
    let changed = CONTROLS.with(|m| {
        let map = m.borrow();
        let ctrl = map.get(&edit_hwnd)?;
        match &ctrl.view {
            NativeControlView::SingleLineEdit {
                field,
                programmatic,
                last_sel,
                ..
            } => {
                if programmatic.get() {
                    return None;
                }
                let ed = field.currentEditor()?;
                let len = ed.string().length();
                let r = ed.selectedRange();
                let lo = r.location.min(len);
                let hi = r.location.saturating_add(r.length).min(len);
                if last_sel.replace((lo, hi)) != (lo, hi) {
                    return Some((lo, hi));
                }
                None
            }
            NativeControlView::MultiLineEdit {
                text_view,
                programmatic,
                last_text,
                last_sel,
                ..
            } => {
                if programmatic.get() {
                    return None;
                }
                let len = text_view.string().length();
                let r = text_view.selectedRange();
                let nat_lo = r.location.min(len);
                let nat_hi = r.location.saturating_add(r.length).min(len);
                let win32_units = last_text.borrow();
                let lo = controls::native_offset_to_win32(&win32_units, nat_lo);
                let hi = controls::native_offset_to_win32(&win32_units, nat_hi);
                if last_sel.replace((lo, hi)) != (lo, hi) {
                    return Some((lo, hi));
                }
                None
            }
            _ => None,
        }
    });
    if let Some((lo, hi)) = changed {
        controls::native_edit_selection_changed(edit_hwnd, lo, hi);
    }
}

/// True when a wheel event lands on a visible, enabled scrollable native child
/// control (`MultiLineEdit` / `NSScrollView`) so AppKit handles scrolling only
/// for that control while non-scrollable areas keep Win32 `WM_MOUSEWHEEL`.
pub(super) fn wheel_targets_native_control(ev: &NSEvent, mtm: MainThreadMarker) -> bool {
    let Some(win) = ev.window(mtm) else { return false };
    let Some(parent_hwnd) = hwnd_for_window(&win) else { return false };
    if !with_win(parent_hwnd, |w| w.has_controls.get()).unwrap_or(false) {
        return false;
    }
    let loc = ev.locationInWindow();
    CONTROLS.with(|m| {
        m.borrow().values().any(|c| {
            if c.parent_hwnd != parent_hwnd {
                return false;
            }
            match &c.view {
                NativeControlView::MultiLineEdit { scroll, enabled, .. } => {
                    enabled.get() && !scroll.isHidden() && {
                        let f = scroll.frame();
                        loc.x >= f.origin.x
                            && loc.x <= f.origin.x + f.size.width
                            && loc.y >= f.origin.y
                            && loc.y <= f.origin.y + f.size.height
                    }
                }
                _ => false,
            }
        })
    })
}

/// If a left mouse-down lands on a visible `SS_NOTIFY` `STATIC` control, queue `STN_CLICKED`.
pub(super) fn notify_static_click_if_hit(ev: &NSEvent, mtm: MainThreadMarker) {
    let Some(win) = ev.window(mtm) else { return };
    let Some(parent_hwnd) = hwnd_for_window(&win) else { return };
    if !with_win(parent_hwnd, |w| w.has_controls.get()).unwrap_or(false) {
        return;
    }
    let loc = ev.locationInWindow();
    let hit = CONTROLS.with(|m| {
        m.borrow()
            .iter()
            .find(|(_, c)| {
                c.parent_hwnd == parent_hwnd
                    && c.notify_click.get()
                    && !c.view.root_view().isHidden()
                    && {
                        let f = c.view.root_view().frame();
                        loc.x >= f.origin.x
                            && loc.x <= f.origin.x + f.size.width
                            && loc.y >= f.origin.y
                            && loc.y <= f.origin.y + f.size.height
                    }
            })
            .map(|(h, _)| *h)
    });
    if let Some(h) = hit {
        controls::native_static_clicked(h);
    }
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

    #[test]
    fn task_dialog_default_button_preserves_cancel_escape_shortcut() {
        // OK (IDOK) + Cancel (IDCANCEL) with explicit nDefaultButton = IDOK (idx 0):
        // OK keeps Return, Cancel keeps AppKit's automatic Escape ("\u{1b}").
        assert_eq!(
            task_dialog_button_key_equivalent(0, "\r", Some(0), Some(1)),
            Some("\r")
        );
        assert_eq!(
            task_dialog_button_key_equivalent(1, "\u{1b}", Some(0), Some(1)),
            None
        );

        // Localized "Abbrechen" with IDCANCEL (idx 2) and explicit nDefaultButton = IDNO (idx 1):
        // Yes loses Return, No gets Return, Abbrechen receives Escape even though its caption != "Cancel".
        assert_eq!(
            task_dialog_button_key_equivalent(0, "\r", Some(1), Some(2)),
            Some("")
        );
        assert_eq!(
            task_dialog_button_key_equivalent(1, "", Some(1), Some(2)),
            Some("\r")
        );
        assert_eq!(
            task_dialog_button_key_equivalent(2, "", Some(1), Some(2)),
            Some("\u{1b}")
        );

        // Localized "Abbrechen" with IDCANCEL (idx 1) and NO explicit default button:
        // idx 0 keeps Return, idx 1 gets Escape.
        assert_eq!(task_dialog_button_key_equivalent(0, "\r", None, Some(1)), None);
        assert_eq!(task_dialog_button_key_equivalent(1, "", None, Some(1)), Some("\u{1b}"));

        // Cancel placed first (idx 0, AppKit defaulted it to "\r") with nDefaultButton = IDOK (idx 1):
        // Cancel moves from Return to Escape, OK gets Return.
        assert_eq!(
            task_dialog_button_key_equivalent(0, "\r", Some(1), Some(0)),
            Some("\u{1b}")
        );
        assert_eq!(
            task_dialog_button_key_equivalent(1, "", Some(1), Some(0)),
            Some("\r")
        );

        // Unrelated custom button titled "Cancel" (nButtonID != IDCANCEL, so cancel_idx = None):
        // AppKit's title-based Escape binding must be stripped so Escape does not click it.
        assert_eq!(task_dialog_button_key_equivalent(0, "\r", None, None), None);
        assert_eq!(task_dialog_button_key_equivalent(1, "\u{1b}", None, None), Some(""));
        assert_eq!(task_dialog_button_key_equivalent(1, "\u{1b}", Some(0), Some(2)), Some(""));
        assert_eq!(task_dialog_button_key_equivalent(2, "", Some(0), Some(2)), Some("\u{1b}"));

        // Localized IDCANCEL button also chosen as default (nDefaultButton = IDCANCEL, idx 1)
        // with TDF_ALLOW_DIALOG_CANCELLATION unset: Cancel keeps Return AND requires a separate
        // hidden Escape shortcut targeting Cancel so Escape still cancels.
        assert_eq!(task_dialog_button_key_equivalent(0, "\r", Some(1), Some(1)), Some(""));
        assert_eq!(task_dialog_button_key_equivalent(1, "", Some(1), Some(1)), Some("\r"));
        assert!(task_dialog_needs_hidden_escape_shortcut(false, Some(1), false));
        // When Cancel is idx 0 and default is implicit (None), idx 0 keeps Return so hidden Escape is also needed.
        assert_eq!(task_dialog_button_key_equivalent(0, "\r", None, Some(0)), None);
        assert!(task_dialog_needs_hidden_escape_shortcut(false, Some(0), false));
        // When a visible Cancel button already carries Escape, no hidden Escape shortcut is needed.
        assert!(!task_dialog_needs_hidden_escape_shortcut(false, Some(1), true));
        assert!(!task_dialog_needs_hidden_escape_shortcut(true, Some(1), true));
        // Without an IDCANCEL button, hidden Escape is only installed under TDF_ALLOW_DIALOG_CANCELLATION.
        assert!(!task_dialog_needs_hidden_escape_shortcut(false, None, false));
        assert!(task_dialog_needs_hidden_escape_shortcut(true, None, false));
    }
}
