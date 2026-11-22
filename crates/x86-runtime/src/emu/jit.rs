//! x86-64 -> arm64 block translator for [`super::emu_run_full`] (aarch64 hosts only).
//!
//! Hot guest blocks (the per-thread [`BlockCache`] blocks, already lowered to
//! [`FastOp`]s) are translated into arm64 code in a per-thread `MAP_JIT` buffer. Translation
//! covers the fast-op prefix of a block; the first generic instruction ends the translated
//! code and the interpreter takes over there.
//!
//! # Execution model
//! - The interpreter enters translated code through the shared `enter` stub, which saves
//!   the host callee-saved registers and loads all 16 guest GPRs into pinned host registers
//!   ([`HG`]). Translated blocks jump to each other directly (patched links, or the
//!   `lookup_*` stubs for indirect targets) without leaving translated code; the shared
//!   `exit` stub writes the GPRs back and returns an exit code to the interpreter.
//! - Flags: within a block, x86 flags live in the host NZCV flags where possible
//!   ([`Src`]); PF is stored eagerly as the parity byte [`Cpu::pfv`]. Flags are only
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

use super::fast::{self, FastOp, RNONE};
use super::{mem, to_mapped_addr, BlockCache, Cpu, BLK_MAX_INSNS};
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
    /// Address of the executable-memory generation counter ([`mem::code_gen`]).
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
    /// 64 KiB-aligned ring ([`JitCache::ras_top`] while translated code runs).
    pub ras: u64,
    /// `&JitCache::ras_top` of the running translation cache, so [`super::jit_host_xfer`]
    /// can hand the top to nested runs on this thread (0 when the RAS is off).
    pub ctx_ras_cell: usize,
    /// Shared AOT probe table ([`super::aot::probe_table`], `AW2_DLL_AOT_PROBE`) searched by
    /// the `lookup_*` stubs after a miss in [`Self::table`]; 0 = none.
    pub aot_dir: usize,
    /// `lookup_*` misses served from [`Self::aot_dir`] (drained by [`JitCache::run`]).
    pub aot_probe_hits: u64,
    /// Reserved padding (keeps `ymm_dirty` 2-byte aligned).
    pub _pad_xmm: u16,
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
            _pad_xmm: 0,
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

/// Windows `__chkstk` (treated specially by the interpreter's call/jmp handling).
const CHKSTK: u64 = 0x141835480;

/// AW2 exe or byte-detected DLL `__chkstk` VA.
#[inline(always)]
fn is_chkstk(ip: u64) -> bool {
    (ip == CHKSTK && crate::exe::aw2_registry_active()) || super::is_routed_dll_chkstk(ip)
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
        *V.get_or_init(|| (super::aot::aot_mode() != 0 || super::aot::building()) && env_knob("AW2_DLL_AOT_PROBE", 1) != 0)
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
            if super::verify_enabled()
                || super::jit_stats::enabled()
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

/// GPRs (bit i = guest register i) an instruction may write, implicit operands included.
fn gpr_writes(ins: &iced_x86::Instruction, f: &mut iced_x86::InstructionInfoFactory) -> u16 {
    use iced_x86::OpAccess as A;
    let mut m = 0u16;
    for u in f.info(ins).used_registers() {
        if !matches!(u.access(), A::Write | A::CondWrite | A::ReadWrite | A::ReadCondWrite) {
            continue;
        }
        let r = u.register().full_register();
        if r.is_gpr64() {
            m |= 1 << r.number();
        }
    }
    m
}

// ---------------------------------------------------------------------------------------------
// Host register assignment.
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
const OFF_YMM_DIRTY: u32 = OFF_JX + offset_of!(JitScratch, ymm_dirty) as u32;
const OFF_MXCSR: u32 = OFF_JX + offset_of!(JitScratch, mxcsr) as u32;
#[cfg(test)]
const OFF_PLAINHITS: u32 = OFF_JX + offset_of!(JitScratch, plain_call_hits) as u32;
const _: () = assert!(
    OFF_AOTDIR % 8 == 0
        && OFF_PROBEHITS % 8 == 0
        && OFF_PROBEHITS < 8 * 4096
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

// ---------------------------------------------------------------------------------------------
// arm64 assembler.
// ---------------------------------------------------------------------------------------------

/// Encoding of a logical immediate (`N:immr:imms` in bits 22..10), if `v` is one.
fn enc_logic_imm(v: u64, x: bool) -> Option<u32> {
    let v = if x { v } else { (v & 0xFFFF_FFFF) | (v << 32) };
    if v == 0 || v == u64::MAX {
        return None;
    }
    let mut size = 64u32;
    while size > 2 {
        let half = size / 2;
        let mask = (1u64 << half) - 1;
        if (v & mask) != ((v >> half) & mask) {
            break;
        }
        size = half;
    }
    let mask = if size == 64 { u64::MAX } else { (1u64 << size) - 1 };
    let elt = v & mask;
    let ones = elt.count_ones();
    let target = (1u64 << ones) - 1;
    for r in 0..size {
        let rot = if r == 0 { elt } else { ((elt >> r) | (elt << (size - r))) & mask };
        if rot == target {
            let immr = (size - r) % size;
            let imms = (((size as i64).wrapping_neg() << 1) as u64 as u32 | (ones - 1)) & 0x3F;
            let n = (size == 64) as u32;
            return Some((n << 22) | (immr << 16) | (imms << 10));
        }
    }
    None
}

struct Asm {
    base: *mut u32,
    /// Address word 0 executes at (`b_to`/`bl_to` targets are absolute): `base` for the
    /// JIT, the final AOT region address for AOT builds (which emit into a scratch buffer).
    vbase: usize,
    pos: usize,
    cap: usize,
    overflow: bool,
}

impl Asm {
    fn new(base: *mut u32, vbase: usize, cap: usize) -> Self {
        Asm { base, vbase, pos: 0, cap, overflow: false }
    }
    #[inline]
    fn emit(&mut self, w: u32) {
        if self.pos < self.cap {
            unsafe { *self.base.add(self.pos) = w };
            self.pos += 1;
        } else {
            self.overflow = true;
        }
    }
    fn addr_of(&self, pos: usize) -> usize {
        self.vbase + pos * 4
    }
    fn read(&self, pos: usize) -> u32 {
        unsafe { *self.base.add(pos) }
    }
    /// Point the branch at `at` to `target` (both word positions in this buffer).
    fn patch(&mut self, at: usize, target: usize) {
        if at >= self.cap || self.overflow {
            return;
        }
        let off = target as i64 - at as i64;
        let ins = self.read(at);
        let new = patch_branch(ins, off);
        unsafe { *self.base.add(at) = new };
    }
    fn bind(&mut self, at: usize) {
        let here = self.pos;
        self.patch(at, here);
    }

    // --- data processing ---
    fn addsub_rr(&mut self, x: bool, sub: bool, s: bool, d: u32, n: u32, m: u32, lsl: u32) {
        self.emit(0x0B00_0000 | sf(x) | (sub as u32) << 30 | (s as u32) << 29 | m << 16 | lsl << 10 | n << 5 | d);
    }
    fn addsub_ri(&mut self, x: bool, sub: bool, s: bool, d: u32, n: u32, imm12: u32, sh12: bool) {
        self.emit(0x1100_0000 | sf(x) | (sub as u32) << 30 | (s as u32) << 29 | (sh12 as u32) << 22 | (imm12 & 0xFFF) << 10 | n << 5 | d);
    }
    /// `opc`: 0 and, 1 orr, 2 eor, 3 ands; `shift`: 0 lsl, 1 lsr, 2 asr, 3 ror.
    fn logic_rr(&mut self, x: bool, opc: u32, d: u32, n: u32, m: u32, shift: u32, amt: u32) {
        self.emit(0x0A00_0000 | sf(x) | opc << 29 | shift << 22 | m << 16 | amt << 10 | n << 5 | d);
    }
    fn logic_ri(&mut self, x: bool, opc: u32, d: u32, n: u32, enc: u32) {
        self.emit(0x1200_0000 | sf(x) | opc << 29 | enc | n << 5 | d);
    }
    fn mov_rr(&mut self, x: bool, d: u32, m: u32) {
        self.logic_rr(x, 1, d, ZR, m, 0, 0);
    }
    fn movz(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x5280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    fn movk(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x7280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    fn movn(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x1280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    fn mov_imm(&mut self, x: bool, d: u32, v: u64) {
        let nch = if x { 4 } else { 2 };
        let v = if x { v } else { v & 0xFFFF_FFFF };
        let ch = |i: u32| ((v >> (16 * i)) & 0xFFFF) as u32;
        let zeros = (0..nch).filter(|&i| ch(i) == 0).count() as u32;
        let ones = (0..nch).filter(|&i| ch(i) == 0xFFFF).count() as u32;
        if nch - zeros > 1 && nch - ones > 1 {
            if let Some(e) = enc_logic_imm(v, x) {
                self.logic_ri(x, 1, d, ZR, e);
                return;
            }
        }
        if ones > zeros {
            let mut first = true;
            for i in 0..nch {
                if ch(i) != 0xFFFF {
                    if first {
                        self.movn(x, d, !ch(i) & 0xFFFF, i);
                        first = false;
                    } else {
                        self.movk(x, d, ch(i), i);
                    }
                }
            }
            if first {
                self.movn(x, d, 0, 0);
            }
        } else {
            let mut first = true;
            for i in 0..nch {
                if ch(i) != 0 {
                    if first {
                        self.movz(x, d, ch(i), i);
                        first = false;
                    } else {
                        self.movk(x, d, ch(i), i);
                    }
                }
            }
            if first {
                self.movz(x, d, 0, 0);
            }
        }
    }
    fn ubfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x5300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    fn sbfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x1300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    fn bfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x3300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    /// Copy the low `w` bits of `n` into `d` (other bits of `d` kept).
    fn bfxil0(&mut self, d: u32, n: u32, w: u32) {
        self.bfm(true, d, n, 0, w - 1);
    }
    /// `ubfx d, n, #lsb, #width` (64-bit).
    fn ubfx(&mut self, d: u32, n: u32, lsb: u32, width: u32) {
        self.ubfm(true, d, n, lsb, lsb + width - 1);
    }
    fn lsl_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.ubfm(x, d, n, (bits - s) % bits, bits - 1 - s);
    }
    fn lsr_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.ubfm(x, d, n, s, bits - 1);
    }
    fn asr_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.sbfm(x, d, n, s, bits - 1);
    }
    fn ror_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        self.emit(0x1380_0000 | sf(x) | (x as u32) << 22 | n << 16 | s << 10 | n << 5 | d);
    }
    /// `op2`: 8 lslv, 9 lsrv, 10 asrv.
    fn shiftv(&mut self, x: bool, op2: u32, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0000 | sf(x) | m << 16 | op2 << 10 | n << 5 | d);
    }
    fn csel(&mut self, x: bool, d: u32, n: u32, m: u32, cond: u32) {
        self.emit(0x1A80_0000 | sf(x) | m << 16 | cond << 12 | n << 5 | d);
    }
    fn csinc(&mut self, x: bool, d: u32, n: u32, m: u32, cond: u32) {
        self.emit(0x1A80_0400 | sf(x) | m << 16 | cond << 12 | n << 5 | d);
    }
    fn cset(&mut self, d: u32, cond: u32) {
        // csinc wd, wzr, wzr, !cond
        self.emit(0x1A80_0400 | ZR << 16 | (cond ^ 1) << 12 | ZR << 5 | d);
    }
    fn clz(&mut self, x: bool, d: u32, n: u32) {
        self.emit(0x5AC0_1000 | sf(x) | n << 5 | d);
    }
    fn rbit(&mut self, x: bool, d: u32, n: u32) {
        self.emit(0x5AC0_0000 | sf(x) | n << 5 | d);
    }
    fn rev(&mut self, x: bool, d: u32, n: u32) {
        self.emit(if x { 0xDAC0_0C00 } else { 0x5AC0_0800 } | n << 5 | d);
    }
    fn mul(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1B00_0000 | sf(x) | m << 16 | ZR << 10 | n << 5 | d);
    }
    fn smulh(&mut self, d: u32, n: u32, m: u32) {
        self.emit(0x9B40_7C00 | m << 16 | n << 5 | d);
    }
    fn umulh(&mut self, d: u32, n: u32, m: u32) {
        self.emit(0x9BC0_7C00 | m << 16 | n << 5 | d);
    }
    fn udiv(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0800 | sf(x) | m << 16 | n << 5 | d);
    }
    fn sdiv(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0C00 | sf(x) | m << 16 | n << 5 | d);
    }
    fn msub(&mut self, x: bool, d: u32, n: u32, m: u32, a: u32) {
        self.emit(0x1B00_8000 | sf(x) | m << 16 | a << 10 | n << 5 | d);
    }
    // --- scalar FP / conversions ---
    fn fmov_gf(&mut self, is_f64: bool, vd: u32, n: u32) {
        self.emit(0x1E27_0000 | sf(is_f64) | (is_f64 as u32) << 22 | n << 5 | vd);
    }
    fn fmov_fg(&mut self, is_f64: bool, d: u32, vn: u32) {
        self.emit(0x1E26_0000 | sf(is_f64) | (is_f64 as u32) << 22 | vn << 5 | d);
    }
    fn scvtf_gp(&mut self, x: bool, is_f64: bool, vd: u32, n: u32) {
        self.emit(0x1E22_0000 | sf(x) | (is_f64 as u32) << 22 | n << 5 | vd);
    }
    fn fcmp(&mut self, is_f64: bool, vn: u32, vm: u32) {
        self.emit(0x1E20_2000 | (is_f64 as u32) << 22 | vm << 16 | vn << 5);
    }
    fn fcvt_d_s(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E22_C000 | vn << 5 | vd);
    }
    fn fcvt_s_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E62_4000 | vn << 5 | vd);
    }
    fn frintn_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E64_4000 | vn << 5 | vd);
    }
    fn frintz_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E65_C000 | vn << 5 | vd);
    }
    fn fcvtzs_d(&mut self, x: bool, d: u32, vn: u32) {
        self.emit(0x1E78_0000 | sf(x) | vn << 5 | d);
    }
    /// Flag-free `d = n + v`.
    fn add_imm(&mut self, x: bool, d: u32, n: u32, v: i64) {
        let (sub, a) = if v < 0 { (true, v.unsigned_abs()) } else { (false, v as u64) };
        if a < 4096 {
            self.addsub_ri(x, sub, false, d, n, a as u32, false);
        } else if a & 0xFFF == 0 && a < (1 << 24) {
            self.addsub_ri(x, sub, false, d, n, (a >> 12) as u32, true);
        } else {
            self.mov_imm(x, XS, v as u64);
            self.addsub_rr(x, false, false, d, n, XS, 0);
        }
    }

    // --- loads / stores ---
    /// Unsigned-offset load/store of `bits` (8/16/32/64/128); `ld` selects load.
    fn ldst_ui(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: u32) {
        let (base, scale) = match bits {
            8 => (0x3900_0000, 0),
            16 => (0x7900_0000, 1),
            32 => (0xB900_0000, 2),
            64 => (0xF900_0000, 3),
            _ => (0x3D80_0000, 4),
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        assert!(off % (1 << scale) == 0 && (off >> scale) < 4096, "ldst offset {off}");
        self.emit(base | opc | (off >> scale) << 10 | n << 5 | t);
    }
    fn ldr(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        self.ldst_ui(true, bits, t, n, off);
    }
    fn str(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        self.ldst_ui(false, bits, t, n, off);
    }
    fn stp_x(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "stp offset {off}");
        self.emit(0xA900_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    fn ldp_x(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "ldp offset {off}");
        self.emit(0xA940_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    fn stp_q(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 16 == 0 && (-1024..=1008).contains(&off), "stp q offset {off}");
        self.emit(0xAD00_0000 | (((off / 16) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    fn ldp_q(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 16 == 0 && (-1024..=1008).contains(&off), "ldp q offset {off}");
        self.emit(0xAD40_0000 | (((off / 16) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    fn ldur_q(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldur q offset {off}");
        self.emit(0x3CC0_0000 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    fn stur_q(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "stur q offset {off}");
        self.emit(0x3C80_0000 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    fn ldapr(&mut self, bits: u32, t: u32, n: u32) {
        let base = match bits {
            8 => 0x38BF_C000,
            16 => 0x78BF_C000,
            32 => 0xB8BF_C000,
            _ => 0xF8BF_C000,
        };
        self.emit(base | n << 5 | t);
    }
    fn stlr(&mut self, bits: u32, t: u32, n: u32) {
        let base = match bits {
            8 => 0x089F_FC00,
            16 => 0x489F_FC00,
            32 => 0x889F_FC00,
            _ => 0xC89F_FC00,
        };
        self.emit(base | n << 5 | t);
    }
    /// `ldur{b,h,}`/`stur{b,h,}` (unscaled signed 9-bit offset, plain non-TSO).
    fn ldur_stur(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldur/stur offset {off}");
        let size = match bits {
            8 => 0u32,
            16 => 1,
            32 => 2,
            _ => 3,
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        self.emit(0x3800_0000 | size << 30 | opc | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `ldapur{b,h,}`/`stlur{b,h,}` (FEAT_LRCPC2): RCpc load-acquire / store-release with a
    /// signed 9-bit unscaled offset.
    fn ldapur_stlur(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldapur/stlur offset {off}");
        let size = match bits {
            8 => 0u32,
            16 => 1,
            32 => 2,
            _ => 3,
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        self.emit(0x1900_0000 | size << 30 | opc | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `str xt, [xn, #off]!` (pre-index).
    fn str_pre(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "str pre offset {off}");
        self.emit(0xF800_0C00 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `ldr xt, [xn], #off` (post-index).
    fn ldr_post(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldr post offset {off}");
        self.emit(0xF840_0400 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `stp xt1, xt2, [xn, #off]!` (pre-index).
    fn stp_pre(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "stp pre offset {off}");
        self.emit(0xA980_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    /// `ldp xt1, xt2, [xn], #off` (post-index; t1 != t2, neither == n).
    fn ldp_post(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off) && t1 != t2 && t1 != n && t2 != n);
        self.emit(0xA8C0_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    fn dmb_ish(&mut self) {
        self.emit(0xD503_3BBF);
    }
    fn dmb_ishld(&mut self) {
        self.emit(0xD503_39BF);
    }
    fn dmb_ishst(&mut self) {
        self.emit(0xD503_3ABF);
    }
    /// `mrs xt, cntvct_el0` (reads the virtual timer count directly without ISB).
    fn mrs_cntvct(&mut self, t: u32) {
        self.emit(0xD53B_E040 | t);
    }
    /// `mrs xt, fpcr` (reads ARM64 floating-point control register).
    fn mrs_fpcr(&mut self, t: u32) {
        self.emit(0xD53B_4400 | t);
    }
    /// `msr fpcr, xt` (writes ARM64 floating-point control register).
    fn msr_fpcr(&mut self, t: u32) {
        self.emit(0xD51B_4400 | t);
    }

    // --- branches (offsets patched later unless given) ---
    fn b(&mut self) -> usize {
        self.emit(0x1400_0000);
        self.pos - 1
    }
    fn b_to(&mut self, target_addr: usize) {
        let here = self.addr_of(self.pos);
        let off = (target_addr as i64 - here as i64) / 4;
        self.emit(0x1400_0000 | (off as u32 & 0x03FF_FFFF));
    }
    fn bcond(&mut self, cond: u32) -> usize {
        self.emit(0x5400_0000 | cond);
        self.pos - 1
    }
    fn cbz(&mut self, x: bool, t: u32) -> usize {
        self.emit(0x3400_0000 | sf(x) | t);
        self.pos - 1
    }
    fn cbnz(&mut self, x: bool, t: u32) -> usize {
        self.emit(0x3500_0000 | sf(x) | t);
        self.pos - 1
    }
    fn tbnz(&mut self, t: u32, bit: u32) -> usize {
        self.emit(0x3700_0000 | (bit >> 5) << 31 | (bit & 31) << 19 | t);
        self.pos - 1
    }
    fn br(&mut self, n: u32) {
        self.emit(0xD61F_0000 | n << 5);
    }
    fn ret(&mut self) {
        self.emit(0xD65F_03C0);
    }
    /// `ret xn` (return-predicted indirect branch).
    fn ret_r(&mut self, n: u32) {
        self.emit(0xD65F_0000 | n << 5);
    }
    /// `bl` placeholder (patched later like [`Self::b`]; the link bit is preserved).
    fn bl(&mut self) -> usize {
        self.emit(0x9400_0000);
        self.pos - 1
    }
    fn bl_to(&mut self, target_addr: usize) {
        let here = self.addr_of(self.pos);
        let off = (target_addr as i64 - here as i64) / 4;
        self.emit(0x9400_0000 | (off as u32 & 0x03FF_FFFF));
    }
    /// `adr xd, .+off` placeholder; set the offset with [`Self::patch_adr`].
    fn adr(&mut self, d: u32) -> usize {
        self.emit(0x1000_0000 | d);
        self.pos - 1
    }
    /// Point the `adr` at word `at` to word `target` of this buffer.
    fn patch_adr(&mut self, at: usize, target: usize) {
        if at >= self.cap || self.overflow {
            return;
        }
        let off = ((target as i64 - at as i64) * 4) as u32;
        let ins = self.read(at) & 0x9F00_001F;
        let new = ins | (off & 3) << 29 | ((off >> 2) & 0x7_FFFF) << 5;
        unsafe { *self.base.add(at) = new };
    }
    fn blr(&mut self, n: u32) {
        self.emit(0xD63F_0000 | n << 5);
    }
}

#[inline(always)]
fn sf(x: bool) -> u32 {
    (x as u32) << 31
}

/// Re-target branch instruction `ins` by `off` words.
fn patch_branch(ins: u32, off: i64) -> u32 {
    let o = off as u32;
    if ins & 0x7C00_0000 == 0x1400_0000 {
        (ins & 0xFC00_0000) | (o & 0x03FF_FFFF)
    } else if ins & 0xFF00_0010 == 0x5400_0000 || ins & 0x7E00_0000 == 0x3400_0000 {
        (ins & 0xFF00_001F) | ((o & 0x7FFFF) << 5)
    } else if ins & 0x7E00_0000 == 0x3600_0000 {
        (ins & 0xFFF8_001F) | ((o & 0x3FFF) << 5)
    } else {
        ins
    }
}

// ---------------------------------------------------------------------------------------------
// Translation cache.
// ---------------------------------------------------------------------------------------------

use super::exec_mem::{alloc_jit, icache_invalidate, jit_writable, jit_write_protect};

const CODE_SIZE: usize = 128 << 20;
/// Tail of each `JitCache` `MAP_JIT` region reserved for 16-byte exact-TSO unaligned LSE2
/// self-patching islands (within ±128 MiB of every instruction in the cache).
const JIT_LSE2_SLAB: usize = 256 << 10;
const CODE_LIMIT: usize = CODE_SIZE - JIT_LSE2_SLAB;

/// Lock-free, async-signal-safe registry of `MAP_JIT` code regions and their paired LSE2
/// exact-TSO unaligned-access patch slabs (used by both `JitCache` and `aot.rs`).
const MAX_LSE2_REGIONS: usize = 128;

struct Lse2Region {
    code_lo: core::sync::atomic::AtomicUsize,
    code_hi: core::sync::atomic::AtomicUsize,
    slab_cur: core::sync::atomic::AtomicUsize,
    slab_end: core::sync::atomic::AtomicUsize,
}

impl Lse2Region {
    const fn new() -> Self {
        Self {
            code_lo: core::sync::atomic::AtomicUsize::new(0),
            code_hi: core::sync::atomic::AtomicUsize::new(0),
            slab_cur: core::sync::atomic::AtomicUsize::new(0),
            slab_end: core::sync::atomic::AtomicUsize::new(0),
        }
    }
}

static LSE2_REGIONS: [Lse2Region; MAX_LSE2_REGIONS] = [const { Lse2Region::new() }; MAX_LSE2_REGIONS];
static LSE2_REGION_N: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// Register `[code_lo, code_hi)` with paired patch slab `[slab_lo, slab_hi)` (same `MAP_JIT`
/// mapping, within ±128 MiB). Returns the slot index if registered.
pub(super) fn register_lse2_region(code_lo: usize, code_hi: usize, slab_lo: usize, slab_hi: usize) -> Option<usize> {
    use core::sync::atomic::Ordering;
    let idx = LSE2_REGION_N.fetch_add(1, Ordering::AcqRel);
    if idx >= MAX_LSE2_REGIONS {
        return None;
    }
    let r = &LSE2_REGIONS[idx];
    r.code_lo.store(code_lo, Ordering::Relaxed);
    r.slab_cur.store(slab_lo, Ordering::Relaxed);
    r.slab_end.store(slab_hi, Ordering::Relaxed);
    // Publish last with Release so signal handlers observing `code_hi != 0` see all fields.
    r.code_hi.store(code_hi, Ordering::Release);
    Some(idx)
}

/// Reset a `JitCache` slot's patch slab cursor when the thread flushes its code buffer.
fn reset_lse2_slab(idx: usize, slab_lo: usize) {
    if idx < MAX_LSE2_REGIONS {
        LSE2_REGIONS[idx].slab_cur.store(slab_lo, core::sync::atomic::Ordering::Release);
    }
}

/// Async-signal-safe, lock-free self-patching of a faulting `ldapr`/`ldapur`/`stlr`/`stlur`
/// instruction inside a registered `MAP_JIT` region to a 16-byte exact-TSO island
/// (`ldur; dmb ishld; b pc+4` or `dmb ishst; stur; b pc+4`). Subsequent executions of `pc`
/// run natively with zero `SIGBUS` traps while preserving exact x86 TSO semantics.
pub(super) unsafe fn patch_lse2_unaligned(pc: usize, insn: u32) -> bool {
    use core::sync::atomic::{AtomicU32, Ordering};
    if !jit_writable() {
        return false;
    }
    let n = LSE2_REGION_N.load(Ordering::Acquire).min(MAX_LSE2_REGIONS);
    let mut found = None;
    for r in &LSE2_REGIONS[..n] {
        let hi = r.code_hi.load(Ordering::Acquire);
        if hi == 0 {
            continue;
        }
        let lo = r.code_lo.load(Ordering::Relaxed);
        if pc >= lo && pc < hi {
            found = Some(r);
            break;
        }
    }
    let Some(r) = found else {
        return false;
    };
    // Already patched concurrently by another thread?
    if (*(pc as *const AtomicU32)).load(Ordering::Acquire) != insn {
        return true;
    }
    let size = insn >> 30;
    let rn = (insn >> 5) & 31;
    let rt = insn & 31;
    let (is_load, plain_word) = if (insn & 0x3FFF_FC00) == 0x38BF_C000 {
        // ldapr -> ldur rt, [rn, #0]
        (true, 0x3840_0000 | size << 30 | rn << 5 | rt)
    } else if (insn & 0x3FE0_0C00) == 0x1940_0000 {
        // ldapur -> ldur rt, [rn, #imm9]
        let imm9 = (insn >> 12) & 0x1FF;
        (true, 0x3840_0000 | size << 30 | imm9 << 12 | rn << 5 | rt)
    } else if (insn & 0x3FFF_FC00) == 0x089F_FC00 {
        // stlr -> stur rt, [rn, #0]
        (false, 0x3800_0000 | size << 30 | rn << 5 | rt)
    } else if (insn & 0x3FE0_0C00) == 0x1900_0000 {
        // stlur -> stur rt, [rn, #imm9]
        let imm9 = (insn >> 12) & 0x1FF;
        (false, 0x3800_0000 | size << 30 | imm9 << 12 | rn << 5 | rt)
    } else {
        return false;
    };
    let isl = r.slab_cur.fetch_add(16, Ordering::AcqRel);
    let slab_end = r.slab_end.load(Ordering::Relaxed);
    if isl.saturating_add(16) > slab_end || !b_reach(pc, isl) || !b_reach(isl + 8, pc + 4) {
        return false;
    }
    let b_back = 0x1400_0000 | (((((pc + 4) as i64 - (isl + 8) as i64) / 4) as u32) & 0x03FF_FFFF);
    let b_to_isl = 0x1400_0000 | ((((isl as i64 - pc as i64) / 4) as u32) & 0x03FF_FFFF);
    // `pc` is inside an executing `MAP_JIT` region so JIT write-protect is currently 1 (RX).
    jit_write_protect(false);
    let p = isl as *mut u32;
    if is_load {
        *p.add(0) = plain_word;
        *p.add(1) = 0xD503_39BF; // dmb ishld
    } else {
        *p.add(0) = 0xD503_3ABF; // dmb ishst
        *p.add(1) = plain_word;
    }
    *p.add(2) = b_back;
    *p.add(3) = 0xD503_201F; // nop
    // Flush the island's icache BEFORE exposing the branch at `pc`.
    icache_invalidate(isl as *mut u8, 16);
    if (*(pc as *const AtomicU32)).compare_exchange(insn, b_to_isl, Ordering::Release, Ordering::Relaxed).is_ok() {
        icache_invalidate(pc as *mut u8, 4);
    }
    jit_write_protect(true);
    true
}
/// Upper bound on one block's code (incl. out-of-line stubs).
const MAX_BLOCK_BYTES: usize = 256 << 10;
const STUBS_BYTES: usize = 4096;
const JBITS: u32 = 18;
const JLEN: usize = 1 << JBITS;
pub(super) const JMUL: u64 = 0x9E37_79B9_7F4A_7C15;

/// Translation index entry (layout read by the `lookup_*` stubs).
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct JEnt {
    ip: u64,
    /// Code usable from translated code (0 if the IP is host-callable).
    fast: u64,
    /// 0 for image code; otherwise the code generation the block was validated in.
    gen: u32,
    /// Visit counter before translation; [`NOJIT`] if not translatable.
    hits: u32,
    /// Code entry for the interpreter (0 = not translated).
    code: u64,
}
const _: () = assert!(core::mem::size_of::<JEnt>() == 32);
const NOJIT: u32 = u32::MAX;

#[derive(Clone, Copy, Default)]
struct JMeta {
    hash: u64,
    len: u32,
}

#[inline(always)]
fn jslot(ip: u64) -> usize {
    (ip.wrapping_mul(JMUL) >> (64 - JBITS)) as usize
}

/// Per-thread translation cache.
pub(super) struct JitCache {
    mem: *mut u8,
    disabled: bool,
    /// True when executing signed static AOT modules from `__TEXT` with zero `MAP_JIT`
    /// allocation (`self.mem == null`, heap `index` + `ras_ring`, signed `self.enter`).
    static_only: bool,
    /// True when dynamic `MAP_JIT` is unavailable on this host/policy, preventing
    /// repeated `init()` calls while waiting for a static AOT module to bind.
    no_jit: bool,
    /// Last [`super::exec_mem::static_aot_gen`] observed by this cache (0 = uninitialized).
    static_aot_gen: u32,
    #[cfg(test)]
    test_forced_static: bool,
    cursor: usize,
    enter: usize,
    exit: usize,
    lookup_rip: usize,
    lookup_jmp: usize,
    lookup_call: usize,
    /// Host call/jmp stub ([`super::jit_host_xfer`]); 0 when `AW2_JIT_HOSTCALL=0`.
    host_xfer: usize,
    /// Shared XMM spill/reload stubs (0 when `PORT_JIT_XMM_PIN=0`) and `clear_ymm_hi` stub.
    spill_xmm: usize,
    reload_xmm: usize,
    clear_ymm_hi: usize,
    index: Box<[JEnt]>,
    meta: Box<[JMeta]>,
    /// All blocks compiled since the last flush, so direct-mapped `index` collisions can
    /// swap a previously compiled block back into its slot without re-translating.
    compiled: std::collections::HashMap<u64, (JEnt, JMeta)>,
    /// Unpatched direct-link sites, by target IP.
    links: std::collections::HashMap<u64, Vec<usize>>,
    /// Branch islands (`ldr x16, .+8; br x16; .quad target`) inside `self.mem` for far
    /// `JIT -> AOT` direct links outside ±128 MiB `b`/`bl` range.
    far_islands: std::collections::HashMap<usize, usize>,
    va_gen: u32,
    hot: u32,
    translated: u64,
    flushes: u64,
    /// Shadow return-address stack ([`ras_enabled`]): [`RAS_LEN`] `(guest return address,
    /// host landing pad)` entries, `RAS_BYTES`-aligned so the top pointer wraps with a
    /// 16-bit `bfxil`; null when off. Cleared to `(0, lookup_rip)` (always correct: the
    /// probe handles any target) on every flush, so no entry outlives its landing pad.
    ras_ring: *mut [u64; 2],
    /// Top entry address while no translated code of this thread is running
    /// ([`JitScratch::ras`] / [`JitScratch::ctx_ras_cell`]).
    ras_top: u64,
    /// `ras_miss` stub (0 when the RAS is off).
    ras_miss: usize,
    /// Wall time spent in [`Self::translate`] on this thread (ns).
    compile_ns: u64,
    /// [`super::aot::epoch`] this cache was last flushed for (AOT module kills).
    aot_epoch: u32,
    /// Slot index in [`LSE2_REGIONS`] for this cache's unaligned-LSE2 patch slab.
    lse2_slot: Option<usize>,
}

/// Shadow return-address stack entries (16 bytes each; the ring is 64 KiB).
const RAS_LEN: usize = 4096;
const RAS_BYTES: usize = RAS_LEN * 16;
const _: () = assert!(RAS_BYTES == 1 << 16);

impl JitCache {
    pub(super) fn new() -> Self {
        Self {
            mem: core::ptr::null_mut(),
            disabled: false,
            static_only: false,
            no_jit: false,
            static_aot_gen: 0,
            #[cfg(test)]
            test_forced_static: false,
            cursor: 0,
            enter: 0,
            exit: 0,
            lookup_rip: 0,
            lookup_jmp: 0,
            lookup_call: 0,
            host_xfer: 0,
            spill_xmm: 0,
            reload_xmm: 0,
            clear_ymm_hi: 0,
            index: Box::new([]),
            meta: Box::new([]),
            compiled: std::collections::HashMap::new(),
            links: std::collections::HashMap::new(),
            far_islands: std::collections::HashMap::new(),
            va_gen: 0,
            hot: 2,
            translated: 0,
            flushes: 0,
            ras_ring: core::ptr::null_mut(),
            ras_top: 0,
            ras_miss: 0,
            compile_ns: 0,
            aot_epoch: 0,
            lse2_slot: None,
        }
    }

    /// Compact mode tag for [`BlockCache`] fast-chain epoch invalidation.
    #[inline(always)]
    pub(super) fn mode_tag(&self) -> u8 {
        (u8::from(self.disabled))
            | (u8::from(self.static_only) << 1)
            | (u8::from(self.no_jit) << 2)
            | (u8::from(!self.mem.is_null()) << 3)
    }

    /// True when guest `ip` is provably sticky-interpreted under the current mode and
    /// static-AOT generation (safe to chain directly in `interp_burst` without re-entering
    /// [`Self::lookup`]).
    #[inline(always)]
    pub(super) fn is_interp_sticky(&self, ip: u64) -> bool {
        if self.disabled {
            return true;
        }
        if super::exec_mem::static_aot_gen() != self.static_aot_gen {
            return false;
        }
        if self.no_jit && !self.static_only {
            return true;
        }
        if self.index.is_empty() {
            return false;
        }
        let e = unsafe { self.index.get_unchecked(jslot(ip)) };
        e.ip == ip && e.code == 0 && e.hits == NOJIT
    }

    /// Force zero-`MAP_JIT` `static_only` mode on this cache (unit tests).
    #[cfg(test)]
    pub(super) fn test_set_static_only(&mut self, v: bool) {
        self.test_forced_static = v;
        self.static_only = v;
        self.no_jit = v;
        self.disabled = false;
        if v {
            self.mem = core::ptr::null_mut();
            self.meta = Box::new([]);
            if self.index.is_empty() {
                self.index = unsafe { super::zeroed_slice::<JEnt>(JLEN) };
            } else {
                self.index.fill(JEnt::default());
            }
            // Read generation BEFORE reading static stubs so concurrent binds cannot be missed.
            self.static_aot_gen = super::exec_mem::static_aot_gen();
            if let Some((enter, lookup_rip, has_ras)) = super::exec_mem::static_aot_enter_stub() {
                self.enter = enter;
                self.lookup_rip = lookup_rip;
                if has_ras && self.ras_ring.is_null() {
                    let ring = unsafe {
                        std::alloc::alloc(
                            std::alloc::Layout::from_size_align(RAS_BYTES, RAS_BYTES).unwrap(),
                        ) as *mut [u64; 2]
                    };
                    self.ras_ring = ring;
                    self.ras_top = ring as u64;
                }
                self.ras_clear();
            } else {
                self.enter = 0;
                self.lookup_rip = 0;
            }
        } else {
            self.static_aot_gen = 0;
        }
    }

    /// Set the translator up now (code region + stubs) if it never was on this thread: forking
    /// harnesses call it before the forks so no child maps / writes a fresh MAP_JIT region.
    pub(super) unsafe fn warm(&mut self) {
        if self.mem.is_null() && !self.static_only && !self.no_jit && !self.disabled {
            self.init();
        }
    }

    #[cold]
    unsafe fn init_static_aot(&mut self) -> bool {
        // Read `static_aot_gen` FIRST (Acquire) so we never cache a newer gen with stale stubs.
        self.static_aot_gen = super::exec_mem::static_aot_gen();
        let Some((enter, lookup_rip, has_ras)) = super::exec_mem::static_aot_enter_stub() else {
            return false;
        };
        if self.index.is_empty() {
            self.index = super::zeroed_slice::<JEnt>(JLEN);
        } else {
            self.index.fill(JEnt::default());
        }
        // Static AOT blocks live in immutable signed `__TEXT` (`gen == 0`) and never need
        // dynamic-code revalidation metadata; skip allocating the 4 MiB `self.meta` slab.
        self.va_gen = crate::exe::va_gen();
        self.aot_epoch = super::aot::epoch();
        self.enter = enter;
        self.lookup_rip = lookup_rip;
        self.static_only = true;
        self.no_jit = true;
        if has_ras {
            if self.ras_ring.is_null() {
                let ring = std::alloc::alloc(
                    std::alloc::Layout::from_size_align(RAS_BYTES, RAS_BYTES).unwrap(),
                ) as *mut [u64; 2];
                self.ras_ring = ring;
            }
            self.ras_top = self.ras_ring as u64;
            self.ras_clear();
        }
        log::info!(
            "emu jit: initialized zero-MAP_JIT static-AOT dispatch on thread (enter={enter:#x}, lookup_rip={lookup_rip:#x}, ras={has_ras})"
        );
        true
    }

    /// Synchronize this cache after initial startup or any static AOT module bind/unbind.
    #[cold]
    #[inline(never)]
    unsafe fn sync_static_aot(&mut self) {
        if self.mem.is_null() && !self.static_only && !self.no_jit {
            let _ = self.init();
            return;
        }
        // Read generation BEFORE reading static stubs/table.
        self.static_aot_gen = super::exec_mem::static_aot_gen();
        if !self.mem.is_null() {
            self.flush();
            return;
        }
        if let Some((enter, lookup_rip, has_ras)) = super::exec_mem::static_aot_enter_stub() {
            if self.index.is_empty() {
                self.index = super::zeroed_slice::<JEnt>(JLEN);
            } else {
                self.index.fill(JEnt::default());
            }
            self.va_gen = crate::exe::va_gen();
            self.aot_epoch = super::aot::epoch();
            self.enter = enter;
            self.lookup_rip = lookup_rip;
            self.static_only = true;
            if has_ras {
                if self.ras_ring.is_null() {
                    let ring = std::alloc::alloc(
                        std::alloc::Layout::from_size_align(RAS_BYTES, RAS_BYTES).unwrap(),
                    ) as *mut [u64; 2];
                    self.ras_ring = ring;
                }
                self.ras_top = self.ras_ring as u64;
                self.ras_clear();
            }
            self.flushes += 1;
        } else {
            if !self.index.is_empty() {
                self.index.fill(JEnt::default());
            }
            self.enter = 0;
            self.lookup_rip = 0;
            self.ras_clear();
            #[cfg(test)]
            if !self.test_forced_static {
                self.static_only = false;
            }
            #[cfg(not(test))]
            {
                self.static_only = false;
            }
            self.flushes += 1;
        }
    }

    #[cold]
    unsafe fn init(&mut self) -> bool {
        if !jit_writable() {
            self.no_jit = true;
            if super::exec_mem::exec_mode() == super::exec_mem::ExecMode::InterpreterOnly {
                self.disabled = true;
                self.static_aot_gen = super::exec_mem::static_aot_gen();
                return false;
            }
            return self.init_static_aot();
        }
        let Some(p) = alloc_jit(CODE_SIZE) else {
            self.no_jit = true;
            if self.init_static_aot() {
                return true;
            }
            log::error!("emu jit: mmap(MAP_JIT) failed; translator disabled on this thread");
            self.disabled = true;
            self.static_aot_gen = super::exec_mem::static_aot_gen();
            return false;
        };
        self.static_aot_gen = super::exec_mem::static_aot_gen();
        self.mem = p;
        self.index = super::zeroed_slice::<JEnt>(JLEN);
        self.meta = super::zeroed_slice::<JMeta>(JLEN);
        self.va_gen = crate::exe::va_gen();
        self.aot_epoch = super::aot::epoch();
        self.hot = hot_threshold();
        let ring = if ras_enabled() {
            // Leaked with the code buffer (thread-lifetime, like the translations).
            std::alloc::alloc(std::alloc::Layout::from_size_align(RAS_BYTES, RAS_BYTES).unwrap()) as *mut [u64; 2]
        } else {
            core::ptr::null_mut()
        };
        jit_write_protect(false);
        let mut a = Asm::new(p as *mut u32, p as usize, STUBS_BYTES / 4);
        let st = emit_stubs(&mut a, !ring.is_null());
        assert!(!a.overflow, "emu jit: shared stubs exceed {STUBS_BYTES} bytes");
        let (enter, exit, lr, lj, lc) = (st.enter, st.exit, st.lookup_rip, st.lookup_jmp, st.lookup_call);
        jit_write_protect(true);
        icache_invalidate(p, STUBS_BYTES);
        let b = p as usize;
        self.lse2_slot = register_lse2_region(b + STUBS_BYTES, b + CODE_LIMIT, b + CODE_LIMIT, b + CODE_SIZE);
        self.enter = b + enter * 4;
        self.exit = b + exit * 4;
        self.lookup_rip = b + lr * 4;
        self.lookup_jmp = b + lj * 4;
        self.lookup_call = b + lc * 4;
        self.host_xfer = st.host_xfer.map_or(0, |p| b + p * 4);
        self.spill_xmm = st.spill_xmm.map_or(0, |p| b + p * 4);
        self.reload_xmm = st.reload_xmm.map_or(0, |p| b + p * 4);
        self.clear_ymm_hi = b + st.clear_ymm_hi * 4;
        self.cursor = STUBS_BYTES;
        if let Some(m) = st.ras_miss {
            self.ras_ring = ring;
            self.ras_top = ring as u64;
            self.ras_miss = b + m * 4;
            self.ras_clear();
        }
        true
    }

    /// Reset every RAS entry to `(0, lookup_rip)`: a hit then just probes the target.
    fn ras_clear(&mut self) {
        if self.ras_ring.is_null() {
            return;
        }
        let ring = unsafe { core::slice::from_raw_parts_mut(self.ras_ring, RAS_LEN) };
        ring.fill([0, self.lookup_rip as u64]);
    }

    fn flush(&mut self) {
        if self.static_only {
            self.index.fill(JEnt::default());
            self.ras_clear();
            self.flushes += 1;
            return;
        }
        self.cursor = STUBS_BYTES;
        self.index.fill(JEnt::default());
        self.compiled.clear();
        self.links.clear();
        self.far_islands.clear();
        if let Some(slot) = self.lse2_slot {
            reset_lse2_slab(slot, self.mem as usize + CODE_LIMIT);
        }
        // Landing pads die with the code: no RAS entry may point into the old code.
        self.ras_clear();
        self.flushes += 1;
        if self.flushes <= 8 || self.flushes.is_power_of_two() {
            log::info!("emu jit: flushed translation cache (#{}, {} blocks translated so far)", self.flushes, self.translated);
        }
    }

    /// Code to run for guest `ip`, translating the block if it became hot; `None` to
    /// interpret.
    #[inline(always)]
    pub(super) unsafe fn lookup(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        if self.disabled {
            return None;
        }
        if crate::diag::jit_filter_active() && !crate::diag::jit_allows(ip) {
            return None;
        }
        if super::exec_mem::static_aot_gen() != self.static_aot_gen {
            self.sync_static_aot();
        }
        if self.mem.is_null() && !self.static_only {
            return None;
        }
        let s = jslot(ip);
        let e = self.index.get_unchecked_mut(s);
        if e.ip == ip {
            if e.code != 0 {
                if e.gen == 0 || e.gen == mem::code_gen() {
                    return Some(e.code as usize);
                }
                return self.revalidate(s, bc);
            }
            if self.static_only || e.hits == NOJIT {
                return None;
            }
            e.hits += 1;
            if e.hits < self.hot {
                return None;
            }
            self.translate(ip, bc)
        } else {
            self.miss(s, ip, bc)
        }
    }

    #[cold]
    #[inline(never)]
    unsafe fn miss(&mut self, s: usize, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        if self.static_only {
            if let Some((code, host)) = super::exec_mem::lookup_static_aot(ip) {
                let ent = JEnt { ip, fast: if host { 0 } else { code as u64 }, gen: 0, hits: 0, code: code as u64 };
                self.index[s] = ent;
                return Some(code);
            }
            self.index[s] = JEnt { ip, fast: 0, gen: 0, hits: NOJIT, code: 0 };
            return None;
        }
        if let Some(&(mut ent, meta)) = self.compiled.get(&ip) {
            if ent.code == 0 {
                return None;
            }
            if ent.gen != 0 {
                let gen = mem::code_gen();
                if ent.gen != gen {
                    if mem::code_hash(to_mapped_addr(ip), meta.len as usize) != meta.hash {
                        self.compiled.remove(&ip);
                        self.index[s] = JEnt { ip, ..JEnt::default() };
                        return self.translate(ip, bc);
                    }
                    ent.gen = gen;
                    self.compiled.insert(ip, (ent, meta));
                }
            }
            self.index[s] = ent;
            self.meta[s] = meta;
            return Some(ent.code as usize);
        }
        if let Some(code) = self.install_aot(s, ip) {
            return Some(code);
        }
        let e = &mut self.index[s];
        if e.code != 0 && self.hot > 1 {
            // Occupied by another compiled block: count challenger visits in `hits` before
            // evicting the resident entry.
            e.hits = e.hits.saturating_add(1);
            if e.hits < self.hot {
                return None;
            }
        } else {
            *e = JEnt { ip, hits: 1, ..JEnt::default() };
            if self.hot > 1 {
                return None;
            }
        }
        self.translate(ip, bc)
    }

    #[cold]
    #[inline(never)]
    unsafe fn revalidate(&mut self, s: usize, bc: *mut BlockCache) -> Option<usize> {
        let e = self.index[s];
        let m = self.meta[s];
        let gen = mem::code_gen();
        if mem::code_hash(to_mapped_addr(e.ip), m.len as usize) == m.hash {
            self.index[s].gen = gen;
            self.compiled.insert(e.ip, (self.index[s], m));
            return Some(e.code as usize);
        }
        self.compiled.remove(&e.ip);
        self.index[s] = JEnt { ip: e.ip, ..JEnt::default() };
        self.translate(e.ip, bc)
    }

    /// Run translated code at `code` with an instruction budget; returns the exit code and
    /// the number of guest instructions executed.
    #[inline(always)]
    pub(super) unsafe fn run(&mut self, cpu: &mut Cpu, code: usize, budget: i64) -> (u64, i64) {
        let vg = crate::exe::va_gen();
        let ep = super::aot::epoch();
        let sgen = super::exec_mem::static_aot_gen();
        if vg != self.va_gen || ep != self.aot_epoch || sgen != self.static_aot_gen {
            // Host dispatch changed: translations may embed stale host-callable decisions
            // (or an AOT/static-AOT module this cache may link into changed).
            let vg_changed = vg != self.va_gen;
            self.va_gen = vg;
            self.aot_epoch = ep;
            if sgen != self.static_aot_gen {
                self.sync_static_aot();
            } else {
                self.flush();
            }
            if vg_changed {
                super::aot::revalidate(vg);
            }
            return (EXIT_RIP, 0);
        }
        if self.enter == 0 {
            return (EXIT_RIP, 0);
        }
        #[cfg(test)]
        let budget = test_budget_cap(budget);
        cpu.jx.budget = budget;
        cpu.jx.ctx_vagen = self.va_gen as u64;
        cpu.jx.table = self.index.as_ptr() as usize;
        cpu.jx.mul = JMUL;
        cpu.jx.codegen = mem::code_gen_ptr();
        cpu.jx.aot_dir = if self.static_only {
            super::exec_mem::static_probe_table()
        } else if super::aot::active() {
            super::aot::probe_table()
        } else {
            0
        };
        let mut yd = 0u16;
        for i in 0..16 {
            if cpu.ymm_hi[i] != 0 {
                yd |= 1 << i;
            }
        }
        cpu.jx.ymm_dirty = yd;
        if !self.ras_ring.is_null() {
            cpu.jx.ras = self.ras_top;
            cpu.jx.ctx_ras_cell = &mut self.ras_top as *mut u64 as usize;
        }
        let f: unsafe extern "C" fn(*mut Cpu, usize) -> u64 = core::mem::transmute(self.enter);
        let r = f(cpu as *mut Cpu, code);
        if !self.ras_ring.is_null() {
            #[cfg(test)]
            if cpu.jx.ras != self.ras_top {
                TEST_RAS_MOVES.with(|c| c.set(c.get() + 1));
            }
            self.ras_top = cpu.jx.ras;
        }
        #[cfg(test)]
        TEST_EXECUTED.with(|c| c.set(c.get() + (budget - cpu.jx.budget) as u64));
        let ph = core::mem::take(&mut cpu.jx.aot_probe_hits);
        if ph != 0 {
            note_probe_hits(ph);
        }
        #[cfg(test)]
        {
            let pl = core::mem::take(&mut cpu.jx.plain_call_hits);
            if pl != 0 {
                TEST_STUB_PLAIN_HITS.with(|c| c.set(c.get() + pl));
                super::note_test_xfer_plain(pl);
            }
        }
        (r, budget - cpu.jx.budget)
    }

    /// Offline differential verifier helper: compile a single guest block at `ip` into this
    /// thread's JIT buffer and leave `self.index[jslot(ip)]` empty so running it via
    /// [`Self::run_code_for_verify`] executes strictly one block (even for self-loops).
    pub(super) unsafe fn compile_single_for_verify(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        if self.enter == 0 {
            self.init();
            if self.enter == 0 {
                return None;
            }
        }
        // Keep the index empty so no unlinked target (including self-loops) chains implicitly.
        self.compiled.clear();
        self.links.clear();
        let s = jslot(ip);
        let code = self.translate_jit(ip, bc)?;
        self.index[s] = JEnt::default();
        self.compiled.clear();
        self.links.clear();
        Some(code)
    }

    /// Offline differential verifier helper: execute translated ARM64 machine code at `code`
    /// using `enter` (either `self.enter` for live JIT or an AOT bank's `enter` stub) with an
    /// empty lookup table and `aot_dir = 0`.
    pub(super) unsafe fn run_code_for_verify(
        &mut self,
        cpu: &mut Cpu,
        bc: *mut BlockCache,
        enter_override: Option<usize>,
        code: usize,
        budget: i64,
    ) -> (u64, i64) {
        if self.enter == 0 {
            self.init();
            if self.enter == 0 {
                return (EXIT_RIP, 0);
            }
        }
        let mut dummy_sp = cpu.gpr[4];
        let enter = enter_override.unwrap_or(self.enter);
        cpu.jx.budget = budget;
        cpu.jx.ctx_vagen = self.va_gen as u64;
        cpu.jx.ctx_blocks = bc as usize;
        cpu.jx.ctx_sp = &mut dummy_sp as *mut u64 as usize;
        cpu.jx.ctx_prof = 0;
        cpu.jx.ctx_entry = 0;
        cpu.jx.table = self.index.as_ptr() as usize;
        cpu.jx.mul = JMUL;
        cpu.jx.codegen = mem::code_gen_ptr();
        cpu.jx.aot_dir = 0;
        let mut yd = 0u16;
        for i in 0..16 {
            if cpu.ymm_hi[i] != 0 {
                yd |= 1 << i;
            }
        }
        cpu.jx.ymm_dirty = yd;
        if !self.ras_ring.is_null() {
            cpu.jx.ras = self.ras_top;
            cpu.jx.ctx_ras_cell = &mut self.ras_top as *mut u64 as usize;
        }
        let f: unsafe extern "C" fn(*mut Cpu, usize) -> u64 = core::mem::transmute(enter);
        let r = f(cpu as *mut Cpu, code);
        if !self.ras_ring.is_null() {
            self.ras_top = cpu.jx.ras;
        }
        (r, budget - cpu.jx.budget)
    }

    /// Return a branch island inside `self.mem` (`ldr x16, .+8; br x16; .quad target`) for
    /// a far target outside ±128 MiB `b`/`bl` range. Caller must hold write-protect off.
    unsafe fn far_island(&mut self, target: usize) -> Option<usize> {
        if let Some(&addr) = self.far_islands.get(&target) {
            return Some(addr);
        }
        if self.cursor + 16 > CODE_LIMIT {
            return None;
        }
        let addr = self.mem as usize + self.cursor;
        let p = addr as *mut u32;
        *p.add(0) = 0x5800_0050; // ldr x16, .+8
        *p.add(1) = 0xD61F_0200; // br x16
        core::ptr::write_unaligned(p.add(2) as *mut u64, target as u64);
        self.cursor += 16;
        icache_invalidate(addr as *mut u8, 16);
        self.far_islands.insert(target, addr);
        Some(addr)
    }

    /// Install a cached `DF_PLAIN` or pure-float-math host function pointer into the running
    /// thread's direct-mapped `table`:
    /// - bit 0 = 1, bit 1 = 0, bit 63 = 0: pure-integer `DF_PLAIN` (`(host_fn as u64) | 1`)
    /// - bit 0 = 1, bit 1 = 1, bit 63 = 0: pure-float-math (`(host_fn as u64) | 3`)
    ///
    /// Unconditionally installs with `code = 0, hits = NOJIT` so `JitCache::lookup` never
    /// mistakes it for translated guest code while any evicted guest block recovers in O(1)
    /// via `aot_probe` / `self.compiled`.
    #[inline(always)]
    pub(super) unsafe fn cache_plain_host(table: usize, target: u64, host_fn: usize, needs_xmm: bool) {
        let addr = host_fn as u64;
        if table == 0 || (addr & ((1u64 << 63) | 3)) != 0 {
            return;
        }
        let tag = if needs_xmm { 3u64 } else { 1u64 };
        let s = jslot(target);
        let cur = &mut *(table as *mut JEnt).add(s);
        *cur = JEnt { ip: target, fast: addr | tag, gen: 0, hits: NOJIT, code: 0 };
    }

    /// Install an inline zero-spill `QueryPerformanceCounter` / `RtlQueryPerformanceCounter`
    /// entry into the running thread's direct-mapped `table` (`fast = qpc_state | (1 << 63) | 1`,
    /// `code = 0`, `hits = NOJIT`).
    #[inline(always)]
    pub(super) unsafe fn cache_qpc_host(table: usize, target: u64, qpc_state: usize) {
        let addr = qpc_state as u64;
        if table == 0 || (addr & ((1u64 << 63) | 3)) != 0 {
            return;
        }
        let s = jslot(target);
        let cur = &mut *(table as *mut JEnt).add(s);
        *cur = JEnt { ip: target, fast: addr | (1u64 << 63) | 1, gen: 0, hits: NOJIT, code: 0 };
    }

    #[cfg(test)]
    pub(super) fn test_table_ptr(&self) -> usize {
        if self.index.is_empty() { 0 } else { self.index.as_ptr() as usize }
    }

    #[cfg(test)]
    pub(super) fn test_slot_entry(&self, target: u64) -> Option<(u64, u64, u32, u32, u64)> {
        if self.index.is_empty() {
            return None;
        }
        let e = self.index[jslot(target)];
        Some((e.ip, e.fast, e.gen, e.hits, e.code))
    }

    #[cfg(test)]
    pub(super) fn test_seed_guest_slot(&mut self, ip: u64, code: u64) {
        if self.index.is_empty() {
            self.index = unsafe { super::zeroed_slice::<JEnt>(JLEN) };
        }
        self.index[jslot(ip)] = JEnt { ip, fast: code, gen: 0, hits: 0, code };
    }

    fn linkable(&self, target: u64) -> Option<usize> {
        if crate::diag::jit_filter_active() && !crate::diag::jit_allows(target) {
            return None;
        }
        if crate::diag::nan_trace_installed() && crate::diag::nan_trace_covers(target) {
            return None;
        }
        let e = &self.index[jslot(target)];
        if e.ip == target && e.code != 0 && e.fast != 0 && e.gen == 0 {
            return Some(e.code as usize);
        }
        if let Some(&(e, _)) = self.compiled.get(&target) {
            if e.code != 0 && e.fast != 0 && e.gen == 0 {
                return Some(e.code as usize);
            }
        }
        None
    }

    /// Install the shared AOT code for `ip` (if any) in index slot `s`, linking this cache's
    /// direct-link sites waiting for it (when within `b` range).
    unsafe fn install_aot(&mut self, s: usize, ip: u64) -> Option<usize> {
        let (code, host) = super::aot::find(ip)?;
        let ent = JEnt { ip, fast: if host { 0 } else { code as u64 }, gen: 0, hits: 0, code: code as u64 };
        self.index[s] = ent;
        self.meta[s] = JMeta::default();
        if !host {
            if let Some(sites) = self.links.remove(&ip) {
                let mut unlinked = Vec::new();
                let mut patched = Vec::new();
                jit_write_protect(false);
                for site in sites {
                    if b_reach(site, code) {
                        write_b(site, code);
                        patched.push(site);
                    } else if let Some(isl) = self.far_island(code) {
                        write_b(site, isl);
                        patched.push(site);
                    } else {
                        unlinked.push(site);
                    }
                }
                jit_write_protect(true);
                for site in patched {
                    icache_invalidate(site as *mut u8, 4);
                }
                if !unlinked.is_empty() {
                    if super::jit_stats::enabled() {
                        super::jit_stats::aot_event(super::jit_stats::TW_FAR, unlinked.len() as u64);
                    }
                    self.links.insert(ip, unlinked);
                }
            }
        }
        if super::jit_stats::enabled() {
            super::jit_stats::aot_event(super::jit_stats::TW_AOT_INSTALL, 1);
        }
        #[cfg(test)]
        TEST_AOT_INSTALLS.with(|c| c.set(c.get() + 1));
        Some(code)
    }

    /// Stub addresses translated blocks of this cache branch to.
    fn stub_addrs(&self) -> StubAddrs {
        StubAddrs {
            exit: self.exit,
            lookup_rip: self.lookup_rip,
            lookup_jmp: self.lookup_jmp,
            lookup_call: self.lookup_call,
            host_xfer: self.host_xfer,
            ras_miss: self.ras_miss,
            spill_xmm: self.spill_xmm,
            reload_xmm: self.reload_xmm,
            clear_ymm_hi: self.clear_ymm_hi,
        }
    }

    /// Translate the block at `ip` (or install its shared AOT code); timed for the
    /// per-thread compile-time diagnostics.
    #[cold]
    #[inline(never)]
    unsafe fn translate(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        if let Some(code) = self.install_aot(jslot(ip), ip) {
            return Some(code);
        }
        let t0 = std::time::Instant::now();
        let r = self.translate_jit(ip, bc);
        let ns = t0.elapsed().as_nanos() as u64;
        self.compile_ns += ns;
        if super::jit_stats::enabled() {
            super::jit_stats::compile(ns, r.is_some());
            if r.is_some() && super::aot::active() {
                super::jit_stats::aot_event(super::jit_stats::TW_AOT_MISS, 1);
            }
        }
        r
    }

    unsafe fn translate_jit(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        let s = jslot(ip);
        let Some(p) = prep_block(bc, ip) else {
            let ent = JEnt { ip, hits: NOJIT, ..JEnt::default() };
            self.index[s] = ent;
            self.compiled.insert(ip, (ent, JMeta::default()));
            return None;
        };
        let mapped = to_mapped_addr(ip);
        let gen = if mem::in_jit(mapped) { mem::code_gen().max(1) } else { 0 };
        if self.cursor + MAX_BLOCK_BYTES > CODE_LIMIT {
            self.flush();
        }
        let (k, host) = (p.k, p.host);
        let ops = &p.ops[..k];

        jit_write_protect(false);
        let start = self.cursor;
        let base = self.mem.add(start) as *mut u32;
        let st = self.stub_addrs();
        let mut tr = new_tr(Asm::new(base, base as usize, MAX_BLOCK_BYTES / 4), ops, &p, &st, false);
        let _main_words = tr.translate(p.term, p.live_out);
        let words = tr.a.pos;
        let overflow = tr.a.overflow;
        let links = core::mem::take(&mut tr.links);
        drop(tr);
        #[cfg(test)]
        if !overflow {
            TEST_DUMP.with(|d| {
                if let Some(v) = d.borrow_mut().as_mut() {
                    let w = core::slice::from_raw_parts(base as *const u32, words).to_vec();
                    v.push(DumpBlock { ip, guest: k, code: base as usize, words: w, main: _main_words });
                }
            });
        }
        if overflow {
            jit_write_protect(true);
            log::warn!("emu jit: block at {ip:#x} exceeded the per-block code limit; interpreting it");
            let ent = JEnt { ip, hits: NOJIT, ..JEnt::default() };
            self.index[s] = ent;
            self.compiled.insert(ip, (ent, JMeta::default()));
            return None;
        }
        let code = self.mem as usize + start;
        self.cursor = (start + words * 4 + 15) & !15;
        let mut patched: Vec<usize> = Vec::new();
        // This block's own direct links (far AOT targets go through a 16-byte branch island).
        let mut far_sites = 0u64;
        for (site_pos, target) in links {
            let site = code + site_pos * 4;
            match self.linkable(target) {
                Some(tc) if b_reach(site, tc) => {
                    write_b(site, tc);
                    patched.push(site);
                }
                Some(tc) => {
                    if let Some(isl) = self.far_island(tc) {
                        write_b(site, isl);
                        patched.push(site);
                    } else {
                        far_sites += 1;
                        self.links.entry(target).or_default().push(site);
                    }
                }
                None => {
                    self.links.entry(target).or_default().push(site);
                }
            }
        }
        if far_sites != 0 && super::jit_stats::enabled() {
            super::jit_stats::aot_event(super::jit_stats::TW_FAR, far_sites);
        }
        // Register, then patch links waiting for this block.
        let ent = JEnt { ip, fast: if host { 0 } else { code as u64 }, gen, hits: 0, code: code as u64 };
        let meta = if gen != 0 {
            let last = &ops[k - 1];
            let len = last.ip.wrapping_add(last.len as u64).wrapping_sub(ip) as usize;
            JMeta { hash: mem::code_hash(mapped, len), len: len as u32 }
        } else {
            if !host {
                if let Some(sites) = self.links.remove(&ip) {
                    for site in sites {
                        write_b(site, code);
                        patched.push(site);
                    }
                }
            }
            JMeta::default()
        };
        self.index[s] = ent;
        self.meta[s] = meta;
        self.compiled.insert(ip, (ent, meta));
        jit_write_protect(true);
        icache_invalidate(code as *mut u8, words * 4);
        for site in patched {
            icache_invalidate(site as *mut u8, 4);
        }
        if gen == 0 {
            super::aot::profile_note(ip);
        }
        self.translated += 1;
        if self.translated.is_power_of_two() && self.translated >= 1024 {
            log::info!(
                "emu jit: {} blocks translated on this thread ({} KiB code, {} flushes, {} ms compiling)",
                self.translated,
                (self.cursor - STUBS_BYTES) >> 10,
                self.flushes,
                self.compile_ns / 1_000_000
            );
        }
        Some(code)
    }
}

/// `b`/`bl` at `site` can reach `target` (±128 MiB).
#[inline]
fn b_reach(site: usize, target: usize) -> bool {
    let d = target as i64 - site as i64;
    (-(128i64 << 20)..(128i64 << 20)).contains(&d)
}

/// Absolute addresses of the stubs a translated block branches to.
#[derive(Clone, Copy)]
struct StubAddrs {
    exit: usize,
    lookup_rip: usize,
    lookup_jmp: usize,
    lookup_call: usize,
    host_xfer: usize,
    ras_miss: usize,
    spill_xmm: usize,
    reload_xmm: usize,
    clear_ymm_hi: usize,
}

/// Decoded, analysed block (everything [`Tr`] needs besides the stubs and the emission
/// mode): shared by the JIT and AOT builds so both translate a block identically.
pub(super) struct Prep {
    pub(super) ops: [FastOp; BLK_MAX_INSNS],
    /// Translated prefix length (supported ops).
    pub(super) k: usize,
    /// The block's own IP is host-callable (`JEnt::fast` = 0).
    pub(super) host: bool,
    wr: [u16; BLK_MAX_INSNS],
    pub(super) term: Term,
    /// See [`Tr::target_host`].
    target_host: bool,
    lazy: bool,
    pub(super) edge: Edges,
    pub(super) live_out: u8,
}

/// Decode and analyse the block at `ip`; None = not translatable (decode error, no
/// supported prefix, `__chkstk`).
pub(super) unsafe fn prep_block(bc: *mut BlockCache, ip: u64) -> Option<Prep> {
    let mut dummy = 0u64;
    let (off, n) = BlockCache::get(bc, ip, &mut dummy).ok()?;
    let mut ops = [FastOp::default(); BLK_MAX_INSNS];
    let n = n.min(BLK_MAX_INSNS);
    ops[..n].copy_from_slice(&(&(*bc).ops)[off..off + n]);
    let mut k = 0;
    while k < n && supported(&ops[k]) {
        k += 1;
    }
    if k == 0
        || is_chkstk(ip)
        || super::is_routed_dll_stack_pos(ip)
        || super::is_routed_regfile(ip)
        || super::is_routed_dll_stub(ip)
    {
        return None;
    }
    let host = resolved_host(bc, ip);
    // GPR write sets, read before `succ_need` may decode further blocks into the arena
    // (`arena[off + i]` is the decoded instruction of `ops[i]`).
    let mut wr = [0u16; BLK_MAX_INSNS];
    if memchk_mask() & 12 != 0 {
        let mut f = iced_x86::InstructionInfoFactory::new();
        for (i, w) in wr.iter_mut().enumerate().take(k) {
            *w = gpr_writes(&(&(*bc).arena)[off + i], &mut f);
        }
    }
    // Successor flag needs (may decode further blocks; `ops` is a private copy).
    let mut term = classify(&ops[..k]);
    let mut resolved = [false; 2];
    if let Term::Jcc { target, .. } | Term::Jmp(target) | Term::Call(target) = term {
        resolved[0] = target == 0 || is_chkstk(target) || resolved_host(bc, target);
    }
    // Fuse `ucomiss/comiss + jp/jnp t1; je/jne t2; fall2` (MSVC's standard FP == / != idiom)
    // into a single 3-way `Term::JccPair`, eliminating the [XCPU, #OFF_ZF] store/reload and
    // extra 1-instruction block between `jp` and `je`/`jne`.
    if budget_light_enabled() && xmmreg_enabled() && !resolved[0] {
        if let Term::Jcc { cc: cc1 @ (11 | 12), target: t1, fall: f1, .. } = term {
            let prev_is_comi = (0..k - 1).rev().find_map(|i| {
                let (_, kl) = op_flags(&ops[i]);
                if kl == 0 {
                    None
                } else {
                    Some(matches!(ops[i].kind, fast::K_COMI_RR | fast::K_COMI_RM))
                }
            });
            if prev_is_comi == Some(true) && !resolved_host(bc, f1) {
                let mut d2 = 0u64;
                if let Ok((off2, n2)) = BlockCache::get(bc, f1, &mut d2) {
                    if n2 >= 1 {
                        let op2 = (&(*bc).ops)[off2];
                        if op2.kind == fast::K_JCC {
                            let cc2 = op2.x & 0x3F;
                            let t2 = op2.imm;
                            let fall2 = op2.ip.wrapping_add(op2.len as u64);
                            if matches!(cc2, 5 | 6)
                                && t2 != 0
                                && !is_chkstk(t2)
                                && !resolved_host(bc, t2)
                            {
                                term = Term::JccPair { cc1, t1, cc2, t2, fall2 };
                            }
                        }
                    }
                }
            }
        }
    }
    let lazy = flaglazy_enabled();
    let depth = if lazy { FLAGLAZY_DEPTH } else { 1 };
    // Per-edge needs / last_bits kills: [taken or only successor, fall-through of a Jcc].
    let mut edge = Edges { live: [FALL; 2], kill: [0; 2] };
    let live_out = match term {
        Term::Fall(x) => {
            edge.live[0] = succ_need(bc, x, depth);
            edge.kill[0] = succ_lb_kill(bc, x, lazy);
            edge.live[0]
        }
        Term::Jcc { target, fall, near, .. } => {
            let t = if near && resolved_host(bc, target) { FALL } else { succ_need(bc, target, depth) };
            let f = succ_need(bc, fall, depth);
            edge.live = [t, f];
            edge.kill = [if resolved[0] { 0 } else { succ_lb_kill(bc, target, lazy) }, succ_lb_kill(bc, fall, lazy)];
            t | f
        }
        Term::JccPair { t1, t2, fall2, .. } => {
            let l1 = succ_need(bc, t1, depth);
            let l2 = succ_need(bc, t2, depth);
            let l3 = succ_need(bc, fall2, depth);
            l1 | l2 | l3
        }
        Term::Jmp(t) => {
            if is_chkstk(t) || resolved_host(bc, t) {
                FALL
            } else {
                edge.live[0] = succ_need(bc, t, depth);
                edge.kill[0] = if resolved[0] { 0 } else { succ_lb_kill(bc, t, lazy) };
                edge.live[0]
            }
        }
        Term::Call(t) => {
            edge.kill[0] = if resolved[0] { 0 } else { succ_lb_kill(bc, t, lazy) };
            if flags_dead_mask() & 1 != 0
                && !super::is_routed_dll_stack_pos(t)
                && !super::is_routed_regfile(t)
            {
                0
            } else {
                FALL
            }
        }
        Term::CallR | Term::CallM if flags_dead_mask() & 1 != 0 => 0,
        Term::Ret if flags_dead_mask() & 2 != 0 => 0,
        _ => FALL,
    };
    if !matches!(term, Term::Jcc { .. }) {
        edge.live = [live_out; 2];
    }
    Some(Prep { ops, k, host, wr, term, target_host: resolved[0], lazy, edge, live_out })
}

/// A fresh translator for `ops` (the prefix of `p.ops`) emitting into `a`; `aot`: AOT
/// emission mode (host-pointer literals always in the pool and recorded as relocations).
fn new_tr<'a>(a: Asm, ops: &'a [FastOp], p: &Prep, st: &StubAddrs, aot: bool) -> Tr<'a> {
    Tr {
        a,
        ops,
        live: [0; BLK_MAX_INSNS + 1],
        fs: FlagState { src: Src::None, host: 0, mem: FALL },
        lb: [LB_NONE; 16],
        side_exits: Vec::new(),
        slow: Vec::new(),
        links: Vec::new(),
        cond_links: Vec::new(),
        link_conds: Vec::new(),
        cold_code_end: 0,
        exit: st.exit,
        lookup_rip: st.lookup_rip,
        lookup_jmp: st.lookup_jmp,
        lookup_call: st.lookup_call,
        host_xfer: st.host_xfer,
        spill_xmm: st.spill_xmm,
        reload_xmm: st.reload_xmm,
        clear_ymm_hi: st.clear_ymm_hi,
        target_host: p.target_host,
        ras_miss: st.ras_miss,
        wr: p.wr,
        grp: [Grp::None; BLK_MAX_INSNS],
        grp_ok: 0,
        grp_k: usize::MAX,
        body: 0,
        skip: 0,
        rsp_al: false,
        xon: xmmreg_enabled(),
        xpin: xmm_pin_enabled(),
        ymm_clean: 0,
        ymm_colds: Vec::new(),
        xmm_hi32_zero: 0,
        xmm_hi64_zero: 0,
        xmm_hi32_live: [0xFFFF; BLK_MAX_INSNS + 1],
        xmm_hi64_live: [0xFFFF; BLK_MAX_INSNS + 1],
        xa_key: None,
        ret_preloaded: false,
        term_is_plain_ret: false,
        term_live_out: 0,
        bud_dec: true,
        fast_nan: fast_nan_enabled(),
        lb_on: last_bits_enabled(),
        xv: if xmm_pin_enabled() { 0xFFFF } else { 0 },
        xd: 0,
        cold: Vec::new(),
        retlit: retlit_enabled(),
        lits: Vec::new(),
        lits128: Vec::new(),
        lazy: p.lazy,
        edge: p.edge,
        pf: PF_NONE,
        aot,
        rlits: Vec::new(),
        relocs: Vec::new(),
    }
}

std::thread_local! {
    /// AOT builds: host-callable decisions consulted while translating the current block
    /// (`(va, host)`), recorded as the block's assumptions; None = not recording.
    static RESOLVE_REC: core::cell::RefCell<Option<Vec<(u64, bool)>>> = const { core::cell::RefCell::new(None) };
    /// Test-only: AOT entries installed into this thread's index.
    #[cfg(test)]
    static TEST_AOT_INSTALLS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test-only: `lookup_*` misses served by the shared AOT probe table on this thread.
    #[cfg(test)]
    static TEST_PROBE_HITS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// `n` stub lookups were served by the shared AOT probe table during one run.
#[cold]
#[inline(never)]
fn note_probe_hits(n: u64) {
    #[cfg(test)]
    TEST_PROBE_HITS.with(|c| c.set(c.get() + n));
    if super::jit_stats::enabled() {
        super::jit_stats::aot_event(super::jit_stats::TW_PROBE_HIT, n);
    }
}

/// `lookup_*` misses served by the shared AOT probe table on this thread (tests).
#[cfg(test)]
pub(super) fn test_probe_hits() -> u64 {
    TEST_PROBE_HITS.with(|c| c.get())
}

/// AOT entries installed into this thread's translation index (tests).
#[cfg(test)]
pub(super) fn test_aot_installs() -> u64 {
    TEST_AOT_INSTALLS.with(|c| c.get())
}

/// `BlockCache::resolve(bc, ip).is_some()`, recorded as an assumption while an AOT build
/// translates a block (every host-callable decision that shapes the code goes through here).
#[inline]
unsafe fn resolved_host(bc: *mut BlockCache, ip: u64) -> bool {
    let h = BlockCache::resolve(bc, ip).is_some()
        || super::is_routed_dll_chkstk(ip)
        || super::is_routed_dll_stack_pos(ip)
        || super::is_routed_regfile(ip)
        || super::is_routed_dll_stub(ip);
    RESOLVE_REC.with(|r| {
        if let Some(v) = r.borrow_mut().as_mut() {
            if !v.iter().any(|&(a, _)| a == ip) {
                v.push((ip, h));
            }
        }
    });
    h
}

/// Write `b target` (or `bl target` if the site holds a `bl`) at `site` (code memory must
/// be writable).
unsafe fn write_b(site: usize, target: usize) {
    let off = (target as i64 - site as i64) / 4;
    let link = *(site as *const u32) & 0x8000_0000;
    *(site as *mut u32) = 0x1400_0000 | link | (off as u32 & 0x03FF_FFFF);
}

/// Explicit stub emission configuration, shared by dynamic JIT/AOT (`StubCfg::live`)
/// and offline static Mach-O `.o` emission (`StubCfg::from_aot_knobs`).
#[derive(Clone, Copy)]
struct StubCfg {
    ras: bool,
    xpin: bool,
    lb_on: bool,
    bud_light: bool,
    aot_on: bool,
    bud: bool,
    pushpop: bool,
    stack_tso_active: bool,
    hx: bool,
    count_probe_hits: bool,
    #[cfg_attr(not(test), allow(dead_code))]
    count_plain_hits: bool,
    /// When true, emit a relocatable `bl 0` (`ARM64_RELOC_BRANCH26 -> _port_static_aot_host_xfer`)
    /// instead of a baked 64-bit host pointer, and pad before `ras_miss` to match `target_ras_miss`.
    static_pic: bool,
    target_ras_miss: Option<usize>,
}

impl StubCfg {
    fn live(ras: bool) -> Self {
        Self {
            ras,
            xpin: xmm_pin_enabled(),
            lb_on: last_bits_enabled(),
            bud_light: budget_light_enabled(),
            aot_on: aot_probe_enabled(),
            bud: budget_backedge_enabled(),
            pushpop: pushpop_enabled(),
            stack_tso_active: mem::tso() && stack_tso(),
            hx: hostcall_enabled(),
            count_probe_hits: cfg!(test) || super::jit_stats::enabled(),
            count_plain_hits: cfg!(test),
            static_pic: false,
            target_ras_miss: None,
        }
    }

    fn from_aot_knobs(knobs: u64, want_ras_miss: Option<usize>, count_probe_hits: bool, count_plain_hits: bool) -> Self {
        let bit = |b: u32| ((knobs >> b) & 1) != 0;
        Self {
            ras: bit(0),
            xpin: bit(30),
            lb_on: bit(32),
            bud_light: bit(34),
            aot_on: bit(27),
            bud: bit(9),
            pushpop: bit(8),
            stack_tso_active: bit(15) && bit(16),
            hx: bit(13),
            count_probe_hits,
            count_plain_hits,
            static_pic: true,
            target_ras_miss: want_ras_miss,
        }
    }
}

/// Shared stubs (never overwritten by translation-cache flushes). `ras`: emit the shadow
/// return-address stack push in `lookup_call` and the `ras_miss` stub.
fn emit_stubs(a: &mut Asm, ras: bool) -> Stubs {
    emit_stubs_cfg(a, StubCfg::live(ras)).0
}

fn emit_stubs_cfg(a: &mut Asm, cfg: StubCfg) -> (Stubs, Option<usize>) {
    let ras = cfg.ras;
    let xpin = cfg.xpin;
    let lb_on = cfg.lb_on;
    let bud_light = cfg.bud_light;
    // enter(cpu: x0, code: x1)
    let enter = a.pos;
    a.emit(0xA980_0000 | ((-12i32 as u32) & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // stp x29, x30, [sp, #-96]!
    a.emit(0x9100_03FD); // mov x29, sp
    a.stp_x(19, 20, 31, 16);
    a.stp_x(21, 22, 31, 32);
    a.stp_x(23, 24, 31, 48);
    a.stp_x(25, 26, 31, 64);
    a.stp_x(27, 28, 31, 80);
    a.mov_rr(true, XCPU, 0);
    for i in 0..8 {
        a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    if xpin {
        for i in 0..8u32 {
            a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
    }
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    // Pack `XBUD = (budget << 1) | (ymm_dirty != 0)` so VEX-128 / `vzeroupper` checks are a
    // single register-only `tbnz x8, #0` while `sub x8, #(2*n)` and `tbnz x8, #63` keep exact
    // budget semantics without touching bit 0.
    a.ldr(64, XBUD, XCPU, OFF_BUDGET);
    a.lsl_i(true, XBUD, XBUD, 1);
    a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
    a.addsub_ri(false, true, true, ZR, XS, 0, false); // cmp w17, #0
    a.cset(XS, NE);
    a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0); // orr x8, x8, x17
    a.br(1);

    // exit (w0 = exit code; rip / target already stored)
    let exit = a.pos;
    for i in 0..8 {
        a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    if xpin {
        for i in 0..8u32 {
            a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    a.asr_i(true, XA, XBUD, 1);
    a.str(64, XA, XCPU, OFF_BUDGET);
    // exit_ns: same, with the GPRs and budget already stored by the caller.
    let exit_ns = a.pos;
    a.ldp_x(19, 20, 31, 16);
    a.ldp_x(21, 22, 31, 32);
    a.ldp_x(23, 24, 31, 48);
    a.ldp_x(25, 26, 31, 64);
    a.ldp_x(27, 28, 31, 80);
    a.emit(0xA8C0_0000 | (12u32 & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // ldp x29, x30, [sp], #96
    a.ret();

    // Shared XMM spill/reload helpers (preserves all GPRs, q0..q15, and NZCV).
    let (spill_xmm, reload_xmm) = if xpin {
        let sp_pos = a.pos;
        for i in 0..8u32 {
            a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        a.ret();

        let rl_pos = a.pos;
        for i in 0..8u32 {
            a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        a.ret();
        (Some(sp_pos), Some(rl_pos))
    } else {
        (None, None)
    };

    // Shared VEX-128 / `vzeroupper` lazy `Cpu::ymm_hi` zeroing stub: clears `Cpu::ymm_hi[r]`
    // for every bit `r` set in BOTH `w16` (mask) and `Cpu::jx.ymm_dirty`, updates
    // `Cpu::jx.ymm_dirty`, and clears bit 0 of `XBUD` once `ymm_dirty` reaches 0.
    // Clobbers only x16, x17 (and bit 0 of x8); preserves all other GPRs and NZCV.
    let clear_ymm_hi = a.pos;
    a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
    a.logic_rr(false, 0, XA, XA, XS, 0, 0); // and w16, w16, w17 (preserves NZCV)
    let ym_done = a.cbz(false, XA);
    a.bic_rr(false, XS, XS, XA);
    a.str(16, XS, XCPU, OFF_YMM_DIRTY);
    let ym_keep_bit = a.cbnz(false, XS);
    a.logic_ri(true, 0, XBUD, XBUD, enc_logic_imm(!1u64, true).unwrap());
    a.bind(ym_keep_bit);
    let ym_loop = a.pos;
    a.rbit(false, XS, XA);
    a.clz(false, XS, XS);
    a.addsub_rr(true, false, false, XS, XCPU, XS, 4); // x17 = XCPU + r*16
    a.stp_x(ZR, ZR, XS, OFF_YMM_HI as i32);
    a.addsub_ri(false, true, false, XS, XA, 1, false);
    a.logic_rr(false, 0, XA, XA, XS, 0, 0); // and w16, w16, w17
    let ym_back = a.cbnz(false, XA);
    a.patch(ym_back, ym_loop);
    a.ret();
    a.bind(ym_done);
    let ym_ret = a.cbnz(false, XS);
    a.logic_ri(true, 0, XBUD, XBUD, enc_logic_imm(!1u64, true).unwrap());
    a.bind(ym_ret);
    a.ret();

    // Hash probe of x0 -> on hit branches to the code (x1); on miss falls through.
    // Clobbers x1, x2, x16, x17 and NZCV.
    let probe = |a: &mut Asm| -> [usize; 3] {
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JBITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 5);
        a.ldp_x(XS, 1, XA, 0);
        a.addsub_rr(true, true, true, ZR, XS, 0, 0); // cmp x17, x0
        let m1 = a.bcond(NE);
        let m2 = a.cbz(true, 1);
        a.ldr(32, XS, XA, 16);
        let ok = a.cbz(false, XS);
        a.ldr(64, 2, XCPU, OFF_CODEGEN);
        a.ldr(32, 2, 2, 0);
        a.addsub_rr(false, true, true, ZR, XS, 2, 0); // cmp w17, w2
        let m3 = a.bcond(NE);
        a.bind(ok);
        [m1, m2, m3]
    };

    // Shared AOT probe table (`aot_probe_enabled`; layout: `aot::probe_table`) search of x0
    // after a `probe` miss: on a live entry branches to `hit` (word position of the stub's
    // hit path, x1 = code); otherwise falls through (and returns the branches to bind
    // there). Same clobbers as `probe`. Emits nothing when the knob is off.
    let aot_on = cfg.aot_on;
    let count_probe_hits = cfg.count_probe_hits;
    let aot_probe = |a: &mut Asm, hit: usize| -> Vec<usize> {
        if !aot_on {
            return Vec::new();
        }
        a.ldr(64, XA, XCPU, OFF_AOTDIR);
        let m0 = a.cbz(true, XA);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.ldr(64, 2, XA, 0); // shift
        a.shiftv(true, 9, XS, XS, 2); // lsrv
        let lp = a.pos;
        a.addsub_rr(true, false, false, 1, XA, XS, 4);
        a.ldp_x(2, 1, 1, 16); // (ip, fast)
        a.addsub_rr(true, true, true, ZR, 2, 0, 0); // cmp x2, x0
        let found = a.bcond(EQ);
        let m1 = a.cbz(true, 2);
        a.addsub_ri(true, false, false, XS, XS, 1, false);
        a.ldr(64, 2, XA, 8); // mask
        a.logic_rr(true, 0, XS, XS, 2, 0, 0);
        let back = a.b();
        a.patch(back, lp);
        a.bind(found);
        let m2 = a.cbz(true, 1);
        // Populate this thread's direct-mapped `table[jslot(x0)]` inline (`gen = 0, hits = 0,
        // code = x1`) so repeated indirect calls/jmps/rets hit in O(1) without probing again.
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JBITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 5);
        a.stp_x(0, 1, XA, 0);
        a.stp_x(ZR, 1, XA, 16);
        if count_probe_hits {
            a.ldr(64, 2, XCPU, OFF_PROBEHITS);
            a.addsub_ri(true, false, false, 2, 2, 1, false);
            a.str(64, 2, XCPU, OFF_PROBEHITS);
        }
        let h = a.b();
        a.patch(h, hit);
        vec![m0, m1, m2]
    };

    // With AW2_JIT_BUDGET_BACKEDGE, blocks only test the budget before backward/RAS
    // transfers; every dynamic transfer that reaches translated code tests it here (hit
    // paths: x0 is a translated guest IP, so leaving with EXIT_RIP there is exact).
    let bud = cfg.bud;

    // lookup_rip(x0 = guest rip): continue there, or exit with EXIT_RIP.
    let lookup_rip = a.pos;
    let miss = probe(a);
    let m_plain_rip = a.tbnz(1, 0);
    let hit = a.pos;
    if bud_light {
        a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
    }
    let bud_rip = bud.then(|| a.tbnz(XBUD, 63));
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    a.bind(m_plain_rip);
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    if let Some(f) = bud_rip {
        a.bind(f);
    }
    let rip_out = a.pos;
    a.str(64, 0, XCPU, OFF_RIP);
    a.movz(false, 0, EXIT_RIP as u32, 0);
    let e = a.addr_of(exit);
    a.b_to(e);

    // lookup_jmp(x0 = target)
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly in `is_plain_jmp` / `is_qpc_jmp`.
    let lookup_jmp = a.pos;
    let miss = probe(a);
    let is_plain_jmp = a.tbnz(1, 0);
    let hit = a.pos;
    if bud {
        if bud_light {
            a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
        }
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    let hx = cfg.hx;
    let mut to_xfer: Vec<usize> = Vec::new();
    let jmp_slow = a.pos;
    if hx {
        // Host tail call (or EXIT_JMP from the helper for guest targets); kind 1 | 2 (dynamic).
        a.movz(true, 2, 3, 0);
        to_xfer.push(a.b());
    } else {
        a.str(64, 0, XCPU, OFF_TARGET);
        a.str(64, 0, XCPU, OFF_RIP);
        a.movz(false, 0, EXIT_JMP as u32, 0);
        a.b_to(e);
    }

    // lookup_call(x0 = target, x3 = return address): push + continue, or EXIT_CALL.
    // With the RAS, call sites enter with `bl lookup_call` (x30 = their landing pad).
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly here.
    let lookup_call = a.pos;
    let miss = probe(a);
    let is_plain = a.tbnz(1, 0);
    let hit = a.pos;
    let unal = if cfg.pushpop {
        // Plain pre-indexed push (see `pushpop_enabled`): no alignment check.
        a.str_pre(T3, HG[4], -8);
        None
    } else {
        a.addsub_ri(true, true, false, XA, HG[4], 8, false); // x16 = rsp - 8
        a.logic_ri(true, 0, XS, XA, enc_logic_imm(7, true).unwrap());
        let unal = a.cbnz(true, XS);
        if cfg.stack_tso_active {
            a.stlr(64, T3, XA);
        } else {
            a.str(64, T3, XA, 0);
        }
        a.mov_rr(true, HG[4], XA);
        Some(unal)
    };
    if ras {
        ras_push(a, T3, 30);
    }
    if bud {
        if bud_light {
            a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
        }
        // The call itself is done (return address pushed): continue at the callee.
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    let call_slow = a.pos;
    if hx {
        // Host call (or EXIT_CALL from the helper for guest targets); nothing pushed yet.
        // kind 0 | 2 (dynamic).
        a.str(64, T3, XCPU, OFF_RIP);
        a.movz(true, 2, 2, 0);
        to_xfer.push(a.b());
    }
    if let Some(u) = unal {
        a.bind(u);
    }
    a.str(64, 0, XCPU, OFF_TARGET);
    a.str(64, T3, XCPU, OFF_RIP);
    a.movz(false, 0, EXIT_CALL as u32, 0);
    a.b_to(e);

    // Fast path (`lookup_call`) for tagged `DF_PLAIN` (bit 0), pure-float-math (bits 0..1),
    // and inline zero-spill QPC (bit 63 | bit 0) host entries cached in `table`.
    a.bind(is_plain);
    let is_qpc = a.tbnz(1, 63);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!3u64, true).unwrap());
    a.addsub_ri(true, true, false, 31, 31, 96, false); // sub sp, sp, #96
    a.stp_x(8, 9, 31, 0);
    a.stp_x(10, 11, 31, 16);
    a.stp_x(12, 13, 31, 32);
    a.stp_x(14, 15, 31, 48);
    a.stp_x(30, T3, 31, 64);
    let no_xmm_in = a.tbz(1, 1);
    if xpin {
        a.v_mov(0, 16);
        a.v_mov(1, 17);
        a.v_mov(2, 18);
        a.v_mov(3, 19);
    } else {
        a.ldp_q(0, 1, XCPU, OFF_XMM as i32);
        a.ldp_q(2, 3, XCPU, (OFF_XMM + 32) as i32);
    }
    a.bind(no_xmm_in);
    if let Some(sp_pos) = spill_xmm {
        let sp_addr = a.addr_of(sp_pos);
        a.bl_to(sp_addr);
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    for (j, gi) in [1u8, 2, 8, 9].into_iter().enumerate() {
        a.mov_rr(true, j as u32, g(gi));
    }
    a.ldp_x(4, 5, g(4), 0x20);
    a.ldp_x(6, 7, g(4), 0x30);
    a.mrs_fpcr(XS);
    a.str(64, XS, 31, 80);
    let fpcr_plain_z = a.cbz(true, XS);
    a.msr_fpcr(ZR);
    a.bind(fpcr_plain_z);
    a.blr(XA);
    a.ldr(64, XS, 31, 80);
    let fpcr_plain_r = a.cbz(true, XS);
    a.msr_fpcr(XS);
    a.bind(fpcr_plain_r);
    a.ldp_x(8, 9, 31, 0);
    a.ldp_x(10, 11, 31, 16);
    a.ldp_x(12, 13, 31, 32);
    a.ldp_x(14, 15, 31, 48);
    if let Some(rl_pos) = reload_xmm {
        let rl_addr = a.addr_of(rl_pos);
        a.bl_to(rl_addr);
        a.v_mov(16, 0);
    } else {
        a.str(128, 0, XCPU, OFF_XMM);
    }
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    a.ldp_x(30, T3, 31, 64);
    a.addsub_ri(true, false, false, 31, 31, 96, false); // add sp, sp, #96
    a.mov_rr(true, g(0), 0);
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    let call_bud_out = if ras {
        let bud_plain = bud.then(|| a.tbnz(XBUD, 63));
        a.ret();
        if let Some(f) = bud_plain {
            a.bind(f);
            let out = a.pos;
            a.mov_rr(true, 0, T3);
            let ro = a.addr_of(rip_out);
            a.b_to(ro);
            Some(out)
        } else {
            None
        }
    } else {
        a.mov_rr(true, 0, T3);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        None
    };

    // Zero-spill inline QPC (`lookup_call`): `x0` = target, `T3` = return address, `x30` = landing pad.
    a.bind(is_qpc);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!((1u64 << 63) | 3), true).unwrap());
    a.ldp_x(2, XS, XA, 0); // x2 = mode, x17 = mult
    a.addsub_ri(true, true, true, ZR, 2, 1, false); // cmp x2, #1
    let qpc_slow = a.bcond(NE);
    a.patch(qpc_slow, call_slow);
    a.mrs_cntvct(2);
    a.ldr(64, XA, XA, 16); // x16 = offset
    a.umulh(2, 2, XS);
    a.addsub_rr(true, false, false, 2, 2, XA, 0);
    a.str(64, 2, g(1), 0); // *lpPerformanceCount = result
    a.movz(false, g(0), 1, 0); // eax = 1 (TRUE, zero-extended to rax)
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    if ras {
        if let Some(out) = call_bud_out {
            let f = a.tbnz(XBUD, 63);
            a.patch(f, out);
        }
        a.ret();
    } else {
        a.mov_rr(true, 0, T3);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // Fast path (`lookup_jmp`) for tail jumps to tagged `DF_PLAIN` / pure-float-math / inline-QPC
    // host entries cached in `table`.
    let mut to_ras_miss: Vec<usize> = Vec::new();
    a.bind(is_plain_jmp);
    let is_qpc_jmp = a.tbnz(1, 63);
    a.ldr_post(T3, g(4), 8); // pop guest return address; g(4) now points to Win64 shadow space
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!3u64, true).unwrap());
    a.addsub_ri(true, true, false, 31, 31, 96, false); // sub sp, sp, #96
    a.stp_x(8, 9, 31, 0);
    a.stp_x(10, 11, 31, 16);
    a.stp_x(12, 13, 31, 32);
    a.stp_x(14, 15, 31, 48);
    a.stp_x(30, T3, 31, 64);
    let no_xmm_in_jmp = a.tbz(1, 1);
    if xpin {
        a.v_mov(0, 16);
        a.v_mov(1, 17);
        a.v_mov(2, 18);
        a.v_mov(3, 19);
    } else {
        a.ldp_q(0, 1, XCPU, OFF_XMM as i32);
        a.ldp_q(2, 3, XCPU, (OFF_XMM + 32) as i32);
    }
    a.bind(no_xmm_in_jmp);
    if let Some(sp_pos) = spill_xmm {
        let sp_addr = a.addr_of(sp_pos);
        a.bl_to(sp_addr);
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    for (j, gi) in [1u8, 2, 8, 9].into_iter().enumerate() {
        a.mov_rr(true, j as u32, g(gi));
    }
    a.ldp_x(4, 5, g(4), 0x20);
    a.ldp_x(6, 7, g(4), 0x30);
    a.mrs_fpcr(XS);
    a.str(64, XS, 31, 80);
    let fpcr_pj_z = a.cbz(true, XS);
    a.msr_fpcr(ZR);
    a.bind(fpcr_pj_z);
    a.blr(XA);
    a.ldr(64, XS, 31, 80);
    let fpcr_pj_r = a.cbz(true, XS);
    a.msr_fpcr(XS);
    a.bind(fpcr_pj_r);
    a.ldp_x(8, 9, 31, 0);
    a.ldp_x(10, 11, 31, 16);
    a.ldp_x(12, 13, 31, 32);
    a.ldp_x(14, 15, 31, 48);
    if let Some(rl_pos) = reload_xmm {
        let rl_addr = a.addr_of(rl_pos);
        a.bl_to(rl_addr);
        a.v_mov(16, 0);
    } else {
        a.str(128, 0, XCPU, OFF_XMM);
    }
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    a.ldp_x(30, T3, 31, 64);
    a.addsub_ri(true, false, false, 31, 31, 96, false); // add sp, sp, #96
    a.mov_rr(true, g(0), 0);
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    a.mov_rr(true, 0, T3);
    if bud {
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    if ras {
        a.ldp_x(1, 2, XRAS, 0);
        a.addsub_ri(true, true, false, XA, XRAS, 16, false);
        a.logic_rr(true, 2, 1, 1, 0, 0, 0); // eor x1, x1, x0
        to_ras_miss.push(a.cbnz(true, 1));
        a.bfxil0(XRAS, XA, 16);
        a.ret_r(2);
    } else {
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // Zero-spill inline QPC (`lookup_jmp`): `x0` = target, `[g(4)]` = guest return address.
    a.bind(is_qpc_jmp);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!((1u64 << 63) | 3), true).unwrap());
    a.ldp_x(2, XS, XA, 0); // x2 = mode, x17 = mult
    a.addsub_ri(true, true, true, ZR, 2, 1, false); // cmp x2, #1
    let qpc_jmp_slow = a.bcond(NE);
    a.patch(qpc_jmp_slow, jmp_slow);
    a.mrs_cntvct(2);
    a.ldr(64, XA, XA, 16); // x16 = offset
    a.umulh(2, 2, XS);
    a.addsub_rr(true, false, false, 2, 2, XA, 0);
    a.str(64, 2, g(1), 0); // *lpPerformanceCount = result
    a.movz(false, g(0), 1, 0); // eax = 1 (TRUE, zero-extended to rax)
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    a.ldr_post(0, g(4), 8); // pop guest return address into x0
    if bud {
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    if ras {
        a.ldp_x(1, 2, XRAS, 0);
        a.addsub_ri(true, true, false, XA, XRAS, 16, false);
        a.logic_rr(true, 2, 1, 1, 0, 0, 0); // eor x1, x1, x0
        to_ras_miss.push(a.cbnz(true, 1));
        a.bfxil0(XRAS, XA, 16);
        a.ret_r(2);
    } else {
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // host_xfer(x0 = target, x2 = kind: bit 0 = jmp (else call), bit 1 = dynamic target from
    // `lookup_call`/`lookup_jmp` (else a static target from `xfer_to`); cpu.rip = return
    // address for calls).
    let mut host_xfer_bl: Option<usize> = None;
    let host_xfer = if hx {
        let p = a.pos;
        for i in 0..8 {
            a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if xpin {
            for i in 0..8u32 {
                a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
            }
        }
        if ras {
            a.str(64, XRAS, XCPU, OFF_RAS);
        }
        a.asr_i(true, XA, XBUD, 1);
        a.str(64, XA, XCPU, OFF_BUDGET);
        a.mov_rr(true, 19, XCPU);
        a.mov_rr(true, 1, 0);
        a.mov_rr(true, 0, XCPU);
        if cfg.static_pic {
            // Resolved permanently at link time by `ARM64_RELOC_BRANCH26` to
            // `_port_static_aot_host_xfer` (zero baked host addresses in __TEXT).
            let bl_pos = a.bl();
            host_xfer_bl = Some(bl_pos * 4);
        } else {
            a.mov_imm(true, XA, super::jit_host_xfer as *const () as u64);
            a.blr(XA);
        }
        a.mov_rr(true, XCPU, 19);
        let out = a.cbnz(true, 0);
        for i in 0..8 {
            a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if xpin {
            for i in 0..8u32 {
                a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
            }
        }
        if ras {
            a.ldr(64, XRAS, XCPU, OFF_RAS);
        }
        a.ldr(64, XBUD, XCPU, OFF_BUDGET);
        a.lsl_i(true, XBUD, XBUD, 1);
        a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
        a.addsub_ri(false, true, true, ZR, XS, 0, false);
        a.cset(XS, NE);
        a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0);
        a.ldr(64, 0, XCPU, OFF_RIP);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        a.bind(out);
        // GPRs and budget are already stored (x19 now holds the Cpu pointer).
        let ens = a.addr_of(exit_ns);
        a.b_to(ens);
        for f in to_xfer {
            a.patch(f, p);
        }
        Some(p)
    } else {
        None
    };

    // ras_miss(x0 = guest return target, XRAS = RAS top whose entry did not match): the
    // entry for this return may sit up to 3 entries deeper (frames left without a
    // translated `ret`: longjmp/SEH unwinds, rets run by the interpreter, host-called guest
    // code). Pop through the first match and return to its landing pad; otherwise keep the
    // stack as is (the return has no entry, e.g. its call was interpreted) and probe.
    // Any entry whose guest address equals the target is a correct continuation: its
    // landing pad transfers to exactly that guest IP.
    let ras_miss = if ras {
        if let Some(want) = cfg.target_ras_miss {
            while a.pos < want && !a.overflow {
                a.emit(0xD503_201F); // nop padding to match .aot stub table offset exactly
            }
        }
        let p = a.pos;
        for f in to_ras_miss {
            a.patch(f, p);
        }
        let mut found = Vec::new();
        for i in 1..=3u32 {
            a.addsub_ri(true, true, false, XA, XRAS, 16 * i, false);
            a.mov_rr(true, 1, XRAS);
            a.bfxil0(1, XA, 16);
            a.ldr(64, 2, 1, 0);
            a.logic_rr(true, 2, 2, 2, 0, 0, 0); // eor x2, x2, x0
            found.push(a.cbz(true, 2));
        }
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        for f in found {
            a.bind(f);
        }
        // x1 = matching entry: new top = the entry below it.
        a.ldr(64, 2, 1, 8);
        a.addsub_ri(true, true, false, XA, 1, 16, false);
        a.mov_rr(true, XRAS, 1);
        a.bfxil0(XRAS, XA, 16);
        a.br(2);
        Some(p)
    } else {
        None
    };

    (
        Stubs { enter, exit, lookup_rip, lookup_jmp, lookup_call, host_xfer, ras_miss, spill_xmm, reload_xmm, clear_ymm_hi },
        host_xfer_bl,
    )
}

/// Push the shadow RAS entry (guest return address in register `g`, host landing pad in
/// register `h`). Updates pinned `XRAS` in place; clobbers x16.
fn ras_push(a: &mut Asm, g: u32, h: u32) {
    a.addsub_ri(true, false, false, XA, XRAS, 16, false);
    a.bfxil0(XRAS, XA, 16);
    a.stp_x(g, h, XRAS, 0);
}

/// Word positions of the shared stubs.
struct Stubs {
    enter: usize,
    exit: usize,
    lookup_rip: usize,
    lookup_jmp: usize,
    lookup_call: usize,
    ras_miss: Option<usize>,
    host_xfer: Option<usize>,
    spill_xmm: Option<usize>,
    reload_xmm: Option<usize>,
    clear_ymm_hi: usize,
}

/// `AW2_JIT_HOSTCALL=0` makes host calls/jmps from translated code leave translated code
/// (`EXIT_CALL`/`EXIT_JMP`, dispatched by the interpreter loop) instead of calling the host
/// function from the `host_xfer` stub / inline. Default on.
fn hostcall_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let on = std::env::var("AW2_JIT_HOSTCALL").map_or(true, |s| s.trim() != "0");
        log::info!("emu jit: in-translation host calls {}", if on { "on" } else { "off (AW2_JIT_HOSTCALL=0)" });
        on
    })
}

/// `AW2_JIT_PLAINCALL=0` disables the inline call sequence for `DF_PLAIN` builtins
/// (memcpy/memset/SRW locks/...) in translated code; they then go through `host_xfer`.
/// Default on; implied off by `AW2_JIT_HOSTCALL=0` and while the interpreter profiler is on.
pub(super) fn plaincall_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        hostcall_enabled()
            && std::env::var("AW2_JIT_PLAINCALL").map_or(true, |s| s.trim() != "0")
            && crate::emu_prof::level() == 0
            && !crate::diag::nan_trace_installed()
    })
}

/// Inline-callable host function for the static call/jmp target `t`: `(fn, bail, needs_xmm)`,
/// `fn` = the `DF_PLAIN` builtin itself or [`super::inline_variant`]'s replacement; `bail` = a 0
/// result means "make the real call" (through `host_xfer`); `needs_xmm` = false for pure-integer
/// `DF_PLAIN` builtins (skip XMM0..3 loads and XMM0 store).
pub(super) fn inline_host_fn_kind(t: u64) -> Option<(usize, bool, bool)> {
    if t == 0 || is_chkstk(t) || !plaincall_enabled() {
        return None;
    }
    // Not `resolve_dispatch`: the result is baked into translated code, so a DLL import stub
    // (`DF_IAT`, slot read per call) must not resolve to its current slot target here.
    let (h, f) = super::resolve_dispatch_static(t);
    if h == 0 {
        return None;
    }
    if f == super::DF_VALID | super::DF_HOST | super::DF_PLAIN {
        return Some((h, false, false));
    }
    if f == super::DF_VALID | super::DF_HOST {
        return super::inline_variant(h).map(|(f2, bail)| (f2, bail, true));
    }
    None
}

/// [`inline_host_fn_kind`] returning `(fn, bail)` for AOT relocations.
pub(super) fn inline_host_fn(t: u64) -> Option<(usize, bool)> {
    inline_host_fn_kind(t).map(|(h, bail, _)| (h, bail))
}

/// Direct `call/jmp` to a mapped DLL's IAT stub (`DF_IAT`, `jmp qword [slot]`): returns
/// `(slot, value, fn, bail, needs_xmm)` if the slot currently holds an inline-callable target.
fn iat_slot_plain(t: u64) -> Option<(u64, u64, usize, bool, bool)> {
    if t == 0 || is_chkstk(t) || !plaincall_enabled() {
        return None;
    }
    let (slot, f) = super::resolve_dispatch_static(t);
    if (f & super::DF_IAT) == 0 || slot == 0 || (slot & 7) != 0 {
        return None;
    }
    let slot = slot as u64;
    if !(0x1_0000_0000..1u64 << 47).contains(&slot) {
        return None;
    }
    let v = unsafe { core::ptr::read_volatile(slot as *const u64) };
    inline_host_fn_kind(v).map(|(h, bail, needs_xmm)| (slot, v, h, bail, needs_xmm))
}

/// `call/jmp qword [abs]` (rip-relative import slot in the exe or a mapped DLL image)
/// whose slot currently holds an inline-callable target: `(slot, value, fn, bail, needs_xmm)`.
/// The translated code re-checks the value.
fn const_slot_plain(op: &FastOp) -> Option<(u64, u64, usize, bool, bool)> {
    if op.base != RNONE || op.index != RNONE || !plaincall_enabled() {
        return None;
    }
    let slot = op.disp as u64;
    let b = crate::exe::base() as u64;
    let in_image = (b..b.wrapping_add(crate::exe::image_size())).contains(&slot) || crate::dll_native::in_mapped_image(slot);
    if !(in_image || cfg!(test)) || !(0x1_0000_0000..1u64 << 47).contains(&slot) || slot & 7 != 0 {
        return None;
    }
    let v = unsafe { core::ptr::read_volatile(slot as *const u64) };
    inline_host_fn_kind(v).map(|(h, bail, needs_xmm)| (slot, v, h, bail, needs_xmm))
}

// ---------------------------------------------------------------------------------------------
// Successor analysis.
// ---------------------------------------------------------------------------------------------

/// Map iced `RflagsBits` to our flag bits.
fn iced_bits(r: u32) -> u8 {
    use iced_x86::RflagsBits as R;
    let mut f = 0;
    if r & R::CF != 0 {
        f |= FC;
    }
    if r & R::ZF != 0 {
        f |= FZ;
    }
    if r & R::SF != 0 {
        f |= FS;
    }
    if r & R::OF != 0 {
        f |= FO;
    }
    if r & R::PF != 0 {
        f |= FP;
    }
    f
}

/// Whether it is safe to speculatively decode a successor block at guest `ip` (for
/// `succ_need` / `succ_lb_kill`). Packed or obfuscated binaries can contain dead
/// fall-through junk bytes that decode as branches targeting unmapped memory or
/// `PAGE_NOACCESS` pages; speculative liveness must never fault trying to read them.
#[inline]
fn can_speculate_code(ip: u64) -> bool {
    let mapped = to_mapped_addr(ip);
    let min_bytes = BLK_MAX_INSNS * 15;
    if !(0x1_0000..0x8000_0000_0000).contains(&mapped) {
        return false;
    }
    #[cfg(not(test))]
    {
        let b = crate::exe::base() as u64;
        let end = b.wrapping_add(crate::exe::image_size());
        let img_end = if ip >= b && ip < end {
            Some(end)
        } else {
            crate::dll_native::mapped_image_end(ip)
        };
        let Some(img_end) = img_end else {
            return false;
        };
        if img_end.saturating_sub(ip) < min_bytes as u64 {
            return false;
        }
    }
    win32_rt::kernel32::memory::is_speculative_read_safe(mapped, min_bytes)
}

/// Flags that may be read at guest `ip` before being overwritten (conservative).
unsafe fn succ_need(bc: *mut BlockCache, ip: u64, depth: u32) -> u8 {
    use iced_x86::Mnemonic as M;
    if !can_speculate_code(ip) || resolved_host(bc, ip) || mem::in_jit(to_mapped_addr(ip)) {
        return FALL;
    }
    let mut dummy = 0u64;
    let Ok((off, n)) = BlockCache::get(bc, ip, &mut dummy) else { return FALL };
    let mut need = 0u8;
    let mut kill = 0u8;
    let lazy = flaglazy_enabled();
    for i in 0..n {
        let ins = &(&(*bc).arena)[off + i];
        need |= iced_bits(ins.rflags_read()) & !kill;
        let mut k = match ins.mnemonic() {
            // Count may be zero (flags unchanged).
            M::Shl | M::Sal | M::Shr | M::Sar | M::Rol | M::Ror | M::Rcl | M::Rcr | M::Shld | M::Shrd => 0,
            _ => iced_bits(ins.rflags_written() | ins.rflags_cleared() | ins.rflags_set()),
        };
        // AW2_JIT_FLAGLAZY (per-edge flushes rely on exact kills): a fast op kills only what
        // the emulator's own model says it overwrites, in case it differs from iced's.
        if lazy {
            let op = &(&(*bc).ops)[off + i];
            if op.kind != fast::K_INSN {
                k &= op_flags(op).1;
            }
        }
        kill |= k;
        if kill & FALL == FALL {
            return need;
        }
        if i + 1 == n {
            use iced_x86::FlowControl as F;
            let flow = ins.flow_control();
            let dead = match flow {
                F::Call | F::IndirectCall => flags_dead_mask() & 1 != 0,
                F::Return => flags_dead_mask() & 2 != 0,
                _ => false,
            };
            if dead {
                return need;
            }
            // AW2_JIT_FLAGLAZY: look through a direct jmp / jcc at the end of the block
            // (values copied first: the recursion may grow the arena).
            let (tgt, nxt) = (ins.near_branch_target(), ins.next_ip());
            if depth > 1 && tgt != 0 {
                let rest = match flow {
                    F::UnconditionalBranch => Some(succ_need(bc, tgt, depth - 1)),
                    F::ConditionalBranch => Some(succ_need(bc, tgt, depth - 1) | succ_need(bc, nxt, depth - 1)),
                    _ => None,
                };
                if let Some(r) = rest {
                    return need | (r & !kill);
                }
            }
        }
    }
    need | (FALL & !kill)
}

// ---------------------------------------------------------------------------------------------
// Block translation.
// ---------------------------------------------------------------------------------------------

pub(super) fn supported(op: &FastOp) -> bool {
    use fast::*;
    matches!(
        op.kind,
        K_NOP
            | K_MOV_RR
            | K_MOV_RM
            | K_MOV_MR
            | K_MOV_RI
            | K_MOV_MI
            | K_MOVZX_RR
            | K_MOVZX_RM
            | K_MOVSX_RR
            | K_MOVSX_RM
            | K_LEA
            | K_ALU_RR
            | K_ALU_RI
            | K_ALU_RM
            | K_ALU_MR
            | K_ALU_MI
            | K_INC_R
            | K_DEC_R
            | K_SHIFT_RI
            | K_SHIFT_RCL
            | K_PUSH_R
            | K_POP_R
            | K_RET
            | K_JCC
            | K_JMP
            | K_CMOV_RR
            | K_CMOV_RM
            | K_SETCC_R
            | K_CALL
            | K_CALL_M
            | K_CALL_R
            | K_JMP_M
            | K_JMP_R
            | K_CMPJ_RR
            | K_CMPJ_RI
            | K_CMPJ_RM
            | K_VSHUF_RR..=K_VSHA_RM
    )
}

#[derive(Clone, Copy)]
pub(super) enum Term {
    /// Straight-line code continues at this IP.
    Fall(u64),
    Jcc { cc: u8, target: u64, fall: u64, near: bool },
    /// Fused `ucomiss/comiss + jp/jnp t1; je/jne t2; fall2`.
    JccPair { cc1: u8, t1: u64, cc2: u8, t2: u64, fall2: u64 },
    Loop { sub: u8, target: u64, fall: u64 },
    Jmp(u64),
    Call(u64),
    CallR,
    CallM,
    JmpR,
    JmpM,
    Ret,
}

fn classify(ops: &[FastOp]) -> Term {
    use fast::*;
    let last = ops[ops.len() - 1];
    let next = last.ip.wrapping_add(last.len as u64);
    match last.kind {
        K_JCC => Term::Jcc { cc: last.x & 0x3F, target: last.imm, fall: next, near: last.len > 2 },
        K_LOOP => Term::Loop { sub: last.x & 0x3F, target: last.imm, fall: next },
        K_JMP => Term::Jmp(last.imm),
        K_CALL => Term::Call(last.imm),
        K_CALL_R => Term::CallR,
        K_CALL_M => Term::CallM,
        K_JMP_R => Term::JmpR,
        K_JMP_M => Term::JmpM,
        K_RET => Term::Ret,
        _ => Term::Fall(next),
    }
}

/// Which x86 flags the host NZCV currently encodes, and how.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Src {
    None,
    /// `subs`-style: C = !CF.
    Sub,
    /// `adds`-style: C = CF.
    Add,
    /// Logic op: CF = OF = 0.
    Logic,
    /// inc/dec: N, Z, V valid; CF is not in NZCV.
    IncDec,
    /// `fcmp` (`comiss`/`ucomiss`/`comisd`/`ucomisd`): CF = LT, ZF = EQ | VS, PF = VS, SF = OF = 0.
    Fcmp,
}

#[derive(Clone, Copy)]
struct FlagState {
    src: Src,
    /// Flags (subset of FNZCV) currently valid in host NZCV.
    host: u8,
    /// Flags currently valid in the `Cpu` flag bytes.
    mem: u8,
}

enum Cond {
    Host(u32),
    /// Boolean in a register; `inv` = branch/select on zero.
    Reg(u32, bool),
    Const(bool),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Chk {
    /// Range + alignment.
    Full,
    /// Alignment only (address already range-checked).
    Align,
    /// rsp-relative stack access (push/pop/call, `[rsp+disp]`): like `Align`, but with
    /// `AW2_JIT_STACK_TSO=0` a plain `ldr`/`str` without acquire/release or alignment check.
    Stack,
    /// Inside read-only image data (`AW2_JIT_RO_PLAIN`): the address is valid and the data
    /// never changes, so loads are a plain `ldr` (no range/alignment check, no acquire);
    /// stores (never expected) are treated like `Align`.
    Ro,
    /// Translation-time constant address (absolute / rip-relative operand) inside
    /// `[0x10000, 2^47)` (`AW2_JIT_MEMCHK` bit 1): no range check; `true` = 8-byte aligned,
    /// so no alignment check either (otherwise like `Align`).
    Abs(bool),
}

/// `PORT_JIT_STACK_TSO` / `AW2_JIT_STACK_TSO` (default 0 in production, 1 in unit tests):
/// TSO ordering (`ldapr`/`stlr`) for rsp-relative stack accesses in translated code.
/// Stack slots are thread-private (and any cross-thread publication happens through a heap/
/// global release store or syscall), matching `push`/`pop` (`AW2_JIT_PUSHPOP`). Set `=1`
/// for strict acquire/release ordering on `[rsp+disp]`.
fn stack_tso() -> bool {
    #[cfg(test)]
    {
        if let Ok(s) = std::env::var("PORT_JIT_STACK_TSO").or_else(|_| std::env::var("AW2_JIT_STACK_TSO")) {
            return s.trim() != "0";
        }
        return true;
    }
    #[cfg(not(test))]
    {
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| {
            let on = std::env::var("PORT_JIT_STACK_TSO")
                .or_else(|_| std::env::var("AW2_JIT_STACK_TSO"))
                .map_or(false, |s| s.trim() != "0");
            log::info!("emu jit: stack TSO {}", if on { "on" } else { "off (plain [rsp+disp])" });
            on
        })
    }
}

struct SideExit {
    fix: usize,
    k: usize,
    fs: FlagState,
    lb: [u8; 16],
    /// Dirty cached XMM registers (`AW2_JIT_XMMREG`) to store before leaving.
    xd: u16,
    /// Pending parity byte (`Tr::pf`).
    pf: u8,
}

struct SlowMem {
    fix: usize,
    back: usize,
    ld: bool,
    reg: u32,
    addr: u32,
    bits: u32,
    /// Side exit to take if the range check fails (None: range already known good).
    exit: Option<usize>,
    /// Range-check `addr + lo` instead of `addr` (lead of a base-register group, `<= 0`).
    lo: i32,
}

struct Tr<'a> {
    a: Asm,
    ops: &'a [FastOp],
    /// `live[k]`: flags live before op `k`; `live[ops.len()]` = live-out.
    live: [u8; BLK_MAX_INSNS + 1],
    fs: FlagState,
    lb: [u8; 16],
    side_exits: Vec<SideExit>,
    slow: Vec<SlowMem>,
    links: Vec<(usize, u64)>,
    /// Hot conditional branches `(cond_site, target)` emitted with a cold `b` trampoline.
    cond_links: Vec<(usize, u64)>,
    /// Pairs `(cold_b_site, hot_cond_site)` recorded when materializing `cond_links`.
    link_conds: Vec<(usize, usize)>,
    /// Word offset right before the literal pool (`emit_lits`); set at the end of `translate`.
    cold_code_end: usize,
    exit: usize,
    lookup_rip: usize,
    lookup_jmp: usize,
    lookup_call: usize,
    /// See [`JitCache::host_xfer`].
    host_xfer: usize,
    /// Shared XMM spill/reload stubs (0 when `PORT_JIT_XMM_PIN=0`) and `clear_ymm_hi` stub.
    spill_xmm: usize,
    reload_xmm: usize,
    clear_ymm_hi: usize,
    /// Static terminator target needs host dispatch (or is 0 / `__chkstk`).
    target_host: bool,
    /// [`JitCache::ras_miss`]: non-zero = shadow return-address stack on (`AW2_JIT_RAS`).
    ras_miss: usize,
    /// Guest GPRs each op may write ([`gpr_writes`]; all zero unless `AW2_JIT_MEMCHK` bit 2
    /// or 3 needs them).
    wr: [u16; BLK_MAX_INSNS],
    /// Base-register check groups (`AW2_JIT_MEMCHK` bit 2), see [`Tr::plan_groups`].
    grp: [Grp; BLK_MAX_INSNS],
    /// Groups whose lead range check has been emitted (bit = group id).
    grp_ok: u64,
    /// Op whose first range-checked access already took its group role.
    grp_k: usize,
    /// Number of ops before the terminator (all of them for `Term::Fall`).
    body: usize,
    /// Following ops already translated together with the current one (push/pop pairs).
    skip: usize,
    /// rsp is known 8-aligned here (tested in this block and only moved by multiples of 8).
    rsp_al: bool,
    /// `AW2_JIT_XMMREG` (see `jit_xmm.rs`).
    xon: bool,
    /// `PORT_JIT_XMM_PIN` (see `jit_xmm.rs`): `xmm0..15` pinned in `v16..v31`.
    xpin: bool,
    /// Bitmask of `Cpu::ymm_hi[0..16]` known zero at the current translation point.
    ymm_clean: u16,
    /// Deferred cold calls to `clear_ymm_hi`: `(tbnz_site, resume_pos, mask)`.
    ymm_colds: Vec<(usize, usize, u16)>,
    /// Bitmasks of guest `xmm0..15` whose upper bits `[127:32]` / `[127:64]` are known 0.
    xmm_hi32_zero: u16,
    xmm_hi64_zero: u16,
    /// `xmm_hi32_live[k]` / `xmm_hi64_live[k]`: guest `xmm0..15` whose bits `[127:32]` /
    /// `[127:64]` may be read at/after op `k`.
    xmm_hi32_live: [u16; BLK_MAX_INSNS + 1],
    xmm_hi64_live: [u16; BLK_MAX_INSNS + 1],
    /// Single-entry address cache for `XA` (`x16`): `Some((base, idx, scale, hi_disp))`.
    xa_key: Option<(u8, u8, u8, i64)>,
    /// Set by `try_batch_epilogue` when `T0` (`x0`) and RAS `(x1, x2)` are already preloaded for `Term::Ret`.
    ret_preloaded: bool,
    /// True when the block ends in `Term::Ret` with `imm == 0`.
    term_is_plain_ret: bool,
    /// Live-out flags for the block's terminator.
    term_live_out: u8,
    /// Whether this block decremented `XBUD` at entry (`PORT_JIT_BUDGET_LIGHT`).
    bud_dec: bool,
    /// `PORT_JIT_FAST_NAN`: native IEEE-754 ARM64 FP without per-op x86 NaN checks.
    fast_nan: bool,
    /// `PORT_JIT_LAST_BITS`: maintain `Cpu::last_bits` for `DF_GAME_REPL` hooks.
    lb_on: bool,
    /// Guest XMM registers with a valid copy in their cache register `v(16 + i)`.
    xv: u16,
    /// Cached guest XMM registers newer than `Cpu::xmm`.
    xd: u16,
    /// Out-of-line NaN fixups.
    cold: Vec<ColdNan>,
    /// `AW2_JIT_RETLIT`.
    retlit: bool,
    /// `ldr` literal sites and their values (pool emitted after the out-of-line code).
    lits: Vec<(usize, u64)>,
    /// 16-byte `ldr qt, <literal>` sites `(site_word, lo64, hi64)` (16B-aligned pool).
    lits128: Vec<(usize, u64, u64)>,
    /// `AW2_JIT_FLAGLAZY` (see `jit_lazy.rs`).
    lazy: bool,
    /// Per-edge successor flag needs and `last_bits` kills.
    edge: Edges,
    /// Pending parity byte: guest GPR whose low byte is `Cpu::pfv` but not stored yet
    /// (`PF_NONE`: nothing pending).
    pf: u8,
    /// AOT emission mode ([`new_tr`]): host function pointers always come from the literal
    /// pool (one private slot each) and are recorded in `relocs`.
    aot: bool,
    /// AOT mode: relocated `ldr` literal sites `(site, value, relocation)` (host function
    /// pointers and import-slot values; `AotReloc::word` is set by `emit_lits`).
    rlits: Vec<(usize, u64, AotReloc)>,
    /// AOT mode: relocated pool slots (filled by `emit_lits`).
    relocs: Vec<AotReloc>,
}

/// Role of an op's memory operand in a base-register check group: accesses `[b+disp]` off
/// the same base register `b`, not written in between, with all displacements within
/// 4 KiB. The lead's range check covers `b + min(disp)`, so `b + disp` of every member lies
/// in `[0x10000, 2^47 + 0x11000)` — exact at the low end; the high end only adds 4 KiB to
/// the existing check's 64 KiB of never-mapped slack above 2^47.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Grp {
    None,
    /// First access: range-check `a + lo` (`lo` = lowest displacement - own, `<= 0`).
    Lead { lo: i16, id: u8 },
    /// Range already checked by the lead of group `id` (only if that check was emitted).
    Member { id: u8 },
}

/// `(reads, kills)` flag sets of a fast op.
fn op_flags(op: &FastOp) -> (u8, u8) {
    use fast::*;
    match op.kind {
        K_ALU_RR | K_ALU_RI | K_ALU_RM | K_ALU_MR | K_ALU_MI => {
            if matches!(op.x & 0x3F, A_ADC | A_SBB) {
                (FC, FALL)
            } else {
                (0, FALL)
            }
        }
        K_CMPJ_RR | K_CMPJ_RI | K_CMPJ_RM | K_SHIFT_RI | K_SHIFT_MI | K_POPCNT_RR | K_POPCNT_RM
        | K_SHD_RI | K_XADD_RR | K_CMPXCHG_RR => (0, FALL),
        K_INC_R | K_DEC_R => (0, FZ | FS | FO | FP),
        K_JCC | K_CMOV_RR | K_CMOV_RM | K_SETCC_R | K_SETCC_M => (cc_reads(op.x & 0x3F), 0),
        // Pure SSE/AVX and flagless integer ops neither read nor write flags.
        K_VSHUF_RR..=K_CVTSI_M
        | K_CVTF2I_R
        | K_CVTF2I_M
        | K_NOT_R
        | K_CX
        | K_VZU
        | K_XCHG_RR
        | K_BSWAP_R
        | K_PUSH_I
        | K_PUSH_M
        | K_POP_M
        | K_LEAVE
        | K_MOVBE_RM..=K_VBLENDV_RM
        | K_YMM_LANE => (0, 0),
        K_COMI_RR | K_COMI_RM | K_NEG_R | K_VTEST_RR | K_VTEST_RM => (0, FALL),
        K_BMI_RR | K_BMI_RM => match op.x & 0x3F {
            0 | 9 => (0, FALL),                 // andn, bextr
            5..=7 => (0, FC | FZ | FS | FO),    // blsr, blsmsk, blsi
            8 => (0, FC | FZ | FS | FO),        // bzhi
            11 => (FC, FC),                     // adcx
            12 => (FO, FO),                     // adox
            _ => (0, 0),                        // shlx/shrx/sarx/rorx/mulx/crc32
        },
        K_INSN => (FALL, FALL),
        K_LOOP => {
            if (op.x & 0x3F) >= 3 {
                (FZ, 0)
            } else {
                (0, 0)
            }
        }
        K_VAES_RR | K_VAES_RM | K_VSHA_RR | K_VSHA_RM | K_VCVT_PH | K_VMASKMOV => (0, 0),
        K_SYS_MISC => match op.x & 0x3F {
            0 => (FC | FZ | FS | FP, 0),
            1 => (0, FC | FZ | FS | FP),
            2 => (FALL, 0),
            3 => (0, FALL),
            _ => (0, 0),
        },
        K_HI8 => match op.x & 0x3F {
            0..=2 | 14 | 22 => (0, 0),
            3..=9 | 15 | 23 | 24 => (0, FALL),
            10 | 11 => (FC, FALL),
            12 | 13 => (0, FZ | FS | FO | FP),
            16..=20 => (FALL, FALL),
            21 => (cc_reads((op.imm as u8) & 0x3F), 0),
            _ => (0, 0),
        },
        K_STRING => {
            if (op.x & 7) >= 3 {
                (FALL, FALL)
            } else {
                (0, 0)
            }
        }
        K_BIT_EXT => {
            if (op.x & 0xF) >= 2 {
                (FALL, FALL)
            } else {
                (0, 0)
            }
        }
        K_ATOMIC => match op.x & 0x3F {
            0..=4 | 8 | 14 | 15 => (0, FALL),
            5 | 6 => (0, FZ | FS | FO | FP),
            9..=12 => (0, FC),
            16 | 17 => (0, FZ),
            _ => (0, 0),
        },
        K_PCMPSTR => (0, FALL),
        K_BT_RR | K_BT_RI | K_BIT_RR | K_BIT_RI => (0, FC),
        K_ROT_RI | K_ROT_MI | K_IMUL_RR..=K_IMUL_RMI => (0, FC | FO),
        K_MUL1_R | K_MUL1_M => {
            if (op.x & 0x3F) <= 1 {
                (0, FC | FO)
            } else {
                (0, 0)
            }
        }
        K_INC_M | K_DEC_M => (0, FZ | FS | FO | FP),
        K_BSF_RR | K_BSF_RM | K_TZCNT_RR | K_TZCNT_RM | K_LZCNT_RR | K_LZCNT_RM => (0, FC | FZ),
        K_BSR_RR | K_BSR_RM => (0, FZ),
        K_FLAG_OP => match op.x & 0x3F {
            0 | 1 => (0, FC),
            2 => (FC, FC),
            _ => (0, 0),
        },
        _ => (0, 0),
    }
}

const A_ADD: u8 = 0;
const A_SUB: u8 = 1;
const A_AND: u8 = 2;
const A_OR: u8 = 3;
const A_XOR: u8 = 4;
const A_CMP: u8 = 5;
const A_TEST: u8 = 6;

impl Tr<'_> {
    /// Translate the block; returns the number of words on the main path (entry through the
    /// terminator, before the out-of-line code).
    fn translate(&mut self, term: Term, live_out: u8) -> usize {
        let n = self.ops.len();
        self.live[n] = live_out;
        self.term_live_out = live_out;
        self.term_is_plain_ret = matches!(term, Term::Ret) && self.ops[n - 1].imm == 0;
        for k in (0..n).rev() {
            let (mut r, kl) = op_flags(&self.ops[k]);
            if k == n - 1 && matches!(term, Term::JccPair { .. }) {
                r |= FZ | FP;
            }
            self.live[k] = (self.live[k + 1] & !kl) | r;
        }
        // Entry: budget check (`XBUD = (budget << 1) | any_ymm_dirty`, so subtract `2 * n`).
        let bud_be = budget_backedge_enabled();
        let bud_dec = !bud_be || !budget_light_enabled() || self.term_tests_budget(&term);
        self.bud_dec = bud_dec;
        if bud_dec {
            self.a.addsub_ri(true, true, false, XBUD, XBUD, (2 * n) as u32, false);
        }
        let bfix = (!bud_be).then(|| self.a.tbnz(XBUD, 63));

        let body = if matches!(term, Term::Fall(_)) { n } else { n - 1 };
        self.body = body;
        // Lazy `Cpu::ymm_hi` zeroing for VEX-128 ops:
        // Bit 0 of `XBUD` (`x8`) mirrors `Cpu::jx.ymm_dirty != 0`. Any block with VEX-128
        // dest regs (and zero `ymm_hi` readers) emits a single 1-cycle `tbnz x8, #0, #cold`
        // at entry — zero memory reads/writes and zero flag/scratch clobbers on the common
        // all-clean path, calling shared `clear_ymm_hi` out-of-line on miss!
        let v128_mask = self.block_v128_mask(body);
        if v128_mask != 0 {
            self.emit_ymm_clear_check(v128_mask);
            self.ymm_clean = v128_mask;
        }
        if self.xon {
            self.compute_xmm_hi_live(body);
        }
        self.plan_groups();
        for k in 0..body {
            if self.skip != 0 {
                // Translated together with the previous op (push/pop or stack mov pair).
                self.skip -= 1;
            } else {
                if self.pf != PF_NONE {
                    self.pf_before_op(k);
                }
                self.op(k);
            }
            if let Some((b, idx, _, _)) = self.xa_key {
                let w = self.wr[k];
                if (w & (1u16 << b)) != 0 || (idx < RNONE && (w & (1u16 << idx)) != 0) {
                    self.xa_key = None;
                }
            }
            self.track_rsp(k);
        }
        self.xa_key = None;
        // Every transfer out of the block (terminator, budget exit) sees Cpu::xmm current;
        // terminators (inline plain calls) may write Cpu::xmm0 directly.
        self.xbarrier();
        if bud_be && self.term_tests_budget(&term) {
            // Leave before the terminator (the interpreter runs it) once the budget is out.
            let se = self.side_exit_here(n - 1);
            let f = self.a.tbnz(XBUD, 63);
            self.bind_side_exit(se, f);
        }
        self.terminator(term, n - 1);
        let main = self.a.pos;

        // Out-of-line code.
        if let Some(bfix) = bfix {
            self.a.bind(bfix);
            self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * n) as u32, false);
            self.exit_side(self.ops[0].ip);
        }
        let slow = core::mem::take(&mut self.slow);
        for s in &slow {
            self.a.bind(s.fix);
            if let Some(se) = s.exit {
                if s.lo != 0 {
                    self.a.addsub_ri(true, true, false, XS, s.addr, (-s.lo) as u32, false);
                    self.range_check_to(XS, se);
                } else {
                    self.range_check_to(s.addr, se);
                }
            }
            if s.ld {
                self.a.ldr(s.bits, s.reg, s.addr, 0);
                self.a.dmb_ishld();
            } else {
                self.a.dmb_ishst();
                self.a.str(s.bits, s.reg, s.addr, 0);
            }
            let here = self.a.pos;
            self.a.b();
            self.a.patch(here, s.back);
        }
        let ymm_colds = core::mem::take(&mut self.ymm_colds);
        for (fix, back, mask) in ymm_colds {
            self.a.bind(fix);
            self.a.movz(false, XA, mask as u32, 0);
            self.a.bl_to(self.clear_ymm_hi);
            let here = self.a.pos;
            self.a.b();
            self.a.patch(here, back);
        }
        self.emit_cold_nan();
        let ses = core::mem::take(&mut self.side_exits);
        for se in &ses {
            if se.fix == usize::MAX {
                continue;
            }
            self.a.bind(se.fix);
            let saved = (self.fs, self.lb, self.pf);
            self.fs = se.fs;
            self.lb = se.lb;
            self.pf = se.pf;
            let need = self.live[se.k];
            self.flush_flags(need);
            self.flush_lb();
            self.xstore(se.xd);
            if self.bud_dec {
                self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * (n - se.k)) as u32, false);
            }
            self.exit_side(self.ops[se.k].ip);
            (self.fs, self.lb, self.pf) = saved;
        }
        let cond_links = core::mem::take(&mut self.cond_links);
        for &(cond_site, target) in &cond_links {
            self.a.bind(cond_site);
            let site = self.a.b();
            self.links.push((site, target));
            self.link_conds.push((site, cond_site));
        }
        let links = core::mem::take(&mut self.links);
        for &(site, target) in &links {
            self.a.bind(site);
            self.a.mov_imm(true, T0, target);
            self.a.b_to(self.lookup_rip);
        }
        self.links = links;
        self.cold_code_end = self.a.pos;
        self.emit_lits();
        main
    }

    /// Store `rip` and exit with `EXIT_SIDE` (interpreter runs that instruction itself).
    fn exit_side(&mut self, rip: u64) {
        self.a.mov_imm(true, T0, rip);
        self.a.str(64, T0, XCPU, OFF_RIP);
        self.a.movz(false, T0, EXIT_SIDE as u32, 0);
        self.a.b_to(self.exit);
    }

    /// Exit with `code`, `cpu.rip = rip`, `jx.target = target`.
    fn exit_code(&mut self, code: u64, rip: u64, target: u64) {
        self.a.mov_imm(true, T0, rip);
        self.a.str(64, T0, XCPU, OFF_RIP);
        self.a.mov_imm(true, T0, target);
        self.a.str(64, T0, XCPU, OFF_TARGET);
        self.a.movz(false, T0, code as u32, 0);
        self.a.b_to(self.exit);
    }

    /// Direct transfer to guest `target` (linked later).
    fn exit_to(&mut self, target: u64) {
        let site = self.a.b();
        self.links.push((site, target));
    }

    /// Dynamic guest call (x0 = target) through `lookup_call`, return address `fall`. With
    /// the RAS, enter it with `bl`: x30 = the landing pad right after it (a transfer to
    /// `fall`), which `lookup_call` pushes on the RAS with `fall`.
    fn call_lookup(&mut self, fall: u64) {
        self.mov_lit(T3, fall);
        if self.ras_miss != 0 {
            self.a.bl_to(self.lookup_call);
            self.exit_to(fall);
        } else {
            self.a.b_to(self.lookup_call);
        }
    }

    /// Translation-time `last_bits` codes of rcx, rdx, r8, r9 (call before `flush_lb`;
    /// `LB_NONE` = the value is in `Cpu::last_bits`).
    fn abi_lb(&self) -> [u8; 4] {
        [self.lb[1], self.lb[2], self.lb[8], self.lb[9]]
    }

    /// Host call (`kind` 0) / tail jump (`kind` 1) to `target` through the `host_xfer` stub.
    /// Flags and last_bits must already be flushed.
    fn xfer_to(&mut self, target: u64, fall: u64, kind: u32) {
        self.xfer_to_src(target, fall, kind, 0);
    }

    /// [`Self::xfer_to`]; `src` != 0: `target` is the value read from guest import slot `src`
    /// (AOT: relocated with the slot, see [`Self::mov_slot_val`]).
    fn xfer_to_src(&mut self, target: u64, fall: u64, kind: u32, src: u64) {
        self.mov_lit(T0, fall);
        self.a.str(64, T0, XCPU, OFF_RIP);
        if src != 0 {
            self.mov_slot_val(T0, target, src);
        } else {
            self.mov_lit(T0, target);
        }
        self.a.movz(true, 2, kind, 0);
        self.a.b_to(self.host_xfer);
    }

    /// Inline call of the host function `h` (`DF_PLAIN` builtin or `inline_variant`: at most
    /// 8 integer args / 4 XMM args, never re-enters guest code) with the register effects of
    /// the interpreter's `host_call` / `host_tail_call`: args = `abi_gpr` rcx/rdx/r8/r9 +
    /// `[rsp+0x20..0x40]` and xmm0..3, rax = x0, xmm0 = q0, last_bits[rax] = full, every
    /// other guest register preserved.
    /// Flags and last_bits must already be flushed (`lbk` from [`Self::abi_lb`] before that).
    /// `tail`: pop the return address first and continue there through `lookup_rip`;
    /// otherwise the caller emits the fall-through transfer.
    /// `bail = Some((target, fall))`: a 0 result means `h` did nothing; then make the real
    /// call/tail jump to guest VA `target` through `host_xfer` (return address `fall`).
    /// `hva`: the guest VA `h` was resolved for ([`inline_host_fn`]; AOT relocation).
    /// `src` != 0: `hva` is the value of guest import slot `src` ([`const_slot_plain`]).
    /// Bitmask of guest XMM registers whose upper 128-bit YMM half is cleared by a VEX-128
    /// op in `ops[..body]`, or `0` if any op in the block can read `Cpu::ymm_hi` (so a
    /// block-entry bulk clear can never clobber a live-in 256-bit YMM register before it is
    /// read, nor before a software side-exit when `!hw_range`).
    fn block_v128_mask(&self, body: usize) -> u16 {
        use fast::*;
        let mut mask = 0u16;
        for op in &self.ops[..body] {
            let mode = op.w & 3;
            let reads_ymm = match op.kind {
                K_YMM_LANE | K_INSN | K_VMASKMOV => true,
                K_VST => (op.w >> 4) >= 5,
                K_MOVMSK => (op.x & 1) != 0,
                K_VCVT_PD => mode == VM_V256 || (op.w & 4) != 0,
                K_VCVT_PH => mode == VM_V256 || ((op.x & 0x1F) != 0 && op.w != 0),
                K_VSHUF_RR | K_VSHUF_RM | K_VINT_RR | K_VINT_RM | K_VFP_RR | K_VFP_RM
                | K_VTEST_RR | K_VTEST_RM | K_VBROADCAST | K_PMOVX | K_VBLENDV_RR
                | K_VBLENDV_RM | K_VFMA_RR | K_VFMA_RM
                | K_VAES_RR | K_VAES_RM | K_VSHA_RR | K_VSHA_RM => mode == VM_V256,
                _ => false,
            };
            if reads_ymm {
                return 0;
            }
            match op.kind {
                K_VSHUF_RR | K_VSHUF_RM | K_VINT_RR | K_VINT_RM | K_VFP_RR | K_VFP_RM
                | K_MOVD_XG | K_CVTSI_R | K_CVTSI_M | K_VBROADCAST | K_PMOVX
                | K_VBLENDV_RR | K_VBLENDV_RM
                    if mode == VM_V128 =>
                {
                    mask |= 1u16 << (op.r0 & 15);
                }
                K_VFMA_RR | K_VFMA_RM if mode != VM_V256 => {
                    mask |= 1u16 << (op.r0 & 15);
                }
                K_VCVT_PD if mode == VM_V128 || ((op.x & 0x1F) >= 2 && mode != VM_LEG) => {
                    mask |= 1u16 << (op.r0 & 15);
                }
                _ => {}
            }
        }
        mask
    }

    /// Backward liveness of guest `xmm0..15` upper bits (`[127:32]` and `[127:64]`): bit `r`
    /// is clear in `xmm_hi32_live[k]` (resp. `xmm_hi64_live[k]`) only if bits `[127:32]` (resp.
    /// `[127:64]`) of `xmm r` are provably overwritten before any instruction in `ops[k..body]`,
    /// side-exit, or block exit can read them.
    fn compute_xmm_hi_live(&mut self, body: usize) {
        use fast::*;
        let hw = hw_range_enabled();
        let mut live32 = 0xFFFFu16;
        let mut live64 = 0xFFFFu16;
        self.xmm_hi32_live[body] = live32;
        self.xmm_hi64_live[body] = live64;
        for k in (0..body).rev() {
            let op = self.ops[k];
            // When software range checks are active, any memory op can side-exit and expose
            // full XMM registers to the interpreter.
            if !hw && (op.base != RNONE || op.index != RNONE || Self::chk_of(&op) == Chk::Full) {
                let is_mem = matches!(
                    op.kind,
                    K_MOV_RM
                        | K_MOV_MR
                        | K_MOV_MI
                        | K_MOVZX_RM
                        | K_MOVSX_RM
                        | K_ALU_RM
                        | K_ALU_MR
                        | K_ALU_MI
                        | K_CMPJ_RM
                        | K_IMUL_RM
                        | K_IMUL_RMI
                        | K_INC_M
                        | K_DEC_M
                        | K_COMI_RM
                        | K_CVTSI_M
                        | K_CVTF2I_M
                        | K_VSHUF_RM
                        | K_VFP_RM
                        | K_VINT_RM
                        | K_VST
                );
                if is_mem {
                    live32 = 0xFFFF;
                    live64 = 0xFFFF;
                    self.xmm_hi32_live[k] = live32;
                    self.xmm_hi64_live[k] = live64;
                    continue;
                }
            }
            let v256 = (op.w & 3) == VM_V256;
            let sub = op.x & 0x3F;
            match xmm_class(&op) {
                XClass::Free => {}
                XClass::Aware if !v256 => match op.kind {
                    K_VFP_RR | K_VFP_RM
                        if matches!(sub, 0..=11 | VF_SQRTSS | VF_SQRTSD | VF_CVTSS2SD | VF_CVTSD2SS) =>
                    {
                        let d = op.r0 & 15;
                        let s1 = (op.imm >> 8) as u8 & 15;
                        let out_64 = matches!(sub, 6..=11 | VF_SQRTSD | VF_CVTSS2SD);
                        let in_b_64 = matches!(sub, 6..=11 | VF_SQRTSD | VF_CVTSD2SS);
                        let in_a_64 = matches!(sub, 6..=11);
                        let d_live64 = (live64 & (1u16 << d)) != 0;
                        let d_live32 = (live32 & (1u16 << d)) != 0;
                        if out_64 {
                            // Writes [63:0] of d, copies [127:64] from s1.
                            live64 &= !(1u16 << d);
                            live32 &= !(1u16 << d);
                            if d_live64 {
                                live64 |= 1u16 << s1;
                                live32 |= 1u16 << s1;
                            }
                        } else {
                            // Writes [31:0] of d, copies [127:32] from s1.
                            live64 &= !(1u16 << d);
                            live32 &= !(1u16 << d);
                            if d_live64 {
                                live64 |= 1u16 << s1;
                                live32 |= 1u16 << s1;
                            } else if d_live32 {
                                live32 |= 1u16 << s1;
                            }
                        }
                        if in_a_64 {
                            live32 |= 1u16 << s1;
                        }
                        if op.kind == K_VFP_RR && in_b_64 {
                            live32 |= 1u16 << (op.r1 & 15);
                        }
                    }
                    K_VFP_RR | K_VFP_RM if matches!(sub, 12..=23) => {
                        let d = op.r0 & 15;
                        let s1 = (op.imm >> 8) as u8 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                        live32 |= 1u16 << s1;
                        live64 |= 1u16 << s1;
                        if op.kind == K_VFP_RR {
                            let s2 = op.r1 & 15;
                            live32 |= 1u16 << s2;
                            live64 |= 1u16 << s2;
                        }
                    }
                    K_VSHUF_RM if matches!((sub, op.w >> 4), (VS_ZX32, 2) | (VS_ZX64, 3) | (VS_MOV, 4)) => {
                        let d = op.r0 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                    }
                    K_VSHUF_RR if sub == VS_ZX32 => {
                        let d = op.r0 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                    }
                    K_VSHUF_RR if sub == VS_ZX64 => {
                        let d = op.r0 & 15;
                        let s2 = op.r1 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                        // Reads bits [63:0] (includes [63:32]) of s2.
                        live32 |= 1u16 << s2;
                    }
                    K_VSHUF_RR if sub == VS_MOV => {
                        let d = op.r0 & 15;
                        let s2 = op.r1 & 15;
                        let dl32 = (live32 & (1u16 << d)) != 0;
                        let dl64 = (live64 & (1u16 << d)) != 0;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                        if dl32 {
                            live32 |= 1u16 << s2;
                        }
                        if dl64 {
                            live64 |= 1u16 << s2;
                        }
                    }
                    K_VSHUF_RR if matches!(sub, VS_MRG32 | VS_MRG64) => {
                        let d = op.r0 & 15;
                        let s1 = (op.imm >> 8) as u8 & 15;
                        let s2 = op.r1 & 15;
                        let dl32 = (live32 & (1u16 << d)) != 0;
                        let dl64 = (live64 & (1u16 << d)) != 0;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                        if sub == VS_MRG64 {
                            if dl64 {
                                live64 |= 1u16 << s1;
                                live32 |= 1u16 << s1;
                            }
                            live32 |= 1u16 << s2;
                        } else {
                            if dl64 {
                                live64 |= 1u16 << s1;
                                live32 |= 1u16 << s1;
                            } else if dl32 {
                                live32 |= 1u16 << s1;
                            }
                        }
                    }
                    K_VSHUF_RR if sub == VS_XOR && ((op.imm >> 8) as u8 & 15) == (op.r1 & 15) => {
                        let d = op.r0 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                    }
                    K_MOVD_XG => {
                        let d = op.r0 & 15;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                    }
                    K_CVTSI_R | K_CVTSI_M => {
                        let d = op.r0 & 15;
                        let s1 = (op.imm >> 8) as u8 & 15;
                        let is_sd = (op.x & 0x3F) != 0;
                        let dl32 = (live32 & (1u16 << d)) != 0;
                        let dl64 = (live64 & (1u16 << d)) != 0;
                        live32 &= !(1u16 << d);
                        live64 &= !(1u16 << d);
                        if is_sd {
                            if dl64 {
                                live64 |= 1u16 << s1;
                                live32 |= 1u16 << s1;
                            }
                        } else if dl64 {
                            live64 |= 1u16 << s1;
                            live32 |= 1u16 << s1;
                        } else if dl32 {
                            live32 |= 1u16 << s1;
                        }
                    }
                    K_COMI_RR | K_COMI_RM => {
                        if (op.x & 0x3F) != 0 {
                            live32 |= 1u16 << (op.r0 & 15);
                            if op.kind == K_COMI_RR {
                                live32 |= 1u16 << (op.r1 & 15);
                            }
                        }
                    }
                    K_CVTF2I_R => {
                        if (op.x & 1) != 0 {
                            live32 |= 1u16 << (op.r1 & 15);
                        }
                    }
                    K_CVTF2I_M => {}
                    K_MOVD_GX => {
                        if op.w == 64 {
                            live32 |= 1u16 << (op.r1 & 15);
                        }
                    }
                    K_VST if matches!(op.w >> 4, 2 | 3) => {
                        if (op.w >> 4) == 3 {
                            live32 |= 1u16 << (op.r1 & 15);
                        }
                    }
                    _ => {
                        live32 = 0xFFFF;
                        live64 = 0xFFFF;
                    }
                },
                _ => {
                    live32 = 0xFFFF;
                    live64 = 0xFFFF;
                }
            }
            self.xmm_hi32_live[k] = live32;
            self.xmm_hi64_live[k] = live64;
        }
    }

    /// Emit a single 1-cycle register-only `tbnz x8, #0, #cold` check calling `clear_ymm_hi`
    /// out-of-line with `mask` when any YMM upper half is dirty. Clobbers nothing on fast path.
    fn emit_ymm_clear_check(&mut self, mask: u16) {
        let fix = self.a.tbnz(XBUD, 0);
        self.ymm_colds.push((fix, self.a.pos, mask));
    }

    /// Store 128-bit vector `v_src` into `Cpu::ymm_hi[reg]` and mark `reg` dirty in
    /// `Cpu::jx.ymm_dirty` and bit 0 of `XBUD`.
    fn write_ymm_hi(&mut self, reg: u8, v_src: u32) {
        let r = reg & 15;
        let bit = 1u16 << r;
        self.a.str(128, v_src, XCPU, OFF_YMM_HI + (r as u32) * 16);
        self.ymm_clean &= !bit;
        self.a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
        self.a.logic_ri(false, 1, XS, XS, enc_logic_imm(bit as u64, false).unwrap());
        self.a.str(16, XS, XCPU, OFF_YMM_DIRTY);
        self.a.logic_ri(true, 1, XBUD, XBUD, enc_logic_imm(1, true).unwrap());
    }

    /// Zero `Cpu::ymm_hi[reg]` for a VEX-128 op (0 instructions when already proven clean,
    /// 1 instruction `stp xzr, xzr` otherwise — `ymm_dirty` stays a conservative superset).
    fn clear_ymm_hi(&mut self, reg: u8) {
        let r = reg & 15;
        let bit = 1u16 << r;
        if self.ymm_clean & bit != 0 {
            return;
        }
        self.a.zero128(XCPU, OFF_YMM_HI + (r as u32) * 16);
        self.ymm_clean |= bit;
    }

    fn plain_call(&mut self, h: usize, hva: u64, _lbk: [u8; 4], tail: bool, bail: Option<(u64, u64)>, src: u64, needs_xmm: bool) {
        if tail {
            self.a.ldr(64, T0, g(4), 0);
            self.a.str(64, T0, XCPU, OFF_RIP);
            self.a.add_imm(true, g(4), g(4), 8);
        }
        // Fast inline `QueryPerformanceCounter`: reads hardware `cntvct_el0` + Q64 scale
        // directly in ~10 instructions with zero XMM/GPR spills and zero `blr`, falling back
        // to the regular `plain_call` if uninitialized / mach fallback / shadow clock freeze.
        let qpc_fn = win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize;
        let rtl_qpc_fn = win32_rt::ntdll::RtlQueryPerformanceCounter as *const () as usize;
        let mut qpc_done = None;
        if bail.is_none() && (h == qpc_fn || h == rtl_qpc_fn) {
            self.mov_qpc_lit(XA);
            self.a.ldp_x(T0, T1, XA, 0); // x0 = mode, x1 = mult
            self.a.addsub_ri(true, true, true, ZR, T0, 1, false); // cmp x0, #1
            let slow = self.a.bcond(NE);
            self.a.mrs_cntvct(T0);
            self.a.ldr(64, T2, XA, 16); // offset
            self.a.umulh(T0, T0, T1);
            self.a.addsub_rr(true, false, false, T0, T0, T2, 0);
            self.a.str(64, T0, g(1), 0); // *lpPerformanceCount = result
            self.a.movz(false, g(0), 1, 0); // return TRUE
            self.set_lb(0, 64);
            self.flush_lb();
            if tail {
                self.a.ldr(64, T0, XCPU, OFF_RIP);
                self.a.b_to(self.lookup_rip);
            } else {
                qpc_done = Some(self.a.b());
            }
            self.a.bind(slow);
        }
        if self.xpin {
            self.a.bl_to(self.spill_xmm);
        }
        if self.ras_miss != 0 {
            self.a.str(64, XRAS, XCPU, OFF_RAS);
        }
        // `DF_PLAIN` builtins are real Win32/CRT `extern "win64"` functions (never `DF_GAME_REPL`),
        // so pass full 64-bit guest GPRs matching real x86-64 hardware and `xfer_plain`.
        for (j, gi) in [1u8, 2, 8, 9].into_iter().enumerate() {
            self.a.mov_rr(true, j as u32, g(gi));
        }
        self.a.ldp_x(4, 5, g(4), 0x20);
        self.a.ldp_x(6, 7, g(4), 0x30);
        // q0..q3 = xmm0..3 only when the callee takes/returns float values (`inline_variant`).
        if needs_xmm {
            for q in 0..4 {
                self.a.ldr(128, q, XCPU, OFF_XMM + 16 * q);
            }
        }
        // Save the caller-saved pinned registers (r10..r15, Cpu pointer, budget) and guest FPCR.
        self.a.addsub_ri(true, true, false, 31, 31, 80, false); // sub sp, sp, #80
        self.a.stp_x(8, 9, 31, 0);
        self.a.stp_x(10, 11, 31, 16);
        self.a.stp_x(12, 13, 31, 32);
        self.a.stp_x(14, 15, 31, 48);
        self.mov_host_lit(XA, h, hva, bail.is_some(), src);
        self.a.mrs_fpcr(XS);
        self.a.str(64, XS, 31, 64);
        let fpcr_zero = self.a.cbz(true, XS);
        self.a.msr_fpcr(ZR);
        self.a.bind(fpcr_zero);
        self.a.blr(XA);
        self.a.ldr(64, XS, 31, 64);
        let fpcr_keep = self.a.cbz(true, XS);
        self.a.msr_fpcr(XS);
        self.a.bind(fpcr_keep);
        self.a.ldp_x(8, 9, 31, 0);
        self.a.ldp_x(10, 11, 31, 16);
        self.a.ldp_x(12, 13, 31, 32);
        self.a.ldp_x(14, 15, 31, 48);
        self.a.addsub_ri(true, false, false, 31, 31, 80, false); // add sp, sp, #80
        if self.xpin {
            self.a.bl_to(self.reload_xmm);
        }
        if self.ras_miss != 0 {
            self.a.ldr(64, XRAS, XCPU, OFF_RAS);
        }
        if let Some((target, fall)) = bail {
            let ok = self.a.cbnz(true, 0);
            if tail {
                self.a.addsub_ri(true, true, false, g(4), g(4), 8, false); // undo the pop
            }
            self.xfer_to_src(target, fall, tail as u32, src);
            self.a.bind(ok);
        }
        self.a.mov_rr(true, g(0), 0);
        if self.xpin {
            self.a.v_mov(16, 0);
        } else {
            self.a.str(128, 0, XCPU, OFF_XMM);
        }
        self.set_lb(0, 64);
        self.flush_lb();
        if tail {
            self.a.ldr(64, T0, XCPU, OFF_RIP);
            self.a.b_to(self.lookup_rip);
        }
        if let Some(d) = qpc_done {
            self.a.bind(d);
        }
    }

    fn side_exit_here(&mut self, k: usize) -> usize {
        self.side_exits.push(SideExit { fix: usize::MAX, k, fs: self.fs, lb: self.lb, xd: self.xd, pf: self.pf });
        self.side_exits.len() - 1
    }

    /// `if !(0x1_0000 <= a < 2^47) goto side_exit[se]`.
    fn range_check_to(&mut self, a: u32, se: usize) {
        if hw_range_enabled() {
            return;
        }
        self.a.addsub_ri(true, true, false, XS, a, 0x10, true);
        self.a.lsr_i(true, XS, XS, 47);
        let f = self.a.cbnz(true, XS);
        self.bind_side_exit(se, f);
    }

    /// Make branch `f` (to be bound out of line) take side exit `se`.
    fn bind_side_exit(&mut self, se: usize, f: usize) {
        // Chain multiple branches to the same side exit through per-site stubs.
        let s = &self.side_exits[se];
        let (k, fs, lb, xd, pf) = (s.k, s.fs, s.lb, s.xd, s.pf);
        if s.fix == usize::MAX {
            self.side_exits[se].fix = f;
        } else {
            self.side_exits.push(SideExit { fix: f, k, fs, lb, xd, pf });
        }
    }

    /// Range check of op `k`'s own memory operand `a` outside [`Self::mem_access`] (ops that
    /// only validate the address): as the lead of a base-register group it covers the group.
    fn range_check_op(&mut self, k: usize, a: u32, se: usize) {
        if hw_range_enabled() {
            return;
        }
        if self.grp_k != k {
            if let Grp::Lead { lo, id } = self.grp[k] {
                self.grp_k = k;
                if lo != 0 {
                    self.a.addsub_ri(true, true, false, XS, a, (-(lo as i32)) as u32, false);
                    self.range_check_to(XS, se);
                } else {
                    self.range_check_to(a, se);
                }
                self.grp_ok |= 1u64 << id;
                return;
            }
            if let Grp::Member { id } = self.grp[k] {
                if self.grp_ok & (1u64 << id) != 0 {
                    // Range already covered by the group's lead.
                    self.grp_k = k;
                    return;
                }
            }
        }
        self.range_check_to(a, se);
    }

    /// `AW2_JIT_MEMCHK` bit 2: group the body's `[base+disp]` accesses (no index, no GS,
    /// base not rsp, |disp| < 4096) by base register and base value (no write to the base in
    /// between, from the iced write sets), at most 4 KiB apart. Groups of two or more get a
    /// `Lead` (first access, range-checks the lowest address) and `Member`s.
    /// Only ops whose first range-checked access is unconditional and is their own memory
    /// operand take part (the lead's check must dominate every member).
    fn plan_groups(&mut self) {
        use fast::*;
        if memchk_mask() & 4 == 0 {
            return;
        }
        // (base, base version, op, disp) in program order.
        let mut acc: Vec<(u8, u16, usize, i64)> = Vec::new();
        let mut ver = [0u16; 16];
        for k in 0..self.ops.len() {
            let op = self.ops[k];
            let kind_ok = if k < self.body {
                matches!(
                    op.kind,
                    K_MOV_RM
                        | K_MOV_MR
                        | K_MOV_MI
                        | K_MOVZX_RM
                        | K_MOVSX_RM
                        | K_ALU_RM
                        | K_ALU_MR
                        | K_ALU_MI
                        | K_CMPJ_RM
                        | K_IMUL_RM
                        | K_IMUL_RMI
                        | K_INC_M
                        | K_DEC_M
                        | K_COMI_RM
                        | K_CVTSI_M
                        | K_CVTF2I_M
                        | K_VSHUF_RM
                        | K_VFP_RM
                        | K_VINT_RM
                        | K_VST
                )
            } else {
                matches!(op.kind, K_CALL_M | K_JMP_M)
            };
            let b = op.base;
            if kind_ok && b < RNONE && b != 4 && op.index == RNONE && (-4095..=4095).contains(&op.disp) {
                acc.push((b, ver[b as usize], k, op.disp));
            }
            let w = self.wr[k];
            for (r, v) in ver.iter_mut().enumerate() {
                if w & (1 << r) != 0 {
                    *v = v.wrapping_add(1);
                }
            }
        }
        let mut id = 0u8;
        let mut done = vec![false; acc.len()];
        for i in 0..acc.len() {
            if done[i] || id >= 64 {
                continue;
            }
            let (b, v, _, _) = acc[i];
            // Members: same base value, greedily while the span stays within 4 KiB (with
            // 32 bytes for the widest 256-bit vector access).
            let mut mem = vec![i];
            let (mut lo, mut hi) = (acc[i].3, acc[i].3);
            for j in i + 1..acc.len() {
                if done[j] || acc[j].0 != b || acc[j].1 != v {
                    continue;
                }
                let (nl, nh) = (lo.min(acc[j].3), hi.max(acc[j].3));
                if nh + 32 - nl > 0x1000 {
                    continue;
                }
                (lo, hi) = (nl, nh);
                mem.push(j);
            }
            if mem.len() < 2 {
                continue;
            }
            for (n, &j) in mem.iter().enumerate() {
                done[j] = true;
                let k = acc[j].2;
                self.grp[k] = if n == 0 { Grp::Lead { lo: (lo - acc[j].3) as i16, id } } else { Grp::Member { id } };
            }
            id += 1;
        }
    }

    /// After op `k`: drop the "rsp 8-aligned" fact unless rsp was left alone or moved by a
    /// multiple of 8 (`and rsp, -8k` establishes it).
    fn track_rsp(&mut self, k: usize) {
        use fast::*;
        if self.wr[k] & (1 << 4) == 0 {
            return;
        }
        let op = self.ops[k];
        let keep = match op.kind {
            K_PUSH_R | K_POP_R => op.r0 != 4,
            K_PUSH_I | K_PUSH_M | K_POP_M => true,
            K_ALU_RI if op.r0 == 4 && op.w == 64 => match op.x & 0x3F {
                A_ADD | A_SUB => op.imm % 8 == 0,
                A_AND if op.imm & 7 == 0 => {
                    self.rsp_al = true;
                    true
                }
                _ => false,
            },
            K_LEA => op.r0 == 4 && op.w == 64 && op.base == 4 && op.index == RNONE && op.disp % 8 == 0,
            _ => false,
        };
        if !keep {
            self.rsp_al = false;
        }
    }

    /// `AW2_JIT_MEMCHK` bit 3: displacement for `ldapur`/`stlur [x23, #disp]` if op `k`'s
    /// `[rsp+disp]` operand (scalar, TSO stack accesses) qualifies: `disp` fits 9 bits and
    /// `disp % 8 + size <= 8`, so with rsp 8-aligned the access stays inside one 16-byte
    /// granule (FEAT_LSE2: no alignment fault, single-copy atomic). rsp alignment is tested
    /// once (side exit at op `k`) until rsp is rewritten.
    fn rsp_slot(&mut self, k: usize, op: &FastOp, bits: u32) -> Option<i32> {
        if memchk_mask() & 8 == 0 || !mem::tso() || !stack_tso() || op.base != 4 || op.index != RNONE || bits > 64 {
            return None;
        }
        let d = op.disp;
        if !(-256..=255).contains(&d) || (d & 7) + (bits / 8) as i64 > 8 {
            return None;
        }
        if bits > 8 && !self.rsp_al {
            let se = self.side_exit_here(k);
            self.a.logic_ri(true, 0, XS, g(4), enc_logic_imm(7, true).unwrap());
            let f = self.a.cbnz(true, XS);
            self.bind_side_exit(se, f);
            self.rsp_al = true;
        }
        Some(d as i32)
    }

    /// `AW2_JIT_BUDGET_BACKEDGE`: whether the block tests the budget before its terminator.
    /// Every cycle of translated code passes a backward direct transfer (target <= block
    /// start: forward-only chains are finite), a RAS `ret` or a lookup stub (which tests).
    fn term_tests_budget(&self, term: &Term) -> bool {
        let ip0 = self.ops[0].ip;
        match *term {
            Term::Fall(_) => false,
            Term::Jcc { target, .. } | Term::Loop { target, .. } => target <= ip0,
            Term::JccPair { t1, t2, .. } => t1 <= ip0 || t2 <= ip0,
            Term::Jmp(t) | Term::Call(t) => !self.target_host && t <= ip0,
            Term::Ret => self.ras_miss != 0 && !budget_light_enabled(),
            Term::CallR | Term::CallM | Term::JmpR | Term::JmpM => false,
        }
    }

    // --- memory ---

    /// Effective address of `op`'s memory operand into `dst` (or an existing register).
    fn addr_into(&mut self, op: &FastOp, dst: u32) -> u32 {
        if dst == XA {
            self.xa_key = None;
        }
        let s = (op.x >> 6) as u32;
        let b_reg = op.base & 0x1F;
        let gs = (op.base & fast::B_GS) != 0;
        let a32 = (op.base & fast::B_ADDR32) != 0;
        let x = !a32;
        let disp = if a32 { op.disp as i32 as i64 } else { op.disp };
        let hb = b_reg != RNONE;
        let hi = op.index != RNONE;
        let r = if hb && hi {
            self.a.addsub_rr(x, false, false, dst, g(b_reg), g(op.index), s);
            if disp != 0 {
                self.a.add_imm(x, dst, dst, disp);
            }
            dst
        } else if hb {
            if disp == 0 && !a32 {
                if !gs {
                    return g(b_reg);
                }
                g(b_reg)
            } else if disp == 0 {
                self.a.mov_rr(false, dst, g(b_reg));
                dst
            } else {
                self.a.add_imm(x, dst, g(b_reg), disp);
                dst
            }
        } else if hi {
            if s == 0 {
                if disp == 0 && !a32 {
                    if !gs {
                        return g(op.index);
                    }
                    g(op.index)
                } else if disp == 0 {
                    self.a.mov_rr(false, dst, g(op.index));
                    dst
                } else {
                    self.a.add_imm(x, dst, g(op.index), disp);
                    dst
                }
            } else {
                self.a.lsl_i(x, dst, g(op.index), s);
                if disp != 0 {
                    self.a.add_imm(x, dst, dst, disp);
                }
                dst
            }
        } else {
            self.a.mov_imm(x, dst, if a32 { (disp as u32) as u64 } else { disp as u64 });
            dst
        };
        if gs {
            self.a.ldr(64, XS, XCPU, OFF_TEB);
            self.a.addsub_rr(true, false, false, dst, r, XS, 0);
            return dst;
        }
        r
    }

    fn chk_of(op: &FastOp) -> Chk {
        if op.base == 4 && op.index == RNONE {
            Chk::Stack
        } else if op.base == RNONE
            && op.index == RNONE
            && ro_plain_enabled()
            && in_ro_data(op.ip, op.disp as u64, 32)
        {
            // (A GS prefix sets a flag bit in `base`, so `base == RNONE` excludes it.)
            Chk::Ro
        } else if op.base == RNONE
            && op.index == RNONE
            && memchk_mask() & 2 != 0
            && (0x1_0000..0x8000_0000_0000).contains(&(op.disp as u64))
        {
            // Constant address known valid at translation time (out-of-range constants keep
            // the run-time check, which always side-exits).
            Chk::Abs(op.disp & 7 == 0)
        } else {
            Chk::Full
        }
    }

    /// Guest load of `bits` at host register `a` into `dst` (zero-extended).
    fn load(&mut self, k: usize, dst: u32, a: u32, bits: u32, chk: Chk) {
        self.mem_access(k, true, dst, a, bits, chk);
    }

    fn store(&mut self, k: usize, src: u32, a: u32, bits: u32, chk: Chk) {
        self.mem_access(k, false, src, a, bits, chk);
    }

    /// Guest load of `bits` from `op`'s memory operand into `dst`, using direct `[base, #disp]`
    /// addressing (`ldapur`/`ldr`/`ldur`) when possible instead of `add x16, base, #disp`.
    fn load_op(&mut self, k: usize, dst: u32, op: &FastOp, bits: u32) {
        self.mem_access_op(k, true, dst, op, bits);
    }

    /// Guest store of `bits` from `src` to `op`'s memory operand, using direct `[base, #disp]`
    /// addressing (`stlur`/`str`/`stur`) when possible.
    fn store_op(&mut self, k: usize, src: u32, op: &FastOp, bits: u32) {
        self.mem_access_op(k, false, src, op, bits);
    }

    fn mem_access_op(&mut self, k: usize, ld: bool, reg: u32, op: &FastOp, bits: u32) {
        // Fast thread-private TEB access (`gs:[disp]`, e.g. `mov rax, gs:[0x58]` for TLS):
        // 2 plain instructions under `hw_range_enabled()` (or 3 + null check otherwise).
        if op.base == (RNONE | fast::B_GS)
            && op.index == RNONE
            && bits <= 64
            && op.disp >= 0
            && op.disp < 4096
            && ((op.disp as u64) & ((bits / 8) as u64 - 1)) == 0
        {
            self.xa_key = None;
            if !hw_range_enabled() {
                let se = self.side_exit_here(k);
                self.a.ldr(64, XA, XCPU, OFF_TEB);
                let f = self.a.cbz(true, XA);
                self.bind_side_exit(se, f);
            } else {
                self.a.ldr(64, XA, XCPU, OFF_TEB);
            }
            self.a.ldst_ui(ld, bits, reg, XA, op.disp as u32);
            return;
        }
        if op.base < RNONE && op.index == RNONE && bits <= 64 {
            let b = g(op.base);
            let d = op.disp;
            let mut chk = Self::chk_of(op);
            let tso = mem::tso() && (chk != Chk::Stack || stack_tso()) && !(chk == Chk::Ro && ld);
            let lse2 = lse2_trap_enabled();
            let bytes = (bits / 8) as u64;
            let use_plain_ui = !tso && d >= 0 && ((d as u64) & (bytes - 1)) == 0 && ((d as u64) / bytes) < 4096;
            let use_plain_ur = !tso && !use_plain_ui && (-256..=255).contains(&d);
            let use_tso_ur = tso && (lse2 || bits == 8) && (-256..=255).contains(&d);
            if use_plain_ui || use_plain_ur || use_tso_ur {
                let mut lead_lo = 0i32;
                let mut lead = None;
                if chk == Chk::Full && self.grp_k != k {
                    match self.grp[k] {
                        Grp::Lead { lo, id } => {
                            self.grp_k = k;
                            lead_lo = lo as i32;
                            lead = Some(id);
                        }
                        Grp::Member { id } if self.grp_ok & (1u64 << id) != 0 => {
                            self.grp_k = k;
                            chk = Chk::Align;
                        }
                        _ => {}
                    }
                }
                if chk == Chk::Full && !hw_range_enabled() {
                    let se = self.side_exit_here(k);
                    let min_d = d + lead_lo as i64;
                    if (0..4096).contains(&min_d) && d < 4096 {
                        self.range_check_to(b, se);
                    } else if min_d < 0 && -min_d < 4096 {
                        self.a.addsub_ri(true, true, false, XS, b, (-min_d) as u32, false);
                        self.range_check_to(XS, se);
                    } else {
                        self.a.add_imm(true, XS, b, min_d);
                        self.range_check_to(XS, se);
                    }
                    if let Some(id) = lead {
                        self.grp_ok |= 1u64 << id;
                    }
                }
                if use_tso_ur {
                    self.a.ldapur_stlur(ld, bits, reg, b, d as i32);
                } else if use_plain_ui {
                    self.a.ldst_ui(ld, bits, reg, b, d as u32);
                } else {
                    self.a.ldur_stur(ld, bits, reg, b, d as i32);
                }
                return;
            }
            // Large displacement `[base + disp]` with `hw_range_enabled()` + `ldapur/stlur`:
            // split `disp = hi_4k + lo_9` when `lo_9` fits signed 9 bits and `hi_4k` fits
            // `imm12 << 12`, caching `(base, RNONE, 0, hi_4k)` in `XA` for nearby fields.
            if hw_range_enabled() && (!tso || lse2 || bits == 8) {
                let hi = (d.wrapping_add(256)) & !0xFFF;
                let lo = d.wrapping_sub(hi);
                if hi != 0 && (-256..=255).contains(&lo) && (-0xFF_F000..=0xFF_F000).contains(&hi) {
                    let key = (op.base, RNONE, 0u8, hi);
                    if self.xa_key != Some(key) {
                        self.a.addsub_ri(true, hi < 0, false, XA, b, (hi.unsigned_abs() >> 12) as u32, true);
                        self.xa_key = Some(key);
                    }
                    if tso {
                        self.a.ldapur_stlur(ld, bits, reg, XA, lo as i32);
                    } else {
                        self.a.ldur_stur(ld, bits, reg, XA, lo as i32);
                    }
                    return;
                }
            }
            if !lse2 {
                if let Some(slot_d) = self.rsp_slot(k, op, bits) {
                    self.a.ldapur_stlur(ld, bits, reg, g(4), slot_d);
                    return;
                }
            }
        }
        // Indexed `[base + idx*scale + disp]` under `hw_range_enabled()`: compute/reuse
        // `XA = base + (idx << scale) + hi_4k` and emit direct `ldapur/stlur/ldur/stur [XA, #lo]`,
        // eliminating redundant index arithmetic across vec3 / struct-array element loads.
        if hw_range_enabled() && op.base < RNONE && op.index < RNONE && bits <= 64 {
            let chk = Self::chk_of(op);
            let tso = mem::tso() && (chk != Chk::Stack || stack_tso()) && !(chk == Chk::Ro && ld);
            let lse2 = lse2_trap_enabled();
            if !tso || lse2 || bits == 8 {
                let scale = op.x >> 6;
                let d = op.disp;
                let hi = (d.wrapping_add(256)) & !0xFFF;
                let lo = d.wrapping_sub(hi);
                if (-256..=255).contains(&lo) {
                    let key = (op.base, op.index, scale, hi);
                    if self.xa_key != Some(key) {
                        self.a.addsub_rr(true, false, false, XA, g(op.base), g(op.index), scale as u32);
                        if hi != 0 {
                            self.a.add_imm(true, XA, XA, hi);
                        }
                        self.xa_key = Some(key);
                    }
                    if tso {
                        self.a.ldapur_stlur(ld, bits, reg, XA, lo as i32);
                    } else {
                        self.a.ldur_stur(ld, bits, reg, XA, lo as i32);
                    }
                    return;
                }
            }
        }
        let a = self.addr_into(op, XA);
        self.mem_access(k, ld, reg, a, bits, Self::chk_of(op));
    }

    fn mem_access(&mut self, k: usize, ld: bool, reg: u32, a: u32, bits: u32, chk: Chk) {
        let lg = match bits {
            8 => 0,
            16 => 1,
            32 => 2,
            _ => 3,
        };
        let mut chk = chk;
        // Base-register group role of op `k`'s (first) range-checked access.
        let mut lo = 0i32;
        let mut lead = None;
        if chk == Chk::Full && self.grp_k != k {
            match self.grp[k] {
                Grp::Lead { lo: l, id } => {
                    self.grp_k = k;
                    lo = l as i32;
                    lead = Some(id);
                }
                Grp::Member { id } if self.grp_ok & (1u64 << id) != 0 => {
                    self.grp_k = k;
                    chk = Chk::Align;
                }
                _ => {}
            }
        }
        let tso = mem::tso() && (chk != Chk::Stack || stack_tso()) && !(chk == Chk::Ro && ld);
        let lse2 = lse2_trap_enabled();
        let mut fix = None;
        let mut exit = None;
        let mut slow_lo = 0;
        let amask = (bits / 8 - 1) as u64;
        match chk {
            Chk::Full if hw_range_enabled() => {
                if bits > 8 && tso && !lse2 {
                    self.a.logic_ri(true, 0, XS, a, enc_logic_imm(amask, true).unwrap());
                    fix = Some(self.a.cbnz(true, XS));
                }
            }
            Chk::Full => {
                let se = self.side_exit_here(k);
                let combined = memchk_mask() & 1 != 0;
                if lo != 0 {
                    // Group lead: range-check the group's lowest address a + lo.
                    self.a.addsub_ri(true, true, false, XS, a, (-lo) as u32, false);
                    if bits == 8 || !tso || lse2 || !combined || (-lo) as u64 & amask != 0 {
                        self.range_check_to(XS, se);
                        if bits > 8 && tso && !lse2 {
                            self.a.logic_ri(true, 0, XS, a, enc_logic_imm(amask, true).unwrap());
                            fix = Some(self.a.cbnz(true, XS));
                        }
                    } else {
                        // a + lo has the alignment of a: one combined test.
                        self.a.addsub_ri(true, true, false, XS, XS, 0x10, true);
                        self.a.logic_ri(true, 0, XS, XS, enc_logic_imm(0xFFFF_8000_0000_0000 | amask, true).unwrap());
                        fix = Some(self.a.cbnz(true, XS));
                        exit = Some(se);
                        slow_lo = lo;
                    }
                } else if bits == 8 || !tso || lse2 {
                    self.range_check_to(a, se);
                } else if combined {
                    // (a - 0x10000) & (bits 47..63 | alignment bits): same test as below.
                    self.a.addsub_ri(true, true, false, XS, a, 0x10, true);
                    self.a.logic_ri(true, 0, XS, XS, enc_logic_imm(0xFFFF_8000_0000_0000 | amask, true).unwrap());
                    fix = Some(self.a.cbnz(true, XS));
                    exit = Some(se);
                } else {
                    self.a.addsub_ri(true, true, false, XS, a, 0x10, true);
                    self.a.ror_i(true, XS, XS, lg);
                    self.a.lsr_i(true, XS, XS, 47 - lg);
                    fix = Some(self.a.cbnz(true, XS));
                    exit = Some(se);
                }
                if let Some(id) = lead {
                    self.grp_ok |= 1u64 << id;
                }
            }
            Chk::Abs(true) => {}
            Chk::Align | Chk::Stack | Chk::Ro | Chk::Abs(false) => {
                if bits > 8 && tso && !lse2 {
                    self.a.logic_ri(true, 0, XS, a, enc_logic_imm(amask, true).unwrap());
                    fix = Some(self.a.cbnz(true, XS));
                }
            }
        }
        if tso {
            if ld {
                self.a.ldapr(bits, reg, a);
            } else {
                self.a.stlr(bits, reg, a);
            }
        } else if ld {
            self.a.ldr(bits, reg, a, 0);
        } else {
            self.a.str(bits, reg, a, 0);
        }
        if let Some(f) = fix {
            let back = self.a.pos;
            self.slow.push(SlowMem { fix: f, back, ld, reg, addr: a, bits, exit, lo: slow_lo });
        }
    }

    /// Store with only an alignment check (address already range-checked in this op).
    fn store_checked_addr(&mut self, src: u32, a: u32, bits: u32) {
        let tso = mem::tso();
        if tso && bits > 8 && !lse2_trap_enabled() {
            self.a.logic_ri(true, 0, XS, a, enc_logic_imm((bits / 8 - 1) as u64, true).unwrap());
            let f = self.a.cbnz(true, XS);
            self.a.stlr(bits, src, a);
            let back = self.a.pos;
            self.slow.push(SlowMem { fix: f, back, ld: false, reg: src, addr: a, bits, exit: None, lo: 0 });
        } else if tso {
            self.a.stlr(bits, src, a);
        } else {
            self.a.str(bits, src, a, 0);
        }
    }

    // --- flags ---

    fn set_lb(&mut self, i: u8, w: u8) {
        if self.lb_on && tracked(i) {
            self.lb[i as usize] = match w {
                8 => 8,
                16 => 16,
                _ => 0,
            };
        }
    }

    fn flush_lb_reg(&mut self, i: usize) {
        if !self.lb_on {
            return;
        }
        let v = self.lb[i];
        if v == LB_NONE {
            return;
        }
        if v == 0 {
            self.a.str(8, ZR, XCPU, OFF_LB + i as u32);
        } else {
            self.a.movz(false, XS, v as u32, 0);
            self.a.str(8, XS, XCPU, OFF_LB + i as u32);
        }
        self.lb[i] = LB_NONE;
    }

    fn flush_lb(&mut self) {
        if !self.lb_on {
            return;
        }
        if self.lazy {
            // Merged zero stores (AW2_JIT_FLAGLAZY).
            self.flush_lb_edge(0);
            return;
        }
        for i in TRACKED {
            self.flush_lb_reg(i as usize);
        }
    }

    /// Store `code` into `last_bits[i]` right now (inside conditional code).
    fn lb_store_now(&mut self, i: u8, w: u8) {
        if !self.lb_on || !tracked(i) {
            return;
        }
        let v = match w {
            8 => 8,
            16 => 16,
            _ => 0,
        };
        if v == 0 {
            self.a.str(8, ZR, XCPU, OFF_LB + i as u32);
        } else {
            self.a.movz(false, XS, v, 0);
            self.a.str(8, XS, XCPU, OFF_LB + i as u32);
        }
    }

    /// Materialize flag `f` (from NZCV) as 0/1 into `r`; requires `f` in `fs.host`.
    fn host_flag(&mut self, f: u8, r: u32) {
        match (f, self.fs.src) {
            (FC, Src::Sub) => self.a.cset(r, LO),
            (FC, Src::Add) => self.a.cset(r, HS),
            (FC, Src::Fcmp) => self.a.cset(r, LT),
            (FZ, Src::Fcmp) => {
                // ZF = (Z == 1 || V == 1); Z and V are mutually exclusive after fcmp.
                self.a.cset(r, EQ);
                self.a.csinc(false, r, r, r, VC);
            }
            (FS | FO, Src::Fcmp) => self.a.movz(false, r, 0, 0),
            (FC, _) | (FO, Src::Logic) => self.a.movz(false, r, 0, 0),
            (FZ, _) => self.a.cset(r, EQ),
            (FS, _) => self.a.cset(r, MI),
            _ => self.a.cset(r, VS),
        }
    }

    fn flag_off(f: u8) -> u32 {
        match f {
            FC => OFF_CF,
            FZ => OFF_ZF,
            FS => OFF_SF,
            _ => OFF_OF,
        }
    }

    /// Write flags in `need` that are only in NZCV to memory.
    fn flush_flags(&mut self, need: u8) {
        if need & FP != 0 && self.pf != PF_NONE {
            self.flush_pf();
        }
        if self.fs.src == Src::Fcmp && (need & self.fs.host & !self.fs.mem & FP) != 0 {
            self.a.cset(XS, VC);
            self.a.str(8, XS, XCPU, OFF_PFV);
            self.fs.mem |= FP;
        }
        let todo = need & self.fs.host & !self.fs.mem & FNZCV;
        for f in [FC, FZ, FS, FO] {
            if todo & f != 0 {
                if f == FC && self.fs.src == Src::IncDec {
                    continue;
                }
                let const_zero = (self.fs.src == Src::Logic && (f == FC || f == FO))
                    || (self.fs.src == Src::Fcmp && (f == FS || f == FO));
                if const_zero {
                    self.a.str(8, ZR, XCPU, Self::flag_off(f));
                } else {
                    self.host_flag(f, XS);
                    self.a.str(8, XS, XCPU, Self::flag_off(f));
                }
                self.fs.mem |= f;
            }
        }
    }

    /// Flag `f` as 0/1 into `r` (from NZCV if there, else memory).
    fn flag_reg(&mut self, f: u8, r: u32) {
        if f == FP {
            if self.fs.src == Src::Fcmp && self.fs.host & FP != 0 {
                self.a.cset(r, VS);
                return;
            }
            if self.pf != PF_NONE {
                self.flush_pf();
            }
            self.a.ldr(8, r, XCPU, OFF_PFV);
            self.a.logic_rr(false, 2, r, r, r, 1, 4);
            self.a.logic_rr(false, 2, r, r, r, 1, 2);
            self.a.logic_rr(false, 2, r, r, r, 1, 1);
            self.a.logic_ri(false, 0, r, r, enc_logic_imm(1, false).unwrap());
            self.a.logic_ri(false, 2, r, r, enc_logic_imm(1, false).unwrap());
        } else if self.fs.host & f != 0 && !(f == FC && self.fs.src == Src::IncDec) {
            self.host_flag(f, r);
        } else {
            self.a.ldr(8, r, XCPU, Self::flag_off(f));
        }
    }

    fn cond(&mut self, cc: u8) -> Cond {
        let need = cc_reads(cc);
        if self.fs.src == Src::Fcmp && need & !self.fs.host == 0 {
            let m = match cc {
                1 | 9 | 13 => Some(Cond::Const(false)), // jo, js, jl (SF = OF = 0)
                2 | 10 | 14 => Some(Cond::Const(true)), // jno, jns, jge
                3 => Some(Cond::Host(LT)),              // jb (a < b || unordered)
                4 => Some(Cond::Host(GE)),              // jae (a >= b, ordered)
                7 => Some(Cond::Host(LE)),              // jbe (a <= b || unordered)
                8 => Some(Cond::Host(GT)),              // ja (a > b, ordered)
                11 => Some(Cond::Host(VS)),             // jp (unordered)
                12 => Some(Cond::Host(VC)),             // jnp (ordered)
                _ => None,                              // je, jne, jle, jg -> flag_reg(FZ)
            };
            if let Some(m) = m {
                return m;
            }
        }
        if need & FP == 0 && need & !self.fs.host == 0 {
            const N: u32 = u32::MAX;
            const SUBMAP: [u32; 17] = [N, VS, VC, LO, HS, EQ, NE, LS, HI, MI, PL, N, N, LT, GE, LE, GT];
            let m = match self.fs.src {
                Src::None | Src::Fcmp => None,
                Src::Sub => Some(Cond::Host(SUBMAP[cc as usize])),
                Src::Add => match cc {
                    3 => Some(Cond::Host(HS)),
                    4 => Some(Cond::Host(LO)),
                    7 | 8 => None,
                    _ => Some(Cond::Host(SUBMAP[cc as usize])),
                },
                Src::Logic => match cc {
                    1 | 3 => Some(Cond::Const(false)),
                    2 | 4 => Some(Cond::Const(true)),
                    7 => Some(Cond::Host(EQ)),
                    8 => Some(Cond::Host(NE)),
                    13 => Some(Cond::Host(MI)),
                    14 => Some(Cond::Host(PL)),
                    _ => Some(Cond::Host(SUBMAP[cc as usize])),
                },
                Src::IncDec => match cc {
                    3 | 4 | 7 | 8 => None,
                    _ => Some(Cond::Host(SUBMAP[cc as usize])),
                },
            };
            if let Some(m) = m {
                return m;
            }
        }
        let pos = if cc % 2 == 1 { cc } else { cc - 1 };
        let inv = cc % 2 == 0;
        match pos {
            1 => self.flag_reg(FO, T5),
            3 => self.flag_reg(FC, T5),
            5 => self.flag_reg(FZ, T5),
            7 => {
                self.flag_reg(FC, T5);
                self.flag_reg(FZ, T6);
                self.a.logic_rr(false, 1, T5, T5, T6, 0, 0);
            }
            9 => self.flag_reg(FS, T5),
            11 => self.flag_reg(FP, T5),
            13 => {
                self.flag_reg(FS, T5);
                self.flag_reg(FO, T6);
                self.a.logic_rr(false, 2, T5, T5, T6, 0, 0);
            }
            _ => {
                self.flag_reg(FS, T5);
                self.flag_reg(FO, T6);
                self.a.logic_rr(false, 2, T5, T5, T6, 0, 0);
                self.flag_reg(FZ, T6);
                self.a.logic_rr(false, 1, T5, T5, T6, 0, 0);
            }
        }
        Cond::Reg(T5, inv)
    }

    /// Branch taken when the condition is FALSE; returns the fixup (None if never taken).
    fn branch_if_false(&mut self, c: &Cond) -> Option<usize> {
        match *c {
            Cond::Host(cnd) => Some(self.a.bcond(cnd ^ 1)),
            Cond::Reg(r, inv) => Some(if inv { self.a.cbnz(false, r) } else { self.a.cbz(false, r) }),
            Cond::Const(true) => None,
            Cond::Const(false) => Some(self.a.b()),
        }
    }

    // --- register helpers ---

    /// Write `src` (value in its low `w` bits) to guest register `i` with x86 width semantics.
    fn wr_gpr(&mut self, i: u8, w: u8, src: u32) {
        let d = g(i);
        match w {
            64 => {
                if d != src {
                    self.a.mov_rr(true, d, src);
                }
            }
            32 => self.a.mov_rr(false, d, src),
            _ => self.a.bfxil0(d, src, w as u32),
        }
        self.set_lb(i, w);
    }

    // --- ops ---

    fn op(&mut self, k: usize) {
        use fast::*;
        let op = self.ops[k];
        if self.xa_key.is_some()
            && !matches!(
                op.kind,
                K_NOP
                    | K_MOV_RR
                    | K_MOV_RI
                    | K_MOV_RM
                    | K_MOV_MR
                    | K_MOV_MI
                    | K_MOVZX_RR
                    | K_MOVSX_RR
                    | K_MOVZX_RM
                    | K_MOVSX_RM
                    | K_VSHUF_RR
                    | K_VSHUF_RM
                    | K_VFP_RR
                    | K_VFP_RM
                    | K_VST
                    | K_COMI_RR
                    | K_COMI_RM
            )
        {
            self.xa_key = None;
        }
        if self.xon && self.xmm_op(k, &op) {
            return;
        }
        let w = op.w;
        match op.kind {
            K_NOP => {}
            K_MOV_RR => {
                if w == 32 || op.r0 != op.r1 {
                    self.wr_gpr(op.r0, w, g(op.r1));
                } else {
                    self.set_lb(op.r0, w);
                }
            }
            K_MOV_RI => {
                if w >= 32 {
                    self.a.mov_imm(w == 64, g(op.r0), op.imm);
                    self.set_lb(op.r0, w);
                } else {
                    self.a.mov_imm(false, T0, op.imm);
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOV_RM => {
                if w == 64
                    && (!mem::tso() || !stack_tso())
                    && op.base == 4
                    && op.index == RNONE
                    && op.r0 != 4
                    && op.disp % 8 == 0
                    && (-512..=504).contains(&op.disp)
                {
                    let body = self.body;
                    if let Some(n) = self.ops.get(k + 1).copied().filter(|n| {
                        k + 1 < body
                            && n.kind == K_MOV_RM
                            && n.w == 64
                            && n.base == 4
                            && n.index == RNONE
                            && n.disp == op.disp + 8
                            && n.r0 != 4
                            && n.r0 != op.r0
                    }) {
                        self.a.ldp_x(g(op.r0), g(n.r0), g(4), op.disp as i32);
                        self.set_lb(op.r0, 64);
                        self.set_lb(n.r0, 64);
                        self.skip = 1;
                        return;
                    }
                }
                if w >= 32 {
                    self.load_op(k, g(op.r0), &op, w as u32);
                    self.set_lb(op.r0, w);
                } else {
                    self.load_op(k, T0, &op, w as u32);
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOV_MR => {
                if w == 64
                    && (!mem::tso() || !stack_tso())
                    && op.base == 4
                    && op.index == RNONE
                    && op.disp % 8 == 0
                    && (-512..=504).contains(&op.disp)
                {
                    let body = self.body;
                    if let Some(n) = self.ops.get(k + 1).copied().filter(|n| {
                        k + 1 < body
                            && n.kind == K_MOV_MR
                            && n.w == 64
                            && n.base == 4
                            && n.index == RNONE
                            && n.disp == op.disp + 8
                    }) {
                        self.a.stp_x(g(op.r1), g(n.r1), g(4), op.disp as i32);
                        self.skip = 1;
                        return;
                    }
                }
                self.store_op(k, g(op.r1), &op, w as u32);
            }
            K_MOV_MI => {
                let src = if op.imm == 0 {
                    ZR
                } else {
                    self.a.mov_imm(w == 64, T0, op.imm);
                    T0
                };
                self.store_op(k, src, &op, w as u32);
            }
            K_MOVZX_RR | K_MOVSX_RR => {
                let sw = (op.x & 0x3F) as u32;
                let sx = op.kind == K_MOVSX_RR;
                if w >= 32 {
                    let x = w == 64;
                    if sx {
                        self.a.sbfm(x, g(op.r0), g(op.r1), 0, sw - 1);
                    } else {
                        self.a.ubfm(x, g(op.r0), g(op.r1), 0, sw - 1);
                    }
                    self.set_lb(op.r0, w);
                } else {
                    if sx {
                        self.a.sbfm(false, T0, g(op.r1), 0, sw - 1);
                    } else {
                        self.a.ubfm(false, T0, g(op.r1), 0, sw - 1);
                    }
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOVZX_RM | K_MOVSX_RM => {
                let sw = (op.x & 0x3F) as u32;
                if op.kind == K_MOVZX_RM && w >= 32 {
                    self.load_op(k, g(op.r0), &op, sw);
                    self.set_lb(op.r0, w);
                } else {
                    self.load_op(k, T0, &op, sw);
                    if op.kind == K_MOVSX_RM {
                        if w >= 32 {
                            self.a.sbfm(w == 64, g(op.r0), T0, 0, sw - 1);
                            self.set_lb(op.r0, w);
                        } else {
                            self.a.sbfm(false, T0, T0, 0, sw - 1);
                            self.wr_gpr(op.r0, w, T0);
                        }
                    } else {
                        self.wr_gpr(op.r0, w, T0);
                    }
                }
            }
            K_LEA => {
                if w == 64 {
                    let r = self.addr_into(&op, g(op.r0));
                    if r != g(op.r0) {
                        self.a.mov_rr(true, g(op.r0), r);
                    }
                    self.set_lb(op.r0, 64);
                } else if w == 32 && op.base < RNONE && (op.base & (fast::B_GS | fast::B_ADDR32)) == 0 {
                    // Direct 32-bit `lea r32, [base + idx*s + disp]` into `wD` (1 insn instead of
                    // `add x16; mov wD, w16`, zero-extending the upper 32 bits automatically).
                    let d = g(op.r0);
                    let s = (op.x >> 6) as u32;
                    let disp = op.disp as i32 as i64;
                    if op.index != RNONE {
                        self.a.addsub_rr(false, false, false, d, g(op.base), g(op.index), s);
                        if disp != 0 {
                            self.a.add_imm(false, d, d, disp);
                        }
                    } else if disp == 0 {
                        self.a.mov_rr(false, d, g(op.base));
                    } else {
                        self.a.add_imm(false, d, g(op.base), disp);
                    }
                    self.set_lb(op.r0, 32);
                } else {
                    let r = self.addr_into(&op, XA);
                    self.wr_gpr(op.r0, w, r);
                }
            }
            K_ALU_RI if op.r0 == 4 && op.w == 64 && (op.x & 0x3F) == A_ADD && self.try_batch_epilogue(k) => {}
            K_ALU_RR | K_ALU_RI | K_ALU_RM | K_ALU_MR | K_ALU_MI | K_CMPJ_RR | K_CMPJ_RI | K_CMPJ_RM => self.alu(k, &op),
            K_INC_R | K_DEC_R => self.incdec(k, &op),
            K_SHIFT_RI => self.shift_imm(k, &op),
            K_SHIFT_RCL => self.shift_cl(k, &op),
            K_PUSH_R if pushpop_enabled() && op.r0 != 4 => {
                if !self.try_batch_push(k) {
                    self.a.str_pre(g(op.r0), g(4), -8);
                }
            }
            K_PUSH_R => {
                self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                self.store(k, g(op.r0), XA, 64, Chk::Stack);
                self.a.mov_rr(true, g(4), XA);
            }
            K_POP_R if pushpop_enabled() && op.r0 != 4 => {
                if !self.try_batch_epilogue(k) {
                    self.a.ldr_post(g(op.r0), g(4), 8);
                    self.set_lb(op.r0, 64);
                }
            }
            K_POP_R => {
                self.load(k, T0, g(4), 64, Chk::Stack);
                self.a.addsub_ri(true, false, false, g(4), g(4), 8, false);
                self.wr_gpr(op.r0, 64, T0);
            }
            K_SETCC_R => {
                let c = self.cond(op.x & 0x3F);
                match c {
                    Cond::Host(cnd) => self.a.cset(T0, cnd),
                    Cond::Reg(r, inv) => {
                        if inv {
                            self.a.logic_ri(false, 2, T0, r, enc_logic_imm(1, false).unwrap());
                        } else {
                            self.a.mov_rr(false, T0, r);
                        }
                    }
                    Cond::Const(b) => self.a.movz(false, T0, b as u32, 0),
                }
                self.wr_gpr(op.r0, 8, T0);
            }
            K_CMOV_RR | K_CMOV_RM => self.cmov(k, &op),
            K_NOT_R => {
                // `orn d, zr, m` (`mvn`).
                self.a.emit(sf(w == 64) | 0x2A20_03E0 | g(op.r0) << 16 | 31 << 5 | T0);
                self.wr_gpr(op.r0, w, T0);
            }
            K_CX => match op.x & 0x3F {
                0 => self.a.sbfm(false, g(0), g(0), 0, 15),
                1 => self.a.sbfm(true, g(0), g(0), 0, 31),
                2 => self.a.asr_i(false, g(2), g(0), 31),
                3 => self.a.asr_i(true, g(2), g(0), 63),
                4 => {
                    self.a.sbfm(false, T0, g(0), 0, 7);
                    self.a.bfxil0(g(0), T0, 16);
                }
                _ => {
                    // Cwd: sign-extend AX into DX:AX (preserving bits [63:16] of RDX).
                    self.a.sbfm(false, T0, g(0), 0, 15);
                    self.a.asr_i(false, T0, T0, 31);
                    self.a.bfxil0(g(2), T0, 16);
                }
            },
            K_XCHG_RR => {
                self.a.mov_rr(true, T0, g(op.r0));
                self.wr_gpr(op.r0, w, g(op.r1));
                self.wr_gpr(op.r1, w, T0);
            }
            K_MOVD_GX => {
                let off = OFF_XMM + (op.r1 as u32 & 15) * 16;
                self.a.ldr(w as u32, g(op.r0), XCPU, off);
                self.set_lb(op.r0, w);
            }
            K_MOVD_XG => {
                let d = (op.r0 as u32 & 15) * 16;
                let s = g(op.r1);
                if op.imm == 64 {
                    self.a.str(64, s, XCPU, OFF_XMM + d);
                } else {
                    self.a.mov_rr(false, T0, s);
                    self.a.str(64, T0, XCPU, OFF_XMM + d);
                }
                self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VZU => {
                self.emit_ymm_clear_check(0xFFFF);
                self.ymm_clean = 0xFFFF;
            }
            K_VST => {
                let a = self.addr_into(&op, XA);
                let chk = Self::chk_of(&op);
                let s = OFF_XMM + (op.r1 as u32 & 15) * 16;
                match op.w >> 4 {
                    2 => {
                        self.a.ldr(32, T0, XCPU, s);
                        self.store(k, T0, a, 32, chk);
                    }
                    3 => {
                        self.a.ldr(64, T0, XCPU, s);
                        self.store(k, T0, a, 64, chk);
                    }
                    6 => {
                        self.a.ldr(64, T0, XCPU, s + 8);
                        self.store(k, T0, a, 64, chk);
                    }
                    4 => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        self.a.ldr(64, T0, XCPU, s);
                        self.a.ldr(64, T1, XCPU, s + 8);
                        if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T0, a, 0);
                        self.a.str(64, T1, a, 8);
                    }
                    _ => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        let sh = OFF_YMM_HI + (op.r1 as u32 & 15) * 16;
                        self.a.ldr(64, T0, XCPU, s);
                        self.a.ldr(64, T1, XCPU, s + 8);
                        self.a.ldr(64, T2, XCPU, sh);
                        self.a.ldr(64, T3, XCPU, sh + 8);
                        if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T0, a, 0);
                        self.a.str(64, T1, a, 8);
                        self.a.str(64, T2, a, 16);
                        self.a.str(64, T3, a, 24);
                    }
                }
            }
            K_CVTSI_R | K_CVTSI_M => {
                let width = (op.imm & 0xFF) as u32;
                let is_sd = (op.x & 0x3F) != 0;
                let src = if op.kind == K_CVTSI_R {
                    g(op.r1)
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, width, Self::chk_of(&op));
                    T0
                };
                self.a.scvtf_gp(width == 64, is_sd, 0, src);
                self.a.fmov_fg(is_sd, T1, 0);
                let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                let d = (op.r0 as u32 & 15) * 16;
                if is_sd {
                    self.a.ldr(64, T2, XCPU, OFF_XMM + s1 + 8);
                    self.a.str(64, T1, XCPU, OFF_XMM + d);
                    self.a.str(64, T2, XCPU, OFF_XMM + d + 8);
                } else {
                    self.a.ldr(64, T2, XCPU, OFF_XMM + s1);
                    self.a.ldr(64, T3, XCPU, OFF_XMM + s1 + 8);
                    self.a.bfxil0(T2, T1, 32);
                    self.a.str(64, T2, XCPU, OFF_XMM + d);
                    self.a.str(64, T3, XCPU, OFF_XMM + d + 8);
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_COMI_RR | K_COMI_RM => {
                let is_sd = (op.x & 0x3F) != 0;
                let bits = if is_sd { 64 } else { 32 };
                let la = self.live[k + 1];
                let off_a = OFF_XMM + (op.r0 as u32 & 15) * 16;
                if op.kind == K_COMI_RM {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T1, a, bits, Self::chk_of(&op));
                } else if la & FALL != 0 {
                    let off_b = OFF_XMM + (op.r1 as u32 & 15) * 16;
                    self.a.ldr(bits, T1, XCPU, off_b);
                }
                if la & FALL != 0 {
                    self.a.ldr(bits, T0, XCPU, off_a);
                    self.a.fmov_gf(is_sd, 0, T0);
                    self.a.fmov_gf(is_sd, 1, T1);
                    self.a.fcmp(is_sd, 0, 1);
                    if la & FC != 0 {
                        self.a.cset(T0, LT);
                        self.a.str(8, T0, XCPU, OFF_CF);
                    }
                    if la & FZ != 0 {
                        self.a.cset(T0, EQ);
                        self.a.cset(T1, VS);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
                        self.a.str(8, T0, XCPU, OFF_ZF);
                    }
                    if la & FS != 0 {
                        self.a.str(8, ZR, XCPU, OFF_SF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, ZR, XCPU, OFF_OF);
                    }
                    if la & FP != 0 {
                        self.a.cset(T0, VC);
                        self.a.str(8, T0, XCPU, OFF_PFV);
                    }
                }
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_CVTF2I_R | K_CVTF2I_M => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let is_sd = (op.x & 1) != 0;
                let trunc = (op.x & 2) != 0;
                let bits = if is_sd { 64 } else { 32 };
                if op.kind == K_CVTF2I_R {
                    let off = OFF_XMM + (op.r1 as u32 & 15) * 16;
                    self.a.ldr(bits, T0, XCPU, off);
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, bits, Self::chk_of(&op));
                }
                self.a.fmov_gf(is_sd, 0, T0);
                if !is_sd {
                    self.a.fcvt_d_s(0, 0);
                }
                if trunc {
                    self.a.frintz_d(0, 0);
                } else {
                    self.a.frintn_d(0, 0);
                }
                let x = w == 64;
                self.a.fcvtzs_d(x, T0, 0);
                let limit = if x { 0x43E0_0000_0000_0000u64 } else { 0x41E0_0000_0000_0000u64 };
                self.a.mov_imm(true, T1, limit);
                self.a.fmov_gf(true, 1, T1);
                self.a.fcmp(true, 0, 1);
                let indef = if x { 0x8000_0000_0000_0000u64 } else { 0x8000_0000u64 };
                self.a.mov_imm(x, T2, indef);
                self.a.csel(x, T0, T2, T0, HS);
                self.wr_gpr(op.r0, w, T0);
            }
            K_IMUL_RR | K_IMUL_RM | K_IMUL_RRI | K_IMUL_RMI => {
                let la = self.live[k + 1];
                let (a_src, b_src) = match op.kind {
                    K_IMUL_RR => (g(op.r0), g(op.r1)),
                    K_IMUL_RM => {
                        self.load_op(k, T1, &op, w as u32);
                        (g(op.r0), T1)
                    }
                    K_IMUL_RRI => {
                        let sext_imm = match w {
                            16 => op.imm as u16 as i16 as i64 as u64,
                            32 => op.imm as u32 as i32 as i64 as u64,
                            _ => op.imm,
                        };
                        self.a.mov_imm(true, T1, sext_imm);
                        (g(op.r1), T1)
                    }
                    _ => {
                        self.load_op(k, T0, &op, w as u32);
                        let sext_imm = match w {
                            16 => op.imm as u16 as i16 as i64 as u64,
                            32 => op.imm as u32 as i32 as i64 as u64,
                            _ => op.imm,
                        };
                        self.a.mov_imm(true, T1, sext_imm);
                        (T0, T1)
                    }
                };
                if la & (FC | FO) == 0 {
                    // Dead CF/OF: single-instruction native 32/64-bit `mul` directly into dst.
                    if w >= 32 {
                        self.a.mul(w == 64, g(op.r0), a_src, b_src);
                        self.set_lb(op.r0, w);
                    } else {
                        self.a.mul(false, T2, a_src, b_src);
                        self.wr_gpr(op.r0, w, T2);
                    }
                    self.fs.host &= !(FC | FO);
                    self.fs.mem &= !(FC | FO);
                    return;
                }
                if w == 64 {
                    self.a.mul(true, T2, a_src, b_src);
                    if la & (FC | FO) != 0 {
                        self.a.smulh(T3, a_src, b_src);
                        self.a.asr_i(true, T4, T2, 63);
                        self.a.logic_rr(true, 2, T3, T3, T4, 0, 0);
                        self.a.clz(true, T4, T3);
                        self.a.lsr_i(true, T4, T4, 6);
                        self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
                        if la & FC != 0 {
                            self.a.str(8, T4, XCPU, OFF_CF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, T4, XCPU, OFF_OF);
                        }
                    }
                } else {
                    self.a.sbfm(true, T0, a_src, 0, (w - 1) as u32);
                    if matches!(op.kind, K_IMUL_RR | K_IMUL_RM) {
                        self.a.sbfm(true, T1, b_src, 0, (w - 1) as u32);
                    }
                    self.a.mul(true, T2, T0, T1);
                    if la & (FC | FO) != 0 {
                        self.a.sbfm(true, T3, T2, 0, (w - 1) as u32);
                        self.a.logic_rr(true, 2, T3, T2, T3, 0, 0);
                        self.a.clz(true, T4, T3);
                        self.a.lsr_i(true, T4, T4, 6);
                        self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
                        if la & FC != 0 {
                            self.a.str(8, T4, XCPU, OFF_CF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, T4, XCPU, OFF_OF);
                        }
                    }
                }
                self.wr_gpr(op.r0, w, T2);
                self.fs.host &= !(FC | FO);
                self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
            }
            K_NEG_R => {
                let la = self.live[k + 1];
                let need_nzcv = la & FNZCV != 0;
                let need_pf = la & FP != 0;
                let d = g(op.r0);
                let res = if w >= 32 {
                    self.a.addsub_rr(w == 64, true, need_nzcv, d, ZR, d, 0);
                    self.set_lb(op.r0, w);
                    d
                } else {
                    let sh = 64 - w as u32;
                    if need_nzcv {
                        self.a.lsl_i(true, T3, d, sh);
                        self.a.addsub_rr(true, true, true, T5, ZR, T3, 0);
                        self.a.lsr_i(true, T2, T5, sh);
                    } else {
                        self.a.addsub_rr(false, true, false, T2, ZR, d, 0);
                    }
                    self.wr_gpr(op.r0, w, T2);
                    T2
                };
                if need_pf {
                    self.a.str(8, res, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if need_pf {
                    self.fs.mem |= FP;
                }
                self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
                self.fs.src = Src::Sub;
            }
            K_INC_M | K_DEC_M => {
                let dec = op.kind == K_DEC_M;
                let wb = w as u32;
                let la = self.live[k + 1];
                if la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
                    self.flush_flags(FC);
                }
                let need_nzcv = la & (FZ | FS | FO) != 0;
                let need_pf = la & FP != 0;
                let addr = self.addr_into(&op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(&op));
                if wb >= 32 {
                    self.a.addsub_ri(wb == 64, dec, need_nzcv, T2, T0, 1, false);
                } else if need_nzcv {
                    let sh = 64 - wb;
                    self.a.lsl_i(true, T3, T0, sh);
                    self.a.mov_imm(true, T4, 1u64 << sh);
                    self.a.addsub_rr(true, dec, true, T5, T3, T4, 0);
                    self.a.lsr_i(true, T2, T5, sh);
                } else {
                    self.a.addsub_ri(false, dec, false, T2, T0, 1, false);
                }
                self.store_checked_addr(T2, addr, wb);
                if need_pf {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !(FZ | FS | FO | FP);
                if need_pf {
                    self.fs.mem |= FP;
                }
                let cf_mem = self.fs.mem & FC;
                self.fs.mem = (self.fs.mem & !FC) | cf_mem;
                self.fs.host = if need_nzcv { la & (FZ | FS | FO) } else { 0 };
                self.fs.src = Src::IncDec;
            }
            K_BSF_RR | K_BSF_RM => {
                let wb = w as u32;
                let la = self.live[k + 1];
                let v = if op.kind == K_BSF_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(&op));
                    T0
                };
                if la & (FC | FZ) != 0 {
                    self.a.clz(true, T2, v);
                    self.a.lsr_i(true, T2, T2, 6);
                    if la & FC != 0 {
                        self.a.str(8, T2, XCPU, OFF_CF);
                    }
                    if la & FZ != 0 {
                        self.a.str(8, T2, XCPU, OFF_ZF);
                    }
                    self.fs.host &= !(FC | FZ);
                    self.fs.mem = (self.fs.mem & !(FC | FZ)) | (la & (FC | FZ));
                }
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let skip = self.a.cbz(true, v);
                self.a.rbit(true, T1, v);
                self.a.clz(true, T1, T1);
                let d = g(op.r0);
                match wb {
                    64 => self.a.mov_rr(true, d, T1),
                    32 => self.a.mov_rr(false, d, T1),
                    _ => self.a.bfxil0(d, T1, wb),
                }
                self.lb_store_now(op.r0, w);
                self.a.bind(skip);
            }
            K_BT_RR | K_BT_RI => {
                let wb = w as u32;
                let la = self.live[k + 1];
                if la & FC != 0 {
                    if op.kind == K_BT_RI {
                        let b = (op.imm as u32) & (wb - 1);
                        self.a.ubfx(T0, g(op.r0), b, 1);
                    } else {
                        self.a.logic_ri(false, 0, T1, g(op.r1), enc_logic_imm((wb - 1) as u64, false).unwrap());
                        self.a.shiftv(true, 9, T0, g(op.r0), T1);
                        self.a.ubfx(T0, T0, 0, 1);
                    }
                    self.a.str(8, T0, XCPU, OFF_CF);
                    self.fs.host &= !FC;
                    self.fs.mem |= FC;
                }
            }
            K_ROT_RI => {
                let wb = w as u32;
                let cnt = op.imm as u32;
                let ror = (op.x & 0x3F) != 0;
                let la = self.live[k + 1];
                let ua = self.ua(op.r0, wb);
                match wb {
                    64 => self.a.ror_i(true, T1, ua, if ror { cnt } else { 64 - cnt }),
                    32 => self.a.ror_i(false, T1, ua, if ror { cnt } else { 32 - cnt }),
                    _ => {
                        let c_mod = cnt % wb;
                        if c_mod == 0 {
                            self.a.mov_rr(false, T1, ua);
                        } else {
                            let rcnt = if ror { c_mod } else { wb - c_mod };
                            self.a.lsr_i(false, T1, ua, rcnt);
                            self.a.lsl_i(false, T2, ua, wb - rcnt);
                            self.a.logic_rr(false, 1, T1, T1, T2, 0, 0);
                            self.a.ubfm(false, T1, T1, 0, wb - 1);
                        }
                    }
                }
                self.wr_gpr(op.r0, w, T1);
                if la & (FC | FO) != 0 {
                    self.a.ubfx(T2, T1, wb - 1, 1);
                    let cf_reg = if ror {
                        if la & FO != 0 {
                            self.a.ubfx(T3, T1, wb - 2, 1);
                            self.a.logic_rr(false, 2, T3, T2, T3, 0, 0);
                        }
                        T2
                    } else {
                        self.a.ubfx(T4, T1, 0, 1);
                        if la & FO != 0 {
                            self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                        }
                        T4
                    };
                    if la & FC != 0 {
                        self.a.str(8, cf_reg, XCPU, OFF_CF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, T3, XCPU, OFF_OF);
                    }
                    self.fs.host &= !(FC | FO);
                    self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
                }
            }
            K_VSHUF_RM if op.w & 3 != VM_V256 && (op.x & 0x3F) <= VS_ZX64 => {
                let a = self.addr_into(&op, XA);
                let chk = Self::chk_of(&op);
                let d = (op.r0 as u32 & 15) * 16;
                match op.x & 0x3F {
                    VS_ZX32 => {
                        self.load(k, T0, a, 32, chk);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_ZX64 => {
                        self.load(k, T0, a, 64, chk);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    _ if chk == Chk::Ro => {
                        self.a.ldr(128, 0, a, 0);
                        self.a.str(128, 0, XCPU, OFF_XMM + d);
                    }
                    _ => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        self.a.ldr(64, T0, a, 0);
                        self.a.ldr(64, T1, a, 8);
                        if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                            self.a.dmb_ishld();
                        }
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VSHUF_RR if op.w & 3 != VM_V256 && (op.x & 0x3F) <= VS_AND => {
                let d = (op.r0 as u32 & 15) * 16;
                let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                let s2 = (op.r1 as u32 & 15) * 16;
                match op.x & 0x3F {
                    VS_MOV => {
                        if d != s2 {
                            self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                            self.a.ldr(64, T1, XCPU, OFF_XMM + s2 + 8);
                            self.a.str(64, T0, XCPU, OFF_XMM + d);
                            self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                        }
                    }
                    VS_ZX32 => {
                        self.a.ldr(32, T0, XCPU, OFF_XMM + s2);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_ZX64 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_MRG32 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s1);
                        self.a.ldr(64, T2, XCPU, OFF_XMM + s1 + 8);
                        self.a.ldr(32, T1, XCPU, OFF_XMM + s2);
                        self.a.bfxil0(T0, T1, 32);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T2, XCPU, OFF_XMM + d + 8);
                    }
                    VS_MRG64 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                        self.a.ldr(64, T1, XCPU, OFF_XMM + s1 + 8);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                    VS_XOR if s1 == s2 => {
                        self.a.str(64, ZR, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    sub => {
                        let opc = match sub {
                            VS_XOR => 2,
                            VS_OR => 1,
                            _ => 0,
                        };
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s1);
                        self.a.ldr(64, T1, XCPU, OFF_XMM + s1 + 8);
                        self.a.ldr(64, T2, XCPU, OFF_XMM + s2);
                        self.a.ldr(64, T3, XCPU, OFF_XMM + s2 + 8);
                        self.a.logic_rr(true, opc, T0, T0, T2, 0, 0);
                        self.a.logic_rr(true, opc, T1, T1, T3, 0, 0);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VSHUF_RR | K_VSHUF_RM | K_VINT_RR | K_VINT_RM | K_VFP_RR | K_VFP_RM => self.vop(k, &op),
            K_BSWAP_R..=K_VSHA_RM => self.ext_op(k, &op),
            _ => unreachable!("unsupported op kind {}", op.kind),
        }
    }

    fn alu(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let sub = op.x & 0x3F;
        let w = op.w as u32;
        let la = self.live[k + 1];
        let need_nzcv = la & FNZCV != 0;
        let need_pf = la & FP != 0;
        let is_cmp = sub == A_CMP || sub == A_TEST;
        let mem_a = matches!(op.kind, K_ALU_MR | K_ALU_MI);
        let mem_dst = mem_a && !is_cmp;
        let writes_reg = !mem_a && !is_cmp;
        if is_cmp && !need_nzcv && !need_pf {
            // Dead compare: only the memory operand's range check remains observable.
            if matches!(op.kind, K_ALU_RM | K_CMPJ_RM | K_ALU_MR | K_ALU_MI) {
                if let Grp::Member { id } = self.grp[k] {
                    if self.grp_ok & (1u64 << id) != 0 {
                        self.grp_k = k;
                        self.fs.host = 0;
                        self.fs.mem &= !FALL;
                        return;
                    }
                }
                if Self::chk_of(op) == Chk::Full {
                    let a = self.addr_into(op, XA);
                    let se = self.side_exit_here(k);
                    self.range_check_op(k, a, se);
                }
            }
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            return;
        }
        // Operands.
        let mut addr = 0u32;
        let a_reg = if mem_a {
            if is_cmp {
                self.load_op(k, T0, op, w);
            } else {
                addr = self.addr_into(op, XA);
                self.load(k, T0, addr, w, Self::chk_of(op));
            }
            T0
        } else {
            g(op.r0)
        };
        enum B {
            R(u32),
            I(u64),
        }
        let b = match op.kind {
            K_ALU_RR | K_ALU_MR | K_CMPJ_RR => B::R(g(op.r1)),
            K_ALU_RI | K_ALU_MI | K_CMPJ_RI => B::I(op.imm),
            _ => {
                self.load_op(k, T1, op, w);
                B::R(T1)
            }
        };
        let mask = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };
        if matches!(sub, A_ADC | A_SBB) {
            self.flag_reg(FC, T6);
        }
        let res: u32;
        if w >= 32 {
            let x = w == 64;
            let dst = if writes_reg {
                g(op.r0)
            } else if mem_dst || need_pf {
                T2
            } else {
                ZR
            };
            let s = need_nzcv || dst == ZR;
            match sub {
                A_ADC | A_SBB => {
                    let b_reg = match b {
                        B::R(r) => r,
                        B::I(v) => {
                            self.a.mov_imm(x, T1, v & mask);
                            T1
                        }
                    };
                    if sub == A_ADC {
                        self.a.addsub_ri(false, true, true, ZR, T6, 1, false);
                        self.a.adcs(x, s, dst, a_reg, b_reg);
                    } else {
                        self.a.addsub_rr(false, true, true, ZR, ZR, T6, 0);
                        self.a.sbcs(x, s, dst, a_reg, b_reg);
                    }
                }
                A_ADD | A_SUB | A_CMP => {
                    let is_sub = sub != A_ADD;
                    match b {
                        B::R(r) => self.a.addsub_rr(x, is_sub, s, dst, a_reg, r, 0),
                        B::I(v) => {
                            let v = v & mask;
                            let nv = v.wrapping_neg() & mask;
                            if v < 4096 {
                                self.a.addsub_ri(x, is_sub, s, dst, a_reg, v as u32, false);
                            } else if v & 0xFFF == 0 && v < (1 << 24) {
                                self.a.addsub_ri(x, is_sub, s, dst, a_reg, (v >> 12) as u32, true);
                            } else if nv != 0 && nv < 4096 {
                                self.a.addsub_ri(x, !is_sub, s, dst, a_reg, nv as u32, false);
                            } else {
                                self.a.mov_imm(x, T1, v);
                                self.a.addsub_rr(x, is_sub, s, dst, a_reg, T1, 0);
                            }
                        }
                    }
                }
                _ => {
                    let opc = match sub {
                        A_AND | A_TEST => {
                            if s {
                                3
                            } else {
                                0
                            }
                        }
                        A_OR => 1,
                        _ => 2,
                    };
                    match b {
                        B::R(r) => self.a.logic_rr(x, opc, dst, a_reg, r, 0, 0),
                        B::I(v) => {
                            if let Some(e) = enc_logic_imm(v & mask, x) {
                                self.a.logic_ri(x, opc, dst, a_reg, e);
                            } else {
                                self.a.mov_imm(x, T1, v & mask);
                                self.a.logic_rr(x, opc, dst, a_reg, T1, 0, 0);
                            }
                        }
                    }
                    if s && (opc == 1 || opc == 2) {
                        self.a.logic_rr(x, 3, ZR, dst, dst, 0, 0); // tst
                    }
                }
            }
            res = dst;
            if writes_reg {
                self.set_lb(op.r0, w as u8);
            }
        } else if matches!(sub, A_ADC | A_SBB) {
            self.a.ubfm(false, T3, a_reg, 0, w - 1);
            match b {
                B::R(r) => self.a.ubfm(false, T4, r, 0, w - 1),
                B::I(v) => self.a.mov_imm(false, T4, v & mask),
            }
            let is_sub = sub == A_SBB;
            self.a.addsub_rr(false, is_sub, false, T5, T3, T4, 0);
            self.a.addsub_rr(false, is_sub, false, T5, T5, T6, 0);
            self.a.ubfm(false, T2, T5, 0, w - 1);
            if need_nzcv {
                if la & FC != 0 {
                    self.a.ubfx(XS, T5, w, 1);
                    self.a.str(8, XS, XCPU, OFF_CF);
                }
                if la & FZ != 0 {
                    self.a.clz(true, XS, T2);
                    self.a.lsr_i(true, XS, XS, 6);
                    self.a.str(8, XS, XCPU, OFF_ZF);
                }
                if la & FS != 0 {
                    self.a.ubfx(XS, T2, w - 1, 1);
                    self.a.str(8, XS, XCPU, OFF_SF);
                }
                if la & FO != 0 {
                    if is_sub {
                        self.a.logic_rr(false, 2, T4, T3, T4, 0, 0);
                        self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                    } else {
                        self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                        self.a.logic_rr(false, 2, T4, T4, T2, 0, 0);
                    }
                    self.a.logic_rr(false, 0, T3, T3, T4, 0, 0);
                    self.a.ubfx(XS, T3, w - 1, 1);
                    self.a.str(8, XS, XCPU, OFF_OF);
                }
            }
            if writes_reg {
                self.wr_gpr(op.r0, w as u8, T2);
            }
            if mem_dst {
                self.store_checked_addr(T2, addr, w);
            }
            if need_pf {
                self.a.str(8, T2, XCPU, OFF_PFV);
            }
            self.fs = FlagState {
                src: Src::None,
                host: 0,
                mem: (self.fs.mem & !FALL) | (la & FALL),
            };
            return;
        } else if sub == A_TEST && need_nzcv && (la & (FS | FP)) == 0 {
            // 8/16-bit `test reg/mem, imm/reg` when only ZF (and constant-0 CF/OF) are live:
            // `tst w_a, #(imm & mask)` directly sets Z in 1 instruction (no shift/un-shift).
            match b {
                B::I(v) => {
                    let mv = v & mask;
                    if let Some(e) = enc_logic_imm(mv, false) {
                        self.a.logic_ri(false, 3, ZR, a_reg, e);
                    } else {
                        self.a.mov_imm(false, T4, mv);
                        self.a.logic_rr(false, 3, ZR, a_reg, T4, 0, 0);
                    }
                }
                B::R(r) => {
                    self.a.ubfm(false, T4, r, 0, w - 1);
                    self.a.logic_rr(false, 3, ZR, a_reg, T4, 0, 0);
                }
            }
            res = ZR;
        } else {
            let sh = 64 - w;
            if need_nzcv {
                self.a.lsl_i(true, T3, a_reg, sh);
                match b {
                    B::R(r) => self.a.lsl_i(true, T4, r, sh),
                    B::I(v) => self.a.mov_imm(true, T4, (v & mask) << sh),
                }
                match sub {
                    A_ADD => self.a.addsub_rr(true, false, true, T5, T3, T4, 0),
                    A_SUB | A_CMP => self.a.addsub_rr(true, true, true, T5, T3, T4, 0),
                    A_AND | A_TEST => self.a.logic_rr(true, 3, T5, T3, T4, 0, 0),
                    A_OR => {
                        self.a.logic_rr(true, 1, T5, T3, T4, 0, 0);
                        self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                    }
                    A_XOR | _ => {
                        self.a.logic_rr(true, 2, T5, T3, T4, 0, 0);
                        self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                    }
                }
                if !is_cmp || need_pf {
                    self.a.lsr_i(true, T2, T5, sh);
                }
            } else {
                let br = match b {
                    B::R(r) => r,
                    B::I(v) => {
                        self.a.mov_imm(false, T4, v & mask);
                        T4
                    }
                };
                match sub {
                    A_ADD => self.a.addsub_rr(false, false, false, T2, a_reg, br, 0),
                    A_SUB | A_CMP => self.a.addsub_rr(false, true, false, T2, a_reg, br, 0),
                    A_AND | A_TEST => self.a.logic_rr(false, 0, T2, a_reg, br, 0, 0),
                    A_OR => self.a.logic_rr(false, 1, T2, a_reg, br, 0, 0),
                    _ => self.a.logic_rr(false, 2, T2, a_reg, br, 0, 0),
                }
            }
            res = T2;
            if writes_reg {
                self.wr_gpr(op.r0, w as u8, T2);
            }
        }
        if mem_dst {
            self.store_checked_addr(res, addr, w);
        }
        // AW2_JIT_FLAGLAZY: the parity byte may stay in the result register (`pf_defer`).
        let pf_mem = need_pf && !self.pf_defer(k, res, op.r0);
        if pf_mem {
            self.a.str(8, res, XCPU, OFF_PFV);
        }
        self.fs.mem &= !FALL;
        if pf_mem {
            self.fs.mem |= FP;
        }
        self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
        self.fs.src = match sub {
            A_ADD | A_ADC => Src::Add,
            A_SUB | A_CMP | A_SBB => Src::Sub,
            _ => Src::Logic,
        };
    }

    fn incdec(&mut self, k: usize, op: &FastOp) {
        let dec = op.kind == fast::K_DEC_R;
        let w = op.w as u32;
        let la = self.live[k + 1];
        if la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
            self.flush_flags(FC);
        }
        let need_nzcv = la & (FZ | FS | FO) != 0;
        let need_pf = la & FP != 0;
        let d = g(op.r0);
        let res;
        if w >= 32 {
            self.a.addsub_ri(w == 64, dec, need_nzcv, d, d, 1, false);
            res = d;
            self.set_lb(op.r0, w as u8);
        } else {
            let sh = 64 - w;
            if need_nzcv {
                self.a.lsl_i(true, T3, d, sh);
                self.a.mov_imm(true, T4, 1u64 << sh);
                self.a.addsub_rr(true, dec, true, T5, T3, T4, 0);
                self.a.lsr_i(true, T2, T5, sh);
            } else {
                self.a.addsub_ri(false, dec, false, T2, d, 1, false);
            }
            self.wr_gpr(op.r0, w as u8, T2);
            res = T2;
        }
        let pf_mem = need_pf && !self.pf_defer(k, res, op.r0);
        if pf_mem {
            self.a.str(8, res, XCPU, OFF_PFV);
        }
        self.fs.mem &= !(FZ | FS | FO | FP);
        if pf_mem {
            self.fs.mem |= FP;
        }
        // CF keeps its state: still in memory if it was (it was flushed above when live).
        let cf_mem = self.fs.mem & FC;
        self.fs.mem = (self.fs.mem & !FC) | cf_mem;
        self.fs.host = if need_nzcv { la & (FZ | FS | FO) } else { 0 };
        self.fs.src = Src::IncDec;
    }

    /// Store computed shift flags: T1 = result, T2 = CF, T3 = OF (`of_zero`: OF = 0).
    fn shift_flags(&mut self, la: u8, w: u32, of_zero: bool) {
        if la & FC != 0 {
            self.a.str(8, T2, XCPU, OFF_CF);
        }
        if la & FO != 0 {
            if of_zero {
                self.a.str(8, ZR, XCPU, OFF_OF);
            } else {
                self.a.str(8, T3, XCPU, OFF_OF);
            }
        }
        if la & FZ != 0 {
            self.a.clz(true, T5, T1);
            self.a.lsr_i(true, T5, T5, 6);
            self.a.str(8, T5, XCPU, OFF_ZF);
        }
        if la & FS != 0 {
            self.a.ubfx(T6, T1, w - 1, 1);
            self.a.str(8, T6, XCPU, OFF_SF);
        }
        if la & FP != 0 {
            self.a.str(8, T1, XCPU, OFF_PFV);
        }
    }

    /// Zero-extended guest operand of width `w` (register `i`) into `T0` (or the register).
    fn ua(&mut self, i: u8, w: u32) -> u32 {
        match w {
            64 => g(i),
            32 => {
                self.a.mov_rr(false, T0, g(i));
                T0
            }
            _ => {
                self.a.ubfm(true, T0, g(i), 0, w - 1);
                T0
            }
        }
    }

    fn shift_imm(&mut self, k: usize, op: &FastOp) {
        let w = op.w as u32;
        let c = op.imm as u32;
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        if w >= 32 && la == 0 && c != 0 && c < w {
            let d = g(op.r0);
            let x = w == 64;
            match sub {
                fast::SHIFT_SHL => self.a.lsl_i(x, d, d, c),
                fast::SHIFT_SHR => self.a.lsr_i(x, d, d, c),
                _ => self.a.asr_i(x, d, d, c),
            }
            self.set_lb(op.r0, w as u8);
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
            return;
        }
        let ua = self.ua(op.r0, w);
        let mut of_zero = false;
        match sub {
            fast::SHIFT_SHL => {
                if c >= 64 {
                    self.a.movz(true, T1, 0, 0);
                } else {
                    self.a.lsl_i(true, T1, ua, c);
                    if w < 64 {
                        self.a.ubfm(true, T1, T1, 0, w - 1);
                    }
                }
                if la & (FC | FO) != 0 {
                    if c <= w {
                        self.a.ubfx(T2, ua, w - c, 1);
                    } else {
                        self.a.movz(true, T2, 0, 0);
                    }
                    self.a.ubfx(T3, T1, w - 1, 1);
                    self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
                }
            }
            fast::SHIFT_SHR => {
                self.a.lsr_i(true, T1, ua, c);
                if la & FC != 0 {
                    self.a.ubfx(T2, ua, c - 1, 1);
                }
                if la & FO != 0 {
                    self.a.ubfx(T3, ua, w - 1, 1);
                }
            }
            _ => {
                let sa = if w < 64 {
                    self.a.sbfm(true, T4, ua, 0, w - 1);
                    T4
                } else {
                    ua
                };
                self.a.asr_i(true, T1, sa, c.min(63));
                if w < 64 {
                    self.a.ubfm(true, T1, T1, 0, w - 1);
                }
                if la & FC != 0 {
                    self.a.ubfx(T2, sa, (c - 1).min(63), 1);
                }
                of_zero = true;
            }
        }
        self.shift_flags(la, w, of_zero);
        self.wr_gpr(op.r0, w as u8, T1);
        self.fs.host = 0;
        self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
        self.fs.src = Src::None;
    }

    fn shift_cl(&mut self, k: usize, op: &FastOp) {
        let w = op.w as u32;
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        // Count 0 leaves everything unchanged: live flags must be in memory either way.
        self.flush_flags(la);
        if tracked(op.r0) {
            self.flush_lb_reg(op.r0 as usize);
        }
        let mask = if w == 64 { 63 } else { 31 };
        self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
        let skip = self.a.cbz(false, T6);
        let ua = self.ua(op.r0, w);
        let mut of_zero = false;
        match sub {
            fast::SHIFT_SHL => {
                if w < 64 {
                    self.a.shiftv(true, 8, T4, ua, T6);
                    self.a.ubfx(T2, T4, w, 1);
                    self.a.ubfm(true, T1, T4, 0, w - 1);
                } else {
                    self.a.shiftv(true, 8, T1, ua, T6);
                    self.a.movz(true, T4, 64, 0);
                    self.a.addsub_rr(true, true, false, T4, T4, T6, 0);
                    self.a.shiftv(true, 9, T2, ua, T4);
                    self.a.ubfx(T2, T2, 0, 1);
                }
                self.a.ubfx(T3, T1, w - 1, 1);
                self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
            }
            fast::SHIFT_SHR => {
                self.a.shiftv(true, 9, T1, ua, T6);
                self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                self.a.shiftv(true, 9, T2, ua, T4);
                self.a.ubfx(T2, T2, 0, 1);
                self.a.ubfx(T3, ua, w - 1, 1);
            }
            _ => {
                let sa = if w < 64 {
                    self.a.sbfm(true, XS, ua, 0, w - 1);
                    XS
                } else {
                    ua
                };
                self.a.shiftv(true, 10, T1, sa, T6);
                if w < 64 {
                    self.a.ubfm(true, T1, T1, 0, w - 1);
                }
                self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                self.a.shiftv(true, 10, T2, sa, T4);
                self.a.ubfx(T2, T2, 0, 1);
                of_zero = true;
            }
        }
        self.shift_flags(la, w, of_zero);
        let d = g(op.r0);
        match w {
            64 => self.a.mov_rr(true, d, T1),
            32 => self.a.mov_rr(false, d, T1),
            _ => self.a.bfxil0(d, T1, w),
        }
        self.lb_store_now(op.r0, w as u8);
        if w == 32 {
            // Count 0: flags unchanged, but a 32-bit destination is still zero-extended.
            let done = self.a.b();
            self.a.bind(skip);
            self.a.mov_rr(false, d, d);
            self.lb_store_now(op.r0, 32);
            self.a.bind(done);
        } else {
            self.a.bind(skip);
        }
        self.fs.host = 0;
        self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
        self.fs.src = Src::None;
    }

    fn cmov(&mut self, k: usize, op: &FastOp) {
        let w = op.w;
        let d = g(op.r0);
        let c = self.cond(op.x & 0x3F);
        let rm = op.kind == fast::K_CMOV_RM;
        if w == 32 {
            self.set_lb(op.r0, 32);
        } else if tracked(op.r0) {
            self.flush_lb_reg(op.r0 as usize);
        }
        if !rm {
            if let Cond::Host(cnd) = c {
                let s = g(op.r1);
                match w {
                    64 => self.a.csel(true, d, s, d, cnd),
                    32 => self.a.csel(false, d, s, d, cnd),
                    _ => {
                        self.a.csel(false, T0, s, d, cnd);
                        self.a.bfxil0(d, T0, 16);
                    }
                }
                if w != 32 && self.lb_on && tracked(op.r0) {
                    let f = self.a.bcond(cnd ^ 1);
                    self.lb_store_now(op.r0, w);
                    self.a.bind(f);
                }
                return;
            }
        }
        if w == 32 {
            // A false condition still zero-extends the destination.
            self.a.mov_rr(false, d, d);
        }
        let skip = self.branch_if_false(&c);
        if skip.is_none() || !matches!(c, Cond::Const(false)) {
            let src = if rm {
                self.load_op(k, T0, op, w as u32);
                T0
            } else {
                g(op.r1)
            };
            match w {
                64 => self.a.mov_rr(true, d, src),
                32 => self.a.mov_rr(false, d, src),
                _ => self.a.bfxil0(d, src, 16),
            }
            if w != 32 {
                self.lb_store_now(op.r0, w);
            }
        }
        if let Some(f) = skip {
            self.a.bind(f);
        }
    }

    /// Batch consecutive `push r` ops + optional dead-flags `sub rsp, imm` into a single
    /// `sub x23, x23, #total` followed by parallel fixed-offset `stp`/`str` stores.
    fn try_batch_push(&mut self, k: usize) -> bool {
        use fast::*;
        let body = self.body;
        let mut m = 0usize;
        while k + m < body && self.ops[k + m].kind == K_PUSH_R && self.ops[k + m].r0 != 4 {
            m += 1;
        }
        let mut sub_imm = 0u64;
        let mut has_sub = false;
        if k + m < body {
            let s = self.ops[k + m];
            if s.kind == K_ALU_RI
                && s.r0 == 4
                && s.w == 64
                && (s.x & 0x3F) == A_SUB
                && s.imm > 0
                && s.imm % 8 == 0
                && s.imm <= 440
                && (self.live[k + m + 1] & FALL) == 0
            {
                sub_imm = s.imm;
                has_sub = true;
            }
        }
        let total = (m as u64) * 8 + sub_imm;
        if (m < 2 && !has_sub) || total > 504 {
            return false;
        }
        if m == 2 && !has_sub {
            self.a.stp_pre(g(self.ops[k + 1].r0), g(self.ops[k].r0), g(4), -16);
            self.skip = 1;
            return true;
        }
        self.a.addsub_ri(true, true, false, g(4), g(4), total as u32, false);
        let mut i = 0usize;
        while i + 1 < m {
            let off = (sub_imm as usize + (m - 2 - i) * 8) as i32;
            self.a.stp_x(g(self.ops[k + i + 1].r0), g(self.ops[k + i].r0), g(4), off);
            i += 2;
        }
        if i < m {
            self.a.str(64, g(self.ops[k + i].r0), g(4), sub_imm as u32);
        }
        if has_sub {
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
        }
        self.skip = m + usize::from(has_sub) - 1;
        true
    }

    /// Batch optional dead-flags `add rsp, imm` + consecutive distinct `pop r` ops (and when
    /// ending at a plain `Term::Ret`, hoist `ldr x0, [x23, #ret_off]` and `ldp x1, x2, [x7]`
    /// BEFORE the parallel `ldp`/`ldr` pops to break serial `x23` chains and hide load latency).
    fn try_batch_epilogue(&mut self, k: usize) -> bool {
        use fast::*;
        if !pushpop_enabled() {
            return false;
        }
        let body = self.body;
        let mut p = k;
        let mut add_imm = 0u64;
        let mut has_add = false;
        if self.ops[p].kind == K_ALU_RI {
            let a = self.ops[p];
            if a.r0 != 4
                || a.w != 64
                || (a.x & 0x3F) != A_ADD
                || a.imm == 0
                || a.imm % 8 != 0
                || a.imm > 440
                || (self.live[p + 1] & FALL) != 0
            {
                return false;
            }
            add_imm = a.imm;
            has_add = true;
            p += 1;
        }
        let pop_start = p;
        let mut seen = 0u16;
        while p < body && self.ops[p].kind == K_POP_R && self.ops[p].r0 != 4 {
            let bit = 1u16 << self.ops[p].r0;
            if seen & bit != 0 {
                break;
            }
            seen |= bit;
            p += 1;
        }
        let m = p - pop_start;
        let ret_off = add_imm + (m as u64) * 8;
        let can_hoist_ret = p == body
            && self.term_is_plain_ret
            && !self.bud_dec
            && self.term_live_out == 0
            && !self.lb_on
            && ret_off <= 504;
        if !can_hoist_ret {
            if (m < 2 && !has_add) || m == 0 || ret_off > 504 {
                return false;
            }
            if m == 2 && !has_add {
                let r0 = self.ops[pop_start].r0;
                let r1 = self.ops[pop_start + 1].r0;
                self.a.ldp_post(g(r0), g(r1), g(4), 16);
                self.set_lb(r0, 64);
                self.set_lb(r1, 64);
                self.skip = 1;
                return true;
            }
        }
        if can_hoist_ret {
            // Start return-address + shadow-RAS loads FIRST so their latency overlaps pops.
            self.a.ldr(64, T0, g(4), ret_off as u32);
            if self.ras_miss != 0 {
                self.a.ldp_x(1, 2, XRAS, 0);
            }
            self.ret_preloaded = true;
        }
        let mut i = 0usize;
        while i + 1 < m {
            let r0 = self.ops[pop_start + i].r0;
            let r1 = self.ops[pop_start + i + 1].r0;
            let off = (add_imm as usize + i * 8) as i32;
            self.a.ldp_x(g(r0), g(r1), g(4), off);
            self.set_lb(r0, 64);
            self.set_lb(r1, 64);
            i += 2;
        }
        if i < m {
            let r0 = self.ops[pop_start + i].r0;
            self.a.ldr(64, g(r0), g(4), (add_imm as usize + i * 8) as u32);
            self.set_lb(r0, 64);
        }
        let total_adj = ret_off + if can_hoist_ret { 8 } else { 0 };
        self.a.addsub_ri(true, false, false, g(4), g(4), total_adj as u32, false);
        if has_add {
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
        }
        self.skip = (p - k) - 1;
        true
    }

    fn terminator(&mut self, term: Term, k: usize) {
        let live_out = self.live[self.ops.len()];
        let op = self.ops[k];
        let fall = op.ip.wrapping_add(op.len as u64);
        match term {
            Term::Fall(x) => {
                self.flush_flags(live_out);
                if self.lazy {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                self.exit_to(x);
            }
            Term::Jcc { cc, target, fall, near } if self.lazy => self.jcc_lazy(cc, target, fall, near),
            Term::Jcc { cc, target, fall, near } => {
                self.flush_flags(live_out);
                self.flush_lb();
                let c = self.cond(cc);
                let taken_host = near && self.target_host;
                if !taken_host {
                    match c {
                        Cond::Const(true) => {
                            self.exit_to(target);
                            return;
                        }
                        Cond::Const(false) => {
                            self.exit_to(fall);
                            return;
                        }
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, target));
                            self.exit_to(fall);
                            return;
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, target));
                            self.exit_to(fall);
                            return;
                        }
                    }
                }
                let fix = match c {
                    Cond::Const(true) => None,
                    Cond::Const(false) => {
                        self.exit_to(fall);
                        return;
                    }
                    Cond::Host(cnd) => Some(self.a.bcond(cnd)),
                    Cond::Reg(r, inv) => Some(if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) }),
                };
                if let Some(f) = fix {
                    self.exit_to(fall);
                    self.a.bind(f);
                }
                self.exit_code(EXIT_JCC, fall, target);
            }
            Term::JccPair { cc1, t1, cc2, t2, fall2 } => {
                self.flush_flags(live_out);
                self.flush_lb();
                if self.fs.src == Src::Fcmp && (self.fs.host & (FP | FZ)) == (FP | FZ) {
                    if cc1 == 11 {
                        // `jp t1; je/jne t2; fall2`: V==1 -> t1; else V==0 so ZF == Z.
                        let f1 = self.a.bcond(VS);
                        self.cond_links.push((f1, t1));
                        let f2 = self.a.bcond(if cc2 == 5 { EQ } else { NE });
                        self.cond_links.push((f2, t2));
                        self.exit_to(fall2);
                    } else {
                        // `jnp t1; je/jne t2; fall2`: V==0 -> t1; else V==1 (unordered, ZF==1).
                        let f1 = self.a.bcond(VC);
                        self.cond_links.push((f1, t1));
                        self.exit_to(if cc2 == 5 { t2 } else { fall2 });
                    }
                } else {
                    // Fallback if flags were materialized before the terminator.
                    let c1 = self.cond(cc1);
                    match c1 {
                        Cond::Const(true) => {
                            self.exit_to(t1);
                            return;
                        }
                        Cond::Const(false) => {}
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, t1));
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, t1));
                        }
                    }
                    let c2 = self.cond(cc2);
                    match c2 {
                        Cond::Const(true) => self.exit_to(t2),
                        Cond::Const(false) => self.exit_to(fall2),
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, t2));
                            self.exit_to(fall2);
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, t2));
                            self.exit_to(fall2);
                        }
                    }
                }
            }
            Term::Jmp(t) => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                if self.lazy && !self.target_host {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                if self.target_host {
                    if let Some((h, bail, needs_xmm)) = inline_host_fn_kind(t) {
                        self.plain_call(h, t, lbk, true, bail.then_some((t, fall)), 0, needs_xmm);
                    } else if let Some((slot, v, h, bail, needs_xmm)) = iat_slot_plain(t) {
                        self.mov_lit(XA, slot);
                        self.a.ldr(64, T0, XA, 0);
                        self.mov_slot_val(XS, v, slot);
                        self.a.addsub_rr(true, true, true, ZR, T0, XS, 0); // cmp x0, x17
                        let slow = self.a.bcond(NE);
                        self.plain_call(h, v, lbk, true, bail.then_some((v, fall)), slot, needs_xmm);
                        self.a.bind(slow);
                        if self.host_xfer != 0 {
                            self.xfer_to(t, fall, 1);
                        } else {
                            self.exit_code(EXIT_JMP, fall, t);
                        }
                    } else if !plaincall_enabled()
                        && self.host_xfer != 0
                        && t != 0
                        && !is_chkstk(t)
                        && !super::is_routed_dll_chkstk(t)
                        && !super::is_routed_dll_stack_pos(t)
                        && !super::is_routed_regfile(t)
                        && !super::is_routed_dll_stub(t)
                    {
                        self.mov_lit(T0, t);
                        self.a.b_to(self.lookup_jmp);
                    } else if self.host_xfer != 0 {
                        self.xfer_to(t, fall, 1);
                    } else {
                        self.exit_code(EXIT_JMP, fall, t);
                    }
                } else {
                    self.exit_to(t);
                }
            }
            Term::Call(t) => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                if self.lazy && !self.target_host {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                if self.target_host {
                    if let Some((h, bail, needs_xmm)) = inline_host_fn_kind(t) {
                        self.plain_call(h, t, lbk, false, bail.then_some((t, fall)), 0, needs_xmm);
                        self.exit_to(fall);
                    } else if let Some((slot, v, h, bail, needs_xmm)) = iat_slot_plain(t) {
                        self.mov_lit(XA, slot);
                        self.a.ldr(64, T0, XA, 0);
                        self.mov_slot_val(XS, v, slot);
                        self.a.addsub_rr(true, true, true, ZR, T0, XS, 0); // cmp x0, x17
                        let slow = self.a.bcond(NE);
                        self.plain_call(h, v, lbk, false, bail.then_some((v, fall)), slot, needs_xmm);
                        self.exit_to(fall);
                        self.a.bind(slow);
                        if self.host_xfer != 0 {
                            self.xfer_to(t, fall, 0);
                        } else {
                            self.exit_code(EXIT_CALL, fall, t);
                        }
                    } else if !plaincall_enabled()
                        && self.host_xfer != 0
                        && t != 0
                        && !is_chkstk(t)
                        && !super::is_routed_dll_chkstk(t)
                        && !super::is_routed_dll_stack_pos(t)
                        && !super::is_routed_regfile(t)
                        && !super::is_routed_dll_stub(t)
                    {
                        self.mov_lit(T0, t);
                        self.call_lookup(fall);
                    } else if self.host_xfer != 0 {
                        self.xfer_to(t, fall, 0);
                    } else {
                        self.exit_code(EXIT_CALL, fall, t);
                    }
                } else {
                    if pushpop_enabled() {
                        // Plain pre-indexed return-address push (see `pushpop_enabled`).
                        self.mov_lit(T0, fall);
                        self.a.str_pre(T0, g(4), -8);
                    } else {
                        self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                        self.mov_lit(T0, fall);
                        self.store(k, T0, XA, 64, Chk::Stack);
                        self.a.mov_rr(true, g(4), XA);
                    }
                    if self.ras_miss != 0 {
                        // RAS entry (fall, landing pad), then `bl` into the callee (the
                        // landing pad is the word after the `bl`, so the CPU's return
                        // predictor agrees with the RAS).
                        let adr = self.a.adr(T1);
                        ras_push(&mut self.a, T0, T1);
                        let site = self.a.bl();
                        self.links.push((site, t));
                        let land = self.a.pos;
                        self.a.patch_adr(adr, land);
                    }
                    self.exit_to(if self.ras_miss != 0 { fall } else { t });
                }
            }
            Term::CallR | Term::JmpR => {
                self.flush_flags(live_out);
                self.flush_lb();
                self.a.mov_rr(true, T0, g(op.r0));
                if matches!(term, Term::CallR) {
                    self.call_lookup(fall);
                } else {
                    self.a.b_to(self.lookup_jmp);
                }
            }
            Term::CallM | Term::JmpM => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                self.flush_lb();
                let is_call = matches!(term, Term::CallM);
                // `call/jmp [rip+disp]` through an import slot holding an inline-callable target:
                // inline call guarded by the slot still holding the translation-time value.
                let mut generic = None;
                if let Some((slot, v, h, bail, needs_xmm)) = const_slot_plain(&op) {
                    self.mov_lit(XA, slot);
                    self.a.ldr(64, T0, XA, 0);
                    self.mov_slot_val(XS, v, slot);
                    self.a.addsub_rr(true, true, true, ZR, T0, XS, 0); // cmp x0, x17
                    generic = Some(self.a.bcond(NE));
                    self.plain_call(h, v, lbk, !is_call, bail.then_some((v, fall)), slot, needs_xmm);
                    if is_call {
                        self.exit_to(fall);
                    }
                    self.a.bind(generic.unwrap());
                }
                if generic.is_none() {
                    self.load_op(k, T0, &op, 64);
                }
                if is_call {
                    self.call_lookup(fall);
                } else {
                    self.a.b_to(self.lookup_jmp);
                }
            }
            Term::Ret => {
                self.flush_flags(live_out);
                self.flush_lb();
                if !self.ret_preloaded {
                    if op.imm == 0 {
                        self.a.ldr_post(T0, g(4), 8);
                    } else {
                        self.a.ldr(64, T0, g(4), 0);
                        self.a.add_imm(true, g(4), g(4), 8 + op.imm as i64);
                    }
                }
                if self.ras_miss != 0 {
                    // Pop pinned `XRAS` top if its guest address matches the return target and
                    // return to its landing pad; otherwise `ras_miss` (x0 = target, XRAS = top).
                    if !self.ret_preloaded {
                        self.a.ldp_x(1, 2, XRAS, 0);
                    }
                    self.a.addsub_ri(true, true, false, XA, XRAS, 16, false);
                    self.a.logic_rr(true, 2, 1, 1, T0, 0, 0); // eor x1, x1, x0
                    let miss = self.a.cbnz(true, 1);
                    self.a.bfxil0(XRAS, XA, 16);
                    self.a.ret_r(2);
                    self.a.bind(miss);
                    self.a.b_to(self.ras_miss);
                } else {
                    self.a.b_to(self.lookup_rip);
                }
            }
            Term::Loop { sub, target, fall } => {
                self.flush_flags(live_out | if sub >= 3 { FZ } else { 0 });
                if sub >= 2 {
                    self.set_lb(1, 64);
                }
                self.flush_lb();
                if sub <= 1 {
                    let taken = self.a.cbz(sub == 0, g(1));
                    self.exit_to(fall);
                    self.a.bind(taken);
                    self.exit_to(target);
                } else {
                    self.a.addsub_ri(true, true, false, g(1), g(1), 1, false);
                    let zero = self.a.cbz(true, g(1));
                    let zf_fail = if sub == 3 {
                        self.a.ldr(8, T0, XCPU, OFF_ZF);
                        Some(self.a.cbz(false, T0))
                    } else if sub == 4 {
                        self.a.ldr(8, T0, XCPU, OFF_ZF);
                        Some(self.a.cbnz(false, T0))
                    } else {
                        None
                    };
                    self.exit_to(target);
                    self.a.bind(zero);
                    if let Some(f) = zf_fail {
                        self.a.bind(f);
                    }
                    self.exit_to(fall);
                }
            }
        }
    }
}

include!("jit_vec.rs");
include!("jit_ext.rs");
include!("jit_xmm.rs");
include!("jit_lazy.rs");
include!("jit_aot.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::exec_mem::{
        bind_static_aot_module, register_static_aot_desc, register_static_aot_module,
        static_aot_gen, test_reset_static_lookups, test_static_lookups,
        unregister_static_aot_for_test, StaticAotEntry, StaticAotModuleDesc,
        STATIC_AOT_DESC_MAGIC, STATIC_AOT_DESC_VERSION, STATIC_AOT_FLAG_HOST_CALLABLE,
    };

    #[test]
    fn static_only_negative_cache_and_rebind_invalidation() {
        let entries_v1 = [
            StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
            StaticAotEntry { rva: 0x2000, code_offset: 0x200, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
        ];
        register_static_aot_module("jit_neg_test.dll", 0x7_0000_0000, &entries_v1);
        assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);

        let mut bc = Box::new(BlockCache::new());
        let mut jc = JitCache::new();
        jc.test_set_static_only(true);
        assert!(jc.meta.is_empty(), "static_only must skip the 4 MiB meta slab");
        let m_static = jc.mode_tag();
        assert_ne!(m_static, 0);

        let hit_ip = 0x3_0000_1000u64;
        let host_ip = 0x3_0000_2000u64;
        let miss_ip = 0x3_0000_3000u64;

        unsafe {
            test_reset_static_lookups();
            // Positive hit populates index; repeat lookups do not hit registry again.
            assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
            assert_eq!(test_static_lookups(), 1);
            assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
            assert_eq!(test_static_lookups(), 1);
            assert!(!jc.is_interp_sticky(hit_ip));

            // Host-callable entry caches code != 0 with fast == 0.
            assert_eq!(jc.lookup(host_ip, &mut *bc), Some(0x7_0000_0200));
            assert_eq!(test_static_lookups(), 2);
            let h_ent = jc.index[jslot(host_ip)];
            assert_eq!(h_ent.ip, host_ip);
            assert_eq!(h_ent.code, 0x7_0000_0200);
            assert_eq!(h_ent.fast, 0);

            // Miss populates a NOJIT negative entry; repeated lookups MUST NOT re-query registry.
            assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
            assert_eq!(test_static_lookups(), 3);
            let neg = jc.index[jslot(miss_ip)];
            assert_eq!(neg.ip, miss_ip);
            assert_eq!(neg.code, 0);
            assert_eq!(neg.fast, 0);
            assert_eq!(neg.hits, NOJIT);
            assert!(jc.is_interp_sticky(miss_ip));

            for _ in 0..16 {
                assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
            }
            assert_eq!(
                test_static_lookups(),
                3,
                "negative cache must eliminate repeated lookup_static_aot calls"
            );

            // Rebinding a new module containing `miss_ip` bumps STATIC_AOT_GEN, invalidates
            // `is_interp_sticky`, flushes the negative cache, and resolves the newly bound block.
            let g_before = static_aot_gen();
            let entries_v2 = [
                StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
                StaticAotEntry { rva: 0x3000, code_offset: 0x300, flags: 0 },
            ];
            register_static_aot_module("jit_neg_test.dll", 0x7_1000_0000, &entries_v2);
            assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);
            assert_ne!(static_aot_gen(), g_before);
            assert!(!jc.is_interp_sticky(miss_ip));

            assert_eq!(jc.lookup(miss_ip, &mut *bc), Some(0x7_1000_0300));
            assert_eq!(test_static_lookups(), 4);
            assert!(!jc.is_interp_sticky(miss_ip));

            // Unbinding bumps generation and flushes cached entries back to None.
            unregister_static_aot_for_test("jit_neg_test.dll");
            assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
            assert_eq!(test_static_lookups(), 5);
            assert!(jc.is_interp_sticky(miss_ip));
        }
    }

    #[test]
    fn no_jit_transitions_to_static_only_on_late_bind_and_skips_meta() {
        let mut bc = Box::new(BlockCache::new());
        let mut jc = JitCache::new();
        // Simulate `!jit_writable()` with no static AOT module bound yet.
        jc.no_jit = true;
        jc.static_aot_gen = static_aot_gen();
        let mode_before = jc.mode_tag();

        let ip = 0x3_8000_1000u64;
        assert!(jc.is_interp_sticky(ip));
        unsafe {
            test_reset_static_lookups();
            // Steady-state no_jit without static modules must not touch static registry or init().
            for _ in 0..8 {
                assert_eq!(jc.lookup(ip, &mut *bc), None);
            }
            assert_eq!(test_static_lookups(), 0);
            assert!(jc.mem.is_null());
            assert!(!jc.static_only);
        }

        // Bind a static module with a signed enter/lookup_rip stub descriptor.
        let name = std::ffi::CString::new("late_bind.dll").unwrap();
        let entries = [StaticAotEntry { rva: 0x1000, code_offset: 0x800, flags: 0 }];
        let desc = StaticAotModuleDesc {
            magic: STATIC_AOT_DESC_MAGIC,
            version: STATIC_AOT_DESC_VERSION,
            knobs: 0,
            image_base: 0x3_8000_0000,
            image_size: 0x1_0000,
            image_hash: [1, 2],
            code_base: 0x8_0000_0000,
            code_len: 0x4000,
            enter_off: 0x40,
            lookup_rip_off: 0x100,
            flags: 0,
            entry_count: 1,
            _pad: 0,
            entries: entries.as_ptr(),
            module_name: name.as_ptr(),
        };
        assert!(unsafe { register_static_aot_desc(&desc) });
        assert_eq!(bind_static_aot_module("late_bind.dll", 0x3_8000_0000, 0x1_0000), 1);

        // Generation bumped: `is_interp_sticky` drops immediately, and `lookup` transitions
        // `JitCache` into `static_only` without allocating `self.meta`.
        assert!(!jc.is_interp_sticky(ip));
        unsafe {
            assert_eq!(jc.lookup(ip, &mut *bc), Some(0x8_0000_0800));
            assert!(jc.static_only);
            assert!(jc.meta.is_empty());
            assert_eq!(jc.enter, 0x8_0000_0040);
            assert_eq!(jc.lookup_rip, 0x8_0000_0100);
            assert_ne!(jc.mode_tag(), mode_before);
        }

        // Unbinding drops back out of `static_only` cleanly.
        unregister_static_aot_for_test("late_bind.dll");
        unsafe {
            assert_eq!(jc.lookup(ip, &mut *bc), None);
            assert!(!jc.static_only);
            assert_eq!(jc.enter, 0);
            assert!(jc.is_interp_sticky(ip));
        }
    }
}
