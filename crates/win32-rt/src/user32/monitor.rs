//! Monitors: EnumDisplayMonitors, MonitorFrom*, GetMonitorInfo, and CCD
//! display-configuration queries (`GetDisplayConfigBufferSizes` /
//! `QueryDisplayConfig` / `DisplayConfigGetDeviceInfo`).
//!
//! Data comes from the main-thread display cache
//! ([`crate::macos::display`]). HMONITORs are stable per CGDirectDisplayID
//! for the life of the process.

use super::*;
use crate::macos::display::{self, Monitor};
use crate::types::*;
use std::sync::Mutex;

const HMONITOR_BASE: usize = 0x1000_0000;
const MONITOR_DEFAULTTONULL: u32 = 0;
const MONITOR_DEFAULTTOPRIMARY: u32 = 1;
const MONITORINFOF_PRIMARY: u32 = 1;

/// Display ids in first-seen order; HMONITOR = base + 0x10 * index.
static IDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn handle_of(m: &Monitor) -> usize {
    let mut ids = IDS.lock().unwrap();
    let i = match ids.iter().position(|&i| i == m.id) {
        Some(i) => i,
        None => {
            ids.push(m.id);
            ids.len() - 1
        }
    };
    HMONITOR_BASE + 0x10 * (i + 1)
}

/// HMONITOR of the display with CGDirectDisplayID `id` (0 if unknown); used
/// by `IDXGIOutput::GetDesc`.
pub fn hmonitor_for_id(id: u32) -> usize {
    display::monitors().iter().find(|m| m.id == id).map(handle_of).unwrap_or(0)
}

fn monitor_of(h: usize) -> Option<(usize, Monitor)> {
    if h < HMONITOR_BASE {
        return None;
    }
    let i = (h - HMONITOR_BASE) / 0x10;
    let id = *IDS.lock().unwrap().get(i.checked_sub(1)?)?;
    let ms = display::monitors();
    let pos = ms.iter().position(|m| m.id == id)?;
    Some((pos, ms[pos].clone()))
}

fn primary() -> Monitor {
    let ms = display::monitors();
    ms.iter().find(|m| m.primary).cloned().unwrap_or_else(|| ms[0].clone())
}

pub fn primary_work_area() -> RECT {
    primary().work
}

pub fn primary_rect() -> RECT {
    primary().rect
}

pub fn primary_dpi() -> u32 {
    primary().dpi()
}

pub fn dpi_for_monitor(h: usize) -> Option<u32> {
    monitor_of(h).map(|(_, m)| m.dpi())
}

pub fn dpi_for_window(hwnd: usize) -> u32 {
    window_monitor(hwnd, 2).map(|m| m.dpi()).unwrap_or_else(primary_dpi)
}

pub fn scale_for_window(hwnd: usize) -> f64 {
    window_monitor(hwnd, 2).map(|m| m.scale_factor).unwrap_or_else(|| primary().scale_factor)
}

fn area(a: &RECT, b: &RECT) -> i64 {
    a.intersect(b).map(|r| r.width() as i64 * r.height() as i64).unwrap_or(0)
}

fn dist2(r: &RECT, p: POINT) -> i64 {
    let dx = (p.x - p.x.clamp(r.left, r.right - 1)) as i64;
    let dy = (p.y - p.y.clamp(r.top, r.bottom - 1)) as i64;
    dx * dx + dy * dy
}

/// Monitor for a rect: largest overlap, else nearest (or primary/none per flags).
fn from_rect(r: &RECT, flags: u32) -> Option<Monitor> {
    let ms = display::monitors();
    if let Some(m) = ms.iter().filter(|m| area(&m.rect, r) > 0).max_by_key(|m| area(&m.rect, r)) {
        return Some(m.clone());
    }
    match flags {
        MONITOR_DEFAULTTONULL => None,
        MONITOR_DEFAULTTOPRIMARY => Some(primary()),
        _ => {
            let c = POINT { x: r.left + r.width() / 2, y: r.top + r.height() / 2 };
            ms.iter().min_by_key(|m| dist2(&m.rect, c)).cloned()
        }
    }
}

fn from_point(p: POINT, flags: u32) -> Option<Monitor> {
    let ms = display::monitors();
    if let Some(m) = ms.iter().find(|m| m.rect.contains(p.x, p.y)) {
        return Some(m.clone());
    }
    match flags {
        MONITOR_DEFAULTTONULL => None,
        MONITOR_DEFAULTTOPRIMARY => Some(primary()),
        _ => ms.iter().min_by_key(|m| dist2(&m.rect, p)).cloned(),
    }
}

fn window_monitor(hwnd: usize, flags: u32) -> Option<Monitor> {
    let r = match window::get(hwnd) {
        Some(w) => {
            let d = w.data();
            if d.minimized {
                d.normal
            } else {
                drop(d);
                window::window_rect_screen(hwnd)?
            }
        }
        None => return if flags == MONITOR_DEFAULTTONULL { None } else { Some(primary()) },
    };
    from_rect(&r, flags)
}

pub fn work_area_for_window(hwnd: usize) -> RECT {
    window_monitor(hwnd, 2).map(|m| m.work).unwrap_or_else(primary_work_area)
}

pub fn monitor_rect_for_window(hwnd: usize) -> RECT {
    window_monitor(hwnd, 2).map(|m| m.rect).unwrap_or_else(primary_rect)
}

pub unsafe extern "win64" fn MonitorFromWindow(hwnd: HWND, flags: u32) -> usize {
    window_monitor(hwnd as usize, flags).map(|m| handle_of(&m)).unwrap_or(0)
}

pub unsafe extern "win64" fn MonitorFromRect(r: *const RECT, flags: u32) -> usize {
    if r.is_null() {
        return 0;
    }
    let rect = r.read_unaligned();
    from_rect(&rect, flags).map(|m| handle_of(&m)).unwrap_or(0)
}

/// `POINT` is passed by value (8 bytes, one register).
pub unsafe extern "win64" fn MonitorFromPoint(p: u64, flags: u32) -> usize {
    let pt = POINT { x: p as u32 as i32, y: (p >> 32) as u32 as i32 };
    from_point(pt, flags).map(|m| handle_of(&m)).unwrap_or(0)
}

/// MONITORINFO (40) / MONITORINFOEX (A: 72, W: 104).
unsafe fn monitor_info(h: usize, mi: *mut u8, wide: bool) -> BOOL {
    if mi.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let cb = (mi as *const u32).read_unaligned();
    let ex_size = if wide { 104 } else { 72 };
    if cb != 40 && cb != ex_size {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let Some((idx, m)) = monitor_of(h) else {
        set_error(e::INVALID_MONITOR_HANDLE);
        return FALSE;
    };
    (mi.add(4) as *mut RECT).write_unaligned(m.rect);
    (mi.add(20) as *mut RECT).write_unaligned(m.work);
    (mi.add(36) as *mut u32).write_unaligned(if m.primary { MONITORINFOF_PRIMARY } else { 0 });
    if cb == ex_size {
        let name = format!("\\\\.\\DISPLAY{}", idx + 1);
        if wide {
            crate::strings::write_wide_trunc(&name, mi.add(40) as *mut u16, 32);
        } else {
            crate::strings::write_ansi_trunc(&name, mi.add(40), 32);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn GetMonitorInfoW(h: usize, mi: *mut u8) -> BOOL {
    monitor_info(h, mi, true)
}

pub unsafe extern "win64" fn GetMonitorInfoA(h: usize, mi: *mut u8) -> BOOL {
    monitor_info(h, mi, false)
}

type MonitorEnumProc = unsafe extern "win64" fn(usize, usize, *mut RECT, LPARAM) -> BOOL;

const OBJ_DC: u32 = 3;
const OBJ_MEMDC: u32 = 10;
const WINDOW_DC_BASE: usize = 0x0300_0000;
const WINDOW_DC_MASK: usize = 0x00FF_FFFC;

/// Resolve the visible clip box (in virtual-screen coordinates) and DC origin
/// for a non-zero `HDC`. Returns `None` if `hdc` is not a valid GDI DC handle.
unsafe fn dc_clip_and_origin(hdc: usize) -> Option<(RECT, POINT)> {
    let ty = crate::misc::gdi32::GetObjectType(hdc);
    if ty != OBJ_DC && ty != OBJ_MEMDC {
        return None;
    }
    // Window DCs allocated by `gdi32::window_dc(hwnd)` live in
    // `WINDOW_DC_BASE..WINDOW_DC_BASE + 0x0100_0000` with the low bits carrying
    // the associated `HWND` (0 for `GetDC(NULL)` screen DC).
    if (WINDOW_DC_BASE..WINDOW_DC_BASE + 0x0100_0000).contains(&hdc) {
        let hwnd = hdc & WINDOW_DC_MASK;
        if hwnd == 0 || hwnd == window::DESKTOP_HWND {
            return Some((super::misc::virtual_screen(), POINT { x: 0, y: 0 }));
        }
        let origin = window::client_origin_screen(hwnd).unwrap_or_default();
        let (cw, ch) = window::client_size(hwnd).unwrap_or((0, 0));
        let clip_screen = RECT::new(origin.x, origin.y, origin.x + cw.max(0), origin.y + ch.max(0));
        return Some((clip_screen, origin));
    }
    // Memory / display DCs created by `CreateCompatibleDC` / `CreateDC*` / `CreateICW`.
    Some((super::misc::virtual_screen(), POINT { x: 0, y: 0 }))
}

pub unsafe extern "win64" fn EnumDisplayMonitors(hdc: usize, clip: *const RECT, proc_: Option<MonitorEnumProc>, lparam: LPARAM) -> BOOL {
    let Some(f) = proc_ else {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    };
    let (dc_clip_screen, dc_origin) = if hdc != 0 {
        let Some((dc_rect, origin)) = dc_clip_and_origin(hdc) else {
            set_error(crate::types::err::ERROR_INVALID_HANDLE);
            return FALSE;
        };
        if dc_rect.right <= dc_rect.left || dc_rect.bottom <= dc_rect.top {
            return TRUE;
        }
        (Some(dc_rect), origin)
    } else {
        (None, POINT { x: 0, y: 0 })
    };
    // When `hdc != 0`, `lprcClip` is in DC coordinates; convert to virtual-screen
    // coordinates and intersect with the DC's clip box.
    let eff_clip_screen: Option<RECT> = if !clip.is_null() {
        let user_clip = clip.read_unaligned();
        let user_clip_screen = if hdc != 0 {
            user_clip.offset(dc_origin.x, dc_origin.y)
        } else {
            user_clip
        };
        if let Some(dc_rect) = dc_clip_screen {
            match dc_rect.intersect(&user_clip_screen) {
                Some(i) => Some(i),
                None => return TRUE,
            }
        } else {
            if user_clip_screen.right <= user_clip_screen.left || user_clip_screen.bottom <= user_clip_screen.top {
                return TRUE;
            }
            Some(user_clip_screen)
        }
    } else {
        dc_clip_screen
    };
    for m in display::monitors() {
        let mut r = m.rect;
        if let Some(ref c) = eff_clip_screen {
            match r.intersect(c) {
                Some(i) => r = i,
                None => continue,
            }
        }
        // When `hdc != 0`, callback `lprcMonitor` is in DC coordinates.
        if hdc != 0 {
            r = r.offset(-dc_origin.x, -dc_origin.y);
        }
        if crate::call_guest8(f as usize, handle_of(&m) as u64, hdc as u64, (&mut r as *mut RECT) as u64, lparam as u64, 0, 0, 0, 0) as BOOL == FALSE {
            break;
        }
    }
    TRUE
}

/// `EnumDisplaySettingsW` (DEVMODEW, 220 bytes): the current mode only.
pub unsafe extern "win64" fn EnumDisplaySettingsW(device: LPCWSTR, mode: u32, dm: *mut u8) -> BOOL {
    const ENUM_CURRENT_SETTINGS: u32 = u32::MAX;
    const ENUM_REGISTRY_SETTINGS: u32 = u32::MAX - 1;
    if dm.is_null() {
        return FALSE;
    }
    let ms = display::monitors();
    let m = match crate::strings::from_wide_opt(device) {
        Some(n) => {
            let idx = n.trim_start_matches("\\\\.\\DISPLAY").parse::<usize>().ok().and_then(|i| i.checked_sub(1));
            match idx.and_then(|i| ms.get(i)) {
                Some(m) => m.clone(),
                None => return FALSE,
            }
        }
        None => primary(),
    };
    if mode != ENUM_CURRENT_SETTINGS && mode != ENUM_REGISTRY_SETTINGS && mode != 0 {
        return FALSE;
    }
    std::ptr::write_bytes(dm, 0, 220);
    *(dm.add(68) as *mut u16) = 220; // dmSize
    // dmFields: BITSPERPEL | PELSWIDTH | PELSHEIGHT | DISPLAYFREQUENCY | POSITION
    *(dm.add(72) as *mut u32) = 0x0004_0000 | 0x0008_0000 | 0x0010_0000 | 0x0040_0000 | 0x0000_0020;
    *(dm.add(76) as *mut i32) = m.rect.left; // dmPosition.x
    *(dm.add(80) as *mut i32) = m.rect.top;
    *(dm.add(168) as *mut u32) = 32; // dmBitsPerPel
    *(dm.add(172) as *mut u32) = m.rect.width() as u32;
    *(dm.add(176) as *mut u32) = m.rect.height() as u32;
    *(dm.add(184) as *mut u32) = m.refresh_hz;
    TRUE
}

pub unsafe extern "win64" fn EnumDisplaySettingsExW(device: LPCWSTR, mode: u32, dm: *mut u8, _flags: u32) -> BOOL {
    EnumDisplaySettingsW(device, mode, dm)
}

pub unsafe extern "win64" fn EnumDisplaySettingsA(device: LPCSTR, mode: u32, dm: *mut u8) -> BOOL {
    const ENUM_CURRENT_SETTINGS: u32 = u32::MAX;
    const ENUM_REGISTRY_SETTINGS: u32 = u32::MAX - 1;
    if dm.is_null() {
        return FALSE;
    }
    let ms = display::monitors();
    let m = match crate::strings::from_ansi_opt(device) {
        Some(n) => {
            let idx = n.trim_start_matches("\\\\.\\DISPLAY").parse::<usize>().ok().and_then(|i| i.checked_sub(1));
            match idx.and_then(|i| ms.get(i)) {
                Some(m) => m.clone(),
                None => return FALSE,
            }
        }
        None => primary(),
    };
    if mode != ENUM_CURRENT_SETTINGS && mode != ENUM_REGISTRY_SETTINGS && mode != 0 {
        return FALSE;
    }
    // DEVMODEA is 156 bytes (dmSize @ +36, dmFields @ +40, dmPosition @ +44,
    // dmBitsPerPel @ +104, dmPelsWidth @ +108, dmPelsHeight @ +112, dmDisplayFrequency @ +120).
    std::ptr::write_bytes(dm, 0, 156);
    std::ptr::write_unaligned(dm.add(36) as *mut u16, 156);
    std::ptr::write_unaligned(
        dm.add(40) as *mut u32,
        0x0004_0000 | 0x0008_0000 | 0x0010_0000 | 0x0040_0000 | 0x0000_0020,
    );
    std::ptr::write_unaligned(dm.add(44) as *mut i32, m.rect.left);
    std::ptr::write_unaligned(dm.add(48) as *mut i32, m.rect.top);
    std::ptr::write_unaligned(dm.add(104) as *mut u32, 32);
    std::ptr::write_unaligned(dm.add(108) as *mut u32, m.rect.width() as u32);
    std::ptr::write_unaligned(dm.add(112) as *mut u32, m.rect.height() as u32);
    std::ptr::write_unaligned(dm.add(120) as *mut u32, m.refresh_hz);
    TRUE
}

pub unsafe extern "win64" fn EnumDisplaySettingsExA(device: LPCSTR, mode: u32, dm: *mut u8, _flags: u32) -> BOOL {
    EnumDisplaySettingsA(device, mode, dm)
}

fn display_device_info(device: Option<&str>, dev_num: u32) -> Option<(String, String, u32, String, String)> {
    const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 0x1;
    const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 0x4;
    const DISPLAY_DEVICE_ACTIVE: u32 = 0x1;
    let ms = display::monitors();
    if let Some(dev) = device {
        let idx = dev.trim_start_matches("\\\\.\\DISPLAY").parse::<usize>().ok().and_then(|i| i.checked_sub(1))?;
        if idx >= ms.len() || dev_num != 0 {
            return None;
        }
        return Some((
            format!("{dev}\\Monitor0"),
            "Generic PnP Monitor".to_string(),
            DISPLAY_DEVICE_ACTIVE | 0x2,
            format!("MONITOR\\APPL0001\\{{4d36e96e-e325-11ce-bfc1-08002be10318}}\\{:04}", idx),
            format!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Class\\{{4d36e96e-e325-11ce-bfc1-08002be10318}}\\{:04}", idx),
        ));
    }
    let m = ms.get(dev_num as usize)?;
    let mut flags = DISPLAY_DEVICE_ATTACHED_TO_DESKTOP;
    if m.primary {
        flags |= DISPLAY_DEVICE_PRIMARY_DEVICE;
    }
    Some((
        format!("\\\\.\\DISPLAY{}", dev_num + 1),
        "Apple GPU".to_string(),
        flags,
        "PCI\\VEN_106B&DEV_0001&SUBSYS_00000000&REV_00".to_string(),
        format!("\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Video\\{{00000000-0000-0000-0000-000000000001}}\\{:04}", dev_num),
    ))
}

pub unsafe extern "win64" fn EnumDisplayDevicesW(
    device: LPCWSTR,
    dev_num: u32,
    dd: *mut u8,
    _flags: u32,
) -> BOOL {
    if dd.is_null() {
        return FALSE;
    }
    let cb = std::ptr::read_unaligned(dd as *const u32);
    if cb < 840 {
        return FALSE;
    }
    let dev_opt = crate::strings::from_wide_opt(device);
    let Some((name, desc, flags, id, key)) = display_device_info(dev_opt.as_deref(), dev_num) else {
        return FALSE;
    };
    std::ptr::write_bytes(dd.add(4), 0, 836);
    crate::strings::write_wide_trunc(&name, dd.add(4) as *mut u16, 32);
    crate::strings::write_wide_trunc(&desc, dd.add(68) as *mut u16, 128);
    std::ptr::write_unaligned(dd.add(324) as *mut u32, flags);
    crate::strings::write_wide_trunc(&id, dd.add(328) as *mut u16, 128);
    crate::strings::write_wide_trunc(&key, dd.add(584) as *mut u16, 128);
    TRUE
}

pub unsafe extern "win64" fn EnumDisplayDevicesA(
    device: LPCSTR,
    dev_num: u32,
    dd: *mut u8,
    _flags: u32,
) -> BOOL {
    if dd.is_null() {
        return FALSE;
    }
    let cb = std::ptr::read_unaligned(dd as *const u32);
    if cb < 424 {
        return FALSE;
    }
    let dev_opt = crate::strings::from_ansi_opt(device);
    let Some((name, desc, flags, id, key)) = display_device_info(dev_opt.as_deref(), dev_num) else {
        return FALSE;
    };
    std::ptr::write_bytes(dd.add(4), 0, 420);
    crate::strings::write_ansi_buf(&name, dd.add(4), 32);
    crate::strings::write_ansi_buf(&desc, dd.add(36), 128);
    std::ptr::write_unaligned(dd.add(164) as *mut u32, flags);
    crate::strings::write_ansi_buf(&id, dd.add(168), 128);
    crate::strings::write_ansi_buf(&key, dd.add(296), 128);
    TRUE
}

pub unsafe extern "win64" fn ChangeDisplaySettingsW(_dm: *const u8, _flags: u32) -> i32 {
    0 // DISP_CHANGE_SUCCESSFUL
}

pub unsafe extern "win64" fn ChangeDisplaySettingsExW(
    _device: LPCWSTR,
    _dm: *const u8,
    _hwnd: HWND,
    _flags: u32,
    _lparam: *mut std::ffi::c_void,
) -> i32 {
    0 // DISP_CHANGE_SUCCESSFUL
}

// ---------------------------------------------------------------------------
// Display configuration (CCD) API
// ---------------------------------------------------------------------------

const CCD_ADAPTER_LUID_LOW: u32 = 0x0000_063A;
const CCD_ADAPTER_LUID_HIGH: i32 = 0;

const DISPLAYCONFIG_PATH_INFO_SIZE: usize = 72;
const DISPLAYCONFIG_MODE_INFO_SIZE: usize = 64;

const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: u32 = 1;
const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: u32 = 2;

const DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME: u32 = 1;
const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME: u32 = 2;
const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE: u32 = 3;
const DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME: u32 = 4;
const DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE: u32 = 5;
const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE: u32 = 6;
const DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION: u32 = 7;
const DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION: u32 = 8;
const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO: u32 = 9;
const DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE: u32 = 10;
const DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL: u32 = 11;
const DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION: u32 = 12;
const DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION: u32 = 13;
const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2: u32 = 15;
const DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE: u32 = 16;
const DISPLAYCONFIG_DEVICE_INFO_GET_DPI_SCALE: u32 = (-3i32) as u32;

unsafe fn write_wide_fixed(dst: *mut u16, cap: usize, s: &str) {
    if cap == 0 {
        return;
    }
    let w = crate::strings::to_wide_z(s);
    let n = w.len().min(cap);
    std::ptr::write_bytes(dst, 0, cap);
    std::ptr::copy_nonoverlapping(w.as_ptr(), dst, n);
    *dst.add(cap - 1) = 0;
}

pub unsafe extern "win64" fn GetDisplayConfigBufferSizes(_flags: u32, paths: *mut u32, modes: *mut u32) -> i32 {
    if paths.is_null() || modes.is_null() {
        return e::INVALID_PARAMETER as i32;
    }
    let n = display::monitors().len().max(1) as u32;
    std::ptr::write_unaligned(paths, n);
    std::ptr::write_unaligned(modes, n * 2);
    0 // ERROR_SUCCESS
}

pub unsafe extern "win64" fn QueryDisplayConfig(
    _flags: u32,
    np: *mut u32,
    p: *mut u8,
    nm: *mut u32,
    m: *mut u8,
    topo: *mut u32,
) -> i32 {
    if np.is_null() || p.is_null() || nm.is_null() || m.is_null() {
        return e::INVALID_PARAMETER as i32;
    }
    let monitors = display::monitors();
    let n = monitors.len() as u32;
    let cap_p = std::ptr::read_unaligned(np);
    let cap_m = std::ptr::read_unaligned(nm);
    if cap_p < n || cap_m < n * 2 {
        std::ptr::write_unaligned(np, n);
        std::ptr::write_unaligned(nm, n * 2);
        return e::INSUFFICIENT_BUFFER as i32;
    }
    std::ptr::write_bytes(p, 0, (n as usize) * DISPLAYCONFIG_PATH_INFO_SIZE);
    std::ptr::write_bytes(m, 0, (n as usize) * 2 * DISPLAYCONFIG_MODE_INFO_SIZE);

    for (i, mon) in monitors.iter().enumerate() {
        let idx = i as u32;
        let hz = mon.refresh_hz.max(60);
        let w = mon.rect.width().max(1) as u32;
        let h = mon.rect.height().max(1) as u32;
        let src_mode_idx = idx * 2;
        let tgt_mode_idx = idx * 2 + 1;

        // DISPLAYCONFIG_PATH_INFO (72 bytes)
        let pi = p.add(i * DISPLAYCONFIG_PATH_INFO_SIZE);
        // sourceInfo @0..20
        (pi.add(0) as *mut u32).write_unaligned(CCD_ADAPTER_LUID_LOW);
        (pi.add(4) as *mut i32).write_unaligned(CCD_ADAPTER_LUID_HIGH);
        (pi.add(8) as *mut u32).write_unaligned(idx);
        (pi.add(12) as *mut u32).write_unaligned(src_mode_idx);
        (pi.add(16) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_SOURCE_IN_USE
        // targetInfo @20..68
        (pi.add(20) as *mut u32).write_unaligned(CCD_ADAPTER_LUID_LOW);
        (pi.add(24) as *mut i32).write_unaligned(CCD_ADAPTER_LUID_HIGH);
        (pi.add(28) as *mut u32).write_unaligned(idx);
        (pi.add(32) as *mut u32).write_unaligned(tgt_mode_idx);
        (pi.add(36) as *mut u32).write_unaligned(10); // DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL
        (pi.add(40) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_ROTATION_IDENTITY
        (pi.add(44) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_SCALING_IDENTITY
        (pi.add(48) as *mut u32).write_unaligned(hz); // refreshRate.Numerator
        (pi.add(52) as *mut u32).write_unaligned(1); // refreshRate.Denominator
        (pi.add(56) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE
        (pi.add(60) as *mut u32).write_unaligned(1); // targetAvailable = TRUE
        (pi.add(64) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_TARGET_IN_USE
        // flags @68
        (pi.add(68) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_PATH_ACTIVE

        // DISPLAYCONFIG_MODE_INFO[src_mode_idx] (SOURCE, 64 bytes)
        let sm = m.add((src_mode_idx as usize) * DISPLAYCONFIG_MODE_INFO_SIZE);
        (sm.add(0) as *mut u32).write_unaligned(DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE);
        (sm.add(4) as *mut u32).write_unaligned(idx);
        (sm.add(8) as *mut u32).write_unaligned(CCD_ADAPTER_LUID_LOW);
        (sm.add(12) as *mut i32).write_unaligned(CCD_ADAPTER_LUID_HIGH);
        (sm.add(16) as *mut u32).write_unaligned(w);
        (sm.add(20) as *mut u32).write_unaligned(h);
        (sm.add(24) as *mut u32).write_unaligned(4); // DISPLAYCONFIG_PIXELFORMAT_32BPP
        (sm.add(28) as *mut i32).write_unaligned(mon.rect.left);
        (sm.add(32) as *mut i32).write_unaligned(mon.rect.top);

        // DISPLAYCONFIG_MODE_INFO[tgt_mode_idx] (TARGET, 64 bytes)
        let tm = m.add((tgt_mode_idx as usize) * DISPLAYCONFIG_MODE_INFO_SIZE);
        (tm.add(0) as *mut u32).write_unaligned(DISPLAYCONFIG_MODE_INFO_TYPE_TARGET);
        (tm.add(4) as *mut u32).write_unaligned(idx);
        (tm.add(8) as *mut u32).write_unaligned(CCD_ADAPTER_LUID_LOW);
        (tm.add(12) as *mut i32).write_unaligned(CCD_ADAPTER_LUID_HIGH);
        (tm.add(16) as *mut u64).write_unaligned((w as u64) * (h as u64) * (hz as u64)); // pixelRate
        (tm.add(24) as *mut u32).write_unaligned(h.saturating_mul(hz)); // hSyncFreq.Numerator
        (tm.add(28) as *mut u32).write_unaligned(1);
        (tm.add(32) as *mut u32).write_unaligned(hz); // vSyncFreq.Numerator
        (tm.add(36) as *mut u32).write_unaligned(1);
        (tm.add(40) as *mut u32).write_unaligned(w); // activeSize.cx
        (tm.add(44) as *mut u32).write_unaligned(h); // activeSize.cy
        (tm.add(48) as *mut u32).write_unaligned(w); // totalSize.cx
        (tm.add(52) as *mut u32).write_unaligned(h); // totalSize.cy
        (tm.add(56) as *mut u32).write_unaligned(255); // D3DKMDT_VSS_OTHER
        (tm.add(60) as *mut u32).write_unaligned(1); // DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE
    }

    std::ptr::write_unaligned(np, n);
    std::ptr::write_unaligned(nm, n * 2);
    if !topo.is_null() {
        std::ptr::write_unaligned(topo, if n > 1 { 4 } else { 1 }); // EXTEND vs INTERNAL
    }
    0 // ERROR_SUCCESS
}

pub unsafe extern "win64" fn DisplayConfigGetDeviceInfo(pkt: *mut u8) -> i32 {
    if pkt.is_null() {
        return e::INVALID_PARAMETER as i32;
    }
    let ty = (pkt as *const u32).read_unaligned();
    let sz = (pkt.add(4) as *const u32).read_unaligned() as usize;
    let id = (pkt.add(16) as *const u32).read_unaligned() as usize;
    let monitors = display::monitors();
    let mon = monitors.get(id).or_else(|| monitors.first());
    match ty {
        DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME => {
            if sz < 20 + 64 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            let dev = format!("\\\\.\\DISPLAY{}", id + 1);
            write_wide_fixed(pkt.add(20) as *mut u16, 32, &dev);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME => {
            if sz < 420 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            let friendly = mon.map(|m| m.name.as_str()).filter(|s| !s.is_empty()).unwrap_or("Apple Display");
            (pkt.add(20) as *mut u32).write_unaligned(1); // friendlyNameFromEdid = 1
            (pkt.add(24) as *mut u32).write_unaligned(10); // DISPLAYPORT_EXTERNAL
            (pkt.add(28) as *mut u16).write_unaligned(0x0610); // Apple EDID manufacturer ID
            (pkt.add(30) as *mut u16).write_unaligned((id as u16).wrapping_add(1));
            (pkt.add(32) as *mut u32).write_unaligned(id as u32);
            write_wide_fixed(pkt.add(36) as *mut u16, 64, friendly);
            let dev_path = format!("\\\\?\\DISPLAY#APP0001#4&1a2b3c&0&UID{}#{{e6f07b5f-ee97-4a90-b076-33f57bf4eaa7}}", id + 1);
            write_wide_fixed(pkt.add(164) as *mut u16, 128, &dev_path);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE => {
            // DISPLAYCONFIG_TARGET_PREFERRED_MODE (80 bytes):
            // header @0..20, width @20, height @24, 4B alignment pad @28..32,
            // DISPLAYCONFIG_TARGET_MODE targetMode @32..80 (8-byte aligned for u64 pixelRate).
            if sz < 80 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            let (w, h, hz) = mon.map_or((1920, 1080, 60), |m| {
                (m.rect.width().max(1) as u32, m.rect.height().max(1) as u32, m.refresh_hz.max(60))
            });
            (pkt.add(20) as *mut u32).write_unaligned(w);
            (pkt.add(24) as *mut u32).write_unaligned(h);
            (pkt.add(28) as *mut u32).write_unaligned(0);
            // targetMode.targetVideoSignalInfo @32 (48 bytes)
            (pkt.add(32) as *mut u64).write_unaligned((w as u64) * (h as u64) * (hz as u64));
            (pkt.add(40) as *mut u32).write_unaligned(h.saturating_mul(hz));
            (pkt.add(44) as *mut u32).write_unaligned(1);
            (pkt.add(48) as *mut u32).write_unaligned(hz);
            (pkt.add(52) as *mut u32).write_unaligned(1);
            (pkt.add(56) as *mut u32).write_unaligned(w);
            (pkt.add(60) as *mut u32).write_unaligned(h);
            (pkt.add(64) as *mut u32).write_unaligned(w);
            (pkt.add(68) as *mut u32).write_unaligned(h);
            (pkt.add(72) as *mut u32).write_unaligned(255);
            (pkt.add(76) as *mut u32).write_unaligned(1);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME => {
            if sz < 20 + 256 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            write_wide_fixed(pkt.add(20) as *mut u16, 128, "\\\\?\\PCI#VEN_106B&DEV_0001&SUBSYS_00000000&REV_00#3&11583659&0&00");
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE => {
            // DISPLAYCONFIG_TARGET_BASE_TYPE (24 bytes): header @0..20, baseOutputTechnology @20.
            if sz < 24 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            let is_builtin = mon.is_some_and(|m| m.name.contains("Built-in") || m.name.contains("Retina"));
            let tech: u32 = if is_builtin { 0x8000_0000 } else { 10 }; // INTERNAL vs DISPLAYPORT_EXTERNAL
            (pkt.add(20) as *mut u32).write_unaligned(tech);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION => {
            // DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION (24 bytes): header @0..20, disableMonitorVirtualResolution @20.
            if sz < 24 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            (pkt.add(20) as *mut u32).write_unaligned(0);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO => {
            if sz < 32 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            (pkt.add(20) as *mut u32).write_unaligned(0); // HDR off
            (pkt.add(24) as *mut u32).write_unaligned(0); // DISPLAYCONFIG_COLOR_ENCODING_RGB
            (pkt.add(28) as *mut u32).write_unaligned(8); // 8 bpc
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL => {
            if sz < 24 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            (pkt.add(20) as *mut u32).write_unaligned(1000); // 80 nits standard SDR white
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION => {
            // DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION (24 bytes): header @0..20, flags @20.
            if sz < 24 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            (pkt.add(20) as *mut u32).write_unaligned(0);
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2 => {
            // DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2 (36 bytes, Win11 24H2):
            // header @0..20, colorCapabilities @20, colorEncoding @24, bitsPerColorChannel @28, activeColorMode @32.
            if sz < 36 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            (pkt.add(20) as *mut u32).write_unaligned(0); // SDR only
            (pkt.add(24) as *mut u32).write_unaligned(0); // RGB
            (pkt.add(28) as *mut u32).write_unaligned(8); // 8 bpc
            (pkt.add(32) as *mut u32).write_unaligned(0); // DISPLAYCONFIG_ADVANCED_COLOR_MODE_SDR
            0
        }
        DISPLAYCONFIG_DEVICE_INFO_GET_DPI_SCALE => {
            // Undocumented CCD DPI scale query (32 bytes: header @0..20, minScaleRel @20, curScaleRel @24, maxScaleRel @28).
            if sz < 32 {
                return e::INSUFFICIENT_BUFFER as i32;
            }
            let dpi = mon.map_or(96u32, |m| m.dpi().max(96));
            // Relative steps from recommended 100% (96 DPI): 0 = 100%, 1 = 125%, 2 = 150%, 3 = 175%, 4 = 200%.
            let steps = ((dpi.saturating_sub(96) + 12) / 24).min(4) as i32;
            (pkt.add(20) as *mut i32).write_unaligned(-steps);
            (pkt.add(24) as *mut i32).write_unaligned(0);
            (pkt.add(28) as *mut i32).write_unaligned(4 - steps);
            0
        }
        _ => {
            // Microsoft Learn DisplayConfigGetDeviceInfo: unrecognized request types return ERROR_INVALID_PARAMETER.
            e::INVALID_PARAMETER as i32
        }
    }
}

/// Native-unsupported on macOS/iOS: WDDM/CCD display hardware state mutations
/// (`SET_TARGET_PERSISTENCE`, `SET_SUPPORT_VIRTUAL_RESOLUTION`, `SET_ADVANCED_COLOR_STATE`,
/// `SET_MONITOR_SPECIALIZATION`, `SET_HDR_STATE`) have no global app-writable equivalent on
/// macOS (Metal configures EDR per-layer). Matches Microsoft Learn `DisplayConfigSetDeviceInfo`:
/// returns `ERROR_INVALID_PARAMETER` for null/undersized/unknown packets and `ERROR_NOT_SUPPORTED`
/// when the display driver does not support the requested set operation.
pub unsafe extern "win64" fn DisplayConfigSetDeviceInfo(pkt: *const u32) -> i32 {
    if pkt.is_null() {
        return e::INVALID_PARAMETER as i32;
    }
    let ty = pkt.read_unaligned();
    let sz = pkt.add(1).read_unaligned() as usize;
    if sz < 20 {
        return e::INVALID_PARAMETER as i32;
    }
    match ty {
        DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE
        | DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION
        | DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE
        | DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION
        | DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE => {
            if sz < 24 {
                return e::INVALID_PARAMETER as i32;
            }
            e::NOT_SUPPORTED as i32
        }
        _ => e::INVALID_PARAMETER as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_handles_and_info() {
        unsafe {
            let h = MonitorFromPoint(0, MONITOR_DEFAULTTOPRIMARY);
            assert_ne!(h, 0);
            let mut buf = [0u8; 104];
            *(buf.as_mut_ptr() as *mut u32) = 104;
            assert_eq!(GetMonitorInfoW(h, buf.as_mut_ptr()), TRUE);
            let r = *(buf.as_ptr().add(4) as *const RECT);
            assert!(r.width() > 0 && r.height() > 0);
            assert_eq!(*(buf.as_ptr().add(36) as *const u32), MONITORINFOF_PRIMARY);
            assert_eq!(crate::strings::from_wide(buf.as_ptr().add(40) as *const u16), "\\\\.\\DISPLAY1");
            // A far-away rect maps to nothing with DEFAULTTONULL.
            let far = RECT::new(-100000, -100000, -99990, -99990);
            assert_eq!(MonitorFromRect(&far, MONITOR_DEFAULTTONULL), 0);
            assert_eq!(MonitorFromRect(&far, 2), h);
            extern "win64" fn cb(_h: usize, _dc: usize, _r: *mut RECT, lp: LPARAM) -> BOOL {
                unsafe { *(lp as *mut u32) += 1 };
                TRUE
            }
            let mut n = 0u32;
            assert_eq!(EnumDisplayMonitors(0, std::ptr::null(), Some(cb), &mut n as *mut u32 as LPARAM), TRUE);
            assert!(n >= 1);
        }
    }

    #[test]
    fn mixed_dpi_monitors_report_distinct_dpi() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = display::build_displays(
            &[
                display::ScreenDesc {
                    id: 901,
                    frame_pt: (0.0, 0.0, 1512.0, 982.0),
                    visible_pt: (0.0, 0.0, 1512.0, 950.0),
                    backing_scale: 2.0,
                    refresh_hz: 120,
                    name: "Retina".into(),
                },
                display::ScreenDesc {
                    id: 902,
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
        display::with_test_displays(d, || unsafe {
            let pt_retina: u64 = (100u32 as u64) | ((100u32 as u64) << 32);
            let pt_ext: u64 = (3200u32 as u64) | ((200u32 as u64) << 32);
            let h0 = MonitorFromPoint(pt_retina, MONITOR_DEFAULTTONULL);
            let h1 = MonitorFromPoint(pt_ext, MONITOR_DEFAULTTONULL);
            assert_ne!(h0, 0);
            assert_ne!(h1, 0);
            assert_ne!(h0, h1);

            let (mut dx0, mut dy0) = (0u32, 0u32);
            let (mut dx1, mut dy1) = (0u32, 0u32);
            assert_eq!(super::misc::GetDpiForMonitor(h0, 0, &mut dx0, &mut dy0), S_OK);
            assert_eq!(super::misc::GetDpiForMonitor(h1, 0, &mut dx1, &mut dy1), S_OK);
            assert_eq!((dx0, dy0), (192, 192));
            assert_eq!((dx1, dy1), (96, 96));

            // Invalid HMONITOR returns E_INVALIDARG.
            assert_eq!(super::misc::GetDpiForMonitor(0xDEAD, 0, &mut dx0, &mut dy0), E_INVALIDARG);

            // GetMonitorInfoW reports each monitor's native rect and primary flag.
            let mut info0 = [0u8; 104];
            (info0.as_mut_ptr() as *mut u32).write_unaligned(104);
            assert_eq!(GetMonitorInfoW(h0, info0.as_mut_ptr()), TRUE);
            assert_eq!((info0.as_ptr().add(4) as *const RECT).read_unaligned(), RECT::new(0, 0, 3024, 1964));
            assert_eq!((info0.as_ptr().add(36) as *const u32).read_unaligned(), MONITORINFOF_PRIMARY);

            let mut info1 = [0u8; 104];
            (info1.as_mut_ptr() as *mut u32).write_unaligned(104);
            assert_eq!(GetMonitorInfoW(h1, info1.as_mut_ptr()), TRUE);
            assert_eq!((info1.as_ptr().add(4) as *const RECT).read_unaligned(), RECT::new(3024, 0, 4944, 1080));
            assert_eq!((info1.as_ptr().add(36) as *const u32).read_unaligned(), 0);

            // MonitorFromRect picks the monitor with larger overlap.
            let r_ext = RECT::new(3100, 100, 4100, 700);
            assert_eq!(MonitorFromRect(&r_ext, MONITOR_DEFAULTTONULL), h1);

            // CCD Display Config reports both monitors cleanly.
            let (mut np, mut nm) = (0u32, 0u32);
            assert_eq!(GetDisplayConfigBufferSizes(2, &mut np, &mut nm), 0);
            assert_eq!((np, nm), (2, 4));
            let mut paths = vec![0u8; (np as usize) * DISPLAYCONFIG_PATH_INFO_SIZE];
            let mut modes = vec![0u8; (nm as usize) * DISPLAYCONFIG_MODE_INFO_SIZE];
            assert_eq!(QueryDisplayConfig(2, &mut np, paths.as_mut_ptr(), &mut nm, modes.as_mut_ptr(), std::ptr::null_mut()), 0);
            assert_eq!((modes.as_ptr().add(16) as *const u32).read_unaligned(), 3024);
            assert_eq!((modes.as_ptr().add(20) as *const u32).read_unaligned(), 1964);
            let mut tgt = [0u8; 420];
            (tgt.as_mut_ptr() as *mut u32).write_unaligned(DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME);
            (tgt.as_mut_ptr().add(4) as *mut u32).write_unaligned(420);
            (tgt.as_mut_ptr().add(16) as *mut u32).write_unaligned(1);
            assert_eq!(DisplayConfigGetDeviceInfo(tgt.as_mut_ptr()), 0);
            assert_eq!(crate::strings::from_wide(tgt.as_ptr().add(36) as *const u16), "External");
            let mut pref = [0u8; 80];
            (pref.as_mut_ptr() as *mut u32).write_unaligned(DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE);
            (pref.as_mut_ptr().add(4) as *mut u32).write_unaligned(80);
            (pref.as_mut_ptr().add(16) as *mut u32).write_unaligned(1);
            assert_eq!(DisplayConfigGetDeviceInfo(pref.as_mut_ptr()), 0);
            assert_eq!((pref.as_ptr().add(20) as *const u32).read_unaligned(), 1920);
            assert_eq!((pref.as_ptr().add(24) as *const u32).read_unaligned(), 1080);
            assert_eq!((pref.as_ptr().add(48) as *const u32).read_unaligned(), 60);
            assert_eq!((pref.as_ptr().add(52) as *const u32).read_unaligned(), 1);
            assert_eq!((pref.as_ptr().add(56) as *const u32).read_unaligned(), 1920);
            assert_eq!((pref.as_ptr().add(60) as *const u32).read_unaligned(), 1080);
        });
    }

    #[test]
    fn enum_display_monitors_hdc_clipping() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = display::build_displays(
            &[
                display::ScreenDesc {
                    id: 951,
                    frame_pt: (0.0, 0.0, 1000.0, 800.0),
                    visible_pt: (0.0, 0.0, 1000.0, 800.0),
                    backing_scale: 1.0,
                    refresh_hz: 60,
                    name: "Left".into(),
                },
                display::ScreenDesc {
                    id: 952,
                    frame_pt: (1000.0, 0.0, 1000.0, 800.0),
                    visible_pt: (1000.0, 0.0, 1000.0, 800.0),
                    backing_scale: 1.0,
                    refresh_hz: 60,
                    name: "Right".into(),
                },
            ],
            0.0,
            false,
        );

        unsafe extern "win64" fn collect(hmon: usize, hdc: usize, rc: *mut RECT, lp: LPARAM) -> BOOL {
            let out = &mut *(lp as *mut Vec<(usize, usize, RECT)>);
            out.push((hmon, hdc, rc.read_unaligned()));
            TRUE
        }

        display::with_test_displays(d, || {
            std::thread::spawn(|| unsafe {
                let cls = crate::strings::to_wide_z("EnumMonHdcTestCls");
                let wc = super::class::WNDCLASSEX {
                    cb_size: std::mem::size_of::<super::class::WNDCLASSEX>() as u32,
                    style: 0,
                    wndproc: super::WNDPROC_HANDLE_DEF_WINDOW_W,
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
                let _ = super::class::RegisterClassExW(&wc);

                // 1. Borderless popup window straddling both monitors: [900, 100, 1300, 500] (400x400).
                let hwnd_span = window::CreateWindowExW(
                    0,
                    cls.as_ptr(),
                    std::ptr::null(),
                    ws::POPUP,
                    900,
                    100,
                    400,
                    400,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                assert!(!hwnd_span.is_null());
                let hdc_span = window::GetDC(hwnd_span);
                let mut got: Vec<(usize, usize, RECT)> = Vec::new();
                assert_eq!(
                    EnumDisplayMonitors(hdc_span, std::ptr::null(), Some(collect), &mut got as *mut _ as LPARAM),
                    TRUE
                );
                assert_eq!(got.len(), 2);
                assert_eq!(got[0].1, hdc_span);
                assert_eq!(got[0].2, RECT::new(0, 0, 100, 400)); // left monitor in DC coords
                assert_eq!(got[1].1, hdc_span);
                assert_eq!(got[1].2, RECT::new(100, 0, 400, 400)); // right monitor in DC coords

                // 2. Same window DC intersected with lprcClip in DC coords ([150, 50, 350, 250]) -> only right monitor!
                got.clear();
                let clip_dc = RECT::new(150, 50, 350, 250);
                assert_eq!(
                    EnumDisplayMonitors(hdc_span, &clip_dc, Some(collect), &mut got as *mut _ as LPARAM),
                    TRUE
                );
                assert_eq!(got.len(), 1);
                assert_eq!(got[0].2, RECT::new(150, 50, 350, 250));

                // 3. Window entirely on left monitor [50, 50, 350, 250] -> only left monitor enumerated.
                let hwnd_left = window::CreateWindowExW(
                    0,
                    cls.as_ptr(),
                    std::ptr::null(),
                    ws::POPUP,
                    50,
                    50,
                    300,
                    200,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                assert!(!hwnd_left.is_null());
                let hdc_left = window::GetDC(hwnd_left);
                got.clear();
                assert_eq!(
                    EnumDisplayMonitors(hdc_left, std::ptr::null(), Some(collect), &mut got as *mut _ as LPARAM),
                    TRUE
                );
                assert_eq!(got.len(), 1);
                assert_eq!(got[0].2, RECT::new(0, 0, 300, 200));

                // 4. Screen DC (GetDC(NULL)) with lprcClip spanning only right monitor.
                let hdc_screen = window::GetDC(std::ptr::null_mut());
                let right_clip = RECT::new(1100, 200, 1400, 600);
                got.clear();
                assert_eq!(
                    EnumDisplayMonitors(hdc_screen, &right_clip, Some(collect), &mut got as *mut _ as LPARAM),
                    TRUE
                );
                assert_eq!(got.len(), 1);
                assert_eq!(got[0].2, right_clip);

                // 5. Invalid non-zero HDC returns FALSE.
                got.clear();
                assert_eq!(
                    EnumDisplayMonitors(0xDEAD_BEEF, std::ptr::null(), Some(collect), &mut got as *mut _ as LPARAM),
                    FALSE
                );
                assert!(got.is_empty());

                let _ = window::DestroyWindow(hwnd_span);
                let _ = window::DestroyWindow(hwnd_left);
            })
            .join()
            .unwrap();
        });
    }

    #[test]
    fn display_config_get_and_set_device_info() {
        unsafe {
            // GET_TARGET_BASE_TYPE (6)
            let mut base_pkt = [0u32; 6];
            base_pkt[0] = DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE;
            base_pkt[1] = 24;
            assert_eq!(DisplayConfigGetDeviceInfo(base_pkt.as_mut_ptr() as *mut u8), 0);
            assert!(base_pkt[5] == 10 || base_pkt[5] == 0x8000_0000);

            // GET_SUPPORT_VIRTUAL_RESOLUTION (7)
            let mut vr_pkt = [0u32; 6];
            vr_pkt[0] = DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION;
            vr_pkt[1] = 24;
            assert_eq!(DisplayConfigGetDeviceInfo(vr_pkt.as_mut_ptr() as *mut u8), 0);

            // GET_MONITOR_SPECIALIZATION (12)
            let mut spec_pkt = [0u32; 6];
            spec_pkt[0] = DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION;
            spec_pkt[1] = 24;
            assert_eq!(DisplayConfigGetDeviceInfo(spec_pkt.as_mut_ptr() as *mut u8), 0);

            // GET_ADVANCED_COLOR_INFO_2 (15)
            let mut ac2_pkt = [0u32; 9];
            ac2_pkt[0] = DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2;
            ac2_pkt[1] = 36;
            assert_eq!(DisplayConfigGetDeviceInfo(ac2_pkt.as_mut_ptr() as *mut u8), 0);
            assert_eq!(ac2_pkt[7], 8); // 8 bpc

            // GET_DPI_SCALE (-3)
            let mut dpi_pkt = [0u32; 8];
            dpi_pkt[0] = DISPLAYCONFIG_DEVICE_INFO_GET_DPI_SCALE;
            dpi_pkt[1] = 32;
            assert_eq!(DisplayConfigGetDeviceInfo(dpi_pkt.as_mut_ptr() as *mut u8), 0);

            // Unknown GET type returns ERROR_INVALID_PARAMETER.
            let mut bad_pkt = [0u32; 8];
            bad_pkt[0] = 9999;
            bad_pkt[1] = 32;
            assert_eq!(DisplayConfigGetDeviceInfo(bad_pkt.as_mut_ptr() as *mut u8), e::INVALID_PARAMETER as i32);

            // SET_* returns ERROR_NOT_SUPPORTED (native-unsupported on macOS) for valid CCD packets,
            // and ERROR_INVALID_PARAMETER for null/undersized/unknown packets.
            assert_eq!(DisplayConfigSetDeviceInfo(std::ptr::null()), e::INVALID_PARAMETER as i32);
            let mut set_hdr = [0u32; 6];
            set_hdr[0] = DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE;
            set_hdr[1] = 24;
            assert_eq!(DisplayConfigSetDeviceInfo(set_hdr.as_ptr()), e::NOT_SUPPORTED as i32);
            set_hdr[1] = 16; // undersized
            assert_eq!(DisplayConfigSetDeviceInfo(set_hdr.as_ptr()), e::INVALID_PARAMETER as i32);
        }
    }
}
