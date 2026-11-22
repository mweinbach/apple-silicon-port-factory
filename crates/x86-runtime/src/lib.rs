//! Universal native x86-64 -> ARM64 execution runtime.
//!
//! Houses all game-agnostic execution machinery extracted from `aw2-game`:
//! - [`emu`]: `iced-x86` interpreter, `FastOp` IR, ARM64/NEON block JIT, persistent `.aot` cache, `DF_REGFILE` tier
//! - [`exe`]: Lock-free `VA_DISPATCH`, TSO unaligned memory ops, TEB/TLS helpers, `call_va*`, `try_call`
//! - [`callctx`], [`lstack`], [`simd_ops`], [`lift_ops`]: Lifted-code ABI, `LiftedFrame`, NEON/TSO/LSE atomics
//! - [`eh_rt`]: Functional MSVC FH4/SEH/setjmp unwinding across lifted, emulated, and host frames
//! - [`dll_native`]: Generic PE IAT/RO/`__chkstk`/`stack_pos` scanners, forwarder aliasing, and whole-DLL lift runtime
//! - [`shadow`], [`difftest`], [`emu_prof`], [`callprof`], [`jobprof`], [`pc_sample`]: Diagnostics & differential oracles
//! - [`hooks`]: Per-game extension hooks (float-return VAs, lean host targets, DLL patchers, lifted images)
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(test, allow(linker_messages))]

#[macro_use]
pub mod macros;

win64_abi::win64_mods! {
    pub mod callctx;
    pub mod callprof;
    pub mod diag;
    pub mod difftest;
    pub mod dll_native;
    pub mod eh_rt;
    pub mod emu;
    pub mod emu_prof;
    pub mod exe;
    pub mod hooks;
    pub mod jobprof;
    pub mod lift_ops;
    pub mod lstack;
    pub mod pc_sample;
    pub mod registry_types;
    pub mod shadow;
    pub mod simd_ops;
}
