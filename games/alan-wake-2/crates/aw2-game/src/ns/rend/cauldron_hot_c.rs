//! Decompiled `rend` Cauldron / visibility / LOD hot functions:
//! - `0x1420cd680` (`unknown_1420cd680`)
//! - `0x1420ceef0` (`unknown_1420ceef0`)
//! - `0x1420d2c00` (`unknown_1420d2c00`)
//! - `0x1420d95d0` (`unknown_1420d95d0`)
//! - `0x1420df870` (`unknown_1420df870`)
//! - `0x142258f90` (`r_142258f90`)
//! - `0x1422611b0` (`unknown_1422611b0`)
//! - `0x142307f90` (`unknown_142307f90`)

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vrcpps_128(v: [f32; 4]) -> [f32; 4] {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_loadu_ps, _mm_rcp_ps, _mm_storeu_ps};
        let mut out = [0.0f32; 4];
        let r = _mm_rcp_ps(_mm_loadu_ps(v.as_ptr()));
        _mm_storeu_ps(out.as_mut_ptr(), r);
        out
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        [1.0 / v[0], 1.0 / v[1], 1.0 / v[2], 1.0 / v[3]]
    }
}

#[inline(always)]
fn vdpps_7f(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_dp_ps, _mm_loadu_ps, _mm_storeu_ps};
        let mut out = [0.0f32; 4];
        let r = _mm_dp_ps(_mm_loadu_ps(a.as_ptr()), _mm_loadu_ps(b.as_ptr()), 0x7f);
        _mm_storeu_ps(out.as_mut_ptr(), r);
        out
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let s = (a[0] * b[0] + a[1] * b[1]) + a[2] * b[2];
        [s, s, s, s]
    }
}

#[inline(always)]
fn vfmsub231_ps(acc: [f32; 4], a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[0].mul_add(b[0], -acc[0]),
        a[1].mul_add(b[1], -acc[1]),
        a[2].mul_add(b[2], -acc[2]),
        a[3].mul_add(b[3], -acc[3]),
    ]
}

#[inline(always)]
fn vfmadd231_ps(acc: [f32; 4], a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[0].mul_add(b[0], acc[0]),
        a[1].mul_add(b[1], acc[1]),
        a[2].mul_add(b[2], acc[2]),
        a[3].mul_add(b[3], acc[3]),
    ]
}

#[inline(always)]
fn vmul_ps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

#[inline(always)]
fn vadd_ps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[inline(always)]
fn vsub_ps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

#[inline(always)]
fn vmax_ps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        if a[0] > b[0] { a[0] } else { b[0] },
        if a[1] > b[1] { a[1] } else { b[1] },
        if a[2] > b[2] { a[2] } else { b[2] },
        if a[3] > b[3] { a[3] } else { b[3] },
    ]
}

#[inline(always)]
fn vsqrt_ps(a: [f32; 4]) -> [f32; 4] {
    [a[0].sqrt(), a[1].sqrt(), a[2].sqrt(), a[3].sqrt()]
}

/// `unknown_1420cd680`
/// Original: 0x1420cd680 (932 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420cd680(
    param_1: usize,
    param_2: usize,
    param_3: u16,
    param_4: u8,
) -> u8 {
    let fn_get_tid = rd::<usize>(exe::ptr::<usize>(0x1_4264_5790) as usize);
    let tid = exe::call_va!(fn_get_tid, () -> u32);
    if rd::<u32>(param_2 + 0x40) != tid {
        let fn_acq = rd::<usize>(exe::ptr::<usize>(0x1_4264_55a8) as usize);
        exe::call_va!(fn_acq, (param_2 + 0x38));
        wr::<u32>(param_2 + 0x40, tid);
    }
    wr::<u32>(param_2 + 0x44, rd::<u32>(param_2 + 0x44).wrapping_add(1));

    let (slot1, idx2) = if (param_4 as i8) < 0 {
        let idx1 = if rd::<u8>(param_1 + 0x4fd8) != 0 {
            (((rd::<u64>(param_2 + 0x140).wrapping_sub(1)) as u32 & 1) as usize) + 4
        } else {
            ((rd::<u64>(param_2 + 0x138).wrapping_sub(1)) as u32 & 3) as usize
        };
        let s1 = rd::<[u8; 0x28]>(param_2 + 0x48 + idx1 * 0x28);
        let flag = rd::<u8>(param_1 + 0x4fd8);
        let i2 = if flag != 0 {
            ((rd::<u32>(param_2 + 0x140) & 1) as usize) + 4
        } else {
            ((rd::<u64>(param_2 + 0x138).wrapping_sub(2)) as u32 & 3) as usize
        };
        (s1, i2)
    } else {
        let idx1 = (rd::<u32>(param_2 + 0x138) & 3) as usize;
        let s1 = rd::<[u8; 0x28]>(param_2 + 0x48 + idx1 * 0x28);
        let flag = rd::<u8>(param_1 + 0x4fd8);
        let i2 = if flag != 0 {
            (((rd::<u64>(param_2 + 0x140).wrapping_sub(1)) as u32 & 1) as usize) + 4
        } else {
            ((rd::<u64>(param_2 + 0x138).wrapping_sub(1)) as u32 & 3) as usize
        };
        (s1, i2)
    };

    let slot2 = rd::<[u8; 0x28]>(param_2 + 0x48 + idx2 * 0x28);
    let s1_ptr = slot1.as_ptr() as usize;
    let s2_ptr = slot2.as_ptr() as usize;

    let eps_pos = rd::<f32>(exe::ptr::<f32>(0x1_4342_6460) as usize);
    let eps_neg = rd::<f32>(exe::ptr::<f32>(0x1_4342_7000) as usize);

    let res: u8 = match param_3 {
        0x83 => {
            let dx = rd::<f32>(s1_ptr) - rd::<f32>(s2_ptr);
            let dy = rd::<f32>(s1_ptr + 0x04) - rd::<f32>(s2_ptr + 0x04);
            let dist = (dy * dy + dx * dx).sqrt();
            (dist >= eps_pos) as u8
        }
        0x84 => {
            let diff = (rd::<f32>(s1_ptr) - rd::<f32>(s2_ptr)).abs();
            (diff >= eps_pos) as u8
        }
        0x85 => {
            let diff = (rd::<f32>(s1_ptr + 0x04) - rd::<f32>(s2_ptr + 0x04)).abs();
            (diff >= eps_pos) as u8
        }
        0x89 => slot1[0x20],
        0x8a => slot1[0x21],
        0x8b => slot1[0x22],
        0x8c => slot1[0x23],
        0x8d => slot1[0x24],
        0x8e => {
            let diff = (rd::<f32>(s1_ptr + 0x18) - rd::<f32>(s2_ptr + 0x18)).abs();
            (diff >= eps_pos) as u8
        }
        0x8f => {
            let diff = rd::<f32>(s1_ptr + 0x18) - rd::<f32>(s2_ptr + 0x18);
            (diff >= eps_pos) as u8
        }
        0x90 => {
            let diff = rd::<f32>(s1_ptr + 0x18) - rd::<f32>(s2_ptr + 0x18);
            (eps_neg >= diff) as u8
        }
        0x91 => {
            let diff = (rd::<f32>(s1_ptr + 0x1c) - rd::<f32>(s2_ptr + 0x1c)).abs();
            (diff >= eps_pos) as u8
        }
        0x92 => {
            let diff = rd::<f32>(s1_ptr + 0x1c) - rd::<f32>(s2_ptr + 0x1c);
            (eps_neg >= diff) as u8
        }
        0x93 => {
            let diff = rd::<f32>(s1_ptr + 0x1c) - rd::<f32>(s2_ptr + 0x1c);
            (diff >= eps_pos) as u8
        }
        _ => 0,
    };

    let new_cnt = rd::<u32>(param_2 + 0x44).wrapping_sub(1);
    wr::<u32>(param_2 + 0x44, new_cnt);
    if new_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32((param_2 + 0x40) as u64, u32::MAX);
        let fn_rel = rd::<usize>(exe::ptr::<usize>(0x1_4264_5598) as usize);
        exe::call_va!(fn_rel, (param_2 + 0x38));
    }
    res
}

/// `unknown_1420ceef0`
/// Original: 0x1420ceef0 (250 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420ceef0(
    param_1: usize,
    param_2: usize,
    param_3: u16,
) -> u8 {
    let edi = param_3 as usize;
    let fn_get_tid = rd::<usize>(exe::ptr::<usize>(0x1_4264_5790) as usize);
    let tid = exe::call_va!(fn_get_tid, () -> u32);
    if rd::<u32>(param_2 + 0x10) != tid {
        let fn_acq = rd::<usize>(exe::ptr::<usize>(0x1_4264_55a8) as usize);
        exe::call_va!(fn_acq, (param_2 + 0x08));
        wr::<u32>(param_2 + 0x10, tid);
    }
    wr::<u32>(param_2 + 0x14, rd::<u32>(param_2 + 0x14).wrapping_add(1));

    let v0 = rd::<u64>(param_2 + 0x240);
    let off0 = ((v0 % 3) as usize) * 0x81 + edi;
    let b0 = rd::<u8>(param_2 + off0 + 0x37) & 1;

    let off1 = if rd::<u8>(param_1 + 0x4fd8) != 0 {
        0x183usize
    } else {
        let v1 = rd::<u64>(param_2 + 0x240).wrapping_add(2);
        ((v1 % 3) as usize) * 0x81
    };
    let b1 = rd::<u8>(param_2 + edi + off1 + 0x37) & 1;
    let res = b1 ^ b0;

    let new_cnt = rd::<u32>(param_2 + 0x14).wrapping_sub(1);
    wr::<u32>(param_2 + 0x14, new_cnt);
    if new_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32((param_2 + 0x10) as u64, u32::MAX);
        let fn_rel = rd::<usize>(exe::ptr::<usize>(0x1_4264_5598) as usize);
        exe::call_va!(fn_rel, (param_2 + 0x08));
    }
    res
}

/// `unknown_1420d2c00`
/// Original: 0x1420d2c00 (177 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420d2c00(
    param_1: usize,
    param_2: u16,
    param_3: u8,
) -> u8 {
    let r11w = param_2.wrapping_sub(1);
    let r10w = param_2.wrapping_sub(0x83);
    let dx = param_2.wrapping_sub(0x95);

    if r11w > 0x80 && r10w > 0x10 && dx > 0x24 {
        return 0;
    }

    let mut r9b = (param_3 >> 4) & 1;
    let mut cl = (param_3 >> 5) & 1;

    if (param_3 & 8) != 0 {
        if r9b == 0 && rd::<u8>(param_1 + 0x5860) == 1 {
            r9b = 1;
        }
        if cl == 0 && rd::<u8>(param_1 + 0x5860) == 0 {
            cl = 1;
        }
        if r9b != 0 && r11w > 0x80 && r10w > 0x10 {
            return 1;
        }
        if cl != 0 && dx > 0x24 {
            return 1;
        }
        return 0;
    }

    if r9b != 0 {
        if r11w > 0x80 && r10w > 0x10 {
            return 1;
        }
        if cl != 0 && dx > 0x24 {
            return 1;
        }
        return 0;
    }

    if cl != 0 && dx > 0x24 {
        return 1;
    }
    0
}

/// `unknown_1420d95d0`
/// Original: 0x1420d95d0 (726 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420d95d0(param_1: usize, param_2: u16) -> u8 {
    let mut bx = param_2;
    if (bx & 0x4000) != 0 {
        let rdi_base = rd::<usize>(param_1 + 0x08);
        bx &= 0xbfff;
        let limit = rd::<u32>(rdi_base + 0x5888) as u64;
        let lock = rdi_base + 0x5aa0;

        let fn_get_tid = rd::<usize>(exe::ptr::<usize>(0x1_4264_5790) as usize);
        let tid = exe::call_va!(fn_get_tid, () -> u32);
        if rd::<u32>(lock + 0x08) != tid {
            let fn_acq = rd::<usize>(exe::ptr::<usize>(0x1_4264_55a8) as usize);
            exe::call_va!(fn_acq, (lock));
            wr::<u32>(lock + 0x08, tid);
        }
        let cnt = rd::<u32>(lock + 0x0c);
        wr::<u32>(lock + 0x0c, cnt);
        let idx = bx.wrapping_sub(0xce) as u64;
        if cnt == 0 {
            crate::simd_ops::atomic_xchg_u32((lock + 0x08) as u64, u32::MAX);
            let fn_rel = rd::<usize>(exe::ptr::<usize>(0x1_4264_5598) as usize);
            exe::call_va!(fn_rel, (lock));
        }

        if idx >= limit {
            return 0;
        }

        let mut buf1 = [0.0f32; 3];
        let mut buf2 = [0.0f32; 3];
        let p1 = exe::call_va!(
            0x1_420d_bdb0,
            (buf1.as_mut_ptr() as usize, param_1, bx as u32, 0u8) -> usize
        );
        let x1 = rd::<f32>(p1);
        let y1 = rd::<f32>(p1 + 0x04);
        let z1 = rd::<f32>(p1 + 0x08);

        let p2 = exe::call_va!(
            0x1_420d_bdb0,
            (buf2.as_mut_ptr() as usize, param_1, bx as u32, 0x80u8) -> usize
        );
        let x2 = rd::<f32>(p2);
        let y2 = rd::<f32>(p2 + 0x04);
        let z2 = rd::<f32>(p2 + 0x08);

        let sq1 = (y1 * y1 + x1 * x1) + z1 * z1;
        let sq2 = (y2 * y2 + x2 * x2) + z2 * z2;
        return ((sq1 > 0.0f32) as u8) ^ ((sq2 > 0.0f32) as u8);
    }

    let rsi = rd::<usize>(param_1);
    if exe::call_va!(0x1_420d_f870, (rsi, bx) -> u8) != 0 {
        return 0;
    }

    if bx.wrapping_sub(0x95) <= 0x24 {
        let rdi = rd::<usize>(rsi + 0x60);
        if rdi == 0 {
            return 0;
        }
        let rsi_0 = rd::<usize>(rsi);
        let bpl = if rd::<u64>(rdi + 0x38) != 0 {
            exe::call_va!(0x1_420d_1240, (rsi_0, rdi, bx as u32, 0x80u8) -> u8)
        } else if rd::<u64>(rdi + 0x40) != 0 {
            exe::call_va!(0x1_420d_55d0, (rsi_0, rdi, bx as u32, 0x80u8) -> u8)
        } else {
            0
        };

        let al = if rd::<u64>(rdi + 0x38) != 0 {
            exe::call_va!(0x1_420d_1240, (rsi_0, rdi, bx as u32, 0u8) -> u8)
        } else if rd::<u64>(rdi + 0x40) != 0 {
            exe::call_va!(0x1_420d_55d0, (rsi_0, rdi, bx as u32, 0u8) -> u8)
        } else {
            0
        };
        return al ^ bpl;
    }

    if bx.wrapping_sub(1) <= 0x80 {
        let rbp = rd::<usize>(rsi + 0x48);
        if rbp == 0 {
            return 0;
        }
        let rsi_0 = rd::<usize>(rsi);
        let dil = exe::call_va!(0x1_420c_eff0, (rsi_0, rbp, bx as u32, 0u8) -> u8);
        let al = exe::call_va!(0x1_420c_eff0, (rsi_0, rbp, bx as u32, 0x80u8) -> u8);
        return al ^ dil;
    }

    if bx.wrapping_sub(0x83) <= 0x10 {
        let rbp = rd::<usize>(rsi + 0x50);
        if rbp == 0 {
            return 0;
        }
        let rsi_0 = rd::<usize>(rsi);
        let dil = exe::call_va!(0x1_420c_d680, (rsi_0, rbp, bx as u32, 0u8) -> u8);
        let al = exe::call_va!(0x1_420c_d680, (rsi_0, rbp, bx as u32, 0x80u8) -> u8);
        return al ^ dil;
    }

    0
}

/// `unknown_1420df870`
/// Original: 0x1420df870 (126 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420df870(param_1: usize, param_2: u16) -> u8 {
    let sub_ptr = if param_2.wrapping_sub(0x95) <= 0x24 {
        rd::<usize>(param_1 + 0x60)
    } else if param_2.wrapping_sub(1) <= 0x80 {
        rd::<usize>(param_1 + 0x48)
    } else if param_2.wrapping_sub(0x83) <= 0x10 {
        rd::<usize>(param_1 + 0x50)
    } else if param_2.wrapping_sub(0xbb) <= 0x07 {
        rd::<usize>(param_1 + 0x58)
    } else {
        return 0;
    };

    if sub_ptr != 0 && exe::call_va!(0x1_411f_4330, (sub_ptr, param_2) -> u8) != 0 {
        1
    } else {
        0
    }
}

/// `r_142258f90`
/// Original: 0x142258f90 (597 bytes, Throws::No)
pub unsafe extern "win64" fn r_142258f90() {
    let g_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let count = rd::<u32>(g_mgr + 0x258) as usize;
    let arr_250 = rd::<usize>(g_mgr + 0x250);
    let arr_270 = rd::<usize>(g_mgr + 0x270);
    let arr_2a0 = rd::<usize>(g_mgr + 0x2a0);

    if count == 0 {
        return;
    }

    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);

    for i in 0..count {
        let entry = arr_250 + i * 0x18;
        let shift = (rd::<u8>(arr_2a0 + i) & 0x1f) as u32;
        let n = rd::<u16>(entry) as usize;
        let dst_base = rd::<usize>(entry + 0x08);
        let src_base = rd::<usize>(entry + 0x10);
        let xmm6 = rd::<f32>(arr_270 + i * 4);
        let mask = 0xffff_ffffu32.wrapping_shl(shift);
        let xmm5 = (rd::<u8>(entry + 0x02) as u32) as f32;

        for j in 0..n {
            let dst_ptr = dst_base + j * 4;
            let src_ptr = src_base + j * 8;
            wr::<f32>(dst_ptr, xmm5);
            if (rd::<u32>(src_ptr + 4) & mask) != 0 {
                let val = (xmm6 + rd::<f32>(src_ptr)) * c_half + c_one;
                let mx = if xmm5 > val { xmm5 } else { val };
                wr::<f32>(dst_ptr, mx);
            }
        }
    }
}

/// `unknown_1422611b0`
/// Original: 0x1422611b0 (860 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1422611b0(param_1: usize) {
    let count_48 = rd::<u64>(param_1 + 0x48);
    let ptr_5c8 = rd::<usize>(param_1 + 0x5c8);
    let ptr_198 = rd::<usize>(param_1 + 0x198);
    let ptr_18 = rd::<usize>(param_1 + 0x18);
    let mut rsi = rd::<usize>(param_1 + 0x38);
    let ptr_180 = rd::<usize>(param_1 + 0x180);

    let num_words = count_48.wrapping_add(0x3f) >> 6;
    if num_words == 0 {
        return;
    }

    let diff_180_38 = ptr_180.wrapping_sub(rsi);
    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_8240) as usize);

    let mut rdi: usize = 0;
    for _ in 0..num_words {
        let mut rbx = rd::<u64>(rsi.wrapping_add(diff_180_38)) | rd::<u64>(rsi);
        while rbx != 0 {
            let tz = rbx.trailing_zeros() as usize;
            rbx ^= 1u64 << tz;
            let rdx = tz + rdi;

            if rd::<i32>(ptr_198 + rdx * 4) == -1 {
                continue;
            }
            let r9 = rd::<usize>(ptr_18 + rdx * 8);
            let idx_50 = rd::<u32>(r9 + 0x50);
            if idx_50 == 0xffff_ffff {
                continue;
            }
            let g_ed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
            let arr_100 = rd::<usize>(g_ed0 + 0x100);
            if rd::<i32>(arr_100 + (idx_50 as usize) * 4) == -1 {
                continue;
            }

            let g_f48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
            let r10 = rd::<usize>(r9 + 0x08);
            let idx_48 = rd::<u32>(r9 + 0x48) as usize;
            let arr_1a8 = rd::<usize>(g_f48 + 0x1a8);
            let cl = rd::<u8>(r10 + 0x272);
            if cl == 0 {
                continue;
            }

            let r8d = rd::<u8>(arr_1a8 + idx_48) as i32;
            let eax = (cl as i32) - 1;
            let lod_idx = if eax < r8d { eax } else { r8d } as isize;
            if rd::<i32>((r10 as isize + lod_idx * 4 + 0x1c8) as usize) <= 0 {
                continue;
            }

            let r9_0 = rd::<usize>(r9);
            if rd::<u64>(r9_0 + 0xc8) != 0 {
                continue;
            }

            if (rd::<u32>(r9 + 0x4c) & 0x4000) != 0 {
                exe::call_va!(0x1_4230_7f90, (r9, r9 + 0x6c));
            } else {
                let mat_base = ptr_5c8 + rdx * 0x30;
                let vec_ptr = rd::<usize>(param_1 + 0xe8) + rdx * 0x10;

                let row2 = rd::<[f32; 4]>(mat_base + 0x20);
                let xmm11 = rd::<[f32; 4]>(vec_ptr);
                let row1 = rd::<[f32; 4]>(mat_base + 0x10);
                let row0 = rd::<[f32; 4]>(mat_base);

                let xmm9 = [row0[0], row0[1], row0[2], 0.0];
                let xmm8 = [row0[3], row1[0], row1[1], 0.0];
                let xmm4 = [row1[2], row1[3], row2[0], 0.0];
                let xmm10 = [row2[1], row2[2], row2[3], c_one];

                let xmm2 = [xmm9[1], xmm9[2], xmm9[0], xmm9[3]];
                let mut xmm6 = vmul_ps(xmm8, xmm2);
                let xmm0 = [xmm4[2], xmm4[0], xmm4[1], xmm4[3]];
                let mut xmm3 = vmul_ps(xmm0, xmm8);
                let xmm1 = [xmm8[2], xmm8[0], xmm8[1], xmm8[3]];
                xmm3 = vfmsub231_ps(xmm3, xmm4, xmm1);

                let xmm0_b = [xmm4[1], xmm4[2], xmm4[0], xmm4[3]];
                let mut xmm1_b = vmul_ps(xmm0_b, xmm9);
                xmm1_b = vfmsub231_ps(xmm1_b, xmm4, xmm2);

                let xmm5 = [xmm3[2], xmm3[0], xmm1_b[1], xmm1_b[2]];
                let xmm7 = [xmm3[1], xmm3[1], xmm1_b[0], xmm1_b[0]];
                let xmm0_c = [xmm8[1], xmm8[2], xmm8[0], xmm8[3]];
                xmm6 = vfmsub231_ps(xmm6, xmm9, xmm0_c);

                let xmm0_d = [xmm5[0], xmm5[1], xmm7[0], xmm7[1]];
                let det = vdpps_7f(xmm9, xmm0_d);
                let rcp = vrcpps_128(det);
                let rcp_sq = vmul_ps(rcp, rcp);
                let rcp_sq_det = vmul_ps(rcp_sq, det);
                let two_rcp = vadd_ps(rcp, rcp);
                let inv_det = vsub_ps(two_rcp, rcp_sq_det);
                let xmm4_inv = [inv_det[0], inv_det[1], inv_det[2], 0.0];

                let shuf_d8 = [xmm5[0], xmm5[2], xmm6[1], xmm6[3]];
                let xmm9_i = vmul_ps(shuf_d8, xmm4_inv);
                let shuf_2d = [xmm5[1], xmm5[3], xmm6[2], xmm6[0]];
                let xmm8_i = vmul_ps(shuf_2d, xmm4_inv);
                let shuf_88 = [xmm7[0], xmm7[2], xmm6[0], xmm6[2]];
                let xmm6_i = vmul_ps(shuf_88, xmm4_inv);

                let dp0 = vdpps_7f(xmm9_i, xmm9_i);
                let dp1 = vdpps_7f(xmm8_i, xmm8_i);
                let mx01 = vmax_ps(dp0, dp1);
                let dp2 = vdpps_7f(xmm6_i, xmm6_i);
                let mx012 = vmax_ps(mx01, dp2);
                let scale = vsqrt_ps(mx012);
                let w_bcast = [xmm11[3]; 4];
                let xmm5_w = vmul_ps(scale, w_bcast);

                let t_z = [xmm10[2]; 4];
                let mut acc = vmul_ps(t_z, xmm6_i);
                let t_y = [xmm10[1]; 4];
                acc = vfmadd231_ps(acc, t_y, xmm8_i);
                let t_x = [xmm10[0]; 4];
                acc = vfmadd231_ps(acc, t_x, xmm9_i);

                let mut out_v = [
                    f32::from_bits(acc[0].to_bits() ^ 0x8000_0000),
                    f32::from_bits(acc[1].to_bits() ^ 0x8000_0000),
                    f32::from_bits(acc[2].to_bits() ^ 0x8000_0000),
                    f32::from_bits(acc[3].to_bits() ^ 0x8000_0000),
                ];
                let p_z = [xmm11[2]; 4];
                out_v = vfmadd231_ps(out_v, p_z, xmm6_i);
                let p_y = [xmm11[1]; 4];
                out_v = vfmadd231_ps(out_v, p_y, xmm8_i);
                let p_x = [xmm11[0]; 4];
                out_v = vfmadd231_ps(out_v, p_x, xmm9_i);

                let mut local_sphere = [out_v[0], out_v[1], out_v[2], xmm5_w[3]];
                exe::call_va!(0x1_4230_7f90, (r9, local_sphere.as_mut_ptr() as usize));
            }
        }
        rsi = rsi.wrapping_add(8);
        rdi = rdi.wrapping_add(0x40);
    }
}

/// `unknown_142307f90`
/// Original: 0x142307f90 (412 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142307f90(param_1: usize, param_2: usize) {
    let idx_48 = rd::<u32>(param_1 + 0x48) as usize;
    let g_f48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let arr_1a8 = rd::<usize>(g_f48 + 0x1a8);
    let r12d = rd::<u8>(arr_1a8 + idx_48) as u32;

    let mut rax_8 = rd::<usize>(param_1 + 0x08);
    if rd::<i32>(rax_8 + 0x138) <= 0 {
        return;
    }

    let mut ebx: i32 = 0;
    loop {
        let arr_130 = rd::<usize>(rax_8 + 0x130);
        let rdx_elem = rd::<usize>((arr_130 as isize + (ebx as isize) * 8) as usize);
        let r8d = rd::<u32>(rdx_elem + 0xa8);
        if r12d == r8d {
            let ecx: u32 = if rd::<i32>(rdx_elem + 0xcc) <= 0 {
                rd::<u32>(rdx_elem + 0xd8)
            } else {
                let g_ed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
                let idx_50 = rd::<u32>(param_1 + 0x50) as usize;
                let base_120 = rd::<usize>(g_ed0 + 0x120);
                let sub_obj = rd::<usize>(base_120 + idx_50 * 0xa0 + (r8d as usize) * 8 + 0x40);
                let cnt = rd::<u16>(sub_obj + 0x2a) as usize;
                let mut found = 0xffff_ffffu32;
                if cnt != 0 {
                    let mut p = rd::<usize>(sub_obj + 0x08);
                    for _ in 0..cnt {
                        if rd::<i32>(p) == ebx {
                            found = rd::<u32>(p + 0x04);
                            break;
                        }
                        p = p.wrapping_add(0x68);
                    }
                }
                found
            };

            let rdi = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            let xmm1 = rd::<[u64; 2]>(param_2);
            let esi = ecx as usize;
            let ebp = esi << 5;
            let arr_3d8 = rd::<usize>(rdi + 0x3d8);
            let lo_16 = rd::<[u64; 2]>(arr_3d8 + ebp);

            let vtbl_340 = rd::<usize>(rdi + 0x340);
            let fn_38 = rd::<u64>(vtbl_340 + 0x38);
            let cap = exe::call_va!(fn_38, (rdi + 0x340) -> u64);
            if cap <= (esi as u64) {
                let vtbl_340_2 = rd::<usize>(rdi + 0x340);
                let fn_28 = rd::<u64>(vtbl_340_2 + 0x28);
                let new_cap = ((esi as u64).wrapping_add(0x400)) & !0x3ffu64;
                exe::call_va!(fn_28, (rdi + 0x340, new_cap));
            }

            let arr_3d8_new = rd::<usize>(rdi + 0x3d8);
            wr::<[u64; 2]>(arr_3d8_new + ebp, lo_16);
            wr::<[u64; 2]>(arr_3d8_new + ebp + 0x10, xmm1);

            let arr_3e8 = rd::<usize>(rdi + 0x3e8);
            let word_ptr = arr_3e8 + (esi >> 6) * 8;
            wr::<u64>(word_ptr, rd::<u64>(word_ptr) | (1u64 << (esi & 0x3f)));
            wr::<u32>(rdi + 0x410, rd::<u32>(rdi + 0x410).wrapping_add(1));
        }

        rax_8 = rd::<usize>(param_1 + 0x08);
        ebx += 1;
        if ebx >= rd::<i32>(rax_8 + 0x138) {
            break;
        }
    }
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_unknown_1420cd680(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, flag_4fd8: u8, p4: u8, op: u16| -> (u8, u32, u32) {
        let mut p1 = vec![0u8; 0x5000];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x4fd8, flag_4fd8);

        let mut p2 = vec![0u8; 0x200];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x40, u32::MAX);
        wr::<u32>(p2p + 0x44, 0);
        wr::<u64>(p2p + 0x138, 2);
        wr::<u64>(p2p + 0x140, 1);

        for slot in 0..6usize {
            let base = p2p + 0x48 + slot * 0x28;
            let s = slot as f32;
            wr::<f32>(base, s * 1.5);
            wr::<f32>(base + 0x04, s * -2.25);
            wr::<f32>(base + 0x18, s * 0.75);
            wr::<f32>(base + 0x1c, s * -0.5);
            wr::<u8>(base + 0x20, (slot as u8) + 10);
            wr::<u8>(base + 0x21, (slot as u8) + 20);
            wr::<u8>(base + 0x22, (slot as u8) + 30);
            wr::<u8>(base + 0x23, (slot as u8) + 40);
            wr::<u8>(base + 0x24, (slot as u8) + 50);
        }

        let r = f(p1p, p2p, op, p4);
        (r, rd::<u32>(p2p + 0x40), rd::<u32>(p2p + 0x44))
    };

    for flag in [0u8, 1u8] {
        for p4 in [0u8, 0x80u8] {
            for op in 0x82u16..=0x94u16 {
                let o = run(f_o, flag, p4, op);
                let n = run(f_n, flag, p4, op);
                crate::diff_eq!(o, n, "1420cd680 out");
            }
        }
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420ceef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, flag_4fd8: u8, idx: u16| -> (u8, u32, u32) {
        let mut p1 = vec![0u8; 0x5000];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x4fd8, flag_4fd8);

        let mut p2 = vec![0u8; 0x300];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x10, u32::MAX);
        wr::<u32>(p2p + 0x14, 0);
        wr::<u64>(p2p + 0x240, 4);

        wr::<u8>(p2p + 0x37 + 0x81 + (idx as usize), 1);
        wr::<u8>(p2p + 0x37 + (idx as usize), 0);
        wr::<u8>(p2p + 0x37 + 0x183 + (idx as usize), 1);

        let r = f(p1p, p2p, idx);
        (r, rd::<u32>(p2p + 0x10), rd::<u32>(p2p + 0x14))
    };

    for flag in [0u8, 1u8] {
        let o = run(f_o, flag, 5);
        let n = run(f_n, flag, 5);
        crate::diff_eq!(o, n, "1420ceef0 out");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420d2c00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut p1 = vec![0u8; 0x5900];
    let p1p = p1.as_mut_ptr() as usize;

    for v5860 in [0u8, 1u8, 2u8] {
        wr::<u8>(p1p + 0x5860, v5860);
        for p2 in [0u16, 1, 0x81, 0x82, 0x83, 0x93, 0x94, 0x95, 0xb9, 0xba] {
            for p3 in [0u8, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38] {
                let o = f_o(p1p, p2, p3);
                let n = f_n(p1p, p2, p3);
                crate::diff_eq!(o, n, "1420d2c00 out");
            }
        }
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420d95d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_df870(_p1: usize, _p2: u16) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_d1240(_p1: usize, _p2: usize, _p3: u16, p4: u8) -> u8 {
        (p4 == 0x80) as u8
    }
    unsafe extern "win64" fn mock_d55d0(_p1: usize, _p2: usize, _p3: u16, p4: u8) -> u8 {
        (p4 == 0) as u8
    }
    unsafe extern "win64" fn mock_ceff0(_p1: usize, _p2: usize, _p3: u16, p4: u8) -> u8 {
        (p4 == 0x80) as u8
    }
    unsafe extern "win64" fn mock_cd680(_p1: usize, _p2: usize, _p3: u16, p4: u8) -> u8 {
        (p4 == 0) as u8
    }
    unsafe extern "win64" fn mock_dbdb0(out: usize, _p1: usize, _p2: u16, p4: u8) -> usize {
        if p4 == 0 {
            wr::<f32>(out, 1.0);
            wr::<f32>(out + 4, 2.0);
            wr::<f32>(out + 8, 3.0);
        } else {
            wr::<f32>(out, 0.0);
            wr::<f32>(out + 4, 0.0);
            wr::<f32>(out + 8, 0.0);
        }
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_f870, mock_df870 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_1240, mock_d1240 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_55d0, mock_d55d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420c_eff0, mock_ceff0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420c_d680, mock_cd680 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420d_bdb0, mock_dbdb0 as *const ())?;

    let run = |f: F, p2: u16| -> u8 {
        let mut sub_60 = [0u8; 0x60];
        wr::<u64>(sub_60.as_mut_ptr() as usize + 0x38, 1);
        let mut sub_48 = [0u8; 0x20];
        let mut sub_50 = [0u8; 0x20];

        let mut inner0 = [0u8; 0x80];
        let i0p = inner0.as_mut_ptr() as usize;
        wr::<usize>(i0p, 0x1000);
        wr::<usize>(i0p + 0x48, sub_48.as_mut_ptr() as usize);
        wr::<usize>(i0p + 0x50, sub_50.as_mut_ptr() as usize);
        wr::<usize>(i0p + 0x60, sub_60.as_mut_ptr() as usize);

        let mut inner1 = vec![0u8; 0x5b00];
        let i1p = inner1.as_mut_ptr() as usize;
        wr::<u32>(i1p + 0x5888, 16);
        wr::<u32>(i1p + 0x5aa8, u32::MAX);
        wr::<u32>(i1p + 0x5aac, 0);

        let p1 = [i0p, i1p];
        f(p1.as_ptr() as usize, p2)
    };

    for p2 in [0x95u16, 0x01u16, 0x83u16, 0x40d0u16, 0x4100u16] {
        let o = run(f_o, p2);
        let n = run(f_n, p2);
        crate::diff_eq!(o, n, "1420d95d0 out");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420df870(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_check(ptr: usize, code: u16) -> u8 {
        ((rd::<u16>(ptr) == code) as u8) * 3
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_4330, mock_check as *const ())?;

    let s48 = 0x01u16;
    let s50 = 0x83u16;
    let s58 = 0xbbu16;
    let s60 = 0x95u16;

    let mut p1 = [0u8; 0x80];
    let p1p = p1.as_mut_ptr() as usize;
    wr::<usize>(p1p + 0x48, (&s48 as *const u16) as usize);
    wr::<usize>(p1p + 0x50, (&s50 as *const u16) as usize);
    wr::<usize>(p1p + 0x58, (&s58 as *const u16) as usize);
    wr::<usize>(p1p + 0x60, (&s60 as *const u16) as usize);

    for p2 in [0x01u16, 0x02, 0x83, 0x84, 0x95, 0x96, 0xbb, 0xbc, 0xf0] {
        let o = f_o(p1p, p2);
        let n = f_n(p1p, p2);
        crate::diff_eq!(o, n, "1420df870 out");
    }
    Ok(1)
}

pub unsafe fn diff_r_142258f90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let s_f48 = rd::<usize>(g_f48);

    let run = |f: F| -> [u32; 6] {
        let mut dst = [0.0f32; 6];
        let mut src = [0u8; 6 * 8];
        let sp = src.as_mut_ptr() as usize;
        let vals: [(f32, u32); 6] = [
            (4.0, 0x0000_0004),
            (10.0, 0x0000_0000),
            (-2.0, 0x0000_0008),
            (8.0, 0x0000_0010),
            (6.0, 0x0000_0004),
            (0.0, 0x0000_0001),
        ];
        for (i, (fv, mv)) in vals.iter().enumerate() {
            wr::<f32>(sp + i * 8, *fv);
            wr::<u32>(sp + i * 8 + 4, *mv);
        }

        let mut entry = [0u8; 0x18];
        let ep = entry.as_mut_ptr() as usize;
        wr::<u16>(ep, 6);
        wr::<u8>(ep + 0x02, 2);
        wr::<usize>(ep + 0x08, dst.as_mut_ptr() as usize);
        wr::<usize>(ep + 0x10, sp);

        let arr_270 = [2.0f32];
        let arr_2a0 = [2u8];

        let mut mgr = vec![0u8; 0x300];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x250, ep);
        wr::<u32>(mp + 0x258, 1);
        wr::<usize>(mp + 0x270, arr_270.as_ptr() as usize);
        wr::<usize>(mp + 0x2a0, arr_2a0.as_ptr() as usize);

        wr::<usize>(g_f48, mp);
        f();
        [
            dst[0].to_bits(),
            dst[1].to_bits(),
            dst[2].to_bits(),
            dst[3].to_bits(),
            dst[4].to_bits(),
            dst[5].to_bits(),
        ]
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_f48, s_f48);

    crate::diff_eq!(o, n, "142258f90 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1422611b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED_SPHERE: [u32; 4] = [0; 4];
    unsafe extern "win64" fn mock_307f90(_obj: usize, sphere_ptr: usize) {
        CAPTURED_SPHERE = rd::<[u32; 4]>(sphere_ptr);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4230_7f90, mock_307f90 as *const ())?;

    let g_ed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let s_ed0 = rd::<usize>(g_ed0);
    let s_f48 = rd::<usize>(g_f48);

    let run = |f: F, flag_4000: u32| -> [u32; 4] {
        CAPTURED_SPHERE = [0; 4];
        let arr_100 = [1i32];
        let mut ed0 = vec![0u8; 0x180];
        wr::<usize>(ed0.as_mut_ptr() as usize + 0x100, arr_100.as_ptr() as usize);
        wr::<usize>(g_ed0, ed0.as_mut_ptr() as usize);

        let arr_1a8 = [0u8];
        let mut f48 = vec![0u8; 0x200];
        wr::<usize>(f48.as_mut_ptr() as usize + 0x1a8, arr_1a8.as_ptr() as usize);
        wr::<usize>(g_f48, f48.as_mut_ptr() as usize);

        let mut r9_0 = vec![0u8; 0x100];
        wr::<u64>(r9_0.as_mut_ptr() as usize + 0xc8, 0);

        let mut r10 = vec![0u8; 0x280];
        let r10p = r10.as_mut_ptr() as usize;
        wr::<i32>(r10p + 0x1c8, 1);
        wr::<u8>(r10p + 0x272, 1);

        let mut r9 = [0u8; 0x90];
        let r9p = r9.as_mut_ptr() as usize;
        wr::<usize>(r9p, r9_0.as_mut_ptr() as usize);
        wr::<usize>(r9p + 0x08, r10p);
        wr::<u32>(r9p + 0x48, 0);
        wr::<u32>(r9p + 0x4c, flag_4000);
        wr::<u32>(r9p + 0x50, 0);
        wr::<[f32; 4]>(r9p + 0x6c, [5.0, 6.0, 7.0, 8.0]);

        let ptr_18 = [r9p];
        let ptr_38 = [1u64];
        let ptr_180 = [0u64];
        let ptr_198 = [0i32];
        let mat_5c8: [f32; 12] = [2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0, 3.0];
        let vec_e8: [f32; 4] = [3.0, 6.0, 9.0, 4.0];

        let mut p1 = vec![0u8; 0x600];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x18, ptr_18.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, ptr_38.as_ptr() as usize);
        wr::<u64>(p1p + 0x48, 1);
        wr::<usize>(p1p + 0xe8, vec_e8.as_ptr() as usize);
        wr::<usize>(p1p + 0x180, ptr_180.as_ptr() as usize);
        wr::<usize>(p1p + 0x198, ptr_198.as_ptr() as usize);
        wr::<usize>(p1p + 0x5c8, mat_5c8.as_ptr() as usize);

        f(p1p);
        CAPTURED_SPHERE
    };

    for flag in [0u32, 0x4000u32] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        wr::<usize>(g_ed0, s_ed0);
        wr::<usize>(g_f48, s_f48);
        crate::diff_eq!(o, n, "1422611b0 out");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_142307f90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_cap(_this: usize) -> u64 {
        16
    }
    let _m1 = crate::difftest::Mock::new(0x1_40f6_7380, mock_cap as *const ())?;

    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_ed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let g_1e8 = exe::ptr::<usize>(0x1_43a0_01e8) as usize;
    let s_f48 = rd::<usize>(g_f48);
    let s_ed0 = rd::<usize>(g_ed0);
    let s_1e8 = rd::<usize>(g_1e8);

    let run = |f: F, cc_val: i32| -> ([u64; 4], u64, u32) {
        let arr_1a8 = [3u8];
        let mut f48 = vec![0u8; 0x200];
        wr::<usize>(f48.as_mut_ptr() as usize + 0x1a8, arr_1a8.as_ptr() as usize);
        wr::<usize>(g_f48, f48.as_mut_ptr() as usize);

        let mut lookup_entry = [0u8; 0x68];
        wr::<i32>(lookup_entry.as_mut_ptr() as usize, 0);
        wr::<u32>(lookup_entry.as_mut_ptr() as usize + 0x04, 2);

        let mut sub_obj = [0u8; 0x40];
        wr::<usize>(sub_obj.as_mut_ptr() as usize + 0x08, lookup_entry.as_ptr() as usize);
        wr::<u16>(sub_obj.as_mut_ptr() as usize + 0x2a, 1);

        let mut base_120 = vec![0u8; 0x100];
        wr::<usize>(
            base_120.as_mut_ptr() as usize + 3 * 8 + 0x40,
            sub_obj.as_ptr() as usize,
        );
        let mut ed0 = vec![0u8; 0x180];
        wr::<usize>(ed0.as_mut_ptr() as usize + 0x120, base_120.as_ptr() as usize);
        wr::<usize>(g_ed0, ed0.as_mut_ptr() as usize);

        let mut vtbl = [0u64; 16];
        vtbl[0x38 / 8] = exe::ptr::<u8>(0x1_40f6_7380) as u64;

        let mut arr_3d8 = [[0u64; 4]; 4];
        arr_3d8[2] = [0x1111_2222, 0x3333_4444, 0, 0];
        let mut arr_3e8 = [0u64; 2];

        let mut mgr_1e8 = vec![0u8; 0x440];
        let mp = mgr_1e8.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x340, vtbl.as_ptr() as usize);
        wr::<usize>(mp + 0x3d8, arr_3d8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x3e8, arr_3e8.as_mut_ptr() as usize);
        wr::<u32>(mp + 0x410, 5);
        wr::<usize>(g_1e8, mp);

        let mut elem = vec![0u8; 0x100];
        let ep = elem.as_mut_ptr() as usize;
        wr::<u32>(ep + 0xa8, 3);
        wr::<i32>(ep + 0xcc, cc_val);
        wr::<u32>(ep + 0xd8, 2);

        let elems = [ep];
        let mut rax_8 = vec![0u8; 0x150];
        let r8p = rax_8.as_mut_ptr() as usize;
        wr::<usize>(r8p + 0x130, elems.as_ptr() as usize);
        wr::<i32>(r8p + 0x138, 1);

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, r8p);
        wr::<u32>(p1p + 0x48, 0);
        wr::<u32>(p1p + 0x50, 0);

        let p2: [u64; 2] = [0xaaaa_bbbb_cccc_dddd, 0xeeee_ffff_1234_5678];
        f(p1p, p2.as_ptr() as usize);

        (arr_3d8[2], arr_3e8[0], rd::<u32>(mp + 0x410))
    };

    for cc_val in [0i32, 1i32] {
        let o = run(f_o, cc_val);
        let n = run(f_n, cc_val);
        wr::<usize>(g_f48, s_f48);
        wr::<usize>(g_ed0, s_ed0);
        wr::<usize>(g_1e8, s_1e8);
        crate::diff_eq!(o, n, "142307f90 out");
    }
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_420c_d680, "unknown_1420cd680", rend::cauldron_hot_c::unknown_1420cd680 as *const (), rend::cauldron_hot_c::diff_unknown_1420cd680),
// Replacement::verified(0x1_420c_eef0, "unknown_1420ceef0", rend::cauldron_hot_c::unknown_1420ceef0 as *const (), rend::cauldron_hot_c::diff_unknown_1420ceef0),
// Replacement::verified(0x1_420d_2c00, "unknown_1420d2c00", rend::cauldron_hot_c::unknown_1420d2c00 as *const (), rend::cauldron_hot_c::diff_unknown_1420d2c00),
// Replacement::verified_throws(0x1_420d_95d0, "unknown_1420d95d0", rend::cauldron_hot_c::unknown_1420d95d0 as *const (), Throws::Discard, rend::cauldron_hot_c::diff_unknown_1420d95d0),
// Replacement::verified(0x1_420d_f870, "unknown_1420df870", rend::cauldron_hot_c::unknown_1420df870 as *const (), rend::cauldron_hot_c::diff_unknown_1420df870),
// Replacement::verified(0x1_4225_8f90, "r_142258f90", rend::cauldron_hot_c::r_142258f90 as *const (), rend::cauldron_hot_c::diff_r_142258f90),
// Replacement::verified(0x1_4226_11b0, "unknown_1422611b0", rend::cauldron_hot_c::unknown_1422611b0 as *const (), rend::cauldron_hot_c::diff_unknown_1422611b0),
// Replacement::verified(0x1_4230_7f90, "unknown_142307f90", rend::cauldron_hot_c::unknown_142307f90 as *const (), rend::cauldron_hot_c::diff_unknown_142307f90),
