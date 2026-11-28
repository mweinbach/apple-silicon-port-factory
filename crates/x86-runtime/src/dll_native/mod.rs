//! Native replacements for hot leaf functions inside mapped third-party PE DLLs
//! (code the interpreter/JIT would otherwise execute guest-side).
//!
//! Each replacement is registered at its mapped address only when the FNV-1a hash
//! of the whole original function body matches the DLL build it was written
//! against, so any other build keeps running the original guest code.

pub mod fwd;
pub mod lift;

/// A replacement for the function at `rva` (`len` code bytes, FNV-1a 64 `fnv`).
pub struct DllPatch {
    pub rva: usize,
    pub len: usize,
    pub fnv: u64,
    pub target: usize,
}

/// A difftest for an installed replacement: `diff(orig, new)` with `orig` = the guest
/// original (`addr` tagged for the emulator) and `new` = the native function.
pub struct DllDiff {
    pub name: &'static str,
    pub addr: usize,
    pub new: usize,
    pub diff: unsafe fn(usize, usize) -> Result<u32, String>,
}

/// Game-registered pinned DLL base addresses (`(lowercase_name, base)`).
static PINNED_BASES: std::sync::RwLock<Vec<(&'static str, usize)>> = std::sync::RwLock::new(Vec::new());
/// Game-registered per-DLL leaf patch providers (`(lowercase_name, base) -> Vec<DllPatch>`).
static DLL_PATCHERS: std::sync::RwLock<Vec<fn(&str, usize) -> Vec<DllPatch>>> = std::sync::RwLock::new(Vec::new());
/// Game-registered extra DLL difftest providers.
static EXTRA_DIFFTESTS: std::sync::RwLock<Vec<fn() -> Vec<DllDiff>>> = std::sync::RwLock::new(Vec::new());

/// Register pinned DLL bases for a target whose static lifts embed absolute VAs.
pub fn register_pinned_bases(bases: &[(&'static str, usize)]) {
    PINNED_BASES.write().unwrap().extend_from_slice(bases);
}

/// Register a per-DLL FNV-1a guarded leaf patch provider.
pub fn register_dll_patcher(patcher: fn(&str, usize) -> Vec<DllPatch>) {
    DLL_PATCHERS.write().unwrap().push(patcher);
}

/// Register an extra difftest provider (included by [`difftests`]).
pub fn register_extra_difftests(provider: fn() -> Vec<DllDiff>) {
    EXTRA_DIFFTESTS.write().unwrap().push(provider);
}

/// Difftests of all installed replacements that have one (run by `--diff-test`).
pub fn difftests() -> Vec<DllDiff> {
    let mut v = fwd::difftests();
    if let Ok(extra) = EXTRA_DIFFTESTS.read() {
        for f in extra.iter() {
            v.extend(f());
        }
    }
    v
}

/// The pinned base of DLL `name` (lower-case loader name), if any target registered one.
pub fn pinned_base(name: &str) -> Option<usize> {
    static OFF: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *OFF.get_or_init(|| std::env::var("AW2_DLL_PIN").is_ok_and(|v| v.trim() == "0")) {
        return None;
    }
    PINNED_BASES
        .read()
        .ok()?
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, b)| b)
}

/// Second install step for DLL `name` (lower-case) mapped at `[base, base + size)`, run by
/// the loader once the image's imports are bound: import-forwarder aliasing ([`fwd`]), then
/// the whole-DLL lift ([`lift`], which keeps VAs already native). Returns the number of VAs
/// registered.
pub unsafe fn imports_bound(name: &str, base: usize, size: usize) -> usize {
    fwd::install(name, base, size) + lift::install(name, base, size)
}

pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}

/// `[base, base + size)` of every PE DLL image passed to [`install`].
static MAPPED: std::sync::RwLock<Vec<(usize, usize)>> = std::sync::RwLock::new(Vec::new());

/// `addr` lies in a mapped PE DLL image (e.g. an import slot of cohtml/RenoirCore).
pub fn in_mapped_image(addr: u64) -> bool {
    mapped_image_end(addr).is_some()
}

/// If `addr` lies inside a mapped PE DLL image, returns that image's exclusive end address.
pub fn mapped_image_end(addr: u64) -> Option<u64> {
    let a = addr as usize;
    MAPPED
        .read()
        .ok()?
        .iter()
        .find(|&&(b, s)| a >= b && a - b < s)
        .map(|&(b, s)| (b + s) as u64)
}

/// Per mapped DLL image `(base, size, iat_start, iat_end)`: its import address table
/// directory (PE data directory 12), registered by [`install`]. An import stub
/// `jmp qword ptr [rip+disp32]` in the image whose slot lies inside this range is dispatched
/// through the slot *at call time* ([`iat_stub_slot`], `emu::DF_IAT`, the run loop's DLL stub
/// route). Delay-load slots live outside it (their own directory, rewritten lazily by the
/// image's delay-load helper) and are never resolved this way.
static IAT_DIRS: std::sync::RwLock<Vec<(usize, usize, usize, usize)>> = std::sync::RwLock::new(Vec::new());
/// Lowest image base / highest image end over [`IAT_DIRS`] (the run loop's cheap gate);
/// `(usize::MAX, 0)` while none is registered.
static IAT_LO: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(usize::MAX);
static IAT_HI: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// `[start, end)` of the IAT directory (data directory 12) of the PE image mapped at
/// `[base, base + size)`, if it has a non-empty one inside the image.
pub(crate) unsafe fn pe_iat_dir(base: usize, size: usize) -> Option<(usize, usize)> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    if nt + 24 > size || rd32(nt) != 0x0000_4550 {
        return None;
    }
    let opt = nt + 24;
    let opt_size = rd16(nt + 20) as usize;
    // (NumberOfRvaAndSizes offset, data directory offset) in the optional header.
    let (n_off, dd_off) = match rd16(opt) {
        0x20b => (108, 112),
        0x10b => (92, 96),
        _ => return None,
    };
    let e = dd_off + 12 * 8;
    if opt + opt_size > size || e + 8 > opt_size || (rd32(opt + n_off) as usize) <= 12 {
        return None;
    }
    let rva = rd32(opt + e) as usize;
    let len = rd32(opt + e + 4) as usize;
    if rva == 0 || len == 0 || rva.checked_add(len).map_or(true, |end| end > size) {
        return None;
    }
    Some((base + rva, base + rva + len))
}

/// Register the IAT directory `iat` of the image `[base, base + size)` (see [`IAT_DIRS`]).
/// Bumps the dispatch generation: a stub VA resolved before is re-resolved.
pub(crate) fn register_iat_dir(base: usize, size: usize, iat: (usize, usize)) {
    use std::sync::atomic::Ordering;
    if let Ok(mut v) = IAT_DIRS.write() {
        v.retain(|&(b, s, ..)| b + s <= base || base + size <= b);
        v.push((base, size, iat.0, iat.1));
        IAT_LO.fetch_min(base, Ordering::AcqRel);
        IAT_HI.fetch_max(base + size, Ordering::AcqRel);
        IAT_SEQ.fetch_add(1, Ordering::Relaxed);
        core::sync::atomic::fence(Ordering::Release);
        let n = v.len().min(MAX_IAT_SPANS);
        for (i, &(b, s, i0, i1)) in v.iter().take(n).enumerate() {
            IAT_SPANS[4 * i].store(b as u64, Ordering::Relaxed);
            IAT_SPANS[4 * i + 1].store(s as u64, Ordering::Relaxed);
            IAT_SPANS[4 * i + 2].store(i0 as u64, Ordering::Relaxed);
            IAT_SPANS[4 * i + 3].store(i1 as u64, Ordering::Relaxed);
        }
        IAT_NSPANS.store(n, Ordering::Relaxed);
        IAT_OVERFLOW.store(v.len() > MAX_IAT_SPANS, Ordering::Relaxed);
        IAT_SEQ.fetch_add(1, Ordering::Release);
    }
    crate::exe::invalidate_dispatch();
}

/// Lock-free seqlock mirror of [`IAT_DIRS`] (`(base, size, iat_lo, iat_hi)` tuples) so parallel
/// AOT installs and runtime dispatch checks never contend on `IAT_DIRS`'s `RwLock`.
const MAX_IAT_SPANS: usize = 64;
static IAT_SPANS: [std::sync::atomic::AtomicU64; 4 * MAX_IAT_SPANS] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 4 * MAX_IAT_SPANS];
static IAT_NSPANS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static IAT_OVERFLOW: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static IAT_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Lock-free lookup of the `(base, size, iat_lo, iat_hi)` entry containing `a` (falling back
/// to `IAT_DIRS` only when more than [`MAX_IAT_SPANS`] DLLs are mapped).
#[inline]
fn iat_dir_for(a: usize) -> Option<(usize, usize, usize, usize)> {
    use std::sync::atomic::Ordering;
    loop {
        let s1 = IAT_SEQ.load(Ordering::Acquire);
        if s1 & 1 != 0 {
            core::hint::spin_loop();
            continue;
        }
        let n = IAT_NSPANS.load(Ordering::Relaxed).min(MAX_IAT_SPANS);
        let mut found = None;
        for i in 0..n {
            let b = IAT_SPANS[4 * i].load(Ordering::Relaxed) as usize;
            let s = IAT_SPANS[4 * i + 1].load(Ordering::Relaxed) as usize;
            if a >= b && a - b < s {
                let i0 = IAT_SPANS[4 * i + 2].load(Ordering::Relaxed) as usize;
                let i1 = IAT_SPANS[4 * i + 3].load(Ordering::Relaxed) as usize;
                found = Some((b, s, i0, i1));
                break;
            }
        }
        let overflow = found.is_none() && IAT_OVERFLOW.load(Ordering::Relaxed);
        core::sync::atomic::fence(Ordering::Acquire);
        let s2 = IAT_SEQ.load(Ordering::Relaxed);
        if s1 == s2 {
            if overflow {
                let Ok(v) = IAT_DIRS.read() else { return None };
                return v.iter().copied().find(|&(b, s, ..)| a >= b && a - b < s);
            }
            return found;
        }
    }
}

/// Image bounds `(base, end)` when `[addr, addr + 2)` lies inside a mapped DLL image with a
/// registered IAT directory (lock-free over [`IAT_SPANS`]).
#[inline]
pub fn iat_image_bounds(addr: u64) -> Option<(usize, usize)> {
    let a = addr as usize;
    let (b, s, ..) = iat_dir_for(a)?;
    if a - b < s.saturating_sub(1) {
        Some((b, b + s))
    } else {
        None
    }
}

/// `[addr, addr + 2)` lies inside a mapped DLL image with a registered IAT directory (so the
/// run loop may read a 2-byte opcode there; the [`iat_gate`] span alone may cover gaps
/// between images).
#[inline]
pub fn in_iat_image(addr: u64) -> bool {
    iat_image_bounds(addr).is_some()
}

/// `(lowest base, span)` over the images with a registered IAT directory: `ip` may be an
/// import stub only if `ip - lowest base < span`. `(0, 0)` when none is registered.
#[inline]
pub fn iat_gate() -> (u64, u64) {
    use std::sync::atomic::Ordering;
    let lo = IAT_LO.load(Ordering::Acquire);
    let hi = IAT_HI.load(Ordering::Acquire);
    if hi <= lo {
        return (0, 0);
    }
    (lo as u64, (hi - lo) as u64)
}

/// Why an import-stub-shaped `jmp qword ptr [rip+disp32]` is not dispatched through its slot.
/// (A DLL image without an IAT directory is not registered at all: [`install`] logs it as
/// `no_iat_dir`, and its stubs stay guest code.)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StubRefusal {
    /// The slot lies outside the image's IAT directory (delay-load slot, or a data pointer):
    /// it may be rewritten lazily, so the guest executes the stub.
    SlotOutsideIat,
}

/// The import slot of the stub at `va`: `va` lies in a mapped DLL image with a registered IAT
/// directory, its bytes (after up to 4 in-image `E9 rel32` incremental-link jump hops, e.g.
/// `v8.dll+0xad7f0 -> +0x426ad0 -> +0x1090110` `[memchr]`) are `FF 25 disp32` (or
/// `48 FF 25 disp32`), and the slot `cur + len + disp32` is an aligned qword inside that
/// image's IAT directory. `Ok(None)`: not a stub (other bytes / outside every registered
/// image); `Err`: stub-shaped but refused.
pub fn iat_stub_slot_checked(va: u64) -> Result<Option<usize>, StubRefusal> {
    let mut a = va as usize;
    let (lo, span) = iat_gate();
    if (va.wrapping_sub(lo)) >= span {
        return Ok(None);
    }
    let Some((b, s, i0, i1)) = iat_dir_for(a) else {
        return Ok(None);
    };
    let end = b + s;
    // SAFETY: `[a, a + len)` lies inside the mapped image `[b, end)` (checked before each read).
    let len = unsafe {
        let rd = |p: usize| core::ptr::read_unaligned(p as *const u8);
        for _ in 0..4 {
            if a + 5 <= end && rd(a) == 0xE9 {
                let rel = core::ptr::read_unaligned((a + 1) as *const i32) as i64;
                let nxt = (a as i64).wrapping_add(5).wrapping_add(rel);
                if nxt >= b as i64 && (nxt as usize) < end && (nxt as usize) != a {
                    a = nxt as usize;
                    continue;
                }
                return Ok(None);
            }
            break;
        }
        if a + 6 <= end && rd(a) == 0xFF && rd(a + 1) == 0x25 {
            6
        } else if a + 7 <= end && rd(a) == 0x48 && rd(a + 1) == 0xFF && rd(a + 2) == 0x25 {
            7
        } else {
            return Ok(None);
        }
    };
    let disp = unsafe { core::ptr::read_unaligned((a + len - 4) as *const i32) } as i64;
    let slot = (a as i64).wrapping_add(len as i64).wrapping_add(disp) as usize;
    if slot & 7 == 0 && slot >= i0 && slot.checked_add(8).is_some_and(|e| e <= i1) {
        Ok(Some(slot))
    } else {
        Err(StubRefusal::SlotOutsideIat)
    }
}

/// [`iat_stub_slot_checked`] without the refusal reason.
#[inline]
pub fn iat_stub_slot(va: u64) -> Option<usize> {
    iat_stub_slot_checked(va).ok().flatten()
}

/// Per mapped DLL image `(base, size)`: its read-only, non-executable sections.
static RO_SECTIONS: std::sync::RwLock<Vec<((usize, usize), Vec<(usize, usize)>)>> = std::sync::RwLock::new(Vec::new());

/// Read-only, non-executable sections `[start, end)` of the main exe (parsed on first use
/// once the exe is mapped).
static EXE_RO: std::sync::OnceLock<((usize, usize), Vec<(usize, usize)>)> = std::sync::OnceLock::new();

/// `[start, end)` of every section of the PE image mapped at `[base, base + size)` (headers
/// included in the mapping) that is readable, not writable and not executable per its
/// section characteristics. Empty if the headers are not a valid PE.
unsafe fn pe_ro_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const MEM_EXECUTE: u32 = 0x2000_0000;
    const MEM_READ: u32 = 0x4000_0000;
    const MEM_WRITE: u32 = 0x8000_0000;
    let mut out = Vec::new();
    if base == 0 || size < 0x400 {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return out;
    }
    let nt = rd32(0x3C) as usize;
    if nt + 24 > size || rd32(nt) != 0x0000_4550 {
        return out;
    }
    let nsec = rd16(nt + 6) as usize;
    let table = nt + 24 + rd16(nt + 20) as usize;
    if table + nsec * 40 > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if ch & MEM_READ == 0 || ch & (MEM_WRITE | MEM_EXECUTE) != 0 || vsize == 0 {
            continue;
        }
        if va.checked_add(vsize).map_or(true, |end| end > size) {
            continue;
        }
        out.push((base + va, base + va + vsize));
    }
    out
}

/// `[a, a + len)` lies entirely inside one read-only, non-executable section of the main exe
/// or of a mapped PE DLL, and `code` (the accessing instruction) lies in the same image (so
/// the access is image-relative and dies with the image).
pub fn in_ro_section(code: u64, a: u64, len: u64) -> bool {
    let (c, a, len) = (code as usize, a as usize, len as usize);
    let Some(end) = a.checked_add(len) else { return false };
    let hit = |img: (usize, usize), secs: &[(usize, usize)]| {
        c >= img.0 && c - img.0 < img.1 && secs.iter().any(|&(s, e)| a >= s && end <= e)
    };
    let exe_base = crate::exe::base();
    // (Unit tests keep the default exe base/size with nothing mapped there.)
    if !cfg!(test) && exe_base != 0 && crate::exe::image_size() != 0 {
        let (img, secs) = EXE_RO.get_or_init(|| {
            let size = crate::exe::image_size() as usize;
            ((exe_base, size), unsafe { pe_ro_sections(exe_base, size) })
        });
        if img.0 == exe_base && hit(*img, secs) {
            return true;
        }
    }
    RO_SECTIONS.read().is_ok_and(|v| v.iter().any(|(img, secs)| hit(*img, secs)))
}

/// MSVC x64 `__chkstk` (`chkstk.asm`, 78 bytes): saves `r10`/`r11` at `[rsp-0x10..rsp)`,
/// computes `r10 = (rsp + 8).saturating_sub(rax)`, compares against `gs:[0x10]` (`TEB.StackLimit`),
/// probes any crossed pages downward by `0x1000` with `mov byte ptr [r11], 0`, restores
/// `r10`/`r11`, `add rsp, 0x10`, `ret`. Detected by byte pattern across executable sections
/// of mapped PE DLLs (no hardcoded RVAs or module lists).
pub(crate) const CHKSTK_SIG: [u8; 78] = [
    0x48, 0x83, 0xEC, 0x10,                               // sub     rsp, 0x10
    0x4C, 0x89, 0x14, 0x24,                               // mov     [rsp], r10
    0x4C, 0x89, 0x5C, 0x24, 0x08,                         // mov     [rsp + 8], r11
    0x4D, 0x33, 0xDB,                                     // xor     r11, r11
    0x4C, 0x8D, 0x54, 0x24, 0x18,                         // lea     r10, [rsp + 0x18]
    0x4C, 0x2B, 0xD0,                                     // sub     r10, rax
    0x4D, 0x0F, 0x42, 0xD3,                               // cmovb   r10, r11
    0x65, 0x4C, 0x8B, 0x1C, 0x25, 0x10, 0x00, 0x00, 0x00, // mov     r11, gs:[0x10]
    0x4D, 0x3B, 0xD3,                                     // cmp     r10, r11
    0x73, 0x16,                                           // jae     +0x16
    0x66, 0x41, 0x81, 0xE2, 0x00, 0xF0,                   // and     r10w, 0xF000
    0x4D, 0x8D, 0x9B, 0x00, 0xF0, 0xFF, 0xFF,             // lea     r11, [r11 - 0x1000]
    0x41, 0xC6, 0x03, 0x00,                               // mov     byte ptr [r11], 0
    0x4D, 0x3B, 0xD3,                                     // cmp     r10, r11
    0x75, 0xF0,                                           // jne     -0x10
    0x4C, 0x8B, 0x14, 0x24,                               // mov     r10, [rsp]
    0x4C, 0x8B, 0x5C, 0x24, 0x08,                         // mov     r11, [rsp + 8]
    0x48, 0x83, 0xC4, 0x10,                               // add     rsp, 0x10
    0xC3,                                                 // ret
];

const MAX_CHKSTK_VAS: usize = 32;
static CHKSTK_DIRS: std::sync::RwLock<Vec<(usize, usize, u64)>> = std::sync::RwLock::new(Vec::new());
static CHKSTK_VAS: [std::sync::atomic::AtomicU64; MAX_CHKSTK_VAS] =
    [const { std::sync::atomic::AtomicU64::new(0) }; MAX_CHKSTK_VAS];
static CHKSTK_NVAS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static CHKSTK_LO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
static CHKSTK_SPAN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Executable sections `[start, end)` of the PE image mapped at `[base, base + size)`.
unsafe fn pe_exec_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const CNT_CODE: u32 = 0x20;
    const MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = Vec::new();
    if base == 0 || size < 0x400 {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return out;
    }
    let nt = rd32(0x3C) as usize;
    if nt + 24 > size || rd32(nt) != 0x0000_4550 {
        return out;
    }
    let nsec = rd16(nt + 6) as usize;
    let table = nt + 24 + rd16(nt + 20) as usize;
    if table + nsec * 40 > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if ch & (MEM_EXECUTE | CNT_CODE) == 0 || vsize == 0 {
            continue;
        }
        let end = va.saturating_add(vsize).min(size);
        if va < end {
            out.push((base + va, base + end));
        }
    }
    out
}

/// Scan executable sections of the PE image mapped at `[base, base + size)` for [`CHKSTK_SIG`]
/// and any direct `jmp rel32` (`E9`) thunks targeting a detected [`CHKSTK_SIG`] body (such as
/// `lld-link` incremental jump stubs in `v8.dll` and `v8_libbase.dll`).
pub(crate) unsafe fn scan_pe_chkstk(base: usize, size: usize) -> Vec<u64> {
    // Anchor on the `gs:` segment override byte (`0x65`) at offset 28 inside `CHKSTK_SIG`:
    // one of the rarest bytes in x86-64 code, letting vectorized `memchr` skip nearly the
    // entire `.text` section without byte-by-byte scalar iteration.
    const ANCHOR_OFF: usize = 28;
    const ANCHOR_BYTE: u8 = 0x65;
    const _: () = assert!(CHKSTK_SIG[ANCHOR_OFF] == ANCHOR_BYTE);

    let secs = pe_exec_sections(base, size);
    let mut bodies = Vec::new();
    let head = u32::from_le_bytes([CHKSTK_SIG[0], CHKSTK_SIG[1], CHKSTK_SIG[2], CHKSTK_SIG[3]]);
    for &(s, e) in &secs {
        let len = e - s;
        if len < CHKSTK_SIG.len() {
            continue;
        }
        let bytes = core::slice::from_raw_parts(s as *const u8, len);
        let search_end = len - CHKSTK_SIG.len() + ANCHOR_OFF + 1;
        let mut pos = ANCHOR_OFF;
        while pos < search_end {
            let p = win32_rt::libc::memchr(
                bytes.as_ptr().add(pos) as *const win32_rt::libc::c_void,
                ANCHOR_BYTE as win32_rt::libc::c_int,
                search_end - pos,
            );
            if p.is_null() {
                break;
            }
            let hit = (p as usize) - (bytes.as_ptr() as usize);
            let off = hit - ANCHOR_OFF;
            if core::ptr::read_unaligned(bytes.as_ptr().add(off) as *const u32) == head
                && bytes[off..off + CHKSTK_SIG.len()] == CHKSTK_SIG
            {
                bodies.push((s + off) as u64);
                pos = off + CHKSTK_SIG.len() + ANCHOR_OFF;
            } else {
                pos = hit + 1;
            }
        }
    }
    if bodies.is_empty() {
        return bodies;
    }
    let mut found = bodies.clone();
    for &(s, e) in &secs {
        let len = e - s;
        if len < 5 {
            continue;
        }
        let bytes = core::slice::from_raw_parts(s as *const u8, len);
        let search_end = len - 4;
        let mut off = 0;
        while off < search_end {
            let p = win32_rt::libc::memchr(
                bytes.as_ptr().add(off) as *const win32_rt::libc::c_void,
                0xE9,
                search_end - off,
            );
            if p.is_null() {
                break;
            }
            let hit = (p as usize) - (bytes.as_ptr() as usize);
            let rel = core::ptr::read_unaligned(bytes.as_ptr().add(hit + 1) as *const i32) as i64;
            let tgt = ((s + hit + 5) as u64).wrapping_add(rel as u64);
            if bodies.contains(&tgt) {
                found.push((s + hit) as u64);
            }
            off = hit + 1;
        }
    }
    found
}

/// Register `__chkstk` VAs detected in `[base, base + size)` (replaces any prior entries in an
/// overlapping span) and bump the dispatch generation.
pub(crate) fn register_chkstk_vas(base: usize, size: usize, vas: &[u64]) {
    use std::sync::atomic::Ordering;
    if let Ok(mut v) = CHKSTK_DIRS.write() {
        v.retain(|&(b, s, _)| b + s <= base || base + size <= b);
        for &va in vas {
            v.push((base, size, va));
        }
        let n = v.len().min(MAX_CHKSTK_VAS);
        let (mut lo, mut hi) = (u64::MAX, 0u64);
        for (i, &(_, _, va)) in v.iter().take(n).enumerate() {
            CHKSTK_VAS[i].store(va, Ordering::Relaxed);
            lo = lo.min(va);
            hi = hi.max(va);
        }
        CHKSTK_NVAS.store(n, Ordering::Release);
        if n == 0 {
            CHKSTK_SPAN.store(0, Ordering::Release);
            CHKSTK_LO.store(u64::MAX, Ordering::Release);
        } else {
            CHKSTK_LO.store(lo, Ordering::Release);
            CHKSTK_SPAN.store(hi - lo + 1, Ordering::Release);
        }
    }
    crate::exe::invalidate_dispatch();
}

/// `va` is a byte-pattern-detected MSVC x64 `__chkstk` helper inside a mapped PE DLL.
#[inline]
pub fn is_dll_chkstk(va: u64) -> bool {
    use std::sync::atomic::Ordering;
    let span = CHKSTK_SPAN.load(Ordering::Acquire);
    if span == 0 {
        return false;
    }
    let lo = CHKSTK_LO.load(Ordering::Acquire);
    if va.wrapping_sub(lo) >= span {
        return false;
    }
    let n = CHKSTK_NVAS.load(Ordering::Acquire).min(MAX_CHKSTK_VAS);
    (0..n).any(|i| CHKSTK_VAS[i].load(Ordering::Relaxed) == va)
}

/// Standalone `lea rax, [rsp]; ret` helper (`48 8D 04 24 C3`, e.g. V8 `GetCurrentStackPosition`)
/// that reads the guest stack pointer at its own entry (`caller_rsp - 8` on `call`, `entry_rsp`
/// on `jmp`) without touching flags or any other register. Detected structurally by byte pattern
/// at 16-byte-aligned function boundaries in executable PE sections (plus standalone 16-byte-
/// aligned `E9 rel32` jump thunks targeting a detected body), with zero hardcoded RVAs.
pub(crate) const STACK_POS_SIG: [u8; 5] = [0x48, 0x8D, 0x04, 0x24, 0xC3];

const MAX_STACK_POS_VAS: usize = 32;
static STACK_POS_DIRS: std::sync::RwLock<Vec<(usize, usize, u64)>> = std::sync::RwLock::new(Vec::new());
static STACK_POS_VAS: [std::sync::atomic::AtomicU64; MAX_STACK_POS_VAS] =
    [const { std::sync::atomic::AtomicU64::new(0) }; MAX_STACK_POS_VAS];
static STACK_POS_NVAS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static STACK_POS_LO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
static STACK_POS_SPAN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Scan executable sections of `[base, base + size)` for standalone [`STACK_POS_SIG`] bodies and
/// standalone 16-byte-aligned `E9 rel32` jump thunks targeting them.
pub(crate) unsafe fn scan_pe_stack_pos(base: usize, size: usize) -> Vec<u64> {
    let secs = pe_exec_sections(base, size);
    let mut bodies = Vec::new();
    let head = u32::from_le_bytes([STACK_POS_SIG[0], STACK_POS_SIG[1], STACK_POS_SIG[2], STACK_POS_SIG[3]]);
    for &(s, e) in &secs {
        let len = e - s;
        if len < STACK_POS_SIG.len() {
            continue;
        }
        let bytes = core::slice::from_raw_parts(s as *const u8, len);
        let limit = len - STACK_POS_SIG.len();
        // Only 16-byte-aligned addresses `(s + off) & 0xF == 0` can match: stride by 16.
        let mut off = (16 - (s & 0xF)) & 0xF;
        while off <= limit {
            if core::ptr::read_unaligned(bytes.as_ptr().add(off) as *const u32) == head
                && bytes[off + 4] == STACK_POS_SIG[4]
                && (off == 0 || matches!(bytes[off - 1], 0xCC | 0xC3 | 0x90))
                && (off + STACK_POS_SIG.len() == len
                    || matches!(bytes[off + STACK_POS_SIG.len()], 0xCC | 0x90))
            {
                bodies.push((s + off) as u64);
            }
            off += 16;
        }
    }
    if bodies.is_empty() {
        return bodies;
    }
    let mut found = bodies.clone();
    for &(s, e) in &secs {
        let len = e - s;
        if len < 5 {
            continue;
        }
        let bytes = core::slice::from_raw_parts(s as *const u8, len);
        let limit = len - 5;
        let mut off = (16 - (s & 0xF)) & 0xF;
        while off <= limit {
            if bytes[off] == 0xE9
                && (off == 0 || matches!(bytes[off - 1], 0xCC | 0xC3 | 0x90))
                && (off + 5 == len || matches!(bytes[off + 5], 0xCC | 0x90 | 0xE9))
            {
                let rel = core::ptr::read_unaligned(bytes.as_ptr().add(off + 1) as *const i32) as i64;
                let tgt = ((s + off + 5) as u64).wrapping_add(rel as u64);
                if bodies.contains(&tgt) {
                    found.push((s + off) as u64);
                }
            }
            off += 16;
        }
    }
    found
}

/// Register [`STACK_POS_SIG`] VAs detected in `[base, base + size)` and bump `va_gen`.
pub(crate) fn register_stack_pos_vas(base: usize, size: usize, vas: &[u64]) {
    use std::sync::atomic::Ordering;
    if let Ok(mut v) = STACK_POS_DIRS.write() {
        v.retain(|&(b, s, _)| b + s <= base || base + size <= b);
        for &va in vas {
            v.push((base, size, va));
        }
        let n = v.len().min(MAX_STACK_POS_VAS);
        let (mut lo, mut hi) = (u64::MAX, 0u64);
        for (i, &(_, _, va)) in v.iter().take(n).enumerate() {
            STACK_POS_VAS[i].store(va, Ordering::Relaxed);
            lo = lo.min(va);
            hi = hi.max(va);
        }
        STACK_POS_NVAS.store(n, Ordering::Release);
        if n == 0 {
            STACK_POS_SPAN.store(0, Ordering::Release);
            STACK_POS_LO.store(u64::MAX, Ordering::Release);
        } else {
            STACK_POS_LO.store(lo, Ordering::Release);
            STACK_POS_SPAN.store(hi - lo + 1, Ordering::Release);
        }
    }
    crate::exe::invalidate_dispatch();
}

/// `va` is a byte-pattern-detected `lea rax, [rsp]; ret` helper (or standalone `E9` thunk to one)
/// inside a mapped PE DLL.
#[inline]
pub fn is_dll_stack_pos(va: u64) -> bool {
    use std::sync::atomic::Ordering;
    let span = STACK_POS_SPAN.load(Ordering::Acquire);
    if span == 0 {
        return false;
    }
    let lo = STACK_POS_LO.load(Ordering::Acquire);
    if va.wrapping_sub(lo) >= span {
        return false;
    }
    let n = STACK_POS_NVAS.load(Ordering::Acquire).min(MAX_STACK_POS_VAS);
    (0..n).any(|i| STACK_POS_VAS[i].load(Ordering::Relaxed) == va)
}

/// Register the native replacements for DLL `name` (lower-case) mapped at
/// `[base, base + size)`. Returns the number of replacements installed.
pub unsafe fn install(name: &str, base: usize, size: usize) -> usize {
    if let Ok(mut v) = MAPPED.write() {
        v.push((base, size));
    }
    if let Some(iat) = pe_iat_dir(base, size) {
        register_iat_dir(base, size, iat);
    } else {
        log::info!("dll_native: {name}: no IAT directory; its import stubs stay guest code (no_iat_dir)");
    }
    let chkstk = scan_pe_chkstk(base, size);
    if !chkstk.is_empty() {
        register_chkstk_vas(base, size, &chkstk);
        let rvas: Vec<String> = chkstk.iter().map(|&va| format!("+0x{:x}", va - base as u64)).collect();
        log::info!("dll_native: {name}: detected {} __chkstk helper(s) ({})", chkstk.len(), rvas.join(", "));
    }
    let stack_pos = scan_pe_stack_pos(base, size);
    if !stack_pos.is_empty() {
        register_stack_pos_vas(base, size, &stack_pos);
        let rvas: Vec<String> = stack_pos.iter().map(|&va| format!("+0x{:x}", va - base as u64)).collect();
        log::info!(
            "dll_native: {name}: detected {} stack_pos helper(s) ({})",
            stack_pos.len(),
            rvas.join(", ")
        );
    }
    let ro = pe_ro_sections(base, size);
    if !ro.is_empty() {
        if let Ok(mut v) = RO_SECTIONS.write() {
            v.push(((base, size), ro));
        }
    }
    let mut patches = Vec::new();
    if let Ok(patchers) = DLL_PATCHERS.read() {
        for pfn in patchers.iter() {
            patches.extend(pfn(name, base));
        }
    }
    if patches.is_empty() {
        return 0;
    }
    let mut n = 0;
    for p in patches {
        if p.rva + p.len > size {
            continue;
        }
        let code = core::slice::from_raw_parts((base + p.rva) as *const u8, p.len);
        if fnv1a(code) != p.fnv {
            log::warn!("dll_native: {name}+0x{:x} code mismatch; keeping guest code", p.rva);
            continue;
        }
        crate::exe::register_native_va((base + p.rva) as u64, p.target);
        n += 1;
    }
    n
}

#[cfg(test)]
mod scan_tests {
    use super::*;

    fn build_pe_with_text(text_rva: usize, text_size: usize) -> Vec<u8> {
        let total = text_rva + text_size;
        let mut buf = vec![0xCCu8; total];
        let w16 = |b: &mut [u8], off: usize, v: u16| b[off..off + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], off: usize, v: u32| b[off..off + 4].copy_from_slice(&v.to_le_bytes());
        w16(&mut buf, 0, 0x5A4D);
        w32(&mut buf, 0x3C, 0x80);
        w32(&mut buf, 0x80, 0x0000_4550);
        w16(&mut buf, 0x80 + 6, 1); // NumberOfSections
        w16(&mut buf, 0x80 + 20, 240); // SizeOfOptionalHeader
        let sec = 0x80 + 24 + 240;
        buf[sec..sec + 5].copy_from_slice(b".text");
        w32(&mut buf, sec + 8, text_size as u32);
        w32(&mut buf, sec + 12, text_rva as u32);
        w32(&mut buf, sec + 36, 0x6000_0020); // CNT_CODE | MEM_EXECUTE | MEM_READ
        buf
    }

    #[test]
    fn fast_scan_pe_chkstk_and_stack_pos_match_bodies_thunks_and_edges() {
        // Test both 16-byte-aligned and intentionally unaligned section starts.
        let mut backing = vec![0u8; 0x3020];
        for misalign in [0usize, 5usize] {
            let pe = build_pe_with_text(0x1000, 0x1000);
            let slice = &mut backing[misalign..misalign + pe.len()];
            slice.copy_from_slice(&pe);
            let base = slice.as_ptr() as usize;
            let text_start = base + 0x1000;

            // Place a false-positive `0x65` byte, a real CHKSTK_SIG at unaligned offset +0x43,
            // and an `E9` thunk at +0x120 targeting +0x43.
            slice[0x1000 + 0x20] = 0x65;
            let chk_off = 0x1000 + 0x43;
            slice[chk_off..chk_off + CHKSTK_SIG.len()].copy_from_slice(&CHKSTK_SIG);
            let chk_thunk_off = 0x1000 + 0x120;
            let rel = (chk_off as i64 - (chk_thunk_off + 5) as i64) as i32;
            slice[chk_thunk_off] = 0xE9;
            slice[chk_thunk_off + 1..chk_thunk_off + 5].copy_from_slice(&rel.to_le_bytes());

            // Place a real 16-byte-aligned STACK_POS_SIG and a 16-byte-aligned `E9` thunk to it,
            // plus an unaligned STACK_POS_SIG that must be ignored.
            let align_delta = (16 - (text_start & 0xF)) & 0xF;
            let sp_off = 0x1000 + align_delta + 0x200;
            slice[sp_off..sp_off + STACK_POS_SIG.len()].copy_from_slice(&STACK_POS_SIG);
            let bad_sp_off = sp_off + 3 + 16; // unaligned
            slice[bad_sp_off..bad_sp_off + STACK_POS_SIG.len()].copy_from_slice(&STACK_POS_SIG);
            let sp_thunk_off = 0x1000 + align_delta + 0x300;
            let sp_rel = (sp_off as i64 - (sp_thunk_off + 5) as i64) as i32;
            slice[sp_thunk_off] = 0xE9;
            slice[sp_thunk_off + 1..sp_thunk_off + 5].copy_from_slice(&sp_rel.to_le_bytes());

            let chk = unsafe { scan_pe_chkstk(base, slice.len()) };
            assert_eq!(
                chk,
                vec![(base + chk_off) as u64, (base + chk_thunk_off) as u64],
                "chkstk scan mismatch (misalign={misalign})"
            );

            let sp = unsafe { scan_pe_stack_pos(base, slice.len()) };
            assert_eq!(
                sp,
                vec![(base + sp_off) as u64, (base + sp_thunk_off) as u64],
                "stack_pos scan mismatch (misalign={misalign})"
            );
        }
    }
}
