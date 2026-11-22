//! Decompiled hot functions shard P (`0x1404b5940..0x1412eeb30`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vbroadcastss, vcmpps, vdivps, vdpps, vmulps, vpermilps, vshufps, vsqrtps, vsubps,
};

#[inline]
unsafe fn call_14217dde0(rcx: usize, rdx: usize, r8: usize, xmm3_bits: u32) {
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4217_dde0,
        rcx as u64,
        rdx as u64,
        r8 as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 0, xmm3_bits as u128],
    );
}

#[inline]
unsafe fn call_142176910(
    rcx: u64,
    xmm1_bits: u32,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
) {
    let xmm = [0, xmm1_bits as u128, 0, 0];
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::lookup_trampoline_or_mapped(0x1_4217_6910);
        crate::simd_ops::set_tls_xmm_in(xmm);
        let f: unsafe extern "win64" fn(u64, f32, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        let _ = f(
            rcx,
            f32::from_bits(xmm1_bits),
            r8,
            r9,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
        );
        let _ = crate::simd_ops::take_tls_xmm_out_opt();
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, arg9, 0, 0, 0, 0, 0, 0, 0];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 128);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4217_6910,
            rcx,
            0,
            r8,
            r9,
            arg5,
            arg6,
            arg7,
            arg8,
            xmm,
        );
    }
}

#[inline(always)]
unsafe fn sso_str_ptr(base: usize) -> usize {
    let b3 = rd::<u8>(base + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cur_bits = (rd::<u32>(base) & 0x00ff_ffff) << 3;
    let inline_ptr = base.wrapping_add(0x08);
    if cur_bits > cap_bits {
        rd::<usize>(inline_ptr)
    } else {
        inline_ptr
    }
}

/// `unknown_1404b5940`
/// Original: 0x1404b5940 (2435 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1404b5940(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut param_2_slot: usize = param_2;
    let rsi = rd::<usize>(param_1 + 0x80);
    let r10 = rd::<usize>(param_1);
    let local_148 = rd::<usize>(param_1 + 0x08);
    let local_1a8 = rd::<usize>(param_1 + 0x10).wrapping_add(rsi.wrapping_mul(3));
    let local_158 = rd::<usize>(param_1 + 0x18);
    let r13 = rd::<usize>(param_1 + 0x20).wrapping_add(rsi.wrapping_shl(6));
    let local_res18 = rd::<usize>(param_1 + 0x28).wrapping_add(rsi.wrapping_shl(4));
    let local_160 = rd::<usize>(param_1 + 0x30).wrapping_add(rsi.wrapping_shl(5));
    let r15 = rd::<usize>(param_1 + 0x58);
    let local_120 = rd::<usize>(param_1 + 0x60).wrapping_add(rsi.wrapping_mul(0x30));
    let local_128 = rd::<usize>(param_1 + 0x68);

    let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
    let id_f63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
    let local_168 = rd::<usize>(tbl_580e0.wrapping_add(id_f63c * 8));
    let id_d910 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d910) as usize) as usize;
    let r12 = rd::<usize>(tbl_580e0.wrapping_add(id_d910 * 8));

    let slot_r15 = r15.wrapping_add(rsi.wrapping_mul(0x18)).wrapping_add(0x08);
    let r10_elem = r10.wrapping_add(rsi.wrapping_mul(0x0c));

    wr::<u8>(rd::<usize>(slot_r15) + 0x18, rd::<u8>(r10_elem + 0x02));
    let obj_deref = rd::<usize>(rd::<usize>(slot_r15));
    wr::<u8>(obj_deref + 0x32, rd::<u8>(local_1a8 + 0x02));
    wr::<u8>(rd::<usize>(slot_r15) + 0x16, rd::<u8>(r10_elem));
    wr::<u8>(rd::<usize>(slot_r15) + 0x17, rd::<u8>(r10_elem + 0x01));

    let r8_handle = rd::<u64>(r10_elem + 0x04);
    let r8d = r8_handle as u32;
    let r9d = (r8_handle >> 32) as u32;
    let mut resolved_handle: u64 = u64::MAX;

    if !(r8d == u32::MAX && r9d == u32::MAX) {
        let rdx_p3_0 = rd::<usize>(param_3);
        let rcx_p3_1 = rd::<usize>(param_3 + 0x08);
        let rax_p3_2 = rd::<usize>(param_3 + 0x10);
        let r10_arr = rd::<usize>(rdx_p3_0 + 0x18);
        let cap = rd::<u32>(rdx_p3_0 + 0x20) as u64;
        let idx = r8d as u64;
        if idx < cap {
            let entry = rd::<u64>(r10_arr.wrapping_add((idx as usize) * 8));
            if (entry as u32) == r8d && ((entry >> 32) as u32) == r9d && r8d != u32::MAX {
                let arr_30 = rd::<usize>(rdx_p3_0 + 0x30);
                let dx_val = rd::<u64>(arr_30.wrapping_add((r8d as usize) * 8)) as u16;
                if dx_val == 0xffff {
                    exe::call_iat!(0x1_4264_67e8, ());
                }
                let mut cur_ptr = rax_p3_2;
                let end_ptr = cur_ptr.wrapping_add(rcx_p3_1.wrapping_mul(2));
                let mut count = (end_ptr.wrapping_sub(cur_ptr) as isize) >> 1;
                while count > 0 {
                    let half = (count as usize) >> 1;
                    let mid_ptr = cur_ptr.wrapping_add(half * 2);
                    if rd::<u16>(mid_ptr) < dx_val {
                        cur_ptr = mid_ptr.wrapping_add(2);
                        count = count - 1 - (half as isize);
                    } else {
                        count = half as isize;
                    }
                }
                if cur_ptr != end_ptr && dx_val >= rd::<u16>(cur_ptr) {
                    resolved_handle = r8_handle;
                }
            }
        }
    }

    let mut xmm6_val: u128 = 0;
    if (resolved_handle as u32) != u32::MAX {
        let mut buf_90 = [0u64; 3];
        exe::call_va!(
            0x1_404b_6f70,
            (param_3, buf_90.as_mut_ptr() as usize, resolved_handle)
        );
        let r8_ptr = (buf_90[0] as usize).wrapping_add((buf_90[2] as usize).wrapping_shl(4));
        if rd::<u64>(r8_ptr) != 0 {
            let b_flag = rd::<u8>(local_1a8 + 0x02);
            let mut tmp_50: u128 = rd::<u128>(local_160 + 0x10);
            let mut out_16: u128 = 0;
            let ret_ptr = exe::call_va!(
                0x1_41fb_7000,
                (
                    (&mut out_16 as *mut u128) as usize,
                    rd::<u64>(local_168),
                    r8_ptr,
                    (&mut tmp_50 as *mut u128) as usize,
                    b_flag as u64
                ) -> usize
            );
            xmm6_val = rd::<u128>(ret_ptr);
        }
    }

    wr::<u128>(rd::<usize>(slot_r15) + 0x20, xmm6_val);

    let rcx_68 = rd::<usize>(rd::<usize>(slot_r15) + 0x68);
    wr::<u64>(rcx_68 + 0x30, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(rcx_68 + 0x38, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(rcx_68 + 0x40, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(rcx_68 + 0x48, 0xffef_ffff_ffff_ffff);
    wr::<u64>(rcx_68 + 0x50, 0xffef_ffff_ffff_ffff);
    wr::<u64>(rcx_68 + 0x58, 0xffef_ffff_ffff_ffff);
    wr::<u32>(rcx_68 + 0x08, 0);
    wr::<u32>(rcx_68 + 0x18, 0);
    wr::<u32>(rcx_68 + 0x28, 0);

    let rcx_f0 = rd::<usize>(rd::<usize>(slot_r15) + 0xf0);
    wr::<u64>(rcx_f0 + 0xf68, rd::<u64>(r12 + 0x10));
    let mut cur_f0 = rd::<usize>(rcx_f0 + 0x08);
    let cnt_f0 = rd::<u32>(rcx_f0 + 0x10) as usize;
    let end_f0 = cur_f0.wrapping_add(cnt_f0.wrapping_mul(0x188));
    while cur_f0 != end_f0 {
        let sub_cnt = rd::<u8>(cur_f0 + 0x30) as usize;
        if sub_cnt != 0 {
            let mut off = 0usize;
            for _ in 0..sub_cnt {
                wr::<u64>(cur_f0 + off + 0x34, 0);
                wr::<u64>(cur_f0 + off + 0x3c, 0);
                wr::<u64>(cur_f0 + off + 0x44, 0);
                wr::<u32>(cur_f0 + off + 0x4c, 0);
                off += 0x1c;
            }
            wr::<u8>(cur_f0 + 0x30, 0);
        }
        cur_f0 = cur_f0.wrapping_add(0x188);
    }

    let ge_2 = (rd::<i32>(local_158.wrapping_add(rsi * 8 + 4)) >= 2) as u8;
    wr::<u8>(rd::<usize>(slot_r15) + 0x14, ge_2);

    let mut opt_50 = [0u8; 32];
    let rcx_40 = rd::<usize>(param_1 + 0x40);
    if rcx_40 != 0 {
        let idx_80 = rd::<usize>(param_1 + 0x80);
        let rbx_plane = rcx_40.wrapping_add(idx_80.wrapping_mul(0x90));
        if rbx_plane != 0 {
            let rax_inner = rd::<usize>(rd::<usize>(slot_r15));
            let xmm6 = rd::<u128>(rax_inner);
            let xmm7 = rd::<u128>(r13 + 0x10);
            let xmm0 = vshufps(xmm7, xmm7, 0xc9);
            let xmm4 = vpermilps(xmm6, 0xd2);
            let xmm2 = vmulps(xmm4, xmm0);
            let xmm1 = vshufps(xmm7, xmm7, 0xd2);
            let xmm5 = vpermilps(xmm6, 0xc9);
            let xmm0 = vmulps(xmm5, xmm1);
            let xmm1 = vsubps(xmm0, xmm2);
            let c_ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
            let xmm3 = vmulps(xmm1, c_ab0);
            let xmm2 = vpermilps(xmm3, 0xc9);
            let xmm4 = vmulps(xmm4, xmm2);
            let xmm0 = vpermilps(xmm3, 0xd2);
            let xmm2 = vmulps(xmm5, xmm0);
            let xmm0 = vpermilps(xmm6, 0xff);
            let xmm1 = vmulps(xmm0, xmm3);
            let xmm3 = vaddps(xmm1, xmm7);
            let xmm2 = vsubps(xmm2, xmm4);
            let xmm5 = vaddps(xmm3, xmm2);

            let c_f30 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7f30) as usize);
            let xmm0 = vdpps(xmm5, xmm5, 0x7f);
            let xmm2 = vcmpps(c_f30, xmm0, 2);
            let xmm0 = vsqrtps(xmm0);
            let xmm1 = vdivps(xmm5, xmm0);
            let mut xmm7_norm = xmm1 & xmm2;

            let xmm0 = vdpps(xmm7_norm, xmm7_norm, 0x7f);
            let xmm1 = vsqrtps(xmm0);
            let len_f32 = f32::from_bits(xmm1 as u32);
            if !len_f32.is_nan() && len_f32 == 0.0f32 {
                let xmm1_fb = rd::<u128>(rbx_plane + 0x10);
                let xmm0_fb = vdpps(xmm1_fb, xmm1_fb, 0x7f);
                let xmm2_fb = vcmpps(c_f30, xmm0_fb, 2);
                let xmm0_fb = vsqrtps(xmm0_fb);
                let xmm1_fb = vdivps(xmm1_fb, xmm0_fb);
                xmm7_norm = xmm1_fb & xmm2_fb;
            }

            let xmm5_40 = rd::<u128>(rbx_plane + 0x40);
            let xmm0 = vdpps(xmm7_norm, xmm7_norm, 0x7f);
            let xmm4 = vcmpps(c_f30, xmm0, 2);
            let xmm0 = vsqrtps(xmm0);
            let xmm3 = vdivps(xmm7_norm, xmm0);

            let xmm1 = vdpps(xmm5_40, xmm5_40, 0x7f);
            let xmm2 = vcmpps(c_f30, xmm1, 2);
            let xmm0 = vsqrtps(xmm1);
            let xmm1 = vdivps(xmm5_40, xmm0);
            let xmm2 = xmm1 & xmm2;
            let xmm0 = xmm3 & xmm4;
            let xmm6_dp = vdpps(xmm0, xmm2, 0x7f);

            let deg = rd::<f32>(exe::ptr::<f32>(0x1_4385_5e60) as usize);
            let deg2rad = rd::<f32>(exe::ptr::<f32>(0x1_4342_6514) as usize);
            let rad = deg * deg2rad;
            let (_, cos_xmm) = crate::simd_ops::call_va8_xmm(
                0x1_4183_ad82,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [rad.to_bits() as u128, 0, 0, 0],
            );
            let cos_val = f32::from_bits(cos_xmm as u32);
            let dp_val = f32::from_bits(xmm6_dp as u32);
            if rd::<u8>(rbx_plane) != 0 && !(dp_val >= cos_val) {
                wr::<u16>(rbx_plane, 0x0100);
            }
            wr::<u128>(rbx_plane + 0x30, xmm7_norm);
            opt_50[0x10] = 0;
            if rd::<u8>(rbx_plane) != 0 && rd::<u8>(r10_elem + 0x03) != 0 {
                let bcast = vbroadcastss(rd::<u32>(r12));
                let prod = vmulps(bcast, rd::<u128>(rbx_plane + 0x10));
                wr::<u128>(opt_50.as_mut_ptr() as usize, prod);
                opt_50[0x10] = 1;
            }
        }
    }

    let rdx_38 = rd::<usize>(param_1 + 0x38);
    let al_flag: u8 = if rdx_38 != 0 {
        let rcx_off = rdx_38.wrapping_add(rd::<usize>(param_1 + 0x80).wrapping_mul(0x30));
        if rcx_off != 0 {
            rd::<u8>(rcx_off + 0x22)
        } else {
            0
        }
    } else {
        0
    };

    let mut vec_80 = [0u64; 2];
    if rd::<u8>(local_128.wrapping_add(rsi)) != 0 && al_flag == 0 {
        let mut arg_r8: u128 = rd::<u128>(local_res18);
        let mut arg_rdx: [u128; 2] = rd::<[u128; 2]>(r13);
        let xmm3_bits = rd::<u32>(r12);
        let rcx_obj = rd::<usize>(slot_r15);
        call_14217dde0(
            rcx_obj,
            arg_rdx.as_mut_ptr() as usize,
            (&mut arg_r8 as *mut u128) as usize,
            xmm3_bits,
        );
    } else if rd::<u8>(local_148.wrapping_add(rsi * 8)) != 0 {
        let mut arg_r8: u128 = rd::<u128>(local_res18);
        let mut arg_rdx: [u128; 2] = rd::<[u128; 2]>(local_160);
        let rcx_obj = rd::<usize>(slot_r15);
        exe::call_va!(
            0x1_4217_db10,
            (
                rcx_obj,
                arg_rdx.as_mut_ptr() as usize,
                (&mut arg_r8 as *mut u128) as usize
            )
        );
    } else {
        let slot_d0 = local_148.wrapping_add(rsi * 8);
        let mut cnt = rd::<i32>(slot_d0 + 0x04);
        let mut skip_to_0eb = false;
        if cnt > 0 {
            cnt = cnt.wrapping_sub(1);
            wr::<i32>(slot_d0 + 0x04, cnt);
            if cnt > 0 {
                skip_to_0eb = true;
            }
        }
        if !skip_to_0eb && rd::<u8>(slot_d0 + 0x01) != 0 {
            skip_to_0eb = true;
        }
        if skip_to_0eb {
            if rd::<u8>(local_1a8 + 0x01) == 0 {
                let mut arg_r8: u128 = rd::<u128>(local_res18);
                let mut arg_rdx: [u128; 2] = rd::<[u128; 2]>(r13);
                let xmm3_bits = rd::<u32>(r12);
                let rcx_obj = rd::<usize>(slot_r15);
                call_14217dde0(
                    rcx_obj,
                    arg_rdx.as_mut_ptr() as usize,
                    (&mut arg_r8 as *mut u128) as usize,
                    xmm3_bits,
                );
            }
        } else {
            let f30 = rd::<f32>(r13 + 0x30);
            if rd::<u8>(local_1a8 + 0x01) != 0 && !f30.is_nan() && f30 == 0.0f32 {
                let id_d08c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d08c) as usize) as usize;
                let p2_tbl = rd::<usize>(param_2_slot + 0x580e0);
                let val_d08c = rd::<u64>(p2_tbl.wrapping_add(id_d08c * 8));
                let mut closure_90: [u64; 4] = [
                    val_d08c,
                    param_1 as u64,
                    param_4 as u64,
                    (&mut param_2_slot as *mut usize) as u64,
                ];
                let vtbl_c40 = exe::ptr::<u8>(0x1_4315_9c40) as u64;
                let mut func_120 = [0u64; 8];
                func_120[0] = vtbl_c40;
                func_120[1] = closure_90.as_mut_ptr() as u64;
                func_120[7] = func_120.as_mut_ptr() as u64;

                let rdi_obj = rd::<usize>(slot_r15);
                let bl_flag = rd::<u8>(r10_elem + 0x03);
                let xmm7_u64 = rd::<u64>(r12 + 0x10);
                let xmm8_u32 = rd::<u32>(r12);
                let mut res18_copy: u128 = rd::<u128>(local_res18);
                let mut r13_copy: [u128; 2] = rd::<[u128; 2]>(r13);

                let mut func_160 = [0u64; 8];
                let copied_ptr = exe::call_va!(
                    0x1_41a4_a210,
                    (
                        func_120.as_mut_ptr() as usize,
                        func_160.as_mut_ptr() as usize
                    ) -> u64
                );
                func_160[7] = copied_ptr;

                let mut opt_100 = opt_50;
                let _ = exe::call_va10(
                    0x1_4217_e2a0,
                    rdi_obj as u64,
                    rd::<u64>(local_168),
                    func_160.as_mut_ptr() as u64,
                    r13_copy.as_mut_ptr() as u64,
                    (&mut res18_copy as *mut u128) as u64,
                    opt_100.as_mut_ptr() as u64,
                    xmm8_u32 as u64,
                    xmm7_u64,
                    bl_flag as u64,
                    vec_80.as_mut_ptr() as u64,
                );

                let rcx_c0 = func_120[7] as usize;
                if rcx_c0 != 0 {
                    let vtbl = rd::<usize>(rcx_c0);
                    let not_self = (rcx_c0 != func_120.as_ptr() as usize) as u8;
                    exe::call_va!(rd::<usize>(vtbl + 0x20), (rcx_c0, not_self));
                }
            } else {
                let mut arg_r8: u128 = rd::<u128>(local_res18);
                let mut arg_rdx: [u128; 2] = rd::<[u128; 2]>(r13);
                let xmm3_bits = rd::<u32>(r12);
                let rcx_obj = rd::<usize>(slot_r15);
                call_14217dde0(
                    rcx_obj,
                    arg_rdx.as_mut_ptr() as usize,
                    (&mut arg_r8 as *mut u128) as usize,
                    xmm3_bits,
                );
            }
        }
    }

    if rd::<u8>(local_158.wrapping_add(rsi * 8)) != 0 {
        let r11b = (rd::<i32>(local_158.wrapping_add(rsi * 8 + 4)) >= 1) as u8;
        let r10_obj = rd::<usize>(slot_r15);
        let eax_cnt = (vec_80[1] as u32) as u64;
        let r8_ptr = vec_80[0];
        let mut span_50: [u64; 2] = [eax_cnt, r8_ptr];
        let rax_e0 = rd::<usize>(r10_obj + 0xe0);
        let mut d0_vec: u128 = rd::<u128>(rax_e0 + 0x10);
        let arg5 = rd::<u64>(r10_obj + 0x78);
        let arg6 = rd::<u64>(r10_obj);
        let arg7 = (&mut d0_vec as *mut u128) as u64;
        let arg8 = rd::<u64>(r10_obj + 0x68);
        let arg9 = span_50.as_mut_ptr() as u64;
        let r9d = rd::<u8>(r10_elem + 0x03) as u64;
        let r8d = r11b as u64;
        let xmm1_bits = rd::<u32>(r12);
        let rcx_f0 = rd::<u64>(r10_obj + 0xf0);
        call_142176910(rcx_f0, xmm1_bits, r8d, r9d, arg5, arg6, arg7, arg8, arg9);
    }

    let final_obj = rd::<usize>(slot_r15);
    wr::<u128>(local_120 + 0x10, rd::<u128>(final_obj + 0x40));
    wr::<u128>(local_120 + 0x20, 0);
    if vec_80[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vec_80[0]));
    }
}

/// `unknown_140f4c9d0`
/// Original: 0x140f4c9d0 (2476 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_140f4c9d0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    let p4_world = rd::<usize>(param_4);
    let tbl_580e0 = rd::<usize>(p4_world + 0x580e0);
    let id_ce88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
    let val_ce88 = rd::<u64>(tbl_580e0.wrapping_add(id_ce88 * 8));
    let id_ceec = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceec) as usize) as usize;
    let val_ceec = rd::<usize>(tbl_580e0.wrapping_add(id_ceec * 8));

    let p3_world = rd::<usize>(param_3);
    let rbx_w40 = p3_world.wrapping_add(0x40);
    let mut span_30: [u64; 2] = rd::<[u64; 2]>(param_3 + 0x08);
    let mut iter_50 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_50.as_mut_ptr() as usize,
            rbx_w40,
            span_30.as_mut_ptr() as usize
        )
    );

    let iter_base = iter_50[0] as usize;
    let mut cur_arch_ptr = iter_50[1] as usize;
    let end_arch_ptr = iter_50[2] as usize;
    let mut ent_count: usize = 0;
    let mut ent_idx: usize = 0;
    let mut comp_base: usize = 0;

    if cur_arch_ptr != end_arch_ptr {
        let arch = rd::<u16>(cur_arch_ptr) as usize;
        let block_ptr = rd::<usize>(rbx_w40.wrapping_add(arch * 8).wrapping_add(0x48));
        ent_count = if block_ptr != 0 {
            rd::<u32>(rbx_w40.wrapping_add(arch * 4).wrapping_add(0x10048)) as usize
        } else {
            0
        };
        let arch_off = arch.wrapping_shl(5);
        let search_len = rd::<u32>(rbx_w40.wrapping_add(arch_off).wrapping_add(0x1806c)) as u64;
        let want_cecc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u64;
        let search_ptr = rd::<u64>(rbx_w40.wrapping_add(arch_off).wrapping_add(0x18058));
        let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_cecc) -> i32);
        if (found as isize) != -1 {
            let tbl = rd::<usize>(rbx_w40.wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5)));
            let off = rd::<u32>(tbl.wrapping_add((found as isize as usize) * 4)) as usize;
            if off != 0 {
                comp_base = off.wrapping_add(block_ptr);
            }
        }
    }

    let mut r14b_flag: u8 = 0;
    if ent_count != 0 {
        let empty_cstr = exe::ptr::<u8>(0x1_426c_dcb0) as u64;
        let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;
        loop {
            let rsi_elem = comp_base.wrapping_add(ent_idx.wrapping_mul(0x140));
            let rcx_idx = rd::<i32>(rsi_elem + 0x11c) as isize;
            let mut temp_str = [0u64; 5];
            temp_str[0] = 0x0400_0004;
            wr::<u8>(temp_str.as_mut_ptr() as usize + 0x08, 0);

            if rcx_idx > -1 && (rcx_idx as usize) < (rd::<u32>(rsi_elem + 0x128) as usize) {
                let rdx_arr = rd::<usize>(rsi_elem + 0x120);
                let rax_src = rdx_arr.wrapping_add((rcx_idx as usize).wrapping_mul(0x28));
                if temp_str.as_ptr() as usize != rax_src {
                    let r9d_len = rd::<u32>(rax_src + 0x04) as u64;
                    let rdx_data = sso_str_ptr(rax_src);
                    let mut view_50: [u64; 2] = [rdx_data as u64, r9d_len];
                    exe::call_va!(
                        0x1_4240_9b50,
                        (
                            temp_str.as_mut_ptr() as usize,
                            view_50.as_mut_ptr() as usize,
                            0u64,
                            r9d_len
                        )
                    );
                }
            } else {
                let mut view_30: [u64; 2] = [empty_cstr, 0];
                exe::call_va!(
                    0x1_4240_9b50,
                    (
                        temp_str.as_mut_ptr() as usize,
                        view_30.as_mut_ptr() as usize,
                        0u64,
                        0u64
                    )
                );
            }

            let rcx_s1 = sso_str_ptr(param_1 + 0x28);
            let r8_len1 = rd::<u32>(param_1 + 0x2c) as usize;
            let ts_ptr = temp_str.as_ptr() as usize;
            let r10_s2 = sso_str_ptr(ts_ptr);
            let r9_len2 = rd::<u32>(ts_ptr + 0x04) as usize;

            let mut matched = false;
            if r9_len2 == r8_len1 {
                let mut eq = true;
                for i in 0..r9_len2 {
                    let c2 = rd::<i8>(lut.wrapping_add(rd::<u8>(r10_s2 + i) as usize));
                    let c1 = rd::<i8>(lut.wrapping_add(rd::<u8>(rcx_s1 + i) as usize));
                    if c2 != c1 {
                        eq = false;
                        break;
                    }
                }
                if eq {
                    let ok = exe::call_va!(0x1_41df_aaa0, (rsi_elem, param_3) -> u8);
                    if ok != 0 {
                        matched = true;
                    }
                }
            }

            exe::call_va!(0x1_4240_ab10, (temp_str.as_mut_ptr() as usize));

            if matched {
                if rd::<u64>(exe::ptr::<u64>(0x1_43a3_40f8) as usize) != 0 {
                    let (dil, r14b_k, sil_k, al_k) = if rd::<u8>(param_1 + 0x5d) != 0 {
                        let d = rd::<u8>(param_1 + 0x5e);
                        let r = rd::<u8>(param_1 + 0x5f);
                        let s = rd::<u8>(param_1 + 0x60);
                        let a = rd::<u8>(param_1 + 0x61);
                        wr::<u32>(param_1 + 0x5e, 0);
                        (d, r, s, a)
                    } else {
                        let ctx0 = exe::call_va!(0x1_420e_8920, () -> usize);
                        let rbx_in = if rd::<u8>(param_1 + 0x5c) != 0 {
                            exe::call_va!(0x1_420e_8800, (ctx0) -> usize)
                        } else {
                            exe::call_va!(0x1_420e_8750, (ctx0) -> usize)
                        };
                        let k1 = rd::<u16>(exe::ptr::<u16>(0x1_4397_da34) as usize) as u32;
                        let d = exe::call_va!(0x1_420e_3850, (rbx_in, k1) -> u8);
                        let k2 = rd::<u16>(exe::ptr::<u16>(0x1_4397_da3c) as usize) as u32;
                        let r = exe::call_va!(0x1_420e_3850, (rbx_in, k2) -> u8);
                        let k3 = rd::<u16>(exe::ptr::<u16>(0x1_4397_da2c) as usize) as u32;
                        let s = exe::call_va!(0x1_420e_3850, (rbx_in, k3) -> u8);
                        let k4 = rd::<u16>(exe::ptr::<u16>(0x1_4397_da30) as usize) as u32;
                        let a = exe::call_va!(0x1_420e_3850, (rbx_in, k4) -> u8);
                        (d, r, s, a)
                    };

                    let base_y: i32 = if dil != 0 { -1 } else { 0 };
                    let ebx_dy: i32 = if r14b_k != 0 { base_y + 1 } else { base_y };
                    let base_x: i32 = if sil_k != 0 { -1 } else { 0 };
                    let edi_dx: i32 = if al_k != 0 { base_x + 1 } else { base_x };

                    if edi_dx != 0 || ebx_dy != 0 || rd::<u8>(param_1 + 0x56) != 0 {
                        let mut vec_buf = [0u64; 3];
                        let rax_vec = exe::call_va!(
                            0x1_40f4_d380,
                            (vec_buf.as_mut_ptr() as usize, param_1, param_2) -> usize
                        );
                        let mut span_50: [u64; 2] =
                            [rd::<u32>(rax_vec + 0x08) as u64, rd::<u64>(rax_vec)];
                        let new_idx = exe::call_va!(
                            0x1_40f4_ddd0,
                            (
                                param_1,
                                param_2,
                                span_50.as_mut_ptr() as usize,
                                edi_dx as u32,
                                ebx_dy as u32
                            ) -> u32
                        );
                        if vec_buf[0] != 0 {
                            exe::call_va!(0x1_4241_c190, (vec_buf[0]));
                        }

                        if new_idx == rd::<u32>(param_1 + 0x50) && rd::<u8>(param_1 + 0x56) == 0 {
                            let str_p1 = sso_str_ptr(param_1);
                            let rsi_slot =
                                exe::call_va!(0x1_4020_c520, (val_ceec.wrapping_add(0x78)) -> usize);
                            wr::<u32>(rsi_slot, 0x0c);
                            let r9_d0 = rd::<u64>(val_ceec + 0xd0);
                            let r8_b0 = rd::<usize>(val_ceec + 0xb0);
                            let edi_c8 = rd::<u32>(val_ceec + 0xc8);
                            let packed = edi_c8
                                .wrapping_sub(rd::<u32>(r8_b0.wrapping_add((r9_d0 as usize) * 8)))
                                | ((r9_d0 as u32) << 16);
                            exe::call_va!(0x1_4022_6cb0, (val_ceec.wrapping_add(0xb0), str_p1));
                            let rax_b = exe::call_va!(
                                0x1_4020_bfd0,
                                (val_ceec.wrapping_add(0xb0), 1u32) -> usize
                            );
                            wr::<u8>(rax_b, 0x0c);
                            wr::<u32>(rsi_slot + 0x04, packed);
                            wr::<u32>(rsi_slot + 0x08, 0xffff_ffff);
                            r14b_flag = 0;
                        } else {
                            wr::<u16>(param_1 + 0x55, 1);
                            wr::<u32>(param_1 + 0x50, new_idx);
                            wr::<u8>(param_1 + 0x54, 1);
                            r14b_flag = 1;
                            let str_p1 = sso_str_ptr(param_1);
                            let rsi_slot =
                                exe::call_va!(0x1_4020_c520, (val_ceec.wrapping_add(0x78)) -> usize);
                            wr::<u32>(rsi_slot, 0x0b);
                            let r9_d0 = rd::<u64>(val_ceec + 0xd0);
                            let r8_b0 = rd::<usize>(val_ceec + 0xb0);
                            let edi_c8 = rd::<u32>(val_ceec + 0xc8);
                            let packed = edi_c8
                                .wrapping_sub(rd::<u32>(r8_b0.wrapping_add((r9_d0 as usize) * 8)))
                                | ((r9_d0 as u32) << 16);
                            exe::call_va!(0x1_4022_6cb0, (val_ceec.wrapping_add(0xb0), str_p1));
                            let rax_b = exe::call_va!(
                                0x1_4020_bfd0,
                                (val_ceec.wrapping_add(0xb0), 1u32) -> usize
                            );
                            wr::<u8>(rax_b, 0x0c);
                            wr::<u32>(rsi_slot + 0x04, packed);
                            wr::<u32>(rsi_slot + 0x08, 0xffff_ffff);
                        }
                    }
                }
                break;
            }

            ent_idx += 1;
            if ent_idx >= ent_count {
                cur_arch_ptr = cur_arch_ptr.wrapping_add(2);
                while cur_arch_ptr != end_arch_ptr {
                    let a = rd::<u16>(cur_arch_ptr) as usize;
                    if rd::<u64>(iter_base.wrapping_add(a * 8).wrapping_add(0x48)) != 0
                        && rd::<u32>(iter_base.wrapping_add(a * 4).wrapping_add(0x10048)) != 0
                    {
                        break;
                    }
                    cur_arch_ptr = cur_arch_ptr.wrapping_add(2);
                }
                if cur_arch_ptr != end_arch_ptr {
                    let arch = rd::<u16>(cur_arch_ptr) as usize;
                    let block_ptr = rd::<usize>(rbx_w40.wrapping_add(arch * 8).wrapping_add(0x48));
                    ent_count = if block_ptr != 0 {
                        rd::<u32>(rbx_w40.wrapping_add(arch * 4).wrapping_add(0x10048)) as usize
                    } else {
                        0
                    };
                    ent_idx = 0;
                    let arch_off = arch.wrapping_shl(5);
                    let search_len =
                        rd::<u32>(rbx_w40.wrapping_add(arch_off).wrapping_add(0x1806c)) as u64;
                    let want_cecc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u64;
                    let search_ptr =
                        rd::<u64>(rbx_w40.wrapping_add(arch_off).wrapping_add(0x18058));
                    let found =
                        exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_cecc) -> i32);
                    comp_base = 0;
                    if (found as isize) != -1 {
                        let tbl = rd::<usize>(
                            rbx_w40.wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5)),
                        );
                        let off = rd::<u32>(tbl.wrapping_add((found as isize as usize) * 4)) as usize;
                        if off != 0 {
                            comp_base = off.wrapping_add(block_ptr);
                        }
                    }
                }
                if ent_idx >= ent_count {
                    break;
                }
            }
        }
    }

    if rd::<u8>(param_1 + 0x54) != 0 {
        wr::<u8>(param_1 + 0x54, 0);
        let mut flag_110: u8 = r14b_flag;
        let mut vec_30 = [0u64; 3];
        exe::call_va!(
            0x1_40f4_d380,
            (vec_30.as_mut_ptr() as usize, param_1, param_2)
        );
        let r8_idx = rd::<i32>(param_1 + 0x50) as isize;
        let rcx_ptr = vec_30[0] as usize;
        let eax_len = (vec_30[1] as u32) as usize;
        let mut val_100: u32 = if r8_idx > -1 && (r8_idx as usize) < eax_len {
            let v = rd::<u32>(rcx_ptr.wrapping_add((r8_idx as usize) * 4));
            if rcx_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (rcx_ptr));
            }
            v
        } else {
            if rcx_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (rcx_ptr));
            }
            0xffff_ffff
        };

        let r14_p1_28 = param_1.wrapping_add(0x28);
        let mut fmt1: [u64; 2] = [exe::ptr::<u8>(0x1_430a_0098) as u64, 0x1d];
        let rbx_str1 = exe::call_va!(
            0x1_4021_a880,
            (fmt1.as_mut_ptr() as usize, r14_p1_28) -> usize
        );

        let mut fmt2: [u64; 2] = [exe::ptr::<u8>(0x1_4330_fa68) as u64, 0x2a];
        let rdi_str2 = exe::call_va!(
            0x1_4021_a880,
            (fmt2.as_mut_ptr() as usize, r14_p1_28) -> usize
        );

        let mut len1 = 0usize;
        while rd::<u8>(rbx_str1.wrapping_add(len1)) != 0 {
            len1 += 1;
        }
        let hash1 = exe::call_va!(0x1_424b_16a0, (rbx_str1, len1 as u64) -> u64);
        let mut pair1: [u64; 2] = [0, hash1];
        exe::call_va!(
            0x1_401f_1a80,
            (
                val_ce88,
                pair1.as_mut_ptr() as usize,
                0u64,
                (&mut val_100 as *mut u32) as usize,
                0u64
            )
        );

        let mut len2 = 0usize;
        while rd::<u8>(rdi_str2.wrapping_add(len2)) != 0 {
            len2 += 1;
        }
        let hash2 = exe::call_va!(0x1_424b_16a0, (rdi_str2, len2 as u64) -> u64);
        let mut pair2: [u64; 2] = [0, hash2];
        exe::call_va!(
            0x1_401e_acd0,
            (
                val_ce88,
                pair2.as_mut_ptr() as usize,
                0u64,
                (&mut flag_110 as *mut u8) as usize,
                0u64
            )
        );
    }

    if rd::<u8>(param_1 + 0x55) != 0 {
        wr::<u8>(param_1 + 0x55, 0);
        let id_ce6c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce6c) as usize) as usize;
        let p4_w = rd::<usize>(param_4);
        let rbx_ce6c = rd::<u64>(rd::<usize>(p4_w + 0x580e0).wrapping_add(id_ce6c * 8));

        let mut elems = [0u8; 0xa0];
        let elems_ptr = elems.as_mut_ptr() as usize;
        wr::<u32>(elems_ptr, 0x0800_0008);
        wr::<u32>(elems_ptr + 0x04, 0);
        wr::<u8>(elems_ptr + 0x08, 0);
        exe::call_va!(0x1_4240_aa00, (elems_ptr, param_1.wrapping_add(0x28)));
        wr::<u8>(elems_ptr + 0x48, 2);

        let mut vec_30 = [0u64; 3];
        exe::call_va!(
            0x1_40f4_d380,
            (vec_30.as_mut_ptr() as usize, param_1, param_2)
        );
        let r8_idx = rd::<i32>(param_1 + 0x50) as isize;
        let rcx_ptr = vec_30[0] as usize;
        let eax_len = (vec_30[1] as u32) as usize;
        let esi_val: i32 = if r8_idx > -1 && (r8_idx as usize) < eax_len {
            let v = rd::<i32>(rcx_ptr.wrapping_add((r8_idx as usize) * 4));
            if rcx_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (rcx_ptr));
            }
            v
        } else {
            if rcx_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (rcx_ptr));
            }
            -1
        };

        wr::<f64>(elems_ptr + 0x50, esi_val as f64);
        wr::<u8>(elems_ptr + 0x98, 1);

        let mut span_30: [usize; 2] = [elems_ptr, elems_ptr.wrapping_add(0xa0)];
        let mut out_50 = [0u64; 4];
        let rax_dc60 = exe::call_va!(
            0x1_40f4_dc60,
            (out_50.as_mut_ptr() as usize, span_30.as_mut_ptr() as usize) -> usize
        );
        let mut nav_view: [u64; 2] = [exe::ptr::<u8>(0x1_4309_c050) as u64, 0x0f];
        exe::call_va!(
            0x1_41e0_d570,
            (
                rbx_ce6c,
                nav_view.as_mut_ptr() as usize,
                rax_dc60,
                0u64
            )
        );
        exe::call_va!(
            0x1_4183_4ee8,
            (
                elems_ptr,
                0x50u64,
                2u64,
                exe::ptr::<u8>(0x1_401e_fe90) as usize
            )
        );
    }
}

/// `unknown_140f55140`
/// Original: 0x140f55140 (1605 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f55140(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let w1 = rd::<usize>(param_1);
    let w1_40 = w1.wrapping_add(0x40);
    let mut span1: [u64; 2] = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter1 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter1.as_mut_ptr() as usize,
            w1_40,
            span1.as_mut_ptr() as usize
        )
    );

    let iter1_base = iter1[0] as usize;
    let mut cur_arch1 = iter1[1] as usize;
    let end_arch1 = iter1[2] as usize;
    let mut cnt1: usize = 0;
    let mut idx1: usize = 0;
    let mut comp1_base: usize = 0;

    if cur_arch1 != end_arch1 {
        let arch1 = rd::<u16>(cur_arch1) as usize;
        let block1 = rd::<usize>(w1_40.wrapping_add(arch1 * 8).wrapping_add(0x48));
        cnt1 = if block1 != 0 {
            rd::<u32>(w1_40.wrapping_add(arch1 * 4).wrapping_add(0x10048)) as usize
        } else {
            0
        };
        let arch1_off = arch1.wrapping_shl(5);
        let search_len1 = rd::<u32>(w1_40.wrapping_add(arch1_off).wrapping_add(0x1806c)) as u64;
        let want_0338 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0338) as usize) as u64;
        let search_ptr1 = rd::<u64>(w1_40.wrapping_add(arch1_off).wrapping_add(0x18058));
        let found1 = exe::call_va!(0x1_4248_9ee0, (search_ptr1, search_len1, want_0338) -> i32);
        if (found1 as isize) != -1 {
            let tbl1 = rd::<usize>(w1_40.wrapping_add(arch1.wrapping_add(0xc03).wrapping_shl(5)));
            let off1 = rd::<u32>(tbl1.wrapping_add((found1 as isize as usize) * 4)) as usize;
            if off1 != 0 {
                comp1_base = off1.wrapping_add(block1);
            }
        }
    }

    if cnt1 == 0 {
        return;
    }

    let empty_cstr = exe::ptr::<u8>(0x1_426c_dcb0) as u64;
    let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;

    loop {
        let elem1 = comp1_base.wrapping_add(idx1.wrapping_mul(0x6c));
        let w2 = rd::<usize>(param_2);
        let w2_40 = w2.wrapping_add(0x40);
        let mut span2: [u64; 2] = rd::<[u64; 2]>(param_2 + 0x08);
        let mut iter2 = [0u64; 3];
        exe::call_va!(
            0x1_401e_7c10,
            (
                iter2.as_mut_ptr() as usize,
                w2_40,
                span2.as_mut_ptr() as usize
            )
        );

        let iter2_base = iter2[0] as usize;
        let mut cur_arch2 = iter2[1] as usize;
        let end_arch2 = iter2[2] as usize;
        let mut cnt2: usize = 0;
        let mut idx2: usize = 0;
        let mut comp2_base: usize = 0;

        if cur_arch2 != end_arch2 {
            let arch2 = rd::<u16>(cur_arch2) as usize;
            let block2 = rd::<usize>(w2_40.wrapping_add(arch2 * 8).wrapping_add(0x48));
            cnt2 = if block2 != 0 {
                rd::<u32>(w2_40.wrapping_add(arch2 * 4).wrapping_add(0x10048)) as usize
            } else {
                0
            };
            let arch2_off = arch2.wrapping_shl(5);
            let search_len2 =
                rd::<u32>(w2_40.wrapping_add(arch2_off).wrapping_add(0x1806c)) as u64;
            let want_cecc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u64;
            let search_ptr2 = rd::<u64>(w2_40.wrapping_add(arch2_off).wrapping_add(0x18058));
            let found2 =
                exe::call_va!(0x1_4248_9ee0, (search_ptr2, search_len2, want_cecc) -> i32);
            if (found2 as isize) != -1 {
                let tbl2 =
                    rd::<usize>(w2_40.wrapping_add(arch2.wrapping_add(0xc03).wrapping_shl(5)));
                let off2 = rd::<u32>(tbl2.wrapping_add((found2 as isize as usize) * 4)) as usize;
                if off2 != 0 {
                    comp2_base = off2.wrapping_add(block2);
                }
            }
        }

        if cnt2 != 0 {
            loop {
                let rsi_elem2 = comp2_base.wrapping_add(idx2.wrapping_mul(0x140));
                let rcx_idx = rd::<i32>(rsi_elem2 + 0x11c) as isize;
                let mut temp_str = [0u64; 5];
                temp_str[0] = 0x0400_0004;
                wr::<u8>(temp_str.as_mut_ptr() as usize + 0x08, 0);

                if rcx_idx > -1 && (rcx_idx as usize) < (rd::<u32>(rsi_elem2 + 0x128) as usize) {
                    let rdx_arr = rd::<usize>(rsi_elem2 + 0x120);
                    let rax_src = rdx_arr.wrapping_add((rcx_idx as usize).wrapping_mul(0x28));
                    if temp_str.as_ptr() as usize != rax_src {
                        let r9d_len = rd::<u32>(rax_src + 0x04) as u64;
                        let rdx_data = sso_str_ptr(rax_src);
                        let mut view_50: [u64; 2] = [rdx_data as u64, r9d_len];
                        exe::call_va!(
                            0x1_4240_9b50,
                            (
                                temp_str.as_mut_ptr() as usize,
                                view_50.as_mut_ptr() as usize,
                                0u64,
                                r9d_len
                            )
                        );
                    }
                } else {
                    let mut view_60: [u64; 2] = [empty_cstr, 0];
                    exe::call_va!(
                        0x1_4240_9b50,
                        (
                            temp_str.as_mut_ptr() as usize,
                            view_60.as_mut_ptr() as usize,
                            0u64,
                            0u64
                        )
                    );
                }

                let rcx_s1 = sso_str_ptr(elem1);
                let r8_len1 = rd::<u32>(elem1 + 0x04) as usize;
                let ts_ptr = temp_str.as_ptr() as usize;
                let r10_s2 = sso_str_ptr(ts_ptr);
                let r9_len2 = rd::<u32>(ts_ptr + 0x04) as usize;

                let mut matched = false;
                if r9_len2 == r8_len1 {
                    let mut eq = true;
                    for i in 0..r9_len2 {
                        let c2 = rd::<i8>(lut.wrapping_add(rd::<u8>(r10_s2 + i) as usize));
                        let c1 = rd::<i8>(lut.wrapping_add(rd::<u8>(rcx_s1 + i) as usize));
                        if c2 != c1 {
                            eq = false;
                            break;
                        }
                    }
                    if eq {
                        let ok = exe::call_va!(0x1_41df_aaa0, (rsi_elem2, param_2) -> u8);
                        if ok != 0 && rd::<u8>(elem1 + 0x5c) != 0 {
                            matched = true;
                        }
                    }
                }

                exe::call_va!(0x1_4240_ab10, (temp_str.as_mut_ptr() as usize));

                if matched {
                    wr::<u8>(param_3, 1);
                    break;
                }

                idx2 += 1;
                if idx2 >= cnt2 {
                    loop {
                        cur_arch2 = cur_arch2.wrapping_add(2);
                        if cur_arch2 == end_arch2 {
                            break;
                        }
                        let a2 = rd::<u16>(cur_arch2) as usize;
                        if rd::<u64>(iter2_base.wrapping_add(a2 * 8).wrapping_add(0x48)) != 0
                            && rd::<u32>(iter2_base.wrapping_add(a2 * 4).wrapping_add(0x10048)) != 0
                        {
                            break;
                        }
                    }
                    if cur_arch2 != end_arch2 {
                        let arch2 = rd::<u16>(cur_arch2) as usize;
                        let block2 = rd::<usize>(w2_40.wrapping_add(arch2 * 8).wrapping_add(0x48));
                        cnt2 = if block2 != 0 {
                            rd::<u32>(w2_40.wrapping_add(arch2 * 4).wrapping_add(0x10048)) as usize
                        } else {
                            0
                        };
                        idx2 = 0;
                        let arch2_off = arch2.wrapping_shl(5);
                        let search_len2 =
                            rd::<u32>(w2_40.wrapping_add(arch2_off).wrapping_add(0x1806c)) as u64;
                        let want_cecc =
                            rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u64;
                        let search_ptr2 =
                            rd::<u64>(w2_40.wrapping_add(arch2_off).wrapping_add(0x18058));
                        let found2 = exe::call_va!(
                            0x1_4248_9ee0,
                            (search_ptr2, search_len2, want_cecc) -> i32
                        );
                        comp2_base = 0;
                        if (found2 as isize) != -1 {
                            let tbl2 = rd::<usize>(
                                w2_40.wrapping_add(arch2.wrapping_add(0xc03).wrapping_shl(5)),
                            );
                            let off2 =
                                rd::<u32>(tbl2.wrapping_add((found2 as isize as usize) * 4))
                                    as usize;
                            if off2 != 0 {
                                comp2_base = off2.wrapping_add(block2);
                            }
                        }
                    }
                    if idx2 >= cnt2 {
                        break;
                    }
                }
            }
        }

        idx1 += 1;
        if idx1 >= cnt1 {
            loop {
                cur_arch1 = cur_arch1.wrapping_add(2);
                if cur_arch1 == end_arch1 {
                    break;
                }
                let a1 = rd::<u16>(cur_arch1) as usize;
                if rd::<u64>(iter1_base.wrapping_add(a1 * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter1_base.wrapping_add(a1 * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
            }
            if cur_arch1 != end_arch1 {
                let arch1 = rd::<u16>(cur_arch1) as usize;
                let block1 = rd::<usize>(w1_40.wrapping_add(arch1 * 8).wrapping_add(0x48));
                cnt1 = if block1 != 0 {
                    rd::<u32>(w1_40.wrapping_add(arch1 * 4).wrapping_add(0x10048)) as usize
                } else {
                    0
                };
                idx1 = 0;
                let arch1_off = arch1.wrapping_shl(5);
                let search_len1 =
                    rd::<u32>(w1_40.wrapping_add(arch1_off).wrapping_add(0x1806c)) as u64;
                let want_0338 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0338) as usize) as u64;
                let search_ptr1 = rd::<u64>(w1_40.wrapping_add(arch1_off).wrapping_add(0x18058));
                let found1 = exe::call_va!(
                    0x1_4248_9ee0,
                    (search_ptr1, search_len1, want_0338) -> i32
                );
                comp1_base = 0;
                if (found1 as isize) != -1 {
                    let tbl1 = rd::<usize>(
                        w1_40.wrapping_add(arch1.wrapping_add(0xc03).wrapping_shl(5)),
                    );
                    let off1 =
                        rd::<u32>(tbl1.wrapping_add((found1 as isize as usize) * 4)) as usize;
                    if off1 != 0 {
                        comp1_base = off1.wrapping_add(block1);
                    }
                }
            }
            if idx1 >= cnt1 {
                break;
            }
        }
    }
}

/// `rend_1412eeb30`
/// Original: 0x1412eeb30 (1055 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412eeb30(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: u8,
) -> usize {
    let last0 = rd::<usize>(param_3);
    let first0 = rd::<usize>(param_2);
    if first0 == last0 {
        wr::<usize>(param_1, last0);
        return param_1;
    }
    let mut cur = first0.wrapping_add(0xa0);
    if cur == last0 {
        wr::<usize>(param_1, last0);
        return param_1;
    }

    loop {
        let saved = rd::<[u8; 0xa0]>(cur);
        let key = rd::<i32>(cur + 0x08);
        let first = rd::<usize>(param_2);
        let insert_pos = if key < rd::<i32>(first + 0x08) {
            if first != cur {
                let mut src = cur;
                let mut dst = cur.wrapping_add(0xa0);
                loop {
                    src = src.wrapping_sub(0xa0);
                    dst = dst.wrapping_sub(0xa0);
                    wr::<[u8; 0xa0]>(dst, rd::<[u8; 0xa0]>(src));
                    if src == first {
                        break;
                    }
                }
            }
            rd::<usize>(param_2)
        } else {
            let mut dst = cur;
            let mut src = cur.wrapping_sub(0xa0);
            while key < rd::<i32>(src + 0x08) {
                wr::<[u8; 0xa0]>(dst, rd::<[u8; 0xa0]>(src));
                dst = src;
                src = src.wrapping_sub(0xa0);
            }
            dst
        };

        wr::<[u8; 0xa0]>(insert_pos, saved);
        cur = cur.wrapping_add(0xa0);
        let last = rd::<usize>(param_3);
        if cur == last {
            wr::<usize>(param_1, last);
            return param_1;
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1404b5940(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: [u64; 16] = [0; 16];

    unsafe extern "win64" fn mock_6f70(p3: usize, out_buf: usize, handle: u64) -> usize {
        let sub_arr = rd::<usize>(p3 + 0x20);
        wr::<u64>(out_buf, sub_arr as u64);
        wr::<u64>(out_buf + 0x08, handle);
        wr::<u64>(out_buf + 0x10, 1);
        out_buf
    }

    unsafe extern "win64" fn mock_7000(
        out_16: usize,
        v_168: u64,
        r8_ptr: usize,
        tmp_50: usize,
        b_flag: u8,
    ) -> usize {
        let tag = rd::<u64>(r8_ptr);
        let t0 = rd::<u64>(tmp_50);
        wr::<u64>(out_16, v_168 ^ tag ^ (b_flag as u64));
        wr::<u64>(out_16 + 0x08, t0);
        out_16
    }

    unsafe extern "win64" fn mock_dde0(rcx_obj: usize, rdx_ptr: usize, r8_ptr: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        let f3 = xmm[3] as u32;
        unsafe {
            CALL_TRACE[0] = 0xdde0;
            CALL_TRACE[1] = rd::<u64>(rdx_ptr) ^ rd::<u64>(r8_ptr) ^ (f3 as u64);
        }
        wr::<u64>(rcx_obj + 0x40, 0x1111_2222_3333_4444);
        wr::<u64>(rcx_obj + 0x48, 0x5555_6666_7777_8888);
    }

    unsafe extern "win64" fn mock_db10(rcx_obj: usize, rdx_ptr: usize, r8_ptr: usize) {
        unsafe {
            CALL_TRACE[0] = 0xdb10;
            CALL_TRACE[1] = rd::<u64>(rdx_ptr) ^ rd::<u64>(r8_ptr);
        }
        wr::<u64>(rcx_obj + 0x40, 0xaaaa_bbbb_cccc_dddd);
        wr::<u64>(rcx_obj + 0x48, 0xeeee_ffff_0000_1111);
    }

    unsafe extern "win64" fn mock_e2a0(
        rcx_obj: usize,
        rdx_168: u64,
        r8_func: usize,
        r9_r13: usize,
        a5_res18: usize,
        a6_opt: usize,
        a7_f32: u32,
        a8_f64: u64,
        a9_bl: u8,
        a10_vec80: usize,
    ) {
        let closure_ptr = rd::<usize>(r8_func + 0x08);
        let c_d08c = rd::<u64>(closure_ptr);
        let has_opt = rd::<u8>(a6_opt + 0x10);
        let opt_lo = if has_opt != 0 { rd::<u64>(a6_opt) } else { 0 };
        unsafe {
            CALL_TRACE[2] = rdx_168 ^ c_d08c ^ (a7_f32 as u64) ^ a8_f64 ^ (a9_bl as u64);
            CALL_TRACE[3] = rd::<u64>(r9_r13) ^ rd::<u64>(a5_res18) ^ opt_lo;
        }
        wr::<u64>(a10_vec80, 0xdead_0000);
        wr::<u32>(a10_vec80 + 0x08, 3);
        wr::<u64>(rcx_obj + 0x40, 0x1234_5678_9abc_def0);
        wr::<u64>(rcx_obj + 0x48, 0x0fed_cba9_8765_4321);
    }

    unsafe extern "win64" fn mock_6910(
        rcx_f0: usize,
        _rdx: u64,
        r8_b: u8,
        r9_b: u8,
        a5_78: u64,
        a6_00: u64,
        a7_d0: usize,
        a8_68: u64,
        a9_span: usize,
    ) {
        let xmm = crate::simd_ops::take_xmm_args();
        let f1 = xmm[1] as u32;
        unsafe {
            CALL_TRACE[4] = (rcx_f0 != 0) as u64
                ^ ((r8_b as u64) << 8)
                ^ ((r9_b as u64) << 16)
                ^ (f1 as u64);
            CALL_TRACE[5] = a5_78 ^ rd::<u64>(a7_d0) ^ rd::<u64>(a9_span) ^ rd::<u64>(a9_span + 8);
            let _ = (a6_00, a8_68);
        }
    }

    unsafe extern "win64" fn mock_c190(ptr: usize) {
        unsafe {
            CALL_TRACE[6] = ptr as u64;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_404b_6f70, mock_6f70 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fb_7000, mock_7000 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4217_dde0, mock_dde0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4217_db10, mock_db10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4217_e2a0, mock_e2a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4217_6910, mock_6910 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let g_f63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_d910 = exe::ptr::<u16>(0x1_43c8_d910) as usize;
    let g_d08c = exe::ptr::<u16>(0x1_43c8_d08c) as usize;
    let s_f63c = rd::<u16>(g_f63c);
    let s_d910 = rd::<u16>(g_d910);
    let s_d08c = rd::<u16>(g_d08c);
    wr::<u16>(g_f63c, 0);
    wr::<u16>(g_d910, 1);
    wr::<u16>(g_d08c, 2);

    let run =
        |f: F, mode: u32| -> ([u64; 16], [u8; 0x30], [u8; 0x60], [u8; 0x50], [u8; 0x20]) {
        unsafe {
            CALL_TRACE = [0; 16];
        }
        let val_168: u64 = 0x7777_8888;
        let mut r12_buf = [0u8; 0x20];
        let r12p = r12_buf.as_mut_ptr() as usize;
        wr::<f32>(r12p, 2.5f32);
        wr::<u64>(r12p + 0x10, 0x3ff0_0000_0000_0000);

        let tbl_580e0: [u64; 3] = [
            (&val_168 as *const u64) as u64,
            r12p as u64,
            0x9999_aaaa,
        ];
        let mut world = vec![0u8; 0x58100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let mut p0_arr = [0u8; 0x20];
        let p0p = p0_arr.as_mut_ptr() as usize;
        wr::<u8>(p0p, 11);
        wr::<u8>(p0p + 1, 22);
        wr::<u8>(p0p + 2, 33);
        wr::<u8>(p0p + 3, 1);
        wr::<u32>(p0p + 4, 1);
        wr::<u32>(p0p + 8, 7);

        let mut p1_arr = [0u8; 0x10];
        let p1p = p1_arr.as_mut_ptr() as usize;
        wr::<u8>(p1p, if mode == 1 { 1 } else { 0 });
        wr::<u8>(p1p + 1, 0);
        wr::<i32>(p1p + 4, 1);

        let p2_arr: [u8; 8] = [1, 1, 44, 0, 0, 0, 0, 0];
        let mut p3_arr = [0u8; 0x10];
        wr::<u8>(p3_arr.as_mut_ptr() as usize, 1);
        wr::<i32>(p3_arr.as_mut_ptr() as usize + 4, 2);

        let mut p4_arr = [0u8; 0x40];
        let p4p = p4_arr.as_mut_ptr() as usize;
        wr::<[f32; 4]>(p4p, [1.0, 2.0, 3.0, 4.0]);
        wr::<[f32; 4]>(p4p + 0x10, [0.0, 1.0, 0.0, 0.0]);
        wr::<f32>(p4p + 0x30, 0.0);

        let mut p5_arr = [0u8; 0x20];
        wr::<[u64; 2]>(p5_arr.as_mut_ptr() as usize, [0x101, 0x202]);

        let mut p6_arr = [0u8; 0x20];
        wr::<[u64; 4]>(p6_arr.as_mut_ptr() as usize, [0x301, 0x302, 0x303, 0x304]);

        let mut p8_plane = [0u8; 0x90];
        let p8p = p8_plane.as_mut_ptr() as usize;
        wr::<u8>(p8p, 1);
        wr::<[f32; 4]>(p8p + 0x10, [1.0, 0.0, 0.0, 0.0]);
        wr::<[f32; 4]>(p8p + 0x40, [0.0, 1.0, 0.0, 0.0]);

        let mut inner_0 = [0u8; 0x40];
        wr::<[f32; 4]>(inner_0.as_mut_ptr() as usize, [0.0, 0.0, 0.0, 1.0]);

        let mut obj_68 = [0u8; 0x60];
        let mut f0_items = [0u8; 0x188];
        let f0ip = f0_items.as_mut_ptr() as usize;
        wr::<u8>(f0ip + 0x30, 1);
        wr::<u64>(f0ip + 0x34, 0xffff_ffff);

        let mut obj_f0 = [0u8; 0xf80];
        let f0p = obj_f0.as_mut_ptr() as usize;
        wr::<usize>(f0p + 0x08, f0ip);
        wr::<u32>(f0p + 0x10, 1);

        let mut obj_e0 = [0u8; 0x30];
        wr::<[u64; 2]>(obj_e0.as_mut_ptr() as usize + 0x10, [0x701, 0x702]);

        let mut main_obj = [0u8; 0x100];
        let mop = main_obj.as_mut_ptr() as usize;
        wr::<usize>(mop, inner_0.as_mut_ptr() as usize);
        wr::<usize>(mop + 0x68, obj_68.as_mut_ptr() as usize);
        wr::<u64>(mop + 0x78, 0x8888);
        wr::<usize>(mop + 0xe0, obj_e0.as_mut_ptr() as usize);
        wr::<usize>(mop + 0xf0, f0p);

        let mut p11_arr = [0u8; 0x20];
        wr::<usize>(p11_arr.as_mut_ptr() as usize + 0x08, mop);

        let mut p12_out = [0u8; 0x30];
        let p13_flags: [u8; 4] = [if mode == 2 { 1 } else { 0 }, 0, 0, 0];

        let mut pkt = [0u64; 17];
        pkt[0] = p0p as u64;
        pkt[1] = p1p as u64;
        pkt[2] = p2_arr.as_ptr() as u64;
        pkt[3] = p3_arr.as_mut_ptr() as u64;
        pkt[4] = p4_arr.as_mut_ptr() as u64;
        pkt[5] = p5_arr.as_mut_ptr() as u64;
        pkt[6] = p6_arr.as_mut_ptr() as u64;
        pkt[7] = 0;
        pkt[8] = p8p as u64;
        pkt[11] = p11_arr.as_mut_ptr() as u64;
        pkt[12] = p12_out.as_mut_ptr() as u64;
        pkt[13] = p13_flags.as_ptr() as u64;
        pkt[16] = 0;

        let h_entries: [u64; 2] = [0, (7u64 << 32) | 1u64];
        let h_u16s: [u64; 2] = [0, 25];
        let sorted_u16: [u16; 3] = [10, 25, 40];
        let sub_arr: [u64; 4] = [0, 0, 0x5555, 0];
        let mut p3_0 = [0u8; 0x40];
        let p3_0p = p3_0.as_mut_ptr() as usize;
        wr::<usize>(p3_0p + 0x18, h_entries.as_ptr() as usize);
        wr::<u32>(p3_0p + 0x20, 2);
        wr::<usize>(p3_0p + 0x30, h_u16s.as_ptr() as usize);

        let param3: [u64; 5] = [
            p3_0p as u64,
            sorted_u16.len() as u64,
            sorted_u16.as_ptr() as u64,
            0,
            sub_arr.as_ptr() as u64,
        ];

        f(
            pkt.as_mut_ptr() as usize,
            wp,
            param3.as_ptr() as usize,
            0x4444,
        );

        let mut plane_snap = [0u8; 0x50];
        plane_snap.copy_from_slice(&p8_plane[..0x50]);
        let mut mop_snap = [0u8; 0x20];
        mop_snap.copy_from_slice(&main_obj[0x10..0x30]);
        (
            unsafe { CALL_TRACE },
            p12_out,
            obj_68,
            plane_snap,
            mop_snap,
        )
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            wr::<u16>(g_f63c, s_f63c);
            wr::<u16>(g_d910, s_d910);
            wr::<u16>(g_d08c, s_d08c);
        }
        crate::diff_eq!(o, n, "1404b5940 mode={}", mode);
    }

    wr::<u16>(g_f63c, s_f63c);
    wr::<u16>(g_d910, s_d910);
    wr::<u16>(g_d08c, s_d08c);
    Ok(3)
}

pub unsafe fn diff_unknown_140f4c9d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_4C9D0: [u64; 12] = [0; 12];
    static mut C520_SLOT: [u32; 4] = [0; 4];
    static mut BFD0_BYTE: u8 = 0;

    unsafe extern "win64" fn mock_7c10(out: usize, w40: usize, span: usize) -> usize {
        wr::<usize>(out, w40);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_9ee0(_p: u64, _n: u32, _want: u16) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _a3: u64, _a4: u64) -> usize {
        let ptr = rd::<usize>(view);
        let len = rd::<u32>(view + 0x08);
        wr::<u32>(dst, 0x0400_0004);
        wr::<u32>(dst + 0x04, len);
        for i in 0..(len as usize).min(16) {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(ptr + i));
        }
        dst
    }
    unsafe extern "win64" fn mock_ab10(_dst: usize) {}
    unsafe extern "win64" fn mock_aaa0(_elem: usize, _p3: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_d380(out_vec: usize, _p1: usize, p2: u64) -> usize {
        wr::<u64>(out_vec, p2);
        wr::<u32>(out_vec + 0x08, 2);
        wr::<u32>(out_vec + 0x0c, 2);
        out_vec
    }
    unsafe extern "win64" fn mock_ddd0(
        _p1: usize,
        _p2: u64,
        span: usize,
        dx: i32,
        dy: i32,
    ) -> u32 {
        unsafe {
            TRACE_4C9D0[0] = rd::<u64>(span);
            TRACE_4C9D0[1] = (dx as i64 as u64) ^ ((dy as i64 as u64) << 16);
        }
        1
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        unsafe {
            TRACE_4C9D0[2] += 1;
        }
    }
    unsafe extern "win64" fn mock_c520(_rcx: usize) -> usize {
        core::ptr::addr_of_mut!(C520_SLOT) as usize
    }
    unsafe extern "win64" fn mock_6cb0(_rcx: usize, s_ptr: usize) {
        unsafe {
            TRACE_4C9D0[3] = rd::<u32>(s_ptr) as u64;
        }
    }
    unsafe extern "win64" fn mock_bfd0(_rcx: usize, _n: u32) -> usize {
        core::ptr::addr_of_mut!(BFD0_BYTE) as usize
    }
    unsafe extern "win64" fn mock_a880(fmt: usize, _arg: usize) -> usize {
        rd::<usize>(fmt)
    }
    unsafe extern "win64" fn mock_16a0(s: usize, len: u64) -> u64 {
        (rd::<u8>(s) as u64) | (len << 8)
    }
    unsafe extern "win64" fn mock_1a80(
        rcx: u64,
        pair: usize,
        _r8: u64,
        val_ptr: usize,
        a5: u32,
    ) {
        unsafe {
            TRACE_4C9D0[4] = rcx ^ rd::<u64>(pair + 0x08) ^ (rd::<u32>(val_ptr) as u64) ^ (a5 as u64);
        }
    }
    unsafe extern "win64" fn mock_acd0(
        rcx: u64,
        pair: usize,
        _r8: u64,
        flag_ptr: usize,
        a5: u32,
    ) {
        unsafe {
            TRACE_4C9D0[5] = rcx ^ rd::<u64>(pair + 0x08) ^ (rd::<u8>(flag_ptr) as u64) ^ (a5 as u64);
        }
    }
    unsafe extern "win64" fn mock_aa00(dst: usize, _src: usize) {
        wr::<u32>(dst + 0x04, 5);
    }
    unsafe extern "win64" fn mock_dc60(out_50: usize, span_30: usize) -> usize {
        let b = rd::<usize>(span_30);
        let e = rd::<usize>(span_30 + 0x08);
        unsafe {
            TRACE_4C9D0[6] = (e.wrapping_sub(b) as u64)
                ^ (rd::<u8>(b + 0x48) as u64)
                ^ (rd::<u8>(b + 0x98) as u64);
            TRACE_4C9D0[7] = rd::<u64>(b + 0x50);
        }
        wr::<u64>(out_50, 0xdc60_0001);
        out_50
    }
    unsafe extern "win64" fn mock_d570(rcx: u64, view: usize, r8: usize, r9: u64) {
        unsafe {
            TRACE_4C9D0[8] = rcx ^ rd::<u64>(view + 0x08) ^ rd::<u64>(r8) ^ r9;
        }
    }
    unsafe extern "win64" fn mock_4ee8(_ptr: usize, sz: u64, cnt: u64, _dtor: usize) {
        unsafe {
            TRACE_4C9D0[9] = (sz << 8) | cnt;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41df_aaa0, mock_aaa0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40f4_d380, mock_d380 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40f4_ddd0, mock_ddd0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4020_c520, mock_c520 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4022_6cb0, mock_6cb0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4020_bfd0, mock_bfd0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4021_a880, mock_a880 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_401f_1a80, mock_1a80 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_401e_acd0, mock_acd0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_40f4_dc60, mock_dc60 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_41e0_d570, mock_d570 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4183_4ee8, mock_4ee8 as *const ())?;

    let g_ce88 = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let g_ceec = exe::ptr::<u16>(0x1_43c8_ceec) as usize;
    let g_cecc = exe::ptr::<u16>(0x1_43c8_cecc) as usize;
    let g_ce6c = exe::ptr::<u16>(0x1_43c8_ce6c) as usize;
    let g_40f8 = exe::ptr::<u64>(0x1_43a3_40f8) as usize;
    let s_ce88 = rd::<u16>(g_ce88);
    let s_ceec = rd::<u16>(g_ceec);
    let s_cecc = rd::<u16>(g_cecc);
    let s_ce6c = rd::<u16>(g_ce6c);
    let s_40f8 = rd::<u64>(g_40f8);
    wr::<u16>(g_ce88, 0);
    wr::<u16>(g_ceec, 1);
    wr::<u16>(g_cecc, 7);
    wr::<u16>(g_ce6c, 2);
    wr::<u64>(g_40f8, 1);

    let run = |f: F, init_idx: u32| -> ([u64; 12], [u32; 4], u8, u32) {
        unsafe {
            TRACE_4C9D0 = [0; 12];
            C520_SLOT = [0; 4];
            BFD0_BYTE = 0;
        }
        let b0_arr: [u32; 4] = [10, 0, 25, 0];
        let mut ceec_obj = [0u8; 0xe0];
        let ceec_p = ceec_obj.as_mut_ptr() as usize;
        wr::<usize>(ceec_p + 0xb0, b0_arr.as_ptr() as usize);
        wr::<u32>(ceec_p + 0xc8, 33);
        wr::<u64>(ceec_p + 0xd0, 1);

        let tbl_580e0: [u64; 3] = [0x1111_0000, ceec_p as u64, 0x2222_0000];
        let mut w4 = vec![0u8; 0x58100];
        let w4p = w4.as_mut_ptr() as usize;
        wr::<usize>(w4p + 0x580e0, tbl_580e0.as_ptr() as usize);
        let p4 = [w4p];

        let mut str_entry = [0u8; 0x28];
        let sep = str_entry.as_mut_ptr() as usize;
        wr::<u32>(sep, 0x0400_0004);
        wr::<u32>(sep + 0x04, 4);
        wr::<[u8; 4]>(sep + 0x08, *b"Menu");

        let mut comp_block = vec![0u8; 0x200];
        let cbp = comp_block.as_mut_ptr() as usize;
        wr::<i32>(cbp + 0x10 + 0x11c, 0);
        wr::<usize>(cbp + 0x10 + 0x120, sep);
        wr::<u32>(cbp + 0x10 + 0x128, 1);

        let off_tbl: [u32; 1] = [0x10];
        let mut w3 = vec![0u8; 0x30000];
        let w3p = w3.as_mut_ptr() as usize;
        wr::<usize>(w3p + 0x88 + 8, cbp);
        wr::<u32>(w3p + 0x10088 + 4, 1);
        wr::<usize>(w3p + 0x40 + (1 + 0xc03) * 0x20, off_tbl.as_ptr() as usize);

        let archs: [u16; 1] = [1];
        let p3: [usize; 3] = [
            w3p,
            archs.as_ptr() as usize,
            archs.as_ptr().wrapping_add(1) as usize,
        ];

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p, 0x0400_0004);
        wr::<u32>(p1p + 0x04, 4);
        wr::<[u8; 4]>(p1p + 0x08, *b"Item");
        wr::<u32>(p1p + 0x28, 0x0400_0004);
        wr::<u32>(p1p + 0x2c, 4);
        wr::<[u8; 4]>(p1p + 0x30, *b"menu");
        wr::<u32>(p1p + 0x50, init_idx);
        wr::<u8>(p1p + 0x5d, 1);
        wr::<u8>(p1p + 0x5e, 0);
        wr::<u8>(p1p + 0x5f, 1);
        wr::<u8>(p1p + 0x60, 1);
        wr::<u8>(p1p + 0x61, 0);

        let vec_items: [u32; 2] = [100, 42];
        f(
            p1p,
            vec_items.as_ptr() as u64,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
        );

        (
            unsafe { TRACE_4C9D0 },
            unsafe { C520_SLOT },
            unsafe { BFD0_BYTE },
            rd::<u32>(p1p + 0x50),
        )
    };

    for idx in [0u32, 1] {
        let o = run(f_o, idx);
        let n = run(f_n, idx);
        if o != n {
            wr::<u16>(g_ce88, s_ce88);
            wr::<u16>(g_ceec, s_ceec);
            wr::<u16>(g_cecc, s_cecc);
            wr::<u16>(g_ce6c, s_ce6c);
            wr::<u64>(g_40f8, s_40f8);
        }
        crate::diff_eq!(o, n, "140f4c9d0 idx={}", idx);
    }

    wr::<u16>(g_ce88, s_ce88);
    wr::<u16>(g_ceec, s_ceec);
    wr::<u16>(g_cecc, s_cecc);
    wr::<u16>(g_ce6c, s_ce6c);
    wr::<u64>(g_40f8, s_40f8);
    Ok(2)
}

pub unsafe fn diff_unknown_140f55140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7c10(out: usize, w40: usize, span: usize) -> usize {
        wr::<usize>(out, w40);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_9ee0(_p: u64, _n: u32, _want: u16) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _a3: u64, _a4: u64) -> usize {
        let ptr = rd::<usize>(view);
        let len = rd::<u32>(view + 0x08);
        wr::<u32>(dst, 0x0400_0004);
        wr::<u32>(dst + 0x04, len);
        for i in 0..(len as usize).min(16) {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(ptr + i));
        }
        dst
    }
    unsafe extern "win64" fn mock_ab10(_dst: usize) {}
    unsafe extern "win64" fn mock_aaa0(_elem: usize, _p2: usize) -> u8 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41df_aaa0, mock_aaa0 as *const ())?;

    let g_0338 = exe::ptr::<u16>(0x1_43c9_0338) as usize;
    let g_cecc = exe::ptr::<u16>(0x1_43c8_cecc) as usize;
    let s_0338 = rd::<u16>(g_0338);
    let s_cecc = rd::<u16>(g_cecc);
    wr::<u16>(g_0338, 3);
    wr::<u16>(g_cecc, 5);

    let run = |f: F, active_flag: u8| -> u8 {
        let mut comp1 = vec![0u8; 0x100];
        let c1p = comp1.as_mut_ptr() as usize;
        wr::<u32>(c1p + 0x10, 0x0400_0004);
        wr::<u32>(c1p + 0x10 + 0x04, 4);
        wr::<[u8; 4]>(c1p + 0x10 + 0x08, *b"Test");
        wr::<u8>(c1p + 0x10 + 0x5c, active_flag);

        let mut str_entry = [0u8; 0x28];
        let sep = str_entry.as_mut_ptr() as usize;
        wr::<u32>(sep, 0x0400_0004);
        wr::<u32>(sep + 0x04, 4);
        wr::<[u8; 4]>(sep + 0x08, *b"tEsT");

        let mut comp2 = vec![0u8; 0x200];
        let c2p = comp2.as_mut_ptr() as usize;
        wr::<i32>(c2p + 0x20 + 0x11c, 0);
        wr::<usize>(c2p + 0x20 + 0x120, sep);
        wr::<u32>(c2p + 0x20 + 0x128, 1);

        let off1: [u32; 1] = [0x10];
        let off2: [u32; 1] = [0x20];

        let mut w1 = vec![0u8; 0x30000];
        let w1p = w1.as_mut_ptr() as usize;
        wr::<usize>(w1p + 0x88 + 8, c1p);
        wr::<u32>(w1p + 0x10088 + 4, 1);
        wr::<usize>(w1p + 0x40 + (1 + 0xc03) * 0x20, off1.as_ptr() as usize);

        let mut w2 = vec![0u8; 0x30000];
        let w2p = w2.as_mut_ptr() as usize;
        wr::<usize>(w2p + 0x88 + 8, c2p);
        wr::<u32>(w2p + 0x10088 + 4, 1);
        wr::<usize>(w2p + 0x40 + (1 + 0xc03) * 0x20, off2.as_ptr() as usize);

        let archs1: [u16; 1] = [1];
        let archs2: [u16; 1] = [1];
        let p1: [usize; 3] = [
            w1p,
            archs1.as_ptr() as usize,
            archs1.as_ptr().wrapping_add(1) as usize,
        ];
        let p2: [usize; 3] = [
            w2p,
            archs2.as_ptr() as usize,
            archs2.as_ptr().wrapping_add(1) as usize,
        ];

        let mut out_flag: u8 = 0;
        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            (&mut out_flag as *mut u8) as usize,
        );
        out_flag
    };

    for flag in [0u8, 1] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        if o != n {
            wr::<u16>(g_0338, s_0338);
            wr::<u16>(g_cecc, s_cecc);
        }
        crate::diff_eq!(o, n, "140f55140 flag={}", flag);
    }

    wr::<u16>(g_0338, s_0338);
    wr::<u16>(g_cecc, s_cecc);
    Ok(2)
}

pub unsafe fn diff_rend_1412eeb30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let keys: [i32; 6] = [40, -10, 25, 40, -50, 12];
    let run = |f: F, count: usize| -> (usize, Vec<u8>) {
        let mut buf = vec![0u8; 6 * 0xa0];
        let base = buf.as_mut_ptr() as usize;
        for i in 0..6 {
            let elem = base + i * 0xa0;
            for b in 0..0xa0 {
                wr::<u8>(elem + b, ((i * 37 + b * 13 + 7) & 0xff) as u8);
            }
            wr::<i32>(elem + 0x08, keys[i]);
        }
        let first: usize = base;
        let last: usize = base + count * 0xa0;
        let mut out_iter: usize = 0;
        let ret = f(
            (&mut out_iter as *mut usize) as usize,
            (&first as *const usize) as usize,
            (&last as *const usize) as usize,
            0,
        );
        let _ = ret;
        (out_iter.wrapping_sub(base), buf)
    };

    for count in [0usize, 1, 6] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "1412eeb30 count={}", count);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_404b_5940, "unknown_1404b5940", rend::cauldron_hot_p::unknown_1404b5940 as *const (), Throws::Caught, rend::cauldron_hot_p::diff_unknown_1404b5940),
// Replacement::verified_throws(0x1_40f4_c9d0, "unknown_140f4c9d0", rend::cauldron_hot_p::unknown_140f4c9d0 as *const (), Throws::Caught, rend::cauldron_hot_p::diff_unknown_140f4c9d0),
// Replacement::verified(0x1_40f5_5140, "unknown_140f55140", rend::cauldron_hot_p::unknown_140f55140 as *const (), rend::cauldron_hot_p::diff_unknown_140f55140),
// Replacement::verified(0x1_412e_eb30, "rend_1412eeb30", rend::cauldron_hot_p::rend_1412eeb30 as *const (), rend::cauldron_hot_p::diff_rend_1412eeb30),
