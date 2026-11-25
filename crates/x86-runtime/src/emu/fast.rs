//! Pre-decoded fast path for [`super::emu_run_full`].
//!
//! When a block is decoded, every instruction also gets a [`FastOp`]: the most common
//! instruction forms (`mov`/`lea`/`movzx`/`movsx`, integer ALU ops, `inc`/`dec`, shifts by
//! immediate or `cl`, `push`/`pop`/`ret`, `jcc`/`jmp`, `cmovcc`/`setcc`, nops) are lowered
//! to a compact record with GPR indices, widths, pre-extended immediates and addressing
//! terms already resolved. Kinds >= [`K_VSHUF_RR`] (run by the out-of-line `exec_ext`)
//! cover the hot SSE/AVX forms (full-width moves and stores, logic/unpack/pack/shuffle,
//! immediate and xmm-count shifts, packed integer arithmetic/compares, scalar and packed
//! float arithmetic and conversions, `movss`/`movsd`/`movd`/`movq`, `cvtsi2ss/sd`,
//! `(u)comiss/sd`, `cvt(t)ss/sd2si`, `vzeroupper`) in legacy, VEX.128 and (where the generic
//! handler supports it) VEX.256 encodings, plus `imul`, `neg`/`not`, memory `inc`/`dec`,
//! `cwde`-family, `bsf`, `bt`, `rol`/`ror` by immediate and register `xchg`. Everything else
//! is [`K_GENERIC`] and runs through the generic `iced_x86::Instruction` interpreter.
//!
//! Every fast handler reproduces the generic handler's observable behaviour exactly
//! (register writes incl. partial-register merging and `last_bits`, memory accesses and
//! their validity checks, the generic flag computations, VEX.128 upper-half zeroing vs
//! legacy-SSE upper-half preservation, NaN propagation). `AW2_EMU_VERIFY=1` runs each
//! eligible fast op on a shadow CPU and compares it (GPRs, flags, xmm/ymm) against the
//! generic result; `emu_tests.rs` checks both paths against Rosetta-captured vectors.

mod decode;
mod exec;
mod ext;
mod more;
#[cfg(test)]
mod tests;

pub(super) use decode::{fuse, lower};
pub(super) use exec::{exec, run_chain, verifiable};
#[cfg_attr(not(target_arch = "aarch64"), allow(unused_imports))]
pub(super) use ext::{
    K_BMI_RM, K_BMI_RR, K_INSN, K_LOOP, K_PMOVX, K_VBLENDV_RM, K_VBLENDV_RR, K_VBROADCAST,
    K_VCVT_PD, K_VFMA_RM, K_VFMA_RR, K_VTEST_RM, K_VTEST_RR, K_YMM_LANE,
};
#[cfg_attr(not(target_arch = "aarch64"), allow(unused_imports))]
pub(super) use more::{
    K_ATOMIC, K_BIT_EXT, K_HI8, K_PCMPSTR, K_STRING, K_SYS_MISC, K_VAES_RM, K_VAES_RR, K_VCVT_PH,
    K_VGFNI_RM, K_VGFNI_RR, K_VMASKMOV, K_VSHA_RM, K_VSHA_RR, K_VVNNI_RM, K_VVNNI_RR,
};

#[cfg(not(target_endian = "little"))]
compile_error!("emu_fast vector lanes assume a little-endian host");

/// "No register" marker for `base`/`index` (reads as 0).
pub(super) const RNONE: u8 = 16;
/// Bit flag in `FastOp::base` indicating a `%gs:` segment-prefixed memory operand.
pub(super) const B_GS: u8 = 0x20;
/// Bit flag in `FastOp::base` indicating a 32-bit address-size override (`0x67` prefix):
/// effective address wraps modulo `2^32` and zero-extends before adding `%gs:` base.
pub(super) const B_ADDR32: u8 = 0x40;

pub(super) const K_GENERIC: u8 = 0;
pub(super) const K_NOP: u8 = 1;
pub(super) const K_MOV_RR: u8 = 2;
pub(super) const K_MOV_RM: u8 = 3;
pub(super) const K_MOV_MR: u8 = 4;
pub(super) const K_MOV_RI: u8 = 5;
pub(super) const K_MOV_MI: u8 = 6;
pub(super) const K_MOVZX_RR: u8 = 7;
pub(super) const K_MOVZX_RM: u8 = 8;
pub(super) const K_MOVSX_RR: u8 = 9;
pub(super) const K_MOVSX_RM: u8 = 10;
pub(super) const K_LEA: u8 = 11;
pub(super) const K_ALU_RR: u8 = 12;
pub(super) const K_ALU_RI: u8 = 13;
pub(super) const K_ALU_RM: u8 = 14;
pub(super) const K_ALU_MR: u8 = 15;
pub(super) const K_ALU_MI: u8 = 16;
pub(super) const K_INC_R: u8 = 17;
pub(super) const K_DEC_R: u8 = 18;
pub(super) const K_SHIFT_RI: u8 = 19;
/// Shift by CL. Count (masked) 0: flags unchanged, but a w=32 destination is still written
/// (upper half zeroed), matching hardware.
pub(super) const K_SHIFT_RCL: u8 = 20;
pub(super) const K_PUSH_R: u8 = 21;
pub(super) const K_POP_R: u8 = 22;
pub(super) const K_RET: u8 = 23;
pub(super) const K_JCC: u8 = 24;
pub(super) const K_JMP: u8 = 25;
pub(super) const K_CMOV_RR: u8 = 26;
pub(super) const K_CMOV_RM: u8 = 27;
pub(super) const K_SETCC_R: u8 = 28;
pub(super) const K_CALL: u8 = 29;
pub(super) const K_CALL_M: u8 = 30;
pub(super) const K_CALL_R: u8 = 31;
pub(super) const K_JMP_M: u8 = 32;
pub(super) const K_JMP_R: u8 = 33;
/// `cmp`/`test` (RR/RI/RM) immediately followed by a `jcc` in the same block: executes the
/// compare and returns [`Fx::FuseJcc`] so the caller runs the next slot (the `jcc`) without
/// a trip through the dispatch loop. The `jcc` slot itself stays a normal [`K_JCC`].
pub(super) const K_CMPJ_RR: u8 = 34;
pub(super) const K_CMPJ_RI: u8 = 35;
pub(super) const K_CMPJ_RM: u8 = 36;

// ---- SSE / AVX kinds ------------------------------------------------------------------------
//
// "V-form" field usage, shared by K_VSHUF_*, K_VINT_* and K_VFP_*:
//   r0             destination XMM/YMM register index (0..15)
//   r1             *_RR: second-source XMM index (the ModRM r/m operand; for legacy
//                  `psrlw xmm, imm8` style shifts it is the destination itself)
//   base/index/disp, x >> 6
//                  *_RM: memory second source, exactly as `set_mem`
//   imm[7:0]       imm8 (shuffle control / shift count), 0 when the instruction has none
//   imm[11:8]      first-source XMM index (VEX.vvvv source; = r0 for 2-operand legacy forms)
//   x & 0x3F       sub-op (VS_* / VI_* / VF_*)
//   w & 3          destination write mode: VM_LEG (legacy SSE: dst bits 255:128 preserved),
//                  VM_V128 (VEX.128: bits 255:128 zeroed), VM_V256 (VEX.256: the upper lane is
//                  computed from the sources' upper lanes by the same per-128-bit-lane op)
//   w >> 4         *_RM: log2 of the memory read size in bytes: 2 = read_mem(32), 3 =
//                  read_mem(64) (zero-extended to 128 bits), 4 = read_u128, 5 = two read_u128
//                  (lo at +0, upper lane at +16; only with VM_V256)
// Semantics: a = xmm[imm[11:8]], b = xmm[r1] or memory; xmm[r0] = op(a, b, imm8); ymm_hi[r0]
// per the write mode (op(ymm_hi[src1], b_hi, imm8) for VM_V256).
/// Moves, logic, unpack/pack, shuffles and shifts (`VS_*` sub-ops).
pub(super) const K_VSHUF_RR: u8 = 37;
pub(super) const K_VSHUF_RM: u8 = 38;
/// Packed integer arithmetic, multiplies, min/max, averages and compares (`VI_*`).
pub(super) const K_VINT_RR: u8 = 39;
pub(super) const K_VINT_RM: u8 = 40;
/// Scalar/packed float arithmetic and float conversions (`VF_*`), x86 NaN/rounding rules.
pub(super) const K_VFP_RR: u8 = 41;
pub(super) const K_VFP_RM: u8 = 42;
/// Vector store `[mem] <- xmm r1`: memory as `set_mem`; `w >> 4` = log2 size in bytes:
/// 2 = low dword (write_mem 32), 3 = low qword (write_mem 64), 4 = xmm (write_u128),
/// 5 = xmm at +0 then ymm_hi at +16 (write_u128 x2). Registers are not modified.
pub(super) const K_VST: u8 = 43;
/// `movd/movq xmm r0, gpr r1`: xmm[r0] = low `imm` (32 or 64) bits of gpr r1, zero-extended;
/// `w & 3` = write mode (VM_LEG or VM_V128).
pub(super) const K_MOVD_XG: u8 = 44;
/// `movd/movq gpr r0, xmm r1`: gpr r0 = low `w` (32 or 64) bits of xmm r1, written like `rset`.
pub(super) const K_MOVD_GX: u8 = 45;
/// `(v)cvtsi2ss/sd xmm r0, [xmm src1,] gpr r1`: low lane of xmm[r0] = float(signed int of
/// width imm[7:0] (32/64)), bits above it from xmm[imm[11:8]]; x & 0x3F: 0 = ss, 1 = sd;
/// `w & 3` = write mode (VM_LEG or VM_V128).
pub(super) const K_CVTSI_R: u8 = 46;
/// Same with a memory integer source (as `set_mem`; read width imm[7:0]).
pub(super) const K_CVTSI_M: u8 = 47;
/// `(v)(u)comiss/(u)comisd xmm r0, xmm r1`: sets ZF/PF/CF (OF = SF = 0) only;
/// x & 0x3F: 0 = ss, 1 = sd.
pub(super) const K_COMI_RR: u8 = 48;
/// Same with a memory second operand (as `set_mem`; read 32 bits for ss, 64 for sd).
pub(super) const K_COMI_RM: u8 = 49;
/// `(v)cvt(t)ss2si/(v)cvt(t)sd2si gpr r0, xmm r1`: `w` = GPR width (32/64, written like
/// `rset`); x & 0x3F: bit 0 = sd source, bit 1 = truncate. Out of range/NaN -> indefinite.
pub(super) const K_CVTF2I_R: u8 = 50;
/// Same with a memory source (as `set_mem`; read 32 bits for ss, 64 for sd).
pub(super) const K_CVTF2I_M: u8 = 51;

// ---- more integer kinds ---------------------------------------------------------------------
/// `imul r0, r1` (2-operand, w = 16/32/64): r0 = low w bits of the signed product;
/// CF = OF = product doesn't fit in w bits; SF/ZF/PF unchanged (as the generic handler).
pub(super) const K_IMUL_RR: u8 = 52;
/// `imul r0, mem` (memory as `set_mem`, read width w).
pub(super) const K_IMUL_RM: u8 = 53;
/// `imul r0, r1, imm` (imm pre-masked to w like `read_op`; sign-extended from w).
pub(super) const K_IMUL_RRI: u8 = 54;
/// `imul r0, mem, imm`.
pub(super) const K_IMUL_RMI: u8 = 55;
/// `neg r0` (non-high-byte GPR, width w): flags as `sub 0, r0`, CF = (r0 != 0).
pub(super) const K_NEG_R: u8 = 56;
/// `not r0` (non-high-byte GPR, width w): flags unchanged.
pub(super) const K_NOT_R: u8 = 57;
/// `inc`/`dec` of memory without `lock` (memory as `set_mem`, width w): flags as add/sub 1
/// with CF preserved. Read-modify-write: not re-executable for verification.
pub(super) const K_INC_M: u8 = 58;
pub(super) const K_DEC_M: u8 = 59;
/// Accumulator sign extensions; x & 0x3F: 0 = cwde, 1 = cdqe, 2 = cdq, 3 = cqo. Like the
/// generic handlers these don't touch `last_bits`.
pub(super) const K_CX: u8 = 60;
/// `vzeroupper`: clears `ymm_hi[0..16]`. No fields used.
pub(super) const K_VZU: u8 = 61;
/// `bsf r0, r1` / `bsf r0, [mem]` (width w = 16/32/64, non-high-byte GPRs). Source 0: ZF=1,
/// CF=1, r0 unchanged; else ZF=0, CF=0, r0 = index (`rset`). Other flags unchanged.
pub(super) const K_BSF_RR: u8 = 62;
pub(super) const K_BSF_RM: u8 = 63;
/// `bt r0, r1` / `bt r0, imm` (register base only, width w = 16/32/64): CF = bit
/// (index & (w-1)) of r0; RI keeps the raw immediate in `imm`. Other flags unchanged.
pub(super) const K_BT_RR: u8 = 64;
pub(super) const K_BT_RI: u8 = 65;
/// `rol`/`ror r0, imm` (non-high-byte GPR, width w): x & 0x3F: 0 = rol, 1 = ror; `imm` = count
/// already masked to 5 (6 for w=64) bits, never 0 (count 0 lowers to K_MOV_RR/K_NOP); CF/OF as generic.
pub(super) const K_ROT_RI: u8 = 66;
/// `xchg r0, r1` (both non-high-byte GPRs, width w): writes r0 then r1 via `rset`.
pub(super) const K_XCHG_RR: u8 = 67;
/// `bswap r0` (non-high-byte GPR, width w = 32/64): reverses byte order; flags unchanged.
pub(super) const K_BSWAP_R: u8 = 68;
/// `bsr r0, r1` / `bsr r0, [mem]` (width w = 16/32/64). Source 0: ZF=1, r0 unchanged;
/// else ZF=0, r0 = 63 - clz64(v). Other flags unchanged.
pub(super) const K_BSR_RR: u8 = 69;
pub(super) const K_BSR_RM: u8 = 70;
/// `tzcnt r0, r1/[mem]` (w = 16/32/64): r0 = ctz(v) (w if v==0), CF = (v==0), ZF = (res==0).
pub(super) const K_TZCNT_RR: u8 = 71;
pub(super) const K_TZCNT_RM: u8 = 72;
/// `lzcnt r0, r1/[mem]` (w = 16/32/64): r0 = clz_w(v), CF = (v==0), ZF = (res==0).
pub(super) const K_LZCNT_RR: u8 = 73;
pub(super) const K_LZCNT_RM: u8 = 74;
/// `popcnt r0, r1/[mem]` (w = 16/32/64): r0 = popcnt(v), ZF = (res==0), CF=SF=OF=PF=0.
pub(super) const K_POPCNT_RR: u8 = 75;
pub(super) const K_POPCNT_RM: u8 = 76;
/// `bts`/`btr`/`btc r0, r1/imm8` (register base, w = 16/32/64; x & 0x3F: 0 = bts, 1 = btr, 2 = btc).
pub(super) const K_BIT_RR: u8 = 77;
pub(super) const K_BIT_RI: u8 = 78;
/// `shld`/`shrd r0, r1, imm8/cl` (w = 16/32/64; x & 0x3F: 0 = shld, 1 = shrd).
pub(super) const K_SHD_RI: u8 = 79;
pub(super) const K_SHD_RCL: u8 = 80;
/// `rol`/`ror r0, cl` (w = 8/16/32/64; x & 0x3F: 0 = rol, 1 = ror).
pub(super) const K_ROT_RCL: u8 = 81;
/// 1-operand `mul`/`imul`/`div`/`idiv` (w = 8/16/32/64; x & 0x3F: 0 = mul, 1 = imul, 2 = div, 3 = idiv).
pub(super) const K_MUL1_R: u8 = 82;
pub(super) const K_MUL1_M: u8 = 83;
/// `xadd r0, r1` (register form, w = 8/16/32/64).
pub(super) const K_XADD_RR: u8 = 84;
/// `cmpxchg r0, r1` (register form, w = 8/16/32/64).
pub(super) const K_CMPXCHG_RR: u8 = 85;
/// `setcc [mem]` (memory as `set_mem`, condition code in `x & 0x3F`).
pub(super) const K_SETCC_M: u8 = 86;
/// 64-bit `push imm`, `push [mem]`, `pop [mem]`, `leave`, and flag/fence ops (`clc`/`stc`/`cmc`/`cld`/`std`/`mfence`).
pub(super) const K_PUSH_I: u8 = 87;
pub(super) const K_PUSH_M: u8 = 88;
pub(super) const K_POP_M: u8 = 89;
pub(super) const K_LEAVE: u8 = 90;
pub(super) const K_FLAG_OP: u8 = 91;
/// Memory shifts (`shl`/`sal`/`shr`/`sar [mem], imm8/cl`, non-`lock`, w = 8/16/32/64).
pub(super) const K_SHIFT_MI: u8 = 92;
pub(super) const K_SHIFT_MCL: u8 = 93;
/// Memory rotates (`rol`/`ror [mem], imm8/cl`, non-`lock`, w = 8/16/32/64).
pub(super) const K_ROT_MI: u8 = 94;
pub(super) const K_ROT_MCL: u8 = 95;
/// `movbe r0, [mem]` / `movbe [mem], r1` (w = 16/32/64).
pub(super) const K_MOVBE_RM: u8 = 96;
pub(super) const K_MOVBE_MR: u8 = 97;
/// `(v)pmovmskb`/`(v)movmskps`/`(v)movmskpd r0, xmm/ymm r1` (`x`: 0..5).
pub(super) const K_MOVMSK: u8 = 98;
/// `(v)pinsrb/w/d/q xmm r0, [xmm s1,] r1/[mem], imm8`: `w & 3` = mode, `w >> 4` = log2(bytes),
/// `x & 0x20 != 0` = memory source, `imm[7:0]` = masked lane, `imm[11:8]` = s1.
pub(super) const K_PINSR: u8 = 99;
/// `(v)pextrb/w/d/q` / `(v)extractps r0/[mem], xmm r1, imm8`: `w` = elem bits (8/16/32/64),
/// `x & 0x20 != 0` = memory destination, `imm` = masked lane.
pub(super) const K_PEXTR: u8 = 100;

// Vector write modes (`FastOp::w & 3` for V-form kinds).
pub(super) const VM_LEG: u8 = 0;
pub(super) const VM_V128: u8 = 1;
pub(super) const VM_V256: u8 = 2;

// K_VSHUF_* sub-ops.
pub(super) const VS_MOV: u8 = 0; // b (movaps/movups/movdqa/movdqu/movapd/movupd/lddqu/movntdqa)
pub(super) const VS_ZX32: u8 = 1; // b & 0xFFFFFFFF (movd/movss loads)
pub(super) const VS_ZX64: u8 = 2; // b & u64::MAX (movq xmm, xmm/m64; movsd loads)
pub(super) const VS_MRG32: u8 = 3; // a[127:32] : b[31:0] (movss xmm, xmm)
pub(super) const VS_MRG64: u8 = 4; // a[127:64] : b[63:0] (movsd xmm, xmm)
pub(super) const VS_XOR: u8 = 5; // pxor/xorps/xorpd
pub(super) const VS_OR: u8 = 6; // por/orps/orpd
pub(super) const VS_AND: u8 = 7; // pand/andps/andpd
pub(super) const VS_ANDN: u8 = 8; // pandn/andnps/andnpd: !a & b
pub(super) const VS_UNPCKLBW: u8 = 9;
pub(super) const VS_UNPCKLWD: u8 = 10;
pub(super) const VS_UNPCKLDQ: u8 = 11; // also unpcklps
pub(super) const VS_UNPCKLQDQ: u8 = 12; // also unpcklpd
pub(super) const VS_UNPCKHBW: u8 = 13;
pub(super) const VS_UNPCKHWD: u8 = 14;
pub(super) const VS_UNPCKHDQ: u8 = 15; // also unpckhps
pub(super) const VS_UNPCKHQDQ: u8 = 16; // also unpckhpd
pub(super) const VS_PACKSSWB: u8 = 17;
pub(super) const VS_PACKSSDW: u8 = 18;
pub(super) const VS_PACKUSWB: u8 = 19;
pub(super) const VS_PACKUSDW: u8 = 20;
pub(super) const VS_PSHUFD: u8 = 21; // b, imm8
pub(super) const VS_PSHUFLW: u8 = 22; // b, imm8
pub(super) const VS_PSHUFHW: u8 = 23; // b, imm8
pub(super) const VS_SHUFPS: u8 = 24; // a, b, imm8
pub(super) const VS_PSHUFB: u8 = 25; // a, control b
// Shifts by imm8: value b, count imm8.
pub(super) const VS_PSRLW_I: u8 = 26;
pub(super) const VS_PSRAW_I: u8 = 27;
pub(super) const VS_PSLLW_I: u8 = 28;
pub(super) const VS_PSRLD_I: u8 = 29;
pub(super) const VS_PSRAD_I: u8 = 30;
pub(super) const VS_PSLLD_I: u8 = 31;
pub(super) const VS_PSRLQ_I: u8 = 32;
pub(super) const VS_PSLLQ_I: u8 = 33;
pub(super) const VS_PSRLDQ_I: u8 = 34; // byte shift
pub(super) const VS_PSLLDQ_I: u8 = 35; // byte shift
// Shifts by xmm/m64 count: value a, count = low 64 bits of b (memory forms read 64 bits).
pub(super) const VS_PSRLW: u8 = 36;
pub(super) const VS_PSRAW: u8 = 37;
pub(super) const VS_PSLLW: u8 = 38;
pub(super) const VS_PSRLD: u8 = 39;
pub(super) const VS_PSRAD: u8 = 40;
pub(super) const VS_PSLLD: u8 = 41;
pub(super) const VS_PSRLQ: u8 = 42;
pub(super) const VS_PSLLQ: u8 = 43;
pub(super) const VS_MOVHLPS: u8 = 44; // a[127:64] : b[127:64]
pub(super) const VS_SHUFPD: u8 = 45; // a[imm[0]], b[imm[1]] (upper lane uses imm >> 2)
pub(super) const VS_VPERMILPS: u8 = 46; // a permuted by b's dword lanes & 3
pub(super) const VS_PALIGNR: u8 = 47; // ((a << 128) | b) >> (imm * 8)
pub(super) const VS_PBLENDW: u8 = 48; // word blend by imm8 (same 8 bits for both 128b lanes)
pub(super) const VS_BLENDPS: u8 = 49; // dword blend by imm8[3:0] (upper lane uses imm8 >> 4)
pub(super) const VS_BLENDPD: u8 = 50; // qword blend by imm8[1:0] (upper lane uses imm8 >> 2)
pub(super) const VS_INSERTPS: u8 = 51; // insertps reg form
pub(super) const VS_INSERTPS_M: u8 = 52; // insertps mem form (count_s = 0)
pub(super) const VS_DDUP: u8 = 53; // movddup (b[63:0] duplicated)
pub(super) const VS_VPERMILPD_I: u8 = 54; // b[imm[0]], b[imm[1]] (upper lane uses imm >> 2)
pub(super) const VS_VPERMILPD: u8 = 55; // a permuted by (b >> 1) & 1 per qword lane

// K_VINT_* sub-ops (all lane-wise on a, b).
pub(super) const VI_PADDB: u8 = 0;
pub(super) const VI_PADDW: u8 = 1;
pub(super) const VI_PADDD: u8 = 2;
pub(super) const VI_PADDQ: u8 = 3;
pub(super) const VI_PSUBB: u8 = 4;
pub(super) const VI_PSUBW: u8 = 5;
pub(super) const VI_PSUBD: u8 = 6;
pub(super) const VI_PSUBQ: u8 = 7;
pub(super) const VI_PADDSB: u8 = 8;
pub(super) const VI_PADDSW: u8 = 9;
pub(super) const VI_PADDUSB: u8 = 10;
pub(super) const VI_PADDUSW: u8 = 11;
pub(super) const VI_PSUBSB: u8 = 12;
pub(super) const VI_PSUBSW: u8 = 13;
pub(super) const VI_PSUBUSB: u8 = 14;
pub(super) const VI_PSUBUSW: u8 = 15;
pub(super) const VI_PMULLW: u8 = 16;
pub(super) const VI_PMULHW: u8 = 17;
pub(super) const VI_PMULHUW: u8 = 18;
pub(super) const VI_PMADDWD: u8 = 19;
pub(super) const VI_PMADDUBSW: u8 = 20;
pub(super) const VI_PMULUDQ: u8 = 21;
pub(super) const VI_PMULLD: u8 = 22;
pub(super) const VI_PMULHRSW: u8 = 23;
pub(super) const VI_PMULDQ: u8 = 24;
pub(super) const VI_PMINUB: u8 = 25;
pub(super) const VI_PMAXUB: u8 = 26;
pub(super) const VI_PMINSW: u8 = 27;
pub(super) const VI_PMAXSW: u8 = 28;
pub(super) const VI_PMINSB: u8 = 29;
pub(super) const VI_PMAXSB: u8 = 30;
pub(super) const VI_PMINUW: u8 = 31;
pub(super) const VI_PMAXUW: u8 = 32;
pub(super) const VI_PMINUD: u8 = 33;
pub(super) const VI_PMAXUD: u8 = 34;
pub(super) const VI_PMINSD: u8 = 35;
pub(super) const VI_PMAXSD: u8 = 36;
pub(super) const VI_PAVGB: u8 = 37;
pub(super) const VI_PAVGW: u8 = 38;
pub(super) const VI_PCMPEQB: u8 = 39;
pub(super) const VI_PCMPEQW: u8 = 40;
pub(super) const VI_PCMPEQD: u8 = 41;
pub(super) const VI_PCMPEQQ: u8 = 42;
pub(super) const VI_PCMPGTB: u8 = 43;
pub(super) const VI_PCMPGTW: u8 = 44;
pub(super) const VI_PCMPGTD: u8 = 45;
pub(super) const VI_PCMPGTQ: u8 = 46;
pub(super) const VI_PABSB: u8 = 47;
pub(super) const VI_PABSW: u8 = 48;
pub(super) const VI_PABSD: u8 = 49;
pub(super) const VI_PSIGNB: u8 = 50;
pub(super) const VI_PSIGNW: u8 = 51;
pub(super) const VI_PSIGND: u8 = 52;
pub(super) const VI_PHADDW: u8 = 53;
pub(super) const VI_PHADDD: u8 = 54;
pub(super) const VI_PHSUBW: u8 = 55;
pub(super) const VI_PHSUBD: u8 = 56;
pub(super) const VI_PSADBW: u8 = 57;
pub(super) const VI_VPSLLVD: u8 = 58;
pub(super) const VI_VPSRLVD: u8 = 59;
pub(super) const VI_VPSRAVD: u8 = 60;
pub(super) const VI_VPSLLVQ: u8 = 61;
pub(super) const VI_VPSRLVQ: u8 = 62;
pub(super) const VI_PCLMULQDQ: u8 = 63;

// K_VFP_* sub-ops. Arithmetic order within each group: add, sub, mul, div, min, max
// (the `op` argument of `sse_arith32/64`). Scalar forms keep a's upper bits.
pub(super) const VF_ADDSS: u8 = 0; // ..= 5 (maxss)
pub(super) const VF_ADDSD: u8 = 6; // ..= 11
pub(super) const VF_ADDPS: u8 = 12; // ..= 17
pub(super) const VF_ADDPD: u8 = 18; // ..= 23
pub(super) const VF_SQRTSS: u8 = 24; // a[127:32] : sqrt(b[31:0])
pub(super) const VF_SQRTSD: u8 = 25;
pub(super) const VF_CVTSS2SD: u8 = 26; // a[127:64] : f64(b[31:0])
pub(super) const VF_CVTSD2SS: u8 = 27; // a[127:32] : f32(b[63:0])
pub(super) const VF_CVTDQ2PS: u8 = 28; // lanes of b
pub(super) const VF_CVTTPS2DQ: u8 = 29;
pub(super) const VF_CVTPS2DQ: u8 = 30;
pub(super) const VF_RCPSS: u8 = 31;
pub(super) const VF_RSQRTSS: u8 = 32;
pub(super) const VF_RCPPS: u8 = 33;
pub(super) const VF_RSQRTPS: u8 = 34;
pub(super) const VF_SQRTPS: u8 = 35;
pub(super) const VF_SQRTPD: u8 = 36;
pub(super) const VF_ADDSUBPS: u8 = 37;
pub(super) const VF_ADDSUBPD: u8 = 38;
pub(super) const VF_HADDPS: u8 = 39;
pub(super) const VF_HSUBPS: u8 = 40;
pub(super) const VF_HADDPD: u8 = 41;
pub(super) const VF_HSUBPD: u8 = 42;
pub(super) const VF_ROUNDSS: u8 = 43;
pub(super) const VF_ROUNDSD: u8 = 44;
pub(super) const VF_ROUNDPS: u8 = 45;
pub(super) const VF_ROUNDPD: u8 = 46;
pub(super) const VF_CMPSS: u8 = 47;
pub(super) const VF_CMPSD: u8 = 48;
pub(super) const VF_CMPPS: u8 = 49;
pub(super) const VF_CMPPD: u8 = 50;
pub(super) const VF_DPPS: u8 = 51;

// ALU sub-ops (`FastOp::x` for K_ALU_*).
const A_ADD: u8 = 0;
const A_SUB: u8 = 1;
const A_AND: u8 = 2;
const A_OR: u8 = 3;
const A_XOR: u8 = 4;
const A_CMP: u8 = 5;
const A_TEST: u8 = 6;
pub(super) const A_ADC: u8 = 7;
pub(super) const A_SBB: u8 = 8;

// Shift sub-ops (`FastOp::x` for K_SHIFT_*).
const S_SHL: u8 = 0;
const S_SHR: u8 = 1;
const S_SAR: u8 = 2;

pub(super) const SHIFT_SHL: u8 = S_SHL;
pub(super) const SHIFT_SHR: u8 = S_SHR;
pub(super) const SHIFT_SAR: u8 = S_SAR;

// Register-width tables indexed by `w >> 4` (8 -> 0, 16 -> 1, 32 -> 2, 64 -> 4).
const W_MASK: [u64; 8] = [0xFF, 0xFFFF, 0xFFFF_FFFF, 0, u64::MAX, 0, 0, 0];
const W_KEEP: [u64; 8] = [!0xFF, !0xFFFF, 0, 0, 0, 0, 0, 0];
const W_LAST: [u8; 8] = [8, 16, 64, 64, 64, 64, 64, 64];

/// One pre-decoded instruction (32 bytes).
#[derive(Clone, Copy)]
#[repr(C)]
pub(super) struct FastOp {
    /// Guest IP of this instruction (0 = empty slot).
    pub ip: u64,
    /// Memory displacement (or absolute address for RIP-relative operands).
    pub disp: i64,
    /// Immediate (already extended/masked as the generic `read_op` would produce),
    /// branch target, or shift count.
    pub imm: u64,
    pub kind: u8,
    /// Instruction length.
    pub len: u8,
    /// Operand width in bits (8/16/32/64).
    pub w: u8,
    pub r0: u8,
    pub r1: u8,
    pub base: u8,
    pub index: u8,
    /// Low 6 bits: ALU/shift sub-op, condition code, or source width (movzx/movsx);
    /// high 2 bits: memory index scale shift.
    pub x: u8,
}

impl Default for FastOp {
    fn default() -> Self {
        Self { ip: 0, disp: 0, imm: 0, kind: K_GENERIC, len: 0, w: 0, r0: 0, r1: 0, base: RNONE, index: RNONE, x: 0 }
    }
}

/// Result of executing a fast op.
pub(super) enum Fx {
    /// Done; `cpu.rip` is already the next instruction or branch target.
    Done,
    /// Taken near conditional branch to `target` (caller applies tail-call dispatch).
    JccNear(u64),
    /// Direct `jmp` to `target` (caller applies the generic `jmp` semantics).
    Jmp(u64),
    /// `call` to `target` (caller applies the generic `call` semantics).
    Call(u64),
    /// Compare done; the caller must now execute the next arena slot (a [`K_JCC`]).
    FuseJcc,
}
