//! dwmapi.dll: Desktop Window Manager attributes and composition timing.
//! Persists per-HWND attributes for accurate `DwmSetWindowAttribute` /
//! `DwmGetWindowAttribute` round-tripping and populates real display refresh
//! rate and QPC timing in `DwmGetCompositionTimingInfo`.

use crate::types::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_INVALID_WINDOW_HANDLE_HR: HRESULT = 0x8007_0578u32 as HRESULT;
const ERROR_INSUFFICIENT_BUFFER_HR: HRESULT = 0x8007_007Au32 as HRESULT;

const DWMWA_NCRENDERING_ENABLED: u32 = 1;
const DWMWA_NCRENDERING_POLICY: u32 = 2;
const DWMWA_TRANSITIONS_FORCEDISABLED: u32 = 3;
const DWMWA_ALLOW_NCPAINT: u32 = 4;
const DWMWA_CAPTION_BUTTON_BOUNDS: u32 = 5;
const DWMWA_NONCLIENT_RTL_LAYOUT: u32 = 6;
const DWMWA_FORCE_ICONIC_REPRESENTATION: u32 = 7;
const DWMWA_FLIP3D_POLICY: u32 = 8;
const DWMWA_EXTENDED_FRAME_BOUNDS: u32 = 9;
const DWMWA_HAS_ICONIC_BITMAP: u32 = 10;
const DWMWA_DISALLOW_PEEK: u32 = 11;
const DWMWA_EXCLUDED_FROM_PEEK: u32 = 12;
const DWMWA_CLOAK: u32 = 13;
const DWMWA_CLOAKED: u32 = 14;
const DWMWA_FREEZE_REPRESENTATION: u32 = 15;
const DWMWA_PASSIVE_UPDATE_MODE: u32 = 16;
const DWMWA_USE_HOSTBACKDROPBRUSH: u32 = 17;
const DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1: u32 = 19;
const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;
const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWA_BORDER_COLOR: u32 = 34;
const DWMWA_CAPTION_COLOR: u32 = 35;
const DWMWA_TEXT_COLOR: u32 = 36;
const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: u32 = 37;
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;

const DWMNCRP_DISABLED: u32 = 1;
const DWM_CLOAKED_APP: u32 = 1;
const DWMWA_COLOR_DEFAULT: u32 = 0xFFFF_FFFF;

static HWND_ATTRS: LazyLock<Mutex<HashMap<(usize, u32), Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn primary_refresh_hz() -> u32 {
    let monitors = crate::macos::display::monitors();
    monitors
        .iter()
        .find(|m| m.primary)
        .or_else(|| monitors.first())
        .map(|m| m.refresh_hz.max(1))
        .unwrap_or(60)
}

fn is_readonly_attr(attr: u32) -> bool {
    matches!(
        attr,
        DWMWA_NCRENDERING_ENABLED
            | DWMWA_CAPTION_BUTTON_BOUNDS
            | DWMWA_EXTENDED_FRAME_BOUNDS
            | DWMWA_CLOAKED
            | DWMWA_VISIBLE_FRAME_BORDER_THICKNESS
    )
}

fn is_known_attr(attr: u32) -> bool {
    matches!(
        attr,
        DWMWA_NCRENDERING_ENABLED
            | DWMWA_NCRENDERING_POLICY
            | DWMWA_TRANSITIONS_FORCEDISABLED
            | DWMWA_ALLOW_NCPAINT
            | DWMWA_CAPTION_BUTTON_BOUNDS
            | DWMWA_NONCLIENT_RTL_LAYOUT
            | DWMWA_FORCE_ICONIC_REPRESENTATION
            | DWMWA_FLIP3D_POLICY
            | DWMWA_EXTENDED_FRAME_BOUNDS
            | DWMWA_HAS_ICONIC_BITMAP
            | DWMWA_DISALLOW_PEEK
            | DWMWA_EXCLUDED_FROM_PEEK
            | DWMWA_CLOAK
            | DWMWA_CLOAKED
            | DWMWA_FREEZE_REPRESENTATION
            | DWMWA_PASSIVE_UPDATE_MODE
            | DWMWA_USE_HOSTBACKDROPBRUSH
            | DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1
            | DWMWA_USE_IMMERSIVE_DARK_MODE
            | DWMWA_WINDOW_CORNER_PREFERENCE
            | DWMWA_BORDER_COLOR
            | DWMWA_CAPTION_COLOR
            | DWMWA_TEXT_COLOR
            | DWMWA_VISIBLE_FRAME_BORDER_THICKNESS
            | DWMWA_SYSTEMBACKDROP_TYPE
    )
}

pub unsafe extern "win64" fn DwmSetWindowAttribute(
    hwnd: HWND,
    attr: u32,
    pv: *const u8,
    cb: u32,
) -> HRESULT {
    if hwnd.is_null() {
        return ERROR_INVALID_WINDOW_HANDLE_HR;
    }
    if pv.is_null() || cb == 0 || cb > 1024 || is_readonly_attr(attr) {
        return E_INVALIDARG;
    }
    let data = std::slice::from_raw_parts(pv, cb as usize).to_vec();
    HWND_ATTRS.lock().unwrap().insert((hwnd as usize, attr), data);
    log::debug!("DwmSetWindowAttribute(hwnd {:#x}, attr {attr}, cb {cb}) -> S_OK", hwnd as usize);
    S_OK
}

pub unsafe extern "win64" fn DwmGetWindowAttribute(
    hwnd: HWND,
    attr: u32,
    pv: *mut u8,
    cb: u32,
) -> HRESULT {
    if pv.is_null() || cb == 0 {
        return E_INVALIDARG;
    }
    if hwnd.is_null() {
        return ERROR_INVALID_WINDOW_HANDLE_HR;
    }
    let key_hwnd = hwnd as usize;

    match attr {
        DWMWA_NCRENDERING_ENABLED => {
            if cb < 4 {
                return E_INVALIDARG;
            }
            let disabled = HWND_ATTRS
                .lock()
                .unwrap()
                .get(&(key_hwnd, DWMWA_NCRENDERING_POLICY))
                .and_then(|v| (v.len() >= 4).then(|| u32::from_le_bytes(v[0..4].try_into().unwrap())))
                == Some(DWMNCRP_DISABLED);
            std::ptr::write_unaligned(pv as *mut u32, (!disabled) as u32);
            return S_OK;
        }
        DWMWA_EXTENDED_FRAME_BOUNDS => {
            if cb < 16 {
                return E_INVALIDARG;
            }
            return match crate::user32::window::window_rect_screen(key_hwnd) {
                Some(r) => {
                    std::ptr::write_unaligned(pv as *mut crate::user32::RECT, r);
                    S_OK
                }
                None => ERROR_INVALID_WINDOW_HANDLE_HR,
            };
        }
        DWMWA_CLOAKED => {
            if cb < 4 {
                return E_INVALIDARG;
            }
            let cloaked = HWND_ATTRS
                .lock()
                .unwrap()
                .get(&(key_hwnd, DWMWA_CLOAK))
                .and_then(|v| (v.len() >= 4).then(|| u32::from_le_bytes(v[0..4].try_into().unwrap())))
                .is_some_and(|v| v != 0);
            std::ptr::write_unaligned(pv as *mut u32, if cloaked { DWM_CLOAKED_APP } else { 0 });
            return S_OK;
        }
        _ => {}
    }

    if let Some(stored) = HWND_ATTRS.lock().unwrap().get(&(key_hwnd, attr)).cloned() {
        if (cb as usize) < stored.len() {
            return ERROR_INSUFFICIENT_BUFFER_HR;
        }
        std::ptr::copy_nonoverlapping(stored.as_ptr(), pv, stored.len());
        return S_OK;
    }

    // Documented defaults for known attributes not yet set on this HWND.
    if !is_known_attr(attr) {
        return E_INVALIDARG;
    }
    match attr {
        DWMWA_CAPTION_BUTTON_BOUNDS => {
            if cb < 16 {
                return E_INVALIDARG;
            }
            std::ptr::write_bytes(pv, 0, 16);
            S_OK
        }
        DWMWA_BORDER_COLOR | DWMWA_CAPTION_COLOR | DWMWA_TEXT_COLOR => {
            if cb < 4 {
                return E_INVALIDARG;
            }
            std::ptr::write_unaligned(pv as *mut u32, DWMWA_COLOR_DEFAULT);
            S_OK
        }
        DWMWA_VISIBLE_FRAME_BORDER_THICKNESS => {
            if cb < 4 {
                return E_INVALIDARG;
            }
            std::ptr::write_unaligned(pv as *mut u32, 1u32);
            S_OK
        }
        _ => {
            if cb < 4 {
                return E_INVALIDARG;
            }
            std::ptr::write_unaligned(pv as *mut u32, 0u32);
            S_OK
        }
    }
}

/// Composition is always on (Windows 8+ semantics).
pub unsafe extern "win64" fn DwmIsCompositionEnabled(enabled: *mut BOOL) -> HRESULT {
    if enabled.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(enabled, TRUE);
    S_OK
}

/// Waits for one compositor refresh interval matching the active display rate.
pub unsafe extern "win64" fn DwmFlush() -> HRESULT {
    let hz = primary_refresh_hz() as u64;
    let us = (1_000_000 / hz).max(1);
    std::thread::sleep(std::time::Duration::from_micros(us));
    S_OK
}

/// `UNSIGNED_RATIO` + `DWM_TIMING_INFO` (`dwmapi.h` uses `<pshpack1.h>` -> 292 bytes).
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
struct UNSIGNED_RATIO {
    ui_numerator: u32,
    ui_denominator: u32,
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
struct DWM_TIMING_INFO {
    cb_size: u32,
    rate_refresh: UNSIGNED_RATIO,
    qpc_refresh_period: u64,
    rate_compose: UNSIGNED_RATIO,
    qpc_vblank: u64,
    c_refresh: u64,
    c_dx_refresh: u32,
    qpc_compose: u64,
    c_frame: u64,
    c_dx_present: u32,
    c_refresh_frame: u64,
    c_frame_submitted: u64,
    c_dx_present_submitted: u32,
    c_frame_confirmed: u64,
    c_dx_present_confirmed: u32,
    c_refresh_confirmed: u64,
    c_dx_refresh_confirmed: u32,
    c_frames_late: u64,
    c_frames_outstanding: u32,
    c_frame_displayed: u64,
    qpc_frame_displayed: u64,
    c_refresh_frame_displayed: u64,
    c_frame_complete: u64,
    qpc_frame_complete: u64,
    c_frame_pending: u64,
    qpc_frame_pending: u64,
    c_frames_displayed: u64,
    c_frames_complete: u64,
    c_frames_pending: u64,
    c_frames_available: u64,
    c_frames_dropped: u64,
    c_frames_missed: u64,
    c_refresh_next_displayed: u64,
    c_refresh_next_presented: u64,
    c_refreshes_displayed: u64,
    c_refreshes_presented: u64,
    c_refresh_started: u64,
    c_pixels_received: u64,
    c_pixels_drawn: u64,
    c_buffers_empty: u64,
}
const _: () = assert!(std::mem::size_of::<DWM_TIMING_INFO>() == 292);

pub unsafe extern "win64" fn DwmGetCompositionTimingInfo(
    _hwnd: HWND,
    timing_info: *mut u8,
) -> HRESULT {
    if timing_info.is_null() {
        return E_INVALIDARG;
    }
    let cb = std::ptr::read_unaligned(timing_info as *const u32) as usize;
    if cb < std::mem::size_of::<DWM_TIMING_INFO>() {
        return E_INVALIDARG;
    }

    let hz = primary_refresh_hz();
    let qpc_freq = crate::kernel32::time::QPC_FREQUENCY as u64;
    let qpc_now = crate::kernel32::time::interrupt_time_100ns();
    let qpc_period = ((qpc_freq + (hz as u64 / 2)) / (hz as u64)).max(1);
    let c_refresh = qpc_now / qpc_period;
    let qpc_vblank = c_refresh.saturating_mul(qpc_period);
    let rate = UNSIGNED_RATIO {
        ui_numerator: hz,
        ui_denominator: 1,
    };

    let info = DWM_TIMING_INFO {
        cb_size: cb as u32,
        rate_refresh: rate,
        qpc_refresh_period: qpc_period,
        rate_compose: rate,
        qpc_vblank,
        c_refresh,
        c_dx_refresh: c_refresh as u32,
        qpc_compose: qpc_vblank,
        c_frame: c_refresh,
        c_dx_present: c_refresh as u32,
        c_refresh_frame: c_refresh,
        c_frame_submitted: c_refresh,
        c_dx_present_submitted: c_refresh as u32,
        c_frame_confirmed: c_refresh,
        c_dx_present_confirmed: c_refresh as u32,
        c_refresh_confirmed: c_refresh,
        c_dx_refresh_confirmed: c_refresh as u32,
        c_frames_late: 0,
        c_frames_outstanding: 0,
        c_frame_displayed: c_refresh,
        qpc_frame_displayed: qpc_vblank,
        c_refresh_frame_displayed: c_refresh,
        c_frame_complete: c_refresh,
        qpc_frame_complete: qpc_vblank,
        c_frame_pending: c_refresh.saturating_add(1),
        qpc_frame_pending: qpc_vblank.saturating_add(qpc_period),
        c_frames_displayed: c_refresh,
        c_frames_complete: c_refresh,
        c_frames_pending: 0,
        c_frames_available: c_refresh,
        c_frames_dropped: 0,
        c_frames_missed: 0,
        c_refresh_next_displayed: c_refresh.saturating_add(1),
        c_refresh_next_presented: c_refresh.saturating_add(1),
        c_refreshes_displayed: c_refresh,
        c_refreshes_presented: c_refresh,
        c_refresh_started: c_refresh,
        c_pixels_received: 0,
        c_pixels_drawn: 0,
        c_buffers_empty: 0,
    };

    std::ptr::write_unaligned(timing_info as *mut DWM_TIMING_INFO, info);
    if cb > std::mem::size_of::<DWM_TIMING_INFO>() {
        let extra = cb - std::mem::size_of::<DWM_TIMING_INFO>();
        std::ptr::write_bytes(timing_info.add(std::mem::size_of::<DWM_TIMING_INFO>()), 0, extra);
    }
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dwm_attributes_roundtrip_and_isolation() {
        let hwnd_a = 0xCAFE_0001usize as HWND;
        let hwnd_b = 0xCAFE_0002usize as HWND;

        unsafe {
            // Default values before setting
            let mut val: u32 = 99;
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_USE_IMMERSIVE_DARK_MODE, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 0);
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_NCRENDERING_ENABLED, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 1);

            // Set dark mode, corner preference, border color, and cloak on hwnd_a
            let dark: u32 = 1;
            let corner: u32 = 2; // DWMWCP_ROUND
            let color: u32 = 0x0022_4466;
            let cloak: u32 = 1;
            assert_eq!(
                DwmSetWindowAttribute(hwnd_a, DWMWA_USE_IMMERSIVE_DARK_MODE, &dark as *const u32 as *const u8, 4),
                S_OK
            );
            assert_eq!(
                DwmSetWindowAttribute(hwnd_a, DWMWA_WINDOW_CORNER_PREFERENCE, &corner as *const u32 as *const u8, 4),
                S_OK
            );
            assert_eq!(
                DwmSetWindowAttribute(hwnd_a, DWMWA_BORDER_COLOR, &color as *const u32 as *const u8, 4),
                S_OK
            );
            assert_eq!(
                DwmSetWindowAttribute(hwnd_a, DWMWA_CLOAK, &cloak as *const u32 as *const u8, 4),
                S_OK
            );

            // Read back on hwnd_a
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_USE_IMMERSIVE_DARK_MODE, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 1);
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_WINDOW_CORNER_PREFERENCE, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 2);
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_BORDER_COLOR, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 0x0022_4466);
            assert_eq!(
                DwmGetWindowAttribute(hwnd_a, DWMWA_CLOAKED, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, DWM_CLOAKED_APP);

            // hwnd_b remains independent
            assert_eq!(
                DwmGetWindowAttribute(hwnd_b, DWMWA_USE_IMMERSIVE_DARK_MODE, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, 0);
            assert_eq!(
                DwmGetWindowAttribute(hwnd_b, DWMWA_BORDER_COLOR, &mut val as *mut u32 as *mut u8, 4),
                S_OK
            );
            assert_eq!(val, DWMWA_COLOR_DEFAULT);

            // Setting read-only attribute fails cleanly
            assert_eq!(
                DwmSetWindowAttribute(hwnd_a, DWMWA_NCRENDERING_ENABLED, &dark as *const u32 as *const u8, 4),
                E_INVALIDARG
            );
        }
    }

    #[test]
    fn dwm_composition_timing_info_populates_real_refresh_and_qpc() {
        unsafe {
            let mut info = DWM_TIMING_INFO {
                cb_size: std::mem::size_of::<DWM_TIMING_INFO>() as u32,
                ..Default::default()
            };
            assert_eq!(
                DwmGetCompositionTimingInfo(std::ptr::null_mut(), &mut info as *mut _ as *mut u8),
                S_OK
            );
            let num = info.rate_refresh.ui_numerator;
            let den = info.rate_refresh.ui_denominator;
            let period = info.qpc_refresh_period;
            let vblank = info.qpc_vblank;
            let c_refresh = info.c_refresh;
            assert!(num >= 30 && num <= 480, "unexpected refresh numerator {num}");
            assert_eq!(den, 1);
            assert_eq!(period, (crate::kernel32::time::QPC_FREQUENCY as u64 + num as u64 / 2) / num as u64);
            assert!(vblank > 0);
            assert_eq!(vblank, c_refresh * period);

            // Undersized buffer rejected
            let mut bad = [0u8; 16];
            std::ptr::write_unaligned(bad.as_mut_ptr() as *mut u32, 16u32);
            assert_eq!(DwmGetCompositionTimingInfo(std::ptr::null_mut(), bad.as_mut_ptr()), E_INVALIDARG);
        }
    }
}
