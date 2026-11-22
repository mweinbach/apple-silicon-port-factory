//! `msvcp140.dll` (MSVC STL runtime shipped with the game) `_Mtx_lock` / `_Mtx_unlock`.
//!
//! cohtml/RenoirCore lock a `std::mutex` ~120 times per frame on the render thread alone
//! (3M lock/unlock pairs per profile, ~87 guest instructions plus three guest->host
//! transitions each). The replacements below run the same steps natively and call the
//! DLL's own kernel32 imports (read from its IAT at call time).
//!
//! `_Mtx_internal_imp_t` layout of this build: `+0x00 u32 type` (`_Mtx_plain = 1`,
//! `_Mtx_try = 2`, `_Mtx_timed = 4`, `| _Mtx_recursive = 0x100`), `+0x10 SRWLOCK`,
//! `+0x48 u32 owner thread id` (`-1` = none), `+0x4c u32 recursion count`.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::{DllDiff, DllPatch};
use crate::exe::{self, rd, wr};

/// Mapped base of the DLL (set by [`patches`] before any replacement is registered).
static BASE: AtomicUsize = AtomicUsize::new(0);

/// `_Mtx_lock` export (`xor edx, edx; jmp mtx_do_lock`).
const RVA_MTX_LOCK: usize = 0x1_25c0;
/// FNV guard length for `_Mtx_lock`: the stub plus everything up to the end of the shared
/// `mtx_do_lock(mtx, const xtime* target)` body at `+0x12690..+0x127c1` it jumps to.
const MTX_LOCK_GUARD_LEN: usize = 0x1_27c1 - RVA_MTX_LOCK;
/// `_Mtx_unlock` export (0x1f bytes).
const RVA_MTX_UNLOCK: usize = 0x1_2630;

/// IAT slots of the DLL's kernel32 imports.
const RVA_IAT_GET_CURRENT_THREAD_ID: usize = 0x5_51f0;
const RVA_IAT_ACQUIRE_SRW_EXCLUSIVE: usize = 0x5_5050;
const RVA_IAT_RELEASE_SRW_EXCLUSIVE: usize = 0x5_5048;

const MTX_RECURSIVE: u32 = 0x100;
const THRD_BUSY: u64 = 3;

pub(crate) fn patches(base: usize) -> Vec<DllPatch> {
    BASE.store(base, Ordering::Relaxed);
    vec![
        DllPatch { rva: RVA_MTX_LOCK, len: MTX_LOCK_GUARD_LEN, fnv: 0x7494_bfb5_95cb_014c, target: mtx_lock as *const () as usize },
        DllPatch { rva: RVA_MTX_UNLOCK, len: 0x1f, fnv: 0x9573_00be_cc59_a48e, target: mtx_unlock as *const () as usize },
    ]
}

#[inline(always)]
unsafe fn import(base: usize, slot_rva: usize) -> u64 {
    rd::<u64>(base + slot_rva)
}

/// The loader binds `GetCurrentThreadId` imports of CRT DLLs to a guest leaf stub
/// (`aw2-loader` `guest_leaf_import_stub`: `mov eax, dword ptr gs:[0x48]; ret`), i.e. the
/// TEB's `ClientId.UniqueThread`, which is what [`win32_rt::teb::thread_id`] returns on the
/// same thread.
const TID_STUB: [u8; 9] = [0x65, 0x8B, 0x04, 0x25, 0x48, 0x00, 0x00, 0x00, 0xC3];

/// The `GetCurrentThreadId` IAT value `v` is the builtin or the guest stub, so the thread
/// id can be read natively (no guest call).
fn tid_import_native(v: u64) -> bool {
    use win32_rt::kernel32;
    crate::emu::resolve_host_callable(v) == Some(kernel32::threads::GetCurrentThreadId as *const () as usize)
        || (crate::emu::resolve_host_callable(v).is_none()
            && (0x1_0000..1u64 << 47).contains(&v)
            && unsafe { core::ptr::read_unaligned(v as usize as *const [u8; 9]) } == TID_STUB)
}

/// 0 = not evaluated, 1 = the DLL's `GetCurrentThreadId` import is native-readable, 2 = not.
static TID_NATIVE: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

/// `GetCurrentThreadId()` as the DLL's import would return it.
#[inline(always)]
unsafe fn thread_id(base: usize) -> u32 {
    let mut s = TID_NATIVE.load(Ordering::Relaxed);
    if s == 0 {
        s = if tid_import_native(import(base, RVA_IAT_GET_CURRENT_THREAD_ID)) { 1 } else { 2 };
        TID_NATIVE.store(s, Ordering::Relaxed);
    }
    if s == 1 {
        win32_rt::teb::thread_id()
    } else {
        exe::call_va!(import(base, RVA_IAT_GET_CURRENT_THREAD_ID), () -> u32)
    }
}

/// [`mtx_lock`] / [`mtx_unlock`] qualify for `emu::DF_PLAIN` (called inline from translated
/// guest code, no bridge) when they provably never run guest code: their only callees are
/// the DLL's `GetCurrentThreadId` / `AcquireSRWLockExclusive` / `ReleaseSRWLockExclusive`
/// imports, so the SRW slots must resolve to exactly that win32-rt builtin (itself plain)
/// and the thread-id slot to the builtin or the guest stub (read natively, [`thread_id`]).
/// Evaluated when the dispatch flags are computed (memoized per `exe::va_gen`); the PE
/// loader binds the IAT once and nothing rewrites it afterwards.
pub(crate) fn plain_native(h: usize) -> bool {
    if h != mtx_lock as *const () as usize && h != mtx_unlock as *const () as usize {
        return false;
    }
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return false;
    }
    use win32_rt::sync;
    let srw = [
        (RVA_IAT_ACQUIRE_SRW_EXCLUSIVE, sync::AcquireSRWLockExclusive as *const () as usize),
        (RVA_IAT_RELEASE_SRW_EXCLUSIVE, sync::ReleaseSRWLockExclusive as *const () as usize),
    ];
    let tid_v = unsafe { import(base, RVA_IAT_GET_CURRENT_THREAD_ID) };
    let tid_ok = tid_import_native(tid_v);
    let ok = tid_ok && srw.iter().all(|&(rva, f)| crate::emu::resolve_host_callable(unsafe { import(base, rva) }) == Some(f));
    static LOGGED: std::sync::Once = std::sync::Once::new();
    LOGGED.call_once(|| {
        let show = |v: u64| crate::emu::resolve_host_callable(v).map_or("guest".to_string(), |h| format!("host {h:#x}"));
        let [(ra, fa), (rr, fr)] = srw;
        let (va, vr) = unsafe { (import(base, ra), import(base, rr)) };
        log::info!(
            "dll_native: msvcp140 _Mtx_lock/_Mtx_unlock inline-callable (DF_PLAIN): {ok} \
             (GetCurrentThreadId slot {tid_v:#x} -> {} native={tid_ok}; AcquireSRWLockExclusive slot {va:#x} -> {} want {fa:#x}; \
             ReleaseSRWLockExclusive slot {vr:#x} -> {} want {fr:#x})",
            show(tid_v),
            show(va),
            show(vr)
        );
    });
    ok
}

/// `_Mtx_lock(_Mtx_t)`: original `msvcp140+0x125c0` -> `mtx_do_lock(mtx, nullptr)` at
/// `+0x12690` (the untimed path; `_Mtx_timedlock`/`_Mtx_trylock` keep the original).
///
/// Plain mutexes (`type & ~0x100 == 1`): acquire the SRW lock unless this thread owns it,
/// then bump the count. Others: acquire unless owned, bump the count, record the owner on
/// the first level; a non-recursive re-lock by the owner undoes the bump and returns
/// `_Thrd_busy` (3). Returns 0 (`_Thrd_success`) otherwise.
pub unsafe extern "win64" fn mtx_lock(mtx: u64) -> u64 {
    let base = BASE.load(Ordering::Relaxed);
    let m = mtx as usize;
    let tid = thread_id(base);
    let ty = rd::<u32>(m);
    if ty & !MTX_RECURSIVE == 1 {
        if rd::<u32>(m + 0x48) != tid {
            exe::call_va!(import(base, RVA_IAT_ACQUIRE_SRW_EXCLUSIVE), (mtx + 0x10));
            wr::<u32>(m + 0x48, tid);
        }
        wr::<u32>(m + 0x4c, rd::<u32>(m + 0x4c).wrapping_add(1));
        return 0;
    }
    if rd::<u32>(m + 0x48) != tid {
        exe::call_va!(import(base, RVA_IAT_ACQUIRE_SRW_EXCLUSIVE), (mtx + 0x10));
    }
    let count = rd::<u32>(m + 0x4c);
    let next = count.wrapping_add(1);
    wr::<u32>(m + 0x4c, next);
    if next as i32 <= 1 {
        wr::<u32>(m + 0x48, tid);
        return 0;
    }
    if rd::<u32>(m) & MTX_RECURSIVE != 0 {
        return 0;
    }
    wr::<u32>(m + 0x4c, count);
    THRD_BUSY
}

/// `_Mtx_unlock(_Mtx_t)`: original `msvcp140+0x12630`. Drops one level; on the last one
/// clears the owner (`-1`) and releases the SRW lock. Returns 0.
pub unsafe extern "win64" fn mtx_unlock(mtx: u64) -> u64 {
    let base = BASE.load(Ordering::Relaxed);
    let m = mtx as usize;
    let count = rd::<u32>(m + 0x4c).wrapping_sub(1);
    wr::<u32>(m + 0x4c, count);
    if count == 0 {
        wr::<u32>(m + 0x48, u32::MAX);
        exe::call_va!(import(base, RVA_IAT_RELEASE_SRW_EXCLUSIVE), (mtx + 0x10));
    }
    0
}

// ---------------------------------------------------------------------------
// Difftests (run by the loader's `--diff-test` after the registry entries)
// ---------------------------------------------------------------------------

/// Difftests for the replacements that are installed (empty if the DLL is not mapped or
/// its code did not match).
pub(crate) fn difftests() -> Vec<DllDiff> {
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Vec::new();
    }
    let mut v = Vec::new();
    let installed = |rva: usize, f: usize| exe::lookup_va_only((base + rva) as u64) == Some(f);
    if installed(RVA_MTX_LOCK, mtx_lock as *const () as usize) {
        v.push(DllDiff { name: "msvcp140!_Mtx_lock", addr: base + RVA_MTX_LOCK, new: mtx_lock as *const () as usize, diff: diff_mtx_lock });
    }
    if installed(RVA_MTX_UNLOCK, mtx_unlock as *const () as usize) {
        v.push(DllDiff { name: "msvcp140!_Mtx_unlock", addr: base + RVA_MTX_UNLOCK, new: mtx_unlock as *const () as usize, diff: diff_mtx_unlock });
    }
    v
}

type MtxFn = unsafe extern "win64" fn(u64) -> u64;

/// Mutex types exercised: the standard ones plus values with other bits set (the
/// original only strips bit 8 before comparing with `_Mtx_plain`).
const TYPES: [u32; 9] = [1, 2, 4, 0x101, 0x102, 0x104, 0x201, 0x103, 0];
/// Initial owner fields: none, and a stale id of another thread (count 0, SRW free).
const OWNERS: [u32; 2] = [u32::MAX, 0x1234_5678];

/// Fresh unlocked mutex: `type`, owner, count 0, SRW free; other bytes patterned.
unsafe fn init_mtx(buf: &mut [u64; 12], ty: u32, owner: u32) -> u64 {
    for (i, w) in buf.iter_mut().enumerate() {
        *w = 0xa5a5_0000_0000_0000 | i as u64;
    }
    let m = buf.as_mut_ptr() as usize;
    wr::<u32>(m, ty);
    wr::<u64>(m + 0x10, 0);
    wr::<u32>(m + 0x48, owner);
    wr::<u32>(m + 0x4c, 0);
    m as u64
}

/// Lock twice and unlock twice; `lock` is the function under test, `unlock` restores
/// (the original). Records every return value and the whole 0x60-byte object after each
/// step. A non-recursive second lock by the owner returns busy and leaves the object as
/// is, so the extra unlock then drives the count below zero on both sides alike.
unsafe fn lock_sequence(lock: MtxFn, unlock: MtxFn, ty: u32, owner: u32) -> Vec<(u64, [u64; 12])> {
    let mut buf = Box::new([0u64; 12]);
    let m = init_mtx(&mut *buf, ty, owner);
    let mut out = Vec::new();
    for step in 0..4 {
        let r = if step < 2 { lock(m) } else { unlock(m) };
        out.push((r, core::ptr::read_unaligned(m as usize as *const [u64; 12])));
    }
    // Leave the SRW lock released whatever the sequence did.
    while rd::<u64>(m as usize + 0x10) != 0 && (rd::<u32>(m as usize + 0x4c) as i32) > 0 {
        unlock(m);
    }
    out
}

pub unsafe fn diff_mtx_lock(orig: usize, new: usize) -> Result<u32, String> {
    let base = BASE.load(Ordering::Relaxed);
    let (orig_lock, new_lock): (MtxFn, MtxFn) = (core::mem::transmute(orig), core::mem::transmute(new));
    // Restore with the original `_Mtx_unlock` (guest code) on both sides.
    let unlock: MtxFn = core::mem::transmute((base + RVA_MTX_UNLOCK) | (orig & (1usize << 46)));
    let mut n = 0;
    for &ty in &TYPES {
        for &owner in &OWNERS {
            let o = lock_sequence(orig_lock, unlock, ty, owner);
            let r = lock_sequence(new_lock, unlock, ty, owner);
            if o != r {
                return Err(format!("mismatch: _Mtx_lock type=0x{ty:x} owner=0x{owner:x}: orig={o:x?} new={r:x?}"));
            }
            n += 1;
        }
    }
    Ok(n)
}

pub unsafe fn diff_mtx_unlock(orig: usize, new: usize) -> Result<u32, String> {
    let base = BASE.load(Ordering::Relaxed);
    let (orig_unlock, new_unlock): (MtxFn, MtxFn) = (core::mem::transmute(orig), core::mem::transmute(new));
    // Lock with the original `_Mtx_lock` (guest code) on both sides.
    let lock: MtxFn = core::mem::transmute((base + RVA_MTX_LOCK) | (orig & (1usize << 46)));
    let mut n = 0;
    for &ty in &TYPES {
        for &owner in &OWNERS {
            let o = lock_sequence(lock, orig_unlock, ty, owner);
            let r = lock_sequence(lock, new_unlock, ty, owner);
            if o != r {
                return Err(format!("mismatch: _Mtx_unlock type=0x{ty:x} owner=0x{owner:x}: orig={o:x?} new={r:x?}"));
            }
            n += 1;
        }
    }
    Ok(n)
}
