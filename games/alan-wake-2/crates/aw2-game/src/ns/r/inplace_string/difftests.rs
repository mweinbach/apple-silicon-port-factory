//! Differential tests for `r::InplaceString`.

use super::*;

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::Rng;

/// A test string object: its own buffer (offset 0 or 4 to cover 4-aligned
/// embedded objects) plus an optional heap buffer.
struct TestStr {
    obj: Vec<u8>,
    off: usize,
    heap: Option<Vec<u8>>,
}

impl TestStr {
    /// Deterministic from `seed`; building twice yields identical, independent objects.
    fn gen(seed: u64) -> TestStr {
        let mut r = Rng::new(seed);
        const CODES: [u8; 10] = [0, 1, 2, 4, 0x10, 0x20, 0x7f, 0x80, 0x81, 0x82];
        let code = CODES[r.below(CODES.len() as u64) as usize];
        let icap = if code < 0x80 { code as u32 * 8 } else { ((code as u32) << 8) - 0x7c00 };
        let heap = icap == 0 || r.below(3) == 0;
        let cap = if heap {
            // capacity units with cap*8 > icap
            (icap / 8 + 1 + r.below(8) as u32) * 8
        } else {
            8 * r.below(icap as u64 / 8 + 1) as u32
        };
        let store = if heap { cap as usize } else { icap as usize };
        let len = if store == 0 { 0 } else { r.below(store.min(200) as u64) as usize };
        let off = if r.below(2) == 0 { 0 } else { 4 };
        let mut obj = r.bytes(off + 8 + (icap as usize).max(8) + 8);
        let bits = ((code as u32) << 24) | (cap / 8);
        obj[off..off + 4].copy_from_slice(&bits.to_le_bytes());
        obj[off + 4..off + 8].copy_from_slice(&(len as u32).to_le_bytes());
        // Mixed-case alphabet so case folding matters, plus some high bytes.
        const ALPHA: &[u8] = b"aAbBcCxXzZ09_/.\x80\xc4\xe4";
        let mut text: Vec<u8> = (0..len).map(|_| ALPHA[r.below(ALPHA.len() as u64) as usize]).collect();
        text.push(0);
        let mut heap_buf = None;
        if heap {
            let mut h = r.bytes(store.max(len + 1));
            h[..len + 1].copy_from_slice(&text);
            heap_buf = Some(h);
        } else if store > 0 {
            obj[off + 8..off + 8 + len + 1].copy_from_slice(&text);
        }
        let mut t = TestStr { obj, off, heap: heap_buf };
        if let Some(h) = t.heap.as_mut() {
            let p = h.as_mut_ptr() as u64;
            t.obj[off + 8..off + 16].copy_from_slice(&p.to_le_bytes());
        }
        t
    }
    /// Bytes available at the character storage (inline area or heap buffer).
    fn storage_len(&self) -> usize {
        match &self.heap {
            Some(h) => h.len(),
            None => self.obj.len() - self.off - 16,
        }
    }
    fn ptr(&mut self) -> *mut InplaceString {
        unsafe { self.obj.as_mut_ptr().add(self.off) as *mut InplaceString }
    }
    fn text(&self) -> Vec<u8> {
        let len = u32::from_le_bytes(self.obj[self.off + 4..self.off + 8].try_into().unwrap()) as usize;
        match &self.heap {
            Some(h) => h[..len.min(h.len())].to_vec(),
            None => self.obj[self.off + 8..(self.off + 8 + len).min(self.obj.len())].to_vec(),
        }
    }
    /// Memory state with the heap pointer field neutralized.
    fn state(&self) -> (Vec<u8>, Option<Vec<u8>>) {
        let mut o = self.obj.clone();
        if self.heap.is_some() {
            o[self.off + 8..self.off + 16].fill(0);
        }
        (o, self.heap.clone())
    }
    /// Classify a returned pointer: (0 = object, 1 = heap, 2 = other) + offset.
    fn rel(&self, p: usize) -> (u8, isize) {
        // Half-open ranges: the two allocations may be adjacent.
        if let Some(h) = &self.heap {
            let hb = h.as_ptr() as usize;
            if p >= hb && p < hb + h.len() {
                return (1, (p - hb) as isize);
            }
        }
        let ob = self.obj.as_ptr() as usize;
        if p >= ob && p < ob + self.obj.len() {
            return (0, (p - ob) as isize);
        }
        (2, p as isize)
    }
}

const STR_CASES: u64 = 400;

type CtorFn = unsafe extern "win64" fn(*mut InplaceString) -> *mut InplaceString;

unsafe fn diff_ctor(orig: usize, new: usize) -> Result<u32, String> {
    let (o, n): (CtorFn, CtorFn) = (std::mem::transmute(orig), std::mem::transmute(new));
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let (ra, rb) = (o(a.ptr()) as usize, n(b.ptr()) as usize);
        diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string128_ctor(orig: usize, new: usize) -> Result<u32, String> {
    diff_ctor(orig, new)
}

pub unsafe fn diff_inplace_string32_ctor(orig: usize, new: usize) -> Result<u32, String> {
    diff_ctor(orig, new)
}

pub unsafe fn diff_inplace_string_data(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let (ra, rb) = (o(a.ptr()) as usize, n(b.ptr()) as usize);
        diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string_is_inline(orig: usize, new: usize) -> Result<u32, String> {
    // Only `al` is defined for a bool return: compare the low byte.
    type F = unsafe extern "win64" fn(*const InplaceString) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        diff_eq!(o(a.ptr()) != 0, n(b.ptr()) != 0, "seed {seed}");
        cases += 1;
    }
    // Header-only sweep over every inline code and a range of capacities.
    for code in 0..=255u32 {
        for units in [0u32, 1, 2, 3, 4, 15, 16, 17, 127, 128, 129, 0x1000, 0xff_ffff] {
            let mut h = [0u8; 16];
            h[..4].copy_from_slice(&((code << 24) | units).to_le_bytes());
            let (x, y) = (o(h.as_ptr() as *const InplaceString), n(h.as_ptr() as *const InplaceString));
            diff_eq!(x != 0, y != 0, "code {code:#x} units {units:#x}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_erase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u64, u64) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0xe2a5e);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let len = TestStr::gen(seed).text().len() as u64;
        for _ in 0..4 {
            let pos = match r.below(5) {
                0 => len,
                1 => len + 1 + r.below(3),
                _ => r.below(len + 1),
            };
            let count = match r.below(5) {
                0 => u64::MAX,
                1 => 0,
                _ => r.below(len + 2),
            };
            let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
            let (ra, rb) = (o(a.ptr(), pos, count) as usize, n(b.ptr(), pos, count) as usize);
            diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed} pos {pos} count {count}");
            diff_eq!(a.state(), b.state(), "memory, seed {seed} pos {pos} count {count}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_rfind_char_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, u8) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x5f1d);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let text = a.text();
        for _ in 0..6 {
            let c = if !text.is_empty() && r.below(2) == 0 {
                let c = text[r.below(text.len() as u64) as usize];
                if r.below(2) == 0 { c.to_ascii_uppercase() } else { c }
            } else {
                r.next_u32() as u8
            };
            diff_eq!(o(a.ptr(), c), n(b.ptr(), c), "seed {seed} char {c:#x} text {:?}", String::from_utf8_lossy(&text));
            cases += 1;
        }
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_find_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *const StrView, u64) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0xf1fd);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let text = a.text();
        let len = text.len() as u64;
        for _ in 0..8 {
            // Needle: a (case-flipped) substring, a random string, or empty.
            let mut needle: Vec<u8> = match r.below(4) {
                0 if len > 0 => {
                    let s = r.below(len) as usize;
                    let e = s + 1 + r.below(len - s as u64) as usize;
                    text[s..e].iter().map(|c| if r.below(2) == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }).collect()
                }
                1 => Vec::new(),
                _ => (0..1 + r.below(4)).map(|_| b"abAB0z"[r.below(6) as usize]).collect(),
            };
            needle.push(0xEE); // guard byte past the view
            let view = StrView { ptr: needle.as_ptr(), len: needle.len() as u64 - 1 };
            let pos = match r.below(6) {
                0 => len,
                1 => u64::MAX,
                2 => (1u64 << 32) + r.below(4),
                _ => r.below(len + 1),
            };
            diff_eq!(o(a.ptr(), &view, pos), n(b.ptr(), &view, pos), "seed {seed} pos {pos} needle {:?} text {:?}", String::from_utf8_lossy(&needle), String::from_utf8_lossy(&text));
            cases += 1;
        }
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_clear(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        o(a.ptr());
        n(b.ptr());
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

/// Run `body` for `per_seed` inputs on each generated string pair; memory of
/// both objects is compared after every seed.
unsafe fn drive(
    salt: u64,
    per_seed: u32,
    mut body: impl FnMut(&mut Rng, &mut TestStr, &mut TestStr, &[u8]) -> Result<(), String>,
) -> Result<u32, String> {
    let mut r = Rng::new(salt);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let text = a.text();
        for _ in 0..per_seed {
            body(&mut r, &mut a, &mut b, &text).map_err(|e| format!("seed {seed} text {:?}: {e}", String::from_utf8_lossy(&text)))?;
            cases += 1;
        }
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(cases)
}

/// Needle bytes for view-taking functions (a case-flipped substring, a
/// prefix/suffix, random short text, or empty) plus a trailing guard byte
/// that is not part of the view.
fn gen_needle(r: &mut Rng, text: &[u8]) -> Vec<u8> {
    let len = text.len() as u64;
    let flip = |r: &mut Rng, s: &[u8]| -> Vec<u8> {
        s.iter().map(|c| if r.below(2) == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }).collect()
    };
    let mut v: Vec<u8> = match r.below(7) {
        0 | 1 if len > 0 => {
            let s = r.below(len) as usize;
            let e = s + 1 + r.below(len - s as u64) as usize;
            let raw = text[s..e].to_vec();
            if r.below(2) == 0 { raw } else { flip(r, &raw) }
        }
        2 if len > 0 => {
            let n = 1 + r.below(len) as usize;
            flip(r, &text[..n])
        }
        3 if len > 0 => {
            let n = 1 + r.below(len) as usize;
            flip(r, &text[text.len() - n..])
        }
        4 => Vec::new(),
        5 => {
            let mut t = text.to_vec();
            t.push(b'a');
            t
        }
        _ => (0..1 + r.below(4)).map(|_| b"abAB0z\xc4"[r.below(7) as usize]).collect(),
    };
    v.push(0xEE);
    v
}

fn view_of(needle: &[u8]) -> StrView {
    StrView { ptr: needle.as_ptr(), len: needle.len() as u64 - 1 }
}

fn gen_char(r: &mut Rng, text: &[u8]) -> u8 {
    if !text.is_empty() && r.below(2) == 0 {
        let c = text[r.below(text.len() as u64) as usize];
        if r.below(2) == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }
    } else {
        r.next_u32() as u8
    }
}

pub unsafe fn diff_inplace_string_find_char_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, u8) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive(1, 6, |r, a, b, text| {
        let c = gen_char(r, text);
        diff_eq!(o(a.ptr(), c), n(b.ptr(), c), "char {c:#x}");
        Ok(())
    })
}

pub unsafe fn diff_inplace_string_find_char_nocase_from(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, u8, u64) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive(2, 6, |r, a, b, text| {
        let c = gen_char(r, text);
        let len = text.len() as u64;
        let pos = match r.below(5) {
            0 => len,
            1 => u64::MAX,
            _ => r.below(len + 1),
        };
        diff_eq!(o(a.ptr(), c, pos), n(b.ptr(), c, pos), "char {c:#x} pos {pos}");
        Ok(())
    })
}

/// Mutating `fn(this, args) -> *mut InplaceString`: compare return (relative)
/// and memory after every call.
unsafe fn drive_mut(salt: u64, per_seed: u32, mut call: impl FnMut(&mut Rng, &[u8], *mut InplaceString, bool) -> usize) -> Result<u32, String> {
    drive(salt, per_seed, |r, a, b, text| {
        let saved = r.0;
        let ra = call(r, text, a.ptr(), true);
        r.0 = saved; // same arguments for the replacement
        let rb = call(r, text, b.ptr(), false);
        diff_eq!(a.rel(ra), b.rel(rb), "return");
        diff_eq!(a.state(), b.state(), "memory");
        Ok(())
    })
}

pub unsafe fn diff_inplace_string_assign_char(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    // Needs room for 2 bytes: skip zero-capacity objects.
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        if a.storage_len() < 2 {
            continue;
        }
        let c = (seed * 37) as u8;
        let (ra, rb) = (o(a.ptr(), c) as usize, n(b.ptr(), c) as usize);
        diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_replace_char(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u8, u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_mut(3, 3, |r, text, p, first| {
        let from = gen_char(r, text);
        let to = r.next_u32() as u8;
        (if first { o } else { n })(p, from, to) as usize
    })
}

pub unsafe fn diff_inplace_string_to_lower(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_mut(4, 1, |_, _, p, first| (if first { o } else { n })(p) as usize)
}

pub unsafe fn diff_inplace_string_to_upper(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_mut(5, 1, |_, _, p, first| (if first { o } else { n })(p) as usize)
}

pub unsafe fn diff_inplace_string_find(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *const StrView, u64) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive(6, 8, |r, a, b, text| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        let len = text.len() as u64;
        let pos = match r.below(6) {
            0 => len,
            1 => u64::MAX,
            2 => (1u64 << 32) + r.below(4),
            _ => r.below(len + 1),
        };
        diff_eq!(o(a.ptr(), &v, pos), n(b.ptr(), &v, pos), "pos {pos} needle {:?}", String::from_utf8_lossy(&needle));
        Ok(())
    })
}

/// `fn(this, &view) -> R` comparisons (R read at its declared width).
unsafe fn drive_view<R: PartialEq + std::fmt::Debug>(salt: u64, o: unsafe extern "win64" fn(*const InplaceString, *const StrView) -> R, n: unsafe extern "win64" fn(*const InplaceString, *const StrView) -> R) -> Result<u32, String> {
    drive(salt, 8, |r, a, b, text| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        diff_eq!(o(a.ptr(), &v), n(b.ptr(), &v), "needle {:?}", String::from_utf8_lossy(&needle));
        Ok(())
    })
}

pub unsafe fn diff_inplace_string_ends_with_nocase(orig: usize, new: usize) -> Result<u32, String> {
    // bool return: only `al` is defined, so read u8 and normalize.
    type F = unsafe extern "win64" fn(*const InplaceString, *const StrView) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive(7, 8, |r, a, b, text| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        diff_eq!(o(a.ptr(), &v) != 0, n(b.ptr(), &v) != 0, "needle {:?}", String::from_utf8_lossy(&needle));
        Ok(())
    })
}

pub unsafe fn diff_inplace_string_starts_with_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *const StrView) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive(8, 8, |r, a, b, text| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        diff_eq!(o(a.ptr(), &v) != 0, n(b.ptr(), &v) != 0, "needle {:?}", String::from_utf8_lossy(&needle));
        Ok(())
    })
}

pub unsafe fn diff_inplace_string_compare_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *const StrView) -> i32;
    drive_view::<i32>(9, std::mem::transmute::<usize, F>(orig), std::mem::transmute::<usize, F>(new))
}

// --- Non-leaf members: `reserve` (0x142408f50) is mocked with a test fixture
// so growth paths run without the engine allocator. The fixture is *not* the
// engine's growth policy; both sides just see the same deterministic callee.

use crate::difftest::Mock;
use std::cell::RefCell;

thread_local! {
    /// `n` arguments of fixture calls since the last [`take_reserve_log`].
    static RESERVE_LOG: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    /// Buffers handed out by the fixture (kept alive until the seed ends).
    static RESERVE_BUFS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// Test stand-in for `reserve(this, n)`: no-op if `n + 1` fits the current
/// storage; otherwise a new 0xCD-filled buffer of `round_up(n + 1, 16)` bytes
/// (made larger than the inline area so the object switches to heap mode)
/// receives `len + 1` old bytes.
unsafe extern "win64" fn fake_reserve(this: *mut InplaceString, n: u64) {
    RESERVE_LOG.with(|l| l.borrow_mut().push(n));
    let need = n + 1;
    let (cap, icap) = (capacity(this) as u64, inline_capacity(this) as u64);
    if need <= cap || (cap <= icap && need <= icap) || need > 1 << 16 {
        return;
    }
    let size = ((need + 15) & !15).max((icap + 16) & !15) as usize;
    let mut buf = vec![0xCDu8; size];
    std::ptr::copy_nonoverlapping(storage(this), buf.as_mut_ptr(), len_of(this) as usize + 1);
    std::ptr::write_unaligned((this as *mut u8).add(8) as *mut *mut u8, buf.as_mut_ptr());
    let b = bits(this);
    std::ptr::write_unaligned(this as *mut u32, (b & 0xff00_0000) | (size as u32 >> 3));
    RESERVE_BUFS.with(|v| v.borrow_mut().push(buf));
}

fn take_reserve_log() -> Vec<u64> {
    RESERVE_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// Object bytes (heap pointer neutralized when in heap mode), the original
/// heap buffer, and where the storage points now: 0 inline, 1 original heap,
/// 2 fixture buffer (with its contents), 3 elsewhere.
type Observed = (Vec<u8>, Option<Vec<u8>>, u8, Vec<u8>);

impl TestStr {
    fn observe(&self) -> Observed {
        let mut o = self.obj.clone();
        let p = unsafe { self.obj.as_ptr().add(self.off) as *const InplaceString };
        let (cap, icap) = unsafe { (capacity(p), inline_capacity(p)) };
        if cap <= icap {
            return (o, self.heap.clone(), 0, Vec::new());
        }
        let at = unsafe { std::ptr::read_unaligned(self.obj.as_ptr().add(self.off + 8) as *const usize) };
        o[self.off + 8..self.off + 16].fill(0);
        if self.heap.as_ref().is_some_and(|h| h.as_ptr() as usize == at) {
            return (o, self.heap.clone(), 1, Vec::new());
        }
        let fake = RESERVE_BUFS.with(|v| v.borrow().iter().find(|b| b.as_ptr() as usize == at).cloned());
        match fake {
            Some(b) => (o, self.heap.clone(), 2, b),
            None => (o, self.heap.clone(), 3, Vec::new()),
        }
    }

    /// An `InplaceString<256>`-sized object of random bytes (for outputs that
    /// the function constructs itself).
    fn gen_raw256(seed: u64) -> TestStr {
        let mut r = Rng::new(seed ^ 0x2560);
        let off = if r.below(2) == 0 { 0 } else { 4 };
        TestStr { obj: r.bytes(off + 8 + 256 + 8), off, heap: None }
    }
}

/// Like [`drive_mut`] with `reserve` mocked: per call compares the returned
/// pointer, the fixture's call log and the observed memory of `this`.
unsafe fn drive_grow(
    salt: u64,
    per_seed: u32,
    mut call: impl FnMut(&mut Rng, &[u8], *mut InplaceString, bool) -> usize,
) -> Result<u32, String> {
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(salt);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let text = a.text();
        for k in 0..per_seed {
            let saved = r.0;
            take_reserve_log();
            let ra = call(&mut r, &text, a.ptr(), true);
            let la = take_reserve_log();
            r.0 = saved;
            let rb = call(&mut r, &text, b.ptr(), false);
            let lb = take_reserve_log();
            let ctx = || format!("seed {seed} step {k} text {:?}", String::from_utf8_lossy(&text));
            if a.rel(ra) != b.rel(rb) {
                return Err(format!("return: orig={:?} new={:?} for {}", a.rel(ra), b.rel(rb), ctx()));
            }
            if la != lb {
                return Err(format!("reserve calls: orig={la:?} new={lb:?} for {}", ctx()));
            }
            let (oa, ob) = (a.observe(), b.observe());
            if oa != ob {
                return Err(format!("memory: orig={oa:?} new={ob:?} for {}", ctx()));
            }
            cases += 1;
        }
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_push_back(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(20, 6, |r, _, p, first| {
        let c = r.next_u32() as u8;
        (if first { o } else { n })(p, c) as usize
    })
}

pub unsafe fn diff_inplace_string_append(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(21, 4, |r, text, p, first| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        (if first { o } else { n })(p, &v) as usize
    })
}

/// `pos`/`count` covering in-range, boundary, past-the-end and npos values.
fn gen_pos_count(r: &mut Rng, vlen: u64) -> (u64, u64) {
    let pos = match r.below(6) {
        0 => vlen,
        1 => vlen + 1 + r.below(3),
        2 => 0,
        _ => r.below(vlen + 1),
    };
    let count = match r.below(5) {
        0 => 0,
        1 => u64::MAX,
        2 => vlen,
        _ => r.below(vlen + 3),
    };
    (pos, count)
}

pub unsafe fn diff_inplace_string_append_range(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView, u64, u64) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(22, 4, |r, text, p, first| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        let (pos, count) = gen_pos_count(r, v.len);
        (if first { o } else { n })(p, &v, pos, count) as usize
    })
}

pub unsafe fn diff_inplace_string_assign_range(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView, u64, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(23, 4, |r, text, p, first| {
        let needle = gen_needle(r, text);
        let len = len_of(p) as u64;
        // A quarter of the cases alias the string's own characters (memmove path).
        let v = if len > 0 && r.below(4) == 0 {
            let s = r.below(len + 1);
            let e = s + r.below(len - s + 1);
            StrView { ptr: storage(p).add(s as usize), len: e - s }
        } else {
            view_of(&needle)
        };
        let (pos, count) = gen_pos_count(r, v.len);
        (if first { o } else { n })(p, &v, pos, count);
        p as usize
    })
}

pub unsafe fn diff_inplace_string_assign_view(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(24, 4, |r, text, p, first| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        (if first { o } else { n })(p, &v) as usize
    })
}

pub unsafe fn diff_inplace_string_copy_assign(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const InplaceString) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut k = 0u64;
    let mut others: Vec<TestStr> = Vec::new();
    drive_grow(25, 3, |r, _, p, first| {
        if first {
            k += 1;
            others.clear();
        }
        // Same source string for both sides (built from the same seed); its
        // memory must be left untouched.
        let mut other = TestStr::gen(100_000 + k);
        let before = other.observe();
        let src = if r.below(6) == 0 { p as *const InplaceString } else { other.ptr() };
        let ret = (if first { o } else { n })(p, src) as usize;
        assert_eq!(before, other.observe(), "source modified");
        others.push(other);
        ret
    })
}

pub unsafe fn diff_inplace_string_assign_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(26, 4, |r, text, p, first| {
        let mut s = gen_needle(r, text);
        *s.last_mut().unwrap() = 0;
        let arg = if r.below(8) == 0 { std::ptr::null() } else { s.as_ptr() };
        (if first { o } else { n })(p, arg) as usize
    })
}

pub unsafe fn diff_inplace_string_substr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *mut InplaceString, u64, u64) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(27);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let len = a.text().len() as u64;
        for k in 0..4 {
            let (mut oa, mut ob) = (TestStr::gen_raw256(seed * 8 + k), TestStr::gen_raw256(seed * 8 + k));
            let (pos, count) = gen_pos_count(&mut r, len);
            take_reserve_log();
            let ra = o(a.ptr(), oa.ptr(), pos, count) as usize;
            let la = take_reserve_log();
            let rb = n(b.ptr(), ob.ptr(), pos, count) as usize;
            let lb = take_reserve_log();
            diff_eq!(oa.rel(ra), ob.rel(rb), "return, seed {seed} pos {pos} count {count}");
            diff_eq!(la, lb, "reserve calls, seed {seed} pos {pos} count {count}");
            diff_eq!(oa.observe(), ob.observe(), "out, seed {seed} pos {pos} count {count}");
            diff_eq!(a.observe(), b.observe(), "this, seed {seed}");
            cases += 1;
        }
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_reserve_if_needed(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(28, 3, |r, _, p, first| {
        let cap = capacity(p) as u64;
        let x = match r.below(4) {
            0 => cap,
            1 => cap.saturating_sub(1),
            _ => r.below(400),
        };
        (if first { o } else { n })(p, x);
        p as usize
    })
}

pub unsafe fn diff_inplace_string_resize(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_grow(29, 3, |r, _, p, first| {
        let len = len_of(p) as u64;
        let cap = capacity(p).max(inline_capacity(p)) as u64;
        let x = match r.below(4) {
            0 => r.below(len + 1),
            1 => len,
            _ => r.below(cap.max(1) + 300),
        };
        (if first { o } else { n })(p, x);
        p as usize
    })
}

// --- Destructor: fake region-allocator chunk from `r::mem` tests.

pub unsafe fn diff_inplace_string_dtor(orig: usize, new: usize) -> Result<u32, String> {
    use crate::ns::r::mem::{mock_free_paths, take_free_calls, FakeChunk};
    type F = unsafe extern "win64" fn(*mut InplaceString);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x14240ab10);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        // The string object: heap mode (most cases), inline, or null heap pointer.
        let mut obj = [0u8; 24];
        let off = 4 * r.below(2) as usize;
        let code = [0u8, 1, 4, 0x10, 0x80][r.below(5) as usize] as u32;
        let icap = if code < 0x80 { code * 8 } else { (code << 8) - 0x7c00 };
        let kind = r.below(8);
        let units = if kind == 0 { icap / 8 } else { icap / 8 + 1 + r.below(64) as u32 };
        obj[off..off + 4].copy_from_slice(&((code << 24) | units).to_le_bytes());
        obj[off + 4..off + 8].copy_from_slice(&r.next_u32().to_le_bytes());
        let heap = if kind == 1 { 0 } else { p };
        obj[off + 8..off + 16].copy_from_slice(&(heap as u64).to_le_bytes());
        // Run both from the same state.
        let (pre_desc, pre_p, pre_obj) = (desc.current(), *(p as *const u64), obj);
        take_free_calls();
        o(obj.as_mut_ptr().add(off) as *mut InplaceString);
        let (desc_a, p_a, obj_a, calls_a) = (desc.current(), *(p as *const u64), obj, take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        obj = pre_obj;
        n(obj.as_mut_ptr().add(off) as *mut InplaceString);
        let (desc_b, p_b, obj_b, calls_b) = (desc.current(), *(p as *const u64), obj, take_free_calls());
        let ctx = format!("case {i}: {what} code {code:#x} units {units} kind {kind}");
        diff_eq!(calls_a, calls_b, "free calls, {ctx}");
        diff_eq!(p_a, p_b, "freed block link, {ctx}");
        diff_eq!(obj_a, obj_b, "string object, {ctx}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, {ctx}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

// --- Move assignment / operator+.

thread_local! {
    static DEALLOCS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_deallocate(_this: usize, p: usize) {
    DEALLOCS.with(|d| d.borrow_mut().push(p));
}

impl TestStr {
    /// Where `p` points: 0 own heap, 1 `other`'s heap, 2 fixture buffer, 3 elsewhere.
    fn whose(&self, other: &TestStr, p: usize) -> u8 {
        let is = |t: &TestStr| t.heap.as_ref().is_some_and(|h| h.as_ptr() as usize == p);
        if is(self) {
            0
        } else if is(other) {
            1
        } else if RESERVE_BUFS.with(|v| v.borrow().iter().any(|b| b.as_ptr() as usize == p)) {
            2
        } else {
            3
        }
    }

    /// Object bytes with the heap pointer replaced by [`Self::whose`] (heap
    /// mode only), plus both heap buffers' contents.
    fn observe_with(&self, other: &TestStr) -> (Vec<u8>, Option<u8>, Option<Vec<u8>>) {
        let mut o = self.obj.clone();
        let p = unsafe { self.obj.as_ptr().add(self.off) as *const InplaceString };
        let heap_mode = unsafe { capacity(p) > inline_capacity(p) };
        let mut tag = None;
        if heap_mode {
            let at = unsafe { std::ptr::read_unaligned(self.obj.as_ptr().add(self.off + 8) as *const usize) };
            tag = Some(self.whose(other, at));
            o[self.off + 8..self.off + 16].fill(0);
        }
        (o, tag, self.heap.clone())
    }
}

pub unsafe fn diff_inplace_string_move_assign(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *mut InplaceString) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let _dealloc = Mock::new(HEAP_DEALLOCATE_VA, fake_deallocate as *const ())?;
    let mut r = Rng::new(0x14240a730);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        for k in 0..4u64 {
            let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
            let (mut sa, mut sb) = (TestStr::gen(50_000 + seed * 4 + k), TestStr::gen(50_000 + seed * 4 + k));
            let selfie = r.below(10) == 0;
            let src_heap = sa.heap.is_some() && !selfie;
            let run = |f: F, t: &mut TestStr, s: &mut TestStr| -> (usize, Vec<usize>, Vec<u64>) {
                DEALLOCS.with(|d| d.borrow_mut().clear());
                take_reserve_log();
                let src = if selfie { t.ptr() } else { s.ptr() };
                let ret = f(t.ptr(), src) as usize;
                (ret, DEALLOCS.with(|d| std::mem::take(&mut *d.borrow_mut())), take_reserve_log())
            };
            let (ra, da, la) = run(o, &mut a, &mut sa);
            let (rb, db, lb) = run(n, &mut b, &mut sb);
            let ctx = format!("seed {seed} k {k} self {selfie}");
            diff_eq!(a.rel(ra), b.rel(rb), "return, {ctx}");
            diff_eq!(la, lb, "reserve calls, {ctx}");
            let (wa, wb): (Vec<u8>, Vec<u8>) = (da.iter().map(|&p| a.whose(&sa, p)).collect(), db.iter().map(|&p| b.whose(&sb, p)).collect());
            diff_eq!(wa, wb, "deallocations, {ctx}");
            diff_eq!(a.observe_with(&sa), b.observe_with(&sb), "this, {ctx}");
            // After a steal, src is inline again but bytes 1..8 of its old heap
            // pointer stay in the inline area (only byte 0 becomes the NUL);
            // those addresses differ between the two sides.
            let src_obs = |s: &TestStr, t: &TestStr, was_heap: bool| {
                let mut ob = s.observe_with(t);
                if was_heap && ob.1.is_none() {
                    ob.0[s.off + 9..s.off + 16].fill(0);
                }
                ob
            };
            diff_eq!(src_obs(&sa, &a, src_heap), src_obs(&sb, &b, src_heap), "src, {ctx}");
            // Fixture-buffer contents when `this` moved to one (copy path growth).
            diff_eq!(a.observe().3, b.observe().3, "this fixture buffer, {ctx}");
            RESERVE_BUFS.with(|v| v.borrow_mut().clear());
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_inplace_string_concat(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *mut InplaceString, *const StrView) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(0x14240a160);
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let text = a.text();
        for k in 0..3 {
            let (mut oa, mut ob) = (TestStr::gen_raw256(seed * 8 + k), TestStr::gen_raw256(seed * 8 + k));
            let needle = gen_needle(&mut r, &text);
            let v = view_of(&needle);
            take_reserve_log();
            let ra = o(a.ptr(), oa.ptr(), &v) as usize;
            let la = take_reserve_log();
            let rb = n(b.ptr(), ob.ptr(), &v) as usize;
            let lb = take_reserve_log();
            diff_eq!(oa.rel(ra), ob.rel(rb), "return, seed {seed} k {k}");
            diff_eq!(la, lb, "reserve calls, seed {seed} k {k}");
            diff_eq!(oa.observe(), ob.observe(), "out, seed {seed} k {k}");
            diff_eq!(a.observe(), b.observe(), "this, seed {seed} k {k}");
            cases += 1;
        }
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(cases)
}

// --- replace: the three temps' destructors are recorded instead of run (the
// fixture `reserve` hands out Vec buffers that the region free can't take).

thread_local! {
    static DTORS: RefCell<Vec<(bool, u32)>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_dtor(s: *mut InplaceString) {
    DTORS.with(|d| d.borrow_mut().push((capacity(s) > inline_capacity(s), len_of(s))));
}

pub unsafe fn diff_inplace_string_replace(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u64, u64, *const StrView) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _dtor = Mock::new(DTOR_VA, fake_dtor as *const ())?;
    let _dealloc = Mock::new(HEAP_DEALLOCATE_VA, fake_deallocate as *const ())?;
    let take = || DTORS.with(|d| std::mem::take(&mut *d.borrow_mut()));
    let mut logs: (Vec<(bool, u32)>, usize) = Default::default();
    let mut bad: Option<String> = None;
    let mut long = Vec::new();
    let cases = drive_grow(0x9d70, 4, |r, text, p, first| {
        let len = len_of(p) as u64;
        let needle;
        let v = match r.below(6) {
            // Aliases the string's own characters.
            0 if len > 0 => {
                let s = r.below(len + 1);
                let e = s + r.below(len - s + 1);
                StrView { ptr: storage(p).add(s as usize), len: e - s }
            }
            // Long enough that the 256-byte temps spill to the heap.
            1 => {
                long = (0..200 + r.below(200)).map(|i| b'a' + (i % 26) as u8).collect::<Vec<u8>>();
                view_of(&long)
            }
            _ => {
                needle = gen_needle(r, text);
                view_of(&needle)
            }
        };
        let (pos, count) = gen_pos_count(r, len);
        take();
        DEALLOCS.with(|d| d.borrow_mut().clear());
        let ret = (if first { o } else { n })(p, pos, count, &v) as usize;
        let now = (take(), DEALLOCS.with(|d| std::mem::take(&mut *d.borrow_mut())).len());
        if first {
            logs = now;
        } else if logs != now && bad.is_none() {
            bad = Some(format!("temp dtor/dealloc calls: orig={logs:?} new={now:?} (pos {pos} count {count})"));
        }
        ret
    })?;
    match bad {
        Some(e) => Err(e),
        None => Ok(cases),
    }
}

pub unsafe fn diff_inplace_string_data_at<const OFFSET: usize>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let base_off = a.off;
        let mut obj_a = vec![0xA5u8; OFFSET + a.obj.len()];
        let mut obj_b = vec![0xA5u8; OFFSET + b.obj.len()];
        obj_a[OFFSET..].copy_from_slice(&a.obj);
        obj_b[OFFSET..].copy_from_slice(&b.obj);
        a.obj = obj_a;
        b.obj = obj_b;
        a.off = OFFSET + base_off;
        b.off = OFFSET + base_off;
        let base_a = a.obj.as_ptr().add(base_off);
        let base_b = b.obj.as_ptr().add(base_off);
        let (ra, rb) = (o(base_a) as usize, n(base_b) as usize);
        diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed} off {OFFSET:#x}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed} off {OFFSET:#x}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string_hash(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        diff_eq!(o(a.ptr()), n(b.ptr()), "hash, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string_trim(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const StrView, u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_mut(0x14240b010, 4, |r, text, p, first| {
        let needle = gen_needle(r, text);
        let v = view_of(&needle);
        let trim_left = (r.below(2) == 0) as u8;
        (if first { o } else { n })(p, &v, trim_left) as usize
    })
}

pub unsafe fn diff_inplace_string_trim_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const u8, u8) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_mut(0x142409230, 4, |r, text, p, first| {
        let mut needle = gen_needle(r, text);
        *needle.last_mut().unwrap() = 0;
        let trim_left = (r.below(2) == 0) as u8;
        (if first { o } else { n })(p, needle.as_ptr(), trim_left) as usize
    })
}

thread_local! {
    static RES_ALLOC_SIZES: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_stl_alloc1(_this: *mut u8, n: usize) -> *mut u8 {
    RES_ALLOC_SIZES.with(|v| v.borrow_mut().push(n));
    let mut buf = vec![0xCDu8; n.max(16)];
    let p = buf.as_mut_ptr();
    RESERVE_BUFS.with(|v| v.borrow_mut().push(buf));
    p
}

pub unsafe fn diff_inplace_string_reserve(orig: usize, new: usize) -> Result<u32, String> {
    use crate::ns::r::mem::{mock_free_paths, take_free_calls, FakeChunk};
    type F = unsafe extern "win64" fn(*mut InplaceString, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _free_mocks = mock_free_paths()?;
    let _alloc_mock = Mock::new(STL_ALLOC1_VA, fake_stl_alloc1 as *const ())?;
    let _dealloc_mock = Mock::new(HEAP_DEALLOCATE_VA, fake_deallocate as *const ())?;
    let mut r = Rng::new(0x142408f50);
    let mut cases = 0u32;
    for seed in 1..=250u64 {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let icap = inline_capacity(a.ptr()) as u64;
        let cap = capacity(a.ptr()) as u64;
        if icap == 0 || cap == 0 {
            continue;
        }
        let desc = chunk.descriptors();
        let (fake_p, _) = chunk.setup(&mut r);
        // Point heap-mode string storage into `FakeChunk` so the shrink-to-inline
        // `region_free` path has valid mimalloc segment/page descriptors.
        let was_heap = cap > icap;
        if was_heap {
            let text = a.text();
            std::ptr::copy_nonoverlapping(text.as_ptr(), fake_p as *mut u8, text.len());
            *((fake_p + text.len()) as *mut u8) = 0;
            a.obj[a.off + 8..a.off + 16].copy_from_slice(&(fake_p as u64).to_le_bytes());
            b.obj[b.off + 8..b.off + 16].copy_from_slice(&(fake_p as u64).to_le_bytes());
        }
        let req = match r.below(6) {
            0 => 0,
            1 => icap.saturating_sub(1),
            2 => icap,
            3 => cap,
            4 => cap + 1 + r.below(64),
            _ => 0x101 + r.below(512),
        };
        let (pre_desc, pre_fake_q) = (desc.current(), *(fake_p as *const u64));
        let pre_text = a.text();
        take_free_calls();
        RES_ALLOC_SIZES.with(|v| v.borrow_mut().clear());
        DEALLOCS.with(|d| d.borrow_mut().clear());
        o(a.ptr(), req);
        let calls_a = take_free_calls();
        let allocs_a = RES_ALLOC_SIZES.with(|v| std::mem::take(&mut *v.borrow_mut()));
        let deallocs_a = DEALLOCS.with(|d| std::mem::take(&mut *d.borrow_mut())).len();
        let obs_a = a.observe();

        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(fake_p as *mut u64) = pre_fake_q;
        if was_heap {
            std::ptr::copy_nonoverlapping(pre_text.as_ptr(), fake_p as *mut u8, pre_text.len());
            *((fake_p + pre_text.len()) as *mut u8) = 0;
        }
        n(b.ptr(), req);
        let calls_b = take_free_calls();
        let allocs_b = RES_ALLOC_SIZES.with(|v| std::mem::take(&mut *v.borrow_mut()));
        let deallocs_b = DEALLOCS.with(|d| std::mem::take(&mut *d.borrow_mut())).len();
        let obs_b = b.observe();

        diff_eq!(allocs_a, allocs_b, "alloc sizes, seed {seed} req {req}");
        diff_eq!(deallocs_a, deallocs_b, "dealloc count, seed {seed} req {req}");
        diff_eq!(calls_a, calls_b, "region_free calls, seed {seed} req {req}");
        diff_eq!(obs_a, obs_b, "observed string, seed {seed} req {req}");
        desc.restore();
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
        cases += 1;
    }
    Ok(cases)
}

thread_local! {
    static FMT_CALLS: RefCell<Vec<(usize, u64, u64)>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_append_format_v(_this: *mut InplaceString, fmt: *const u8, va: *const u64) {
    let a0 = exe::rd::<u64>(va as usize);
    let a1 = exe::rd::<u64>(va as usize + 8);
    FMT_CALLS.with(|c| c.borrow_mut().push((fmt as usize, a0, a1)));
}

pub unsafe fn diff_inplace_string_append_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const u8, u64, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _mock = Mock::new(APPEND_FORMAT_V_VA, fake_append_format_v as *const ())?;
    let mut r = Rng::new(0x14240a460);
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let fmt = r.next_u64() as usize as *const u8;
        let (x0, x1) = (r.next_u64(), r.next_u64());
        FMT_CALLS.with(|c| c.borrow_mut().clear());
        o(a.ptr(), fmt, x0, x1);
        let ca = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        n(b.ptr(), fmt, x0, x1);
        let cb = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        diff_eq!(ca, cb, "fmt calls, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, *const u8, u64, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _mock = Mock::new(APPEND_FORMAT_V_VA, fake_append_format_v as *const ())?;
    let mut r = Rng::new(0x142409620);
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen(seed), TestStr::gen(seed));
        let fmt = r.next_u64() as usize as *const u8;
        let (x0, x1) = (r.next_u64(), r.next_u64());
        FMT_CALLS.with(|c| c.borrow_mut().clear());
        o(a.ptr(), fmt, x0, x1);
        let ca = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        n(b.ptr(), fmt, x0, x1);
        let cb = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        diff_eq!(ca, cb, "fmt calls, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

unsafe extern "win64" fn fake_append_format_v_1(_this: *mut InplaceString, fmt: *const u8, va: *const u64) {
    let a0 = exe::rd::<u64>(va as usize);
    FMT_CALLS.with(|c| c.borrow_mut().push((fmt as usize, a0, 0)));
}

pub unsafe fn diff_inplace_string_format_ctor<const UNITS: u8>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut InplaceString, u64, *const u8, u64) -> *mut InplaceString;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _mock = Mock::new(APPEND_FORMAT_V_VA, fake_append_format_v_1 as *const ())?;
    let mut r = Rng::new(0x1401f2430 ^ UNITS as u64);
    for seed in 0..STR_CASES {
        let (mut a, mut b) = (TestStr::gen_raw256(seed), TestStr::gen_raw256(seed));
        let tag = r.next_u64();
        let fmt = r.next_u64() as usize as *const u8;
        let x0 = r.next_u64();
        FMT_CALLS.with(|c| c.borrow_mut().clear());
        let ra = o(a.ptr(), tag, fmt, x0) as usize;
        let ca = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        let rb = n(b.ptr(), tag, fmt, x0) as usize;
        let cb = FMT_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        diff_eq!(a.rel(ra), b.rel(rb), "return, seed {seed}");
        diff_eq!(ca, cb, "fmt calls, seed {seed}");
        diff_eq!(a.state(), b.state(), "memory, seed {seed}");
    }
    Ok(STR_CASES as u32)
}

pub unsafe fn diff_inplace_string_uninit_move<const UNITS: u8, const STRIDE: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, *mut u8) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(0x1401f2480 ^ STRIDE as u64);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let count = r.below(5) as usize;
        let mut src_a = r.bytes(count * STRIDE + 8);
        for k in 0..count {
            let p = src_a.as_mut_ptr().add(k * STRIDE) as *mut InplaceString;
            construct_empty(p, UNITS);
            let len = r.below(32) as usize;
            for c in 0..len {
                *(p as *mut u8).add(8 + c) = b'a' + ((c + k) % 26) as u8;
            }
            *(p as *mut u8).add(8 + len) = 0;
            set_len(p, len as u32);
        }
        let mut src_b = src_a.clone();
        let mut dst_a = r.bytes(count * STRIDE + 16);
        let mut dst_b = dst_a.clone();
        let ra = o(src_a.as_mut_ptr(), src_a.as_mut_ptr().add(count * STRIDE), dst_a.as_mut_ptr()) as usize
            - dst_a.as_ptr() as usize;
        let rb = n(src_b.as_mut_ptr(), src_b.as_mut_ptr().add(count * STRIDE), dst_b.as_mut_ptr()) as usize
            - dst_b.as_ptr() as usize;
        diff_eq!(ra, rb, "return offset, case {i} count {count}");
        diff_eq!(src_a, src_b, "src, case {i} count {count}");
        diff_eq!(dst_a, dst_b, "dst, case {i} count {count}");
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(CASES)
}

pub unsafe fn diff_inplace_string128_uninit_move_iter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const *mut u8, *const *mut u8, *mut u8) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(0x140220bf0);
    const CASES: u32 = 150;
    const STRIDE: usize = 0x88;
    for i in 0..CASES {
        let count = r.below(5) as usize;
        let mut src_a = r.bytes(count * STRIDE + 8);
        for k in 0..count {
            let p = src_a.as_mut_ptr().add(k * STRIDE) as *mut InplaceString;
            construct_empty(p, 0x10);
            let len = r.below(32) as usize;
            for c in 0..len {
                *(p as *mut u8).add(8 + c) = b'a' + ((c + k) % 26) as u8;
            }
            *(p as *mut u8).add(8 + len) = 0;
            set_len(p, len as u32);
        }
        let mut src_b = src_a.clone();
        let mut dst_a = r.bytes(count * STRIDE + 16);
        let mut dst_b = dst_a.clone();
        let (fa, la) = (src_a.as_mut_ptr(), src_a.as_mut_ptr().add(count * STRIDE));
        let (fb, lb) = (src_b.as_mut_ptr(), src_b.as_mut_ptr().add(count * STRIDE));
        let ra = o(&fa, &la, dst_a.as_mut_ptr()) as usize - dst_a.as_ptr() as usize;
        let rb = n(&fb, &lb, dst_b.as_mut_ptr()) as usize - dst_b.as_ptr() as usize;
        diff_eq!(ra, rb, "return offset, case {i} count {count}");
        diff_eq!(src_a, src_b, "src, case {i} count {count}");
        diff_eq!(dst_a, dst_b, "dst, case {i} count {count}");
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(CASES)
}

pub unsafe fn diff_tagged_inplace_string64_uninit_move_iter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const *mut u8, *const *mut u8, *mut u8) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _reserve = Mock::new(RESERVE_VA, fake_reserve as *const ())?;
    let mut r = Rng::new(0x140304fb0);
    const CASES: u32 = 150;
    const STRIDE: usize = 0x4c;
    for i in 0..CASES {
        let count = r.below(5) as usize;
        let mut src_a = r.bytes(count * STRIDE + 8);
        for k in 0..count {
            let base = src_a.as_mut_ptr().add(k * STRIDE);
            exe::wr::<u32>(base as usize, r.next_u32());
            let p = base.add(4) as *mut InplaceString;
            construct_empty(p, 0x08);
            let len = r.below(24) as usize;
            for c in 0..len {
                *(p as *mut u8).add(8 + c) = b'a' + ((c + k) % 26) as u8;
            }
            *(p as *mut u8).add(8 + len) = 0;
            set_len(p, len as u32);
        }
        let mut src_b = src_a.clone();
        let mut dst_a = r.bytes(count * STRIDE + 16);
        let mut dst_b = dst_a.clone();
        let (fa, la) = (src_a.as_mut_ptr(), src_a.as_mut_ptr().add(count * STRIDE));
        let (fb, lb) = (src_b.as_mut_ptr(), src_b.as_mut_ptr().add(count * STRIDE));
        let ra = o(&fa, &la, dst_a.as_mut_ptr()) as usize - dst_a.as_ptr() as usize;
        let rb = n(&fb, &lb, dst_b.as_mut_ptr()) as usize - dst_b.as_ptr() as usize;
        diff_eq!(ra, rb, "return offset, case {i} count {count}");
        diff_eq!(src_a, src_b, "src, case {i} count {count}");
        diff_eq!(dst_a, dst_b, "dst, case {i} count {count}");
        RESERVE_BUFS.with(|v| v.borrow_mut().clear());
    }
    Ok(CASES)
}

thread_local! {
    static VEC_DTOR_OFFS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
    static VEC_BASE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe extern "win64" fn fake_vec_elem_dtor(s: *mut InplaceString) {
    let off = (s as usize).wrapping_sub(VEC_BASE.with(|b| b.get()));
    VEC_DTOR_OFFS.with(|v| v.borrow_mut().push(off));
}

pub unsafe fn diff_inplace_string_vec_destroy<const STRIDE: usize>(orig: usize, new: usize) -> Result<u32, String> {
    use crate::ns::r::mem::{mock_free_paths, take_free_calls, FakeChunk};
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _free_mocks = mock_free_paths()?;
    let _dtor_mock = Mock::new(DTOR_VA, fake_vec_elem_dtor as *const ())?;
    let mut r = Rng::new(0x1401ea4e0 ^ STRIDE as u64);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let ptr = if r.below(8) == 0 { 0 } else { p };
        let len = if ptr == 0 { 0 } else { r.below(6) as u32 };
        let cap = r.next_u32();
        let mut hdr0 = [0u8; 16];
        hdr0[0..8].copy_from_slice(&ptr.to_le_bytes());
        hdr0[8..12].copy_from_slice(&len.to_le_bytes());
        hdr0[12..16].copy_from_slice(&cap.to_le_bytes());
        VEC_BASE.with(|b| b.set(ptr));
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));

        let mut ha = hdr0;
        take_free_calls();
        VEC_DTOR_OFFS.with(|v| v.borrow_mut().clear());
        o(ha.as_mut_ptr());
        let (desc_a, p_a, calls_a) = (desc.current(), *(p as *const u64), take_free_calls());
        let dtors_a = VEC_DTOR_OFFS.with(|v| std::mem::take(&mut *v.borrow_mut()));

        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        let mut hb = hdr0;
        n(hb.as_mut_ptr());
        let (desc_b, p_b, calls_b) = (desc.current(), *(p as *const u64), take_free_calls());
        let dtors_b = VEC_DTOR_OFFS.with(|v| std::mem::take(&mut *v.borrow_mut()));

        diff_eq!(dtors_a, dtors_b, "dtors, case {i} {what} len {len}");
        diff_eq!(ha, hb, "hdr, case {i} {what} len {len}");
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what}");
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES)
}

thread_local! {
    static VEC_REALLOC_LOG: RefCell<Vec<(&'static str, usize, usize, usize)>> = const { RefCell::new(Vec::new()) };
    static VEC_REALLOC_RET: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe extern "win64" fn fake_vec_alloc(this: *mut u8, n: usize) -> *mut u8 {
    let ret = VEC_REALLOC_RET.with(|r| r.get());
    VEC_REALLOC_LOG.with(|v| v.borrow_mut().push(("alloc", this as usize, n, ret)));
    ret as *mut u8
}

unsafe extern "win64" fn fake_vec_move(first: *mut u8, last: *mut u8, dst: *mut u8) -> *mut u8 {
    VEC_REALLOC_LOG.with(|v| v.borrow_mut().push(("move", first as usize, last as usize, dst as usize)));
    dst
}

unsafe extern "win64" fn fake_vec_destroy(vec: *mut u8) {
    VEC_REALLOC_LOG.with(|v| v.borrow_mut().push(("destroy", vec as usize, 0, 0)));
    exe::wr::<usize>(vec as usize, 0);
    exe::wr::<usize>(vec as usize + 8, 0);
}

pub unsafe fn diff_inplace_string_vec_realloc<
    const STRIDE: usize,
    const ALLOC_VA: u64,
    const MOVE_VA: u64,
    const DESTROY_VA: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_alloc = Mock::new(ALLOC_VA, fake_vec_alloc as *const ())?;
    let _m_move = Mock::new(MOVE_VA, fake_vec_move as *const ())?;
    let _m_destroy = Mock::new(DESTROY_VA, fake_vec_destroy as *const ())?;
    let mut r = Rng::new(0x1401f22d0 ^ STRIDE as u64);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let mut hdr0 = [0u8; 16];
        for b in &mut hdr0 {
            *b = r.next_u32() as u8;
        }
        let new_cap = r.next_u64() as usize;
        VEC_REALLOC_RET.with(|c| c.set(r.next_u64() as usize));
        let (mut ha, mut hb) = (hdr0, hdr0);
        VEC_REALLOC_LOG.with(|v| v.borrow_mut().clear());
        o(ha.as_mut_ptr(), new_cap);
        let mut la = VEC_REALLOC_LOG.with(|v| std::mem::take(&mut *v.borrow_mut()));
        n(hb.as_mut_ptr(), new_cap);
        let mut lb = VEC_REALLOC_LOG.with(|v| std::mem::take(&mut *v.borrow_mut()));
        // Neutralize `this` address in `alloc` and `destroy` entries (`ha` vs `hb`).
        for e in &mut la {
            if e.0 == "alloc" || e.0 == "destroy" {
                e.1 = 0;
            }
        }
        for e in &mut lb {
            if e.0 == "alloc" || e.0 == "destroy" {
                e.1 = 0;
            }
        }
        diff_eq!(la, lb, "calls, case {i}");
        diff_eq!(ha, hb, "hdr, case {i}");
    }
    Ok(CASES)
}



/// An inline `InplaceString<256>` test object holding `text` (at offset 0 or 4).
fn inline256(text: &[u8], off: usize) -> Vec<u8> {
    let mut o = vec![0xEEu8; off + 8 + 256 + 8];
    o[off..off + 4].copy_from_slice(&0x2000_0020u32.to_le_bytes());
    o[off + 4..off + 8].copy_from_slice(&(text.len() as u32).to_le_bytes());
    o[off + 8..off + 8 + text.len()].copy_from_slice(text);
    o[off + 8 + text.len()] = 0;
    o
}

/// `a` from the usual generator (all storage modes); `b` derived from `a`'s
/// text (equal, case-flipped, truncated, extended, one char changed, or
/// unrelated) so every exit is reached. Both argument orders are run.
pub unsafe fn diff_inplace_string_less_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const InplaceString, *const InplaceString) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1402174f0);
    const ALPHA: &[u8] = b"aAbBzZ09_/\x80\xc4\xe4";
    let mut cases = 0;
    for seed in 0..STR_CASES {
        let a = TestStr::gen(seed);
        let text = a.text();
        for _ in 0..8 {
            let mut t: Vec<u8> = match r.below(6) {
                0 => text.clone(),
                1 => text.iter().map(|&c| if r.below(2) == 0 { c ^ 0x20 } else { c }).collect(),
                2 => text[..r.below(text.len() as u64 + 1) as usize].to_vec(),
                3 => {
                    let mut t = text.clone();
                    t.extend((0..1 + r.below(3)).map(|_| ALPHA[r.below(ALPHA.len() as u64) as usize]));
                    t
                }
                4 if !text.is_empty() => {
                    let mut t = text.clone();
                    let k = r.below(t.len() as u64) as usize;
                    t[k] = ALPHA[r.below(ALPHA.len() as u64) as usize];
                    t
                }
                _ => (0..r.below(12)).map(|_| ALPHA[r.below(ALPHA.len() as u64) as usize]).collect(),
            };
            t.truncate(250);
            let off = r.below(2) as usize * 4;
            let b = inline256(&t, off);
            let pa = a.obj.as_ptr().add(a.off) as *const InplaceString;
            let pb = b.as_ptr().add(off) as *const InplaceString;
            for (x, y) in [(pa, pb), (pb, pa)] {
                let (ro, rn) = (o(x, y) as u32, n(x, y) as u32);
                if ro != rn {
                    return Err(format!(
                        "seed {seed}: a {:?} b {:?}: orig {ro} new {rn}",
                        String::from_utf8_lossy(&text),
                        String::from_utf8_lossy(&t)
                    ));
                }
                cases += 1;
            }
        }
    }
    Ok(cases)
}
