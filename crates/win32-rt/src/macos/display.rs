//! Display cache: monitors, per-monitor backing scale, title-bar height.
//!
//! Filled on the AppKit main thread (at init and on
//! `applicationDidChangeScreenParameters:`); Win32 threads only read it, so
//! they never call AppKit/CoreGraphics (see the threading rules in
//! [`super`]). Without a main loop (tests, `--no-run`, `PORT_HEADLESS=1` /
//! `AW2_HEADLESS=1`) a single 1920x1080 monitor at scale 1.0 (96 DPI) is
//! reported.

use crate::user32::{POINT, RECT};
use std::sync::RwLock;

#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    /// CGDirectDisplayID (stable key for HMONITOR).
    pub id: u32,
    pub primary: bool,
    /// Monitor rect in Win32 pixels (primary top-left = 0,0; y down).
    pub rect: RECT,
    /// Work area (minus menu bar and Dock), Win32 pixels.
    pub work: RECT,
    /// Per-monitor backing scale factor (e.g. 2.0 on Retina, 1.0 on 1x external).
    pub scale_factor: f64,
    /// Monitor frame in Cocoa global points `(x, y_bottom, width, height)`.
    pub cocoa_frame: (f64, f64, f64, f64),
    pub refresh_hz: u32,
    pub name: String,
}

impl Monitor {
    /// Effective DPI for this monitor (`(scale_factor * 96.0).round()`),
    /// honoring an explicit `PORT_DPI` / `AW2_DPI` override if configured.
    pub fn dpi(&self) -> u32 {
        if let Some(override_dpi) = env_dpi_override() {
            return override_dpi;
        }
        (self.scale_factor.max(0.5) * 96.0).round().max(48.0) as u32
    }
}

fn env_dpi_override() -> Option<u32> {
    std::env::var("PORT_DPI")
        .or_else(|_| std::env::var("AW2_DPI"))
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|&d| d >= 48)
}

/// Raw per-screen inputs (Cocoa points + backing scale) used to build a
/// [`Displays`] snapshot. Kept pure so mixed-DPI layouts can be unit-tested
/// deterministically without AppKit.
#[derive(Clone, Debug)]
pub struct ScreenDesc {
    pub id: u32,
    /// Cocoa global frame `(x, y_bottom, width, height)` in points.
    pub frame_pt: (f64, f64, f64, f64),
    /// Cocoa visible frame `(x, y_bottom, width, height)` in points.
    pub visible_pt: (f64, f64, f64, f64),
    /// Physical backing scale factor (`NSScreen.backingScaleFactor`).
    pub backing_scale: f64,
    pub refresh_hz: u32,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Displays {
    pub monitors: Vec<Monitor>,
    /// Primary display backing scale (Win32 pixels per point on primary, or 1.0).
    pub scale: f64,
    /// Primary display height in points (Cocoa <-> top-left y flip).
    pub primary_h_pt: f64,
    /// Title-bar height of a standard titled NSWindow, points.
    pub titlebar_pt: f64,
}

impl Displays {
    fn headless() -> Displays {
        let r = RECT::new(0, 0, 1920, 1080);
        Displays {
            monitors: vec![Monitor {
                id: 1,
                primary: true,
                rect: r,
                work: r,
                scale_factor: 1.0,
                cocoa_frame: (0.0, 0.0, 1920.0, 1080.0),
                refresh_hz: 60,
                name: "Headless".into(),
            }],
            scale: 1.0,
            primary_h_pt: 1080.0,
            titlebar_pt: 28.0,
        }
    }
}

/// Build a [`Displays`] configuration from Cocoa screen descriptors, giving
/// each monitor its own `scale_factor` and exact native pixel dimensions while
/// keeping adjacent mixed-DPI monitors flush without gaps or overlaps.
pub fn build_displays(screens: &[ScreenDesc], titlebar_pt: f64, retina: bool) -> Displays {
    if screens.is_empty() {
        return Displays::headless();
    }
    let p0 = &screens[0];
    let (_px0, _py0, primary_w_pt, primary_h_pt) = p0.frame_pt;
    let primary_scale = if retina { p0.backing_scale.max(1.0) } else { 1.0 };
    let primary_w_px = (primary_w_pt * primary_scale).round() as i32;
    let primary_h_px = (primary_h_pt * primary_scale).round() as i32;

    let mut monitors = Vec::with_capacity(screens.len());
    for (i, s) in screens.iter().enumerate() {
        let scale_i = if retina { s.backing_scale.max(1.0) } else { 1.0 };
        let (fx, fy, fw, fh) = s.frame_pt;
        let (vx, vy, vw, vh) = s.visible_pt;
        let top_pt = primary_h_pt - (fy + fh);
        let vis_top_pt = primary_h_pt - (vy + vh);

        let w_px = (fw * scale_i).round() as i32;
        let h_px = (fh * scale_i).round() as i32;

        let (left, top) = if i == 0 {
            (0, 0)
        } else {
            let left = if fx >= primary_w_pt {
                primary_w_px + ((fx - primary_w_pt) * scale_i).round() as i32
            } else if fx + fw <= 0.0 {
                ((fx + fw) * scale_i).round() as i32 - w_px
            } else {
                (fx * primary_scale).round() as i32
            };
            let top = if top_pt >= primary_h_pt {
                primary_h_px + ((top_pt - primary_h_pt) * scale_i).round() as i32
            } else if top_pt + fh <= 0.0 {
                ((top_pt + fh) * scale_i).round() as i32 - h_px
            } else {
                (top_pt * primary_scale).round() as i32
            };
            (left, top)
        };

        let rect = RECT::new(left, top, left + w_px, top + h_px);
        let work_left = left + ((vx - fx) * scale_i).round() as i32;
        let work_top = top + ((vis_top_pt - top_pt) * scale_i).round() as i32;
        let work = RECT::new(
            work_left,
            work_top,
            work_left + (vw * scale_i).round() as i32,
            work_top + (vh * scale_i).round() as i32,
        );

        monitors.push(Monitor {
            id: s.id,
            primary: i == 0,
            rect,
            work,
            scale_factor: scale_i,
            cocoa_frame: s.frame_pt,
            refresh_hz: if s.refresh_hz == 0 { 120 } else { s.refresh_hz },
            name: s.name.clone(),
        });
    }

    Displays {
        monitors,
        scale: primary_scale,
        primary_h_pt,
        titlebar_pt: if titlebar_pt >= 0.0 { titlebar_pt } else { 28.0 },
    }
}

static CACHE: RwLock<Option<Displays>> = RwLock::new(None);

/// Snapshot of the display configuration.
pub fn get() -> Displays {
    if let Some(d) = CACHE.read().unwrap().as_ref() {
        return d.clone();
    }
    Displays::headless()
}

/// Primary monitor's backing scale factor.
pub fn scale() -> f64 {
    CACHE.read().unwrap().as_ref().map(|d| d.scale).unwrap_or(1.0)
}

pub fn monitors() -> Vec<Monitor> {
    get().monitors
}

pub fn titlebar_height_pt() -> f64 {
    CACHE.read().unwrap().as_ref().map(|d| d.titlebar_pt).unwrap_or(28.0)
}

pub fn primary_height_pt() -> f64 {
    CACHE.read().unwrap().as_ref().map(|d| d.primary_h_pt).unwrap_or(1080.0)
}

/// Retina policy switch: `PORT_RETINA=0` / `AW2_RETINA=0` makes one Win32 pixel one point.
pub(crate) fn retina_enabled() -> bool {
    let val = std::env::var("PORT_RETINA").or_else(|_| std::env::var("AW2_RETINA"));
    !matches!(val.as_deref(), Ok("0") | Ok("false") | Ok("off"))
}

/// Default maximum long-edge resolution in pixels for embedded iOS displays.
///
/// On 3x Retina iPhones (e.g. `2868x1320` = 3.79 Mpx), capping the logical Win32
/// display resolution to 1600 px on the long edge (`1600x736` = 1.18 Mpx, preserving
/// exact native aspect ratio) cuts G-buffer/RT/TAA/upscaler unified-memory usage by
/// >3x while keeping native full-bleed crispness on a phone screen. Override with
/// `PORT_IOS_MAX_WIDTH` / `PORT_MAX_DISPLAY_WIDTH` (set `0` for uncapped native) or
/// lock an exact size with `PORT_DISPLAY_RES=WxH`.
pub const DEFAULT_IOS_MAX_LONG_EDGE: u32 = 1600;

fn parse_res_env(name: &str) -> Option<(u32, u32)> {
    let raw = std::env::var(name).ok()?;
    let (w, h) = raw.trim().split_once(['x', 'X'])?;
    let w = w.trim().parse::<u32>().ok()?;
    let h = h.trim().parse::<u32>().ok()?;
    (w >= 320 && h >= 200).then_some((w, h))
}

/// Pure host-surface resolution normalizer (unit-tested on macOS and iOS).
pub fn normalize_host_surface_size_with_config(
    raw_w: u32,
    raw_h: u32,
    raw_scale: f64,
    forced_res: Option<(u32, u32)>,
    max_long_edge: u32,
) -> (u32, u32, f64) {
    let rw = raw_w.max(1);
    let rh = raw_h.max(1);
    let rs = if raw_scale.is_finite() && raw_scale > 0.0 { raw_scale } else { 1.0 };
    if let Some((fw, fh)) = forced_res {
        let s = (rs * (fw as f64 / rw as f64)).max(0.5);
        return (fw.max(320), fh.max(200), s);
    }
    let long_edge = rw.max(rh);
    if max_long_edge >= 320 && long_edge > max_long_edge {
        let ratio = max_long_edge as f64 / long_edge as f64;
        let round8 = |v: f64, min_v: u32| -> u32 {
            let r = ((v / 8.0).round() as u32).saturating_mul(8);
            r.max(min_v)
        };
        let w = if rw >= rh { max_long_edge & !7 } else { round8(rw as f64 * ratio, 200) };
        let h = if rh > rw { max_long_edge & !7 } else { round8(rh as f64 * ratio, 200) };
        let s = (rs * (w as f64 / rw as f64)).max(0.5);
        return (w.max(320), h.max(200), s);
    }
    (rw, rh, rs)
}

/// Normalize a raw physical host surface `(raw_w, raw_h, raw_scale)` into the logical
/// Win32 display resolution, honoring `PORT_DISPLAY_RES=WxH` and `PORT_IOS_MAX_WIDTH` /
/// `PORT_MAX_DISPLAY_WIDTH` (defaulting to [`DEFAULT_IOS_MAX_LONG_EDGE`] on iOS).
pub fn normalize_host_surface_size(raw_w: u32, raw_h: u32, raw_scale: f64) -> (u32, u32, f64) {
    let forced = parse_res_env("PORT_DISPLAY_RES");
    let default_cap = if cfg!(target_os = "ios") { DEFAULT_IOS_MAX_LONG_EDGE } else { 0 };
    let cap = std::env::var("PORT_IOS_MAX_WIDTH")
        .or_else(|_| std::env::var("PORT_MAX_DISPLAY_WIDTH"))
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .unwrap_or(default_cap);
    normalize_host_surface_size_with_config(raw_w, raw_h, raw_scale, forced, cap)
}

/// Update the cached primary display from an embedded host surface (iOS `UIView`
/// / `CAMetalLayer`, or tests). Safe to call from any thread.
pub fn update_from_host_surface(width_px: u32, height_px: u32, scale: f64, refresh_hz: u32) {
    let w_px = width_px.max(1);
    let h_px = height_px.max(1);
    let s = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
    let w_pt = w_px as f64 / s;
    let h_pt = h_px as f64 / s;
    let desc = ScreenDesc {
        id: 1,
        frame_pt: (0.0, 0.0, w_pt, h_pt),
        visible_pt: (0.0, 0.0, w_pt, h_pt),
        backing_scale: s,
        refresh_hz: if refresh_hz == 0 { 60 } else { refresh_hz },
        name: if cfg!(target_os = "ios") { "iOS Main Screen".into() } else { "Host Surface".into() },
    };
    let d = build_displays(&[desc], 0.0, retina_enabled());
    *CACHE.write().unwrap() = Some(d);
}

/// Re-read `UIScreen.mainScreen` on iOS. Main thread only.
#[cfg(target_os = "ios")]
pub(super) fn refresh_ios() {
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject};
    use objc2_core_foundation::CGRect;

    unsafe {
        let Some(cls) = AnyClass::get(c"UIScreen") else {
            return;
        };
        let screen: *mut AnyObject = msg_send![cls, mainScreen];
        if screen.is_null() {
            return;
        }
        let native_bounds: CGRect = msg_send![screen, nativeBounds];
        let native_scale: f64 = msg_send![screen, nativeScale];
        let fps: isize = msg_send![screen, maximumFramesPerSecond];
        let raw_w = native_bounds.size.width.round().max(1.0) as u32;
        let raw_h = native_bounds.size.height.round().max(1.0) as u32;
        // Landscape-normalize when UIScreen.nativeBounds reports portrait before view attach.
        let (land_w, land_h) = if raw_h > raw_w { (raw_h, raw_w) } else { (raw_w, raw_h) };
        let raw_scale = if native_scale.is_finite() && native_scale > 0.0 { native_scale } else { 1.0 };
        let (w_px, h_px, scale) = normalize_host_surface_size(land_w, land_h, raw_scale);
        let hz = if fps > 0 { fps as u32 } else { 60 };
        update_from_host_surface(w_px, h_px, scale, hz);
        log::info!("displays(ios): raw {land_w}x{land_h}@{raw_scale:.2} -> logical {w_px}x{h_px}@{scale:.2}, {hz} Hz");
    }
}

/// Re-read NSScreen. Main thread only.
#[cfg(target_os = "macos")]
pub(super) fn refresh(mtm: objc2_foundation::MainThreadMarker) {
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2_app_kit::{NSScreen, NSWindow, NSWindowStyleMask};
    use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

    let screens = NSScreen::screens(mtm);
    let n = screens.count();
    if n == 0 {
        log::warn!("display: NSScreen reports no screens; using headless defaults");
        return;
    }

    let env_hz = std::env::var("PORT_REFRESH_HZ")
        .or_else(|_| std::env::var("AW2_REFRESH_HZ"))
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok());

    let mut descs = Vec::with_capacity(n);
    for i in 0..n {
        let s = screens.objectAtIndex(i);
        let id: u32 = unsafe {
            let desc: Option<Retained<AnyObject>> = msg_send![&*s, deviceDescription];
            let key = NSString::from_str("NSScreenNumber");
            let num: Option<Retained<AnyObject>> = match desc {
                Some(d) => msg_send![&*d, objectForKey: &*key],
                None => None,
            };
            match num {
                Some(v) => msg_send![&*v, unsignedIntValue],
                None => i as u32 + 1,
            }
        };
        let f = s.frame();
        let v = s.visibleFrame();
        let hz = env_hz.unwrap_or_else(|| s.maximumFramesPerSecond().max(0) as u32);
        descs.push(ScreenDesc {
            id,
            frame_pt: (f.origin.x, f.origin.y, f.size.width, f.size.height),
            visible_pt: (v.origin.x, v.origin.y, v.size.width, v.size.height),
            backing_scale: s.backingScaleFactor().max(1.0),
            refresh_hz: hz,
            name: s.localizedName().to_string(),
        });
    }
    let probe = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0));
    let framed = NSWindow::frameRectForContentRect_styleMask(probe, NSWindowStyleMask::Titled, mtm);
    let titlebar_pt = (framed.size.height - probe.size.height).max(0.0);
    let d = build_displays(&descs, titlebar_pt, retina_enabled());
    let changed = CACHE
        .read()
        .unwrap()
        .as_ref()
        .map(|o| o.monitors != d.monitors || o.scale != d.scale)
        .unwrap_or(true);
    if changed {
        log::info!("displays: primary scale {} titlebar {}pt", d.scale, d.titlebar_pt);
        for m in &d.monitors {
            log::info!(
                "  display {} {:?}{}: scale={} ({} DPI) rect ({},{})-({},{}) work ({},{})-({},{}) {} Hz",
                m.id,
                m.name,
                if m.primary { " [primary]" } else { "" },
                m.scale_factor,
                m.dpi(),
                m.rect.left,
                m.rect.top,
                m.rect.right,
                m.rect.bottom,
                m.work.left,
                m.work.top,
                m.work.right,
                m.work.bottom,
                m.refresh_hz
            );
        }
    }
    *CACHE.write().unwrap() = Some(d);
}

fn rect_overlap_area(a: &RECT, b: &RECT) -> i64 {
    a.intersect(b).map(|r| r.width() as i64 * r.height() as i64).unwrap_or(0)
}

fn dist2_px(r: &RECT, x: i32, y: i32) -> i64 {
    let dx = (x - x.clamp(r.left, (r.right - 1).max(r.left))) as i64;
    let dy = (y - y.clamp(r.top, (r.bottom - 1).max(r.top))) as i64;
    dx * dx + dy * dy
}

fn dist2_cocoa(f: (f64, f64, f64, f64), cx: f64, cy: f64) -> f64 {
    let (fx, fy, fw, fh) = f;
    let dx = cx - cx.clamp(fx, fx + fw);
    let dy = cy - cy.clamp(fy, fy + fh);
    dx * dx + dy * dy
}

fn monitor_for_px_rect_in(ms: &[Monitor], r: &RECT) -> Monitor {
    if let Some(m) = ms.iter().filter(|m| rect_overlap_area(&m.rect, r) > 0).max_by_key(|m| rect_overlap_area(&m.rect, r)) {
        return m.clone();
    }
    let cx = r.left + r.width() / 2;
    let cy = r.top + r.height() / 2;
    ms.iter().min_by_key(|m| dist2_px(&m.rect, cx, cy)).cloned().unwrap_or_else(|| ms[0].clone())
}

fn monitor_for_px_point_in(ms: &[Monitor], p: POINT) -> Monitor {
    if let Some(m) = ms.iter().find(|m| m.rect.contains(p.x, p.y)) {
        return m.clone();
    }
    ms.iter().min_by_key(|m| dist2_px(&m.rect, p.x, p.y)).cloned().unwrap_or_else(|| ms[0].clone())
}

fn monitor_for_cocoa_point_in(ms: &[Monitor], cx: f64, cy: f64) -> Monitor {
    if let Some(m) = ms.iter().find(|m| {
        let (fx, fy, fw, fh) = m.cocoa_frame;
        cx >= fx && cx < fx + fw && cy >= fy && cy <= fy + fh
    }) {
        return m.clone();
    }
    ms.iter()
        .min_by(|a, b| {
            dist2_cocoa(a.cocoa_frame, cx, cy)
                .partial_cmp(&dist2_cocoa(b.cocoa_frame, cx, cy))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
        .unwrap_or_else(|| ms[0].clone())
}

/// Backing scale factor of the monitor that owns Win32 pixel rect `r`.
pub fn scale_for_rect(r: &RECT) -> f64 {
    let d = get();
    monitor_for_px_rect_in(&d.monitors, r).scale_factor
}

/// Backing scale factor of the monitor containing Cocoa global point `(cx, cy)`.
pub fn scale_for_cocoa_point(cx: f64, cy: f64) -> f64 {
    let d = get();
    monitor_for_cocoa_point_in(&d.monitors, cx, cy).scale_factor
}

/// Win32 pixel rect (top-left origin) -> Cocoa global rect (points), using
/// the target monitor's own scale factor.
pub fn px_to_cocoa(r: &RECT) -> (f64, f64, f64, f64) {
    let d = get();
    let m = monitor_for_px_rect_in(&d.monitors, r);
    let (mx, my, _mw, mh) = m.cocoa_frame;
    let s = m.scale_factor;
    let x = mx + (r.left - m.rect.left) as f64 / s;
    let w = r.width() as f64 / s;
    let hh = r.height() as f64 / s;
    let top_cocoa = (my + mh) - (r.top - m.rect.top) as f64 / s;
    let y = top_cocoa - hh;
    (x, y, w, hh)
}

/// Cocoa global rect (points) -> Win32 pixel rect, using the screen's own scale factor.
pub fn cocoa_to_px(x: f64, y: f64, w: f64, h: f64) -> RECT {
    let d = get();
    let m = monitor_for_cocoa_point_in(&d.monitors, x + w * 0.5, y + h * 0.5);
    let (mx, my, _mw, mh) = m.cocoa_frame;
    let s = m.scale_factor;
    let left = m.rect.left + ((x - mx) * s).round() as i32;
    let top = m.rect.top + (((my + mh) - (y + h)) * s).round() as i32;
    let right = left + (w * s).round() as i32;
    let bottom = top + (h * s).round() as i32;
    RECT::new(left, top, right, bottom)
}

/// Cocoa global point -> Win32 screen pixel, using the containing monitor's scale factor.
pub fn cocoa_point_to_px(x: f64, y: f64) -> POINT {
    let d = get();
    let m = monitor_for_cocoa_point_in(&d.monitors, x, y);
    let (mx, my, _mw, mh) = m.cocoa_frame;
    let s = m.scale_factor;
    POINT {
        x: m.rect.left + ((x - mx) * s).floor() as i32,
        y: m.rect.top + (((my + mh) - y) * s).floor() as i32,
    }
}

/// Win32 screen pixel -> CoreGraphics global point (top-left origin, points)
/// for `CGWarpMouseCursorPosition`, using the containing monitor's scale factor.
pub fn px_point_to_cg(p: POINT) -> (f64, f64) {
    let d = get();
    let m = monitor_for_px_point_in(&d.monitors, p);
    let (mx, my, _mw, mh) = m.cocoa_frame;
    let m_top_pt = d.primary_h_pt - (my + mh);
    let cx = mx + (p.x - m.rect.left) as f64 / m.scale_factor;
    let cy = m_top_pt + (p.y - m.rect.top) as f64 / m.scale_factor;
    (cx, cy)
}

#[cfg(test)]
pub(crate) fn with_test_displays<R>(d: Displays, f: impl FnOnce() -> R) -> R {
    let prev = CACHE.write().unwrap().replace(d);
    struct Restore(Option<Displays>);
    impl Drop for Restore {
        fn drop(&mut self) {
            *CACHE.write().unwrap() = self.0.take();
        }
    }
    let _r = Restore(prev);
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_defaults_and_flip() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = get();
        assert!(!d.monitors.is_empty());
        if CACHE.read().unwrap().is_none() {
            let r = RECT::new(100, 50, 900, 650);
            let (x, y, w, h) = px_to_cocoa(&r);
            assert_eq!((x, y, w, h), (100.0, 1080.0 - 650.0, 800.0, 600.0));
            assert_eq!(cocoa_to_px(x, y, w, h), r);
        }
    }

    #[test]
    fn mixed_dpi_layout_and_coord_roundtrip() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = build_displays(
            &[
                // Primary: 2x Retina (1512x982 pt -> 3024x1964 px, 192 DPI)
                ScreenDesc {
                    id: 100,
                    frame_pt: (0.0, 0.0, 1512.0, 982.0),
                    visible_pt: (0.0, 0.0, 1512.0, 950.0),
                    backing_scale: 2.0,
                    refresh_hz: 120,
                    name: "Built-in Retina".into(),
                },
                // Secondary: 1x external 1080p to the right (1920x1080 pt -> 1920x1080 px, 96 DPI)
                ScreenDesc {
                    id: 200,
                    frame_pt: (1512.0, -98.0, 1920.0, 1080.0),
                    visible_pt: (1512.0, -98.0, 1920.0, 1055.0),
                    backing_scale: 1.0,
                    refresh_hz: 60,
                    name: "External 1080p".into(),
                },
            ],
            28.0,
            true,
        );
        assert_eq!(d.monitors.len(), 2);
        assert_eq!(d.monitors[0].rect, RECT::new(0, 0, 3024, 1964));
        assert_eq!(d.monitors[0].scale_factor, 2.0);
        assert_eq!(d.monitors[0].dpi(), 192);
        assert_eq!(d.monitors[1].rect, RECT::new(3024, 0, 4944, 1080));
        assert_eq!(d.monitors[1].scale_factor, 1.0);
        assert_eq!(d.monitors[1].dpi(), 96);

        with_test_displays(d, || {
            // Roundtrip a window rect on the 2x Retina monitor.
            let r_retina = RECT::new(200, 100, 1800, 1000);
            let (cx, cy, cw, ch) = px_to_cocoa(&r_retina);
            assert_eq!((cx, cw, ch), (100.0, 800.0, 450.0));
            assert_eq!(cocoa_to_px(cx, cy, cw, ch), r_retina);
            assert_eq!(scale_for_rect(&r_retina), 2.0);

            // Roundtrip a window rect on the 1x external monitor.
            let r_ext = RECT::new(3124, 50, 4124, 650);
            let (ex, ey, ew, eh) = px_to_cocoa(&r_ext);
            assert_eq!((ex, ew, eh), (1612.0, 1000.0, 600.0));
            assert_eq!(cocoa_to_px(ex, ey, ew, eh), r_ext);
            assert_eq!(scale_for_rect(&r_ext), 1.0);

            // Point mapping across both displays.
            let p_ext = POINT { x: 3224, y: 100 };
            let (gx, gy) = px_point_to_cg(p_ext);
            assert_eq!((gx, gy), (1712.0, 100.0));
            assert_eq!(cocoa_point_to_px(1712.0, 982.0 - 100.0), p_ext);
        });
    }

    #[test]
    fn host_surface_updates_display_cache() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prev = CACHE.read().unwrap().clone();
        update_from_host_surface(2796, 1290, 3.0, 120);
        let d = get();
        assert_eq!(d.monitors.len(), 1);
        assert_eq!(d.monitors[0].rect, RECT::new(0, 0, 2796, 1290));
        assert_eq!(d.monitors[0].refresh_hz, 120);
        assert_eq!(d.titlebar_pt, 0.0);
        *CACHE.write().unwrap() = prev;
    }

    #[test]
    fn normalize_host_surface_caps_retina_and_preserves_aspect() {
        // iPhone 17 Pro Max 2868x1320 @ 3x capped at 1600 -> 1600x736
        let (w, h, s) = normalize_host_surface_size_with_config(2868, 1320, 3.0, None, 1600);
        assert_eq!((w, h), (1600, 736));
        assert!((s - 3.0 * (1600.0 / 2868.0)).abs() < 1e-6);

        // Forced PORT_DISPLAY_RES overrides cap
        let (fw, fh, _) = normalize_host_surface_size_with_config(2868, 1320, 3.0, Some((1280, 720)), 1600);
        assert_eq!((fw, fh), (1280, 720));

        // Uncapped (0) preserves exact physical size
        let (uw, uh, us) = normalize_host_surface_size_with_config(2868, 1320, 3.0, None, 0);
        assert_eq!((uw, uh, us), (2868, 1320, 3.0));
    }
}
