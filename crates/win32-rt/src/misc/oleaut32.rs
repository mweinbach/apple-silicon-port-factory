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
const DISP_E_TYPEMISMATCH: i32 = 0x8002_0005u32 as i32;
const DISP_E_BADVARTYPE: i32 = 0x8002_0008u32 as i32;
const DISP_E_OVERFLOW: i32 = 0x8002_000Au32 as i32;
const DISP_E_ARRAYISLOCKED: i32 = 0x8002_000Du32 as i32;
const E_OUTOFMEMORY: i32 = 0x8007_000Eu32 as i32;

const VT_EMPTY: u16 = 0;
const VT_NULL: u16 = 1;
const VT_I2: u16 = 2;
const VT_I4: u16 = 3;
const VT_R4: u16 = 4;
const VT_R8: u16 = 5;
const VT_CY: u16 = 6;
const VT_DATE: u16 = 7;
const VT_BSTR: u16 = 8;
const VT_DISPATCH: u16 = 9;
const VT_ERROR: u16 = 10;
const VT_BOOL: u16 = 11;
const VT_VARIANT: u16 = 12;
const VT_UNKNOWN: u16 = 13;
const VT_DECIMAL: u16 = 14;
const VT_I1: u16 = 16;
const VT_UI1: u16 = 17;
const VT_UI2: u16 = 18;
const VT_UI4: u16 = 19;
const VT_I8: u16 = 20;
const VT_UI8: u16 = 21;
const VT_INT: u16 = 22;
const VT_UINT: u16 = 23;
const VT_RECORD: u16 = 36;
const VT_ARRAY: u16 = 0x2000;
const VT_BYREF: u16 = 0x4000;

const FADF_BSTR: u16 = 0x0100;
const FADF_UNKNOWN: u16 = 0x0200;
const FADF_DISPATCH: u16 = 0x0400;
const FADF_VARIANT: u16 = 0x0800;

const IRECORDINFO_RECORD_COPY: usize = 5;
const IRECORDINFO_RECORD_CREATE: usize = 16;
const IRECORDINFO_RECORD_DESTROY: usize = 18;

pub unsafe extern "win64" fn VariantInit(v: *mut u8) {
    if !v.is_null() {
        std::ptr::write_bytes(v, 0, 24);
    }
}

/// VARIANT: frees BSTRs and owned SAFEARRAYs, destroys VT_RECORD via IRecordInfo,
/// and releases IUnknown/IDispatch via vtable slot 2. Never frees VT_BYREF payloads.
pub unsafe extern "win64" fn VariantClear(v: *mut u8) -> i32 {
    if v.is_null() {
        return crate::types::S_OK;
    }
    let vt = std::ptr::read_unaligned(v as *const u16);
    if (vt & VT_BYREF) == 0 {
        if (vt & VT_ARRAY) != 0 {
            let psa = std::ptr::read_unaligned(v.add(8) as *const *mut u8);
            if !psa.is_null() {
                let hr = SafeArrayDestroy(psa);
                if hr != crate::types::S_OK {
                    return hr;
                }
            }
        } else {
            match vt {
                VT_BSTR => SysFreeString(std::ptr::read_unaligned(v.add(8) as *const *mut u16)),
                VT_DISPATCH | VT_UNKNOWN => {
                    let obj = std::ptr::read_unaligned(v.add(8) as *const *mut c_void);
                    if !obj.is_null() {
                        crate::com::com_call_release(obj);
                    }
                }
                VT_RECORD => {
                    let pv_rec = std::ptr::read_unaligned(v.add(8) as *const *mut c_void);
                    let rec_info = std::ptr::read_unaligned(v.add(16) as *const *mut c_void);
                    if !rec_info.is_null() {
                        if !pv_rec.is_null() {
                            let _ = crate::com::com_call_slot(
                                rec_info,
                                IRECORDINFO_RECORD_DESTROY,
                                pv_rec as u64,
                                0,
                                0,
                            );
                        }
                        crate::com::com_call_release(rec_info);
                    }
                }
                _ => {}
            }
        }
    }
    std::ptr::write_bytes(v, 0, 24);
    crate::types::S_OK
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

unsafe fn copy_bstr(s: *const u16) -> Result<*mut u16, i32> {
    if s.is_null() {
        return Ok(std::ptr::null_mut());
    }
    let bytes = SysStringByteLen(s as *mut u16);
    let dup = SysAllocStringByteLen(s as *const u8, bytes);
    if dup.is_null() {
        Err(E_OUTOFMEMORY)
    } else {
        Ok(dup)
    }
}

unsafe fn copy_record_into(out: &mut [u8; 24], pv_rec: *mut c_void, rec_info: *mut c_void) -> i32 {
    if rec_info.is_null() {
        if !pv_rec.is_null() {
            return DISP_E_BADVARTYPE;
        }
        std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, VT_RECORD);
        return crate::types::S_OK;
    }
    let new_rec = if pv_rec.is_null() {
        std::ptr::null_mut()
    } else {
        let Some(raw) = crate::com::com_call_slot_raw(rec_info, IRECORDINFO_RECORD_CREATE, &[]) else {
            return DISP_E_BADVARTYPE;
        };
        let created = raw as *mut c_void;
        if created.is_null() {
            return E_OUTOFMEMORY;
        }
        let hr = crate::com::com_call_slot(
            rec_info,
            IRECORDINFO_RECORD_COPY,
            pv_rec as u64,
            created as u64,
            0,
        );
        if hr < 0 {
            let _ = crate::com::com_call_slot(
                rec_info,
                IRECORDINFO_RECORD_DESTROY,
                created as u64,
                0,
                0,
            );
            return hr;
        }
        created
    };
    crate::com::com_call_add_ref(rec_info);
    std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, VT_RECORD);
    std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut c_void, new_rec);
    std::ptr::write_unaligned(out.as_mut_ptr().add(16) as *mut *mut c_void, rec_info);
    crate::types::S_OK
}

unsafe fn variant_copy_into_buf(out: &mut [u8; 24], src: *const u8) -> i32 {
    *out = [0u8; 24];
    let vt = std::ptr::read_unaligned(src as *const u16);

    if (vt & VT_BYREF) != 0 {
        // VT_BYREF: copy pointer without taking ownership.
        std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24);
        return crate::types::S_OK;
    }

    if (vt & VT_ARRAY) != 0 {
        std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24);
        let psa = std::ptr::read_unaligned(src.add(8) as *const *const u8);
        let mut dup: *mut u8 = std::ptr::null_mut();
        if !psa.is_null() {
            let hr = SafeArrayCopy(psa, &mut dup);
            if hr != crate::types::S_OK {
                *out = [0u8; 24];
                return hr;
            }
        }
        std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut u8, dup);
        return crate::types::S_OK;
    }

    match vt {
        VT_BSTR => {
            std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24);
            let s = std::ptr::read_unaligned(src.add(8) as *const *const u16);
            match copy_bstr(s) {
                Ok(dup) => {
                    std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut u16, dup);
                    crate::types::S_OK
                }
                Err(hr) => {
                    *out = [0u8; 24];
                    hr
                }
            }
        }
        VT_UNKNOWN | VT_DISPATCH => {
            std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24);
            let obj = std::ptr::read_unaligned(src.add(8) as *const *mut c_void);
            if !obj.is_null() {
                crate::com::com_call_add_ref(obj);
            }
            crate::types::S_OK
        }
        VT_RECORD => {
            let pv_rec = std::ptr::read_unaligned(src.add(8) as *const *mut c_void);
            let rec_info = std::ptr::read_unaligned(src.add(16) as *const *mut c_void);
            copy_record_into(out, pv_rec, rec_info)
        }
        _ => {
            std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24);
            crate::types::S_OK
        }
    }
}

pub unsafe extern "win64" fn VariantCopy(dst: *mut u8, src: *const u8) -> i32 {
    if dst.is_null() || src.is_null() {
        return crate::types::E_INVALIDARG;
    }
    if dst as *const u8 == src {
        return crate::types::S_OK;
    }
    let mut tmp = [0u8; 24];
    let hr = variant_copy_into_buf(&mut tmp, src);
    if hr != crate::types::S_OK {
        return hr;
    }
    let clr_hr = VariantClear(dst);
    if clr_hr != crate::types::S_OK {
        let _ = VariantClear(tmp.as_mut_ptr());
        return clr_hr;
    }
    std::ptr::copy_nonoverlapping(tmp.as_ptr(), dst, 24);
    crate::types::S_OK
}

unsafe fn variant_copy_ind_into_buf(out: &mut [u8; 24], src: *const u8, depth: u8) -> i32 {
    if depth > 8 {
        return crate::types::E_INVALIDARG;
    }
    *out = [0u8; 24];
    let vt = std::ptr::read_unaligned(src as *const u16);
    if (vt & VT_BYREF) == 0 {
        return variant_copy_into_buf(out, src);
    }

    let inner_vt = vt & !VT_BYREF;
    let ref_ptr = std::ptr::read_unaligned(src.add(8) as *const *const u8);

    if inner_vt == VT_VARIANT {
        if ref_ptr.is_null() {
            return crate::types::E_INVALIDARG;
        }
        // Per Windows semantics, a VT_BYREF|VT_VARIANT cannot point to another VT_BYREF|VT_VARIANT.
        let nested_vt = std::ptr::read_unaligned(ref_ptr as *const u16);
        if nested_vt == (VT_BYREF | VT_VARIANT) {
            return crate::types::E_INVALIDARG;
        }
        return variant_copy_ind_into_buf(out, ref_ptr, depth + 1);
    }

    if (inner_vt & VT_ARRAY) != 0 {
        if ref_ptr.is_null() {
            return crate::types::E_INVALIDARG;
        }
        let psa = std::ptr::read_unaligned(ref_ptr as *const *const u8);
        let mut dup: *mut u8 = std::ptr::null_mut();
        if !psa.is_null() {
            let hr = SafeArrayCopy(psa, &mut dup);
            if hr != crate::types::S_OK {
                return hr;
            }
        }
        std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
        std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut u8, dup);
        return crate::types::S_OK;
    }

    match inner_vt {
        VT_BSTR => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            let s = std::ptr::read_unaligned(ref_ptr as *const *const u16);
            match copy_bstr(s) {
                Ok(dup) => {
                    std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, VT_BSTR);
                    std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut u16, dup);
                    crate::types::S_OK
                }
                Err(hr) => hr,
            }
        }
        VT_UNKNOWN | VT_DISPATCH => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            let obj = std::ptr::read_unaligned(ref_ptr as *const *mut c_void);
            if !obj.is_null() {
                crate::com::com_call_add_ref(obj);
            }
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
            std::ptr::write_unaligned(out.as_mut_ptr().add(8) as *mut *mut c_void, obj);
            crate::types::S_OK
        }
        VT_RECORD => {
            let rec_info = std::ptr::read_unaligned(src.add(16) as *const *mut c_void);
            copy_record_into(out, ref_ptr as *mut c_void, rec_info)
        }
        VT_DECIMAL => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::copy_nonoverlapping(ref_ptr, out.as_mut_ptr(), 16);
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, VT_DECIMAL);
            crate::types::S_OK
        }
        VT_I1 | VT_UI1 => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
            out[8] = std::ptr::read_unaligned(ref_ptr);
            crate::types::S_OK
        }
        VT_I2 | VT_UI2 | VT_BOOL => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
            std::ptr::copy_nonoverlapping(ref_ptr, out.as_mut_ptr().add(8), 2);
            crate::types::S_OK
        }
        VT_I4 | VT_UI4 | VT_R4 | VT_ERROR | VT_INT | VT_UINT => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
            std::ptr::copy_nonoverlapping(ref_ptr, out.as_mut_ptr().add(8), 4);
            crate::types::S_OK
        }
        VT_I8 | VT_UI8 | VT_R8 | VT_CY | VT_DATE => {
            if ref_ptr.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, inner_vt);
            std::ptr::copy_nonoverlapping(ref_ptr, out.as_mut_ptr().add(8), 8);
            crate::types::S_OK
        }
        _ => DISP_E_BADVARTYPE,
    }
}

pub unsafe extern "win64" fn VariantCopyInd(dst: *mut u8, src: *const u8) -> i32 {
    if dst.is_null() || src.is_null() {
        return crate::types::E_INVALIDARG;
    }
    let vt = std::ptr::read_unaligned(src as *const u16);
    if (vt & VT_BYREF) == 0 {
        return VariantCopy(dst, src);
    }
    let mut tmp = [0u8; 24];
    let hr = variant_copy_ind_into_buf(&mut tmp, src, 0);
    if hr != crate::types::S_OK {
        return hr;
    }
    // Note: if dst == src, src was VT_BYREF so clearing dst does not free the pointed-to data
    // (which we already deep-copied/AddRef'd into `tmp`).
    let clr_hr = VariantClear(dst);
    if clr_hr != crate::types::S_OK {
        let _ = VariantClear(tmp.as_mut_ptr());
        return clr_hr;
    }
    std::ptr::copy_nonoverlapping(tmp.as_ptr(), dst, 24);
    crate::types::S_OK
}

enum CoercedVal {
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
    Str(String),
}

unsafe fn read_variant_value(src: *const u8, vt: u16) -> Result<CoercedVal, i32> {
    let p = src.add(8);
    match vt {
        VT_EMPTY => Ok(CoercedVal::I64(0)),
        VT_I1 => Ok(CoercedVal::I64(std::ptr::read_unaligned(p as *const i8) as i64)),
        VT_UI1 => Ok(CoercedVal::U64(std::ptr::read_unaligned(p) as u64)),
        VT_I2 => Ok(CoercedVal::I64(std::ptr::read_unaligned(p as *const i16) as i64)),
        VT_UI2 => Ok(CoercedVal::U64(std::ptr::read_unaligned(p as *const u16) as u64)),
        VT_I4 | VT_INT => Ok(CoercedVal::I64(std::ptr::read_unaligned(p as *const i32) as i64)),
        VT_UI4 | VT_UINT => Ok(CoercedVal::U64(std::ptr::read_unaligned(p as *const u32) as u64)),
        VT_I8 => Ok(CoercedVal::I64(std::ptr::read_unaligned(p as *const i64))),
        VT_UI8 => Ok(CoercedVal::U64(std::ptr::read_unaligned(p as *const u64))),
        VT_R4 => Ok(CoercedVal::F64(std::ptr::read_unaligned(p as *const f32) as f64)),
        VT_R8 => Ok(CoercedVal::F64(std::ptr::read_unaligned(p as *const f64))),
        VT_BOOL => Ok(CoercedVal::Bool(std::ptr::read_unaligned(p as *const i16) != 0)),
        VT_BSTR => {
            let bstr = std::ptr::read_unaligned(p as *const *const u16);
            if bstr.is_null() {
                Ok(CoercedVal::Str(String::new()))
            } else {
                let len = SysStringLen(bstr as *mut u16) as usize;
                let mut units = Vec::with_capacity(len);
                for i in 0..len {
                    units.push(std::ptr::read_unaligned(bstr.add(i)));
                }
                Ok(CoercedVal::Str(String::from_utf16_lossy(&units)))
            }
        }
        _ => Err(DISP_E_TYPEMISMATCH),
    }
}

fn coerced_to_i64(v: &CoercedVal) -> Result<i64, i32> {
    match v {
        CoercedVal::I64(n) => Ok(*n),
        CoercedVal::U64(n) => i64::try_from(*n).map_err(|_| DISP_E_OVERFLOW),
        CoercedVal::F64(f) => {
            if !f.is_finite() {
                return Err(DISP_E_OVERFLOW);
            }
            let r = f.round();
            if r < i64::MIN as f64 || r >= (i64::MAX as f64) {
                return Err(DISP_E_OVERFLOW);
            }
            Ok(r as i64)
        }
        CoercedVal::Bool(b) => Ok(if *b { -1 } else { 0 }),
        CoercedVal::Str(s) => {
            let t = s.trim();
            if let Ok(n) = t.parse::<i64>() {
                Ok(n)
            } else if let Ok(f) = t.parse::<f64>() {
                coerced_to_i64(&CoercedVal::F64(f))
            } else {
                Err(DISP_E_TYPEMISMATCH)
            }
        }
    }
}

fn coerced_to_u64(v: &CoercedVal) -> Result<u64, i32> {
    match v {
        CoercedVal::I64(n) => u64::try_from(*n).map_err(|_| DISP_E_OVERFLOW),
        CoercedVal::U64(n) => Ok(*n),
        CoercedVal::F64(f) => {
            if !f.is_finite() || *f < 0.0 || *f >= (u64::MAX as f64) {
                return Err(DISP_E_OVERFLOW);
            }
            Ok(f.round() as u64)
        }
        CoercedVal::Bool(b) => Ok(if *b { u64::MAX } else { 0 }),
        CoercedVal::Str(s) => s.trim().parse::<u64>().map_err(|_| DISP_E_TYPEMISMATCH),
    }
}

fn coerced_to_f64(v: &CoercedVal) -> Result<f64, i32> {
    match v {
        CoercedVal::I64(n) => Ok(*n as f64),
        CoercedVal::U64(n) => Ok(*n as f64),
        CoercedVal::F64(f) => Ok(*f),
        CoercedVal::Bool(b) => Ok(if *b { -1.0 } else { 0.0 }),
        CoercedVal::Str(s) => s.trim().parse::<f64>().map_err(|_| DISP_E_TYPEMISMATCH),
    }
}

fn coerced_to_bool(v: &CoercedVal) -> Result<bool, i32> {
    match v {
        CoercedVal::I64(n) => Ok(*n != 0),
        CoercedVal::U64(n) => Ok(*n != 0),
        CoercedVal::F64(f) => Ok(*f != 0.0),
        CoercedVal::Bool(b) => Ok(*b),
        CoercedVal::Str(s) => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "#true#" => Ok(true),
            "false" | "#false#" => Ok(false),
            other => other
                .parse::<f64>()
                .map(|f| f != 0.0)
                .map_err(|_| DISP_E_TYPEMISMATCH),
        },
    }
}

fn coerced_to_string(v: &CoercedVal) -> String {
    match v {
        CoercedVal::I64(n) => n.to_string(),
        CoercedVal::U64(n) => n.to_string(),
        CoercedVal::F64(f) => f.to_string(),
        CoercedVal::Bool(b) => if *b { "True".into() } else { "False".into() },
        CoercedVal::Str(s) => s.clone(),
    }
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
    if vt == VT_EMPTY {
        return VariantClear(dst);
    }
    if cur_vt == VT_NULL {
        return DISP_E_TYPEMISMATCH;
    }
    let val = match read_variant_value(src, cur_vt) {
        Ok(v) => v,
        Err(hr) => return hr,
    };

    // Build converted 24-byte VARIANT in a local buffer first so `dst == src` is safe.
    let mut out = [0u8; 24];
    std::ptr::write_unaligned(out.as_mut_ptr() as *mut u16, vt);
    let payload = out.as_mut_ptr().add(8);
    let conv: Result<(), i32> = match vt {
        VT_I1 => coerced_to_i64(&val)
            .and_then(|n| i8::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload as *mut i8, n)),
        VT_UI1 => coerced_to_u64(&val)
            .and_then(|n| u8::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload, n)),
        VT_I2 => coerced_to_i64(&val)
            .and_then(|n| i16::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload as *mut i16, n)),
        VT_UI2 => coerced_to_u64(&val)
            .and_then(|n| u16::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload as *mut u16, n)),
        VT_I4 | VT_INT => coerced_to_i64(&val)
            .and_then(|n| i32::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload as *mut i32, n)),
        VT_UI4 | VT_UINT => coerced_to_u64(&val)
            .and_then(|n| u32::try_from(n).map_err(|_| DISP_E_OVERFLOW))
            .map(|n| std::ptr::write_unaligned(payload as *mut u32, n)),
        VT_I8 => coerced_to_i64(&val).map(|n| std::ptr::write_unaligned(payload as *mut i64, n)),
        VT_UI8 => coerced_to_u64(&val).map(|n| std::ptr::write_unaligned(payload as *mut u64, n)),
        VT_R4 => coerced_to_f64(&val).map(|f| std::ptr::write_unaligned(payload as *mut f32, f as f32)),
        VT_R8 => coerced_to_f64(&val).map(|f| std::ptr::write_unaligned(payload as *mut f64, f)),
        VT_BOOL => coerced_to_bool(&val).map(|b| {
            let vb: i16 = if b { -1 } else { 0 };
            std::ptr::write_unaligned(payload as *mut i16, vb);
        }),
        VT_BSTR => {
            let s = coerced_to_string(&val);
            let w: Vec<u16> = s.encode_utf16().collect();
            let bstr = SysAllocStringLen(w.as_ptr(), w.len() as u32);
            std::ptr::write_unaligned(payload as *mut *mut u16, bstr);
            Ok(())
        }
        _ => Err(DISP_E_TYPEMISMATCH),
    };
    if let Err(hr) = conv {
        return hr;
    }
    let clr_hr = VariantClear(dst);
    if clr_hr != crate::types::S_OK {
        let _ = VariantClear(out.as_mut_ptr());
        return clr_hr;
    }
    std::ptr::copy_nonoverlapping(out.as_ptr(), dst, 24);
    crate::types::S_OK
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

fn vt_safearray_features(vt: u16) -> u16 {
    match vt {
        VT_BSTR => FADF_BSTR,
        VT_UNKNOWN => FADF_UNKNOWN,
        VT_DISPATCH => FADF_DISPATCH,
        VT_VARIANT => FADF_VARIANT,
        _ => 0,
    }
}

unsafe fn safearray_total_elems(psa: *const u8) -> usize {
    let dims = std::ptr::read_unaligned(psa as *const u16) as usize;
    if dims == 0 {
        return 0;
    }
    let mut total: usize = 1;
    for i in 0..dims {
        let cnt = std::ptr::read_unaligned(psa.add(24 + i * 8) as *const u32) as usize;
        total = total.saturating_mul(cnt);
    }
    total
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
        let p = crate::heap::alloc(total.saturating_mul(elem_sz as usize), true);
        if p.is_null() {
            crate::heap::free(sa as *mut c_void);
            return std::ptr::null_mut();
        }
        p
    } else {
        std::ptr::null_mut()
    };
    std::ptr::write_unaligned(sa.add(0) as *mut u16, dims as u16);
    std::ptr::write_unaligned(sa.add(2) as *mut u16, vt_safearray_features(vt));
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
    let locks = std::ptr::read_unaligned(psa.add(8) as *const u32);
    if locks > 0 {
        return DISP_E_ARRAYISLOCKED;
    }
    let features = std::ptr::read_unaligned(psa.add(2) as *const u16);
    let elem_sz = std::ptr::read_unaligned(psa.add(4) as *const u32) as usize;
    let total = safearray_total_elems(psa);
    let data = std::ptr::read_unaligned(psa.add(16) as *const *mut u8);
    if !data.is_null() && total > 0 {
        if (features & FADF_BSTR) != 0 && elem_sz == std::mem::size_of::<*mut u16>() {
            for i in 0..total {
                let b = std::ptr::read_unaligned(data.add(i * elem_sz) as *const *mut u16);
                if !b.is_null() {
                    SysFreeString(b);
                }
            }
        } else if (features & (FADF_UNKNOWN | FADF_DISPATCH)) != 0
            && elem_sz == std::mem::size_of::<*mut c_void>()
        {
            for i in 0..total {
                let obj = std::ptr::read_unaligned(data.add(i * elem_sz) as *const *mut c_void);
                if !obj.is_null() {
                    crate::com::com_call_release(obj);
                }
            }
        } else if (features & FADF_VARIANT) != 0 && elem_sz == 24 {
            for i in 0..total {
                let _ = VariantClear(data.add(i * 24));
            }
        }
        crate::heap::free(data as *mut c_void);
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
    let features = std::ptr::read_unaligned(psa.add(2) as *const u16);
    let sz = SafeArrayGetElemsize(psa) as usize;
    if (features & FADF_BSTR) != 0 && sz == std::mem::size_of::<*mut u16>() {
        let src_bstr = std::ptr::read_unaligned(ptr as *const *const u16);
        match copy_bstr(src_bstr) {
            Ok(dup) => {
                std::ptr::write_unaligned(out as *mut *mut u16, dup);
                crate::types::S_OK
            }
            Err(e) => e,
        }
    } else if (features & (FADF_UNKNOWN | FADF_DISPATCH)) != 0
        && sz == std::mem::size_of::<*mut c_void>()
    {
        let obj = std::ptr::read_unaligned(ptr as *const *mut c_void);
        if !obj.is_null() {
            crate::com::com_call_add_ref(obj);
        }
        std::ptr::write_unaligned(out as *mut *mut c_void, obj);
        crate::types::S_OK
    } else if (features & FADF_VARIANT) != 0 && sz == 24 {
        let mut tmp = [0u8; 24];
        let copy_hr = variant_copy_into_buf(&mut tmp, ptr as *const u8);
        if copy_hr != crate::types::S_OK {
            return copy_hr;
        }
        std::ptr::copy_nonoverlapping(tmp.as_ptr(), out as *mut u8, 24);
        crate::types::S_OK
    } else {
        std::ptr::copy_nonoverlapping(ptr as *const u8, out as *mut u8, sz);
        crate::types::S_OK
    }
}

pub unsafe extern "win64" fn SafeArrayPutElement(
    psa: *mut u8,
    indices: *const i32,
    val: *const c_void,
) -> i32 {
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let hr = SafeArrayPtrOfIndex(psa, indices, &mut ptr);
    if hr != crate::types::S_OK {
        return hr;
    }
    let features = std::ptr::read_unaligned(psa.add(2) as *const u16);
    let sz = SafeArrayGetElemsize(psa) as usize;
    // Per Windows SafeArrayPutElement ABI, `val` is the BSTR or COM interface pointer itself
    // (and may be NULL) for VT_BSTR / VT_UNKNOWN / VT_DISPATCH, and a pointer to the value
    // for all other types (including VT_VARIANT and scalars).
    if (features & FADF_BSTR) != 0 && sz == std::mem::size_of::<*mut u16>() {
        let dup = match copy_bstr(val as *const u16) {
            Ok(d) => d,
            Err(e) => return e,
        };
        let old = std::ptr::read_unaligned(ptr as *const *mut u16);
        std::ptr::write_unaligned(ptr as *mut *mut u16, dup);
        if !old.is_null() {
            SysFreeString(old);
        }
        crate::types::S_OK
    } else if (features & (FADF_UNKNOWN | FADF_DISPATCH)) != 0
        && sz == std::mem::size_of::<*mut c_void>()
    {
        let new_obj = val as *mut c_void;
        if !new_obj.is_null() {
            crate::com::com_call_add_ref(new_obj);
        }
        let old = std::ptr::read_unaligned(ptr as *const *mut c_void);
        std::ptr::write_unaligned(ptr as *mut *mut c_void, new_obj);
        if !old.is_null() {
            crate::com::com_call_release(old);
        }
        crate::types::S_OK
    } else if (features & FADF_VARIANT) != 0 && sz == 24 {
        if val.is_null() {
            return crate::types::E_INVALIDARG;
        }
        VariantCopy(ptr as *mut u8, val as *const u8)
    } else {
        if val.is_null() {
            return crate::types::E_INVALIDARG;
        }
        std::ptr::copy_nonoverlapping(val as *const u8, ptr as *mut u8, sz);
        crate::types::S_OK
    }
}

pub unsafe extern "win64" fn SafeArrayCopy(psa: *const u8, out: *mut *mut u8) -> i32 {
    if out.is_null() {
        return crate::types::E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if psa.is_null() {
        return crate::types::S_OK;
    }
    let dims = std::ptr::read_unaligned(psa as *const u16) as usize;
    let features = std::ptr::read_unaligned(psa.add(2) as *const u16);
    let elem_sz = std::ptr::read_unaligned(psa.add(4) as *const u32) as usize;
    let hdr_sz = 24 + dims * 8;
    let total = safearray_total_elems(psa);
    let dup = crate::heap::alloc(hdr_sz, true) as *mut u8;
    if dup.is_null() {
        return E_OUTOFMEMORY;
    }
    std::ptr::copy_nonoverlapping(psa, dup, hdr_sz);
    std::ptr::write_unaligned(dup.add(8) as *mut u32, 0);
    let src_data = std::ptr::read_unaligned(psa.add(16) as *const *const u8);
    let data_bytes = total.saturating_mul(elem_sz);
    let dst_data = if data_bytes > 0 && !src_data.is_null() {
        let p = crate::heap::alloc(data_bytes, true) as *mut u8;
        if p.is_null() {
            crate::heap::free(dup as *mut c_void);
            return E_OUTOFMEMORY;
        }
        if (features & FADF_BSTR) != 0 && elem_sz == std::mem::size_of::<*mut u16>() {
            for i in 0..total {
                let s = std::ptr::read_unaligned(src_data.add(i * elem_sz) as *const *const u16);
                match copy_bstr(s) {
                    Ok(cloned) => {
                        std::ptr::write_unaligned(p.add(i * elem_sz) as *mut *mut u16, cloned);
                    }
                    Err(hr) => {
                        for j in 0..i {
                            let prev =
                                std::ptr::read_unaligned(p.add(j * elem_sz) as *const *mut u16);
                            if !prev.is_null() {
                                SysFreeString(prev);
                            }
                        }
                        crate::heap::free(p as *mut c_void);
                        crate::heap::free(dup as *mut c_void);
                        return hr;
                    }
                }
            }
        } else if (features & (FADF_UNKNOWN | FADF_DISPATCH)) != 0
            && elem_sz == std::mem::size_of::<*mut c_void>()
        {
            for i in 0..total {
                let obj = std::ptr::read_unaligned(src_data.add(i * elem_sz) as *const *mut c_void);
                if !obj.is_null() {
                    crate::com::com_call_add_ref(obj);
                }
                std::ptr::write_unaligned(p.add(i * elem_sz) as *mut *mut c_void, obj);
            }
        } else if (features & FADF_VARIANT) != 0 && elem_sz == 24 {
            for i in 0..total {
                let dst_v = p.add(i * 24);
                let src_v = src_data.add(i * 24);
                let hr = VariantCopy(dst_v, src_v);
                if hr != crate::types::S_OK {
                    for j in 0..i {
                        let _ = VariantClear(p.add(j * 24));
                    }
                    crate::heap::free(p as *mut c_void);
                    crate::heap::free(dup as *mut c_void);
                    return hr;
                }
            }
        } else {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_clear_frees_safearray_and_change_type_coerces() {
        unsafe {
            // 1. VariantClear frees VT_ARRAY | VT_I4 without leaking or hitting a stub.
            let mut bounds = [0u8; 8];
            (bounds.as_mut_ptr() as *mut u32).write_unaligned(4); // cElements = 4
            (bounds.as_mut_ptr().add(4) as *mut i32).write_unaligned(0); // lLbound = 0
            let psa = SafeArrayCreate(VT_I4, 1, bounds.as_ptr());
            assert!(!psa.is_null());
            let mut var = [0u8; 24];
            VariantInit(var.as_mut_ptr());
            (var.as_mut_ptr() as *mut u16).write_unaligned(0x2000 | VT_I4);
            (var.as_mut_ptr().add(8) as *mut *mut u8).write_unaligned(psa);
            assert_eq!(VariantClear(var.as_mut_ptr()), crate::types::S_OK);
            assert_eq!((var.as_ptr() as *const u16).read_unaligned(), VT_EMPTY);

            // 2. VariantChangeType: I4 -> BSTR -> R8 -> BOOL -> I2 (including in-place)
            let mut v_i4 = [0u8; 24];
            VariantInit(v_i4.as_mut_ptr());
            (v_i4.as_mut_ptr() as *mut u16).write_unaligned(VT_I4);
            (v_i4.as_mut_ptr().add(8) as *mut i32).write_unaligned(-42);

            let mut v_str = [0u8; 24];
            VariantInit(v_str.as_mut_ptr());
            assert_eq!(VariantChangeType(v_str.as_mut_ptr(), v_i4.as_ptr(), 0, VT_BSTR), crate::types::S_OK);
            assert_eq!((v_str.as_ptr() as *const u16).read_unaligned(), VT_BSTR);
            let bstr = (v_str.as_ptr().add(8) as *const *const u16).read_unaligned();
            assert_eq!(crate::strings::from_wide(bstr), "-42");

            // In-place BSTR -> R8
            assert_eq!(VariantChangeType(v_str.as_mut_ptr(), v_str.as_ptr(), 0, VT_R8), crate::types::S_OK);
            assert_eq!((v_str.as_ptr() as *const u16).read_unaligned(), VT_R8);
            assert_eq!((v_str.as_ptr().add(8) as *const f64).read_unaligned(), -42.0);

            // R8 -> BOOL (-1 = VARIANT_TRUE)
            let mut v_bool = [0u8; 24];
            VariantInit(v_bool.as_mut_ptr());
            assert_eq!(VariantChangeType(v_bool.as_mut_ptr(), v_str.as_ptr(), 0, VT_BOOL), crate::types::S_OK);
            assert_eq!((v_bool.as_ptr().add(8) as *const i16).read_unaligned(), -1);

            // Overflow check: -42 -> VT_UI4 returns DISP_E_OVERFLOW
            let mut v_u4 = [0u8; 24];
            VariantInit(v_u4.as_mut_ptr());
            assert_eq!(VariantChangeType(v_u4.as_mut_ptr(), v_i4.as_ptr(), 0, VT_UI4), DISP_E_OVERFLOW);
        }
    }

    #[test]
    fn variant_copy_copyind_and_clear_all_kinds_both_orders_no_double_free() {
        use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

        #[repr(C)]
        struct MockComObj {
            vtbl: *const usize,
            refs: AtomicU32,
        }
        unsafe extern "win64" fn mock_qi(
            _this: *mut MockComObj,
            _riid: *const c_void,
            _ppv: *mut *mut c_void,
        ) -> i32 {
            crate::types::E_NOINTERFACE
        }
        unsafe extern "win64" fn mock_add_ref(this: *mut MockComObj) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn mock_release(this: *mut MockComObj) -> u32 {
            let prev = (*this).refs.fetch_sub(1, Ordering::AcqRel);
            assert!(prev > 0, "double Release detected on MockComObj");
            prev - 1
        }
        let com_vtbl: [usize; 3] = [
            mock_qi as *const () as usize,
            mock_add_ref as *const () as usize,
            mock_release as *const () as usize,
        ];

        // Mock IRecordInfo (19 vtable slots: 0..2 IUnknown, 5 RecordCopy, 16 RecordCreate, 18 RecordDestroy).
        #[repr(C)]
        struct MockRecInfo {
            vtbl: *const usize,
            refs: AtomicU32,
            live_records: AtomicI32,
        }
        unsafe extern "win64" fn ri_add_ref(this: *mut MockRecInfo) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn ri_release(this: *mut MockRecInfo) -> u32 {
            let prev = (*this).refs.fetch_sub(1, Ordering::AcqRel);
            assert!(prev > 0, "double Release on MockRecInfo");
            prev - 1
        }
        unsafe extern "win64" fn ri_record_copy(
            _this: *mut MockRecInfo,
            src: *const u64,
            dst: *mut u64,
        ) -> i32 {
            if src.is_null() || dst.is_null() {
                return crate::types::E_INVALIDARG;
            }
            std::ptr::write_unaligned(dst, std::ptr::read_unaligned(src));
            crate::types::S_OK
        }
        unsafe extern "win64" fn ri_record_create(this: *mut MockRecInfo) -> *mut c_void {
            (*this).live_records.fetch_add(1, Ordering::AcqRel);
            crate::heap::alloc(8, true)
        }
        unsafe extern "win64" fn ri_record_destroy(
            this: *mut MockRecInfo,
            rec: *mut c_void,
        ) -> i32 {
            let prev = (*this).live_records.fetch_sub(1, Ordering::AcqRel);
            assert!(prev > 0, "double RecordDestroy detected");
            crate::heap::free(rec);
            crate::types::S_OK
        }
        let mut ri_vtbl = [0usize; 19];
        ri_vtbl[0] = mock_qi as *const () as usize;
        ri_vtbl[1] = ri_add_ref as *const () as usize;
        ri_vtbl[2] = ri_release as *const () as usize;
        ri_vtbl[IRECORDINFO_RECORD_COPY] = ri_record_copy as *const () as usize;
        ri_vtbl[IRECORDINFO_RECORD_CREATE] = ri_record_create as *const () as usize;
        ri_vtbl[IRECORDINFO_RECORD_DESTROY] = ri_record_destroy as *const () as usize;

        for clear_src_first in [true, false] {
            unsafe {
                // 1. VT_BSTR: deep copy & clear in both orders
                let w_hello = crate::strings::to_wide_z("HelloVariant");
                let mut src = [0u8; 24];
                let mut dst = [0u8; 24];
                VariantInit(src.as_mut_ptr());
                VariantInit(dst.as_mut_ptr());
                (src.as_mut_ptr() as *mut u16).write_unaligned(VT_BSTR);
                (src.as_mut_ptr().add(8) as *mut *mut u16)
                    .write_unaligned(SysAllocString(w_hello.as_ptr()));
                assert_eq!(VariantCopy(dst.as_mut_ptr(), src.as_ptr()), crate::types::S_OK);
                let src_p = (src.as_ptr().add(8) as *const *mut u16).read_unaligned();
                let dst_p = (dst.as_ptr().add(8) as *const *mut u16).read_unaligned();
                assert_ne!(src_p, dst_p);
                // Self-copy is a safe no-op.
                assert_eq!(VariantCopy(dst.as_mut_ptr(), dst.as_ptr()), crate::types::S_OK);
                assert_eq!((dst.as_ptr().add(8) as *const *mut u16).read_unaligned(), dst_p);

                if clear_src_first {
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                    assert_eq!(crate::strings::from_wide(dst_p), "HelloVariant");
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                } else {
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                    assert_eq!(crate::strings::from_wide(src_p), "HelloVariant");
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                }

                // 2. VT_ARRAY | VT_I4: deep copy via SafeArrayCopy & clear in both orders
                let psa = SafeArrayCreateVector(VT_I4, 0, 3);
                assert!(!psa.is_null());
                for (i, val) in [10i32, 20, 30].iter().enumerate() {
                    let idx = i as i32;
                    assert_eq!(
                        SafeArrayPutElement(psa, &idx, val as *const i32 as *const c_void),
                        crate::types::S_OK
                    );
                }
                VariantInit(src.as_mut_ptr());
                VariantInit(dst.as_mut_ptr());
                (src.as_mut_ptr() as *mut u16).write_unaligned(VT_ARRAY | VT_I4);
                (src.as_mut_ptr().add(8) as *mut *mut u8).write_unaligned(psa);
                assert_eq!(VariantCopy(dst.as_mut_ptr(), src.as_ptr()), crate::types::S_OK);
                let psa_dst = (dst.as_ptr().add(8) as *const *mut u8).read_unaligned();
                assert_ne!(psa, psa_dst);

                let check_i4_sa = |arr: *mut u8| {
                    for (i, expected) in [10i32, 20, 30].iter().enumerate() {
                        let idx = i as i32;
                        let mut got = 0i32;
                        assert_eq!(
                            SafeArrayGetElement(arr, &idx, &mut got as *mut i32 as *mut c_void),
                            crate::types::S_OK
                        );
                        assert_eq!(got, *expected);
                    }
                };
                if clear_src_first {
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                    check_i4_sa(psa_dst);
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                } else {
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                    check_i4_sa(psa);
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                }

                // 3. VT_ARRAY | VT_BSTR: deep-copies both SAFEARRAY and each BSTR element
                let psa_bstr = SafeArrayCreateVector(VT_BSTR, 0, 2);
                let mut raw_data: *mut c_void = std::ptr::null_mut();
                assert_eq!(SafeArrayAccessData(psa_bstr, &mut raw_data), crate::types::S_OK);
                let b0 = SysAllocString(crate::strings::to_wide_z("alpha").as_ptr());
                let b1 = SysAllocString(crate::strings::to_wide_z("beta").as_ptr());
                (raw_data as *mut *mut u16).write_unaligned(b0);
                (raw_data as *mut *mut u16).add(1).write_unaligned(b1);
                assert_eq!(SafeArrayUnaccessData(psa_bstr), crate::types::S_OK);

                VariantInit(src.as_mut_ptr());
                VariantInit(dst.as_mut_ptr());
                (src.as_mut_ptr() as *mut u16).write_unaligned(VT_ARRAY | VT_BSTR);
                (src.as_mut_ptr().add(8) as *mut *mut u8).write_unaligned(psa_bstr);
                assert_eq!(VariantCopy(dst.as_mut_ptr(), src.as_ptr()), crate::types::S_OK);
                let psa_bstr_dst = (dst.as_ptr().add(8) as *const *mut u8).read_unaligned();
                assert_ne!(psa_bstr, psa_bstr_dst);

                let check_bstr_sa = |arr: *mut u8| {
                    let mut p: *mut c_void = std::ptr::null_mut();
                    assert_eq!(SafeArrayAccessData(arr, &mut p), crate::types::S_OK);
                    let s0 = (p as *const *const u16).read_unaligned();
                    let s1 = (p as *const *const u16).add(1).read_unaligned();
                    assert_eq!(crate::strings::from_wide(s0), "alpha");
                    assert_eq!(crate::strings::from_wide(s1), "beta");
                    assert_eq!(SafeArrayUnaccessData(arr), crate::types::S_OK);
                };
                if clear_src_first {
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                    check_bstr_sa(psa_bstr_dst);
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                } else {
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                    check_bstr_sa(psa_bstr);
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                }

                // 4. VT_UNKNOWN and VT_DISPATCH: AddRef on copy, Release on clear in both orders
                for vt_com in [VT_UNKNOWN, VT_DISPATCH] {
                    let mut obj = MockComObj {
                        vtbl: com_vtbl.as_ptr(),
                        refs: AtomicU32::new(1),
                    };
                    let obj_ptr = &mut obj as *mut MockComObj as *mut c_void;
                    VariantInit(src.as_mut_ptr());
                    VariantInit(dst.as_mut_ptr());
                    (src.as_mut_ptr() as *mut u16).write_unaligned(vt_com);
                    (src.as_mut_ptr().add(8) as *mut *mut c_void).write_unaligned(obj_ptr);

                    assert_eq!(VariantCopy(dst.as_mut_ptr(), src.as_ptr()), crate::types::S_OK);
                    assert_eq!(obj.refs.load(Ordering::Acquire), 2);
                    if clear_src_first {
                        assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                        assert_eq!(obj.refs.load(Ordering::Acquire), 1);
                        assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                    } else {
                        assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                        assert_eq!(obj.refs.load(Ordering::Acquire), 1);
                        assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                    }
                    assert_eq!(obj.refs.load(Ordering::Acquire), 0);
                }

                // 5. VT_BYREF (scalar, BSTR, SAFEARRAY, IUnknown, VT_VARIANT):
                //    VariantCopy copies pointer without taking ownership;
                //    VariantCopyInd dereferences into an owned copy.
                let mut i4_target = 777i32;
                let mut byref_i4 = [0u8; 24];
                VariantInit(byref_i4.as_mut_ptr());
                (byref_i4.as_mut_ptr() as *mut u16).write_unaligned(VT_BYREF | VT_I4);
                (byref_i4.as_mut_ptr().add(8) as *mut *mut i32).write_unaligned(&mut i4_target);

                let mut byref_copy = [0u8; 24];
                VariantInit(byref_copy.as_mut_ptr());
                assert_eq!(VariantCopy(byref_copy.as_mut_ptr(), byref_i4.as_ptr()), crate::types::S_OK);
                assert_eq!((byref_copy.as_ptr() as *const u16).read_unaligned(), VT_BYREF | VT_I4);
                assert_eq!(
                    (byref_copy.as_ptr().add(8) as *const *mut i32).read_unaligned(),
                    &mut i4_target as *mut i32
                );

                // VariantCopyInd resolves VT_BYREF|VT_I4 -> owned VT_I4 (also in-place!).
                let mut ind_out = [0u8; 24];
                VariantInit(ind_out.as_mut_ptr());
                assert_eq!(VariantCopyInd(ind_out.as_mut_ptr(), byref_i4.as_ptr()), crate::types::S_OK);
                assert_eq!((ind_out.as_ptr() as *const u16).read_unaligned(), VT_I4);
                assert_eq!((ind_out.as_ptr().add(8) as *const i32).read_unaligned(), 777);
                assert_eq!(VariantCopyInd(byref_copy.as_mut_ptr(), byref_copy.as_ptr()), crate::types::S_OK);
                assert_eq!((byref_copy.as_ptr() as *const u16).read_unaligned(), VT_I4);
                assert_eq!((byref_copy.as_ptr().add(8) as *const i32).read_unaligned(), 777);
                assert_eq!(VariantClear(byref_i4.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(VariantClear(byref_copy.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(VariantClear(ind_out.as_mut_ptr()), crate::types::S_OK);

                // VT_BYREF | VT_ARRAY | VT_I4: VariantCopy does not free; VariantCopyInd deep-copies.
                let mut sa_owner = SafeArrayCreateVector(VT_I4, 0, 1);
                let idx0 = 0i32;
                let v99 = 99i32;
                assert_eq!(SafeArrayPutElement(sa_owner, &idx0, &v99 as *const i32 as *const c_void), crate::types::S_OK);
                let mut byref_sa = [0u8; 24];
                VariantInit(byref_sa.as_mut_ptr());
                (byref_sa.as_mut_ptr() as *mut u16).write_unaligned(VT_BYREF | VT_ARRAY | VT_I4);
                (byref_sa.as_mut_ptr().add(8) as *mut *mut *mut u8).write_unaligned(&mut sa_owner);

                let mut byref_sa_copy = [0u8; 24];
                VariantInit(byref_sa_copy.as_mut_ptr());
                assert_eq!(VariantCopy(byref_sa_copy.as_mut_ptr(), byref_sa.as_ptr()), crate::types::S_OK);
                assert_eq!(VariantCopyInd(ind_out.as_mut_ptr(), byref_sa.as_ptr()), crate::types::S_OK);
                assert_eq!((ind_out.as_ptr() as *const u16).read_unaligned(), VT_ARRAY | VT_I4);
                let ind_psa = (ind_out.as_ptr().add(8) as *const *mut u8).read_unaligned();
                assert_ne!(ind_psa, sa_owner);

                // Clearing both VT_BYREF variants must NOT destroy `sa_owner`.
                assert_eq!(VariantClear(byref_sa.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(VariantClear(byref_sa_copy.as_mut_ptr()), crate::types::S_OK);
                let mut check = 0i32;
                assert_eq!(SafeArrayGetElement(sa_owner, &idx0, &mut check as *mut i32 as *mut c_void), crate::types::S_OK);
                assert_eq!(check, 99);
                if clear_src_first {
                    assert_eq!(SafeArrayDestroy(sa_owner), crate::types::S_OK);
                    assert_eq!(SafeArrayGetElement(ind_psa, &idx0, &mut check as *mut i32 as *mut c_void), crate::types::S_OK);
                    assert_eq!(check, 99);
                    assert_eq!(VariantClear(ind_out.as_mut_ptr()), crate::types::S_OK);
                } else {
                    assert_eq!(VariantClear(ind_out.as_mut_ptr()), crate::types::S_OK);
                    assert_eq!(SafeArrayGetElement(sa_owner, &idx0, &mut check as *mut i32 as *mut c_void), crate::types::S_OK);
                    assert_eq!(check, 99);
                    assert_eq!(SafeArrayDestroy(sa_owner), crate::types::S_OK);
                }

                // VT_BYREF | VT_UNKNOWN: VariantCopy leaves refs=1; VariantCopyInd AddRefs to 2.
                let mut ref_obj = MockComObj { vtbl: com_vtbl.as_ptr(), refs: AtomicU32::new(1) };
                let mut ref_obj_ptr = &mut ref_obj as *mut MockComObj as *mut c_void;
                let mut byref_unk = [0u8; 24];
                VariantInit(byref_unk.as_mut_ptr());
                (byref_unk.as_mut_ptr() as *mut u16).write_unaligned(VT_BYREF | VT_UNKNOWN);
                (byref_unk.as_mut_ptr().add(8) as *mut *mut *mut c_void).write_unaligned(&mut ref_obj_ptr);
                assert_eq!(VariantCopy(dst.as_mut_ptr(), byref_unk.as_ptr()), crate::types::S_OK);
                assert_eq!(ref_obj.refs.load(Ordering::Acquire), 1);
                assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(ref_obj.refs.load(Ordering::Acquire), 1);
                assert_eq!(VariantCopyInd(dst.as_mut_ptr(), byref_unk.as_ptr()), crate::types::S_OK);
                assert_eq!((dst.as_ptr() as *const u16).read_unaligned(), VT_UNKNOWN);
                assert_eq!(ref_obj.refs.load(Ordering::Acquire), 2);
                assert_eq!(VariantClear(byref_unk.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(ref_obj.refs.load(Ordering::Acquire), 2);
                assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(ref_obj.refs.load(Ordering::Acquire), 1);

                // VT_BYREF | VT_VARIANT pointing to an owned VT_BSTR resolves cleanly.
                let mut inner_bstr_v = [0u8; 24];
                VariantInit(inner_bstr_v.as_mut_ptr());
                (inner_bstr_v.as_mut_ptr() as *mut u16).write_unaligned(VT_BSTR);
                (inner_bstr_v.as_mut_ptr().add(8) as *mut *mut u16)
                    .write_unaligned(SysAllocString(crate::strings::to_wide_z("Nested").as_ptr()));
                let mut byref_var = [0u8; 24];
                VariantInit(byref_var.as_mut_ptr());
                (byref_var.as_mut_ptr() as *mut u16).write_unaligned(VT_BYREF | VT_VARIANT);
                (byref_var.as_mut_ptr().add(8) as *mut *mut u8).write_unaligned(inner_bstr_v.as_mut_ptr());
                assert_eq!(VariantCopyInd(dst.as_mut_ptr(), byref_var.as_ptr()), crate::types::S_OK);
                assert_eq!((dst.as_ptr() as *const u16).read_unaligned(), VT_BSTR);
                let resolved_b = (dst.as_ptr().add(8) as *const *const u16).read_unaligned();
                assert_ne!(resolved_b, (inner_bstr_v.as_ptr().add(8) as *const *const u16).read_unaligned());
                assert_eq!(VariantClear(byref_var.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(VariantClear(inner_bstr_v.as_mut_ptr()), crate::types::S_OK);
                assert_eq!(crate::strings::from_wide(resolved_b), "Nested");
                assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);

                // 6. VT_RECORD: copies via IRecordInfo::RecordCreate + RecordCopy, clears via RecordDestroy + Release.
                let mut rec_info = MockRecInfo {
                    vtbl: ri_vtbl.as_ptr(),
                    refs: AtomicU32::new(1),
                    live_records: AtomicI32::new(0),
                };
                let rec_info_ptr = &mut rec_info as *mut MockRecInfo as *mut c_void;
                let rec0 = ri_record_create(&mut rec_info);
                (rec0 as *mut u64).write_unaligned(0xDEAD_BEEF_CAFE_BABE);
                VariantInit(src.as_mut_ptr());
                VariantInit(dst.as_mut_ptr());
                (src.as_mut_ptr() as *mut u16).write_unaligned(VT_RECORD);
                (src.as_mut_ptr().add(8) as *mut *mut c_void).write_unaligned(rec0);
                (src.as_mut_ptr().add(16) as *mut *mut c_void).write_unaligned(rec_info_ptr);

                assert_eq!(VariantCopy(dst.as_mut_ptr(), src.as_ptr()), crate::types::S_OK);
                let rec1 = (dst.as_ptr().add(8) as *const *mut c_void).read_unaligned();
                assert_ne!(rec0, rec1);
                assert_eq!(rec_info.refs.load(Ordering::Acquire), 2);
                assert_eq!(rec_info.live_records.load(Ordering::Acquire), 2);

                if clear_src_first {
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                    assert_eq!(rec_info.live_records.load(Ordering::Acquire), 1);
                    assert_eq!((rec1 as *const u64).read_unaligned(), 0xDEAD_BEEF_CAFE_BABE);
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                } else {
                    assert_eq!(VariantClear(dst.as_mut_ptr()), crate::types::S_OK);
                    assert_eq!(rec_info.live_records.load(Ordering::Acquire), 1);
                    assert_eq!((rec0 as *const u64).read_unaligned(), 0xDEAD_BEEF_CAFE_BABE);
                    assert_eq!(VariantClear(src.as_mut_ptr()), crate::types::S_OK);
                }
                assert_eq!(rec_info.refs.load(Ordering::Acquire), 0);
                assert_eq!(rec_info.live_records.load(Ordering::Acquire), 0);

                // Invalid VT_RECORD (non-null pvRecord with null pRecInfo) fails loudly with DISP_E_BADVARTYPE.
                let mut bad_rec = [0u8; 24];
                VariantInit(bad_rec.as_mut_ptr());
                (bad_rec.as_mut_ptr() as *mut u16).write_unaligned(VT_RECORD);
                (bad_rec.as_mut_ptr().add(8) as *mut usize).write_unaligned(0x1234);
                assert_eq!(VariantCopy(dst.as_mut_ptr(), bad_rec.as_ptr()), DISP_E_BADVARTYPE);

                // 7. SafeArrayPutElement / SafeArrayGetElement deep-copy BSTRs, AddRef COM, and copy VARIANTs.
                let sa_b = SafeArrayCreateVector(VT_BSTR, 0, 1);
                let idx0 = 0i32;
                let src_b0 = SysAllocString(crate::strings::to_wide_z("first").as_ptr());
                let src_b1 = SysAllocString(crate::strings::to_wide_z("second").as_ptr());
                assert_eq!(SafeArrayPutElement(sa_b, &idx0, src_b0 as *const c_void), crate::types::S_OK);
                // Overwriting replaces and frees the first internal copy.
                assert_eq!(SafeArrayPutElement(sa_b, &idx0, src_b1 as *const c_void), crate::types::S_OK);
                SysFreeString(src_b0);
                SysFreeString(src_b1);
                let mut got_b: *mut u16 = std::ptr::null_mut();
                assert_eq!(
                    SafeArrayGetElement(sa_b, &idx0, &mut got_b as *mut *mut u16 as *mut c_void),
                    crate::types::S_OK
                );
                assert!(!got_b.is_null());
                if clear_src_first {
                    assert_eq!(SafeArrayDestroy(sa_b), crate::types::S_OK);
                    assert_eq!(crate::strings::from_wide(got_b), "second");
                    SysFreeString(got_b);
                } else {
                    assert_eq!(crate::strings::from_wide(got_b), "second");
                    SysFreeString(got_b);
                    assert_eq!(SafeArrayDestroy(sa_b), crate::types::S_OK);
                }

                // SafeArrayPutElement / SafeArrayGetElement on VT_UNKNOWN: AddRef on put & get, Release on overwrite/destroy.
                let sa_u = SafeArrayCreateVector(VT_UNKNOWN, 0, 1);
                let mut com_elem = MockComObj { vtbl: com_vtbl.as_ptr(), refs: AtomicU32::new(1) };
                let com_ptr = &mut com_elem as *mut MockComObj as *mut c_void;
                assert_eq!(SafeArrayPutElement(sa_u, &idx0, com_ptr), crate::types::S_OK);
                assert_eq!(com_elem.refs.load(Ordering::Acquire), 2);
                // Self-overwrite preserves refcount cleanly.
                assert_eq!(SafeArrayPutElement(sa_u, &idx0, com_ptr), crate::types::S_OK);
                assert_eq!(com_elem.refs.load(Ordering::Acquire), 2);
                let mut got_u: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    SafeArrayGetElement(sa_u, &idx0, &mut got_u as *mut *mut c_void as *mut c_void),
                    crate::types::S_OK
                );
                assert_eq!(got_u, com_ptr);
                assert_eq!(com_elem.refs.load(Ordering::Acquire), 3);
                assert_eq!(SafeArrayDestroy(sa_u), crate::types::S_OK);
                assert_eq!(com_elem.refs.load(Ordering::Acquire), 2);
                crate::com::com_call_release(got_u);
                assert_eq!(com_elem.refs.load(Ordering::Acquire), 1);

                // SafeArrayPutElement / SafeArrayGetElement on VT_VARIANT with owned BSTR.
                let sa_v = SafeArrayCreateVector(VT_VARIANT, 0, 1);
                let mut elem_v = [0u8; 24];
                VariantInit(elem_v.as_mut_ptr());
                (elem_v.as_mut_ptr() as *mut u16).write_unaligned(VT_BSTR);
                (elem_v.as_mut_ptr().add(8) as *mut *mut u16)
                    .write_unaligned(SysAllocString(crate::strings::to_wide_z("var-in-sa").as_ptr()));
                assert_eq!(SafeArrayPutElement(sa_v, &idx0, elem_v.as_ptr() as *const c_void), crate::types::S_OK);
                assert_eq!(VariantClear(elem_v.as_mut_ptr()), crate::types::S_OK);
                let mut got_v = [0u8; 24];
                assert_eq!(SafeArrayGetElement(sa_v, &idx0, got_v.as_mut_ptr() as *mut c_void), crate::types::S_OK);
                assert_eq!(SafeArrayDestroy(sa_v), crate::types::S_OK);
                assert_eq!((got_v.as_ptr() as *const u16).read_unaligned(), VT_BSTR);
                let got_vb = (got_v.as_ptr().add(8) as *const *const u16).read_unaligned();
                assert_eq!(crate::strings::from_wide(got_vb), "var-in-sa");
                assert_eq!(VariantClear(got_v.as_mut_ptr()), crate::types::S_OK);

                // 8. VariantChangeType propagates DISP_E_ARRAYISLOCKED without clobbering locked dst.
                let locked_sa = SafeArrayCreateVector(VT_I4, 0, 1);
                assert_eq!(SafeArrayLock(locked_sa), crate::types::S_OK);
                let mut locked_dst = [0u8; 24];
                VariantInit(locked_dst.as_mut_ptr());
                (locked_dst.as_mut_ptr() as *mut u16).write_unaligned(VT_ARRAY | VT_I4);
                (locked_dst.as_mut_ptr().add(8) as *mut *mut u8).write_unaligned(locked_sa);
                let mut src_num = [0u8; 24];
                VariantInit(src_num.as_mut_ptr());
                (src_num.as_mut_ptr() as *mut u16).write_unaligned(VT_I4);
                (src_num.as_mut_ptr().add(8) as *mut i32).write_unaligned(123);
                assert_eq!(
                    VariantChangeType(locked_dst.as_mut_ptr(), src_num.as_ptr(), 0, VT_BSTR),
                    DISP_E_ARRAYISLOCKED
                );
                assert_eq!((locked_dst.as_ptr() as *const u16).read_unaligned(), VT_ARRAY | VT_I4);
                assert_eq!((locked_dst.as_ptr().add(8) as *const *mut u8).read_unaligned(), locked_sa);
                assert_eq!(SafeArrayUnlock(locked_sa), crate::types::S_OK);
                assert_eq!(VariantClear(locked_dst.as_mut_ptr()), crate::types::S_OK);
            }
        }
    }
}
