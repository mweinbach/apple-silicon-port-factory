//! `IMFAttributes` and `IMFMediaType` COM implementations for `mfplat.dll`.
//!
//! Shares a single `#[repr(C)]` [`AttrHeader`] across `IMFAttributes`,
//! `IMFMediaType`, and `IMFSample`, backed by a reentrant-locked
//! ([`crate::sync::ReentrantLock`]) GUID key-value store with full
//! `PROPVARIANT` round-trip support.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::com::{
    com_call_add_ref, com_call_qi, com_call_release, guid, read_guid, register_vtable,
    write_null_out, Guid, IID_IUnknown,
};
use crate::misc::ole32::{CoTaskMemAlloc, CoTaskMemFree};
use crate::strings::{read_wide_units_z, DEFAULT_MAX_STR_LEN};
use crate::sync::ReentrantLock;
use crate::types::{
    BOOL, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, FALSE,
    HRESULT, S_FALSE, S_OK, TRUE,
};

use super::{
    IID_IMFAttributes, IID_IMFMediaType, MF_ATTRIBUTES_MATCH_ALL_ITEMS,
    MF_ATTRIBUTES_MATCH_INTERSECTION, MF_ATTRIBUTES_MATCH_OUR_ITEMS, MF_ATTRIBUTES_MATCH_SMALLER,
    MF_ATTRIBUTES_MATCH_THEIR_ITEMS, MF_ATTRIBUTE_BLOB, MF_ATTRIBUTE_DOUBLE, MF_ATTRIBUTE_GUID,
    MF_ATTRIBUTE_IUNKNOWN, MF_ATTRIBUTE_STRING, MF_ATTRIBUTE_UINT32, MF_ATTRIBUTE_UINT64,
    MF_E_ATTRIBUTENOTFOUND, MF_E_BUFFERTOOSMALL, MF_MEDIATYPE_EQUAL_FORMAT_DATA,
    MF_MEDIATYPE_EQUAL_FORMAT_TYPES, MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA,
    MF_MEDIATYPE_EQUAL_MAJOR_TYPES, MF_MT_ALL_SAMPLES_INDEPENDENT, MF_MT_FIXED_SIZE_SAMPLES,
    MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_MT_USER_DATA, STRSAFE_E_INSUFFICIENT_BUFFER,
};

const VT_EMPTY: u16 = 0;
const VT_R8: u16 = 5;
const VT_UNKNOWN: u16 = 13;
const VT_UI4: u16 = 19;
const VT_UI8: u16 = 21;
const VT_LPWSTR: u16 = 31;
const VT_BLOB: u16 = 65;
const VT_CLSID: u16 = 72;
const VT_VECTOR_UI1: u16 = 0x1011; // 4113

// Common uncompressed media subtype GUIDs (for IMFMediaType::IsCompressedFormat)
const MF_AUDIO_FORMAT_PCM: Guid = guid("00000001-0000-0010-8000-00aa00389b71");
const MF_AUDIO_FORMAT_FLOAT: Guid = guid("00000003-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_RGB32: Guid = guid("00000016-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_ARGB32: Guid = guid("00000015-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_RGB24: Guid = guid("00000014-0000-0010-8000-00aa00389b71");
pub(super) const MF_VIDEO_FORMAT_NV12: Guid = guid("3231564e-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_YUY2: Guid = guid("32595559-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_YV12: Guid = guid("32315659-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_IYUV: Guid = guid("56555949-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_I420: Guid = guid("30323449-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_UYVY: Guid = guid("59565955-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_P010: Guid = guid("30313050-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_AYUV: Guid = guid("56555941-0000-0010-8000-00aa00389b71");

#[derive(Clone)]
enum AttrValue {
    Uint32(u32),
    Uint64(u64),
    Double(f64),
    Guid(Guid),
    String(Vec<u16>), // UTF-16 without trailing NUL
    Blob(Vec<u8>),
    Unknown(usize), // AddRef'd COM pointer (or 0 for null)
}

impl AttrValue {
    fn mf_type(&self) -> u32 {
        match self {
            AttrValue::Uint32(_) => MF_ATTRIBUTE_UINT32,
            AttrValue::Uint64(_) => MF_ATTRIBUTE_UINT64,
            AttrValue::Double(_) => MF_ATTRIBUTE_DOUBLE,
            AttrValue::Guid(_) => MF_ATTRIBUTE_GUID,
            AttrValue::String(_) => MF_ATTRIBUTE_STRING,
            AttrValue::Blob(_) => MF_ATTRIBUTE_BLOB,
            AttrValue::Unknown(_) => MF_ATTRIBUTE_IUNKNOWN,
        }
    }

    fn equals(&self, other: &AttrValue) -> bool {
        match (self, other) {
            (AttrValue::Uint32(a), AttrValue::Uint32(b)) => a == b,
            (AttrValue::Uint64(a), AttrValue::Uint64(b)) => a == b,
            (AttrValue::Double(a), AttrValue::Double(b)) => a == b,
            (AttrValue::Guid(a), AttrValue::Guid(b)) => a == b,
            (AttrValue::String(a), AttrValue::String(b)) => a == b,
            (AttrValue::Blob(a), AttrValue::Blob(b)) => a == b,
            (AttrValue::Unknown(a), AttrValue::Unknown(b)) => a == b,
            _ => false,
        }
    }

    unsafe fn release_if_unk(&self) {
        if let AttrValue::Unknown(p) = *self {
            if p != 0 {
                com_call_release(p as *mut c_void);
            }
        }
    }

    unsafe fn clone_retained(&self) -> AttrValue {
        if let AttrValue::Unknown(p) = *self {
            if p != 0 {
                com_call_add_ref(p as *mut c_void);
            }
        }
        self.clone()
    }
}

/// Write an [`AttrValue`] into a 24-byte Win64 `PROPVARIANT` buffer.
unsafe fn write_propvariant(pv: *mut u8, val: &AttrValue) -> HRESULT {
    std::ptr::write_bytes(pv, 0, 24);
    match val {
        AttrValue::Uint32(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_UI4);
            std::ptr::write_unaligned(pv.add(8) as *mut u32, *v);
            S_OK
        }
        AttrValue::Uint64(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_UI8);
            std::ptr::write_unaligned(pv.add(8) as *mut u64, *v);
            S_OK
        }
        AttrValue::Double(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_R8);
            std::ptr::write_unaligned(pv.add(8) as *mut f64, *v);
            S_OK
        }
        AttrValue::Guid(g) => {
            let mem = CoTaskMemAlloc(16) as *mut Guid;
            if mem.is_null() {
                return E_OUTOFMEMORY;
            }
            std::ptr::write_unaligned(mem, *g);
            std::ptr::write_unaligned(pv as *mut u16, VT_CLSID);
            std::ptr::write_unaligned(pv.add(8) as *mut *mut Guid, mem);
            S_OK
        }
        AttrValue::String(units) => {
            let bytes = (units.len() + 1) * 2;
            let mem = CoTaskMemAlloc(bytes) as *mut u16;
            if mem.is_null() {
                return E_OUTOFMEMORY;
            }
            if !units.is_empty() {
                std::ptr::copy_nonoverlapping(units.as_ptr(), mem, units.len());
            }
            std::ptr::write_unaligned(mem.add(units.len()), 0u16);
            std::ptr::write_unaligned(pv as *mut u16, VT_LPWSTR);
            std::ptr::write_unaligned(pv.add(8) as *mut *mut u16, mem);
            S_OK
        }
        AttrValue::Blob(bytes) => {
            let mem = if bytes.is_empty() {
                std::ptr::null_mut()
            } else {
                let p = CoTaskMemAlloc(bytes.len()) as *mut u8;
                if p.is_null() {
                    return E_OUTOFMEMORY;
                }
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
                p
            };
            std::ptr::write_unaligned(pv as *mut u16, VT_VECTOR_UI1);
            std::ptr::write_unaligned(pv.add(8) as *mut u32, bytes.len() as u32);
            std::ptr::write_unaligned(pv.add(16) as *mut *mut u8, mem);
            S_OK
        }
        AttrValue::Unknown(p) => {
            if *p != 0 {
                com_call_add_ref(*p as *mut c_void);
            }
            std::ptr::write_unaligned(pv as *mut u16, VT_UNKNOWN);
            std::ptr::write_unaligned(pv.add(8) as *mut usize, *p);
            S_OK
        }
    }
}

/// Parse a Win64 `PROPVARIANT` (24 bytes) into an [`AttrValue`].
/// When `retain_unknown` is true, `AddRef`s any `VT_UNKNOWN` interface pointer.
unsafe fn read_propvariant(pv: *const u8, retain_unknown: bool) -> Result<AttrValue, HRESULT> {
    if pv.is_null() {
        return Err(E_POINTER);
    }
    let vt = std::ptr::read_unaligned(pv as *const u16);
    match vt {
        VT_UI4 => Ok(AttrValue::Uint32(std::ptr::read_unaligned(pv.add(8) as *const u32))),
        VT_UI8 => Ok(AttrValue::Uint64(std::ptr::read_unaligned(pv.add(8) as *const u64))),
        VT_R8 => Ok(AttrValue::Double(std::ptr::read_unaligned(pv.add(8) as *const f64))),
        VT_CLSID => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *const Guid);
            if p.is_null() {
                return Err(E_INVALIDARG);
            }
            Ok(AttrValue::Guid(std::ptr::read_unaligned(p)))
        }
        VT_LPWSTR => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *const u16);
            if p.is_null() {
                return Err(E_INVALIDARG);
            }
            Ok(AttrValue::String(read_wide_units_z(p, DEFAULT_MAX_STR_LEN)))
        }
        VT_VECTOR_UI1 | VT_BLOB => {
            let len = std::ptr::read_unaligned(pv.add(8) as *const u32) as usize;
            let p = std::ptr::read_unaligned(pv.add(16) as *const *const u8);
            if len > 0 && p.is_null() {
                return Err(E_INVALIDARG);
            }
            let mut v = vec![0u8; len];
            if len > 0 {
                std::ptr::copy_nonoverlapping(p, v.as_mut_ptr(), len);
            }
            Ok(AttrValue::Blob(v))
        }
        VT_UNKNOWN => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const usize);
            if retain_unknown && p != 0 {
                com_call_add_ref(p as *mut c_void);
            }
            Ok(AttrValue::Unknown(p))
        }
        VT_EMPTY => Err(E_INVALIDARG),
        _ => Err(E_INVALIDARG),
    }
}

/// Release any heap/COM payload owned by a temporary `PROPVARIANT`.
pub(super) unsafe fn clear_propvariant_local(pv: *mut u8) {
    let vt = std::ptr::read_unaligned(pv as *const u16);
    match vt {
        VT_CLSID | VT_LPWSTR => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
            if !p.is_null() {
                CoTaskMemFree(p);
            }
        }
        VT_VECTOR_UI1 | VT_BLOB => {
            let p = std::ptr::read_unaligned(pv.add(16) as *const *mut c_void);
            if !p.is_null() {
                CoTaskMemFree(p);
            }
        }
        VT_UNKNOWN => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
            if !p.is_null() {
                com_call_release(p);
            }
        }
        _ => {}
    }
    std::ptr::write_bytes(pv, 0, 24);
}

pub(super) struct AttrStore {
    lock: ReentrantLock,
    items: Mutex<Vec<(Guid, AttrValue)>>,
}

impl AttrStore {
    pub(super) fn new(cap: usize) -> Self {
        Self {
            lock: ReentrantLock::new(),
            items: Mutex::new(Vec::with_capacity(cap.min(256))),
        }
    }

    fn set_item(&self, key: Guid, val: AttrValue) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            if let Some((_, existing)) = items.iter_mut().find(|(k, _)| *k == key) {
                let old = std::mem::replace(existing, val);
                unsafe { old.release_if_unk() };
            } else {
                items.push((key, val));
            }
        });
    }

    fn delete_item(&self, key: &Guid) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            if let Some(idx) = items.iter().position(|(k, _)| k == key) {
                let (_, old) = items.remove(idx);
                unsafe { old.release_if_unk() };
            }
        });
    }

    fn delete_all(&self) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            for (_, old) in items.drain(..) {
                unsafe { old.release_if_unk() };
            }
        });
    }

    fn get_clone(&self, key: &Guid) -> Option<AttrValue> {
        self.lock.with_lock(|| {
            let items = self.items.lock().unwrap();
            items
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| unsafe { v.clone_retained() })
        })
    }

    fn snapshot_keys_and_values(&self) -> Vec<(Guid, AttrValue)> {
        self.lock.with_lock(|| {
            let items = self.items.lock().unwrap();
            items
                .iter()
                .map(|(k, v)| (*k, unsafe { v.clone_retained() }))
                .collect()
        })
    }
}

impl Drop for AttrStore {
    fn drop(&mut self) {
        if let Ok(mut items) = self.items.lock() {
            for (_, v) in items.drain(..) {
                unsafe { v.release_if_unk() };
            }
        }
    }
}

/// Common header for `IMFAttributes`, `IMFMediaType`, and `IMFSample` so all 30
/// attribute vtable slots share one zero-overhead implementation.
#[repr(C)]
pub(super) struct AttrHeader {
    pub(super) vtbl: *const usize,
    pub(super) refs: AtomicU32,
    pub(super) attrs: AttrStore,
}

#[repr(C)]
struct AttrObj {
    hdr: AttrHeader,
}

#[repr(C)]
struct MediaTypeObj {
    hdr: AttrHeader,
}

#[inline]
unsafe fn hdr_ref<'a>(this: usize) -> Option<&'a AttrHeader> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const AttrHeader))
    }
}

fn is_our_attr_vtable(vt: *const usize) -> bool {
    vt == attr_vtable().as_ptr()
        || vt == media_type_vtable().as_ptr()
        || vt == super::sample::sample_vtable().as_ptr()
}

pub(super) unsafe fn qi_attr_common(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
    extra_iid: Option<Guid>,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUnknown || iid == IID_IMFAttributes || extra_iid.is_some_and(|e| e == iid) {
        h.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn attr_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    qi_attr_common(this, riid, ppv, None)
}

unsafe extern "win64" fn mediatype_qi(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    qi_attr_common(this, riid, ppv, Some(IID_IMFMediaType))
}

unsafe extern "win64" fn attr_add_ref(this: usize) -> u32 {
    let Some(h) = hdr_ref(this) else {
        return 0;
    };
    h.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn attr_release_obj<T>(this: usize) -> u32 {
    let Some(h) = hdr_ref(this) else {
        return 0;
    };
    let prev = h.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut T));
    }
    next
}

// ---- IMFAttributes methods (slots 3..32) ----

unsafe extern "win64" fn attr_get_item(this: usize, key: *const Guid, pv: *mut u8) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let hr = if pv.is_null() {
        S_OK
    } else {
        write_propvariant(pv, &v)
    };
    v.release_if_unk();
    hr
}

unsafe extern "win64" fn attr_get_item_type(
    this: usize,
    key: *const Guid,
    out_type: *mut u32,
) -> HRESULT {
    if out_type.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let ty = v.mf_type();
    v.release_if_unk();
    std::ptr::write_unaligned(out_type, ty);
    S_OK
}

unsafe extern "win64" fn attr_compare_item(
    this: usize,
    key: *const Guid,
    value: *const u8,
    result: *mut BOOL,
) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(result, FALSE);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Ok(rhs) = read_propvariant(value, false) else {
        return E_INVALIDARG;
    };
    let Some(lhs) = h.attrs.get_clone(&k) else {
        return S_OK;
    };
    let eq = lhs.equals(&rhs);
    lhs.release_if_unk();
    std::ptr::write_unaligned(result, if eq { TRUE } else { FALSE });
    S_OK
}

/// Read all items from any `IMFAttributes*` (fast-path for ours, COM vtable fallback for foreign).
unsafe fn snapshot_any_attributes(ptr: *mut c_void) -> Result<Vec<(Guid, AttrValue)>, HRESULT> {
    if ptr.is_null() {
        return Err(E_POINTER);
    }
    let vt = std::ptr::read_unaligned(ptr as *const *const usize);
    if is_our_attr_vtable(vt) {
        let h = &*(ptr as *const AttrHeader);
        return Ok(h.attrs.snapshot_keys_and_values());
    }
    let get_count: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(30)));
    let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut Guid, *mut u8) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(31)));
    let mut count = 0u32;
    let hr = get_count(ptr, &mut count);
    if hr < 0 {
        return Err(hr);
    }
    let mut out: Vec<(Guid, AttrValue)> = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut g = Guid::default();
        let mut pv = [0u8; 24];
        let hr = get_by_idx(ptr, i, &mut g, pv.as_mut_ptr());
        if hr < 0 {
            for (_, v) in out {
                v.release_if_unk();
            }
            return Err(hr);
        }
        let parsed = read_propvariant(pv.as_ptr(), true);
        clear_propvariant_local(pv.as_mut_ptr());
        match parsed {
            Ok(v) => out.push((g, v)),
            Err(e) => {
                for (_, v) in out {
                    v.release_if_unk();
                }
                return Err(e);
            }
        }
    }
    Ok(out)
}

fn compare_attr_lists(
    ours: &[(Guid, AttrValue)],
    theirs: &[(Guid, AttrValue)],
    match_type: u32,
) -> Result<bool, HRESULT> {
    let subset_match = |a: &[(Guid, AttrValue)], b: &[(Guid, AttrValue)]| -> bool {
        a.iter()
            .all(|(k, v)| b.iter().find(|(bk, _)| bk == k).is_some_and(|(_, bv)| v.equals(bv)))
    };
    match match_type {
        MF_ATTRIBUTES_MATCH_OUR_ITEMS => Ok(subset_match(ours, theirs)),
        MF_ATTRIBUTES_MATCH_THEIR_ITEMS => Ok(subset_match(theirs, ours)),
        MF_ATTRIBUTES_MATCH_ALL_ITEMS => {
            Ok(ours.len() == theirs.len() && subset_match(ours, theirs))
        }
        MF_ATTRIBUTES_MATCH_INTERSECTION => Ok(ours.iter().all(|(k, v)| {
            theirs
                .iter()
                .find(|(bk, _)| bk == k)
                .is_none_or(|(_, bv)| v.equals(bv))
        })),
        MF_ATTRIBUTES_MATCH_SMALLER => {
            if ours.len() <= theirs.len() {
                Ok(subset_match(ours, theirs))
            } else {
                Ok(subset_match(theirs, ours))
            }
        }
        _ => Err(E_INVALIDARG),
    }
}

unsafe extern "win64" fn attr_compare(
    this: usize,
    theirs: *mut c_void,
    match_type: u32,
    result: *mut BOOL,
) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(result, FALSE);
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if theirs.is_null() {
        return E_INVALIDARG;
    }
    let ours = h.attrs.snapshot_keys_and_values();
    let theirs_snap = match snapshot_any_attributes(theirs) {
        Ok(v) => v,
        Err(hr) => {
            for (_, v) in ours {
                v.release_if_unk();
            }
            return hr;
        }
    };
    let cmp = compare_attr_lists(&ours, &theirs_snap, match_type);
    for (_, v) in ours {
        v.release_if_unk();
    }
    for (_, v) in theirs_snap {
        v.release_if_unk();
    }
    match cmp {
        Ok(b) => {
            std::ptr::write_unaligned(result, if b { TRUE } else { FALSE });
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn attr_get_uint32(this: usize, key: *const Guid, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Uint32(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_uint64(this: usize, key: *const Guid, out: *mut u64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Uint64(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_double(this: usize, key: *const Guid, out: *mut f64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Double(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_guid(this: usize, key: *const Guid, out: *mut Guid) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Guid(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_string_length(
    this: usize,
    key: *const Guid,
    out_len: *mut u32,
) -> HRESULT {
    if out_len.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::String(units) => {
            std::ptr::write_unaligned(out_len, units.len() as u32);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_string(
    this: usize,
    key: *const Guid,
    buf: *mut u16,
    buf_cch: u32,
    out_len: *mut u32,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::String(units) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if !out_len.is_null() {
        std::ptr::write_unaligned(out_len, units.len() as u32);
    }
    if buf.is_null() || (buf_cch as usize) < units.len() + 1 {
        return STRSAFE_E_INSUFFICIENT_BUFFER;
    }
    if !units.is_empty() {
        std::ptr::copy_nonoverlapping(units.as_ptr(), buf, units.len());
    }
    std::ptr::write_unaligned(buf.add(units.len()), 0u16);
    S_OK
}

unsafe extern "win64" fn attr_get_allocated_string(
    this: usize,
    key: *const Guid,
    out_ptr: *mut *mut u16,
    out_len: *mut u32,
) -> HRESULT {
    if out_ptr.is_null() || out_len.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
    std::ptr::write_unaligned(out_len, 0);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::String(units) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let mem = CoTaskMemAlloc((units.len() + 1) * 2) as *mut u16;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    if !units.is_empty() {
        std::ptr::copy_nonoverlapping(units.as_ptr(), mem, units.len());
    }
    std::ptr::write_unaligned(mem.add(units.len()), 0u16);
    std::ptr::write_unaligned(out_ptr, mem);
    std::ptr::write_unaligned(out_len, units.len() as u32);
    S_OK
}

unsafe extern "win64" fn attr_get_blob_size(
    this: usize,
    key: *const Guid,
    out_size: *mut u32,
) -> HRESULT {
    if out_size.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Blob(b) => {
            std::ptr::write_unaligned(out_size, b.len() as u32);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_blob(
    this: usize,
    key: *const Guid,
    buf: *mut u8,
    buf_size: u32,
    out_size: *mut u32,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Blob(bytes) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if !out_size.is_null() {
        std::ptr::write_unaligned(out_size, bytes.len() as u32);
    }
    if (buf_size as usize) < bytes.len() {
        return MF_E_BUFFERTOOSMALL;
    }
    if !bytes.is_empty() {
        if buf.is_null() {
            return E_POINTER;
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
    }
    S_OK
}

unsafe extern "win64" fn attr_get_allocated_blob(
    this: usize,
    key: *const Guid,
    out_buf: *mut *mut u8,
    out_size: *mut u32,
) -> HRESULT {
    if out_buf.is_null() || out_size.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out_buf, std::ptr::null_mut());
    std::ptr::write_unaligned(out_size, 0);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Blob(bytes) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let mem = CoTaskMemAlloc(bytes.len().max(1)) as *mut u8;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem, bytes.len());
    }
    std::ptr::write_unaligned(out_buf, mem);
    std::ptr::write_unaligned(out_size, bytes.len() as u32);
    S_OK
}

unsafe extern "win64" fn attr_get_unknown(
    this: usize,
    key: *const Guid,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    let (Some(h), Some(k), Some(iid)) = (hdr_ref(this), read_guid(key), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Unknown(p) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if p == 0 {
        return E_NOINTERFACE;
    }
    let hr = com_call_qi(p as *mut c_void, &iid, ppv);
    com_call_release(p as *mut c_void);
    hr
}

unsafe extern "win64" fn attr_set_item(this: usize, key: *const Guid, pv: *const u8) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    match read_propvariant(pv, true) {
        Ok(val) => {
            h.attrs.set_item(k, val);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn attr_delete_item(this: usize, key: *const Guid) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.delete_item(&k);
    S_OK
}

unsafe extern "win64" fn attr_delete_all_items(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    h.attrs.delete_all();
    S_OK
}

unsafe extern "win64" fn attr_set_uint32(this: usize, key: *const Guid, value: u32) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Uint32(value));
    S_OK
}

unsafe extern "win64" fn attr_set_uint64(this: usize, key: *const Guid, value: u64) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Uint64(value));
    S_OK
}

pub(super) unsafe extern "win64" fn attr_set_double(
    this: usize,
    key: *const Guid,
    value: f64,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Double(value));
    S_OK
}

// Slot 23 takes `f64` in Win64 slot 3 (`xmm2`) after two integer/pointer slots.
crate::win64_bridge!(fn attr_set_double(usize, *const Guid, f64) -> HRESULT);

unsafe extern "win64" fn attr_set_guid(
    this: usize,
    key: *const Guid,
    value: *const Guid,
) -> HRESULT {
    let (Some(h), Some(k), Some(v)) = (hdr_ref(this), read_guid(key), read_guid(value)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Guid(v));
    S_OK
}

unsafe extern "win64" fn attr_set_string(
    this: usize,
    key: *const Guid,
    value: *const u16,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if value.is_null() {
        return E_INVALIDARG;
    }
    h.attrs
        .set_item(k, AttrValue::String(read_wide_units_z(value, DEFAULT_MAX_STR_LEN)));
    S_OK
}

unsafe extern "win64" fn attr_set_blob(
    this: usize,
    key: *const Guid,
    buf: *const u8,
    size: u32,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if size > 0 && buf.is_null() {
        return E_POINTER;
    }
    let mut data = vec![0u8; size as usize];
    if size > 0 {
        std::ptr::copy_nonoverlapping(buf, data.as_mut_ptr(), size as usize);
    }
    h.attrs.set_item(k, AttrValue::Blob(data));
    S_OK
}

unsafe extern "win64" fn attr_set_unknown(
    this: usize,
    key: *const Guid,
    unk: *mut c_void,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if !unk.is_null() {
        com_call_add_ref(unk);
    }
    h.attrs.set_item(k, AttrValue::Unknown(unk as usize));
    S_OK
}

unsafe extern "win64" fn attr_lock_store(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    h.attrs.lock.lock();
    S_OK
}

unsafe extern "win64" fn attr_unlock_store(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if h.attrs.lock.unlock() {
        S_OK
    } else {
        E_FAIL
    }
}

unsafe extern "win64" fn attr_get_count(this: usize, out_count: *mut u32) -> HRESULT {
    if out_count.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let n = h
        .attrs
        .lock
        .with_lock(|| h.attrs.items.lock().unwrap().len() as u32);
    std::ptr::write_unaligned(out_count, n);
    S_OK
}

unsafe extern "win64" fn attr_get_item_by_index(
    this: usize,
    index: u32,
    out_key: *mut Guid,
    out_pv: *mut u8,
) -> HRESULT {
    if out_key.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let entry = h.attrs.lock.with_lock(|| {
        let items = h.attrs.items.lock().unwrap();
        items
            .get(index as usize)
            .map(|(k, v)| (*k, v.clone_retained()))
    });
    let Some((k, v)) = entry else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out_key, k);
    let hr = if out_pv.is_null() {
        S_OK
    } else {
        write_propvariant(out_pv, &v)
    };
    v.release_if_unk();
    hr
}

unsafe extern "win64" fn attr_copy_all_items(this: usize, dest: *mut c_void) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if dest.is_null() {
        return E_POINTER;
    }
    if dest as usize == this {
        return S_OK;
    }
    let snap = h.attrs.snapshot_keys_and_values();
    let dst_vt = std::ptr::read_unaligned(dest as *const *const usize);
    if is_our_attr_vtable(dst_vt) {
        let dh = &*(dest as *const AttrHeader);
        dh.attrs.delete_all();
        for (k, v) in snap {
            dh.attrs.set_item(k, v);
        }
        return S_OK;
    }
    let del_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dst_vt.add(20)));
    let set_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dst_vt.add(18)));
    let hr = del_all(dest);
    if hr < 0 {
        for (_, v) in snap {
            v.release_if_unk();
        }
        return hr;
    }
    let mut final_hr = S_OK;
    for (k, v) in snap {
        let mut pv = [0u8; 24];
        let whr = write_propvariant(pv.as_mut_ptr(), &v);
        v.release_if_unk();
        if whr < 0 {
            final_hr = whr;
            continue;
        }
        let shr = set_item(dest, &k, pv.as_ptr());
        clear_propvariant_local(pv.as_mut_ptr());
        if shr < 0 && final_hr == S_OK {
            final_hr = shr;
        }
    }
    final_hr
}

pub(super) fn fill_attr_slots<T>(qi: usize) -> [usize; 33] {
    [
        qi,
        attr_add_ref as *const () as usize,
        attr_release_obj::<T> as *const () as usize,
        attr_get_item as *const () as usize,
        attr_get_item_type as *const () as usize,
        attr_compare_item as *const () as usize,
        attr_compare as *const () as usize,
        attr_get_uint32 as *const () as usize,
        attr_get_uint64 as *const () as usize,
        attr_get_double as *const () as usize,
        attr_get_guid as *const () as usize,
        attr_get_string_length as *const () as usize,
        attr_get_string as *const () as usize,
        attr_get_allocated_string as *const () as usize,
        attr_get_blob_size as *const () as usize,
        attr_get_blob as *const () as usize,
        attr_get_allocated_blob as *const () as usize,
        attr_get_unknown as *const () as usize,
        attr_set_item as *const () as usize,
        attr_delete_item as *const () as usize,
        attr_delete_all_items as *const () as usize,
        attr_set_uint32 as *const () as usize,
        attr_set_uint64 as *const () as usize,
        crate::bridge_entry!(attr_set_double) as usize,
        attr_set_guid as *const () as usize,
        attr_set_string as *const () as usize,
        attr_set_blob as *const () as usize,
        attr_set_unknown as *const () as usize,
        attr_lock_store as *const () as usize,
        attr_unlock_store as *const () as usize,
        attr_get_count as *const () as usize,
        attr_get_item_by_index as *const () as usize,
        attr_copy_all_items as *const () as usize,
    ]
}

fn attr_vtable() -> &'static [usize; 33] {
    static VTBL: OnceLock<[usize; 33]> = OnceLock::new();
    VTBL.get_or_init(|| register_vtable(fill_attr_slots::<AttrObj>(attr_qi as *const () as usize)))
}

pub(super) unsafe fn create_attributes(
    pp_mf_attributes: *mut *mut c_void,
    c_initial_size: u32,
) -> HRESULT {
    if pp_mf_attributes.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(AttrObj {
        hdr: AttrHeader {
            vtbl: attr_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(c_initial_size as usize),
        },
    });
    std::ptr::write_unaligned(pp_mf_attributes, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// IMFMediaType (inherits IMFAttributes, slots 33..37)
// ============================================================================

fn is_uncompressed_subtype(g: &Guid) -> bool {
    *g == MF_AUDIO_FORMAT_PCM
        || *g == MF_AUDIO_FORMAT_FLOAT
        || *g == MF_VIDEO_FORMAT_RGB32
        || *g == MF_VIDEO_FORMAT_ARGB32
        || *g == MF_VIDEO_FORMAT_RGB24
        || *g == MF_VIDEO_FORMAT_NV12
        || *g == MF_VIDEO_FORMAT_YUY2
        || *g == MF_VIDEO_FORMAT_YV12
        || *g == MF_VIDEO_FORMAT_IYUV
        || *g == MF_VIDEO_FORMAT_I420
        || *g == MF_VIDEO_FORMAT_UYVY
        || *g == MF_VIDEO_FORMAT_P010
        || *g == MF_VIDEO_FORMAT_AYUV
}

unsafe extern "win64" fn mediatype_get_major_type(this: usize, out: *mut Guid) -> HRESULT {
    attr_get_guid(this, &MF_MT_MAJOR_TYPE, out)
}

unsafe extern "win64" fn mediatype_is_compressed_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    // Explicit MF_MT_ALL_SAMPLES_INDEPENDENT == 0 or MF_MT_FIXED_SIZE_SAMPLES == 0 => compressed.
    if let Some(AttrValue::Uint32(v)) = h.attrs.get_clone(&MF_MT_ALL_SAMPLES_INDEPENDENT) {
        if v == 0 {
            std::ptr::write_unaligned(out, TRUE);
            return S_OK;
        }
    }
    if let Some(AttrValue::Uint32(v)) = h.attrs.get_clone(&MF_MT_FIXED_SIZE_SAMPLES) {
        if v == 0 {
            std::ptr::write_unaligned(out, TRUE);
            return S_OK;
        }
    }
    if let Some(AttrValue::Guid(sub)) = h.attrs.get_clone(&MF_MT_SUBTYPE) {
        let compressed = !is_uncompressed_subtype(&sub);
        std::ptr::write_unaligned(out, if compressed { TRUE } else { FALSE });
        return S_OK;
    }
    std::ptr::write_unaligned(out, FALSE);
    S_OK
}

unsafe extern "win64" fn mediatype_is_equal(
    this: usize,
    other: *mut c_void,
    out_flags: *mut u32,
) -> HRESULT {
    if other.is_null() || out_flags.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out_flags, 0);
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let ours = h.attrs.snapshot_keys_and_values();
    let theirs = match snapshot_any_attributes(other) {
        Ok(v) => v,
        Err(hr) => {
            for (_, v) in ours {
                v.release_if_unk();
            }
            return hr;
        }
    };
    fn find_attr<'a>(list: &'a [(Guid, AttrValue)], key: &Guid) -> Option<&'a AttrValue> {
        list.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    let mut flags = 0u32;
    let major_eq = match (
        find_attr(&ours, &MF_MT_MAJOR_TYPE),
        find_attr(&theirs, &MF_MT_MAJOR_TYPE),
    ) {
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if major_eq {
        flags |= MF_MEDIATYPE_EQUAL_MAJOR_TYPES;
    }
    let sub_eq = match (
        find_attr(&ours, &MF_MT_SUBTYPE),
        find_attr(&theirs, &MF_MT_SUBTYPE),
    ) {
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if major_eq && sub_eq {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_TYPES;
    }
    let user_eq = match (
        find_attr(&ours, &MF_MT_USER_DATA),
        find_attr(&theirs, &MF_MT_USER_DATA),
    ) {
        (None, None) => true,
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if user_eq {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA;
    }
    let all_non_user_match = {
        let ours_f: Vec<_> = ours.iter().filter(|(k, _)| *k != MF_MT_USER_DATA).collect();
        let theirs_f: Vec<_> = theirs.iter().filter(|(k, _)| *k != MF_MT_USER_DATA).collect();
        ours_f.len() == theirs_f.len()
            && ours_f.iter().all(|(k, v)| {
                theirs_f
                    .iter()
                    .find(|(bk, _)| bk == k)
                    .is_some_and(|(_, bv)| v.equals(bv))
            })
    };
    if major_eq && sub_eq && all_non_user_match {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_DATA;
    }
    for (_, v) in ours {
        v.release_if_unk();
    }
    for (_, v) in theirs {
        v.release_if_unk();
    }
    std::ptr::write_unaligned(out_flags, flags);
    const ALL_EQUAL: u32 = MF_MEDIATYPE_EQUAL_MAJOR_TYPES
        | MF_MEDIATYPE_EQUAL_FORMAT_TYPES
        | MF_MEDIATYPE_EQUAL_FORMAT_DATA
        | MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA;
    if flags == ALL_EQUAL {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn mediatype_get_representation(
    _this: usize,
    _guid_repr: *const Guid,
    out_repr: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out_repr);
    E_NOTIMPL
}

unsafe extern "win64" fn mediatype_free_representation(
    _this: usize,
    _guid_repr: *const Guid,
    _repr: *const c_void,
) -> HRESULT {
    S_OK
}

fn media_type_vtable() -> &'static [usize; 38] {
    static VTBL: OnceLock<[usize; 38]> = OnceLock::new();
    VTBL.get_or_init(|| {
        let base = fill_attr_slots::<MediaTypeObj>(mediatype_qi as *const () as usize);
        let mut full = [0usize; 38];
        full[..33].copy_from_slice(&base);
        full[33] = mediatype_get_major_type as *const () as usize;
        full[34] = mediatype_is_compressed_format as *const () as usize;
        full[35] = mediatype_is_equal as *const () as usize;
        full[36] = mediatype_get_representation as *const () as usize;
        full[37] = mediatype_free_representation as *const () as usize;
        register_vtable(full)
    })
}

pub(super) unsafe fn create_media_type(pp_mf_type: *mut *mut c_void) -> HRESULT {
    if pp_mf_type.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(MediaTypeObj {
        hdr: AttrHeader {
            vtbl: media_type_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(16),
        },
    });
    std::ptr::write_unaligned(pp_mf_type, Box::into_raw(obj) as *mut c_void);
    S_OK
}
