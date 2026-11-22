//! Decompiled hot functions shard Z4B (`0x141ef44d0..0x14202b8d0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn permilps(a: [f32; 4], imm: u8) -> [f32; 4] {
    [
        a[(imm & 3) as usize],
        a[((imm >> 2) & 3) as usize],
        a[((imm >> 4) & 3) as usize],
        a[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn shufps(a: [f32; 4], b: [f32; 4], imm: u8) -> [f32; 4] {
    [
        a[(imm & 3) as usize],
        a[((imm >> 2) & 3) as usize],
        b[((imm >> 4) & 3) as usize],
        b[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn addps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[inline(always)]
fn subps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

#[inline(always)]
fn mulps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

#[inline(always)]
fn blendps(a: [f32; 4], b: [f32; 4], mask: u8) -> [f32; 4] {
    [
        if (mask & 1) != 0 { b[0] } else { a[0] },
        if (mask & 2) != 0 { b[1] } else { a[1] },
        if (mask & 4) != 0 { b[2] } else { a[2] },
        if (mask & 8) != 0 { b[3] } else { a[3] },
    ]
}

#[inline(always)]
fn hsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - a[1], a[2] - a[3], b[0] - b[1], b[2] - b[3]]
}

#[inline(always)]
fn haddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + a[1], a[2] + a[3], b[0] + b[1], b[2] + b[3]]
}

#[inline(always)]
fn addsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] + b[1], a[2] - b[2], a[3] + b[3]]
}

#[inline(always)]
fn xorps_u32(a: [f32; 4], mask: [u32; 4]) -> [f32; 4] {
    [
        f32::from_bits(a[0].to_bits() ^ mask[0]),
        f32::from_bits(a[1].to_bits() ^ mask[1]),
        f32::from_bits(a[2].to_bits() ^ mask[2]),
        f32::from_bits(a[3].to_bits() ^ mask[3]),
    ]
}

#[inline(always)]
fn qmul_simd(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
    let x3 = permilps(lhs, 0x11);
    let x1 = mulps(x3, rhs);
    let x2 = permilps(lhs, 0xbb);
    let x5 = permilps(rhs, 0x1b);
    let x0 = mulps(x2, x5);
    let x4 = hsubps(x1, x0);
    let x0_b = mulps(x3, x5);
    let x2_b = mulps(x2, rhs);
    let x3_b = haddps(x2_b, x0_b);
    let x0_c = shufps(x4, x3_b, 0xb1);
    let x1_c = blendps(x3_b, x4, 0x0c);
    let x1_d = addsubps(x1_c, x0_c);
    permilps(x1_d, 0x9c)
}

/// `unknown_141ef44d0`
/// Original: 0x141ef44d0 (110 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ef44d0(param_1: usize, param_2: usize) -> usize {
    let inner = rd::<usize>(param_2 + 0x78);
    let val = rd::<u64>(inner + 0x18);
    if val == 0x100 {
        wr::<u8>(param_1 + 0x10, 0);
        return param_1;
    }
    let mut opt: [u64; 2] = [val, 1];
    exe::call_va!(
        0x1_41ef_4380,
        (param_1, param_2, opt.as_mut_ptr() as usize)
    );
    param_1
}

/// `unknown_141f288b0`
/// Original: 0x141f288b0 (111 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f288b0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let idx = rd::<usize>(param_1 + 0x20);
    let p0 = rd::<usize>(param_1);
    let p1 = rd::<usize>(param_1 + 0x08);
    let elem_14 = p0.wrapping_add(idx.wrapping_mul(0x14));
    let elem_18 = p1.wrapping_add(idx.wrapping_mul(0x18));
    if rd::<u8>(elem_14) != 0 {
        let dt = rd::<f32>(param_2) * rd::<f32>(param_3);
        let dt_xmm1 = dt.to_bits() as u128;
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4103_93f0,
            elem_14 as u64,
            0,
            elem_18 as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dt_xmm1, 0, 0],
        );
        let p2 = rd::<usize>(param_1 + 0x10);
        wr::<u32>(p2.wrapping_add(idx.wrapping_mul(4)), rd::<u32>(elem_18));
    }
}

/// `unknown_141f3a990`
/// Original: 0x141f3a990 (136 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f3a990(param_1: usize, param_2: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_3f40) as usize) == 0 {
        return;
    }
    let rax = rd::<usize>(param_1);
    let rbx = rd::<usize>(param_1 + 0x18);
    let item_ptr = rax.wrapping_add(rbx.wrapping_shl(5)).wrapping_add(0x10);
    let mut stack_buf: [u64; 6] = [
        rd::<u64>(item_ptr),
        rd::<u64>(item_ptr + 0x08),
        rd::<u64>(param_2),
        rd::<u64>(param_2 + 0x08),
        rd::<u64>(param_2 + 0x10),
        rbx as u64,
    ];
    let rdx_ptr = stack_buf.as_mut_ptr() as usize;
    let rcx_ptr = rdx_ptr.wrapping_add(0x10);
    let res = exe::call_va!(0x1_41f3_aae0, (rcx_ptr, rdx_ptr) -> u8);
    let out_arr = rd::<usize>(param_1 + 0x08);
    wr::<u8>(out_arr.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(1), res);
}

/// `unknown_141f79660`
/// Original: 0x141f79660 (360 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141f79660(param_1: usize) {
    let g_300 = exe::ptr::<u64>(0x1_43a0_8300) as usize;

    let mut rbx = rd::<usize>(param_1 + 0x58);
    let rsi = rbx.wrapping_add((rd::<u64>(param_1 + 0x40) as usize).wrapping_shl(4));
    while rbx != rsi {
        let rax = rd::<u64>(rbx);
        if rax == rd::<u64>(param_1 + 0x48)
            || (rd::<u64>(param_1 + 0x30) > 0 && rd::<u64>(param_1 + 0x28) == rax)
        {
            rbx = rbx.wrapping_add(0x10);
        } else {
            break;
        }
    }
    while rbx != rsi {
        let r8 = rd::<u64>(rbx + 0x08);
        let rcx = rd::<u64>(g_300);
        exe::call_va!(0x1_4248_b1c0, (rcx, rbx, r8));
        rbx = rbx.wrapping_add(0x10);
        while rbx != rsi {
            let rax = rd::<u64>(rbx);
            if rax == rd::<u64>(param_1 + 0x48)
                || (rd::<u64>(param_1 + 0x30) > 0 && rd::<u64>(param_1 + 0x28) == rax)
            {
                rbx = rbx.wrapping_add(0x10);
            } else {
                break;
            }
        }
    }

    let mut rbx2 = rd::<usize>(param_1 + 0xb8);
    let rsi2 = rbx2.wrapping_add((rd::<u64>(param_1 + 0xa0) as usize).wrapping_shl(4));
    while rbx2 != rsi2 {
        let rax = rd::<u64>(rbx2);
        if rax == rd::<u64>(param_1 + 0xa8)
            || (rd::<u64>(param_1 + 0x90) > 0 && rd::<u64>(param_1 + 0x88) == rax)
        {
            rbx2 = rbx2.wrapping_add(0x10);
        } else {
            break;
        }
    }
    let rbp2 = rd::<usize>(param_1 + 0xb8)
        .wrapping_add((rd::<u64>(param_1 + 0xa0) as usize).wrapping_shl(4));
    while rbx2 != rbp2 {
        let r8 = rd::<u64>(rbx2 + 0x08);
        let rcx = rd::<u64>(g_300);
        exe::call_va!(0x1_4248_b1c0, (rcx, rbx2, r8));
        rbx2 = rbx2.wrapping_add(0x10);
        while rbx2 != rsi2 {
            let rax = rd::<u64>(rbx2);
            if rax == rd::<u64>(param_1 + 0xa8)
                || (rd::<u64>(param_1 + 0x90) > 0 && rd::<u64>(param_1 + 0x88) == rax)
            {
                rbx2 = rbx2.wrapping_add(0x10);
            } else {
                break;
            }
        }
    }

    wr::<u8>(param_1 + 0x110, 1);
}

/// `unknown_141f91e90`
/// Original: 0x141f91e90 (446 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141f91e90(param_1: usize) {
    let idx = rd::<usize>(param_1 + 0x20);
    let rbx = rd::<usize>(param_1 + 0x10).wrapping_add(idx.wrapping_shl(7));
    let rax = rd::<usize>(param_1 + 0x08).wrapping_add(idx.wrapping_shl(5));
    let ecx = rd::<u32>(rd::<usize>(param_1).wrapping_add(idx.wrapping_mul(4)));

    if rd::<u8>(rbx + 0x78) != 0 {
        if ecx != 0 {
            exe::call_va!(0x1_4110_b9a0, (rbx + 0x70));
            wr::<u8>(rbx + 0x78, 0);
        }
        return;
    }

    if ecx != 0 {
        return;
    }

    let xmm1 = rd::<[f32; 4]>(rax);
    let xmm5_const = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_83b0) as usize);
    let xmm8_const = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_8240) as usize);

    let xmm0 = mulps(xmm5_const, xmm1);
    let xmm4 = mulps(xmm0, xmm1);
    let xmm0_b = permilps(xmm1, 0xe0);
    let xmm2 = mulps(xmm0_b, xmm5_const);
    let xmm3 = xmm1;
    let xmm1_b = permilps(xmm1, 0xd9);
    let xmm0_c = permilps(xmm3, 0xff);
    let xmm7 = mulps(xmm2, xmm1_b);
    let xmm1_c = mulps(xmm5_const, xmm3);
    let xmm5 = mulps(xmm1_c, xmm0_c);

    let xmm0_d = permilps(xmm4, 0xc1);
    let xmm1_d = permilps(xmm4, 0xda);
    let xmm3_b = permilps(xmm5, 0xd2);
    let xmm2_b = subps(xmm8_const, xmm0_d);
    let xmm6 = subps(xmm2_b, xmm1_d);
    let xmm1_e = permilps(xmm5, 0xc9);
    let xmm2_zero = [0.0f32; 4];
    let xmm0_e = permilps(xmm7, 0xd8);
    let xmm4_b = addps(xmm3_b, xmm0_e);
    let xmm5_b = shufps(xmm6, xmm4_b, 0x44);
    let xmm0_f = permilps(xmm7, 0xe1);
    let xmm3_c = subps(xmm0_f, xmm1_e);
    let xmm1_f = shufps(xmm3_c, xmm2_zero, 0xee);
    let xmm0_g = shufps(xmm3_c, xmm2_zero, 0x44);
    let xmm2_c = shufps(xmm5_b, xmm0_g, 0x88);
    let xmm0_h = shufps(xmm5_b, xmm0_g, 0xdd);
    let xmm5_c = permilps(xmm0_h, 0xd2);
    let xmm4_c = shufps(xmm6, xmm4_b, 0xee);
    let xmm0_i = shufps(xmm4_c, xmm1_f, 0x88);
    let xmm1_g = blendps(xmm8_const, rd::<[f32; 4]>(rax + 0x10), 0x07);
    let xmm3_d = permilps(xmm0_i, 0xc9);
    let row1 = shufps(xmm5_c, xmm3_d, 0x49);
    let xmm0_j = shufps(xmm2_c, xmm5_c, 0x4e);
    let row0 = shufps(xmm2_c, xmm0_j, 0x84);
    let xmm0_k = shufps(xmm3_d, xmm1_g, 0x4e);
    let row2 = shufps(xmm0_k, xmm1_g, 0x98);

    let mut mat: [[f32; 4]; 6] = [row0, row1, row2, row0, row1, row2];
    exe::call_va!(0x1_41f9_3520, (rbx, mat.as_mut_ptr() as usize));
}

/// `unknown_141f93520`
/// Original: 0x141f93520 (135 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f93520(param_1: usize, param_2: usize) {
    let mut local_30 = [0u64; 8];
    let mut local_70 = [0u64; 18];
    let p30 = local_30.as_mut_ptr() as usize;
    let p70 = local_70.as_mut_ptr() as usize;

    exe::call_va!(
        0x1_4204_a880,
        (
            p30,
            param_2,
            rd::<u64>(param_1 + 0x1c),
            param_1 + 0x24,
            param_1 + 0x2c,
            rd::<u32>(param_1 + 0x68),
        )
    );
    exe::call_va!(0x1_4204_a3b0, (p70, param_1, p30));
    exe::call_va!(0x1_4110_b810, (param_1 + 0x70, p70));
    wr::<u8>(param_1 + 0x78, 1);

    let rdx = rd::<usize>(p70 + 0x48);
    if rdx != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((rdx + 0x24) as u64, u32::MAX);
        if old == 1 {
            let rcx = rd::<usize>(rdx + 0x28);
            if rcx != 0 {
                exe::call_va!(0x1_4241_d6f0, (rcx, rdx));
            }
        }
    }
}

/// `coregame_141fc89c0`
/// Original: 0x141fc89c0 (158 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141fc89c0(param_1: usize, param_2: usize) {
    exe::call_va!(0x1_41e0_4ee0, (param_1 + 0x1c8, rd::<u64>(param_2)));
    let id = rd::<i32>(param_1 + 0x18);
    let mgr = if id == -1 {
        0u64
    } else {
        let mut key_slot: u32 = id as u32;
        let sp = exe::call_va!(
            0x1_4141_0810,
            (0usize, (&mut key_slot as *mut u32) as usize) -> usize
        );
        rd::<u64>(rd::<usize>(sp) + 0x28)
    };
    exe::call_va!(0x1_41e0_bc70, (param_1 + 0x228, mgr, rd::<u64>(param_2)));
    exe::call_va!(0x1_41e1_0100, (param_1 + 0x168, param_1, rd::<u64>(param_2)));
    exe::call_va!(0x1_41fc_7e80, (param_1, param_2));
    let rax = rd::<usize>(param_2);
    let idx = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce6c) as usize) as usize;
    let tbl = rd::<usize>(rax + 0x580e0);
    let rcx = rd::<u64>(tbl.wrapping_add(idx * 8));
    exe::call_va!(0x1_41e0_ccf0, (rcx, param_1));
}

#[inline(always)]
unsafe fn eval_transform_chain(
    base_a: usize,
    base_b: usize,
    c_addr: usize,
    d_addr: usize,
    swap_ab: bool,
) -> [[f32; 4]; 2] {
    let mask_b00 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7b00) as usize);
    let mask_760 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let scale_ab0 = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7ab0) as usize);

    let t_a_inv = xorps_u32(rd::<[f32; 4]>(base_a + 0x10), mask_760);
    let q_a = xorps_u32(rd::<[f32; 4]>(base_a), mask_b00);
    let q_a_d2 = permilps(q_a, 0xd2);
    let q_a_c9 = permilps(q_a, 0xc9);
    let q_a_ff = permilps(q_a, 0xff);

    let t_a_cross = mulps(
        subps(
            mulps(permilps(t_a_inv, 0xd2), q_a_c9),
            mulps(permilps(t_a_inv, 0xc9), q_a_d2),
        ),
        scale_ab0,
    );

    let b_lo = rd::<[f32; 4]>(base_b);
    let b_hi = rd::<[f32; 4]>(base_b + 0x10);
    let t_b_cross = mulps(
        subps(
            mulps(permilps(b_hi, 0xd2), q_a_c9),
            mulps(permilps(b_hi, 0xc9), q_a_d2),
        ),
        scale_ab0,
    );
    let q_ab = qmul_simd(b_lo, q_a);

    let c_lo = rd::<[f32; 4]>(c_addr + 0x80);
    let c_hi = rd::<[f32; 4]>(c_addr + 0x90);
    let q_ab_c9 = permilps(q_ab, 0xc9);
    let q_ab_d2 = permilps(q_ab, 0xd2);
    let t_c_cross = mulps(
        subps(
            mulps(permilps(c_hi, 0xd2), q_ab_c9),
            mulps(permilps(c_hi, 0xc9), q_ab_d2),
        ),
        scale_ab0,
    );
    let q_abc = qmul_simd(c_lo, q_ab);

    let d_lo = rd::<[f32; 4]>(d_addr);
    let d_hi = rd::<[f32; 4]>(d_addr + 0x10);
    let q_abc_c9 = permilps(q_abc, 0xc9);
    let q_abc_d2 = permilps(q_abc, 0xd2);
    let t_d_cross = mulps(
        subps(
            mulps(permilps(d_hi, 0xd2), q_abc_c9),
            mulps(permilps(d_hi, 0xc9), q_abc_d2),
        ),
        scale_ab0,
    );
    let out_q = qmul_simd(d_lo, q_abc);

    let term_a = addps(
        subps(
            mulps(permilps(t_a_cross, 0xd2), q_a_c9),
            mulps(permilps(t_a_cross, 0xc9), q_a_d2),
        ),
        addps(mulps(t_a_cross, q_a_ff), t_a_inv),
    );
    let term_b = addps(
        subps(
            mulps(permilps(t_b_cross, 0xd2), q_a_c9),
            mulps(permilps(t_b_cross, 0xc9), q_a_d2),
        ),
        addps(mulps(t_b_cross, q_a_ff), b_hi),
    );
    let term_c = addps(
        subps(
            mulps(permilps(t_c_cross, 0xd2), q_ab_c9),
            mulps(permilps(t_c_cross, 0xc9), q_ab_d2),
        ),
        addps(mulps(permilps(q_ab, 0xff), t_c_cross), c_hi),
    );
    let term_d = addps(
        subps(
            mulps(permilps(t_d_cross, 0xd2), q_abc_c9),
            mulps(permilps(t_d_cross, 0xc9), q_abc_d2),
        ),
        addps(mulps(permilps(q_abc, 0xff), t_d_cross), d_hi),
    );

    let sum_ab = if swap_ab {
        addps(term_b, term_a)
    } else {
        addps(term_a, term_b)
    };
    let out_t = addps(addps(sum_ab, term_c), term_d);
    [out_q, out_t]
}

/// `unknown_141fd6020`
/// Original: 0x141fd6020 (2518 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fd6020(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let rsi = rd::<usize>(param_1 + 0x38);
    let r15 = rd::<usize>(param_1 + 0x08);
    let mut r10 = rd::<u64>(r15.wrapping_add(rsi.wrapping_mul(8)));
    if (r10 as i32) == -1 {
        r10 = u64::MAX;
    }

    let mut valid_handle = u64::MAX;
    if (r10 as i32) != -1 || ((r10 >> 32) as i32) != -1 {
        let rdx_obj = rd::<usize>(param_2);
        let count_rcx = rd::<usize>(param_2 + 0x08);
        let arr_u16 = rd::<usize>(param_2 + 0x10);
        let idx_u32 = r10 as u32;
        let cap_u32 = rd::<u32>(rdx_obj + 0x20);
        if (idx_u32 as u64) < (cap_u32 as u64) {
            let slot_val = rd::<u64>(rd::<usize>(rdx_obj + 0x18).wrapping_add((idx_u32 as usize) * 8));
            if (slot_val as u32) == idx_u32
                && ((slot_val >> 32) as u32) == ((r10 >> 32) as u32)
                && (idx_u32 as i32) != -1
            {
                let target_u16 =
                    rd::<u16>(rd::<usize>(rdx_obj + 0x30).wrapping_add((idx_u32 as usize) * 8));
                if target_u16 == 0xffff {
                    let abort_fn = rd::<usize>(exe::ptr::<usize>(0x1_4264_67e8) as usize);
                    exe::call_va!(abort_fn, ());
                    return;
                }
                let mut first = arr_u16;
                let end = arr_u16.wrapping_add(count_rcx.wrapping_mul(2));
                let mut rem = (end.wrapping_sub(first) as isize) >> 1;
                while rem > 0 {
                    let half = ((rem as usize) >> 1) as isize;
                    let mid = first.wrapping_add((half as usize) * 2);
                    if rd::<u16>(mid) < target_u16 {
                        first = mid.wrapping_add(2);
                        rem -= half + 1;
                    } else {
                        rem = half;
                    }
                }
                if first != end && target_u16 >= rd::<u16>(first) {
                    valid_handle = r10;
                }
            }
        }
    }

    if (valid_handle as i32) == -1 {
        return;
    }

    let rbx_entry = rd::<usize>(param_1).wrapping_add(rsi.wrapping_mul(0x30));
    let mut local_40 = [0u64; 5];
    exe::call_va!(
        0x1_4117_0720,
        (param_2, local_40.as_mut_ptr() as usize, valid_handle as u32)
    );

    let rax_40 = local_40[0] as usize;
    let r12_48 = local_40[1] as usize;
    let r13_50 = local_40[2] as usize;
    let r11_off = (local_40[4] as usize).wrapping_shl(5);
    let r9d = rd::<u32>(rbx_entry);
    let r14 = rd::<usize>(rax_40.wrapping_add(r11_off));
    let r10_48 = rd::<usize>(r14 + 0x48);
    let edx_cnt = rd::<u32>(r10_48 + 0x48) as usize;
    if edx_cnt == 0 {
        return;
    }

    let r8_arr = rd::<usize>(r10_48 + 0x40);
    let mut cx: u16 = 0;
    loop {
        let item_off = (cx as usize).wrapping_mul(0xb0);
        if rd::<u32>(r8_arr.wrapping_add(item_off).wrapping_add(0x28)) == r9d {
            break;
        }
        cx = cx.wrapping_add(1);
        if (cx as usize) == edx_cnt {
            return;
        }
    }
    if cx == 0xffff {
        return;
    }

    let edx_idx = cx as usize;
    let rcx_bone = rd::<i32>(rd::<usize>(r14 + 0xd0).wrapping_add(edx_idx * 4)) as isize;
    if rcx_bone == -1 {
        return;
    }

    let c_addr = r8_arr.wrapping_add(edx_idx.wrapping_mul(0xb0));
    let d_addr = rbx_entry.wrapping_add(0x10);
    let rdx_20 = rd::<usize>(r14 + 0x20);
    let bone_off = (rcx_bone as usize).wrapping_shl(5);

    let mut out_d0 = eval_transform_chain(
        r12_48.wrapping_add(r11_off),
        rd::<usize>(rdx_20 + 0x50).wrapping_add(bone_off),
        c_addr,
        d_addr,
        false,
    );
    let mut out_f0 = eval_transform_chain(
        r13_50.wrapping_add(r11_off),
        rd::<usize>(rdx_20 + 0x58).wrapping_add(bone_off),
        c_addr,
        d_addr,
        true,
    );

    let rbx_30 = rd::<u64>(param_1 + 0x30);
    let mut arg1_110: [u64; 5] = [
        r15 as u64,
        rd::<u64>(param_1 + 0x10),
        rd::<u64>(param_1 + 0x18),
        rbx_30,
        rsi as u64,
    ];
    let mut arg2_20: [u64; 3] = [
        rd::<u64>(param_3),
        rd::<u64>(param_3 + 0x08),
        rd::<u64>(param_3 + 0x10),
    ];
    exe::call_va!(
        0x1_41e2_3170,
        (
            arg1_110.as_mut_ptr() as usize,
            arg2_20.as_mut_ptr() as usize,
            out_d0.as_mut_ptr() as usize,
        )
    );

    arg1_110 = [
        r15 as u64,
        rd::<u64>(param_1 + 0x20),
        rd::<u64>(param_1 + 0x28),
        rbx_30,
        rsi as u64,
    ];
    arg2_20 = [
        rd::<u64>(param_4),
        rd::<u64>(param_4 + 0x08),
        rd::<u64>(param_4 + 0x10),
    ];
    exe::call_va!(
        0x1_41e2_2e20,
        (
            arg1_110.as_mut_ptr() as usize,
            arg2_20.as_mut_ptr() as usize,
            out_f0.as_mut_ptr() as usize,
        )
    );
}

#[inline(always)]
unsafe fn skip_empty_ctrl_c0(mut ctrl: usize, mut elem: usize) -> (usize, usize) {
    while rd::<i8>(ctrl) < -1 {
        let mut mask: u32 = 0;
        for i in 0..16usize {
            if rd::<i8>(ctrl + i) < -1 {
                mask |= 1u32 << i;
            }
        }
        let step = mask.wrapping_add(1).trailing_zeros() as usize;
        ctrl = ctrl.wrapping_add(step);
        elem = elem.wrapping_add(step.wrapping_mul(0xc0));
    }
    (ctrl, elem)
}

/// `unknown_14202b8d0`
/// Original: 0x14202b8d0 (1038 bytes, Throws::Discard)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_14202b8d0(param_1: usize, param_2: f32) {
    unknown_14202b8d0_impl(param_1, param_2);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_14202b8d0(param_1: usize, _xmm0: f32, param_2: f32) {
    unknown_14202b8d0_impl(param_1, param_2);
}

#[inline(always)]
unsafe fn unknown_14202b8d0_impl(param_1: usize, param_2: f32) {
    let ctrl_beg = rd::<usize>(param_1 + 0x30);
    let elem_beg = rd::<usize>(param_1 + 0x38);
    let (mut rdi, mut rbx) = skip_empty_ctrl_c0(ctrl_beg, elem_beg);
    let ctrl_end = ctrl_beg.wrapping_add(rd::<usize>(param_1 + 0x48));

    if rdi != ctrl_end {
        let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        loop {
            let xm0 = param_2 + rd::<f32>(rbx + 0xb4);
            let xm1 = xm0 / rd::<f32>(rbx + 0xb0);
            wr::<f32>(rbx + 0xb4, xm0);
            let xm7 = if xm1 < one { xm1 } else { one };

            let mut out_40: [usize; 2] = [0; 2];
            exe::call_va!(
                0x1_411b_9b80,
                (param_1, out_40.as_mut_ptr() as usize, rbx)
            );
            let rcx = out_40[1].wrapping_add(0x10);
            if rcx != 0 {
                let kind = rd::<u8>(rcx + 0x40);
                if kind == 2 {
                    if rd::<u8>(rbx + 0x50) != 2 || rd::<u8>(rbx + 0xa0) != 2 {
                        exe::call_va!(0x1_4020_0070, ());
                        return;
                    }
                    let v = (one - xm7) * rd::<f32>(rbx + 0x10) + xm7 * rd::<f32>(rbx + 0x60);
                    wr::<f32>(rcx, v);
                } else if kind == 4 {
                    if rd::<u8>(rbx + 0x50) != 4 || rd::<u8>(rbx + 0xa0) != 4 {
                        exe::call_va!(0x1_4020_0070, ());
                        return;
                    }
                    for k in 0..2usize {
                        let a = rd::<f32>(rbx + 0x10 + k * 4);
                        let b = rd::<f32>(rbx + 0x60 + k * 4);
                        wr::<f32>(rcx + k * 4, (b - a) * xm7 + a);
                    }
                } else if kind == 6 {
                    if rd::<u8>(rbx + 0x50) != 6 || rd::<u8>(rbx + 0xa0) != 6 {
                        exe::call_va!(0x1_4020_0070, ());
                        return;
                    }
                    for k in 0..3usize {
                        let a = rd::<f32>(rbx + 0x10 + k * 4);
                        let b = rd::<f32>(rbx + 0x60 + k * 4);
                        wr::<f32>(rcx + k * 4, (b - a) * xm7 + a);
                    }
                } else if kind == 8 {
                    if rd::<u8>(rbx + 0x50) != 8 || rd::<u8>(rbx + 0xa0) != 8 {
                        exe::call_va!(0x1_4020_0070, ());
                        return;
                    }
                    for k in 0..4usize {
                        let a = rd::<f32>(rbx + 0x10 + k * 4);
                        let b = rd::<f32>(rbx + 0x60 + k * 4);
                        wr::<f32>(rcx + k * 4, (b - a) * xm7 + a);
                    }
                }
            }

            let cur_t = rd::<f32>(rbx + 0xb4);
            let max_t = rd::<f32>(rbx + 0xb0);
            let (next_rdi, next_rbx) =
                skip_empty_ctrl_c0(rdi.wrapping_add(1), rbx.wrapping_add(0xc0));
            if !(cur_t < max_t) {
                let mut erase_it: [usize; 2] = [rdi, rbx];
                exe::call_va!(
                    0x1_411b_90b0,
                    (param_1 + 0x30, erase_it.as_mut_ptr() as usize)
                );
            }
            rdi = next_rdi;
            rbx = next_rbx;
            let end_now = rd::<usize>(param_1 + 0x30).wrapping_add(rd::<usize>(param_1 + 0x48));
            if rdi == end_now {
                break;
            }
        }
    }

    let mut cur_60 = rd::<usize>(param_1 + 0x60);
    let end_60 = cur_60.wrapping_add((rd::<u32>(param_1 + 0x68) as usize) * 8);
    while cur_60 != end_60 {
        let mut out_40: [usize; 2] = [0; 2];
        exe::call_va!(
            0x1_411b_9b80,
            (param_1, out_40.as_mut_ptr() as usize, cur_60)
        );
        let rax = out_40[1].wrapping_add(0x10);
        if rax != 0 {
            let tag = rd::<u8>(rax + 0x40);
            if tag == 0x14 || tag == 0x13 {
                wr::<u32>(rax + 0x08, 0);
            }
        }
        cur_60 = cur_60.wrapping_add(8);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141ef44d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_ARGS: (u64, u8) = (0, 0);

    unsafe extern "win64" fn mock_4380(p1: usize, _p2: usize, opt: usize) {
        let v = rd::<u64>(opt);
        let has = rd::<u8>(opt + 0x08);
        CALL_ARGS = (v, has);
        wr::<u64>(p1, v ^ 0x55aa);
        wr::<u8>(p1 + 0x10, has);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41ef_4380, mock_4380 as *const ())?;

    let run = |f: F, val: u64| -> (bool, u64, u8, (u64, u8)) {
        CALL_ARGS = (0, 0);
        let mut inner = [0u8; 0x20];
        let ip = inner.as_mut_ptr() as usize;
        wr::<u64>(ip + 0x18, val);

        let mut p2 = [0u8; 0x80];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x78, ip);

        let mut p1 = [0xffu8; 0x20];
        let p1p = p1.as_mut_ptr() as usize;
        let ret = f(p1p, p2p);
        (ret == p1p, rd::<u64>(p1p), rd::<u8>(p1p + 0x10), CALL_ARGS)
    };

    for val in [0x100u64, 0x42] {
        let o = run(f_o, val);
        let n = run(f_n, val);
        crate::diff_eq!(o, n, "141ef44d0 val={}", val);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f288b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, active: u8| -> (u32, [u32; 6]) {
        let mut p0 = [0u8; 0x30];
        let mut p1 = [0u32; 12];
        let mut p2 = [0xdead_beefu32; 4];

        let b0 = p0.as_mut_ptr() as usize + 0x14;
        let b1 = p1.as_mut_ptr() as usize + 0x18;

        wr::<u8>(b0, active);
        wr::<f32>(b0 + 0x04, 2.0);
        wr::<f32>(b0 + 0x08, 8.0);
        wr::<f32>(b0 + 0x0c, 2.0);
        wr::<f32>(b0 + 0x10, 0.5);

        wr::<f32>(b1, 1.0);
        wr::<f32>(b1 + 0x04, 2.0);
        wr::<f32>(b1 + 0x08, 3.0);
        wr::<f32>(b1 + 0x0c, 4.0);
        wr::<f32>(b1 + 0x10, 1.0);
        wr::<f32>(b1 + 0x14, 2.0);

        let dt: f32 = 0.25;
        let scale: f32 = 0.5;
        let hdr: [u64; 5] = [
            p0.as_mut_ptr() as usize as u64,
            p1.as_mut_ptr() as usize as u64,
            p2.as_mut_ptr() as usize as u64,
            0,
            1,
        ];

        f(
            hdr.as_ptr() as usize,
            (&dt as *const f32) as usize,
            (&scale as *const f32) as usize,
        );
        (p2[1], rd::<[u32; 6]>(b1))
    };

    for active in [0u8, 1] {
        let o = run(f_o, active);
        let n = run(f_n, active);
        crate::diff_eq!(o, n, "141f288b0 active={}", active);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f3a990(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_STATE: ([u64; 4], [u64; 2]) = ([0; 4], [0; 2]);

    unsafe extern "win64" fn mock_aae0(rcx: usize, rdx: usize) -> u8 {
        let a = rd::<[u64; 4]>(rcx);
        let b = rd::<[u64; 2]>(rdx);
        CALL_STATE = (a, b);
        ((a[0] ^ a[3] ^ b[0]) & 0xff) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f3_aae0, mock_aae0 as *const ())?;

    let g_f40 = exe::ptr::<u8>(0x1_4385_3f40) as usize;
    let saved_f40 = rd::<u8>(g_f40);

    let run = |f: F, enabled: u8| -> (u8, ([u64; 4], [u64; 2])) {
        CALL_STATE = ([0; 4], [0; 2]);
        wr::<u8>(g_f40, enabled);

        let mut arr0 = [0u8; 0x80];
        let a0p = arr0.as_mut_ptr() as usize;
        wr::<u64>(a0p + 2 * 0x20 + 0x10, 0x1122_3344);
        wr::<u64>(a0p + 2 * 0x20 + 0x18, 0x5566_7788);

        let mut arr1 = [0x99u8; 0x20];
        let a1p = arr1.as_mut_ptr() as usize;

        let p1: [u64; 4] = [a0p as u64, a1p as u64, 0, 2];
        let p2: [u64; 3] = [0xaaaa_0001, 0xbbbb_0002, 0xcccc_0003];

        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (rd::<u8>(a1p + 2 * 8 + 1), CALL_STATE)
    };

    for en in [0u8, 1] {
        let o = run(f_o, en);
        let n = run(f_n, en);
        wr::<u8>(g_f40, saved_f40);
        crate::diff_eq!(o, n, "141f3a990 en={}", en);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f79660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VISITED: [u64; 8] = [0; 8];
    static mut VISIT_CNT: usize = 0;

    unsafe extern "win64" fn mock_b1c0(ctx: u64, slot: usize, val: u64) {
        let idx = VISIT_CNT;
        if idx < 8 {
            VISITED[idx] = ctx ^ rd::<u64>(slot) ^ (val << 8);
            VISIT_CNT = idx + 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_b1c0, mock_b1c0 as *const ())?;

    let g_300 = exe::ptr::<u64>(0x1_43a0_8300) as usize;
    let saved_300 = rd::<u64>(g_300);
    wr::<u64>(g_300, 0x7000);

    let run = |f: F| -> (u8, usize, [u64; 8]) {
        VISITED = [0; 8];
        VISIT_CNT = 0;

        let tbl1: [u64; 8] = [
            0xffff, 10, // empty sentinel
            0xdead, 20, // tombstone
            0x0101, 30, // valid
            0xffff, 40, // empty sentinel
        ];
        let tbl2: [u64; 8] = [
            0x0202, 50, // valid
            0xeeee, 60, // empty sentinel
            0xbeef, 70, // tombstone
            0x0303, 80, // valid
        ];

        let mut p1 = [0u8; 0x120];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x28, 0xdead);
        wr::<u64>(p1p + 0x30, 1);
        wr::<u64>(p1p + 0x40, 4);
        wr::<u64>(p1p + 0x48, 0xffff);
        wr::<usize>(p1p + 0x58, tbl1.as_ptr() as usize);

        wr::<u64>(p1p + 0x88, 0xbeef);
        wr::<u64>(p1p + 0x90, 1);
        wr::<u64>(p1p + 0xa0, 4);
        wr::<u64>(p1p + 0xa8, 0xeeee);
        wr::<usize>(p1p + 0xb8, tbl2.as_ptr() as usize);

        f(p1p);
        (rd::<u8>(p1p + 0x110), VISIT_CNT, VISITED)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u64>(g_300, saved_300);
    crate::diff_eq!(o, n, "141f79660");
    Ok(1)
}

pub unsafe fn diff_unknown_141f91e90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MAT_OUT: [u32; 12] = [0; 12];
    static mut SUB_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_b9a0(_p: usize) {
        SUB_CALLS += 10;
    }
    unsafe extern "win64" fn mock_3520(rbx: usize, mat: usize) {
        SUB_CALLS += 1;
        MAT_OUT = rd::<[u32; 12]>(mat);
        wr::<u8>(rbx + 0x78, 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4110_b9a0, mock_b9a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f9_3520, mock_3520 as *const ())?;

    let run = |f: F, active: u8, flag: u32| -> (u8, u32, [u32; 12]) {
        MAT_OUT = [0; 12];
        SUB_CALLS = 0;

        let flags: [u32; 2] = [0, flag];
        let mut xforms = [0.0f32; 16];
        // index 1 at +8 floats (32 bytes): quaternion + translation
        xforms[8] = 0.0;
        xforms[9] = 0.70710677;
        xforms[10] = 0.0;
        xforms[11] = 0.70710677;
        xforms[12] = 3.5;
        xforms[13] = -2.0;
        xforms[14] = 7.25;
        xforms[15] = 99.0;

        let mut objs = [0u8; 0x100];
        let obp = objs.as_mut_ptr() as usize;
        wr::<u8>(obp + 0x80 + 0x78, active);

        let p1: [u64; 5] = [
            flags.as_ptr() as usize as u64,
            xforms.as_ptr() as usize as u64,
            obp as u64,
            0,
            1,
        ];

        f(p1.as_ptr() as usize);
        (rd::<u8>(obp + 0x80 + 0x78), SUB_CALLS, MAT_OUT)
    };

    for (active, flag) in [(0u8, 0u32), (0, 1), (1, 1), (1, 0)] {
        let o = run(f_o, active, flag);
        let n = run(f_n, active, flag);
        crate::diff_eq!(o, n, "141f91e90 active={} flag={}", active, flag);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141f93520(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut REF_OBJ_PTR: usize = 0;
    static mut CALL_LOG: (u64, u32, u64, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_a880(
        out_30: usize,
        p2: usize,
        v1c: u64,
        p24: usize,
        p2c: usize,
        v68: u32,
    ) {
        let tag = rd::<u64>(p2) ^ v1c ^ rd::<u64>(p24) ^ rd::<u64>(p2c);
        wr::<u64>(out_30, tag);
        CALL_LOG.0 = tag;
        CALL_LOG.1 = v68;
    }
    unsafe extern "win64" fn mock_a3b0(out_70: usize, _p1: usize, in_30: usize) {
        wr::<u64>(out_70, rd::<u64>(in_30).wrapping_add(0x10));
        wr::<usize>(out_70 + 0x48, REF_OBJ_PTR);
    }
    unsafe extern "win64" fn mock_b810(dst_70: usize, in_70: usize) {
        wr::<u64>(dst_70, rd::<u64>(in_70));
        CALL_LOG.2 = rd::<u64>(in_70);
    }
    unsafe extern "win64" fn mock_d6f0(owner: u64, _obj: usize) {
        CALL_LOG.3 = owner as u32;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_a880, mock_a880 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4204_a3b0, mock_a3b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4110_b810, mock_b810 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let run = |f: F, init_ref: u32| -> (u8, u64, u32, (u64, u32, u64, u32)) {
        CALL_LOG = (0, 0, 0, 0);
        let mut ref_obj = [0u8; 0x40];
        let rop = ref_obj.as_mut_ptr() as usize;
        wr::<u32>(rop + 0x24, init_ref);
        wr::<u64>(rop + 0x28, 0x7788);
        REF_OBJ_PTR = if init_ref == 0 { 0 } else { rop };

        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x1c, 0x1111);
        wr::<u64>(p1p + 0x24, 0x2222);
        wr::<u64>(p1p + 0x2c, 0x3333);
        wr::<u32>(p1p + 0x68, 9);

        let p2: u64 = 0x4444;
        f(p1p, (&p2 as *const u64) as usize);
        (
            rd::<u8>(p1p + 0x78),
            rd::<u64>(p1p + 0x70),
            rd::<u32>(rop + 0x24),
            CALL_LOG,
        )
    };

    for rc in [1u32, 2, 0] {
        let o = run(f_o, rc);
        let n = run(f_n, rc);
        crate::diff_eq!(o, n, "141f93520 rc={}", rc);
    }
    Ok(3)
}

pub unsafe fn diff_coregame_141fc89c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: [u64; 6] = [0; 6];
    static mut MGR_OBJ: [u64; 8] = [0; 8];
    static mut MGR_PTR: usize = 0;

    unsafe extern "win64" fn mock_4ee0(rcx: usize, rdx: u64) {
        CALLS[0] = (rcx as u64).wrapping_sub(CALLS[5]) ^ rdx;
    }
    unsafe extern "win64" fn mock_0810(_rcx: usize, key_ptr: usize) -> usize {
        CALLS[1] = rd::<u32>(key_ptr) as u64;
        core::ptr::addr_of!(MGR_PTR) as usize
    }
    unsafe extern "win64" fn mock_bc70(_rcx: usize, mgr: u64, r8: u64) {
        CALLS[2] = mgr ^ r8;
    }
    unsafe extern "win64" fn mock_0100(_rcx: usize, _rdx: usize, r8: u64) {
        CALLS[3] = r8;
    }
    unsafe extern "win64" fn mock_7e80(_rcx: usize, rdx: usize) {
        CALLS[4] = rd::<u64>(rdx);
    }
    unsafe extern "win64" fn mock_ccf0(rcx: u64, _rdx: usize) {
        CALLS[5] = rcx;
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e0_4ee0, mock_4ee0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41e0_bc70, mock_bc70 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41e1_0100, mock_0100 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41fc_7e80, mock_7e80 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e0_ccf0, mock_ccf0 as *const ())?;

    let g_e6c = exe::ptr::<u16>(0x1_43c8_ce6c) as usize;
    let saved_e6c = rd::<u16>(g_e6c);
    wr::<u16>(g_e6c, 1);

    let run = |f: F, id: i32| -> [u64; 6] {
        MGR_OBJ[5] = 0x9988_7766;
        MGR_PTR = core::ptr::addr_of!(MGR_OBJ) as usize;

        let mut p1 = [0u8; 0x260];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x18, id);

        let tbl: [u64; 4] = [0x1111, 0x2222_3333, 0x4444, 0x5555];
        let mut ctx = vec![0u8; 0x58100];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x580e0, tbl.as_ptr() as usize);

        CALLS = [0, 0, 0, 0, 0, p1p as u64];
        let p2: usize = cp;
        f(p1p, (&p2 as *const usize) as usize);
        CALLS
    };

    for id in [-1i32, 7] {
        let o = run(f_o, id);
        let n = run(f_n, id);
        wr::<u16>(g_e6c, saved_e6c);
        crate::diff_eq!(o, n, "141fc89c0 id={}", id);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141fd6020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut STATE_40: [u64; 5] = [0; 5];
    static mut OUT_1: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);
    static mut OUT_2: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);

    unsafe extern "win64" fn mock_0720(_p2: usize, out: usize) {
        wr::<[u64; 5]>(out, STATE_40);
    }
    unsafe extern "win64" fn mock_3170(a1: usize, a2: usize, xform: usize) {
        OUT_1 = (
            rd::<[u64; 5]>(a1),
            rd::<[u64; 3]>(a2),
            rd::<[u32; 8]>(xform),
        );
    }
    unsafe extern "win64" fn mock_2e20(a1: usize, a2: usize, xform: usize) {
        OUT_2 = (
            rd::<[u64; 5]>(a1),
            rd::<[u64; 3]>(a2),
            rd::<[u32; 8]>(xform),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4117_0720, mock_0720 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e2_3170, mock_3170 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41e2_2e20, mock_2e20 as *const ())?;

    let run = |f: F, handle: u64, key_u16: u16| -> (([u64; 5], [u64; 3], [u32; 8]), ([u64; 5], [u64; 3], [u32; 8])) {
        OUT_1 = ([0; 5], [0; 3], [0; 8]);
        OUT_2 = ([0; 5], [0; 3], [0; 8]);

        let handles_arr: [u64; 2] = [0, handle];
        let slots_18: [u64; 2] = [0, 0x0000_0002_0000_0001];
        let slots_30: [u64; 2] = [0, key_u16 as u64];
        let sorted_u16: [u16; 3] = [10, 20, 30];

        let mut rdx_obj = [0u8; 0x40];
        let rop = rdx_obj.as_mut_ptr() as usize;
        wr::<usize>(rop + 0x18, slots_18.as_ptr() as usize);
        wr::<u32>(rop + 0x20, 2);
        wr::<usize>(rop + 0x30, slots_30.as_ptr() as usize);

        let p2: [usize; 3] = [rop, 3, sorted_u16.as_ptr() as usize];

        let b50: [f32; 8] = [0.0, 0.0, 0.70710677, 0.70710677, 1.0, 2.0, 3.0, 0.0];
        let b58: [f32; 8] = [0.70710677, 0.0, 0.0, 0.70710677, 4.0, 5.0, 6.0, 0.0];
        let mut rdx_20 = [0u8; 0x60];
        let r20p = rdx_20.as_mut_ptr() as usize;
        wr::<usize>(r20p + 0x50, b50.as_ptr() as usize);
        wr::<usize>(r20p + 0x58, b58.as_ptr() as usize);

        let mut r8_elem = [0u8; 0xb0];
        let r8p = r8_elem.as_mut_ptr() as usize;
        wr::<u32>(r8p + 0x28, 77);
        wr::<[f32; 8]>(
            r8p + 0x80,
            [0.0, 0.70710677, 0.0, 0.70710677, -1.0, 0.5, 2.5, 0.0],
        );

        let mut r10_48 = [0u8; 0x50];
        let r10p = r10_48.as_mut_ptr() as usize;
        wr::<usize>(r10p + 0x40, r8p);
        wr::<u32>(r10p + 0x48, 1);

        let bone_map: [i32; 1] = [0];
        let mut r14_obj = [0u8; 0xe0];
        let r14p = r14_obj.as_mut_ptr() as usize;
        wr::<usize>(r14p + 0x20, r20p);
        wr::<usize>(r14p + 0x48, r10p);
        wr::<usize>(r14p + 0xd0, bone_map.as_ptr() as usize);

        let r14_table: [usize; 1] = [r14p];
        let a_12: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 0.0];
        let a_13: [f32; 8] = [0.0, 0.0, 0.0, 1.0, -5.0, -10.0, 15.0, 0.0];

        STATE_40 = [
            r14_table.as_ptr() as usize as u64,
            a_12.as_ptr() as usize as u64,
            a_13.as_ptr() as usize as u64,
            0,
            0,
        ];

        let mut p1_entries = [0u8; 0x60];
        let pep = p1_entries.as_mut_ptr() as usize;
        wr::<u32>(pep + 0x30, 77);
        wr::<[f32; 8]>(
            pep + 0x30 + 0x10,
            [0.0, 0.0, 0.0, 1.0, 0.25, 0.5, 0.75, 0.0],
        );

        let p1: [u64; 8] = [
            pep as u64,
            handles_arr.as_ptr() as usize as u64,
            0x101,
            0x102,
            0x201,
            0x202,
            0x301,
            1,
        ];
        let p3: [u64; 3] = [0xaa, 0xbb, 0xcc];
        let p4: [u64; 3] = [0xdd, 0xee, 0xff];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
        );
        let mut o1 = OUT_1;
        let mut o2 = OUT_2;
        o1.0[0] = 0;
        o2.0[0] = 0;
        (o1, o2)
    };

    for (h, k) in [
        (u64::MAX, 20u16),
        (0x0000_0002_0000_0001, 5),
        (0x0000_0002_0000_0001, 20),
    ] {
        let o = run(f_o, h, k);
        let n = run(f_n, h, k);
        crate::diff_eq!(o, n, "141fd6020 h={:#x} k={}", h, k);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14202b8d0(orig: usize, new: usize) -> Result<u32, String> {
    static mut TARGET_BASE: usize = 0;
    static mut ERASE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_9b80(_p1: usize, out: usize, r8: usize) {
        let idx = rd::<usize>(r8);
        let target = TARGET_BASE.wrapping_add(idx * 0x80);
        wr::<usize>(out, 0);
        wr::<usize>(out + 0x08, target.wrapping_sub(0x10));
    }
    unsafe extern "win64" fn mock_90b0(_tbl: usize, _it: usize) {
        ERASE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_411b_9b80, mock_9b80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411b_90b0, mock_90b0 as *const ())?;

    let run = |fn_addr: usize, kind: u8, dt: f32| -> ([u32; 4], u32, u32, u32) {
        ERASE_COUNT = 0;
        let mut targets = [0u8; 0x100];
        let tp = targets.as_mut_ptr() as usize;
        TARGET_BASE = tp;

        wr::<u8>(tp + 0x40, kind);
        wr::<u8>(tp + 0x80 + 0x40, 0x14);
        wr::<u32>(tp + 0x80 + 0x08, 0x7777);

        let mut ctrl = [-128i8; 32];
        ctrl[1] = 0;
        ctrl[2] = -1;

        let mut elems = [0u8; 0xc0 * 2];
        let ep = elems.as_mut_ptr() as usize;
        let e1 = ep + 0xc0;
        wr::<usize>(e1, 0); // target index 0
        wr::<f32>(e1 + 0x10, 1.0);
        wr::<f32>(e1 + 0x14, 2.0);
        wr::<f32>(e1 + 0x18, 3.0);
        wr::<f32>(e1 + 0x1c, 4.0);
        wr::<u8>(e1 + 0x50, kind);
        wr::<f32>(e1 + 0x60, 5.0);
        wr::<f32>(e1 + 0x64, 6.0);
        wr::<f32>(e1 + 0x68, 7.0);
        wr::<f32>(e1 + 0x6c, 8.0);
        wr::<u8>(e1 + 0xa0, kind);
        wr::<f32>(e1 + 0xb0, 2.0);
        wr::<f32>(e1 + 0xb4, 0.5);

        let arr_60: [usize; 1] = [1]; // target index 1

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x30, ctrl.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x38, ep);
        wr::<usize>(p1p + 0x48, 2);
        wr::<usize>(p1p + 0x60, arr_60.as_ptr() as usize);
        wr::<u32>(p1p + 0x68, 1);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, dt.to_bits() as u128, 0, 0],
        );

        (
            rd::<[u32; 4]>(tp),
            rd::<u32>(e1 + 0xb4),
            ERASE_COUNT,
            rd::<u32>(tp + 0x80 + 0x08),
        )
    };

    for (kind, dt) in [(2u8, 0.5f32), (4, 0.5), (6, 0.5), (8, 2.0)] {
        let o = run(orig, kind, dt);
        let n = run(new, kind, dt);
        crate::diff_eq!(o, n, "14202b8d0 kind={} dt={}", kind, dt);
    }
    Ok(4)
}
