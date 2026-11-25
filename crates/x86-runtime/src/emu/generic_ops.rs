//! Generic `iced-x86` instruction interpreter (`Cpu::exec_generic_ins`): integer ALU,
//! shifts/rotates, bit tests, multiply/divide, atomics (`lock`, `cmpxchg8b`/`16b`), `cpuid`,
//! `rdtsc`, descriptor-table stores, string ops, and fallback dispatch into `simd_mov` / `simd_fp`.

use super::cpu::{cas128_full, Cpu};
use super::{fast, mem};
use iced_x86::{ConditionCode, FlowControl, Instruction, Mnemonic, OpKind, Register};

impl Cpu {
    pub(super) unsafe fn exec_generic_ins(&mut self, ins_ref: &Instruction) -> bool {
        if super::denorm::software_mode(self.jx.mxcsr) {
            if let Some(handled) = super::denorm::execute(self, ins_ref) {
                return handled;
            }
        }
        self.exec_generic_inner(ins_ref)
    }

    pub(super) unsafe fn exec_generic_inner(&mut self, ins_ref: &Instruction) -> bool {
        let ins = *ins_ref;
        let mn = ins.mnemonic();
        let cpu = self;
        if super::mmx::uses_mmx(&ins) { return super::mmx::execute(cpu, &ins); }
        match mn {
            Mnemonic::Mfence | Mnemonic::Sfence | Mnemonic::Lfence => {
                core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
            }
            Mnemonic::Nop | Mnemonic::Pause | Mnemonic::Fnop | Mnemonic::Endbr64 | Mnemonic::Endbr32
            | Mnemonic::Clflush | Mnemonic::Prefetch | Mnemonic::Prefetcht0 | Mnemonic::Prefetcht1
            | Mnemonic::Prefetcht2 | Mnemonic::Prefetchnta | Mnemonic::Prefetchw | Mnemonic::Prefetchwt1
            | Mnemonic::Wait | Mnemonic::Rdsspd | Mnemonic::Rdsspq => {}
            Mnemonic::Emms | Mnemonic::Femms => super::mmx::emms(),
            Mnemonic::Maskmovdqu | Mnemonic::Vmaskmovdqu => super::mmx::maskmovdqu(cpu, &ins),
            Mnemonic::Clc => cpu.cf = false,
            Mnemonic::Stc => cpu.cf = true,
            Mnemonic::Cmc => cpu.cf = !cpu.cf,
            Mnemonic::Cld => cpu.df = false,
            Mnemonic::Std => cpu.df = true,
            Mnemonic::Leave => {
                cpu.gpr[4] = cpu.gpr[5];
                let v = cpu.read_mem(cpu.gpr[4], 64);
                cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                cpu.gpr[5] = v;
                cpu.last_bits[4] = 64;
                cpu.last_bits[5] = 64;
            }
            Mnemonic::Push => {
                // 64-bit mode: 8 bytes, or 2 with a 66h operand-size prefix (push r16/m16/imm16).
                let bits = if Cpu::op_bits(&ins, 0) == 16 { 16 } else { 64 };
                let v = cpu.read_op(&ins, 0, bits);
                cpu.gpr[4] = cpu.gpr[4].wrapping_sub((bits / 8) as u64);
                cpu.write_mem(cpu.gpr[4], bits, v);
            }
            Mnemonic::Pop => {
                let bits = if Cpu::op_bits(&ins, 0) == 16 { 16 } else { 64 };
                let v = cpu.read_mem(cpu.gpr[4], bits);
                cpu.gpr[4] = cpu.gpr[4].wrapping_add((bits / 8) as u64);
                cpu.write_op(&ins, 0, bits, v);
            }
            Mnemonic::Lea => {
                let a = cpu.eff_addr(&ins);
                let bits = Cpu::op_bits(&ins, 0);
                cpu.write_op(&ins, 0, bits, a);
            }
            Mnemonic::Mov | Mnemonic::Movnti => {
                let bits = Cpu::op_bits(&ins, 0);
                let v = cpu.read_op(&ins, 1, bits);
                cpu.write_op(&ins, 0, bits, v);
            }
            Mnemonic::Movzx => {
                let dst_bits = Cpu::op_bits(&ins, 0);
                let src_bits = Cpu::op_bits(&ins, 1);
                let v = cpu.read_op(&ins, 1, src_bits);
                let mask = if src_bits >= 64 { u64::MAX } else { (1u64 << src_bits) - 1 };
                cpu.write_op(&ins, 0, dst_bits, v & mask);
            }
            Mnemonic::Movsx | Mnemonic::Movsxd => {
                let dst_bits = Cpu::op_bits(&ins, 0);
                let src_bits = Cpu::op_bits(&ins, 1);
                let v = cpu.read_op(&ins, 1, src_bits);
                let sext = match src_bits {
                    8 => v as i8 as i64 as u64,
                    16 => v as i16 as i64 as u64,
                    32 => v as i32 as i64 as u64,
                    _ => v,
                };
                cpu.write_op(&ins, 0, dst_bits, sext);
            }
            Mnemonic::Cbw => {
                cpu.gpr[0] = (cpu.gpr[0] & !0xFFFF) | ((cpu.gpr[0] as u8 as i8 as i16 as u16) as u64);
            }
            Mnemonic::Cwde => {
                cpu.gpr[0] = (cpu.gpr[0] as u16 as i16 as i32 as u32) as u64;
            }
            Mnemonic::Cdqe => {
                cpu.gpr[0] = cpu.gpr[0] as i32 as i64 as u64;
            }
            Mnemonic::Cwd => {
                let hi = if (cpu.gpr[0] as i16) < 0 { 0xFFFFu64 } else { 0 };
                cpu.gpr[2] = (cpu.gpr[2] & !0xFFFF) | hi;
            }
            Mnemonic::Cdq => {
                cpu.gpr[2] = if (cpu.gpr[0] as i32) < 0 { 0xFFFF_FFFF } else { 0 };
            }
            Mnemonic::Cqo => {
                cpu.gpr[2] = if (cpu.gpr[0] as i64) < 0 { u64::MAX } else { 0 };
            }
            Mnemonic::Xor => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a ^ b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a ^ b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::Or => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a | b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a | b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::And => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a & b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a & b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::Test => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                cpu.update_flags_logic(a & b, bits);
            }
            Mnemonic::Add => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Sub => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(b));
                    let _ = cpu.update_flags_sub(a, b, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Cmp => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let _ = cpu.update_flags_sub(a, b, bits);
            }
            Mnemonic::Inc => {
                let bits = Cpu::op_bits(&ins, 0);
                let saved_cf = cpu.cf;
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(1));
                    let _ = cpu.update_flags_add(a, 1, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, 1, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
                cpu.cf = saved_cf;
            }
            Mnemonic::Dec => {
                let bits = Cpu::op_bits(&ins, 0);
                let saved_cf = cpu.cf;
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(1));
                    let _ = cpu.update_flags_sub(a, 1, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(a, 1, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
                cpu.cf = saved_cf;
            }
            Mnemonic::Not => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let _ = cpu.atomic_rmw(addr, bits, |old| !old);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    cpu.write_op(&ins, 0, bits, !a);
                }
            }
            Mnemonic::Neg => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| 0u64.wrapping_sub(old));
                    let _ = cpu.update_flags_sub(0, a, bits);
                    cpu.cf = a != 0;
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(0, a, bits);
                    cpu.cf = a != 0;
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Shl | Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let sub = match mn {
                        Mnemonic::Shr => fast::SHIFT_SHR,
                        Mnemonic::Sar => fast::SHIFT_SAR,
                        _ => fast::SHIFT_SHL,
                    };
                    let res = cpu.shift(sub, a, cnt, bits as u8);
                    cpu.write_op(&ins, 0, bits, res);
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    // Count 0: flags unchanged, but a 32-bit register destination is still
                    // written (upper half zeroed), as on hardware.
                    cpu.write_op(&ins, 0, 32, a);
                }
            }
            Mnemonic::Rol => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let res = match bits {
                        8 => (a as u8).rotate_left(cnt) as u64,
                        16 => (a as u16).rotate_left(cnt) as u64,
                        32 => (a as u32).rotate_left(cnt) as u64,
                        _ => a.rotate_left(cnt),
                    };
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.cf = (res & 1) != 0;
                    cpu.of = ((res >> (bits - 1)) & 1 != 0) ^ cpu.cf;
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    cpu.write_op(&ins, 0, 32, a); // count 0 still zero-extends a 32-bit register
                }
            }
            Mnemonic::Ror => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let res = match bits {
                        8 => (a as u8).rotate_right(cnt) as u64,
                        16 => (a as u16).rotate_right(cnt) as u64,
                        32 => (a as u32).rotate_right(cnt) as u64,
                        _ => a.rotate_right(cnt),
                    };
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.cf = ((res >> (bits - 1)) & 1) != 0;
                    cpu.of = cpu.cf ^ ((res >> (bits - 2)) & 1 != 0);
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    cpu.write_op(&ins, 0, 32, a); // count 0 still zero-extends a 32-bit register
                }
            }
            Mnemonic::Bsr => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let v = src & mask;
                if v == 0 {
                    cpu.zf = true;
                } else {
                    cpu.zf = false;
                    let idx = 63u64.wrapping_sub(v.leading_zeros() as u64);
                    cpu.write_op(&ins, 0, bits, idx);
                }
            }
            Mnemonic::Bsf | Mnemonic::Tzcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let v = src & mask;
                let is_tzcnt = ins.mnemonic() == Mnemonic::Tzcnt;
                if v == 0 {
                    // BSF: ZF=1, dest unchanged. TZCNT: CF=1, dest = operand size, ZF = (dest == 0) = 0.
                    cpu.zf = !is_tzcnt;
                    cpu.cf = true;
                    if is_tzcnt {
                        cpu.write_op(&ins, 0, bits, bits as u64);
                    }
                } else {
                    let idx = v.trailing_zeros() as u64;
                    cpu.zf = is_tzcnt && idx == 0;
                    cpu.cf = false;
                    cpu.write_op(&ins, 0, bits, idx);
                }
            }
            Mnemonic::Lzcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let res = match bits {
                    16 => (src as u16).leading_zeros() as u64,
                    32 => (src as u32).leading_zeros() as u64,
                    _ => src.leading_zeros() as u64,
                };
                cpu.write_op(&ins, 0, bits, res);
                cpu.zf = res == 0;
                cpu.cf = res == bits as u64;
            }
            Mnemonic::Popcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let res = (src & mask).count_ones() as u64;
                cpu.write_op(&ins, 0, bits, res);
                cpu.zf = res == 0;
                cpu.cf = false;
                cpu.sf = false;
                cpu.of = false;
                cpu.set_pf(false);
            }
            Mnemonic::Bswap => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 0, bits);
                let res = match bits {
                    32 => (src as u32).swap_bytes() as u64,
                    _ => src.swap_bytes(),
                };
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.op_kind(0) == OpKind::Memory {
                    let base_addr = cpu.eff_addr(&ins);
                    let (addr, b) = if ins.op_kind(1) == OpKind::Register {
                        let bit_off = match bits {
                            16 => cpu.read_op(&ins, 1, 16) as i16 as i64,
                            32 => cpu.read_op(&ins, 1, 32) as i32 as i64,
                            _ => cpu.read_op(&ins, 1, 64) as i64,
                        };
                        let elem_bytes = (bits / 8) as i64;
                        let elem_idx = bit_off.div_euclid(bits as i64);
                        let rem = bit_off.rem_euclid(bits as i64) as u32;
                        ((base_addr as i64).wrapping_add(elem_idx.wrapping_mul(elem_bytes)) as u64, rem)
                    } else {
                        (base_addr, (cpu.read_op(&ins, 1, bits) as u32) & (bits - 1))
                    };
                    match ins.mnemonic() {
                        Mnemonic::Bts => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old | (1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        Mnemonic::Btr => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old & !(1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        Mnemonic::Btc => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old ^ (1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        _ => {
                            let a = cpu.read_mem(addr, bits);
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                    }
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let b = (cpu.read_op(&ins, 1, bits) as u32) & (bits - 1);
                    cpu.cf = ((a >> b) & 1) != 0;
                    match ins.mnemonic() {
                        Mnemonic::Bts => cpu.write_op(&ins, 0, bits, a | (1u64 << b)),
                        Mnemonic::Btr => cpu.write_op(&ins, 0, bits, a & !(1u64 << b)),
                        Mnemonic::Btc => cpu.write_op(&ins, 0, bits, a ^ (1u64 << b)),
                        _ => {}
                    }
                }
            }
            Mnemonic::Adc => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let full = (ua as u128) + (ub as u128) + (cpu.cf as u128);
                let res = (full as u64) & mask;
                let sign_bit = 1u64 << (bits - 1);
                cpu.cf = full > mask as u128;
                cpu.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
                cpu.zf = res == 0;
                cpu.sf = (res & sign_bit) != 0;
                cpu.pfv = res as u8;
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Sbb => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let c = cpu.cf as u64;
                let res = ua.wrapping_sub(ub).wrapping_sub(c) & mask;
                let sign_bit = 1u64 << (bits - 1);
                cpu.cf = (ua as u128) < (ub as u128) + (c as u128);
                cpu.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
                cpu.zf = res == 0;
                cpu.sf = (res & sign_bit) != 0;
                cpu.pfv = res as u8;
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Mul => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 0, bits);
                if bits == 64 {
                    let prod = (cpu.gpr[0] as u128).wrapping_mul(src as u128);
                    cpu.gpr[0] = prod as u64;
                    cpu.gpr[2] = (prod >> 64) as u64;
                    cpu.cf = cpu.gpr[2] != 0;
                    cpu.of = cpu.cf;
                } else if bits == 32 {
                    let prod = (cpu.gpr[0] as u32 as u64).wrapping_mul(src as u32 as u64);
                    cpu.gpr[0] = prod as u32 as u64;
                    cpu.gpr[2] = (prod >> 32) as u32 as u64;
                    cpu.cf = cpu.gpr[2] != 0;
                    cpu.of = cpu.cf;
                } else if bits == 16 {
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
            Mnemonic::Div => {
                let bits = Cpu::op_bits(&ins, 0);
                let d = cpu.read_op(&ins, 0, bits);
                if bits == 64 && d != 0 {
                    let n = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                    cpu.gpr[0] = (n / (d as u128)) as u64;
                    cpu.gpr[2] = (n % (d as u128)) as u64;
                } else if bits == 32 && (d as u32) != 0 {
                    let n = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                    let d32 = d as u32 as u64;
                    cpu.gpr[0] = (n / d32) as u32 as u64;
                    cpu.gpr[2] = (n % d32) as u32 as u64;
                } else if bits == 16 && (d as u16) != 0 {
                    let n = ((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32);
                    let d16 = d as u16 as u32;
                    cpu.set_reg(Register::AX, (n / d16) as u64);
                    cpu.set_reg(Register::DX, (n % d16) as u64);
                } else if bits == 8 && (d as u8) != 0 {
                    let n = cpu.gpr[0] as u16;
                    let d8 = d as u8 as u16;
                    cpu.set_reg(Register::AL, (n / d8) as u64);
                    cpu.set_reg(Register::AH, (n % d8) as u64);
                }
            }
            Mnemonic::Idiv => {
                let bits = Cpu::op_bits(&ins, 0);
                let d = cpu.read_op(&ins, 0, bits);
                if bits == 64 && d != 0 {
                    let n = (((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128)) as i128;
                    let d64 = d as i64 as i128;
                    cpu.gpr[0] = n.wrapping_div(d64) as i64 as u64;
                    cpu.gpr[2] = n.wrapping_rem(d64) as i64 as u64;
                } else if bits == 32 && (d as u32) != 0 {
                    let n = (((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64)) as i64;
                    let d32 = d as u32 as i32 as i64;
                    cpu.gpr[0] = n.wrapping_div(d32) as i32 as u32 as u64;
                    cpu.gpr[2] = n.wrapping_rem(d32) as i32 as u32 as u64;
                } else if bits == 16 && (d as u16) != 0 {
                    let n = (((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32)) as i32;
                    let d16 = d as u16 as i16 as i32;
                    cpu.set_reg(Register::AX, n.wrapping_div(d16) as u64);
                    cpu.set_reg(Register::DX, n.wrapping_rem(d16) as u64);
                } else if bits == 8 && (d as u8) != 0 {
                    let n = cpu.gpr[0] as u16 as i16;
                    let d8 = d as u8 as i8 as i16;
                    cpu.set_reg(Register::AL, n.wrapping_div(d8) as u64);
                    cpu.set_reg(Register::AH, n.wrapping_rem(d8) as u64);
                }
            }
            Mnemonic::Imul => {
                let bits = Cpu::op_bits(&ins, 0);
                let sext = |v: u64, b: u32| -> i128 {
                    match b {
                        8 => v as u8 as i8 as i128,
                        16 => v as u16 as i16 as i128,
                        32 => v as u32 as i32 as i128,
                        _ => v as i64 as i128,
                    }
                };
                match ins.op_count() {
                    1 => {
                        let src = cpu.read_op(&ins, 0, bits);
                        if bits == 64 {
                            let prod = (cpu.gpr[0] as i64 as i128).wrapping_mul(src as i64 as i128);
                            cpu.gpr[0] = prod as i64 as u64;
                            cpu.gpr[2] = (prod >> 64) as i64 as u64;
                            cpu.cf = prod != (prod as i64 as i128);
                            cpu.of = cpu.cf;
                        } else if bits == 32 {
                            let prod = (cpu.gpr[0] as u32 as i32 as i64).wrapping_mul(src as u32 as i32 as i64);
                            cpu.gpr[0] = prod as i32 as u32 as u64;
                            cpu.gpr[2] = ((prod >> 32) as i32 as u32) as u64;
                            cpu.cf = prod != (prod as i32 as i64);
                            cpu.of = cpu.cf;
                        } else if bits == 16 {
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
                        let a = sext(cpu.read_op(&ins, 0, bits), bits);
                        let b = sext(cpu.read_op(&ins, 1, bits), bits);
                        let prod = a.wrapping_mul(b);
                        let res = prod as u64;
                        cpu.write_op(&ins, 0, bits, res);
                        let fits = match bits {
                            16 => prod == (prod as i16 as i128),
                            32 => prod == (prod as i32 as i128),
                            _ => prod == (prod as i64 as i128),
                        };
                        cpu.cf = !fits;
                        cpu.of = !fits;
                    }
                    3 => {
                        let a = sext(cpu.read_op(&ins, 1, bits), bits);
                        let b = sext(cpu.read_op(&ins, 2, bits), bits);
                        let prod = a.wrapping_mul(b);
                        let res = prod as u64;
                        cpu.write_op(&ins, 0, bits, res);
                        let fits = match bits {
                            16 => prod == (prod as i16 as i128),
                            32 => prod == (prod as i32 as i128),
                            _ => prod == (prod as i64 as i128),
                        };
                        cpu.cf = !fits;
                        cpu.of = !fits;
                    }
                    _ => {}
                }
            }
            Mnemonic::Xchg => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.op_kind(0) == OpKind::Memory {
                    let b = cpu.read_op(&ins, 1, bits);
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |_| b);
                    cpu.write_op(&ins, 1, bits, a);
                } else if ins.op_kind(1) == OpKind::Memory {
                    let a = cpu.read_op(&ins, 0, bits);
                    let addr = cpu.eff_addr(&ins);
                    let (b, _) = cpu.atomic_rmw(addr, bits, |_| a);
                    cpu.write_op(&ins, 0, bits, b);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let b = cpu.read_op(&ins, 1, bits);
                    cpu.write_op(&ins, 0, bits, b);
                    cpu.write_op(&ins, 1, bits, a);
                }
            }
            Mnemonic::Cmpxchg => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let acc = cpu.gpr[0] & mask;
                if ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (ok, dst) = cpu.atomic_cmpxchg(addr, bits, acc, src & mask);
                    let _ = cpu.update_flags_sub(acc, dst, bits);
                    if ok {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        let acc_reg = match bits {
                            8 => Register::AL,
                            16 => Register::AX,
                            32 => Register::EAX,
                            _ => Register::RAX,
                        };
                        cpu.set_reg(acc_reg, dst);
                    }
                } else {
                    let dst = cpu.read_op(&ins, 0, bits);
                    let _ = cpu.update_flags_sub(acc, dst, bits);
                    if (acc & mask) == (dst & mask) {
                        cpu.zf = true;
                        cpu.write_op(&ins, 0, bits, src);
                    } else {
                        cpu.zf = false;
                        let acc_reg = match bits {
                            8 => Register::AL,
                            16 => Register::AX,
                            32 => Register::EAX,
                            _ => Register::RAX,
                        };
                        cpu.set_reg(acc_reg, dst);
                    }
                }
            }
            Mnemonic::Cmpxchg8b | Mnemonic::Cmpxchg16b => {
                let addr = cpu.eff_addr(&ins);
                if mn == Mnemonic::Cmpxchg8b {
                    let expected = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                    let desired = ((cpu.gpr[1] as u32 as u64) << 32) | (cpu.gpr[3] as u32 as u64);
                    let (ok, actual) = cpu.atomic_cmpxchg(addr, 64, expected, desired);
                    cpu.zf = ok;
                    if !ok {
                        cpu.set_reg(Register::EAX, actual as u32 as u64);
                        cpu.set_reg(Register::EDX, (actual >> 32) as u32 as u64);
                    }
                } else {
                    let expected = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                    let desired = ((cpu.gpr[1] as u128) << 64) | (cpu.gpr[3] as u128);
                    let fast = if addr & 15 == 0 && crate::emu::regfile_ext::is_valid_guest_range(addr, 16) {
                        cas128_full(addr, expected, desired)
                    } else {
                        None
                    };
                    let actual = if let Some(a) = fast {
                        a
                    } else {
                        let _g = super::cpu::lock_atomic_shard(addr);
                        let actual = cpu.read_u128(addr);
                        if actual == expected {
                            cpu.write_u128(addr, desired);
                        }
                        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
                        actual
                    };
                    if actual == expected {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        cpu.set_reg(Register::RAX, actual as u64);
                        cpu.set_reg(Register::RDX, (actual >> 64) as u64);
                    }
                }
            }
            Mnemonic::Xadd => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 1, bits, a);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.write_op(&ins, 1, bits, a);
                }
            }
            Mnemonic::Cpuid => cpu.cpuid(),
            Mnemonic::Xgetbv | Mnemonic::Fxsave | Mnemonic::Fxsave64 | Mnemonic::Fxrstor | Mnemonic::Fxrstor64
            | Mnemonic::Xsave | Mnemonic::Xsave64 | Mnemonic::Xsaveopt | Mnemonic::Xsaveopt64 | Mnemonic::Xsavec
            | Mnemonic::Xsavec64 | Mnemonic::Xrstor | Mnemonic::Xrstor64 => {
                if let Err(fault) = cpu.exec_xstate(&ins) {
                    fault.dispatch(cpu, ins.ip());
                }
            }
            Mnemonic::Rdtsc | Mnemonic::Rdtscp => cpu.read_timestamp(mn == Mnemonic::Rdtscp),
            // Unprivileged x86-64 system-table / machine-status store instructions
            // (used by OS runtimes and integrity probes; legal in ring 3 when UMIP=0).
            Mnemonic::Sidt => {
                let a = cpu.eff_addr(&ins);
                // Standard 64-bit Windows IDT descriptor: 256 16-byte gates (limit 0x0fff)
                // at a canonical kernel-space base address.
                cpu.write_mem(a, 16, 0x0fff);
                cpu.write_mem(a.wrapping_add(2), 64, 0xffff_f800_0000_0000);
            }
            Mnemonic::Sgdt => {
                let a = cpu.eff_addr(&ins);
                cpu.write_mem(a, 16, 0x0057);
                cpu.write_mem(a.wrapping_add(2), 64, 0xffff_f800_0000_1000);
            }
            Mnemonic::Sldt => {
                // Windows x64 does not use an LDT; memory operand is always 16 bits.
                cpu.write_op(&ins, 0, 16, 0);
            }
            Mnemonic::Str => {
                // Windows x64 task register selector (KGDT64_SYS_TSS = 0x40).
                cpu.write_op(&ins, 0, 16, 0x0040);
            }
            Mnemonic::Smsw => {
                // Standard protected+paging+NE+WP CR0 machine status word.
                // Memory destination stores low 16 bits; register destination stores
                // 16/32/64 bits according to operand width.
                cpu.write_op(&ins, 0, 16, 0x8005_0033);
            }
            Mnemonic::Stosb | Mnemonic::Stosw | Mnemonic::Stosd | Mnemonic::Stosq => {
                let elem_bytes: u64 = match mn {
                    Mnemonic::Stosb => 1,
                    Mnemonic::Stosw => 2,
                    Mnemonic::Stosd => 4,
                    _ => 8,
                };
                let count = if ins.has_rep_prefix() { cpu.gpr[1] } else { 1 };
                let val = cpu.gpr[0];
                if cpu.df {
                    for _ in 0..count {
                        cpu.write_mem(cpu.gpr[7], (elem_bytes * 8) as u32, val);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_sub(elem_bytes);
                    }
                } else if elem_bytes == 1 && count > 0 {
                    if mem::tso() {
                        // Plain bulk stores: order them after all earlier guest accesses.
                        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                    }
                    core::ptr::write_bytes(cpu.gpr[7] as *mut u8, val as u8, count as usize);
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(count);
                } else {
                    for _ in 0..count {
                        cpu.write_mem(cpu.gpr[7], (elem_bytes * 8) as u32, val);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(elem_bytes);
                    }
                }
                cpu.last_bits[7] = 64;
                if ins.has_rep_prefix() {
                    cpu.gpr[1] = 0;
                    cpu.last_bits[1] = 64;
                }
            }
            Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq => {
                if mn == Mnemonic::Movsd && (ins.op_kind(0) == OpKind::Register || ins.op_kind(1) == OpKind::Register) {
                    // SSE movsd
                    match (ins.op_kind(0), ins.op_kind(1)) {
                        (OpKind::Register, OpKind::Register) => {
                            if let (Some(d), Some(s)) = (Cpu::xmm_idx(ins.op_register(0)), Cpu::xmm_idx(ins.op_register(1))) {
                                cpu.xmm[d] = (cpu.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF);
                            }
                        }
                        (OpKind::Register, OpKind::Memory) => {
                            if let Some(d) = Cpu::xmm_idx(ins.op_register(0)) {
                                let a = cpu.eff_addr(&ins);
                                cpu.xmm[d] = cpu.read_mem(a, 64) as u128;
                            }
                        }
                        (OpKind::Memory, OpKind::Register) => {
                            if let Some(s) = Cpu::xmm_idx(ins.op_register(1)) {
                                let a = cpu.eff_addr(&ins);
                                cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                            }
                        }
                        _ => {}
                    }
                } else {
                    let elem_bytes: u64 = match mn {
                        Mnemonic::Movsb => 1,
                        Mnemonic::Movsw => 2,
                        Mnemonic::Movsd => 4,
                        _ => 8,
                    };
                    let count = if ins.has_rep_prefix() { cpu.gpr[1] } else { 1 };
                    let total_bytes = count.wrapping_mul(elem_bytes);
                    if cpu.df {
                        let bits = (elem_bytes * 8) as u32;
                        for _ in 0..count {
                            let v = cpu.read_mem(cpu.gpr[6], bits);
                            cpu.write_mem(cpu.gpr[7], bits, v);
                            cpu.gpr[6] = cpu.gpr[6].wrapping_sub(elem_bytes);
                            cpu.gpr[7] = cpu.gpr[7].wrapping_sub(elem_bytes);
                        }
                    } else if total_bytes > 0 {
                        let (src, dst) = (cpu.gpr[6], cpu.gpr[7]);
                        let dist = dst.wrapping_sub(src);
                        if mem::tso() {
                            // Plain bulk copy: order it after all earlier guest accesses.
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                        }
                        if dist == 0 || dist >= total_bytes {
                            // No destructive overlap (dst before src, or disjoint): x86's
                            // forward element-by-element copy equals memmove.
                            core::ptr::copy(src as *const u8, dst as *mut u8, total_bytes as usize);
                        } else if dist >= elem_bytes {
                            // dst inside (src, src+len): the first `dist` bytes are replicated
                            // (std::fill idiom). Copy in non-overlapping chunks of `dist` bytes.
                            let mut off = 0u64;
                            while off < total_bytes {
                                let n = dist.min(total_bytes - off);
                                core::ptr::copy_nonoverlapping((src + off) as *const u8, (dst + off) as *mut u8, n as usize);
                                off += n;
                            }
                        } else {
                            let bits = (elem_bytes * 8) as u32;
                            for i in 0..count {
                                let v = cpu.read_mem(src.wrapping_add(i * elem_bytes), bits);
                                cpu.write_mem(dst.wrapping_add(i * elem_bytes), bits, v);
                            }
                        }
                        if mem::tso() {
                            // ...and keep its plain loads before later guest loads.
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                        }
                        cpu.gpr[6] = src.wrapping_add(total_bytes);
                        cpu.gpr[7] = dst.wrapping_add(total_bytes);
                    }
                    cpu.last_bits[6] = 64;
                    cpu.last_bits[7] = 64;
                    if ins.has_rep_prefix() {
                        cpu.gpr[1] = 0;
                        cpu.last_bits[1] = 64;
                    }
                }
            }
            Mnemonic::Xorps | Mnemonic::Xorpd | Mnemonic::Pxor | Mnemonic::Vxorps | Mnemonic::Vxorpd | Mnemonic::Vpxor => {
                if let Some(d) = Cpu::xmm_idx(ins.op_register(0)) {
                    // Legacy SSE: dst ^= src (2 operands). VEX: dst = src1 ^ src2 (3 operands).
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let ymm = Cpu::is_ymm(ins.op_register(0));
                    let (a_lo, a_hi) = cpu.vsrc(&ins, s1_op, ymm);
                    let (b_lo, b_hi) = cpu.vsrc(&ins, s2_op, ymm);
                    cpu.vdst(&ins, d, a_lo ^ b_lo, a_hi ^ b_hi);
                }
            }
            _ => {
                if !cpu.exec_simd_mov_int(mn, &ins) && !cpu.exec_simd_fp_ext(mn, &ins) {
                    let cc = ins.condition_code();
                    if cc != ConditionCode::None && ins.flow_control() != FlowControl::ConditionalBranch {
                        if ins.op_count() == 1 {
                            let v = cpu.eval_cc(cc) as u64;
                            cpu.write_op(&ins, 0, 8, v);
                        } else if ins.op_count() == 2 {
                            let bits = Cpu::op_bits(&ins, 0);
                            if cpu.eval_cc(cc) {
                                let v = cpu.read_op(&ins, 1, bits);
                                cpu.write_op(&ins, 0, bits, v);
                            } else if bits == 32 {
                                let cur = cpu.read_op(&ins, 0, 32);
                                cpu.write_op(&ins, 0, 32, cur);
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
