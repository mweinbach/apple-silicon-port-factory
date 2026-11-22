//! uxtheme.dll / comctl32.dll: Desktop Visual Styles (honest unthemed fallback),
//! common controls initialization, and `_TrackMouseEvent` per-HWND state tracking.

use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[(17, "InitCommonControls"), (345, "TaskDialogIndirect")];

const E_HANDLE: HRESULT = 0x8007_0006u32 as i32;

// TRACKMOUSEEVENT.dwFlags
const TME_HOVER: u32 = 0x0000_0001;
const TME_LEAVE: u32 = 0x0000_0002;
const TME_NONCLIENT: u32 = 0x0000_0010;
const TME_QUERY: u32 = 0x4000_0000;
const TME_CANCEL: u32 = 0x8000_0000;
const TME_VALID_TRACK_MASK: u32 = TME_HOVER | TME_LEAVE | TME_NONCLIENT;
const HOVER_DEFAULT: u32 = 0xFFFF_FFFF;
const DEFAULT_HOVER_MS: u32 = 400;

#[derive(Clone, Copy, Debug, Default)]
struct MouseTrackEntry {
    flags: u32,
    hover_time: u32,
}

#[derive(Default)]
struct MouseTrackState {
    per_hwnd: HashMap<usize, MouseTrackEntry>,
    last_hwnd: usize,
}

fn mouse_track() -> &'static Mutex<MouseTrackState> {
    static S: OnceLock<Mutex<MouseTrackState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(MouseTrackState::default()))
}

pub unsafe extern "win64" fn IsAppThemed() -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn IsThemeActive() -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn OpenThemeData(_hwnd: HWND, _class_list: LPCWSTR) -> usize {
    0
}

pub unsafe extern "win64" fn OpenThemeDataEx(_hwnd: HWND, _class_list: LPCWSTR, _flags: u32) -> usize {
    0
}

pub unsafe extern "win64" fn OpenThemeDataForDpi(_hwnd: HWND, _class_list: LPCWSTR, _dpi: u32) -> usize {
    0
}

pub unsafe extern "win64" fn CloseThemeData(htheme: usize) -> HRESULT {
    if htheme == 0 {
        E_HANDLE
    } else {
        S_OK
    }
}

pub unsafe extern "win64" fn SetWindowTheme(_hwnd: HWND, _sub_app: LPCWSTR, _sub_id: LPCWSTR) -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn GetThemeColor(
    htheme: usize,
    _part: i32,
    _state: i32,
    _prop: i32,
    color: *mut u32,
) -> HRESULT {
    if htheme == 0 {
        return E_HANDLE;
    }
    if color.is_null() {
        return E_POINTER;
    }
    color.write_unaligned(0x0020_2020);
    S_OK
}

pub unsafe extern "win64" fn GetThemeSysColor(_htheme: usize, color_id: i32) -> u32 {
    crate::user32::misc::GetSysColor(color_id)
}

pub unsafe extern "win64" fn DrawThemeBackground(
    htheme: usize,
    _hdc: usize,
    _part: i32,
    _state: i32,
    _rect: *const u8,
    _clip: *const u8,
) -> HRESULT {
    if htheme == 0 {
        return E_HANDLE;
    }
    stub!("DrawThemeBackground(htheme={htheme:#x}) -> E_NOTIMPL");
    E_NOTIMPL
}

pub unsafe extern "win64" fn InitCommonControls() {}

pub unsafe extern "win64" fn InitCommonControlsEx(icce: *const u8) -> BOOL {
    if icce.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb_size = (icce as *const u32).read_unaligned();
    if cb_size < 8 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    TRUE
}

/// `TRACKMOUSEEVENT` (x64, 24 bytes): `cbSize` @0, `dwFlags` @4, `hwndTrack` @8, `dwHoverTime` @16.
pub unsafe extern "win64" fn _TrackMouseEvent(tme: *mut u8) -> BOOL {
    if tme.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb_size = (tme as *const u32).read_unaligned();
    if cb_size < 24 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let flags = (tme.add(4) as *const u32).read_unaligned();
    let hwnd = (tme.add(8) as *const usize).read_unaligned();
    let hover_time = (tme.add(16) as *const u32).read_unaligned();

    let mut st = mouse_track().lock().unwrap();
    if (flags & TME_QUERY) != 0 {
        let query_hwnd = if hwnd != 0 { hwnd } else { st.last_hwnd };
        if let Some(&entry) = st.per_hwnd.get(&query_hwnd) {
            (tme.add(4) as *mut u32).write_unaligned(entry.flags);
            (tme.add(8) as *mut usize).write_unaligned(query_hwnd);
            (tme.add(16) as *mut u32).write_unaligned(entry.hover_time);
        } else {
            (tme.add(4) as *mut u32).write_unaligned(0);
            (tme.add(16) as *mut u32).write_unaligned(0);
        }
        return TRUE;
    }

    if hwnd == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }

    let track_bits = flags & TME_VALID_TRACK_MASK;
    if (flags & TME_CANCEL) != 0 {
        if let Some(entry) = st.per_hwnd.get_mut(&hwnd) {
            entry.flags &= !track_bits;
            if entry.flags == 0 {
                st.per_hwnd.remove(&hwnd);
                if st.last_hwnd == hwnd {
                    st.last_hwnd = 0;
                }
            }
        }
        return TRUE;
    }

    let eff_hover = if hover_time == HOVER_DEFAULT { DEFAULT_HOVER_MS } else { hover_time };
    let entry = st.per_hwnd.entry(hwnd).or_default();
    entry.flags |= track_bits;
    if (track_bits & TME_HOVER) != 0 {
        entry.hover_time = eff_hover;
    }
    st.last_hwnd = hwnd;
    TRUE
}

pub unsafe extern "win64" fn TaskDialogIndirect(
    _config: *const u8,
    button: *mut i32,
    radio_button: *mut i32,
    verification_flag: *mut BOOL,
) -> HRESULT {
    if !button.is_null() {
        button.write_unaligned(0);
    }
    if !radio_button.is_null() {
        radio_button.write_unaligned(0);
    }
    if !verification_flag.is_null() {
        verification_flag.write_unaligned(FALSE);
    }
    stub!("TaskDialogIndirect -> E_NOTIMPL (native task dialogs not implemented)");
    E_NOTIMPL
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tme(flags: u32, hwnd: usize, hover: u32) -> [u8; 24] {
        let mut buf = [0u8; 24];
        buf[0..4].copy_from_slice(&24u32.to_le_bytes());
        buf[4..8].copy_from_slice(&flags.to_le_bytes());
        buf[8..16].copy_from_slice(&hwnd.to_le_bytes());
        buf[16..20].copy_from_slice(&hover.to_le_bytes());
        buf
    }

    #[test]
    fn honest_unthemed_fallback_and_task_dialog() {
        unsafe {
            assert_eq!(IsThemeActive(), FALSE);
            assert_eq!(IsAppThemed(), FALSE);
            assert_eq!(OpenThemeData(std::ptr::null_mut(), std::ptr::null()), 0);
            assert_eq!(OpenThemeDataEx(std::ptr::null_mut(), std::ptr::null(), 0), 0);
            assert_eq!(OpenThemeDataForDpi(std::ptr::null_mut(), std::ptr::null(), 96), 0);
            assert_eq!(CloseThemeData(0), E_HANDLE);
            assert_eq!(DrawThemeBackground(0, 0, 0, 0, std::ptr::null(), std::ptr::null()), E_HANDLE);

            let mut btn = 99;
            let mut radio = 99;
            let mut verified = TRUE;
            let hr = TaskDialogIndirect(std::ptr::null(), &mut btn, &mut radio, &mut verified);
            assert_eq!(hr, E_NOTIMPL);
            assert_eq!((btn, radio, verified), (0, 0, FALSE));
        }
    }

    #[test]
    fn track_mouse_event_stores_queries_and_cancels_per_hwnd() {
        unsafe {
            let hwnd = 0x00AB_0010usize;

            // Register HOVER + LEAVE with HOVER_DEFAULT.
            let mut reg = make_tme(TME_HOVER | TME_LEAVE, hwnd, HOVER_DEFAULT);
            assert_eq!(_TrackMouseEvent(reg.as_mut_ptr()), TRUE);

            // TME_QUERY returns stored flags and resolved default hover time (400 ms).
            let mut q = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q[4..8].try_into().unwrap()), TME_HOVER | TME_LEAVE);
            assert_eq!(u32::from_le_bytes(q[16..20].try_into().unwrap()), DEFAULT_HOVER_MS);

            // Cancel TME_HOVER only; TME_LEAVE remains active.
            let mut cancel_hover = make_tme(TME_CANCEL | TME_HOVER, hwnd, 0);
            assert_eq!(_TrackMouseEvent(cancel_hover.as_mut_ptr()), TRUE);

            let mut q2 = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q2.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q2[4..8].try_into().unwrap()), TME_LEAVE);

            // Cancel remaining TME_LEAVE -> flags become 0.
            let mut cancel_leave = make_tme(TME_CANCEL | TME_LEAVE, hwnd, 0);
            assert_eq!(_TrackMouseEvent(cancel_leave.as_mut_ptr()), TRUE);

            let mut q3 = make_tme(TME_QUERY, hwnd, 0);
            assert_eq!(_TrackMouseEvent(q3.as_mut_ptr()), TRUE);
            assert_eq!(u32::from_le_bytes(q3[4..8].try_into().unwrap()), 0);

            // Undersized cbSize must fail.
            let mut bad = [0u8; 24];
            bad[0..4].copy_from_slice(&12u32.to_le_bytes());
            assert_eq!(_TrackMouseEvent(bad.as_mut_ptr()), FALSE);
        }
    }
}
