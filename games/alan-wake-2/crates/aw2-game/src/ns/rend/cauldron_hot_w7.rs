//! Decompiled hot functions shard W7 (`0x141834ee8..0x1424d0470`).

use crate::exe::{self, rd, wr};

/// `global_141834ee8` (`eh_vector_destructor_iterator`)
/// Original: 0x141834ee8 (107 bytes, Throws::Caught)
pub unsafe extern "win64" fn global_141834ee8(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut cur = param_1.wrapping_add(param_2.wrapping_mul(param_3));
    let mut rem = param_3;
    while rem != 0 {
        rem = rem.wrapping_sub(1);
        cur = cur.wrapping_sub(param_2);
        exe::call_va!(param_4, (cur));
    }
}

/// `global_141834fc0` (`eh_vector_constructor_iterator`)
/// Original: 0x141834fc0 (112 bytes, Throws::Caught)
pub unsafe extern "win64" fn global_141834fc0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    _param_5: usize,
) {
    let mut cur = param_1;
    let mut i: usize = 0;
    while i != param_3 {
        exe::call_va!(param_4, (cur));
        cur = cur.wrapping_add(param_2);
        i = i.wrapping_add(1);
    }
}

/// `global_141835030` (`eh_vector_copy_constructor_iterator`)
/// Original: 0x141835030 (115 bytes, Throws::Caught)
pub unsafe extern "win64" fn global_141835030(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    _param_6: usize,
) {
    let mut dst_cur = param_1;
    let mut src_cur = param_2;
    let mut i: usize = 0;
    while i != param_4 {
        exe::call_va!(param_5, (dst_cur, src_cur));
        dst_cur = dst_cur.wrapping_add(param_3);
        src_cur = src_cur.wrapping_add(param_3);
        i = i.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn set_hud_flag(map_addr: usize, key_slot: &mut usize, str_va: u64, val: u8) {
    *key_slot = exe::ptr::<u8>(str_va) as usize;
    let out = exe::call_va!(
        0x1_4024_cb30,
        (map_addr, (key_slot as *mut usize) as usize) -> usize
    );
    wr::<u8>(out, val);
}

#[inline(always)]
unsafe fn eq_hud_mode(l58: usize, dat_va: u64) -> bool {
    exe::call_va!(0x1_401e_6ff0, (l58, exe::ptr::<u8>(dat_va) as usize) -> u8) != 0
}

/// `unknown_1418937b0`
/// Original: 0x1418937b0 (3072 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1418937b0(param_1: usize, param_2: usize, param_3: usize) {
    const S_CROSSHAIR: u64 = 0x1_4309_9910;
    const S_ITEM_ESCAPE: u64 = 0x1_4309_9920;
    const S_GRAPPLE_ESCAPE: u64 = 0x1_4309_9930;
    const S_INTERACTS: u64 = 0x1_4309_9940;
    const S_LAMP: u64 = 0x1_4309_994c;
    const S_INSPECT_OVERLAY: u64 = 0x1_4309_9958;
    const S_SCENE_PROMPT: u64 = 0x1_4309_9968;
    const S_CALL_TO_ACTION: u64 = 0x1_4309_9978;
    const S_DEBUG: u64 = 0x1_4309_9988;
    const S_GOALS_HUD: u64 = 0x1_4309_9990;
    const S_WEAPON_UPGRADES: u64 = 0x1_4309_99a0;
    const S_INSPECTS: u64 = 0x1_4309_99b0;
    const S_GAME_CURSOR: u64 = 0x1_4309_99c0;
    const S_MISSIONS: u64 = 0x1_4309_99d0;
    const S_TEXT_PROMPT: u64 = 0x1_4309_99e0;
    const S_ACTION_CALLOUT_GROUP: u64 = 0x1_4309_99f0;
    const S_HUD_WIDGETS: u64 = 0x1_4309_9a08;
    const S_MIND_PLACE_OVERLAY: u64 = 0x1_4309_9a18;
    const S_CONVERSATION_INDICATORS: u64 = 0x1_4309_9a30;
    const S_NOTIFICATIONS: u64 = 0x1_4309_9a48;
    const S_MODULE_REMINDERS: u64 = 0x1_4309_9a58;
    const S_FMT_HUD: u64 = 0x1_4309_9a68;
    const S_QUICK_SLOTS: u64 = 0x1_4309_9a80;
    const S_LIGHT_ARRAY_PROMPT: u64 = 0x1_4309_9a90;
    const S_GENERATOR: u64 = 0x1_4309_9aa8;
    const S_COMPUTER_INDICATOR: u64 = 0x1_4309_9ac8;
    const S_COMPUTER_PROMPT: u64 = 0x1_4309_9ae0;
    const S_LIGHT_SHIFTER: u64 = 0x1_4309_9af0;
    const S_SIGNAGE_OVERLAY: u64 = 0x1_4309_9b00;
    const S_LOCK_PUZZLE: u64 = 0x1_4309_9b10;
    const S_CASE_BOARD_DRAWER_OVERLAY: u64 = 0x1_4309_9b20;

    let mut local_58 = [0u64; 8];
    let l58 = local_58.as_mut_ptr() as usize;

    exe::call_va!(0x1_4189_43c0, (param_1));
    let _ = exe::call_va!(0x1_41df_ae90, (l58, param_3) -> usize);

    let map_addr = param_1.wrapping_add(0x08);
    let mut key_slot: usize = 0;

    if eq_hud_mode(l58, 0x1_43a1_0c40) {
        set_hud_flag(map_addr, &mut key_slot, S_GRAPPLE_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ITEM_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_HUD_WIDGETS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0ad8) {
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0e70) {
        set_hud_flag(map_addr, &mut key_slot, S_CROSSHAIR, 0);
        set_hud_flag(map_addr, &mut key_slot, S_DEBUG, 0);
        set_hud_flag(map_addr, &mut key_slot, S_GOALS_HUD, 0);
    } else if eq_hud_mode(l58, 0x1_43a1_0a38) {
        set_hud_flag(map_addr, &mut key_slot, S_DEBUG, 0);
        set_hud_flag(map_addr, &mut key_slot, S_GOALS_HUD, 0);
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GRAPPLE_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ITEM_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CONVERSATION_INDICATORS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GENERATOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LOCK_PUZZLE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_SIGNAGE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LIGHT_ARRAY_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_QUICK_SLOTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0950) {
        set_hud_flag(map_addr, &mut key_slot, S_LAMP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GRAPPLE_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ITEM_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_HUD_WIDGETS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CONVERSATION_INDICATORS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CALL_TO_ACTION, 1);
        set_hud_flag(map_addr, &mut key_slot, S_SCENE_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MODULE_REMINDERS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GENERATOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_SIGNAGE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LIGHT_ARRAY_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LIGHT_SHIFTER, 1);
        set_hud_flag(map_addr, &mut key_slot, S_QUICK_SLOTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0900) {
        set_hud_flag(map_addr, &mut key_slot, S_LAMP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GRAPPLE_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ITEM_ESCAPE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CONVERSATION_INDICATORS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CALL_TO_ACTION, 1);
        set_hud_flag(map_addr, &mut key_slot, S_SCENE_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MODULE_REMINDERS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GENERATOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LOCK_PUZZLE, 1);
        set_hud_flag(map_addr, &mut key_slot, S_SIGNAGE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LIGHT_ARRAY_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LIGHT_SHIFTER, 1);
        set_hud_flag(map_addr, &mut key_slot, S_QUICK_SLOTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0d08)
        || eq_hud_mode(l58, 0x1_43a1_0e98)
        || eq_hud_mode(l58, 0x1_43a1_0fb0)
        || eq_hud_mode(l58, 0x1_43a1_0b78)
        || eq_hud_mode(l58, 0x1_43a1_0ab0)
    {
        set_hud_flag(map_addr, &mut key_slot, S_CROSSHAIR, 0);
        set_hud_flag(map_addr, &mut key_slot, S_LAMP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_HUD_WIDGETS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MISSIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MODULE_REMINDERS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_QUICK_SLOTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0c68) {
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MIND_PLACE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GAME_CURSOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INSPECTS, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_10c8) {
        set_hud_flag(map_addr, &mut key_slot, S_INTERACTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MIND_PLACE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GAME_CURSOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INSPECTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_WEAPON_UPGRADES, 1);
        set_hud_flag(map_addr, &mut key_slot, S_CASE_BOARD_DRAWER_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INSPECT_OVERLAY, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0d80) {
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_MIND_PLACE_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GAME_CURSOR, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0928) || eq_hud_mode(l58, 0x1_43a1_0a88) {
        set_hud_flag(map_addr, &mut key_slot, S_CROSSHAIR, 0);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_GAME_CURSOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INSPECTS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_INSPECT_OVERLAY, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
        set_hud_flag(map_addr, &mut key_slot, S_LOCK_PUZZLE, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0c90)
        || eq_hud_mode(l58, 0x1_43a1_0f38)
        || eq_hud_mode(l58, 0x1_43a1_0fd8)
    {
        set_hud_flag(map_addr, &mut key_slot, S_GAME_CURSOR, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_1078) {
        set_hud_flag(map_addr, &mut key_slot, S_CROSSHAIR, 0);
        set_hud_flag(map_addr, &mut key_slot, S_COMPUTER_PROMPT, 1);
        set_hud_flag(map_addr, &mut key_slot, S_COMPUTER_INDICATOR, 1);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
        set_hud_flag(map_addr, &mut key_slot, S_ACTION_CALLOUT_GROUP, 1);
    } else if eq_hud_mode(l58, 0x1_43a1_0dd0) {
        set_hud_flag(map_addr, &mut key_slot, S_CROSSHAIR, 0);
        set_hud_flag(map_addr, &mut key_slot, S_TEXT_PROMPT, 0);
        set_hud_flag(map_addr, &mut key_slot, S_DEBUG, 0);
        set_hud_flag(map_addr, &mut key_slot, S_GOALS_HUD, 0);
        set_hud_flag(map_addr, &mut key_slot, S_NOTIFICATIONS, 1);
    }

    let head = rd::<usize>(map_addr);
    let mut rbx = rd::<usize>(head);
    if rd::<u8>(rbx + 0x19) == 0 {
        let fmt_ptr = exe::ptr::<u8>(S_FMT_HUD) as usize as u64;
        loop {
            let mut fmt_span: [u64; 2] = [fmt_ptr, 0x14];
            let formatted = exe::call_va!(
                0x1_4024_cc10,
                (fmt_span.as_mut_ptr() as usize, rbx.wrapping_add(0x20)) -> usize
            );
            let mut slen: usize = 0;
            while rd::<u8>(formatted.wrapping_add(slen)) != 0 {
                slen += 1;
            }
            let hash = exe::call_va!(0x1_424b_16a0, (formatted, slen as u64) -> u64);
            let mut local_68: [u64; 2] = [0, hash];
            exe::call_va!(
                0x1_401e_acd0,
                (
                    param_2,
                    local_68.as_mut_ptr() as usize,
                    0usize,
                    rbx.wrapping_add(0x28),
                    0u32
                )
            );

            let right = rd::<usize>(rbx + 0x10);
            if rd::<u8>(right + 0x19) == 0 {
                rbx = right;
                let mut left = rd::<usize>(rbx);
                while rd::<u8>(left + 0x19) == 0 {
                    rbx = left;
                    left = rd::<usize>(left);
                }
            } else {
                let mut parent = rd::<usize>(rbx + 0x08);
                while rd::<u8>(parent + 0x19) == 0 && rbx == rd::<usize>(parent + 0x10) {
                    rbx = parent;
                    parent = rd::<usize>(parent + 0x08);
                }
                rbx = parent;
            }
            if rd::<u8>(rbx + 0x19) != 0 {
                break;
            }
        }
    }

    exe::call_va!(0x1_4240_ab10, (l58));
}

/// `r_14243d920`
/// Original: 0x14243d920 (628 bytes, Throws::No)
pub unsafe extern "win64" fn r_14243d920(param_1: usize) {
    let r14 = rd::<usize>(param_1 + 0x80);
    let ebp = rd::<u32>(r14 + 0x20) as u64;
    let rbx = r14.wrapping_add(0x28);

    let old_base = rd::<usize>(rbx);
    let old_end = old_base.wrapping_add((rd::<u32>(rbx + 0x08) as usize).wrapping_mul(0x88));
    exe::call_va!(0x1_413a_cb00, (param_1, old_base, old_end));
    wr::<u32>(rbx + 0x08, 0);

    let mut local_res8: usize = rbx;
    let cap = rd::<u32>(rbx + 0x0c) as u64;
    if ebp > cap {
        let grown = (cap >> 1).wrapping_add(cap);
        let new_cap = if grown < ebp { ebp } else { grown };
        exe::call_va!(0x1_413a_cc00, (rbx, new_cap));
        if ebp != 0 {
            let mut rax = rd::<usize>(rbx)
                .wrapping_add((rd::<u32>(rbx + 0x08) as usize).wrapping_mul(0x88))
                .wrapping_add(0x40);
            let mut rem = ebp;
            while rem != 0 {
                wr::<[u64; 17]>(rax - 0x40, [0; 17]);
                let ecx = (rd::<u32>(rax - 0x30) & 0xff00_0002) | 2;
                wr::<u32>(rax - 0x30, ecx);
                wr::<u8>(rax - 0x2d, 2);
                wr::<u32>(rax - 0x2c, 0);
                wr::<u8>(rax - 0x28, 0);
                wr::<u64>(rax - 0x18, 0);
                wr::<u64>(rax - 0x10, 0);
                wr::<u64>(rax - 0x08, 0);
                wr::<u64>(rax, 0);
                wr::<u64>(rax + 0x08, 0);
                wr::<u16>(rax + 0x10, 0x10);
                wr::<u64>(rax + 0x18, 0);
                wr::<u32>(rax + 0x20, 1);
                wr::<u64>(rax + 0x28, 0);
                wr::<u64>(rax + 0x30, 0);
                wr::<u64>(rax + 0x38, 0);
                rax = rax.wrapping_add(0x88);
                rem -= 1;
            }
        }
        wr::<u32>(rbx + 0x08, ebp as u32);
    } else if ebp != 0 {
        exe::call_va!(
            0x1_413a_bd80,
            ((&mut local_res8 as *mut usize) as usize, 0usize, ebp)
        );
        wr::<u32>(rbx + 0x08, ebp as u32);
    }

    let _ = crate::simd_ops::atomic_xchg_u32((r14 + 0x38) as u64, ebp as u32);

    let tid_or_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as u64;
    let g_320 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8320) as usize);
    let vtbl_320 = rd::<usize>(g_320);
    let u6 = exe::call_va!(rd::<usize>(vtbl_320 + 0x120), (g_320) -> u64);

    let p0 = rd::<usize>(param_1);
    let mut cur_cb = rd::<usize>(p0 + 0x4e8);
    let end_cb = cur_cb.wrapping_add((rd::<u32>(p0 + 0x4f0) as usize) * 8);
    while cur_cb != end_cb {
        let obj = rd::<usize>(cur_cb);
        let vtbl = rd::<usize>(obj);
        exe::call_va!(rd::<usize>(vtbl + 0x18), (obj));
        cur_cb = cur_cb.wrapping_add(8);
    }

    if ebp != 0 {
        let mut idx: u64 = 0;
        let mut off_18: usize = 0;
        let c70 = rd::<u128>(exe::ptr::<u8>(0x1_4335_4c70) as usize);
        while idx < ebp {
            let arr_18 = rd::<usize>(r14 + 0x18);
            let mut arg5 = [0u64; 9];
            let a5p = arg5.as_mut_ptr() as usize;
            wr::<u128>(a5p, c70);
            wr::<u64>(a5p + 0x10, 0);
            wr::<u64>(a5p + 0x18, 0);
            wr::<u32>(a5p + 0x20, 1);
            wr::<u32>(a5p + 0x24, 0x14);
            wr::<u64>(a5p + 0x28, idx);
            wr::<u64>(a5p + 0x30, tid_or_idx);
            wr::<u64>(a5p + 0x38, u6);
            wr::<u64>(a5p + 0x40, 0);

            let mut local_78 = [0u64; 2];
            let l78p = local_78.as_mut_ptr() as usize;
            wr::<u128>(l78p, rd::<u128>(arr_18.wrapping_add(off_18 + 0x08)));

            let _ = exe::call_va!(
                0x1_4247_dea0,
                (
                    rd::<usize>(param_1),
                    (&mut local_res8 as *mut usize) as usize,
                    l78p,
                    r14,
                    a5p
                ) -> usize
            );
            idx += 1;
            off_18 = off_18.wrapping_add(0x18);
        }
    }

    let p0_end = rd::<usize>(param_1);
    let mut cur_cb2 = rd::<usize>(p0_end + 0x4e8);
    let end_cb2 = cur_cb2.wrapping_add((rd::<u32>(p0_end + 0x4f0) as usize) * 8);
    while cur_cb2 != end_cb2 {
        let obj = rd::<usize>(cur_cb2);
        let vtbl = rd::<usize>(obj);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (obj));
        cur_cb2 = cur_cb2.wrapping_add(8);
    }
}

/// `r_142479a20` (`r::Loader_Adaptor_HotloadDataCache::vfunc_1`)
/// Original: 0x142479a20 (52 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_142479a20(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    param_4: u8,
) -> u8 {
    if (param_4 & 4) == 0 {
        let mut local_18 = rd::<[u64; 2]>(param_2);
        let ok = exe::call_va!(
            0x1_4247_9e50,
            (rd::<usize>(param_1 + 0x08), local_18.as_mut_ptr() as usize) -> u8
        );
        if ok != 0 {
            return 1;
        }
    }
    0
}

/// `std_1424d0470` (`std::_Func_impl_no_alloc<snd::SoundBankResource::requestLoad::<lambda_1>,void,r::LoadedData&&>::vfunc_2`)
/// Original: 0x1424d0470 (117 bytes, Throws::No)
pub unsafe extern "win64" fn std_1424d0470(param_1: usize, param_2: usize) {
    let mut local_68 = [0u64; 12];
    let rdi = exe::call_va!(
        0x1_4101_f7d0,
        (local_68.as_mut_ptr() as usize, param_2) -> usize
    );
    exe::call_va!(0x1_424d_0890, (rd::<u64>(param_1 + 0x08)));
    let g_2b8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
    exe::call_va!(0x1_4242_0ed0, (g_2b8, rd::<u64>(param_1 + 0x08)));
    let p30 = rd::<u64>(rdi + 0x30);
    if p30 != 0 {
        let p48 = rd::<usize>(rdi + 0x48);
        let r9d = rd::<u16>(rdi + 0x40) as u32;
        let vtbl = rd::<usize>(p48);
        let r8 = rd::<u64>(rdi + 0x38);
        exe::call_va!(rd::<usize>(vtbl + 0x10), (p48, p30, r8, r9d));
    }
    exe::call_va!(0x1_4240_ab10, (rdi));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_global_141834ee8(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DTOR_SEQ: u64 = 0;

    unsafe extern "win64" fn host_dtor(elem_ptr: usize) {
        let v = rd::<u64>(elem_ptr);
        DTOR_SEQ = DTOR_SEQ.wrapping_mul(131).wrapping_add(v);
        wr::<u64>(elem_ptr, v ^ 0xffff_0000);
    }

    let run = |f: F, count: usize| -> ([u64; 4], u64) {
        DTOR_SEQ = 0;
        let mut arr: [u64; 4] = [10, 20, 30, 40];
        f(
            arr.as_mut_ptr() as usize,
            8,
            count,
            host_dtor as *const () as usize,
        );
        (arr, DTOR_SEQ)
    };

    for cnt in [0usize, 1, 4] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141834ee8 cnt={}", cnt);
    }
    Ok(3)
}

pub unsafe fn diff_global_141834fc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CTOR_COUNTER: u64 = 0;

    unsafe extern "win64" fn host_ctor(elem_ptr: usize) {
        CTOR_COUNTER += 1;
        wr::<u64>(elem_ptr, 0x1000 + CTOR_COUNTER);
    }

    let run = |f: F, count: usize| -> ([u64; 4], u64) {
        CTOR_COUNTER = 0;
        let mut arr = [0u64; 4];
        f(
            arr.as_mut_ptr() as usize,
            8,
            count,
            host_ctor as *const () as usize,
            0,
        );
        (arr, CTOR_COUNTER)
    };

    for cnt in [0usize, 2, 4] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141834fc0 cnt={}", cnt);
    }
    Ok(3)
}

pub unsafe fn diff_global_141835030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut COPY_SEQ: u64 = 0;

    unsafe extern "win64" fn host_copy_ctor(dst_ptr: usize, src_ptr: usize) {
        let v = rd::<u64>(src_ptr);
        COPY_SEQ = COPY_SEQ.wrapping_mul(257).wrapping_add(v);
        wr::<u64>(dst_ptr, v.wrapping_add(0x500));
    }

    let run = |f: F, count: usize| -> ([u64; 4], u64) {
        COPY_SEQ = 0;
        let src: [u64; 4] = [11, 22, 33, 44];
        let mut dst = [0u64; 4];
        f(
            dst.as_mut_ptr() as usize,
            src.as_ptr() as usize,
            8,
            count,
            host_copy_ctor as *const () as usize,
            0,
        );
        (dst, COPY_SEQ)
    };

    for cnt in [0usize, 2, 4] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141835030 cnt={}", cnt);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1418937b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_DAT: usize = 0;
    static mut LOG_HASH: u64 = 0;
    static mut DUMMY_SLOT: u8 = 0;

    fn mix_log(v: u64) {
        unsafe {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_43c0(p1: usize) {
        mix_log(0x43c0 ^ (rd::<u32>(p1) as u64));
    }
    unsafe extern "win64" fn mock_ae90(l58: usize, p3: usize) -> usize {
        wr::<u64>(l58, rd::<u64>(p3));
        l58
    }
    unsafe extern "win64" fn mock_6ff0(_l58: usize, dat_ptr: usize) -> u8 {
        if dat_ptr == TARGET_DAT {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_cb30(_map: usize, key_ptr: usize) -> usize {
        let str_ptr = rd::<usize>(key_ptr);
        let b0 = rd::<u8>(str_ptr) as u64;
        let b1 = rd::<u8>(str_ptr + 1) as u64;
        let prev = DUMMY_SLOT as u64;
        mix_log((b0 << 16) | (b1 << 8) | prev);
        core::ptr::addr_of_mut!(DUMMY_SLOT) as usize
    }
    unsafe extern "win64" fn mock_cc10(fmt_span: usize, key_node: usize) -> usize {
        let fmt_len = rd::<u64>(fmt_span + 0x08);
        mix_log(fmt_len ^ rd::<u64>(key_node));
        exe::ptr::<u8>(0x1_4309_9910) as usize // "crosshair\0"
    }
    unsafe extern "win64" fn mock_16a0(_s: usize, len: u64) -> u64 {
        0x7777_0000 + len
    }
    unsafe extern "win64" fn mock_acd0(
        p2: usize,
        l68: usize,
        _r8: usize,
        val_ptr: usize,
        flag: u32,
    ) {
        let h = rd::<u64>(l68 + 0x08);
        let v = rd::<u8>(val_ptr) as u64;
        mix_log(rd::<u64>(p2) ^ h ^ (v << 8) ^ (flag as u64));
    }
    unsafe extern "win64" fn mock_ab10(l58: usize) {
        mix_log(0xab10 ^ rd::<u64>(l58));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4189_43c0, mock_43c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4024_cb30, mock_cb30 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4024_cc10, mock_cc10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401e_acd0, mock_acd0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, dat_va: u64| -> (u64, u8) {
        TARGET_DAT = if dat_va != 0 {
            exe::ptr::<u8>(dat_va) as usize
        } else {
            0
        };
        LOG_HASH = 0;
        DUMMY_SLOT = 9;

        // Build a 2-node RB-tree (n0 -> right child n1 -> head)
        let mut head = [0u8; 0x30];
        let mut n0 = [0u8; 0x30];
        let mut n1 = [0u8; 0x30];
        let hp = head.as_mut_ptr() as usize;
        let n0p = n0.as_mut_ptr() as usize;
        let n1p = n1.as_mut_ptr() as usize;

        wr::<usize>(hp, n0p); // leftmost = n0
        wr::<usize>(hp + 0x08, n0p); // root = n0
        wr::<usize>(hp + 0x10, n1p); // rightmost = n1
        wr::<u8>(hp + 0x19, 1); // is_nil = 1

        wr::<usize>(n0p, hp);
        wr::<usize>(n0p + 0x08, hp);
        wr::<usize>(n0p + 0x10, n1p);
        wr::<u8>(n0p + 0x19, 0);
        wr::<u64>(n0p + 0x20, 0x1111);
        wr::<u8>(n0p + 0x28, 1);

        wr::<usize>(n1p, hp);
        wr::<usize>(n1p + 0x08, n0p);
        wr::<usize>(n1p + 0x10, hp);
        wr::<u8>(n1p + 0x19, 0);
        wr::<u64>(n1p + 0x20, 0x2222);
        wr::<u8>(n1p + 0x28, 0);

        let mut p1 = [0u64; 3];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p, 0x55);
        wr::<usize>(p1p + 0x08, hp);

        let p2: u64 = 0x9999;
        let p3: u64 = 0xaaaa;
        f(
            p1p,
            (&p2 as *const u64) as usize,
            (&p3 as *const u64) as usize,
        );
        (LOG_HASH, DUMMY_SLOT)
    };

    for va in [
        0x1_43a1_0c40u64,
        0x1_43a1_0e70,
        0x1_43a1_0a38,
        0x1_43a1_0900,
        0x1_43a1_0e98,
        0x1_43a1_10c8,
        0x1_43a1_0a88,
        0x1_43a1_1078,
        0x1_43a1_0dd0,
        0,
    ] {
        let o = run(f_o, va);
        let n = run(f_n, va);
        crate::diff_eq!(o, n, "1418937b0 va={:#x}", va);
    }
    Ok(10)
}

pub unsafe fn diff_r_14243d920(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    fn trace_mix(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_cb00(_p1: usize, beg: usize, end: usize) {
        trace_mix(0xcb00 ^ (end.wrapping_sub(beg) as u64));
    }
    unsafe extern "win64" fn mock_cc00(vec_ptr: usize, new_cap: u64) {
        trace_mix(0xcc00 ^ new_cap);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_bd80(res8_ptr: usize, _rdx: usize, cnt: u64) {
        let rbx = rd::<usize>(res8_ptr);
        trace_mix(0xbd80 ^ (rd::<u32>(rbx + 0x0c) as u64) ^ cnt);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        7
    }
    unsafe extern "win64" fn mock_v120(_this: usize) -> u64 {
        0x4242_0000
    }
    unsafe extern "win64" fn mock_cb_begin(this: usize) {
        trace_mix(0x1800 ^ rd::<u64>(this + 0x08));
    }
    unsafe extern "win64" fn mock_cb_end(this: usize) {
        trace_mix(0x2000 ^ rd::<u64>(this + 0x08));
    }
    unsafe extern "win64" fn mock_dea0(
        p0: usize,
        res8_ptr: usize,
        l78_ptr: usize,
        _r14: usize,
        a5_ptr: usize,
    ) -> usize {
        let l78 = rd::<[u64; 2]>(l78_ptr);
        let a5 = rd::<[u64; 9]>(a5_ptr);
        wr::<u64>(res8_ptr, a5[5]);
        trace_mix(rd::<u64>(p0) ^ l78[0] ^ l78[1] ^ a5[1] ^ a5[4] ^ a5[5] ^ a5[6] ^ a5[7]);
        res8_ptr
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_cb00, mock_cb00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_cc00, mock_cc00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413a_bd80, mock_bd80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4247_dea0, mock_dea0 as *const ())?;

    let g_320 = exe::ptr::<usize>(0x1_43a0_8320) as usize;
    let s_320 = rd::<usize>(g_320);

    let mut vtbl_320 = [0usize; 40];
    vtbl_320[0x120 / 8] = mock_v120 as *const () as usize;
    let obj_320: [usize; 1] = [vtbl_320.as_ptr() as usize];
    wr::<usize>(g_320, obj_320.as_ptr() as usize);

    let mut cb_vtbl = [0usize; 6];
    cb_vtbl[0x18 / 8] = mock_cb_begin as *const () as usize;
    cb_vtbl[0x20 / 8] = mock_cb_end as *const () as usize;

    let run = |f: F, init_cap: u32, want_cnt: u32| -> (u64, [u8; 0x88 * 2], u32, u32, u32) {
        TRACE = 0;
        let mut elem_buf = [0x5au8; 0x88 * 2];
        let mut items_18 = [0u64; 6];
        items_18[1] = 0x1111_2222;
        items_18[2] = 0x3333_4444;
        items_18[4] = 0x5555_6666;
        items_18[5] = 0x7777_8888;

        let mut r14_buf = [0u8; 0x40];
        let r14p = r14_buf.as_mut_ptr() as usize;
        wr::<usize>(r14p + 0x18, items_18.as_ptr() as usize);
        wr::<u32>(r14p + 0x20, want_cnt);
        wr::<usize>(r14p + 0x28, elem_buf.as_mut_ptr() as usize);
        wr::<u32>(r14p + 0x30, 1);
        wr::<u32>(r14p + 0x34, init_cap);
        wr::<u32>(r14p + 0x38, 99);

        let cb_obj: [u64; 2] = [cb_vtbl.as_ptr() as usize as u64, 0xabcd];
        let cb_list: [usize; 1] = [cb_obj.as_ptr() as usize];

        let mut p0_buf = [0u8; 0x500];
        let p0p = p0_buf.as_mut_ptr() as usize;
        wr::<u64>(p0p, 0xfeed_0000);
        wr::<usize>(p0p + 0x4e8, cb_list.as_ptr() as usize);
        wr::<u32>(p0p + 0x4f0, 1);

        let mut p1_buf = [0u8; 0x90];
        let p1p = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1p, p0p);
        wr::<usize>(p1p + 0x80, r14p);

        f(p1p);
        (
            TRACE,
            elem_buf,
            rd::<u32>(r14p + 0x30),
            rd::<u32>(r14p + 0x34),
            rd::<u32>(r14p + 0x38),
        )
    };

    for (cap, cnt) in [(1u32, 2u32), (4, 2), (2, 0)] {
        let o = run(f_o, cap, cnt);
        let n = run(f_n, cap, cnt);
        wr::<usize>(g_320, s_320);
        crate::diff_eq!(o, n, "14243d920 cap={} cnt={}", cap, cnt);
        wr::<usize>(g_320, obj_320.as_ptr() as usize);
    }
    wr::<usize>(g_320, s_320);
    Ok(3)
}

pub unsafe fn diff_r_142479a20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9e50(inner: usize, pair_ptr: usize) -> u8 {
        let p = rd::<[u64; 2]>(pair_ptr);
        if inner == 0x1234 && p[0] == 0xaaaa && p[1] == 0xbbbb {
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4247_9e50, mock_9e50 as *const ())?;

    let p1: [usize; 2] = [0, 0x1234];
    for (p0, flags) in [(0xaaaau64, 0u8), (0xaaaa, 4), (0xcccc, 0)] {
        let pair: [u64; 2] = [p0, 0xbbbb];
        let o = f_o(p1.as_ptr() as usize, pair.as_ptr() as usize, 0, flags);
        let n = f_n(p1.as_ptr() as usize, pair.as_ptr() as usize, 0, flags);
        crate::diff_eq!(o, n, "142479a20 p0={:#x} flags={}", p0, flags);
    }
    Ok(3)
}

pub unsafe fn diff_std_1424d0470(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    fn log_mix(v: u64) {
        unsafe {
            CALL_LOG = CALL_LOG.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_f7d0(dst: usize, src: usize) -> usize {
        wr::<[u64; 12]>(dst, rd::<[u64; 12]>(src));
        dst
    }
    unsafe extern "win64" fn mock_0890(res_ptr: u64) {
        log_mix(0x0890 ^ res_ptr);
    }
    unsafe extern "win64" fn mock_0ed0(mgr: u64, res_ptr: u64) {
        log_mix(0x0ed0 ^ mgr ^ res_ptr);
    }
    unsafe extern "win64" fn mock_vfunc_10(this: usize, p30: u64, p38: u64, p40: u32) {
        log_mix((rd::<u64>(this + 0x08)) ^ p30 ^ p38 ^ (p40 as u64));
    }
    unsafe extern "win64" fn mock_ab10(rdi: usize) {
        log_mix(0xab10 ^ rd::<u64>(rdi));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4101_f7d0, mock_f7d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424d_0890, mock_0890 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_0ed0, mock_0ed0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let g_2b8 = exe::ptr::<u64>(0x1_43a0_82b8) as usize;
    let s_2b8 = rd::<u64>(g_2b8);
    wr::<u64>(g_2b8, 0x7777_8888);

    let mut vtbl = [0usize; 4];
    vtbl[2] = mock_vfunc_10 as *const () as usize;
    let alloc_obj: [u64; 2] = [vtbl.as_ptr() as usize as u64, 0x5555];

    let run = |f: F, has_buf: bool| -> u64 {
        CALL_LOG = 0;
        let p1: [u64; 2] = [0, 0x1234_5678];
        let mut p2 = [0u64; 12];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 0x9999);
        if has_buf {
            wr::<u64>(p2p + 0x30, 0xaaaa_bbbb);
            wr::<u64>(p2p + 0x38, 0x100);
            wr::<u16>(p2p + 0x40, 16);
            wr::<usize>(p2p + 0x48, alloc_obj.as_ptr() as usize);
        }
        f(p1.as_ptr() as usize, p2p);
        CALL_LOG
    };

    for has_buf in [true, false] {
        let o = run(f_o, has_buf);
        let n = run(f_n, has_buf);
        wr::<u64>(g_2b8, s_2b8);
        crate::diff_eq!(o, n, "1424d0470 has_buf={}", has_buf);
        wr::<u64>(g_2b8, 0x7777_8888);
    }
    wr::<u64>(g_2b8, s_2b8);
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4183_4ee8, "global_141834ee8", rend::cauldron_hot_w7::global_141834ee8 as *const (), Throws::Caught, rend::cauldron_hot_w7::diff_global_141834ee8),
// Replacement::verified_throws(0x1_4183_4fc0, "global_141834fc0", rend::cauldron_hot_w7::global_141834fc0 as *const (), Throws::Caught, rend::cauldron_hot_w7::diff_global_141834fc0),
// Replacement::verified_throws(0x1_4183_5030, "global_141835030", rend::cauldron_hot_w7::global_141835030 as *const (), Throws::Caught, rend::cauldron_hot_w7::diff_global_141835030),
// Replacement::verified_throws(0x1_4189_37b0, "unknown_1418937b0", rend::cauldron_hot_w7::unknown_1418937b0 as *const (), Throws::Caught, rend::cauldron_hot_w7::diff_unknown_1418937b0),
// Replacement::verified(0x1_4243_d920, "r_14243d920", rend::cauldron_hot_w7::r_14243d920 as *const (), rend::cauldron_hot_w7::diff_r_14243d920),
// Replacement::verified_throws(0x1_4247_9a20, "r_142479a20", rend::cauldron_hot_w7::r_142479a20 as *const (), Throws::Discard, rend::cauldron_hot_w7::diff_r_142479a20),
// Replacement::verified(0x1_424d_0470, "std_1424d0470", rend::cauldron_hot_w7::std_1424d0470 as *const (), rend::cauldron_hot_w7::diff_std_1424d0470),
