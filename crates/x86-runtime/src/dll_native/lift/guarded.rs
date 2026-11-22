//! Guarded guest memory accesses: loads / stores that report a hardware fault instead of
//! taking the process down (target/review/exec/g18/seh_guard_plan.md).
//!
//! The accesses of an instruction inside an SEH `__except` scope of a lifted fn (dll_lift
//! `seh_scopes` `seh_guards`, gen_subsystems `seh_guard_lines`) go through
//! [`crate::dll_native::lift::eh::EhFrame`]'s `ru*` / `wu*`, which use these probes: on a
//! fault the frame raises STATUS_ACCESS_VIOLATION at the instruction, and the `__except` body
//! runs as in the guest.
//!
//! aarch64: each probe is a leaf asm function in the range [`aw2_probe_begin`,
//! `aw2_probe_end`); it sets x0 = 0 before its one access and returns. A SIGSEGV / SIGBUS at a
//! pc in that range is resumed by [`fixup_darwin_ss`] (aw2-loader `signals::handler`, and the
//! lib tests' handler) at the return address with x0 = 1. Ordering is that of
//! `simd_ops::ru*` / `wu*` (TSO): naturally aligned -> ldapr / stlr, otherwise plain.
//! Other targets (x86_64 build): no fixup, the probes are the plain accesses (a fault stays
//! fatal there, as before).

/// Result of a load probe (x0, x1).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Probe {
    /// 0 = loaded, 1 = faulted.
    pub fault: u64,
    pub v: u64,
}

#[cfg(target_arch = "aarch64")]
extern "C" {
    fn aw2_probe_ld8(a: u64) -> Probe;
    fn aw2_probe_ld16(a: u64) -> Probe;
    fn aw2_probe_ld32(a: u64) -> Probe;
    fn aw2_probe_ld64(a: u64) -> Probe;
    fn aw2_probe_st8(a: u64, v: u64) -> u64;
    fn aw2_probe_st16(a: u64, v: u64) -> u64;
    fn aw2_probe_st32(a: u64, v: u64) -> u64;
    fn aw2_probe_st64(a: u64, v: u64) -> u64;
    static aw2_probe_begin: u8;
    static aw2_probe_end: u8;
}

// Every instruction that may fault is in [begin, end); each probe zeroes x0 before it, so the
// fixup only sets x0 = 1 and returns to lr (leaf functions: lr is the caller's).
#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".text",
    // ldapr* is FEAT_LRCPC (every Apple silicon core has it); global_asm is assembled
    // without the crate's target features, so enable it explicitly.
    ".arch_extension rcpc",
    ".p2align 2",
    ".globl _aw2_probe_begin",
    ".private_extern _aw2_probe_begin",
    "_aw2_probe_begin:",
    // loads: x0 = address -> x0 = fault, x1 = value
    ".globl _aw2_probe_ld8",
    ".private_extern _aw2_probe_ld8",
    "_aw2_probe_ld8:",
    "mov x2, x0",
    "mov x0, #0",
    "ldaprb w1, [x2]",
    "ret",
    ".globl _aw2_probe_ld16",
    ".private_extern _aw2_probe_ld16",
    "_aw2_probe_ld16:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #1",
    "b.ne 1f",
    "ldaprh w1, [x2]",
    "ret",
    "1: ldrh w1, [x2]",
    "ret",
    ".globl _aw2_probe_ld32",
    ".private_extern _aw2_probe_ld32",
    "_aw2_probe_ld32:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #3",
    "b.ne 1f",
    "ldapr w1, [x2]",
    "ret",
    "1: ldr w1, [x2]",
    "ret",
    ".globl _aw2_probe_ld64",
    ".private_extern _aw2_probe_ld64",
    "_aw2_probe_ld64:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #7",
    "b.ne 1f",
    "ldapr x1, [x2]",
    "ret",
    "1: ldr x1, [x2]",
    "ret",
    // stores: x0 = address, x1 = value -> x0 = fault
    ".globl _aw2_probe_st8",
    ".private_extern _aw2_probe_st8",
    "_aw2_probe_st8:",
    "mov x2, x0",
    "mov x0, #0",
    "stlrb w1, [x2]",
    "ret",
    ".globl _aw2_probe_st16",
    ".private_extern _aw2_probe_st16",
    "_aw2_probe_st16:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #1",
    "b.ne 1f",
    "stlrh w1, [x2]",
    "ret",
    "1: strh w1, [x2]",
    "ret",
    ".globl _aw2_probe_st32",
    ".private_extern _aw2_probe_st32",
    "_aw2_probe_st32:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #3",
    "b.ne 1f",
    "stlr w1, [x2]",
    "ret",
    "1: str w1, [x2]",
    "ret",
    ".globl _aw2_probe_st64",
    ".private_extern _aw2_probe_st64",
    "_aw2_probe_st64:",
    "mov x2, x0",
    "mov x0, #0",
    "tst x2, #7",
    "b.ne 1f",
    "stlr x1, [x2]",
    "ret",
    "1: str x1, [x2]",
    "ret",
    ".globl _aw2_probe_end",
    ".private_extern _aw2_probe_end",
    "_aw2_probe_end:",
    "brk #0xf00d",
);

macro_rules! probe {
    ($ld:ident, $st:ident, $pld:ident, $pst:ident, $plain_rd:ident, $plain_wr:ident, $t:ty) => {
        /// Guarded load: `None` = the access faulted.
        #[inline(always)]
        pub unsafe fn $ld(a: u64) -> Option<$t> {
            #[cfg(target_arch = "aarch64")]
            {
                let p = unsafe { $pld(a) };
                if p.fault == 0 {
                    Some(p.v as $t)
                } else {
                    None
                }
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                Some(unsafe { crate::simd_ops::$plain_rd(a) })
            }
        }
        /// Guarded store: `false` = the access faulted (nothing was written).
        #[inline(always)]
        pub unsafe fn $st(a: u64, v: $t) -> bool {
            #[cfg(target_arch = "aarch64")]
            {
                unsafe { $pst(a, v as u64) == 0 }
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                unsafe { crate::simd_ops::$plain_wr(a, v) };
                true
            }
        }
    };
}
probe!(ld8, st8, aw2_probe_ld8, aw2_probe_st8, ru8, wu8, u8);
probe!(ld16, st16, aw2_probe_ld16, aw2_probe_st16, ru16, wu16, u16);
probe!(ld32, st32, aw2_probe_ld32, aw2_probe_st32, ru32, wu32, u32);
probe!(ld64, st64, aw2_probe_ld64, aw2_probe_st64, ru64, wu64, u64);

/// [`ld8`]..[`ld64`] by width in bytes (zero-extended).
pub unsafe fn load(bytes: u8, a: u64) -> Option<u64> {
    unsafe {
        match bytes {
            1 => ld8(a).map(u64::from),
            2 => ld16(a).map(u64::from),
            4 => ld32(a).map(u64::from),
            _ => ld64(a),
        }
    }
}

/// [`st8`]..[`st64`] by width in bytes (`v` truncated).
pub unsafe fn store(bytes: u8, a: u64, v: u64) -> bool {
    unsafe {
        match bytes {
            1 => st8(a, v as u8),
            2 => st16(a, v as u16),
            4 => st32(a, v as u32),
            _ => st64(a, v),
        }
    }
}

/// Whether host pc `pc` is a probe's access (async-signal-safe).
#[inline]
pub fn is_probe_pc(pc: u64) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        let b = core::ptr::addr_of!(aw2_probe_begin) as u64;
        let e = core::ptr::addr_of!(aw2_probe_end) as u64;
        (b..e).contains(&pc)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = pc;
        false
    }
}

/// SIGSEGV / SIGBUS handler step (async-signal-safe): when the faulting pc of Darwin arm64
/// thread state `ss` (`__darwin_arm_thread_state64`: x0..x28, fp, lr, sp, pc) is a probe's
/// access, resume at its return address with x0 = 1 (faulted) and return true.
///
/// # Safety
/// `ss` points at the thread state of a signal's `mcontext` (`uc_mcontext + 16`).
pub unsafe fn fixup_darwin_ss(ss: *mut u64) -> bool {
    unsafe {
        let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
        if !is_probe_pc(pc) {
            return false;
        }
        *ss.add(0) = 1;
        *ss.add(32) = *ss.add(30);
        true
    }
}

/// Lib tests: a SIGSEGV / SIGBUS handler doing [`fixup_darwin_ss`]; any other fault restores
/// the previous action and returns (the access faults again under it).
#[cfg(all(test, target_arch = "aarch64", target_os = "macos"))]
pub(crate) fn install_test_fault_handler() {
    use win32_rt::libc;
    use core::sync::atomic::{AtomicPtr, Ordering};
    use std::sync::Once;
    static ONCE: Once = Once::new();
    static PREV: [AtomicPtr<libc::sigaction>; 2] = [AtomicPtr::new(core::ptr::null_mut()), AtomicPtr::new(core::ptr::null_mut())];
    extern "C" fn handler(sig: libc::c_int, _info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
        unsafe {
            let mc = *((uc as *const u8).add(48) as *const *mut u8); // ucontext_t::uc_mcontext
            if fixup_darwin_ss(mc.add(16) as *mut u64) {
                return;
            }
            let prev = PREV[if sig == libc::SIGSEGV { 0 } else { 1 }].load(Ordering::Acquire);
            libc::sigaction(sig, prev, core::ptr::null_mut());
        }
    }
    ONCE.call_once(|| unsafe {
        for (k, sig) in [libc::SIGSEGV, libc::SIGBUS].into_iter().enumerate() {
            let mut sa: libc::sigaction = core::mem::zeroed();
            sa.sa_sigaction = handler as *const () as usize;
            sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
            libc::sigemptyset(&mut sa.sa_mask);
            let prev: *mut libc::sigaction = Box::leak(Box::new(core::mem::zeroed()));
            assert_eq!(libc::sigaction(sig, &sa, prev), 0);
            PREV[k].store(prev, Ordering::Release);
        }
    });
}

#[cfg(all(test, target_arch = "aarch64", target_os = "macos"))]
mod tests {
    use super::*;

    /// Unmapped (macOS `__PAGEZERO`).
    const BAD: u64 = 0x10;

    #[test]
    fn probes_load_and_store_mapped_memory() {
        let mut buf = [0u8; 32];
        let b = buf.as_mut_ptr() as u64;
        unsafe {
            assert!(st64(b + 8, 0x1122_3344_5566_7788));
            assert_eq!(ld64(b + 8), Some(0x1122_3344_5566_7788));
            assert_eq!(ld32(b + 8), Some(0x5566_7788));
            assert_eq!(ld16(b + 10), Some(0x5566));
            assert_eq!(ld8(b + 15), Some(0x11));
            // unaligned
            assert!(st64(b + 3, 0x0102_0304_0506_0708));
            assert_eq!(ld64(b + 3), Some(0x0102_0304_0506_0708));
            assert!(st32(b + 17, 0xAABB_CCDD));
            assert_eq!(ld32(b + 17), Some(0xAABB_CCDD));
            assert!(st16(b + 21, 0xEEFF));
            assert_eq!(ld16(b + 21), Some(0xEEFF));
            assert!(st8(b + 24, 0x5A));
            assert_eq!(load(1, b + 24), Some(0x5A));
            assert!(store(4, b + 24, 0x1_0000_0001));
            assert_eq!(load(8, b + 24), Some(0x0000_0000_0000_0001));
        }
    }

    #[test]
    fn probes_report_a_fault_and_continue() {
        install_test_fault_handler();
        unsafe {
            for a in [BAD, BAD + 1, BAD + 3] {
                assert_eq!(ld8(a), None);
                assert_eq!(ld16(a), None);
                assert_eq!(ld32(a), None);
                assert_eq!(ld64(a), None);
                assert!(!st8(a, 1));
                assert!(!st16(a, 1));
                assert!(!st32(a, 1));
                assert!(!st64(a, 1));
            }
            // A read-only page: loads succeed, stores fault.
            let ro = win32_rt::libc::mmap(
                core::ptr::null_mut(),
                0x4000,
                win32_rt::libc::PROT_READ,
                win32_rt::libc::MAP_PRIVATE | win32_rt::libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(ro, win32_rt::libc::MAP_FAILED);
            let r = ro as u64;
            assert_eq!(ld64(r + 8), Some(0));
            assert!(!st64(r + 8, 7));
            assert!(!st32(r + 1, 7));
            assert_eq!(ld64(r + 8), Some(0), "a faulted store writes nothing");
            win32_rt::libc::munmap(ro, 0x4000);
        }
        assert!(!is_probe_pc(probes_report_a_fault_and_continue as *const () as u64));
    }
}
