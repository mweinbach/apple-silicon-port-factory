//! Decompiled `rend` hot functions shard ZA23 (`0x1424b2600..0x1424e08e0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn vrcpss_f32(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        f32::from_bits(crate::simd_ops::vrcpss(0, x.to_bits()) as u32)
    }
}

#[inline(always)]
unsafe fn remap_linear_rcp(
    val_bits: u32,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    one: f32,
) -> f32 {
    use crate::simd_ops::{vblendvps, vcmpss};
    let x = f32::from_bits(val_bits);
    let max_lo = if x > x0 { x } else { x0 };
    let clamped = if max_lo < x1 { max_lo } else { x1 };
    let num = clamped - x0;
    let dx = x1 - x0;
    let eq0 = vcmpss(0, dx.to_bits(), 0.0f32.to_bits(), 0);
    let denom = f32::from_bits(vblendvps(dx.to_bits() as u128, one.to_bits() as u128, eq0) as u32);
    let rcp = vrcpss_f32(denom);
    let nr = (rcp + rcp) - (rcp * rcp * denom);
    let interp = y0 + (num * nr) * (y1 - y0);
    let le_lo = vcmpss(0, x.to_bits(), x0.to_bits(), 2);
    let sel1 = vblendvps(interp.to_bits() as u128, y0.to_bits() as u128, le_lo);
    let le_hi = vcmpss(0, x1.to_bits(), x.to_bits(), 2);
    let sel2 = vblendvps(sel1, y1.to_bits() as u128, le_hi);
    f32::from_bits(sel2 as u32)
}

#[inline(always)]
unsafe fn inplace_string_data_ptr(str_ptr: usize, hdr: u32) -> usize {
    let top = hdr >> 24;
    let inline_cap = if top < 0x80 {
        top.wrapping_mul(8)
    } else {
        (top << 8).wrapping_sub(0x7c00)
    };
    let alloc_cap = (hdr & 0x00ff_ffff) << 3;
    if alloc_cap > inline_cap {
        rd::<usize>(str_ptr + 0x08)
    } else {
        str_ptr + 0x08
    }
}

/// `cityhash_1424b2600`
/// Original: 0x1424b2600 (1526 bytes, Throws::No)
pub unsafe extern "win64" fn cityhash_1424b2600(param_1: usize) -> u64 {
    const K0: u64 = 0xc3a5_c85c_97cb_3127; // -0x3c5a37a36834ced9
    const K1: u64 = 0xb492_b66f_be98_f273; // -0x4b6d499041670d8d
    const K2: u64 = 0x9ae1_6a3b_2f90_404f; // -0x651e95c4d06fbfb1
    const K2_SHL8: u64 = 0xe16a_3b2f_9040_4f00; // -0x1e95c4d06fbfb100
    const K_MUL: u64 = 0x9ddf_ea08_eb38_2d69; // -0x622015f714c7d297

    // InplaceString<2048>: header 0x8400_0100 requires 8 + 2048 = 2056 bytes
    let mut local_str = [0u64; 0x104];
    let str_ptr = local_str.as_mut_ptr() as usize;
    wr::<u32>(str_ptr, 0x8400_0100);
    wr::<u32>(str_ptr + 0x04, 0);
    wr::<u8>(str_ptr + 0x08, 0);

    let view: [u64; 2] = [rd::<u64>(param_1), rd::<u64>(param_1 + 0x08)];
    exe::call_va!(
        0x1_4240_9b50,
        (str_ptr, view.as_ptr() as usize, 0u64, view[1])
    );

    let len0 = rd::<u32>(str_ptr + 0x04) as usize;
    let hdr0 = rd::<u32>(str_ptr);
    let data0 = inplace_string_data_ptr(str_ptr, hdr0);
    if len0 != 0 {
        let lut = exe::ptr::<u8>(0x1_427e_54e0) as usize;
        for i in 0..len0 {
            let b = rd::<u8>(data0 + i) as usize;
            wr::<u8>(data0 + i, rd::<u8>(lut + b));
        }
    }

    let len = rd::<u32>(str_ptr + 0x04) as u64;
    let hdr = rd::<u32>(str_ptr);
    let s = inplace_string_data_ptr(str_ptr, hdr);
    let len_usize = len as usize;

    let result: u64 = if len <= 0x20 {
        if len <= 0x10 {
            if len >= 8 {
                let r8 = K2.wrapping_add(len.wrapping_mul(2));
                let rax = rd::<u64>(s).wrapping_sub(0x651e_95c4_d06f_bfb1);
                let rcx = rd::<u64>(s + len_usize - 8);
                let rdx = rax.rotate_right(0x19).wrapping_add(rcx).wrapping_mul(r8);
                let rcx2 = (rcx.rotate_right(0x25).wrapping_mul(r8).wrapping_add(rax) ^ rdx)
                    .wrapping_mul(r8);
                let rax2 = ((rcx2 >> 0x2f) ^ rcx2 ^ rdx).wrapping_mul(r8);
                ((rax2 >> 0x2f) ^ rax2).wrapping_mul(r8)
            } else if len >= 4 {
                let r8 = K2.wrapping_add(len.wrapping_mul(2));
                let rdx = rd::<u32>(s + len_usize - 4) as u64;
                let rax = rd::<u32>(s) as u64;
                let rcx = (len.wrapping_add(rax.wrapping_mul(8)) ^ rdx).wrapping_mul(r8);
                let rax2 = ((rcx >> 0x2f) ^ rcx ^ rdx).wrapping_mul(r8);
                ((rax2 >> 0x2f) ^ rax2).wrapping_mul(r8)
            } else if len != 0 {
                let mid = rd::<u8>(s + ((len >> 1) as usize)) as u64;
                let rdx = mid
                    .wrapping_mul(K2_SHL8)
                    .wrapping_add((rd::<u8>(s) as u64).wrapping_mul(K2));
                let last = rd::<u8>(s + len_usize - 1) as u64;
                let rax = ((len.wrapping_add(last.wrapping_mul(4)) as u32) as u64).wrapping_mul(K0);
                let rdx2 = rdx ^ rax;
                ((rdx2 >> 0x2f) ^ rdx2).wrapping_mul(K2)
            } else {
                K2
            }
        } else {
            let r11 = K2.wrapping_add(len.wrapping_mul(2));
            let rdx = rd::<u64>(s).wrapping_mul(K1);
            let rcx = rd::<u64>(s + 0x08);
            let r8 = r11.wrapping_mul(rd::<u64>(s + len_usize - 0x08));
            let r9 = rcx
                .wrapping_add(K2)
                .rotate_right(0x12)
                .wrapping_add(r8)
                .wrapping_add(rdx);
            let r10 = (rcx
                .wrapping_add(rdx)
                .rotate_right(0x2b)
                .wrapping_add(r8.rotate_right(0x1e))
                .wrapping_add(rd::<u64>(s + len_usize - 0x10).wrapping_mul(K2))
                ^ r9)
                .wrapping_mul(r11);
            let rax = ((r10 >> 0x2f) ^ r9 ^ r10).wrapping_mul(r11);
            ((rax >> 0x2f) ^ rax).wrapping_mul(r11)
        }
    } else if len <= 0x40 {
        let r9_end = s + len_usize;
        let r14 = K2.wrapping_add(len.wrapping_mul(2));
        let rdx = rd::<u64>(r9_end - 0x18);
        let rbp = rd::<u64>(r9_end - 0x20);
        let rax = rd::<u64>(s + 0x18);
        let mut rcx = rax.wrapping_mul(9);
        let r10 = rd::<u64>(r9_end - 0x08);
        let rbx_v = r14.wrapping_mul(rd::<u64>(r9_end - 0x10));
        let mut rdi = rd::<u64>(s).wrapping_mul(K2).wrapping_add(r10);
        let r8 = (rbp ^ rdi).wrapping_add(1).wrapping_add(rcx);
        rcx = rcx.wrapping_add(rd::<u64>(s + 0x10).wrapping_mul(K2));
        let r11 = rcx.rotate_right(0x2a).wrapping_add(rdx);
        let r9 = rdx.wrapping_add(rcx);
        let rcx2 = rd::<u64>(s + 0x08).rotate_right(0x1e).wrapping_add(rdx);
        let rax2 = r8.wrapping_add(rcx2.wrapping_mul(9));
        rdi = rdi
            .rotate_right(0x2b)
            .wrapping_add(rax2)
            .wrapping_mul(r14)
            .swap_bytes();
        rdi = rdi
            .wrapping_add(r8)
            .wrapping_add(rbx_v)
            .wrapping_mul(r14)
            .swap_bytes();
        rdi = rdi
            .wrapping_add(r9)
            .wrapping_add(r11)
            .wrapping_add(r10)
            .wrapping_mul(r14)
            .swap_bytes();
        rdi = rdi
            .wrapping_add(r9)
            .wrapping_add(rd::<u64>(s + 0x08))
            .wrapping_mul(r14)
            .wrapping_add(rbx_v)
            .wrapping_add(rbp);
        ((rdi >> 0x2f) ^ rdi).wrapping_mul(r14).wrapping_add(r11)
    } else {
        let r9_end = s + len_usize;
        let mut r12 = rd::<u64>(r9_end - 0x28);
        let mut r14 = rd::<u64>(r9_end - 0x38).wrapping_add(rd::<u64>(r9_end - 0x10));
        let rdx = (rd::<u64>(r9_end - 0x30).wrapping_add(len) ^ rd::<u64>(r9_end - 0x18))
            .wrapping_mul(K_MUL);
        let rcx = ((rdx >> 0x2f) ^ rd::<u64>(r9_end - 0x18) ^ rdx).wrapping_mul(K_MUL);
        let mut r10 = ((rcx >> 0x2f) ^ rcx).wrapping_mul(K_MUL);
        let mut rsi = rd::<u64>(r9_end - 0x40).wrapping_add(len);
        let rcx_a = r12.wrapping_add(rsi);
        let r8_a = rsi;
        rsi = rsi.wrapping_add(rd::<u64>(r9_end - 0x30).wrapping_add(rd::<u64>(r9_end - 0x38)));
        let mut r11 = r12.wrapping_add(rsi);
        let mut sp_890 = r11;
        let rax_a = rcx_a.wrapping_add(r10).rotate_right(0x15).wrapping_add(r8_a);
        rsi = rsi.rotate_right(0x2c).wrapping_add(rax_a);

        let r8_b = rd::<u64>(r9_end - 0x08);
        let mut rbp = rd::<u64>(r9_end - 0x20).wrapping_add(K1).wrapping_add(r14);
        let rcx_b = r8_b.wrapping_add(rbp);
        let rdx_b = rbp;
        rbp = rbp.wrapping_add(rd::<u64>(r9_end - 0x10).wrapping_add(rd::<u64>(r9_end - 0x18)));
        let mut r9 = r8_b.wrapping_add(rbp);
        let mut sp_898 = r9;
        let rax_b = rcx_b.wrapping_add(r12).rotate_right(0x15).wrapping_add(rdx_b);
        rbp = rbp.rotate_right(0x2c).wrapping_add(rax_b);

        r12 = r12.wrapping_mul(K1).wrapping_add(rd::<u64>(s));
        let mut rem = len.wrapping_sub(1) & !0x3fu64;
        let mut p = s + 0x30;
        let mut r8 = r10;
        let mut rdi: u64;

        loop {
            let w_m28 = rd::<u64>(p - 0x28);
            rdi = (r11
                .wrapping_add(w_m28)
                .wrapping_add(r14)
                .wrapping_add(r12)
                .rotate_right(0x25)
                .wrapping_mul(K1))
                ^ rbp;
            let w_0 = rd::<u64>(p);
            let w_m08 = rd::<u64>(p - 0x08);
            r14 = r14
                .wrapping_add(rsi.wrapping_add(w_0))
                .rotate_right(0x2a)
                .wrapping_mul(K1)
                .wrapping_add(sp_890)
                .wrapping_add(w_m08);
            r12 = r9.wrapping_add(r8).rotate_right(0x21).wrapping_mul(K1);
            let w_m18 = rd::<u64>(p - 0x18);
            rsi = rsi.wrapping_mul(K1).wrapping_add(rd::<u64>(p - 0x30));
            let rcx_1 = w_m18.wrapping_add(rsi);
            let rdx_1 = rsi;
            let w_m20 = rd::<u64>(p - 0x20);
            rsi = rsi.wrapping_add(w_m20.wrapping_add(w_m28));
            r10 = w_m18.wrapping_add(rsi);
            sp_890 = r10;
            let rax_1 = sp_898
                .wrapping_add(rcx_1)
                .wrapping_add(rdi)
                .rotate_right(0x15)
                .wrapping_add(rdx_1);
            rsi = rsi.rotate_right(0x2c).wrapping_add(rax_1);

            let w_p08 = rd::<u64>(p + 0x08);
            rbp = rbp.wrapping_add(rd::<u64>(p - 0x10)).wrapping_add(r12);
            let rcx_2 = w_m20.wrapping_add(w_p08).wrapping_add(rbp);
            let rdx_2 = rbp;
            rbp = rbp.wrapping_add(w_m08.wrapping_add(w_0));
            r9 = w_p08.wrapping_add(rbp);
            sp_898 = r9;
            let rax_2 = rcx_2
                .wrapping_add(r14)
                .rotate_right(0x15)
                .wrapping_add(rdx_2);
            rbp = rbp.rotate_right(0x2c).wrapping_add(rax_2);

            r8 = rdi;
            p = p.wrapping_add(0x40);
            rem = rem.wrapping_sub(0x40);
            if rem == 0 {
                break;
            }
            r11 = r10;
        }

        let rsi_f = (rsi ^ rbp).wrapping_mul(K_MUL);
        let rdx_f = ((rsi_f >> 0x2f) ^ rsi_f ^ rbp).wrapping_mul(K_MUL);
        let rax_f = (r9 ^ r10).wrapping_mul(K_MUL);
        let rcx_f = ((rax_f >> 0x2f) ^ r9 ^ rax_f).wrapping_mul(K_MUL);
        let rdx_f2 = ((rdx_f ^ (rdx_f >> 0x2f)).wrapping_mul(K_MUL)).wrapping_add(r12);
        let rax_f2 = ((rcx_f >> 0x2f) ^ rcx_f).wrapping_mul(K_MUL).wrapping_add(rdi);
        let rcx_f2 = (((r14 >> 0x2f) ^ r14)
            .wrapping_mul(K1)
            .wrapping_add(rax_f2)
            ^ rdx_f2)
            .wrapping_mul(K_MUL);
        let rax_f3 = ((rcx_f2 >> 0x2f) ^ rcx_f2 ^ rdx_f2).wrapping_mul(K_MUL);
        ((rax_f3 >> 0x2f) ^ rax_f3).wrapping_mul(K_MUL)
    };

    exe::call_va!(0x1_4240_ab10, (str_ptr));
    result
}

/// `r_1424b87a0`
/// Original: 0x1424b87a0 (296 bytes, Throws::No)
pub unsafe extern "win64" fn r_1424b87a0(param_1: usize, param_2: usize) {
    use crate::simd_ops::{vaddps, vbroadcastss, vmaxps, vminps, vmulps, vpermilps};

    let min_x = rd::<f32>(param_1);
    let max_x = rd::<f32>(param_1 + 0x0c);
    if min_x > max_x
        && rd::<f32>(param_1 + 0x04) > rd::<f32>(param_1 + 0x10)
        && rd::<f32>(param_1 + 0x08) > rd::<f32>(param_1 + 0x14)
    {
        return;
    }

    let is_inf = exe::call_va!(0x1_4058_8bd0, (param_1) -> u8);
    if is_inf != 0 {
        return;
    }

    let row0 = rd::<u128>(param_2);
    let row1 = rd::<u128>(param_2 + 0x10);
    let row2 = rd::<u128>(param_2 + 0x20);
    let row3 = rd::<u128>(param_2 + 0x30);

    let xmm10 = vmulps(vbroadcastss(min_x.to_bits()), row0);
    let xmm8 = vmulps(vbroadcastss(rd::<u32>(param_1 + 0x04)), row1);
    let xmm6 = vmulps(vbroadcastss(rd::<u32>(param_1 + 0x08)), row2);

    let xmm9 = vmulps(vbroadcastss(max_x.to_bits()), row0);
    let xmm7 = vmulps(vbroadcastss(rd::<u32>(param_1 + 0x10)), row1);
    let xmm4 = vmulps(vbroadcastss(rd::<u32>(param_1 + 0x14)), row2);

    let min0 = vminps(xmm10, xmm9);
    let min1 = vminps(xmm8, xmm7);
    let sum_min01 = vaddps(min1, min0);
    let max0 = vmaxps(xmm10, xmm9);
    let min2 = vminps(xmm6, xmm4);
    let xmm5 = vaddps(vaddps(sum_min01, min2), row3);

    let max1 = vmaxps(xmm8, xmm7);
    let sum_max01 = vaddps(max1, max0);
    let min_z_out = vpermilps(xmm5, 0xaa) as u32;
    let max2 = vmaxps(xmm6, xmm4);
    let xmm3 = vaddps(vaddps(sum_max01, max2), row3);

    wr::<u64>(param_1, xmm5 as u64);
    wr::<u32>(param_1 + 0x08, min_z_out);
    let max_z_out = vpermilps(xmm3, 0xaa) as u32;
    wr::<u64>(param_1 + 0x0c, xmm3 as u64);
    wr::<u32>(param_1 + 0x14, max_z_out);
}

/// `unknown_1424ce980`
/// Original: 0x1424ce980 (123 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1424ce980(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let stamp = rd::<u64>(param_1 + 0x11a8);
    if rd::<u64>(param_3 + 0x20) != stamp {
        let len = rd::<u32>(param_3 + 0x08) as u64;
        let f_14 = rd::<u32>(param_3 + 0x14);
        let f_10 = rd::<u32>(param_3 + 0x10);
        let ptr = rd::<u64>(param_3);
        let mut local_30: [u64; 2] = [len, ptr];
        let xmm0_val = (len as u128) | ((ptr as u128) << 64);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_424c_e1a0,
            param_1 as u64,
            param_2 as u64,
            local_30.as_mut_ptr() as usize as u64,
            0,
            f_14 as u64,
            0,
            0,
            0,
            [xmm0_val, f_14 as u128, 0, f_10 as u128],
        );
        wr::<u64>(param_3 + 0x20, rd::<u64>(param_1 + 0x11a8));
        return;
    }
    wr::<u64>(param_3 + 0x20, stamp);
}

/// `unknown_1424cea10`
/// Original: 0x1424cea10 (1741 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1424cea10(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
    _param_4_xmm3: usize,
) {
    let [_xmm0, _xmm1, xmm2, xmm3] = crate::simd_ops::take_xmm_args();
    let xmm11_in = xmm2 as u32;
    let xmm13_in = xmm3 as u32;

    wr::<u64>(param_2 + 0x10, 0);
    wr::<u32>(param_2 + 0x08, 0);
    wr::<u8>(param_2 + 0x18, 0);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(param_1 + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (param_1));
        wr::<u32>(param_1 + 0x08, tid);
    }
    wr::<u32>(param_1 + 0x0c, rd::<u32>(param_1 + 0x0c).wrapping_add(1));

    let mut ebx_total: u32 = 0;
    let mut rcx_item = rd::<usize>(param_1 + 0x1110);
    let count_1118 = rd::<u32>(param_1 + 0x1118) as usize;
    let r15_end = rcx_item.wrapping_add(count_1118.wrapping_mul(0x78));
    while rcx_item != r15_end {
        wr::<u32>(rcx_item + 0x10, 0);
        let mut rax_ptr = rd::<usize>(rcx_item);
        let len_08 = rd::<u32>(rcx_item + 0x08) as usize;
        let rdi_end = rax_ptr.wrapping_add(len_08 * 4);
        if rax_ptr != rdi_end {
            let mut r8_sentinel = rd::<usize>(param_1 + 0x1128);
            while rax_ptr != rdi_end {
                let r10b = rd::<u8>(rax_ptr);
                let hash = ((r10b as u64) ^ 0xcbf2_9ce4_8422_2325u64)
                    .wrapping_mul(0x0000_0100_0000_01b3u64);
                let bucket = (rd::<u64>(param_1 + 0x1150) & hash) as usize;
                let buckets_base = rd::<usize>(param_1 + 0x1138);
                let bucket_addr = buckets_base.wrapping_add(bucket * 0x10);
                let mut r9_node = rd::<usize>(bucket_addr + 0x08);
                if r9_node == r8_sentinel {
                    r9_node = 0;
                } else {
                    let rdx_last = rd::<usize>(bucket_addr);
                    if r10b != rd::<u8>(r9_node + 0x10) {
                        loop {
                            if r9_node == rdx_last {
                                r8_sentinel = rd::<usize>(param_1 + 0x1128);
                                r9_node = 0;
                                break;
                            }
                            r9_node = rd::<usize>(r9_node + 0x08);
                            if r10b == rd::<u8>(r9_node + 0x10) {
                                r8_sentinel = rd::<usize>(param_1 + 0x1128);
                                break;
                            }
                        }
                    }
                }
                if r9_node == 0 {
                    r9_node = r8_sentinel;
                }
                if r9_node != r8_sentinel {
                    let w = rd::<u8>(r9_node + 0x11) as u32;
                    wr::<u32>(rcx_item + 0x10, rd::<u32>(rcx_item + 0x10).wrapping_add(w));
                    let w2 = rd::<u8>(r9_node + 0x11) as u32;
                    ebx_total = ebx_total.wrapping_add(w2);
                    r8_sentinel = rd::<usize>(param_1 + 0x1128);
                }
                rax_ptr = rax_ptr.wrapping_add(4);
            }
        }
        rcx_item = rcx_item.wrapping_add(0x78);
    }

    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm13_f = if rd::<u8>(param_1 + 0x1194) != 0 {
        remap_linear_rcp(
            xmm13_in,
            rd::<f32>(param_1 + 0x1198),
            rd::<f32>(param_1 + 0x119c),
            rd::<f32>(param_1 + 0x11a0),
            rd::<f32>(param_1 + 0x11a4),
            one,
        )
    } else {
        f32::from_bits(xmm13_in)
    };

    let xmm6_f = if rd::<u8>(param_1 + 0x116c) != 0 {
        remap_linear_rcp(
            xmm11_in,
            rd::<f32>(param_1 + 0x1170),
            rd::<f32>(param_1 + 0x1174),
            rd::<f32>(param_1 + 0x1178),
            rd::<f32>(param_1 + 0x117c),
            one,
        )
    } else {
        f32::from_bits(xmm11_in)
    };

    let total_f = ebx_total as f32;
    let xmm14 = if total_f == 0.0f32 {
        0.0f32
    } else {
        one / total_f
    };

    let mut rdi_grp = rd::<usize>(param_1 + 0x1110);
    let r12_grp_end = rdi_grp.wrapping_add((rd::<u32>(param_1 + 0x1118) as usize).wrapping_mul(0x78));
    while rdi_grp != r12_grp_end {
        let mode = rd::<i32>(rdi_grp + 0x70);
        let xmm8 = if mode == 0 {
            xmm13_f
        } else if mode == 1 {
            one - xmm13_f
        } else {
            one
        };
        let xmm12 = (rd::<u32>(rdi_grp + 0x10) as f32) * xmm14;
        let mut rbx_elem = rd::<usize>(rdi_grp + 0x50);
        let elem_cnt = rd::<u32>(rdi_grp + 0x58) as usize;
        let r15_elem_end = rbx_elem.wrapping_add(elem_cnt.wrapping_mul(0x28));
        while rbx_elem != r15_elem_end {
            let e1 = rd::<f32>(rbx_elem + 0x04);
            let e0 = rd::<f32>(rbx_elem);
            let inv_falloff = one / rd::<f32>(exe::ptr::<f32>(0x1_4387_18f8) as usize);
            let mut xmm2_w = if e0 > xmm6_f {
                one - (e0 - xmm6_f) * inv_falloff
            } else if xmm6_f > e1 {
                one - (xmm6_f - e1) * inv_falloff
            } else {
                one
            };
            if xmm2_w < 0.0f32 {
                xmm2_w = 0.0f32;
            }
            if xmm2_w > one {
                xmm2_w = one;
            }
            let val = (xmm2_w * xmm12) * xmm8;
            wr::<f32>(rbx_elem + 0x1c, val);
            wr::<f32>(rbx_elem + 0x24, val);

            let cur_len = rd::<u32>(param_2 + 0x08) as u64;
            let cur_cap = rd::<u32>(param_2 + 0x0c) as u64;
            if (cur_len << 3) == (cur_cap << 3) {
                exe::call_va!(0x1_401e_6d50, (param_2, cur_len.wrapping_add(1)));
            }
            let idx = rd::<u32>(param_2 + 0x08) as usize;
            let arr = rd::<usize>(param_2);
            wr::<usize>(arr + idx * 8, rbx_elem);
            wr::<u32>(param_2 + 0x08, (idx as u32).wrapping_add(1));

            rbx_elem = rbx_elem.wrapping_add(0x28);
        }
        rdi_grp = rdi_grp.wrapping_add(0x78);
    }

    let r8d_len1 = rd::<u32>(param_2 + 0x08) as u64;
    let rax_arr1 = rd::<usize>(param_2) as u64;
    let mut local_110: u64 = rax_arr1.wrapping_add(r8d_len1 * 8);
    let mut local_118: u64 = rax_arr1;
    let r9d_byte1 = (local_110 as u8) as u32;
    exe::call_va!(
        0x1_413e_4ad0,
        (
            (&mut local_118 as *mut u64) as usize,
            (&mut local_110 as *mut u64) as usize,
            r8d_len1,
            r9d_byte1
        )
    );

    let mut xmm4_sum1c: f32 = 0.0;
    let mut xmm3_sum24: f32 = 0.0;
    let mut r14d_cnt: u32 = 0;
    for ecx in 0..3u32 {
        let len = rd::<u32>(param_2 + 0x08);
        let arr = rd::<usize>(param_2);
        if ecx >= len {
            break;
        }
        let elem = rd::<usize>(arr + (ecx as usize) * 8);
        if elem != 0 {
            if ecx < 3 {
                xmm3_sum24 += rd::<f32>(elem + 0x24);
            }
            if ecx < 2 {
                xmm4_sum1c += rd::<f32>(elem + 0x1c);
                r14d_cnt = r14d_cnt.wrapping_add(1);
                wr::<f32>(
                    param_2 + 0x10,
                    rd::<f32>(elem + 0x08) + rd::<f32>(param_2 + 0x10),
                );
                wr::<f32>(
                    param_2 + 0x14,
                    rd::<f32>(elem + 0x0c) + rd::<f32>(param_2 + 0x14),
                );
            }
        }
    }

    let xmm5_inv1c = if xmm4_sum1c == 0.0f32 {
        0.0f32
    } else {
        one / xmm4_sum1c
    };
    let xmm2_inv24 = if xmm3_sum24 == 0.0f32 {
        0.0f32
    } else {
        one / xmm3_sum24
    };

    let r8_arr = rd::<usize>(param_2);
    let mut rax_it = r8_arr;
    let init_len = rd::<u32>(param_2 + 0x08) as usize;
    if rax_it < r8_arr.wrapping_add(init_len * 8) {
        loop {
            let rdx_elem = rd::<usize>(rax_it);
            let v1c = rd::<f32>(rdx_elem + 0x1c);
            let new_1c = if v1c > 0.0f32 && rax_it < r8_arr.wrapping_add(0x10) {
                v1c * xmm5_inv1c
            } else {
                0.0f32
            };
            wr::<f32>(rdx_elem + 0x1c, new_1c);

            let v24 = rd::<f32>(rdx_elem + 0x24);
            let new_24 = if v24 > 0.0f32 && rax_it < r8_arr.wrapping_add(0x18) {
                v24 * xmm2_inv24
            } else {
                0.0f32
            };
            wr::<f32>(rdx_elem + 0x24, new_24);

            let scale_1164 = rd::<f32>(param_1 + 0x1164);
            wr::<f32>(rdx_elem + 0x18, new_1c * scale_1164);
            wr::<f32>(rdx_elem + 0x20, new_24 * scale_1164);

            rax_it = rax_it.wrapping_add(8);
            let cur_len = rd::<u32>(param_2 + 0x08) as usize;
            if rax_it >= r8_arr.wrapping_add(cur_len * 8) {
                break;
            }
        }
    }

    let mut rdi_extra = param_1.wrapping_add(0x90);
    let extra_cnt = rd::<u64>(rdi_extra + 0x78) as usize;
    let rbx_extra_end = rdi_extra.wrapping_add(extra_cnt.wrapping_mul(0x28));
    if rdi_extra != rbx_extra_end {
        let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_64e4) as usize);
        while rdi_extra != rbx_extra_end {
            if rd::<u32>(rdi_extra + 0x10) != 0 && rd::<f32>(rdi_extra + 0x18) > thresh {
                let cur_len = rd::<u32>(param_2 + 0x08) as u64;
                let cur_cap = rd::<u32>(param_2 + 0x0c) as u64;
                if (cur_len << 3) == (cur_cap << 3) {
                    exe::call_va!(0x1_401e_6d50, (param_2, cur_len.wrapping_add(1)));
                }
                let idx = rd::<u32>(param_2 + 0x08) as usize;
                let arr = rd::<usize>(param_2);
                wr::<usize>(arr + idx * 8, rdi_extra);
                wr::<u32>(param_2 + 0x08, (idx as u32).wrapping_add(1));
                wr::<f32>(
                    param_2 + 0x10,
                    rd::<f32>(rdi_extra + 0x08) + rd::<f32>(param_2 + 0x10),
                );
                wr::<f32>(
                    param_2 + 0x14,
                    rd::<f32>(rdi_extra + 0x0c) + rd::<f32>(param_2 + 0x14),
                );
                r14d_cnt = r14d_cnt.wrapping_add(1);
            }
            rdi_extra = rdi_extra.wrapping_add(0x28);
        }
    }

    if r14d_cnt != 0 {
        let cnt_f = r14d_cnt as f32;
        wr::<f32>(param_2 + 0x10, rd::<f32>(param_2 + 0x10) / cnt_f);
        wr::<f32>(param_2 + 0x14, rd::<f32>(param_2 + 0x14) / cnt_f);
    }

    let r8d_len2 = rd::<u32>(param_2 + 0x08) as u64;
    let rax_arr2 = rd::<usize>(param_2) as u64;
    let mut local_20: u64 = rax_arr2.wrapping_add(r8d_len2 * 8);
    let mut local_28: u64 = rax_arr2;
    let r9d_byte2 = (local_110 as u8) as u32;
    exe::call_va!(
        0x1_413e_4dd0,
        (
            (&mut local_28 as *mut u64) as usize,
            (&mut local_20 as *mut u64) as usize,
            r8d_len2,
            r9d_byte2
        )
    );

    let new_lock_cnt = rd::<u32>(param_1 + 0x0c).wrapping_sub(1);
    wr::<u32>(param_1 + 0x0c, new_lock_cnt);
    if new_lock_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (param_1));
    }
}

/// `unknown_1424d4a00`
/// Original: 0x1424d4a00 (125 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1424d4a00() -> usize {
    let cur = rd::<usize>(exe::ptr::<usize>(0x1_43a4_6568) as usize);
    if cur != 0 {
        return cur;
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    let epoch = rd::<i32>(tls0 + 0x74);
    let guard_ptr = exe::ptr::<i32>(0x1_43d0_19f0) as usize;
    let obj_ptr = exe::ptr::<u8>(0x1_43d0_1920) as usize;
    if rd::<i32>(guard_ptr) > epoch {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            exe::call_va!(0x1_424d_48e0, (obj_ptr));
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(0x1_4264_2470) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
            return obj_ptr;
        }
    }
    obj_ptr
}

/// `anonymous_namespace_1424dfb90`
/// Original: 0x1424dfb90 (428 bytes, Throws::Discard)
pub unsafe extern "win64" fn anonymous_namespace_1424dfb90(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
) -> u32 {
    let [xmm0, xmm1, xmm2, xmm3] = crate::simd_ops::take_xmm_args();
    let idx_4c = rd::<i32>(param_1 + 0x4c);
    if idx_4c <= -1 {
        return u32::MAX;
    }

    let idx_5c = rd::<i32>(param_1 + 0x5c) as isize;
    let rdi = (exe::ptr::<u8>(0x1_43c4_7ce0) as usize as isize + idx_5c * 0x70) as usize;
    let table_ptr = rd::<usize>(rdi);
    let esi = rd::<u32>((table_ptr as isize + (idx_4c as isize) * 4) as usize);
    let elem_addr = rd::<usize>(param_1).wrapping_add((esi as usize) * 0x60);
    let xmm6 = rd::<f32>(elem_addr + 0x40);

    let (ret_rax, _) = crate::simd_ops::call_va8_xmm(
        0x1_424d_f170,
        param_1 as u64,
        param_2 as u64,
        esi as u64,
        0,
        0,
        0,
        0,
        0,
        [xmm0, xmm1, xmm2, xmm3],
    );
    let rdx = ret_rax as usize;

    let r8d = rd::<u32>(param_1 + 0x50);
    wr::<u32>(param_1 + 0x30, 0x3f80_0000);
    let eax_cap = rd::<u32>(rdi + 0x34);
    let new_40 = rd::<f32>(rdx + 0x40);
    let cur_44 = rd::<f32>(param_1 + 0x44);

    let xmm5 = if r8d > eax_cap {
        let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let xmm4 = eax_cap as f32;
        let xmm3_sub = cur_44 - (xmm6 / xmm4);
        let xmm0_inv = one / xmm4;
        xmm3_sub + (xmm0_inv * new_40)
    } else if r8d < 5 {
        let c5 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d48) as usize);
        let c6 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d6c) as usize);
        (cur_44 * c5 + new_40) / c6
    } else {
        let r8_f = r8d as f32;
        let r8_p1_f = r8d.wrapping_add(1) as f32;
        (r8_f * cur_44 + new_40) / r8_p1_f
    };
    wr::<f32>(param_1 + 0x44, xmm5);

    if r8d > rd::<u32>(rdi + 0x34) {
        let diff = xmm6 - rd::<f32>(rdx + 0x40);
        let abs_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8720) as usize);
        let abs_diff = f32::from_bits(diff.to_bits() & abs_mask);
        let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4387_2868) as usize);
        let cur_48 = rd::<f32>(param_1 + 0x48);
        let step = rd::<f32>(exe::ptr::<f32>(0x1_4387_28a0) as usize);
        let next_48 = if thresh > abs_diff {
            cur_48 + step
        } else {
            cur_48 - step
        };
        let lo = rd::<f32>(exe::ptr::<f32>(0x1_4342_708c) as usize);
        let hi = rd::<f32>(exe::ptr::<f32>(0x1_4342_6960) as usize);
        let max_lo = if next_48 > lo { next_48 } else { lo };
        let clamped_48 = if max_lo < hi { max_lo } else { hi };
        wr::<f32>(param_1 + 0x48, clamped_48);
    } else if rd::<u8>(param_1 + 0x61) != 0 {
        wr::<u32>(
            param_1 + 0x30,
            rd::<u32>(exe::ptr::<u32>(0x1_4387_2908) as usize),
        );
    }

    let last_idx = rd::<u32>(rdi + 0x34).wrapping_sub(1) as usize;
    let last_elem = rd::<usize>(param_1).wrapping_add(last_idx * 0x60);
    let dl = (!rd::<u8>(last_elem + 0x51)) & 1;
    wr::<u32>(param_1 + 0x50, r8d.wrapping_add(1));
    wr::<u64>(param_1 + 0x28, xmm2 as u64);
    wr::<u8>(param_1 + 0x62, dl);
    esi
}

/// `anonymous_namespace_1424e0030`
/// Original: 0x1424e0030 (1171 bytes, Throws::Discard)
pub unsafe extern "win64" fn anonymous_namespace_1424e0030(param_1: usize) {
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let rbp_base = exe::ptr::<u8>(0x1_43c4_7ce0) as usize;
    let idx_8c = rd::<i32>(param_1 + 0x8c) as isize;
    let tbl_entry = (rbp_base as isize + idx_8c * 0x70) as usize;

    let mut xmm3: f32 = 0.0;
    let mut xmm5: f32 = 0.0;
    let mut xmm6: f32 = 0.0;

    let eax_30 = rd::<u32>(tbl_entry + 0x30);
    let xmm1_cnt = eax_30 as f32;
    let mul_2c = rd::<u32>(tbl_entry + 0x2c);

    let c_03 = rd::<f32>(exe::ptr::<f32>(0x1_4342_66b8) as usize);
    let rbx_i64 = crate::simd_ops::cvt_f32_i64(xmm1_cnt * c_03) as u32;
    let ebx = rbx_i64.wrapping_mul(mul_2c);

    let c_06 = rd::<f32>(exe::ptr::<f32>(0x1_4342_67cc) as usize);
    let r11_i64 = crate::simd_ops::cvt_f32_i64(xmm1_cnt * c_06) as u32;
    let r11d = r11_i64.wrapping_mul(mul_2c);

    wr::<u64>(param_1 + 0x13c, 0);
    let xmm1_thresh = rd::<f32>(exe::ptr::<f32>(0x1_4387_29a8) as usize);
    let mut xmm2: f32 = 0.0;

    for i in 0..ebx {
        let rdx = rd::<usize>(param_1 + 0x30);
        let elem = rdx.wrapping_add((i as usize) * 0x60);
        if (rd::<u8>(elem + 0x51) & 1) != 0 && xmm1_thresh > rd::<f32>(elem + 0x40) {
            xmm3 += one;
        }
        xmm2 += rd::<f32>(elem + 0x14);
        wr::<f32>(param_1 + 0x13c, xmm2);
    }

    if ebx < r11d {
        let r9 = rd::<usize>(param_1 + 0x30);
        for i in ebx..r11d {
            let elem = r9.wrapping_add((i as usize) * 0x60);
            if (rd::<u8>(elem + 0x51) & 1) != 0 && xmm1_thresh > rd::<f32>(elem + 0x40) {
                xmm5 += one;
            }
        }
    }

    if r11d < rd::<u32>(param_1 + 0x38) {
        let mut xmm2_hi = rd::<f32>(param_1 + 0x140);
        let mut edx = r11d;
        loop {
            let rax = rd::<usize>(param_1 + 0x30);
            let elem = rax.wrapping_add((edx as usize) * 0x60);
            if (rd::<u8>(elem + 0x51) & 1) != 0 && xmm1_thresh > rd::<f32>(elem + 0x40) {
                xmm6 += one;
            }
            xmm2_hi += rd::<f32>(elem + 0x14);
            edx = edx.wrapping_add(1);
            wr::<f32>(param_1 + 0x140, xmm2_hi);
            if edx >= rd::<u32>(param_1 + 0x38) {
                break;
            }
        }
    }

    let f_13c = rd::<f32>(param_1 + 0x13c);
    let ebx_f = ebx as f32;
    let new_13c = f_13c / ebx_f;
    let f_140 = rd::<f32>(param_1 + 0x140);
    wr::<f32>(param_1 + 0x13c, new_13c);

    let cap_34 = rd::<u32>(tbl_entry + 0x34);
    let rem_hi_f = cap_34.wrapping_sub(r11d) as f32;
    wr::<f32>(param_1 + 0x140, f_140 / rem_hi_f);

    xmm3 /= ebx_f;
    let mid_cnt_f = r11d.wrapping_sub(ebx) as f32;
    let xmm2_mid = xmm5 / mid_cnt_f;
    wr::<f32>(param_1 + 0x134, xmm2_mid);
    let sum_3_2 = xmm3 + xmm2_mid;
    wr::<f32>(param_1 + 0x130, xmm3);

    let rem_hi_f2 = rd::<u32>(tbl_entry + 0x34).wrapping_sub(r11d) as f32;
    let xmm1_hi_ratio = xmm6 / rem_hi_f2;
    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let xmm0_avg = sum_3_2 * c_half;
    wr::<f32>(param_1 + 0x138, xmm1_hi_ratio);
    wr::<f32>(param_1 + 0x144, xmm0_avg);

    let x0_9f8 = rd::<f32>(exe::ptr::<f32>(0x1_4387_29f8) as usize);
    let xmm6_remapped = remap_linear_rcp(xmm0_avg.to_bits(), x0_9f8, one, 0.0f32, one, one);
    wr::<f32>(param_1 + 0x144, xmm6_remapped);

    let str_listener = exe::ptr::<u8>(0x1_433d_6d80) as usize;
    let id = exe::call_va!(0x1_4158_1080, (str_listener) -> u32);
    let c_100 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ea4) as usize);
    let rtpc_val = xmm6_remapped * c_100;
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4158_9810,
        id as u64,
        0,
        u64::MAX,
        0,
        4,
        0,
        0,
        0,
        [0, rtpc_val.to_bits() as u128, 0, 0],
    );
}

/// `anonymous_namespace_1424e08e0`
/// Original: 0x1424e08e0 (211 bytes, Throws::No)
pub unsafe extern "win64" fn anonymous_namespace_1424e08e0(
    param_1: usize,
    param_2: usize,
    param_3: i32,
) {
    let rdi = (rd::<usize>(param_2) as isize + (param_3 as isize) * 0x60) as usize;
    if (rd::<u8>(rdi + 0x51) & 1) == 0 {
        return;
    }

    let rbx = param_1.wrapping_add(0x100);
    let edx_len = rd::<u32>(rbx + 0x08) as u64;
    let eax_cap = rd::<u32>(rbx + 0x0c) as u64;
    if edx_len.wrapping_mul(0x50) == eax_cap.wrapping_mul(0x50) {
        exe::call_va!(0x1_413f_c7f0, (rbx, edx_len.wrapping_add(1)));
    }

    let idx0 = rd::<u32>(rbx + 0x08) as usize;
    let zero_dst = rd::<usize>(rbx).wrapping_add(idx0 * 0x50);
    wr::<[u8; 0x50]>(zero_dst, [0u8; 0x50]);

    let new_len = rd::<u32>(rbx + 0x08).wrapping_add(1);
    wr::<u32>(rbx + 0x08, new_len);
    let dst = rd::<usize>(rbx)
        .wrapping_add((new_len as usize) * 0x50)
        .wrapping_sub(0x50);
    wr::<usize>(dst, rdi);
    wr::<u32>(dst + 0x40, rd::<u32>(param_2 + 0x44));

    let xmm0 = rd::<u128>(rdi);
    let xmm1 = crate::simd_ops::vaddps(xmm0, rd::<u128>(rdi + 0x10));
    let half_vec = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let xmm2 = crate::simd_ops::vmulps(xmm1, half_vec);
    wr::<u128>(dst + 0x10, xmm2);
    wr::<u128>(dst + 0x20, rd::<u128>(rdi + 0x30));
    wr::<u128>(dst + 0x30, rd::<u128>(rdi + 0x20));
}

// =============================================================================
// Differential tests
// =============================================================================

struct GlobalSave {
    addr: usize,
    bytes: Vec<u8>,
}

impl GlobalSave {
    unsafe fn new(va: u64, len: usize) -> Self {
        let addr = exe::ptr::<u8>(va) as usize;
        let mut bytes = vec![0u8; len];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = rd::<u8>(addr + i);
        }
        Self { addr, bytes }
    }
}

impl Drop for GlobalSave {
    fn drop(&mut self) {
        unsafe {
            for (i, &b) in self.bytes.iter().enumerate() {
                wr::<u8>(self.addr + i, b);
            }
        }
    }
}

pub unsafe fn diff_cityhash_1424b2600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9b50(
        this: usize,
        view: usize,
        pos: u64,
        count: u64,
    ) -> usize {
        let src = rd::<usize>(view).wrapping_add(pos as usize);
        let n = count as usize;
        for i in 0..n {
            wr::<u8>(this + 0x08 + i, rd::<u8>(src + i));
        }
        wr::<u8>(this + 0x08 + n, 0);
        wr::<u32>(this + 0x04, n as u32);
        this
    }
    unsafe extern "win64" fn mock_ab10(_this: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let sample = b"data/shaders/build/pc_dx12/UberShader_Mesh_Forward_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let lengths = [0usize, 2, 5, 12, 24, 48, 80];

    let run = |f: F, len: usize| -> u64 {
        let view: [u64; 2] = [sample.as_ptr() as usize as u64, len as u64];
        f(view.as_ptr() as usize)
    };

    for &len in &lengths {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1424b2600 len={}", len);
    }
    Ok(lengths.len() as u32)
}

pub unsafe fn diff_r_1424b87a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut IS_INF: u8 = 0;

    unsafe extern "win64" fn mock_8bd0(_aabb: usize) -> u8 {
        IS_INF
    }

    let _m1 = crate::difftest::Mock::new(0x1_4058_8bd0, mock_8bd0 as *const ())?;

    let run = |f: F, mode: u8| -> [u32; 6] {
        IS_INF = u8::from(mode == 1);
        let mut aabb: [f32; 6] = if mode == 0 {
            [5.0, 6.0, 7.0, 1.0, 2.0, 3.0]
        } else {
            [-1.0, -2.0, -3.0, 4.0, 5.0, 6.0]
        };
        let mat: [f32; 16] = [
            2.0, -0.5, 1.0, 0.0,
            0.25, 3.0, -1.5, 0.0,
            -1.0, 0.5, 2.5, 0.0,
            10.0, 20.0, 30.0, 1.0,
        ];
        f(aabb.as_mut_ptr() as usize, mat.as_ptr() as usize);
        core::array::from_fn(|i| aabb[i].to_bits())
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1424b87a0 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1424ce980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: (bool, usize, u64, u64, u32, u32, u32) = (false, 0, 0, 0, 0, 0, 0);
    static mut P1_BUF: [u8; 0x11b0] = [0; 0x11b0];

    unsafe extern "win64" fn mock_e1a0(
        p1: usize,
        p2: usize,
        r8_pair: usize,
        _r9: usize,
        arg5: u32,
    ) {
        let [_, xmm1, _, xmm3] = crate::simd_ops::take_xmm_args();
        let expected_p1 = P1_BUF.as_ptr() as usize;
        CALL_LOG = (
            p1 == expected_p1,
            p2,
            rd::<u64>(r8_pair),
            rd::<u64>(r8_pair + 0x08),
            xmm1 as u32,
            xmm3 as u32,
            arg5,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_424c_e1a0, mock_e1a0 as *const ())?;

    let run = |f: F, same_stamp: bool| -> ((bool, usize, u64, u64, u32, u32, u32), u64) {
        CALL_LOG = (false, 0, 0, 0, 0, 0, 0);
        P1_BUF = [0; 0x11b0];
        let p1p = P1_BUF.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x11a8, 0x7788_99aa);

        let mut p3 = [0u8; 0x28];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p, 0x1234_5678);
        wr::<u32>(p3p + 0x08, 7);
        wr::<f32>(p3p + 0x10, 1.5);
        wr::<f32>(p3p + 0x14, 2.5);
        wr::<u64>(p3p + 0x20, if same_stamp { 0x7788_99aa } else { 0x1111 });

        f(p1p, 0x42, p3p);
        (CALL_LOG, rd::<u64>(p3p + 0x20))
    };

    for same_stamp in [false, true] {
        let o = run(f_o, same_stamp);
        let n = run(f_n, same_stamp);
        crate::diff_eq!(o, n, "1424ce980 same_stamp={}", same_stamp);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1424cea10(orig: usize, new: usize) -> Result<u32, String> {
    static mut P1_STATE: [u8; 0x11c0] = [0; 0x11c0];
    static mut P2_STATE: [u8; 0x20] = [0; 0x20];
    static mut GRP_BUF: [u8; 0x78] = [0; 0x78];
    static mut ELEMS_BUF: [u8; 0x28 * 3] = [0; 0x28 * 3];
    static mut KEYS_BUF: [u32; 2] = [0; 2];
    static mut BUCKETS_BUF: [usize; 4] = [0; 4];
    static mut SENTINEL_NODE: [u8; 0x18] = [0; 0x18];
    static mut VALID_NODE: [u8; 0x18] = [0; 0x18];
    static mut PTR_ARR: [usize; 8] = [0; 8];
    static mut SORT_CALLS: (u64, u64) = (0, 0);

    unsafe extern "win64" fn mock_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_lock(_lock: usize) {}
    unsafe extern "win64" fn mock_6d50(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_4ad0(_first: usize, _last: usize, cnt: u64, _b: u32) {
        SORT_CALLS.0 = cnt;
    }
    unsafe extern "win64" fn mock_4dd0(_first: usize, _last: usize, cnt: u64, _b: u32) {
        SORT_CALLS.1 = cnt;
    }

    let _im1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_lock as *const ())?;
    let _im3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_lock as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_401e_6d50, mock_6d50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413e_4ad0, mock_4ad0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413e_4dd0, mock_4dd0 as *const ())?;
    let _g1 = GlobalSave::new(0x1_4387_18f8, 4);

    let run = |fn_addr: usize, remap_flags: u8| -> (u32, u32, u32, (u64, u64), Vec<u32>, [usize; 8]) {
        P1_STATE = [0; 0x11c0];
        P2_STATE = [0; 0x20];
        GRP_BUF = [0; 0x78];
        ELEMS_BUF = [0; 0x28 * 3];
        KEYS_BUF = [5, 9];
        BUCKETS_BUF = [0; 4];
        SENTINEL_NODE = [0; 0x18];
        VALID_NODE = [0; 0x18];
        PTR_ARR = [0; 8];
        SORT_CALLS = (0, 0);

        wr::<f32>(exe::ptr::<f32>(0x1_4387_18f8) as usize, 4.0);

        let p1p = P1_STATE.as_mut_ptr() as usize;
        let p2p = P2_STATE.as_mut_ptr() as usize;
        let grpp = GRP_BUF.as_mut_ptr() as usize;
        let elp = ELEMS_BUF.as_mut_ptr() as usize;
        let sentp = SENTINEL_NODE.as_mut_ptr() as usize;
        let valp = VALID_NODE.as_mut_ptr() as usize;
        let bktp = BUCKETS_BUF.as_mut_ptr() as usize;
        let arrp = PTR_ARR.as_mut_ptr() as usize;

        wr::<u8>(valp + 0x10, 5);
        wr::<u8>(valp + 0x11, 4);
        wr::<usize>(valp + 0x08, sentp);

        for b in 0..2 {
            wr::<usize>(bktp + b * 0x10, valp);
            wr::<usize>(bktp + b * 0x10 + 0x08, valp);
        }

        for i in 0..3 {
            let e = elp + i * 0x28;
            wr::<f32>(e, 0.2 * (i as f32));
            wr::<f32>(e + 0x04, 0.8);
            wr::<f32>(e + 0x08, 10.0 + (i as f32));
            wr::<f32>(e + 0x0c, 20.0 + (i as f32));
        }

        wr::<usize>(grpp, KEYS_BUF.as_ptr() as usize);
        wr::<u32>(grpp + 0x08, 2);
        wr::<usize>(grpp + 0x50, elp);
        wr::<u32>(grpp + 0x58, 3);
        wr::<i32>(grpp + 0x70, i32::from(remap_flags));

        wr::<u32>(p1p + 0x08, u32::MAX);
        wr::<u32>(p1p + 0x90 + 0x10, 1);
        wr::<f32>(p1p + 0x90 + 0x08, 5.0);
        wr::<f32>(p1p + 0x90 + 0x0c, 6.0);
        wr::<f32>(p1p + 0x90 + 0x18, 0.5);
        wr::<u64>(p1p + 0x90 + 0x78, 1);

        wr::<usize>(p1p + 0x1110, grpp);
        wr::<u32>(p1p + 0x1118, 1);
        wr::<usize>(p1p + 0x1128, sentp);
        wr::<usize>(p1p + 0x1138, bktp);
        wr::<u64>(p1p + 0x1150, 1);
        wr::<f32>(p1p + 0x1164, 2.0);
        wr::<u8>(p1p + 0x116c, remap_flags);
        wr::<f32>(p1p + 0x1170, 0.0);
        wr::<f32>(p1p + 0x1174, 2.0);
        wr::<f32>(p1p + 0x1178, 0.1);
        wr::<f32>(p1p + 0x117c, 0.9);
        wr::<u8>(p1p + 0x1194, remap_flags);
        wr::<f32>(p1p + 0x1198, 0.0);
        wr::<f32>(p1p + 0x119c, 2.0);
        wr::<f32>(p1p + 0x11a0, 0.2);
        wr::<f32>(p1p + 0x11a4, 0.8);

        wr::<usize>(p2p, arrp);
        wr::<u32>(p2p + 0x0c, 8);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            p2p as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0.5f32.to_bits() as u128, 0.75f32.to_bits() as u128],
        );

        let mut elem_words = Vec::with_capacity(30);
        for i in 0..30 {
            elem_words.push(rd::<u32>(elp + i * 4));
        }
        (
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x10),
            rd::<u32>(p2p + 0x14),
            SORT_CALLS,
            elem_words,
            PTR_ARR,
        )
    };

    for remap_flags in [0u8, 1u8] {
        let o = run(orig, remap_flags);
        let n = run(new, remap_flags);
        crate::diff_eq!(o, n, "1424cea10 remap_flags={}", remap_flags);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1424d4a00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a4_6568, 8);
    let _g2 = GlobalSave::new(0x1_43d0_19f0, 4);

    static mut INIT_LOG: u32 = 0;

    unsafe extern "win64" fn mock_4a78(_g: usize) {
        INIT_LOG += 1;
    }
    unsafe extern "win64" fn mock_48e0(_obj: usize) {
        INIT_LOG += 10;
    }
    unsafe extern "win64" fn mock_4d18(_dtor: usize) {
        INIT_LOG += 100;
    }
    unsafe extern "win64" fn mock_4a0c(g: usize) {
        wr::<i32>(g, i32::MIN);
        INIT_LOG += 1000;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4183_4a78, mock_4a78 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424d_48e0, mock_48e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4183_4d18, mock_4d18 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_4a0c as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut fake_tls = [0u8; 0x100];
    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);

    let mut run = |f: F, mode: u8| -> (usize, u32) {
        INIT_LOG = 0;
        let tp = fake_tls.as_mut_ptr() as usize;
        wr::<i32>(tp + 0x74, 0);
        let ptr_568 = exe::ptr::<usize>(0x1_43a4_6568) as usize;
        let guard_9f0 = exe::ptr::<i32>(0x1_43d0_19f0) as usize;
        match mode {
            0 => {
                wr::<usize>(ptr_568, 0x1234_5678);
                wr::<i32>(guard_9f0, i32::MIN);
            }
            1 => {
                wr::<usize>(ptr_568, 0);
                wr::<i32>(guard_9f0, i32::MIN);
            }
            _ => {
                wr::<usize>(ptr_568, 0);
                wr::<i32>(guard_9f0, -1);
            }
        }
        (f(), INIT_LOG)
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            wr::<usize>(tls_slot, saved_tls0);
        }
        crate::diff_eq!(o, n, "1424d4a00 mode={}", mode);
    }
    wr::<usize>(tls_slot, saved_tls0);
    Ok(3)
}

pub unsafe fn diff_anonymous_namespace_1424dfb90(orig: usize, new: usize) -> Result<u32, String> {
    let _g1 = GlobalSave::new(0x1_43c4_7ce0, 0x70);
    let _g2 = GlobalSave::new(0x1_4387_2868, 4);
    let _g3 = GlobalSave::new(0x1_4387_28a0, 4);
    let _g4 = GlobalSave::new(0x1_4387_2908, 4);

    static mut NEW_ELEM: [u8; 0x60] = [0; 0x60];

    unsafe extern "win64" fn mock_f170(_p1: usize, _p2: usize, _idx: u32) -> usize {
        NEW_ELEM.as_mut_ptr() as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_424d_f170, mock_f170 as *const ())?;

    let run = |fn_addr: usize, mode: u8| -> (u32, u32, u32, u32, u32, u64, u8) {
        NEW_ELEM = [0; 0x60];
        wr::<f32>(NEW_ELEM.as_mut_ptr() as usize + 0x40, 12.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_2868) as usize, 5.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_28a0) as usize, 0.25);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_2908) as usize, 0.75);

        let idx_map: [u32; 4] = [1, 0, 2, 1];
        let mut elems = [0u8; 0x60 * 3];
        let elp = elems.as_mut_ptr() as usize;
        wr::<f32>(elp + 0x60 + 0x40, 6.0);
        wr::<u8>(elp + 2 * 0x60 + 0x51, 0);

        let tbl_ptr = exe::ptr::<u8>(0x1_43c4_7ce0) as usize;
        wr::<usize>(tbl_ptr, idx_map.as_ptr() as usize);
        wr::<u32>(tbl_ptr + 0x34, 3);

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, elp);
        wr::<f32>(p1p + 0x44, 4.0);
        wr::<f32>(p1p + 0x48, 0.5);
        wr::<i32>(p1p + 0x4c, if mode == 0 { -1 } else { 0 });
        wr::<u32>(
            p1p + 0x50,
            match mode {
                1 => 2,
                2 => 3,
                _ => 6,
            },
        );
        wr::<i32>(p1p + 0x5c, 0);
        wr::<u8>(p1p + 0x61, 1);

        let (ret, _) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0x99,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0xdead_beef_cafe_babeu64 as u128, 0],
        );

        (
            ret as u32,
            rd::<u32>(p1p + 0x30),
            rd::<u32>(p1p + 0x44),
            rd::<u32>(p1p + 0x48),
            rd::<u32>(p1p + 0x50),
            rd::<u64>(p1p + 0x28),
            rd::<u8>(p1p + 0x62),
        )
    };

    for mode in 0..4u8 {
        let o = run(orig, mode);
        let n = run(new, mode);
        crate::diff_eq!(o, n, "1424dfb90 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_anonymous_namespace_1424e0030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43c4_7ce0, 0x70);
    let _g2 = GlobalSave::new(0x1_4387_29a8, 4);
    let _g3 = GlobalSave::new(0x1_4387_29f8, 4);

    static mut RTPC_LOG: (u32, u32, u64, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_get_id(_s: usize) -> u32 {
        0x7777
    }
    unsafe extern "win64" fn mock_set_rtpc(
        id: u32,
        _rdx: usize,
        obj_id: u64,
        _r9: usize,
        interp: u32,
    ) {
        let [_, xmm1, _, _] = crate::simd_ops::take_xmm_args();
        RTPC_LOG = (id, xmm1 as u32, obj_id, interp);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4158_1080, mock_get_id as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4158_9810, mock_set_rtpc as *const ())?;

    let run = |f: F| -> ([u32; 6], (u32, u32, u64, u32)) {
        RTPC_LOG = (0, 0, 0, 0);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_29a8) as usize, 10.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_29f8) as usize, 0.2);

        let tbl_ptr = exe::ptr::<u8>(0x1_43c4_7ce0) as usize;
        wr::<u32>(tbl_ptr + 0x2c, 2);
        wr::<u32>(tbl_ptr + 0x30, 10); // 10*0.3*2 = 6, 10*0.6*2 = 12
        wr::<u32>(tbl_ptr + 0x34, 16);

        let mut elems = [0u8; 0x60 * 16];
        let elp = elems.as_mut_ptr() as usize;
        for i in 0..16 {
            let e = elp + i * 0x60;
            wr::<f32>(e + 0x14, (i as f32) + 1.0);
            wr::<f32>(e + 0x40, if (i & 1) == 0 { 5.0 } else { 15.0 });
            wr::<u8>(e + 0x51, 1);
        }

        let mut p1 = [0u8; 0x150];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x30, elp);
        wr::<u32>(p1p + 0x38, 16);
        wr::<i32>(p1p + 0x8c, 0);

        f(p1p);
        let out_fields = [
            rd::<u32>(p1p + 0x130),
            rd::<u32>(p1p + 0x134),
            rd::<u32>(p1p + 0x138),
            rd::<u32>(p1p + 0x13c),
            rd::<u32>(p1p + 0x140),
            rd::<u32>(p1p + 0x144),
        ];
        (out_fields, RTPC_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1424e0030");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_1424e08e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut P1_BUF: [u8; 0x120] = [0; 0x120];
    static mut P2_BUF: [u8; 0x50] = [0; 0x50];
    static mut ELEMS_BUF: [u8; 0x60 * 2] = [0; 0x60 * 2];
    static mut OUT_ARR: [u8; 0x50 * 2] = [0; 0x50 * 2];

    unsafe extern "win64" fn mock_c7f0(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413f_c7f0, mock_c7f0 as *const ())?;

    let run = |f: F, active: u8| -> (u32, u32, [u8; 0x50]) {
        P1_BUF = [0; 0x120];
        P2_BUF = [0; 0x50];
        ELEMS_BUF = [0; 0x60 * 2];
        OUT_ARR = [0xff; 0x50 * 2];

        let p1p = P1_BUF.as_mut_ptr() as usize;
        let p2p = P2_BUF.as_mut_ptr() as usize;
        let elp = ELEMS_BUF.as_mut_ptr() as usize;
        let outp = OUT_ARR.as_mut_ptr() as usize;

        let e1 = elp + 0x60;
        wr::<[f32; 4]>(e1, [2.0, 4.0, 6.0, 8.0]);
        wr::<[f32; 4]>(e1 + 0x10, [10.0, 12.0, 14.0, 16.0]);
        wr::<[u32; 4]>(e1 + 0x20, [0x11, 0x22, 0x33, 0x44]);
        wr::<[u32; 4]>(e1 + 0x30, [0x55, 0x66, 0x77, 0x88]);
        wr::<u8>(e1 + 0x51, active);

        wr::<usize>(p2p, elp);
        wr::<u32>(p2p + 0x44, 0x3f80_0000);

        wr::<usize>(p1p + 0x100, outp);
        wr::<u32>(p1p + 0x108, 0);
        wr::<u32>(p1p + 0x10c, 0);

        f(p1p, p2p, 1);
        let mut first_slot = [0u8; 0x50];
        first_slot.copy_from_slice(&OUT_ARR[..0x50]);
        (
            rd::<u32>(p1p + 0x108),
            rd::<u32>(p1p + 0x10c),
            first_slot,
        )
    };

    for active in [0u8, 1u8] {
        let o = run(f_o, active);
        let n = run(f_n, active);
        crate::diff_eq!(o, n, "1424e08e0 active={}", active);
    }
    Ok(2)
}
