//! SSE/AVX floating-point, FMA, AES, BMI, x87, and system instruction execution for [`super::Cpu`].

use super::simd_mov::vops;
use super::*;

const CRC32C_TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = i as u32;
        let mut b = 0;
        while b < 8 {
            c = if (c & 1) != 0 { (c >> 1) ^ 0x82F6_3B78 } else { c >> 1 };
            b += 1;
        }
        t[i] = c;
        i += 1;
    }
    t
};

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
                const SBOX: [u8; 256] = [
                    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
                    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
                    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
                    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
                    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
                    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
                    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
                    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
                    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
                    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
                    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
                    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
                    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
                    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
                    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
                    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
                ];
                const INV_SBOX: [u8; 256] = [
                    0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
                    0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
                    0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
                    0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
                    0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
                    0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
                    0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
                    0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
                    0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
                    0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
                    0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
                    0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
                    0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
                    0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
                    0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
                    0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d,
                ];
                let xtime = |x: u8| -> u8 { (x << 1) ^ (if (x & 0x80) != 0 { 0x1b } else { 0 }) };
                let gmul = |mut a: u8, mut b: u8| -> u8 {
                    let mut p = 0u8;
                    for _ in 0..8 {
                        if (b & 1) != 0 {
                            p ^= a;
                        }
                        a = xtime(a);
                        b >>= 1;
                    }
                    p
                };
                let inv_mix_cols = |st: &mut [u8; 16]| {
                    for c in 0..4 {
                        let i = c * 4;
                        let (s0, s1, s2, s3) = (st[i], st[i + 1], st[i + 2], st[i + 3]);
                        st[i] = gmul(s0, 0x0e) ^ gmul(s1, 0x0b) ^ gmul(s2, 0x0d) ^ gmul(s3, 0x09);
                        st[i + 1] = gmul(s0, 0x09) ^ gmul(s1, 0x0e) ^ gmul(s2, 0x0b) ^ gmul(s3, 0x0d);
                        st[i + 2] = gmul(s0, 0x0d) ^ gmul(s1, 0x09) ^ gmul(s2, 0x0e) ^ gmul(s3, 0x0b);
                        st[i + 3] = gmul(s0, 0x0b) ^ gmul(s1, 0x0d) ^ gmul(s2, 0x09) ^ gmul(s3, 0x0e);
                    }
                };
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    match mn {
                        Mnemonic::Aesimc | Mnemonic::Vaesimc => {
                            let src = match ins.op_kind(1) {
                                OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                                OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                                _ => 0,
                            };
                            let mut st = src.to_le_bytes();
                            inv_mix_cols(&mut st);
                            cpu.xmm[d] = u128::from_le_bytes(st);
                        }
                        Mnemonic::Aeskeygenassist | Mnemonic::Vaeskeygenassist => {
                            let src = match ins.op_kind(1) {
                                OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                                OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                                _ => 0,
                            };
                            let rcon = ins.immediate8() as u32;
                            let x1 = ((src >> 32) & 0xFFFF_FFFF) as u32;
                            let x3 = ((src >> 96) & 0xFFFF_FFFF) as u32;
                            let sub_word = |w: u32| -> u32 {
                                let b0 = SBOX[(w & 0xFF) as usize] as u32;
                                let b1 = SBOX[((w >> 8) & 0xFF) as usize] as u32;
                                let b2 = SBOX[((w >> 16) & 0xFF) as usize] as u32;
                                let b3 = SBOX[((w >> 24) & 0xFF) as usize] as u32;
                                b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
                            };
                            let t1 = sub_word(x1);
                            let t3 = sub_word(x3);
                            let w0 = t1;
                            let w1 = t1.rotate_right(8) ^ rcon;
                            let w2 = t3;
                            let w3 = t3.rotate_right(8) ^ rcon;
                            cpu.xmm[d] = (w0 as u128)
                                | ((w1 as u128) << 32)
                                | ((w2 as u128) << 64)
                                | ((w3 as u128) << 96);
                        }
                        _ => {
                            let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let state = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                            let rkey = match ins.op_kind(s2_op) {
                                OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i]),
                                OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                                _ => 0,
                            };
                            let mut st = state.to_le_bytes();
                            let is_dec = matches!(
                                mn,
                                Mnemonic::Aesdec | Mnemonic::Vaesdec | Mnemonic::Aesdeclast | Mnemonic::Vaesdeclast
                            );
                            let is_last = matches!(
                                mn,
                                Mnemonic::Aesenclast | Mnemonic::Vaesenclast | Mnemonic::Aesdeclast | Mnemonic::Vaesdeclast
                            );
                            if !is_dec {
                                // ShiftRows then SubBytes
                                let tmp = st;
                                let sr: [usize; 16] = [0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];
                                for i in 0..16 {
                                    st[i] = SBOX[tmp[sr[i]] as usize];
                                }
                                if !is_last {
                                    for c in 0..4 {
                                        let i = c * 4;
                                        let (s0, s1, s2, s3) = (st[i], st[i + 1], st[i + 2], st[i + 3]);
                                        st[i] = xtime(s0) ^ xtime(s1) ^ s1 ^ s2 ^ s3;
                                        st[i + 1] = s0 ^ xtime(s1) ^ xtime(s2) ^ s2 ^ s3;
                                        st[i + 2] = s0 ^ s1 ^ xtime(s2) ^ xtime(s3) ^ s3;
                                        st[i + 3] = xtime(s0) ^ s0 ^ s1 ^ s2 ^ xtime(s3);
                                    }
                                }
                            } else {
                                // InvShiftRows then InvSubBytes
                                let tmp = st;
                                let isr: [usize; 16] = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];
                                for i in 0..16 {
                                    st[i] = INV_SBOX[tmp[isr[i]] as usize];
                                }
                                if !is_last {
                                    inv_mix_cols(&mut st);
                                }
                            }
                            cpu.xmm[d] = u128::from_le_bytes(st) ^ rkey;
                        }
                    }
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
                    let x = f32::from_bits(b_bits);
                    let r = if is_rsqrt { 1.0f32 / sse_sqrt32(x) } else { 1.0f32 / x };
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF) | (r.to_bits() as u128);
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
                    let q0 = ((a_lo >> (((imm & 1) as u32) * 64)) as u64) as u128;
                    let q1 = ((b_lo >> ((((imm >> 1) & 1) as u32) * 64)) as u64) as u128;
                    let lo = q0 | (q1 << 64);
                    let hi = if ymm {
                        let q2 = ((a_hi >> ((((imm >> 2) & 1) as u32) * 64)) as u64) as u128;
                        let q3 = ((b_hi >> ((((imm >> 3) & 1) as u32) * 64)) as u64) as u128;
                        q2 | (q3 << 64)
                    } else {
                        0
                    };
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
                    let imm = cpu.read_op(ins, imm_op, 8) as u32;
                    let align128 = |va: u128, vb: u128| -> u128 {
                        if imm >= 32 {
                            0
                        } else if imm >= 16 {
                            va >> ((imm - 16) * 8)
                        } else if imm == 0 {
                            vb
                        } else {
                            (vb >> (imm * 8)) | (va << ((16 - imm) * 8))
                        }
                    };
                    let lo = align128(a_lo, b_lo);
                    let hi = if ymm { align128(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Fnstcw | Mnemonic::Fstcw => {
                // Default x87 control word (all exceptions masked, 64-bit precision, round-to-nearest).
                let a = cpu.eff_addr(ins);
                cpu.write_mem(a, 16, 0x037F);
            }
            Mnemonic::Fldcw => {}
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
                    let clmul128 = |va: u128, vb: u128| -> u128 {
                        let x = if (imm & 0x01) != 0 { (va >> 64) as u64 } else { va as u64 };
                        let y = if (imm & 0x10) != 0 { (vb >> 64) as u64 } else { vb as u64 };
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
                    };
                    let lo = clmul128(a_lo, b_lo);
                    let hi = if ymm { clmul128(a_hi, b_hi) } else { 0 };
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
                // SSE4.2 CRC32C (Castagnoli, reflected polynomial 0x82F63B78) via 256-entry LUT.
                let src_bits = Cpu::op_bits(ins, 1);
                let v = cpu.read_op(ins, 1, src_bits);
                let mut crc = cpu.read_op(ins, 0, 32) as u32;
                let nbytes = src_bits >> 3;
                for i in 0..nbytes {
                    let b = ((v >> (8 * i)) as u8) as u32;
                    crc = CRC32C_TABLE[((crc ^ b) & 0xFF) as usize] ^ (crc >> 8);
                }
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
                    let imm = cpu.read_op(ins, 2, 8) as u32;
                    let q = ymm_qwords(s_lo, s_hi);
                    let r: [u64; 4] = [
                        q[(imm & 3) as usize],
                        q[((imm >> 2) & 3) as usize],
                        q[((imm >> 4) & 3) as usize],
                        q[((imm >> 6) & 3) as usize],
                    ];
                    let (lo, hi) = qwords_ymm(&r);
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
            Mnemonic::Phaddd | Mnemonic::Phaddw | Mnemonic::Phsubd | Mnemonic::Phsubw | Mnemonic::Vphaddd
            | Mnemonic::Vphaddw | Mnemonic::Vphsubd | Mnemonic::Vphsubw => {
                // Horizontal add/sub of adjacent pairs: low half of each 128-bit lane from src1,
                // high half from src2 (PHSUB: even - odd). 64-bit MMX forms are not handled.
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
                        Mnemonic::Sha1rnds4 => sha1rnds4(src1, src2, ins.immediate8()),
                        Mnemonic::Sha1nexte => sha1nexte(src1, src2),
                        Mnemonic::Sha1msg1 => sha1msg1(src1, src2),
                        Mnemonic::Sha1msg2 => sha1msg2(src1, src2),
                        Mnemonic::Sha256rnds2 => sha256rnds2(src1, src2, cpu.xmm[0]),
                        Mnemonic::Sha256msg1 => sha256msg1(src1, src2),
                        _ => sha256msg2(src1, src2),
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

#[inline(always)]
fn sha_lanes(v: u128) -> [u32; 4] {
    [v as u32, (v >> 32) as u32, (v >> 64) as u32, (v >> 96) as u32]
}

#[inline(always)]
fn sha_pack(w: [u32; 4]) -> u128 {
    (w[0] as u128)
        | ((w[1] as u128) << 32)
        | ((w[2] as u128) << 64)
        | ((w[3] as u128) << 96)
}

/// Intel SDM `SHA1RNDS4 xmm1, xmm2/m128, imm8`.
pub(super) fn sha1rnds4(src1: u128, src2: u128, imm8: u8) -> u128 {
    let [mut d, mut c, mut b, mut a] = sha_lanes(src1);
    let [w3, w2, w1, w0e] = sha_lanes(src2);
    let func = imm8 & 3;
    let k: u32 = match func {
        0 => 0x5A82_7999,
        1 => 0x6ED9_EBA1,
        2 => 0x8F1B_BCDC,
        _ => 0xCA62_C1D6,
    };
    let f = |b: u32, c: u32, d: u32| -> u32 {
        match func {
            0 => (b & c) ^ ((!b) & d),
            1 | 3 => b ^ c ^ d,
            _ => (b & c) ^ (b & d) ^ (c & d),
        }
    };
    // Round 0: W0E already includes E_0.
    let a1 = f(b, c, d).wrapping_add(a.rotate_left(5)).wrapping_add(w0e).wrapping_add(k);
    let mut e = d;
    d = c;
    c = b.rotate_left(30);
    b = a;
    a = a1;
    for w in [w1, w2, w3] {
        let an = f(b, c, d)
            .wrapping_add(a.rotate_left(5))
            .wrapping_add(w)
            .wrapping_add(e)
            .wrapping_add(k);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = an;
    }
    sha_pack([d, c, b, a])
}

/// Intel SDM `SHA1NEXTE xmm1, xmm2/m128`.
pub(super) fn sha1nexte(src1: u128, src2: u128) -> u128 {
    let tmp = ((src1 >> 96) as u32).rotate_left(30);
    let [s0, s1, s2, s3] = sha_lanes(src2);
    sha_pack([s0, s1, s2, s3.wrapping_add(tmp)])
}

/// Intel SDM `SHA1MSG1 xmm1, xmm2/m128`.
pub(super) fn sha1msg1(src1: u128, src2: u128) -> u128 {
    let [w3, w2, w1, w0] = sha_lanes(src1);
    let [_, _, w5, w4] = sha_lanes(src2);
    sha_pack([w5 ^ w3, w4 ^ w2, w3 ^ w1, w2 ^ w0])
}

/// Intel SDM `SHA1MSG2 xmm1, xmm2/m128`.
pub(super) fn sha1msg2(src1: u128, src2: u128) -> u128 {
    let [s0, s1, s2, s3] = sha_lanes(src1);
    let [w15, w14, w13, _] = sha_lanes(src2);
    let w16 = (s3 ^ w13).rotate_left(1);
    let w17 = (s2 ^ w14).rotate_left(1);
    let w18 = (s1 ^ w15).rotate_left(1);
    let w19 = (s0 ^ w16).rotate_left(1);
    sha_pack([w19, w18, w17, w16])
}

/// Intel SDM `SHA256RNDS2 xmm1, xmm2/m128, <XMM0>`.
pub(super) fn sha256rnds2(src1: u128, src2: u128, xmm0: u128) -> u128 {
    let [mut h, mut g, mut d, mut c] = sha_lanes(src1);
    let [mut f, mut e, mut b, mut a] = sha_lanes(src2);
    let wk = [xmm0 as u32, (xmm0 >> 32) as u32];
    for w in wk {
        let ch = (e & f) ^ ((!e) & g);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let t1 = ch.wrapping_add(s1).wrapping_add(w).wrapping_add(h);
        let a_new = t1.wrapping_add(maj).wrapping_add(s0);
        let e_new = t1.wrapping_add(d);
        h = g;
        g = f;
        f = e;
        e = e_new;
        d = c;
        c = b;
        b = a;
        a = a_new;
    }
    sha_pack([f, e, b, a])
}

/// Intel SDM `SHA256MSG1 xmm1, xmm2/m128`.
pub(super) fn sha256msg1(src1: u128, src2: u128) -> u128 {
    let [w0, w1, w2, w3] = sha_lanes(src1);
    let w4 = src2 as u32;
    let s0 = |x: u32| x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    sha_pack([
        w0.wrapping_add(s0(w1)),
        w1.wrapping_add(s0(w2)),
        w2.wrapping_add(s0(w3)),
        w3.wrapping_add(s0(w4)),
    ])
}

/// Intel SDM `SHA256MSG2 xmm1, xmm2/m128`.
pub(super) fn sha256msg2(src1: u128, src2: u128) -> u128 {
    let [s0, s1, s2, s3] = sha_lanes(src1);
    let [_, _, w14, w15] = sha_lanes(src2);
    let sig1 = |x: u32| x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
    let w16 = s0.wrapping_add(sig1(w14));
    let w17 = s1.wrapping_add(sig1(w15));
    let w18 = s2.wrapping_add(sig1(w16));
    let w19 = s3.wrapping_add(sig1(w17));
    sha_pack([w16, w17, w18, w19])
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
