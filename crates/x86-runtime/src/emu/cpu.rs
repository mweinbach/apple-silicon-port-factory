//! Guest x86-64 [`Cpu`] state, register/flag/memory/atomic accessors, hardware `MXCSR`↔`FPCR`
//! guards, precise fault dispatching, and scalar SSE numeric semantics.

use super::{eh, jit, mem, regfile_ext};
use crate::exe::{rd, teb_self, wr};
use iced_x86::{ConditionCode, EncodingKind, Instruction, OpKind, Register};

/// CPU state for the `iced-x86` interpreter.
///
/// GPR indexing:
/// `0: rax, 1: rcx, 2: rdx, 3: rbx, 4: rsp, 5: rbp, 6: rsi, 7: rdi, 8..16: r8..r15`.
///
/// `repr(C)`: the block translator ([`jit`]) addresses fields by offset.
#[derive(Clone)]
#[repr(C)]
pub struct Cpu {
    pub gpr: [u64; 16],
    pub xmm: [u128; 16],
    pub ymm_hi: [u128; 16],
    pub rip: u64,
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    /// Parity-flag source byte: PF is set iff this byte has an even number of 1 bits
    /// (stored lazily as the low result byte; see [`Cpu::pf`]).
    pub pfv: u8,
    /// Direction flag (STD/CLD); string instructions step backwards when set.
    pub df: bool,
    /// Width of the last write per GPR: 8 or 16 for partial writes, anything else (64, or
    /// 0 from translated code) means a full-width write. Only read by [`Cpu::abi_gpr`].
    pub last_bits: [u8; 16],
    /// Scratch area shared with translated code.
    pub(super) jx: jit::JitScratch,
}

/// Result of an OS entropy query (`getentropy`) returned to JIT/AOT code in `(x0, x1)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub(super) struct EntropyResult {
    /// 64 random bits on success, or `0` on failure.
    pub val: u64,
    /// `1` when entropy was produced (`CF = 1`), `0` on failure (`CF = 0`).
    pub ok: u64,
}

#[cfg(test)]
std::thread_local! {
    static TEST_ENTROPY: core::cell::Cell<Option<fn() -> EntropyResult>> = const { core::cell::Cell::new(None) };
}

/// Test-only: override `RDRAND`/`RDSEED` entropy generation on the current thread (for
/// deterministic cross-tier difftests and failure-path `CF = 0` testing).
#[cfg(test)]
pub(super) fn with_test_entropy<R>(hook: fn() -> EntropyResult, f: impl FnOnce() -> R) -> R {
    struct Restore(Option<fn() -> EntropyResult>);
    impl Drop for Restore {
        fn drop(&mut self) {
            TEST_ENTROPY.with(|slot| slot.set(self.0));
        }
    }
    let _restore = Restore(TEST_ENTROPY.with(|slot| slot.replace(Some(hook))));
    f()
}

extern "C" {
    fn getentropy(buf: *mut core::ffi::c_void, buflen: usize) -> core::ffi::c_int;
}

/// Query real OS cryptographic entropy (`getentropy`) for `RDRAND`/`RDSEED`.
/// Apple Silicon M-series does not expose ARM `FEAT_RNG` (`RNDR`), so Darwin's
/// `getentropy(2)` (backed by the kernel CSPRNG / hardware TRNG) is the native source.
pub(super) extern "C" fn guest_entropy_raw() -> EntropyResult {
    #[cfg(test)]
    if let Some(hook) = TEST_ENTROPY.with(|slot| slot.get()) {
        let res = hook();
        return if res.ok != 0 {
            EntropyResult { val: res.val, ok: 1 }
        } else {
            EntropyResult { val: 0, ok: 0 }
        };
    }
    let mut buf = [0u8; 8];
    let rc = unsafe { getentropy(buf.as_mut_ptr() as *mut core::ffi::c_void, buf.len()) };
    if rc == 0 {
        EntropyResult { val: u64::from_le_bytes(buf), ok: 1 }
    } else {
        EntropyResult { val: 0, ok: 0 }
    }
}

impl Cpu {
    pub(super) fn exec_entropy(&mut self, r0: u8, w: u8) {
        let res = guest_entropy_raw();
        let val = if res.ok != 0 { res.val } else { 0 };
        let i = r0 as usize & 15;
        match w {
            16 => self.gpr[i] = (self.gpr[i] & !0xFFFF) | (val & 0xFFFF),
            32 => self.gpr[i] = val & 0xFFFF_FFFF,
            _ => self.gpr[i] = val,
        }
        self.last_bits[i] = w;
        self.cf = res.ok != 0;
        self.zf = false;
        self.sf = false;
        self.of = false;
        self.set_pf(false);
    }

    pub(super) fn read_timestamp(&mut self, ordered: bool) {
        let t = guest_tsc(ordered);
        self.gpr[0] = u64::from(t as u32);
        self.gpr[2] = t >> 32;
        self.last_bits[0] = 64;
        self.last_bits[2] = 64;
        if ordered {
            self.gpr[1] = 0; // IA32_TSC_AUX
            self.last_bits[1] = 64;
        }
    }

    /// Current guest x86-64 `MXCSR` register value.
    #[inline(always)]
    pub(crate) fn mxcsr(&self) -> u32 {
        self.jx.mxcsr
    }

    /// Write `self.ymm_hi[d] = v` and maintain the `self.jx.ymm_dirty` bitmask incrementally.
    #[inline(always)]
    pub(super) fn set_ymm_hi(&mut self, d: usize, v: u128) {
        self.ymm_hi[d] = v;
        if v != 0 {
            self.jx.ymm_dirty |= 1u16 << d;
        } else {
            self.jx.ymm_dirty &= !(1u16 << d);
        }
    }

    /// Zero `self.ymm_hi[d]` and clear bit `d` in `self.jx.ymm_dirty`.
    #[inline(always)]
    pub(super) fn clear_ymm_hi_reg(&mut self, d: usize) {
        self.ymm_hi[d] = 0;
        self.jx.ymm_dirty &= !(1u16 << d);
    }

    /// Zero all 16 `self.ymm_hi` upper halves (`vzeroupper`/`vzeroall`) and clear `self.jx.ymm_dirty`.
    #[inline(always)]
    pub(super) fn clear_all_ymm_hi(&mut self) {
        self.ymm_hi = [0; 16];
        self.jx.ymm_dirty = 0;
    }

    pub fn new_zeroed() -> Self {
        Self {
            gpr: [0; 16],
            xmm: [0; 16],
            ymm_hi: [0; 16],
            rip: 0,
            cf: false,
            zf: false,
            sf: false,
            of: false,
            pfv: 1,
            df: false,
            last_bits: [64; 16],
            jx: jit::JitScratch { teb: unsafe { teb_self() } as u64, ..jit::JitScratch::default() },
        }
    }
}

pub(super) const RET_SENTINEL: u64 = 0xDEAD_BEEF_CAFE_0000;

/// ARM rounding, output flush, alternative handling and input flush controls.
#[cfg(all(test, target_arch = "aarch64"))]
pub(super) const FPCR_GUEST_MASK: u64 = (7u64 << 22) | 3;

/// Convert x86-64 `MXCSR` control bits (`FTZ`=15, `DAZ`=6, `RC`=14:13) to ARM64 `FPCR`
/// (`FZ`=24, `RMode`=23:22). Note `RMode` swaps `RM`/`RP` relative to `MXCSR.RC`:
/// `RC 00->00 (RN), 01->10 (RM), 10->01 (RP), 11->11 (RZ)`.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) fn mxcsr_to_fpcr(mxcsr: u32) -> u64 {
    let rc_lo = ((mxcsr >> 13) & 1) as u64;
    let rc_hi = ((mxcsr >> 14) & 1) as u64;
    let ftz = u64::from(mxcsr >> 15 & 1);
    let daz = u64::from(mxcsr >> 6 & 1);
    let flush = if ftz == daz {
        ftz << 24
    } else if super::features::host().has(super::features::HostFeatures::AFP) {
        // With AH=1 FZ controls outputs only. FIZ can flush inputs independently with AH=0.
        (ftz << 24) | (ftz << 1) | daz
    } else {
        0 // Mixed modes execute through the denormal-aware interpreter.
    };
    flush | (rc_lo << 23) | (rc_hi << 22)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(super) fn read_fpcr() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, fpcr", out(reg) v, options(nomem, nostack, preserves_flags));
    }
    v
}

#[cfg(not(target_arch = "aarch64"))]
#[allow(dead_code)]
#[inline(always)]
pub(super) fn read_fpcr() -> u64 {
    0
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(super) fn write_fpcr(v: u64) {
    unsafe {
        core::arch::asm!("msr fpcr, {}", in(reg) v, options(nomem, nostack, preserves_flags));
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[allow(dead_code)]
#[inline(always)]
pub(super) fn write_fpcr(_v: u64) {}

/// Apply a guest `MXCSR` value to ARM64 hardware `FPCR`, skipping the serializing `msr`
/// when `FPCR` already matches.
#[inline(always)]
pub(super) fn apply_guest_mxcsr_hw(mxcsr: u32) {
    #[cfg(target_arch = "aarch64")]
    {
        let cur = read_fpcr();
        let target = mxcsr_to_fpcr(mxcsr);
        if target != cur {
            write_fpcr(target);
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = mxcsr;
}

/// RAII guard for one guest execution region (`emu_run_region`): installs the guest thread's
/// `MXCSR` into hardware `FPCR` on entry and writes back `Cpu::jx.mxcsr` + restores the host's
/// saved `FPCR` on exit (including EH unwinds).
pub(super) struct GuestFpcrGuard {
    mxcsr_ptr: *const u32,
    entry_mxcsr: u32,
    #[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
    saved_host_fpcr: u64,
    previous_host: Option<u64>,
}

impl GuestFpcrGuard {
    #[inline(always)]
    pub(super) unsafe fn enter(mxcsr_ptr: *const u32) -> Self {
        let entry_mxcsr = *mxcsr_ptr;
        let saved_host_fpcr = read_fpcr();
        let previous_host = HOST_FPCR.with(|host| host.replace(Some(saved_host_fpcr)));
        apply_guest_mxcsr_hw(entry_mxcsr);
        Self { mxcsr_ptr, entry_mxcsr, saved_host_fpcr, previous_host }
    }
}

impl Drop for GuestFpcrGuard {
    #[inline(always)]
    fn drop(&mut self) {
        HOST_FPCR.with(|host| host.set(self.previous_host));
        let cur_m = unsafe { *self.mxcsr_ptr };
        if cur_m != self.entry_mxcsr {
            win32_rt::ucrt::runtime::set_guest_mxcsr(cur_m);
        }
        #[cfg(target_arch = "aarch64")]
        {
            if read_fpcr() != self.saved_host_fpcr {
                write_fpcr(self.saved_host_fpcr);
            }
        }
    }
}

thread_local! {
    static HOST_FPCR: core::cell::Cell<Option<u64>> = const { core::cell::Cell::new(None) };
}

pub(super) fn host_fpcr() -> u64 {
    HOST_FPCR.with(|host| host.get().unwrap_or_else(read_fpcr))
}

#[inline]
fn install_fpcr(value: u64) {
    if read_fpcr() != value { write_fpcr(value); }
}

/// RAII guard around full host calls (`host_call` / `host_tail_call`): syncs `Cpu::jx.mxcsr`
/// to `win32_rt`'s thread-local `MXCSR` (for `_controlfp`/`_control87` and nested guest
/// callbacks), installs the embedding thread's saved `FPCR` while host code runs, and pulls
/// any updated `MXCSR` back into `Cpu::jx.mxcsr` + hardware `FPCR` upon return.
pub(super) struct HostFpcrGuard {
    mxcsr_ptr: *mut u32,
}

impl HostFpcrGuard {
    #[inline(always)]
    pub(super) unsafe fn enter(mxcsr_ptr: *mut u32) -> Self {
        let m = *mxcsr_ptr;
        win32_rt::ucrt::runtime::set_guest_mxcsr(m);
        install_fpcr(host_fpcr());
        Self { mxcsr_ptr }
    }
}

impl Drop for HostFpcrGuard {
    #[inline(always)]
    fn drop(&mut self) {
        let m = win32_rt::ucrt::runtime::guest_mxcsr() & 0xFFFF;
        unsafe {
            *self.mxcsr_ptr = m;
        }
        apply_guest_mxcsr_hw(m);
    }
}

/// Installs the embedding FP environment for direct leaf calls, including nested callbacks.
pub(super) struct ClearFpcrGuard(u64);

impl ClearFpcrGuard {
    #[inline]
    pub(super) fn enter() -> Self {
        let saved = read_fpcr();
        let host = HOST_FPCR.with(|fpcr| fpcr.get().unwrap_or(saved));
        if host != saved { write_fpcr(host); }
        Self(saved)
    }
}

impl Drop for ClearFpcrGuard {
    #[inline]
    fn drop(&mut self) { install_fpcr(self.0); }
}

#[cfg(test)]
std::thread_local! {
    static STRICT_FAULTS: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Unit tests only: the interpreter/JIT fuzzers probe invalid addresses on purpose and compare
/// the lenient results, so tests keep the old behaviour (int3 returns, invalid accesses read 0)
/// unless the thread opted in with [`strict_faults_for_test`]. Always `false` outside tests.
#[inline(always)]
fn lenient_in_tests() -> bool {
    #[cfg(test)]
    {
        !STRICT_FAULTS.with(|c| c.get())
    }
    #[cfg(not(test))]
    {
        false
    }
}

/// Unit tests: use the production int3 / invalid-address behaviour on this thread.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn strict_faults_for_test(on: bool) {
    STRICT_FAULTS.with(|c| c.set(on));
}

/// `AW2_EMU_INT3=return` (A/B only): an `int 3` / `ud2` no vectored handler takes makes the
/// guest function return (the old behaviour). Default: unhandled exception (crash report,
/// the application's filter, exit), as on Windows.
pub(super) fn int3_returns() -> bool {
    if lenient_in_tests() {
        return true;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let r = std::env::var("AW2_EMU_INT3").is_ok_and(|v| v.trim() == "return");
        if r {
            log::warn!("emu: AW2_EMU_INT3=return: unhandled int3/ud2 return from the guest function (A/B only)");
        }
        r
    })
}

/// `AW2_EMU_BADADDR=ignore` (A/B only): accesses outside `[0x10000, 2^47)` read 0 / drop
/// the write forever (the old behaviour). Default: see [`cold_bad_addr`].
fn bad_addr_ignored() -> bool {
    if lenient_in_tests() {
        return true;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let r = std::env::var("AW2_EMU_BADADDR").is_ok_and(|v| v.trim() == "ignore");
        if r {
            log::warn!("emu: AW2_EMU_BADADDR=ignore: invalid guest accesses read 0 (A/B only)");
        }
        r
    })
}

/// Consecutive invalid accesses at one `rip` that raise an access violation (a pointer walk
/// such as `rbx = [rbx+0xd8]` that left the address space spins forever on reads of 0).
const BAD_ADDR_RUN_LIMIT: u32 = 100_000;

std::thread_local! {
    /// (`rip`, consecutive invalid accesses there).
    static BAD_ADDR_RUN: core::cell::Cell<(u64, u32)> = const { core::cell::Cell::new((0, 0)) };
    /// Precise guest faults: (address, write) of the current instruction's first invalid access.
    pub(super) static FAULT_PENDING: core::cell::Cell<Option<(u64, bool)>> = const { core::cell::Cell::new(None) };
}

/// Precise guest faults (target/review/exec/g18/seh_guard_plan.md): set only by the dll_lift
/// difftest child (`AW2_DLL_DIFF_PRECISE=1`) or `regfile::run_oracle`, default off.
/// Bit 0 = global [`set_precise_faults`] flag; bits 1..31 = active non-test oracle depth
/// ([`enter_precise_oracle`]/[`leave_precise_oracle`]) so production builds never pay a TLS
/// `_tlv_get_addr` call on every guest memory store.
static PRECISE_FAULTS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// Shared timestamp reader passed to generated Rust AOT dylibs at registration.
#[inline]
pub extern "C" fn guest_tsc(ordered: bool) -> u64 {
    super::tsc::read_tsc(ordered, super::tsc_is_frozen())
}

/// Read the guest timestamp for lifted RDTSC code.
#[inline]
pub fn guest_rdtsc() -> u64 {
    guest_tsc(false)
}

/// Read the guest timestamp after prior loads for lifted RDTSCP code.
#[inline]
pub fn guest_rdtscp() -> u64 {
    guest_tsc(true)
}

/// Turn precise guest faults on / off for runs that start afterwards (difftest child only).
pub fn set_precise_faults(on: bool) {
    if on {
        PRECISE_FAULTS.fetch_or(1, core::sync::atomic::Ordering::Relaxed);
    } else {
        PRECISE_FAULTS.fetch_and(!1, core::sync::atomic::Ordering::Relaxed);
    }
}

/// Increment active non-test oracle precise-fault depth (`regfile::run_oracle`).
#[cfg(not(test))]
#[inline]
pub(super) fn enter_precise_oracle() {
    PRECISE_FAULTS.fetch_add(2, core::sync::atomic::Ordering::Relaxed);
}

/// Decrement active non-test oracle precise-fault depth (`regfile::run_oracle`).
#[cfg(not(test))]
#[inline]
pub(super) fn leave_precise_oracle() {
    PRECISE_FAULTS.fetch_sub(2, core::sync::atomic::Ordering::Relaxed);
}

#[cfg(test)]
std::thread_local! {
    /// Lib tests: precise guest faults on this thread only (tests run in parallel).
    pub(super) static PRECISE_LOCAL: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Lib tests: [`set_precise_faults`] for this thread only.
#[cfg(test)]
pub(crate) fn set_precise_faults_local(on: bool) {
    PRECISE_LOCAL.with(|p| p.set(on));
}

/// Whether runs starting now use precise guest faults.
#[inline(always)]
pub fn precise_faults() -> bool {
    #[cfg(test)]
    {
        PRECISE_FAULTS.load(core::sync::atomic::Ordering::Relaxed) != 0 || PRECISE_LOCAL.with(|p| p.get())
    }
    #[cfg(not(test))]
    {
        PRECISE_FAULTS.load(core::sync::atomic::Ordering::Relaxed) != 0
    }
}

/// Access violations dispatched from guest instruction boundaries (precise mode), process-wide.
pub static PRECISE_FAULT_DISPATCHES: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

/// Precise mode: dispatch STATUS_ACCESS_VIOLATION (Information = [write, address]) from the
/// guest state in `cpu` (rip = the faulting instruction, registers as before it). The raise is
/// an [`eh::callout`] of `RaiseException` itself, so the dispatcher takes this region's guest
/// context as the raise context (ExceptionAddress / ControlPc = the instruction). A landing in
/// this region loads its state into `cpu`; a continued exception leaves `cpu` as it was (the
/// instruction runs again, as on Windows). Nested runs (filters) start below the guest rsp.
#[cold]
#[inline(never)]
pub(super) unsafe fn guest_fault_dispatch(cpu: &mut Cpu, sp_ptr: *mut u64, addr: u64, write: bool) {
    PRECISE_FAULT_DISPATCHES.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    crate::dll_native::lift::eh::note_seh_fault(addr);
    let saved_sp = *sp_ptr;
    *sp_ptr = (cpu.gpr[4].wrapping_sub(256)) & !0xFu64;
    let raise = win32_rt::unwind::RaiseException as *const () as usize;
    let info = [write as u64, addr];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let _ = eh::callout(cpu, raise, *sp_ptr, move || {
        win32_rt::unwind::raise_exception_raw(
            win32_rt::unwind::STATUS_ACCESS_VIOLATION as u64,
            0,
            2,
            info.as_ptr() as u64,
        );
        (0, 0)
    });
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    *sp_ptr = saved_sp;
    cpu.last_bits = [64; 16];
}

/// A guest access outside `[0x10000, 2^47)`. The access reads 0 / is dropped, unless:
/// - the address is in the null page (`< 0x10000`), or
/// - this is the `BAD_ADDR_RUN_LIMIT`th invalid access in a row at the same `rip`;
/// then it raises `STATUS_ACCESS_VIOLATION` (vectored handlers, then the unhandled path;
/// an access violation is not resumable mid-instruction, so a handler asking to continue
/// is logged and the process exits). The WARN is rate-limited (first 32, then powers of 2).
#[cold]
#[inline(never)]
fn cold_bad_addr(cpu: &Cpu, what: &str, addr: u64, bits: u32, write: bool) {
    use core::sync::atomic::{AtomicU64, Ordering};
    static SEEN: AtomicU64 = AtomicU64::new(0);
    let n = SEEN.fetch_add(1, Ordering::Relaxed) + 1;
    let rip = cpu.rip;
    if n <= 32 || n.is_power_of_two() {
        log::warn!("emu: {what} invalid addr={addr:#x} (bits={bits}) around rip={rip:#x} [{n} invalid accesses so far]");
    }
    if precise_faults() {
        // Raised at the end of the instruction (see `PRECISE_FAULTS`); the first access counts.
        FAULT_PENDING.with(|p| {
            if p.get().is_none() {
                p.set(Some((addr, write)));
            }
        });
        return;
    }
    if bad_addr_ignored() {
        return;
    }
    let run = BAD_ADDR_RUN.with(|r| {
        let (last, k) = r.get();
        let k = if last == rip { k.saturating_add(1) } else { 1 };
        r.set((rip, k));
        k
    });
    if addr >= 0x1_0000 && run < BAD_ADDR_RUN_LIMIT {
        return;
    }
    log::error!(
        "emu: raising an access violation: {what} addr={addr:#x} (bits={bits}) at rip={rip:#x} (next instruction){}",
        if addr < 0x1_0000 { " (null page)".to_string() } else { format!(" after {run} consecutive invalid accesses there") }
    );
    unsafe {
        let mut rec = win32_rt::unwind::guest_exception_record(
            win32_rt::unwind::STATUS_ACCESS_VIOLATION,
            rip,
            &[write as u64, addr],
        );
        let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, rip, cpu.rflags());
        if win32_rt::unwind::dispatch_veh_guest(&mut rec, &mut ctx) {
            log::error!("emu: a vectored handler asked to continue an emulated access violation; not resumable mid-instruction");
        }
        win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, false);
    }
}

/// `Register as u8` -> packed `(gpr index, width, high-byte)` (0 = not a GPR). See [`Cpu::reg_idx`].
static GPR_TABLE: [u8; 256] = build_gpr_table();

const fn build_gpr_table() -> [u8; 256] {
    const fn pack(idx: u8, w: u8, hi: bool) -> u8 {
        0x80 | idx | (w << 4) | if hi { 0x40 } else { 0 }
    }
    let mut t = [0u8; 256];
    let r64 = [
        Register::RAX, Register::RCX, Register::RDX, Register::RBX, Register::RSP, Register::RBP, Register::RSI, Register::RDI,
        Register::R8, Register::R9, Register::R10, Register::R11, Register::R12, Register::R13, Register::R14, Register::R15,
    ];
    let r32 = [
        Register::EAX, Register::ECX, Register::EDX, Register::EBX, Register::ESP, Register::EBP, Register::ESI, Register::EDI,
        Register::R8D, Register::R9D, Register::R10D, Register::R11D, Register::R12D, Register::R13D, Register::R14D, Register::R15D,
    ];
    let r16 = [
        Register::AX, Register::CX, Register::DX, Register::BX, Register::SP, Register::BP, Register::SI, Register::DI,
        Register::R8W, Register::R9W, Register::R10W, Register::R11W, Register::R12W, Register::R13W, Register::R14W, Register::R15W,
    ];
    let r8 = [
        Register::AL, Register::CL, Register::DL, Register::BL, Register::SPL, Register::BPL, Register::SIL, Register::DIL,
        Register::R8L, Register::R9L, Register::R10L, Register::R11L, Register::R12L, Register::R13L, Register::R14L, Register::R15L,
    ];
    let mut i = 0;
    while i < 16 {
        t[r64[i] as usize] = pack(i as u8, 3, false);
        t[r32[i] as usize] = pack(i as u8, 2, false);
        t[r16[i] as usize] = pack(i as u8, 1, false);
        t[r8[i] as usize] = pack(i as u8, 0, false);
        i += 1;
    }
    t[Register::AH as usize] = pack(0, 0, true);
    t[Register::CH as usize] = pack(1, 0, true);
    t[Register::DH as usize] = pack(2, 0, true);
    t[Register::BH as usize] = pack(3, 0, true);
    t
}

impl Cpu {
    /// x86 PF: even parity of the low result byte.
    #[inline(always)]
    pub(super) fn pf(&self) -> bool {
        self.pfv.count_ones() & 1 == 0
    }

    #[inline(always)]
    pub(super) fn set_pf(&mut self, pf: bool) {
        self.pfv = (!pf) as u8;
    }

    #[inline(always)]
    pub(super) fn reg_idx(r: Register) -> Option<(usize, u32, bool)> {
        let e = GPR_TABLE[r as usize];
        if e == 0 {
            return None;
        }
        // Packed: bits 0..4 = gpr index, bits 4..6 = width code (0:8,1:16,2:32,3:64), bit 6 = high-byte, bit 7 = valid.
        let idx = (e & 0x0F) as usize;
        let bits = 8u32 << ((e >> 4) & 0x3);
        Some((idx, bits, (e & 0x40) != 0))
    }

    #[inline(always)]
    pub(super) fn xmm_idx(r: Register) -> Option<usize> {
        let idx = match r {
            Register::XMM0 | Register::YMM0 => 0, Register::XMM1 | Register::YMM1 => 1,
            Register::XMM2 | Register::YMM2 => 2, Register::XMM3 | Register::YMM3 => 3,
            Register::XMM4 | Register::YMM4 => 4, Register::XMM5 | Register::YMM5 => 5,
            Register::XMM6 | Register::YMM6 => 6, Register::XMM7 | Register::YMM7 => 7,
            Register::XMM8 | Register::YMM8 => 8, Register::XMM9 | Register::YMM9 => 9,
            Register::XMM10 | Register::YMM10 => 10, Register::XMM11 | Register::YMM11 => 11,
            Register::XMM12 | Register::YMM12 => 12, Register::XMM13 | Register::YMM13 => 13,
            Register::XMM14 | Register::YMM14 => 14, Register::XMM15 | Register::YMM15 => 15,
            _ => return Option::None,
        };
        Some(idx)
    }

    #[inline(always)]
    pub(super) fn is_ymm(r: Register) -> bool {
        matches!(
            r,
            Register::YMM0
                | Register::YMM1
                | Register::YMM2
                | Register::YMM3
                | Register::YMM4
                | Register::YMM5
                | Register::YMM6
                | Register::YMM7
                | Register::YMM8
                | Register::YMM9
                | Register::YMM10
                | Register::YMM11
                | Register::YMM12
                | Register::YMM13
                | Register::YMM14
                | Register::YMM15
        )
    }

    #[inline(always)]
    pub(super) fn get_reg(&self, r: Register) -> u64 {
        let Some((idx, bits, hi8)) = Self::reg_idx(r) else { return 0 };
        let v = self.gpr[idx];
        match (bits, hi8) {
            (64, _) => v,
            (32, _) => v as u32 as u64,
            (16, _) => v as u16 as u64,
            (8, false) => v as u8 as u64,
            (8, true) => ((v >> 8) & 0xFF) as u64,
            _ => v,
        }
    }

    #[inline(always)]
    pub(super) fn set_reg(&mut self, r: Register, val: u64) {
        let Some((idx, bits, hi8)) = Self::reg_idx(r) else { return };
        self.last_bits[idx] = match (bits, hi8) {
            (8, false) => 8,
            (16, _) => 16,
            _ => 64,
        };
        match (bits, hi8) {
            (64, _) => self.gpr[idx] = val,
            (32, _) => self.gpr[idx] = val as u32 as u64,
            (16, _) => self.gpr[idx] = (self.gpr[idx] & !0xFFFF) | (val & 0xFFFF),
            (8, false) => self.gpr[idx] = (self.gpr[idx] & !0xFF) | (val & 0xFF),
            (8, true) => self.gpr[idx] = (self.gpr[idx] & !0xFF00) | ((val & 0xFF) << 8),
            _ => {}
        }
    }

    #[inline(always)]
    pub(super) fn abi_gpr(&self, idx: usize) -> u64 {
        match self.last_bits[idx] {
            8 => self.gpr[idx] & 0xFF,
            16 => self.gpr[idx] & 0xFFFF,
            _ => self.gpr[idx],
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn eff_addr(&self, ins: &Instruction) -> u64 {
        if ins.is_ip_rel_memory_operand() {
            return ins.ip_rel_memory_address();
        }
        let mut addr: u64 = ins.memory_displacement64();
        let base_reg = ins.memory_base();
        let mut is_a32 = false;
        if base_reg != Register::None {
            if base_reg == Register::RIP {
                addr = addr.wrapping_add(ins.next_ip());
            } else {
                if matches!(Self::reg_idx(base_reg), Some((_, 32, _))) {
                    is_a32 = true;
                }
                addr = addr.wrapping_add(self.get_reg(base_reg));
            }
        }
        let index = ins.memory_index();
        if index != Register::None {
            if matches!(Self::reg_idx(index), Some((_, 32, _))) {
                is_a32 = true;
            }
            addr = addr.wrapping_add(self.get_reg(index).wrapping_mul(ins.memory_index_scale() as u64));
        }
        if is_a32 {
            addr = (addr as u32) as u64;
        }
        if ins.segment_prefix() == Register::GS {
            let teb = teb_self() as u64;
            addr = teb.wrapping_add(addr);
        }
        addr
    }

    #[inline(always)]
    pub(super) unsafe fn read_mem(&self, addr: u64, bits: u32) -> u64 {
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "read_mem", addr, bits, false);
            return 0;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::load_acq(p, if bits > 64 { 64 } else { bits });
        }
        match bits {
            8 => rd::<u8>(p) as u64,
            16 => rd::<u16>(p) as u64,
            32 => rd::<u32>(p) as u64,
            64 => rd::<u64>(p),
            _ => rd::<u64>(p),
        }
    }

    #[inline(always)]
    pub(super) unsafe fn write_mem(&self, addr: u64, bits: u32, val: u64) {
        if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
            return;
        }
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "write_mem", addr, bits, true);
            return;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::store_rel(p, if bits > 64 { 64 } else { bits }, val);
        }
        match bits {
            8 => wr::<u8>(p, val as u8),
            16 => wr::<u16>(p, val as u16),
            32 => wr::<u32>(p, val as u32),
            64 => wr::<u64>(p, val),
            _ => wr::<u64>(p, val),
        }
    }

    #[inline]
    pub(super) unsafe fn read_u128(&self, addr: u64) -> u128 {
        if !regfile_ext::is_valid_guest_range(addr, 16) {
            cold_bad_addr(self, "read_u128", addr, 128, false);
            return 0;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::load128_acq(p);
        }
        rd::<u128>(p)
    }

    #[inline]
    pub(super) unsafe fn write_u128(&self, addr: u64, val: u128) {
        if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
            return;
        }
        if !regfile_ext::is_valid_guest_range(addr, 16) {
            cold_bad_addr(self, "write_u128", addr, 128, true);
            return;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::store128_rel(p, val);
        }
        wr::<u128>(p, val);
    }

    /// `true` for VEX-encoded instructions (a VEX.128 write zeroes bits 255:128 of the
    /// destination; a legacy SSE write preserves them).
    #[inline(always)]
    pub(super) fn is_vex(ins: &Instruction) -> bool {
        ins.encoding() == EncodingKind::VEX
    }

    /// Read vector source operand `op` (XMM/YMM register or memory) as `(bits 127:0, bits 255:128)`.
    /// Memory operands read exactly `memory_size()` bytes (so `m32`/`m64` sources never over-read);
    /// the upper half is only produced when `ymm` is set (register) / the operand is 256-bit (memory).
    #[inline]
    pub(super) unsafe fn vsrc(&self, ins: &Instruction, op: u32, ymm: bool) -> (u128, u128) {
        match ins.op_kind(op) {
            OpKind::Register => match Self::xmm_idx(ins.op_register(op)) {
                Some(i) => (self.xmm[i], if ymm { self.ymm_hi[i] } else { 0 }),
                None => (0, 0),
            },
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                let n = ins.memory_size().size();
                let lo = match n {
                    1 => self.read_mem(a, 8) as u128,
                    2 => self.read_mem(a, 16) as u128,
                    4 => self.read_mem(a, 32) as u128,
                    8 => self.read_mem(a, 64) as u128,
                    _ => self.read_u128(a),
                };
                let hi = if ymm && n >= 32 { self.read_u128(a.wrapping_add(16)) } else { 0 };
                (lo, hi)
            }
            _ => (0, 0),
        }
    }

    /// Write a full-width vector result to XMM/YMM register `d` (operand 0): YMM destinations
    /// get `hi`, VEX.128 destinations get their upper half zeroed, legacy SSE preserves it.
    #[inline]
    pub(super) fn vdst(&mut self, ins: &Instruction, d: usize, lo: u128, hi: u128) {
        self.xmm[d] = lo;
        if Self::is_ymm(ins.op_register(0)) {
            self.set_ymm_hi(d, hi);
        } else if Self::is_vex(ins) {
            self.clear_ymm_hi_reg(d);
        }
    }

    /// VEX.128 forms zero bits 255:128 of the destination register.
    #[inline(always)]
    pub(super) fn vex128_zero_upper(&mut self, ins: &Instruction, d: usize) {
        if Self::is_vex(ins) && !Self::is_ymm(ins.op_register(0)) {
            self.clear_ymm_hi_reg(d);
        }
    }

    /// x86 `lock`-prefixed read-modify-write: atomic and a full memory barrier. An aarch64
    /// `casal` alone is not a full barrier (a later load may be satisfied before its store is
    /// visible), which breaks StoreLoad-dependent guest code such as work-stealing deque pops
    /// (`lock xadd [bottom]` then load `top`), so follow it with a `dmb ish`.
    #[inline]
    pub(super) unsafe fn atomic_rmw(&self, addr: u64, bits: u32, f: impl FnMut(u64) -> u64) -> (u64, u64) {
        let r = self.atomic_rmw_raw(addr, bits, f);
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        r
    }

    /// `lock cmpxchg`: atomic compare-exchange plus a full barrier (see [`Self::atomic_rmw`]).
    #[inline]
    pub(super) unsafe fn atomic_cmpxchg(&self, addr: u64, bits: u32, expected: u64, desired: u64) -> (bool, u64) {
        let r = self.atomic_cmpxchg_raw(addr, bits, expected, desired);
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        r
    }

    #[inline]
    unsafe fn atomic_rmw_raw(&self, addr: u64, bits: u32, mut f: impl FnMut(u64) -> u64) -> (u64, u64) {
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "atomic_rmw", addr, bits, true);
            return (0, 0);
        }
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let p = addr as usize;
        #[cfg(target_arch = "aarch64")]
        {
            if p & (bytes - 1) == 0 || ((p & 15) + bytes <= 16 && super::features::host().has(super::features::HostFeatures::LSE2)) {
                let mut cur = read_unaligned_bits(p, bits);
                loop {
                    let next = f(cur) & mask;
                    let actual = lse_casal(p, bits, cur, next);
                    if actual == cur {
                        return (cur, next);
                    }
                    cur = actual;
                }
            }
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};
            if (p & (bytes - 1)) == 0 {
                match bits {
                    8 => {
                        let a = &*(p as *const AtomicU8);
                        let mut cur = a.load(Ordering::Relaxed);
                        loop {
                            let next = f(cur as u64) as u8;
                            match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                                Ok(_) => return (cur as u64, next as u64),
                                Err(actual) => cur = actual,
                            }
                        }
                    }
                    16 => {
                        let a = &*(p as *const AtomicU16);
                        let mut cur = a.load(Ordering::Relaxed);
                        loop {
                            let next = f(cur as u64) as u16;
                            match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                                Ok(_) => return (cur as u64, next as u64),
                                Err(actual) => cur = actual,
                            }
                        }
                    }
                    32 => {
                        let a = &*(p as *const AtomicU32);
                        let mut cur = a.load(Ordering::Relaxed);
                        loop {
                            let next = f(cur as u64) as u32;
                            match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                                Ok(_) => return (cur as u64, next as u64),
                                Err(actual) => cur = actual,
                            }
                        }
                    }
                    _ => {
                        let a = &*(p as *const AtomicU64);
                        let mut cur = a.load(Ordering::Relaxed);
                        loop {
                            let next = f(cur);
                            match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                                Ok(_) => return (cur, next),
                                Err(actual) => cur = actual,
                            }
                        }
                    }
                }
            }
        }
        let _g = lock_atomic_shard(addr);
        let old = self.read_mem(addr, bits) & mask;
        let new = f(old) & mask;
        self.write_mem(addr, bits, new);
        (old, new)
    }

    #[inline]
    unsafe fn atomic_cmpxchg_raw(&self, addr: u64, bits: u32, expected: u64, desired: u64) -> (bool, u64) {
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "atomic_cmpxchg", addr, bits, true);
            return (false, 0);
        }
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let exp = expected & mask;
        let des = desired & mask;
        let p = addr as usize;
        #[cfg(target_arch = "aarch64")]
        {
            if p & (bytes - 1) == 0 || ((p & 15) + bytes <= 16 && super::features::host().has(super::features::HostFeatures::LSE2)) {
                let actual = lse_casal(p, bits, exp, des);
                return (actual == exp, actual);
            }
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};
            if (p & (bytes - 1)) == 0 {
                return match bits {
                    8 => {
                        let a = &*(p as *const AtomicU8);
                        match a.compare_exchange(exp as u8, des as u8, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(v) => (true, v as u64),
                            Err(v) => (false, v as u64),
                        }
                    }
                    16 => {
                        let a = &*(p as *const AtomicU16);
                        match a.compare_exchange(exp as u16, des as u16, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(v) => (true, v as u64),
                            Err(v) => (false, v as u64),
                        }
                    }
                    32 => {
                        let a = &*(p as *const AtomicU32);
                        match a.compare_exchange(exp as u32, des as u32, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(v) => (true, v as u64),
                            Err(v) => (false, v as u64),
                        }
                    }
                    _ => {
                        let a = &*(p as *const AtomicU64);
                        match a.compare_exchange(exp, des, Ordering::SeqCst, Ordering::SeqCst) {
                            Ok(v) => (true, v),
                            Err(v) => (false, v),
                        }
                    }
                };
            }
        }
        let _g = lock_atomic_shard(addr);
        let cur = self.read_mem(addr, bits) & mask;
        if cur == exp {
            self.write_mem(addr, bits, des);
            (true, cur)
        } else {
            (false, cur)
        }
    }

    #[inline(always)]
    pub(super) fn op_bits(ins: &Instruction, op: u32) -> u32 {
        match ins.op_kind(op) {
            OpKind::Register => {
                let r = ins.op_register(op);
                if let Some((_, bits, _)) = Self::reg_idx(r) {
                    bits
                } else {
                    128
                }
            }
            OpKind::Memory => (ins.memory_size().size() as u32 * 8).max(8),
            OpKind::Immediate8 => 8,
            OpKind::Immediate16 => 16,
            OpKind::Immediate32 => 32,
            OpKind::Immediate64 | OpKind::Immediate8to64 | OpKind::Immediate32to64 => 64,
            OpKind::Immediate8to16 => 16,
            OpKind::Immediate8to32 => 32,
            _ => 64,
        }
    }

    #[inline(always)]
    pub(super) unsafe fn read_op(&self, ins: &Instruction, op: u32, default_bits: u32) -> u64 {
        let raw = match ins.op_kind(op) {
            OpKind::Register => return self.get_reg(ins.op_register(op)),
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                let b = (ins.memory_size().size() as u32 * 8).max(default_bits);
                return self.read_mem(a, b);
            }
            OpKind::Immediate8 => ins.immediate8() as u64,
            OpKind::Immediate16 => ins.immediate16() as u64,
            OpKind::Immediate32 => ins.immediate32() as u64,
            OpKind::Immediate64 => ins.immediate64(),
            OpKind::Immediate8to16 => ins.immediate8to16() as u16 as u64,
            OpKind::Immediate8to32 => ins.immediate8to32() as u32 as u64,
            OpKind::Immediate8to64 => ins.immediate8to64() as u64,
            OpKind::Immediate32to64 => ins.immediate32to64() as u64,
            _ => 0,
        };
        if default_bits < 64 {
            raw & ((1u64 << default_bits) - 1)
        } else {
            raw
        }
    }

    #[inline(always)]
    pub(super) unsafe fn write_op(&mut self, ins: &Instruction, op: u32, bits: u32, val: u64) {
        match ins.op_kind(op) {
            OpKind::Register => self.set_reg(ins.op_register(op), val),
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                self.write_mem(a, bits, val);
            }
            _ => {}
        }
    }

    #[inline(always)]
    pub(super) fn update_flags_logic(&mut self, res: u64, bits: u32) {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let r = res & mask;
        let sign_bit = 1u64 << (bits - 1);
        self.cf = false;
        self.of = false;
        self.zf = r == 0;
        self.sf = (r & sign_bit) != 0;
        self.pfv = r as u8;
    }

    #[inline(always)]
    pub(super) fn update_flags_add(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let ua = a & mask;
        let ub = b & mask;
        let sign_bit = 1u64 << (bits - 1);
        let res = ua.wrapping_add(ub) & mask;
        self.cf = res < ua;
        self.zf = res == 0;
        self.sf = (res & sign_bit) != 0;
        self.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
        self.pfv = res as u8;
        res
    }

    #[inline(always)]
    pub(super) fn update_flags_sub(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let ua = a & mask;
        let ub = b & mask;
        let sign_bit = 1u64 << (bits - 1);
        let res = ua.wrapping_sub(ub) & mask;
        self.cf = ua < ub;
        self.zf = res == 0;
        self.sf = (res & sign_bit) != 0;
        self.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
        self.pfv = res as u8;
        res
    }

    #[inline(always)]
    pub(super) fn eval_cc(&self, cc: ConditionCode) -> bool {
        match cc {
            ConditionCode::o => self.of,
            ConditionCode::no => !self.of,
            ConditionCode::b => self.cf,
            ConditionCode::ae => !self.cf,
            ConditionCode::e => self.zf,
            ConditionCode::ne => !self.zf,
            ConditionCode::be => self.cf || self.zf,
            ConditionCode::a => !self.cf && !self.zf,
            ConditionCode::s => self.sf,
            ConditionCode::ns => !self.sf,
            ConditionCode::p => self.pf(),
            ConditionCode::np => !self.pf(),
            ConditionCode::l => self.sf != self.of,
            ConditionCode::ge => self.sf == self.of,
            ConditionCode::le => self.zf || (self.sf != self.of),
            ConditionCode::g => !self.zf && (self.sf == self.of),
            ConditionCode::None => false,
        }
    }

    #[inline(always)]
    pub fn rflags(&self) -> u32 {
        0x202
            | (self.cf as u32)
            | ((self.pf() as u32) << 2)
            | ((self.zf as u32) << 6)
            | ((self.sf as u32) << 7)
            | ((self.df as u32) << 10)
            | ((self.of as u32) << 11)
    }

    #[inline(always)]
    pub fn set_rflags(&mut self, f: u32) {
        self.cf = (f & 1) != 0;
        self.set_pf(((f >> 2) & 1) != 0);
        self.zf = ((f >> 6) & 1) != 0;
        self.sf = ((f >> 7) & 1) != 0;
        self.df = ((f >> 10) & 1) != 0;
        self.of = ((f >> 11) & 1) != 0;
    }
}

// ---------------------------------------------------------------------------------------------
// x86 SSE numeric semantics (MXCSR default: all exceptions masked, round-to-nearest-even).
// ---------------------------------------------------------------------------------------------

/// x86 NaN result rules for SSE arithmetic: a NaN source is propagated quieted (the first source
/// wins when both are NaN); an invalid operation on non-NaN inputs yields the negative "real
/// indefinite" QNaN (`0xFFC00000`), whereas aarch64 would produce the positive default NaN.
#[inline(always)]
pub(super) fn x86_nan32(a: f32, b: f32, r: f32) -> f32 {
    if !r.is_nan() {
        r
    } else if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        f32::from_bits(0xFFC0_0000)
    }
}

/// See [`x86_nan32`].
#[inline(always)]
pub(super) fn x86_nan64(a: f64, b: f64, r: f64) -> f64 {
    if !r.is_nan() {
        r
    } else if a.is_nan() {
        f64::from_bits(a.to_bits() | 0x0008_0000_0000_0000)
    } else if b.is_nan() {
        f64::from_bits(b.to_bits() | 0x0008_0000_0000_0000)
    } else {
        f64::from_bits(0xFFF8_0000_0000_0000)
    }
}

/// `addss/subss/mulss/divss/minss/maxss` (and the packed lanes) with x86 semantics.
/// `op`: 0 add, 1 sub, 2 mul, 3 div, 4 min, 5 max. MIN/MAX return the second operand when
/// either input is NaN or both are zero (of either sign).
#[inline(always)]
pub(super) fn sse_arith32(op: u8, a: f32, b: f32) -> f32 {
    match op {
        0 => x86_nan32(a, b, a + b),
        1 => x86_nan32(a, b, a - b),
        2 => x86_nan32(a, b, a * b),
        3 => x86_nan32(a, b, a / b),
        _ => {
            let selected = if (op == 4 && a < b) || (op == 5 && a > b) { a } else { b };
            f32::from_bits(super::denorm::minmax_result(selected.to_bits().into(), false) as u32)
        }
    }
}

/// See [`sse_arith32`].
#[inline(always)]
pub(super) fn sse_arith64(op: u8, a: f64, b: f64) -> f64 {
    match op {
        0 => x86_nan64(a, b, a + b),
        1 => x86_nan64(a, b, a - b),
        2 => x86_nan64(a, b, a * b),
        3 => x86_nan64(a, b, a / b),
        _ => {
            let selected = if (op == 4 && a < b) || (op == 5 && a > b) { a } else { b };
            f64::from_bits(super::denorm::minmax_result(selected.to_bits().into(), true) as u64)
        }
    }
}

#[inline(always)]
pub(super) fn sse_sqrt32(a: f32) -> f32 {
    x86_nan32(a, a, a.sqrt())
}

#[inline(always)]
pub(super) fn sse_sqrt64(a: f64) -> f64 {
    x86_nan64(a, a, a.sqrt())
}

/// `cvt(t)ss2si` / `cvt(t)sd2si` to a 32-bit integer: NaN and out-of-range inputs produce the
/// "integer indefinite" value `0x80000000`. Non-truncating forms round to nearest-even (MXCSR).
#[inline(always)]
pub(super) fn sse_cvt_i32(f: f64, trunc: bool) -> u32 {
    let r = if trunc { f.trunc() } else { f.round_ties_even() };
    if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
        0x8000_0000
    } else {
        r as i32 as u32
    }
}

/// 64-bit counterpart of [`sse_cvt_i32`] (indefinite = `0x8000000000000000`).
#[inline(always)]
pub(super) fn sse_cvt_i64(f: f64, trunc: bool) -> u64 {
    let r = if trunc { f.trunc() } else { f.round_ties_even() };
    if r.is_nan() || r >= 9223372036854775808.0 || r < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        r as i64 as u64
    }
}

/// `cmpps`/`cmpss`/`vcmpXX` predicate evaluation (imm8[3:0]; imm8[4] only selects signalling
/// vs quiet behaviour, which does not affect the result with exceptions masked).
#[inline(always)]
pub(super) fn sse_cmp_pred(fa: f64, fb: f64, pred: u8) -> bool {
    let unord = fa.is_nan() || fb.is_nan();
    match pred & 0x0F {
        0x00 => !unord && fa == fb, // EQ_OQ
        0x01 => !unord && fa < fb,  // LT_OS
        0x02 => !unord && fa <= fb, // LE_OS
        0x03 => unord,              // UNORD_Q
        0x04 => unord || fa != fb,  // NEQ_UQ
        0x05 => unord || fa >= fb,  // NLT_US
        0x06 => unord || fa > fb,   // NLE_US
        0x07 => !unord,             // ORD_Q
        0x08 => unord || fa == fb,  // EQ_UQ
        0x09 => unord || fa < fb,   // NGE_US
        0x0A => unord || fa <= fb,  // NGT_US
        0x0B => false,              // FALSE_OQ
        0x0C => !unord && fa != fb, // NEQ_OQ
        0x0D => !unord && fa >= fb, // GE_OS
        0x0E => !unord && fa > fb,  // GT_OS
        _ => true,                  // TRUE_UQ
    }
}

/// `lock cmpxchg16b` on a 16-byte-aligned guest address: a real 128-bit CAS (LSE `caspal`)
/// so it is atomic against concurrent 64-bit locked ops / stores to either half, followed by
/// x86's full barrier. Returns the previous value. `None` on hosts without it (callers fall
/// back to a mutex).
#[cfg(target_arch = "aarch64")]
#[inline]
pub(super) unsafe fn cas128_full(addr: u64, expected: u128, desired: u128) -> Option<u128> {
    let (mut lo, mut hi) = (expected as u64, (expected >> 64) as u64);
    core::arch::asm!(
        "caspal x0, x1, x2, x3, [{p}]",
        p = in(reg) addr,
        inout("x0") lo,
        inout("x1") hi,
        in("x2") desired as u64,
        in("x3") (desired >> 64) as u64,
        options(nostack),
    );
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    Some(((hi as u128) << 64) | lo as u128)
}

#[cfg(not(target_arch = "aarch64"))]
#[inline]
pub(super) unsafe fn cas128_full(_addr: u64, _expected: u128, _desired: u128) -> Option<u128> {
    None
}

/// Number of cache-line-isolated spinlock shards for 16-byte-boundary-crossing LOCK operations.
/// Every 2/4/8/16-byte access crossing a 16-byte boundary starts in granule `addr >> 4`, so
/// hashing by `(addr >> 4) & (ATOMIC_LOCK_SHARDS - 1)` serializes all overlapping boundary-crossing
/// accesses across every execution tier (Generic, FastOp, Live JIT, Disk AOT, Static Mach-O AOT).
pub(super) const ATOMIC_LOCK_SHARDS: usize = 64;
#[cfg(target_arch = "aarch64")]
pub(super) const ATOMIC_LOCK_SHARD_BITS: u32 = 6;
#[cfg(target_arch = "aarch64")]
pub(super) const ATOMIC_LOCK_STRIDE_SHIFT: u32 = 7; // 128 bytes (Apple Silicon cache line)

#[repr(C, align(128))]
struct AtomicLockShard {
    lock: core::sync::atomic::AtomicU32,
}

impl AtomicLockShard {
    const fn new() -> Self {
        Self { lock: core::sync::atomic::AtomicU32::new(0) }
    }
}

#[cfg(target_arch = "aarch64")]
const _: () = assert!(ATOMIC_LOCK_SHARDS == (1usize << ATOMIC_LOCK_SHARD_BITS));
#[cfg(target_arch = "aarch64")]
const _: () = assert!(core::mem::size_of::<AtomicLockShard>() == (1usize << ATOMIC_LOCK_STRIDE_SHIFT));
const _: () = assert!(core::mem::offset_of!(AtomicLockShard, lock) == 0);

static ATOMIC_LOCKS: [AtomicLockShard; ATOMIC_LOCK_SHARDS] =
    [const { AtomicLockShard::new() }; ATOMIC_LOCK_SHARDS];

/// Base address of [`ATOMIC_LOCKS`] exposed to translated code via `Cpu::jx.atomic_locks`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(super) fn atomic_locks_ptr() -> usize {
    ATOMIC_LOCKS.as_ptr() as usize
}

/// RAII guard for one shard in [`ATOMIC_LOCKS`].
pub(super) struct AtomicShardGuard {
    lock: &'static core::sync::atomic::AtomicU32,
}

impl Drop for AtomicShardGuard {
    #[inline(always)]
    fn drop(&mut self) {
        self.lock.store(0, core::sync::atomic::Ordering::Release);
    }
}

/// Acquire the spinlock shard governing 16-byte-crossing LOCK ops starting in granule `addr >> 4`.
/// Uses the exact same `0 = unlocked, 1 = locked` protocol as translated ARM64 code.
#[inline]
pub(super) fn lock_atomic_shard(addr: u64) -> AtomicShardGuard {
    use core::sync::atomic::Ordering;
    let idx = ((addr >> 4) as usize) & (ATOMIC_LOCK_SHARDS - 1);
    let lock = &ATOMIC_LOCKS[idx].lock;
    while lock.swap(1, Ordering::Acquire) != 0 {
        while lock.load(Ordering::Relaxed) != 0 {
            core::hint::spin_loop();
        }
    }
    AtomicShardGuard { lock }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn read_unaligned_bits(addr: usize, bits: u32) -> u64 {
    match bits {
        8 => core::ptr::read_unaligned(addr as *const u8) as u64,
        16 => core::ptr::read_unaligned(addr as *const u16) as u64,
        32 => core::ptr::read_unaligned(addr as *const u32) as u64,
        _ => core::ptr::read_unaligned(addr as *const u64),
    }
}

/// Single-instruction LSE compare-and-swap (`CASAL{B,H,,}`), which `FEAT_LSE2` permits on any
/// misaligned address that does not cross a 16-byte boundary. Returns the zero-extended old value.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn lse_casal(addr: usize, bits: u32, expected: u64, desired: u64) -> u64 {
    match bits {
        8 => {
            let mut cur = expected as u32;
            core::arch::asm!(
                "casalb {c:w}, {d:w}, [{p}]",
                p = in(reg) addr,
                c = inout(reg) cur,
                d = in(reg) desired as u32,
                options(nostack),
            );
            (cur & 0xFF) as u64
        }
        16 => {
            let mut cur = expected as u32;
            core::arch::asm!(
                "casalh {c:w}, {d:w}, [{p}]",
                p = in(reg) addr,
                c = inout(reg) cur,
                d = in(reg) desired as u32,
                options(nostack),
            );
            (cur & 0xFFFF) as u64
        }
        32 => {
            let mut cur = expected as u32;
            core::arch::asm!(
                "casal {c:w}, {d:w}, [{p}]",
                p = in(reg) addr,
                c = inout(reg) cur,
                d = in(reg) desired as u32,
                options(nostack),
            );
            cur as u64
        }
        _ => {
            let mut cur = expected;
            core::arch::asm!(
                "casal {c:x}, {d:x}, [{p}]",
                p = in(reg) addr,
                c = inout(reg) cur,
                d = in(reg) desired,
                options(nostack),
            );
            cur
        }
    }
}
