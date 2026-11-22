//! Decompiled `rend` hot functions shard Z4A (`0x141e22e20..0x141ec85a0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vpermilps(v: [f32; 4], imm: u8) -> [f32; 4] {
    [
        v[(imm & 3) as usize],
        v[((imm >> 2) & 3) as usize],
        v[((imm >> 4) & 3) as usize],
        v[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn vmulps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

#[inline(always)]
fn vaddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[inline(always)]
fn vsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

#[inline(always)]
fn vhsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - a[1], a[2] - a[3], b[0] - b[1], b[2] - b[3]]
}

#[inline(always)]
fn vhaddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + a[1], a[2] + a[3], b[0] + b[1], b[2] + b[3]]
}

#[inline(always)]
fn vshufps(a: [f32; 4], b: [f32; 4], imm: u8) -> [f32; 4] {
    [
        a[(imm & 3) as usize],
        a[((imm >> 2) & 3) as usize],
        b[((imm >> 4) & 3) as usize],
        b[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn vblendps(a: [f32; 4], b: [f32; 4], mask: u8) -> [f32; 4] {
    [
        if (mask & 1) != 0 { b[0] } else { a[0] },
        if (mask & 2) != 0 { b[1] } else { a[1] },
        if (mask & 4) != 0 { b[2] } else { a[2] },
        if (mask & 8) != 0 { b[3] } else { a[3] },
    ]
}

#[inline(always)]
fn vaddsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] + b[1], a[2] - b[2], a[3] + b[3]]
}

#[inline(always)]
fn vxorps_u32(a: [f32; 4], mask: [u32; 4]) -> [f32; 4] {
    [
        f32::from_bits(a[0].to_bits() ^ mask[0]),
        f32::from_bits(a[1].to_bits() ^ mask[1]),
        f32::from_bits(a[2].to_bits() ^ mask[2]),
        f32::from_bits(a[3].to_bits() ^ mask[3]),
    ]
}

#[inline(always)]
unsafe fn resolve_parent_transform(
    ptr0: usize,
    idx: usize,
    scene: usize,
    slot_va: usize,
) -> Option<[f32; 8]> {
    let raw = rd::<u64>(ptr0.wrapping_add(idx.wrapping_mul(8)));
    let h = if (raw as u32) != u32::MAX {
        raw
    } else {
        u64::MAX
    };
    let resolved_lo: u32 = if (h as u32) == u32::MAX && ((h >> 32) as u32) == u32::MAX {
        u32::MAX
    } else {
        let lo = (h as u32) as u64;
        let cap = rd::<u32>(scene + 0x20) as u64;
        if lo < cap {
            let arr_18 = rd::<usize>(scene + 0x18);
            let entry = rd::<u64>(arr_18.wrapping_add((lo as usize) * 8));
            if (entry as u32) == (lo as u32) && ((entry >> 32) as u32) == ((h >> 32) as u32) {
                h as u32
            } else {
                u32::MAX
            }
        } else {
            u32::MAX
        }
    };

    if resolved_lo == u32::MAX {
        return None;
    }

    let arr_30 = rd::<usize>(scene + 0x30);
    let rbx = rd::<u64>(arr_30.wrapping_add((resolved_lo as usize) * 8));
    if (rbx as u16) == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        unreachable!();
    }

    let r8d = rd::<u16>(exe::ptr::<u16>(slot_va as u64) as usize) as u32;
    let r11 = (rbx as u16) as usize;
    let arg_rcx = rd::<u64>(scene.wrapping_add(r11 * 32 + 0x18098));
    let arg_rdx = rd::<u32>(scene.wrapping_add(r11 * 32 + 0x180ac));
    let eax = exe::call_va!(0x1_4248_9ee0, (arg_rcx, arg_rdx, r8d) -> i32);
    let rcx_idx = eax as isize as usize;
    let ptr_40 = rd::<usize>(scene.wrapping_add((r11 + 0xc03) * 32 + 0x40));
    let edx_off = rd::<u32>(ptr_40.wrapping_add(rcx_idx.wrapping_mul(4))) as usize;
    let base_88 = rd::<usize>(scene.wrapping_add(r11 * 8 + 0x88));
    let row_off = ((rbx >> 32) as usize).wrapping_shl(5);
    Some(rd::<[f32; 8]>(base_88.wrapping_add(edx_off).wrapping_add(row_off)))
}

#[inline(always)]
unsafe fn compose_parent_child(par: [f32; 8], child: [f32; 8]) -> [f32; 8] {
    let xmm10 = [par[0], par[1], par[2], par[3]];
    let par_p = [par[4], par[5], par[6], par[7]];
    let xmm4 = [child[0], child[1], child[2], child[3]];
    let xmm9 = [child[4], child[5], child[6], child[7]];

    let xmm8 = vpermilps(xmm10, 0xd2);
    let xmm7 = vpermilps(xmm10, 0xc9);
    let xmm0 = vpermilps(xmm9, 0xd2);
    let xmm3 = vmulps(xmm0, xmm7);
    let xmm1 = vpermilps(xmm9, 0xc9);
    let xmm2 = vmulps(xmm1, xmm8);
    let xmm3 = vsubps(xmm3, xmm2);
    let scale = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7ab0) as usize);
    let xmm6 = vmulps(xmm3, scale);

    let xmm3_q = vpermilps(xmm4, 0x11);
    let xmm1_q = vmulps(xmm10, xmm3_q);
    let xmm2_q = vpermilps(xmm4, 0xbb);
    let xmm5_q = vpermilps(xmm10, 0x1b);
    let xmm0_q = vmulps(xmm2_q, xmm5_q);
    let xmm4_hsub = vhsubps(xmm1_q, xmm0_q);
    let xmm2_m = vmulps(xmm10, xmm2_q);
    let xmm0_m = vmulps(xmm3_q, xmm5_q);
    let xmm3_hadd = vhaddps(xmm2_m, xmm0_m);
    let xmm0_shuf = vshufps(xmm4_hsub, xmm3_hadd, 0xb1);
    let xmm1_blend = vblendps(xmm3_hadd, xmm4_hsub, 0x0c);
    let xmm1_addsub = vaddsubps(xmm1_blend, xmm0_shuf);
    let out_q = vpermilps(xmm1_addsub, 0x9c);

    let xmm0_p = vpermilps(xmm6, 0xd2);
    let xmm3_p = vmulps(xmm0_p, xmm7);
    let xmm1_p = vpermilps(xmm6, 0xc9);
    let xmm0_w = vpermilps(xmm10, 0xff);
    let xmm2_p = vmulps(xmm1_p, xmm8);
    let xmm1_w = vmulps(xmm0_w, xmm6);
    let xmm4_p = vsubps(xmm3_p, xmm2_p);
    let xmm2_add = vaddps(xmm1_w, xmm9);
    let xmm3_add = vaddps(xmm4_p, xmm2_add);
    let out_p = vaddps(xmm3_add, par_p);

    [
        out_q[0], out_q[1], out_q[2], out_q[3], out_p[0], out_p[1], out_p[2], out_p[3],
    ]
}

/// `unknown_141e22e20`
/// Original: 0x141e22e20 (831 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e22e20(param_1: usize, param_2: usize, param_3: usize) {
    let idx = rd::<usize>(param_1 + 0x20);
    let off = idx.wrapping_shl(5);
    let arr_in = rd::<usize>(param_1 + 0x08);
    let in_tf = rd::<[f32; 8]>(param_3);
    let cur_tf = rd::<[f32; 8]>(arr_in.wrapping_add(off));
    if (0..4).all(|i| (in_tf[i] == cur_tf[i]) && (cur_tf[i + 4] == in_tf[i + 4])) {
        return;
    }

    let ptr0 = rd::<usize>(param_1);
    let scene = rd::<usize>(param_2);
    let in_bits = rd::<[u32; 8]>(param_3);
    wr::<[u32; 8]>(arr_in.wrapping_add(off), in_bits);

    let parent_tf = resolve_parent_transform(ptr0, idx, scene, 0x1_43c8_f6d8);
    let arr_out = rd::<usize>(param_1 + 0x10);
    if let Some(par) = parent_tf {
        let fresh_in = rd::<[f32; 8]>(param_3);
        let out_tf = compose_parent_child(par, fresh_in);
        wr::<[f32; 8]>(arr_out.wrapping_add(off), out_tf);
    } else {
        let fresh_bits = rd::<[u32; 8]>(param_3);
        wr::<[u32; 8]>(arr_out.wrapping_add(off), fresh_bits);
    }

    let rax = rd::<usize>(param_1 + 0x18);
    let idx_after = rd::<usize>(param_1 + 0x20);
    let arg = rd::<usize>(rax.wrapping_add(idx_after.wrapping_mul(8)).wrapping_add(0x10));
    exe::call_va!(0x1_41e2_2790, (arg));
}

/// `unknown_141e23170`
/// Original: 0x141e23170 (831 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e23170(param_1: usize, param_2: usize, param_3: usize) {
    let idx = rd::<usize>(param_1 + 0x20);
    let off = idx.wrapping_shl(5);
    let arr_in = rd::<usize>(param_1 + 0x08);
    let in_tf = rd::<[f32; 8]>(param_3);
    let cur_tf = rd::<[f32; 8]>(arr_in.wrapping_add(off));
    if (0..4).all(|i| (in_tf[i] == cur_tf[i]) && (cur_tf[i + 4] == in_tf[i + 4])) {
        return;
    }

    let ptr0 = rd::<usize>(param_1);
    let scene = rd::<usize>(param_2);
    let in_bits = rd::<[u32; 8]>(param_3);
    wr::<[u32; 8]>(arr_in.wrapping_add(off), in_bits);

    let parent_tf = resolve_parent_transform(ptr0, idx, scene, 0x1_43c8_d120);
    let arr_out = rd::<usize>(param_1 + 0x10);
    if let Some(par) = parent_tf {
        let fresh_in = rd::<[f32; 8]>(param_3);
        let out_tf = compose_parent_child(par, fresh_in);
        wr::<[f32; 8]>(arr_out.wrapping_add(off), out_tf);
    } else {
        let fresh_bits = rd::<[u32; 8]>(param_3);
        wr::<[u32; 8]>(arr_out.wrapping_add(off), fresh_bits);
    }

    let rax = rd::<usize>(param_1 + 0x18);
    let idx_after = rd::<usize>(param_1 + 0x20);
    let arg = rd::<usize>(rax.wrapping_add(idx_after.wrapping_mul(8)).wrapping_add(0x10));
    exe::call_va!(0x1_41e2_2790, (arg));
}

/// `unknown_141e234c0`
/// Original: 0x141e234c0 (971 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e234c0(param_1: usize, param_2: usize, param_3: usize) {
    let idx = rd::<usize>(param_1 + 0x20);
    let off = idx.wrapping_shl(5);
    let arr_in = rd::<usize>(param_1 + 0x10);
    let in_tf = rd::<[f32; 8]>(param_3);
    let cur_tf = rd::<[f32; 8]>(arr_in.wrapping_add(off));
    if (0..4).all(|i| (in_tf[i] == cur_tf[i]) && (cur_tf[i + 4] == in_tf[i + 4])) {
        return;
    }

    let ptr0 = rd::<usize>(param_1);
    let scene = rd::<usize>(param_2);
    let in_bits = rd::<[u32; 8]>(param_3);
    wr::<[u32; 8]>(arr_in.wrapping_add(off), in_bits);

    let parent_tf = resolve_parent_transform(ptr0, idx, scene, 0x1_43c8_d120);
    let arr_out = rd::<usize>(param_1 + 0x08);
    if let Some(par) = parent_tf {
        let fresh_in = rd::<[f32; 8]>(param_3);
        let xmm4 = [fresh_in[0], fresh_in[1], fresh_in[2], fresh_in[3]];
        let xmm7 = [fresh_in[4], fresh_in[5], fresh_in[6], fresh_in[7]];
        let mask_q = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7b00) as usize);
        let mask_p = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_8760) as usize);
        let scale = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7ab0) as usize);

        let xmm9 = vxorps_u32([par[0], par[1], par[2], par[3]], mask_q);
        let xmm13 = vxorps_u32([par[4], par[5], par[6], par[7]], mask_p);
        let xmm12 = vpermilps(xmm9, 0xc9);
        let xmm11 = vpermilps(xmm9, 0xd2);
        let xmm5 = vpermilps(xmm9, 0x1b);
        let xmm8 = vpermilps(xmm9, 0xff);

        let xmm0_a = vpermilps(xmm13, 0xc9);
        let xmm3_a = vmulps(xmm0_a, xmm11);
        let xmm1_a = vpermilps(xmm13, 0xd2);
        let xmm2_a = vmulps(xmm1_a, xmm12);
        let xmm0_a = vsubps(xmm2_a, xmm3_a);
        let xmm10 = vmulps(xmm0_a, scale);

        let xmm0_b = vpermilps(xmm7, 0xc9);
        let xmm3_b = vmulps(xmm0_b, xmm11);
        let xmm1_b = vpermilps(xmm7, 0xd2);
        let xmm2_b = vmulps(xmm1_b, xmm12);
        let xmm0_b = vsubps(xmm2_b, xmm3_b);
        let xmm6 = vmulps(xmm0_b, scale);

        let xmm3_q = vpermilps(xmm4, 0x11);
        let xmm1_q = vmulps(xmm3_q, xmm9);
        let xmm2_q = vpermilps(xmm4, 0xbb);
        let xmm0_q = vmulps(xmm2_q, xmm5);
        let xmm4_hsub = vhsubps(xmm1_q, xmm0_q);
        let xmm0_m = vmulps(xmm3_q, xmm5);
        let xmm2_m = vmulps(xmm2_q, xmm9);
        let xmm3_hadd = vhaddps(xmm2_m, xmm0_m);
        let xmm0_shuf = vshufps(xmm4_hsub, xmm3_hadd, 0xb1);
        let xmm1_blend = vblendps(xmm3_hadd, xmm4_hsub, 0x0c);
        let xmm1_addsub = vaddsubps(xmm1_blend, xmm0_shuf);
        let out_q = vpermilps(xmm1_addsub, 0x9c);

        let xmm1_g = vpermilps(xmm6, 0xd2);
        let xmm0_g = vpermilps(xmm6, 0xc9);
        let xmm3_g = vmulps(xmm0_g, xmm11);
        let xmm0_gw = vmulps(xmm6, xmm8);
        let xmm2_g = vmulps(xmm1_g, xmm12);
        let xmm4_g = vsubps(xmm2_g, xmm3_g);
        let xmm1_ga = vaddps(xmm0_gw, xmm7);
        let xmm5_g = vaddps(xmm4_g, xmm1_ga);

        let xmm2_t = vpermilps(xmm10, 0xc9);
        let xmm3_t = vmulps(xmm2_t, xmm11);
        let xmm0_t = vpermilps(xmm10, 0xd2);
        let xmm1_t = vmulps(xmm0_t, xmm12);
        let xmm2_tw = vmulps(xmm8, xmm10);
        let xmm0_ta = vaddps(xmm2_tw, xmm13);
        let xmm4_t = vsubps(xmm1_t, xmm3_t);
        let xmm1_tb = vaddps(xmm4_t, xmm0_ta);
        let out_p = vaddps(xmm5_g, xmm1_tb);

        let out_tf = [
            out_q[0], out_q[1], out_q[2], out_q[3], out_p[0], out_p[1], out_p[2], out_p[3],
        ];
        wr::<[f32; 8]>(arr_out.wrapping_add(off), out_tf);
    } else {
        let fresh_bits = rd::<[u32; 8]>(param_3);
        wr::<[u32; 8]>(arr_out.wrapping_add(off), fresh_bits);
    }

    let rax = rd::<usize>(param_1 + 0x18);
    let idx_after = rd::<usize>(param_1 + 0x20);
    let arg = rd::<usize>(rax.wrapping_add(idx_after.wrapping_mul(8)).wrapping_add(0x10));
    exe::call_va!(0x1_41e2_2790, (arg));
}

/// `unknown_141e99650`
/// Original: 0x141e99650 (568 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e99650(param_1: usize, param_2: usize) {
    let r14 = rd::<usize>(param_1);
    let mut arg_pair = rd::<[u64; 2]>(param_1 + 0x08);
    let r11 = r14.wrapping_add(0x40);
    let mut span_out = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            span_out.as_mut_ptr() as usize,
            r11,
            arg_pair.as_mut_ptr() as usize
        )
    );

    let mut iter = [0u64; 9];
    let iter_base = iter.as_mut_ptr() as usize;
    wr::<usize>(iter_base, r11);
    wr::<u64>(iter_base + 0x08, span_out[0]);
    wr::<u64>(iter_base + 0x10, span_out[1]);
    wr::<u64>(iter_base + 0x18, span_out[2]);
    wr::<u64>(iter_base + 0x28, 0);
    wr::<u64>(iter_base + 0x30, 0);

    if span_out[1] != span_out[2] {
        let idx = rd::<u16>(span_out[1] as usize) as usize;
        let ptr_48 = rd::<u64>(r11.wrapping_add(idx * 8 + 0x48));
        wr::<u64>(iter_base + 0x20, ptr_48);
        let len_40 = if ptr_48 != 0 {
            rd::<u32>(r11.wrapping_add(idx * 4 + 0x10048)) as u64
        } else {
            0
        };
        wr::<u64>(iter_base + 0x28, len_40);
    }

    exe::call_va!(0x1_4100_8650, (iter_base));

    let mut r8 = rd::<u64>(iter_base + 0x30);
    let mut r9 = rd::<u64>(iter_base + 0x28);
    while r8 < r9 {
        while r8 < r9 {
            let src = rd::<usize>(iter_base + 0x38);
            let dst = rd::<usize>(iter_base + 0x40);
            let b = rd::<u8>(src.wrapping_add(r8 as usize));
            wr::<u8>(dst.wrapping_add(r8 as usize), b);
            wr::<u8>(src.wrapping_add(r8 as usize), 0);
            r8 = r8.wrapping_add(1);
            wr::<u64>(iter_base + 0x30, r8);
        }

        let mut cur_ptr = rd::<usize>(iter_base + 0x10).wrapping_add(2);
        let end_ptr = rd::<usize>(iter_base + 0x18);
        wr::<usize>(iter_base + 0x10, cur_ptr);
        if cur_ptr != end_ptr {
            let rdx_base = rd::<usize>(iter_base + 0x08);
            loop {
                let idx = rd::<u16>(cur_ptr) as usize;
                if rd::<u64>(rdx_base.wrapping_add(idx * 8 + 0x48)) != 0
                    && rd::<u32>(rdx_base.wrapping_add(idx * 4 + 0x10048)) != 0
                {
                    break;
                }
                cur_ptr = cur_ptr.wrapping_add(2);
                wr::<usize>(iter_base + 0x10, cur_ptr);
                if cur_ptr == end_ptr {
                    break;
                }
            }
            if cur_ptr != end_ptr {
                let idx = rd::<u16>(cur_ptr) as usize;
                let rcx_base = rd::<usize>(iter_base);
                let ptr_48 = rd::<u64>(rcx_base.wrapping_add(idx * 8 + 0x48));
                wr::<u64>(iter_base + 0x20, ptr_48);
                let len_40 = if ptr_48 != 0 {
                    rd::<u32>(rcx_base.wrapping_add(idx * 4 + 0x10048)) as u64
                } else {
                    0
                };
                wr::<u64>(iter_base + 0x28, len_40);
                wr::<u64>(iter_base + 0x30, 0);
            }
        }

        exe::call_va!(0x1_4100_8650, (iter_base));
        r8 = rd::<u64>(iter_base + 0x30);
        r9 = rd::<u64>(iter_base + 0x28);
    }

    let p2_0 = rd::<usize>(param_2);
    let p2_00 = rd::<usize>(p2_0);
    let mut cur_elem = rd::<usize>(p2_00 + 0x138);
    let elem_cnt = rd::<u32>(p2_00 + 0x140) as usize;
    let end_elem = cur_elem.wrapping_add(elem_cnt * 8);
    while cur_elem != end_elem {
        let obj = rd::<usize>(cur_elem);
        let raw_h = rd::<u64>(obj + 0x40);
        let idx32 = if raw_h == 0 { u32::MAX } else { raw_h as u32 };
        let arr_30 = rd::<usize>(r14 + 0x30);
        let rsi = rd::<u64>(arr_30.wrapping_add((idx32 as usize) * 8));
        if (rsi as u16) == 0xffff {
            exe::call_iat!(0x1_4264_67e8, ());
            unreachable!();
        }
        let r8d = rd::<u16>(exe::ptr::<u16>(0x1_43cd_08a4) as usize) as u32;
        let edi = (rsi as u16) as usize;
        let arg_rcx = rd::<u64>(r14.wrapping_add(edi * 32 + 0x18098));
        let arg_rdx = rd::<u32>(r14.wrapping_add(edi * 32 + 0x180ac));
        let eax = exe::call_va!(0x1_4248_9ee0, (arg_rcx, arg_rdx, r8d) -> i32);
        cur_elem = cur_elem.wrapping_add(8);
        let row = (rsi >> 32) as usize;
        let ptr_40 = rd::<usize>(r14.wrapping_add((edi + 0xc03) * 32 + 0x40));
        let edx_off = rd::<u32>(ptr_40.wrapping_add((eax as isize as usize).wrapping_mul(4))) as usize;
        let base_88 = rd::<usize>(r14.wrapping_add(edi * 8 + 0x88));
        wr::<u8>(base_88.wrapping_add(edx_off).wrapping_add(row), 1);
    }
}

/// `unknown_141ec0f90`
/// Original: 0x141ec0f90 (75 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ec0f90(param_1: usize) -> u8 {
    let prefix = exe::ptr::<u8>(0x1_428d_b250) as usize;
    let table = exe::ptr::<u8>(0x1_427e_54e0) as usize;
    let mut rem = 0x1fu32;
    let mut i = 0usize;
    loop {
        let b_in = rd::<u8>(param_1.wrapping_add(i)) as usize;
        let b_pre = rd::<u8>(prefix.wrapping_add(i)) as usize;
        i += 1;
        let c_in = rd::<u8>(table.wrapping_add(b_in));
        let c_pre = rd::<u8>(table.wrapping_add(b_pre));
        rem -= 1;
        if rem == 0 || c_in == 0 || c_in != c_pre {
            return u8::from(c_in == c_pre);
        }
    }
}

/// `unknown_141ec4a40`
/// Original: 0x141ec4a40 (61 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ec4a40(mut param_1: usize, mut param_2: u64) {
    while param_2 != 0 {
        exe::call_va!(0x1_401d_e830, (param_1.wrapping_add(0x20)));
        exe::call_va!(0x1_4037_e270, (param_1));
        param_1 = param_1.wrapping_add(0x30);
        param_2 = param_2.wrapping_sub(1);
    }
}

#[inline(always)]
unsafe fn c_strlen(ptr: usize) -> u64 {
    let mut len = 0u64;
    while rd::<u8>(ptr.wrapping_add(len as usize)) != 0 {
        len += 1;
    }
    len
}

#[inline(always)]
unsafe fn lua_gc_barrier_thread(rdi: usize) {
    let al = rd::<u8>(rdi + 0x01);
    if (al & 4) != 0 {
        let rcx = rd::<usize>(rdi + 0x18);
        wr::<u8>(rdi + 0x01, al & !4);
        wr::<u64>(rdi + 0x68, rd::<u64>(rcx + 0x30));
        wr::<usize>(rcx + 0x30, rdi);
    }
}

#[inline(always)]
unsafe fn lua_index2addr(rdi: usize, idx: i32) -> usize {
    if idx > 0 {
        let rcx = rd::<usize>(rdi + 0x10)
            .wrapping_add(((idx - 1) as isize as usize).wrapping_mul(0x18));
        if rcx < rd::<usize>(rdi + 0x08) {
            rcx
        } else {
            exe::ptr::<u8>(0x1_4298_68d0) as usize
        }
    } else if idx > -10000 {
        rd::<usize>(rdi + 0x08).wrapping_add((idx as isize as usize).wrapping_mul(0x18))
    } else {
        exe::call_va!(0x1_4211_6d40, (rdi, idx as u32) -> usize)
    }
}

#[inline(always)]
unsafe fn lua_abs_idx(rdi: usize, top: usize, idx: i32) -> u32 {
    if (idx as u32).wrapping_add(0x270f) <= 0x270f {
        let diff = (top.wrapping_sub(rd::<usize>(rdi + 0x10)) as i64) / 24;
        (diff as i32).wrapping_add(1).wrapping_add(idx) as u32
    } else {
        idx as u32
    }
}

/// `unknown_141ec85a0`
/// Original: 0x141ec85a0 (4773 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141ec85a0(
    param_1: usize,
    _param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> u8 {
    let rdi = rd::<usize>(param_1 + 0x08);
    let mut p6_slot = param_6;
    let p6_deref = rd::<usize>(p6_slot);
    let r10_off = rd::<u32>(p6_deref + 0x14).wrapping_add(rd::<u32>(p6_deref + 0x10)) as usize;
    let r10 = p6_deref.wrapping_add(r10_off);

    let mut str_1e8 = [0u64; 18];
    let s1e8 = str_1e8.as_mut_ptr() as usize;
    wr::<u64>(s1e8, 0x1000_0010);
    wr::<u8>(s1e8 + 0x08, 0);
    let mut local_70 = [0u64; 2];
    if r10 == 0 {
        wr::<u32>(s1e8 + 0x04, 0);
        wr::<u8>(s1e8 + 0x08, 0);
    } else {
        let len = c_strlen(r10);
        local_70 = [r10 as u64, len];
        exe::call_va!(
            0x1_4240_9b50,
            (s1e8, local_70.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    exe::call_va!(0x1_4211_9080, (rdi, 0u32, 0u32));
    let stack_diff = (rd::<usize>(rdi + 0x08).wrapping_sub(rd::<usize>(rdi + 0x10))) as i64;
    let r14d = (stack_diff / 24) as i32;
    exe::call_va!(0x1_4211_9080, (rdi, 0u32, 0u32));
    lua_gc_barrier_thread(rdi);

    let reg_slot = exe::call_va!(0x1_4211_6d40, (rdi, 0xffff_d8eeu32) -> usize);
    let top0 = rd::<usize>(rdi + 0x08);
    wr::<[u64; 3]>(top0, rd::<[u64; 3]>(reg_slot));
    let top1 = rd::<usize>(rdi + 0x08).wrapping_add(0x18);
    wr::<usize>(rdi + 0x08, top1);

    let mut tv_50 = [0u64; 3];
    let tv50_ptr = tv_50.as_mut_ptr() as usize;
    let s_idx = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_6980) as usize, 7u32) -> u64
    );
    wr::<u64>(tv50_ptr, s_idx);
    wr::<u32>(tv50_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            top1.wrapping_sub(0x30),
            tv50_ptr,
            rd::<usize>(rdi + 0x08).wrapping_sub(0x18)
        )
    );
    let top2 = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
    wr::<usize>(rdi + 0x08, top2);
    wr::<u8>(rd::<usize>(top2.wrapping_sub(0x18)) + 0x04, 1);
    exe::call_va!(0x1_4211_84a0, (rdi, 0xffff_fffeu32));

    let addr1 = lua_index2addr(rdi, r14d);
    wr::<u8>(rd::<usize>(addr1) + 0x05, 1);
    exe::call_va!(0x1_4211_8910, (rdi, r14d as u32));

    let addr2 = lua_index2addr(rdi, r14d);
    let s_env = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_69cc) as usize, 4u32) -> u64
    );
    wr::<u64>(tv50_ptr, s_env);
    wr::<u32>(tv50_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            addr2,
            tv50_ptr,
            rd::<usize>(rdi + 0x08).wrapping_sub(0x18)
        )
    );

    let top3 = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
    wr::<usize>(rdi + 0x08, top3);
    wr::<u64>(top3, rd::<u64>(param_5 + 0x10));
    wr::<u32>(top3 + 0x10, 2);
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));

    let addr3 = lua_index2addr(rdi, r14d);
    let s_self = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_537c) as usize, 4u32) -> u64
    );
    wr::<u64>(tv50_ptr, s_self);
    wr::<u32>(tv50_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            addr3,
            tv50_ptr,
            rd::<usize>(rdi + 0x08).wrapping_sub(0x18)
        )
    );
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));
    lua_gc_barrier_thread(rdi);

    let env_tbl = exe::call_va!(0x1_4211_6d40, (rdi, 0xffff_d8f0u32) -> usize);
    let s_envs = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_6838) as usize, 0x0cu32) -> u64
    );
    wr::<u64>(tv50_ptr, s_envs);
    wr::<u32>(tv50_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c940,
        (rdi, env_tbl, tv50_ptr, rd::<usize>(rdi + 0x08))
    );

    let top4 = rd::<usize>(rdi + 0x08).wrapping_add(0x18);
    wr::<usize>(rdi + 0x08, top4);
    wr::<u64>(top4, rd::<u64>(param_5 + 0x10));
    wr::<u32>(top4 + 0x10, 2);
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));
    exe::call_va!(0x1_4211_8910, (rdi, r14d as u32));

    let r8_top = rd::<usize>(rdi + 0x08);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            r8_top.wrapping_sub(0x48),
            r8_top.wrapping_sub(0x30),
            r8_top.wrapping_sub(0x18)
        )
    );
    let top5 = rd::<usize>(rdi + 0x08).wrapping_sub(0x48);
    wr::<usize>(rdi + 0x08, top5);
    wr::<u64>(top5, rd::<u64>(param_5 + 0x10));
    wr::<u32>(top5 + 0x10, 2);
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));

    let cls_tbl = exe::call_va!(0x1_4211_6d40, (rdi, 0xffff_d8f0u32) -> usize);
    let s_cls = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_68f8) as usize, 0x18u32) -> u64
    );
    wr::<u64>(tv50_ptr, s_cls);
    wr::<u32>(tv50_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            cls_tbl,
            tv50_ptr,
            rd::<usize>(rdi + 0x08).wrapping_sub(0x18)
        )
    );
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));

    let mut seq = rd::<u32>(param_1 + 0x240);
    if seq == 0 {
        seq = 1;
    }
    wr::<u32>(param_1 + 0x240, seq.wrapping_add(1));
    wr::<u32>(param_5 + 0x18, seq);
    exe::call_va!(0x1_41ee_2350, (rdi, r14d as u32, p6_slot));

    let r10_p5 = rd::<usize>(param_5);
    let p5_id = rd::<u64>(param_5 + 0x10);
    let rdx_78 = rd::<usize>(rdi + 0x78);
    let saved_18 = rd::<u64>(rdx_78 + 0x18);
    wr::<u64>(rdx_78 + 0x18, p5_id);

    let mut str_158 = [0u64; 34];
    let s158 = str_158.as_mut_ptr() as usize;
    wr::<u64>(s158, 0x2000_0020);
    wr::<u8>(s158 + 0x08, 0);

    let name_cstr = exe::call_va!(0x1_401d_f680, (s1e8) -> u64);
    let code_len = rd::<u32>(r10_p5 + 0xbc);
    let code_cstr = exe::call_va!(0x1_401d_f680, (r10_p5 + 0xb8) -> u64);
    let ok = exe::call_va!(
        0x1_4246_e970,
        (rdi, code_cstr, code_len, s158, name_cstr, r14d as u32) -> u8
    );

    if ok == 0 {
        let mut err_buf = [0u64; 3];
        let ret_err = exe::call_va!(0x1_41ef_44d0, (err_buf.as_mut_ptr() as usize, rdi) -> usize);
        let cur_ctx = rd::<u64>(rd::<usize>(rdi + 0x78) + 0x18);
        if cur_ctx == 0x100 {
            wr::<u8>(tv50_ptr + 0x08, 0);
        } else {
            wr::<u64>(local_70.as_mut_ptr() as usize, cur_ctx);
            wr::<u8>((local_70.as_mut_ptr() as usize) + 0x08, 1);
            wr::<[u64; 2]>(tv50_ptr, local_70);
        }
        let cb_fn = rd::<usize>(exe::ptr::<usize>(0x1_43a3_9378) as usize);
        if cb_fn != 0 {
            let mut arg4_copy = rd::<[u64; 3]>(ret_err);
            exe::call_va!(
                cb_fn,
                (
                    (&mut p6_slot as *mut usize) as usize,
                    s158,
                    tv50_ptr,
                    arg4_copy.as_mut_ptr() as usize
                )
            );
        }
        exe::call_va!(0x1_4211_8120, (rdi, r14d as u32));
        exe::call_va!(0x1_4240_ab10, (s158));
        wr::<u64>(rd::<usize>(rdi + 0x78) + 0x18, saved_18);
        exe::call_va!(0x1_4240_ab10, (s1e8));
        return 0;
    }

    exe::call_va!(0x1_4240_ab10, (s158));
    wr::<u64>(rd::<usize>(rdi + 0x78) + 0x18, saved_18);
    let top_nil = rd::<usize>(rdi + 0x08);
    wr::<u32>(top_nil + 0x10, 0);
    wr::<usize>(rdi + 0x08, top_nil.wrapping_add(0x18));

    let mut tv_30 = [0u64; 3];
    let tv30_ptr = tv_30.as_mut_ptr() as usize;
    let cls_tbl2 = exe::call_va!(0x1_4211_6d40, (rdi, 0xffff_d8f0u32) -> usize);
    let s_cls2 = exe::call_va!(
        0x1_4210_3330,
        (rdi, exe::ptr::<u8>(0x1_4336_68f8) as usize, 0x18u32) -> u64
    );
    wr::<u64>(tv30_ptr, s_cls2);
    wr::<u32>(tv30_ptr + 0x10, 5);
    exe::call_va!(
        0x1_420e_c690,
        (
            rdi,
            cls_tbl2,
            tv30_ptr,
            rd::<usize>(rdi + 0x08).wrapping_sub(0x18)
        )
    );
    let mut cur_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
    wr::<usize>(rdi + 0x08, cur_top);

    const CALLBACK_SPECS: [(u64, u32, u64, u32); 5] = [
        (0x1_4336_6928, 0x09, 0x1_4336_6848, 0x13),
        (0x1_4336_6938, 0x0a, 0x1_4336_6860, 0x14),
        (0x1_4336_69f0, 0x17, 0x1_4336_6878, 0x21),
        (0x1_4336_6a08, 0x18, 0x1_4336_68a0, 0x22),
        (0x1_4336_6a28, 0x16, 0x1_4336_6948, 0x20),
    ];

    for (fn_va, fn_len, cb_va, cb_len) in CALLBACK_SPECS {
        let rbx_id = rd::<u64>(param_5 + 0x10);
        let esi = lua_abs_idx(rdi, cur_top, r14d);
        exe::call_va!(0x1_4211_8a60, (rdi, exe::ptr::<u8>(fn_va) as usize, fn_len));
        let ty = exe::call_va!(0x1_4211_81c0, (rdi, esi) -> i32);
        if ty == 7 {
            lua_gc_barrier_thread(rdi);
            let tbl = exe::call_va!(0x1_4211_6d40, (rdi, 0xffff_d8f0u32) -> usize);
            let s_cb = exe::call_va!(
                0x1_4210_3330,
                (rdi, exe::ptr::<u8>(cb_va) as usize, cb_len) -> u64
            );
            wr::<u64>(tv30_ptr, s_cb);
            wr::<u32>(tv30_ptr + 0x10, 5);
            exe::call_va!(0x1_420e_c940, (rdi, tbl, tv30_ptr, rd::<usize>(rdi + 0x08)));
            let t1 = rd::<usize>(rdi + 0x08).wrapping_add(0x18);
            wr::<usize>(rdi + 0x08, t1);
            wr::<u64>(t1, rbx_id);
            wr::<u32>(t1 + 0x10, 2);
            wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));
            lua_gc_barrier_thread(rdi);
            let t2 = rd::<usize>(rdi + 0x08);
            wr::<[u64; 3]>(t2, rd::<[u64; 3]>(t2.wrapping_sub(0x48)));
            let r9_old = rd::<usize>(rdi + 0x08);
            let r8_new = r9_old.wrapping_add(0x18);
            wr::<usize>(rdi + 0x08, r8_new);
            exe::call_va!(
                0x1_420e_c690,
                (rdi, r8_new.wrapping_sub(0x48), r8_new.wrapping_sub(0x30), r9_old)
            );
            cur_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x60);
        } else {
            cur_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
        }
        wr::<usize>(rdi + 0x08, cur_top);
    }

    let upd_idx = lua_abs_idx(rdi, cur_top, r14d);
    exe::call_va!(
        0x1_4211_8a60,
        (rdi, exe::ptr::<u8>(0x1_4336_6a40) as usize, 6u32)
    );
    if exe::call_va!(0x1_4211_81c0, (rdi, upd_idx) -> i32) == 7 {
        let ref_id = exe::call_va!(0x1_4211_8640, (rdi, 0xffff_ffffu32) -> u32);
        let slot = exe::call_va!(0x1_4105_a770, (param_1 + 0x248, param_5 + 0x10) -> usize);
        wr::<u32>(slot + 0x08, ref_id);
    }
    wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));

    let init_idx = exe::call_va!(0x1_4211_8020, (rdi, r14d as u32) -> u32);
    exe::call_va!(
        0x1_4211_8a60,
        (rdi, exe::ptr::<u8>(0x1_4336_6a48) as usize, 4u32)
    );
    if exe::call_va!(0x1_4211_81c0, (rdi, init_idx) -> i32) == 7 {
        wr::<u32>(param_5 + 0x08, 1);
        let ref_id = exe::call_va!(0x1_4211_8640, (rdi, 0xffff_ffffu32) -> u32);
        wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));
        wr::<u64>(local_70.as_mut_ptr() as usize, rd::<u64>(param_5 + 0x10));
        wr::<u32>((local_70.as_mut_ptr() as usize) + 0x08, ref_id);
        let ecx_len = rd::<u32>(param_3 + 0x30);
        let rdx_end = rd::<usize>(param_3 + 0x28).wrapping_add((ecx_len as usize) * 16);
        let cap = rd::<u32>(param_3 + 0x34) & 0x3fff_ffff;
        if ecx_len == cap {
            let mut pos_slot = rdx_end;
            exe::call_va!(
                0x1_4026_8f50,
                (
                    param_3 + 0x28,
                    (&mut pos_slot as *mut usize) as usize,
                    local_70.as_mut_ptr() as usize
                )
            );
        } else {
            wr::<u32>(param_3 + 0x30, ecx_len.wrapping_add(1));
            wr::<[u64; 2]>(rdx_end, local_70);
        }
    } else {
        wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));
        wr::<u32>(param_5 + 0x08, 3);
    }

    wr::<u32>(param_5 + 0x1c, 0);
    const FLAG_SPECS: [(u64, u32, u32); 5] = [
        (0x1_4336_6928, 0x09, 0x01),
        (0x1_4336_6938, 0x0a, 0x02),
        (0x1_4336_69f0, 0x17, 0x10),
        (0x1_4336_6a08, 0x18, 0x20),
        (0x1_4336_6a28, 0x16, 0x04),
    ];
    for (fn_va, fn_len, bit) in FLAG_SPECS {
        let idx = exe::call_va!(0x1_4211_8020, (rdi, r14d as u32) -> u32);
        exe::call_va!(0x1_4211_8a60, (rdi, exe::ptr::<u8>(fn_va) as usize, fn_len));
        if exe::call_va!(0x1_4211_81c0, (rdi, idx) -> i32) == 7 {
            wr::<u32>(param_5 + 0x1c, rd::<u32>(param_5 + 0x1c) | bit);
        }
        wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));
    }

    let mut rsi_top = rd::<usize>(rdi + 0x08);
    let rcx_p6 = rd::<usize>(p6_slot);
    let rdx_path =
        rcx_p6.wrapping_add(rd::<u32>(rcx_p6 + 0x14).wrapping_add(rd::<u32>(rcx_p6 + 0x10)) as usize);
    let path_arg = if rd::<u8>(rdx_path) == 0x40 {
        rdx_path.wrapping_add(1)
    } else {
        rdx_path
    };
    if exe::call_va!(0x1_41ec_0f90, (path_arg) -> u8) != 0 {
        wr::<u32>(param_5 + 0x1c, rd::<u32>(param_5 + 0x1c) | 0x08);
        let cnt_48 = rd::<u32>(param_4 + 0x48) as usize;
        let arr_40 = rd::<usize>(param_4 + 0x40);
        let target_id = rd::<u64>(param_5 + 0x10);
        let mut found_idx = cnt_48;
        for i in 0..cnt_48 {
            if rd::<u64>(arr_40.wrapping_add(i * 8)) == target_id {
                found_idx = i;
                break;
            }
        }
        let flag_ptr = param_4.wrapping_add(0x138).wrapping_add(found_idx);
        wr::<u8>(flag_ptr, rd::<u8>(flag_ptr) & !0x03);

        let add_idx = exe::call_va!(0x1_4211_8020, (rdi, r14d as u32) -> u32);
        exe::call_va!(
            0x1_4211_8a60,
            (rdi, exe::ptr::<u8>(0x1_4336_53c8) as usize, 0x0au32)
        );
        if exe::call_va!(0x1_4211_81c0, (rdi, add_idx) -> i32) == 7 {
            wr::<u8>(flag_ptr, rd::<u8>(flag_ptr) | 0x01);
        }
        wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_sub(0x18));

        let rem_idx = exe::call_va!(0x1_4211_8020, (rdi, r14d as u32) -> u32);
        exe::call_va!(
            0x1_4211_8a60,
            (rdi, exe::ptr::<u8>(0x1_4336_5390) as usize, 0x0du32)
        );
        if exe::call_va!(0x1_4211_81c0, (rdi, rem_idx) -> i32) == 7 {
            wr::<u8>(flag_ptr, rd::<u8>(flag_ptr) | 0x02);
        }
        rsi_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
        wr::<usize>(rdi + 0x08, rsi_top);
    }

    let p6_obj = rd::<usize>(p6_slot);
    let mut rbx_cur = p6_obj.wrapping_add(rd::<u32>(p6_obj + 0x38) as usize);
    let r13_end = rbx_cur.wrapping_add((rd::<u32>(p6_obj + 0x3c) as usize) * 16);
    while rbx_cur != r13_end {
        let cur_p6 = rd::<usize>(p6_slot);
        let r9_str = cur_p6
            .wrapping_add(rd::<u32>(rbx_cur + 0x0c).wrapping_add(rd::<u32>(cur_p6 + 0x10)) as usize);
        let r15d = lua_abs_idx(rdi, rsi_top, r14d);
        if r9_str == 0 {
            wr::<u32>(rsi_top + 0x10, 0);
            wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));
        } else {
            let slen = c_strlen(r9_str);
            exe::call_va!(0x1_4211_8a60, (rdi, r9_str, slen));
        }
        if exe::call_va!(0x1_4211_81c0, (rdi, r15d) -> i32) == 7 {
            let cnt_fb8 = rd::<i32>(exe::ptr::<i32>(0x1_43a4_6fb8) as usize);
            if cnt_fb8 > 0 {
                let arr_fb0 = rd::<usize>(exe::ptr::<usize>(0x1_43a4_6fb0) as usize);
                for k in 0..(cnt_fb8 as usize) {
                    let s_ptr = rd::<usize>(arr_fb0.wrapping_add(k * 8));
                    let s_len = c_strlen(s_ptr);
                    let h = exe::call_va!(0x1_424b_1ec0, (s_ptr, s_len) -> u32);
                    if h == rd::<u32>(rbx_cur + 0x08) {
                        let arr_fc0 = rd::<usize>(exe::ptr::<usize>(0x1_43a4_6fc0) as usize);
                        let v0 = rd::<u32>(arr_fc0.wrapping_add(k * 4));
                        let v1 = exe::call_va!(0x1_4211_8640, (rdi, 0xffff_ffffu32) -> u32);
                        let mut node: [u64; 4] = [
                            (v0 as u64) | ((v1 as u64) << 32),
                            0,
                            rd::<u64>(rbx_cur),
                            rd::<u64>(param_5 + 0x10),
                        ];
                        exe::call_va!(
                            0x1_4104_e610,
                            (rd::<usize>(param_3), node.as_mut_ptr() as usize)
                        );
                        break;
                    }
                }
            }
        }
        rsi_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
        wr::<usize>(rdi + 0x08, rsi_top);
        rbx_cur = rbx_cur.wrapping_add(16);
    }

    let p6_obj2 = rd::<usize>(p6_slot);
    let mut rbx_cur2 = p6_obj2.wrapping_add(rd::<u32>(p6_obj2 + 0x40) as usize);
    let r13_end2 = rbx_cur2.wrapping_add((rd::<u32>(p6_obj2 + 0x44) as usize) * 16);
    while rbx_cur2 != r13_end2 {
        let cur_p6 = rd::<usize>(p6_slot);
        let r9_str = cur_p6
            .wrapping_add(rd::<u32>(rbx_cur2 + 0x0c).wrapping_add(rd::<u32>(cur_p6 + 0x10)) as usize);
        let r15d = lua_abs_idx(rdi, rsi_top, r14d);
        if r9_str == 0 {
            wr::<u32>(rsi_top + 0x10, 0);
            wr::<usize>(rdi + 0x08, rd::<usize>(rdi + 0x08).wrapping_add(0x18));
        } else {
            let slen = c_strlen(r9_str);
            exe::call_va!(0x1_4211_8a60, (rdi, r9_str, slen));
        }
        if exe::call_va!(0x1_4211_81c0, (rdi, r15d) -> i32) == 7 {
            let v0 = rd::<u32>(rbx_cur2 + 0x08);
            let v1 = exe::call_va!(0x1_4211_8640, (rdi, 0xffff_ffffu32) -> u32);
            let mut node: [u64; 4] = [
                (v0 as u64) | ((v1 as u64) << 32),
                0,
                rd::<u64>(rbx_cur2),
                rd::<u64>(param_5 + 0x10),
            ];
            exe::call_va!(
                0x1_4104_e610,
                (rd::<usize>(param_3 + 0x10), node.as_mut_ptr() as usize)
            );
        }
        rsi_top = rd::<usize>(rdi + 0x08).wrapping_sub(0x18);
        wr::<usize>(rdi + 0x08, rsi_top);
        rbx_cur2 = rbx_cur2.wrapping_add(16);
    }

    exe::call_va!(0x1_4211_8120, (rdi, r14d as u32));
    exe::call_va!(0x1_4240_ab10, (s1e8));
    1
}

// =============================================================================
// Differential tests
// =============================================================================

unsafe fn run_transform_diff(
    orig: usize,
    new: usize,
    swapped_io: bool,
    label: &str,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NOTIFIED_ARG: usize = 0;

    unsafe extern "win64" fn mock_9ee0(_rcx: u64, _rdx: u32, _r8d: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_2790(arg: usize) {
        NOTIFIED_ARG = arg;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e2_2790, mock_2790 as *const ())?;

    let run = |f: F, mode: u8| -> ([u32; 8], [u32; 8], usize) {
        NOTIFIED_ARG = 0;

        let in_tf: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 1.5, -2.25, 3.75, 1.0];
        let par_tf: [f32; 8] = [0.5, -0.5, 0.5, 0.5, 10.0, 20.0, -5.0, 0.0];

        let mut arr_a = [0f32; 16];
        let mut arr_b = [0f32; 16];
        if mode == 0 {
            arr_a[8..16].copy_from_slice(&in_tf);
            arr_b[8..16].copy_from_slice(&in_tf);
        }

        let mut handles = [u64::MAX; 2];
        if mode == 2 {
            handles[1] = 0x0000_0005_0000_0000u64;
        }

        let mut notify_tbl = [0usize; 6];
        notify_tbl[3] = 0x1234_5678;

        let mut p1 = [0u64; 5];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, handles.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x08, arr_a.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x10, arr_b.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x18, notify_tbl.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x20, 1);

        let mut sparse_18 = [0x0000_0005_0000_0000u64; 2];
        let mut dense_30 = [0x0000_0001_0000_0000u64; 2];
        let mut col_offs = [0u32; 4];
        col_offs[0] = 0;
        let mut comp_data = [0f32; 16];
        comp_data[8..16].copy_from_slice(&par_tf);

        let mut scene = vec![0u8; 0x18100];
        let sp = scene.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x18, sparse_18.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x20, 2);
        wr::<usize>(sp + 0x30, dense_30.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x88, comp_data.as_mut_ptr() as usize);
        wr::<usize>(sp + 0xc03 * 32 + 0x40, col_offs.as_mut_ptr() as usize);

        let mut p2 = [0u64; 4];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, sp);

        f(p1p, p2p, in_tf.as_ptr() as usize);

        let out_a = rd::<[u32; 8]>((arr_a.as_ptr() as usize) + 32);
        let out_b = rd::<[u32; 8]>((arr_b.as_ptr() as usize) + 32);
        if swapped_io {
            (out_b, out_a, NOTIFIED_ARG)
        } else {
            (out_a, out_b, NOTIFIED_ARG)
        }
    };

    for mode in [0u8, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "{} mode={}", label, mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141e22e20(orig: usize, new: usize) -> Result<u32, String> {
    run_transform_diff(orig, new, false, "141e22e20")
}

pub unsafe fn diff_unknown_141e23170(orig: usize, new: usize) -> Result<u32, String> {
    run_transform_diff(orig, new, false, "141e23170")
}

pub unsafe fn diff_unknown_141e234c0(orig: usize, new: usize) -> Result<u32, String> {
    run_transform_diff(orig, new, true, "141e234c0")
}

pub unsafe fn diff_unknown_141e99650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut U16_SPAN: [u16; 2] = [0, 1];
    static mut SRC_BYTES: [u8; 4] = [0; 4];
    static mut DST_BYTES: [u8; 4] = [0; 4];

    unsafe extern "win64" fn mock_7c10(out: usize, r11: usize, _pair: usize) -> usize {
        let base_u16 = core::ptr::addr_of_mut!(U16_SPAN) as usize;
        wr::<usize>(out, r11);
        wr::<usize>(out + 0x08, base_u16);
        wr::<usize>(out + 0x10, base_u16 + 4);
        out
    }

    unsafe extern "win64" fn mock_8650(iter: usize) {
        wr::<usize>(iter + 0x38, core::ptr::addr_of_mut!(SRC_BYTES) as usize);
        wr::<usize>(iter + 0x40, core::ptr::addr_of_mut!(DST_BYTES) as usize);
    }

    unsafe extern "win64" fn mock_9ee0(_rcx: u64, _rdx: u32, _r8d: u32) -> i32 {
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4100_8650, mock_8650 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;

    let run = |f: F, empty_span: bool| -> ([u8; 4], [u8; 4], [u8; 4]) {
        U16_SPAN = [0, 1];
        SRC_BYTES = [0x11, 0x22, 0x33, 0x44];
        DST_BYTES = [0; 4];

        let mut scene = vec![0u8; 0x18100];
        let sp = scene.as_mut_ptr() as usize;
        if !empty_span {
            wr::<u64>(sp + 0x40 + 0x48, 1);
            wr::<u32>(sp + 0x40 + 0x10048, 3);
        }

        let mut dense_30 = [0x0000_0002_0000_0000u64; 2];
        let mut col_offs = [1u32; 2];
        let mut flag_buf = [0u8; 4];
        wr::<usize>(sp + 0x30, dense_30.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x88, flag_buf.as_mut_ptr() as usize);
        wr::<usize>(sp + 0xc03 * 32 + 0x40, col_offs.as_mut_ptr() as usize);

        let mut elem_obj = [0u8; 0x48];
        wr::<u64>((elem_obj.as_mut_ptr() as usize) + 0x40, 1);
        let mut elem_ptrs = [elem_obj.as_mut_ptr() as usize];

        let mut inner_p2 = [0u8; 0x148];
        let ip2 = inner_p2.as_mut_ptr() as usize;
        wr::<usize>(ip2 + 0x138, elem_ptrs.as_mut_ptr() as usize);
        wr::<u32>(ip2 + 0x140, 1);

        let mut mid_p2 = [ip2];
        let mut outer_p2 = [mid_p2.as_mut_ptr() as usize];

        let mut p1 = [sp as u64, 10, 20];
        f(p1.as_mut_ptr() as usize, outer_p2.as_mut_ptr() as usize);

        (SRC_BYTES, DST_BYTES, flag_buf)
    };

    for empty in [false, true] {
        let o = run(f_o, empty);
        let n = run(f_n, empty);
        crate::diff_eq!(o, n, "141e99650 empty={}", empty);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141ec0f90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [&[u8]; 4] = [
        b"sourcedata\\lua_scripts\\systems\\test.lua\0",
        b"SOURCEDATA/LUA_SCRIPTS/SYSTEMS/other.lua\0",
        b"sourcedata\\lua_scripts\\actors\\test.lua\0",
        b"short\0",
    ];

    for (idx, cstr) in cases.iter().enumerate() {
        let o = f_o(cstr.as_ptr() as usize);
        let n = f_n(cstr.as_ptr() as usize);
        crate::diff_eq!(o, n, "141ec0f90 case={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141ec4a40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG: Vec<(u8, usize)> = Vec::new();

    unsafe extern "win64" fn mock_e830(p: usize) {
        LOG.push((1, p));
    }
    unsafe extern "win64" fn mock_e270(p: usize) {
        LOG.push((2, p));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4037_e270, mock_e270 as *const ())?;

    let run = |f: F, cnt: u64| -> Vec<(u8, usize)> {
        LOG.clear();
        let base = 0x1000_0000usize;
        f(base, cnt);
        LOG.iter().map(|&(k, p)| (k, p - base)).collect()
    };

    for cnt in [0u64, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141ec4a40 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141ec85a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DUMMY_OBJ: [u8; 64] = [0; 64];
    static mut REG_TV: [u64; 3] = [0; 3];
    static mut ERR_VAL: [u64; 3] = [0x1111, 0x2222, 0x3333];
    static mut UPD_SLOT: [u32; 4] = [0; 4];
    static mut LOAD_OK: u8 = 1;
    static mut CALL_TRACE: Vec<(u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_9b50(this: usize, view: usize, _pos: u64, _cnt: u64) {
        CALL_TRACE.push((1, rd::<u64>(view + 8)));
        wr::<u32>(this + 4, rd::<u32>(view + 8));
    }
    unsafe extern "win64" fn mock_9080(_l: usize, a2: u32, a3: u32) {
        CALL_TRACE.push((2, ((a2 as u64) << 32) | (a3 as u64)));
    }
    unsafe extern "win64" fn mock_6d40(_l: usize, idx: u32) -> usize {
        CALL_TRACE.push((3, idx as u64));
        core::ptr::addr_of_mut!(REG_TV) as usize
    }
    unsafe extern "win64" fn mock_3330(_l: usize, _s: usize, len: u32) -> u64 {
        CALL_TRACE.push((4, len as u64));
        0x5000 + len as u64
    }
    unsafe extern "win64" fn mock_c690(_l: usize, _t: usize, _k: usize, _v: usize) {
        CALL_TRACE.push((5, 0));
    }
    unsafe extern "win64" fn mock_84a0(_l: usize, idx: u32) {
        CALL_TRACE.push((6, idx as u64));
    }
    unsafe extern "win64" fn mock_8910(_l: usize, idx: u32) {
        CALL_TRACE.push((7, idx as u64));
    }
    unsafe extern "win64" fn mock_c940(_l: usize, _t: usize, _k: usize, _v: usize) {
        CALL_TRACE.push((8, 0));
    }
    unsafe extern "win64" fn mock_2350(_l: usize, idx: u32, _p6: usize) {
        CALL_TRACE.push((9, idx as u64));
    }
    unsafe extern "win64" fn mock_f680(s: usize) -> u64 {
        (s + 8) as u64
    }
    unsafe extern "win64" fn mock_e970(
        _l: usize,
        _code: u64,
        code_len: u32,
        _err: usize,
        _name: u64,
        idx: u32,
    ) -> u8 {
        CALL_TRACE.push((10, ((code_len as u64) << 32) | (idx as u64)));
        LOAD_OK
    }
    unsafe extern "win64" fn mock_44d0(out: usize, _l: usize) -> usize {
        wr::<[u64; 3]>(out, ERR_VAL);
        out
    }
    unsafe extern "win64" fn mock_8120(_l: usize, idx: u32) {
        CALL_TRACE.push((11, idx as u64));
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {
        CALL_TRACE.push((12, 0));
    }
    unsafe extern "win64" fn mock_8a60(l: usize, _s: usize, len: u64) {
        CALL_TRACE.push((13, len));
        wr::<usize>(l + 0x08, rd::<usize>(l + 0x08).wrapping_add(0x18));
    }
    unsafe extern "win64" fn mock_81c0(_l: usize, idx: u32) -> i32 {
        CALL_TRACE.push((14, idx as u64));
        7
    }
    unsafe extern "win64" fn mock_8640(_l: usize, _idx: u32) -> u32 {
        CALL_TRACE.push((15, 0));
        42
    }
    unsafe extern "win64" fn mock_a770(_map: usize, _key: usize) -> usize {
        core::ptr::addr_of_mut!(UPD_SLOT) as usize
    }
    unsafe extern "win64" fn mock_8020(_l: usize, idx: u32) -> u32 {
        idx + 10
    }
    unsafe extern "win64" fn mock_8f50(_vec: usize, _pos: usize, _item: usize) {
        CALL_TRACE.push((16, 0));
    }
    unsafe extern "win64" fn mock_e610(_dst: usize, node: usize) {
        CALL_TRACE.push((17, rd::<u64>(node)));
    }

    let _m01 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4211_9080, mock_9080 as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_4211_6d40, mock_6d40 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4210_3330, mock_3330 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_420e_c690, mock_c690 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_4211_84a0, mock_84a0 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4211_8910, mock_8910 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_420e_c940, mock_c940 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_41ee_2350, mock_2350 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_401d_f680, mock_f680 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4246_e970, mock_e970 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_41ef_44d0, mock_44d0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4211_8120, mock_8120 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4211_8a60, mock_8a60 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4211_81c0, mock_81c0 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4211_8640, mock_8640 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4105_a770, mock_a770 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4211_8020, mock_8020 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_4026_8f50, mock_8f50 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_4104_e610, mock_e610 as *const ())?;

    let run = |f: F, ok_flag: u8| -> (u8, u32, [u32; 8], u8, u32, Vec<(u32, u64)>) {
        LOAD_OK = ok_flag;
        CALL_TRACE.clear();
        DUMMY_OBJ = [0; 64];
        UPD_SLOT = [0; 4];
        REG_TV = [core::ptr::addr_of_mut!(DUMMY_OBJ) as u64, 0, 5];

        let mut lua_stack = [0u64; 64];
        let stack_base = (lua_stack.as_mut_ptr() as usize) + 0x60;
        wr::<usize>(
            stack_base,
            core::ptr::addr_of_mut!(DUMMY_OBJ) as usize,
        );

        let mut lua_g = [0u8; 0x40];
        let mut lua_ctx = [0u8; 0x20];
        wr::<u64>((lua_ctx.as_mut_ptr() as usize) + 0x18, 0x7777);

        let mut lua_state = [0u8; 0x90];
        let lp = lua_state.as_mut_ptr() as usize;
        wr::<u8>(lp + 0x01, 4);
        wr::<usize>(lp + 0x08, stack_base + 0x18);
        wr::<usize>(lp + 0x10, stack_base);
        wr::<usize>(lp + 0x18, lua_g.as_mut_ptr() as usize);
        wr::<usize>(lp + 0x78, lua_ctx.as_mut_ptr() as usize);

        let mut p1 = [0u8; 0x260];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, lp);
        wr::<u32>(p1p + 0x240, 0);

        let mut vec_buf = [0u64; 4];
        let mut p3 = [0u8; 0x40];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x10, 0x9000);
        wr::<usize>(p3p + 0x28, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(p3p + 0x30, 0);
        wr::<u32>(p3p + 0x34, 4);

        let mut p4_ids = [0xABCD_1234u64; 2];
        let mut p4 = [0u8; 0x150];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x40, p4_ids.as_mut_ptr() as usize);
        wr::<u32>(p4p + 0x48, 2);
        wr::<u8>(p4p + 0x138, 0xff);

        let mut p5_inner = [0u8; 0xd0];
        wr::<u32>((p5_inner.as_mut_ptr() as usize) + 0xbc, 12);
        let mut p5 = [0u32; 8];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<usize>(p5p, p5_inner.as_mut_ptr() as usize);
        wr::<u64>(p5p + 0x10, 0xABCD_1234);

        let mut p6_blob = [0u8; 0xc0];
        let p6b = p6_blob.as_mut_ptr() as usize;
        let path_bytes = b"@sourcedata\\lua_scripts\\systems\\sys.lua\0";
        p6_blob[0x50..0x50 + path_bytes.len()].copy_from_slice(path_bytes);
        let fn_bytes = b"my_cb\0";
        p6_blob[0x80..0x80 + fn_bytes.len()].copy_from_slice(fn_bytes);
        wr::<u32>(p6b + 0x10, 0x40);
        wr::<u32>(p6b + 0x14, 0x10);
        wr::<u32>(p6b + 0x38, 0);
        wr::<u32>(p6b + 0x3c, 0);
        wr::<u32>(p6b + 0x40, 0x90);
        wr::<u32>(p6b + 0x44, 1);
        wr::<u64>(p6b + 0x90, 0x5566_7788);
        wr::<u32>(p6b + 0x98, 99);
        wr::<u32>(p6b + 0x9c, 0x40);

        let mut p6 = [p6b];
        let ret = f(p1p, 0, p3p, p4p, p5p, p6.as_mut_ptr() as usize);
        (
            ret,
            rd::<u32>(p1p + 0x240),
            p5,
            rd::<u8>(p4p + 0x138),
            UPD_SLOT[2],
            CALL_TRACE.clone(),
        )
    };

    for ok_flag in [0u8, 1] {
        let o = run(f_o, ok_flag);
        let n = run(f_n, ok_flag);
        crate::diff_eq!(o, n, "141ec85a0 ok={}", ok_flag);
    }
    Ok(2)
}
