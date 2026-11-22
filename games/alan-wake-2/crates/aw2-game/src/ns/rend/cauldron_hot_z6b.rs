//! Decompiled `rend` hot functions shard Z6B (`0x1421c5230..0x1421defd0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vblendvps, vbroadcastss, vcmpps, vdivps, vdpps, vmulps, vpermilps, vsqrtps, vsubps,
};

#[repr(C, align(16))]
#[derive(Clone, Copy)]
struct Align16([u8; 16]);

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
unsafe fn vrcpps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::{__m128, _mm_rcp_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rcp_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrcpps(a)
    }
}

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
unsafe fn cvt_f32_bits_to_f16(f_bits: u32) -> u16 {
    #[cfg(target_arch = "x86_64")]
    {
        let out: u64;
        core::arch::asm!(
            "vmovd {x}, {inp:e}",
            "vbroadcastss {x}, {x}",
            "vcvtps2ph {x}, {x}, 0",
            "vmovq {out}, {x}",
            x = out(xmm_reg) _,
            inp = in(reg) f_bits,
            out = out(reg) out,
            options(pure, nomem, nostack, preserves_flags),
        );
        out as u16
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vcvtps2ph(vbroadcastss(f_bits)) as u16
    }
}

#[inline(always)]
unsafe fn call_va_xmm(
    va: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    xmm: [u128; 4],
) -> (u64, u128) {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::lookup_native_va(va).unwrap_or_else(|| exe::to_mapped_addr(va));
        crate::simd_ops::set_tls_xmm_in(xmm);
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        let mut out_x0 = 0u128;
        core::arch::asm!(
            "movups xmm0, [{xin} + 0x00]",
            "movups xmm1, [{xin} + 0x10]",
            "movups xmm2, [{xin} + 0x20]",
            "movups xmm3, [{xin} + 0x30]",
            xin = in(reg) xmm.as_ptr(),
            options(nostack, preserves_flags),
        );
        let r = f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
        core::arch::asm!(
            "movups [{xout}], xmm0",
            xout = in(reg) &mut out_x0,
            options(nostack, preserves_flags),
        );
        (r, crate::simd_ops::take_tls_xmm_out(out_x0))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::call_va8_xmm(va, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, xmm)
    }
}

/// `unknown_1421c5230`
/// Original: 0x1421c5230 (253 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421c5230(
    param_1: usize,
    param_2: usize,
    param_3: i32,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) -> i32 {
    let mut best_idx: i32 = -1;
    let base_ptr = rd::<usize>(param_5 + 0x08);
    let mut best_dist = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd0) as usize);

    if param_3 > 0 {
        for i in 0..(param_3 as usize) {
            let cand_idx = rd::<i32>(param_4.wrapping_add(i * 4));
            let prev_best = best_dist;
            let stride = rd::<i32>(param_1);
            let elem_off = cand_idx.wrapping_mul(stride) as isize as usize;
            let elem_ptr = base_ptr.wrapping_add(elem_off);
            let (_, x0) = call_va_xmm(
                0x1_421c_2ef0,
                param_1 as u64,
                param_2 as u64,
                elem_ptr as u64,
                param_6 as u64,
                0,
                0,
                0,
                0,
                [0; 4],
            );
            let dist = f32::from_bits(x0 as u32);
            if prev_best > dist {
                best_idx = cand_idx;
            }
            if dist < best_dist {
                best_dist = dist;
            }
        }
    }

    wr::<f32>(param_7, best_dist);
    best_idx
}

/// `unknown_1421c5660`
/// Original: 0x1421c5660 (994 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421c5660(param_1: usize, param_2: usize, param_3: usize) {
    let p_60 = rd::<usize>(param_3 + 0x60);
    let r12 = rd::<usize>(rd::<usize>(param_1));

    if p_60 == 0 {
        let p_58 = rd::<usize>(r12 + 0x58);
        wr::<u16>(param_2, rd::<u16>(p_58 + 0x04));
        wr::<u8>(param_2 + 0x02, rd::<u8>(p_58 + 0x07));
        wr::<u32>(param_2 + 0x04, rd::<u32>(p_58));
        wr::<u32>(param_2 + 0x08, 0);
    } else {
        let r14 = rd::<usize>(r12 + 0x40);
        let size1 = rd::<i32>(r14) as isize as usize;
        let q1 = (size1 + 15) / 16;
        let mut stack_buf1 = [Align16([0; 16]); 64];
        let mut heap_buf1: Vec<Align16>;
        let buf1_ptr = if q1 <= 64 {
            stack_buf1.as_mut_ptr() as usize
        } else {
            heap_buf1 = vec![Align16([0; 16]); q1];
            heap_buf1.as_mut_ptr() as usize
        };

        let rbx_10 = rd::<u64>(param_1 + 0x10);
        let esi_40 = rd::<u32>(rd::<usize>(param_2 + 0x40));
        let edi_48 = rd::<u32>(rd::<usize>(param_2 + 0x48));

        let mut frame = [Align16([0; 16]); 6];
        let fp = frame.as_mut_ptr() as usize;
        let ptr_00 = fp;
        let ptr_10 = fp + 0x10;
        let ptr_20 = fp + 0x20;
        let ptr_40 = fp + 0x40;

        exe::call_va!(0x1_421c_6440, (ptr_00, param_1, param_3));

        let arg5_110 = rd::<u64>(param_3 + 0x110);
        let arg6_100 = rd::<u64>(param_3 + 0x100);
        let r8_58 = rd::<u64>(param_3 + 0x58);
        exe::call_va!(
            0x1_421c_47f0,
            (r14, buf1_ptr, r8_58, rbx_10, arg5_110, arg6_100)
        );

        let saved_xmm6 = rd::<u128>(ptr_00);
        wr::<u128>(ptr_10, rd::<u128>(param_3 + 0xd0));
        wr::<[u128; 2]>(ptr_20, rd::<[u128; 2]>(param_3 + 0x90));
        wr::<[u128; 2]>(ptr_40, rd::<[u128; 2]>(param_3 + 0x70));

        exe::call_va!(
            0x1_421c_3150,
            (
                r14, buf1_ptr, p_60, ptr_40, ptr_20, ptr_00, ptr_10, esi_40, edi_48
            )
        );

        let idx_1000 = rd::<i32>(p_60 + 0x1000) as isize;
        let cl = rd::<u8>(p_60.wrapping_add((idx_1000 * 64) as usize + 0x30));
        let sil: u8 = if (cl & rd::<u8>(r12 + 0x4a)) == cl {
            cl
        } else {
            0xff
        };

        let count_7 = (rd::<u8>(r14 + 0x07) as i8) as i32;
        let size2 = ((count_7 << 2) as u32 as usize) * 4;
        let q2 = (size2 + 15) / 16;
        let mut stack_buf2 = [Align16([0; 16]); 64];
        let mut heap_buf2: Vec<Align16>;
        let buf2_ptr = if q2 <= 64 {
            stack_buf2.as_mut_ptr() as usize
        } else {
            heap_buf2 = vec![Align16([0; 16]); q2];
            heap_buf2.as_mut_ptr() as usize
        };

        let xmm1_28 = rd::<u32>(param_1 + 0x28);
        let _ = call_va_xmm(
            0x1_421c_2d10,
            r14 as u64,
            0,
            buf2_ptr as u64,
            0,
            0,
            0,
            0,
            0,
            [0, xmm1_28 as u128, 0, 0],
        );

        let r10 = (rd::<u8>(r14 + 0x06) as i8) as isize;
        let r11 = rd::<usize>(r14 + 0x18);
        if r10 > 0 {
            for i in 0..(r10 as usize) {
                let arr_20 = rd::<usize>(r14 + 0x20);
                if rd::<u8>(arr_20.wrapping_add(i)) == 6 {
                    let dst_idx = rd::<i32>(r11.wrapping_add(i * 4)) as isize;
                    wr::<u32>(buf2_ptr.wrapping_add((dst_idx * 4) as usize), 0);
                }
            }
        }

        if sil == 1 {
            exe::call_va!(0x1_421c_2b70, (r14, buf2_ptr));
        }

        exe::call_va!(
            0x1_421c_2660,
            (r12, ptr_00, buf1_ptr, sil as u32, buf2_ptr)
        );

        let r14_idx = rd::<i32>(ptr_00) as isize;
        let arr_58 = rd::<usize>(r12 + 0x58);
        let entry_58 = arr_58.wrapping_add((r14_idx * 8) as usize);
        let esi_state = rd::<i16>(entry_58 + 0x04);
        let xmm3_t = rd::<f32>(param_3 + 0x48) + rd::<f32>(entry_58);
        let mut out_c0: u64 = 0;
        let (al_ok, _) = call_va_xmm(
            0x1_421c_2760,
            r12 as u64,
            (&mut out_c0 as *mut u64) as u64,
            esi_state as i32 as i64 as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, xmm3_t.to_bits() as u128],
        );

        let (final_si, final_al, final_xmm2_bits) = if (al_ok as u8) != 0 {
            let si = (out_c0 >> 32) as i16;
            let al = (out_c0 >> 56) as u8;
            let x2 = out_c0 as u32;
            wr::<i16>(param_2, si);
            wr::<u8>(param_2 + 0x02, al);
            (si, al, x2)
        } else {
            let al = rd::<u8>(entry_58 + 0x07);
            let x2 = rd::<u32>(entry_58);
            wr::<i16>(param_2, esi_state);
            wr::<u8>(param_2 + 0x02, al);
            (esi_state, al, x2)
        };
        wr::<u32>(param_2 + 0x04, final_xmm2_bits);

        if rd::<f32>(param_1 + 0x38) > 0.0f32 && (final_al & 1) == 0 {
            let xmm1_3c = rd::<u32>(param_1 + 0x3c);
            let xmm0_48 = rd::<u32>(param_3 + 0x48);
            let tbl_8 = rd::<usize>(rd::<usize>(param_1) + 0x08);
            let r9_val = rd::<u64>(tbl_8.wrapping_add((final_si as isize as usize) * 8));
            wr::<u128>(ptr_10, saved_xmm6);
            let (_, x0_6680) = call_va_xmm(
                0x1_421c_6680,
                param_3 as u64,
                p_60 as u64,
                ptr_10 as u64,
                r9_val,
                final_xmm2_bits as u64,
                xmm1_3c as u64,
                xmm0_48 as u64,
                0,
                [0; 4],
            );
            wr::<u32>(param_2 + 0x08, x0_6680 as u32);
        }
    }

    wr::<u32>(param_2 + 0x0c, 0);
    if (rd::<u8>(param_1 + 0x46) & 2) != 0 {
        wr::<i32>(param_2 + 0x50, rd::<i16>(param_2) as i32);
        wr::<u32>(param_2 + 0x54, rd::<u32>(param_2 + 0x04));
        wr::<u32>(param_2 + 0x58, 8);
        wr::<u32>(param_2 + 0x5c, 8);
    }
}

/// `unknown_1421c5a50`
/// Original: 0x1421c5a50 (2526 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421c5a50(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u32,
) {
    let rdi_40 = rd::<usize>(param_2 + 0x40);
    let count_7 = (rd::<u8>(rdi_40 + 0x07) as i8) as i32;
    let size1 = ((count_7 << 2) as u32 as usize) * 4;
    let q1 = (size1 + 15) / 16;
    let mut stack_buf1 = [Align16([0; 16]); 64];
    let mut heap_buf1: Vec<Align16>;
    let r15_buf = if q1 <= 64 {
        stack_buf1.as_mut_ptr() as usize
    } else {
        heap_buf1 = vec![Align16([0; 16]); q1];
        heap_buf1.as_mut_ptr() as usize
    };

    let xmm1_28 = rd::<u32>(param_4 + 0x28);
    let _ = call_va_xmm(
        0x1_421c_2d10,
        rdi_40 as u64,
        0,
        r15_buf as u64,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1_28 as u128, 0, 0],
    );

    let r10_cnt = rd::<u8>(param_4 + 0x21) as usize;
    let r11_map = rd::<usize>(rdi_40 + 0x18);
    for i in 0..r10_cnt {
        let b_idx = rd::<u8>(rd::<usize>(param_4 + 0x18).wrapping_add(i)) as usize;
        let d_idx = rd::<i32>(r11_map.wrapping_add(b_idx * 4)) as isize;
        let scale_ptr = rd::<usize>(rd::<usize>(param_5 + 0x38).wrapping_add(i * 8));
        let scale = rd::<f32>(scale_ptr);
        let slot = r15_buf.wrapping_add((d_idx * 4) as usize);
        wr::<f32>(slot, scale * rd::<f32>(slot));
    }

    let idx_1000 = rd::<i32>(param_3 + 0x1000) as isize;
    let raw_sil = rd::<u8>(param_3.wrapping_add((idx_1000 * 64) as usize + 0x30));
    let sil: u8 = if (rd::<u8>(param_2 + 0x4a) & raw_sil) == raw_sil {
        if raw_sil == 1 {
            exe::call_va!(0x1_421c_2b70, (rdi_40, r15_buf));
        }
        raw_sil
    } else {
        0xff
    };

    let mut frame = [Align16([0; 16]); 9];
    let fp = frame.as_mut_ptr() as usize;
    let ptr_10 = fp + 0x10;
    let ptr_30 = fp + 0x30;
    let ptr_40 = fp + 0x40;
    let ptr_50 = fp + 0x50;
    let ptr_60 = fp + 0x60;
    let ptr_70 = fp + 0x70;
    let ptr_80 = fp + 0x80;

    exe::call_va!(0x1_421c_6440, (ptr_10, param_4, param_1));

    let buf_sz = rd::<i32>(rdi_40) as isize as usize;
    let qb = (buf_sz + 15) / 16;
    let mut stack_buf20 = [Align16([0; 16]); 64];
    let mut stack_buf30 = [Align16([0; 16]); 64];
    let mut heap_buf20: Vec<Align16>;
    let mut heap_buf30: Vec<Align16>;
    let (buf20_ptr, buf30_ptr) = if qb <= 64 {
        (
            stack_buf20.as_mut_ptr() as usize,
            stack_buf30.as_mut_ptr() as usize,
        )
    } else {
        heap_buf20 = vec![Align16([0; 16]); qb];
        heap_buf30 = vec![Align16([0; 16]); qb];
        (
            heap_buf20.as_mut_ptr() as usize,
            heap_buf30.as_mut_ptr() as usize,
        )
    };

    let mut r8_state = rd::<i16>(param_5) as i32;
    let mut xmm4_t = rd::<f32>(param_5 + 0x04);
    let arr_18 = rd::<usize>(param_2 + 0x18);
    let ent_18 = arr_18.wrapping_add((r8_state as isize as usize) * 0x14);
    let dur_0 = rd::<f32>(ent_18 + 0x04);
    if xmm4_t >= dur_0 {
        let next_st = rd::<i16>(ent_18 + 0x0e);
        if next_st != -1 {
            xmm4_t = (xmm4_t - dur_0) + rd::<f32>(ent_18 + 0x10);
            r8_state = next_st as i32;
        } else {
            xmm4_t = dur_0;
        }
    }

    let xmm3_fps = rd::<f32>(param_2);
    let rbx_40 = rd::<usize>(param_2 + 0x40);
    let r9_st = r8_state as isize;
    let r11d_stride = rd::<i32>(rbx_40);
    let max_frame = (rd::<i16>(arr_18.wrapping_add((r9_st as usize) * 0x14 + 0x0c)) as i32) - 1;
    let fl_frame = crate::simd_ops::cvt_f32_i32((xmm4_t * xmm3_fps).floor()) as i32;
    let esi_f0 = if fl_frame < max_frame {
        fl_frame
    } else {
        max_frame
    };
    let edi_f1 = if esi_f0 + 1 < max_frame {
        esi_f0 + 1
    } else {
        max_frame
    };

    let arr_20 = rd::<usize>(param_2 + 0x20).wrapping_add((r9_st as usize) * 0x18);
    let r10_data = rd::<usize>(arr_20 + 0x08);
    let rax_times = rd::<usize>(arr_20 + 0x10);
    let frac_t = (xmm4_t - rd::<f32>(rax_times.wrapping_add((esi_f0 as isize as usize) * 8)))
        * xmm3_fps;
    let off_f0 = r11d_stride.wrapping_mul(esi_f0) as isize as usize;
    let off_f1 = edi_f1.wrapping_mul(r11d_stride) as isize as usize;
    exe::call_va!(
        0x1_421c_5420,
        (
            rbx_40,
            buf20_ptr,
            r10_data.wrapping_add(off_f0),
            r10_data.wrapping_add(off_f1),
            frac_t.to_bits()
        )
    );

    let copy_len = rd::<i32>(rdi_40) as isize as usize;
    core::ptr::copy_nonoverlapping(buf20_ptr as *const u8, buf30_ptr as *mut u8, copy_len);

    let xmm9_saved = rd::<u128>(ptr_10);
    wr::<u128>(ptr_30, rd::<u128>(param_1 + 0xd0));
    wr::<[u128; 2]>(ptr_50, rd::<[u128; 2]>(param_1 + 0x70));
    wr::<[u128; 2]>(ptr_70, rd::<[u128; 2]>(param_1 + 0x90));
    let arg8_40 = rd::<u32>(rd::<usize>(param_5 + 0x40));
    let arg9_48 = rd::<u32>(rd::<usize>(param_5 + 0x48));
    exe::call_va!(
        0x1_421c_3150,
        (
            rdi_40, buf30_ptr, param_3, ptr_50, ptr_70, ptr_10, ptr_30, arg8_40, arg9_48
        )
    );

    let cnt_21 = rd::<u8>(param_4 + 0x21) as usize;
    for i in 0..cnt_21 {
        let f_ptr = rd::<usize>(rd::<usize>(param_5 + 0x30).wrapping_add(i * 8));
        let h16 = cvt_f32_bits_to_f16(rd::<u32>(f_ptr));
        let b_idx = rd::<u8>(rd::<usize>(param_4 + 0x18).wrapping_add(i)) as usize;
        let off = rd::<i32>(rd::<usize>(rdi_40 + 0x28).wrapping_add(b_idx * 4)) as isize;
        wr::<u16>(buf30_ptr.wrapping_add(off as usize), h16);
    }

    let xmm6_dt = f32::from_bits(param_6);
    let cur_state = rd::<i16>(param_5) as i32;
    let next_t = xmm6_dt + rd::<f32>(param_5 + 0x04);
    let mut out_148: u64 = 0;
    let (al_ok1, _) = call_va_xmm(
        0x1_421c_2760,
        param_2 as u64,
        (&mut out_148 as *mut u64) as u64,
        cur_state as i64 as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 0, next_t.to_bits() as u128],
    );

    let cond_direct = (al_ok1 as u8) != 0
        && (sil == 0xff || (((out_148 >> 48) as u8) & sil) == sil);
    let (_, x0_cost) = call_va_xmm(
        0x1_421c_2ef0,
        rdi_40 as u64,
        buf30_ptr as u64,
        buf20_ptr as u64,
        r15_buf as u64,
        0,
        0,
        0,
        0,
        [0; 4],
    );
    let xmm7_cost = f32::from_bits(x0_cost as u32);

    let (final_cx, final_al, final_xmm1_bits) =
        if cond_direct && !(rd::<f32>(param_4 + 0x34) < xmm7_cost) {
            let x1 = out_148 as u32;
            wr::<u32>(param_5 + 0x04, x1);
            ((out_148 >> 32) as u16, (out_148 >> 56) as u8, x1)
        } else {
            let mut out_30: u64 = 0;
            exe::call_va!(
                0x1_421c_2660,
                (
                    param_2,
                    (&mut out_30 as *mut u64) as usize,
                    buf30_ptr,
                    sil as u32,
                    r15_buf
                )
            );
            let cand_idx = (out_30 as i32) as isize;
            let arr_58 = rd::<usize>(param_2 + 0x58);
            let ent_58 = arr_58.wrapping_add((cand_idx * 8) as usize);
            let cand_t = xmm6_dt + rd::<f32>(ent_58);
            let cand_st = rd::<i16>(ent_58 + 0x04) as i32;
            let mut out_138: u64 = 0;
            let (al_ok2, _) = call_va_xmm(
                0x1_421c_2760,
                param_2 as u64,
                (&mut out_138 as *mut u64) as u64,
                cand_st as i64 as u64,
                0,
                0,
                0,
                0,
                0,
                [0, 0, 0, cand_t.to_bits() as u128],
            );
            let best_cost = f32::from_bits((out_30 >> 32) as u32);
            if (al_ok2 as u8) != 0
                && (!cond_direct || (best_cost + rd::<f32>(param_4 + 0x34)) < xmm7_cost)
            {
                let x1 = out_138 as u32;
                wr::<u32>(param_5 + 0x04, x1);
                ((out_138 >> 32) as u16, (out_138 >> 56) as u8, x1)
            } else {
                let x1 = out_148 as u32;
                wr::<u32>(param_5 + 0x04, x1);
                ((out_148 >> 32) as u16, (out_148 >> 56) as u8, x1)
            }
        };

    wr::<u16>(param_5, final_cx);
    wr::<u8>(param_5 + 0x02, final_al);

    if rd::<f32>(param_4 + 0x38) > 0.0f32 && (final_al & 1) == 0 {
        let tbl_8 = rd::<usize>(rd::<usize>(param_4) + 0x08);
        let r9_val = rd::<u64>(tbl_8.wrapping_add((final_cx as i16 as isize as usize) * 8));
        let xmm0_3c = rd::<u32>(param_4 + 0x3c);
        wr::<u128>(ptr_10, xmm9_saved);
        let (_, x0_6680) = call_va_xmm(
            0x1_421c_6680,
            param_1 as u64,
            param_3 as u64,
            ptr_10 as u64,
            r9_val,
            final_xmm1_bits as u64,
            xmm0_3c as u64,
            param_6 as u64,
            0,
            [0; 4],
        );
        let xmm1_prev = if (rd::<u8>(param_4 + 0x46) & 8) != 0 {
            0u32
        } else {
            rd::<u32>(param_5 + 0x08)
        };
        let rcp = vrcpss_f32(rd::<f32>(param_4 + 0x38));
        let xmm0_rcp = if !rcp.is_nan() { rcp.to_bits() } else { 0u32 };
        let (_, x0_e8c0) = call_va_xmm(
            0x1_4102_e8c0,
            0,
            0,
            0,
            (param_5 + 0x0c) as u64,
            param_6 as u64,
            0,
            0,
            0,
            [
                xmm0_rcp as u128,
                xmm1_prev as u128,
                (x0_6680 as u32) as u128,
                0,
            ],
        );
        wr::<u32>(param_5 + 0x08, x0_e8c0 as u32);
    }

    wr::<u32>(param_5 + 0x10, 0x3f80_0000);
    let cl_46 = rd::<u8>(param_4 + 0x46);
    let al_2 = rd::<u8>(param_5 + 0x02);
    if (cl_46 & 0x20) == 0
        || (al_2 & 0x01) != 0
        || ((cl_46 & 0x40) != 0 && (al_2 & 0x08) == 0)
        || (al_2 & 0x04) != 0
    {
        return;
    }

    let st_idx = rd::<i16>(param_5) as isize;
    let tbl_8 = rd::<usize>(rd::<usize>(param_4) + 0x08);
    let rbx_anim = rd::<usize>(tbl_8.wrapping_add((st_idx as usize) * 8));
    let off_64 = rd::<i32>(rbx_anim + 0x64) as isize;
    let count_60 = rd::<i32>(rbx_anim + 0x60) as isize;
    let xmm7_frame = rd::<f32>(rbx_anim + 0x18) * rd::<f32>(param_5 + 0x04);
    let rdx_arr = if off_64 == 0 {
        0
    } else {
        (rbx_anim + 0x64).wrapping_add(off_64 as usize)
    };

    let mut found_ent: usize = 0;
    if count_60 > 0 {
        for idx in 0..(count_60 as usize) {
            let cur_ent = rdx_arr.wrapping_add(idx * 12);
            let f_lo = rd::<i32>(cur_ent) as f32;
            if xmm7_frame >= f_lo {
                let f_hi = rd::<i32>(cur_ent + 0x04) as f32;
                if f_hi > xmm7_frame {
                    found_ent = cur_ent;
                    break;
                }
            }
        }
    }

    let tbl_6090 = exe::ptr::<u64>(0x1_4386_6090) as usize;
    if found_ent != 0 && rd::<u8>(found_ent + 0x08) == 2 {
        let xmm2_cur = rd::<f32>(param_5 + 0x04);
        let xmm1_end = (rd::<i32>(found_ent + 0x04) as f32) * rd::<f32>(rbx_anim + 0x18);
        let xmm6_rem = xmm1_end - xmm2_cur;
        let fn_idx1 = rd::<u16>(rbx_anim + 0x0e) as usize;
        let fn1 = rd::<u64>(tbl_6090.wrapping_add(fn_idx1 * 8));
        let _ = call_va_xmm(
            fn1,
            ptr_70 as u64,
            rbx_anim as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, xmm2_cur.to_bits() as u128, 0],
        );

        let xmm1_sum = xmm6_rem + rd::<f32>(param_5 + 0x04);
        let xmm2_lim = rd::<f32>(rbx_anim + 0x14);
        let xmm2_clamped = if xmm2_lim < xmm1_sum {
            xmm2_lim
        } else {
            xmm1_sum
        };
        let fn_idx2 = rd::<u16>(rbx_anim + 0x0e) as usize;
        let fn2 = rd::<u64>(tbl_6090.wrapping_add(fn_idx2 * 8));
        let _ = call_va_xmm(
            fn2,
            ptr_50 as u64,
            rbx_anim as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, xmm2_clamped.to_bits() as u128, 0],
        );

        let _ = call_va_xmm(
            0x1_421c_2390,
            param_3 as u64,
            ptr_10 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, xmm6_rem.to_bits() as u128, 0],
        );

        let xmm5 = vbroadcastss(xmm6_rem.to_bits());
        let xmm1 = vrcpps_128(xmm5);
        let xmm2_nr = vmulps(vmulps(xmm1, xmm1), xmm5);
        let xmm3 = vsubps(vaddps(xmm1, xmm1), xmm2_nr);
        let xmm2_diff = vsubps(rd::<u128>(ptr_60), rd::<u128>(ptr_80));
        let xmm0_eps = rd::<u128>(exe::ptr::<u128>(0x1_4342_7f30) as usize);
        let xmm4 = vmulps(xmm3, xmm2_diff);
        let xmm1_dp = vdpps(xmm4, xmm4, 0x7f);
        let xmm2_mask = vcmpps(xmm0_eps, xmm1_dp, 2);
        let xmm1_rsq = vrsqrtps_128(xmm1_dp);
        let xmm6_dir = vmulps(xmm2_mask & xmm1_rsq, xmm4);
        let xmm2_dp = vdpps(xmm4, xmm6_dir, 0x7f);
        let xmm0_dp = vdpps(xmm2_dp, xmm2_dp, 0x7f);
        let c_001 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
        if !(f32::from_bits(xmm0_dp as u32) > c_001) {
            return;
        }
        let xmm1_r = vrcpps_128(xmm2_dp);
        let xmm2_r = vmulps(vmulps(xmm1_r, xmm1_r), xmm2_dp);
        let xmm3_r = vsubps(vaddps(xmm1_r, xmm1_r), xmm2_r);
        let xmm2_div = vdivps(rd::<u128>(ptr_10), xmm5);
        let xmm1_proj = vdpps(xmm2_div, xmm6_dir, 0x7f);
        let xmm3_out = vmulps(xmm3_r, xmm1_proj);
        wr::<u32>(param_5 + 0x10, xmm3_out as u32);
        wr::<u128>(param_5 + 0x20, xmm6_dir);
        return;
    }

    let _ = call_va_xmm(
        0x1_421c_1b80,
        param_3 as u64,
        ptr_10 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0; 4],
    );

    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm5_inv = one / rd::<f32>(rbx_anim + 0x18);
    let xmm3_max = rd::<f32>(rbx_anim + 0x14);
    let t_minus = (xmm7_frame - one) * xmm5_inv;
    let xmm8_lo = if 0.0f32 > t_minus { 0.0f32 } else { t_minus };
    let t_plus = (xmm7_frame + one) * xmm5_inv;
    let xmm7_hi = if xmm3_max < t_plus { xmm3_max } else { t_plus };

    let fn_idx_a = rd::<u16>(rbx_anim + 0x0e) as usize;
    let fn_a = rd::<u64>(tbl_6090.wrapping_add(fn_idx_a * 8));
    let _ = call_va_xmm(
        fn_a,
        ptr_30 as u64,
        rbx_anim as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm8_lo.to_bits() as u128, 0],
    );

    let fn_idx_b = rd::<u16>(rbx_anim + 0x0e) as usize;
    let fn_b = rd::<u64>(tbl_6090.wrapping_add(fn_idx_b * 8));
    let _ = call_va_xmm(
        fn_b,
        ptr_50 as u64,
        rbx_anim as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm7_hi.to_bits() as u128, 0],
    );

    let mask_b00 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7b00) as usize);
    let xmm11 = mask_b00 ^ rd::<u128>(ptr_30);
    let xmm10 = vsubps(rd::<u128>(ptr_60), rd::<u128>(ptr_40));
    let xmm8 = vbroadcastss((xmm7_hi - xmm8_lo).to_bits());
    let xmm0 = vpermilps(xmm10, 0xc9);
    let xmm1 = vpermilps(xmm10, 0xd2);
    let xmm9 = vpermilps(xmm11, 0xc9);
    let xmm2 = vmulps(xmm1, xmm9);
    let xmm4 = vpermilps(xmm11, 0xd2);
    let xmm3 = vsubps(xmm2, vmulps(xmm0, xmm4));
    let c_ab0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize);
    let xmm6 = vmulps(xmm3, c_ab0);
    let xmm1_rcp = vrcpps_128(xmm8);
    let xmm2_rcp = vmulps(vmulps(xmm1_rcp, xmm1_rcp), xmm8);
    let xmm5 = vsubps(vaddps(xmm1_rcp, xmm1_rcp), xmm2_rcp);
    let xmm4_c = vsubps(
        vmulps(vpermilps(xmm6, 0xd2), xmm9),
        vmulps(vpermilps(xmm6, 0xc9), xmm4),
    );
    let xmm3_sum = vaddps(
        xmm4_c,
        vaddps(vmulps(vpermilps(xmm11, 0xff), xmm6), xmm10),
    );
    let xmm0_eq = vcmpps(0, xmm8, 0);
    let xmm1_bl = vblendvps(xmm5, 0, xmm0_eq);
    let xmm10_v = vmulps(xmm3_sum, xmm1_bl);
    let xmm8_dp = vdpps(xmm10_v, xmm10_v, 0x7f);
    let xmm7_rsq = vrsqrtps_128(xmm8_dp);
    let c_120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let xmm5_half = vmulps(xmm7_rsq, c_120);
    let c_fa0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7fa0) as usize);
    let xmm9_lt = vcmpps(c_fa0, xmm8_dp, 1);
    let c_460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
    let xmm1_nr = vmulps(vmulps(xmm8_dp, xmm7_rsq), xmm7_rsq);
    let xmm3_nr = vmulps(vsubps(c_460, xmm1_nr), xmm5_half);
    let xmm0_10 = rd::<u128>(ptr_10);
    let xmm2_len = vsqrtps(vdpps(xmm0_10, xmm0_10, 0x7f));
    let xmm4_len = vmulps(xmm3_nr, xmm2_len);
    let c_240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let xmm1_out = vblendvps(c_240, xmm4_len, xmm9_lt);
    wr::<u32>(param_5 + 0x10, xmm1_out as u32);

    let xmm2_nr2 = vmulps(vmulps(xmm8_dp, xmm7_rsq), xmm7_rsq);
    let xmm4_nr2 = vmulps(vsubps(c_460, xmm2_nr2), xmm5_half);
    let xmm5_vec = vmulps(xmm4_nr2, xmm10_v);
    let xmm2_out = vblendvps(0, xmm5_vec, xmm9_lt);
    wr::<u128>(param_5 + 0x20, xmm2_out);
}

/// `unknown_1421c7d10`
/// Original: 0x1421c7d10 (388 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421c7d10(param_1: usize) {
    let edx = rd::<u32>(param_1 + 0x10);
    let p_168 = rd::<usize>(param_1 + 0x168);
    let rdi = rd::<usize>(param_1 + 0x08);
    let xmm6 = rd::<f32>(param_1 + 0x48);
    let bit_arr = rd::<usize>(p_168 + 0x10);
    let r14 = rd::<usize>(param_1);
    let mask_word = rd::<u64>(bit_arr.wrapping_add(((edx >> 6) as usize) * 8));

    if ((mask_word >> (edx & 0x3f)) & 1) == 0 {
        exe::call_va!(0x1_421c_5660, (r14, rdi, param_1));
    } else {
        let r8_60 = rd::<usize>(param_1 + 0x60);
        if r8_60 != 0 {
            let bx = rd::<i16>(rdi);
            let rdx_arg = rd::<usize>(rd::<usize>(r14));
            let xmm7 = rd::<f32>(rdi + 0x04);
            exe::call_va!(
                0x1_421c_5a50,
                (param_1, rdx_arg, r8_60, r14, rdi, xmm6.to_bits())
            );
            let r8_new = rd::<i16>(rdi) as isize;
            let xmm1: f32 = if (r8_new as i16) == bx {
                let cur_t = rd::<f32>(rdi + 0x04);
                let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
                let diff = cur_t - xmm7;
                let xmm5 = f32::from_bits(diff.to_bits() & !sign_mask);
                let rcx_inner = rd::<usize>(rd::<usize>(r14));
                let arr_18 = rd::<usize>(rcx_inner + 0x18);
                let ent = arr_18.wrapping_add((r8_new * 0x14) as usize);
                if rd::<i16>(ent + 0x0e) == (r8_new as i16) {
                    let xmm0 = rd::<f32>(ent + 0x04);
                    let xmm2 = xmm0 - xmm5;
                    if xmm5 < xmm2 {
                        xmm5
                    } else {
                        xmm2
                    }
                } else {
                    xmm5
                }
            } else {
                rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd0) as usize)
            };

            if xmm1 > rd::<f32>(r14 + 0x30) {
                let xmm1_2c = rd::<f32>(r14 + 0x2c);
                if (rd::<u8>(r14 + 0x46) & 2) != 0 {
                    let edx_58 = rd::<u32>(rdi + 0x58);
                    let xmm0_4 = rd::<u32>(rdi + 0x04);
                    let ecx_sub = edx_58.wrapping_sub(7);
                    let cur_5c = rd::<u32>(rdi + 0x5c);
                    let new_5c = if cur_5c > ecx_sub { cur_5c } else { ecx_sub };
                    let slot = (edx_58 & 7) as usize;
                    wr::<u32>(rdi + 0x5c, new_5c);
                    wr::<u32>(rdi + 0x58, edx_58.wrapping_add(1));
                    let ent_off = rdi.wrapping_add(0x60 + slot * 12);
                    wr::<u32>(ent_off, rd::<u32>(rdi + 0x50));
                    wr::<f32>(ent_off + 0x08, xmm1_2c);
                    wr::<u32>(ent_off + 0x04, rd::<u32>(rdi + 0x54));
                    wr::<u32>(rdi + 0x54, xmm0_4);
                    wr::<i32>(rdi + 0x50, r8_new as i32);
                } else {
                    let _ = call_va_xmm(
                        0x1_421d_5920,
                        param_1 as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [0, xmm1_2c.to_bits() as u128, 0, 0],
                    );
                }
            }
        }
    }

    if (rd::<u8>(r14 + 0x46) & 2) != 0 {
        let xmm1_4 = rd::<f32>(rdi + 0x04);
        let _ = call_va_xmm(
            0x1_421d_efd0,
            (rdi + 0x50) as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, xmm1_4.to_bits() as u128, xmm6.to_bits() as u128, 0],
        );
    }
}

/// `unknown_1421d5920`
/// Original: 0x1421d5920 (69 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_1421d5920(param_1: usize, param_2: f32) -> u8 {
    unknown_1421d5920_impl(param_1, param_2)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_1421d5920(param_1: usize, _xmm0: f32, param_2: f32) -> u8 {
    unknown_1421d5920_impl(param_1, param_2)
}

#[inline(always)]
unsafe fn unknown_1421d5920_impl(param_1: usize, param_2: f32) -> u8 {
    let val = if let Some(xmm) = crate::simd_ops::take_tls_xmm_in_if_set() {
        f32::from_bits(xmm[1] as u32)
    } else {
        param_2
    };
    let al = rd::<u8>(param_1 + 0x15a);
    let new_al = al | 0x08;
    let out_f = if (al & 0x08) != 0 {
        let cur = rd::<f32>(param_1 + 0x144);
        let min_v = if val < cur { val } else { cur };
        wr::<f32>(param_1 + 0x144, min_v);
        wr::<u8>(param_1 + 0x15a, new_al);
        min_v
    } else {
        wr::<u8>(param_1 + 0x15a, new_al);
        wr::<f32>(param_1 + 0x144, val);
        val
    };
    crate::simd_ops::set_xmm_ret(out_f.to_bits() as u128);
    new_al
}

/// `unknown_1421defd0`
/// Original: 0x1421defd0 (428 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_1421defd0(param_1: usize, param_2: f32, param_3: f32) {
    unknown_1421defd0_impl(param_1, param_2, param_3);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_1421defd0(
    param_1: usize,
    _xmm0: f32,
    param_2: f32,
    param_3: f32,
) {
    unknown_1421defd0_impl(param_1, param_2, param_3);
}

#[inline(always)]
unsafe fn unknown_1421defd0_impl(param_1: usize, param_2: f32, param_3: f32) {
    let (p2, p3) = if let Some(xmm) = crate::simd_ops::take_tls_xmm_in_if_set() {
        (f32::from_bits(xmm[1] as u32), f32::from_bits(xmm[2] as u32))
    } else {
        (param_2, param_3)
    };

    let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
    wr::<f32>(param_1 + 0x04, p2);
    let count = rd::<u32>(param_1 + 0x08).wrapping_sub(rd::<u32>(param_1 + 0x0c)) as usize;
    for i in 0..count {
        let base_c = rd::<u32>(param_1 + 0x0c);
        let slot = (base_c.wrapping_add(i as u32) & 7) as usize;
        let ent = param_1.wrapping_add(slot * 12);
        let rem = rd::<f32>(ent + 0x18) - p3;
        wr::<f32>(ent + 0x18, rem);
        if rem > eps {
            wr::<f32>(ent + 0x14, p3 + rd::<f32>(ent + 0x14));
        } else {
            wr::<u32>(ent + 0x18, 0);
        }
    }

    let mut ecx = rd::<u32>(param_1 + 0x0c);
    let r8d = rd::<u32>(param_1 + 0x08);
    while r8d != ecx {
        let slot = (ecx & 7) as usize;
        let v = rd::<f32>(param_1.wrapping_add(slot * 12) + 0x18);
        if v.is_nan() || v != 0.0f32 {
            break;
        }
        let next = ecx.wrapping_add(1);
        ecx = if next < r8d { next } else { r8d };
        wr::<u32>(param_1 + 0x0c, ecx);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1421c5230(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, usize, usize, usize, usize) -> i32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let op_byte: u8 = 0;
    let off_zero: i32 = 0;
    let mut hdr = [0u8; 0x20];
    let hp = hdr.as_mut_ptr() as usize;
    wr::<i32>(hp, 8);
    wr::<u8>(hp + 0x07, 1);
    wr::<usize>(hp + 0x08, (&op_byte as *const u8) as usize);
    wr::<usize>(hp + 0x10, (&off_zero as *const i32) as usize);

    let p2_f16: u64 = 0x3c00; // f16(1.0) in lane 0
    let elems_f16: [u64; 4] = [
        0x4400, // 4.0 -> dist 3.0
        0x4000, // 2.0 -> dist 1.0
        0x3800, // 0.5 -> dist 0.5 (min)
        0x4200, // 3.0 -> dist 2.0
    ];
    let mut p5 = [0u8; 0x10];
    wr::<usize>(p5.as_mut_ptr() as usize + 0x08, elems_f16.as_ptr() as usize);
    let weights = Align16([0, 0, 0x80, 0x3f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let indices: [i32; 3] = [0, 2, 3];

    let run = |f: F, count: i32| -> (i32, u32) {
        let mut out_dist: u32 = 0;
        let ret = f(
            hp,
            (&p2_f16 as *const u64) as usize,
            count,
            indices.as_ptr() as usize,
            p5.as_ptr() as usize,
            (&weights as *const Align16) as usize,
            (&mut out_dist as *mut u32) as usize,
        );
        (ret, out_dist)
    };

    for cnt in [0i32, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1421c5230 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421c5660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_6440(out_16: usize, _p1: usize, _p3: usize) {
        wr::<u64>(out_16, 0x1122_3344_5566_7788);
        wr::<u64>(out_16 + 0x08, 0x99aa_bbcc_ddee_ff00);
        TRACE = TRACE.wrapping_mul(31).wrapping_add(1);
    }
    unsafe extern "win64" fn mock_47f0(
        _r14: usize,
        buf1: usize,
        r8_58: u64,
        r9_10: u64,
        a5: u64,
        a6: u64,
    ) {
        wr::<u32>(buf1, (r8_58 as u32) ^ (r9_10 as u32) ^ (a5 as u32) ^ (a6 as u32));
        TRACE = TRACE.wrapping_mul(31).wrapping_add(2);
    }
    unsafe extern "win64" fn mock_3150(
        _r14: usize,
        buf1: usize,
        _p60: usize,
        p40: usize,
        p20: usize,
        p00: usize,
        p10: usize,
        esi: u32,
        edi: u32,
    ) {
        let mix = rd::<u32>(p40)
            ^ rd::<u32>(p20)
            ^ rd::<u32>(p00)
            ^ rd::<u32>(p10)
            ^ esi
            ^ edi;
        wr::<u32>(buf1 + 0x04, mix);
        TRACE = TRACE.wrapping_mul(31).wrapping_add(mix as u64);
    }
    unsafe extern "win64" fn mock_2d10(_r14: usize, _rdx: usize, buf2: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        wr::<u32>(buf2, xmm[1] as u32);
        wr::<u32>(buf2 + 0x04, 0x1234_5678);
    }
    unsafe extern "win64" fn mock_2b70(_r14: usize, buf2: usize) {
        wr::<u32>(buf2 + 0x08, 0x7777_8888);
        TRACE = TRACE.wrapping_mul(31).wrapping_add(5);
    }
    unsafe extern "win64" fn mock_2660(
        _r12: usize,
        out_idx: usize,
        buf1: usize,
        sil: u32,
        buf2: usize,
    ) {
        wr::<i32>(out_idx, 1);
        TRACE = TRACE
            .wrapping_mul(31)
            .wrapping_add((rd::<u32>(buf1) ^ rd::<u32>(buf2) ^ sil) as u64);
    }
    unsafe extern "win64" fn mock_2760(_r12: usize, out_8: usize, st: i32) -> u8 {
        let xmm = crate::simd_ops::take_xmm_args();
        let t_bits = xmm[3] as u32;
        wr::<u32>(out_8, t_bits);
        wr::<i16>(out_8 + 0x04, (st + 1) as i16);
        wr::<u8>(out_8 + 0x07, 0x04);
        1
    }
    unsafe extern "win64" fn mock_6680(
        _p3: usize,
        _p60: usize,
        p10: usize,
        r9_val: u64,
        a5: u32,
        a6: u32,
        a7: u32,
    ) -> f32 {
        let v = f32::from_bits(
            (rd::<u32>(p10) ^ (r9_val as u32) ^ a5 ^ a6 ^ a7) & 0x3fff_ffff,
        );
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_6440, mock_6440 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_47f0, mock_47f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_421c_3150, mock_3150 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_421c_2d10, mock_2d10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_421c_2b70, mock_2b70 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_421c_2660, mock_2660 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_421c_2760, mock_2760 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_421c_6680, mock_6680 as *const ())?;

    let run = |f: F, has_p60: bool| -> ([u8; 0x60], u64) {
        TRACE = 0;

        let map_18: [i32; 2] = [1, 0];
        let ops_20: [u8; 2] = [6, 1];
        let mut r14_struct = [0u8; 0x30];
        let r14p = r14_struct.as_mut_ptr() as usize;
        wr::<i32>(r14p, 32);
        wr::<u8>(r14p + 0x06, 2);
        wr::<u8>(r14p + 0x07, 1);
        wr::<usize>(r14p + 0x18, map_18.as_ptr() as usize);
        wr::<usize>(r14p + 0x20, ops_20.as_ptr() as usize);

        let mut arr_58 = [0u8; 0x10];
        let a58p = arr_58.as_mut_ptr() as usize;
        wr::<f32>(a58p, 1.25);
        wr::<i16>(a58p + 0x04, 2);
        wr::<u8>(a58p + 0x07, 0x02);
        wr::<f32>(a58p + 0x08, 2.5);
        wr::<i16>(a58p + 0x0c, 1);
        wr::<u8>(a58p + 0x0f, 0x00);

        let mut r12_struct = [0u8; 0x60];
        let r12p = r12_struct.as_mut_ptr() as usize;
        wr::<usize>(r12p + 0x40, r14p);
        wr::<u8>(r12p + 0x4a, 0x03);
        wr::<usize>(r12p + 0x58, a58p);

        let tbl_8: [u64; 4] = [0x100, 0x200, 0x300, 0x400];
        let mut deref_p1 = [0u8; 0x10];
        let dp1 = deref_p1.as_mut_ptr() as usize;
        wr::<usize>(dp1, r12p);
        wr::<usize>(dp1 + 0x08, tbl_8.as_ptr() as usize);

        let mut p1 = [0u8; 0x50];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, dp1);
        wr::<u64>(p1p + 0x10, 0xabc);
        wr::<f32>(p1p + 0x28, 3.5);
        wr::<f32>(p1p + 0x38, 1.0);
        wr::<f32>(p1p + 0x3c, 0.5);
        wr::<u8>(p1p + 0x46, 0x02);

        let v40: u32 = 11;
        let v48: u32 = 22;
        let mut p2 = [0u8; 0x60];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x40, (&v40 as *const u32) as usize);
        wr::<usize>(p2p + 0x48, (&v48 as *const u32) as usize);

        let mut p60_buf = vec![0u8; 0x1010];
        let p60p = p60_buf.as_mut_ptr() as usize;
        wr::<u8>(p60p + 0x30, 1);
        wr::<i32>(p60p + 0x1000, 0);

        let mut p3 = [0u8; 0x120];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<f32>(p3p + 0x48, 0.25);
        wr::<u64>(p3p + 0x58, 0x111);
        wr::<usize>(p3p + 0x60, if has_p60 { p60p } else { 0 });
        wr::<u32>(p3p + 0x70, 0x222);
        wr::<u32>(p3p + 0x90, 0x333);
        wr::<u32>(p3p + 0xd0, 0x444);
        wr::<u64>(p3p + 0x100, 0x555);
        wr::<u64>(p3p + 0x110, 0x666);

        f(p1p, p2p, p3p);
        (p2, TRACE)
    };

    for has_p60 in [false, true] {
        let o = run(f_o, has_p60);
        let n = run(f_n, has_p60);
        crate::diff_eq!(o, n, "1421c5660 has_p60={}", has_p60);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421c5a50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    unsafe extern "win64" fn mock_2d10(_rdi: usize, _rdx: usize, buf: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        wr::<f32>(buf, f32::from_bits(xmm[1] as u32));
        wr::<f32>(buf + 0x04, 2.0);
    }
    unsafe extern "win64" fn mock_2b70(_rdi: usize, buf: usize) {
        wr::<u32>(buf + 0x08, 0x1234);
    }
    unsafe extern "win64" fn mock_6440(out_10: usize, _p4: usize, _p1: usize) {
        wr::<[f32; 4]>(out_10, [2.0, 0.0, 0.0, 0.0]);
    }
    unsafe extern "win64" fn mock_5420(
        _rbx: usize,
        dst: usize,
        src0: usize,
        src1: usize,
        frac_bits: u32,
    ) {
        wr::<u32>(dst, rd::<u32>(src0) ^ rd::<u32>(src1) ^ frac_bits);
    }
    unsafe extern "win64" fn mock_3150(
        _rdi: usize,
        dst: usize,
        _p3: usize,
        p50: usize,
        p70: usize,
        p10: usize,
        p30: usize,
        a8: u32,
        a9: u32,
    ) {
        let mix = rd::<u32>(p50) ^ rd::<u32>(p70) ^ rd::<u32>(p10) ^ rd::<u32>(p30) ^ a8 ^ a9;
        wr::<u32>(dst + 0x04, mix);
    }
    unsafe extern "win64" fn mock_2760(_p2: usize, out_8: usize, st: i32) -> u8 {
        let xmm = crate::simd_ops::take_xmm_args();
        wr::<u32>(out_8, xmm[3] as u32);
        wr::<i16>(out_8 + 0x04, st as i16);
        wr::<u8>(out_8 + 0x06, 0x01);
        wr::<u8>(out_8 + 0x07, 0x08);
        1
    }
    unsafe extern "win64" fn mock_2ef0(_rdi: usize, b30: usize, b20: usize, _r15: usize) -> f32 {
        let v = if (rd::<u32>(b30) ^ rd::<u32>(b20)) != 0 {
            0.25f32
        } else {
            0.5f32
        };
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_2660(
        _p2: usize,
        out_8: usize,
        _b30: usize,
        _sil: u32,
        _r15: usize,
    ) {
        wr::<i32>(out_8, 0);
        wr::<f32>(out_8 + 0x04, 0.1);
    }
    unsafe extern "win64" fn mock_6680(
        _p1: usize,
        _p3: usize,
        _p10: usize,
        _r9: u64,
        _a5: u32,
        _a6: u32,
        _a7: u32,
    ) -> f32 {
        let v = 1.5f32;
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_e8c0(
        _rcx: usize,
        _rdx: usize,
        _r8: usize,
        r9_ptr: usize,
        _a5: u32,
    ) -> f32 {
        let xmm = crate::simd_ops::take_xmm_args();
        wr::<u32>(r9_ptr, xmm[0] as u32);
        let v = f32::from_bits(xmm[1] as u32) + f32::from_bits(xmm[2] as u32);
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_1b80(_p3: usize, out_10: usize) {
        wr::<[f32; 4]>(out_10, [3.0, 4.0, 0.0, 0.0]);
    }
    unsafe extern "win64" fn mock_2390(_p3: usize, out_10: usize) {
        wr::<[f32; 4]>(out_10, [6.0, 0.0, 0.0, 0.0]);
    }
    unsafe extern "win64" fn mock_pose_eval(out_32: usize, _anim: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm[2] as u32);
        wr::<[f32; 4]>(out_32, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(out_32 + 0x10, [t * 2.0, t, 0.0, 0.0]);
        TRACE = TRACE.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_2d10, mock_2d10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_2b70, mock_2b70 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_421c_6440, mock_6440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_421c_5420, mock_5420 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_421c_3150, mock_3150 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_421c_2760, mock_2760 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_421c_2ef0, mock_2ef0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_421c_2660, mock_2660 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_421c_6680, mock_6680 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4102_e8c0, mock_e8c0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_421c_1b80, mock_1b80 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_421c_2390, mock_2390 as *const ())?;

    let slot_6090 = exe::ptr::<usize>(0x1_4386_6090) as usize;
    let saved_6090 = rd::<usize>(slot_6090);
    wr::<usize>(slot_6090, mock_pose_eval as *const () as usize);

    let run = |f: F, seg_kind: u8| -> ([u8; 0x50], u64) {
        TRACE = 0;

        let map_18: [i32; 2] = [0, 1];
        let map_28: [i32; 2] = [8, 12];
        let mut rdi_40 = [0u8; 0x30];
        let r40p = rdi_40.as_mut_ptr() as usize;
        wr::<i32>(r40p, 32);
        wr::<u8>(r40p + 0x07, 1);
        wr::<usize>(r40p + 0x18, map_18.as_ptr() as usize);
        wr::<usize>(r40p + 0x28, map_28.as_ptr() as usize);

        let mut arr_18 = [0u8; 0x28];
        let a18p = arr_18.as_mut_ptr() as usize;
        wr::<f32>(a18p + 0x04, 2.0);
        wr::<i16>(a18p + 0x0c, 2);
        wr::<i16>(a18p + 0x0e, -1);

        let frame_data: [u32; 16] = [0x1111_2222; 16];
        let frame_times: [u64; 2] = [0.0f32.to_bits() as u64, 1.0f32.to_bits() as u64];
        let mut arr_20 = [0u8; 0x30];
        let a20p = arr_20.as_mut_ptr() as usize;
        wr::<usize>(a20p + 0x08, frame_data.as_ptr() as usize);
        wr::<usize>(a20p + 0x10, frame_times.as_ptr() as usize);

        let mut arr_58 = [0u8; 0x10];
        wr::<f32>(arr_58.as_mut_ptr() as usize, 0.2);

        let mut p2 = [0u8; 0x60];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<f32>(p2p, 10.0);
        wr::<usize>(p2p + 0x18, a18p);
        wr::<usize>(p2p + 0x20, a20p);
        wr::<usize>(p2p + 0x40, r40p);
        wr::<u8>(p2p + 0x4a, 0x01);
        wr::<usize>(p2p + 0x58, arr_58.as_ptr() as usize);

        let mut p3 = vec![0u8; 0x1010];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u8>(p3p + 0x30, 1);

        let mut anim = [0u8; 0x80];
        let anp = anim.as_mut_ptr() as usize;
        wr::<u16>(anp + 0x0e, 0);
        wr::<f32>(anp + 0x14, 5.0);
        wr::<f32>(anp + 0x18, 10.0);
        wr::<i32>(anp + 0x60, 1);
        wr::<i32>(anp + 0x64, 0x0c); // entry at anp + 0x70
        wr::<i32>(anp + 0x70, 0);
        wr::<i32>(anp + 0x74, 20);
        wr::<u8>(anp + 0x78, seg_kind);

        let tbl_8: [usize; 2] = [anp, anp];
        let mut dp4 = [0u8; 0x10];
        wr::<usize>(dp4.as_mut_ptr() as usize + 0x08, tbl_8.as_ptr() as usize);

        let idx_bytes: [u8; 2] = [0, 1];
        let mut p4 = [0u8; 0x50];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p, dp4.as_ptr() as usize);
        wr::<usize>(p4p + 0x18, idx_bytes.as_ptr() as usize);
        wr::<u8>(p4p + 0x21, 1);
        wr::<f32>(p4p + 0x28, 1.0);
        wr::<f32>(p4p + 0x34, 1.0);
        wr::<f32>(p4p + 0x38, 2.0);
        wr::<f32>(p4p + 0x3c, 0.5);
        wr::<u8>(p4p + 0x46, 0x20);

        let f_30: f32 = 1.5;
        let p_30: [usize; 1] = [(&f_30 as *const f32) as usize];
        let f_38: f32 = 2.0;
        let p_38: [usize; 1] = [(&f_38 as *const f32) as usize];
        let v40: u32 = 7;
        let v48: u32 = 9;
        let mut p5 = [0u8; 0x50];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<i16>(p5p, 0);
        wr::<f32>(p5p + 0x04, 0.25);
        wr::<f32>(p5p + 0x08, 0.5);
        wr::<usize>(p5p + 0x30, p_30.as_ptr() as usize);
        wr::<usize>(p5p + 0x38, p_38.as_ptr() as usize);
        wr::<usize>(p5p + 0x40, (&v40 as *const u32) as usize);
        wr::<usize>(p5p + 0x48, (&v48 as *const u32) as usize);

        let p1 = [0u8; 0x100];
        f(p1.as_ptr() as usize, p2p, p3p, p4p, p5p, 0.1f32.to_bits());
        (p5, TRACE)
    };

    let mut res = Ok(2);
    for seg_kind in [0u8, 2] {
        let o = run(f_o, seg_kind);
        let n = run(f_n, seg_kind);
        if o != n {
            res = Err(format!("mismatch for 1421c5a50 seg_kind={seg_kind}"));
            break;
        }
    }
    wr::<usize>(slot_6090, saved_6090);
    res
}

pub unsafe fn diff_unknown_1421c7d10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TAG: u64 = 0;

    unsafe extern "win64" fn mock_5660(_r14: usize, rdi: usize, _p1: usize) {
        wr::<i16>(rdi, 3);
        wr::<f32>(rdi + 0x04, 1.5);
        CALL_TAG = CALL_TAG.wrapping_mul(31).wrapping_add(0x5660);
    }
    unsafe extern "win64" fn mock_5a50(
        _p1: usize,
        _rdx: usize,
        _r8: usize,
        _r14: usize,
        rdi: usize,
        _dt: u32,
    ) {
        wr::<f32>(rdi + 0x04, rd::<f32>(rdi + 0x04) + 0.5);
        CALL_TAG = CALL_TAG.wrapping_mul(31).wrapping_add(0x5a50);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_5660, mock_5660 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_5a50, mock_5a50 as *const ())?;

    let run = |f: F, bit_set: bool, flag_46: u8| -> ([u8; 0xc0], u32, u8, u64) {
        CALL_TAG = 0;

        let mut arr_18 = [0u8; 0x30];
        let a18p = arr_18.as_mut_ptr() as usize;
        wr::<f32>(a18p + 0x14 + 0x04, 2.0);
        wr::<i16>(a18p + 0x14 + 0x0e, 1);

        let mut inner0 = [0u8; 0x20];
        wr::<usize>(inner0.as_mut_ptr() as usize + 0x18, a18p);
        let p_inner0 = inner0.as_ptr() as usize;
        let pp_inner0 = (&p_inner0 as *const usize) as usize;

        let mut r14 = [0u8; 0x50];
        let r14p = r14.as_mut_ptr() as usize;
        wr::<usize>(r14p, pp_inner0);
        wr::<f32>(r14p + 0x2c, 0.75);
        wr::<f32>(r14p + 0x30, 0.1);
        wr::<u8>(r14p + 0x46, flag_46);

        let mut rdi = [0u8; 0xc0];
        let rdip = rdi.as_mut_ptr() as usize;
        wr::<i16>(rdip, 1);
        wr::<f32>(rdip + 0x04, 0.25);
        wr::<i32>(rdip + 0x50, 1);
        wr::<f32>(rdip + 0x54, 0.25);
        wr::<u32>(rdip + 0x58, 8);
        wr::<u32>(rdip + 0x5c, 8);

        let mask_word: u64 = if bit_set { 1 } else { 0 };
        let mut p168 = [0u8; 0x20];
        wr::<usize>(p168.as_mut_ptr() as usize + 0x10, (&mask_word as *const u64) as usize);

        let mut p1 = [0u8; 0x170];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, r14p);
        wr::<usize>(p1p + 0x08, rdip);
        wr::<u32>(p1p + 0x10, 0);
        wr::<f32>(p1p + 0x48, 0.1);
        wr::<usize>(p1p + 0x60, 0x1000);
        wr::<f32>(p1p + 0x144, 2.0);
        wr::<u8>(p1p + 0x15a, 0);
        wr::<usize>(p1p + 0x168, p168.as_ptr() as usize);

        f(p1p);
        (rdi, rd::<u32>(p1p + 0x144), rd::<u8>(p1p + 0x15a), CALL_TAG)
    };

    for (bit_set, flag_46) in [(false, 2u8), (true, 2), (true, 0)] {
        let o = run(f_o, bit_set, flag_46);
        let n = run(f_n, bit_set, flag_46);
        crate::diff_eq!(o, n, "1421c7d10 bit={} flag={}", bit_set, flag_46);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421d5920(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, init_flag: u8, init_f: f32, arg_f: f32| -> (u32, u8, u32) {
        let mut p1 = [0u8; 0x160];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<f32>(p1p + 0x144, init_f);
        wr::<u8>(p1p + 0x15a, init_flag);
        let (_, x0) = call_va_xmm(
            fn_addr as u64,
            p1p as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, arg_f.to_bits() as u128, 0, 0],
        );
        (rd::<u32>(p1p + 0x144), rd::<u8>(p1p + 0x15a), x0 as u32)
    };

    for (fl, f0, f1) in [(0u8, 5.0f32, 2.5f32), (8, 5.0, 2.5), (8, 1.5, 3.0)] {
        let o = run(orig, fl, f0, f1);
        let n = run(new, fl, f0, f1);
        crate::diff_eq!(o, n, "1421d5920 fl={} f0={} f1={}", fl, f0, f1);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421defd0(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, dt: f32| -> [u32; 28] {
        let mut buf = [0u32; 28];
        let bp = buf.as_mut_ptr() as usize;
        wr::<i32>(bp, 1);
        wr::<f32>(bp + 0x04, 0.5);
        wr::<u32>(bp + 0x08, 10); // head
        wr::<u32>(bp + 0x0c, 8);  // tail (2 active entries at slots 0 and 1)

        // Slot 0: expires when dt >= 0.2
        wr::<i32>(bp + 0x10, 2);
        wr::<f32>(bp + 0x14, 1.0);
        wr::<f32>(bp + 0x18, 0.15);

        // Slot 1: stays active when dt == 0.2
        wr::<i32>(bp + 0x1c, 3);
        wr::<f32>(bp + 0x20, 2.0);
        wr::<f32>(bp + 0x24, 0.5);

        let _ = call_va_xmm(
            fn_addr as u64,
            bp as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 1.25f32.to_bits() as u128, dt.to_bits() as u128, 0],
        );
        buf
    };

    for dt in [0.05f32, 0.2] {
        let o = run(orig, dt);
        let n = run(new, dt);
        crate::diff_eq!(o, n, "1421defd0 dt={}", dt);
    }
    Ok(2)
}
