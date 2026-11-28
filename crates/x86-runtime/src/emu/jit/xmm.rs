//! Guest XMM register cache (`AW2_JIT_XMMREG` / `PORT_JIT_XMM_PIN`) and main-path
//! 64-bit / 128-bit constant literal pools (`AW2_JIT_RETLIT`).

use super::aot::{AotReloc, RELOC_HOST_FN, RELOC_QPC_STATE, RELOC_SLOT_VAL, RELOC_XINPUT_STATE};
use super::emit::Asm;
use super::lazy::PF_NONE;
use super::translate::{Chk, FlagState, Grp, Src, Tr};
use super::*;
use crate::emu::fast::{self, FastOp, RNONE};

/// `AW2_JIT_XMMREG` (default 1): keep guest XMM registers in host `v16..v31` within a
/// block, NaN fixups out of line. `0`: jit5 code.
pub(in crate::emu) fn xmmreg_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().xmmreg) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_XMMREG", 1) != 0)
}

/// `PORT_JIT_XMM_PIN` (default 1 when `AW2_JIT_XMMREG=1`): keep guest `xmm0..15` permanently
/// pinned in host `v16..v31` across blocks (loaded in `enter`, stored in `exit`/`host_xfer`,
/// saved/restored across `plain_call` / `lookup_call` host calls). Eliminates all per-block
/// `Cpu::xmm` loads/stores and side-exit XMM flushes.
pub(in crate::emu) fn xmm_pin_enabled() -> bool {
    if !xmmreg_enabled() {
        return false;
    }
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().xmm_pin) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("PORT_JIT_XMM_PIN", 1) != 0)
}

/// `PORT_JIT_FAST_NAN` (default 1 in production, 0 in unit tests unless overridden): use
/// native IEEE-754 ARM64 scalar/packed FP instructions directly without per-op x86 QNaN
/// sign-bit / payload-tiebreak checks (`fcmeq+uminv+fmov+cbz` / `fcmp+b.vs`). Matches
/// Rosetta 2 / FEX / Box64 FP semantics; `0` preserves bit-exact x86 NaN payloads for
/// differential testing.
pub(in crate::emu) fn fast_nan_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().fast_nan) {
        return v;
    }
    #[cfg(test)]
    {
        if let Ok(s) = std::env::var("PORT_JIT_FAST_NAN") {
            if let Ok(v) = s.trim().parse::<u32>() {
                return v != 0;
            }
        }
        return false;
    }
    #[cfg(not(test))]
    {
        static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *V.get_or_init(|| {
            if crate::emu::verify_enabled() || std::env::args().any(|a| a == "--diff-test") {
                return false;
            }
            env_knob("PORT_JIT_FAST_NAN", 1) != 0
        })
    }
}

/// `AW2_JIT_RETLIT` (default 1): 64-bit code-address constants on the main path (guest
/// return addresses pushed by calls / passed to `lookup_call` / `host_xfer`, host call
/// targets, import-slot addresses and values) that `mov_imm` would build with three or more
/// `movz`/`movk` words are one `ldr xN, =literal` from a pool after the block's
/// out-of-line code. `0`: `movz`/`movk` (jit5 code).
pub(in crate::emu) fn retlit_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().retlit) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_RETLIT", 1) != 0)
}

/// Host vector register caching guest `xmm0` (guest `xmm i` lives in `v(XV0 + i)`).
const XV0: u32 = 16;
/// Host vector register pinning guest `ymm_hi[0..8]` when `xpin` is active (`v8..v15`).
const YV0: u32 = 8;

#[inline(always)]
pub(super) fn xh(i: u8) -> u32 {
    XV0 + (i & 15) as u32
}

#[inline(always)]
pub(super) fn yh(i: u8) -> Option<u32> {
    let r = i & 15;
    (r < 8).then_some(YV0 + r as u32)
}

#[inline(always)]
pub(super) fn xoff(i: u8) -> u32 {
    OFF_XMM + 16 * (i & 15) as u32
}

/// How an op relates to the XMM cache.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum XClass {
    /// Never touches `Cpu::xmm` (integer / flag / stack / string ops; `vzeroupper` only
    /// writes `Cpu::ymm_hi`, which is never cached).
    Free,
    /// Translated cache-aware by [`Tr::xop`].
    Aware,
    /// May read or write `Cpu::xmm` directly: dirty registers are stored and the cache is
    /// dropped before it.
    Barrier,
}

pub(super) fn xmm_class(op: &FastOp) -> XClass {
    use fast::*;
    match op.kind {
        K_SYS_MISC if matches!(op.x & 0x3F, 9 | 13 | 16) => XClass::Barrier,
        K_NOP..=K_CMPJ_RM
        | K_IMUL_RR..=K_MOVBE_MR
        | K_BMI_RR
        | K_BMI_RM
        | K_LOOP
        | K_SYS_MISC..=K_ATOMIC
        | K_MMX_VSHUF_RR..=K_MMX_VINT_RM => XClass::Free,
        K_VSHUF_RR..=K_CVTF2I_M | K_VFMA_RR | K_VFMA_RM => XClass::Aware,
        _ => XClass::Barrier,
    }
}

/// Out-of-line x86 NaN fixup of a scalar / packed arithmetic op: `v0 = a` (from `va`),
/// `v1 = b` (from `vb`), `v2 = arm64 result` -> fixed `v2`, then back to the main path.
pub(super) struct ColdNan {
    pub(super) fix: usize,
    pub(super) back: usize,
    pub(super) is_d: bool,
    pub(super) unary: bool,
    pub(super) va: u32,
    pub(super) vb: u32,
}

impl Asm {
    /// `ldr s/d/q t, [n, #off]` (vector, unsigned offset; `bits` 32/64/128).
    pub(super) fn ldr_v(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        match bits {
            32 => {
                assert!(off % 4 == 0 && off / 4 < 4096, "ldr s offset {off}");
                self.emit(0xBD40_0000 | (off / 4) << 10 | n << 5 | t);
            }
            64 => {
                assert!(off % 8 == 0 && off / 8 < 4096, "ldr d offset {off}");
                self.emit(0xFD40_0000 | (off / 8) << 10 | n << 5 | t);
            }
            _ => self.ldr(128, t, n, off),
        }
    }
    /// `str s/d/q t, [n, #off]` (vector, unsigned offset; `bits` 32/64/128).
    pub(super) fn str_v(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        match bits {
            32 => {
                assert!(off % 4 == 0 && off / 4 < 4096, "str s offset {off}");
                self.emit(0xBD00_0000 | (off / 4) << 10 | n << 5 | t);
            }
            64 => {
                assert!(off % 8 == 0 && off / 8 < 4096, "str d offset {off}");
                self.emit(0xFD00_0000 | (off / 8) << 10 | n << 5 | t);
            }
            _ => self.str(128, t, n, off),
        }
    }
    /// `ldur/stur s/d t, [n, #off]` (unscaled signed 9-bit offset, 32/64-bit scalar FP).
    pub(super) fn ldur_stur_v(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldur/stur v offset {off}");
        let base = match (ld, bits) {
            (true, 32) => 0xBC40_0000u32,
            (false, 32) => 0xBC00_0000,
            (true, _) => 0xFC40_0000,
            (false, _) => 0xFC00_0000,
        };
        self.emit(base | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `ldr qt, <literal>` placeholder (128-bit PC-relative literal load).
    pub(super) fn ldr_q_lit(&mut self, t: u32) -> usize {
        self.emit(0x9C00_0000 | t);
        self.pos - 1
    }
    /// `fmov sd, sn` / `fmov dd, dn` (scalar register move: upper bits of `vd` cleared).
    pub(super) fn fmov_s(&mut self, is_f64: bool, vd: u32, vn: u32) {
        self.emit(0x1E20_4000 | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    /// `ins vd.<T>[di], vn.<T>[si]` (`lg` = log2 of the element bytes).
    pub(super) fn v_ins_lane(&mut self, lg: u32, vd: u32, di: u32, vn: u32, si: u32) {
        let imm5 = (di << (lg + 1)) | (1u32 << lg);
        let imm4 = si << lg;
        self.emit(0x6E00_0400 | imm5 << 16 | imm4 << 11 | vn << 5 | vd);
    }
    /// `ldr xt, <literal>` placeholder; set the offset with [`Self::patch_ldr_lit`].
    pub(super) fn ldr_lit(&mut self, t: u32) -> usize {
        self.emit(0x5800_0000 | t);
        self.pos - 1
    }
    /// Point the `ldr` literal at word `at` to word `target` of this buffer.
    pub(super) fn patch_ldr_lit(&mut self, at: usize, target: usize) {
        if at >= self.cap || self.overflow {
            return;
        }
        let off = (target as i64 - at as i64) as u32;
        let ins = self.read(at) & 0xFF00_001F;
        unsafe { *self.base.add(at) = ins | (off & 0x7_FFFF) << 5 };
    }
}

impl Tr<'_> {
    // --- cache state ---

    /// Host register holding guest `xmm i`, loading it from `Cpu::xmm` on first use.
    pub(super) fn xget(&mut self, i: u8) -> u32 {
        if self.xpin {
            return xh(i);
        }
        if !self.xon {
            self.a.ldr(128, xh(i), XCPU, xoff(i));
            return xh(i);
        }
        let b = 1u16 << (i & 15);
        if self.xv & b == 0 {
            self.a.ldr(128, xh(i), XCPU, xoff(i));
            self.xv |= b;
        }
        xh(i)
    }

    /// Record upper-lane-zero knowledge for guest `xmm d` (`hi32`: bits `[127:32]` known 0;
    /// `hi64`: bits `[127:64]` known 0).
    #[inline(always)]
    pub(super) fn x_mark_hi(&mut self, d: u8, hi32: bool, hi64: bool) {
        let b = 1u16 << (d & 15);
        if hi32 {
            self.xmm_hi32_zero |= b;
        } else {
            self.xmm_hi32_zero &= !b;
        }
        if hi64 {
            self.xmm_hi64_zero |= b;
        } else {
            self.xmm_hi64_zero &= !b;
        }
    }

    #[inline(always)]
    pub(super) fn x_clobber_hi(&mut self, d: u8) {
        let m = !(1u16 << (d & 15));
        self.xmm_hi32_zero &= m;
        self.xmm_hi64_zero &= m;
    }

    /// Guest `xmm i` was just written into its cache register (call only after the op's
    /// last possible side exit).
    pub(super) fn xdef(&mut self, i: u8) {
        self.x_clobber_hi(i);
        if self.xpin {
            return;
        }
        if !self.xon {
            self.a.str(128, xh(i), XCPU, xoff(i));
            return;
        }
        let b = 1u16 << (i & 15);
        self.xv |= b;
        self.xd |= b;
    }

    /// Guest `xmm i` was written to `Cpu::xmm` directly: forget the cached copy.
    pub(super) fn xdrop(&mut self, i: u8) {
        self.x_clobber_hi(i);
        if self.xpin {
            return;
        }
        let b = !(1u16 << (i & 15));
        self.xv &= b;
        self.xd &= b;
    }

    /// Write 128-bit vector temp `v_src` into guest `xmm i` (into pinned `v(16+i)` when
    /// `xpin`, else through to `Cpu::xmm[i]`).
    pub(super) fn xwrite(&mut self, i: u8, v_src: u32) {
        self.x_clobber_hi(i);
        if self.xpin {
            self.a.v_mov(xh(i), v_src);
        } else {
            self.a.str(128, v_src, XCPU, xoff(i));
            self.xdrop(i);
        }
    }

    /// Copy of guest `xmm i` into vector temp `t` (cache register or `Cpu::xmm`).
    pub(super) fn xsrc(&mut self, t: u32, i: u8) {
        if self.xpin || self.xv & (1u16 << (i & 15)) != 0 {
            self.a.v_mov(t, xh(i));
        } else {
            self.a.ldr(128, t, XCPU, xoff(i));
        }
    }

    /// Host register pinning guest `ymm_hi[i]` (`v8..v15` for `i < 8` when `xpin`), else `None`.
    #[inline(always)]
    pub(super) fn yh_reg(&self, i: u8) -> Option<u32> {
        if self.xpin { yh(i) } else { None }
    }

    /// Copy of guest `ymm_hi[i]` into vector temp `t` (pinned `v8..v15` or `Cpu::ymm_hi`).
    pub(super) fn ysrc(&mut self, t: u32, i: u8) {
        if let Some(vy) = self.yh_reg(i) {
            self.a.v_mov(t, vy);
        } else {
            self.a.ldr(128, t, XCPU, OFF_YMM_HI + 16 * (i & 15) as u32);
        }
    }

    /// Host vector register holding guest `ymm_hi[i]`: pinned `v8..v15` with zero copies,
    /// otherwise loaded into `fallback_tmp`.
    pub(super) fn yget(&mut self, i: u8, fallback_tmp: u32) -> u32 {
        if let Some(vy) = self.yh_reg(i) {
            vy
        } else {
            self.a.ldr(128, fallback_tmp, XCPU, OFF_YMM_HI + 16 * (i & 15) as u32);
            fallback_tmp
        }
    }

    /// Low `bits` (32/64) of guest `xmm i` into GPR `t`.
    pub(super) fn xgpr(&mut self, bits: u32, t: u32, i: u8) {
        if self.xpin || self.xv & (1u16 << (i & 15)) != 0 {
            self.a.fmov_fg(bits == 64, t, xh(i));
        } else {
            self.a.ldr(bits, t, XCPU, xoff(i));
        }
    }

    /// Store the registers in `xd` to `Cpu::xmm`.
    pub(super) fn xstore(&mut self, xd: u16) {
        if self.xpin {
            return;
        }
        for i in 0..16u8 {
            if xd & (1 << i) != 0 {
                self.a.str(128, xh(i), XCPU, xoff(i));
            }
        }
    }

    /// Store every dirty register (they stay cached, now clean).
    pub(super) fn xflush(&mut self) {
        if self.xpin {
            return;
        }
        let xd = self.xd;
        self.xstore(xd);
        self.xd = 0;
    }

    /// Store every dirty register and forget the cache.
    pub(super) fn xbarrier(&mut self) {
        self.xmm_hi32_zero = 0;
        self.xmm_hi64_zero = 0;
        if self.xpin {
            return;
        }
        self.xflush();
        self.xv = 0;
    }

    /// `AW2_JIT_XMMREG` dispatch at the start of op `k`: `true` if translated here.
    pub(super) fn xmm_op(&mut self, k: usize, op: &FastOp) -> bool {
        match xmm_class(op) {
            XClass::Free => false,
            XClass::Aware => {
                if self.xop(k, op) {
                    return true;
                }
                self.xbarrier();
                false
            }
            XClass::Barrier => {
                self.xbarrier();
                false
            }
        }
    }

    /// Cache-aware translation of the `XClass::Aware` kinds (`false`: not handled).
    pub(super) fn xop(&mut self, k: usize, op: &FastOp) -> bool {
        use fast::*;
        let v256 = op.w & 3 == VM_V256;
        let sub = op.x & 0x3F;
        match op.kind {
            K_VFP_RR | K_VFP_RM
                if !v256 && matches!(sub, 0..=11 | VF_SQRTSS | VF_SQRTSD | VF_CVTSS2SD | VF_CVTSD2SS) =>
            {
                self.x_fp_scalar(k, op, sub)
            }
            K_VFP_RR | K_VFP_RM if !v256 && matches!(sub, 12..=23) => self.x_fp_packed(k, op, sub),
            K_VFP_RR | K_VFP_RM
                if v256 && matches!(sub, 12..=23) && (self.fast_nan || matches!(sub, 16 | 17 | 22 | 23)) =>
            {
                self.x_fp_packed_256(k, op, sub)
            }
            K_VSHUF_RM if !v256 && matches!((sub, op.w >> 4), (VS_ZX32, 2) | (VS_ZX64, 3) | (VS_MOV, 4)) => {
                self.x_load(k, op, sub)
            }
            K_VSHUF_RM if v256 && sub == VS_MOV && (op.w >> 4) == 5 => self.x_load_256(k, op),
            K_VSHUF_RM if !v256 && matches!(sub, VS_XOR | VS_OR | VS_AND | VS_ANDN) && (op.w >> 4) == 4 => {
                self.x_logic_rm(k, op, sub)
            }
            K_VSHUF_RM if v256 && matches!(sub, VS_XOR | VS_OR | VS_AND | VS_ANDN) && (op.w >> 4) == 5 => {
                self.x_logic_rm_256(k, op, sub)
            }
            K_VSHUF_RR if !v256 && sub <= VS_ANDN => self.x_move(k, op, sub),
            K_VSHUF_RR if v256 && matches!(sub, VS_MOV | VS_XOR | VS_OR | VS_AND | VS_ANDN) => {
                self.x_move_256(op, sub)
            }
            K_VSHUF_RR if !v256 && self.x_shuf_rr(op, sub) => {}
            K_VFMA_RR | K_VFMA_RM => self.emit_vfma_op(k, op),
            // Everything else: the generic NEON path (operands via `xsrc`, result written
            // through to `Cpu::xmm` by `vstore_dst`, which drops the cached copy).
            K_VSHUF_RR | K_VSHUF_RM | K_VINT_RR | K_VINT_RM | K_VFP_RR | K_VFP_RM => self.vop(k, op),
            K_VST => self.x_vst(k, op),
            K_MOVD_GX => {
                let w = op.w;
                self.xgpr(w as u32, g(op.r0), op.r1);
                self.set_lb(op.r0, w);
            }
            K_MOVD_XG => {
                let d = op.r0 & 15;
                // fmov s/d writes the low 32/64 bits and clears the rest of the register.
                self.a.fmov_gf(op.imm == 64, xh(d), g(op.r1));
                self.xdef(d);
                self.x_mark_hi(d, op.imm != 64, true);
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(d);
                }
            }
            K_CVTSI_R | K_CVTSI_M => {
                let width = (op.imm & 0xFF) as u32;
                let is_sd = (op.x & 0x3F) != 0;
                let src = if op.kind == K_CVTSI_R {
                    g(op.r1)
                } else {
                    self.load_op(k, T0, op, width);
                    T0
                };
                let d = op.r0 & 15;
                let s1 = (op.imm >> 8) as u8 & 15;
                let hi_zero = if is_sd {
                    self.xmm_hi64_zero & (1u16 << s1) != 0
                } else {
                    self.xmm_hi32_zero & (1u16 << s1) != 0
                };
                let hi_dead = if is_sd {
                    (self.xmm_hi64_live[k + 1] & (1u16 << d)) == 0
                } else {
                    (self.xmm_hi32_live[k + 1] & (1u16 << d)) == 0
                };
                if hi_zero || hi_dead {
                    // Upper bits of merge source s1 are known 0 (or dead after op k): ARM64
                    // scalar scvtf directly into sD/dD clears upper bits in 1 instruction.
                    self.a.scvtf_gp(width == 64, is_sd, xh(d), src);
                    self.xdef(d);
                    self.x_mark_hi(d, !is_sd, true);
                } else {
                    self.a.scvtf_gp(width == 64, is_sd, 2, src);
                    let va = self.xget(s1);
                    self.x_merge_lane(d, s1, va, if is_sd { 3 } else { 2 });
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(d);
                }
            }
            K_COMI_RR | K_COMI_RM => self.x_comi(k, op),
            K_CVTF2I_R | K_CVTF2I_M => self.x_cvtf2i(k, op),
            _ => return false,
        }
        true
    }

    /// `vd = va` with element 0 (`lg`: 2 = 32-bit, 3 = 64-bit) replaced by `v2`'s; guest
    /// `xmm d` = that (dirty). `va` holds guest `xmm s1` (the merge source).
    pub(super) fn x_merge_lane(&mut self, d: u8, s1: u8, va: u32, lg: u32) {
        let vd = xh(d);
        if d != s1 {
            self.a.v_mov(vd, va);
        }
        self.a.v_ins_lane(lg, vd, 0, 2, 0);
        self.xdef(d);
    }

    /// `addss`..`maxsd`, `sqrtss/sd`, `cvtss2sd`, `cvtsd2ss` (legacy and VEX 128).
    pub(super) fn x_fp_scalar(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let is_minmax = matches!(sub, 4 | 5 | 10 | 11);
        let cvt = matches!(sub, VF_CVTSS2SD | VF_CVTSD2SS);
        if is_minmax || (!cvt && !self.fast_nan) {
            // fcmp clobbers NZCV.
            self.flush_flags(self.live[k]);
            self.fs.host = 0;
        }
        let vb = if op.kind == K_VFP_RM {
            self.vload_b(k, op);
            1
        } else {
            self.xget(op.r1)
        };
        let s1 = (op.imm >> 8) as u8 & 15;
        let d = op.r0 & 15;
        let va = self.xget(s1);
        let is_sd_op = matches!(sub, 6..=11 | VF_SQRTSD | VF_CVTSS2SD);
        let hi_zero = if is_sd_op {
            self.xmm_hi64_zero & (1u16 << s1) != 0
        } else {
            self.xmm_hi32_zero & (1u16 << s1) != 0
        };
        let hi_dead = if is_sd_op {
            (self.xmm_hi64_live[k + 1] & (1u16 << d)) == 0
        } else {
            (self.xmm_hi32_live[k + 1] & (1u16 << d)) == 0
        };
        let direct = (hi_zero || hi_dead) && (self.fast_nan || is_minmax || cvt);
        let dst_v = if direct { xh(d) } else { 2 };
        let lg = match sub {
            0..=3 | 6..=9 => {
                let is_sd = sub >= 6;
                let aop = if is_sd { sub - 6 } else { sub };
                self.a.s_farith(is_sd, aop, dst_v, va, vb);
                if !self.fast_nan {
                    self.a.fcmp(is_sd, 2, 2);
                    let fix = self.a.bcond(VS);
                    self.cold.push(ColdNan { fix, back: self.a.pos, is_d: is_sd, unary: false, va, vb });
                }
                if is_sd { 3 } else { 2 }
            }
            4 | 5 | 10 | 11 => {
                // x86 min/max: the second operand unless a < b (a > b), so NaNs and
                // equal zeros give b.
                let is_sd = sub >= 6;
                let is_max = sub == 5 || sub == 11;
                self.a.fcmp(is_sd, va, vb);
                self.a.s_fcsel(is_sd, dst_v, va, vb, if is_max { GT } else { MI });
                self.flush_minmax_result(dst_v, is_sd);
                if is_sd { 3 } else { 2 }
            }
            VF_SQRTSS | VF_SQRTSD => {
                let is_sd = sub == VF_SQRTSD;
                self.a.s_fsqrt(is_sd, dst_v, vb);
                if !self.fast_nan {
                    self.a.fcmp(is_sd, 2, 2);
                    let fix = self.a.bcond(VS);
                    self.cold.push(ColdNan { fix, back: self.a.pos, is_d: is_sd, unary: true, va, vb });
                }
                if is_sd { 3 } else { 2 }
            }
            VF_CVTSS2SD => {
                self.a.fcvt_d_s(dst_v, vb);
                3
            }
            _ => {
                self.a.fcvt_s_d(dst_v, vb);
                2
            }
        };
        if direct {
            self.xdef(d);
            self.x_mark_hi(d, lg == 2, true);
        } else {
            self.x_merge_lane(d, s1, va, lg);
        }
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
    }

    /// `addps`..`maxpd` (legacy and VEX 128).
    pub(super) fn x_fp_packed(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let vb = if op.kind == K_VFP_RM {
            self.vload_b(k, op);
            1
        } else {
            self.xget(op.r1)
        };
        let s1 = (op.imm >> 8) as u8 & 15;
        let d = op.r0 & 15;
        let va = self.xget(s1);
        let vd = xh(d);
        let is_pd = sub >= 18;
        match sub {
            16 | 17 | 22 | 23 => {
                // fcmgt + bsl (NZCV untouched): a where a < b (a > b), else b.
                let dst = if vd != va && vd != vb { vd } else { 2 };
                if sub == 17 || sub == 23 {
                    self.a.v_fcmgt(is_pd, dst, va, vb);
                } else {
                    self.a.v_fcmgt(is_pd, dst, vb, va);
                }
                self.a.v_bsl(dst, va, vb);
                self.flush_minmax_result(dst, is_pd);
                if dst != vd {
                    self.a.v_mov(vd, dst);
                }
            }
            _ => {
                let aop = if is_pd { sub - 18 } else { sub - 12 };
                if self.fast_nan {
                    self.a.v_farith(is_pd, aop, vd, va, vb);
                } else {
                    self.a.v_farith(is_pd, aop, 2, va, vb);
                    // Any NaN lane -> cold fixup (cbz: NZCV untouched).
                    self.a.v_fcmeq(is_pd, 3, 2, 2);
                    self.a.v_uminv_s(3, 3);
                    self.a.fmov_fg(false, T0, 3);
                    let fix = self.a.cbz(false, T0);
                    self.cold.push(ColdNan { fix, back: self.a.pos, is_d: is_pd, unary: false, va, vb });
                    self.a.v_mov(vd, 2);
                }
            }
        }
        self.xdef(d);
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
    }

    /// 256-bit `vaddps`..`vmaxpd` directly on pinned `(xh, yh)` registers (2 NEON instructions
    /// for register-register ops; 1 `ldp q` + 2 NEON instructions for memory ops).
    pub(super) fn x_fp_packed_256(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let is_mem = op.kind == K_VFP_RM;
        if is_mem {
            self.vec256_mem_op(k, op, true, 1, 7);
        }
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        let va_lo = self.xget(s1);
        let vb_lo = if is_mem { 1 } else { self.xget(op.r1) };
        let vd_lo = xh(d);
        let va_hi = self.yget(s1, 0);
        let vb_hi = if is_mem { 7 } else { self.yget(op.r1, 1) };
        let yh_dst = self.yh_reg(d);
        // Use v5 (not v3/v4) as the unpinned upper-half dest so `flush_minmax_result`
        // (`flush_denorm_lanes`, which clobbers v3/v4) never aliases the result register.
        let vd_hi = yh_dst.unwrap_or(5);
        let is_pd = sub >= 18;
        match sub {
            16 | 17 | 22 | 23 => {
                for (vd, va, vb) in [(vd_lo, va_lo, vb_lo), (vd_hi, va_hi, vb_hi)] {
                    let dst = if vd != va && vd != vb { vd } else { 2 };
                    if sub == 17 || sub == 23 {
                        self.a.v_fcmgt(is_pd, dst, va, vb);
                    } else {
                        self.a.v_fcmgt(is_pd, dst, vb, va);
                    }
                    self.a.v_bsl(dst, va, vb);
                    self.flush_minmax_result(dst, is_pd);
                    if dst != vd {
                        self.a.v_mov(vd, dst);
                    }
                }
            }
            _ => {
                let aop = if is_pd { sub - 18 } else { sub - 12 };
                self.a.v_farith(is_pd, aop, vd_lo, va_lo, vb_lo);
                self.a.v_farith(is_pd, aop, vd_hi, va_hi, vb_hi);
            }
        }
        self.xdef(d);
        if yh_dst.is_some() {
            let bit = 1u16 << d;
            self.ymm_clean &= !bit;
            self.ymm_dirty_pending |= bit;
        } else {
            self.write_ymm_hi(d, vd_hi);
        }
    }

    /// 256-bit vector load (`ld = true`) or store (`ld = false`) for `op`'s memory operand
    /// into/from `(v_lo, v_hi)`, participating in `plan_groups` and emitting a single
    /// `ldp q`/`stp q` when possible.
    pub(super) fn vec256_mem_op(&mut self, k: usize, op: &FastOp, ld: bool, v_lo: u32, v_hi: u32) {
        if ld {
            debug_assert_ne!(v_lo, v_hi, "ldp q requires distinct destination registers");
        }
        let mut chk = Self::chk_of(op);
        if ld && chk == Chk::Ro && op.base == RNONE && op.index == RNONE {
            self.xa_key = None;
            self.mov_lit(XA, op.disp as u64);
            self.a.ldp_q(v_lo, v_hi, XA, 0);
            return;
        }
        let tso = self.vec_tso(k, chk, ld);
        if op.base < RNONE && op.index == RNONE {
            let b = g(op.base);
            let d = op.disp;
            let use_pair = d % 16 == 0 && (-1024..=1008).contains(&d);
            let use_ui = !use_pair && d >= 0 && (d as u64) % 16 == 0 && (d as u64) / 16 + 1 < 4096;
            if use_pair || use_ui {
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
                if !ld && tso {
                    self.a.dmb_ishst();
                }
                if use_pair {
                    if ld {
                        self.a.ldp_q(v_lo, v_hi, b, d as i32);
                    } else {
                        self.a.stp_q(v_lo, v_hi, b, d as i32);
                    }
                } else {
                    self.a.ldst_ui(ld, 128, v_lo, b, d as u32);
                    self.a.ldst_ui(ld, 128, v_hi, b, d as u32 + 16);
                }
                if ld && tso {
                    self.a.dmb_ishld();
                }
                return;
            }
            // Large 16B-aligned displacement `[base + disp]` under `hw_range_enabled()`:
            // share the 512-byte `XA = base + hi` window cache with scalar/vec accesses
            // and emit a single `ldp q`/`stp q [XA, #lo]`.
            if hw_range_enabled() && d % 16 == 0 {
                let cached_lo = match self.xa_key {
                    Some((kb, RNONE, 0, prev_hi)) if kb == op.base => d
                        .checked_sub(prev_hi)
                        .filter(|lo| lo % 16 == 0 && (-1024..=1008).contains(lo)),
                    _ => None,
                };
                let lo = if let Some(lo) = cached_lo {
                    Some(lo)
                } else if let Some(sum) = d.checked_add(256) {
                    let hi = sum & !0x1FF;
                    let lo = d.wrapping_sub(hi);
                    if hi != 0 && lo % 16 == 0 && (-1024..=1008).contains(&lo) {
                        self.a.add_imm(true, XA, b, hi);
                        self.xa_key = Some((op.base, RNONE, 0u8, hi));
                        Some(lo)
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(lo) = lo {
                    if !ld && tso {
                        self.a.dmb_ishst();
                    }
                    if ld {
                        self.a.ldp_q(v_lo, v_hi, XA, lo as i32);
                    } else {
                        self.a.stp_q(v_lo, v_hi, XA, lo as i32);
                    }
                    if ld && tso {
                        self.a.dmb_ishld();
                    }
                    return;
                }
            }
        }
        let a = self.addr_into(op, XA);
        if chk == Chk::Full && !hw_range_enabled() {
            let se = self.side_exit_here(k);
            self.range_check_op(k, a, se);
        }
        if !ld && tso {
            self.a.dmb_ishst();
        }
        if ld {
            self.a.ldp_q(v_lo, v_hi, a, 0);
        } else {
            self.a.stp_q(v_lo, v_hi, a, 0);
        }
        if ld && tso {
            self.a.dmb_ishld();
        }
    }

    /// 256-bit `vmovaps/vmovups/vmovdqa/vmovdqu ymm, m256`: single `ldp q` directly into
    /// `(xh(d), yh(d))` when pinned (or `(xh(d), v0)` + `str q` for `ymm8..15`).
    pub(super) fn x_load_256(&mut self, k: usize, op: &FastOp) {
        let d = op.r0 & 15;
        let vd = xh(d);
        if let Some(vy) = self.yh_reg(d) {
            self.vec256_mem_op(k, op, true, vd, vy);
            self.xdef(d);
            let bit = 1u16 << d;
            self.ymm_clean &= !bit;
            self.ymm_dirty_pending |= bit;
        } else {
            self.vec256_mem_op(k, op, true, vd, 0);
            self.xdef(d);
            self.write_ymm_hi(d, 0);
        }
    }

    /// 256-bit bitwise `vxorps`/`vorps`/`vandps`/`vandnps` with memory operand directly into
    /// `(xh(d), yh(d))`.
    pub(super) fn x_logic_rm_256(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        self.vec256_mem_op(k, op, true, 1, 7);
        let va_lo = self.xget(s1);
        let vd_lo = xh(d);
        let va_hi = self.yget(s1, 0);
        let yh_dst = self.yh_reg(d);
        let vd_hi = yh_dst.unwrap_or(0);
        for (vd, va, vb) in [(vd_lo, va_lo, 1u32), (vd_hi, va_hi, 7u32)] {
            match sub {
                VS_XOR => self.a.v_eor(vd, va, vb),
                VS_OR => self.a.v_orr(vd, va, vb),
                VS_AND => self.a.v_and(vd, va, vb),
                _ => self.a.v_bic(vd, vb, va),
            }
        }
        self.xdef(d);
        if yh_dst.is_some() {
            let bit = 1u16 << d;
            self.ymm_clean &= !bit;
            self.ymm_dirty_pending |= bit;
        } else {
            self.write_ymm_hi(d, vd_hi);
        }
    }

    /// 256-bit register moves & bitwise ops (`VS_MOV`/`VS_XOR`/`VS_OR`/`VS_AND`/`VS_ANDN`).
    pub(super) fn x_move_256(&mut self, op: &FastOp, sub: u8) {
        use fast::*;
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        let s2 = op.r1 & 15;
        let vd_lo = xh(d);
        let yh_dst = self.yh_reg(d);
        if sub == VS_XOR && s1 == s2 {
            self.a.v_zero(vd_lo);
            self.xdef(d);
            self.x_mark_hi(d, true, true);
            self.clear_ymm_hi(d);
            return;
        }
        if sub == VS_MOV {
            if d != s2 {
                self.xsrc(vd_lo, s2);
                self.xdef(d);
                if let Some(vy) = yh_dst {
                    self.ysrc(vy, s2);
                    let bit = 1u16 << d;
                    self.ymm_clean &= !bit;
                    self.ymm_dirty_pending |= bit;
                } else {
                    self.ysrc(0, s2);
                    self.write_ymm_hi(d, 0);
                }
            }
            return;
        }
        let va_lo = self.xget(s1);
        let vb_lo = self.xget(s2);
        let va_hi = self.yget(s1, 0);
        let vb_hi = self.yget(s2, 1);
        let vd_hi = yh_dst.unwrap_or(0);
        for (vd, va, vb) in [(vd_lo, va_lo, vb_lo), (vd_hi, va_hi, vb_hi)] {
            match sub {
                VS_XOR => self.a.v_eor(vd, va, vb),
                VS_OR => self.a.v_orr(vd, va, vb),
                VS_AND => self.a.v_and(vd, va, vb),
                _ => self.a.v_bic(vd, vb, va),
            }
        }
        self.xdef(d);
        if yh_dst.is_some() {
            let bit = 1u16 << d;
            self.ymm_clean &= !bit;
            self.ymm_dirty_pending |= bit;
        } else {
            self.write_ymm_hi(d, vd_hi);
        }
    }

    /// Direct 1-instruction scalar FP load/store (`ldr/str/ldur/stur s/d`) for non-TSO
    /// `[base+disp]` operands (stack with `stack_tso=0`, RO constants, or `PORT_JIT_TSO=0`).
    /// Returns `true` when emitted.
    pub(super) fn scalar_fp_mem_op(&mut self, k: usize, ld: bool, v: u32, op: &FastOp, bits: u32) -> bool {
        if op.base >= RNONE || op.index != RNONE || (bits != 32 && bits != 64) {
            return false;
        }
        let mut chk = Self::chk_of(op);
        let tso = self.scalar_tso(k, chk, ld);
        if tso {
            return false;
        }
        let b = g(op.base);
        let d = op.disp;
        let bytes = (bits / 8) as u64;
        let use_ui = d >= 0 && ((d as u64) & (bytes - 1)) == 0 && ((d as u64) / bytes) < 4096;
        let use_ur = !use_ui && (-256..=255).contains(&d);
        if !use_ui && !use_ur {
            return false;
        }
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
        if use_ui {
            if ld {
                self.a.ldr_v(bits, v, b, d as u32);
            } else {
                self.a.str_v(bits, v, b, d as u32);
            }
        } else {
            self.a.ldur_stur_v(ld, bits, v, b, d as i32);
        }
        true
    }

    /// 128-bit vector load (`ld = true`) or store (`ld = false`) for `op`'s memory operand
    /// into/from vector register `v`, participating in `plan_groups` and using direct
    /// `[base, #disp]` (`ldr q`/`str q`/`ldur q`/`stur q`) when possible.
    pub(super) fn vec128_mem_op(&mut self, k: usize, op: &FastOp, ld: bool, v: u32) {
        let mut chk = Self::chk_of(op);
        if ld && chk == Chk::Ro && op.base == RNONE && op.index == RNONE {
            self.xa_key = None;
            self.mov_lit(XA, op.disp as u64);
            self.a.ldst_ui(true, 128, v, XA, 0);
            return;
        }
        let tso = self.vec_tso(k, chk, ld);
        if op.base < RNONE && op.index == RNONE {
            let b = g(op.base);
            let d = op.disp;
            let use_ui = d >= 0 && (d as u64) % 16 == 0 && (d as u64) / 16 < 4096;
            let use_ur = !use_ui && (-256..=255).contains(&d);
            if use_ui || use_ur {
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
                if !ld && tso {
                    self.a.dmb_ishst();
                }
                if use_ui {
                    self.a.ldst_ui(ld, 128, v, b, d as u32);
                } else if ld {
                    self.a.ldur_q(v, b, d as i32);
                } else {
                    self.a.stur_q(v, b, d as i32);
                }
                if ld && tso {
                    self.a.dmb_ishld();
                }
                return;
            }
            // Large or unaligned-negative displacement `[base + disp]` under `hw_range_enabled()`:
            // share the 512-byte `XA = base + hi` window cache with scalar/vec accesses and emit
            // direct `ldur q`/`stur q [XA, #lo]`.
            if hw_range_enabled() {
                let cached_lo = match self.xa_key {
                    Some((kb, RNONE, 0, prev_hi)) if kb == op.base => {
                        d.checked_sub(prev_hi).filter(|lo| (-256..=255).contains(lo))
                    }
                    _ => None,
                };
                let lo = if let Some(lo) = cached_lo {
                    Some(lo)
                } else if let Some(sum) = d.checked_add(256) {
                    let hi = sum & !0x1FF;
                    let lo = d.wrapping_sub(hi);
                    if hi != 0 && (-256..=255).contains(&lo) {
                        self.a.add_imm(true, XA, b, hi);
                        self.xa_key = Some((op.base, RNONE, 0u8, hi));
                        Some(lo)
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(lo) = lo {
                    if !ld && tso {
                        self.a.dmb_ishst();
                    }
                    if ld {
                        self.a.ldur_q(v, XA, lo as i32);
                    } else {
                        self.a.stur_q(v, XA, lo as i32);
                    }
                    if ld && tso {
                        self.a.dmb_ishld();
                    }
                    return;
                }
            }
        }
        let a = self.addr_into(op, XA);
        if chk == Chk::Full && !hw_range_enabled() {
            let se = self.side_exit_here(k);
            self.range_check_op(k, a, se);
        }
        if !ld && tso {
            self.a.dmb_ishst();
        }
        self.a.ldst_ui(ld, 128, v, a, 0);
        if ld && tso {
            self.a.dmb_ishld();
        }
    }

    /// `movd/movss xmm, m32`, `movq/movsd xmm, m64`, `movaps/movups/movdqa/movdqu xmm, m128`
    /// (legacy and VEX 128): straight into the cache register.
    pub(super) fn x_load(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let d = op.r0 & 15;
        let vd = xh(d);
        match sub {
            VS_ZX32 => {
                if !self.scalar_fp_mem_op(k, true, vd, op, 32) {
                    self.load_op(k, T0, op, 32);
                    self.a.fmov_gf(false, vd, T0);
                }
                self.xdef(d);
                self.x_mark_hi(d, true, true);
            }
            VS_ZX64 => {
                if !self.scalar_fp_mem_op(k, true, vd, op, 64) {
                    self.load_op(k, T0, op, 64);
                    self.a.fmov_gf(true, vd, T0);
                }
                self.xdef(d);
                self.x_mark_hi(d, false, true);
            }
            _ => {
                self.vec128_mem_op(k, op, true, vd);
                self.xdef(d);
            }
        }
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
    }

    /// Direct 3-register 128-bit bitwise op with memory operand (`xorps`/`orps`/`andps`/`andnps`).
    pub(super) fn x_logic_rm(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        let vd = xh(d);
        self.vec128_mem_op(k, op, true, 1);
        let va = self.xget(s1);
        match sub {
            VS_XOR => self.a.v_eor(vd, va, 1),
            VS_OR => self.a.v_orr(vd, va, 1),
            VS_AND => self.a.v_and(vd, va, 1),
            _ => self.a.v_bic(vd, 1, va),
        }
        self.xdef(d);
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
    }

    /// Register moves / merges / logic ops (`VS_MOV..=VS_ANDN`, legacy and VEX 128).
    pub(super) fn x_move(&mut self, k: usize, op: &FastOp, sub: u8) {
        use fast::*;
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        let s2 = op.r1 & 15;
        let vd = xh(d);
        match sub {
            VS_MOV => {
                if d != s2 {
                    let h32 = self.xmm_hi32_zero & (1u16 << s2) != 0;
                    let h64 = self.xmm_hi64_zero & (1u16 << s2) != 0;
                    self.xsrc(vd, s2);
                    self.xdef(d);
                    self.x_mark_hi(d, h32, h64);
                }
            }
            VS_ZX32 | VS_ZX64 => {
                let is_d = sub == VS_ZX64;
                if self.xv & (1u16 << s2) != 0 {
                    self.a.fmov_s(is_d, vd, xh(s2));
                } else {
                    // Scalar vector load: upper bits cleared.
                    self.a.ldr_v(if is_d { 64 } else { 32 }, vd, XCPU, xoff(s2));
                }
                self.xdef(d);
                self.x_mark_hi(d, !is_d, true);
            }
            VS_MRG32 | VS_MRG64 => {
                let lg = if sub == VS_MRG32 { 2 } else { 3 };
                let hi_zero = if lg == 2 {
                    self.xmm_hi32_zero & (1u16 << s1) != 0
                } else {
                    self.xmm_hi64_zero & (1u16 << s1) != 0
                };
                let hi_dead = if lg == 3 {
                    (self.xmm_hi64_live[k + 1] & (1u16 << d)) == 0
                } else {
                    (self.xmm_hi32_live[k + 1] & (1u16 << d)) == 0
                };
                let vb = self.xget(s2);
                let va = self.xget(s1);
                if hi_zero || hi_dead {
                    self.a.fmov_s(lg == 3, vd, vb);
                    self.xdef(d);
                    self.x_mark_hi(d, lg == 2, true);
                } else {
                    if d == s1 {
                        self.a.v_ins_lane(lg, vd, 0, vb, 0);
                    } else if d != s2 {
                        self.a.v_mov(vd, va);
                        self.a.v_ins_lane(lg, vd, 0, vb, 0);
                    } else {
                        // d == s2 != s1: merge in a temp (vd still holds b).
                        self.a.v_mov(0, va);
                        self.a.v_ins_lane(lg, 0, 0, vb, 0);
                        self.a.v_mov(vd, 0);
                    }
                    self.xdef(d);
                }
            }
            VS_XOR if s1 == s2 => {
                self.a.v_zero(vd);
                self.xdef(d);
                self.x_mark_hi(d, true, true);
            }
            _ => {
                let va = self.xget(s1);
                let vb = self.xget(s2);
                match sub {
                    VS_XOR => self.a.v_eor(vd, va, vb),
                    VS_OR => self.a.v_orr(vd, va, vb),
                    VS_AND => self.a.v_and(vd, va, vb),
                    _ => self.a.v_bic(vd, vb, va),
                }
                self.xdef(d);
            }
        }
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
    }

    /// Direct 1-instruction register-to-register NEON shuffles/unpacks directly into `xh(d)`
    /// (zero `v0`/`v1` bounces and zero `tbl` literal pool). Returns `true` if handled.
    pub(super) fn x_shuf_rr(&mut self, op: &FastOp, sub: u8) -> bool {
        use fast::*;
        let d = op.r0 & 15;
        let s1 = (op.imm >> 8) as u8 & 15;
        let s2 = op.r1 & 15;
        let imm8 = (op.imm & 0xFF) as u8;
        let vd = xh(d);
        match sub {
            VS_PSHUFD => {
                let vb = self.xget(s2);
                match imm8 {
                    0x00 => self.a.v_dup_lane_s(vd, vb, 0),
                    0x55 => self.a.v_dup_lane_s(vd, vb, 1),
                    0xAA => self.a.v_dup_lane_s(vd, vb, 2),
                    0xFF => self.a.v_dup_lane_s(vd, vb, 3),
                    0xE4 => self.a.v_mov(vd, vb),
                    0x4E => self.a.v_ext(vd, vb, vb, 8),
                    0xB1 => self.a.v_rev64_4s(vd, vb),
                    _ => return false,
                }
            }
            VS_SHUFPS if s1 == s2 => {
                // `shufps(a, a, imm8)` is identical to `pshufd(a, imm8)`.
                let va = self.xget(s1);
                match imm8 {
                    0x00 => self.a.v_dup_lane_s(vd, va, 0),
                    0x55 => self.a.v_dup_lane_s(vd, va, 1),
                    0xAA => self.a.v_dup_lane_s(vd, va, 2),
                    0xFF => self.a.v_dup_lane_s(vd, va, 3),
                    0xE4 => self.a.v_mov(vd, va),
                    0x4E => self.a.v_ext(vd, va, va, 8),
                    0xB1 => self.a.v_rev64_4s(vd, va),
                    _ => return false,
                }
            }
            VS_SHUFPS => {
                let va = self.xget(s1);
                let vb = self.xget(s2);
                match imm8 {
                    0x44 => self.a.v_zip1(3, vd, va, vb),
                    0xEE => self.a.v_zip2(3, vd, va, vb),
                    0x88 => self.a.v_uzp1(2, vd, va, vb),
                    0xDD => self.a.v_uzp2(2, vd, va, vb),
                    0x4E => self.a.v_ext(vd, va, vb, 8),
                    _ => return false,
                }
            }
            VS_UNPCKLBW => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip1(0, vd, va, vb);
            }
            VS_UNPCKLWD => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip1(1, vd, va, vb);
            }
            VS_UNPCKLDQ => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip1(2, vd, va, vb);
            }
            VS_UNPCKLQDQ => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip1(3, vd, va, vb);
            }
            VS_UNPCKHBW => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip2(0, vd, va, vb);
            }
            VS_UNPCKHWD => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip2(1, vd, va, vb);
            }
            VS_UNPCKHDQ => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip2(2, vd, va, vb);
            }
            VS_UNPCKHQDQ => {
                let (va, vb) = (self.xget(s1), self.xget(s2));
                self.a.v_zip2(3, vd, va, vb);
            }
            VS_DDUP => {
                let vb = self.xget(s2);
                self.a.v_zip1(3, vd, vb, vb);
            }
            _ => return false,
        }
        self.xdef(d);
        if op.w & 3 == VM_V128 {
            self.clear_ymm_hi(d);
        }
        true
    }

    /// Vector stores (`K_VST`): from the cache register if valid, else `Cpu::xmm`.
    pub(super) fn x_vst(&mut self, k: usize, op: &FastOp) {
        let s = op.r1 & 15;
        let cached = self.xv & (1u16 << s) != 0;
        match op.w >> 4 {
            2 => {
                if !cached || !self.scalar_fp_mem_op(k, false, xh(s), op, 32) {
                    self.xgpr(32, T0, s);
                    self.store_op(k, T0, op, 32);
                }
            }
            3 => {
                if !cached || !self.scalar_fp_mem_op(k, false, xh(s), op, 64) {
                    self.xgpr(64, T0, s);
                    self.store_op(k, T0, op, 64);
                }
            }
            6 => {
                if cached {
                    self.a.v_umov_d(T0, xh(s), 1);
                } else {
                    self.a.ldr(64, T0, XCPU, xoff(s) + 8);
                }
                self.store_op(k, T0, op, 64);
            }
            4 => {
                let v = if cached {
                    xh(s)
                } else {
                    self.a.ldr(128, 0, XCPU, xoff(s));
                    0
                };
                self.vec128_mem_op(k, op, false, v);
            }
            _ => {
                let v_lo = if cached {
                    xh(s)
                } else {
                    self.a.ldr(128, 0, XCPU, xoff(s));
                    0
                };
                let v_hi = self.yget(s, 1);
                self.vec256_mem_op(k, op, false, v_lo, v_hi);
            }
        }
    }

    /// `comiss/ucomiss/comisd/ucomisd` on the cache registers.
    /// Keeps results live in host NZCV (`Src::Fcmp`) so a following `ja`/`jbe`/`jae`/`jb`/
    /// `jp`/`jnp` emits a single ARM64 conditional branch with zero `Cpu` flag stores/loads!
    pub(super) fn x_comi(&mut self, k: usize, op: &FastOp) {
        let is_sd = (op.x & 0x3F) != 0;
        let bits = if is_sd { 64 } else { 32 };
        let la = self.live[k + 1];
        let mem_b = op.kind == fast::K_COMI_RM;
        let mut loaded_direct = false;
        if mem_b {
            loaded_direct = self.scalar_fp_mem_op(k, true, 1, op, bits);
            if !loaded_direct {
                self.load_op(k, T1, op, bits);
            }
        }
        self.pf = PF_NONE;
        if la & FALL != 0 {
            let vb = if mem_b {
                if !loaded_direct {
                    self.a.fmov_gf(is_sd, 1, T1);
                }
                1
            } else {
                self.xget(op.r1)
            };
            let va = self.xget(op.r0);
            self.a.fcmp(is_sd, va, vb);
            self.fs = FlagState { src: Src::Fcmp, host: la & FALL, mem: self.fs.mem & !FALL };
        } else {
            self.fs = FlagState { src: Src::None, host: 0, mem: self.fs.mem & !FALL };
        }
    }

    /// `cvtss2si/cvttss2si/cvtsd2si/cvttsd2si` reading the cache register directly.
    pub(super) fn x_cvtf2i(&mut self, k: usize, op: &FastOp) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let w = op.w;
        let is_sd = (op.x & 1) != 0;
        let trunc = (op.x & 2) != 0;
        let bits = if is_sd { 64 } else { 32 };
        let src = if op.kind == fast::K_CVTF2I_R {
            let s = op.r1 & 15;
            if self.xv & (1u16 << s) != 0 {
                xh(s)
            } else {
                self.a.ldr_v(bits, 0, XCPU, xoff(s));
                0
            }
        } else {
            self.load_op(k, T0, op, bits);
            self.a.fmov_gf(is_sd, 0, T0);
            0
        };
        let src = if is_sd {
            src
        } else {
            self.a.fcvt_d_s(0, src);
            0
        };
        if trunc {
            self.a.frintz_d(0, src);
        } else {
            self.a.s_frint(true, 4, 0, src);
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

    /// Out-of-line NaN fixups (after the main path).
    pub(super) fn emit_cold_nan(&mut self) {
        let mut cold = core::mem::take(&mut self.cold);
        for c in &cold {
            self.a.bind(c.fix);
            if !c.unary {
                self.a.v_mov(0, c.va);
            }
            if c.vb != 1 {
                self.a.v_mov(1, c.vb);
            }
            self.emit_x86_nan_fixup(c.is_d, c.unary);
            let here = self.a.pos;
            self.a.b();
            self.a.patch(here, c.back);
        }
        cold.clear();
        self.cold = cold;
    }

    // --- AW2_JIT_RETLIT ---

    /// `reg = v` (64-bit code address / import slot constant on the main path): an `ldr`
    /// from the block's literal pool when `mov_imm` would need 3+ words and
    /// `AW2_JIT_RETLIT` is on.
    pub(super) fn mov_lit(&mut self, reg: u32, v: u64) {
        if reg == XA {
            self.xa_key = None;
        }
        let p = self.a.pos;
        self.a.mov_imm(true, reg, v);
        if self.retlit && self.a.pos - p >= 3 && !self.a.overflow {
            self.a.pos = p;
            let at = self.a.ldr_lit(reg);
            self.lits.push((at, v));
        }
    }

    /// `reg = h`, host function pointer resolved for guest VA `va` (`bail`: see
    /// `inline_host_fn`). JIT: [`Self::mov_lit`]. AOT: always a private pool slot,
    /// recorded as a relocation (the pointer differs between builds; the VA doesn't).
    /// `src` != 0: `va` was read from guest import slot `src`.
    pub(super) fn mov_host_lit(&mut self, reg: u32, h: usize, va: u64, bail: bool, src: u64) {
        if !self.aot {
            self.mov_lit(reg, h as u64);
            return;
        }
        let at = self.a.ldr_lit(reg);
        self.rlits.push((at, h as u64, AotReloc { word: 0, kind: RELOC_HOST_FN, bail, va, src }));
    }

    /// `reg = v`, the value of guest import slot `src` at translation time
    /// (`const_slot_plain`). JIT: [`Self::mov_lit`]. AOT: a private pool slot recorded as a
    /// relocation: the slot may hold a different (equivalent) value in the loading process.
    pub(super) fn mov_slot_val(&mut self, reg: u32, v: u64, src: u64) {
        if !self.aot {
            self.mov_lit(reg, v);
            return;
        }
        let at = self.a.ldr_lit(reg);
        self.rlits.push((at, v, AotReloc { word: 0, kind: RELOC_SLOT_VAL, bail: false, va: v, src }));
    }

    /// `reg = win32_rt::kernel32::time::qpc_fast_state_ptr()` (relocated via `RELOC_QPC_STATE` in AOT).
    pub(super) fn mov_qpc_lit(&mut self, reg: u32) {
        let p = win32_rt::kernel32::time::qpc_fast_state_ptr() as u64;
        if !self.aot {
            self.mov_lit(reg, p);
            return;
        }
        let at = self.a.ldr_lit(reg);
        self.rlits.push((at, p, AotReloc { word: 0, kind: RELOC_QPC_STATE, bail: false, va: 0, src: 0 }));
    }

    /// `reg = win32_rt::misc::xinput1_4::xinput_fast_table_ptr()` (relocated via `RELOC_XINPUT_STATE` in AOT).
    pub(super) fn mov_xinput_lit(&mut self, reg: u32) {
        let p = win32_rt::misc::xinput1_4::xinput_fast_table_ptr() as u64;
        if !self.aot {
            self.mov_lit(reg, p);
            return;
        }
        let at = self.a.ldr_lit(reg);
        self.rlits.push((at, p, AotReloc { word: 0, kind: RELOC_XINPUT_STATE, bail: false, va: 0, src: 0 }));
    }

    /// Load a 128-bit constant `(lo, hi)` into vector register `vreg` via a 1-instruction
    /// PC-relative `ldr q, <literal>` from the block's 16-byte-aligned literal pool.
    pub(super) fn mov_lit128(&mut self, vreg: u32, lo: u64, hi: u64) {
        let at = self.a.ldr_q_lit(vreg);
        self.lits128.push((at, lo, hi));
    }

    /// Literal pool (after all out-of-line code; never executed). 8-byte aligned for 64-bit
    /// slots, 16-byte aligned for 128-bit vector slots (blocks start 16-byte aligned).
    pub(super) fn emit_lits(&mut self) {
        if self.lits.is_empty() && self.rlits.is_empty() && self.lits128.is_empty() {
            return;
        }
        if !self.lits128.is_empty() {
            while self.a.pos % 4 != 0 {
                self.a.emit(0xD503_201F); // nop (16-byte align for ldr q)
            }
            let mut lits128 = core::mem::take(&mut self.lits128);
            let mut pool128 = core::mem::take(&mut self.pool128);
            pool128.clear();
            for &(at, lo, hi) in &lits128 {
                let slot = match pool128.iter().find(|&&(plo, phi, _)| plo == lo && phi == hi) {
                    Some(&(_, _, s)) => s,
                    None => {
                        let s = self.a.pos;
                        self.a.emit(lo as u32);
                        self.a.emit((lo >> 32) as u32);
                        self.a.emit(hi as u32);
                        self.a.emit((hi >> 32) as u32);
                        pool128.push((lo, hi, s));
                        s
                    }
                };
                self.a.patch_ldr_lit(at, slot);
            }
            lits128.clear();
            self.lits128 = lits128;
            pool128.clear();
            self.pool128 = pool128;
        }
        if self.lits.is_empty() && self.rlits.is_empty() {
            return;
        }
        if self.a.pos % 2 != 0 {
            self.a.emit(0xD503_201F); // nop (padding)
        }
        let mut lits = core::mem::take(&mut self.lits);
        let mut pool = core::mem::take(&mut self.pool);
        pool.clear();
        for &(at, v) in &lits {
            let slot = match pool.iter().find(|&&(pv, _)| pv == v) {
                Some(&(_, s)) => s,
                None => {
                    let s = self.a.pos;
                    self.a.emit(v as u32);
                    self.a.emit((v >> 32) as u32);
                    pool.push((v, s));
                    s
                }
            };
            self.a.patch_ldr_lit(at, slot);
        }
        lits.clear();
        self.lits = lits;
        pool.clear();
        self.pool = pool;
        let mut rlits = core::mem::take(&mut self.rlits);
        for &(at, v, r) in &rlits {
            let s = self.a.pos;
            self.a.emit(v as u32);
            self.a.emit((v >> 32) as u32);
            self.a.patch_ldr_lit(at, s);
            self.relocs.push(AotReloc { word: s as u32, ..r });
        }
        rlits.clear();
        self.rlits = rlits;
    }
}
