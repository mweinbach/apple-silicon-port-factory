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

use super::{sse_arith32, sse_arith64, sse_cmp_pred, sse_cvt_i32, sse_cvt_i64, sse_sqrt32, sse_sqrt64, x86_nan32, x86_nan64, Cpu, OpKind, Register};
use iced_x86::{EncodingKind, Instruction, Mnemonic};

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

#[inline(always)]
fn gpr_of(r: Register) -> Option<(u8, u8)> {
    let (idx, bits, hi) = Cpu::reg_idx(r)?;
    if hi {
        return None;
    }
    Some((idx as u8, bits as u8))
}

/// Memory operand → `(base, index, scale_shift, disp)`; `None` for forms the fast path
/// doesn't handle (non-GS segment overrides, 16-bit addressing, ...).
fn mem_of(ins: &Instruction) -> Option<(u8, u8, u8, i64)> {
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
fn imm_of(ins: &Instruction, op: u32, w: u32) -> Option<u64> {
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
fn mem_bits(ins: &Instruction) -> u32 {
    ins.memory_size().size() as u32 * 8
}

fn is_w(w: u32) -> bool {
    matches!(w, 8 | 16 | 32 | 64)
}

/// Mark `cmp`/`test` + `jcc` pairs inside one decoded block for fused execution.
pub(super) fn fuse(ops: &mut [FastOp]) {
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
fn xreg(r: Register) -> Option<u8> {
    Cpu::xmm_idx(r).map(|i| i as u8)
}

/// Destination write mode of a vector instruction (`None` for EVEX and other encodings,
/// which stay generic).
fn vmode(ins: &Instruction) -> Option<u8> {
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
fn vmem_code(n: usize) -> Option<u8> {
    match n {
        4 => Some(2),
        8 => Some(3),
        16 => Some(4),
        32 => Some(5),
        _ => None,
    }
}

/// Fill memory addressing fields (as `set_mem` in [`lower_inner`]).
fn put_mem(ins: &Instruction, f: &mut FastOp) -> Option<()> {
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
fn lower_v(ins: &Instruction, f: &mut FastOp, rr: u8, rm: u8, sub: u8, y256: bool, mem_bytes: Option<usize>) -> Option<()> {
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
        M::Clc | M::Stc | M::Cmc | M::Cld | M::Std | M::Mfence | M::Sfence | M::Lfence => {
            f.kind = K_FLAG_OP;
            f.x = match mn {
                M::Clc => 0,
                M::Stc => 1,
                M::Cmc => 2,
                M::Cld => 3,
                M::Std => 4,
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
pub(super) fn lower(ins: &Instruction) -> FastOp {
    let mut f = FastOp { ip: ins.ip(), len: ins.len() as u8, ..FastOp::default() };
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
        Mnemonic::Nop | Mnemonic::Pause | Mnemonic::Fnop | Mnemonic::Endbr64 | Mnemonic::Endbr32
        | Mnemonic::Clflush | Mnemonic::Prefetch | Mnemonic::Prefetcht0 | Mnemonic::Prefetcht1
        | Mnemonic::Prefetcht2 | Mnemonic::Prefetchnta | Mnemonic::Prefetchw | Mnemonic::Prefetchwt1
        | Mnemonic::Wait | Mnemonic::Rdsspd | Mnemonic::Rdsspq | Mnemonic::Emms | Mnemonic::Femms => {
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

impl Cpu {
    /// Same as `get_reg` for a non-high-byte GPR.
    #[inline(always)]
    fn rget(&self, i: u8, w: u8) -> u64 {
        self.gpr[i as usize & 15] & W_MASK[(w >> 4) as usize & 7]
    }

    /// Same as `set_reg` for a non-high-byte GPR (incl. `last_bits`): 8/16-bit writes merge,
    /// 32-bit writes zero-extend.
    #[inline(always)]
    fn rset(&mut self, i: u8, w: u8, val: u64) {
        let i = i as usize & 15;
        let k = (w >> 4) as usize & 7;
        self.gpr[i] = (self.gpr[i] & W_KEEP[k]) | (val & W_MASK[k]);
        self.last_bits[i] = W_LAST[k];
    }

    #[cold]
    #[inline(never)]
    fn faddr_slow(&self, op: &FastOp, idx: u64) -> u64 {
        let b = op.base;
        if b & B_ADDR32 == 0 {
            let mut a = (op.disp as u64).wrapping_add(idx);
            let reg = b & 0x1F;
            if reg < 16 {
                a = a.wrapping_add(self.gpr[reg as usize]);
            }
            if b & B_GS != 0 {
                let t = if self.jx.teb != 0 {
                    self.jx.teb
                } else {
                    unsafe { crate::exe::teb_self() as u64 }
                };
                a = a.wrapping_add(t);
            }
            a
        } else {
            let reg = b & 0x1F;
            let base = if reg < 16 { self.gpr[reg as usize] } else { 0 };
            let mut a = ((op.disp as u64).wrapping_add(base).wrapping_add(idx) as u32) as u64;
            if b & B_GS != 0 {
                let t = if self.jx.teb != 0 {
                    self.jx.teb
                } else {
                    unsafe { crate::exe::teb_self() as u64 }
                };
                a = a.wrapping_add(t);
            }
            a
        }
    }

    #[inline(always)]
    fn faddr(&self, op: &FastOp) -> u64 {
        let idx = if op.index < 16 {
            self.gpr[op.index as usize] << (op.x >> 6)
        } else {
            0
        };
        let b = op.base;
        if b < 16 {
            (op.disp as u64).wrapping_add(self.gpr[b as usize]).wrapping_add(idx)
        } else if b == RNONE {
            (op.disp as u64).wrapping_add(idx)
        } else {
            self.faddr_slow(op, idx)
        }
    }

    /// Direct `cmp`/`test` flag update for fused `K_CMPJ_*` ops (avoids general 9-way `alu` match).
    #[inline(always)]
    fn cmp_or_test(&mut self, sub: u8, a: u64, b: u64, w: u32) {
        if sub == A_CMP {
            let _ = self.update_flags_sub(a, b, w);
        } else {
            self.update_flags_logic(a & b, w);
        }
    }

    #[inline(always)]
    fn eval_cc_u8(&self, cc: u8) -> bool {
        match cc & 0x3F {
            1 => self.of,
            2 => !self.of,
            3 => self.cf,
            4 => !self.cf,
            5 => self.zf,
            6 => !self.zf,
            7 => self.cf || self.zf,
            8 => !self.cf && !self.zf,
            9 => self.sf,
            10 => !self.sf,
            11 => self.pf(),
            12 => !self.pf(),
            13 => self.sf != self.of,
            14 => self.sf == self.of,
            15 => self.zf || (self.sf != self.of),
            16 => !self.zf && (self.sf == self.of),
            _ => false,
        }
    }

    /// ALU op with the generic handlers' flag semantics; returns the value to write
    /// back (`None` for `cmp`/`test`).
    #[inline(always)]
    fn alu(&mut self, sub: u8, a: u64, b: u64, w: u32) -> Option<u64> {
        match sub {
            A_ADD => Some(self.update_flags_add(a, b, w)),
            A_SUB => Some(self.update_flags_sub(a, b, w)),
            A_AND => {
                let r = a & b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_OR => {
                let r = a | b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_XOR => {
                let r = a ^ b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_CMP => {
                let _ = self.update_flags_sub(a, b, w);
                None
            }
            A_ADC => {
                let mask = if w >= 64 { u64::MAX } else { (1u64 << w) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let full = (ua as u128) + (ub as u128) + (self.cf as u128);
                let res = (full as u64) & mask;
                let sign_bit = 1u64 << (w - 1);
                self.cf = full > mask as u128;
                self.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
                self.zf = res == 0;
                self.sf = (res & sign_bit) != 0;
                self.pfv = res as u8;
                Some(res)
            }
            A_SBB => {
                let mask = if w >= 64 { u64::MAX } else { (1u64 << w) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let c = self.cf as u64;
                let res = ua.wrapping_sub(ub).wrapping_sub(c) & mask;
                let sign_bit = 1u64 << (w - 1);
                self.cf = (ua as u128) < (ub as u128) + (c as u128);
                self.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
                self.zf = res == 0;
                self.sf = (res & sign_bit) != 0;
                self.pfv = res as u8;
                Some(res)
            }
            _ => {
                self.update_flags_logic(a & b, w);
                None
            }
        }
    }

    /// x86 SHL/SHR/SAR (`cnt` already masked and non-zero) with full flags: CF = last bit
    /// shifted out, OF = defined count-1 value, SF/ZF/PF from the result. Shared with the
    /// generic handler.
    #[inline(always)]
    pub(super) fn shift(&mut self, sub: u8, a: u64, cnt: u32, w: u8) -> u64 {
        let wb = w as u32;
        let vm = if wb >= 64 { u64::MAX } else { (1u64 << wb) - 1 };
        let ua = a & vm;
        let msb = 1u64 << (wb - 1);
        let (r, cf, of) = match sub {
            S_SHL => {
                let r = ua.wrapping_shl(cnt) & vm;
                let cf = cnt <= wb && (ua >> (wb - cnt)) & 1 != 0;
                (r, cf, ((r & msb) != 0) ^ cf)
            }
            S_SHR => (ua >> cnt, (ua >> (cnt - 1)) & 1 != 0, ua & msb != 0),
            _ => {
                let sa = sext(ua, w) as i64;
                let r = (sa >> cnt.min(63)) as u64 & vm;
                (r, (sa >> (cnt - 1).min(63)) & 1 != 0, false)
            }
        };
        self.cf = cf;
        self.of = of;
        self.zf = r == 0;
        self.sf = r & msb != 0;
        self.pfv = r as u8;
        r
    }
}

pub(super) const SHIFT_SHL: u8 = S_SHL;
pub(super) const SHIFT_SHR: u8 = S_SHR;
pub(super) const SHIFT_SAR: u8 = S_SAR;

#[inline(always)]
fn sext(v: u64, sw: u8) -> u64 {
    match sw {
        8 => v as i8 as i64 as u64,
        16 => v as i16 as i64 as u64,
        32 => v as i32 as i64 as u64,
        _ => v,
    }
}

// ---- vector lane helpers (little-endian hosts: lane i of a u128 is array element i) ----------

#[cfg(not(target_endian = "little"))]
compile_error!("emu_fast vector lanes assume a little-endian host");

/// Reinterpret a 128-bit value as lanes.
#[inline(always)]
fn ln<T: Copy, const N: usize>(v: u128) -> [T; N] {
    debug_assert_eq!(core::mem::size_of::<[T; N]>(), 16);
    unsafe { core::mem::transmute_copy(&v) }
}

/// Inverse of [`ln`].
#[inline(always)]
fn pk<T: Copy, const N: usize>(a: [T; N]) -> u128 {
    debug_assert_eq!(core::mem::size_of::<[T; N]>(), 16);
    unsafe { core::mem::transmute_copy(&a) }
}

/// Lane-wise binary op.
#[inline(always)]
fn map2<T: Copy, const N: usize>(a: u128, b: u128, f: impl Fn(T, T) -> T) -> u128 {
    let (x, y): ([T; N], [T; N]) = (ln(a), ln(b));
    pk::<T, N>(core::array::from_fn(|i| f(x[i], y[i])))
}

/// Lane-wise unary op.
#[inline(always)]
fn map1<T: Copy, const N: usize>(a: u128, f: impl Fn(T) -> T) -> u128 {
    let x: [T; N] = ln(a);
    pk::<T, N>(core::array::from_fn(|i| f(x[i])))
}

/// Reinterpret a 128-bit value as a NEON `uint8x16_t` (zero-cost register move on ARM64).
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn u128_to_u8x16(v: u128) -> core::arch::aarch64::uint8x16_t {
    unsafe { core::mem::transmute(v) }
}

/// Extract the 16 MSBs of 16 bytes into a 16-bit mask (`pmovmskb` equivalent) using NEON.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn neon_movemask_u8(v: u128) -> u32 {
    use core::arch::aarch64::*;
    unsafe {
        let sign = vreinterpretq_u8_s8(vshrq_n_s8::<7>(vreinterpretq_s8_u8(u128_to_u8x16(v))));
        const MASK: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let bits = vandq_u8(sign, vld1q_u8(MASK.as_ptr()));
        let p1 = vpaddq_u8(bits, bits);
        let p2 = vpaddq_u8(p1, p1);
        let p3 = vpaddq_u8(p2, p2);
        vgetq_lane_u16::<0>(vreinterpretq_u16_u8(p3)) as u32
    }
}

/// Interleave the low (`hi == false`) or high halves of the lanes of `a` and `b`.
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
fn unpack<T: Copy + Default, const N: usize>(a: u128, b: u128, hi: bool) -> u128 {
    let (x, y): ([T; N], [T; N]) = (ln(a), ln(b));
    let o = if hi { N / 2 } else { 0 };
    pk::<T, N>(core::array::from_fn(|i| if i & 1 == 0 { x[o + i / 2] } else { y[o + i / 2] }))
}

/// `K_VSHUF_*` sub-ops. Mirrors the generic handlers (shift-count clamping, byte shifts, ...).
#[inline(always)]
fn vshuf(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    let ic = imm as u64;
    let bc = b as u64;
    match sub {
        VS_MOV => b,
        VS_ZX32 => b & 0xFFFF_FFFF,
        VS_ZX64 => b & 0xFFFF_FFFF_FFFF_FFFF,
        VS_MRG32 => (a & !0xFFFF_FFFFu128) | (b & 0xFFFF_FFFF),
        VS_MRG64 => (a & !0xFFFF_FFFF_FFFF_FFFFu128) | (b & 0xFFFF_FFFF_FFFF_FFFF),
        VS_XOR => a ^ b,
        VS_OR => a | b,
        VS_AND => a & b,
        VS_ANDN => !a & b,
        VS_UNPCKLBW => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                core::mem::transmute(core::arch::aarch64::vzip1q_u8(u128_to_u8x16(a), u128_to_u8x16(b)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u8, 16>(a, b, false)
        }
        VS_UNPCKLWD => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                core::mem::transmute(vzip1q_u16(vreinterpretq_u16_u8(u128_to_u8x16(a)), vreinterpretq_u16_u8(u128_to_u8x16(b))))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u16, 8>(a, b, false)
        }
        VS_UNPCKLDQ => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                core::mem::transmute(vzip1q_u32(vreinterpretq_u32_u8(u128_to_u8x16(a)), vreinterpretq_u32_u8(u128_to_u8x16(b))))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u32, 4>(a, b, false)
        }
        VS_UNPCKLQDQ => (a as u64 as u128) | ((b as u64 as u128) << 64),
        VS_UNPCKHBW => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                core::mem::transmute(core::arch::aarch64::vzip2q_u8(u128_to_u8x16(a), u128_to_u8x16(b)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u8, 16>(a, b, true)
        }
        VS_UNPCKHWD => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                core::mem::transmute(vzip2q_u16(vreinterpretq_u16_u8(u128_to_u8x16(a)), vreinterpretq_u16_u8(u128_to_u8x16(b))))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u16, 8>(a, b, true)
        }
        VS_UNPCKHDQ => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                core::mem::transmute(vzip2q_u32(vreinterpretq_u32_u8(u128_to_u8x16(a)), vreinterpretq_u32_u8(u128_to_u8x16(b))))
            }
            #[cfg(not(target_arch = "aarch64"))]
            unpack::<u32, 4>(a, b, true)
        }
        VS_UNPCKHQDQ => (a >> 64) | ((b >> 64) << 64),
        VS_PACKSSWB => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                let va = vreinterpretq_s16_u8(u128_to_u8x16(a));
                let vb = vreinterpretq_s16_u8(u128_to_u8x16(b));
                core::mem::transmute(vcombine_s8(vqmovn_s16(va), vqmovn_s16(vb)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                let (x, y): ([i16; 8], [i16; 8]) = (ln(a), ln(b));
                pk::<i8, 16>(core::array::from_fn(|i| if i < 8 { x[i] } else { y[i - 8] }.clamp(-128, 127) as i8))
            }
        }
        VS_PACKUSWB => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                let va = vreinterpretq_s16_u8(u128_to_u8x16(a));
                let vb = vreinterpretq_s16_u8(u128_to_u8x16(b));
                core::mem::transmute(vcombine_u8(vqmovun_s16(va), vqmovun_s16(vb)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                let (x, y): ([i16; 8], [i16; 8]) = (ln(a), ln(b));
                pk::<u8, 16>(core::array::from_fn(|i| if i < 8 { x[i] } else { y[i - 8] }.clamp(0, 255) as u8))
            }
        }
        VS_PACKSSDW => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                let va = vreinterpretq_s32_u8(u128_to_u8x16(a));
                let vb = vreinterpretq_s32_u8(u128_to_u8x16(b));
                core::mem::transmute(vcombine_s16(vqmovn_s32(va), vqmovn_s32(vb)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                let (x, y): ([i32; 4], [i32; 4]) = (ln(a), ln(b));
                pk::<i16, 8>(core::array::from_fn(|i| if i < 4 { x[i] } else { y[i - 4] }.clamp(-32768, 32767) as i16))
            }
        }
        VS_PACKUSDW => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                let va = vreinterpretq_s32_u8(u128_to_u8x16(a));
                let vb = vreinterpretq_s32_u8(u128_to_u8x16(b));
                core::mem::transmute(vcombine_u16(vqmovun_s32(va), vqmovun_s32(vb)))
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                let (x, y): ([i32; 4], [i32; 4]) = (ln(a), ln(b));
                pk::<u16, 8>(core::array::from_fn(|i| if i < 4 { x[i] } else { y[i - 4] }.clamp(0, 65535) as u16))
            }
        }
        VS_PSHUFD => {
            let x: [u32; 4] = ln(b);
            let m = imm as usize;
            pk::<u32, 4>([x[m & 3], x[(m >> 2) & 3], x[(m >> 4) & 3], x[(m >> 6) & 3]])
        }
        VS_PSHUFLW => {
            let x: [u16; 8] = ln(b);
            let m = imm as usize;
            pk::<u16, 8>([x[m & 3], x[(m >> 2) & 3], x[(m >> 4) & 3], x[(m >> 6) & 3], x[4], x[5], x[6], x[7]])
        }
        VS_PSHUFHW => {
            let x: [u16; 8] = ln(b);
            let m = imm as usize;
            pk::<u16, 8>([
                x[0],
                x[1],
                x[2],
                x[3],
                x[4 + (m & 3)],
                x[4 + ((m >> 2) & 3)],
                x[4 + ((m >> 4) & 3)],
                x[4 + ((m >> 6) & 3)],
            ])
        }
        VS_SHUFPS => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            let m = imm as usize;
            pk::<u32, 4>([x[m & 3], x[(m >> 2) & 3], y[(m >> 4) & 3], y[(m >> 6) & 3]])
        }
        VS_PSHUFB => {
            #[cfg(target_arch = "aarch64")]
            unsafe {
                use core::arch::aarch64::*;
                // x86 ignores bits 6..4 of each control byte and only zeroes when bit 7 is set;
                // ARM64 `vqtbl1q_u8` zeroes for any index >= 16, so mask with `0x8F`.
                let idx = vandq_u8(u128_to_u8x16(b), vdupq_n_u8(0x8F));
                core::mem::transmute(vqtbl1q_u8(u128_to_u8x16(a), idx))
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                let (x, m): ([u8; 16], [u8; 16]) = (ln(a), ln(b));
                pk::<u8, 16>(core::array::from_fn(|i| if m[i] & 0x80 != 0 { 0 } else { x[(m[i] & 15) as usize] }))
            }
        }
        VS_PSRLW_I => sh_w(b, ic, 0),
        VS_PSRAW_I => sh_w(b, ic, 1),
        VS_PSLLW_I => sh_w(b, ic, 2),
        VS_PSRLD_I => sh_d(b, ic, 0),
        VS_PSRAD_I => sh_d(b, ic, 1),
        VS_PSLLD_I => sh_d(b, ic, 2),
        VS_PSRLQ_I => sh_q(b, ic, false),
        VS_PSLLQ_I => sh_q(b, ic, true),
        VS_PSRLDQ_I | VS_PSLLDQ_I => {
            let n = ic.min(16) as u32;
            if n >= 16 {
                0
            } else if sub == VS_PSRLDQ_I {
                b >> (n * 8)
            } else {
                b << (n * 8)
            }
        }
        VS_PSRLW => sh_w(a, bc, 0),
        VS_PSRAW => sh_w(a, bc, 1),
        VS_PSLLW => sh_w(a, bc, 2),
        VS_PSRLD => sh_d(a, bc, 0),
        VS_PSRAD => sh_d(a, bc, 1),
        VS_PSLLD => sh_d(a, bc, 2),
        VS_PSRLQ => sh_q(a, bc, false),
        VS_PSLLQ => sh_q(a, bc, true),
        VS_MOVHLPS => (a & !0xFFFF_FFFF_FFFF_FFFFu128) | (b >> 64),
        VS_SHUFPD => {
            let (x, y): ([u64; 2], [u64; 2]) = (ln(a), ln(b));
            pk::<u64, 2>([x[(imm & 1) as usize], y[((imm >> 1) & 1) as usize]])
        }
        VS_VPERMILPS => {
            let (x, c): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            pk::<u32, 4>([
                x[(c[0] & 3) as usize],
                x[(c[1] & 3) as usize],
                x[(c[2] & 3) as usize],
                x[(c[3] & 3) as usize],
            ])
        }
        VS_PALIGNR => {
            if ic >= 32 {
                0
            } else if ic >= 16 {
                a >> ((ic - 16) * 8)
            } else if ic == 0 {
                b
            } else {
                (b >> (ic * 8)) | (a << ((16 - ic) * 8))
            }
        }
        VS_PBLENDW => {
            let (x, y): ([u16; 8], [u16; 8]) = (ln(a), ln(b));
            pk::<u16, 8>(core::array::from_fn(|i| if (imm >> i) & 1 != 0 { y[i] } else { x[i] }))
        }
        VS_BLENDPS => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            pk::<u32, 4>([
                if imm & 1 != 0 { y[0] } else { x[0] },
                if imm & 2 != 0 { y[1] } else { x[1] },
                if imm & 4 != 0 { y[2] } else { x[2] },
                if imm & 8 != 0 { y[3] } else { x[3] },
            ])
        }
        VS_BLENDPD => {
            let (x, y): ([u64; 2], [u64; 2]) = (ln(a), ln(b));
            pk::<u64, 2>([if imm & 1 != 0 { y[0] } else { x[0] }, if imm & 2 != 0 { y[1] } else { x[1] }])
        }
        VS_INSERTPS | VS_INSERTPS_M => {
            let count_s = if sub == VS_INSERTPS_M { 0 } else { ((imm >> 6) & 3) as usize };
            let count_d = ((imm >> 4) & 3) as usize;
            let zmask = imm & 0xF;
            let (mut x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            x[count_d] = y[count_s];
            for i in 0..4 {
                if (zmask >> i) & 1 != 0 {
                    x[i] = 0;
                }
            }
            pk::<u32, 4>(x)
        }
        VS_DDUP => {
            let q = (b as u64) as u128;
            q | (q << 64)
        }
        _ => b,
    }
}

/// Word shifts: `k` 0 = logical right, 1 = arithmetic right, 2 = left.
#[inline(always)]
fn sh_w(v: u128, cnt: u64, k: u8) -> u128 {
    match k {
        1 => {
            let s = cnt.min(15) as u32;
            map1::<i16, 8>(v, |x| x >> s)
        }
        _ if cnt >= 16 => 0,
        0 => map1::<u16, 8>(v, |x| x >> cnt as u32),
        _ => map1::<u16, 8>(v, |x| x << cnt as u32),
    }
}

/// Dword shifts (see [`sh_w`]).
#[inline(always)]
fn sh_d(v: u128, cnt: u64, k: u8) -> u128 {
    match k {
        1 => {
            let s = cnt.min(31) as u32;
            map1::<i32, 4>(v, |x| x >> s)
        }
        _ if cnt >= 32 => 0,
        0 => map1::<u32, 4>(v, |x| x >> cnt as u32),
        _ => map1::<u32, 4>(v, |x| x << cnt as u32),
    }
}

/// Qword logical shifts.
#[inline(always)]
fn sh_q(v: u128, cnt: u64, left: bool) -> u128 {
    if cnt >= 64 {
        0
    } else if left {
        map1::<u64, 2>(v, |x| x << cnt as u32)
    } else {
        map1::<u64, 2>(v, |x| x >> cnt as u32)
    }
}

/// `K_VINT_*` sub-ops (NEON-accelerated on `aarch64`, exact portable fallback elsewhere).
#[inline(always)]
fn vint(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::*;
        unsafe {
            let ua8 = u128_to_u8x16(a);
            let ub8 = u128_to_u8x16(b);
            match sub {
                VI_PADDB => return core::mem::transmute(vaddq_u8(ua8, ub8)),
                VI_PADDW => return core::mem::transmute(vaddq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PADDD => return core::mem::transmute(vaddq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PADDQ => return core::mem::transmute(vaddq_u64(vreinterpretq_u64_u8(ua8), vreinterpretq_u64_u8(ub8))),
                VI_PSUBB => return core::mem::transmute(vsubq_u8(ua8, ub8)),
                VI_PSUBW => return core::mem::transmute(vsubq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PSUBD => return core::mem::transmute(vsubq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PSUBQ => return core::mem::transmute(vsubq_u64(vreinterpretq_u64_u8(ua8), vreinterpretq_u64_u8(ub8))),
                VI_PADDSB => return core::mem::transmute(vqaddq_s8(vreinterpretq_s8_u8(ua8), vreinterpretq_s8_u8(ub8))),
                VI_PADDSW => return core::mem::transmute(vqaddq_s16(vreinterpretq_s16_u8(ua8), vreinterpretq_s16_u8(ub8))),
                VI_PADDUSB => return core::mem::transmute(vqaddq_u8(ua8, ub8)),
                VI_PADDUSW => return core::mem::transmute(vqaddq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PSUBSB => return core::mem::transmute(vqsubq_s8(vreinterpretq_s8_u8(ua8), vreinterpretq_s8_u8(ub8))),
                VI_PSUBSW => return core::mem::transmute(vqsubq_s16(vreinterpretq_s16_u8(ua8), vreinterpretq_s16_u8(ub8))),
                VI_PSUBUSB => return core::mem::transmute(vqsubq_u8(ua8, ub8)),
                VI_PSUBUSW => return core::mem::transmute(vqsubq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PMULLW => return core::mem::transmute(vmulq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PMULLD => return core::mem::transmute(vmulq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PMULHW => {
                    let va = vreinterpretq_s16_u8(ua8);
                    let vb = vreinterpretq_s16_u8(ub8);
                    let lo = vshrn_n_s32::<16>(vmull_s16(vget_low_s16(va), vget_low_s16(vb)));
                    let hi = vshrn_n_s32::<16>(vmull_high_s16(va, vb));
                    return core::mem::transmute(vcombine_s16(lo, hi));
                }
                VI_PMULHUW => {
                    let va = vreinterpretq_u16_u8(ua8);
                    let vb = vreinterpretq_u16_u8(ub8);
                    let lo = vshrn_n_u32::<16>(vmull_u16(vget_low_u16(va), vget_low_u16(vb)));
                    let hi = vshrn_n_u32::<16>(vmull_high_u16(va, vb));
                    return core::mem::transmute(vcombine_u16(lo, hi));
                }
                VI_PMULHRSW => {
                    // Widening signed multiply + non-saturating rounding shift right by 15
                    // matches x86 bit-exactly even for `i16::MIN * i16::MIN` (`0x8000`).
                    let va = vreinterpretq_s16_u8(ua8);
                    let vb = vreinterpretq_s16_u8(ub8);
                    let lo = vrshrn_n_s32::<15>(vmull_s16(vget_low_s16(va), vget_low_s16(vb)));
                    let hi = vrshrn_n_s32::<15>(vmull_high_s16(va, vb));
                    return core::mem::transmute(vcombine_s16(lo, hi));
                }
                VI_PMULUDQ => {
                    let va = vmovn_u64(vreinterpretq_u64_u8(ua8));
                    let vb = vmovn_u64(vreinterpretq_u64_u8(ub8));
                    return core::mem::transmute(vmull_u32(va, vb));
                }
                VI_PMULDQ => {
                    let va = vmovn_s64(vreinterpretq_s64_u8(ua8));
                    let vb = vmovn_s64(vreinterpretq_s64_u8(ub8));
                    return core::mem::transmute(vmull_s32(va, vb));
                }
                VI_PMADDWD => {
                    let va = vreinterpretq_s16_u8(ua8);
                    let vb = vreinterpretq_s16_u8(ub8);
                    let lo = vmull_s16(vget_low_s16(va), vget_low_s16(vb));
                    let hi = vmull_high_s16(va, vb);
                    return core::mem::transmute(vpaddq_s32(lo, hi));
                }
                VI_PMINUB => return core::mem::transmute(vminq_u8(ua8, ub8)),
                VI_PMAXUB => return core::mem::transmute(vmaxq_u8(ua8, ub8)),
                VI_PMINSW => return core::mem::transmute(vminq_s16(vreinterpretq_s16_u8(ua8), vreinterpretq_s16_u8(ub8))),
                VI_PMAXSW => return core::mem::transmute(vmaxq_s16(vreinterpretq_s16_u8(ua8), vreinterpretq_s16_u8(ub8))),
                VI_PMINSB => return core::mem::transmute(vminq_s8(vreinterpretq_s8_u8(ua8), vreinterpretq_s8_u8(ub8))),
                VI_PMAXSB => return core::mem::transmute(vmaxq_s8(vreinterpretq_s8_u8(ua8), vreinterpretq_s8_u8(ub8))),
                VI_PMINUW => return core::mem::transmute(vminq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PMAXUW => return core::mem::transmute(vmaxq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PMINUD => return core::mem::transmute(vminq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PMAXUD => return core::mem::transmute(vmaxq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PMINSD => return core::mem::transmute(vminq_s32(vreinterpretq_s32_u8(ua8), vreinterpretq_s32_u8(ub8))),
                VI_PMAXSD => return core::mem::transmute(vmaxq_s32(vreinterpretq_s32_u8(ua8), vreinterpretq_s32_u8(ub8))),
                VI_PAVGB => return core::mem::transmute(vrhaddq_u8(ua8, ub8)),
                VI_PAVGW => return core::mem::transmute(vrhaddq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PCMPEQB => return core::mem::transmute(vceqq_u8(ua8, ub8)),
                VI_PCMPEQW => return core::mem::transmute(vceqq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PCMPEQD => return core::mem::transmute(vceqq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PCMPEQQ => return core::mem::transmute(vceqq_u64(vreinterpretq_u64_u8(ua8), vreinterpretq_u64_u8(ub8))),
                VI_PCMPGTB => return core::mem::transmute(vcgtq_s8(vreinterpretq_s8_u8(ua8), vreinterpretq_s8_u8(ub8))),
                VI_PCMPGTW => return core::mem::transmute(vcgtq_s16(vreinterpretq_s16_u8(ua8), vreinterpretq_s16_u8(ub8))),
                VI_PCMPGTD => return core::mem::transmute(vcgtq_s32(vreinterpretq_s32_u8(ua8), vreinterpretq_s32_u8(ub8))),
                VI_PCMPGTQ => return core::mem::transmute(vcgtq_s64(vreinterpretq_s64_u8(ua8), vreinterpretq_s64_u8(ub8))),
                // Wrapping `vabsq_s*` (`ABS`, not `SQABS`) preserves `MIN -> MIN`, matching x86 `pabs*`.
                VI_PABSB => return core::mem::transmute(vabsq_s8(vreinterpretq_s8_u8(ub8))),
                VI_PABSW => return core::mem::transmute(vabsq_s16(vreinterpretq_s16_u8(ub8))),
                VI_PABSD => return core::mem::transmute(vabsq_s32(vreinterpretq_s32_u8(ub8))),
                VI_PHADDW => return core::mem::transmute(vpaddq_u16(vreinterpretq_u16_u8(ua8), vreinterpretq_u16_u8(ub8))),
                VI_PHADDD => return core::mem::transmute(vpaddq_u32(vreinterpretq_u32_u8(ua8), vreinterpretq_u32_u8(ub8))),
                VI_PHSUBW => {
                    let va = vreinterpretq_u16_u8(ua8);
                    let vb = vreinterpretq_u16_u8(ub8);
                    return core::mem::transmute(vsubq_u16(vuzp1q_u16(va, vb), vuzp2q_u16(va, vb)));
                }
                VI_PHSUBD => {
                    let va = vreinterpretq_u32_u8(ua8);
                    let vb = vreinterpretq_u32_u8(ub8);
                    return core::mem::transmute(vsubq_u32(vuzp1q_u32(va, vb), vuzp2q_u32(va, vb)));
                }
                VI_PSADBW => {
                    let d = vabdq_u8(ua8, ub8);
                    let s16 = vpaddlq_u8(d);
                    let s32 = vpaddlq_u16(s16);
                    return core::mem::transmute(vpaddlq_u32(s32));
                }
                _ => {}
            }
        }
    }
    let mask = |c: bool| if c { !0 } else { 0 };
    match sub {
        VI_PADDB => map2::<u8, 16>(a, b, |x, y| x.wrapping_add(y)),
        VI_PADDW => map2::<u16, 8>(a, b, |x, y| x.wrapping_add(y)),
        VI_PADDD => map2::<u32, 4>(a, b, |x, y| x.wrapping_add(y)),
        VI_PADDQ => map2::<u64, 2>(a, b, |x, y| x.wrapping_add(y)),
        VI_PSUBB => map2::<u8, 16>(a, b, |x, y| x.wrapping_sub(y)),
        VI_PSUBW => map2::<u16, 8>(a, b, |x, y| x.wrapping_sub(y)),
        VI_PSUBD => map2::<u32, 4>(a, b, |x, y| x.wrapping_sub(y)),
        VI_PSUBQ => map2::<u64, 2>(a, b, |x, y| x.wrapping_sub(y)),
        VI_PADDSB => map2::<i8, 16>(a, b, |x, y| x.saturating_add(y)),
        VI_PADDSW => map2::<i16, 8>(a, b, |x, y| x.saturating_add(y)),
        VI_PADDUSB => map2::<u8, 16>(a, b, |x, y| x.saturating_add(y)),
        VI_PADDUSW => map2::<u16, 8>(a, b, |x, y| x.saturating_add(y)),
        VI_PSUBSB => map2::<i8, 16>(a, b, |x, y| x.saturating_sub(y)),
        VI_PSUBSW => map2::<i16, 8>(a, b, |x, y| x.saturating_sub(y)),
        VI_PSUBUSB => map2::<u8, 16>(a, b, |x, y| x.saturating_sub(y)),
        VI_PSUBUSW => map2::<u16, 8>(a, b, |x, y| x.saturating_sub(y)),
        VI_PMULLW => map2::<u16, 8>(a, b, |x, y| x.wrapping_mul(y)),
        VI_PMULHW => map2::<i16, 8>(a, b, |x, y| ((x as i32 * y as i32) >> 16) as i16),
        VI_PMULHUW => map2::<u16, 8>(a, b, |x, y| ((x as u32 * y as u32) >> 16) as u16),
        VI_PMULHRSW => map2::<i16, 8>(a, b, |x, y| ((((x as i32 * y as i32) >> 14) + 1) >> 1) as i16),
        VI_PMULLD => map2::<u32, 4>(a, b, |x, y| x.wrapping_mul(y)),
        VI_PMULUDQ => map2::<u64, 2>(a, b, |x, y| (x as u32 as u64).wrapping_mul(y as u32 as u64)),
        VI_PMULDQ => map2::<u64, 2>(a, b, |x, y| (x as u32 as i32 as i64).wrapping_mul(y as u32 as i32 as i64) as u64),
        VI_PMADDWD => {
            let (x, y): ([i16; 8], [i16; 8]) = (ln(a), ln(b));
            pk::<i32, 4>(core::array::from_fn(|i| {
                (x[2 * i] as i32 * y[2 * i] as i32).wrapping_add(x[2 * i + 1] as i32 * y[2 * i + 1] as i32)
            }))
        }
        VI_PMADDUBSW => {
            let (x, y): ([u8; 16], [i8; 16]) = (ln(a), ln(b));
            pk::<i16, 8>(core::array::from_fn(|i| {
                let s = x[2 * i] as i32 * y[2 * i] as i32 + x[2 * i + 1] as i32 * y[2 * i + 1] as i32;
                s.clamp(i16::MIN as i32, i16::MAX as i32) as i16
            }))
        }
        VI_PMINUB => map2::<u8, 16>(a, b, |x, y| x.min(y)),
        VI_PMAXUB => map2::<u8, 16>(a, b, |x, y| x.max(y)),
        VI_PMINSW => map2::<i16, 8>(a, b, |x, y| x.min(y)),
        VI_PMAXSW => map2::<i16, 8>(a, b, |x, y| x.max(y)),
        VI_PMINSB => map2::<i8, 16>(a, b, |x, y| x.min(y)),
        VI_PMAXSB => map2::<i8, 16>(a, b, |x, y| x.max(y)),
        VI_PMINUW => map2::<u16, 8>(a, b, |x, y| x.min(y)),
        VI_PMAXUW => map2::<u16, 8>(a, b, |x, y| x.max(y)),
        VI_PMINUD => map2::<u32, 4>(a, b, |x, y| x.min(y)),
        VI_PMAXUD => map2::<u32, 4>(a, b, |x, y| x.max(y)),
        VI_PMINSD => map2::<i32, 4>(a, b, |x, y| x.min(y)),
        VI_PMAXSD => map2::<i32, 4>(a, b, |x, y| x.max(y)),
        VI_PAVGB => map2::<u8, 16>(a, b, |x, y| ((x as u16 + y as u16 + 1) >> 1) as u8),
        VI_PAVGW => map2::<u16, 8>(a, b, |x, y| ((x as u32 + y as u32 + 1) >> 1) as u16),
        VI_PCMPEQB => map2::<u8, 16>(a, b, |x, y| mask(x == y) as u8),
        VI_PCMPEQW => map2::<u16, 8>(a, b, |x, y| mask(x == y) as u16),
        VI_PCMPEQD => map2::<u32, 4>(a, b, |x, y| mask(x == y) as u32),
        VI_PCMPEQQ => map2::<u64, 2>(a, b, |x, y| mask(x == y)),
        VI_PCMPGTB => map2::<i8, 16>(a, b, |x, y| mask(x > y) as i8),
        VI_PCMPGTW => map2::<i16, 8>(a, b, |x, y| mask(x > y) as i16),
        VI_PCMPGTD => map2::<i32, 4>(a, b, |x, y| mask(x > y) as i32),
        VI_PCMPGTQ => map2::<i64, 2>(a, b, |x, y| mask(x > y) as i64),
        VI_PABSB => map1::<i8, 16>(b, |x| x.wrapping_abs()),
        VI_PABSW => map1::<i16, 8>(b, |x| x.wrapping_abs()),
        VI_PABSD => map1::<i32, 4>(b, |x| x.wrapping_abs()),
        VI_PSIGNB => map2::<i8, 16>(a, b, |x, y| if y < 0 { x.wrapping_neg() } else if y == 0 { 0 } else { x }),
        VI_PSIGNW => map2::<i16, 8>(a, b, |x, y| if y < 0 { x.wrapping_neg() } else if y == 0 { 0 } else { x }),
        VI_PSIGND => map2::<i32, 4>(a, b, |x, y| if y < 0 { x.wrapping_neg() } else if y == 0 { 0 } else { x }),
        VI_PHADDW => {
            let (x, y): ([u16; 8], [u16; 8]) = (ln(a), ln(b));
            pk::<u16, 8>(core::array::from_fn(|i| if i < 4 { x[2 * i].wrapping_add(x[2 * i + 1]) } else { y[2 * (i - 4)].wrapping_add(y[2 * (i - 4) + 1]) }))
        }
        VI_PHADDD => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            pk::<u32, 4>(core::array::from_fn(|i| if i < 2 { x[2 * i].wrapping_add(x[2 * i + 1]) } else { y[2 * (i - 2)].wrapping_add(y[2 * (i - 2) + 1]) }))
        }
        VI_PHSUBW => {
            let (x, y): ([u16; 8], [u16; 8]) = (ln(a), ln(b));
            pk::<u16, 8>(core::array::from_fn(|i| if i < 4 { x[2 * i].wrapping_sub(x[2 * i + 1]) } else { y[2 * (i - 4)].wrapping_sub(y[2 * (i - 4) + 1]) }))
        }
        VI_PHSUBD => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            pk::<u32, 4>(core::array::from_fn(|i| if i < 2 { x[2 * i].wrapping_sub(x[2 * i + 1]) } else { y[2 * (i - 2)].wrapping_sub(y[2 * (i - 2) + 1]) }))
        }
        VI_PSADBW => {
            let (x, y): ([u8; 16], [u8; 16]) = (ln(a), ln(b));
            let s0: u64 = (0..8).map(|i| x[i].abs_diff(y[i]) as u64).sum();
            let s1: u64 = (8..16).map(|i| x[i].abs_diff(y[i]) as u64).sum();
            pk::<u64, 2>([s0, s1])
        }
        VI_VPSLLVD => map2::<u32, 4>(a, b, |x, c| if c > 31 { 0 } else { x << c }),
        VI_VPSRLVD => map2::<u32, 4>(a, b, |x, c| if c > 31 { 0 } else { x >> c }),
        VI_VPSRAVD => map2::<u32, 4>(a, b, |x, c| ((x as i32) >> c.min(31)) as u32),
        VI_VPSLLVQ => map2::<u64, 2>(a, b, |x, c| if c > 63 { 0 } else { x << c }),
        VI_VPSRLVQ => map2::<u64, 2>(a, b, |x, c| if c > 63 { 0 } else { x >> c }),
        _ => {
            let x = if (imm & 0x01) != 0 { (a >> 64) as u64 } else { a as u64 };
            let y = if (imm & 0x10) != 0 { (b >> 64) as u64 } else { b as u64 };
            #[cfg(all(target_arch = "aarch64", target_feature = "aes"))]
            {
                unsafe { core::arch::aarch64::vmull_p64(x, y) }
            }
            #[cfg(not(all(target_arch = "aarch64", target_feature = "aes")))]
            {
                let mut res = 0u128;
                let mut xx = x as u128;
                let mut yy = y;
                while yy != 0 {
                    if (yy & 1) != 0 {
                        res ^= xx;
                    }
                    xx <<= 1;
                    yy >>= 1;
                }
                res
            }
        }
    }
}

/// `K_VFP_*` sub-ops (NEON-accelerated on `aarch64` with exact x86 NaN/rounding fallback).
#[inline(always)]
fn vfp(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::*;
        unsafe {
            match sub {
                12..=15 => {
                    let va = vreinterpretq_f32_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f32_u8(u128_to_u8x16(b));
                    let r = match sub {
                        12 => vaddq_f32(va, vb),
                        13 => vsubq_f32(va, vb),
                        14 => vmulq_f32(va, vb),
                        _ => vdivq_f32(va, vb),
                    };
                    // Fast exit when no lane is NaN (all finite/Inf/±0 results are bit-exact with x86).
                    if vminvq_u32(vceqq_f32(r, r)) != 0 {
                        return core::mem::transmute(r);
                    }
                }
                16 => {
                    let va = vreinterpretq_f32_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f32_u8(u128_to_u8x16(b));
                    return core::mem::transmute(vbslq_f32(vcltq_f32(va, vb), va, vb));
                }
                17 => {
                    let va = vreinterpretq_f32_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f32_u8(u128_to_u8x16(b));
                    return core::mem::transmute(vbslq_f32(vcgtq_f32(va, vb), va, vb));
                }
                18..=21 => {
                    let va = vreinterpretq_f64_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f64_u8(u128_to_u8x16(b));
                    let r = match sub {
                        18 => vaddq_f64(va, vb),
                        19 => vsubq_f64(va, vb),
                        20 => vmulq_f64(va, vb),
                        _ => vdivq_f64(va, vb),
                    };
                    if vminvq_u32(vreinterpretq_u32_u64(vceqq_f64(r, r))) != 0 {
                        return core::mem::transmute(r);
                    }
                }
                22 => {
                    let va = vreinterpretq_f64_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f64_u8(u128_to_u8x16(b));
                    return core::mem::transmute(vbslq_f64(vcltq_f64(va, vb), va, vb));
                }
                23 => {
                    let va = vreinterpretq_f64_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f64_u8(u128_to_u8x16(b));
                    return core::mem::transmute(vbslq_f64(vcgtq_f64(va, vb), va, vb));
                }
                VF_SQRTPS => {
                    let r = vsqrtq_f32(vreinterpretq_f32_u8(u128_to_u8x16(b)));
                    if vminvq_u32(vceqq_f32(r, r)) != 0 {
                        return core::mem::transmute(r);
                    }
                }
                VF_SQRTPD => {
                    let r = vsqrtq_f64(vreinterpretq_f64_u8(u128_to_u8x16(b)));
                    if vminvq_u32(vreinterpretq_u32_u64(vceqq_f64(r, r))) != 0 {
                        return core::mem::transmute(r);
                    }
                }
                VF_CVTDQ2PS => {
                    return core::mem::transmute(vcvtq_f32_s32(vreinterpretq_s32_u8(u128_to_u8x16(b))));
                }
                VF_ROUNDPS => {
                    let vb = vreinterpretq_f32_u8(u128_to_u8x16(b));
                    let r = match imm & 3 {
                        0 => vrndnq_f32(vb),
                        1 => vrndmq_f32(vb),
                        2 => vrndpq_f32(vb),
                        _ => vrndq_f32(vb),
                    };
                    return core::mem::transmute(r);
                }
                VF_ROUNDPD => {
                    let vb = vreinterpretq_f64_u8(u128_to_u8x16(b));
                    let r = match imm & 3 {
                        0 => vrndnq_f64(vb),
                        1 => vrndmq_f64(vb),
                        2 => vrndpq_f64(vb),
                        _ => vrndq_f64(vb),
                    };
                    return core::mem::transmute(r);
                }
                VF_HADDPS | VF_HSUBPS => {
                    let va = vreinterpretq_f32_u8(u128_to_u8x16(a));
                    let vb = vreinterpretq_f32_u8(u128_to_u8x16(b));
                    let ev = vuzp1q_f32(va, vb);
                    let od = vuzp2q_f32(va, vb);
                    let r = if sub == VF_HADDPS { vaddq_f32(ev, od) } else { vsubq_f32(ev, od) };
                    if vminvq_u32(vceqq_f32(r, r)) != 0 {
                        return core::mem::transmute(r);
                    }
                }
                _ => {}
            }
        }
    }
    const M32: u128 = 0xFFFF_FFFF;
    const M64: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    let f32b = |v: u128| f32::from_bits(v as u32);
    let f64b = |v: u128| f64::from_bits(v as u64);
    let rmode32 = |f: f32| match imm & 3 {
        0 => f.round_ties_even(),
        1 => f.floor(),
        2 => f.ceil(),
        _ => f.trunc(),
    };
    let rmode64 = |f: f64| match imm & 3 {
        0 => f.round_ties_even(),
        1 => f.floor(),
        2 => f.ceil(),
        _ => f.trunc(),
    };
    match sub {
        0..=5 => (a & !M32) | sse_arith32(sub, f32b(a), f32b(b)).to_bits() as u128,
        6..=11 => (a & !M64) | sse_arith64(sub - 6, f64b(a), f64b(b)).to_bits() as u128,
        12..=17 => map2::<u32, 4>(a, b, |x, y| sse_arith32(sub - 12, f32::from_bits(x), f32::from_bits(y)).to_bits()),
        18..=23 => map2::<u64, 2>(a, b, |x, y| sse_arith64(sub - 18, f64::from_bits(x), f64::from_bits(y)).to_bits()),
        VF_SQRTSS => (a & !M32) | sse_sqrt32(f32b(b)).to_bits() as u128,
        VF_SQRTSD => (a & !M64) | sse_sqrt64(f64b(b)).to_bits() as u128,
        VF_SQRTPS => map1::<u32, 4>(b, |x| sse_sqrt32(f32::from_bits(x)).to_bits()),
        VF_SQRTPD => map1::<u64, 2>(b, |x| sse_sqrt64(f64::from_bits(x)).to_bits()),
        VF_CVTSS2SD => (a & !M64) | (f32b(b) as f64).to_bits() as u128,
        VF_CVTSD2SS => (a & !M32) | (f64b(b) as f32).to_bits() as u128,
        VF_CVTDQ2PS => map1::<u32, 4>(b, |x| (x as i32 as f32).to_bits()),
        VF_CVTTPS2DQ | VF_CVTPS2DQ => {
            let trunc = sub == VF_CVTTPS2DQ;
            map1::<u32, 4>(b, |x| {
                let f = f32::from_bits(x);
                let r = if trunc { f } else { f.round_ties_even() };
                if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
                    i32::MIN as u32
                } else {
                    r as i32 as u32
                }
            })
        }
        // rcp: 1/x already propagates a NaN quieted (x86 rule). rsqrt: the sqrt gets the x86
        // rules (negative -> -qNaN indefinite), which 1/r then passes through.
        VF_RCPSS => (a & !M32) | (1.0f32 / f32b(b)).to_bits() as u128,
        VF_RSQRTSS => (a & !M32) | (1.0f32 / sse_sqrt32(f32b(b))).to_bits() as u128,
        VF_RCPPS => map1::<u32, 4>(b, |x| (1.0f32 / f32::from_bits(x)).to_bits()),
        VF_RSQRTPS => map1::<u32, 4>(b, |x| (1.0f32 / sse_sqrt32(f32::from_bits(x))).to_bits()),
        // addsub / hadd / hsub: `l op r` with x86 NaN rules, `l` being the first source
        // (both NaN -> l quieted; invalid -> -qNaN indefinite).
        VF_ADDSUBPS => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            pk::<u32, 4>(core::array::from_fn(|i| {
                let (fa, fb) = (f32::from_bits(x[i]), f32::from_bits(y[i]));
                x86_nan32(fa, fb, if i & 1 == 0 { fa - fb } else { fa + fb }).to_bits()
            }))
        }
        VF_ADDSUBPD => {
            let (x, y): ([u64; 2], [u64; 2]) = (ln(a), ln(b));
            let (a0, a1) = (f64::from_bits(x[0]), f64::from_bits(x[1]));
            let (b0, b1) = (f64::from_bits(y[0]), f64::from_bits(y[1]));
            pk::<u64, 2>([x86_nan64(a0, b0, a0 - b0).to_bits(), x86_nan64(a1, b1, a1 + b1).to_bits()])
        }
        VF_HADDPS | VF_HSUBPS => {
            let is_add = sub == VF_HADDPS;
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            let op = |u: u32, v: u32| {
                let (fu, fv) = (f32::from_bits(u), f32::from_bits(v));
                x86_nan32(fu, fv, if is_add { fu + fv } else { fu - fv }).to_bits()
            };
            pk::<u32, 4>([op(x[0], x[1]), op(x[2], x[3]), op(y[0], y[1]), op(y[2], y[3])])
        }
        VF_HADDPD | VF_HSUBPD => {
            let is_add = sub == VF_HADDPD;
            let (x, y): ([u64; 2], [u64; 2]) = (ln(a), ln(b));
            let op = |u: u64, v: u64| {
                let (fu, fv) = (f64::from_bits(u), f64::from_bits(v));
                x86_nan64(fu, fv, if is_add { fu + fv } else { fu - fv }).to_bits()
            };
            pk::<u64, 2>([op(x[0], x[1]), op(y[0], y[1])])
        }
        VF_ROUNDSS => (a & !M32) | rmode32(f32b(b)).to_bits() as u128,
        VF_ROUNDSD => (a & !M64) | rmode64(f64b(b)).to_bits() as u128,
        VF_ROUNDPS => map1::<u32, 4>(b, |x| rmode32(f32::from_bits(x)).to_bits()),
        VF_ROUNDPD => map1::<u64, 2>(b, |x| rmode64(f64::from_bits(x)).to_bits()),
        VF_CMPSS => {
            let m = if sse_cmp_pred(f32b(a) as f64, f32b(b) as f64, imm) { M32 } else { 0 };
            (a & !M32) | m
        }
        VF_CMPSD => {
            let m = if sse_cmp_pred(f64b(a), f64b(b), imm) { M64 } else { 0 };
            (a & !M64) | m
        }
        VF_CMPPS => map2::<u32, 4>(a, b, |x, y| {
            if sse_cmp_pred(f32::from_bits(x) as f64, f32::from_bits(y) as f64, imm) { !0 } else { 0 }
        }),
        VF_CMPPD => map2::<u64, 2>(a, b, |x, y| {
            if sse_cmp_pred(f64::from_bits(x), f64::from_bits(y), imm) { !0 } else { 0 }
        }),
        VF_DPPS => {
            let (x, y): ([u32; 4], [u32; 4]) = (ln(a), ln(b));
            let p = |i: usize| if (imm >> (4 + i)) & 1 != 0 { f32::from_bits(x[i]) * f32::from_bits(y[i]) } else { 0.0 };
            let sum = ((p(0) + p(1)) + (p(2) + p(3))).to_bits();
            pk::<u32, 4>(core::array::from_fn(|i| if (imm >> i) & 1 != 0 { sum } else { 0 }))
        }
        _ => b,
    }
}

impl Cpu {
    /// Memory second source of a V-form `*_RM` op: `(bits 127:0, bits 255:128)`.
    #[inline(always)]
    unsafe fn vload(&self, op: &FastOp) -> (u128, u128) {
        let a = self.faddr(op);
        match op.w >> 4 {
            2 => (self.read_mem(a, 32) as u128, 0),
            3 => (self.read_mem(a, 64) as u128, 0),
            4 => (self.read_u128(a), 0),
            _ => (self.read_u128(a), self.read_u128(a.wrapping_add(16))),
        }
    }

    /// Execute a V-form op with lane function `f` (second source from memory if `mem`).
    /// Monomorphized over `F` and `IS_VSHUF` so the lane op inlines cleanly and 128-bit ops
    /// skip loading `ymm_hi`.
    #[inline(always)]
    unsafe fn vexec<const IS_VSHUF: bool, F: Fn(u8, u128, u128, u8) -> u128>(&mut self, op: &FastOp, mem: bool, f: F) {
        let sub = op.x & 0x3F;
        let imm8 = op.imm as u8;
        let s1 = (op.imm >> 8) as usize & 15;
        let mode = op.w & 3;
        let (b_lo, b_hi) = if mem {
            self.vload(op)
        } else {
            let r = op.r1 as usize & 15;
            (self.xmm[r], if mode == VM_V256 { self.ymm_hi[r] } else { 0 })
        };
        let lo = f(sub, self.xmm[s1], b_lo, imm8);
        let d = op.r0 as usize & 15;
        match mode {
            VM_LEG => self.xmm[d] = lo,
            VM_V128 => {
                self.xmm[d] = lo;
                self.ymm_hi[d] = 0;
            }
            _ => {
                let hi_imm = if IS_VSHUF {
                    match sub {
                        VS_SHUFPD | VS_BLENDPD => imm8 >> 2,
                        VS_BLENDPS => imm8 >> 4,
                        _ => imm8,
                    }
                } else {
                    imm8
                };
                let b_lane_hi = if IS_VSHUF && (VS_PSRLW..=VS_PSLLQ).contains(&sub) {
                    b_lo
                } else {
                    b_hi
                };
                let hi = f(sub, self.ymm_hi[s1], b_lane_hi, hi_imm);
                self.xmm[d] = lo;
                self.ymm_hi[d] = hi;
            }
        }
    }

    /// `(u)comiss/(u)comisd` flags (same as the generic handlers).
    #[inline(always)]
    fn comi(&mut self, fa: f64, fb: f64) {
        self.sf = false;
        self.of = false;
        let (zf, pf, cf) = if fa.is_nan() || fb.is_nan() {
            (true, true, true)
        } else if fa > fb {
            (false, false, false)
        } else if fa < fb {
            (false, false, true)
        } else {
            (true, false, false)
        };
        self.zf = zf;
        self.set_pf(pf);
        self.cf = cf;
    }

    /// 2/3-operand `imul` result + CF/OF (generic semantics; other flags untouched).
    #[inline(always)]
    fn imul_w(&mut self, a: u64, b: u64, w: u8) -> u64 {
        let (a, b) = (sext(a, w) as i64 as i128, sext(b, w) as i64 as i128);
        let prod = a.wrapping_mul(b);
        let fits = match w {
            16 => prod == (prod as i16 as i128),
            32 => prod == (prod as i32 as i128),
            _ => prod == (prod as i64 as i128),
        };
        self.cf = !fits;
        self.of = !fits;
        prod as u64
    }
}

/// Run fast ops of one decoded block starting at `op` (a non-generic op; `cpu.rip` already
/// holds its next IP) until one leaves straight-line flow, the next slot is generic or not
/// sequential, the block ends (`left` = slots after `op`), or `budget` ops have run.
/// Fused `cmp/test + jcc` pairs execute both slots. Returns the last op's effect, the number
/// of ops executed (>= 1) and the last op executed.
///
/// Kept out of line and small so the loop state stays in registers (the giant
/// `emu_run_full` frame spills everything).
#[inline(never)]
pub(super) unsafe fn run_chain(cpu: &mut Cpu, mut op: *const FastOp, mut left: usize, budget: u32) -> (Fx, u32, *const FastOp) {
    let mut n = 0u32;
    loop {
        let fx = exec(cpu, &*op);
        n += 1;
        match fx {
            Fx::Done => {
                if left != 0 && n < budget {
                    let nx = op.add(1);
                    if (*nx).kind != K_GENERIC && (*nx).ip == cpu.rip {
                        op = nx;
                        left -= 1;
                        cpu.rip = (*nx).ip.wrapping_add((*nx).len as u64);
                        continue;
                    }
                }
                return (Fx::Done, n, op);
            }
            Fx::FuseJcc => {
                // `fuse` guarantees the next slot of this block is the paired `K_JCC`.
                // Evaluate the branch inline without a recursive `exec` call; when not taken,
                // keep executing straight through the rest of the block!
                let j = op.add(1);
                let jop = &*j;
                cpu.rip = jop.ip.wrapping_add(jop.len as u64);
                n += 1;
                if cpu.eval_cc_u8(jop.x) {
                    if jop.len <= 2 {
                        cpu.rip = jop.imm;
                        return (Fx::Done, n, j);
                    }
                    return (Fx::JccNear(jop.imm), n, j);
                }
                left -= 1;
                if left != 0 && n < budget {
                    let nx = j.add(1);
                    if (*nx).kind != K_GENERIC && (*nx).ip == cpu.rip {
                        op = nx;
                        left -= 1;
                        cpu.rip = (*nx).ip.wrapping_add((*nx).len as u64);
                        continue;
                    }
                }
                return (Fx::Done, n, j);
            }
            other => return (other, n, op),
        }
    }
}

/// Execute a non-generic fast op. `cpu.rip` must already hold the next instruction's IP.
#[inline(always)]
pub(super) unsafe fn exec(cpu: &mut Cpu, op: &FastOp) -> Fx {
    let w = op.w;
    match op.kind {
        K_NOP => {}
        K_MOV_RR => {
            let v = cpu.rget(op.r1, w);
            cpu.rset(op.r0, w, v);
        }
        K_MOV_RM => {
            let v = cpu.read_mem(cpu.faddr(op), w as u32);
            cpu.rset(op.r0, w, v);
        }
        K_MOV_MR => {
            let v = cpu.rget(op.r1, w);
            cpu.write_mem(cpu.faddr(op), w as u32, v);
        }
        K_MOV_RI => cpu.rset(op.r0, w, op.imm),
        K_MOV_MI => cpu.write_mem(cpu.faddr(op), w as u32, op.imm),
        K_MOVZX_RR => {
            let v = cpu.rget(op.r1, op.x & 0x3F);
            cpu.rset(op.r0, w, v);
        }
        K_MOVZX_RM => {
            let v = cpu.read_mem(cpu.faddr(op), (op.x & 0x3F) as u32);
            cpu.rset(op.r0, w, v);
        }
        K_MOVSX_RR => {
            let sw = op.x & 0x3F;
            let v = sext(cpu.rget(op.r1, sw), sw);
            cpu.rset(op.r0, w, v);
        }
        K_MOVSX_RM => {
            let sw = op.x & 0x3F;
            let v = sext(cpu.read_mem(cpu.faddr(op), sw as u32), sw);
            cpu.rset(op.r0, w, v);
        }
        K_LEA => {
            let a = cpu.faddr(op);
            cpu.rset(op.r0, w, a);
        }
        K_ALU_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_RI => {
            let a = cpu.rget(op.r0, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, op.imm, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let a = cpu.rget(op.r0, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_MR => {
            let addr = cpu.faddr(op);
            let b = cpu.rget(op.r1, w);
            let a = cpu.read_mem(addr, w as u32);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_ALU_MI => {
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, op.imm, w as u32) {
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_INC_R => {
            let saved_cf = cpu.cf;
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_add(a, 1, w as u32);
            cpu.rset(op.r0, w, r);
            cpu.cf = saved_cf;
        }
        K_DEC_R => {
            let saved_cf = cpu.cf;
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_sub(a, 1, w as u32);
            cpu.rset(op.r0, w, r);
            cpu.cf = saved_cf;
        }
        K_SHIFT_RI => {
            let a = cpu.rget(op.r0, w);
            let r = cpu.shift(op.x & 0x3F, a, op.imm as u32, w);
            cpu.rset(op.r0, w, r);
        }
        K_SHIFT_RCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = (cpu.gpr[1] as u8 as u32) & mask;
            if cnt > 0 {
                let a = cpu.rget(op.r0, w);
                let r = cpu.shift(op.x & 0x3F, a, cnt, w);
                cpu.rset(op.r0, w, r);
            } else if w == 32 {
                // Count 0: flags unchanged, but a 32-bit destination is still zero-extended.
                cpu.rset(op.r0, 32, cpu.rget(op.r0, 32));
            }
        }
        K_PUSH_R => {
            let v = cpu.gpr[op.r0 as usize & 15];
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, v);
        }
        K_POP_R => {
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            cpu.rset(op.r0, 64, v);
        }
        K_RET => {
            let rsp = cpu.gpr[4];
            let ret_addr = super::rd::<u64>(rsp as usize);
            cpu.gpr[4] = rsp.wrapping_add(8 + op.imm);
            cpu.rip = ret_addr;
        }
        K_JCC => {
            if cpu.eval_cc_u8(op.x) {
                if op.len <= 2 {
                    cpu.rip = op.imm;
                } else {
                    return Fx::JccNear(op.imm);
                }
            }
        }
        K_JMP => return Fx::Jmp(op.imm),
        K_CALL => return Fx::Call(op.imm),
        K_CALL_R => return Fx::Call(cpu.gpr[op.r0 as usize & 15]),
        K_CALL_M => return Fx::Call(cpu.read_mem(cpu.faddr(op), 64)),
        K_JMP_R => return Fx::Jmp(cpu.gpr[op.r0 as usize & 15]),
        K_JMP_M => return Fx::Jmp(cpu.read_mem(cpu.faddr(op), 64)),
        K_CMPJ_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            cpu.cmp_or_test(op.x & 0x3F, a, b, w as u32);
            return Fx::FuseJcc;
        }
        K_CMPJ_RI => {
            let a = cpu.rget(op.r0, w);
            cpu.cmp_or_test(op.x & 0x3F, a, op.imm, w as u32);
            return Fx::FuseJcc;
        }
        K_CMPJ_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let a = cpu.rget(op.r0, w);
            cpu.cmp_or_test(op.x & 0x3F, a, b, w as u32);
            return Fx::FuseJcc;
        }
        K_CMOV_RR => {
            if cpu.eval_cc_u8(op.x) {
                let v = cpu.rget(op.r1, w);
                cpu.rset(op.r0, w, v);
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_CMOV_RM => {
            if cpu.eval_cc_u8(op.x) {
                let v = cpu.read_mem(cpu.faddr(op), w as u32);
                cpu.rset(op.r0, w, v);
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_SETCC_R => {
            let v = cpu.eval_cc_u8(op.x) as u64;
            cpu.rset(op.r0, 8, v);
        }
        _ => {
            if op.kind >= K_VSHUF_RR {
                exec_ext(cpu, op);
            }
        }
    }
    Fx::Done
}

/// Handlers for kinds >= [`K_VSHUF_RR`] (SSE/AVX and the extra integer ops). Out of line so
/// the hot integer `exec`/`run_chain` code stays compact; all of them return [`Fx::Done`].
#[inline(never)]
pub(super) unsafe fn exec_ext(cpu: &mut Cpu, op: &FastOp) {
    let w = op.w;
    match op.kind {
        K_VSHUF_RR => cpu.vexec::<true, _>(op, false, vshuf),
        K_VSHUF_RM => cpu.vexec::<true, _>(op, true, vshuf),
        K_VINT_RR => cpu.vexec::<false, _>(op, false, vint),
        K_VINT_RM => cpu.vexec::<false, _>(op, true, vint),
        K_VFP_RR => cpu.vexec::<false, _>(op, false, vfp),
        K_VFP_RM => cpu.vexec::<false, _>(op, true, vfp),
        K_VST => {
            let a = cpu.faddr(op);
            let s = op.r1 as usize & 15;
            match op.w >> 4 {
                2 => cpu.write_mem(a, 32, cpu.xmm[s] as u32 as u64),
                3 => cpu.write_mem(a, 64, cpu.xmm[s] as u64),
                4 => cpu.write_u128(a, cpu.xmm[s]),
                6 => cpu.write_mem(a, 64, (cpu.xmm[s] >> 64) as u64),
                _ => {
                    cpu.write_u128(a, cpu.xmm[s]);
                    cpu.write_u128(a.wrapping_add(16), cpu.ymm_hi[s]);
                }
            }
        }
        K_MOVD_XG => {
            let v = cpu.gpr[op.r1 as usize & 15];
            let d = op.r0 as usize & 15;
            cpu.xmm[d] = if op.imm == 64 { v as u128 } else { v as u32 as u128 };
            if op.w & 3 == VM_V128 {
                cpu.ymm_hi[d] = 0;
            }
        }
        K_MOVD_GX => {
            let v = cpu.xmm[op.r1 as usize & 15] as u64;
            cpu.rset(op.r0, w, v);
        }
        K_CVTSI_R | K_CVTSI_M => {
            let width = op.imm as u8;
            let v = if op.kind == K_CVTSI_R { cpu.rget(op.r1, width) } else { cpu.read_mem(cpu.faddr(op), width as u32) };
            let iv = if width == 64 { v as i64 } else { v as u32 as i32 as i64 };
            let base = cpu.xmm[(op.imm >> 8) as usize & 15];
            let d = op.r0 as usize & 15;
            cpu.xmm[d] = if op.x & 0x3F == 0 {
                (base & !0xFFFF_FFFF) | ((iv as f32).to_bits() as u128)
            } else {
                (base & !0xFFFF_FFFF_FFFF_FFFF) | ((iv as f64).to_bits() as u128)
            };
            if op.w & 3 == VM_V128 {
                cpu.ymm_hi[d] = 0;
            }
        }
        K_COMI_RR | K_COMI_RM => {
            let sd = op.x & 0x3F != 0;
            let a = cpu.xmm[op.r0 as usize & 15];
            let b = if op.kind == K_COMI_RR {
                cpu.xmm[op.r1 as usize & 15]
            } else {
                cpu.read_mem(cpu.faddr(op), if sd { 64 } else { 32 }) as u128
            };
            if sd {
                cpu.comi(f64::from_bits(a as u64), f64::from_bits(b as u64));
            } else {
                cpu.comi(f32::from_bits(a as u32) as f64, f32::from_bits(b as u32) as f64);
            }
        }
        K_CVTF2I_R | K_CVTF2I_M => {
            let sd = op.x & 1 != 0;
            let trunc = op.x & 2 != 0;
            let s = if op.kind == K_CVTF2I_R {
                cpu.xmm[op.r1 as usize & 15] as u64
            } else {
                cpu.read_mem(cpu.faddr(op), if sd { 64 } else { 32 })
            };
            let f = if sd { f64::from_bits(s) } else { f32::from_bits(s as u32) as f64 };
            let iv = if w == 64 { sse_cvt_i64(f, trunc) } else { sse_cvt_i32(f, trunc) as u64 };
            cpu.rset(op.r0, w, iv);
        }
        K_IMUL_RR => {
            let r = cpu.imul_w(cpu.rget(op.r0, w), cpu.rget(op.r1, w), w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let r = cpu.imul_w(cpu.rget(op.r0, w), b, w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RRI => {
            let r = cpu.imul_w(cpu.rget(op.r1, w), op.imm, w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RMI => {
            let a = cpu.read_mem(cpu.faddr(op), w as u32);
            let r = cpu.imul_w(a, op.imm, w);
            cpu.rset(op.r0, w, r);
        }
        K_NEG_R => {
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_sub(0, a, w as u32);
            cpu.cf = a != 0;
            cpu.rset(op.r0, w, r);
        }
        K_NOT_R => {
            let a = cpu.rget(op.r0, w);
            cpu.rset(op.r0, w, !a);
        }
        K_INC_M | K_DEC_M => {
            let saved_cf = cpu.cf;
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            let r = if op.kind == K_INC_M { cpu.update_flags_add(a, 1, w as u32) } else { cpu.update_flags_sub(a, 1, w as u32) };
            cpu.write_mem(addr, w as u32, r);
            cpu.cf = saved_cf;
        }
        K_CX => match op.x & 0x3F {
            0 => cpu.gpr[0] = (cpu.gpr[0] as u16 as i16 as i32 as u32) as u64,
            1 => cpu.gpr[0] = cpu.gpr[0] as i32 as i64 as u64,
            2 => cpu.gpr[2] = if (cpu.gpr[0] as i32) < 0 { 0xFFFF_FFFF } else { 0 },
            3 => cpu.gpr[2] = if (cpu.gpr[0] as i64) < 0 { u64::MAX } else { 0 },
            4 => cpu.gpr[0] = (cpu.gpr[0] & !0xFFFF) | ((cpu.gpr[0] as u8 as i8 as i16 as u16) as u64),
            _ => {
                let hi = if (cpu.gpr[0] as i16) < 0 { 0xFFFFu64 } else { 0 };
                cpu.gpr[2] = (cpu.gpr[2] & !0xFFFF) | hi;
            }
        },
        K_VZU => cpu.ymm_hi = [0; 16],
        K_BSF_RR | K_BSF_RM => {
            let v = if op.kind == K_BSF_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            cpu.cf = v == 0;
            cpu.zf = v == 0;
            if v != 0 {
                cpu.rset(op.r0, w, v.trailing_zeros() as u64);
            }
        }
        K_BT_RR | K_BT_RI => {
            let idx = if op.kind == K_BT_RR { cpu.rget(op.r1, w) } else { op.imm };
            cpu.cf = (cpu.rget(op.r0, w) >> (idx as u32 & (w as u32 - 1))) & 1 != 0;
        }
        K_ROT_RI | K_ROT_RCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_ROT_RI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            if cnt != 0 {
                let a = cpu.rget(op.r0, w);
                let ror = op.x & 0x3F != 0;
                let res = match (w, ror) {
                    (8, false) => (a as u8).rotate_left(cnt) as u64,
                    (16, false) => (a as u16).rotate_left(cnt) as u64,
                    (32, false) => (a as u32).rotate_left(cnt) as u64,
                    (_, false) => a.rotate_left(cnt),
                    (8, true) => (a as u8).rotate_right(cnt) as u64,
                    (16, true) => (a as u16).rotate_right(cnt) as u64,
                    (32, true) => (a as u32).rotate_right(cnt) as u64,
                    (_, true) => a.rotate_right(cnt),
                };
                cpu.rset(op.r0, w, res);
                let msb = (res >> (w - 1)) & 1 != 0;
                if ror {
                    cpu.cf = msb;
                    cpu.of = msb ^ ((res >> (w - 2)) & 1 != 0);
                } else {
                    cpu.cf = res & 1 != 0;
                    cpu.of = msb ^ cpu.cf;
                }
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_XCHG_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            cpu.rset(op.r0, w, b);
            cpu.rset(op.r1, w, a);
        }
        K_BSWAP_R => {
            let v = cpu.rget(op.r0, w);
            let res = if w == 64 { v.swap_bytes() } else { (v as u32).swap_bytes() as u64 };
            cpu.rset(op.r0, w, res);
        }
        K_BSR_RR | K_BSR_RM => {
            let v = if op.kind == K_BSR_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            cpu.zf = v == 0;
            if v != 0 {
                cpu.rset(op.r0, w, 63u64.wrapping_sub(v.leading_zeros() as u64));
            }
        }
        K_TZCNT_RR | K_TZCNT_RM => {
            let v = if op.kind == K_TZCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            if v == 0 {
                cpu.zf = false;
                cpu.cf = true;
                cpu.rset(op.r0, w, w as u64);
            } else {
                let idx = v.trailing_zeros() as u64;
                cpu.zf = idx == 0;
                cpu.cf = false;
                cpu.rset(op.r0, w, idx);
            }
        }
        K_LZCNT_RR | K_LZCNT_RM => {
            let v = if op.kind == K_LZCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            let res = match w {
                16 => (v as u16).leading_zeros() as u64,
                32 => (v as u32).leading_zeros() as u64,
                _ => v.leading_zeros() as u64,
            };
            cpu.rset(op.r0, w, res);
            cpu.zf = res == 0;
            cpu.cf = res == w as u64;
        }
        K_POPCNT_RR | K_POPCNT_RM => {
            let v = if op.kind == K_POPCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            let res = v.count_ones() as u64;
            cpu.rset(op.r0, w, res);
            cpu.zf = res == 0;
            cpu.cf = false;
            cpu.sf = false;
            cpu.of = false;
            cpu.set_pf(false);
        }
        K_BIT_RR | K_BIT_RI => {
            let idx = if op.kind == K_BIT_RR { cpu.rget(op.r1, w) } else { op.imm };
            let b = (idx as u32) & (w as u32 - 1);
            let a = cpu.rget(op.r0, w);
            cpu.cf = ((a >> b) & 1) != 0;
            let res = match op.x & 0x3F {
                0 => a | (1u64 << b),
                1 => a & !(1u64 << b),
                _ => a ^ (1u64 << b),
            };
            cpu.rset(op.r0, w, res);
        }
        K_SHD_RI | K_SHD_RCL => {
            let mask = if w == 64 { 63u32 } else { 31u32 };
            let cnt = if op.kind == K_SHD_RI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            if cnt != 0 {
                let dst = cpu.rget(op.r0, w);
                let src = cpu.rget(op.r1, w);
                let shrd = (op.x & 0x3F) != 0;
                let wb = w as u32;
                let res = if shrd {
                    if wb == 64 {
                        (dst >> cnt) | (src.wrapping_shl(64 - cnt))
                    } else if wb == 32 {
                        (((dst as u32) >> cnt) | ((src as u32).wrapping_shl(32 - cnt))) as u64
                    } else {
                        (((dst as u16) >> cnt) | ((src as u16).wrapping_shl(16 - cnt))) as u64
                    }
                } else if wb == 64 {
                    (dst << cnt) | (src.wrapping_shr(64 - cnt))
                } else if wb == 32 {
                    (((dst as u32) << cnt) | ((src as u32).wrapping_shr(32 - cnt))) as u64
                } else {
                    (((dst as u16) << cnt) | ((src as u16).wrapping_shr(16 - cnt))) as u64
                };
                cpu.update_flags_logic(res, wb);
                cpu.cf = if shrd {
                    ((dst >> (cnt - 1)) & 1) != 0
                } else {
                    ((dst >> (wb - cnt)) & 1) != 0
                };
                cpu.rset(op.r0, w, res);
            }
        }
        K_MUL1_R | K_MUL1_M => {
            let src = if op.kind == K_MUL1_R { cpu.rget(op.r0, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            match op.x & 0x3F {
                0 => {
                    if w == 64 {
                        let prod = (cpu.gpr[0] as u128).wrapping_mul(src as u128);
                        cpu.gpr[0] = prod as u64;
                        cpu.gpr[2] = (prod >> 64) as u64;
                        cpu.cf = cpu.gpr[2] != 0;
                        cpu.of = cpu.cf;
                    } else if w == 32 {
                        let prod = (cpu.gpr[0] as u32 as u64).wrapping_mul(src as u32 as u64);
                        cpu.gpr[0] = prod as u32 as u64;
                        cpu.gpr[2] = (prod >> 32) as u32 as u64;
                        cpu.cf = cpu.gpr[2] != 0;
                        cpu.of = cpu.cf;
                    } else if w == 16 {
                        let prod = (cpu.gpr[0] as u16 as u32) * (src as u16 as u32);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.set_reg(Register::DX, (prod >> 16) as u64);
                        cpu.cf = (prod >> 16) != 0;
                        cpu.of = cpu.cf;
                    } else {
                        let prod = (cpu.gpr[0] as u8 as u16) * (src as u8 as u16);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.cf = (prod >> 8) != 0;
                        cpu.of = cpu.cf;
                    }
                }
                1 => {
                    if w == 64 {
                        let prod = (cpu.gpr[0] as i64 as i128).wrapping_mul(src as i64 as i128);
                        cpu.gpr[0] = prod as i64 as u64;
                        cpu.gpr[2] = (prod >> 64) as i64 as u64;
                        cpu.cf = prod != (prod as i64 as i128);
                        cpu.of = cpu.cf;
                    } else if w == 32 {
                        let prod = (cpu.gpr[0] as u32 as i32 as i64).wrapping_mul(src as u32 as i32 as i64);
                        cpu.gpr[0] = prod as i32 as u32 as u64;
                        cpu.gpr[2] = ((prod >> 32) as i32 as u32) as u64;
                        cpu.cf = prod != (prod as i32 as i64);
                        cpu.of = cpu.cf;
                    } else if w == 16 {
                        let prod = (cpu.gpr[0] as u16 as i16 as i32) * (src as u16 as i16 as i32);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.set_reg(Register::DX, (prod >> 16) as u64);
                        cpu.cf = prod != (prod as i16 as i32);
                        cpu.of = cpu.cf;
                    } else {
                        let prod = (cpu.gpr[0] as u8 as i8 as i16) * (src as u8 as i8 as i16);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.cf = prod != (prod as i8 as i16);
                        cpu.of = cpu.cf;
                    }
                }
                2 => {
                    if w == 64 && src != 0 {
                        let n = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                        cpu.gpr[0] = (n / (src as u128)) as u64;
                        cpu.gpr[2] = (n % (src as u128)) as u64;
                    } else if w == 32 && (src as u32) != 0 {
                        let n = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                        let d32 = src as u32 as u64;
                        cpu.gpr[0] = (n / d32) as u32 as u64;
                        cpu.gpr[2] = (n % d32) as u32 as u64;
                    } else if w == 16 && (src as u16) != 0 {
                        let n = ((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32);
                        let d16 = src as u16 as u32;
                        cpu.set_reg(Register::AX, (n / d16) as u64);
                        cpu.set_reg(Register::DX, (n % d16) as u64);
                    } else if w == 8 && (src as u8) != 0 {
                        let n = cpu.gpr[0] as u16;
                        let d8 = src as u8 as u16;
                        cpu.set_reg(Register::AL, (n / d8) as u64);
                        cpu.set_reg(Register::AH, (n % d8) as u64);
                    }
                }
                _ => {
                    if w == 64 && src != 0 {
                        let n = (((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128)) as i128;
                        let d64 = src as i64 as i128;
                        cpu.gpr[0] = n.wrapping_div(d64) as i64 as u64;
                        cpu.gpr[2] = n.wrapping_rem(d64) as i64 as u64;
                    } else if w == 32 && (src as u32) != 0 {
                        let n = (((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64)) as i64;
                        let d32 = src as u32 as i32 as i64;
                        cpu.gpr[0] = n.wrapping_div(d32) as i32 as u32 as u64;
                        cpu.gpr[2] = n.wrapping_rem(d32) as i32 as u32 as u64;
                    } else if w == 16 && (src as u16) != 0 {
                        let n = (((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32)) as i32;
                        let d16 = src as u16 as i16 as i32;
                        cpu.set_reg(Register::AX, n.wrapping_div(d16) as u64);
                        cpu.set_reg(Register::DX, n.wrapping_rem(d16) as u64);
                    } else if w == 8 && (src as u8) != 0 {
                        let n = cpu.gpr[0] as u16 as i16;
                        let d8 = src as u8 as i8 as i16;
                        cpu.set_reg(Register::AL, n.wrapping_div(d8) as u64);
                        cpu.set_reg(Register::AH, n.wrapping_rem(d8) as u64);
                    }
                }
            }
        }
        K_XADD_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            let res = cpu.update_flags_add(a, b, w as u32);
            cpu.rset(op.r0, w, res);
            cpu.rset(op.r1, w, a);
        }
        K_CMPXCHG_RR => {
            let acc = cpu.rget(0, w);
            let dst = cpu.rget(op.r0, w);
            let src = cpu.rget(op.r1, w);
            let _ = cpu.update_flags_sub(acc, dst, w as u32);
            if acc == dst {
                cpu.zf = true;
                cpu.rset(op.r0, w, src);
            } else {
                cpu.zf = false;
                cpu.rset(0, w, dst);
            }
        }
        K_SETCC_M => {
            let v = cpu.eval_cc_u8(op.x) as u64;
            cpu.write_mem(cpu.faddr(op), 8, v);
        }
        K_PUSH_I => {
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, op.imm);
        }
        K_PUSH_M => {
            let v = cpu.read_mem(cpu.faddr(op), 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, v);
        }
        K_POP_M => {
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            let a = cpu.faddr(op);
            cpu.write_mem(a, 64, v);
        }
        K_LEAVE => {
            cpu.gpr[4] = cpu.gpr[5];
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            cpu.rset(5, 64, v);
        }
        K_FLAG_OP => match op.x & 0x3F {
            0 => cpu.cf = false,
            1 => cpu.cf = true,
            2 => cpu.cf = !cpu.cf,
            3 => cpu.df = false,
            4 => cpu.df = true,
            _ => core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst),
        },
        K_SHIFT_MI | K_SHIFT_MCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_SHIFT_MI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if cnt > 0 {
                let r = cpu.shift(op.x & 0x3F, a, cnt, w);
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_ROT_MI | K_ROT_MCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_ROT_MI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if cnt != 0 {
                let ror = op.x & 0x3F != 0;
                let res = match (w, ror) {
                    (8, false) => (a as u8).rotate_left(cnt) as u64,
                    (16, false) => (a as u16).rotate_left(cnt) as u64,
                    (32, false) => (a as u32).rotate_left(cnt) as u64,
                    (_, false) => a.rotate_left(cnt),
                    (8, true) => (a as u8).rotate_right(cnt) as u64,
                    (16, true) => (a as u16).rotate_right(cnt) as u64,
                    (32, true) => (a as u32).rotate_right(cnt) as u64,
                    (_, true) => a.rotate_right(cnt),
                };
                cpu.write_mem(addr, w as u32, res);
                let msb = (res >> (w - 1)) & 1 != 0;
                if ror {
                    cpu.cf = msb;
                    cpu.of = msb ^ ((res >> (w - 2)) & 1 != 0);
                } else {
                    cpu.cf = res & 1 != 0;
                    cpu.of = msb ^ cpu.cf;
                }
            }
        }
        K_MOVBE_RM => {
            let v = cpu.read_mem(cpu.faddr(op), w as u32);
            let res = match w {
                16 => (v as u16).swap_bytes() as u64,
                32 => (v as u32).swap_bytes() as u64,
                _ => v.swap_bytes(),
            };
            cpu.rset(op.r0, w, res);
        }
        K_MOVBE_MR => {
            let v = cpu.rget(op.r1, w);
            let res = match w {
                16 => (v as u16).swap_bytes() as u64,
                32 => (v as u32).swap_bytes() as u64,
                _ => v.swap_bytes(),
            };
            cpu.write_mem(cpu.faddr(op), w as u32, res);
        }
        K_MOVMSK => {
            let s = op.r1 as usize & 15;
            let lo = cpu.xmm[s];
            #[inline(always)]
            fn mask_b(v: u128) -> u64 {
                #[cfg(target_arch = "aarch64")]
                {
                    neon_movemask_u8(v) as u64
                }
                #[cfg(not(target_arch = "aarch64"))]
                {
                    let b: [u8; 16] = ln(v);
                    (0..16).fold(0u64, |acc, i| acc | (((b[i] >> 7) as u64) << i))
                }
            }
            #[inline(always)]
            fn mask_ps(v: u128) -> u64 {
                ((v >> 31) & 1) as u64
                    | (((v >> 63) & 1) as u64) << 1
                    | (((v >> 95) & 1) as u64) << 2
                    | (((v >> 127) & 1) as u64) << 3
            }
            let res = match op.x & 0x3F {
                0 => mask_b(lo),
                1 => mask_b(lo) | (mask_b(cpu.ymm_hi[s]) << 16),
                2 => mask_ps(lo),
                3 => mask_ps(lo) | (mask_ps(cpu.ymm_hi[s]) << 4),
                4 => ((lo >> 63) & 1) as u64 | (((lo >> 127) & 1) as u64) << 1,
                _ => {
                    let hi = cpu.ymm_hi[s];
                    ((lo >> 63) & 1) as u64
                        | (((lo >> 127) & 1) as u64) << 1
                        | (((hi >> 63) & 1) as u64) << 2
                        | (((hi >> 127) & 1) as u64) << 3
                }
            };
            cpu.rset(op.r0, w, res);
        }
        K_PINSR => {
            let lg = (op.w >> 4) as u32;
            let ebits = 8u32 << lg;
            let lane = (op.imm & 0xFF) as u32;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let d = op.r0 as usize & 15;
            let src = if op.x & 0x20 != 0 {
                cpu.read_mem(cpu.faddr(op), ebits)
            } else {
                cpu.rget(op.r1, ebits as u8)
            };
            let mask = if ebits == 64 { u64::MAX as u128 } else { (1u128 << ebits) - 1 };
            let sh = lane * ebits;
            cpu.xmm[d] = (cpu.xmm[s1] & !(mask << sh)) | (((src as u128) & mask) << sh);
            if op.w & 3 == VM_V128 {
                cpu.ymm_hi[d] = 0;
            }
        }
        K_PEXTR => {
            let ebits = w as u32;
            let lane = (op.imm & 0xFF) as u32;
            let s = op.r1 as usize & 15;
            let mask = if ebits == 64 { u64::MAX as u128 } else { (1u128 << ebits) - 1 };
            let val = ((cpu.xmm[s] >> (lane * ebits)) & mask) as u64;
            if op.x & 0x20 != 0 {
                cpu.write_mem(cpu.faddr(op), ebits, val);
            } else {
                let gw = if ebits == 64 { 64 } else { 32 };
                cpu.rset(op.r0, gw, val);
            }
        }
        _ => exec_ext_more(cpu, op),
    }
}

/// Whether a fast op of this kind may be re-executed on a shadow CPU for verification
/// (no non-idempotent memory writes, no control transfer that needs host dispatch).
#[inline]
pub(super) fn verifiable(kind: u8) -> bool {
    !matches!(
        kind,
        K_GENERIC
            | K_ALU_MR
            | K_ALU_MI
            | K_JMP
            | K_CALL
            | K_CALL_M
            | K_CALL_R
            | K_JMP_M
            | K_JMP_R
            | K_INC_M
            | K_DEC_M
            | K_PUSH_M
            | K_POP_M
            | K_LEAVE
            | K_SHIFT_MI
            | K_SHIFT_MCL
            | K_ROT_MI
            | K_ROT_MCL
            | K_INSN
            | K_LOOP
            | K_VCVT_PH..=K_PCMPSTR
    )
}

include!("fast_ext.rs");

#[cfg(test)]
mod fast_tests {
    use super::*;

    /// Allocate a guest-valid buffer aligned to `align` (always inside `[0x1_0000, 0x8000_0000_0000)`).
    struct AlignedBuf {
        ptr: *mut u8,
        layout: std::alloc::Layout,
    }

    impl AlignedBuf {
        fn new(size: usize, align: usize) -> Self {
            let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
            let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
            assert!(!ptr.is_null());
            assert!(super::super::regfile_ext::is_valid_guest_range(ptr as u64, size));
            Self { ptr, layout }
        }
        fn addr(&self) -> u64 {
            self.ptr as u64
        }
        fn as_slice(&self) -> &[u8] {
            unsafe { core::slice::from_raw_parts(self.ptr, self.layout.size()) }
        }
        fn as_mut_slice(&mut self) -> &mut [u8] {
            unsafe { core::slice::from_raw_parts_mut(self.ptr, self.layout.size()) }
        }
    }

    impl Drop for AlignedBuf {
        fn drop(&mut self) {
            unsafe { std::alloc::dealloc(self.ptr, self.layout) }
        }
    }

    fn base_cpu() -> Cpu {
        let mut cpu = Cpu::new_zeroed();
        for i in 0..16 {
            cpu.gpr[i] = 0x1111_2222_3333_4400u64.wrapping_add((i as u64) * 0x0102_0304_0506_0701);
            cpu.last_bits[i] = 64;
            cpu.xmm[i] = 0xDEAD_BEEF_CAFE_BABE_0123_4567_89AB_CDF0u128 ^ ((i as u128) * 0x0F1E_2D3C_4B5A_6978);
            cpu.ymm_hi[i] = 0xA5A5_5A5A_1234_5678_9ABC_DEF0_FEDC_BA98u128 ^ ((i as u128) * 0x1111_2222_3333_4444);
        }
        cpu.cf = true;
        cpu.zf = false;
        cpu.sf = true;
        cpu.of = true;
        cpu.set_pf(false);
        cpu.df = false;
        cpu
    }

    /// Execute `code` once through the `FastOp` path and once through `exec_generic_ins` on cloned
    /// CPUs + memory buffers (using `rdi`/`rsi`/`rbx`/`rdx` mapped to `buf_a`/`buf_b` as configured
    /// by `setup`), verifying every lowered op is non-`K_GENERIC` and asserting bit-exact equality
    /// of all GPRs, `last_bits`, RFLAGS, XMM/YMM registers, and memory contents.
    fn diff_seq(
        label: &str,
        code: &[u8],
        buf_size: usize,
        align: usize,
        mut setup: impl FnMut(&mut Cpu, &mut AlignedBuf, &mut AlignedBuf),
    ) {
        let mut buf_f_a = AlignedBuf::new(buf_size, align);
        let mut buf_f_b = AlignedBuf::new(buf_size, align);
        let mut buf_g_a = AlignedBuf::new(buf_size, align);
        let mut buf_g_b = AlignedBuf::new(buf_size, align);

        let mut cpu_f = base_cpu();
        let mut cpu_g = base_cpu();
        setup(&mut cpu_f, &mut buf_f_a, &mut buf_f_b);
        setup(&mut cpu_g, &mut buf_g_a, &mut buf_g_b);

        // Decode with a stable guest IP in `code`.
        let mut dec = iced_x86::Decoder::with_ip(64, code, 0x1000_0000, iced_x86::DecoderOptions::NONE);
        let mut insns = Vec::new();
        let mut ops = Vec::new();
        while dec.can_decode() {
            let ins = dec.decode();
            assert!(!ins.is_invalid(), "{label}: invalid test encoding");
            let op = lower(&ins);
            assert_ne!(op.kind, K_GENERIC, "{label}: {:?} fell back to K_GENERIC", ins.mnemonic());
            insns.push(ins);
            ops.push(op);
        }
        fuse(&mut ops);

        unsafe {
            for (ins, op) in insns.iter().zip(ops.iter()) {
                cpu_f.rip = op.ip.wrapping_add(op.len as u64);
                let _ = exec(&mut cpu_f, op);
                cpu_g.rip = ins.next_ip();
                let handled = cpu_g.exec_generic_ins(ins);
                assert!(handled, "{label}: generic handler did not handle {:?}", ins.mnemonic());
            }
        }

        // Normalize buffer base pointers in GPRs before comparing (since fast & generic use separate buffers).
        // Check exact [base, base + sz) first so adjacent heap allocations never alias into the +-64 slack window.
        let norm_gpr = |v: u64, ba: u64, bb: u64| -> u64 {
            let sz = buf_size as u64;
            if v >= ba && v < ba.wrapping_add(sz) {
                0xAAAA_0000_0000_0000u64.wrapping_add(v.wrapping_sub(ba))
            } else if v >= bb && v < bb.wrapping_add(sz) {
                0xBBBB_0000_0000_0000u64.wrapping_add(v.wrapping_sub(bb))
            } else if v >= ba.wrapping_sub(64) && v <= ba.wrapping_add(sz + 64) {
                0xAAAA_0000_0000_0000u64.wrapping_add(v.wrapping_sub(ba))
            } else if v >= bb.wrapping_sub(64) && v <= bb.wrapping_add(sz + 64) {
                0xBBBB_0000_0000_0000u64.wrapping_add(v.wrapping_sub(bb))
            } else {
                v
            }
        };
        for i in 0..16 {
            let gf = norm_gpr(cpu_f.gpr[i], buf_f_a.addr(), buf_f_b.addr());
            let gg = norm_gpr(cpu_g.gpr[i], buf_g_a.addr(), buf_g_b.addr());
            assert_eq!(gf, gg, "{label}: gpr[{i}] mismatch: fast={gf:#x} gen={gg:#x}");
            assert_eq!(
                cpu_f.last_bits[i], cpu_g.last_bits[i],
                "{label}: last_bits[{i}] mismatch: fast={} gen={}",
                cpu_f.last_bits[i], cpu_g.last_bits[i]
            );
        }
        assert_eq!(cpu_f.cf, cpu_g.cf, "{label}: CF mismatch");
        assert_eq!(cpu_f.zf, cpu_g.zf, "{label}: ZF mismatch");
        assert_eq!(cpu_f.sf, cpu_g.sf, "{label}: SF mismatch");
        assert_eq!(cpu_f.of, cpu_g.of, "{label}: OF mismatch");
        assert_eq!(cpu_f.pf(), cpu_g.pf(), "{label}: PF mismatch");
        assert_eq!(cpu_f.df, cpu_g.df, "{label}: DF mismatch");
        for i in 0..16 {
            assert_eq!(
                cpu_f.xmm[i], cpu_g.xmm[i],
                "{label}: xmm[{i}] mismatch: fast={:#034x} gen={:#034x}",
                cpu_f.xmm[i], cpu_g.xmm[i]
            );
            assert_eq!(
                cpu_f.ymm_hi[i], cpu_g.ymm_hi[i],
                "{label}: ymm_hi[{i}] mismatch: fast={:#034x} gen={:#034x}",
                cpu_f.ymm_hi[i], cpu_g.ymm_hi[i]
            );
        }
        assert_eq!(buf_f_a.as_slice(), buf_g_a.as_slice(), "{label}: buf_a memory mismatch");
        assert_eq!(buf_f_b.as_slice(), buf_g_b.as_slice(), "{label}: buf_b memory mismatch");
    }

    #[test]
    fn fast_struct_size_is_32() {
        assert_eq!(core::mem::size_of::<FastOp>(), 32);
    }

    #[test]
    fn fast_rep_movs_differential() {
        let forms: &[(&str, &[u8], usize)] = &[
            ("movsb", &[0xA4], 1),
            ("rep_movsb", &[0xF3, 0xA4], 1),
            ("rep_movsw", &[0xF3, 0x66, 0xA5], 2),
            ("rep_movsd", &[0xF3, 0xA5], 4),
            ("rep_movsq", &[0xF3, 0x48, 0xA5], 8),
        ];
        for &(name, code, elem) in forms {
            // Disjoint forward & backward, rcx=0, unaligned offsets
            for &df in &[false, true] {
                for &cnt in &[0usize, 1, 3, 17, 65] {
                    for &off in &[0usize, 1, 3, 5] {
                        diff_seq(&format!("{name}_disjoint_df{df}_c{cnt}_o{off}"), code, 1024, 64, |cpu, a, b| {
                            for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                                *byte = (i.wrapping_mul(37).wrapping_add(13)) as u8;
                            }
                            b.as_mut_slice().fill(0xCC);
                            cpu.df = df;
                            cpu.gpr[1] = cnt as u64;
                            let base_off = if df { 600 + off } else { off };
                            cpu.gpr[6] = a.addr() + base_off as u64;
                            cpu.gpr[7] = b.addr() + base_off as u64;
                        });
                    }
                }
            }
            // Overlapping cases within the same buffer (both DF=0 and DF=1, including dist < elem & non-multiples)
            for &(dist, df) in &[(0isize, false), (1, false), (3, false), (4, false), (6, false), (-5, false), (0, true), (1, true), (3, true), (6, true), (-1, true), (-3, true), (-6, true)] {
                let cnt = 24usize;
                diff_seq(&format!("{name}_ovlp_d{dist}_df{df}"), code, 1024, 64, |cpu, a, _| {
                    for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                        *byte = (i.wrapping_mul(53).wrapping_add(7)) as u8;
                    }
                    cpu.df = df;
                    cpu.gpr[1] = cnt as u64;
                    let src_off = if df { 512isize } else { 128isize };
                    let dst_off = src_off + dist;
                    assert!(dst_off >= (cnt * elem) as isize || !df);
                    cpu.gpr[6] = a.addr() + src_off as u64;
                    cpu.gpr[7] = a.addr() + dst_off as u64;
                });
            }
        }
    }

    #[test]
    fn fast_rep_stos_and_lods_differential() {
        let stos_forms: &[(&str, &[u8])] = &[
            ("stosb", &[0xAA]),
            ("rep_stosb", &[0xF3, 0xAA]),
            ("rep_stosw", &[0xF3, 0x66, 0xAB]),
            ("rep_stosd", &[0xF3, 0xAB]),
            ("rep_stosq", &[0xF3, 0x48, 0xAB]),
        ];
        let patterns = [0u64, 0xA5A5_A5A5_A5A5_A5A5, 0x1122_3344_5566_7788];
        for &(name, code) in stos_forms {
            for &pat in &patterns {
                for &df in &[false, true] {
                    for &cnt in &[0usize, 1, 3, 7, 15, 16, 17, 65] {
                        for &off in &[0usize, 1, 3] {
                            diff_seq(&format!("{name}_{pat:#x}_df{df}_c{cnt}_o{off}"), code, 1024, 64, |cpu, a, _| {
                                a.as_mut_slice().fill(0xEE);
                                cpu.df = df;
                                cpu.gpr[0] = pat;
                                cpu.gpr[1] = cnt as u64;
                                cpu.gpr[7] = a.addr() + if df { 700 + off as u64 } else { off as u64 };
                            });
                        }
                    }
                }
            }
        }
        let lods_forms: &[(&str, &[u8])] = &[
            ("lodsb", &[0xAC]),
            ("rep_lodsb", &[0xF3, 0xAC]),
            ("rep_lodsw", &[0xF3, 0x66, 0xAD]),
            ("rep_lodsd", &[0xF3, 0xAD]),
            ("rep_lodsq", &[0xF3, 0x48, 0xAD]),
        ];
        for &(name, code) in lods_forms {
            for &df in &[false, true] {
                for &cnt in &[0usize, 1, 5] {
                    diff_seq(&format!("{name}_df{df}_c{cnt}"), code, 512, 64, |cpu, a, _| {
                        for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                            *byte = (i.wrapping_mul(19).wrapping_add(3)) as u8;
                        }
                        cpu.df = df;
                        cpu.gpr[1] = cnt as u64;
                        cpu.gpr[6] = a.addr() + if df { 256 } else { 5 };
                    });
                }
            }
        }
    }

    #[test]
    fn fast_neon_scasb_differential() {
        // Test repne scasb (F2 AE), repe scasb (F3 AE), single scasb (AE), and repne scasw/d/q.
        let cases: &[(&str, &[u8])] = &[
            ("repne_scasb", &[0xF2, 0xAE]),
            ("repe_scasb", &[0xF3, 0xAE]),
            ("scasb", &[0xAE]),
            ("repne_scasw", &[0xF2, 0x66, 0xAF]),
            ("repe_scasd", &[0xF3, 0xAF]),
            ("repne_scasq", &[0xF2, 0x48, 0xAF]),
        ];
        for &(name, code) in cases {
            for &cnt in &[0usize, 1, 7, 15, 16, 17, 32, 37, 80] {
                for &match_pos in &[None, Some(0usize), Some(7), Some(15), Some(16), Some(31), Some(36)] {
                    for &off in &[0usize, 1, 3, 15] {
                        diff_seq(&format!("{name}_c{cnt}_m{match_pos:?}_o{off}"), code, 512, 64, |cpu, a, _| {
                            let is_repe = name.starts_with("repe_");
                            let fill = if is_repe { 0x41u8 } else { 0x80u8 };
                            a.as_mut_slice().fill(fill);
                            if let Some(pos) = match_pos {
                                if off + pos * 8 + 8 <= 512 {
                                    // Place differing or matching byte/element
                                    let val = if is_repe { 0x01u8 } else { 0x41u8 };
                                    if name.ends_with('b') {
                                        a.as_mut_slice()[off + pos] = val;
                                    } else {
                                        for k in 0..8 {
                                            a.as_mut_slice()[off + pos * 8 + k] = val;
                                        }
                                    }
                                }
                            }
                            cpu.df = false;
                            cpu.gpr[0] = 0x4141_4141_4141_4141;
                            cpu.gpr[1] = cnt as u64;
                            cpu.gpr[7] = a.addr() + off as u64;
                        });
                    }
                }
            }
        }
        // 4 KiB page-boundary straddling test for NEON `repne`/`repe scasb` + DF=1 backward scan.
        for &(name, code, is_repe) in &[("repne_page", &[0xF2u8, 0xAE][..], false), ("repe_page", &[0xF3u8, 0xAE][..], true)] {
            for &start_off in &[4080usize, 4088, 4095, 4096] {
                for &hit_rel in &[None, Some(3usize), Some(8), Some(16), Some(35)] {
                    diff_seq(&format!("{name}_s{start_off}_h{hit_rel:?}"), code, 8192, 4096, |cpu, a, _| {
                        a.as_mut_slice().fill(if is_repe { 0x55 } else { 0xAA });
                        if let Some(rel) = hit_rel {
                            a.as_mut_slice()[start_off + rel] = if is_repe { 0xFF } else { 0x55 };
                        }
                        cpu.df = false;
                        cpu.gpr[0] = 0x55;
                        cpu.gpr[1] = 48;
                        cpu.gpr[7] = a.addr() + start_off as u64;
                    });
                }
            }
            // DF=1 backward scan
            diff_seq(&format!("{name}_df1"), code, 512, 64, |cpu, a, _| {
                a.as_mut_slice().fill(if is_repe { 0x55 } else { 0xAA });
                a.as_mut_slice()[180] = if is_repe { 0x11 } else { 0x55 };
                cpu.df = true;
                cpu.gpr[0] = 0x55;
                cpu.gpr[1] = 40;
                cpu.gpr[7] = a.addr() + 200;
            });
        }
    }

    #[test]
    fn fast_neon_cmpsb_differential() {
        let cases: &[(&str, &[u8], bool)] = &[
            ("repe_cmpsb", &[0xF3, 0xA6], false),
            ("repne_cmpsb", &[0xF2, 0xA6], true),
            ("cmpsb", &[0xA6], false),
            ("repe_cmpsw", &[0xF3, 0x66, 0xA7], false),
            ("repe_cmpsd", &[0xF3, 0xA7], false),
            ("repe_cmpsq", &[0xF3, 0x48, 0xA7], false),
        ];
        for &(name, code, stop_on_eq) in cases {
            for &cnt in &[0usize, 1, 7, 15, 16, 17, 32, 45] {
                for &hit in &[None, Some(0usize), Some(1), Some(15), Some(16), Some(31), Some(40)] {
                    diff_seq(&format!("{name}_c{cnt}_h{hit:?}"), code, 1024, 64, |cpu, a, b| {
                        for i in 0..1024 {
                            let v = ((i * 29 + 11) & 0xFF) as u8;
                            a.as_mut_slice()[i] = v;
                            b.as_mut_slice()[i] = if stop_on_eq { v ^ 0x55 } else { v };
                        }
                        let off_a = 1usize;
                        let off_b = 5usize;
                        // Re-sync relative offsets
                        for i in 0..512 {
                            let v = ((i * 29 + 11) & 0xFF) as u8;
                            a.as_mut_slice()[off_a + i] = v;
                            b.as_mut_slice()[off_b + i] = if stop_on_eq { v.wrapping_add(1) } else { v };
                        }
                        if let Some(pos) = hit {
                            if name.ends_with('b') {
                                b.as_mut_slice()[off_b + pos] = if stop_on_eq {
                                    a.as_slice()[off_a + pos]
                                } else {
                                    a.as_slice()[off_a + pos].wrapping_add(0x7F)
                                };
                            } else if off_b + pos * 8 + 8 <= 1024 {
                                b.as_mut_slice()[off_b + pos * 8] ^= 0x80;
                            }
                        }
                        cpu.df = false;
                        cpu.gpr[1] = cnt as u64;
                        cpu.gpr[6] = a.addr() + off_a as u64;
                        cpu.gpr[7] = b.addr() + off_b as u64;
                    });
                }
            }
        }
        // Page-straddling with mismatched page offsets for `p1` and `p2` + DF=1.
        for &(name, code, stop_on_eq) in &[("repe_cmpsb_page", &[0xF3u8, 0xA6][..], false), ("repne_cmpsb_page", &[0xF2u8, 0xA6][..], true)] {
            for &hit in &[None, Some(4usize), Some(15), Some(16), Some(47)] {
                diff_seq(&format!("{name}_h{hit:?}"), code, 8192, 4096, |cpu, a, b| {
                    let s1 = 4088usize;
                    let s2 = 4092usize;
                    for i in 0..64 {
                        let v = (i * 13 + 5) as u8;
                        a.as_mut_slice()[s1 + i] = v;
                        b.as_mut_slice()[s2 + i] = if stop_on_eq { v ^ 0xFF } else { v };
                    }
                    if let Some(pos) = hit {
                        b.as_mut_slice()[s2 + pos] = if stop_on_eq {
                            a.as_slice()[s1 + pos]
                        } else {
                            a.as_slice()[s1 + pos].wrapping_sub(3)
                        };
                    }
                    cpu.df = false;
                    cpu.gpr[1] = 48;
                    cpu.gpr[6] = a.addr() + s1 as u64;
                    cpu.gpr[7] = b.addr() + s2 as u64;
                });
            }
            diff_seq(&format!("{name}_df1"), code, 512, 64, |cpu, a, b| {
                a.as_mut_slice().fill(0x22);
                b.as_mut_slice().fill(if stop_on_eq { 0x33 } else { 0x22 });
                b.as_mut_slice()[190] = if stop_on_eq { 0x22 } else { 0x99 };
                cpu.df = true;
                cpu.gpr[1] = 30;
                cpu.gpr[6] = a.addr() + 200;
                cpu.gpr[7] = b.addr() + 200;
            });
        }
    }

    #[test]
    fn fast_neon_simd_vshuf_vint_differential() {
        // Test edge cases: pabs*(MIN), pack* saturation, pshufb bits 6..4, pmulhrsw(-32768*-32768),
        // pmulhw/huw/udq/dq/maddwd, phadd/phsub, psadbw, pmovmskb/vpmovmskb/movmskps, pclmulqdq,
        // pmovsx/zx, vblendv*, and VEX.128 upper-lane zeroing vs legacy SSE preservation!
        let insns: &[(&str, &[u8])] = &[
            ("pabsb_leg", &[0x66, 0x0F, 0x38, 0x1C, 0xC1]),               // pabsb xmm0, xmm1
            ("vpabsb_128", &[0xC4, 0xE2, 0x79, 0x1C, 0xC1]),              // vpabsb xmm0, xmm1
            ("vpabsb_256", &[0xC4, 0xE2, 0x7D, 0x1C, 0xC1]),              // vpabsb ymm0, ymm1
            ("pabsw_leg", &[0x66, 0x0F, 0x38, 0x1D, 0xC1]),               // pabsw xmm0, xmm1
            ("pabsd_leg", &[0x66, 0x0F, 0x38, 0x1E, 0xC1]),               // pabsd xmm0, xmm1
            ("packsswb", &[0x66, 0x0F, 0x63, 0xC1]),                      // packsswb xmm0, xmm1
            ("packuswb", &[0x66, 0x0F, 0x67, 0xC1]),                      // packuswb xmm0, xmm1
            ("packssdw", &[0x66, 0x0F, 0x6B, 0xC1]),                      // packssdw xmm0, xmm1
            ("packusdw", &[0x66, 0x0F, 0x38, 0x2B, 0xC1]),                // packusdw xmm0, xmm1
            ("vpackuswb_128", &[0xC5, 0xF9, 0x67, 0xC2]),                 // vpackuswb xmm0, xmm0, xmm2
            ("pshufb", &[0x66, 0x0F, 0x38, 0x00, 0xC2]),                  // pshufb xmm0, xmm2
            ("vpshufb_128", &[0xC4, 0xE2, 0x79, 0x00, 0xC2]),             // vpshufb xmm0, xmm0, xmm2
            ("vpshufb_256", &[0xC4, 0xE2, 0x7D, 0x00, 0xC2]),             // vpshufb ymm0, ymm0, ymm2
            ("pmulhrsw", &[0x66, 0x0F, 0x38, 0x0B, 0xC1]),                // pmulhrsw xmm0, xmm1
            ("pmulhw", &[0x66, 0x0F, 0xE5, 0xC1]),                        // pmulhw xmm0, xmm1
            ("pmulhuw", &[0x66, 0x0F, 0xE4, 0xC1]),                       // pmulhuw xmm0, xmm1
            ("pmuludq", &[0x66, 0x0F, 0xF4, 0xC1]),                       // pmuludq xmm0, xmm1
            ("pmuldq", &[0x66, 0x0F, 0x38, 0x28, 0xC1]),                  // pmuldq xmm0, xmm1
            ("pmaddwd", &[0x66, 0x0F, 0xF5, 0xC1]),                       // pmaddwd xmm0, xmm1
            ("phaddw", &[0x66, 0x0F, 0x38, 0x01, 0xC1]),                  // phaddw xmm0, xmm1
            ("phaddd", &[0x66, 0x0F, 0x38, 0x02, 0xC1]),                  // phaddd xmm0, xmm1
            ("phsubw", &[0x66, 0x0F, 0x38, 0x05, 0xC1]),                  // phsubw xmm0, xmm1
            ("phsubd", &[0x66, 0x0F, 0x38, 0x06, 0xC1]),                  // phsubd xmm0, xmm1
            ("psadbw", &[0x66, 0x0F, 0xF6, 0xC1]),                        // psadbw xmm0, xmm1
            ("pavgb", &[0x66, 0x0F, 0xE0, 0xC1]),                         // pavgb xmm0, xmm1
            ("pavgw", &[0x66, 0x0F, 0xE3, 0xC1]),                         // pavgw xmm0, xmm1
            ("pcmpeqb", &[0x66, 0x0F, 0x74, 0xC1]),                       // pcmpeqb xmm0, xmm1
            ("vpcmpeqb_128", &[0xC5, 0xF9, 0x74, 0xC1]),                  // vpcmpeqb xmm0, xmm0, xmm1
            ("pcmpgtb", &[0x66, 0x0F, 0x64, 0xC1]),                       // pcmpgtb xmm0, xmm1
            ("pcmpgtq", &[0x66, 0x0F, 0x38, 0x37, 0xC1]),                 // pcmpgtq xmm0, xmm1
            ("pmovmskb", &[0x66, 0x0F, 0xD7, 0xC1]),                      // pmovmskb eax, xmm1
            ("vpmovmskb_256", &[0xC5, 0xFD, 0xD7, 0xC1]),                 // vpmovmskb eax, ymm1
            ("movmskps", &[0x0F, 0x50, 0xC1]),                            // movmskps eax, xmm1
            ("vmovmskps_256", &[0xC5, 0xFC, 0x50, 0xC1]),                 // vmovmskps eax, ymm1
            ("pclmulqdq_00", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x00]),      // pclmulqdq xmm0, xmm1, 0x00
            ("pclmulqdq_01", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x01]),      // pclmulqdq xmm0, xmm1, 0x01
            ("pclmulqdq_10", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x10]),      // pclmulqdq xmm0, xmm1, 0x10
            ("vpclmulqdq_11", &[0xC4, 0xE3, 0x79, 0x44, 0xC1, 0x11]),     // vpclmulqdq xmm0, xmm0, xmm1, 0x11
            ("pmovsxbw", &[0x66, 0x0F, 0x38, 0x20, 0xC1]),                // pmovsxbw xmm0, xmm1
            ("vpmovzxbw_256", &[0xC4, 0xE2, 0x7D, 0x30, 0xC1]),           // vpmovzxbw ymm0, xmm1
            ("vpmovsxbd_256", &[0xC4, 0xE2, 0x7D, 0x21, 0xC1]),           // vpmovsxbd ymm0, xmm1
            ("vpmovzxbq_256", &[0xC4, 0xE2, 0x7D, 0x32, 0xC1]),           // vpmovzxbq ymm0, xmm1
            ("vpmovsxwd_256", &[0xC4, 0xE2, 0x7D, 0x23, 0xC1]),           // vpmovsxwd ymm0, xmm1
            ("vpmovzxwq_256", &[0xC4, 0xE2, 0x7D, 0x34, 0xC1]),           // vpmovzxwq ymm0, xmm1
            ("vpmovsxdq_256", &[0xC4, 0xE2, 0x7D, 0x25, 0xC1]),           // vpmovsxdq ymm0, xmm1
            ("vpmovzxdq_256", &[0xC4, 0xE2, 0x7D, 0x35, 0xC1]),           // vpmovzxdq ymm0, xmm1
            ("pblendvb", &[0x66, 0x0F, 0x38, 0x10, 0xC1]),                // pblendvb xmm0, xmm1, xmm0
            ("vpblendvb_256", &[0xC4, 0xE3, 0x7D, 0x4C, 0xC1, 0x20]),     // vpblendvb ymm0, ymm0, ymm1, ymm2
            ("vblendvps_256", &[0xC4, 0xE3, 0x7D, 0x4A, 0xC1, 0x20]),     // vblendvps ymm0, ymm0, ymm1, ymm2
            ("vblendvpd_256", &[0xC4, 0xE3, 0x7D, 0x4B, 0xC1, 0x20]),     // vblendvpd ymm0, ymm0, ymm1, ymm2
            ("punpcklbw", &[0x66, 0x0F, 0x60, 0xC1]),                     // punpcklbw xmm0, xmm1
            ("punpckhwd", &[0x66, 0x0F, 0x69, 0xC1]),                     // punpckhwd xmm0, xmm1
            ("punpckldq", &[0x66, 0x0F, 0x62, 0xC1]),                     // punpckldq xmm0, xmm1
            ("punpckhdq", &[0x66, 0x0F, 0x6A, 0xC1]),                     // punpckhdq xmm0, xmm1
        ];
        for &(name, code) in insns {
            diff_seq(name, code, 128, 16, |cpu, _, _| {
                // Edge-case lanes: i8::MIN, i16::MIN, i32::MIN, saturation boundaries, control bytes with bits 6..4 set!
                cpu.xmm[0] = u128::from_le_bytes([
                    0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0x00, 0x01, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00, 0xFE, 0xFF,
                ]);
                cpu.xmm[1] = u128::from_le_bytes([
                    0x00, 0x80, 0x80, 0x80, 0x00, 0x40, 0x01, 0x40, 0x00, 0x80, 0x7F, 0xFF, 0x00, 0x01, 0x80, 0x7F,
                ]);
                // pshufb control bytes with bits 6..4 set (`0x31`, `0x7F`, `0x52`, `0x80`, `0xFF`)
                cpu.xmm[2] = u128::from_le_bytes([
                    0x00, 0x31, 0x7F, 0x80, 0x52, 0x63, 0xFF, 0x90, 0x1E, 0x2D, 0x4C, 0x85, 0x0A, 0x70, 0x81, 0x0F,
                ]);
                cpu.ymm_hi[0] = 0x8000_7FFF_FFFF_0001_8080_7F7F_0100_FFFE;
                cpu.ymm_hi[1] = 0x8000_8000_0001_FFFF_4001_4000_8000_7FFF;
                cpu.ymm_hi[2] = 0xFF80_7F31_6352_9000_4C2D_1E85_0F81_700A;
            });
        }
    }

    #[test]
    fn fast_neon_vfp_and_fma_differential() {
        let insns: &[(&str, &[u8])] = &[
            ("addps", &[0x0F, 0x58, 0xC1]),
            ("subps", &[0x0F, 0x5C, 0xC1]),
            ("mulps", &[0x0F, 0x59, 0xC1]),
            ("divps", &[0x0F, 0x5E, 0xC1]),
            ("minps", &[0x0F, 0x5D, 0xC1]),
            ("maxps", &[0x0F, 0x5F, 0xC1]),
            ("sqrtps", &[0x0F, 0x51, 0xC1]),
            ("addpd", &[0x66, 0x0F, 0x58, 0xC1]),
            ("subpd", &[0x66, 0x0F, 0x5C, 0xC1]),
            ("mulpd", &[0x66, 0x0F, 0x59, 0xC1]),
            ("divpd", &[0x66, 0x0F, 0x5E, 0xC1]),
            ("minpd", &[0x66, 0x0F, 0x5D, 0xC1]),
            ("maxpd", &[0x66, 0x0F, 0x5F, 0xC1]),
            ("sqrtpd", &[0x66, 0x0F, 0x51, 0xC1]),
            ("cvtdq2ps", &[0x0F, 0x5B, 0xC1]),
            ("roundps_0", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x00]),
            ("roundps_1", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x01]),
            ("roundps_2", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x02]),
            ("roundps_3", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x03]),
            ("roundpd_0", &[0x66, 0x0F, 0x3A, 0x09, 0xC1, 0x00]),
            ("haddps", &[0xF2, 0x0F, 0x7C, 0xC1]),
            ("hsubps", &[0xF2, 0x0F, 0x7D, 0xC1]),
            ("vaddps_128", &[0xC5, 0xF8, 0x58, 0xC1]),
            ("vfmadd213ps_256", &[0xC4, 0xE2, 0x75, 0xA8, 0xC2]),
            ("vfmsub213ps_256", &[0xC4, 0xE2, 0x75, 0xAA, 0xC2]),
            ("vfnmadd213ps_256", &[0xC4, 0xE2, 0x75, 0xAC, 0xC2]),
            ("vfnmsub213ps_256", &[0xC4, 0xE2, 0x75, 0xAE, 0xC2]),
            ("vfmaddsub213ps_256", &[0xC4, 0xE2, 0x75, 0xA6, 0xC2]),
            ("vfmsubadd213ps_256", &[0xC4, 0xE2, 0x75, 0xA7, 0xC2]),
            ("vfmadd213pd_256", &[0xC4, 0xE2, 0xF5, 0xA8, 0xC2]),
            ("vfmaddsub213pd_256", &[0xC4, 0xE2, 0xF5, 0xA6, 0xC2]),
        ];
        // Test 1: finite normal/negative/zero/half values
        let f32v = |arr: [f32; 4]| -> u128 { pk::<u32, 4>(arr.map(f32::to_bits)) };
        let f64v = |arr: [f64; 2]| -> u128 { pk::<u64, 2>(arr.map(f64::to_bits)) };
        let datasets: &[(&str, u128, u128, u128)] = &[
            ("finite", f32v([1.5, -2.5, 0.0, -0.0]), f32v([-0.0, 0.0, 3.25, -4.75]), f32v([0.5, -1.5, 2.5, -3.5])),
            (
                "inf_nan",
                pk::<u32, 4>([f32::INFINITY.to_bits(), 0x7FC0_0011, 0x7F80_0005, (-1.0f32).to_bits()]),
                pk::<u32, 4>([f32::INFINITY.to_bits(), 0x7FC0_0022, 1.0f32.to_bits(), (-0.0f32).to_bits()]),
                f32v([1.0, 2.0, -3.0, 4.0]),
            ),
            (
                "f64_special",
                f64v([0.0, f64::NEG_INFINITY]),
                pk::<u64, 2>([(-0.0f64).to_bits(), 0x7FF8_0000_0000_00AA]),
                f64v([2.5, -3.5]),
            ),
        ];
        for &(dname, a, b, c) in datasets {
            for &(name, code) in insns {
                diff_seq(&format!("{name}_{dname}"), code, 128, 16, |cpu, _, _| {
                    cpu.xmm[0] = a;
                    cpu.xmm[1] = b;
                    cpu.xmm[2] = c;
                    cpu.ymm_hi[0] = b;
                    cpu.ymm_hi[1] = c;
                    cpu.ymm_hi[2] = a;
                });
            }
        }
    }

    #[test]
    fn fast_aes_f16c_vmaskmov_crc32_differential() {
        let insns: &[(&str, &[u8])] = &[
            // Legacy AES (must preserve ymm_hi[0])
            ("aesenc", &[0x66, 0x0F, 0x38, 0xDC, 0xC1]),
            ("aesenclast", &[0x66, 0x0F, 0x38, 0xDD, 0xC1]),
            ("aesdec", &[0x66, 0x0F, 0x38, 0xDE, 0xC1]),
            ("aesdeclast", &[0x66, 0x0F, 0x38, 0xDF, 0xC1]),
            ("aesimc", &[0x66, 0x0F, 0x38, 0xDB, 0xC1]),
            ("aeskeygenassist", &[0x66, 0x0F, 0x3A, 0xDF, 0xC1, 0x36]),
            // VEX AES (must zero ymm_hi[0]) + memory form
            ("vaesenc", &[0xC4, 0xE2, 0x71, 0xDC, 0xC2]),              // vaesenc xmm0, xmm1, xmm2
            ("vaesenc_m", &[0xC4, 0xE2, 0x71, 0xDC, 0x07]),            // vaesenc xmm0, xmm1, [rdi]
            ("vaesenclast", &[0xC4, 0xE2, 0x71, 0xDD, 0xC2]),
            ("vaesdec", &[0xC4, 0xE2, 0x71, 0xDE, 0xC2]),
            ("vaesdeclast", &[0xC4, 0xE2, 0x71, 0xDF, 0xC2]),
            ("vaesimc", &[0xC4, 0xE2, 0x79, 0xDB, 0xC1]),
            ("vaeskeygenassist", &[0xC4, 0xE3, 0x79, 0xDF, 0xC1, 0x1B]),
            // F16C
            ("vcvtph2ps_128", &[0xC4, 0xE2, 0x79, 0x13, 0xC1]),
            ("vcvtph2ps_256", &[0xC4, 0xE2, 0x7D, 0x13, 0xC1]),
            ("vcvtph2ps_128_m", &[0xC4, 0xE2, 0x79, 0x13, 0x07]),
            ("vcvtps2ph_128", &[0xC4, 0xE3, 0x79, 0x1D, 0xC8, 0x00]),  // vcvtps2ph xmm0, xmm1, 0
            ("vcvtps2ph_256", &[0xC4, 0xE3, 0x7D, 0x1D, 0xC8, 0x00]),  // vcvtps2ph xmm0, ymm1, 0
            ("vcvtps2ph_256_m", &[0xC4, 0xE3, 0x7D, 0x1D, 0x0F, 0x00]), // vcvtps2ph [rdi], ymm1, 0
            // VMASKMOV (loads & stores, 128 & 256)
            ("vpmaskmovd_ld_128", &[0xC4, 0xE2, 0x71, 0x8C, 0x07]),    // vpmaskmovd xmm0, xmm1, [rdi]
            ("vpmaskmovd_ld_256", &[0xC4, 0xE2, 0x75, 0x8C, 0x07]),    // vpmaskmovd ymm0, ymm1, [rdi]
            ("vpmaskmovq_st_256", &[0xC4, 0xE2, 0xF5, 0x8E, 0x17]),    // vpmaskmovq [rdi], ymm1, ymm2
            // CRC32 (8/16/32/64-bit src, reg & mem)
            ("crc32_r32_r8", &[0xF2, 0x0F, 0x38, 0xF0, 0xC3]),         // crc32 eax, bl
            ("crc32_r32_r16", &[0x66, 0xF2, 0x0F, 0x38, 0xF1, 0xC3]),  // crc32 eax, bx
            ("crc32_r32_r32", &[0xF2, 0x0F, 0x38, 0xF1, 0xC3]),        // crc32 eax, ebx
            ("crc32_r64_r64", &[0xF2, 0x48, 0x0F, 0x38, 0xF1, 0xC3]),  // crc32 rax, rbx
            ("crc32_r64_m64", &[0xF2, 0x48, 0x0F, 0x38, 0xF1, 0x07]),  // crc32 rax, [rdi]
        ];
        for &(name, code) in insns {
            diff_seq(name, code, 256, 32, |cpu, a, _| {
                for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                    *byte = (i.wrapping_mul(41).wrapping_add(9)) as u8;
                }
                cpu.gpr[7] = a.addr() + 3; // unaligned!
                // Half-float test values: +0, -0, 1.0 (0x3C00), subnormal (0x0001), -Inf (0xFC00), qNaN (0x7E01), 65504 (0x7BFF), -2.5 (0xC100)
                cpu.xmm[1] = u128::from_le_bytes([
                    0x00, 0x00, 0x00, 0x80, 0x00, 0x3C, 0x01, 0x00, 0x00, 0xFC, 0x01, 0x7E, 0xFF, 0x7B, 0x00, 0xC1,
                ]);
                cpu.ymm_hi[1] = 0x8000_0000_0000_0000_8000_0000_0000_0000;
            });
        }
    }

    #[test]
    fn fast_pcmpstr_differential() {
        // Test all 4 agg modes (0..4), unsigned/signed bytes & words, polarities (0x00, 0x10, 0x30),
        // imm bit 6 (0/1), implicit & explicit, index & mask!
        let imms: &[u8] = &[
            0x00, 0x02, 0x04, 0x06, 0x08, 0x0C, 0x10, 0x18, 0x38, 0x40, 0x48, 0x4C, 0x58, 0x6E, 0x78, // bytes
            0x01, 0x05, 0x07, 0x09, 0x0D, 0x19, 0x39, 0x49, 0x6F, 0x79, // words
        ];
        for &imm in imms {
            let forms: &[(&str, [u8; 6])] = &[
                ("pcmpistri", [0x66, 0x0F, 0x3A, 0x63, 0xC1, imm]),
                ("pcmpistrm", [0x66, 0x0F, 0x3A, 0x62, 0xC1, imm]),
                ("pcmpestri", [0x66, 0x0F, 0x3A, 0x61, 0xC1, imm]),
                ("pcmpestrm", [0x66, 0x0F, 0x3A, 0x60, 0xC1, imm]),
            ];
            for &(name, code) in forms {
                diff_seq(&format!("{name}_{imm:#04x}"), &code, 128, 16, |cpu, _, _| {
                    cpu.gpr[0] = 5;  // eax = explicit len a
                    cpu.gpr[2] = 11; // edx = explicit len b
                    cpu.xmm[0] = u128::from_le_bytes(*b"a\x7F\x80\xFEz\0bcdefghijk");
                    cpu.xmm[1] = u128::from_le_bytes(*b"hello a\x7F\x80w\0xyz12");
                });
                // Empty needle (`a_len == 0`) edge case (especially Equal Ordered with polarity/MSB)
                diff_seq(&format!("{name}_empty_{imm:#04x}"), &code, 128, 16, |cpu, _, _| {
                    cpu.gpr[0] = 0;
                    cpu.gpr[2] = 0;
                    cpu.xmm[0] = 0;
                    cpu.xmm[1] = 0;
                });
            }
        }
    }

    #[test]
    fn fast_hi8_sys_misc_bit_ext_atomic_differential() {
        let cases: &[(&str, &[u8])] = &[
            // K_HI8
            ("hi8_mov_ri", &[0xB4, 0x7A]),                         // mov ah, 0x7A
            ("hi8_mov_rr", &[0x88, 0xFC]),                         // mov ah, bh
            ("hi8_mov_rm", &[0x8A, 0x27]),                         // mov ah, [rdi]
            ("hi8_mov_mr", &[0x88, 0x27]),                         // mov [rdi], ah
            ("hi8_movzx", &[0x0F, 0xB6, 0xC4]),                    // movzx eax, ah
            ("hi8_movsx", &[0x0F, 0xBE, 0xC4]),                    // movsx eax, ah
            ("hi8_add_rr", &[0x00, 0xFC]),                         // add ah, bh
            ("hi8_sub_ri", &[0x80, 0xEC, 0x55]),                   // sub ah, 0x55
            ("hi8_cmp_rm", &[0x3A, 0x27]),                         // cmp ah, [rdi]
            ("hi8_test_mr", &[0x84, 0x27]),                        // test [rdi], ah
            ("hi8_inc", &[0xFE, 0xC4]),                            // inc ah
            ("hi8_dec", &[0xFE, 0xCC]),                            // dec ah
            ("hi8_neg", &[0xF6, 0xDC]),                            // neg ah
            ("hi8_not", &[0xF6, 0xD4]),                            // not ah
            ("hi8_shl", &[0xC0, 0xE4, 0x03]),                      // shl ah, 3
            ("hi8_sar", &[0xC0, 0xFC, 0x02]),                      // sar ah, 2
            ("hi8_rol", &[0xC0, 0xC4, 0x05]),                      // rol ah, 5
            ("hi8_ror", &[0xC0, 0xCC, 0x03]),                      // ror ah, 3
            ("hi8_setc", &[0x0F, 0x92, 0xC4]),                     // setc ah
            ("hi8_xchg", &[0x86, 0xE7]),                           // xchg ah, bh
            ("hi8_xadd", &[0x0F, 0xC0, 0xFC]),                     // xadd ah, bh
            ("hi8_cmpxchg", &[0x0F, 0xB0, 0xFC]),                  // cmpxchg ah, bh
            // K_SYS_MISC
            ("lahf", &[0x9F]),
            ("sahf", &[0x9E]),
            ("pushfq_popfq", &[0x9C, 0xF8, 0x9D]),                 // pushfq; clc; popfq
            ("pushf_popf", &[0x66, 0x9C, 0xF9, 0x66, 0x9D]),       // pushf; stc; popf
            ("stmxcsr", &[0x0F, 0xAE, 0x1F]),                      // stmxcsr [rdi]
            ("fnstcw", &[0x9B, 0xD9, 0x3F]),                       // fnstcw [rdi]
            ("xlatb", &[0xD7]),
            ("xgetbv", &[0x0F, 0x01, 0xD0]),
            ("rdtsc", &[0x0F, 0x31]),
            ("rdtscp", &[0x0F, 0x01, 0xF9]),
            ("cpuid_0", &[0x31, 0xC0, 0x0F, 0xA2]),
            ("cpuid_1", &[0xB8, 0x01, 0x00, 0x00, 0x00, 0x0F, 0xA2]),
            ("cpuid_7", &[0xB8, 0x07, 0x00, 0x00, 0x00, 0x0F, 0xA2]),
            ("push16_pop16", &[0x66, 0x50, 0x66, 0x5B]),           // push ax; pop bx
            // K_BIT_EXT
            ("pdep_64", &[0xC4, 0xE2, 0xF3, 0xF5, 0xC3]),          // pdep rax, rcx, rbx
            ("pext_64", &[0xC4, 0xE2, 0xF2, 0xF5, 0xC3]),          // pext rax, rcx, rbx
            ("pdep_32_m", &[0xC4, 0xE2, 0x73, 0xF5, 0x07]),        // pdep eax, ecx, [rdi]
            ("rcl_64_i", &[0x48, 0xC1, 0xD0, 0x07]),               // rcl rax, 7
            ("rcr_8_i", &[0xC0, 0xD8, 0x0B]),                      // rcr al, 11 (tests % 9!)
            ("rcl_32_m", &[0xD3, 0x17]),                           // rcl dword ptr [rdi], cl
            // K_ATOMIC
            ("lock_add", &[0xF0, 0x48, 0x01, 0x07]),               // lock add [rdi], rax
            ("lock_sub", &[0xF0, 0x48, 0x29, 0x07]),               // lock sub [rdi], rax
            ("lock_and", &[0xF0, 0x48, 0x21, 0x07]),               // lock and [rdi], rax
            ("lock_or", &[0xF0, 0x48, 0x09, 0x07]),                // lock or [rdi], rax
            ("lock_xor", &[0xF0, 0x48, 0x31, 0x07]),               // lock xor [rdi], rax
            ("lock_inc", &[0xF0, 0x48, 0xFF, 0x07]),               // lock inc qword ptr [rdi]
            ("lock_dec", &[0xF0, 0x48, 0xFF, 0x0F]),               // lock dec qword ptr [rdi]
            ("lock_not", &[0xF0, 0x48, 0xF7, 0x17]),               // lock not qword ptr [rdi]
            ("lock_neg", &[0xF0, 0x48, 0xF7, 0x1F]),               // lock neg qword ptr [rdi]
            ("xchg_mr", &[0x48, 0x87, 0x07]),                      // xchg [rdi], rax
            ("lock_xadd", &[0xF0, 0x48, 0x0F, 0xC1, 0x07]),        // lock xadd [rdi], rax
            ("lock_cmpxchg_ok", &[0xF0, 0x48, 0x0F, 0xB1, 0x0F]),  // lock cmpxchg [rdi], rcx
            ("lock_cmpxchg8b", &[0xF0, 0x0F, 0xC7, 0x0F]),         // lock cmpxchg8b [rdi]
            ("lock_cmpxchg16b", &[0xF0, 0x48, 0x0F, 0xC7, 0x0F]),  // lock cmpxchg16b [rdi]
            ("bt_m_imm", &[0x48, 0x0F, 0xBA, 0x27, 0x25]),         // bt qword ptr [rdi], 37
            ("bts_m_neg", &[0x48, 0x0F, 0xAB, 0x17]),              // bts qword ptr [rdi], rdx (rdx = -19!)
            ("btr_m_pos", &[0x48, 0x0F, 0xB3, 0x0F]),              // btr qword ptr [rdi], rcx
            ("btc_m_neg", &[0x48, 0x0F, 0xBB, 0x17]),              // btc qword ptr [rdi], rdx
        ];
        for &(name, code) in cases {
            diff_seq(name, code, 512, 64, |cpu, a, b| {
                for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                    *byte = (i.wrapping_mul(17).wrapping_add(5)) as u8;
                }
                for (i, byte) in b.as_mut_slice().iter_mut().enumerate() {
                    *byte = (i.wrapping_mul(23).wrapping_add(1)) as u8;
                }
                cpu.gpr[4] = b.addr() + 256; // rsp
                cpu.gpr[7] = a.addr() + 64;  // rdi (16-byte aligned, with room below for negative `bts`!)
                if name == "xlatb" {
                    cpu.gpr[3] = a.addr();   // rbx = table base
                    cpu.gpr[0] = 0x1234_002A;
                } else if name == "lock_cmpxchg_ok" {
                    let cur = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                    cpu.gpr[0] = cur;
                } else if name == "lock_cmpxchg8b" {
                    let cur = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                    cpu.gpr[0] = cur as u32 as u64;
                    cpu.gpr[2] = cur >> 32;
                } else if name == "lock_cmpxchg16b" {
                    let lo = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                    let hi = u64::from_le_bytes(a.as_slice()[72..80].try_into().unwrap());
                    cpu.gpr[0] = lo;
                    cpu.gpr[2] = hi;
                } else if name == "bts_m_neg" || name == "btc_m_neg" {
                    cpu.gpr[2] = (-19i64) as u64;
                } else if name == "btr_m_pos" {
                    cpu.gpr[1] = 73;
                } else if name == "rcl_32_m" {
                    cpu.gpr[1] = 5;
                }
            });
        }
    }

    #[test]
    fn fast_cmpj_fusion_and_run_chain_end_to_end() {
        // End-to-end guest function exercising fused `cmp`/`test` + `jcc` (RR, RI, RM, taken &
        // fall-through chains) plus a 200-iteration loop, verified against `TEST_FAST_OFF = true`.
        //
        // Args (Win64): rcx = n (200), rdx = ptr to u64 threshold (50)
        //   0x00: xor eax, eax              ; 31 C0
        //   0x02: test rcx, rcx             ; 48 85 C9  (K_CMPJ_RR, not taken)
        //   0x05: jz done (+0x1E -> 0x25)   ; 74 1E
        // loop (0x07):
        //   0x07: cmp rcx, [rdx]            ; 48 3B 0A  (K_CMPJ_RM)
        //   0x0A: jb below (+0x04 -> 0x10)  ; 72 04
        //   0x0C: add rax, 2                ; 48 83 C0 02
        // below (0x10):
        //   0x10: test cl, 1                ; F6 C1 01  (K_CMPJ_RI)
        //   0x13: jz even (+0x04 -> 0x19)   ; 74 04
        //   0x15: add rax, rcx              ; 48 01 C8
        // even (0x18? wait 0x15+3=0x18): fix jz +0x03 -> 0x18
        //   0x18: dec rcx                   ; 48 FF C9
        //   0x1B: cmp rcx, 0                ; 48 83 F9 00 (K_CMPJ_RI)
        //   0x1F: jne loop (-0x1A -> 0x07)  ; 75 E6
        // done (0x21):
        //   0x21: ret                       ; C3
        #[rustfmt::skip]
        let code: [u8; 34] = [
            0x31, 0xC0,
            0x48, 0x85, 0xC9,
            0x74, 0x1A,
            // 0x07: loop
            0x48, 0x3B, 0x0A,
            0x72, 0x04,
            0x48, 0x83, 0xC0, 0x02,
            // 0x10: below
            0xF6, 0xC1, 0x01,
            0x74, 0x03,
            0x48, 0x01, 0xC8,
            // 0x18: even
            0x48, 0xFF, 0xC9,
            0x48, 0x83, 0xF9, 0x00,
            0x75, 0xE6,
            // 0x21: done
            0xC3,
        ];
        let mut code_buf = AlignedBuf::new(64, 16);
        code_buf.as_mut_slice()[..code.len()].copy_from_slice(&code);
        let mut thresh = AlignedBuf::new(16, 16);
        thresh.as_mut_slice()[..8].copy_from_slice(&50u64.to_le_bytes());

        let run_mode = |fast_on: bool| -> u64 {
            let prev = super::super::TEST_FAST_OFF.with(|f| f.replace(!fast_on));
            super::super::jit::force(2);
            let (res, _) = unsafe {
                super::super::emu_run_full(code_buf.addr(), [200, thresh.addr(), 0, 0, 0, 0, 0, 0], [0; 4], [0; 8])
            };
            super::super::TEST_FAST_OFF.with(|f| f.set(prev));
            super::super::jit::force(0);
            res
        };
        let fast_res = run_mode(true);
        let gen_res = run_mode(false);
        // Expected: for k in 1..=200: (+2 if k >= 50 => 151*2 = 302) + (sum of odd k in 1..=200 => 100^2 = 10000) = 10302.
        assert_eq!(fast_res, 10302);
        assert_eq!(fast_res, gen_res);
    }
}
