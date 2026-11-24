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

/// Log a failed `QueryInterface` (`E_NOINTERFACE`) once per distinct `(class_name, iid)` at
/// warning level, demoting repeat probes to debug level.
pub fn warn_qi_failed_once(class_name: &str, iid: &Guid) {
    static SEEN: Mutex<Option<std::collections::HashSet<(String, Guid)>>> = Mutex::new(None);
    let first = {
        let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(std::collections::HashSet::new)
            .insert((class_name.to_string(), *iid))
    };
    if first {
        log::warn!(target: "com", "{class_name}::QueryInterface({iid:?}) -> E_NOINTERFACE");
    } else {
        log::debug!(target: "com", "{class_name}::QueryInterface({iid:?}) -> E_NOINTERFACE");
    }
}

/// Log an unregistered COM CLSID (`REGDB_E_CLASSNOTREG`) once per distinct `(api, clsid, iid)`
/// at warning level, demoting repeat probes to debug level.
pub fn warn_unknown_clsid_once(api: &str, clsid: Option<&Guid>, iid: Option<&Guid>) {
    static SEEN: Mutex<Option<std::collections::HashSet<(String, Option<Guid>, Option<Guid>)>>> =
        Mutex::new(None);
    let key = (api.to_string(), clsid.copied(), iid.copied());
    let first = {
        let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(std::collections::HashSet::new).insert(key)
    };
    let clsid_s = clsid.map_or_else(|| "NULL".to_string(), |g| format!("{g:?}"));
    let iid_s = iid.map_or_else(|| "NULL".to_string(), |g| format!("{g:?}"));
    if first {
        log::warn!(
            target: "com",
            "{api}: unregistered COM class {clsid_s} (iid {iid_s}) -> REGDB_E_CLASSNOTREG"
        );
    } else {
        log::debug!(
            target: "com",
            "{api}: unregistered COM class {clsid_s} (iid {iid_s}) -> REGDB_E_CLASSNOTREG"
        );
    }
}

/// Log an unregistered WinRT activatable class name once per distinct `(api, class_name)` at
/// warning level, demoting repeat probes to debug level.
pub fn warn_unknown_winrt_class_once(api: &str, class_name: &str) {
    static SEEN: Mutex<Option<std::collections::HashSet<(String, String)>>> = Mutex::new(None);
    let first = {
        let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(std::collections::HashSet::new)
            .insert((api.to_string(), class_name.to_string()))
    };
    if first {
        log::warn!(
            target: "com",
            "{api}({class_name:?}): class not registered -> REGDB_E_CLASSNOTREG"
        );
    } else {
        log::debug!(
            target: "com",
            "{api}({class_name:?}): class not registered -> REGDB_E_CLASSNOTREG"
        );
    }
}

pub const IID_IUnknown: Guid = guid("00000000-0000-0000-c000-000000000046");
pub const IID_IClassFactory: Guid = guid("00000001-0000-0000-c000-000000000046");
pub const IID_IMalloc: Guid = guid("00000002-0000-0000-c000-000000000046");
pub const IID_IMarshal: Guid = guid("00000003-0000-0000-c000-000000000046");
pub const IID_IStream: Guid = guid("0000000c-0000-0000-c000-000000000046");
pub const IID_ISequentialStream: Guid = guid("0c733a30-2a1c-11ce-ade5-00aa0044773d");
pub const IID_IAgileObject: Guid = guid("94ea2b94-e9cc-49e0-c0ff-ee64ca8f5b90");
pub const IID_IDispatch: Guid = guid("00020400-0000-0000-c000-000000000046");
pub const IID_IClassFactory2: Guid = guid("b196b28f-bab4-101a-b69c-00aa00341d07");
pub const IID_IInspectable: Guid = guid("af86e2e0-b12d-4c6a-9c5a-d7aa65101e90");
pub const IID_IActivationFactory: Guid = guid("00000035-0000-0000-c000-000000000046");
pub const IID_IConnectionPointContainer: Guid = guid("b196b284-bab4-101a-b69c-00aa00341d07");
pub const IID_IEnumConnectionPoints: Guid = guid("b196b285-bab4-101a-b69c-00aa00341d07");
pub const IID_IConnectionPoint: Guid = guid("b196b286-bab4-101a-b69c-00aa00341d07");
pub const IID_IEnumConnections: Guid = guid("b196b287-bab4-101a-b69c-00aa00341d07");
pub const IID_IPropertyValue: Guid = guid("4bd682dd-7554-40e9-9a9b-82654ede7e62");
pub const IID_IPropertyValueStatics: Guid = guid("629bdbc8-d932-4ff4-96b9-8d96c5c1e858");
pub const IID_IUriRuntimeClass: Guid = guid("9e365e57-48b2-4160-956f-c7385120bbfc");
pub const IID_IUriRuntimeClassFactory: Guid = guid("44a9796f-723e-4fdf-a218-033e75b0c084");
pub const IID_IShellItem: Guid = guid("43826d1e-e718-42ee-bc55-a1e261c37bfe");

pub const CONNECT_E_NOCONNECTION: HRESULT = 0x8004_0200u32 as i32;
#[allow(dead_code)]
pub const CONNECT_E_ADVISELIMIT: HRESULT = 0x8004_0201u32 as i32;
pub const CONNECT_E_CANNOTCONNECT: HRESULT = 0x8004_0202u32 as i32;

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

/// Register every native method pointer in a `#[repr(C)]` static COM vtable struct with
/// `VA_DISPATCH` once per vtable address so ARM64 guest indirect calls dispatch to native
/// Rust instead of executing host `__TEXT` as x86-64 bytes.
///
/// Holds `SEEN` across registration and only marks `base` initialized after every slot has
/// been published, so concurrent callers wait until all method pointers are registered.
#[allow(dead_code)]
pub fn register_vtbl_struct<V: 'static>(vtbl: &'static V) {
    let base = vtbl as *const V as usize;
    static SEEN: Mutex<Option<std::collections::HashSet<usize>>> = Mutex::new(None);
    let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    let seen = g.get_or_insert_with(std::collections::HashSet::new);
    if !seen.contains(&base) {
        let count = std::mem::size_of::<V>() / std::mem::size_of::<usize>();
        let ptrs = unsafe { std::slice::from_raw_parts(vtbl as *const V as *const usize, count) };
        register_vtable_ptrs(ptrs);
        seen.insert(base);
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
        warn_qi_failed_once(std::any::type_name::<T>(), &iid);
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
    warn_qi_failed_once(std::any::type_name::<T>(), &iid);
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

// ---------------------------------------------------------------------------
// Typed COM interface & callback registry (IID + vtable slot metadata)
// ---------------------------------------------------------------------------

/// Return type category for a COM vtable slot descriptor.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComReturnKind {
    HResult,
    U32,
    U64,
    Void,
}

/// Argument type category for a COM vtable slot parameter (excluding `this`).
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComArgKind {
    Ptr,
    GuidPtr,
    OutPtr,
    U32,
    I32,
    U64,
    I64,
    Bool,
    HString,
}

/// Metadata describing one COM vtable slot.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComSlotDesc {
    pub slot: usize,
    pub name: &'static str,
    /// Number of Win64 parameters beyond `this` (0..=7).
    pub arity: usize,
    pub args: &'static [ComArgKind],
    pub ret: ComReturnKind,
}

/// Metadata describing a COM/WinRT interface, its vtable slots, and optional host vtable pointer.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ComInterfaceDesc {
    pub iid: Guid,
    pub name: &'static str,
    pub base_iid: Option<Guid>,
    pub slots: &'static [ComSlotDesc],
    pub host_vtbl: Option<usize>,
}

const SLOTS_IUNKNOWN: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 0,
        name: "QueryInterface",
        arity: 2,
        args: &[ComArgKind::GuidPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 1,
        name: "AddRef",
        arity: 0,
        args: &[],
        ret: ComReturnKind::U32,
    },
    ComSlotDesc {
        slot: 2,
        name: "Release",
        arity: 0,
        args: &[],
        ret: ComReturnKind::U32,
    },
];

const SLOTS_ICLASSFACTORY: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "CreateInstance",
        arity: 3,
        args: &[ComArgKind::Ptr, ComArgKind::GuidPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "LockServer",
        arity: 1,
        args: &[ComArgKind::Bool],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_ICLASSFACTORY2: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 5,
        name: "GetLicInfo",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "RequestLicKey",
        arity: 2,
        args: &[ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "CreateInstanceLic",
        arity: 5,
        args: &[
            ComArgKind::Ptr,
            ComArgKind::Ptr,
            ComArgKind::GuidPtr,
            ComArgKind::Ptr,
            ComArgKind::OutPtr,
        ],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IINSPECTABLE: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "GetIids",
        arity: 2,
        args: &[ComArgKind::OutPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "GetRuntimeClassName",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "GetTrustLevel",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IACTIVATIONFACTORY: &[ComSlotDesc] = &[ComSlotDesc {
    slot: 6,
    name: "ActivateInstance",
    arity: 1,
    args: &[ComArgKind::OutPtr],
    ret: ComReturnKind::HResult,
}];

const SLOTS_ICONNECTIONPOINTCONTAINER: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "EnumConnectionPoints",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "FindConnectionPoint",
        arity: 2,
        args: &[ComArgKind::GuidPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_ICONNECTIONPOINT: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "GetConnectionInterface",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "GetConnectionPointContainer",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "Advise",
        arity: 2,
        args: &[ComArgKind::Ptr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "Unadvise",
        arity: 1,
        args: &[ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "EnumConnections",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IENUM_GENERIC: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "Next",
        arity: 3,
        args: &[ComArgKind::U32, ComArgKind::OutPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "Skip",
        arity: 1,
        args: &[ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "Reset",
        arity: 0,
        args: &[],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "Clone",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IDISPATCH: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "GetTypeInfoCount",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "GetTypeInfo",
        arity: 3,
        args: &[ComArgKind::U32, ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "GetIDsOfNames",
        arity: 5,
        args: &[
            ComArgKind::GuidPtr,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::U32,
            ComArgKind::OutPtr,
        ],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "Invoke",
        arity: 7,
        args: &[
            ComArgKind::I32,
            ComArgKind::GuidPtr,
            ComArgKind::U32,
            ComArgKind::U32,
            ComArgKind::Ptr,
            ComArgKind::OutPtr,
            ComArgKind::OutPtr,
        ],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IMARSHAL: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "GetUnmarshalClass",
        arity: 6,
        args: &[
            ComArgKind::GuidPtr,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::OutPtr,
        ],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "GetMarshalSizeMax",
        arity: 6,
        args: &[
            ComArgKind::GuidPtr,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::OutPtr,
        ],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "MarshalInterface",
        arity: 6,
        args: &[
            ComArgKind::Ptr,
            ComArgKind::GuidPtr,
            ComArgKind::Ptr,
            ComArgKind::U32,
            ComArgKind::Ptr,
            ComArgKind::U32,
        ],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "UnmarshalInterface",
        arity: 3,
        args: &[ComArgKind::Ptr, ComArgKind::GuidPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "ReleaseMarshalData",
        arity: 1,
        args: &[ComArgKind::Ptr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 8,
        name: "DisconnectObject",
        arity: 1,
        args: &[ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_ISEQUENTIALSTREAM: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "Read",
        arity: 3,
        args: &[ComArgKind::OutPtr, ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "Write",
        arity: 3,
        args: &[ComArgKind::Ptr, ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_ISTREAM: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 5,
        name: "Seek",
        arity: 3,
        args: &[ComArgKind::I64, ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "SetSize",
        arity: 1,
        args: &[ComArgKind::U64],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "CopyTo",
        arity: 4,
        args: &[ComArgKind::Ptr, ComArgKind::U64, ComArgKind::OutPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 8,
        name: "Commit",
        arity: 1,
        args: &[ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 9,
        name: "Revert",
        arity: 0,
        args: &[],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 10,
        name: "LockRegion",
        arity: 3,
        args: &[ComArgKind::U64, ComArgKind::U64, ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 11,
        name: "UnlockRegion",
        arity: 3,
        args: &[ComArgKind::U64, ComArgKind::U64, ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 12,
        name: "Stat",
        arity: 2,
        args: &[ComArgKind::OutPtr, ComArgKind::U32],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 13,
        name: "Clone",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IMALLOC: &[ComSlotDesc] = &[
    ComSlotDesc { slot: 3, name: "Alloc", arity: 1, args: &[ComArgKind::U64], ret: ComReturnKind::U64 },
    ComSlotDesc { slot: 4, name: "Realloc", arity: 2, args: &[ComArgKind::Ptr, ComArgKind::U64], ret: ComReturnKind::U64 },
    ComSlotDesc { slot: 5, name: "Free", arity: 1, args: &[ComArgKind::Ptr], ret: ComReturnKind::Void },
    ComSlotDesc { slot: 6, name: "GetSize", arity: 1, args: &[ComArgKind::Ptr], ret: ComReturnKind::U64 },
    ComSlotDesc { slot: 7, name: "DidAlloc", arity: 1, args: &[ComArgKind::Ptr], ret: ComReturnKind::U32 },
    ComSlotDesc { slot: 8, name: "HeapMinimize", arity: 0, args: &[], ret: ComReturnKind::Void },
];

const SLOTS_IURI_FACTORY: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 6,
        name: "CreateUri",
        arity: 2,
        args: &[ComArgKind::HString, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "CreateWithRelativeUri",
        arity: 3,
        args: &[ComArgKind::HString, ComArgKind::HString, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

const SLOTS_IURI: &[ComSlotDesc] = &[
    ComSlotDesc { slot: 6, name: "get_AbsoluteUri", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 7, name: "get_DisplayUri", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 8, name: "get_Domain", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 9, name: "get_Extension", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 10, name: "get_Fragment", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 11, name: "get_Host", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 12, name: "get_Password", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 13, name: "get_Path", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 14, name: "get_Query", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 15, name: "get_QueryParsed", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 16, name: "get_RawUri", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 17, name: "get_SchemeName", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 18, name: "get_UserName", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 19, name: "get_Port", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 20, name: "get_Suspicious", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 21, name: "Equals", arity: 2, args: &[ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 22, name: "CombineUri", arity: 2, args: &[ComArgKind::HString, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
];

const SLOTS_IPROPERTYVALUE: &[ComSlotDesc] = &[
    ComSlotDesc { slot: 6, name: "get_Type", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 7, name: "get_IsNumericScalar", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 8, name: "GetUInt8", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 9, name: "GetInt16", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 10, name: "GetUInt16", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 11, name: "GetInt32", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 12, name: "GetUInt32", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 13, name: "GetInt64", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 14, name: "GetUInt64", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 15, name: "GetSingle", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 16, name: "GetDouble", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 17, name: "GetChar16", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 18, name: "GetBoolean", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 19, name: "GetString", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 20, name: "GetGuid", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 21, name: "GetDateTime", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 22, name: "GetTimeSpan", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 23, name: "GetPoint", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 24, name: "GetSize", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 25, name: "GetRect", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 26, name: "GetUInt8Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 27, name: "GetInt16Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 28, name: "GetUInt16Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 29, name: "GetInt32Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 30, name: "GetUInt32Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 31, name: "GetInt64Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 32, name: "GetUInt64Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 33, name: "GetSingleArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 34, name: "GetDoubleArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 35, name: "GetChar16Array", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 36, name: "GetBooleanArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 37, name: "GetStringArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 38, name: "GetInspectableArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 39, name: "GetGuidArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 40, name: "GetDateTimeArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 41, name: "GetTimeSpanArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 42, name: "GetPointArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 43, name: "GetSizeArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 44, name: "GetRectArray", arity: 2, args: &[ComArgKind::OutPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
];

const SLOTS_IPROPERTYVALUESTATICS: &[ComSlotDesc] = &[
    ComSlotDesc { slot: 6, name: "CreateEmpty", arity: 1, args: &[ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 7, name: "CreateUInt8", arity: 2, args: &[ComArgKind::U32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 8, name: "CreateInt16", arity: 2, args: &[ComArgKind::I32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 9, name: "CreateUInt16", arity: 2, args: &[ComArgKind::U32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 10, name: "CreateInt32", arity: 2, args: &[ComArgKind::I32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 11, name: "CreateUInt32", arity: 2, args: &[ComArgKind::U32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 12, name: "CreateInt64", arity: 2, args: &[ComArgKind::I64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 13, name: "CreateUInt64", arity: 2, args: &[ComArgKind::U64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 14, name: "CreateSingle", arity: 2, args: &[ComArgKind::U32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 15, name: "CreateDouble", arity: 2, args: &[ComArgKind::U64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 16, name: "CreateChar16", arity: 2, args: &[ComArgKind::U32, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 17, name: "CreateBoolean", arity: 2, args: &[ComArgKind::Bool, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 18, name: "CreateString", arity: 2, args: &[ComArgKind::HString, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 19, name: "CreateInspectable", arity: 2, args: &[ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 20, name: "CreateGuid", arity: 2, args: &[ComArgKind::GuidPtr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 21, name: "CreateDateTime", arity: 2, args: &[ComArgKind::I64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 22, name: "CreateTimeSpan", arity: 2, args: &[ComArgKind::I64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 23, name: "CreatePoint", arity: 2, args: &[ComArgKind::U64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 24, name: "CreateSize", arity: 2, args: &[ComArgKind::U64, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 25, name: "CreateRect", arity: 2, args: &[ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 26, name: "CreateUInt8Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 27, name: "CreateInt16Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 28, name: "CreateUInt16Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 29, name: "CreateInt32Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 30, name: "CreateUInt32Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 31, name: "CreateInt64Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 32, name: "CreateUInt64Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 33, name: "CreateSingleArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 34, name: "CreateDoubleArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 35, name: "CreateChar16Array", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 36, name: "CreateBooleanArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 37, name: "CreateStringArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 38, name: "CreateInspectableArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 39, name: "CreateGuidArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 40, name: "CreateDateTimeArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 41, name: "CreateTimeSpanArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 42, name: "CreatePointArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 43, name: "CreateSizeArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
    ComSlotDesc { slot: 44, name: "CreateRectArray", arity: 3, args: &[ComArgKind::U32, ComArgKind::Ptr, ComArgKind::OutPtr], ret: ComReturnKind::HResult },
];

const SLOTS_ISHELLITEM: &[ComSlotDesc] = &[
    ComSlotDesc {
        slot: 3,
        name: "BindToHandler",
        arity: 4,
        args: &[ComArgKind::Ptr, ComArgKind::GuidPtr, ComArgKind::GuidPtr, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 4,
        name: "GetParent",
        arity: 1,
        args: &[ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 5,
        name: "GetDisplayName",
        arity: 2,
        args: &[ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 6,
        name: "GetAttributes",
        arity: 2,
        args: &[ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
    ComSlotDesc {
        slot: 7,
        name: "Compare",
        arity: 3,
        args: &[ComArgKind::Ptr, ComArgKind::U32, ComArgKind::OutPtr],
        ret: ComReturnKind::HResult,
    },
];

fn builtin_interface_descs() -> Vec<ComInterfaceDesc> {
    vec![
        ComInterfaceDesc { iid: IID_IUnknown, name: "IUnknown", base_iid: None, slots: SLOTS_IUNKNOWN, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IClassFactory, name: "IClassFactory", base_iid: Some(IID_IUnknown), slots: SLOTS_ICLASSFACTORY, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IClassFactory2, name: "IClassFactory2", base_iid: Some(IID_IClassFactory), slots: SLOTS_ICLASSFACTORY2, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IMalloc, name: "IMalloc", base_iid: Some(IID_IUnknown), slots: SLOTS_IMALLOC, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IMarshal, name: "IMarshal", base_iid: Some(IID_IUnknown), slots: SLOTS_IMARSHAL, host_vtbl: None },
        ComInterfaceDesc { iid: IID_ISequentialStream, name: "ISequentialStream", base_iid: Some(IID_IUnknown), slots: SLOTS_ISEQUENTIALSTREAM, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IStream, name: "IStream", base_iid: Some(IID_ISequentialStream), slots: SLOTS_ISTREAM, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IAgileObject, name: "IAgileObject", base_iid: Some(IID_IUnknown), slots: &[], host_vtbl: None },
        ComInterfaceDesc { iid: IID_IDispatch, name: "IDispatch", base_iid: Some(IID_IUnknown), slots: SLOTS_IDISPATCH, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IInspectable, name: "IInspectable", base_iid: Some(IID_IUnknown), slots: SLOTS_IINSPECTABLE, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IActivationFactory, name: "IActivationFactory", base_iid: Some(IID_IInspectable), slots: SLOTS_IACTIVATIONFACTORY, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IConnectionPointContainer, name: "IConnectionPointContainer", base_iid: Some(IID_IUnknown), slots: SLOTS_ICONNECTIONPOINTCONTAINER, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IConnectionPoint, name: "IConnectionPoint", base_iid: Some(IID_IUnknown), slots: SLOTS_ICONNECTIONPOINT, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IEnumConnectionPoints, name: "IEnumConnectionPoints", base_iid: Some(IID_IUnknown), slots: SLOTS_IENUM_GENERIC, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IEnumConnections, name: "IEnumConnections", base_iid: Some(IID_IUnknown), slots: SLOTS_IENUM_GENERIC, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IPropertyValue, name: "IPropertyValue", base_iid: Some(IID_IInspectable), slots: SLOTS_IPROPERTYVALUE, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IPropertyValueStatics, name: "IPropertyValueStatics", base_iid: Some(IID_IInspectable), slots: SLOTS_IPROPERTYVALUESTATICS, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IUriRuntimeClass, name: "IUriRuntimeClass", base_iid: Some(IID_IInspectable), slots: SLOTS_IURI, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IUriRuntimeClassFactory, name: "IUriRuntimeClassFactory", base_iid: Some(IID_IInspectable), slots: SLOTS_IURI_FACTORY, host_vtbl: None },
        ComInterfaceDesc { iid: IID_IShellItem, name: "IShellItem", base_iid: Some(IID_IUnknown), slots: SLOTS_ISHELLITEM, host_vtbl: None },
    ]
}

fn interface_registry() -> &'static Mutex<Vec<ComInterfaceDesc>> {
    static REG: OnceLock<Mutex<Vec<ComInterfaceDesc>>> = OnceLock::new();
    REG.get_or_init(|| {
        let builtins = builtin_interface_descs();
        for d in &builtins {
            name_iid(d.iid, d.name);
        }
        Mutex::new(builtins)
    })
}

/// Register (or update) a typed COM/WinRT interface descriptor by IID.
#[allow(dead_code)]
pub fn register_interface(desc: ComInterfaceDesc) {
    name_iid(desc.iid, desc.name);
    let mut reg = interface_registry().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = reg.iter_mut().find(|d| d.iid == desc.iid) {
        let prev_vtbl = existing.host_vtbl;
        *existing = desc;
        if existing.host_vtbl.is_none() {
            existing.host_vtbl = prev_vtbl;
        }
    } else {
        reg.push(desc);
    }
}

/// Register a host COM vtable's function pointers with `VA_DISPATCH` AND associate
/// the vtable address with `iid` in the COM interface registry.
#[allow(dead_code)]
pub unsafe fn register_host_vtable(iid: Guid, vtbl: *const usize, slot_count: usize) {
    if vtbl.is_null() || slot_count == 0 {
        return;
    }
    let slice = std::slice::from_raw_parts(vtbl, slot_count);
    register_vtable_ptrs(slice);
    let mut reg = interface_registry().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(existing) = reg.iter_mut().find(|d| d.iid == iid) {
        existing.host_vtbl = Some(vtbl as usize);
    } else {
        reg.push(ComInterfaceDesc {
            iid,
            name: iid_name(&iid).unwrap_or("UnknownCOM"),
            base_iid: Some(IID_IUnknown),
            slots: &[],
            host_vtbl: Some(vtbl as usize),
        });
    }
}

/// Look up a COM interface descriptor by IID.
#[allow(dead_code)]
pub fn lookup_interface(iid: &Guid) -> Option<ComInterfaceDesc> {
    interface_registry()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|d| d.iid == *iid)
        .copied()
}

/// Look up a COM interface descriptor by its registered host vtable pointer.
#[allow(dead_code)]
pub fn lookup_interface_by_host_vtbl(vtbl: *const usize) -> Option<ComInterfaceDesc> {
    let addr = vtbl as usize;
    if addr == 0 {
        return None;
    }
    interface_registry()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|d| d.host_vtbl == Some(addr))
        .copied()
}

/// Resolve a COM vtable slot descriptor by `(iid, slot)`, walking `base_iid` inheritance.
#[allow(dead_code)]
pub fn lookup_slot(iid: &Guid, slot: usize) -> Option<ComSlotDesc> {
    let reg = interface_registry().lock().unwrap_or_else(|e| e.into_inner());
    let mut cur = Some(*iid);
    for _ in 0..16 {
        let g = cur?;
        let desc = reg.iter().find(|d| d.iid == g)?;
        if let Some(s) = desc.slots.iter().find(|s| s.slot == slot) {
            return Some(*s);
        }
        cur = desc.base_iid;
    }
    None
}

/// Resolve a COM vtable slot descriptor by `(iid, method_name)`, walking `base_iid` inheritance.
#[allow(dead_code)]
pub fn lookup_slot_by_name(iid: &Guid, method_name: &str) -> Option<ComSlotDesc> {
    let reg = interface_registry().lock().unwrap_or_else(|e| e.into_inner());
    let mut cur = Some(*iid);
    for _ in 0..16 {
        let g = cur?;
        let desc = reg.iter().find(|d| d.iid == g)?;
        if let Some(s) = desc.slots.iter().find(|s| s.name == method_name) {
            return Some(*s);
        }
        cur = desc.base_iid;
    }
    None
}

/// Invoke a typed COM interface vtable slot on a guest or host object, validating
/// interface/slot registration and argument arity before calling through `call_guest8`.
/// Always reads vtable and slot pointers with `read_unaligned`.
#[allow(dead_code)]
pub unsafe fn com_invoke_interface_slot(
    obj: *mut c_void,
    iid: &Guid,
    slot: usize,
    args: &[u64],
) -> Result<u64, HRESULT> {
    if obj.is_null() {
        return Err(E_POINTER);
    }
    let Some(slot_desc) = lookup_slot(iid, slot) else {
        return Err(if lookup_interface(iid).is_some() { E_NOTIMPL } else { E_NOINTERFACE });
    };
    if args.len() != slot_desc.arity || slot_desc.arity > 7 {
        return Err(E_INVALIDARG);
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return Err(E_POINTER);
    }
    let method = std::ptr::read_unaligned(vtbl.add(slot));
    if method == 0 {
        return Err(E_NOTIMPL);
    }
    let a = |i: usize| args.get(i).copied().unwrap_or(0);
    Ok(crate::call_guest8(
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

/// Invoke a typed COM interface method by name on a guest or host object.
#[allow(dead_code)]
pub unsafe fn com_invoke_interface_method(
    obj: *mut c_void,
    iid: &Guid,
    method_name: &str,
    args: &[u64],
) -> Result<u64, HRESULT> {
    let Some(slot_desc) = lookup_slot_by_name(iid, method_name) else {
        return Err(if lookup_interface(iid).is_some() { E_NOTIMPL } else { E_NOINTERFACE });
    };
    com_invoke_interface_slot(obj, iid, slot_desc.slot, args)
}

/// Convenience wrapper for `HRESULT`-returning COM slots via [`com_invoke_interface_slot`].
#[allow(dead_code)]
pub unsafe fn com_invoke_hresult(
    obj: *mut c_void,
    iid: &Guid,
    slot: usize,
    args: &[u64],
) -> HRESULT {
    match com_invoke_interface_slot(obj, iid, slot, args) {
        Ok(raw) => raw as HRESULT,
        Err(hr) => hr,
    }
}

// ---------------------------------------------------------------------------
// Reusable Host `IConnectionPointContainer` / `IConnectionPoint` for guest sinks
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConnectData {
    pub punk: *mut c_void,
    pub cookie: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<ConnectData>() == 16);

#[allow(dead_code)]
pub struct ConnectionPointCore {
    sink_iid: Guid,
    next_cookie: AtomicU32,
    sinks: Mutex<Vec<(u32, std::sync::Arc<ComPtr>)>>,
}

#[allow(dead_code)]
impl ConnectionPointCore {
    pub fn new(sink_iid: Guid) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            sink_iid,
            next_cookie: AtomicU32::new(1),
            sinks: Mutex::new(Vec::new()),
        })
    }

    pub fn sink_iid(&self) -> Guid {
        self.sink_iid
    }

    pub fn sink_count(&self) -> usize {
        self.sinks.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// Snapshot currently advised guest sinks under lock, drop the lock, and
    /// invoke `slot` on each sink through [`com_invoke_interface_slot`].
    pub unsafe fn broadcast_slot(&self, slot: usize, args: &[u64]) -> Vec<(u32, Result<u64, HRESULT>)> {
        let snapshot: Vec<(u32, std::sync::Arc<ComPtr>)> = {
            let g = self.sinks.lock().unwrap_or_else(|e| e.into_inner());
            g.iter().map(|(c, p)| (*c, std::sync::Arc::clone(p))).collect()
        };
        let mut results = Vec::with_capacity(snapshot.len());
        for (cookie, sink) in snapshot {
            let r = com_invoke_interface_slot(sink.as_ptr(), &self.sink_iid, slot, args);
            results.push((cookie, r));
        }
        results
    }
}

struct CpInner {
    core: std::sync::Arc<ConnectionPointCore>,
    container: Option<std::sync::Arc<CpcCore>>,
}

struct CpcCore {
    points: Vec<std::sync::Arc<ConnectionPointCore>>,
}

struct EnumConnInner {
    items: Vec<(u32, std::sync::Arc<ComPtr>)>,
    cursor: Mutex<usize>,
}

struct EnumCpInner {
    points: Vec<std::sync::Arc<ConnectionPointCore>>,
    container: std::sync::Arc<CpcCore>,
    cursor: Mutex<usize>,
}

unsafe extern "win64" fn enum_conn_qi(
    this: *mut ComObj<EnumConnInner>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IEnumConnections])
}

unsafe extern "win64" fn enum_conn_next(
    this: *mut ComObj<EnumConnInner>,
    c_conn: u32,
    rgcd: *mut ConnectData,
    pc_fetched: *mut u32,
) -> HRESULT {
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, 0);
    }
    if c_conn > 0 && rgcd.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    let slice: Vec<(u32, std::sync::Arc<ComPtr>)> = {
        let mut cur = inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
        let avail = inner.items.len().saturating_sub(*cur);
        let take = (c_conn as usize).min(avail);
        let out: Vec<_> = inner.items[*cur..*cur + take]
            .iter()
            .map(|(c, p)| (*c, std::sync::Arc::clone(p)))
            .collect();
        *cur += take;
        out
    };
    for (i, (cookie, ptr)) in slice.iter().enumerate() {
        com_call_add_ref(ptr.as_ptr());
        std::ptr::write_unaligned(
            rgcd.add(i),
            ConnectData {
                punk: ptr.as_ptr(),
                cookie: *cookie,
                _pad: 0,
            },
        );
    }
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, slice.len() as u32);
    }
    if slice.len() == c_conn as usize { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn enum_conn_skip(this: *mut ComObj<EnumConnInner>, c_conn: u32) -> HRESULT {
    let inner = ComObj::inner(this);
    let mut cur = inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
    let target = cur.saturating_add(c_conn as usize);
    if target <= inner.items.len() {
        *cur = target;
        S_OK
    } else {
        *cur = inner.items.len();
        S_FALSE
    }
}

unsafe extern "win64" fn enum_conn_reset(this: *mut ComObj<EnumConnInner>) -> HRESULT {
    *ComObj::inner(this).cursor.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    S_OK
}

unsafe extern "win64" fn enum_conn_clone(
    this: *mut ComObj<EnumConnInner>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    let cur = *inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
    let cloned = ComObj::new(
        enum_conn_vtbl(),
        EnumConnInner {
            items: inner.items.clone(),
            cursor: Mutex::new(cur),
        },
    );
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

fn enum_conn_vtbl() -> *const usize {
    static VT: OnceLock<[usize; 7]> = OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                enum_conn_qi as *const () as usize,
                com_add_ref::<EnumConnInner> as *const () as usize,
                com_release::<EnumConnInner> as *const () as usize,
                enum_conn_next as *const () as usize,
                enum_conn_skip as *const () as usize,
                enum_conn_reset as *const () as usize,
                enum_conn_clone as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { register_host_vtable(IID_IEnumConnections, ptr, 7) };
    ptr
}

unsafe extern "win64" fn cp_qi(
    this: *mut ComObj<CpInner>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IConnectionPoint])
}

unsafe extern "win64" fn cp_get_connection_interface(
    this: *mut ComObj<CpInner>,
    p_iid: *mut Guid,
) -> HRESULT {
    if p_iid.is_null() {
        return E_POINTER;
    }
    write_guid(p_iid, ComObj::inner(this).core.sink_iid);
    S_OK
}

unsafe extern "win64" fn cp_get_cpc(
    this: *mut ComObj<CpInner>,
    pp_cpc: *mut *mut c_void,
) -> HRESULT {
    if pp_cpc.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_cpc);
    let Some(cpc) = &ComObj::inner(this).container else {
        return E_NOINTERFACE;
    };
    let obj = ComObj::new(cpc_vtbl(), std::sync::Arc::clone(cpc));
    std::ptr::write_unaligned(pp_cpc, obj as *mut c_void);
    S_OK
}

unsafe extern "win64" fn cp_advise(
    this: *mut ComObj<CpInner>,
    p_unk_sink: *mut c_void,
    pdw_cookie: *mut u32,
) -> HRESULT {
    if pdw_cookie.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pdw_cookie, 0);
    if p_unk_sink.is_null() {
        return E_POINTER;
    }
    let core = &ComObj::inner(this).core;
    // QueryInterface on guest sink OUTSIDE any lock.
    let mut sink_iface: *mut c_void = std::ptr::null_mut();
    let hr = com_call_qi(p_unk_sink, &core.sink_iid, &mut sink_iface);
    if hr < 0 || sink_iface.is_null() {
        return CONNECT_E_CANNOTCONNECT;
    }
    let Some(sink_ptr) = ComPtr::from_raw(sink_iface) else {
        return CONNECT_E_CANNOTCONNECT;
    };
    let cookie = core.next_cookie.fetch_add(1, Ordering::Relaxed);
    core.sinks
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push((cookie, std::sync::Arc::new(sink_ptr)));
    std::ptr::write_unaligned(pdw_cookie, cookie);
    S_OK
}

unsafe extern "win64" fn cp_unadvise(this: *mut ComObj<CpInner>, dw_cookie: u32) -> HRESULT {
    let core = &ComObj::inner(this).core;
    let removed = {
        let mut sinks = core.sinks.lock().unwrap_or_else(|e| e.into_inner());
        sinks
            .iter()
            .position(|(c, _)| *c == dw_cookie)
            .map(|i| sinks.remove(i))
    };
    // Drop the removed Arc<ComPtr> (invoking guest Release) strictly OUTSIDE the lock.
    match removed {
        Some(entry) => {
            drop(entry);
            S_OK
        }
        None => CONNECT_E_NOCONNECTION,
    }
}

unsafe extern "win64" fn cp_enum_connections(
    this: *mut ComObj<CpInner>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_POINTER;
    }
    let items: Vec<(u32, std::sync::Arc<ComPtr>)> = {
        let g = ComObj::inner(this).core.sinks.lock().unwrap_or_else(|e| e.into_inner());
        g.iter().map(|(c, p)| (*c, std::sync::Arc::clone(p))).collect()
    };
    let obj = ComObj::new(enum_conn_vtbl(), EnumConnInner { items, cursor: Mutex::new(0) });
    std::ptr::write_unaligned(pp_enum, obj as *mut c_void);
    S_OK
}

fn cp_vtbl() -> *const usize {
    static VT: OnceLock<[usize; 8]> = OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                cp_qi as *const () as usize,
                com_add_ref::<CpInner> as *const () as usize,
                com_release::<CpInner> as *const () as usize,
                cp_get_connection_interface as *const () as usize,
                cp_get_cpc as *const () as usize,
                cp_advise as *const () as usize,
                cp_unadvise as *const () as usize,
                cp_enum_connections as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { register_host_vtable(IID_IConnectionPoint, ptr, 8) };
    ptr
}

unsafe extern "win64" fn enum_cp_qi(
    this: *mut ComObj<EnumCpInner>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IEnumConnectionPoints])
}

unsafe extern "win64" fn enum_cp_next(
    this: *mut ComObj<EnumCpInner>,
    c_conn: u32,
    pp_cp: *mut *mut c_void,
    pc_fetched: *mut u32,
) -> HRESULT {
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, 0);
    }
    if c_conn > 0 && pp_cp.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    let mut cur = inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
    let avail = inner.points.len().saturating_sub(*cur);
    let take = (c_conn as usize).min(avail);
    for i in 0..take {
        let obj = ComObj::new(
            cp_vtbl(),
            CpInner {
                core: std::sync::Arc::clone(&inner.points[*cur + i]),
                container: Some(std::sync::Arc::clone(&inner.container)),
            },
        );
        std::ptr::write_unaligned(pp_cp.add(i), obj as *mut c_void);
    }
    *cur += take;
    if !pc_fetched.is_null() {
        std::ptr::write_unaligned(pc_fetched, take as u32);
    }
    if take == c_conn as usize { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn enum_cp_skip(this: *mut ComObj<EnumCpInner>, c_conn: u32) -> HRESULT {
    let inner = ComObj::inner(this);
    let mut cur = inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
    let target = cur.saturating_add(c_conn as usize);
    if target <= inner.points.len() {
        *cur = target;
        S_OK
    } else {
        *cur = inner.points.len();
        S_FALSE
    }
}

unsafe extern "win64" fn enum_cp_reset(this: *mut ComObj<EnumCpInner>) -> HRESULT {
    *ComObj::inner(this).cursor.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    S_OK
}

unsafe extern "win64" fn enum_cp_clone(
    this: *mut ComObj<EnumCpInner>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    let cur = *inner.cursor.lock().unwrap_or_else(|e| e.into_inner());
    let cloned = ComObj::new(
        enum_cp_vtbl(),
        EnumCpInner {
            points: inner.points.clone(),
            container: std::sync::Arc::clone(&inner.container),
            cursor: Mutex::new(cur),
        },
    );
    std::ptr::write_unaligned(pp_enum, cloned as *mut c_void);
    S_OK
}

fn enum_cp_vtbl() -> *const usize {
    static VT: OnceLock<[usize; 7]> = OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                enum_cp_qi as *const () as usize,
                com_add_ref::<EnumCpInner> as *const () as usize,
                com_release::<EnumCpInner> as *const () as usize,
                enum_cp_next as *const () as usize,
                enum_cp_skip as *const () as usize,
                enum_cp_reset as *const () as usize,
                enum_cp_clone as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { register_host_vtable(IID_IEnumConnectionPoints, ptr, 7) };
    ptr
}

unsafe extern "win64" fn cpc_qi(
    this: *mut ComObj<std::sync::Arc<CpcCore>>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IConnectionPointContainer])
}

unsafe extern "win64" fn cpc_enum_cps(
    this: *mut ComObj<std::sync::Arc<CpcCore>>,
    pp_enum: *mut *mut c_void,
) -> HRESULT {
    if pp_enum.is_null() {
        return E_POINTER;
    }
    let core = ComObj::inner(this);
    let obj = ComObj::new(
        enum_cp_vtbl(),
        EnumCpInner {
            points: core.points.clone(),
            container: std::sync::Arc::clone(core),
            cursor: Mutex::new(0),
        },
    );
    std::ptr::write_unaligned(pp_enum, obj as *mut c_void);
    S_OK
}

unsafe extern "win64" fn cpc_find_cp(
    this: *mut ComObj<std::sync::Arc<CpcCore>>,
    riid: *const Guid,
    pp_cp: *mut *mut c_void,
) -> HRESULT {
    if pp_cp.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_cp);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let core = ComObj::inner(this);
    let Some(cp) = core.points.iter().find(|p| p.sink_iid == iid) else {
        return CONNECT_E_NOCONNECTION;
    };
    let obj = ComObj::new(
        cp_vtbl(),
        CpInner {
            core: std::sync::Arc::clone(cp),
            container: Some(std::sync::Arc::clone(core)),
        },
    );
    std::ptr::write_unaligned(pp_cp, obj as *mut c_void);
    S_OK
}

fn cpc_vtbl() -> *const usize {
    static VT: OnceLock<[usize; 5]> = OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                cpc_qi as *const () as usize,
                com_add_ref::<std::sync::Arc<CpcCore>> as *const () as usize,
                com_release::<std::sync::Arc<CpcCore>> as *const () as usize,
                cpc_enum_cps as *const () as usize,
                cpc_find_cp as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { register_host_vtable(IID_IConnectionPointContainer, ptr, 5) };
    ptr
}

/// Create a host `IConnectionPointContainer` COM object exposing the given connection points.
#[allow(dead_code)]
pub fn create_connection_point_container(
    points: Vec<std::sync::Arc<ConnectionPointCore>>,
) -> *mut c_void {
    let core = std::sync::Arc::new(CpcCore { points });
    ComObj::new(cpc_vtbl(), core) as *mut c_void
}

/// Create a standalone host `IConnectionPoint` COM object backed by `core`.
#[allow(dead_code)]
pub fn create_connection_point(core: std::sync::Arc<ConnectionPointCore>) -> *mut c_void {
    ComObj::new(cp_vtbl(), CpInner { core, container: None }) as *mut c_void
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

    #[test]
    fn typed_interface_registry_and_connection_point_guest_sink_dispatch() {
        const IID_TEST_EVENT_SINK: Guid = guid("11223344-5566-7788-99aa-bbccddeeff00");
        const SINK_SLOTS: &[ComSlotDesc] = &[ComSlotDesc {
            slot: 3,
            name: "OnNotify",
            arity: 2,
            args: &[ComArgKind::U32, ComArgKind::U64],
            ret: ComReturnKind::HResult,
        }];

        register_interface(ComInterfaceDesc {
            iid: IID_TEST_EVENT_SINK,
            name: "ITestEventSink",
            base_iid: Some(IID_IUnknown),
            slots: SINK_SLOTS,
            host_vtbl: None,
        });

        // Verify IID + slot lookup (both own slot and inherited IUnknown slots).
        assert_eq!(lookup_interface(&IID_TEST_EVENT_SINK).unwrap().name, "ITestEventSink");
        assert_eq!(lookup_slot(&IID_TEST_EVENT_SINK, 0).unwrap().name, "QueryInterface");
        assert_eq!(lookup_slot(&IID_TEST_EVENT_SINK, 3).unwrap().name, "OnNotify");
        assert_eq!(lookup_slot_by_name(&IID_TEST_EVENT_SINK, "OnNotify").unwrap().slot, 3);
        assert_eq!(lookup_slot(&IID_IActivationFactory, 6).unwrap().name, "ActivateInstance");
        assert_eq!(lookup_slot(&IID_IActivationFactory, 4).unwrap().name, "GetRuntimeClassName");

        // Build a guest-style COM sink with vtable + refcount + callback log.
        #[repr(C)]
        struct GuestSink {
            vtbl: *const usize,
            refs: AtomicU32,
            events: Mutex<Vec<(u32, u64)>>,
        }

        unsafe extern "win64" fn sink_qi(
            this: *mut GuestSink,
            riid: *const Guid,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            if ppv.is_null() {
                return E_POINTER;
            }
            write_null_out(ppv);
            let Some(g) = read_guid(riid) else {
                return E_POINTER;
            };
            if g == IID_IUnknown || g == IID_TEST_EVENT_SINK {
                (*this).refs.fetch_add(1, Ordering::AcqRel);
                std::ptr::write_unaligned(ppv, this as *mut c_void);
                S_OK
            } else {
                E_NOINTERFACE
            }
        }

        unsafe extern "win64" fn sink_add_ref(this: *mut GuestSink) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }

        unsafe extern "win64" fn sink_release(this: *mut GuestSink) -> u32 {
            (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1
        }

        unsafe extern "win64" fn sink_on_notify(
            this: *mut GuestSink,
            code: u32,
            payload: u64,
        ) -> HRESULT {
            (*this).events.lock().unwrap().push((code, payload));
            S_OK
        }

        let sink_vtbl: [usize; 4] = [
            sink_qi as *const () as usize,
            sink_add_ref as *const () as usize,
            sink_release as *const () as usize,
            sink_on_notify as *const () as usize,
        ];
        let mut sink = GuestSink {
            vtbl: sink_vtbl.as_ptr(),
            refs: AtomicU32::new(1),
            events: Mutex::new(Vec::new()),
        };
        let sink_ptr = (&mut sink as *mut GuestSink) as *mut c_void;

        unsafe {
            // Arity validation rejects wrong argument counts cleanly.
            assert_eq!(
                com_invoke_interface_slot(sink_ptr, &IID_TEST_EVENT_SINK, 3, &[1]),
                Err(E_INVALIDARG)
            );
            assert_eq!(
                com_invoke_interface_slot(sink_ptr, &IID_TEST_EVENT_SINK, 99, &[]),
                Err(E_NOTIMPL)
            );

            // Direct typed method invocation via call_guest8.
            assert_eq!(
                com_invoke_interface_method(sink_ptr, &IID_TEST_EVENT_SINK, "OnNotify", &[7, 0xCAFE]),
                Ok(S_OK as u64)
            );
            assert_eq!(sink.events.lock().unwrap().clone(), vec![(7, 0xCAFE)]);

            // Host IConnectionPointContainer -> FindConnectionPoint -> Advise -> broadcast -> Unadvise.
            let cp_core = ConnectionPointCore::new(IID_TEST_EVENT_SINK);
            let cpc = create_connection_point_container(vec![std::sync::Arc::clone(&cp_core)]);
            assert_eq!(
                lookup_interface_by_host_vtbl(cpc_vtbl()).unwrap().iid,
                IID_IConnectionPointContainer
            );

            // Unknown sink IID -> CONNECT_E_NOCONNECTION.
            let mut bad_cp: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                com_invoke_hresult(
                    cpc,
                    &IID_IConnectionPointContainer,
                    4,
                    &[&IID_TEST_UNKNOWN as *const Guid as u64, &mut bad_cp as *mut *mut c_void as u64]
                ),
                CONNECT_E_NOCONNECTION
            );

            // FindConnectionPoint for IID_TEST_EVENT_SINK.
            let mut cp_iface: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                com_invoke_hresult(
                    cpc,
                    &IID_IConnectionPointContainer,
                    4,
                    &[&IID_TEST_EVENT_SINK as *const Guid as u64, &mut cp_iface as *mut *mut c_void as u64]
                ),
                S_OK
            );
            assert!(!cp_iface.is_null());

            // Advise guest sink -> QI adds ref (1 -> 2), returns cookie.
            let mut cookie = 0u32;
            assert_eq!(
                com_invoke_hresult(
                    cp_iface,
                    &IID_IConnectionPoint,
                    5,
                    &[sink_ptr as u64, &mut cookie as *mut u32 as u64]
                ),
                S_OK
            );
            assert_ne!(cookie, 0);
            assert_eq!(sink.refs.load(Ordering::Acquire), 2);
            assert_eq!(cp_core.sink_count(), 1);

            // Broadcast invokes guest sink slot 3 with locks dropped.
            let res = cp_core.broadcast_slot(3, &[42, 0x1234_5678]);
            assert_eq!(res, vec![(cookie, Ok(S_OK as u64))]);
            assert_eq!(
                sink.events.lock().unwrap().clone(),
                vec![(7, 0xCAFE), (42, 0x1234_5678)]
            );

            // EnumConnections yields our advised sink.
            let mut enum_conn: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                com_invoke_hresult(
                    cp_iface,
                    &IID_IConnectionPoint,
                    7,
                    &[&mut enum_conn as *mut *mut c_void as u64]
                ),
                S_OK
            );
            let mut cd = ConnectData { punk: std::ptr::null_mut(), cookie: 0, _pad: 0 };
            let mut fetched = 0u32;
            assert_eq!(
                com_invoke_hresult(
                    enum_conn,
                    &IID_IEnumConnections,
                    3,
                    &[1, &mut cd as *mut ConnectData as u64, &mut fetched as *mut u32 as u64]
                ),
                S_OK
            );
            assert_eq!(fetched, 1);
            assert_eq!(cd.cookie, cookie);
            assert_eq!(cd.punk, sink_ptr);
            assert_eq!(com_call_release(cd.punk), 2);
            assert_eq!(com_call_release(enum_conn), 0);

            // Unadvise releases sink ref back to 1.
            assert_eq!(
                com_invoke_hresult(cp_iface, &IID_IConnectionPoint, 6, &[cookie as u64]),
                S_OK
            );
            assert_eq!(sink.refs.load(Ordering::Acquire), 1);
            assert_eq!(
                com_invoke_hresult(cp_iface, &IID_IConnectionPoint, 6, &[cookie as u64]),
                CONNECT_E_NOCONNECTION
            );

            assert_eq!(com_call_release(cp_iface), 0);
            assert_eq!(com_call_release(cpc), 0);
        }
    }

    #[test]
    fn register_vtbl_struct_serializes_concurrent_callers_until_complete() {
        use std::sync::atomic::AtomicUsize;

        #[repr(C)]
        struct FakeVtbl {
            m0: usize,
            m1: usize,
            m2: usize,
        }
        static FAKE_VTBL: FakeVtbl = FakeVtbl {
            m0: 0x1001_0000,
            m1: 0x1002_0000,
            m2: 0x1003_0000,
        };
        static REG_COUNT: AtomicUsize = AtomicUsize::new(0);
        static PREV_REGISTRAR: AtomicUsize = AtomicUsize::new(0);

        fn slow_registrar(va: u64, ptr: usize) {
            if matches!(va, 0x1001_0000 | 0x1002_0000 | 0x1003_0000) {
                std::thread::sleep(std::time::Duration::from_millis(5));
                REG_COUNT.fetch_add(1, Ordering::Release);
            }
            let prev = PREV_REGISTRAR.load(Ordering::Acquire);
            if prev != 0 {
                let f: crate::NativeVaRegistrarFn = unsafe { std::mem::transmute(prev) };
                f(va, ptr);
            }
        }

        let prev = crate::NATIVE_VA_REGISTRAR
            .swap(slow_registrar as *const () as usize, Ordering::AcqRel);
        PREV_REGISTRAR.store(prev, Ordering::Release);
        struct RestoreGuard(usize);
        impl Drop for RestoreGuard {
            fn drop(&mut self) {
                crate::NATIVE_VA_REGISTRAR.store(self.0, Ordering::Release);
            }
        }
        let _restore = RestoreGuard(prev);

        let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let b = barrier.clone();
            handles.push(std::thread::spawn(move || {
                b.wait();
                register_vtbl_struct(&FAKE_VTBL);
                // Every returning thread must observe all 3 slots fully registered.
                assert_eq!(REG_COUNT.load(Ordering::Acquire), 3);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        drop(_restore);
        assert_eq!(REG_COUNT.load(Ordering::Acquire), 3);
    }
}
