//! Portable `x86-64` instruction execution engine (`iced-x86` decoder + Win64 ABI bridge)
//! for calling unlifted PE functions (`DllMain`, CRT init, third-party DLLs, or fallback
//! executable VAs) on non-`x86_64` hosts (`aarch64-apple-darwin` / `aarch64-apple-ios`).
//!
//! Whenever a `call` or `jmp` targets an address explicitly registered in
//! [`crate::exe::lookup_native_va`], execution dispatches directly to the compiled native
//! `aarch64` Rust function (`win32-rt` builtin export thunk, COM vtable method, or registered
//! game replacement). Unregistered host `__TEXT` addresses are never implicitly authorized.
//! Otherwise, it decodes and executes the guest `x86-64` instructions in place against guest
//! memory and the thread's TEB (`gs:[offset]`).
//!
//! Submodule layout:
//! - [`cpu`]: `Cpu` state, flags, memory/atomic accessors, `MXCSR`↔`FPCR` guards, precise faults, scalar SSE helpers
//! - [`routes`]: primary-EXE CRT/thunk routes (`crate::hooks`), DLL IAT/`__chkstk`/`stack_pos`/`DF_REGFILE` routing, `resolve_dispatch`
//! - [`dispatch`]: Win64 ↔ host call bridges (`host_call_asm`, `call_va8`, `lean_call`, `xfer_plain`) and unified `apply_fx`/`jit_host_xfer`
//! - [`block_cache`]: per-thread `BlockCache`, `ThreadEmuState`, guest stack & `TLS_EMU`/`CUR_CPU`
//! - [`interp`]: `emu_run*` entry points, `interp_burst`, and main `emu_run_region` loop
//! - [`generic_ops`]: generic `iced-x86` instruction interpreter (`Cpu::exec_generic_ins`)
//! - [`tsc`]: guest timestamp counter shared with standalone Rust AOT output
//! - [`fast`], [`jit`], [`aot`], [`static_macho`], [`verify`], [`eh`], [`regfile`], [`simd_kernels`], [`simd_mov`], [`simd_fp`]

mod block_cache;
mod aot_strict;
mod cpu;
mod denorm;
mod dispatch;
pub mod eh;
pub mod exec_mem;
mod fast;
mod features;
pub(crate) mod cpuid;
mod generic_ops;
mod interp;
mod mem;
mod mmx;
pub mod regfile;
pub mod regfile_ext;
mod routes;
mod simd_fp;
mod simd_kernels;
mod simd_mov;
mod tsc;
mod xstate;
#[cfg(target_arch = "aarch64")]
mod jit;
#[cfg(not(target_arch = "aarch64"))]
#[path = "jit_stub.rs"]
mod jit;
mod jit_stats;
#[cfg(target_arch = "aarch64")]
pub(crate) mod aot;
#[cfg(target_arch = "aarch64")]
pub mod static_macho;
#[cfg(target_arch = "aarch64")]
pub mod verify;

#[cfg(test)]
#[path = "tests.rs"]
mod sse_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod jit_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod cpu_basics_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod isa_extensions_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod jit_coverage;
#[cfg(all(test, target_arch = "aarch64"))]
mod jit_bench;

// ---------------------------------------------------------------------------------------------
// Public & crate-internal API re-exports (preserving 100% of `crate::emu::*` paths)
// ---------------------------------------------------------------------------------------------

pub use block_cache::{current_guest_state, jit_warm, with_frozen_tsc, zero_guest_stack_unused};
pub(crate) use block_cache::{
    eh_emu_state, eh_set_emu_state, out_split, shadow_prepare, tsc_is_frozen, GUEST_ROOM_MIN,
};

pub use cpu::{
    guest_rdtsc, guest_rdtscp, guest_tsc, precise_faults, set_precise_faults, Cpu,
    PRECISE_FAULT_DISPATCHES,
};
#[cfg(test)]
pub(crate) use cpu::set_precise_faults_local;

pub use tsc::GUEST_TSC;

pub use dispatch::{
    call_host_dispatched, call_host_for_va, call_host_with_xmm, call_va8, peek_incoming_rsp,
    read_caller_extra_args, set_incoming_rsp, set_outgoing_rsp, take_incoming_rsp,
    take_outgoing_rsp,
};
#[cfg(target_arch = "aarch64")]
pub use dispatch::port_static_aot_host_xfer;
#[cfg(target_arch = "aarch64")]
pub(crate) use dispatch::{call_host_dispatched_in, call_lean_in, shadow_lean_xmm, shadow_run_guest, shadow_run_lean};

pub use aot_strict::{
    aot_fallback_report, aot_strict_mode, clear_aot_strict_mode_override,
    enable_translated_accounting, format_aot_fallback_summary, reset_aot_fallback_counters,
    set_aot_strict_mode, translated_accounting_enabled, AotFallbackReport, AotFallbackSite,
    AotStrictMode, EXIT_AOT_FALLBACK,
};

pub use interp::{
    clear_fail_on_unimpl_op_override, emu_run, emu_run_full, emu_run_full_rax, emu_run_with_xmm,
    fail_on_unimpl_op_enabled, parse_env_bool, set_fail_on_unimpl_op, EXIT_UNIMPL_OP,
};
#[cfg(test)]
pub(crate) use interp::{test_last_steps, test_total_steps};

/// Static lowering decision for one decoded operand form. This never emits or executes code.
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstructionLowering {
    Arm64,
    FastInterpreter,
    MmxDispatch,
    GenericDispatch,
}

#[cfg(target_arch = "aarch64")]
pub fn instruction_lowering(ins: &iced_x86::Instruction) -> InstructionLowering {
    let op = fast::lower(ins);
    if op.kind == fast::K_GENERIC {
        if mmx::uses_mmx(ins) {
            return InstructionLowering::MmxDispatch;
        }
        InstructionLowering::GenericDispatch
    } else if jit::supported(&op) {
        InstructionLowering::Arm64
    } else {
        InstructionLowering::FastInterpreter
    }
}

pub use mem::note_code_region;

pub use routes::{
    resolve_dispatch, resolve_host_callable, to_mapped_addr, DF_FLOAT_RET, DF_GAME_REPL, DF_HOST,
    DF_IAT, DF_LEAN, DF_MARK, DF_PLAIN, DF_SHADOW, DF_TRACE, DF_VALID, EXE_ROUTED, EXE_ROUTE_OFF,
    LEAN_FLAGS,
};
#[cfg(target_arch = "aarch64")]
pub(crate) use routes::{resolve_dispatch_in, resolve_dispatch_static};

// ---------------------------------------------------------------------------------------------
// Internal re-exports shared with descendant submodules (`super::*` / `crate::emu::*`)
// ---------------------------------------------------------------------------------------------

use block_cache::{BlockCache, TLS_EMU};
#[cfg(target_arch = "aarch64")]
#[allow(unused_imports)]
use block_cache::{fast_enabled, thunk_elided, verify_enabled, zeroed_slice, BLK_MAX_INSNS};
#[cfg(test)]
#[allow(unused_imports)]
use block_cache::{BlkEntry, ThreadEmuState, TEST_FAST_OFF};

use cpu::{
    apply_guest_mxcsr_hw, cas128_full, sse_arith32, sse_arith64, sse_cmp_pred, sse_cvt_i32,
    sse_cvt_i64, sse_sqrt32, sse_sqrt64, FAULT_PENDING, RET_SENTINEL,
};
#[cfg(not(target_arch = "aarch64"))]
use cpu::{x86_nan32, x86_nan64};
#[cfg(not(test))]
use cpu::{enter_precise_oracle, leave_precise_oracle};
#[cfg(test)]
use cpu::PRECISE_LOCAL;
#[cfg(target_arch = "aarch64")]
#[allow(unused_imports)]
use cpu::{read_fpcr, write_fpcr};
#[cfg(all(test, target_arch = "aarch64"))]
use cpu::FPCR_GUEST_MASK;

#[cfg(target_arch = "aarch64")]
use dispatch::jit_host_xfer;
#[cfg(all(test, target_arch = "aarch64"))]
#[allow(unused_imports)]
use dispatch::lean_call;
#[cfg(test)]
#[allow(unused_imports)]
use dispatch::{host_call_asm, note_test_xfer_plain, test_xfer_plain, TEST_PLAIN_HOST_CALLS};

#[cfg(test)]
#[allow(unused_imports)]
use interp::{TEST_INTERP_BURSTS, TEST_INTERP_BURST_CHAINS};

use routes::lookup_routed_regfile;
#[cfg(target_arch = "aarch64")]
use routes::{
    inline_variant, is_routed_dll_chkstk, is_routed_dll_stack_pos, is_routed_dll_stub,
    is_routed_regfile,
};
#[cfg(all(test, target_arch = "aarch64"))]
#[allow(unused_imports)]
use routes::{dll_chkstk_routed, dll_routed, dll_stack_pos_routed};
#[cfg(test)]
#[allow(unused_imports)]
use routes::{dll_stub_route, DLL_CHKSTK_OFF, DLL_STACK_POS_OFF};

#[cfg(all(test, target_arch = "aarch64"))]
use mod_tests::test_translate;

use iced_x86::{Instruction, Mnemonic, OpKind, Register};
#[cfg(test)]
#[allow(unused_imports)]
use iced_x86::{ConditionCode, Decoder, DecoderOptions, EncodingKind, FlowControl};
#[cfg(test)]
#[allow(unused_imports)]
use crate::exe::{base, lookup_native_va, rd, teb_self, wr};

// ---------------------------------------------------------------------------------------------
// Null-call hook & AOT / JIT host control entry points
// ---------------------------------------------------------------------------------------------

/// Difftest hook (`dll_lift` fuzz children only; 0 = none): called on every guest `call` to
/// address 0, which the emulator turns into `rax = 0` and goes on (natively a fault), on a run
/// that ends at rip 0 (a `jmp`/`ret` to 0) and on a run started at 0 (`call_va*(0)`), so the
/// harness can tell runs that went on past one (see `dll_native::lift::diff`, "null_call").
pub static NULL_CALL_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cold]
#[inline(never)]
fn note_null_call() {
    let h = NULL_CALL_HOOK.load(std::sync::atomic::Ordering::Relaxed);
    if h != 0 {
        let f: fn() = unsafe { core::mem::transmute(h) };
        f();
    }
}

/// AOT translation cache (`AW2_DLL_AOT`, `aot.rs`): PE image `name` was mapped, relocated
/// and import-bound at `[base, base + size)` (also feeds the `AW2_JIT_PROFILE` dump).
pub fn aot_module_mapped(name: &str, base: usize, size: usize) {
    #[cfg(target_arch = "aarch64")]
    aot::module_mapped(name, base, size);
    #[cfg(not(target_arch = "aarch64"))]
    let _ = (name, base, size);
}

/// Enable hardware `FEAT_LSE2` `SIGBUS` alignment-trap recovery in translated code
/// (`PORT_JIT_LSE2_TRAP`): called once by `port-loader::signals::install_handlers()` after
/// installing the `SIGBUS` handler that emulates 16-byte-boundary-crossing `ldapr`/`ldapur`/
/// `stlr`/`stlur` faults.
pub fn enable_lse2_trap() {
    #[cfg(target_arch = "aarch64")]
    jit::enable_lse2_trap();
}

/// Enable hardware MMU range checking in translated code (`PORT_JIT_HW_RANGE`): called once
/// by `port-loader::signals::install_handlers()`. Eliminates software `[0x10000, 2^47)`
/// range checks (`sub`/`lsr`/`cbnz`) by relying on Darwin's 4 GiB `__PAGEZERO` + 47-bit
/// user VA space + signal handler.
pub fn enable_hw_range() {
    #[cfg(target_arch = "aarch64")]
    jit::enable_hw_range();
}

/// Async-signal-safe, lock-free self-patching of a faulting unaligned `ldapr`/`ldapur`/
/// `stlr`/`stlur` inside an executing `MAP_JIT` AOT/JIT region to a 16-byte exact-TSO island.
/// Returns `true` if the site was patched (or already patched by another thread).
#[cfg(target_arch = "aarch64")]
pub unsafe fn patch_lse2_unaligned(pc: usize, insn: u32) -> bool {
    jit::patch_lse2_unaligned(pc, insn)
}

/// AOT translation cache activation point (loader: after the registry is applied, before
/// DllMains / difftests run): load the caches of the images mapped since the last call, or
/// build them and exit (`AW2_DLL_AOT_BUILD`).
pub fn aot_activate() {
    #[cfg(target_arch = "aarch64")]
    aot::activate();
}

/// Merge the `PORT_JIT_PROFILE` / `AW2_JIT_PROFILE` block profile recorded so far into the cache directory.
pub fn aot_profile_dump() {
    #[cfg(target_arch = "aarch64")]
    aot::profile_dump();
}

#[cfg(target_arch = "aarch64")]
pub use aot::{
    aot_cached_preferred_base, aot_prewarm_all_mapped, discover_all_callback_roots,
    disk_aot_disabled, flush_runtime_callbacks, image_fingerprint, record_runtime_callback,
    set_disk_aot_disabled, AotPrewarmReport, CallbackKind, CallbackRoot, ImageFingerprint,
};

/// Permanently disable disk-sourced native AOT caches (no-op on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn set_disk_aot_disabled(_disabled: bool) {}

/// True when disk-sourced native AOT caches are disabled (always true on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn disk_aot_disabled() -> bool {
    true
}

/// True if guest block `ip` is currently backed by a live AOT translation (dynamic or static).
#[cfg(target_arch = "aarch64")]
pub fn aot_has_block(ip: u64) -> bool {
    aot::find(ip).is_some()
}

/// True if guest block `ip` is currently backed by a live AOT translation (always false on non-aarch64).
#[cfg(not(target_arch = "aarch64"))]
pub fn aot_has_block(_ip: u64) -> bool {
    false
}

/// Look up the compiled `image_base` of a matching dynamic `.aot` cache (no-op on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn aot_cached_preferred_base(_name: &str, _image_size: usize) -> Option<usize> {
    None
}

/// Outcome of profile-free offline `.aot` block-cache precompilation for one mapped PE module.
#[cfg(not(target_arch = "aarch64"))]
#[derive(Debug, Clone)]
pub struct AotPrewarmReport {
    pub module: String,
    pub roots: usize,
    pub blocks: usize,
    pub code_bytes: usize,
    pub cache_path: Option<std::path::PathBuf>,
    pub skipped_reason: Option<String>,
}

/// Snapshot of execution-tier counters for boot reports (`port-runner --report-json`).
#[derive(Debug, Clone, Default)]
pub struct TierStats {
    pub exec_mode: &'static str,
    pub decoded_interp_blocks: u64,
    pub jit_blocks_compiled: u64,
    pub aot_active: bool,
    pub aot_kills: u64,
    pub aot_revived: u64,
    pub static_aot_modules: u64,
    pub static_aot_blocks: u64,
    pub static_aot_bytes: u64,
}

/// Capture process-wide execution tier counters without mutating emulator state.
pub fn tier_stats() -> TierStats {
    #[cfg(target_arch = "aarch64")]
    {
        let mode = match exec_mem::exec_mode() {
            exec_mem::ExecMode::Auto => "auto",
            exec_mem::ExecMode::JitWritable => "jit_writable",
            exec_mem::ExecMode::StaticAotOnly => "static_aot_only",
            exec_mem::ExecMode::InterpreterOnly => "interpreter_only",
        };
        let aot_c = aot::counts();
        let (s_mods, s_blocks, s_bytes) = exec_mem::static_aot_stats();
        TierStats {
            exec_mode: mode,
            decoded_interp_blocks: block_cache::decoded_blocks_count(),
            jit_blocks_compiled: jit::total_jit_blocks_compiled(),
            aot_active: aot::active(),
            aot_kills: aot_c.kills,
            aot_revived: aot_c.revived,
            static_aot_modules: s_mods as u64,
            static_aot_blocks: s_blocks as u64,
            static_aot_bytes: s_bytes as u64,
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        TierStats {
            exec_mode: "interpreter_only",
            decoded_interp_blocks: block_cache::decoded_blocks_count(),
            ..TierStats::default()
        }
    }
}

/// Profile-free offline `.aot` block-cache precompiler (no-op on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn aot_prewarm_all_mapped(_max_roots_per_module: usize) -> Vec<AotPrewarmReport> {
    Vec::new()
}
