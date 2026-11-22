//! Decompiled hot runtime functions (`cauldron_hot_j`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{vpcmpgtb, vpmovmskb};

#[inline(always)]
fn key_lt_3fff(a: u64, b: u64) -> bool {
    let wa = (a as u16) & 0x3fff;
    let wb = (b as u16) & 0x3fff;
    if wa != wb {
        wa < wb
    } else {
        a < b
    }
}

#[inline(always)]
unsafe fn lower_bound_3fff(mut base: usize, mut count: isize, key: u64) -> usize {
    if count > 0 {
        while count > 0 {
            let half = (count as usize) >> 1;
            let mid = base.wrapping_add(half * 8);
            if key_lt_3fff(rd::<u64>(mid), key) {
                base = mid.wrapping_add(8);
                count = count.wrapping_sub((half as isize).wrapping_add(1));
            } else {
                count = half as isize;
            }
        }
    }
    base
}

#[inline(always)]
unsafe fn skip_ctrl_neg(mut r13: usize, mut r14: usize) -> (usize, usize) {
    if rd::<i8>(r13) < -1 {
        let all_ff = u128::MAX;
        loop {
            let cmp = vpcmpgtb(all_ff, rd::<u128>(r13));
            let mask = (vpmovmskb(cmp) as u32).wrapping_add(1);
            let step = if mask != 0 {
                mask.trailing_zeros() as usize
            } else {
                0
            };
            r13 = r13.wrapping_add(step);
            r14 = r14.wrapping_add(step.wrapping_mul(0x18));
            if rd::<i8>(r13) >= -1 {
                break;
            }
        }
    }
    (r13, r14)
}

/// `unknown_141e59da0`
/// Original: `0x141e59da0` (1953 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_141e59da0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let mut vec_30_ptr: usize = 0;
    let mut edi_len: u32 = 0;
    let mut esi_cap: u32 = 0;
    let mut r11_total: u64 = 0;

    let span1 = rd::<[u64; 2]>(param_1 + 0x08);
    let rbx_world = rd::<usize>(param_1);
    let r14_base = rbx_world.wrapping_add(0x40);
    let mut iter_init1 = [0u64; 3];
    let ret_iter1 = exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init1.as_mut_ptr() as usize,
            r14_base,
            span1.as_ptr() as usize
        ) -> usize
    );
    let iter1_base = rd::<usize>(ret_iter1);
    let mut cur_arch1 = rd::<usize>(ret_iter1 + 0x08);
    let end_arch1 = rd::<usize>(ret_iter1 + 0x10);

    if cur_arch1 != end_arch1 {
        loop {
            let arch_id = rd::<u16>(cur_arch1) as usize;
            let add_cnt = if rd::<u64>(rbx_world.wrapping_add(arch_id * 8).wrapping_add(0x88)) != 0
            {
                rd::<u32>(rbx_world.wrapping_add(arch_id * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            r11_total = r11_total.wrapping_add(add_cnt);
            cur_arch1 = cur_arch1.wrapping_add(2);
            if cur_arch1 == end_arch1 {
                break;
            }
            while cur_arch1 != end_arch1 {
                let a_id = rd::<u16>(cur_arch1) as usize;
                if rd::<u64>(iter1_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter1_base.wrapping_add(a_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch1 = cur_arch1.wrapping_add(2);
            }
            if cur_arch1 == end_arch1 {
                break;
            }
        }
    }

    if r11_total != 0 {
        vec_30_ptr = exe::call_va!(0x1_4241_c0a0, (r11_total.wrapping_mul(8), 8u64) -> usize);
        edi_len = 0;
        esi_cap = r11_total as u32;
    }

    let span2 = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init2 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init2.as_mut_ptr() as usize,
            r14_base,
            span2.as_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 12];
    outer[0] = r14_base as u64;
    outer[1] = iter_init2[0];
    outer[2] = iter_init2[1];
    outer[3] = iter_init2[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(r14_base.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(r14_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_40fb_7180, (outer.as_mut_ptr() as usize));

    while outer[6] < outer[5] {
        let mut rbx_idx = outer[6] as usize;
        let r15_cnt = outer[5] as usize;
        while rbx_idx < r15_cnt {
            let r13_item = (outer[7] as usize).wrapping_add(rbx_idx.wrapping_mul(0x18));
            let cl = rd::<u8>((outer[8] as usize).wrapping_add(rbx_idx));
            if (cl & 3) == 1 || cl == 0 {
                let cur_end = vec_30_ptr.wrapping_add((edi_len as usize) * 8);
                let cap_end = vec_30_ptr.wrapping_add((esi_cap as usize) * 8);
                if cur_end == cap_end {
                    let byte_len = cur_end.wrapping_sub(vec_30_ptr);
                    let cur_cnt = ((byte_len as isize) >> 3) as usize;
                    let req_cnt = cur_cnt.wrapping_add(1);
                    let cap_cnt = ((cap_end.wrapping_sub(vec_30_ptr) as isize) >> 3) as usize;
                    let grown = (cap_cnt >> 1).wrapping_add(cap_cnt);
                    let new_cap = if grown < req_cnt { req_cnt } else { grown };
                    let new_ptr =
                        exe::call_va!(0x1_4241_c0a0, ((new_cap * 8) as u64, 8u64) -> usize);
                    let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, vec_30_ptr, byte_len) -> usize);
                    if vec_30_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (vec_30_ptr));
                    }
                    vec_30_ptr = new_ptr;
                    esi_cap = new_cap as u32;
                }
                let val = rd::<u64>(r13_item);
                wr::<u64>(vec_30_ptr.wrapping_add((edi_len as usize) * 8), val);
                edi_len = edi_len.wrapping_add(1);
            }
            rbx_idx += 1;
            outer[6] = rbx_idx as u64;
        }

        let mut cur_arch = (outer[2] as usize).wrapping_add(2);
        outer[2] = cur_arch as u64;
        let end_arch = outer[3] as usize;
        if cur_arch != end_arch {
            let iter_base = outer[1] as usize;
            while cur_arch != end_arch {
                let a_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_base.wrapping_add(a_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
            }
            outer[2] = cur_arch as u64;
            if cur_arch != end_arch {
                let base_0 = outer[0] as usize;
                let a_id = rd::<u16>(cur_arch) as usize;
                let cp = rd::<u64>(base_0.wrapping_add(a_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(a_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_40fb_7180, (outer.as_mut_ptr() as usize));
    }

    let vec_30_end = vec_30_ptr.wrapping_add((edi_len as usize) * 8);
    let vec_30_cnt = (vec_30_end.wrapping_sub(vec_30_ptr) as isize) >> 3;
    let mut sort1_begin = vec_30_ptr;
    let mut sort1_end = vec_30_end;
    exe::call_va!(
        0x1_40fb_9d30,
        (
            (&mut sort1_begin as *mut usize) as usize,
            (&mut sort1_end as *mut usize) as usize,
            vec_30_cnt as u64,
            0u64
        )
    );

    let r15_p3_30 = param_3.wrapping_add(0x30);
    let p3_len = rd::<u32>(r15_p3_30 + 0x08) as usize;
    let p3_ptr = rd::<usize>(r15_p3_30);
    let mut sort2_begin = p3_ptr;
    let mut sort2_end = p3_ptr.wrapping_add(p3_len * 8);
    exe::call_va!(
        0x1_40fb_9d30,
        (
            (&mut sort2_begin as *mut usize) as usize,
            (&mut sort2_end as *mut usize) as usize,
            p3_len as u64,
            0u64
        )
    );

    let ctrl_base = rd::<usize>(param_3);
    let entries_base = rd::<usize>(param_3 + 0x08);
    let (mut r13_ctrl, mut r14_entry) = skip_ctrl_neg(ctrl_base, entries_base);
    let ctrl_end = ctrl_base.wrapping_add(rd::<usize>(param_3 + 0x18));

    while r13_ctrl != ctrl_end {
        let mut rsi_buf: usize = 0;
        let mut r12_len: u32 = 0;
        let mut r15_cap: u32 = 0;

        let entry_cnt = rd::<u32>(r14_entry + 0x10) as usize;
        if entry_cnt != 0 {
            rsi_buf = exe::call_va!(0x1_4241_c0a0, ((entry_cnt << 5) as u64, 8u64) -> usize);
            exe::call_va!(0x1_40b1_3950, (0u64, 0u64, rsi_buf));
            r12_len = 0;
            r15_cap = entry_cnt as u32;
        }

        let mut rbx_sub = rd::<usize>(r14_entry + 0x08);
        let sub_end = rbx_sub.wrapping_add((rd::<u32>(r14_entry + 0x10) as usize) << 5);
        while rbx_sub != sub_end {
            let sub_key = rd::<u64>(rbx_sub);
            let lb = lower_bound_3fff(vec_30_ptr, vec_30_cnt, sub_key);
            if lb == vec_30_end || key_lt_3fff(sub_key, rd::<u64>(lb)) {
                let cur_end = rsi_buf.wrapping_add((r12_len as usize) << 5);
                let cap_end = rsi_buf.wrapping_add((r15_cap as usize) << 5);
                if cur_end == cap_end {
                    let cur_items = ((cur_end.wrapping_sub(rsi_buf) as isize) >> 5) as usize;
                    let req_items = cur_items.wrapping_add(1);
                    let cap_items = ((cap_end.wrapping_sub(rsi_buf) as isize) >> 5) as usize;
                    let grown = (cap_items >> 1).wrapping_add(cap_items);
                    let new_cap = if grown < req_items { req_items } else { grown };
                    let new_buf =
                        exe::call_va!(0x1_4241_c0a0, ((new_cap << 5) as u64, 8u64) -> usize);
                    exe::call_va!(0x1_40b1_3950, (rsi_buf, cur_end, new_buf));
                    if rsi_buf != cur_end {
                        let mut p = rsi_buf;
                        while p != cur_end {
                            crate::simd_ops::atomic_xchg_u64((p + 0x08) as u64, 0);
                            p = p.wrapping_add(0x20);
                        }
                    }
                    if rsi_buf != 0 {
                        exe::call_va!(0x1_4241_c190, (rsi_buf));
                    }
                    rsi_buf = new_buf;
                    r15_cap = new_cap as u32;
                }
                let dst = rsi_buf.wrapping_add((r12_len as usize) << 5);
                wr::<u64>(dst, 0);
                wr::<u64>(dst + 0x08, 0);
                wr::<u32>(dst + 0x10, 0);
                wr::<u8>(dst + 0x14, 0);
                exe::call_va!(0x1_401e_84a0, (dst, rbx_sub));
                wr::<u8>(dst + 0x18, rd::<u8>(rbx_sub + 0x18));
                r12_len = r12_len.wrapping_add(1);
            }
            rbx_sub = rbx_sub.wrapping_add(0x20);
        }

        let p3_cur_len = rd::<u32>(r15_p3_30 + 0x08) as usize;
        let p3_cur_ptr = rd::<usize>(r15_p3_30);
        let p3_cur_end = p3_cur_ptr.wrapping_add(p3_cur_len * 8);
        let entry_key = rd::<u64>(r14_entry);
        let lb2 = lower_bound_3fff(p3_cur_ptr, p3_cur_len as isize, entry_key);
        let found_in_p3: u8 = if lb2 == p3_cur_end || key_lt_3fff(entry_key, rd::<u64>(lb2)) {
            0
        } else {
            1
        };

        let rbx_buf_end = rsi_buf.wrapping_add((r12_len as usize) << 5);
        let span_items = ((rbx_buf_end.wrapping_sub(rsi_buf) as isize) >> 5) as u64;
        let mut span_arg = [span_items, rsi_buf as u64];
        exe::call_va!(
            0x1_41f5_b690,
            (
                rd::<usize>(param_2),
                r14_entry,
                span_arg.as_mut_ptr() as usize,
                found_in_p3 as u64
            )
        );

        let mut p = rsi_buf;
        while p != rbx_buf_end {
            crate::simd_ops::atomic_xchg_u64((p + 0x08) as u64, 0);
            p = p.wrapping_add(0x20);
        }
        if rsi_buf != 0 {
            exe::call_va!(0x1_4241_c190, (rsi_buf));
        }

        r13_ctrl = r13_ctrl.wrapping_add(1);
        r14_entry = r14_entry.wrapping_add(0x18);
        let (next_ctrl, next_entry) = skip_ctrl_neg(r13_ctrl, r14_entry);
        r13_ctrl = next_ctrl;
        r14_entry = next_entry;
    }

    exe::call_va!(0x1_401d_e830, (r15_p3_30));
    if vec_30_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec_30_ptr));
    }
}

#[inline(always)]
unsafe fn call_141eb13e0(rcx: u64, rdx: u64, f32_bits: u32, r9: u64) {
    let xmm = [0, 0, f32_bits as u128, 0];
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::lookup_native_va(0x1_41eb_13e0)
            .unwrap_or_else(|| exe::to_mapped_addr(0x1_41eb_13e0));
        crate::simd_ops::set_tls_xmm_in(xmm);
        let f: unsafe extern "win64" fn(u64, u64, f32, u64) = core::mem::transmute(addr);
        f(rcx, rdx, f32::from_bits(f32_bits), r9);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_41eb_13e0,
            rcx,
            rdx,
            f32_bits as u64,
            r9,
            0,
            0,
            0,
            0,
            xmm,
        );
    }
}

#[inline(always)]
unsafe fn rd_acq_u32(addr: usize) -> u32 {
    if (addr & 3) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU32)).load(core::sync::atomic::Ordering::Acquire)
    } else {
        let v = rd::<u32>(addr);
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn rd_acq_usize(addr: usize) -> usize {
    if (addr & 7) == 0 {
        (*(addr as *const core::sync::atomic::AtomicUsize)).load(core::sync::atomic::Ordering::Acquire)
    } else {
        let v = rd::<usize>(addr);
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn wr_rel_u64(addr: usize, val: u64) {
    if (addr & 7) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU64)).store(val, core::sync::atomic::Ordering::Release);
    } else {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
        wr::<u64>(addr, val);
    }
}

/// `unknown_141eb1010`
/// Original: `0x141eb1010` (951 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_141eb1010(param_1: usize, param_2: usize) {
    let rdi_global = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let r12 = rd::<u64>(param_1 + 0x18);
    let r9_bytes = r12.wrapping_mul(0x2c);
    let r14_size = r9_bytes.wrapping_add(4);

    let mut state_80: [u64; 4];
    if r14_size < 0xffffc0 {
        let rdi = rd::<usize>(rdi_global + 0x168);
        let mut rsi: usize = 0;
        if r14_size != 0 {
            exe::call_iat!(0x1_4264_55a8, (rdi + 0x18));
            let mut rdx = rd_acq_usize(rdi + 0x20);
            let r9_aligned = (r14_size.wrapping_add(7)) & !7u64;
            rsi = rd_acq_usize(rdi + 0x28);
            let mut r8 = rsi.wrapping_add(8).wrapping_add(r9_aligned as usize);
            if r8 > rd::<usize>(rdi + 0x10) {
                while rsi <= rdx && r8 > rdx && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
                    core::hint::spin_loop();
                    rdx = rd_acq_usize(rdi + 0x20);
                }
                if rsi <= rd::<usize>(rdi + 0x10).wrapping_sub(8) {
                    wr_rel_u64(rsi, 0x0000_00ff_ffff_ffff);
                }
                rsi = rd::<usize>(rdi + 0x08);
                r8 = rsi.wrapping_add(8).wrapping_add(r9_aligned as usize);
            }
            while rsi <= rdx && r8 > rdx && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
                core::hint::spin_loop();
                rdx = rd_acq_usize(rdi + 0x20);
            }
            wr_rel_u64(rsi, 0x0000_69fa_fafa_faff);
            crate::simd_ops::atomic_xchg_u64((rdi + 0x28) as u64, r8 as u64);
            exe::call_iat!(0x1_4264_5598, (rdi + 0x18));
            rsi = rsi.wrapping_add(8);
        }
        let hdr = ((r12 as u32) << 9) | 0x57;
        wr::<u32>(rsi, hdr);
        let stream_start = rsi.wrapping_add(4) as u64;
        state_80 = [rsi as u64, stream_start, stream_start, r14_size];
    } else {
        let alloc_ptr = exe::call_va!(0x1_4020_c080, (param_1, param_2, 0u64, r9_bytes) -> u64);
        let rdi = rd::<usize>(rdi_global + 0x168);
        exe::call_iat!(0x1_4264_55a8, (rdi + 0x18));
        let mut rdx = rd_acq_usize(rdi + 0x20);
        let mut rsi = rd_acq_usize(rdi + 0x28);
        let mut r8 = rsi.wrapping_add(0x20);
        if r8 > rd::<usize>(rdi + 0x10) {
            while rsi <= rdx && r8 > rdx && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
                core::hint::spin_loop();
                rdx = rd_acq_usize(rdi + 0x20);
            }
            if rsi <= rd::<usize>(rdi + 0x10).wrapping_sub(8) {
                wr_rel_u64(rsi, 0x0000_00ff_ffff_ffff);
            }
            rsi = rd::<usize>(rdi + 0x08);
            r8 = rsi.wrapping_add(0x20);
        }
        while rsi <= rdx && r8 > rdx && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
            core::hint::spin_loop();
            rdx = rd_acq_usize(rdi + 0x20);
        }
        wr_rel_u64(rsi, 0x0000_69fa_fafa_faff);
        crate::simd_ops::atomic_xchg_u64((rdi + 0x28) as u64, r8 as u64);
        exe::call_iat!(0x1_4264_5598, (rdi + 0x18));
        let rcx_hdr = rsi.wrapping_add(8);
        wr::<u32>(rcx_hdr + 0x04, 0x57);
        wr::<u64>(rcx_hdr + 0x0c, alloc_ptr);
        wr::<u32>(rcx_hdr, ((r12 as u32) << 9) | 1);
        state_80 = [rcx_hdr as u64, alloc_ptr, alloc_ptr, 0x14];
    }

    if r12 != 0 {
        let mut r15: u64 = 0;
        let mut rbx_off: u64 = 0;
        while r15 < r12 {
            let mut ctx_p1: [u64; 8] = [
                rd::<u64>(param_1 + 0x20),
                rd::<u64>(param_1 + 0x28),
                rd::<u64>(param_1 + 0x30),
                rd::<u64>(param_1 + 0x38),
                rd::<u64>(param_1 + 0x40),
                rd::<u64>(param_1 + 0x48),
                rd::<u64>(param_1 + 0x08),
                r15,
            ];
            let rdx_arg = rd::<u64>(param_1 + 0x50).wrapping_add(rbx_off);
            let f32_bits = rd::<u32>(param_2);
            call_141eb13e0(
                ctx_p1.as_mut_ptr() as u64,
                rdx_arg,
                f32_bits,
                state_80.as_mut_ptr() as u64,
            );
            r15 = r15.wrapping_add(1);
            rbx_off = rbx_off.wrapping_add(0x10);
        }
    }

    let r9_global = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let r8_hdr = state_80[0] as usize;
    let diff = state_80[2].wrapping_sub(state_80[1]) as i64;
    let prod_hi = ((diff as i128) * (0x2e8b_a2e8_ba2e_8ba3u64 as i64 as i128)) >> 64;
    let sar3 = (prod_hi as i64) >> 3;
    let written_cnt = (sar3.wrapping_add(((sar3 as u64) >> 63) as i64)) as u32;
    let old_hdr = rd::<u32>(r8_hdr);
    wr::<u32>(r8_hdr, (written_cnt << 9) | (old_hdr & 0x1ff));
    let ring = rd::<usize>(r9_global + 0x168);
    wr_rel_u64(r8_hdr.wrapping_sub(8), state_80[3]);
    crate::simd_ops::atomic_add_u32((ring + 0x30) as u64, 1);
    if rd::<u8>(r9_global + 0x198) != 0 {
        exe::call_iat!(
            0x1_4264_5580,
            (rd::<u64>(r9_global + 0x1a0), 1u64, 0u64)
        );
    }
}

/// `unknown_141ec57f0`
/// Original: `0x141ec57f0` (191 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_141ec57f0(param_1: usize) {
    let flag_c0e = rd::<u8>(exe::ptr::<u8>(0x1_43a2_8c0e) as usize);
    let cfg_val = if flag_c0e != 0 {
        rd::<i32>(exe::ptr::<i32>(0x1_4385_37f8) as usize)
    } else {
        rd::<i32>(exe::ptr::<i32>(0x1_4385_37d8) as usize)
    };
    let rbp = rd::<usize>(param_1 + 0x08);
    let r8 = ((cfg_val as i64) as u64).wrapping_shl(10);
    let rbx = rd::<usize>(rbp + 0x18);
    let mut dl = rd::<u8>(rbx + 0x21);
    let rsi = if dl != 0 {
        rd::<u64>(rbx + 0x40).wrapping_sub(rd::<u64>(rbx + 0x48))
    } else {
        0
    };
    let v48 = rd::<u64>(rbx + 0x48);
    let rcx_new = if r8 > v48 { 0 } else { v48.wrapping_sub(r8) };
    wr::<u64>(rbx + 0x40, rcx_new);
    let mut rdi: u64 = 0;
    if rcx_new <= rd::<u64>(rbx + 0x48) {
        loop {
            let rax = exe::call_va!(0x1_4210_c770, (rbp) -> u64);
            dl = rd::<u8>(rbx + 0x21);
            rdi = rdi.wrapping_add(rax);
            if dl == 0 || rd::<u64>(rbx + 0x40) > rd::<u64>(rbx + 0x48) {
                break;
            }
        }
    }
    if dl != 0 {
        let mut rax = rd::<u64>(rbx + 0x48).wrapping_add(rdi).wrapping_add(rsi);
        if (rax as i64) < 0 {
            rax = 0;
        }
        wr::<u64>(rbx + 0x40, rax);
    }
}

/// `unknown_141ec6cc0`
/// Original: `0x141ec6cc0` (120 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_141ec6cc0(param_1: usize, param_2: usize) {
    let p1_0 = rd::<usize>(param_1);
    let p1_1 = rd::<usize>(param_1 + 0x08);
    let p1_2 = rd::<usize>(param_1 + 0x10);
    let elem = p1_0.wrapping_add(p1_2 << 5);
    if (rd::<u8>(elem + 0x1c) & 0x30) != 0 {
        let mut arg3: u8 = 0;
        let id = rd::<u64>(p1_1.wrapping_add(p1_2 * 8).wrapping_add(0x10));
        let mut arg2: [u64; 2] = [id, 0];
        exe::call_va!(
            0x1_4105_b9b0,
            (
                param_2,
                arg2.as_mut_ptr() as usize,
                (&mut arg3 as *mut u8) as usize
            )
        );
    }
}

/// `unknown_141ec7170`
/// Original: `0x141ec7170` (1489 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_141ec7170(param_1: usize, param_2: usize) {
    let rdx_tbl = rd::<usize>(param_1 + 0x580e0);
    let id_658 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5658) as usize) as usize;
    let local_50 = rd::<usize>(rdx_tbl.wrapping_add(id_658 * 8));
    let id_afc = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5afc) as usize) as usize;
    let local_58 = rd::<u64>(rdx_tbl.wrapping_add(id_afc * 8));
    let id_eec = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceec) as usize) as usize;
    let local_60 = rd::<u64>(rdx_tbl.wrapping_add(id_eec * 8));
    let id_6e4 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_56e4) as usize) as usize;
    let r12_val = rd::<u64>(rdx_tbl.wrapping_add(id_6e4 * 8));
    let r13 = rd::<usize>(local_50 + 0x08);

    let mut res_f0: usize = param_1;
    exe::call_va!(0x1_41ef_4550, (r13, (&mut res_f0 as *mut usize) as usize));
    let r15 = res_f0;
    let r15_0 = rd::<usize>(r15);
    let id_b28 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5b28) as usize) as usize;
    let lut_20 = rd::<usize>(r15_0 + 0x20);
    let idx_b28 = rd::<u16>(lut_20.wrapping_add(id_b28 * 2)) as usize;
    let r15_8 = rd::<usize>(r15 + 0x08);
    let span_len = rd::<u32>(r15_8.wrapping_add(idx_b28 * 16).wrapping_add(0x08)) as u64;
    let span_ptr = rd::<u64>(r15_8.wrapping_add(idx_b28 * 16));
    let span: [u64; 2] = [span_len, span_ptr];

    let r11_base = r15.wrapping_add(0x40);
    let mut span_arg1 = span;
    let mut iter_init1 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init1.as_mut_ptr() as usize,
            r11_base,
            span_arg1.as_mut_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 12];
    outer[0] = r11_base as u64;
    outer[1] = iter_init1[0];
    outer[2] = iter_init1[1];
    outer[3] = iter_init1[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(r11_base.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(r11_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4105_7fd0, (outer.as_mut_ptr() as usize));

    while outer[6] < outer[5] {
        let mut rbx = outer[6] as usize;
        let rsi = outer[5] as usize;
        let r11_arr = outer[7] as usize;
        let r14_arr = outer[8] as usize;
        while rbx < rsi {
            let rcx = rd::<usize>(r11_arr.wrapping_add(rbx * 8));
            let off = rd::<u32>(rcx + 0x14).wrapping_add(rd::<u32>(rcx + 0x10)) as usize;
            let mut rdx = rcx.wrapping_add(off);
            if rd::<u8>(rdx) == 0x40 {
                rdx = rdx.wrapping_add(1);
            }
            let mut r10: usize = 0x1f;
            let mut r9 = exe::ptr::<u8>(0x1_428d_b250) as usize;
            let tbl = exe::ptr::<u8>(0x1_427e_54e0) as usize;
            let matched = loop {
                let b_str = rd::<u8>(rdx) as usize;
                rdx = rdx.wrapping_add(1);
                let r8d = rd::<u8>(tbl + b_str);
                let b_pat = rd::<u8>(r9) as usize;
                r9 = r9.wrapping_add(1);
                let eax = rd::<u8>(tbl + b_pat);
                r10 -= 1;
                if r10 == 0 || r8d == 0 || r8d != eax {
                    break r8d == eax;
                }
            };
            if matched {
                let off2 = rd::<u32>(rcx + 0x14).wrapping_add(rd::<u32>(rcx + 0x10)) as usize;
                let mut rdx2 = rcx.wrapping_add(off2);
                if rd::<u8>(rdx2) == 0x40 {
                    rdx2 = rdx2.wrapping_add(1);
                }
                let mut local_rbp = [0u8; 0xb0];
                exe::call_va!(0x1_41ec_0dd0, (local_rbp.as_mut_ptr() as usize, rdx2));
                let r9_val = rd::<u64>(r14_arr.wrapping_add(rbx * 8));
                exe::call_va!(
                    0x1_41ec_2d20,
                    (r13, r12_val, local_rbp.as_mut_ptr() as usize, r9_val)
                );
                exe::call_va!(0x1_4240_ab10, (local_rbp.as_mut_ptr() as usize));
            }
            rbx += 1;
            outer[6] = rbx as u64;
        }

        let mut cur_arch = (outer[2] as usize).wrapping_add(2);
        outer[2] = cur_arch as u64;
        let end_arch = outer[3] as usize;
        if cur_arch != end_arch {
            let iter_base = outer[1] as usize;
            while cur_arch != end_arch {
                let a_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_base.wrapping_add(a_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
            }
            outer[2] = cur_arch as u64;
            if cur_arch != end_arch {
                let base_0 = outer[0] as usize;
                let a_id = rd::<u16>(cur_arch) as usize;
                let cp = rd::<u64>(base_0.wrapping_add(a_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(a_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_4105_7fd0, (outer.as_mut_ptr() as usize));
    }

    let budget_sec: f64 = if rd::<u8>(exe::ptr::<u8>(0x1_43a2_8c0e) as usize) == 0 {
        rd::<f64>(exe::ptr::<f64>(0x1_4342_6b90) as usize)
    } else {
        rd::<f64>(exe::ptr::<f64>(0x1_4342_68a0) as usize)
    };

    let mut qpc_start_raw: i64 = 0;
    exe::call_iat!(0x1_4264_56f8, ((&mut qpc_start_raw as *mut i64) as usize));
    let freq_init = exe::call_va!(0x1_4240_6920, () -> i64);
    let start_qpc = qpc_start_raw.wrapping_sub(((freq_init as f64) * 0.0f64) as i64);

    let mut span_arg2 = span;
    let mut iter_init2 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init2.as_mut_ptr() as usize,
            r11_base,
            span_arg2.as_mut_ptr() as usize
        ) -> usize
    );

    outer[0] = r11_base as u64;
    outer[1] = iter_init2[0];
    outer[2] = iter_init2[1];
    outer[3] = iter_init2[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(r11_base.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(r11_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4105_7fd0, (outer.as_mut_ptr() as usize));

    if outer[6] < outer[5] {
        let r13_buf1 = param_2.wrapping_add(0x158);
        let r12_buf2 = param_2.wrapping_add(0x170);
        let mut r14_cnt = outer[5] as usize;
        let mut rsi = outer[6] as usize;
        loop {
            let rdx_arg6 = (outer[7] as usize).wrapping_add(rsi * 8);
            let val_70 = rd::<u64>((outer[8] as usize).wrapping_add(rsi * 8));
            let rcx_arg5 = (outer[9] as usize).wrapping_add(rsi << 5);
            wr::<u64>(rcx_arg5 + 0x10, val_70);
            exe::call_va!(
                0x1_41ec_85a0,
                (local_50, local_58, local_60, r12_val, rcx_arg5, rdx_arg6)
            );

            let chunk_ptr = outer[4] as usize;
            let ent_id = rd::<u64>(chunk_ptr.wrapping_add(rsi * 8).wrapping_add(0x10));
            let prev_id = rd::<u64>(param_2 + 0x58);
            let inc = if ent_id != prev_id { 1u64 } else { 0u64 };
            wr::<u64>(param_2 + 0x60, rd::<u64>(param_2 + 0x60).wrapping_add(inc));
            wr::<u64>(param_2 + 0x58, ent_id);

            let bx_val = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5ccc) as usize);

            let p_160 = rd::<usize>(r13_buf1 + 0x08);
            if p_160 == 0 || rd::<u64>(p_160 + 0x08).wrapping_add(1) > 0x4000 {
                exe::call_va!(0x1_4020_e8f0, (r13_buf1));
            }
            let p_160 = rd::<usize>(r13_buf1 + 0x08);
            let idx_160 = rd::<usize>(p_160 + 0x08);
            wr::<u32>(
                p_160.wrapping_add(idx_160 * 4).wrapping_add(0x10),
                ent_id as u32,
            );
            let p_160 = rd::<usize>(r13_buf1 + 0x08);
            wr::<u64>(p_160 + 0x08, rd::<u64>(p_160 + 0x08).wrapping_add(1));
            wr::<u64>(r13_buf1 + 0x10, rd::<u64>(r13_buf1 + 0x10).wrapping_add(1));

            let p_178 = rd::<usize>(r12_buf2 + 0x08);
            if p_178 == 0 || rd::<u64>(p_178 + 0x08).wrapping_add(1) > 0x8000 {
                exe::call_va!(0x1_4020_e8f0, (r12_buf2));
            }
            let p_178 = rd::<usize>(r12_buf2 + 0x08);
            let idx_178 = rd::<usize>(p_178 + 0x08);
            wr::<u16>(p_178.wrapping_add(idx_178 * 2).wrapping_add(0x10), bx_val);
            let p_178 = rd::<usize>(r12_buf2 + 0x08);
            wr::<u64>(p_178 + 0x08, rd::<u64>(p_178 + 0x08).wrapping_add(1));
            wr::<u64>(r12_buf2 + 0x10, rd::<u64>(r12_buf2 + 0x10).wrapping_add(1));

            let mut cur_qpc: i64 = 0;
            exe::call_iat!(0x1_4264_56f8, ((&mut cur_qpc as *mut i64) as usize));
            let elapsed = cur_qpc.wrapping_sub(start_qpc) as f64;
            let cur_freq = exe::call_va!(0x1_4240_6920, () -> i64) as f64;
            if (elapsed / cur_freq) > budget_sec {
                break;
            }

            rsi += 1;
            outer[6] = rsi as u64;
            if rsi >= r14_cnt {
                let mut cur_arch = (outer[2] as usize).wrapping_add(2);
                outer[2] = cur_arch as u64;
                let end_arch = outer[3] as usize;
                if cur_arch != end_arch {
                    let iter_base = outer[1] as usize;
                    loop {
                        let a_id = rd::<u16>(cur_arch) as usize;
                        if rd::<u64>(iter_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                            && rd::<u32>(iter_base.wrapping_add(a_id * 4).wrapping_add(0x10048))
                                != 0
                        {
                            break;
                        }
                        cur_arch = cur_arch.wrapping_add(2);
                        outer[2] = cur_arch as u64;
                        if cur_arch == end_arch {
                            break;
                        }
                    }
                    if cur_arch != end_arch {
                        let base_0 = outer[0] as usize;
                        let a_id = rd::<u16>(cur_arch) as usize;
                        let cp = rd::<u64>(base_0.wrapping_add(a_id * 8).wrapping_add(0x48));
                        outer[4] = cp;
                        outer[5] = if cp != 0 {
                            rd::<u32>(base_0.wrapping_add(a_id * 4).wrapping_add(0x10048)) as u64
                        } else {
                            0
                        };
                        outer[6] = 0;
                    }
                }
                exe::call_va!(0x1_4105_7fd0, (outer.as_mut_ptr() as usize));
                rsi = outer[6] as usize;
                r14_cnt = outer[5] as usize;
                if rsi >= r14_cnt {
                    break;
                }
            }
        }
    }

    let rax_78 = rd::<usize>(r13 + 0x78);
    wr::<u64>(rax_78, 0);
    wr::<u64>(rax_78 + 0x08, 0);
    wr::<u64>(rax_78 + 0x10, 0);
    wr::<u64>(rax_78 + 0x20, 0x100);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141e59da0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    #[repr(align(16))]
    struct Arena([u8; 0x1000]);
    static mut ARENA: Arena = Arena([0; 0x1000]);
    static mut ARENA_OFF: usize = 0;
    static mut STREAM0_PTR: usize = 0;
    static mut STREAM1_PTR: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_iter_begin(out: usize, r14: u64, span: usize) -> usize {
        wr::<u64>(out, r14);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        unsafe {
            let base = core::ptr::addr_of_mut!(ARENA.0) as usize;
            let off = ARENA_OFF;
            let aligned = ((sz as usize) + 15) & !15;
            ARENA_OFF = off + aligned.max(16);
            base + off
        }
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        unsafe { rec(0xc190 ^ ((ptr != 0) as u64)) };
    }
    unsafe extern "win64" fn mock_fb7180(outer: usize) {
        if rd::<u64>(outer + 0x30) < rd::<u64>(outer + 0x28) {
            wr::<usize>(outer + 0x38, unsafe { STREAM0_PTR });
            wr::<usize>(outer + 0x40, unsafe { STREAM1_PTR });
        }
    }
    unsafe extern "win64" fn mock_sort(begin_ptr: usize, _end_ptr: usize, count: u64, _z: u64) {
        let base = rd::<usize>(begin_ptr);
        let n = count as usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = rd::<u64>(base + i * 8);
                let b = rd::<u64>(base + j * 8);
                if key_lt_3fff(b, a) {
                    wr::<u64>(base + i * 8, b);
                    wr::<u64>(base + j * 8, a);
                }
            }
        }
    }
    unsafe extern "win64" fn mock_b13950(src_begin: usize, src_end: usize, dst: usize) {
        let mut s = src_begin;
        let mut d = dst;
        while s != src_end {
            wr::<[u64; 4]>(d, rd::<[u64; 4]>(s));
            s += 0x20;
            d += 0x20;
        }
    }
    unsafe extern "win64" fn mock_1e84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
    }
    unsafe extern "win64" fn mock_f5b690(ctx: usize, entry: usize, span: usize, found: u8) {
        let cnt = rd::<u64>(span);
        let ptr = rd::<usize>(span + 0x08);
        let first_id = if cnt > 0 { rd::<u64>(ptr) } else { 0 };
        let first_b18 = if cnt > 0 { rd::<u8>(ptr + 0x18) as u64 } else { 0 };
        unsafe {
            rec(
                0xb690
                    ^ (ctx as u64)
                    ^ rd::<u64>(entry)
                    ^ (cnt << 16)
                    ^ first_id
                    ^ (first_b18 << 24)
                    ^ (found as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_1de830(v: usize) {
        unsafe { rec(0xe830 ^ (rd::<u32>(v + 0x08) as u64)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40fb_7180, mock_fb7180 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40fb_9d30, mock_sort as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40b1_3950, mock_b13950 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401e_84a0, mock_1e84a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41f5_b690, mock_f5b690 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401d_e830, mock_1de830 as *const ())?;

    let run = |f: F| -> (u64, [u64; 2]) {
        unsafe {
            ARENA.0 = [0; 0x1000];
            ARENA_OFF = 0;
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        // 3 items in archetype 0: keys 0x0005, 0x0002 (skipped by flag=2), 0x0001
        let mut s0 = [0u64; 9];
        s0[0] = 0x1000_0005;
        s0[3] = 0x1000_0002;
        s0[6] = 0x1000_0001;
        let s1: [u8; 3] = [1, 2, 0];
        unsafe {
            STREAM0_PTR = s0.as_mut_ptr() as usize;
            STREAM1_PTR = s1.as_ptr() as usize;
        }

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<u64>(wp + 0x88, 0x1234);
        wr::<u32>(wp + 0x10088, 3);

        let arch_list: [u16; 1] = [0];
        let p1: [u64; 3] = [
            wp as u64,
            arch_list.as_ptr() as u64,
            unsafe { arch_list.as_ptr().add(1) } as u64,
        ];
        let p2: [u64; 1] = [0x7777];

        // Sub-entries for valid hash slot: one present in vec_30 (0x1000_0001), one absent (0x1000_0003)
        let mut sub_items = [0u8; 0x40];
        let sp = sub_items.as_mut_ptr() as usize;
        wr::<u64>(sp, 0x1000_0001);
        wr::<u64>(sp + 0x08, 0xaaaa);
        wr::<u8>(sp + 0x18, 7);
        wr::<u64>(sp + 0x20, 0x1000_0003);
        wr::<u64>(sp + 0x28, 0xbbbb);
        wr::<u8>(sp + 0x38, 9);

        // Two hash slots: slot 0 is empty (-128 / 0x80), slot 1 is occupied (0), slot 2 is end sentinel (0xff = -1)
        let mut ctrl = [0x80u8; 32];
        ctrl[0] = 0x80;
        ctrl[1] = 0x00;
        ctrl[2] = 0xff;

        let mut entries = [0u64; 6];
        // slot 1 (at +0x18 = entries[3..6])
        entries[3] = 0x2000_000a;
        entries[4] = sp as u64;
        entries[5] = 2;

        let mut p3_sort_data: [u64; 2] = [0x2000_000f, 0x2000_000a];
        let mut p3 = [0u8; 0x48];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p, ctrl.as_ptr() as usize);
        wr::<usize>(p3p + 0x08, entries.as_ptr() as usize);
        wr::<usize>(p3p + 0x18, 2);
        wr::<usize>(p3p + 0x30, p3_sort_data.as_mut_ptr() as usize);
        wr::<u32>(p3p + 0x38, 2);

        f(p1.as_ptr() as usize, p2.as_ptr() as usize, p3p);
        (unsafe { TRACE_HASH }, p3_sort_data)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e59da0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eb1010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_13e0(p1: usize, p2: usize, _r8: usize, p4: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        let f_bits = xmm[2] as u32;
        let idx = rd::<u64>(p1 + 0x38);
        let out_cur = rd::<usize>(p4 + 0x10);
        wr::<u32>(out_cur, (idx as u32).wrapping_add(0x100));
        wr::<u32>(out_cur + 0x04, f_bits);
        wr::<u64>(out_cur + 0x08, rd::<u64>(p2));
        wr::<usize>(p4 + 0x10, out_cur + 0x2c);
        unsafe {
            TRACE_HASH = (TRACE_HASH
                ^ idx
                ^ (f_bits as u64)
                ^ rd::<u64>(p1)
                ^ rd::<u64>(p1 + 0x30))
            .wrapping_mul(0x1000_0000_01b3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_41eb_13e0, mock_13e0 as *const ())?;

    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_f58 = rd::<usize>(g_f58);

    let run = |f: F, wrap_around: bool| -> (u64, Vec<u8>, usize, u32) {
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }
        let mut ring_mem = vec![0u8; 0x200];
        let rm_start = ring_mem.as_mut_ptr() as usize;
        let rm_end = rm_start + 0x180;
        let init_write = if wrap_around {
            rm_end - 0x10
        } else {
            rm_start
        };

        let mut ring_struct = [0u8; 0x40];
        let rdi = ring_struct.as_mut_ptr() as usize;
        wr::<usize>(rdi + 0x08, rm_start);
        wr::<usize>(rdi + 0x10, rm_end);
        wr::<usize>(rdi + 0x20, rm_start);
        wr::<usize>(rdi + 0x28, init_write);
        wr::<u32>(rdi + 0x30, 5);
        wr::<u32>(rdi + 0x34, 5);

        let mut global_ctx = [0u8; 0x1b0];
        let gctx = global_ctx.as_mut_ptr() as usize;
        wr::<usize>(gctx + 0x168, rdi);
        wr::<u8>(gctx + 0x198, 0);
        wr::<usize>(g_f58, gctx);

        let items_50: [u64; 4] = [0xaaaa_1111, 0, 0xbbbb_2222, 0];
        let mut p1 = [0u8; 0x58];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x08, 0x9999);
        wr::<u64>(p1p + 0x18, 2);
        wr::<u64>(p1p + 0x20, 0x11);
        wr::<u64>(p1p + 0x28, 0x22);
        wr::<u64>(p1p + 0x30, 0x33);
        wr::<u64>(p1p + 0x38, 0x44);
        wr::<u64>(p1p + 0x40, 0x55);
        wr::<u64>(p1p + 0x48, 0x66);
        wr::<usize>(p1p + 0x50, items_50.as_ptr() as usize);

        let param2_f32: f32 = 0.75;
        f(p1p, (&param2_f32 as *const f32) as usize);

        let new_write_off = rd::<usize>(rdi + 0x28).wrapping_sub(rm_start);
        (
            unsafe { TRACE_HASH },
            ring_mem,
            new_write_off,
            rd::<u32>(rdi + 0x30),
        )
    };

    let o = (run(f_o, false), run(f_o, true));
    let n = (run(f_n, false), run(f_n, true));
    wr::<usize>(g_f58, s_f58);
    crate::diff_eq!(o, n, "141eb1010 out");
    Ok(2)
}

pub unsafe fn diff_unknown_141ec57f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_RBX: usize = 0;

    unsafe extern "win64" fn mock_c770(_rbp: usize) -> u64 {
        let rbx = unsafe { TARGET_RBX };
        wr::<u64>(rbx + 0x40, rd::<u64>(rbx + 0x40).wrapping_add(0x500));
        0x300
    }

    let _m1 = crate::difftest::Mock::new(0x1_4210_c770, mock_c770 as *const ())?;

    let g_c0e = exe::ptr::<u8>(0x1_43a2_8c0e) as usize;
    let g_7d8 = exe::ptr::<i32>(0x1_4385_37d8) as usize;
    let g_7f8 = exe::ptr::<i32>(0x1_4385_37f8) as usize;
    let s_c0e = rd::<u8>(g_c0e);
    let s_7d8 = rd::<i32>(g_7d8);
    let s_7f8 = rd::<i32>(g_7f8);

    let run = |f: F, use_alt: u8, active: u8| -> (u64, u64) {
        wr::<u8>(g_c0e, use_alt);
        wr::<i32>(g_7d8, 2);
        wr::<i32>(g_7f8, 1);

        let mut rbx_buf = [0u8; 0x50];
        let rbx = rbx_buf.as_mut_ptr() as usize;
        wr::<u8>(rbx + 0x21, active);
        wr::<u64>(rbx + 0x40, 0x1200);
        wr::<u64>(rbx + 0x48, 0x1000);
        unsafe {
            TARGET_RBX = rbx;
        }

        let mut rbp_buf = [0u8; 0x20];
        let rbp = rbp_buf.as_mut_ptr() as usize;
        wr::<usize>(rbp + 0x18, rbx);

        let p1: [usize; 2] = [0, rbp];
        f(p1.as_ptr() as usize);
        (rd::<u64>(rbx + 0x40), rd::<u64>(rbx + 0x48))
    };

    let o = (run(f_o, 0, 1), run(f_o, 1, 1), run(f_o, 0, 0));
    let n = (run(f_n, 0, 1), run(f_n, 1, 1), run(f_n, 0, 0));
    wr::<u8>(g_c0e, s_c0e);
    wr::<i32>(g_7d8, s_7d8);
    wr::<i32>(g_7f8, s_7f8);
    crate::diff_eq!(o, n, "141ec57f0 out");
    Ok(3)
}

pub unsafe fn diff_unknown_141ec6cc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_b9b0(p2: usize, a2: usize, a3: usize) {
        wr::<u64>(p2, rd::<u64>(a2));
        wr::<u8>(p2 + 0x08, rd::<u8>(a2 + 0x08).wrapping_add(1));
        wr::<u8>(p2 + 0x09, rd::<u8>(a3).wrapping_add(2));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4105_b9b0, mock_b9b0 as *const ())?;

    let run = |f: F, mask: u8| -> (u64, u8, u8) {
        let mut arr0 = [0u8; 0x40];
        arr0[0x20 + 0x1c] = mask;

        let arr1: [u64; 4] = [0, 0, 0x1111_2222, 0xdead_beef_cafe_babe];
        let p1: [usize; 3] = [arr0.as_ptr() as usize, arr1.as_ptr() as usize, 1];
        let mut out_p2 = [0u8; 16];
        f(p1.as_ptr() as usize, out_p2.as_mut_ptr() as usize);
        let p2p = out_p2.as_ptr() as usize;
        (rd::<u64>(p2p), rd::<u8>(p2p + 8), rd::<u8>(p2p + 9))
    };

    let o = (run(f_o, 0x20), run(f_o, 0x01));
    let n = (run(f_n, 0x20), run(f_n, 0x01));
    crate::diff_eq!(o, n, "141ec6cc0 out");
    Ok(2)
}

pub unsafe fn diff_unknown_141ec7170(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut WORLD_PTR: usize = 0;
    static mut STREAM_68: usize = 0;
    static mut STREAM_70: usize = 0;
    static mut STREAM_78: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_ef4550(_r13: usize, io_ptr: usize) {
        wr::<usize>(io_ptr, unsafe { WORLD_PTR });
    }
    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, span: usize) -> usize {
        let cnt = rd::<u64>(span) as usize;
        let ptr = rd::<u64>(span + 0x08);
        wr::<u64>(out, r11);
        wr::<u64>(out + 0x08, ptr);
        wr::<u64>(out + 0x10, ptr.wrapping_add((cnt * 2) as u64));
        out
    }
    unsafe extern "win64" fn mock_057fd0(outer: usize) {
        if rd::<u64>(outer + 0x30) < rd::<u64>(outer + 0x28) {
            wr::<usize>(outer + 0x38, unsafe { STREAM_68 });
            wr::<usize>(outer + 0x40, unsafe { STREAM_70 });
            wr::<usize>(outer + 0x48, unsafe { STREAM_78 });
        }
    }
    unsafe extern "win64" fn mock_ec0dd0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
    }
    unsafe extern "win64" fn mock_ec2d20(r13: usize, r12: u64, rbp_ptr: usize, r9_val: u64) {
        let _ = r13;
        unsafe { rec(0x2d20 ^ r12 ^ rd::<u64>(rbp_ptr) ^ r9_val) };
    }
    unsafe extern "win64" fn mock_40ab10(_p: usize) {
        unsafe { rec(0xab10) };
    }
    unsafe extern "win64" fn mock_406920() -> i64 {
        1_000_000_000_000_000
    }
    unsafe extern "win64" fn mock_ec85a0(
        a1: usize,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: usize,
        a6: usize,
    ) {
        let _ = a1;
        unsafe {
            rec(0x85a0 ^ a2 ^ a3 ^ a4 ^ rd::<u64>(a5 + 0x10) ^ rd::<u64>(a6));
        }
    }
    unsafe extern "win64" fn mock_20e8f0(_p: usize) {
        unsafe { rec(0xe8f0) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_41ef_4550, mock_ef4550 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4105_7fd0, mock_057fd0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41ec_0dd0, mock_ec0dd0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41ec_2d20, mock_ec2d20 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_ab10, mock_40ab10 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_6920, mock_406920 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41ec_85a0, mock_ec85a0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4020_e8f0, mock_20e8f0 as *const ())?;

    let g_658 = exe::ptr::<u16>(0x1_43cd_5658) as usize;
    let g_afc = exe::ptr::<u16>(0x1_43cd_5afc) as usize;
    let g_eec = exe::ptr::<u16>(0x1_43c8_ceec) as usize;
    let g_6e4 = exe::ptr::<u16>(0x1_43cd_56e4) as usize;
    let g_b28 = exe::ptr::<u16>(0x1_43cd_5b28) as usize;
    let g_c0e = exe::ptr::<u8>(0x1_43a2_8c0e) as usize;
    let g_ccc = exe::ptr::<u16>(0x1_43cd_5ccc) as usize;

    let s_658 = rd::<u16>(g_658);
    let s_afc = rd::<u16>(g_afc);
    let s_eec = rd::<u16>(g_eec);
    let s_6e4 = rd::<u16>(g_6e4);
    let s_b28 = rd::<u16>(g_b28);
    let s_c0e = rd::<u8>(g_c0e);
    let s_ccc = rd::<u16>(g_ccc);

    wr::<u16>(g_658, 0);
    wr::<u16>(g_afc, 1);
    wr::<u16>(g_eec, 2);
    wr::<u16>(g_6e4, 3);
    wr::<u16>(g_b28, 0);
    wr::<u8>(g_c0e, 1);
    wr::<u16>(g_ccc, 0x4242);

    let run = |f: F| -> (u64, u64, u64, [u32; 2], [u16; 2], [u64; 5]) {
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let mut str_obj0 = [0u8; 0x60];
        let p_str0 = str_obj0.as_mut_ptr() as usize;
        wr::<u32>(p_str0 + 0x10, 0x10);
        wr::<u32>(p_str0 + 0x14, 0x08);
        let path0 = b"@SourceData\\lua_scripts\\systems\\test.lua\0";
        str_obj0[0x18..0x18 + path0.len()].copy_from_slice(path0);

        let mut str_obj1 = [0u8; 0x60];
        let p_str1 = str_obj1.as_mut_ptr() as usize;
        wr::<u32>(p_str1 + 0x10, 0x10);
        wr::<u32>(p_str1 + 0x14, 0x08);
        let path1 = b"other\\folder\\script.lua\0";
        str_obj1[0x18..0x18 + path1.len()].copy_from_slice(path1);

        let s68: [usize; 2] = [p_str0, p_str1];
        let s70: [u64; 2] = [0x1111_2222, 0x3333_4444];
        let mut s78 = [0u64; 8];
        unsafe {
            STREAM_68 = s68.as_ptr() as usize;
            STREAM_70 = s70.as_ptr() as usize;
            STREAM_78 = s78.as_mut_ptr() as usize;
        }

        let chunk: [u64; 4] = [0, 0, 0xaaaa_0001, 0xbbbb_0002];
        let arch_ids: [u16; 1] = [0];
        let mut r15_8 = [0u8; 0x10];
        wr::<usize>(r15_8.as_mut_ptr() as usize, arch_ids.as_ptr() as usize);
        wr::<u32>(r15_8.as_mut_ptr() as usize + 0x08, 1);

        let lut_20: [u16; 1] = [0];
        let mut r15_0 = [0u8; 0x28];
        wr::<usize>(r15_0.as_mut_ptr() as usize + 0x20, lut_20.as_ptr() as usize);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, r15_0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, r15_8.as_ptr() as usize);
        let r11 = wp + 0x40;
        wr::<usize>(r11 + 0x48, chunk.as_ptr() as usize);
        wr::<u32>(r11 + 0x10048, 2);
        unsafe {
            WORLD_PTR = wp;
        }

        let mut rax_78: [u64; 5] = [9, 9, 9, 9, 9];
        let mut r13_obj = [0u8; 0x80];
        wr::<usize>(r13_obj.as_mut_ptr() as usize + 0x78, rax_78.as_mut_ptr() as usize);
        let mut local_50_obj = [0u8; 0x10];
        wr::<usize>(local_50_obj.as_mut_ptr() as usize + 0x08, r13_obj.as_mut_ptr() as usize);

        let tbl_580e0: [u64; 4] = [
            local_50_obj.as_mut_ptr() as u64,
            0x101,
            0x202,
            0x303,
        ];
        let mut p1 = vec![0u8; 0x58100];
        wr::<usize>(p1.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

        let mut buf_160 = [0u8; 0x20];
        let mut buf_178 = [0u8; 0x20];
        let mut p2 = [0u8; 0x190];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x160, buf_160.as_mut_ptr() as usize);
        wr::<usize>(p2p + 0x178, buf_178.as_mut_ptr() as usize);

        f(p1.as_mut_ptr() as usize, p2p);

        let b160_p = buf_160.as_ptr() as usize;
        let b178_p = buf_178.as_ptr() as usize;
        (
            unsafe { TRACE_HASH },
            rd::<u64>(p2p + 0x58),
            rd::<u64>(p2p + 0x60),
            [rd::<u32>(b160_p + 0x10), rd::<u32>(b160_p + 0x14)],
            [rd::<u16>(b178_p + 0x10), rd::<u16>(b178_p + 0x12)],
            rax_78,
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u16>(g_658, s_658);
    wr::<u16>(g_afc, s_afc);
    wr::<u16>(g_eec, s_eec);
    wr::<u16>(g_6e4, s_6e4);
    wr::<u16>(g_b28, s_b28);
    wr::<u8>(g_c0e, s_c0e);
    wr::<u16>(g_ccc, s_ccc);

    crate::diff_eq!(o, n, "141ec7170 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_41e5_9da0, "unknown_141e59da0", rend::cauldron_hot_j::unknown_141e59da0 as *const (), Throws::Caught, rend::cauldron_hot_j::diff_unknown_141e59da0),
// Replacement::verified_throws(0x1_41eb_1010, "unknown_141eb1010", rend::cauldron_hot_j::unknown_141eb1010 as *const (), Throws::Discard, rend::cauldron_hot_j::diff_unknown_141eb1010),
// Replacement::verified_throws(0x1_41ec_57f0, "unknown_141ec57f0", rend::cauldron_hot_j::unknown_141ec57f0 as *const (), Throws::Discard, rend::cauldron_hot_j::diff_unknown_141ec57f0),
// Replacement::verified(0x1_41ec_6cc0, "unknown_141ec6cc0", rend::cauldron_hot_j::unknown_141ec6cc0 as *const (), rend::cauldron_hot_j::diff_unknown_141ec6cc0),
// Replacement::verified_throws(0x1_41ec_7170, "unknown_141ec7170", rend::cauldron_hot_j::unknown_141ec7170 as *const (), Throws::Caught, rend::cauldron_hot_j::diff_unknown_141ec7170),
