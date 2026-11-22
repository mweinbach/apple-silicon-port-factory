//! Decompiled hot functions shard X1 (`0x1401e84a0..0x141001100`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{vaddps, vmulps, vpermilps, vshufps, vsubps};

/// `unknown_1401e84a0`
/// Original: 0x1401e84a0 (117 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401e84a0(param_1: usize, param_2: usize) -> usize {
    exe::call_va!(0x1_401e_8620, (param_1));
    wr::<u64>(param_1, rd::<u64>(param_2));
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(param_2 + 0x14) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_2 + 0x14) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }
    let v8 = rd::<u64>(param_2 + 0x08);
    let _ = crate::simd_ops::atomic_xchg_u64((param_1 + 0x08) as u64, v8);
    let v10 = rd::<u32>(param_2 + 0x10);
    let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x10) as u64, v10);
    crate::simd_ops::wu8((param_2 + 0x14) as u64, 0); // spinlock release (x86 mov = release)
    param_1
}

/// `unknown_1401eacd0`
/// Original: 0x1401eacd0 (372 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401eacd0(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    param_4: usize,
    param_5: u8,
) {
    let mut entry_ptr: Option<usize> = None;
    if rd::<u64>(param_1 + 0x40) == rd::<u64>(param_1 + 0x38) {
        let end_init = rd::<usize>(param_1 + 0x78)
            .wrapping_add((rd::<usize>(param_1 + 0x48)).wrapping_mul(0x28));
        entry_ptr = Some(end_init);
    } else {
        let mut probe: [u64; 2] = [0; 2];
        let _ = exe::call_va!(
            0x1_401e_8680,
            (param_1, probe.as_mut_ptr() as usize, param_2) -> usize
        );
        if probe[0] != u64::MAX {
            entry_ptr = Some(
                rd::<usize>(param_1 + 0x78).wrapping_add((probe[0] as usize).wrapping_mul(0x28)),
            );
        }
    }

    if let Some(r10) = entry_ptr {
        let end_ptr = rd::<usize>(param_1 + 0x78)
            .wrapping_add((rd::<usize>(param_1 + 0x48)).wrapping_mul(0x28));
        if r10 != end_ptr {
            let b = rd::<u8>(param_4);
            let edx = rd::<u32>(r10 + 0x14);
            wr::<u16>(r10 + 0x10, 0);
            let mask_val = if b != 0 { u64::MAX } else { 0 };
            wr::<u8>(r10 + 0x13, 0xff);
            wr::<u64>(r10 + 0x18, mask_val);
            let bit_idx = (edx >> 5) as usize;
            let word_off = (bit_idx >> 6) * 8;
            let bit_pos = (edx >> 5) & 0x3f;
            let word_addr = param_1.wrapping_add(word_off).wrapping_add(0x118);
            wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_pos));
            wr::<u8>(r10 + 0x12, rd::<u8>(r10 + 0x12) | param_5);
            return;
        }
    }

    let free_base = rd::<usize>(param_1 + 0x100);
    let free_cnt = rd::<u32>(param_1 + 0x108);
    let free_end = free_base.wrapping_add((free_cnt as usize) * 4);
    let edx = if free_base == free_end {
        let idx = rd::<u32>(param_1 + 0x110);
        wr::<u32>(param_1 + 0x110, idx.wrapping_add(1));
        idx
    } else {
        let idx = rd::<u32>(free_end.wrapping_sub(4));
        wr::<u32>(param_1 + 0x108, free_cnt.wrapping_sub(1));
        idx
    };

    let b = rd::<u8>(param_4);
    let mask_val = if b != 0 { u64::MAX } else { 0 };
    let bit_idx = (edx >> 5) as usize;
    let word_off = (bit_idx >> 6) * 8;
    let bit_pos = (edx >> 5) & 0x3f;
    let word_addr = param_1.wrapping_add(word_off).wrapping_add(0x118);
    wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_pos));

    let dst = exe::call_va!(0x1_401e_b860, (param_1, param_2) -> usize);
    let mut val_lo = [0u8; 8];
    val_lo[2] = param_5;
    val_lo[3] = 0xff;
    val_lo[4..8].copy_from_slice(&edx.to_le_bytes());
    wr::<u64>(dst + 0x10, u64::from_le_bytes(val_lo));
    wr::<u64>(dst + 0x18, mask_val);
    wr::<u64>(dst + 0x20, 0);
}

/// `unknown_14024cc10`
/// Original: 0x14024cc10 (249 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14024cc10(param_1: usize, param_2: usize) -> usize {
    let rsi = exe::call_va!(0x1_423f_dfc0, () -> usize);
    let mut arg_val: [u64; 2] = [rd::<u64>(param_2), 0];
    let mut fmt_args: [u64; 2] = [0xc, arg_val.as_mut_ptr() as usize as u64];
    let mut fmt_str: [u64; 2] = rd::<[u64; 2]>(param_1);

    let mut iter_buf = [0u64; 0x140 / 8];
    let bp = iter_buf.as_mut_ptr() as usize;
    wr::<usize>(bp, exe::ptr::<u8>(0x1_4307_9330) as usize);
    wr::<usize>(bp + 0x08, rsi);
    wr::<u64>(bp + 0x10, 0);
    wr::<u64>(bp + 0x18, 0x7ff);
    wr::<u64>(bp + 0x20, 0);
    wr::<u64>(bp + 0x28, 0x7ff);
    wr::<usize>(bp + 0x30, rsi);

    exe::call_va!(
        0x1_4146_1fb0,
        (
            bp,
            fmt_str.as_mut_ptr() as usize,
            fmt_args.as_mut_ptr() as usize,
            0u32
        )
    );

    let limit = rd::<u64>(bp + 0x28);
    let count = rd::<u64>(bp + 0x20);
    let mut rbx: u64 = if limit > count {
        limit.wrapping_sub(count)
    } else {
        0
    };
    let size = rd::<u64>(bp + 0x10);
    if size < rbx {
        rbx = size;
    }
    let mut out_ptr = rd::<usize>(bp + 0x30);
    if rd::<usize>(bp + 0x08) == out_ptr {
        out_ptr = out_ptr.wrapping_add(rbx as usize);
    }
    wr::<u8>(out_ptr, 0);
    rsi
}

/// `unknown_1403eac30`
/// Original: 0x1403eac30 (660 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_1403eac30(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: f32,
    param_5: f32,
) -> u8 {
    unknown_1403eac30_impl(param_1, param_2, param_3, param_4, param_5)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_1403eac30(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _r9: usize,
    param_5_bits: u32,
    _xmm0: f32,
    _xmm1: f32,
    _xmm2: f32,
    param_4: f32,
) -> u8 {
    unknown_1403eac30_impl(
        param_1,
        param_2,
        param_3,
        param_4,
        f32::from_bits(param_5_bits),
    )
}

#[inline(always)]
unsafe fn unknown_1403eac30_impl(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: f32,
    param_5: f32,
) -> u8 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();

    let c_7b00 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7b00) as usize);
    let xmm5_0 = c_7b00 ^ rd::<u128>(param_3);
    let c_8760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
    let xmm6 = rd::<u128>(param_2);
    let xmm10 = vpermilps(xmm5_0, 0xc9);
    let xmm9 = vpermilps(xmm5_0, 0xd2);
    let xmm11 = c_8760 ^ rd::<u128>(param_3 + 0x10);
    let xmm7_0 = vpermilps(xmm5_0, 0xff);

    let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let xmm8_0 = vmulps(
        vsubps(
            vmulps(vpermilps(xmm11, 0xd2), xmm10),
            vmulps(vpermilps(xmm11, 0xc9), xmm9),
        ),
        c_7ab0,
    );
    let xmm5_1 = vmulps(
        vsubps(
            vmulps(vshufps(xmm6, xmm6, 0xd2), xmm10),
            vmulps(vshufps(xmm6, xmm6, 0xc9), xmm9),
        ),
        c_7ab0,
    );
    let xmm4_0 = vsubps(
        vmulps(vpermilps(xmm5_1, 0xd2), xmm10),
        vmulps(vpermilps(xmm5_1, 0xc9), xmm9),
    );
    let xmm5_2 = vaddps(xmm4_0, vaddps(vmulps(xmm5_1, xmm7_0), xmm6));

    let xmm4_1 = vsubps(
        vmulps(vpermilps(xmm8_0, 0xd2), xmm10),
        vmulps(vpermilps(xmm8_0, 0xc9), xmm9),
    );
    let xmm1_1 = vaddps(xmm4_1, vaddps(vmulps(xmm7_0, xmm8_0), xmm11));
    let xmm7 = vaddps(xmm5_2, xmm1_1);

    let y = f32::from_bits(vpermilps(xmm7, 0x55) as u32);
    let z = f32::from_bits(vpermilps(xmm7, 0xaa) as u32);
    if !(param_4 <= y && y <= param_5) {
        return 0;
    }

    let x = f32::from_bits(xmm7 as u32);
    let mut rbx = rd::<usize>(param_1);
    let mut rsi = rd::<u32>(param_1 + 0x08) as usize;
    let mut rdi: usize = 0;
    let mut xmm6_sum: f32 = 0.0;
    let mut pts: [[f32; 2]; 2] = [[0.0, 0.0], [0.0, 0.0]];

    let last_addr = rbx.wrapping_add(rsi.wrapping_mul(8)).wrapping_sub(8);
    let last_x = rd::<f32>(last_addr) - x;
    let last_y = rd::<f32>(last_addr + 0x04) - z;
    pts[1] = [last_x, last_y];

    while rsi != 0 {
        let cur_x = rd::<f32>(rbx) - x;
        let cur_y = rd::<f32>(rbx + 0x04) - z;
        pts[rdi] = [cur_x, cur_y];
        let prev = pts[rdi ^ 1];
        let atan_y = cur_y * prev[0] - cur_x * prev[1];
        let atan_x = cur_x * prev[0] + cur_y * prev[1];
        let (_, atan_xmm0) = crate::simd_ops::call_va8_xmm(
            0x1_4183_ad94,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [atan_y.to_bits() as u128, atan_x.to_bits() as u128, 0, 0],
        );
        let angle = f32::from_bits(atan_xmm0 as u32);
        rdi ^= 1;
        rbx = rbx.wrapping_add(8);
        xmm6_sum += angle;
        rsi -= 1;
    }

    let mask_ff0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
    let abs_sum = f32::from_bits((!mask_ff0) & xmm6_sum.to_bits());
    let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_6c2c) as usize);
    u8::from(abs_sum > thresh)
}

/// `unknown_140460c40`
/// Original: 0x140460c40 (243 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140460c40(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
    } else {
        0
    };
    if ent_count != 0 {
        let v0 = rd::<usize>(world);
        let id_4b4 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_84b4) as usize) as usize;
        let lut = rd::<usize>(v0 + 0x20);
        let idx_4b4 = rd::<u16>(lut.wrapping_add(id_4b4 * 2)) as usize;
        let entry_4b4 = rd::<usize>(world + 0x08).wrapping_add(idx_4b4 * 16);
        let len_4b4 = rd::<u32>(entry_4b4 + 0x08) as u64;
        let ptr_4b4 = rd::<u64>(entry_4b4);

        let id_648 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_8648) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_648 = rd::<u64>(tbl_580e0.wrapping_add(id_648 * 8));

        let mut pkt = [0u64; 8];
        pkt[0] = param_2 as u64;
        pkt[1] = block as u64;
        pkt[2] = world as u64;
        pkt[3] = ent_count;

        let mut tmp_out = [0u64; 4];
        let out_ptr = exe::call_va!(
            0x1_4046_0e20,
            (pkt.as_mut_ptr() as usize, tmp_out.as_mut_ptr() as usize) -> usize
        );
        let offs = rd::<[u64; 4]>(out_ptr);
        pkt[4..8].copy_from_slice(&offs);

        let mut arg_r8: [u64; 4] = [val_648, world as u64, len_4b4, ptr_4b4];
        exe::call_va!(
            0x1_4046_0d40,
            (
                0usize,
                pkt.as_mut_ptr() as usize,
                arg_r8.as_mut_ptr() as usize
            )
        );
    }
}

/// `unknown_1404b9a10`
/// Original: 0x1404b9a10 (85 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1404b9a10(param_1: usize) -> u64 {
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(param_1 + 0x18) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0x18) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }
    wr::<u32>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    crate::simd_ops::wu8((param_1 + 0x18) as u64, 0); // spinlock release (x86 mov = release)
    0
}

/// `content_14093ec10`
/// Original: 0x14093ec10 (731 bytes, Throws::Discard)
pub unsafe extern "win64" fn content_14093ec10(
    param_1: usize,
    param_2: usize,
    _param_3: u8,
) -> u64 {
    let s_name = rd::<u64>(exe::ptr::<u64>(0x1_4387_dd30) as usize);
    let crc = rd::<u32>(exe::ptr::<u32>(0x1_4387_db3c) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, s_name, crc, 0xau32, 1u32)
    );
    exe::call_va!(0x1_4094_69e0, (param_1, param_2));

    let c_7660 = rd::<[u32; 4]>(exe::ptr::<u8>(0x1_4342_7660) as usize);
    if rd::<i32>(param_2 + 0x328) < 0 {
        if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u64>(param_1 + 0x40, 0x3f80_0000);
            wr::<u32>(param_1 + 0x48, 0);
            wr::<u32>(param_1 + 0x4c, 0);
            wr::<u32>(param_1 + 0x50, c_7660[0]);
            wr::<u32>(param_1 + 0x64, c_7660[1]);
            wr::<u32>(param_1 + 0x68, c_7660[2]);
            wr::<u32>(param_1 + 0x58, c_7660[2]);
            wr::<u32>(param_1 + 0x5c, c_7660[3]);
            wr::<u32>(param_1 + 0x6c, c_7660[3]);
            wr::<u32>(param_1 + 0x54, c_7660[1]);
            wr::<u32>(param_1 + 0x60, c_7660[0]);
        }
    } else {
        exe::call_va!(0x1_4071_5850, (rd::<usize>(param_2 + 0x338), param_1 + 0x40));
    }

    if rd::<i32>(param_2 + 0x328) < 2 {
        if rd::<u8>(param_2 + 0x369) != 0 {
            let mut empty_str: [u64; 2] = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
            exe::call_va!(
                0x1_4240_9b50,
                (
                    param_1 + 0x70,
                    empty_str.as_mut_ptr() as usize,
                    0u32,
                    0u32
                )
            );
        }
    } else {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (s338, param_1 + 0x70));
    }

    let mut count: i32 = 0;
    if rd::<i32>(param_2 + 0x328) >= 2 {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(
            rd::<usize>(vtbl + 0x48),
            (s338, (&mut count as *mut i32) as usize)
        );
    }
    exe::call_va!(0x1_4094_3c70, (param_1 + 0xa8, count as i64));
    if count > 0 {
        let mut rbx = rd::<usize>(param_1 + 0xa8);
        let mut rem = count as u32;
        while rem != 0 {
            if rd::<i32>(param_2 + 0x328) < 2 {
                if rd::<u8>(param_2 + 0x369) != 0 {
                    let mut local_50 = [0u64; 3];
                    let l50p = local_50.as_mut_ptr() as usize;
                    let _ = exe::call_va!(0x1_401e_84a0, (rbx, l50p) -> usize);
                    let _ = crate::simd_ops::atomic_xchg_u64((l50p + 0x08) as u64, 0);
                }
            } else {
                let rdi = rd::<usize>(param_2 + 0x338);
                exe::call_va!(0x1_401e_8620, (rbx));
                let vtbl = rd::<usize>(rdi);
                exe::call_va!(rd::<usize>(vtbl), (rdi, rbx));
            }
            rbx = rbx.wrapping_add(0x18);
            rem -= 1;
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(3) <= 1
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            exe::call_va!(rd::<usize>(vtbl + 0x38), (inner, 4u32, 0u32));
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(4) <= 5
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            exe::call_va!(rd::<usize>(vtbl + 0x38), (inner, 4u32, 0u32));
        }
    }

    if rd::<i32>(param_2 + 0x328) < 9 {
        if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u8>(param_1 + 0xb8, 0);
        }
    } else {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x80), (s338, param_1 + 0xb8));
    }

    exe::call_va!(0x1_4094_2d40, (param_2));
    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

#[inline(always)]
unsafe fn deser_bool(param_1: usize, param_2: usize, off: usize, min_ver: i32, default: u8) {
    if rd::<i32>(param_2 + 0x328) >= min_ver {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x80), (s338, param_1.wrapping_add(off)));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1.wrapping_add(off), default);
    }
}

#[inline(always)]
unsafe fn deser_u32(param_1: usize, param_2: usize, off: usize, min_ver: i32, default: u32) {
    if rd::<i32>(param_2 + 0x328) >= min_ver {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x48), (s338, param_1.wrapping_add(off)));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1.wrapping_add(off), default);
    }
}

/// `content_140d40150`
/// Original: 0x140d40150 (1523 bytes, Throws::Discard)
pub unsafe extern "win64" fn content_140d40150(
    param_1: usize,
    param_2: usize,
    _param_3: u8,
) -> u64 {
    let s_name = rd::<u64>(exe::ptr::<u64>(0x1_4387_f6d8) as usize);
    let crc = rd::<u32>(exe::ptr::<u32>(0x1_4387_f64c) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, s_name, crc, 0x1au32, 1u32)
    );
    exe::call_va!(0x1_406e_02b0, (param_1, param_2));

    deser_bool(param_1, param_2, 0x08, 3, 1);
    deser_bool(param_1, param_2, 0x09, 4, 0);
    deser_bool(param_1, param_2, 0x0a, 0x11, 1);
    deser_bool(param_1, param_2, 0x0b, 7, 0);

    let mut cnt1: i32 = 0;
    if rd::<i32>(param_2 + 0x328) >= 0x19 {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(
            rd::<usize>(vtbl + 0x48),
            (s338, (&mut cnt1 as *mut i32) as usize)
        );
    }
    exe::call_va!(0x1_40d4_ac20, (param_1 + 0x38, cnt1 as i64));
    if cnt1 > 0 {
        let mut rbx = rd::<usize>(param_1 + 0x38);
        let mut rem = cnt1 as u32;
        while rem != 0 {
            if rd::<i32>(param_2 + 0x328) >= 0x19 {
                exe::call_va!(0x1_406d_cbc0, (rd::<usize>(param_2 + 0x338), rbx));
            } else if rd::<u8>(param_2 + 0x369) != 0 {
                let mut zero32 = [0u64; 4];
                exe::call_va!(0x1_404b_9990, (rbx, zero32.as_mut_ptr() as usize));
            }
            rbx = rbx.wrapping_add(0x20);
            rem -= 1;
        }
    }

    deser_bool(param_1, param_2, 0x1c, 0x13, 0);

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(9) <= 3
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            exe::call_va!(rd::<usize>(vtbl + 0x38), (inner, 1u32, 0u32));
        }
    }

    deser_bool(param_1, param_2, 0x10, 0xa, 0);
    deser_bool(param_1, param_2, 0x11, 0xb, 0);
    deser_bool(param_1, param_2, 0x12, 0xe, 0);
    deser_bool(param_1, param_2, 0x13, 0x12, 0);
    deser_bool(param_1, param_2, 0x14, 0xc, 0);
    deser_u32(param_1, param_2, 0x18, 2, 0);

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(6) <= 8
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            exe::call_va!(rd::<usize>(vtbl + 0x38), (inner, 4u32, 0u32));
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(5) <= 0xf
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            exe::call_va!(rd::<usize>(vtbl + 0x38), (inner, 4u32, 0u32));
        }
    }

    deser_bool(param_1, param_2, 0x1d, 0x15, 0);
    deser_bool(param_1, param_2, 0x1e, 0x15, 0);
    deser_bool(param_1, param_2, 0x1f, 0x16, 0);
    deser_bool(param_1, param_2, 0x20, 0x18, 0);
    deser_u32(param_1, param_2, 0x24, 0x15, 5);

    exe::call_va!(0x1_40d4_a530, (param_2, param_1 + 0x28));

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(9) <= 6
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let vtbl = rd::<usize>(inner);
            let mut sink = [0u8; 0x18];
            exe::call_va!(
                rd::<usize>(vtbl + 0x28),
                (inner, sink.as_mut_ptr() as usize, 0x18u32)
            );
        }
    }

    let mut cnt2: i32 = 0;
    if rd::<i32>(param_2 + 0x328) >= 8 {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(
            rd::<usize>(vtbl + 0x48),
            (s338, (&mut cnt2 as *mut i32) as usize)
        );
    }
    exe::call_va!(0x1_40a0_2480, (param_1 + 0x48, cnt2 as i64));
    if cnt2 > 0 {
        let s340 = rd::<usize>(param_2 + 0x340);
        let vtbl = rd::<usize>(s340);
        let byte_len = ((cnt2 as i64) * 0x30) as u64;
        exe::call_va!(
            rd::<usize>(vtbl + 0x28),
            (s340, rd::<u64>(param_1 + 0x48), byte_len)
        );
    }

    let mut cnt3: i32 = 0;
    if rd::<i32>(param_2 + 0x328) >= 8 {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(
            rd::<usize>(vtbl + 0x48),
            (s338, (&mut cnt3 as *mut i32) as usize)
        );
    }
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x58, cnt3 as i64));
    if cnt3 > 0 {
        let s340 = rd::<usize>(param_2 + 0x340);
        let vtbl = rd::<usize>(s340);
        let byte_len = ((cnt3 as i64) << 2) as u64;
        exe::call_va!(
            rd::<usize>(vtbl + 0x28),
            (s340, rd::<u64>(param_1 + 0x58), byte_len)
        );
    }

    if rd::<i32>(param_2 + 0x328) >= 0x14 {
        let s338 = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x38), (s338, param_1 + 0x68));
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x38), (s338, param_1 + 0x6c));
        let vtbl = rd::<usize>(s338);
        exe::call_va!(rd::<usize>(vtbl + 0x38), (s338, param_1 + 0x70));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x68, 0x3f80_0000);
        wr::<u32>(param_1 + 0x6c, 0x3f80_0000);
        wr::<u32>(param_1 + 0x70, 0x3f80_0000);
    }

    deser_bool(param_1, param_2, 0x15, 0x17, 0);
    deser_u32(param_1, param_2, 0x0c, 0x1a, 0);

    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `unknown_140f2daa0`
/// Original: 0x140f2daa0 (345 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_140f2daa0(param_1: usize) {
    let world = rd::<usize>(param_1);
    let tbl_0 = rd::<usize>(world + 0x580e0);
    let id_e9c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce9c) as usize) as usize;
    let val_e9c = rd::<u64>(tbl_0.wrapping_add(id_e9c * 8));
    let id_114 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d114) as usize) as usize;
    let val_114 = rd::<u64>(tbl_0.wrapping_add(id_114 * 8));

    let v0 = rd::<usize>(world);
    let id_0cc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d0cc) as usize) as usize;
    let lut = rd::<usize>(v0 + 0x20);
    let idx_0cc = rd::<u16>(lut.wrapping_add(id_0cc * 2)) as usize;
    let entry_0cc = rd::<usize>(world + 0x08).wrapping_add(idx_0cc * 16);
    let len_0cc = rd::<u32>(entry_0cc + 0x08) as u64;
    let ptr_0cc = rd::<u64>(entry_0cc);

    let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let r15 = world
        .wrapping_add(0x580f0)
        .wrapping_add(worker_idx.wrapping_mul(0x188));

    let world2 = rd::<usize>(param_1);
    let tbl_1 = rd::<usize>(world2 + 0x580e0);
    let id_0c0 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d0c0) as usize) as usize;
    let val_0c0 = rd::<u64>(tbl_1.wrapping_add(id_0c0 * 8));
    let id_170 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d170) as usize) as usize;
    let val_170 = rd::<u64>(tbl_1.wrapping_add(id_170 * 8));
    let id_080 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d080) as usize) as usize;
    let val_080 = rd::<u64>(tbl_1.wrapping_add(id_080 * 8));
    let id_014 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d014) as usize) as usize;
    let val_014 = rd::<u64>(tbl_1.wrapping_add(id_014 * 8));
    let id_018 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d018) as usize) as usize;
    let val_018 = rd::<u64>(tbl_1.wrapping_add(id_018 * 8));

    let _ = exe::call_va!(0x1_4240_7c80, () -> u32);

    let mut arg_80: [u64; 3] = [world as u64, len_0cc, ptr_0cc];
    exe::call_va!(
        0x1_41dc_2180,
        (
            val_e9c,
            val_114,
            world,
            world,
            arg_80.as_mut_ptr() as usize,
            r15,
            val_0c0,
            val_170,
            val_080,
            val_014,
            val_018
        )
    );
}

/// `unknown_140f79790`
/// Original: 0x140f79790 (497 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f79790(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world = rd::<usize>(param_1);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_63c = rd::<u64>(tbl_580e0.wrapping_add(id_63c * 8));

        let mut pkt = [0u64; 6];
        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let arch_usize = arch as usize;
            let block = rd::<usize>(world.wrapping_add(arch_usize * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add(arch_usize * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_d00 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_6d00) as usize) as u32;
                let arch_off = arch_usize.wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_d00) -> i32);
                let off_slot = world
                    .wrapping_add(arch_usize.wrapping_add(0xc03).wrapping_shl(5))
                    .wrapping_add(0x40);
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_87c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d87c) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_87c) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                if off1 != 0 {
                    off1 = block.wrapping_add(off1);
                }
                if off2 != 0 {
                    off2 = block.wrapping_add(off2);
                }

                pkt[0] = (pkt[0] & !0xffffu64) | (arch as u64);
                pkt[1] = block as u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count;
                pkt[4] = off1 as u64;
                pkt[5] = off2 as u64;

                exe::call_va!(0x1_41e1_efc0, (pkt.as_mut_ptr() as usize, val_63c));
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_140fa69b0`
/// Original: 0x140fa69b0 (97 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fa69b0(param_1: usize, param_2: usize, param_3: usize) {
    let p1 = rd::<usize>(param_1);
    let p2 = rd::<usize>(param_2);
    let v1 = rd::<u16>(p1);
    let v2 = rd::<u16>(p2);
    if v2 < v1 {
        wr::<u16>(p2, v1);
        wr::<u16>(p1, v2);
    }
    let p3 = rd::<usize>(param_3);
    let p2_b = rd::<usize>(param_2);
    let v3 = rd::<u16>(p3);
    let v2_b = rd::<u16>(p2_b);
    if v3 < v2_b {
        wr::<u16>(p3, v2_b);
        wr::<u16>(p2_b, v3);
        let p1_c = rd::<usize>(param_1);
        let p2_c = rd::<usize>(param_2);
        let v1_c = rd::<u16>(p1_c);
        let v2_c = rd::<u16>(p2_c);
        if v2_c < v1_c {
            wr::<u16>(p2_c, v1_c);
            wr::<u16>(p1_c, v2_c);
        }
    }
}

/// `unknown_141001100`
/// Original: 0x141001100 (306 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141001100(_param_1: usize, param_2: usize, param_3: usize) {
    let count = rd::<u64>(param_2 + 0x18);
    for idx in 0..count {
        let mut pkt = [0u64; 14];
        pkt[0..12].copy_from_slice(&rd::<[u64; 12]>(param_2 + 0x20));
        pkt[12] = rd::<u64>(param_2 + 0x08);
        pkt[13] = idx;
        exe::call_va!(
            0x1_41e8_f2d0,
            (pkt.as_mut_ptr() as usize, rd::<u64>(param_3))
        );
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1401e84a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut INIT_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_8620(dst: usize) {
        INIT_CALLS += 1;
        wr::<u8>(dst + 0x14, 0x77);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8620, mock_8620 as *const ())?;

    let run = |f: F| -> ([u8; 24], [u8; 24], usize, u32) {
        INIT_CALLS = 0;
        let mut dst = [0xaau8; 24];
        let mut src = [0u8; 24];
        let dp = dst.as_mut_ptr() as usize;
        let sp = src.as_mut_ptr() as usize;
        wr::<u64>(sp, 0x1122_3344_5566_7788);
        wr::<u64>(sp + 0x08, 0x99aa_bbcc_ddee_ff00);
        wr::<u32>(sp + 0x10, 0x1234_5678);
        wr::<u8>(sp + 0x14, 0);

        let ret = f(dp, sp);
        (dst, src, ret.wrapping_sub(dp), INIT_CALLS)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1401e84a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1401eacd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PROBE_SLOT: u64 = 0;
    static mut INSERT_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_8680(_p1: usize, out_probe: usize, _p2: usize) -> usize {
        wr::<u64>(out_probe, PROBE_SLOT);
        wr::<u64>(out_probe + 0x08, 0);
        out_probe
    }
    unsafe extern "win64" fn mock_b860(p1: usize, p2: usize) -> usize {
        INSERT_CALLS += 1;
        let buckets = rd::<usize>(p1 + 0x78);
        let dst = buckets + 0x28; // slot 1
        wr::<u128>(dst, rd::<u128>(p2));
        dst
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8680, mock_8680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_b860, mock_b860 as *const ())?;

    let run =
        |f: F, empty_map: bool, probe_slot: u64, free_cnt: u32, flag_b: u8, p5: u8| -> ([u8; 0x50], [u32; 4], u64, u32) {
            PROBE_SLOT = probe_slot;
            INSERT_CALLS = 0;

            let mut buckets = [0u8; 0x50]; // 2 slots of 0x28 bytes
            let bp = buckets.as_mut_ptr() as usize;
            // Pre-populate slot 0 with edx = (1 << 11) | (5 << 5) = 0x8a0 (word 1, bit 5)
            wr::<u16>(bp + 0x10, 0x9999);
            wr::<u8>(bp + 0x12, 0x02);
            wr::<u32>(bp + 0x14, (1u32 << 11) | (5u32 << 5));

            let mut free_list: [u32; 2] = [10, (7u32 << 5)];
            let mut p1 = [0u8; 0x130];
            let p1p = p1.as_mut_ptr() as usize;
            wr::<u64>(p1p + 0x38, 1);
            wr::<u64>(p1p + 0x40, if empty_map { 1 } else { 2 });
            wr::<u64>(p1p + 0x48, 2); // cap = 2
            wr::<usize>(p1p + 0x78, bp);
            wr::<usize>(p1p + 0x100, free_list.as_mut_ptr() as usize);
            wr::<u32>(p1p + 0x108, free_cnt);
            wr::<u32>(p1p + 0x110, 3u32 << 5);

            let key: [u64; 2] = [0x1111_2222, 0x3333_4444];
            let b_val: u8 = flag_b;
            f(
                p1p,
                key.as_ptr() as usize,
                0,
                (&b_val as *const u8) as usize,
                p5,
            );

            // Clear uninitialized stack tail at slot + 0x20 copied by original's 24-byte struct copy
            wr::<u64>(bp + 0x20, 0);
            wr::<u64>(bp + 0x48, 0);

            let meta = [
                rd::<u32>(p1p + 0x108),
                rd::<u32>(p1p + 0x110),
                rd::<u64>(p1p + 0x118) as u32,
                rd::<u64>(p1p + 0x120) as u32,
            ];
            let hi_bits = (rd::<u64>(p1p + 0x118) >> 32) ^ (rd::<u64>(p1p + 0x120) >> 32);
            (buckets, meta, hi_bits, INSERT_CALLS)
        };

    for (empty_map, probe_slot, free_cnt, flag_b, p5) in [
        (false, 0u64, 0u32, 1u8, 0x04u8),      // existing hit at slot 0
        (false, u64::MAX, 2u32, 0u8, 0x08u8),  // miss -> pop from free list
        (true, 0u64, 0u32, 1u8, 0x10u8),       // empty map -> bump +0x110
    ] {
        let o = run(f_o, empty_map, probe_slot, free_cnt, flag_b, p5);
        let n = run(f_n, empty_map, probe_slot, free_cnt, flag_b, p5);
        crate::diff_eq!(
            o,
            n,
            "1401eacd0 empty={} probe={:#x} free={}",
            empty_map,
            probe_slot,
            free_cnt
        );
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14024cc10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TLS_BUF: [u8; 32] = [0; 32];
    static mut MOCK_SIZE: u64 = 0;
    static mut MOCK_COUNT: u64 = 0;
    static mut SWITCH_PTR: bool = false;
    static mut CAPTURED_META: [u64; 5] = [0; 5];

    unsafe extern "win64" fn mock_dfc0() -> usize {
        core::ptr::addr_of_mut!(TLS_BUF) as usize
    }
    unsafe extern "win64" fn mock_1fb0(
        buf_ptr: usize,
        fmt_str_ptr: usize,
        fmt_args_ptr: usize,
        r9: u32,
    ) {
        let vtbl = rd::<u64>(buf_ptr);
        let cap = rd::<u64>(buf_ptr + 0x18);
        let str_len = rd::<u64>(fmt_str_ptr + 0x08);
        let desc = rd::<u64>(fmt_args_ptr);
        let arg_val = rd::<u64>(rd::<usize>(fmt_args_ptr + 0x08));
        CAPTURED_META = [vtbl, cap ^ (r9 as u64), str_len, desc, arg_val];

        wr::<u64>(buf_ptr + 0x10, MOCK_SIZE);
        wr::<u64>(buf_ptr + 0x20, MOCK_COUNT);
        if SWITCH_PTR {
            wr::<usize>(buf_ptr + 0x08, buf_ptr + 0x38);
            wr::<usize>(buf_ptr + 0x30, (core::ptr::addr_of_mut!(TLS_BUF) as usize) + 7);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_423f_dfc0, mock_dfc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4146_1fb0, mock_1fb0 as *const ())?;

    let run = |f: F, sz: u64, cnt: u64, sw: bool| -> ([u8; 32], [u64; 5], usize) {
        TLS_BUF = [0x5au8; 32];
        MOCK_SIZE = sz;
        MOCK_COUNT = cnt;
        SWITCH_PTR = sw;
        CAPTURED_META = [0; 5];

        let p1: [u64; 2] = [0x1234_0000, 14];
        let p2: [u64; 1] = [0xabcd_ef01];
        let ret = f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        let base = core::ptr::addr_of!(TLS_BUF) as usize;
        (TLS_BUF, CAPTURED_META, ret.wrapping_sub(base))
    };

    for (sz, cnt, sw) in [(5u64, 0u64, false), (20, 0x7ff - 3, false), (10, 0, true)] {
        let o = run(f_o, sz, cnt, sw);
        let n = run(f_n, sz, cnt, sw);
        crate::diff_eq!(o, n, "14024cc10 sz={} cnt={} sw={}", sz, cnt, sw);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1403eac30(orig: usize, new: usize) -> Result<u32, String> {
    let poly: [[f32; 2]; 4] = [[-2.0, -2.0], [2.0, -2.0], [2.0, 2.0], [-2.0, 2.0]];
    let mut p1: [u64; 2] = [0; 2];
    let p1p = p1.as_mut_ptr() as usize;
    wr::<usize>(p1p, poly.as_ptr() as usize);
    wr::<u32>(p1p + 0x08, 4);

    // Identity rotation quaternion (0, 0, 0, 1) and translation (1.0, 2.0, 3.0, 0.0)
    let p3: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0];
    let p3p = p3.as_ptr() as usize;

    let call_one = |fn_addr: usize, pt: [f32; 4], min_y: f32, max_y: f32| -> u8 {
        let p2p = pt.as_ptr() as usize;
        #[cfg(not(target_arch = "aarch64"))]
        {
            type F = unsafe extern "win64" fn(usize, usize, usize, f32, f32) -> u8;
            let f: F = core::mem::transmute(fn_addr);
            f(p1p, p2p, p3p, min_y, max_y)
        }
        #[cfg(target_arch = "aarch64")]
        {
            crate::simd_ops::call_va8_xmm(
                fn_addr as u64,
                p1p as u64,
                p2p as u64,
                p3p as u64,
                0,
                max_y.to_bits() as u64,
                0,
                0,
                0,
                [0, 0, 0, min_y.to_bits() as u128],
            )
            .0 as u8
        }
    };

    for (pt, min_y, max_y) in [
        ([1.5f32, 2.5, 3.5, 0.0], -1.0f32, 2.0f32), // local (0.5, 0.5, 0.5) -> inside Y & inside XZ square
        ([5.0f32, 2.5, 3.5, 0.0], -1.0f32, 2.0f32), // local (4.0, 0.5, 0.5) -> inside Y & outside XZ square
        ([1.5f32, 0.0, 3.5, 0.0], -1.0f32, 2.0f32), // local Y = -2.0 < min_y
        ([1.5f32, 5.0, 3.5, 0.0], -1.0f32, 2.0f32), // local Y = 3.0 > max_y
    ] {
        let o = call_one(orig, pt, min_y, max_y);
        let n = call_one(new, pt, min_y, max_y);
        crate::diff_eq!(o, n, "1403eac30 pt={:?}", pt);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_140460c40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_0e20(in_ptr: usize, out_ptr: usize) -> usize {
        let arch = rd::<u16>(in_ptr) as u64;
        let block = rd::<u64>(in_ptr + 0x08);
        let cnt = rd::<u64>(in_ptr + 0x18);
        wr::<u64>(out_ptr, block + arch * 0x10);
        wr::<u64>(out_ptr + 0x08, block + cnt * 0x100);
        wr::<u64>(out_ptr + 0x10, 0xaaaa);
        wr::<u64>(out_ptr + 0x18, 0xbbbb);
        out_ptr
    }
    unsafe extern "win64" fn mock_0d40(_rcx: usize, rdx_pkt: usize, r8_info: usize) {
        let mut pkt = rd::<[u64; 8]>(rdx_pkt);
        let mut info = rd::<[u64; 4]>(r8_info);
        let world = pkt[2] as usize;
        pkt[0] &= 0xffff;
        pkt[2] = 0;
        info[1] = (info[1] as usize).wrapping_sub(world) as u64;
        let dst = world + 0x200;
        wr::<[u64; 8]>(dst, pkt);
        wr::<[u64; 4]>(dst + 64, info);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4046_0e20, mock_0e20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4046_0d40, mock_0d40 as *const ())?;

    let g_4b4 = exe::ptr::<u16>(0x1_43c9_84b4) as usize;
    let g_648 = exe::ptr::<u16>(0x1_43c9_8648) as usize;
    let s_4b4 = rd::<u16>(g_4b4);
    let s_648 = rd::<u16>(g_648);
    wr::<u16>(g_4b4, 1);
    wr::<u16>(g_648, 0);

    let run = |f: F, arch: u16| -> [u64; 12] {
        let lut: [u16; 4] = [0, 1, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x1234_5678);
        wr::<u32>(ap + 0x18, 9);

        let tbl_580e0: [u64; 2] = [0xcafe_babe_1234, 0];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 2 * 8, 0x50_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 3);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 12]>(wp + 0x200)
    };

    for arch in [1u16, 2u16] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        wr::<u16>(g_4b4, s_4b4);
        wr::<u16>(g_648, s_648);
        crate::diff_eq!(o, n, "140460c40 arch={}", arch);
        wr::<u16>(g_4b4, 1);
        wr::<u16>(g_648, 0);
    }
    wr::<u16>(g_4b4, s_4b4);
    wr::<u16>(g_648, s_648);
    Ok(2)
}

pub unsafe fn diff_unknown_1404b9a10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u8; 32], u64) {
        let mut buf = [0xffu8; 32];
        let bp = buf.as_mut_ptr() as usize;
        wr::<u8>(bp + 0x18, 0);
        let ret = f(bp);
        (buf, ret)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1404b9a10 out");
    Ok(1)
}

pub unsafe fn diff_content_14093ec10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    fn mix_log(v: u64) {
        unsafe {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_9fa0(
        _p2: usize,
        _p1: usize,
        s_name: u64,
        crc: u32,
        a5: u32,
        a6: u32,
    ) {
        mix_log(s_name ^ (crc as u64) ^ ((a5 as u64) << 16) ^ (a6 as u64));
    }
    unsafe extern "win64" fn mock_69e0(_p1: usize, _p2: usize) {
        mix_log(0x69e0);
    }
    unsafe extern "win64" fn mock_5850(_s338: usize, dst: usize) {
        wr::<u64>(dst, 0x1111_2222);
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, src: usize, a3: u32, a4: u32) {
        wr::<u64>(dst, rd::<u64>(src + 0x08) ^ ((a3 as u64) << 8) ^ (a4 as u64) ^ 0x9b50);
    }
    unsafe extern "win64" fn mock_3c70(arr_slot: usize, cnt: i64) {
        mix_log(0x3c70 ^ (cnt as u64));
        let _ = arr_slot;
    }
    unsafe extern "win64" fn mock_8620(elem: usize) {
        wr::<u64>(elem, 0x8620);
    }
    unsafe extern "win64" fn mock_84a0(elem: usize, src: usize) -> usize {
        wr::<u64>(elem, rd::<u64>(src) ^ 0x84a0);
        elem
    }
    unsafe extern "win64" fn mock_2d40(_p2: usize) {
        mix_log(0x2d40);
    }
    unsafe extern "win64" fn mock_9e70(_p2: usize) {
        mix_log(0x9e70);
    }
    unsafe extern "win64" fn mock_v00(_this: usize, elem: usize) {
        wr::<u64>(elem + 0x08, 0xbeef);
    }
    unsafe extern "win64" fn mock_v20(_this: usize, dst: usize) {
        wr::<u64>(dst, 0x2020);
    }
    unsafe extern "win64" fn mock_v48(_this: usize, out_cnt: usize) {
        wr::<i32>(out_cnt, 2);
    }
    unsafe extern "win64" fn mock_v80(_this: usize, out_b: usize) {
        wr::<u8>(out_b, 0x5a);
    }
    unsafe extern "win64" fn mock_skip38(_this: usize, bytes: u32, r8: u32) {
        mix_log(0x3800 ^ (bytes as u64) ^ ((r8 as u64) << 8));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4094_69e0, mock_69e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4071_5850, mock_5850 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4094_3c70, mock_3c70 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401e_8620, mock_8620 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4094_2d40, mock_2d40 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl_338 = [0usize; 20];
    vtbl_338[0] = mock_v00 as *const () as usize;
    vtbl_338[0x20 / 8] = mock_v20 as *const () as usize;
    vtbl_338[0x48 / 8] = mock_v48 as *const () as usize;
    vtbl_338[0x80 / 8] = mock_v80 as *const () as usize;
    let obj_338: [usize; 1] = [vtbl_338.as_ptr() as usize];

    let mut vtbl_320 = [0usize; 10];
    vtbl_320[0x38 / 8] = mock_skip38 as *const () as usize;
    let inner_320: [usize; 1] = [vtbl_320.as_ptr() as usize];
    let obj_320: [usize; 2] = [0, inner_320.as_ptr() as usize];

    let run = |f: F, ver: i32, init_def: u8| -> (u64, u64, [u8; 0xc0], [u8; 0x30]) {
        LOG_HASH = 0;
        let mut elems = [0u8; 0x30];
        let mut p1 = [0xeeu8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xa8, elems.as_mut_ptr() as usize);

        let mut p2 = [0u8; 0x370];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x318, 5);
        wr::<usize>(p2p + 0x320, obj_320.as_ptr() as usize);
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, obj_338.as_ptr() as usize);
        wr::<u8>(p2p + 0x369, init_def);

        let ret = f(p1p, p2p, 1);
        wr::<usize>(p1p + 0xa8, 0);
        (ret, LOG_HASH, p1, elems)
    };

    for (ver, init_def) in [(-1i32, 1u8), (4, 1), (9, 0)] {
        let o = run(f_o, ver, init_def);
        let n = run(f_n, ver, init_def);
        crate::diff_eq!(o, n, "14093ec10 ver={} def={}", ver, init_def);
    }
    Ok(3)
}

pub unsafe fn diff_content_140d40150(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    fn mix_log(v: u64) {
        unsafe {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_9fa0(
        _p2: usize,
        _p1: usize,
        s_name: u64,
        crc: u32,
        a5: u32,
        a6: u32,
    ) {
        mix_log(s_name ^ (crc as u64) ^ ((a5 as u64) << 16) ^ (a6 as u64));
    }
    unsafe extern "win64" fn mock_02b0(_p1: usize, _p2: usize) {
        mix_log(0x02b0);
    }
    unsafe extern "win64" fn mock_ac20(_slot: usize, cnt: i64) {
        mix_log(0xac20 ^ (cnt as u64));
    }
    unsafe extern "win64" fn mock_cbc0(_s338: usize, elem: usize) {
        wr::<u64>(elem, 0xcbc0);
    }
    unsafe extern "win64" fn mock_9990(elem: usize, src: usize) {
        wr::<u64>(elem, rd::<u64>(src) ^ 0x9990);
    }
    unsafe extern "win64" fn mock_a530(_p2: usize, dst: usize) {
        wr::<u64>(dst, 0xa530);
    }
    unsafe extern "win64" fn mock_2480(_slot: usize, cnt: i64) {
        mix_log(0x2480 ^ (cnt as u64));
    }
    unsafe extern "win64" fn mock_cc50(_slot: usize, cnt: i64) {
        mix_log(0xcc50 ^ (cnt as u64));
    }
    unsafe extern "win64" fn mock_9e70(_p2: usize) {
        mix_log(0x9e70);
    }
    unsafe extern "win64" fn mock_v38_float(_this: usize, out_f: usize) {
        wr::<u32>(out_f, 0x4000_0000);
    }
    unsafe extern "win64" fn mock_v48(_this: usize, out_u32: usize) {
        wr::<u32>(out_u32, 2);
    }
    unsafe extern "win64" fn mock_v80(_this: usize, out_u8: usize) {
        wr::<u8>(out_u8, 0x33);
    }
    unsafe extern "win64" fn mock_320_v28(_this: usize, _buf: usize, len: u32) {
        mix_log(0x3228 ^ (len as u64));
    }
    unsafe extern "win64" fn mock_320_v38(_this: usize, bytes: u32, r8: u32) {
        mix_log(0x3238 ^ (bytes as u64) ^ ((r8 as u64) << 8));
    }
    unsafe extern "win64" fn mock_340_v28(_this: usize, dst_ptr: u64, byte_len: u64) {
        mix_log(0x3428 ^ dst_ptr ^ byte_len);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_406e_02b0, mock_02b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40d4_ac20, mock_ac20 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_406d_cbc0, mock_cbc0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_404b_9990, mock_9990 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40d4_a530, mock_a530 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40a0_2480, mock_2480 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_406d_cc50, mock_cc50 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl_338 = [0usize; 20];
    vtbl_338[0x38 / 8] = mock_v38_float as *const () as usize;
    vtbl_338[0x48 / 8] = mock_v48 as *const () as usize;
    vtbl_338[0x80 / 8] = mock_v80 as *const () as usize;
    let obj_338: [usize; 1] = [vtbl_338.as_ptr() as usize];

    let mut vtbl_320 = [0usize; 10];
    vtbl_320[0x28 / 8] = mock_320_v28 as *const () as usize;
    vtbl_320[0x38 / 8] = mock_320_v38 as *const () as usize;
    let inner_320: [usize; 1] = [vtbl_320.as_ptr() as usize];
    let obj_320: [usize; 2] = [0, inner_320.as_ptr() as usize];

    let mut vtbl_340 = [0usize; 8];
    vtbl_340[0x28 / 8] = mock_340_v28 as *const () as usize;
    let obj_340: [usize; 1] = [vtbl_340.as_ptr() as usize];

    let run = |f: F, ver: i32, init_def: u8| -> (u64, u64, [u8; 0x80], [u8; 0x40]) {
        LOG_HASH = 0;
        let mut elems38 = [0u8; 0x40];
        let mut p1 = [0xeeu8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x38, elems38.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x48, 0x4800_0000);
        wr::<u64>(p1p + 0x58, 0x5800_0000);

        let mut p2 = [0u8; 0x370];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x318, 5);
        wr::<usize>(p2p + 0x320, obj_320.as_ptr() as usize);
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, obj_338.as_ptr() as usize);
        wr::<usize>(p2p + 0x340, obj_340.as_ptr() as usize);
        wr::<u8>(p2p + 0x369, init_def);

        let ret = f(p1p, p2p, 1);
        wr::<usize>(p1p + 0x38, 0);
        (ret, LOG_HASH, p1, elems38)
    };

    for (ver, init_def) in [(1i32, 1u8), (10, 1), (0x1a, 0)] {
        let o = run(f_o, ver, init_def);
        let n = run(f_n, ver, init_def);
        crate::diff_eq!(o, n, "140d40150 ver={} def={}", ver, init_def);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140f2daa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7c80() -> u32 {
        2
    }
    unsafe extern "win64" fn mock_2180(
        a1: u64,
        a2: u64,
        a3_world: usize,
        a4_world: usize,
        a5_ptr: usize,
        a6_worker_ptr: usize,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
        a11: u64,
    ) {
        let mut a5 = rd::<[u64; 3]>(a5_ptr);
        a5[0] = (a5[0] as usize).wrapping_sub(a3_world) as u64;
        let dst = a3_world + 0x200;
        wr::<u64>(dst, a1);
        wr::<u64>(dst + 8, a2);
        wr::<u64>(dst + 16, a4_world.wrapping_sub(a3_world) as u64);
        wr::<[u64; 3]>(dst + 24, a5);
        wr::<u64>(dst + 48, a6_worker_ptr.wrapping_sub(a3_world) as u64);
        wr::<[u64; 5]>(dst + 56, [a7, a8, a9, a10, a11]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41dc_2180, mock_2180 as *const ())?;

    let g_e9c = exe::ptr::<u16>(0x1_43c8_ce9c) as usize;
    let g_114 = exe::ptr::<u16>(0x1_43c8_d114) as usize;
    let g_0cc = exe::ptr::<u16>(0x1_43c8_d0cc) as usize;
    let g_0c0 = exe::ptr::<u16>(0x1_43c8_d0c0) as usize;
    let g_170 = exe::ptr::<u16>(0x1_43c8_d170) as usize;
    let g_080 = exe::ptr::<u16>(0x1_43c8_d080) as usize;
    let g_014 = exe::ptr::<u16>(0x1_43c8_d014) as usize;
    let g_018 = exe::ptr::<u16>(0x1_43c8_d018) as usize;
    let saved = (
        rd::<u16>(g_e9c),
        rd::<u16>(g_114),
        rd::<u16>(g_0cc),
        rd::<u16>(g_0c0),
        rd::<u16>(g_170),
        rd::<u16>(g_080),
        rd::<u16>(g_014),
        rd::<u16>(g_018),
    );
    wr::<u16>(g_e9c, 0);
    wr::<u16>(g_114, 1);
    wr::<u16>(g_0cc, 1);
    wr::<u16>(g_0c0, 2);
    wr::<u16>(g_170, 3);
    wr::<u16>(g_080, 4);
    wr::<u16>(g_014, 5);
    wr::<u16>(g_018, 6);

    let run = |f: F| -> [u64; 12] {
        let lut: [u16; 4] = [0, 1, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x9999_8888);
        wr::<u32>(ap + 0x18, 12);

        let tbl_580e0: [u64; 8] = [0x101, 0x202, 0x303, 0x404, 0x505, 0x606, 0x707, 0];
        let mut world = vec![0u8; 0x58600];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize);
        rd::<[u64; 12]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_e9c, saved.0);
    wr::<u16>(g_114, saved.1);
    wr::<u16>(g_0cc, saved.2);
    wr::<u16>(g_0c0, saved.3);
    wr::<u16>(g_170, saved.4);
    wr::<u16>(g_080, saved.5);
    wr::<u16>(g_014, saved.6);
    wr::<u16>(g_018, saved.7);

    crate::diff_eq!(o, n, "140f2daa0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_140f79790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x7770 && n == 6 {
            match want {
                10 => 0,
                20 => 1,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_efc0(pkt_ptr: usize, rdx_63c: u64) {
        let mut pkt = rd::<[u64; 6]>(pkt_ptr);
        let world = pkt[2] as usize;
        pkt[0] &= 0xffff;
        pkt[2] = 0;
        let dst = world + 0x200;
        wr::<[u64; 6]>(dst, pkt);
        wr::<u64>(dst + 48, rdx_63c);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e1_efc0, mock_efc0 as *const ())?;

    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_d00 = exe::ptr::<u16>(0x1_43cc_6d00) as usize;
    let g_87c = exe::ptr::<u16>(0x1_43c8_d87c) as usize;
    let s_63c = rd::<u16>(g_63c);
    let s_d00 = rd::<u16>(g_d00);
    let s_87c = rd::<u16>(g_87c);
    wr::<u16>(g_63c, 1);
    wr::<u16>(g_d00, 10);
    wr::<u16>(g_87c, 20);

    let run = |f: F| -> [u64; 7] {
        let off_tbl: [u32; 2] = [0x140, 0x280];
        let tbl_580e0: [u64; 2] = [0, 0xabcd_1234_5678];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x60_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 4);
        wr::<u64>(wp + 0x18098 + 0x20, 0x7770);
        wr::<u32>(wp + 0x180ac + 0x20, 6);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 7]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_63c, s_63c);
    wr::<u16>(g_d00, s_d00);
    wr::<u16>(g_87c, s_87c);

    crate::diff_eq!(o, n, "140f79790 out");
    Ok(1)
}

pub unsafe fn diff_unknown_140fa69b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, vals: [u16; 3]| -> [u16; 3] {
        let mut v = vals;
        let p0 = (&mut v[0] as *mut u16) as usize;
        let p1 = (&mut v[1] as *mut u16) as usize;
        let p2 = (&mut v[2] as *mut u16) as usize;
        f(
            (&p0 as *const usize) as usize,
            (&p1 as *const usize) as usize,
            (&p2 as *const usize) as usize,
        );
        v
    };

    for perm in [
        [10u16, 20, 30],
        [10, 30, 20],
        [20, 10, 30],
        [20, 30, 10],
        [30, 10, 20],
        [30, 20, 10],
    ] {
        let o = run(f_o, perm);
        let n = run(f_n, perm);
        crate::diff_eq!(o, n, "140fa69b0 perm={:?}", perm);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_141001100(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    unsafe extern "win64" fn mock_f2d0(pkt_ptr: usize, rdx_val: u64) {
        let pkt = rd::<[u64; 14]>(pkt_ptr);
        for &w in &pkt {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(w);
        }
        LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(rdx_val);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e8_f2d0, mock_f2d0 as *const ())?;

    let run = |f: F, count: u64| -> u64 {
        LOG_HASH = 0;
        let mut p2 = [0u64; 16];
        p2[1] = 0x9999_0001; // +0x08
        p2[3] = count; // +0x18
        for i in 0..12 {
            p2[4 + i] = 0x1000 + i as u64; // +0x20..+0x80
        }
        let p3: u64 = 0xdead_beef_cafe;
        f(0, p2.as_ptr() as usize, (&p3 as *const u64) as usize);
        LOG_HASH
    };

    for cnt in [0u64, 3u64] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141001100 cnt={}", cnt);
    }
    Ok(2)
}

// REGISTRY (for games/alan-wake-2/crates/aw2-game/src/registry/core.rs):
// Replacement::verified(0x1_401e_84a0, "unknown_1401e84a0", rend::cauldron_hot_x1::unknown_1401e84a0 as *const (), rend::cauldron_hot_x1::diff_unknown_1401e84a0),
// Replacement::verified_throws(0x1_401e_acd0, "unknown_1401eacd0", rend::cauldron_hot_x1::unknown_1401eacd0 as *const (), Throws::Discard, rend::cauldron_hot_x1::diff_unknown_1401eacd0),
// Replacement::verified_throws(0x1_4024_cc10, "unknown_14024cc10", rend::cauldron_hot_x1::unknown_14024cc10 as *const (), Throws::Caught, rend::cauldron_hot_x1::diff_unknown_14024cc10),
// Replacement::verified(0x1_403e_ac30, "unknown_1403eac30", rend::cauldron_hot_x1::unknown_1403eac30 as *const (), rend::cauldron_hot_x1::diff_unknown_1403eac30),
// Replacement::verified(0x1_4046_0c40, "unknown_140460c40", rend::cauldron_hot_x1::unknown_140460c40 as *const (), rend::cauldron_hot_x1::diff_unknown_140460c40),
// Replacement::verified(0x1_404b_9a10, "unknown_1404b9a10", rend::cauldron_hot_x1::unknown_1404b9a10 as *const (), rend::cauldron_hot_x1::diff_unknown_1404b9a10),
// Replacement::verified_throws(0x1_4093_ec10, "content_14093ec10", rend::cauldron_hot_x1::content_14093ec10 as *const (), Throws::Discard, rend::cauldron_hot_x1::diff_content_14093ec10),
// Replacement::verified_throws(0x1_40d4_0150, "content_140d40150", rend::cauldron_hot_x1::content_140d40150 as *const (), Throws::Discard, rend::cauldron_hot_x1::diff_content_140d40150),
// Replacement::verified_throws(0x1_40f2_daa0, "unknown_140f2daa0", rend::cauldron_hot_x1::unknown_140f2daa0 as *const (), Throws::Caught, rend::cauldron_hot_x1::diff_unknown_140f2daa0),
// Replacement::verified(0x1_40f7_9790, "unknown_140f79790", rend::cauldron_hot_x1::unknown_140f79790 as *const (), rend::cauldron_hot_x1::diff_unknown_140f79790),
// Replacement::verified(0x1_40fa_69b0, "unknown_140fa69b0", rend::cauldron_hot_x1::unknown_140fa69b0 as *const (), rend::cauldron_hot_x1::diff_unknown_140fa69b0),
// Replacement::verified_throws(0x1_4100_1100, "unknown_141001100", rend::cauldron_hot_x1::unknown_141001100 as *const (), Throws::Discard, rend::cauldron_hot_x1::diff_unknown_141001100),
