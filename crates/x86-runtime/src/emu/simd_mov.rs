//! SSE/AVX data movement, shuffle, unpack, blend, and integer SIMD execution for [`super::Cpu`].

use super::simd_kernels as vops;
use super::*;

impl Cpu {
    #[inline]
    pub(super) unsafe fn exec_simd_mov_int(&mut self, mn: Mnemonic, ins_ref: &Instruction) -> bool {
        let ins = ins_ref;
        let cpu = self;
        match mn {
            Mnemonic::Movups | Mnemonic::Movaps | Mnemonic::Movdqu | Mnemonic::Movdqa | Mnemonic::Movapd | Mnemonic::Movupd
            | Mnemonic::Movntdq | Mnemonic::Movntps | Mnemonic::Movntpd | Mnemonic::Movntdqa
            | Mnemonic::Vmovups | Mnemonic::Vmovaps | Mnemonic::Vmovdqu | Mnemonic::Vmovdqa | Mnemonic::Vmovapd | Mnemonic::Vmovupd
            | Mnemonic::Vmovntdq | Mnemonic::Vmovntps | Mnemonic::Vmovntpd | Mnemonic::Vmovntdqa
            | Mnemonic::Lddqu | Mnemonic::Vlddqu => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        if let (Some((d, ymm)), Some(s)) =
                            (vops::xmm_reg_info(ins.op_register(0)), vops::xmm_idx_fast(ins.op_register(1)))
                        {
                            let lo = cpu.xmm[s];
                            let hi = if ymm { cpu.ymm_hi[s] } else { 0 };
                            vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            let lo = cpu.read_u128(a);
                            let hi = if ymm { cpu.read_u128(a.wrapping_add(16)) } else { 0 };
                            vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_u128(a, cpu.xmm[s]);
                            if ymm {
                                cpu.write_u128(a.wrapping_add(16), cpu.ymm_hi[s]);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Shrd | Mnemonic::Shld => {
                let bits = Cpu::op_bits(ins, 0);
                let dst = cpu.read_op(ins, 0, bits);
                let src = cpu.read_op(ins, 1, bits);
                let count_mask = if bits == 64 { 63u32 } else { 31u32 };
                let cnt = (cpu.read_op(ins, 2, 8) as u32) & count_mask;
                if cnt != 0 {
                    let res = if mn == Mnemonic::Shrd {
                        if bits == 64 {
                            (dst >> cnt) | (src << (64 - cnt))
                        } else if bits == 32 {
                            (((dst as u32) >> cnt) | ((src as u32) << (32 - cnt))) as u64
                        } else {
                            let d16 = dst as u16;
                            let s16 = src as u16;
                            ((d16 >> cnt) | (s16 << (16 - cnt))) as u64
                        }
                    } else if bits == 64 {
                        (dst << cnt) | (src >> (64 - cnt))
                    } else if bits == 32 {
                        (((dst as u32) << cnt) | ((src as u32) >> (32 - cnt))) as u64
                    } else {
                        let d16 = dst as u16;
                        let s16 = src as u16;
                        ((d16 << cnt) | (s16 >> (16 - cnt))) as u64
                    };
                    cpu.update_flags_logic(res, bits);
                    cpu.cf = if mn == Mnemonic::Shrd {
                        ((dst >> (cnt - 1)) & 1) != 0
                    } else {
                        ((dst >> (bits - cnt)) & 1) != 0
                    };
                    cpu.write_op(ins, 0, bits, res);
                }
            }
            Mnemonic::Vperm2f128 | Mnemonic::Vperm2i128 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_lo, s1_hi) = vops::load_vec_reg(cpu, ins.op_register(1), true);
                    let (s2_lo, s2_hi) = vops::load_vec_op(cpu, ins, 2, true);
                    let imm = cpu.read_op(ins, 3, 8) as u8;
                    let (lo, hi) = vops::vperm2_128(s1_lo, s1_hi, s2_lo, s2_hi, imm);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
            }
            Mnemonic::Vinsertf128 | Mnemonic::Vinserti128 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let s1 = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d);
                    let (inserted, _) = vops::load_vec_op(cpu, ins, 2, false);
                    let imm = cpu.read_op(ins, 3, 8) & 1;
                    let lo = cpu.xmm[s1];
                    let hi = cpu.ymm_hi[s1];
                    if imm == 0 {
                        cpu.xmm[d] = inserted;
                        cpu.set_ymm_hi(d, hi);
                    } else {
                        cpu.xmm[d] = lo;
                        cpu.set_ymm_hi(d, inserted);
                    }
                }
            }
            Mnemonic::Vextractf128 | Mnemonic::Vextracti128 => {
                let s = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                let imm = cpu.read_op(ins, 2, 8) & 1;
                let val = if imm == 0 { cpu.xmm[s] } else { cpu.ymm_hi[s] };
                match ins.op_kind(0) {
                    OpKind::Register => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            cpu.xmm[d] = val;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    OpKind::Memory => {
                        cpu.write_u128(cpu.eff_addr(ins), val);
                    }
                    _ => {}
                }
            }
            Mnemonic::Pextrb | Mnemonic::Vpextrb | Mnemonic::Pextrw | Mnemonic::Vpextrw
            | Mnemonic::Pextrd | Mnemonic::Vpextrd | Mnemonic::Pextrq | Mnemonic::Vpextrq => {
                let elem_bits: u32 = match mn {
                    Mnemonic::Pextrb | Mnemonic::Vpextrb => 8,
                    Mnemonic::Pextrw | Mnemonic::Vpextrw => 16,
                    Mnemonic::Pextrd | Mnemonic::Vpextrd => 32,
                    _ => 64,
                };
                let s = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                let max_lanes = 128 / elem_bits;
                let lane = (cpu.read_op(ins, 2, 8) as u32) & (max_lanes - 1);
                let mask = if elem_bits >= 64 { u64::MAX as u128 } else { (1u128 << elem_bits) - 1 };
                let val = ((cpu.xmm[s] >> (lane * elem_bits)) & mask) as u64;
                match ins.op_kind(0) {
                    OpKind::Register => {
                        let dst_bits = if elem_bits == 64 { 64 } else { 32 };
                        cpu.write_op(ins, 0, dst_bits, val);
                    }
                    OpKind::Memory => {
                        cpu.write_mem(cpu.eff_addr(ins), elem_bits, val);
                    }
                    _ => {}
                }
            }
            Mnemonic::Pinsrb | Mnemonic::Vpinsrb | Mnemonic::Pinsrw | Mnemonic::Vpinsrw
            | Mnemonic::Pinsrd | Mnemonic::Vpinsrd | Mnemonic::Pinsrq | Mnemonic::Vpinsrq => {
                let elem_bits: u32 = match mn {
                    Mnemonic::Pinsrb | Mnemonic::Vpinsrb => 8,
                    Mnemonic::Pinsrw | Mnemonic::Vpinsrw => 16,
                    Mnemonic::Pinsrd | Mnemonic::Vpinsrd => 32,
                    _ => 64,
                };
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (base_op, src_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let base_val = vops::xmm_idx_fast(ins.op_register(base_op)).map_or(0, |i| cpu.xmm[i]);
                    let src_val = cpu.read_op(ins, src_op, elem_bits);
                    let max_lanes = 128 / elem_bits;
                    let lane = (cpu.read_op(ins, imm_op, 8) as u32) & (max_lanes - 1);
                    let mask = if elem_bits >= 64 { u64::MAX as u128 } else { (1u128 << elem_bits) - 1 };
                    let shift = lane * elem_bits;
                    cpu.xmm[d] = (base_val & !(mask << shift)) | (((src_val as u128) & mask) << shift);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Movq | Mnemonic::Vmovq | Mnemonic::Movd | Mnemonic::Vmovd => {
                let is_64 = matches!(mn, Mnemonic::Movq | Mnemonic::Vmovq);
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        match (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(ins.op_register(1))) {
                            (Some(d), Some(s)) => {
                                cpu.xmm[d] = if is_64 {
                                    cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF
                                } else {
                                    cpu.xmm[s] & 0xFFFF_FFFF
                                };
                            }
                            (Some(d), None) => {
                                let v = cpu.get_reg(ins.op_register(1));
                                cpu.xmm[d] = if is_64 { v as u128 } else { (v as u32) as u128 };
                            }
                            (None, Some(s)) => {
                                let v = if is_64 { cpu.xmm[s] as u64 } else { (cpu.xmm[s] as u32) as u64 };
                                cpu.set_reg(ins.op_register(0), v);
                            }
                            _ => {}
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, if is_64 { 64 } else { 32 }) as u128;
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            if is_64 {
                                cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                            } else {
                                cpu.write_mem(a, 32, cpu.xmm[s] as u32 as u64);
                            }
                        }
                    }
                    _ => {}
                }
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Stmxcsr | Mnemonic::Vstmxcsr => {
                let a = cpu.eff_addr(ins);
                cpu.write_mem(a, 32, cpu.jx.mxcsr as u64);
            }
            Mnemonic::Ldmxcsr | Mnemonic::Vldmxcsr => {
                let a = cpu.eff_addr(ins);
                let m = cpu.read_mem(a, 32) as u32;
                if m >> 16 != 0 {
                    super::xstate::Fault::gp().dispatch(cpu, ins.ip());
                    return true;
                }
                if super::precise_faults() && super::FAULT_PENDING.with(|p| p.get().is_some()) {
                    return true;
                }
                cpu.jx.mxcsr = m;
                super::apply_guest_mxcsr_hw(m);
            }
            Mnemonic::Scasb | Mnemonic::Scasw | Mnemonic::Scasd | Mnemonic::Scasq => {
                let (bits, step) = match mn {
                    Mnemonic::Scasb => (8u32, 1u64),
                    Mnemonic::Scasw => (16, 2),
                    Mnemonic::Scasd => (32, 4),
                    _ => (64, 8),
                };
                let step = if cpu.df { step.wrapping_neg() } else { step };
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let acc = cpu.gpr[0] & mask;
                if ins.has_repne_prefix() {
                    while cpu.gpr[1] != 0 {
                        let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                        cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                        let _ = cpu.update_flags_sub(acc, m, bits);
                        if acc == m {
                            break;
                        }
                    }
                } else if ins.has_repe_prefix() {
                    while cpu.gpr[1] != 0 {
                        let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                        cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                        let _ = cpu.update_flags_sub(acc, m, bits);
                        if acc != m {
                            break;
                        }
                    }
                } else {
                    let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                    let _ = cpu.update_flags_sub(acc, m, bits);
                }
                cpu.last_bits[7] = 64;
                if ins.has_repne_prefix() || ins.has_repe_prefix() {
                    cpu.last_bits[1] = 64;
                }
            }
            Mnemonic::Movlhps | Mnemonic::Vmovlhps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_idx, s2_op) = if ins.op_count() >= 3 {
                        (vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d), 2)
                    } else {
                        (d, 1)
                    };
                    let s2_lo = vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] & 0xFFFF_FFFF_FFFF_FFFF);
                    cpu.xmm[d] = (cpu.xmm[s1_idx] & 0xFFFF_FFFF_FFFF_FFFF) | (s2_lo << 64);
                    if mn == Mnemonic::Vmovlhps {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Movhlps | Mnemonic::Vmovhlps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_idx, s2_op) = if ins.op_count() >= 3 {
                        (vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d), 2)
                    } else {
                        (d, 1)
                    };
                    let s2_hi = vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] >> 64);
                    cpu.xmm[d] = (cpu.xmm[s1_idx] & !0xFFFF_FFFF_FFFF_FFFFu128) | s2_hi;
                    if mn == Mnemonic::Vmovhlps {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Movss | Mnemonic::Vmovss => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        let s_reg = ins.op_register(ins.op_count() - 1);
                        if let (Some(d), Some(s)) = (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(s_reg)) {
                            let hi = if ins.op_count() >= 3 {
                                vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                            } else {
                                cpu.xmm[d]
                            };
                            cpu.xmm[d] = (hi & !0xFFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF);
                            cpu.vex128_zero_upper(ins, d);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, 32) as u128;
                            cpu.vex128_zero_upper(ins, d);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 32, cpu.xmm[s] as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Vmovsd => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        let s_reg = ins.op_register(ins.op_count() - 1);
                        if let (Some(d), Some(s)) = (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(s_reg)) {
                            let hi = if ins.op_count() >= 3 {
                                vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                            } else {
                                cpu.xmm[d]
                            };
                            cpu.xmm[d] = (hi & !0xFFFF_FFFF_FFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF);
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, 64) as u128;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Andn => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let b = cpu.read_op(ins, 2, bits);
                let res = (!a) & b;
                cpu.write_op(ins, 0, bits, res);
                cpu.update_flags_logic(res, bits);
            }
            Mnemonic::Shrx | Mnemonic::Shlx | Mnemonic::Sarx => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(ins, 2, bits) as u32) & mask;
                let res = match mn {
                    Mnemonic::Shlx => a.wrapping_shl(cnt),
                    Mnemonic::Shrx => {
                        let val_mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                        (a & val_mask).wrapping_shr(cnt)
                    }
                    _ => {
                        if bits == 64 {
                            ((a as i64) >> cnt) as u64
                        } else {
                            ((a as i32) >> cnt) as u32 as u64
                        }
                    }
                };
                cpu.write_op(ins, 0, bits, res);
            }
            Mnemonic::Rorx => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(ins, 2, 8) as u32) & mask;
                let res = if bits == 64 {
                    a.rotate_right(cnt)
                } else {
                    (a as u32).rotate_right(cnt) as u64
                };
                cpu.write_op(ins, 0, bits, res);
            }
            Mnemonic::Bzhi => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let idx = (cpu.read_op(ins, 2, bits) & 0xFF) as u32;
                let res = if idx >= bits {
                    cpu.cf = true;
                    a
                } else {
                    cpu.cf = false;
                    if idx == 0 { 0 } else { a & ((1u64 << idx) - 1) }
                };
                cpu.write_op(ins, 0, bits, res);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                cpu.zf = (res & mask) == 0;
                cpu.sf = (res & (1u64 << (bits - 1))) != 0;
                cpu.of = false;
            }
            Mnemonic::Blsr | Mnemonic::Blsmsk | Mnemonic::Blsi => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let res = match mn {
                    Mnemonic::Blsr => a & a.wrapping_sub(1),
                    Mnemonic::Blsmsk => a ^ a.wrapping_sub(1),
                    _ => a & a.wrapping_neg(),
                };
                cpu.write_op(ins, 0, bits, res);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                cpu.zf = (res & mask) == 0;
                cpu.sf = (res & (1u64 << (bits - 1))) != 0;
                cpu.cf = if mn == Mnemonic::Blsi { (a & mask) != 0 } else { (a & mask) == 0 };
                cpu.of = false;
            }
            Mnemonic::Mulx => {
                let bits = Cpu::op_bits(ins, 0);
                let src = cpu.read_op(ins, 2, bits);
                if bits == 64 {
                    let prod = (cpu.gpr[2] as u128).wrapping_mul(src as u128);
                    cpu.write_op(ins, 1, 64, prod as u64);
                    cpu.write_op(ins, 0, 64, (prod >> 64) as u64);
                } else {
                    let prod = (cpu.gpr[2] as u32 as u64).wrapping_mul(src as u32 as u64);
                    cpu.write_op(ins, 1, 32, prod as u32 as u64);
                    cpu.write_op(ins, 0, 32, (prod >> 32) as u32 as u64);
                }
            }
            Mnemonic::Addss | Mnemonic::Subss | Mnemonic::Mulss | Mnemonic::Divss | Mnemonic::Minss | Mnemonic::Maxss
            | Mnemonic::Vaddss | Mnemonic::Vsubss | Mnemonic::Vmulss | Mnemonic::Vdivss | Mnemonic::Vminss | Mnemonic::Vmaxss
            | Mnemonic::Addsd | Mnemonic::Subsd | Mnemonic::Mulsd | Mnemonic::Divsd | Mnemonic::Minsd | Mnemonic::Maxsd
            | Mnemonic::Vaddsd | Mnemonic::Vsubsd | Mnemonic::Vmulsd | Mnemonic::Vdivsd | Mnemonic::Vminsd | Mnemonic::Vmaxsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let a = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let (b, _) = cpu.vsrc(ins, s2_op, false);
                    let op: u8 = match mn {
                        Mnemonic::Addss | Mnemonic::Vaddss | Mnemonic::Addsd | Mnemonic::Vaddsd => 0,
                        Mnemonic::Subss | Mnemonic::Vsubss | Mnemonic::Subsd | Mnemonic::Vsubsd => 1,
                        Mnemonic::Mulss | Mnemonic::Vmulss | Mnemonic::Mulsd | Mnemonic::Vmulsd => 2,
                        Mnemonic::Divss | Mnemonic::Vdivss | Mnemonic::Divsd | Mnemonic::Vdivsd => 3,
                        Mnemonic::Minss | Mnemonic::Vminss | Mnemonic::Minsd | Mnemonic::Vminsd => 4,
                        _ => 5,
                    };
                    let is_sd = matches!(
                        mn,
                        Mnemonic::Addsd | Mnemonic::Subsd | Mnemonic::Mulsd | Mnemonic::Divsd | Mnemonic::Minsd | Mnemonic::Maxsd
                            | Mnemonic::Vaddsd | Mnemonic::Vsubsd | Mnemonic::Vmulsd | Mnemonic::Vdivsd | Mnemonic::Vminsd
                            | Mnemonic::Vmaxsd
                    );
                    cpu.xmm[d] = if is_sd {
                        let r = sse_arith64(op, f64::from_bits(a as u64), f64::from_bits(b as u64));
                        (a & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128)
                    } else {
                        let r = sse_arith32(op, f32::from_bits(a as u32), f32::from_bits(b as u32));
                        (a & !0xFFFF_FFFF) | (r.to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Sqrtss | Mnemonic::Vsqrtss | Mnemonic::Sqrtsd | Mnemonic::Vsqrtsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let (b, _) = cpu.vsrc(ins, src_op, false);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Sqrtsd | Mnemonic::Vsqrtsd) {
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | (sse_sqrt64(f64::from_bits(b as u64)).to_bits() as u128)
                    } else {
                        (base & !0xFFFF_FFFF) | (sse_sqrt32(f32::from_bits(b as u32)).to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Comiss | Mnemonic::Ucomiss | Mnemonic::Vcomiss | Mnemonic::Vucomiss => {
                let a_bits = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i] as u32);
                let b_bits = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i] as u32),
                    OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                    _ => 0,
                };
                let fa = f32::from_bits(a_bits);
                let fb = f32::from_bits(b_bits);
                let unord = !(fa <= fb) & !(fa > fb);
                cpu.sf = false;
                cpu.of = false;
                cpu.zf = !(fa < fb) & !(fa > fb);
                cpu.cf = !(fa >= fb);
                cpu.pfv = (!unord) as u8;
            }
            Mnemonic::Comisd | Mnemonic::Ucomisd | Mnemonic::Vcomisd | Mnemonic::Vucomisd => {
                let a_bits = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i] as u64);
                let b_bits = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i] as u64),
                    OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                    _ => 0,
                };
                let fa = f64::from_bits(a_bits);
                let fb = f64::from_bits(b_bits);
                let unord = !(fa <= fb) & !(fa > fb);
                cpu.sf = false;
                cpu.of = false;
                cpu.zf = !(fa < fb) & !(fa > fb);
                cpu.cf = !(fa >= fb);
                cpu.pfv = (!unord) as u8;
            }
            Mnemonic::Cvtdq2ps | Mnemonic::Vcvtdq2ps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (v_lo, v_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::cvtdq2ps(v_lo);
                    let hi = if ymm { vops::cvtdq2ps(v_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Cvtsi2ss | Mnemonic::Vcvtsi2ss | Mnemonic::Cvtsi2sd | Mnemonic::Vcvtsi2sd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let bits = Cpu::op_bits(ins, src_op);
                    let v = cpu.read_op(ins, src_op, bits);
                    let iv = if bits == 64 { v as i64 } else { v as u32 as i32 as i64 };
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtsi2ss | Mnemonic::Vcvtsi2ss) {
                        (base & !0xFFFF_FFFF) | ((iv as f32).to_bits() as u128)
                    } else {
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | ((iv as f64).to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Cvtss2sd | Mnemonic::Vcvtss2sd | Mnemonic::Cvtsd2ss | Mnemonic::Vcvtsd2ss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let (s, _) = cpu.vsrc(ins, src_op, false);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtss2sd | Mnemonic::Vcvtss2sd) {
                        let f = f32::from_bits(s as u32) as f64;
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | (f.to_bits() as u128)
                    } else {
                        let f = f64::from_bits(s as u64) as f32;
                        (base & !0xFFFF_FFFF) | (f.to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Cvttss2si | Mnemonic::Cvtss2si | Mnemonic::Vcvttss2si | Mnemonic::Vcvtss2si
            | Mnemonic::Cvttsd2si | Mnemonic::Cvtsd2si | Mnemonic::Vcvttsd2si | Mnemonic::Vcvtsd2si => {
                let dst_bits = Cpu::op_bits(ins, 0);
                let (s, _) = cpu.vsrc(ins, 1, false);
                let is_sd = matches!(mn, Mnemonic::Cvttsd2si | Mnemonic::Cvtsd2si | Mnemonic::Vcvttsd2si | Mnemonic::Vcvtsd2si);
                let trunc = matches!(mn, Mnemonic::Cvttss2si | Mnemonic::Vcvttss2si | Mnemonic::Cvttsd2si | Mnemonic::Vcvttsd2si);
                let f = if is_sd { f64::from_bits(s as u64) } else { f32::from_bits(s as u32) as f64 };
                let iv = if dst_bits == 64 { sse_cvt_i64(f, trunc) } else { sse_cvt_i32(f, trunc) as u64 };
                cpu.write_op(ins, 0, dst_bits, iv);
            }
            Mnemonic::Psrlw
            | Mnemonic::Vpsrlw
            | Mnemonic::Psllw
            | Mnemonic::Vpsllw
            | Mnemonic::Psraw
            | Mnemonic::Vpsraw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let kind = match mn {
                        Mnemonic::Psraw | Mnemonic::Vpsraw => 2,
                        Mnemonic::Psrlw | Mnemonic::Vpsrlw => 1,
                        _ => 0,
                    };
                    let lo = vops::pshift_w(v_lo, cnt, kind);
                    let hi = if ymm { vops::pshift_w(v_hi, cnt, kind) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrld
            | Mnemonic::Vpsrld
            | Mnemonic::Pslld
            | Mnemonic::Vpslld
            | Mnemonic::Psrad
            | Mnemonic::Vpsrad => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let kind = match mn {
                        Mnemonic::Psrad | Mnemonic::Vpsrad => 2,
                        Mnemonic::Psrld | Mnemonic::Vpsrld => 1,
                        _ => 0,
                    };
                    let lo = vops::pshift_d(v_lo, cnt, kind);
                    let hi = if ymm { vops::pshift_d(v_hi, cnt, kind) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrlq | Mnemonic::Vpsrlq | Mnemonic::Psllq | Mnemonic::Vpsllq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let is_shr = matches!(mn, Mnemonic::Psrlq | Mnemonic::Vpsrlq);
                    let lo = vops::pshift_q(v_lo, cnt, is_shr);
                    let hi = if ymm { vops::pshift_q(v_hi, cnt, is_shr) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrldq | Mnemonic::Vpsrldq | Mnemonic::Pslldq | Mnemonic::Vpslldq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_op(cpu, ins, src_op, ymm);
                    let bytes_cnt = (cpu.read_op(ins, cnt_op, 8) as u32).min(16);
                    let is_shr = matches!(mn, Mnemonic::Psrldq | Mnemonic::Vpsrldq);
                    let lo = vops::pshift_dq(v_lo, bytes_cnt, is_shr);
                    let hi = if ymm { vops::pshift_dq(v_hi, bytes_cnt, is_shr) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Paddd | Mnemonic::Vpaddd | Mnemonic::Psubd | Mnemonic::Vpsubd | Mnemonic::Paddq | Mnemonic::Vpaddq
            | Mnemonic::Psubq | Mnemonic::Vpsubq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Paddd | Mnemonic::Vpaddd => (vops::paddd(a_lo, b_lo), if ymm { vops::paddd(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psubd | Mnemonic::Vpsubd => (vops::psubd(a_lo, b_lo), if ymm { vops::psubd(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Paddq | Mnemonic::Vpaddq => (vops::paddq(a_lo, b_lo), if ymm { vops::paddq(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psubq(a_lo, b_lo), if ymm { vops::psubq(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pand | Mnemonic::Vpand | Mnemonic::Andps | Mnemonic::Vandps | Mnemonic::Andpd | Mnemonic::Vandpd
            | Mnemonic::Por | Mnemonic::Vpor | Mnemonic::Orps | Mnemonic::Vorps | Mnemonic::Orpd | Mnemonic::Vorpd
            | Mnemonic::Pandn | Mnemonic::Vpandn | Mnemonic::Andnps | Mnemonic::Vandnps | Mnemonic::Andnpd | Mnemonic::Vandnpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Por | Mnemonic::Vpor | Mnemonic::Orps | Mnemonic::Vorps | Mnemonic::Orpd | Mnemonic::Vorpd => {
                            (vops::v_or(a_lo, b_lo), if ymm { vops::v_or(a_hi, b_hi) } else { 0 })
                        }
                        Mnemonic::Pandn | Mnemonic::Vpandn | Mnemonic::Andnps | Mnemonic::Vandnps | Mnemonic::Andnpd | Mnemonic::Vandnpd => {
                            (vops::v_andn(a_lo, b_lo), if ymm { vops::v_andn(a_hi, b_hi) } else { 0 })
                        }
                        _ => (vops::v_and(a_lo, b_lo), if ymm { vops::v_and(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pshufd | Mnemonic::Vpshufd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s_lo, s_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = ins.immediate8();
                    let lo = vops::pshufd(s_lo, imm);
                    let hi = if ymm { vops::pshufd(s_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Shufps | Mnemonic::Vshufps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::shufps(a_lo, b_lo, imm);
                    let hi = if ymm { vops::shufps(a_hi, b_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpcklqdq | Mnemonic::Vpunpcklqdq | Mnemonic::Unpcklpd | Mnemonic::Vunpcklpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpcklqdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpcklqdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Unpcklps | Mnemonic::Vunpcklps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckldq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckldq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpcklbw | Mnemonic::Vpunpcklbw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpcklbw(a_lo, b_lo);
                    let hi = if ymm { vops::punpcklbw(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psignb | Mnemonic::Vpsignb | Mnemonic::Psignw | Mnemonic::Vpsignw | Mnemonic::Psignd
            | Mnemonic::Vpsignd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Psignb | Mnemonic::Vpsignb => (vops::psignb(a_lo, b_lo), if ymm { vops::psignb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psignw | Mnemonic::Vpsignw => (vops::psignw(a_lo, b_lo), if ymm { vops::psignw(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psignd(a_lo, b_lo), if ymm { vops::psignd(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqb | Mnemonic::Vpcmpeqb | Mnemonic::Pcmpgtb | Mnemonic::Vpcmpgtb => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqb | Mnemonic::Vpcmpeqb) {
                        (vops::pcmpeqb(a_lo, b_lo), if ymm { vops::pcmpeqb(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtb(a_lo, b_lo), if ymm { vops::pcmpgtb(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqw | Mnemonic::Vpcmpeqw | Mnemonic::Pcmpgtw | Mnemonic::Vpcmpgtw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqw | Mnemonic::Vpcmpeqw) {
                        (vops::pcmpeqw(a_lo, b_lo), if ymm { vops::pcmpeqw(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtw(a_lo, b_lo), if ymm { vops::pcmpgtw(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqd | Mnemonic::Vpcmpeqd | Mnemonic::Pcmpgtd | Mnemonic::Vpcmpgtd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqd | Mnemonic::Vpcmpeqd) {
                        (vops::pcmpeqd(a_lo, b_lo), if ymm { vops::pcmpeqd(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtd(a_lo, b_lo), if ymm { vops::pcmpgtd(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqq | Mnemonic::Vpcmpeqq | Mnemonic::Pcmpgtq | Mnemonic::Vpcmpgtq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqq | Mnemonic::Vpcmpeqq) {
                        (vops::pcmpeqq(a_lo, b_lo), if ymm { vops::pcmpeqq(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtq(a_lo, b_lo), if ymm { vops::pcmpgtq(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pmovmskb | Mnemonic::Vpmovmskb => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let mask = if ymm {
                        vops::pmovmskb_ymm(cpu.xmm[s], cpu.ymm_hi[s])
                    } else {
                        vops::pmovmskb(cpu.xmm[s])
                    };
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Movmskps | Mnemonic::Vmovmskps => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let mask = if ymm {
                        vops::movmskps_ymm(cpu.xmm[s], cpu.ymm_hi[s])
                    } else {
                        vops::movmskps(cpu.xmm[s])
                    };
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Movmskpd | Mnemonic::Vmovmskpd => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let mask = if ymm {
                        vops::movmskpd_ymm(cpu.xmm[s], cpu.ymm_hi[s])
                    } else {
                        vops::movmskpd(cpu.xmm[s])
                    };
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Vpbroadcastb | Mnemonic::Vpbroadcastw | Mnemonic::Vpbroadcastd | Mnemonic::Vpbroadcastq
            | Mnemonic::Vbroadcastss | Mnemonic::Vbroadcastsd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let elem_bits: u32 = match mn {
                        Mnemonic::Vpbroadcastb => 8,
                        Mnemonic::Vpbroadcastw => 16,
                        Mnemonic::Vpbroadcastd | Mnemonic::Vbroadcastss => 32,
                        _ => 64,
                    };
                    let val = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), elem_bits) as u128,
                        _ => 0,
                    };
                    let out = vops::pbroadcast(val, elem_bits);
                    cpu.xmm[d] = out;
                    if ymm {
                        cpu.set_ymm_hi(d, out);
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Pshufb | Mnemonic::Vpshufb => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::pshufb(a_lo, b_lo);
                    let hi = if ymm { vops::pshufb(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Paddb | Mnemonic::Vpaddb | Mnemonic::Psubb | Mnemonic::Vpsubb | Mnemonic::Paddw | Mnemonic::Vpaddw
            | Mnemonic::Psubw | Mnemonic::Vpsubw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Paddb | Mnemonic::Vpaddb => (vops::paddb(a_lo, b_lo), if ymm { vops::paddb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psubb | Mnemonic::Vpsubb => (vops::psubb(a_lo, b_lo), if ymm { vops::psubb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Paddw | Mnemonic::Vpaddw => (vops::paddw(a_lo, b_lo), if ymm { vops::paddw(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psubw(a_lo, b_lo), if ymm { vops::psubw(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Ptest | Mnemonic::Vptest => {
                let s1_reg = ins.op_register(0);
                let ymm = vops::is_ymm_fast(s1_reg);
                let (a_lo, a_hi) = vops::load_vec_reg(cpu, s1_reg, ymm);
                let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                if ymm {
                    cpu.zf = ((a_lo & b_lo) | (a_hi & b_hi)) == 0;
                    cpu.cf = (((!a_lo) & b_lo) | ((!a_hi) & b_hi)) == 0;
                } else {
                    cpu.zf = (a_lo & b_lo) == 0;
                    cpu.cf = ((!a_lo) & b_lo) == 0;
                }
                cpu.sf = false;
                cpu.of = false;
                cpu.set_pf(false);
            }
            Mnemonic::Vzeroupper => {
                cpu.clear_all_ymm_hi();
            }
            Mnemonic::Vzeroall => {
                cpu.xmm = [0; 16];
                cpu.clear_all_ymm_hi();
            }
            Mnemonic::Roundss | Mnemonic::Vroundss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let b_bits = match ins.op_kind(s2_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u32),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                        _ => 0,
                    };
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let r = vops::round_f32(f32::from_bits(b_bits), imm, cpu.jx.mxcsr);
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF) | (r.to_bits() as u128);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Roundsd | Mnemonic::Vroundsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let b_bits = match ins.op_kind(s2_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => 0,
                    };
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let r = vops::round_f64(f64::from_bits(b_bits), imm, cpu.jx.mxcsr);
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Roundps | Mnemonic::Vroundps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let d_lo = vops::round_ps(lo, imm, cpu.jx.mxcsr);
                    let d_hi = if ymm { vops::round_ps(hi, imm, cpu.jx.mxcsr) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, d_lo, d_hi);
                }
            }
            Mnemonic::Roundpd | Mnemonic::Vroundpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let d_lo = vops::round_pd(lo, imm, cpu.jx.mxcsr);
                    let d_hi = if ymm { vops::round_pd(hi, imm, cpu.jx.mxcsr) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, d_lo, d_hi);
                }
            }
            Mnemonic::Vcvtph2ps => {
                if let Some((d, is_256)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let raw = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => {
                            let addr = cpu.eff_addr(ins);
                            if is_256 {
                                cpu.read_u128(addr)
                            } else {
                                cpu.read_mem(addr, 64) as u128
                            }
                        }
                        _ => 0,
                    };
                    let (d_lo, d_hi) = vops::vcvtph2ps(raw, is_256);
                    vops::store_vec_dst(cpu, ins, d, is_256, d_lo, d_hi);
                }
            }
            Mnemonic::Vcvtps2ph => {
                let (s, is_256) = vops::xmm_reg_info(ins.op_register(1)).unwrap_or((0, false));
                let (lo, hi) = (cpu.xmm[s], if is_256 { cpu.ymm_hi[s] } else { 0 });
                let imm = ins.immediate8();
                let packed = vops::vcvtps2ph(lo, hi, is_256, imm, cpu.jx.mxcsr);
                match ins.op_kind(0) {
                    OpKind::Register => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            cpu.xmm[d] = packed;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    OpKind::Memory => {
                        let addr = cpu.eff_addr(ins);
                        if is_256 {
                            cpu.write_u128(addr, packed);
                        } else {
                            cpu.write_mem(addr, 64, packed as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Movhpd | Mnemonic::Vmovhpd | Mnemonic::Movhps | Mnemonic::Vmovhps => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, (cpu.xmm[s] >> 64) as u64);
                        }
                    }
                    (OpKind::Register, _) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let (s1_op, m_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let lo = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i] as u64);
                            let hi = if ins.op_kind(m_op) == OpKind::Memory {
                                cpu.read_mem(cpu.eff_addr(ins), 64)
                            } else {
                                0
                            };
                            cpu.xmm[d] = (lo as u128) | ((hi as u128) << 64);
                            if Cpu::is_vex(ins) {
                                cpu.clear_ymm_hi_reg(d);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Movlpd | Mnemonic::Vmovlpd | Mnemonic::Movlps | Mnemonic::Vmovlps => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                        }
                    }
                    (OpKind::Register, _) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let (s1_op, m_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| (cpu.xmm[i] >> 64) as u64);
                            let lo = if ins.op_kind(m_op) == OpKind::Memory {
                                cpu.read_mem(cpu.eff_addr(ins), 64)
                            } else {
                                0
                            };
                            cpu.xmm[d] = (lo as u128) | ((hi as u128) << 64);
                            if Cpu::is_vex(ins) {
                                cpu.clear_ymm_hi_reg(d);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Addps | Mnemonic::Subps | Mnemonic::Mulps | Mnemonic::Divps | Mnemonic::Minps | Mnemonic::Maxps
            | Mnemonic::Vaddps | Mnemonic::Vsubps | Mnemonic::Vmulps | Mnemonic::Vdivps | Mnemonic::Vminps | Mnemonic::Vmaxps
            | Mnemonic::Addpd | Mnemonic::Subpd | Mnemonic::Mulpd | Mnemonic::Divpd | Mnemonic::Minpd | Mnemonic::Maxpd
            | Mnemonic::Vaddpd | Mnemonic::Vsubpd | Mnemonic::Vmulpd | Mnemonic::Vdivpd | Mnemonic::Vminpd | Mnemonic::Vmaxpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: u8 = match mn {
                        Mnemonic::Addps | Mnemonic::Vaddps | Mnemonic::Addpd | Mnemonic::Vaddpd => 0,
                        Mnemonic::Subps | Mnemonic::Vsubps | Mnemonic::Subpd | Mnemonic::Vsubpd => 1,
                        Mnemonic::Mulps | Mnemonic::Vmulps | Mnemonic::Mulpd | Mnemonic::Vmulpd => 2,
                        Mnemonic::Divps | Mnemonic::Vdivps | Mnemonic::Divpd | Mnemonic::Vdivpd => 3,
                        Mnemonic::Minps | Mnemonic::Vminps | Mnemonic::Minpd | Mnemonic::Vminpd => 4,
                        _ => 5,
                    };
                    let is_pd = matches!(
                        mn,
                        Mnemonic::Addpd | Mnemonic::Subpd | Mnemonic::Mulpd | Mnemonic::Divpd | Mnemonic::Minpd | Mnemonic::Maxpd
                            | Mnemonic::Vaddpd | Mnemonic::Vsubpd | Mnemonic::Vmulpd | Mnemonic::Vdivpd | Mnemonic::Vminpd
                            | Mnemonic::Vmaxpd
                    );
                    let (lo, hi) = if is_pd {
                        (vops::fp_bin_pd(op, a_lo, b_lo), if ymm { vops::fp_bin_pd(op, a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::fp_bin_ps(op, a_lo, b_lo), if ymm { vops::fp_bin_ps(op, a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Sqrtps | Mnemonic::Vsqrtps | Mnemonic::Sqrtpd | Mnemonic::Vsqrtpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s_lo, s_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Sqrtpd | Mnemonic::Vsqrtpd) {
                        (vops::sqrt_pd(s_lo), if ymm { vops::sqrt_pd(s_hi) } else { 0 })
                    } else {
                        (vops::sqrt_ps(s_lo), if ymm { vops::sqrt_ps(s_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Blendps | Mnemonic::Vblendps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::blendps_imm(a_lo, b_lo, imm & 0xF);
                    let hi = if ymm { vops::blendps_imm(a_hi, b_hi, imm >> 4) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Blendpd | Mnemonic::Vblendpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::blendpd_imm(a_lo, b_lo, imm);
                    let hi = if ymm { vops::blendpd_imm(a_hi, b_hi, imm >> 2) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Insertps | Mnemonic::Vinsertps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let a = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let (b_val, from_mem) = match ins.op_kind(s2_op) {
                        OpKind::Register => (
                            vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i]),
                            false,
                        ),
                        OpKind::Memory => (cpu.read_mem(cpu.eff_addr(ins), 32) as u128, true),
                        _ => (0, false),
                    };
                    cpu.xmm[d] = vops::insertps(a, b_val, imm, from_mem);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Pshuflw | Mnemonic::Vpshuflw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::pshuflw(src_lo, imm);
                    let hi = if ymm { vops::pshuflw(src_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pshufhw | Mnemonic::Vpshufhw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::pshufhw(src_lo, imm);
                    let hi = if ymm { vops::pshufhw(src_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Vpermilps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    if matches!(ins.op_kind(2), OpKind::Immediate8) {
                        let imm = cpu.read_op(ins, 2, 8) as u8;
                        let lo = vops::pshufd(src_lo, imm);
                        let hi = if ymm { vops::pshufd(src_hi, imm) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    } else {
                        let (ctrl_lo, ctrl_hi) = vops::load_vec_op(cpu, ins, 2, ymm);
                        let lo = vops::vpermilps_var(src_lo, ctrl_lo);
                        let hi = if ymm { vops::vpermilps_var(src_hi, ctrl_hi) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    }
                }
            }
            Mnemonic::Vpermilpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    if matches!(ins.op_kind(2), OpKind::Immediate8) {
                        let imm = cpu.read_op(ins, 2, 8) as u8;
                        let lo = vops::shufpd(src_lo, src_lo, imm);
                        let hi = if ymm { vops::shufpd(src_hi, src_hi, imm >> 2) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    } else {
                        let (ctrl_lo, ctrl_hi) = vops::load_vec_op(cpu, ins, 2, ymm);
                        let lo = vops::vpermilpd_var(src_lo, ctrl_lo);
                        let hi = if ymm { vops::vpermilpd_var(src_hi, ctrl_hi) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    }
                }
            }
            Mnemonic::Cvttps2dq | Mnemonic::Vcvttps2dq | Mnemonic::Cvtps2dq | Mnemonic::Vcvtps2dq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let trunc = matches!(mn, Mnemonic::Cvttps2dq | Mnemonic::Vcvttps2dq);
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::cvtps2dq(src_lo, trunc);
                    let hi = if ymm { vops::cvtps2dq(src_hi, trunc) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Unpckhpd | Mnemonic::Vunpckhpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckhqdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckhqdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpistri
            | Mnemonic::Vpcmpistri
            | Mnemonic::Pcmpistrm
            | Mnemonic::Vpcmpistrm
            | Mnemonic::Pcmpestri
            | Mnemonic::Vpcmpestri
            | Mnemonic::Pcmpestrm
            | Mnemonic::Vpcmpestrm
            | Mnemonic::Pcmpestri64 | Mnemonic::Vpcmpestri64
            | Mnemonic::Pcmpestrm64 | Mnemonic::Vpcmpestrm64 => {
                let long = matches!(mn, Mnemonic::Pcmpestri64 | Mnemonic::Vpcmpestri64 | Mnemonic::Pcmpestrm64 | Mnemonic::Vpcmpestrm64);
                let is_explicit = long || matches!(
                    mn,
                    Mnemonic::Pcmpestri
                        | Mnemonic::Vpcmpestri
                        | Mnemonic::Pcmpestrm
                        | Mnemonic::Vpcmpestrm
                );
                let is_mask = matches!(
                    mn,
                    Mnemonic::Pcmpestrm64 | Mnemonic::Vpcmpestrm64 | Mnemonic::Pcmpistrm
                        | Mnemonic::Vpcmpistrm
                        | Mnemonic::Pcmpestrm
                        | Mnemonic::Vpcmpestrm
                );
                let a = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i]);
                let b = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                    OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                    _ => 0,
                };
                vops::pcmpstr(cpu, a, b, ins.immediate8(), is_explicit, is_mask, long, Cpu::is_vex(ins));
            }
            Mnemonic::Cvtps2pd | Mnemonic::Vcvtps2pd | Mnemonic::Cvtdq2pd | Mnemonic::Vcvtdq2pd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    // Source is m64/xmm[63:0] (xmm dest) or m128/xmm (ymm dest): preserve exact memory read width.
                    let (raw, _) = cpu.vsrc(ins, 1, false);
                    let (lo, hi) = if matches!(mn, Mnemonic::Cvtdq2pd | Mnemonic::Vcvtdq2pd) {
                        vops::cvtdq2pd(raw, ymm)
                    } else {
                        vops::cvtps2pd(raw, ymm)
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Cvtpd2ps | Mnemonic::Vcvtpd2ps | Mnemonic::Cvttpd2dq | Mnemonic::Vcvttpd2dq | Mnemonic::Cvtpd2dq
            | Mnemonic::Vcvtpd2dq => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_256 = match ins.op_kind(1) {
                        OpKind::Register => vops::is_ymm_fast(ins.op_register(1)),
                        OpKind::Memory => ins.memory_size().size() == 32,
                        _ => false,
                    };
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, src_256);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtpd2ps | Mnemonic::Vcvtpd2ps) {
                        vops::cvtpd2ps(lo, hi, src_256)
                    } else {
                        let trunc = matches!(mn, Mnemonic::Cvttpd2dq | Mnemonic::Vcvttpd2dq);
                        vops::cvtpd2dq(lo, hi, src_256, trunc)
                    };
                    if Cpu::is_vex(ins) {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Pmovsxbw
            | Mnemonic::Vpmovsxbw
            | Mnemonic::Pmovzxbw
            | Mnemonic::Vpmovzxbw
            | Mnemonic::Pmovsxbd
            | Mnemonic::Vpmovsxbd
            | Mnemonic::Pmovzxbd
            | Mnemonic::Vpmovzxbd
            | Mnemonic::Pmovsxbq
            | Mnemonic::Vpmovsxbq
            | Mnemonic::Pmovzxbq
            | Mnemonic::Vpmovzxbq
            | Mnemonic::Pmovsxwd
            | Mnemonic::Vpmovsxwd
            | Mnemonic::Pmovzxwd
            | Mnemonic::Vpmovzxwd
            | Mnemonic::Pmovsxwq
            | Mnemonic::Vpmovsxwq
            | Mnemonic::Pmovzxwq
            | Mnemonic::Vpmovzxwq
            | Mnemonic::Pmovsxdq
            | Mnemonic::Vpmovsxdq
            | Mnemonic::Pmovzxdq
            | Mnemonic::Vpmovzxdq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let s_op = if ins.op_count() >= 2 { 1 } else { 0 };
                    // Preserve exact memory read width (`m16`/`m32`/`m64`/`m128`).
                    let (raw, _) = cpu.vsrc(ins, s_op, false);
                    let code: u8 = match mn {
                        Mnemonic::Pmovsxbw | Mnemonic::Vpmovsxbw => 0,
                        Mnemonic::Pmovzxbw | Mnemonic::Vpmovzxbw => 1,
                        Mnemonic::Pmovsxbd | Mnemonic::Vpmovsxbd => 2,
                        Mnemonic::Pmovzxbd | Mnemonic::Vpmovzxbd => 3,
                        Mnemonic::Pmovsxbq | Mnemonic::Vpmovsxbq => 4,
                        Mnemonic::Pmovzxbq | Mnemonic::Vpmovzxbq => 5,
                        Mnemonic::Pmovsxwd | Mnemonic::Vpmovsxwd => 6,
                        Mnemonic::Pmovzxwd | Mnemonic::Vpmovzxwd => 7,
                        Mnemonic::Pmovsxwq | Mnemonic::Vpmovsxwq => 8,
                        Mnemonic::Pmovzxwq | Mnemonic::Vpmovzxwq => 9,
                        Mnemonic::Pmovsxdq | Mnemonic::Vpmovsxdq => 10,
                        _ => 11,
                    };
                    let (out_lo, out_hi) = vops::pmovsx_zx(raw, code, ymm);
                    vops::store_vec_dst(cpu, ins, d, ymm, out_lo, out_hi);
                }
            }
            _ => return false,
        }
        true
    }
}
