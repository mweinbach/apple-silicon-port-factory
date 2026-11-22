//! Decompiled hot functions shard ZA06 (`0x141462630..0x1414684d0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(never)]
unsafe fn throw_format_error(msg_va: u64) -> ! {
    let mut src_data: [u64; 2] = [exe::ptr::<u8>(msg_va) as usize as u64, 1];
    let mut tmp_exc: [u64; 3] = [exe::ptr::<u8>(0x1_4266_80a0) as usize as u64, 0, 0];
    exe::call_iat!(
        0x1_4264_6338,
        (
            src_data.as_mut_ptr() as usize,
            (tmp_exc.as_mut_ptr() as usize) + 0x08
        )
    );
    tmp_exc[0] = exe::ptr::<u8>(0x1_4341_f118) as usize as u64;
    let mut thrown_exc = [0u64; 4];
    exe::call_va!(
        0x1_4146_2ac0,
        (thrown_exc.as_mut_ptr() as usize, tmp_exc.as_mut_ptr() as usize)
    );
    exe::call_va!(
        0x1_4183_acce,
        (
            thrown_exc.as_mut_ptr() as usize,
            exe::ptr::<u8>(0x1_437f_eb90) as usize
        )
    );
    unreachable!()
}

/// `fmt_141462630`
/// Original: 0x141462630 (800 bytes, Throws::No)
pub unsafe extern "win64" fn fmt_141462630(param_1: usize, _param_2: usize) -> usize {
    let [_, xmm1, _, _] = crate::simd_ops::take_xmm_args();
    let raw = xmm1 as u64;
    let mut rsi = raw & 0x000f_ffff_ffff_ffffu64;
    let eax_exp = ((raw >> 0x34) as u32) & 0x7ff;

    let ebp: i32;
    if eax_exp == 0 {
        if rsi == 0 {
            wr::<u64>(param_1, 0);
            wr::<u32>(param_1 + 0x08, 0);
            return param_1;
        }
        ebp = -0x432;
    } else {
        ebp = (eax_exp as i32).wrapping_sub(0x433);
        if rsi == 0 {
            let ebx = (ebp.wrapping_mul(0x9a209).wrapping_sub(0x3fe1f)) >> 0x15;
            let edx = ebx.wrapping_neg();
            let esi = ((edx.wrapping_mul(0x1a934f)) >> 0x13).wrapping_add(ebp);
            let mut cache = [0u64; 2];
            exe::call_va!(
                0x1_4146_1770,
                (cache.as_mut_ptr() as usize, edx as u32)
            );
            let r9 = cache[1];
            let r8_sh = (0xbi32.wrapping_sub(esi) as u32) & 0x3f;
            let rdx_lo = r9.wrapping_sub(r9 >> 0x36) >> r8_sh;
            let rcx_hi = (r9 >> 0x35).wrapping_add(r9) >> r8_sh;
            let r8_bound = if (ebp.wrapping_sub(2) as u32) <= 1 {
                rdx_lo
            } else {
                rdx_lo.wrapping_add(1)
            };
            let rdx_q = (((rcx_hi as u128) * (0xcccc_cccc_cccc_cccdu64 as u128)) >> 64) as u64 >> 3;
            wr::<u64>(param_1, rdx_q);
            if rdx_q.wrapping_mul(10) >= r8_bound {
                wr::<i32>(param_1 + 0x08, ebx.wrapping_add(1));
                let add_exp = exe::call_va!(0x1_4146_1860, (param_1) -> i32);
                wr::<i32>(param_1 + 0x08, rd::<i32>(param_1 + 0x08).wrapping_add(add_exp));
                return param_1;
            }
            wr::<i32>(param_1 + 0x08, ebx);
            let sh_mid = (0xai32.wrapping_sub(esi) as u32) & 0x3f;
            let mut rcx_mid = (r9 >> sh_mid).wrapping_add(1) >> 1;
            wr::<u64>(param_1, rcx_mid);
            if ebp.wrapping_add(0x4d) == 0 {
                if (rcx_mid & 1) != 0 {
                    rcx_mid = rcx_mid.wrapping_sub(1);
                }
                wr::<u64>(param_1, rcx_mid);
                return param_1;
            }
            if rcx_mid >= r8_bound {
                return param_1;
            }
            wr::<u64>(param_1, rcx_mid.wrapping_add(1));
            return param_1;
        }
        rsi |= 0x0010_0000_0000_0000u64;
    }

    let not_sil = !(rsi as u8);
    let eax_k = (ebp.wrapping_mul(0x4d105)) >> 0x14;
    let ebx_neg_k = 2i32.wrapping_sub(eax_k);
    let mut cache = [0u64; 2];
    exe::call_va!(
        0x1_4146_1770,
        (cache.as_mut_ptr() as usize, ebx_neg_k as u32)
    );
    let r9_lo = cache[0];
    let rbp_hi = cache[1];
    let r10_two_fc = rsi.wrapping_add(rsi);
    let r12d_beta = ((ebx_neg_k.wrapping_mul(0x1a934f)) >> 0x13).wrapping_add(ebp);
    let r15d_deltai = (rbp_hi >> ((0x3fi32.wrapping_sub(r12d_beta) as u32) & 0x3f)) as u32;
    let r8_shl = (r10_two_fc | 1) << ((r12d_beta as u32) & 0x3f);

    let prod_hi = (rbp_hi as u128) * (r8_shl as u128);
    let prod_lo_hi = (((r9_lo as u128) * (r8_shl as u128)) >> 64) as u64;
    let sum = prod_hi.wrapping_add(prod_lo_hi as u128);
    let rcx_low = sum as u64;
    let r11_high = (sum >> 64) as u64;
    let cl_zero = u8::from(rcx_low == 0);

    let mut r14 = ((((r11_high as u128) * (0x20c4_9ba5_e353_f7cfu64 as u128)) >> 64) as u64) >> 7;
    wr::<u64>(param_1, r14);
    let mut r11d = (r11_high as u32).wrapping_sub((r14 as u32).wrapping_mul(1000));

    if r11d < r15d_deltai {
        if r11d != 0 || (cl_zero & (rsi as u8)) == 0 {
            wr::<i32>(param_1 + 0x08, eax_k.wrapping_add(1));
            let add_exp = exe::call_va!(0x1_4146_1860, (param_1) -> i32);
            wr::<i32>(param_1 + 0x08, rd::<i32>(param_1 + 0x08).wrapping_add(add_exp));
            return param_1;
        }
        r11d = 1000;
        r14 = r14.wrapping_sub(1);
    } else if r11d == r15d_deltai {
        let rcx_m1 = r10_two_fc.wrapping_sub(1);
        let p_lo = (r9_lo as u128) * (rcx_m1 as u128);
        let rax_lo = p_lo as u64;
        let rdx_hi = (p_lo >> 64) as u64;
        let r8_sh = (0x40i32.wrapping_sub(r12d_beta) as u32) & 0x3f;
        let rcx_sh = (r12d_beta as u32) & 0x3f;
        let r9_comb = rbp_hi.wrapping_mul(rcx_m1).wrapping_add(rdx_hi);
        let rax_shr = rax_lo >> r8_sh;
        let r10b = ((r9_comb >> r8_sh) as u8) & 1;
        let rdx_or = (r9_comb << rcx_sh) | rax_shr;
        let al_cond = (u8::from(rdx_or == 0) & not_sil) | r10b;
        if al_cond != 0 {
            wr::<i32>(param_1 + 0x08, eax_k.wrapping_add(1));
            let add_exp = exe::call_va!(0x1_4146_1860, (param_1) -> i32);
            wr::<i32>(param_1 + 0x08, rd::<i32>(param_1 + 0x08).wrapping_add(add_exp));
            return param_1;
        }
    }

    wr::<i32>(param_1 + 0x08, eax_k);
    let r11d_adj = r11d.wrapping_sub(r15d_deltai >> 1).wrapping_add(0x32);
    let edx_mul = r11d_adj.wrapping_mul(0x290);
    let sil_odd = (r11d_adj as u8) & 1;
    let r9_sig = ((edx_mul >> 16) as u64).wrapping_add(r14.wrapping_mul(10));
    wr::<u64>(param_1, r9_sig);
    if (edx_mul as u16) >= 0x290 {
        return param_1;
    }

    let r8_sh = (0x40i32.wrapping_sub(r12d_beta) as u32) & 0x3f;
    let rcx_sh = (r12d_beta as u32) & 0x3f;
    let p2 = (r9_lo as u128) * (r10_two_fc as u128);
    let rax_lo2 = (p2 as u64) >> r8_sh;
    let rbp_comb = rbp_hi.wrapping_mul(r10_two_fc).wrapping_add((p2 >> 64) as u64);
    let dl_bit = ((rbp_comb >> r8_sh) as u8) & 1;
    let r10_rem = (rbp_comb << rcx_sh) | rax_lo2;
    if dl_bit != sil_odd || (r10_rem == 0 && (r9_sig & 1) != 0) {
        wr::<u64>(param_1, r9_sig.wrapping_sub(1));
    }
    param_1
}

/// `unknown_141462f40`
/// Original: 0x141462f40 (1448 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141462f40(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let ty = rd::<u32>(param_3 + 0x10);
    let idx = ty.wrapping_sub(1);
    if idx > 0xe {
        wr::<u64>(param_1, rd::<u64>(param_2));
        return param_1;
    }

    let mut specs: [u32; 4] = [0, 0xffff_ffff, 0x2000_0000, 0x0100_0000];
    match ty {
        1 => {
            exe::call_va!(0x1_4146_5430, (param_1, rd::<usize>(param_2), rd::<u32>(param_3)));
        }
        2 => {
            exe::call_va!(0x1_4146_5650, (param_1, rd::<usize>(param_2), rd::<u32>(param_3)));
        }
        3 => {
            exe::call_va!(0x1_4146_5770, (param_1, rd::<usize>(param_2), rd::<u64>(param_3)));
        }
        4 => {
            exe::call_va!(0x1_4146_58f0, (param_1, rd::<usize>(param_2), rd::<u64>(param_3)));
        }
        7 => {
            let b = rd::<u8>(param_3);
            let s_ptr = if b != 0 {
                exe::ptr::<u8>(0x1_4307_82bc) as usize
            } else {
                exe::ptr::<u8>(0x1_4307_82b4) as usize
            };
            let mut s_len: usize = 0;
            while rd::<u8>(s_ptr + s_len) != 0 {
                s_len += 1;
            }
            let mut sv: [u64; 2] = [s_ptr as u64, s_len as u64];
            let mut out_res: u64 = 0;
            exe::call_va!(
                0x1_4146_ac70,
                (
                    (&mut out_res as *mut u64) as usize,
                    rd::<usize>(param_2),
                    specs.as_mut_ptr() as usize,
                    0usize,
                    s_len as u64,
                    sv.as_mut_ptr() as usize
                )
            );
            wr::<u64>(param_1, out_res);
        }
        8 => {
            let ch = rd::<u8>(param_3);
            let rbx = rd::<usize>(param_2);
            let cur_len = rd::<u64>(rbx + 0x10);
            if cur_len.wrapping_add(1) > rd::<u64>(rbx + 0x18) {
                let vfn = rd::<u64>(rd::<usize>(rbx));
                exe::call_va!(vfn, (rbx));
            }
            let buf = rd::<usize>(rbx + 0x08);
            let pos = rd::<usize>(rbx + 0x10);
            wr::<u8>(buf.wrapping_add(pos), ch);
            wr::<u64>(rbx + 0x10, (pos as u64).wrapping_add(1));
            wr::<usize>(param_1, rbx);
        }
        9 => {
            let mut f_bits = rd::<u32>(param_3);
            let rsi = rd::<usize>(param_2);
            let dsign_fn = rd::<u64>(exe::ptr::<u64>(0x1_4264_6638) as usize);
            let as_f64_bits = (f32::from_bits(f_bits) as f64).to_bits();
            let (dsign_ret, _) = crate::simd_ops::call_va8_xmm(
                dsign_fn,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [as_f64_bits as u128, 0, 0, 0],
            );
            let edx_sign = dsign_ret as u32;
            let mut rdi_fspecs: u64 = 0;
            if edx_sign != 0 {
                f_bits ^= 0x8000_0000;
                rdi_fspecs = 1u64 << 40;
            }
            if (f_bits & 0x7f80_0000) != 0x7f80_0000 {
                let (dec_fp, _) = crate::simd_ops::call_va8_xmm(
                    0x1_4146_23b0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [f_bits as u128, 0, 0, 0],
                );
                let mut dec_fp_slot: u64 = dec_fp;
                exe::call_va!(
                    0x1_4146_84d0,
                    (
                        param_1,
                        rsi,
                        (&mut dec_fp_slot as *mut u64) as usize,
                        specs.as_mut_ptr() as usize,
                        rdi_fspecs,
                        0u64
                    )
                );
            } else {
                let is_nan = f32::from_bits(f_bits).is_nan();
                let str_va = if is_nan { 0x1_433e_060c } else { 0x1_4342_0018 };
                let mut specs_copy = specs;
                let mut nan_inf: [u64; 2] = [
                    u64::from(edx_sign != 0),
                    exe::ptr::<u8>(str_va) as usize as u64,
                ];
                let len = 3u64 + u64::from(edx_sign != 0);
                exe::call_va!(
                    0x1_4146_add0,
                    (
                        param_1,
                        rsi,
                        specs_copy.as_mut_ptr() as usize,
                        0usize,
                        len,
                        nan_inf.as_mut_ptr() as usize
                    )
                );
            }
        }
        10 | 11 => {
            let mut d_bits = rd::<u64>(param_3);
            let rsi = rd::<usize>(param_2);
            let dsign_fn = rd::<u64>(exe::ptr::<u64>(0x1_4264_6638) as usize);
            let (dsign_ret, _) = crate::simd_ops::call_va8_xmm(
                dsign_fn,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [d_bits as u128, 0, 0, 0],
            );
            let edx_sign = dsign_ret as u32;
            let mut rdi_fspecs: u64 = 0;
            if edx_sign != 0 {
                d_bits ^= 0x8000_0000_0000_0000u64;
                rdi_fspecs = 1u64 << 40;
            }
            if (d_bits & 0x7ff0_0000_0000_0000u64) != 0x7ff0_0000_0000_0000u64 {
                let mut dec_fp: [u64; 2] = [0; 2];
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_4146_2630,
                    dec_fp.as_mut_ptr() as usize as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, d_bits as u128, 0, 0],
                );
                exe::call_va!(
                    0x1_4146_8fb0,
                    (
                        param_1,
                        rsi,
                        dec_fp.as_mut_ptr() as usize,
                        specs.as_mut_ptr() as usize,
                        rdi_fspecs,
                        0u64
                    )
                );
            } else {
                let is_nan = f64::from_bits(d_bits).is_nan();
                let str_va = if is_nan { 0x1_433e_060c } else { 0x1_4342_0018 };
                let mut specs_copy = specs;
                let mut nan_inf: [u64; 2] = [
                    u64::from(edx_sign != 0),
                    exe::ptr::<u8>(str_va) as usize as u64,
                ];
                let len = 3u64 + u64::from(edx_sign != 0);
                exe::call_va!(
                    0x1_4146_add0,
                    (
                        param_1,
                        rsi,
                        specs_copy.as_mut_ptr() as usize,
                        0usize,
                        len,
                        nan_inf.as_mut_ptr() as usize
                    )
                );
            }
        }
        12 => {
            let s_ptr = rd::<usize>(param_3);
            let rbx = rd::<usize>(param_2);
            if s_ptr == 0 {
                throw_format_error(0x1_4342_0000);
            }
            let mut s_len: usize = 0;
            while rd::<u8>(s_ptr + s_len) != 0 {
                s_len += 1;
            }
            exe::call_va!(0x1_401f_9050, (rbx, s_ptr, s_ptr.wrapping_add(s_len)));
            wr::<usize>(param_1, rbx);
        }
        13 => {
            let s_ptr = rd::<usize>(param_3);
            let rbx = rd::<usize>(param_2);
            let s_len = rd::<usize>(param_3 + 0x08);
            exe::call_va!(0x1_401f_9050, (rbx, s_ptr, s_ptr.wrapping_add(s_len)));
            wr::<usize>(param_1, rbx);
        }
        14 => {
            let uptr = rd::<u64>(param_3);
            let mut digits: u32 = 0;
            let mut tmp = uptr;
            loop {
                digits = digits.wrapping_add(1);
                tmp >>= 4;
                if tmp == 0 {
                    break;
                }
            }
            let mut ptr_data: [u64; 2] = [uptr, digits as u64];
            let mut out_res: u64 = 0;
            exe::call_va!(
                0x1_4146_b420,
                (
                    (&mut out_res as *mut u64) as usize,
                    rd::<usize>(param_2),
                    specs.as_mut_ptr() as usize,
                    0usize,
                    (digits as u64).wrapping_add(2),
                    ptr_data.as_mut_ptr() as usize
                )
            );
            wr::<u64>(param_1, out_res);
        }
        15 => {
            let val_ptr = rd::<usize>(param_3);
            let custom_fn = rd::<u64>(param_3 + 0x08);
            let mut parse_ctx = [0u64; 3];
            let mut fmt_ctx: [u64; 4] = [
                rd::<u64>(param_2),
                rd::<u64>(param_2 + 0x08),
                rd::<u64>(param_2 + 0x10),
                rd::<u64>(param_2 + 0x18),
            ];
            exe::call_va!(
                custom_fn,
                (
                    val_ptr,
                    parse_ctx.as_mut_ptr() as usize,
                    fmt_ctx.as_mut_ptr() as usize
                )
            );
            wr::<u64>(param_1, fmt_ctx[0]);
        }
        _ => {
            wr::<u64>(param_1, rd::<u64>(param_2));
        }
    }
    param_1
}

/// `unknown_141463c60`
/// Original: 0x141463c60 (650 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141463c60(param_1: usize, param_2: usize) {
    let mut rsi = rd::<usize>(param_1);
    let len = rd::<i64>(param_1 + 0x08);
    let r12 = rsi.wrapping_add(len as usize);

    if len < 0x20 {
        let mut rdi = rsi;
        while rdi != r12 {
            let r14 = rdi;
            let al = rd::<u8>(rdi);
            rdi = rdi.wrapping_add(1);
            if al == b'{' {
                let rbx = rd::<usize>(param_2 + 0x18);
                exe::call_va!(0x1_401f_9050, (rbx, rsi, rdi.wrapping_sub(1)));
                wr::<usize>(param_2 + 0x18, rbx);
                let next = exe::call_va!(0x1_4146_4d70, (r14, r12, param_2) -> usize);
                rdi = next;
                rsi = next;
            } else if al == b'}' {
                if rdi == r12 || rd::<u8>(rdi) != b'}' {
                    throw_format_error(0x1_4341_ffc0);
                }
                let rbx = rd::<usize>(param_2 + 0x18);
                exe::call_va!(0x1_401f_9050, (rbx, rsi, rdi));
                wr::<usize>(param_2 + 0x18, rbx);
                rdi = rdi.wrapping_add(1);
                rsi = rdi;
            }
        }
        let rbx = rd::<usize>(param_2 + 0x18);
        exe::call_va!(0x1_401f_9050, (rbx, rsi, r12));
        wr::<usize>(param_2 + 0x18, rbx);
    } else {
        let mut ctx_slot: usize = param_2;
        while rsi != r12 {
            let mut r14 = rsi;
            if rd::<u8>(rsi) != b'{' {
                let start = rsi.wrapping_add(1);
                r14 = exe::call_va!(
                    0x1_4183_acd4,
                    (start, 0x7bu32, r12.wrapping_sub(start)) -> usize
                );
                if r14 == 0 {
                    exe::call_va!(
                        0x1_4146_3ef0,
                        ((&mut ctx_slot as *mut usize) as usize, rsi, r12)
                    );
                    return;
                }
            }
            if rsi != r14 {
                let mut rax = exe::call_va!(
                    0x1_4183_acd4,
                    (rsi, 0x7du32, r14.wrapping_sub(rsi)) -> usize
                );
                while rax != 0 {
                    let rdi = rax.wrapping_add(1);
                    if rdi == r14 || rd::<u8>(rdi) != b'}' {
                        throw_format_error(0x1_4341_ffc0);
                    }
                    let rbx = rd::<usize>(param_2 + 0x18);
                    exe::call_va!(0x1_401f_9050, (rbx, rsi, rdi));
                    wr::<usize>(param_2 + 0x18, rbx);
                    rsi = rdi.wrapping_add(1);
                    rax = exe::call_va!(
                        0x1_4183_acd4,
                        (rsi, 0x7du32, r14.wrapping_sub(rsi)) -> usize
                    );
                }
                let rbx = rd::<usize>(param_2 + 0x18);
                exe::call_va!(0x1_401f_9050, (rbx, rsi, r14));
                wr::<usize>(param_2 + 0x18, rbx);
            }
            rsi = exe::call_va!(0x1_4146_4d70, (r14, r12, param_2) -> usize);
        }
    }
}

/// `unknown_141463ef0`
/// Original: 0x141463ef0 (302 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141463ef0(
    param_1: usize,
    mut param_2: usize,
    param_3: usize,
) {
    if param_2 == param_3 {
        return;
    }
    let mut rdi = exe::call_va!(
        0x1_4183_acd4,
        (param_2, 0x7du32, param_3.wrapping_sub(param_2)) -> usize
    );
    while rdi != 0 {
        let next = rdi.wrapping_add(1);
        if next == param_3 || rd::<u8>(next) != b'}' {
            throw_format_error(0x1_4341_ffc0);
        }
        let rsi = rd::<usize>(param_1);
        let rbx = rd::<usize>(rsi + 0x18);
        exe::call_va!(0x1_401f_9050, (rbx, param_2, next));
        wr::<usize>(rsi + 0x18, rbx);
        param_2 = next.wrapping_add(1);
        rdi = exe::call_va!(
            0x1_4183_acd4,
            (param_2, 0x7du32, param_3.wrapping_sub(param_2)) -> usize
        );
    }
    let rsi = rd::<usize>(param_1);
    let rbx = rd::<usize>(rsi + 0x18);
    exe::call_va!(0x1_401f_9050, (rbx, param_2, param_3));
    wr::<usize>(rsi + 0x18, rbx);
}

/// `unknown_141464d70`
/// Original: 0x141464d70 (1046 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141464d70(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut rdi = param_1.wrapping_add(1);
    if rdi == param_2 {
        throw_format_error(0x1_4307_e060);
    }
    let al = rd::<u8>(rdi);
    if al == b'}' {
        let idx = exe::call_va!(0x1_401f_76e0, (param_3) -> u32);
        exe::call_va!(0x1_4146_2110, (param_3, idx));
        return rdi.wrapping_add(1);
    }
    if al == b'{' {
        let rbx = rd::<usize>(param_3 + 0x18);
        exe::call_va!(0x1_401f_9050, (rbx, rdi, rdi.wrapping_add(1)));
        wr::<usize>(param_3 + 0x18, rbx);
        return rdi.wrapping_add(1);
    }

    let mut id_adapter: [u64; 2] = [param_3 as u64, 0];
    let arg_id: u32;
    if al == b':' {
        arg_id = exe::call_va!(0x1_401f_76e0, (param_3) -> u32);
    } else {
        rdi = exe::call_va!(
            0x1_4146_7910,
            (rdi, param_2, id_adapter.as_mut_ptr() as usize) -> usize
        );
        if rdi == param_2 {
            throw_format_error(0x1_4341_ffa0);
        }
        arg_id = id_adapter[1] as u32;
    }

    let ch = rd::<u8>(rdi);
    if ch == b'}' {
        exe::call_va!(0x1_4146_2110, (param_3, arg_id));
        return rdi.wrapping_add(1);
    }
    if ch != b':' {
        throw_format_error(0x1_4341_ffa0);
    }
    rdi = rdi.wrapping_add(1);

    let mut arg_val: [u64; 3] = [0; 3];
    let desc = rd::<u64>(param_3 + 0x20);
    let arg_type: u32;
    if (desc as i64) < 0 {
        let count = (desc & 0x7fff_ffff_ffff_ffffu64) as i32;
        if (arg_id as i32) >= count {
            throw_format_error(0x1_4307_e0e0);
        }
        let elem = rd::<usize>(param_3 + 0x28)
            .wrapping_add(((arg_id as i32 as isize) * 0x18) as usize);
        arg_val[0] = rd::<u64>(elem);
        arg_val[1] = rd::<u64>(elem + 0x08);
        arg_val[2] = rd::<u64>(elem + 0x10);
        arg_type = arg_val[2] as u32;
    } else {
        if (arg_id as i32) >= 0xf {
            throw_format_error(0x1_4307_e0e0);
        }
        let sh = (arg_id.wrapping_mul(4)) & 0x3f;
        arg_type = ((desc >> sh) as u32) & 0xf;
        arg_val[2] = arg_type as u64;
        if arg_type != 0 {
            let elem = rd::<usize>(param_3 + 0x28)
                .wrapping_add(((arg_id as i32 as isize) * 0x10) as usize);
            arg_val[0] = rd::<u64>(elem);
            arg_val[1] = rd::<u64>(elem + 0x08);
        }
    }

    if arg_type == 0 {
        throw_format_error(0x1_4307_e0e0);
    }

    let out_slot = param_3 + 0x18;
    if arg_type == 0xf {
        let advance = rdi.wrapping_sub(rd::<usize>(param_3));
        wr::<usize>(param_3, rdi);
        wr::<u64>(
            param_3 + 0x08,
            rd::<u64>(param_3 + 0x08).wrapping_sub(advance as u64),
        );
        exe::call_va!(arg_val[1], (arg_val[0] as usize, param_3, out_slot));
        rdi = rd::<usize>(param_3);
    } else {
        let mut specs: [u32; 4] = [0, 0xffff_ffff, 0x2000_0000, 0x0100_0000];
        let mut specs_ctx: [u64; 4] = [
            specs.as_mut_ptr() as usize as u64,
            param_3 as u64,
            out_slot as u64,
            arg_type as u64,
        ];
        rdi = exe::call_va!(
            0x1_4146_35a0,
            (rdi, param_2, specs_ctx.as_mut_ptr() as usize) -> usize
        );
        if rdi == param_2 || rd::<u8>(rdi) != b'}' {
            throw_format_error(0x1_4341_ffa0);
        }
        let mut vis_ctx: [u64; 3] = [
            rd::<u64>(out_slot),
            specs.as_mut_ptr() as usize as u64,
            rd::<u64>(param_3 + 0x30),
        ];
        let mut out_res: u64 = 0;
        let ret_ptr = exe::call_va!(
            0x1_4146_3800,
            (
                (&mut out_res as *mut u64) as usize,
                vis_ctx.as_mut_ptr() as usize,
                arg_val.as_mut_ptr() as usize
            ) -> usize
        );
        wr::<u64>(out_slot, rd::<u64>(ret_ptr));
    }

    if rdi == param_2 || rd::<u8>(rdi) != b'}' {
        throw_format_error(0x1_4341_ffe0);
    }
    rdi.wrapping_add(1)
}

/// `unknown_141465430`
/// Original: 0x141465430 (328 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141465430(
    param_1: usize,
    param_2: usize,
    param_3: u32,
) -> usize {
    let mut r10_size = rd::<u64>(param_2 + 0x10);
    let r9_neg = (param_3 >> 31) as u64;
    let mut edi_abs = if r9_neg != 0 {
        param_3.wrapping_neg()
    } else {
        param_3
    };
    let r8_cap = rd::<u64>(param_2 + 0x18);
    let bsr_idx = (31 - (edi_abs | 1).leading_zeros()) as usize;
    let tbl_val = rd::<u64>((exe::ptr::<u64>(0x1_4341_f130) as usize) + bsr_idx * 8);
    let ebp_digits = (tbl_val.wrapping_add(edi_abs as u64) >> 32) as u32;
    let r14_digits = ebp_digits as i32 as i64 as u64;
    let new_size = r14_digits.wrapping_add(r9_neg).wrapping_add(r10_size);

    if new_size <= r8_cap {
        let mut rcx = rd::<usize>(param_2 + 0x08).wrapping_add(r10_size as usize);
        wr::<u64>(param_2 + 0x10, new_size);
        r10_size = new_size;
        if rcx != 0 {
            if r9_neg != 0 {
                wr::<u8>(rcx, b'-');
                rcx = rcx.wrapping_add(1);
            }
            rcx = rcx.wrapping_add(r14_digits as usize);
            let digits_tbl = exe::ptr::<u16>(0x1_4341_fed0) as usize;
            while edi_abs >= 100 {
                rcx = rcx.wrapping_sub(2);
                let q = (((edi_abs as u64) * 0x51eb_851fu64) >> 37) as u32;
                let rem = edi_abs.wrapping_sub(q.wrapping_mul(100)) as usize;
                wr::<u16>(rcx, rd::<u16>(digits_tbl + rem * 2));
                edi_abs = q;
            }
            if edi_abs < 10 {
                wr::<usize>(param_1, param_2);
                wr::<u8>(rcx.wrapping_sub(1), (edi_abs as u8).wrapping_add(b'0'));
                return param_1;
            }
            wr::<usize>(param_1, param_2);
            wr::<u16>(rcx.wrapping_sub(2), rd::<u16>(digits_tbl + (edi_abs as usize) * 2));
            return param_1;
        }
    }

    if r9_neg != 0 {
        if r10_size.wrapping_add(1) > r8_cap {
            let vfn = rd::<u64>(rd::<usize>(param_2));
            exe::call_va!(vfn, (param_2));
        }
        let pos = rd::<usize>(param_2 + 0x10);
        let buf = rd::<usize>(param_2 + 0x08);
        wr::<u8>(buf.wrapping_add(pos), b'-');
        wr::<u64>(param_2 + 0x10, (pos as u64).wrapping_add(1));
    }
    let mut tmp_out = [0u64; 2];
    let ret = exe::call_va!(
        0x1_4146_6800,
        (tmp_out.as_mut_ptr() as usize, param_2, edi_abs, ebp_digits) -> usize
    );
    wr::<u64>(param_1, rd::<u64>(ret + 0x08));
    param_1
}

/// `unknown_141465650`
/// Original: 0x141465650 (281 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141465650(
    param_1: usize,
    param_2: usize,
    param_3: u32,
) -> usize {
    let mut val = param_3;
    let bsr_idx = (31 - (val | 1).leading_zeros()) as usize;
    let rdi_size = rd::<u64>(param_2 + 0x10);
    let tbl_val = rd::<u64>((exe::ptr::<u64>(0x1_4341_f130) as usize) + bsr_idx * 8);
    let r9d_digits = (tbl_val.wrapping_add(val as u64) >> 32) as u32;
    let rdx_digits = r9d_digits as i32 as i64 as u64;
    let new_size = rdx_digits.wrapping_add(rdi_size);

    if new_size <= rd::<u64>(param_2 + 0x18) {
        wr::<u64>(param_2 + 0x10, new_size);
        let mut rcx = rd::<usize>(param_2 + 0x08).wrapping_add(rdi_size as usize);
        if rcx != 0 {
            rcx = rcx.wrapping_add(rdx_digits as usize);
            let digits_tbl = exe::ptr::<u16>(0x1_4341_fed0) as usize;
            while val >= 100 {
                rcx = rcx.wrapping_sub(2);
                let q = (((val as u64) * 0x51eb_851fu64) >> 37) as u32;
                let rem = val.wrapping_sub(q.wrapping_mul(100)) as usize;
                wr::<u16>(rcx, rd::<u16>(digits_tbl + rem * 2));
                val = q;
            }
            if val < 10 {
                wr::<usize>(param_1, param_2);
                wr::<u8>(rcx.wrapping_sub(1), (val as u8).wrapping_add(b'0'));
                return param_1;
            }
            wr::<usize>(param_1, param_2);
            wr::<u16>(rcx.wrapping_sub(2), rd::<u16>(digits_tbl + (val as usize) * 2));
            return param_1;
        }
    }

    let mut tmp_out = [0u64; 2];
    let ret = exe::call_va!(
        0x1_4146_6800,
        (tmp_out.as_mut_ptr() as usize, param_2, val, r9d_digits) -> usize
    );
    wr::<u64>(param_1, rd::<u64>(ret + 0x08));
    param_1
}

/// `unknown_141465770`
/// Original: 0x141465770 (377 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141465770(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let r10_neg = param_3 >> 63;
    let mut rdi_abs = if r10_neg != 0 {
        param_3.wrapping_neg()
    } else {
        param_3
    };
    let mut r9_size = rd::<u64>(param_2 + 0x10);
    let bsr_idx = (63 - (rdi_abs | 1).leading_zeros()) as usize;
    let edx_b = rd::<u8>((exe::ptr::<u8>(0x1_4341_f470) as usize) + bsr_idx) as u32;
    let pow10 = rd::<u64>((exe::ptr::<u64>(0x1_4341_ef00) as usize) + (edx_b as usize) * 8);
    let lt = u32::from(rdi_abs < pow10);
    let ebp_digits = edx_b.wrapping_sub(lt);
    let r8_cap = rd::<u64>(param_2 + 0x18);
    let r14_digits = (edx_b as u64).wrapping_sub(lt as u64);
    let new_size = r14_digits.wrapping_add(r10_neg).wrapping_add(r9_size);

    if new_size <= r8_cap {
        let mut rcx = rd::<usize>(param_2 + 0x08).wrapping_add(r9_size as usize);
        wr::<u64>(param_2 + 0x10, new_size);
        r9_size = new_size;
        if rcx != 0 {
            if r10_neg != 0 {
                wr::<u8>(rcx, b'-');
                rcx = rcx.wrapping_add(1);
            }
            let mut r8_ptr = rcx.wrapping_add(r14_digits as usize);
            let digits_tbl = exe::ptr::<u16>(0x1_4341_fed0) as usize;
            while rdi_abs >= 100 {
                let hi = (((rdi_abs as u128) * (0x47ae_147a_e147_ae15u64 as u128)) >> 64) as u64;
                r8_ptr = r8_ptr.wrapping_sub(2);
                let q = ((rdi_abs.wrapping_sub(hi) >> 1).wrapping_add(hi)) >> 6;
                let rem = rdi_abs.wrapping_sub(q.wrapping_mul(100)) as usize;
                wr::<u16>(r8_ptr, rd::<u16>(digits_tbl + rem * 2));
                rdi_abs = q;
            }
            if rdi_abs < 10 {
                wr::<usize>(param_1, param_2);
                wr::<u8>(r8_ptr.wrapping_sub(1), (rdi_abs as u8).wrapping_add(b'0'));
                return param_1;
            }
            wr::<u16>(r8_ptr.wrapping_sub(2), rd::<u16>(digits_tbl + (rdi_abs as usize) * 2));
            wr::<usize>(param_1, param_2);
            return param_1;
        }
    }

    if r10_neg != 0 {
        if r9_size.wrapping_add(1) > r8_cap {
            let vfn = rd::<u64>(rd::<usize>(param_2));
            exe::call_va!(vfn, (param_2));
        }
        let pos = rd::<usize>(param_2 + 0x10);
        let buf = rd::<usize>(param_2 + 0x08);
        wr::<u8>(buf.wrapping_add(pos), b'-');
        wr::<u64>(param_2 + 0x10, (pos as u64).wrapping_add(1));
    }
    let mut tmp_out = [0u64; 2];
    let ret = exe::call_va!(
        0x1_4146_6910,
        (tmp_out.as_mut_ptr() as usize, param_2, rdi_abs, ebp_digits) -> usize
    );
    wr::<u64>(param_1, rd::<u64>(ret + 0x08));
    param_1
}

/// `unknown_1414658f0`
/// Original: 0x1414658f0 (310 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1414658f0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let mut val = param_3;
    let bsr_idx = (63 - (val | 1).leading_zeros()) as usize;
    let r9d_b = rd::<u8>((exe::ptr::<u8>(0x1_4341_f470) as usize) + bsr_idx) as u32;
    let pow10 = rd::<u64>((exe::ptr::<u64>(0x1_4341_ef00) as usize) + (r9d_b as usize) * 8);
    let lt = u32::from(val < pow10);
    let r9d_digits = r9d_b.wrapping_sub(lt);
    let r11_size = rd::<u64>(param_2 + 0x10);
    let rdx_digits = (r9d_b as u64).wrapping_sub(lt as u64);
    let new_size = rdx_digits.wrapping_add(r11_size);

    if new_size <= rd::<u64>(param_2 + 0x18) {
        wr::<u64>(param_2 + 0x10, new_size);
        let rcx = rd::<usize>(param_2 + 0x08).wrapping_add(r11_size as usize);
        if rcx != 0 {
            let mut r9_ptr = rcx.wrapping_add(rdx_digits as usize);
            let digits_tbl = exe::ptr::<u16>(0x1_4341_fed0) as usize;
            while val >= 100 {
                let hi = (((val as u128) * (0x47ae_147a_e147_ae15u64 as u128)) >> 64) as u64;
                r9_ptr = r9_ptr.wrapping_sub(2);
                let q = ((val.wrapping_sub(hi) >> 1).wrapping_add(hi)) >> 6;
                let rem = val.wrapping_sub(q.wrapping_mul(100)) as usize;
                wr::<u16>(r9_ptr, rd::<u16>(digits_tbl + rem * 2));
                val = q;
            }
            if val < 10 {
                wr::<usize>(param_1, param_2);
                wr::<u8>(r9_ptr.wrapping_sub(1), (val as u8).wrapping_add(b'0'));
                return param_1;
            }
            wr::<u16>(r9_ptr.wrapping_sub(2), rd::<u16>(digits_tbl + (val as usize) * 2));
            wr::<usize>(param_1, param_2);
            return param_1;
        }
    }

    let mut tmp_out = [0u64; 2];
    let ret = exe::call_va!(
        0x1_4146_6910,
        (tmp_out.as_mut_ptr() as usize, param_2, val, r9d_digits) -> usize
    );
    wr::<u64>(param_1, rd::<u64>(ret + 0x08));
    param_1
}

/// `curl_1414684d0`
/// Original: 0x1414684d0 (1058 bytes, Throws::Caught)
pub unsafe extern "win64" fn curl_1414684d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
    param_6: u64,
) -> usize {
    let mut local_p5: u64 = param_5;
    let r13d_sig = rd::<u32>(param_3);
    let mut local_sig: u32 = r13d_sig;
    let mut local_num_zeros: i32;

    let bsr_idx = (31 - (r13d_sig | 1).leading_zeros()) as usize;
    let tbl_val = rd::<u64>((exe::ptr::<u64>(0x1_4341_f130) as usize) + bsr_idx * 8);
    let r9d_num_digits = ((r13d_sig as u64).wrapping_add(tbl_val) >> 32) as u32;
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

    let r12d_exp = rd::<i32>(param_3 + 0x04);
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
            let mut local_71: i32 = eax_prec.wrapping_sub(r8d_output_exp);
            if (dl_flags & 8) != 0 {
                r14_size = r14_size.wrapping_add(1);
                if local_71 <= 0 && cl_fmt != 2 {
                    local_71 = 1;
                }
                if local_71 > 0 {
                    r14_size = r14_size.wrapping_add(local_71 as u32 as u64);
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
                (&mut local_sig as *mut u32) as usize,
                (&mut local_num_digits as *mut u32) as usize,
                param_3,
                gp,
                (&mut local_p5 as *mut u64) as usize,
                bytes_30.as_mut_ptr().add(1) as usize,
                (&mut local_71 as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(2) as usize,
            ];
            exe::call_va!(
                0x1_4146_c8a0,
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
                (&mut local_sig as *mut u32) as usize,
                (&mut local_num_digits as *mut u32) as usize,
                (&mut local_output_exp as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(1) as usize,
                gp,
                (&mut local_67 as *mut i32) as usize,
                bytes_30.as_mut_ptr().add(2) as usize,
            ];
            exe::call_va!(
                0x1_4146_ca00,
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
            (&mut local_sig as *mut u32) as usize,
            (&mut local_num_digits as *mut u32) as usize,
        ];
        let add_len = (has_pt as u32)
            .wrapping_add((r8d_neg as u32).wrapping_add(1)) as u64;
        let total_size = r14_size.wrapping_add(add_len);
        exe::call_va!(
            0x1_4146_cb60,
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

    let mut exp_lambda = [0u8; 32];
    let elp = exp_lambda.as_mut_ptr() as usize;
    wr::<u8>(elp, bytes_30[3]);
    wr::<u32>(elp + 0x04, r13d_sig);
    wr::<u32>(elp + 0x08, r9d_num_digits);
    wr::<u8>(elp + 0x0c, r10b_dec_pt);
    wr::<u32>(elp + 0x10, r8d_zeros);
    wr::<u8>(elp + 0x14, bytes_30[2]);
    wr::<u8>(elp + 0x15, (((!dl_flags) & 1) << 5) | 0x45);
    wr::<i32>(elp + 0x18, r11d_exp10);

    if rd::<i32>(param_4) > 0 {
        let total_sz = u64::from(r10b_dec_pt != 0)
            .wrapping_add(exp_digits)
            .wrapping_add(2)
            .wrapping_add(r14_size);
        let mut out_res: u64 = 0;
        exe::call_va!(
            0x1_4146_c740,
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
        let mut dummy: u64 = 0;
        let ret_ptr = exe::call_va!(
            0x1_4146_8900,
            (elp, (&mut dummy as *mut u64) as usize, param_2) -> usize
        );
        wr::<u64>(param_1, rd::<u64>(ret_ptr));
    }
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_fmt_141462630(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, val: f64| -> (u64, i32) {
        let mut out = [0u64; 2];
        let op = out.as_mut_ptr() as usize;
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            op as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, val.to_bits() as u128, 0, 0],
        );
        (rd::<u64>(op), rd::<i32>(op + 0x08))
    };

    let cases: [f64; 12] = [
        0.0,
        1.0,
        2.0,
        4.0,
        0.5,
        0.25,
        0.1,
        3.141592653589793,
        123456.789,
        1e-20,
        1e25,
        f64::from_bits(1),
    ];
    for (i, &v) in cases.iter().enumerate() {
        let o = run(orig, v);
        let n = run(new, v);
        crate::diff_eq!(o, n, "141462630 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_141462f40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let cstr = b"hello\0";
    let run = |f: F, ty: u32, v0: u64, v1: u64| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut visitor: [u64; 4] = [bp as u64, 0, 0, 0];
        let mut arg: [u64; 3] = [v0, v1, ty as u64];
        let mut out_ptr: usize = 0;

        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            visitor.as_mut_ptr() as usize,
            arg.as_mut_ptr() as usize,
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    let cases: [(u32, u64, u64); 11] = [
        (0, 0, 0),
        (1, (-42i32 as u32) as u64, 0),
        (2, 12345u64, 0),
        (3, (-9876543210i64) as u64, 0),
        (4, 9876543210u64, 0),
        (7, 1, 0),
        (7, 0, 0),
        (8, b'Z' as u64, 0),
        (9, (-2.5f32).to_bits() as u64, 0),
        (10, 6.25f64.to_bits(), 0),
        (12, cstr.as_ptr() as usize as u64, 0),
    ];

    for (i, &(ty, v0, v1)) in cases.iter().enumerate() {
        let o = run(f_o, ty, v0, v1);
        let n = run(f_n, ty, v0, v1);
        crate::diff_eq!(o, n, "141462f40 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_141463c60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 256] = [0; 256];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, fmt_bytes: &[u8]| -> Vec<u8> {
        CHAR_BUF = [0; 256];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 256);

        let mut arg_values: [[u64; 2]; 2] = [[42, 0], [b"world".as_ptr() as usize as u64, 5]];
        let mut handler: [u64; 7] = [
            fmt_bytes.as_ptr() as usize as u64,
            fmt_bytes.len() as u64,
            0,
            bp as u64,
            0x01 | (0x0d << 4),
            arg_values.as_mut_ptr() as usize as u64,
            0,
        ];
        let mut sv: [u64; 2] = [fmt_bytes.as_ptr() as usize as u64, fmt_bytes.len() as u64];
        f(sv.as_mut_ptr() as usize, handler.as_mut_ptr() as usize);
        let len = rd::<u64>(bp + 0x10) as usize;
        CHAR_BUF[..len].to_vec()
    };

    let cases: [&[u8]; 4] = [
        b"short literal {{}} {}",
        b"num={0} str={1}!",
        b"0123456789abcdef0123456789abcdef plain }} tail",
        b"0123456789abcdef0123456789abcdef val={} }} str={}",
    ];
    for (i, &s) in cases.iter().enumerate() {
        let o = run(f_o, s);
        let n = run(f_n, s);
        crate::diff_eq!(o, n, "141463c60 case={}", i);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_141463ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, text: &[u8]| -> Vec<u8> {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut handler: [u64; 4] = [0, 0, 0, bp as u64];
        let mut h_ptr: usize = handler.as_mut_ptr() as usize;
        let start = text.as_ptr() as usize;
        f(
            (&mut h_ptr as *mut usize) as usize,
            start,
            start + text.len(),
        );
        let len = rd::<u64>(bp + 0x10) as usize;
        CHAR_BUF[..len].to_vec()
    };

    for (i, s) in [
        &b""[..],
        &b"plain text"[..],
        &b"escaped }} brace }} end"[..],
    ]
    .iter()
    .enumerate()
    {
        let o = run(f_o, s);
        let n = run(f_n, s);
        crate::diff_eq!(o, n, "141463ef0 case={}", i);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141464d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, spec: &[u8]| -> (usize, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut arg_values: [[u64; 2]; 2] = [[255, 0], [77, 0]];
        let start = spec.as_ptr() as usize;
        let end = start + spec.len();
        let mut handler: [u64; 7] = [
            start as u64,
            spec.len() as u64,
            0,
            bp as u64,
            0x01 | (0x01 << 4),
            arg_values.as_mut_ptr() as usize as u64,
            0,
        ];
        let ret = f(start, end, handler.as_mut_ptr() as usize);
        let len = rd::<u64>(bp + 0x10) as usize;
        (ret.wrapping_sub(start), CHAR_BUF[..len].to_vec())
    };

    for (i, s) in [
        &b"{}"[..],
        &b"{{rest"[..],
        &b"{1}"[..],
        &b"{:04x}"[..],
        &b"{0:+6d}"[..],
    ]
    .iter()
    .enumerate()
    {
        let o = run(f_o, s);
        let n = run(f_n, s);
        crate::diff_eq!(o, n, "141464d70 case={}", i);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_141465430(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 64] = [0; 64];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, v: i32| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 64];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 64);

        let mut out_ptr: usize = 0;
        let ret = f((&mut out_ptr as *mut usize) as usize, bp, v as u32);
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    for v in [0i32, 7, 42, 99, 100, 123456, -1, -42, -987654, i32::MIN, i32::MAX] {
        let o = run(f_o, v);
        let n = run(f_n, v);
        crate::diff_eq!(o, n, "141465430 v={}", v);
    }
    Ok(11)
}

pub unsafe fn diff_unknown_141465650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 64] = [0; 64];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, v: u32| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 64];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 64);

        let mut out_ptr: usize = 0;
        let ret = f((&mut out_ptr as *mut usize) as usize, bp, v);
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    for v in [0u32, 5, 10, 99, 100, 999, 1234567, u32::MAX] {
        let o = run(f_o, v);
        let n = run(f_n, v);
        crate::diff_eq!(o, n, "141465650 v={}", v);
    }
    Ok(8)
}

pub unsafe fn diff_unknown_141465770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 64] = [0; 64];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, v: i64| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 64];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 64);

        let mut out_ptr: usize = 0;
        let ret = f((&mut out_ptr as *mut usize) as usize, bp, v as u64);
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    for v in [
        0i64,
        8,
        42,
        100,
        1234567890123,
        -1,
        -99,
        -1234567890123,
        i64::MIN,
        i64::MAX,
    ] {
        let o = run(f_o, v);
        let n = run(f_n, v);
        crate::diff_eq!(o, n, "141465770 v={}", v);
    }
    Ok(10)
}

pub unsafe fn diff_unknown_1414658f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 64] = [0; 64];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, v: u64| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 64];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 64);

        let mut out_ptr: usize = 0;
        let ret = f((&mut out_ptr as *mut usize) as usize, bp, v);
        let len = rd::<u64>(bp + 0x10) as usize;
        (
            ret == (&mut out_ptr as *mut usize) as usize && out_ptr == bp,
            CHAR_BUF[..len].to_vec(),
        )
    };

    for v in [0u64, 9, 10, 99, 100, 1000, 987654321012345, u64::MAX] {
        let o = run(f_o, v);
        let n = run(f_n, v);
        crate::diff_eq!(o, n, "1414658f0 v={}", v);
    }
    Ok(8)
}

pub unsafe fn diff_curl_1414684d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHAR_BUF: [u8; 128] = [0; 128];
    static mut FMT_BUF: [u64; 4] = [0; 4];

    let run = |f: F, sig: u32, exp: i32, width: u32, p5: u64| -> (bool, Vec<u8>) {
        CHAR_BUF = [0; 128];
        let bp = core::ptr::addr_of_mut!(FMT_BUF) as usize;
        let cp = core::ptr::addr_of_mut!(CHAR_BUF) as usize;
        wr::<usize>(bp + 0x08, cp);
        wr::<u64>(bp + 0x10, 0);
        wr::<u64>(bp + 0x18, 128);

        let mut dec_fp: [u32; 2] = [sig, exp as u32];
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

    let cases: [(u32, i32, u32, u64); 6] = [
        (31415, -4, 0, 0xffff_ffffu64),
        (125, 2, 0, 0xffff_ffffu64 | (1u64 << 40)),
        (42, -5, 0, 0xffff_ffffu64 | (2u64 << 32)),
        (5, 0, 0, 0xffff_ffffu64 | (1u64 << 32)),
        (1234, -8, 12, 0xffff_ffffu64 | (1u64 << 32) | (1u64 << 48)),
        (99, 1, 8, 4u64 | (2u64 << 32) | (8u64 << 48)),
    ];

    for (i, &(sig, exp, width, p5)) in cases.iter().enumerate() {
        let o = run(f_o, sig, exp, width, p5);
        let n = run(f_n, sig, exp, width, p5);
        crate::diff_eq!(o, n, "1414684d0 case={}", i);
    }
    Ok(cases.len() as u32)
}
