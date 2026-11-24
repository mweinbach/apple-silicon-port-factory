//! x86-64 -> arm64 block translator for [`super::emu_run_full`] (aarch64 hosts only).
//!
//! Hot guest blocks (the per-thread [`super::BlockCache`] blocks, already lowered to
//! [`super::fast::FastOp`]s) are translated into arm64 code in a per-thread `MAP_JIT` buffer.
//! Translation covers the fast-op prefix of a block; the first generic instruction ends the
//! translated code and the interpreter takes over there.
//!
//! # Execution model
//! - The interpreter enters translated code through the shared `enter` stub, which saves
//!   the host callee-saved registers and loads all 16 guest GPRs into pinned host registers
//!   ([`HG`]). Translated blocks jump to each other directly (patched links, or the
//!   `lookup_*` stubs for indirect targets) without leaving translated code; the shared
//!   `exit` stub writes the GPRs back and returns an exit code to the interpreter.
//! - Flags: within a block, x86 flags live in the host NZCV flags where possible
//!   (`translate::Src`); PF is stored eagerly as the parity byte [`Cpu::pfv`]. Flags are only
//!   computed when live (backward liveness over the block plus a scan of the statically
//!   known successor blocks), and are written to the `Cpu` flag bytes before leaving a block.
//! - `x8` counts the remaining instruction budget; every block entry subtracts its length
//!   and exits when it goes negative, so the interpreter keeps its step counts/profiling.
//! - Every guest memory access is range-checked like `Cpu::read_mem`/`write_mem`; failures
//!   side-exit to the interpreter at that instruction, which then performs it with the
//!   generic semantics. With TSO emulation on, loads/stores use `ldapr`/`stlr` (unaligned
//!   accesses take an out-of-line plain access + `dmb`, exactly like `emu_mem`).
//! - `Cpu::last_bits` is maintained for the registers [`Cpu::abi_gpr`] reads (rax, rcx, rdx,
//!   r8, r9); translated code writes 0 for full-width writes.
//!
//! Blocks from run-time generated guest code (V8) record a hash of their bytes and are
//! revalidated after every executable-memory change, like the block cache; they are never
//! the target of a patched direct link. Everything is flushed when the host dispatch table
//! changes ([`crate::exe::va_gen`]) or the code buffer fills up.

#![allow(clippy::too_many_arguments)]

mod aot;
mod cache;
mod emit;
mod ext;
mod lazy;
mod more;
mod ops;
mod stubs;
mod translate;
mod vec;
mod xmm;
#[cfg(test)]
mod tests;

pub(super) use aot::{
    aot_emit_stubs, aot_knob_bits, aot_translate, emit_static_stubs_with_veneer, AotBlock,
    AotReloc, AotStubOffs, AOT_MAX_BLOCK_BYTES, AOT_STUBS_BYTES, RELOC_HOST_FN, RELOC_QPC_STATE,
    RELOC_SLOT_VAL,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(super) use aot::{emit_static_stubs, test_emit_stubs_and_veneer_for_knobs};
pub(super) use cache::{
    inline_host_fn, inline_host_fn_kind, patch_lse2_unaligned, prep_block, register_lse2_region,
    unregister_lse2_region, JitCache, TrScratch, JMUL,
};
#[cfg(test)]
pub(super) use cache::{test_aot_installs, test_probe_hits};
#[cfg(test)]
#[allow(unused_imports)]
pub(super) use lazy::{flaglazy_enabled, Edges};
#[cfg(test)]
#[allow(unused_imports)]
pub(super) use stubs::plaincall_enabled;
pub(super) use translate::Term;
#[cfg(test)]
#[allow(unused_imports)]
pub(super) use translate::supported;
#[cfg(test)]
#[allow(unused_imports)]
pub(super) use xmm::{fast_nan_enabled, retlit_enabled, xmm_pin_enabled, xmmreg_enabled};

use crate::emu::Cpu;
use core::mem::offset_of;

/// Fields of [`super::Cpu`] used by translated code.
#[derive(Clone, Copy)]
#[repr(C)]
pub(super) struct JitScratch {
    /// Remaining instruction budget (translated blocks exit before it goes negative).
    pub budget: i64,
    /// Target of the control transfer that ended the translated run (see `EXIT_*`).
    pub target: u64,
    /// Translation index base (for the `lookup_*` stubs).
    pub table: usize,
    /// Index hash multiplier.
    pub mul: u64,
    /// Address of the executable-memory generation counter ([`crate::emu::mem::code_gen`]).
    pub codegen: usize,
    /// Cached TEB self pointer (`gs:[0x30]`) for `%gs:` segment-prefixed memory accesses.
    pub teb: u64,
    /// `emu_run_full` context for [`super::jit_host_xfer`] (host calls made from translated
    /// code): guest stack pointer cell, block cache, profiler, entry IP.
    pub ctx_sp: usize,
    pub ctx_blocks: usize,
    pub ctx_prof: usize,
    pub ctx_entry: u64,
    /// [`crate::exe::va_gen`] the running translations were made for.
    pub ctx_vagen: u64,
    /// Shadow return-address stack (`AW2_JIT_RAS`): address of the top entry in the
    /// 64 KiB-aligned ring (`JitCache::ras_top` while translated code runs).
    pub ras: u64,
    /// `&JitCache::ras_top` of the running translation cache, so [`super::jit_host_xfer`]
    /// can hand the top to nested runs on this thread (0 when the RAS is off).
    pub ctx_ras_cell: usize,
    /// Shared AOT probe table ([`crate::emu::aot::probe_table`], `AW2_DLL_AOT_PROBE`) searched by
    /// the `lookup_*` stubs after a miss in [`Self::table`]; 0 = none.
    pub aot_dir: usize,
    /// `lookup_*` misses served from [`Self::aot_dir`] (drained by [`JitCache::run`]).
    pub aot_probe_hits: u64,
    /// Non-zero when translated `rdtsc`/`rdtscp` must return deterministic [`crate::emu::GUEST_TSC`]
    /// (populated from [`crate::emu::tsc_is_frozen`] at translated-code entry).
    pub tsc_frozen: u8,
    /// Non-zero when `fast::Cpu::rset` should maintain `Cpu::last_bits` (mirrors `PORT_JIT_LAST_BITS` / verify).
    pub lb_on: u8,
    /// Bitmask of `Cpu::ymm_hi[0..16]` registers that may be non-zero (0 = all upper halves
    /// zero, allowing VEX-128 ops to skip per-op `Cpu::ymm_hi` zeroing stores).
    pub ymm_dirty: u16,
    /// Guest x86 `MXCSR` state (`0x1F80` default; bit 15 = FTZ, bit 6 = DAZ, bits 14:13 = RC).
    pub mxcsr: u32,
    /// Test-only: dynamic `DF_PLAIN` / pure-float-math / inline-QPC calls served directly in
    /// `lookup_call` / `lookup_jmp` from `table`.
    #[cfg(test)]
    pub plain_call_hits: u64,
}

impl Default for JitScratch {
    fn default() -> Self {
        Self {
            budget: 0,
            target: 0,
            table: 0,
            mul: 0,
            codegen: 0,
            teb: 0,
            ctx_sp: 0,
            ctx_blocks: 0,
            ctx_prof: 0,
            ctx_entry: 0,
            ctx_vagen: 0,
            ras: 0,
            ctx_ras_cell: 0,
            aot_dir: 0,
            aot_probe_hits: 0,
            tsc_frozen: 0,
            lb_on: 1,
            ymm_dirty: 0,
            mxcsr: 0x1F80,
            #[cfg(test)]
            plain_call_hits: 0,
        }
    }
}

/// Continue interpreting at `cpu.rip`.
pub(super) const EXIT_RIP: u64 = 0;
/// `call jx.target` with `cpu.rip` = return address (interpreter applies host dispatch).
pub(super) const EXIT_CALL: u64 = 1;
/// `jmp jx.target` (interpreter applies host dispatch / `__chkstk`).
pub(super) const EXIT_JMP: u64 = 2;
/// Taken near `jcc jx.target` (interpreter applies host dispatch).
pub(super) const EXIT_JCC: u64 = 3;
/// Side exit: the interpreter must execute the instruction at `cpu.rip` itself before
/// re-entering translated code (a block starting there would exit again immediately).
pub(super) const EXIT_SIDE: u64 = 4;

/// Registered EXE `__chkstk` VA or byte-detected DLL `__chkstk` VA.
#[inline(always)]
fn is_chkstk(ip: u64) -> bool {
    crate::hooks::is_exe_chkstk_va(ip) || crate::emu::is_routed_dll_chkstk(ip)
}

std::thread_local! {
    /// Test override: 0 = environment, 1 = force on (translate on first visit), 2 = force off.
    static FORCE: core::cell::Cell<u8> = const { core::cell::Cell::new(0) };
    /// Test-only: guest instructions executed in translated code on this thread.
    #[cfg(test)]
    static TEST_EXECUTED: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test-only: translated-code runs on this thread that left the RAS top moved.
    #[cfg(test)]
    static TEST_RAS_MOVES: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test-only: dynamic `DF_PLAIN` / float-math / QPC transfers served directly in
    /// `lookup_call` / `lookup_jmp` stubs without entering `host_xfer`.
    #[cfg(test)]
    static TEST_STUB_PLAIN_HITS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Runs of translated code on this thread that moved the shadow RAS top (tests).
#[cfg(test)]
pub(super) fn test_ras_moves() -> u64 {
    TEST_RAS_MOVES.with(|c| c.get())
}

/// Dynamic plain / float-math / QPC host transfers served directly in `lookup_call`/`lookup_jmp`
/// on this thread (tests).
#[cfg(test)]
pub(super) fn test_stub_plain_hits() -> u64 {
    TEST_STUB_PLAIN_HITS.with(|c| c.get())
}

/// Guest instructions executed in translated code on this thread (tests).
#[cfg(test)]
pub(super) fn test_executed() -> u64 {
    TEST_EXECUTED.with(|c| c.get())
}

/// Force the translator on/off for this thread (tests; shadow probe children).
pub(super) fn force(mode: u8) {
    FORCE.with(|f| f.set(mode));
}

/// One translated block captured by [`test_dump_start`] (tests).
#[cfg(test)]
pub(super) struct DumpBlock {
    pub ip: u64,
    /// Guest instructions translated.
    pub guest: usize,
    /// Host code address.
    pub code: usize,
    /// All emitted words (main path first, then out-of-line code).
    pub words: Vec<u32>,
    /// Words on the main path (entry through the terminator).
    pub main: usize,
}

#[cfg(test)]
std::thread_local! {
    static TEST_DUMP: core::cell::RefCell<Option<Vec<DumpBlock>>> = const { core::cell::RefCell::new(None) };
}

/// Start capturing translated blocks on this thread (tests).
#[cfg(test)]
pub(super) fn test_dump_start() {
    TEST_DUMP.with(|d| *d.borrow_mut() = Some(Vec::new()));
}

/// Stop capturing and return the blocks translated since [`test_dump_start`] (tests).
#[cfg(test)]
pub(super) fn test_dump_take() -> Vec<DumpBlock> {
    TEST_DUMP.with(|d| d.borrow_mut().take().unwrap_or_default())
}

/// Coarse class of an emitted arm64 instruction word (tests: `jit_bench` breakdowns).
#[cfg(test)]
pub(super) fn test_classify(w: u32) -> &'static str {
    let rn = (w >> 5) & 31;
    let rd = w & 31;
    let cpu_class = |off: u32| -> &'static str {
        if [OFF_CF, OFF_ZF, OFF_SF, OFF_OF, OFF_PFV, OFF_DF].contains(&off) {
            "cpu flags ld/st"
        } else if (OFF_LB..OFF_LB + 16).contains(&off) {
            "cpu last_bits ld/st"
        } else if (OFF_XMM..OFF_XMM + 256).contains(&off) {
            "cpu xmm ld/st"
        } else if (OFF_GPR..OFF_GPR + 128).contains(&off) {
            "cpu gpr ld/st"
        } else {
            "cpu other ld/st"
        }
    };
    if (w & 0x3fff_fc00) == 0x38bf_c000 || (w & 0x3fe0_0c00) == 0x1940_0000 {
        return "ldapr (guest)";
    }
    if (w & 0x3fff_fc00) == 0x089f_fc00 || (w & 0x3fe0_0c00) == 0x1900_0000 {
        return "stlr (guest)";
    }
    if (w & 0x3fff_fc00) == 0x08df_fc00 {
        return "ldar";
    }
    if (w & 0xffff_f0ff) == 0xd503_30bf {
        return "dmb";
    }
    if (w & 0x3b00_0000) == 0x3900_0000 {
        // ld/st unsigned immediate.
        let size = w >> 30;
        let v = (w >> 26) & 1;
        let opc = (w >> 22) & 3;
        let scale = if v == 1 && size == 0 && opc & 2 != 0 { 16 } else { 1 << size };
        let off = ((w >> 10) & 0xfff) * scale;
        if rn == XCPU {
            return cpu_class(off);
        }
        if rn == 31 {
            return "host stack ld/st";
        }
        return "ldr/str (guest, plain)";
    }
    if (w & 0x3a00_0000) == 0x2800_0000 {
        // ldp/stp.
        if rn == XCPU {
            return "cpu ldp/stp";
        }
        if rn == 31 {
            return "host stack ldp/stp";
        }
        return "ldp/stp (other)";
    }
    if (w & 0x3b20_0000) == 0x3800_0000 || (w & 0x3b20_0c00) == 0x3820_0800 {
        return if rn == XCPU { "cpu other ld/st" } else { "ldr/str (other forms)" };
    }
    if (w & 0x7c00_0000) == 0x1400_0000 {
        return if w & 0x8000_0000 != 0 { "bl" } else { "b" };
    }
    if (w & 0xff00_0010) == 0x5400_0000 {
        return "b.cond";
    }
    if (w & 0x7e00_0000) == 0x3400_0000 {
        return "cbz/cbnz";
    }
    if (w & 0x7e00_0000) == 0x3600_0000 {
        return "tbz/tbnz";
    }
    if (w & 0xfe1f_fc1f) == 0xd61f_0000 {
        return "br/blr/ret";
    }
    if (w & 0xffff_ffe0) == 0xd53b_4200 || (w & 0xffff_ffe0) == 0xd51b_4200 {
        return "mrs/msr nzcv";
    }
    if (w & 0x1fe0_0000) == 0x1a80_0000 {
        return "csel/cset";
    }
    if (w & 0x1fe0_0000) == 0x1a40_0000 {
        return "ccmp";
    }
    if (w & 0x1f80_0000) == 0x1280_0000 {
        return "movz/movk/movn";
    }
    let addsub = (w & 0x1f00_0000) == 0x0b00_0000 || (w & 0x1f00_0000) == 0x1100_0000;
    let logic = (w & 0x1f00_0000) == 0x0a00_0000 || (w & 0x1f80_0000) == 0x1200_0000;
    let sets_flags = (addsub && (w >> 29) & 1 == 1) || (logic && (w >> 29) & 3 == 3);
    if sets_flags {
        return if rd == 31 { "cmp/tst" } else { "alu (flag-setting)" };
    }
    if rd == XS || rd == XA {
        return "alu -> x16/x17 (addr/check)";
    }
    "alu"
}

fn env_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let on = std::env::var("AW2_JIT").map_or(true, |s| s.trim() != "0");
        log::info!("emu: x86->arm64 block translator {}", if on { "enabled" } else { "disabled (AW2_JIT=0)" });
        on
    })
}

fn env_hot() -> u32 {
    static V: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_JIT_HOT").ok().and_then(|s| s.trim().parse().ok()).unwrap_or(2).max(1))
}

/// Whether the translator is enabled for this thread (`AW2_JIT=0` disables it).
pub(super) fn enabled() -> bool {
    match FORCE.with(|f| f.get()) {
        1 => true,
        2 => false,
        _ => env_enabled(),
    }
}

fn hot_threshold() -> u32 {
    if FORCE.with(|f| f.get()) == 1 {
        1
    } else {
        env_hot()
    }
}

// ---------------------------------------------------------------------------------------------
// Codegen knobs (default on; `=0` restores the previous code generation).
// ---------------------------------------------------------------------------------------------

/// Test-only per-thread overrides of the codegen knobs (`None` = environment). Set them on
/// a fresh thread before its first translation: the RAS decision is taken when the
/// thread's translation cache is created.
#[cfg(test)]
#[derive(Clone, Copy, Default)]
pub(super) struct TestKnobs {
    pub ras: Option<bool>,
    pub flags_dead: Option<u8>,
    pub ro_plain: Option<bool>,
    pub memchk: Option<u8>,
    pub pushpop: Option<bool>,
    pub budget_be: Option<bool>,
    /// Cap on the instruction budget handed to translated code (forces budget exits).
    pub budget_cap: Option<i64>,
    /// `AW2_JIT_XMMREG` ([`xmmreg_enabled`]).
    pub xmmreg: Option<bool>,
    /// `AW2_JIT_RETLIT` ([`retlit_enabled`]).
    pub retlit: Option<bool>,
    /// `AW2_JIT_FLAGLAZY` ([`flaglazy_enabled`]).
    pub flaglazy: Option<bool>,
    /// `AW2_DLL_AOT_PROBE` ([`aot_probe_enabled`]).
    pub aot_probe: Option<bool>,
    /// `PORT_JIT_XMM_PIN` ([`xmm_pin_enabled`]).
    pub xmm_pin: Option<bool>,
    /// `PORT_JIT_FAST_NAN` ([`fast_nan_enabled`]).
    pub fast_nan: Option<bool>,
    /// `PORT_JIT_LAST_BITS` ([`last_bits_enabled`]).
    pub last_bits: Option<bool>,
    /// `PORT_JIT_LSE2_TRAP` ([`lse2_trap_enabled`]).
    pub lse2_trap: Option<bool>,
    /// `PORT_JIT_HW_RANGE` ([`hw_range_enabled`]).
    pub hw_range: Option<bool>,
}

#[cfg(test)]
impl TestKnobs {
    /// Every knob from the environment.
    pub const NONE: Self = Self {
        ras: None,
        flags_dead: None,
        ro_plain: None,
        memchk: None,
        pushpop: None,
        budget_be: None,
        budget_cap: None,
        xmmreg: None,
        retlit: None,
        flaglazy: None,
        aot_probe: None,
        xmm_pin: None,
        fast_nan: None,
        last_bits: None,
        lse2_trap: None,
        hw_range: None,
    };
}

#[cfg(test)]
std::thread_local! {
    static TEST_KNOBS: core::cell::Cell<TestKnobs> = const { core::cell::Cell::new(TestKnobs::NONE) };
    /// Test-only read-only data ranges `[start, end)` for `AW2_JIT_RO_PLAIN` (the fuzz
    /// programs' constant pools are not in a PE image).
    static TEST_RO: core::cell::RefCell<Vec<(u64, u64)>> = const { core::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(super) fn test_set_knobs(k: TestKnobs) {
    TEST_KNOBS.with(|c| c.set(k));
}

#[cfg(test)]
pub(super) fn test_set_ro(ranges: Vec<(u64, u64)>) {
    TEST_RO.with(|c| *c.borrow_mut() = ranges);
}

fn env_knob(name: &str, default: u32) -> u32 {
    let v = std::env::var(name).ok().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(default);
    log::info!("emu jit: {name}={v}{}", if v == default { " (default)" } else { "" });
    v
}

/// `AW2_JIT_RAS` (default 1): shadow return-address stack. Translated guest calls push
/// `(guest return address, host landing pad)` on a per-thread ring and enter the callee
/// with `bl`; translated `ret` compares the popped guest return address with the top entry
/// and returns to the landing pad with a host `ret` (predicted by the CPU's return stack),
/// falling back to the `lookup_rip` hash probe. `0`: every `ret` goes through `lookup_rip`.
pub(super) fn ras_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().ras) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_RAS", 1) != 0)
}

/// `AW2_DLL_AOT_PROBE` (default 1; only with `AW2_DLL_AOT` > 0 or `AW2_DLL_AOT_BUILD`):
/// after a miss in the thread's index, the `lookup_*` stubs search the shared AOT probe
/// table ([`JitScratch::aot_dir`]) and continue in AOT code without leaving translated
/// code. Part of the AOT key (the stub layout of an AOT region depends on it). `0`, or
/// AOT off: exactly the previous stubs.
pub(super) fn aot_probe_enabled() -> bool {
    // Tests pick it per thread (default off: `aot_mode` is per-thread there).
    #[cfg(test)]
    return TEST_KNOBS.with(|c| c.get().aot_probe).unwrap_or(false);
    #[cfg(not(test))]
    {
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| {
            (crate::emu::aot::aot_mode() != 0 || crate::emu::aot::building())
                && env_knob("AW2_DLL_AOT_PROBE", 1) != 0
        })
    }
}

/// `AW2_JIT_FLAGS_DEAD_AT_CALL` bit mask (default 1): x86 flags are dead (not written
/// back to the `Cpu`, not computed for them) at
/// - bit 0: `call` (static, register, memory): the Win64 ABI leaves flags undefined across
///   calls, in both directions;
/// - bit 1: `ret` (riskier: hand-written code could return a result in the flags).
///
/// `0`: flags are live at every call/ret (previous behaviour).
fn flags_dead_mask() -> u8 {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().flags_dead) {
        return v;
    }
    #[cfg(test)]
    let def = 1u32;
    #[cfg(not(test))]
    let def = 3u32;
    static V: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *V.get_or_init(|| (env_knob("AW2_JIT_FLAGS_DEAD_AT_CALL", def) & 3) as u8)
}

/// `AW2_JIT_RO_PLAIN` (default 1): loads from an absolute / rip-relative address inside a
/// read-only, non-executable section of the exe or a mapped PE DLL (constants, vtables,
/// import slots) are a plain `ldr` without range check or acquire ordering.
fn ro_plain_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().ro_plain) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_RO_PLAIN", 1) != 0)
}

/// `[a, a + len)` lies in one read-only data section of the image containing guest code
/// address `code` (see [`ro_plain_enabled`]).
fn in_ro_data(code: u64, a: u64, len: u64) -> bool {
    #[cfg(test)]
    {
        let hit = TEST_RO.with(|c| c.borrow().iter().any(|&(s, e)| a >= s && a.saturating_add(len) <= e));
        if hit {
            return true;
        }
    }
    crate::dll_native::in_ro_section(code, a, len)
}

/// `AW2_JIT_MEMCHK` bit mask (default 15): cheaper guest memory checks.
/// - bit 0: one combined range + alignment test (`sub; and #0xffff8000_0000000N; cbnz`)
///   instead of `sub; ror; lsr; cbnz` for TSO accesses wider than a byte;
/// - bit 1: absolute / rip-relative operands (translation-time constant address) inside
///   `[0x10000, 2^47)`: no range check, and no alignment check when 8-byte aligned;
/// - bit 2: accesses off the same unchanged base register in a block share one range
///   check of the group's lowest address (later members only check alignment);
/// - bit 3: `[rsp+disp]` scalar moves use `ldapur`/`stlur [x23, #disp]` (FEAT_LRCPC2) with
///   no per-access alignment check once rsp is known 8-aligned in the block (checked once,
///   side exit; the access cannot cross a 16-byte boundary, so LSE2 never faults it).
///
/// `0`: previous checks.
fn memchk_mask() -> u8 {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().memchk) {
        return v;
    }
    static V: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *V.get_or_init(|| (env_knob("AW2_JIT_MEMCHK", 15) & 15) as u8)
}

/// `AW2_JIT_PUSHPOP` (default 1): `push`/`pop` of registers, `push imm`/`push [m]` and the
/// return-address push of guest calls use plain pre/post-indexed `str`/`ldr` (adjacent
/// pushes/pops fused into one `stp`/`ldp` with writeback): no alignment check, no rsp
/// copy. The slots are below rsp when written (never visible to another thread before a
/// later release store publishes them), like the plain `ret` load. `0`: previous code.
fn pushpop_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().pushpop) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_PUSHPOP", 1) != 0)
}

/// `AW2_JIT_BUDGET_BACKEDGE` (default 1): blocks still subtract their length from the
/// budget on entry (exact instruction counts), but only test it before a backward direct
/// transfer (target <= block start), a RAS `ret` and in the shared lookup stubs' hit paths
/// (every cycle of translated code contains one of those). `0`: test on every block entry.
pub(super) fn budget_backedge_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().budget_be) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_BUDGET_BACKEDGE", 1) != 0)
}

/// `PORT_JIT_BUDGET_LIGHT` (default 1 in production when profiling/stats/verify are off,
/// 0 in unit tests): only emit the `sub x8, x8, #n` budget decrement on blocks that actually
/// test the budget (`term_tests_budget`: backedges and RAS `ret`s), eliminating the serial
/// `sub x8` at the start of forward blocks and `call` blocks.
pub(super) fn budget_light_enabled() -> bool {
    if !budget_backedge_enabled() {
        return false;
    }
    #[cfg(test)]
    {
        return false;
    }
    #[cfg(not(test))]
    {
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| {
            if crate::emu::verify_enabled()
                || crate::emu::jit_stats::enabled()
                || crate::emu_prof::level() != 0
                || std::env::args().any(|a| a == "--diff-test")
            {
                return false;
            }
            env_knob("PORT_JIT_BUDGET_LIGHT", 1) != 0
        })
    }
}

/// `PORT_JIT_LAST_BITS` (default 1 when `aw2_registry_active()` or in `#[cfg(test)]`, else 0):
/// maintain `Cpu::last_bits` for Win64 `abi_gpr` masking of `DF_GAME_REPL` hooks.
/// In universal `port-runner` (no `DF_GAME_REPL` hooks), `last_bits` is never read.
pub(super) fn last_bits_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().last_bits) {
        return v;
    }
    #[cfg(test)]
    {
        if let Ok(s) = std::env::var("PORT_JIT_LAST_BITS") {
            if let Ok(v) = s.trim().parse::<u32>() {
                return v != 0;
            }
        }
        return true;
    }
    #[cfg(not(test))]
    {
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| {
            let def = u32::from(crate::exe::aw2_registry_active());
            env_knob("PORT_JIT_LAST_BITS", def) != 0
        })
    }
}

static LSE2_TRAP_READY: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Mark that the host process has installed a `SIGBUS` alignment-fault handler (`DFSC == 0x21`)
/// capable of emulating 16-byte-boundary-crossing `ldapr`/`ldapur`/`stlr`/`stlur` accesses.
pub(super) fn enable_lse2_trap() {
    LSE2_TRAP_READY.store(true, core::sync::atomic::Ordering::Release);
}

/// `PORT_JIT_LSE2_TRAP` (default 1 when the host `SIGBUS` LSE2 handler is installed, else 0):
/// rely on Apple Silicon `FEAT_LSE2` hardware support for unaligned acquire/release accesses
/// within a 16-byte granule plus the `SIGBUS` handler for rare 16-byte boundary crossings.
/// Eliminates all per-access alignment checks, `SlowMem` stubs, and `rsp_al` guards, and
/// emits single-instruction `ldapur`/`stlur [base, #disp]` for `[base + disp]` operands.
pub(super) fn lse2_trap_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().lse2_trap) {
        return v;
    }
    #[cfg(test)]
    {
        if let Ok(s) = std::env::var("PORT_JIT_LSE2_TRAP") {
            if let Ok(v) = s.trim().parse::<u32>() {
                return v != 0;
            }
        }
        return LSE2_TRAP_READY.load(core::sync::atomic::Ordering::Acquire);
    }
    #[cfg(not(test))]
    {
        if !LSE2_TRAP_READY.load(core::sync::atomic::Ordering::Acquire) {
            return false;
        }
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| env_knob("PORT_JIT_LSE2_TRAP", 1) != 0)
    }
}

static HW_RANGE_READY: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Mark that the host process has installed hardware fault handlers (`SIGSEGV`/`SIGBUS`) so
/// translated code can omit software `[0x10000, 2^47)` range checks and rely on Darwin's
/// 4 GiB `__PAGEZERO` + 47-bit user VA space.
pub(super) fn enable_hw_range() {
    HW_RANGE_READY.store(true, core::sync::atomic::Ordering::Release);
}

/// `PORT_JIT_HW_RANGE` (default 1 when host signal handlers are installed, else 0):
/// rely on hardware MMU faults for out-of-range guest pointers instead of emitting 3-insn
/// `sub`/`lsr`/`cbnz` + side-exit stubs on every heap memory access.
pub(super) fn hw_range_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().hw_range) {
        return v;
    }
    #[cfg(test)]
    {
        if let Ok(s) = std::env::var("PORT_JIT_HW_RANGE") {
            if let Ok(v) = s.trim().parse::<u32>() {
                return v != 0;
            }
        }
        return HW_RANGE_READY.load(core::sync::atomic::Ordering::Acquire);
    }
    #[cfg(not(test))]
    {
        if !HW_RANGE_READY.load(core::sync::atomic::Ordering::Acquire) {
            return false;
        }
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| env_knob("PORT_JIT_HW_RANGE", 1) != 0)
    }
}

#[cfg(test)]
fn test_budget_cap(b: i64) -> i64 {
    TEST_KNOBS.with(|c| c.get().budget_cap).map_or(b, |cap| b.min(cap))
}

// ---------------------------------------------------------------------------------------------
// Host register assignment & Cpu layout constants.
// ---------------------------------------------------------------------------------------------

/// Host register holding guest GPR `i` (rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8..r15).
const HG: [u32; 16] = [19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 9, 10, 11, 12, 13, 14];
const XCPU: u32 = 15;
const XBUD: u32 = 8;
const T0: u32 = 0;
const T1: u32 = 1;
const T2: u32 = 2;
const T3: u32 = 3;
const T4: u32 = 4;
const T5: u32 = 5;
const T6: u32 = 6;
/// Pinned shadow return-address stack top pointer (`Cpu::jx.ras`) while in translated code.
const XRAS: u32 = 7;
/// Address temp.
const XA: u32 = 16;
/// Scratch for checks / large constants.
const XS: u32 = 17;
const ZR: u32 = 31;

#[inline(always)]
fn g(i: u8) -> u32 {
    HG[i as usize & 15]
}

/// GPRs whose `last_bits` matter (read by `abi_gpr`).
const TRACKED: [u8; 5] = [0, 1, 2, 8, 9];
#[inline(always)]
fn tracked(i: u8) -> bool {
    matches!(i, 0 | 1 | 2 | 8 | 9)
}
const LB_NONE: u8 = 0xFF;

// Cpu field offsets.
const OFF_GPR: u32 = offset_of!(Cpu, gpr) as u32;
const OFF_RIP: u32 = offset_of!(Cpu, rip) as u32;
const OFF_XMM: u32 = offset_of!(Cpu, xmm) as u32;
const OFF_YMM_HI: u32 = offset_of!(Cpu, ymm_hi) as u32;
const OFF_CF: u32 = offset_of!(Cpu, cf) as u32;
const OFF_ZF: u32 = offset_of!(Cpu, zf) as u32;
const OFF_SF: u32 = offset_of!(Cpu, sf) as u32;
const OFF_OF: u32 = offset_of!(Cpu, of) as u32;
const OFF_PFV: u32 = offset_of!(Cpu, pfv) as u32;
const OFF_DF: u32 = offset_of!(Cpu, df) as u32;
const OFF_LB: u32 = offset_of!(Cpu, last_bits) as u32;
const OFF_JX: u32 = offset_of!(Cpu, jx) as u32;
const OFF_BUDGET: u32 = OFF_JX + offset_of!(JitScratch, budget) as u32;
const OFF_TARGET: u32 = OFF_JX + offset_of!(JitScratch, target) as u32;
const OFF_TABLE: u32 = OFF_JX + offset_of!(JitScratch, table) as u32;
const OFF_CODEGEN: u32 = OFF_JX + offset_of!(JitScratch, codegen) as u32;
const OFF_TEB: u32 = OFF_JX + offset_of!(JitScratch, teb) as u32;
const OFF_RAS: u32 = OFF_JX + offset_of!(JitScratch, ras) as u32;
const OFF_AOTDIR: u32 = OFF_JX + offset_of!(JitScratch, aot_dir) as u32;
const OFF_PROBEHITS: u32 = OFF_JX + offset_of!(JitScratch, aot_probe_hits) as u32;
const OFF_TSC_FROZEN: u32 = OFF_JX + offset_of!(JitScratch, tsc_frozen) as u32;
const OFF_YMM_DIRTY: u32 = OFF_JX + offset_of!(JitScratch, ymm_dirty) as u32;
const OFF_MXCSR: u32 = OFF_JX + offset_of!(JitScratch, mxcsr) as u32;
#[cfg(test)]
const OFF_PLAINHITS: u32 = OFF_JX + offset_of!(JitScratch, plain_call_hits) as u32;
const _: () = assert!(
    OFF_AOTDIR % 8 == 0
        && OFF_PROBEHITS % 8 == 0
        && OFF_PROBEHITS < 8 * 4096
        && OFF_TSC_FROZEN < 4096
        && OFF_YMM_DIRTY % 2 == 0
        && OFF_YMM_DIRTY < 2 * 4096
        && OFF_MXCSR % 4 == 0
        && OFF_MXCSR < 4 * 4096
        && OFF_YMM_HI % 8 == 0
        && OFF_YMM_HI <= 504
);
const _: () = assert!(OFF_GPR == 0);
const _: () = assert!(OFF_TABLE + 8 == OFF_JX + offset_of!(JitScratch, mul) as u32);
const _: () = assert!(OFF_CF + 1 == OFF_ZF && OFF_ZF + 1 == OFF_SF && OFF_SF + 1 == OFF_OF);

// arm64 condition codes.
const EQ: u32 = 0;
const NE: u32 = 1;
const HS: u32 = 2;
const LO: u32 = 3;
const MI: u32 = 4;
const PL: u32 = 5;
const VS: u32 = 6;
const VC: u32 = 7;
const HI: u32 = 8;
const LS: u32 = 9;
const GE: u32 = 10;
const LT: u32 = 11;
const GT: u32 = 12;
const LE: u32 = 13;

// x86 flag bits (liveness sets).
pub(super) const FC: u8 = 1;
pub(super) const FZ: u8 = 2;
pub(super) const FS: u8 = 4;
pub(super) const FO: u8 = 8;
pub(super) const FP: u8 = 16;
const FNZCV: u8 = FC | FZ | FS | FO;
pub(super) const FALL: u8 = 31;

/// Flags read by x86 condition code `cc` (iced numbering, as in `FastOp::x`).
fn cc_reads(cc: u8) -> u8 {
    match cc {
        1 | 2 => FO,
        3 | 4 => FC,
        5 | 6 => FZ,
        7 | 8 => FC | FZ,
        9 | 10 => FS,
        11 | 12 => FP,
        13 | 14 => FS | FO,
        15 | 16 => FZ | FS | FO,
        _ => FALL,
    }
}

const A_ADD: u8 = 0;
const A_SUB: u8 = 1;
const A_AND: u8 = 2;
const A_OR: u8 = 3;
const A_XOR: u8 = 4;
const A_CMP: u8 = 5;
const A_TEST: u8 = 6;
