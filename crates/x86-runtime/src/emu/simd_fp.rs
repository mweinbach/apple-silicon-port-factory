//! SSE/AVX floating-point, FMA, AES, BMI, x87, and system instruction execution for [`super::Cpu`].

use super::simd_kernels as vops;
#[cfg(test)]
pub(super) use super::simd_kernels::{sha1nexte, sha256rnds2};
use super::*;

impl Cpu {
    #[inline]
    pub(super) unsafe fn exec_simd_fp_ext(&mut self, mn: Mnemonic, ins_ref: &Instruction) -> bool {
        let ins = ins_ref;
        let cpu = self;
        match mn {
            Mnemonic::Aesenc
            | Mnemonic::Vaesenc
            | Mnemonic::Aesenclast
            | Mnemonic::Vaesenclast
            | Mnemonic::Aesdec
            | Mnemonic::Vaesdec
            | Mnemonic::Aesdeclast
            | Mnemonic::Vaesdeclast
            | Mnemonic::Aesimc
            | Mnemonic::Vaesimc
            | Mnemonic::Aeskeygenassist
            | Mnemonic::Vaeskeygenassist => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (sub, state, src_op, imm8) = match mn {
                        Mnemonic::Aesimc | Mnemonic::Vaesimc => (4u8, 0u128, 1u32, 0u8),
                        Mnemonic::Aeskeygenassist | Mnemonic::Vaeskeygenassist => (5, 0, 1, ins.immediate8()),
                        _ => {
                            let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let st = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                            let sub = match mn {
                                Mnemonic::Aesenc | Mnemonic::Vaesenc => 0,
                                Mnemonic::Aesenclast | Mnemonic::Vaesenclast => 1,
                                Mnemonic::Aesdec | Mnemonic::Vaesdec => 2,
                                _ => 3,
                            };
                            (sub, st, s2_op, 0)
                        }
                    };
                    let src = match ins.op_kind(src_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(src_op)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                        _ => 0,
                    };
                    cpu.xmm[d] = vops::aes_op_128(sub, state, src, imm8);
                    if matches!(
                        mn,
                        Mnemonic::Vaesenc
                            | Mnemonic::Vaesenclast
                            | Mnemonic::Vaesdec
                            | Mnemonic::Vaesdeclast
                            | Mnemonic::Vaesimc
                            | Mnemonic::Vaeskeygenassist
                    ) {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Vbroadcastf128 | Mnemonic::Vbroadcasti128 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let val = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                        _ => 0,
                    };
                    cpu.xmm[d] = val;
                    cpu.set_ymm_hi(d, val);
                }
            }
            Mnemonic::Rsqrtss | Mnemonic::Vrsqrtss | Mnemonic::Rcpss | Mnemonic::Vrcpss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let is_rsqrt = matches!(mn, Mnemonic::Rsqrtss | Mnemonic::Vrsqrtss);
                    let src_op = ins.op_count() - 1;
                    let hi = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let b_bits = match ins.op_kind(src_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(src_op)).map_or(0, |i| cpu.xmm[i] as u32),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                        _ => 0,
                    };
                    let r = vops::rcp_rsqrt_ps(u128::from(b_bits), is_rsqrt) as u32;
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF) | u128::from(r);
                    if matches!(mn, Mnemonic::Vrsqrtss | Mnemonic::Vrcpss) {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Rsqrtps | Mnemonic::Vrsqrtps | Mnemonic::Rcpps | Mnemonic::Vrcpps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let is_rsqrt = matches!(mn, Mnemonic::Rsqrtps | Mnemonic::Vrsqrtps);
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::rcp_rsqrt_ps(src_lo, is_rsqrt);
                    let hi = if ymm { vops::rcp_rsqrt_ps(src_hi, is_rsqrt) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Cmpsb | Mnemonic::Cmpsw | Mnemonic::Cmpsq | Mnemonic::Cmpss | Mnemonic::Vcmpss | Mnemonic::Cmpsd
            | Mnemonic::Vcmpsd => {
                let is_sd = matches!(mn, Mnemonic::Cmpsd | Mnemonic::Vcmpsd);
                let is_string = matches!(mn, Mnemonic::Cmpsb | Mnemonic::Cmpsw | Mnemonic::Cmpsq)
                    || (mn == Mnemonic::Cmpsd && ins.op_kind(0) != OpKind::Register);
                if is_string {
                    // CMPS: flags from [rsi] - [rdi].
                    let (bits, step) = match mn {
                        Mnemonic::Cmpsb => (8u32, 1u64),
                        Mnemonic::Cmpsw => (16, 2),
                        Mnemonic::Cmpsd => (32, 4),
                        _ => (64, 8),
                    };
                    let step = if cpu.df { step.wrapping_neg() } else { step };
                    let rep = ins.has_repe_prefix() || ins.has_repne_prefix();
                    loop {
                        if rep && cpu.gpr[1] == 0 {
                            break;
                        }
                        let a = cpu.read_mem(cpu.gpr[6], bits);
                        let b = cpu.read_mem(cpu.gpr[7], bits);
                        cpu.gpr[6] = cpu.gpr[6].wrapping_add(step);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                        let _ = cpu.update_flags_sub(a, b, bits);
                        if !rep {
                            break;
                        }
                        cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                        if (ins.has_repe_prefix() && !cpu.zf) || (ins.has_repne_prefix() && cpu.zf) {
                            break;
                        }
                    }
                    cpu.last_bits[6] = 64;
                    cpu.last_bits[7] = 64;
                    if rep {
                        cpu.last_bits[1] = 64;
                    }
                } else if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let a_vec = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    if !is_sd {
                        let fa = f32::from_bits(a_vec as u32) as f64;
                        let b_bits = match ins.op_kind(s2_op) {
                            OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u32),
                            OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                            _ => 0,
                        };
                        let fb = f32::from_bits(b_bits) as f64;
                        let mask = if sse_cmp_pred(fa, fb, imm) { 0xFFFF_FFFFu128 } else { 0 };
                        cpu.xmm[d] = (a_vec & !0xFFFF_FFFF) | mask;
                    } else {
                        let fa = f64::from_bits(a_vec as u64);
                        let b_bits = match ins.op_kind(s2_op) {
                            OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u64),
                            OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                            _ => 0,
                        };
                        let fb = f64::from_bits(b_bits);
                        let mask = if sse_cmp_pred(fa, fb, imm) { 0xFFFF_FFFF_FFFF_FFFFu128 } else { 0 };
                        cpu.xmm[d] = (a_vec & !0xFFFF_FFFF_FFFF_FFFF) | mask;
                    }
                    if matches!(mn, Mnemonic::Vcmpss | Mnemonic::Vcmpsd) {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Cmpps | Mnemonic::Vcmpps | Mnemonic::Cmppd | Mnemonic::Vcmppd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let is_pd = matches!(mn, Mnemonic::Cmppd | Mnemonic::Vcmppd);
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let (lo, hi) = if is_pd {
                        (vops::cmp_pd(a_lo, b_lo, imm), if ymm { vops::cmp_pd(a_hi, b_hi, imm) } else { 0 })
                    } else {
                        (vops::cmp_ps(a_lo, b_lo, imm), if ymm { vops::cmp_ps(a_hi, b_hi, imm) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Blendvps
            | Mnemonic::Vblendvps
            | Mnemonic::Blendvpd
            | Mnemonic::Vblendvpd
            | Mnemonic::Pblendvb
            | Mnemonic::Vpblendvb => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, mask_reg) = if ins.op_count() >= 4 {
                        (1, 2, ins.op_register(3))
                    } else {
                        (0, 1, Register::XMM0)
                    };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (m_lo, m_hi) = vops::load_vec_reg(cpu, mask_reg, ymm);
                    let op: fn(u128, u128, u128) -> u128 = match mn {
                        Mnemonic::Pblendvb | Mnemonic::Vpblendvb => vops::blendvb,
                        Mnemonic::Blendvps | Mnemonic::Vblendvps => vops::blendvps,
                        _ => vops::blendvpd,
                    };
                    let lo = op(a_lo, b_lo, m_lo);
                    let hi = if ymm { op(a_hi, b_hi, m_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Shufpd | Mnemonic::Vshufpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::shufpd(a_lo, b_lo, imm);
                    let hi = if ymm { vops::shufpd(a_hi, b_hi, imm >> 2) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Vfmadd132ss
            | Mnemonic::Vfmadd213ss
            | Mnemonic::Vfmadd231ss
            | Mnemonic::Vfmsub132ss
            | Mnemonic::Vfmsub213ss
            | Mnemonic::Vfmsub231ss
            | Mnemonic::Vfnmadd132ss
            | Mnemonic::Vfnmadd213ss
            | Mnemonic::Vfnmadd231ss
            | Mnemonic::Vfnmsub132ss
            | Mnemonic::Vfnmsub213ss
            | Mnemonic::Vfnmsub231ss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let s2_idx = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                    let v1 = f32::from_bits(cpu.xmm[d] as u32);
                    let v2 = f32::from_bits(cpu.xmm[s2_idx] as u32);
                    let v3 = match ins.op_kind(2) {
                        OpKind::Register => f32::from_bits(
                            vops::xmm_idx_fast(ins.op_register(2)).map_or(0, |i| cpu.xmm[i] as u32),
                        ),
                        OpKind::Memory => f32::from_bits(cpu.read_mem(cpu.eff_addr(ins), 32) as u32),
                        _ => 0.0,
                    };
                    let (m1, m2, acc) = match mn {
                        Mnemonic::Vfmadd132ss
                        | Mnemonic::Vfmsub132ss
                        | Mnemonic::Vfnmadd132ss
                        | Mnemonic::Vfnmsub132ss => (v1, v3, v2),
                        Mnemonic::Vfmadd213ss
                        | Mnemonic::Vfmsub213ss
                        | Mnemonic::Vfnmadd213ss
                        | Mnemonic::Vfnmsub213ss => (v2, v1, v3),
                        _ => (v2, v3, v1),
                    };
                    let neg_mul = matches!(
                        mn,
                        Mnemonic::Vfnmadd132ss
                            | Mnemonic::Vfnmadd213ss
                            | Mnemonic::Vfnmadd231ss
                            | Mnemonic::Vfnmsub132ss
                            | Mnemonic::Vfnmsub213ss
                            | Mnemonic::Vfnmsub231ss
                    );
                    let neg_acc = matches!(
                        mn,
                        Mnemonic::Vfmsub132ss
                            | Mnemonic::Vfmsub213ss
                            | Mnemonic::Vfmsub231ss
                            | Mnemonic::Vfnmsub132ss
                            | Mnemonic::Vfnmsub213ss
                            | Mnemonic::Vfnmsub231ss
                    );
                    let lhs = if neg_mul { -m1 } else { m1 };
                    let rhs = if neg_acc { -acc } else { acc };
                    let res = lhs.mul_add(m2, rhs);
                    cpu.xmm[d] = (cpu.xmm[d] & !0xFFFF_FFFF) | (res.to_bits() as u128);
                    cpu.clear_ymm_hi_reg(d);
                }
            }
            Mnemonic::Vfmadd132sd
            | Mnemonic::Vfmadd213sd
            | Mnemonic::Vfmadd231sd
            | Mnemonic::Vfmsub132sd
            | Mnemonic::Vfmsub213sd
            | Mnemonic::Vfmsub231sd
            | Mnemonic::Vfnmadd132sd
            | Mnemonic::Vfnmadd213sd
            | Mnemonic::Vfnmadd231sd
            | Mnemonic::Vfnmsub132sd
            | Mnemonic::Vfnmsub213sd
            | Mnemonic::Vfnmsub231sd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let s2_idx = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                    let v1 = f64::from_bits(cpu.xmm[d] as u64);
                    let v2 = f64::from_bits(cpu.xmm[s2_idx] as u64);
                    let v3 = match ins.op_kind(2) {
                        OpKind::Register => f64::from_bits(
                            vops::xmm_idx_fast(ins.op_register(2)).map_or(0, |i| cpu.xmm[i] as u64),
                        ),
                        OpKind::Memory => f64::from_bits(cpu.read_mem(cpu.eff_addr(ins), 64)),
                        _ => 0.0,
                    };
                    let (m1, m2, acc) = match mn {
                        Mnemonic::Vfmadd132sd
                        | Mnemonic::Vfmsub132sd
                        | Mnemonic::Vfnmadd132sd
                        | Mnemonic::Vfnmsub132sd => (v1, v3, v2),
                        Mnemonic::Vfmadd213sd
                        | Mnemonic::Vfmsub213sd
                        | Mnemonic::Vfnmadd213sd
                        | Mnemonic::Vfnmsub213sd => (v2, v1, v3),
                        _ => (v2, v3, v1),
                    };
                    let neg_mul = matches!(
                        mn,
                        Mnemonic::Vfnmadd132sd
                            | Mnemonic::Vfnmadd213sd
                            | Mnemonic::Vfnmadd231sd
                            | Mnemonic::Vfnmsub132sd
                            | Mnemonic::Vfnmsub213sd
                            | Mnemonic::Vfnmsub231sd
                    );
                    let neg_acc = matches!(
                        mn,
                        Mnemonic::Vfmsub132sd
                            | Mnemonic::Vfmsub213sd
                            | Mnemonic::Vfmsub231sd
                            | Mnemonic::Vfnmsub132sd
                            | Mnemonic::Vfnmsub213sd
                            | Mnemonic::Vfnmsub231sd
                    );
                    let lhs = if neg_mul { -m1 } else { m1 };
                    let rhs = if neg_acc { -acc } else { acc };
                    let res = lhs.mul_add(m2, rhs);
                    cpu.xmm[d] = (cpu.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | (res.to_bits() as u128);
                    cpu.clear_ymm_hi_reg(d);
                }
            }
            Mnemonic::Vfmadd132ps
            | Mnemonic::Vfmadd213ps
            | Mnemonic::Vfmadd231ps
            | Mnemonic::Vfmsub132ps
            | Mnemonic::Vfmsub213ps
            | Mnemonic::Vfmsub231ps
            | Mnemonic::Vfnmadd132ps
            | Mnemonic::Vfnmadd213ps
            | Mnemonic::Vfnmadd231ps
            | Mnemonic::Vfnmsub132ps
            | Mnemonic::Vfnmsub213ps
            | Mnemonic::Vfnmsub231ps
            | Mnemonic::Vfmaddsub132ps
            | Mnemonic::Vfmaddsub213ps
            | Mnemonic::Vfmaddsub231ps
            | Mnemonic::Vfmsubadd132ps
            | Mnemonic::Vfmsubadd213ps
            | Mnemonic::Vfmsubadd231ps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let v1 = (cpu.xmm[d], if ymm { cpu.ymm_hi[d] } else { 0 });
                    let v2 = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                    let v3 = vops::load_vec_op(cpu, ins, 2, ymm);
                    let (m1, m2, acc) = match mn {
                        Mnemonic::Vfmadd132ps
                        | Mnemonic::Vfmsub132ps
                        | Mnemonic::Vfnmadd132ps
                        | Mnemonic::Vfnmsub132ps
                        | Mnemonic::Vfmaddsub132ps
                        | Mnemonic::Vfmsubadd132ps => (v1, v3, v2),
                        Mnemonic::Vfmadd213ps
                        | Mnemonic::Vfmsub213ps
                        | Mnemonic::Vfnmadd213ps
                        | Mnemonic::Vfnmsub213ps
                        | Mnemonic::Vfmaddsub213ps
                        | Mnemonic::Vfmsubadd213ps => (v2, v1, v3),
                        _ => (v2, v3, v1),
                    };
                    let neg_mul = matches!(
                        mn,
                        Mnemonic::Vfnmadd132ps
                            | Mnemonic::Vfnmadd213ps
                            | Mnemonic::Vfnmadd231ps
                            | Mnemonic::Vfnmsub132ps
                            | Mnemonic::Vfnmsub213ps
                            | Mnemonic::Vfnmsub231ps
                    );
                    let acc_mode = match mn {
                        Mnemonic::Vfmsub132ps
                        | Mnemonic::Vfmsub213ps
                        | Mnemonic::Vfmsub231ps
                        | Mnemonic::Vfnmsub132ps
                        | Mnemonic::Vfnmsub213ps
                        | Mnemonic::Vfnmsub231ps => 1u8,
                        Mnemonic::Vfmaddsub132ps | Mnemonic::Vfmaddsub213ps | Mnemonic::Vfmaddsub231ps => 2,
                        Mnemonic::Vfmsubadd132ps | Mnemonic::Vfmsubadd213ps | Mnemonic::Vfmsubadd231ps => 3,
                        _ => 0,
                    };
                    cpu.xmm[d] = vops::fma_ps(m1.0, m2.0, acc.0, neg_mul, acc_mode);
                    if ymm {
                        cpu.set_ymm_hi(d, vops::fma_ps(m1.1, m2.1, acc.1, neg_mul, acc_mode));
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Vfmadd132pd
            | Mnemonic::Vfmadd213pd
            | Mnemonic::Vfmadd231pd
            | Mnemonic::Vfmsub132pd
            | Mnemonic::Vfmsub213pd
            | Mnemonic::Vfmsub231pd
            | Mnemonic::Vfnmadd132pd
            | Mnemonic::Vfnmadd213pd
            | Mnemonic::Vfnmadd231pd
            | Mnemonic::Vfnmsub132pd
            | Mnemonic::Vfnmsub213pd
            | Mnemonic::Vfnmsub231pd
            | Mnemonic::Vfmaddsub132pd
            | Mnemonic::Vfmaddsub213pd
            | Mnemonic::Vfmaddsub231pd
            | Mnemonic::Vfmsubadd132pd
            | Mnemonic::Vfmsubadd213pd
            | Mnemonic::Vfmsubadd231pd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let v1 = (cpu.xmm[d], if ymm { cpu.ymm_hi[d] } else { 0 });
                    let v2 = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                    let v3 = vops::load_vec_op(cpu, ins, 2, ymm);
                    let (m1, m2, acc) = match mn {
                        Mnemonic::Vfmadd132pd
                        | Mnemonic::Vfmsub132pd
                        | Mnemonic::Vfnmadd132pd
                        | Mnemonic::Vfnmsub132pd
                        | Mnemonic::Vfmaddsub132pd
                        | Mnemonic::Vfmsubadd132pd => (v1, v3, v2),
                        Mnemonic::Vfmadd213pd
                        | Mnemonic::Vfmsub213pd
                        | Mnemonic::Vfnmadd213pd
                        | Mnemonic::Vfnmsub213pd
                        | Mnemonic::Vfmaddsub213pd
                        | Mnemonic::Vfmsubadd213pd => (v2, v1, v3),
                        _ => (v2, v3, v1),
                    };
                    let neg_mul = matches!(
                        mn,
                        Mnemonic::Vfnmadd132pd
                            | Mnemonic::Vfnmadd213pd
                            | Mnemonic::Vfnmadd231pd
                            | Mnemonic::Vfnmsub132pd
                            | Mnemonic::Vfnmsub213pd
                            | Mnemonic::Vfnmsub231pd
                    );
                    let acc_mode = match mn {
                        Mnemonic::Vfmsub132pd
                        | Mnemonic::Vfmsub213pd
                        | Mnemonic::Vfmsub231pd
                        | Mnemonic::Vfnmsub132pd
                        | Mnemonic::Vfnmsub213pd
                        | Mnemonic::Vfnmsub231pd => 1u8,
                        Mnemonic::Vfmaddsub132pd | Mnemonic::Vfmaddsub213pd | Mnemonic::Vfmaddsub231pd => 2,
                        Mnemonic::Vfmsubadd132pd | Mnemonic::Vfmsubadd213pd | Mnemonic::Vfmsubadd231pd => 3,
                        _ => 0,
                    };
                    cpu.xmm[d] = vops::fma_pd(m1.0, m2.0, acc.0, neg_mul, acc_mode);
                    if ymm {
                        cpu.set_ymm_hi(d, vops::fma_pd(m1.1, m2.1, acc.1, neg_mul, acc_mode));
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Unpckhps | Mnemonic::Vunpckhps | Mnemonic::Punpckhdq | Mnemonic::Vpunpckhdq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckhdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckhdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpckldq | Mnemonic::Vpunpckldq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckldq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckldq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpckhqdq | Mnemonic::Vpunpckhqdq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckhqdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckhqdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpckhbw
            | Mnemonic::Vpunpckhbw
            | Mnemonic::Punpcklwd
            | Mnemonic::Vpunpcklwd
            | Mnemonic::Punpckhwd
            | Mnemonic::Vpunpckhwd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: fn(u128, u128) -> u128 = match mn {
                        Mnemonic::Punpckhbw | Mnemonic::Vpunpckhbw => vops::punpckhbw,
                        Mnemonic::Punpcklwd | Mnemonic::Vpunpcklwd => vops::punpcklwd,
                        _ => vops::punpckhwd,
                    };
                    let lo = op(a_lo, b_lo);
                    let hi = if ymm { op(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pminub
            | Mnemonic::Vpminub
            | Mnemonic::Pmaxub
            | Mnemonic::Vpmaxub
            | Mnemonic::Pminsb
            | Mnemonic::Vpminsb
            | Mnemonic::Pmaxsb
            | Mnemonic::Vpmaxsb
            | Mnemonic::Pminuw
            | Mnemonic::Vpminuw
            | Mnemonic::Pmaxuw
            | Mnemonic::Vpmaxuw
            | Mnemonic::Pminsw
            | Mnemonic::Vpminsw
            | Mnemonic::Pmaxsw
            | Mnemonic::Vpmaxsw
            | Mnemonic::Pminud
            | Mnemonic::Vpminud
            | Mnemonic::Pmaxud
            | Mnemonic::Vpmaxud
            | Mnemonic::Pminsd
            | Mnemonic::Vpminsd
            | Mnemonic::Pmaxsd
            | Mnemonic::Vpmaxsd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: fn(u128, u128) -> u128 = match mn {
                        Mnemonic::Pminub | Mnemonic::Vpminub => vops::pminub,
                        Mnemonic::Pmaxub | Mnemonic::Vpmaxub => vops::pmaxub,
                        Mnemonic::Pminsb | Mnemonic::Vpminsb => vops::pminsb,
                        Mnemonic::Pmaxsb | Mnemonic::Vpmaxsb => vops::pmaxsb,
                        Mnemonic::Pminuw | Mnemonic::Vpminuw => vops::pminuw,
                        Mnemonic::Pmaxuw | Mnemonic::Vpmaxuw => vops::pmaxuw,
                        Mnemonic::Pminsw | Mnemonic::Vpminsw => vops::pminsw,
                        Mnemonic::Pmaxsw | Mnemonic::Vpmaxsw => vops::pmaxsw,
                        Mnemonic::Pminud | Mnemonic::Vpminud => vops::pminud,
                        Mnemonic::Pmaxud | Mnemonic::Vpmaxud => vops::pmaxud,
                        Mnemonic::Pminsd | Mnemonic::Vpminsd => vops::pminsd,
                        _ => vops::pmaxsd,
                    };
                    let lo = op(a_lo, b_lo);
                    let hi = if ymm { op(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Movshdup
            | Mnemonic::Vmovshdup
            | Mnemonic::Movsldup
            | Mnemonic::Vmovsldup
            | Mnemonic::Movddup
            | Mnemonic::Vmovddup => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let s_op = ins.op_count() - 1;
                    let is_ddup = matches!(mn, Mnemonic::Movddup | Mnemonic::Vmovddup);
                    let (src_lo, src_hi) = if is_ddup && !ymm && ins.op_kind(s_op) == OpKind::Memory {
                        // `movddup xmm, m64` only touches 8 bytes in memory.
                        (cpu.read_mem(cpu.eff_addr(ins), 64) as u128, 0)
                    } else {
                        vops::load_vec_op(cpu, ins, s_op, ymm)
                    };
                    let op: fn(u128) -> u128 = if is_ddup {
                        vops::movddup
                    } else if matches!(mn, Mnemonic::Movshdup | Mnemonic::Vmovshdup) {
                        vops::movshdup
                    } else {
                        vops::movsldup
                    };
                    let lo = op(src_lo);
                    let hi = if ymm { op(src_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Addsubps
            | Mnemonic::Vaddsubps
            | Mnemonic::Addsubpd
            | Mnemonic::Vaddsubpd
            | Mnemonic::Haddps
            | Mnemonic::Vhaddps
            | Mnemonic::Hsubps
            | Mnemonic::Vhsubps
            | Mnemonic::Haddpd
            | Mnemonic::Vhaddpd
            | Mnemonic::Hsubpd
            | Mnemonic::Vhsubpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Addsubps | Mnemonic::Vaddsubps => (
                            vops::addsub_ps(a_lo, b_lo),
                            if ymm { vops::addsub_ps(a_hi, b_hi) } else { 0 },
                        ),
                        Mnemonic::Addsubpd | Mnemonic::Vaddsubpd => (
                            vops::addsub_pd(a_lo, b_lo),
                            if ymm { vops::addsub_pd(a_hi, b_hi) } else { 0 },
                        ),
                        Mnemonic::Haddps | Mnemonic::Vhaddps | Mnemonic::Hsubps | Mnemonic::Vhsubps => {
                            let is_add = matches!(mn, Mnemonic::Haddps | Mnemonic::Vhaddps);
                            (
                                vops::hadd_sub_ps(a_lo, b_lo, is_add),
                                if ymm { vops::hadd_sub_ps(a_hi, b_hi, is_add) } else { 0 },
                            )
                        }
                        _ => {
                            let is_add = matches!(mn, Mnemonic::Haddpd | Mnemonic::Vhaddpd);
                            (
                                vops::hadd_sub_pd(a_lo, b_lo, is_add),
                                if ymm { vops::hadd_sub_pd(a_hi, b_hi, is_add) } else { 0 },
                            )
                        }
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pmulhw
            | Mnemonic::Vpmulhw
            | Mnemonic::Pmulhuw
            | Mnemonic::Vpmulhuw
            | Mnemonic::Pmullw
            | Mnemonic::Vpmullw
            | Mnemonic::Pmulhrsw
            | Mnemonic::Vpmulhrsw
            | Mnemonic::Pmulld
            | Mnemonic::Vpmulld
            | Mnemonic::Pmuludq
            | Mnemonic::Vpmuludq
            | Mnemonic::Pmuldq
            | Mnemonic::Vpmuldq
            | Mnemonic::Pmaddwd
            | Mnemonic::Vpmaddwd
            | Mnemonic::Pmaddubsw
            | Mnemonic::Vpmaddubsw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: fn(u128, u128) -> u128 = match mn {
                        Mnemonic::Pmulhw | Mnemonic::Vpmulhw => vops::pmulhw,
                        Mnemonic::Pmulhuw | Mnemonic::Vpmulhuw => vops::pmulhuw,
                        Mnemonic::Pmullw | Mnemonic::Vpmullw => vops::pmullw,
                        Mnemonic::Pmulhrsw | Mnemonic::Vpmulhrsw => vops::pmulhrsw,
                        Mnemonic::Pmulld | Mnemonic::Vpmulld => vops::pmulld,
                        Mnemonic::Pmuludq | Mnemonic::Vpmuludq => vops::pmuludq,
                        Mnemonic::Pmuldq | Mnemonic::Vpmuldq => vops::pmuldq,
                        Mnemonic::Pmaddwd | Mnemonic::Vpmaddwd => vops::pmaddwd,
                        _ => vops::pmaddubsw,
                    };
                    let lo = op(a_lo, b_lo);
                    let hi = if ymm { op(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Packuswb
            | Mnemonic::Vpackuswb
            | Mnemonic::Packsswb
            | Mnemonic::Vpacksswb
            | Mnemonic::Packssdw
            | Mnemonic::Vpackssdw
            | Mnemonic::Packusdw
            | Mnemonic::Vpackusdw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: fn(u128, u128) -> u128 = match mn {
                        Mnemonic::Packuswb | Mnemonic::Vpackuswb => vops::packuswb,
                        Mnemonic::Packsswb | Mnemonic::Vpacksswb => vops::packsswb,
                        Mnemonic::Packssdw | Mnemonic::Vpackssdw => vops::packssdw,
                        _ => vops::packusdw,
                    };
                    let lo = op(a_lo, b_lo);
                    let hi = if ymm { op(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Paddsb
            | Mnemonic::Vpaddsb
            | Mnemonic::Psubsb
            | Mnemonic::Vpsubsb
            | Mnemonic::Paddusb
            | Mnemonic::Vpaddusb
            | Mnemonic::Psubusb
            | Mnemonic::Vpsubusb
            | Mnemonic::Paddsw
            | Mnemonic::Vpaddsw
            | Mnemonic::Psubsw
            | Mnemonic::Vpsubsw
            | Mnemonic::Paddusw
            | Mnemonic::Vpaddusw
            | Mnemonic::Psubusw
            | Mnemonic::Vpsubusw
            | Mnemonic::Pavgb
            | Mnemonic::Vpavgb
            | Mnemonic::Pavgw
            | Mnemonic::Vpavgw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: fn(u128, u128) -> u128 = match mn {
                        Mnemonic::Paddsb | Mnemonic::Vpaddsb => vops::paddsb,
                        Mnemonic::Psubsb | Mnemonic::Vpsubsb => vops::psubsb,
                        Mnemonic::Paddusb | Mnemonic::Vpaddusb => vops::paddusb,
                        Mnemonic::Psubusb | Mnemonic::Vpsubusb => vops::psubusb,
                        Mnemonic::Paddsw | Mnemonic::Vpaddsw => vops::paddsw,
                        Mnemonic::Psubsw | Mnemonic::Vpsubsw => vops::psubsw,
                        Mnemonic::Paddusw | Mnemonic::Vpaddusw => vops::paddusw,
                        Mnemonic::Psubusw | Mnemonic::Vpsubusw => vops::psubusw,
                        Mnemonic::Pavgb | Mnemonic::Vpavgb => vops::pavgb,
                        _ => vops::pavgw,
                    };
                    let lo = op(a_lo, b_lo);
                    let hi = if ymm { op(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pabsb
            | Mnemonic::Vpabsb
            | Mnemonic::Pabsw
            | Mnemonic::Vpabsw
            | Mnemonic::Pabsd
            | Mnemonic::Vpabsd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let s_op = ins.op_count() - 1;
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, s_op, ymm);
                    let op: fn(u128) -> u128 = match mn {
                        Mnemonic::Pabsb | Mnemonic::Vpabsb => vops::pabsb,
                        Mnemonic::Pabsw | Mnemonic::Vpabsw => vops::pabsw,
                        _ => vops::pabsd,
                    };
                    let lo = op(src_lo);
                    let hi = if ymm { op(src_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Palignr | Mnemonic::Vpalignr => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::palignr(a_lo, b_lo, imm);
                    let hi = if ymm { vops::palignr(a_hi, b_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Fnstcw | Mnemonic::Fstcw => {
                let a = cpu.eff_addr(ins);
                cpu.write_mem(a, 16, super::regfile_ext::x87_snapshot().cw.into());
            }
            Mnemonic::Fldcw => {
                let cw = cpu.read_mem(cpu.eff_addr(ins), 16) as u16;
                if !super::precise_faults() || super::FAULT_PENDING.with(|p| p.get().is_none()) {
                    super::regfile_ext::with_x87_mut(|st| st.cw = super::xstate::control_word(cw));
                }
            }
            Mnemonic::Dpps | Mnemonic::Vdpps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::dp_ps(a_lo, b_lo, imm);
                    let hi = if ymm { vops::dp_ps(a_hi, b_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psadbw | Mnemonic::Vpsadbw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::psadbw(a_lo, b_lo);
                    let hi = if ymm { vops::psadbw(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pclmulqdq | Mnemonic::Vpclmulqdq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::pclmulqdq(a_lo, b_lo, imm);
                    let hi = if ymm { vops::pclmulqdq(a_hi, b_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Movbe => {
                let bits = Cpu::op_bits(ins, 0);
                let v = cpu.read_op(ins, 1, bits);
                let r = match bits {
                    16 => (v as u16).swap_bytes() as u64,
                    32 => (v as u32).swap_bytes() as u64,
                    _ => v.swap_bytes(),
                };
                cpu.write_op(ins, 0, bits, r);
            }
            Mnemonic::Lahf => {
                // AH = SF:ZF:0:AF:0:PF:1:CF (AF is not tracked; reads as 0).
                let ah = ((cpu.sf as u64) << 7) | ((cpu.zf as u64) << 6) | ((cpu.pf() as u64) << 2) | 2 | (cpu.cf as u64);
                cpu.set_reg(Register::AH, ah);
            }
            Mnemonic::Sahf => {
                let ah = (cpu.gpr[0] >> 8) & 0xFF;
                cpu.sf = ah & 0x80 != 0;
                cpu.zf = ah & 0x40 != 0;
                cpu.set_pf(ah & 0x04 != 0);
                cpu.cf = ah & 0x01 != 0;
            }
            Mnemonic::Rcl | Mnemonic::Rcr => {
                let bits = Cpu::op_bits(ins, 0);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let raw = (cpu.read_op(ins, 1, 8) as u32) & if bits == 64 { 63 } else { 31 };
                if raw != 0 {
                    // Rotate through CF: a (bits+1)-bit rotation.
                    let cnt = match bits {
                        8 => raw % 9,
                        16 => raw % 17,
                        _ => raw,
                    };
                    let mut a = cpu.read_op(ins, 0, bits) & mask;
                    let mut cf = cpu.cf;
                    let top = bits - 1;
                    for _ in 0..cnt {
                        if mn == Mnemonic::Rcl {
                            let out = (a >> top) & 1 != 0;
                            a = ((a << 1) | cf as u64) & mask;
                            cf = out;
                        } else {
                            let out = a & 1 != 0;
                            a = (a >> 1) | ((cf as u64) << top);
                            cf = out;
                        }
                    }
                    cpu.write_op(ins, 0, bits, a);
                    cpu.cf = cf;
                    // OF is architecturally defined for count 1 only; use the count-1 formula.
                    cpu.of = if mn == Mnemonic::Rcl {
                        ((a >> top) & 1 != 0) ^ cf
                    } else {
                        ((a >> top) ^ (a >> (top - 1))) & 1 != 0
                    };
                }
            }
            Mnemonic::Bextr => {
                // BMI1: dst = (src >> ctl[7:0]) & ((1 << ctl[15:8]) - 1). ZF from result, CF=OF=0.
                let bits = Cpu::op_bits(ins, 0);
                let src = cpu.read_op(ins, 1, bits);
                let ctl = cpu.read_op(ins, 2, bits);
                let start = (ctl & 0xFF) as u32;
                let len = ((ctl >> 8) & 0xFF) as u32;
                let v = if start >= bits { 0 } else { src >> start };
                let res = if len >= 64 { v } else { v & ((1u64 << len) - 1) };
                let res = if bits == 32 { res & 0xFFFF_FFFF } else { res };
                cpu.write_op(ins, 0, bits, res);
                cpu.update_flags_logic(res, bits);
            }
            Mnemonic::Pdep | Mnemonic::Pext => {
                // BMI2, no flags: iterate only set bits of the mask via isolate-LSB / clear-LSB.
                let bits = Cpu::op_bits(ins, 0);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let src = cpu.read_op(ins, 1, bits) & mask;
                let mut mm = cpu.read_op(ins, 2, bits) & mask;
                let mut res = 0u64;
                let mut k = 0u32;
                if mn == Mnemonic::Pdep {
                    while mm != 0 {
                        let bit = mm & mm.wrapping_neg();
                        if ((src >> k) & 1) != 0 {
                            res |= bit;
                        }
                        mm &= mm - 1;
                        k += 1;
                    }
                } else {
                    while mm != 0 {
                        let bit = mm & mm.wrapping_neg();
                        if (src & bit) != 0 {
                            res |= 1u64 << k;
                        }
                        mm &= mm - 1;
                        k += 1;
                    }
                }
                cpu.write_op(ins, 0, bits, res);
            }
            Mnemonic::Adcx | Mnemonic::Adox => {
                // ADX: add with carry-in/out through CF (adcx) or OF (adox) only; other flags untouched.
                let bits = Cpu::op_bits(ins, 0);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let a = cpu.read_op(ins, 0, bits) & mask;
                let b = cpu.read_op(ins, 1, bits) & mask;
                let c = if mn == Mnemonic::Adcx { cpu.cf } else { cpu.of };
                let full = (a as u128) + (b as u128) + (c as u128);
                cpu.write_op(ins, 0, bits, (full as u64) & mask);
                if mn == Mnemonic::Adcx {
                    cpu.cf = full > mask as u128;
                } else {
                    cpu.of = full > mask as u128;
                }
            }
            Mnemonic::Lodsb | Mnemonic::Lodsw | Mnemonic::Lodsd | Mnemonic::Lodsq => {
                let (reg, bits) = match mn {
                    Mnemonic::Lodsb => (Register::AL, 8u32),
                    Mnemonic::Lodsw => (Register::AX, 16),
                    Mnemonic::Lodsd => (Register::EAX, 32),
                    _ => (Register::RAX, 64),
                };
                let step = (bits / 8) as u64;
                let step = if cpu.df { step.wrapping_neg() } else { step };
                let rep = ins.has_rep_prefix();
                let count = if rep { cpu.gpr[1] } else { 1 };
                if count > 0 {
                    // Only the last element loaded is architecturally visible.
                    let last = cpu.gpr[6].wrapping_add(step.wrapping_mul(count - 1));
                    let v = cpu.read_mem(last, bits);
                    cpu.set_reg(reg, v);
                    cpu.gpr[6] = cpu.gpr[6].wrapping_add(step.wrapping_mul(count));
                    cpu.last_bits[6] = 64;
                }
                if rep {
                    cpu.gpr[1] = 0;
                    cpu.last_bits[1] = 64;
                }
            }
            Mnemonic::Xlatb => {
                let v = cpu.read_mem(cpu.gpr[3].wrapping_add(cpu.gpr[0] & 0xFF), 8);
                cpu.set_reg(Register::AL, v);
            }
            Mnemonic::Jrcxz | Mnemonic::Jecxz => {
                let c = if mn == Mnemonic::Jecxz { cpu.gpr[1] as u32 as u64 } else { cpu.gpr[1] };
                if c == 0 {
                    cpu.rip = ins.near_branch_target();
                }
            }
            Mnemonic::Loop | Mnemonic::Loope | Mnemonic::Loopne => {
                cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                let take = cpu.gpr[1] != 0
                    && match mn {
                        Mnemonic::Loope => cpu.zf,
                        Mnemonic::Loopne => !cpu.zf,
                        _ => true,
                    };
                if take {
                    cpu.rip = ins.near_branch_target();
                }
            }
            Mnemonic::Crc32 => {
                let src_bits = Cpu::op_bits(ins, 1);
                let v = cpu.read_op(ins, 1, src_bits);
                let crc = vops::crc32c_step(cpu.read_op(ins, 0, 32) as u32, v, src_bits);
                let dst_bits = Cpu::op_bits(ins, 0);
                cpu.write_op(ins, 0, dst_bits, crc as u64);
            }
            Mnemonic::Pblendw | Mnemonic::Vpblendw | Mnemonic::Vpblendd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let (lo, hi) = if mn == Mnemonic::Vpblendd {
                        // One imm bit per dword across the full 256 bits.
                        (
                            vops::blendps_imm(a_lo, b_lo, imm & 0x0F),
                            if ymm { vops::blendps_imm(a_hi, b_hi, imm >> 4) } else { 0 },
                        )
                    } else {
                        // One imm bit per word, the same 8 bits reused for each 128-bit lane.
                        (
                            vops::pblendw_imm(a_lo, b_lo, imm),
                            if ymm { vops::pblendw_imm(a_hi, b_hi, imm) } else { 0 },
                        )
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Extractps | Mnemonic::Vextractps => {
                if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                    let imm = (cpu.read_op(ins, 2, 8) & 3) as u32;
                    let v = (cpu.xmm[s] >> (32 * imm)) as u32 as u64;
                    let bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, bits, v);
                }
            }
            Mnemonic::Vpermd | Mnemonic::Vpermps => {
                // dst[i] = src2[idx[i] & 7]; operand 1 holds the indices.
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (i_lo, i_hi) = vops::load_vec_reg(cpu, ins.op_register(1), true);
                    let (t_lo, t_hi) = vops::load_vec_op(cpu, ins, 2, true);
                    let (lo, hi) = vops::vpermd_ymm(i_lo, i_hi, t_lo, t_hi);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
            }
            Mnemonic::Vpermq | Mnemonic::Vpermpd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s_lo, s_hi) = vops::load_vec_op(cpu, ins, 1, true);
                    let (lo, hi) = vops::vpermq(s_lo, s_hi, cpu.read_op(ins, 2, 8) as u8);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
            }
            Mnemonic::Vpsllvd | Mnemonic::Vpsrlvd | Mnemonic::Vpsravd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                    let (c_lo, c_hi) = vops::load_vec_op(cpu, ins, 2, ymm);
                    let kind = match mn {
                        Mnemonic::Vpsllvd => 0u8,
                        Mnemonic::Vpsrlvd => 1,
                        _ => 2,
                    };
                    cpu.xmm[d] = vops::vpshift_vd(a_lo, c_lo, kind);
                    if ymm {
                        cpu.set_ymm_hi(d, vops::vpshift_vd(a_hi, c_hi, kind));
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Vpsllvq | Mnemonic::Vpsrlvq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                    let (c_lo, c_hi) = vops::load_vec_op(cpu, ins, 2, ymm);
                    let is_shr = mn == Mnemonic::Vpsrlvq;
                    cpu.xmm[d] = vops::vpshift_vq(a_lo, c_lo, is_shr);
                    if ymm {
                        cpu.set_ymm_hi(d, vops::vpshift_vq(a_hi, c_hi, is_shr));
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Pushf | Mnemonic::Pushfq => {
                // RFLAGS image: reserved bit 1 and IF always set; AF/TF are not modelled.
                let fl = 0x202u64
                    | (cpu.cf as u64)
                    | ((cpu.pf() as u64) << 2)
                    | ((cpu.zf as u64) << 6)
                    | ((cpu.sf as u64) << 7)
                    | ((cpu.df as u64) << 10)
                    | ((cpu.of as u64) << 11);
                if mn == Mnemonic::Pushfq {
                    cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
                    cpu.write_mem(cpu.gpr[4], 64, fl);
                } else {
                    cpu.gpr[4] = cpu.gpr[4].wrapping_sub(2);
                    cpu.write_mem(cpu.gpr[4], 16, fl);
                }
            }
            Mnemonic::Popf | Mnemonic::Popfq => {
                let fl = if mn == Mnemonic::Popfq {
                    let v = cpu.read_mem(cpu.gpr[4], 64);
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                    v
                } else {
                    let v = cpu.read_mem(cpu.gpr[4], 16);
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(2);
                    v
                };
                cpu.cf = fl & 0x001 != 0;
                cpu.set_pf(fl & 0x004 != 0);
                cpu.zf = fl & 0x040 != 0;
                cpu.sf = fl & 0x080 != 0;
                cpu.df = fl & 0x400 != 0;
                cpu.of = fl & 0x800 != 0;
            }
            Mnemonic::Phaddsw | Mnemonic::Phsubsw | Mnemonic::Vphaddsw | Mnemonic::Vphsubsw => {
                let (d, ymm) = vops::xmm_reg_info(ins.op_register(0)).unwrap();
                let src = u32::from(Cpu::is_vex(ins));
                let (a, ah) = cpu.vsrc(ins, src, ymm);
                let (b, bh) = cpu.vsrc(ins, src + 1, ymm);
                let sub = matches!(mn, Mnemonic::Phsubsw | Mnemonic::Vphsubsw);
                cpu.vdst(ins, d, vops::phadd_sub_sw(a, b, sub), vops::phadd_sub_sw(ah, bh, sub));
            }
            Mnemonic::Phminposuw | Mnemonic::Vphminposuw => {
                let (src, _) = cpu.vsrc(ins, 1, false);
                let (i, min) = (0..8).map(|i| (i, (src >> (16 * i)) as u16)).min_by_key(|&(_, v)| v).unwrap();
                cpu.vdst(ins, Cpu::xmm_idx(ins.op_register(0)).unwrap(), u128::from(min) | (i as u128) << 16, 0);
            }
            Mnemonic::Mpsadbw | Mnemonic::Vmpsadbw => {
                let (d, ymm) = vops::xmm_reg_info(ins.op_register(0)).unwrap();
                let src = u32::from(Cpu::is_vex(ins));
                let (a, ah) = cpu.vsrc(ins, src, ymm);
                let (b, bh) = cpu.vsrc(ins, src + 1, ymm);
                let sad = |a: u128, b: u128, control: u8| {
                    let (a, b) = (a.to_le_bytes(), b.to_le_bytes());
                    let i = usize::from(control >> 2 & 1) * 4;
                    let j = usize::from(control & 3) * 4;
                    (0..8).fold(0u128, |result, k| {
                        let sum: u16 = (0..4).map(|n| u16::from(a[i + k + n].abs_diff(b[j + n]))).sum();
                        result | (u128::from(sum) << (16 * k))
                    })
                };
                let imm = ins.immediate8();
                cpu.vdst(ins, d, sad(a, b, imm), sad(ah, bh, imm >> 3));
            }
            Mnemonic::Dppd | Mnemonic::Vdppd => {
                let src = u32::from(Cpu::is_vex(ins));
                let (a, _) = cpu.vsrc(ins, src, false);
                let (b, _) = cpu.vsrc(ins, src + 1, false);
                let imm = ins.immediate8();
                let product = |shift: u32, bit: u8| {
                    if imm & bit == 0 {
                        return 0.0;
                    }
                    super::sse_arith64(
                        2,
                        f64::from_bits((a >> shift) as u64),
                        f64::from_bits((b >> shift) as u64),
                    )
                };
                let sum = u128::from(super::sse_arith64(0, product(0, 16), product(64, 32)).to_bits());
                let lo = if imm & 1 != 0 { sum } else { 0 };
                let hi = if imm & 2 != 0 { sum << 64 } else { 0 };
                cpu.vdst(ins, Cpu::xmm_idx(ins.op_register(0)).unwrap(), lo | hi, 0);
            }
            Mnemonic::Phaddd | Mnemonic::Phaddw | Mnemonic::Phsubd | Mnemonic::Phsubw | Mnemonic::Vphaddd
            | Mnemonic::Vphaddw | Mnemonic::Vphsubd | Mnemonic::Vphsubw => {
                // Horizontal add/sub of adjacent pairs: low half of each 128-bit lane from src1,
                // high half from src2 (PHSUB: even - odd). MMX forms use the physical x87 slots in mmx.
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let sub = matches!(mn, Mnemonic::Phsubd | Mnemonic::Phsubw | Mnemonic::Vphsubd | Mnemonic::Vphsubw);
                    let word = matches!(mn, Mnemonic::Phaddw | Mnemonic::Phsubw | Mnemonic::Vphaddw | Mnemonic::Vphsubw);
                    let (lo, hi) = if word {
                        (
                            vops::phadd_sub_w(a_lo, b_lo, sub),
                            if ymm { vops::phadd_sub_w(a_hi, b_hi, sub) } else { 0 },
                        )
                    } else {
                        (
                            vops::phadd_sub_d(a_lo, b_lo, sub),
                            if ymm { vops::phadd_sub_d(a_hi, b_hi, sub) } else { 0 },
                        )
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Vtestps | Mnemonic::Vtestpd => {
                // Like PTEST but only the per-element sign bits participate.
                let ymm = vops::is_ymm_fast(ins.op_register(0));
                let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(0), ymm);
                let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                let m: u128 = if mn == Mnemonic::Vtestps {
                    0x8000_0000_8000_0000_8000_0000_8000_0000
                } else {
                    0x8000_0000_0000_0000_8000_0000_0000_0000
                };
                cpu.zf = (((a_lo & b_lo) | (a_hi & b_hi)) & m) == 0;
                cpu.cf = (((!a_lo & b_lo) | (!a_hi & b_hi)) & m) == 0;
                cpu.sf = false;
                cpu.of = false;
                cpu.set_pf(false);
            }
            Mnemonic::Vpmaskmovd | Mnemonic::Vpmaskmovq | Mnemonic::Vmaskmovps | Mnemonic::Vmaskmovpd => {
                // Element-wise masked load/store keyed on mask sign bits. Masked-off elements are
                // never touched in memory (faults are suppressed on hardware); loads zero them.
                let q = matches!(mn, Mnemonic::Vpmaskmovq | Mnemonic::Vmaskmovpd);
                let esz: u32 = if q { 8 } else { 4 };
                let per_lane = (16 / esz) as usize;
                let addr = cpu.eff_addr(ins);
                let mask_of = |lo: u128, hi: u128, i: usize| -> bool {
                    let v = if i < per_lane { lo } else { hi };
                    let sh = (i % per_lane) as u32 * esz * 8;
                    (v >> (sh + esz * 8 - 1)) & 1 != 0
                };
                if ins.op_kind(0) == OpKind::Register {
                    if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                        let (m_lo, m_hi) = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                        let n = if ymm { 2 * per_lane } else { per_lane };
                        let mut out = [0u128; 2];
                        for i in 0..n {
                            if mask_of(m_lo, m_hi, i) {
                                let v = cpu.read_mem(addr.wrapping_add((i as u64) * esz as u64), esz * 8) as u128;
                                out[i / per_lane] |= v << ((i % per_lane) as u32 * esz * 8);
                            }
                        }
                        vops::store_vec_dst(cpu, ins, d, ymm, out[0], out[1]);
                    }
                } else {
                    let ymm = vops::is_ymm_fast(ins.op_register(1));
                    let (m_lo, m_hi) = vops::load_vec_reg(cpu, ins.op_register(1), ymm);
                    let (s_lo, s_hi) = vops::load_vec_reg(cpu, ins.op_register(2), ymm);
                    let n = if ymm { 2 * per_lane } else { per_lane };
                    for i in 0..n {
                        if mask_of(m_lo, m_hi, i) {
                            let src = if i < per_lane { s_lo } else { s_hi };
                            let v = (src >> ((i % per_lane) as u32 * esz * 8)) as u64;
                            cpu.write_mem(addr.wrapping_add((i as u64) * esz as u64), esz * 8, v);
                        }
                    }
                }
            }
            Mnemonic::Sha1rnds4
            | Mnemonic::Sha1nexte
            | Mnemonic::Sha1msg1
            | Mnemonic::Sha1msg2
            | Mnemonic::Sha256rnds2
            | Mnemonic::Sha256msg1
            | Mnemonic::Sha256msg2 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src1 = cpu.xmm[d];
                    let src2 = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                        _ => 0,
                    };
                    let res = match mn {
                        Mnemonic::Sha1rnds4 => vops::sha1rnds4(src1, src2, ins.immediate8()),
                        Mnemonic::Sha1nexte => vops::sha1nexte(src1, src2),
                        Mnemonic::Sha1msg1 => vops::sha1msg1(src1, src2),
                        Mnemonic::Sha1msg2 => vops::sha1msg2(src1, src2),
                        Mnemonic::Sha256rnds2 => vops::sha256rnds2(src1, src2, cpu.xmm[0]),
                        Mnemonic::Sha256msg1 => vops::sha256msg1(src1, src2),
                        _ => vops::sha256msg2(src1, src2),
                    };
                    // Legacy SSE encoding: preserves upper 128 bits (`ymm_hi[d]`).
                    cpu.xmm[d] = res;
                }
            }
            _ => return super::regfile_ext::exec_x87_ins(cpu, mn, ins),
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_x86::{Decoder, DecoderOptions};

    fn pack_f32(v: [f32; 4]) -> u128 {
        (v[0].to_bits() as u128)
            | ((v[1].to_bits() as u128) << 32)
            | ((v[2].to_bits() as u128) << 64)
            | ((v[3].to_bits() as u128) << 96)
    }

    fn unpack_f32_bits(v: u128) -> [u32; 4] {
        [v as u32, (v >> 32) as u32, (v >> 64) as u32, (v >> 96) as u32]
    }

    fn pack_f64(v: [f64; 2]) -> u128 {
        (v[0].to_bits() as u128) | ((v[1].to_bits() as u128) << 64)
    }

    fn unpack_f64_bits(v: u128) -> [u64; 2] {
        [v as u64, (v >> 64) as u64]
    }

    fn exec_bytes(cpu: &mut Cpu, code: &[u8]) {
        let mut dec = Decoder::with_ip(64, code, 0x1000, DecoderOptions::NONE);
        let ins = dec.decode();
        let mn = ins.mnemonic();
        unsafe {
            if !cpu.exec_simd_mov_int(mn, &ins) {
                assert!(cpu.exec_simd_fp_ext(mn, &ins), "unhandled test insn: {ins}");
            }
        }
    }

    #[test]
    fn simd_fp_fma_ps_and_pd_all_variants() {
        let m1 = pack_f32([1.5, -2.0, 3.25, 0.5]);
        let m2 = pack_f32([4.0, 5.0, -2.0, 8.0]);
        let acc = pack_f32([10.0, 20.0, 30.0, 40.0]);
        for neg_mul in [false, true] {
            for acc_mode in 0u8..4 {
                let got = unpack_f32_bits(vops::fma_ps(m1, m2, acc, neg_mul, acc_mode));
                for lane in 0..4 {
                    let a = f32::from_bits((m1 >> (lane * 32)) as u32);
                    let b = f32::from_bits((m2 >> (lane * 32)) as u32);
                    let c = f32::from_bits((acc >> (lane * 32)) as u32);
                    let neg_acc = match acc_mode {
                        1 => true,
                        2 => (lane & 1) == 0,
                        3 => (lane & 1) == 1,
                        _ => false,
                    };
                    let lhs = if neg_mul { -a } else { a };
                    let rhs = if neg_acc { -c } else { c };
                    assert_eq!(got[lane], lhs.mul_add(b, rhs).to_bits());
                }
            }
        }

        let dm1 = pack_f64([1.25, -3.5]);
        let dm2 = pack_f64([2.0, 4.0]);
        let dacc = pack_f64([10.0, 20.0]);
        for neg_mul in [false, true] {
            for acc_mode in 0u8..4 {
                let got = unpack_f64_bits(vops::fma_pd(dm1, dm2, dacc, neg_mul, acc_mode));
                for lane in 0..2 {
                    let a = f64::from_bits((dm1 >> (lane * 64)) as u64);
                    let b = f64::from_bits((dm2 >> (lane * 64)) as u64);
                    let c = f64::from_bits((dacc >> (lane * 64)) as u64);
                    let neg_acc = match acc_mode {
                        1 => true,
                        2 => lane == 0,
                        3 => lane == 1,
                        _ => false,
                    };
                    let lhs = if neg_mul { -a } else { a };
                    let rhs = if neg_acc { -c } else { c };
                    assert_eq!(got[lane], lhs.mul_add(b, rhs).to_bits());
                }
            }
        }

        // Verify pinned FMLA operand order on dual-qNaN `(m1, m2)` with finite `acc`:
        // ARM64 `fmla vd, vn, vm` selects `vn` (`m1`) before `vm` (`m2`).
        let qnan1 = f32::from_bits(0x7FF0_0000);
        let qnan2 = f32::from_bits(0xFFFF_FFFF);
        let dual_ps = unpack_f32_bits(vops::fma_ps(
            pack_f32([qnan1, qnan2, 1.0, qnan1]),
            pack_f32([qnan2, qnan1, qnan2, 2.0]),
            pack_f32([1.0, 2.0, 3.0, qnan2]),
            false,
            0,
        ));
        assert_eq!(dual_ps[0], 0x7FF0_0000);
        assert_eq!(dual_ps[1], 0xFFFF_FFFF);
        assert_eq!(dual_ps[2], 0xFFFF_FFFF);
        assert_eq!(dual_ps[3], 0xFFFF_FFFF); // acc NaN wins first in ARM64 FMLA/FMADD
    }

    #[test]
    fn simd_fp_cmp_ps_and_pd_all_16_predicates_with_nan() {
        let qnan = f32::from_bits(0x7FC0_0123);
        let a = pack_f32([-0.0, 1.0, 5.0, qnan]);
        let b = pack_f32([0.0, 2.0, 3.0, 1.0]);
        for pred in 0u8..32 {
            let got = unpack_f32_bits(vops::cmp_ps(a, b, pred));
            for lane in 0..4 {
                let fa = f32::from_bits((a >> (lane * 32)) as u32) as f64;
                let fb = f32::from_bits((b >> (lane * 32)) as u32) as f64;
                let exp = if sse_cmp_pred(fa, fb, pred) { u32::MAX } else { 0 };
                assert_eq!(got[lane], exp, "cmp_ps pred={pred} lane={lane}");
            }
        }

        let dnan = f64::from_bits(0x7FF8_0000_0000_0007);
        let pairs = [
            (pack_f64([-0.0, 2.0]), pack_f64([0.0, 1.0])),
            (pack_f64([1.0, dnan]), pack_f64([2.0, 1.0])),
            (pack_f64([f64::NEG_INFINITY, 1.0]), pack_f64([f64::INFINITY, dnan])),
        ];
        for &(da, db) in &pairs {
            for pred in 0u8..32 {
                let got = unpack_f64_bits(vops::cmp_pd(da, db, pred));
                for lane in 0..2 {
                    let fa = f64::from_bits((da >> (lane * 64)) as u64);
                    let fb = f64::from_bits((db >> (lane * 64)) as u64);
                    let exp = if sse_cmp_pred(fa, fb, pred) { u64::MAX } else { 0 };
                    assert_eq!(got[lane], exp, "cmp_pd pred={pred} lane={lane}");
                }
            }
        }
    }

    #[test]
    fn simd_fp_hadd_hsub_addsub_nan_rules() {
        let qnan_a = f32::from_bits(0x7FC0_00AA);
        let snan_b = f32::from_bits(0x7F80_00BB);
        let a = pack_f32([f32::INFINITY, qnan_a, 10.0, 3.0]);
        let b = pack_f32([f32::INFINITY, snan_b, 4.0, 2.0]);
        // addsubps: [inf - inf -> -qNaN, qnan_a + snan_b -> qnan_a, 10 - 4, 3 + 2]
        let as_res = unpack_f32_bits(vops::addsub_ps(a, b));
        assert_eq!(as_res[0], 0xFFC0_0000);
        assert_eq!(as_res[1], 0x7FC0_00AA);
        assert_eq!(as_res[2], 6.0f32.to_bits());
        assert_eq!(as_res[3], 5.0f32.to_bits());

        // hsubps: [inf - qnan_a -> qnan_a, 10 - 3, inf - snan_b -> quiet(snan_b), 4 - 2]
        let hs_res = unpack_f32_bits(vops::hadd_sub_ps(a, b, false));
        assert_eq!(hs_res[0], 0x7FC0_00AA);
        assert_eq!(hs_res[1], 7.0f32.to_bits());
        assert_eq!(hs_res[2], 0x7FC0_00BB);
        assert_eq!(hs_res[3], 2.0f32.to_bits());

        // haddpd with inf + (-inf) -> -qNaN indefinite
        let da = pack_f64([f64::INFINITY, f64::NEG_INFINITY]);
        let db = pack_f64([f64::from_bits(0x7FF8_0000_0000_0011), f64::from_bits(0x7FF0_0000_0000_0022)]);
        let hd = unpack_f64_bits(vops::hadd_sub_pd(da, db, true));
        assert_eq!(hd[0], 0xFFF8_0000_0000_0000);
        assert_eq!(hd[1], 0x7FF8_0000_0000_0011);
    }

    #[test]
    fn simd_fp_dpps_mask_and_pairwise_sum() {
        // Lane 3 is NaN * Inf, masked OFF by imm[7] = 0 -> must contribute +0.0!
        let a = pack_f32([1.0, 2.0, 3.0, f32::NAN]);
        let b = pack_f32([4.0, 5.0, 6.0, f32::INFINITY]);
        // Mask: multiply lanes 0,1,2 (0b0111 << 4 = 0x70), store to lanes 0 and 2 (0b0101 = 0x05).
        let got = unpack_f32_bits(vops::dp_ps(a, b, 0x75));
        let sum = (1.0f32 * 4.0 + 2.0 * 5.0) + (3.0 * 6.0 + 0.0);
        assert_eq!(got, [sum.to_bits(), 0, sum.to_bits(), 0]);
    }

    #[test]
    fn simd_fp_pmaddwd_pmaddubsw_psadbw_edges() {
        // pmaddwd: i16::MIN * i16::MIN + i16::MIN * i16::MIN wraps to i32::MIN (0x8000_0000).
        let min16 = i16::MIN as u16 as u128;
        let a = min16 | (min16 << 16) | (3u128 << 32) | (((-4i16) as u16 as u128) << 48);
        let b = min16 | (min16 << 16) | (5u128 << 32) | (2u128 << 48);
        let wd = vops::pmaddwd(a, b);
        assert_eq!(wd as u32, 0x8000_0000);
        assert_eq!((wd >> 32) as u32, 7u32); // 3*5 + (-4)*2 = 7

        // pmaddubsw: unsigned * signed with positive & negative saturation.
        let ua: u128 = 255 | (255u128 << 8) | (255u128 << 16) | (255u128 << 24);
        let sb: u128 = (127u8 as u128)
            | ((127u8 as u128) << 8)
            | (((-128i8) as u8 as u128) << 16)
            | (((-128i8) as u8 as u128) << 24);
        let ubsw = vops::pmaddubsw(ua, sb);
        assert_eq!(ubsw as u16 as i16, i16::MAX); // 255*127*2 = 64770 -> 32767
        assert_eq!((ubsw >> 16) as u16 as i16, i16::MIN); // 255*(-128)*2 = -65280 -> -32768

        // psadbw: max diff 8*255 = 2040 in low half, 8*1 = 8 in high half.
        let pa: u128 = 0xFFFF_FFFF_FFFF_FFFFu128 | (0x0202_0202_0202_0202u128 << 64);
        let pb: u128 = 0x0101_0101_0101_0101u128 << 64;
        let sad = vops::psadbw(pa, pb);
        assert_eq!(sad as u64, 2040);
        assert_eq!((sad >> 64) as u64, 8);
    }

    #[test]
    fn simd_fp_vpshift_vd_vq_large_counts() {
        // Specifically exercises counts >= 256 (low byte == 0) to verify ARM NEON vshlq clamping.
        let a: u128 = 0x8000_0001u128
            | (0x8000_0000u128 << 32)
            | (0x1234_5678u128 << 64)
            | (0x8000_0004u128 << 96);
        let c: u128 = 1u128 | (31u128 << 32) | (256u128 << 64) | ((u32::MAX as u128) << 96);
        let sll = vops::vpshift_vd(a, c, 0);
        assert_eq!(sll as u32, 2);
        assert_eq!((sll >> 32) as u32, 0);
        assert_eq!((sll >> 64) as u32, 0);
        assert_eq!((sll >> 96) as u32, 0);

        let srl = vops::vpshift_vd(a, c, 1);
        assert_eq!(srl as u32, 0x4000_0000);
        assert_eq!((srl >> 32) as u32, 1);
        assert_eq!((srl >> 64) as u32, 0);
        assert_eq!((srl >> 96) as u32, 0);

        let sra = vops::vpshift_vd(a, c, 2);
        assert_eq!(sra as u32, 0xC000_0000);
        assert_eq!((sra >> 32) as u32, 0xFFFF_FFFF);
        assert_eq!((sra >> 64) as u32, 0); // positive >> 31 == 0
        assert_eq!((sra >> 96) as u32, 0xFFFF_FFFF); // negative >> 31 == -1

        let qa: u128 = 0x8000_0000_0000_0001u128 | (0xDEAD_BEEF_CAFE_BABEu128 << 64);
        let qc: u128 = 63u128 | (256u128 << 64);
        let qsll = vops::vpshift_vq(qa, qc, false);
        assert_eq!(qsll as u64, 0x8000_0000_0000_0000);
        assert_eq!((qsll >> 64) as u64, 0);
        let qsrl = vops::vpshift_vq(qa, qc, true);
        assert_eq!(qsrl as u64, 1);
        assert_eq!((qsrl >> 64) as u64, 0);
    }

    #[test]
    fn simd_fp_vpermd_blends_and_pminmax() {
        let t_lo: u128 = 0x10u128 | (0x20u128 << 32) | (0x30u128 << 64) | (0x40u128 << 96);
        let t_hi: u128 = 0x50u128 | (0x60u128 << 32) | (0x70u128 << 64) | (0x80u128 << 96);
        // Indices include high bits (should mask with & 7).
        let i_lo: u128 = 7u128 | (14u128 << 32) | (0u128 << 64) | (5u128 << 96);
        let i_hi: u128 = 3u128 | (1u128 << 32) | (4u128 << 64) | (2u128 << 96);
        let (r_lo, r_hi) = vops::vpermd_ymm(i_lo, i_hi, t_lo, t_hi);
        assert_eq!(r_lo, 0x80u128 | (0x70u128 << 32) | (0x10u128 << 64) | (0x60u128 << 96));
        assert_eq!(r_hi, 0x40u128 | (0x20u128 << 32) | (0x50u128 << 64) | (0x30u128 << 96));

        // Signed vs unsigned min/max edge: 0x80 (-128) vs 0x7F (+127).
        let a: u128 = 0x80u128 | (0x8000u128 << 16) | (0x8000_0000u128 << 32);
        let b: u128 = 0x7Fu128 | (0x7FFFu128 << 16) | (0x7FFF_FFFFu128 << 32);
        assert_eq!(vops::pminsb(a, b) as u8, 0x80);
        assert_eq!(vops::pmaxsb(a, b) as u8, 0x7F);
        assert_eq!(vops::pminub(a, b) as u8, 0x7F);
        assert_eq!(vops::pmaxub(a, b) as u8, 0x80);
        assert_eq!((vops::pminsw(a, b) >> 16) as u16, 0x8000);
        assert_eq!((vops::pminuw(a, b) >> 16) as u16, 0x7FFF);
        assert_eq!((vops::pminsd(a, b) >> 32) as u32, 0x8000_0000);
        assert_eq!((vops::pminud(a, b) >> 32) as u32, 0x7FFF_FFFF);

        // Blendvps & blendvpd only inspect sign bit.
        let mask_ps: u128 = 0x7FFF_FFFFu128 | (0x8000_0000u128 << 32);
        assert_eq!(vops::blendvps(t_lo, t_hi, mask_ps), 0x10u128 | (0x60u128 << 32) | (0x30u128 << 64) | (0x40u128 << 96));
        assert_eq!(vops::pblendw_imm(0, u128::MAX, 0b1010_0101), 0xFFFF_0000_FFFF_0000_0000_FFFF_0000_FFFF);
    }

    #[test]
    fn simd_fp_crc32_pdep_pext_and_vex128_zeroing() {
        let mut cpu = Cpu::new_zeroed();
        // CRC32C of "12345678" (8 bytes, initial ~0 -> final ^ ~0 == 0xE3069283 / 2-step check)
        // Standard check: crc32 eax, ecx (F2 0F 38 F1 C1) with initial 0xFFFF_FFFF and 0x3433_3231
        cpu.gpr[0] = 0xFFFF_FFFF;
        cpu.gpr[1] = 0x3433_3231; // "1234"
        exec_bytes(&mut cpu, &[0xF2, 0x0F, 0x38, 0xF1, 0xC1]); // crc32 eax, ecx
        let mut expected = 0xFFFF_FFFFu32;
        for &byte in b"1234" {
            expected ^= byte as u32;
            for _ in 0..8 {
                expected = if expected & 1 != 0 { (expected >> 1) ^ 0x82F6_3B78 } else { expected >> 1 };
            }
        }
        assert_eq!(cpu.gpr[0], expected as u64);

        // PDEP r64, r64, r64 (C4 E2 F3 F5 C2 -> pdep rax, rcx, rdx)
        cpu.gpr[1] = 0b1011;
        cpu.gpr[2] = 0x0001_0010_0100_1000;
        exec_bytes(&mut cpu, &[0xC4, 0xE2, 0xF3, 0xF5, 0xC2]);
        assert_eq!(cpu.gpr[0], 0x0001_0000_0100_1000);

        // PEXT r64, r64, r64 (C4 E2 F2 F5 C2 -> pext rax, rcx, rdx)
        cpu.gpr[1] = 0x0001_0000_0100_1000;
        cpu.gpr[2] = 0x0001_0010_0100_1000;
        exec_bytes(&mut cpu, &[0xC4, 0xE2, 0xF2, 0xF5, 0xC2]);
        assert_eq!(cpu.gpr[0], 0b1011);

        // Legacy SSE ADDPS xmm0, xmm1 (0F 58 C1) MUST preserve ymm_hi[0].
        cpu.xmm[0] = pack_f32([1.0, 2.0, 3.0, 4.0]);
        cpu.xmm[1] = pack_f32([10.0, 20.0, 30.0, 40.0]);
        cpu.ymm_hi[0] = 0xDEAD_BEEF_CAFE_BABE_1111_2222_3333_4444;
        exec_bytes(&mut cpu, &[0x0F, 0x58, 0xC1]);
        assert_eq!(cpu.xmm[0], pack_f32([11.0, 22.0, 33.0, 44.0]));
        assert_eq!(cpu.ymm_hi[0], 0xDEAD_BEEF_CAFE_BABE_1111_2222_3333_4444);

        // VEX.128 VADDPS xmm0, xmm0, xmm1 (C5 F8 58 C1) MUST zero ymm_hi[0].
        exec_bytes(&mut cpu, &[0xC5, 0xF8, 0x58, 0xC1]);
        assert_eq!(cpu.ymm_hi[0], 0);

        // VEX.128 VBLENDPS xmm0, xmm0, xmm1, 0x5 (C4 E3 79 0C C1 05) MUST zero ymm_hi[0].
        cpu.ymm_hi[0] = 0x9999_8888_7777_6666;
        exec_bytes(&mut cpu, &[0xC4, 0xE3, 0x79, 0x0C, 0xC1, 0x05]);
        assert_eq!(cpu.ymm_hi[0], 0);

        // VEX.128 VINSERTPS xmm0, xmm0, xmm1, 0x00 (C4 E3 79 21 C1 00) MUST zero ymm_hi[0].
        cpu.ymm_hi[0] = 0xAAAA_BBBB_CCCC_DDDD;
        exec_bytes(&mut cpu, &[0xC4, 0xE3, 0x79, 0x21, 0xC1, 0x00]);
        assert_eq!(cpu.ymm_hi[0], 0);
    }
}
