//! Minimal COM object model for the DXGI / D3D12 implementation.
//!
//! * [`Guid`] with a `const` parser (`guid("...")`) and Windows formatting.
//! * Vtables live in guest-visible memory: every slot points at a 16-byte
//!   jump thunk in an executable arena (registered with our VirtualQuery /
//!   VirtualProtect), never directly at host `__TEXT`. Code that patches
//!   vtable slots or the first bytes of a method (Streamline's
//!   `registerHookForClassInstance` / `restoreOriginalCode`) therefore only
//!   ever touches guest-style pages.
//! * Methods we do not implement get a *trap* thunk that logs
//!   `UNIMPLEMENTED IFoo::Method`, prints a guest backtrace and exits, so
//!   missing pieces are obvious instead of silently returning garbage.
//! * [`Obj<T>`] is the common object header: vtable pointer, refcount,
//!   private data (`SetPrivateData` / `SetName`), then the Rust state `T`.
//!   One vtable per class: every DXGI/D3D12 interface family we serve is a
//!   single-inheritance chain (`IDXGIFactory`..`7`, `ID3D12Device`..`N`), so
//!   `QueryInterface` for any IID in the chain returns the same pointer.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// GUID (re-exported from canonical `crate::com`)
// ---------------------------------------------------------------------------

pub use crate::com::{
    guid, iid_name, name_iid, Guid, PrivateData, DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_FOUND,
    IID_IUnknown, WKPDID_D3DDebugObjectName, WKPDID_D3DDebugObjectNameW,
};

pub const DXGI_ERROR_UNSUPPORTED: HRESULT = 0x887A_0004u32 as i32;
pub const DXGI_ERROR_INVALID_CALL: HRESULT = 0x887A_0001u32 as i32;
pub const DXGI_ERROR_WAS_STILL_DRAWING: HRESULT = 0x887A_000Au32 as i32;

// ---------------------------------------------------------------------------
// Executable thunk arena
// ---------------------------------------------------------------------------

struct Arena {
    cur: usize,
    end: usize,
}

#[cfg(target_arch = "x86_64")]
static CODE: Mutex<Arena> = Mutex::new(Arena { cur: 0, end: 0 });
static DATA: Mutex<Arena> = Mutex::new(Arena { cur: 0, end: 0 });
const CHUNK: usize = 0x10000;

fn arena_alloc(a: &Mutex<Arena>, len: usize, exec: bool) -> usize {
    let len = (len + 15) & !15;
    let mut a = a.lock().unwrap();
    if a.cur == 0 || a.cur + len > a.end {
        let size = CHUNK.max((len + 0xFFFF) & !0xFFFF);
        let prot = libc::PROT_READ | libc::PROT_WRITE | if exec { libc::PROT_EXEC } else { 0 };
        let p = unsafe { libc::mmap(std::ptr::null_mut(), size, prot, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0) };
        assert!(p != libc::MAP_FAILED, "COM arena mmap failed");
        let p = p as usize;
        use crate::kernel32::memory::{register_image, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};
        register_image(p, size, &[(p, p + size, if exec { PAGE_EXECUTE_READWRITE } else { PAGE_READWRITE })]);
        a.cur = p;
        a.end = p + size;
    }
    let p = a.cur;
    a.cur += len;
    p
}

/// Map thunk address -> "IFoo::Method" (symbolization / traps).
static THUNK_NAMES: Mutex<Option<HashMap<usize, String>>> = Mutex::new(None);

pub fn thunk_name(addr: usize) -> Option<String> {
    let g = THUNK_NAMES.lock().unwrap();
    let map = g.as_ref()?;
    map.get(&addr).or_else(|| map.get(&(addr & !15))).cloned()
}

#[cfg(target_arch = "x86_64")]
fn jump_thunk(target: usize) -> usize {
    let p = arena_alloc(&CODE, 16, true);
    let mut c = [0xCCu8; 16];
    c[0] = 0xFF;
    c[1] = 0x25; // jmp [rip+0]
    c[2..6].copy_from_slice(&[0, 0, 0, 0]);
    c[6..14].copy_from_slice(&(target as u64).to_le_bytes());
    unsafe { std::ptr::copy_nonoverlapping(c.as_ptr(), p as *mut u8, 16) };
    p
}

#[cfg(not(target_arch = "x86_64"))]
fn jump_thunk(target: usize) -> usize {
    target
}

#[cfg(target_arch = "x86_64")]
struct TrapId {
    name: String,
}

/// `regs` = rax rcx rdx rbx rbp rsi rdi r8..r15 as pushed by the trap thunk.
#[cfg(target_arch = "x86_64")]
unsafe extern "win64" fn trap_handler(id: *const TrapId, ret: usize, regs: *const [u64; 15]) -> ! {
    let id = &*id;
    d3d12_metal::trace::flush();
    log::error!("UNIMPLEMENTED {} (COM method) called from {}", id.name, crate::symbolize(ret));
    eprintln!("UNIMPLEMENTED {} called from {}", id.name, crate::symbolize(ret));
    let r = &*regs;
    let mut ctx = crate::unwind::Context::zeroed();
    ctx.gpr[0] = r[0];
    ctx.gpr[1] = r[1];
    ctx.gpr[2] = r[2];
    ctx.gpr[3] = r[3];
    ctx.gpr[4] = regs as u64 + 15 * 8 + 8;
    ctx.gpr[5] = r[4];
    ctx.gpr[6] = r[5];
    ctx.gpr[7] = r[6];
    ctx.gpr[8..16].copy_from_slice(&r[7..15]);
    ctx.rip = ret as u64;
    log::error!("guest backtrace:");
    crate::unwind::log_backtrace(&ctx);
    crate::crash::report(&format!("unimplemented COM method {}", id.name), None, &ctx);
    if let Some(s) = std::env::var("PORT_EXIT_HOLD")
        .or_else(|_| std::env::var("AW2_EXIT_HOLD"))
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
    {
        log::error!("EXIT_HOLD: holding {s} s before exit");
        libc::sleep(s);
    }
    crate::process::exit_process(0xC000_0139)
}

#[cfg(not(target_arch = "x86_64"))]
const MAX_COM_TRAPS: usize = 1024;

#[cfg(not(target_arch = "x86_64"))]
static COM_TRAP_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[cfg(not(target_arch = "x86_64"))]
#[cold]
#[inline(never)]
unsafe fn com_trap_dispatch(idx: usize) -> ! {
    let name = COM_TRAP_NAMES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(idx)
        .cloned()
        .unwrap_or_else(|| format!("<unknown COM trap #{idx}>"));
    d3d12_metal::trace::flush();
    let caller = crate::crash::guest_caller_hint()
        .map(|pc| crate::symbolize(pc as usize))
        .unwrap_or_else(|| "<unknown caller>".to_string());
    log::error!("UNIMPLEMENTED {name} (COM method) called from {caller}");
    eprintln!("UNIMPLEMENTED {name} called from {caller}");
    let ctx = crate::crash::capture_guest_context()
        .unwrap_or_else(crate::unwind::Context::zeroed);
    crate::crash::report(&format!("unimplemented COM method {name}"), None, &ctx);
    if let Some(s) = std::env::var("PORT_EXIT_HOLD")
        .or_else(|_| std::env::var("AW2_EXIT_HOLD"))
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
    {
        log::error!("EXIT_HOLD: holding {s} s before exit");
        libc::sleep(s);
    }
    crate::process::exit_process(0xC000_0139)
}

#[cfg(not(target_arch = "x86_64"))]
#[cold]
#[inline(never)]
unsafe extern "win64" fn com_trap_entry<const N: usize>() -> ! {
    com_trap_dispatch(N)
}

#[cfg(not(target_arch = "x86_64"))]
#[cold]
#[inline(never)]
unsafe extern "win64" fn com_trap_overflow() -> ! {
    com_trap_dispatch(usize::MAX)
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! com_trap_row16 {
    ($base:expr) => {
        [
            com_trap_entry::<{ $base + 0 }>,
            com_trap_entry::<{ $base + 1 }>,
            com_trap_entry::<{ $base + 2 }>,
            com_trap_entry::<{ $base + 3 }>,
            com_trap_entry::<{ $base + 4 }>,
            com_trap_entry::<{ $base + 5 }>,
            com_trap_entry::<{ $base + 6 }>,
            com_trap_entry::<{ $base + 7 }>,
            com_trap_entry::<{ $base + 8 }>,
            com_trap_entry::<{ $base + 9 }>,
            com_trap_entry::<{ $base + 10 }>,
            com_trap_entry::<{ $base + 11 }>,
            com_trap_entry::<{ $base + 12 }>,
            com_trap_entry::<{ $base + 13 }>,
            com_trap_entry::<{ $base + 14 }>,
            com_trap_entry::<{ $base + 15 }>,
        ]
    };
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! com_trap_block64 {
    ($base:expr) => {
        [
            com_trap_row16!($base + 0),
            com_trap_row16!($base + 16),
            com_trap_row16!($base + 32),
            com_trap_row16!($base + 48),
        ]
    };
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! com_trap_block256 {
    ($base:expr) => {
        [
            com_trap_block64!($base + 0),
            com_trap_block64!($base + 64),
            com_trap_block64!($base + 128),
            com_trap_block64!($base + 192),
        ]
    };
}

#[cfg(not(target_arch = "x86_64"))]
static COM_TRAP_TABLE: [[[[unsafe extern "win64" fn() -> !; 16]; 4]; 4]; 4] = [
    com_trap_block256!(0),
    com_trap_block256!(256),
    com_trap_block256!(512),
    com_trap_block256!(768),
];

#[cfg(not(target_arch = "x86_64"))]
fn com_trap_fn_ptr(idx: usize) -> usize {
    let b256 = idx / 256;
    let rem256 = idx % 256;
    let b64 = rem256 / 64;
    let rem64 = rem256 % 64;
    let r16 = rem64 / 16;
    let c = rem64 % 16;
    COM_TRAP_TABLE[b256][b64][r16][c] as *const () as usize
}

#[cfg(target_arch = "x86_64")]
fn trap_thunk(name: String) -> usize {
    let id = Box::leak(Box::new(TrapId { name }));
    let mut c = Vec::with_capacity(80);
    for r in (0x50..=0x57u8).rev() {
        c.extend_from_slice(&[0x41, r]); // push r15..r8
    }
    c.extend_from_slice(&[0x57, 0x56, 0x55, 0x53, 0x52, 0x51, 0x50]); // push rdi rsi rbp rbx rdx rcx rax
    c.extend_from_slice(&[0x49, 0x89, 0xE0]); // mov r8, rsp
    c.extend_from_slice(&[0x48, 0x8B, 0x54, 0x24, 0x78]); // mov rdx, [rsp+0x78]
    c.extend_from_slice(&[0x48, 0xB9]);
    c.extend_from_slice(&(id as *const TrapId as u64).to_le_bytes());
    c.extend_from_slice(&[0x48, 0xB8]);
    c.extend_from_slice(&(trap_handler as *const () as u64).to_le_bytes());
    c.extend_from_slice(&[0x48, 0x83, 0xE4, 0xF0]); // and rsp, -16
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]); // sub rsp, 0x20
    c.extend_from_slice(&[0xFF, 0xD0, 0x0F, 0x0B]); // call rax; ud2
    let p = arena_alloc(&CODE, c.len(), true);
    unsafe { std::ptr::copy_nonoverlapping(c.as_ptr(), p as *mut u8, c.len()) };
    p
}

#[cfg(not(target_arch = "x86_64"))]
fn trap_thunk(name: String) -> usize {
    let mut names = COM_TRAP_NAMES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = names.iter().position(|n| n == &name) {
        return com_trap_fn_ptr(existing);
    }
    let idx = names.len();
    if idx < MAX_COM_TRAPS {
        names.push(name);
        com_trap_fn_ptr(idx)
    } else {
        log::warn!(
            target: "d3d",
            "COM trap table full ({MAX_COM_TRAPS}); {name} using overflow trap"
        );
        com_trap_overflow as *const () as usize
    }
}

/// Build a vtable for interface `iface` from `(method name, implementation)`
/// pairs in slot order. `None` = trap.
pub fn build_vtable(iface: &str, slots: &[(&str, Option<*const ()>)]) -> *const usize {
    let tbl = arena_alloc(&DATA, slots.len() * 8, false) as *mut usize;
    let mut names = THUNK_NAMES.lock().unwrap();
    let names = names.get_or_insert_with(HashMap::new);
    for (i, (m, f)) in slots.iter().enumerate() {
        let full = format!("{iface}::{m}");
        let t = match f {
            Some(f) => {
                let target = *f as usize;
                let th = jump_thunk(target);
                crate::register_native_va(th as u64, target);
                crate::register_native_va(target as u64, target);
                th
            }
            None => {
                let th = trap_thunk(full.clone());
                crate::register_native_va(th as u64, th);
                th
            }
        };
        names.insert(t, full);
        unsafe { *tbl.add(i) = t };
    }
    tbl
}

// ---------------------------------------------------------------------------
// Objects
// ---------------------------------------------------------------------------

/// Per-class description: the vtable and the IIDs answered by QueryInterface.
pub struct Class {
    pub name: &'static str,
    pub vtbl: *const usize,
    pub iids: &'static [Guid],
}
unsafe impl Sync for Class {}
unsafe impl Send for Class {}

/// Implemented by every Rust type wrapped in [`Obj`].
pub trait ComClass: Sized + Send + Sync + 'static {
    const TRACE_RELEASE: bool = true;
    fn class() -> &'static Class;
    /// Called for IIDs not in `class().iids`; return a pointer (already
    /// AddRef'd) for tear-off interfaces.
    unsafe fn query_extra(_this: *mut Obj<Self>, _iid: &Guid) -> Option<*mut c_void> {
        None
    }
    /// Called once when the refcount drops to zero (before drop).
    fn final_release(&self) {}
}

#[repr(C)]
pub struct Obj<T: ComClass> {
    pub vtbl: *const usize,
    pub refs: AtomicU32,
    pub private: Mutex<PrivateData>,
    pub name: Mutex<Option<String>>,
    pub inner: T,
}

impl<T: ComClass> Obj<T> {
    /// Allocate with refcount 1.
    pub fn new(inner: T) -> *mut Obj<T> {
        Box::into_raw(Box::new(Obj {
            vtbl: T::class().vtbl,
            refs: AtomicU32::new(1),
            private: Mutex::new(PrivateData::default()),
            name: Mutex::new(None),
            inner,
        }))
    }
    pub unsafe fn get<'a>(this: *mut Obj<T>) -> &'a T {
        &(*this).inner
    }
    pub unsafe fn add_ref(this: *mut Obj<T>) -> u32 {
        (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }
    pub unsafe fn release(this: *mut Obj<T>) -> u32 {
        let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
        if n == 0 {
            (*this).inner.final_release();
            drop(Box::from_raw(this));
        }
        n
    }
    /// Is `p` an object of class `T` (vtable identity check)?
    pub unsafe fn from_iface(p: *mut c_void) -> Option<*mut Obj<T>> {
        if p.is_null() {
            return None;
        }
        (*(p as *const *const usize) == T::class().vtbl).then_some(p as *mut Obj<T>)
    }
    pub fn debug_name(&self) -> String {
        self.name.lock().unwrap().clone().unwrap_or_default()
    }
}

/// Call IUnknown::AddRef through the vtable of any COM pointer (guest or native).
pub unsafe fn com_add_ref(p: *mut c_void) -> u32 {
    crate::com::com_call_add_ref(p)
}

pub unsafe fn com_release(p: *mut c_void) -> u32 {
    crate::com::com_call_release(p)
}

pub unsafe fn com_query(p: *mut c_void, iid: &Guid, out: *mut *mut c_void) -> HRESULT {
    crate::com::com_call_qi(p, iid as *const Guid, out)
}

// Generic IUnknown / private-data method bodies, monomorphized per class.

pub unsafe extern "win64" fn qi<T: ComClass>(this: *mut Obj<T>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    *out = std::ptr::null_mut();
    let Some(iid) = Guid::read(riid) else { return E_POINTER };
    let c = T::class();
    if iid == IID_IUnknown || c.iids.contains(&iid) {
        Obj::add_ref(this);
        *out = this as *mut c_void;
        return S_OK;
    }
    if let Some(p) = T::query_extra(this, &iid) {
        *out = p;
        return S_OK;
    }
    crate::com::warn_qi_failed_once(c.name, &iid);
    E_NOINTERFACE
}

pub unsafe extern "win64" fn add_ref<T: ComClass>(this: *mut Obj<T>) -> u32 {
    Obj::add_ref(this)
}

pub unsafe extern "win64" fn release<T: ComClass>(this: *mut Obj<T>) -> u32 {
    let n = Obj::release(this);
    if n == 0 && T::TRACE_RELEASE && super::tr::on() {
        super::tr::untrack(this as u64);
        super::tr::record(&super::tr::Call::Release { object: this as u64 });
    }
    n
}

pub unsafe extern "win64" fn set_private_data<T: ComClass>(this: *mut Obj<T>, g: *const Guid, size: u32, data: *const c_void) -> HRESULT {
    let mut pd = (*this).private.lock().unwrap();
    let hr = pd.set_private_data(g, size, data);
    if hr == S_OK {
        if let Some(gv) = Guid::read(g) {
            if gv == WKPDID_D3DDebugObjectName || gv == WKPDID_D3DDebugObjectNameW {
                *(*this).name.lock().unwrap() = pd.debug_name().map(str::to_string);
            }
        }
    }
    hr
}

pub unsafe extern "win64" fn set_private_data_interface<T: ComClass>(this: *mut Obj<T>, g: *const Guid, p: *mut c_void) -> HRESULT {
    (*this).private.lock().unwrap().set_private_data_interface(g, p)
}

pub const DXGI_ERROR_NOT_FOUND_PRIV: HRESULT = DXGI_ERROR_NOT_FOUND;

pub unsafe extern "win64" fn get_private_data<T: ComClass>(this: *mut Obj<T>, g: *const Guid, size: *mut u32, data: *mut c_void) -> HRESULT {
    (*this).private.lock().unwrap().get_private_data(g, size, data)
}

/// Write an interface pointer to `out` after QI'ing `obj` for `riid`.
pub unsafe fn query_out(obj: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(iid) = Guid::read(riid) else { return E_POINTER };
    com_query(obj, &iid, out)
}

/// Lazily build a class once.
pub fn class_once(cell: &'static OnceLock<Class>, f: impl FnOnce() -> Class) -> &'static Class {
    cell.get_or_init(f)
}

/// Shorthand for a slot implemented by `f`.
macro_rules! slot {
    ($name:literal) => {
        ($name, None)
    };
    ($name:literal => $f:expr) => {
        ($name, Some($f as *const ()))
    };
}

/// Standard IUnknown slots for class `T`.
pub fn iunknown_slots<T: ComClass>() -> Vec<(&'static str, Option<*const ()>)> {
    vec![
        ("QueryInterface", Some(qi::<T> as *const ())),
        ("AddRef", Some(add_ref::<T> as *const ())),
        ("Release", Some(release::<T> as *const ())),
    ]
}

/// IUnknown + `SetPrivateData`/`SetPrivateDataInterface`/`GetPrivateData`
/// in IDXGIObject order (QI, AddRef, Release, SetPrivateData,
/// SetPrivateDataInterface, GetPrivateData).
pub fn dxgi_object_slots<T: ComClass>() -> Vec<(&'static str, Option<*const ()>)> {
    let mut v = iunknown_slots::<T>();
    v.push(("SetPrivateData", Some(set_private_data::<T> as *const ())));
    v.push(("SetPrivateDataInterface", Some(set_private_data_interface::<T> as *const ())));
    v.push(("GetPrivateData", Some(get_private_data::<T> as *const ())));
    v
}

/// Write a UTF-16 string into a fixed `WCHAR[n]` buffer (NUL-terminated).
pub unsafe fn write_wstr(dst: *mut u16, n: usize, s: &str) {
    let w: Vec<u16> = s.encode_utf16().take(n - 1).collect();
    std::ptr::copy_nonoverlapping(w.as_ptr(), dst, w.len());
    *dst.add(w.len()) = 0;
}

/// Read a NUL-terminated UTF-16 string.
pub unsafe fn read_wstr(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    String::from_utf16_lossy(std::slice::from_raw_parts(p, n))
}

// ---------------------------------------------------------------------------
// D3D12 objects: name-based vtables over the generated slot lists
// ---------------------------------------------------------------------------

/// `ID3D12Object::SetName`: also visible as WKPDID_D3DDebugObjectNameW.
pub unsafe extern "win64" fn set_name<T: ComClass>(this: *mut Obj<T>, name: *const u16) -> HRESULT {
    let mut pd = (*this).private.lock().unwrap();
    let hr = pd.set_name(name);
    if hr == S_OK {
        *(*this).name.lock().unwrap() = pd.debug_name().map(str::to_string);
    }
    hr
}

/// The process-wide ID3D12Device (D3D12 devices are singletons per adapter).
pub static DEVICE: std::sync::atomic::AtomicPtr<c_void> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

/// `ID3D12DeviceChild::GetDevice`.
pub unsafe extern "win64" fn get_device<T: ComClass>(_this: *mut Obj<T>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let d = DEVICE.load(Ordering::Acquire);
    if d.is_null() {
        if !out.is_null() {
            *out = std::ptr::null_mut();
        }
        return E_FAIL;
    }
    query_out(d, riid, out)
}

/// Build a vtable for `iface` whose slot order is `names` (a generated
/// list from `vtables.rs`). IUnknown, `ID3D12Object` private data/`SetName`
/// and `ID3D12DeviceChild::GetDevice` are filled generically; `impls` gives
/// the rest by method name; everything else traps with its name.
/// Panics on an impl name that is not in `names` (catches typos/order bugs).
pub fn build_named<T: ComClass>(iface: &str, names: &[&str], impls: &[(&str, *const ())]) -> *const usize {
    for (n, _) in impls {
        assert!(names.contains(n), "{iface}: no method named {n}");
    }
    let slots: Vec<(&str, Option<*const ()>)> = names
        .iter()
        .map(|&n| {
            if let Some((_, f)) = impls.iter().find(|(m, _)| *m == n) {
                return (n, Some(*f));
            }
            let g: Option<*const ()> = match n {
                "QueryInterface" => Some(qi::<T> as *const ()),
                "AddRef" => Some(add_ref::<T> as *const ()),
                "Release" => Some(release::<T> as *const ()),
                "GetPrivateData" => Some(get_private_data::<T> as *const ()),
                "SetPrivateData" => Some(set_private_data::<T> as *const ()),
                "SetPrivateDataInterface" => Some(set_private_data_interface::<T> as *const ()),
                "SetName" => Some(set_name::<T> as *const ()),
                "GetDevice" => Some(get_device::<T> as *const ()),
                _ => None,
            };
            (n, g)
        })
        .collect();
    build_vtable(iface, &slots)
}

/// Shorthand: `m!("Name" => func)` → `("Name", func as *const ())`.
macro_rules! m {
    ($name:literal => $f:expr) => {
        ($name, $f as *const ())
    };
}

// Make the macros importable with `use crate::d3d12::com::*`.
pub(crate) use {m, slot};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_parse() {
        let g = guid("770aae78-f26f-4dba-a829-253c83d1b387");
        assert_eq!(g.d1, 0x770aae78);
        assert_eq!(g.d2, 0xf26f);
        assert_eq!(g.d3, 0x4dba);
        assert_eq!(g.d4, [0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87]);
        assert_eq!(std::mem::size_of::<Guid>(), 16);
    }

    struct Dummy(u32);
    static DUMMY: OnceLock<Class> = OnceLock::new();
    const IID_DUMMY: Guid = guid("11111111-2222-3333-4444-555555555555");
    unsafe extern "win64" fn get_val(this: *mut Obj<Dummy>) -> u32 {
        Obj::get(this).0
    }
    impl ComClass for Dummy {
        fn class() -> &'static Class {
            class_once(&DUMMY, || {
                let mut s = dxgi_object_slots::<Dummy>();
                s.push(slot!("GetVal" => get_val));
                s.push(slot!("Missing"));
                s.push(slot!("MissingTwo"));
                Class { name: "IDummy", vtbl: build_vtable("IDummy", &s), iids: &[IID_DUMMY] }
            })
        }
    }

    #[test]
    fn object_through_vtable() {
        unsafe {
            let o = Obj::new(Dummy(42)) as *mut c_void;
            let vt = *(o as *const *const usize);
            let get: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vt.add(6));
            assert_eq!(get(o), 42);
            let mut q: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(o, &IID_DUMMY, &mut q), S_OK);
            assert_eq!(q, o);
            assert_eq!(com_query(o, &guid("99999999-2222-3333-4444-555555555555"), &mut q), E_NOINTERFACE);
            // private data round trip
            let set: unsafe extern "win64" fn(*mut c_void, *const Guid, u32, *const c_void) -> HRESULT = std::mem::transmute(*vt.add(3));
            let getp: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32, *mut c_void) -> HRESULT = std::mem::transmute(*vt.add(5));
            let name = "abc\0";
            assert_eq!(set(o, &WKPDID_D3DDebugObjectName, 4, name.as_ptr() as *const c_void), S_OK);
            let mut sz = 0u32;
            assert_eq!(getp(o, &WKPDID_D3DDebugObjectName, &mut sz, std::ptr::null_mut()), S_OK);
            assert_eq!(sz, 4);
            assert_eq!((*(o as *mut Obj<Dummy>)).debug_name(), "abc");
            let trap1 = *vt.add(7);
            let trap2 = *vt.add(8);
            assert_ne!(trap1, trap2, "distinct missing COM slots must have distinct trap entries");
            assert_eq!(thunk_name(trap1).as_deref(), Some("IDummy::Missing"));
            assert_eq!(thunk_name(trap2).as_deref(), Some("IDummy::MissingTwo"));
            assert_eq!(com_release(o), 1);
            assert_eq!(com_release(o), 0);
        }
    }
}
