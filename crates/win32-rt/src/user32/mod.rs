//! user32.dll — windows, message queues, input, cursor, monitors.
//!
//! # Model
//!
//! The Win32 window system is modelled entirely in Rust; AppKit is only a
//! backend (see [`crate::macos`]):
//!
//! * **Windows** ([`window`]): an `HWND` table. Each window belongs to the
//!   thread that created it; its window procedure only ever runs on that
//!   thread (as on Windows). Top-level windows get an `NSWindow` whose
//!   content view is backed by a `CAMetalLayer` ([`crate::macos::metal_layer_for_hwnd`]).
//! * **Message queues** ([`message`]): one per Win32 thread, created on first
//!   use. State lives under the global kernel wait lock
//!   ([`crate::handles::wait_lock`]) so `MsgWaitForMultipleObjectsEx` is just
//!   another waitable. Retrieval order follows Windows: sent messages (always
//!   dispatched inside Get/Peek), posted, input, `WM_QUIT`, `WM_PAINT`,
//!   `WM_TIMER`.
//! * **Native events** arrive on the process main thread (AppKit). They never
//!   call guest code there: input becomes queued input messages for the
//!   owning thread, and window-state changes (resize, activation, close
//!   button, ...) become *internal sent items* — closures executed on the
//!   owner thread the next time it looks at its queue, which then sends the
//!   usual `WM_*` sequence through the normal Win32 code path.
//!
//! # Coordinates (Retina policy)
//!
//! Win32 coordinates are **backing pixels** of the main display: one Win32
//! pixel = one physical pixel at scale `S` = the main display's backing
//! scale factor (2.0 on Retina). Screen origin is the top-left of the main
//! display (CoreGraphics global space × S, y down). DPI is reported as 96
//! everywhere: a Retina panel looks like a 100%-scaled monitor of its pixel
//! size, so every number a per-monitor-aware app sees is consistent and it
//! renders at native resolution (AW2 has its own render scale/DLSS/FSR).
//! `AW2_DPI=<n>` overrides the reported DPI. `AW2_RETINA=0` forces S = 1
//! (points; lower GPU cost, the compositor upscales). Secondary displays
//! with a different scale use the main display's S (documented limitation).

#![allow(clippy::upper_case_acronyms)]

pub mod class;
pub mod cursor;
pub mod defproc;
pub mod dialog;
pub mod input;
pub mod keymap;
pub mod message;
pub mod misc;
pub mod monitor;
pub mod window;
pub mod wsprintf;

/// Tests that touch process-wide window-system state (activation, focus,
/// raw-input registrations, the cursor) hold this lock.
#[cfg(test)]
pub(crate) static TEST_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub static EXPORTS: &[crate::Export] = include!(concat!(env!("OUT_DIR"), "/user32_exports.rs"));

pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type ATOM = u16;
pub type WNDPROC = unsafe extern "win64" fn(usize, u32, WPARAM, LPARAM) -> LRESULT;
/// Strongly-typed internal host window procedure signature.
pub type TypedWndProc = unsafe extern "win64" fn(crate::types::HWND, u32, WPARAM, LPARAM) -> LRESULT;

/// Opaque Windows-style high-word (`0xFFFFxxxx`) proc handles for built-in internal host
/// window procedures. Never expose raw host code addresses via `Get/SetWindowLongPtr(GWLP_WNDPROC)`
/// or `GetClassInfoEx`.
pub const WNDPROC_HANDLE_DEF_WINDOW_W: usize = 0xFFFF_0001;
pub const WNDPROC_HANDLE_DEF_WINDOW_A: usize = 0xFFFF_0002;
pub const WNDPROC_HANDLE_DEF_DLG_W: usize = 0xFFFF_0003;
pub const WNDPROC_HANDLE_DEF_DLG_A: usize = 0xFFFF_0004;
pub const WNDPROC_HANDLE_BUTTON: usize = 0xFFFF_0005;
pub const WNDPROC_HANDLE_EDIT: usize = 0xFFFF_0006;
pub const WNDPROC_HANDLE_DEFAULT_CTRL: usize = 0xFFFF_0007;
const WNDPROC_HANDLE_DYNAMIC_BASE: usize = 0xFFFF_0100;

static TYPED_HOST_WNDPROCS: std::sync::Mutex<Vec<TypedWndProc>> = std::sync::Mutex::new(Vec::new());

/// Explicitly register an internal Rust `TypedWndProc` under an opaque `0xFFFFxxxx` proc handle
/// so it can be invoked via `CallWindowProcW/A` and `window::call_window_proc` without exposing
/// raw host pointers or widening general native dispatch.
pub fn register_typed_host_wndproc(proc_: TypedWndProc) -> usize {
    let mut reg = TYPED_HOST_WNDPROCS.lock().unwrap_or_else(|e| e.into_inner());
    let target = proc_ as *const () as usize;
    if let Some(idx) = reg.iter().position(|&f| (f as *const () as usize) == target) {
        return WNDPROC_HANDLE_DYNAMIC_BASE + idx;
    }
    let idx = reg.len();
    reg.push(proc_);
    WNDPROC_HANDLE_DYNAMIC_BASE + idx
}

/// Resolve the synthetic `user32.dll` export thunk VAs for the 4 built-in Windows default
/// window procedures (`DefWindowProcW/A`, `DefDlgProcW/A`), cached once `user32.dll` is loaded.
fn user32_defproc_thunks() -> Option<[(usize, TypedWndProc); 4]> {
    static CACHE: std::sync::OnceLock<[(usize, TypedWndProc); 4]> = std::sync::OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Some(*cached);
    }
    let l = crate::try_loader()?;
    let hmod = l.get_module_handle(Some("user32.dll"))?;
    let def_w = l.get_proc_address(hmod, crate::ProcName::Name("DefWindowProcW")).unwrap_or(0);
    let def_a = l.get_proc_address(hmod, crate::ProcName::Name("DefWindowProcA")).unwrap_or(0);
    let dlg_w = l.get_proc_address(hmod, crate::ProcName::Name("DefDlgProcW")).unwrap_or(0);
    let dlg_a = l.get_proc_address(hmod, crate::ProcName::Name("DefDlgProcA")).unwrap_or(0);
    let table: [(usize, TypedWndProc); 4] = [
        (def_w, defproc::DefWindowProcW),
        (def_a, defproc::DefWindowProcA),
        (dlg_w, dialog::DefDlgProcW),
        (dlg_a, dialog::DefDlgProcA),
    ];
    let _ = CACHE.set(table);
    Some(table)
}

/// Resolve `p` strictly as an explicitly authorized typed internal `WNDPROC`:
/// - opaque built-in `0xFFFF_0001..=0xFFFF_0007` handles,
/// - Rust-registered typed handles (`0xFFFF_0100..`), or
/// - `user32.dll`'s 4 built-in `DefWindowProcW/A` / `DefDlgProcW/A` export thunk VAs.
///
/// Arbitrary raw host addresses and unrelated `VA_DISPATCH` entries NEVER match.
pub(crate) fn resolve_typed_wndproc(p: usize) -> Option<TypedWndProc> {
    match p {
        WNDPROC_HANDLE_DEF_WINDOW_W => return Some(defproc::DefWindowProcW),
        WNDPROC_HANDLE_DEF_WINDOW_A => return Some(defproc::DefWindowProcA),
        WNDPROC_HANDLE_DEF_DLG_W => return Some(dialog::DefDlgProcW),
        WNDPROC_HANDLE_DEF_DLG_A => return Some(dialog::DefDlgProcA),
        WNDPROC_HANDLE_BUTTON => return Some(dialog::button_wndproc),
        WNDPROC_HANDLE_EDIT => return Some(dialog::edit_wndproc),
        WNDPROC_HANDLE_DEFAULT_CTRL => return Some(dialog::default_control_wndproc),
        _ => {}
    }
    if p >= WNDPROC_HANDLE_DYNAMIC_BASE && (p >> 16) == 0xFFFF {
        let idx = p - WNDPROC_HANDLE_DYNAMIC_BASE;
        let reg = TYPED_HOST_WNDPROCS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(&f) = reg.get(idx) {
            return Some(f);
        }
        return None;
    }
    if let Some(thunks) = user32_defproc_thunks() {
        for (va, f) in thunks {
            if va != 0 && va == p {
                return Some(f);
            }
        }
    }
    None
}

/// True strictly if `p` points to validated executable guest code (an executable section of
/// a real mapped guest PE image, or committed executable guest `VirtualAlloc`/`MapViewOfFile`
/// memory — excluding synthetic builtins, host `__TEXT`, and non-executable pages).
#[inline]
pub(crate) fn is_validated_guest_wndproc_target(p: usize) -> bool {
    if p == 0 {
        return false;
    }
    crate::try_loader().is_some_and(|l| l.is_guest_pe_code(p))
        || crate::kernel32::memory::is_guest_exec_vm_addr(p)
}

/// Dispatch a `WNDPROC` target for `CallWindowProcW/A` and `window::call_window_proc`.
///
/// Enforces strict separation between typed internal host `WNDPROC`s and validated guest code:
/// 1. Explicitly authorized typed internal `WNDPROC` (opaque `0xFFFFxxxx` handle or `user32.dll`
///    `DefWindowProcW/A` / `DefDlgProcW/A` thunk) -> invoked directly with typed `WNDPROC` ABI.
/// 2. Validated executable guest code target when a guest translator dispatcher is active ->
///    invoked via `crate::call_guest8`.
/// 3. Anything else (arbitrary host `__TEXT`, raw host fn pointers, non-`WNDPROC` Win32/COM
///    thunks, non-executable guest memory, or guest code without a guest dispatcher) -> rejected
///    BEFORE invocation with `ERROR_INVALID_PARAMETER`.
pub(crate) unsafe fn dispatch_wndproc(
    p: usize,
    hwnd: crate::types::HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if p == 0 {
        return 0;
    }
    if let Some(f) = resolve_typed_wndproc(p) {
        return f(hwnd, msg, wparam, lparam);
    }
    if crate::has_guest_dispatcher() && is_validated_guest_wndproc_target(p) {
        return crate::call_guest8(
            p,
            hwnd as u64,
            msg as u64,
            wparam as u64,
            lparam as u64,
            0,
            0,
            0,
            0,
        ) as LRESULT;
    }
    log::warn!("user32: CallWindowProc rejected unauthorized target {p:#x}");
    set_error(e::INVALID_PARAMETER);
    0
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl RECT {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> RECT {
        RECT { left, top, right, bottom }
    }
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
    pub fn offset(&self, dx: i32, dy: i32) -> RECT {
        RECT::new(self.left + dx, self.top + dy, self.right + dx, self.bottom + dy)
    }
    pub fn intersect(&self, o: &RECT) -> Option<RECT> {
        let r = RECT::new(self.left.max(o.left), self.top.max(o.top), self.right.min(o.right), self.bottom.min(o.bottom));
        (r.right > r.left && r.bottom > r.top).then_some(r)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

/// `MSG` (48 bytes on x64).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MSG {
    pub hwnd: usize,
    pub message: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
    pub time: u32,
    pub pt: POINT,
    pub l_private: u32,
}
const _: () = assert!(std::mem::size_of::<MSG>() == 48);

pub fn make_long(lo: i32, hi: i32) -> isize {
    ((lo as u32 & 0xFFFF) | ((hi as u32 & 0xFFFF) << 16)) as i32 as isize
}

/// Window messages used by the implementation.
pub mod wm {
    pub const NULL: u32 = 0x0000;
    pub const CREATE: u32 = 0x0001;
    pub const DESTROY: u32 = 0x0002;
    pub const MOVE: u32 = 0x0003;
    pub const SIZE: u32 = 0x0005;
    pub const ACTIVATE: u32 = 0x0006;
    pub const SETFOCUS: u32 = 0x0007;
    pub const KILLFOCUS: u32 = 0x0008;
    pub const ENABLE: u32 = 0x000A;
    pub const SETTEXT: u32 = 0x000C;
    pub const GETTEXT: u32 = 0x000D;
    pub const GETTEXTLENGTH: u32 = 0x000E;
    pub const PAINT: u32 = 0x000F;
    pub const CLOSE: u32 = 0x0010;
    pub const QUIT: u32 = 0x0012;
    pub const ERASEBKGND: u32 = 0x0014;
    pub const SHOWWINDOW: u32 = 0x0018;
    pub const ACTIVATEAPP: u32 = 0x001C;
    pub const SETCURSOR: u32 = 0x0020;
    pub const MOUSEACTIVATE: u32 = 0x0021;
    pub const GETMINMAXINFO: u32 = 0x0024;
    pub const NEXTDLGCTL: u32 = 0x0028;
    pub const VKEYTOITEM: u32 = 0x002E;
    pub const CHARTOITEM: u32 = 0x002F;
    pub const QUERYDRAGICON: u32 = 0x0037;
    pub const COMPAREITEM: u32 = 0x0039;
    pub const WINDOWPOSCHANGING: u32 = 0x0046;
    pub const WINDOWPOSCHANGED: u32 = 0x0047;
    pub const DISPLAYCHANGE: u32 = 0x007E;
    pub const GETICON: u32 = 0x007F;
    pub const SETICON: u32 = 0x0080;
    pub const NCCREATE: u32 = 0x0081;
    pub const NCDESTROY: u32 = 0x0082;
    pub const NCCALCSIZE: u32 = 0x0083;
    pub const NCHITTEST: u32 = 0x0084;
    pub const NCPAINT: u32 = 0x0085;
    pub const NCACTIVATE: u32 = 0x0086;
    pub const GETDLGCODE: u32 = 0x0087;
    pub const BM_GETCHECK: u32 = 0x00F0;
    pub const BM_SETCHECK: u32 = 0x00F1;
    pub const BM_CLICK: u32 = 0x00F5;
    pub const INPUT: u32 = 0x00FF;
    pub const KEYDOWN: u32 = 0x0100;
    pub const KEYUP: u32 = 0x0101;
    pub const CHAR: u32 = 0x0102;
    pub const DEADCHAR: u32 = 0x0103;
    pub const SYSKEYDOWN: u32 = 0x0104;
    pub const SYSKEYUP: u32 = 0x0105;
    pub const SYSCHAR: u32 = 0x0106;
    pub const INITDIALOG: u32 = 0x0110;
    pub const COMMAND: u32 = 0x0111;
    pub const SYSCOMMAND: u32 = 0x0112;
    pub const TIMER: u32 = 0x0113;
    pub const CTLCOLORMSGBOX: u32 = 0x0132;
    pub const CTLCOLORSTATIC: u32 = 0x0138;
    pub const MOUSEMOVE: u32 = 0x0200;
    pub const LBUTTONDOWN: u32 = 0x0201;
    pub const LBUTTONUP: u32 = 0x0202;
    pub const LBUTTONDBLCLK: u32 = 0x0203;
    pub const RBUTTONDOWN: u32 = 0x0204;
    pub const RBUTTONUP: u32 = 0x0205;
    pub const RBUTTONDBLCLK: u32 = 0x0206;
    pub const MBUTTONDOWN: u32 = 0x0207;
    pub const MBUTTONUP: u32 = 0x0208;
    pub const MBUTTONDBLCLK: u32 = 0x0209;
    pub const MOUSEWHEEL: u32 = 0x020A;
    pub const XBUTTONDOWN: u32 = 0x020B;
    pub const XBUTTONUP: u32 = 0x020C;
    pub const XBUTTONDBLCLK: u32 = 0x020D;
    pub const MOUSEHWHEEL: u32 = 0x020E;
    pub const MOUSEFIRST: u32 = 0x0200;
    pub const MOUSELAST: u32 = 0x020E;
    pub const KEYFIRST: u32 = 0x0100;
    pub const KEYLAST: u32 = 0x0109;
    pub const CAPTURECHANGED: u32 = 0x0215;
    pub const DEVICECHANGE: u32 = 0x0219;
    pub const ENTERSIZEMOVE: u32 = 0x0231;
    pub const EXITSIZEMOVE: u32 = 0x0232;
    pub const DPICHANGED: u32 = 0x02E0;
    pub const USER: u32 = 0x0400;
    pub const APP: u32 = 0x8000;
}

/// Window styles.
pub mod ws {
    pub const POPUP: u32 = 0x8000_0000;
    pub const CHILD: u32 = 0x4000_0000;
    pub const MINIMIZE: u32 = 0x2000_0000;
    pub const VISIBLE: u32 = 0x1000_0000;
    pub const DISABLED: u32 = 0x0800_0000;
    pub const MAXIMIZE: u32 = 0x0100_0000;
    pub const CAPTION: u32 = 0x00C0_0000;
    pub const BORDER: u32 = 0x0080_0000;
    pub const SYSMENU: u32 = 0x0008_0000;
    pub const THICKFRAME: u32 = 0x0004_0000;
    pub const MINIMIZEBOX: u32 = 0x0002_0000;
    pub const MAXIMIZEBOX: u32 = 0x0001_0000;
    pub const GROUP: u32 = 0x0002_0000;
    pub const TABSTOP: u32 = 0x0001_0000;
    pub const EX_TOPMOST: u32 = 0x0000_0008;
    pub const EX_TOOLWINDOW: u32 = 0x0000_0080;
    pub const EX_NOACTIVATE: u32 = 0x0800_0000;
}

/// Errors used by user32.
pub mod e {
    pub const INVALID_DATA: u32 = 13;
    pub const INVALID_WINDOW_HANDLE: u32 = 1400;
    pub const CANNOT_FIND_WND_CLASS: u32 = 1407;
    pub const CLASS_ALREADY_EXISTS: u32 = 1410;
    pub const CLASS_DOES_NOT_EXIST: u32 = 1411;
    pub const CLASS_HAS_WINDOWS: u32 = 1412;
    pub const CONTROL_ID_NOT_FOUND: u32 = 1421;
    pub const INVALID_THREAD_ID: u32 = 1444;
    pub const WINDOW_OF_OTHER_THREAD: u32 = 1408;
    pub const INVALID_PARAMETER: u32 = 87;
    pub const INSUFFICIENT_BUFFER: u32 = 122;
    pub const INVALID_INDEX: u32 = 1413;
    pub const INVALID_MONITOR_HANDLE: u32 = 1461;
    pub const NOT_SUPPORTED: u32 = 50;
    pub const CALL_NOT_IMPLEMENTED: u32 = 120;
    pub const RESOURCE_TYPE_NOT_FOUND: u32 = 1813;
    pub const RESOURCE_NAME_NOT_FOUND: u32 = 1814;
    pub const TIMEOUT: u32 = 1460;
    pub const INVALID_CURSOR_HANDLE: u32 = 1402;
}

pub(crate) fn set_error(code: u32) {
    // The AppKit main thread has no TEB (touching it would adopt the thread
    // and write the gs slots).
    if crate::macos::is_main_thread() {
        return;
    }
    crate::teb::set_last_error(code);
}

/// Milliseconds since boot, as `GetTickCount` / `MSG::time`. Safe to call on
/// the AppKit main thread (no TEB access).
pub fn tick_ms() -> u32 {
    (crate::kernel32::time::interrupt_time_100ns() / 10_000) as u32
}

/// `IS_INTRESOURCE`.
pub fn is_int_resource(p: *const u16) -> bool {
    (p as usize) >> 16 == 0
}
