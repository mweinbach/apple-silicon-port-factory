//! Decompiled hot functions shard X5 (`0x1420cba20..0x142222300`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vblendps, vblendvps, vcmpps, vcvtph2ps, vhaddps, vmaxps, vminps, vmovmskps, vmulps,
    vpcmpeqd, vpermilps, vshufps, vpslldq, vpsrldq, vsubps,
};

#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Align16([u8; 16]);

#[inline(always)]
unsafe fn vrsqrtps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::{__m128, _mm_rsqrt_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rsqrt_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrsqrtps(a)
    }
}

#[inline(always)]
fn vfmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4u32 {
        let fa = f32::from_bits((a >> (i * 32)) as u32);
        let fb = f32::from_bits((b >> (i * 32)) as u32);
        let fc = f32::from_bits((acc >> (i * 32)) as u32);
        let f = fa.mul_add(fb, fc);
        r |= (f.to_bits() as u128) << (i * 32);
    }
    r
}

#[inline(always)]
unsafe fn call_expf(x: f32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad58,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [x.to_bits() as u128, 0, 0, 0],
    );
    f32::from_bits(x0 as u32)
}

#[inline(always)]
unsafe fn call_powf(x: f32, y: f32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad46,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [x.to_bits() as u128, y.to_bits() as u128, 0, 0],
    );
    f32::from_bits(x0 as u32)
}

/// `unknown_1420cba20`
/// Original: 0x1420cba20 (174 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420cba20(param_1: usize, param_2: u64) -> u8 {
    let reg_count = rd::<u32>(param_1 + 0x04) as usize;
    let mut inline_regs = [Align16([0; 16]); 64];
    let mut heap_regs: Vec<Align16>;
    let r14 = if reg_count <= 64 {
        inline_regs.as_mut_ptr() as usize
    } else {
        heap_regs = vec![Align16([0; 16]); reg_count];
        heap_regs.as_mut_ptr() as usize
    };

    let mut rbx = rd::<usize>(param_1 + 0x18);
    let insn_count = rd::<u32>(param_1 + 0x0c) as usize;
    let r15 = rd::<u64>(param_1 + 0x20);
    let rdi = rbx.wrapping_add(insn_count.wrapping_mul(0x0e));
    if rbx != rdi {
        let r12 = exe::ptr::<u64>(0x1_4298_0680) as usize;
        while rbx != rdi {
            let opcode = rd::<u8>(rbx + 0x0d) as usize;
            let fn_va = rd::<u64>(r12.wrapping_add(opcode * 8));
            let step = exe::call_va!(fn_va, (rbx, r15, r14, param_2) -> i64);
            rbx = rbx.wrapping_add((step as usize).wrapping_mul(0x0e));
        }
    }
    1
}

/// `unknown_1420e3ef0`
/// Original: 0x1420e3ef0 (558 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420e3ef0(
    param_1: usize,
    param_2: usize,
    param_3: u16,
) -> usize {
    let mut buf_50 = [0u64; 0x910 / 8];
    let base_50 = buf_50.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_4183_acc2, (base_50, 0u32, 0x910u64) -> usize);
    wr::<usize>(base_50, base_50 + 0x10);
    wr::<u32>(base_50 + 0x08, 0);
    wr::<u32>(base_50 + 0x0c, 0x10);
    exe::call_va!(0x1_4218_3fb0, (rd::<u64>(param_2), base_50));

    let entries_base = rd::<usize>(param_2 + 0x4fe0);
    let entries_cnt = rd::<u32>(param_2 + 0x4fe8) as usize;
    let entries_end = entries_base.wrapping_add(entries_cnt.wrapping_mul(0x78));
    let mut cur = entries_base;

    while cur != entries_end {
        let matched = exe::call_va!(0x1_420e_3850, (cur, param_3) -> u8);
        if matched != 0 {
            if rd::<u64>(cur + 0x48) != 0 || rd::<u64>(cur + 0x50) != 0 {
                wr::<u64>(param_1, u64::MAX);
                wr::<u64>(param_1 + 0x08, 0);
                wr::<u64>(param_1 + 0x10, 1);
                wr::<u8>(param_1 + 0x18, 1);
                exe::call_va!(0x1_411f_5db0, (base_50));
                return param_1;
            }
            let r8 = rd::<usize>(cur + 0x60);
            if r8 != 0 {
                let count = rd::<u32>(base_50 + 0x08) as usize;
                let mut rcx = rd::<usize>(base_50);
                let r10 = rcx.wrapping_add(count.wrapping_mul(0x90));
                let rdx = rd::<u64>(r8 + 0x320);
                let r9 = (rd::<u8>(r8 + 0x328) as i8) as i64;
                let mut found_val: u64 = 0;

                while rcx != r10 {
                    let kind = (rd::<u8>(rcx + 0x18) as i8) as i64;
                    if kind == r9 {
                        let ok = if r9 == -1 || r9 == 0 {
                            true
                        } else if r9 == 1 {
                            rd::<u64>(rcx + 0x10) == rdx
                        } else {
                            rd::<u32>(rcx + 0x10) == (rdx as u32)
                        };
                        if ok {
                            found_val = rd::<u64>(rcx);
                            break;
                        }
                    }
                    rcx = rcx.wrapping_add(0x90);
                }

                wr::<u64>(param_1, rd::<u64>(r8));
                wr::<u64>(param_1 + 0x08, found_val);
                wr::<u64>(param_1 + 0x10, 0);
                wr::<u8>(param_1 + 0x18, 1);
                exe::call_va!(0x1_411f_5db0, (base_50));
                return param_1;
            }
        }
        cur = cur.wrapping_add(0x78);
    }

    wr::<u32>(param_1, 9);
    wr::<u8>(param_1 + 0x18, 0);
    exe::call_va!(0x1_411f_5db0, (base_50));
    param_1
}

/// `unknown_1421377d0`
/// Original: 0x1421377d0 (1268 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_1421377d0(
    param_1: usize,
    param_2: usize,
    param_3: f32,
    param_4: u8,
    param_5: u8,
    param_6: usize,
    param_7: usize,
) -> usize {
    unknown_1421377d0_impl(
        param_1, param_2, param_3, param_4, param_5, param_6, param_7,
    )
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_1421377d0(
    param_1: usize,
    param_2: usize,
    _r8: usize,
    param_4: u8,
    param_5: u8,
    param_6: usize,
    param_7: usize,
    _xmm0: f32,
    _xmm1: f32,
    param_3: f32,
) -> usize {
    unknown_1421377d0_impl(
        param_1, param_2, param_3, param_4, param_5, param_6, param_7,
    )
}

#[inline(always)]
unsafe fn voxel_flat_idx(grid: usize, x: i32, y: i32, z: i32) -> isize {
    let stride_x = rd::<i32>(grid + 0x08);
    let stride_y = rd::<i32>(grid + 0x0c);
    z.wrapping_mul(stride_x)
        .wrapping_add(x)
        .wrapping_mul(stride_y)
        .wrapping_add(y) as isize
}

#[inline(always)]
unsafe fn unknown_1421377d0_impl(
    param_1: usize,
    param_2: usize,
    param_3: f32,
    param_4: u8,
    param_5: u8,
    param_6: usize,
    param_7: usize,
) -> usize {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let r12d = rd::<i32>(param_2);
    let r8 = rd::<usize>(param_6 + 0x28);
    let min_x = rd::<i32>(r8 + 0x1c);
    let min_y = rd::<i32>(r8 + 0x20);
    let max_x = rd::<i32>(r8 + 0x28);
    let max_y = rd::<i32>(r8 + 0x2c);
    let r15d = rd::<i32>(param_2 + 0x08);

    if r12d < min_x || r15d < rd::<i32>(r8 + 0x24) || r12d > max_x || r15d > rd::<i32>(r8 + 0x30)
    {
        wr::<u8>(param_1, 1);
        wr::<u8>(param_1 + 0x10, 0);
        return param_1;
    }

    let y_in = rd::<i32>(param_2 + 0x04);
    let scaled = (param_3 * rd::<f32>(r8 + 0x04)).floor();
    let ecx_delta = crate::simd_ops::cvt_f32_i32(scaled) as i32;
    let y_hi = y_in.wrapping_add(ecx_delta);
    let initial_y = if y_hi < max_y { y_hi } else { max_y };
    let y_lo = y_in.wrapping_sub(ecx_delta);

    let flag2_init = rd::<u8>(param_6 + 0x20) != 0
        && (rd::<u8>(
            (rd::<usize>(param_6 + 0x18) as isize + voxel_flat_idx(r8, r12d, initial_y, r15d))
                as usize,
        ) & 2)
            != 0;
    let r11b_init = param_4 == 0 && flag2_init;

    let r10_data = rd::<usize>(param_6 + 0x08);
    let corners_base = exe::ptr::<i32>(0x1_433e_2570) as usize;
    let edi_init = rd::<i32>(r8 + 0x0c);
    let esi_init = rd::<i32>(r8 + 0x08);
    let xmm7_max = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd0) as usize);
    let xmm8_eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_7008) as usize);

    let mut corner_solid_init = false;
    for c in 0..8usize {
        let cp = corners_base.wrapping_add(c * 12);
        let dx = rd::<i32>(cp);
        let dy = rd::<i32>(cp + 0x04);
        let dz = rd::<i32>(cp + 0x08);
        let idx = r15d
            .wrapping_add(dz)
            .wrapping_mul(esi_init)
            .wrapping_add(dx)
            .wrapping_add(r12d)
            .wrapping_mul(edi_init)
            .wrapping_add(initial_y.wrapping_add(dy)) as isize;
        let f1 = rd::<f32>((r10_data as isize + idx * 12 + 4) as usize);
        if !(xmm7_max > f1) || !(f1 > xmm8_eps) {
            corner_solid_init = true;
            break;
        }
    }

    let dl_init = corner_solid_init || r11b_init;
    let mut all_solid = dl_init;
    let mut prev_solid = dl_init;
    let mut r11d = initial_y;
    let mut last_tested_y;
    let mut xmm6_y = initial_y;
    let mut eax_y = initial_y;

    if r11d >= y_lo {
        let xmm9_01 = rd::<f32>(exe::ptr::<f32>(0x1_4342_65d4) as usize);
        loop {
            last_tested_y = r11d;
            let rdx_grid = rd::<usize>(param_6 + 0x28);
            if r11d < min_y {
                let next_y = r11d.wrapping_add(1);
                xmm6_y = next_y;
                eax_y = next_y;
                break;
            }

            let flag8 = param_5 != 0
                && (rd::<u8>(
                    (rd::<usize>(param_6 + 0x18) as isize
                        + voxel_flat_idx(rdx_grid, r12d, r11d, r15d)) as usize,
                ) & 8)
                    != 0;
            if r11d >= y_in && flag8 {
                wr::<u8>(param_1, 0);
                wr::<u8>(param_1 + 0x10, 0);
                return param_1;
            }

            let flag2 = rd::<u8>(param_6 + 0x20) != 0
                && (rd::<u8>(
                    (rd::<usize>(param_6 + 0x18) as isize
                        + voxel_flat_idx(rdx_grid, r12d, r11d, r15d)) as usize,
                ) & 2)
                    != 0;
            let mut r10b = param_4 == 0 && flag2;

            let grid_a = rd::<usize>(param_6 + 0x28);
            let data_a = rd::<usize>(param_6 + 0x08);
            let edi_a = rd::<i32>(grid_a + 0x0c);
            let esi_a = rd::<i32>(grid_a + 0x08);
            for c in 0..8usize {
                let cp = corners_base.wrapping_add(c * 12);
                let dx = rd::<i32>(cp);
                let dy = rd::<i32>(cp + 0x04);
                let dz = rd::<i32>(cp + 0x08);
                let idx = r15d
                    .wrapping_add(dz)
                    .wrapping_mul(esi_a)
                    .wrapping_add(dx)
                    .wrapping_add(r12d)
                    .wrapping_mul(edi_a)
                    .wrapping_add(r11d.wrapping_add(dy)) as isize;
                let f1 = rd::<f32>((data_a as isize + idx * 12 + 4) as usize);
                if !(xmm7_max > f1) || !(f1 > xmm8_eps) {
                    r10b = true;
                    break;
                }
            }

            let mut cleared_at_c02 = false;
            if r11d >= y_in && !r10b {
                let flag2_b = rd::<u8>(param_6 + 0x20) != 0
                    && (rd::<u8>(
                        (rd::<usize>(param_6 + 0x18) as isize
                            + voxel_flat_idx(rd::<usize>(param_6 + 0x28), r12d, r11d, r15d))
                            as usize,
                    ) & 2)
                        != 0;
                if param_4 != 0 || !flag2_b {
                    let grid_b = rd::<usize>(param_6 + 0x28);
                    let data_b = rd::<usize>(param_6 + 0x08);
                    let xmm4 = xmm9_01 * rd::<f32>(param_7);
                    let ebx_b = rd::<i32>(grid_b + 0x0c);
                    let edi_b = rd::<i32>(grid_b + 0x08);
                    let xmm5 = rd::<f32>(param_7 + 0x08);

                    for c in 0..8usize {
                        let cp = corners_base.wrapping_add(c * 12);
                        let dx = rd::<i32>(cp);
                        let dy = rd::<i32>(cp + 0x04);
                        let dz = rd::<i32>(cp + 0x08);
                        let idx = r15d
                            .wrapping_add(dz)
                            .wrapping_mul(edi_b)
                            .wrapping_add(dx)
                            .wrapping_add(r12d)
                            .wrapping_mul(ebx_b)
                            .wrapping_add(r11d.wrapping_add(dy))
                            as isize;
                        let cell = (data_b as isize + idx * 12) as usize;
                        let f0 = rd::<f32>(cell);
                        let f1 = rd::<f32>(cell + 0x04);
                        if xmm5 > f1 {
                            continue;
                        }
                        if !f1.is_nan() && !xmm7_max.is_nan() && f1 == xmm7_max {
                            continue;
                        }
                        let f2 = rd::<f32>(cell + 0x08);
                        let len = (f2 * f2 + f0 * f0).sqrt();
                        if !(xmm4 > len) {
                            r10b = false;
                            all_solid = false;
                            cleared_at_c02 = true;
                            break;
                        }
                    }
                }
            }

            if !cleared_at_c02 {
                all_solid = all_solid && r10b;
                if prev_solid && !r10b {
                    wr::<u8>(param_1, 0);
                    wr::<i32>(param_1 + 0x04, r12d);
                    wr::<i32>(param_1 + 0x08, r11d.wrapping_add(1));
                    wr::<i32>(param_1 + 0x0c, r15d);
                    wr::<u8>(param_1 + 0x10, 1);
                    return param_1;
                }
            }

            r11d = r11d.wrapping_sub(1);
            prev_solid = r10b;
            if r11d < y_lo {
                xmm6_y = last_tested_y;
                eax_y = last_tested_y;
                break;
            }
        }
    }

    wr::<u8>(param_1, 0);
    if all_solid {
        wr::<i32>(param_1 + 0x04, r12d);
        wr::<i32>(param_1 + 0x08, xmm6_y);
        wr::<i32>(param_1 + 0x0c, r15d);
        wr::<u8>(param_1 + 0x10, 1);
    } else if prev_solid {
        wr::<i32>(param_1 + 0x04, r12d);
        wr::<i32>(param_1 + 0x08, eax_y.wrapping_add(1));
        wr::<i32>(param_1 + 0x0c, r15d);
        wr::<u8>(param_1 + 0x10, 1);
    } else {
        wr::<u8>(param_1 + 0x10, 0);
    }
    param_1
}

/// `physics_142153950`
/// Original: 0x142153950 (618 bytes, Throws::No)
pub unsafe extern "win64" fn physics_142153950(param_1: usize, param_2: usize) {
    let mut rbx = rd::<u64>(param_2);
    let mut xmm13 = rd::<u128>(exe::ptr::<u128>(0x1_4342_89d0) as usize);
    let mut xmm12 = rd::<u128>(exe::ptr::<u128>(0x1_4342_86f0) as usize);

    if rbx != 0 {
        let mut r11 = rd::<usize>(param_1 + 0x40).wrapping_add(0x30);
        let mut r10 = rd::<usize>(param_2 + 0x08);
        let xmm15 = rd::<u128>(exe::ptr::<u128>(0x1_433e_4530) as usize);

        while rbx != 0 {
            let mut rax = rd::<usize>(r11);
            let count = rd::<u32>(r11 + 0x08) as usize;
            let rcx_end = rax.wrapping_add(count * 8);
            if rax != rcx_end {
                let r9 = rd::<usize>(param_1 + 0xa0);
                while rax != rcx_end {
                    let edx = rd::<u32>(rax) as usize;
                    let ymm14_lo = rd::<u128>(r10);
                    let ymm14_hi = rd::<u128>(r10 + 0x10);
                    let elem = r9.wrapping_add(edx * 0x18);
                    let mut xmm7 = rd::<u128>(elem);
                    let mut xmm11 = vpermilps(rd::<u128>(elem + 0x08), 0x39);
                    let cmp = vcmpps(xmm11, xmm7, 1);
                    let mask = (vmovmskps(cmp) as u32) & 7;
                    if mask != 7 {
                        let xmm4 = vaddps(ymm14_lo, ymm14_lo);
                        let xmm1_a = vmulps(ymm14_lo, xmm4);
                        let xmm3_a = vpsrldq(vpslldq(vpermilps(xmm1_a, 0xc1), 4), 4);
                        let xmm2_a = vpsrldq(vpslldq(vpermilps(xmm1_a, 0xda), 4), 4);
                        let xmm6 = vsubps(vsubps(xmm15, xmm3_a), xmm2_a);
                        let xmm3_b = vmulps(vshufps(ymm14_lo, ymm14_lo, 0xd0), vpermilps(xmm4, 0xe6));
                        let xmm2_c = vmulps(vshufps(ymm14_lo, ymm14_lo, 0xff), vpermilps(xmm4, 0xc9));
                        let xmm5_a = vaddps(xmm2_c, xmm3_b);
                        let xmm4_a = vsubps(xmm3_b, xmm2_c);
                        let xmm2_d = vpermilps(vshufps(xmm5_a, xmm4_a, 0x49), 0x78);
                        let xmm3 = vpermilps(vshufps(xmm6, xmm2_d, 0x4c), 0x78);
                        let xmm2 = vpermilps(vshufps(xmm6, xmm2_d, 0xed), 0x72);
                        let xmm0_g = vpermilps(vshufps(xmm5_a, xmm4_a, 0xa0), 0x88);
                        let xmm4_col = vblendps(xmm0_g, xmm6, 0x0c);

                        let xmm10 = vmulps(vpermilps(xmm7, 0x00), xmm3);
                        let xmm8 = vmulps(vpermilps(xmm7, 0x55), xmm2);
                        let xmm9 = vmulps(vpermilps(xmm11, 0x00), xmm3);
                        let xmm6_m = vmulps(vpermilps(xmm11, 0x55), xmm2);
                        let xmm5_m = vmulps(vpermilps(xmm11, 0xaa), xmm4_col);
                        let xmm7_m = vmulps(vpermilps(xmm7, 0xaa), xmm4_col);

                        let max_xyz = vaddps(
                            vaddps(vmaxps(xmm8, xmm6_m), vmaxps(xmm10, xmm9)),
                            vmaxps(xmm7_m, xmm5_m),
                        );
                        let min_xyz = vaddps(
                            vaddps(vminps(xmm8, xmm6_m), vminps(xmm10, xmm9)),
                            vminps(xmm7_m, xmm5_m),
                        );
                        xmm7 = vaddps(min_xyz, ymm14_hi);
                        xmm11 = vaddps(max_xyz, ymm14_hi);
                    }
                    rax = rax.wrapping_add(8);
                    xmm12 = vminps(xmm7, xmm12);
                    xmm13 = vmaxps(xmm11, xmm13);
                }
            }
            r10 = r10.wrapping_add(0x20);
            r11 = r11.wrapping_add(0xb0);
            rbx -= 1;
        }
    }

    let xmm0 = vshufps(xmm12, xmm13, 0x4e);
    let xmm1 = vshufps(xmm0, xmm13, 0x98);
    wr::<u128>(param_1 + 0x110, xmm12);
    wr::<u128>(param_1 + 0x118, xmm1);
}

/// `physics_142155510`
/// Original: 0x142155510 (256 bytes, Throws::Caught)
pub unsafe extern "win64" fn physics_142155510(param_1: usize, param_2: usize, param_3: u16) {
    let rbx = param_3 as u32;
    let rdx_arr = rd::<usize>(param_1 + 0x10);
    let eax = rbx.wrapping_add(1);
    let next_val = if eax == rd::<u32>(param_1 + 0x18) {
        rd::<u16>(param_1 + 0x28)
    } else {
        rd::<u16>(rdx_arr.wrapping_add((eax as usize) * 2))
    };
    let cur_val = rd::<u16>(rdx_arr.wrapping_add((rbx as usize) * 2));

    if next_val.wrapping_sub(cur_val) == 0 {
        let len = rd::<u32>(param_2 + 0x08) as u64;
        let cap = rd::<u32>(param_2 + 0x0c) as u64;
        if len * 2 == cap * 2 {
            let min_cap = len + 1;
            let grow = (cap >> 1) + cap;
            let new_cap = if grow < min_cap { min_cap } else { grow };
            exe::call_va!(0x1_40fb_3a60, (param_2, new_cap));
        }
        let cur_len = rd::<u32>(param_2 + 0x08) as usize;
        let base = rd::<usize>(param_2);
        wr::<u16>(base.wrapping_add(cur_len * 2), param_3);
        wr::<u32>(param_2 + 0x08, (cur_len as u32).wrapping_add(1));
    } else {
        let mut local_vec: [u64; 2] = [0, 0];
        let vec_ptr = local_vec.as_mut_ptr() as usize;
        exe::call_va!(0x1_4215_5660, (param_1, vec_ptr, param_3 as u32));
        let mut cur = rd::<usize>(vec_ptr);
        let count = rd::<u32>(vec_ptr + 0x08) as usize;
        let end = cur.wrapping_add(count * 2);
        while cur != end {
            let child = rd::<u16>(cur);
            physics_142155510(param_1, param_2, child);
            cur = cur.wrapping_add(2);
        }
        let buf_ptr = rd::<usize>(vec_ptr);
        if buf_ptr != 0 {
            exe::call_va!(0x1_4241_c190, (buf_ptr));
        }
    }
}

/// `unknown_1421c2ef0`
/// Original: 0x1421c2ef0 (596 bytes, Throws::No, FLOAT_RETURN)
pub unsafe extern "win64" fn unknown_1421c2ef0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let r11 = (rd::<u8>(param_1 + 0x07) as i8) as isize;
    let xmm8 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8480) as usize);
    let xmm9 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8550) as usize);
    let xmm7 = rd::<u128>(exe::ptr::<u128>(0x1_4342_9a80) as usize);
    let abs_mask = !xmm7;
    let mut xmm5: u128 = 0;

    if r11 > 0 {
        let rbx = rd::<usize>(param_1 + 0x08);
        let mut r8_ptr = rd::<usize>(param_1 + 0x10);
        let mut r9_ptr = param_4;

        for idx in 0..(r11 as usize) {
            let op = rd::<u8>(rbx.wrapping_add(idx));
            let r10 = (rd::<i32>(r8_ptr) as isize) as usize;
            let xmm6 = rd::<u128>(r9_ptr);
            let p2 = param_2.wrapping_add(r10);
            let p3 = param_3.wrapping_add(r10);

            match op {
                0 | 4 => {
                    let xmm2 = vcvtph2ps(rd::<u64>(p2));
                    let xmm0 = vcvtph2ps(rd::<u64>(p3));
                    let xmm3 = abs_mask & vsubps(xmm2, xmm0);
                    xmm5 = vfmadd231ps_128(xmm5, xmm6, xmm3);
                }
                1 | 7 => {
                    let xmm0 = vcvtph2ps(rd::<u64>(p3));
                    let xmm2 = vcvtph2ps(rd::<u64>(p2));
                    let xmm3 = abs_mask & vsubps(xmm2, xmm0);
                    let mask = vcmpps(xmm3, xmm8, 1);
                    let xmm1 = vsubps(xmm9, xmm3);
                    let blended = vblendvps(xmm1, xmm3, mask);
                    xmm5 = vfmadd231ps_128(xmm5, xmm6, blended);
                }
                2 => {
                    let ymm0_lo = vcvtph2ps(rd::<u64>(p2));
                    let ymm0_hi = vcvtph2ps(rd::<u64>(p2 + 0x08));
                    let ymm1_lo = vcvtph2ps(rd::<u64>(p3));
                    let ymm1_hi = vcvtph2ps(rd::<u64>(p3 + 0x08));
                    let xmm2_c = vcvtph2ps(rd::<u64>(p3 + 0x10));
                    let ymm4_lo = abs_mask & vsubps(ymm0_lo, ymm1_lo);
                    let ymm4_hi = abs_mask & vsubps(ymm0_hi, ymm1_hi);
                    let xmm0_c = vcvtph2ps(rd::<u64>(p2 + 0x10));
                    let sum_ab = vaddps(ymm4_hi, ymm4_lo);
                    let xmm3 = abs_mask & vsubps(xmm0_c, xmm2_c);
                    let xmm1 = vaddps(xmm3, sum_ab);
                    xmm5 = vfmadd231ps_128(xmm5, xmm6, xmm1);
                }
                3 => {
                    let ymm1_lo = vcvtph2ps(rd::<u64>(p3));
                    let ymm1_hi = vcvtph2ps(rd::<u64>(p3 + 0x08));
                    let ymm0_lo = vcvtph2ps(rd::<u64>(p2));
                    let ymm0_hi = vcvtph2ps(rd::<u64>(p2 + 0x08));
                    let sub_lo = vsubps(ymm0_lo, ymm1_lo);
                    let sub_hi = vsubps(ymm0_hi, ymm1_hi);
                    let xmm0_c = vcvtph2ps(rd::<u64>(p2 + 0x10));
                    let sq_lo = vmulps(sub_lo, sub_lo);
                    let sq_hi = vmulps(sub_hi, sub_hi);
                    let xmm2_c = vcvtph2ps(rd::<u64>(p3 + 0x10));
                    let xmm3 = vsubps(xmm0_c, xmm2_c);
                    let mut xmm2 = vaddps(sq_hi, sq_lo);
                    xmm2 = vfmadd231ps_128(xmm2, xmm3, xmm3);
                    let eps = rd::<u128>(exe::ptr::<u128>(0x1_4342_7f40) as usize);
                    let xmm0 = vaddps(xmm2, eps);
                    let xmm1 = vrsqrtps_128(xmm0);
                    let dist = vmulps(xmm1, xmm2);
                    xmm5 = vfmadd231ps_128(xmm5, xmm6, dist);
                }
                5 => {
                    let xmm1 = rd::<u64>(p2) as u128;
                    let xmm0 = rd::<u64>(p3) as u128;
                    let eq = vpcmpeqd(xmm1, xmm0);
                    let blended = vblendvps(xmm6, 0, eq);
                    xmm5 = vaddps(blended, xmm5);
                }
                6 => {
                    let ymm1_lo = vcvtph2ps(rd::<u64>(p3));
                    let ymm1_hi = vcvtph2ps(rd::<u64>(p3 + 0x08));
                    let ymm0_lo = vcvtph2ps(rd::<u64>(p2));
                    let ymm0_hi = vcvtph2ps(rd::<u64>(p2 + 0x08));
                    let prod_lo = vmulps(ymm1_lo, ymm0_lo);
                    let prod_hi = vmulps(ymm1_hi, ymm0_hi);
                    let xmm0_c = vcvtph2ps(rd::<u64>(p2 + 0x10));
                    let mut xmm4 = vaddps(prod_hi, prod_lo);
                    let xmm2_c = vcvtph2ps(rd::<u64>(p3 + 0x10));
                    xmm4 = vfmadd231ps_128(xmm4, xmm0_c, xmm2_c);
                    let ones = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
                    let xmm1 = vsubps(ones, xmm4);
                    xmm5 = vfmadd231ps_128(xmm5, xmm6, xmm1);
                }
                _ => {}
            }

            r8_ptr = r8_ptr.wrapping_add(4);
            r9_ptr = r9_ptr.wrapping_add(16);
        }
    }

    let xmm1 = vhaddps(xmm5, xmm5);
    let xmm0 = vaddps(vpermilps(xmm1, 0xb1), xmm1);
    crate::simd_ops::set_xmm_ret(xmm0);
    f32::from_bits(xmm0 as u32)
}

/// `unknown_1421d4c70`
/// Original: 0x1421d4c70 (330 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421d4c70(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> u8 {
    let r9 = rd::<u64>(param_1 + 0x18);
    let eax_30 = rd::<u32>(param_1 + 0x30) as u64;
    wr::<u64>(param_1 + 0x20, r9);
    wr::<u64>(param_1 + 0x28, eax_30.wrapping_add(r9));
    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x10, 0xffff_ffff);

    let p_160 = rd::<usize>(param_1 + 0x160);
    let base_30 = rd::<u64>(p_160 + 0x10);
    let tbl_a8 = rd::<usize>(p_160 + 0x08);
    let p_168 = rd::<usize>(param_1 + 0x168);
    let mut word_count = rd::<u16>(p_168 + 0x02) as usize;
    let mut mask_a_ptr = rd::<usize>(p_168 + 0x08);
    let base_40 = rd::<u64>(p_168 + 0xc0);
    let mut mask_b_ptr = param_2;

    let mut ebp: u32 = 0;
    let mut r14b: u8 = 0;

    while word_count != 0 {
        let mut rbx = rd::<u64>(mask_a_ptr) & rd::<u64>(mask_b_ptr);
        mask_a_ptr = mask_a_ptr.wrapping_add(8);
        mask_b_ptr = mask_b_ptr.wrapping_add(8);
        word_count -= 1;

        while rbx != 0 {
            let rdi = rbx.trailing_zeros();
            r14b = 1;
            let edx = rdi.wrapping_add(ebp);
            let entry = tbl_a8.wrapping_add((edx as usize) * 12);
            let off0 = rd::<u32>(entry + 0x04) as u64;
            wr::<u64>(param_1, off0.wrapping_add(base_30));
            let off1 = rd::<u32>(entry + 0x08) as u64;
            wr::<u64>(param_1 + 0x08, off1.wrapping_add(base_40));
            wr::<u32>(param_1 + 0x10, edx);
            let kind = rd::<u8>(entry) as usize;
            let fn_va = rd::<u64>(param_3.wrapping_add(kind * 8));
            exe::call_va!(fn_va, (param_1));
            rbx ^= 1u64 << (rdi & 63);
        }

        ebp = ebp.wrapping_add(0x40);
    }

    r14b
}

/// `puppet_1421db4a0`
/// Original: 0x1421db4a0 (401 bytes, Throws::Discard)
pub unsafe extern "win64" fn puppet_1421db4a0(param_1: usize, param_2: usize) -> u32 {
    exe::call_va!(0x1_4249_68a0, (param_1 + 0xc0, param_2));
    let vtbl = rd::<usize>(param_1);
    exe::call_va!(rd::<usize>(vtbl + 0xf8), (param_1));

    let aligned_hdr = rd::<usize>(param_1 + 0xc0).wrapping_add(0x13) & !0x0fusize;
    let base_off = rd::<u32>(aligned_hdr) as usize;
    let mut reloc_ptr = aligned_hdr.wrapping_add(0x0c);
    let mut reloc_count = rd::<u32>(aligned_hdr + 0x08) as usize;
    let rdx_base = aligned_hdr.wrapping_add(base_off);

    while reloc_count != 0 {
        let rel_off = rd::<u32>(reloc_ptr) as usize;
        reloc_ptr = reloc_ptr.wrapping_add(4);
        let target = rdx_base.wrapping_add(rel_off);
        wr::<u64>(target, rd::<u64>(target).wrapping_add(rdx_base as u64));
        reloc_count -= 1;
    }

    wr::<usize>(param_1 + 0xb8, rdx_base);
    if rd::<u16>(rdx_base + 0x06) > 0 {
        let off_30 = rd::<i32>(rdx_base + 0x30);
        let rbp = if off_30 == 0 {
            0
        } else {
            (rdx_base + 0x30).wrapping_add(off_30 as isize as usize)
        };
        let r15 = rd::<usize>(rdx_base + 0x88);
        let g_b38 = exe::ptr::<u8>(0x1_43a8_ab38) as usize;
        let g_b40 = exe::ptr::<usize>(0x1_43a8_ab40) as usize;
        let g_b78 = exe::ptr::<u64>(0x1_43a8_ab78) as usize;
        let g_b80 = exe::ptr::<u64>(0x1_43a8_ab80) as usize;

        let mut di: u16 = 0;
        loop {
            let idx = di as usize;
            let slot_ptr = r15.wrapping_add(idx * 8);
            let str_ptr = rd::<usize>(slot_ptr);
            let mut len: usize = 0;
            while rd::<u8>(str_ptr.wrapping_add(len)) != 0 {
                len += 1;
            }
            let hash_id = rd::<u32>(rbp.wrapping_add(idx * 4));
            let mut str_view: [u64; 2] = [str_ptr as u64, len as u64];
            exe::call_va!(
                0x1_4240_b380,
                (g_b38, hash_id, str_view.as_mut_ptr() as usize)
            );
            let mut key_qword: u64 = hash_id as u64;
            let tid = exe::call_iat!(0x1_4264_5790, () -> u32) as u64;
            if rd::<u64>(g_b80) == tid {
                let _ = crate::simd_ops::atomic_add_u64(g_b78 as u64, 1);
            } else {
                exe::call_va!(0x1_4137_9040, (g_b78, 0u64, 1u32));
            }
            let mut out_iter: u64 = 0;
            let res_ptr = exe::call_va!(
                0x1_4137_90c0,
                (
                    g_b38,
                    (&mut out_iter as *mut u64) as usize,
                    (&mut key_qword as *mut u64) as usize
                ) -> usize
            );
            let node = rd::<usize>(res_ptr);
            let val = if node == rd::<usize>(g_b40) {
                0
            } else {
                rd::<u64>(node + 0x18)
            };
            let _ = crate::simd_ops::atomic_sub_u64(g_b78 as u64, 1);
            di = di.wrapping_add(1);
            wr::<u64>(slot_ptr, val);
            let cur_hdr = rd::<usize>(param_1 + 0xb8);
            if di >= rd::<u16>(cur_hdr + 0x06) {
                break;
            }
        }
    }

    crate::simd_ops::atomic_xchg_u32((param_1 + 0x80) as u64, 4)
}

#[inline(always)]
unsafe fn calc_remaining_budget_206560(r12: usize) -> u64 {
    let total = rd::<u64>(r12);
    let sub_obj = rd::<usize>(r12 + 0x18);
    let used = (rd::<u64>(sub_obj + 0x170) << 16).wrapping_add(rd::<u64>(sub_obj + 0x210));
    let clamped = if used < total { used } else { total };
    total.wrapping_sub(clamped)
}

#[inline(always)]
unsafe fn enqueue_item_280(param_1: usize, slot_ptr: usize) {
    let item = rd::<usize>(slot_ptr);
    if item != 0 {
        let prev = crate::simd_ops::atomic_xadd_u32((item + 0x24) as u64, 1);
        if prev == 0 {
            let cb_ctx = rd::<usize>(item + 0x28);
            if cb_ctx != 0 {
                exe::call_va!(0x1_4241_d6f0, (cb_ctx, item));
            }
        }
    }
    let frame_id = rd::<u64>(param_1 + 0x18);
    let vec_280 = param_1 + 0x280;
    let len = rd::<u32>(vec_280 + 0x08) as u64;
    let cap = rd::<u32>(vec_280 + 0x0c) as u64;
    if (len << 4) == (cap << 4) {
        let min_cap = len + 1;
        let grow = (cap >> 1) + cap;
        let new_cap = if grow < min_cap { min_cap } else { grow };
        exe::call_va!(0x1_412d_f790, (vec_280, new_cap));
    }
    let cur_len = rd::<u32>(vec_280 + 0x08) as usize;
    let dst = rd::<usize>(vec_280).wrapping_add(cur_len * 16);
    wr::<usize>(dst, item);
    wr::<u64>(dst + 0x08, frame_id);
    wr::<u32>(vec_280 + 0x08, (cur_len as u32).wrapping_add(1));
}

/// `unknown_142206560`
/// Original: 0x142206560 (2008 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142206560(param_1: usize, param_2: usize) {
    let g_2f0 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
    let mut rbx_ptr = rd::<usize>(g_2f0 + 0x4e8);
    let count_4f0 = rd::<u32>(g_2f0 + 0x4f0) as usize;
    let rdi_end = rbx_ptr.wrapping_add(count_4f0 * 8);
    while rbx_ptr != rdi_end {
        let listener = rd::<usize>(rbx_ptr);
        let vtbl = rd::<usize>(listener);
        exe::call_va!(rd::<usize>(vtbl + 0x18), (listener));
        rbx_ptr = rbx_ptr.wrapping_add(8);
    }

    let r12 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
    let mut r13_rem = calc_remaining_budget_206560(r12);

    let mut qpc: i64 = 0;
    let _ = exe::call_iat!(0x1_4264_56f8, ((&mut qpc as *mut i64) as usize));
    let _freq = exe::call_va!(0x1_4240_6920, () -> i64);
    let mut timer_68: [u64; 5] = [qpc as u64, 0, 0, 0, 0];

    let mut r15b_time_exceeded = false;
    let mut ebx_flags: u32 = 0;
    let mut rsp_110_failed = false;

    let mut rdi = rd::<usize>(param_1 + 0x50);
    let rsi_end = rdi.wrapping_add((rd::<u32>(param_1 + 0x58) as usize) * 8);
    let mut sil_loop1_failed = false;

    while rdi != rsi_end {
        let item = rd::<usize>(rdi);
        let target_lod = rd::<u8>(item + 0xbb).wrapping_sub(1);
        if rd::<u8>(item + 0x158 + (target_lod as usize)) == 0 {
            let ok = exe::call_va!(0x1_4220_8b10, (item, target_lod as u32) -> u8);
            if ok == 0 {
                sil_loop1_failed = true;
                rsp_110_failed = true;
                ebx_flags = 2;
                break;
            }
        }
        wr::<u8>(item + 0xbd, target_lod);
        enqueue_item_280(param_1, rdi);
        r13_rem = calc_remaining_budget_206560(r12);
        rdi = rdi.wrapping_add(8);
    }

    let rsp_128_loop1_failed = sil_loop1_failed;
    let raw_total = rd::<u64>(r12);
    let total_f32 = if (raw_total as i64) < 0 {
        let half = ((raw_total >> 1) | (raw_total & 1)) as i64 as f32;
        half + half
    } else {
        (raw_total as i64) as f32
    };
    let mut xmm1 = total_f32 * rd::<f32>(exe::ptr::<f32>(0x1_4386_7c98) as usize);
    let two63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);
    let mut off: u64 = 0;
    if xmm1 >= two63 {
        xmm1 -= two63;
        if xmm1 < two63 {
            off = 0x8000_0000_0000_0000;
        }
    }
    let rdx_thresh = crate::simd_ops::cvt_f32_i64(xmm1).wrapping_add(off);

    let mut rsp_120_stopped = false;
    let mut sil = rsp_110_failed;
    let xmm7_1000 = rd::<f64>(exe::ptr::<f64>(0x1_4342_6d28) as usize);

    for bucket_idx in 0..8usize {
        if rsp_120_stopped {
            break;
        }
        let bucket_ptr = param_2.wrapping_add(bucket_idx * 16);
        let mut cur = rd::<usize>(bucket_ptr);
        let end = cur.wrapping_add((rd::<u32>(bucket_ptr + 0x08) as usize) * 16);
        while cur != end {
            let cur_288 = rd::<u32>(param_1 + 0x288) as u64;
            let max_288 = (rd::<i32>(exe::ptr::<i32>(0x1_4386_7c38) as usize) as i64) as u64;
            if cur_288 > max_288 {
                ebx_flags |= 0x20;
                rsp_120_stopped = true;
                break;
            }
            if r13_rem <= rdx_thresh {
                ebx_flags |= 0x04;
                rsp_120_stopped = true;
                break;
            }
            if r15b_time_exceeded {
                ebx_flags |= 0x01;
                rsp_120_stopped = true;
                break;
            }
            if rd::<u8>(param_1 + 0x3a) != 0 {
                let (_, elapsed_xmm0) = crate::simd_ops::call_va8_xmm(
                    0x1_401e_42d0,
                    timer_68.as_mut_ptr() as usize as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0; 4],
                );
                let elapsed = f64::from_bits(elapsed_xmm0 as u64);
                let max_ms = rd::<f32>(exe::ptr::<f32>(0x1_4386_7cf8) as usize) as f64;
                r15b_time_exceeded = !(elapsed * xmm7_1000 < max_ms);
            } else {
                r15b_time_exceeded = false;
            }
            if r15b_time_exceeded {
                ebx_flags |= 0x01;
                rsp_120_stopped = true;
                break;
            }
            if sil {
                ebx_flags |= 0x02;
                rsp_120_stopped = true;
                break;
            }
            let item = rd::<usize>(cur);
            let target_lod = rd::<u8>(cur + 0x08);
            if rd::<u8>(item + 0x158 + (target_lod as usize)) == 0 {
                let ok = exe::call_va!(0x1_4220_8b10, (item, target_lod as u32) -> u8);
                if ok == 0 {
                    rsp_110_failed = true;
                    ebx_flags |= 0x02;
                    rsp_120_stopped = true;
                    break;
                }
            }
            wr::<u8>(item + 0xbd, target_lod);
            enqueue_item_280(param_1, cur);
            r13_rem = calc_remaining_budget_206560(r12);
            sil = rsp_110_failed;
            cur = cur.wrapping_add(16);
        }
        sil = rsp_110_failed;
    }

    let (_, now_xmm0) =
        crate::simd_ops::call_va8_xmm(0x1_4240_69a0, 0, 0, 0, 0, 0, 0, 0, 0, [0; 4]);
    let xmm6_now = (f64::from_bits(now_xmm0 as u64)) as f32;
    let r12d_prev_gen = rd::<u32>(param_1 + 0x14);
    wr::<u32>(param_1 + 0x14, r12d_prev_gen.wrapping_add(1));

    let mut edx_step: u8 = 8;
    let mut bucket_ptr = param_2.wrapping_add(0x100);
    for _ in 0..8usize {
        let mut cur = rd::<usize>(bucket_ptr);
        let end = cur.wrapping_add((rd::<u32>(bucket_ptr + 0x08) as usize) * 16);
        while cur != end {
            let cur_288 = rd::<u32>(param_1 + 0x288) as u64;
            let max_288 = (rd::<i32>(exe::ptr::<i32>(0x1_4386_7c38) as usize) as i64) as u64;
            let mut mark_deferred = false;
            if cur_288 > max_288 {
                ebx_flags |= 0x20;
                mark_deferred = true;
            } else {
                if !sil && r13_rem > rdx_thresh {
                    let item = rd::<usize>(cur);
                    let delay_thresh = rd::<f32>(exe::ptr::<f32>(0x1_4386_7d30) as usize);
                    if rd::<u32>(item + 0xc8) != r12d_prev_gen
                        || delay_thresh > (xmm6_now - rd::<f32>(item + 0xcc))
                    {
                        ebx_flags |= 0x10;
                        mark_deferred = true;
                    }
                }
                if !mark_deferred {
                    if !r15b_time_exceeded {
                        if rd::<u8>(param_1 + 0x3a) != 0 {
                            let (_, elapsed_xmm0) = crate::simd_ops::call_va8_xmm(
                                0x1_401e_42d0,
                                timer_68.as_mut_ptr() as usize as u64,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                [0; 4],
                            );
                            let elapsed = f64::from_bits(elapsed_xmm0 as u64);
                            let max_ms = rd::<f32>(exe::ptr::<f32>(0x1_4386_7cf8) as usize) as f64;
                            r15b_time_exceeded = !(elapsed * xmm7_1000 < max_ms);
                        } else {
                            r15b_time_exceeded = false;
                        }
                    }
                    if r15b_time_exceeded {
                        ebx_flags |= 0x01;
                        mark_deferred = true;
                    } else {
                        let item = rd::<usize>(cur);
                        let target_lod = rd::<u8>(item + 0xbd).wrapping_add(edx_step);
                        let mut ok = true;
                        if rd::<u8>(item + 0x158 + (target_lod as usize)) == 0 {
                            ok = exe::call_va!(0x1_4220_8b10, (item, target_lod as u32) -> u8) != 0;
                        }
                        if !ok {
                            sil = true;
                            rsp_110_failed = true;
                            ebx_flags |= 0x08;
                            mark_deferred = true;
                        } else {
                            wr::<u8>(item + 0xbd, target_lod);
                            enqueue_item_280(param_1, cur);
                            r13_rem = calc_remaining_budget_206560(r12);
                            sil = rsp_110_failed;
                        }
                    }
                }
            }
            if mark_deferred {
                let item = rd::<usize>(cur);
                if rd::<u32>(item + 0xc8) != r12d_prev_gen {
                    wr::<f32>(item + 0xcc, xmm6_now);
                }
                wr::<u32>(item + 0xc8, rd::<u32>(param_1 + 0x14));
            }
            cur = cur.wrapping_add(16);
        }
        edx_step = edx_step.wrapping_sub(1);
        bucket_ptr = bucket_ptr.wrapping_sub(16);
    }

    let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
    let flag_221: u8 = if sil || r13_rem < rdx_thresh || rd::<f32>(param_1 + 0x10) > 0.0f32 {
        1
    } else {
        0
    };
    wr::<u8>(g_2b8 + 0x221, flag_221);
    wr::<u32>(param_1 + 0x290, ebx_flags);
    if !rsp_128_loop1_failed && !rsp_120_stopped {
        wr::<u64>(param_1 + 0x18, rd::<u64>(param_1 + 0x18).wrapping_add(1));
    }

    let mut rbx_end_ptr = rd::<usize>(g_2f0 + 0x4e8);
    let count_end = rd::<u32>(g_2f0 + 0x4f0) as usize;
    let rdi_end2 = rbx_end_ptr.wrapping_add(count_end * 8);
    while rbx_end_ptr != rdi_end2 {
        let listener = rd::<usize>(rbx_end_ptr);
        let vtbl = rd::<usize>(listener);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (listener));
        rbx_end_ptr = rbx_end_ptr.wrapping_add(8);
    }
}

#[inline(always)]
unsafe fn is_valid_bucket_20ee70(param_1: usize, bucket: usize) -> bool {
    let k = rd::<u64>(bucket);
    rd::<u64>(param_1 + 0x98) != k
        && (rd::<u64>(param_1 + 0x80) == 0 || rd::<u64>(param_1 + 0x78) != k)
}

#[inline(always)]
unsafe fn linear_to_srgb_byte(
    val: f32,
    thresh: f32,
    lin_mul: f32,
    exp_y: f32,
    pow_mul: f32,
    pow_sub: f32,
    scale_255: f32,
) -> u8 {
    let srgb = if val > thresh {
        let p = call_powf(val, exp_y);
        p * pow_mul - pow_sub
    } else {
        val * lin_mul
    };
    let rounded = (srgb * scale_255).round_ties_even();
    let ecx = crate::simd_ops::cvt_f32_i32(rounded) as i32;
    if ecx < 0 {
        0
    } else if ecx > 0xff {
        0xff
    } else {
        ecx as u8
    }
}

/// `unknown_14220ee70`
/// Original: 0x14220ee70 (2121 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14220ee70(param_1: usize) {
    let mut ebx = rd::<i32>(param_1 + 0x18);
    let rcx_48 = rd::<usize>(param_1 + 0x48);
    if rcx_48 != 0 {
        let add = exe::call_va!(0x1_4243_b250, (rcx_48) -> i32);
        ebx = ebx.wrapping_add(add);
    }
    let r12 = (ebx as i64) as u64;

    let mut desc_buf = [0u8; 0x30];
    let desc_ptr = desc_buf.as_mut_ptr() as usize;

    let cur_b0 = rd::<usize>(param_1 + 0xb0);
    if cur_b0 == 0 || ((rd::<i32>(cur_b0 + 0x0c) as i64) as u64) < r12 {
        desc_buf.fill(0);
        wr::<u32>(desc_ptr, 0x0c);
        wr::<i32>(desc_ptr + 0x04, ebx);
        wr::<u32>(desc_ptr + 0x08, 1);
        wr::<u32>(desc_ptr + 0x0c, 1);
        wr::<u8>(desc_ptr + 0x10, 0x19);
        wr::<u64>(desc_ptr + 0x14, 0);
        wr::<u128>(desc_ptr + 0x1c, 0);
        wr::<u32>(desc_ptr + 0x2c, 0xffff_ffff);

        let mut tmp_ptr: usize = 0;
        let res_ptr = exe::call_va!(
            0x1_412e_85c0,
            ((&mut tmp_ptr as *mut usize) as usize, 0usize, desc_ptr) -> usize
        );
        let new_tex = rd::<usize>(res_ptr);
        wr::<usize>(res_ptr, 0);
        let old_tex = rd::<usize>(param_1 + 0xb0);
        wr::<usize>(param_1 + 0xb0, new_tex);
        if old_tex != 0 {
            exe::call_va!(0x1_4204_f860, (old_tex));
            exe::call_va!(0x1_4244_2de0, (old_tex));
        }
        if tmp_ptr != 0 {
            exe::call_va!(0x1_4204_f860, (tmp_ptr));
            exe::call_va!(0x1_4244_2de0, (tmp_ptr));
        }
    }

    let cur_b8 = rd::<usize>(param_1 + 0xb8);
    if cur_b8 == 0 || ((rd::<i32>(cur_b8 + 0x0c) as i64) as u64) < r12 {
        desc_buf.fill(0);
        wr::<u32>(desc_ptr, 0x100);
        wr::<i32>(desc_ptr + 0x04, ebx);
        wr::<u32>(desc_ptr + 0x08, 1);
        wr::<u32>(desc_ptr + 0x0c, 1);
        wr::<u8>(desc_ptr + 0x10, 0x10);
        wr::<u64>(desc_ptr + 0x14, 0);
        wr::<u128>(desc_ptr + 0x1c, 0);
        wr::<u32>(desc_ptr + 0x2c, 0xffff_ffff);

        let mut tmp_ptr: usize = 0;
        let res_ptr = exe::call_va!(
            0x1_412e_85c0,
            ((&mut tmp_ptr as *mut usize) as usize, 0usize, desc_ptr) -> usize
        );
        let new_tex = rd::<usize>(res_ptr);
        wr::<usize>(res_ptr, 0);
        let old_tex = rd::<usize>(param_1 + 0xb8);
        wr::<usize>(param_1 + 0xb8, new_tex);
        if old_tex != 0 {
            exe::call_va!(0x1_4204_f860, (old_tex));
            exe::call_va!(0x1_4244_2de0, (old_tex));
        }
        if tmp_ptr != 0 {
            exe::call_va!(0x1_4204_f860, (tmp_ptr));
            exe::call_va!(0x1_4244_2de0, (tmp_ptr));
        }
    }

    wr::<u32>(
        param_1 + 0xc4,
        rd::<u32>(exe::ptr::<u32>(0x1_4386_7e40) as usize),
    );

    let mut r14 = rd::<usize>(param_1 + 0xa8);
    let r15_end = r14.wrapping_add(rd::<usize>(param_1 + 0x90) * 16);
    while r14 != r15_end && !is_valid_bucket_20ee70(param_1, r14) {
        r14 = r14.wrapping_add(16);
    }
    while r14 != r15_end {
        let item = rd::<usize>(r14 + 0x08);
        exe::call_va!(0x1_4221_12e0, (item + 0x18, 0usize, item + 0x0c));
        exe::call_va!(0x1_4221_0fa0, (item + 0xc8, 0usize, item + 0x18));
        r14 = r14.wrapping_add(16);
        while r14 != r15_end && !is_valid_bucket_20ee70(param_1, r14) {
            r14 = r14.wrapping_add(16);
        }
    }

    exe::call_va!(
        0x1_4206_aad0,
        (
            rd::<usize>(rd::<usize>(param_1 + 0xb0)),
            0usize,
            0u32,
            1u32,
            desc_ptr
        )
    );
    let rbx_data = rd::<usize>(desc_ptr);
    let rdi_pitch = (rd::<i32>(desc_ptr + 0x08) as isize) as usize;
    let _ = exe::call_va!(
        0x1_4183_acc2,
        (rbx_data, 0u32, (rdi_pitch as u64).wrapping_mul(r12)) -> usize
    );

    let mut r8 = rd::<usize>(param_1 + 0xa8);
    let r9_end = r8.wrapping_add(rd::<usize>(param_1 + 0x90) * 16);
    while r8 != r9_end && !is_valid_bucket_20ee70(param_1, r8) {
        r8 = r8.wrapping_add(16);
    }
    let r10_end = rd::<usize>(param_1 + 0xa8).wrapping_add(rd::<usize>(param_1 + 0x90) * 16);
    if r8 != r10_end {
        let c_001 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
        while r8 != r10_end {
            let item = rd::<usize>(r8 + 0x08);
            let slot_idx = rd::<u32>(item) as usize;
            let dst = rbx_data.wrapping_add(slot_idx.wrapping_mul(rdi_pitch));
            wr::<u32>(dst, 0x4130_0000);
            wr::<u32>(dst + 0x04, 0x40a0_0000);
            wr::<f32>(dst + 0x08, c_001 * rd::<f32>(item + 0x04));
            wr::<u32>(dst + 0x0c, rd::<u32>(item + 0x08));
            for k in 0..11usize {
                wr::<u128>(dst + 0x10 + k * 16, rd::<u128>(item + 0x18 + k * 16));
            }
            r8 = r8.wrapping_add(16);
            while r8 != r9_end && !is_valid_bucket_20ee70(param_1, r8) {
                r8 = r8.wrapping_add(16);
            }
        }
    }

    exe::call_va!(
        0x1_4206_a7e0,
        (
            rd::<usize>(rd::<usize>(param_1 + 0xb0)),
            0usize,
            0u32,
            desc_ptr
        )
    );

    let rbx_tex = rd::<usize>(rd::<usize>(param_1 + 0xb8));
    let mut rdi_staging: usize = 0;
    let d3d_res = rd::<usize>(rbx_tex + 0x70);
    let mut res_desc = [0u32; 16];
    let vtbl_res = rd::<usize>(d3d_res);
    exe::call_va!(
        rd::<usize>(vtbl_res + 0x50),
        (d3d_res, res_desc.as_mut_ptr() as usize)
    );

    if (rd::<u8>(rbx_tex + 0x28) & 0x10) != 0 {
        rdi_staging = rbx_tex;
        wr::<usize>(desc_ptr + 0x10, 0);
    } else if res_desc[0] == 3 {
        let w = rd::<u32>(rbx_tex).max(1);
        let h = rd::<u32>(rbx_tex + 0x04).max(1);
        let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
        let mut ctx_pair: [u64; 2] = [rd::<u64>(g_c58 + 0x40), rd::<u64>(g_c58 + 0x38)];
        let fmt = rd::<u8>(rbx_tex + 0x18);
        rdi_staging = exe::call_va!(
            0x1_4205_a6a0,
            (
                w,
                h,
                0u64,
                0u64,
                fmt,
                0x10u32,
                ctx_pair.as_mut_ptr() as usize
            ) -> usize
        );
        wr::<usize>(desc_ptr + 0x10, rdi_staging);
    } else if res_desc[0] == 4 {
        let w = rd::<u32>(rbx_tex).max(1);
        let h = rd::<u32>(rbx_tex + 0x04).max(1);
        let d = rd::<u32>(rbx_tex + 0x08).max(1);
        let fmt = rd::<u8>(rbx_tex + 0x18);
        let name_ptr = exe::ptr::<u8>(0x1_433d_c990) as usize;
        let mut tmp_tex: usize = 0;
        let out_ptr = exe::call_va!(
            0x1_4206_a100,
            (
                (&mut tmp_tex as *mut usize) as usize,
                w,
                h,
                d,
                1u32,
                fmt,
                0x10u32,
                0usize,
                name_ptr
            ) -> usize
        );
        rdi_staging = rd::<usize>(out_ptr);
        wr::<usize>(out_ptr, 0);
        wr::<usize>(desc_ptr + 0x10, rdi_staging);
        if tmp_tex != 0 {
            exe::call_va!(0x1_4206_b020, (tmp_tex));
            exe::call_va!(0x1_4244_2de0, (tmp_tex));
            rdi_staging = rd::<usize>(desc_ptr + 0x10);
        }
    }

    let staging_res = rd::<usize>(rdi_staging + 0x70);
    let staging_vtbl = rd::<usize>(staging_res);
    let hr = exe::call_va!(
        rd::<usize>(staging_vtbl + 0x40),
        (staging_res, 0u32, 0usize, desc_ptr) -> u32
    );
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }

    let pitch = rd::<i32>(rdi_staging + 0x20);
    wr::<i32>(desc_ptr + 0x08, pitch);
    wr::<u32>(desc_ptr + 0x0c, rd::<u32>(rdi_staging + 0x24));
    wr::<u32>(desc_ptr + 0x18, 1);
    let mapped_ptr = rd::<usize>(desc_ptr);
    let r14_pitch = (pitch as isize) as usize;
    let _ = exe::call_va!(
        0x1_4183_acc2,
        (mapped_ptr, 0u32, (r14_pitch as u64).wrapping_mul(r12)) -> usize
    );

    let mut rbx_cur = rd::<usize>(param_1 + 0xa8);
    let r12_end = rbx_cur.wrapping_add(rd::<usize>(param_1 + 0x90) * 16);
    while rbx_cur != r12_end && !is_valid_bucket_20ee70(param_1, rbx_cur) {
        rbx_cur = rbx_cur.wrapping_add(16);
    }
    let r13_end = rd::<usize>(param_1 + 0xa8).wrapping_add(rd::<usize>(param_1 + 0x90) * 16);
    if rbx_cur != r13_end {
        let xmm9_thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_64ac) as usize);
        let xmm8_lin = rd::<f32>(exe::ptr::<f32>(0x1_4342_6dfc) as usize);
        let xmm11_exp = rd::<f32>(exe::ptr::<f32>(0x1_4342_6734) as usize);
        let xmm12_mul = rd::<f32>(exe::ptr::<f32>(0x1_4342_6900) as usize);
        let xmm13_sub = rd::<f32>(exe::ptr::<f32>(0x1_4342_6568) as usize);
        let xmm10_255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);

        while rbx_cur != r13_end {
            let item = rd::<usize>(rbx_cur + 0x08);
            let slot_idx = rd::<u32>(item) as usize;
            let row_dst = mapped_ptr.wrapping_add(slot_idx.wrapping_mul(r14_pitch));
            let mut src_ptr = item + 0xc8;
            for i in 0..0x100usize {
                let f0 = rd::<f32>(src_ptr);
                let f1 = rd::<f32>(src_ptr + 0x04);
                let f2 = rd::<f32>(src_ptr + 0x08);
                let b0 = linear_to_srgb_byte(
                    f0,
                    xmm9_thresh,
                    xmm8_lin,
                    xmm11_exp,
                    xmm12_mul,
                    xmm13_sub,
                    xmm10_255,
                );
                let b1 = linear_to_srgb_byte(
                    f1,
                    xmm9_thresh,
                    xmm8_lin,
                    xmm11_exp,
                    xmm12_mul,
                    xmm13_sub,
                    xmm10_255,
                );
                let b2 = linear_to_srgb_byte(
                    f2,
                    xmm9_thresh,
                    xmm8_lin,
                    xmm11_exp,
                    xmm12_mul,
                    xmm13_sub,
                    xmm10_255,
                );
                let packed =
                    0xff00_0000u32 | ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
                wr::<u32>(row_dst.wrapping_add(i * 4), packed);
                src_ptr = src_ptr.wrapping_add(0x0c);
            }
            rbx_cur = rbx_cur.wrapping_add(16);
            while rbx_cur != r12_end && !is_valid_bucket_20ee70(param_1, rbx_cur) {
                rbx_cur = rbx_cur.wrapping_add(16);
            }
        }
    }

    exe::call_va!(
        0x1_4206_a7e0,
        (
            rd::<usize>(rd::<usize>(param_1 + 0xb8)),
            0usize,
            0u32,
            desc_ptr
        )
    );
    exe::call_va!(0x1_4220_ecd0, (param_1));
    wr::<u8>(param_1 + 0xc0, 0);
}

/// `unknown_142211a90`
/// Original: 0x142211a90 (98 bytes, Throws::No, FLOAT_RETURN)
pub unsafe extern "win64" fn unknown_142211a90(param_1: f32, param_2: f32, param_3: f32) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let c_001 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
    let xmm4 = param_1 + c_001;
    let xmm3 = if xmm4.is_nan() || xmm4 != 0.0f32 {
        rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize) / xmm4
    } else {
        0.0f32
    };
    let t = xmm3 * param_3;
    let sq = t * t;
    let neg_sq = f32::from_bits(sq.to_bits() ^ 0x8000_0000);
    let arg = neg_sq / (param_2 + param_2);
    let exp_val = call_expf(arg);
    let two_pi = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d88) as usize);
    let res = exp_val / (param_2 * two_pi);
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

/// `rend_142222300`
/// Original: 0x142222300 (831 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142222300(param_1: usize) {
    let edi: u32 = if rd::<u8>(rd::<usize>(param_1) + 0x31) != 0 {
        4
    } else {
        1
    };
    let rdi = edi as usize;

    let mut vec_50: [u64; 2] = [0, 0];
    let mut vec_60: [u64; 2] = [0, 0];
    let mut vec_70: [u64; 2] = [0, 0];
    let p50 = vec_50.as_mut_ptr() as usize;
    let p60 = vec_60.as_mut_ptr() as usize;
    let p70 = vec_70.as_mut_ptr() as usize;

    let buf_50_data = exe::call_va!(0x1_401d_ec00, (4u32, rdi as u64) -> usize);
    wr::<usize>(p50, buf_50_data);
    wr::<u32>(p50 + 0x0c, edi);
    let _ = exe::call_va!(0x1_4183_acc2, (buf_50_data, 0u32, (rdi * 8) as u64) -> usize);
    wr::<u32>(p50 + 0x08, edi);

    exe::call_va!(0x1_412e_a6a0, (p70, rdi as u64));
    exe::call_va!(0x1_412e_a6a0, (p60, rdi as u64));

    let mut precalc_loaded = false;
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_7fe0) as usize) != 0 {
        let rbx = rd::<usize>(param_1);
        let b3 = rd::<u8>(exe::ptr::<u8>(0x1_43c1_c3f3) as usize) as u32;
        let cap_bits = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (rd::<u32>(exe::ptr::<u32>(0x1_43c1_c3f0) as usize) & 0x00ff_ffff) << 3;
        let f3f8_addr = exe::ptr::<u8>(0x1_43c1_c3f8) as usize;
        let rdx_str = if len_bits > cap_bits {
            rd::<usize>(f3f8_addr)
        } else {
            f3f8_addr
        };

        let fmt_str = exe::ptr::<u8>(0x1_433f_0778) as usize;
        let formatted = exe::call_va!(
            0x1_423f_e030,
            (
                fmt_str,
                rdx_str,
                rd::<u32>(rbx + 0x170),
                rd::<u32>(rbx + 0x174)
            ) -> usize
        );

        let mut str_buf = [0u64; 36];
        let sb = str_buf.as_mut_ptr() as usize;
        wr::<u64>(sb, 0x2000_0020);
        wr::<u8>(sb + 0x08, 0);
        if formatted == 0 {
            wr::<u32>(sb + 0x04, 0);
            wr::<u8>(sb + 0x08, 0);
        } else {
            let mut len: usize = 0;
            while rd::<u8>(formatted.wrapping_add(len)) != 0 {
                len += 1;
            }
            let mut view: [u64; 2] = [formatted as u64, len as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (sb, view.as_mut_ptr() as usize, 0u32, len as u64)
            );
        }

        let rax_0 = rd::<usize>(param_1);
        let ok1 = exe::call_va!(
            0x1_4222_0df0,
            (
                rbx,
                rd::<u32>(rax_0 + 0x170),
                rd::<u32>(rax_0 + 0x174),
                p60,
                p70,
                p50,
                edi,
                sb
            ) -> u8
        );
        exe::call_va!(0x1_4240_ab10, (sb));
        if ok1 != 0 {
            precalc_loaded = true;
        } else {
            let rbx2 = rd::<usize>(param_1);
            let str2_ptr = exe::call_va!(
                0x1_4222_08c0,
                (
                    rbx2,
                    sb,
                    0u32,
                    rd::<u32>(rbx2 + 0x170),
                    rd::<u32>(rbx2 + 0x174),
                    1u8
                ) -> usize
            );
            let rdx2 = rd::<usize>(param_1);
            let ok2 = exe::call_va!(
                0x1_4222_0df0,
                (
                    rbx2,
                    rd::<u32>(rdx2 + 0x170),
                    rd::<u32>(rdx2 + 0x174),
                    p60,
                    p70,
                    p50,
                    edi,
                    str2_ptr
                ) -> u8
            );
            exe::call_va!(0x1_4240_ab10, (sb));
            if ok2 != 0 {
                precalc_loaded = true;
            } else if rd::<u8>(param_1 + 0x08) == 0 {
                let rbx3 = rd::<usize>(param_1);
                let str3_ptr = exe::call_va!(
                    0x1_4222_08c0,
                    (
                        rbx3,
                        sb,
                        0u32,
                        rd::<u32>(rbx3 + 0x170),
                        rd::<u32>(rbx3 + 0x174),
                        0u8
                    ) -> usize
                );
                let rdx3 = rd::<usize>(param_1);
                let ok3 = exe::call_va!(
                    0x1_4222_0df0,
                    (
                        rbx3,
                        rd::<u32>(rdx3 + 0x170),
                        rd::<u32>(rdx3 + 0x174),
                        p60,
                        p70,
                        p50,
                        edi,
                        str3_ptr
                    ) -> u8
                );
                exe::call_va!(0x1_4240_ab10, (sb));
                if ok3 != 0 {
                    precalc_loaded = true;
                }
            }
        }
    }

    if !precalc_loaded {
        let rcx_obj = rd::<usize>(param_1);
        exe::call_va!(
            0x1_4221_fc20,
            (
                rcx_obj,
                0u32,
                rd::<u32>(rcx_obj + 0x170),
                rd::<u32>(rcx_obj + 0x174),
                rd::<u8>(param_1 + 0x08),
                p60,
                p70,
                p50,
                edi
            )
        );
    }

    exe::call_va!(
        0x1_4221_f780,
        (rd::<usize>(param_1), p60, p70, p50, edi)
    );

    let data_70 = rd::<usize>(p70);
    let data_60 = rd::<usize>(p60);
    for i in 0..rdi {
        let ptr_60 = rd::<usize>(data_60.wrapping_add(i * 8));
        if ptr_60 != 0 {
            exe::call_va!(0x1_4244_2de0, (ptr_60));
        }
        let ptr_70 = rd::<usize>(data_70.wrapping_add(i * 8));
        if ptr_70 != 0 {
            exe::call_va!(0x1_4244_2de0, (ptr_70));
        }
    }

    exe::call_va!(0x1_401e_59a0, (p60));
    exe::call_va!(0x1_401e_59a0, (p70));
    exe::call_va!(0x1_401d_e830, (p50));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1420cba20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_op0(insn: usize, ctx: u64, regs: usize, param2: u64) -> i64 {
        let imm = rd::<u16>(insn + 0x02) as u64;
        let step = rd::<u16>(insn + 0x04) as i64;
        wr::<u64>(regs, imm ^ ctx);
        TRACE = TRACE
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(rd::<u64>(regs) ^ param2 ^ (step as u64));
        step
    }

    let _m1 = crate::difftest::Mock::new(0x1_420c_b780, mock_op0 as *const ())?;

    let run = |f: F, insn_cnt: u32| -> (u8, u64) {
        TRACE = 0xcbf2_9ce4_8422_2325;
        let mut insns = [0u8; 0x0e * 3];
        let ip = insns.as_mut_ptr() as usize;
        // Instruction 0: opcode 0, imm=0x1111, step=2 (jumps to instruction 2)
        wr::<u16>(ip + 0x02, 0x1111);
        wr::<u16>(ip + 0x04, 2);
        wr::<u8>(ip + 0x0d, 0);
        // Instruction 2: opcode 0, imm=0x3333, step=1 (finishes at instruction 3)
        let ip2 = ip + 0x0e * 2;
        wr::<u16>(ip2 + 0x02, 0x3333);
        wr::<u16>(ip2 + 0x04, 1);
        wr::<u8>(ip2 + 0x0d, 0);

        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x04, 4);
        wr::<u32>(p1p + 0x0c, insn_cnt);
        wr::<usize>(p1p + 0x18, ip);
        wr::<u64>(p1p + 0x20, 0xaaaa_bbbb_cccc_dddd);

        let ret = f(p1p, 0x1234_5678);
        (ret, TRACE)
    };

    for cnt in [0u32, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420cba20 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420e3ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DTOR_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_3fb0(_ctx: u64, vec_ptr: usize) {
        let data = rd::<usize>(vec_ptr);
        wr::<u32>(vec_ptr + 0x08, 2);
        // Entry 0: kind = 1, key_u64 = 0x1111_2222_3333_4444, val = 0xaaaa_0001
        wr::<u64>(data, 0xaaaa_0001);
        wr::<u64>(data + 0x10, 0x1111_2222_3333_4444);
        wr::<u8>(data + 0x18, 1);
        // Entry 1: kind = 2, key_u32 = 0x7777_8888, val = 0xbbbb_0002
        let e1 = data + 0x90;
        wr::<u64>(e1, 0xbbbb_0002);
        wr::<u32>(e1 + 0x10, 0x7777_8888);
        wr::<u8>(e1 + 0x18, 2);
    }
    unsafe extern "win64" fn mock_3850(entry: usize, want: u16) -> u8 {
        u8::from(rd::<u16>(entry) == want)
    }
    unsafe extern "win64" fn mock_5db0(_vec_ptr: usize) {
        DTOR_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4218_3fb0, mock_3fb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_3850, mock_3850 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411f_5db0, mock_5db0 as *const ())?;

    let run = |f: F, want: u16| -> (u64, u64, u8, u8, u32) {
        DTOR_COUNT = 0;

        let mut r8_obj1 = [0u8; 0x340];
        let r8p1 = r8_obj1.as_mut_ptr() as usize;
        wr::<u64>(r8p1, 0x9001);
        wr::<u64>(r8p1 + 0x320, 0x1111_2222_3333_4444);
        wr::<u8>(r8p1 + 0x328, 1);

        let mut r8_obj2 = [0u8; 0x340];
        let r8p2 = r8_obj2.as_mut_ptr() as usize;
        wr::<u64>(r8p2, 0x9002);
        wr::<u64>(r8p2 + 0x320, 0xffff_ffff_7777_8888);
        wr::<u8>(r8p2 + 0x328, 2);

        let mut entries = [0u8; 0x78 * 3];
        let ep = entries.as_mut_ptr() as usize;
        // Entry 0 (want=10): has +0x48 != 0 -> Path B
        wr::<u16>(ep, 10);
        wr::<u64>(ep + 0x48, 1);
        // Entry 1 (want=20): has r8_obj1 (kind=1) -> Path C
        let e1 = ep + 0x78;
        wr::<u16>(e1, 20);
        wr::<usize>(e1 + 0x60, r8p1);
        // Entry 2 (want=30): has r8_obj2 (kind=2) -> Path C
        let e2 = ep + 0xf0;
        wr::<u16>(e2, 30);
        wr::<usize>(e2 + 0x60, r8p2);

        let mut p2 = vec![0u8; 0x5000];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 0x5555);
        wr::<usize>(p2p + 0x4fe0, ep);
        wr::<u32>(p2p + 0x4fe8, 3);

        let mut out = [0u64; 4];
        let op = out.as_mut_ptr() as usize;
        let ret = f(op, p2p, want);
        assert_eq!(ret, op);
        let has_val = rd::<u8>(op + 0x18);
        if has_val == 0 {
            (rd::<u32>(op) as u64, 0, 0, 0, DTOR_COUNT)
        } else {
            (
                rd::<u64>(op),
                rd::<u64>(op + 0x08),
                rd::<u8>(op + 0x10),
                has_val,
                DTOR_COUNT,
            )
        }
    };

    for want in [10u16, 20, 30, 99] {
        let o = run(f_o, want);
        let n = run(f_n, want);
        crate::diff_eq!(o, n, "1420e3ef0 want={}", want);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1421377d0(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize,
               coord: [i32; 3],
               radius: f32,
               p4: u8,
               p5: u8|
     -> (u8, i32, i32, i32, u8) {
        let mut grid_meta = [0u8; 0x40];
        let gp = grid_meta.as_mut_ptr() as usize;
        wr::<f32>(gp + 0x04, 1.0); // scale
        wr::<i32>(gp + 0x08, 4); // stride_x
        wr::<i32>(gp + 0x0c, 4); // stride_y
        wr::<i32>(gp + 0x1c, 0); // min_x
        wr::<i32>(gp + 0x20, 0); // min_y
        wr::<i32>(gp + 0x24, 0); // min_z
        wr::<i32>(gp + 0x28, 2); // max_x
        wr::<i32>(gp + 0x2c, 2); // max_y
        wr::<i32>(gp + 0x30, 2); // max_z

        let mut flags = [0u8; 128];
        // Set flag 2 at (x=1, y=2, z=1): idx = (1 * 4 + 1) * 4 + 2 = 22
        flags[22] = 2;
        // Set flag 8 at (x=0, y=1, z=0): idx = (0 * 4 + 0) * 4 + 1 = 1
        flags[1] = 8;

        // 128 cells of [f32; 3] = 384 floats, default f1 = 1.0 (non-solid since > -1e-5 and < MAX)
        let mut cells = [0.0f32; 384];
        for i in 0..128usize {
            cells[i * 3 + 1] = 1.0;
        }

        let mut p6 = [0u8; 0x30];
        let p6p = p6.as_mut_ptr() as usize;
        wr::<usize>(p6p + 0x08, cells.as_ptr() as usize);
        wr::<usize>(p6p + 0x18, flags.as_ptr() as usize);
        wr::<u8>(p6p + 0x20, 1);
        wr::<usize>(p6p + 0x28, gp);

        let p7: [f32; 3] = [10.0, 0.0, 2.0];
        let mut out = [0u8; 0x18];
        let op = out.as_mut_ptr() as usize;

        let (ret, _) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            op as u64,
            coord.as_ptr() as usize as u64,
            0,
            p4 as u64,
            p5 as u64,
            p6p as u64,
            p7.as_ptr() as usize as u64,
            0,
            [0, 0, radius.to_bits() as u128, 0],
        );
        assert_eq!(ret as usize, op);
        (
            rd::<u8>(op),
            rd::<i32>(op + 0x04),
            rd::<i32>(op + 0x08),
            rd::<i32>(op + 0x0c),
            rd::<u8>(op + 0x10),
        )
    };

    let cases = [
        ([-1, 1, 1], 1.0f32, 0u8, 0u8), // out of bounds
        ([1, 1, 1], 1.0f32, 0u8, 0u8),  // transition from solid at y=2 to open at y=1
        ([0, 1, 0], 1.0f32, 0u8, 1u8),  // hits flag 8 at y=1
        ([1, 2, 1], 0.0f32, 0u8, 0u8),  // all_solid at y=2
    ];

    for (idx, &(coord, r, p4, p5)) in cases.iter().enumerate() {
        let o = run(orig, coord, r, p4, p5);
        let n = run(new, coord, r, p4, p5);
        crate::diff_eq!(o, n, "1421377d0 case={}", idx);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_physics_142153950(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u64| -> [u32; 6] {
        // Two local AABBs of 24 bytes each ([min.x, min.y, min.z, max.x, max.y, max.z])
        let aabbs: [f32; 12] = [
            -1.0, -2.0, -3.0, 1.0, 2.0, 3.0, // valid AABB 0
            5.0, 5.0, 5.0, -5.0, -5.0, -5.0, // inverted AABB 1 (max < min -> skipped)
        ];
        let indices: [u32; 4] = [0, 0, 1, 0];

        let mut p40 = [0u8; 0x100];
        let p40p = p40.as_mut_ptr() as usize;
        wr::<usize>(p40p + 0x30, indices.as_ptr() as usize);
        wr::<u32>(p40p + 0x38, 2);

        // Transform: quaternion (0, 0, 0.70710678, 0.70710678), translation (10, 20, 30, 0)
        let transforms: [f32; 8] = [
            0.0,
            0.0,
            core::f32::consts::FRAC_1_SQRT_2,
            core::f32::consts::FRAC_1_SQRT_2,
            10.0,
            20.0,
            30.0,
            0.0,
        ];

        let mut p1 = [0u8; 0x130];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x40, p40p);
        wr::<usize>(p1p + 0xa0, aabbs.as_ptr() as usize);

        let p2: [u64; 2] = [count, transforms.as_ptr() as usize as u64];
        f(p1p, p2.as_ptr() as usize);
        rd::<[u32; 6]>(p1p + 0x110)
    };

    for cnt in [0u64, 1] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "142153950 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_physics_142155510(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHILDREN: [u16; 2] = [1, 2];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_3a60(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_5660(_p1: usize, out_vec: usize, idx: u16) {
        if idx == 0 {
            wr::<usize>(out_vec, core::ptr::addr_of!(CHILDREN) as usize);
            wr::<u32>(out_vec + 0x08, 2);
        }
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fb_3a60, mock_3a60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4215_5660, mock_5660 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F| -> ([u16; 4], u32, u32, u32) {
        FREE_COUNT = 0;
        // Node 0: non-leaf (arr[0]=0, arr[1]=5 -> diff != 0 -> expands children 1 and 2)
        // Node 1: leaf (arr[1]=5, arr[2]=5 -> diff == 0)
        // Node 2: last node (idx+1 == 3 == +0x18, +0x28 = 7, arr[2]=7 -> diff == 0)
        let arr: [u16; 3] = [0, 5, 7];
        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x10, arr.as_ptr() as usize);
        wr::<u32>(p1p + 0x18, 3);
        wr::<u16>(p1p + 0x28, 7);

        let mut out_items = [0u16; 4];
        let mut p2 = [0u8; 0x10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, out_items.as_mut_ptr() as usize);
        wr::<u32>(p2p + 0x08, 0);
        wr::<u32>(p2p + 0x0c, 1);

        f(p1p, p2p, 0);
        (
            out_items,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x0c),
            FREE_COUNT,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142155510 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421c2ef0(orig: usize, new: usize) -> Result<u32, String> {
    let ops: [u8; 6] = [0, 1, 2, 3, 5, 6];
    let offsets: [i32; 6] = [0, 8, 16, 40, 64, 72];

    // Pack f16 values (0x3c00 = 1.0, 0x4000 = 2.0, 0x4200 = 3.0, 0x3800 = 0.5)
    let mut buf_a = [0u16; 48];
    let mut buf_b = [0u16; 48];
    for i in 0..48usize {
        buf_a[i] = 0x3c00 + ((i as u16 % 4) * 0x0100);
        buf_b[i] = 0x3800 + (((i + 1) as u16 % 4) * 0x0100);
    }

    let weights: [[f32; 4]; 6] = [
        [0.5, 1.0, 1.5, 2.0],
        [0.25, 0.5, 0.75, 1.0],
        [1.0, 0.5, 0.25, 0.125],
        [0.2, 0.4, 0.6, 0.8],
        [1.0, 2.0, 3.0, 4.0],
        [0.3, 0.6, 0.9, 1.2],
    ];

    let mut p1 = [0u8; 0x18];
    let p1p = p1.as_mut_ptr() as usize;
    wr::<u8>(p1p + 0x07, 6);
    wr::<usize>(p1p + 0x08, ops.as_ptr() as usize);
    wr::<usize>(p1p + 0x10, offsets.as_ptr() as usize);

    let r_o = crate::simd_ops::call_va8_xmm(
        orig as u64,
        p1p as u64,
        buf_a.as_ptr() as usize as u64,
        buf_b.as_ptr() as usize as u64,
        weights.as_ptr() as usize as u64,
        0,
        0,
        0,
        0,
        [0; 4],
    )
    .1 as u32;
    let r_n = crate::simd_ops::call_va8_xmm(
        new as u64,
        p1p as u64,
        buf_a.as_ptr() as usize as u64,
        buf_b.as_ptr() as usize as u64,
        weights.as_ptr() as usize as u64,
        0,
        0,
        0,
        0,
        [0; 4],
    )
    .1 as u32;
    crate::diff_eq!(r_o, r_n, "1421c2ef0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421d4c70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG: [u64; 6] = [0; 6];
    static mut LOG_LEN: usize = 0;

    unsafe extern "win64" fn mock_cb0(p1: usize) {
        let i = LOG_LEN;
        LOG[i * 3] = rd::<u64>(p1);
        LOG[i * 3 + 1] = rd::<u64>(p1 + 0x08);
        LOG[i * 3 + 2] = rd::<u32>(p1 + 0x10) as u64;
        LOG_LEN = i + 1;
    }

    let run = |f: F, mask_b: u64| -> (u8, [u64; 6], [u64; 6]) {
        LOG = [0; 6];
        LOG_LEN = 0;

        // 3 entries of 12 bytes: (kind: u8, pad: [u8;3], off0: u32, off1: u32)
        let mut entries = [0u8; 36];
        let ep = entries.as_mut_ptr() as usize;
        wr::<u8>(ep, 0);
        wr::<u32>(ep + 0x04, 0x10);
        wr::<u32>(ep + 0x08, 0x20);

        wr::<u8>(ep + 12, 0);
        wr::<u32>(ep + 12 + 0x04, 0x30);
        wr::<u32>(ep + 12 + 0x08, 0x40);

        wr::<u8>(ep + 24, 0);
        wr::<u32>(ep + 24 + 0x04, 0x50);
        wr::<u32>(ep + 24 + 0x08, 0x60);

        let mut p160 = [0u8; 0x20];
        let p160p = p160.as_mut_ptr() as usize;
        wr::<usize>(p160p + 0x08, ep);
        wr::<u64>(p160p + 0x10, 0x1000);

        let mask_a: [u64; 1] = [0b101];
        let mut p168 = [0u8; 0xd0];
        let p168p = p168.as_mut_ptr() as usize;
        wr::<u16>(p168p + 0x02, 1);
        wr::<usize>(p168p + 0x08, mask_a.as_ptr() as usize);
        wr::<u64>(p168p + 0xc0, 0x2000);

        let mut p1 = [0u8; 0x170];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 0x500);
        wr::<u32>(p1p + 0x30, 0x40);
        wr::<usize>(p1p + 0x160, p160p);
        wr::<usize>(p1p + 0x168, p168p);

        let cb_table: [usize; 1] = [mock_cb0 as *const () as usize];
        let mask_b_arr: [u64; 1] = [mask_b];

        let ret = f(p1p, mask_b_arr.as_ptr() as usize, cb_table.as_ptr() as usize);
        (ret, rd::<[u64; 6]>(p1p), LOG)
    };

    for mb in [0u64, 0b111] {
        let o = run(f_o, mb);
        let n = run(f_n, mb);
        crate::diff_eq!(o, n, "1421d4c70 mb={}", mb);
    }
    Ok(2)
}

pub unsafe fn diff_puppet_1421db4a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;
    static mut FOUND_NODE: [u64; 4] = [0, 0, 0, 0x7777_8888];

    unsafe extern "win64" fn mock_68a0(_p: usize, p2: usize) {
        LOG_HASH = LOG_HASH.wrapping_add(p2 as u64);
    }
    unsafe extern "win64" fn mock_vf8(this: usize) {
        wr::<u32>(this + 0x84, 0x42);
    }
    unsafe extern "win64" fn mock_b380(_g: usize, hash_id: u32, view_ptr: usize) {
        let len = rd::<u64>(view_ptr + 0x08);
        LOG_HASH = LOG_HASH.wrapping_add((hash_id as u64) ^ (len << 16));
    }
    unsafe extern "win64" fn mock_9040(_lock: usize, _rdx: u64, _r8: u32) {
        let g_b78 = exe::ptr::<u64>(0x1_43a8_ab78) as usize;
        wr::<u64>(g_b78, rd::<u64>(g_b78).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_90c0(_g: usize, out_iter: usize, key_ptr: usize) -> usize {
        let k = rd::<u64>(key_ptr);
        if k == 0x1111 {
            wr::<usize>(out_iter, core::ptr::addr_of!(FOUND_NODE) as usize);
        } else {
            let sentinel = rd::<usize>(exe::ptr::<usize>(0x1_43a8_ab40) as usize);
            wr::<usize>(out_iter, sentinel);
        }
        out_iter
    }

    let _m1 = crate::difftest::Mock::new(0x1_4249_68a0, mock_68a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_b380, mock_b380 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4137_9040, mock_9040 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4137_90c0, mock_90c0 as *const ())?;

    let g_b40 = exe::ptr::<usize>(0x1_43a8_ab40) as usize;
    let g_b78 = exe::ptr::<u64>(0x1_43a8_ab78) as usize;
    let g_b80 = exe::ptr::<u64>(0x1_43a8_ab80) as usize;
    let saved = (rd::<usize>(g_b40), rd::<u64>(g_b78), rd::<u64>(g_b80));

    let run = |f: F| -> (u32, u32, u32, [u64; 2], u64, u64) {
        LOG_HASH = 0;
        wr::<usize>(g_b40, 0xdead_0000);
        wr::<u64>(g_b78, 10);
        wr::<u64>(g_b80, 0);

        let mut blob = [Align16([0; 16]); 16];
        let bp = blob.as_mut_ptr() as usize;
        // Aligned header at bp + 0x10:
        // +0x00: base_off = 0x20 (so rdx_base = bp + 0x30)
        // +0x08: reloc_count = 1
        // +0x0c: reloc[0] = 0x10 (adds rdx_base to [rdx_base + 0x10])
        wr::<u32>(bp + 0x10, 0x20);
        wr::<u32>(bp + 0x18, 1);
        wr::<u32>(bp + 0x1c, 0x10);

        let rdx_base = bp + 0x30;
        wr::<u16>(rdx_base + 0x06, 2); // 2 strings
        wr::<u64>(rdx_base + 0x10, 0x123); // relocated field
        wr::<i32>(rdx_base + 0x30, 0x10); // hashes at rdx_base + 0x40
        wr::<u32>(rdx_base + 0x40, 0x1111);
        wr::<u32>(rdx_base + 0x44, 0x2222);

        let s0 = b"joint_a\0";
        let s1 = b"joint_bb\0";
        let mut str_slots: [u64; 2] = [s0.as_ptr() as usize as u64, s1.as_ptr() as usize as u64];
        wr::<usize>(rdx_base + 0x88, str_slots.as_mut_ptr() as usize);

        let mut vtbl = [0usize; 32];
        vtbl[0xf8 / 8] = mock_vf8 as *const () as usize;

        let mut obj = [0u8; 0xd0];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op, vtbl.as_ptr() as usize);
        wr::<u32>(op + 0x80, 9);
        wr::<usize>(op + 0xc0, bp);

        let prev_state = f(op, 0x1234);
        let reloc_res = rd::<u64>(rdx_base + 0x10).wrapping_sub(rdx_base as u64);
        (
            prev_state,
            rd::<u32>(op + 0x80),
            rd::<u32>(op + 0x84),
            str_slots,
            reloc_res,
            LOG_HASH,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_b40, saved.0);
    wr::<u64>(g_b78, saved.1);
    wr::<u64>(g_b80, saved.2);

    crate::diff_eq!(o, n, "1421db4a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142206560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_listener_begin(this: usize) {
        TRACE = TRACE.wrapping_add(rd::<u64>(this + 0x08));
    }
    unsafe extern "win64" fn mock_listener_end(this: usize) {
        TRACE = TRACE.wrapping_add(rd::<u64>(this + 0x08) * 3);
    }
    unsafe extern "win64" fn mock_6920() -> i64 {
        1_000_000
    }
    unsafe extern "win64" fn mock_8b10(_item: usize, lod: u32) -> u8 {
        u8::from(lod != 99)
    }
    unsafe extern "win64" fn mock_d6f0(ctx: u64, _item: usize) {
        TRACE = TRACE.wrapping_add(ctx);
    }
    unsafe extern "win64" fn mock_f790(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_69a0() -> u64 {
        crate::simd_ops::set_xmm_ret(2.5f64.to_bits() as u128);
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_6920, mock_6920 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_8b10, mock_8b10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412d_f790, mock_f790 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_69a0, mock_69a0 as *const ())?;

    let g_2f0 = exe::ptr::<usize>(0x1_43a0_82f0) as usize;
    let g_698 = exe::ptr::<usize>(0x1_43a3_4698) as usize;
    let g_2b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let g_c98 = exe::ptr::<f32>(0x1_4386_7c98) as usize;
    let g_c38 = exe::ptr::<i32>(0x1_4386_7c38) as usize;
    let g_d30 = exe::ptr::<f32>(0x1_4386_7d30) as usize;
    let saved = (
        rd::<usize>(g_2f0),
        rd::<usize>(g_698),
        rd::<usize>(g_2b8),
        rd::<f32>(g_c98),
        rd::<i32>(g_c38),
        rd::<f32>(g_d30),
    );

    let run = |f: F| -> (u64, u32, u32, u64, u8, u8, u8, u32) {
        TRACE = 0;

        let mut l_vtbl = [0usize; 5];
        l_vtbl[3] = mock_listener_begin as *const () as usize;
        l_vtbl[4] = mock_listener_end as *const () as usize;
        let listener_obj: [u64; 2] = [l_vtbl.as_ptr() as usize as u64, 0x111];
        let listener_list: [usize; 1] = [listener_obj.as_ptr() as usize];

        let mut mgr_2f0 = [0u8; 0x500];
        let m2f0 = mgr_2f0.as_mut_ptr() as usize;
        wr::<usize>(m2f0 + 0x4e8, listener_list.as_ptr() as usize);
        wr::<u32>(m2f0 + 0x4f0, 1);
        wr::<usize>(g_2f0, m2f0);

        let mut sub_698 = [0u8; 0x220];
        wr::<u64>(sub_698.as_mut_ptr() as usize + 0x170, 1);
        wr::<u64>(sub_698.as_mut_ptr() as usize + 0x210, 0x1000);
        let mut obj_698 = [0u64; 4];
        obj_698[0] = 0x10_0000;
        obj_698[3] = sub_698.as_ptr() as usize as u64;
        wr::<usize>(g_698, obj_698.as_ptr() as usize);

        let mut obj_2b8 = [0u8; 0x230];
        let p2b8 = obj_2b8.as_mut_ptr() as usize;
        wr::<usize>(g_2b8, p2b8);

        wr::<f32>(g_c98, 0.1);
        wr::<i32>(g_c38, 10);
        wr::<f32>(g_d30, 0.5);

        let mut item0 = [0u8; 0x180];
        let i0 = item0.as_mut_ptr() as usize;
        wr::<u32>(i0 + 0x24, 0);
        wr::<u64>(i0 + 0x28, 0x500);
        wr::<u8>(i0 + 0xbb, 3);
        wr::<u8>(i0 + 0x158 + 2, 1);

        let mut item1 = [0u8; 0x180];
        let i1 = item1.as_mut_ptr() as usize;
        wr::<u32>(i1 + 0x24, 1);
        wr::<u8>(i1 + 0x158 + 4, 0);

        let mut item2 = [0u8; 0x180];
        let i2 = item2.as_mut_ptr() as usize;
        wr::<u8>(i2 + 0xbd, 1);
        wr::<u32>(i2 + 0xc8, 5);
        wr::<f32>(i2 + 0xcc, 1.0); // 2.5 - 1.0 = 1.5 >= 0.5 -> proceeds to enqueue

        let list_50: [usize; 1] = [i0];
        let b0_entries: [u64; 2] = [i1 as u64, 4];
        let b8_entries: [u64; 2] = [i2 as u64, 0];

        let mut p2 = [0u8; 0x120];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, b0_entries.as_ptr() as usize);
        wr::<u32>(p2p + 0x08, 1);
        wr::<usize>(p2p + 0x100, b8_entries.as_ptr() as usize);
        wr::<u32>(p2p + 0x108, 1);

        let mut q280 = [0u64; 16];
        let mut p1 = [0u8; 0x2a0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<f32>(p1p + 0x10, 0.0);
        wr::<u32>(p1p + 0x14, 5);
        wr::<u64>(p1p + 0x18, 100);
        wr::<u8>(p1p + 0x3a, 0);
        wr::<usize>(p1p + 0x50, list_50.as_ptr() as usize);
        wr::<u32>(p1p + 0x58, 1);
        wr::<usize>(p1p + 0x280, q280.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x288, 0);
        wr::<u32>(p1p + 0x28c, 8);

        f(p1p, p2p);
        (
            TRACE,
            rd::<u32>(p1p + 0x288),
            rd::<u32>(p1p + 0x290),
            rd::<u64>(p1p + 0x18),
            rd::<u8>(i0 + 0xbd),
            rd::<u8>(i1 + 0xbd),
            rd::<u8>(i2 + 0xbd),
            rd::<u8>(p2b8 + 0x221) as u32,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f0, saved.0);
    wr::<usize>(g_698, saved.1);
    wr::<usize>(g_2b8, saved.2);
    wr::<f32>(g_c98, saved.3);
    wr::<i32>(g_c38, saved.4);
    wr::<f32>(g_d30, saved.5);

    crate::diff_eq!(o, n, "142206560 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14220ee70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_B0: [u8; 0x100] = [0; 0x100];
    static mut BUF_B8: [u32; 256] = [0; 256];
    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_b250(p48: usize) -> i32 {
        rd::<i32>(p48)
    }
    unsafe extern "win64" fn mock_12e0(rcx_18: usize, _rdx: usize, r8_0c: usize) {
        TRACE = TRACE.wrapping_add((rcx_18 - r8_0c) as u64);
    }
    unsafe extern "win64" fn mock_0fa0(rcx_c8: usize, _rdx: usize, r8_18: usize) {
        TRACE = TRACE.wrapping_add((rcx_c8 - r8_18) as u64);
    }
    unsafe extern "win64" fn mock_aad0(
        _tex: usize,
        _rdx: usize,
        _mip: u32,
        mode: u32,
        out_map: usize,
    ) {
        TRACE = TRACE.wrapping_add(mode as u64);
        wr::<usize>(out_map, core::ptr::addr_of_mut!(BUF_B0) as usize);
        wr::<i32>(out_map + 0x08, 0xc0);
    }
    unsafe extern "win64" fn mock_a7e0(
        _tex: usize,
        _rdx: usize,
        mip: u32,
        out_map: usize,
    ) {
        TRACE = TRACE.wrapping_add((mip as u64) + (rd::<i32>(out_map + 0x08) as u64));
    }
    unsafe extern "win64" fn mock_get_desc(_res: usize, out_desc: usize) {
        wr::<u32>(out_desc, 3);
    }
    unsafe extern "win64" fn mock_map(
        _res: usize,
        _sub: u32,
        _range: usize,
        out_map: usize,
    ) -> u32 {
        wr::<usize>(out_map, core::ptr::addr_of_mut!(BUF_B8) as usize);
        0
    }
    unsafe extern "win64" fn mock_ecd0(p1: usize) {
        TRACE = TRACE.wrapping_add(rd::<u32>(p1 + 0xc4) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_b250, mock_b250 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4221_12e0, mock_12e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4221_0fa0, mock_0fa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4206_aad0, mock_aad0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4206_a7e0, mock_a7e0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4220_ecd0, mock_ecd0 as *const ())?;

    let run = |f: F| -> (u64, [u32; 8], [u32; 4], u8) {
        BUF_B0 = [0; 0x100];
        BUF_B8 = [0; 256];
        TRACE = 0;

        let mut res_vtbl = [0usize; 12];
        res_vtbl[0x40 / 8] = mock_map as *const () as usize;
        res_vtbl[0x50 / 8] = mock_get_desc as *const () as usize;
        let res_obj: [usize; 1] = [res_vtbl.as_ptr() as usize];

        let mut tex_b0 = [0u8; 0x80];
        let mut holder_b0 = [0u8; 0x20];
        wr::<usize>(holder_b0.as_mut_ptr() as usize, tex_b0.as_mut_ptr() as usize);
        wr::<i32>(holder_b0.as_mut_ptr() as usize + 0x0c, 4);

        let mut tex_b8 = [0u8; 0x80];
        let tb8 = tex_b8.as_mut_ptr() as usize;
        wr::<i32>(tb8 + 0x0c, 4);
        wr::<i32>(tb8 + 0x20, 1024);
        wr::<u32>(tb8 + 0x24, 1024);
        wr::<u8>(tb8 + 0x28, 0x10);
        wr::<usize>(tb8 + 0x70, res_obj.as_ptr() as usize);
        let mut holder_b8 = [0u8; 0x20];
        wr::<usize>(holder_b8.as_mut_ptr() as usize, tb8);
        wr::<i32>(holder_b8.as_mut_ptr() as usize + 0x0c, 4);

        let mut item = vec![0u8; 0xd00];
        let ip = item.as_mut_ptr() as usize;
        wr::<u32>(ip, 0); // slot 0
        wr::<f32>(ip + 0x04, 250.0);
        wr::<u32>(ip + 0x08, 0x1234_5678);
        wr::<u64>(ip + 0x18, 0xaaaa_bbbb_cccc_dddd);
        // Set first two RGB triples at +0xc8 (one below sRGB threshold, one above)
        wr::<[f32; 3]>(ip + 0xc8, [0.001, 0.002, 0.5]);
        wr::<[f32; 3]>(ip + 0xc8 + 12, [0.25, 0.75, 1.2]);

        let buckets: [u64; 4] = [u64::MAX, 0, 42, ip as u64];
        let p48_val: i32 = 1;

        let mut p1 = [0u8; 0xd0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x18, 0);
        wr::<usize>(p1p + 0x48, (&p48_val as *const i32) as usize);
        wr::<u64>(p1p + 0x78, u64::MAX - 1);
        wr::<u64>(p1p + 0x80, 0);
        wr::<u64>(p1p + 0x90, 2);
        wr::<u64>(p1p + 0x98, u64::MAX);
        wr::<usize>(p1p + 0xa8, buckets.as_ptr() as usize);
        wr::<usize>(p1p + 0xb0, holder_b0.as_ptr() as usize);
        wr::<usize>(p1p + 0xb8, holder_b8.as_ptr() as usize);
        wr::<u8>(p1p + 0xc0, 1);

        f(p1p);
        let b0_head = rd::<[u32; 8]>(core::ptr::addr_of!(BUF_B0) as usize);
        let b8_head = [BUF_B8[0], BUF_B8[1], BUF_B8[2], BUF_B8[255]];
        (TRACE, b0_head, b8_head, rd::<u8>(p1p + 0xc0))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14220ee70 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142211a90(orig: usize, new: usize) -> Result<u32, String> {
    let cases: [(f32, f32, f32); 4] = [
        (-0.001, 0.5, 1.0),
        (0.25, 0.1, 0.5),
        (1.0, 2.0, 0.75),
        (0.0, 0.05, 0.01),
    ];

    for (idx, &(a, b, c)) in cases.iter().enumerate() {
        let r_o = crate::simd_ops::call_va8_xmm(
            orig as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [
                a.to_bits() as u128,
                b.to_bits() as u128,
                c.to_bits() as u128,
                0,
            ],
        )
        .1 as u32;
        let r_n = crate::simd_ops::call_va8_xmm(
            new as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [
                a.to_bits() as u128,
                b.to_bits() as u128,
                c.to_bits() as u128,
                0,
            ],
        )
        .1 as u32;
        crate::diff_eq!(r_o, r_n, "142211a90 case={}", idx);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_rend_142222300(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_50: [u64; 4] = [0; 4];
    static mut BUF_60: [usize; 4] = [0; 4];
    static mut BUF_70: [usize; 4] = [0; 4];
    static mut A6A0_CALL: u32 = 0;
    static mut CALL_SEQ: u32 = 0;
    static mut SUCCEED_AT: u32 = 0;
    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_ec00(_rcx: u32, count: u64) -> usize {
        TRACE = TRACE.wrapping_add(count);
        core::ptr::addr_of_mut!(BUF_50) as usize
    }
    unsafe extern "win64" fn mock_a6a0(vec_ptr: usize, count: u64) {
        let is_60 = A6A0_CALL == 1;
        A6A0_CALL += 1;
        let ptr = if is_60 {
            BUF_60 = [0x6001, 0, 0x6003, 0];
            core::ptr::addr_of_mut!(BUF_60) as usize
        } else {
            BUF_70 = [0, 0x7002, 0, 0x7004];
            core::ptr::addr_of_mut!(BUF_70) as usize
        };
        wr::<usize>(vec_ptr, ptr);
        wr::<u32>(vec_ptr + 0x08, count as u32);
        wr::<u32>(vec_ptr + 0x0c, count as u32);
        TRACE = TRACE.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_e030(_fmt: usize, _base: usize, w: u32, h: u32) -> usize {
        TRACE = TRACE.wrapping_add((w as u64) * 100 + (h as u64));
        b"precalc_path\0".as_ptr() as usize
    }
    unsafe extern "win64" fn mock_9b50(sb: usize, view: usize, _flag: u32) {
        let len = rd::<u64>(view + 0x08);
        TRACE = TRACE.wrapping_add(len);
        wr::<u32>(sb, 0x2000_0000 | (len as u32));
    }
    unsafe extern "win64" fn mock_08c0(
        _rbx: usize,
        sb: usize,
        _r8: u32,
        w: u32,
        h: u32,
        flag: u8,
    ) -> usize {
        TRACE = TRACE.wrapping_add((w as u64) ^ (h as u64) ^ (flag as u64));
        sb
    }
    unsafe extern "win64" fn mock_0df0(
        _rbx: usize,
        w: u32,
        h: u32,
        _p60: usize,
        _p70: usize,
        _p50: usize,
        cnt: u32,
        _sb: usize,
    ) -> u8 {
        CALL_SEQ += 1;
        TRACE = TRACE.wrapping_add((w as u64) + (h as u64) + (cnt as u64));
        u8::from(CALL_SEQ == SUCCEED_AT)
    }
    unsafe extern "win64" fn mock_ab10(_sb: usize) {
        TRACE = TRACE.wrapping_add(0xab10);
    }
    unsafe extern "win64" fn mock_fc20(
        _rcx: usize,
        _rdx: u32,
        w: u32,
        h: u32,
        flag8: u8,
        _p60: usize,
        _p70: usize,
        _p50: usize,
        cnt: u32,
    ) {
        TRACE = TRACE.wrapping_add(0xfc200 + (w as u64) + (h as u64) + (flag8 as u64) + (cnt as u64));
    }
    unsafe extern "win64" fn mock_f780(
        _rcx: usize,
        _p60: usize,
        _p70: usize,
        _p50: usize,
        cnt: u32,
    ) {
        TRACE = TRACE.wrapping_add(0xf7800 + (cnt as u64));
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        TRACE = TRACE.wrapping_add(ptr as u64);
    }
    unsafe extern "win64" fn mock_59a0(_vec: usize) {
        TRACE = TRACE.wrapping_add(0x59a0);
    }
    unsafe extern "win64" fn mock_e830(_vec: usize) {
        TRACE = TRACE.wrapping_add(0xe830);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ec00, mock_ec00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_a6a0, mock_a6a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_423f_e030, mock_e030 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4222_08c0, mock_08c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4222_0df0, mock_0df0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4221_fc20, mock_fc20 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4221_f780, mock_f780 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401e_59a0, mock_59a0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let g_fe0 = exe::ptr::<u8>(0x1_4386_7fe0) as usize;
    let s_fe0 = rd::<u8>(g_fe0);

    let run = |f: F, fe0: u8, succeed_at: u32, flag_31: u8| -> (u64, u32) {
        BUF_50 = [0xdead; 4];
        A6A0_CALL = 0;
        CALL_SEQ = 0;
        SUCCEED_AT = succeed_at;
        TRACE = 0;
        wr::<u8>(g_fe0, fe0);

        let mut inner = [0u8; 0x180];
        let ip = inner.as_mut_ptr() as usize;
        wr::<u8>(ip + 0x31, flag_31);
        wr::<u32>(ip + 0x170, 1920);
        wr::<u32>(ip + 0x174, 1080);

        let mut p1 = [0u8; 0x10];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, ip);
        wr::<u8>(p1p + 0x08, 0);

        f(p1p);
        (TRACE, CALL_SEQ)
    };

    for (fe0, suc, f31) in [(1u8, 2u32, 1u8), (1, 99, 0)] {
        let o = run(f_o, fe0, suc, f31);
        let n = run(f_n, fe0, suc, f31);
        wr::<u8>(g_fe0, s_fe0);
        crate::diff_eq!(o, n, "142222300 fe0={} suc={} f31={}", fe0, suc, f31);
    }
    Ok(2)
}

// REGISTRY (for games/alan-wake-2/crates/aw2-game/src/registry/core.rs):
// FLOAT_RETURN:
//   0x1_421c_2ef0 (`unknown_1421c2ef0` returns `f32` in `xmm0`)
//   0x1_4221_1a90 (`unknown_142211a90` returns `f32` in `xmm0`)
// Replacement::verified(0x1_420c_ba20, "unknown_1420cba20", rend::cauldron_hot_x5::unknown_1420cba20 as *const (), rend::cauldron_hot_x5::diff_unknown_1420cba20),
// Replacement::verified_throws(0x1_420e_3ef0, "unknown_1420e3ef0", rend::cauldron_hot_x5::unknown_1420e3ef0 as *const (), Throws::Caught, rend::cauldron_hot_x5::diff_unknown_1420e3ef0),
// Replacement::verified(0x1_4213_77d0, "unknown_1421377d0", rend::cauldron_hot_x5::unknown_1421377d0 as *const (), rend::cauldron_hot_x5::diff_unknown_1421377d0),
// Replacement::verified(0x1_4215_3950, "physics_142153950", rend::cauldron_hot_x5::physics_142153950 as *const (), rend::cauldron_hot_x5::diff_physics_142153950),
// Replacement::verified_throws(0x1_4215_5510, "physics_142155510", rend::cauldron_hot_x5::physics_142155510 as *const (), Throws::Caught, rend::cauldron_hot_x5::diff_physics_142155510),
// Replacement::verified(0x1_421c_2ef0, "unknown_1421c2ef0", rend::cauldron_hot_x5::unknown_1421c2ef0 as *const (), rend::cauldron_hot_x5::diff_unknown_1421c2ef0),
// Replacement::verified(0x1_421d_4c70, "unknown_1421d4c70", rend::cauldron_hot_x5::unknown_1421d4c70 as *const (), rend::cauldron_hot_x5::diff_unknown_1421d4c70),
// Replacement::verified_throws(0x1_421d_b4a0, "puppet_1421db4a0", rend::cauldron_hot_x5::puppet_1421db4a0 as *const (), Throws::Discard, rend::cauldron_hot_x5::diff_puppet_1421db4a0),
// Replacement::verified_throws(0x1_4220_6560, "unknown_142206560", rend::cauldron_hot_x5::unknown_142206560 as *const (), Throws::Caught, rend::cauldron_hot_x5::diff_unknown_142206560),
// Replacement::verified_throws(0x1_4220_ee70, "unknown_14220ee70", rend::cauldron_hot_x5::unknown_14220ee70 as *const (), Throws::Caught, rend::cauldron_hot_x5::diff_unknown_14220ee70),
// Replacement::verified(0x1_4221_1a90, "unknown_142211a90", rend::cauldron_hot_x5::unknown_142211a90 as *const (), rend::cauldron_hot_x5::diff_unknown_142211a90),
// Replacement::verified_throws(0x1_4222_2300, "rend_142222300", rend::cauldron_hot_x5::rend_142222300 as *const (), Throws::Caught, rend::cauldron_hot_x5::diff_rend_142222300),
