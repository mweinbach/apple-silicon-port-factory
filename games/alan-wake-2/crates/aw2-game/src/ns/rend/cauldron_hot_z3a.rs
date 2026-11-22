//! Decompiled `rend` hot functions shard Z3A (`0x141960f50..0x1419ec390`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    cvt_f32_i32, vaddps, vblendps, vdpps, vinsertps, vmulps, vpermilps, vshufps, vsqrtps, vsubps,
};

#[inline(always)]
unsafe fn inplace_str_view(s: usize) -> [u64; 2] {
    let hdr = rd::<u32>(s);
    let len = rd::<u32>(s + 0x04) as u64;
    let b3 = (hdr >> 24) as u32;
    let inline_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap_bits = (hdr & 0x00ff_ffff) << 3;
    let ptr = if cap_bits > inline_bits {
        rd::<u64>(s + 0x08)
    } else {
        (s + 0x08) as u64
    };
    [ptr, len]
}

#[inline(always)]
unsafe fn resolve_ecs_component(
    world: usize,
    entity_idx: u32,
    comp_id_va: u64,
    elem_size: usize,
) -> Option<usize> {
    let ent_tbl = rd::<usize>(world + 0x30);
    let entry = rd::<u64>(ent_tbl.wrapping_add((entity_idx as usize) * 8));
    if (entry as u16) == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        return None;
    }
    let arch = (entry & 0xffff) as usize;
    let row = (entry >> 32) as usize;
    let base_ptr = rd::<usize>(world.wrapping_add(arch * 8 + 0x88));
    let arch_off = arch * 0x20;
    let map_cnt = rd::<u32>(world.wrapping_add(arch_off + 0x1_80ac));
    let map_ptr = rd::<usize>(world.wrapping_add(arch_off + 0x1_8098));
    let comp_id = rd::<u16>(exe::ptr::<u16>(comp_id_va) as usize) as u32;
    let slot = exe::call_va!(0x1_4248_9ee0, (map_ptr, map_cnt, comp_id) -> i32);
    let mut off = if (slot as i64) != -1 {
        let tbl = rd::<usize>(world.wrapping_add((arch + 0xc03) * 0x20 + 0x40));
        rd::<u32>(tbl.wrapping_add((slot as usize) * 4)) as usize
    } else {
        0
    };
    if off != 0 {
        off = off.wrapping_add(base_ptr);
    }
    Some(off.wrapping_add(row.wrapping_mul(elem_size)))
}

#[inline(always)]
unsafe fn build_world_query_desc(world: usize, idx_va: u64) -> [u64; 3] {
    let vtbl = rd::<usize>(world);
    let map_ptr = rd::<usize>(vtbl + 0x20);
    let key_idx = rd::<u16>(exe::ptr::<u16>(idx_va) as usize) as usize;
    let slot = rd::<u16>(map_ptr.wrapping_add(key_idx * 2)) as usize;
    let desc_base = rd::<usize>(world + 0x08).wrapping_add(slot * 16);
    [
        world as u64,
        rd::<u32>(desc_base + 0x08) as u64,
        rd::<u64>(desc_base),
    ]
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

#[inline(always)]
unsafe fn call_acosf(x: f32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_adac,
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
fn quat_to_dir(xmm3: u128, xmm9_c: u128, xmm14_c: u128) -> u128 {
    let xmm1 = vmulps(vpermilps(xmm3, 0xca), vpermilps(xmm3, 0xc4));
    let xmm5 = vmulps(xmm1, xmm14_c);
    let xmm1_b = vmulps(vpermilps(xmm3, 0xdf), vpermilps(xmm3, 0xd1));
    let xmm3_b = vmulps(xmm1_b, xmm14_c);
    let xmm4 = vaddps(xmm3_b, xmm5);
    let xmm0 = vsubps(xmm5, xmm3_b);
    let xmm2 = vshufps(xmm4, xmm0, 0x50);
    let xmm1_c = vsubps(xmm9_c, xmm4);
    vshufps(xmm2, xmm1_c, 0xe8)
}

#[inline(always)]
unsafe fn vec_angle_5f(v_a: u128, v_b: u128, one: f32, neg_one: f32) -> f32 {
    let dot = f32::from_bits(vdpps(v_a, v_b, 0x5f) as u32);
    let len_a = f32::from_bits(vsqrtps(vdpps(v_a, v_a, 0x5f)) as u32);
    let len_b = f32::from_bits(vsqrtps(vdpps(v_b, v_b, 0x5f)) as u32);
    let prod = len_a * len_b;
    let inv = if !(prod == 0.0) { one / prod } else { 0.0 };
    let mut c = dot * inv;
    c = if c > neg_one { c } else { neg_one };
    c = if c < one { c } else { one };
    call_acosf(c)
}

/// `bigfish_141960f50`
/// Original: 0x141960f50 (1046 bytes, Throws::Caught)
pub unsafe extern "win64" fn bigfish_141960f50(param_1: usize, param_2: usize) {
    if rd::<u64>(exe::ptr::<u64>(0x1_43a3_40f8) as usize) == 0 {
        return;
    }

    let mut cur_tbl = exe::ptr::<u8>(0x1_4272_28f0) as usize;
    let end_tbl = exe::ptr::<u8>(0x1_4272_2990) as usize;
    let mut handle_slot: u64 = 0;

    while cur_tbl != end_tbl {
        let key_ptr = rd::<usize>(cur_tbl);
        let key = rd::<u16>(key_ptr);
        let c0 = exe::call_va!(0x1_420e_8920, () -> u64);
        let c1 = exe::call_va!(0x1_420e_8750, (c0) -> u64);
        let pressed = exe::call_va!(0x1_420e_3850, (c1, key) -> u8);
        if pressed != 0 {
            let ev_arg = rd::<u64>(cur_tbl + 0x08);
            let id = rd::<u32>(param_1 + 0x18);
            handle_slot = (handle_slot & !0xffff_ffffu64) | (id as u64);
            let res = exe::call_va!(
                0x1_4141_0810,
                (0usize, (&mut handle_slot as *mut u64) as usize) -> usize
            );
            let obj = rd::<usize>(rd::<usize>(res) + 0x28);
            let vfn = rd::<usize>(rd::<usize>(obj) + 0x250);
            exe::call_va!(vfn, (obj, ev_arg));
        }
        cur_tbl = cur_tbl.wrapping_add(0x10);
    }

    let rbx_world = rd::<usize>(param_2);
    let desc_05d4 = build_world_query_desc(rbx_world, 0x1_43c9_05d4);
    let mut desc_work = build_world_query_desc(rbx_world, 0x1_43c8_ceb4);
    let mut sv = inplace_str_view(exe::ptr::<u8>(0x1_43a2_0fd8) as usize);

    exe::call_va!(
        0x1_401e_8cc0,
        (
            (&mut handle_slot as *mut u64) as usize,
            sv.as_mut_ptr() as usize,
            desc_work.as_mut_ptr() as usize,
        )
    );

    let ent_idx1 = handle_slot as u32;
    if ent_idx1 == u32::MAX {
        return;
    }

    let r14_world = desc_work[0] as usize;
    let Some(comp1) = resolve_ecs_component(r14_world, ent_idx1, 0x1_43c8_cecc, 0x140) else {
        return;
    };

    let mut local_str = [0u64; 6];
    let s_ptr = local_str.as_mut_ptr() as usize;
    wr::<u64>(s_ptr, 0x0400_0004);
    wr::<u8>(s_ptr + 0x08, 0);

    let idx = rd::<i32>(comp1 + 0x11c) as i64;
    let cnt = rd::<u32>(comp1 + 0x128) as u64;
    if idx >= 0 && (idx as u64) < cnt {
        let elem = rd::<usize>(comp1 + 0x120).wrapping_add((idx as usize) * 0x28);
        if s_ptr != elem {
            let mut elem_sv = inplace_str_view(elem);
            let elem_len = elem_sv[1];
            exe::call_va!(
                0x1_4240_9b50,
                (s_ptr, elem_sv.as_mut_ptr() as usize, 0u64, elem_len)
            );
        }
    } else {
        let mut empty_sv: [u64; 2] = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (s_ptr, empty_sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    desc_work = desc_05d4;
    exe::call_va!(
        0x1_4196_2240,
        (
            (&mut handle_slot as *mut u64) as usize,
            s_ptr,
            desc_work.as_mut_ptr() as usize,
        )
    );
    exe::call_va!(0x1_4240_ab10, (s_ptr));

    let ent_idx2 = handle_slot as u32;
    if ent_idx2 == u32::MAX {
        return;
    }

    let Some(comp2) = resolve_ecs_component(rbx_world, ent_idx2, 0x1_43c9_0338, 0x6c) else {
        return;
    };

    wr::<u8>(comp2 + 0x5d, 1);
    for (off, key_va) in [
        (0x5eusize, 0x1_4397_d9a0u64),
        (0x5f, 0x1_4397_d988),
        (0x60, 0x1_4397_d984),
        (0x61, 0x1_4397_d9a4),
    ] {
        let key = rd::<u16>(exe::ptr::<u16>(key_va) as usize);
        let c0 = exe::call_va!(0x1_420e_8920, () -> u64);
        let c1 = exe::call_va!(0x1_420e_8750, (c0) -> u64);
        let b = exe::call_va!(0x1_420e_3850, (c1, key) -> u8);
        wr::<u8>(comp2 + off, b);
    }
}

/// `bigfish_141961470`
/// Original: 0x141961470 (423 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_141961470(param_1: usize, param_2: usize) {
    let mut rdi: usize = 0;
    let id = rd::<i32>(param_1 + 0x18);
    if id != -1 {
        let mut local_id: u64 = id as u32 as u64;
        let res = exe::call_va!(
            0x1_4141_0810,
            (param_1, (&mut local_id as *mut u64) as usize) -> usize
        );
        rdi = rd::<usize>(rd::<usize>(res) + 0x28);
    }

    let mut evt = [0u8; 0x30];
    let ep = evt.as_mut_ptr() as usize;

    let vfn_68 = rd::<usize>(rd::<usize>(rdi) + 0x68);
    let w = exe::call_va!(vfn_68, (rdi) -> u32);
    let x_val = cvt_f32_i32(((w as u64 as f32) * rd::<f32>(param_2)).round_ties_even()) as i32;
    wr::<i32>(ep + 0x08, x_val);

    let vfn_70 = rd::<usize>(rd::<usize>(rdi) + 0x70);
    let h = exe::call_va!(vfn_70, (rdi) -> u32);
    let b_flag = rd::<u8>(param_2 + 0x0c);
    let y_val =
        cvt_f32_i32(((h as u64 as f32) * rd::<f32>(param_2 + 0x04)).round_ties_even()) as i32;
    wr::<i32>(ep + 0x0c, y_val);

    if b_flag != rd::<u8>(param_1 + 0x298) {
        wr::<u32>(ep, if b_flag != 0 { 1 } else { 2 });
        let vfn_e8 = rd::<usize>(rd::<usize>(rdi) + 0xe8);
        exe::call_va!(vfn_e8, (rdi, ep, 0u64, 0u64));
    }

    if rd::<i32>(ep + 0x08) != rd::<i32>(param_1 + 0x29c)
        || rd::<i32>(ep + 0x0c) != rd::<i32>(param_1 + 0x2a0)
    {
        wr::<u32>(ep, 0);
        let vfn_e8 = rd::<usize>(rd::<usize>(rdi) + 0xe8);
        exe::call_va!(vfn_e8, (rdi, ep, 0u64, 0u64));
    }

    let wheel = rd::<f32>(param_2 + 0x08);
    if !(wheel == rd::<f32>(param_1 + 0x2a4)) {
        let scale = rd::<f32>(exe::ptr::<f32>(0x1_4342_7138) as usize);
        wr::<f32>(ep + 0x14, wheel * scale);
        wr::<f32>(ep + 0x10, 0.0);
        wr::<u32>(ep, 3);
        let vfn_e8 = rd::<usize>(rd::<usize>(rdi) + 0xe8);
        exe::call_va!(vfn_e8, (rdi, ep, 0u64, 0u64));
    }

    let out_x = rd::<u32>(ep + 0x08);
    let out_y = rd::<u32>(ep + 0x0c);
    wr::<f32>(param_1 + 0x2a4, wheel);
    wr::<u64>(param_1 + 0x29c, (out_x as u64) | ((out_y as u64) << 32));
    wr::<u8>(param_1 + 0x298, b_flag);
}

/// `bigfish_141961620`
/// Original: 0x141961620 (1860 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_141961620(param_1: usize, param_2: usize) {
    let world0 = rd::<usize>(param_2);
    let mut desc_ced4 = build_world_query_desc(world0, 0x1_43c8_ced4);
    let mut sv_0b58 = inplace_str_view(exe::ptr::<u8>(0x1_43a2_0b58) as usize);
    let mut out_handle: u64 = 0;

    exe::call_va!(
        0x1_401e_8cc0,
        (
            (&mut out_handle as *mut u64) as usize,
            sv_0b58.as_mut_ptr() as usize,
            desc_ced4.as_mut_ptr() as usize,
        )
    );

    let ent_idx = out_handle as u32;
    if ent_idx == u32::MAX {
        return;
    }

    let rdi_world = desc_ced4[0] as usize;
    let mut r12b = false;
    let mut flag_77: u8 = 0;

    let Some(rbx_comp) = resolve_ecs_component(rdi_world, ent_idx, 0x1_43c8_cecc, 0x140) else {
        return;
    };

    let mut tmp_str = [0u64; 18];
    let ts_ptr = tmp_str.as_mut_ptr() as usize;

    let s1 = exe::call_va!(0x1_41df_ae90, (ts_ptr, rbx_comp) -> usize);
    let flag_6f = exe::call_va!(
        0x1_401e_6ff0,
        (s1, exe::ptr::<u8>(0x1_43a2_0fd8) as usize) -> u8
    );
    exe::call_va!(0x1_4240_ab10, (ts_ptr));

    let s2 = exe::call_va!(0x1_41df_ae90, (ts_ptr, rbx_comp) -> usize);
    let mut sv2 = inplace_str_view(s2);
    let cur_state = exe::call_va!(0x1_41df_a9a0, (rbx_comp, sv2.as_mut_ptr() as usize) -> i32);
    exe::call_va!(0x1_4240_ab10, (ts_ptr));

    let prev_state = rd::<i32>(param_1 + 0x260);
    if prev_state != cur_state {
        let mut sv_0d38 = inplace_str_view(exe::ptr::<u8>(0x1_43a2_0d38) as usize);
        let st_0d38 =
            exe::call_va!(0x1_41df_a9a0, (rbx_comp, sv_0d38.as_mut_ptr() as usize) -> i32);
        let match_target = if cur_state == st_0d38 {
            true
        } else {
            let mut sv_0bf8 = inplace_str_view(exe::ptr::<u8>(0x1_43a2_0bf8) as usize);
            let st_0bf8 =
                exe::call_va!(0x1_41df_a9a0, (rbx_comp, sv_0bf8.as_mut_ptr() as usize) -> i32);
            cur_state == st_0bf8
        };

        if match_target {
            for str_va in [0x1_43a2_1230u64, 0x1_43a2_0818, 0x1_43a2_0d60] {
                let mut sv_chk = inplace_str_view(exe::ptr::<u8>(str_va) as usize);
                let st_chk =
                    exe::call_va!(0x1_41df_a9a0, (rbx_comp, sv_chk.as_mut_ptr() as usize) -> i32);
                if prev_state == st_chk || (st_chk >= 0 && st_chk < prev_state) {
                    flag_77 = 1;
                    break;
                }
            }
        }

        wr::<i32>(param_1 + 0x260, cur_state);
        if rd::<u8>(param_1 + 0x2ac) != 0 && flag_6f == 0 {
            r12b = true;
        }
        wr::<u8>(param_1 + 0x2ac, flag_6f);
    }

    let mut flag_7f: u8 = 0;
    for str_va in [0x1_43a2_1230u64, 0x1_43a2_0d60, 0x1_43a2_0818] {
        let mut desc_ceb4 = build_world_query_desc(rdi_world, 0x1_43c8_ceb4);
        let mut sv_chk = inplace_str_view(exe::ptr::<u8>(str_va) as usize);
        let ok = exe::call_va!(
            0x1_41df_9270,
            (
                sv_chk.as_mut_ptr() as usize,
                rbx_comp,
                desc_ceb4.as_mut_ptr() as usize,
            ) -> u8
        );
        if ok != 0 {
            flag_7f = 1;
            break;
        }
    }

    if flag_7f == 0 && flag_77 == 0 && flag_6f == 0 && !r12b {
        return;
    }

    let mut obj_28: usize = 0;
    let id = rd::<i32>(param_1 + 0x18);
    if id != -1 {
        let mut local_id: u64 = id as u32 as u64;
        let res = exe::call_va!(
            0x1_4141_0810,
            (0usize, (&mut local_id as *mut u64) as usize) -> usize
        );
        obj_28 = rd::<usize>(rd::<usize>(res) + 0x28);
    }

    let mut cur = rd::<usize>(param_1 + 0x250);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0x258) as usize) * 8);
    let p1_280 = param_1.wrapping_add(0x280);
    while cur != end {
        let item = rd::<u64>(cur);
        let w = rd::<u64>(param_2);
        exe::call_va!(0x1_418a_0a90, (item, obj_28, param_1, p1_280, w));
        cur = cur.wrapping_add(8);
    }

    if flag_7f != 0 || flag_6f != 0 {
        let w = rd::<u64>(param_2);
        exe::call_va!(0x1_41df_37e0, (p1_280, param_1, w));
    }

    if flag_77 != 0 || r12b {
        let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_d850) as usize);
        wr::<u8>(g + 0x120, 1);
        exe::call_va!(0x1_41a0_dec0, (g));
        if flag_77 != 0 {
            let w = rd::<usize>(param_2);
            let arr = rd::<usize>(w + 0x5_80e0);
            let i_r9 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce80) as usize) as usize;
            let i_r8 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce84) as usize) as usize;
            let i_rdx = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
            let arg_r9 = rd::<u64>(arr.wrapping_add(i_r9 * 8));
            let arg_r8 = rd::<u64>(arr.wrapping_add(i_r8 * 8));
            let arg_rdx = rd::<u64>(arr.wrapping_add(i_rdx * 8));
            let g2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_d850) as usize);
            exe::call_va!(0x1_41a0_efc0, (g2, arg_rdx, arg_r8, arg_r9));
        }
    }
}

/// `bigfish_141961db0`
/// Original: 0x141961db0 (583 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_141961db0(param_1: usize, param_2: usize, param_3: usize) {
    let id0 = rd::<i32>(param_1 + 0x18);
    if id0 == -1 {
        return;
    }

    let mut local_id: u64 = id0 as u32 as u64;
    let res0 = exe::call_va!(
        0x1_4141_0810,
        (param_1, (&mut local_id as *mut u64) as usize) -> usize
    );
    let obj0 = rd::<usize>(rd::<usize>(res0) + 0x28);
    let vfn_1e0 = rd::<usize>(rd::<usize>(obj0) + 0x1e0);
    let active = exe::call_va!(vfn_1e0, (obj0) -> u8);
    if active == 0 {
        return;
    }

    wr::<u32>(param_1 + 0x2a8, rd::<u32>(param_1 + 0x2a8).wrapping_add(1));

    let w3 = rd::<usize>(param_3);
    let arr_580e0 = rd::<usize>(w3 + 0x5_80e0);
    let idx_030c = rd::<u16>(exe::ptr::<u16>(0x1_43c9_030c) as usize) as usize;
    let r14_vec = rd::<usize>(arr_580e0.wrapping_add(idx_030c * 8));
    let mut rbx_cur = rd::<usize>(r14_vec);
    let rdi_end = rbx_cur.wrapping_add((rd::<u32>(r14_vec + 0x08) as usize) * 16);

    if rbx_cur == rdi_end {
        if rd::<u32>(param_1 + 0x29c) != 0 || rd::<u32>(param_1 + 0x2a0) != 0 {
            wr::<u64>(param_1 + 0x29c, 0);
            wr::<u8>(param_1 + 0x298, 0);
            let mut evt = [0u8; 0x30];
            let mut obj1: usize = 0;
            let id1 = rd::<i32>(param_1 + 0x18);
            if id1 != -1 {
                local_id = id1 as u32 as u64;
                let res1 = exe::call_va!(
                    0x1_4141_0810,
                    (0usize, (&mut local_id as *mut u64) as usize) -> usize
                );
                obj1 = rd::<usize>(rd::<usize>(res1) + 0x28);
            }
            let vfn_e8 = rd::<usize>(rd::<usize>(obj1) + 0xe8);
            exe::call_va!(vfn_e8, (obj1, evt.as_mut_ptr() as usize, 0u64, 0u64));
        }
    } else {
        while rbx_cur != rdi_end {
            let mut item = rd::<[u64; 2]>(rbx_cur);
            exe::call_va!(0x1_4196_1470, (param_1, item.as_mut_ptr() as usize));
            rbx_cur = rbx_cur.wrapping_add(16);
        }
        wr::<u32>(r14_vec + 0x08, 0);
    }

    let w3_b = rd::<usize>(param_3);
    let arr_b = rd::<usize>(w3_b + 0x5_80e0);
    let idx_cebc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cebc) as usize) as usize;
    let ptr_cebc = rd::<usize>(arr_b.wrapping_add(idx_cebc * 8));
    let val_cebc = rd::<u32>(ptr_cebc);
    let ref_ea8 = rd::<u32>(exe::ptr::<u32>(0x1_43a2_9ea8) as usize);
    let is_ne = val_cebc != ref_ea8;
    wr::<u8>(param_1 + 0x130, if is_ne { 1 } else { 0 });

    exe::call_va!(0x1_41fc_89c0, (param_1, param_2, param_3));
    if !is_ne {
        exe::call_va!(0x1_4196_0f50, (param_1, param_2));
    }

    let mut obj2: usize = 0;
    let id2 = rd::<i32>(param_1 + 0x18);
    if id2 != -1 {
        local_id = (local_id & !0xffff_ffffu64) | (id2 as u32 as u64);
        let res2 = exe::call_va!(
            0x1_4141_0810,
            (0usize, (&mut local_id as *mut u64) as usize) -> usize
        );
        obj2 = rd::<usize>(rd::<usize>(res2) + 0x28);
    }
    exe::call_va!(0x1_41df_69d0, (param_1 + 0x268, obj2, rd::<u64>(param_2)));

    let mut cur_40 = rd::<usize>(param_1 + 0x240);
    let end_40 = cur_40.wrapping_add((rd::<u32>(param_1 + 0x248) as usize) * 8);
    while cur_40 != end_40 {
        let elem = rd::<usize>(cur_40);
        let vfn_10 = rd::<usize>(rd::<usize>(elem) + 0x10);
        let mut obj3: usize = 0;
        let id3 = rd::<i32>(param_1 + 0x18);
        if id3 != -1 {
            local_id = (local_id & !0xffff_ffffu64) | (id3 as u32 as u64);
            let res3 = exe::call_va!(
                0x1_4141_0810,
                (0usize, (&mut local_id as *mut u64) as usize) -> usize
            );
            obj3 = rd::<usize>(rd::<usize>(res3) + 0x28);
        }
        exe::call_va!(vfn_10, (elem, obj3));
        cur_40 = cur_40.wrapping_add(8);
    }

    exe::call_va!(0x1_4196_1620, (param_1, param_2));
    exe::call_va!(0x1_4196_0ba0, (param_1, param_2));
}

/// `unknown_1419df070`
/// Original: 0x1419df070 (271 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1419df070(param_1: usize) -> u64 {
    let p18 = rd::<usize>(param_1 + 0x18);
    let idx = rd::<usize>(param_1 + 0x28);
    let off = idx.wrapping_mul(0x198);
    let mut rbx = rd::<usize>(p18.wrapping_add(off + 0xa8));
    let mut rax = rd::<u32>(p18.wrapping_add(off + 0xb0)) as u64;
    let rdi = rbx.wrapping_add((rax as usize).wrapping_mul(0xf0));
    if rbx != rdi {
        let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let mut handle_buf = [0u64; 4];
        let hp = handle_buf.as_mut_ptr() as usize;
        loop {
            let gt = rd::<i32>(rbx + 0x54) > 0;
            wr::<u8>(rbx + 0x68, if gt { 1 } else { 0 });
            let h_id = rd::<u32>(rbx + 0x48);
            exe::call_va!(0x1_419e_43c0, (hp, h_id));
            if rd::<u8>(rbx + 0x68) != 0 {
                let mode = rd::<u32>(rbx + 0x60);
                match mode {
                    0 => {
                        let base0 = rd::<usize>(param_1);
                        let v = rd::<u8>(base0.wrapping_add(idx.wrapping_mul(0x38) + 0x1e));
                        wr::<u8>(rbx + 0x68, v);
                    }
                    1 => {
                        let base8 = rd::<usize>(param_1 + 0x08);
                        let fval = rd::<f32>(base8.wrapping_add(idx.wrapping_mul(0x10)));
                        wr::<u8>(rbx + 0x68, if one > fval { 1 } else { 0 });
                    }
                    3 => {
                        wr::<u8>(rbx + 0x68, 0);
                    }
                    4 => {
                        wr::<u8>(rbx + 0x68, 1);
                    }
                    6 => {
                        let p = exe::call_va!(0x1_4021_7df0, (hp) -> usize);
                        let v = rd::<u8>(p + 0xd8);
                        wr::<u8>(rbx + 0x68, v);
                    }
                    _ => {}
                }
            }
            rbx = rbx.wrapping_add(0xf0);
            rax = rd::<u64>(hp + 0x08);
            wr::<u64>(hp + 0x08, 0);
            if rbx == rdi {
                break;
            }
        }
    }
    rax
}

/// `unknown_1419e43c0`
/// Original: 0x1419e43c0 (325 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1419e43c0(param_1: usize, param_2: u32) -> usize {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_d9e0) as usize);
    let rdi = rd::<i32>(rsi + 0x204) as i64 as u64;
    if rdi != 0 {
        let mut r11 = rd::<usize>(rsi + 0x208);
        let mut r10: u64 = 0;
        loop {
            let str_ptr = rd::<usize>(r11);
            let mut len = 0usize;
            while rd::<u8>(str_ptr.wrapping_add(len)) != 0 {
                len += 1;
            }
            let hash = if len == 0 {
                0u32
            } else {
                let mut h = 0x811c_9dc5u32;
                for i in 0..len {
                    let c = (rd::<i8>(str_ptr.wrapping_add(i)) as i32 as u32) | 0x20;
                    h = (c ^ h).wrapping_mul(0x0100_0193);
                }
                h
            };
            if param_2 == hash {
                let arr_210 = rd::<usize>(rsi + 0x210);
                let src = arr_210.wrapping_add((r10 as usize) * 8);
                wr::<u64>(param_1, 0);
                wr::<u64>(param_1 + 0x08, 0);
                wr::<u32>(param_1 + 0x10, 0);
                wr::<u8>(param_1 + 0x14, 0);
                exe::call_va!(0x1_4034_ff70, (param_1, src));
                return param_1;
            }
            r10 += 1;
            r11 = r11.wrapping_add(0xf8);
            if r10 >= rdi {
                break;
            }
        }
    }

    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x10, 0);
    wr::<u8>(param_1 + 0x14, 0);
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(param_1 + 0x14) != 0 {
            core::hint::spin_loop();
        }
        let prev = crate::simd_ops::atomic_xchg_u8((param_1 + 0x14) as u64, 1);
        if prev == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x10, 0);
    wr::<u8>(param_1 + 0x14, 0);
    wr::<u64>(param_1, 0);
    param_1
}

/// `std_1419eb8f0`
/// Original: 0x1419eb8f0 (352 bytes, Throws::Discard)
pub unsafe extern "win64" fn std_1419eb8f0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
) {
    let mut local_res8: u64 = u64::MAX;
    if rd::<u64>(param_1 + 0x18) != 0 {
        let idx = rd::<usize>(param_1 + 0x28);
        let base = rd::<usize>(param_1);
        let slot = base.wrapping_add(idx.wrapping_mul(0x18));
        wr::<u64>(slot + 0x10, local_res8);
        wr::<u32>(slot + 0x08, 0);
        return;
    }

    let mut copy_p3 = rd::<[u64; 3]>(param_3);
    let idx4 = rd::<i32>(param_4) as isize;
    let rbx_id = rd::<u64>((param_4 as isize).wrapping_add(idx4 * 8 + 4) as usize);
    let rdx_arg = if (rbx_id as u32) != u32::MAX {
        rbx_id
    } else {
        local_res8
    };
    let res_ptr = exe::call_va!(
        0x1_4020_d4a0,
        (
            (&mut local_res8 as *mut u64) as usize,
            rdx_arg,
            copy_p3.as_mut_ptr() as usize,
        ) -> usize
    );
    if rd::<i32>(res_ptr) != -1 {
        let comp_ptr = exe::call_va!(0x1_403e_a9a0, (param_3, rbx_id) -> usize);
        let mut copy_p2 = rd::<[u64; 3]>(param_2);
        let mut copy_p1 = rd::<[u64; 6]>(param_1);
        exe::call_va!(
            0x1_419e_c390,
            (
                copy_p1.as_mut_ptr() as usize,
                copy_p2.as_mut_ptr() as usize,
                comp_ptr,
                param_5,
            )
        );
        copy_p2 = rd::<[u64; 3]>(param_2);
        copy_p1 = rd::<[u64; 6]>(param_1);
        exe::call_va!(
            0x1_419e_bcd0,
            (
                copy_p1.as_mut_ptr() as usize,
                copy_p2.as_mut_ptr() as usize,
                comp_ptr,
            )
        );
    }
}

/// `std_1419ec390`
/// Original: 0x1419ec390 (2269 bytes, Throws::Caught)
pub unsafe extern "win64" fn std_1419ec390(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut buf_50 = rd::<[u64; 4]>(param_1);
    let ent_row = rd::<usize>(param_1 + 0x28);
    let rsi_state = rd::<usize>(param_1).wrapping_add(ent_row.wrapping_mul(0x18));
    let wt_ptr = rd::<usize>(param_1 + 0x08).wrapping_add(ent_row.wrapping_mul(0x20));
    let cc_base = rd::<usize>(param_1 + 0x10);
    let cc_inner = rd::<usize>(rd::<usize>(cc_base.wrapping_add(ent_row.wrapping_mul(0x18) + 0x08)));
    let height = rd::<f32>(cc_inner + 0x20);
    let ent_pos = rd::<u128>(wt_ptr + 0x10);

    let rsp_110 = vinsertps(ent_pos, 0, 0x50);
    let pos_y_vec = vshufps(ent_pos, ent_pos, 0x55);
    let half_f = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let half_h = (half_f * height).to_bits() as u128;
    let rsp_100 = vaddps(vinsertps(0, half_h, 0x10), vinsertps(0, pos_y_vec, 0x10));

    let k_075 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6820) as usize);
    let k_2 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize);
    let rsp_318 = call_powf(height * k_075, k_2);

    let mut cam_q_raw = [0u32; 4];
    exe::call_va!(0x1_4027_5010, (cam_q_raw.as_mut_ptr() as usize, param_3));
    let mut cam_q = (cam_q_raw[1] as u128) | ((cam_q_raw[2] as u128) << 32);
    cam_q = vinsertps(cam_q, cam_q_raw[3] as u128, 0x20);
    cam_q = vinsertps(cam_q, cam_q_raw[0] as u128, 0x30);

    let cam_pos_xy = rd::<u64>(param_3 + 0x24) as u128;
    let cam_pos_z = rd::<u32>(param_3 + 0x2c) as u128;
    let xmm12_cam_pos = vinsertps(cam_pos_xy, cam_pos_z, 0x20);

    let c_3e4530 = rd::<u128>(exe::ptr::<u128>(0x1_433e_4530) as usize);
    let c_427ab0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize);
    let xmm11_cam_dir = quat_to_dir(cam_q, c_3e4530, c_427ab0);

    let deg_to_rad = rd::<f32>(exe::ptr::<f32>(0x1_4342_6514) as usize);
    let fov_deg_038 = rd::<f32>(exe::ptr::<f32>(0x1_4384_9038) as usize);
    let xmm15_f = (fov_deg_038 * deg_to_rad) * half_f;

    wr::<u32>(rsi_state + 0x08, 0);

    let r11_storage = rd::<usize>(param_2).wrapping_add(0x40);
    let mut query_tail = rd::<u128>(param_2 + 0x08);
    exe::call_va!(
        0x1_401e_7c10,
        (
            buf_50.as_mut_ptr() as usize,
            r11_storage,
            (&mut query_tail as *mut u128) as usize,
        )
    );

    let r13_storage = buf_50[0] as usize;
    let mut arch_cur = buf_50[1] as usize;
    let arch_end = buf_50[2] as usize;
    let mut count: usize = 0;
    let mut r14_idx: usize = 0;
    let mut r15_chunk: usize = 0;
    let mut col0: usize = 0;
    let mut col1: usize = 0;
    let mut col2: usize = 0;
    let mut col3: usize = 0;

    if arch_cur != arch_end {
        let arch = rd::<u16>(arch_cur) as usize;
        r15_chunk = rd::<usize>(r11_storage.wrapping_add(arch * 8 + 0x48));
        if r15_chunk != 0 {
            count = rd::<u32>(r11_storage.wrapping_add(arch * 4 + 0x1_0048)) as usize;
        }
        exe::call_va!(
            0x1_4046_e2f0,
            (buf_50.as_mut_ptr() as usize, r11_storage, arch as u32)
        );
        col0 = if buf_50[0] != 0 {
            (buf_50[0] as usize).wrapping_add(r15_chunk)
        } else {
            0
        };
        col1 = if buf_50[1] != 0 {
            (buf_50[1] as usize).wrapping_add(r15_chunk)
        } else {
            0
        };
        col2 = if buf_50[2] != 0 {
            (buf_50[2] as usize).wrapping_add(r15_chunk)
        } else {
            0
        };
        col3 = if buf_50[3] != 0 {
            (buf_50[3] as usize).wrapping_add(r15_chunk)
        } else {
            0
        };
    }

    if count == 0 {
        return;
    }

    let one_f = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let neg_one_f = rd::<f32>(exe::ptr::<f32>(0x1_4342_7080) as usize);
    let sign_mask = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);

    loop {
        let elem0 = col0.wrapping_add(r14_idx.wrapping_mul(0x28));
        let r12_wt = col2.wrapping_add(r14_idx.wrapping_mul(0x20));
        let rax_off = col3.wrapping_add(r14_idx.wrapping_mul(0x10));

        let off_vec = rd::<u128>(rax_off);
        let world_pos = vaddps(off_vec, rd::<u128>(r12_wt + 0x10));
        let xz_pos = vblendps(world_pos, 0, 0x2);
        let delta_xz = vsubps(xz_pos, rsp_110);
        let dist_sq = f32::from_bits(vdpps(delta_xz, delta_xz, 0x7f) as u32);

        'entity_check: {
            if dist_sq > rd::<f32>(elem0) {
                break 'entity_check;
            }

            if rd::<u8>(elem0 + 0x22) == 0 {
                let y_only = vinsertps(0, vpermilps(world_pos, 0x55), 0x10);
                let dy = vsubps(y_only, rsp_100);
                let dy_sq = f32::from_bits(vdpps(dy, dy, 0x7f) as u32);
                let max_dy_sq = if rd::<u8>(elem0 + 0x14) != 0 {
                    rd::<f32>(elem0 + 0x10)
                } else {
                    rsp_318
                };
                if dy_sq > max_dy_sq {
                    break 'entity_check;
                }
            }

            if dist_sq > rd::<f32>(exe::ptr::<f32>(0x1_4384_8fb0) as usize) {
                let cam_to_ent = vsubps(xz_pos, xmm12_cam_pos);
                let angle1 = vec_angle_5f(xmm11_cam_dir, cam_to_ent, one_f, neg_one_f);
                let max_angle1 = if rd::<u8>(elem0 + 0x1c) != 0 {
                    rd::<f32>(elem0 + 0x18)
                } else {
                    let fov_000 = rd::<f32>(exe::ptr::<f32>(0x1_4384_9000) as usize);
                    (fov_000 * deg_to_rad) * half_f
                };
                if angle1 > max_angle1 {
                    break 'entity_check;
                }
            }

            if rd::<u8>(elem0 + 0x23) == 0 {
                let target_q = rd::<u128>(r12_wt);
                let target_dir = quat_to_dir(target_q, c_3e4530, c_427ab0);
                let neg_dir = target_dir ^ sign_mask;
                let angle2 = vec_angle_5f(neg_dir, delta_xz, one_f, neg_one_f);
                let max_angle2 = if rd::<u8>(elem0 + 0x0c) != 0 {
                    rd::<f32>(elem0 + 0x08)
                } else {
                    xmm15_f
                };
                if angle2 > max_angle2 {
                    break 'entity_check;
                }
            }

            let min_dist_sq = rd::<f32>(elem0 + 0x04);
            let ge = min_dist_sq >= dist_sq;
            if rd::<u8>(elem0 + 0x20) == 0 && (rd::<u8>(elem0 + 0x21) == 0 || !ge) {
                let start_pos = vaddps(rd::<u128>(r12_wt + 0x10), rd::<u128>(rax_off));
                let mut ray_opts = [0u64; 10];
                let rop = ray_opts.as_mut_ptr() as usize;
                wr::<u64>(rop, 0x15);
                wr::<u64>(rop + 0x40, 0);
                wr::<u32>(rop + 0x48, 1);

                let mut tmp_fn = [0u64; 8];
                let tfp = tmp_fn.as_mut_ptr() as usize;
                let heap_fn = exe::call_va!(0x1_4183_4d64, (0x68u64) -> usize);
                wr::<usize>(heap_fn, exe::ptr::<u8>(0x1_4313_e9c8) as usize);
                wr::<[u64; 6]>(heap_fn + 0x08, rd::<[u64; 6]>(param_1));
                wr::<u64>(heap_fn + 0x38, col0 as u64);
                wr::<u64>(heap_fn + 0x40, col1 as u64);
                wr::<u64>(heap_fn + 0x48, col2 as u64);
                wr::<u64>(heap_fn + 0x50, col3 as u64);
                wr::<u64>(heap_fn + 0x58, r15_chunk as u64);
                wr::<u64>(heap_fn + 0x60, r14_idx as u64);
                wr::<usize>(tfp + 0x38, heap_fn);

                exe::call_va!(0x1_401f_a380, (tfp, rop + 0x08));
                let rem_fn = rd::<usize>(tfp + 0x38);
                if rem_fn != 0 {
                    let vfn = rd::<usize>(rd::<usize>(rem_fn) + 0x20);
                    let is_heap = if rem_fn != tfp { 1u8 } else { 0u8 };
                    exe::call_va!(vfn, (rem_fn, is_heap));
                }

                wr::<u128>(buf_50.as_mut_ptr() as usize, start_pos);
                let mut end_pos: u128 = xmm12_cam_pos;
                let mut hit_out = [0u8; 0x80];
                let hop = hit_out.as_mut_ptr() as usize;

                exe::call_va!(
                    0x1_41e9_2fe0,
                    (
                        hop,
                        rd::<u64>(param_4),
                        buf_50.as_mut_ptr() as usize,
                        (&mut end_pos as *mut u128) as usize,
                        rop,
                    )
                );

                let blocked = rd::<u8>(hop + 0x70) != 0;
                let ro_fn = rd::<usize>(rop + 0x40);
                if ro_fn != 0 {
                    let vfn = rd::<usize>(rd::<usize>(ro_fn) + 0x20);
                    let is_heap = if ro_fn != (rop + 0x08) { 1u8 } else { 0u8 };
                    exe::call_va!(vfn, (ro_fn, is_heap));
                }
                if blocked {
                    break 'entity_check;
                }
            }

            let ent_handle = rd::<u64>(r15_chunk.wrapping_add(r14_idx * 8 + 0x10));
            let len = rd::<u32>(rsi_state + 0x08);
            let cap = rd::<u32>(rsi_state + 0x0c);
            if (len as u64) * 8 == (cap as u64) * 8 {
                let need = (len as u64) + 1;
                let grow = ((cap >> 1) as u64) + (cap as u64);
                let new_cap = if grow < need { need } else { grow };
                exe::call_va!(0x1_401d_eba0, (rsi_state, new_cap));
            }
            let cur_len = rd::<u32>(rsi_state + 0x08) as usize;
            let data_ptr = rd::<usize>(rsi_state);
            wr::<u64>(data_ptr.wrapping_add(cur_len * 8), ent_handle);
            wr::<u32>(rsi_state + 0x08, (cur_len as u32).wrapping_add(1));
        }

        r14_idx += 1;
        if r14_idx >= count {
            loop {
                arch_cur = arch_cur.wrapping_add(2);
                if arch_cur == arch_end {
                    break;
                }
                let a = rd::<u16>(arch_cur) as usize;
                if rd::<usize>(r13_storage.wrapping_add(a * 8 + 0x48)) != 0
                    && rd::<u32>(r13_storage.wrapping_add(a * 4 + 0x1_0048)) != 0
                {
                    break;
                }
            }
            if arch_cur != arch_end {
                let a = rd::<u16>(arch_cur) as usize;
                r15_chunk = rd::<usize>(r11_storage.wrapping_add(a * 8 + 0x48));
                count = if r15_chunk != 0 {
                    rd::<u32>(r11_storage.wrapping_add(a * 4 + 0x1_0048)) as usize
                } else {
                    0
                };
                r14_idx = 0;
                exe::call_va!(
                    0x1_4046_e2f0,
                    (buf_50.as_mut_ptr() as usize, r11_storage, a as u32)
                );
                col0 = if buf_50[0] != 0 {
                    (buf_50[0] as usize).wrapping_add(r15_chunk)
                } else {
                    0
                };
                col1 = if buf_50[1] != 0 {
                    (buf_50[1] as usize).wrapping_add(r15_chunk)
                } else {
                    0
                };
                col2 = if buf_50[2] != 0 {
                    (buf_50[2] as usize).wrapping_add(r15_chunk)
                } else {
                    0
                };
                col3 = if buf_50[3] != 0 {
                    (buf_50[3] as usize).wrapping_add(r15_chunk)
                } else {
                    0
                };
            }
            if r14_idx >= count {
                break;
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_bigfish_141960f50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HOLDER_PTR: usize = 0;
    static mut KEY_CALLS: u32 = 0;
    static mut V250_ARG: u64 = 0;
    static mut STR_ASSIGN: (u64, u64, u64) = (0, 0, 0);
    static mut DTOR_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_0810(_p1: usize, _id_ptr: usize) -> usize {
        HOLDER_PTR
    }
    unsafe extern "win64" fn mock_v250(_this: usize, arg: u64) {
        V250_ARG ^= arg;
    }
    unsafe extern "win64" fn mock_8920() -> u64 {
        0x1111
    }
    unsafe extern "win64" fn mock_8750(c: u64) -> u64 {
        c + 1
    }
    unsafe extern "win64" fn mock_3850(_c: u64, _key: u16) -> u8 {
        KEY_CALLS += 1;
        if KEY_CALLS == 2 || KEY_CALLS == 12 || KEY_CALLS == 14 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_8cc0(out: usize, _sv: usize, _desc: usize) {
        wr::<u64>(out, 0);
    }
    unsafe extern "win64" fn mock_9ee0(_map: usize, _cnt: u32, _id: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_9b50(_this: usize, sv: usize, pos: u64, count: u64) {
        STR_ASSIGN = (rd::<u64>(sv + 0x08), pos, count);
    }
    unsafe extern "win64" fn mock_2240(out: usize, _s: usize, _desc: usize) {
        wr::<u64>(out, 0);
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {
        DTOR_CALLS += 1;
    }

    let _m0 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_420e_8920, mock_8920 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_8750, mock_8750 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420e_3850, mock_3850 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_8cc0, mock_8cc0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4196_2240, mock_2240 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let mut vtbl = vec![0u8; 0x260];
    wr::<usize>(
        vtbl.as_mut_ptr() as usize + 0x250,
        mock_v250 as *const () as usize,
    );
    let mut obj_28: usize = vtbl.as_mut_ptr() as usize;
    let mut inner = [0u8; 0x30];
    wr::<usize>(
        inner.as_mut_ptr() as usize + 0x28,
        (&mut obj_28 as *mut usize) as usize,
    );
    let mut holder: usize = inner.as_mut_ptr() as usize;
    HOLDER_PTR = (&mut holder as *mut usize) as usize;

    let g_40f8 = exe::ptr::<u64>(0x1_43a3_40f8) as usize;
    let saved_40f8 = rd::<u64>(g_40f8);

    let run = |f: F, idx_11c: i32| -> ([u8; 5], u32, u64, (u64, u64, u64), u32) {
        KEY_CALLS = 0;
        V250_ARG = 0;
        STR_ASSIGN = (0, 0, 0);
        DTOR_CALLS = 0;
        wr::<u64>(g_40f8, 1);

        let mut slot_map = vec![0u16; 0x10000];
        let mut world_vtbl = [0u8; 0x30];
        wr::<usize>(world_vtbl.as_mut_ptr() as usize + 0x20, slot_map.as_mut_ptr() as usize);

        let mut desc_arr = [0u64; 4];
        desc_arr[0] = 0xaaaa_bbbb;
        desc_arr[1] = 7;

        let mut ent_tbl = [0u64; 4];
        ent_tbl[0] = 0;

        let mut off_tbl = [0u32; 4];
        off_tbl[0] = 0x10;

        let mut elem_str = [0u64; 6];
        let esp = elem_str.as_mut_ptr() as usize;
        wr::<u32>(esp, 0x0400_0004);
        wr::<u32>(esp + 0x04, 5);

        let mut comp_buf = vec![0u8; 0x200];
        let cbp = comp_buf.as_mut_ptr() as usize;
        wr::<i32>(cbp + 0x10 + 0x11c, idx_11c);
        wr::<usize>(cbp + 0x10 + 0x120, esp);
        wr::<u32>(cbp + 0x10 + 0x128, 2);

        let mut world = vec![0u8; 0x1_9000];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, world_vtbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x08, desc_arr.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x30, ent_tbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x88, cbp);
        wr::<usize>(wp + (0xc03 * 0x20) + 0x40, off_tbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x1_8098, 0x1234);
        wr::<u32>(wp + 0x1_80ac, 1);

        let mut p1 = [0u8; 0x30];
        let mut p2: usize = wp;
        f(p1.as_mut_ptr() as usize, (&mut p2 as *mut usize) as usize);

        let flags = rd::<[u8; 5]>(cbp + 0x10 + 0x5d);
        (flags, KEY_CALLS, V250_ARG, STR_ASSIGN, DTOR_CALLS)
    };

    for idx in [0i32, -1] {
        let o = run(f_o, idx);
        let n = run(f_n, idx);
        wr::<u64>(g_40f8, saved_40f8);
        crate::diff_eq!(o, n, "141960f50 idx={}", idx);
    }
    Ok(2)
}

pub unsafe fn diff_bigfish_141961470(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HOLDER_PTR: usize = 0;
    static mut EVT_LOG: Vec<[u32; 6]> = Vec::new();

    unsafe extern "win64" fn mock_0810(_p1: usize, _id_ptr: usize) -> usize {
        HOLDER_PTR
    }
    unsafe extern "win64" fn mock_get_w(_this: usize) -> u32 {
        1920
    }
    unsafe extern "win64" fn mock_get_h(_this: usize) -> u32 {
        1080
    }
    unsafe extern "win64" fn mock_send_evt(_this: usize, evt: usize, _a3: u64, _a4: u64) {
        EVT_LOG.push(rd::<[u32; 6]>(evt));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;

    let mut vtbl = [0u8; 0x100];
    let vp = vtbl.as_mut_ptr() as usize;
    wr::<usize>(vp + 0x68, mock_get_w as *const () as usize);
    wr::<usize>(vp + 0x70, mock_get_h as *const () as usize);
    wr::<usize>(vp + 0xe8, mock_send_evt as *const () as usize);

    let mut obj_28: usize = vp;
    let mut inner = [0u8; 0x30];
    wr::<usize>(inner.as_mut_ptr() as usize + 0x28, (&mut obj_28 as *mut usize) as usize);
    let mut holder: usize = inner.as_mut_ptr() as usize;
    HOLDER_PTR = (&mut holder as *mut usize) as usize;

    let run = |f: F, b_flag: u8, wheel: f32| -> (u8, i32, i32, u32, Vec<[u32; 6]>) {
        EVT_LOG.clear();
        let mut p1 = [0u8; 0x2b0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x18, 5);
        wr::<u8>(p1p + 0x298, 0);
        wr::<i32>(p1p + 0x29c, 100);
        wr::<i32>(p1p + 0x2a0, 200);
        wr::<f32>(p1p + 0x2a4, 1.0);

        let mut p2 = [0u8; 0x10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<f32>(p2p, 0.25);
        wr::<f32>(p2p + 0x04, 0.5);
        wr::<f32>(p2p + 0x08, wheel);
        wr::<u8>(p2p + 0x0c, b_flag);

        f(p1p, p2p);
        (
            rd::<u8>(p1p + 0x298),
            rd::<i32>(p1p + 0x29c),
            rd::<i32>(p1p + 0x2a0),
            rd::<u32>(p1p + 0x2a4),
            EVT_LOG.clone(),
        )
    };

    for (b_flag, wheel) in [(1u8, 2.5f32), (0, 1.0)] {
        let o = run(f_o, b_flag, wheel);
        let n = run(f_n, b_flag, wheel);
        crate::diff_eq!(o, n, "141961470 b={} w={}", b_flag, wheel);
    }
    Ok(2)
}

pub unsafe fn diff_bigfish_141961620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: (u32, u32, u32, u64, u64, u64) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_8cc0(out: usize, _sv: usize, _desc: usize) {
        wr::<u64>(out, 0);
    }
    unsafe extern "win64" fn mock_9ee0(_map: usize, _cnt: u32, _id: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_ae90(out: usize, _comp: usize) -> usize {
        wr::<u64>(out, 0x0400_0004);
        wr::<u8>(out + 0x08, 0);
        out
    }
    unsafe extern "win64" fn mock_6ff0(_s1: usize, _s2: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {
        CALL_TRACE.0 += 1;
    }
    unsafe extern "win64" fn mock_a9a0(_comp: usize, _sv: usize) -> i32 {
        3
    }
    unsafe extern "win64" fn mock_9270(_sv: usize, _comp: usize, _desc: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_0a90(_item: u64, _obj: usize, _p1: usize, _p280: usize, _w: u64) {
        CALL_TRACE.1 += 1;
    }
    unsafe extern "win64" fn mock_37e0(_p280: usize, _p1: usize, _w: u64) {
        CALL_TRACE.2 += 1;
    }
    unsafe extern "win64" fn mock_dec0(_g: usize) {
        CALL_TRACE.0 += 10;
    }
    unsafe extern "win64" fn mock_efc0(_g: usize, rdx: u64, r8: u64, r9: u64) {
        CALL_TRACE.3 = rdx;
        CALL_TRACE.4 = r8;
        CALL_TRACE.5 = r9;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8cc0, mock_8cc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41df_a9a0, mock_a9a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41df_9270, mock_9270 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_418a_0a90, mock_0a90 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_41df_37e0, mock_37e0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_41a0_dec0, mock_dec0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_41a0_efc0, mock_efc0 as *const ())?;

    let g_d850 = exe::ptr::<usize>(0x1_4397_d850) as usize;
    let saved_d850 = rd::<usize>(g_d850);

    let run = |f: F, init_260: i32| -> (i32, u8, u8, (u32, u32, u32, u64, u64, u64)) {
        CALL_TRACE = (0, 0, 0, 0, 0, 0);

        let mut g_buf = [0u8; 0x130];
        wr::<usize>(g_d850, g_buf.as_mut_ptr() as usize);

        let mut slot_map = vec![0u16; 0x10000];
        let mut world_vtbl = [0u8; 0x30];
        wr::<usize>(world_vtbl.as_mut_ptr() as usize + 0x20, slot_map.as_mut_ptr() as usize);

        let mut desc_arr = [0x1111u64, 5];
        let mut ent_tbl = [0u64; 2];
        let mut off_tbl = [0x10u32; 2];
        let mut comp_buf = vec![0u8; 0x200];
        let mut arr_580e0 = vec![0x9999_0001u64; 0x10000];

        let mut world = vec![0u8; 0x5_9000];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, world_vtbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x08, desc_arr.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x30, ent_tbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x88, comp_buf.as_mut_ptr() as usize);
        wr::<usize>(wp + (0xc03 * 0x20) + 0x40, off_tbl.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x5_80e0, arr_580e0.as_mut_ptr() as usize);

        let mut items = [0x5555u64, 0x6666];
        let mut p1 = [0u8; 0x2c0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x18, -1);
        wr::<usize>(p1p + 0x250, items.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x258, 2);
        wr::<i32>(p1p + 0x260, init_260);
        wr::<u8>(p1p + 0x2ac, 1);

        let mut p2: usize = wp;
        f(p1p, (&mut p2 as *mut usize) as usize);

        (
            rd::<i32>(p1p + 0x260),
            rd::<u8>(p1p + 0x2ac),
            g_buf[0x120],
            CALL_TRACE,
        )
    };

    for init_st in [5i32, 3] {
        let o = run(f_o, init_st);
        let n = run(f_n, init_st);
        wr::<usize>(g_d850, saved_d850);
        crate::diff_eq!(o, n, "141961620 st={}", init_st);
    }
    Ok(2)
}

pub unsafe fn diff_bigfish_141961db0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HOLDER_PTR: usize = 0;
    static mut SUB_LOG: (u32, u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_0810(_p1: usize, _id_ptr: usize) -> usize {
        HOLDER_PTR
    }
    unsafe extern "win64" fn mock_v1e0(_this: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_ve8(_this: usize, _evt: usize, _a3: u64, _a4: u64) {
        SUB_LOG.0 += 1;
    }
    unsafe extern "win64" fn mock_v10(_this: usize, _obj: usize) {
        SUB_LOG.1 += 1;
    }
    unsafe extern "win64" fn mock_1470(_p1: usize, _item: usize) {
        SUB_LOG.2 += 1;
    }
    unsafe extern "win64" fn mock_89c0(_p1: usize, _p2: usize, _p3: usize) {
        SUB_LOG.3 += 1;
    }
    unsafe extern "win64" fn mock_0f50(_p1: usize, _p2: usize) {
        SUB_LOG.4 += 1;
    }
    unsafe extern "win64" fn mock_69d0(_p268: usize, _obj: usize, _w: u64) {
        SUB_LOG.5 += 1;
    }
    unsafe extern "win64" fn mock_1620(_p1: usize, _p2: usize) {
        SUB_LOG.5 += 10;
    }
    unsafe extern "win64" fn mock_0ba0(_p1: usize, _p2: usize) {
        SUB_LOG.5 += 100;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4196_1470, mock_1470 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41fc_89c0, mock_89c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4196_0f50, mock_0f50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41df_69d0, mock_69d0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4196_1620, mock_1620 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4196_0ba0, mock_0ba0 as *const ())?;

    let mut vtbl = [0u8; 0x200];
    let vp = vtbl.as_mut_ptr() as usize;
    wr::<usize>(vp + 0xe8, mock_ve8 as *const () as usize);
    wr::<usize>(vp + 0x1e0, mock_v1e0 as *const () as usize);

    let mut obj_28: usize = vp;
    let mut inner = [0u8; 0x30];
    wr::<usize>(inner.as_mut_ptr() as usize + 0x28, (&mut obj_28 as *mut usize) as usize);
    let mut holder: usize = inner.as_mut_ptr() as usize;
    HOLDER_PTR = (&mut holder as *mut usize) as usize;

    let mut elem_vtbl = [0u8; 0x20];
    wr::<usize>(elem_vtbl.as_mut_ptr() as usize + 0x10, mock_v10 as *const () as usize);
    let mut elem_obj: usize = elem_vtbl.as_mut_ptr() as usize;
    let mut elem_arr: [usize; 1] = [(&mut elem_obj as *mut usize) as usize];

    let mut run = |f: F, vec_cnt: u32| -> (u32, u8, u64, u8, u32, (u32, u32, u32, u32, u32, u32)) {
        SUB_LOG = (0, 0, 0, 0, 0, 0);

        let mut evt_items = [0x1111u64; 4];
        let mut r14_vec = [0u8; 0x10];
        let rvp = r14_vec.as_mut_ptr() as usize;
        wr::<usize>(rvp, evt_items.as_mut_ptr() as usize);
        wr::<u32>(rvp + 0x08, vec_cnt);

        let mut cebc_val: u32 = rd::<u32>(exe::ptr::<u32>(0x1_43a2_9ea8) as usize);
        let mut arr_580e0 = vec![(&mut cebc_val as *mut u32) as usize; 0x10000];
        let idx_030c = rd::<u16>(exe::ptr::<u16>(0x1_43c9_030c) as usize) as usize;
        arr_580e0[idx_030c] = rvp;

        let mut w3 = vec![0u8; 0x5_8100];
        wr::<usize>(w3.as_mut_ptr() as usize + 0x5_80e0, arr_580e0.as_mut_ptr() as usize);

        let mut p1 = [0u8; 0x2b0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x18, 2);
        wr::<usize>(p1p + 0x240, elem_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x248, 1);
        wr::<u8>(p1p + 0x298, 1);
        wr::<u64>(p1p + 0x29c, 0x0010_0000_0020);
        wr::<u32>(p1p + 0x2a8, 7);

        let mut p2_val: u64 = 0xaaaa_bbbb;
        let mut p3_val: usize = w3.as_mut_ptr() as usize;

        f(
            p1p,
            (&mut p2_val as *mut u64) as usize,
            (&mut p3_val as *mut usize) as usize,
        );
        (
            rd::<u32>(p1p + 0x2a8),
            rd::<u8>(p1p + 0x298),
            rd::<u64>(p1p + 0x29c),
            rd::<u8>(p1p + 0x130),
            rd::<u32>(rvp + 0x08),
            SUB_LOG,
        )
    };

    for cnt in [0u32, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141961db0 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1419df070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut INNER_OBJ: [u8; 0xe0] = [0; 0xe0];

    unsafe extern "win64" fn mock_43c0(out: usize, id: u32) -> usize {
        wr::<u64>(out, id as u64);
        wr::<u64>(out + 0x08, (id as u64) + 0x5000);
        out
    }
    unsafe extern "win64" fn mock_7df0(_hp: usize) -> usize {
        INNER_OBJ[0xd8] = 1;
        core::ptr::addr_of_mut!(INNER_OBJ) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_419e_43c0, mock_43c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4021_7df0, mock_7df0 as *const ())?;

    let run = |f: F, fval: f32| -> (u64, [u8; 5]) {
        let mut elems = vec![0u8; 5 * 0xf0];
        let ep = elems.as_mut_ptr() as usize;
        for (i, mode) in [0u32, 1, 3, 4, 6].into_iter().enumerate() {
            let base = ep + i * 0xf0;
            wr::<u32>(base + 0x48, (i as u32) + 10);
            wr::<i32>(base + 0x54, 1);
            wr::<u32>(base + 0x60, mode);
            wr::<u8>(base + 0x68, 0xff);
        }

        let mut b0 = vec![0u8; 2 * 0x38];
        b0[0x38 + 0x1e] = 1;

        let mut b8 = vec![0u8; 2 * 0x10];
        wr::<f32>(b8.as_mut_ptr() as usize + 0x10, fval);

        let mut b18 = vec![0u8; 2 * 0x198];
        let b18p = b18.as_mut_ptr() as usize;
        wr::<usize>(b18p + 0x198 + 0xa8, ep);
        wr::<u32>(b18p + 0x198 + 0xb0, 5);

        let mut p1: [usize; 6] = [
            b0.as_mut_ptr() as usize,
            b8.as_mut_ptr() as usize,
            0,
            b18p,
            0,
            1,
        ];

        let ret = f(p1.as_mut_ptr() as usize);
        let out_flags = [
            rd::<u8>(ep + 0x68),
            rd::<u8>(ep + 0xf0 + 0x68),
            rd::<u8>(ep + 2 * 0xf0 + 0x68),
            rd::<u8>(ep + 3 * 0xf0 + 0x68),
            rd::<u8>(ep + 4 * 0xf0 + 0x68),
        ];
        (ret, out_flags)
    };

    for fval in [0.5f32, 1.5] {
        let o = run(f_o, fval);
        let n = run(f_n, fval);
        crate::diff_eq!(o, n, "1419df070 fval={}", fval);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1419e43c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_ff70(out: usize, src: usize) {
        wr::<u64>(out, rd::<u64>(src));
        wr::<u64>(out + 0x08, 0x9999);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4034_ff70, mock_ff70 as *const ())?;

    let g_d9e0 = exe::ptr::<usize>(0x1_4397_d9e0) as usize;
    let saved_d9e0 = rd::<usize>(g_d9e0);

    let run = |f: F, target_hash: u32| -> [u8; 0x18] {
        let s0 = b"\0";
        let s1 = b"AbC\0";
        let mut entries = vec![0u8; 2 * 0xf8];
        let ep = entries.as_mut_ptr() as usize;
        wr::<usize>(ep, s0.as_ptr() as usize);
        wr::<usize>(ep + 0xf8, s1.as_ptr() as usize);

        let vals: [u64; 2] = [0x1111_2222, 0x3333_4444];
        let mut mgr = vec![0u8; 0x220];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<i32>(mp + 0x204, 2);
        wr::<usize>(mp + 0x208, ep);
        wr::<usize>(mp + 0x210, vals.as_ptr() as usize);
        wr::<usize>(g_d9e0, mp);

        let mut out = [0x55u8; 0x18];
        let ret = f(out.as_mut_ptr() as usize, target_hash);
        assert_eq!(ret, out.as_mut_ptr() as usize);
        out
    };

    // Hash for "AbC" (lowercased to "abc"):
    let mut h_abc = 0x811c_9dc5u32;
    for &b in b"abc" {
        h_abc = ((b as u32) ^ h_abc).wrapping_mul(0x0100_0193);
    }

    for h in [0u32, h_abc, 0xdead_beef] {
        let o = run(f_o, h);
        let n = run(f_n, h);
        wr::<usize>(g_d9e0, saved_d9e0);
        crate::diff_eq!(o, n, "1419e43c0 h={:#x}", h);
    }
    Ok(3)
}

pub unsafe fn diff_std_1419eb8f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: (u64, u64, u64, u64) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_d4a0(out: usize, rdx: u64, _p3: usize) -> usize {
        wr::<i32>(out, if rdx == u64::MAX { -1 } else { 7 });
        out
    }
    unsafe extern "win64" fn mock_a9a0(_p3: usize, id: u64) -> usize {
        (id as usize).wrapping_add(0x1000)
    }
    unsafe extern "win64" fn mock_c390(p1: usize, p2: usize, comp: usize, p5: u64) {
        CALL_LOG.0 = rd::<u64>(p1) ^ rd::<u64>(p2);
        CALL_LOG.1 = comp as u64;
        CALL_LOG.2 = p5;
    }
    unsafe extern "win64" fn mock_bcd0(p1: usize, p2: usize, comp: usize) {
        CALL_LOG.3 = rd::<u64>(p1 + 0x28) ^ rd::<u64>(p2 + 0x10) ^ (comp as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_d4a0, mock_d4a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_403e_a9a0, mock_a9a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_419e_c390, mock_c390 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_419e_bcd0, mock_bcd0 as *const ())?;

    static mut STATE_ARR: [u64; 6] = [0; 6];

    let run = |f: F, disabled: u64| -> ([u64; 6], (u64, u64, u64, u64)) {
        CALL_LOG = (0, 0, 0, 0);

        STATE_ARR = [0x2222_3333_4444_5555u64; 6];
        let mut p1: [u64; 6] = [
            core::ptr::addr_of_mut!(STATE_ARR) as usize as u64,
            0x100,
            0x200,
            disabled,
            0x400,
            1,
        ];
        let mut p2: [u64; 3] = [0xaa, 0xbb, 0xcc];
        let mut p3: [u64; 3] = [0xdd, 0xee, 0xff];
        let mut p4 = [0u8; 0x20];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<i32>(p4p, 1);
        wr::<u64>(p4p + 0x0c, 0x42);

        f(
            p1.as_mut_ptr() as usize,
            p2.as_mut_ptr() as usize,
            p3.as_mut_ptr() as usize,
            p4p,
            0x9876_5432,
        );
        (STATE_ARR, CALL_LOG)
    };

    for dis in [1u64, 0] {
        let o = run(f_o, dis);
        let n = run(f_n, dis);
        crate::diff_eq!(o, n, "1419eb8f0 dis={}", dis);
    }
    Ok(2)
}

pub unsafe fn diff_std_1419ec390(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST: [u16; 1] = [0];
    static mut FN_BUF: [u64; 14] = [0; 14];
    static mut FN_VTBL: [usize; 6] = [0; 6];
    static mut VEC_BUF: [u64; 8] = [0; 8];
    static mut RAY_BLOCKED: u8 = 0;

    unsafe extern "win64" fn mock_fn_dtor(_this: usize, _dealloc: u8) {}
    unsafe extern "win64" fn mock_5010(out: usize, _cam: usize) {
        wr::<[f32; 4]>(out, [1.0, 0.0, 0.0, 0.0]);
    }
    unsafe extern "win64" fn mock_7c10(out: usize, storage: usize, _tail: usize) {
        let beg = core::ptr::addr_of!(ARCH_LIST) as usize;
        wr::<u64>(out, storage as u64);
        wr::<u64>(out + 0x08, beg as u64);
        wr::<u64>(out + 0x10, (beg + 2) as u64);
    }
    unsafe extern "win64" fn mock_e2f0(out: usize, _storage: usize, _arch: u32) {
        wr::<[u64; 4]>(out, [0x100, 0x200, 0x300, 0x400]);
    }
    unsafe extern "win64" fn mock_4d64(_sz: u64) -> usize {
        core::ptr::addr_of_mut!(FN_BUF) as usize
    }
    unsafe extern "win64" fn mock_a380(src: usize, dst: usize) {
        FN_VTBL[4] = mock_fn_dtor as *const () as usize;
        let p = rd::<usize>(src + 0x38);
        if p != 0 {
            wr::<usize>(p, core::ptr::addr_of!(FN_VTBL) as usize);
        }
        wr::<usize>(dst + 0x38, p);
        wr::<usize>(src + 0x38, 0);
    }
    unsafe extern "win64" fn mock_2fe0(
        hit_out: usize,
        _scene: u64,
        _start: usize,
        _end: usize,
        _opts: usize,
    ) {
        wr::<u8>(hit_out + 0x70, RAY_BLOCKED);
    }
    unsafe extern "win64" fn mock_eba0(vec_hdr: usize, new_cap: u64) {
        wr::<usize>(vec_hdr, core::ptr::addr_of_mut!(VEC_BUF) as usize);
        wr::<u32>(vec_hdr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4027_5010, mock_5010 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4046_e2f0, mock_e2f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4183_4d64, mock_4d64 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401f_a380, mock_a380 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e9_2fe0, mock_2fe0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;

    let run = |f: F, blocked: u8| -> (u32, [u64; 2]) {
        RAY_BLOCKED = blocked;
        VEC_BUF = [0; 8];

        let mut chunk = vec![0u8; 0x600];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x10, 0x1001);
        wr::<u64>(cp + 0x18, 0x1002);

        // Entity 0 at col0 + 0x00: within distance, passes angle & raycast
        wr::<f32>(cp + 0x100, 100.0); // max dist_sq
        wr::<f32>(cp + 0x104, 0.0); // min dist_sq
        wr::<f32>(cp + 0x108, 3.14); // max angle2
        wr::<u8>(cp + 0x10c, 1);
        wr::<f32>(cp + 0x110, 100.0); // max dy_sq
        wr::<u8>(cp + 0x114, 1);
        wr::<f32>(cp + 0x118, 3.14); // max angle1
        wr::<u8>(cp + 0x11c, 1);

        // Entity 1 at col0 + 0x28: fails distance check (max dist_sq = 0.01)
        wr::<f32>(cp + 0x128, 0.01);

        // WorldTransforms at col2 (0x300):
        wr::<[f32; 4]>(cp + 0x300, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(cp + 0x310, [0.0, 0.0, 2.0, 1.0]);
        wr::<[f32; 4]>(cp + 0x320, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(cp + 0x330, [0.0, 0.0, 10.0, 1.0]);

        let mut storage = vec![0u8; 0x1_0100];
        let sp = storage.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x40 + 0x48, cp);
        wr::<u32>(sp + 0x40 + 0x1_0048, 2);

        let mut state_vec = [0u8; 0x18];
        let mut ent_wt = [0.0f32; 8];
        let mut cc_inner = [0u8; 0x30];
        wr::<f32>(cc_inner.as_mut_ptr() as usize + 0x20, 1.8);
        let cc_ptr: usize = cc_inner.as_mut_ptr() as usize;
        let cc_comp: [usize; 3] = [0, (&cc_ptr as *const usize) as usize, 0];

        let mut p1: [usize; 6] = [
            state_vec.as_mut_ptr() as usize,
            ent_wt.as_mut_ptr() as usize,
            cc_comp.as_ptr() as usize,
            0,
            0,
            0,
        ];
        let mut p2: [usize; 3] = [sp, 0, 0];
        let mut cam = [0u8; 0x40];
        wr::<[f32; 3]>(cam.as_mut_ptr() as usize + 0x24, [0.0, 0.0, 0.0]);
        let mut scene: u64 = 0x7777;

        f(
            p1.as_mut_ptr() as usize,
            p2.as_mut_ptr() as usize,
            cam.as_mut_ptr() as usize,
            (&mut scene as *mut u64) as usize,
        );

        let out_len = rd::<u32>(state_vec.as_ptr() as usize + 0x08);
        (out_len, [VEC_BUF[0], VEC_BUF[1]])
    };

    for blocked in [0u8, 1] {
        let o = run(f_o, blocked);
        let n = run(f_n, blocked);
        crate::diff_eq!(o, n, "1419ec390 blocked={}", blocked);
    }
    Ok(2)
}
