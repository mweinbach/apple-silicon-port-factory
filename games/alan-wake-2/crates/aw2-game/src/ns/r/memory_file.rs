//! `r::MemoryFile` / `r::MemoryStream` (`0x138`-byte in-memory stream with an
//! embedded `InplaceString<256>` name/tag at `+0x000..+0x108`) and
//! `r::CachedFile::~CachedFile` (`0x1424b4860`).
//!
//! Layout of `r::MemoryStream` (`0x138` bytes):
//! - `+0x000..+0x108`: `r::InplaceString<256>` (name/tag)
//! - `+0x108`: `u32 mode` (`0` = read, `1` = write)
//! - `+0x110`: `*mut u8 base`
//! - `+0x118`: `usize capacity`
//! - `+0x120`: `*mut u8 cursor`
//! - `+0x128`: `u8 is_external` (`0` = owned via `r::mem::free`, `!= 0` = borrowed)
//! - `+0x130`: `usize written_len`

use super::inplace_string::{inplace_string_assign_cstr, inplace_string_data, InplaceString, StrView};
use crate::exe;
use std::sync::atomic::{AtomicU32, Ordering};

const CACHED_FILE_VTABLE_VA: u64 = 0x1_4341_0bb8;
const REGION_FREE_PTR_VA: u64 = 0x1_4244_2de0;
const STRING_DTOR_VA: u64 = 0x1_4240_ab10;
const STRING_RESIZE_VA: u64 = 0x1_4240_a960;
const STRING_MOVE_ASSIGN_VA: u64 = 0x1_4240_a730;
const STRING_ASSIGN_RANGE_VA: u64 = 0x1_4240_9b50;
const EMPTY_CSTR_VA: u64 = 0x1_426c_dcb0;
const J_MEMCPY_VA: u64 = 0x1_4183_acbc;
const RESET_FIELDS_VA: u64 = 0x1_413d_92e0;
const STREAM_GROW_VA: u64 = 0x1_424b_48d0;

#[inline(always)]
unsafe fn construct_empty_256(this: *mut u8) {
    let t = this as usize;
    let b = (exe::rd::<u32>(t) & 0xff00_0020) | 0x20;
    exe::wr::<u32>(t, b);
    exe::wr::<u8>(t + 3, 0x20);
    exe::wr::<u32>(t + 4, 0);
    exe::wr::<u8>(t + 8, 0);
}

#[inline(always)]
unsafe fn copy_stream_fields(dst: usize, src: usize) {
    exe::wr::<u32>(dst + 0x108, exe::rd::<u32>(src + 0x108));
    exe::wr::<usize>(dst + 0x110, exe::rd::<usize>(src + 0x110));
    exe::wr::<usize>(dst + 0x118, exe::rd::<usize>(src + 0x118));
    exe::wr::<usize>(dst + 0x120, exe::rd::<usize>(src + 0x120));
    exe::wr::<u8>(dst + 0x128, exe::rd::<u8>(src + 0x128));
    exe::wr::<usize>(dst + 0x130, exe::rd::<usize>(src + 0x130));
}

/// `r::CachedFile::~CachedFile()` — sets vtable to `0x143410bb8`, releases the
/// ref-counted buffer block at `[this + 0x128]` (if non-null and `fetch_sub`
/// reaches 1, freeing `[blk]` and `blk` via `0x142442de0`), zeroes
/// `[this + 0x128]`, and destroys the embedded `InplaceString` at `this + 8`.
///
/// Original: 0x1424b4860 (101 bytes).
pub unsafe extern "win64" fn cached_file_dtor(this: *mut u8) {
    let t = this as usize;
    exe::wr::<usize>(t, exe::ptr::<u8>(CACHED_FILE_VTABLE_VA) as usize);
    let blk = exe::rd::<usize>(t + 0x128);
    if blk != 0 {
        let rc = &*((blk + 8) as *const AtomicU32);
        if rc.fetch_sub(1, Ordering::AcqRel) == 1 {
            let buf = exe::rd::<*mut u8>(blk);
            if !buf.is_null() {
                let _ = exe::call_va!(REGION_FREE_PTR_VA, buf);
            }
            let _ = exe::call_va!(REGION_FREE_PTR_VA, blk as *mut u8);
        }
        exe::wr::<usize>(t + 0x128, 0);
    }
    let _ = exe::call_va!(STRING_DTOR_VA, (t + 8) as *mut InplaceString);
}

/// Clear the embedded `InplaceString<256>` (`len = 0`, `*data() = 0`) and zero
/// all stream fields (`+0x108..+0x138`).
///
/// Original: 0x1413d92e0 (104 bytes, 7 call sites).
pub unsafe extern "win64" fn memory_stream_reset_fields(this: *mut u8) {
    let t = this as usize;
    exe::wr::<u32>(t + 4, 0);
    let d = inplace_string_data(this as *mut InplaceString);
    exe::wr::<u8>(d as usize, 0);
    exe::wr::<u32>(t + 0x108, 0);
    exe::wr::<usize>(t + 0x110, 0);
    exe::wr::<usize>(t + 0x118, 0);
    exe::wr::<usize>(t + 0x120, 0);
    exe::wr::<u8>(t + 0x128, 0);
    exe::wr::<usize>(t + 0x130, 0);
}

/// Grow the stream's buffer to at least `min_cap` bytes (`max(cap + cap / 2, min_cap)`),
/// preserving written bytes and cursor offset, and freeing the previous buffer
/// if owned (`is_external == 0`).
///
/// Original: 0x1424b48d0 (415 bytes, 7 call sites).
pub unsafe extern "win64" fn memory_stream_grow(this: *mut u8, min_cap: usize) {
    let t = this as usize;
    let cur = exe::rd::<usize>(t + 0x120);
    let offset = if cur != 0 {
        let off = cur.wrapping_sub(exe::rd::<usize>(t + 0x110));
        let w = exe::rd::<usize>(t + 0x130);
        exe::wr::<usize>(t + 0x130, w.max(off));
        off
    } else {
        0
    };
    let old_cap = exe::rd::<usize>(t + 0x118);
    let new_cap = (old_cap.wrapping_add(old_cap >> 1)).max(min_cap);
    exe::wr::<usize>(t + 0x118, new_cap);
    let new_buf = super::mem::region_alloc::<16>(new_cap);
    let old_base = exe::rd::<usize>(t + 0x110);
    if old_base != 0 {
        let cur_off = exe::rd::<usize>(t + 0x120).wrapping_sub(old_base);
        let copy_len = cur_off.max(exe::rd::<usize>(t + 0x130));
        if copy_len != 0 {
            let _ = exe::call_va!(J_MEMCPY_VA, new_buf, old_base as *const u8, copy_len);
        }
    }
    let old_base2 = exe::rd::<*mut u8>(t + 0x110);
    if !old_base2.is_null() && exe::rd::<u8>(t + 0x128) == 0 {
        let _ = exe::call_va!(REGION_FREE_PTR_VA, old_base2);
    }
    exe::wr::<*mut u8>(t + 0x110, new_buf);
    exe::wr::<*mut u8>(t + 0x120, new_buf.wrapping_add(offset));
}

/// Reset stream fields, set `mode` (`r8d`) and `name` (`rdx`), and grow to `cap` (`r9`).
///
/// Original: 0x1424b4a80 (166 bytes).
pub unsafe extern "win64" fn memory_stream_init(
    this: *mut u8,
    name: *const u8,
    mode: u32,
    cap: usize,
) {
    let _ = exe::call_va!(RESET_FIELDS_VA, this);
    exe::wr::<u32>(this as usize + 0x108, mode);
    let _ = inplace_string_assign_cstr(this as *mut InplaceString, name);
    let _ = exe::call_va!(STREAM_GROW_VA, this, cap);
}

/// Write a length-prefixed `InplaceString` (`u32 len` + `len` bytes) at `[this + 0x120]`.
///
/// Original: 0x1424b4cb0 (131 bytes, 337 call sites).
pub unsafe extern "win64" fn memory_stream_write_inplace_string(
    this: *mut u8,
    s: *const InplaceString,
) {
    let t = this as usize;
    let cur0 = exe::rd::<usize>(t + 0x120);
    let len0 = exe::rd::<u32>(s as usize + 4);
    exe::wr::<u32>(cur0, len0);
    let cur1 = exe::rd::<usize>(t + 0x120).wrapping_add(4);
    exe::wr::<usize>(t + 0x120, cur1);
    let len = exe::rd::<u32>(s as usize + 4) as usize;
    let src = inplace_string_data(s);
    let _ = exe::call_va!(J_MEMCPY_VA, cur1 as *mut u8, src, len);
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x120).wrapping_add(len));
}

/// Write a length-prefixed C-string (`u32 strlen` + `strlen` bytes) at `[this + 0x120]`.
///
/// Original: 0x1424b4d40 (82 bytes).
pub unsafe extern "win64" fn memory_stream_write_cstr(this: *mut u8, cstr: *const u8) {
    let t = this as usize;
    let mut len = 0usize;
    while exe::rd::<u8>((cstr as usize).wrapping_add(len)) != 0 {
        len += 1;
    }
    let len_u32 = len as u32;
    let len_sext = len_u32 as i32 as isize as usize;
    let cur0 = exe::rd::<usize>(t + 0x120);
    exe::wr::<u32>(cur0, len_u32);
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x120).wrapping_add(4));
    let cur1 = exe::rd::<*mut u8>(t + 0x120);
    let _ = exe::call_va!(J_MEMCPY_VA, cur1, cstr, len_sext);
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x120).wrapping_add(len_sext));
}

/// Read a length-prefixed `InplaceString` (`i32 len` + `len` bytes) from `[this + 0x120]`.
///
/// Original: 0x1424b4da0 (151 bytes, 299 call sites).
pub unsafe extern "win64" fn memory_stream_read_inplace_string(
    this: *mut u8,
    s: *mut InplaceString,
) {
    let t = this as usize;
    let cur0 = exe::rd::<usize>(t + 0x120);
    let len = exe::rd::<i32>(cur0) as i64 as u64;
    exe::wr::<usize>(t + 0x120, cur0.wrapping_add(4));
    let _ = exe::call_va!(STRING_RESIZE_VA, s, len);
    let dst = inplace_string_data(s);
    let cur1 = exe::rd::<*const u8>(t + 0x120);
    let _ = exe::call_va!(J_MEMCPY_VA, dst, cur1, len as usize);
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x120).wrapping_add(len as usize));
}

/// Copy `count` bytes from `other`'s cursor into `this`'s cursor if `other` is
/// non-null, `other`'s written length is non-zero and `>= count`, and `this`
/// has `>= count` bytes of remaining capacity; advances both cursors and updates
/// `this.written_len = this.cursor - this.base`.
///
/// Original: 0x1424b4e40 (179 bytes).
pub unsafe extern "win64" fn memory_stream_copy_from(
    this: *mut u8,
    other: *mut u8,
    count: usize,
) -> u8 {
    if other.is_null() {
        return 0;
    }
    let t = this as usize;
    let o = other as usize;
    let o_wlen = exe::rd::<usize>(o + 0x130);
    let o_cur = exe::rd::<usize>(o + 0x120);
    let o_avail = o_cur.wrapping_sub(exe::rd::<usize>(o + 0x110)).max(o_wlen);
    if o_avail == 0 || o_avail < count {
        return 0;
    }
    let t_cur = exe::rd::<usize>(t + 0x120);
    let t_rem = exe::rd::<usize>(t + 0x118)
        .wrapping_sub(t_cur)
        .wrapping_add(exe::rd::<usize>(t + 0x110));
    if count > t_rem {
        return 0;
    }
    let _ = exe::call_va!(J_MEMCPY_VA, t_cur as *mut u8, o_cur as *const u8, count);
    exe::wr::<usize>(o + 0x120, exe::rd::<usize>(o + 0x120).wrapping_add(count));
    let new_t_cur = exe::rd::<usize>(t + 0x120).wrapping_add(count);
    exe::wr::<usize>(t + 0x120, new_t_cur);
    exe::wr::<usize>(t + 0x130, new_t_cur.wrapping_sub(exe::rd::<usize>(t + 0x110)));
    1
}

/// Move-assign `other` into `this`; returns `this`.
///
/// Original: 0x1424b4f00 (161 bytes).
pub unsafe extern "win64" fn memory_stream_move_assign(
    this: *mut u8,
    other: *mut u8,
) -> *mut u8 {
    if this != other {
        let t = this as usize;
        let o = other as usize;
        let old_base = exe::rd::<*mut u8>(t + 0x110);
        if !old_base.is_null() && exe::rd::<u8>(t + 0x128) == 0 {
            let _ = exe::call_va!(REGION_FREE_PTR_VA, old_base);
        }
        let _ = exe::call_va!(
            STRING_MOVE_ASSIGN_VA,
            this as *mut InplaceString,
            other as *mut InplaceString,
        );
        copy_stream_fields(t, o);
        let _ = exe::call_va!(RESET_FIELDS_VA, other);
    }
    this
}

/// Construct a write-mode stream with initial capacity `cap`; returns `this`.
///
/// Original: 0x1424b4fb0 (119 bytes, 4 call sites).
pub unsafe extern "win64" fn memory_stream_ctor_cap(this: *mut u8, cap: usize) -> *mut u8 {
    construct_empty_256(this);
    let _ = exe::call_va!(RESET_FIELDS_VA, this);
    exe::wr::<u32>(this as usize + 0x108, 1);
    let empty = StrView { ptr: exe::ptr::<u8>(EMPTY_CSTR_VA), len: 0 };
    let _ = exe::call_va!(STRING_ASSIGN_RANGE_VA, this as *mut InplaceString, &empty, 0u64, 0u64);
    let _ = exe::call_va!(STREAM_GROW_VA, this, cap);
    this
}

/// Move-construct `this` from `other`; returns `this`.
///
/// Original: 0x1424b5030 (148 bytes, 3 call sites).
pub unsafe extern "win64" fn memory_stream_move_ctor(this: *mut u8, other: *mut u8) -> *mut u8 {
    construct_empty_256(this);
    let _ = exe::call_va!(
        STRING_MOVE_ASSIGN_VA,
        this as *mut InplaceString,
        other as *mut InplaceString,
    );
    copy_stream_fields(this as usize, other as usize);
    let _ = exe::call_va!(RESET_FIELDS_VA, other);
    this
}

/// Read `count` raw bytes from `[this + 0x120]` into `dst` and advance cursor.
///
/// Original: 0x1424b50d0 (52 bytes).
pub unsafe extern "win64" fn memory_stream_read_bytes(
    this: *mut u8,
    dst: *mut u8,
    count: usize,
) {
    let t = this as usize;
    let cur = exe::rd::<*const u8>(t + 0x120);
    let _ = exe::call_va!(J_MEMCPY_VA, dst, cur, count);
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x120).wrapping_add(count));
}

/// Rewind cursor to `base`, set `mode = 1` (write), and reset `written_len = 0`.
///
/// Original: 0x1424b5110 (36 bytes).
pub unsafe extern "win64" fn memory_stream_rewind_write(this: *mut u8) {
    let t = this as usize;
    exe::wr::<usize>(t + 0x120, exe::rd::<usize>(t + 0x110));
    exe::wr::<u32>(t + 0x108, 1);
    exe::wr::<usize>(t + 0x130, 0);
}

/// Rewind cursor to `base`, set `mode = 0` (read), and update
/// `written_len = max(written_len, old_cursor - base)`.
///
/// Original: 0x1424b5140 (56 bytes).
pub unsafe extern "win64" fn memory_stream_rewind_read(this: *mut u8) {
    let t = this as usize;
    let w = exe::rd::<usize>(t + 0x130);
    let base = exe::rd::<usize>(t + 0x110);
    let off = exe::rd::<usize>(t + 0x120).wrapping_sub(base);
    exe::wr::<u32>(t + 0x108, 0);
    exe::wr::<usize>(t + 0x120, base);
    exe::wr::<usize>(t + 0x130, w.max(off));
}

/// Destructor: frees `base` (if non-null and `is_external == 0`) via `0x142442de0`,
/// resets fields via `0x1413d92e0`, and destroys the `InplaceString<256>` at `this`.
///
/// Original: 0x1424b5180 (54 bytes, 4 call sites).
pub unsafe extern "win64" fn memory_stream_dtor(this: *mut u8) {
    let t = this as usize;
    let base = exe::rd::<*mut u8>(t + 0x110);
    if !base.is_null() && exe::rd::<u8>(t + 0x128) == 0 {
        let _ = exe::call_va!(REGION_FREE_PTR_VA, base);
    }
    let _ = exe::call_va!(RESET_FIELDS_VA, this);
    let _ = exe::call_va!(STRING_DTOR_VA, this as *mut InplaceString);
}

/// Construct a read-mode stream from `(buf, size, is_external)`; returns `this`.
///
/// Original: 0x1424b51c0 (239 bytes).
pub unsafe extern "win64" fn memory_stream_ctor_buf(
    this: *mut u8,
    buf: *mut u8,
    size: usize,
    is_external: u32,
) -> *mut u8 {
    let t = this as usize;
    construct_empty_256(this);
    let empty = StrView { ptr: exe::ptr::<u8>(EMPTY_CSTR_VA), len: 0 };
    if is_external == 0 {
        let _ = exe::call_va!(RESET_FIELDS_VA, this);
        exe::wr::<u32>(t + 0x108, 0);
        let _ = exe::call_va!(
            STRING_ASSIGN_RANGE_VA,
            this as *mut InplaceString,
            &empty,
            0u64,
            0u64,
        );
        let _ = exe::call_va!(STREAM_GROW_VA, this, size);
        let _ = exe::call_va!(J_MEMCPY_VA, exe::rd::<*mut u8>(t + 0x110), buf, size);
        exe::wr::<usize>(t + 0x130, size);
    } else {
        let _ = exe::call_va!(RESET_FIELDS_VA, this);
        let _ = exe::call_va!(
            STRING_ASSIGN_RANGE_VA,
            this as *mut InplaceString,
            &empty,
            0u64,
            0u64,
        );
        exe::wr::<u32>(t + 0x108, 0);
        exe::wr::<*mut u8>(t + 0x110, buf);
        exe::wr::<usize>(t + 0x118, size);
        exe::wr::<*mut u8>(t + 0x120, buf);
        exe::wr::<u8>(t + 0x128, 1);
        exe::wr::<usize>(t + 0x130, size);
    }
    this
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::{Mock, Rng};
use std::cell::{Cell, RefCell};

thread_local! {
    static STREAM_LOG: RefCell<Vec<(&'static str, usize, usize, usize)>> = const { RefCell::new(Vec::new()) };
    static GROW_FAKE_BUF: Cell<usize> = const { Cell::new(0) };
}

fn take_stream_log() -> Vec<(&'static str, usize, usize, usize)> {
    STREAM_LOG.with(|v| std::mem::take(&mut *v.borrow_mut()))
}

unsafe extern "win64" fn fake_free_ptr(p: *mut u8) {
    STREAM_LOG.with(|v| v.borrow_mut().push(("free", p as usize, 0, 0)));
}

unsafe extern "win64" fn fake_str_dtor(s: *mut InplaceString) {
    STREAM_LOG.with(|v| v.borrow_mut().push(("str_dtor", s as usize, 0, 0)));
}

unsafe extern "win64" fn fake_memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    STREAM_LOG.with(|v| v.borrow_mut().push(("memcpy", dst as usize, src as usize, n)));
    dst
}

unsafe extern "win64" fn fake_stream_grow(this: *mut u8, cap: usize) {
    let t = this as usize;
    let fake_buf = GROW_FAKE_BUF.with(|b| b.get());
    STREAM_LOG.with(|v| v.borrow_mut().push(("grow", t, cap, fake_buf)));
    exe::wr::<usize>(t + 0x110, fake_buf);
    exe::wr::<usize>(t + 0x118, cap);
    exe::wr::<usize>(t + 0x120, fake_buf);
}

unsafe extern "win64" fn fake_str_resize(s: *mut InplaceString, n: u64) {
    STREAM_LOG.with(|v| v.borrow_mut().push(("resize", s as usize, n as usize, 0)));
    exe::wr::<u32>(s as usize + 4, n as u32);
}

pub unsafe fn diff_cached_file_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_free_ptr as *const ())?;
    let _m_dtor = Mock::new(STRING_DTOR_VA, fake_str_dtor as *const ())?;
    let mut r = Rng::new(0x1424b4860);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    let mut blk = [0u64; 2];
    for i in 0..CASES {
        let has_blk = r.below(4) != 0;
        let rc = match r.below(4) {
            0 => 1u32,
            1 => 2u32,
            _ => r.next_u32().max(1),
        };
        let buf_ptr = if r.below(3) == 0 { 0usize } else { r.next_u64() as usize };
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        let blk_addr = if has_blk { blk.as_mut_ptr() as usize } else { 0 };
        exe::wr::<usize>(obj0.as_mut_ptr() as usize + 0x128, blk_addr);

        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            exe::wr::<usize>(blk.as_mut_ptr() as usize, buf_ptr);
            exe::wr::<u32>((blk.as_mut_ptr() as usize) + 8, rc);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8);
            (obj, blk, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "obj, case {i}");
        diff_eq!(a.1, b.1, "blk, case {i}");
        diff_eq!(a.2, b.2, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_reset_fields(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1413d92e0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    let mut heap_buf = [0u8; 32];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        let is_heap = r.below(3) == 0;
        let units = if is_heap { 0x40u32 } else { 0x20u32 };
        exe::wr::<u32>(obj0.as_mut_ptr() as usize, (0x20 << 24) | units);
        if is_heap {
            exe::wr::<usize>(obj0.as_mut_ptr() as usize + 8, heap_buf.as_mut_ptr() as usize);
        }
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            heap_buf.fill(0x5a);
            f(obj.as_mut_ptr() as *mut u8);
            (obj, heap_buf)
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "obj, case {i}");
        diff_eq!(a.1, b.1, "heap_buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, u32, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_grow = Mock::new(STREAM_GROW_VA, fake_stream_grow as *const ())?;
    let mut r = Rng::new(0x1424b4a80);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        construct_empty_256(obj0.as_mut_ptr());
        let has_name = r.below(4) != 0;
        let nlen = r.below(40) as usize;
        let mut name = Vec::with_capacity(nlen + 1);
        for _ in 0..nlen {
            name.push(b'a' + (r.below(26) as u8));
        }
        name.push(0);
        let nptr = if has_name { name.as_ptr() } else { std::ptr::null() };
        let mode = r.next_u32();
        let cap = r.next_u64() as usize;
        GROW_FAKE_BUF.with(|b| b.set(r.next_u64() as usize));

        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8, nptr, mode, cap);
            (obj, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "obj, case {i}");
        diff_eq!(a.1, b.1, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_write_inplace_string(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b4cb0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    let mut s_buf = [0u64; 6];
    let mut out_buf = [0u8; 32];
    for i in 0..CASES {
        let mut s0 = [0u8; 48];
        for b in &mut s0 {
            *b = r.next_u32() as u8;
        }
        let code = [0u8, 1, 4, 0x20, 0x80][r.below(5) as usize] as u32;
        let icap = if code < 0x80 { code * 8 } else { (code << 8) - 0x7c00 };
        let is_heap = r.below(2) == 0;
        let units = if is_heap { icap / 8 + 4 } else { icap / 8 };
        exe::wr::<u32>(s0.as_mut_ptr() as usize, (code << 24) | units);
        exe::wr::<u32>(s0.as_mut_ptr() as usize + 4, r.below(1024) as u32);

        let cur_ptr = out_buf.as_mut_ptr() as usize + (i as usize % 8);
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(s0.as_ptr(), s_buf.as_mut_ptr() as *mut u8, 48);
            out_buf.fill(0);
            exe::wr::<usize>((obj.as_mut_ptr() as usize) + 0x120, cur_ptr);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8, s_buf.as_ptr() as *const InplaceString);
            (exe::rd::<usize>((obj.as_ptr() as usize) + 0x120), out_buf, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "cursor, case {i}");
        diff_eq!(a.1, b.1, "out_buf, case {i}");
        diff_eq!(a.2, b.2, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_write_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b4d40);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    let mut out_buf = [0u8; 32];
    for i in 0..CASES {
        let clen = r.below(64) as usize;
        let mut cstr = vec![b'x'; clen];
        cstr.push(0);
        let cur_ptr = out_buf.as_mut_ptr() as usize + (i as usize % 8);
        let mut run = |f: F| {
            out_buf.fill(0);
            exe::wr::<usize>((obj.as_mut_ptr() as usize) + 0x120, cur_ptr);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8, cstr.as_ptr());
            (exe::rd::<usize>((obj.as_ptr() as usize) + 0x120), out_buf, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "cursor, case {i}");
        diff_eq!(a.1, b.1, "out_buf, case {i}");
        diff_eq!(a.2, b.2, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_read_inplace_string(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_res = Mock::new(STRING_RESIZE_VA, fake_str_resize as *const ())?;
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b4da0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    let mut s_buf = [0u64; 6];
    let mut in_buf = [0u8; 32];
    for i in 0..CASES {
        let mut s0 = [0u8; 48];
        for b in &mut s0 {
            *b = r.next_u32() as u8;
        }
        let code = [0u8, 1, 4, 0x20, 0x80][r.below(5) as usize] as u32;
        let icap = if code < 0x80 { code * 8 } else { (code << 8) - 0x7c00 };
        let is_heap = r.below(2) == 0;
        let units = if is_heap { icap / 8 + 4 } else { icap / 8 };
        exe::wr::<u32>(s0.as_mut_ptr() as usize, (code << 24) | units);

        let cur_ptr = in_buf.as_mut_ptr() as usize + (i as usize % 8);
        let raw_len = match r.below(4) {
            0 => 0i32,
            1 => r.below(256) as i32,
            2 => -1i32,
            _ => r.next_u32() as i32,
        };
        exe::wr::<i32>(cur_ptr, raw_len);

        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(s0.as_ptr(), s_buf.as_mut_ptr() as *mut u8, 48);
            exe::wr::<usize>((obj.as_mut_ptr() as usize) + 0x120, cur_ptr);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8, s_buf.as_mut_ptr() as *mut InplaceString);
            (exe::rd::<usize>((obj.as_ptr() as usize) + 0x120), s_buf, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "cursor, case {i}");
        diff_eq!(a.1, b.1, "s_buf, case {i}");
        diff_eq!(a.2, b.2, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_copy_from(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b4e40);
    const CASES: u32 = 300;
    let mut this_obj = [0u64; 0x138 / 8];
    let mut other_obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let null_other = r.below(8) == 0;
        let t_base = 0x1000_0000usize;
        let t_cap = r.below(1024) as usize;
        let t_cur = t_base + r.below((t_cap + 32) as u64) as usize;
        let t_wlen = r.below(1024) as usize;

        let o_base = 0x2000_0000usize;
        let o_cur = o_base + r.below(512) as usize;
        let o_wlen = if r.below(4) == 0 { 0 } else { r.below(1024) as usize };
        let count = match r.below(5) {
            0 => 0,
            1 => o_wlen,
            2 => (t_base + t_cap).wrapping_sub(t_cur),
            _ => r.below(1024) as usize,
        };

        let mut run = |f: F| {
            let t = this_obj.as_mut_ptr() as usize;
            let op = other_obj.as_mut_ptr() as usize;
            exe::wr::<usize>(t + 0x110, t_base);
            exe::wr::<usize>(t + 0x118, t_cap);
            exe::wr::<usize>(t + 0x120, t_cur);
            exe::wr::<usize>(t + 0x130, t_wlen);
            exe::wr::<usize>(op + 0x110, o_base);
            exe::wr::<usize>(op + 0x120, o_cur);
            exe::wr::<usize>(op + 0x130, o_wlen);
            take_stream_log();
            let other_arg = if null_other { std::ptr::null_mut() } else { op as *mut u8 };
            let ret = f(t as *mut u8, other_arg, count) != 0;
            (
                ret,
                exe::rd::<usize>(t + 0x120),
                exe::rd::<usize>(t + 0x130),
                exe::rd::<usize>(op + 0x120),
                take_stream_log(),
            )
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a, b, "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_move_assign(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_free_ptr as *const ())?;
    let mut r = Rng::new(0x1424b4f00);
    const CASES: u32 = 200;
    let mut this_obj = [0u64; 0x138 / 8];
    let mut other_obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut t0 = [0u8; 0x138];
        let mut o0 = [0u8; 0x138];
        for b in &mut t0 {
            *b = r.next_u32() as u8;
        }
        for b in &mut o0 {
            *b = r.next_u32() as u8;
        }
        construct_empty_256(t0.as_mut_ptr());
        construct_empty_256(o0.as_mut_ptr());
        let o_len = r.below(32) as usize;
        for k in 0..o_len {
            o0[8 + k] = b'a' + (k as u8 % 26);
        }
        o0[8 + o_len] = 0;
        exe::wr::<u32>(o0.as_mut_ptr() as usize + 4, o_len as u32);
        if r.below(3) == 0 {
            exe::wr::<usize>(t0.as_mut_ptr() as usize + 0x110, 0);
        }
        t0[0x128] = (r.below(2) == 0) as u8;
        let self_assign = r.below(8) == 0;

        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(t0.as_ptr(), this_obj.as_mut_ptr() as *mut u8, 0x138);
            std::ptr::copy_nonoverlapping(o0.as_ptr(), other_obj.as_mut_ptr() as *mut u8, 0x138);
            take_stream_log();
            let tp = this_obj.as_mut_ptr() as *mut u8;
            let op = if self_assign { tp } else { other_obj.as_mut_ptr() as *mut u8 };
            let ret = f(tp, op);
            (ret, this_obj, other_obj, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "ret, case {i}");
        diff_eq!(a.1, b.1, "this, case {i}");
        diff_eq!(a.2, b.2, "other, case {i}");
        diff_eq!(a.3, b.3, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_ctor_cap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_grow = Mock::new(STREAM_GROW_VA, fake_stream_grow as *const ())?;
    let mut r = Rng::new(0x1424b4fb0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        let cap = r.next_u64() as usize;
        GROW_FAKE_BUF.with(|b| b.set(r.next_u64() as usize));
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            take_stream_log();
            let ret = f(obj.as_mut_ptr() as *mut u8, cap);
            (ret, obj, take_stream_log())
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "ret, case {i}");
        diff_eq!(a.1, b.1, "obj, case {i}");
        diff_eq!(a.2, b.2, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_move_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1424b5030);
    const CASES: u32 = 200;
    let mut this_obj = [0u64; 0x138 / 8];
    let mut other_obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut t0 = [0u8; 0x138];
        let mut o0 = [0u8; 0x138];
        for b in &mut t0 {
            *b = r.next_u32() as u8;
        }
        for b in &mut o0 {
            *b = r.next_u32() as u8;
        }
        construct_empty_256(o0.as_mut_ptr());
        let o_len = r.below(32) as usize;
        for k in 0..o_len {
            o0[8 + k] = b'a' + (k as u8 % 26);
        }
        o0[8 + o_len] = 0;
        exe::wr::<u32>(o0.as_mut_ptr() as usize + 4, o_len as u32);

        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(t0.as_ptr(), this_obj.as_mut_ptr() as *mut u8, 0x138);
            std::ptr::copy_nonoverlapping(o0.as_ptr(), other_obj.as_mut_ptr() as *mut u8, 0x138);
            let ret = f(this_obj.as_mut_ptr() as *mut u8, other_obj.as_mut_ptr() as *mut u8);
            (ret, this_obj, other_obj)
        };
        let a = run(o);
        let b = run(n);
        diff_eq!(a.0, b.0, "ret, case {i}");
        diff_eq!(a.1, b.1, "this, case {i}");
        diff_eq!(a.2, b.2, "other, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_read_bytes(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b50d0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let cur0 = r.next_u64() as usize;
        let dst = r.next_u64() as usize as *mut u8;
        let count = r.next_u64() as usize;
        let mut run = |f: F| {
            exe::wr::<usize>((obj.as_mut_ptr() as usize) + 0x120, cur0);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8, dst, count);
            (exe::rd::<usize>((obj.as_ptr() as usize) + 0x120), take_stream_log())
        };
        diff_eq!(run(o), run(n), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_rewind(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1424b5110 ^ orig as u64);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            f(obj.as_mut_ptr() as *mut u8);
            obj
        };
        diff_eq!(run(o), run(n), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_free_ptr as *const ())?;
    let _m_dtor = Mock::new(STRING_DTOR_VA, fake_str_dtor as *const ())?;
    let mut r = Rng::new(0x1424b5180);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        construct_empty_256(obj0.as_mut_ptr());
        if r.below(3) == 0 {
            exe::wr::<usize>(obj0.as_mut_ptr() as usize + 0x110, 0);
        }
        obj0[0x128] = (r.below(2) == 0) as u8;
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            take_stream_log();
            f(obj.as_mut_ptr() as *mut u8);
            (obj, take_stream_log())
        };
        diff_eq!(run(o), run(n), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_memory_stream_ctor_buf(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_grow = Mock::new(STREAM_GROW_VA, fake_stream_grow as *const ())?;
    let _m_cpy = Mock::new(J_MEMCPY_VA, fake_memcpy as *const ())?;
    let mut r = Rng::new(0x1424b51c0);
    const CASES: u32 = 200;
    let mut obj = [0u64; 0x138 / 8];
    for i in 0..CASES {
        let mut obj0 = [0u8; 0x138];
        for b in &mut obj0 {
            *b = r.next_u32() as u8;
        }
        let buf = r.next_u64() as usize as *mut u8;
        let size = r.next_u64() as usize;
        let is_ext = if r.below(2) == 0 { 0u32 } else { r.next_u32().max(1) };
        GROW_FAKE_BUF.with(|b| b.set(r.next_u64() as usize));
        let mut run = |f: F| {
            std::ptr::copy_nonoverlapping(obj0.as_ptr(), obj.as_mut_ptr() as *mut u8, 0x138);
            take_stream_log();
            let ret = f(obj.as_mut_ptr() as *mut u8, buf, size, is_ext);
            (ret, obj, take_stream_log())
        };
        diff_eq!(run(o), run(n), "case {i}");
    }
    Ok(CASES)
}
