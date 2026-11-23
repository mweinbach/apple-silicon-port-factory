//! uxtheme.dll / comctl32.dll: Desktop Visual Styles (honest unthemed fallback),
//! common controls initialization, and `_TrackMouseEvent` per-HWND state tracking.

use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (17, "InitCommonControls"),
    (345, "TaskDialogIndirect"),
    (410, "SetWindowSubclass"),
    (411, "GetWindowSubclass"),
    (412, "RemoveWindowSubclass"),
    (413, "DefSubclassProc"),
];

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

// ---------------------------------------------------------------------------
// comctl32 window subclassing (ordinals 410..=413)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct SubclassNode {
    seq: u64,
    pfn: usize,
    id: usize,
    ref_data: usize,
}

struct SubclassHeader {
    orig_wndproc: usize,
    next_seq: u64,
    /// Ordered oldest (bottom) -> newest (top), strictly increasing `seq`.
    nodes: Vec<SubclassNode>,
    active_calls: u32,
}

#[derive(Clone, Copy, Debug)]
struct ActiveSubclassFrame {
    hwnd: usize,
    /// Upper bound for the next node to dispatch (`u64::MAX` before entering top).
    last_seq: u64,
    /// Snapshot of the original base WNDPROC captured when entering the master proc.
    orig_wndproc: usize,
}

fn subclass_table() -> &'static Mutex<HashMap<usize, SubclassHeader>> {
    static T: OnceLock<Mutex<HashMap<usize, SubclassHeader>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

thread_local! {
    static ACTIVE_SUBCLASS_CALLS: std::cell::RefCell<Vec<ActiveSubclassFrame>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

fn master_wndproc_va() -> usize {
    let va = master_subclass_wndproc as *const () as usize;
    crate::register_native_va(va as u64, va);
    va
}

/// Remove any remaining subclass state for a destroyed window (called from
/// `user32::window::free_window` as well as after `WM_NCDESTROY`).
pub(crate) fn forget_window_subclasses(hwnd: usize) {
    subclass_table().lock().unwrap_or_else(|e| e.into_inner()).remove(&hwnd);
}

fn try_detach_header_locked(
    map: &mut HashMap<usize, SubclassHeader>,
    hwnd: usize,
) -> Option<usize> {
    let should_remove = map
        .get(&hwnd)
        .is_some_and(|h| h.nodes.is_empty() && h.active_calls == 0);
    if should_remove {
        map.remove(&hwnd).map(|h| h.orig_wndproc)
    } else {
        None
    }
}

unsafe fn restore_orig_wndproc_if_master(hwnd: usize, orig_wndproc: usize) {
    if orig_wndproc == 0 || !crate::user32::window::exists(hwnd) {
        return;
    }
    let master = master_wndproc_va();
    if let Some(w) = crate::user32::window::get(hwnd) {
        let mut d = w.data();
        if d.wndproc == master {
            d.wndproc = orig_wndproc;
        }
    }
}

unsafe extern "win64" fn master_subclass_wndproc(
    hwnd: HWND,
    u_msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    let h = hwnd as usize;
    let orig_wndproc = {
        let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        match map.get_mut(&h) {
            Some(hdr) => {
                hdr.active_calls = hdr.active_calls.saturating_add(1);
                hdr.orig_wndproc
            }
            None => {
                drop(map);
                return crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param);
            }
        }
    };

    ACTIVE_SUBCLASS_CALLS.with(|stack| {
        stack.borrow_mut().push(ActiveSubclassFrame {
            hwnd: h,
            last_seq: u64::MAX,
            orig_wndproc,
        });
    });

    struct MasterExitGuard {
        hwnd: usize,
        is_ncdestroy: bool,
    }
    impl Drop for MasterExitGuard {
        fn drop(&mut self) {
            ACTIVE_SUBCLASS_CALLS.with(|stack| {
                stack.borrow_mut().pop();
            });
            let restore = {
                let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
                if self.is_ncdestroy {
                    map.remove(&self.hwnd);
                    None
                } else if let Some(hdr) = map.get_mut(&self.hwnd) {
                    hdr.active_calls = hdr.active_calls.saturating_sub(1);
                    try_detach_header_locked(&mut map, self.hwnd)
                } else {
                    None
                }
            };
            if let Some(orig) = restore {
                unsafe { restore_orig_wndproc_if_master(self.hwnd, orig) };
            }
        }
    }

    let _guard = MasterExitGuard {
        hwnd: h,
        is_ncdestroy: u_msg == crate::user32::wm::NCDESTROY,
    };

    DefSubclassProc(hwnd, u_msg, w_param, l_param)
}

pub unsafe extern "win64" fn SetWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
    dw_ref_data: usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let master = master_wndproc_va();
    let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
    let hdr = map.entry(h).or_insert_with(|| {
        let orig = crate::user32::window::get(h)
            .map(|w| {
                let mut d = w.data();
                let prev = d.wndproc;
                d.wndproc = master;
                prev
            })
            .unwrap_or(0);
        SubclassHeader {
            orig_wndproc: orig,
            next_seq: 1,
            nodes: Vec::new(),
            active_calls: 0,
        }
    });

    // Updating an existing (pfn, id) pair updates dwRefData in place without changing order.
    if let Some(existing) = hdr
        .nodes
        .iter_mut()
        .find(|n| n.pfn == pfn_subclass && n.id == u_id_subclass)
    {
        existing.ref_data = dw_ref_data;
        return TRUE;
    }

    let seq = hdr.next_seq;
    hdr.next_seq = hdr.next_seq.saturating_add(1);
    hdr.nodes.push(SubclassNode {
        seq,
        pfn: pfn_subclass,
        id: u_id_subclass,
        ref_data: dw_ref_data,
    });
    TRUE
}

pub unsafe extern "win64" fn GetWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
    pdw_ref_data: *mut usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(node) = map
        .get(&h)
        .and_then(|hdr| hdr.nodes.iter().find(|n| n.pfn == pfn_subclass && n.id == u_id_subclass))
    {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(node.ref_data);
        }
        TRUE
    } else {
        if !pdw_ref_data.is_null() {
            pdw_ref_data.write_unaligned(0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn RemoveWindowSubclass(
    hwnd: HWND,
    pfn_subclass: usize,
    u_id_subclass: usize,
) -> BOOL {
    let h = hwnd as usize;
    if h == 0 || !crate::user32::window::exists(h) {
        crate::teb::set_last_error(err::ERROR_INVALID_WINDOW_HANDLE);
        return FALSE;
    }
    if pfn_subclass == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let (removed, restore_orig) = {
        let mut map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        let Some(hdr) = map.get_mut(&h) else {
            return FALSE;
        };
        let Some(pos) = hdr
            .nodes
            .iter()
            .position(|n| n.pfn == pfn_subclass && n.id == u_id_subclass)
        else {
            return FALSE;
        };
        hdr.nodes.remove(pos);
        let restore = try_detach_header_locked(&mut map, h);
        (true, restore)
    };

    if let Some(orig) = restore_orig {
        restore_orig_wndproc_if_master(h, orig);
    }
    removed as BOOL
}

pub unsafe extern "win64" fn DefSubclassProc(
    hwnd: HWND,
    u_msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    let h = hwnd as usize;
    let frame_info: Option<(usize, u64, usize)> = ACTIVE_SUBCLASS_CALLS.with(|stack| {
        let s = stack.borrow();
        s.iter()
            .enumerate()
            .rev()
            .find(|(_, f)| f.hwnd == h)
            .map(|(idx, f)| (idx, f.last_seq, f.orig_wndproc))
    });

    let Some((frame_idx, saved_last_seq, frame_orig)) = frame_info else {
        return crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param);
    };

    // Find the highest-seq subclass strictly below `saved_last_seq`.
    let next_node: Option<SubclassNode> = {
        let map = subclass_table().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&h).and_then(|hdr| {
            hdr.nodes
                .iter()
                .rev()
                .find(|n| n.seq < saved_last_seq)
                .copied()
        })
    };

    struct RestoreSeqGuard {
        frame_idx: usize,
        saved_last_seq: u64,
    }
    impl Drop for RestoreSeqGuard {
        fn drop(&mut self) {
            ACTIVE_SUBCLASS_CALLS.with(|stack| {
                let mut s = stack.borrow_mut();
                if let Some(frame) = s.get_mut(self.frame_idx) {
                    frame.last_seq = self.saved_last_seq;
                }
            });
        }
    }

    let next_seq = next_node.map(|n| n.seq).unwrap_or(0);
    ACTIVE_SUBCLASS_CALLS.with(|stack| {
        if let Some(frame) = stack.borrow_mut().get_mut(frame_idx) {
            frame.last_seq = next_seq;
        }
    });
    let _restore = RestoreSeqGuard {
        frame_idx,
        saved_last_seq,
    };

    if let Some(node) = next_node {
        // All locks are dropped before invoking guest SUBCLASSPROC (6 args).
        crate::call_guest8(
            node.pfn,
            h as u64,
            u_msg as u64,
            w_param as u64,
            l_param as u64,
            node.id as u64,
            node.ref_data as u64,
            0,
            0,
        ) as isize
    } else {
        let master = master_wndproc_va();
        if frame_orig != 0 && frame_orig != master {
            crate::call_guest8(
                frame_orig,
                h as u64,
                u_msg as u64,
                w_param as u64,
                l_param as u64,
                0,
                0,
                0,
                0,
            ) as isize
        } else {
            crate::user32::defproc::DefWindowProcW(hwnd, u_msg, w_param, l_param)
        }
    }
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

    #[test]
    fn comctl32_subclass_ordinals_chaining_reentrancy_and_cleanup() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!crate::macos::active(), "unit tests must stay headless");

        // Verify ordinal resolution 410..=413 via comctl32.dll -> uxtheme.dll alias.
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 410).unwrap() as usize,
            SetWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 411).unwrap() as usize,
            GetWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 412).unwrap() as usize,
            RemoveWindowSubclass as *const () as usize
        );
        assert_eq!(
            crate::lookup_ordinal("comctl32.dll", 413).unwrap() as usize,
            DefSubclassProc as *const () as usize
        );

        static TRACE: Mutex<Vec<String>> = Mutex::new(Vec::new());
        const WM_PING: u32 = crate::user32::wm::USER + 1;
        const WM_REENTRANT: u32 = crate::user32::wm::USER + 2;
        const WM_REMOVE_SELF: u32 = crate::user32::wm::USER + 3;
        const WM_REMOVE_NEIGHBOR: u32 = crate::user32::wm::USER + 4;

        unsafe extern "win64" fn base_wndproc(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("base:{msg:#x}:{wp:#x}:{lp:#x}"));
            }
            if msg == WM_PING {
                return 100;
            }
            crate::user32::defproc::DefWindowProcW(hwnd, msg, wp, lp)
        }

        unsafe extern "win64" fn sub_a(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("A:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 1 } else { r }
        }

        unsafe extern "win64" fn sub_b(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("B:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            if msg == WM_REMOVE_SELF {
                assert_eq!(RemoveWindowSubclass(hwnd, sub_b as *const () as usize, uid), TRUE);
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 10 } else { r }
        }

        unsafe extern "win64" fn sub_c(
            hwnd: HWND,
            msg: u32,
            wp: usize,
            lp: isize,
            uid: usize,
            ref_data: usize,
        ) -> isize {
            if matches!(msg, WM_PING | WM_REENTRANT | WM_REMOVE_SELF | WM_REMOVE_NEIGHBOR | crate::user32::wm::NCDESTROY) {
                TRACE.lock().unwrap().push(format!("C:msg={msg:#x}:id={uid}:ref={ref_data:#x}"));
            }
            if msg == WM_REENTRANT {
                // Nested SendMessage while inside SUBCLASSPROC C.
                let nested = crate::user32::message::SendMessageW(hwnd, WM_PING, 7, 9);
                assert_eq!(nested, 1111);
            }
            if msg == WM_REMOVE_NEIGHBOR {
                // Remove B while C is dispatching; DefSubclassProc must skip B and hit A.
                assert_eq!(RemoveWindowSubclass(hwnd, sub_b as *const () as usize, 20), TRUE);
            }
            let r = DefSubclassProc(hwnd, msg, wp, lp);
            if msg == WM_PING { r + 1000 } else { r }
        }

        std::thread::spawn(|| unsafe {
            TRACE.lock().unwrap().clear();

            // Invalid / null HWND must fail with ERROR_INVALID_WINDOW_HANDLE.
            assert_eq!(SetWindowSubclass(std::ptr::null_mut(), sub_a as *const () as usize, 1, 0), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);
            assert_eq!(RemoveWindowSubclass(0xDEAD_BEEFusize as HWND, sub_a as *const () as usize, 1), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);
            let mut out_ref = 0x55usize;
            assert_eq!(GetWindowSubclass(std::ptr::null_mut(), sub_a as *const () as usize, 1, &mut out_ref), FALSE);
            assert_eq!(out_ref, 0);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_WINDOW_HANDLE);

            // Create a headless window.
            let cls = crate::strings::to_wide_z("ComctlSubclassTestWnd");
            let wc = crate::user32::class::WNDCLASS {
                style: 0,
                wndproc: base_wndproc as *const () as usize,
                cls_extra: 0,
                wnd_extra: 0,
                instance: 0,
                icon: 0,
                cursor: 0,
                background: 0,
                menu_name: 0,
                class_name: cls.as_ptr() as usize,
            };
            let _ = crate::user32::class::RegisterClassW(&wc);
            let hwnd = crate::user32::window::CreateWindowExW(
                0,
                cls.as_ptr(),
                std::ptr::null(),
                crate::user32::ws::POPUP,
                0,
                0,
                100,
                100,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null());

            // Stack A -> B -> C (LIFO: C runs first, then B, then A, then base_wndproc).
            assert_eq!(SetWindowSubclass(hwnd, sub_a as *const () as usize, 10, 0xAAA1), TRUE);
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB1), TRUE);
            assert_eq!(SetWindowSubclass(hwnd, sub_c as *const () as usize, 30, 0xCCC1), TRUE);

            // Update B's dwRefData in place without changing LIFO order.
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB2), TRUE);

            // GetWindowSubclass (including unaligned output pointer and null pointer check).
            let mut unaligned_buf = [0u8; 16];
            let unaligned_ptr = unaligned_buf.as_mut_ptr().add(3) as *mut usize;
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, unaligned_ptr), TRUE);
            assert_eq!(unaligned_ptr.read_unaligned(), 0xBBB2);
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), TRUE);
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 999, unaligned_ptr), FALSE);
            assert_eq!(unaligned_ptr.read_unaligned(), 0);

            // 1. LIFO chaining + 6-arg delivery + base_wndproc fallback.
            TRACE.lock().unwrap().clear();
            let res = crate::user32::message::SendMessageW(hwnd, WM_PING, 1, 2);
            assert_eq!(res, 1111); // 100 + 1 + 10 + 1000
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x401:id=30:ref=0xccc1",
                    "B:msg=0x401:id=20:ref=0xbbb2",
                    "A:msg=0x401:id=10:ref=0xaaa1",
                    "base:0x401:0x1:0x2",
                ]
            );

            // 2. Re-entrant SendMessage inside C.
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REENTRANT, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x402:id=30:ref=0xccc1",
                    "C:msg=0x401:id=30:ref=0xccc1",
                    "B:msg=0x401:id=20:ref=0xbbb2",
                    "A:msg=0x401:id=10:ref=0xaaa1",
                    "base:0x401:0x7:0x9",
                    "B:msg=0x402:id=20:ref=0xbbb2",
                    "A:msg=0x402:id=10:ref=0xaaa1",
                    "base:0x402:0x0:0x0",
                ]
            );

            // 3. Neighbor removal during dispatch (C removes B, then calls DefSubclassProc -> A).
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REMOVE_NEIGHBOR, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "C:msg=0x404:id=30:ref=0xccc1",
                    "A:msg=0x404:id=10:ref=0xaaa1",
                    "base:0x404:0x0:0x0",
                ]
            );
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), FALSE);

            // 4. Re-add B and test self-removal during dispatch (B removes itself, then calls DefSubclassProc -> A).
            assert_eq!(SetWindowSubclass(hwnd, sub_b as *const () as usize, 20, 0xBBB3), TRUE);
            TRACE.lock().unwrap().clear();
            crate::user32::message::SendMessageW(hwnd, WM_REMOVE_SELF, 0, 0);
            assert_eq!(
                TRACE.lock().unwrap().clone(),
                vec![
                    "B:msg=0x403:id=20:ref=0xbbb3",
                    "C:msg=0x403:id=30:ref=0xccc1",
                    "A:msg=0x403:id=10:ref=0xaaa1",
                    "base:0x403:0x0:0x0",
                ]
            );
            assert_eq!(GetWindowSubclass(hwnd, sub_b as *const () as usize, 20, std::ptr::null_mut()), FALSE);

            // 5. WM_NCDESTROY is delivered through remaining chain (C -> A -> base) and cleans up table.
            TRACE.lock().unwrap().clear();
            assert_eq!(crate::user32::window::DestroyWindow(hwnd), TRUE);
            let tr = TRACE.lock().unwrap().clone();
            assert!(tr.iter().any(|s| s == "C:msg=0x82:id=30:ref=0xccc1"));
            assert!(tr.iter().any(|s| s == "A:msg=0x82:id=10:ref=0xaaa1"));
            assert!(tr.iter().any(|s| s == "base:0x82:0x0:0x0"));
            assert!(!subclass_table().lock().unwrap().contains_key(&(hwnd as usize)));
        })
        .join()
        .unwrap();
    }
}
