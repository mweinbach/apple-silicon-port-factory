//! MSVC STL instantiations linked into the exe (third-party).
//!
//! AGENTS.md §6 says to leave CRT/STL code alone. The owner overrode that for
//! the hot `r::CharTraits` string-search helpers: on arm64 every call to them
//! from native code goes through the x86 interpreter, and the loading thread
//! spends about 20% of its samples in these two functions alone
//! (`out/hitch/emu-prof-*.log`). The algorithms are the upstream MSVC STL ones
//! (`<xstring>` `_Traits_find_first_not_of` / `_Traits_find_last_of`,
//! Apache-2.0). Only the comparison is game-specific: `r::CharTraits` compares
//! through the fold table at 0x1426d01e0, the same table the `*_nocase`
//! compares use.
//!
//! Both functions are leaves with no side effects. Native callers call the Rust
//! versions directly, and guest callers reach them through the VA registry.

use crate::exe::{self, rd};
use crate::ns::r::inplace_string::StrView;

/// `r::CharTraits<char>` fold table (to-upper; 256 bytes).
pub const FOLD_VA: u64 = 0x1_426d_01e0;

/// Pointer to the fold table in the mapped exe.
#[inline(always)]
pub fn fold_table() -> usize {
    exe::ptr::<u8>(FOLD_VA) as usize
}

/// `fold[c]` (equality only, so signedness doesn't matter).
#[inline(always)]
pub unsafe fn fold(t: usize, c: u8) -> u8 {
    rd::<u8>(t + c as usize)
}

/// Bitmap of the folded values of `set[..n]`, for O(1) membership tests.
/// Equivalent to the upstream inner loop `for c in set: if eq(c, ch)`, because
/// that loop only tests fold equality and has no side effects.
#[inline(always)]
unsafe fn fold_set(t: usize, set: usize, n: u64) -> [u64; 4] {
    let mut m = [0u64; 4];
    for i in 0..n as usize {
        let f = fold(t, rd::<u8>(set + i));
        m[(f >> 6) as usize] |= 1u64 << (f & 63);
    }
    m
}

#[inline(always)]
fn in_set(m: &[u64; 4], f: u8) -> bool {
    m[(f >> 6) as usize] & (1u64 << (f & 63)) != 0
}

/// Native `find_first_not_of` on a raw view (same semantics as
/// [`traits_find_first_not_of`]; for direct use by other ports).
#[inline]
pub unsafe fn find_first_not_of_raw(hay: u64, hay_len: u64, set: u64, set_len: u64, pos: u64) -> u64 {
    if pos >= hay_len {
        return u64::MAX;
    }
    let end = hay.wrapping_add(hay_len);
    let mut p = hay.wrapping_add(pos);
    if p >= end {
        return u64::MAX;
    }
    let t = fold_table();
    let m = fold_set(t, set as usize, set_len);
    while p < end {
        if !in_set(&m, fold(t, rd::<u8>(p as usize))) {
            return p - hay;
        }
        p += 1;
    }
    u64::MAX
}

/// Native `find_first_of` on a raw view (the inlined form callers use after
/// `find_first_not_of`): first index `>= pos` whose fold is in the set, or npos.
#[inline]
pub unsafe fn find_first_of_raw(hay: u64, hay_len: u64, set: u64, set_len: u64, pos: u64) -> u64 {
    if pos >= hay_len {
        return u64::MAX;
    }
    let t = fold_table();
    let m = fold_set(t, set as usize, set_len);
    let mut p = hay + pos;
    let end = hay + hay_len;
    while p < end {
        if in_set(&m, fold(t, rd::<u8>(p as usize))) {
            return p - hay;
        }
        p += 1;
    }
    u64::MAX
}

/// `std::_Traits_find_first_not_of<r::CharTraits<char>>` as instantiated at
/// 0x1401e1a40 (166 bytes; fndb labels it `_Traits_find_first_of`, but the code is
/// `find_first_not_of`): `(view*, set_view*, pos) -> index | npos`.
///
/// From the disassembly: npos if `pos >= view.len` or `view.ptr + pos >= view.ptr + view.len`.
/// Otherwise it returns the first index `>= pos` whose folded char matches no
/// folded char of the set. An empty set returns `pos`. The set's ptr/len are
/// read once.
pub unsafe extern "win64" fn traits_find_first_not_of(hay: *const StrView, set: *const StrView, pos: u64) -> u64 {
    let (hp, hl) = (rd::<u64>(hay as usize), rd::<u64>(hay as usize + 8));
    if pos >= hl {
        return u64::MAX;
    }
    let (sp, sl) = (rd::<u64>(set as usize), rd::<u64>(set as usize + 8));
    find_first_not_of_raw(hp, hl, sp, sl, pos)
}

/// Native `find_last_of` on raw pointers (same semantics as [`traits_find_last_of`]).
#[inline]
pub unsafe fn find_last_of_raw(hay: u64, hay_len: u64, set: u64, set_len: u64) -> u64 {
    if set_len == 0 || hay_len == 0 {
        return u64::MAX;
    }
    let t = fold_table();
    let m = fold_set(t, set as usize, set_len);
    let mut p = hay + (hay_len - 1);
    loop {
        if in_set(&m, fold(t, rd::<u8>(p as usize))) {
            return p - hay;
        }
        if p == hay {
            return u64::MAX;
        }
        p -= 1;
    }
}

/// `std::_Traits_find_last_of<r::CharTraits<char>>` as instantiated at 0x1401e22e0
/// (145 bytes): `(hay, hay_len, start_at, set, set_len) -> index | npos`, with
/// `set_len` as the 5th (stack) argument.
///
/// This instantiation never reads `start_at` (r8): the scan always starts at
/// `hay_len - 1`, as if called with npos. Callers such as 0x1401dfa80 leave r8
/// as garbage. npos if `set_len == 0` or `hay_len == 0`.
pub unsafe extern "win64" fn traits_find_last_of(hay: *const u8, hay_len: u64, _start_at: u64, set: *const u8, set_len: u64) -> u64 {
    find_last_of_raw(hay as u64, hay_len, set as u64, set_len)
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::difftest::Rng;

/// Alphabet mixing case pairs (fold-equal), separators and high bytes.
const ALPHA: &[u8] = b"aAbB/\\.:_z\xe4\xc4\xff\x00";

fn rand_str(r: &mut Rng, max: u64) -> Vec<u8> {
    let n = r.below(max + 1) as usize;
    (0..n).map(|_| ALPHA[r.below(ALPHA.len() as u64) as usize]).collect()
}

pub unsafe fn diff_traits_find_first_not_of(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const StrView, *const StrView, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e1a40);
    const CASES: u32 = 4000;
    for i in 0..CASES {
        let hay = rand_str(&mut r, 24);
        let set = rand_str(&mut r, 4);
        let pos = match r.below(8) {
            0 => u64::MAX,
            1 => hay.len() as u64,
            _ => r.below(hay.len() as u64 + 2),
        };
        // Views at a 4-byte offset every other case (under-aligned headers).
        let off = (i % 2) as usize * 4;
        let mut buf = vec![0u8; 40];
        let hv = buf.as_mut_ptr().add(off);
        exe::wr::<u64>(hv as usize, hay.as_ptr() as u64);
        exe::wr::<u64>(hv as usize + 8, hay.len() as u64);
        let sv = hv.add(16);
        exe::wr::<u64>(sv as usize, set.as_ptr() as u64);
        exe::wr::<u64>(sv as usize + 8, set.len() as u64);
        let a = o(hv as *const StrView, sv as *const StrView, pos);
        let b = n(hv as *const StrView, sv as *const StrView, pos);
        if a != b {
            return Err(format!("case {i}: hay {:?} set {:?} pos {pos}: orig {a:#x} new {b:#x}", hay, set));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_traits_find_last_of(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u64, u64, *const u8, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e22e0);
    const CASES: u32 = 4000;
    for i in 0..CASES {
        let hay = rand_str(&mut r, 24);
        let set = rand_str(&mut r, 4);
        // start_at is ignored by this instantiation; vary it to prove that.
        let start = r.next_u64();
        let a = o(hay.as_ptr(), hay.len() as u64, start, set.as_ptr(), set.len() as u64);
        let b = n(hay.as_ptr(), hay.len() as u64, start, set.as_ptr(), set.len() as u64);
        if a != b {
            return Err(format!("case {i}: hay {:?} set {:?}: orig {a:#x} new {b:#x}", hay, set));
        }
    }
    Ok(CASES)
}

// ---------------------------------------------------------------------------
// std::exception destructor reachable only from EH metadata
// ---------------------------------------------------------------------------

/// `std::exception` vftable (`??_7exception@std@@6B@`; RTTI COL -> `.?AVexception@std@@`).
const EXCEPTION_VFTABLE: u64 = 0x1_4266_80a0;
/// `j___std_exception_destroy` (VCRUNTIME140 import thunk, `jmp [0x142646330]`).
const J_STD_EXCEPTION_DESTROY: u64 = 0x1_4183_ac8c;
/// IAT slot of `VCRUNTIME140!__std_exception_destroy`.
const STD_EXCEPTION_DESTROY_IAT: u64 = 0x1_4264_6330;

/// `std::exception::~exception` body at 0x141498b6c (MSVC STL `<vcruntime_exception.h>`):
/// ```text
/// lea rax, [std::exception::`vftable']   ; 0x1426680a0
/// mov [rcx], rax
/// add rcx, 8                              ; &this->_Data
/// jmp j___std_exception_destroy           ; 0x14183ac8c
/// ```
/// It has no `.pdata` (leaf tail jump) and no code references, so fndb/Ghidra never listed
/// it and it got no shard. It's only referenced as `ThrowInfo::pmfnUnwind` of
/// 0x1437fce18 (`std::future_error`) and 0x1437fce80 (`Concurrency::invalid_operation`), and
/// it runs when the CRT destroys such a thrown object. Seen as the one guest-x86 exe block
/// in the gameplay gate: msvcp140 `__ExceptionPtrDestroy` (entered through the exe thunk
/// 0x1418348af) calling it for an `exception_ptr` that holds a `future_error`. Of the 70
/// ThrowInfos and their CatchableTypes in the exe, this is the only unwind/copy-ctor target
/// that has no replacement.
pub unsafe extern "win64" fn exception_dtor_141498b6c(this: u64) {
    exe::wr::<u64>(this as usize, exe::ptr::<u8>(EXCEPTION_VFTABLE) as u64);
    exe::func::<unsafe extern "win64" fn(u64)>(J_STD_EXCEPTION_DESTROY)(this.wrapping_add(8));
}

std::thread_local! {
    /// `__std_exception_destroy` arguments seen by the difftest mock.
    static DESTROY_CALLS: std::cell::RefCell<Vec<u64>> = const { std::cell::RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn mock_std_exception_destroy(data: u64) {
    DESTROY_CALLS.with(|c| c.borrow_mut().push(data));
}

/// Same object image (vftable store, the rest untouched) and the same single
/// `__std_exception_destroy(this + 8)` call, at 8-aligned and under-aligned `this`.
pub unsafe fn diff_exception_dtor_141498b6c(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _iat = crate::difftest::IatMock::new(STD_EXCEPTION_DESTROY_IAT, mock_std_exception_destroy as *const ())?;
    let mut r = Rng::new(0x141498b6c);
    let mut buf = vec![0u8; 64];
    const CASES: u32 = 64;
    for i in 0..CASES {
        let skew = [0usize, 4, 1, 8][(i % 4) as usize];
        let init = r.bytes(buf.len());
        let this = buf.as_mut_ptr() as u64 + skew as u64;
        let mut run = |f: F| {
            buf.copy_from_slice(&init);
            DESTROY_CALLS.with(|c| c.borrow_mut().clear());
            f(this);
            (buf.clone(), DESTROY_CALLS.with(|c| c.borrow().clone()))
        };
        let a = run(o);
        let b = run(n);
        if a != b {
            return Err(format!("case {i} (skew {skew}): orig {:x?} calls {:x?}, new {:x?} calls {:x?}", &a.0[..24], a.1, &b.0[..24], b.1));
        }
        if a.1 != [this + 8] {
            return Err(format!("case {i}: expected one destroy({:#x}), got {:x?}", this + 8, a.1));
        }
    }
    Ok(CASES)
}
