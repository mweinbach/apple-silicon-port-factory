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
#[cfg(target_arch = "aarch64")]
const MC_NS_ARM64: usize = 288; // offsetof(struct __darwin_mcontext64, __ns), 32 x u128 __v at +0

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

/// ARMv8.1 LSE atomic read-modify-write operation kind.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AtomicRmwOp {
    Add, // LDADD{,A,L,AL}
    Clr, // LDCLR{,A,L,AL} (mem &= !val)
    Eor, // LDEOR{,A,L,AL} (mem ^= val)
    Set, // LDSET{,A,L,AL} (mem |= val)
    Swp, // SWP{,A,L,AL}
}

/// Decoded AArch64 memory load, store, or LSE atomic RMW/CAS instruction for subpage
/// access emulation on 16 KiB Apple Silicon pages.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug)]
enum Arm64MemOp {
    GprLoad { rt: usize, bytes: usize, signed_64: bool, signed_32: bool },
    GprStore { rt: usize, bytes: usize },
    SimdLoad { vt: usize, bytes: usize },
    SimdStore { vt: usize, bytes: usize },
    GprPairLoad { rt1: usize, rt2: usize, rn: usize, imm: i64, elem_bytes: usize, signed_64: bool },
    GprPairStore { rt1: usize, rt2: usize, rn: usize, imm: i64, elem_bytes: usize },
    SimdPairLoad { vt1: usize, vt2: usize, rn: usize, imm: i64, elem_bytes: usize },
    SimdPairStore { vt1: usize, vt2: usize, rn: usize, imm: i64, elem_bytes: usize },
    /// LSE `LDADD`/`LDCLR`/`LDEOR`/`LDSET`/`SWP` (`rs` = operand, `rt` = old-value dest).
    AtomicRmw { op: AtomicRmwOp, rs: usize, rt: usize, bytes: usize },
    /// LSE `CAS{,A,L,AL}` (`rs` = expected in / old out, `rt` = replacement in).
    AtomicCas { rs: usize, rt: usize, bytes: usize },
    /// LSE `CASP{,A,L,AL}` (`rs, rs+1` = expected in / old out, `rt, rt+1` = replacement in).
    AtomicCasp { rs: usize, rt: usize, elem_bytes: usize },
}

#[cfg(target_arch = "aarch64")]
impl Arm64MemOp {
    fn is_atomic(self) -> bool {
        matches!(self, Self::AtomicRmw { .. } | Self::AtomicCas { .. } | Self::AtomicCasp { .. })
    }
    /// Logical write-permission requirement (`true` for stores and all LSE RMW/CAS atomics).
    fn requires_write(self) -> bool {
        matches!(
            self,
            Self::GprStore { .. }
                | Self::SimdStore { .. }
                | Self::GprPairStore { .. }
                | Self::SimdPairStore { .. }
                | Self::AtomicRmw { .. }
                | Self::AtomicCas { .. }
                | Self::AtomicCasp { .. }
        )
    }
    /// Check compatibility with hardware `ESR_EL1.WnR`.
    /// On Apple Silicon, LSE RMW/CAS atomics (`casal`, `ldaddal`, etc.) faulting on a
    /// `PROT_NONE` page can report `WnR = 0` (read phase of RMW) or `WnR = 1`.
    fn matches_esr_write(self, esr_is_write: bool) -> bool {
        if self.is_atomic() {
            true
        } else {
            self.requires_write() == esr_is_write
        }
    }
    fn total_bytes(self) -> usize {
        match self {
            Self::GprLoad { bytes, .. }
            | Self::GprStore { bytes, .. }
            | Self::SimdLoad { bytes, .. }
            | Self::SimdStore { bytes, .. }
            | Self::AtomicRmw { bytes, .. }
            | Self::AtomicCas { bytes, .. } => bytes,
            Self::GprPairLoad { elem_bytes, .. }
            | Self::GprPairStore { elem_bytes, .. }
            | Self::SimdPairLoad { elem_bytes, .. }
            | Self::SimdPairStore { elem_bytes, .. }
            | Self::AtomicCasp { elem_bytes, .. } => elem_bytes * 2,
        }
    }
    /// Recover the instruction's exact starting effective address from `insn` and `ss`,
    /// verifying that `fault_addr` (`FAR_EL1`) falls within `[ea, ea + total_bytes)`.
    ///
    /// This handles Apple Silicon reporting `FAR_EL1` at the second 16 KiB host-page
    /// boundary when an unaligned single-register or pair access starts on an accessible
    /// host page and crosses into an adjacent host page kept `PROT_NONE` for a sibling.
    unsafe fn effective_addr(self, insn: u32, fault_addr: u64, ss: *const u64) -> Option<usize> {
        let total = self.total_bytes();
        let ea: usize = match self {
            Self::GprPairLoad { rn, imm, .. }
            | Self::GprPairStore { rn, imm, .. }
            | Self::SimdPairLoad { rn, imm, .. }
            | Self::SimdPairStore { rn, imm, .. } => {
                // `ss[0..=30]` are X0..X30, `ss[31]` is SP.
                (*ss.add(rn)).wrapping_add(imm as u64) as usize
            }
            Self::AtomicRmw { .. } | Self::AtomicCas { .. } | Self::AtomicCasp { .. } => {
                let rn = ((insn >> 5) & 0x1F) as usize;
                *ss.add(rn) as usize
            }
            Self::GprLoad { bytes, .. }
            | Self::GprStore { bytes, .. }
            | Self::SimdLoad { bytes, .. }
            | Self::SimdStore { bytes, .. } => {
                let rn = ((insn >> 5) & 0x1F) as usize;
                let base = *ss.add(rn);
                // 1. Unsigned 12-bit scaled offset: LDR/STR GPR (0x39) or SIMD&FP (0x3D).
                if (insn & 0x3B00_0000) == 0x3900_0000 {
                    let imm12 = ((insn >> 10) & 0xFFF) as u64;
                    base.wrapping_add(imm12 * (bytes as u64)) as usize
                }
                // 2. Register-offset: LDR/STR GPR (0x3820_0800) or SIMD&FP (0x3C20_0800).
                else if (insn & 0x3B20_0C00) == 0x3820_0800 {
                    let rm = ((insn >> 16) & 0x1F) as usize;
                    let raw_rm = if rm == 31 { 0u64 } else { *ss.add(rm) };
                    let option = (insn >> 13) & 7;
                    let ext = match option {
                        0b010 => (raw_rm as u32) as u64,              // UXTW
                        0b011 | 0b111 => raw_rm,                      // LSL / SXTX
                        0b110 => (raw_rm as i32) as i64 as u64,       // SXTW
                        _ => return None,
                    };
                    let shift = if ((insn >> 12) & 1) != 0 {
                        bytes.trailing_zeros()
                    } else {
                        0
                    };
                    base.wrapping_add(ext.wrapping_shl(shift)) as usize
                }
                // 3. Unscaled 9-bit signed offset: LDUR/STUR GPR/SIMD (0x38/0x3C) or LDAPUR/STLUR (0x19).
                else if (insn & 0x3B20_0C00) == 0x3800_0000 || (insn & 0x3F20_0C00) == 0x1900_0000 {
                    let simm9 = (((insn >> 12) & 0x1FF) as i32) << 23 >> 23;
                    base.wrapping_add(simm9 as i64 as u64) as usize
                }
                // 4. Plain [Xn] base register: LDAR/LDAPR/STLR.
                else if (insn & 0x3FFF_FC00) == 0x08DF_FC00
                    || (insn & 0x3FFF_FC00) == 0x38BF_C000
                    || (insn & 0x3FFF_FC00) == 0x089F_FC00
                {
                    base as usize
                } else {
                    return None;
                }
            }
        };
        let f = fault_addr as usize;
        if f >= ea && f < ea.saturating_add(total) {
            Some(ea)
        } else {
            None
        }
    }
}

/// Decode a single AArch64 general-purpose store instruction (no writeback).
/// Returns `(rt, bytes)`.
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

/// Decode an AArch64 scalar GPR, SIMD&FP (`B`/`H`/`S`/`D`/`Q`), or non-writeback
/// pair (`LDP`/`STP`) load or store instruction.
#[cfg(target_arch = "aarch64")]
fn decode_arm64_mem_op(insn: u32) -> Option<Arm64MemOp> {
    // 1. SIMD&FP single-register unsigned-offset (`LDR`/`STR {B,H,S,D,Q}t, [Xn, #pimm]`)
    //    or unscaled/register-offset without writeback (`LDUR`/`STUR`, `LDR`/`STR [Xn, Xm]`).
    let is_simd_ui = (insn & 0x3F00_0000) == 0x3D00_0000;
    let is_simd_ur_or_reg = (insn & 0x3F00_0000) == 0x3C00_0000
        && ((insn & 0x0020_0C00) == 0x0000_0000 || (insn & 0x0020_0C00) == 0x0020_0800);
    if is_simd_ui || is_simd_ur_or_reg {
        let size = (insn >> 30) & 3;
        let opc = (insn >> 22) & 3;
        let vt = (insn & 0x1F) as usize;
        return match (size, opc) {
            (0, 0) => Some(Arm64MemOp::SimdStore { vt, bytes: 1 }),
            (0, 1) => Some(Arm64MemOp::SimdLoad { vt, bytes: 1 }),
            (0, 2) => Some(Arm64MemOp::SimdStore { vt, bytes: 16 }),
            (0, 3) => Some(Arm64MemOp::SimdLoad { vt, bytes: 16 }),
            (1, 0) => Some(Arm64MemOp::SimdStore { vt, bytes: 2 }),
            (1, 1) => Some(Arm64MemOp::SimdLoad { vt, bytes: 2 }),
            (2, 0) => Some(Arm64MemOp::SimdStore { vt, bytes: 4 }),
            (2, 1) => Some(Arm64MemOp::SimdLoad { vt, bytes: 4 }),
            (3, 0) => Some(Arm64MemOp::SimdStore { vt, bytes: 8 }),
            (3, 1) => Some(Arm64MemOp::SimdLoad { vt, bytes: 8 }),
            _ => None,
        };
    }

    // 2. Non-writeback SIMD&FP signed-offset pair `LDP`/`STP {S,D,Q}t1, t2, [Xn, #imm]`.
    if (insn & 0x3F80_0000) == 0x2D00_0000 {
        let opc = (insn >> 30) & 3;
        let elem_bytes = match opc {
            0 => 4usize,
            1 => 8,
            2 => 16,
            _ => return None,
        };
        let is_load = ((insn >> 22) & 1) != 0;
        let imm7 = (((insn >> 15) & 0x7F) as i32) << 25 >> 25;
        let imm = (imm7 as i64) * (elem_bytes as i64);
        let vt2 = ((insn >> 10) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let vt1 = (insn & 0x1F) as usize;
        return Some(if is_load {
            Arm64MemOp::SimdPairLoad { vt1, vt2, rn, imm, elem_bytes }
        } else {
            Arm64MemOp::SimdPairStore { vt1, vt2, rn, imm, elem_bytes }
        });
    }

    // 3. Non-writeback GPR signed-offset pair `LDP`/`STP`/`LDPSW {W,X}t1, t2, [Xn, #imm]`.
    if (insn & 0x3F80_0000) == 0x2900_0000 {
        let opc = (insn >> 30) & 3;
        let is_load = ((insn >> 22) & 1) != 0;
        let (elem_bytes, signed_64) = match (opc, is_load) {
            (0, _) => (4usize, false),
            (1, true) => (4usize, true),
            (2, _) => (8usize, false),
            _ => return None,
        };
        let imm7 = (((insn >> 15) & 0x7F) as i32) << 25 >> 25;
        let imm = (imm7 as i64) * (elem_bytes as i64);
        let rt2 = ((insn >> 10) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rt1 = (insn & 0x1F) as usize;
        return Some(if is_load {
            Arm64MemOp::GprPairLoad { rt1, rt2, rn, imm, elem_bytes, signed_64 }
        } else {
            Arm64MemOp::GprPairStore { rt1, rt2, rn, imm, elem_bytes }
        });
    }

    // 4. ARMv8.1 LSE atomics: LDADD/LDCLR/LDEOR/LDSET/SWP, CAS, and CASP.
    if (insn & 0x3F20_0C00) == 0x3820_0000 {
        let op = match (insn >> 12) & 0xF {
            0 => Some(AtomicRmwOp::Add),
            1 => Some(AtomicRmwOp::Clr),
            2 => Some(AtomicRmwOp::Eor),
            3 => Some(AtomicRmwOp::Set),
            8 => Some(AtomicRmwOp::Swp),
            _ => None,
        };
        if let Some(op) = op {
            let size = ((insn >> 30) & 3) as usize;
            let bytes = 1usize << size;
            let rs = ((insn >> 16) & 0x1F) as usize;
            let rt = (insn & 0x1F) as usize;
            return Some(Arm64MemOp::AtomicRmw { op, rs, rt, bytes });
        }
    }
    if (insn & 0x3FA0_7C00) == 0x08A0_7C00 {
        let size = ((insn >> 30) & 3) as usize;
        let bytes = 1usize << size;
        let rs = ((insn >> 16) & 0x1F) as usize;
        let rt = (insn & 0x1F) as usize;
        return Some(Arm64MemOp::AtomicCas { rs, rt, bytes });
    }
    if (insn & 0xBFA0_7C00) == 0x0820_7C00 {
        let elem_bytes = if ((insn >> 30) & 1) != 0 { 8usize } else { 4 };
        let rs = ((insn >> 16) & 0x1F) as usize;
        let rt = (insn & 0x1F) as usize;
        if (rs & 1) == 0 && (rt & 1) == 0 && rs < 30 && rt < 30 {
            return Some(Arm64MemOp::AtomicCasp { rs, rt, elem_bytes });
        }
    }

    // 5. Scalar GPR load or store.
    if let Some((rt, bytes)) = decode_arm64_store(insn) {
        return Some(Arm64MemOp::GprStore { rt, bytes });
    }
    if let Some(ld) = decode_arm64_load(insn) {
        return Some(Arm64MemOp::GprLoad {
            rt: ld.rt,
            bytes: ld.bytes,
            signed_64: ld.signed_64,
            signed_32: ld.signed_32,
        });
    }
    None
}

/// Decode the `(start_addr, byte_len, requires_write)` of the faulting AArch64 memory instruction at `pc`.
#[cfg(target_arch = "aarch64")]
unsafe fn decoded_access_span(fault_addr: u64, pc: u64, esr_is_write: bool, ss: *const u64) -> Option<(usize, usize, bool)> {
    if pc < 0x10000 || (pc & 3) != 0 {
        return None;
    }
    let insn = std::ptr::read_unaligned(pc as *const u32);
    let op = decode_arm64_mem_op(insn)?;
    if !op.matches_esr_write(esr_is_write) {
        return None;
    }
    let ea = op.effective_addr(insn, fault_addr, ss)?;
    Some((ea, op.total_bytes(), op.requires_write()))
}

/// Returns `true` iff the 4 KiB page containing `addr` is logically committed and
/// permits the requested access (`read` or `write`) without `PAGE_NOACCESS`/`PAGE_GUARD`.
#[cfg(target_arch = "aarch64")]
fn subpage_allows_access(addr: usize, is_write: bool) -> bool {
    use win32_rt::kernel32::memory::{
        self, MEM_COMMIT, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS,
        PAGE_READWRITE, PAGE_WRITECOPY,
    };
    let Some(mbi) = memory::query(addr) else {
        return false;
    };
    if mbi.state != MEM_COMMIT || (mbi.protect & 0xFF) == PAGE_NOACCESS || (mbi.protect & PAGE_GUARD) != 0 {
        return false;
    }
    if is_write {
        matches!(
            mbi.protect & 0xFF,
            PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY
        )
    } else {
        true
    }
}

/// Returns the first 4 KiB subpage in `[start_addr, start_addr + len)` that does not allow
/// the requested access, or `None` if every touched subpage permits it.
#[cfg(target_arch = "aarch64")]
fn first_blocked_subpage(start_addr: usize, len: usize, is_write: bool) -> Option<usize> {
    let end_addr = start_addr.checked_add(len.checked_sub(1)?)?;
    let mut p = start_addr & !0xFFF;
    let last_p = end_addr & !0xFFF;
    while p <= last_p {
        if !subpage_allows_access(p, is_write) {
            return Some(p);
        }
        p = match p.checked_add(0x1000) {
            Some(next) => next,
            None => break,
        };
    }
    None
}

/// Transfer `buf` bytes at guest address `[start_addr, start_addr + buf.len())` through an
/// isolated ephemeral `mach_vm_remap` alias mapping so the guest virtual address stays
/// strictly `PROT_NONE` for the entire operation — concurrent threads touching `PAGE_GUARD`
/// or `PAGE_NOACCESS` sibling subpages on the same 16 KiB host page are never exposed to a
/// transient readable/writable window.
#[cfg(target_arch = "aarch64")]
unsafe fn isolated_hpage_transfer(start_addr: usize, buf: &mut [u8], is_write: bool) -> bool {
    extern "C" {
        static mach_task_self_: libc::mach_port_t;
        fn mach_vm_remap(
            target_task: libc::mach_port_t,
            target_address: *mut u64,
            size: u64,
            mask: u64,
            flags: i32,
            src_task: libc::mach_port_t,
            src_address: u64,
            copy: i32,
            cur_protection: *mut i32,
            max_protection: *mut i32,
            inheritance: i32,
        ) -> i32;
        fn mach_vm_protect(
            target_task: libc::mach_port_t,
            address: u64,
            size: u64,
            set_maximum: i32,
            new_protection: i32,
        ) -> i32;
        fn mach_vm_deallocate(
            target_task: libc::mach_port_t,
            address: u64,
            size: u64,
        ) -> i32;
    }
    const VM_FLAGS_ANYWHERE: i32 = 1;
    const VM_INHERIT_NONE: i32 = 2;
    const VM_PROT_READ: i32 = 1;
    const VM_PROT_WRITE: i32 = 2;

    let task = mach_task_self_;
    let len = buf.len();
    let mut done = 0usize;
    while done < len {
        let cur_addr = start_addr + done;
        let hpage = cur_addr & !0x3FFF;
        let off = cur_addr - hpage;
        let chunk = (len - done).min(0x4000 - off);

        let mut alias: u64 = 0;
        let mut cur_prot: i32 = 0;
        let mut max_prot: i32 = 0;
        let kr = mach_vm_remap(
            task,
            &mut alias,
            0x4000,
            0,
            VM_FLAGS_ANYWHERE,
            task,
            hpage as u64,
            0, // copy = FALSE: share the exact underlying physical page frame.
            &mut cur_prot,
            &mut max_prot,
            VM_INHERIT_NONE,
        );
        if kr != 0 {
            return false;
        }
        let want_prot = if is_write { VM_PROT_READ | VM_PROT_WRITE } else { VM_PROT_READ };
        if mach_vm_protect(task, alias, 0x4000, 0, want_prot) != 0 {
            let _ = mach_vm_deallocate(task, alias, 0x4000);
            return false;
        }
        let alias_ptr = (alias as usize + off) as *mut u8;
        if is_write {
            core::sync::atomic::fence(Ordering::Release);
            std::ptr::copy_nonoverlapping(buf.as_ptr().add(done), alias_ptr, chunk);
        } else {
            std::ptr::copy_nonoverlapping(alias_ptr as *const u8, buf.as_mut_ptr().add(done), chunk);
            core::sync::atomic::fence(Ordering::Acquire);
        }
        let _ = mach_vm_deallocate(task, alias, 0x4000);
        done += chunk;
    }
    true
}

/// Execute an ARMv8.1 LSE atomic operation (`LDADD`/`LDCLR`/`LDEOR`/`LDSET`/`SWP`,
/// `CAS`, or `CASP`) at guest address `start_addr` through an isolated `mach_vm_remap`
/// alias so the guest VA stays strictly `PROT_NONE` throughout.
///
/// Naturally aligned single-host-page atomics execute real hardware atomic instructions
/// (`AtomicU{8,16,32,64}` / `caspal`) directly on the shared physical page frame through
/// the alias; unaligned or cross-host-page atomics serialize under `UNALIGNED_ATOMIC_LOCK`.
#[cfg(target_arch = "aarch64")]
unsafe fn isolated_hpage_atomic(start_addr: usize, op: Arm64MemOp, ss: *mut u64) -> bool {
    use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8};
    extern "C" {
        static mach_task_self_: libc::mach_port_t;
        fn mach_vm_remap(
            target_task: libc::mach_port_t,
            target_address: *mut u64,
            size: u64,
            mask: u64,
            flags: i32,
            src_task: libc::mach_port_t,
            src_address: u64,
            copy: i32,
            cur_protection: *mut i32,
            max_protection: *mut i32,
            inheritance: i32,
        ) -> i32;
        fn mach_vm_protect(
            target_task: libc::mach_port_t,
            address: u64,
            size: u64,
            set_maximum: i32,
            new_protection: i32,
        ) -> i32;
        fn mach_vm_deallocate(target_task: libc::mach_port_t, address: u64, size: u64) -> i32;
    }
    const VM_FLAGS_ANYWHERE: i32 = 1;
    const VM_INHERIT_NONE: i32 = 2;
    const VM_PROT_RW: i32 = 1 | 2;

    let total_bytes = op.total_bytes();
    let end_addr = match start_addr.checked_add(total_bytes - 1) {
        Some(e) => e,
        None => return false,
    };
    let hpage = start_addr & !0x3FFF;
    let aligned = (start_addr & (total_bytes - 1)) == 0 && (end_addr & !0x3FFF) == hpage;

    if aligned {
        let task = mach_task_self_;
        let mut alias: u64 = 0;
        let mut cur_prot: i32 = 0;
        let mut max_prot: i32 = 0;
        if mach_vm_remap(
            task,
            &mut alias,
            0x4000,
            0,
            VM_FLAGS_ANYWHERE,
            task,
            hpage as u64,
            0,
            &mut cur_prot,
            &mut max_prot,
            VM_INHERIT_NONE,
        ) != 0
        {
            return false;
        }
        if mach_vm_protect(task, alias, 0x4000, 0, VM_PROT_RW) != 0 {
            let _ = mach_vm_deallocate(task, alias, 0x4000);
            return false;
        }
        let ptr = (alias as usize + (start_addr - hpage)) as *mut u8;
        match op {
            Arm64MemOp::AtomicRmw { op: rmw, rs, rt, bytes } => {
                let val = if rs == 31 { 0u64 } else { *ss.add(rs) };
                let old: u64 = match bytes {
                    1 => {
                        let a = &*(ptr as *const AtomicU8);
                        let v = val as u8;
                        (match rmw {
                            AtomicRmwOp::Add => a.fetch_add(v, Ordering::SeqCst),
                            AtomicRmwOp::Clr => a.fetch_and(!v, Ordering::SeqCst),
                            AtomicRmwOp::Eor => a.fetch_xor(v, Ordering::SeqCst),
                            AtomicRmwOp::Set => a.fetch_or(v, Ordering::SeqCst),
                            AtomicRmwOp::Swp => a.swap(v, Ordering::SeqCst),
                        }) as u64
                    }
                    2 => {
                        let a = &*(ptr as *const AtomicU16);
                        let v = val as u16;
                        (match rmw {
                            AtomicRmwOp::Add => a.fetch_add(v, Ordering::SeqCst),
                            AtomicRmwOp::Clr => a.fetch_and(!v, Ordering::SeqCst),
                            AtomicRmwOp::Eor => a.fetch_xor(v, Ordering::SeqCst),
                            AtomicRmwOp::Set => a.fetch_or(v, Ordering::SeqCst),
                            AtomicRmwOp::Swp => a.swap(v, Ordering::SeqCst),
                        }) as u64
                    }
                    4 => {
                        let a = &*(ptr as *const AtomicU32);
                        let v = val as u32;
                        (match rmw {
                            AtomicRmwOp::Add => a.fetch_add(v, Ordering::SeqCst),
                            AtomicRmwOp::Clr => a.fetch_and(!v, Ordering::SeqCst),
                            AtomicRmwOp::Eor => a.fetch_xor(v, Ordering::SeqCst),
                            AtomicRmwOp::Set => a.fetch_or(v, Ordering::SeqCst),
                            AtomicRmwOp::Swp => a.swap(v, Ordering::SeqCst),
                        }) as u64
                    }
                    8 => {
                        let a = &*(ptr as *const AtomicU64);
                        match rmw {
                            AtomicRmwOp::Add => a.fetch_add(val, Ordering::SeqCst),
                            AtomicRmwOp::Clr => a.fetch_and(!val, Ordering::SeqCst),
                            AtomicRmwOp::Eor => a.fetch_xor(val, Ordering::SeqCst),
                            AtomicRmwOp::Set => a.fetch_or(val, Ordering::SeqCst),
                            AtomicRmwOp::Swp => a.swap(val, Ordering::SeqCst),
                        }
                    }
                    _ => {
                        let _ = mach_vm_deallocate(task, alias, 0x4000);
                        return false;
                    }
                };
                if rt < 31 {
                    *ss.add(rt) = old;
                }
            }
            Arm64MemOp::AtomicCas { rs, rt, bytes } => {
                let exp = if rs == 31 { 0u64 } else { *ss.add(rs) };
                let rep = if rt == 31 { 0u64 } else { *ss.add(rt) };
                let old: u64 = match bytes {
                    1 => {
                        let a = &*(ptr as *const AtomicU8);
                        match a.compare_exchange(exp as u8, rep as u8, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(o) | Err(o) => o as u64,
                        }
                    }
                    2 => {
                        let a = &*(ptr as *const AtomicU16);
                        match a.compare_exchange(exp as u16, rep as u16, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(o) | Err(o) => o as u64,
                        }
                    }
                    4 => {
                        let a = &*(ptr as *const AtomicU32);
                        match a.compare_exchange(exp as u32, rep as u32, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(o) | Err(o) => o as u64,
                        }
                    }
                    8 => {
                        let a = &*(ptr as *const AtomicU64);
                        match a.compare_exchange(exp, rep, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(o) | Err(o) => o,
                        }
                    }
                    _ => {
                        let _ = mach_vm_deallocate(task, alias, 0x4000);
                        return false;
                    }
                };
                if rs < 31 {
                    *ss.add(rs) = old;
                }
            }
            Arm64MemOp::AtomicCasp { rs, rt, elem_bytes } => {
                if elem_bytes == 4 {
                    let exp = (*ss.add(rs) as u32 as u64) | ((*ss.add(rs + 1) as u32 as u64) << 32);
                    let rep = (*ss.add(rt) as u32 as u64) | ((*ss.add(rt + 1) as u32 as u64) << 32);
                    let a = &*(ptr as *const AtomicU64);
                    let old = match a.compare_exchange(exp, rep, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(o) | Err(o) => o,
                    };
                    *ss.add(rs) = (old as u32) as u64;
                    *ss.add(rs + 1) = ((old >> 32) as u32) as u64;
                } else {
                    let mut lo = *ss.add(rs);
                    let mut hi = *ss.add(rs + 1);
                    let rep_lo = *ss.add(rt);
                    let rep_hi = *ss.add(rt + 1);
                    core::arch::asm!(
                        "caspal x0, x1, x2, x3, [{p}]",
                        p = in(reg) ptr,
                        inout("x0") lo,
                        inout("x1") hi,
                        in("x2") rep_lo,
                        in("x3") rep_hi,
                        options(nostack),
                    );
                    *ss.add(rs) = lo;
                    *ss.add(rs + 1) = hi;
                }
            }
            _ => {
                let _ = mach_vm_deallocate(task, alias, 0x4000);
                return false;
            }
        }
        let _ = mach_vm_deallocate(task, alias, 0x4000);
        return true;
    }

    // Unaligned or cross-host-page atomic: serialize via lock and transfer through isolated aliases.
    static UNALIGNED_ATOMIC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _g = UNALIGNED_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut cur = [0u8; 16];
    if !isolated_hpage_transfer(start_addr, &mut cur[..total_bytes], false) {
        return false;
    }
    let mut next = cur;
    match op {
        Arm64MemOp::AtomicRmw { op: rmw, rs, rt, bytes } => {
            let val = if rs == 31 { 0u64 } else { *ss.add(rs) };
            let mut old_arr = [0u8; 8];
            old_arr[..bytes].copy_from_slice(&cur[..bytes]);
            let old = u64::from_le_bytes(old_arr);
            let mask = if bytes == 8 { u64::MAX } else { (1u64 << (bytes * 8)) - 1 };
            let res = match rmw {
                AtomicRmwOp::Add => old.wrapping_add(val) & mask,
                AtomicRmwOp::Clr => (old & !val) & mask,
                AtomicRmwOp::Eor => (old ^ val) & mask,
                AtomicRmwOp::Set => (old | val) & mask,
                AtomicRmwOp::Swp => val & mask,
            };
            next[..bytes].copy_from_slice(&res.to_le_bytes()[..bytes]);
            if !isolated_hpage_transfer(start_addr, &mut next[..bytes], true) {
                return false;
            }
            if rt < 31 {
                *ss.add(rt) = old;
            }
        }
        Arm64MemOp::AtomicCas { rs, rt, bytes } => {
            let mask = if bytes == 8 { u64::MAX } else { (1u64 << (bytes * 8)) - 1 };
            let exp = (if rs == 31 { 0u64 } else { *ss.add(rs) }) & mask;
            let rep = (if rt == 31 { 0u64 } else { *ss.add(rt) }) & mask;
            let mut old_arr = [0u8; 8];
            old_arr[..bytes].copy_from_slice(&cur[..bytes]);
            let old = u64::from_le_bytes(old_arr);
            if old == exp {
                next[..bytes].copy_from_slice(&rep.to_le_bytes()[..bytes]);
                if !isolated_hpage_transfer(start_addr, &mut next[..bytes], true) {
                    return false;
                }
            }
            if rs < 31 {
                *ss.add(rs) = old;
            }
        }
        Arm64MemOp::AtomicCasp { rs, rt, elem_bytes } => {
            if elem_bytes == 4 {
                let old_lo = u32::from_le_bytes(cur[0..4].try_into().unwrap());
                let old_hi = u32::from_le_bytes(cur[4..8].try_into().unwrap());
                if old_lo == *ss.add(rs) as u32 && old_hi == *ss.add(rs + 1) as u32 {
                    next[0..4].copy_from_slice(&(*ss.add(rt) as u32).to_le_bytes());
                    next[4..8].copy_from_slice(&(*ss.add(rt + 1) as u32).to_le_bytes());
                    if !isolated_hpage_transfer(start_addr, &mut next[..8], true) {
                        return false;
                    }
                }
                *ss.add(rs) = old_lo as u64;
                *ss.add(rs + 1) = old_hi as u64;
            } else {
                let old_lo = u64::from_le_bytes(cur[0..8].try_into().unwrap());
                let old_hi = u64::from_le_bytes(cur[8..16].try_into().unwrap());
                if old_lo == *ss.add(rs) && old_hi == *ss.add(rs + 1) {
                    next[0..8].copy_from_slice(&(*ss.add(rt)).to_le_bytes());
                    next[8..16].copy_from_slice(&(*ss.add(rt + 1)).to_le_bytes());
                    if !isolated_hpage_transfer(start_addr, &mut next[..16], true) {
                        return false;
                    }
                }
                *ss.add(rs) = old_lo;
                *ss.add(rs + 1) = old_hi;
            }
        }
        _ => return false,
    }
    true
}

/// Emulate one AArch64 GPR, SIMD&FP, pair, or LSE atomic instruction against logically
/// accessible 4 KiB subpage(s) inside a 16 KiB host page kept `PROT_NONE` for a
/// `PAGE_GUARD`/`PAGE_NOACCESS` sibling. Validates EVERY 4 KiB subpage touched by
/// `[ea, ea + total_bytes)` and performs the memory transfer through an isolated
/// `mach_vm_remap` alias so the guest VA remains `PROT_NONE` throughout.
/// Caller MUST hold `HPAGE_LOCKS[hpage]`.
#[cfg(target_arch = "aarch64")]
unsafe fn try_emulate_subpage_access(fault_addr: u64, esr_is_write: bool, mc: *mut u8, ss: *mut u64) -> bool {
    let pc = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
    if pc < 0x10000 || (pc & 3) != 0 {
        return false;
    }
    let insn = std::ptr::read_unaligned(pc as *const u32);
    let Some(op) = decode_arm64_mem_op(insn) else {
        return false;
    };
    if !op.matches_esr_write(esr_is_write) {
        return false;
    }
    let is_write = op.requires_write();
    let Some(start_addr) = op.effective_addr(insn, fault_addr, ss) else {
        return false;
    };
    let total_bytes = op.total_bytes();
    if total_bytes == 0 || total_bytes > 32 {
        return false;
    }
    if first_blocked_subpage(start_addr, total_bytes, is_write).is_some() {
        return false;
    }

    if op.is_atomic() {
        if !isolated_hpage_atomic(start_addr, op, ss) {
            return false;
        }
        let end_addr = start_addr + total_bytes - 1;
        win32_rt::kernel32::memory::mark_write_watch_dirty(start_addr);
        if (end_addr & !0x3FFF) != (start_addr & !0x3FFF) {
            win32_rt::kernel32::memory::mark_write_watch_dirty(end_addr);
        }
        *ss.add(32) = (*ss.add(32) & !0x0000_FFFF_FFFF_FFFF) | pc.wrapping_add(4);
        return true;
    }

    let ns_v = mc.add(MC_NS_ARM64) as *mut u128;
    let mut buf = [0u8; 32];

    if is_write {
        match op {
            Arm64MemOp::GprStore { rt, bytes } => {
                let val = if rt == 31 { 0u64 } else { *ss.add(rt) };
                buf[..bytes].copy_from_slice(&val.to_le_bytes()[..bytes]);
            }
            Arm64MemOp::SimdStore { vt, bytes } => {
                let v = std::ptr::read_unaligned(ns_v.add(vt)).to_le_bytes();
                buf[..bytes].copy_from_slice(&v[..bytes]);
            }
            Arm64MemOp::GprPairStore { rt1, rt2, elem_bytes, .. } => {
                let v1 = if rt1 == 31 { 0u64 } else { *ss.add(rt1) };
                let v2 = if rt2 == 31 { 0u64 } else { *ss.add(rt2) };
                buf[..elem_bytes].copy_from_slice(&v1.to_le_bytes()[..elem_bytes]);
                buf[elem_bytes..elem_bytes * 2].copy_from_slice(&v2.to_le_bytes()[..elem_bytes]);
            }
            Arm64MemOp::SimdPairStore { vt1, vt2, elem_bytes, .. } => {
                let v1 = std::ptr::read_unaligned(ns_v.add(vt1)).to_le_bytes();
                let v2 = std::ptr::read_unaligned(ns_v.add(vt2)).to_le_bytes();
                buf[..elem_bytes].copy_from_slice(&v1[..elem_bytes]);
                buf[elem_bytes..elem_bytes * 2].copy_from_slice(&v2[..elem_bytes]);
            }
            _ => return false,
        }
        if !isolated_hpage_transfer(start_addr, &mut buf[..total_bytes], true) {
            return false;
        }
        let end_addr = start_addr + total_bytes - 1;
        win32_rt::kernel32::memory::mark_write_watch_dirty(start_addr);
        if (end_addr & !0x3FFF) != (start_addr & !0x3FFF) {
            win32_rt::kernel32::memory::mark_write_watch_dirty(end_addr);
        }
    } else {
        if !isolated_hpage_transfer(start_addr, &mut buf[..total_bytes], false) {
            return false;
        }
        match op {
            Arm64MemOp::GprLoad { rt, bytes, signed_64, signed_32 } => {
                let val: u64 = match (bytes, signed_64, signed_32) {
                    (1, false, false) => buf[0] as u64,
                    (1, true, false) => (buf[0] as i8) as i64 as u64,
                    (1, false, true) => (buf[0] as i8) as i32 as u32 as u64,
                    (2, false, false) => u16::from_le_bytes([buf[0], buf[1]]) as u64,
                    (2, true, false) => i16::from_le_bytes([buf[0], buf[1]]) as i64 as u64,
                    (2, false, true) => i16::from_le_bytes([buf[0], buf[1]]) as i32 as u32 as u64,
                    (4, false, false) => u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64,
                    (4, true, false) => i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as i64 as u64,
                    (8, false, false) => u64::from_le_bytes(buf[..8].try_into().unwrap()),
                    _ => return false,
                };
                if rt < 31 {
                    *ss.add(rt) = val;
                }
            }
            Arm64MemOp::SimdLoad { vt, bytes } => {
                let mut v128 = [0u8; 16];
                v128[..bytes].copy_from_slice(&buf[..bytes]);
                std::ptr::write_unaligned(ns_v.add(vt), u128::from_le_bytes(v128));
            }
            Arm64MemOp::GprPairLoad { rt1, rt2, elem_bytes, signed_64, .. } => {
                let decode_elem = |slice: &[u8]| -> u64 {
                    match (elem_bytes, signed_64) {
                        (4, false) => u32::from_le_bytes(slice.try_into().unwrap()) as u64,
                        (4, true) => i32::from_le_bytes(slice.try_into().unwrap()) as i64 as u64,
                        (8, false) => u64::from_le_bytes(slice.try_into().unwrap()),
                        _ => 0,
                    }
                };
                let v1 = decode_elem(&buf[..elem_bytes]);
                let v2 = decode_elem(&buf[elem_bytes..elem_bytes * 2]);
                if rt1 < 31 {
                    *ss.add(rt1) = v1;
                }
                if rt2 < 31 {
                    *ss.add(rt2) = v2;
                }
            }
            Arm64MemOp::SimdPairLoad { vt1, vt2, elem_bytes, .. } => {
                let mut v1 = [0u8; 16];
                let mut v2 = [0u8; 16];
                v1[..elem_bytes].copy_from_slice(&buf[..elem_bytes]);
                v2[..elem_bytes].copy_from_slice(&buf[elem_bytes..elem_bytes * 2]);
                std::ptr::write_unaligned(ns_v.add(vt1), u128::from_le_bytes(v1));
                std::ptr::write_unaligned(ns_v.add(vt2), u128::from_le_bytes(v2));
            }
            _ => return false,
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
unsafe fn try_dispatch_guest_page_fault(fault_addr: u64, pc: u64, is_write: bool, mc: *mut u8, ss: *mut u64) -> bool {
    use win32_rt::kernel32::memory::{self, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS};
    thread_local! {
        static IN_FAULT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        static LAST_RACE_RETRY: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
    }
    if IN_FAULT.with(|f| f.get()) {
        return false;
    }
    let (span_start, span_bytes, is_write) =
        decoded_access_span(fault_addr, pc, is_write, ss).unwrap_or((fault_addr as usize, 1, is_write));
    // If an unaligned multi-byte scalar or SIMD access starts on an accessible 4 KiB subpage
    // and spans into a protected or non-writable sibling 4 KiB subpage, report and handle the
    // fault at that protected subpage boundary (matching x86-64 hardware CR2).
    let eff_fault_addr = match first_blocked_subpage(span_start, span_bytes, is_write) {
        Some(blocked_p) if blocked_p != (span_start & !0xFFF) => blocked_p as u64,
        _ => fault_addr,
    };
    let eff_addr = eff_fault_addr as usize;
    let hpage = eff_addr & !0x3FFF;
    let sub_is_protected = |p: usize| -> bool {
        memory::query(p).is_some_and(|m| {
            m.state == MEM_COMMIT && (m.protect == PAGE_NOACCESS || (m.protect & PAGE_GUARD) != 0)
        })
    };
    let Some(initial_mbi) = memory::query(eff_addr) else {
        return false;
    };
    if initial_mbi.state != MEM_COMMIT {
        return false;
    }
    let initial_fault_prot =
        !subpage_allows_access(eff_addr, is_write) || initial_mbi.protect == PAGE_NOACCESS || (initial_mbi.protect & PAGE_GUARD) != 0;
    let initial_any_sibling_prot = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
    // If every 4 KiB subpage touched by the access is logically accessible but shares a
    // `PROT_NONE` 16 KiB host page with a `PAGE_GUARD` or non-guest `PAGE_NOACCESS` sibling,
    // emulate the access transparently through an isolated alias mapping so sibling protections
    // stay intact even under concurrent threads.
    if !initial_fault_prot && initial_any_sibling_prot {
        let _hpage_guard = HPAGE_LOCKS[(hpage >> 14) & 63].lock().unwrap();
        if let Some(mbi) = memory::query(eff_addr) {
            let now_fault_prot =
                !subpage_allows_access(eff_addr, is_write) || mbi.protect == PAGE_NOACCESS || (mbi.protect & PAGE_GUARD) != 0;
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
                    && try_emulate_subpage_access(fault_addr, is_write, mc, ss)
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
            let key = (pc, eff_fault_addr);
            if LAST_RACE_RETRY.with(|c| c.get()) != key {
                let _hpage_guard = HPAGE_LOCKS[(hpage >> 14) & 63].lock().unwrap();
                if let Some(mbi) = memory::query(eff_addr) {
                    let still_prot = mbi.protect == PAGE_NOACCESS
                        || (mbi.protect & PAGE_GUARD) != 0
                        || (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
                    if mbi.state == MEM_COMMIT && !still_prot {
                        memory::ensure_host_page_readable(eff_addr);
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
    let Some(mbi) = memory::query(eff_addr) else {
        return false;
    };
    if mbi.state != MEM_COMMIT {
        return false;
    }
    let is_guard = (mbi.protect & PAGE_GUARD) != 0;
    let fault_protected = !subpage_allows_access(eff_addr, is_write) || mbi.protect == PAGE_NOACCESS || is_guard;
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
        let _consumed = memory::consume_guard_page(eff_addr);
        // Release HPAGE_LOCKS before dispatching the VEH on `PageFaultHelper` (a separate
        // OS thread). If the VEH accesses the just-consumed 4 KiB page while a sibling 4 KiB
        // subpage keeps the 16 KiB host page PROT_NONE, the helper thread faults and acquires
        // this same lock to emulate that access; holding it across dispatch would deadlock.
        drop(hpage_guard.take());
        handled = deliver_page_fault(unwind::STATUS_GUARD_PAGE_VIOLATION, eff_fault_addr, is_write);
        if handled {
            let _emul_guard = hpage_lock.lock().unwrap();
            let now_any_sibling = (0..4).any(|i| sub_is_protected(hpage + i * 0x1000));
            if now_any_sibling && first_blocked_subpage(span_start, span_bytes, is_write).is_none() {
                // Sibling 4 KiB subpages remain guarded/noaccess (host page stays PROT_NONE):
                // emulate this single faulting instruction through an isolated alias.
                handled = try_emulate_subpage_access(fault_addr, is_write, mc, ss);
            }
        }
        IN_FAULT.with(|f| f.set(false));
        return handled;
    }
    if fault_protected {
        let av_write = if in_guest(eff_addr) { false } else { is_write };
        handled = deliver_page_fault(unwind::STATUS_ACCESS_VIOLATION, eff_fault_addr, av_write);
        if !handled {
            IN_FAULT.with(|f| f.set(false));
            return false;
        }
    }
    // Unlock any sibling 4 KiB encrypted PAGE_NOACCESS sub-pages sharing this 16 KiB host page
    // (never eagerly consume sibling PAGE_GUARD pages).
    let fault_page4 = eff_addr & !0xFFF;
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
    } else if still_prot && in_guest(eff_addr) {
        memory::ensure_host_page_readable(eff_addr);
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
            if try_dispatch_guest_page_fault(clean_fault, pc, is_write, mc, ss) {
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

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn arm64_mem_op_decodes_simd_scalar_and_pair() {
        // Unsigned-offset SIMD LDR/STR Q, D, S, H, B
        assert!(matches!(decode_arm64_mem_op(0x3DC0_0003), Some(Arm64MemOp::SimdLoad { vt: 3, bytes: 16 })));
        assert!(matches!(decode_arm64_mem_op(0x3D80_0005), Some(Arm64MemOp::SimdStore { vt: 5, bytes: 16 })));
        assert!(matches!(decode_arm64_mem_op(0xFD40_0007), Some(Arm64MemOp::SimdLoad { vt: 7, bytes: 8 })));
        assert!(matches!(decode_arm64_mem_op(0xFD00_0008), Some(Arm64MemOp::SimdStore { vt: 8, bytes: 8 })));
        assert!(matches!(decode_arm64_mem_op(0xBD40_0009), Some(Arm64MemOp::SimdLoad { vt: 9, bytes: 4 })));
        assert!(matches!(decode_arm64_mem_op(0xBD00_000A), Some(Arm64MemOp::SimdStore { vt: 10, bytes: 4 })));
        // Unscaled LDUR/STUR Q, D, S
        assert!(matches!(decode_arm64_mem_op(0x3CC0_0004), Some(Arm64MemOp::SimdLoad { vt: 4, bytes: 16 })));
        assert!(matches!(decode_arm64_mem_op(0x3C80_0006), Some(Arm64MemOp::SimdStore { vt: 6, bytes: 16 })));
        assert!(matches!(decode_arm64_mem_op(0xFC40_000B), Some(Arm64MemOp::SimdLoad { vt: 11, bytes: 8 })));
        assert!(matches!(decode_arm64_mem_op(0xBC00_000C), Some(Arm64MemOp::SimdStore { vt: 12, bytes: 4 })));
        // Non-writeback pair LDP/STP Q and X
        assert!(matches!(
            decode_arm64_mem_op(0xAD40_0401),
            Some(Arm64MemOp::SimdPairLoad { vt1: 1, vt2: 1, elem_bytes: 16, .. })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xAD00_0802),
            Some(Arm64MemOp::SimdPairStore { vt1: 2, vt2: 2, elem_bytes: 16, .. })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xA940_0C03),
            Some(Arm64MemOp::GprPairLoad { rt1: 3, rt2: 3, elem_bytes: 8, signed_64: false, .. })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xA900_1004),
            Some(Arm64MemOp::GprPairStore { rt1: 4, rt2: 4, elem_bytes: 8, .. })
        ));
        // ARMv8.1 LSE atomics: ldaddal, ldclral, ldeoral, ldsetal, swpal, casal, caspal_x
        assert!(matches!(
            decode_arm64_mem_op(0xB8E1_0042),
            Some(Arm64MemOp::AtomicRmw { op: AtomicRmwOp::Add, rs: 1, rt: 2, bytes: 4 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xF8E3_1044),
            Some(Arm64MemOp::AtomicRmw { op: AtomicRmwOp::Clr, rs: 3, rt: 4, bytes: 8 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0x78E5_2046),
            Some(Arm64MemOp::AtomicRmw { op: AtomicRmwOp::Eor, rs: 5, rt: 6, bytes: 2 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0x38E7_3048),
            Some(Arm64MemOp::AtomicRmw { op: AtomicRmwOp::Set, rs: 7, rt: 8, bytes: 1 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xB8E9_804A),
            Some(Arm64MemOp::AtomicRmw { op: AtomicRmwOp::Swp, rs: 9, rt: 10, bytes: 4 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0xC8EB_FC4C),
            Some(Arm64MemOp::AtomicCas { rs: 11, rt: 12, bytes: 8 })
        ));
        assert!(matches!(
            decode_arm64_mem_op(0x4862_FC44),
            Some(Arm64MemOp::AtomicCasp { rs: 2, rt: 4, elem_bytes: 8 })
        ));

        // Verify effective_addr recovers the true starting address even when FAR_EL1
        // reports the second 16 KiB host-page boundary on a straddling access.
        let mut ss = [0u64; 33];
        ss[2] = 0x1000_3FF8; // base X2 = 8 bytes before 16 KiB boundary 0x1000_4000
        ss[3] = 0x10;        // index X3
        let far_boundary = 0x1000_4000u64;
        unsafe {
            // LDR Q1, [X2, #0] (0x3DC0_0041) straddling 0x1000_3FF8..0x1000_4007
            let op_q = decode_arm64_mem_op(0x3DC0_0041).unwrap();
            assert_eq!(op_q.effective_addr(0x3DC0_0041, far_boundary, ss.as_ptr()), Some(0x1000_3FF8));
            // LDUR Q1, [X2, #-8] (simm9 = -8 = 0x1F8, base = 0x1000_4000)
            ss[4] = 0x1000_4000;
            let ldur_q = 0x3CC0_0000 | (0x1F8 << 12) | (4 << 5) | 1;
            let op_ur = decode_arm64_mem_op(ldur_q).unwrap();
            assert_eq!(op_ur.effective_addr(ldur_q, far_boundary, ss.as_ptr()), Some(0x1000_3FF8));
            // LDR X1, [X4, X3, LSL #0] with X4 = 0x1000_3FEC, X3 = 0x10 -> 0x1000_3FFC (straddles 0x1000_4000)
            ss[4] = 0x1000_3FEC;
            let ldr_reg = 0xF860_6800 | (3 << 16) | (4 << 5) | 1;
            let op_reg = decode_arm64_mem_op(ldr_reg).unwrap();
            assert_eq!(op_reg.effective_addr(ldr_reg, far_boundary, ss.as_ptr()), Some(0x1000_3FFC));
            // Reject FAR outside [ea, ea + total_bytes)
            assert_eq!(op_q.effective_addr(0x3DC0_0041, 0x1000_4010, ss.as_ptr()), None);
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn isolated_hpage_transfer_keeps_guest_va_prot_none() {
        unsafe {
            // Allocate two contiguous 16 KiB host pages and lock them PROT_NONE.
            let raw = libc::mmap(
                std::ptr::null_mut(),
                0x8000,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(raw, libc::MAP_FAILED);
            assert_eq!(libc::mprotect(raw, 0x8000, libc::PROT_NONE), 0);

            // Write 16 bytes straddling the 16 KiB host-page boundary (0x3FF8..0x4007)
            // through isolated mach_vm_remap aliases without ever unprotecting `raw`.
            let mut pat = [0x10u8, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xA9, 0xBA, 0xCB, 0xDC, 0xED, 0xFE, 0x0F];
            let straddle = (raw as usize) + 0x3FF8;
            assert!(isolated_hpage_transfer(straddle, &mut pat, true));

            // Read back through isolated aliases and verify exact round-trip.
            let mut got = [0u8; 16];
            assert!(isolated_hpage_transfer(straddle, &mut got, false));
            assert_eq!(got, pat);

            // Verify via mach_vm_region that `raw` is STILL VM_PROT_NONE (0).
            #[repr(C)]
            #[derive(Default)]
            struct VmRegionBasicInfo64 {
                protection: i32,
                max_protection: i32,
                inheritance: u32,
                shared: u32,
                reserved: u32,
                offset: u64,
                behavior: i32,
                user_wired_count: u16,
            }
            extern "C" {
                static mach_task_self_: libc::mach_port_t;
                fn mach_vm_region(
                    task: libc::mach_port_t,
                    address: *mut u64,
                    size: *mut u64,
                    flavor: i32,
                    info: *mut i32,
                    count: *mut u32,
                    object_name: *mut libc::mach_port_t,
                ) -> i32;
            }
            let mut qaddr = raw as u64;
            let mut qsize = 0u64;
            let mut info = VmRegionBasicInfo64::default();
            let mut cnt = (std::mem::size_of::<VmRegionBasicInfo64>() / 4) as u32;
            let mut obj: libc::mach_port_t = 0;
            assert_eq!(
                mach_vm_region(mach_task_self_, &mut qaddr, &mut qsize, 9, &mut info as *mut _ as *mut i32, &mut cnt, &mut obj),
                0
            );
            assert_eq!(info.protection, 0, "guest VA must remain VM_PROT_NONE during and after isolated transfer");

            // Restore read permission and confirm bytes landed in the original mapping.
            assert_eq!(libc::mprotect(raw, 0x8000, libc::PROT_READ), 0);
            let mut direct = [0u8; 16];
            std::ptr::copy_nonoverlapping(straddle as *const u8, direct.as_mut_ptr(), 16);
            assert_eq!(direct, pat);

            libc::munmap(raw, 0x8000);
        }
    }
}
