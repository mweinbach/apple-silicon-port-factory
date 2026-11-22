//! `r::InplaceString<N>` — Remedy's small-buffer string.
//!
//! Layout (recovered from the functions below and from embedded instances,
//! e.g. `coregame::ProgramFlowLoadingStateParameters` built by 0x14184a9b0
//! has three of them at +0x24/+0xac/+0x134, stride 0x88 = 8 + 128):
//!
//! | off | size | field |
//! |-----|------|-------|
//! | +0  | u32  | bits 0..24: capacity in 8-byte units; bits 24..32 (byte +3): inline-capacity code |
//! | +4  | u32  | length (bytes, excluding the NUL) |
//! | +8  | N / 8 | inline chars, or (when capacity > inline capacity) a heap `char*` |
//!
//! Inline capacity code `c` → bytes: `c < 0x80 ? c*8 : c*256 - 0x7c00`.
//! Objects are only 4-byte aligned when embedded (e.g. at +0x24), so the heap
//! pointer is read unaligned.
//!
//! Ownership: RTTI has `r::InplaceString<64>` (`.?AV?$InplaceString@$0EA@@r@@`).
//! The out-of-line members live in the r:: core block (0x142409xxx-0x14240axxx,
//! between `r::ThreadManager::Worker` and `r::StreamingFile` vftable methods);
//! the constructors are COMDAT instances next to their game-code users. Case-
//! insensitive search uses the ASCII lower-case table at 0x1427e54e0 (.rdata).
//!
//! The member names (`data`, `isInline`, `clear`, `erase`, `rfind`, `find`)
//! are inferred from behavior; the exe has no symbols for them.

use crate::exe;

/// ASCII to-lower table in `.rdata` (`table['A'] == 'a'`, identity elsewhere).
const TOLOWER_TABLE_VA: u64 = 0x1_427e_54e0;

#[inline(always)]
unsafe fn tolower(c: u8) -> u8 {
    *exe::ptr::<u8>(TOLOWER_TABLE_VA).add(c as usize)
}

/// ASCII to-upper table in `.rdata` (`table['a'] == 'A'`, identity elsewhere).
/// The `*_nocase` comparisons that order strings use this one (as `i8`).
const TOUPPER_TABLE_VA: u64 = 0x1_426d_01e0;

#[inline(always)]
unsafe fn toupper(c: u8) -> u8 {
    *exe::ptr::<u8>(TOUPPER_TABLE_VA).add(c as usize)
}

/// Opaque header; see the module docs.
#[repr(C, packed)]
pub struct InplaceString {
    pub bits: u32,
    pub len: u32,
    // +8: inline chars or heap pointer
}
const _: () = assert!(std::mem::size_of::<InplaceString>() == 8);

#[inline(always)]
unsafe fn bits(s: *const InplaceString) -> u32 {
    std::ptr::read_unaligned(s as *const u32)
}

/// Inline capacity in bytes from the code in byte +3.
#[inline(always)]
unsafe fn inline_capacity(s: *const InplaceString) -> u32 {
    let c = *(s as *const u8).add(3) as u32;
    if c < 0x80 {
        c * 8
    } else {
        (c << 8).wrapping_sub(0x7c00)
    }
}

/// Current capacity in bytes (24-bit field in 8-byte units).
#[inline(always)]
unsafe fn capacity(s: *const InplaceString) -> u32 {
    (bits(s) & 0xff_ffff) << 3
}

/// Character storage: the inline buffer at +8, or the heap pointer stored there.
#[inline(always)]
unsafe fn storage(s: *const InplaceString) -> *mut u8 {
    let inline = (s as *mut u8).add(8);
    if capacity(s) > inline_capacity(s) {
        std::ptr::read_unaligned(inline as *const *mut u8)
    } else {
        inline
    }
}

#[inline(always)]
unsafe fn construct_empty(this: *mut InplaceString, units: u8) -> *mut InplaceString {
    let p = this as *mut u32;
    let mut b = std::ptr::read_unaligned(p);
    b &= 0xff00_0000 | units as u32; // `and dword [rcx], 0xff0000NN`
    std::ptr::write_unaligned(p, b);
    std::ptr::write_unaligned(p, b | units as u32); // capacity = N/8 units
    *(this as *mut u8).add(3) = units; // inline-capacity code
    std::ptr::write_unaligned((this as *mut u32).add(1), 0); // len
    *(this as *mut u8).add(8) = 0; // NUL
    this
}

/// `r::InplaceString<128>::InplaceString()` — empty string, 128 inline bytes.
///
/// Original: 0x1401df360 (28 bytes, leaf). `and [rcx],0xff000010; or [rcx],0x10;
/// mov byte [rcx+3],0x10; mov dword [rcx+4],0; mov byte [rcx+8],0; ret rcx`.
/// Evidence for N=128: same init sequence inlined three times in 0x14184a9b0
/// (`ProgramFlowServerStateLoadingLevel` ctor) at stride 0x88. Boot: ~9.2k calls.
pub unsafe extern "win64" fn inplace_string128_ctor(this: *mut InplaceString) -> *mut InplaceString {
    construct_empty(this, 0x10)
}

/// `r::InplaceString<32>::InplaceString()` — empty string, 32 inline bytes.
///
/// Original: 0x1401eeda0 (28 bytes, leaf); identical to 0x1401df360 with code 4.
pub unsafe extern "win64" fn inplace_string32_ctor(this: *mut InplaceString) -> *mut InplaceString {
    construct_empty(this, 0x04)
}

/// `r::InplaceString<>::data()` — pointer to the characters (inline or heap).
///
/// Original: 0x1401df680 (54 bytes, leaf, 211 callers; N-independent so all
/// instantiations fold to it). Boot: ~3.7k calls.
pub unsafe extern "win64" fn inplace_string_data(this: *const InplaceString) -> *mut u8 {
    storage(this)
}

/// Accessor returning `.data()` of an `r::InplaceString<>` member embedded at
/// byte offset `OFFSET` inside `this` (e.g. `r::File::vfunc_1` at `+0x08`,
/// `r::MemoryFile::vfunc_1` at `+0x0c`, `r::FileArchive::File::vfunc_1` at `+0x28`,
/// and unnamed accessors at `+0x10`, `+0x40`, `+0x50`, `+0x80`, `+0x110`, `+0x140`).
///
/// Originals: 15 functions of 55 or 64 bytes listed in `registry.rs`
/// (including 0x1401e7bd0 with 3,440 call sites).
pub unsafe extern "win64" fn inplace_string_data_at<const OFFSET: usize>(this: *const u8) -> *mut u8 {
    storage(this.add(OFFSET) as *const InplaceString)
}

/// `r::InplaceString<>::isInline()` — `capacity <= inline capacity`.
///
/// Original: 0x1401df6c0 (60 bytes, leaf, 28 callers). Returns `setbe al`.
pub unsafe extern "win64" fn inplace_string_is_inline(this: *const InplaceString) -> bool {
    capacity(this) <= inline_capacity(this)
}

/// `r::InplaceString<>::erase(pos, count)` — remove `count` chars at `pos`.
///
/// Original: 0x142409430 (189 bytes, leaf, 7 callers). No-op if `pos > len`;
/// `count` is clamped to `len - pos`; moves the tail *including the NUL*
/// forward byte by byte; `len -= count` (32-bit); returns `this`.
pub unsafe extern "win64" fn inplace_string_erase(this: *mut InplaceString, pos: u64, count: u64) -> *mut InplaceString {
    let len = (*this).len as u64;
    if pos > len {
        return this;
    }
    let avail = len - pos;
    let n = if count > avail { avail } else { count };
    let d = storage(this).add(pos as usize);
    // r9 = avail - n + 1 iterations (the +1 copies the terminator).
    let iters = (avail - n).wrapping_add(1);
    for i in 0..iters as usize {
        *d.add(i) = *d.add(i + n as usize);
    }
    (*this).len = (*this).len.wrapping_sub(n as u32);
    this
}

/// `r::InplaceString<>::rfind(char)` — last index whose char equals `c`
/// ignoring ASCII case, or -1.
///
/// Original: 0x142409500 (165 bytes, leaf, 3 callers). Starts at
/// `min(npos, len-1)` (a folded `pos = npos` default) and walks down to 0.
pub unsafe extern "win64" fn inplace_string_rfind_char_nocase(this: *const InplaceString, c: u8) -> i64 {
    let len = (*this).len as u64;
    if len == 0 {
        return -1;
    }
    let want = tolower(c);
    let mut i = (len - 1).min(u64::MAX);
    let s = storage(this);
    loop {
        if tolower(*s.add(i as usize)) == want {
            return i as i64;
        }
        if i == 0 {
            return -1;
        }
        i -= 1;
    }
}

/// A `(pointer, length)` view, as passed by reference (MSVC
/// `std::basic_string_view<char, r::CharTraits<char>>` layout).
#[repr(C, packed)]
pub struct StrView {
    pub ptr: *const u8,
    pub len: u64,
}
const _: () = assert!(std::mem::size_of::<StrView>() == 16);

/// `r::InplaceString<>::find(view, pos)` — first index `>= pos` where `view`
/// matches ignoring ASCII case, or -1.
///
/// Original: 0x14240a490 (243 bytes, leaf, 16 callers). Returns -1 if
/// `pos >= len` or the view is empty. The candidate range uses 32-bit signed
/// arithmetic: `last = (i32)(len - view.len)` (32-bit subtraction), the start
/// index is `(i32)pos`, and the loop runs while `i <= last`.
pub unsafe extern "win64" fn inplace_string_find_nocase(this: *const InplaceString, needle: *const StrView, pos: u64) -> i64 {
    let len = (*this).len;
    if pos >= len as u64 {
        return -1;
    }
    let nlen = (*needle).len;
    if nlen == 0 {
        return -1;
    }
    let hay = storage(this);
    let last = len.wrapping_sub(nlen as u32) as i32;
    let start = pos as u32 as i32;
    if start > last {
        return -1;
    }
    let np = (*needle).ptr;
    let mut i = start as i64;
    while i <= last as i64 {
        let mut k = 0u64;
        loop {
            let h = *hay.offset(i as isize + k as isize);
            if tolower(h) != tolower(*np.add(k as usize)) {
                break;
            }
            if k == nlen - 1 {
                return i;
            }
            k += 1;
            if k >= nlen {
                break;
            }
        }
        i += 1;
    }
    -1
}

/// `r::InplaceString<>::clear()` — length 0 and a NUL at the start.
///
/// Original: 0x14240a8f0 (64 bytes, leaf, 9 callers). Returns nothing.
pub unsafe extern "win64" fn inplace_string_clear(this: *mut InplaceString) {
    (*this).len = 0;
    *storage(this) = 0;
}

/// `r::InplaceString<>::find_nocase(char)` — first index whose char equals `c`
/// ignoring ASCII case, or -1.
///
/// Original: 0x142409270 (107 bytes, leaf). Same as 0x1424093b0 with pos 0.
pub unsafe extern "win64" fn inplace_string_find_char_nocase(this: *const InplaceString, c: u8) -> i64 {
    inplace_string_find_char_nocase_from(this, c, 0)
}

/// `r::InplaceString<>::find_nocase(char, pos)` — first index `>= pos` whose
/// char equals `c` ignoring ASCII case, or -1.
///
/// Original: 0x1424093b0 (111 bytes, leaf). `pos` is compared 64-bit with the
/// 32-bit length.
pub unsafe extern "win64" fn inplace_string_find_char_nocase_from(this: *const InplaceString, c: u8, pos: u64) -> i64 {
    let want = tolower(c);
    let s = storage(this);
    let len = (*this).len as u64;
    let mut i = pos;
    while i < len {
        if tolower(*s.add(i as usize)) == want {
            return i as i64;
        }
        i += 1;
    }
    -1
}

/// `r::InplaceString<>::assign(char)` — the string becomes `c` (length 1).
///
/// Original: 0x1424092f0 (76 bytes, leaf). No capacity check; returns `this`.
pub unsafe extern "win64" fn inplace_string_assign_char(this: *mut InplaceString, c: u8) -> *mut InplaceString {
    let s = storage(this);
    *s = c;
    *s.add(1) = 0;
    (*this).len = 1;
    this
}

/// `r::InplaceString<>::replace(char from, char to)` — case-sensitive, all
/// occurrences; returns `this`.
///
/// Original: 0x1424095b0 (99 bytes, leaf). The length is re-read every step.
pub unsafe extern "win64" fn inplace_string_replace_char(this: *mut InplaceString, from: u8, to: u8) -> *mut InplaceString {
    let s = storage(this);
    if (*this).len != 0 {
        let mut i = 0u64;
        loop {
            if *s.add(i as usize) == from {
                *s.add(i as usize) = to;
            }
            i += 1;
            if i >= (*this).len as u64 {
                break;
            }
        }
    }
    this
}

/// `r::InplaceString<>::toLower()` — in place via the 0x1427e54e0 table; returns `this`.
///
/// Original: 0x142409690 (105 bytes, leaf).
pub unsafe extern "win64" fn inplace_string_to_lower(this: *mut InplaceString) -> *mut InplaceString {
    let n = (*this).len as usize;
    let s = storage(this);
    for i in 0..n {
        *s.add(i) = tolower(*s.add(i));
    }
    this
}

/// `r::InplaceString<>::toUpper()` — in place via the 0x1426d01e0 table; returns `this`.
///
/// Original: 0x142409ae0 (105 bytes, leaf; same code as 0x142409690 with the
/// to-upper table).
pub unsafe extern "win64" fn inplace_string_to_upper(this: *mut InplaceString) -> *mut InplaceString {
    let n = (*this).len as usize;
    let s = storage(this);
    for i in 0..n {
        *s.add(i) = toupper(*s.add(i));
    }
    this
}

/// `r::InplaceString<>::find(view, pos)` — case-sensitive variant of
/// [`inplace_string_find_nocase`] (same 32-bit signed index arithmetic).
///
/// Original: 0x142409750 (235 bytes, leaf).
pub unsafe extern "win64" fn inplace_string_find(this: *const InplaceString, needle: *const StrView, pos: u64) -> i64 {
    let len = (*this).len;
    if pos >= len as u64 {
        return -1;
    }
    let nlen = (*needle).len;
    if nlen == 0 {
        return -1;
    }
    let hay = storage(this);
    let last = len.wrapping_sub(nlen as u32) as i32;
    let start = pos as u32 as i32;
    if start > last {
        return -1;
    }
    let np = (*needle).ptr;
    let mut i = start as i64;
    while i <= last as i64 {
        let mut k = 0u64;
        loop {
            if *hay.offset(i as isize + k as isize) != *np.add(k as usize) {
                break;
            }
            if k == nlen - 1 {
                return i;
            }
            k += 1;
            if k >= nlen {
                break;
            }
        }
        i += 1;
    }
    -1
}

/// `r::InplaceString<>::endsWith_nocase(view)`.
///
/// Original: 0x1424098f0 (150 bytes, leaf). An empty view matches only an
/// empty string; otherwise compares the last `view.len` chars through the
/// to-upper table.
pub unsafe extern "win64" fn inplace_string_ends_with_nocase(this: *const InplaceString, v: *const StrView) -> bool {
    let n = (*v).len;
    let len = (*this).len as u64;
    if n == 0 {
        return len == 0;
    }
    if len < n {
        return false;
    }
    let s = storage(this).add((len - n) as usize);
    let p = (*v).ptr;
    for k in 0..n as usize {
        if toupper(*s.add(k)) as i8 != toupper(*p.add(k)) as i8 {
            return false;
        }
    }
    true
}

/// `r::InplaceString<>::startsWith_nocase(view)`.
///
/// Original: 0x142409990 (151 bytes, leaf). False if the view is longer; an
/// empty view matches only an empty string; compares through the to-lower table.
pub unsafe extern "win64" fn inplace_string_starts_with_nocase(this: *const InplaceString, v: *const StrView) -> bool {
    let n = (*v).len;
    let len = (*this).len as u64;
    if len < n {
        return false;
    }
    if n == 0 {
        return len == 0;
    }
    let s = storage(this);
    let p = (*v).ptr;
    for k in 0..n as usize {
        if tolower(*p.add(k)) != tolower(*s.add(k)) {
            return false;
        }
    }
    true
}

/// `r::InplaceString<>::compare_nocase(view)` — -1 / 0 / 1.
///
/// Original: 0x142409a30 (162 bytes, leaf). Compares `min(len, view.len)`
/// chars as `(i8)toupper(c)`; ties are broken by length.
pub unsafe extern "win64" fn inplace_string_compare_nocase(this: *const InplaceString, v: *const StrView) -> i32 {
    let vn = (*v).len;
    let len = (*this).len as u64;
    let n = if vn < len { vn } else { len };
    let s = storage(this);
    let p = (*v).ptr;
    for k in 0..n as usize {
        let (a, b) = (toupper(*s.add(k)) as i8, toupper(*p.add(k)) as i8);
        if a < b {
            return -1;
        }
        if a > b {
            return 1;
        }
    }
    if len > vn {
        1
    } else if len == vn {
        0
    } else {
        -1
    }
}

/// `a < b` for two `r::InplaceString`s, case-insensitive. The comparator of
/// the engine's string-keyed ordered maps. Compares `min(a.len, b.len)` chars
/// as `(i8)toupper(c)`; on a tie the shorter string is less. Returns 1/0 in
/// the full `eax` (both exits zero-extend).
///
/// Original: 0x1402174f0 (238 bytes, leaf, 142 callers; 11% of the main
/// thread's interpreter samples in the post-path-port profile).
pub unsafe extern "win64" fn inplace_string_less_nocase(a: *const InplaceString, b: *const InplaceString) -> u64 {
    let (pa, pb) = (storage(a), storage(b));
    let (la, lb) = (len_of(a) as u64, len_of(b) as u64);
    for k in 0..la.min(lb) as usize {
        let (x, y) = (toupper(*pa.add(k)) as i8, toupper(*pb.add(k)) as i8);
        if x < y {
            return 1;
        }
        if x > y {
            return 0;
        }
    }
    (la < lb) as u64
}

// ---------------------------------------------------------------------------
// Non-leaf members. Callees are called through their exe addresses (exactly
// like the original's `call rel32`, so hooks and difftest mocks apply).
// ---------------------------------------------------------------------------

/// `reserve(this, n)` (not decompiled): grows the buffer to hold `n` chars +
/// NUL via the engine allocator (0x140209f30), copying with `strncpy_s`.
const RESERVE_VA: u64 = 0x1_4240_8f50;
/// `assign(this, &view, pos, count)` — 0x142409b50 (below).
const ASSIGN_VIEW_RANGE_VA: u64 = 0x1_4240_9b50;
/// `append(this, &view)` — 0x14240a630 (below).
const APPEND_VIEW_VA: u64 = 0x1_4240_a630;
/// CRT import thunks (`jmp [IAT]`).
const J_MEMCPY_VA: u64 = 0x1_4183_acbc;
const J_MEMSET_VA: u64 = 0x1_4183_acc2;
const J_MEMMOVE_VA: u64 = 0x1_4183_acc8;

#[inline(always)]
unsafe fn reserve(this: *mut InplaceString, n: u64) {
    exe::call_va!(RESERVE_VA, (this, n));
}

#[inline(always)]
unsafe fn len_of(s: *const InplaceString) -> u32 {
    std::ptr::read_unaligned((s as *const u32).add(1))
}

#[inline(always)]
unsafe fn set_len(s: *mut InplaceString, n: u32) {
    std::ptr::write_unaligned((s as *mut u32).add(1), n)
}

/// `r::InplaceString<>::push_back(char)` — returns `this`.
///
/// Original: 0x14240a590 (152 bytes). Calls `reserve(this, len + 1)` when
/// `len + 1 >= capacity` (64-bit compare), then writes the char and a NUL
/// (at the re-read length + 1) and increments the length.
pub unsafe extern "win64" fn inplace_string_push_back(this: *mut InplaceString, c: u8) -> *mut InplaceString {
    let mut len = len_of(this) as u64;
    if len + 1 >= capacity(this) as u64 {
        reserve(this, len + 1);
        len = len_of(this) as u64;
    }
    let d = storage(this);
    *d.add(len as usize) = c;
    *d.add(len_of(this) as usize + 1) = 0;
    set_len(this, len_of(this).wrapping_add(1));
    this
}

/// `r::InplaceString<>::append(view)` — returns `this`.
///
/// Original: 0x14240a630 (243 bytes). No-op for an empty view. `n = len +
/// view.len` (64-bit); `reserve(this, n)` when `n >= capacity`; `memcpy` to
/// `data + len` (len re-read after reserve); NUL at `n`; `len = (u32)n`.
pub unsafe extern "win64" fn inplace_string_append(this: *mut InplaceString, v: *const StrView) -> *mut InplaceString {
    let vlen = (*v).len;
    if vlen == 0 {
        return this;
    }
    let n = (len_of(this) as u64).wrapping_add(vlen);
    if n >= capacity(this) as u64 {
        reserve(this, n);
    }
    let dst = storage(this).add(len_of(this) as usize);
    exe::call_va!(J_MEMCPY_VA, (dst, (*v).ptr, vlen) -> *mut u8);
    *storage(this).add(n as usize) = 0;
    set_len(this, n as u32);
    this
}

/// `r::InplaceString<>::append(view, pos, count)` — appends
/// `view[pos .. pos + min(count, view.len - pos)]`; no-op if `pos >= view.len`.
///
/// Original: 0x142409350 (78 bytes); calls 0x14240a630. Returns `this`.
pub unsafe extern "win64" fn inplace_string_append_range(
    this: *mut InplaceString,
    v: *const StrView,
    pos: u64,
    count: u64,
) -> *mut InplaceString {
    let vlen = (*v).len;
    if pos < vlen {
        let rest = vlen - pos;
        let sub = StrView { ptr: (*v).ptr.add(pos as usize), len: if rest < count { rest } else { count } };
        exe::call_va!(APPEND_VIEW_VA, (this, &sub as *const StrView) -> *mut InplaceString);
    }
    this
}

/// `r::InplaceString<>::assign(view, pos, count)` — no return value.
///
/// Original: 0x142409b50 (451 bytes, ~10k call sites). No-op if `pos >
/// view.len`; `count == 0` clears. Otherwise `count = min(count, view.len -
/// pos)`; if the *whole view* lies inside `[data, data + len]` the chars are
/// `memmove`d (destination from `data()` 0x1401df680), else `reserve(this,
/// count)` when `count >= capacity` and `memcpy`. Then NUL and `len = count`.
pub unsafe extern "win64" fn inplace_string_assign_range(this: *mut InplaceString, v: *const StrView, pos: u64, count: u64) {
    let vlen = (*v).len;
    if pos > vlen {
        return;
    }
    let mut count = count;
    if count == 0 {
        set_len(this, 0);
        *storage(this) = 0;
        return;
    }
    if vlen - pos < count {
        count = vlen - pos;
    }
    let vptr = (*v).ptr;
    let d = storage(this);
    if vptr as usize >= d as usize && (vptr as usize).wrapping_add(vlen as usize) <= (d as usize).wrapping_add(len_of(this) as usize) {
        exe::call_va!(J_MEMMOVE_VA, (inplace_string_data(this), vptr.add(pos as usize), count) -> *mut u8);
    } else {
        if count >= capacity(this) as u64 {
            reserve(this, count);
        }
        exe::call_va!(J_MEMCPY_VA, (storage(this), vptr.add(pos as usize), count) -> *mut u8);
    }
    *storage(this).add(count as usize) = 0;
    set_len(this, count as u32);
}

#[inline(always)]
unsafe fn assign_range(this: *mut InplaceString, v: &StrView, pos: u64, count: u64) {
    exe::call_va!(ASSIGN_VIEW_RANGE_VA, (this, v as *const StrView, pos, count));
}

/// `r::InplaceString<>::assign(view)` — `assign(view, 0, view.len)` on a
/// copy of the view; returns `this`.
///
/// Original: 0x14240a120 (45 bytes).
pub unsafe extern "win64" fn inplace_string_assign_view(this: *mut InplaceString, v: *const StrView) -> *mut InplaceString {
    let copy = StrView { ptr: (*v).ptr, len: (*v).len };
    assign_range(this, &copy, 0, copy.len);
    this
}

/// `r::InplaceString<>::operator=(const InplaceString&)` — copy assignment
/// (no-op on self-assignment); returns `this`.
///
/// Original: 0x14240aa00 (122 bytes); `assign({other.data, other.len}, 0, other.len)`.
pub unsafe extern "win64" fn inplace_string_copy_assign(this: *mut InplaceString, other: *const InplaceString) -> *mut InplaceString {
    if this as *const InplaceString != other {
        let n = len_of(other) as u64;
        let v = StrView { ptr: storage(other), len: n };
        assign_range(this, &v, 0, n);
    }
    this
}

/// `r::InplaceString<>::operator=(const char*)` — `nullptr` clears, else
/// `assign({s, strlen(s)}, 0, strlen(s))`; returns `this`.
///
/// Original: 0x14240aa80 (138 bytes).
pub unsafe extern "win64" fn inplace_string_assign_cstr(this: *mut InplaceString, s: *const u8) -> *mut InplaceString {
    if s.is_null() {
        set_len(this, 0);
        *storage(this) = 0;
        return this;
    }
    let mut n = 0u64;
    while *s.add(n as usize) != 0 {
        n += 1;
    }
    let v = StrView { ptr: s, len: n };
    assign_range(this, &v, 0, n);
    this
}

/// `r::InplaceString<>::substr(pos, count)` — constructs `*out` as an empty
/// `InplaceString<256>` (code 0x20) and assigns `{data, len}[pos, count]`
/// into it; returns `out`.
///
/// Original: 0x142409850 (151 bytes). The `[rsp+0x20]` writes are the EH
/// "constructed" flag only.
pub unsafe extern "win64" fn inplace_string_substr(
    this: *const InplaceString,
    out: *mut InplaceString,
    pos: u64,
    count: u64,
) -> *mut InplaceString {
    construct_empty(out, 0x20);
    let v = StrView { ptr: storage(this), len: len_of(this) as u64 };
    assign_range(out, &v, pos, count);
    out
}

/// `r::InplaceString<>::reserve_if_needed(n)` (inferred name) — tail-calls
/// `reserve(this, n)` when `n >= capacity` (64-bit compare).
///
/// Original: 0x14240a940 (20 bytes).
pub unsafe extern "win64" fn inplace_string_reserve_if_needed(this: *mut InplaceString, n: u64) {
    if n >= capacity(this) as u64 {
        reserve(this, n);
    }
}

/// `r::InplaceString<>::resize(n)` — always calls `reserve(this, n)`, zero-
/// fills `[old_len, n)` when growing, NUL at `n`, `len = (u32)n`.
///
/// Original: 0x14240a960 (144 bytes).
pub unsafe extern "win64" fn inplace_string_resize(this: *mut InplaceString, n: u64) {
    let old = len_of(this) as u64;
    reserve(this, n);
    let d = storage(this);
    if n > old {
        exe::call_va!(J_MEMSET_VA, (d.add(old as usize), 0i32, n - old) -> *mut u8);
    }
    *d.add(n as usize) = 0;
    set_len(this, n as u32);
}

/// `r::mem::HeapResource` slot 2 (deallocate), 0x14020b500.
const HEAP_DEALLOCATE_VA: u64 = 0x1_4020_b500;
const COPY_ASSIGN_VA: u64 = 0x1_4240_aa00;

/// `r::InplaceString<>::operator=(InplaceString&&)` — move assignment;
/// returns `this` (no-op on self-assignment).
///
/// Original: 0x14240a730 (440 bytes). The heap buffer is stolen only if
/// `src.len >= inline_capacity(this)` *and* `src` is in heap mode. In that
/// case this's own heap buffer is freed first via `HeapResource`
/// deallocate (0x14020b500, `rcx` unused). `this` takes src's pointer,
/// capacity units and length (it keeps its own inline code), and `src`
/// becomes an empty inline string. Otherwise src's characters are copied
/// with `assign({data, len}, 0, len)` (0x142409b50).
pub unsafe extern "win64" fn inplace_string_move_assign(this: *mut InplaceString, src: *mut InplaceString) -> *mut InplaceString {
    if this == src {
        return this;
    }
    let slen = len_of(src);
    if slen >= inline_capacity(this) && capacity(src) > inline_capacity(src) {
        if capacity(this) > inline_capacity(this) {
            exe::call_va!(HEAP_DEALLOCATE_VA, (this as *mut u8, inplace_string_data(this)));
        }
        let (sb, tb) = (bits(src), bits(this));
        std::ptr::write_unaligned((this as *mut u8).add(8) as *mut *mut u8, storage(src));
        std::ptr::write_unaligned(this as *mut u32, ((tb ^ sb) & 0xff_ffff) ^ tb);
        set_len(this, len_of(src));
        let units = inline_capacity(src) >> 3;
        set_len(src, 0);
        let sb = bits(src);
        std::ptr::write_unaligned(src as *mut u32, sb ^ ((units ^ sb) & 0xff_ffff));
        *inplace_string_data(src) = 0;
        return this;
    }
    let v = StrView { ptr: storage(src), len: slen as u64 };
    assign_range(this, &v, 0, slen as u64);
    this
}

/// `r::InplaceString<>::operator+(view) const` — constructs `*out` as an
/// empty `InplaceString<256>`, copy-assigns `*this` (0x14240aa00), appends the
/// view (0x14240a630) and returns `out`.
///
/// Original: 0x14240a160 (104 bytes). The `[rsp+0x20]` writes are EH state.
pub unsafe extern "win64" fn inplace_string_concat(
    this: *const InplaceString,
    out: *mut InplaceString,
    v: *const StrView,
) -> *mut InplaceString {
    construct_empty(out, 0x20);
    exe::call_va!(COPY_ASSIGN_VA, (out, this) -> *mut InplaceString);
    let copy = StrView { ptr: (*v).ptr, len: (*v).len };
    exe::call_va!(APPEND_VIEW_VA, (out, &copy as *const StrView) -> *mut InplaceString);
    out
}

const MOVE_ASSIGN_VA: u64 = 0x1_4240_a730;
const DTOR_VA: u64 = 0x1_4240_ab10;

/// A stack `InplaceString<256>` (code 0x20, 256 inline bytes), as the
/// original builds it: header qword 0x20000020 (len 0) and a NUL.
#[repr(C, align(8))]
struct Local256([u8; 0x108]);

impl Local256 {
    fn new() -> Local256 {
        let mut b = [0u8; 0x108];
        b[..8].copy_from_slice(&0x2000_0020u64.to_le_bytes());
        Local256(b)
    }
    fn ptr(&mut self) -> *mut InplaceString {
        self.0.as_mut_ptr() as *mut InplaceString
    }
}

/// `r::InplaceString<>::replace(pos, count, view)` (name inferred from
/// behavior): `this = this[..pos] + view + this[pos+count..]`; returns `this`.
///
/// Original: 0x142409d70 (937 bytes). It builds the result in a stack
/// `InplaceString<256>`:
/// - if `pos != 0`: assign `this[0, pos)` (0x142409b50) to a second temp and
///   move-assign it in (0x14240a730);
/// - append the view (0x14240a630);
/// - if `pos + count < len` (64-bit, wrapping): append `this[pos+count..]`
///   from a third temp.
///
/// Then it copies the result back into `this`. That copy is an inlined
/// `assign({data, len}, 0, len)`, so it's called here as 0x142409b50. Every
/// temp is destroyed with 0x14240ab10.
pub unsafe extern "win64" fn inplace_string_replace(
    this: *mut InplaceString,
    pos: u64,
    count: u64,
    v: *const StrView,
) -> *mut InplaceString {
    let mut out = Local256::new();
    if pos != 0 {
        let mut head = Local256::new();
        let whole = StrView { ptr: storage(this), len: len_of(this) as u64 };
        assign_range(head.ptr(), &whole, 0, pos);
        exe::call_va!(MOVE_ASSIGN_VA, (out.ptr(), head.ptr()) -> *mut InplaceString);
        exe::call_va!(DTOR_VA, (head.ptr()));
    }
    let copy = StrView { ptr: (*v).ptr, len: (*v).len };
    exe::call_va!(APPEND_VIEW_VA, (out.ptr(), &copy as *const StrView) -> *mut InplaceString);
    let rest = pos.wrapping_add(count);
    if rest < len_of(this) as u64 {
        let mut tail = Local256::new();
        let whole = StrView { ptr: storage(this), len: len_of(this) as u64 };
        assign_range(tail.ptr(), &whole, rest, u64::MAX);
        let tv = StrView { ptr: storage(tail.ptr()), len: len_of(tail.ptr()) as u64 };
        exe::call_va!(APPEND_VIEW_VA, (out.ptr(), &tv as *const StrView) -> *mut InplaceString);
        exe::call_va!(DTOR_VA, (tail.ptr()));
    }
    let n = len_of(out.ptr()) as u64;
    let ov = StrView { ptr: storage(out.ptr()), len: n };
    assign_range(this, &ov, 0, n);
    exe::call_va!(DTOR_VA, (out.ptr()));
    this
}

/// `r::InplaceString<>::~InplaceString()` — frees a heap buffer.
///
/// Original: 0x14240ab10 (233 bytes, ~20k call sites). Inline strings (and a
/// null heap pointer) are left alone; otherwise the region allocator's
/// inlined small-block free (see [`super::mem`]; the same code as
/// `r::mem::HeapResource` slot 2, 0x14020b500).
pub unsafe extern "win64" fn inplace_string_dtor(this: *mut InplaceString) {
    if capacity(this) <= inline_capacity(this) {
        return;
    }
    let p = storage(this);
    if !p.is_null() {
        super::mem::region_free(p);
    }
}

/// CityHash64 (`0x1424b16a0`, `(ptr, len) -> u64`).
const CITYHASH64_VA: u64 = 0x1_424b_16a0;

/// `r::InplaceString<>::hash()` — CityHash64 of `(data(), len)`.
///
/// Original: 0x142409d20 (66 bytes).
pub unsafe extern "win64" fn inplace_string_hash(this: *const InplaceString) -> u64 {
    exe::call_va!(CITYHASH64_VA, (storage(this) as *const u8, len_of(this) as u64) -> u64)
}

/// String-view trim helper (`0x141378c80`, `(out, &mut view, &chars_view, trim_left, trim_right)`).
const STRVIEW_TRIM_VA: u64 = 0x1_4137_8c80;
/// `InplaceString<>::erase(this, pos, count)` (`0x142409430`).
const ERASE_VA: u64 = 0x1_4240_9430;
/// `InplaceString<>::trim(this, &chars_view, trim_left)` (`0x14240b010`).
const TRIM_VIEW_VA: u64 = 0x1_4240_b010;

/// `r::InplaceString<>::trim(view, trim_left)` (name inferred) — trims matching
/// characters from the right (always) and optionally the left (`trim_left != 0`)
/// using `0x141378c80` and [`inplace_string_erase`] (`0x142409430`); returns `this`.
///
/// Original: 0x14240b010 (298 bytes).
pub unsafe extern "win64" fn inplace_string_trim(
    this: *mut InplaceString,
    chars: *const StrView,
    trim_left: u8,
) -> *mut InplaceString {
    let mut src = StrView { ptr: storage(this), len: len_of(this) as u64 };
    let mut trimmed = StrView { ptr: std::ptr::null(), len: 0 };
    let chars_copy = StrView { ptr: (*chars).ptr, len: (*chars).len };
    exe::call_va!(
        STRVIEW_TRIM_VA,
        (
            &mut trimmed as *mut StrView,
            &mut src as *mut StrView,
            &chars_copy as *const StrView,
            trim_left,
            1u8
        ) -> *mut StrView
    );
    let d = storage(this) as usize;
    let t_ptr = trimmed.ptr as usize;
    let t_len = trimmed.len as usize;
    let right_pos = t_len.wrapping_sub(d).wrapping_add(t_ptr);
    if right_pos < len_of(this) as usize {
        exe::call_va!(ERASE_VA, (this, right_pos as u64, u64::MAX) -> *mut InplaceString);
    }
    let d2 = storage(this) as usize;
    let left_count = t_ptr.wrapping_sub(d2);
    if left_count != 0 {
        exe::call_va!(ERASE_VA, (this, 0u64, left_count as u64) -> *mut InplaceString);
    }
    this
}

/// `r::InplaceString<>::trim(cstr, trim_left)` — wraps `cstr` in a [`StrView`]
/// via `strlen` and calls `0x14240b010` ([`inplace_string_trim`]).
///
/// Original: 0x142409230 (57 bytes).
pub unsafe extern "win64" fn inplace_string_trim_cstr(
    this: *mut InplaceString,
    cstr: *const u8,
    trim_left: u8,
) -> *mut InplaceString {
    let mut n = 0usize;
    while exe::rd::<u8>(cstr as usize + n) != 0 {
        n += 1;
    }
    let v = StrView { ptr: cstr, len: n as u64 };
    exe::call_va!(TRIM_VIEW_VA, (this, &v as *const StrView, trim_left) -> *mut InplaceString)
}

/// `r::mem::StlAllocator<char>::allocate` (`0x140209f30`).
const STL_ALLOC1_VA: u64 = 0x1_4020_9f30;
/// IAT slot of `api-ms-win-crt-string-l1-1-0.dll!strncpy_s` (`0x1426469f0`).
const STRNCPY_S_IAT: u64 = 0x1_4264_69f0;

/// `r::InplaceString<>::reserve(n)` — adjust capacity for `n` characters
/// (`need = n.wrapping_add(1)` bytes including NUL).
///
/// Original: 0x142408f50 (723 bytes, 184 call sites).
/// - When `need <= inline_capacity(this)`: if currently in heap mode, copies
///   up to `icap - 1` characters back into the inline buffer with `strncpy_s`,
///   frees the old heap buffer with [`super::mem::region_free`], and resets the
///   capacity to `icap` (and if `icap <= len`, clamps `len = icap - 1` and
///   writes NUL).
/// - When `need > inline_capacity(this)` and `need > capacity(this)`: grows
///   capacity (at least 1.5x when `need > 0x100`, rounded up to a multiple of
///   16 and capped at `0x7ff_fff8`), allocates via `0x140209f30`, copies with
///   `strncpy_s`, frees any previous heap buffer via `0x14020b500`, updates
///   the capacity field, and clamps `len` if needed.
pub unsafe extern "win64" fn inplace_string_reserve(this: *mut InplaceString, n: u64) {
    let need = n.wrapping_add(1);
    let icap = inline_capacity(this) as u64;
    let new_cap_bytes: u32 = if need <= icap {
        let cap = capacity(this) as u64;
        if cap > icap {
            let old_p = storage(this);
            let inline_buf = (this as *mut u8).add(8);
            exe::call_iat!(
                STRNCPY_S_IAT,
                (
                    inline_buf,
                    icap as usize,
                    old_p as *const u8,
                    (icap as usize).wrapping_sub(1)
                ) -> i32
            );
            if !old_p.is_null() {
                super::mem::region_free(old_p);
            }
            let new_ic = inline_capacity(this);
            let units = new_ic >> 3;
            let b = bits(this);
            exe::wr::<u32>(this as usize, ((units ^ b) & 0xff_ffff) ^ b);
            new_ic
        } else {
            cap as u32
        }
    } else {
        let b0 = bits(this);
        let units0 = (b0 & 0xff_ffff) as u64;
        let cap0 = (units0 << 3) as u32 as u64;
        if need <= cap0 {
            return;
        }
        const MAX_CAP: u64 = 0x7ff_fff8;
        let mut target = need;
        if target > 0x100 {
            let grown = ((units0 as u32).wrapping_mul(4) as u64).wrapping_add(cap0).min(MAX_CAP);
            if grown > target {
                target = grown;
            }
        }
        let alloc_bytes = target.wrapping_add(0xf) & !0xfu64;
        let alloc_bytes = alloc_bytes.min(MAX_CAP);
        let new_p = exe::call_va!(STL_ALLOC1_VA, (this as *mut u8, alloc_bytes as usize) -> *mut u8);
        let old_p = storage(this);
        exe::call_iat!(
            STRNCPY_S_IAT,
            (
                new_p,
                alloc_bytes as usize,
                old_p as *const u8,
                (alloc_bytes as usize).wrapping_sub(1)
            ) -> i32
        );
        if capacity(this) > inline_capacity(this) {
            exe::call_va!(HEAP_DEALLOCATE_VA, (this as *mut u8, old_p));
        }
        exe::wr::<*mut u8>(this as usize + 8, new_p);
        let units = (alloc_bytes as u32) >> 3;
        let b = bits(this);
        let new_b = ((units ^ b) & 0xff_ffff) ^ b;
        exe::wr::<u32>(this as usize, new_b);
        (new_b & 0xff_ffff) << 3
    };
    if new_cap_bytes <= len_of(this) {
        let clamped = new_cap_bytes.wrapping_sub(1);
        set_len(this, clamped);
        let d = storage(this);
        exe::wr::<u8>(d as usize + clamped as usize, 0);
    }
}

/// `r::InplaceString<>::appendFormatV(this, fmt, va_list)` (`0x14240a1d0`).
const APPEND_FORMAT_V_VA: u64 = 0x1_4240_a1d0;

/// `r::InplaceString<>::appendFormat(this, fmt, ...)` — spills varargs and
/// calls `0x14240a1d0` (`appendFormatV`).
///
/// Original: 0x14240a460 (34 bytes).
pub unsafe extern "win64" fn inplace_string_append_format(
    this: *mut InplaceString,
    fmt: *const u8,
    a0: u64,
    a1: u64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        let mut rbp: usize;
        std::arch::asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
        exe::wr::<u64>(rbp + 0x20, a0);
        exe::wr::<u64>(rbp + 0x28, a1);
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, (rbp + 0x20) as *const u64));
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (mut a2, mut a3, mut a4, mut a5) = (0u64, 0u64, 0u64, 0u64);
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!(
            "mov {a2}, x4",
            "mov {a3}, x5",
            "mov {a4}, x6",
            "mov {a5}, x7",
            a2 = out(reg) a2,
            a3 = out(reg) a3,
            a4 = out(reg) a4,
            a5 = out(reg) a5,
            options(nomem, nostack, preserves_flags),
        );
        let va = [a0, a1, a2, a3, a4, a5];
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, va.as_ptr()));
    }
}

/// `r::InplaceString<>::format(this, fmt, ...)` — clears `this` (`len = 0`,
/// `*data() = 0`), then calls `0x14240a1d0` (`appendFormatV`) with varargs.
///
/// Original: 0x142409620 (104 bytes).
pub unsafe extern "win64" fn inplace_string_format(
    this: *mut InplaceString,
    fmt: *const u8,
    a0: u64,
    a1: u64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        let mut rbp: usize;
        std::arch::asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
        exe::wr::<u64>(rbp + 0x20, a0);
        exe::wr::<u64>(rbp + 0x28, a1);
        set_len(this, 0);
        *storage(this) = 0;
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, (rbp + 0x20) as *const u64));
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (mut a2, mut a3, mut a4, mut a5) = (0u64, 0u64, 0u64, 0u64);
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!(
            "mov {a2}, x4",
            "mov {a3}, x5",
            "mov {a4}, x6",
            "mov {a5}, x7",
            a2 = out(reg) a2,
            a3 = out(reg) a3,
            a4 = out(reg) a4,
            a5 = out(reg) a5,
            options(nomem, nostack, preserves_flags),
        );
        let va = [a0, a1, a2, a3, a4, a5];
        set_len(this, 0);
        *storage(this) = 0;
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, va.as_ptr()));
    }
}

/// `r::InplaceString<N>::InplaceString(tag, fmt, ...)` — constructs an empty
/// `InplaceString<N>` (`UNITS = N / 8`), formats `fmt` with varargs starting
/// at `r9` via `0x14240a1d0`, and returns `this`.
///
/// Originals: 0x1401f2430 (`UNITS = 0x20`, 1,977 callers), 0x1405fc3c0 (`0x10`,
/// 60 callers), 0x1412dfd70 (`0x08`), 0x1405594c0 (`0x04`), 0x1405fc370 (`0x02`).
pub unsafe extern "win64" fn inplace_string_format_ctor<const UNITS: u8>(
    this: *mut InplaceString,
    _tag: u64,
    fmt: *const u8,
    a0: u64,
) -> *mut InplaceString {
    #[cfg(target_arch = "x86_64")]
    {
        let mut rbp: usize;
        std::arch::asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
        exe::wr::<u64>(rbp + 0x28, a0);
        construct_empty(this, UNITS);
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, (rbp + 0x28) as *const u64));
        this
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (mut a1, mut a2, mut a3, mut a4) = (0u64, 0u64, 0u64, 0u64);
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!(
            "mov {a1}, x4",
            "mov {a2}, x5",
            "mov {a3}, x6",
            "mov {a4}, x7",
            a1 = out(reg) a1,
            a2 = out(reg) a2,
            a3 = out(reg) a3,
            a4 = out(reg) a4,
            options(nomem, nostack, preserves_flags),
        );
        let va = [a0, a1, a2, a3, a4];
        construct_empty(this, UNITS);
        exe::call_va!(APPEND_FORMAT_V_VA, (this, fmt, va.as_ptr()));
        this
    }
}

/// Uninitialized-move a range `[first, last)` of `InplaceString<N>` (`UNITS = N / 8`,
/// `STRIDE = N + 8`) into `dst` by constructing each destination element empty
/// and calling `inplace_string_move_assign` (`0x14240a730`). Returns `dst` past
/// the last moved element.
///
/// Originals: 0x1401f2480 (`UNITS = 0x10`, `STRIDE = 0x88`) and 0x1401ece50
/// (`UNITS = 0x20`, `STRIDE = 0x108`).
pub unsafe extern "win64" fn inplace_string_uninit_move<const UNITS: u8, const STRIDE: usize>(
    mut first: *mut u8,
    last: *mut u8,
    mut dst: *mut u8,
) -> *mut u8 {
    while first != last {
        construct_empty(dst as *mut InplaceString, UNITS);
        exe::call_va!(
            MOVE_ASSIGN_VA,
            (dst as *mut InplaceString, first as *mut InplaceString) -> *mut InplaceString
        );
        dst = dst.add(STRIDE);
        first = first.add(STRIDE);
    }
    dst
}

/// Iterator-pointer wrapper of [`inplace_string_uninit_move`] (`*first_ptr`, `*last_ptr`).
///
/// Original: 0x140220bf0 (133 bytes).
pub unsafe extern "win64" fn inplace_string128_uninit_move_iter(
    first_ptr: *const *mut u8,
    last_ptr: *const *mut u8,
    dst: *mut u8,
) -> *mut u8 {
    let first = exe::rd::<*mut u8>(first_ptr as usize);
    let last = exe::rd::<*mut u8>(last_ptr as usize);
    inplace_string_uninit_move::<0x10, 0x88>(first, last, dst)
}

/// Uninitialized-move of `{ u32 tag, InplaceString<64> }` (`STRIDE = 0x4c`)
/// from `*first_ptr .. *last_ptr` into `dst`.
///
/// Original: 0x140304fb0 (133 bytes).
pub unsafe extern "win64" fn tagged_inplace_string64_uninit_move_iter(
    first_ptr: *const *mut u8,
    last_ptr: *const *mut u8,
    mut dst: *mut u8,
) -> *mut u8 {
    let mut first = exe::rd::<*mut u8>(first_ptr as usize);
    let last = exe::rd::<*mut u8>(last_ptr as usize);
    while first != last {
        exe::wr::<u32>(dst as usize, exe::rd::<u32>(first as usize));
        let d_str = dst.add(4) as *mut InplaceString;
        construct_empty(d_str, 0x08);
        exe::call_va!(
            MOVE_ASSIGN_VA,
            (d_str, first.add(4) as *mut InplaceString) -> *mut InplaceString
        );
        dst = dst.add(0x4c);
        first = first.add(0x4c);
    }
    dst
}

/// Destroy all `len` elements of an `r::Vector<InplaceString<N>>` (`STRIDE = N + 8`),
/// region-free its buffer (if non-null), and zero the first two qwords (`ptr`, `len`/`cap`).
///
/// Originals: 0x1401ea4e0 (`STRIDE = 0x88`) and 0x1401ea410 (`STRIDE = 0x108`).
pub unsafe extern "win64" fn inplace_string_vec_destroy<const STRIDE: usize>(vec: *mut u8) {
    let v = vec as usize;
    let mut cur = exe::rd::<usize>(v);
    let len = exe::rd::<u32>(v + 8) as usize;
    let end = cur.wrapping_add(len.wrapping_mul(STRIDE));
    while cur != end {
        exe::call_va!(DTOR_VA, (cur as *mut InplaceString));
        cur = cur.wrapping_add(STRIDE);
    }
    let p = exe::rd::<usize>(v) as *mut u8;
    if !p.is_null() {
        super::mem::region_free(p);
    }
    exe::wr::<usize>(v, 0);
    exe::wr::<usize>(v + 8, 0);
}

/// Reallocate an `r::Vector<InplaceString<N>>` to `new_cap` elements: allocate
/// via `ALLOC_VA`, move-construct `len` elements via `MOVE_VA`, destroy/free
/// the old buffer via `DESTROY_VA`, and store `(new_buf, saved_len, new_cap as u32)`.
///
/// Originals: 0x1401f22d0 (100 bytes, 3,624 callers, `InplaceString<128>`) and
/// 0x1401ec9d0 (100 bytes, 807 callers, `InplaceString<256>`).
pub unsafe extern "win64" fn inplace_string_vec_realloc<
    const STRIDE: usize,
    const ALLOC_VA: u64,
    const MOVE_VA: u64,
    const DESTROY_VA: u64,
>(
    vec: *mut u8,
    new_cap: usize,
) {
    let v = vec as usize;
    let saved_len = exe::rd::<u32>(v + 8);
    let new_buf = exe::call_va!(ALLOC_VA, (vec, new_cap) -> *mut u8);
    let cur_len = exe::rd::<u32>(v + 8) as usize;
    let old_buf = exe::rd::<*mut u8>(v);
    let old_end = (old_buf as usize).wrapping_add(cur_len.wrapping_mul(STRIDE)) as *mut u8;
    exe::call_va!(MOVE_VA, (old_buf, old_end, new_buf) -> *mut u8);
    exe::call_va!(DESTROY_VA, (vec));
    exe::wr::<*mut u8>(v, new_buf);
    exe::wr::<u32>(v + 0x0c, new_cap as u32);
    exe::wr::<u32>(v + 0x08, saved_len);
}


pub mod difftests;
pub use difftests::*;
