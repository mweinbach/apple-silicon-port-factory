//! `mfc140u.dll` — Microsoft Foundation Classes 14.x (Unicode, x64) runtime.
//!
//! MFC 14.x (`mfc140u.dll`, binary-compatible across Visual C++ 2015–2022 / toolset 14.0–14.44)
//! exports its symbols by ordinal only (`NONAME`). Ordinals and struct layouts below were verified
//! against retail Microsoft `mfc140u.dll` v14.44.35211.0 and its official Microsoft PDB
//! (`mfc140u.amd64.pdb`, GUID `0B34A86EA5B44CB674FEE9DF38D833931`).
//!
//! Implements real per-thread and per-module MFC state (`_AFX_THREAD_STATE`, `AFX_MODULE_STATE`,
//! `AFX_MAINTAIN_STATE2`), `CWinApp` lifecycle, `AFX_EXTENSION_MODULE` / `CDynLinkLibrary`
//! registration, temp-map reference counting, `AfxWndProc`, and MFC heap/exception helpers.

use crate::kernel32::module::GetModuleFileNameW;
use crate::kernel32::threads::{GetCurrentThread, GetCurrentThreadId};
use crate::types::{BOOL, DWORD, HINSTANCE, HMODULE, HWND, UINT};
use crate::user32::defproc::DefWindowProcW;
use crate::user32::{LPARAM, LRESULT, WPARAM};
use crate::{Export, Ptr};
use std::cell::RefCell;
use std::collections::HashSet;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{LazyLock, Mutex, Once};

const WM_QUERYAFXWNDPROC: UINT = 0x0360;
const AFX_DEFAULT_AUTOSAVE_INTERVAL_MS: i32 = 300_000; // 0x493e0

// ---------------------------------------------------------------------------
// 1. MSVC x64 MFC 14.x Struct Layouts (verified against mfc140u.dll 14.44)
// ---------------------------------------------------------------------------

/// `AFX_MODULE_STATE` (`0x108` bytes on x64).
///
/// Key offsets read/written by guest MFC glue (`dllmodul.cpp` / `mfcs140u.lib`) and `mfc140u.dll`:
/// - `+0x00`: vtable (`CNoTrackObject`)
/// - `+0x08`: `m_pCurrentWinApp` (`CWinApp*`, read by `AfxGetApp()`)
/// - `+0x10`: `m_hCurrentInstanceHandle` (`HINSTANCE`, read by `AfxGetInstanceHandle()`)
/// - `+0x18`: `m_hCurrentResourceHandle` (`HINSTANCE`, read by `AfxGetResourceHandle()`)
/// - `+0x20`: `m_lpszCurrentAppName` (`LPCWSTR`)
/// - `+0x28`: `m_bDLL` (`u8`)
/// - `+0x29`: `m_bSystem` (`u8`)
/// - `+0x2c`: `m_fRegisteredClasses` (`u32`)
/// - `+0x30`: `m_pClassInit` (`CRuntimeClass*`)
/// - `+0x38`: `m_classList.m_pHead` (`*mut c_void`)
/// - `+0x40`: `m_classList.m_nNextOffset` (`0x20`)
/// - `+0x48`: `m_pFactoryInit` (`COleObjectFactory*`)
/// - `+0x50`: `m_factoryList.m_pHead` (`*mut c_void`)
/// - `+0x58`: `m_factoryList.m_nNextOffset` (`0x40`)
/// - `+0x64`: `m_bUserCtrl` (`BOOL`, initialized to `1`)
/// - `+0x68`: `m_strUnregisterList` (`CStringW` data pointer)
/// - `+0x70`: `m_pfnAfxWndProc` (`WNDPROC`)
/// - `+0x78`: `m_dwVersion` (`DWORD`)
/// - `+0x88`: `m_libraryList.m_pHead` (`*mut CDynLinkLibrary`)
/// - `+0x90`: `m_libraryList.m_nNextOffset` (`0x78`)
/// - `+0x98`: `m_hActCtx` / module context handle
/// - `+0xa8`: `m_frameList.m_pHead` (`*mut c_void`)
/// - `+0xb0`: `m_frameList.m_nNextOffset` (`0x20`)
/// - `+0xc0`: `m_nSlot` (`-1`)
/// - `+0xf0`: `m_pTypeLibCacheMap` (`*mut c_void`)
/// - `+0xf8`: `m_thread` (`CThreadLocal<AFX_MODULE_THREAD_STATE>`)
#[repr(C)]
pub struct AfxModuleState {
    pub vtable: *const Ptr,                      // +0x00
    pub current_win_app: *mut CWinApp,           // +0x08
    pub current_instance_handle: HINSTANCE,      // +0x10
    pub current_resource_handle: HINSTANCE,      // +0x18
    pub current_app_name: *const u16,            // +0x20
    pub is_dll: u8,                              // +0x28
    pub is_system: u8,                           // +0x29
    pub reserved_flags: [u8; 2],                 // +0x2a
    pub registered_classes: DWORD,               // +0x2c
    pub class_init: *mut c_void,                 // +0x30
    pub class_list_head: *mut c_void,            // +0x38
    pub class_list_next_offset: usize,           // +0x40
    pub factory_init: *mut c_void,               // +0x48
    pub factory_list_head: *mut c_void,          // +0x50
    pub factory_list_next_offset: usize,         // +0x58
    pub object_count: i32,                       // +0x60
    pub user_ctrl: BOOL,                         // +0x64
    pub unregister_list_str: *mut u16,           // +0x68
    pub afx_wnd_proc: *const c_void,             // +0x70
    pub version: DWORD,                          // +0x78
    pub _pad7c: u32,                             // +0x7c
    pub filter_tooltip_fn: *const c_void,        // +0x80
    pub library_list_head: *mut CDynLinkLibrary, // +0x88
    pub library_list_next_offset: usize,         // +0x90
    pub act_ctx: *mut c_void,                    // +0x98
    pub _reserved_a0: usize,                     // +0xa0
    pub frame_list_head: *mut c_void,            // +0xa8
    pub frame_list_next_offset: usize,           // +0xb0
    pub _reserved_b8: usize,                     // +0xb8
    pub slot_index: i32,                         // +0xc0
    pub _padc4: u32,                             // +0xc4
    pub typelib_cache: *mut c_void,              // +0xc8
    pub _reserved_d0: [usize; 2],                // +0xd0
    pub typelib_cache_map: *mut c_void,          // +0xe0
    pub flags_e8: u32,                           // +0xe8
    pub _padec: u32,                             // +0xec
    pub occ_manager: *mut c_void,                // +0xf0
    pub thread_local_slot: i32,                  // +0xf8
    pub thread_local_pad: u32,                   // +0xfc
    pub thread_local_extra: usize,               // +0x100
}
const _: () = assert!(std::mem::size_of::<AfxModuleState>() == 0x108);

/// `_AFX_THREAD_STATE` — per-thread MFC state.
///
/// Offset `+0x08` holds `m_pModuleState`, matching `mfc140u.dll` (`AfxSetModuleState`,
/// `AfxGetModuleState`, `AFX_MAINTAIN_STATE2::ctor/dtor`).
#[repr(C)]
pub struct AfxThreadState {
    pub vtable: *const Ptr,                     // +0x00
    pub module_state: *mut AfxModuleState,      // +0x08
    pub prev_module_state: *mut AfxModuleState, // +0x10
    pub current_win_thread: *mut CWinApp,       // +0x18
    pub temp_map_lock: i32,                     // +0x20
    pub _pad24: u32,                            // +0x24
}

/// `AFX_MAINTAIN_STATE2` (`0x10` bytes on x64).
///
/// RAII helper saving and restoring `_AFX_THREAD_STATE::m_pModuleState`.
#[repr(C)]
pub struct AfxMaintainState2 {
    pub prev_module_state: *mut AfxModuleState, // +0x00
    pub thread_state: *mut AfxThreadState,      // +0x08
}
const _: () = assert!(std::mem::size_of::<AfxMaintainState2>() == 0x10);

/// `AFX_EXTENSION_MODULE` (`0x28` bytes on x64).
#[repr(C)]
pub struct AfxExtensionModule {
    pub initialized: BOOL,                 // +0x00
    pub _pad04: u32,                       // +0x04
    pub module: HMODULE,                   // +0x08
    pub resource: HMODULE,                 // +0x10
    pub first_shared_class: *mut c_void,   // +0x18
    pub first_shared_factory: *mut c_void, // +0x20
}
const _: () = assert!(std::mem::size_of::<AfxExtensionModule>() == 0x28);

/// `CDynLinkLibrary` (`0x80` bytes on x64).
#[repr(C)]
pub struct CDynLinkLibrary {
    pub vtable: *const Ptr,                // +0x00
    pub dw_ref: u32,                       // +0x08
    pub _pad0c: u32,                       // +0x0c
    pub _cmd_target_fields: [usize; 5],    // +0x10..+0x38
    pub module_state: *mut AfxModuleState, // +0x38
    pub module: HMODULE,                   // +0x40
    pub resource: HMODULE,                 // +0x48
    pub class_list_head: *mut c_void,      // +0x50
    pub class_list_next_offset: usize,     // +0x58
    pub factory_list_head: *mut c_void,    // +0x60
    pub factory_list_next_offset: usize,   // +0x68
    pub is_system: BOOL,                   // +0x70
    pub _pad74: u32,                       // +0x74
    pub next_dll: *mut CDynLinkLibrary,    // +0x78
}
const _: () = assert!(std::mem::size_of::<CDynLinkLibrary>() == 0x80);

/// `CWinApp` (`0x178` bytes on x64).
#[repr(C)]
pub struct CWinApp {
    pub vtable: *const Ptr,                  // +0x00
    pub dw_ref: u32,                         // +0x08
    pub _pad0c: u32,                         // +0x0c
    pub _cmd_target_fields: [usize; 5],      // +0x10..+0x38
    pub module_state: *mut AfxModuleState,   // +0x38
    pub main_wnd: *mut c_void,               // +0x40
    pub active_wnd: *mut c_void,             // +0x48
    pub auto_delete: BOOL,                   // +0x50
    pub _pad54: u32,                         // +0x54
    pub thread_handle: *mut c_void,          // +0x58
    pub thread_id: DWORD,                    // +0x60
    pub _pad64: u32,                         // +0x64
    pub _thread_fields: [usize; 4],          // +0x68..+0x88
    pub instance: HINSTANCE,                 // +0x88
    pub cmd_line: *mut u16,                  // +0x90
    pub cmd_show: i32,                       // +0x98
    pub _pad9c: u32,                         // +0x9c
    pub app_name: *mut u16,                  // +0xa0
    pub registry_key: *mut u16,              // +0xa8
    pub doc_manager: *mut c_void,            // +0xb0
    pub profile_name: *mut u16,              // +0xb8
    pub help_mode: i32,                      // +0xc0
    pub _padc4: u32,                         // +0xc4
    pub exe_name: *mut u16,                  // +0xc8
    pub help_file_path: *mut u16,            // +0xd0
    pub recent_file_list: *mut c_void,       // +0xd8
    pub _pad_e0: [usize; 4],                 // +0xe0..+0x100
    pub lang_resource_dll: HMODULE,          // +0x100
    pub _pad_108: [usize; 6],                // +0x108..+0x138
    pub num_preview_pages: usize,            // +0x138
    pub _pad_140: usize,                     // +0x140
    pub _pad_148: u32,                       // +0x148
    pub restart_manager_flags: DWORD,        // +0x14c
    pub autosave_interval: i32,              // +0x150
    pub taskbar_interaction_enabled: BOOL,   // +0x154
    pub is_win7_or_greater: BOOL,            // +0x158
    pub _pad_15c: u32,                       // +0x15c
    pub register_with_restart_manager: BOOL, // +0x160
    pub _pad_164: u32,                       // +0x164
    pub _tail: [usize; 2],                   // +0x168..+0x178
}
const _: () = assert!(std::mem::size_of::<CWinApp>() == 0x178);

/// Minimal `ATL::CStringData` header (`0x18` bytes) preceding a `CStringW` character buffer.
#[repr(C)]
struct AtlStringData {
    string_mgr: *const Ptr, // +0x00
    data_length: i32,       // +0x08
    alloc_length: i32,      // +0x0c
    refs: AtomicI32,        // +0x10
    _pad14: u32,            // +0x14
    nul_char: u16,          // +0x18
    _pad1a: [u16; 3],       // +0x1a
}

/// `AFX_MSGMAP_ENTRY` (`0x30` bytes on x64).
#[repr(C)]
struct AfxMsgMapEntry {
    message: UINT,
    code: UINT,
    id: UINT,
    last_id: UINT,
    sig: usize,
    pfn: *const (),
}

/// `AFX_MSGMAP` (`0x10` bytes on x64).
#[repr(C)]
pub struct AfxMsgMap {
    pfn_get_base_map: *const (),
    entries: *const AfxMsgMapEntry,
}
unsafe impl Sync for AfxMsgMap {}

// ---------------------------------------------------------------------------
// 2. Static Vtables, Process-Wide Default Module State, and Thread-Local State
// ---------------------------------------------------------------------------

unsafe extern "win64" fn noop_vfunc(_this: *mut c_void) -> usize {
    0
}

unsafe extern "win64" fn atl_string_mgr_free(_mgr: *mut c_void, data: *mut c_void) {
    if !data.is_null() {
        let _ = data;
    }
}

static ATL_STRING_MGR_VT: [Ptr; 4] = [
    Ptr(noop_vfunc as *const ()),
    Ptr(atl_string_mgr_free as *const ()),
    Ptr(noop_vfunc as *const ()),
    Ptr(noop_vfunc as *const ()),
];

#[repr(C)]
struct StaticStringMgr {
    #[allow(dead_code)]
    vtable: *const Ptr,
}
unsafe impl Sync for StaticStringMgr {}

static ATL_STRING_MGR: StaticStringMgr = StaticStringMgr {
    vtable: ATL_STRING_MGR_VT.as_ptr(),
};

/// Shared immutable empty `CStringData` (`refs = -1`, locked/nil sentinel in ATL).
struct StaticEmptyString {
    hdr: AtlStringData,
}
unsafe impl Sync for StaticEmptyString {}

static EMPTY_CSTRING_W: StaticEmptyString = StaticEmptyString {
    hdr: AtlStringData {
        string_mgr: &ATL_STRING_MGR as *const StaticStringMgr as *const Ptr,
        data_length: 0,
        alloc_length: 0,
        refs: AtomicI32::new(-1),
        _pad14: 0,
        nul_char: 0,
        _pad1a: [0; 3],
    },
};

fn empty_cstring_w_ptr() -> *mut u16 {
    &EMPTY_CSTRING_W.hdr.nul_char as *const u16 as *mut u16
}

unsafe extern "win64" fn module_state_scalar_dtor(
    this: *mut AfxModuleState,
    flags: u32,
) -> *mut AfxModuleState {
    afx_module_state_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        c_no_track_object_delete(this.cast());
    }
    this
}

unsafe extern "win64" fn dyn_link_library_scalar_dtor(
    this: *mut CDynLinkLibrary,
    flags: u32,
) -> *mut CDynLinkLibrary {
    if !this.is_null() {
        unlink_dyn_link_library(this);
        if (flags & 1) != 0 {
            operator_delete(this.cast());
        }
    }
    this
}

unsafe extern "win64" fn win_app_scalar_dtor(this: *mut CWinApp, flags: u32) -> *mut CWinApp {
    c_win_app_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        operator_delete(this.cast());
    }
    this
}

static MODULE_STATE_VT: [Ptr; 2] = [
    Ptr(module_state_scalar_dtor as *const ()),
    Ptr(noop_vfunc as *const ()),
];

static DYN_LINK_LIBRARY_VT: [Ptr; 2] = [
    Ptr(noop_vfunc as *const ()),
    Ptr(dyn_link_library_scalar_dtor as *const ()),
];

static WIN_APP_VT: [Ptr; 2] = [
    Ptr(noop_vfunc as *const ()),
    Ptr(win_app_scalar_dtor as *const ()),
];

static EMPTY_MSG_MAP_ENTRIES: [AfxMsgMapEntry; 1] = [AfxMsgMapEntry {
    message: 0,
    code: 0,
    id: 0,
    last_id: 0,
    sig: 0,
    pfn: std::ptr::null(),
}];
unsafe impl Sync for AfxMsgMapEntry {}

static WIN_APP_MSG_MAP: AfxMsgMap = AfxMsgMap {
    pfn_get_base_map: std::ptr::null(),
    entries: EMPTY_MSG_MAP_ENTRIES.as_ptr(),
};

static REGISTERED_VTABLES: Once = Once::new();

/// Register static MFC vtable method pointers in `VA_DISPATCH`.
pub fn register_static_vtables() {
    REGISTERED_VTABLES.call_once(|| {
        for table in [
            &ATL_STRING_MGR_VT[..],
            &MODULE_STATE_VT[..],
            &DYN_LINK_LIBRARY_VT[..],
            &WIN_APP_VT[..],
            &MFC_EXCEPTION_VT[..],
        ] {
            for entry in table {
                let addr = entry.0 as usize;
                if addr != 0 {
                    crate::register_native_va(addr as u64, addr);
                }
            }
        }
    });
}

struct SyncModuleState(Box<AfxModuleState>);
unsafe impl Send for SyncModuleState {}
unsafe impl Sync for SyncModuleState {}

/// Process-wide default `AFX_MODULE_STATE` (`AfxGetAppModuleState()`), used whenever a
/// thread's `m_pModuleState` is `NULL`.
static DEFAULT_MODULE_STATE: LazyLock<SyncModuleState> = LazyLock::new(|| {
    register_static_vtables();
    let mut state: Box<AfxModuleState> = unsafe { Box::new(std::mem::zeroed()) };
    unsafe {
        afx_module_state_ctor(&mut *state, 0, afx_wnd_proc as *const c_void, 0x0e00, 0);
    }
    SyncModuleState(state)
});

fn default_module_state_ptr() -> *mut AfxModuleState {
    &*DEFAULT_MODULE_STATE.0 as *const AfxModuleState as *mut AfxModuleState
}

/// Lock guarding `AFX_MODULE_STATE::library_list_head` mutations across threads.
static LIBRARY_LIST_LOCK: Mutex<()> = Mutex::new(());

/// Heap pointers allocated by `CWinApp` / `AfxWinInit` for `m_pszAppName` / `m_pszExeName`,
/// so `CWinApp::~CWinApp` only frees strings it actually owns.
static OWNED_APP_STRINGS: LazyLock<Mutex<HashSet<usize>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

static AFX_INIT_INSTANCE_COUNT: AtomicI32 = AtomicI32::new(0);

thread_local! {
    static THREAD_STATE: RefCell<Box<AfxThreadState>> = RefCell::new({
        register_static_vtables();
        Box::new(AfxThreadState {
            vtable: MODULE_STATE_VT.as_ptr(),
            module_state: std::ptr::null_mut(),
            prev_module_state: std::ptr::null_mut(),
            current_win_thread: std::ptr::null_mut(),
            temp_map_lock: 0,
            _pad24: 0,
        })
    });
}

fn current_thread_state_ptr() -> *mut AfxThreadState {
    THREAD_STATE.with(|cell| {
        let mut guard = cell.borrow_mut();
        &mut **guard as *mut AfxThreadState
    })
}

unsafe fn alloc_owned_wide_string(src: *const u16) -> *mut u16 {
    if src.is_null() {
        return std::ptr::null_mut();
    }
    let src_bytes = src.cast::<u8>();
    let mut len = 0usize;
    while std::ptr::read_unaligned(src_bytes.add(len * 2) as *const u16) != 0 {
        len += 1;
    }
    let bytes = (len + 1) * std::mem::size_of::<u16>();
    let dst = crate::ucrt::heap::malloc(bytes).cast::<u16>();
    if dst.is_null() {
        return std::ptr::null_mut();
    }
    // Guest pointers may be unaligned; copy via `u8` to avoid aligned `u16` copy UB.
    std::ptr::copy_nonoverlapping(src_bytes, dst.cast::<u8>(), bytes);
    OWNED_APP_STRINGS.lock().unwrap().insert(dst as usize);
    dst
}

unsafe fn free_owned_wide_string(ptr: *mut u16) {
    if ptr.is_null() {
        return;
    }
    if OWNED_APP_STRINGS.lock().unwrap().remove(&(ptr as usize)) {
        crate::ucrt::heap::free(ptr.cast());
    }
}

unsafe fn unlink_dyn_link_library(dll: *mut CDynLinkLibrary) {
    if dll.is_null() {
        return;
    }
    let _g = LIBRARY_LIST_LOCK.lock().unwrap();
    let ms = std::ptr::read_unaligned(std::ptr::addr_of!((*dll).module_state));
    if ms.is_null() {
        return;
    }
    let mut prev: *mut CDynLinkLibrary = std::ptr::null_mut();
    let mut cur = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).library_list_head));
    while !cur.is_null() {
        let next = std::ptr::read_unaligned(std::ptr::addr_of!((*cur).next_dll));
        if cur == dll {
            if prev.is_null() {
                std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ms).library_list_head), next);
            } else {
                std::ptr::write_unaligned(std::ptr::addr_of_mut!((*prev).next_dll), next);
            }
            std::ptr::write_unaligned(
                std::ptr::addr_of_mut!((*dll).next_dll),
                std::ptr::null_mut(),
            );
            return;
        }
        prev = cur;
        cur = next;
    }
}

// ---------------------------------------------------------------------------
// 3. MFC Export Implementations
// ---------------------------------------------------------------------------

/// `#2327` — `AFX_MODULE_STATE* AfxSetModuleState(AFX_MODULE_STATE* pNewState)`.
///
/// Sets the calling thread's active `AFX_MODULE_STATE` and returns the previous one.
unsafe extern "win64" fn afx_set_module_state(
    new_state: *mut AfxModuleState,
) -> *mut AfxModuleState {
    register_static_vtables();
    let ts = current_thread_state_ptr();
    let prev = std::ptr::read_unaligned(std::ptr::addr_of!((*ts).module_state));
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).module_state), new_state);
    prev
}

/// `#2212` — `AFX_MODULE_STATE* AfxGetModuleState()`.
///
/// Returns the calling thread's active `AFX_MODULE_STATE`, falling back to the
/// process-wide default application module state when none is active.
unsafe extern "win64" fn afx_get_module_state() -> *mut AfxModuleState {
    register_static_vtables();
    let ts = current_thread_state_ptr();
    let cur = std::ptr::read_unaligned(std::ptr::addr_of!((*ts).module_state));
    if !cur.is_null() {
        cur
    } else {
        default_module_state_ptr()
    }
}

/// `#323` — `AFX_MAINTAIN_STATE2::AFX_MAINTAIN_STATE2(AFX_MODULE_STATE* pModuleState)`.
unsafe extern "win64" fn afx_maintain_state2_ctor(
    this: *mut AfxMaintainState2,
    new_state: *mut AfxModuleState,
) -> *mut AfxMaintainState2 {
    register_static_vtables();
    if this.is_null() {
        return this;
    }
    let ts = current_thread_state_ptr();
    let prev = std::ptr::read_unaligned(std::ptr::addr_of!((*ts).module_state));
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).prev_module_state), prev);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).thread_state), ts);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).module_state), new_state);
    this
}

/// `#1039` — `AFX_MAINTAIN_STATE2::~AFX_MAINTAIN_STATE2()`.
unsafe extern "win64" fn afx_maintain_state2_dtor(this: *mut AfxMaintainState2) {
    if this.is_null() {
        return;
    }
    let ts = std::ptr::read_unaligned(std::ptr::addr_of!((*this).thread_state));
    if !ts.is_null() {
        let prev = std::ptr::read_unaligned(std::ptr::addr_of!((*this).prev_module_state));
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).module_state), prev);
    }
}

/// `#324` — `AFX_MODULE_STATE::AFX_MODULE_STATE(BOOL bDLL, WNDPROC pfnAfxWndProc, DWORD dwVersion, BOOL bSystem)`.
unsafe extern "win64" fn afx_module_state_ctor(
    this: *mut AfxModuleState,
    is_dll: BOOL,
    pfn_afx_wnd_proc: *const c_void,
    version: DWORD,
    is_system: BOOL,
) -> *mut AfxModuleState {
    register_static_vtables();
    if this.is_null() {
        return this;
    }
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).vtable),
        MODULE_STATE_VT.as_ptr(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).class_list_head),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).class_list_next_offset), 0x20);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).factory_list_head),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).factory_list_next_offset),
        0x40,
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).registered_classes), 0);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).is_dll), (is_dll != 0) as u8);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).is_system),
        (is_system != 0) as u8,
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).user_ctrl), 1);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).unregister_list_str),
        empty_cstring_w_ptr(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).afx_wnd_proc),
        pfn_afx_wnd_proc,
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).version), version);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).library_list_head),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).library_list_next_offset),
        0x78,
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).frame_list_head),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).frame_list_next_offset), 0x20);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).slot_index), -1);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).typelib_cache),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).typelib_cache_map),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).flags_e8), 0);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).thread_local_pad), 0);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).thread_local_extra), 0);
    this
}

/// `#1040` — `AFX_MODULE_STATE::~AFX_MODULE_STATE()`.
unsafe extern "win64" fn afx_module_state_dtor(this: *mut AfxModuleState) {
    if this.is_null() {
        return;
    }
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).vtable),
        MODULE_STATE_VT.as_ptr(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).occ_manager),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).unregister_list_str),
        empty_cstring_w_ptr(),
    );
}

/// `#983` — `CWinApp::CWinApp(LPCWSTR lpszAppName)`.
unsafe extern "win64" fn c_win_app_ctor(this: *mut CWinApp, app_name: *const u16) -> *mut CWinApp {
    register_static_vtables();
    if this.is_null() {
        return this;
    }
    let ms = afx_get_module_state();
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).vtable), WIN_APP_VT.as_ptr());
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).dw_ref), 1);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).module_state), ms);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).main_wnd),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).active_wnd),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).auto_delete), 1);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).thread_handle),
        GetCurrentThread(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).thread_id),
        GetCurrentThreadId(),
    );

    let dup_name = alloc_owned_wide_string(app_name);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).app_name), dup_name);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).instance),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).cmd_line),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).registry_key),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).doc_manager),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).profile_name),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).help_mode), 0);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).exe_name),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).help_file_path),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).recent_file_list),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).lang_resource_dll),
        std::ptr::null_mut(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).num_preview_pages), 0x200);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).restart_manager_flags), 0);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).autosave_interval),
        AFX_DEFAULT_AUTOSAVE_INTERVAL_MS,
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).taskbar_interaction_enabled),
        1,
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).is_win7_or_greater), 1);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).register_with_restart_manager),
        1,
    );

    // Register this CWinApp with the active AFX_MODULE_STATE and thread state.
    if !ms.is_null() {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ms).current_win_app), this);
    }
    let ts = current_thread_state_ptr();
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).current_win_thread), this);
    this
}

/// `#1450` — `CWinApp::~CWinApp()`.
unsafe extern "win64" fn c_win_app_dtor(this: *mut CWinApp) {
    if this.is_null() {
        return;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).vtable), WIN_APP_VT.as_ptr());
    let app_name = std::ptr::read_unaligned(std::ptr::addr_of!((*this).app_name));
    free_owned_wide_string(app_name);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).app_name),
        std::ptr::null_mut(),
    );

    let exe_name = std::ptr::read_unaligned(std::ptr::addr_of!((*this).exe_name));
    free_owned_wide_string(exe_name);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).exe_name),
        std::ptr::null_mut(),
    );

    let ms = std::ptr::read_unaligned(std::ptr::addr_of!((*this).module_state));
    if !ms.is_null() && std::ptr::read_unaligned(std::ptr::addr_of!((*ms).current_win_app)) == this
    {
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*ms).current_win_app),
            std::ptr::null_mut(),
        );
    }
    let ts = current_thread_state_ptr();
    if std::ptr::read_unaligned(std::ptr::addr_of!((*ts).current_win_thread)) == this {
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*ts).current_win_thread),
            std::ptr::null_mut(),
        );
    }
}

/// `#7716` — `BOOL CWinApp::InitInstance()`.
unsafe extern "win64" fn c_win_app_init_instance(this: *mut CWinApp) -> BOOL {
    if !this.is_null() {
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).lang_resource_dll),
            std::ptr::null_mut(),
        );
    }
    AFX_INIT_INSTANCE_COUNT.fetch_add(1, Ordering::Relaxed);
    1
}

/// `#4445` — `int CWinApp::ExitInstance()`.
unsafe extern "win64" fn c_win_app_exit_instance(this: *mut CWinApp) -> i32 {
    if !this.is_null() {
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).lang_resource_dll),
            std::ptr::null_mut(),
        );
    }
    AFX_INIT_INSTANCE_COUNT.fetch_sub(1, Ordering::Relaxed);
    0
}

/// `#7719` — `void CWinApp::InitLibId()` (default no-op in `mfc140u.dll`).
unsafe extern "win64" fn c_win_app_init_lib_id(_this: *mut c_void) {}

/// `#7393` — `const AFX_MSGMAP* CWinApp::GetThisMessageMap()`.
unsafe extern "win64" fn c_win_app_get_this_message_map() -> *const AfxMsgMap {
    &WIN_APP_MSG_MAP
}

/// `#2161` — `void AfxCoreInitModule()`.
///
/// Propagates the process-wide activation context from the default app module state into the
/// current module state if not yet set.
unsafe extern "win64" fn afx_core_init_module() {
    let cur = afx_get_module_state();
    let app = default_module_state_ptr();
    if !cur.is_null() && !app.is_null() {
        let cur_ctx = std::ptr::read_unaligned(std::ptr::addr_of!((*cur).act_ctx));
        if cur_ctx.is_null() {
            let app_ctx = std::ptr::read_unaligned(std::ptr::addr_of!((*app).act_ctx));
            std::ptr::write_unaligned(std::ptr::addr_of_mut!((*cur).act_ctx), app_ctx);
        }
    }
}

/// `#2369` — `BOOL AfxWinInit(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPWSTR lpCmdLine, int nCmdShow)`.
unsafe extern "win64" fn afx_win_init(
    h_instance: HINSTANCE,
    _h_prev_instance: HINSTANCE,
    lp_cmd_line: *mut u16,
    n_cmd_show: i32,
) -> BOOL {
    let ms = afx_get_module_state();
    if ms.is_null() {
        return 0;
    }
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*ms).current_instance_handle),
        h_instance,
    );
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*ms).current_resource_handle),
        h_instance,
    );

    let app = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).current_win_app));
    if !app.is_null() {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*app).instance), h_instance);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*app).cmd_line), lp_cmd_line);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*app).cmd_show), n_cmd_show);

        // Populate m_pszExeName / m_pszAppName from the module filename when unset,
        // matching CWinApp::SetCurrentHandles().
        if crate::try_loader().is_some() {
            let mut buf = [0u16; 260];
            let n = GetModuleFileNameW(h_instance, buf.as_mut_ptr(), buf.len() as DWORD) as usize;
            if n > 0 && n < buf.len() {
                let slice = &buf[..n];
                let start = slice
                    .iter()
                    .rposition(|&c| c == b'\\' as u16 || c == b'/' as u16)
                    .map_or(0, |i| i + 1);
                let end = slice[start..]
                    .iter()
                    .rposition(|&c| c == b'.' as u16)
                    .map_or(n, |i| start + i);
                let mut stem: Vec<u16> = slice[start..end].to_vec();
                stem.push(0);
                if std::ptr::read_unaligned(std::ptr::addr_of!((*app).exe_name)).is_null() {
                    let p = alloc_owned_wide_string(stem.as_ptr());
                    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*app).exe_name), p);
                }
                if std::ptr::read_unaligned(std::ptr::addr_of!((*app).app_name)).is_null() {
                    let p = alloc_owned_wide_string(stem.as_ptr());
                    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*app).app_name), p);
                }
            }
        }
    }
    1
}

/// `#2371` — `void AfxWinTerm()`.
unsafe extern "win64" fn afx_win_term() {
    let ts = current_thread_state_ptr();
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).temp_map_lock), 0);
}

/// `#2234` — `BOOL AfxInitExtensionModule(AFX_EXTENSION_MODULE& state, HMODULE hModule)`.
unsafe extern "win64" fn afx_init_extension_module(
    state: *mut AfxExtensionModule,
    h_module: HMODULE,
) -> BOOL {
    if state.is_null() {
        return 0;
    }
    if std::ptr::read_unaligned(std::ptr::addr_of!((*state).initialized)) != 0 {
        return 1;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*state).initialized), 1);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*state).module), h_module);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*state).resource), h_module);

    let ms = afx_get_module_state();
    if !ms.is_null() {
        let cls_head = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).class_list_head));
        let cls_init = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).class_init));
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*state).first_shared_class),
            cls_head,
        );
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ms).class_list_head), cls_init);

        let fac_head = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).factory_list_head));
        let fac_init = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).factory_init));
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*state).first_shared_factory),
            fac_head,
        );
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ms).factory_list_head), fac_init);
    }
    1
}

/// `#2336` — `void AfxTermExtensionModule(AFX_EXTENSION_MODULE& state, BOOL bAll)`.
unsafe extern "win64" fn afx_term_extension_module(state: *mut AfxExtensionModule, all: BOOL) {
    if state.is_null() || std::ptr::read_unaligned(std::ptr::addr_of!((*state).initialized)) == 0 {
        return;
    }
    let target_mod = std::ptr::read_unaligned(std::ptr::addr_of!((*state).module));
    let ms = afx_get_module_state();
    if !ms.is_null() {
        // Collect matching CDynLinkLibrary nodes under lock, then destroy them outside the lock.
        let mut to_delete: Vec<*mut CDynLinkLibrary> = Vec::new();
        {
            let _g = LIBRARY_LIST_LOCK.lock().unwrap();
            let mut prev: *mut CDynLinkLibrary = std::ptr::null_mut();
            let mut cur = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).library_list_head));
            while !cur.is_null() {
                let next = std::ptr::read_unaligned(std::ptr::addr_of!((*cur).next_dll));
                let cur_mod = std::ptr::read_unaligned(std::ptr::addr_of!((*cur).module));
                if all != 0 || cur_mod == target_mod {
                    if prev.is_null() {
                        std::ptr::write_unaligned(
                            std::ptr::addr_of_mut!((*ms).library_list_head),
                            next,
                        );
                    } else {
                        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*prev).next_dll), next);
                    }
                    std::ptr::write_unaligned(
                        std::ptr::addr_of_mut!((*cur).next_dll),
                        std::ptr::null_mut(),
                    );
                    to_delete.push(cur);
                } else {
                    prev = cur;
                }
                cur = next;
            }
        }
        for node in to_delete {
            let vt = std::ptr::read_unaligned(std::ptr::addr_of!((*node).vtable));
            if vt == DYN_LINK_LIBRARY_VT.as_ptr() {
                operator_delete(node.cast());
            } else if !vt.is_null() {
                let dtor = std::ptr::read_unaligned(vt.add(1)).0 as usize;
                if dtor != 0 {
                    crate::call_guest8(dtor, node as u64, 1, 0, 0, 0, 0, 0, 0);
                }
            }
        }
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*state).initialized), 0);
}

/// `#473` — `CDynLinkLibrary::CDynLinkLibrary(AFX_EXTENSION_MODULE& state, BOOL bSystem)`.
unsafe extern "win64" fn c_dyn_link_library_ctor(
    this: *mut CDynLinkLibrary,
    state: *mut AfxExtensionModule,
    is_system: BOOL,
) -> *mut CDynLinkLibrary {
    register_static_vtables();
    if this.is_null() {
        return this;
    }
    let ms = afx_get_module_state();
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).vtable),
        DYN_LINK_LIBRARY_VT.as_ptr(),
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).dw_ref), 1);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).module_state), ms);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).class_list_next_offset), 0x20);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).factory_list_next_offset),
        0x40,
    );
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).is_system), is_system);
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*this).next_dll),
        std::ptr::null_mut(),
    );

    if !state.is_null() {
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).module),
            std::ptr::read_unaligned(std::ptr::addr_of!((*state).module)),
        );
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).resource),
            std::ptr::read_unaligned(std::ptr::addr_of!((*state).resource)),
        );
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).class_list_head),
            std::ptr::read_unaligned(std::ptr::addr_of!((*state).first_shared_class)),
        );
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*this).factory_list_head),
            std::ptr::read_unaligned(std::ptr::addr_of!((*state).first_shared_factory)),
        );
    }

    if !ms.is_null() {
        let _g = LIBRARY_LIST_LOCK.lock().unwrap();
        let prev_head = std::ptr::read_unaligned(std::ptr::addr_of!((*ms).library_list_head));
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*this).next_dll), prev_head);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ms).library_list_head), this);
    }
    this
}

/// `#2266` — `void AfxLockTempMaps()`.
unsafe extern "win64" fn afx_lock_temp_maps() {
    let ts = current_thread_state_ptr();
    let count = std::ptr::read_unaligned(std::ptr::addr_of!((*ts).temp_map_lock));
    std::ptr::write_unaligned(
        std::ptr::addr_of_mut!((*ts).temp_map_lock),
        count.wrapping_add(1),
    );
}

/// `#2360` — `BOOL AfxUnlockTempMaps(BOOL bDeleteTemps)`.
unsafe extern "win64" fn afx_unlock_temp_maps(_delete_temps: BOOL) -> BOOL {
    let ts = current_thread_state_ptr();
    let count = std::ptr::read_unaligned(std::ptr::addr_of!((*ts).temp_map_lock));
    if count > 0 {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*ts).temp_map_lock), count - 1);
    }
    0
}

/// `#2338` — `void AfxTermThread(HINSTANCE hInstTerm)`.
unsafe extern "win64" fn afx_term_thread(h_inst_term: HINSTANCE) {
    afx_lock_temp_maps();
    afx_unlock_temp_maps(-1);
    if h_inst_term.is_null() {
        let ts = current_thread_state_ptr();
        std::ptr::write_unaligned(
            std::ptr::addr_of_mut!((*ts).current_win_thread),
            std::ptr::null_mut(),
        );
    }
}

/// `#2372` — `LRESULT AfxWndProc(HWND hWnd, UINT nMsg, WPARAM wParam, LPARAM lParam)`.
unsafe extern "win64" fn afx_wnd_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_QUERYAFXWNDPROC {
        return 1;
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

/// `#1489` — `void* operator new(size_t nSize)`.
unsafe extern "win64" fn operator_new(size: usize) -> *mut c_void {
    let ptr = crate::ucrt::heap::malloc(size.max(1));
    if ptr.is_null() {
        afx_throw_memory_exception();
    }
    ptr
}

/// `#1491` — `void operator delete(void* p)` (also ICF'd with `CObject::operator delete`).
unsafe extern "win64" fn operator_delete(ptr: *mut c_void) {
    if !ptr.is_null() {
        crate::ucrt::heap::free(ptr);
    }
}

/// `#1492` — `void CNoTrackObject::operator delete(void* p)`.
unsafe extern "win64" fn c_no_track_object_delete(ptr: *mut c_void) {
    if !ptr.is_null() {
        crate::ucrt::heap::free(ptr);
    }
}

// ---------------------------------------------------------------------------
// 3b. Typed MSVC x64 MFC Exception Objects & RTTI Throw Metadata
// ---------------------------------------------------------------------------

/// `CSimpleException` / `CMemoryException` / `CInvalidArgException` layout (x64):
/// - `+0x00`: `vtable` (`*const Ptr`)
/// - `+0x08`: `m_bAutoDelete` (`BOOL`)
/// - `+0x0C`: `m_bReadyForDelete` (`BOOL`)
/// - `+0x10`: `m_nResourceID` (`u32`)
/// - `+0x14`: `m_bInitialized` (`BOOL`)
/// - `+0x18`: `m_bLoaded` (`BOOL`)
/// - `+0x1C`: `m_szMessage` (`[u16; 128]`)
#[repr(C)]
struct MfcSimpleException {
    vtable: *const Ptr,
    auto_delete: BOOL,
    ready_for_delete: BOOL,
    resource_id: u32,
    initialized: BOOL,
    loaded: BOOL,
    message: [u16; 128],
}

unsafe extern "win64" fn mfc_exc_get_runtime_class(_this: *mut c_void) -> *mut c_void {
    std::ptr::null_mut()
}

unsafe extern "win64" fn mfc_exc_scalar_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    if !this.is_null() && (flags & 1) != 0 {
        let slab = mfc_throw_slab();
        let mem_addr = std::ptr::addr_of!(slab.memory_exc) as usize;
        let inv_addr = std::ptr::addr_of!(slab.invalid_arg_exc) as usize;
        let addr = this as usize;
        if addr != mem_addr && addr != inv_addr {
            operator_delete(this);
        }
    }
    this
}

unsafe extern "win64" fn mfc_exc_noop(_this: *mut c_void) {}

unsafe extern "win64" fn mfc_exc_get_error_message(
    this: *const MfcSimpleException,
    lpsz_error: *mut u16,
    n_max_error: u32,
    pn_help_context: *mut u32,
) -> BOOL {
    if !pn_help_context.is_null() {
        std::ptr::write_unaligned(pn_help_context, 0);
    }
    if this.is_null() || lpsz_error.is_null() || n_max_error == 0 {
        return 0;
    }
    let msg = std::ptr::read_unaligned(std::ptr::addr_of!((*this).message));
    let cap = (n_max_error as usize).saturating_sub(1).min(msg.len());
    let mut n = 0usize;
    while n < cap && msg[n] != 0 {
        std::ptr::write_unaligned(lpsz_error.cast::<u8>().add(n * 2) as *mut u16, msg[n]);
        n += 1;
    }
    std::ptr::write_unaligned(lpsz_error.cast::<u8>().add(n * 2) as *mut u16, 0);
    1
}

unsafe extern "win64" fn mfc_exc_report_error(
    _this: *mut c_void,
    _n_type: u32,
    _n_message_id: u32,
) -> i32 {
    0
}

static MFC_EXCEPTION_VT: [Ptr; 8] = [
    Ptr(mfc_exc_get_runtime_class as *const ()),
    Ptr(mfc_exc_scalar_dtor as *const ()),
    Ptr(mfc_exc_noop as *const ()), // Serialize
    Ptr(mfc_exc_noop as *const ()), // AssertValid
    Ptr(mfc_exc_noop as *const ()), // Dump
    Ptr(mfc_exc_get_error_message as *const ()),
    Ptr(mfc_exc_report_error as *const ()),
    Ptr(std::ptr::null()),
];

/// MSVC x64 `_TypeDescriptor` with fixed 40-byte inline name buffer.
#[repr(C)]
struct MfcTypeDescriptor {
    p_vftable: u64,
    spare: u64,
    name: [u8; 40],
}

impl MfcTypeDescriptor {
    fn new(name: &[u8]) -> Self {
        let mut buf = [0u8; 40];
        let n = name.len().min(buf.len() - 1);
        buf[..n].copy_from_slice(&name[..n]);
        Self {
            p_vftable: 0,
            spare: 0,
            name: buf,
        }
    }
}

/// MSVC x64 `_CatchableType` (28 bytes).
#[repr(C)]
struct MfcCatchableType {
    properties: u32, // 0x01 = CT_IsSimpleType (pointer copy)
    p_type: i32,     // RVA from throw_image_base to MfcTypeDescriptor
    mdisp: i32,
    pdisp: i32,
    vdisp: i32,
    size_or_offset: i32,
    copy_fn: i32,
}

/// MSVC x64 `_CatchableTypeArray` for 5 pointer base types.
#[repr(C)]
struct MfcCatchableTypeArray5 {
    n_catchable_types: i32,
    types: [i32; 5],
}

/// MSVC x64 `_ThrowInfo` (28 bytes).
#[repr(C)]
struct MfcThrowInfo {
    attributes: u32,
    pmfn_unwind: i32,
    p_forward_compat: i32,
    p_catchable_type_array: i32,
}

/// Contiguous slab holding MSVC x64 EH/RTTI metadata and MFC preallocated singleton
/// exception objects so every RVA inside `MfcThrowInfo` / `MfcCatchableTypeArray5` /
/// `MfcCatchableType` resolves cleanly from `throw_image_base = &slab as *const _ as usize`.
#[repr(C)]
struct MfcThrowSlab {
    td_memory: MfcTypeDescriptor,
    td_invalid_arg: MfcTypeDescriptor,
    td_simple: MfcTypeDescriptor,
    td_exception: MfcTypeDescriptor,
    td_object: MfcTypeDescriptor,
    td_void_ptr: MfcTypeDescriptor,
    ct_memory: MfcCatchableType,
    ct_invalid_arg: MfcCatchableType,
    ct_simple: MfcCatchableType,
    ct_exception: MfcCatchableType,
    ct_object: MfcCatchableType,
    ct_void_ptr: MfcCatchableType,
    cta_memory: MfcCatchableTypeArray5,
    cta_invalid_arg: MfcCatchableTypeArray5,
    ti_memory: MfcThrowInfo,
    ti_invalid_arg: MfcThrowInfo,
    memory_exc: MfcSimpleException,
    invalid_arg_exc: MfcSimpleException,
}

struct SyncThrowSlab(Box<MfcThrowSlab>);
unsafe impl Send for SyncThrowSlab {}
unsafe impl Sync for SyncThrowSlab {}

fn make_simple_exception(msg: &str) -> MfcSimpleException {
    let mut wide = [0u16; 128];
    for (dst, ch) in wide.iter_mut().take(127).zip(msg.encode_utf16()) {
        *dst = ch;
    }
    MfcSimpleException {
        vtable: MFC_EXCEPTION_VT.as_ptr(),
        auto_delete: 0, // MFC preallocated singletons use m_bAutoDelete = FALSE
        ready_for_delete: 0,
        resource_id: 0,
        initialized: 1,
        loaded: 1,
        message: wide,
    }
}

static MFC_THROW_SLAB: LazyLock<SyncThrowSlab> = LazyLock::new(|| {
    register_static_vtables();
    let mk_ct = || MfcCatchableType {
        properties: 0x01, // CT_IsSimpleType: 8-byte pointer copy
        p_type: 0,
        mdisp: 0,
        pdisp: -1,
        vdisp: 0,
        size_or_offset: 8,
        copy_fn: 0,
    };
    let mut slab = Box::new(MfcThrowSlab {
        td_memory: MfcTypeDescriptor::new(b".PEAVCMemoryException@@"),
        td_invalid_arg: MfcTypeDescriptor::new(b".PEAVCInvalidArgException@@"),
        td_simple: MfcTypeDescriptor::new(b".PEAVCSimpleException@@"),
        td_exception: MfcTypeDescriptor::new(b".PEAVCException@@"),
        td_object: MfcTypeDescriptor::new(b".PEAVCObject@@"),
        td_void_ptr: MfcTypeDescriptor::new(b".PEAX"),
        ct_memory: mk_ct(),
        ct_invalid_arg: mk_ct(),
        ct_simple: mk_ct(),
        ct_exception: mk_ct(),
        ct_object: mk_ct(),
        ct_void_ptr: mk_ct(),
        cta_memory: MfcCatchableTypeArray5 {
            n_catchable_types: 5,
            types: [0; 5],
        },
        cta_invalid_arg: MfcCatchableTypeArray5 {
            n_catchable_types: 5,
            types: [0; 5],
        },
        ti_memory: MfcThrowInfo {
            attributes: 0,
            pmfn_unwind: 0,
            p_forward_compat: 0,
            p_catchable_type_array: 0,
        },
        ti_invalid_arg: MfcThrowInfo {
            attributes: 0,
            pmfn_unwind: 0,
            p_forward_compat: 0,
            p_catchable_type_array: 0,
        },
        memory_exc: make_simple_exception("Out of memory."),
        invalid_arg_exc: make_simple_exception("Invalid argument."),
    });

    let base = &*slab as *const MfcThrowSlab as usize;
    let rva = |p: *const u8| (p as usize - base) as i32;

    slab.ct_memory.p_type = rva(std::ptr::addr_of!(slab.td_memory).cast());
    slab.ct_invalid_arg.p_type = rva(std::ptr::addr_of!(slab.td_invalid_arg).cast());
    slab.ct_simple.p_type = rva(std::ptr::addr_of!(slab.td_simple).cast());
    slab.ct_exception.p_type = rva(std::ptr::addr_of!(slab.td_exception).cast());
    slab.ct_object.p_type = rva(std::ptr::addr_of!(slab.td_object).cast());
    slab.ct_void_ptr.p_type = rva(std::ptr::addr_of!(slab.td_void_ptr).cast());

    let rva_ct_memory = rva(std::ptr::addr_of!(slab.ct_memory).cast());
    let rva_ct_invalid_arg = rva(std::ptr::addr_of!(slab.ct_invalid_arg).cast());
    let rva_ct_simple = rva(std::ptr::addr_of!(slab.ct_simple).cast());
    let rva_ct_exception = rva(std::ptr::addr_of!(slab.ct_exception).cast());
    let rva_ct_object = rva(std::ptr::addr_of!(slab.ct_object).cast());
    let rva_ct_void_ptr = rva(std::ptr::addr_of!(slab.ct_void_ptr).cast());

    slab.cta_memory.types = [
        rva_ct_memory,
        rva_ct_simple,
        rva_ct_exception,
        rva_ct_object,
        rva_ct_void_ptr,
    ];
    slab.cta_invalid_arg.types = [
        rva_ct_invalid_arg,
        rva_ct_simple,
        rva_ct_exception,
        rva_ct_object,
        rva_ct_void_ptr,
    ];

    slab.ti_memory.p_catchable_type_array = rva(std::ptr::addr_of!(slab.cta_memory).cast());
    slab.ti_invalid_arg.p_catchable_type_array =
        rva(std::ptr::addr_of!(slab.cta_invalid_arg).cast());

    SyncThrowSlab(slab)
});

fn mfc_throw_slab() -> &'static MfcThrowSlab {
    &MFC_THROW_SLAB.0
}

unsafe fn raise_typed_mfc_exception(
    exc: *mut MfcSimpleException,
    throw_info: *const MfcThrowInfo,
    image_base: usize,
) {
    // MSVC throws MFC exceptions by pointer (`CMemoryException*` / `CInvalidArgException*`):
    // `information[1]` points to the 8-byte pointer value on the stack, `information[2]` is
    // the non-null `_ThrowInfo`, and `information[3]` is the base for all RVAs in the slab.
    let mut exc_ptr = exc;
    let args: [u64; 4] = [
        0x1993_0520,
        std::ptr::addr_of_mut!(exc_ptr) as u64,
        throw_info as u64,
        image_base as u64,
    ];
    #[cfg(target_arch = "x86_64")]
    {
        type RaiseExceptionFn = unsafe extern "win64" fn(u32, u32, u32, *const u64);
        let raise: RaiseExceptionFn =
            std::mem::transmute(crate::unwind::RaiseException as *const ());
        raise(
            crate::unwind::CXX_EXCEPTION,
            crate::unwind::EXCEPTION_NONCONTINUABLE,
            4,
            args.as_ptr(),
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = crate::unwind::RaiseException(
            crate::unwind::CXX_EXCEPTION as u64,
            crate::unwind::EXCEPTION_NONCONTINUABLE as u64,
            4,
            args.as_ptr() as u64,
            0,
            0,
        );
    }
}

/// `#2346` — `void AfxThrowMemoryException()`.
unsafe extern "win64" fn afx_throw_memory_exception() {
    let slab = mfc_throw_slab();
    let base = slab as *const MfcThrowSlab as usize;
    let exc = std::ptr::addr_of!(slab.memory_exc) as *mut MfcSimpleException;
    raise_typed_mfc_exception(exc, std::ptr::addr_of!(slab.ti_memory), base);
}

/// `#2344` — `void AfxThrowInvalidArgException()`.
unsafe extern "win64" fn afx_throw_invalid_arg_exception() {
    let slab = mfc_throw_slab();
    let base = slab as *const MfcThrowSlab as usize;
    let exc = std::ptr::addr_of!(slab.invalid_arg_exc) as *mut MfcSimpleException;
    raise_typed_mfc_exception(exc, std::ptr::addr_of!(slab.ti_invalid_arg), base);
}

/// `#3728` — `void CException::Delete()`.
unsafe extern "win64" fn c_exception_delete(this: *mut u8) {
    if this.is_null() {
        return;
    }
    // `CException::m_bAutoDelete` is at offset +0x08 (BOOL).
    let auto_delete = std::ptr::read_unaligned(this.add(8) as *const BOOL);
    if auto_delete != 0 {
        let vt = std::ptr::read_unaligned(this as *const *const Ptr);
        if !vt.is_null() {
            let dtor = std::ptr::read_unaligned(vt.add(1)).0 as usize;
            if dtor != 0 {
                crate::call_guest8(dtor, this as u64, 1, 0, 0, 0, 0, 0, 0);
                return;
            }
        }
        operator_delete(this.cast());
    }
}

// ---------------------------------------------------------------------------
// 4. Export Table (`EXTRA_EXPORTS`) & Ordinal Table (`ORDINALS`)
// ---------------------------------------------------------------------------

pub static EXTRA_EXPORTS: &[Export] = &[
    Export {
        name: "??0AFX_MAINTAIN_STATE2@@QEAA@PEAVAFX_MODULE_STATE@@@Z",
        ptr: Ptr(afx_maintain_state2_ctor as *const ()),
    },
    Export {
        name: "??0AFX_MODULE_STATE@@QEAA@HP6A_JPEAUHWND__@@I_K_J@ZKH@Z",
        ptr: Ptr(afx_module_state_ctor as *const ()),
    },
    Export {
        name: "??0CDynLinkLibrary@@QEAA@AEAUAFX_EXTENSION_MODULE@@H@Z",
        ptr: Ptr(c_dyn_link_library_ctor as *const ()),
    },
    Export {
        name: "??0CWinApp@@QEAA@PEB_W@Z",
        ptr: Ptr(c_win_app_ctor as *const ()),
    },
    Export {
        name: "??1AFX_MAINTAIN_STATE2@@QEAA@XZ",
        ptr: Ptr(afx_maintain_state2_dtor as *const ()),
    },
    Export {
        name: "??1AFX_MODULE_STATE@@UEAA@XZ",
        ptr: Ptr(afx_module_state_dtor as *const ()),
    },
    Export {
        name: "??1CWinApp@@UEAA@XZ",
        ptr: Ptr(c_win_app_dtor as *const ()),
    },
    Export {
        name: "??2@YAPEAX_K@Z",
        ptr: Ptr(operator_new as *const ()),
    },
    Export {
        name: "??3@YAXPEAX@Z",
        ptr: Ptr(operator_delete as *const ()),
    },
    Export {
        name: "??3CNoTrackObject@@SAXPEAX@Z",
        ptr: Ptr(c_no_track_object_delete as *const ()),
    },
    Export {
        name: "?AfxCoreInitModule@@YAXXZ",
        ptr: Ptr(afx_core_init_module as *const ()),
    },
    Export {
        name: "?AfxGetModuleState@@YAPEAVAFX_MODULE_STATE@@XZ",
        ptr: Ptr(afx_get_module_state as *const ()),
    },
    Export {
        name: "?AfxInitExtensionModule@@YAHAEAUAFX_EXTENSION_MODULE@@PEAUHINSTANCE__@@@Z",
        ptr: Ptr(afx_init_extension_module as *const ()),
    },
    Export {
        name: "?AfxLockTempMaps@@YAXXZ",
        ptr: Ptr(afx_lock_temp_maps as *const ()),
    },
    Export {
        name: "?AfxSetModuleState@@YAPEAVAFX_MODULE_STATE@@PEAV1@@Z",
        ptr: Ptr(afx_set_module_state as *const ()),
    },
    Export {
        name: "?AfxTermExtensionModule@@YAXAEAUAFX_EXTENSION_MODULE@@H@Z",
        ptr: Ptr(afx_term_extension_module as *const ()),
    },
    Export {
        name: "?AfxTermThread@@YAXPEAUHINSTANCE__@@@Z",
        ptr: Ptr(afx_term_thread as *const ()),
    },
    Export {
        name: "?AfxThrowInvalidArgException@@YAXXZ",
        ptr: Ptr(afx_throw_invalid_arg_exception as *const ()),
    },
    Export {
        name: "?AfxThrowMemoryException@@YAXXZ",
        ptr: Ptr(afx_throw_memory_exception as *const ()),
    },
    Export {
        name: "?AfxUnlockTempMaps@@YAHH@Z",
        ptr: Ptr(afx_unlock_temp_maps as *const ()),
    },
    Export {
        name: "?AfxWinInit@@YAHPEAUHINSTANCE__@@0PEA_WH@Z",
        ptr: Ptr(afx_win_init as *const ()),
    },
    Export {
        name: "?AfxWinTerm@@YAXXZ",
        ptr: Ptr(afx_win_term as *const ()),
    },
    Export {
        name: "?AfxWndProc@@YA_JPEAUHWND__@@I_K_J@Z",
        ptr: Ptr(afx_wnd_proc as *const ()),
    },
    Export {
        name: "?Delete@CException@@QEAAXXZ",
        ptr: Ptr(c_exception_delete as *const ()),
    },
    Export {
        name: "?ExitInstance@CWinApp@@UEAAHXZ",
        ptr: Ptr(c_win_app_exit_instance as *const ()),
    },
    Export {
        name: "?GetThisMessageMap@CWinApp@@KAPEBUAFX_MSGMAP@@XZ",
        ptr: Ptr(c_win_app_get_this_message_map as *const ()),
    },
    Export {
        name: "?InitInstance@CWinApp@@UEAAHXZ",
        ptr: Ptr(c_win_app_init_instance as *const ()),
    },
    Export {
        name: "?InitLibId@CWinApp@@UEAAXXZ",
        ptr: Ptr(c_win_app_init_lib_id as *const ()),
    },
];

pub static ORDINALS: &[(u16, &str)] = &[
    (323, "??0AFX_MAINTAIN_STATE2@@QEAA@PEAVAFX_MODULE_STATE@@@Z"),
    (
        324,
        "??0AFX_MODULE_STATE@@QEAA@HP6A_JPEAUHWND__@@I_K_J@ZKH@Z",
    ),
    (
        473,
        "??0CDynLinkLibrary@@QEAA@AEAUAFX_EXTENSION_MODULE@@H@Z",
    ),
    (983, "??0CWinApp@@QEAA@PEB_W@Z"),
    (1039, "??1AFX_MAINTAIN_STATE2@@QEAA@XZ"),
    (1040, "??1AFX_MODULE_STATE@@UEAA@XZ"),
    (1450, "??1CWinApp@@UEAA@XZ"),
    (1489, "??2@YAPEAX_K@Z"),
    (1491, "??3@YAXPEAX@Z"),
    (1492, "??3CNoTrackObject@@SAXPEAX@Z"),
    (2161, "?AfxCoreInitModule@@YAXXZ"),
    (2212, "?AfxGetModuleState@@YAPEAVAFX_MODULE_STATE@@XZ"),
    (
        2234,
        "?AfxInitExtensionModule@@YAHAEAUAFX_EXTENSION_MODULE@@PEAUHINSTANCE__@@@Z",
    ),
    (2266, "?AfxLockTempMaps@@YAXXZ"),
    (2327, "?AfxSetModuleState@@YAPEAVAFX_MODULE_STATE@@PEAV1@@Z"),
    (
        2336,
        "?AfxTermExtensionModule@@YAXAEAUAFX_EXTENSION_MODULE@@H@Z",
    ),
    (2338, "?AfxTermThread@@YAXPEAUHINSTANCE__@@@Z"),
    (2344, "?AfxThrowInvalidArgException@@YAXXZ"),
    (2346, "?AfxThrowMemoryException@@YAXXZ"),
    (2360, "?AfxUnlockTempMaps@@YAHH@Z"),
    (2369, "?AfxWinInit@@YAHPEAUHINSTANCE__@@0PEA_WH@Z"),
    (2371, "?AfxWinTerm@@YAXXZ"),
    (2372, "?AfxWndProc@@YA_JPEAUHWND__@@I_K_J@Z"),
    (3728, "?Delete@CException@@QEAAXXZ"),
    (4445, "?ExitInstance@CWinApp@@UEAAHXZ"),
    (7393, "?GetThisMessageMap@CWinApp@@KAPEBUAFX_MSGMAP@@XZ"),
    (7716, "?InitInstance@CWinApp@@UEAAHXZ"),
    (7719, "?InitLibId@CWinApp@@UEAAXXZ"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_2327_and_lifecycle_exports_resolve() {
        for &(ord, name) in ORDINALS {
            let by_ord = crate::lookup_ordinal("mfc140u.dll", ord);
            let by_name = crate::lookup("mfc140u.dll", name);
            assert!(by_ord.is_some(), "missing ordinal #{ord} ({name})");
            assert_eq!(by_ord, by_name, "ordinal #{ord} mismatch with {name}");
        }
        assert_eq!(
            crate::lookup_ordinal("MFC140U.DLL", 2327),
            Some(afx_set_module_state as *const ())
        );
        assert!(crate::lookup_ordinal("mfc140u.dll", 65535).is_none());
    }

    #[test]
    fn afx_set_and_get_module_state_is_thread_local() {
        unsafe {
            let mut mod_a: AfxModuleState = std::mem::zeroed();
            let mut mod_b: AfxModuleState = std::mem::zeroed();
            afx_module_state_ctor(&mut mod_a, 1, afx_wnd_proc as *const c_void, 0x0e00, 0);
            afx_module_state_ctor(&mut mod_b, 1, afx_wnd_proc as *const c_void, 0x0e00, 0);

            // Default fallback is non-null process-wide module state.
            let initial = afx_set_module_state(std::ptr::null_mut());
            let fallback = afx_get_module_state();
            assert!(!fallback.is_null());

            // Switching module state returns previous value and updates AfxGetModuleState.
            assert!(afx_set_module_state(&mut mod_a).is_null());
            assert_eq!(afx_get_module_state(), &mut mod_a as *mut AfxModuleState);

            // Another thread sees its own independent TLS state (fallback, not mod_a).
            let mod_a_addr = &mut mod_a as *mut AfxModuleState as usize;
            let other_saw = std::thread::spawn(move || {
                let got = afx_get_module_state() as usize;
                assert_ne!(got, mod_a_addr);
                got
            })
            .join()
            .unwrap();
            assert_eq!(other_saw, fallback as usize);

            // AFX_MAINTAIN_STATE2 pushes mod_b and restores mod_a on dtor.
            let mut guard: AfxMaintainState2 = std::mem::zeroed();
            afx_maintain_state2_ctor(&mut guard, &mut mod_b);
            assert_eq!(afx_get_module_state(), &mut mod_b as *mut AfxModuleState);
            assert_eq!(guard.prev_module_state, &mut mod_a as *mut AfxModuleState);
            afx_maintain_state2_dtor(&mut guard);
            assert_eq!(afx_get_module_state(), &mut mod_a as *mut AfxModuleState);

            // Restore initial state.
            assert_eq!(
                afx_set_module_state(initial),
                &mut mod_a as *mut AfxModuleState
            );
            afx_module_state_dtor(&mut mod_b);
            afx_module_state_dtor(&mut mod_a);
        }
    }

    #[test]
    fn regular_mfc_dll_full_attach_and_detach_lifecycle() {
        unsafe {
            let fake_hinst = 0x18000_0000usize as HINSTANCE;
            let mut mod_state: AfxModuleState = std::mem::zeroed();
            let mut ext_mod: AfxExtensionModule = std::mem::zeroed();
            let mut app: CWinApp = std::mem::zeroed();

            // 1. RawDllMain(DLL_PROCESS_ATTACH): AfxSetModuleState(&mod_state)
            let saved = afx_set_module_state(&mut mod_state);

            // 2. _CRT_INIT static ctors: AFX_MODULE_STATE::ctor + CWinApp::ctor
            afx_module_state_ctor(&mut mod_state, 1, afx_wnd_proc as *const c_void, 0x0e00, 0);
            assert_eq!(mod_state.is_dll, 1);
            assert_eq!(mod_state.version, 0x0e00);
            assert_eq!(mod_state.library_list_next_offset, 0x78);

            let app_name: [u16; 5] = [b'T' as u16, b'e' as u16, b's' as u16, b't' as u16, 0];
            c_win_app_ctor(&mut app, app_name.as_ptr());
            assert_eq!(mod_state.current_win_app, &mut app as *mut CWinApp);
            assert_eq!(app.module_state, &mut mod_state as *mut AfxModuleState);

            // 3. DllMain(DLL_PROCESS_ATTACH): AfxCoreInitModule, AfxWinInit, InitInstance,
            //    AfxInitExtensionModule, new CDynLinkLibrary, restore module state.
            afx_core_init_module();
            assert_eq!(
                afx_win_init(fake_hinst, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                1
            );
            assert_eq!(mod_state.current_instance_handle, fake_hinst);
            assert_eq!(app.instance, fake_hinst);
            assert_eq!(c_win_app_init_instance(&mut app), 1);

            assert_eq!(afx_init_extension_module(&mut ext_mod, fake_hinst), 1);
            assert_eq!(ext_mod.initialized, 1);
            assert_eq!(ext_mod.module, fake_hinst);

            let dyn_lib =
                operator_new(std::mem::size_of::<CDynLinkLibrary>()).cast::<CDynLinkLibrary>();
            assert!(!dyn_lib.is_null());
            c_dyn_link_library_ctor(dyn_lib, &mut ext_mod, 0);
            assert_eq!(mod_state.library_list_head, dyn_lib);
            assert_eq!((*dyn_lib).module, fake_hinst);

            assert_eq!(
                afx_set_module_state(saved),
                &mut mod_state as *mut AfxModuleState
            );

            // 4. DllMain(DLL_THREAD_DETACH): AFX_MAINTAIN_STATE2 + temp maps + AfxTermThread
            let mut maintain: AfxMaintainState2 = std::mem::zeroed();
            afx_maintain_state2_ctor(&mut maintain, &mut mod_state);
            afx_lock_temp_maps();
            assert_eq!(afx_unlock_temp_maps(-1), 0);
            afx_term_thread(fake_hinst);
            afx_maintain_state2_dtor(&mut maintain);

            // 5. DllMain(DLL_PROCESS_DETACH) + atexit dtors
            let saved2 = afx_set_module_state(&mut mod_state);
            assert_eq!(c_win_app_exit_instance(&mut app), 0);
            afx_lock_temp_maps();
            assert_eq!(afx_unlock_temp_maps(-1), 0);
            afx_win_term();
            afx_term_extension_module(&mut ext_mod, 1);
            assert_eq!(ext_mod.initialized, 0);
            assert!(mod_state.library_list_head.is_null());

            c_win_app_dtor(&mut app);
            assert!(mod_state.current_win_app.is_null());
            afx_module_state_dtor(&mut mod_state);
            afx_set_module_state(saved2);
        }
    }

    #[test]
    fn afx_wnd_proc_answers_query_message() {
        unsafe {
            assert_eq!(
                afx_wnd_proc(std::ptr::null_mut(), WM_QUERYAFXWNDPROC, 0, 0),
                1
            );
        }
    }

    #[test]
    fn null_pointers_and_operator_new_delete_are_safe() {
        unsafe {
            assert!(afx_maintain_state2_ctor(std::ptr::null_mut(), std::ptr::null_mut()).is_null());
            afx_maintain_state2_dtor(std::ptr::null_mut());
            assert!(
                afx_module_state_ctor(std::ptr::null_mut(), 1, std::ptr::null(), 0, 0).is_null()
            );
            afx_module_state_dtor(std::ptr::null_mut());
            assert!(c_win_app_ctor(std::ptr::null_mut(), std::ptr::null()).is_null());
            c_win_app_dtor(std::ptr::null_mut());
            assert_eq!(
                afx_init_extension_module(std::ptr::null_mut(), std::ptr::null_mut()),
                0
            );
            afx_term_extension_module(std::ptr::null_mut(), 1);
            assert!(
                c_dyn_link_library_ctor(std::ptr::null_mut(), std::ptr::null_mut(), 0).is_null()
            );
            c_exception_delete(std::ptr::null_mut());
            operator_delete(std::ptr::null_mut());
            c_no_track_object_delete(std::ptr::null_mut());

            let p = operator_new(64);
            assert!(!p.is_null());
            std::ptr::write_bytes(p as *mut u8, 0xAB, 64);
            operator_delete(p);
        }
    }

    #[test]
    fn c_win_app_copies_odd_aligned_wide_app_name() {
        unsafe {
            let mut mod_state: AfxModuleState = std::mem::zeroed();
            afx_module_state_ctor(&mut mod_state, 1, afx_wnd_proc as *const c_void, 0x0e00, 0);
            let saved = afx_set_module_state(&mut mod_state);

            // Build a UTF-16LE string starting at an odd byte address (unaligned for u16).
            let expected: Vec<u16> = "OddAlignedApp\0".encode_utf16().collect();
            let mut raw = vec![0xFFu8; 1 + expected.len() * 2 + 1];
            for (i, &ch) in expected.iter().enumerate() {
                raw[1 + i * 2..1 + i * 2 + 2].copy_from_slice(&ch.to_le_bytes());
            }
            let odd_ptr = raw.as_ptr().add(1) as *const u16;
            assert_eq!((odd_ptr as usize) % 2, 1);

            let mut app: CWinApp = std::mem::zeroed();
            c_win_app_ctor(&mut app, odd_ptr);
            assert!(!app.app_name.is_null());
            for (i, &ch) in expected.iter().enumerate() {
                assert_eq!(std::ptr::read_unaligned(app.app_name.add(i)), ch);
            }

            c_win_app_dtor(&mut app);
            afx_module_state_dtor(&mut mod_state);
            afx_set_module_state(saved);
        }
    }

    #[test]
    fn mfc_throw_metadata_matches_msvc_cxx_abi() {
        unsafe {
            let slab = mfc_throw_slab();
            let base = slab as *const MfcThrowSlab as usize;

            let check_throw = |ti: *const MfcThrowInfo, expected_types: &[&str]| {
                assert!(!ti.is_null());
                let attrs = std::ptr::read_unaligned(ti as *const u32);
                let unwind_rva = std::ptr::read_unaligned((ti as *const u8).add(4) as *const i32);
                let cta_rva = std::ptr::read_unaligned((ti as *const u8).add(12) as *const i32);
                assert_eq!(attrs, 0);
                assert_eq!(unwind_rva, 0);
                assert!(cta_rva > 0);

                let cta = (base as isize + cta_rva as isize) as *const i32;
                let count = std::ptr::read_unaligned(cta) as usize;
                assert_eq!(count, expected_types.len());

                for (i, &want) in expected_types.iter().enumerate() {
                    let ct_rva = std::ptr::read_unaligned(cta.add(1 + i));
                    let ct = (base as isize + ct_rva as isize) as *const u8;
                    let props = std::ptr::read_unaligned(ct as *const u32);
                    let td_rva = std::ptr::read_unaligned(ct.add(4) as *const i32);
                    let mdisp = std::ptr::read_unaligned(ct.add(8) as *const i32);
                    let pdisp = std::ptr::read_unaligned(ct.add(12) as *const i32);
                    let vdisp = std::ptr::read_unaligned(ct.add(16) as *const i32);
                    let size = std::ptr::read_unaligned(ct.add(20) as *const i32);
                    assert_eq!(props, 0x01, "expected CT_IsSimpleType for {want}");
                    assert_eq!([mdisp, pdisp, vdisp], [0, -1, 0]);
                    assert_eq!(size, 8);

                    let td = (base as isize + td_rva as isize) as *const u8;
                    let got = std::ffi::CStr::from_ptr(td.add(16) as *const libc::c_char)
                        .to_str()
                        .unwrap();
                    assert_eq!(got, want);
                }
            };

            check_throw(
                std::ptr::addr_of!(slab.ti_memory),
                &[
                    ".PEAVCMemoryException@@",
                    ".PEAVCSimpleException@@",
                    ".PEAVCException@@",
                    ".PEAVCObject@@",
                    ".PEAX",
                ],
            );
            check_throw(
                std::ptr::addr_of!(slab.ti_invalid_arg),
                &[
                    ".PEAVCInvalidArgException@@",
                    ".PEAVCSimpleException@@",
                    ".PEAVCException@@",
                    ".PEAVCObject@@",
                    ".PEAX",
                ],
            );

            // Verify preallocated singletons have m_bAutoDelete == FALSE (safe Delete())
            // and GetErrorMessage works cleanly even on an odd-aligned destination buffer.
            let mut raw = vec![0xEEu8; 1 + 64 * 2];
            let odd_dst = raw.as_mut_ptr().add(1) as *mut u16;
            let mut help_ctx = 99u32;
            assert_eq!(
                mfc_exc_get_error_message(
                    std::ptr::addr_of!(slab.memory_exc),
                    odd_dst,
                    64,
                    &mut help_ctx
                ),
                1
            );
            assert_eq!(help_ctx, 0);
            let mut out = Vec::new();
            for i in 0..64 {
                let ch = std::ptr::read_unaligned(odd_dst.cast::<u8>().add(i * 2) as *const u16);
                if ch == 0 {
                    break;
                }
                out.push(ch);
            }
            assert_eq!(String::from_utf16(&out).unwrap(), "Out of memory.");

            c_exception_delete(std::ptr::addr_of!(slab.memory_exc) as *mut u8);
            c_exception_delete(std::ptr::addr_of!(slab.invalid_arg_exc) as *mut u8);
        }
    }
}
