//! Decompiled `rend` hot functions shard Z8B (`0x14222ef60..0x14226a820`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `std_14222ef60`
/// Original: `0x14222ef60` (388 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn std_14222ef60(param_1: usize) {
    let mut rdx = rd::<usize>(param_1 + 0x90);
    if rd::<u8>(rdx + 0x270) == 0 {
        return;
    }

    let rbx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed8) as usize);
    let rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let mut r9d: i32 = 0;

    loop {
        let b272 = rd::<u8>(rdx + 0x272) as i32;
        if b272 != 0 {
            let ecx = b272 - 1;
            let idx = if ecx < r9d { ecx } else { r9d };
            let cnt = rd::<i32>((rdx as isize + (idx as isize) * 4 + 0x1c8) as usize);
            if cnt > 0 {
                let ecx_98 = rd::<u32>(param_1 + 0x98) as usize;
                let rax_110 = rd::<usize>(rbx + 0x110);
                let edx_slot = rd::<u32>(rax_110 + ecx_98 * 4);
                if edx_slot != 0xffff_ffff {
                    let rax_140 = rd::<usize>(rdi + 0x140);
                    let r10 = rax_140.wrapping_add((edx_slot as usize) * 0x18);
                    let mut node_rdx = rd::<usize>(rsi);
                    let mut node_rax = rd::<usize>(node_rdx + 0x08);
                    if rd::<u8>(node_rax + 0x19) == 0 {
                        let key = rd::<u64>(r10 + 0x08);
                        loop {
                            if rd::<u64>(node_rax + 0x20) < key {
                                node_rax = rd::<usize>(node_rax + 0x10);
                            } else {
                                node_rdx = node_rax;
                                node_rax = rd::<usize>(node_rax);
                            }
                            if rd::<u8>(node_rax + 0x19) != 0 {
                                break;
                            }
                        }
                    }
                    if rd::<u8>(node_rdx + 0x19) != 0
                        || rd::<u64>(r10 + 0x08) < rd::<u64>(node_rdx + 0x20)
                    {
                        exe::call_iat!(
                            0x1_4264_5890,
                            (exe::ptr::<u8>(0x1_4313_f720) as usize)
                        );
                        return;
                    }
                    let r10d = rd::<u32>(r10 + 0x10);
                    let elem_ptr =
                        rd::<usize>(node_rdx + 0x5a0).wrapping_add((r10d as usize) << 6);
                    let r8_obj = rd::<usize>(elem_ptr);
                    let b272_inner = rd::<u8>(r8_obj + 0x272) as i32;
                    if b272_inner != 0 {
                        let ecx_inner = b272_inner - 1;
                        let idx_inner = if ecx_inner < r9d { ecx_inner } else { r9d };
                        let cnt_inner = rd::<i32>(
                            (r8_obj as isize + (idx_inner as isize) * 4 + 0x1c8) as usize,
                        );
                        if cnt_inner > 0 {
                            let rcx_bits = rd::<usize>(node_rdx + 0x570);
                            let bit_pos = r10d.wrapping_add(r9d as u32);
                            let word_idx = (bit_pos >> 6) as usize;
                            let bit_idx = bit_pos & 0x3f;
                            let word_addr = rcx_bits.wrapping_add(word_idx * 8);
                            let word = rd::<u64>(word_addr);
                            wr::<u64>(word_addr, word | (1u64 << bit_idx));
                        }
                    }
                }
            }
        }
        rdx = rd::<usize>(param_1 + 0x90);
        r9d += 1;
        let limit = rd::<u8>(rdx + 0x270) as i32;
        if r9d >= limit {
            break;
        }
    }
}

/// `std_14222f0f0`
/// Original: `0x14222f0f0` (387 bytes, `Throws::No`)
pub unsafe extern "win64" fn std_14222f0f0(param_1: usize) {
    let r10 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff70) as usize);
    for off in [0x40usize, 0x48, 0x50, 0x58, 0x60, 0x68, 0x70, 0x78] {
        let ptr = rd::<usize>(param_1 + off);
        if ptr != 0 {
            let r8d = rd::<u32>(ptr + 0x58);
            let rdx = rd::<usize>(r10 + 0x78);
            let word_idx = (r8d >> 6) as usize;
            let bit_idx = r8d & 0x3f;
            let word_addr = rdx.wrapping_add(word_idx * 8);
            let word = rd::<u64>(word_addr);
            wr::<u64>(word_addr, word | (1u64 << bit_idx));
        }
    }
}

/// Helper for binding a texture source slot in `unknown_14226a820`.
#[inline(always)]
unsafe fn bind_draw_source_slot(
    slot_id_va: u64,
    out_view_va: u64,
    out_tex_va: u64,
    rbx_tex: usize,
) {
    let r14_id = rd::<i32>(exe::ptr::<i32>(slot_id_va) as usize) as isize;
    let tbl_8590 = exe::ptr::<u8>(0x1_4385_8590) as isize;
    let tbl_37b0 = exe::ptr::<u8>(0x1_43a6_37b0) as isize;
    let tbl_37b8 = exe::ptr::<u8>(0x1_43a6_37b8) as isize;
    let tbl_37bc = exe::ptr::<u8>(0x1_43a6_37bc) as isize;

    let esi_kind = rd::<i32>((tbl_8590 + r14_id * 4) as usize);
    let rdi_fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);

    let view0 = if esi_kind == 0 {
        if rbx_tex != 0 {
            rd::<u64>(rbx_tex + 0x78)
        } else {
            rd::<u64>(rdi_fcb8 + 0x88)
        }
    } else if esi_kind == 5 {
        if rbx_tex != 0 {
            rd::<u64>(rbx_tex + 0x90)
        } else {
            rd::<u64>(rdi_fcb8 + 0x90)
        }
    } else {
        rd::<u64>(rdi_fcb8 + 0x88)
    };

    let pair: [u64; 2] = [view0, rbx_tex as u64];
    let r8_len = rd::<i32>((tbl_37b8 + r14_id * 16) as usize) as isize;
    let rcx_off = rd::<i32>((tbl_37b0 + r14_id * 16) as usize) as isize;
    if rcx_off >= 0 {
        let r12_tls = rd::<usize>(exe::tls_pointer());
        let dst = (r12_tls as isize + rcx_off + 0x63d0) as usize;
        core::ptr::copy_nonoverlapping(pair.as_ptr() as *const u8, dst as *mut u8, r8_len as usize);
        let idx_bc = rd::<i32>((tbl_37bc + r14_id * 16) as usize) as isize;
        if idx_bc != -1 {
            let counter_addr = (r12_tls + 0x463e0).wrapping_add((idx_bc as usize) * 4);
            let next = rd::<i32>(counter_addr).wrapping_add(1);
            wr::<i32>(counter_addr, if next == -1 { 0 } else { next });
        }
    }

    wr::<usize>(exe::ptr::<usize>(out_tex_va) as usize, rbx_tex);
    let view1 = if esi_kind == 0 {
        if rbx_tex != 0 {
            rd::<u64>(rbx_tex + 0x78)
        } else {
            rd::<u64>(rdi_fcb8 + 0x88)
        }
    } else if esi_kind == 5 {
        if rbx_tex != 0 {
            rd::<u64>(rbx_tex + 0x90)
        } else {
            rd::<u64>(rdi_fcb8 + 0x90)
        }
    } else {
        rd::<u64>(rdi_fcb8 + 0x88)
    };
    wr::<u64>(exe::ptr::<u64>(out_view_va) as usize, view1);
}

/// `unknown_14226a820`
/// Original: `0x14226a820` (4875 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_14226a820(param_1: usize) {
    let rdi_58 = rd::<usize>(param_1 + 0x58);
    if rdi_58 != 0 {
        let rbx_lock = rdi_58 + 0x08;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rbx_lock + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (rbx_lock));
            wr::<u32>(rbx_lock + 0x08, tid);
        }
        wr::<i32>(
            rbx_lock + 0x0c,
            rd::<i32>(rbx_lock + 0x0c).wrapping_add(1),
        );

        let vec_hdr = rdi_58 + 0x18;
        let mut r14d: i32 = 0;
        if rd::<u32>(vec_hdr + 0x08) > 0 {
            loop {
                let rdx_arr = rd::<usize>(vec_hdr);
                let off = (r14d as isize as usize).wrapping_mul(0x68);
                if rd::<u8>(rdx_arr + off + 0x38) == 0 {
                    let frame_cur = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize);
                    let r8d = frame_cur.wrapping_sub(rd::<i32>(rdx_arr + off + 0x64));
                    let seed_addr = exe::ptr::<u64>(0x1_4385_8588) as usize;
                    let rcx_seed = rd::<u64>(seed_addr);
                    wr::<u64>(seed_addr, rcx_seed.wrapping_mul(0x5851_f42d_4c95_7f2d));
                    let xored = (rcx_seed >> 0x16) ^ rcx_seed;
                    let shift = ((rcx_seed >> 0x3d) + 0x16) as u32;
                    let rnd32 = (xored >> shift) as u32;
                    let thresh = (rnd32 % 0x21d).wrapping_add(0x3c) as i32;
                    if r8d > thresh {
                        let base = rd::<usize>(vec_hdr);
                        let rsi_ptr = rd::<usize>(base + off);
                        if rsi_ptr != 0 {
                            exe::call_va!(0x1_4204_f860, (rsi_ptr));
                            exe::call_va!(0x1_4244_2de0, (rsi_ptr));
                        }
                        let base2 = rd::<usize>(vec_hdr);
                        wr::<usize>(base2 + off, 0);
                    }
                }
                r14d = r14d.wrapping_add(1);
                let count = rd::<u32>(vec_hdr + 0x08) as u64;
                if (r14d as i64 as u64) >= count {
                    break;
                }
            }
        }
        exe::call_va!(0x1_412f_6b70, (vec_hdr, 0u32));
        let new_depth = rd::<i32>(rbx_lock + 0x0c).wrapping_sub(1);
        wr::<i32>(rbx_lock + 0x0c, new_depth);
        if new_depth == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rbx_lock));
        }
    }

    let rdx_200 = rd::<usize>(param_1 + 0x200);
    if rdx_200 != 0 {
        let rcx_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        exe::call_va!(0x1_4248_5780, (rcx_mgr, rdx_200 + 0x2a8));
    }

    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    let tls_467e0 = tls0.wrapping_add(0x467e0);

    let rbx_160 = rd::<usize>(param_1 + 0x160);
    if rbx_160 != 0 {
        let rsi_ff18 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize);
        let mut edi: i32 = 0;
        let mut rdx_arr = rd::<usize>(rbx_160 + 0x40);
        if rd::<u32>(rbx_160 + 0x48) > 0 {
            loop {
                let elem = rd::<u64>(rdx_arr.wrapping_add((edi as isize as usize) * 8));
                exe::call_va!(0x1_4225_8080, (rsi_ff18, elem));
                edi = edi.wrapping_add(1);
                let cnt = rd::<u32>(rbx_160 + 0x48) as u64;
                rdx_arr = rd::<usize>(rbx_160 + 0x40);
                if (edi as i64 as u64) >= cnt {
                    break;
                }
            }
        }
        wr::<u32>(rbx_160 + 0x48, 0);
    }

    let mut rbx_a8 = rd::<usize>(param_1 + 0xa8).wrapping_add(0xb4);
    for _ in 0..0x13usize {
        if rd::<u8>(rbx_a8 - 0xb4) != 0 {
            let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
            if rd::<u32>(rbx_a8 - 0x04) != tid {
                exe::call_iat!(0x1_4264_55a8, (rbx_a8 - 0x0c));
                wr::<u32>(rbx_a8 - 0x04, tid);
            }
            let cnt = rd::<u32>(rbx_a8);
            wr::<u32>(rbx_a8 - 0x34, 0);
            wr::<u32>(rbx_a8 - 0x44, 0);
            wr::<u32>(rbx_a8 - 0x24, 0);
            wr::<u32>(rbx_a8 - 0x14, 0);
            wr::<u32>(rbx_a8, cnt);
            if cnt == 0 {
                let _ = crate::simd_ops::atomic_xchg_u32((rbx_a8 - 0x04) as u64, u32::MAX);
                exe::call_iat!(0x1_4264_5598, (rbx_a8 - 0x0c));
            }
        }
        rbx_a8 = rbx_a8.wrapping_add(0xd8);
    }

    let idx_c80 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let entry_980 = (exe::ptr::<u8>(0x1_43c1_6980) as usize).wrapping_add(idx_c80 * 0x158);
    exe::call_va!(0x1_40fe_7130, (entry_980, 4u32));
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8da) as usize, 0);

    let r8_448 = rd::<usize>(param_1 + 0x448);
    let rdx_48 = rd::<usize>(param_1 + 0x48);
    let idx_38 = rd::<i32>(rdx_48 + 0x38) as isize;
    let slot_20 = rd::<usize>((rdx_48 as isize + idx_38 * 8 + 0x20) as usize);
    let sil = if r8_448 != 0 {
        r8_448 == slot_20
    } else {
        slot_20 != 0
    };

    let r9b = rd::<u8>(rdx_48 + 0x66);
    let eax_11c = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
    let cond_hdr = r9b == 0
        && if eax_11c == 0 {
            rd::<u8>(rdx_48 + 0x67) != 0
        } else {
            eax_11c != 2
        };
    let cl = cond_hdr as u8;
    let r14b = if rd::<u8>(param_1 + 0x49d) == cl {
        true
    } else {
        cond_hdr
    };

    let dil = if r9b == 0 {
        rd::<f32>(exe::ptr::<f32>(0x1_4384_1640) as usize)
            != rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize)
    } else {
        false
    };

    let bl = if !sil {
        true
    } else if rd::<u8>(rdx_48 + 0x4c) != 0 {
        r14b
    } else {
        dil
    };

    if rd::<u8>(exe::ptr::<u8>(0x1_43a2_8cc6) as usize) != 0 {
        let mut r8_dump = r8_448;
        if r8_dump == 0 {
            let idx = rd::<i32>(rdx_48 + 0x38) as isize;
            r8_dump = rd::<usize>((rdx_48 as isize + idx * 8 + 0x20) as usize);
        }
        if r8_dump != 0 {
            let r9d_mode = if rd::<u8>(rdx_48 + 0x4c) != 0 && rd::<u8>(param_1 + 0x49d) == 0 {
                4u32
            } else {
                1u32
            };
            let mut sv_exr: [u64; 2] = [exe::ptr::<u8>(0x1_433f_5d60) as u64, 0x1e];
            let arg5 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7080) as usize);
            let arg8 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6b74) as usize);
            exe::call_va!(
                0x1_422a_5780,
                (
                    sv_exr.as_mut_ptr() as usize,
                    r8_dump,
                    r8_dump,
                    r9d_mode,
                    arg5,
                    0u32,
                    0u32,
                    arg8,
                )
            );
        }
    }

    let mut r13d: u32 = 0;
    let has_src = rd::<usize>(param_1 + 0x448) != 0 || {
        let p48 = rd::<usize>(param_1 + 0x48);
        let idx = rd::<i32>(p48 + 0x38) as isize;
        rd::<usize>((p48 as isize + idx * 8 + 0x20) as usize) != 0
    };

    if has_src && bl {
        if !sil {
            let mut zero_col = [0u32; 4];
            let p48 = rd::<usize>(param_1 + 0x48);
            let idx = rd::<i32>(p48 + 0x38) as isize;
            let rdx_rt = rd::<usize>((p48 as isize + idx * 8 + 0x08) as usize);
            exe::call_va!(
                0x1_4206_edb0,
                (p48, rdx_rt, zero_col.as_mut_ptr() as usize)
            );
        }

        let mut rbx_src = rd::<usize>(param_1 + 0x448);
        if rbx_src == 0 {
            let p48 = rd::<usize>(param_1 + 0x48);
            let idx = rd::<i32>(p48 + 0x38) as isize;
            rbx_src = rd::<usize>((p48 as isize + idx * 8 + 0x20) as usize);
        }
        let r12_130 = rd::<usize>(param_1 + 0x130);
        let p48 = rd::<usize>(param_1 + 0x48);
        let idx = rd::<i32>(p48 + 0x38) as isize;
        let rdx_rt = rd::<usize>((p48 as isize + idx * 8 + 0x08) as usize);

        let mut rt_desc = [0u8; 0x118];
        let rt_ptr = rt_desc.as_mut_ptr() as usize;
        wr::<u32>(rt_ptr, 1);
        wr::<usize>(rt_ptr + 0xc8, rdx_rt);
        exe::call_va!(0x1_4205_49b0, (rt_ptr, 0u32));

        exe::call_va!(
            0x1_4206_f420,
            (
                tls_467e0,
                rd::<u32>(exe::ptr::<u32>(0x1_43a3_39e8) as usize),
                rd::<u32>(exe::ptr::<u32>(0x1_43a3_39ec) as usize),
                rd::<u32>(exe::ptr::<u32>(0x1_43a3_3c28) as usize),
                rd::<u32>(exe::ptr::<u32>(0x1_43a3_3c2c) as usize),
                rd::<u32>(exe::ptr::<u32>(0x1_43a0_0154) as usize),
                rd::<u32>(exe::ptr::<u32>(0x1_4384_1708) as usize),
            )
        );

        let is_hdr_4c = rd::<u8>(rd::<usize>(param_1 + 0x48) + 0x4c) != 0;
        let skip_to_b20 = if is_hdr_4c {
            !r14b && r12_130 != 0
        } else {
            !dil && r12_130 != 0
        };

        if skip_to_b20 {
            if !sil {
                let rdx_tex = rd::<usize>(rbx_src);
                let w = rd::<u32>(rdx_tex);
                let h = rd::<u32>(rdx_tex + 0x04);
                let rcx_130 = rd::<usize>(param_1 + 0x130);
                exe::call_va!(0x1_4239_8b20, (rcx_130, rdx_tex, w, 0u32, w, h));
            }
        } else {
            let (shader_rcx, mode_408) = if is_hdr_4c {
                exe::call_va!(0x1_422e_6dc0, ());
                let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                let g_440c = exe::ptr::<i32>(0x1_43cf_440c) as usize;
                if rd::<i32>(g_440c) > tls_epoch {
                    exe::call_va!(0x1_4183_4a78, (g_440c));
                    if rd::<i32>(g_440c) == -1 {
                        let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f280) as u64, 0x0d];
                        exe::call_va!(
                            0x1_4220_ea70,
                            (
                                exe::ptr::<u8>(0x1_43cf_3b58) as usize,
                                sv.as_mut_ptr() as usize,
                            )
                        );
                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_3b5c) as usize, 9);
                        wr::<usize>(
                            exe::ptr::<usize>(0x1_43cf_3b50) as usize,
                            exe::ptr::<u8>(0x1_43cf_3b60) as usize,
                        );
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_3b60) as usize, 0);
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_3b68) as usize, 0);
                        exe::call_va!(
                            0x1_4183_4d18,
                            (exe::ptr::<u8>(0x1_4262_ee20) as usize)
                        );
                        exe::call_va!(0x1_4183_4a0c, (g_440c));
                    }
                }

                let rbx_tex = if rbx_src != 0 { rd::<usize>(rbx_src) } else { 0 };
                bind_draw_source_slot(
                    0x1_43cf_3b58,
                    0x1_43cf_3b60,
                    0x1_43cf_3b68,
                    rbx_tex,
                );

                let rcx_48 = rd::<usize>(param_1 + 0x48);
                if rd::<u8>(rcx_48 + 0x66) == 0 {
                    let eax_m = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
                    let c_m = if eax_m == 0 {
                        rd::<u8>(rcx_48 + 0x67) != 0
                    } else {
                        eax_m != 2
                    };
                    if c_m && rd::<u8>(param_1 + 0x49d) != 0 {
                        r13d = 0x100;
                    }
                    if c_m {
                        r13d |= 0x200;
                    }
                }

                let g_838 = exe::ptr::<u64>(0x1_43c8_c838) as usize;
                let mut sh = rd::<u64>(g_838);
                if sh == 0 {
                    let mut sv_rfx: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                    let rfx = exe::call_va!(
                        0x1_4239_70b0,
                        (
                            rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                            sv_rfx.as_mut_ptr() as usize,
                        ) -> u64
                    );
                    let mut sv_tech: [u64; 2] = [exe::ptr::<u8>(0x1_433f_4648) as u64, 0x1c];
                    let tech = exe::call_va!(
                        0x1_4239_8a00,
                        (rfx, sv_tech.as_mut_ptr() as usize) -> u64
                    );
                    wr::<u64>(g_838, tech);
                    exe::call_va!(0x1_4239_7700, (g_838));
                    sh = rd::<u64>(g_838);
                }
                (sh, 1u32)
            } else {
                exe::call_va!(0x1_422e_6dc0, ());
                let rcx_48 = rd::<usize>(param_1 + 0x48);
                if rd::<u8>(rcx_48 + 0x66) == 0 {
                    r13d = 0x400;
                    if rd::<f32>(exe::ptr::<f32>(0x1_4384_1640) as usize) > 0.0 {
                        r13d = 0x4400;
                        let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                        let g_2c70 = exe::ptr::<i32>(0x1_43cf_2c70) as usize;
                        if rd::<i32>(g_2c70) > tls_epoch {
                            exe::call_va!(0x1_4183_4a78, (g_2c70));
                            if rd::<i32>(g_2c70) == -1 {
                                let mut sv: [u64; 2] =
                                    [exe::ptr::<u8>(0x1_433f_4668) as u64, 0x11];
                                exe::call_va!(
                                    0x1_4220_ea70,
                                    (
                                        exe::ptr::<u8>(0x1_43cf_3310) as usize,
                                        sv.as_mut_ptr() as usize,
                                    )
                                );
                                wr::<u32>(exe::ptr::<u32>(0x1_43cf_3314) as usize, 0);
                                wr::<usize>(
                                    exe::ptr::<usize>(0x1_43cf_3308) as usize,
                                    exe::ptr::<u8>(0x1_43cf_3318) as usize,
                                );
                                exe::call_va!(
                                    0x1_4183_4d18,
                                    (exe::ptr::<u8>(0x1_4262_ee10) as usize)
                                );
                                exe::call_va!(0x1_4183_4a0c, (g_2c70));
                            }
                        }
                        let mut gamma_bits = rd::<u32>(exe::ptr::<u32>(0x1_4384_1640) as usize);
                        let rcx_3310 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_3310) as usize) as isize;
                        let tbl_37b8 = exe::ptr::<u8>(0x1_43a6_37b8) as isize;
                        let r8_len =
                            rd::<i32>((tbl_37b8 + rcx_3310 * 16) as usize) as i64 as u64;
                        exe::call_va!(
                            0x1_4209_a900,
                            (
                                rcx_3310 as i64 as u64,
                                0u32,
                                r8_len,
                                (&mut gamma_bits as *mut u32) as usize,
                            )
                        );
                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_3318) as usize, gamma_bits);
                    }
                }

                let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                let g_2ac0 = exe::ptr::<i32>(0x1_43cf_2ac0) as usize;
                if rd::<i32>(g_2ac0) > tls_epoch {
                    exe::call_va!(0x1_4183_4a78, (g_2ac0));
                    if rd::<i32>(g_2ac0) == -1 {
                        let mut sv: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f280) as u64, 0x0d];
                        exe::call_va!(
                            0x1_4220_ea70,
                            (
                                exe::ptr::<u8>(0x1_43cf_4880) as usize,
                                sv.as_mut_ptr() as usize,
                            )
                        );
                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_4884) as usize, 9);
                        wr::<usize>(
                            exe::ptr::<usize>(0x1_43cf_4878) as usize,
                            exe::ptr::<u8>(0x1_43cf_4888) as usize,
                        );
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_4888) as usize, 0);
                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_4890) as usize, 0);
                        exe::call_va!(
                            0x1_4183_4d18,
                            (exe::ptr::<u8>(0x1_4262_ee00) as usize)
                        );
                        exe::call_va!(0x1_4183_4a0c, (g_2ac0));
                    }
                }

                let rbx_tex = if rbx_src != 0 { rd::<usize>(rbx_src) } else { 0 };
                bind_draw_source_slot(
                    0x1_43cf_4880,
                    0x1_43cf_4888,
                    0x1_43cf_4890,
                    rbx_tex,
                );

                let g_7d8 = exe::ptr::<u64>(0x1_43c8_c7d8) as usize;
                let mut sh = rd::<u64>(g_7d8);
                if sh == 0 {
                    let mut sv_rfx: [u64; 2] = [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                    let rfx = exe::call_va!(
                        0x1_4239_70b0,
                        (
                            rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                            sv_rfx.as_mut_ptr() as usize,
                        ) -> u64
                    );
                    let mut sv_tech: [u64; 2] = [exe::ptr::<u8>(0x1_433f_4638) as u64, 0x0d];
                    let tech = exe::call_va!(
                        0x1_4239_8a00,
                        (rfx, sv_tech.as_mut_ptr() as usize) -> u64
                    );
                    wr::<u64>(g_7d8, tech);
                    exe::call_va!(0x1_4239_7700, (g_7d8));
                    sh = rd::<u64>(g_7d8);
                }
                (sh, 2u32)
            };

            let pass_obj = exe::call_va!(0x1_4220_e860, (shader_rcx, r13d) -> u64);
            exe::call_va!(0x1_4224_ded0, (pass_obj));
            exe::call_va!(0x1_422e_6b00, ());

            if rd::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize) > 0 {
                let rbx_408 = rd::<usize>(exe::ptr::<usize>(0x1_4385_8408) as usize);
                let rdi_410 = rd::<usize>(exe::ptr::<usize>(0x1_4385_8410) as usize);
                if rbx_408 != 0 && rdi_410 != 0 {
                    let mut rt_desc2 = [0u8; 0x118];
                    let rt2_ptr = rt_desc2.as_mut_ptr() as usize;
                    wr::<u32>(rt2_ptr, 1);
                    wr::<usize>(rt2_ptr + 0x08, rdi_410);
                    exe::call_va!(0x1_4205_49b0, (rt2_ptr, 0u32));
                    exe::call_va!(0x1_422e_6dc0, ());
                    exe::call_va!(0x1_4206_7c90, (rd::<usize>(rdi_410)));

                    let mut zero_col2 = [0u32; 4];
                    exe::call_va!(
                        0x1_4206_edb0,
                        (0usize, rd::<usize>(rdi_410), zero_col2.as_mut_ptr() as usize)
                    );

                    let tls_epoch = rd::<i32>(rd::<usize>(exe::tls_pointer()) + 0x74);
                    let g_31d4 = exe::ptr::<i32>(0x1_43cf_31d4) as usize;
                    if rd::<i32>(g_31d4) > tls_epoch {
                        exe::call_va!(0x1_4183_4a78, (g_31d4));
                        if rd::<i32>(g_31d4) == -1 {
                            let mut sv: [u64; 2] =
                                [exe::ptr::<u8>(0x1_433e_f280) as u64, 0x0d];
                            exe::call_va!(
                                0x1_4220_ea70,
                                (
                                    exe::ptr::<u8>(0x1_43cf_3da8) as usize,
                                    sv.as_mut_ptr() as usize,
                                )
                            );
                            wr::<u32>(exe::ptr::<u32>(0x1_43cf_3dac) as usize, 9);
                            wr::<usize>(
                                exe::ptr::<usize>(0x1_43cf_3da0) as usize,
                                exe::ptr::<u8>(0x1_43cf_3db0) as usize,
                            );
                            wr::<u64>(exe::ptr::<u64>(0x1_43cf_3db0) as usize, 0);
                            wr::<u64>(exe::ptr::<u64>(0x1_43cf_3db8) as usize, 0);
                            exe::call_va!(
                                0x1_4183_4d18,
                                (exe::ptr::<u8>(0x1_4262_edf0) as usize)
                            );
                            exe::call_va!(0x1_4183_4a0c, (g_31d4));
                        }
                    }

                    let rbx_tex2 = rd::<usize>(rbx_408);
                    bind_draw_source_slot(
                        0x1_43cf_3da8,
                        0x1_43cf_3db0,
                        0x1_43cf_3db8,
                        rbx_tex2,
                    );

                    if mode_408 == 1 {
                        let g_900 = exe::ptr::<u64>(0x1_43c8_c900) as usize;
                        let mut sh2 = rd::<u64>(g_900);
                        if sh2 == 0 {
                            let mut sv_rfx: [u64; 2] =
                                [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                            let rfx = exe::call_va!(
                                0x1_4239_70b0,
                                (
                                    rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                                    sv_rfx.as_mut_ptr() as usize,
                                ) -> u64
                            );
                            let mut sv_tech: [u64; 2] =
                                [exe::ptr::<u8>(0x1_433f_4648) as u64, 0x1c];
                            let tech = exe::call_va!(
                                0x1_4239_8a00,
                                (rfx, sv_tech.as_mut_ptr() as usize) -> u64
                            );
                            wr::<u64>(g_900, tech);
                            exe::call_va!(0x1_4239_7700, (g_900));
                            sh2 = rd::<u64>(g_900);
                        }
                        let pass2 = exe::call_va!(0x1_4220_e860, (sh2, r13d) -> u64);
                        exe::call_va!(0x1_4224_ded0, (pass2));
                    } else if mode_408 == 2 {
                        let g_950 = exe::ptr::<u64>(0x1_43c8_c950) as usize;
                        let mut sh2 = rd::<u64>(g_950);
                        if sh2 == 0 {
                            let mut sv_rfx: [u64; 2] =
                                [exe::ptr::<u8>(0x1_433e_f290) as u64, 8];
                            let rfx = exe::call_va!(
                                0x1_4239_70b0,
                                (
                                    rd::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize),
                                    sv_rfx.as_mut_ptr() as usize,
                                ) -> u64
                            );
                            let mut sv_tech: [u64; 2] =
                                [exe::ptr::<u8>(0x1_433f_4638) as u64, 0x0d];
                            let tech = exe::call_va!(
                                0x1_4239_8a00,
                                (rfx, sv_tech.as_mut_ptr() as usize) -> u64
                            );
                            wr::<u64>(g_950, tech);
                            exe::call_va!(0x1_4239_7700, (g_950));
                            sh2 = rd::<u64>(g_950);
                        }
                        let pass2 = exe::call_va!(0x1_4220_e860, (sh2, r13d) -> u64);
                        exe::call_va!(0x1_4224_ded0, (pass2));
                    }

                    exe::call_va!(0x1_422e_6b00, ());
                    exe::call_va!(0x1_4206_94a0, (rd::<usize>(rdi_410)));
                }
            }
        }
    }

    if rd::<u8>(param_1 + 0x460) != 0 {
        let r12_448 = rd::<usize>(param_1 + 0x448);
        wr::<usize>(param_1 + 0x448, 0);
        if r12_448 != 0 {
            let r13_ff18 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize);
            let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
            if rd::<u32>(r13_ff18 + 0x10) != tid {
                exe::call_iat!(0x1_4264_55a8, (r13_ff18 + 0x08));
                wr::<u32>(r13_ff18 + 0x10, tid);
            }
            wr::<i32>(
                r13_ff18 + 0x14,
                rd::<i32>(r13_ff18 + 0x14).wrapping_add(1),
            );

            let r8d_cnt = rd::<u32>(r13_ff18 + 0x20) as usize;
            let rdi_arr = rd::<usize>(r13_ff18 + 0x18);
            let mut idx = 0usize;
            while idx < r8d_cnt {
                let r14_off = idx * 0x68;
                let rbx_elem = rd::<usize>(rdi_arr + r14_off);
                if rbx_elem == r12_448 {
                    wr::<u8>(rdi_arr + r14_off + 0x38, 0);
                    wr::<u32>(
                        rdi_arr + r14_off + 0x64,
                        rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize),
                    );
                    if rd::<u8>(rdi_arr + r14_off + 0x39) != 0 {
                        if rbx_elem != 0 {
                            exe::call_va!(0x1_4204_f860, (rbx_elem));
                            exe::call_va!(0x1_4244_2de0, (rbx_elem));
                        }
                        wr::<usize>(rdi_arr + r14_off, 0);
                        let rdi_cur = rd::<usize>(r13_ff18 + 0x18);
                        let total_off = (rd::<u32>(r13_ff18 + 0x20) as usize) * 0x68;
                        let last_ptr = rdi_cur + total_off - 0x68;
                        let dst_ptr = rdi_cur + r14_off;
                        wr::<u64>(dst_ptr, rd::<u64>(last_ptr));
                        wr::<[u64; 4]>(dst_ptr + 0x08, rd::<[u64; 4]>(last_ptr + 0x08));
                        wr::<[u64; 2]>(dst_ptr + 0x28, rd::<[u64; 2]>(last_ptr + 0x28));
                        wr::<u8>(dst_ptr + 0x38, rd::<u8>(last_ptr + 0x38));
                        wr::<u8>(dst_ptr + 0x39, rd::<u8>(last_ptr + 0x39));
                        exe::call_va!(0x1_4240_aa00, (dst_ptr + 0x3c, last_ptr + 0x3c));
                        wr::<u32>(dst_ptr + 0x64, rd::<u32>(last_ptr + 0x64));
                        let tail_str = rd::<usize>(r13_ff18 + 0x18)
                            .wrapping_add((rd::<u32>(r13_ff18 + 0x20) as usize) * 0x68)
                            .wrapping_sub(0x2c);
                        exe::call_va!(0x1_4240_ab10, (tail_str));
                        wr::<u32>(
                            r13_ff18 + 0x20,
                            rd::<u32>(r13_ff18 + 0x20).wrapping_sub(1),
                        );
                    }
                    break;
                }
                idx += 1;
            }

            let new_depth = rd::<i32>(r13_ff18 + 0x14).wrapping_sub(1);
            wr::<i32>(r13_ff18 + 0x14, new_depth);
            if new_depth == 0 {
                let _ = crate::simd_ops::atomic_xchg_u32((r13_ff18 + 0x10) as u64, u32::MAX);
                exe::call_iat!(0x1_4264_5598, (r13_ff18 + 0x08));
            }
        }
        wr::<u8>(param_1 + 0x460, 0);
    }

    wr::<usize>(param_1 + 0x448, 0);
    let rdx_1a8 = rd::<usize>(param_1 + 0x1a8);
    if rdx_1a8 != 0 {
        let rcx_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        exe::call_va!(0x1_4248_5780, (rcx_mgr, rdx_1a8 + 0x628));
    }

    exe::call_va!(0x1_421e_f120, (rd::<usize>(param_1 + 0x598)));
    wr::<u32>(exe::ptr::<u32>(0x1_43a2_b14c) as usize, 0);
    exe::call_va!(0x1_4205_6660, ());

    let val_918 = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
    let mut tmp_390 = [0u8; 40];
    let ret_ptr = exe::call_va!(0x1_4207_34e0, (tmp_390.as_mut_ptr() as usize, 0u32) -> usize);
    let xmm6_lo = rd::<u64>(ret_ptr);
    let xmm6_hi = rd::<u64>(ret_ptr + 0x08);

    let vec_addr = exe::ptr::<usize>(0x1_43a6_31e8) as usize;
    let cnt_addr = exe::ptr::<u32>(0x1_43a6_31f0) as usize;
    let cap_addr = exe::ptr::<u32>(0x1_43a6_31f4) as usize;

    let mut r9d_cnt = rd::<u32>(cnt_addr) as usize;
    let mut rdx_base = rd::<usize>(vec_addr);
    let cap = rd::<u32>(cap_addr) as usize;
    if rdx_base.wrapping_add(r9d_cnt * 24) == rdx_base.wrapping_add(cap * 24) {
        exe::call_va!(
            0x1_411d_5690,
            ((cap as u64) * 3, (r9d_cnt + 1) as u64)
        );
        r9d_cnt = rd::<u32>(cnt_addr) as usize;
        rdx_base = rd::<usize>(vec_addr);
    }
    let dst_entry = rdx_base.wrapping_add(r9d_cnt * 24);
    wr::<u64>(dst_entry, val_918);
    wr::<u64>(dst_entry + 0x08, xmm6_lo);
    wr::<u64>(dst_entry + 0x10, xmm6_hi);
    wr::<u32>(cnt_addr, (r9d_cnt as u32).wrapping_add(1));

    let rcx_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    exe::call_va!(
        0x1_4248_5780,
        (rcx_mgr, exe::ptr::<u8>(0x1_4397_ff38) as usize)
    );
    let rdx_1e8 = rd::<usize>(param_1 + 0x1e8);
    let rcx_mgr2 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    exe::call_va!(0x1_4248_5780, (rcx_mgr2, rdx_1e8 + 0x60));
}

pub unsafe fn diff_std_14222ef60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fed0) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fed8) as *mut u8, 8);
    let s3 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff48) as *mut u8, 8);

    let run = |f: F, count: u8| -> [u64; 2] {
        let mut bit_arr = [0u64; 2];

        let mut inner_obj = [0u8; 0x280];
        let iop = inner_obj.as_mut_ptr() as usize;
        wr::<u8>(iop + 0x272, 2);
        wr::<i32>(iop + 0x1c8, 1);
        wr::<i32>(iop + 0x1cc, 1);

        let mut elem_arr = [0u8; 8 * 0x40];
        wr::<usize>(elem_arr.as_mut_ptr() as usize + 5 * 0x40, iop);

        let mut nil_node = [0u8; 0x30];
        let nilp = nil_node.as_mut_ptr() as usize;
        wr::<u8>(nilp + 0x19, 1);

        let mut root_node = [0u8; 0x5c0];
        let rootp = root_node.as_mut_ptr() as usize;
        wr::<usize>(rootp, nilp);
        wr::<usize>(rootp + 0x08, nilp);
        wr::<usize>(rootp + 0x10, nilp);
        wr::<u8>(rootp + 0x19, 0);
        wr::<u64>(rootp + 0x20, 0x1234_5678);
        wr::<usize>(rootp + 0x570, bit_arr.as_mut_ptr() as usize);
        wr::<usize>(rootp + 0x5a0, elem_arr.as_mut_ptr() as usize);

        wr::<usize>(nilp + 0x08, rootp);
        let tree_head_ptr: usize = nilp;

        let slot_table: [u32; 4] = [0xffff_ffff, 0, 0xffff_ffff, 0xffff_ffff];
        let mut fed0_obj = [0u8; 0x120];
        wr::<usize>(
            fed0_obj.as_mut_ptr() as usize + 0x110,
            slot_table.as_ptr() as usize,
        );

        let mut entry_140 = [0u8; 0x30];
        let e140p = entry_140.as_mut_ptr() as usize;
        wr::<u64>(e140p + 0x08, 0x1234_5678);
        wr::<u32>(e140p + 0x10, 5);

        let mut ff48_obj = [0u8; 0x150];
        wr::<usize>(ff48_obj.as_mut_ptr() as usize + 0x140, e140p);

        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fed0) as usize,
            fed0_obj.as_ptr() as usize,
        );
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fed8) as usize,
            (&tree_head_ptr as *const usize) as usize,
        );
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_ff48) as usize,
            ff48_obj.as_ptr() as usize,
        );

        let mut obj_90 = [0u8; 0x280];
        let o90p = obj_90.as_mut_ptr() as usize;
        wr::<u8>(o90p + 0x270, count);
        wr::<u8>(o90p + 0x272, 2);
        wr::<i32>(o90p + 0x1c8, 3);
        wr::<i32>(o90p + 0x1cc, 4);

        let mut p1 = [0u8; 0xa0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x90, o90p);
        wr::<u32>(p1p + 0x98, 1);

        f(p1p);
        bit_arr
    };

    for cnt in [0u8, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "14222ef60 cnt={cnt}");
    }

    s3.restore();
    s2.restore();
    s1.restore();
    Ok(2)
}

pub unsafe fn diff_std_14222f0f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff70) as *mut u8, 8);

    let run = |f: F, mask: u8| -> [u64; 4] {
        let mut bit_arr = [0u64; 4];
        let mut ff70_obj = [0u8; 0x80];
        wr::<usize>(
            ff70_obj.as_mut_ptr() as usize + 0x78,
            bit_arr.as_mut_ptr() as usize,
        );
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_ff70) as usize,
            ff70_obj.as_ptr() as usize,
        );

        let mut subs = [[0u8; 0x60]; 8];
        let bit_indices: [u32; 8] = [3, 64, 65, 127, 128, 195, 250, 42];
        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        for i in 0..8usize {
            let sp = subs[i].as_mut_ptr() as usize;
            wr::<u32>(sp + 0x58, bit_indices[i]);
            if (mask & (1 << i)) != 0 {
                wr::<usize>(p1p + 0x40 + i * 8, sp);
            }
        }

        f(p1p);
        bit_arr
    };

    for mask in [0b1010_0101u8, 0b1111_1111] {
        let o = run(f_o, mask);
        let n = run(f_n, mask);
        crate::diff_eq!(o, n, "14222f0f0 mask={mask}");
    }

    s1.restore();
    Ok(2)
}

pub unsafe fn diff_unknown_14226a820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn noop0() {
        unsafe { rec(0x1000) };
    }
    unsafe extern "win64" fn noop1(a: u64) {
        unsafe { rec(0x1001 ^ (a != 0) as u64) };
    }
    unsafe extern "win64" fn mock_tid() -> u32 {
        unsafe { rec(0x7790) };
        0x4242
    }
    unsafe extern "win64" fn mock_acq(_lock: usize) {
        unsafe { rec(0x55a8) };
    }
    unsafe extern "win64" fn mock_rel(_lock: usize) {
        unsafe { rec(0x5598) };
    }
    unsafe extern "win64" fn mock_6b70(_vec: usize, _flag: u8) {
        unsafe { rec(0x6b70) };
    }
    unsafe extern "win64" fn mock_5780(mgr: u64, _ptr: usize) {
        unsafe { rec(0x5780 ^ mgr) };
    }
    unsafe extern "win64" fn mock_8080(mgr: u64, item: u64) {
        unsafe { rec(0x8080 ^ mgr ^ item) };
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        unsafe { rec(0x7c80) };
        2
    }
    unsafe extern "win64" fn mock_7130(_ptr: usize, v: u32) {
        unsafe { rec(0x7130 ^ (v as u64)) };
    }
    unsafe extern "win64" fn mock_a5780(
        sv: usize,
        _r1: usize,
        _r2: usize,
        mode: u32,
        a5: u32,
        _a6: u32,
        _a7: u32,
        _a8: u32,
    ) {
        unsafe { rec(0xa578 ^ rd::<u64>(sv + 0x08) ^ (mode as u64) ^ (a5 as u64)) };
    }
    unsafe extern "win64" fn mock_edb0(_rcx: usize, rdx: u64, col: usize) {
        unsafe { rec(0xedb0 ^ rdx ^ (rd::<u32>(col) as u64)) };
    }
    unsafe extern "win64" fn mock_49b0(rt: usize, flag: u32) {
        unsafe { rec(0x49b0 ^ (rd::<u32>(rt) as u64) ^ (flag as u64)) };
    }
    unsafe extern "win64" fn mock_f420(
        _tls: usize,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        z0: u32,
        z1: u32,
    ) {
        unsafe {
            rec(0xf420
                ^ (x as u64)
                ^ (y as u64)
                ^ (w as u64)
                ^ (h as u64)
                ^ (z0 as u64)
                ^ (z1 as u64));
        }
    }
    unsafe extern "win64" fn mock_ea70(out_id: usize, sv: usize) {
        wr::<i32>(out_id, 1);
        unsafe { rec(0xea70 ^ rd::<u64>(sv + 0x08)) };
    }
    unsafe extern "win64" fn mock_a0c(guard: usize) {
        wr::<i32>(guard, 0);
        unsafe { rec(0xa0c0) };
    }
    unsafe extern "win64" fn mock_70b0(mgr: u64, sv: usize) -> u64 {
        unsafe { rec(0x70b0 ^ mgr ^ rd::<u64>(sv + 0x08)) };
        0x5555_0001
    }
    unsafe extern "win64" fn mock_8a00(rfx: u64, sv: usize) -> u64 {
        unsafe { rec(0x8a00 ^ rfx ^ rd::<u64>(sv + 0x08)) };
        0x6666_0002
    }
    unsafe extern "win64" fn mock_8b20(
        rcx: u64,
        _rdx: usize,
        w1: u32,
        r9: u32,
        w2: u32,
        h: u32,
    ) {
        unsafe {
            rec(0x8b20 ^ rcx ^ (w1 as u64) ^ (r9 as u64) ^ (w2 as u64) ^ (h as u64));
        }
    }
    unsafe extern "win64" fn mock_a900(id: u64, edx: u32, r8: u64, ptr: usize) {
        unsafe { rec(0xa900 ^ id ^ (edx as u64) ^ r8 ^ (rd::<u32>(ptr) as u64)) };
    }
    unsafe extern "win64" fn mock_e860(tech: u64, flags: u32) -> u64 {
        unsafe { rec(0xe860 ^ tech ^ (flags as u64)) };
        0x7777_0003
    }
    unsafe extern "win64" fn mock_aa00(_dst: usize, _src: usize) {
        unsafe { rec(0xaa00) };
    }
    unsafe extern "win64" fn mock_ab10(_ptr: usize) {
        unsafe { rec(0xab10) };
    }
    unsafe extern "win64" fn mock_34e0(buf: usize, edx: u32) -> usize {
        unsafe { rec(0x34e0 ^ (edx as u64)) };
        wr::<u64>(buf, 0x1111_2222_3333_4444);
        wr::<u64>(buf + 0x08, 0x5555_6666_7777_8888);
        buf
    }
    unsafe extern "win64" fn mock_5690(rcx: u64, rdx: u64) {
        unsafe { rec(0x5690 ^ rcx ^ rdx) };
    }

    let _i1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _i3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;

    let _m1 = crate::difftest::Mock::new(0x1_4204_f860, noop1 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, noop1 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_6b70, mock_6b70 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_5780, mock_5780 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4183_49c0, noop0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4225_8080, mock_8080 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40fe_7130, mock_7130 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_422a_5780, mock_a5780 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4206_edb0, mock_edb0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4205_49b0, mock_49b0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4206_f420, mock_f420 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_422e_6dc0, noop0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4183_4a78, noop1 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4183_4d18, noop1 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_a0c as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4239_70b0, mock_70b0 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4239_8a00, mock_8a00 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_4239_7700, noop1 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_4239_8b20, mock_8b20 as *const ())?;
    let _m22 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;
    let _m23 = crate::difftest::Mock::new(0x1_4220_e860, mock_e860 as *const ())?;
    let _m24 = crate::difftest::Mock::new(0x1_4224_ded0, noop1 as *const ())?;
    let _m25 = crate::difftest::Mock::new(0x1_422e_6b00, noop0 as *const ())?;
    let _m26 = crate::difftest::Mock::new(0x1_4206_7c90, noop1 as *const ())?;
    let _m27 = crate::difftest::Mock::new(0x1_4206_94a0, noop1 as *const ())?;
    let _m28 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m29 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m30 = crate::difftest::Mock::new(0x1_421e_f120, noop1 as *const ())?;
    let _m31 = crate::difftest::Mock::new(0x1_4205_6660, noop0 as *const ())?;
    let _m32 = crate::difftest::Mock::new(0x1_4207_34e0, mock_34e0 as *const ())?;
    let _m33 = crate::difftest::Mock::new(0x1_411d_5690, mock_5690 as *const ())?;

    let tls_arr = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_arr);
    let mut tls_buf = vec![0u8; 0x47400];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    wr::<usize>(tls_arr, tls0);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_0180) as *mut u8, 4);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8588) as *mut u8, 8);
    let s3 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_82f8) as *mut u8, 8);
    let s4 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff18) as *mut u8, 8);
    let s5 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d8da) as *mut u8, 1);
    let s6 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a2_b11c) as *mut u8, 4);
    let s7 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4384_1640) as *mut u8, 4);
    let s8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a2_8cc6) as *mut u8, 1);
    let s9 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb8) as *mut u8, 8);
    let s10 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_440c) as *mut u8, 4);
    let s11 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_3b50) as *mut u8, 0x20);
    let s12 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_2c70) as *mut u8, 4);
    let s13 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_3308) as *mut u8, 0x18);
    let s14 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_2ac0) as *mut u8, 4);
    let s15 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4878) as *mut u8, 0x20);
    let s16 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_31d4) as *mut u8, 4);
    let s17 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_3da0) as *mut u8, 0x20);
    let s18 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c838) as *mut u8, 8);
    let s19 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c7d8) as *mut u8, 8);
    let s20 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c900) as *mut u8, 8);
    let s21 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c950) as *mut u8, 8);
    let s22 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_83f8) as *mut u8, 0x20);
    let s23 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a2_b14c) as *mut u8, 4);
    let s24 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_5918) as *mut u8, 8);
    let s25 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_31e8) as *mut u8, 0x10);
    let s26 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_0210) as *mut u8, 8);
    let s27 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8590) as *mut u8, 0x10);
    let s28 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_37b0) as *mut u8, 0x20);

    let run = |f: F, mode: u32| -> (u64, [u64; 3], [u64; 2], i32, u32, u8, usize) {
        core::ptr::write_bytes(tls0 as *mut u8, 0, 0x47400);
        wr::<u8>(tls0 + 0x70, 1);
        wr::<i32>(tls0 + 0x74, 0);

        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let tbl_8590 = exe::ptr::<u8>(0x1_4385_8590) as usize;
        let tbl_37b0 = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
        wr::<i32>(tbl_8590 + 4, 0);
        wr::<i32>(tbl_37b0 + 0x10, 0x10);
        wr::<i32>(tbl_37b0 + 0x18, 16);
        wr::<i32>(tbl_37b0 + 0x1c, 2);

        wr::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize, 1000);
        wr::<u64>(exe::ptr::<u64>(0x1_4385_8588) as usize, 0x1234_5678_9abc_def0);
        wr::<u64>(exe::ptr::<u64>(0x1_43a0_82f8) as usize, 0x82f8_0000);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8da) as usize, 7);
        wr::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize, 1);
        wr::<f32>(exe::ptr::<f32>(0x1_4384_1640) as usize, 2.2);
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc6) as usize, 1);
        wr::<u64>(exe::ptr::<u64>(0x1_43a0_0210) as usize, 0x0210_0000);

        wr::<i32>(exe::ptr::<i32>(0x1_43cf_440c) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_2c70) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_2ac0) as usize, -1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_31d4) as usize, -1);

        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c838) as usize, 0);
        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c7d8) as usize, 0);
        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c900) as usize, 0);
        wr::<u64>(exe::ptr::<u64>(0x1_43c8_c950) as usize, 0);

        let mut fcb8_buf = [0u8; 0xa0];
        wr::<u64>(fcb8_buf.as_mut_ptr() as usize + 0x88, 0xaaaa_8888);
        wr::<u64>(fcb8_buf.as_mut_ptr() as usize + 0x90, 0xbbbb_9999);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fcb8) as usize,
            fcb8_buf.as_ptr() as usize,
        );

        let mut tex_obj = [0u8; 0xa0];
        let tex_p = tex_obj.as_mut_ptr() as usize;
        wr::<u32>(tex_p, 1920);
        wr::<u32>(tex_p + 0x04, 1080);
        wr::<u64>(tex_p + 0x78, 0x7878_1111);
        wr::<u64>(tex_p + 0x90, 0x9090_2222);
        let wrapper_ptr: usize = tex_p;
        let wrapper_addr = (&wrapper_ptr as *const usize) as usize;

        let rt_obj: u64 = 0xdead_be01;
        wr::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize, 1);
        wr::<usize>(exe::ptr::<usize>(0x1_4385_8408) as usize, wrapper_addr);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4385_8410) as usize,
            (&rt_obj as *const u64) as usize,
        );

        let mut ff18_elems = [0u8; 2 * 0x68];
        let fe_p = ff18_elems.as_mut_ptr() as usize;
        wr::<usize>(fe_p, wrapper_addr);
        wr::<u8>(fe_p + 0x38, 1);
        wr::<u8>(fe_p + 0x39, 1);
        wr::<i32>(fe_p + 0x64, 10);

        let mut ff18_mgr = [0u8; 0x30];
        let fm_p = ff18_mgr.as_mut_ptr() as usize;
        wr::<u32>(fm_p + 0x10, 0xffff_ffff);
        wr::<usize>(fm_p + 0x18, fe_p);
        wr::<u32>(fm_p + 0x20, 1);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize, fm_p);

        let mut p58_elems = [0u8; 0x68];
        let p58e = p58_elems.as_mut_ptr() as usize;
        wr::<usize>(p58e, 0x7777_0000);
        wr::<u8>(p58e + 0x38, 0);
        wr::<i32>(p58e + 0x64, 0);

        let mut p58_mgr = [0u8; 0x30];
        let p58m = p58_mgr.as_mut_ptr() as usize;
        wr::<u32>(p58m + 0x10, 0xffff_ffff);
        wr::<usize>(p58m + 0x18, p58e);
        wr::<u32>(p58m + 0x20, 1);

        let arr_160: [u64; 2] = [0x1111, 0x2222];
        let mut p160_obj = [0u8; 0x50];
        wr::<usize>(p160_obj.as_mut_ptr() as usize + 0x40, arr_160.as_ptr() as usize);
        wr::<u32>(p160_obj.as_mut_ptr() as usize + 0x48, 2);

        let mut pa8_obj = vec![0u8; 0x13 * 0xd8 + 0x100];
        wr::<u8>(pa8_obj.as_mut_ptr() as usize, 1);
        wr::<u32>(pa8_obj.as_mut_ptr() as usize + 0xb0, 0xffff_ffff);

        let mut p48_obj = [0u8; 0x80];
        let p48p = p48_obj.as_mut_ptr() as usize;
        wr::<usize>(p48p + 0x08, 0x8888_0000);
        wr::<usize>(p48p + 0x20, wrapper_addr);
        wr::<i32>(p48p + 0x38, 0);
        wr::<u8>(p48p + 0x4c, if mode == 0 { 1 } else { 0 });
        wr::<u8>(p48p + 0x66, 0);
        wr::<u8>(p48p + 0x67, 1);

        let mut out_vec = [0u64; 6];
        wr::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize, 0x9999_aaaa_bbbb_cccc);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43a6_31e8) as usize,
            out_vec.as_mut_ptr() as usize,
        );
        wr::<u32>(exe::ptr::<u32>(0x1_43a6_31f0) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_43a6_31f4) as usize, 2);

        let mut p1 = vec![0u8; 0x600];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x48, p48p);
        wr::<usize>(p1p + 0x58, p58m);
        wr::<usize>(p1p + 0xa8, pa8_obj.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x130, 0);
        wr::<usize>(p1p + 0x160, p160_obj.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x1a8, 0x1a80_0000);
        wr::<usize>(p1p + 0x1e8, 0x1e80_0000);
        wr::<usize>(p1p + 0x200, 0x2000_0000);
        wr::<usize>(p1p + 0x448, wrapper_addr);
        wr::<u8>(p1p + 0x460, 1);
        wr::<u8>(p1p + 0x49d, 1);
        wr::<usize>(p1p + 0x598, 0x5980_0000);

        f(p1p);

        (
            unsafe { TRACE_HASH },
            [out_vec[0], out_vec[1], out_vec[2]],
            rd::<[u64; 2]>(tls0 + 0x63e0),
            rd::<i32>(tls0 + 0x463e8),
            rd::<u32>(exe::ptr::<u32>(0x1_43a6_31f0) as usize),
            rd::<u8>(p1p + 0x460),
            rd::<usize>(p1p + 0x448),
        )
    };

    for mode in 0..2 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14226a820 mode={mode}");
    }

    s28.restore();
    s27.restore();
    s26.restore();
    s25.restore();
    s24.restore();
    s23.restore();
    s22.restore();
    s21.restore();
    s20.restore();
    s19.restore();
    s18.restore();
    s17.restore();
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

// REGISTRY:
// Replacement::verified_throws(0x1_4222_ef60, "std_14222ef60", rend::cauldron_hot_z8b::std_14222ef60 as *const (), Throws::Discard, rend::cauldron_hot_z8b::diff_std_14222ef60),
// Replacement::verified(0x1_4222_f0f0, "std_14222f0f0", rend::cauldron_hot_z8b::std_14222f0f0 as *const (), rend::cauldron_hot_z8b::diff_std_14222f0f0),
// Replacement::verified_throws(0x1_4226_a820, "unknown_14226a820", rend::cauldron_hot_z8b::unknown_14226a820 as *const (), Throws::Caught, rend::cauldron_hot_z8b::diff_unknown_14226a820),
