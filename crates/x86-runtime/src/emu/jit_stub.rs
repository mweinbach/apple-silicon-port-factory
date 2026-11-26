//! Non-aarch64 stand-in for the x86-64 -> arm64 block translator (`emu_jit.rs`): the
//! translator is never enabled, the interpreter runs everything.

use super::{BlockCache, Cpu};

/// Fields of [`super::Cpu`] used by translated code (unused here).
#[derive(Clone, Copy)]
#[repr(C)]
pub(super) struct JitScratch {
    pub budget: i64,
    pub target: u64,
    pub table: usize,
    pub mul: u64,
    pub codegen: usize,
    pub teb: u64,
    pub ctx_sp: usize,
    pub ctx_blocks: usize,
    pub ctx_prof: usize,
    pub ctx_entry: u64,
    pub ctx_vagen: u64,
    pub ras: u64,
    pub ctx_ras_cell: usize,
    pub aot_dir: usize,
    pub aot_probe_hits: u64,
    pub _pad_xmm: u8,
    /// Maintain `Cpu::last_bits` in `fast::Cpu::rset` (mirrors `PORT_JIT_LAST_BITS`).
    pub lb_on: u8,
    pub ymm_dirty: u16,
    pub mxcsr: u32,
    /// Embedding thread FPCR for native calls made directly by generated code.
    pub host_fpcr: u64,
    pub x87_state: usize,
    pub x87_helpers: usize,
}

impl Default for JitScratch {
    fn default() -> Self {
        Self {
            budget: 0,
            target: 0,
            table: 0,
            mul: 0,
            codegen: 0,
            x87_state: 0,
            x87_helpers: 0,
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
            lb_on: 1,
            ymm_dirty: 0,
            mxcsr: 0x1F80,
            host_fpcr: 0,
        }
    }
}

pub(super) const EXIT_RIP: u64 = 0;
pub(super) const EXIT_CALL: u64 = 1;
pub(super) const EXIT_JMP: u64 = 2;
pub(super) const EXIT_JCC: u64 = 3;
pub(super) const EXIT_SIDE: u64 = 4;

pub(super) fn force(_mode: u8) {}

pub(super) fn enabled() -> bool {
    false
}

#[allow(dead_code)]
pub(super) fn inline_host_fn_kind(_t: u64) -> Option<(usize, bool, bool)> {
    None
}

#[allow(dead_code)]
pub(super) fn inline_host_fn(_t: u64) -> Option<(usize, bool)> {
    None
}

pub(super) struct JitCache;

impl JitCache {
    pub(super) fn new() -> Self {
        JitCache
    }

    pub(super) unsafe fn warm(&mut self) {}

    #[allow(dead_code)]
    #[inline(always)]
    pub(super) unsafe fn cache_plain_host(
        _table: usize,
        _target: u64,
        _host_fn: usize,
        _needs_xmm: bool,
    ) {
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(super) unsafe fn cache_qpc_host(_table: usize, _target: u64, _qpc_state: usize) {}

    #[inline(always)]
    pub(super) unsafe fn lookup_fast_hit(&self, _ip: u64) -> Option<usize> {
        None
    }

    #[inline(always)]
    pub(super) unsafe fn lookup(&mut self, _ip: u64, _bc: *mut BlockCache) -> Option<usize> {
        None
    }

    #[inline(always)]
    pub(super) fn owns_code(&self, _code: usize) -> bool {
        false
    }

    pub(super) unsafe fn run(&mut self, _cpu: &mut Cpu, _code: usize, _budget: i64) -> (u64, i64) {
        unreachable!("block translator is aarch64-only")
    }
}
