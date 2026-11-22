//! Decompiled hot functions shard ZA07A (`0x141468fb0..0x141962560`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn buf_push_byte(rbx: usize, ch: u8) {
    let pos = rd::<u64>(rbx + 0x10);
    if pos.wrapping_add(1) > rd::<u64>(rbx + 0x18) {
        let vfn = rd::<u64>(rd::<usize>(rbx));
        exe::call_va!(vfn, (rbx));
    }
    let cur = rd::<usize>(rbx + 0x10);
    let data = rd::<usize>(rbx + 0x08);
    wr::<u8>(data.wrapping_add(cur), ch);
    wr::<u64>(rbx + 0x10, rd::<u64>(rbx + 0x10).wrapping_add(1));
}

#[inline(always)]
unsafe fn pad_repeat(rbx: usize, specs: usize, mut count: u64) {
    if count == 0 {
        return;
    }
    let fill_len = rd::<u8>(specs + 0x0f) as usize;
    let fill_ptr = specs.wrapping_add(0x0b);
    if fill_len == 1 {
        while count != 0 {
            let pos = rd::<u64>(rbx + 0x10);
            if pos.wrapping_add(1) > rd::<u64>(rbx + 0x18) {
                let vfn = rd::<u64>(rd::<usize>(rbx));
                exe::call_va!(vfn, (rbx));
            }
            let cur = rd::<usize>(rbx + 0x10);
            let ch = rd::<u8>(fill_ptr);
            let data = rd::<usize>(rbx + 0x08);
            wr::<u8>(data.wrapping_add(cur), ch);
            wr::<u64>(rbx + 0x10, rd::<u64>(rbx + 0x10).wrapping_add(1));
            count -= 1;
        }
    } else {
        let fill_end = fill_ptr.wrapping_add(fill_len);
        while count != 0 {
            exe::call_va!(0x1_401f_9050, (rbx, fill_ptr, fill_end));
            count -= 1;
        }
    }
}

/// `curl_141468fb0`
/// Original: 0x141468fb0 (1089 bytes, Throws::Caught)
pub unsafe extern "win64" fn curl_141468fb0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
    param_6: u64,
) -> usize {
    let mut local_p5: u64 = param_5;
    let r13_sig = rd::<u64>(param_3);
    let mut local_sig: u64 = r13_sig;
    let mut local_num_zeros: i32;

    let bsr_idx = (63 - (r13_sig | 1).leading_zeros()) as usize;
    let r8d_b = rd::<u8>((exe::ptr::<u8>(0x1_4341_f470) as usize) + bsr_idx) as u32;
    let pow10 = rd::<u64>((exe::ptr::<u64>(0x1_4341_ef00) as usize) + (r8d_b as usize) * 8);
    let r9d_num_digits = r8d_b.wrapping_sub(u32::from(r13_sig < pow10));
    let mut local_num_digits: u32 = r9d_num_digits;

    let mut bytes_30 = [0u8; 8];
    bytes_30[2] = 0x30;
    let sign_b = (param_5 >> 40) as u8;
    bytes_30[3] = sign_b;
    bytes_30[0] = sign_b;
    let mut r14_size: u64 = (r9d_num_digits as u64).wrapping_add(u64::from(sign_b != 0));
    let dl_flags = (param_5 >> 48) as u8;

    let mut r10b_dec_pt: u8 = if (dl_flags & 2) != 0 {
        let pt = exe::call_va!(0x1_4146_af70, (param_6) -> u8);
        bytes_30[3] = bytes_30[0];
        pt
    } else {
        b'.'
    };
    bytes_30[1] = r10b_dec_pt;

    let r12d_exp = rd::<i32>(param_3 + 0x08);
    let r8d_output_exp = r12d_exp.wrapping_add(r9d_num_digits as i32);
    let r11d_exp10 = r8d_output_exp.wrapping_sub(1);
    let eax_prec = param_5 as i32;
    let cl_fmt = (param_5 >> 32) as u8;

    let use_fixed = if cl_fmt == 1 {
        false
    } else if cl_fmt != 0 {
        true
    } else {
        let limit = if eax_prec > 0 { eax_prec } else { 0x10 };
        r11d_exp10 >= -4 && r11d_exp10 < limit
    };

    if use_fixed {
        let mut local_output_exp: i32 = r8d_output_exp;
        if r12d_exp >= 0 {
            r14_size = r14_size.wrapping_add(r12d_exp as u32 as u64);
            let mut local_79: i32 = eax_prec.wrapping_sub(r8d_output_exp);
            if (dl_flags & 8) != 0 {
                r14_size = r14_size.wrapping_add(1);
                if local_79 <= 0 && cl_fmt != 2 {
                    local_79 = 1;
                }
                if local_79 > 0 {
                    r14_size = r14_size.wrapping_add(local_79 as u32 as u64);
                }
            }
            let mut grouping = [0u64; 8];
            let gp = grouping.as_mut_ptr() as usize;
            exe::call_va!(
                0x1_4146_ab30,
                (gp, param_6, ((dl_flags >> 1) & 1) as u32)
            );
            let sep_cnt = exe::call_va!(
                0x1_4146_6360,
                (gp, local_output_exp as u32) -> u32
            );
            r14_size = r14_size.wrapping_add(sep_cnt as u64);

            let mut lambda_cap: [usize; 9] = [
                bytes_30.as_mut_ptr() as usize,
                (&mut local_sig as *mut u64) as usize,
                (&mut local_num_digits as *mut u32) as usize,
                param_3,
                gp,
                (&mut local_p5 as *mut u64) as usize,
                bytes_30.as_mut_ptr().add(1) as usize,
                (&mut local_79 as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(2) as usize,
            ];
            exe::call_va!(
                0x1_4146_ce20,
                (
                    param_1,
                    param_2,
                    param_4,
                    r9d_num_digits as u64,
                    r14_size,
                    lambda_cap.as_mut_ptr() as usize
                )
            );
            exe::call_va!(0x1_4020_3690, (gp + 0x20));
            exe::call_va!(0x1_4020_3690, (gp));
            return param_1;
        }

        if r8d_output_exp > 0 {
            let diff = eax_prec.wrapping_sub(r9d_num_digits as i32);
            let mut local_67: i32 = if (dl_flags & 8) != 0 { diff } else { 0 };
            let pos_zeros = if local_67 > 0 { local_67 as u32 } else { 0 };
            r14_size = r14_size.wrapping_add((pos_zeros.wrapping_add(1)) as u64);

            let mut grouping = [0u64; 8];
            let gp = grouping.as_mut_ptr() as usize;
            exe::call_va!(
                0x1_4146_ab30,
                (gp, param_6, ((dl_flags >> 1) & 1) as u32)
            );
            let sep_cnt = exe::call_va!(
                0x1_4146_6360,
                (gp, local_num_digits) -> u32
            );
            r14_size = r14_size.wrapping_add(sep_cnt as u64);

            let mut lambda_cap: [usize; 8] = [
                bytes_30.as_mut_ptr() as usize,
                (&mut local_sig as *mut u64) as usize,
                (&mut local_num_digits as *mut u32) as usize,
                (&mut local_output_exp as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(1) as usize,
                gp,
                (&mut local_67 as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(2) as usize,
            ];
            exe::call_va!(
                0x1_4146_cf80,
                (
                    param_1,
                    param_2,
                    param_4,
                    r9d_num_digits as u64,
                    r14_size,
                    lambda_cap.as_mut_ptr() as usize
                )
            );
            exe::call_va!(0x1_4020_3690, (gp + 0x20));
            exe::call_va!(0x1_4020_3690, (gp));
            return param_1;
        }

        let mut r8d_neg = r8d_output_exp.wrapping_neg();
        local_num_zeros = r8d_neg;
        if r9d_num_digits == 0 && eax_prec >= 0 && eax_prec < r8d_neg {
            r8d_neg = eax_prec;
            local_num_zeros = eax_prec;
        }
        let has_pt = u8::from(!(r8d_neg == 0 && r9d_num_digits == 0 && (dl_flags & 8) == 0));
        bytes_30[4] = has_pt;

        let mut lambda_cap: [usize; 7] = [
            bytes_30.as_mut_ptr() as usize,
            bytes_30.as_mut_ptr().add(2) as usize,
            bytes_30.as_mut_ptr().add(4) as usize,
            bytes_30.as_mut_ptr().add(1) as usize,
            (&mut local_num_zeros as *mut i32) as usize,
            (&mut local_sig as *mut u64) as usize,
            (&mut local_num_digits as *mut u32) as usize,
        ];
        let add_len = (has_pt as u32)
            .wrapping_add((r8d_neg as u32).wrapping_add(1)) as u64;
        let total_size = r14_size.wrapping_add(add_len);
        exe::call_va!(
            0x1_4146_d0e0,
            (
                param_1,
                param_2,
                param_4,
                r9d_num_digits as u64,
                total_size,
                lambda_cap.as_mut_ptr() as usize
            )
        );
        return param_1;
    }

    let mut r8d_zeros: u32 = 0;
    if (dl_flags & 8) != 0 {
        let diff = eax_prec.wrapping_sub(r9d_num_digits as i32);
        if diff >= 0 {
            r8d_zeros = diff as u32;
        }
        r14_size = r14_size.wrapping_add(r8d_zeros as u64);
    } else if r9d_num_digits == 1 {
        r10b_dec_pt = 0;
        bytes_30[1] = 0;
    }

    let abs_exp = if r11d_exp10 < 0 {
        r11d_exp10.wrapping_neg()
    } else {
        r11d_exp10
    };
    let exp_digits: u64 = if abs_exp < 100 {
        2
    } else {
        3 + u64::from(abs_exp >= 1000)
    };

    let mut exp_lambda = [0u8; 40];
    let elp = exp_lambda.as_mut_ptr() as usize;
    wr::<u8>(elp, bytes_30[3]);
    wr::<u64>(elp + 0x08, r13_sig);
    wr::<u32>(elp + 0x10, r9d_num_digits);
    wr::<u8>(elp + 0x14, r10b_dec_pt);
    wr::<u32>(elp + 0x18, r8d_zeros);
    wr::<u8>(elp + 0x1c, bytes_30[2]);
    wr::<u8>(elp + 0x1d, (((!dl_flags) & 1) << 5) | 0x45);
    wr::<i32>(elp + 0x20, r11d_exp10);

    if rd::<i32>(param_4) > 0 {
        let total_sz = u64::from(r10b_dec_pt != 0)
            .wrapping_add(exp_digits)
            .wrapping_add(2)
            .wrapping_add(r14_size);
        let mut out_res: u64 = 0;
        exe::call_va!(
            0x1_4146_ccc0,
            (
                (&mut out_res as *mut u64) as usize,
                param_2,
                param_4,
                r9d_num_digits as u64,
                total_sz,
                elp
            )
        );
        wr::<u64>(param_1, out_res);
    } else {
        let mut dummy: [u64; 2] = [0; 2];
        let ret_ptr = exe::call_va!(
            0x1_4146_9400,
            (elp, dummy.as_mut_ptr() as usize, param_2) -> usize
        );
        wr::<u64>(param_1, rd::<u64>(ret_ptr));
    }
    param_1
}

/// `unknown_14146ac70`
/// Original: 0x14146ac70 (349 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14146ac70(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: usize,
    param_5: u64,
    param_6: usize,
) -> usize {
    let align_idx = (rd::<u8>(param_3 + 0x09) & 0xf) as usize;
    let width = rd::<u32>(param_3) as u64;
    let pad_total = if width <= param_5 {
        0u64
    } else {
        width.wrapping_sub(param_5)
    };
    let shift =
        (rd::<i8>((exe::ptr::<i8>(0x1_4307_e134) as usize) + align_idx) as i32 as u32) & 0x3f;
    let left_pad = pad_total >> shift;
    let right_pad = pad_total.wrapping_sub(left_pad);

    pad_repeat(param_2, param_3, left_pad);

    let s_ptr = rd::<usize>(param_6);
    let s_len = rd::<usize>(param_6 + 0x08);
    exe::call_va!(0x1_401f_9050, (param_2, s_ptr, s_ptr.wrapping_add(s_len)));

    pad_repeat(param_2, param_3, right_pad);

    wr::<usize>(param_1, param_2);
    param_1
}

/// `unknown_14146add0`
/// Original: 0x14146add0 (412 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14146add0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: usize,
    param_5: u64,
    param_6: usize,
) -> usize {
    let align_idx = (rd::<u8>(param_3 + 0x09) & 0xf) as usize;
    let width = rd::<u32>(param_3) as u64;
    let pad_total = if width <= param_5 {
        0u64
    } else {
        width.wrapping_sub(param_5)
    };
    let shift =
        (rd::<i8>((exe::ptr::<i8>(0x1_4307_e134) as usize) + align_idx) as i32 as u32) & 0x3f;
    let left_pad = pad_total >> shift;
    let right_pad = pad_total.wrapping_sub(left_pad);

    pad_repeat(param_2, param_3, left_pad);

    let sign_idx = rd::<u8>(param_6) as usize;
    if sign_idx != 0 {
        let sign_ch = rd::<u8>((exe::ptr::<u8>(0x1_4342_001c) as usize) + sign_idx);
        buf_push_byte(param_2, sign_ch);
    }

    let s_ptr = rd::<usize>(param_6 + 0x08);
    exe::call_va!(0x1_401f_9050, (param_2, s_ptr, s_ptr.wrapping_add(3)));

    pad_repeat(param_2, param_3, right_pad);

    wr::<usize>(param_1, param_2);
    param_1
}

/// `unknown_14146b420`
/// Original: 0x14146b420 (444 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14146b420(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
    param_6: usize,
) -> usize {
    let align_idx = (rd::<u8>(param_3 + 0x09) & 0xf) as usize;
    let width = rd::<u32>(param_3) as u64;
    let pad_total = if width <= param_5 {
        0u64
    } else {
        width.wrapping_sub(param_5)
    };
    let shift =
        (rd::<i8>((exe::ptr::<i8>(0x1_4307_e0a8) as usize) + align_idx) as i32 as u32) & 0x3f;
    let left_pad = pad_total >> shift;
    let right_pad = pad_total.wrapping_sub(left_pad);

    let mut local_res20: usize = param_4;
    pad_repeat(param_2, param_3, left_pad);

    buf_push_byte(param_2, b'0');
    buf_push_byte(param_2, b'x');

    let digits = rd::<u32>(param_6 + 0x08);
    let uptr = rd::<u64>(param_6);
    exe::call_va!(
        0x1_4146_9ab0,
        (
            (&mut local_res20 as *mut usize) as usize,
            param_2,
            uptr,
            digits,
            0u8
        )
    );
    let rbx = local_res20;

    pad_repeat(rbx, param_3, right_pad);

    wr::<usize>(param_1, rbx);
    param_1
}

/// `unknown_141484ec0`
/// Original: 0x141484ec0 (154 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141484ec0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u8,
) -> usize {
    let rbx = rd::<usize>(param_2 + 0x10);
    if rbx == 0 {
        return exe::call_va!(0x1_4253_7280, (param_1, param_3, param_4, 0u8) -> usize);
    }
    wr::<u32>(param_2 + 0x18, rd::<u32>(param_2 + 0x18).wrapping_add(1));
    let next = rd::<u64>(rbx);
    wr::<u64>(param_2 + 0x10, next);
    if param_4 != 0 {
        if (rd::<u8>(param_2 + 0x0f) & 1) != 0 {
            wr::<u64>(rbx, 0);
            return rbx;
        }
        let flag_580 = rd::<u8>(exe::ptr::<u8>(0x1_43a0_9580) as usize);
        let r8_size = rd::<u32>(param_2 + 0x1c) as usize;
        if flag_580 != 0 {
            for i in 0..r8_size {
                wr::<u8>(rbx.wrapping_add(i), 0);
            }
            return rbx;
        }
        exe::call_va!(0x1_4183_acc2, (rbx, 0u32, r8_size as u64));
    }
    rbx
}

/// `bigfish_141962000`
/// Original: 0x141962000 (455 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_141962000(param_1: usize) {
    let eax = rd::<i32>(param_1 + 0x18);
    wr::<u32>(param_1 + 0x2a8, 0);
    let mut local_id: u64 = 0;
    let r14: usize = if eax != -1 {
        local_id = (local_id & !0xffff_ffffu64) | (eax as u32 as u64);
        let res = exe::call_va!(
            0x1_4141_0810,
            (param_1, (&mut local_id as *mut u64) as usize) -> usize
        );
        rd::<usize>(rd::<usize>(res) + 0x28)
    } else {
        0
    };

    exe::call_va!(0x1_41fc_8b40, (param_1));
    exe::call_va!(0x1_41df_7140, (param_1.wrapping_add(0x268), r14));
    exe::call_va!(0x1_41df_5490, (param_1.wrapping_add(0x280), r14));

    let mut rbx = rd::<usize>(param_1 + 0x240);
    let cnt_248 = rd::<u32>(param_1 + 0x248) as usize;
    let rdi_end = rbx.wrapping_add(cnt_248 * 8);
    while rbx != rdi_end {
        let elem = rd::<usize>(rbx);
        let vfn = rd::<usize>(rd::<usize>(elem) + 0x08);
        exe::call_va!(vfn, (elem, r14));
        rbx = rbx.wrapping_add(8);
    }

    let mut rbx = rd::<usize>(param_1 + 0x250);
    let cnt_258 = rd::<u32>(param_1 + 0x258) as usize;
    let p_110 = rd::<usize>(param_1 + 0x110);
    let rsi_end = rbx.wrapping_add(cnt_258 * 8);
    let rdi_world = rd::<usize>(p_110);
    while rbx != rsi_end {
        let elem = rd::<usize>(rbx);
        let vfn = rd::<usize>(rd::<usize>(elem) + 0x08);
        exe::call_va!(
            vfn,
            (
                elem,
                r14,
                param_1,
                param_1.wrapping_add(0x280),
                rdi_world
            )
        );
        rbx = rbx.wrapping_add(8);
    }

    let arr_580e0 = rd::<usize>(rdi_world + 0x5_80e0);
    let idx_ce84 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce84) as usize) as usize;
    local_id = 0;
    let rdx_elem = rd::<usize>(arr_580e0.wrapping_add(idx_ce84 * 8));
    wr::<u64>(rdx_elem + 0x2c, u64::MAX);
    wr::<u16>(rdx_elem, 0x0100);
    wr::<u64>(param_1 + 0x29c, 0);
    let eax2 = rd::<i32>(param_1 + 0x18);
    wr::<u32>(param_1 + 0x2a4, 0);
    wr::<u8>(param_1 + 0x298, 0);

    let mut r15: usize = 0;
    if eax2 != -1 {
        local_id = (local_id & !0xffff_ffffu64) | (eax2 as u32 as u64);
        let res = exe::call_va!(
            0x1_4141_0810,
            (idx_ce84, (&mut local_id as *mut u64) as usize) -> usize
        );
        r15 = rd::<usize>(rd::<usize>(res) + 0x28);
    }

    let vfn_218 = rd::<usize>(rd::<usize>(r15) + 0x218);
    let rbx_view = exe::call_va!(vfn_218, (r15) -> usize);
    if rbx_view != 0 {
        let vfn_60 = rd::<usize>(rd::<usize>(rbx_view) + 0x60);
        exe::call_va!(vfn_60, (rbx_view));
        let vfn_28 = rd::<usize>(rd::<usize>(rbx_view) + 0x28);
        exe::call_va!(vfn_28, (rbx_view));
        if param_1 != 0 {
            let vfn_28_b = rd::<usize>(rd::<usize>(rbx_view) + 0x28);
            exe::call_va!(vfn_28_b, (rbx_view));
            exe::call_va!(0x1_4196_2560, (rbx_view, param_1));
            let vfn_48 = rd::<usize>(rd::<usize>(rbx_view) + 0x48);
            exe::call_va!(vfn_48, (rbx_view, param_1));
        } else {
            exe::call_va!(0x1_4196_2560, (rbx_view, 0usize));
        }
        let vfn_48_2 = rd::<usize>(rd::<usize>(rbx_view) + 0x48);
        exe::call_va!(vfn_48_2, (rbx_view, param_1));
        let vfn_68 = rd::<usize>(rd::<usize>(rbx_view) + 0x68);
        exe::call_va!(
            vfn_68,
            (
                rbx_view,
                exe::ptr::<u8>(0x1_4310_6600) as usize,
                param_1
            )
        );
    }
}

/// `cohtml_141962560`
/// Original: 0x141962560 (184 bytes, Throws::Caught)
pub unsafe extern "win64" fn cohtml_141962560(param_1: usize, param_2: usize) {
    let mut td = [0u64; 2];
    let td_ptr = td.as_mut_ptr() as usize;
    let vfn_10 = rd::<usize>(rd::<usize>(param_1) + 0x10);
    exe::call_va!(
        vfn_10,
        (
            param_1,
            td_ptr,
            exe::ptr::<u8>(0x1_4310_6600) as usize,
            param_2
        )
    );

    let rbx = rd::<usize>(td_ptr);
    if rbx != 0 {
        let rdi_vfn = rd::<usize>(rd::<usize>(rbx) + 0x20);
        let fn_141960da0 = exe::ptr::<u8>(0x1_4196_0da0) as usize as u64;
        let td_hi = rd::<u32>(td_ptr + 0x0c);
        let rsi = rd::<u64>(td_ptr + 0x08);
        let handler = exe::call_iat!(0x1_4264_6cc0, (0x20u32) -> usize);
        if handler != 0 {
            wr::<u64>(handler + 0x08, 0);
            wr::<usize>(handler, exe::ptr::<u8>(0x1_4310_6688) as usize);
            wr::<u64>(handler + 0x10, fn_141960da0);
            wr::<u32>(handler + 0x18, 0);
            wr::<u32>(handler + 0x1c, td_hi);
        }
        wr::<u64>(handler + 0x08, rsi);
        exe::call_va!(
            rdi_vfn,
            (rbx, exe::ptr::<u8>(0x1_4310_6610) as usize, handler)
        );
    }

    exe::call_iat!(0x1_4264_6d10, (td_ptr));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_curl_141468fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, sig: u64, exp: i32, width: u32, p5: u64| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut dec_fp: [u64; 2] = [sig, exp as u32 as u64];
        let mut specs: [u32; 4] = [width, 0xffff_ffff, 0x2000_0000, 0x0100_0000];
        let mut out_ptr: usize = 0;

        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            bp,
            dec_fp.as_mut_ptr() as usize,
            specs.as_mut_ptr() as usize,
            p5,
            0,
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    let cases: [(u64, i32, u32, u64); 6] = [
        (3141592653589793, -15, 0, 0xffff_ffffu64),
        (125, 2, 0, 0xffff_ffffu64 | (1u64 << 40)),
        (42, -5, 0, 0xffff_ffffu64 | (2u64 << 32)),
        (5, 0, 0, 0xffff_ffffu64 | (1u64 << 32)),
        (12345678, -12, 16, 0xffff_ffffu64 | (1u64 << 32) | (1u64 << 48)),
        (99, 1, 8, 4u64 | (2u64 << 32) | (8u64 << 48)),
    ];

    for (i, &(sig, exp, width, p5)) in cases.iter().enumerate() {
        let o = run(f_o, sig, exp, width, p5);
        let n = run(f_n, sig, exp, width, p5);
        crate::diff_eq!(o, n, "141468fb0 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_14146ac70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, width: u32, align: u8, fill: &[u8], text: &[u8]| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut specs = [0u8; 16];
        let sp = specs.as_mut_ptr() as usize;
        wr::<u32>(sp, width);
        wr::<u8>(sp + 0x09, align);
        for (i, &b) in fill.iter().enumerate() {
            wr::<u8>(sp + 0x0b + i, b);
        }
        wr::<u8>(sp + 0x0f, fill.len() as u8);

        let mut sv: [u64; 2] = [text.as_ptr() as usize as u64, text.len() as u64];
        let mut out_ptr: usize = 0;
        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            bp,
            sp,
            0,
            text.len() as u64,
            sv.as_mut_ptr() as usize,
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    let cases: [(u32, u8, &[u8], &[u8]); 5] = [
        (0, 0, b" ", b"hello"),
        (10, 1, b"*", b"abc"),
        (10, 2, b".", b"abc"),
        (10, 3, b"_", b"abc"),
        (9, 2, b"xy", b"test"),
    ];

    for (i, &(w, align, fill, text)) in cases.iter().enumerate() {
        let o = run(f_o, w, align, fill, text);
        let n = run(f_n, w, align, fill, text);
        crate::diff_eq!(o, n, "14146ac70 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_14146add0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, width: u32, align: u8, fill: &[u8], sign: u8, tag: &[u8; 3]| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut specs = [0u8; 16];
        let sp = specs.as_mut_ptr() as usize;
        wr::<u32>(sp, width);
        wr::<u8>(sp + 0x09, align);
        for (i, &b) in fill.iter().enumerate() {
            wr::<u8>(sp + 0x0b + i, b);
        }
        wr::<u8>(sp + 0x0f, fill.len() as u8);

        let mut nan_inf: [u64; 2] = [sign as u64, tag.as_ptr() as usize as u64];
        let content_len = 3u64 + u64::from(sign != 0);
        let mut out_ptr: usize = 0;
        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            bp,
            sp,
            0,
            content_len,
            nan_inf.as_mut_ptr() as usize,
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    let cases: [(u32, u8, &[u8], u8, &[u8; 3]); 4] = [
        (0, 0, b" ", 0, b"inf"),
        (8, 1, b" ", 1, b"nan"),
        (9, 2, b".", 2, b"inf"),
        (8, 3, b"ab", 1, b"nan"),
    ];

    for (i, &(w, align, fill, sign, tag)) in cases.iter().enumerate() {
        let o = run(f_o, w, align, fill, sign, tag);
        let n = run(f_n, w, align, fill, sign, tag);
        crate::diff_eq!(o, n, "14146add0 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_14146b420(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, width: u32, align: u8, fill: &[u8], uptr: u64, digits: u32| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut specs = [0u8; 16];
        let sp = specs.as_mut_ptr() as usize;
        wr::<u32>(sp, width);
        wr::<u8>(sp + 0x09, align);
        for (i, &b) in fill.iter().enumerate() {
            wr::<u8>(sp + 0x0b + i, b);
        }
        wr::<u8>(sp + 0x0f, fill.len() as u8);

        let mut ptr_data: [u64; 2] = [uptr, digits as u64];
        let mut out_ptr: usize = 0;
        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            bp,
            sp,
            0,
            (digits as u64) + 2,
            ptr_data.as_mut_ptr() as usize,
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    let cases: [(u32, u8, &[u8], u64, u32); 4] = [
        (0, 0, b" ", 0, 1),
        (14, 1, b" ", 0xdead_beef, 8),
        (10, 2, b"0", 0x1234, 4),
        (9, 3, b"xy", 0xff, 2),
    ];

    for (i, &(w, align, fill, uptr, digits)) in cases.iter().enumerate() {
        let o = run(f_o, w, align, fill, uptr, digits);
        let n = run(f_n, w, align, fill, uptr, digits);
        crate::diff_eq!(o, n, "14146b420 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_141484ec0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BLOCK_BUF: [u8; 32] = [0; 32];
    static mut FALLBACK_ARGS: (usize, u64, u8, u8) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_fallback(p1: usize, sz: u64, z: u8, a4: u8) -> usize {
        FALLBACK_ARGS = (p1, sz, z, a4);
        0x7777_8888
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_7280, mock_fallback as *const ())?;
    let g_580 = exe::ptr::<u8>(0x1_43a0_9580) as usize;
    let saved_580 = rd::<u8>(g_580);

    let run = |f: F, has_free: bool, zflag: u8, f_0f: u8, f_580: u8| -> (usize, u64, u32, [u8; 32], (usize, u64, u8, u8)) {
        FALLBACK_ARGS = (0, 0, 0, 0);
        BLOCK_BUF = [0xaa; 32];
        let blk_ptr = core::ptr::addr_of_mut!(BLOCK_BUF) as usize;
        wr::<u64>(blk_ptr, 0x1234_5678_9abc_def0);
        wr::<u8>(g_580, f_580);

        let mut page = [0u8; 0x28];
        let pp = page.as_mut_ptr() as usize;
        wr::<u8>(pp + 0x0f, f_0f);
        wr::<usize>(pp + 0x10, if has_free { blk_ptr } else { 0 });
        wr::<u32>(pp + 0x18, 4);
        wr::<u32>(pp + 0x1c, 24);

        let ret = f(0x1111, pp, 24, zflag);
        let norm_ret = if ret == blk_ptr { 1 } else { ret };
        (
            norm_ret,
            rd::<u64>(pp + 0x10),
            rd::<u32>(pp + 0x18),
            BLOCK_BUF,
            FALLBACK_ARGS,
        )
    };

    let cases: [(bool, u8, u8, u8); 5] = [
        (false, 1, 0, 0),
        (true, 0, 0, 0),
        (true, 1, 1, 0),
        (true, 1, 0, 0),
        (true, 1, 0, 1),
    ];

    for (i, &(has_free, zflag, f_0f, f_580)) in cases.iter().enumerate() {
        let o = run(f_o, has_free, zflag, f_0f, f_580);
        let n = run(f_n, has_free, zflag, f_0f, f_580);
        wr::<u8>(g_580, saved_580);
        crate::diff_eq!(o, n, "141484ec0 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_bigfish_141962000(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HOLDER_PTR: usize = 0;
    static mut VIEW_PTR: usize = 0;
    static mut TRACE: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_0810(_p1: usize, id_ptr: usize) -> usize {
        TRACE.push(0x0810_0000 | (rd::<u32>(id_ptr) as u64));
        HOLDER_PTR
    }
    unsafe extern "win64" fn mock_8b40(_p1: usize) {
        TRACE.push(0x8b40);
    }
    unsafe extern "win64" fn mock_7140(_p268: usize, r14: usize) {
        TRACE.push(0x7140 ^ (r14 as u64));
    }
    unsafe extern "win64" fn mock_5490(_p280: usize, r14: usize) {
        TRACE.push(0x5490 ^ (r14 as u64));
    }
    unsafe extern "win64" fn mock_2560(_view: usize, p1: usize) {
        TRACE.push(0x2560_0000 | u64::from(p1 != 0));
    }
    unsafe extern "win64" fn mock_elem240_v8(_this: usize, _r14: usize) {
        TRACE.push(0x2408);
    }
    unsafe extern "win64" fn mock_elem250_v8(
        _this: usize,
        _r14: usize,
        p1: usize,
        p280: usize,
        _world: usize,
    ) {
        TRACE.push(0x2508_0000 | ((p280.wrapping_sub(p1)) as u64));
    }
    unsafe extern "win64" fn mock_r15_v218(_this: usize) -> usize {
        TRACE.push(0x0218);
        VIEW_PTR
    }
    unsafe extern "win64" fn mock_view_v60(_this: usize) {
        TRACE.push(0x0060);
    }
    unsafe extern "win64" fn mock_view_v28(_this: usize) {
        TRACE.push(0x0028);
    }
    unsafe extern "win64" fn mock_view_v48(_this: usize, p1: usize) {
        TRACE.push(0x0048_0000 | u64::from(p1 != 0));
    }
    unsafe extern "win64" fn mock_view_v68(_this: usize, name: usize, p1: usize) {
        TRACE.push(0x0068_0000 | (rd::<u8>(name) as u64) | (u64::from(p1 != 0) << 8));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fc_8b40, mock_8b40 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41df_7140, mock_7140 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41df_5490, mock_5490 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4196_2560, mock_2560 as *const ())?;

    let mut r15_vtbl = vec![0u8; 0x228];
    wr::<usize>(
        r15_vtbl.as_mut_ptr() as usize + 0x218,
        mock_r15_v218 as *const () as usize,
    );
    let mut r15_obj: usize = r15_vtbl.as_mut_ptr() as usize;
    let mut inner = [0u8; 0x30];
    wr::<usize>(
        inner.as_mut_ptr() as usize + 0x28,
        (&mut r15_obj as *mut usize) as usize,
    );
    let mut holder: usize = inner.as_mut_ptr() as usize;
    HOLDER_PTR = (&mut holder as *mut usize) as usize;

    let mut view_vtbl = [0u8; 0x78];
    let vvp = view_vtbl.as_mut_ptr() as usize;
    wr::<usize>(vvp + 0x28, mock_view_v28 as *const () as usize);
    wr::<usize>(vvp + 0x48, mock_view_v48 as *const () as usize);
    wr::<usize>(vvp + 0x60, mock_view_v60 as *const () as usize);
    wr::<usize>(vvp + 0x68, mock_view_v68 as *const () as usize);
    let mut view_obj: usize = vvp;

    let mut e240_vtbl = [0usize; 2];
    e240_vtbl[1] = mock_elem240_v8 as *const () as usize;
    let mut e240_obj: usize = e240_vtbl.as_mut_ptr() as usize;
    let mut arr_240: [usize; 1] = [(&mut e240_obj as *mut usize) as usize];

    let mut e250_vtbl = [0usize; 2];
    e250_vtbl[1] = mock_elem250_v8 as *const () as usize;
    let mut e250_obj: usize = e250_vtbl.as_mut_ptr() as usize;
    let mut arr_250: [usize; 1] = [(&mut e250_obj as *mut usize) as usize];

    let mut run = |f: F, has_view: bool| -> (u16, u64, u8, u64, u32, u32, Vec<u64>) {
        TRACE.clear();
        VIEW_PTR = if has_view {
            (&mut view_obj as *mut usize) as usize
        } else {
            0
        };

        let mut ce84_elem = [0u8; 0x40];
        let cep = ce84_elem.as_mut_ptr() as usize;
        let idx_ce84 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce84) as usize) as usize;
        let mut arr_580e0 = vec![0usize; idx_ce84 + 4];
        arr_580e0[idx_ce84] = cep;

        let mut world = vec![0u8; 0x5_8100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x5_80e0, arr_580e0.as_mut_ptr() as usize);
        let mut world_ptr: usize = wp;

        let mut page = [0u8; 0x2b0];
        let pp = page.as_mut_ptr() as usize;
        wr::<i32>(pp + 0x18, 7);
        wr::<usize>(pp + 0x110, (&mut world_ptr as *mut usize) as usize);
        wr::<usize>(pp + 0x240, arr_240.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x248, 1);
        wr::<usize>(pp + 0x250, arr_250.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x258, 1);
        wr::<u8>(pp + 0x298, 9);
        wr::<u64>(pp + 0x29c, 0x1111_2222_3333_4444);
        wr::<u32>(pp + 0x2a4, 0x5555_6666);
        wr::<u32>(pp + 0x2a8, 0x7777_8888);

        f(pp);
        (
            rd::<u16>(cep),
            rd::<u64>(cep + 0x2c),
            rd::<u8>(pp + 0x298),
            rd::<u64>(pp + 0x29c),
            rd::<u32>(pp + 0x2a4),
            rd::<u32>(pp + 0x2a8),
            TRACE.clone(),
        )
    };

    for has_view in [true, false] {
        let o = run(f_o, has_view);
        let n = run(f_n, has_view);
        crate::diff_eq!(o, n, "141962000 has_view={}", has_view);
    }
    Ok(2)
}

pub unsafe fn diff_cohtml_141962560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BINDER_OBJ: usize = 0;
    static mut HANDLER_BUF: [u64; 4] = [0; 4];
    static mut TRACE: (u8, usize, u8, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_v10(_this: usize, td_out: usize, name: usize, p2: usize) {
        TRACE.0 = rd::<u8>(name);
        TRACE.1 = p2;
        wr::<usize>(td_out, BINDER_OBJ);
        wr::<u64>(td_out + 0x08, (p2 as u64) | (0xabcd_0000_0000_0000u64));
    }
    unsafe extern "win64" fn mock_v20(_this: usize, method_name: usize, _handler: usize) {
        TRACE.2 = rd::<u8>(method_name);
    }
    unsafe extern "win64" fn mock_op_new(sz: u32) -> usize {
        TRACE.3 = sz;
        core::ptr::addr_of_mut!(HANDLER_BUF) as usize
    }
    unsafe extern "win64" fn mock_td_dtor(_td: usize) {
        TRACE.3 += 100;
    }

    let _m1 = crate::difftest::IatMock::new(0x1_4264_6cc0, mock_op_new as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_6d10, mock_td_dtor as *const ())?;

    let mut binder_vtbl = [0usize; 6];
    binder_vtbl[4] = mock_v20 as *const () as usize;
    let mut binder_inst: usize = binder_vtbl.as_mut_ptr() as usize;

    let mut p1_vtbl = [0usize; 4];
    p1_vtbl[2] = mock_v10 as *const () as usize;
    let mut p1_inst: usize = p1_vtbl.as_mut_ptr() as usize;

    let mut run = |f: F, non_null_binder: bool| -> ([u64; 4], (u8, usize, u8, u32)) {
        HANDLER_BUF = [0; 4];
        TRACE = (0, 0, 0, 0);
        BINDER_OBJ = if non_null_binder {
            (&mut binder_inst as *mut usize) as usize
        } else {
            0
        };
        f((&mut p1_inst as *mut usize) as usize, 0x4321);
        (HANDLER_BUF, TRACE)
    };

    for non_null in [true, false] {
        let o = run(f_o, non_null);
        let n = run(f_n, non_null);
        crate::diff_eq!(o, n, "141962560 non_null={}", non_null);
    }
    Ok(2)
}
