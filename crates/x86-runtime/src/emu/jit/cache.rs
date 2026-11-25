//! Per-thread ARM64 JIT translation cache (`JitCache`), LSE2 unaligned-access patch registry,
//! block analysis (`Prep`, `prep_block`), successor liveness (`succ_need`), and host inline lookup.

use super::aot::AotReloc;
use super::emit::Asm;
use super::lazy::{flaglazy_enabled, succ_lb_kill, Edges, FLAGLAZY_DEPTH, PF_NONE};
use super::stubs::{emit_stubs, plaincall_enabled};
use super::translate::{
    classify, op_flags, supported, FlagState, Grp, SideExit, SlowMem, Src, SysCold, Term, Tr,
};
use super::xmm::{fast_nan_enabled, retlit_enabled, xmm_pin_enabled, xmmreg_enabled, ColdNan};
use super::*;
use crate::emu::exec_mem::{alloc_jit, free_jit, icache_invalidate, jit_writable, jit_write_protect};
use crate::emu::fast::{self, FastOp, RNONE};
use crate::emu::{mem, to_mapped_addr, BlockCache, Cpu, BLK_MAX_INSNS};

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

/// Fast-path GPR write bitmask derived directly from pre-decoded [`FastOp`] metadata without
/// invoking `iced_x86::InstructionInfoFactory::info`. Returns `None` for rare/implicit-register
/// kinds so callers fall back to [`gpr_writes`].
#[inline]
fn fast_gpr_writes(op: &FastOp, ins: &iced_x86::Instruction) -> Option<u16> {
    use fast::*;
    let r0 = 1u16 << (op.r0 & 15);
    Some(match op.kind {
        K_NOP => {
            // `rol r, 0` and `rdsspd`/`rdsspq` can also lower to `K_NOP`; only short-circuit true `nop`.
            if ins.mnemonic() == iced_x86::Mnemonic::Nop {
                0
            } else {
                return None;
            }
        }
        K_MOV_MR
        | K_MOV_MI
        | K_ALU_MR
        | K_ALU_MI
        | K_CMPJ_RR
        | K_CMPJ_RI
        | K_CMPJ_RM
        | K_JCC
        | K_JMP
        | K_JMP_R
        | K_JMP_M
        | K_VSHUF_RR
        | K_VSHUF_RM
        | K_VINT_RR
        | K_VINT_RM
        | K_VFP_RR
        | K_VFP_RM
        | K_VST
        | K_MOVD_XG
        | K_CVTSI_R
        | K_CVTSI_M
        | K_COMI_RR
        | K_COMI_RM
        | K_INC_M
        | K_DEC_M
        | K_VZU
        | K_BT_RR
        | K_BT_RI
        | K_SETCC_M
        | K_FLAG_OP
        | K_SHIFT_MI
        | K_SHIFT_MCL
        | K_ROT_MI
        | K_ROT_MCL
        | K_MOVBE_MR
        | K_VBROADCAST
        | K_PMOVX
        | K_VCVT_PD
        | K_VFMA_RR
        | K_VFMA_RM
        | K_VBLENDV_RR
        | K_VBLENDV_RM
        | K_VTEST_RR
        | K_VTEST_RM
        | K_YMM_LANE
        | K_VAES_RR
        | K_VAES_RM
        | K_VCVT_PH
        | K_VMASKMOV
        | K_VSHA_RR
        | K_VSHA_RM => 0,
        K_ALU_RR | K_ALU_RI | K_ALU_RM => {
            if matches!(op.x & 0x3F, A_CMP | A_TEST) {
                0
            } else {
                r0
            }
        }
        K_MOV_RR
        | K_MOV_RM
        | K_MOV_RI
        | K_MOVZX_RR
        | K_MOVZX_RM
        | K_MOVSX_RR
        | K_MOVSX_RM
        | K_LEA
        | K_INC_R
        | K_DEC_R
        | K_SHIFT_RI
        | K_SHIFT_RCL
        | K_CMOV_RR
        | K_CMOV_RM
        | K_SETCC_R
        | K_MOVD_GX
        | K_CVTF2I_R
        | K_CVTF2I_M
        | K_IMUL_RR
        | K_IMUL_RM
        | K_IMUL_RRI
        | K_IMUL_RMI
        | K_NEG_R
        | K_NOT_R
        | K_BSF_RR
        | K_BSF_RM
        | K_ROT_RI
        | K_BSWAP_R
        | K_BSR_RR
        | K_BSR_RM
        | K_TZCNT_RR
        | K_TZCNT_RM
        | K_LZCNT_RR
        | K_LZCNT_RM
        | K_POPCNT_RR
        | K_POPCNT_RM
        | K_BIT_RR
        | K_BIT_RI
        | K_SHD_RI
        | K_SHD_RCL
        | K_ROT_RCL
        | K_MOVBE_RM
        | K_MOVMSK => r0,
        K_BMI_RR | K_BMI_RM => {
            if (op.x & 0x3F) == 10 {
                // `mulx r0, r1, src` writes both destination GPRs.
                r0 | (1u16 << (op.r1 & 15))
            } else {
                r0
            }
        }
        K_PUSH_R | K_PUSH_I | K_PUSH_M | K_POP_M | K_RET | K_CALL | K_CALL_R | K_CALL_M => 1u16 << 4,
        K_POP_R => r0 | (1u16 << 4),
        K_LEAVE => (1u16 << 4) | (1u16 << 5),
        K_XCHG_RR | K_XADD_RR => r0 | (1u16 << (op.r1 & 15)),
        _ => return None,
    })
}

// ---------------------------------------------------------------------------------------------
// Translation cache & LSE2 region registry.
// ---------------------------------------------------------------------------------------------

const CODE_SIZE: usize = 128 << 20;
/// Tail of each `JitCache` `MAP_JIT` region reserved for 16-byte exact-TSO unaligned LSE2
/// self-patching islands (within ±128 MiB of every instruction in the cache).
const JIT_LSE2_SLAB: usize = 256 << 10;
const CODE_LIMIT: usize = CODE_SIZE - JIT_LSE2_SLAB;

/// Lock-free, async-signal-safe registry of `MAP_JIT` code regions and their paired LSE2
/// exact-TSO unaligned-access patch slabs (used by both `JitCache` and `aot`).
pub(super) const MAX_LSE2_REGIONS: usize = 128;

struct Lse2Region {
    /// Lock-free seqlock state (mod 4):
    /// - `0`: FREE (reusable slot)
    /// - `1`: WRITING (claimed by `register_lse2_region`)
    /// - `2`: LIVE (readable by signal handlers)
    /// - `3`: UNREGISTERING (being cleared by `unregister_lse2_region`)
    seq: core::sync::atomic::AtomicUsize,
    code_lo: core::sync::atomic::AtomicUsize,
    code_hi: core::sync::atomic::AtomicUsize,
    slab_cur: core::sync::atomic::AtomicUsize,
    slab_end: core::sync::atomic::AtomicUsize,
}

impl Lse2Region {
    const fn new() -> Self {
        Self {
            seq: core::sync::atomic::AtomicUsize::new(0),
            code_lo: core::sync::atomic::AtomicUsize::new(0),
            code_hi: core::sync::atomic::AtomicUsize::new(0),
            slab_cur: core::sync::atomic::AtomicUsize::new(0),
            slab_end: core::sync::atomic::AtomicUsize::new(0),
        }
    }
}

static LSE2_REGIONS: [Lse2Region; MAX_LSE2_REGIONS] = [const { Lse2Region::new() }; MAX_LSE2_REGIONS];
/// High-water mark of ever-used slots in `LSE2_REGIONS` (`<= MAX_LSE2_REGIONS`).
static LSE2_REGION_N: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
static TOTAL_JIT_BLOCKS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

pub(crate) fn total_jit_blocks_compiled() -> u64 {
    TOTAL_JIT_BLOCKS.load(core::sync::atomic::Ordering::Relaxed)
}

/// Register `[code_lo, code_hi)` with paired patch slab `[slab_lo, slab_hi)` (same `MAP_JIT`
/// mapping, within ±128 MiB). Claims any FREE slot (recycling slots released via
/// [`unregister_lse2_region`]) in a lock-free, async-signal-safe manner.
pub(in crate::emu) fn register_lse2_region(
    code_lo: usize,
    code_hi: usize,
    slab_lo: usize,
    slab_hi: usize,
) -> Option<usize> {
    use core::sync::atomic::Ordering;
    for (idx, r) in LSE2_REGIONS.iter().enumerate() {
        let s = r.seq.load(Ordering::Acquire);
        if (s & 3) != 0 {
            continue;
        }
        if r.seq
            .compare_exchange(s, s.wrapping_add(1), Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            continue;
        }
        r.code_lo.store(code_lo, Ordering::Relaxed);
        r.code_hi.store(code_hi, Ordering::Relaxed);
        r.slab_cur.store(slab_lo, Ordering::Relaxed);
        r.slab_end.store(slab_hi, Ordering::Relaxed);
        LSE2_REGION_N.fetch_max(idx + 1, Ordering::Release);
        // Publish LIVE (state 2) with Release so signal handlers observing `seq == s + 2` see all fields.
        r.seq.store(s.wrapping_add(2), Ordering::Release);
        return Some(idx);
    }
    None
}

/// Unregister a previously registered LSE2 slot so it can be safely recycled by a future
/// thread/module without tearing concurrent async-signal-handler readers.
pub(in crate::emu) fn unregister_lse2_region(idx: usize) {
    use core::sync::atomic::{fence, Ordering};
    if idx >= MAX_LSE2_REGIONS {
        return;
    }
    let r = &LSE2_REGIONS[idx];
    let s = r.seq.load(Ordering::Acquire);
    if (s & 3) != 2 {
        return;
    }
    // Transition LIVE (2) -> UNREGISTERING (3); signal handlers reject any slot whose `seq & 3 != 2`
    // or whose `seq` changed across their read fence.
    r.seq.store(s.wrapping_add(1), Ordering::Release);
    fence(Ordering::SeqCst);
    r.code_hi.store(0, Ordering::Relaxed);
    r.code_lo.store(0, Ordering::Relaxed);
    r.slab_cur.store(0, Ordering::Relaxed);
    r.slab_end.store(0, Ordering::Relaxed);
    // Transition UNREGISTERING (3) -> FREE (0) so a new registration may reclaim this slot.
    r.seq.store(s.wrapping_add(2), Ordering::Release);
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
pub(in crate::emu) unsafe fn patch_lse2_unaligned(pc: usize, insn: u32) -> bool {
    use core::sync::atomic::{fence, AtomicU32, Ordering};
    if !jit_writable() {
        return false;
    }
    let n = LSE2_REGION_N.load(Ordering::Acquire).min(MAX_LSE2_REGIONS);
    let mut found: Option<(&Lse2Region, usize, usize)> = None;
    for r in &LSE2_REGIONS[..n] {
        let s1 = r.seq.load(Ordering::Acquire);
        if (s1 & 3) != 2 {
            continue;
        }
        let hi = r.code_hi.load(Ordering::Relaxed);
        let lo = r.code_lo.load(Ordering::Relaxed);
        let slab_end = r.slab_end.load(Ordering::Relaxed);
        fence(Ordering::SeqCst);
        let s2 = r.seq.load(Ordering::Relaxed);
        if s1 != s2 || hi == 0 {
            continue;
        }
        if pc >= lo && pc < hi {
            found = Some((r, s1, slab_end));
            break;
        }
    }
    let Some((r, seq_live, slab_end)) = found else {
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
    if r.seq.load(Ordering::Acquire) != seq_live {
        return false;
    }
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
pub(super) const MAX_BLOCK_BYTES: usize = 256 << 10;
pub(super) const STUBS_BYTES: usize = 4096;
pub(super) const JBITS: u32 = 18;
const JLEN: usize = 1 << JBITS;
pub(in crate::emu) const JMUL: u64 = 0x9E37_79B9_7F4A_7C15;

/// Translation index entry (layout read by the `lookup_*` stubs).
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub(super) struct JEnt {
    pub(super) ip: u64,
    /// Code usable from translated code (0 if the IP is host-callable).
    pub(super) fast: u64,
    /// 0 for image code; otherwise the code generation the block was validated in.
    pub(super) gen: u32,
    /// Visit counter before translation; [`NOJIT`] if not translatable.
    pub(super) hits: u32,
    /// Code entry for the interpreter (0 = not translated).
    pub(super) code: u64,
}
const _: () = assert!(core::mem::size_of::<JEnt>() == 32);
pub(super) const NOJIT: u32 = u32::MAX;

#[derive(Clone, Copy, Default)]
pub(super) struct JMeta {
    hash: u64,
    len: u32,
}

#[inline(always)]
pub(super) fn jslot(ip: u64) -> usize {
    (ip.wrapping_mul(JMUL) >> (64 - JBITS)) as usize
}

/// Per-thread translation cache.
pub(in crate::emu) struct JitCache {
    pub(super) mem: *mut u8,
    pub(super) live_tsc_mode: TscMode,
    disabled: bool,
    /// True when executing signed static AOT modules from `__TEXT` with zero `MAP_JIT`
    /// allocation (`self.mem == null`, heap `index` + `ras_ring`, signed `self.enter`).
    pub(super) static_only: bool,
    /// True when dynamic `MAP_JIT` is unavailable on this host/policy, preventing
    /// repeated `init()` calls while waiting for a static AOT module to bind.
    pub(super) no_jit: bool,
    /// Last [`crate::emu::exec_mem::static_aot_gen`] observed by this cache (0 = uninitialized).
    pub(super) static_aot_gen: u32,
    #[cfg(test)]
    test_forced_static: bool,
    cursor: usize,
    pub(super) enter: usize,
    exit: usize,
    pub(super) lookup_rip: usize,
    lookup_jmp: usize,
    lookup_call: usize,
    /// Host call/jmp stub ([`crate::emu::jit_host_xfer`]); 0 when `AW2_JIT_HOSTCALL=0`.
    host_xfer: usize,
    /// Shared XMM spill/reload stubs (0 when `PORT_JIT_XMM_PIN=0`) and `clear_ymm_hi` stub.
    spill_xmm: usize,
    reload_xmm: usize,
    clear_ymm_hi: usize,
    rdtsc_slow: usize,
    rdtscp_slow: usize,
    pub(super) index: Box<[JEnt]>,
    pub(super) meta: Box<[JMeta]>,
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
    /// [`crate::emu::aot::epoch`] this cache was last flushed for (AOT module kills).
    aot_epoch: u32,
    /// Slot index in [`LSE2_REGIONS`] for this cache's unaligned-LSE2 patch slab.
    pub(super) lse2_slot: Option<usize>,
    /// Reusable per-thread translation scratch buffers + `iced_x86::InstructionInfoFactory`.
    tr_scratch: TrScratch,
}

impl Drop for JitCache {
    fn drop(&mut self) {
        if let Some(slot) = self.lse2_slot.take() {
            unregister_lse2_region(slot);
        }
        if !self.mem.is_null() {
            unsafe { free_jit(self.mem, CODE_SIZE) };
            self.mem = core::ptr::null_mut();
        }
        if !self.ras_ring.is_null() {
            let layout = std::alloc::Layout::from_size_align(RAS_BYTES, RAS_BYTES).unwrap();
            unsafe { std::alloc::dealloc(self.ras_ring as *mut u8, layout) };
            self.ras_ring = core::ptr::null_mut();
        }
    }
}

/// Shadow return-address stack entries (16 bytes each; the ring is 64 KiB).
const RAS_LEN: usize = 4096;
const RAS_BYTES: usize = RAS_LEN * 16;
const _: () = assert!(RAS_BYTES == 1 << 16);

impl JitCache {
    pub(in crate::emu) fn new() -> Self {
        Self {
            mem: core::ptr::null_mut(),
            live_tsc_mode: TscMode::host(),
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
            rdtsc_slow: 0,
            rdtscp_slow: 0,
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
            tr_scratch: TrScratch::new(),
        }
    }

    /// Ultra-fast read-only probe for a live, already-compiled dynamic JIT/AOT block at `ip`.
    /// Returns `Some(code)` only when `self` is in dynamic JIT mode (no static AOT bound),
    /// all generation counters (`va_gen`, `aot_epoch`, `static_aot_gen`) are current, and
    /// `index[jslot(ip)]` holds a validated entry for `ip`. Because every such entry passed
    /// [`prep_block_with`] or [`crate::emu::aot::check_block`] under the current `va_gen`, callers
    /// can bypass the per-block DLL/regfile/IAT routing checks on a hit.
    #[inline(always)]
    pub(in crate::emu) unsafe fn lookup_fast_hit(&self, ip: u64) -> Option<usize> {
        if self.disabled
            || self.static_only
            || self.mem.is_null()
            || self.enter == 0
            || self.index.is_empty()
            || crate::emu::exec_mem::has_bound_static_aot()
        {
            return None;
        }
        if crate::diag::jit_filter_active() && !crate::diag::jit_allows(ip) {
            return None;
        }
        if self.va_gen != crate::exe::va_gen()
            || self.aot_epoch != crate::emu::aot::epoch()
            || self.static_aot_gen != crate::emu::exec_mem::static_aot_gen()
        {
            return None;
        }
        let e = self.index.get_unchecked(jslot(ip));
        if e.ip == ip && e.code != 0 && (e.gen == 0 || e.gen == mem::code_gen()) {
            Some(e.code as usize)
        } else {
            None
        }
    }

    /// Compact mode tag for [`BlockCache`] fast-chain epoch invalidation.
    #[inline(always)]
    pub(in crate::emu) fn mode_tag(&self) -> u8 {
        (u8::from(self.disabled))
            | (u8::from(self.static_only) << 1)
            | (u8::from(self.no_jit) << 2)
            | (u8::from(!self.mem.is_null()) << 3)
    }

    /// True when guest `ip` is provably sticky-interpreted under the current mode and
    /// static-AOT generation (safe to chain directly in `interp_burst` without re-entering
    /// [`Self::lookup`]).
    #[inline(always)]
    pub(in crate::emu) fn is_interp_sticky(&self, ip: u64) -> bool {
        if self.disabled {
            return true;
        }
        if crate::emu::exec_mem::static_aot_gen() != self.static_aot_gen {
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
    pub(in crate::emu) fn test_set_static_only(&mut self, v: bool) {
        self.test_forced_static = v;
        self.static_only = v;
        self.no_jit = v;
        self.disabled = false;
        if v {
            if let Some(slot) = self.lse2_slot.take() {
                unregister_lse2_region(slot);
            }
            if !self.mem.is_null() {
                unsafe { free_jit(self.mem, CODE_SIZE) };
                self.mem = core::ptr::null_mut();
            }
            self.meta = Box::new([]);
            if self.index.is_empty() {
                self.index = unsafe { crate::emu::zeroed_slice::<JEnt>(JLEN) };
            } else {
                self.index.fill(JEnt::default());
            }
            // Read generation BEFORE reading static stubs so concurrent binds cannot be missed.
            self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
            if let Some((enter, lookup_rip, has_ras)) = crate::emu::exec_mem::static_aot_enter_stub() {
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
    pub(in crate::emu) unsafe fn warm(&mut self) {
        if self.mem.is_null() && !self.static_only && !self.no_jit && !self.disabled {
            self.init();
        }
    }

    #[cold]
    unsafe fn init_static_aot(&mut self) -> bool {
        // Read `static_aot_gen` FIRST (Acquire) so we never cache a newer gen with stale stubs.
        self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
        let Some((enter, lookup_rip, has_ras)) = crate::emu::exec_mem::static_aot_enter_stub() else {
            return false;
        };
        if self.index.is_empty() {
            self.index = crate::emu::zeroed_slice::<JEnt>(JLEN);
        } else {
            self.index.fill(JEnt::default());
        }
        // Static AOT blocks live in immutable signed `__TEXT` (`gen == 0`) and never need
        // dynamic-code revalidation metadata; skip allocating the 4 MiB `self.meta` slab.
        self.va_gen = crate::exe::va_gen();
        self.aot_epoch = crate::emu::aot::epoch();
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
        self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
        if !self.mem.is_null() {
            self.flush();
            return;
        }
        if let Some((enter, lookup_rip, has_ras)) = crate::emu::exec_mem::static_aot_enter_stub() {
            if self.index.is_empty() {
                self.index = crate::emu::zeroed_slice::<JEnt>(JLEN);
            } else {
                self.index.fill(JEnt::default());
            }
            self.va_gen = crate::exe::va_gen();
            self.aot_epoch = crate::emu::aot::epoch();
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
            if crate::emu::exec_mem::exec_mode() == crate::emu::exec_mem::ExecMode::InterpreterOnly {
                self.disabled = true;
                self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
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
            self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
            return false;
        };
        self.static_aot_gen = crate::emu::exec_mem::static_aot_gen();
        self.mem = p;
        self.index = crate::emu::zeroed_slice::<JEnt>(JLEN);
        self.meta = crate::emu::zeroed_slice::<JMeta>(JLEN);
        self.va_gen = crate::exe::va_gen();
        self.aot_epoch = crate::emu::aot::epoch();
        self.hot = hot_threshold();
        let ring = if ras_enabled() {
            // Released in `Drop for JitCache` along with the `MAP_JIT` code buffer.
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
        self.rdtsc_slow = b + st.rdtsc_slow * 4;
        self.rdtscp_slow = b + st.rdtscp_slow * 4;
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
    pub(in crate::emu) unsafe fn lookup(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        if self.disabled {
            return None;
        }
        if crate::diag::jit_filter_active() && !crate::diag::jit_allows(ip) {
            return None;
        }
        if crate::emu::exec_mem::static_aot_gen() != self.static_aot_gen {
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
            if let Some((code, host)) = crate::emu::exec_mem::lookup_static_aot(ip) {
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
    pub(in crate::emu) unsafe fn run(&mut self, cpu: &mut Cpu, code: usize, budget: i64) -> (u64, i64) {
        let vg = crate::exe::va_gen();
        let ep = crate::emu::aot::epoch();
        let sgen = crate::emu::exec_mem::static_aot_gen();
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
                crate::emu::aot::revalidate(vg);
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
        cpu.jx.atomic_locks = crate::emu::cpu::atomic_locks_ptr();
        cpu.jx.sys_helpers = crate::emu::cpu::sys_helpers_ptr();
        cpu.jx.x87_cw = crate::emu::regfile_ext::x87_cw_ptr();
        cpu.jx.tsc_mode = self.live_tsc_mode.for_run();
        cpu.jx.aot_dir = if self.static_only {
            crate::emu::exec_mem::static_probe_table()
        } else if crate::emu::aot::active() {
            crate::emu::aot::probe_table()
        } else {
            0
        };
        // `cpu.jx.ymm_dirty` is maintained incrementally across interpreter and translated writes;
        // skip the 256-byte `cpu.ymm_hi` scan on every `JitCache::run()`.
        #[cfg(debug_assertions)]
        for i in 0..16 {
            debug_assert!(
                cpu.ymm_hi[i] == 0 || (cpu.jx.ymm_dirty & (1u16 << i)) != 0,
                "ymm_dirty stale at reg {i} on JitCache::run(): ymm_hi={:#x}, ymm_dirty={:#x}",
                cpu.ymm_hi[i],
                cpu.jx.ymm_dirty
            );
        }
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
                crate::emu::note_test_xfer_plain(pl);
            }
        }
        (r, budget - cpu.jx.budget)
    }

    /// Offline differential verifier helper: compile a single guest block at `ip` into this
    /// thread's JIT buffer and leave `self.index[jslot(ip)]` empty so running it via
    /// [`Self::run_code_for_verify`] executes strictly one block (even for self-loops).
    pub(in crate::emu) unsafe fn compile_single_for_verify(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
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
    pub(in crate::emu) unsafe fn run_code_for_verify(
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
        cpu.jx.atomic_locks = crate::emu::cpu::atomic_locks_ptr();
        cpu.jx.sys_helpers = crate::emu::cpu::sys_helpers_ptr();
        cpu.jx.x87_cw = crate::emu::regfile_ext::x87_cw_ptr();
        cpu.jx.tsc_mode = self.live_tsc_mode.for_run();
        cpu.jx.aot_dir = 0;
        // Verifier trials (`verify.rs::seed_trial`) seed `cpu.ymm_hi` directly; compute `ymm_dirty` here.
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
    pub(in crate::emu) unsafe fn cache_plain_host(table: usize, target: u64, host_fn: usize, needs_xmm: bool) {
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
    pub(in crate::emu) unsafe fn cache_qpc_host(table: usize, target: u64, qpc_state: usize) {
        let addr = qpc_state as u64;
        if table == 0 || (addr & ((1u64 << 63) | 3)) != 0 {
            return;
        }
        let s = jslot(target);
        let cur = &mut *(table as *mut JEnt).add(s);
        *cur = JEnt { ip: target, fast: addr | (1u64 << 63) | 1, gen: 0, hits: NOJIT, code: 0 };
    }

    #[cfg(test)]
    pub(in crate::emu) fn test_table_ptr(&self) -> usize {
        if self.index.is_empty() { 0 } else { self.index.as_ptr() as usize }
    }

    #[cfg(test)]
    pub(in crate::emu) fn test_slot_entry(&self, target: u64) -> Option<(u64, u64, u32, u32, u64)> {
        if self.index.is_empty() {
            return None;
        }
        let e = self.index[jslot(target)];
        Some((e.ip, e.fast, e.gen, e.hits, e.code))
    }

    #[cfg(test)]
    pub(in crate::emu) fn test_seed_guest_slot(&mut self, ip: u64, code: u64) {
        if self.index.is_empty() {
            self.index = unsafe { crate::emu::zeroed_slice::<JEnt>(JLEN) };
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
        let (code, host) = crate::emu::aot::find(ip)?;
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
                    if crate::emu::jit_stats::enabled() {
                        crate::emu::jit_stats::aot_event(crate::emu::jit_stats::TW_FAR, unlinked.len() as u64);
                    }
                    self.links.insert(ip, unlinked);
                }
            }
        }
        if crate::emu::jit_stats::enabled() {
            crate::emu::jit_stats::aot_event(crate::emu::jit_stats::TW_AOT_INSTALL, 1);
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
            rdtsc_slow: self.rdtsc_slow,
            rdtscp_slow: self.rdtscp_slow,
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
        if crate::emu::jit_stats::enabled() {
            crate::emu::jit_stats::compile(ns, r.is_some());
            if r.is_some() && crate::emu::aot::active() {
                crate::emu::jit_stats::aot_event(crate::emu::jit_stats::TW_AOT_MISS, 1);
            }
        }
        r
    }

    unsafe fn translate_jit(&mut self, ip: u64, bc: *mut BlockCache) -> Option<usize> {
        let s = jslot(ip);
        let Some(p) = prep_block_with(bc, ip, &mut self.tr_scratch.info_factory) else {
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
        let mut tr = new_tr(Asm::new(base, base as usize, MAX_BLOCK_BYTES / 4), ops, &p, &st, false, &mut self.tr_scratch);
        let _main_words = tr.translate(p.term, p.live_out);
        let words = tr.a.pos;
        let overflow = tr.a.overflow;
        let mut links = core::mem::take(&mut tr.links);
        tr.return_scratch(&mut self.tr_scratch);
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
            links.clear();
            self.tr_scratch.links = links;
            jit_write_protect(true);
            log::warn!("emu jit: block at {ip:#x} exceeded the per-block code limit; interpreting it");
            let ent = JEnt { ip, hits: NOJIT, ..JEnt::default() };
            self.index[s] = ent;
            self.compiled.insert(ip, (ent, JMeta::default()));
            return None;
        }
        let code = self.mem as usize + start;
        self.cursor = (start + words * 4 + 15) & !15;
        let mut patched = core::mem::take(&mut self.tr_scratch.patched);
        patched.clear();
        // This block's own direct links (far AOT targets go through a 16-byte branch island).
        let mut far_sites = 0u64;
        for &(site_pos, target) in &links {
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
        links.clear();
        self.tr_scratch.links = links;
        if far_sites != 0 && crate::emu::jit_stats::enabled() {
            crate::emu::jit_stats::aot_event(crate::emu::jit_stats::TW_FAR, far_sites);
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
        for &site in &patched {
            icache_invalidate(site as *mut u8, 4);
        }
        patched.clear();
        self.tr_scratch.patched = patched;
        if gen == 0 {
            crate::emu::aot::profile_note(ip);
        }
        self.translated += 1;
        TOTAL_JIT_BLOCKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
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
pub(super) struct StubAddrs {
    pub(super) exit: usize,
    pub(super) lookup_rip: usize,
    pub(super) lookup_jmp: usize,
    pub(super) lookup_call: usize,
    pub(super) host_xfer: usize,
    pub(super) ras_miss: usize,
    pub(super) spill_xmm: usize,
    pub(super) reload_xmm: usize,
    pub(super) clear_ymm_hi: usize,
    pub(super) rdtsc_slow: usize,
    pub(super) rdtscp_slow: usize,
}

/// Decoded, analysed block (everything [`Tr`] needs besides the stubs and the emission
/// mode): shared by the JIT and AOT builds so both translate a block identically.
pub(in crate::emu) struct Prep {
    pub(in crate::emu) ops: [FastOp; BLK_MAX_INSNS],
    /// Translated prefix length (supported ops).
    pub(in crate::emu) k: usize,
    /// The block's own IP is host-callable (`JEnt::fast` = 0).
    pub(in crate::emu) host: bool,
    pub(super) wr: [u16; BLK_MAX_INSNS],
    pub(in crate::emu) term: Term,
    /// See [`Tr::target_host`].
    pub(super) target_host: bool,
    pub(super) lazy: bool,
    pub(in crate::emu) edge: Edges,
    pub(in crate::emu) live_out: u8,
}

/// Decode and analyse the block at `ip`; None = not translatable (decode error, no
/// supported prefix, `__chkstk`).
pub(in crate::emu) unsafe fn prep_block(bc: *mut BlockCache, ip: u64) -> Option<Prep> {
    let mut f = iced_x86::InstructionInfoFactory::new();
    prep_block_with(bc, ip, &mut f)
}

/// [`prep_block`] reusing a caller-owned `InstructionInfoFactory` (eliminating per-block
/// heap/struct construction) and fast-pathing GPR write masks for common [`FastOp`] kinds.
pub(super) unsafe fn prep_block_with(
    bc: *mut BlockCache,
    ip: u64,
    f: &mut iced_x86::InstructionInfoFactory,
) -> Option<Prep> {
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
        || crate::emu::is_routed_dll_stack_pos(ip)
        || crate::emu::is_routed_regfile(ip)
        || crate::emu::is_routed_dll_stub(ip)
    {
        return None;
    }
    let host = resolved_host(bc, ip);
    // GPR write sets, read before `succ_need` may decode further blocks into the arena
    // (`arena[off + i]` is the decoded instruction of `ops[i]`).
    let mut wr = [0u16; BLK_MAX_INSNS];
    if memchk_mask() & 12 != 0 {
        for (i, w) in wr.iter_mut().enumerate().take(k) {
            let ins = &(&(*bc).arena)[off + i];
            let mask = fast_gpr_writes(&ops[i], ins).unwrap_or_else(|| gpr_writes(ins, f));
            #[cfg(debug_assertions)]
            debug_assert_eq!(
                mask,
                gpr_writes(ins, f),
                "fast_gpr_writes mismatch for kind={} ins={:?}",
                ops[i].kind,
                ins
            );
            *w = mask;
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
                && !crate::emu::is_routed_dll_stack_pos(t)
                && !crate::emu::is_routed_regfile(t)
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

/// Reusable per-thread / per-worker scratch vectors + `InstructionInfoFactory` for block
/// translation (`JitCache::translate_jit` and `aot_translate`), avoiding ~13 fresh heap
/// allocations per translated block.
pub(in crate::emu) struct TrScratch {
    pub(super) info_factory: iced_x86::InstructionInfoFactory,
    pub(super) side_exits: Vec<SideExit>,
    pub(super) slow: Vec<SlowMem>,
    pub(super) links: Vec<(usize, u64)>,
    pub(super) cond_links: Vec<(usize, u64)>,
    pub(super) link_conds: Vec<(usize, usize)>,
    pub(super) ymm_colds: Vec<(usize, usize, u16)>,
    pub(super) tsc_colds: Vec<(usize, usize, bool)>,
    pub(super) sys_colds: Vec<SysCold>,
    pub(super) cold: Vec<ColdNan>,
    pub(super) lits: Vec<(usize, u64)>,
    pub(super) lits128: Vec<(usize, u64, u64)>,
    pub(super) pool: Vec<(u64, usize)>,
    pub(super) pool128: Vec<(u64, u64, usize)>,
    pub(super) rlits: Vec<(usize, u64, AotReloc)>,
    pub(super) relocs: Vec<AotReloc>,
    pub(super) patched: Vec<usize>,
}

impl TrScratch {
    pub(in crate::emu) fn new() -> Self {
        Self {
            info_factory: iced_x86::InstructionInfoFactory::new(),
            side_exits: Vec::new(),
            slow: Vec::new(),
            links: Vec::new(),
            cond_links: Vec::new(),
            link_conds: Vec::new(),
            ymm_colds: Vec::new(),
            tsc_colds: Vec::new(),
            sys_colds: Vec::new(),
            cold: Vec::new(),
            lits: Vec::new(),
            lits128: Vec::new(),
            pool: Vec::new(),
            pool128: Vec::new(),
            rlits: Vec::new(),
            relocs: Vec::new(),
            patched: Vec::new(),
        }
    }
}

/// A fresh translator for `ops` (the prefix of `p.ops`) emitting into `a`; `aot`: AOT
/// emission mode (host-pointer literals always in the pool and recorded as relocations).
/// Borrows reusable vector capacities from `sc`; call [`Tr::return_scratch`] when done.
pub(super) fn new_tr<'a>(
    a: Asm,
    ops: &'a [FastOp],
    p: &Prep,
    st: &StubAddrs,
    aot: bool,
    sc: &mut TrScratch,
) -> Tr<'a> {
    let mut side_exits = core::mem::take(&mut sc.side_exits);
    let mut slow = core::mem::take(&mut sc.slow);
    let mut links = core::mem::take(&mut sc.links);
    let mut cond_links = core::mem::take(&mut sc.cond_links);
    let mut link_conds = core::mem::take(&mut sc.link_conds);
    let mut ymm_colds = core::mem::take(&mut sc.ymm_colds);
    let tsc_colds = core::mem::take(&mut sc.tsc_colds);
    let mut sys_colds = core::mem::take(&mut sc.sys_colds);
    let mut cold = core::mem::take(&mut sc.cold);
    let mut lits = core::mem::take(&mut sc.lits);
    let mut lits128 = core::mem::take(&mut sc.lits128);
    let mut pool = core::mem::take(&mut sc.pool);
    let mut pool128 = core::mem::take(&mut sc.pool128);
    let mut rlits = core::mem::take(&mut sc.rlits);
    let mut relocs = core::mem::take(&mut sc.relocs);
    side_exits.clear();
    slow.clear();
    links.clear();
    cond_links.clear();
    link_conds.clear();
    ymm_colds.clear();
    sys_colds.clear();
    cold.clear();
    lits.clear();
    lits128.clear();
    pool.clear();
    pool128.clear();
    rlits.clear();
    relocs.clear();
    Tr {
        a,
        ops,
        live: [0; BLK_MAX_INSNS + 1],
        fs: FlagState { src: Src::None, host: 0, mem: FALL },
        lb: [LB_NONE; 16],
        side_exits,
        slow,
        links,
        cond_links,
        link_conds,
        cold_code_end: 0,
        exit: st.exit,
        lookup_rip: st.lookup_rip,
        lookup_jmp: st.lookup_jmp,
        lookup_call: st.lookup_call,
        host_xfer: st.host_xfer,
        spill_xmm: st.spill_xmm,
        reload_xmm: st.reload_xmm,
        clear_ymm_hi: st.clear_ymm_hi,
        rdtsc_slow: st.rdtsc_slow,
        rdtscp_slow: st.rdtscp_slow,
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
        ymm_colds,
        tsc_colds,
        sys_colds,
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
        cold,
        retlit: retlit_enabled(),
        lits,
        lits128,
        pool,
        pool128,
        lazy: p.lazy,
        edge: p.edge,
        pf: PF_NONE,
        aot,
        rlits,
        relocs,
    }
}

std::thread_local! {
    /// AOT builds: host-callable decisions consulted while translating the current block
    /// (`(va, host)`), recorded as the block's assumptions; None = not recording.
    pub(super) static RESOLVE_REC: core::cell::RefCell<Option<Vec<(u64, bool)>>> = const { core::cell::RefCell::new(None) };
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
    if crate::emu::jit_stats::enabled() {
        crate::emu::jit_stats::aot_event(crate::emu::jit_stats::TW_PROBE_HIT, n);
    }
}

/// `lookup_*` misses served by the shared AOT probe table on this thread (tests).
#[cfg(test)]
pub(in crate::emu) fn test_probe_hits() -> u64 {
    TEST_PROBE_HITS.with(|c| c.get())
}

/// AOT entries installed into this thread's translation index (tests).
#[cfg(test)]
pub(in crate::emu) fn test_aot_installs() -> u64 {
    TEST_AOT_INSTALLS.with(|c| c.get())
}

/// `BlockCache::resolve(bc, ip).is_some()`, recorded as an assumption while an AOT build
/// translates a block (every host-callable decision that shapes the code goes through here).
#[inline]
pub(super) unsafe fn resolved_host(bc: *mut BlockCache, ip: u64) -> bool {
    let h = BlockCache::resolve(bc, ip).is_some()
        || crate::emu::is_routed_dll_chkstk(ip)
        || crate::emu::is_routed_dll_stack_pos(ip)
        || crate::emu::is_routed_regfile(ip)
        || crate::emu::is_routed_dll_stub(ip);
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

/// Inline-callable host function for the static call/jmp target `t`: `(fn, bail, needs_xmm)`,
/// `fn` = the `DF_PLAIN` builtin itself or [`crate::emu::inline_variant`]'s replacement; `bail` = a 0
/// result means "make the real call" (through `host_xfer`); `needs_xmm` = false for pure-integer
/// `DF_PLAIN` builtins (skip XMM0..3 loads and XMM0 store).
pub(in crate::emu) fn inline_host_fn_kind(t: u64) -> Option<(usize, bool, bool)> {
    if t == 0 || is_chkstk(t) || !plaincall_enabled() {
        return None;
    }
    // Not `resolve_dispatch`: the result is baked into translated code, so a DLL import stub
    // (`DF_IAT`, slot read per call) must not resolve to its current slot target here.
    let (h, f) = crate::emu::resolve_dispatch_static(t);
    if h == 0 {
        return None;
    }
    if f == crate::emu::DF_VALID | crate::emu::DF_HOST | crate::emu::DF_PLAIN {
        return Some((h, false, false));
    }
    if f == crate::emu::DF_VALID | crate::emu::DF_HOST {
        return crate::emu::inline_variant(h).map(|(f2, bail)| (f2, bail, true));
    }
    None
}

/// [`inline_host_fn_kind`] returning `(fn, bail)` for AOT relocations.
pub(in crate::emu) fn inline_host_fn(t: u64) -> Option<(usize, bool)> {
    inline_host_fn_kind(t).map(|(h, bail, _)| (h, bail))
}

/// Direct `call/jmp` to a mapped DLL's IAT stub (`DF_IAT`, `jmp qword [slot]`): returns
/// `(slot, value, fn, bail, needs_xmm)` if the slot currently holds an inline-callable target.
pub(super) fn iat_slot_plain(t: u64) -> Option<(u64, u64, usize, bool, bool)> {
    if t == 0 || is_chkstk(t) || !plaincall_enabled() {
        return None;
    }
    let (slot, f) = crate::emu::resolve_dispatch_static(t);
    if (f & crate::emu::DF_IAT) == 0 || slot == 0 || (slot & 7) != 0 {
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
pub(super) fn const_slot_plain(op: &FastOp) -> Option<(u64, u64, usize, bool, bool)> {
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
pub(super) fn can_speculate_code(ip: u64) -> bool {
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
