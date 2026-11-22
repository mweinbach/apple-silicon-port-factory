//! `cohtml.windowsdesktop.dll` — native Rust replacement for Coherent Gameface (`1.40.3.4`).
//!
//! `AlanWake2.exe` imports 69 symbols from `cohtml.windowsdesktop.dll`. Because
//! `cohtml.windowsdesktop.dll` is the sole importer of `renoircore.windowsdesktop.dll`,
//! `v8.dll`, `v8_libbase.dll`, and `v8_libplatform.dll`, replacing `cohtml.windowsdesktop.dll`
//! with a native builtin eliminates all 5 Cohtml/Renoir/V8 PE DLLs at once.

use crate::{Export, Ptr};
use std::ffi::{c_void, CStr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Once;

pub static ORDINALS: &[(u16, &str)] = &[];

static NEXT_VIEW_ID: AtomicU32 = AtomicU32::new(1);
static NEXT_FRAME_ID: AtomicU32 = AtomicU32::new(1);
static REGISTER_VTABLES: Once = Once::new();

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ScreenInfo {
    pub available_width: u32,  // +0x00
    pub available_height: u32, // +0x04
    pub width: u32,            // +0x08
    pub height: u32,           // +0x0c
    pub color_depth: u32,      // +0x10
    pub pixel_depth: u32,      // +0x14
}
const _: () = assert!(std::mem::size_of::<ScreenInfo>() == 0x18);

static EMPTY_CSTR: &[u8] = b"\0";

static BASE_OBJECT_VTBL: [Ptr; 16] = [
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
    Ptr(base_vfunc_ret0 as *const ()),
];

#[repr(C)]
pub struct CohtmlDummyObject {
    pub vtbl: *const [Ptr; 16],
}

unsafe impl Send for CohtmlDummyObject {}
unsafe impl Sync for CohtmlDummyObject {}

static DUMMY_SCRIPT_COMPILER: CohtmlDummyObject = CohtmlDummyObject {
    vtbl: &BASE_OBJECT_VTBL,
};

static DUMMY_TASK_SCHEDULER: CohtmlDummyObject = CohtmlDummyObject {
    vtbl: &BASE_OBJECT_VTBL,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CohtmlBinder {
    pub vtbl: *const [Ptr; 80], // +0x00
    pub mode: u32,              // +0x08
    pub bool_val: u8,           // +0x0c
    pub _pad: [u8; 3],          // +0x0d..+0x10
    pub str_ptr: *const u8,     // +0x10
    pub str_len: usize,         // +0x18
    pub f64_val: f64,           // +0x20
}

unsafe impl Send for CohtmlBinder {}
unsafe impl Sync for CohtmlBinder {}

impl CohtmlBinder {
    const fn new() -> Self {
        Self {
            vtbl: &BINDER_VTBL,
            mode: 0,
            bool_val: 0,
            _pad: [0; 3],
            str_ptr: std::ptr::null(),
            str_len: 0,
            f64_val: 0.0,
        }
    }
}

static mut FALLBACK_BINDER: CohtmlBinder = CohtmlBinder::new();

#[repr(C)]
struct CohtmlArgsBinder {
    vtbl: *const [Ptr; 16],
    arg_count: i32,
    _pad: i32,
    args: [CohtmlBinder; 5],
}

unsafe extern "win64" fn args_binder_get_arg(
    this: *mut CohtmlArgsBinder,
    index: i32,
) -> *mut CohtmlBinder {
    if this.is_null() {
        return &raw mut FALLBACK_BINDER;
    }
    let idx = index.clamp(0, 4) as usize;
    &raw mut (*this).args[idx]
}

unsafe extern "win64" fn args_binder_get_count(this: *const CohtmlArgsBinder) -> i32 {
    if this.is_null() {
        0
    } else {
        (*this).arg_count
    }
}

static ARGS_BINDER_VTBL: [Ptr; 16] = [
    Ptr(base_vfunc_ret0 as *const ()),       // 0 (+0x00)
    Ptr(args_binder_get_arg as *const ()),   // 1 (+0x08): GetArgument(index)
    Ptr(base_vfunc_ret0 as *const ()),       // 2 (+0x10)
    Ptr(base_vfunc_ret0 as *const ()),       // 3 (+0x18)
    Ptr(base_vfunc_ret0 as *const ()),       // 4 (+0x20): End
    Ptr(args_binder_get_count as *const ()), // 5 (+0x28): GetArgumentsCount
    Ptr(base_vfunc_ret0 as *const ()),       // 6 (+0x30): HasError (0 = false)
    Ptr(base_vfunc_ret0 as *const ()),       // 7 (+0x38)
    Ptr(base_vfunc_ret0 as *const ()),       // 8 (+0x40)
    Ptr(base_vfunc_ret0 as *const ()),       // 9 (+0x48)
    Ptr(base_vfunc_ret0 as *const ()),       // 10 (+0x50)
    Ptr(base_vfunc_ret0 as *const ()),       // 11 (+0x58)
    Ptr(base_vfunc_ret0 as *const ()),       // 12 (+0x60)
    Ptr(base_vfunc_ret0 as *const ()),       // 13 (+0x68)
    Ptr(base_vfunc_ret0 as *const ()),       // 14 (+0x70)
    Ptr(base_vfunc_ret0 as *const ()),       // 15 (+0x78)
];

#[repr(C)]
pub struct CohtmlLibrary {
    pub vtbl: *const [Ptr; 16], // +0x00
    pub _pad: [u64; 11],        // +0x08..+0x60
}

#[repr(C)]
pub struct CohtmlSystem {
    pub vtbl: *const [Ptr; 30], // +0x00
    pub id: u32,                // +0x08
    pub _pad: [u8; 0x864],      // +0x0c..+0x870
}

#[repr(C)]
pub struct CohtmlSystemRenderer {
    pub vtbl: *const [Ptr; 10], // +0x00
    pub backend: *mut c_void,   // +0x08
    pub _pad: [u64; 11],        // +0x10..+0x68
}

#[repr(C)]
pub struct CohtmlViewRenderer {
    pub vtbl: *const [Ptr; 12], // +0x00
    pub view: *mut CohtmlView,  // +0x08
    pub _pad: [u64; 6],         // +0x10..+0x40
}

#[repr(C)]
pub struct CohtmlView {
    pub vtbl: *const [Ptr; 80],                        // +0x00
    pub listener: *mut c_void,                         // +0x08
    pub id: u32,                                       // +0x10
    pub width: u32,                                    // +0x14
    pub height: u32,                                   // +0x18
    pub continuous_repaint: u8,                        // +0x1c
    pub ready_for_bindings: u8,                        // +0x1d
    pub _reserved_flags: [u8; 2],                      // +0x1e..+0x20
    pub binder: CohtmlBinder,                          // +0x20..+0x48
    pub advance_count: u32,                            // +0x48
    pub _pad1: u32,                                    // +0x4c
    pub _reserved_handlers: [usize; 4],                // +0x50..+0x70
    pub _pad: [u8; 0x1b0],
}

unsafe extern "win64" fn base_vfunc_ret0() -> usize {
    0
}

// ---------------------------------------------------------------------------
// cohtml::Binder vtable (64 slots)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn binder_dtor(this: *mut CohtmlBinder, _flags: u32) -> *mut CohtmlBinder {
    this
}

/// Slot 2 (`+0x10`): `Binder::RegisterType(this, sret: *mut TypeDescription, name, obj) -> *mut TypeDescription`.
/// Remedy's `CoherentBind` functions (`0x142501640`, `0x1425015d0`, `0x1424e4de0`, `0x141df4980`,
/// `0x141e052f0`) pass an uninitialized 16-byte stack `TypeDescription` in `rdx` (`sret`) and
/// immediately test `qword ptr [sret] != 0` before chaining `.Property(...)` calls. Zeroing `sret`
/// makes every `CoherentBind` cleanly skip property registration without dereferencing null.
unsafe extern "win64" fn binder_register_type(
    _this: *mut CohtmlBinder,
    sret: *mut [usize; 2],
    _name: *const u8,
    _obj: *mut c_void,
) -> *mut [usize; 2] {
    if !sret.is_null() {
        *sret = [0, 0];
    }
    sret
}

unsafe extern "win64" fn binder_set_mode(this: *mut CohtmlBinder, mode: u32) -> usize {
    if !this.is_null() {
        (*this).mode = mode;
    }
    0
}

unsafe extern "win64" fn binder_get_mode(this: *const CohtmlBinder) -> usize {
    if this.is_null() {
        0
    } else {
        (*this).mode as usize
    }
}

unsafe extern "win64" fn binder_read_zero(_this: *mut CohtmlBinder, out: *mut u64) -> usize {
    if !out.is_null() {
        std::ptr::write_unaligned(out, 0);
    }
    0
}

/// Slot 55 (`+0x1b8`): `Binder::Read(this, out_str: *mut *const u8, out_len: *mut usize)`.
/// `0x140264100` (`CoherentRead` for `r::InplaceString`) calls this slot and immediately runs
/// `strlen(*out_str)` without a null check, so `*out_str` must point to a valid NUL-terminated string.
unsafe extern "win64" fn binder_read_string(
    this: *mut CohtmlBinder,
    out_str: *mut *const u8,
    out_len: *mut usize,
) -> usize {
    let (ptr, len) = if !this.is_null() && !(*this).str_ptr.is_null() {
        ((*this).str_ptr, (*this).str_len)
    } else {
        (EMPTY_CSTR.as_ptr(), 0)
    };
    if !out_str.is_null() {
        std::ptr::write_unaligned(out_str, ptr);
    }
    if !out_len.is_null() {
        std::ptr::write_unaligned(out_len, len);
    }
    0
}

unsafe extern "win64" fn binder_read_f64(this: *mut CohtmlBinder, out_val: *mut f64) -> usize {
    if !out_val.is_null() {
        let v = if this.is_null() { 0.0 } else { (*this).f64_val };
        std::ptr::write_unaligned(out_val, v);
    }
    0
}

unsafe extern "win64" fn binder_read_bool(this: *mut CohtmlBinder, out_val: *mut u8) -> usize {
    if !out_val.is_null() {
        let v = if this.is_null() { 0 } else { (*this).bool_val };
        std::ptr::write_unaligned(out_val, v);
    }
    0
}

static BINDER_VTBL: [Ptr; 80] = [
    Ptr(binder_dtor as *const ()),          // 0 (+0x000): ~Binder
    Ptr(base_vfunc_ret0 as *const ()),      // 1 (+0x008): OnBindingsReleased
    Ptr(binder_register_type as *const ()), // 2 (+0x010): RegisterType (sret TypeDescription)
    Ptr(base_vfunc_ret0 as *const ()),      // 3 (+0x018): TryBindTypeByHands
    Ptr(base_vfunc_ret0 as *const ()),      // 4 (+0x020): EndType
    Ptr(base_vfunc_ret0 as *const ()),      // 5 (+0x028): ReadType / BeginType
    Ptr(base_vfunc_ret0 as *const ()),      // 6 (+0x030)
    Ptr(base_vfunc_ret0 as *const ()),      // 7 (+0x038)
    Ptr(base_vfunc_ret0 as *const ()),      // 8 (+0x040): PeekValueType
    Ptr(base_vfunc_ret0 as *const ()),      // 9 (+0x048): EndType(void*)
    Ptr(base_vfunc_ret0 as *const ()),      // 10 (+0x050): BeginEvent
    Ptr(base_vfunc_ret0 as *const ()),      // 11 (+0x058): EndEvent
    Ptr(base_vfunc_ret0 as *const ()),      // 12 (+0x060): TypeName
    Ptr(base_vfunc_ret0 as *const ()),      // 13 (+0x068)
    Ptr(binder_set_mode as *const ()),      // 14 (+0x070): SetMode
    Ptr(binder_get_mode as *const ()),      // 15 (+0x078): GetMode
    Ptr(base_vfunc_ret0 as *const ()),      // 16 (+0x080)
    Ptr(base_vfunc_ret0 as *const ()),      // 17 (+0x088)
    Ptr(base_vfunc_ret0 as *const ()),      // 18 (+0x090): PropertyName
    Ptr(base_vfunc_ret0 as *const ()),      // 19 (+0x098)
    Ptr(base_vfunc_ret0 as *const ()),      // 20 (+0x0a0): BindNull
    Ptr(base_vfunc_ret0 as *const ()),      // 21 (+0x0a8)
    Ptr(base_vfunc_ret0 as *const ()),      // 22 (+0x0b0)
    Ptr(base_vfunc_ret0 as *const ()),      // 23 (+0x0b8)
    Ptr(base_vfunc_ret0 as *const ()),      // 24 (+0x0c0): Bind(const char*)
    Ptr(base_vfunc_ret0 as *const ()),      // 25 (+0x0c8): Bind(double)
    Ptr(base_vfunc_ret0 as *const ()),      // 26 (+0x0d0): Bind(float)
    Ptr(base_vfunc_ret0 as *const ()),      // 27 (+0x0d8)
    Ptr(base_vfunc_ret0 as *const ()),      // 28 (+0x0e0)
    Ptr(base_vfunc_ret0 as *const ()),      // 29 (+0x0e8)
    Ptr(base_vfunc_ret0 as *const ()),      // 30 (+0x0f0)
    Ptr(base_vfunc_ret0 as *const ()),      // 31 (+0x0f8)
    Ptr(base_vfunc_ret0 as *const ()),      // 32 (+0x100): Bind(i32)
    Ptr(base_vfunc_ret0 as *const ()),      // 33 (+0x108): Bind(u32)
    Ptr(base_vfunc_ret0 as *const ()),      // 34 (+0x110)
    Ptr(base_vfunc_ret0 as *const ()),      // 35 (+0x118)
    Ptr(base_vfunc_ret0 as *const ()),      // 36 (+0x120)
    Ptr(base_vfunc_ret0 as *const ()),      // 37 (+0x128): Bind(bool)
    Ptr(base_vfunc_ret0 as *const ()),      // 38 (+0x130): BindArrayBegin
    Ptr(base_vfunc_ret0 as *const ()),      // 39 (+0x138): BindArrayEnd
    Ptr(base_vfunc_ret0 as *const ()),      // 40 (+0x140)
    Ptr(base_vfunc_ret0 as *const ()),      // 41 (+0x148)
    Ptr(base_vfunc_ret0 as *const ()),      // 42 (+0x150)
    Ptr(base_vfunc_ret0 as *const ()),      // 43 (+0x158)
    Ptr(base_vfunc_ret0 as *const ()),      // 44 (+0x160)
    Ptr(base_vfunc_ret0 as *const ()),      // 45 (+0x168)
    Ptr(base_vfunc_ret0 as *const ()),      // 46 (+0x170): ReadArrayBegin
    Ptr(base_vfunc_ret0 as *const ()),      // 47 (+0x178): ReadArrayElement
    Ptr(base_vfunc_ret0 as *const ()),      // 48 (+0x180): ReadArrayEnd
    Ptr(base_vfunc_ret0 as *const ()),      // 49 (+0x188)
    Ptr(base_vfunc_ret0 as *const ()),      // 50 (+0x190)
    Ptr(base_vfunc_ret0 as *const ()),      // 51 (+0x198)
    Ptr(base_vfunc_ret0 as *const ()),      // 52 (+0x1a0)
    Ptr(binder_read_zero as *const ()),     // 53 (+0x1a8): Read(float&) / Read(primitive&)
    Ptr(binder_read_zero as *const ()),     // 54 (+0x1b0): Read(double&) / Read(primitive&)
    Ptr(binder_read_string as *const ()),   // 55 (+0x1b8): Read(const char*&, size_t&)
    Ptr(binder_read_f64 as *const ()),      // 56 (+0x1c0): Read(double&)
    Ptr(base_vfunc_ret0 as *const ()),      // 57 (+0x1c8)
    Ptr(base_vfunc_ret0 as *const ()),      // 58 (+0x1d0)
    Ptr(base_vfunc_ret0 as *const ()),      // 59 (+0x1d8)
    Ptr(base_vfunc_ret0 as *const ()),      // 60 (+0x1e0)
    Ptr(base_vfunc_ret0 as *const ()),      // 61 (+0x1e8)
    Ptr(base_vfunc_ret0 as *const ()),      // 62 (+0x1f0)
    Ptr(base_vfunc_ret0 as *const ()),      // 63 (+0x1f8)
    Ptr(base_vfunc_ret0 as *const ()),      // 64 (+0x200)
    Ptr(base_vfunc_ret0 as *const ()),      // 65 (+0x208)
    Ptr(base_vfunc_ret0 as *const ()),      // 66 (+0x210)
    Ptr(base_vfunc_ret0 as *const ()),      // 67 (+0x218)
    Ptr(base_vfunc_ret0 as *const ()),      // 68 (+0x220)
    Ptr(binder_read_bool as *const ()),     // 69 (+0x228): Read(bool&)
    Ptr(base_vfunc_ret0 as *const ()),      // 70 (+0x230)
    Ptr(base_vfunc_ret0 as *const ()),      // 71 (+0x238)
    Ptr(base_vfunc_ret0 as *const ()),      // 72 (+0x240)
    Ptr(base_vfunc_ret0 as *const ()),      // 73 (+0x248)
    Ptr(base_vfunc_ret0 as *const ()),      // 74 (+0x250)
    Ptr(base_vfunc_ret0 as *const ()),      // 75 (+0x258)
    Ptr(base_vfunc_ret0 as *const ()),      // 76 (+0x260)
    Ptr(base_vfunc_ret0 as *const ()),      // 77 (+0x268)
    Ptr(base_vfunc_ret0 as *const ()),      // 78 (+0x270)
    Ptr(base_vfunc_ret0 as *const ()),      // 79 (+0x278)
];

// ---------------------------------------------------------------------------
// cohtml::Library vtable (16 slots, matches 0x180616858)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn library_dtor(this: *mut CohtmlLibrary, flags: u32) -> *mut CohtmlLibrary {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

unsafe extern "win64" fn library_create_system(
    _this: *mut CohtmlLibrary,
    _settings: *const c_void,
) -> *mut CohtmlSystem {
    ensure_vtables_registered();
    let sys = Box::new(CohtmlSystem {
        vtbl: &SYSTEM_VTBL,
        id: 1,
        _pad: [0u8; 0x864],
    });
    Box::into_raw(sys)
}

unsafe extern "win64" fn library_stop_workers(_this: *mut CohtmlLibrary) {}

unsafe extern "win64" fn library_uninitialize(_this: *mut CohtmlLibrary, _free_check: u8) {}

unsafe extern "win64" fn library_execute_work(
    _this: *mut CohtmlLibrary,
    _work_type: u32,
    _exec_type: u32,
    _family_id: u32,
) {
}

unsafe extern "win64" fn library_create_script_compiler(
    _this: *mut CohtmlLibrary,
) -> *const CohtmlDummyObject {
    ensure_vtables_registered();
    &DUMMY_SCRIPT_COMPILER
}

unsafe extern "win64" fn library_get_version() -> *const u8 {
    b"1.40.3.4\0".as_ptr()
}

static LIBRARY_VTBL: [Ptr; 16] = [
    Ptr(library_dtor as *const ()),                   // 0 (+0x00)
    Ptr(library_create_system as *const ()),          // 1 (+0x08): CreateSystem
    Ptr(library_stop_workers as *const ()),           // 2 (+0x10): StopWorkers
    Ptr(library_uninitialize as *const ()),           // 3 (+0x18): Uninitialize
    Ptr(base_vfunc_ret0 as *const ()),                // 4 (+0x20): ReserveWorkers
    Ptr(library_execute_work as *const ()),           // 5 (+0x28): ExecuteWork
    Ptr(base_vfunc_ret0 as *const ()),                // 6 (+0x30): ScheduleWork
    Ptr(library_create_script_compiler as *const ()), // 7 (+0x38): CreateScriptCompiler
    Ptr(base_vfunc_ret0 as *const ()),                // 8 (+0x40): GetFeatureInterface
    Ptr(base_vfunc_ret0 as *const ()),                // 9 (+0x48)
    Ptr(base_vfunc_ret0 as *const ()),                // 10 (+0x50)
    Ptr(base_vfunc_ret0 as *const ()),                // 11 (+0x58)
    Ptr(base_vfunc_ret0 as *const ()),                // 12 (+0x60)
    Ptr(base_vfunc_ret0 as *const ()),                // 13 (+0x68)
    Ptr(library_get_version as *const ()),            // 14 (+0x70)
    Ptr(library_get_version as *const ()),            // 15 (+0x78)
];

// ---------------------------------------------------------------------------
// cohtml::System vtable (30 slots, matches 0x180618650)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn system_get_id(this: *const CohtmlSystem) -> u32 {
    if this.is_null() {
        0
    } else {
        (*this).id
    }
}

unsafe extern "win64" fn system_dtor(this: *mut CohtmlSystem, flags: u32) -> *mut CohtmlSystem {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

unsafe extern "win64" fn system_create_view(
    _this: *mut CohtmlSystem,
    settings: *const u8,
) -> *mut CohtmlView {
    ensure_vtables_registered();
    let mut listener = std::ptr::null_mut();
    let mut width = 1920u32;
    let mut height = 1080u32;
    if !settings.is_null() {
        listener = std::ptr::read_unaligned(settings as *const *mut c_void);
        let w = std::ptr::read_unaligned(settings.add(0x10) as *const u32);
        let h = std::ptr::read_unaligned(settings.add(0x14) as *const u32);
        if w > 0 && w <= 16384 {
            width = w;
        }
        if h > 0 && h <= 16384 {
            height = h;
        }
    }
    let id = NEXT_VIEW_ID.fetch_add(1, Ordering::Relaxed);
    let view = Box::new(CohtmlView {
        vtbl: &VIEW_VTBL,
        listener,
        id,
        width,
        height,
        continuous_repaint: 0,
        ready_for_bindings: 1,
        _reserved_flags: [0; 2],
        binder: CohtmlBinder::new(),
        advance_count: 0,
        _pad1: 0,
        _reserved_handlers: [0; 4],
        _pad: [0u8; 0x1b0],
    });
    Box::into_raw(view)
}

unsafe extern "win64" fn system_create_system_renderer(
    _this: *mut CohtmlSystem,
    _settings: *const c_void,
) -> *mut CohtmlSystemRenderer {
    ensure_vtables_registered();
    let sr = Box::new(CohtmlSystemRenderer {
        vtbl: &SYSTEM_RENDERER_VTBL,
        backend: std::ptr::null_mut(),
        _pad: [0u64; 11],
    });
    Box::into_raw(sr)
}

unsafe extern "win64" fn system_destroy(this: *mut CohtmlSystem) {
    if !this.is_null() {
        let _ = Box::from_raw(this);
    }
}

unsafe extern "win64" fn system_advance(_this: *mut CohtmlSystem, _time_ms_slot: u64) {}

static SYSTEM_VTBL: [Ptr; 30] = [
    Ptr(system_get_id as *const ()),                 // 0 (+0x00)
    Ptr(base_vfunc_ret0 as *const ()),               // 1 (+0x08)
    Ptr(system_dtor as *const ()),                   // 2 (+0x10)
    Ptr(system_create_view as *const ()),            // 3 (+0x18): CreateView
    Ptr(system_create_system_renderer as *const ()), // 4 (+0x20): CreateSystemRenderer
    Ptr(base_vfunc_ret0 as *const ()),               // 5 (+0x28)
    Ptr(system_create_system_renderer as *const ()), // 6 (+0x30): CreateSystemRendererWithExternalRenderingLibrary
    Ptr(base_vfunc_ret0 as *const ()),               // 7 (+0x38)
    Ptr(base_vfunc_ret0 as *const ()),               // 8 (+0x40)
    Ptr(base_vfunc_ret0 as *const ()),               // 9 (+0x48)
    Ptr(base_vfunc_ret0 as *const ()),               // 10 (+0x50)
    Ptr(base_vfunc_ret0 as *const ()),               // 11 (+0x58)
    Ptr(base_vfunc_ret0 as *const ()),               // 12 (+0x60)
    Ptr(system_destroy as *const ()),                // 13 (+0x68): Destroy
    Ptr(system_advance as *const ()),                // 14 (+0x70): Advance
    Ptr(base_vfunc_ret0 as *const ()),               // 15 (+0x78)
    Ptr(base_vfunc_ret0 as *const ()),               // 16 (+0x80)
    Ptr(base_vfunc_ret0 as *const ()),               // 17 (+0x88)
    Ptr(base_vfunc_ret0 as *const ()),               // 18 (+0x90)
    Ptr(base_vfunc_ret0 as *const ()),               // 19 (+0x98)
    Ptr(base_vfunc_ret0 as *const ()),               // 20 (+0xa0)
    Ptr(system_create_system_renderer as *const ()), // 21 (+0xa8)
    Ptr(base_vfunc_ret0 as *const ()),               // 22 (+0xb0)
    Ptr(base_vfunc_ret0 as *const ()),               // 23 (+0xb8)
    Ptr(base_vfunc_ret0 as *const ()),               // 24 (+0xc0)
    Ptr(base_vfunc_ret0 as *const ()),               // 25 (+0xc8)
    Ptr(base_vfunc_ret0 as *const ()),               // 26 (+0xd0)
    Ptr(base_vfunc_ret0 as *const ()),               // 27 (+0xd8)
    Ptr(base_vfunc_ret0 as *const ()),               // 28 (+0xe0)
    Ptr(base_vfunc_ret0 as *const ()),               // 29 (+0xe8)
];

// ---------------------------------------------------------------------------
// cohtml::SystemRenderer vtable (10 slots)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn sysrend_dtor(
    this: *mut CohtmlSystemRenderer,
    flags: u32,
) -> *mut CohtmlSystemRenderer {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

unsafe extern "win64" fn sysrend_create_view_renderer(
    _this: *mut CohtmlSystemRenderer,
    view: *mut CohtmlView,
    _settings: *const c_void,
) -> *mut CohtmlViewRenderer {
    ensure_vtables_registered();
    let vr = Box::new(CohtmlViewRenderer {
        vtbl: &VIEW_RENDERER_VTBL,
        view,
        _pad: [0u64; 6],
    });
    Box::into_raw(vr)
}

unsafe extern "win64" fn sysrend_destroy(this: *mut CohtmlSystemRenderer) {
    if !this.is_null() {
        let _ = Box::from_raw(this);
    }
}

unsafe extern "win64" fn sysrend_register_render_thread(
    this: *mut CohtmlSystemRenderer,
    backend: *mut c_void,
) {
    if !this.is_null() {
        (*this).backend = backend;
    }
}

unsafe extern "win64" fn sysrend_free_rendering_resources(_this: *mut CohtmlSystemRenderer) {}

static SYSTEM_RENDERER_VTBL: [Ptr; 10] = [
    Ptr(sysrend_dtor as *const ()),                     // 0 (+0x00)
    Ptr(sysrend_create_view_renderer as *const ()),     // 1 (+0x08): CreateViewRenderer
    Ptr(sysrend_destroy as *const ()),                  // 2 (+0x10): Destroy
    Ptr(sysrend_register_render_thread as *const ()),   // 3 (+0x18): RegisterRenderThread
    Ptr(sysrend_free_rendering_resources as *const ()), // 4 (+0x20): FreeRenderingResources
    Ptr(base_vfunc_ret0 as *const ()),                  // 5 (+0x28)
    Ptr(base_vfunc_ret0 as *const ()),                  // 6 (+0x30)
    Ptr(base_vfunc_ret0 as *const ()),                  // 7 (+0x38)
    Ptr(base_vfunc_ret0 as *const ()),                  // 8 (+0x40)
    Ptr(base_vfunc_ret0 as *const ()),                  // 9 (+0x48)
];

// ---------------------------------------------------------------------------
// cohtml::ViewRenderer vtable (12 slots)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn viewrend_dtor(
    this: *mut CohtmlViewRenderer,
    flags: u32,
) -> *mut CohtmlViewRenderer {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

unsafe extern "win64" fn viewrend_destroy(this: *mut CohtmlViewRenderer) {
    if !this.is_null() {
        let _ = Box::from_raw(this);
    }
}

unsafe extern "win64" fn viewrend_paint(
    _this: *mut CohtmlViewRenderer,
    _frame_id: u32,
    _clear: u8,
) {
}

unsafe extern "win64" fn viewrend_set_render_target(
    _this: *mut CohtmlViewRenderer,
    _tex: *mut c_void,
    _depth: *mut c_void,
    _width: u32,
    _height: u32,
    _samples: u32,
) {
}

unsafe extern "win64" fn viewrend_free_resources(_this: *mut CohtmlViewRenderer) {}

static VIEW_RENDERER_VTBL: [Ptr; 12] = [
    Ptr(viewrend_dtor as *const ()),              // 0 (+0x00)
    Ptr(viewrend_destroy as *const ()),           // 1 (+0x08): Destroy
    Ptr(viewrend_paint as *const ()),             // 2 (+0x10): Paint
    Ptr(viewrend_set_render_target as *const ()), // 3 (+0x18): SetRenderTarget
    Ptr(viewrend_destroy as *const ()),           // 4 (+0x20)
    Ptr(viewrend_free_resources as *const ()),    // 5 (+0x28): FreeRenderingResources
    Ptr(base_vfunc_ret0 as *const ()),            // 6 (+0x30)
    Ptr(base_vfunc_ret0 as *const ()),            // 7 (+0x38)
    Ptr(base_vfunc_ret0 as *const ()),            // 8 (+0x40)
    Ptr(base_vfunc_ret0 as *const ()),            // 9 (+0x48)
    Ptr(base_vfunc_ret0 as *const ()),            // 10 (+0x50)
    Ptr(base_vfunc_ret0 as *const ()),            // 11 (+0x58)
];

// ---------------------------------------------------------------------------
// cohtml::View vtable (80 slots, matches 0x180619f08)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn view_dtor(this: *mut CohtmlView, flags: u32) -> *mut CohtmlView {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

unsafe extern "win64" fn view_destroy(this: *mut CohtmlView) {
    if !this.is_null() {
        let _ = Box::from_raw(this);
    }
}

unsafe extern "win64" fn view_load_url(this: *mut CohtmlView, _url: *const u8) {
    if !this.is_null() {
        (*this).ready_for_bindings = 1;
    }
}

unsafe extern "win64" fn view_reload(_this: *mut CohtmlView) {}

unsafe extern "win64" fn view_get_task_scheduler(
    _this: *mut CohtmlView,
) -> *const CohtmlDummyObject {
    ensure_vtables_registered();
    &DUMMY_TASK_SCHEDULER
}

/// Optional per-game UI bridge hook for Coherent Gameface views.
///
/// Kept `None` by default so generic Cohtml games never touch foreign VAs or inject
/// game-specific UI events. Game targets (e.g. Alan Wake 2) register their own hook
/// via [`set_view_hook`].
pub trait CohtmlViewHook: Send + Sync {
    unsafe fn on_first_advance(&self, _view_id: u32) {}
    unsafe fn on_advance(&self, _view_id: u32, _advance_count: u32) {}
    unsafe fn on_register_event(&self, _view_id: u32, _event_name: &str, _handler: usize) {}
    unsafe fn on_trigger_event(&self, _view_id: u32, _advance_count: u32, _event_name: Option<&str>) {}
}

static VIEW_HOOK: std::sync::RwLock<Option<Box<dyn CohtmlViewHook>>> = std::sync::RwLock::new(None);

/// Install a game-specific [`CohtmlViewHook`] for builtin `cohtml.WindowsDesktop.dll`.
pub fn set_view_hook(hook: Box<dyn CohtmlViewHook>) {
    *VIEW_HOOK.write().unwrap() = Some(hook);
}

/// Invoke a zero-argument Cohtml guest event handler (`IEventHandler::Invoke`).
pub unsafe fn fire_event_0(handler: usize) {
    if handler == 0 {
        return;
    }
    let vtbl = std::ptr::read_unaligned(handler as *const usize);
    if vtbl == 0 {
        return;
    }
    let vfunc_1 = std::ptr::read_unaligned((vtbl + 8) as *const usize);
    if vfunc_1 == 0 {
        return;
    }
    let mut args_binder = CohtmlArgsBinder {
        vtbl: &ARGS_BINDER_VTBL,
        arg_count: 0,
        _pad: 0,
        args: [CohtmlBinder::new(); 5],
    };
    let _ = crate::call_guest8(
        vfunc_1,
        handler as u64,
        (&raw mut args_binder) as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    );
}

unsafe extern "win64" fn view_advance(this: *mut CohtmlView, _time_ms_slot: u64) -> u32 {
    if !this.is_null() {
        (*this).advance_count = (*this).advance_count.saturating_add(1);
        let view_id = (*this).id;
        let adv = (*this).advance_count;
        if adv == 1 && !(*this).listener.is_null() {
            if let Ok(guard) = VIEW_HOOK.read() {
                if let Some(h) = guard.as_ref() {
                    h.on_first_advance(view_id);
                }
            }
            let listener = (*this).listener as usize;
            let vtbl = std::ptr::read_unaligned(listener as *const usize);
            if vtbl != 0 {
                // Slot 7 (+0x38): cohtml::IViewListener::OnReadyForBindings
                let on_ready = std::ptr::read_unaligned((vtbl + 0x38) as *const usize);
                if on_ready != 0 {
                    let _ = crate::call_guest8(on_ready, listener as u64, 0, 0, 0, 0, 0, 0, 0);
                }
                // Slot 6 (+0x30): cohtml::IViewListener::OnFinishLoad
                let on_finish = std::ptr::read_unaligned((vtbl + 0x30) as *const usize);
                if on_finish != 0 {
                    let _ = crate::call_guest8(on_finish, listener as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        if let Ok(guard) = VIEW_HOOK.read() {
            if let Some(h) = guard.as_ref() {
                h.on_advance(view_id, adv);
            }
        }
    }
    NEXT_FRAME_ID.fetch_add(1, Ordering::Relaxed)
}

unsafe extern "win64" fn view_get_id(this: *const CohtmlView) -> u32 {
    if this.is_null() {
        1
    } else {
        (*this).id
    }
}

unsafe extern "win64" fn view_resize(this: *mut CohtmlView, width: u32, height: u32) {
    if !this.is_null() {
        (*this).width = width;
        (*this).height = height;
    }
}

unsafe extern "win64" fn view_continuous_repaint(this: *mut CohtmlView, enable: u8) {
    if !this.is_null() {
        (*this).continuous_repaint = enable;
    }
}

unsafe extern "win64" fn view_get_width(this: *const CohtmlView) -> u32 {
    if this.is_null() {
        1920
    } else {
        (*this).width
    }
}

unsafe extern "win64" fn view_get_height(this: *const CohtmlView) -> u32 {
    if this.is_null() {
        1080
    } else {
        (*this).height
    }
}

unsafe extern "win64" fn view_get_scale() -> f32 {
    1.0
}

unsafe extern "win64" fn view_register_for_event(
    this: *mut CohtmlView,
    name: *const u8,
    handler: *mut c_void,
) -> u64 {
    if !this.is_null() && !name.is_null() && !handler.is_null() {
        if let Ok(event_name) = CStr::from_ptr(name as *const i8).to_str() {
            if let Ok(guard) = VIEW_HOOK.read() {
                if let Some(h) = guard.as_ref() {
                    h.on_register_event((*this).id, event_name, handler as usize);
                }
            }
        }
    }
    1
}

unsafe extern "win64" fn view_trigger_event(this: *mut CohtmlView, name: *const u8) -> usize {
    if !this.is_null() {
        let event_name = if name.is_null() {
            None
        } else {
            CStr::from_ptr(name as *const i8).to_str().ok()
        };
        if let Ok(guard) = VIEW_HOOK.read() {
            if let Some(h) = guard.as_ref() {
                h.on_trigger_event((*this).id, (*this).advance_count, event_name);
            }
        }
    }
    0
}

unsafe extern "win64" fn view_bind_call(
    this: *mut CohtmlView,
    name: *const u8,
    handler: *mut c_void,
) -> u64 {
    view_register_for_event(this, name, handler)
}

unsafe extern "win64" fn view_is_ready_for_bindings(this: *const CohtmlView) -> u8 {
    if this.is_null() {
        1
    } else {
        (*this).ready_for_bindings
    }
}

unsafe extern "win64" fn view_get_binder(this: *mut CohtmlView) -> *mut CohtmlBinder {
    ensure_vtables_registered();
    if this.is_null() {
        &raw mut FALLBACK_BINDER
    } else {
        &raw mut (*this).binder
    }
}

static VIEW_VTBL: [Ptr; 80] = [
    Ptr(view_dtor as *const ()),                  // 0 (+0x000)
    Ptr(view_destroy as *const ()),               // 1 (+0x008): Destroy
    Ptr(view_load_url as *const ()),              // 2 (+0x010): LoadURL
    Ptr(view_reload as *const ()),                // 3 (+0x018): Reload
    Ptr(base_vfunc_ret0 as *const ()),            // 4 (+0x020): UnloadDocument
    Ptr(view_get_task_scheduler as *const ()),    // 5 (+0x028): GetTaskScheduler
    Ptr(view_advance as *const ()),               // 6 (+0x030): Advance
    Ptr(view_get_id as *const ()),                // 7 (+0x038): GetId
    Ptr(base_vfunc_ret0 as *const ()),            // 8 (+0x040)
    Ptr(view_resize as *const ()),                // 9 (+0x048): Resize
    Ptr(base_vfunc_ret0 as *const ()),            // 10 (+0x050)
    Ptr(base_vfunc_ret0 as *const ()),            // 11 (+0x058)
    Ptr(view_continuous_repaint as *const ()),    // 12 (+0x060): ContinuousRepaint
    Ptr(view_get_width as *const ()),             // 13 (+0x068): GetWidth
    Ptr(view_get_height as *const ()),            // 14 (+0x070): GetHeight
    Ptr(base_vfunc_ret0 as *const ()),            // 15 (+0x078)
    Ptr(base_vfunc_ret0 as *const ()),            // 16 (+0x080)
    Ptr(base_vfunc_ret0 as *const ()),            // 17 (+0x088)
    Ptr(base_vfunc_ret0 as *const ()),            // 18 (+0x090)
    Ptr(base_vfunc_ret0 as *const ()),            // 19 (+0x098)
    Ptr(base_vfunc_ret0 as *const ()),            // 20 (+0x0a0)
    Ptr(base_vfunc_ret0 as *const ()),            // 21 (+0x0a8)
    Ptr(base_vfunc_ret0 as *const ()),            // 22 (+0x0b0)
    Ptr(base_vfunc_ret0 as *const ()),            // 23 (+0x0b8)
    Ptr(base_vfunc_ret0 as *const ()),            // 24 (+0x0c0)
    Ptr(base_vfunc_ret0 as *const ()),            // 25 (+0x0c8): MouseEvent
    Ptr(base_vfunc_ret0 as *const ()),            // 26 (+0x0d0): KeyEvent
    Ptr(base_vfunc_ret0 as *const ()),            // 27 (+0x0d8): TouchEvent
    Ptr(base_vfunc_ret0 as *const ()),            // 28 (+0x0e0)
    Ptr(base_vfunc_ret0 as *const ()),            // 29 (+0x0e8)
    Ptr(base_vfunc_ret0 as *const ()),            // 30 (+0x0f0)
    Ptr(base_vfunc_ret0 as *const ()),            // 31 (+0x0f8)
    Ptr(base_vfunc_ret0 as *const ()),            // 32 (+0x100)
    Ptr(base_vfunc_ret0 as *const ()),            // 33 (+0x108)
    Ptr(view_get_scale as *const ()),             // 34 (+0x110)
    Ptr(base_vfunc_ret0 as *const ()),            // 35 (+0x118)
    Ptr(base_vfunc_ret0 as *const ()),            // 36 (+0x120)
    Ptr(base_vfunc_ret0 as *const ()),            // 37 (+0x128)
    Ptr(base_vfunc_ret0 as *const ()),            // 38 (+0x130)
    Ptr(base_vfunc_ret0 as *const ()),            // 39 (+0x138)
    Ptr(base_vfunc_ret0 as *const ()),            // 40 (+0x140)
    Ptr(base_vfunc_ret0 as *const ()),            // 41 (+0x148)
    Ptr(base_vfunc_ret0 as *const ()),            // 42 (+0x150)
    Ptr(base_vfunc_ret0 as *const ()),            // 43 (+0x158)
    Ptr(base_vfunc_ret0 as *const ()),            // 44 (+0x160)
    Ptr(base_vfunc_ret0 as *const ()),            // 45 (+0x168)
    Ptr(base_vfunc_ret0 as *const ()),            // 46 (+0x170)
    Ptr(base_vfunc_ret0 as *const ()),            // 47 (+0x178)
    Ptr(base_vfunc_ret0 as *const ()),            // 48 (+0x180)
    Ptr(base_vfunc_ret0 as *const ()),            // 49 (+0x188)
    Ptr(base_vfunc_ret0 as *const ()),            // 50 (+0x190)
    Ptr(base_vfunc_ret0 as *const ()),            // 51 (+0x198)
    Ptr(base_vfunc_ret0 as *const ()),            // 52 (+0x1a0)
    Ptr(base_vfunc_ret0 as *const ()),            // 53 (+0x1a8)
    Ptr(view_register_for_event as *const ()),    // 54 (+0x1b0): RegisterForEvent (used by 0x1425036b0)
    Ptr(base_vfunc_ret0 as *const ()),            // 55 (+0x1b8)
    Ptr(view_bind_call as *const ()),             // 56 (+0x1c0): BindCall (used by 0x1425036b0)
    Ptr(base_vfunc_ret0 as *const ()),            // 57 (+0x1c8)
    Ptr(base_vfunc_ret0 as *const ()),            // 58 (+0x1d0): UnregisterFromEvent
    Ptr(base_vfunc_ret0 as *const ()),            // 59 (+0x1d8)
    Ptr(view_is_ready_for_bindings as *const ()), // 60 (+0x1e0): IsReadyForBindings
    Ptr(base_vfunc_ret0 as *const ()),            // 61 (+0x1e8)
    Ptr(base_vfunc_ret0 as *const ()),            // 62 (+0x1f0)
    Ptr(base_vfunc_ret0 as *const ()),            // 63 (+0x1f8)
    Ptr(base_vfunc_ret0 as *const ()),            // 64 (+0x200)
    Ptr(base_vfunc_ret0 as *const ()),            // 65 (+0x208): SetUserBackgroundObject
    Ptr(base_vfunc_ret0 as *const ()),            // 66 (+0x210)
    Ptr(view_get_binder as *const ()),            // 67 (+0x218): GetBinder
    Ptr(base_vfunc_ret0 as *const ()),            // 68 (+0x220): SynchronizeModels
    Ptr(base_vfunc_ret0 as *const ()),            // 69 (+0x228)
    Ptr(base_vfunc_ret0 as *const ()),            // 70 (+0x230)
    Ptr(base_vfunc_ret0 as *const ()),            // 71 (+0x238)
    Ptr(base_vfunc_ret0 as *const ()),            // 72 (+0x240): CreateModel
    Ptr(base_vfunc_ret0 as *const ()),            // 73 (+0x248)
    Ptr(view_trigger_event as *const ()),         // 74 (+0x250): TriggerEvent
    Ptr(base_vfunc_ret0 as *const ()),            // 75 (+0x258)
    Ptr(base_vfunc_ret0 as *const ()),            // 76 (+0x260)
    Ptr(base_vfunc_ret0 as *const ()),            // 77 (+0x268)
    Ptr(base_vfunc_ret0 as *const ()),            // 78 (+0x270)
    Ptr(base_vfunc_ret0 as *const ()),            // 79 (+0x278)
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in LIBRARY_VTBL
            .iter()
            .chain(SYSTEM_VTBL.iter())
            .chain(SYSTEM_RENDERER_VTBL.iter())
            .chain(VIEW_RENDERER_VTBL.iter())
            .chain(VIEW_VTBL.iter())
            .chain(BINDER_VTBL.iter())
            .chain(ARGS_BINDER_VTBL.iter())
            .chain(BASE_OBJECT_VTBL.iter())
        {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Exported C++ methods & functions (all 69 symbols)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn cohtml_noop() {}

unsafe extern "win64" fn cohtml_ret0() -> usize {
    0
}

unsafe extern "win64" fn cohtml_ret_true() -> u8 {
    1
}

unsafe extern "win64" fn cohtml_ret_neg1() -> i32 {
    -1
}

unsafe extern "win64" fn cohtml_base_ctor(this: *mut *const [Ptr; 16]) -> *mut *const [Ptr; 16] {
    ensure_vtables_registered();
    if !this.is_null() {
        *this = &BASE_OBJECT_VTBL;
    }
    this
}

unsafe extern "win64" fn cohtml_base_dtor(this: *mut *const [Ptr; 16]) {
    if !this.is_null() {
        *this = &BASE_OBJECT_VTBL;
    }
}

unsafe extern "win64" fn on_screen_info_requested(
    _this: *mut c_void,
    sret: *mut ScreenInfo,
) -> *mut ScreenInfo {
    if !sret.is_null() {
        *sret = ScreenInfo {
            available_width: 1920,
            available_height: 1080,
            width: 1920,
            height: 1080,
            color_depth: 24,
            pixel_depth: 24,
        };
    }
    sret
}

unsafe extern "win64" fn cohtml_operator_new(size: usize) -> *mut c_void {
    let alloc_sz = size.max(16);
    crate::ucrt::heap::calloc(1, alloc_sz)
}

unsafe extern "win64" fn cohtml_operator_delete(ptr: *mut c_void) {
    if !ptr.is_null() {
        crate::ucrt::heap::free(ptr);
    }
}

unsafe fn write_to_string_buf(s: &[u8], buf: *mut u8, inout_len: *mut usize) -> u8 {
    if inout_len.is_null() {
        return 0;
    }
    let cap = *inout_len;
    if !buf.is_null() && cap > 0 {
        let copy_len = s.len().min(cap.saturating_sub(1));
        std::ptr::copy_nonoverlapping(s.as_ptr(), buf, copy_len);
        *buf.add(copy_len) = 0;
    }
    *inout_len = s.len();
    1
}

unsafe extern "win64" fn coherent_to_string_f64(
    val: f64,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    let s = format!("{val}");
    write_to_string_buf(s.as_bytes(), buf, inout_len)
}
crate::win64_bridge!(fn coherent_to_string_f64(f64, *mut u8, *mut usize) -> u8);

unsafe extern "win64" fn coherent_to_string_f32(
    val: f32,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    let s = format!("{val}");
    write_to_string_buf(s.as_bytes(), buf, inout_len)
}
crate::win64_bridge!(fn coherent_to_string_f32(f32, *mut u8, *mut usize) -> u8);

unsafe extern "win64" fn coherent_to_string_i32(
    val: i32,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    let s = format!("{val}");
    write_to_string_buf(s.as_bytes(), buf, inout_len)
}

unsafe extern "win64" fn coherent_to_string_u32(
    val: u32,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    let s = format!("{val}");
    write_to_string_buf(s.as_bytes(), buf, inout_len)
}

unsafe extern "win64" fn coherent_to_string_bool(
    val: u8,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    let s: &[u8] = if val != 0 { b"true" } else { b"false" };
    write_to_string_buf(s, buf, inout_len)
}

unsafe extern "win64" fn coherent_to_string_cstr(
    val: *const u8,
    buf: *mut u8,
    inout_len: *mut usize,
) -> u8 {
    if inout_len.is_null() {
        return 0;
    }
    if val.is_null() {
        *inout_len = 0;
        return 1;
    }
    let bytes = CStr::from_ptr(val as *const libc::c_char).to_bytes();
    if !buf.is_null() {
        let cap = *inout_len;
        let copy_len = bytes.len().min(cap);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy_len);
    }
    *inout_len = bytes.len();
    1
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

unsafe extern "win64" fn library_decode_url_string(
    url: *const u8,
    len: u32,
    out: *mut u8,
    out_len: *mut u32,
) {
    let slice = if url.is_null() || len == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(url, len as usize)
    };
    let mut decoded = Vec::with_capacity(slice.len() + 1);
    let mut i = 0;
    while i < slice.len() {
        if slice[i] == b'%' && i + 2 < slice.len() {
            if let (Some(hi), Some(lo)) = (hex_nibble(slice[i + 1]), hex_nibble(slice[i + 2])) {
                decoded.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        if slice[i] == 0 {
            break;
        }
        decoded.push(slice[i]);
        i += 1;
    }
    decoded.push(0);
    if !out_len.is_null() {
        *out_len = decoded.len() as u32;
    }
    if !out.is_null() {
        std::ptr::copy_nonoverlapping(decoded.as_ptr(), out, decoded.len());
    }
}

unsafe extern "win64" fn library_initialize(
    _license_key: *const u8,
    _params: *const c_void,
) -> *mut CohtmlLibrary {
    ensure_vtables_registered();
    let lib = Box::new(CohtmlLibrary {
        vtbl: &LIBRARY_VTBL,
        _pad: [0u64; 11],
    });
    Box::into_raw(lib)
}

pub static EXTRA_EXPORTS: &[Export] = &[
    Export {
        name: "?ComputeTextDirection@IInternationalizationManager@cohtml@@UEBA?AW4TextDirection@i18n@2@PEBDIPEAUTextDirectionResult@12@@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "?OnBindingsReleased@IViewListener@cohtml@@UEAAXXZ",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnScreenInfoRequested@IViewListener@cohtml@@UEAA?AUScreenInfo@2@XZ",
        ptr: Ptr(on_screen_info_requested as *const ()),
    },
    Export {
        name: "?OnNodeTouched@IViewListener@cohtml@@UEAA?AW4Actions@EventAction@2@PEBVINodeProxy@2@PEBUTouchEventData@2@PEAXW4PhaseType@InputEventPhase@2@@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "?OnNodeKeyEvent@IViewListener@cohtml@@UEAA?AW4Actions@EventAction@2@PEBVINodeProxy@2@PEBUKeyEventData@2@PEAXW4PhaseType@InputEventPhase@2@@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "?OnAudioStreamCreated@IViewListener@cohtml@@UEAAXHHHM@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioStreamClosed@IViewListener@cohtml@@UEAAXH@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioStreamPlay@IViewListener@cohtml@@UEAAXH@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioStreamPause@IViewListener@cohtml@@UEAAXH@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioDataReceived@IViewListener@cohtml@@UEAAXHHPEAPEAMH@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioStreamEnded@IViewListener@cohtml@@UEAAXH@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnAudioStreamVolumeChanged@IViewListener@cohtml@@UEAAXHM@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnTextInputTypeChanged@IViewListener@cohtml@@UEAAXW4ControlType@TextInputControlType@2@@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnLoadFailed@IViewListener@cohtml@@UEAAXPEBD0@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnDOMBuilt@IViewListener@cohtml@@UEAAXXZ",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?GetCurrentPosition@IBreakIterator@i18n@cohtml@@UEAAHXZ",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "?MoveToCandidateAfter@IBreakIterator@i18n@cohtml@@UEAAHH@Z",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "?ComputeBaseDirection@IInternationalizationManager@cohtml@@UEBA?AW4TextDirection@i18n@2@PEBDI@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "??1ITextTransformationManager@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@N@cohtml@@SA_NNPEADPEA_K@Z",
        ptr: Ptr(crate::bridge_entry!(coherent_to_string_f64)),
    },
    Export {
        name: "?OnCaretRectChanged@IViewListener@cohtml@@UEAAXHHII@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnCursorChanged@IViewListener@cohtml@@UEAAXW4Cursors@CursorTypes@2@PEBDPEBM2@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnScriptContextCreated@IViewListener@cohtml@@UEAAXXZ",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnResourceRequestIntercepted@IViewListener@cohtml@@UEAA?AW4InterceptRequestStatus@2@PEBD@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "?OnClipboardTextSet@IViewListener@cohtml@@UEAAXPEBDI@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnClipboardTextGet@IViewListener@cohtml@@UEAAXPEAUIClipboardData@12@@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "?OnCreateWebSocket@IViewListener@cohtml@@UEAAPEAVIClientSideSocket@2@PEAVISocketListener@2@PEBDPEAPEBDI@Z",
        ptr: Ptr(cohtml_ret0 as *const ()),
    },
    Export {
        name: "?OnNavigateTo@IViewListener@cohtml@@UEAA_NPEBD@Z",
        ptr: Ptr(cohtml_ret_true as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@I@cohtml@@SA_NIPEADPEA_K@Z",
        ptr: Ptr(coherent_to_string_u32 as *const ()),
    },
    Export {
        name: "??0IFileSystemReader@cohtml@@IEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "?MoveToNext@IBreakIterator@i18n@cohtml@@UEAAHXZ",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "?Translate@ILocalizationManager@cohtml@@UEAAXPEBDPEAUTranslationData@12@@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "??1IFileSystemReader@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??0IViewListener@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1IViewListener@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??0IAsyncResourceHandler@cohtml@@IEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1IAsyncResourceHandler@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??0IPerformanceHandler@Profile@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1IPerformanceHandler@Profile@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??1ISyncStreamReader@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "?Initialize@Library@cohtml@@SAPEAV12@PEBDAEBULibraryParams@2@@Z",
        ptr: Ptr(library_initialize as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@_N@cohtml@@SA_N_NPEADPEA_K@Z",
        ptr: Ptr(coherent_to_string_bool as *const ()),
    },
    Export {
        name: "?HintThreadUsage@Library@cohtml@@SAXW4WorkType@2@@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "??2IEventHandler@cohtml@@SAPEAX_K@Z",
        ptr: Ptr(cohtml_operator_new as *const ()),
    },
    Export {
        name: "??0ILocalizationManager@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "?MoveToPrevious@IBreakIterator@i18n@cohtml@@UEAAHXZ",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "??0ISyncStreamReader@cohtml@@IEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "?ResetToBeginning@IBreakIterator@i18n@cohtml@@UEAAHXZ",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "?DecodeURLString@Library@cohtml@@SAXPEBDIPEADPEAI@Z",
        ptr: Ptr(library_decode_url_string as *const ()),
    },
    Export {
        name: "??1IAllocator@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??0IAllocator@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1IBreakIterator@i18n@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??3IEventHandler@cohtml@@SAXPEAX@Z",
        ptr: Ptr(cohtml_operator_delete as *const ()),
    },
    Export {
        name: "??1TypeDescription@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "??0IBreakIterator@i18n@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1IInternationalizationManager@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "?ClearText@IBreakIterator@i18n@cohtml@@UEAAXXZ",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "??0ILogHandler@Logging@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??0IInternationalizationManager@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??3Property@cohtml@@SAXPEAX@Z",
        ptr: Ptr(cohtml_operator_delete as *const ()),
    },
    Export {
        name: "?ResetToEnd@IBreakIterator@i18n@cohtml@@UEAAHXZ",
        ptr: Ptr(cohtml_ret_neg1 as *const ()),
    },
    Export {
        name: "?SetText@IBreakIterator@i18n@cohtml@@UEAAXPEBDI@Z",
        ptr: Ptr(cohtml_noop as *const ()),
    },
    Export {
        name: "??2Property@cohtml@@SAPEAX_K@Z",
        ptr: Ptr(cohtml_operator_new as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@PEBD@cohtml@@SA_NPEBDPEADPEA_K@Z",
        ptr: Ptr(coherent_to_string_cstr as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@M@cohtml@@SA_NMPEADPEA_K@Z",
        ptr: Ptr(crate::bridge_entry!(coherent_to_string_f32)),
    },
    Export {
        name: "??1ILogHandler@Logging@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "??0ITextTransformationManager@cohtml@@QEAA@XZ",
        ptr: Ptr(cohtml_base_ctor as *const ()),
    },
    Export {
        name: "??1ILocalizationManager@cohtml@@UEAA@XZ",
        ptr: Ptr(cohtml_base_dtor as *const ()),
    },
    Export {
        name: "?Invoke@?$CoherentToString@H@cohtml@@SA_NHPEADPEA_K@Z",
        ptr: Ptr(coherent_to_string_i32 as *const ()),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cohtml_exports_and_lifecycle() {
        assert_eq!(EXTRA_EXPORTS.len(), 69);
        unsafe {
            let lib = library_initialize(b"key\0".as_ptr(), std::ptr::null());
            assert!(!lib.is_null());
            let sys = library_create_system(lib, std::ptr::null());
            assert!(!sys.is_null());
            let view = system_create_view(sys, std::ptr::null());
            assert!(!view.is_null());
            assert_eq!(view_get_width(view), 1920);
            assert_eq!(view_get_height(view), 1080);
            assert!(view_get_id(view) >= 1);
            assert_eq!(view_is_ready_for_bindings(view), 1);

            let compiler = library_create_script_compiler(lib);
            assert!(!compiler.is_null());
            assert_eq!((*compiler).vtbl, &BASE_OBJECT_VTBL as *const _);

            let scheduler = view_get_task_scheduler(view);
            assert!(!scheduler.is_null());
            assert_eq!((*scheduler).vtbl, &BASE_OBJECT_VTBL as *const _);

            let binder = view_get_binder(view);
            assert!(!binder.is_null());
            assert_eq!((*binder).vtbl, &BINDER_VTBL as *const _);

            // Verify Binder::RegisterType (slot 2) zeroes the sret TypeDescription.
            let mut type_desc = [usize::MAX; 2];
            let ret_desc = binder_register_type(
                binder,
                &mut type_desc,
                b"TestType\0".as_ptr(),
                std::ptr::null_mut(),
            );
            assert_eq!(ret_desc, &mut type_desc as *mut _);
            assert_eq!(type_desc, [0, 0]);

            // Verify Binder::Read string (slot 55) returns a non-null empty C string.
            let mut str_ptr: *const u8 = std::ptr::null();
            let mut str_len: usize = 999;
            binder_read_string(binder, &mut str_ptr, &mut str_len);
            assert!(!str_ptr.is_null());
            assert_eq!(*str_ptr, 0);
            assert_eq!(str_len, 0);

            binder_set_mode(binder, 2);
            assert_eq!(binder_get_mode(binder), 2);

            let mut out = [0u8; 32];
            let mut out_len = 0u32;
            let url = b"coui://uiresources/hello%20world";
            library_decode_url_string(url.as_ptr(), url.len() as u32, out.as_mut_ptr(), &mut out_len);
            assert_eq!(out_len, 31);
            assert_eq!(&out[..30], b"coui://uiresources/hello world");

            view_destroy(view);
            system_destroy(sys);
            library_dtor(lib, 1);
        }
    }
}
