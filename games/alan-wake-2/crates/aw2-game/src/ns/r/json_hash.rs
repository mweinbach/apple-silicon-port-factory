//! Case-insensitive FNV-1a hashes of a JSON value's string form, plus
//! case-insensitive RapidJSON object member lookup (`0x14247f340`).

use super::inplace_string::InplaceString;
use crate::exe;

/// `void toString(const rapidjson::Value&, r::InplaceString<256>&)` (inferred).
const VALUE_TO_STRING_VA: u64 = 0x1_413c_1770;
/// `r::InplaceString<>::~InplaceString()`.
const STRING_DTOR_VA: u64 = 0x1_4240_ab10;

/// A local `r::InplaceString<256>` as the originals lay it out at `[rsp+0x20]`.
#[repr(C, align(8))]
struct Local256([u8; 8 + 256 + 8]);

/// Build the string, run `hash` over its bytes, destroy it (including on unwind).
#[inline(always)]
unsafe fn with_value_string<R>(value: *const u8, hash: impl FnOnce(&[u8]) -> R) -> R {
    let mut s = Local256([0; 8 + 256 + 8]);
    let p = s.0.as_mut_ptr();
    // `mov qword [rsp+0x20], 0x20000020; mov byte [rsp+0x28], 0`
    std::ptr::write(p as *mut u64, 0x2000_0020);
    *p.add(8) = 0;
    let this = p as *mut InplaceString;
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(VALUE_TO_STRING_VA) as usize,
        &[value as u64, this as u64],
        &[],
    ) {
        exe::call_va!(STRING_DTOR_VA, (this));
        e.rethrow();
    }
    let bits = std::ptr::read(p as *const u32);
    let code = bits >> 24;
    let inline = if code < 0x80 { code * 8 } else { (code << 8).wrapping_sub(0x7c00) };
    let data = if (bits & 0xff_ffff) << 3 > inline { std::ptr::read(p.add(8) as *const *const u8) } else { p.add(8) as *const u8 };
    let len = std::ptr::read(p.add(4) as *const u32) as usize;
    let r = hash(if len == 0 { &[] } else { std::slice::from_raw_parts(data, len) });
    exe::call_va!(STRING_DTOR_VA, (this));
    r
}

/// 32-bit case-folded FNV-1a of a JSON value's string form; returns `true`.
///
/// Original: 0x1413c32d0 (181 bytes). Per byte: `h = (h ^ ((i8)c | 0x20)) *
/// 0x1000193` on 32 bits, basis 0x811c9dc5; `*out = 0` for an empty string.
pub unsafe extern "win64" fn json_hash_nocase_fnv32(value: *const u8, out: *mut u32) -> bool {
    let h = with_value_string(value, |s| {
        if s.is_empty() {
            return 0u32;
        }
        let mut h = 0x811c_9dc5u32;
        for &c in s {
            h = (h ^ ((c as i8 as i32 as u32) | 0x20)).wrapping_mul(0x0100_0193);
        }
        h
    });
    exe::wr::<u32>(out as usize, h);
    true
}

/// 64-bit case-folded FNV-1a of a JSON value's string form; returns `true`.
///
/// Original: 0x1413c3390 (199 bytes). Per byte: `h = (h ^ ((i64)(i8)c |
/// 0x20)) * 0x100000001b3`, basis 0xcbf29ce484222325; 0 for an empty string.
pub unsafe extern "win64" fn json_hash_nocase_fnv64(value: *const u8, out: *mut u64) -> bool {
    let h = with_value_string(value, |s| {
        if s.is_empty() {
            return 0u64;
        }
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &c in s {
            h = (h ^ ((c as i8 as i64 as u64) | 0x20)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    });
    exe::wr::<u64>(out as usize, h);
    true
}

/// 256-byte case-folding table at `0x1427e54e0`.
const CASE_TABLE_VA: u64 = 0x1_427e_54e0;
const PTR48_MASK: usize = 0xffff_ffff_ffff;
const SHORT_STRING_FLAG: u16 = 0x1000;

/// Case-insensitive RapidJSON object member lookup by C-string `name` (`rdx`),
/// with a 1-entry sequential cursor cache at `[this + 0x10]` (last matched
/// index) and `[this + 0x14]` (cached member count).
///
/// Original: 0x14247f340 (344 bytes, 2,741 call sites).
pub unsafe extern "win64" fn json_find_member_nocase(this: *mut u8, name: *const u8) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let val = rd::<usize>(t + 0x08);
    let tbl = exe::ptr::<u8>(CASE_TABLE_VA) as usize;
    let members_base = rd::<usize>(val + 0x08) & PTR48_MASK;
    let count = rd::<u32>(val);

    // Fast path: check `last_idx + 1` when `cached_count == count`.
    if rd::<u32>(t + 0x14) == count {
        let next_idx = rd::<u32>(t + 0x10).wrapping_add(1);
        if next_idx < count {
            let entry = members_base.wrapping_add((next_idx as usize) << 5);
            wr::<u32>(t + 0x10, next_idx);
            let key_str = if rd::<u16>(entry + 0x0e) & SHORT_STRING_FLAG != 0 {
                entry
            } else {
                rd::<usize>(entry + 0x08) & PTR48_MASK
            };
            let mut i = 0usize;
            let matched = loop {
                let ca = rd::<u8>(tbl + rd::<u8>(key_str.wrapping_add(i)) as usize);
                let cb = rd::<u8>(tbl + rd::<u8>((name as usize).wrapping_add(i)) as usize);
                i += 1;
                if ca == 0 || ca != cb {
                    break ca == cb;
                }
            };
            if matched {
                return entry as *mut u8;
            }
        }
    }

    let mut name_len = 0usize;
    while rd::<u8>((name as usize).wrapping_add(name_len)) != 0 {
        name_len += 1;
    }

    for idx in 0..count {
        let entry = members_base.wrapping_add((idx as usize) << 5);
        let is_short = (rd::<u16>(entry + 0x0e) & SHORT_STRING_FLAG) != 0;
        let key_len: u32 = if is_short {
            0xdu32.wrapping_sub(rd::<i8>(entry + 0x0d) as i32 as u32)
        } else {
            rd::<u32>(entry)
        };
        if key_len as usize == name_len {
            let key_str = if is_short {
                entry
            } else {
                rd::<usize>(entry + 0x08) & PTR48_MASK
            };
            let mut i = 0usize;
            let matched = loop {
                let ca = rd::<u8>(tbl + rd::<u8>((name as usize).wrapping_add(i)) as usize);
                let cb = rd::<u8>(tbl + rd::<u8>(key_str.wrapping_add(i)) as usize);
                i += 1;
                if ca == 0 || ca != cb {
                    break ca == cb;
                }
            };
            if matched {
                wr::<u32>(t + 0x10, idx);
                wr::<u32>(t + 0x14, count);
                return entry as *mut u8;
            }
        }
    }
    std::ptr::null_mut()
}

/// Set the embedded `InplaceString` at `(*this) + 0x140` to `cstr` (if `*this`
/// is non-null).
///
/// Original: 0x14247f4a0 (144 bytes).
pub unsafe extern "win64" fn json_set_error_cstr(this: *mut u8, cstr: *const u8) {
    let inner = exe::rd::<usize>(this as usize);
    if inner != 0 {
        let _ = super::inplace_string::inplace_string_assign_cstr(
            (inner + 0x140) as *mut InplaceString,
            cstr,
        );
    }
}

/// Set the embedded `InplaceString` at `(*this) + 0x140` to `s->c_str()` (if
/// `*this` is non-null).
///
/// Original: 0x14247f540 (206 bytes).
pub unsafe extern "win64" fn json_set_error_inplace_string(
    this: *mut u8,
    s: *const InplaceString,
) {
    let cstr = super::inplace_string::inplace_string_data(s);
    json_set_error_cstr(this, cstr);
}

/// Assign `cstr` to the embedded `InplaceString` at `this + OFF`.
///
/// Originals: 0x142482460 / 0x1424825a0 (`OFF = 0x50`, 123 bytes),
/// 0x142482500 / 0x142482640 (`OFF = 0x40`, 123 bytes).
pub unsafe extern "win64" fn json_writer_set_str_at<const OFF: usize>(
    this: *mut u8,
    cstr: *const u8,
) {
    let _ = super::inplace_string::inplace_string_assign_cstr(
        (this as usize + OFF) as *mut InplaceString,
        cstr,
    );
}

/// Default constructor for an `r::JSONException` subclass with vtable `VTBL_VA`.
///
/// Originals: 0x1401e7b60 (`0x143079f28`), 0x1413c16b0 (`0x14340ce18`),
/// 0x1413c16e0 (`0x14340cdd0`), 0x1413c1710 (`0x14340ce00`),
/// 0x1413c1740 (`0x14340cde8`), 0x1413c32a0 (`0x14340ce30`).
pub unsafe extern "win64" fn json_exc_default_ctor<const VTBL_VA: u64>(
    this: *mut u8,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x10, 0);
    wr::<usize>(t + 0x08, exe::ptr::<u8>(0x1_4340_ed48) as usize);
    wr::<usize>(t, exe::ptr::<u8>(VTBL_VA) as usize);
    this
}

const EMPTY_CSTR_VA: u64 = 0x1_426c_dcb0;
const UNSPECIFIED_EXC_VA: u64 = 0x1_4340_ed48;
const EXC_FORMAT_MSG_VA: u64 = 0x1_4249_8f00;

#[inline(always)]
unsafe fn doc_name_or_empty(doc: *const u8) -> *const u8 {
    if doc.is_null() {
        exe::ptr::<u8>(EMPTY_CSTR_VA)
    } else {
        super::inplace_string::inplace_string_data((doc as usize + 0x140) as *const InplaceString)
    }
}

/// Return the document name (`(*(this->doc + 0x140)).c_str()`), or `""` if null.
///
/// Originals: 0x1424813d0 (`DOC_OFF = 0`, 81 bytes), 0x142482000 (`DOC_OFF = 0x10`, 82 bytes).
pub unsafe extern "win64" fn json_doc_name_at<const DOC_OFF: usize>(
    this: *const u8,
) -> *const u8 {
    let doc = exe::rd::<*const u8>((this as usize) + DOC_OFF);
    doc_name_or_empty(doc)
}

/// Construct a member-related `r::JSONException` subclass (`VTBL_VA`, `FMT_VA`)
/// taking `(this, doc, member_name)`.
///
/// Originals (138 bytes each):
/// - 0x142482100 (`VTBL = 0x14340cdd0`, `FMT = 0x14340cf50`)
/// - 0x142482220 (`VTBL = 0x14340ce00`, `FMT = 0x14340ce98`)
/// - 0x1424822b0 (`VTBL = 0x14340ce18`, `FMT = 0x14340cf00`)
/// - 0x142482340 (`VTBL = 0x1433fe840`, `FMT = 0x14340cf28`, 1,324 call sites)
/// - 0x1424823d0 (`VTBL = 0x143079f28`, `FMT = 0x14340ced8`, 32 call sites)
pub unsafe extern "win64" fn json_exc_ctor_member<const VTBL_VA: u64, const FMT_VA: u64>(
    this: *mut u8,
    doc: *const u8,
    member: *const u8,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, exe::ptr::<u8>(UNSPECIFIED_EXC_VA) as usize);
    wr::<*const u8>(t + 0x10, doc);
    wr::<usize>(t, exe::ptr::<u8>(VTBL_VA) as usize);
    let dname = doc_name_or_empty(doc);
    exe::call_va!(
        EXC_FORMAT_MSG_VA,
        (
            this,
            exe::ptr::<u8>(FMT_VA),
            member as usize,
            dname as usize
        )
    );
    this
}

/// Construct `r::JSONException_NullIterator` (`0x142482190`, 138 bytes).
pub unsafe extern "win64" fn json_exc_ctor_null_iter(
    this: *mut u8,
    doc: *const u8,
    _r8: usize,
    r9: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, exe::ptr::<u8>(UNSPECIFIED_EXC_VA) as usize);
    wr::<*const u8>(t + 0x10, doc);
    wr::<usize>(t, exe::ptr::<u8>(0x1_4340_cde8) as usize);
    let dname = doc_name_or_empty(doc);
    exe::call_va!(
        EXC_FORMAT_MSG_VA,
        (
            this,
            exe::ptr::<u8>(0x1_4340_ce40),
            dname as usize,
            r9
        )
    );
    this
}

/// Construct `r::JSONException_ParseFailed` (`0x142481ec0`, 146 bytes).
pub unsafe extern "win64" fn json_exc_ctor_parse_failed(
    this: *mut u8,
    doc: *const u8,
    err_str: usize,
    line: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, exe::ptr::<u8>(UNSPECIFIED_EXC_VA) as usize);
    wr::<*const u8>(t + 0x10, doc);
    wr::<usize>(t, exe::ptr::<u8>(0x1_4340_d028) as usize);
    let dname = doc_name_or_empty(doc);
    exe::call_va!(
        EXC_FORMAT_MSG_VA,
        (
            this,
            exe::ptr::<u8>(0x1_4340_cfd0),
            dname as usize,
            line,
            err_str
        )
    );
    this
}

/// Construct `r::JSONException_SaveFailed` (`0x142481f60`, 149 bytes).
pub unsafe extern "win64" fn json_exc_ctor_save_failed(
    this: *mut u8,
    doc: *const u8,
    key: usize,
    type_str: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, exe::ptr::<u8>(UNSPECIFIED_EXC_VA) as usize);
    wr::<*const u8>(t + 0x10, doc);
    wr::<usize>(t, exe::ptr::<u8>(0x1_4340_d010) as usize);
    let dname = doc_name_or_empty(doc);
    exe::call_va!(
        EXC_FORMAT_MSG_VA,
        (
            this,
            exe::ptr::<u8>(0x1_4340_cf80),
            dname as usize,
            key,
            type_str
        )
    );
    this
}

/// Construct `r::JSONException_IndexOutOfBounds` (`0x142482060`, 146 bytes).
pub unsafe extern "win64" fn json_exc_ctor_index_oob(
    this: *mut u8,
    doc: *const u8,
    idx: usize,
    count: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, exe::ptr::<u8>(UNSPECIFIED_EXC_VA) as usize);
    wr::<*const u8>(t + 0x10, doc);
    wr::<usize>(t, exe::ptr::<u8>(0x1_4340_ce30) as usize);
    let dname = doc_name_or_empty(doc);
    exe::call_va!(
        EXC_FORMAT_MSG_VA,
        (
            this,
            exe::ptr::<u8>(0x1_4340_ce58),
            idx,
            count,
            dname as usize
        )
    );
    this
}

/// `rapidjson::GenericValue::GetDouble()` (`0x1413c21b0`, 87 bytes).
pub unsafe extern "win64" fn rapidjson_value_get_double(this: *const u8) -> f64 {
    use exe::rd;
    let v = this as usize;
    let flags = rd::<u16>(v + 0x0e);
    let r = if (flags & 0x0200) != 0 {
        rd::<f64>(v)
    } else if (flags & 0x0020) != 0 {
        rd::<i32>(v) as f64
    } else if (flags & 0x0040) != 0 {
        rd::<u32>(v) as f64
    } else if (flags & 0x0080) != 0 {
        rd::<i64>(v) as f64
    } else {
        let u = rd::<u64>(v);
        if (u as i64) >= 0 {
            (u as i64) as f64
        } else {
            let half = (((u >> 1) | (u & 1)) as i64) as f64;
            half + half
        }
    };
    crate::simd_ops::set_xmm_ret(r.to_bits() as u128);
    r
}

/// `rapidjson::GenericValue::GenericValue(int64_t)` (`0x1413c36d0`, 95 bytes).
pub unsafe extern "win64" fn rapidjson_value_set_i64(this: *mut u8, val: i64) -> *mut u8 {
    use exe::wr;
    let v = this as usize;
    wr::<u64>(v + 0x08, 0);
    wr::<i64>(v, val);
    if val >= 0 {
        let mut f: u16 = if (val as u64 & 0xffff_ffff_0000_0000) == 0 {
            0x01d6
        } else {
            0x0196
        };
        if (val as u64 & 0xffff_ffff_8000_0000) == 0 {
            f |= 0x0020;
        }
        wr::<u16>(v + 0x0e, f);
    } else if val >= i32::MIN as i64 {
        wr::<u16>(v + 0x0e, 0x00b6);
    } else {
        wr::<u16>(v + 0x0e, 0x0096);
    }
    this
}

const OP_NEW_VA: u64 = 0x1_4183_4d64;
const CALLOC_R9_VA: u64 = 0x1_4020_c080;
const J_MEMSET_VA: u64 = 0x1_4183_acc2;
const J_MEMCPY_VA: u64 = 0x1_4183_acbc;
const J_FREE_SIZED_VA: u64 = 0x1_4183_4d30;
const POOL_MALLOC_VA: u64 = 0x1_413c_3800;
const ADD_MEMBER_RAW_VA: u64 = 0x1_413c_3460;

/// `rapidjson::MemoryPoolAllocator::Malloc(size)` (`0x1413c3800`, 203 bytes, 57 call sites).
pub unsafe extern "win64" fn rapidjson_pool_malloc(this: *mut u8, size: usize) -> *mut u8 {
    use exe::{rd, wr};
    if size == 0 {
        return std::ptr::null_mut();
    }
    let t = this as usize;
    let aligned = size.wrapping_add(7) & !7;
    let mut chunk = rd::<usize>(t);
    if chunk == 0 || rd::<usize>(chunk + 0x08).wrapping_add(aligned) > rd::<usize>(chunk) {
        let mut cap = rd::<usize>(t + 0x08);
        if cap <= aligned {
            cap = aligned;
        }
        if rd::<usize>(t + 0x18) == 0 {
            let base_alloc = exe::call_va!(OP_NEW_VA, (1usize) -> *mut u8);
            wr::<*mut u8>(t + 0x18, base_alloc);
            wr::<*mut u8>(t + 0x20, base_alloc);
        }
        let alloc_bytes = cap.wrapping_add(0x18);
        let new_chunk = exe::call_va!(CALLOC_R9_VA, (0usize, 0usize, 0usize, alloc_bytes) -> *mut u8);
        exe::call_va!(J_MEMSET_VA, (new_chunk, 0i32, alloc_bytes) -> *mut u8);
        if new_chunk.is_null() {
            return std::ptr::null_mut();
        }
        chunk = new_chunk as usize;
        wr::<usize>(chunk, cap);
        wr::<usize>(chunk + 0x08, 0);
        wr::<usize>(chunk + 0x10, rd::<usize>(t));
        wr::<usize>(t, chunk);
    }
    let off = rd::<usize>(chunk + 0x08);
    let ret = chunk.wrapping_add(0x18).wrapping_add(off) as *mut u8;
    wr::<usize>(chunk + 0x08, off.wrapping_add(aligned));
    ret
}

/// `rapidjson::MemoryPoolAllocator::Realloc(orig_ptr, orig_size, new_size)`
/// (`0x1413c3730`, 196 bytes).
pub unsafe extern "win64" fn rapidjson_pool_realloc(
    this: *mut u8,
    orig_ptr: *mut u8,
    orig_size: usize,
    new_size: usize,
) -> *mut u8 {
    use exe::{rd, wr};
    if orig_ptr.is_null() {
        return exe::call_va!(POOL_MALLOC_VA, (this, new_size) -> *mut u8);
    }
    if new_size == 0 {
        return std::ptr::null_mut();
    }
    let orig_aligned = orig_size.wrapping_add(7) & !7;
    let new_aligned = new_size.wrapping_add(7) & !7;
    if orig_aligned >= new_aligned {
        return orig_ptr;
    }
    let chunk = rd::<usize>(this as usize);
    let used = rd::<usize>(chunk + 0x08);
    let base = used.wrapping_sub(orig_aligned);
    if orig_ptr as usize == chunk.wrapping_add(0x18).wrapping_add(base) {
        let new_used = base.wrapping_add(new_aligned);
        if new_used <= rd::<usize>(chunk) {
            wr::<usize>(chunk + 0x08, new_used);
            return orig_ptr;
        }
    }
    let new_ptr = exe::call_va!(POOL_MALLOC_VA, (this, new_aligned) -> *mut u8);
    if new_ptr.is_null() {
        return std::ptr::null_mut();
    }
    if orig_aligned != 0 {
        exe::call_va!(J_MEMCPY_VA, (new_ptr, orig_ptr as *const u8, orig_aligned) -> *mut u8);
    }
    new_ptr
}

/// `rapidjson::MemoryPoolAllocator::~MemoryPoolAllocator()` (`0x1413c35d0`, 98 bytes, 14 call sites).
pub unsafe extern "win64" fn rapidjson_pool_dtor(this: *mut u8) {
    use exe::{rd, wr};
    let t = this as usize;
    let mut chunk = rd::<usize>(t);
    if chunk != 0 {
        while chunk != rd::<usize>(t + 0x10) {
            let next = rd::<usize>(chunk + 0x10);
            if chunk != 0 {
                exe::call_va!(0x1_4244_2de0, (chunk as *mut u8));
            }
            wr::<usize>(t, next);
            chunk = next;
            if chunk == 0 {
                break;
            }
        }
        if chunk != 0 && chunk == rd::<usize>(t + 0x10) {
            wr::<usize>(chunk + 0x08, 0);
        }
    }
    exe::call_va!(J_FREE_SIZED_VA, (rd::<*mut u8>(t + 0x20), 1usize));
}

/// `rapidjson::GenericValue::SetStringRaw(str_ref, allocator)` (`0x1413c3640`, 138 bytes).
pub unsafe extern "win64" fn rapidjson_value_set_string_raw(
    this: *mut u8,
    str_ref: *const u8,
    allocator: *mut u8,
) {
    use exe::{rd, wr};
    let v = this as usize;
    let sr = str_ref as usize;
    let len = rd::<u32>(sr + 0x08);
    let dst = if len <= 0x0d {
        wr::<u16>(v + 0x0e, 0x1c05);
        wr::<u8>(v + 0x0d, 0x0du8.wrapping_sub(len as u8));
        this
    } else {
        wr::<u16>(v + 0x0e, 0x0c05);
        wr::<u32>(v, len);
        let p = exe::call_va!(POOL_MALLOC_VA, (allocator, len.wrapping_add(1) as usize) -> *mut u8);
        wr::<u64>(v + 0x08, (rd::<u64>(v + 0x08) & 0xffff_0000_0000_0000) | (p as u64));
        p
    };
    let copy_len = rd::<u32>(sr + 0x08) as usize;
    exe::call_va!(J_MEMCPY_VA, (dst, rd::<*const u8>(sr), copy_len) -> *mut u8);
    let term_len = rd::<u32>(sr + 0x08) as usize;
    wr::<u8>((dst as usize).wrapping_add(term_len), 0);
}

/// `rapidjson::GenericValue::AddMember(name, value, allocator)` (`0x1413c3460`, 355 bytes, 42 call sites).
pub unsafe extern "win64" fn rapidjson_add_member_raw(
    this: *mut u8,
    name: *mut u8,
    value: *mut u8,
    allocator: *mut u8,
) -> *mut u8 {
    use exe::{rd, wr};
    let v = this as usize;
    let cap = rd::<u32>(v + 0x04);
    if rd::<u32>(v) >= cap {
        if cap == 0 {
            wr::<u32>(v + 0x04, 0x10);
            let p = exe::call_va!(POOL_MALLOC_VA, (allocator, 0x200usize) -> *mut u8);
            wr::<u64>(v + 0x08, (rd::<u64>(v + 0x08) & 0xffff_0000_0000_0000) | (p as u64));
        } else {
            let new_cap = cap.wrapping_add(cap.wrapping_add(1) >> 1);
            wr::<u32>(v + 0x04, new_cap);
            let orig_ptr = (rd::<usize>(v + 0x08) & PTR48_MASK) as *mut u8;
            let new_bytes = (new_cap as usize) << 5;
            let orig_bytes = (cap as usize) << 5;
            let new_ptr = if orig_ptr.is_null() {
                exe::call_va!(POOL_MALLOC_VA, (allocator, new_bytes) -> *mut u8)
            } else if new_bytes == 0 {
                std::ptr::null_mut()
            } else if orig_bytes >= new_bytes {
                orig_ptr
            } else {
                let chunk = rd::<usize>(allocator as usize);
                let used = rd::<usize>(chunk + 0x08);
                let base = used.wrapping_sub(orig_bytes);
                if orig_ptr as usize == chunk.wrapping_add(0x18).wrapping_add(base)
                    && base.wrapping_add(new_bytes) <= rd::<usize>(chunk)
                {
                    wr::<usize>(chunk + 0x08, base.wrapping_add(new_bytes));
                    orig_ptr
                } else {
                    let p = exe::call_va!(POOL_MALLOC_VA, (allocator, new_bytes) -> *mut u8);
                    if !p.is_null() {
                        if orig_bytes != 0 {
                            exe::call_va!(J_MEMCPY_VA, (p, orig_ptr as *const u8, orig_bytes) -> *mut u8);
                        }
                        p
                    } else {
                        std::ptr::null_mut()
                    }
                }
            };
            wr::<u64>(v + 0x08, (rd::<u64>(v + 0x08) & 0xffff_0000_0000_0000) | (new_ptr as u64));
        }
    }
    let members = rd::<usize>(v + 0x08) & PTR48_MASK;
    let idx = rd::<u32>(v) as usize;
    std::ptr::copy_nonoverlapping(name, (members + (idx << 5)) as *mut u8, 16);
    wr::<u16>((name as usize) + 0x0e, 0);
    let idx2 = rd::<u32>(v) as usize;
    std::ptr::copy_nonoverlapping(value, (members + (idx2 << 5) + 0x10) as *mut u8, 16);
    wr::<u16>((value as usize) + 0x0e, 0);
    wr::<u32>(v, rd::<u32>(v).wrapping_add(1));
    this
}

#[inline(always)]
unsafe fn cstrlen32(s: *const u8) -> u32 {
    let base = s as usize;
    if exe::rd::<u8>(base) == 0 {
        return 0;
    }
    let mut cur = base;
    loop {
        cur = cur.wrapping_add(1);
        if exe::rd::<u8>(cur) == 0 {
            return (cur as u32).wrapping_sub(base as u32);
        }
    }
}

#[inline(always)]
unsafe fn build_string_val(val: &mut [u8; 16], s: *const u8, len: u32, allocator: *mut u8) {
    use exe::{rd, wr};
    *val = [0u8; 16];
    let vp = val.as_mut_ptr() as usize;
    let dst = if len <= 0x0d {
        wr::<u16>(vp + 0x0e, 0x1c05);
        wr::<u8>(vp + 0x0d, 0x0du8.wrapping_sub(len as u8));
        vp as *mut u8
    } else {
        wr::<u16>(vp + 0x0e, 0x0c05);
        wr::<u32>(vp, len);
        let p = exe::call_va!(POOL_MALLOC_VA, (allocator, len.wrapping_add(1) as usize) -> *mut u8);
        wr::<u64>(vp + 0x08, (rd::<u64>(vp + 0x08) & 0xffff_0000_0000_0000) | (p as u64));
        p
    };
    exe::call_va!(J_MEMCPY_VA, (dst, s, len as usize) -> *mut u8);
    wr::<u8>((dst as usize).wrapping_add(len as usize), 0);
}

#[inline(always)]
unsafe fn add_named_value(this: *mut u8, name: *const u8, val_v: &mut [u8; 16]) {
    use exe::rd;
    let t = this as usize;
    let obj_val = rd::<*mut u8>(t + 0x08);
    let doc = rd::<usize>(rd::<usize>(t));
    let mut name_v = [0u8; 16];
    let alloc1 = rd::<*mut u8>(doc + 0x10);
    let nlen = cstrlen32(name);
    build_string_val(&mut name_v, name, nlen, alloc1);
    let alloc2 = rd::<*mut u8>(doc + 0x10);
    exe::call_va!(
        ADD_MEMBER_RAW_VA,
        (
            obj_val,
            name_v.as_mut_ptr(),
            val_v.as_mut_ptr(),
            alloc2
        ) -> *mut u8
    );
}

/// `r::Json::addMember(name, const char* const& val)` (`0x1413c3940`, 404 bytes, 1,450 call sites).
pub unsafe extern "win64" fn json_add_member_cstr_ptr(
    this: *mut u8,
    name: *const u8,
    val_ptr: *const *const u8,
) {
    use exe::rd;
    let t = this as usize;
    let obj_val = rd::<*mut u8>(t + 0x08);
    let doc = rd::<usize>(rd::<usize>(t));
    let mut val_v = [0u8; 16];
    let s = rd::<*const u8>(val_ptr as usize);
    let alloc0 = rd::<*mut u8>(doc + 0x10);
    let vlen = cstrlen32(s);
    build_string_val(&mut val_v, s, vlen, alloc0);
    let mut name_v = [0u8; 16];
    let alloc1 = rd::<*mut u8>(doc + 0x10);
    let nlen = cstrlen32(name);
    build_string_val(&mut name_v, name, nlen, alloc1);
    let alloc2 = rd::<*mut u8>(doc + 0x10);
    exe::call_va!(
        ADD_MEMBER_RAW_VA,
        (
            obj_val,
            name_v.as_mut_ptr(),
            val_v.as_mut_ptr(),
            alloc2
        ) -> *mut u8
    );
}

/// `r::Json::addMember(name, const char* val)` (`0x1424813b0`, 24 bytes).
pub unsafe extern "win64" fn json_add_member_cstr(
    this: *mut u8,
    name: *const u8,
    val: *const u8,
) {
    json_add_member_cstr_ptr(this, name, &val);
}

/// `r::Json::addMember(name, const InplaceString& val)` (`0x1413c3c60`, 424 bytes, 57 call sites).
pub unsafe extern "win64" fn json_add_member_inplace_string(
    this: *mut u8,
    name: *const u8,
    val: *const InplaceString,
) {
    use exe::rd;
    let t = this as usize;
    let obj_val = rd::<*mut u8>(t + 0x08);
    let doc = rd::<usize>(rd::<usize>(t));
    let mut val_v = [0u8; 16];
    let alloc0 = rd::<*mut u8>(doc + 0x10);
    let vlen = rd::<u32>(val as usize + 0x04);
    let s = super::inplace_string::inplace_string_data(val);
    build_string_val(&mut val_v, s, vlen, alloc0);
    let mut name_v = [0u8; 16];
    let alloc1 = rd::<*mut u8>(doc + 0x10);
    let nlen = cstrlen32(name);
    build_string_val(&mut name_v, name, nlen, alloc1);
    let alloc2 = rd::<*mut u8>(doc + 0x10);
    exe::call_va!(
        ADD_MEMBER_RAW_VA,
        (
            obj_val,
            name_v.as_mut_ptr(),
            val_v.as_mut_ptr(),
            alloc2
        ) -> *mut u8
    );
}

/// `r::Json::addMember(name, const m::Vec2& val)` (`0x1413c3e10`, 448 bytes, 25 call sites).
pub unsafe extern "win64" fn json_add_member_vec2f(
    this: *mut u8,
    name: *const u8,
    vec2: *const u8,
) {
    use exe::rd;
    let t = this as usize;
    let obj_val = rd::<*mut u8>(t + 0x08);
    let doc = rd::<usize>(rd::<usize>(t));
    let mut val_v = [0u8; 16];
    let mut tmp = [0u8; 8 + 128 + 8];
    let tmp_ptr = tmp.as_mut_ptr() as *mut InplaceString;
    exe::call_va!(0x1_4240_2410, (tmp_ptr, vec2) -> *mut InplaceString);
    let alloc0 = rd::<*mut u8>(doc + 0x10);
    let s = super::inplace_string::inplace_string_data(tmp_ptr);
    let vlen = rd::<u32>(tmp.as_ptr() as usize + 0x04);
    build_string_val(&mut val_v, s, vlen, alloc0);
    exe::call_va!(STRING_DTOR_VA, (tmp_ptr));
    let mut name_v = [0u8; 16];
    let alloc1 = rd::<*mut u8>(doc + 0x10);
    let nlen = cstrlen32(name);
    build_string_val(&mut name_v, name, nlen, alloc1);
    let alloc2 = rd::<*mut u8>(doc + 0x10);
    exe::call_va!(
        ADD_MEMBER_RAW_VA,
        (
            obj_val,
            name_v.as_mut_ptr(),
            val_v.as_mut_ptr(),
            alloc2
        ) -> *mut u8
    );
}

/// `r::Json::addMember(name, const m::Vec2i& val)` (`0x1413c3fd0`, 464 bytes, 6 call sites).
pub unsafe extern "win64" fn json_add_member_vec2i(
    this: *mut u8,
    name: *const u8,
    vec2i: *const u8,
) {
    use exe::rd;
    let t = this as usize;
    let obj_val = rd::<*mut u8>(t + 0x08);
    let doc = rd::<usize>(rd::<usize>(t));
    let mut val_v = [0u8; 16];
    let mut tmp = [0u8; 8 + 128 + 8];
    let tmp_ptr = tmp.as_mut_ptr() as *mut InplaceString;
    let y = rd::<u32>(vec2i as usize + 0x04);
    let x = rd::<u32>(vec2i as usize);
    let tag = rd::<u8>(exe::ptr::<u8>(0x1_426e_7d15) as usize);
    exe::call_va!(
        0x1_405f_c3c0,
        (tmp_ptr, tag, exe::ptr::<u8>(0x1_433f_eadc), x, y) -> *mut InplaceString
    );
    let alloc0 = rd::<*mut u8>(doc + 0x10);
    let s = super::inplace_string::inplace_string_data(tmp_ptr);
    let vlen = rd::<u32>(tmp.as_ptr() as usize + 0x04);
    build_string_val(&mut val_v, s, vlen, alloc0);
    exe::call_va!(STRING_DTOR_VA, (tmp_ptr));
    let mut name_v = [0u8; 16];
    let alloc1 = rd::<*mut u8>(doc + 0x10);
    let nlen = cstrlen32(name);
    build_string_val(&mut name_v, name, nlen, alloc1);
    let alloc2 = rd::<*mut u8>(doc + 0x10);
    exe::call_va!(
        ADD_MEMBER_RAW_VA,
        (
            obj_val,
            name_v.as_mut_ptr(),
            val_v.as_mut_ptr(),
            alloc2
        ) -> *mut u8
    );
}

/// `r::Json::addMember(name, const int8_t& val)` (`0x14247f690`, 278 bytes).
pub unsafe extern "win64" fn json_add_member_i8(this: *mut u8, name: *const u8, val: *const i8) {
    use exe::{rd, wr};
    let x = rd::<i8>(val as usize) as i64;
    let mut val_v = [0u8; 16];
    wr::<i64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, if x < 0 { 0x00b6 } else { 0x01f6 });
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMemberNull(name)` (`0x14247fad0` / `0x142480020`, 246 bytes).
pub unsafe extern "win64" fn json_add_member_null(this: *mut u8, name: *const u8) {
    let mut val_v = [0u8; 16];
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const uint64_t& val)` (`0x142480120`, 341 bytes).
pub unsafe extern "win64" fn json_add_member_u64(this: *mut u8, name: *const u8, val: *const u64) {
    use exe::{rd, wr};
    let x = rd::<u64>(val as usize);
    let mut val_v = [0u8; 16];
    wr::<u64>(val_v.as_mut_ptr() as usize, x);
    let mut f: u16 = if (x as i64) < 0 { 0x0116 } else { 0x0196 };
    if (x & 0xffff_ffff_0000_0000) == 0 {
        f = if (x as i64) < 0 { 0x0156 } else { 0x01d6 };
    }
    if (x & 0xffff_ffff_8000_0000) == 0 {
        f |= 0x0020;
    }
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, f);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const uint16_t& val)` (`0x142480280`, 277 bytes).
pub unsafe extern "win64" fn json_add_member_u16(this: *mut u8, name: *const u8, val: *const u16) {
    use exe::{rd, wr};
    let x = rd::<u16>(val as usize) as u64;
    let mut val_v = [0u8; 16];
    wr::<u64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, 0x01f6);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const int64_t& val)` (`0x1424803a0`, 266 bytes).
pub unsafe extern "win64" fn json_add_member_i64(this: *mut u8, name: *const u8, val: *const i64) {
    let x = exe::rd::<i64>(val as usize);
    let mut val_v = [0u8; 16];
    rapidjson_value_set_i64(val_v.as_mut_ptr(), x);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const uint8_t& val)` (`0x1424804f0`, 277 bytes).
pub unsafe extern "win64" fn json_add_member_u8(this: *mut u8, name: *const u8, val: *const u8) {
    use exe::{rd, wr};
    let x = rd::<u8>(val as usize) as u64;
    let mut val_v = [0u8; 16];
    wr::<u64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, 0x01f6);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const double& val)` (`0x142480610`, 262 bytes).
pub unsafe extern "win64" fn json_add_member_f64(this: *mut u8, name: *const u8, val: *const f64) {
    use exe::{rd, wr};
    let x = rd::<f64>(val as usize);
    let mut val_v = [0u8; 16];
    wr::<f64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, 0x0216);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const uint32_t& val)` (`0x142480d00`, 277 bytes).
pub unsafe extern "win64" fn json_add_member_u32(this: *mut u8, name: *const u8, val: *const u32) {
    use exe::{rd, wr};
    let x = rd::<u32>(val as usize);
    let mut val_v = [0u8; 16];
    wr::<u64>(val_v.as_mut_ptr() as usize, x as u64);
    let f = if (x as i32) < 0 { 0x01d6 } else { 0x01f6 };
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, f);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const float& val)` (`0x142480e20`, 266 bytes).
pub unsafe extern "win64" fn json_add_member_f32(this: *mut u8, name: *const u8, val: *const f32) {
    use exe::{rd, wr};
    let x = rd::<f32>(val as usize) as f64;
    let mut val_v = [0u8; 16];
    wr::<f64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, 0x0216);
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const bool& val)` (`0x142480f30`, 261 bytes).
pub unsafe extern "win64" fn json_add_member_bool(this: *mut u8, name: *const u8, val: *const u8) {
    use exe::{rd, wr};
    let b = rd::<u8>(val as usize) != 0;
    let mut val_v = [0u8; 16];
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, if b { 0x000a } else { 0x0009 });
    add_named_value(this, name, &mut val_v);
}

/// `r::Json::addMember(name, const int32_t& val)` (`0x142481290`, 277 bytes, 1,430 call sites).
pub unsafe extern "win64" fn json_add_member_i32(this: *mut u8, name: *const u8, val: *const i32) {
    use exe::{rd, wr};
    let x = rd::<i32>(val as usize) as i64;
    let mut val_v = [0u8; 16];
    wr::<i64>(val_v.as_mut_ptr() as usize, x);
    wr::<u16>(val_v.as_mut_ptr() as usize + 0x0e, if x < 0 { 0x00b6 } else { 0x01f6 });
    add_named_value(this, name, &mut val_v);
}

/// Return the key C-string of a RapidJSON member iterator (`0x14247f1a0`, 32 bytes).
pub unsafe extern "win64" fn json_iter_key_cstr(this: *const u8) -> *const u8 {
    use exe::rd;
    let entry = rd::<usize>(this as usize);
    if (rd::<u16>(entry + 0x0e) & SHORT_STRING_FLAG) != 0 {
        entry as *const u8
    } else {
        (rd::<usize>(entry + 0x08) & PTR48_MASK) as *const u8
    }
}

/// Look up a JSON object member by `name` (case-insensitive) and return a
/// pointer to its value (`entry + 0x10`) if present and non-null (`[entry + 0x1e] != 0`),
/// or null (`0x14247f310`, 41 bytes).
pub unsafe extern "win64" fn json_find_member_value(this: *mut u8, name: *const u8) -> *mut u8 {
    let entry = json_find_member_nocase(this, name);
    if entry.is_null() || exe::rd::<u16>(entry as usize + 0x1e) == 0 {
        std::ptr::null_mut()
    } else {
        unsafe { entry.add(0x10) }
    }
}

/// Return the `end()` member iterator `{ members + count * 32, doc }` for a
/// JSON object (`0x14247fbe0`, 45 bytes).
pub unsafe extern "win64" fn json_object_end(this: *const u8, out: *mut u8) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let val = rd::<usize>(t + 0x08);
    let count = rd::<u32>(val) as usize;
    let members = rd::<usize>(val + 0x08) & PTR48_MASK;
    let o = out as usize;
    wr::<usize>(o + 0x08, rd::<usize>(t));
    wr::<usize>(o, members.wrapping_add(count << 5));
    out
}

/// Return the `begin()` member iterator `{ members, doc }` for a JSON object
/// (`0x14247fc20`, 35 bytes).
pub unsafe extern "win64" fn json_object_begin(this: *const u8, out: *mut u8) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let val = rd::<usize>(t + 0x08);
    let members = rd::<usize>(val + 0x08) & PTR48_MASK;
    let o = out as usize;
    wr::<usize>(o + 0x08, rd::<usize>(t));
    wr::<usize>(o, members);
    out
}

/// Return `true` iff the JSON object has a sub-object member (`flags == 3`)
/// named `name` (`0x14247fc50`, 47 bytes).
pub unsafe extern "win64" fn json_has_object_member(this: *mut u8, name: *const u8) -> bool {
    let entry = json_find_member_nocase(this, name);
    if entry.is_null() {
        return false;
    }
    let e = entry as usize;
    exe::rd::<u16>(e + 0x1e) != 0 && e.wrapping_add(0x10) != 0 && exe::rd::<u16>(e + 0x1e) == 3
}

/// Return `true` iff the JSON object has a non-null member (`flags != 0`) named
/// `name` (`0x142480cc0`, 49 bytes).
pub unsafe extern "win64" fn json_has_member(this: *mut u8, name: *const u8) -> bool {
    let entry = json_find_member_nocase(this, name);
    if entry.is_null() {
        return false;
    }
    let e = entry as usize;
    exe::rd::<u16>(e + 0x1e) != 0 && e.wrapping_add(0x10) != 0
}

/// Construct `r::JSONPrettyWriter` (`0x1424826e0` / `0x142482860`, 87 bytes).
pub unsafe extern "win64" fn json_pretty_writer_ctor(this: *mut u8, os: usize) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    wr::<u64>(t + 0x30, 0x200);
    wr::<u64>(t + 0x08, 0);
    wr::<u64>(t + 0x10, 0);
    wr::<u64>(t + 0x18, 0);
    wr::<u64>(t + 0x20, 0);
    wr::<u64>(t + 0x28, 0);
    wr::<u8>(t + 0x3c, 0);
    wr::<u64>(t + 0x44, 1);
    wr::<usize>(t, os);
    wr::<u32>(t + 0x38, 0x144);
    let b = (rd::<u32>(t + 0x50) & 0xff00_0020) | 0x20;
    wr::<u32>(t + 0x50, b);
    wr::<u32>(t + 0x54, 0);
    wr::<u8>(t + 0x58, 0);
    wr::<u8>(t + 0x53, 0x20);
    wr::<u64>(t + 0x158, 0);
    wr::<u8>(t + 0x40, 0x09);
    this
}

/// Construct `r::JSONWriter` (`0x1424827a0` / `0x142482920`, 75 bytes).
pub unsafe extern "win64" fn json_writer_ctor(this: *mut u8, os: usize) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    wr::<u64>(t + 0x30, 0x200);
    wr::<u64>(t + 0x08, 0);
    wr::<u64>(t + 0x10, 0);
    wr::<u64>(t + 0x18, 0);
    wr::<u64>(t + 0x20, 0);
    wr::<u64>(t + 0x28, 0);
    wr::<u8>(t + 0x3c, 0);
    wr::<usize>(t, os);
    wr::<u32>(t + 0x38, 0x144);
    let b = (rd::<u32>(t + 0x40) & 0xff00_0020) | 0x20;
    wr::<u32>(t + 0x40, b);
    wr::<u32>(t + 0x44, 0);
    wr::<u8>(t + 0x48, 0);
    wr::<u8>(t + 0x43, 0x20);
    wr::<u64>(t + 0x148, 0);
    this
}

/// Destroy `r::JSONWriter` (`STR_OFF = 0x40`: `0x1424829e0`, `0x142482a80`) or
/// `r::JSONPrettyWriter` (`STR_OFF = 0x50`: `0x142482a30`, `0x142482ad0`).
pub unsafe extern "win64" fn json_writer_dtor<const STR_OFF: usize>(this: *mut u8) {
    let t = this as usize;
    exe::call_va!(STRING_DTOR_VA, ((t + STR_OFF) as *mut InplaceString));
    let pool = exe::rd::<*mut u8>(t + 0x10);
    if !pool.is_null() {
        exe::call_va!(0x1_413c_35d0, (pool));
        exe::call_va!(J_FREE_SIZED_VA, (pool, 0x8028usize));
    }
}

// ---------------------------------------------------------------------------
// Differential tests: 0x1413c1770 and the destructor are mocked. The fake
// converter treats `value` as a test record `{ len: u32, heap: u32, bytes }`
// and writes the bytes inline (or, with `heap`, points the string at the
// record's bytes with a capacity above the inline one); the fake destructor
// records the address it was given relative to the string the converter saw.
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::{Mock, Rng};
use std::cell::Cell;

thread_local! {
    static SEEN_STRING: Cell<usize> = const { Cell::new(0) };
    static DTOR_DELTA: Cell<Option<isize>> = const { Cell::new(None) };
}

unsafe extern "win64" fn fake_to_string(value: *const u8, s: *mut InplaceString) {
    SEEN_STRING.with(|c| c.set(s as usize));
    let len = std::ptr::read_unaligned(value as *const u32) as usize;
    let heap = std::ptr::read_unaligned(value.add(4) as *const u32) != 0;
    let src = value.add(8);
    let p = s as *mut u8;
    if heap {
        // capacity (units) above the 256-byte inline area -> heap mode
        let b = std::ptr::read_unaligned(p as *const u32);
        std::ptr::write_unaligned(p as *mut u32, (b & 0xff00_0000) | (((len as u32 + 1 + 15) & !15).max(272) >> 3));
        std::ptr::write_unaligned(p.add(8) as *mut *const u8, src);
    } else {
        std::ptr::copy_nonoverlapping(src, p.add(8), len);
        *p.add(8 + len) = 0;
    }
    std::ptr::write_unaligned(p.add(4) as *mut u32, len as u32);
}

unsafe extern "win64" fn fake_dtor(s: *mut InplaceString) {
    let seen = SEEN_STRING.with(|c| c.get());
    DTOR_DELTA.with(|c| c.set(Some(s as isize - seen as isize)));
}

/// Test records: empty, short/long ASCII with mixed case, high bytes
/// (sign extension), and heap-mode strings up to 600 bytes.
fn gen_record(r: &mut Rng, i: u64) -> Vec<u8> {
    let heap = i % 5 == 4;
    let len = match i {
        0 => 0,
        1 => 1,
        _ if heap => r.below(600) as usize,
        _ => r.below(256) as usize,
    };
    let mut v = Vec::with_capacity(8 + len + 1);
    v.extend_from_slice(&(len as u32).to_le_bytes());
    v.extend_from_slice(&(heap as u32).to_le_bytes());
    const ALPHA: &[u8] = b"aAzZ09_@`[{.\x80\xc4\xff/ ";
    for _ in 0..len {
        v.push(if r.below(4) == 0 { r.next_u32() as u8 } else { ALPHA[r.below(ALPHA.len() as u64) as usize] });
    }
    v.push(0);
    v
}

unsafe fn diff_hash<T: Copy + PartialEq + std::fmt::Debug + Default>(orig: usize, new: usize) -> Result<u32, String> {
    type F<T> = unsafe extern "win64" fn(*const u8, *mut T) -> u8;
    let (o, n): (F<T>, F<T>) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _conv = Mock::new(VALUE_TO_STRING_VA, fake_to_string as *const ())?;
    let _dtor = Mock::new(STRING_DTOR_VA, fake_dtor as *const ())?;
    let mut r = Rng::new(0x1413c32d0);
    const CASES: u64 = 600;
    for i in 0..CASES {
        let rec = gen_record(&mut r, i);
        let (mut ha, mut hb) = (T::default(), T::default());
        DTOR_DELTA.with(|c| c.set(None));
        let ra = o(rec.as_ptr(), &mut ha) != 0;
        let da = DTOR_DELTA.with(|c| c.take());
        let rb = n(rec.as_ptr(), &mut hb) != 0;
        let db = DTOR_DELTA.with(|c| c.take());
        diff_eq!(ha, hb, "hash, case {i} len {}", rec.len() - 9);
        diff_eq!(ra, rb, "return, case {i}");
        diff_eq!(da, db, "destructor argument, case {i}");
        if da != Some(0) {
            return Err(format!("destructor not called on the converted string (case {i}: {da:?})"));
        }
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_json_hash_nocase_fnv32(orig: usize, new: usize) -> Result<u32, String> {
    diff_hash::<u32>(orig, new)
}

pub unsafe fn diff_json_hash_nocase_fnv64(orig: usize, new: usize) -> Result<u32, String> {
    diff_hash::<u64>(orig, new)
}

pub unsafe fn diff_json_find_member_nocase(orig: usize, new: usize) -> Result<u32, String> {
    use exe::{rd, wr};
    type F = unsafe extern "win64" fn(*mut u8, *const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x14247f340);
    const CASES: u32 = 500;
    let mut long_bufs: Vec<Vec<u8>> = Vec::new();
    for case in 0..CASES {
        long_bufs.clear();
        let count = r.below(8) as u32;
        let mut members = vec![0u8; (count as usize).max(1) * 32];
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for idx in 0..count as usize {
            let entry = members.as_mut_ptr().add(idx * 32) as usize;
            let is_short = r.below(2) == 0;
            let klen = if is_short {
                r.below(14) as usize
            } else {
                r.below(28) as usize
            };
            let mut key = Vec::with_capacity(klen + 1);
            for _ in 0..klen {
                let c = b"aAbBcCdDeEfFzZ019_-."[r.below(20) as usize];
                key.push(c);
            }
            key.push(0);
            if is_short {
                std::ptr::copy_nonoverlapping(key.as_ptr(), entry as *mut u8, key.len());
                wr::<u8>(entry + 0x0d, (0xd - klen as i32) as u8);
                wr::<u16>(entry + 0x0e, SHORT_STRING_FLAG | (r.next_u32() as u16 & !SHORT_STRING_FLAG));
            } else {
                wr::<u32>(entry, klen as u32);
                let hi_garbage = (r.next_u64() as usize) & !PTR48_MASK;
                let mut lbuf = key.clone();
                let lptr = lbuf.as_mut_ptr() as usize;
                long_bufs.push(lbuf);
                wr::<usize>(entry + 0x08, hi_garbage | (lptr & PTR48_MASK));
                wr::<u16>(entry + 0x0e, r.next_u32() as u16 & !SHORT_STRING_FLAG);
            }
            keys.push(key);
        }

        // Build query name: either case-permuted version of an existing key, or random.
        let mut query = if count > 0 && r.below(4) != 0 {
            let k = &keys[r.below(count as u64) as usize];
            let mut q = k.clone();
            for b in q.iter_mut() {
                if *b != 0 && r.below(2) == 0 {
                    *b = if b.is_ascii_lowercase() {
                        b.to_ascii_uppercase()
                    } else {
                        b.to_ascii_lowercase()
                    };
                }
            }
            q
        } else {
            let qlen = r.below(20) as usize;
            let mut q = Vec::with_capacity(qlen + 1);
            for _ in 0..qlen {
                q.push(b"aAbBcCdDeEfFzZ019_-."[r.below(20) as usize]);
            }
            q.push(0);
            q
        };
        if *query.last().unwrap_or(&1) != 0 {
            query.push(0);
        }

        let mut val_obj = [0u64; 2];
        wr::<u32>(val_obj.as_mut_ptr() as usize, count);
        let hi_garbage = (r.next_u64() as usize) & !PTR48_MASK;
        wr::<usize>(
            (val_obj.as_mut_ptr() as usize) + 8,
            hi_garbage | ((members.as_ptr() as usize) & PTR48_MASK),
        );

        let cached_count = if r.below(2) == 0 { count } else { r.next_u32() };
        let last_idx = match r.below(4) {
            0 => u32::MAX,
            1 => count,
            _ => r.below(count.max(1) as u64) as u32,
        };
        let mut this_a = [0u64; 3];
        wr::<usize>((this_a.as_mut_ptr() as usize) + 0x08, val_obj.as_ptr() as usize);
        wr::<u32>((this_a.as_mut_ptr() as usize) + 0x10, last_idx);
        wr::<u32>((this_a.as_mut_ptr() as usize) + 0x14, cached_count);
        let mut this_b = this_a;

        let ra = o(this_a.as_mut_ptr() as *mut u8, query.as_ptr());
        let rb = n(this_b.as_mut_ptr() as *mut u8, query.as_ptr());
        diff_eq!(ra, rb, "return, case {case} count {count}");
        diff_eq!(
            (rd::<u32>((this_a.as_ptr() as usize) + 0x10), rd::<u32>((this_a.as_ptr() as usize) + 0x14)),
            (rd::<u32>((this_b.as_ptr() as usize) + 0x10), rd::<u32>((this_b.as_ptr() as usize) + 0x14)),
            "cursor state, case {case} count {count}"
        );
    }
    Ok(CASES)
}

pub unsafe fn diff_json_set_error_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x14247f4a0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let has_inner = r.below(5) != 0;
        let has_cstr = r.below(4) != 0;
        let clen = r.below(48) as usize;
        let mut cstr = Vec::with_capacity(clen + 1);
        for _ in 0..clen {
            cstr.push(b'a' + (r.below(26) as u8));
        }
        cstr.push(0);
        let cptr = if has_cstr { cstr.as_ptr() } else { std::ptr::null() };

        let mut inner_a = vec![0u8; 0x140 + 8 + 64 + 8];
        for b in &mut inner_a {
            *b = r.next_u32() as u8;
        }
        // Construct a valid inline InplaceString<64> (`UNITS = 8`) at `+0x140`.
        let str_off = 0x140usize;
        exe::wr::<u32>(inner_a.as_mut_ptr() as usize + str_off, 0x0800_0008);
        let init_len = r.below(16) as u32;
        exe::wr::<u32>(inner_a.as_mut_ptr() as usize + str_off + 4, init_len);
        inner_a[str_off + 8 + init_len as usize] = 0;
        let mut inner_b = inner_a.clone();

        let ptr_a: usize = if has_inner { inner_a.as_mut_ptr() as usize } else { 0 };
        let ptr_b: usize = if has_inner { inner_b.as_mut_ptr() as usize } else { 0 };
        o(&ptr_a as *const usize as *mut u8, cptr);
        n(&ptr_b as *const usize as *mut u8, cptr);
        diff_eq!(inner_a, inner_b, "inner state, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_set_error_inplace_string(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x14247f540);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let has_inner = r.below(5) != 0;
        let slen = r.below(40) as usize;
        let mut s_buf = [0u8; 8 + 64 + 8];
        exe::wr::<u32>(s_buf.as_mut_ptr() as usize, 0x0800_0008);
        exe::wr::<u32>(s_buf.as_mut_ptr() as usize + 4, slen as u32);
        for k in 0..slen {
            s_buf[8 + k] = b'a' + (r.below(26) as u8);
        }
        s_buf[8 + slen] = 0;

        let mut inner_a = vec![0u8; 0x140 + 8 + 64 + 8];
        for b in &mut inner_a {
            *b = r.next_u32() as u8;
        }
        let str_off = 0x140usize;
        exe::wr::<u32>(inner_a.as_mut_ptr() as usize + str_off, 0x0800_0008);
        let init_len = r.below(16) as u32;
        exe::wr::<u32>(inner_a.as_mut_ptr() as usize + str_off + 4, init_len);
        inner_a[str_off + 8 + init_len as usize] = 0;
        let mut inner_b = inner_a.clone();

        let ptr_a: usize = if has_inner { inner_a.as_mut_ptr() as usize } else { 0 };
        let ptr_b: usize = if has_inner { inner_b.as_mut_ptr() as usize } else { 0 };
        o(&ptr_a as *const usize as *mut u8, s_buf.as_ptr() as *const InplaceString);
        n(&ptr_b as *const usize as *mut u8, s_buf.as_ptr() as *const InplaceString);
        diff_eq!(inner_a, inner_b, "inner state, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_writer_set_str_at<const OFF: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x142482460 ^ OFF as u64);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let has_cstr = r.below(4) != 0;
        let clen = r.below(48) as usize;
        let mut cstr = Vec::with_capacity(clen + 1);
        for _ in 0..clen {
            cstr.push(b'a' + (r.below(26) as u8));
        }
        cstr.push(0);
        let cptr = if has_cstr { cstr.as_ptr() } else { std::ptr::null() };

        let mut buf_a = vec![0u8; OFF + 8 + 64 + 8];
        for b in &mut buf_a {
            *b = r.next_u32() as u8;
        }
        exe::wr::<u32>(buf_a.as_mut_ptr() as usize + OFF, 0x0800_0008);
        let init_len = r.below(16) as u32;
        exe::wr::<u32>(buf_a.as_mut_ptr() as usize + OFF + 4, init_len);
        buf_a[OFF + 8 + init_len as usize] = 0;
        let mut buf_b = buf_a.clone();

        o(buf_a.as_mut_ptr(), cptr);
        n(buf_b.as_mut_ptr(), cptr);
        diff_eq!(buf_a, buf_b, "state, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_exc_default_ctor<const VTBL_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(VTBL_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let mut a = [r.next_u64(); 3];
        let mut b = a;
        let ra = o(a.as_mut_ptr() as *mut u8);
        let rb = n(b.as_mut_ptr() as *mut u8);
        diff_eq!(ra - a.as_ptr() as usize, rb - b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(a, b, "exc, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_doc_name_at<const DOC_OFF: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1424813d0 ^ DOC_OFF as u64);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut doc = vec![0u8; 0x140 + 8 + 64 + 8];
        let heap = r.below(2) == 0;
        let heap_str = b"heap_doc.json\0";
        if heap {
            exe::wr::<u32>(doc.as_mut_ptr() as usize + 0x140, 0x0800_0020);
            exe::wr::<usize>(doc.as_mut_ptr() as usize + 0x148, heap_str.as_ptr() as usize);
        } else {
            exe::wr::<u32>(doc.as_mut_ptr() as usize + 0x140, 0x0800_0008);
            doc[0x148..0x148 + 9].copy_from_slice(b"inline.j\0");
        }
        let doc_ptr = if r.below(4) == 0 { 0 } else { doc.as_ptr() as usize };
        let mut obj = [0u8; 0x20];
        exe::wr::<usize>(obj.as_mut_ptr() as usize + DOC_OFF, doc_ptr);
        diff_eq!(o(obj.as_ptr()), n(obj.as_ptr()), "case {i}");
    }
    Ok(CASES)
}

thread_local! {
    static EXC_FMT_LOG: std::cell::RefCell<Vec<(usize, usize, usize, usize, usize)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_exc_format(
    this: *mut u8,
    fmt: *const u8,
    a0: usize,
    a1: usize,
    a2: usize,
) {
    EXC_FMT_LOG.with(|l| l.borrow_mut().push((this as usize, fmt as usize, a0, a1, a2)));
    exe::wr::<usize>(this as usize + 0x08, 0xfeed_face);
}

unsafe fn make_fake_doc(r: &mut Rng) -> (Vec<u8>, usize) {
    let mut doc = vec![0u8; 0x140 + 8 + 64 + 8];
    if r.below(2) == 0 {
        exe::wr::<u32>(doc.as_mut_ptr() as usize + 0x140, 0x0800_0020);
        exe::wr::<usize>(doc.as_mut_ptr() as usize + 0x148, 0x1234_5678);
    } else {
        exe::wr::<u32>(doc.as_mut_ptr() as usize + 0x140, 0x0800_0008);
    }
    let ptr = if r.below(3) == 0 { 0 } else { doc.as_ptr() as usize };
    (doc, ptr)
}

pub unsafe fn diff_json_exc_ctor_member<const VTBL_VA: u64, const FMT_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(EXC_FORMAT_MSG_VA, fake_exc_format as *const ())?;
    let mut r = Rng::new(VTBL_VA ^ FMT_VA);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let (_doc, dptr) = make_fake_doc(&mut r);
        let member = r.next_u64() as usize as *const u8;
        let mut exc = [r.next_u64(); 3];
        let snap = exc;
        EXC_FMT_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(exc.as_mut_ptr() as *mut u8, dptr as *const u8, member);
        let (ea, la) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        exc = snap;
        let rb = n(exc.as_mut_ptr() as *mut u8, dptr as *const u8, member);
        let (eb, lb) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ea, eb, "exc, case {i}");
        diff_eq!(
            la.iter().map(|&(t, f, x, y, _)| (t, f, x, y)).collect::<Vec<_>>(),
            lb.iter().map(|&(t, f, x, y, _)| (t, f, x, y)).collect::<Vec<_>>(),
            "fmt log, case {i}"
        );
    }
    Ok(CASES)
}

pub unsafe fn diff_json_exc_ctor_null_iter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(EXC_FORMAT_MSG_VA, fake_exc_format as *const ())?;
    let mut r = Rng::new(0x142482190);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let (_doc, dptr) = make_fake_doc(&mut r);
        let a2 = r.next_u64() as usize;
        let a3 = r.next_u64() as usize;
        let mut exc = [r.next_u64(); 3];
        let snap = exc;
        EXC_FMT_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(exc.as_mut_ptr() as *mut u8, dptr as *const u8, a2, a3);
        let (ea, la) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        exc = snap;
        let rb = n(exc.as_mut_ptr() as *mut u8, dptr as *const u8, a2, a3);
        let (eb, lb) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ea, eb, "exc, case {i}");
        diff_eq!(
            la.iter().map(|&(t, f, x, _, _)| (t, f, x)).collect::<Vec<_>>(),
            lb.iter().map(|&(t, f, x, _, _)| (t, f, x)).collect::<Vec<_>>(),
            "fmt log, case {i}"
        );
    }
    Ok(CASES)
}

pub unsafe fn diff_json_exc_ctor_3arg(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(EXC_FORMAT_MSG_VA, fake_exc_format as *const ())?;
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let (_doc, dptr) = make_fake_doc(&mut r);
        let a2 = r.next_u32() as usize;
        let a3 = r.next_u32() as usize;
        let mut exc = [r.next_u64(); 3];
        let snap = exc;
        EXC_FMT_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(exc.as_mut_ptr() as *mut u8, dptr as *const u8, a2, a3);
        let (ea, la) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        exc = snap;
        let rb = n(exc.as_mut_ptr() as *mut u8, dptr as *const u8, a2, a3);
        let (eb, lb) = (exc, EXC_FMT_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ea, eb, "exc, case {i}");
        diff_eq!(la, lb, "fmt log, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_rapidjson_value_get_double(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> f64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1413c21b0);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let mut v = [0u8; 16];
        let flags: u16 = match r.below(6) {
            0 => 0x0216, // double
            1 => 0x00b6, // int32
            2 => 0x01d6, // uint32
            3 => 0x0096, // int64
            4 => 0x0116, // uint64 (high bit set)
            _ => r.next_u32() as u16,
        };
        let bits = match r.below(5) {
            0 => 0u64,
            1 => u64::MAX,
            2 => 1u64 << 63,
            3 => 0x3ff0_0000_0000_0000u64,
            _ => r.next_u64(),
        };
        exe::wr::<u64>(v.as_mut_ptr() as usize, bits);
        exe::wr::<u16>(v.as_mut_ptr() as usize + 0x0e, flags);
        let ra = o(v.as_ptr()).to_bits();
        let rb = n(v.as_ptr()).to_bits();
        diff_eq!(ra, rb, "case {i} flags {flags:#x} bits {bits:#x}");
    }
    Ok(CASES)
}

pub unsafe fn diff_rapidjson_value_set_i64(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, i64) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1413c36d0);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let val: i64 = match i {
            0 => 0,
            1 => 1,
            2 => -1,
            3 => i32::MAX as i64,
            4 => i32::MAX as i64 + 1,
            5 => u32::MAX as i64,
            6 => u32::MAX as i64 + 1,
            7 => i32::MIN as i64,
            8 => i32::MIN as i64 - 1,
            9 => i64::MAX,
            10 => i64::MIN,
            _ => r.next_u64() as i64,
        };
        let mut a = [r.next_u64(); 2];
        let mut b = a;
        let ra = o(a.as_mut_ptr() as *mut u8, val);
        let rb = n(b.as_mut_ptr() as *mut u8, val);
        diff_eq!(ra - a.as_ptr() as usize, rb - b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(a, b, "val, case {i} input {val}");
    }
    Ok(CASES)
}

thread_local! {
    static POOL_BUF: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static POOL_LOG: std::cell::RefCell<Vec<(&'static str, usize, usize)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_op_new(sz: usize) -> *mut u8 {
    POOL_LOG.with(|l| l.borrow_mut().push(("op_new", sz, 0)));
    0x7777_0000 as *mut u8
}

unsafe extern "win64" fn fake_calloc_r9(_a: usize, _b: usize, _c: usize, sz: usize) -> *mut u8 {
    POOL_LOG.with(|l| l.borrow_mut().push(("calloc", sz, 0)));
    POOL_BUF.with(|b| b.borrow_mut().as_mut_ptr())
}

unsafe extern "win64" fn fake_pool_memset(dst: *mut u8, val: i32, sz: usize) -> *mut u8 {
    POOL_LOG.with(|l| l.borrow_mut().push(("memset", dst as usize, (val as usize) ^ sz)));
    dst
}

pub unsafe fn diff_rapidjson_pool_malloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_new = Mock::new(OP_NEW_VA, fake_op_new as *const ())?;
    let _m_cal = Mock::new(CALLOC_R9_VA, fake_calloc_r9 as *const ())?;
    let _m_set = Mock::new(J_MEMSET_VA, fake_pool_memset as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 64]);
    let mut r = Rng::new(0x1413c3800);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let mut chunk_a = [0u64; 8];
        chunk_a[0] = r.below(128);
        chunk_a[1] = r.below(64);
        chunk_a[2] = r.next_u64();
        let mut chunk_b = chunk_a;
        let has_chunk = r.below(3) != 0;
        let cap_cfg = 16 + r.below(256) as u64;
        let base_alloc = if r.below(2) == 0 { 0 } else { 0x5555_0000u64 };
        let size = match r.below(4) {
            0 => 0,
            1 => 1 + r.below(32) as usize,
            _ => 1 + r.below(512) as usize,
        };
        let mut pool_a = [
            if has_chunk { chunk_a.as_mut_ptr() as u64 } else { 0 },
            cap_cfg,
            0,
            base_alloc,
            base_alloc,
        ];
        let mut pool_b = [
            if has_chunk { chunk_b.as_mut_ptr() as u64 } else { 0 },
            cap_cfg,
            0,
            base_alloc,
            base_alloc,
        ];

        POOL_BUF.with(|b| b.borrow_mut().fill(0));
        POOL_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(pool_a.as_mut_ptr() as *mut u8, size);
        let (ba, la) = (
            POOL_BUF.with(|b| b.borrow().clone()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| b.borrow_mut().fill(0));
        let rb = n(pool_b.as_mut_ptr() as *mut u8, size);
        let (bb, lb) = (
            POOL_BUF.with(|b| b.borrow().clone()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        if ra != 0 && has_chunk && pool_a[0] == chunk_a.as_ptr() as u64 {
            diff_eq!(
                ra.wrapping_sub(chunk_a.as_ptr() as usize),
                rb.wrapping_sub(chunk_b.as_ptr() as usize),
                "ret in existing chunk, case {i}"
            );
            diff_eq!(chunk_a, chunk_b, "existing chunk, case {i}");
        } else {
            diff_eq!(ra, rb, "ret, case {i}");
            diff_eq!(&ba[..16], &bb[..16], "new chunk cap/size, case {i}");
        }
        diff_eq!(&pool_a[1..], &pool_b[1..], "pool state, case {i}");
        diff_eq!(la, lb, "log, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_pool_malloc_fn(pool: *mut u8, sz: usize) -> *mut u8 {
    POOL_LOG.with(|l| l.borrow_mut().push(("pool_malloc", pool as usize, sz)));
    POOL_BUF.with(|b| {
        let mut v = b.borrow_mut();
        let off = exe::rd::<usize>(v.as_ptr() as usize);
        let next = off + ((sz + 15) & !15).min(1024);
        exe::wr::<usize>(v.as_mut_ptr() as usize, next);
        unsafe { v.as_mut_ptr().add(off) }
    })
}

pub unsafe fn diff_rapidjson_pool_realloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1413c3730);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let mut chunk = vec![0u8; 512];
        let cap = 256usize;
        let orig_sz = r.below(64) as usize;
        let orig_aligned = (orig_sz + 7) & !7;
        let used = orig_aligned + (r.below(8) as usize) * 8;
        exe::wr::<usize>(chunk.as_mut_ptr() as usize, cap);
        exe::wr::<usize>(chunk.as_mut_ptr() as usize + 8, used);
        let in_place = r.below(2) == 0;
        let orig_off = if in_place {
            0x18 + used - orig_aligned
        } else {
            0x18
        };
        let orig_ptr = if r.below(5) == 0 {
            std::ptr::null_mut()
        } else {
            unsafe { chunk.as_mut_ptr().add(orig_off) }
        };
        let new_sz = match r.below(4) {
            0 => 0,
            1 => orig_sz,
            _ => r.below(300) as usize,
        };
        let mut chunk_b = chunk.clone();
        let orig_ptr_b = if orig_ptr.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { chunk_b.as_mut_ptr().add(orig_off) }
        };
        let mut pool_a = [chunk.as_mut_ptr() as usize];
        let mut pool_b = [chunk_b.as_mut_ptr() as usize];

        POOL_BUF.with(|b| exe::wr::<usize>(b.borrow_mut().as_mut_ptr() as usize, 16));
        POOL_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(pool_a.as_mut_ptr() as *mut u8, orig_ptr, orig_sz, new_sz);
        let la = POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        POOL_BUF.with(|b| exe::wr::<usize>(b.borrow_mut().as_mut_ptr() as usize, 16));
        let rb = n(pool_b.as_mut_ptr() as *mut u8, orig_ptr_b, orig_sz, new_sz);
        let lb = POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        let rel_a = if ra == orig_ptr as usize { 1usize } else { ra };
        let rel_b = if rb == orig_ptr_b as usize { 1usize } else { rb };
        diff_eq!(rel_a, rel_b, "ret, case {i}");
        diff_eq!(&chunk[..24], &chunk_b[..24], "chunk hdr, case {i}");
        diff_eq!(
            la.iter().map(|&(k, _, s)| (k, s)).collect::<Vec<_>>(),
            lb.iter().map(|&(k, _, s)| (k, s)).collect::<Vec<_>>(),
            "log, case {i}"
        );
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_free_sized(p: *mut u8, sz: usize) {
    POOL_LOG.with(|l| l.borrow_mut().push(("free_sized", p as usize, sz)));
}

unsafe extern "win64" fn fake_mem_free(p: *mut u8) {
    POOL_LOG.with(|l| l.borrow_mut().push(("mem_free", p as usize, 0)));
}

pub unsafe fn diff_rapidjson_pool_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(0x1_4244_2de0, fake_mem_free as *const ())?;
    let _m_del = Mock::new(J_FREE_SIZED_VA, fake_free_sized as *const ())?;
    let mut r = Rng::new(0x1413c35d0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut user_chunk = [r.next_u64(), r.next_u64(), 0u64];
        let c1 = [r.next_u64(), r.next_u64(), user_chunk.as_mut_ptr() as u64];
        let c0 = [r.next_u64(), r.next_u64(), c1.as_ptr() as u64];
        let head = match r.below(4) {
            0 => 0,
            1 => user_chunk.as_mut_ptr() as u64,
            2 => c1.as_ptr() as u64,
            _ => c0.as_ptr() as u64,
        };
        let own_base = r.next_u64();
        let mut pool_a = [head, 1024, user_chunk.as_mut_ptr() as u64, own_base, own_base];
        let mut pool_b = pool_a;
        let u_snap = user_chunk;

        POOL_LOG.with(|l| l.borrow_mut().clear());
        o(pool_a.as_mut_ptr() as *mut u8);
        let (ua, la) = (user_chunk, POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));

        user_chunk = u_snap;
        n(pool_b.as_mut_ptr() as *mut u8);
        let (ub, lb) = (user_chunk, POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())));

        diff_eq!(pool_a, pool_b, "pool, case {i}");
        diff_eq!(ua, ub, "user_chunk, case {i}");
        diff_eq!(la, lb, "log, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_rapidjson_value_set_string_raw(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1413c3640);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let len = r.below(40) as usize;
        let mut src = vec![0u8; len + 1];
        for b in &mut src[..len] {
            *b = b'a' + (r.below(26) as u8);
        }
        let mut str_ref = [0u64; 2];
        exe::wr::<usize>(str_ref.as_mut_ptr() as usize, src.as_ptr() as usize);
        exe::wr::<u32>(str_ref.as_mut_ptr() as usize + 8, len as u32);
        let mut va = [r.next_u64(); 2];
        let mut vb = va;
        let alloc = 0x9999_0000 as *mut u8;

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        POOL_LOG.with(|l| l.borrow_mut().clear());
        o(va.as_mut_ptr() as *mut u8, str_ref.as_ptr() as *const u8, alloc);
        let (ba, la) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(vb.as_mut_ptr() as *mut u8, str_ref.as_ptr() as *const u8, alloc);
        let (bb, lb) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(va, vb, "val, case {i} len {len}");
        diff_eq!(ba, bb, "pool buf, case {i}");
        diff_eq!(la, lb, "log, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_rapidjson_add_member_raw(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, *mut u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1413c3460);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let mut chunk = vec![0u8; 2048];
        exe::wr::<usize>(chunk.as_mut_ptr() as usize, 1024);
        let cap = match r.below(4) {
            0 => 0u32,
            1 => 2u32,
            _ => 4u32,
        };
        let count = if cap == 0 {
            0
        } else if r.below(2) == 0 {
            cap - 1
        } else {
            cap
        };
        let orig_bytes = (cap as usize) * 32;
        exe::wr::<usize>(chunk.as_mut_ptr() as usize + 8, orig_bytes);
        let members_ptr = unsafe { chunk.as_mut_ptr().add(0x18) } as usize;
        let hi_garbage = (r.next_u64() as usize) & !PTR48_MASK;
        let mut obj_a = [0u64; 2];
        exe::wr::<u32>(obj_a.as_mut_ptr() as usize, count);
        exe::wr::<u32>(obj_a.as_mut_ptr() as usize + 4, cap);
        exe::wr::<usize>(
            obj_a.as_mut_ptr() as usize + 8,
            hi_garbage | (if cap == 0 { 0 } else { members_ptr & PTR48_MASK }),
        );
        let mut obj_b = obj_a;
        let mut name_a = [r.next_u64(), r.next_u64()];
        let mut name_b = name_a;
        let mut val_a = [r.next_u64(), r.next_u64()];
        let mut val_b = val_a;
        let mut pool = [chunk.as_mut_ptr() as usize];
        let snap_chunk = chunk.clone();

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..1024].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        POOL_LOG.with(|l| l.borrow_mut().clear());
        let ra = o(
            obj_a.as_mut_ptr() as *mut u8,
            name_a.as_mut_ptr() as *mut u8,
            val_a.as_mut_ptr() as *mut u8,
            pool.as_mut_ptr() as *mut u8,
        );
        let (ca, ba, la) = (
            chunk.clone(),
            POOL_BUF.with(|b| b.borrow()[..1024].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        chunk.copy_from_slice(&snap_chunk);
        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..1024].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        let rb = n(
            obj_b.as_mut_ptr() as *mut u8,
            name_b.as_mut_ptr() as *mut u8,
            val_b.as_mut_ptr() as *mut u8,
            pool.as_mut_ptr() as *mut u8,
        );
        let (cb, bb, lb) = (
            chunk.clone(),
            POOL_BUF.with(|b| b.borrow()[..1024].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(ra - obj_a.as_ptr() as usize, rb - obj_b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(obj_a, obj_b, "obj, case {i}");
        diff_eq!(name_a, name_b, "name, case {i}");
        diff_eq!(val_a, val_b, "val, case {i}");
        diff_eq!(ca, cb, "chunk, case {i}");
        diff_eq!(ba, bb, "pool buf, case {i}");
        diff_eq!(la, lb, "log, case {i}");
    }
    Ok(CASES)
}

thread_local! {
    static ADD_RAW_LOG: std::cell::RefCell<Vec<(usize, [u8; 16], [u8; 16], usize)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_add_member_raw(
    obj: *mut u8,
    name: *mut u8,
    val: *mut u8,
    alloc: *mut u8,
) -> *mut u8 {
    let mut nb = [0u8; 16];
    let mut vb = [0u8; 16];
    std::ptr::copy_nonoverlapping(name, nb.as_mut_ptr(), 16);
    std::ptr::copy_nonoverlapping(val, vb.as_mut_ptr(), 16);
    ADD_RAW_LOG.with(|l| l.borrow_mut().push((obj as usize, nb, vb, alloc as usize)));
    obj
}

fn gen_cstr(r: &mut Rng) -> Vec<u8> {
    let len = match r.below(4) {
        0 => 0,
        1 => r.below(14) as usize,
        2 => 13,
        _ => 14 + r.below(24) as usize,
    };
    let mut v = Vec::with_capacity(len + 1);
    for _ in 0..len {
        v.push(b'a' + (r.below(26) as u8));
    }
    v.push(0);
    v
}

pub unsafe fn diff_json_add_member_scalar(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_pool = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    let _m_add = Mock::new(ADD_MEMBER_RAW_VA, fake_add_member_raw as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let name = gen_cstr(&mut r);
        let val_bits: u64 = match i % 10 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => i32::MIN as i64 as u64,
            4 => i32::MAX as u64,
            5 => u32::MAX as u64,
            6 => u32::MAX as u64 + 1,
            7 => 1u64 << 63,
            8 => (-123.456f64).to_bits(),
            _ => r.next_u64(),
        };
        let doc: [usize; 3] = [0, 0, 0x8888_0000];
        let doc_ptr = doc.as_ptr() as usize;
        let this_obj: [usize; 2] = [&doc_ptr as *const usize as usize, 0x7777_0000];

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        POOL_LOG.with(|l| l.borrow_mut().clear());
        ADD_RAW_LOG.with(|l| l.borrow_mut().clear());
        o(
            this_obj.as_ptr() as *mut u8,
            name.as_ptr(),
            &val_bits as *const u64 as *const u8,
        );
        let (ba, la, aa) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(
            this_obj.as_ptr() as *mut u8,
            name.as_ptr(),
            &val_bits as *const u64 as *const u8,
        );
        let (bb, lb, ab) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(aa, ab, "add_raw, case {i}");
        diff_eq!(ba, bb, "pool_buf, case {i}");
        diff_eq!(la, lb, "pool_log, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_add_member_cstr_ptr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_pool = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    let _m_add = Mock::new(ADD_MEMBER_RAW_VA, fake_add_member_raw as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1413c3940);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let name = gen_cstr(&mut r);
        let val = gen_cstr(&mut r);
        let vptr = val.as_ptr();
        let doc: [usize; 3] = [0, 0, 0x8888_0000];
        let doc_ptr = doc.as_ptr() as usize;
        let this_obj: [usize; 2] = [&doc_ptr as *const usize as usize, 0x7777_0000];

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        ADD_RAW_LOG.with(|l| l.borrow_mut().clear());
        o(this_obj.as_ptr() as *mut u8, name.as_ptr(), &vptr);
        let (ba, aa) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(this_obj.as_ptr() as *mut u8, name.as_ptr(), &vptr);
        let (bb, ab) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(aa, ab, "add_raw, case {i}");
        diff_eq!(ba, bb, "pool_buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_add_member_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_pool = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    let _m_add = Mock::new(ADD_MEMBER_RAW_VA, fake_add_member_raw as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1424813b0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let name = gen_cstr(&mut r);
        let val = gen_cstr(&mut r);
        let doc: [usize; 3] = [0, 0, 0x8888_0000];
        let doc_ptr = doc.as_ptr() as usize;
        let this_obj: [usize; 2] = [&doc_ptr as *const usize as usize, 0x7777_0000];

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        ADD_RAW_LOG.with(|l| l.borrow_mut().clear());
        o(this_obj.as_ptr() as *mut u8, name.as_ptr(), val.as_ptr());
        let (ba, aa) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(this_obj.as_ptr() as *mut u8, name.as_ptr(), val.as_ptr());
        let (bb, ab) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(aa, ab, "add_raw, case {i}");
        diff_eq!(ba, bb, "pool_buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_add_member_inplace_string(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_pool = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    let _m_add = Mock::new(ADD_MEMBER_RAW_VA, fake_add_member_raw as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(0x1413c3c60);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let name = gen_cstr(&mut r);
        let val = gen_cstr(&mut r);
        let vlen = val.len() - 1;
        let mut s_buf = [0u8; 8 + 64 + 8];
        if r.below(2) == 0 {
            exe::wr::<u32>(s_buf.as_mut_ptr() as usize, 0x0800_0008);
            s_buf[8..8 + val.len()].copy_from_slice(&val);
        } else {
            exe::wr::<u32>(s_buf.as_mut_ptr() as usize, 0x0800_0020);
            exe::wr::<usize>(s_buf.as_mut_ptr() as usize + 8, val.as_ptr() as usize);
        }
        exe::wr::<u32>(s_buf.as_mut_ptr() as usize + 4, vlen as u32);
        let doc: [usize; 3] = [0, 0, 0x8888_0000];
        let doc_ptr = doc.as_ptr() as usize;
        let this_obj: [usize; 2] = [&doc_ptr as *const usize as usize, 0x7777_0000];

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        ADD_RAW_LOG.with(|l| l.borrow_mut().clear());
        o(
            this_obj.as_ptr() as *mut u8,
            name.as_ptr(),
            s_buf.as_ptr() as *const InplaceString,
        );
        let (ba, aa) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(
            this_obj.as_ptr() as *mut u8,
            name.as_ptr(),
            s_buf.as_ptr() as *const InplaceString,
        );
        let (bb, ab) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(aa, ab, "add_raw, case {i}");
        diff_eq!(ba, bb, "pool_buf, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_vec2_to_str(
    out: *mut InplaceString,
    vec2: *const u8,
) -> *mut InplaceString {
    let p = out as usize;
    exe::wr::<u32>(p, 0x1000_0010);
    let x = exe::rd::<u32>(vec2 as usize);
    let y = exe::rd::<u32>(vec2 as usize + 4);
    let s = format!("{x:08x},{y:08x}\0");
    let len = s.len() - 1;
    exe::wr::<u32>(p + 4, len as u32);
    std::ptr::copy_nonoverlapping(s.as_ptr(), (p + 8) as *mut u8, s.len());
    out
}

pub unsafe fn diff_json_add_member_vec2(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_pool = Mock::new(POOL_MALLOC_VA, fake_pool_malloc_fn as *const ())?;
    let _m_add = Mock::new(ADD_MEMBER_RAW_VA, fake_add_member_raw as *const ())?;
    let _m_v2 = Mock::new(0x1_4240_2410, fake_vec2_to_str as *const ())?;
    POOL_BUF.with(|b| *b.borrow_mut() = vec![0u8; 4096]);
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let name = gen_cstr(&mut r);
        let vec2: [i32; 2] = [r.next_u32() as i32, r.next_u32() as i32];
        let doc: [usize; 3] = [0, 0, 0x8888_0000];
        let doc_ptr = doc.as_ptr() as usize;
        let this_obj: [usize; 2] = [&doc_ptr as *const usize as usize, 0x7777_0000];

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        ADD_RAW_LOG.with(|l| l.borrow_mut().clear());
        o(this_obj.as_ptr() as *mut u8, name.as_ptr(), vec2.as_ptr() as *const u8);
        let (ba, aa) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        POOL_BUF.with(|b| {
            let mut v = b.borrow_mut();
            v[..256].fill(0);
            exe::wr::<usize>(v.as_mut_ptr() as usize, 16);
        });
        n(this_obj.as_ptr() as *mut u8, name.as_ptr(), vec2.as_ptr() as *const u8);
        let (bb, ab) = (
            POOL_BUF.with(|b| b.borrow()[..256].to_vec()),
            ADD_RAW_LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
        );

        diff_eq!(aa, ab, "add_raw, case {i}");
        diff_eq!(ba, bb, "pool_buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_iter_key_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x14247f1a0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut entry = [r.next_u64(); 2];
        let flags = if r.below(2) == 0 {
            SHORT_STRING_FLAG | (r.next_u32() as u16)
        } else {
            (r.next_u32() as u16) & !SHORT_STRING_FLAG
        };
        exe::wr::<u16>(entry.as_mut_ptr() as usize + 0x0e, flags);
        let it = [entry.as_ptr() as usize];
        diff_eq!(o(it.as_ptr() as *const u8), n(it.as_ptr() as *const u8), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_object_iter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut val = [0u64; 2];
        exe::wr::<u32>(val.as_mut_ptr() as usize, r.below(64) as u32);
        exe::wr::<u64>(val.as_mut_ptr() as usize + 8, r.next_u64());
        let this_obj: [usize; 2] = [r.next_u64() as usize, val.as_ptr() as usize];
        let mut oa = [0u64; 2];
        let mut ob = [0u64; 2];
        let ra = o(this_obj.as_ptr() as *const u8, oa.as_mut_ptr() as *mut u8);
        let rb = n(this_obj.as_ptr() as *const u8, ob.as_mut_ptr() as *mut u8);
        diff_eq!(ra - oa.as_ptr() as usize, rb - ob.as_ptr() as usize, "ret, case {i}");
        diff_eq!(oa, ob, "iter, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_member_query(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut member = [0u8; 32];
        member[..4].copy_from_slice(b"key\0");
        exe::wr::<u8>(member.as_mut_ptr() as usize + 0x0d, 10);
        exe::wr::<u16>(member.as_mut_ptr() as usize + 0x0e, SHORT_STRING_FLAG);
        let vflags: u16 = match r.below(4) {
            0 => 0,
            1 => 3,
            2 => 4,
            _ => r.next_u32() as u16,
        };
        exe::wr::<u16>(member.as_mut_ptr() as usize + 0x1e, vflags);
        let mut val_obj = [0u64; 2];
        exe::wr::<u32>(val_obj.as_mut_ptr() as usize, 1);
        exe::wr::<usize>(val_obj.as_mut_ptr() as usize + 8, member.as_ptr() as usize);
        let mut this_a: [u64; 3] = [0, val_obj.as_ptr() as u64, 0];
        let mut this_b = this_a;
        let q = if r.below(3) == 0 {
            b"miss\0".as_ptr()
        } else {
            b"KEY\0".as_ptr()
        };
        let ra = o(this_a.as_mut_ptr() as *mut u8, q);
        let rb = n(this_b.as_mut_ptr() as *mut u8, q);
        diff_eq!(ra != 0, rb != 0, "ret, case {i}");
        diff_eq!(this_a, this_b, "this, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_json_writer_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(orig as u64);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut a = vec![0u8; 0x168];
        for b in &mut a {
            *b = r.next_u32() as u8;
        }
        let mut b = a.clone();
        let os = r.next_u64() as usize;
        let ra = o(a.as_mut_ptr(), os);
        let rb = n(b.as_mut_ptr(), os);
        diff_eq!(ra - a.as_ptr() as usize, rb - b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(a, b, "writer, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_pool_dtor_rec(p: *mut u8) {
    POOL_LOG.with(|l| l.borrow_mut().push(("pool_dtor", p as usize, 0)));
}

unsafe extern "win64" fn fake_str_dtor_rec(s: *mut InplaceString) {
    POOL_LOG.with(|l| l.borrow_mut().push(("str_dtor", s as usize, 0)));
}

pub unsafe fn diff_json_writer_dtor<const STR_OFF: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_str = Mock::new(STRING_DTOR_VA, fake_str_dtor_rec as *const ())?;
    let _m_pool = Mock::new(0x1_413c_35d0, fake_pool_dtor_rec as *const ())?;
    let _m_del = Mock::new(J_FREE_SIZED_VA, fake_free_sized as *const ())?;
    let mut r = Rng::new(orig as u64 ^ STR_OFF as u64);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let mut w = vec![0u8; 0x168];
        let pool_ptr = if r.below(2) == 0 { 0 } else { r.next_u64() as usize };
        exe::wr::<usize>(w.as_mut_ptr() as usize + 0x10, pool_ptr);

        POOL_LOG.with(|l| l.borrow_mut().clear());
        o(w.as_mut_ptr());
        let la = POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        n(w.as_mut_ptr());
        let lb = POOL_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        diff_eq!(la, lb, "log, case {i}");
    }
    Ok(CASES)
}

