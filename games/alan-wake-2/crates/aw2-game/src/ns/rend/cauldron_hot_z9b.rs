//! Decompiled `rend` hot functions shard Z9B (`0x142333360..0x14233f590`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn init_static_prop_id(
    tls0: usize,
    guard_va: u64,
    out_va: u64,
    str_va: u64,
    str_len: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let mut view: [u64; 2] = [exe::ptr::<u8>(str_va) as usize as u64, str_len];
            let out_ptr = exe::ptr::<u32>(out_va) as usize;
            exe::call_va!(0x1_4220_ea70, (out_ptr, view.as_mut_ptr() as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn init_static_tweakable(
    tls0: usize,
    guard_va: u64,
    twk_va: u64,
    vals_va: u64,
    dtor_va: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let twk = exe::ptr::<u8>(twk_va) as usize;
            wr::<u32>(twk + 0x08, 2);
            wr::<u8>(twk + 0x0c, 0);
            wr::<usize>(twk, exe::ptr::<u8>(0x1_4307_9e38) as usize);
            wr::<[u8; 16]>(
                twk + 0x10,
                rd::<[u8; 16]>(exe::ptr::<u8>(vals_va) as usize),
            );
            let cnt_ptr = exe::ptr::<i32>(0x1_43a0_01ac) as usize;
            let cnt = rd::<i32>(cnt_ptr);
            if cnt < 0x1000 {
                let arr_ptr = exe::ptr::<usize>(0x1_43a0_0290) as usize;
                wr::<usize>(arr_ptr.wrapping_add((cnt as isize as usize) * 8), twk);
                wr::<i32>(cnt_ptr, cnt.wrapping_add(1));
            }
            let dtor = exe::ptr::<u8>(dtor_va) as usize;
            let _ = exe::call_va!(0x1_4183_4d18, (dtor) -> i32);
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn find_prop_entry(map: usize, key_va: u64) -> usize {
    let keys = rd::<usize>(map);
    let count = rd::<u32>(map + 0x08) as usize;
    if count == 0 {
        return 0;
    }
    let target = rd::<u32>(exe::ptr::<u32>(key_va) as usize);
    for i in 0..count {
        if rd::<u32>(keys.wrapping_add(i * 4)) == target {
            let vals = rd::<usize>(map + 0x10);
            return vals.wrapping_add(i * 0x18);
        }
    }
    0
}

#[inline(always)]
fn quantize_u8(val: f32, scale: f32) -> u32 {
    let prod = val * scale;
    let rounded = f32::from_bits(crate::simd_ops::vroundss(0, prod.to_bits(), 0xc) as u32);
    let iv = crate::simd_ops::cvt_f32_i32(rounded) as i32;
    if iv < 0 {
        0
    } else if iv > 0xff {
        0xff
    } else {
        iv as u32
    }
}

#[inline(always)]
unsafe fn clear_inplace_string(s_ptr: usize) {
    let b3 = rd::<u8>(s_ptr + 0x03) as u32;
    let mut data_ptr = s_ptr.wrapping_add(0x08);
    wr::<u32>(s_ptr + 0x04, 0);
    let inline_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap_bits = (rd::<u32>(s_ptr) & 0x00ff_ffff) << 3;
    if cap_bits > inline_bits {
        data_ptr = rd::<usize>(data_ptr);
    }
    wr::<u8>(data_ptr, 0);
}

/// `rend_142333360`
/// Original: 0x142333360 (5099 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142333360(param_1: usize, param_2: usize) {
    let slot_idx = rd::<u16>(param_2 + 0x20) as usize;
    let flags = crate::simd_ops::atomic_xchg_u8((param_2 + 0x38) as u64, 0);

    if (flags & 2) != 0 {
        let off_80 = slot_idx.wrapping_mul(0x80);
        let dst_80 = rd::<usize>(param_1 + 0x38).wrapping_add(off_80);
        wr::<[u64; 16]>(dst_80, [0u64; 16]);

        let p18 = rd::<usize>(param_2 + 0x18);
        if p18 != 0 {
            let p70 = rd::<usize>(p18 + 0x70);
            if p70 != 0 {
                let mut str_buf = [0u64; 34];
                let sbp = str_buf.as_mut_ptr() as usize;
                wr::<u64>(sbp, 0x2000_0020);
                wr::<u8>(sbp + 0x08, 0);
                let dst_arg = rd::<usize>(param_1 + 0x38).wrapping_add(off_80);
                let _ = exe::call_va!(
                    0x1_4233_f390,
                    (p70, param_2, dst_arg, 0u64, sbp) -> u64
                );
                exe::call_va!(0x1_4240_ab10, (sbp));
            }
        }

        let word_addr = param_1.wrapping_add((slot_idx >> 6) * 8 + 0x2078);
        let mask = 1u64 << (slot_idx & 0x3f);
        let mut cur = rd::<u64>(word_addr);
        loop {
            let (ok, old) =
                crate::simd_ops::atomic_cmpxchg_u64(word_addr as u64, cur, cur | mask);
            if ok {
                break;
            }
            cur = old;
        }
    }

    if (flags & 1) == 0 {
        return;
    }

    let base_0 = rd::<usize>(param_1);
    let rsi_entry = rd::<usize>(param_1 + 0x10).wrapping_add(slot_idx.wrapping_mul(0x38));
    let rbx_map = rd::<usize>(rd::<usize>(param_2));
    let tls0 = rd::<usize>(exe::tls_pointer());

    init_static_prop_id(tls0, 0x1_43cf_9d50, 0x1_43cf_9cac, 0x1_433f_d8c8, 0x12);
    init_static_prop_id(tls0, 0x1_43cf_9d48, 0x1_43cf_9e1c, 0x1_433f_d898, 0x11);
    init_static_prop_id(tls0, 0x1_43cf_9c84, 0x1_43cf_9d4c, 0x1_433f_d8b0, 0x16);
    init_static_prop_id(tls0, 0x1_43cf_9cfc, 0x1_43cf_9e18, 0x1_433f_d878, 0x0e);
    init_static_prop_id(tls0, 0x1_43cf_9cbc, 0x1_43cf_9dc8, 0x1_433f_d888, 0x0c);
    init_static_prop_id(tls0, 0x1_43cf_9d88, 0x1_43cf_9d7c, 0x1_433f_d860, 0x07);

    let ent_spec = find_prop_entry(rbx_map, 0x1_43cf_9cac);
    let (spec_xy, spec_z) = if ent_spec != 0 {
        (rd::<u64>(ent_spec + 0x08), rd::<u32>(ent_spec + 0x10))
    } else {
        (0u64, 0u32)
    };

    let ent_tr = find_prop_entry(rbx_map, 0x1_43cf_9e1c);
    let trans_xy = if ent_tr != 0 {
        rd::<u64>(ent_tr + 0x08)
    } else {
        let ent_tdr = find_prop_entry(rbx_map, 0x1_43cf_9d4c);
        if ent_tdr != 0 {
            rd::<u64>(ent_tdr + 0x08)
        } else {
            0u64
        }
    };

    let ent_sf = find_prop_entry(rbx_map, 0x1_43cf_9e18);
    let (sf_xy, sf_z) = if ent_sf != 0 {
        (rd::<u64>(ent_sf + 0x08), rd::<u32>(ent_sf + 0x10))
    } else {
        (0u64, 0u32)
    };

    let ent_sw = find_prop_entry(rbx_map, 0x1_43cf_9dc8);
    let sw_bits = if ent_sw != 0 {
        rd::<u32>(ent_sw + 0x08)
    } else {
        0u32
    };

    let ent_sc = find_prop_entry(rbx_map, 0x1_43cf_9d7c);
    let scatter_flag = if ent_sc != 0 {
        rd::<u8>(ent_sc + 0x08)
    } else {
        0u8
    };

    let r12_arg1 = rd::<usize>(exe::ptr::<usize>(0x1_4397_feb0) as usize);
    let mode_82_a = rd::<u8>(rd::<usize>(param_2) + 0x82);

    init_static_tweakable(
        tls0,
        0x1_43cf_9e24,
        0x1_4388_1c58,
        0x1_4342_8130,
        0x1_4263_7cf0,
    );
    init_static_tweakable(
        tls0,
        0x1_43cf_9cc0,
        0x1_4388_1c78,
        0x1_4342_8070,
        0x1_4263_7c60,
    );

    let xmm3_val: f32 = match mode_82_a {
        0 | 2 => rd::<f32>(exe::ptr::<f32>(0x1_4388_1c68) as usize),
        1 => rd::<f32>(exe::ptr::<f32>(0x1_4388_1c88) as usize),
        _ => 0.0f32,
    };

    let mut falloff_vec: [u32; 3] = if scatter_flag != 0 {
        [sf_xy as u32, (sf_xy >> 32) as u32, sf_z]
    } else {
        [0, 0, 0]
    };
    let xmm2_val = f32::from_bits(sw_bits);
    let (i_var9_u64, _) = crate::simd_ops::call_va8_xmm(
        0x1_4221_0a10,
        r12_arg1 as u64,
        falloff_vec.as_mut_ptr() as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm2_val.to_bits() as u128, xmm3_val.to_bits() as u128],
    );
    let i_var9 = i_var9_u64 as u32;

    let mode_82_b = rd::<u8>(rd::<usize>(param_2) + 0x82);
    let r9d: u32 = match mode_82_b {
        1 => 0x100,
        2 => 0x200,
        _ => 0,
    };

    let scale_255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);
    let spec_r = quantize_u8(f32::from_bits(spec_xy as u32), scale_255);
    let spec_g = quantize_u8(f32::from_bits((spec_xy >> 32) as u32), scale_255);
    let spec_b = quantize_u8(f32::from_bits(spec_z), scale_255);
    let word0 = ((((spec_b | r9d) << 8) | spec_g) << 8) | spec_r;
    wr::<u32>(base_0.wrapping_add(slot_idx * 8), word0);

    let trans_hi = quantize_u8(f32::from_bits((trans_xy >> 32) as u32), scale_255);
    let trans_lo = quantize_u8(f32::from_bits(trans_xy as u32), scale_255);
    let word1 = (((i_var9 << 8) | trans_hi) << 8) | trans_lo;
    wr::<u32>(base_0.wrapping_add(slot_idx * 8 + 0x04), word1);

    if rd::<usize>(param_2 + 0x18) == 0 {
        return;
    }

    init_static_prop_id(tls0, 0x1_43cf_9d84, 0x1_43cf_9e10, 0x1_433d_6e80, 0x0f);
    init_static_prop_id(tls0, 0x1_43cf_9de8, 0x1_43cf_9e0c, 0x1_433f_a670, 0x12);
    init_static_prop_id(tls0, 0x1_43cf_9e28, 0x1_43cf_9d80, 0x1_433f_a688, 0x11);
    init_static_prop_id(tls0, 0x1_43cf_9cb4, 0x1_43cf_9cf8, 0x1_433f_d868, 0x0f);
    init_static_prop_id(tls0, 0x1_43cf_9d20, 0x1_43cf_9e60, 0x1_433f_d5c8, 0x0f);
    init_static_prop_id(tls0, 0x1_43cf_9cf0, 0x1_43cf_9d28, 0x1_433f_d5d8, 0x11);
    init_static_prop_id(tls0, 0x1_43cf_9e2c, 0x1_43cf_9e14, 0x1_433f_d5a0, 0x13);
    init_static_prop_id(tls0, 0x1_43cf_9cb0, 0x1_43cf_9dec, 0x1_433f_d5b8, 0x0e);

    let ent_cm = find_prop_entry(rbx_map, 0x1_43cf_9e10);
    let cm = if ent_cm != 0 {
        rd::<[f32; 4]>(ent_cm + 0x08)
    } else {
        rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_8240) as usize)
    };
    let max_23 = if cm[2] > cm[3] { cm[2] } else { cm[3] };
    let max_01 = if cm[0] > cm[1] { cm[0] } else { cm[1] };
    let max_c = if max_01 > max_23 { max_01 } else { max_23 };
    wr::<f32>(rsi_entry + 0x04, max_c);

    let inv_max = if max_c == 0.0f32 {
        0.0f32
    } else {
        rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize) / max_c
    };
    let cm_r = quantize_u8(cm[0] * inv_max, scale_255);
    let cm_g = quantize_u8(cm[1] * inv_max, scale_255);
    let cm_b = quantize_u8(cm[2] * inv_max, scale_255);
    let cm_a = quantize_u8(cm[3] * inv_max, scale_255);
    wr::<u32>(
        rsi_entry,
        (((((cm_a << 8) | cm_b) << 8) | cm_g) << 8) | cm_r,
    );

    let ent_em = find_prop_entry(rbx_map, 0x1_43cf_9e0c);
    let (em_xy, em_z) = if ent_em != 0 {
        (rd::<u64>(ent_em + 0x08), rd::<u32>(ent_em + 0x10))
    } else {
        (0u64, 0u32)
    };
    let em_r = quantize_u8(f32::from_bits(em_xy as u32), scale_255);
    let em_g = quantize_u8(f32::from_bits((em_xy >> 32) as u32), scale_255);
    let em_b = quantize_u8(f32::from_bits(em_z), scale_255);
    wr::<u32>(
        rsi_entry + 0x08,
        ((((em_b | 0xffff_ff00) << 8) | em_g) << 8) | em_r,
    );

    let ent_ei = find_prop_entry(rbx_map, 0x1_43cf_9d80);
    let ei_bits = if ent_ei != 0 {
        rd::<u32>(ent_ei + 0x08)
    } else {
        0u32
    };
    wr::<u32>(rsi_entry + 0x0c, ei_bits);

    let ent_sr = find_prop_entry(rbx_map, 0x1_43cf_9cf8);
    let sr_u64 = if ent_sr != 0 {
        rd::<u64>(ent_sr + 0x08)
    } else {
        0u64
    };
    wr::<u32>(rsi_entry + 0x10, sr_u64 as u32);
    wr::<u32>(rsi_entry + 0x14, (sr_u64 >> 32) as u32);

    let ent_sub = find_prop_entry(rbx_map, 0x1_43cf_9e60);
    let sub_u64 = if ent_sub != 0 {
        rd::<u64>(ent_sub + 0x08)
    } else {
        let ent_swr = find_prop_entry(rbx_map, 0x1_43cf_9d28);
        if ent_swr != 0 {
            rd::<u64>(ent_swr + 0x08)
        } else {
            0u64
        }
    };
    wr::<u32>(rsi_entry + 0x18, sub_u64 as u32);
    wr::<u32>(rsi_entry + 0x1c, (sub_u64 >> 32) as u32);

    let ent_ssr = find_prop_entry(rbx_map, 0x1_43cf_9e14);
    let ssr_u64 = if ent_ssr != 0 {
        rd::<u64>(ent_ssr + 0x08)
    } else {
        0u64
    };
    wr::<u32>(rsi_entry + 0x20, ssr_u64 as u32);
    wr::<u32>(rsi_entry + 0x24, (ssr_u64 >> 32) as u32);

    let byte_83 = rd::<u8>(rd::<usize>(param_2) + 0x83) as u32;
    wr::<u32>(rsi_entry + 0x28, byte_83);

    let ent_of = find_prop_entry(rbx_map, 0x1_43cf_9dec);
    if ent_of != 0 {
        wr::<u32>(rsi_entry + 0x2c, rd::<u8>(ent_of + 0x08) as u32);
    }

    let mut out_len: u64 = 0;
    let mut rbx_len: u64 = 0;
    let mut buf_1088 = [0u64; 522];
    let p18_b = rd::<usize>(param_2 + 0x18);
    let p70_b = rd::<usize>(p18_b + 0x70);
    if p70_b != 0 {
        let mut err_str = [0u64; 130];
        let esp = err_str.as_mut_ptr() as usize;
        wr::<u32>(esp, 0x8000_0080);
        wr::<u32>(esp + 0x04, 0);
        wr::<u8>(esp + 0x08, 0);
        let ok = exe::call_va!(
            0x1_4233_f590,
            (
                p70_b,
                param_2,
                buf_1088.as_mut_ptr() as usize,
                0u64,
                (&mut out_len as *mut u64) as usize,
                esp
            ) -> u8
        );
        rbx_len = if ok != 0 { out_len } else { 0 };
        exe::call_va!(0x1_4240_ab10, (esp));
    }

    if rbx_len != rd::<u32>(rsi_entry + 0x34) as u64 {
        wr::<u64>(rsi_entry + 0x30, 0);
        wr::<u8>(param_1 + 0x30, 1);
    }
    if rd::<u32>(rsi_entry + 0x34) == 0 && rbx_len != 0 {
        let cur_off = rd::<u32>(param_1 + 0x28);
        wr::<u32>(rsi_entry + 0x30, cur_off);
        wr::<u32>(rsi_entry + 0x34, rbx_len as u32);
        let needed_cap =
            ((cur_off as u64).wrapping_add(0x1fff).wrapping_add(rbx_len)) & !0x1fffu64;
        let cur_cap = rd::<u32>(param_1 + 0x2c) as u64;
        if needed_cap > cur_cap {
            exe::call_va!(0x1_402d_7910, (param_1 + 0x20, needed_cap));
        }
        let new_size = (rd::<u32>(param_1 + 0x28) as u64).wrapping_add(rbx_len);
        exe::call_va!(0x1_4028_e490, (param_1 + 0x20, new_size));
    }

    let dst_ptr = rd::<usize>(param_1 + 0x20).wrapping_add(rd::<u32>(rsi_entry + 0x30) as usize);
    let _ = exe::call_va!(
        0x1_4183_acbc,
        (dst_ptr, buf_1088.as_ptr() as usize, rbx_len) -> usize
    );
}

/// `rend_14233f390`
/// Original: 0x14233f390 (503 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233f390(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: u64,
    param_5: usize,
) -> u64 {
    let l2 = rd::<usize>(param_1);
    let slot_m30 = rd::<usize>(l2 + 0x08).wrapping_sub(0x30);
    let nil_obj = exe::ptr::<u8>(0x1_4298_68d0) as usize;
    if slot_m30 != nil_obj && rd::<u32>(slot_m30 + 0x10) == 0 {
        return (slot_m30 as u64) & !0xffu64;
    }

    wr::<u64>(
        exe::ptr::<u64>(0x1_43a3_6fa8) as usize,
        rd::<u64>(param_2),
    );
    let b1 = rd::<u8>(l2 + 0x01);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6ed8) as usize, param_2);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_71a8) as usize, param_3);
    wr::<u32>(exe::ptr::<u32>(0x1_43a2_b40c) as usize, 0x10);

    if (b1 & 4) != 0 {
        let g = rd::<usize>(l2 + 0x18);
        wr::<u8>(l2 + 0x01, b1 & !4);
        wr::<u64>(l2 + 0x68, rd::<u64>(g + 0x30));
        wr::<usize>(g + 0x30, l2);
    }

    let top = rd::<usize>(l2 + 0x08);
    wr::<[u8; 16]>(top, rd::<[u8; 16]>(top - 0x30));
    wr::<u64>(top + 0x10, rd::<u64>(top - 0x20));
    wr::<usize>(l2 + 0x08, top.wrapping_add(0x18));

    let l2_call = rd::<usize>(param_1);
    let func_slot = rd::<usize>(l2_call + 0x08).wrapping_sub(0x18);
    let mut call_data: [u64; 2] = [func_slot as u64, 0];
    let cb = exe::ptr::<u8>(0x1_4211_6cd0) as usize;
    let rel_off = func_slot.wrapping_sub(rd::<usize>(l2_call + 0x30)) as u64;
    let status = exe::call_va!(
        0x1_4210_dbb0,
        (
            l2_call,
            cb,
            call_data.as_mut_ptr() as usize,
            rel_off,
            0u64
        ) -> i32
    );

    let ret: u64 = if status == 0 {
        1
    } else {
        let rbx = rd::<usize>(param_1);
        let mut rdx_err = rd::<usize>(rbx + 0x08).wrapping_sub(0x18);
        let mut has_str = true;
        if rd::<u32>(rdx_err + 0x10) != 5 {
            let b1_err = rd::<u8>(rbx + 0x01);
            if (b1_err & 4) != 0 {
                let g = rd::<usize>(rbx + 0x18);
                wr::<u8>(rbx + 0x01, b1_err & !4);
                wr::<u64>(rbx + 0x68, rd::<u64>(g + 0x30));
                wr::<usize>(g + 0x30, rbx);
            }
            let conv = exe::call_va!(0x1_420e_cc90, (rbx) -> i32);
            if conv != 0 {
                let g = rd::<usize>(rbx + 0x18);
                if rd::<u64>(g + 0x48) >= rd::<u64>(g + 0x40) {
                    exe::call_va!(0x1_4210_c770, (rbx));
                }
                rdx_err = rd::<usize>(rbx + 0x08).wrapping_sub(0x18);
            } else {
                has_str = false;
            }
        }

        if has_str {
            let str_ptr = rd::<usize>(rdx_err).wrapping_add(0x18);
            if str_ptr != 0 {
                let mut len: u64 = 0;
                while rd::<u8>(str_ptr.wrapping_add(len as usize)) != 0 {
                    len = len.wrapping_add(1);
                }
                call_data[0] = str_ptr as u64;
                call_data[1] = len;
                exe::call_va!(
                    0x1_4240_9b50,
                    (param_5, call_data.as_mut_ptr() as usize, 0u64, len)
                );
            } else {
                clear_inplace_string(param_5);
            }
        } else {
            clear_inplace_string(param_5);
        }

        let rcx_end = rd::<usize>(param_1);
        wr::<usize>(rcx_end + 0x08, rd::<usize>(rcx_end + 0x08).wrapping_sub(0x18));
        0
    };

    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6ed8) as usize, 0);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_6fa8) as usize, 0);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_71a8) as usize, 0);
    wr::<u32>(exe::ptr::<u32>(0x1_43a2_b40c) as usize, 0);
    ret
}

/// `rend_14233f590`
/// Original: 0x14233f590 (552 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233f590(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: u64,
    param_5: usize,
    param_6: usize,
) -> u64 {
    let l2 = rd::<usize>(param_1);
    let slot_m18 = rd::<usize>(l2 + 0x08).wrapping_sub(0x18);
    let nil_obj = exe::ptr::<u8>(0x1_4298_68d0) as usize;
    if slot_m18 != nil_obj && rd::<u32>(slot_m18 + 0x10) == 0 {
        return (slot_m18 as u64) & !0xffu64;
    }

    wr::<u64>(
        exe::ptr::<u64>(0x1_43a3_6fa8) as usize,
        rd::<u64>(param_2),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_7038) as usize,
        param_1.wrapping_add(0x20),
    );
    let b1 = rd::<u8>(l2 + 0x01);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6ed8) as usize, param_2);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6df8) as usize, param_3);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_70e8) as usize, 0x1000);

    if (b1 & 4) != 0 {
        let g = rd::<usize>(l2 + 0x18);
        wr::<u8>(l2 + 0x01, b1 & !4);
        wr::<u64>(l2 + 0x68, rd::<u64>(g + 0x30));
        wr::<usize>(g + 0x30, l2);
    }

    let top = rd::<usize>(l2 + 0x08);
    wr::<[u8; 16]>(top, rd::<[u8; 16]>(top - 0x18));
    wr::<u64>(top + 0x10, rd::<u64>(top - 0x08));
    wr::<usize>(l2 + 0x08, top.wrapping_add(0x18));

    let l2_call = rd::<usize>(param_1);
    let func_slot = rd::<usize>(l2_call + 0x08).wrapping_sub(0x18);
    let mut call_data: [u64; 2] = [func_slot as u64, 0];
    let cb = exe::ptr::<u8>(0x1_4211_6cd0) as usize;
    let rel_off = func_slot.wrapping_sub(rd::<usize>(l2_call + 0x30)) as u64;
    let status = exe::call_va!(
        0x1_4210_dbb0,
        (
            l2_call,
            cb,
            call_data.as_mut_ptr() as usize,
            rel_off,
            0u64
        ) -> i32
    );

    let ret: u64 = if status == 0 {
        1
    } else {
        let rbx = rd::<usize>(param_1);
        let mut rdx_err = rd::<usize>(rbx + 0x08).wrapping_sub(0x18);
        let mut has_str = true;
        if rd::<u32>(rdx_err + 0x10) != 5 {
            let b1_err = rd::<u8>(rbx + 0x01);
            if (b1_err & 4) != 0 {
                let g = rd::<usize>(rbx + 0x18);
                wr::<u8>(rbx + 0x01, b1_err & !4);
                wr::<u64>(rbx + 0x68, rd::<u64>(g + 0x30));
                wr::<usize>(g + 0x30, rbx);
            }
            let conv = exe::call_va!(0x1_420e_cc90, (rbx) -> i32);
            if conv != 0 {
                let g = rd::<usize>(rbx + 0x18);
                if rd::<u64>(g + 0x48) >= rd::<u64>(g + 0x40) {
                    exe::call_va!(0x1_4210_c770, (rbx));
                }
                rdx_err = rd::<usize>(rbx + 0x08).wrapping_sub(0x18);
            } else {
                has_str = false;
            }
        }

        if has_str {
            let str_ptr = rd::<usize>(rdx_err).wrapping_add(0x18);
            if str_ptr != 0 {
                let mut len: u64 = 0;
                while rd::<u8>(str_ptr.wrapping_add(len as usize)) != 0 {
                    len = len.wrapping_add(1);
                }
                call_data[0] = str_ptr as u64;
                call_data[1] = len;
                exe::call_va!(
                    0x1_4240_9b50,
                    (param_6, call_data.as_mut_ptr() as usize, 0u64, len)
                );
            } else {
                clear_inplace_string(param_6);
            }
        } else {
            clear_inplace_string(param_6);
        }

        let rcx_end = rd::<usize>(param_1);
        wr::<usize>(rcx_end + 0x08, rd::<usize>(rcx_end + 0x08).wrapping_sub(0x18));
        0
    };

    let bytes_written =
        rd::<u64>(exe::ptr::<u64>(0x1_43a3_6df8) as usize).wrapping_sub(param_3 as u64);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6ed8) as usize, 0);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_6fa8) as usize, 0);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_6df8) as usize, 0);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_70e8) as usize, 0);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_7038) as usize, 0);
    wr::<u64>(param_5, bytes_written);
    ret
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_142333360(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_CALLS: (u32, u32, u64, u64, u32) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_0a10(
        _rcx: usize,
        vec_ptr: usize,
        xmm2: f32,
        xmm3: f32,
    ) -> u32 {
        let v0 = rd::<f32>(vec_ptr);
        let v1 = rd::<f32>(vec_ptr + 4);
        let v2 = rd::<f32>(vec_ptr + 8);
        let sum = ((v0 + v1 + v2 + xmm2 + xmm3) * 100.0) as u32;
        TRACE_CALLS.0 = sum;
        0x1a2b
    }

    unsafe extern "win64" fn mock_f390(
        _p1: usize,
        _p2: usize,
        dst_80: usize,
        _p4: u64,
        _sbp: usize,
    ) -> u64 {
        wr::<u64>(dst_80, 0x1122_3344_5566_7788);
        TRACE_CALLS.1 += 1;
        1
    }

    unsafe extern "win64" fn mock_f590(
        _p1: usize,
        _p2: usize,
        buf_ptr: usize,
        _p4: u64,
        out_len_ptr: usize,
        _esp: usize,
    ) -> u8 {
        for i in 0..16usize {
            wr::<u8>(buf_ptr + i, (i as u8).wrapping_mul(7).wrapping_add(3));
        }
        wr::<u64>(out_len_ptr, 16);
        1
    }

    unsafe extern "win64" fn mock_ab10(_rcx: usize) {
        TRACE_CALLS.4 += 1;
    }

    unsafe extern "win64" fn mock_7910(_vec: usize, cap: u64) {
        TRACE_CALLS.2 = cap;
    }

    unsafe extern "win64" fn mock_e490(_vec: usize, new_sz: u64) {
        TRACE_CALLS.3 = new_sz;
    }

    unsafe extern "win64" fn mock_memcpy(dst: usize, src: usize, len: usize) -> usize {
        for i in 0..len {
            wr::<u8>(dst + i, rd::<u8>(src + i));
        }
        dst
    }

    let _m1 = crate::difftest::Mock::new(0x1_4221_0a10, mock_0a10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4233_f390, mock_f390 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4233_f590, mock_f590 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_402d_7910, mock_7910 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4028_e490, mock_e490 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4183_acbc, mock_memcpy as *const ())?;

    let guard_vas: [u64; 16] = [
        0x1_43cf_9d50,
        0x1_43cf_9d48,
        0x1_43cf_9c84,
        0x1_43cf_9cfc,
        0x1_43cf_9cbc,
        0x1_43cf_9d88,
        0x1_43cf_9e24,
        0x1_43cf_9cc0,
        0x1_43cf_9d84,
        0x1_43cf_9de8,
        0x1_43cf_9e28,
        0x1_43cf_9cb4,
        0x1_43cf_9d20,
        0x1_43cf_9cf0,
        0x1_43cf_9e2c,
        0x1_43cf_9cb0,
    ];
    let prop_id_vas: [u64; 14] = [
        0x1_43cf_9cac,
        0x1_43cf_9e1c,
        0x1_43cf_9d4c,
        0x1_43cf_9e18,
        0x1_43cf_9dc8,
        0x1_43cf_9d7c,
        0x1_43cf_9e10,
        0x1_43cf_9e0c,
        0x1_43cf_9d80,
        0x1_43cf_9cf8,
        0x1_43cf_9e60,
        0x1_43cf_9d28,
        0x1_43cf_9e14,
        0x1_43cf_9dec,
    ];

    let mut saved_guards = [0i32; 16];
    for (i, &va) in guard_vas.iter().enumerate() {
        let p = exe::ptr::<i32>(va) as usize;
        saved_guards[i] = rd::<i32>(p);
        wr::<i32>(p, i32::MIN);
    }
    let mut saved_ids = [0u32; 14];
    for (i, &va) in prop_id_vas.iter().enumerate() {
        let p = exe::ptr::<u32>(va) as usize;
        saved_ids[i] = rd::<u32>(p);
        wr::<u32>(p, (100 + i) as u32);
    }
    let twk1_p = exe::ptr::<f32>(0x1_4388_1c68) as usize;
    let twk2_p = exe::ptr::<f32>(0x1_4388_1c88) as usize;
    let saved_twk1 = rd::<f32>(twk1_p);
    let saved_twk2 = rd::<f32>(twk2_p);
    wr::<f32>(twk1_p, 0.5);
    wr::<f32>(twk2_p, 0.15);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let run = |f: F, flags: u8, mode_82: u8| -> (
        [u32; 4],
        [u32; 14],
        [u64; 16],
        [u8; 32],
        u64,
        u8,
        u8,
        (u32, u32, u64, u64, u32),
    ) {
        TRACE_CALLS = (0, 0, 0, 0, 0);

        let mut tls_blk = [0u8; 0x80];
        let tp = tls_blk.as_mut_ptr() as usize;
        wr::<i32>(tp + 0x74, 0);
        wr::<usize>(tls_slot, tp);

        let keys: [u32; 12] = [100, 102, 103, 104, 105, 106, 107, 108, 109, 111, 112, 113];
        let mut vals = [0u8; 12 * 0x18];
        let vp = vals.as_mut_ptr() as usize;
        // 100: SpecularMultiplier (vec3)
        wr::<f32>(vp + 0 * 0x18 + 0x08, 0.25);
        wr::<f32>(vp + 0 * 0x18 + 0x0c, 0.50);
        wr::<f32>(vp + 0 * 0x18 + 0x10, 0.75);
        // 102: TranslucencyDepthRange (vec2)
        wr::<f32>(vp + 1 * 0x18 + 0x08, 0.20);
        wr::<f32>(vp + 1 * 0x18 + 0x0c, 0.80);
        // 103: ScatterFalloff (vec3)
        wr::<f32>(vp + 2 * 0x18 + 0x08, 0.10);
        wr::<f32>(vp + 2 * 0x18 + 0x0c, 0.30);
        wr::<f32>(vp + 2 * 0x18 + 0x10, 0.60);
        // 104: ScatterWidth (f32)
        wr::<f32>(vp + 3 * 0x18 + 0x08, 0.40);
        // 105: Scatter (u8)
        wr::<u8>(vp + 4 * 0x18 + 0x08, 1);
        // 106: ColorMultiplier (vec4)
        wr::<f32>(vp + 5 * 0x18 + 0x08, 0.5);
        wr::<f32>(vp + 5 * 0x18 + 0x0c, 1.0);
        wr::<f32>(vp + 5 * 0x18 + 0x10, 2.0);
        wr::<f32>(vp + 5 * 0x18 + 0x14, 0.25);
        // 107: EmissionMultiplier (vec3)
        wr::<f32>(vp + 6 * 0x18 + 0x08, 0.1);
        wr::<f32>(vp + 6 * 0x18 + 0x0c, 0.5);
        wr::<f32>(vp + 6 * 0x18 + 0x10, 0.9);
        // 108: EmissionIntensity (f32)
        wr::<f32>(vp + 7 * 0x18 + 0x08, 3.5);
        // 109: SmoothnessRange (vec2)
        wr::<f32>(vp + 8 * 0x18 + 0x08, 0.15);
        wr::<f32>(vp + 8 * 0x18 + 0x0c, 0.85);
        // 111: ScatterWidthRange (vec2)
        wr::<f32>(vp + 9 * 0x18 + 0x08, 0.05);
        wr::<f32>(vp + 9 * 0x18 + 0x0c, 0.95);
        // 112: ShadowSoftnessRange (vec2)
        wr::<f32>(vp + 10 * 0x18 + 0x08, 0.12);
        wr::<f32>(vp + 10 * 0x18 + 0x0c, 0.34);
        // 113: OpacityFadeout (u8)
        wr::<u8>(vp + 11 * 0x18 + 0x08, 7);

        let mut prop_map = [0u64; 4];
        let pmp = prop_map.as_mut_ptr() as usize;
        wr::<usize>(pmp, keys.as_ptr() as usize);
        wr::<u32>(pmp + 0x08, 12);
        wr::<usize>(pmp + 0x10, vp);

        let mut inner_0 = [0u8; 0x90];
        let i0p = inner_0.as_mut_ptr() as usize;
        wr::<usize>(i0p, pmp);
        wr::<u8>(i0p + 0x82, mode_82);
        wr::<u8>(i0p + 0x83, 0x2a);

        let mut obj_18 = [0u8; 0x80];
        let o18p = obj_18.as_mut_ptr() as usize;
        wr::<usize>(o18p + 0x70, 0xdead_beef);

        let mut p2 = [0u8; 0x40];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, i0p);
        wr::<usize>(p2p + 0x18, o18p);
        wr::<u16>(p2p + 0x20, 1);
        wr::<u8>(p2p + 0x38, flags);

        let mut arr_0 = [0u32; 4];
        let mut arr_10 = [0u32; 28];
        let mut arr_20 = [0u8; 32];
        let mut arr_38 = [0xaa_aa_aa_aa_aa_aa_aa_aau64; 32];

        let mut p1 = vec![0u8; 0x2100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, arr_0.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x10, arr_10.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x20, arr_20.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x28, 4);
        wr::<u32>(p1p + 0x2c, 0x1000);
        wr::<usize>(p1p + 0x38, arr_38.as_mut_ptr() as usize);

        f(p1p, p2p);
        wr::<usize>(tls_slot, saved_tls0);

        let mut slot1_10 = [0u32; 14];
        slot1_10.copy_from_slice(&arr_10[14..28]);
        let mut slot1_38 = [0u64; 16];
        slot1_38.copy_from_slice(&arr_38[16..32]);

        (
            arr_0,
            slot1_10,
            slot1_38,
            arr_20,
            rd::<u64>(p1p + 0x2078),
            rd::<u8>(p1p + 0x30),
            rd::<u8>(p2p + 0x38),
            TRACE_CALLS,
        )
    };

    for (flags, mode) in [(2u8, 0u8), (1, 1), (3, 2)] {
        let o = run(f_o, flags, mode);
        let n = run(f_n, flags, mode);
        if o != n {
            for (i, &va) in guard_vas.iter().enumerate() {
                wr::<i32>(exe::ptr::<i32>(va) as usize, saved_guards[i]);
            }
            for (i, &va) in prop_id_vas.iter().enumerate() {
                wr::<u32>(exe::ptr::<u32>(va) as usize, saved_ids[i]);
            }
            wr::<f32>(twk1_p, saved_twk1);
            wr::<f32>(twk2_p, saved_twk2);
        }
        crate::diff_eq!(o, n, "142333360 flags={} mode={}", flags, mode);
    }

    for (i, &va) in guard_vas.iter().enumerate() {
        wr::<i32>(exe::ptr::<i32>(va) as usize, saved_guards[i]);
    }
    for (i, &va) in prop_id_vas.iter().enumerate() {
        wr::<u32>(exe::ptr::<u32>(va) as usize, saved_ids[i]);
    }
    wr::<f32>(twk1_p, saved_twk1);
    wr::<f32>(twk2_p, saved_twk2);
    Ok(3)
}

pub unsafe fn diff_rend_14233f390(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG_390: (i32, u64, u64, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_dbb0(
        _l2: usize,
        _cb: usize,
        _cd: usize,
        rel_off: u64,
        _arg5: u64,
    ) -> i32 {
        CALL_LOG_390.1 = rel_off;
        CALL_LOG_390.0
    }

    unsafe extern "win64" fn mock_cc90(_l2: usize) -> i32 {
        CALL_LOG_390.3 += 1;
        1
    }

    unsafe extern "win64" fn mock_c770(_l2: usize) {
        CALL_LOG_390.3 += 10;
    }

    unsafe extern "win64" fn mock_9b50(
        _this: usize,
        view_ptr: usize,
        _pos: u64,
        count: u64,
    ) -> usize {
        CALL_LOG_390.2 = rd::<u64>(view_ptr + 8) ^ (count << 16);
        _this
    }

    let _m1 = crate::difftest::Mock::new(0x1_4210_dbb0, mock_dbb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_cc90, mock_cc90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4210_c770, mock_c770 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let g_fa8 = exe::ptr::<u64>(0x1_43a3_6fa8) as usize;
    let g_ed8 = exe::ptr::<usize>(0x1_43a3_6ed8) as usize;
    let g_1a8 = exe::ptr::<usize>(0x1_43a3_71a8) as usize;
    let g_40c = exe::ptr::<u32>(0x1_43a2_b40c) as usize;
    let saved = (
        rd::<u64>(g_fa8),
        rd::<usize>(g_ed8),
        rd::<usize>(g_1a8),
        rd::<u32>(g_40c),
    );

    let run = |f: F, mode: u32| -> (u8, usize, u8, [u64; 3], (i32, u64, u64, u32)) {
        CALL_LOG_390 = (if mode == 2 { 1 } else { 0 }, 0, 0, 0);

        let mut tstr_obj = [0u8; 0x30];
        let tsp = tstr_obj.as_mut_ptr() as usize;
        for (i, &b) in b"err_msg\0".iter().enumerate() {
            wr::<u8>(tsp + 0x18 + i, b);
        }

        let mut stack_buf = [0u64; 24];
        let sbp = stack_buf.as_mut_ptr() as usize;
        let top_init = sbp + 0x48;
        wr::<usize>(top_init - 0x30, tsp);
        wr::<u64>(top_init - 0x28, 0x1234_5678);
        wr::<u32>(top_init - 0x20, if mode == 0 { 0 } else { 6 });

        let mut g_state = [0u8; 0x60];
        let gsp = g_state.as_mut_ptr() as usize;
        wr::<u64>(gsp + 0x30, 0x7777);
        wr::<u64>(gsp + 0x40, 100);
        wr::<u64>(gsp + 0x48, 200);

        let mut l2 = [0u8; 0x80];
        let l2p = l2.as_mut_ptr() as usize;
        wr::<u8>(l2p + 0x01, 0x04);
        wr::<usize>(l2p + 0x08, top_init);
        wr::<usize>(l2p + 0x18, gsp);
        wr::<usize>(l2p + 0x30, sbp);

        let mut p1: usize = l2p;
        let mut p2: u64 = 0xfeed_1234;
        let mut err_str = [0u64; 34];
        let esp = err_str.as_mut_ptr() as usize;
        wr::<u64>(esp, 0x2000_0020);
        wr::<u8>(esp + 0x08, 0);

        let ret = f(
            (&mut p1 as *mut usize) as usize,
            (&mut p2 as *mut u64) as usize,
            0x9000,
            0,
            esp,
        );
        let top_delta = rd::<usize>(l2p + 0x08).wrapping_sub(sbp);
        let top0 = rd::<u64>(top_init);
        let copied = [
            if top0 != 0 { top0.wrapping_sub(tsp as u64) } else { 0 },
            rd::<u64>(top_init + 0x08),
            rd::<u64>(top_init + 0x10),
        ];
        (ret as u8, top_delta, rd::<u8>(l2p + 0x01), copied, CALL_LOG_390)
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        wr::<u64>(g_fa8, saved.0);
        wr::<usize>(g_ed8, saved.1);
        wr::<usize>(g_1a8, saved.2);
        wr::<u32>(g_40c, saved.3);
        crate::diff_eq!(o, n, "14233f390 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_rend_14233f590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64, usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG_590: (i32, u64, u64, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_dbb0(
        _l2: usize,
        _cb: usize,
        _cd: usize,
        rel_off: u64,
        _arg5: u64,
    ) -> i32 {
        CALL_LOG_590.1 = rel_off;
        let g_df8 = exe::ptr::<u64>(0x1_43a3_6df8) as usize;
        wr::<u64>(g_df8, rd::<u64>(g_df8).wrapping_add(48));
        CALL_LOG_590.0
    }

    unsafe extern "win64" fn mock_cc90(_l2: usize) -> i32 {
        CALL_LOG_590.3 += 1;
        1
    }

    unsafe extern "win64" fn mock_c770(_l2: usize) {
        CALL_LOG_590.3 += 10;
    }

    unsafe extern "win64" fn mock_9b50(
        _this: usize,
        view_ptr: usize,
        _pos: u64,
        count: u64,
    ) -> usize {
        CALL_LOG_590.2 = rd::<u64>(view_ptr + 8) ^ (count << 16);
        _this
    }

    let _m1 = crate::difftest::Mock::new(0x1_4210_dbb0, mock_dbb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_cc90, mock_cc90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4210_c770, mock_c770 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let g_fa8 = exe::ptr::<u64>(0x1_43a3_6fa8) as usize;
    let g_038 = exe::ptr::<usize>(0x1_43a3_7038) as usize;
    let g_0e8 = exe::ptr::<u64>(0x1_43a3_70e8) as usize;
    let g_df8 = exe::ptr::<usize>(0x1_43a3_6df8) as usize;
    let g_ed8 = exe::ptr::<usize>(0x1_43a3_6ed8) as usize;
    let saved = (
        rd::<u64>(g_fa8),
        rd::<usize>(g_038),
        rd::<u64>(g_0e8),
        rd::<usize>(g_df8),
        rd::<usize>(g_ed8),
    );

    let run = |f: F, mode: u32| -> (u8, u64, usize, u8, [u64; 3], (i32, u64, u64, u32)) {
        CALL_LOG_590 = (if mode == 2 { 1 } else { 0 }, 0, 0, 0);

        let mut tstr_obj = [0u8; 0x30];
        let tsp = tstr_obj.as_mut_ptr() as usize;
        for (i, &b) in b"fail_590\0".iter().enumerate() {
            wr::<u8>(tsp + 0x18 + i, b);
        }

        let mut stack_buf = [0u64; 24];
        let sbp = stack_buf.as_mut_ptr() as usize;
        let top_init = sbp + 0x30;
        wr::<usize>(top_init - 0x18, tsp);
        wr::<u64>(top_init - 0x10, 0x8765_4321);
        wr::<u32>(top_init - 0x08, if mode == 0 { 0 } else { 6 });

        let mut g_state = [0u8; 0x60];
        let gsp = g_state.as_mut_ptr() as usize;
        wr::<u64>(gsp + 0x30, 0x8888);
        wr::<u64>(gsp + 0x40, 100);
        wr::<u64>(gsp + 0x48, 200);

        let mut l2 = [0u8; 0x80];
        let l2p = l2.as_mut_ptr() as usize;
        wr::<u8>(l2p + 0x01, 0x04);
        wr::<usize>(l2p + 0x08, top_init);
        wr::<usize>(l2p + 0x18, gsp);
        wr::<usize>(l2p + 0x30, sbp);

        let mut p1 = [0usize; 6];
        p1[0] = l2p;
        let mut p2: u64 = 0xcafe_5678;
        let mut out_len: u64 = 0;
        let mut err_str = [0u64; 130];
        let esp = err_str.as_mut_ptr() as usize;
        wr::<u32>(esp, 0x8000_0080);
        wr::<u32>(esp + 0x04, 0);
        wr::<u8>(esp + 0x08, 0);

        let ret = f(
            p1.as_mut_ptr() as usize,
            (&mut p2 as *mut u64) as usize,
            0x4000,
            0,
            (&mut out_len as *mut u64) as usize,
            esp,
        );
        let top_delta = rd::<usize>(l2p + 0x08).wrapping_sub(sbp);
        let top0 = rd::<u64>(top_init);
        let copied = [
            if top0 != 0 { top0.wrapping_sub(tsp as u64) } else { 0 },
            rd::<u64>(top_init + 0x08),
            rd::<u64>(top_init + 0x10),
        ];
        (
            ret as u8,
            out_len,
            top_delta,
            rd::<u8>(l2p + 0x01),
            copied,
            CALL_LOG_590,
        )
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        wr::<u64>(g_fa8, saved.0);
        wr::<usize>(g_038, saved.1);
        wr::<u64>(g_0e8, saved.2);
        wr::<usize>(g_df8, saved.3);
        wr::<usize>(g_ed8, saved.4);
        crate::diff_eq!(o, n, "14233f590 mode={}", mode);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4233_3360, "rend_142333360", rend::cauldron_hot_z9b::rend_142333360 as *const (), Throws::Caught, rend::cauldron_hot_z9b::diff_rend_142333360),
// Replacement::verified_throws(0x1_4233_f390, "rend_14233f390", rend::cauldron_hot_z9b::rend_14233f390 as *const (), Throws::Discard, rend::cauldron_hot_z9b::diff_rend_14233f390),
// Replacement::verified_throws(0x1_4233_f590, "rend_14233f590", rend::cauldron_hot_z9b::rend_14233f590 as *const (), Throws::Discard, rend::cauldron_hot_z9b::diff_rend_14233f590),
