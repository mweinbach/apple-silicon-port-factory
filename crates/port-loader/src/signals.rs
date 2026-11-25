//! Hardware exceptions: SIGSEGV/SIGBUS/SIGILL/SIGFPE/SIGTRAP -> Windows SEH.
//!
//! The handler runs on a per-thread `sigaltstack`. It converts the x86-64
//! `ucontext` (as presented by Rosetta) into an `EXCEPTION_RECORD` + `CONTEXT`,
//! copies both onto the faulting thread's own stack and rewrites the signal
//! context so that, when the handler returns, the thread "calls"
//! `win32_rt::unwind::dispatch_hardware_exception(rec, ctx)` — which runs
//! vectored + frame-based handlers on the normal stack and never returns
//! (it restores the possibly-modified CONTEXT). Wine uses the same trick.
//!
//! `int 0x29` (`__fastfail`) is recognised and terminates immediately, like
//! Windows (fast-fail bypasses all handlers).

use win32_rt::unwind::{self, Context, ExceptionRecord};
use std::sync::atomic::{AtomicBool, Ordering};

const ALT_STACK: usize = 2 * 1024 * 1024;

// Darwin x86_64 ucontext / mcontext offsets.
const UC_MCONTEXT: usize = 48;
const MC_ES: usize = 0; // trapno u16, cpu u16, err u32, faultvaddr u64
const MC_SS: usize = 16; // 21 x u64

// __ss register order.
const SS_RCX: usize = 2;
const SS_RDX: usize = 3;
const SS_RSP: usize = 7;
const SS_RIP: usize = 16;
const SS_RFLAGS: usize = 17;

static INSTALLED: AtomicBool = AtomicBool::new(false);

/// Give the calling thread an alternate signal stack.
pub fn install_altstack() {
    unsafe {
        let mem = libc::mmap(
            std::ptr::null_mut(),
            ALT_STACK,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if mem == libc::MAP_FAILED {
            log::error!("sigaltstack allocation failed");
            return;
        }
        let ss = libc::stack_t { ss_sp: mem, ss_size: ALT_STACK, ss_flags: 0 };
        libc::sigaltstack(&ss, std::ptr::null_mut());
    }
}

pub fn install_handlers() {
    if INSTALLED.swap(true, Ordering::AcqRel) {
        return;
    }
    install_altstack();
    let _ = win32_rt::thread::THREAD_START_HOOK.set(install_altstack);
    // SIGUSR2: per-thread snapshots for crash reports.
    win32_rt::crash::install();
    unsafe {
        for sig in [libc::SIGSEGV, libc::SIGBUS, libc::SIGILL, libc::SIGFPE, libc::SIGTRAP] {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = handler as *const () as usize;
            sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK | libc::SA_NODEFER;
            libc::sigemptyset(&mut sa.sa_mask);
            libc::sigaction(sig, &sa, std::ptr::null_mut());
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        // LSE2 self-patching rewrites faulting instructions inside MAP_JIT code pages;
        // only enable it when dynamic JIT pages are writable (never on locked-down iOS).
        if x86_runtime::emu::exec_mem::jit_writable() {
            x86_runtime::emu::enable_lse2_trap();
        }
        x86_runtime::emu::enable_hw_range();
    }
}

unsafe fn rd<T: Copy>(p: *const u8, o: usize) -> T {
    std::ptr::read_unaligned(p.add(o) as *const T)
}

fn in_guest(addr: usize) -> bool {
    crate::loader::get().module_at(addr).is_some_and(|m| !m.is_builtin())
}

unsafe fn fatal(msg: &str, mc: *mut u8) -> ! {
    let ctx = win32_rt::crash::context_from_host_mcontext(mc);
    let l = crate::loader::get();
    eprintln!("FATAL: {msg} at {}", l.symbolize_addr(ctx.rip as usize));
    log::error!("FATAL: {msg} at {}", l.symbolize_addr(ctx.rip as usize));
    win32_rt::crash::report(msg, None, &ctx);
    crate::thunks::exit_hold();
    win32_rt::process::record_exit_code(3);
    win32_rt::process::run_exit_hooks();
    libc::_exit(3)
}

#[cfg(target_arch = "x86_64")]
extern "C" fn handler(sig: libc::c_int, info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    unsafe {
        let mc = *((uc as *const u8).add(UC_MCONTEXT) as *const *mut u8);
        let ss = mc.add(MC_SS) as *mut u64;
        let rip = *ss.add(SS_RIP) as usize;
        let fault_addr: u64 = rd(mc, MC_ES + 8);
        let err: u32 = rd(mc, MC_ES + 4);

        let mut rec: ExceptionRecord = std::mem::zeroed();
        rec.address = rip as *mut _;
        let mut ctx_rip = rip as u64;

        let rip_in_guest = in_guest(rip);
        let bytes: [u8; 2] = if rip_in_guest { [*(rip as *const u8), *((rip + 1) as *const u8)] } else { [0, 0] };

        match sig {
            libc::SIGSEGV | libc::SIGBUS | libc::SIGILL => {
                if bytes == [0xCD, 0x29] {
                    let code = *ss.add(SS_RCX);
                    let l = crate::loader::get();
                    eprintln!("FAST FAIL (int 0x29) code {code} at {}", l.symbolize_addr(rip));
                    log::error!("FAST FAIL (__fastfail({code})) at {} - terminating (bypasses SEH)", l.symbolize_addr(rip));
                    fatal("__fastfail", mc);
                }
                if sig == libc::SIGILL || bytes == [0x0F, 0x0B] {
                    rec.code = unwind::STATUS_ILLEGAL_INSTRUCTION;
                } else {
                    let is_write = (err & 2) != 0;
                    if win32_rt::kernel32::memory::try_handle_write_watch_fault(fault_addr as usize, is_write) {
                        return;
                    }
                    let is_guard = win32_rt::kernel32::memory::consume_guard_page(fault_addr as usize).is_some();
                    rec.code = if is_guard {
                        unwind::STATUS_GUARD_PAGE_VIOLATION
                    } else {
                        unwind::STATUS_ACCESS_VIOLATION
                    };
                    rec.number_parameters = 2;
                    let kind = if fault_addr as usize == rip {
                        8
                    } else if is_write {
                        1
                    } else {
                        0
                    };
                    rec.information[0] = kind;
                    rec.information[1] = fault_addr;
                    // Fault on a Windows thread's stack guard -> stack overflow.
                    if !is_guard {
                        let t = win32_rt::teb::current_raw();
                        if !t.is_null() {
                            let lo: usize = win32_rt::teb::field(t, win32_rt::teb::off::STACK_LIMIT);
                            if (fault_addr as usize) < lo && (fault_addr as usize) + 0x10000 >= lo {
                                fatal("stack overflow (STATUS_STACK_OVERFLOW)", mc);
                            }
                        }
                    }
                }
            }
            libc::SIGFPE => {
                let code = (*info).si_code;
                rec.code = match code {
                    7 /* FPE_INTDIV */ => unwind::STATUS_INTEGER_DIVIDE_BY_ZERO,
                    8 /* FPE_INTOVF */ => 0xC000_0095,
                    1 /* FPE_FLTDIV */ => 0xC000_008E,
                    2 /* FPE_FLTOVF */ => 0xC000_0091,
                    3 /* FPE_FLTUND */ => 0xC000_0093,
                    4 /* FPE_FLTRES */ => 0xC000_008F,
                    5 /* FPE_FLTINV */ => 0xC000_0090,
                    _ => 0xC000_0090,
                };
            }
            libc::SIGTRAP => {
                // int3: rip points after the 0xCC; Windows reports the int3 address.
                if rip > 0 && *((rip - 1) as *const u8) == 0xCC {
                    ctx_rip = rip as u64 - 1;
                    rec.address = (rip - 1) as *mut _;
                }
                rec.code = unwind::STATUS_BREAKPOINT;
                rec.number_parameters = 1;
            }
            _ => fatal("unexpected signal", mc),
        }

        // CONTEXT from the signal frame.
        let mut ctx = win32_rt::crash::context_from_mcontext(mc);
        ctx.rip = ctx_rip;

        if !rip_in_guest {
            let mut probe_ctx = Context::zeroed();
            if !unwind::cross_host_frames(&ctx, &mut probe_ctx) {
                let what = format!(
                    "host fault: {} (signal {sig}, addr {fault_addr:#x})",
                    unwind::describe_exception(&rec)
                );
                fatal(&what, mc);
            }
        }

        // Build the frame on the faulting stack (below a 128-byte gap).
        let mut sp = (ctx.gpr[unwind::RSP] as usize - 128) & !63;
        sp -= std::mem::size_of::<Context>();
        sp &= !63;
        let ctx_p = sp as *mut Context;
        std::ptr::write(ctx_p, ctx);
        sp -= std::mem::size_of::<ExceptionRecord>();
        sp &= !15;
        let rec_p = sp as *mut ExceptionRecord;
        std::ptr::write(rec_p, rec);
        // Shadow space + fake return address; rsp % 16 == 8 at function entry.
        sp -= 0x20;
        sp &= !15;
        sp -= 8;
        *(sp as *mut u64) = 0;
        *ss.add(SS_RCX) = rec_p as u64;
        *ss.add(SS_RDX) = ctx_p as u64;
        *ss.add(SS_RSP) = sp as u64;
        *ss.add(SS_RIP) = unwind::dispatch_hardware_exception as *const () as usize as u64;
        // Clear the direction and trap flags for the dispatcher.
        *ss.add(SS_RFLAGS) &= !(0x400 | 0x100);
    }
}

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
struct BridgeFrame {
    gpr: [u64; 8],
    xmm: [u128; 4],
}

#[cfg(target_arch = "aarch64")]
extern "C" {
    fn aarch64_bridge_trampoline();
}

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".global _aarch64_bridge_trampoline",
    ".p2align 2",
    "_aarch64_bridge_trampoline:",
    "sub sp, sp, #0x90",
    "stp x29, x30, [sp, #0x80]",
    "add x29, sp, #0x80",
    "stp x0, x1, [sp, #0x00]",
    "stp x2, x3, [sp, #0x10]",
    "stp x4, x5, [sp, #0x20]",
    "stp x6, x7, [sp, #0x30]",
    "stp q0, q1, [sp, #0x40]",
    "stp q2, q3, [sp, #0x60]",
    "mov x0, x16",
    "mov x1, sp",
    "add x2, sp, #0x90",
    "bl _aarch64_bridge_dispatch",
    "ldr x0, [sp, #0x00]",
    "ldr q0, [sp, #0x40]",
    "ldp x29, x30, [sp, #0x80]",
    "add sp, sp, #0x90",
    "ret",
);

#[cfg(target_arch = "aarch64")]
#[no_mangle]
unsafe extern "C" fn aarch64_bridge_dispatch(
    raw_pc: u64,
    frame: *mut BridgeFrame,
    caller_sp: *const u64,
) {
    let orig_tag = crate::hooks::ORIG_TRAMPOLINE_TAG as u64;
    let is_orig = (raw_pc & orig_tag) != 0;
    let target_va = raw_pc & !orig_tag;
    let gpr_args = (*frame).gpr;
    let xmm_args = (*frame).xmm;
    let mut stack_args = [0u64; 8];
    if (caller_sp as usize) >= 0x10000 {
        for i in 0..8 {
            stack_args[i] = std::ptr::read_unaligned(caller_sp.add(i));
        }
    }
    let (ret_gpr, ret_xmm) = if is_orig {
        x86_runtime::emu::emu_run_full(target_va, gpr_args, xmm_args, stack_args)
    } else if let Some(host_fn) = x86_runtime::exe::lookup_va_only(target_va) {
        x86_runtime::emu::call_host_with_xmm(host_fn, gpr_args, xmm_args, stack_args)
    } else {
        x86_runtime::emu::emu_run_full(target_va, gpr_args, xmm_args, stack_args)
    };
    (*frame).gpr[0] = ret_gpr;
    (*frame).xmm[0] = ret_xmm;
}

#[cfg(target_arch = "aarch64")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Arm64Load {
    rt: usize,
    bytes: usize,
    signed_64: bool,
    signed_32: bool,
}

/// Decode a single AArch64 general-purpose load instruction (no writeback) so
/// reads from `KUSER_SHARED_DATA` (`0x7FFE0000..0x7FFE1000`, inside Apple
/// Silicon's 4 GB `__PAGEZERO`) can be emulated from the shadow page.
#[cfg(target_arch = "aarch64")]
fn decode_arm64_load(insn: u32) -> Option<Arm64Load> {
    let rt = (insn & 0x1F) as usize;
    let size = ((insn >> 30) & 3) as usize;
    let bytes = 1usize << size;

    // LDAPR{B,H,} (ARMv8.3-RCPC): xx11_1000_1011_1111_1100_00nn_nnnt_tttt
    if (insn & 0x3FFF_FC00) == 0x38BF_C000 {
        return Some(Arm64Load { rt, bytes, signed_64: false, signed_32: false });
    }
    // LDAR{B,H,} / LDAXR: xx00_1000_1101_1111_1111_11nn_nnnt_tttt
    if (insn & 0x3FFF_FC00) == 0x08DF_FC00 {
        return Some(Arm64Load { rt, bytes, signed_64: false, signed_32: false });
    }
    // LDAPUR{B,H,SB,SH,SW,} (ARMv8.4-RCPC2): xx01_1001_oo0i_iiii_iiii_00nn_nnnt_tttt
    if (insn & 0x3F20_0C00) == 0x1900_0000 {
        let opc = (insn >> 22) & 3;
        return match opc {
            1 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: false }),
            2 => Some(Arm64Load { rt, bytes, signed_64: true, signed_32: false }),
            3 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: true }),
            _ => None,
        };
    }
    // Unsigned-offset LDR{B,H,SB,SH,SW,}: xx11_1001_ooii_iiii_iiii_iinn_nnnt_tttt
    if (insn & 0x3F00_0000) == 0x3900_0000 {
        let opc = (insn >> 22) & 3;
        return match opc {
            1 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: false }),
            2 => Some(Arm64Load { rt, bytes, signed_64: true, signed_32: false }),
            3 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: true }),
            _ => None,
        };
    }
    // Unscaled LDUR or register-offset LDR (no writeback): xx11_1000_oo...
    if (insn & 0x3F00_0000) == 0x3800_0000 {
        let is_ldur = (insn & 0x0020_0C00) == 0x0000_0000;
        let is_reg = (insn & 0x0020_0C00) == 0x0020_0800;
        if is_ldur || is_reg {
            let opc = (insn >> 22) & 3;
            return match opc {
                1 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: false }),
                2 => Some(Arm64Load { rt, bytes, signed_64: true, signed_32: false }),
                3 => Some(Arm64Load { rt, bytes, signed_64: false, signed_32: true }),
                _ => None,
            };
        }
    }
    None
}

#[cfg(target_arch = "aarch64")]
static LSE2_UNALIGNED_TRAPS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Total 16-byte-boundary-crossing `ldapr`/`ldapur`/`stlr`/`stlur` alignment traps emulated
/// in this process (diagnostics).
#[cfg(all(test, target_arch = "aarch64"))]
pub fn lse2_unaligned_trap_count() -> u64 {
    LSE2_UNALIGNED_TRAPS.load(Ordering::Relaxed)
}

/// Decode an AArch64 RCpc/Release acquire-load or release-store instruction (`ldapr{b,h,}`,
/// `ldapur{b,h,}`, `stlr{b,h,}`, `stlur{b,h,}`) that can raise a `FEAT_LSE2` alignment fault
/// (`SIGBUS`, `DFSC == 0x21`) when crossing a 16-byte boundary: returns `(is_load, rt, bytes)`.
#[cfg(target_arch = "aarch64")]
fn decode_arm64_lse2_op(insn: u32) -> Option<(bool, usize, usize)> {
    let rt = (insn & 0x1F) as usize;
    let size = ((insn >> 30) & 3) as usize;
    let bytes = 1usize << size;
    // LDAPR{B,H,}
    if (insn & 0x3FFF_FC00) == 0x38BF_C000 {
        return Some((true, rt, bytes));
    }
    // LDAPUR{B,H,}
    if (insn & 0x3FE0_0C00) == 0x1940_0000 {
        return Some((true, rt, bytes));
    }
    // STLR{B,H,}
    if (insn & 0x3FFF_FC00) == 0x089F_FC00 {
        return Some((false, rt, bytes));
    }
    // STLUR{B,H,}
    if (insn & 0x3FE0_0C00) == 0x1900_0000 {
        return Some((false, rt, bytes));
    }
    None
}

/// Emulate a `FEAT_LSE2` alignment fault (`SIGBUS`, Data Abort `DFSC == 0x21`) on an
/// `ldapr`/`ldapur`/`stlr`/`stlur` that straddles a 16-byte granule boundary, preserving
/// TSO ordering via an acquire/release fence and unaligned access, then advancing `pc += 4`.
#[cfg(target_arch = "aarch64")]
unsafe fn try_emulate_unaligned_lse2(fault_addr: u64, esr: u32, ss: *mut u64) -> bool {
    let ec = (esr >> 26) & 0x3F;
    let dfsc = esr & 0x3F;
    if (ec != 0x24 && ec != 0x25) || dfsc != 0x21 {
        return false;
    }
    if !(0x1_0000..0x8000_0000_0000).contains(&fault_addr) {
        return false;
    }
    let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
    if pc < 0x10000 || (pc & 3) != 0 {
        return false;
    }
    let insn = core::ptr::read_volatile(pc as *const u32);
    // Another thread may have concurrently patched this LSE2 instruction to `b island` while
    // our SIGBUS was being delivered: retry at the same `pc` so we execute the exact-TSO island.
    if (insn & 0xFC00_0000) == 0x1400_0000 {
        return true;
    }
    let Some((is_load, rt, bytes)) = decode_arm64_lse2_op(insn) else {
        return false;
    };
    if bytes <= 1 {
        return false;
    }
    let p = fault_addr as *mut u8;
    if is_load {
        let val: u64 = match bytes {
            2 => std::ptr::read_unaligned(p as *const u16) as u64,
            4 => std::ptr::read_unaligned(p as *const u32) as u64,
            8 => std::ptr::read_unaligned(p as *const u64),
            _ => return false,
        };
        core::sync::atomic::fence(Ordering::Acquire);
        if rt < 31 {
            *ss.add(rt) = val;
        }
    } else {
        // rt == 31 is xzr/wzr (0), NOT sp (which lives at ss[31]).
        let val = if rt == 31 { 0u64 } else { *ss.add(rt) };
        core::sync::atomic::fence(Ordering::Release);
        match bytes {
            2 => std::ptr::write_unaligned(p as *mut u16, val as u16),
            4 => std::ptr::write_unaligned(p as *mut u32, val as u32),
            8 => std::ptr::write_unaligned(p as *mut u64, val),
            _ => return false,
        }
    }
    *ss.add(32) = (*ss.add(32) & !0x0000_FFFF_FFFF_FFFF) | pc.wrapping_add(4);
    let patched = x86_runtime::emu::patch_lse2_unaligned(pc as usize, insn);
    let n = LSE2_UNALIGNED_TRAPS.fetch_add(1, Ordering::Relaxed) + 1;
    if n == 1 || (n >= 1024 && n.is_power_of_two()) {
        log::info!("signals: emulated {n} FEAT_LSE2 unaligned alignment trap(s) (latest addr={fault_addr:#x} pc={pc:#x} bytes={bytes} load={is_load} patched={patched})");
    }
    true
}

#[cfg(target_arch = "aarch64")]
unsafe fn try_emulate_kuser_read(fault_addr: u64, ss: *mut u64) -> bool {
    let base = win32_rt::teb::KUSER_SHARED_DATA as u64;
    let size = win32_rt::teb::KUSER_SHARED_DATA_SIZE as u64;
    if fault_addr < base || fault_addr >= base + size {
        return false;
    }
    let shadow = win32_rt::teb::kuser_shadow_page();
    if shadow.is_null() {
        return false;
    }
    let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
    if pc < 0x10000 || (pc & 3) != 0 {
        return false;
    }
    let insn = std::ptr::read_unaligned(pc as *const u32);
    let Some(ld) = decode_arm64_load(insn) else {
        return false;
    };
    let off = (fault_addr - base) as usize;
    if off + ld.bytes > win32_rt::teb::KUSER_SHARED_DATA_SIZE {
        return false;
    }
    let src = shadow.add(off);
    let val: u64 = match (ld.bytes, ld.signed_64, ld.signed_32) {
        (1, false, false) => std::ptr::read_unaligned(src) as u64,
        (1, true, false) => std::ptr::read_unaligned(src as *const i8) as i64 as u64,
        (1, false, true) => std::ptr::read_unaligned(src as *const i8) as i32 as u32 as u64,
        (2, false, false) => std::ptr::read_unaligned(src as *const u16) as u64,
        (2, true, false) => std::ptr::read_unaligned(src as *const i16) as i64 as u64,
        (2, false, true) => std::ptr::read_unaligned(src as *const i16) as i32 as u32 as u64,
        (4, false, false) => std::ptr::read_unaligned(src as *const u32) as u64,
        (4, true, false) => std::ptr::read_unaligned(src as *const i32) as i64 as u64,
        (8, false, false) => std::ptr::read_unaligned(src as *const u64),
        _ => return false,
    };
    if ld.rt < 31 {
        *ss.add(ld.rt) = val;
    }
    *ss.add(32) = (*ss.add(32) & !0x0000_FFFF_FFFF_FFFF) | pc.wrapping_add(4);
    true
}

/// Decode a single AArch64 general-purpose store instruction (no writeback) so
/// writes to a logically writable 4 KiB subpage sharing a `PROT_NONE` 16 KiB host
/// page with a `PAGE_GUARD`/`PAGE_NOACCESS` sibling can complete without exposing
/// the protected siblings. Returns `(rt, bytes)`.
#[cfg(target_arch = "aarch64")]
fn decode_arm64_store(insn: u32) -> Option<(usize, usize)> {
    let rt = (insn & 0x1F) as usize;
    let size = ((insn >> 30) & 3) as usize;
    let bytes = 1usize << size;
    // STLR{B,H,}
    if (insn & 0x3FFF_FC00) == 0x089F_FC00 {
        return Some((rt, bytes));
    }
    // STLUR{B,H,}
    if (insn & 0x3FE0_0C00) == 0x1900_0000 {
        return Some((rt, bytes));
    }
    // Unsigned-offset STR{B,H,W,X}: xx11_1001_00ii_iiii_iiii_iinn_nnnt_tttt
    if (insn & 0x3FC0_0000) == 0x3900_0000 {
        return Some((rt, bytes));
    }
    // Unscaled STUR or register-offset STR (no writeback): xx11_1000_00...
    if (insn & 0x3FC0_0000) == 0x3800_0000 {
        let is_stur = (insn & 0x0020_0C00) == 0x0000_0000;
        let is_reg = (insn & 0x0020_0C00) == 0x0020_0800;
        if is_stur || is_reg {
            return Some((rt, bytes));
        }
    }
    None
}

/// Emulate one AArch64 GPR load or store against a logically accessible 4 KiB subpage
/// inside a 16 KiB host page that is kept `PROT_NONE` because a sibling 4 KiB subpage
/// is still `PAGE_GUARD` or `PAGE_NOACCESS`. Caller MUST hold `HPAGE_LOCKS[hpage]`.
#[cfg(target_arch = "aarch64")]
unsafe fn try_emulate_subpage_access(fault_addr: u64, is_write: bool, ss: *mut u64, logical_prot: u32) -> bool {
    use win32_rt::kernel32::memory::{
        PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_READWRITE, PAGE_WRITECOPY,
    };
    if (logical_prot & 0xFF) == PAGE_NOACCESS || (logical_prot & PAGE_GUARD) != 0 {
        return false;
    }
    let writable = matches!(
        logical_prot & 0xFF,
        PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY
    );
    if is_write && !writable {
        return false;
    }
    let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
    if pc < 0x10000 || (pc & 3) != 0 {
        return false;
    }
    let insn = std::ptr::read_unaligned(pc as *const u32);
    let hpage = (fault_addr as usize) & !0x3FFF;
    let p = fault_addr as usize as *mut u8;
    if is_write {
        let Some((rt, bytes)) = decode_arm64_store(insn) else {
            return false;
        };
        let val = if rt == 31 { 0u64 } else { *ss.add(rt) };
        if libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_READ | libc::PROT_WRITE) != 0 {
            return false;
        }
        core::sync::atomic::fence(Ordering::Release);
        match bytes {
            1 => std::ptr::write_unaligned(p, val as u8),
            2 => std::ptr::write_unaligned(p as *mut u16, val as u16),
            4 => std::ptr::write_unaligned(p as *mut u32, val as u32),
            8 => std::ptr::write_unaligned(p as *mut u64, val),
            _ => {
                let _ = libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_NONE);
                return false;
            }
        }
        let _ = libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_NONE);
        win32_rt::kernel32::memory::mark_write_watch_dirty(fault_addr as usize);
    } else {
        let Some(ld) = decode_arm64_load(insn) else {
            return false;
        };
        if libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_READ) != 0 {
            return false;
        }
        let val: u64 = match (ld.bytes, ld.signed_64, ld.signed_32) {
            (1, false, false) => std::ptr::read_unaligned(p) as u64,
            (1, true, false) => std::ptr::read_unaligned(p as *const i8) as i64 as u64,
            (1, false, true) => std::ptr::read_unaligned(p as *const i8) as i32 as u32 as u64,
            (2, false, false) => std::ptr::read_unaligned(p as *const u16) as u64,
            (2, true, false) => std::ptr::read_unaligned(p as *const i16) as i64 as u64,
            (2, false, true) => std::ptr::read_unaligned(p as *const i16) as i32 as u32 as u64,
            (4, false, false) => std::ptr::read_unaligned(p as *const u32) as u64,
            (4, true, false) => std::ptr::read_unaligned(p as *const i32) as i64 as u64,
            (8, false, false) => std::ptr::read_unaligned(p as *const u64),
            _ => {
                let _ = libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_NONE);
                return false;
            }
        };
        core::sync::atomic::fence(Ordering::Acquire);
        let _ = libc::mprotect(hpage as *mut libc::c_void, 0x4000, libc::PROT_NONE);
        if ld.rt < 31 {
            *ss.add(ld.rt) = val;
        }
    }
    *ss.add(32) = (*ss.add(32) & !0x0000_FFFF_FFFF_FFFF) | pc.wrapping_add(4);
    true
}

#[cfg(target_arch = "aarch64")]
unsafe fn deliver_page_fault_inner(code: u32, addr: u64, is_write: bool, gpr: [u64; 16], guest_rip: u64) -> bool {
    let xmm = [0u128; 16];
    if code == unwind::STATUS_GUARD_PAGE_VIOLATION {
        let kind = if is_write { 1u64 } else { 0u64 };
        let mut rec = unwind::guest_exception_record(code, guest_rip, &[kind, addr]);
        let mut ctx = unwind::guest_context(&gpr, &xmm, guest_rip, 0x202);
        return unwind::dispatch_veh_guest(&mut rec, &mut ctx);
    }
    let kinds: &[u64] = if is_write { &[1, 8, 0] } else { &[8, 0] };
    for &kind in kinds {
        let rip = if kind == 8 { addr } else { guest_rip };
        let mut rec = unwind::guest_exception_record(code, rip, &[kind, addr]);
        let mut ctx = unwind::guest_context(&gpr, &xmm, rip, 0x202);
        if unwind::dispatch_veh_guest(&mut rec, &mut ctx) {
            return true;
        }
    }
    false
}

/// Per-thread helper pthread for dispatching `PAGE_NOACCESS` / `PAGE_GUARD` guest
/// exception handlers (`Wow64PrepareForException` / VEH).
///
/// Running `dispatch_veh_guest` on a separate OS thread gives the handler its own
/// `x86-runtime` `TLS_EMU` guest stack so it NEVER overwrites the interrupted
/// thread's live guest stack frame (whose `TLS_EMU.state.sp` is stale mid-block),
/// while temporarily adopting the faulting thread's Windows TEB so TID/TLS/fibers
/// remain identical.
#[cfg(target_arch = "aarch64")]
struct PageFaultHelper {
    req: std::sync::Arc<(std::sync::Mutex<Option<PageFaultReq>>, std::sync::Condvar)>,
    resp: std::sync::Arc<(std::sync::Mutex<Option<bool>>, std::sync::Condvar)>,
}

#[cfg(target_arch = "aarch64")]
struct PageFaultReq {
    teb: *mut u8,
    code: u32,
    addr: u64,
    is_write: bool,
    gpr: [u64; 16],
    guest_rip: u64,
}

#[cfg(target_arch = "aarch64")]
unsafe impl Send for PageFaultReq {}

#[cfg(target_arch = "aarch64")]
impl PageFaultHelper {
    fn new() -> Self {
        let req = std::sync::Arc::new((std::sync::Mutex::new(None::<PageFaultReq>), std::sync::Condvar::new()));
        let resp = std::sync::Arc::new((std::sync::Mutex::new(None::<bool>), std::sync::Condvar::new()));
        let req_w = std::sync::Arc::clone(&req);
        let resp_w = std::sync::Arc::clone(&resp);
        std::thread::Builder::new()
            .name("win32-pagefault".into())
            .stack_size(4 * 1024 * 1024)
            .spawn(move || loop {
                let job = {
                    let mut g = req_w.0.lock().unwrap();
                    loop {
                        if let Some(j) = g.take() {
                            break j;
                        }
                        g = req_w.1.wait(g).unwrap();
                    }
                };
                let ok = unsafe {
                    if !job.teb.is_null() {
                        win32_rt::teb::install(job.teb);
                    }
                    let res = deliver_page_fault_inner(job.code, job.addr, job.is_write, job.gpr, job.guest_rip);
                    if !job.teb.is_null() {
                        win32_rt::teb::uninstall();
                    }
                    res
                };
                let mut rg = resp_w.0.lock().unwrap();
                *rg = Some(ok);
                resp_w.1.notify_one();
            })
            .expect("spawn win32-pagefault helper");
        Self { req, resp }
    }

    unsafe fn dispatch(&self, code: u32, addr: u64, is_write: bool) -> bool {
        let teb = win32_rt::teb::current_raw();
        let (gpr, guest_rip) = x86_runtime::emu::current_guest_state().unwrap_or(([0u64; 16], addr));
        if !teb.is_null() {
            win32_rt::teb::uninstall();
        }
        {
            let mut g = self.req.0.lock().unwrap();
            *g = Some(PageFaultReq { teb, code, addr, is_write, gpr, guest_rip });
            self.req.1.notify_one();
        }
        let ok = {
            let mut rg = self.resp.0.lock().unwrap();
            loop {
                if let Some(v) = rg.take() {
                    break v;
                }
                rg = self.resp.1.wait(rg).unwrap();
            }
        };
        if !teb.is_null() {
            win32_rt::teb::install(teb);
        }
        ok
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn deliver_page_fault(code: u32, addr: u64, is_write: bool) -> bool {
    thread_local! {
        static HELPER: std::cell::OnceCell<PageFaultHelper> = const { std::cell::OnceCell::new() };
    }
    HELPER.with(|h| h.get_or_init(PageFaultHelper::new).dispatch(code, addr, is_write))
}

#[cfg(target_arch = "aarch64")]
static HPAGE_LOCKS: std::sync::LazyLock<[std::sync::Mutex<()>; 64]> =
    std::sync::LazyLock::new(|| std::array::from_fn(|_| std::sync::Mutex::new(())));

/// Handle a host `SIGBUS`/`SIGSEGV` caused by the emulator (or loader) touching a
/// `PAGE_NOACCESS` / `PAGE_GUARD` guest page (e.g. on-demand page-encrypted `.text`
/// protected via `VirtualProtect(..., PAGE_NOACCESS)` and decrypted in a
/// `Wow64PrepareForException` / VEH handler).
///
/// Because Apple Silicon uses 16 KiB hardware pages while Windows uses 4 KiB pages,
/// we serialize unlocking per 16 KiB host page, keep the host page `PROT_NONE` in
/// `VirtualProtect` until all 4 KiB sub-pages in the host page are unlocked, and
/// eagerly unlock every `PAGE_NOACCESS` 4 KiB sibling inside the same 16 KiB host
/// page so no thread can ever race into a still-encrypted sibling sub-page.
#[cfg(target_arch = "aarch64")]
unsafe fn try_dispatch_guest_page_fault(fault_addr: u64, pc: u64, is_write: bool, ss: *mut u64) -> bool {
    use win32_rt::kernel32::memory::{self, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS};
    thread_local! {
        static IN_FAULT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        static LAST_RACE_RETRY: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
    }
    if IN_FAULT.with(|f| f.get()) {
        return false;
    }
    let addr = fault_addr as usize;
    let hpage = addr & !0x3FFF;
    let sub_is_protected = |p: usize| -> bool {
        memory::query(p).is_some_and(|m| {
            m.state == MEM_COMMIT && (m.protect == PAGE_NOACCESS || (m.protect & PAGE_GUARD) != 0)
        })
    };
    let Some(initial_mbi) = memory::query(addr) else {
        return false;
    };
    if initial_mbi.state != MEM_COMMIT {
        return false;
    }
    let initial_fault_prot = initial_mbi.protect == PAGE_NOACCESS || (initial_mbi.protect & PAGE_GUARD) != 0;
    let initial_any_sibling_prot = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
    // If the faulting 4 KiB subpage is logically accessible but shares a `PROT_NONE` 16 KiB
    // host page with a `PAGE_GUARD` or non-guest `PAGE_NOACCESS` sibling, emulate the access
    // transparently (even if no VEH handler is currently registered) so sibling protections stay intact.
    if !initial_fault_prot && initial_any_sibling_prot {
        let _hpage_guard = HPAGE_LOCKS[(hpage >> 14) & 63].lock().unwrap();
        if let Some(mbi) = memory::query(addr) {
            let now_fault_prot = mbi.protect == PAGE_NOACCESS || (mbi.protect & PAGE_GUARD) != 0;
            let now_any_sibling = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
            if mbi.state == MEM_COMMIT && !now_fault_prot {
                if !now_any_sibling {
                    return true;
                }
                let any_sibling_guard = (0..4).any(|i| {
                    memory::query(hpage + i * 0x1000)
                        .is_some_and(|m| m.state == MEM_COMMIT && (m.protect & PAGE_GUARD) != 0)
                });
                let any_guest_pe_noaccess_sibling = (0..4).any(|i| {
                    let sub = hpage + i * 0x1000;
                    in_guest(sub)
                        && memory::query(sub).is_some_and(|m| m.state == MEM_COMMIT && m.protect == PAGE_NOACCESS)
                });
                if (any_sibling_guard || !any_guest_pe_noaccess_sibling || !unwind::has_guest_exception_handler())
                    && try_emulate_subpage_access(fault_addr, is_write, ss, mbi.protect)
                {
                    return true;
                }
            }
        }
    }
    if !unwind::has_guest_exception_handler() {
        return false;
    }
    // Only dispatch guest VEH handlers on threads that own a Windows TEB.
    // Non-guest host background threads (e.g. AOT builders) have TEB == null and
    // would fault immediately on `gs:[0x60]` inside any guest VEH handler.
    if win32_rt::teb::current_raw().is_null() {
        return false;
    }
    let pre_protected = initial_fault_prot || initial_any_sibling_prot;
    if !pre_protected {
        // Cross-core race: another thread just unlocked this 16 KiB host page via
        // VirtualProtect while our read fault was already in flight to the kernel.
        // Retry once if the page is now logically readable and recently transitioned.
        if !is_write && memory::hpage_had_prot_change(hpage) {
            let key = (pc, fault_addr);
            if LAST_RACE_RETRY.with(|c| c.get()) != key {
                let _hpage_guard = HPAGE_LOCKS[(hpage >> 14) & 63].lock().unwrap();
                if let Some(mbi) = memory::query(addr) {
                    let still_prot = mbi.protect == PAGE_NOACCESS
                        || (mbi.protect & PAGE_GUARD) != 0
                        || (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
                    if mbi.state == MEM_COMMIT && !still_prot {
                        memory::ensure_host_page_readable(addr);
                        LAST_RACE_RETRY.with(|c| c.set(key));
                        return true;
                    }
                }
            }
        }
        return false;
    }
    LAST_RACE_RETRY.with(|c| c.set((0, 0)));

    let hpage_lock = &HPAGE_LOCKS[(hpage >> 14) & 63];
    let mut hpage_guard = Some(hpage_lock.lock().unwrap());
    let Some(mbi) = memory::query(addr) else {
        return false;
    };
    if mbi.state != MEM_COMMIT {
        return false;
    }
    let is_guard = (mbi.protect & PAGE_GUARD) != 0;
    let fault_protected = mbi.protect == PAGE_NOACCESS || is_guard;
    let any_sibling_protected = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
    if !fault_protected && !any_sibling_protected {
        // Another thread finished unlocking this 16 KiB host page while we waited.
        return true;
    }

    IN_FAULT.with(|f| f.set(true));
    let mut handled = false;
    if is_guard {
        // Windows clears PAGE_GUARD atomically BEFORE delivering STATUS_GUARD_PAGE_VIOLATION
        // so that VirtualQuery inside the handler observes the underlying protection and
        // returning EXCEPTION_CONTINUE_EXECUTION completes the faulting access once.
        let _consumed = memory::consume_guard_page(addr);
        // Release HPAGE_LOCKS before dispatching the VEH on `PageFaultHelper` (a separate
        // OS thread). If the VEH accesses the just-consumed 4 KiB page while a sibling 4 KiB
        // subpage keeps the 16 KiB host page PROT_NONE, the helper thread faults and acquires
        // this same lock to emulate that access; holding it across dispatch would deadlock.
        drop(hpage_guard.take());
        handled = deliver_page_fault(unwind::STATUS_GUARD_PAGE_VIOLATION, fault_addr, is_write);
        if handled {
            let _emul_guard = hpage_lock.lock().unwrap();
            if let Some(now_mbi) = memory::query(addr) {
                let now_fault_prot =
                    now_mbi.protect == PAGE_NOACCESS || (now_mbi.protect & PAGE_GUARD) != 0;
                let now_any_sibling = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
                if now_mbi.state == MEM_COMMIT && !now_fault_prot && now_any_sibling {
                    // Sibling 4 KiB subpages remain guarded/noaccess (host page stays PROT_NONE):
                    // emulate this single faulting instruction without exposing sibling subpages.
                    handled = try_emulate_subpage_access(
                        fault_addr,
                        is_write,
                        ss,
                        now_mbi.protect,
                    );
                }
            }
        }
        IN_FAULT.with(|f| f.set(false));
        return handled;
    }
    if fault_protected {
        handled = deliver_page_fault(unwind::STATUS_ACCESS_VIOLATION, fault_addr, false);
        if !handled {
            IN_FAULT.with(|f| f.set(false));
            return false;
        }
    }
    // Unlock any sibling 4 KiB encrypted PAGE_NOACCESS sub-pages sharing this 16 KiB host page
    // (never eagerly consume sibling PAGE_GUARD pages).
    let fault_page4 = addr & !0xFFF;
    for i in 0..4 {
        let sub = hpage + i * 0x1000;
        if sub == fault_page4 {
            continue;
        }
        let sub_noaccess = memory::query(sub).is_some_and(|m| m.state == MEM_COMMIT && m.protect == PAGE_NOACCESS);
        if sub_noaccess && in_guest(sub) {
            let _ = deliver_page_fault(unwind::STATUS_ACCESS_VIOLATION, sub as u64, false);
        }
    }
    // Restore host readability if any non-guest/permanent PAGE_NOACCESS sibling
    // caused VirtualProtect to defer mprotect, or if the faulting page itself was
    // already logically readable (e.g. PE header or .rdata sharing the 16 KiB page).
    let still_prot = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
    if !fault_protected {
        if !still_prot {
            let host_p = memory::win_prot_to_host(mbi.protect);
            let host_p = if host_p & libc::PROT_WRITE != 0 {
                host_p & !libc::PROT_EXEC
            } else {
                host_p
            };
            if libc::mprotect(hpage as *mut libc::c_void, 0x4000, host_p) == 0 {
                handled = true;
            }
        }
    } else if still_prot {
        memory::ensure_host_page_readable(addr);
    }
    IN_FAULT.with(|f| f.set(false));
    handled
}

#[cfg(not(target_arch = "x86_64"))]
extern "C" fn handler(sig: libc::c_int, _info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    unsafe {
        let _ = (MC_ES, MC_SS, SS_RCX, SS_RDX, SS_RSP, SS_RIP, SS_RFLAGS, in_guest, rd::<u64>, Context::zeroed);
        let mc = *((uc as *const u8).add(UC_MCONTEXT) as *const *mut u8);
        let fault_addr: u64 = rd(mc, 0);
        let ss = mc.add(16) as *mut u64;
        // Direct guest reads of Windows KUSER_SHARED_DATA (`0x7FFE0000..0x7FFE1000`):
        // Apple Silicon's mandatory 4 GB `__PAGEZERO` prevents mapping that address,
        // so emulate load instructions transparently against the shadow page.
        #[cfg(target_arch = "aarch64")]
        if (sig == libc::SIGSEGV || sig == libc::SIGBUS) && try_emulate_kuser_read(fault_addr & 0x0000_FFFF_FFFF_FFFF, ss) {
            return;
        }
        #[cfg(target_arch = "aarch64")]
        if sig == libc::SIGBUS {
            let esr: u32 = rd(mc, 8);
            if try_emulate_unaligned_lse2(fault_addr & 0x0000_FFFF_FFFF_FFFF, esr, ss) {
                return;
            }
        }
        #[cfg(target_arch = "aarch64")]
        if sig == libc::SIGSEGV || sig == libc::SIGBUS {
            let esr: u32 = rd(mc, 8);
            let ec = (esr >> 26) & 0x3F;
            let is_write = (ec == 0x24 || ec == 0x25) && (esr & (1 << 6)) != 0;
            let clean_fault = fault_addr & 0x0000_FFFF_FFFF_FFFF;
            if win32_rt::kernel32::memory::try_handle_write_watch_fault(clean_fault as usize, is_write) {
                return;
            }
            let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
            if try_dispatch_guest_page_fault(clean_fault, pc, is_write, ss) {
                return;
            }
        }
        // A guarded access (SEH `__except` scope of a lifted fn, x86-runtime lift::guarded):
        // the probe returns "faulted" and the lifted frame raises the access violation.
        if (sig == libc::SIGSEGV || sig == libc::SIGBUS) && x86_runtime::dll_native::lift::guarded::fixup_darwin_ss(ss) {
            return;
        }
        let raw_pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
        let clean_fault = fault_addr & 0x0000_FFFF_FFFF_FFFF;
        let orig_tag = crate::hooks::ORIG_TRAMPOLINE_TAG as u64;
        let is_orig = (raw_pc & orig_tag) != 0;
        let target_va = raw_pc & !orig_tag;
        if (sig == libc::SIGBUS || sig == libc::SIGSEGV || sig == libc::SIGILL)
            && (clean_fault == raw_pc || sig == libc::SIGILL)
            && (is_orig
                || in_guest(target_va as usize)
                || x86_runtime::exe::lookup_va_only(target_va).is_some())
        {
            *ss.add(16) = raw_pc;
            *ss.add(32) = aarch64_bridge_trampoline as *const () as usize as u64;
            return;
        }
        let mut rec: ExceptionRecord = std::mem::zeroed();
        rec.code = match sig {
            libc::SIGILL => unwind::STATUS_ILLEGAL_INSTRUCTION,
            libc::SIGFPE => unwind::STATUS_INTEGER_DIVIDE_BY_ZERO,
            libc::SIGTRAP => unwind::STATUS_BREAKPOINT,
            _ => unwind::STATUS_ACCESS_VIOLATION,
        };
        rec.number_parameters = 2;
        rec.information[1] = fault_addr;
        let mut what = format!(
            "host fault: {} (signal {sig}, addr {fault_addr:#x})",
            unwind::describe_exception(&rec)
        );
        if let Some(hint) = x86_runtime::lstack::describe_canary(fault_addr) {
            what.push_str("\n  ");
            what.push_str(&hint);
        }
        fatal(&what, mc);
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::*;

    #[test]
    fn decode_arm64_kuser_load_encodings() {
        // lapdr w9, [x3]
        assert_eq!(decode_arm64_load(0xb8bf_c069), Some(Arm64Load { rt: 9, bytes: 4, signed_64: false, signed_32: false }));
        // ldapr x9, [x3]
        assert_eq!(decode_arm64_load(0xf8bf_c069), Some(Arm64Load { rt: 9, bytes: 8, signed_64: false, signed_32: false }));
        // ldaprb w9, [x3]
        assert_eq!(decode_arm64_load(0x38bf_c069), Some(Arm64Load { rt: 9, bytes: 1, signed_64: false, signed_32: false }));
        // ldaprh w9, [x3]
        assert_eq!(decode_arm64_load(0x78bf_c069), Some(Arm64Load { rt: 9, bytes: 2, signed_64: false, signed_32: false }));
        // ldar w9, [x3]
        assert_eq!(decode_arm64_load(0x88df_fc69), Some(Arm64Load { rt: 9, bytes: 4, signed_64: false, signed_32: false }));
        // ldar x9, [x3]
        assert_eq!(decode_arm64_load(0xc8df_fc69), Some(Arm64Load { rt: 9, bytes: 8, signed_64: false, signed_32: false }));
        // ldapur w9, [x3, #4]
        assert_eq!(decode_arm64_load(0x9940_4069), Some(Arm64Load { rt: 9, bytes: 4, signed_64: false, signed_32: false }));
        // ldapur x9, [x3, #4]
        assert_eq!(decode_arm64_load(0xd940_4069), Some(Arm64Load { rt: 9, bytes: 8, signed_64: false, signed_32: false }));
        // ldr w9, [x3, #4]
        assert_eq!(decode_arm64_load(0xb940_0469), Some(Arm64Load { rt: 9, bytes: 4, signed_64: false, signed_32: false }));
        // ldr x9, [x3, #8]
        assert_eq!(decode_arm64_load(0xf940_0469), Some(Arm64Load { rt: 9, bytes: 8, signed_64: false, signed_32: false }));
        // ldur w9, [x3, #4]
        assert_eq!(decode_arm64_load(0xb840_4069), Some(Arm64Load { rt: 9, bytes: 4, signed_64: false, signed_32: false }));
        // ldr x9, [x3, x4]
        assert_eq!(decode_arm64_load(0xf864_6869), Some(Arm64Load { rt: 9, bytes: 8, signed_64: false, signed_32: false }));
        // ldrsw x9, [x3, #4]
        assert_eq!(decode_arm64_load(0xb980_0469), Some(Arm64Load { rt: 9, bytes: 4, signed_64: true, signed_32: false }));
        // Stores must NOT match as loads (str w9, [x3, #4] = 0xb9000469)
        assert_eq!(decode_arm64_load(0xb900_0469), None);
    }

    #[test]
    fn decode_arm64_lse2_op_encodings() {
        assert_eq!(decode_arm64_lse2_op(0x78bf_c069), Some((true, 9, 2))); // ldaprh w9, [x3]
        assert_eq!(decode_arm64_lse2_op(0xb8bf_c069), Some((true, 9, 4))); // ldapr w9, [x3]
        assert_eq!(decode_arm64_lse2_op(0xf8bf_c069), Some((true, 9, 8))); // ldapr x9, [x3]
        assert_eq!(decode_arm64_lse2_op(0xd941_0041), Some((true, 1, 8))); // ldapur x1, [x2, #16]
        assert_eq!(decode_arm64_lse2_op(0x9940_4069), Some((true, 9, 4))); // ldapur w9, [x3, #4]
        assert_eq!(decode_arm64_lse2_op(0x489f_fc69), Some((false, 9, 2))); // stlrh w9, [x3]
        assert_eq!(decode_arm64_lse2_op(0x889f_fc69), Some((false, 9, 4))); // stlr w9, [x3]
        assert_eq!(decode_arm64_lse2_op(0xc89f_fc69), Some((false, 9, 8))); // stlr x9, [x3]
        assert_eq!(decode_arm64_lse2_op(0xd901_005f), Some((false, 31, 8))); // stlur xzr, [x2, #16]
        // Plain ldr/str must NOT match LSE2 op decoder.
        assert_eq!(decode_arm64_lse2_op(0xf940_0469), None);
        assert_eq!(decode_arm64_lse2_op(0xb900_0469), None);
    }

    #[test]
    fn live_unaligned_lse2_sigbus_emulation() {
        install_handlers();
        #[repr(C, align(32))]
        struct Buf([u8; 32]);
        let mut buf = Buf([0xAAu8; 32]);
        let base = buf.0.as_mut_ptr();
        let before = lse2_unaligned_trap_count();
        unsafe {
            // 64-bit store & load crossing 16-byte boundary (offset 12..20) via stlur/ldapur
            let v64: u64 = 0x1122_3344_5566_7788;
            let got64: u64;
            core::arch::asm!(
                "stlur {v}, [{p}, #12]",
                "ldapur {out}, [{p}, #12]",
                p = in(reg) base,
                v = in(reg) v64,
                out = out(reg) got64,
                options(nostack),
            );
            assert_eq!(got64, v64);
            // 32-bit store & load crossing 16-byte boundary (offset 13..17) via stlr/ldapr
            let v32: u32 = 0xDEAD_BEEF;
            let got32: u32;
            let p13 = base.add(13);
            core::arch::asm!(
                "stlr {v:w}, [{p}]",
                "ldapr {out:w}, [{p}]",
                p = in(reg) p13,
                v = in(reg) v32,
                out = out(reg) got32,
                options(nostack),
            );
            assert_eq!(got32, v32);
            // 16-bit store & load crossing 16-byte boundary (offset 15..17)
            let v16: u32 = 0xCAFE;
            let got16: u32;
            core::arch::asm!(
                "stlurh {v:w}, [{p}, #15]",
                "ldapurh {out:w}, [{p}, #15]",
                p = in(reg) base,
                v = in(reg) v16,
                out = out(reg) got16,
                options(nostack),
            );
            assert_eq!(got16, v16);
            // stlur xzr across boundary must write 0 (NOT sp!)
            let got_zero: u64;
            core::arch::asm!(
                "stlur xzr, [{p}, #12]",
                "ldapur {out}, [{p}, #12]",
                p = in(reg) base,
                out = out(reg) got_zero,
                options(nostack),
            );
            assert_eq!(got_zero, 0);
        }
        assert!(
            lse2_unaligned_trap_count() >= before + 8,
            "expected >= 8 emulated LSE2 SIGBUS traps, got {}",
            lse2_unaligned_trap_count() - before
        );
    }
}
