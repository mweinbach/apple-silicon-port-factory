//! Decompiled hot functions shard ZA05 (`0x1411dcc60..0x1414623b0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `d3d_1411dcc60`
/// Original: 0x1411dcc60 (389 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_1411dcc60(
    param_1: usize,
    _param_2: usize,
    param_3: usize,
    _param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: usize,
) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let pool = rd::<usize>(heap + 0xb0);
    let mut r15 = rd::<usize>(pool + 0x10);
    if r15 != 0 && (r15 & 0xf) == 0 {
        wr::<u32>(pool + 0x18, rd::<u32>(pool + 0x18).wrapping_add(1));
        wr::<usize>(pool + 0x10, rd::<usize>(r15));
    } else {
        r15 = exe::call_va!(0x1_4253_5070, (heap, 0xa8u64, 0x10u64) -> usize);
        if r15 == 0 {
            let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
            r15 = 0;
        }
    }

    let rbx = rd::<u64>(param_8);
    let rdi = rd::<u64>(param_7);
    let rsi = rd::<u64>(param_6);
    let r14 = rd::<u64>(param_5);

    wr::<u64>(r15, u64::MAX);
    wr::<u64>(r15 + 0x08, u64::MAX);
    wr::<u64>(r15 + 0x10, 0);
    wr::<u8>(r15 + 0x18, 0);
    wr::<u64>(r15 + 0x1c, 0);
    wr::<u64>(r15 + 0x24, 0);
    wr::<u32>(r15 + 0x38, u32::MAX);
    wr::<u32>(r15 + 0x3c, 0);
    wr::<u64>(r15 + 0x30, 0);
    wr::<u64>(r15 + 0x40, 0);
    wr::<u32>(r15 + 0x48, 0);
    wr::<u16>(r15 + 0x50, 0xff00);
    wr::<u64>(r15 + 0x60, 0);
    wr::<u64>(r15 + 0x58, 0);
    wr::<u32>(r15 + 0x68, 0);
    wr::<usize>(r15 + 0x70, param_3);
    if param_3 != 0 {
        let vtbl = rd::<usize>(param_3);
        exe::call_va!(rd::<usize>(vtbl + 0x08), (param_3));
    }
    wr::<u64>(r15 + 0x78, r14);
    wr::<u64>(r15 + 0x80, rsi);
    wr::<u64>(r15 + 0x88, rdi);
    wr::<u64>(r15 + 0x90, rbx);
    wr::<u64>(r15 + 0x98, 0);
    wr::<u64>(r15 + 0xa0, 5);

    wr::<usize>(param_1, r15);
    param_1
}

/// `unknown_1411ec200`
/// Original: 0x1411ec200 (392 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1411ec200(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> usize {
    let mut iter = [0u64; 6];
    let iter_ptr = iter.as_mut_ptr() as usize;

    let tag2 = rd::<i8>(param_2 + 0x20);
    let r9b: i8 = if tag2 == -1 {
        -1
    } else if tag2 == 0 {
        wr::<u64>(iter_ptr, rd::<u64>(param_2));
        wr::<u64>(iter_ptr + 0x08, rd::<u64>(param_2 + 0x08));
        wr::<u128>(iter_ptr + 0x10, rd::<u128>(param_2 + 0x10));
        0
    } else {
        1
    };

    let tag3 = rd::<i8>(param_3 + 0x20);
    let r8b: i8 = if tag3 == -1 {
        -1
    } else if tag3 == 0 {
        0
    } else {
        1
    };

    let mut dl: i8 = if r9b == -1 {
        -1
    } else if r9b == 0 {
        0
    } else {
        1
    };
    wr::<i8>(iter_ptr + 0x20, dl);

    if dl == 0 && r8b == 1 {
        let mut rdi = rd::<usize>(iter_ptr + 0x08);
        let mut rax = rd::<usize>(iter_ptr);
        loop {
            let count = rd::<usize>(rax);
            let base = rd::<usize>(rax + 0x08);
            let end = base.wrapping_add(count.wrapping_mul(2));
            if rdi == end {
                wr::<usize>(param_1, param_4);
                return param_1;
            }
            if dl != 0 {
                exe::call_va!(0x1_4020_0070, ());
                return param_1;
            }
            let len = rd::<u32>(param_4 + 0x08) as u64;
            let cap = rd::<u32>(param_4 + 0x0c) as u64;
            if len.wrapping_mul(2) == cap.wrapping_mul(2) {
                let need = len.wrapping_add(1);
                let grow = (cap >> 1).wrapping_add(cap);
                let new_cap = if grow < need { need } else { grow };
                exe::call_va!(0x1_40fb_3a60, (param_4, new_cap));
            }
            let cur_len = rd::<u32>(param_4 + 0x08) as usize;
            let buf = rd::<usize>(param_4);
            let val_ptr = rd::<usize>(iter_ptr + 0x10);
            wr::<u16>(buf.wrapping_add(cur_len.wrapping_mul(2)), rd::<u16>(val_ptr));
            wr::<u32>(param_4 + 0x08, (cur_len as u32).wrapping_add(1));
            rdi = rdi.wrapping_add(2);
            wr::<usize>(iter_ptr + 0x08, rdi);
            exe::call_va!(0x1_420a_3250, (iter_ptr));
            dl = rd::<i8>(iter_ptr + 0x20);
            if dl != 0 {
                break;
            }
            rdi = rd::<usize>(iter_ptr + 0x08);
            rax = rd::<usize>(iter_ptr);
        }
    }

    exe::call_va!(0x1_4020_0070, ());
    param_1
}

/// `sparsehash_1411f00d0`
/// Original: 0x1411f00d0 (305 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1411f00d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> usize {
    let data = rd::<usize>(param_1);
    let idx = (param_3.wrapping_sub(data) as isize) >> 1;
    let cap_bytes = (rd::<u32>(param_1 + 0x0c) as u64).wrapping_mul(2);
    let r13 = rd::<u32>(param_1 + 0x08) as u64;
    let len_bytes = r13.wrapping_mul(2);
    let off_bytes = (idx as usize).wrapping_mul(2);

    let out_ptr: usize;
    if len_bytes == cap_bytes {
        let new_len = r13.wrapping_add(1);
        let old_cap = ((cap_bytes as i64) >> 1) as u64;
        let grow = (old_cap >> 1).wrapping_add(old_cap);
        let new_cap = if grow < new_len { new_len } else { grow };
        let new_buf = exe::call_va!(0x1_4028_e1c0, (old_cap, new_cap) -> usize);
        if idx != 0 {
            let cur_data = rd::<usize>(param_1);
            core::ptr::copy(cur_data as *const u8, new_buf as *mut u8, off_bytes);
        }
        out_ptr = new_buf.wrapping_add(off_bytes);
        wr::<u16>(out_ptr, rd::<u16>(param_4));
        let cur_data = rd::<usize>(param_1);
        let src_tail = cur_data.wrapping_add(off_bytes);
        let cur_len_bytes = (rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(2);
        let tail_bytes = cur_len_bytes
            .wrapping_sub(src_tail)
            .wrapping_add(cur_data);
        if tail_bytes != 0 {
            core::ptr::copy(
                src_tail as *const u8,
                out_ptr.wrapping_add(2) as *mut u8,
                tail_bytes,
            );
        }
        exe::call_va!(0x1_401d_e830, (param_1));
        wr::<usize>(param_1, new_buf);
        wr::<u32>(param_1 + 0x08, new_len as u32);
        wr::<u32>(param_1 + 0x0c, new_cap as u32);
    } else {
        out_ptr = data.wrapping_add(off_bytes);
        let end_ptr = data.wrapping_add(len_bytes as usize);
        if out_ptr == end_ptr {
            let _ = exe::call_va!(0x1_411f_0210, (param_1, param_4) -> usize);
        } else {
            wr::<u16>(end_ptr, rd::<u16>(end_ptr.wrapping_sub(2)));
            let cur_len_bytes = (rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(2);
            let cur_data = rd::<usize>(param_1);
            let move_bytes = cur_data
                .wrapping_sub(2)
                .wrapping_add(cur_len_bytes.wrapping_sub(out_ptr));
            let dst = cur_len_bytes
                .wrapping_sub(move_bytes)
                .wrapping_add(cur_data);
            if move_bytes != 0 {
                core::ptr::copy(out_ptr as *const u8, dst as *mut u8, move_bytes);
            }
            wr::<u16>(out_ptr, rd::<u16>(param_4));
            wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
        }
    }

    wr::<usize>(param_2, out_ptr);
    param_2
}

/// `sparsehash_1411f0210`
/// Original: 0x1411f0210 (85 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1411f0210(param_1: usize, param_2: usize) -> usize {
    let rdx = (rd::<u32>(param_1 + 0x08) as u64).wrapping_mul(2);
    let rax = (rd::<u32>(param_1 + 0x0c) as u64).wrapping_mul(2);
    if rdx == rax {
        let new_cap = ((rdx as i64) >> 1).wrapping_add(1) as u64;
        exe::call_va!(0x1_40fa_5d10, (param_1, new_cap));
    }
    let idx = rd::<u32>(param_1 + 0x08) as usize;
    let data = rd::<usize>(param_1);
    let val = rd::<u16>(param_2);
    wr::<u16>(data.wrapping_add(idx.wrapping_mul(2)), val);
    wr::<u32>(param_1 + 0x08, (idx as u32).wrapping_add(1));
    let new_len = rd::<u32>(param_1 + 0x08) as usize;
    rd::<usize>(param_1)
        .wrapping_add(new_len.wrapping_mul(2))
        .wrapping_sub(2)
}

/// `physics_14123dc30`
/// Original: 0x14123dc30 (173 bytes, Throws::No)
pub unsafe extern "win64" fn physics_14123dc30() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let pool = rd::<usize>(heap + 0x58);
    let mut rcx = rd::<usize>(pool + 0x10);
    if rcx != 0 && (rcx & 0xf) == 0 {
        wr::<u32>(pool + 0x18, rd::<u32>(pool + 0x18).wrapping_add(1));
        wr::<usize>(pool + 0x10, rd::<usize>(rcx));
    } else {
        rcx = exe::call_va!(0x1_4253_5070, (heap, 0x50u64, 0x10u64) -> usize);
        if rcx == 0 {
            let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
            rcx = 0;
        }
    }

    wr::<u64>(rcx + 0x08, 0);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x40, 0);
    wr::<usize>(rcx, exe::ptr::<u8>(0x1_433e_2618) as usize);
    wr::<usize>(rcx + 0x20, exe::ptr::<u8>(0x1_4307_82a0) as usize);
    wr::<u64>(rcx + 0x18, u64::MAX);
    wr::<u64>(rcx + 0x28, 0);
    wr::<u64>(rcx + 0x30, 0);
    wr::<u64>(rcx + 0x38, 0);
    wr::<u64>(rcx + 0x48, 0);
    rcx
}

/// `unknown_141291860`
/// Original: 0x141291860 (222 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141291860(param_1: usize) {
    let mut rbx = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let rsi = rbx.wrapping_add(count.wrapping_mul(0x2a8));
    while rbx != rsi {
        exe::call_va!(0x1_4128_ff80, (rbx));
        rbx = rbx.wrapping_add(0x2a8);
    }

    let r9 = rd::<usize>(param_1);
    if r9 != 0 && r9 != param_1.wrapping_add(0x10) {
        let rcx = r9.wrapping_sub(1) & !0x01ff_ffffusize;
        let slice_idx = r9.wrapping_sub(rcx) >> 16;
        let mut rdx = rcx
            .wrapping_add(0x108)
            .wrapping_add(slice_idx.wrapping_mul(0x50));
        let back_off = rd::<u32>(rdx + 0x04) as usize;
        rdx = rdx.wrapping_sub(back_off);
        if exe::teb() == rd::<usize>(rcx + 0x100) {
            if rd::<u8>(rdx + 0x0e) == 0 {
                wr::<usize>(r9, rd::<usize>(rdx + 0x20));
                let next_used = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                wr::<u32>(rdx + 0x18, next_used);
                wr::<usize>(rdx + 0x20, r9);
                if next_used == 0 {
                    exe::call_va!(0x1_4253_71a0, (rdx));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (rcx, rdx, 1u8, r9));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (rcx, rdx, 0u8, r9));
        }
    }

    wr::<usize>(param_1, param_1.wrapping_add(0x10));
    wr::<u32>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x0c, 0x10);
}

#[inline(always)]
unsafe fn mimalloc_alloc_aligned16(bytes: u64) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let r10 = rd::<usize>(tls0 + 0x6348);
    let mut out: usize = 0;
    if (bytes & bytes.wrapping_sub(1)) == 0 && bytes.wrapping_sub(0x10) <= 0x3f0 {
        let idx = (bytes.wrapping_add(7) >> 3) as usize;
        let page = rd::<usize>(r10 + 0x08 + idx * 8);
        let head = rd::<usize>(page + 0x10);
        if head == 0 {
            out = exe::call_va!(0x1_4253_7280, (r10, bytes, 0u64, 0u64) -> usize);
        } else {
            wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
            wr::<usize>(page + 0x10, rd::<usize>(head));
            out = head;
        }
    } else if bytes <= 0x7fff_ffff_ffff_ffff {
        let mut hit = false;
        if bytes.wrapping_sub(0x10) <= 0x3f0 {
            let idx = (bytes.wrapping_add(7) >> 3) as usize;
            let page = rd::<usize>(r10 + 0x08 + idx * 8);
            let head = rd::<usize>(page + 0x10);
            if head != 0 && (head & 0xf) == 0 {
                wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                wr::<usize>(page + 0x10, rd::<usize>(head));
                out = head;
                hit = true;
            }
        }
        if !hit {
            out = exe::call_va!(0x1_4253_5070, (r10, bytes, 0x10u64) -> usize);
        }
    }
    if out == 0 {
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
        0
    } else {
        out
    }
}

/// `unknown_141291e20`
/// Original: 0x141291e20 (425 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141291e20(param_1: usize, param_2: u64) {
    let r13d = rd::<u32>(param_1 + 0x08);
    let mut r12 = param_2;
    let r14 = if param_2 <= 0x10 && rd::<usize>(param_1) != param_1.wrapping_add(0x10) {
        r12 = 0x10;
        param_1.wrapping_add(0x10)
    } else {
        let bytes = param_2.wrapping_mul(0x90);
        mimalloc_alloc_aligned16(bytes)
    };

    let mut rbx = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let rbp = rbx.wrapping_add(count.wrapping_mul(0x90));
    let mut rdi = r14;
    while rbx != rbp {
        exe::call_va!(0x1_4129_2640, (rdi, rbx));
        rdi = rdi.wrapping_add(0x90);
        rbx = rbx.wrapping_add(0x90);
    }

    exe::call_va!(0x1_411f_5db0, (param_1));
    wr::<usize>(param_1, r14);
    wr::<u32>(param_1 + 0x08, r13d);
    wr::<u32>(param_1 + 0x0c, r12 as u32);
}

/// `r_1412dc6c0`
/// Original: 0x1412dc6c0 (1714 bytes, Throws::No)
pub unsafe extern "win64" fn r_1412dc6c0(param_1: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let tbl_base = exe::ptr::<u8>(0x1_4000_0000) as usize;

    if rd::<i32>(exe::ptr::<i32>(0x1_43cf_0088) as usize) > rd::<i32>(tls0 + 0x74) {
        let guard = exe::ptr::<i32>(0x1_43cf_0088) as usize;
        exe::call_va!(0x1_4183_4a78, (guard));
        if rd::<i32>(guard) == -1 {
            wr::<u32>(exe::ptr::<u32>(0x1_43ce_ff28) as usize, 0);
            wr::<u8>(exe::ptr::<u8>(0x1_43ce_ff2c) as usize, 0);
            wr::<usize>(
                exe::ptr::<usize>(0x1_43ce_ff20) as usize,
                exe::ptr::<u8>(0x1_4307_95f8) as usize,
            );
            wr::<u16>(exe::ptr::<u16>(0x1_43ce_ff30) as usize, 0);
            let idx = rd::<i32>(exe::ptr::<i32>(0x1_43a0_01ac) as usize);
            if idx < 0x1000 {
                let slot = tbl_base
                    .wrapping_add(0x3a0_0290)
                    .wrapping_add((idx as isize as usize).wrapping_mul(8));
                wr::<usize>(slot, exe::ptr::<u8>(0x1_43ce_ff20) as usize);
                wr::<i32>(exe::ptr::<i32>(0x1_43a0_01ac) as usize, idx.wrapping_add(1));
            }
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(0x1_4262_7730) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard));
        }
    }

    if rd::<i32>(exe::ptr::<i32>(0x1_43ce_ff18) as usize) > rd::<i32>(tls0 + 0x74) {
        let guard = exe::ptr::<i32>(0x1_43ce_ff18) as usize;
        exe::call_va!(0x1_4183_4a78, (guard));
        if rd::<i32>(guard) == -1 {
            exe::call_va!(0x1_401d_f790, (exe::ptr::<u8>(0x1_43ce_fed0) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard));
        }
    }

    let xmm7: f32 = if rd::<u8>(exe::ptr::<u8>(0x1_43ce_ff30) as usize) != 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize) != 0
    {
        let (_, xmm_d) = crate::simd_ops::call_va8_xmm(
            0x1_401e_42d0,
            exe::ptr::<u8>(0x1_43ce_fed0) as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, 0],
        );
        let f_in = f64::from_bits(xmm_d as u64) as f32;
        let (_, xmm_s) = crate::simd_ops::call_va8_xmm(
            0x1_4243_be50,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [f_in.to_bits() as u128, 0, 0, 0],
        );
        let f_out = f32::from_bits(xmm_s as u32);
        let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
        let c_mul = rd::<f32>(exe::ptr::<f32>(0x1_4342_672c) as usize);
        (f_out - c_half) * c_mul
    } else {
        0.0f32
    };

    let init_cvar = |guard_va: u64,
                     ptr_va: u64,
                     id_va: u64,
                     type_va: u64,
                     val_va: u64,
                     name_va: u64,
                     name_len: u64,
                     cvar_type: u32,
                     zero_floats: usize,
                     atexit_va: u64| {
        if rd::<i32>(exe::ptr::<i32>(guard_va) as usize) > rd::<i32>(tls0 + 0x74) {
            let guard = exe::ptr::<i32>(guard_va) as usize;
            exe::call_va!(0x1_4183_4a78, (guard));
            if rd::<i32>(guard) == -1 {
                wr::<usize>(exe::ptr::<usize>(ptr_va) as usize, 0);
                let mut sv = [exe::ptr::<u8>(name_va) as u64, name_len];
                exe::call_va!(
                    0x1_4220_ea70,
                    (exe::ptr::<u8>(id_va) as usize, sv.as_mut_ptr() as usize)
                );
                wr::<u32>(exe::ptr::<u32>(type_va) as usize, cvar_type);
                let val_ptr = exe::ptr::<u8>(val_va) as usize;
                wr::<usize>(exe::ptr::<usize>(ptr_va) as usize, val_ptr);
                for i in 0..zero_floats {
                    wr::<u32>(val_ptr + i * 4, 0);
                }
                exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
                exe::call_va!(0x1_4183_4a0c, (guard));
            }
        }
    };

    init_cvar(
        0x1_43ce_fe44,
        0x1_43cf_0090,
        0x1_43cf_0098,
        0x1_43cf_009c,
        0x1_43cf_00a0,
        0x1_433e_f7c0,
        0x14,
        0,
        0,
        0x1_4262_7720,
    );
    let id1 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0098) as usize) as isize;
    let sz1 = rd::<i32>(
        tbl_base
            .wrapping_add(0x3a6_37b8)
            .wrapping_add((id1 as usize).wrapping_mul(16)),
    ) as i64;
    exe::call_va!(0x1_4209_a900, (id1 as i64, 0u32, sz1, param_1 + 0x14));
    wr::<u32>(
        exe::ptr::<u32>(0x1_43cf_00a0) as usize,
        rd::<u32>(param_1 + 0x14),
    );

    init_cvar(
        0x1_43ce_fe80,
        0x1_43cf_00b0,
        0x1_43cf_00b8,
        0x1_43cf_00bc,
        0x1_43cf_00c0,
        0x1_433e_f788,
        0x10,
        1,
        2,
        0x1_4262_7710,
    );
    let id2 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_00b8) as usize) as isize;
    let sz2 = rd::<i32>(
        tbl_base
            .wrapping_add(0x3a6_37b8)
            .wrapping_add((id2 as usize).wrapping_mul(16)),
    ) as i64;
    exe::call_va!(0x1_4209_a900, (id2 as i64, 0u32, sz2, param_1 + 0x04));
    wr::<u64>(
        exe::ptr::<u64>(0x1_43cf_00c0) as usize,
        rd::<u64>(param_1 + 0x04),
    );

    init_cvar(
        0x1_43ce_fe40,
        0x1_43ce_fff0,
        0x1_43ce_fff8,
        0x1_43ce_fffc,
        0x1_43cf_0000,
        0x1_433e_f7a0,
        0x1d,
        2,
        3,
        0x1_4262_7700,
    );
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let sh_z = (one - rd::<f32>(param_1 + 0x20)) + xmm7;
    let sh_y = (one - rd::<f32>(param_1 + 0x1c)) + xmm7;
    let sh_x = (one - rd::<f32>(param_1 + 0x18)) + xmm7;
    let mut vec_buf = [sh_x, sh_y, sh_z, 0.0f32];
    let id3 = rd::<i32>(exe::ptr::<i32>(0x1_43ce_fff8) as usize) as isize;
    let entry3 = tbl_base
        .wrapping_add(0x3a6_37b0)
        .wrapping_add((id3 as usize).wrapping_mul(16));
    let sz3 = rd::<i32>(entry3 + 0x08) as usize;
    let off3 = rd::<i32>(entry3) as isize;
    if off3 >= 0 {
        let dst = tls0.wrapping_add(0x63d0).wrapping_add(off3 as usize);
        core::ptr::copy_nonoverlapping(vec_buf.as_ptr() as *const u8, dst as *mut u8, sz3);
        let cbuf = rd::<i32>(entry3 + 0x0c) as isize;
        if cbuf != -1 {
            let cnt_addr = tls0
                .wrapping_add(0x463e0)
                .wrapping_add((cbuf as usize).wrapping_mul(4));
            let next = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next == -1 { 0 } else { next });
        }
    }
    wr::<f32>(exe::ptr::<f32>(0x1_43cf_0000) as usize, sh_x);
    wr::<f32>(exe::ptr::<f32>(0x1_43cf_0004) as usize, sh_y);
    wr::<f32>(exe::ptr::<f32>(0x1_43cf_0008) as usize, sh_z);

    init_cvar(
        0x1_43ce_fe1c,
        0x1_43ce_ff40,
        0x1_43ce_ff48,
        0x1_43ce_ff4c,
        0x1_43ce_ff50,
        0x1_433e_f7d8,
        0x20,
        2,
        3,
        0x1_4262_76f0,
    );
    let hi_z = (one - rd::<f32>(param_1 + 0x2c)) + xmm7;
    let hi_y = (one - rd::<f32>(param_1 + 0x28)) + xmm7;
    let hi_x = (one - rd::<f32>(param_1 + 0x24)) + xmm7;
    vec_buf[0] = hi_x;
    vec_buf[1] = hi_y;
    vec_buf[2] = hi_z;
    let id4 = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ff48) as usize) as isize;
    let entry4 = tbl_base
        .wrapping_add(0x3a6_37b0)
        .wrapping_add((id4 as usize).wrapping_mul(16));
    let sz4 = rd::<i32>(entry4 + 0x08) as usize;
    let off4 = rd::<i32>(entry4) as isize;
    if off4 >= 0 {
        let dst = tls0.wrapping_add(0x63d0).wrapping_add(off4 as usize);
        core::ptr::copy_nonoverlapping(vec_buf.as_ptr() as *const u8, dst as *mut u8, sz4);
        let cbuf = rd::<i32>(entry4 + 0x0c) as isize;
        if cbuf != -1 {
            let cnt_addr = tls0
                .wrapping_add(0x463e0)
                .wrapping_add((cbuf as usize).wrapping_mul(4));
            let next = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next == -1 { 0 } else { next });
        }
    }
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_ff50) as usize, hi_x);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_ff54) as usize, hi_y);
    wr::<f32>(exe::ptr::<f32>(0x1_43ce_ff58) as usize, hi_z);

    init_cvar(
        0x1_43cf_00f0,
        0x1_43cf_00d0,
        0x1_43cf_00d8,
        0x1_43cf_00dc,
        0x1_43cf_00e0,
        0x1_433e_f800,
        0x14,
        0,
        0,
        0x1_4262_76e0,
    );
    let id5 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_00d8) as usize) as isize;
    let sz5 = rd::<i32>(
        tbl_base
            .wrapping_add(0x3a6_37b8)
            .wrapping_add((id5 as usize).wrapping_mul(16)),
    ) as i64;
    exe::call_va!(0x1_4209_a900, (id5 as i64, 0u32, sz5, param_1 + 0x30));
    wr::<u32>(
        exe::ptr::<u32>(0x1_43cf_00e0) as usize,
        rd::<u32>(param_1 + 0x30),
    );
    wr::<u8>(param_1, 0);
}

/// `std_1412df250`
/// Original: 0x1412df250 (97 bytes, Throws::Caught)
pub unsafe extern "win64" fn std_1412df250(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let mut rbx = param_2;
    while rbx != param_3 {
        let rdx = rd::<usize>(rbx);
        if rdx != 0 {
            let prev = crate::simd_ops::atomic_xadd_u32((rdx + 0x24) as u64, u32::MAX);
            if prev == 1 {
                let rcx = rd::<usize>(rdx + 0x28);
                if rcx != 0 {
                    exe::call_va!(0x1_4241_d6f0, (rcx, rdx));
                }
            }
        }
        wr::<usize>(rbx, 0);
        rbx = rbx.wrapping_add(0x10);
    }
}

/// `std_1412df620`
/// Original: 0x1412df620 (360 bytes, Throws::Discard)
pub unsafe extern "win64" fn std_1412df620(param_1: usize, param_2: u64) {
    let ebp = rd::<u32>(param_1 + 0x08);
    let bytes = param_2 << 4;
    let rbx = mimalloc_alloc_aligned16(bytes);

    let mut rcx = rd::<usize>(param_1);
    let r8 = rcx.wrapping_add((rd::<u32>(param_1 + 0x08) as usize) << 4);
    let mut rdx = rbx;
    while rcx != r8 {
        let rax = rd::<u64>(rcx);
        wr::<u64>(rcx, 0);
        wr::<u64>(rdx, rax);
        wr::<u32>(rdx + 0x08, rd::<u32>(rcx + 0x08));
        rdx = rdx.wrapping_add(0x10);
        rcx = rcx.wrapping_add(0x10);
    }

    exe::call_va!(0x1_412d_f190, (param_1));
    wr::<usize>(param_1, rbx);
    wr::<u32>(param_1 + 0x08, ebp);
    wr::<u32>(param_1 + 0x0c, param_2 as u32);
}

/// `rend_1412ed8e0`
/// Original: 0x1412ed8e0 (129 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ed8e0(param_1: usize, param_2: usize) -> usize {
    wr::<u64>(param_1, 0);
    let mut len: usize = 0;
    while rd::<u8>(param_2.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    let mut sv = [param_2 as u64, len as u64];
    exe::call_va!(0x1_4220_ea70, (param_1 + 0x08, sv.as_mut_ptr() as usize));
    let rcx = param_1.wrapping_add(0x10);
    wr::<u32>(param_1 + 0x0c, 8);
    wr::<usize>(param_1, rcx);
    let g_ptr = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4b68) as usize);
    if g_ptr != 0 {
        wr::<u64>(rcx, rd::<u64>(g_ptr));
    } else {
        wr::<u64>(rcx, 0);
    }
    param_1
}

/// `rend_1412ee6f0`
/// Original: 0x1412ee6f0 (394 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ee6f0(param_1: usize, param_2: u64) {
    let ebp = rd::<u32>(param_1 + 0x08);
    let bytes = param_2.wrapping_mul(0x50);
    let rdi = mimalloc_alloc_aligned16(bytes);

    let rcx = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let copy_bytes = count.wrapping_mul(0x50);
    if copy_bytes != 0 {
        core::ptr::copy_nonoverlapping(rcx as *const u8, rdi as *mut u8, copy_bytes);
    }

    exe::call_va!(0x1_401d_e830, (param_1));
    wr::<u32>(param_1 + 0x08, ebp);
    wr::<u32>(param_1 + 0x0c, param_2 as u32);
    wr::<usize>(param_1, rdi);
}

/// `rend_1412ee880`
/// Original: 0x1412ee880 (680 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ee880(param_1: usize, param_2: u64) {
    let ebp = rd::<u32>(param_1 + 0x08);
    let bytes = param_2.wrapping_mul(0xd0);
    let rbx = mimalloc_alloc_aligned16(bytes);

    let mut r9 = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let r10 = r9.wrapping_add(count.wrapping_mul(0xd0));
    let mut dst = rbx;
    while r9 != r10 {
        wr::<u32>(dst, rd::<u32>(r9));
        core::ptr::copy_nonoverlapping((r9 + 0x08) as *const u8, (dst + 0x08) as *mut u8, 0xc8);
        r9 = r9.wrapping_add(0xd0);
        dst = dst.wrapping_add(0xd0);
    }

    exe::call_va!(0x1_401d_e830, (param_1));
    wr::<usize>(param_1, rbx);
    wr::<u32>(param_1 + 0x08, ebp);
    wr::<u32>(param_1 + 0x0c, param_2 as u32);
}

/// `sparsehash_1413a2140`
/// Original: 0x1413a2140 (79 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1413a2140(param_1: usize, param_2: usize) -> usize {
    let rax = rd::<usize>(param_2);
    wr::<usize>(param_2, 0);
    let rdx = rd::<usize>(param_1);
    wr::<usize>(param_1, rax);
    if rdx != 0 {
        let rdi = rdx.wrapping_sub(0xffe8);
        let root = rd::<usize>(rdx + 0x10);
        if root != 0 {
            exe::call_va!(0x1_4242_df10, (param_1, root));
        }
        exe::call_va!(0x1_4244_2de0, (rdi));
    }
    param_1
}

/// `fmt_1414623b0`
/// Original: 0x1414623b0 (639 bytes, Throws::No)
pub unsafe extern "win64" fn fmt_1414623b0(_xmm0_slot: usize) -> u64 {
    let [xmm0, _, _, _] = crate::simd_ops::take_xmm_args();
    let bits = xmm0 as u32;
    let exp_byte = (bits >> 23) as u8;
    let mut mant = bits & 0x007f_ffff;
    let r8d: u32;
    if exp_byte != 0 {
        r8d = (exp_byte as u32).wrapping_add(0xffff_ff6a);
        if mant == 0 {
            return exe::call_va!(0x1_4146_43a0, (r8d) -> u64);
        }
        mant |= 0x0080_0000;
    } else {
        if mant == 0 {
            return 0;
        }
        r8d = 0xffff_ff6b;
    }

    let bl = (!mant as u8) & 1;
    let r13d = mant.wrapping_mul(2);
    let esi = (((r8d as i32).wrapping_mul(0x4d105)) >> 20) as u32;
    let ecx_idx = 1i32.wrapping_sub(esi as i32);
    let tbl = exe::ptr::<u64>(0x1_4341_fa98) as usize;
    let r12 = rd::<u64>(tbl.wrapping_add((ecx_idx as isize as usize).wrapping_mul(8)));
    let edi = ((ecx_idx.wrapping_mul(0x1a9_34f) >> 19) as u32).wrapping_add(r8d);
    let ebp = (r12 >> (0x3fu32.wrapping_sub(edi) & 0x3f)) as u32;

    let sh_mant = (r13d | 1) << (edi & 0x1f);
    let mul_rhs = (sh_mant as u64) << 32;
    let prod = (r12 as u128).wrapping_mul(mul_rhs as u128);
    let rdx_hi = (prod >> 64) as u64;
    let rax_top = (rdx_hi >> 32) as u32;
    let z_low = (rdx_hi as u32 == 0) as u8;

    let mut r9d: u32 = 0;
    let mut r10d = (((rax_top as u64).wrapping_mul(0x51eb_851f)) >> 37) as u32;
    let mut r11d = rax_top.wrapping_sub(r10d.wrapping_mul(100));

    let mut take_fast = false;
    if r11d < ebp {
        if r11d != 0 || ((bl ^ 1) & z_low) == 0 {
            take_fast = true;
        } else {
            r10d = r10d.wrapping_sub(1);
            r11d = 100;
        }
    } else if r11d == ebp {
        let rcx_mul = ((r13d.wrapping_sub(1)) as u64).wrapping_mul(r12);
        let r8b = ((rcx_mul >> (0x40u32.wrapping_sub(edi) & 0x3f)) as u8) & 1;
        let rcx_sh = (rcx_mul >> (0x20u32.wrapping_sub(edi) & 0x3f)) as u32;
        let dl = if rcx_sh == 0 { bl } else { 0 };
        if (dl | r8b) != 0 {
            take_fast = true;
        }
    }

    if take_fast {
        let mut eax = r10d.wrapping_mul(0xc28f_5c29).rotate_right(2);
        while eax <= 0x028f_5c28 {
            r10d = eax;
            r9d = r9d.wrapping_add(2);
            eax = eax.wrapping_mul(0xc28f_5c29).rotate_right(2);
        }
        let ecx = r10d.wrapping_mul(0xcccc_cccd).rotate_right(1);
        let mut exp_add = r9d | 1;
        if ecx <= 0x1999_9999 {
            r10d = ecx;
        } else {
            exp_add = r9d;
        }
        let hi = exp_add.wrapping_add(1).wrapping_add(esi);
        return ((hi as u64) << 32) | (r10d as u64);
    }

    r11d = r11d.wrapping_sub(ebp >> 1).wrapping_add(5);
    let edx = r11d.wrapping_mul(0x199a);
    let bl_odd = (!r11d as u8) & 1;
    let mut r8_out = (edx >> 16).wrapping_add(r10d.wrapping_mul(10));
    if (edx as u16) < 0x199a {
        let rdx_mul = (r13d as u64).wrapping_mul(r12);
        let cl = ((rdx_mul >> (0x40u32.wrapping_sub(edi) & 0x3f)) as u8) & 1;
        let r10_sh = (rdx_mul >> (0x20u32.wrapping_sub(edi) & 0x3f)) as u32;
        if cl != bl_odd || ((r8_out & 1) != 0 && r10_sh == 0) {
            r8_out = r8_out.wrapping_sub(1);
        }
    }
    ((esi as u64) << 32) | (r8_out as u64)
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_d3d_1411dcc60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF: [u64; 24] = [0; 24];
    static mut ADDREF_CNT: u32 = 0;

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_addref(_this: usize) {
        ADDREF_CNT = ADDREF_CNT.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, pool_hit: bool, with_obj: bool| -> (bool, u32, usize, u32, [u64; 18], u16, u8) {
        ALLOC_BUF = [0x5555_5555_5555_5555; 24];
        ADDREF_CNT = 0;
        let alloc_ptr = core::ptr::addr_of_mut!(ALLOC_BUF) as usize;
        wr::<u64>(alloc_ptr, 0x1234_5678);

        let mut heap = [0usize; 32];
        let mut pool = [0usize; 8];
        let hp = heap.as_mut_ptr() as usize;
        let pp = pool.as_mut_ptr() as usize;
        wr::<usize>(hp + 0xb0, pp);
        wr::<usize>(pp + 0x10, if pool_hit { alloc_ptr } else { 0 });
        wr::<u32>(pp + 0x18, 7);
        wr::<usize>(tls0 + 0x6348, hp);

        let vtbl: [usize; 2] = [0, mock_addref as *const () as usize];
        let obj: usize = vtbl.as_ptr() as usize;
        let p3 = if with_obj { (&obj as *const usize) as usize } else { 0 };

        let mut out_slot: usize = 0;
        let out_ptr = (&mut out_slot as *mut usize) as usize;
        let a5: u64 = 0x1111_2222_3333_4444;
        let a6: u64 = 0x5555_6666_7777_8888;
        let a7: u64 = 0x9999_aaaa_bbbb_cccc;
        let a8: u64 = 0xdddd_eeee_ffff_0001;

        let ret = f(
            out_ptr,
            0,
            p3,
            0,
            (&a5 as *const u64) as usize,
            (&a6 as *const u64) as usize,
            (&a7 as *const u64) as usize,
            (&a8 as *const u64) as usize,
        );
        wr::<usize>(tls0 + 0x6348, saved_heap);

        let r15 = out_slot;
        let fields = [
            rd::<u64>(r15),
            rd::<u64>(r15 + 0x08),
            rd::<u64>(r15 + 0x10),
            rd::<u8>(r15 + 0x18) as u64,
            rd::<u64>(r15 + 0x1c),
            rd::<u64>(r15 + 0x24),
            rd::<u64>(r15 + 0x30),
            rd::<u32>(r15 + 0x38) as u64,
            rd::<u32>(r15 + 0x3c) as u64,
            rd::<u64>(r15 + 0x40),
            rd::<u32>(r15 + 0x48) as u64,
            rd::<u64>(r15 + 0x58),
            rd::<u64>(r15 + 0x60),
            rd::<u32>(r15 + 0x68) as u64,
            (rd::<usize>(r15 + 0x70) == p3) as u64,
            rd::<u64>(r15 + 0x78),
            rd::<u64>(r15 + 0x80),
            rd::<u64>(r15 + 0x88) ^ rd::<u64>(r15 + 0x90),
        ];
        (
            ret == out_ptr && r15 == alloc_ptr,
            rd::<u32>(pp + 0x18),
            rd::<usize>(pp + 0x10),
            ADDREF_CNT,
            fields,
            rd::<u16>(r15 + 0x50),
            rd::<u8>(r15 + 0xa0),
        )
    };

    for (pool_hit, with_obj) in [(true, true), (true, false), (false, true)] {
        let o = run(f_o, pool_hit, with_obj);
        let n = run(f_n, pool_hit, with_obj);
        crate::diff_eq!(o, n, "1411dcc60 pool_hit={} with_obj={}", pool_hit, with_obj);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1411ec200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CAP: u64 = 0;
    static mut ABORT_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_3a60(vec_ptr: usize, new_cap: u64) {
        GROW_CAP = new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_3250(iter_ptr: usize) {
        let cur_val_ptr = rd::<usize>(iter_ptr + 0x10);
        wr::<usize>(iter_ptr + 0x10, cur_val_ptr.wrapping_add(2));
    }
    unsafe extern "win64" fn mock_0070() {
        ABORT_CALLED = ABORT_CALLED.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fb_3a60, mock_3a60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420a_3250, mock_3250 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4020_0070, mock_0070 as *const ())?;

    let run = |f: F, count: usize| -> (bool, u32, u32, u64, [u16; 4], u32) {
        GROW_CAP = 0;
        ABORT_CALLED = 0;

        let src_vals: [u16; 4] = [0x101, 0x202, 0x303, 0x404];
        let mut dst_vals = [0u16; 4];
        let hdr: [usize; 2] = [count, 0x1000];

        let mut p2 = [0u8; 0x28];
        let mut p3 = [0u8; 0x28];
        let p2p = p2.as_mut_ptr() as usize;
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p2p, hdr.as_ptr() as usize);
        wr::<usize>(p2p + 0x08, 0x1000);
        wr::<usize>(p2p + 0x10, src_vals.as_ptr() as usize);
        wr::<i8>(p2p + 0x20, 0);
        wr::<i8>(p3p + 0x20, 1);

        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, dst_vals.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, 0);
        wr::<u32>(vp + 0x0c, 2);

        let mut out_slot: usize = 0;
        let out_ptr = (&mut out_slot as *mut usize) as usize;
        let ret = f(out_ptr, p2p, p3p, vp);
        (
            ret == out_ptr && out_slot == vp,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            GROW_CAP,
            dst_vals,
            ABORT_CALLED,
        )
    };

    for count in [0usize, 2, 3] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "1411ec200 count={}", count);
    }
    Ok(3)
}

pub unsafe fn diff_sparsehash_1411f00d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_STORAGE: [u16; 16] = [0; 16];
    static mut FREED_CNT: u32 = 0;

    unsafe extern "win64" fn mock_e1c0(_old_cap: u64, _new_cap: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_STORAGE) as usize
    }
    unsafe extern "win64" fn mock_e830(_vec: usize) {
        FREED_CNT = FREED_CNT.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4028_e1c0, mock_e1c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, len: u32, cap: u32, pos_idx: usize| -> (bool, usize, u32, u32, u32, Vec<u16>) {
        NEW_STORAGE = [0; 16];
        FREED_CNT = 0;

        let mut buf: [u16; 8] = [10, 20, 30, 40, 0, 0, 0, 0];
        let bp = buf.as_mut_ptr() as usize;
        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, bp);
        wr::<u32>(vp + 0x08, len);
        wr::<u32>(vp + 0x0c, cap);

        let mut out_iter: usize = 0;
        let out_ptr = (&mut out_iter as *mut usize) as usize;
        let val: u16 = 99;
        let pos_ptr = bp.wrapping_add(pos_idx * 2);

        let ret = f(vp, out_ptr, pos_ptr, (&val as *const u16) as usize);
        let final_data = rd::<usize>(vp);
        let final_len = rd::<u32>(vp + 0x08);
        let final_cap = rd::<u32>(vp + 0x0c);
        let rel_off = out_iter.wrapping_sub(final_data);
        let mut elems = Vec::new();
        for i in 0..(final_len as usize) {
            elems.push(rd::<u16>(final_data + i * 2));
        }
        (ret == out_ptr, rel_off, final_len, final_cap, FREED_CNT, elems)
    };

    for (len, cap, pos_idx) in [(3u32, 6u32, 3usize), (3, 6, 1), (4, 4, 2), (4, 4, 0)] {
        let o = run(f_o, len, cap, pos_idx);
        let n = run(f_n, len, cap, pos_idx);
        crate::diff_eq!(o, n, "1411f00d0 len={} cap={} pos={}", len, cap, pos_idx);
    }
    Ok(4)
}

pub unsafe fn diff_sparsehash_1411f0210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_ARG: u64 = 0;

    unsafe extern "win64" fn mock_5d10(vec_ptr: usize, need: u64) {
        GROW_ARG = need;
        wr::<u32>(vec_ptr + 0x0c, (need as u32) + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fa_5d10, mock_5d10 as *const ())?;

    let run = |f: F, len: u32, cap: u32| -> (usize, u32, u32, u64, [u16; 6]) {
        GROW_ARG = 0;
        let mut buf: [u16; 6] = [11, 22, 33, 0, 0, 0];
        let bp = buf.as_mut_ptr() as usize;
        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, bp);
        wr::<u32>(vp + 0x08, len);
        wr::<u32>(vp + 0x0c, cap);

        let val: u16 = 0x7788;
        let ret = f(vp, (&val as *const u16) as usize);
        (
            ret.wrapping_sub(bp),
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            GROW_ARG,
            buf,
        )
    };

    for (len, cap) in [(2u32, 4u32), (3, 3)] {
        let o = run(f_o, len, cap);
        let n = run(f_n, len, cap);
        crate::diff_eq!(o, n, "1411f0210 len={} cap={}", len, cap);
    }
    Ok(2)
}

pub unsafe fn diff_physics_14123dc30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BLOCK_MEM: [u64; 10] = [0; 10];

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(BLOCK_MEM) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, pool_hit: bool| -> (usize, u32, usize, [u64; 10]) {
        BLOCK_MEM = [0xaaaa_bbbb_cccc_dddd; 10];
        let bp = core::ptr::addr_of_mut!(BLOCK_MEM) as usize;
        wr::<u64>(bp, 0x4321);

        let mut heap = [0usize; 16];
        let mut pool = [0usize; 6];
        let hp = heap.as_mut_ptr() as usize;
        let pp = pool.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x58, pp);
        wr::<usize>(pp + 0x10, if pool_hit { bp } else { 0 });
        wr::<u32>(pp + 0x18, 2);
        wr::<usize>(tls0 + 0x6348, hp);

        let ret = f();
        wr::<usize>(tls0 + 0x6348, saved_heap);
        (ret, rd::<u32>(pp + 0x18), rd::<usize>(pp + 0x10), BLOCK_MEM)
    };

    for pool_hit in [true, false] {
        let o = run(f_o, pool_hit);
        let n = run(f_n, pool_hit);
        crate::diff_eq!(o, n, "14123dc30 pool_hit={}", pool_hit);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141291860(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DTOR_SUM: usize = 0;
    static mut LOG_3200: (usize, usize, u8, usize) = (0, 0, 0, 0);
    static mut LOG_71A0: usize = 0;

    unsafe extern "win64" fn mock_ff80(elem: usize) {
        DTOR_SUM = DTOR_SUM.wrapping_add(rd::<u32>(elem) as usize);
    }
    unsafe extern "win64" fn mock_3200(seg: usize, hdr: usize, flag: u8, ptr: usize) {
        LOG_3200 = (seg, hdr, flag, ptr);
    }
    unsafe extern "win64" fn mock_71a0(hdr: usize) {
        LOG_71A0 = hdr;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4128_ff80, mock_ff80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let r10 = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, mode: u8| -> (bool, u32, u32, usize, (usize, usize, u8, usize), usize, u32) {
        DTOR_SUM = 0;
        LOG_3200 = (0, 0, 0, 0);
        LOG_71A0 = 0;
        core::ptr::write_bytes(r10 as *mut u8, 0, 0x1_2000);

        let block_ptr = r10 + 0x1_0000;
        wr::<u32>(block_ptr, 10);
        wr::<u32>(block_ptr + 0x2a8, 25);

        let hdr = r10 + 0x108 + 0x50;
        wr::<u32>(hdr + 0x04, 0);
        wr::<u64>(hdr + 0x20, 0x9999);
        wr::<u32>(hdr + 0x18, 1);

        let mut vec_buf = [0u64; 6];
        let vp = vec_buf.as_mut_ptr() as usize;
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 0x20);

        match mode {
            0 => {
                wr::<usize>(vp, vp + 0x10);
                wr::<u32>(vp + 0x08, 0);
            }
            1 => {
                wr::<usize>(r10 + 0x100, 0);
                wr::<usize>(vp, block_ptr);
            }
            2 => {
                wr::<usize>(r10 + 0x100, exe::teb());
                wr::<u8>(hdr + 0x0e, 1);
                wr::<usize>(vp, block_ptr);
            }
            _ => {
                wr::<usize>(r10 + 0x100, exe::teb());
                wr::<u8>(hdr + 0x0e, 0);
                wr::<usize>(vp, block_ptr);
            }
        }

        f(vp);
        (
            rd::<usize>(vp) == vp + 0x10,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            DTOR_SUM,
            LOG_3200,
            LOG_71A0,
            rd::<u32>(hdr + 0x18),
        )
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141291860 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141291e20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u8; 0x300] = [0; 0x300];
    static mut FREED_CNT: u32 = 0;

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }
    unsafe extern "win64" fn mock_2640(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src) ^ 0x55aa);
    }
    unsafe extern "win64" fn mock_5db0(_vec: usize) {
        FREED_CNT = FREED_CNT.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4129_2640, mock_2640 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411f_5db0, mock_5db0 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, new_cap: u64, start_inline: bool| -> (bool, u32, u32, u32, u64, u64) {
        DST_BUF = [0; 0x300];
        FREED_CNT = 0;
        let dp = core::ptr::addr_of_mut!(DST_BUF) as usize;

        let mut heap = [0usize; 0xa0];
        let mut page = [0usize; 6];
        let hp = heap.as_mut_ptr() as usize;
        let pp = page.as_mut_ptr() as usize;
        for s in heap.iter_mut() {
            *s = pp;
        }
        wr::<usize>(tls0 + 0x6348, hp);

        let mut ext_src = [0u8; 0x200];
        let esp = ext_src.as_mut_ptr() as usize;
        wr::<u64>(esp, 0x1111);
        wr::<u64>(esp + 0x90, 0x2222);

        let mut vec_buf = [0u8; 0x200];
        let vp = vec_buf.as_mut_ptr() as usize;
        let src_ptr = if start_inline { vp + 0x10 } else { esp };
        wr::<u64>(src_ptr, 0x1111);
        wr::<u64>(src_ptr + 0x90, 0x2222);
        wr::<usize>(vp, src_ptr);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 16);

        f(vp, new_cap);
        wr::<usize>(tls0 + 0x6348, saved_heap);

        let final_ptr = rd::<usize>(vp);
        let is_inline = final_ptr == vp + 0x10;
        (
            if is_inline { true } else { final_ptr == dp },
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            FREED_CNT,
            rd::<u64>(final_ptr),
            rd::<u64>(final_ptr + 0x90),
        )
    };

    for (new_cap, start_inline) in [(8u64, false), (4, true), (20, false)] {
        let o = run(f_o, new_cap, start_inline);
        let n = run(f_n, new_cap, start_inline);
        crate::diff_eq!(o, n, "141291e20 cap={} inline={}", new_cap, start_inline);
    }
    Ok(3)
}

pub unsafe fn diff_r_1412dc6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_A900_SUM: u64 = 0;

    unsafe extern "win64" fn mock_42d0(_timer: usize) -> u64 {
        crate::simd_ops::set_xmm_ret(0.75f64.to_bits() as u128);
        0
    }
    unsafe extern "win64" fn mock_be50(_x: usize) -> u64 {
        let [xmm0, _, _, _] = crate::simd_ops::take_xmm_args();
        let v = f32::from_bits(xmm0 as u32) + 0.125f32;
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        0
    }
    unsafe extern "win64" fn mock_a900(id: i64, _zero: u32, sz: i64, ptr: usize) {
        CALL_A900_SUM = CALL_A900_SUM
            .wrapping_add(id as u64)
            .wrapping_add(sz as u64)
            .wrapping_add(rd::<u32>(ptr) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_42d0, mock_42d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_be50, mock_be50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;

    let guards: [u64; 7] = [
        0x1_43cf_0088,
        0x1_43ce_ff18,
        0x1_43ce_fe44,
        0x1_43ce_fe80,
        0x1_43ce_fe40,
        0x1_43ce_fe1c,
        0x1_43cf_00f0,
    ];
    let mut saved_guards = [0i32; 7];
    for (i, &g) in guards.iter().enumerate() {
        saved_guards[i] = rd::<i32>(exe::ptr::<i32>(g) as usize);
        wr::<i32>(exe::ptr::<i32>(g) as usize, i32::MIN);
    }

    let cvar_ids: [u64; 5] = [
        0x1_43cf_0098,
        0x1_43cf_00b8,
        0x1_43ce_fff8,
        0x1_43ce_ff48,
        0x1_43cf_00d8,
    ];
    let mut saved_ids = [0i32; 5];
    for (i, &va) in cvar_ids.iter().enumerate() {
        saved_ids[i] = rd::<i32>(exe::ptr::<i32>(va) as usize);
        wr::<i32>(exe::ptr::<i32>(va) as usize, i as i32);
    }

    let tbl_base = exe::ptr::<u8>(0x1_4000_0000) as usize + 0x3a6_37b0;
    let mut saved_tbl = [0u8; 80];
    core::ptr::copy_nonoverlapping(tbl_base as *const u8, saved_tbl.as_mut_ptr(), 80);
    for i in 0..5usize {
        let e = tbl_base + i * 16;
        wr::<i32>(e, (i as i32) * 16);
        wr::<i32>(e + 0x04, 0);
        wr::<i32>(e + 0x08, 12);
        wr::<i32>(e + 0x0c, i as i32);
    }

    let saved_ff30 = rd::<u8>(exe::ptr::<u8>(0x1_43ce_ff30) as usize);
    let saved_3cab = rd::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize);

    let tls0 = rd::<usize>(exe::tls_pointer());

    let run = |f: F, active: bool| -> (u8, u64, [u32; 10], [u32; 6], [i32; 2]) {
        CALL_A900_SUM = 0;
        wr::<u8>(exe::ptr::<u8>(0x1_43ce_ff30) as usize, u8::from(active));
        wr::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize, u8::from(active));
        core::ptr::write_bytes((tls0 + 0x63d0) as *mut u8, 0, 0x60);
        wr::<i32>(tls0 + 0x463e0 + 2 * 4, 5);
        wr::<i32>(tls0 + 0x463e0 + 3 * 4, -2);

        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p, 1);
        wr::<f32>(p1p + 0x04, 0.25);
        wr::<f32>(p1p + 0x08, 0.75);
        wr::<f32>(p1p + 0x14, 1.25);
        wr::<f32>(p1p + 0x18, 0.1);
        wr::<f32>(p1p + 0x1c, 0.2);
        wr::<f32>(p1p + 0x20, 0.3);
        wr::<f32>(p1p + 0x24, 0.4);
        wr::<f32>(p1p + 0x28, 0.5);
        wr::<f32>(p1p + 0x2c, 0.6);
        wr::<f32>(p1p + 0x30, 0.9);

        f(p1p);

        let globals = [
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_00a0) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_00c0) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_00c4) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_0000) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_0004) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_0008) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ff50) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ff54) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ff58) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43cf_00e0) as usize),
        ];
        let tls_vals = [
            rd::<u32>(tls0 + 0x63d0 + 32),
            rd::<u32>(tls0 + 0x63d0 + 36),
            rd::<u32>(tls0 + 0x63d0 + 40),
            rd::<u32>(tls0 + 0x63d0 + 48),
            rd::<u32>(tls0 + 0x63d0 + 52),
            rd::<u32>(tls0 + 0x63d0 + 56),
        ];
        let cnts = [
            rd::<i32>(tls0 + 0x463e0 + 2 * 4),
            rd::<i32>(tls0 + 0x463e0 + 3 * 4),
        ];
        (rd::<u8>(p1p), CALL_A900_SUM, globals, tls_vals, cnts)
    };

    let res = (|| -> Result<u32, String> {
        for active in [false, true] {
            let o = run(f_o, active);
            let n = run(f_n, active);
            crate::diff_eq!(o, n, "1412dc6c0 active={}", active);
        }
        Ok(2)
    })();

    wr::<u8>(exe::ptr::<u8>(0x1_43ce_ff30) as usize, saved_ff30);
    wr::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize, saved_3cab);
    core::ptr::copy_nonoverlapping(saved_tbl.as_ptr(), tbl_base as *mut u8, 80);
    for (i, &va) in cvar_ids.iter().enumerate() {
        wr::<i32>(exe::ptr::<i32>(va) as usize, saved_ids[i]);
    }
    for (i, &g) in guards.iter().enumerate() {
        wr::<i32>(exe::ptr::<i32>(g) as usize, saved_guards[i]);
    }
    res
}

pub unsafe fn diff_std_1412df250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DESTROY_LOG: usize = 0;

    unsafe extern "win64" fn mock_d6f0(rcx: usize, rdx: usize) {
        DESTROY_LOG = DESTROY_LOG.wrapping_add(rcx ^ rd::<u32>(rdx) as usize);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let run = |f: F| -> ([usize; 6], u32, u32, usize) {
        DESTROY_LOG = 0;
        let mut obj1 = [0u8; 0x30];
        let mut obj2 = [0u8; 0x30];
        let o1 = obj1.as_mut_ptr() as usize;
        let o2 = obj2.as_mut_ptr() as usize;
        wr::<u32>(o1, 0x1234);
        wr::<u32>(o1 + 0x24, 1);
        wr::<usize>(o1 + 0x28, 0x7700);
        wr::<u32>(o2, 0x5678);
        wr::<u32>(o2 + 0x24, 2);
        wr::<usize>(o2 + 0x28, 0x8800);

        let mut arr: [usize; 6] = [o1, 0xaa, 0, 0xbb, o2, 0xcc];
        let start = arr.as_mut_ptr() as usize;
        let end = start + 6 * core::mem::size_of::<usize>();
        f(0, start, end);
        (arr, rd::<u32>(o1 + 0x24), rd::<u32>(o2 + 0x24), DESTROY_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412df250");
    Ok(1)
}

pub unsafe fn diff_std_1412df620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_16: [u64; 16] = [0; 16];
    static mut FREE_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_7280(_heap: usize, _sz: u64, _a3: u64, _a4: u64) -> usize {
        core::ptr::addr_of_mut!(ALLOC_16) as usize
    }
    unsafe extern "win64" fn mock_f190(_vec: usize) {
        FREE_CALLED = FREE_CALLED.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_7280, mock_7280 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412d_f190, mock_f190 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, pool_hit: bool| -> (bool, u32, u32, u32, [u64; 4], [u64; 6]) {
        ALLOC_16 = [0; 16];
        FREE_CALLED = 0;
        let ap = core::ptr::addr_of_mut!(ALLOC_16) as usize;

        let mut heap = [0usize; 32];
        let mut page = [0usize; 6];
        let hp = heap.as_mut_ptr() as usize;
        let pp = page.as_mut_ptr() as usize;
        for s in heap.iter_mut() {
            *s = pp;
        }
        wr::<usize>(pp + 0x10, if pool_hit { ap } else { 0 });
        wr::<usize>(tls0 + 0x6348, hp);

        let mut src: [u64; 4] = [0x1111_2222, 0x33, 0x4444_5555, 0x66];
        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, src.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 2);

        f(vp, 4);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        (
            rd::<usize>(vp) == ap,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            FREE_CALLED,
            src,
            [
                ALLOC_16[0],
                ALLOC_16[1],
                ALLOC_16[2],
                ALLOC_16[3],
                ALLOC_16[4],
                ALLOC_16[5],
            ],
        )
    };

    for pool_hit in [true, false] {
        let o = run(f_o, pool_hit);
        let n = run(f_n, pool_hit);
        crate::diff_eq!(o, n, "1412df620 pool_hit={}", pool_hit);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412ed8e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut OUT_OBJ: [u64; 4] = [0; 4];

    unsafe extern "win64" fn mock_ea70(dst: usize, sv: usize) {
        let len = rd::<u64>(sv + 0x08) as u32;
        wr::<u32>(dst, len.wrapping_add(0x100));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;

    let g_slot = exe::ptr::<usize>(0x1_43a3_4b68) as usize;
    let saved_g = rd::<usize>(g_slot);
    let dummy_val: u64 = 0xfeed_face_cafe_beef;

    let run = |f: F, has_global: bool| -> (usize, [u64; 4]) {
        OUT_OBJ = [0xffff_ffff_ffff_ffff; 4];
        let op = core::ptr::addr_of_mut!(OUT_OBJ) as usize;
        wr::<usize>(
            g_slot,
            if has_global {
                (&dummy_val as *const u64) as usize
            } else {
                0
            },
        );
        let cstr = b"hello_rend\0";
        let ret = f(op, cstr.as_ptr() as usize);
        wr::<usize>(g_slot, saved_g);
        (ret, OUT_OBJ)
    };

    for has_global in [false, true] {
        let o = run(f_o, has_global);
        let n = run(f_n, has_global);
        crate::diff_eq!(o, n, "1412ed8e0 has_global={}", has_global);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412ee6f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_50: [u64; 30] = [0; 30];
    static mut FREE_CNT: u32 = 0;

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_50) as usize
    }
    unsafe extern "win64" fn mock_e830(_vec: usize) {
        FREE_CNT = FREE_CNT.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, count: u32| -> (bool, u32, u32, u32, [u64; 20]) {
        NEW_50 = [0; 30];
        FREE_CNT = 0;
        let np = core::ptr::addr_of_mut!(NEW_50) as usize;

        let mut heap = [0usize; 0x90];
        let mut page = [0usize; 6];
        let hp = heap.as_mut_ptr() as usize;
        let pp = page.as_mut_ptr() as usize;
        for s in heap.iter_mut() {
            *s = pp;
        }
        wr::<usize>(tls0 + 0x6348, hp);

        let mut src = [0u64; 20];
        for (i, v) in src.iter_mut().enumerate() {
            *v = (i as u64 + 1).wrapping_mul(0x1001_0001);
        }
        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, src.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, count);
        wr::<u32>(vp + 0x0c, count);

        f(vp, 3);
        wr::<usize>(tls0 + 0x6348, saved_heap);

        let mut copied = [0u64; 20];
        copied.copy_from_slice(&NEW_50[..20]);
        (
            rd::<usize>(vp) == np,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            FREE_CNT,
            copied,
        )
    };

    for count in [0u32, 2] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "1412ee6f0 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412ee880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_D0: [u8; 0x200] = [0; 0x200];
    static mut FREE_CNT: u32 = 0;

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_D0) as usize
    }
    unsafe extern "win64" fn mock_e830(_vec: usize) {
        FREE_CNT = FREE_CNT.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, count: u32| -> (bool, u32, u32, u32, Vec<u8>) {
        NEW_D0 = [0xee; 0x200];
        FREE_CNT = 0;
        let np = core::ptr::addr_of_mut!(NEW_D0) as usize;

        let mut heap = [0usize; 0x90];
        let mut page = [0usize; 6];
        let hp = heap.as_mut_ptr() as usize;
        let pp = page.as_mut_ptr() as usize;
        for s in heap.iter_mut() {
            *s = pp;
        }
        wr::<usize>(tls0 + 0x6348, hp);

        let mut src = [0u8; 0x1a0];
        for (i, b) in src.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(17).wrapping_add(3);
        }
        let mut vec_s = [0u64; 2];
        let vp = vec_s.as_mut_ptr() as usize;
        wr::<usize>(vp, src.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, count);
        wr::<u32>(vp + 0x0c, count);

        f(vp, 2);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        (
            rd::<usize>(vp) == np,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            FREE_CNT,
            NEW_D0.to_vec(),
        )
    };

    for count in [0u32, 2] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "1412ee880 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_1413a2140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_DF10: (usize, usize) = (0, 0);
    static mut LOG_2DE0: usize = 0;

    unsafe extern "win64" fn mock_df10(p1: usize, root: usize) {
        LOG_DF10 = (p1, root);
    }
    unsafe extern "win64" fn mock_2de0(base: usize) {
        LOG_2DE0 = base;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4242_df10, mock_df10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    static mut P1_SLOT: usize = 0;
    static mut P2_SLOT: usize = 0;
    static mut NODE_MEM: [u64; 4] = [0; 4];

    let run = |f: F, has_old: bool, has_root: bool| -> (usize, usize, usize, (usize, usize), usize) {
        LOG_DF10 = (0, 0);
        LOG_2DE0 = 0;
        NODE_MEM = [0; 4];
        let np = core::ptr::addr_of_mut!(NODE_MEM) as usize;
        wr::<usize>(np + 0x10, if has_root { 0xdead_0010 } else { 0 });

        let p1 = core::ptr::addr_of_mut!(P1_SLOT) as usize;
        let p2 = core::ptr::addr_of_mut!(P2_SLOT) as usize;
        wr::<usize>(p1, if has_old { np } else { 0 });
        wr::<usize>(p2, 0xcafe_beef);

        let ret = f(p1, p2);
        (ret, rd::<usize>(p1), rd::<usize>(p2), LOG_DF10, LOG_2DE0)
    };

    for (has_old, has_root) in [(false, false), (true, false), (true, true)] {
        let o = run(f_o, has_old, has_root);
        let n = run(f_n, has_old, has_root);
        crate::diff_eq!(o, n, "1413a2140 old={} root={}", has_old, has_root);
    }
    Ok(3)
}

pub unsafe fn diff_fmt_1414623b0(orig: usize, new: usize) -> Result<u32, String> {
    let inputs: [u32; 16] = [
        0x0000_0000,
        0x8000_0000,
        0x0000_0001,
        0x007f_ffff,
        0x3f80_0000,
        0x4000_0000,
        0x3f00_0000,
        0x3dcc_cccd,
        0x3e99_999a,
        0x4049_0fdb,
        0x42c8_0000,
        0x4640_e400,
        0x2edbe6ff,
        0x60ad_78ec,
        0x0080_0000,
        0x7f7f_ffff,
    ];

    for &bits in &inputs {
        let (o, _) = crate::simd_ops::call_va8_xmm(
            orig as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [bits as u128, 0, 0, 0],
        );
        let (n, _) = crate::simd_ops::call_va8_xmm(
            new as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [bits as u128, 0, 0, 0],
        );
        crate::diff_eq!(o, n, "1414623b0 bits={:#010x}", bits);
    }
    Ok(inputs.len() as u32)
}
