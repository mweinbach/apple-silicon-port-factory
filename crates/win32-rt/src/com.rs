//! Universal, alignment-safe COM building blocks shared across `win32-rt` DLLs
//! (`d3d12`, `d3d11`, `dxgi`, `d3dcompiler_47`, `dwrite`, `d2d1`, `dstorage`,
//! `directml`, `msctf`, `mfplat`, `xaudio2_9redist`, etc.).
//!
//! * [`Guid`] + compile-time [`guid`] parser + global IID name registry for logs.
//! * [`ComObj<T>`] refcounted object header (`#[repr(C)] { vtbl, refs, inner }`)
//!   with generic `extern "win64"` [`com_add_ref`], [`com_release`], [`qi_helper`],
//!   [`qi_from_table`], and [`com_from_iface`].
//! * Vtable registration helpers ([`register_vtable`], [`register_vtable_ptrs`],
//!   [`register_vtable_once`], [`register_static_vtable_once`],
//!   [`register_static_ptr_slice`]) calling [`crate::register_native_va`].
//! * Guest/host-safe vtable dispatch ([`com_call_qi`], [`com_call_add_ref`],
//!   [`com_call_release`], [`com_call_slot`]) via [`crate::call_guest8`] and
//!   unaligned pointer reads, plus [`ComPtr`] (Send+Sync refcounted wrapper).
//! * [`PrivateData`]: canonical `ID3D12Object` / `IDXGIObject` / `IDMLObject`
//!   `GetPrivateData` / `SetPrivateData` / `SetPrivateDataInterface` / `SetName`
//!   key-value store.

use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, Once, OnceLock};

// ---------------------------------------------------------------------------
// GUID + IID name registry
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Guid {
    pub d1: u32,
    pub d2: u16,
    pub d3: u16,
    pub d4: [u8; 8],
}

const _: () = assert!(std::mem::size_of::<Guid>() == 16);
const _: () = assert!(std::mem::align_of::<Guid>() == 4);

const fn hex(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex digit in GUID"),
    }
}

const fn hexn(s: &[u8], at: usize, n: usize) -> u64 {
    let mut v = 0u64;
    let mut i = 0;
    while i < n {
        v = (v << 4) | hex(s[at + i]) as u64;
        i += 1;
    }
    v
}

/// Parse `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"` at compile time.
pub const fn guid(s: &str) -> Guid {
    let b = s.as_bytes();
    assert!(b.len() == 36);
    let mut d4 = [0u8; 8];
    d4[0] = hexn(b, 19, 2) as u8;
    d4[1] = hexn(b, 21, 2) as u8;
    let mut i = 0;
    while i < 6 {
        d4[2 + i] = hexn(b, 24 + 2 * i, 2) as u8;
        i += 1;
    }
    Guid {
        d1: hexn(b, 0, 8) as u32,
        d2: hexn(b, 9, 4) as u16,
        d3: hexn(b, 14, 4) as u16,
        d4,
    }
}

impl Guid {
    #[allow(dead_code)]
    pub const ZERO: Guid = Guid { d1: 0, d2: 0, d3: 0, d4: [0; 8] };

    #[allow(dead_code)]
    #[inline]
    pub const fn new(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Self {
        Self { d1, d2, d3, d4 }
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn from_bytes(b: [u8; 16]) -> Self {
        Self {
            d1: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            d2: u16::from_le_bytes([b[4], b[5]]),
            d3: u16::from_le_bytes([b[6], b[7]]),
            d4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn to_bytes(self) -> [u8; 16] {
        let a = self.d1.to_le_bytes();
        let b = self.d2.to_le_bytes();
        let c = self.d3.to_le_bytes();
        [
            a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], self.d4[0], self.d4[1], self.d4[2],
            self.d4[3], self.d4[4], self.d4[5], self.d4[6], self.d4[7],
        ]
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn from_win_guid(g: GUID) -> Self {
        Self { d1: g.data1, d2: g.data2, d3: g.data3, d4: g.data4 }
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn to_win_guid(self) -> GUID {
        GUID { data1: self.d1, data2: self.d2, data3: self.d3, data4: self.d4 }
    }

    #[inline]
    pub unsafe fn read(p: *const Guid) -> Option<Guid> {
        if p.is_null() {
            None
        } else {
            Some(std::ptr::read_unaligned(p))
        }
    }
}

impl From<GUID> for Guid {
    #[inline]
    fn from(g: GUID) -> Self {
        Self::from_win_guid(g)
    }
}

impl From<Guid> for GUID {
    #[inline]
    fn from(g: Guid) -> Self {
        g.to_win_guid()
    }
}

impl From<[u8; 16]> for Guid {
    #[inline]
    fn from(b: [u8; 16]) -> Self {
        Self::from_bytes(b)
    }
}

impl From<Guid> for [u8; 16] {
    #[inline]
    fn from(g: Guid) -> Self {
        g.to_bytes()
    }
}

impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            self.d1,
            self.d2,
            self.d3,
            self.d4[0],
            self.d4[1],
            self.d4[2],
            self.d4[3],
            self.d4[4],
            self.d4[5],
            self.d4[6],
            self.d4[7]
        )?;
        if let Some(n) = iid_name(self) {
            write!(f, " {n}")?;
        }
        Ok(())
    }
}

static IID_NAMES: Mutex<Vec<(Guid, &'static str)>> = Mutex::new(Vec::new());

/// Register a human-readable IID name for debug formatting and logs.
pub fn name_iid(g: Guid, n: &'static str) {
    let mut v = IID_NAMES.lock().unwrap_or_else(|e| e.into_inner());
    if !v.iter().any(|(x, _)| *x == g) {
        v.push((g, n));
    }
}

pub fn iid_name(g: &Guid) -> Option<&'static str> {
    IID_NAMES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|(x, _)| x == g)
        .map(|(_, n)| *n)
}

pub const IID_IUnknown: Guid = guid("00000000-0000-0000-c000-000000000046");

/// `WKPDID_D3DDebugObjectName` / `WKPDID_D3DDebugObjectNameW`: names set through private data.
pub const WKPDID_D3DDebugObjectName: Guid = guid("429b8c22-9188-4b0c-8742-acb0bf85c200");
pub const WKPDID_D3DDebugObjectNameW: Guid = guid("4cca5fd8-921f-42c8-8566-70caf2a9b741");

pub const DXGI_ERROR_NOT_FOUND: HRESULT = 0x887A_0002u32 as i32;
pub const DXGI_ERROR_MORE_DATA: HRESULT = 0x887A_0003u32 as i32;

// ---------------------------------------------------------------------------
// Unaligned GUID & out-pointer helpers
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[inline]
pub unsafe fn read_guid(p: *const Guid) -> Option<Guid> {
    Guid::read(p)
}

#[allow(dead_code)]
#[inline]
pub unsafe fn read_guid_from_bytes(p: *const u8) -> Option<Guid> {
    if p.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(p as *const Guid))
    }
}

#[allow(dead_code)]
#[inline]
pub unsafe fn read_guid_bytes(p: *const u8) -> Option<[u8; 16]> {
    if p.is_null() {
        return None;
    }
    let mut b = [0u8; 16];
    std::ptr::copy_nonoverlapping(p, b.as_mut_ptr(), 16);
    Some(b)
}

#[allow(dead_code)]
#[inline]
pub unsafe fn write_guid(dst: *mut Guid, g: Guid) {
    if !dst.is_null() {
        std::ptr::write_unaligned(dst, g);
    }
}

#[allow(dead_code)]
#[inline]
pub unsafe fn write_guid_bytes(dst: *mut u8, g: &[u8; 16]) {
    if !dst.is_null() {
        std::ptr::copy_nonoverlapping(g.as_ptr(), dst, 16);
    }
}

#[allow(dead_code)]
#[inline]
pub unsafe fn write_null_out<T>(pp: *mut *mut T) {
    if !pp.is_null() {
        std::ptr::write_unaligned(pp, std::ptr::null_mut());
    }
}

// ---------------------------------------------------------------------------
// Vtable VA_DISPATCH registration helpers
// ---------------------------------------------------------------------------

/// Register every non-zero native function pointer in `ptrs` with `VA_DISPATCH`.
#[allow(dead_code)]
pub fn register_vtable_ptrs(ptrs: &[usize]) {
    for &addr in ptrs {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
}

/// Register every non-zero native function pointer in `slots` with `VA_DISPATCH`
/// and return `slots` (for use inside `OnceLock::get_or_init`).
#[allow(dead_code)]
pub fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    register_vtable_ptrs(&slots);
    slots
}

/// Lazily initialize and register a fixed-size vtable once via `OnceLock`,
/// returning its stable pointer.
#[allow(dead_code)]
pub fn register_vtable_once<const N: usize>(
    cell: &'static OnceLock<[usize; N]>,
    init: impl FnOnce() -> [usize; N],
) -> *const usize {
    cell.get_or_init(|| register_vtable(init())).as_ptr()
}

/// Register a static vtable's function pointers exactly once via `Once`.
#[allow(dead_code)]
pub fn register_static_vtable_once(once: &'static Once, ptrs: &[usize]) {
    once.call_once(|| register_vtable_ptrs(ptrs));
}

/// Register every non-zero entry in a slice of [`crate::Ptr`] (static `Vtable` arrays).
#[allow(dead_code)]
pub fn register_static_ptr_slice(ptrs: &[crate::Ptr]) {
    for p in ptrs {
        let addr = p.0 as usize;
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
}

// ---------------------------------------------------------------------------
// Generic refcounted COM object (`ComObj<T>`) & IUnknown helpers
// ---------------------------------------------------------------------------

/// Standard single-vtable refcounted COM object header (`#[repr(C)]`).
///
/// The interface pointer handed to guest code is `*mut ComObj<T>` itself, so
/// `*iface` is `vtbl` (`*const usize`).
#[allow(dead_code)]
#[repr(C)]
pub struct ComObj<T> {
    pub vtbl: *const usize,
    pub refs: AtomicU32,
    pub inner: T,
}

#[allow(dead_code)]
impl<T> ComObj<T> {
    /// Allocate a new COM object on the heap with reference count 1.
    #[inline]
    pub fn new(vtbl: *const usize, inner: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            vtbl,
            refs: AtomicU32::new(1),
            inner,
        }))
    }

    #[inline]
    pub unsafe fn inner<'a>(this: *mut Self) -> &'a T {
        &(*this).inner
    }

    #[inline]
    pub unsafe fn ref_count(this: *const Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        (*this).refs.load(Ordering::Acquire)
    }

    #[inline]
    pub unsafe fn add_ref(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }

    #[inline]
    pub unsafe fn release(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        let prev = (*this).refs.fetch_sub(1, Ordering::AcqRel);
        if prev <= 1 {
            drop(Box::from_raw(this));
            0
        } else {
            prev - 1
        }
    }

    /// Downcast an arbitrary interface pointer back to `&ComObj<T>` by verifying
    /// its vtable pointer identity (unaligned-safe).
    #[inline]
    pub unsafe fn from_iface<'a>(ptr: *mut c_void, expected_vtbl: *const usize) -> Option<&'a Self> {
        if ptr.is_null() {
            return None;
        }
        let vt = std::ptr::read_unaligned(ptr as *const *const usize);
        if vt == expected_vtbl {
            Some(&*(ptr as *const Self))
        } else {
            None
        }
    }
}

/// Generic `extern "win64"` `IUnknown::AddRef` body for any `ComObj<T>`.
#[allow(dead_code)]
pub unsafe extern "win64" fn com_add_ref<T>(this: *mut ComObj<T>) -> u32 {
    ComObj::add_ref(this)
}

/// Generic `extern "win64"` `IUnknown::Release` body for any `ComObj<T>`.
#[allow(dead_code)]
pub unsafe extern "win64" fn com_release<T>(this: *mut ComObj<T>) -> u32 {
    ComObj::release(this)
}

/// Downcast a raw COM interface pointer to `&ComObj<T>` using its vtable pointer.
#[allow(dead_code)]
#[inline]
pub unsafe fn com_from_iface<'a, T>(
    ptr: *mut c_void,
    expected_vtbl: *const usize,
) -> Option<&'a ComObj<T>> {
    ComObj::from_iface(ptr, expected_vtbl)
}

/// Standard single-interface / inheritance-chain `IUnknown::QueryInterface` helper
/// for [`ComObj<T>`]. Always accepts `IID_IUnknown` in addition to `iids`.
#[allow(dead_code)]
pub unsafe fn qi_helper<T>(
    this: *mut ComObj<T>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
    iids: &[Guid],
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iids.contains(&iid) {
        ComObj::add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// Byte-pointer variant of [`qi_helper`] for callers declaring `riid: *const u8`.
#[allow(dead_code)]
#[inline]
pub unsafe fn qi_helper_bytes<T>(
    this: *mut ComObj<T>,
    riid: *const u8,
    ppv: *mut *mut c_void,
    iids: &[Guid],
) -> HRESULT {
    qi_helper(this, riid as *const Guid, ppv, iids)
}

/// Multi-interface `QueryInterface` helper for objects exposing secondary vtables
/// or tear-off interface pointers (each entry pairs an IID with the interface
/// pointer to return after `AddRef`ing `this`). Always answers `IID_IUnknown`
/// with `this`.
#[allow(dead_code)]
pub unsafe fn qi_from_table<T>(
    this: *mut ComObj<T>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
    entries: &[(Guid, *mut c_void)],
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown {
        ComObj::add_ref(this);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return S_OK;
    }
    for &(entry_iid, iface_ptr) in entries {
        if iid == entry_iid && !iface_ptr.is_null() {
            ComObj::add_ref(this);
            std::ptr::write_unaligned(ppv, iface_ptr);
            return S_OK;
        }
    }
    E_NOINTERFACE
}

// ---------------------------------------------------------------------------
// Guest/native COM vtable calls & refcounted `ComPtr`
// ---------------------------------------------------------------------------

/// Invoke `IUnknown::QueryInterface` (vtable slot 0) on any COM object (guest or native).
pub unsafe fn com_call_qi(
    obj: *mut c_void,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if obj.is_null() || out.is_null() {
        return E_POINTER;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return E_POINTER;
    }
    let method = std::ptr::read_unaligned(vtbl);
    if method == 0 {
        return E_NOINTERFACE;
    }
    crate::call_guest8(method, obj as u64, riid as u64, out as u64, 0, 0, 0, 0, 0) as HRESULT
}

/// Byte-pointer variant of [`com_call_qi`].
#[allow(dead_code)]
#[inline]
pub unsafe fn com_call_qi_bytes(
    obj: *mut c_void,
    riid: *const u8,
    out: *mut *mut c_void,
) -> HRESULT {
    com_call_qi(obj, riid as *const Guid, out)
}

/// Invoke `IUnknown::AddRef` (vtable slot 1) on any COM object (guest or native).
pub unsafe fn com_call_add_ref(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(1));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke `IUnknown::Release` (vtable slot 2) on any COM object (guest or native).
pub unsafe fn com_call_release(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(2));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke an arbitrary COM vtable `slot` with up to 3 integer/pointer arguments beyond `this`.
#[allow(dead_code)]
pub unsafe fn com_call_slot(
    obj: *mut c_void,
    slot: usize,
    a1: u64,
    a2: u64,
    a3: u64,
) -> HRESULT {
    if obj.is_null() {
        return E_POINTER;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return E_POINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    if method == 0 {
        return E_NOTIMPL;
    }
    crate::call_guest8(method, obj as u64, a1, a2, a3, 0, 0, 0, 0) as HRESULT
}

/// Invoke an arbitrary COM vtable `slot` with up to 7 arguments beyond `this`, returning raw `u64`.
#[allow(dead_code)]
pub unsafe fn com_call_slot_raw(obj: *mut c_void, slot: usize, args: &[u64]) -> Option<u64> {
    if obj.is_null() {
        return None;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return None;
    }
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    if method == 0 {
        return None;
    }
    let a = |i: usize| args.get(i).copied().unwrap_or(0);
    Some(crate::call_guest8(
        method,
        obj as u64,
        a(0),
        a(1),
        a(2),
        a(3),
        a(4),
        a(5),
        a(6),
    ))
}

/// `Send + Sync` refcounted COM interface pointer wrapper.
///
/// Calls `IUnknown::AddRef` / `IUnknown::Release` via [`crate::call_guest8`], so it works
/// safely for both native Rust COM objects and guest x86-64 COM objects on ARM64 and x86_64.
#[derive(Debug)]
pub struct ComPtr(*mut c_void);

unsafe impl Send for ComPtr {}
unsafe impl Sync for ComPtr {}

#[allow(dead_code)]
impl ComPtr {
    /// Retain a borrowed COM pointer (calls `AddRef`). Returns `None` if null.
    #[inline]
    pub unsafe fn retain(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            com_call_add_ref(ptr);
            Some(Self(ptr))
        }
    }

    /// Take ownership of an already-`AddRef`'d COM pointer without calling `AddRef`.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Release ownership without calling `IUnknown::Release`.
    #[inline]
    pub fn into_raw(self) -> *mut c_void {
        let p = self.0;
        std::mem::forget(self);
        p
    }

    /// Invoke `IUnknown::QueryInterface` on the wrapped pointer.
    pub unsafe fn query_interface(&self, iid: &Guid) -> Result<ComPtr, HRESULT> {
        let mut out: *mut c_void = std::ptr::null_mut();
        let hr = com_call_qi(self.0, iid as *const Guid, &mut out);
        if hr >= 0 && !out.is_null() {
            Ok(Self(out))
        } else if hr >= 0 {
            Err(E_NOINTERFACE)
        } else {
            Err(hr)
        }
    }
}

impl Clone for ComPtr {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { com_call_add_ref(self.0) };
        Self(self.0)
    }
}

impl Drop for ComPtr {
    #[inline]
    fn drop(&mut self) {
        unsafe { com_call_release(self.0) };
    }
}

// ---------------------------------------------------------------------------
// Shared `PrivateData` store (`ID3D12Object` / `IDXGIObject` / `IDMLObject`)
// ---------------------------------------------------------------------------

enum PrivEntry {
    Bytes(Vec<u8>),
    Iface(ComPtr),
}

/// Canonical COM private-data key-value store implementing `GetPrivateData`,
/// `SetPrivateData`, `SetPrivateDataInterface`, and `SetName` (`WKPDID_D3DDebugObjectName*`).
#[derive(Default)]
pub struct PrivateData {
    entries: Vec<(Guid, PrivEntry)>,
    debug_name: Option<String>,
}

impl PrivateData {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            debug_name: None,
        }
    }

    #[inline]
    pub fn debug_name(&self) -> Option<&str> {
        self.debug_name.as_deref()
    }

    /// Implement `SetPrivateData(guid, data_size, p_data)`.
    pub unsafe fn set_private_data(
        &mut self,
        g: *const Guid,
        size: u32,
        data: *const c_void,
    ) -> HRESULT {
        let Some(g) = read_guid(g) else {
            return E_INVALIDARG;
        };
        if let Some(i) = self.entries.iter().position(|(k, _)| *k == g) {
            self.entries.remove(i);
        }
        if data.is_null() || size == 0 {
            if g == WKPDID_D3DDebugObjectName || g == WKPDID_D3DDebugObjectNameW {
                self.debug_name = None;
            }
            return S_OK;
        }
        let mut bytes = vec![0u8; size as usize];
        std::ptr::copy_nonoverlapping(data as *const u8, bytes.as_mut_ptr(), size as usize);
        if g == WKPDID_D3DDebugObjectName {
            self.debug_name = Some(
                String::from_utf8_lossy(&bytes)
                    .trim_end_matches('\0')
                    .to_string(),
            );
        } else if g == WKPDID_D3DDebugObjectNameW {
            let w: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            self.debug_name = Some(
                String::from_utf16_lossy(&w)
                    .trim_end_matches('\0')
                    .to_string(),
            );
        }
        self.entries.push((g, PrivEntry::Bytes(bytes)));
        S_OK
    }

    /// Implement `SetPrivateDataInterface(guid, p_unknown)`.
    pub unsafe fn set_private_data_interface(
        &mut self,
        g: *const Guid,
        unk: *mut c_void,
    ) -> HRESULT {
        let Some(g) = read_guid(g) else {
            return E_INVALIDARG;
        };
        if let Some(i) = self.entries.iter().position(|(k, _)| *k == g) {
            self.entries.remove(i);
        }
        if let Some(ptr) = ComPtr::retain(unk) {
            self.entries.push((g, PrivEntry::Iface(ptr)));
        }
        S_OK
    }

    /// Implement `GetPrivateData(guid, p_data_size, p_data)` (unaligned-safe).
    pub unsafe fn get_private_data(
        &self,
        g: *const Guid,
        size: *mut u32,
        data: *mut c_void,
    ) -> HRESULT {
        let Some(g) = read_guid(g) else {
            return E_INVALIDARG;
        };
        if size.is_null() {
            return E_INVALIDARG;
        }
        let Some((_, e)) = self.entries.iter().find(|(k, _)| *k == g) else {
            std::ptr::write_unaligned(size, 0);
            return DXGI_ERROR_NOT_FOUND;
        };
        let avail = std::ptr::read_unaligned(size) as usize;
        match e {
            PrivEntry::Bytes(b) => {
                if data.is_null() {
                    std::ptr::write_unaligned(size, b.len() as u32);
                    return S_OK;
                }
                if avail < b.len() {
                    std::ptr::write_unaligned(size, b.len() as u32);
                    return DXGI_ERROR_MORE_DATA;
                }
                if !b.is_empty() {
                    std::ptr::copy_nonoverlapping(b.as_ptr(), data as *mut u8, b.len());
                }
                std::ptr::write_unaligned(size, b.len() as u32);
                S_OK
            }
            PrivEntry::Iface(p) => {
                const PTR_SIZE: usize = std::mem::size_of::<*mut c_void>();
                if data.is_null() {
                    std::ptr::write_unaligned(size, PTR_SIZE as u32);
                    return S_OK;
                }
                if avail < PTR_SIZE {
                    std::ptr::write_unaligned(size, PTR_SIZE as u32);
                    return DXGI_ERROR_MORE_DATA;
                }
                com_call_add_ref(p.as_ptr());
                std::ptr::write_unaligned(data as *mut *mut c_void, p.as_ptr());
                std::ptr::write_unaligned(size, PTR_SIZE as u32);
                S_OK
            }
        }
    }

    /// Implement `SetName(LPCWSTR)` via `WKPDID_D3DDebugObjectNameW`.
    pub unsafe fn set_name(&mut self, name: *const u16) -> HRESULT {
        if name.is_null() {
            return self.set_private_data(&WKPDID_D3DDebugObjectNameW, 0, std::ptr::null());
        }
        let mut units = crate::strings::read_wide_units_z(name, 65_536);
        units.push(0);
        let byte_len = (units.len() * 2) as u32;
        self.set_private_data(
            &WKPDID_D3DDebugObjectNameW,
            byte_len,
            units.as_ptr() as *const c_void,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    const IID_TEST_PRIMARY: Guid = guid("12345678-9abc-def0-1234-56789abcdef0");
    const IID_TEST_SECONDARY: Guid = guid("abcdef01-2345-6789-abcd-ef0123456789");
    const IID_TEST_UNKNOWN: Guid = guid("deadbeef-0000-0000-0000-000000000001");

    struct TestInner {
        val: u32,
        dropped: Arc<AtomicBool>,
        secondary_vtbl: *const usize,
    }

    impl Drop for TestInner {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::Release);
        }
    }

    unsafe extern "win64" fn test_qi(
        this: *mut ComObj<TestInner>,
        riid: *const Guid,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        if this.is_null() {
            return qi_helper(this, riid, ppv, &[IID_TEST_PRIMARY]);
        }
        let sec_ptr = std::ptr::addr_of!((*this).inner.secondary_vtbl) as *mut c_void;
        qi_from_table(
            this,
            riid,
            ppv,
            &[
                (IID_TEST_PRIMARY, this as *mut c_void),
                (IID_TEST_SECONDARY, sec_ptr),
            ],
        )
    }

    unsafe extern "win64" fn test_add_values(
        this: *mut ComObj<TestInner>,
        a: u64,
        b: u64,
        c: u64,
    ) -> HRESULT {
        (ComObj::inner(this).val as u64 + a + b + c) as HRESULT
    }

    fn test_vtbl() -> *const usize {
        static VT: OnceLock<[usize; 4]> = OnceLock::new();
        register_vtable_once(&VT, || {
            [
                test_qi as *const () as usize,
                com_add_ref::<TestInner> as *const () as usize,
                com_release::<TestInner> as *const () as usize,
                test_add_values as *const () as usize,
            ]
        })
    }

    #[test]
    fn guid_parse_bytes_and_win_conversion() {
        let g = guid("770aae78-f26f-4dba-a829-253c83d1b387");
        assert_eq!(g.d1, 0x770a_ae78);
        assert_eq!(g.d2, 0xf26f);
        assert_eq!(g.d3, 0x4dba);
        assert_eq!(g.d4, [0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87]);
        assert_eq!(Guid::from_bytes(g.to_bytes()), g);

        let wg: GUID = g.into();
        assert_eq!(Guid::from(wg), g);

        // Unaligned read/write into odd byte offset.
        let mut buf = [0u8; 32];
        unsafe {
            let p = buf.as_mut_ptr().add(3) as *mut Guid;
            write_guid(p, g);
            assert_eq!(read_guid(p), Some(g));
            assert_eq!(read_guid_from_bytes(buf.as_ptr().add(3)), Some(g));
            assert_eq!(read_guid_bytes(buf.as_ptr().add(3)), Some(g.to_bytes()));
            assert_eq!(read_guid(std::ptr::null()), None);
        }
    }

    #[test]
    fn com_obj_lifecycle_qi_comptr_and_slot_call() {
        let dropped = Arc::new(AtomicBool::new(false));
        let vt = test_vtbl();
        let raw = ComObj::new(
            vt,
            TestInner {
                val: 10,
                dropped: dropped.clone(),
                secondary_vtbl: vt,
            },
        ) as *mut c_void;

        unsafe {
            assert!(com_from_iface::<TestInner>(raw, vt).is_some());
            assert!(com_from_iface::<TestInner>(raw, std::ptr::null()).is_none());
            assert!(com_from_iface::<TestInner>(std::ptr::null_mut(), vt).is_none());

            // Arbitrary slot invocation via call_guest8.
            assert_eq!(com_call_slot(raw, 3, 2, 3, 4), 19);
            assert_eq!(com_call_slot_raw(raw, 3, &[5, 6, 7]), Some(28));

            // Take ownership via ComPtr without extra AddRef.
            let p1 = ComPtr::from_raw(raw).unwrap();
            assert_eq!(ComObj::ref_count(raw as *const ComObj<TestInner>), 1);

            // Clone + QueryInterface (primary, secondary, unknown).
            let p2 = p1.clone();
            assert_eq!(ComObj::ref_count(raw as *const ComObj<TestInner>), 2);

            let q_primary = p1.query_interface(&IID_TEST_PRIMARY).unwrap();
            assert_eq!(q_primary.as_ptr(), raw);
            assert_eq!(ComObj::ref_count(raw as *const ComObj<TestInner>), 3);

            let mut sec_out: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(raw, &IID_TEST_SECONDARY, &mut sec_out), S_OK);
            assert_ne!(sec_out, raw);
            assert!(!sec_out.is_null());
            assert_eq!(ComObj::ref_count(raw as *const ComObj<TestInner>), 4);
            // Release the secondary QI ref directly on primary object.
            assert_eq!(com_call_release(raw), 3);

            assert_eq!(
                p1.query_interface(&IID_TEST_UNKNOWN).unwrap_err(),
                E_NOINTERFACE
            );

            drop(q_primary);
            drop(p2);
            assert!(!dropped.load(Ordering::Acquire));
            drop(p1);
            assert!(dropped.load(Ordering::Acquire));
        }
    }

    #[test]
    fn private_data_bytes_interfaces_and_set_name() {
        let mut pd = PrivateData::new();
        let key_bytes = guid("aaaaaaaa-1111-2222-3333-444444444444");
        let key_iface = guid("bbbbbbbb-1111-2222-3333-444444444444");

        unsafe {
            // Missing key -> DXGI_ERROR_NOT_FOUND
            let mut sz = 99u32;
            assert_eq!(
                pd.get_private_data(&key_bytes, &mut sz, std::ptr::null_mut()),
                DXGI_ERROR_NOT_FOUND
            );
            assert_eq!(sz, 0);

            // Bytes round-trip + MORE_DATA check.
            let payload = [1u8, 2, 3, 4, 5];
            assert_eq!(
                pd.set_private_data(&key_bytes, payload.len() as u32, payload.as_ptr() as *const c_void),
                S_OK
            );
            let mut query_sz = 0u32;
            assert_eq!(
                pd.get_private_data(&key_bytes, &mut query_sz, std::ptr::null_mut()),
                S_OK
            );
            assert_eq!(query_sz, 5);
            let mut small = [0u8; 3];
            let mut small_sz = 3u32;
            assert_eq!(
                pd.get_private_data(&key_bytes, &mut small_sz, small.as_mut_ptr() as *mut c_void),
                DXGI_ERROR_MORE_DATA
            );
            assert_eq!(small_sz, 5);
            let mut out = [0u8; 5];
            assert_eq!(
                pd.get_private_data(&key_bytes, &mut small_sz, out.as_mut_ptr() as *mut c_void),
                S_OK
            );
            assert_eq!(out, payload);

            // SetName (UTF-16) and clear with NULL.
            let wname = crate::strings::to_wide_z("TestBuffer");
            assert_eq!(pd.set_name(wname.as_ptr()), S_OK);
            assert_eq!(pd.debug_name(), Some("TestBuffer"));
            assert_eq!(pd.set_name(std::ptr::null()), S_OK);
            assert_eq!(pd.debug_name(), None);

            // SetPrivateDataInterface retains & releases COM object cleanly.
            let dropped = Arc::new(AtomicBool::new(false));
            let vt = test_vtbl();
            let obj = ComObj::new(
                vt,
                TestInner {
                    val: 7,
                    dropped: dropped.clone(),
                    secondary_vtbl: vt,
                },
            ) as *mut c_void;
            assert_eq!(pd.set_private_data_interface(&key_iface, obj), S_OK);
            assert_eq!(ComObj::ref_count(obj as *const ComObj<TestInner>), 2);

            let mut got_ptr: *mut c_void = std::ptr::null_mut();
            let mut psz = 8u32;
            assert_eq!(
                pd.get_private_data(
                    &key_iface,
                    &mut psz,
                    (&mut got_ptr as *mut *mut c_void) as *mut c_void
                ),
                S_OK
            );
            assert_eq!(got_ptr, obj);
            assert_eq!(ComObj::ref_count(obj as *const ComObj<TestInner>), 3);
            assert_eq!(com_call_release(got_ptr), 2);
            // Drop our original ref; PrivateData still holds 1 ref.
            assert_eq!(com_call_release(obj), 1);
            assert!(!dropped.load(Ordering::Acquire));
            // Drop PrivateData -> releases last ref.
            drop(pd);
            assert!(dropped.load(Ordering::Acquire));
        }
    }
}
