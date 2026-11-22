//! iOS / UIKit host-surface backend for `user32` and `dxgi`.
//!
//! On iOS, `UIApplicationMain` owns the process main thread and presents a
//! fullscreen `UIView` backed by a `CAMetalLayer`. Instead of creating desktop
//! `NSWindow`s, the Swift/Objective-C host app calls [`attach_host_surface`]
//! once its `CAMetalLayer` is ready, [`resize_host_surface`] on layout or
//! orientation changes, and [`touch_event`] / [`key_event`] for input.
//!
//! The surface registry and touch/key bridges are compiled on both macOS and
//! iOS so the lifecycle can be unit-tested directly on macOS without a
//! simulator; iOS-only `UIKit` (`UIScreen`, `UIPasteboard`) calls are gated
//! behind `#[cfg(target_os = "ios")]`.

use super::{display, NativeDesc, WindowShared};
use crate::user32::input::{self, MouseButton, NativeKey, NativeMouse};
use crate::user32::window::{self, NativeEvent};
use crate::user32::{POINT, RECT};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// Touch event phases passed from the iOS host (`UITouch.Phase`).
pub const TOUCH_BEGAN: u32 = 0;
pub const TOUCH_MOVED: u32 = 1;
pub const TOUCH_ENDED: u32 = 2;
pub const TOUCH_CANCELLED: u32 = 3;

/// Mouse button ids passed from the iOS touch/trackpad host.
pub const MOUSE_LEFT: u32 = 0;
pub const MOUSE_RIGHT: u32 = 1;
pub const MOUSE_MIDDLE: u32 = 2;

/// Host callback invoked when a Windows game sets XInput slot-0 rumble motors
/// (`left` = low-frequency 0..65535, `right` = high-frequency 0..65535) and no
/// rumble-capable physical controller owns slot 0 (routes to iPhone CoreHaptics
/// Taptic Engine via `PhoneHaptics.swift`).
pub type RumbleCallback = extern "C" fn(u16, u16);

static RUMBLE_CB: Mutex<Option<RumbleCallback>> = Mutex::new(None);

/// Register (or clear with `None`) the host phone-haptics rumble callback.
pub fn set_rumble_callback(cb: Option<RumbleCallback>) {
    let prev = {
        let mut slot = RUMBLE_CB.lock().unwrap();
        let prev = *slot;
        *slot = cb;
        prev
    };
    if prev.is_some() && cb.is_none() {
        // Ensure the previous callback stops any ongoing Taptic Engine vibration.
        if let Some(f) = prev {
            f(0, 0);
        }
    }
    log::info!(
        "uikit: phone haptics rumble passthrough {}",
        if cb.is_some() { "enabled" } else { "disabled" }
    );
}

/// True when the host has registered a phone-haptics rumble callback (e.g. iPhone Taptic Engine).
pub fn has_rumble_callback() -> bool {
    RUMBLE_CB.lock().unwrap().is_some()
}

/// Dispatch XInput motor speeds `(left, right)` (0..65535) to the registered host rumble callback.
pub fn dispatch_host_rumble(left: u16, right: u16) {
    let cb = *RUMBLE_CB.lock().unwrap();
    if let Some(f) = cb {
        f(left, right);
    }
}

/// Native iOS hardware + software keyboard presence state reported by `KeyboardBridge.swift`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyboardState {
    pub hardware_connected: bool,
    pub software_visible: bool,
    pub software_height_px: u32,
}

#[derive(Default)]
struct HostSurfaceState {
    layer: usize,
    /// Raw physical UIView pixel dimensions reported by `MetalSurfaceView`.
    raw_width_px: u32,
    raw_height_px: u32,
    raw_scale: f64,
    /// Logical Win32 display dimensions (normalized/capped via `display::normalize_host_surface_size`).
    width_px: u32,
    height_px: u32,
    scale: f64,
    /// Active DXGI swapchain backbuffer dimensions (for exact `.resizeAspect` touch mapping).
    swapchain_px: Option<(u32, u32)>,
    active_hwnd: usize,
    windows: Vec<usize>,
    last_touch_px: Option<(f32, f32)>,
    keyboard: KeyboardState,
}

fn state() -> &'static Mutex<HostSurfaceState> {
    static S: OnceLock<Mutex<HostSurfaceState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HostSurfaceState::default()))
}

/// Record the active DXGI swapchain dimensions so physical UIKit touch coordinates map
/// accurately through `CALayerContentsGravity.resizeAspect` into Win32 client space.
pub fn set_active_swapchain_size(w: u32, h: u32) {
    if w > 0 && h > 0 {
        state().lock().unwrap().swapchain_px = Some((w, h));
    }
}

/// Map a physical surface touch coordinate `(x_px, y_px)` inside `(raw_w, raw_h)` through
/// `CALayerContentsGravity.resizeAspect` into active Win32 client/swapchain space `(dst_w, dst_h)`.
pub fn map_surface_point(
    x_px: f32,
    y_px: f32,
    raw_w: u32,
    raw_h: u32,
    dst_w: u32,
    dst_h: u32,
) -> (f32, f32) {
    if raw_w == 0 || raw_h == 0 || dst_w == 0 || dst_h == 0 || (raw_w == dst_w && raw_h == dst_h) {
        return (x_px, y_px);
    }
    let vw = raw_w as f32;
    let vh = raw_h as f32;
    let cw = dst_w as f32;
    let ch = dst_h as f32;
    let scale = (vw / cw).min(vh / ch);
    if !scale.is_finite() || scale <= 0.0 {
        return (x_px, y_px);
    }
    let rendered_w = cw * scale;
    let rendered_h = ch * scale;
    let off_x = (vw - rendered_w) * 0.5;
    let off_y = (vh - rendered_h) * 0.5;
    let mx = ((x_px - off_x) / scale).clamp(0.0, (cw - 1.0).max(0.0));
    let my = ((y_px - off_y) / scale).clamp(0.0, (ch - 1.0).max(0.0));
    (mx, my)
}

/// Register the host `UIView`'s `CAMetalLayer*` and initial drawable size in
/// physical pixels.
///
/// Updates the display cache, binds any already-created top-level Win32 HWNDs
/// to the layer, activates the platform backend on iOS, and starts
/// `GameController.framework` monitoring.
pub fn attach_host_surface(layer: *mut c_void, width_px: u32, height_px: u32, scale: f64) {
    let raw_w = width_px.max(1);
    let raw_h = height_px.max(1);
    let raw_s = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let (w, h, s) = display::normalize_host_surface_size(raw_w, raw_h, raw_s);

    let refresh_hz = {
        #[cfg(target_os = "ios")]
        {
            query_screen_refresh_hz()
        }
        #[cfg(not(target_os = "ios"))]
        {
            60
        }
    };
    display::update_from_host_surface(w, h, s, refresh_hz);

    let to_notify: Vec<usize> = {
        let mut st = state().lock().unwrap();
        st.layer = layer as usize;
        st.raw_width_px = raw_w;
        st.raw_height_px = raw_h;
        st.raw_scale = raw_s;
        st.width_px = w;
        st.height_px = h;
        st.scale = s;
        st.windows.clone()
    };

    let client = RECT::new(0, 0, w as i32, h as i32);
    for hwnd in to_notify {
        let shared = Arc::new(WindowShared {
            layer: layer as usize,
            client_px: AtomicU64::new((w as u64) | ((h as u64) << 32)),
            scale: AtomicU64::new(s.to_bits()),
            occluded: AtomicBool::new(false),
        });
        super::register_shared(hwnd, shared);
        window::native_event(hwnd, NativeEvent::Frame { client });
        window::native_event(hwnd, NativeEvent::Activated(true));
    }

    #[cfg(target_os = "ios")]
    {
        super::mark_active(true);
        super::gamepad::start();
    }

    log::info!(
        "uikit: attached host CAMetalLayer {:p} (raw {raw_w}x{raw_h}@{raw_s:.2} -> logical {w}x{h}@{s:.2}, {refresh_hz} Hz)",
        layer
    );
}

/// Update the host surface pixel dimensions (e.g. orientation change, split-view
/// resize, external display switch) and deliver `WM_SIZE` to attached HWNDs.
pub fn resize_host_surface(width_px: u32, height_px: u32, scale: f64) {
    let raw_w = width_px.max(1);
    let raw_h = height_px.max(1);
    let raw_s = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let (w, h, s) = display::normalize_host_surface_size(raw_w, raw_h, raw_s);

    let (changed, to_notify) = {
        let mut st = state().lock().unwrap();
        let changed = st.width_px != w || st.height_px != h || (st.scale - s).abs() > 1e-4;
        st.raw_width_px = raw_w;
        st.raw_height_px = raw_h;
        st.raw_scale = raw_s;
        st.width_px = w;
        st.height_px = h;
        st.scale = s;
        (changed, st.windows.clone())
    };
    if !changed {
        return;
    }

    let cur_hz = display::monitors().first().map_or(60, |m| m.refresh_hz);
    display::update_from_host_surface(w, h, s, cur_hz);

    let client = RECT::new(0, 0, w as i32, h as i32);
    for hwnd in to_notify {
        if let Some(shared) = super::window_shared(hwnd) {
            shared.set_client_size(w as i32, h as i32);
            shared.scale.store(s.to_bits(), Ordering::Release);
        }
        window::native_event(hwnd, NativeEvent::Frame { client });
    }
}

/// Detach the host `CAMetalLayer*` when the iOS view tears down or moves to background.
pub fn detach_host_surface() {
    let to_notify: Vec<usize> = {
        let mut st = state().lock().unwrap();
        st.layer = 0;
        st.last_touch_px = None;
        st.windows.clone()
    };
    for hwnd in to_notify {
        if let Some(shared) = super::window_shared(hwnd) {
            shared.occluded.store(true, Ordering::Release);
        }
        window::native_event(hwnd, NativeEvent::Activated(false));
    }
    dispatch_host_rumble(0, 0);
    log::info!("uikit: detached host CAMetalLayer");
}

/// Forward a single-finger iOS touch event (in physical surface pixels) into
/// `user32` mouse input (`WM_MOUSEMOVE` + `WM_LBUTTONDOWN` / `WM_LBUTTONUP`).
pub fn touch_event(phase: u32, x_px: f32, y_px: f32) {
    let (hwnd, mx, my, dx, dy) = {
        let mut st = state().lock().unwrap();
        let hwnd = if st.active_hwnd != 0 {
            st.active_hwnd
        } else {
            st.windows.last().copied().unwrap_or(0)
        };
        let (dst_w, dst_h) = st.swapchain_px.unwrap_or((st.width_px, st.height_px));
        let (mx, my) = map_surface_point(x_px, y_px, st.raw_width_px, st.raw_height_px, dst_w, dst_h);
        let raw_s = st.raw_scale.max(st.scale).max(1.0);
        let (dx, dy) = match (phase, st.last_touch_px) {
            (TOUCH_MOVED, Some((lx, ly))) => ((x_px - lx) as f64 / raw_s, (y_px - ly) as f64 / raw_s),
            _ => (0.0, 0.0),
        };
        match phase {
            TOUCH_BEGAN | TOUCH_MOVED => st.last_touch_px = Some((x_px, y_px)),
            _ => st.last_touch_px = None,
        }
        (hwnd, mx, my, dx, dy)
    };
    if hwnd == 0 {
        return;
    }
    let pt = POINT { x: mx.round() as i32, y: my.round() as i32 };
    match phase {
        TOUCH_BEGAN => {
            input::native_mouse(NativeMouse::Move { hwnd, client: Some(pt), screen: pt, dx: 0.0, dy: 0.0 });
            input::native_mouse(NativeMouse::Button {
                hwnd,
                client: Some(pt),
                screen: pt,
                button: MouseButton::Left,
                down: true,
                clicks: 1,
            });
        }
        TOUCH_MOVED => {
            input::native_mouse(NativeMouse::Move { hwnd, client: Some(pt), screen: pt, dx, dy });
        }
        TOUCH_ENDED | TOUCH_CANCELLED => {
            input::native_mouse(NativeMouse::Move { hwnd, client: Some(pt), screen: pt, dx: 0.0, dy: 0.0 });
            input::native_mouse(NativeMouse::Button {
                hwnd,
                client: Some(pt),
                screen: pt,
                button: MouseButton::Left,
                down: false,
                clicks: 1,
            });
        }
        _ => {}
    }
}

/// Forward a hardware/software keyboard event (`mac_keycode` = Apple HID/virtual key code)
/// into `user32::input::native_key`.
pub fn key_event(mac_keycode: u16, down: bool, repeat: bool, utf16_char: u16) {
    let chars = if down && utf16_char != 0 { vec![utf16_char] } else { Vec::new() };
    input::native_key(NativeKey { mac: mac_keycode, down, repeat, chars });
}

/// Forward Unicode text typed via the native iOS software keyboard (`UIKeyInput`) into Win32.
pub fn text_input(text: &str) {
    if !text.is_empty() {
        input::native_text(text);
    }
}

/// Forward an explicit mouse button press/release (left/right/middle) at physical surface pixels.
pub fn mouse_button_event(button: u32, down: bool, x_px: f32, y_px: f32) {
    let (hwnd, mx, my) = {
        let st = state().lock().unwrap();
        let hwnd = if st.active_hwnd != 0 {
            st.active_hwnd
        } else {
            st.windows.last().copied().unwrap_or(0)
        };
        let (dst_w, dst_h) = st.swapchain_px.unwrap_or((st.width_px, st.height_px));
        let (mx, my) = map_surface_point(x_px, y_px, st.raw_width_px, st.raw_height_px, dst_w, dst_h);
        (hwnd, mx, my)
    };
    if hwnd == 0 {
        return;
    }
    let btn = match button {
        MOUSE_RIGHT => MouseButton::Right,
        MOUSE_MIDDLE => MouseButton::Middle,
        _ => MouseButton::Left,
    };
    let pt = POINT { x: mx.round() as i32, y: my.round() as i32 };
    input::native_mouse(NativeMouse::Move { hwnd, client: Some(pt), screen: pt, dx: 0.0, dy: 0.0 });
    input::native_mouse(NativeMouse::Button {
        hwnd,
        client: Some(pt),
        screen: pt,
        button: btn,
        down,
        clicks: 1,
    });
}

/// Forward a 2-finger touch scroll gesture (in Win32 wheel units: +/-120 per detent)
/// at physical surface coordinates into `WM_MOUSEWHEEL` / `WM_MOUSEHWHEEL`.
pub fn scroll_wheel_event(x_px: f32, y_px: f32, delta_y: i32, delta_x: i32) {
    if delta_y == 0 && delta_x == 0 {
        return;
    }
    let (hwnd, mx, my) = {
        let st = state().lock().unwrap();
        let hwnd = if st.active_hwnd != 0 {
            st.active_hwnd
        } else {
            st.windows.last().copied().unwrap_or(0)
        };
        let (dst_w, dst_h) = st.swapchain_px.unwrap_or((st.width_px, st.height_px));
        let (mx, my) = map_surface_point(x_px, y_px, st.raw_width_px, st.raw_height_px, dst_w, dst_h);
        (hwnd, mx, my)
    };
    if hwnd == 0 {
        return;
    }
    let pt = POINT { x: mx.round() as i32, y: my.round() as i32 };
    input::native_mouse(NativeMouse::Move { hwnd, client: Some(pt), screen: pt, dx: 0.0, dy: 0.0 });
    if delta_y != 0 {
        input::native_mouse(NativeMouse::Wheel { screen: pt, delta: delta_y, horizontal: false });
    }
    if delta_x != 0 {
        input::native_mouse(NativeMouse::Wheel { screen: pt, delta: delta_x, horizontal: true });
    }
}

/// Update native hardware/software keyboard detection state from the iOS UIKit host.
pub fn set_keyboard_state(hardware_connected: bool, software_visible: bool, software_height_px: u32) {
    let next = KeyboardState {
        hardware_connected,
        software_visible,
        software_height_px: if software_visible { software_height_px } else { 0 },
    };
    let prev = {
        let mut st = state().lock().unwrap();
        let prev = st.keyboard;
        st.keyboard = next;
        prev
    };
    if prev != next {
        log::info!(
            "uikit: native keyboard state changed (hardware={}, software_visible={}, height_px={})",
            next.hardware_connected,
            next.software_visible,
            next.software_height_px
        );
    }
}

/// Snapshot of the currently detected native iOS hardware & software keyboard state.
pub fn keyboard_state() -> KeyboardState {
    state().lock().unwrap().keyboard
}

/// Update the virtual XInput touch-overlay gamepad state from the iOS UIKit overlay.
pub fn virtual_gamepad_state(connected: bool, pad: crate::misc::xinput1_4::Gamepad) {
    crate::misc::xinput1_4::set_virtual_gamepad(connected, pad);
}

/// Bind a Win32 top-level `hwnd` to the attached host surface (called by
/// `macos::window_create` on iOS, and testable directly on macOS).
#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_create(hwnd: usize, desc: NativeDesc) -> bool {
    let (layer, w, h, scale) = {
        let mut st = state().lock().unwrap();
        if !st.windows.contains(&hwnd) {
            st.windows.push(hwnd);
        }
        if st.active_hwnd == 0 {
            st.active_hwnd = hwnd;
        }
        let w = if st.width_px > 0 { st.width_px } else { desc.client.width().max(1) as u32 };
        let h = if st.height_px > 0 { st.height_px } else { desc.client.height().max(1) as u32 };
        let scale = if st.scale > 0.0 { st.scale } else { 1.0 };
        (st.layer, w, h, scale)
    };
    let shared = Arc::new(WindowShared {
        layer,
        client_px: AtomicU64::new((w as u64) | ((h as u64) << 32)),
        scale: AtomicU64::new(scale.to_bits()),
        occluded: AtomicBool::new(layer == 0),
    });
    super::register_shared(hwnd, shared);
    if layer != 0 {
        let client = RECT::new(0, 0, w as i32, h as i32);
        window::native_event(hwnd, NativeEvent::Frame { client });
    }
    true
}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_destroy(hwnd: usize) {
    super::unregister_shared(hwnd);
    let mut st = state().lock().unwrap();
    st.windows.retain(|&h| h != hwnd);
    if st.active_hwnd == hwnd {
        st.active_hwnd = st.windows.last().copied().unwrap_or(0);
    }
}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_show(hwnd: usize, show: bool, activate: bool) {
    if show && activate {
        window_activate(hwnd);
    }
}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_activate(hwnd: usize) {
    let has_layer = {
        let mut st = state().lock().unwrap();
        st.active_hwnd = hwnd;
        st.layer != 0
    };
    if has_layer {
        window::native_event(hwnd, NativeEvent::Activated(true));
    }
}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_set_frame(_hwnd: usize, _client: RECT) {
    // Fullscreen UIKit host controls the drawable frame; ignore desktop move/resize requests.
}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_set_style(_hwnd: usize, _desc: NativeDesc) {}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_minimize(_hwnd: usize, _minimize: bool) {}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn window_set_title(_hwnd: usize, _title: String) {}

#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub(super) fn alert(caption: &str, text: &str, _buttons: &[String], critical: bool) -> Option<usize> {
    if critical {
        log::error!("uikit alert [{caption}]: {text}");
    } else {
        log::warn!("uikit alert [{caption}]: {text}");
    }
    None
}

// ---------------------------------------------------------------------------
// iOS-only UIKit bridges (UIScreen, UIPasteboard) via objc2::msg_send!
// ---------------------------------------------------------------------------

#[cfg(target_os = "ios")]
pub(super) fn init() {
    display::refresh_ios();
    super::gamepad::start();
}

#[cfg(target_os = "ios")]
fn query_screen_refresh_hz() -> u32 {
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject};
    unsafe {
        let Some(cls) = AnyClass::get(c"UIScreen") else {
            return 60;
        };
        let screen: *mut AnyObject = msg_send![cls, mainScreen];
        if screen.is_null() {
            return 60;
        }
        let fps: isize = msg_send![screen, maximumFramesPerSecond];
        if fps > 0 { fps as u32 } else { 60 }
    }
}

#[cfg(target_os = "ios")]
fn general_pasteboard() -> Option<*mut objc2::runtime::AnyObject> {
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject};
    unsafe {
        let cls = AnyClass::get(c"UIPasteboard")?;
        let pb: *mut AnyObject = msg_send![cls, generalPasteboard];
        (!pb.is_null()).then_some(pb)
    }
}

#[cfg(target_os = "ios")]
pub(super) fn clipboard_get_text() -> Option<String> {
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2_foundation::NSString;
    unsafe {
        let pb = general_pasteboard()?;
        let s: Option<Retained<NSString>> = msg_send![pb, string];
        s.map(|ns| ns.to_string())
    }
}

#[cfg(target_os = "ios")]
pub(super) fn clipboard_set_text(text: &str) -> Option<isize> {
    use objc2::msg_send;
    use objc2_foundation::NSString;
    unsafe {
        let pb = general_pasteboard()?;
        let ns = NSString::from_str(text);
        let _: () = msg_send![pb, setString: &*ns];
        let count: isize = msg_send![pb, changeCount];
        Some(count)
    }
}

#[cfg(target_os = "ios")]
pub(super) fn clipboard_clear() -> isize {
    use objc2::msg_send;
    use objc2_foundation::NSString;
    unsafe {
        let Some(pb) = general_pasteboard() else {
            return 0;
        };
        let empty = NSString::from_str("");
        let _: () = msg_send![pb, setString: &*empty];
        let count: isize = msg_send![pb, changeCount];
        count
    }
}

#[cfg(target_os = "ios")]
pub(super) fn clipboard_change_count() -> isize {
    use objc2::msg_send;
    unsafe {
        let Some(pb) = general_pasteboard() else {
            return 0;
        };
        let count: isize = msg_send![pb, changeCount];
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_surface_attach_bind_resize_detach_lifecycle() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let fake_layer = 0xDEAD_BEEFusize as *mut c_void;
        let hwnd = 0x0009_1050usize;

        attach_host_surface(fake_layer, 2532, 1170, 3.0);
        let desc = NativeDesc {
            client: RECT::new(0, 0, 1280, 720),
            style: 0,
            exstyle: 0,
            title: "iOS Test".into(),
            topmost: false,
            dblclks: false,
        };
        assert!(window_create(hwnd, desc));
        assert_eq!(super::super::metal_layer_for_hwnd(hwnd), Some(fake_layer));
        let sh = super::super::window_shared(hwnd).expect("shared registered");
        assert_eq!(sh.client_size(), (2532, 1170));
        assert!(!sh.occluded.load(Ordering::Acquire));

        resize_host_surface(1170, 2532, 3.0);
        assert_eq!(sh.client_size(), (1170, 2532));

        detach_host_surface();
        assert!(sh.occluded.load(Ordering::Acquire));

        window_destroy(hwnd);
        assert!(super::super::window_shared(hwnd).is_none());
    }

    #[test]
    fn native_keyboard_state_and_virtual_gamepad_roundtrip() {
        set_keyboard_state(true, false, 0);
        assert_eq!(
            keyboard_state(),
            KeyboardState { hardware_connected: true, software_visible: false, software_height_px: 0 }
        );
        set_keyboard_state(false, true, 912);
        assert_eq!(
            keyboard_state(),
            KeyboardState { hardware_connected: false, software_visible: true, software_height_px: 912 }
        );
        set_keyboard_state(false, false, 0);
        assert_eq!(keyboard_state(), KeyboardState::default());

        let pad = crate::misc::xinput1_4::Gamepad {
            buttons: crate::misc::xinput1_4::btn::A | crate::misc::xinput1_4::btn::DPAD_UP,
            left_trigger: 128,
            right_trigger: 255,
            thumb_lx: 16384,
            thumb_ly: -16384,
            thumb_rx: 0,
            thumb_ry: 32767,
        };
        virtual_gamepad_state(true, pad);
        let got = crate::misc::xinput1_4::virtual_gamepad_state().expect("virtual pad connected");
        assert_eq!(got.0, pad);
        virtual_gamepad_state(false, crate::misc::xinput1_4::Gamepad::default());
        assert!(crate::misc::xinput1_4::virtual_gamepad_state().is_none());
    }

    #[test]
    fn scroll_wheel_event_posts_wheel_messages_to_active_hwnd() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        std::thread::spawn(|| {
            let cls = crate::strings::to_wide_z("UikitScrollTestWnd");
            let wc = crate::user32::class::WNDCLASSEX {
                cb_size: std::mem::size_of::<crate::user32::class::WNDCLASSEX>() as u32,
                style: 0,
                wndproc: crate::user32::defproc::DefWindowProcW as *const () as usize,
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
            let hwnd = unsafe {
                let _ = crate::user32::class::RegisterClassExW(&wc);
                crate::user32::window::CreateWindowExW(
                    0,
                    cls.as_ptr(),
                    std::ptr::null(),
                    0,
                    0,
                    0,
                    800,
                    600,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };
            assert!(!hwnd.is_null());
            let hwnd_u = hwnd as usize;
            let desc = NativeDesc {
                client: RECT::new(0, 0, 800, 600),
                style: 0,
                exstyle: 0,
                title: "Scroll".into(),
                topmost: false,
                dblclks: false,
            };
            assert!(window_create(hwnd_u, desc));
            window_activate(hwnd_u);
            unsafe {
                let _ = crate::user32::input::SetFocus(hwnd);
            }

            scroll_wheel_event(400.0, 300.0, -240, 120);

            let mut saw_vwheel = None;
            let mut saw_hwheel = None;
            let mut msg = crate::user32::MSG::default();
            unsafe {
                for _ in 0..16 {
                    if crate::user32::message::PeekMessageW(
                        &mut msg,
                        hwnd,
                        crate::user32::wm::MOUSEWHEEL,
                        crate::user32::wm::MOUSEHWHEEL,
                        1,
                    ) == 0
                    {
                        break;
                    }
                    if msg.message == crate::user32::wm::MOUSEWHEEL {
                        saw_vwheel = Some((msg.wparam >> 16) as i16);
                    } else if msg.message == crate::user32::wm::MOUSEHWHEEL {
                        saw_hwheel = Some((msg.wparam >> 16) as i16);
                    }
                }
                let _ = crate::user32::window::DestroyWindow(hwnd);
            }
            window_destroy(hwnd_u);
            assert_eq!(saw_vwheel, Some(-240));
            assert_eq!(saw_hwheel, Some(120));
        })
        .join()
        .unwrap();
    }

    #[test]
    fn map_surface_point_handles_identity_downscale_and_letterbox() {
        // Identity
        assert_eq!(map_surface_point(100.0, 200.0, 1600, 736, 1600, 736), (100.0, 200.0));
        // Uniform downscale 2868x1320 -> 1600x736 (center maps to center)
        let (cx, cy) = map_surface_point(1434.0, 660.0, 2868, 1320, 1600, 736);
        assert!((cx - 800.0).abs() < 1.0);
        assert!((cy - 368.0).abs() < 1.0);
        // Letterboxed 16:9 (1280x720) inside 2868x1320: center maps to (640, 360), left bar clamps to 0
        let (lx, ly) = map_surface_point(1434.0, 660.0, 2868, 1320, 1280, 720);
        assert!((lx - 640.0).abs() < 1.0);
        assert!((ly - 360.0).abs() < 1.0);
        let (bar_x, _) = map_surface_point(10.0, 660.0, 2868, 1320, 1280, 720);
        assert_eq!(bar_x, 0.0);
    }
}
