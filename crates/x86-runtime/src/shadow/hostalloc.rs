//! Host allocations inside probe runs. Every registered malloc zone is redirected to a
//! private bump arena (never tracked or compared) and every free is a no-op, so host-internal
//! heap traffic (emulator block decode, Rust temporaries of a replacement) never shows up as a
//! diff and libmalloc metadata stays pristine. Guest heap traffic is separate
//! (`win32_rt::shadow` bump arena, tracked and compared).

use win32_rt::libc;
use std::sync::atomic::{AtomicUsize, Ordering};

static BASE: AtomicUsize = AtomicUsize::new(0);
static END: AtomicUsize = AtomicUsize::new(0);
static CUR: AtomicUsize = AtomicUsize::new(0);
/// Allocations served from the arena (diagnostics, tests).
pub static COUNT: AtomicUsize = AtomicUsize::new(0);

const MAX_ZONES: usize = 16;
/// (zone, original `size`, original `claimed_address`), filled by [`prepare`].
static mut ZONES: [(usize, usize, usize); MAX_ZONES] = [(0, 0, 0); MAX_ZONES];
static NZ: AtomicUsize = AtomicUsize::new(0);

extern "C" {
    fn malloc_get_all_zones(task: u32, reader: usize, addresses: *mut *mut usize, count: *mut u32) -> i32;
    fn malloc_default_zone() -> usize;
    static mach_task_self_: u32;
}

// malloc_zone_t field offsets (malloc/malloc.h).
const F_SIZE: usize = 16;
const F_MALLOC: usize = 24;
const F_CALLOC: usize = 32;
const F_VALLOC: usize = 40;
const F_FREE: usize = 48;
const F_REALLOC: usize = 56;
const F_BATCH_MALLOC: usize = 80;
const F_BATCH_FREE: usize = 88;
const F_VERSION: usize = 104;
const F_MEMALIGN: usize = 112;
const F_FREE_DEFINITE: usize = 120;
const F_CLAIMED: usize = 136;
const F_TRY_FREE_DEFAULT: usize = 144;
const F_WITH_OPTIONS: usize = 152;
const F_T_MALLOC: usize = 160;
const F_T_CALLOC: usize = 168;
const F_T_REALLOC: usize = 176;
const F_T_MEMALIGN: usize = 184;
const F_T_WITH_OPTIONS: usize = 192;
const ZONE_BYTES: usize = 200;

#[inline]
fn in_arena(p: usize) -> bool {
    p >= BASE.load(Ordering::Relaxed) && p < END.load(Ordering::Relaxed)
}

unsafe fn bump(size: usize, align: usize) -> usize {
    let align = align.max(16).next_power_of_two();
    let end_lim = END.load(Ordering::Relaxed);
    let mut cur = CUR.load(Ordering::Relaxed);
    loop {
        let start = (cur + 16 + align - 1) & !(align - 1);
        let end = start + size.max(1);
        if end > end_lim {
            return 0;
        }
        match CUR.compare_exchange_weak(cur, (end + 15) & !15, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_) => {
                *((start - 8) as *mut usize) = size;
                COUNT.fetch_add(1, Ordering::Relaxed);
                return start;
            }
            Err(c) => cur = c,
        }
    }
}

unsafe fn orig_size_of(z: usize, p: usize) -> usize {
    let n = NZ.load(Ordering::Relaxed);
    let zones = &*(&raw const ZONES);
    for &(zz, sz, _) in &zones[..n] {
        if zz == z && sz != 0 {
            let f: extern "C" fn(usize, usize) -> usize = core::mem::transmute(sz);
            return f(zz, p);
        }
    }
    0
}

/// Size of a pre-existing (foreign) block: ask every zone's original `size`.
unsafe fn foreign_size(p: usize) -> usize {
    let n = NZ.load(Ordering::Relaxed);
    let zones = &*(&raw const ZONES);
    for &(zz, sz, _) in &zones[..n] {
        if sz != 0 {
            let f: extern "C" fn(usize, usize) -> usize = core::mem::transmute(sz);
            let s = f(zz, p);
            if s != 0 {
                return s;
            }
        }
    }
    0
}

extern "C" fn z_size(z: usize, p: usize) -> usize {
    unsafe {
        if in_arena(p) {
            *((p - 8) as *const usize)
        } else {
            orig_size_of(z, p)
        }
    }
}
extern "C" fn z_malloc(_z: usize, size: usize) -> usize {
    unsafe { bump(size, 16) }
}
extern "C" fn z_calloc(_z: usize, n: usize, size: usize) -> usize {
    // The arena is fresh anonymous memory and never reused: already zero.
    unsafe { bump(n.saturating_mul(size), 16) }
}
extern "C" fn z_valloc(_z: usize, size: usize) -> usize {
    unsafe { bump(size, super::handler::page()) }
}
extern "C" fn z_free(_z: usize, _p: usize) {}
extern "C" fn z_realloc(_z: usize, p: usize, size: usize) -> usize {
    unsafe {
        let new = bump(size, 16);
        if p != 0 && new != 0 {
            let old = if in_arena(p) { *((p - 8) as *const usize) } else { foreign_size(p) };
            core::ptr::copy_nonoverlapping(p as *const u8, new as *mut u8, old.min(size));
        }
        new
    }
}
extern "C" fn z_batch_malloc(_z: usize, size: usize, results: *mut usize, num: u32) -> u32 {
    unsafe {
        for i in 0..num as usize {
            let p = bump(size, 16);
            if p == 0 {
                return i as u32;
            }
            *results.add(i) = p;
        }
        num
    }
}
extern "C" fn z_batch_free(_z: usize, _ptrs: *mut usize, _num: u32) {}
extern "C" fn z_memalign(_z: usize, align: usize, size: usize) -> usize {
    unsafe { bump(size, align) }
}
extern "C" fn z_free_definite(_z: usize, _p: usize, _size: usize) {}
extern "C" fn z_claimed(z: usize, p: usize) -> i32 {
    if in_arena(p) {
        return 1;
    }
    unsafe {
        let n = NZ.load(Ordering::Relaxed);
        let zones = &*(&raw const ZONES);
        for &(zz, _, cl) in &zones[..n] {
            if zz == z && cl != 0 {
                let f: extern "C" fn(usize, usize) -> i32 = core::mem::transmute(cl);
                return f(zz, p);
            }
        }
    }
    0
}
extern "C" fn z_try_free_default(_z: usize, _p: usize) {}
extern "C" fn z_with_options(_z: usize, align: usize, size: usize, _opts: u64) -> usize {
    unsafe { bump(size, align) }
}
extern "C" fn z_t_malloc(_z: usize, size: usize, _t: u64) -> usize {
    unsafe { bump(size, 16) }
}
extern "C" fn z_t_calloc(_z: usize, n: usize, size: usize, _t: u64) -> usize {
    unsafe { bump(n.saturating_mul(size), 16) }
}
extern "C" fn z_t_realloc(z: usize, p: usize, size: usize, _t: u64) -> usize {
    z_realloc(z, p, size)
}
extern "C" fn z_t_memalign(_z: usize, align: usize, size: usize, _t: u64) -> usize {
    unsafe { bump(size, align) }
}
extern "C" fn z_t_with_options(_z: usize, align: usize, size: usize, _opts: u64, _t: u64) -> usize {
    unsafe { bump(size, align) }
}

/// Orchestrator, before any run: record the zones and the arena. Allocation-free after the
/// zone list call.
pub unsafe fn prepare(arena: usize, len: usize) {
    BASE.store(arena, Ordering::Relaxed);
    END.store(arena + len, Ordering::Relaxed);
    CUR.store(arena, Ordering::Relaxed);
    let mut list: *mut usize = core::ptr::null_mut();
    let mut count: u32 = 0;
    let mut all: Vec<usize> = Vec::with_capacity(MAX_ZONES + 1);
    if malloc_get_all_zones(mach_task_self_, 0, &mut list, &mut count) == 0 && !list.is_null() {
        for i in 0..count as usize {
            all.push(*list.add(i));
        }
    }
    let dz = malloc_default_zone();
    if dz != 0 && !all.contains(&dz) {
        all.push(dz);
    }
    let zones = &mut *(&raw mut ZONES);
    let mut n = 0;
    for z in all {
        if z == 0 || n == MAX_ZONES {
            continue;
        }
        let version = *((z + F_VERSION) as *const u32);
        let size = *((z + F_SIZE) as *const usize);
        let claimed = if version >= 10 { *((z + F_CLAIMED) as *const usize) } else { 0 };
        zones[n] = (z, size, claimed);
        n += 1;
    }
    NZ.store(n, Ordering::Relaxed);
}

/// Run child, before tracking starts: redirect every zone to the arena.
pub unsafe fn enable() {
    let pg = super::handler::page();
    let n = NZ.load(Ordering::Relaxed);
    let zones = &*(&raw const ZONES);
    for &(z, _, _) in &zones[..n] {
        let lo = z & !(pg - 1);
        let hi = (z + ZONE_BYTES + pg - 1) & !(pg - 1);
        if libc::mprotect(lo as *mut libc::c_void, hi - lo, libc::PROT_READ | libc::PROT_WRITE) != 0 {
            continue;
        }
        let version = *((z + F_VERSION) as *const u32);
        let set = |off: usize, f: usize| *((z + off) as *mut usize) = f;
        set(F_SIZE, z_size as *const () as usize);
        set(F_MALLOC, z_malloc as *const () as usize);
        set(F_CALLOC, z_calloc as *const () as usize);
        set(F_VALLOC, z_valloc as *const () as usize);
        set(F_FREE, z_free as *const () as usize);
        set(F_REALLOC, z_realloc as *const () as usize);
        set(F_BATCH_MALLOC, z_batch_malloc as *const () as usize);
        set(F_BATCH_FREE, z_batch_free as *const () as usize);
        if version >= 5 {
            set(F_MEMALIGN, z_memalign as *const () as usize);
        }
        if version >= 6 {
            set(F_FREE_DEFINITE, z_free_definite as *const () as usize);
        }
        if version >= 10 {
            set(F_CLAIMED, z_claimed as *const () as usize);
        }
        if version >= 13 {
            set(F_TRY_FREE_DEFAULT, z_try_free_default as *const () as usize);
        }
        if version >= 15 {
            set(F_WITH_OPTIONS, z_with_options as *const () as usize);
        }
        if version >= 16 {
            set(F_T_MALLOC, z_t_malloc as *const () as usize);
            set(F_T_CALLOC, z_t_calloc as *const () as usize);
            set(F_T_REALLOC, z_t_realloc as *const () as usize);
            set(F_T_MEMALIGN, z_t_memalign as *const () as usize);
            set(F_T_WITH_OPTIONS, z_t_with_options as *const () as usize);
        }
    }
}
