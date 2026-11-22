//! oleaut32.dll: BSTR allocation and VariantClear (imported by ordinal).

use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (2, "SysAllocString"),
    (3, "SysReAllocString"),
    (4, "SysAllocStringLen"),
    (5, "SysReAllocStringLen"),
    (6, "SysFreeString"),
    (7, "SysStringLen"),
    (8, "VariantInit"),
    (9, "VariantClear"),
    (10, "VariantCopy"),
    (11, "VariantCopyInd"),
    (12, "VariantChangeType"),
    (15, "SafeArrayCreate"),
    (16, "SafeArrayDestroy"),
    (17, "SafeArrayGetDim"),
    (18, "SafeArrayGetElemsize"),
    (19, "SafeArrayGetUBound"),
    (20, "SafeArrayGetLBound"),
    (21, "SafeArrayLock"),
    (22, "SafeArrayUnlock"),
    (23, "SafeArrayAccessData"),
    (24, "SafeArrayUnaccessData"),
    (25, "SafeArrayGetElement"),
    (26, "SafeArrayPutElement"),
    (27, "SafeArrayCopy"),
    (147, "VariantChangeTypeEx"),
    (148, "SafeArrayPtrOfIndex"),
    (149, "SysStringByteLen"),
    (150, "SysAllocStringByteLen"),
    (184, "SystemTimeToVariantTime"),
    (185, "VariantTimeToSystemTime"),
    (411, "SafeArrayCreateVector"),
];

/// BSTR = u16 data preceded by a u32 byte length; NUL-terminated.
unsafe fn alloc_bstr(src: *const u8, bytes: usize) -> *mut u16 {
    let p = crate::heap::alloc(4 + bytes + 2, false) as *mut u8;
    if p.is_null() {
        return std::ptr::null_mut();
    }
    *(p as *mut u32) = bytes as u32;
    if !src.is_null() {
        std::ptr::copy_nonoverlapping(src, p.add(4), bytes);
    } else {
        std::ptr::write_bytes(p.add(4), 0, bytes);
    }
    *(p.add(4 + bytes) as *mut u16) = 0;
    p.add(4) as *mut u16
}

pub unsafe extern "win64" fn SysAllocString(s: *const u16) -> *mut u16 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    alloc_bstr(s as *const u8, crate::strings::wcslen(s) * 2)
}
pub unsafe extern "win64" fn SysAllocStringLen(s: *const u16, n: u32) -> *mut u16 {
    alloc_bstr(s as *const u8, n as usize * 2)
}
pub unsafe extern "win64" fn SysAllocStringByteLen(s: *const u8, n: u32) -> *mut u16 {
    alloc_bstr(s, n as usize)
}
pub unsafe extern "win64" fn SysFreeString(b: *mut u16) {
    if !b.is_null() {
        crate::heap::free((b as *mut u8).sub(4) as *mut c_void);
    }
}
pub unsafe extern "win64" fn SysStringLen(b: *const u16) -> u32 {
    if b.is_null() {
        0
    } else {
        *((b as *const u8).sub(4) as *const u32) / 2
    }
}
pub unsafe extern "win64" fn SysStringByteLen(b: *const u16) -> u32 {
    if b.is_null() {
        0
    } else {
        *((b as *const u8).sub(4) as *const u32)
    }
}
pub unsafe extern "win64" fn VariantInit(v: *mut u8) {
    std::ptr::write_bytes(v, 0, 24);
}
/// VARIANT: frees BSTRs; releases IUnknown/IDispatch via vtable slot 2.
pub unsafe extern "win64" fn VariantClear(v: *mut u8) -> i32 {
    if v.is_null() {
        return 0;
    }
    let vt = *(v as *const u16);
    const VT_BSTR: u16 = 8;
    const VT_DISPATCH: u16 = 9;
    const VT_UNKNOWN: u16 = 13;
    match vt {
        VT_BSTR => SysFreeString(*(v.add(8) as *const *mut u16)),
        VT_DISPATCH | VT_UNKNOWN => {
            let obj = *(v.add(8) as *const *mut *const usize);
            if !obj.is_null() {
                let release: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*(*obj).add(2));
                release(obj as *mut c_void);
            }
        }
        x if x & 0x2000 != 0 => stub!("VariantClear: SAFEARRAY payload leaked"),
        _ => {}
    }
    std::ptr::write_bytes(v, 0, 24);
    0
}

pub unsafe extern "win64" fn SafeArrayGetDim(psa: *const u16) -> u32 {
    if psa.is_null() {
        0
    } else {
        std::ptr::read_unaligned(psa) as u32
    }
}

pub unsafe extern "win64" fn SysReAllocString(pbstr: *mut *mut u16, s: *const u16) -> i32 {
    if pbstr.is_null() {
        return 0;
    }
    let old = std::ptr::read_unaligned(pbstr);
    let new = SysAllocString(s);
    if !old.is_null() {
        SysFreeString(old);
    }
    std::ptr::write_unaligned(pbstr, new);
    1
}

pub unsafe extern "win64" fn SysReAllocStringLen(pbstr: *mut *mut u16, s: *const u16, n: u32) -> i32 {
    if pbstr.is_null() {
        return 0;
    }
    let old = std::ptr::read_unaligned(pbstr);
    let new = SysAllocStringLen(s, n);
    if !old.is_null() {
        SysFreeString(old);
    }
    std::ptr::write_unaligned(pbstr, new);
    1
}

pub unsafe extern "win64" fn VariantCopy(dst: *mut u8, src: *const u8) -> i32 {
    if dst.is_null() || src.is_null() {
        return crate::types::E_INVALIDARG;
    }
    if dst as *const u8 == src {
        return crate::types::S_OK;
    }
    VariantClear(dst);
    std::ptr::copy_nonoverlapping(src, dst, 24);
    let vt = std::ptr::read_unaligned(src as *const u16);
    if vt == 8 {
        let s = std::ptr::read_unaligned(src.add(8) as *const *const u16);
        let dup = if s.is_null() { std::ptr::null_mut() } else { SysAllocStringLen(s, SysStringLen(s as *mut u16)) };
        std::ptr::write_unaligned(dst.add(8) as *mut *mut u16, dup);
    }
    crate::types::S_OK
}

pub unsafe extern "win64" fn VariantCopyInd(dst: *mut u8, src: *const u8) -> i32 {
    VariantCopy(dst, src)
}

pub unsafe extern "win64" fn VariantChangeType(
    dst: *mut u8,
    src: *const u8,
    _flags: u16,
    vt: u16,
) -> i32 {
    if dst.is_null() || src.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let cur_vt = std::ptr::read_unaligned(src as *const u16);
    if cur_vt == vt {
        return VariantCopy(dst, src);
    }
    stub!("VariantChangeType({cur_vt} -> {vt}) -> DISP_E_TYPEMISMATCH");
    0x8002_0005u32 as i32
}

pub unsafe extern "win64" fn VariantChangeTypeEx(
    dst: *mut u8,
    src: *const u8,
    _lcid: u32,
    flags: u16,
    vt: u16,
) -> i32 {
    VariantChangeType(dst, src, flags, vt)
}

fn vt_elem_size(vt: u16) -> u32 {
    match vt {
        16 | 17 => 1,
        2 | 11 | 18 => 2,
        3 | 4 | 10 | 19 | 22 | 23 => 4,
        5 | 6 | 7 | 8 | 9 | 13 | 20 | 21 => 8,
        12 => 24,
        _ => 8,
    }
}

pub unsafe extern "win64" fn SafeArrayCreate(vt: u16, dims: u32, bounds: *const u8) -> *mut u8 {
    if dims == 0 || dims > 64 || bounds.is_null() {
        return std::ptr::null_mut();
    }
    let elem_sz = vt_elem_size(vt);
    let mut total: usize = 1;
    for i in 0..dims as usize {
        let cnt = std::ptr::read_unaligned(bounds.add(i * 8) as *const u32) as usize;
        total = total.saturating_mul(cnt);
    }
    let hdr_sz = 24 + (dims as usize) * 8;
    let sa = crate::heap::alloc(hdr_sz, true) as *mut u8;
    if sa.is_null() {
        return std::ptr::null_mut();
    }
    let data = if total > 0 {
        crate::heap::alloc(total.saturating_mul(elem_sz as usize), true)
    } else {
        std::ptr::null_mut()
    };
    std::ptr::write_unaligned(sa.add(0) as *mut u16, dims as u16);
    std::ptr::write_unaligned(sa.add(4) as *mut u32, elem_sz);
    std::ptr::write_unaligned(sa.add(16) as *mut *mut c_void, data);
    // Windows stores rgsabound in reverse order (dim 1 last).
    for i in 0..dims as usize {
        let src_b = bounds.add(i * 8);
        let dst_b = sa.add(24 + (dims as usize - 1 - i) * 8);
        std::ptr::copy_nonoverlapping(src_b, dst_b, 8);
    }
    sa
}

pub unsafe extern "win64" fn SafeArrayCreateVector(vt: u16, lbound: i32, count: u32) -> *mut u8 {
    let mut bound = [0u8; 8];
    bound[0..4].copy_from_slice(&count.to_le_bytes());
    bound[4..8].copy_from_slice(&lbound.to_le_bytes());
    SafeArrayCreate(vt, 1, bound.as_ptr())
}

pub unsafe extern "win64" fn SafeArrayDestroy(psa: *mut u8) -> i32 {
    if psa.is_null() {
        return crate::types::S_OK;
    }
    let data = std::ptr::read_unaligned(psa.add(16) as *const *mut c_void);
    if !data.is_null() {
        crate::heap::free(data);
    }
    crate::heap::free(psa as *mut c_void);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayGetElemsize(psa: *const u8) -> u32 {
    if psa.is_null() {
        0
    } else {
        std::ptr::read_unaligned(psa.add(4) as *const u32)
    }
}

pub unsafe extern "win64" fn SafeArrayGetLBound(psa: *const u8, dim: u32, out: *mut i32) -> i32 {
    if psa.is_null() || out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let dims = std::ptr::read_unaligned(psa as *const u16) as u32;
    if dim == 0 || dim > dims {
        return 0x8002_000Bu32 as i32; // DISP_E_BADINDEX
    }
    let idx = (dims - dim) as usize;
    let lb = std::ptr::read_unaligned(psa.add(24 + idx * 8 + 4) as *const i32);
    std::ptr::write_unaligned(out, lb);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayGetUBound(psa: *const u8, dim: u32, out: *mut i32) -> i32 {
    if psa.is_null() || out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let dims = std::ptr::read_unaligned(psa as *const u16) as u32;
    if dim == 0 || dim > dims {
        return 0x8002_000Bu32 as i32; // DISP_E_BADINDEX
    }
    let idx = (dims - dim) as usize;
    let cnt = std::ptr::read_unaligned(psa.add(24 + idx * 8) as *const u32) as i32;
    let lb = std::ptr::read_unaligned(psa.add(24 + idx * 8 + 4) as *const i32);
    std::ptr::write_unaligned(out, lb.wrapping_add(cnt).wrapping_sub(1));
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayLock(psa: *mut u8) -> i32 {
    if psa.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let locks = std::ptr::read_unaligned(psa.add(8) as *const u32);
    std::ptr::write_unaligned(psa.add(8) as *mut u32, locks.saturating_add(1));
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayUnlock(psa: *mut u8) -> i32 {
    if psa.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let locks = std::ptr::read_unaligned(psa.add(8) as *const u32);
    std::ptr::write_unaligned(psa.add(8) as *mut u32, locks.saturating_sub(1));
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayAccessData(psa: *mut u8, out: *mut *mut c_void) -> i32 {
    if psa.is_null() || out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let _ = SafeArrayLock(psa);
    let data = std::ptr::read_unaligned(psa.add(16) as *const *mut c_void);
    std::ptr::write_unaligned(out, data);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayUnaccessData(psa: *mut u8) -> i32 {
    SafeArrayUnlock(psa)
}

pub unsafe extern "win64" fn SafeArrayPtrOfIndex(
    psa: *mut u8,
    indices: *const i32,
    out: *mut *mut c_void,
) -> i32 {
    if psa.is_null() || indices.is_null() || out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let dims = std::ptr::read_unaligned(psa as *const u16) as usize;
    let elem_sz = std::ptr::read_unaligned(psa.add(4) as *const u32) as usize;
    let data = std::ptr::read_unaligned(psa.add(16) as *const *mut u8);
    if data.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let mut flat: usize = 0;
    let mut stride: usize = 1;
    for d in 0..dims {
        let b_idx = dims - 1 - d;
        let cnt = std::ptr::read_unaligned(psa.add(24 + b_idx * 8) as *const u32) as i32;
        let lb = std::ptr::read_unaligned(psa.add(24 + b_idx * 8 + 4) as *const i32);
        let i = std::ptr::read_unaligned(indices.add(d)) - lb;
        if i < 0 || i >= cnt {
            return 0x8002_000Bu32 as i32;
        }
        flat += (i as usize) * stride;
        stride *= cnt as usize;
    }
    std::ptr::write_unaligned(out, data.add(flat * elem_sz) as *mut c_void);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayGetElement(
    psa: *mut u8,
    indices: *const i32,
    out: *mut c_void,
) -> i32 {
    if out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let hr = SafeArrayPtrOfIndex(psa, indices, &mut ptr);
    if hr != crate::types::S_OK {
        return hr;
    }
    let sz = SafeArrayGetElemsize(psa) as usize;
    std::ptr::copy_nonoverlapping(ptr as *const u8, out as *mut u8, sz);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayPutElement(
    psa: *mut u8,
    indices: *const i32,
    val: *const c_void,
) -> i32 {
    if val.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let hr = SafeArrayPtrOfIndex(psa, indices, &mut ptr);
    if hr != crate::types::S_OK {
        return hr;
    }
    let sz = SafeArrayGetElemsize(psa) as usize;
    std::ptr::copy_nonoverlapping(val as *const u8, ptr as *mut u8, sz);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SafeArrayCopy(psa: *const u8, out: *mut *mut u8) -> i32 {
    if out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    if psa.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
        return crate::types::S_OK;
    }
    let dims = std::ptr::read_unaligned(psa as *const u16) as usize;
    let elem_sz = std::ptr::read_unaligned(psa.add(4) as *const u32) as usize;
    let hdr_sz = 24 + dims * 8;
    let mut total: usize = 1;
    for i in 0..dims {
        let cnt = std::ptr::read_unaligned(psa.add(24 + i * 8) as *const u32) as usize;
        total = total.saturating_mul(cnt);
    }
    let dup = crate::heap::alloc(hdr_sz, true) as *mut u8;
    if dup.is_null() {
        return 0x8007_000Eu32 as i32;
    }
    std::ptr::copy_nonoverlapping(psa, dup, hdr_sz);
    std::ptr::write_unaligned(dup.add(8) as *mut u32, 0);
    let src_data = std::ptr::read_unaligned(psa.add(16) as *const *const u8);
    let data_bytes = total.saturating_mul(elem_sz);
    let dst_data = if data_bytes > 0 && !src_data.is_null() {
        let p = crate::heap::alloc(data_bytes, false) as *mut u8;
        if !p.is_null() {
            std::ptr::copy_nonoverlapping(src_data, p, data_bytes);
        }
        p as *mut c_void
    } else {
        std::ptr::null_mut()
    };
    std::ptr::write_unaligned(dup.add(16) as *mut *mut c_void, dst_data);
    std::ptr::write_unaligned(out, dup);
    crate::types::S_OK
}

pub unsafe extern "win64" fn SystemTimeToVariantTime(_st: *const u16, out: *mut f64) -> i32 {
    if !out.is_null() {
        std::ptr::write_unaligned(out, 0.0);
    }
    0
}

pub unsafe extern "win64" fn VariantTimeToSystemTime(_vt: f64, st: *mut u16) -> i32 {
    if !st.is_null() {
        std::ptr::write_bytes(st, 0, 8);
    }
    0
}
