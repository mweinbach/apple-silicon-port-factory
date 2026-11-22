//! The per-thread EH record chain (target/review/exec/g12/eh_lift_plan.md §4.2): intrusive,
//! stack-allocated records linked through `CallCtx::eh_top`, and the hooks through which
//! win32-rt's dispatcher / unwinder walks them (`win32_rt::unwind` "EH record chain").
//!
//! Records (all start with an [`EhLink`]):
//! - `EH_EMU_ENTRY`: [`crate::emu::eh::EmuEntry`], one per emulator run;
//! - `EH_LIFTED`: [`crate::dll_native::lift::eh::EhFrame`], one per live lifted function with a
//!   FuncInfo (`EhMeta`);
//! - `EH_CROSSING` / `EH_BARRIER`: win32-rt's own (dispatcher crossings, `call_guest8`).
use crate::callctx::ctx;
use win32_rt::unwind::{self, Context, EhHooks, EhLink, LiftedInfo, EH_BARRIER, EH_CROSSING, EH_EMU_ENTRY, EH_HOST_FRAME, EH_LIFTED};
use core::sync::atomic::{AtomicBool, Ordering};

/// Link `rec` as this thread's innermost record.
#[inline(always)]
pub unsafe fn push(rec: *mut EhLink) {
    let c = ctx();
    unsafe { (*rec).next = c.eh_top.get() };
    c.eh_top.set(rec);
}

/// Unlink `rec`, normally the innermost record. A landing restores the head itself (the
/// skipped frames' records are simply abandoned), so a pop always finds its record on top;
/// anything else is a bookkeeping bug, unlinked in place and logged.
#[inline(always)]
pub unsafe fn pop(rec: *mut EhLink) {
    let c = ctx();
    if c.eh_top.get() == rec {
        c.eh_top.set(unsafe { (*rec).next });
    } else {
        unsafe { pop_slow(rec) };
    }
}

#[cold]
#[inline(never)]
unsafe fn pop_slow(rec: *mut EhLink) {
    let c = ctx();
    let mut p = c.eh_top.get();
    unsafe {
        while !p.is_null() {
            if (*p).next == rec {
                log::error!(
                    target: "win32_rt::seh",
                    "EH chain: non-LIFO pop of {} (innermost: {})",
                    describe(rec),
                    describe(c.eh_top.get())
                );
                (*p).next = (*rec).next;
                return;
            }
            p = (*p).next;
        }
        log::error!(target: "win32_rt::seh", "EH chain: pop of {}: not linked", describe(rec));
    }
}

/// Number of records linked on this thread.
pub fn depth() -> usize {
    let mut n = 0;
    let mut p = ctx().eh_top.get();
    while !p.is_null() {
        n += 1;
        p = unsafe { (*p).next };
    }
    n
}

/// What the innermost record says about the frame it owns.
pub enum Innermost {
    /// No records on this thread.
    None,
    /// An emulated region suspended in a host call: the calling guest frame's context.
    Callout(Box<Context>),
    /// An emulated region running guest code (or entered without a host call in flight).
    Emu,
    /// A lifted EH frame.
    Lifted(LiftedInfo),
    /// A dispatcher crossing.
    Crossing,
    /// A Rust API's guest callback (opaque).
    Barrier,
}

impl core::fmt::Debug for Innermost {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Innermost::None => write!(f, "None"),
            Innermost::Callout(c) => write!(f, "Callout(rip=0x{:x}, rsp=0x{:x})", c.rip, c.gpr[4]),
            Innermost::Emu => write!(f, "Emu"),
            Innermost::Lifted(l) => write!(f, "Lifted({l:?})"),
            Innermost::Crossing => write!(f, "Crossing"),
            Innermost::Barrier => write!(f, "Barrier"),
        }
    }
}

/// See [`Innermost`].
pub fn innermost() -> Innermost {
    let r = ctx().eh_top.get();
    if r.is_null() {
        return Innermost::None;
    }
    unsafe {
        match (*r).kind {
            EH_EMU_ENTRY => {
                let mut c = Box::new(Context::zeroed());
                if crate::emu::eh::callout_context(r, &mut c) {
                    Innermost::Callout(c)
                } else {
                    Innermost::Emu
                }
            }
            EH_LIFTED => Innermost::Lifted(crate::dll_native::lift::eh::lifted_info(r)),
            EH_CROSSING => Innermost::Crossing,
            _ => Innermost::Barrier,
        }
    }
}

/// One-line description of a record, for logs.
pub fn describe(r: *mut EhLink) -> String {
    if r.is_null() {
        return "(none)".into();
    }
    unsafe {
        match (*r).kind {
            EH_EMU_ENTRY => crate::emu::eh::describe(r),
            EH_LIFTED => crate::dll_native::lift::eh::describe(r),
            EH_CROSSING => "exception dispatcher crossing".into(),
            EH_BARRIER => "Rust API guest callback barrier".into(),
            EH_HOST_FRAME => "native exception-handling frame".into(),
            k => format!("EH record of unknown kind {k}"),
        }
    }
}

fn hook_head() -> *mut EhLink {
    ctx().eh_top.get()
}

fn hook_set_head(r: *mut EhLink) {
    ctx().eh_top.set(r);
}

unsafe fn hook_callout(r: *mut EhLink, out: &mut Context) -> bool {
    unsafe { crate::emu::eh::callout_context(r, out) }
}

unsafe fn hook_callout_fn(r: *mut EhLink) -> usize {
    unsafe { crate::emu::eh::callout_fn(r) }
}

unsafe fn hook_lifted(r: *mut EhLink) -> LiftedInfo {
    unsafe { crate::dll_native::lift::eh::lifted_info(r) }
}

/// End VA of the guest instruction at `va` (decoded from the image bytes; `va` + 1 if they
/// do not decode, which only a corrupt state IP could give).
unsafe fn hook_insn_end(va: u64) -> u64 {
    let bytes = unsafe { core::slice::from_raw_parts(va as *const u8, 15) };
    let mut dec = iced_x86::Decoder::with_ip(64, bytes, va, iced_x86::DecoderOptions::NONE);
    let insn = dec.decode();
    if insn.is_invalid() {
        va + 1
    } else {
        insn.next_ip()
    }
}

/// Establisher frames outside the TEB stack: this thread's emulator stack, lstack arena and
/// big-frame arena.
pub(crate) fn hook_frame_ok(frame: u64) -> bool {
    let (lo, hi) = crate::emu::eh::stack_range();
    if frame >= lo && frame < hi {
        return true;
    }
    let base = ctx().ls_base.get() as u64;
    if base != 0 && frame >= base && frame < base + crate::lstack::ARENA_BYTES as u64 {
        return true;
    }
    crate::simd_ops::big_frame_arena_if_any().is_some_and(|(lo, hi)| frame >= lo as u64 && frame < hi as u64)
}

unsafe fn hook_land(r: *mut EhLink, c: &Context) -> ! {
    unsafe {
        match (*r).kind {
            EH_EMU_ENTRY => crate::emu::eh::land(r, c),
            EH_LIFTED => crate::dll_native::lift::eh::land(r, c),
            _ => {
                log::error!(target: "win32_rt::seh", "EH landing at 0x{:x} in {}", c.rip, describe(r));
                std::process::abort()
            }
        }
    }
}

static REGISTERED: AtomicBool = AtomicBool::new(false);

/// Register the chain hooks with win32-rt (idempotent). Called before the first record is
/// linked (`ensure_hooks`) and from `exe::set_base`.
pub fn register_hooks() {
    unwind::set_eh_hooks(EhHooks {
        head: hook_head,
        set_head: hook_set_head,
        callout: hook_callout,
        callout_fn: hook_callout_fn,
        lifted: hook_lifted,
        insn_end: hook_insn_end,
        frame_ok: hook_frame_ok,
        land: hook_land,
        describe,
    });
    REGISTERED.store(true, Ordering::Release);
}

/// [`register_hooks`] unless done (one relaxed load on the record push paths).
#[inline(always)]
pub fn ensure_hooks() {
    if !REGISTERED.load(Ordering::Relaxed) {
        register_hooks();
    }
}
