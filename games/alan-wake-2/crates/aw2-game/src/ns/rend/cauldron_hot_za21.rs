//! Decompiled `rend` hot functions shard ZA21 (`0x142299ad0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

const SLIDERS_AND_KNOBS: [(u64, u64, u64, u64, u64); 16] = [
    (0x1_43cf_376c, 0x1_43cf_5300, 0x1_433f_4848, 0x12, 0x1_4262_d660),
    (0x1_43cf_4014, 0x1_43cf_4f68, 0x1_433f_4830, 0x12, 0x1_4262_d650),
    (0x1_43cf_2dbc, 0x1_43cf_3750, 0x1_433f_4818, 0x12, 0x1_4262_d640),
    (0x1_43cf_5294, 0x1_43cf_3480, 0x1_433f_4800, 0x12, 0x1_4262_d630),
    (0x1_43cf_39fc, 0x1_43cf_2d88, 0x1_433f_47e8, 0x12, 0x1_4262_d620),
    (0x1_43cf_4ad4, 0x1_43cf_3f60, 0x1_433f_47d0, 0x12, 0x1_4262_d610),
    (0x1_43cf_2c48, 0x1_43cf_4160, 0x1_433f_47b8, 0x12, 0x1_4262_d600),
    (0x1_43cf_5198, 0x1_43cf_4620, 0x1_433f_47a0, 0x12, 0x1_4262_d5f0),
    (0x1_43cf_3800, 0x1_43cf_5140, 0x1_433f_4788, 0x10, 0x1_4262_d5e0),
    (0x1_43cf_4ad0, 0x1_43cf_4eb8, 0x1_433f_4588, 0x10, 0x1_4262_d5d0),
    (0x1_43cf_4374, 0x1_43cf_48c0, 0x1_433f_4570, 0x10, 0x1_4262_d5c0),
    (0x1_43cf_4f58, 0x1_43cf_33e0, 0x1_433f_4558, 0x10, 0x1_4262_d5b0),
    (0x1_43cf_2b6c, 0x1_43cf_42f0, 0x1_433f_4540, 0x10, 0x1_4262_d5a0),
    (0x1_43cf_37d8, 0x1_43cf_49f8, 0x1_433f_4528, 0x10, 0x1_4262_d590),
    (0x1_43cf_3fc4, 0x1_43cf_3d88, 0x1_433f_4510, 0x10, 0x1_4262_d580),
    (0x1_43cf_3768, 0x1_43cf_3a68, 0x1_433f_44f8, 0x10, 0x1_4262_d570),
];

const SAMPLERS_B1: [(u64, u64, u64, u64, u64, u64, usize); 7] = [
    (0x1_43cf_51c0, 0x1_43cf_51d0, 0x1_433f_44e8, 0x0e, 0x1_4262_d560, 0x1_43a3_4f48, 0x2b8),
    (0x1_43cf_4be0, 0x1_43cf_2a38, 0x1_433f_44d8, 0x0f, 0x1_4262_d550, 0x1_43a3_48d8, 0x2c0),
    (0x1_43cf_4668, 0x1_43cf_4bd0, 0x1_433f_44b8, 0x18, 0x1_4262_d540, 0x1_43a3_49b8, 0x2c8),
    (0x1_43cf_3bd4, 0x1_43cf_4e50, 0x1_433f_4498, 0x19, 0x1_4262_d530, 0x1_43a3_4e78, 0x2d0),
    (0x1_43cf_3718, 0x1_43cf_2d70, 0x1_433f_4480, 0x15, 0x1_4262_d520, 0x1_43a3_47b8, 0x2d8),
    (0x1_43cf_48e0, 0x1_43cf_4060, 0x1_433f_4470, 0x0d, 0x1_4262_d510, 0x1_43a3_53b8, 0x2e0),
    (0x1_43cf_5108, 0x1_43cf_32c8, 0x1_433f_4460, 0x0e, 0x1_4262_d500, 0x1_43a3_5148, 0x2e8),
];

const SAMPLERS_B2: [(u64, u64, u64, u64, u64, usize); 9] = [
    (0x1_43cf_47e0, 0x1_43cf_50e0, 0x1_433f_4450, 0x1_4262_d4f0, 0x1_43a3_4df8, 0x2f0),
    (0x1_43cf_3d1c, 0x1_43cf_4d30, 0x1_433f_4438, 0x1_4262_d4e0, 0x1_43a3_4a38, 0x2f8),
    (0x1_43cf_3fc0, 0x1_43cf_4e68, 0x1_433f_4418, 0x1_4262_d4d0, 0x1_43a3_5328, 0x300),
    (0x1_43cf_52b0, 0x1_43cf_3c58, 0x1_433f_4400, 0x1_4262_d4c0, 0x1_43a3_4b68, 0x308),
    (0x1_43cf_3638, 0x1_43cf_5048, 0x1_433f_43e8, 0x1_4262_d4b0, 0x1_43a3_5898, 0x310),
    (0x1_43cf_46d8, 0x1_43cf_37a0, 0x1_433f_43d0, 0x1_4262_d4a0, 0x1_43a3_55f8, 0x318),
    (0x1_43cf_41e0, 0x1_43cf_4d50, 0x1_433f_43b8, 0x1_4262_d490, 0x1_43a3_51f8, 0x320),
    (0x1_43cf_3678, 0x1_43cf_3980, 0x1_433f_43a0, 0x1_4262_d480, 0x1_43a3_4848, 0x328),
    (0x1_43cf_3cbc, 0x1_43cf_3fa0, 0x1_433f_4388, 0x1_4262_d470, 0x1_43a3_5448, 0x330),
];

const SAMPLERS_B3: [(u64, u64, u64, u64, u64, usize); 3] = [
    (0x1_43cf_396c, 0x1_43cf_44b0, 0x1_433f_4370, 0x1_4262_d460, 0x1_43a3_4d08, 0x338),
    (0x1_43cf_367c, 0x1_43cf_4b98, 0x1_433f_4770, 0x1_4262_d450, 0x1_43a3_4728, 0x340),
    (0x1_43cf_5244, 0x1_43cf_3460, 0x1_433f_4758, 0x1_4262_d440, 0x1_43a3_4fc8, 0x348),
];

#[inline(always)]
unsafe fn tls_prop_copy(tls0: usize, tbl_base: usize, id: i32, src_ptr: usize) {
    let entry = tbl_base
        .wrapping_add(0x3a6_37b0)
        .wrapping_add((id as isize as usize).wrapping_mul(16));
    let sz = rd::<i32>(entry + 0x08) as i64 as u64;
    let off = rd::<i32>(entry);
    if off >= 0 {
        let dst = tls0.wrapping_add(0x63d0).wrapping_add(off as usize);
        exe::call_va!(0x1_4183_acbc, (dst, src_ptr, sz));
        let cnt_idx = rd::<i32>(entry + 0x0c);
        if cnt_idx != -1 {
            let cnt_addr = tls0
                .wrapping_add(0x463e0)
                .wrapping_add((cnt_idx as isize as usize).wrapping_mul(4));
            let next_cnt = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next_cnt == -1 { 0 } else { next_cnt });
        }
    }
}

#[inline(always)]
unsafe fn init_cvar_inline(
    tls0: usize,
    guard_va: u64,
    id_va: u64,
    name_va: u64,
    name_len: u64,
    cvar_type: u32,
    init_mode: u8,
    atexit_va: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let mut sv: [u64; 2] = [exe::ptr::<u8>(name_va) as usize as u64, name_len];
            let id_ptr = exe::ptr::<i32>(id_va) as usize;
            exe::call_va!(0x1_4220_ea70, (id_ptr, sv.as_mut_ptr() as usize));
            wr::<u32>(id_ptr + 0x04, cvar_type);
            wr::<usize>(id_ptr.wrapping_sub(0x08), id_ptr + 0x08);
            if init_mode == 1 {
                wr::<u32>(id_ptr + 0x10, 0);
            } else if init_mode == 2 {
                let def_ptr = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4b68) as usize);
                let def_val = if def_ptr != 0 { rd::<u64>(def_ptr) } else { 0 };
                wr::<u64>(id_ptr + 0x08, def_val);
            }
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn init_sampler_d8e0(
    tls0: usize,
    guard_va: u64,
    id_va: u64,
    name_va: u64,
    atexit_va: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let base_ptr = (exe::ptr::<i32>(id_va) as usize).wrapping_sub(0x08);
            exe::call_va!(0x1_412e_d8e0, (base_ptr, exe::ptr::<u8>(name_va) as usize));
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn update_cvar_u32(
    tls0: usize,
    tbl_base: usize,
    guard_va: u64,
    id_va: u64,
    name_va: u64,
    name_len: u64,
    cvar_type: u32,
    atexit_va: u64,
    val: u32,
) {
    init_cvar_inline(
        tls0, guard_va, id_va, name_va, name_len, cvar_type, 0, atexit_va,
    );
    let mut tmp: u64 = val as u64;
    let id_ptr = exe::ptr::<i32>(id_va) as usize;
    let id = rd::<i32>(id_ptr);
    tls_prop_copy(tls0, tbl_base, id, (&mut tmp as *mut u64) as usize);
    wr::<u32>(id_ptr + 0x08, val);
}

/// `unknown_142299ad0`
/// Original: 0x142299ad0 (12158 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142299ad0(param_1: usize) {
    let g_fc58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let (r8b, r12d): (u8, u32) = if rd::<u8>(g_fc58 + 0xac) != 0 {
        (1, rd::<u32>(exe::ptr::<u32>(0x1_43bb_d480) as usize))
    } else {
        (0, 0)
    };
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8d6) as usize, r8b);
    let b_98b8 = rd::<u8>(exe::ptr::<u8>(0x1_4386_98b8) as usize);
    wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc6) as usize, b_98b8);
    wr::<u8>(exe::ptr::<u8>(0x1_4386_98b8) as usize, 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4386_98b4) as usize, 1);

    let dil = u8::from(r12d == 1);
    let ebx: u32;
    if rd::<u8>(exe::ptr::<u8>(0x1_43a2_8d34) as usize) != 0 && r8b != 0 {
        wr::<u8>(exe::ptr::<u8>(0x1_4385_82c0) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_82bc) as usize, 1);
        let cc5 = rd::<u8>(exe::ptr::<u8>(0x1_43a2_8cc5) as usize);
        wr::<u32>(
            exe::ptr::<u32>(0x1_43a6_3760) as usize,
            if cc5 != 0 { 2 } else { 0 },
        );
        wr::<u8>(exe::ptr::<u8>(0x1_43a6_374c) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a440) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a43c) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_ac68) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_ac64) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8c3f) as usize, 0);
        ebx = 0;
        wr::<u32>(exe::ptr::<u32>(0x1_43bb_d128) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_43bb_d114) as usize, 1);
        wr::<u32>(
            exe::ptr::<u32>(0x1_4397_ff50) as usize,
            rd::<u32>(exe::ptr::<u32>(0x1_43bb_d4a8) as usize),
        );
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8c2) as usize, dil);
        wr::<u8>(
            exe::ptr::<u8>(0x1_4397_d8b7) as usize,
            u8::from(r12d == 2),
        );
    } else {
        wr::<u8>(exe::ptr::<u8>(0x1_4385_82c0) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_82bc) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_43a6_3760) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_43a6_374c) as usize, 1);
        wr::<u32>(
            exe::ptr::<u32>(0x1_4397_ff50) as usize,
            rd::<u32>(exe::ptr::<u32>(0x1_43bb_d4a8) as usize),
        );
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8c2) as usize, dil);
        wr::<u8>(
            exe::ptr::<u8>(0x1_4397_d8b7) as usize,
            u8::from(r12d == 2),
        );
        ebx = rd::<u32>(exe::ptr::<u32>(0x1_43bb_d128) as usize);
    }

    let cl = u8::from(
        r8b != 0 && r12d == 0 && rd::<u8>(exe::ptr::<u8>(0x1_4386_a040) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8d3) as usize, cl);

    let dl = u8::from(
        r8b != 0 && r12d != 2 && rd::<u8>(exe::ptr::<u8>(0x1_4386_9948) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8c7) as usize, dl);

    let r9b = u8::from(
        r8b != 0 && r12d == 0 && rd::<u8>(exe::ptr::<u8>(0x1_4386_a3a0) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8be) as usize, r9b);

    let r10b = u8::from(
        r8b != 0 && r12d != 2 && rd::<u8>(exe::ptr::<u8>(0x1_4386_a4c8) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8c3) as usize, r10b);

    let r11b = u8::from(
        r8b != 0
            && r12d != 2
            && r10b == 0
            && rd::<u8>(exe::ptr::<u8>(0x1_4386_9b10) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8ce) as usize, r11b);

    let r14b_init = u8::from(
        r8b != 0
            && r12d == 0
            && r11b == 0
            && r10b == 0
            && rd::<u8>(exe::ptr::<u8>(0x1_4386_a088) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8d2) as usize, r14b_init);

    let r15b = u8::from(
        r8b != 0 && r12d == 0 && rd::<u8>(exe::ptr::<u8>(0x1_4386_9ad8) as usize) != 0,
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8b3) as usize, r15b);

    let sil = u8::from(r8b != 0 && rd::<u8>(exe::ptr::<u8>(0x1_4386_b898) as usize) != 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8bf) as usize, sil);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8ba) as usize, 0);

    let bb = u8::from(rd::<u8>(exe::ptr::<u8>(0x1_4386_a440) as usize) != 0 && r11b != 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8bb) as usize, bb);
    wr::<u8>(
        exe::ptr::<u8>(0x1_4397_d8b6) as usize,
        rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize),
    );

    let any_active = r8b != 0
        && (r9b != 0
            || cl != 0
            || r14b_init != 0
            || r15b != 0
            || dl != 0
            || r10b != 0
            || r11b != 0
            || r12d != 0
            || sil != 0);

    let old_cf = rd::<u8>(exe::ptr::<u8>(0x1_4397_d8cf) as usize);
    let mut r14b: u8;
    let new_cf: u8;
    if any_active {
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_4385_5868) as usize, ebx);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8cb) as usize, old_cf);
        r14b = 1;
        if rd::<u8>(exe::ptr::<u8>(0x1_4385_83cf) as usize) == 0
            && ((r11b != 0 && rd::<u8>(exe::ptr::<u8>(0x1_43a2_8c3f) as usize) != 0)
                || (dil != 0 && ebx != 0))
        {
            new_cf = 1;
        } else {
            new_cf = 0;
        }
    } else {
        r14b = 0;
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4385_5868) as usize, ebx);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8cb) as usize, old_cf);
        new_cf = 0;
    }
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8cf) as usize, new_cf);
    wr::<u8>(
        exe::ptr::<u8>(0x1_4397_d8c6) as usize,
        rd::<u8>(exe::ptr::<u8>(0x1_4386_9f60) as usize),
    );
    wr::<u32>(
        exe::ptr::<u32>(0x1_4397_ff34) as usize,
        rd::<u32>(exe::ptr::<u32>(0x1_4386_a470) as usize),
    );
    wr::<u32>(
        exe::ptr::<u32>(0x1_4397_ff54) as usize,
        rd::<u32>(exe::ptr::<u32>(0x1_4386_98f8) as usize),
    );
    let b_d863 = rd::<u8>(exe::ptr::<u8>(0x1_4397_d863) as usize);
    if rd::<u8>(exe::ptr::<u8>(0x1_43c8_c810) as usize) != b_d863 {
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc7) as usize, 1);
    }
    wr::<u8>(exe::ptr::<u8>(0x1_43c8_c810) as usize, b_d863);
    r14b |= rd::<u8>(exe::ptr::<u8>(0x1_4386_9f78) as usize);

    if new_cf != 0 {
        let w_a68 = rd::<u16>(exe::ptr::<u16>(0x1_43a3_3a68) as usize) as u32;
        let w_a6c = rd::<u16>(exe::ptr::<u16>(0x1_43a3_3a6c) as usize) as u32;
        exe::call_va!(
            0x1_4205_8260,
            (w_a68, w_a6c, r8b, 2u32, u8::from(ebx == 3))
        );
    } else if old_cf != 0 {
        exe::call_va!(0x1_4205_8180, ());
    }

    exe::call_va!(0x1_4226_8cc0, (param_1));

    let p48 = rd::<usize>(param_1 + 0x48);
    let fmt_byte: u8 = if rd::<u8>(p48 + 0x4c) == 0 {
        0x16
    } else if rd::<u8>(p48 + 0x66) != 0 {
        0x18
    } else {
        let mode = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
        if mode == 0 {
            if rd::<u8>(p48 + 0x67) == 0 {
                0x18
            } else {
                0x16
            }
        } else if mode == 2 {
            0x18
        } else {
            0x16
        }
    };
    wr::<u8>(param_1 + 0x488, fmt_byte);

    let xmm7_bits = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
    wr::<u32>(param_1 + 0x4a0, xmm7_bits);
    wr::<u32>(param_1 + 0x4a4, xmm7_bits);
    wr::<u32>(
        exe::ptr::<u32>(0x1_43a2_b1f8) as usize,
        rd::<u32>(exe::ptr::<u32>(0x1_4386_9a60) as usize),
    );
    wr::<u8>(exe::ptr::<u8>(0x1_4384_3fce) as usize, 1);
    wr::<u64>(
        exe::ptr::<u64>(0x1_4397_fda0) as usize,
        rd::<u64>(exe::ptr::<u64>(0x1_4397_fda8) as usize),
    );
    wr::<u64>(
        exe::ptr::<u64>(0x1_4397_fd90) as usize,
        rd::<u64>(exe::ptr::<u64>(0x1_4397_fe40) as usize),
    );
    wr::<u8>(param_1 + 0x489, 0x18);

    if rd::<usize>(param_1 + 0x50) != 0 {
        let obj = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc48) as usize);
        let vtbl = rd::<usize>(obj);
        let arg2 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5918) as usize);
        exe::call_va!(rd::<usize>(vtbl + 0x08), (obj, arg2));
    }

    let lock_ptr = exe::ptr::<u8>(0x1_4385_7a20) as usize;
    let owner_ptr = exe::ptr::<u32>(0x1_4385_7a28) as usize;
    let depth_ptr = exe::ptr::<u32>(0x1_4385_7a2c) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_ptr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_ptr));
        wr::<u32>(owner_ptr, tid);
    }
    wr::<u32>(depth_ptr, rd::<u32>(depth_ptr).wrapping_add(1));
    let arg_5918 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5918) as usize);
    let mut cur_cb = rd::<usize>(exe::ptr::<usize>(0x1_43a6_31f8) as usize);
    let cb_cnt = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3200) as usize) as usize;
    let end_cb = cur_cb.wrapping_add(cb_cnt.wrapping_mul(8));
    while cur_cb != end_cb {
        let obj = rd::<usize>(cur_cb);
        let vtbl = rd::<usize>(obj);
        exe::call_va!(rd::<usize>(vtbl + 0x08), (obj, arg_5918));
        cur_cb = cur_cb.wrapping_add(8);
    }
    let next_depth = rd::<u32>(depth_ptr).wrapping_sub(1);
    wr::<u32>(depth_ptr, next_depth);
    if next_depth == 0 {
        crate::simd_ops::atomic_xchg_u32(owner_ptr as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_ptr));
    }

    let tls0 = rd::<usize>(exe::tls_pointer());
    let tbl_base = exe::ptr::<u8>(0x1_4000_0000) as usize;

    let g_47e4 = exe::ptr::<i32>(0x1_43cf_47e4) as usize;
    if rd::<i32>(g_47e4) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (g_47e4));
        if rd::<i32>(g_47e4) == -1 {
            exe::call_va!(0x1_401d_f790, (exe::ptr::<u8>(0x1_43cf_3720) as usize));
            exe::call_va!(0x1_4183_4a0c, (g_47e4));
        }
    }

    wr::<f64>(
        exe::ptr::<f64>(0x1_43cf_3730) as usize,
        rd::<f64>(exe::ptr::<f64>(0x1_43cf_3728) as usize),
    );
    let cur_time: f64 = if rd::<u8>(exe::ptr::<u8>(0x1_43cf_3740) as usize) != 0 {
        rd::<f64>(exe::ptr::<f64>(0x1_43cf_3738) as usize)
    } else {
        let mut qpc: i64 = 0;
        exe::call_iat!(0x1_4264_56f8, ((&mut qpc as *mut i64) as usize));
        let diff = qpc.wrapping_sub(rd::<i64>(exe::ptr::<i64>(0x1_43cf_3720) as usize));
        let freq = exe::call_va!(0x1_4240_6920, () -> i64);
        (diff as f64) / (freq as f64)
    };
    let prev_time = rd::<f64>(exe::ptr::<f64>(0x1_43cf_3730) as usize);
    wr::<f64>(exe::ptr::<f64>(0x1_43cf_3728) as usize, cur_time);
    let dt = cur_time - prev_time;
    let accum = dt + rd::<f64>(exe::ptr::<f64>(0x1_43a0_0168) as usize);
    wr::<f64>(exe::ptr::<f64>(0x1_43a0_0168) as usize, accum);
    wr::<f32>(exe::ptr::<f32>(0x1_43a0_017c) as usize, dt as f32);

    let mut xmm3 = rd::<f32>(exe::ptr::<f32>(0x1_4342_714c) as usize);
    let abs_xmm3 = f32::from_bits(
        xmm3.to_bits() & rd::<u32>(exe::ptr::<u32>(0x1_4342_8720) as usize),
    );
    let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_6de8) as usize);
    if !(thresh >= abs_xmm3) {
        if rd::<u32>(exe::ptr::<u32>(0x1_4397_ff30) as usize) == 4
            && rd::<u8>(exe::ptr::<u8>(0x1_4386_9fd0) as usize) != 0
        {
            xmm3 = f32::from_bits(xmm7_bits);
        } else {
            let num = rd::<i32>(exe::ptr::<i32>(0x1_43a3_3a68) as usize) as f32;
            let den = rd::<i32>(exe::ptr::<i32>(0x1_43a3_3b98) as usize) as f32;
            let ratio = num / den;
            let (_, log2_xmm) = crate::simd_ops::call_va8_xmm(
                0x1_4254_76d0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [ratio.to_bits() as u128, 0, 0, 0],
            );
            xmm3 = f32::from_bits(log2_xmm as u32) - f32::from_bits(xmm7_bits);
        }
    }
    let idx_361 = rd::<u8>(exe::ptr::<u8>(0x1_4386_a361) as usize) as usize;
    let bias = rd::<f32>((exe::ptr::<f32>(0x1_4386_a364) as usize).wrapping_add(idx_361 * 4));
    if xmm3 + bias != rd::<f32>(exe::ptr::<f32>(0x1_4397_db78) as usize) {
        exe::call_va!(0x1_4205_33b0, ());
    }

    for &(guard_va, id_va, name_va, name_len, atexit_va) in &SLIDERS_AND_KNOBS {
        init_cvar_inline(
            tls0, guard_va, id_va, name_va, name_len, 0, 1, atexit_va,
        );
        let id_ptr = exe::ptr::<i32>(id_va) as usize;
        let id = rd::<i32>(id_ptr) as i64;
        let sz = rd::<i32>(
            tbl_base
                .wrapping_add(0x3a6_37b8)
                .wrapping_add((id as isize as usize).wrapping_mul(16)),
        ) as i64;
        exe::call_va!(0x1_4209_a900, (id, 0u32, sz, id_ptr + 0x10));
        wr::<u32>(id_ptr + 0x08, rd::<u32>(id_ptr + 0x10));
    }

    let p230 = rd::<usize>(param_1 + 0x230);
    if p230 != 0 {
        exe::call_va!(0x1_423c_7200, (p230));
    }
    let p1d0 = rd::<usize>(param_1 + 0x1d0);
    if p1d0 != 0 {
        exe::call_va!(0x1_4224_7710, (p1d0));
    }
    let p1a8 = rd::<usize>(param_1 + 0x1a8);
    if p1a8 != 0 {
        exe::call_va!(0x1_4225_c9a0, (p1a8));
    }

    for &(guard_va, id_va, name_va, name_len, atexit_va, src_va, ctx_off) in &SAMPLERS_B1 {
        init_cvar_inline(
            tls0, guard_va, id_va, name_va, name_len, 8, 2, atexit_va,
        );
        let src_ptr = rd::<usize>(exe::ptr::<usize>(src_va) as usize);
        let mut val: u64 = if src_ptr != 0 { rd::<u64>(src_ptr) } else { 0 };
        let id_ptr = exe::ptr::<i32>(id_va) as usize;
        let id = rd::<i32>(id_ptr);
        tls_prop_copy(tls0, tbl_base, id, (&mut val as *mut u64) as usize);
        wr::<u64>(id_ptr + 0x08, val);
        let handle = rd::<u64>(val as usize);
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
        wr::<u64>(ctx + 0x4b8, rd::<u64>(ctx + 0x4b8).wrapping_add(1));
        wr::<u64>(ctx + ctx_off, handle);
    }

    for &(guard_va, id_va, name_va, atexit_va, src_va, ctx_off) in &SAMPLERS_B2 {
        init_sampler_d8e0(tls0, guard_va, id_va, name_va, atexit_va);
        let src_ptr = rd::<usize>(exe::ptr::<usize>(src_va) as usize);
        let mut val: u64 = if src_ptr != 0 { rd::<u64>(src_ptr) } else { 0 };
        let id_ptr = exe::ptr::<i32>(id_va) as usize;
        let id = rd::<i32>(id_ptr) as i64;
        let sz = rd::<i32>(
            tbl_base
                .wrapping_add(0x3a6_37b8)
                .wrapping_add((id as isize as usize).wrapping_mul(16)),
        ) as i64;
        exe::call_va!(0x1_4209_a900, (id, 0u32, sz, (&mut val as *mut u64) as usize));
        wr::<u64>(id_ptr + 0x08, val);
        let handle = rd::<u64>(val as usize);
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
        wr::<u64>(ctx + 0x4b8, rd::<u64>(ctx + 0x4b8).wrapping_add(1));
        wr::<u64>(ctx + ctx_off, handle);
    }

    for &(guard_va, id_va, name_va, atexit_va, src_va, ctx_off) in &SAMPLERS_B3 {
        init_sampler_d8e0(tls0, guard_va, id_va, name_va, atexit_va);
        let src_ptr = rd::<usize>(exe::ptr::<usize>(src_va) as usize);
        let mut val: u64 = if src_ptr != 0 { rd::<u64>(src_ptr) } else { 0 };
        let id_ptr = exe::ptr::<i32>(id_va) as usize;
        let id = rd::<i32>(id_ptr);
        tls_prop_copy(tls0, tbl_base, id, (&mut val as *mut u64) as usize);
        wr::<u64>(id_ptr + 0x08, val);
        let handle = rd::<u64>(val as usize);
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
        wr::<u64>(ctx + 0x4b8, rd::<u64>(ctx + 0x4b8).wrapping_add(1));
        wr::<u64>(ctx + ctx_off, handle);
    }

    let mat_samplers: [usize; 3] = [
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_4d58) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_57b8) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_5498) as usize),
    ];
    init_sampler_d8e0(
        tls0,
        0x1_43cf_4f80,
        0x1_43cf_5280,
        0x1_433f_4740,
        0x1_4262_d430,
    );
    let mat_idx = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0144) as usize) as usize;
    let mat_src = mat_samplers[mat_idx];
    let mut mat_val: u64 = if mat_src != 0 { rd::<u64>(mat_src) } else { 0 };
    let mat_id_ptr = exe::ptr::<i32>(0x1_43cf_5280) as usize;
    tls_prop_copy(
        tls0,
        tbl_base,
        rd::<i32>(mat_id_ptr),
        (&mut mat_val as *mut u64) as usize,
    );
    wr::<u64>(mat_id_ptr + 0x08, mat_val);
    let mat_handle = rd::<u64>(mat_val as usize);
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let base_cnt = rd::<u64>(ctx + 0x4b8);
    wr::<u64>(ctx + 0x4b8, base_cnt.wrapping_add(1));
    wr::<u64>(ctx + 0x350, mat_handle);

    const EXTRA_SAMPLERS: [(u64, usize, u64); 6] = [
        (0x1_43a3_4b68, 0x358, 2),
        (0x1_43a3_4a38, 0x360, 3),
        (0x1_43a3_4b68, 0x368, 4),
        (0x1_43a3_51f8, 0x370, 5),
        (0x1_43a3_5448, 0x378, 6),
        (0x1_43a3_4728, 0x380, 7),
    ];
    for &(src_va, off, step) in &EXTRA_SAMPLERS {
        let p1 = rd::<usize>(exe::ptr::<usize>(src_va) as usize);
        let p2 = rd::<usize>(p1);
        let h = rd::<u64>(p2);
        wr::<u64>(ctx + 0x4b8, base_cnt.wrapping_add(step));
        wr::<u64>(ctx + off, h);
    }

    let hdr_val = u32::from(rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c) != 0);
    update_cvar_u32(
        tls0,
        tbl_base,
        0x1_43cf_3184,
        0x1_43cf_3ca8,
        0x1_433f_4738,
        0x06,
        0x0c,
        0x1_4262_d420,
        hdr_val,
    );

    let scrgb_val = u32::from(rd::<u8>(param_1 + 0x49d) != 0);
    update_cvar_u32(
        tls0,
        tbl_base,
        0x1_43cf_3a5c,
        0x1_43cf_4e80,
        0x1_433f_4728,
        0x0c,
        0x0c,
        0x1_4262_d410,
        scrgb_val,
    );

    let sdr_mult = if rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c) == 0 {
        xmm7_bits
    } else {
        rd::<u32>(exe::ptr::<u32>(0x1_4384_1634) as usize)
    };
    update_cvar_u32(
        tls0,
        tbl_base,
        0x1_43cf_2b1c,
        0x1_43cf_3b38,
        0x1_433f_4708,
        0x1a,
        0,
        0x1_4262_d400,
        sdr_mult,
    );

    let max_nits = rd::<u32>(rd::<usize>(param_1 + 0x48) + 0x50);
    update_cvar_u32(
        tls0,
        tbl_base,
        0x1_43cf_4c34,
        0x1_43cf_3c90,
        0x1_433f_46f0,
        0x10,
        0,
        0x1_4262_d3f0,
        max_nits,
    );

    let sdr_sec = if rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c) == 0 {
        rd::<u32>(param_1 + 0x68c)
    } else {
        xmm7_bits
    };
    update_cvar_u32(
        tls0,
        tbl_base,
        0x1_43cf_3f8c,
        0x1_43cf_3810,
        0x1_433f_46d0,
        0x19,
        0,
        0x1_4262_d3e0,
        sdr_sec,
    );

    if rd::<u64>(exe::ptr::<u64>(0x1_43a0_0188) as usize) != 0 {
        let rdx_ptr = if r14b != 0 {
            exe::ptr::<u8>(0x1_4386_a050) as usize
        } else {
            exe::ptr::<u8>(0x1_4386_a048) as usize
        };
        exe::call_va!(0x1_4220_c7e0, (0usize, rdx_ptr));
    }

    let g_5394 = exe::ptr::<i32>(0x1_43cf_5394) as usize;
    if rd::<i32>(g_5394) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (g_5394));
        if rd::<i32>(g_5394) == -1 {
            wr::<u32>(
                exe::ptr::<u32>(0x1_43cf_3180) as usize,
                rd::<u32>(exe::ptr::<u32>(0x1_43bb_ce30) as usize),
            );
            exe::call_va!(0x1_4183_4a0c, (g_5394));
        }
    }

    let g_5130 = exe::ptr::<i32>(0x1_43cf_5130) as usize;
    if rd::<i32>(g_5130) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (g_5130));
        if rd::<i32>(g_5130) == -1 {
            wr::<u8>(
                exe::ptr::<u8>(0x1_43cf_4998) as usize,
                rd::<u8>(exe::ptr::<u8>(0x1_4386_8068) as usize),
            );
            exe::call_va!(0x1_4183_4a0c, (g_5130));
        }
    }

    let cl_f0 = rd::<u8>(exe::ptr::<u8>(0x1_43c8_c7f0) as usize);
    let dl_d3 = rd::<u8>(exe::ptr::<u8>(0x1_4397_d8d3) as usize);
    if dl_d3 != 0 || r12d != 0 {
        if cl_f0 == 0 {
            wr::<u32>(
                exe::ptr::<u32>(0x1_43cf_3180) as usize,
                rd::<u32>(exe::ptr::<u32>(0x1_43bb_ce30) as usize),
            );
            wr::<u8>(
                exe::ptr::<u8>(0x1_43cf_4998) as usize,
                rd::<u8>(exe::ptr::<u8>(0x1_4386_8068) as usize),
            );
            wr::<u32>(exe::ptr::<u32>(0x1_43bb_ce30) as usize, 0);
            wr::<u8>(exe::ptr::<u8>(0x1_43bb_ce1c) as usize, 1);
            wr::<u8>(exe::ptr::<u8>(0x1_4386_8068) as usize, 0);
            wr::<u8>(exe::ptr::<u8>(0x1_4386_8064) as usize, 1);
        }
        wr::<u8>(exe::ptr::<u8>(0x1_43c8_c7f0) as usize, 1);
    } else {
        if cl_f0 != 0 {
            wr::<u32>(
                exe::ptr::<u32>(0x1_43bb_ce30) as usize,
                rd::<u32>(exe::ptr::<u32>(0x1_43cf_3180) as usize),
            );
            wr::<u8>(exe::ptr::<u8>(0x1_43bb_ce1c) as usize, 1);
            wr::<u8>(
                exe::ptr::<u8>(0x1_4386_8068) as usize,
                rd::<u8>(exe::ptr::<u8>(0x1_43cf_4998) as usize),
            );
            wr::<u8>(exe::ptr::<u8>(0x1_4386_8064) as usize, 1);
        }
        wr::<u8>(exe::ptr::<u8>(0x1_43c8_c7f0) as usize, 0);
    }

    let p200 = rd::<usize>(param_1 + 0x200);
    if p200 != 0 {
        exe::call_va!(0x1_4225_5340, (p200));
    }
    exe::call_va!(0x1_4230_b890, (rd::<usize>(param_1 + 0x1e8)));
    let p150 = rd::<usize>(param_1 + 0x150);
    if p150 != 0 {
        wr::<u32>(p150 + 0x08, 0);
    }
    exe::call_va!(0x1_4206_5820, ());
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_unknown_142299ad0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    unsafe fn mix(v: u64) {
        TRACE_HASH = TRACE_HASH
            .wrapping_mul(0x0100_0000_01b3)
            .wrapping_add(v ^ 0x9e37_79b9_7f4a_7c15);
    }

    unsafe extern "win64" fn mock_8260(a1: u32, a2: u32, a3: u8, a4: u32, a5: u8) {
        mix(0x8260 ^ (a1 as u64) ^ ((a2 as u64) << 16) ^ ((a3 as u64) << 32) ^ ((a4 as u64) << 40) ^ ((a5 as u64) << 48));
    }
    unsafe extern "win64" fn mock_8180() {
        mix(0x8180);
    }
    unsafe extern "win64" fn mock_8cc0(p1: usize) {
        mix(0x8cc0 ^ (rd::<u8>(p1 + 0x49d) as u64));
    }
    unsafe extern "win64" fn mock_vtbl_08(_this: usize, arg2: usize) {
        mix(0x0808 ^ (arg2 as u64));
    }
    unsafe extern "win64" fn mock_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_srw_acq(_lock: usize) {
        mix(0x55a8);
    }
    unsafe extern "win64" fn mock_srw_rel(_lock: usize) {
        mix(0x5598);
    }
    unsafe extern "win64" fn mock_qpc(out: usize) -> i32 {
        wr::<i64>(out, 5_000_000);
        1
    }
    unsafe extern "win64" fn mock_6920() -> i64 {
        1_000_000
    }
    unsafe extern "win64" fn mock_33b0() {
        mix(0x33b0);
    }
    unsafe extern "win64" fn mock_a900(id: i64, zero: u32, sz: i64, ptr: usize) {
        if id < 16 {
            wr::<f32>(ptr, (id as f32) * 0.5 + 1.0);
        }
        mix(0xa900 ^ (id as u64) ^ ((zero as u64) << 16) ^ ((sz as u64) << 32));
    }
    unsafe extern "win64" fn mock_7200(p: usize) {
        mix(0x7200 ^ (p as u64));
    }
    unsafe extern "win64" fn mock_7710(p: usize) {
        mix(0x7710 ^ (p as u64));
    }
    unsafe extern "win64" fn mock_c9a0(p: usize) {
        mix(0xc9a0 ^ (p as u64));
    }
    unsafe extern "win64" fn mock_c7e0(_rcx: usize, rdx: usize) {
        let off = rdx.wrapping_sub(exe::ptr::<u8>(0x1_4386_a048) as usize);
        mix(0xc7e0 ^ (off as u64));
    }
    unsafe extern "win64" fn mock_5340(p: usize) {
        mix(0x5340 ^ (p as u64));
    }
    unsafe extern "win64" fn mock_b890(p: usize) {
        mix(0xb890 ^ (p as u64));
    }
    unsafe extern "win64" fn mock_5820() {
        mix(0x5820);
    }

    let _m01 = crate::difftest::Mock::new(0x1_4205_8260, mock_8260 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4205_8180, mock_8180 as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_4226_8cc0, mock_8cc0 as *const ())?;
    let _m04 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _m05 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_srw_acq as *const ())?;
    let _m06 = crate::difftest::IatMock::new(0x1_4264_5598, mock_srw_rel as *const ())?;
    let _m07 = crate::difftest::IatMock::new(0x1_4264_56f8, mock_qpc as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4240_6920, mock_6920 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4205_33b0, mock_33b0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_423c_7200, mock_7200 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4224_7710, mock_7710 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4225_c9a0, mock_c9a0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4220_c7e0, mock_c7e0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4225_5340, mock_5340 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4230_b890, mock_b890 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4206_5820, mock_5820 as *const ())?;

    let mut all_guards = [0u64; 44];
    all_guards[0] = 0x1_43cf_47e4;
    for i in 0..16 {
        all_guards[1 + i] = SLIDERS_AND_KNOBS[i].0;
    }
    for i in 0..7 {
        all_guards[17 + i] = SAMPLERS_B1[i].0;
    }
    for i in 0..9 {
        all_guards[24 + i] = SAMPLERS_B2[i].0;
    }
    for i in 0..3 {
        all_guards[33 + i] = SAMPLERS_B3[i].0;
    }
    all_guards[36] = 0x1_43cf_4f80;
    all_guards[37] = 0x1_43cf_3184;
    all_guards[38] = 0x1_43cf_3a5c;
    all_guards[39] = 0x1_43cf_2b1c;
    all_guards[40] = 0x1_43cf_4c34;
    all_guards[41] = 0x1_43cf_3f8c;
    all_guards[42] = 0x1_43cf_5394;
    all_guards[43] = 0x1_43cf_5130;

    let mut saved_guards = [0i32; 44];
    for (i, &va) in all_guards.iter().enumerate() {
        let p = exe::ptr::<i32>(va) as usize;
        saved_guards[i] = rd::<i32>(p);
        wr::<i32>(p, i32::MIN);
    }

    let mut all_ids = [0u64; 41];
    for i in 0..16 {
        all_ids[i] = SLIDERS_AND_KNOBS[i].1;
    }
    for i in 0..7 {
        all_ids[16 + i] = SAMPLERS_B1[i].1;
    }
    for i in 0..9 {
        all_ids[23 + i] = SAMPLERS_B2[i].1;
    }
    for i in 0..3 {
        all_ids[32 + i] = SAMPLERS_B3[i].1;
    }
    all_ids[35] = 0x1_43cf_5280;
    all_ids[36] = 0x1_43cf_3ca8;
    all_ids[37] = 0x1_43cf_4e80;
    all_ids[38] = 0x1_43cf_3b38;
    all_ids[39] = 0x1_43cf_3c90;
    all_ids[40] = 0x1_43cf_3810;

    let mut saved_id_blocks = [[0u8; 24]; 41];
    for (i, &va) in all_ids.iter().enumerate() {
        let p = exe::ptr::<u8>(va) as usize;
        saved_id_blocks[i] = rd::<[u8; 24]>(p);
        wr::<i32>(p, i as i32);
    }

    let tbl_7b0 = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let mut saved_tbl = [0u8; 41 * 16];
    core::ptr::copy_nonoverlapping(tbl_7b0 as *const u8, saved_tbl.as_mut_ptr(), 41 * 16);
    for i in 0..41usize {
        let e = tbl_7b0 + i * 16;
        wr::<i32>(e, (i as i32) * 8);
        wr::<i32>(e + 0x04, 0);
        wr::<i32>(e + 0x08, if i >= 36 { 4 } else { 8 });
        wr::<i32>(e + 0x0c, i as i32);
    }

    const SAMPLER_SRCS: [u64; 22] = [
        0x1_43a3_4f48,
        0x1_43a3_48d8,
        0x1_43a3_49b8,
        0x1_43a3_4e78,
        0x1_43a3_47b8,
        0x1_43a3_53b8,
        0x1_43a3_5148,
        0x1_43a3_4df8,
        0x1_43a3_4a38,
        0x1_43a3_5328,
        0x1_43a3_4b68,
        0x1_43a3_5898,
        0x1_43a3_55f8,
        0x1_43a3_51f8,
        0x1_43a3_4848,
        0x1_43a3_5448,
        0x1_43a3_4d08,
        0x1_43a3_4728,
        0x1_43a3_4fc8,
        0x1_43a3_4d58,
        0x1_43a3_57b8,
        0x1_43a3_5498,
    ];
    static mut SAMPLER_HANDLES: [u64; 22] = [0; 22];
    static mut SAMPLER_PTRS: [usize; 22] = [0; 22];
    let mut saved_sampler_srcs = [0usize; 22];
    for (i, &va) in SAMPLER_SRCS.iter().enumerate() {
        SAMPLER_HANDLES[i] = 0x7000_0000_0000_0000u64 + (i as u64) * 0x101;
        SAMPLER_PTRS[i] = core::ptr::addr_of_mut!(SAMPLER_HANDLES[i]) as usize;
        let p = exe::ptr::<usize>(va) as usize;
        saved_sampler_srcs[i] = rd::<usize>(p);
        wr::<usize>(p, core::ptr::addr_of_mut!(SAMPLER_PTRS[i]) as usize);
    }

    let tls0 = rd::<usize>(exe::tls_pointer());
    let mut saved_tls_63d0 = [0u8; 41 * 8];
    let mut saved_tls_463e0 = [0u8; 41 * 4];
    core::ptr::copy_nonoverlapping(
        (tls0 + 0x63d0) as *const u8,
        saved_tls_63d0.as_mut_ptr(),
        41 * 8,
    );
    core::ptr::copy_nonoverlapping(
        (tls0 + 0x463e0) as *const u8,
        saved_tls_463e0.as_mut_ptr(),
        41 * 4,
    );

    let vtbl_slot: [usize; 2] = [0, mock_vtbl_08 as *const () as usize];
    let fake_com_obj: [usize; 1] = [vtbl_slot.as_ptr() as usize];
    let cb_list: [usize; 1] = [fake_com_obj.as_ptr() as usize];

    let mut fc58_buf = [0u8; 0xc0];
    let mut fcb8_buf = [0u8; 0x500];

    let saved_fc58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let saved_fc48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc48) as usize);
    let saved_fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let saved_31f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a6_31f8) as usize);
    let saved_3200 = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3200) as usize);
    let saved_3a68 = rd::<u32>(exe::ptr::<u32>(0x1_43a3_3a68) as usize);
    let saved_3b98 = rd::<u32>(exe::ptr::<u32>(0x1_43a3_3b98) as usize);

    const MISC_QWORDS: [u64; 49] = [
        0x1_4397_d8b0,
        0x1_4397_d8b8,
        0x1_4397_d8c0,
        0x1_4397_d8c8,
        0x1_4397_d8d0,
        0x1_43a2_8cc4,
        0x1_4386_98b4,
        0x1_43a2_8d34,
        0x1_4385_82bc,
        0x1_43a6_3760,
        0x1_43a6_374c,
        0x1_4386_a43c,
        0x1_4386_ac64,
        0x1_43a2_8c3f,
        0x1_43bb_d128,
        0x1_43bb_d114,
        0x1_4397_ff50,
        0x1_4385_5868,
        0x1_4397_ff30,
        0x1_43c8_c810,
        0x1_43a2_b1f8,
        0x1_4384_3fce,
        0x1_4397_fda0,
        0x1_4397_fd90,
        0x1_4385_7a28,
        0x1_43cf_3720,
        0x1_43cf_3728,
        0x1_43cf_3730,
        0x1_43cf_3740,
        0x1_43a0_0168,
        0x1_43a0_017c,
        0x1_43a0_0144,
        0x1_43a0_0188,
        0x1_43c8_c7f0,
        0x1_43bb_d480,
        0x1_4386_a040,
        0x1_4386_9948,
        0x1_4386_a3a0,
        0x1_4386_a4c8,
        0x1_4386_9b10,
        0x1_4386_a088,
        0x1_4386_9ad8,
        0x1_4386_b898,
        0x1_4385_83cf,
        0x1_43cf_3180,
        0x1_43cf_4998,
        0x1_43bb_ce30,
        0x1_43bb_ce1c,
        0x1_4386_8064,
    ];
    let mut saved_misc = [0u64; 49];
    for (i, &va) in MISC_QWORDS.iter().enumerate() {
        saved_misc[i] = rd::<u64>(exe::ptr::<u64>(va) as usize);
    }

    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc58) as usize,
        fc58_buf.as_mut_ptr() as usize,
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc48) as usize,
        fake_com_obj.as_ptr() as usize,
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fcb8) as usize,
        fcb8_buf.as_mut_ptr() as usize,
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a6_31f8) as usize,
        cb_list.as_ptr() as usize,
    );
    wr::<u32>(exe::ptr::<u32>(0x1_43a6_3200) as usize, 1);
    wr::<u32>(exe::ptr::<u32>(0x1_43a3_3a68) as usize, 1920);
    wr::<u32>(exe::ptr::<u32>(0x1_43a3_3b98) as usize, 960);

    let mut run = |f: F, mode: u8| -> (u64, [u8; 16], [u64; 26], u64, u32) {
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        core::ptr::write_bytes(fcb8_buf.as_mut_ptr(), 0, 0x500);
        core::ptr::write_bytes((tls0 + 0x63d0) as *mut u8, 0, 41 * 8);
        core::ptr::write_bytes((tls0 + 0x463e0) as *mut u8, 0, 41 * 4);

        fc58_buf[0xac] = if mode == 0 { 0 } else { 1 };
        wr::<u32>(exe::ptr::<u32>(0x1_43bb_d480) as usize, if mode == 1 { 1 } else { 0 });
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8d34) as usize, u8::from(mode == 2));
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc5) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_43bb_d128) as usize, 3);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a040) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_9948) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a3a0) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a4c8) as usize, u8::from(mode == 1));
        wr::<u8>(exe::ptr::<u8>(0x1_4386_9b10) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a088) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_9ad8) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_b898) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_83cf) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8cf) as usize, u8::from(mode == 0));
        wr::<u32>(exe::ptr::<u32>(0x1_4385_7a28) as usize, u32::MAX);
        wr::<u32>(exe::ptr::<u32>(0x1_4385_7a2c) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_43cf_3740) as usize, 0);
        wr::<i64>(exe::ptr::<i64>(0x1_43cf_3720) as usize, 1_000_000);
        wr::<f64>(exe::ptr::<f64>(0x1_43cf_3728) as usize, 2.0);
        wr::<f64>(exe::ptr::<f64>(0x1_43a0_0168) as usize, 10.0);
        wr::<u32>(exe::ptr::<u32>(0x1_43a0_0144) as usize, (mode as u32) % 3);
        wr::<u64>(exe::ptr::<u64>(0x1_43a0_0188) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_43c8_c7f0) as usize, u8::from(mode == 0));

        let mut p48_buf = [0u8; 0x80];
        p48_buf[0x4c] = u8::from(mode != 0);
        p48_buf[0x66] = u8::from(mode == 2);
        wr::<f32>(p48_buf.as_mut_ptr() as usize + 0x50, 400.0);

        let mut p150_buf = [0u32; 4];
        p150_buf[2] = 0x9999;

        let mut p1_buf = [0u8; 0x700];
        let p1p = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x48, p48_buf.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x50, if mode != 0 { 1 } else { 0 });
        wr::<usize>(p1p + 0x150, p150_buf.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x1a8, 0x1a80);
        wr::<usize>(p1p + 0x1d0, 0x1d00);
        wr::<usize>(p1p + 0x1e8, 0x1e80);
        wr::<usize>(p1p + 0x200, 0x2000);
        wr::<usize>(p1p + 0x230, 0x2300);
        wr::<u8>(p1p + 0x49d, mode);
        wr::<f32>(p1p + 0x68c, 2.25);

        f(p1p);

        let flags = [
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8d6) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8d3) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8c7) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8be) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8c3) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ce) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8d2) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8b3) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8bf) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4397_d8cf) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_43c8_c7f0) as usize),
            rd::<u8>(p1p + 0x488),
            rd::<u8>(p1p + 0x489),
            rd::<u8>(exe::ptr::<u8>(0x1_43cf_3cb0) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_43cf_4e88) as usize),
        ];
        let ctx_p = fcb8_buf.as_ptr() as usize;
        let ctx_handles = rd::<[u64; 26]>(ctx_p + 0x2b8);
        let ctx_cnt = rd::<u64>(ctx_p + 0x4b8);
        (TRACE_HASH, flags, ctx_handles, ctx_cnt, p150_buf[2])
    };

    let res = (|| -> Result<u32, String> {
        for mode in [0u8, 1, 2] {
            let o = run(f_o, mode);
            let n = run(f_n, mode);
            crate::diff_eq!(o, n, "142299ad0 mode={}", mode);
        }
        Ok(3)
    })();

    for (i, &va) in MISC_QWORDS.iter().enumerate().rev() {
        wr::<u64>(exe::ptr::<u64>(va) as usize, saved_misc[i]);
    }
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize, saved_fc58);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc48) as usize, saved_fc48);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, saved_fcb8);
    wr::<usize>(exe::ptr::<usize>(0x1_43a6_31f8) as usize, saved_31f8);
    wr::<u32>(exe::ptr::<u32>(0x1_43a6_3200) as usize, saved_3200);
    wr::<u32>(exe::ptr::<u32>(0x1_43a3_3a68) as usize, saved_3a68);
    wr::<u32>(exe::ptr::<u32>(0x1_43a3_3b98) as usize, saved_3b98);

    core::ptr::copy_nonoverlapping(
        saved_tls_63d0.as_ptr(),
        (tls0 + 0x63d0) as *mut u8,
        41 * 8,
    );
    core::ptr::copy_nonoverlapping(
        saved_tls_463e0.as_ptr(),
        (tls0 + 0x463e0) as *mut u8,
        41 * 4,
    );
    for (i, &va) in SAMPLER_SRCS.iter().enumerate() {
        wr::<usize>(exe::ptr::<usize>(va) as usize, saved_sampler_srcs[i]);
    }
    core::ptr::copy_nonoverlapping(saved_tbl.as_ptr(), tbl_7b0 as *mut u8, 41 * 16);
    for (i, &va) in all_ids.iter().enumerate() {
        wr::<[u8; 24]>(exe::ptr::<u8>(va) as usize, saved_id_blocks[i]);
    }
    for (i, &va) in all_guards.iter().enumerate() {
        wr::<i32>(exe::ptr::<i32>(va) as usize, saved_guards[i]);
    }
    res
}
