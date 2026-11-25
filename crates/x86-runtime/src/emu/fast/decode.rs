//! Lowering of decoded `iced_x86::Instruction`s to [`FastOp`] records and `cmp`/`test`+`jcc` fusion.

use super::ext::lower_more;
use super::*;
use crate::emu::{Cpu, OpKind, Register};
use iced_x86::{EncodingKind, Instruction, Mnemonic};

#[inline(always)]
pub(super) fn gpr_of(r: Register) -> Option<(u8, u8)> {
    let (idx, bits, hi) = Cpu::reg_idx(r)?;
    if hi {
        return None;
    }
    Some((idx as u8, bits as u8))
}

/// Memory operand → `(base, index, scale_shift, disp)`; `None` for forms the fast path
/// doesn't handle (non-GS segment overrides, 16-bit addressing, ...).
pub(super) fn mem_of(ins: &Instruction) -> Option<(u8, u8, u8, i64)> {
    let gs = match ins.segment_prefix() {
        Register::None => 0u8,
        Register::GS => B_GS,
        _ => return None,
    };
    if ins.is_ip_rel_memory_operand() {
        return Some((RNONE | gs, RNONE, 0, ins.ip_rel_memory_address() as i64));
    }
    let mut a32 = 0u8;
    let base = match ins.memory_base() {
        Register::None => RNONE,
        r => match gpr_of(r) {
            Some((i, 64)) => i,
            Some((i, 32)) => {
                a32 = B_ADDR32;
                i
            }
            _ => return None,
        },
    };
    let index = match ins.memory_index() {
        Register::None => RNONE,
        r => match gpr_of(r) {
            Some((i, 64)) if a32 == 0 => i,
            Some((i, 32)) if base == RNONE || a32 != 0 => {
                a32 = B_ADDR32;
                i
            }
            _ => return None,
        },
    };
    let scale = match ins.memory_index_scale() {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        _ => return None,
    };
    Some((base | gs | a32, index, scale, ins.memory_displacement64() as i64))
}

/// The value the generic `read_op(ins, op, w)` returns for an immediate operand.
pub(super) fn imm_of(ins: &Instruction, op: u32, w: u32) -> Option<u64> {
    let raw = match ins.op_kind(op) {
        OpKind::Immediate8 => ins.immediate8() as u64,
        OpKind::Immediate16 => ins.immediate16() as u64,
        OpKind::Immediate32 => ins.immediate32() as u64,
        OpKind::Immediate64 => ins.immediate64(),
        OpKind::Immediate8to16 => ins.immediate8to16() as u16 as u64,
        OpKind::Immediate8to32 => ins.immediate8to32() as u32 as u64,
        OpKind::Immediate8to64 => ins.immediate8to64() as u64,
        OpKind::Immediate32to64 => ins.immediate32to64() as u64,
        _ => return None,
    };
    Some(if w < 64 { raw & ((1u64 << w) - 1) } else { raw })
}

#[inline(always)]
pub(super) fn mem_bits(ins: &Instruction) -> u32 {
    ins.memory_size().size() as u32 * 8
}

pub(super) fn is_w(w: u32) -> bool {
    matches!(w, 8 | 16 | 32 | 64)
}

/// Mark `cmp`/`test` + `jcc` pairs inside one decoded block for fused execution.
pub(in crate::emu) fn fuse(ops: &mut [FastOp]) {
    for i in 0..ops.len().saturating_sub(1) {
        if ops[i + 1].kind != K_JCC {
            continue;
        }
        let sub = ops[i].x & 0x3F;
        if sub != A_CMP && sub != A_TEST {
            continue;
        }
        ops[i].kind = match ops[i].kind {
            K_ALU_RR => K_CMPJ_RR,
            K_ALU_RI => K_CMPJ_RI,
            K_ALU_RM => K_CMPJ_RM,
            k => k,
        };
    }
}

/// XMM/YMM register index.
#[inline]
pub(super) fn xreg(r: Register) -> Option<u8> {
    Cpu::xmm_idx(r).map(|i| i as u8)
}

/// Destination write mode of a vector instruction (`None` for EVEX and other encodings,
/// which stay generic).
pub(super) fn vmode(ins: &Instruction) -> Option<u8> {
    match ins.encoding() {
        EncodingKind::Legacy => Some(VM_LEG),
        EncodingKind::VEX => {
            let ymm = ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register && Cpu::is_ymm(ins.op_register(0));
            Some(if ymm { VM_V256 } else { VM_V128 })
        }
        _ => None,
    }
}

/// `FastOp::w >> 4` memory size code for an `n`-byte vector read.
pub(super) fn vmem_code(n: usize) -> Option<u8> {
    match n {
        4 => Some(2),
        8 => Some(3),
        16 => Some(4),
        32 => Some(5),
        _ => None,
    }
}

/// Fill memory addressing fields (as `set_mem` in [`lower_inner`]).
pub(super) fn put_mem(ins: &Instruction, f: &mut FastOp) -> Option<()> {
    let (b, i, s, d) = mem_of(ins)?;
    f.base = b;
    f.index = i;
    f.disp = d;
    f.x = (f.x & 0x3F) | (s << 6);
    Some(())
}

/// Lower a V-form instruction (see the K_VSHUF_* comment) to `rr`/`rm` with sub-op `sub`.
/// `y256`: the VEX.256 form may be lowered (the generic handler applies the same op to the
/// upper lane). `mem_bytes`: memory read size override (default `memory_size()`).
pub(super) fn lower_v(ins: &Instruction, f: &mut FastOp, rr: u8, rm: u8, sub: u8, y256: bool, mem_bytes: Option<usize>) -> Option<()> {
    let mode = vmode(ins)?;
    if mode == VM_V256 && !y256 {
        return None;
    }
    let n = ins.op_count();
    if n == 0 || n > 4 || ins.op_kind(0) != OpKind::Register {
        return None;
    }
    let d = xreg(ins.op_register(0))?;
    let has_imm = ins.op_kind(n - 1) == OpKind::Immediate8;
    let nregs = n - has_imm as u32;
    let imm8 = if has_imm { ins.immediate8() } else { 0 };
    let (s1, s2op) = match nregs {
        1 => (d, None),
        2 => (d, Some(1)),
        3 => {
            if ins.op_kind(1) != OpKind::Register {
                return None;
            }
            (xreg(ins.op_register(1))?, Some(2))
        }
        _ => return None,
    };
    f.x = sub;
    match s2op {
        None => {
            f.kind = rr;
            f.r1 = d;
            f.w = mode;
        }
        Some(k) => match ins.op_kind(k) {
            OpKind::Register => {
                f.kind = rr;
                f.r1 = xreg(ins.op_register(k))?;
                f.w = mode;
            }
            OpKind::Memory => {
                let code = vmem_code(mem_bytes.unwrap_or(ins.memory_size().size()))?;
                if mem_bytes.is_none() && (mode == VM_V256) != (code == 5) {
                    return None;
                }
                f.kind = rm;
                put_mem(ins, f)?;
                f.w = mode | (code << 4);
            }
            _ => return None,
        },
    }
    f.r0 = d;
    f.imm = imm8 as u64 | ((s1 as u64) << 8);
    Some(())
}

/// Lower a vector store `[mem] <- xmm/ymm` of `bytes` bytes (op 0 memory, op 1 register).
fn lower_vst(ins: &Instruction, f: &mut FastOp, bytes: usize) -> Option<()> {
    vmode(ins)?;
    if ins.op_count() != 2 || ins.op_kind(0) != OpKind::Memory || ins.op_kind(1) != OpKind::Register {
        return None;
    }
    let s = xreg(ins.op_register(1))?;
    let code = vmem_code(bytes)?;
    f.kind = K_VST;
    f.r1 = s;
    f.w = code << 4;
    put_mem(ins, f)
}

/// Shared lowering for the scalar `movss`/`movsd` (SSE + VEX) forms.
fn lower_movs(ins: &Instruction, f: &mut FastOp, is_sd: bool) -> Option<()> {
    let (bytes, mrg, zx) = if is_sd { (8, VS_MRG64, VS_ZX64) } else { (4, VS_MRG32, VS_ZX32) };
    match (ins.op_kind(0), ins.op_kind(1)) {
        (OpKind::Register, OpKind::Register) => lower_v(ins, f, K_VSHUF_RR, K_VSHUF_RM, mrg, false, None),
        (OpKind::Register, OpKind::Memory) => {
            if ins.memory_size().size() != bytes {
                return None;
            }
            lower_v(ins, f, K_VSHUF_RR, K_VSHUF_RM, zx, false, None)
        }
        (OpKind::Memory, OpKind::Register) => lower_vst(ins, f, bytes),
        _ => None,
    }
}

/// Mnemonic -> (V-form kind pair, sub-op, VEX.256 allowed, memory size override).
fn vop_of(ins: &Instruction) -> Option<(u8, u8, u8, bool, Option<usize>)> {
    use Mnemonic as M;
    let imm_form = ins.op_count() > 0 && ins.op_kind(ins.op_count() - 1) == OpKind::Immediate8;
    let s = |sub: u8| Some((K_VSHUF_RR, K_VSHUF_RM, sub, true, None));
    let i = |sub: u8| Some((K_VINT_RR, K_VINT_RM, sub, true, None));
    let fp = |sub: u8, y: bool| Some((K_VFP_RR, K_VFP_RM, sub, y, None));
    // Shifts: imm8 count or xmm/m128 count (both VEX.128 and VEX.256; memory count reads low 64 bits).
    let sh = |imm_sub: u8, reg_sub: u8| {
        if imm_form {
            Some((K_VSHUF_RR, K_VSHUF_RM, imm_sub, true, None))
        } else {
            Some((K_VSHUF_RR, K_VSHUF_RM, reg_sub, true, Some(8)))
        }
    };
    let arith = |m: M| -> u8 {
        match m {
            M::Addss | M::Vaddss | M::Addsd | M::Vaddsd | M::Addps | M::Vaddps | M::Addpd | M::Vaddpd => 0,
            M::Subss | M::Vsubss | M::Subsd | M::Vsubsd | M::Subps | M::Vsubps | M::Subpd | M::Vsubpd => 1,
            M::Mulss | M::Vmulss | M::Mulsd | M::Vmulsd | M::Mulps | M::Vmulps | M::Mulpd | M::Vmulpd => 2,
            M::Divss | M::Vdivss | M::Divsd | M::Vdivsd | M::Divps | M::Vdivps | M::Divpd | M::Vdivpd => 3,
            M::Minss | M::Vminss | M::Minsd | M::Vminsd | M::Minps | M::Vminps | M::Minpd | M::Vminpd => 4,
            _ => 5,
        }
    };
    let mn = ins.mnemonic();
    match mn {
        M::Movups | M::Movaps | M::Movdqu | M::Movdqa | M::Movapd | M::Movupd | M::Movntdqa | M::Lddqu
        | M::Vmovups | M::Vmovaps | M::Vmovdqu | M::Vmovdqa | M::Vmovapd | M::Vmovupd | M::Vmovntdqa | M::Vlddqu => {
            s(VS_MOV)
        }
        M::Xorps | M::Xorpd | M::Pxor | M::Vxorps | M::Vxorpd | M::Vpxor => s(VS_XOR),
        M::Por | M::Orps | M::Orpd | M::Vpor | M::Vorps | M::Vorpd => s(VS_OR),
        M::Pand | M::Andps | M::Andpd | M::Vpand | M::Vandps | M::Vandpd => s(VS_AND),
        M::Pandn | M::Andnps | M::Andnpd | M::Vpandn | M::Vandnps | M::Vandnpd => s(VS_ANDN),
        M::Punpcklbw | M::Vpunpcklbw => s(VS_UNPCKLBW),
        M::Punpcklwd | M::Vpunpcklwd => s(VS_UNPCKLWD),
        M::Punpckldq | M::Vpunpckldq | M::Unpcklps | M::Vunpcklps => s(VS_UNPCKLDQ),
        M::Punpcklqdq | M::Vpunpcklqdq | M::Unpcklpd | M::Vunpcklpd => s(VS_UNPCKLQDQ),
        M::Punpckhbw | M::Vpunpckhbw => s(VS_UNPCKHBW),
        M::Punpckhwd | M::Vpunpckhwd => s(VS_UNPCKHWD),
        M::Punpckhdq | M::Vpunpckhdq | M::Unpckhps | M::Vunpckhps => s(VS_UNPCKHDQ),
        M::Punpckhqdq | M::Vpunpckhqdq | M::Unpckhpd | M::Vunpckhpd => s(VS_UNPCKHQDQ),
        M::Packsswb | M::Vpacksswb => s(VS_PACKSSWB),
        M::Packssdw | M::Vpackssdw => s(VS_PACKSSDW),
        M::Packuswb | M::Vpackuswb => s(VS_PACKUSWB),
        M::Packusdw | M::Vpackusdw => s(VS_PACKUSDW),
        M::Pshufd | M::Vpshufd => s(VS_PSHUFD),
        M::Vpermilps => s(if imm_form { VS_PSHUFD } else { VS_VPERMILPS }),
        M::Vpermilpd => s(if imm_form { VS_VPERMILPD_I } else { VS_VPERMILPD }),
        M::Pshuflw | M::Vpshuflw => s(VS_PSHUFLW),
        M::Pshufhw | M::Vpshufhw => s(VS_PSHUFHW),
        M::Shufps | M::Vshufps => s(VS_SHUFPS),
        M::Shufpd | M::Vshufpd => s(VS_SHUFPD),
        M::Movlhps | M::Vmovlhps => Some((K_VSHUF_RR, K_VSHUF_RM, VS_UNPCKLQDQ, false, None)),
        M::Movhlps | M::Vmovhlps => Some((K_VSHUF_RR, K_VSHUF_RM, VS_MOVHLPS, false, None)),
        M::Palignr | M::Vpalignr => s(VS_PALIGNR),
        M::Pblendw | M::Vpblendw => s(VS_PBLENDW),
        M::Blendps | M::Vblendps | M::Vpblendd => s(VS_BLENDPS),
        M::Blendpd | M::Vblendpd => s(VS_BLENDPD),
        M::Insertps | M::Vinsertps => {
            if ins.op_count() >= 3 && ins.op_kind(ins.op_count() - 2) == OpKind::Memory {
                Some((K_VSHUF_RR, K_VSHUF_RM, VS_INSERTPS_M, false, Some(4)))
            } else {
                Some((K_VSHUF_RR, K_VSHUF_RM, VS_INSERTPS, false, None))
            }
        }
        M::Pshufb | M::Vpshufb => s(VS_PSHUFB),
        M::Psrlw | M::Vpsrlw => sh(VS_PSRLW_I, VS_PSRLW),
        M::Psraw | M::Vpsraw => sh(VS_PSRAW_I, VS_PSRAW),
        M::Psllw | M::Vpsllw => sh(VS_PSLLW_I, VS_PSLLW),
        M::Psrld | M::Vpsrld => sh(VS_PSRLD_I, VS_PSRLD),
        M::Psrad | M::Vpsrad => sh(VS_PSRAD_I, VS_PSRAD),
        M::Pslld | M::Vpslld => sh(VS_PSLLD_I, VS_PSLLD),
        M::Psrlq | M::Vpsrlq => sh(VS_PSRLQ_I, VS_PSRLQ),
        M::Psllq | M::Vpsllq => sh(VS_PSLLQ_I, VS_PSLLQ),
        M::Psrldq | M::Vpsrldq if imm_form => s(VS_PSRLDQ_I),
        M::Pslldq | M::Vpslldq if imm_form => s(VS_PSLLDQ_I),
        M::Paddb | M::Vpaddb => i(VI_PADDB),
        M::Paddw | M::Vpaddw => i(VI_PADDW),
        M::Paddd | M::Vpaddd => i(VI_PADDD),
        M::Paddq | M::Vpaddq => i(VI_PADDQ),
        M::Psubb | M::Vpsubb => i(VI_PSUBB),
        M::Psubw | M::Vpsubw => i(VI_PSUBW),
        M::Psubd | M::Vpsubd => i(VI_PSUBD),
        M::Psubq | M::Vpsubq => i(VI_PSUBQ),
        M::Paddsb | M::Vpaddsb => i(VI_PADDSB),
        M::Paddsw | M::Vpaddsw => i(VI_PADDSW),
        M::Paddusb | M::Vpaddusb => i(VI_PADDUSB),
        M::Paddusw | M::Vpaddusw => i(VI_PADDUSW),
        M::Psubsb | M::Vpsubsb => i(VI_PSUBSB),
        M::Psubsw | M::Vpsubsw => i(VI_PSUBSW),
        M::Psubusb | M::Vpsubusb => i(VI_PSUBUSB),
        M::Psubusw | M::Vpsubusw => i(VI_PSUBUSW),
        M::Pmullw | M::Vpmullw => i(VI_PMULLW),
        M::Pmulhw | M::Vpmulhw => i(VI_PMULHW),
        M::Pmulhuw | M::Vpmulhuw => i(VI_PMULHUW),
        M::Pmaddwd | M::Vpmaddwd => i(VI_PMADDWD),
        M::Pmaddubsw | M::Vpmaddubsw => i(VI_PMADDUBSW),
        M::Pmuludq | M::Vpmuludq => i(VI_PMULUDQ),
        M::Pmulld | M::Vpmulld => i(VI_PMULLD),
        M::Pmulhrsw | M::Vpmulhrsw => i(VI_PMULHRSW),
        M::Pmuldq | M::Vpmuldq => i(VI_PMULDQ),
        M::Pminub | M::Vpminub => i(VI_PMINUB),
        M::Pmaxub | M::Vpmaxub => i(VI_PMAXUB),
        M::Pminsw | M::Vpminsw => i(VI_PMINSW),
        M::Pmaxsw | M::Vpmaxsw => i(VI_PMAXSW),
        M::Pminsb | M::Vpminsb => i(VI_PMINSB),
        M::Pmaxsb | M::Vpmaxsb => i(VI_PMAXSB),
        M::Pminuw | M::Vpminuw => i(VI_PMINUW),
        M::Pmaxuw | M::Vpmaxuw => i(VI_PMAXUW),
        M::Pminud | M::Vpminud => i(VI_PMINUD),
        M::Pmaxud | M::Vpmaxud => i(VI_PMAXUD),
        M::Pminsd | M::Vpminsd => i(VI_PMINSD),
        M::Pmaxsd | M::Vpmaxsd => i(VI_PMAXSD),
        M::Pavgb | M::Vpavgb => i(VI_PAVGB),
        M::Pavgw | M::Vpavgw => i(VI_PAVGW),
        M::Pcmpeqb | M::Vpcmpeqb => i(VI_PCMPEQB),
        M::Pcmpeqw | M::Vpcmpeqw => i(VI_PCMPEQW),
        M::Pcmpeqd | M::Vpcmpeqd => i(VI_PCMPEQD),
        M::Pcmpeqq | M::Vpcmpeqq => i(VI_PCMPEQQ),
        M::Pcmpgtb | M::Vpcmpgtb => i(VI_PCMPGTB),
        M::Pcmpgtw | M::Vpcmpgtw => i(VI_PCMPGTW),
        M::Pcmpgtd | M::Vpcmpgtd => i(VI_PCMPGTD),
        M::Pcmpgtq | M::Vpcmpgtq => i(VI_PCMPGTQ),
        M::Pabsb | M::Vpabsb => i(VI_PABSB),
        M::Pabsw | M::Vpabsw => i(VI_PABSW),
        M::Pabsd | M::Vpabsd => i(VI_PABSD),
        M::Psignb | M::Vpsignb => i(VI_PSIGNB),
        M::Psignw | M::Vpsignw => i(VI_PSIGNW),
        M::Psignd | M::Vpsignd => i(VI_PSIGND),
        M::Phaddw | M::Vphaddw => i(VI_PHADDW),
        M::Phaddd | M::Vphaddd => i(VI_PHADDD),
        M::Phsubw | M::Vphsubw => i(VI_PHSUBW),
        M::Phsubd | M::Vphsubd => i(VI_PHSUBD),
        M::Psadbw | M::Vpsadbw => i(VI_PSADBW),
        M::Vpsllvd => i(VI_VPSLLVD),
        M::Vpsrlvd => i(VI_VPSRLVD),
        M::Vpsravd => i(VI_VPSRAVD),
        M::Vpsllvq => i(VI_VPSLLVQ),
        M::Vpsrlvq => i(VI_VPSRLVQ),
        M::Pclmulqdq | M::Vpclmulqdq => i(VI_PCLMULQDQ),
        M::Addss | M::Subss | M::Mulss | M::Divss | M::Minss | M::Maxss | M::Vaddss | M::Vsubss | M::Vmulss
        | M::Vdivss | M::Vminss | M::Vmaxss => fp(VF_ADDSS + arith(mn), false),
        M::Addsd | M::Subsd | M::Mulsd | M::Divsd | M::Minsd | M::Maxsd | M::Vaddsd | M::Vsubsd | M::Vmulsd
        | M::Vdivsd | M::Vminsd | M::Vmaxsd => fp(VF_ADDSD + arith(mn), false),
        M::Addps | M::Subps | M::Mulps | M::Divps | M::Minps | M::Maxps | M::Vaddps | M::Vsubps | M::Vmulps
        | M::Vdivps | M::Vminps | M::Vmaxps => fp(VF_ADDPS + arith(mn), true),
        M::Addpd | M::Subpd | M::Mulpd | M::Divpd | M::Minpd | M::Maxpd | M::Vaddpd | M::Vsubpd | M::Vmulpd
        | M::Vdivpd | M::Vminpd | M::Vmaxpd => fp(VF_ADDPD + arith(mn), true),
        M::Sqrtss | M::Vsqrtss => fp(VF_SQRTSS, false),
        M::Sqrtsd | M::Vsqrtsd => fp(VF_SQRTSD, false),
        M::Sqrtps | M::Vsqrtps => fp(VF_SQRTPS, true),
        M::Sqrtpd | M::Vsqrtpd => fp(VF_SQRTPD, true),
        M::Cvtss2sd | M::Vcvtss2sd => fp(VF_CVTSS2SD, false),
        M::Cvtsd2ss | M::Vcvtsd2ss => fp(VF_CVTSD2SS, false),
        M::Cvtdq2ps | M::Vcvtdq2ps => fp(VF_CVTDQ2PS, true),
        M::Cvttps2dq | M::Vcvttps2dq => fp(VF_CVTTPS2DQ, true),
        M::Cvtps2dq | M::Vcvtps2dq => fp(VF_CVTPS2DQ, true),
        M::Rcpss | M::Vrcpss => Some((K_VFP_RR, K_VFP_RM, VF_RCPSS, false, Some(4))),
        M::Rsqrtss | M::Vrsqrtss => Some((K_VFP_RR, K_VFP_RM, VF_RSQRTSS, false, Some(4))),
        M::Rcpps | M::Vrcpps => fp(VF_RCPPS, true),
        M::Rsqrtps | M::Vrsqrtps => fp(VF_RSQRTPS, true),
        M::Addsubps | M::Vaddsubps => fp(VF_ADDSUBPS, true),
        M::Addsubpd | M::Vaddsubpd => fp(VF_ADDSUBPD, true),
        M::Haddps | M::Vhaddps => fp(VF_HADDPS, true),
        M::Hsubps | M::Vhsubps => fp(VF_HSUBPS, true),
        M::Haddpd | M::Vhaddpd => fp(VF_HADDPD, true),
        M::Hsubpd | M::Vhsubpd => fp(VF_HSUBPD, true),
        M::Roundss | M::Vroundss => Some((K_VFP_RR, K_VFP_RM, VF_ROUNDSS, false, Some(4))),
        M::Roundsd | M::Vroundsd => Some((K_VFP_RR, K_VFP_RM, VF_ROUNDSD, false, Some(8))),
        M::Roundps | M::Vroundps => fp(VF_ROUNDPS, true),
        M::Roundpd | M::Vroundpd => fp(VF_ROUNDPD, true),
        M::Cmpss | M::Vcmpss => Some((K_VFP_RR, K_VFP_RM, VF_CMPSS, false, Some(4))),
        M::Cmpsd | M::Vcmpsd if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register => {
            Some((K_VFP_RR, K_VFP_RM, VF_CMPSD, false, Some(8)))
        }
        M::Cmpps | M::Vcmpps => fp(VF_CMPPS, true),
        M::Cmppd | M::Vcmppd => fp(VF_CMPPD, true),
        M::Dpps | M::Vdpps => fp(VF_DPPS, true),
        _ => None,
    }
}

/// Lower SSE/AVX and the extra integer instructions (kinds >= 37).
fn lower_ext(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    use Mnemonic as M;
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };
    match mn {
        // Full-width moves: stores first (op 0 = memory), everything else via the table.
        M::Movups | M::Movaps | M::Movdqu | M::Movdqa | M::Movapd | M::Movupd | M::Movntdq | M::Movntps | M::Movntpd
        | M::Vmovups | M::Vmovaps | M::Vmovdqu | M::Vmovdqa | M::Vmovapd | M::Vmovupd | M::Vmovntdq | M::Vmovntps
        | M::Vmovntpd
            if k0 == OpKind::Memory =>
        {
            if n != 2 || k1 != OpKind::Register {
                return None;
            }
            let ymm = Cpu::is_ymm(ins.op_register(1));
            if ymm && ins.encoding() != EncodingKind::VEX {
                return None;
            }
            lower_vst(ins, &mut f, if ymm { 32 } else { 16 })?;
        }
        M::Movntdq | M::Movntps | M::Movntpd | M::Vmovntdq | M::Vmovntps | M::Vmovntpd => return None,
        M::Movss | M::Vmovss => {
            if n < 2 {
                return None;
            }
            lower_movs(ins, &mut f, false)?;
        }
        M::Movsd | M::Vmovsd => {
            // `Movsd` is also the string move (whose operands are not Register/Memory).
            if n < 2 {
                return None;
            }
            lower_movs(ins, &mut f, true)?;
        }
        M::Movlps | M::Movlpd | M::Vmovlps | M::Vmovlpd => {
            if n < 2 {
                return None;
            }
            if k0 == OpKind::Memory {
                lower_vst(ins, &mut f, 8)?;
            } else {
                lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, VS_MRG64, false, Some(8))?;
            }
        }
        M::Movhps | M::Movhpd | M::Vmovhps | M::Vmovhpd => {
            if n < 2 {
                return None;
            }
            if k0 == OpKind::Memory {
                vmode(ins)?;
                if n != 2 || k1 != OpKind::Register {
                    return None;
                }
                f.kind = K_VST;
                f.r1 = xreg(ins.op_register(1))?;
                f.w = 6 << 4;
                put_mem(ins, &mut f)?;
            } else {
                lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, VS_UNPCKLQDQ, false, Some(8))?;
            }
        }
        M::Movshdup | M::Vmovshdup | M::Movsldup | M::Vmovsldup => {
            lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, VS_PSHUFD, true, None)?;
            let imm8 = if matches!(mn, M::Movshdup | M::Vmovshdup) { 0xF5u64 } else { 0xA0u64 };
            f.imm = (f.imm & !0xFF) | imm8;
        }
        M::Movddup | M::Vmovddup => {
            let ymm = n > 0 && k0 == OpKind::Register && Cpu::is_ymm(ins.op_register(0));
            lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, VS_DDUP, true, if ymm { None } else { Some(8) })?;
        }
        M::Movd | M::Vmovd | M::Movq | M::Vmovq => {
            if n != 2 {
                return None;
            }
            let is_64 = matches!(mn, M::Movq | M::Vmovq);
            let mode = vmode(ins)?;
            if mode == VM_V256 {
                return None;
            }
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) => {
                    match (xreg(ins.op_register(0)), xreg(ins.op_register(1))) {
                        (Some(_), Some(_)) => {
                            if !is_64 {
                                return None;
                            }
                            lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, VS_ZX64, false, None)?;
                        }
                        (Some(d), None) => {
                            let (g, gw) = gpr_of(ins.op_register(1))?;
                            if gw != if is_64 { 64 } else { 32 } {
                                return None;
                            }
                            f.kind = K_MOVD_XG;
                            f.r0 = d;
                            f.r1 = g;
                            f.imm = gw as u64;
                            f.w = mode;
                        }
                        (None, Some(s)) => {
                            let (g, gw) = gpr_of(ins.op_register(0))?;
                            if gw != if is_64 { 64 } else { 32 } {
                                return None;
                            }
                            f.kind = K_MOVD_GX;
                            f.r0 = g;
                            f.r1 = s;
                            f.w = gw;
                        }
                        _ => return None,
                    }
                }
                (OpKind::Register, OpKind::Memory) => {
                    xreg(ins.op_register(0))?;
                    let bytes = if is_64 { 8 } else { 4 };
                    if ins.memory_size().size() != bytes {
                        return None;
                    }
                    lower_v(ins, &mut f, K_VSHUF_RR, K_VSHUF_RM, if is_64 { VS_ZX64 } else { VS_ZX32 }, false, None)?;
                }
                (OpKind::Memory, OpKind::Register) => {
                    lower_vst(ins, &mut f, if is_64 { 8 } else { 4 })?;
                }
                _ => return None,
            }
        }
        M::Cvtsi2ss | M::Vcvtsi2ss | M::Cvtsi2sd | M::Vcvtsi2sd => {
            let mode = vmode(ins)?;
            if mode == VM_V256 || k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let (s1, src) = match n {
                2 => (d, 1),
                3 => (xreg(ins.op_register(1))?, 2),
                _ => return None,
            };
            let width = match ins.op_kind(src) {
                OpKind::Register => {
                    let (g, gw) = gpr_of(ins.op_register(src))?;
                    f.kind = K_CVTSI_R;
                    f.r1 = g;
                    gw
                }
                OpKind::Memory => {
                    f.kind = K_CVTSI_M;
                    put_mem(ins, &mut f)?;
                    mem_bits(ins) as u8
                }
                _ => return None,
            };
            if width != 32 && width != 64 {
                return None;
            }
            f.r0 = d;
            f.x = (f.x & 0xC0) | matches!(mn, M::Cvtsi2sd | M::Vcvtsi2sd) as u8;
            f.imm = width as u64 | ((s1 as u64) << 8);
            f.w = mode;
        }
        M::Comiss | M::Ucomiss | M::Vcomiss | M::Vucomiss | M::Comisd | M::Ucomisd | M::Vcomisd | M::Vucomisd => {
            vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let is_sd = matches!(mn, M::Comisd | M::Ucomisd | M::Vcomisd | M::Vucomisd);
            f.r0 = xreg(ins.op_register(0))?;
            match k1 {
                OpKind::Register => {
                    f.kind = K_COMI_RR;
                    f.r1 = xreg(ins.op_register(1))?;
                }
                OpKind::Memory => {
                    f.kind = K_COMI_RM;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.x = (f.x & 0xC0) | is_sd as u8;
        }
        M::Cvttss2si | M::Cvtss2si | M::Vcvttss2si | M::Vcvtss2si | M::Cvttsd2si | M::Cvtsd2si | M::Vcvttsd2si
        | M::Vcvtsd2si => {
            vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let is_sd = matches!(mn, M::Cvttsd2si | M::Cvtsd2si | M::Vcvttsd2si | M::Vcvtsd2si);
            let trunc = matches!(mn, M::Cvttss2si | M::Vcvttss2si | M::Cvttsd2si | M::Vcvttsd2si);
            let (g, gw) = gpr_of(ins.op_register(0))?;
            if gw != 32 && gw != 64 {
                return None;
            }
            match k1 {
                OpKind::Register => {
                    f.kind = K_CVTF2I_R;
                    f.r1 = xreg(ins.op_register(1))?;
                }
                OpKind::Memory => {
                    if ins.memory_size().size() != if is_sd { 8 } else { 4 } {
                        return None;
                    }
                    f.kind = K_CVTF2I_M;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.r0 = g;
            f.w = gw;
            f.x = (f.x & 0xC0) | is_sd as u8 | (trunc as u8) << 1;
        }
        M::Imul => {
            if n == 1 {
                match k0 {
                    OpKind::Register => {
                        let (a, w) = gpr_of(ins.op_register(0))?;
                        f.kind = K_MUL1_R;
                        f.r0 = a;
                        f.w = w;
                        f.x = 1;
                    }
                    OpKind::Memory => {
                        let w = mem_bits(ins);
                        if !is_w(w) {
                            return None;
                        }
                        f.kind = K_MUL1_M;
                        f.w = w as u8;
                        f.x = 1;
                        put_mem(ins, &mut f)?;
                    }
                    _ => return None,
                }
                return Some(f);
            }
            if n < 2 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            if w < 16 {
                return None;
            }
            f.r0 = a;
            f.w = w;
            let mem = match k1 {
                OpKind::Register => {
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w2 != w {
                        return None;
                    }
                    f.r1 = b;
                    false
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    true
                }
                _ => return None,
            };
            match n {
                2 => f.kind = if mem { K_IMUL_RM } else { K_IMUL_RR },
                3 => {
                    f.imm = imm_of(ins, 2, w as u32)?;
                    f.kind = if mem { K_IMUL_RMI } else { K_IMUL_RRI };
                }
                _ => return None,
            }
        }
        M::Mul | M::Div | M::Idiv => {
            if n != 1 {
                return None;
            }
            let sub = match mn {
                M::Mul => 0,
                M::Div => 2,
                _ => 3,
            };
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    f.kind = K_MUL1_R;
                    f.r0 = a;
                    f.w = w;
                    f.x = sub;
                }
                OpKind::Memory => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.kind = K_MUL1_M;
                    f.w = w as u8;
                    f.x = sub;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
        }
        M::Neg | M::Not => {
            if n != 1 || k0 != OpKind::Register || ins.has_lock_prefix() {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            f.kind = if mn == M::Neg { K_NEG_R } else { K_NOT_R };
            f.r0 = a;
            f.w = w;
        }
        M::Inc | M::Dec => {
            if n != 1 || k0 != OpKind::Memory || ins.has_lock_prefix() {
                return None;
            }
            let w = mem_bits(ins);
            if !is_w(w) {
                return None;
            }
            f.kind = if mn == M::Inc { K_INC_M } else { K_DEC_M };
            f.w = w as u8;
            put_mem(ins, &mut f)?;
        }
        M::Cbw | M::Cwde | M::Cdqe | M::Cwd | M::Cdq | M::Cqo => {
            f.kind = K_CX;
            f.x = match mn {
                M::Cwde => 0,
                M::Cdqe => 1,
                M::Cdq => 2,
                M::Cqo => 3,
                M::Cbw => 4,
                _ => 5,
            };
        }
        M::Vzeroupper => f.kind = K_VZU,
        M::Bswap => {
            if n != 1 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            f.kind = K_BSWAP_R;
            f.r0 = a;
            f.w = w;
        }
        M::Bsf | M::Bsr | M::Tzcnt | M::Lzcnt | M::Popcnt | M::Bt => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            if w < 16 {
                return None;
            }
            f.r0 = a;
            f.w = w;
            let (rr_kind, rm_kind) = match mn {
                M::Bsf => (K_BSF_RR, K_BSF_RM),
                M::Bsr => (K_BSR_RR, K_BSR_RM),
                M::Tzcnt => (K_TZCNT_RR, K_TZCNT_RM),
                M::Lzcnt => (K_LZCNT_RR, K_LZCNT_RM),
                M::Popcnt => (K_POPCNT_RR, K_POPCNT_RM),
                _ => (K_BT_RR, 0),
            };
            match k1 {
                OpKind::Register => {
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w2 != w {
                        return None;
                    }
                    f.r1 = b;
                    f.kind = rr_kind;
                }
                OpKind::Memory if rm_kind != 0 => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = rm_kind;
                }
                OpKind::Immediate8 if mn == M::Bt => {
                    f.imm = ins.immediate8() as u64;
                    f.kind = K_BT_RI;
                }
                _ => return None,
            }
        }
        M::Bts | M::Btr | M::Btc => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            if w < 16 {
                return None;
            }
            f.r0 = a;
            f.w = w;
            f.x = match mn {
                M::Bts => 0,
                M::Btr => 1,
                _ => 2,
            };
            match k1 {
                OpKind::Register => {
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w2 != w {
                        return None;
                    }
                    f.r1 = b;
                    f.kind = K_BIT_RR;
                }
                OpKind::Immediate8 => {
                    f.imm = ins.immediate8() as u64;
                    f.kind = K_BIT_RI;
                }
                _ => return None,
            }
        }
        M::Shld | M::Shrd => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            let (b, w2) = gpr_of(ins.op_register(1))?;
            if w != w2 || w < 16 {
                return None;
            }
            f.r0 = a;
            f.r1 = b;
            f.w = w;
            f.x = (mn == M::Shrd) as u8;
            let mask = if w == 64 { 63 } else { 31 };
            match ins.op_kind(2) {
                OpKind::Immediate8 => {
                    let cnt = (ins.immediate8() as u64) & mask;
                    if cnt == 0 {
                        f.kind = K_NOP;
                        f.x = 0;
                        return Some(f);
                    }
                    if cnt > w as u64 {
                        return None;
                    }
                    f.imm = cnt;
                    f.kind = K_SHD_RI;
                }
                OpKind::Register if ins.op_register(2) == Register::CL => {
                    f.kind = K_SHD_RCL;
                }
                _ => return None,
            }
        }
        M::Rol | M::Ror => {
            if n != 2 {
                return None;
            }
            f.x = (mn == M::Ror) as u8;
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    f.r0 = a;
                    f.w = w;
                    match k1 {
                        OpKind::Immediate8 => {
                            f.imm = (ins.immediate8() as u64) & if w == 64 { 63 } else { 31 };
                            f.kind = K_ROT_RI;
                            if f.imm == 0 {
                                // Flags unchanged; a 32-bit destination is still zero-extended.
                                f.kind = if w == 32 { K_MOV_RR } else { K_NOP };
                                f.r1 = a;
                                f.x = 0;
                            }
                        }
                        OpKind::Register if ins.op_register(1) == Register::CL => {
                            f.kind = K_ROT_RCL;
                        }
                        _ => return None,
                    }
                }
                OpKind::Memory => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.w = w as u8;
                    put_mem(ins, &mut f)?;
                    match k1 {
                        OpKind::Immediate8 => {
                            f.imm = (ins.immediate8() as u64) & if w == 64 { 63 } else { 31 };
                            if f.imm == 0 {
                                return None;
                            }
                            f.kind = K_ROT_MI;
                        }
                        OpKind::Register if ins.op_register(1) == Register::CL => {
                            f.kind = K_ROT_MCL;
                        }
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        M::Movbe => {
            if n != 2 {
                return None;
            }
            match (k0, k1) {
                (OpKind::Register, OpKind::Memory) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if (w != 16 && w != 32 && w != 64) || mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_MOVBE_RM;
                    f.r0 = a;
                    f.w = w;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Memory, OpKind::Register) => {
                    let (b, w) = gpr_of(ins.op_register(1))?;
                    if (w != 16 && w != 32 && w != 64) || mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_MOVBE_MR;
                    f.r1 = b;
                    f.w = w;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
        }
        M::Pmovmskb | M::Vpmovmskb | M::Movmskps | M::Vmovmskps | M::Movmskpd | M::Vmovmskpd => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            let s = xreg(ins.op_register(1))?;
            let is_256 = Cpu::is_ymm(ins.op_register(1));
            if is_256 && ins.encoding() != EncodingKind::VEX {
                return None;
            }
            let base_op = match mn {
                M::Pmovmskb | M::Vpmovmskb => 0,
                M::Movmskps | M::Vmovmskps => 2,
                _ => 4,
            };
            f.kind = K_MOVMSK;
            f.r0 = d;
            f.r1 = s;
            f.w = w;
            f.x = base_op + (is_256 as u8);
        }
        M::Pinsrb | M::Vpinsrb | M::Pinsrw | M::Vpinsrw | M::Pinsrd | M::Vpinsrd | M::Pinsrq | M::Vpinsrq => {
            let mode = vmode(ins)?;
            if mode == VM_V256 || k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let (s1, src_op, imm_op) = match n {
                3 => (d, 1, 2),
                4 => (xreg(ins.op_register(1))?, 2, 3),
                _ => return None,
            };
            if ins.op_kind(imm_op) != OpKind::Immediate8 {
                return None;
            }
            let lg_bytes: u8 = match mn {
                M::Pinsrb | M::Vpinsrb => 0,
                M::Pinsrw | M::Vpinsrw => 1,
                M::Pinsrd | M::Vpinsrd => 2,
                _ => 3,
            };
            let max_lanes = 16u8 >> lg_bytes;
            let lane = ins.immediate8() & (max_lanes - 1);
            match ins.op_kind(src_op) {
                OpKind::Register => {
                    let (g, gw) = gpr_of(ins.op_register(src_op))?;
                    if gw < 32 {
                        return None;
                    }
                    f.r1 = g;
                    f.x = 0;
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    f.x |= 0x20;
                }
                _ => return None,
            }
            f.kind = K_PINSR;
            f.r0 = d;
            f.w = mode | (lg_bytes << 4);
            f.imm = (lane as u64) | ((s1 as u64) << 8);
        }
        M::Pextrb | M::Vpextrb | M::Pextrw | M::Vpextrw | M::Pextrd | M::Vpextrd | M::Pextrq | M::Vpextrq
        | M::Extractps | M::Vextractps => {
            let mode = vmode(ins)?;
            if mode == VM_V256 || n != 3 || k1 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            let s = xreg(ins.op_register(1))?;
            let elem_bits: u8 = match mn {
                M::Pextrb | M::Vpextrb => 8,
                M::Pextrw | M::Vpextrw => 16,
                M::Pextrd | M::Vpextrd | M::Extractps | M::Vextractps => 32,
                _ => 64,
            };
            let max_lanes = 128 / elem_bits;
            let lane = ins.immediate8() & (max_lanes - 1);
            match k0 {
                OpKind::Register => {
                    let (g, gw) = gpr_of(ins.op_register(0))?;
                    if gw != 32 && gw != 64 {
                        return None;
                    }
                    f.r0 = g;
                    f.x = 0;
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    f.x |= 0x20;
                }
                _ => return None,
            }
            f.kind = K_PEXTR;
            f.r1 = s;
            f.w = elem_bits;
            f.imm = lane as u64;
        }
        M::Xchg | M::Xadd | M::Cmpxchg => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            let (b, w2) = gpr_of(ins.op_register(1))?;
            if w2 != w {
                return None;
            }
            f.kind = match mn {
                M::Xchg => K_XCHG_RR,
                M::Xadd => K_XADD_RR,
                _ => K_CMPXCHG_RR,
            };
            f.r0 = a;
            f.r1 = b;
            f.w = w;
        }
        M::Leave => {
            f.kind = K_LEAVE;
        }
        M::Clc | M::Stc | M::Cmc | M::Cld | M::Std | M::Mfence | M::Sfence | M::Lfence
        | M::Serialize => {
            f.kind = K_FLAG_OP;
            f.x = match mn {
                M::Clc => 0,
                M::Stc => 1,
                M::Cmc => 2,
                M::Cld => 3,
                M::Std => 4,
                M::Serialize => 6,
                _ => 5,
            };
        }
        _ => {
            let (rr, rm, sub, y256, mem_bytes) = vop_of(ins)?;
            lower_v(ins, &mut f, rr, rm, sub, y256, mem_bytes)?;
        }
    }
    Some(f)
}

/// Lower one decoded instruction (falls back to [`K_GENERIC`]).
pub(in crate::emu) fn lower(ins: &Instruction) -> FastOp {
    let mut f = FastOp { ip: ins.ip(), len: ins.len() as u8, ..FastOp::default() };
    if crate::emu::mmx::uses_mmx(ins) { return f; }
    if let Some(g) = lower_inner(ins, f) {
        f = g;
    } else if let Some(g) = lower_ext(ins, f) {
        f = g;
    } else if let Some(g) = lower_more(ins, f) {
        f = g;
    }
    f
}

fn lower_inner(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    let mn = ins.mnemonic();
    let k0 = if ins.op_count() > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if ins.op_count() > 1 { ins.op_kind(1) } else { OpKind::Register };
    let set_mem = |f: &mut FastOp| -> Option<()> {
        let (b, i, s, d) = mem_of(ins)?;
        f.base = b;
        f.index = i;
        f.disp = d;
        f.x = (f.x & 0x3F) | (s << 6);
        Some(())
    };
    match mn {
        Mnemonic::Nop | Mnemonic::Reservednop | Mnemonic::Pause | Mnemonic::Endbr64 | Mnemonic::Endbr32
        | Mnemonic::Clflush | Mnemonic::Prefetch | Mnemonic::Prefetcht0 | Mnemonic::Prefetcht1
        | Mnemonic::Prefetcht2 | Mnemonic::Prefetchnta | Mnemonic::Prefetchw | Mnemonic::Prefetchwt1
        | Mnemonic::Wait | Mnemonic::Rdsspd | Mnemonic::Rdsspq => {
            f.kind = K_NOP;
        }
        Mnemonic::Movnti => {
            if ins.op_count() != 2 || k0 != OpKind::Memory || k1 != OpKind::Register {
                return None;
            }
            let (b, w) = gpr_of(ins.op_register(1))?;
            if (w != 32 && w != 64) || mem_bits(ins) != w as u32 {
                return None;
            }
            f.kind = K_MOV_MR;
            f.r1 = b;
            f.w = w;
            set_mem(&mut f)?;
        }
        Mnemonic::Mov => {
            if ins.op_count() != 2 {
                return None;
            }
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w != w2 {
                        return None;
                    }
                    f.kind = K_MOV_RR;
                    f.r0 = a;
                    f.r1 = b;
                    f.w = w;
                }
                (OpKind::Register, OpKind::Memory) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_MOV_RM;
                    f.r0 = a;
                    f.w = w;
                    set_mem(&mut f)?;
                }
                (OpKind::Memory, OpKind::Register) => {
                    let (b, w) = gpr_of(ins.op_register(1))?;
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_MOV_MR;
                    f.r1 = b;
                    f.w = w;
                    set_mem(&mut f)?;
                }
                (OpKind::Register, _) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    f.imm = imm_of(ins, 1, w as u32)?;
                    f.kind = K_MOV_RI;
                    f.r0 = a;
                    f.w = w;
                }
                (OpKind::Memory, _) => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.imm = imm_of(ins, 1, w)?;
                    f.kind = K_MOV_MI;
                    f.w = w as u8;
                    set_mem(&mut f)?;
                }
                _ => return None,
            }
        }
        Mnemonic::Movzx | Mnemonic::Movsx | Mnemonic::Movsxd => {
            if ins.op_count() != 2 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            let zx = mn == Mnemonic::Movzx;
            match k1 {
                OpKind::Register => {
                    let (b, sw) = gpr_of(ins.op_register(1))?;
                    f.kind = if zx { K_MOVZX_RR } else { K_MOVSX_RR };
                    f.r1 = b;
                    f.x = sw;
                }
                OpKind::Memory => {
                    let sw = mem_bits(ins);
                    if !is_w(sw) {
                        return None;
                    }
                    f.kind = if zx { K_MOVZX_RM } else { K_MOVSX_RM };
                    f.x = sw as u8;
                    set_mem(&mut f)?;
                }
                _ => return None,
            }
            if f.x & 0x3F == 64 {
                // Source width 64 doesn't fit the 6-bit field; leave it to the generic path.
                return None;
            }
            f.r0 = a;
            f.w = w;
        }
        Mnemonic::Lea => {
            if k0 != OpKind::Register || k1 != OpKind::Memory {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            f.kind = K_LEA;
            f.r0 = a;
            f.w = w;
            set_mem(&mut f)?;
        }
        Mnemonic::Add | Mnemonic::Sub | Mnemonic::And | Mnemonic::Or | Mnemonic::Xor | Mnemonic::Cmp
        | Mnemonic::Test | Mnemonic::Adc | Mnemonic::Sbb => {
            if ins.op_count() != 2 || ins.has_lock_prefix() {
                return None;
            }
            f.x = match mn {
                Mnemonic::Add => A_ADD,
                Mnemonic::Sub => A_SUB,
                Mnemonic::And => A_AND,
                Mnemonic::Or => A_OR,
                Mnemonic::Xor => A_XOR,
                Mnemonic::Cmp => A_CMP,
                Mnemonic::Adc => A_ADC,
                Mnemonic::Sbb => A_SBB,
                _ => A_TEST,
            };
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w != w2 {
                        return None;
                    }
                    f.kind = K_ALU_RR;
                    f.r0 = a;
                    f.r1 = b;
                    f.w = w;
                }
                (OpKind::Register, OpKind::Memory) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_ALU_RM;
                    f.r0 = a;
                    f.w = w;
                    set_mem(&mut f)?;
                }
                (OpKind::Memory, OpKind::Register) => {
                    let (b, w) = gpr_of(ins.op_register(1))?;
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_ALU_MR;
                    f.r1 = b;
                    f.w = w;
                    set_mem(&mut f)?;
                }
                (OpKind::Register, _) => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    f.imm = imm_of(ins, 1, w as u32)?;
                    f.kind = K_ALU_RI;
                    f.r0 = a;
                    f.w = w;
                }
                (OpKind::Memory, _) => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.imm = imm_of(ins, 1, w)?;
                    f.kind = K_ALU_MI;
                    f.w = w as u8;
                    set_mem(&mut f)?;
                }
                _ => return None,
            }
        }
        Mnemonic::Inc | Mnemonic::Dec => {
            if ins.op_count() != 1 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            f.kind = if mn == Mnemonic::Inc { K_INC_R } else { K_DEC_R };
            f.r0 = a;
            f.w = w;
        }
        Mnemonic::Shl | Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar => {
            if ins.op_count() != 2 {
                return None;
            }
            f.x = match mn {
                Mnemonic::Shr => S_SHR,
                Mnemonic::Sar => S_SAR,
                _ => S_SHL,
            };
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    f.r0 = a;
                    f.w = w;
                    let mask = if w == 64 { 63 } else { 31 };
                    match k1 {
                        OpKind::Immediate8 => {
                            let cnt = (ins.immediate8() as u32) & mask;
                            if cnt == 0 {
                                // Flags unchanged; a 32-bit destination is still zero-extended.
                                f.kind = if w == 32 { K_MOV_RR } else { K_NOP };
                                f.r1 = a;
                                return Some(f);
                            }
                            f.kind = K_SHIFT_RI;
                            f.imm = cnt as u64;
                        }
                        OpKind::Register if ins.op_register(1) == Register::CL => {
                            f.kind = K_SHIFT_RCL;
                        }
                        _ => return None,
                    }
                }
                OpKind::Memory => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.w = w as u8;
                    set_mem(&mut f)?;
                    let mask = if w == 64 { 63 } else { 31 };
                    match k1 {
                        OpKind::Immediate8 => {
                            let cnt = (ins.immediate8() as u32) & mask;
                            if cnt == 0 {
                                return None;
                            }
                            f.kind = K_SHIFT_MI;
                            f.imm = cnt as u64;
                        }
                        OpKind::Register if ins.op_register(1) == Register::CL => {
                            f.kind = K_SHIFT_MCL;
                        }
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        Mnemonic::Push | Mnemonic::Pop => {
            if ins.op_count() != 1 {
                return None;
            }
            let push = mn == Mnemonic::Push;
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if w != 64 {
                        return None;
                    }
                    f.kind = if push { K_PUSH_R } else { K_POP_R };
                    f.r0 = a;
                    f.w = 64;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != 64 {
                        return None;
                    }
                    f.kind = if push { K_PUSH_M } else { K_POP_M };
                    f.w = 64;
                    set_mem(&mut f)?;
                }
                _ if push && Cpu::op_bits(ins, 0) != 16 => {
                    f.imm = imm_of(ins, 0, 64)?;
                    f.kind = K_PUSH_I;
                    f.w = 64;
                }
                _ => return None,
            }
        }
        Mnemonic::Ret => {
            match ins.op_count() {
                0 => f.kind = K_RET,
                1 if k0 == OpKind::Immediate16 => {
                    f.kind = K_RET;
                    f.imm = ins.immediate16() as u64;
                }
                _ => return None,
            }
        }
        _ if ins.is_jcc_short_or_near() => {
            let cc = ins.condition_code() as u8;
            if cc == 0 {
                return None;
            }
            f.kind = K_JCC;
            f.x = cc;
            f.imm = ins.near_branch_target();
        }
        _ if ins.is_jmp_short_or_near() => {
            f.kind = K_JMP;
            f.imm = ins.near_branch_target();
        }
        Mnemonic::Call | Mnemonic::Jmp => {
            if ins.op_count() != 1 {
                return None;
            }
            let call = mn == Mnemonic::Call;
            match k0 {
                OpKind::NearBranch64 if call => {
                    f.kind = K_CALL;
                    f.imm = ins.near_branch_target();
                }
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if w != 64 {
                        return None;
                    }
                    f.kind = if call { K_CALL_R } else { K_JMP_R };
                    f.r0 = a;
                    f.w = 64;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != 64 {
                        return None;
                    }
                    f.kind = if call { K_CALL_M } else { K_JMP_M };
                    f.w = 64;
                    set_mem(&mut f)?;
                }
                _ => return None,
            }
        }
        Mnemonic::Cmova | Mnemonic::Cmovae | Mnemonic::Cmovb | Mnemonic::Cmovbe | Mnemonic::Cmove
        | Mnemonic::Cmovg | Mnemonic::Cmovge | Mnemonic::Cmovl | Mnemonic::Cmovle | Mnemonic::Cmovne
        | Mnemonic::Cmovno | Mnemonic::Cmovnp | Mnemonic::Cmovns | Mnemonic::Cmovo | Mnemonic::Cmovp
        | Mnemonic::Cmovs => {
            if ins.op_count() != 2 || k0 != OpKind::Register {
                return None;
            }
            let cc = ins.condition_code() as u8;
            if cc == 0 {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            f.r0 = a;
            f.w = w;
            f.x = cc;
            match k1 {
                OpKind::Register => {
                    let (b, w2) = gpr_of(ins.op_register(1))?;
                    if w != w2 {
                        return None;
                    }
                    f.kind = K_CMOV_RR;
                    f.r1 = b;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.kind = K_CMOV_RM;
                    set_mem(&mut f)?;
                    // set_mem keeps the low 6 bits (the condition code).
                }
                _ => return None,
            }
        }
        Mnemonic::Seta | Mnemonic::Setae | Mnemonic::Setb | Mnemonic::Setbe | Mnemonic::Sete
        | Mnemonic::Setg | Mnemonic::Setge | Mnemonic::Setl | Mnemonic::Setle | Mnemonic::Setne
        | Mnemonic::Setno | Mnemonic::Setnp | Mnemonic::Setns | Mnemonic::Seto | Mnemonic::Setp
        | Mnemonic::Sets => {
            if ins.op_count() != 1 {
                return None;
            }
            let cc = ins.condition_code() as u8;
            if cc == 0 {
                return None;
            }
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if w != 8 {
                        return None;
                    }
                    f.kind = K_SETCC_R;
                    f.r0 = a;
                    f.w = 8;
                    f.x = cc;
                }
                OpKind::Memory => {
                    f.kind = K_SETCC_M;
                    f.w = 8;
                    f.x = cc;
                    set_mem(&mut f)?;
                }
                _ => return None,
            }
        }
        _ => return None,
    }
    Some(f)
}
