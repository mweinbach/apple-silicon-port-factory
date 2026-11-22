//! Per-thread guest-call context (forwarded RAX/XMM arguments, XMM return, guest RSP windows).
//!
//! Every lifted call goes through `call_va8` -> `call_host_for_va`, which touched six separate
//! `thread_local!` cells about a dozen times per call. On macOS each `thread_local!` access is a
//! `_tlv_get_addr` call, and gameplay profiles showed that cost (plus the surrounding dispatch)
//! as ~35-55% of the busy time of the main, renderer and culling threads.
//!
//! All of that state now lives in one `CallCtx` per thread. On aarch64 Darwin the pointer is kept
//! in a pthread TSD slot and read inline (`mrs TPIDRRO_EL0` + one load), verified once against
//! `pthread_getspecific`; other targets use a single `thread_local!`.
use core::cell::{Cell, UnsafeCell};

/// One slot of the per-thread dispatch cache (see [`crate::emu::resolve_dispatch`]).
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DispatchSlot {
    pub va: u64,
    pub host_fn: usize,
    /// `crate::emu::DF_*` flags.
    pub flags: u32,
    /// `crate::exe::va_gen()` the slot was filled under (0 = empty).
    pub gen: u32,
}

#[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
impl DispatchSlot {
    const EMPTY: Self = Self { va: 0, host_fn: 0, flags: 0, gen: 0 };
}

pub const DCACHE_BITS: u32 = 11;
pub const DCACHE_LEN: usize = 1 << DCACHE_BITS;

#[repr(C, align(16))]
pub struct CallCtx {
    pub xmm_in: Cell<[u128; 4]>,
    pub xmm_out: Cell<Option<u128>>,
    pub rax_in: Cell<u64>,
    pub incoming_rsp: Cell<(u64, u64)>,
    pub outgoing_rsp: Cell<(u64, u64)>,
    pub xmm_in_set: Cell<bool>,
    /// This thread's emulated GS block (`win32_rt::teb::gs_slots_ptr`), null until first use.
    pub gs: Cell<*const usize>,
    /// Clean lifted-stack arena (`crate::lstack`): current top, 0 = not set up (or disabled).
    pub ls_top: Cell<usize>,
    /// Lowest address a frame may reach (arena base + margin); 0 = never set up.
    pub ls_lo: Cell<usize>,
    /// Arena allocation base, released when the `CallCtx` is dropped; 0 = none.
    pub ls_base: Cell<usize>,
    /// `crate::lstack::LS_*` flags; non-zero sends `LiftedFrame::drop` to the slow path.
    pub ls_flags: Cell<u32>,
    /// Arena top at the innermost guest -> host transition (`emu::host_call`), 0 = none (the
    /// arena's initial top). Lifted bytes below it are charged to the guest stack
    /// (`crate::lstack` "Stack account").
    pub ls_anchor: Cell<usize>,
    /// Host `sp` below which a lifted frame entry raises `STATUS_STACK_OVERFLOW`
    /// (host thread stack low end + margin); 0 = no check.
    pub ls_hlo: Cell<usize>,
    /// This thread's emulator stack `(floor, top)`: the lowest guest `rsp` a lifted budget or a
    /// nested region start may reach, and the stack top; (0, 0) = no emulator state yet.
    pub ls_gstack: Cell<(u64, u64)>,
    /// Innermost record of this thread's EH chain (`win32_rt::unwind::EhLink`), null = none.
    /// Emulated regions, lifted EH frames and dispatcher crossings push/pop records so the
    /// unwinder can walk guest frames that are not on the host stack (`crate::eh_rt`).
    pub eh_top: Cell<*mut win32_rt::unwind::EhLink>,
    /// Direct-mapped VA -> (host fn, ABI flags) cache. Thread-private, so no atomics.
    pub dcache: UnsafeCell<[DispatchSlot; DCACHE_LEN]>,
}

impl Drop for CallCtx {
    fn drop(&mut self) {
        crate::lstack::release_arena(self);
    }
}

#[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
impl CallCtx {
    const fn new() -> Self {
        Self {
            xmm_in: Cell::new([0; 4]),
            xmm_out: Cell::new(None),
            rax_in: Cell::new(0),
            incoming_rsp: Cell::new((0, 0)),
            outgoing_rsp: Cell::new((0, 0)),
            xmm_in_set: Cell::new(false),
            gs: Cell::new(core::ptr::null()),
            ls_top: Cell::new(0),
            ls_lo: Cell::new(0),
            ls_base: Cell::new(0),
            ls_flags: Cell::new(0),
            ls_anchor: Cell::new(0),
            ls_hlo: Cell::new(0),
            ls_gstack: Cell::new((0, 0)),
            eh_top: Cell::new(core::ptr::null_mut()),
            dcache: UnsafeCell::new([DispatchSlot::EMPTY; DCACHE_LEN]),
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
mod imp {
    use super::CallCtx;
    use core::cell::Cell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    unsafe extern "C" {
        fn pthread_key_create(key: *mut usize, dtor: Option<unsafe extern "C" fn(*mut u8)>) -> i32;
        fn pthread_getspecific(key: usize) -> *mut u8;
        fn pthread_setspecific(key: usize, value: *const u8) -> i32;
    }

    /// pthread key + 1 (0 = not created yet).
    static KEY_P1: AtomicUsize = AtomicUsize::new(0);
    /// 1 = inline TSD reads verified to match `pthread_getspecific`; 2 = use the libc call.
    static DIRECT: AtomicUsize = AtomicUsize::new(0);
    /// `KEY_P1` once inline reads are verified (`DIRECT == 1`), else 0: the fast path's only load.
    static FAST_KEY_P1: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" fn dtor(p: *mut u8) {
        if !p.is_null() {
            drop(unsafe { Box::from_raw(p as *mut CallCtx) });
        }
    }

    #[inline(always)]
    unsafe fn tsd_get(key: usize) -> *mut u8 {
        let base: usize;
        unsafe {
            core::arch::asm!("mrs {b}, TPIDRRO_EL0", b = out(reg) base, options(nomem, nostack, preserves_flags, pure));
            *((base & !7) as *const *mut u8).add(key)
        }
    }

    #[inline(always)]
    pub fn ctx() -> &'static CallCtx {
        let fk = FAST_KEY_P1.load(Ordering::Relaxed);
        if fk != 0 {
            let p = unsafe { tsd_get(fk - 1) };
            if !p.is_null() {
                return unsafe { &*(p as *const CallCtx) };
            }
        }
        ctx_slow()
    }

    /// Heap-allocate a fresh `CallCtx` without building it on the stack first.
    fn alloc_ctx() -> *mut CallCtx {
        let mut b = Box::<CallCtx>::new_uninit();
        let p = b.as_mut_ptr();
        unsafe {
            core::ptr::addr_of_mut!((*p).xmm_in).write(Cell::new([0; 4]));
            core::ptr::addr_of_mut!((*p).xmm_out).write(Cell::new(None));
            core::ptr::addr_of_mut!((*p).rax_in).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).incoming_rsp).write(Cell::new((0, 0)));
            core::ptr::addr_of_mut!((*p).outgoing_rsp).write(Cell::new((0, 0)));
            core::ptr::addr_of_mut!((*p).xmm_in_set).write(Cell::new(false));
            core::ptr::addr_of_mut!((*p).gs).write(Cell::new(core::ptr::null()));
            core::ptr::addr_of_mut!((*p).ls_top).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_lo).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_base).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_flags).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_anchor).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_hlo).write(Cell::new(0));
            core::ptr::addr_of_mut!((*p).ls_gstack).write(Cell::new((0, 0)));
            core::ptr::addr_of_mut!((*p).eh_top).write(Cell::new(core::ptr::null_mut()));
            // All-zero `DispatchSlot`s are empty (gen 0).
            core::ptr::addr_of_mut!((*p).dcache).write_bytes(0, 1);
            Box::into_raw(b.assume_init())
        }
    }

    #[cold]
    #[inline(never)]
    fn ctx_slow() -> &'static CallCtx {
        let mut kp1 = KEY_P1.load(Ordering::Acquire);
        if kp1 == 0 {
            let mut k = 0usize;
            let rc = unsafe { pthread_key_create(&mut k, Some(dtor)) };
            assert_eq!(rc, 0, "pthread_key_create failed for CallCtx");
            match KEY_P1.compare_exchange(0, k + 1, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => kp1 = k + 1,
                Err(existing) => kp1 = existing, // lost the race; leak the spare key
            }
        }
        let key = kp1 - 1;
        let mut p = unsafe { pthread_getspecific(key) };
        if p.is_null() {
            p = alloc_ctx() as *mut u8;
            unsafe { pthread_setspecific(key, p) };
        }
        if DIRECT.load(Ordering::Relaxed) == 0 {
            let ok = unsafe { tsd_get(key) } == p;
            DIRECT.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
            if ok {
                FAST_KEY_P1.store(kp1, Ordering::Relaxed);
            } else {
                log::warn!("callctx: inline TSD read mismatch; using pthread_getspecific");
            }
        }
        unsafe { &*(p as *const CallCtx) }
    }
}

#[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
mod imp {
    use super::CallCtx;
    std::thread_local! {
        static CTX: CallCtx = const { CallCtx::new() };
    }
    #[inline(always)]
    pub fn ctx() -> &'static CallCtx {
        // The thread-local outlives every use on its own thread.
        CTX.with(|c| unsafe { &*(c as *const CallCtx) })
    }
}

pub use imp::ctx;
