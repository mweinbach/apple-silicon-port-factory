//! Decompiled hot functions shard N1 (`0x14226bb40`, `0x142306c20`).

use crate::exe::{self, rd, wr};

/// `unknown_14226bb40`
/// Original: `0x14226bb40` (3682 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_14226bb40(param_1: usize) {
    exe::call_va!(0x1_4226_8dd0, (param_1, 0u32));
    if rd::<u8>(exe::ptr::<u8>(0x1_43a2_8cc6) as usize) != 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4386_a630) as usize) == 0
        && rd::<u64>(param_1 + 0x448) != 0
    {
        return;
    }

    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    let r15_tls = tls0.wrapping_add(0x467e0);

    let c0 = rd::<f32>(param_1 + 0x48c);
    let c1 = rd::<f32>(param_1 + 0x490);
    let c2 = rd::<f32>(param_1 + 0x494);
    let c3_bits = rd::<u32>(param_1 + 0x498);

    let p48 = rd::<usize>(param_1 + 0x48);
    let cl_4c = rd::<u8>(p48 + 0x4c);
    let xmm6 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let c_634 = rd::<f32>(exe::ptr::<f32>(0x1_4384_1634) as usize);
    let scale = if cl_4c == 0 { xmm6 } else { c_634 };

    let mut xmm7 = scale * c0;
    let mut xmm8 = scale * c1;
    let mut xmm9 = scale * c2;

    if cl_4c != 0 && rd::<u8>(param_1 + 0x49d) == 0 {
        let m_50c = rd::<f32>(exe::ptr::<f32>(0x1_4342_650c) as usize);
        let m_59c = rd::<f32>(exe::ptr::<f32>(0x1_4342_659c) as usize);
        let m_88c = rd::<f32>(exe::ptr::<f32>(0x1_4342_688c) as usize);
        let xmm5 = (xmm7 * m_50c + xmm8 * m_59c) + xmm9 * m_88c;

        let m_89c = rd::<f32>(exe::ptr::<f32>(0x1_4342_689c) as usize);
        let m_578 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6578) as usize);
        let m_4f0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_64f0) as usize);
        let xmm4 = (xmm8 * m_89c + xmm7 * m_578) + xmm9 * m_4f0;

        let m_7ec = rd::<f32>(exe::ptr::<f32>(0x1_4342_67ec) as usize);
        let m_6c8 = rd::<f32>(exe::ptr::<f32>(0x1_4342_66c8) as usize);
        let m_558 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6558) as usize);
        let xmm0 = (xmm7 * m_7ec + xmm8 * m_6c8) + xmm9 * m_558;

        let m_844 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6844) as usize);
        xmm7 = xmm0 * m_844;
        xmm8 = xmm4 * m_844;
        xmm9 = xmm5 * m_844;
    }

    let mut clear_col: [u32; 4] = [xmm7.to_bits(), xmm8.to_bits(), xmm9.to_bits(), c3_bits];

    if rd::<u64>(param_1 + 0x448) == 0 {
        wr::<u8>(param_1 + 0x460, 1);
        let mut desc_20 = [0u8; 0x30];
        let dp = desc_20.as_mut_ptr() as usize;
        wr::<u32>(dp, rd::<u32>(exe::ptr::<u32>(0x1_43a3_3c28) as usize));
        wr::<u32>(dp + 0x04, rd::<u32>(exe::ptr::<u32>(0x1_43a3_3c2c) as usize));
        wr::<u32>(dp + 0x08, 1);
        wr::<u32>(dp + 0x0c, 1);
        wr::<u8>(dp + 0x10, rd::<u8>(param_1 + 0x488));
        wr::<u64>(dp + 0x14, 1);
        wr::<u128>(dp + 0x1c, 0);
        wr::<u32>(dp + 0x2c, 0xffff_ffff);

        let rbx_58 = rd::<usize>(param_1 + 0x58);
        let r9_slot = exe::call_va!(0x1_412c_f460, (param_1 + 0x448) -> usize);
        let rax_1e0 = exe::call_va!(
            0x1_4225_81e0,
            (
                rbx_58,
                dp,
                exe::ptr::<u8>(0x1_433f_5dd0) as usize,
                r9_slot
            ) -> usize
        );
        let target_448 = rd::<usize>(rd::<usize>(param_1 + 0x448));
        exe::call_va!(
            0x1_4206_edb0,
            (rax_1e0, target_448, clear_col.as_mut_ptr() as usize)
        );
    }

    let r14 = rd::<usize>(rd::<usize>(param_1 + 0x448));
    let p1c0 = rd::<usize>(param_1 + 0x1c0);
    if p1c0 != 0 {
        let f_17c = rd::<u32>(exe::ptr::<u32>(0x1_43a0_017c) as usize);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_421f_1d50,
            p1c0 as u64,
            f_17c as u64,
            2,
            0,
            0,
            0,
            0,
            0,
            [0, f_17c as u128, 0, 0],
        );
    }

    let mut hud_ds_desc = [0u8; 0x18];
    let hdp = hud_ds_desc.as_mut_ptr() as usize;
    wr::<u32>(hdp, rd::<u32>(r14));
    wr::<u32>(hdp + 0x04, rd::<u32>(r14 + 0x04));
    wr::<u8>(hdp + 0x08, 2);
    wr::<u64>(hdp + 0x0c, 8);
    let rbx_hud_ds = exe::call_va!(
        0x1_4225_7c60,
        (
            rd::<usize>(param_1 + 0x58),
            hdp,
            exe::ptr::<u8>(0x1_433f_5da8) as usize
        ) -> usize
    );
    let hud_ds_target = rd::<usize>(rbx_hud_ds);

    let mut rt_100 = [0u8; 0x118];
    let rt_100_ptr = rt_100.as_mut_ptr() as usize;
    wr::<u32>(rt_100_ptr, 1);
    wr::<usize>(rt_100_ptr + 0xc8, r14);
    wr::<usize>(rt_100_ptr + 0x108, hud_ds_target);
    exe::call_va!(0x1_4205_49b0, (rt_100_ptr, 0u32));

    wr::<u32>(r15_tls + 0xa84, 0);
    wr::<u32>(r15_tls + 0xa88, 0);
    wr::<f32>(
        r15_tls + 0xa8c,
        rd::<i32>(exe::ptr::<i32>(0x1_43a3_3c28) as usize) as f32,
    );
    wr::<f32>(
        r15_tls + 0xa90,
        rd::<i32>(exe::ptr::<i32>(0x1_43a3_3c2c) as usize) as f32,
    );
    wr::<u32>(
        r15_tls + 0xa94,
        rd::<u32>(exe::ptr::<u32>(0x1_43a0_0154) as usize),
    );
    wr::<u32>(
        r15_tls + 0xa98,
        rd::<u32>(exe::ptr::<u32>(0x1_4384_1708) as usize),
    );
    exe::call_va!(0x1_4206_f8c0, (r15_tls, r15_tls + 0xa84));
    exe::call_va!(
        0x1_4229_d440,
        (exe::ptr::<u8>(0x1_43a3_3c28) as usize)
    );
    exe::call_va!(0x1_4223_9b60, (rd::<usize>(param_1 + 0xa8), 1u32));

    if rd::<u64>(param_1 + 0x5a8) != 0 {
        let mut quad_verts: [u128; 4] = [
            rd::<u128>(exe::ptr::<u8>(0x1_4342_81d0) as usize),
            rd::<u128>(exe::ptr::<u8>(0x1_4342_96a0) as usize),
            rd::<u128>(exe::ptr::<u8>(0x1_4342_96b0) as usize),
            rd::<u128>(exe::ptr::<u8>(0x1_4342_96a0) as usize),
        ];
        let c_0_5 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
        let c_0_0 = 0.0f32;
        let c_2_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize);
        let row2_const = rd::<u128>(exe::ptr::<u8>(0x1_4342_7a60) as usize);

        for axis in 0..2usize {
            let rbx_off = axis * 4;
            let v = rd::<f32>(param_1 + 0x4a0 + rbx_off);
            if v != xmm6 {
                let xmm1 = (xmm6 - (xmm6 / v)) * c_0_5;
                let mut rect: [f32; 4] = [xmm6, xmm6, c_0_0, c_0_0];
                rect[axis] = xmm1;
                rect[axis + 2] = xmm6 - xmm1;

                exe::call_va!(0x1_422e_6dc0, ());

                let xmm3 = rect[0] - c_0_0;
                let xmm2 = rect[1] - c_0_0;
                let mut mat1 = [0u32; 16];
                mat1[0] = xmm3.to_bits();
                mat1[5] = xmm2.to_bits();
                wr::<u128>(mat1.as_mut_ptr() as usize + 0x20, row2_const);
                mat1[12] = ((xmm3 - xmm6) + c_0_0).to_bits();
                mat1[13] = ((xmm6 - xmm2) - c_0_0).to_bits();
                mat1[14] = c_0_0.to_bits();
                mat1[15] = xmm6.to_bits();
                exe::call_va!(
                    0x1_422e_58b0,
                    (mat1.as_mut_ptr() as usize, quad_verts.as_mut_ptr() as usize)
                );

                let xmm4 = xmm6 - rect[2];
                let xmm1_b = xmm6 - rect[3];
                let mut mat2 = [0u32; 16];
                mat2[0] = xmm4.to_bits();
                mat2[5] = xmm1_b.to_bits();
                wr::<u128>(mat2.as_mut_ptr() as usize + 0x20, row2_const);
                let xmm3_b = (xmm6 - xmm1_b) - (c_2_0 * rect[3]);
                let xmm0_b = (xmm4 - xmm6) + (c_2_0 * rect[2]);
                mat2[12] = xmm0_b.to_bits();
                mat2[13] = xmm3_b.to_bits();
                mat2[14] = c_0_0.to_bits();
                mat2[15] = xmm6.to_bits();
                exe::call_va!(
                    0x1_422e_58b0,
                    (mat2.as_mut_ptr() as usize, quad_verts.as_mut_ptr() as usize)
                );
            }
        }

        let b428 = rd::<u8>(param_1 + 0x428);
        let need_create_ss = if b428 != 0 {
            true
        } else if rd::<u8>(param_1 + 0x429) == 0 {
            false
        } else {
            let rcx_430 = rd::<usize>(param_1 + 0x430);
            rcx_430 == 0
                || rd::<u32>(rcx_430 + 0x08) != rd::<u32>(r14)
                || rd::<u32>(rcx_430 + 0x0c) != rd::<u32>(r14 + 0x04)
                || rd::<u8>(rcx_430 + 0x14) != rd::<u8>(r14 + 0x18)
        };

        let tbl_8590 = exe::ptr::<u8>(0x1_4385_8590) as isize;
        let tbl_37b8 = exe::ptr::<u8>(0x1_43a6_37b8) as isize;

        if need_create_ss {
            let rbx_f18 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize);
            let r9_slot = exe::call_va!(0x1_412c_f460, (param_1 + 0x430) -> usize);
            let mut desc_50 = [0u8; 0x30];
            let dp = desc_50.as_mut_ptr() as usize;
            wr::<u32>(dp, rd::<u32>(r14));
            wr::<u32>(dp + 0x04, rd::<u32>(r14 + 0x04));
            wr::<u32>(dp + 0x08, 1);
            wr::<u32>(dp + 0x0c, 1);
            wr::<u8>(dp + 0x10, rd::<u8>(r14 + 0x18));
            wr::<u64>(dp + 0x14, 1);
            wr::<u128>(dp + 0x1c, 0);
            wr::<u32>(dp + 0x2c, 0xffff_ffff);
            exe::call_va!(
                0x1_4225_81e0,
                (
                    rbx_f18,
                    dp,
                    exe::ptr::<u8>(0x1_433f_5d90) as usize,
                    r9_slot
                )
            );

            let rdx_430 = rd::<usize>(rd::<usize>(param_1 + 0x430));
            let mut rt_2e0 = [0u8; 0x118];
            let rt_2e0_ptr = rt_2e0.as_mut_ptr() as usize;
            exe::call_va!(0x1_4205_4c80, (rt_2e0_ptr));
            wr::<u32>(rt_2e0_ptr, 1);
            wr::<usize>(rt_2e0_ptr + 0xc8, rdx_430);
            wr::<usize>(rt_2e0_ptr + 0x108, 0);
            exe::call_va!(0x1_4205_49b0, (rt_2e0_ptr, 0u32));

            let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
            let g_4e94 = exe::ptr::<i32>(0x1_43cf_4e94) as usize;
            if rd::<i32>(g_4e94) > tls_epoch {
                exe::call_va!(0x1_4183_4a78, (g_4e94));
                if rd::<i32>(g_4e94) == -1 {
                    let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f280) as u64, 0xd];
                    exe::call_va!(
                        0x1_4220_ea70,
                        (
                            exe::ptr::<u8>(0x1_43cf_2b50) as usize,
                            sv.as_mut_ptr() as usize
                        )
                    );
                    wr::<u32>(exe::ptr::<u32>(0x1_43cf_2b54) as usize, 9);
                    wr::<usize>(
                        exe::ptr::<usize>(0x1_43cf_2b48) as usize,
                        exe::ptr::<u8>(0x1_43cf_2b58) as usize,
                    );
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_2b58) as usize, 0);
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_2b60) as usize, 0);
                    exe::call_va!(
                        0x1_4183_4d18,
                        (exe::ptr::<u8>(0x1_4262_ede0) as usize)
                    );
                    exe::call_va!(0x1_4183_4a0c, (g_4e94));
                }
            }

            let r10 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_2b50) as usize) as isize;
            let ebx = rd::<i32>((tbl_8590 + r10 * 4) as usize);
            let r15_fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
            let val_0 = if ebx == 0 {
                rd::<u64>(r14 + 0x78)
            } else if ebx == 5 {
                rd::<u64>(r14 + 0x90)
            } else {
                rd::<u64>(r15_fcb8 + 0x88)
            };
            let mut pair_60: [u64; 2] = [val_0, r14 as u64];
            let r8_off = rd::<i32>((tbl_37b8 + r10 * 16) as usize) as i64 as u64;
            exe::call_va!(
                0x1_4209_a900,
                (
                    r10 as u32,
                    0u32,
                    r8_off,
                    pair_60.as_mut_ptr() as usize
                )
            );
            wr::<usize>(exe::ptr::<usize>(0x1_43cf_2b60) as usize, r14);
            let val_1 = if ebx == 0 {
                rd::<u64>(r14 + 0x78)
            } else if ebx == 5 {
                rd::<u64>(r14 + 0x90)
            } else {
                rd::<u64>(r15_fcb8 + 0x88)
            };
            wr::<u64>(exe::ptr::<u64>(0x1_43cf_2b58) as usize, val_1);

            let xmm7_c = if rd::<u8>(exe::ptr::<u8>(0x1_4397_d82f) as usize) != 0 {
                xmm6 / rd::<f32>(exe::ptr::<f32>(0x1_4384_1638) as usize)
            } else {
                xmm6
            };

            let tls_epoch2 = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
            let g_5390 = exe::ptr::<i32>(0x1_43cf_5390) as usize;
            if rd::<i32>(g_5390) > tls_epoch2 {
                exe::call_va!(0x1_4183_4a78, (g_5390));
                if rd::<i32>(g_5390) == -1 {
                    let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433f_5d80) as u64, 0xf];
                    exe::call_va!(
                        0x1_4220_ea70,
                        (
                            exe::ptr::<u8>(0x1_43cf_4a20) as usize,
                            sv.as_mut_ptr() as usize
                        )
                    );
                    wr::<u32>(exe::ptr::<u32>(0x1_43cf_4a24) as usize, 3);
                    wr::<usize>(
                        exe::ptr::<usize>(0x1_43cf_4a18) as usize,
                        exe::ptr::<u8>(0x1_43cf_4a28) as usize,
                    );
                    wr::<u128>(exe::ptr::<u128>(0x1_43cf_4a28) as usize, 0);
                    exe::call_va!(
                        0x1_4183_4d18,
                        (exe::ptr::<u8>(0x1_4262_edd0) as usize)
                    );
                    exe::call_va!(0x1_4183_4a0c, (g_5390));
                }
            }

            let mut col_30: [f32; 4] = [xmm7_c, xmm7_c, xmm7_c, xmm6];
            let rcx_a20 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_4a20) as usize) as isize;
            let r8_off2 = rd::<i32>((tbl_37b8 + rcx_a20 * 16) as usize) as i64 as u64;
            exe::call_va!(
                0x1_4209_a900,
                (
                    rcx_a20 as i64 as u64,
                    0u32,
                    r8_off2,
                    col_30.as_mut_ptr() as usize
                )
            );
            wr::<u128>(
                exe::ptr::<u128>(0x1_43cf_4a28) as usize,
                rd::<u128>(col_30.as_ptr() as usize),
            );
            exe::call_va!(0x1_422e_6dc0, ());

            let g_820 = exe::ptr::<u64>(0x1_43c8_c820) as usize;
            let mut shader_820 = rd::<u64>(g_820);
            if shader_820 == 0 {
                let mut sv_rfx: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                let rfx = exe::call_va!(
                    0x1_4239_70b0,
                    (
                        rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                        sv_rfx.as_mut_ptr() as usize
                    ) -> u64
                );
                let mut sv_copy: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f278) as u64, 4];
                let tech =
                    exe::call_va!(0x1_4239_8a00, (rfx, sv_copy.as_mut_ptr() as usize) -> u64);
                wr::<u64>(g_820, tech);
                exe::call_va!(0x1_4239_7700, (g_820));
                shader_820 = rd::<u64>(g_820);
            }
            let pass_obj = exe::call_va!(0x1_4220_e860, (shader_820, 0x10u32) -> u64);
            exe::call_va!(0x1_4224_ded0, (pass_obj));
            exe::call_va!(0x1_422e_6b00, ());
            exe::call_va!(0x1_4205_49b0, (rt_100_ptr, 0u32));
        }

        if rd::<u8>(param_1 + 0x429) != 0 {
            if rd::<u64>(param_1 + 0x430) != 0 {
                let mut rt_400 = [0u8; 0x118];
                let rt_400_ptr = rt_400.as_mut_ptr() as usize;
                exe::call_va!(0x1_4205_4c80, (rt_400_ptr));
                wr::<u32>(rt_400_ptr, 1);
                wr::<usize>(rt_400_ptr + 0xc8, r14);
                wr::<usize>(rt_400_ptr + 0x108, 0);
                exe::call_va!(0x1_4205_49b0, (rt_400_ptr, 0u32));

                let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                let g_30e0 = exe::ptr::<i32>(0x1_43cf_30e0) as usize;
                if rd::<i32>(g_30e0) > tls_epoch {
                    exe::call_va!(0x1_4183_4a78, (g_30e0));
                    if rd::<i32>(g_30e0) == -1 {
                        let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f280) as u64, 0xd];
                        exe::call_va!(
                            0x1_4220_ea70,
                            (
                                exe::ptr::<u8>(0x1_43cf_4698) as usize,
                                sv.as_mut_ptr() as usize
                            )
                        );
                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_469c) as usize, 9);
                        wr::<usize>(
                            exe::ptr::<usize>(0x1_43cf_4690) as usize,
                            exe::ptr::<u8>(0x1_43cf_46a0) as usize,
                        );
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_46a0) as usize, 0);
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_46a8) as usize, 0);
                        exe::call_va!(
                            0x1_4183_4d18,
                            (exe::ptr::<u8>(0x1_4262_edc0) as usize)
                        );
                        exe::call_va!(0x1_4183_4a0c, (g_30e0));
                    }
                }

                let rbx_tex = rd::<usize>(rd::<usize>(param_1 + 0x430));
                let r10 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_4698) as usize) as isize;
                let r12d = rd::<i32>((tbl_8590 + r10 * 4) as usize);
                let r15_fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
                let val_0 = if r12d == 0 {
                    if rbx_tex != 0 {
                        rd::<u64>(rbx_tex + 0x78)
                    } else {
                        rd::<u64>(r15_fcb8 + 0x88)
                    }
                } else if r12d == 5 {
                    if rbx_tex != 0 {
                        rd::<u64>(rbx_tex + 0x90)
                    } else {
                        rd::<u64>(r15_fcb8 + 0x90)
                    }
                } else {
                    rd::<u64>(r15_fcb8 + 0x88)
                };
                let mut pair_70: [u64; 2] = [val_0, rbx_tex as u64];
                let r8_off = rd::<i32>((tbl_37b8 + r10 * 16) as usize) as i64 as u64;
                exe::call_va!(
                    0x1_4209_a900,
                    (
                        r10 as u32,
                        0u32,
                        r8_off,
                        pair_70.as_mut_ptr() as usize
                    )
                );
                wr::<usize>(exe::ptr::<usize>(0x1_43cf_46a8) as usize, rbx_tex);
                let val_1 = if r12d == 0 {
                    if rbx_tex != 0 {
                        rd::<u64>(rbx_tex + 0x78)
                    } else {
                        rd::<u64>(r15_fcb8 + 0x88)
                    }
                } else if r12d == 5 {
                    if rbx_tex != 0 {
                        rd::<u64>(rbx_tex + 0x90)
                    } else {
                        rd::<u64>(r15_fcb8 + 0x90)
                    }
                } else {
                    rd::<u64>(r15_fcb8 + 0x88)
                };
                wr::<u64>(exe::ptr::<u64>(0x1_43cf_46a0) as usize, val_1);

                let xmm7_c = if rd::<u8>(exe::ptr::<u8>(0x1_4397_d82f) as usize) == 0 {
                    xmm6
                } else {
                    rd::<f32>(exe::ptr::<f32>(0x1_4384_1634) as usize)
                };

                let tls_epoch2 = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                let g_4930 = exe::ptr::<i32>(0x1_43cf_4930) as usize;
                if rd::<i32>(g_4930) > tls_epoch2 {
                    exe::call_va!(0x1_4183_4a78, (g_4930));
                    if rd::<i32>(g_4930) == -1 {
                        let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433f_5d80) as u64, 0xf];
                        exe::call_va!(
                            0x1_4220_ea70,
                            (
                                exe::ptr::<u8>(0x1_43cf_3a38) as usize,
                                sv.as_mut_ptr() as usize
                            )
                        );
                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_3a3c) as usize, 3);
                        wr::<usize>(
                            exe::ptr::<usize>(0x1_43cf_3a30) as usize,
                            exe::ptr::<u8>(0x1_43cf_3a40) as usize,
                        );
                        wr::<u128>(exe::ptr::<u128>(0x1_43cf_3a40) as usize, 0);
                        exe::call_va!(
                            0x1_4183_4d18,
                            (exe::ptr::<u8>(0x1_4262_edb0) as usize)
                        );
                        exe::call_va!(0x1_4183_4a0c, (g_4930));
                    }
                }

                let mut col_40: [f32; 4] = [xmm7_c, xmm7_c, xmm7_c, xmm6];
                let rcx_a38 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_3a38) as usize) as isize;
                let r8_off2 = rd::<i32>((tbl_37b8 + rcx_a38 * 16) as usize) as i64 as u64;
                exe::call_va!(
                    0x1_4209_a900,
                    (
                        rcx_a38 as i64 as u64,
                        0u32,
                        r8_off2,
                        col_40.as_mut_ptr() as usize
                    )
                );
                wr::<u128>(
                    exe::ptr::<u128>(0x1_43cf_3a40) as usize,
                    rd::<u128>(col_40.as_ptr() as usize),
                );
                exe::call_va!(0x1_422e_6dc0, ());

                let g_8c8 = exe::ptr::<u64>(0x1_43c8_c8c8) as usize;
                let mut shader_8c8 = rd::<u64>(g_8c8);
                if shader_8c8 == 0 {
                    let mut sv_rfx: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                    let rfx = exe::call_va!(
                        0x1_4239_70b0,
                        (
                            rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                            sv_rfx.as_mut_ptr() as usize
                        ) -> u64
                    );
                    let mut sv_copy: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f278) as u64, 4];
                    let tech =
                        exe::call_va!(0x1_4239_8a00, (rfx, sv_copy.as_mut_ptr() as usize) -> u64);
                    wr::<u64>(g_8c8, tech);
                    exe::call_va!(0x1_4239_7700, (g_8c8));
                    shader_8c8 = rd::<u64>(g_8c8);
                }
                let pass_obj = exe::call_va!(0x1_4220_e860, (shader_8c8, 0x10u32) -> u64);
                exe::call_va!(0x1_4224_ded0, (pass_obj));
                exe::call_va!(0x1_422e_6b00, ());
                exe::call_va!(0x1_4205_49b0, (rt_100_ptr, 0u32));
            }
        } else if rd::<u8>(param_1 + 0x428) == 0 {
            let old_430 = rd::<u64>(param_1 + 0x430);
            wr::<u64>(param_1 + 0x430, 0);
            if old_430 != 0 {
                let rcx_f18 = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize);
                exe::call_va!(0x1_4225_8080, (rcx_f18, old_430));
            }
        }

        let _ = crate::simd_ops::atomic_xchg_u8((param_1 + 0x428) as u64, 0);

        let p5a8 = rd::<usize>(param_1 + 0x5a8);
        let vtbl0 = rd::<usize>(p5a8);
        exe::call_va!(rd::<usize>(vtbl0), (p5a8, r14));

        let p5a8 = rd::<usize>(param_1 + 0x5a8);
        let vtbl1 = rd::<usize>(p5a8);
        exe::call_va!(
            rd::<usize>(vtbl1 + 0x08),
            (p5a8, exe::ptr::<u8>(0x1_43a3_3958) as usize)
        );

        let p5a8 = rd::<usize>(param_1 + 0x5a8);
        let vtbl2 = rd::<usize>(p5a8);
        let flag_4c = rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c);
        exe::call_va!(
            rd::<usize>(vtbl2 + 0x10),
            (p5a8, rd::<u32>(r14), rd::<u32>(r14 + 0x04), flag_4c)
        );

        let p5a8 = rd::<usize>(param_1 + 0x5a8);
        let vtbl3 = rd::<usize>(p5a8);
        let flag_4c = rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c);
        exe::call_va!(
            rd::<usize>(vtbl3 + 0x18),
            (p5a8, r14, rd::<u64>(rbx_hud_ds), flag_4c)
        );

        let p5a8 = rd::<usize>(param_1 + 0x5a8);
        let vtbl4 = rd::<usize>(p5a8);
        exe::call_va!(rd::<usize>(vtbl4), (p5a8, 0u64));

        exe::call_va!(0x1_4205_49b0, (rt_100_ptr, 0u32));
    }

    exe::call_va!(0x1_4223_9b60, (rd::<usize>(param_1 + 0xa8), 2u32));
    exe::call_va!(
        0x1_4225_7af0,
        (
            rd::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize),
            rbx_hud_ds
        )
    );
}

/// `unknown_142306c20`
/// Original: `0x142306c20` (1621 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_142306c20(param_1: usize, param_2: usize) {
    if param_2 == 0 {
        return;
    }

    let mut vec_50: [u64; 2] = [0, 0];
    let vec_50_ptr = vec_50.as_mut_ptr() as usize;
    exe::call_va!(0x1_410e_f200, (vec_50_ptr, param_2 as u64));

    let mut rdi = rd::<usize>(vec_50_ptr);
    let xmm1_inf = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
    let r10_tbl = exe::ptr::<usize>(0x1_43be_60a8) as usize;
    let c_9a38 = rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize);
    let c_9a40 = rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize);

    for r8 in 0..param_2 {
        let r9 = param_1.wrapping_add(r8 * 0x60);
        let eax = rd::<u32>(r9);
        let rcx_hi = (eax >> 16) as usize;
        let eax_lo = (eax & 0xffff) as usize;
        let rdx = rd::<usize>(r10_tbl.wrapping_add(rcx_hi * 8)).wrapping_add(eax_lo * 0x98);

        wr::<u64>(rdx, 0);
        wr::<u64>(rdx + 0x08, 0);
        wr::<u64>(rdx + 0x10, 0);
        wr::<u64>(rdx + 0x18, 0);
        wr::<u64>(rdx + 0x20, 0);
        wr::<u64>(rdx + 0x30, 0);
        wr::<u64>(rdx + 0x38, 0);
        wr::<u64>(rdx + 0x40, 0);
        wr::<usize>(rdx + 0x28, rdx.wrapping_add(0x38));
        wr::<u32>(rdx + 0x30, 0);
        wr::<u32>(rdx + 0x34, 1);
        wr::<u64>(rdx + 0x50, u64::MAX);
        wr::<u64>(rdx + 0x58, u64::MAX);
        wr::<u64>(rdx + 0x60, u64::MAX);
        wr::<u32>(rdx + 0x68, u32::MAX);
        wr::<u64>(rdx + 0x6c, 0);
        wr::<u64>(rdx + 0x74, 0);
        wr::<u64>(rdx + 0x7c, c_9a38);
        wr::<u32>(rdx + 0x84, c_9a40);
        wr::<u32>(rdx + 0x88, xmm1_inf);
        wr::<u32>(rdx + 0x8c, xmm1_inf);
        wr::<u32>(rdx + 0x90, xmm1_inf);
        wr::<u32>(rdx + 0x4c, rd::<u32>(rdx + 0x4c) & 0xff00_0000);
        wr::<u8>(rdx + 0x4f, 0);
        wr::<usize>(rdi.wrapping_add(r8 * 8), rdx);
    }

    let r8_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let len_58 = rd::<u32>(vec_50_ptr + 0x08) as u64;
    let mut span_70: [u64; 2] = [len_58, rdi as u64];
    exe::call_va!(0x1_4225_d5b0, (r8_mgr, span_70.as_mut_ptr() as usize));

    let mut total_rcx: u64 = 0;
    for r13 in 0..param_2 {
        let item = param_1.wrapping_add(r13 * 0x60);
        let r12d = rd::<u32>(item + 0x10);
        let edx = rd::<u32>(item + 0x14);
        let mut xmm6 = rd::<f32>(item + 0x50);
        let rax_obj = rd::<usize>(item + 0x08);
        let r15 = rd::<usize>(rax_obj + 0xc0);
        let sub_cnt = rd::<i32>(r15 + 0x138) as i64 as u64;
        total_rcx = total_rcx.wrapping_add(sub_cnt);

        let rbx_node = rd::<usize>(rdi.wrapping_add(r13 * 8));
        let cur_4c = rd::<u32>(rbx_node + 0x4c);
        wr::<u32>(rbx_node + 0x4c, cur_4c ^ ((cur_4c ^ r12d) & 0x00ff_ffff));
        wr::<u8>(rbx_node + 0x4f, edx as u8);

        let r14 = rd::<usize>(item + 0x08);
        if r14 != 0 {
            let old = crate::simd_ops::atomic_xadd_u32((r14 + 0x24) as u64, 1);
            if old.wrapping_add(1) == 1 {
                let rcx_28 = rd::<usize>(r14 + 0x28);
                if rcx_28 != 0 {
                    exe::call_va!(0x1_4241_d6f0, (rcx_28, r14));
                }
            }
            rdi = rd::<usize>(vec_50_ptr);
        }

        let old_r14 = rd::<usize>(rbx_node);
        if old_r14 != 0 {
            let old = crate::simd_ops::atomic_xadd_u32((old_r14 + 0x24) as u64, u32::MAX);
            if old == 1 {
                let rcx_28 = rd::<usize>(old_r14 + 0x28);
                if rcx_28 != 0 {
                    exe::call_va!(0x1_4241_d6f0, (rcx_28, old_r14));
                }
            }
            rdi = rd::<usize>(vec_50_ptr);
        }

        wr::<usize>(rbx_node, r14);
        wr::<usize>(rbx_node + 0x08, r15);

        let r10 = rd::<u32>(rbx_node + 0x48) as usize;
        let r8_28 = rd::<usize>(r8_mgr + 0x28);
        let b0 = (!(r12d >> 6) as u8) & 1;
        let b1 = if (r12d & 0x110) != 0 { 2u8 } else { 0u8 };
        let b2 = ((r12d as u8) & 2).wrapping_shl(1);
        wr::<u8>(r8_28.wrapping_add(r10 * 0x0c + 2), b0 | b1 | b2);
        wr::<u8>(rd::<usize>(r8_mgr + 0x218).wrapping_add(r10), edx as u8);

        let word_idx = r10 >> 6;
        let bit_idx = r10 & 0x3f;
        let bit = 1u64 << bit_idx;
        let p180 = rd::<usize>(r8_mgr + 0x180).wrapping_add(word_idx * 8);
        wr::<u64>(p180, rd::<u64>(p180) | bit);

        wr::<u32>(
            rd::<usize>(r8_mgr + 0x198).wrapping_add(r10 * 4),
            rd::<u32>(r15 + 0x26c),
        );
        wr::<u8>(
            rd::<usize>(r8_mgr + 0x1a8).wrapping_add(r10),
            rd::<u8>(r15 + 0x271),
        );
        wr::<u8>(rd::<usize>(r8_mgr + 0x1b8).wrapping_add(r10), 0xff);

        let p38 = rd::<usize>(r8_mgr + 0x38).wrapping_add(word_idx * 8);
        wr::<u64>(p38, rd::<u64>(p38) | bit);

        let b272 = rd::<u8>(r15 + 0x272);
        if b272 != 0 {
            let sub_eax = (b272 as i32) - 1;
            let ecx = if sub_eax < 0 { sub_eax } else { 0 };
            if rd::<i32>((r15 as isize + 0x1c8 + (ecx as isize) * 4) as usize) > 0
                && rd::<i32>(rbx_node + 0x50) == -1
            {
                let mut tmp_40 = [0u8; 16];
                let ret_ptr = exe::call_va!(
                    0x1_4223_0fe0,
                    (
                        rd::<usize>(rbx_node + 0x08).wrapping_add(8),
                        tmp_40.as_mut_ptr() as usize,
                        rd::<u32>(rbx_node + 0x48)
                    ) -> usize
                );
                wr::<u64>(rbx_node + 0x50, rd::<u64>(ret_ptr));
                wr::<u32>(rbx_node + 0x58, rd::<u32>(ret_ptr + 0x08));
            }
        }

        let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
        if rd::<u8>(g_c58 + 0xac) != 0 && (r12d & 1) == 0 {
            exe::call_va!(0x1_4230_40c0, (rbx_node));
        }
        exe::call_va!(0x1_4230_5b70, (rbx_node));

        let r10_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
        let r9_idx = rd::<u32>(rbx_node + 0x48) as usize;
        let p638 = rd::<usize>(r10_mgr + 0x638).wrapping_add((r9_idx >> 6) * 8);
        wr::<u64>(p638, rd::<u64>(p638) | (1u64 << (r9_idx & 0x3f)));

        let dst_5c8 = rd::<usize>(r10_mgr + 0x5c8).wrapping_add(r9_idx * 0x30);
        wr::<u64>(dst_5c8, rd::<u64>(item + 0x20));
        wr::<u32>(dst_5c8 + 0x08, rd::<u32>(item + 0x28));
        wr::<u64>(dst_5c8 + 0x0c, rd::<u64>(item + 0x2c));
        wr::<u32>(dst_5c8 + 0x14, rd::<u32>(item + 0x34));
        wr::<u64>(dst_5c8 + 0x18, rd::<u64>(item + 0x38));
        wr::<u32>(dst_5c8 + 0x20, rd::<u32>(item + 0x40));
        wr::<u64>(dst_5c8 + 0x24, rd::<u64>(item + 0x44));
        wr::<u32>(dst_5c8 + 0x2c, rd::<u32>(item + 0x4c));

        let r8_idx = rd::<u32>(rbx_node + 0x48) as usize;
        let p38_2 = rd::<usize>(r10_mgr + 0x38).wrapping_add((r8_idx >> 6) * 8);
        wr::<u64>(p38_2, rd::<u64>(p38_2) | (1u64 << (r8_idx & 0x3f)));

        let r9_idx2 = rd::<u32>(rbx_node + 0x48) as usize;
        let p650 = rd::<usize>(r10_mgr + 0x650).wrapping_add((r9_idx2 >> 6) * 8);
        wr::<u64>(p650, rd::<u64>(p650) | (1u64 << (r9_idx2 & 0x3f)));

        let dst_5d8 = rd::<usize>(r10_mgr + 0x5d8).wrapping_add(r9_idx2 * 0x30);
        wr::<u64>(dst_5d8, rd::<u64>(item + 0x20));
        wr::<u32>(dst_5d8 + 0x08, rd::<u32>(item + 0x28));
        wr::<u64>(dst_5d8 + 0x0c, rd::<u64>(item + 0x2c));
        wr::<u32>(dst_5d8 + 0x14, rd::<u32>(item + 0x34));
        wr::<u64>(dst_5d8 + 0x18, rd::<u64>(item + 0x38));
        wr::<u32>(dst_5d8 + 0x20, rd::<u32>(item + 0x40));
        wr::<u64>(dst_5d8 + 0x24, rd::<u64>(item + 0x44));
        wr::<u32>(dst_5d8 + 0x2c, rd::<u32>(item + 0x4c));

        let rax_b8 = rd::<usize>(rbx_node + 0x08);
        let xmm2 = rd::<f32>(rax_b8 + 0x248);
        if !(xmm6 >= 0.0f32) {
            xmm6 = if 0.0f32 <= xmm2 {
                xmm2
            } else {
                rd::<f32>(exe::ptr::<f32>(0x1_4386_c220) as usize)
            };
        }
        let ecx_idx = rd::<u32>(rbx_node + 0x48) as usize;
        wr::<f32>(
            rd::<usize>(r10_mgr + 0x1f8).wrapping_add(ecx_idx * 4),
            xmm6,
        );

        let rax_item8 = rd::<usize>(item + 0x08);
        if rd::<u64>(rax_item8 + 0xc8) != 0 {
            let rax_b8_2 = rd::<usize>(rbx_node + 0x08);
            let rdx_arg = rax_b8_2.wrapping_add(0x2d8);
            let r9_arg = rax_b8_2.wrapping_add(0x2e8);
            let stack_arg5 = r9_arg.wrapping_add(0x0c);
            let xmm2_bits = rd::<u32>(rdx_arg + 0x0c);
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4230_2fe0,
                rbx_node as u64,
                rdx_arg as u64,
                xmm2_bits as u64,
                r9_arg as u64,
                stack_arg5 as u64,
                0,
                0,
                0,
                [0, 0, xmm2_bits as u128, 0],
            );
        }
    }

    let buf_rbx = exe::call_va!(0x1_4241_c0a0, ((param_2 * 0x18) as u64, 0x10u64) -> usize);
    for r11 in 0..param_2 {
        let item = param_1.wrapping_add(r11 * 0x60);
        let dst = buf_rbx.wrapping_add(r11 * 0x18);
        wr::<u32>(dst, rd::<u32>(item + 0x10));
        wr::<u32>(dst + 0x04, rd::<u32>(item + 0x14));
        wr::<u64>(dst + 0x08, rd::<u64>(rd::<usize>(item + 0x08) + 0xc0));
        wr::<u64>(
            dst + 0x10,
            rd::<u64>(rd::<usize>(rdi.wrapping_add(r11 * 8)) + 0x20),
        );
    }
    exe::call_va!(0x1_4230_76a0, (buf_rbx, param_2 as u64, total_rcx));
    exe::call_va!(0x1_4241_c190, (buf_rbx));

    for i in 0..param_2 {
        let slot = param_1.wrapping_add(i * 0x60 + 0x08);
        let rdx_obj = rd::<usize>(slot);
        if rdx_obj != 0 {
            let old = crate::simd_ops::atomic_xadd_u32((rdx_obj + 0x24) as u64, u32::MAX);
            if old == 1 {
                let rcx_28 = rd::<usize>(rdx_obj + 0x28);
                if rcx_28 != 0 {
                    exe::call_va!(0x1_4241_d6f0, (rcx_28, rdx_obj));
                }
            }
        }
        wr::<u64>(slot, 0);
    }

    let rdi_final = rd::<usize>(vec_50_ptr);
    if rdi_final != 0 {
        exe::call_va!(0x1_4241_c190, (rdi_final));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14226bb40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;
    static mut CREATED_SLOT_PTR: usize = 0;
    static mut HUD_DS_SLOT_PTR: usize = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_8dd0(_rcx: usize, rdx: u32) {
        unsafe { rec(0x8dd0 ^ (rdx as u64)) };
    }
    unsafe extern "win64" fn noop0() {
        unsafe { rec(0x1000) };
    }
    unsafe extern "win64" fn noop1(_a: usize) {
        unsafe { rec(0x1001) };
    }
    unsafe extern "win64" fn mock_f460(slot: usize) -> usize {
        wr::<usize>(slot, 0);
        slot
    }
    unsafe extern "win64" fn mock_81e0(
        _mgr: usize,
        desc: usize,
        _name: usize,
        out_slot: usize,
    ) -> usize {
        unsafe {
            rec(
                0x81e0
                    ^ (rd::<u32>(desc) as u64)
                    ^ ((rd::<u32>(desc + 0x04) as u64) << 16)
                    ^ ((rd::<u8>(desc + 0x10) as u64) << 32),
            );
            let wrapper = CREATED_SLOT_PTR;
            wr::<usize>(out_slot, wrapper);
            out_slot
        }
    }
    unsafe extern "win64" fn mock_edb0(_rcx: usize, target: usize, col_ptr: usize) {
        unsafe {
            rec(
                0xedb0
                    ^ (rd::<u32>(target) as u64)
                    ^ (rd::<u32>(col_ptr) as u64)
                    ^ ((rd::<u32>(col_ptr + 0x04) as u64) << 16)
                    ^ ((rd::<u32>(col_ptr + 0x08) as u64) << 32)
                    ^ (rd::<u32>(col_ptr + 0x0c) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_1d50(_rcx: usize, _rdx: usize, r8: u8) {
        unsafe { rec(0x1d50 ^ (r8 as u64)) };
    }
    unsafe extern "win64" fn mock_7c60(_mgr: usize, desc: usize, _name: usize) -> usize {
        unsafe {
            rec(
                0x7c60
                    ^ (rd::<u32>(desc) as u64)
                    ^ ((rd::<u32>(desc + 0x04) as u64) << 16)
                    ^ ((rd::<u8>(desc + 0x08) as u64) << 32)
                    ^ (rd::<u64>(desc + 0x0c) << 40),
            );
            HUD_DS_SLOT_PTR
        }
    }
    unsafe extern "win64" fn mock_49b0(rt: usize, flag: u32) {
        unsafe {
            rec(
                0x49b0
                    ^ (rd::<u32>(rt) as u64)
                    ^ (flag as u64)
                    ^ ((rd::<usize>(rt + 0xc8) != 0) as u64)
                    ^ (((rd::<usize>(rt + 0x108) != 0) as u64) << 4),
            );
        }
    }
    unsafe extern "win64" fn mock_f8c0(tls_ctx: usize, vp_ptr: usize) {
        unsafe {
            rec(
                0xf8c0
                    ^ ((vp_ptr == tls_ctx + 0xa84) as u64)
                    ^ (rd::<u32>(tls_ctx + 0xa8c) as u64)
                    ^ ((rd::<u32>(tls_ctx + 0xa90) as u64) << 16)
                    ^ (rd::<u32>(tls_ctx + 0xa94) as u64)
                    ^ (rd::<u32>(tls_ctx + 0xa98) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_9b60(_rcx: usize, stage: u32) {
        unsafe { rec(0x9b60 ^ (stage as u64)) };
    }
    unsafe extern "win64" fn mock_58b0(mat: usize, verts: usize) {
        unsafe {
            rec(
                0x58b0
                    ^ (rd::<u32>(mat) as u64)
                    ^ ((rd::<u32>(mat + 0x14) as u64) << 8)
                    ^ ((rd::<u32>(mat + 0x30) as u64) << 16)
                    ^ ((rd::<u32>(mat + 0x34) as u64) << 24)
                    ^ (rd::<u64>(verts + 0x08) ^ rd::<u64>(verts + 0x18)),
            );
        }
    }
    unsafe extern "win64" fn mock_ea70(out_id: usize, sv: usize) {
        wr::<i32>(out_id, 0);
        unsafe { rec(0xea70 ^ rd::<u64>(sv + 0x08)) };
    }
    unsafe extern "win64" fn mock_a0c(guard: usize) {
        wr::<i32>(guard, 1);
    }
    unsafe extern "win64" fn mock_a900(id: u32, rdx: u32, r8: u64, ptr: usize) {
        unsafe {
            rec(
                0xa900
                    ^ (id as u64)
                    ^ (rdx as u64)
                    ^ r8
                    ^ (rd::<u32>(ptr) as u64)
                    ^ ((rd::<u32>(ptr + 0x04) as u64) << 16),
            );
        }
    }
    unsafe extern "win64" fn mock_70b0(mgr: u64, sv: usize) -> u64 {
        unsafe { rec(0x70b0 ^ mgr ^ rd::<u64>(sv + 0x08)) };
        0x5555_0001
    }
    unsafe extern "win64" fn mock_8a00(rfx: u64, sv: usize) -> u64 {
        unsafe { rec(0x8a00 ^ rfx ^ rd::<u64>(sv + 0x08)) };
        0x6666_0002
    }
    unsafe extern "win64" fn mock_e860(tech: u64, flags: u32) -> u64 {
        unsafe { rec(0xe860 ^ tech ^ (flags as u64)) };
        0x7777_0003
    }
    unsafe extern "win64" fn mock_8080(mgr: u64, old_430: u64) {
        unsafe { rec(0x8080 ^ mgr ^ (old_430 != 0) as u64) };
    }
    unsafe extern "win64" fn mock_7af0(mgr: u64, hud_ds: usize) {
        unsafe { rec(0x7af0 ^ mgr ^ (rd::<u64>(hud_ds) & 0xffff)) };
    }
    unsafe extern "win64" fn vfn0(_this: usize, rdx: usize) {
        unsafe { rec(0xc000 ^ ((rdx != 0) as u64)) };
    }
    unsafe extern "win64" fn vfn1(_this: usize, _rdx: usize) {
        unsafe { rec(0xc001) };
    }
    unsafe extern "win64" fn vfn2(_this: usize, w: u32, h: u32, f: u8) {
        unsafe { rec(0xc002 ^ (w as u64) ^ ((h as u64) << 16) ^ ((f as u64) << 32)) };
    }
    unsafe extern "win64" fn vfn3(_this: usize, _r14: usize, ds: u64, f: u8) {
        unsafe { rec(0xc003 ^ ds ^ ((f as u64) << 32)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4226_8dd0, mock_8dd0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4183_49c0, noop0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412c_f460, mock_f460 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4225_81e0, mock_81e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4206_edb0, mock_edb0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_421f_1d50, mock_1d50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4225_7c60, mock_7c60 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4205_49b0, mock_49b0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4206_f8c0, mock_f8c0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4229_d440, noop1 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4223_9b60, mock_9b60 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_422e_6dc0, noop0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_422e_58b0, mock_58b0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4183_4a78, noop1 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4183_4d18, noop1 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_a0c as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4239_70b0, mock_70b0 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_4239_8a00, mock_8a00 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_4239_7700, noop1 as *const ())?;
    let _m22 = crate::difftest::Mock::new(0x1_4220_e860, mock_e860 as *const ())?;
    let _m23 = crate::difftest::Mock::new(0x1_4224_ded0, noop1 as *const ())?;
    let _m24 = crate::difftest::Mock::new(0x1_422e_6b00, noop0 as *const ())?;
    let _m25 = crate::difftest::Mock::new(0x1_4225_8080, mock_8080 as *const ())?;
    let _m26 = crate::difftest::Mock::new(0x1_4225_7af0, mock_7af0 as *const ())?;

    let tls_arr = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_arr);
    let mut tls_buf = vec![0u8; 0x47400];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    wr::<usize>(tls_arr, tls0);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a2_8cc6) as *mut u8, 1);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4386_a630) as *mut u8, 1);
    let s3 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_3c28) as *mut u8, 8);
    let s4 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff18) as *mut u8, 8);
    let s5 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb8) as *mut u8, 8);
    let s6 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d82f) as *mut u8, 1);
    let s7 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4e94) as *mut u8, 4);
    let s8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_2b48) as *mut u8, 0x20);
    let s9 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_5390) as *mut u8, 4);
    let s10 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4a18) as *mut u8, 0x20);
    let s11 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_30e0) as *mut u8, 4);
    let s12 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4690) as *mut u8, 0x20);
    let s13 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4930) as *mut u8, 4);
    let s14 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_3a30) as *mut u8, 0x20);
    let s15 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c820) as *mut u8, 8);
    let s16 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c8c8) as *mut u8, 8);

    let run = |f: F, mode: u32| -> (u64, [u8; 0x18], u8, u8, bool) {
        core::ptr::write_bytes(tls0 as *mut u8, 0, 0x47400);
        wr::<u8>(tls0 + 0x70, 1);
        wr::<i32>(tls0 + 0x74, 0);

        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc6) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_a630) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_43a3_3c28) as usize, 1920);
        wr::<u32>(exe::ptr::<u32>(0x1_43a3_3c2c) as usize, 1080);
        wr::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize, 0x1234_0000);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d82f) as usize, 1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_4e94) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_5390) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_30e0) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_4930) as usize, -1);
        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c820) as usize, 0);
        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c8c8) as usize, 0);

        let mut fcb8_buf = [0u8; 0xa0];
        wr::<u64>(fcb8_buf.as_mut_ptr() as usize + 0x88, 0xaaaa_8888);
        wr::<u64>(fcb8_buf.as_mut_ptr() as usize + 0x90, 0xbbbb_9999);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fcb8) as usize,
            fcb8_buf.as_ptr() as usize,
        );

        let mut r14_buf = [0u8; 0xa0];
        let r14p = r14_buf.as_mut_ptr() as usize;
        wr::<u32>(r14p, 1920);
        wr::<u32>(r14p + 0x04, 1080);
        wr::<u8>(r14p + 0x18, 7);
        wr::<u64>(r14p + 0x78, 0x1111_7878);
        wr::<u64>(r14p + 0x90, 0x2222_9090);
        let wrapper_ptr: usize = r14p;

        let hud_ds_slot: u64 = 0xdead_cefa;
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            CREATED_SLOT_PTR = (&wrapper_ptr as *const usize) as usize;
            HUD_DS_SLOT_PTR = (&hud_ds_slot as *const u64) as usize;
        }

        let mut p48_buf = [0u8; 0x60];
        wr::<u8>(p48_buf.as_mut_ptr() as usize + 0x4c, 1);

        let vtbl: [usize; 4] = [
            vfn0 as *const () as usize,
            vfn1 as *const () as usize,
            vfn2 as *const () as usize,
            vfn3 as *const () as usize,
        ];
        let p5a8_obj: usize = vtbl.as_ptr() as usize;

        let mut p1 = vec![0u8; 0x600];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x48, p48_buf.as_ptr() as usize);
        wr::<usize>(p1p + 0x58, 0x5858);
        wr::<usize>(p1p + 0xa8, 0xa8a8);
        wr::<usize>(p1p + 0x1c0, 0x1c00);
        wr::<u8>(p1p + 0x428, if mode == 0 { 1 } else { 0 });
        wr::<u8>(p1p + 0x429, if mode == 0 { 1 } else { 0 });
        wr::<usize>(
            p1p + 0x430,
            if mode == 1 {
                (&wrapper_ptr as *const usize) as usize
            } else {
                0
            },
        );
        wr::<usize>(
            p1p + 0x448,
            if mode == 1 {
                (&wrapper_ptr as *const usize) as usize
            } else {
                0
            },
        );
        wr::<u8>(p1p + 0x488, 9);
        wr::<f32>(p1p + 0x48c, 0.25);
        wr::<f32>(p1p + 0x490, 0.5);
        wr::<f32>(p1p + 0x494, 0.75);
        wr::<f32>(p1p + 0x498, 1.0);
        wr::<u8>(p1p + 0x49d, 0);
        wr::<f32>(p1p + 0x4a0, 2.0);
        wr::<f32>(p1p + 0x4a4, 1.5);
        wr::<usize>(p1p + 0x5a8, (&p5a8_obj as *const usize) as usize);

        f(p1p);

        let tls_viewport = rd::<[u8; 0x18]>(tls0 + 0x467e0 + 0xa84);
        (
            unsafe { TRACE_HASH },
            tls_viewport,
            rd::<u8>(p1p + 0x428),
            rd::<u8>(p1p + 0x460),
            rd::<usize>(p1p + 0x430) != 0,
        )
    };

    for mode in 0..2 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14226bb40 mode={mode}");
    }

    s16.restore();
    s15.restore();
    s14.restore();
    s13.restore();
    s12.restore();
    s11.restore();
    s10.restore();
    s9.restore();
    s8.restore();
    s7.restore();
    s6.restore();
    s5.restore();
    s4.restore();
    s3.restore();
    s2.restore();
    s1.restore();
    wr::<usize>(tls_arr, saved_tls0);

    Ok(2)
}

pub unsafe fn diff_unknown_142306c20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_BUF_PTR: usize = 0;
    static mut ALLOC_BUF_PTR: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_resize(vec_hdr: usize, count: u64) {
        unsafe {
            rec(0xf200 ^ count);
            wr::<usize>(vec_hdr, VEC_BUF_PTR);
            wr::<u32>(vec_hdr + 0x08, count as u32);
            wr::<u32>(vec_hdr + 0x0c, count as u32);
        }
    }
    unsafe extern "win64" fn mock_d5b0(_mgr: usize, span: usize) {
        let cnt = rd::<u64>(span) as usize;
        let ptr = rd::<usize>(span + 0x08);
        unsafe { rec(0xd5b0 ^ (cnt as u64)) };
        for i in 0..cnt {
            let node = rd::<usize>(ptr + i * 8);
            wr::<u32>(node + 0x48, i as u32);
        }
    }
    unsafe extern "win64" fn mock_d6f0(ctx: usize, _obj: usize) {
        unsafe { rec(0xd6f0 ^ (ctx as u64)) };
    }
    unsafe extern "win64" fn mock_0fe0(_rcx: usize, tmp: usize, idx: u32) -> usize {
        unsafe { rec(0x0fe0 ^ (idx as u64)) };
        wr::<u64>(tmp, 0x1122_3344_5566_7788);
        wr::<u32>(tmp + 0x08, 0x99aa_bbcc);
        tmp
    }
    unsafe extern "win64" fn mock_40c0(node: usize) {
        unsafe { rec(0x40c0 ^ (rd::<u32>(node + 0x48) as u64)) };
    }
    unsafe extern "win64" fn mock_5b70(node: usize) {
        let idx = rd::<u32>(node + 0x48) as u64;
        wr::<u64>(node + 0x20, 0x8000 + idx);
        unsafe { rec(0x5b70 ^ idx) };
    }
    unsafe extern "win64" fn mock_2fe0(
        node: usize,
        rdx: usize,
        _r8: u64,
        r9: usize,
        arg5: usize,
    ) {
        unsafe {
            rec(
                0x2fe0
                    ^ (rd::<u32>(node + 0x48) as u64)
                    ^ (rd::<u32>(rdx + 0x0c) as u64)
                    ^ (arg5.wrapping_sub(r9) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_alloc(sz: u64, align: u64) -> usize {
        unsafe {
            rec(0xc0a0 ^ sz ^ align);
            ALLOC_BUF_PTR
        }
    }
    unsafe extern "win64" fn mock_76a0(buf: usize, cnt: u64, total: u64) {
        unsafe {
            rec(0x76a0 ^ cnt ^ total);
            for i in 0..(cnt as usize) {
                let p = buf + i * 0x18;
                rec(
                    (rd::<u32>(p) as u64)
                        ^ ((rd::<u32>(p + 0x04) as u64) << 16)
                        ^ rd::<u64>(p + 0x10),
                );
            }
        }
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        unsafe { rec(0xc190) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_410e_f200, mock_resize as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_d5b0, mock_d5b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4223_0fe0, mock_0fe0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4230_40c0, mock_40c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4230_5b70, mock_5b70 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4230_2fe0, mock_2fe0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4230_76a0, mock_76a0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let s_60a8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43be_60a8) as *mut u8, 16);
    let s_ff48 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff48) as *mut u8, 8);
    let s_fc58 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fc58) as *mut u8, 8);

    let run = |f: F| -> (u64, [u8; 0x60], [u8; 0x60], [u64; 4], [f32; 2], [u8; 8]) {
        let mut nodes = vec![0u8; 2 * 0x98];
        let np = nodes.as_mut_ptr() as usize;
        wr::<u32>(np + 0x4c, 0xaa00_0000);
        wr::<u64>(np + 0x20, 0x1111);
        wr::<u32>(np + 0x98 + 0x4c, 0xbb00_0000);
        wr::<u64>(np + 0x98 + 0x20, 0x2222);

        wr::<usize>(exe::ptr::<usize>(0x1_43be_60a8) as usize, np);

        let mut vec_buf = [0usize; 4];
        let mut alloc_buf = [0u8; 0x60];
        unsafe {
            VEC_BUF_PTR = vec_buf.as_mut_ptr() as usize;
            ALLOC_BUF_PTR = alloc_buf.as_mut_ptr() as usize;
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let mut arr_28 = [0u8; 0x20];
        let mut arr_38 = [0u64; 1];
        let mut arr_180 = [0u64; 1];
        let mut arr_198 = [0u32; 2];
        let mut arr_1a8 = [0u8; 2];
        let mut arr_1b8 = [0u8; 2];
        let mut arr_1f8 = [0.0f32; 2];
        let mut arr_218 = [0u8; 2];
        let mut arr_5c8 = [0u8; 0x60];
        let mut arr_5d8 = [0u8; 0x60];
        let mut arr_638 = [0u64; 1];
        let mut arr_650 = [0u64; 1];

        let mut mgr = vec![0u8; 0x680];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x28, arr_28.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x38, arr_38.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x180, arr_180.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x198, arr_198.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x1a8, arr_1a8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x1b8, arr_1b8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x1f8, arr_1f8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x218, arr_218.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x5c8, arr_5c8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x5d8, arr_5d8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x638, arr_638.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x650, arr_650.as_mut_ptr() as usize);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize, mp);

        let mut fc58_buf = [0u8; 0xc0];
        wr::<u8>(fc58_buf.as_mut_ptr() as usize + 0xac, 1);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc58) as usize,
            fc58_buf.as_ptr() as usize,
        );

        let mut r15_buf = vec![0u8; 0x320];
        let r15p = r15_buf.as_mut_ptr() as usize;
        wr::<i32>(r15p + 0x138, 7);
        wr::<i32>(r15p + 0x1c8, 3);
        wr::<f32>(r15p + 0x248, 2.5);
        wr::<u32>(r15p + 0x26c, 0x4242_1337);
        wr::<u8>(r15p + 0x271, 0x5a);
        wr::<u8>(r15p + 0x272, 1);
        wr::<f32>(r15p + 0x2e4, 1.25);

        let mut obj0 = [0u8; 0xd0];
        let o0p = obj0.as_mut_ptr() as usize;
        wr::<u32>(o0p + 0x24, 0);
        wr::<usize>(o0p + 0x28, 0x7001);
        wr::<usize>(o0p + 0xc0, r15p);
        wr::<u64>(o0p + 0xc8, 1);

        let mut obj1 = [0u8; 0xd0];
        let o1p = obj1.as_mut_ptr() as usize;
        wr::<u32>(o1p + 0x24, 2);
        wr::<usize>(o1p + 0x28, 0x7002);
        wr::<usize>(o1p + 0xc0, r15p);
        wr::<u64>(o1p + 0xc8, 0);

        let mut items = [0u8; 0xc0];
        let ip = items.as_mut_ptr() as usize;
        // Item 0
        wr::<u32>(ip, 0);
        wr::<usize>(ip + 0x08, o0p);
        wr::<u32>(ip + 0x10, 0x0012_3400);
        wr::<u32>(ip + 0x14, 0x33);
        for k in 0..12usize {
            wr::<u32>(ip + 0x20 + k * 4, 0x100 + k as u32);
        }
        wr::<f32>(ip + 0x50, -1.0);

        // Item 1
        let ip1 = ip + 0x60;
        wr::<u32>(ip1, 1);
        wr::<usize>(ip1 + 0x08, o1p);
        wr::<u32>(ip1 + 0x10, 0x0055_0113);
        wr::<u32>(ip1 + 0x14, 0x44);
        for k in 0..12usize {
            wr::<u32>(ip1 + 0x20 + k * 4, 0x200 + k as u32);
        }
        wr::<f32>(ip1 + 0x50, 3.5);

        f(ip, 0);
        f(ip, 2);

        let masks = [arr_38[0], arr_180[0], arr_638[0], arr_650[0]];
        let bytes = [
            arr_28[2],
            arr_28[14],
            arr_1a8[0],
            arr_1a8[1],
            arr_1b8[0],
            arr_1b8[1],
            arr_218[0],
            arr_218[1],
        ];
        (
            unsafe { TRACE_HASH },
            arr_5c8,
            arr_5d8,
            masks,
            arr_1f8,
            bytes,
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    s_fc58.restore();
    s_ff48.restore();
    s_60a8.restore();

    crate::diff_eq!(o, n, "142306c20 out");
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4226_bb40, "unknown_14226bb40", rend::cauldron_hot_n1::unknown_14226bb40 as *const (), Throws::Caught, rend::cauldron_hot_n1::diff_unknown_14226bb40),
// Replacement::verified_throws(0x1_4230_6c20, "unknown_142306c20", rend::cauldron_hot_n1::unknown_142306c20 as *const (), Throws::Caught, rend::cauldron_hot_n1::diff_unknown_142306c20),
