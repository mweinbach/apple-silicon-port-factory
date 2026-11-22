//! Decompiled `rend` hot functions shard ZA17 (`0x142202be0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn fdiv_x86(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return f32::from_bits(a.to_bits() | 0x0040_0000);
    }
    if b.is_nan() {
        return f32::from_bits(b.to_bits() | 0x0040_0000);
    }
    let q = a / b;
    if q.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        q
    }
}

#[inline(always)]
fn fadd_x86(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return f32::from_bits(a.to_bits() | 0x0040_0000);
    }
    if b.is_nan() {
        return f32::from_bits(b.to_bits() | 0x0040_0000);
    }
    let s = a + b;
    if s.is_nan() {
        f32::from_bits(0xffc0_0000)
    } else {
        s
    }
}

#[inline(always)]
unsafe fn call_powf(x: f32, y: f32) -> f32 {
    let (_, xmm0) = crate::simd_ops::call_va8_xmm(
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
    f32::from_bits(xmm0 as u32)
}

#[inline(always)]
unsafe fn init_cvar_param(
    tls0: usize,
    guard_va: u64,
    ptr_va: u64,
    id_va: u64,
    type_va: u64,
    val_va: u64,
    name_va: u64,
    name_len: u64,
    cvar_type: u32,
    zero_dwords: usize,
    atexit_va: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            wr::<usize>(exe::ptr::<usize>(ptr_va) as usize, 0);
            let mut sv: [u64; 2] = [exe::ptr::<u8>(name_va) as usize as u64, name_len];
            exe::call_va!(
                0x1_4220_ea70,
                (exe::ptr::<u8>(id_va) as usize, sv.as_mut_ptr() as usize)
            );
            wr::<u32>(exe::ptr::<u32>(type_va) as usize, cvar_type);
            let val_ptr = exe::ptr::<u8>(val_va) as usize;
            wr::<usize>(exe::ptr::<usize>(ptr_va) as usize, val_ptr);
            for i in 0..zero_dwords {
                wr::<u32>(val_ptr + i * 4, 0);
            }
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn update_tls_cbuffer(base: usize, tls0: usize, id: i32, src_ptr: usize) {
    let idx = id as isize as usize;
    let entry = base.wrapping_add(0x3a6_37b0).wrapping_add(idx.wrapping_mul(16));
    let sz = rd::<i32>(entry + 0x08) as i64 as u64;
    let off = rd::<i32>(entry) as isize;
    if off >= 0 {
        let dst = tls0.wrapping_add(0x63d0).wrapping_add(off as usize);
        exe::call_va!(0x1_4183_acbc, (dst, src_ptr, sz));
        let cbuf = rd::<i32>(entry + 0x0c) as isize;
        if cbuf != -1 {
            let cnt_addr = tls0
                .wrapping_add(0x463e0)
                .wrapping_add((cbuf as usize).wrapping_mul(4));
            let next = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next == -1 { 0 } else { next });
        }
    }
}

#[inline(always)]
unsafe fn select_tex_srv(mode: i32, tex: usize, fallback_base: usize) -> u64 {
    if mode == 0 {
        if tex != 0 {
            rd::<u64>(tex + 0x78)
        } else {
            rd::<u64>(fallback_base + 0x88)
        }
    } else if mode == 5 {
        if tex != 0 {
            rd::<u64>(tex + 0x90)
        } else {
            rd::<u64>(fallback_base + 0x90)
        }
    } else {
        rd::<u64>(fallback_base + 0x88)
    }
}

#[inline(always)]
unsafe fn call_set_lut_slot(lut_mgr: usize, slot: u32, res_ptr: usize, weight: f32) {
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_423c_5090,
        lut_mgr as u64,
        slot as u64,
        res_ptr as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 0, weight.to_bits() as u128],
    );
}

/// `unknown_142202be0`
/// Original: 0x142202be0 (5341 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142202be0(
    param_1: usize,
    param_2: u8,
    _param_3: usize,
    _param_4: usize,
) {
    let [_xmm0, _xmm1, _xmm2, xmm3_in] = crate::simd_ops::take_xmm_args();
    let param_4_bits = xmm3_in as u32;

    let base = exe::ptr::<u8>(0x1_4000_0000) as usize;
    let idx_c01 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7c01) as usize) as usize;
    let xmm13 = rd::<f32>(base + 0x386_7c04 + idx_c01 * 4);
    let idx_b21 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7b21) as usize) as usize;
    let xmm14 = rd::<f32>(base + 0x386_7b24 + idx_b21 * 4);
    let idx_2c1 = rd::<u8>(exe::ptr::<u8>(0x1_4386_72c1) as usize) as usize;
    let xmm15 = rd::<f32>(base + 0x386_72c4 + idx_2c1 * 4);
    let idx_a61 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7a61) as usize) as usize;
    let xmm9 = rd::<f32>(base + 0x386_7a64 + idx_a61 * 4);

    let xmm11 = xmm14 * xmm13;
    let xmm10 = rd::<f32>(exe::ptr::<f32>(0x1_4386_7b40) as usize);
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);

    let p1 = call_powf(xmm10, xmm13);
    let xmm12 = f32::from_bits(p1.to_bits() ^ sign_mask);
    let p2 = call_powf(xmm9, xmm11);
    let p3 = call_powf(xmm10, xmm13);
    let xmm8 = p3 * p2;
    let p4 = call_powf(xmm9, xmm13);
    let p5 = call_powf(xmm10, xmm11);
    let xmm7_t = (p4 * p5) * xmm15;
    let p6 = call_powf(xmm9, xmm11);
    let xmm6_t = p6 * xmm15;
    let p7 = call_powf(xmm10, xmm11);
    let xmm3_t = p7 * xmm15;

    let xmm5 = xmm8 - xmm7_t;
    let xmm4 = xmm6_t - xmm3_t;
    let xmm3_div = fdiv_x86(fadd_x86(fdiv_x86(xmm5 * xmm15, xmm4), xmm12), xmm3_t);
    let xmm6_gt = f32::from_bits(xmm3_div.to_bits() ^ sign_mask);
    let xmm7_gt = fdiv_x86(xmm5, xmm4);
    let mut gt_params: [f32; 4] = [xmm13, xmm14, xmm6_gt, xmm7_gt];

    let tls0 = rd::<usize>(exe::tls_pointer());

    init_cvar_param(
        tls0,
        0x1_43ce_ff78,
        0x1_43ce_fe60,
        0x1_43ce_fe68,
        0x1_43ce_fe6c,
        0x1_43ce_fe70,
        0x1_433e_f998,
        0x12,
        3,
        4,
        0x1_4262_7800,
    );
    let id_gt = rd::<i32>(exe::ptr::<i32>(0x1_43ce_fe68) as usize);
    update_tls_cbuffer(base, tls0, id_gt, gt_params.as_mut_ptr() as usize);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe70) as usize, xmm13);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe74) as usize, xmm14);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe78) as usize, xmm6_gt);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe7c) as usize, xmm7_gt);

    init_cvar_param(
        tls0,
        0x1_43cf_00a8,
        0x1_43ce_fe48,
        0x1_43ce_fe50,
        0x1_43ce_fe54,
        0x1_43ce_fe58,
        0x1_433e_f968,
        0x14,
        0,
        0,
        0x1_4262_77f0,
    );
    let idx_wp = rd::<u8>(exe::ptr::<u8>(0x1_4386_7a61) as usize) as usize;
    let mut wp: f32 = rd::<f32>(base + 0x386_7a64 + idx_wp * 4);
    let id_wp = rd::<i32>(exe::ptr::<i32>(0x1_43ce_fe50) as usize);
    update_tls_cbuffer(base, tls0, id_wp, (&mut wp as *mut f32) as usize);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe58) as usize, wp);

    init_cvar_param(
        tls0,
        0x1_43cf_00ac,
        0x1_43cf_00f8,
        0x1_43cf_0100,
        0x1_43cf_0104,
        0x1_43cf_0108,
        0x1_433e_f980,
        0x13,
        2,
        3,
        0x1_4262_77e0,
    );
    let idx_ct = rd::<u8>(exe::ptr::<u8>(0x1_4386_73a9) as usize) as usize;
    let ct_off = base + 0x386_73ac + idx_ct * 12;
    let mut ct_buf = [0u32; 4];
    wr::<u64>(ct_buf.as_mut_ptr() as usize, rd::<u64>(ct_off));
    wr::<u32>(ct_buf.as_mut_ptr() as usize + 8, rd::<u32>(ct_off + 8));
    let id_ct = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0100) as usize);
    update_tls_cbuffer(base, tls0, id_ct, ct_buf.as_mut_ptr() as usize);
    wr::<u64>(exe::ptr::<u64>(0x1_43cf_0108) as usize, rd::<u64>(ct_off));
    wr::<u32>(
        exe::ptr::<u32>(0x1_43cf_0110) as usize,
        rd::<u32>(ct_off + 8),
    );

    init_cvar_param(
        tls0,
        0x1_43cf_00ec,
        0x1_43cf_0030,
        0x1_43cf_0038,
        0x1_43cf_003c,
        0x1_43cf_0040,
        0x1_433e_f9e0,
        0x1d,
        2,
        3,
        0x1_4262_77d0,
    );
    let idx_cts = rd::<u8>(exe::ptr::<u8>(0x1_4386_7989) as usize) as usize;
    let cts_off = base + 0x386_798c + idx_cts * 12;
    wr::<u64>(ct_buf.as_mut_ptr() as usize, rd::<u64>(cts_off));
    wr::<u32>(ct_buf.as_mut_ptr() as usize + 8, rd::<u32>(cts_off + 8));
    let id_cts = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0038) as usize);
    update_tls_cbuffer(base, tls0, id_cts, ct_buf.as_mut_ptr() as usize);
    wr::<u64>(exe::ptr::<u64>(0x1_43cf_0040) as usize, rd::<u64>(cts_off));
    wr::<u32>(
        exe::ptr::<u32>(0x1_43cf_0048) as usize,
        rd::<u32>(cts_off + 8),
    );

    init_cvar_param(
        tls0,
        0x1_43ce_fef8,
        0x1_43ce_ff80,
        0x1_43ce_ff88,
        0x1_43ce_ff8c,
        0x1_43ce_ff90,
        0x1_433e_fa00,
        0x0e,
        0,
        0,
        0x1_4262_77c0,
    );
    let idx_vig = rd::<u8>(exe::ptr::<u8>(0x1_4386_7481) as usize) as usize;
    let mut vig_exp: f32 = rd::<f32>(base + 0x386_7484 + idx_vig * 4);
    let id_vig = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ff88) as usize);
    update_tls_cbuffer(base, tls0, id_vig, (&mut vig_exp as *mut f32) as usize);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_ff90) as usize, vig_exp);

    init_cvar_param(
        tls0,
        0x1_43ce_ff38,
        0x1_43cf_0070,
        0x1_43cf_0078,
        0x1_43cf_007c,
        0x1_43cf_0080,
        0x1_433e_f9b0,
        0x19,
        1,
        2,
        0x1_4262_77b0,
    );
    let mid_gray_in = rd::<f32>(exe::ptr::<f32>(0x1_4342_6628) as usize);
    let mid_gray_out = if param_2 == 0 { xmm15 } else { mid_gray_in };
    let mut mid_gray_pair: [f32; 2] = [mid_gray_in, mid_gray_out];
    let id_mg = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0078) as usize);
    update_tls_cbuffer(base, tls0, id_mg, mid_gray_pair.as_mut_ptr() as usize);
    wr::<u64>(
        exe::ptr::<u64>(0x1_43cf_0080) as usize,
        rd::<u64>(mid_gray_pair.as_ptr() as usize),
    );

    init_cvar_param(
        tls0,
        0x1_43cf_00c8,
        0x1_43cf_0050,
        0x1_43cf_0058,
        0x1_43cf_005c,
        0x1_43cf_0060,
        0x1_433e_f9d0,
        0x0c,
        9,
        4,
        0x1_4262_77a0,
    );
    let fg_res = rd::<usize>(exe::ptr::<usize>(0x1_43bb_cff8) as usize);
    let rdi_fg = if fg_res != 0 {
        rd::<usize>(fg_res + 0x1a0)
    } else {
        0
    };
    let id_fg = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0058) as usize);
    let mode_fg = rd::<i32>(base + 0x385_8590 + (id_fg as isize as usize) * 4);
    let fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let srv_fg = select_tex_srv(mode_fg, rdi_fg, fcb8);
    let mut fg_pair: [u64; 2] = [srv_fg, rdi_fg as u64];
    update_tls_cbuffer(base, tls0, id_fg, fg_pair.as_mut_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43cf_0068) as usize, rdi_fg);
    let srv_fg2 = select_tex_srv(mode_fg, rdi_fg, fcb8);
    wr::<u64>(exe::ptr::<u64>(0x1_43cf_0060) as usize, srv_fg2);

    let fgi_val: f32 = if exe::call_va!(0x1_4225_8d40, () -> u8) == 0 {
        init_cvar_param(
            tls0,
            0x1_43ce_fe20,
            0x1_43ce_ffb0,
            0x1_43ce_ffb8,
            0x1_43ce_ffbc,
            0x1_43ce_ffc0,
            0x1_433e_f8f8,
            0x15,
            0,
            0,
            0x1_4262_7780,
        );
        let idx_fgi = rd::<u8>(exe::ptr::<u8>(0x1_4386_7641) as usize) as usize;
        let mut fgi: f32 = rd::<f32>(base + 0x386_7644 + idx_fgi * 4);
        let id_fgi = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ffb8) as usize) as isize;
        let sz_fgi = rd::<i32>(base + 0x3a6_37b8 + (id_fgi as usize) * 16) as i64 as u64;
        exe::call_va!(
            0x1_4209_a900,
            (
                id_fgi as i64 as u64,
                0u32,
                sz_fgi,
                (&mut fgi as *mut f32) as usize
            )
        );
        wr::<f32>(exe::ptr::<f32>(0x1_43ce_ffc0) as usize, fgi);
        fgi
    } else {
        init_cvar_param(
            tls0,
            0x1_43ce_fea0,
            0x1_43ce_ff60,
            0x1_43ce_ff68,
            0x1_43ce_ff6c,
            0x1_43ce_ff70,
            0x1_433e_f8f8,
            0x15,
            0,
            0,
            0x1_4262_7790,
        );
        let mut fgi: f32 = 0.0f32;
        let id_fgi = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ff68) as usize) as isize;
        let sz_fgi = rd::<i32>(base + 0x3a6_37b8 + (id_fgi as usize) * 16) as i64 as u64;
        exe::call_va!(
            0x1_4209_a900,
            (
                id_fgi as i64 as u64,
                0u32,
                sz_fgi,
                (&mut fgi as *mut f32) as usize
            )
        );
        wr::<f32>(exe::ptr::<f32>(0x1_43ce_ff70) as usize, fgi);
        fgi
    };

    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4220_2830,
        0,
        param_2 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [fgi_val.to_bits() as u128, 0, 0, param_4_bits as u128],
    );

    let mut edi_h: i32 = 0x200;
    let rcx_fg = rd::<usize>(exe::ptr::<usize>(0x1_43bb_cff8) as usize);
    let esi_w: i32 = if rcx_fg != 0 {
        let w = exe::call_va!(0x1_4220_aec0, (rcx_fg) -> i32);
        let rcx_fg2 = rd::<usize>(exe::ptr::<usize>(0x1_43bb_cff8) as usize);
        if rcx_fg2 != 0 {
            edi_h = exe::call_va!(0x1_4220_ae60, (rcx_fg2) -> i32);
        }
        w
    } else {
        0x200
    };

    init_cvar_param(
        tls0,
        0x1_43ce_fe18,
        0x1_43ce_ff98,
        0x1_43ce_ffa0,
        0x1_43ce_ffa4,
        0x1_43ce_ffa8,
        0x1_433e_f910,
        0x12,
        0x11,
        2,
        0x1_4262_7770,
    );

    let mut r8d = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize) & (0x8000_01ffu32 as i32);
    if r8d < 0 {
        r8d = ((r8d.wrapping_sub(1) as u32 | 0xffff_fe00) as i32).wrapping_add(1);
    }
    r8d = r8d.wrapping_add(1);

    let mut ecx_3 = r8d;
    let mut denom_3: i32 = 3;
    let mut acc_3: f64 = 0.0;
    if r8d > 0 {
        loop {
            let q = ecx_3 / 3;
            let rem = ecx_3 - q * 3;
            acc_3 += (rem as f64) / (denom_3 as f64);
            ecx_3 = q;
            denom_3 = denom_3.wrapping_mul(3);
            if q <= 0 {
                break;
            }
        }
    }
    let off_y: i32 = ((acc_3 as f32) * (edi_h as f32)) as i32;

    let mut cur_2 = r8d;
    let mut denom_2: i32 = 2;
    let mut acc_2: f64 = 0.0;
    if cur_2 > 0 {
        loop {
            let q = cur_2 / 2;
            let rem = cur_2 % 2;
            acc_2 += (rem as f64) / (denom_2 as f64);
            denom_2 = denom_2.wrapping_add(denom_2);
            cur_2 = q;
            if q <= 0 {
                break;
            }
        }
    }
    let off_x: i32 = ((acc_2 as f32) * (esi_w as f32)) as i32;

    let mut fg_off_pair: [i32; 2] = [off_x, off_y];
    let id_fgo = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ffa0) as usize);
    update_tls_cbuffer(base, tls0, id_fgo, fg_off_pair.as_mut_ptr() as usize);
    wr::<i32>(exe::ptr::<i32>(0x1_43ce_ffa8) as usize, off_x);
    wr::<i32>(exe::ptr::<i32>(0x1_43ce_ffac) as usize, off_y);

    init_cvar_param(
        tls0,
        0x1_43ce_fea4,
        0x1_43ce_fe88,
        0x1_43ce_fe90,
        0x1_43ce_fe94,
        0x1_43ce_fe98,
        0x1_433e_f800,
        0x14,
        0,
        0,
        0x1_4262_7760,
    );
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let mut bright: f32 = one;
    let id_tb = rd::<i32>(exe::ptr::<i32>(0x1_43ce_fe90) as usize);
    update_tls_cbuffer(base, tls0, id_tb, (&mut bright as *mut f32) as usize);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_fe98) as usize, one);

    let r10_obj = rd::<usize>(param_1);
    if exe::call_va!(0x1_4225_8d40, () -> u8) == 0 {
        let idx_7c1 = rd::<u8>(exe::ptr::<u8>(0x1_4386_77c1) as usize) as usize;
        let v_7c4 = rd::<f32>(base + 0x386_77c4 + idx_7c1 * 4);
        let idx_701 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7701) as usize) as usize;
        let v_704 = rd::<f32>(base + 0x386_7704 + idx_701 * 4);
        let diff = v_704 - v_7c4;
        let v_b = fdiv_x86(f32::from_bits(v_7c4.to_bits() ^ sign_mask), diff);
        let v_a = fdiv_x86(one, diff);
        let d1 = u8::from(rd::<f32>(r10_obj + 0x04) != v_a || rd::<f32>(r10_obj + 0x08) != v_b);
        wr::<u8>(r10_obj, rd::<u8>(r10_obj) | d1);
        wr::<f32>(r10_obj + 0x04, v_a);
        wr::<f32>(r10_obj + 0x08, v_b);

        let obj = rd::<usize>(param_1);
        let idx_201 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7201) as usize) as usize;
        let v_204 = rd::<f32>(base + 0x386_7204 + idx_201 * 4);
        let d2 = u8::from(rd::<f32>(obj + 0x14) != v_204);
        wr::<u8>(obj, rd::<u8>(obj) | d2);
        wr::<f32>(obj + 0x14, v_204);

        let idx_129 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7129) as usize) as usize;
        let off_12c = base + 0x386_712c + idx_129 * 12;
        let mut vec3 = [0u32; 4];
        wr::<u64>(vec3.as_mut_ptr() as usize, rd::<u64>(off_12c));
        wr::<u32>(vec3.as_mut_ptr() as usize + 8, rd::<u32>(off_12c + 8));
        exe::call_va!(
            0x1_412d_c600,
            (rd::<usize>(param_1), vec3.as_mut_ptr() as usize)
        );

        let idx_569 = rd::<u8>(exe::ptr::<u8>(0x1_4386_7569) as usize) as usize;
        let off_56c = base + 0x386_756c + idx_569 * 12;
        wr::<u64>(vec3.as_mut_ptr() as usize, rd::<u64>(off_56c));
        wr::<u32>(vec3.as_mut_ptr() as usize + 8, rd::<u32>(off_56c + 8));
        exe::call_va!(
            0x1_412d_c660,
            (rd::<usize>(param_1), vec3.as_mut_ptr() as usize)
        );

        let obj2 = rd::<usize>(param_1);
        let idx_8a1 = rd::<u8>(exe::ptr::<u8>(0x1_4386_78a1) as usize) as usize;
        let v_8a4 = rd::<f32>(base + 0x386_78a4 + idx_8a1 * 4);
        let d3 = u8::from(rd::<f32>(obj2 + 0x30) != v_8a4);
        wr::<u8>(obj2, rd::<u8>(obj2) | d3);
        wr::<f32>(obj2 + 0x30, v_8a4);
    } else {
        let d1 = u8::from(rd::<f32>(r10_obj + 0x04) != one || rd::<f32>(r10_obj + 0x08) != 0.0f32);
        wr::<u8>(r10_obj, rd::<u8>(r10_obj) | d1);
        wr::<f32>(r10_obj + 0x04, one);
        wr::<f32>(r10_obj + 0x08, 0.0f32);

        let obj = rd::<usize>(param_1);
        let d2 = u8::from(rd::<f32>(obj + 0x14) != one);
        wr::<u8>(obj, rd::<u8>(obj) | d2);
        wr::<u32>(obj + 0x14, 0x3f80_0000);

        let mut vec3 = [0u32; 4];
        exe::call_va!(
            0x1_412d_c600,
            (rd::<usize>(param_1), vec3.as_mut_ptr() as usize)
        );
        vec3 = [0u32; 4];
        exe::call_va!(
            0x1_412d_c660,
            (rd::<usize>(param_1), vec3.as_mut_ptr() as usize)
        );

        let obj2 = rd::<usize>(param_1);
        let d3 = u8::from(rd::<f32>(obj2 + 0x30) != one);
        wr::<u8>(obj2, rd::<u8>(obj2) | d3);
        wr::<u32>(obj2 + 0x30, 0x3f80_0000);

        init_cvar_param(
            tls0,
            0x1_43ce_ffc8,
            0x1_43cf_0010,
            0x1_43cf_0018,
            0x1_43cf_001c,
            0x1_43cf_0020,
            0x1_433e_fa00,
            0x0e,
            0,
            0,
            0x1_4262_7750,
        );
        let mut zero_f: f32 = 0.0f32;
        let id_vig2 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0018) as usize) as isize;
        let sz_vig2 = rd::<i32>(base + 0x3a6_37b8 + (id_vig2 as usize) * 16) as i64 as u64;
        exe::call_va!(
            0x1_4209_a900,
            (
                id_vig2 as i64 as u64,
                0u32,
                sz_vig2,
                (&mut zero_f as *mut f32) as usize
            )
        );
        wr::<f32>(exe::ptr::<f32>(0x1_43cf_0020) as usize, zero_f);
    }

    if rd::<u8>(rd::<usize>(param_1)) != 0 {
        let lut = rd::<usize>(param_1 + 0x08);
        wr::<u8>(lut + 0x24, 1);
    }
    exe::call_va!(0x1_412d_c6c0, (rd::<usize>(param_1)));

    let use_default_lut = if rd::<u64>(exe::ptr::<u64>(0x1_43a3_3638) as usize) == 0 {
        true
    } else if rd::<u64>(exe::ptr::<u64>(0x1_43a3_35e8) as usize) != 0
        && exe::call_va!(0x1_4225_8d40, () -> u8) == 0
    {
        let lut = rd::<usize>(param_1 + 0x08);
        if rd::<i32>(lut + 0x28) != 2 {
            wr::<i32>(lut + 0x28, 2);
            wr::<u8>(lut + 0x24, 1);
        }
        let w0 = one - rd::<f32>(exe::ptr::<f32>(0x1_4397_ff04) as usize);
        call_set_lut_slot(
            rd::<usize>(param_1 + 0x08),
            0,
            exe::ptr::<u8>(0x1_43a3_3638) as usize,
            w0,
        );
        let w1 = rd::<f32>(exe::ptr::<f32>(0x1_4397_ff04) as usize);
        call_set_lut_slot(
            rd::<usize>(param_1 + 0x08),
            1,
            exe::ptr::<u8>(0x1_43a3_35e8) as usize,
            w1,
        );
        false
    } else if exe::call_va!(0x1_4225_8d40, () -> u8) == 0 {
        let lut = rd::<usize>(param_1 + 0x08);
        if rd::<i32>(lut + 0x28) != 1 {
            wr::<i32>(lut + 0x28, 1);
            wr::<u8>(lut + 0x24, 1);
        }
        call_set_lut_slot(
            rd::<usize>(param_1 + 0x08),
            0,
            exe::ptr::<u8>(0x1_43a3_3638) as usize,
            one,
        );
        false
    } else {
        true
    };

    if use_default_lut {
        let lut = rd::<usize>(param_1 + 0x08);
        if rd::<u64>(exe::ptr::<u64>(0x1_43bb_cff0) as usize) != 0 {
            if rd::<i32>(lut + 0x28) != 1 {
                wr::<i32>(lut + 0x28, 1);
                wr::<u8>(lut + 0x24, 1);
            }
            call_set_lut_slot(
                rd::<usize>(param_1 + 0x08),
                0,
                exe::ptr::<u8>(0x1_43bb_cff0) as usize,
                one,
            );
        } else if rd::<i32>(lut + 0x28) != 0 {
            wr::<i32>(lut + 0x28, 0);
            wr::<u8>(lut + 0x24, 1);
        }
    }

    exe::call_va!(0x1_423c_40e0, (rd::<usize>(param_1 + 0x08)));
    let lut_after = rd::<usize>(param_1 + 0x08);
    let mut rdi_cc = rd::<usize>(rd::<usize>(lut_after));
    if rdi_cc == 0 {
        let def_cc = rd::<usize>(exe::ptr::<usize>(0x1_43bb_cff0) as usize);
        if def_cc != 0 {
            rdi_cc = rd::<usize>(def_cc + 0x1a0);
        }
    }

    init_cvar_param(
        tls0,
        0x1_43ce_ffe8,
        0x1_43ce_feb0,
        0x1_43ce_feb8,
        0x1_43ce_febc,
        0x1_43ce_fec0,
        0x1_433e_f8c8,
        0x19,
        9,
        4,
        0x1_4262_7740,
    );

    let id_cc = rd::<i32>(exe::ptr::<i32>(0x1_43ce_feb8) as usize);
    let mode_cc = rd::<i32>(base + 0x385_8590 + (id_cc as isize as usize) * 4);
    let fcb8_cc = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let srv_cc = select_tex_srv(mode_cc, rdi_cc, fcb8_cc);
    let mut cc_pair: [u64; 2] = [srv_cc, rdi_cc as u64];
    let sz_cc = rd::<i32>(base + 0x3a6_37b8 + (id_cc as isize as usize) * 16) as i64 as u64;
    exe::call_va!(
        0x1_4209_a900,
        (
            id_cc as u32 as u64,
            0u32,
            sz_cc,
            cc_pair.as_mut_ptr() as usize
        )
    );
    wr::<usize>(exe::ptr::<usize>(0x1_43ce_fec8) as usize, rdi_cc);
    let srv_cc2 = select_tex_srv(mode_cc, rdi_cc, fcb8_cc);
    wr::<u64>(exe::ptr::<u64>(0x1_43ce_fec0) as usize, srv_cc2);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142202be0(orig: usize, new: usize) -> Result<u32, String> {
    static mut TRACE_HASH: u64 = 0;
    static mut FLAG_8D40: u8 = 0;
    static mut NEXT_CVAR_ID: i32 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
    }

    unsafe extern "win64" fn mock_4a78(_guard: usize) {
        rec(0x4a78);
    }
    unsafe extern "win64" fn mock_4a0c(guard: usize) {
        wr::<i32>(guard, i32::MIN);
        rec(0x4a0c);
    }
    unsafe extern "win64" fn mock_4d18(_dtor: usize) -> i32 {
        rec(0x4d18);
        0
    }
    unsafe extern "win64" fn mock_ea70(out_id: usize, sv: usize) {
        let len = rd::<u64>(sv + 0x08);
        let id = NEXT_CVAR_ID;
        NEXT_CVAR_ID = NEXT_CVAR_ID.wrapping_add(1) & 0x0f;
        wr::<i32>(out_id, id);
        rec(0xea70 ^ len ^ (id as u64));
    }
    unsafe extern "win64" fn mock_a900(id: u64, edx: u32, sz: u64, ptr: usize) {
        let v0 = rd::<u32>(ptr) as u64;
        rec(0xa900 ^ id ^ (edx as u64) ^ sz ^ v0);
    }
    unsafe extern "win64" fn mock_8d40() -> u8 {
        FLAG_8D40
    }
    unsafe extern "win64" fn mock_2830(_rcx: usize, rdx: u8) {
        let [x0, _, _, x3] = crate::simd_ops::take_xmm_args();
        rec(0x2830 ^ (rdx as u64) ^ (x0 as u32 as u64) ^ ((x3 as u32 as u64) << 32));
    }
    unsafe extern "win64" fn mock_aec0(_rcx: usize) -> i32 {
        256
    }
    unsafe extern "win64" fn mock_ae60(_rcx: usize) -> i32 {
        128
    }
    unsafe extern "win64" fn mock_c600(_obj: usize, vec_ptr: usize) {
        rec(0xc600 ^ rd::<u64>(vec_ptr) ^ (rd::<u32>(vec_ptr + 8) as u64));
    }
    unsafe extern "win64" fn mock_c660(_obj: usize, vec_ptr: usize) {
        rec(0xc660 ^ rd::<u64>(vec_ptr) ^ (rd::<u32>(vec_ptr + 8) as u64));
    }
    unsafe extern "win64" fn mock_c6c0(obj: usize) {
        rec(0xc6c0 ^ (rd::<u8>(obj) as u64));
    }
    unsafe extern "win64" fn mock_5090(_lut: usize, slot: u32, _res: usize) {
        let [_, _, _, x3] = crate::simd_ops::take_xmm_args();
        rec(0x5090 ^ (slot as u64) ^ ((x3 as u32 as u64) << 16));
    }
    unsafe extern "win64" fn mock_40e0(lut: usize) {
        rec(0x40e0 ^ (rd::<u32>(lut + 0x28) as u64));
    }

    let _m01 = crate::difftest::Mock::new(0x1_4183_4a78, mock_4a78 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_4a0c as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_4183_4d18, mock_4d18 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_4225_8d40, mock_8d40 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4220_2830, mock_2830 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4220_aec0, mock_aec0 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4220_ae60, mock_ae60 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_412d_c600, mock_c600 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_412d_c660, mock_c660 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_412d_c6c0, mock_c6c0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_423c_5090, mock_5090 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_423c_40e0, mock_40e0 as *const ())?;

    let tls_arr = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_arr);
    let mut tls_buf = vec![0u8; 0x47000];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    wr::<usize>(tls_arr, tls0);

    let s_cvars = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43ce_fe18) as *mut u8, 0x300);
    let s_37b0 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_37b0) as *mut u8, 0x100);
    let s_8590 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8590) as *mut u8, 0x40);
    let s_fcb8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb8) as *mut u8, 8);
    let s_cff8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43bb_cff8) as *mut u8, 8);
    let s_cff0 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43bb_cff0) as *mut u8, 8);
    let s_0180 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_0180) as *mut u8, 4);
    let s_3638 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_3638) as *mut u8, 8);
    let s_35e8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_35e8) as *mut u8, 8);
    let s_ff04 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff04) as *mut u8, 4);

    static mut FCB8_MEM: [u8; 0xa0] = [0; 0xa0];
    static mut FG_TEX_MEM: [u8; 0xa0] = [0; 0xa0];
    static mut FG_RES_MEM: [u8; 0x1b0] = [0; 0x1b0];
    static mut CC_TEX_MEM: [u8; 0xa0] = [0; 0xa0];
    static mut CC_RES_MEM: [u8; 0x1b0] = [0; 0x1b0];
    static mut OBJ0_MEM: [u8; 0x40] = [0; 0x40];
    static mut LUT_FIRST_PTR: usize = 0;
    static mut OBJ1_MEM: [u8; 0x40] = [0; 0x40];
    static mut PARAM1_MEM: [usize; 2] = [0; 2];

    let guards: [u64; 13] = [
        0x1_43ce_ff78,
        0x1_43cf_00a8,
        0x1_43cf_00ac,
        0x1_43cf_00ec,
        0x1_43ce_fef8,
        0x1_43ce_ff38,
        0x1_43cf_00c8,
        0x1_43ce_fe20,
        0x1_43ce_fea0,
        0x1_43ce_fe18,
        0x1_43ce_fea4,
        0x1_43ce_ffc8,
        0x1_43ce_ffe8,
    ];

    let run = |fn_va: usize, mode: u32| -> (u64, [u8; 0x40], [u8; 0x40], Vec<u8>, Vec<u8>, [i32; 4]) {
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        FLAG_8D40 = if (mode & 1) != 0 { 1 } else { 0 };
        NEXT_CVAR_ID = 0;

        core::ptr::write_bytes(tls0 as *mut u8, 0, 0x47000);
        wr::<i32>(tls0 + 0x74, 0);
        core::ptr::write_bytes(exe::ptr::<u8>(0x1_43ce_fe18) as *mut u8, 0, 0x300);
        for &g in &guards {
            wr::<i32>(exe::ptr::<i32>(g) as usize, -1);
        }

        let tbl_37b0 = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
        let tbl_8590 = exe::ptr::<u8>(0x1_4385_8590) as usize;
        for i in 0..16usize {
            let e = tbl_37b0 + i * 16;
            wr::<i32>(e, (i as i32) * 16);
            wr::<i32>(e + 0x04, 0);
            wr::<i32>(e + 0x08, 4);
            wr::<i32>(e + 0x0c, (i & 3) as i32);
            wr::<i32>(tbl_8590 + i * 4, ((i + mode as usize) % 3) as i32 * 5);
        }
        wr::<i32>(tls0 + 0x463e0, -2);

        let fcb8_p = core::ptr::addr_of_mut!(FCB8_MEM) as usize;
        wr::<u64>(fcb8_p + 0x88, 0x1111_8888);
        wr::<u64>(fcb8_p + 0x90, 0x2222_9090);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, fcb8_p);

        let fg_tex_p = core::ptr::addr_of_mut!(FG_TEX_MEM) as usize;
        wr::<u64>(fg_tex_p + 0x78, 0x3333_7878);
        wr::<u64>(fg_tex_p + 0x90, 0x4444_9090);
        let fg_res_p = core::ptr::addr_of_mut!(FG_RES_MEM) as usize;
        wr::<usize>(fg_res_p + 0x1a0, fg_tex_p);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43bb_cff8) as usize,
            if (mode & 2) != 0 { fg_res_p } else { 0 },
        );

        let cc_tex_p = core::ptr::addr_of_mut!(CC_TEX_MEM) as usize;
        wr::<u64>(cc_tex_p + 0x78, 0x5555_7878);
        wr::<u64>(cc_tex_p + 0x90, 0x6666_9090);
        let cc_res_p = core::ptr::addr_of_mut!(CC_RES_MEM) as usize;
        wr::<usize>(cc_res_p + 0x1a0, cc_tex_p);
        wr::<usize>(exe::ptr::<usize>(0x1_43bb_cff0) as usize, cc_res_p);

        wr::<i32>(
            exe::ptr::<i32>(0x1_43a0_0180) as usize,
            if (mode & 2) != 0 { -7 } else { 13 },
        );
        wr::<u64>(
            exe::ptr::<u64>(0x1_43a3_3638) as usize,
            if (mode & 2) != 0 { 0xdead_3638 } else { 0 },
        );
        wr::<u64>(
            exe::ptr::<u64>(0x1_43a3_35e8) as usize,
            if mode == 2 { 0xdead_35e8 } else { 0 },
        );
        wr::<f32>(exe::ptr::<f32>(0x1_4397_ff04) as usize, 0.35f32);

        OBJ0_MEM = [0; 0x40];
        OBJ1_MEM = [0; 0x40];
        let obj0_p = core::ptr::addr_of_mut!(OBJ0_MEM) as usize;
        let obj1_p = core::ptr::addr_of_mut!(OBJ1_MEM) as usize;
        wr::<f32>(obj0_p + 0x04, 0.5);
        wr::<f32>(obj0_p + 0x08, 0.25);
        wr::<f32>(obj0_p + 0x14, 1.0);
        wr::<f32>(obj0_p + 0x30, 0.75);

        LUT_FIRST_PTR = if (mode & 1) != 0 { cc_tex_p } else { 0 };
        wr::<usize>(obj1_p, core::ptr::addr_of_mut!(LUT_FIRST_PTR) as usize);
        wr::<i32>(obj1_p + 0x28, 99);

        PARAM1_MEM = [obj0_p, obj1_p];
        let p1_p = core::ptr::addr_of_mut!(PARAM1_MEM) as usize;

        let p2: u8 = if (mode & 2) != 0 { 1 } else { 0 };
        let _ = crate::simd_ops::call_va8_xmm(
            fn_va as u64,
            p1_p as u64,
            p2 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 1.5f32.to_bits() as u128, 2.25f32.to_bits() as u128],
        );

        let tls_cbuf = rd::<[u8; 0x100]>(tls0 + 0x63d0);
        let cvar_globals = rd::<[u8; 0x300]>(exe::ptr::<u8>(0x1_43ce_fe18) as usize);
        let cnts = rd::<[i32; 4]>(tls0 + 0x463e0);
        (
            TRACE_HASH,
            OBJ0_MEM,
            OBJ1_MEM,
            tls_cbuf.to_vec(),
            cvar_globals.to_vec(),
            cnts,
        )
    };

    let res = (|| -> Result<u32, String> {
        for mode in 0..4u32 {
            let o = run(orig, mode);
            let n = run(new, mode);
            crate::diff_eq!(o, n, "142202be0 mode={mode}");
        }
        Ok(4)
    })();

    s_ff04.restore();
    s_35e8.restore();
    s_3638.restore();
    s_0180.restore();
    s_cff0.restore();
    s_cff8.restore();
    s_fcb8.restore();
    s_8590.restore();
    s_37b0.restore();
    s_cvars.restore();
    wr::<usize>(tls_arr, saved_tls0);

    res
}
