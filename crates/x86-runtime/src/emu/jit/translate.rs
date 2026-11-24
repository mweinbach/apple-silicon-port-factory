//! Block translator core (`Tr`), flag/liveness state, memory access emission, and inline host calls.

use super::aot::AotReloc;
use super::cache::TrScratch;
use super::emit::{enc_logic_imm, Asm};
use super::lazy::{Edges, PF_NONE};
use super::xmm::{xmm_class, ColdNan, XClass};
use super::*;
use crate::emu::fast::{self, FastOp, RNONE};
use crate::emu::{mem, BLK_MAX_INSNS};

pub(in crate::emu) fn supported(op: &FastOp) -> bool {
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
pub(in crate::emu) enum Term {
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

pub(super) fn classify(ops: &[FastOp]) -> Term {
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
pub(super) enum Src {
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
pub(super) struct FlagState {
    pub(super) src: Src,
    /// Flags (subset of FNZCV) currently valid in host NZCV.
    pub(super) host: u8,
    /// Flags currently valid in the `Cpu` flag bytes.
    pub(super) mem: u8,
}

pub(super) enum Cond {
    Host(u32),
    /// Boolean in a register; `inv` = branch/select on zero.
    Reg(u32, bool),
    Const(bool),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Chk {
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
pub(super) fn stack_tso() -> bool {
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

pub(super) struct SideExit {
    pub(super) fix: usize,
    pub(super) k: usize,
    pub(super) fs: FlagState,
    pub(super) lb: [u8; 16],
    /// Dirty cached XMM registers (`AW2_JIT_XMMREG`) to store before leaving.
    pub(super) xd: u16,
    /// Pending parity byte (`Tr::pf`).
    pub(super) pf: u8,
}

pub(super) struct SlowMem {
    pub(super) fix: usize,
    pub(super) back: usize,
    pub(super) ld: bool,
    pub(super) reg: u32,
    pub(super) addr: u32,
    pub(super) bits: u32,
    /// Side exit to take if the range check fails (None: range already known good).
    pub(super) exit: Option<usize>,
    /// Range-check `addr + lo` instead of `addr` (lead of a base-register group, `<= 0`).
    pub(super) lo: i32,
}

pub(super) struct Tr<'a> {
    pub(super) a: Asm,
    pub(super) ops: &'a [FastOp],
    /// `live[k]`: flags live before op `k`; `live[ops.len()]` = live-out.
    pub(super) live: [u8; BLK_MAX_INSNS + 1],
    pub(super) fs: FlagState,
    pub(super) lb: [u8; 16],
    pub(super) side_exits: Vec<SideExit>,
    pub(super) slow: Vec<SlowMem>,
    pub(super) links: Vec<(usize, u64)>,
    /// Hot conditional branches `(cond_site, target)` emitted with a cold `b` trampoline.
    pub(super) cond_links: Vec<(usize, u64)>,
    /// Pairs `(cold_b_site, hot_cond_site)` recorded when materializing `cond_links`.
    pub(super) link_conds: Vec<(usize, usize)>,
    /// Word offset right before the literal pool (`emit_lits`); set at the end of `translate`.
    pub(super) cold_code_end: usize,
    pub(super) exit: usize,
    pub(super) lookup_rip: usize,
    pub(super) lookup_jmp: usize,
    pub(super) lookup_call: usize,
    /// See `JitCache::host_xfer`.
    pub(super) host_xfer: usize,
    /// Shared XMM spill/reload stubs (0 when `PORT_JIT_XMM_PIN=0`) and `clear_ymm_hi` stub.
    pub(super) spill_xmm: usize,
    pub(super) reload_xmm: usize,
    pub(super) clear_ymm_hi: usize,
    pub(super) rdtsc_slow: usize,
    pub(super) rdtscp_slow: usize,
    /// Static terminator target needs host dispatch (or is 0 / `__chkstk`).
    pub(super) target_host: bool,
    /// `JitCache::ras_miss`: non-zero = shadow return-address stack on (`AW2_JIT_RAS`).
    pub(super) ras_miss: usize,
    /// Guest GPRs each op may write (`gpr_writes`; all zero unless `AW2_JIT_MEMCHK` bit 2
    /// or 3 needs them).
    pub(super) wr: [u16; BLK_MAX_INSNS],
    /// Base-register check groups (`AW2_JIT_MEMCHK` bit 2), see [`Tr::plan_groups`].
    pub(super) grp: [Grp; BLK_MAX_INSNS],
    /// Groups whose lead range check has been emitted (bit = group id).
    pub(super) grp_ok: u64,
    /// Op whose first range-checked access already took its group role.
    pub(super) grp_k: usize,
    /// Number of ops before the terminator (all of them for `Term::Fall`).
    pub(super) body: usize,
    /// Following ops already translated together with the current one (push/pop pairs).
    pub(super) skip: usize,
    /// rsp is known 8-aligned here (tested in this block and only moved by multiples of 8).
    pub(super) rsp_al: bool,
    /// `AW2_JIT_XMMREG` (see `xmm.rs`).
    pub(super) xon: bool,
    /// `PORT_JIT_XMM_PIN` (see `xmm.rs`): `xmm0..15` pinned in `v16..v31`.
    pub(super) xpin: bool,
    /// Bitmask of `Cpu::ymm_hi[0..16]` known zero at the current translation point.
    pub(super) ymm_clean: u16,
    /// Deferred cold calls to `clear_ymm_hi`: `(tbnz_site, resume_pos, mask)`.
    pub(super) ymm_colds: Vec<(usize, usize, u16)>,
    /// (branch to fallback, resume after counter sample, ordered read).
    pub(super) tsc_colds: Vec<(usize, usize, bool)>,
    /// Bitmasks of guest `xmm0..15` whose upper bits `[127:32]` / `[127:64]` are known 0.
    pub(super) xmm_hi32_zero: u16,
    pub(super) xmm_hi64_zero: u16,
    /// `xmm_hi32_live[k]` / `xmm_hi64_live[k]`: guest `xmm0..15` whose bits `[127:32]` /
    /// `[127:64]` may be read at/after op `k`.
    pub(super) xmm_hi32_live: [u16; BLK_MAX_INSNS + 1],
    pub(super) xmm_hi64_live: [u16; BLK_MAX_INSNS + 1],
    /// Single-entry address cache for `XA` (`x16`): `Some((base, idx, scale, hi_disp))`.
    pub(super) xa_key: Option<(u8, u8, u8, i64)>,
    /// Set by `try_batch_epilogue` when `T0` (`x0`) and RAS `(x1, x2)` are already preloaded for `Term::Ret`.
    pub(super) ret_preloaded: bool,
    /// True when the block ends in `Term::Ret` with `imm == 0`.
    pub(super) term_is_plain_ret: bool,
    /// Live-out flags for the block's terminator.
    pub(super) term_live_out: u8,
    /// Whether this block decremented `XBUD` at entry (`PORT_JIT_BUDGET_LIGHT`).
    pub(super) bud_dec: bool,
    /// `PORT_JIT_FAST_NAN`: native IEEE-754 ARM64 FP without per-op x86 NaN checks.
    pub(super) fast_nan: bool,
    /// `PORT_JIT_LAST_BITS`: maintain `Cpu::last_bits` for `DF_GAME_REPL` hooks.
    pub(super) lb_on: bool,
    /// Guest XMM registers with a valid copy in their cache register `v(16 + i)`.
    pub(super) xv: u16,
    /// Cached guest XMM registers newer than `Cpu::xmm`.
    pub(super) xd: u16,
    /// Out-of-line NaN fixups.
    pub(super) cold: Vec<ColdNan>,
    /// `AW2_JIT_RETLIT`.
    pub(super) retlit: bool,
    /// `ldr` literal sites and their values (pool emitted after the out-of-line code).
    pub(super) lits: Vec<(usize, u64)>,
    /// 16-byte `ldr qt, <literal>` sites `(site_word, lo64, hi64)` (16B-aligned pool).
    pub(super) lits128: Vec<(usize, u64, u64)>,
    /// Reusable deduplication pools for `emit_lits`.
    pub(super) pool: Vec<(u64, usize)>,
    pub(super) pool128: Vec<(u64, u64, usize)>,
    /// `AW2_JIT_FLAGLAZY` (see `lazy.rs`).
    pub(super) lazy: bool,
    /// Per-edge successor flag needs and `last_bits` kills.
    pub(super) edge: Edges,
    /// Pending parity byte: guest GPR whose low byte is `Cpu::pfv` but not stored yet
    /// (`PF_NONE`: nothing pending).
    pub(super) pf: u8,
    /// AOT emission mode (`new_tr`): host function pointers always come from the literal
    /// pool (one private slot each) and are recorded in `relocs`.
    pub(super) aot: bool,
    /// AOT mode: relocated `ldr` literal sites `(site, value, relocation)` (host function
    /// pointers and import-slot values; `AotReloc::word` is set by `emit_lits`).
    pub(super) rlits: Vec<(usize, u64, AotReloc)>,
    /// AOT mode: relocated pool slots (filled by `emit_lits`).
    pub(super) relocs: Vec<AotReloc>,
}

/// Role of an op's memory operand in a base-register check group: accesses `[b+disp]` off
/// the same base register `b`, not written in between, with all displacements within
/// 4 KiB. The lead's range check covers `b + min(disp)`, so `b + disp` of every member lies
/// in `[0x10000, 2^47 + 0x11000)` — exact at the low end; the high end only adds 4 KiB to
/// the existing check's 64 KiB of never-mapped slack above 2^47.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Grp {
    None,
    /// First access: range-check `a + lo` (`lo` = lowest displacement - own, `<= 0`).
    Lead { lo: i16, id: u8 },
    /// Range already checked by the lead of group `id` (only if that check was emitted).
    Member { id: u8 },
}

/// `(reads, kills)` flag sets of a fast op.
pub(super) fn op_flags(op: &FastOp) -> (u8, u8) {
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

impl Tr<'_> {
    /// Translate the block; returns the number of words on the main path (entry through the
    /// terminator, before the out-of-line code).
    pub(super) fn translate(&mut self, term: Term, live_out: u8) -> usize {
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
        let mut slow = core::mem::take(&mut self.slow);
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
        slow.clear();
        self.slow = slow;
        let mut ymm_colds = core::mem::take(&mut self.ymm_colds);
        for &(fix, back, mask) in &ymm_colds {
            self.a.bind(fix);
            self.a.movz(false, XA, mask as u32, 0);
            self.a.bl_to(self.clear_ymm_hi);
            let here = self.a.pos;
            self.a.b();
            self.a.patch(here, back);
        }
        ymm_colds.clear();
        self.ymm_colds = ymm_colds;
        self.emit_timestamp_fallbacks();
        self.emit_cold_nan();
        let mut ses = core::mem::take(&mut self.side_exits);
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
        ses.clear();
        self.side_exits = ses;
        let mut cond_links = core::mem::take(&mut self.cond_links);
        for &(cond_site, target) in &cond_links {
            self.a.bind(cond_site);
            let site = self.a.b();
            self.links.push((site, target));
            self.link_conds.push((site, cond_site));
        }
        cond_links.clear();
        self.cond_links = cond_links;
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

    /// Return all reusable scratch `Vec` buffers back to `sc` (cleared, preserving heap capacity).
    pub(super) fn return_scratch(mut self, sc: &mut TrScratch) {
        self.side_exits.clear();
        self.slow.clear();
        self.links.clear();
        self.cond_links.clear();
        self.link_conds.clear();
        self.ymm_colds.clear();
        self.tsc_colds.clear();
        self.cold.clear();
        self.lits.clear();
        self.lits128.clear();
        self.pool.clear();
        self.pool128.clear();
        self.rlits.clear();
        self.relocs.clear();
        sc.side_exits = self.side_exits;
        sc.slow = self.slow;
        sc.links = self.links;
        sc.cond_links = self.cond_links;
        sc.link_conds = self.link_conds;
        sc.ymm_colds = self.ymm_colds;
        sc.tsc_colds = self.tsc_colds;
        sc.cold = self.cold;
        sc.lits = self.lits;
        sc.lits128 = self.lits128;
        sc.pool = self.pool;
        sc.pool128 = self.pool128;
        sc.rlits = self.rlits;
        sc.relocs = self.relocs;
    }

    /// Store `rip` and exit with `EXIT_SIDE` (interpreter runs that instruction itself).
    pub(super) fn exit_side(&mut self, rip: u64) {
        self.a.mov_imm(true, T0, rip);
        self.a.str(64, T0, XCPU, OFF_RIP);
        self.a.movz(false, T0, EXIT_SIDE as u32, 0);
        self.a.b_to(self.exit);
    }

    /// Exit with `code`, `cpu.rip = rip`, `jx.target = target`.
    pub(super) fn exit_code(&mut self, code: u64, rip: u64, target: u64) {
        self.a.mov_imm(true, T0, rip);
        self.a.str(64, T0, XCPU, OFF_RIP);
        self.a.mov_imm(true, T0, target);
        self.a.str(64, T0, XCPU, OFF_TARGET);
        self.a.movz(false, T0, code as u32, 0);
        self.a.b_to(self.exit);
    }

    /// Direct transfer to guest `target` (linked later).
    pub(super) fn exit_to(&mut self, target: u64) {
        let site = self.a.b();
        self.links.push((site, target));
    }

    /// Dynamic guest call (x0 = target) through `lookup_call`, return address `fall`. With
    /// the RAS, enter it with `bl`: x30 = the landing pad right after it (a transfer to
    /// `fall`), which `lookup_call` pushes on the RAS with `fall`.
    pub(super) fn call_lookup(&mut self, fall: u64) {
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
    pub(super) fn abi_lb(&self) -> [u8; 4] {
        [self.lb[1], self.lb[2], self.lb[8], self.lb[9]]
    }

    /// Host call (`kind` 0) / tail jump (`kind` 1) to `target` through the `host_xfer` stub.
    /// Flags and last_bits must already be flushed.
    pub(super) fn xfer_to(&mut self, target: u64, fall: u64, kind: u32) {
        self.xfer_to_src(target, fall, kind, 0);
    }

    /// [`Self::xfer_to`]; `src` != 0: `target` is the value read from guest import slot `src`
    /// (AOT: relocated with the slot, see [`Self::mov_slot_val`]).
    pub(super) fn xfer_to_src(&mut self, target: u64, fall: u64, kind: u32, src: u64) {
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

    /// Bitmask of guest XMM registers whose upper 128-bit YMM half is cleared by a VEX-128
    /// op in `ops[..body]`, or `0` if any op in the block can read `Cpu::ymm_hi` (so a
    /// block-entry bulk clear can never clobber a live-in 256-bit YMM register before it is
    /// read, nor before a software side-exit when `!hw_range`).
    pub(super) fn block_v128_mask(&self, body: usize) -> u16 {
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
    pub(super) fn compute_xmm_hi_live(&mut self, body: usize) {
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
    pub(super) fn emit_ymm_clear_check(&mut self, mask: u16) {
        let fix = self.a.tbnz(XBUD, 0);
        self.ymm_colds.push((fix, self.a.pos, mask));
    }

    /// Store 128-bit vector `v_src` into `Cpu::ymm_hi[reg]` and mark `reg` dirty in
    /// `Cpu::jx.ymm_dirty` and bit 0 of `XBUD`.
    pub(super) fn write_ymm_hi(&mut self, reg: u8, v_src: u32) {
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
    pub(super) fn clear_ymm_hi(&mut self, reg: u8) {
        let r = reg & 15;
        let bit = 1u16 << r;
        if self.ymm_clean & bit != 0 {
            return;
        }
        self.a.zero128(XCPU, OFF_YMM_HI + (r as u32) * 16);
        self.ymm_clean |= bit;
    }

    /// Inline call of the host function `h` (`DF_PLAIN` builtin or `inline_variant`: at most
    /// 8 integer args / 4 XMM args, never re-enters guest code) with the register effects of
    /// the interpreter's `host_call` / `host_tail_call`.
    pub(super) fn plain_call(
        &mut self,
        h: usize,
        hva: u64,
        _lbk: [u8; 4],
        tail: bool,
        bail: Option<(u64, u64)>,
        src: u64,
        needs_xmm: bool,
    ) {
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
        // AAPCS64 designates `v16..v31` as caller-saved; even integer-only `DF_PLAIN` builtins
        // (e.g. NEON `memcpy`/`memset` or LLVM auto-vectorized Rust) can clobber `v16..v31`,
        // so pinned XMM registers must always be spilled before `blr` and reloaded afterward.
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
        // Fast-path FPCR: skip `mrs fpcr` when guest `MXCSR == 0x1F80` (default IEEE-754 state).
        self.a.ldr(32, XS, XCPU, OFF_MXCSR);
        self.a.addsub_ri(false, true, false, XS, XS, 1, true); // sub w17, w17, #0x1000
        self.a.addsub_ri(false, true, false, XS, XS, 0xF80, false); // sub w17, w17, #0xF80
        self.a.str(64, ZR, 31, 64); // saved_fpcr = 0
        let fpcr_def = self.a.cbz(false, XS);
        self.a.mrs_fpcr(XS);
        self.a.str(64, XS, 31, 64);
        let fpcr_zero = self.a.cbz(true, XS);
        self.a.msr_fpcr(ZR);
        self.a.bind(fpcr_zero);
        self.a.bind(fpcr_def);
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
        // Only write `q0` back to guest `xmm0` for float-ABI (`needs_xmm`) callees; integer-only
        // `DF_PLAIN` builtins preserve guest `xmm0` instead of clobbering it with leftover `q0`.
        if needs_xmm {
            if self.xpin {
                self.a.v_mov(16, 0);
            } else {
                self.a.str(128, 0, XCPU, OFF_XMM);
            }
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

    pub(super) fn side_exit_here(&mut self, k: usize) -> usize {
        self.side_exits.push(SideExit { fix: usize::MAX, k, fs: self.fs, lb: self.lb, xd: self.xd, pf: self.pf });
        self.side_exits.len() - 1
    }

    /// `if !(0x1_0000 <= a < 2^47) goto side_exit[se]`.
    pub(super) fn range_check_to(&mut self, a: u32, se: usize) {
        if hw_range_enabled() {
            return;
        }
        self.a.addsub_ri(true, true, false, XS, a, 0x10, true);
        self.a.lsr_i(true, XS, XS, 47);
        let f = self.a.cbnz(true, XS);
        self.bind_side_exit(se, f);
    }

    /// Make branch `f` (to be bound out of line) take side exit `se`.
    pub(super) fn bind_side_exit(&mut self, se: usize, f: usize) {
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
    pub(super) fn range_check_op(&mut self, k: usize, a: u32, se: usize) {
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
    pub(super) fn plan_groups(&mut self) {
        use fast::*;
        if memchk_mask() & 4 == 0 {
            return;
        }
        // (base, base version, op, disp) in program order (fixed stack arrays, zero heap allocs).
        let mut acc = [(0u8, 0u16, 0usize, 0i64); BLK_MAX_INSNS];
        let mut acc_n = 0usize;
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
                acc[acc_n] = (b, ver[b as usize], k, op.disp);
                acc_n += 1;
            }
            let w = self.wr[k];
            for (r, v) in ver.iter_mut().enumerate() {
                if w & (1 << r) != 0 {
                    *v = v.wrapping_add(1);
                }
            }
        }
        let mut id = 0u8;
        let mut done = [false; BLK_MAX_INSNS];
        let mut mem = [0usize; BLK_MAX_INSNS];
        for i in 0..acc_n {
            if done[i] || id >= 64 {
                continue;
            }
            let (b, v, _, _) = acc[i];
            // Members: same base value, greedily while the span stays within 4 KiB (with
            // 32 bytes for the widest 256-bit vector access).
            mem[0] = i;
            let mut mem_n = 1usize;
            let (mut lo, mut hi) = (acc[i].3, acc[i].3);
            for j in i + 1..acc_n {
                if done[j] || acc[j].0 != b || acc[j].1 != v {
                    continue;
                }
                let (nl, nh) = (lo.min(acc[j].3), hi.max(acc[j].3));
                if nh + 32 - nl > 0x1000 {
                    continue;
                }
                (lo, hi) = (nl, nh);
                mem[mem_n] = j;
                mem_n += 1;
            }
            if mem_n < 2 {
                continue;
            }
            for (n, &j) in mem[..mem_n].iter().enumerate() {
                done[j] = true;
                let k = acc[j].2;
                self.grp[k] = if n == 0 { Grp::Lead { lo: (lo - acc[j].3) as i16, id } } else { Grp::Member { id } };
            }
            id += 1;
        }
    }

    /// After op `k`: drop the "rsp 8-aligned" fact unless rsp was left alone or moved by a
    /// multiple of 8 (`and rsp, -8k` establishes it).
    pub(super) fn track_rsp(&mut self, k: usize) {
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
    pub(super) fn rsp_slot(&mut self, k: usize, op: &FastOp, bits: u32) -> Option<i32> {
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
    pub(super) fn term_tests_budget(&self, term: &Term) -> bool {
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
    pub(super) fn addr_into(&mut self, op: &FastOp, dst: u32) -> u32 {
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

    pub(super) fn chk_of(op: &FastOp) -> Chk {
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
    pub(super) fn load(&mut self, k: usize, dst: u32, a: u32, bits: u32, chk: Chk) {
        self.mem_access(k, true, dst, a, bits, chk);
    }

    pub(super) fn store(&mut self, k: usize, src: u32, a: u32, bits: u32, chk: Chk) {
        self.mem_access(k, false, src, a, bits, chk);
    }

    /// Guest load of `bits` from `op`'s memory operand into `dst`, using direct `[base, #disp]`
    /// addressing (`ldapur`/`ldr`/`ldur`) when possible instead of `add x16, base, #disp`.
    pub(super) fn load_op(&mut self, k: usize, dst: u32, op: &FastOp, bits: u32) {
        self.mem_access_op(k, true, dst, op, bits);
    }

    /// Guest store of `bits` from `src` to `op`'s memory operand, using direct `[base, #disp]`
    /// addressing (`stlur`/`str`/`stur`) when possible.
    pub(super) fn store_op(&mut self, k: usize, src: u32, op: &FastOp, bits: u32) {
        self.mem_access_op(k, false, src, op, bits);
    }

    pub(super) fn mem_access_op(&mut self, k: usize, ld: bool, reg: u32, op: &FastOp, bits: u32) {
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

    pub(super) fn mem_access(&mut self, k: usize, ld: bool, reg: u32, a: u32, bits: u32, chk: Chk) {
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
    pub(super) fn store_checked_addr(&mut self, src: u32, a: u32, bits: u32) {
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

    pub(super) fn set_lb(&mut self, i: u8, w: u8) {
        if self.lb_on && tracked(i) {
            self.lb[i as usize] = match w {
                8 => 8,
                16 => 16,
                _ => 0,
            };
        }
    }

    pub(super) fn flush_lb_reg(&mut self, i: usize) {
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

    pub(super) fn flush_lb(&mut self) {
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
    pub(super) fn lb_store_now(&mut self, i: u8, w: u8) {
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
    pub(super) fn host_flag(&mut self, f: u8, r: u32) {
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

    pub(super) fn flag_off(f: u8) -> u32 {
        match f {
            FC => OFF_CF,
            FZ => OFF_ZF,
            FS => OFF_SF,
            _ => OFF_OF,
        }
    }

    /// Write flags in `need` that are only in NZCV to memory.
    pub(super) fn flush_flags(&mut self, need: u8) {
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
    pub(super) fn flag_reg(&mut self, f: u8, r: u32) {
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

    pub(super) fn cond(&mut self, cc: u8) -> Cond {
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
    pub(super) fn branch_if_false(&mut self, c: &Cond) -> Option<usize> {
        match *c {
            Cond::Host(cnd) => Some(self.a.bcond(cnd ^ 1)),
            Cond::Reg(r, inv) => Some(if inv { self.a.cbnz(false, r) } else { self.a.cbz(false, r) }),
            Cond::Const(true) => None,
            Cond::Const(false) => Some(self.a.b()),
        }
    }

    // --- register helpers ---

    /// Write `src` (value in its low `w` bits) to guest register `i` with x86 width semantics.
    pub(super) fn wr_gpr(&mut self, i: u8, w: u8, src: u32) {
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
}
