//! Alan Wake 2 re-export of `x86_runtime::simd_ops` plus 11 hand-decompiled `r::*` helpers.

pub use x86_runtime::simd_ops::*;

#[inline(always)]
fn exe_va(va: u64) -> usize {
    crate::exe::ptr::<u8>(va) as usize
}

/// `0x1412eda20` — `r::DynamicArray<T (0xd0)>::push_back_zeroed`
pub unsafe extern "win64" fn fn_1412eda20(rdi: usize) -> usize {
    use crate::exe::{call_va, rd, wr};
    let cap = rd::<u32>(rdi + 0xc) as u64;
    let len = rd::<u32>(rdi + 0x8) as u64;
    if len == cap {
        let new_cap = (cap + (cap >> 1)).max(len + 1);
        call_va!(0x1412ee880, (rdi, new_cap) -> ());
    }
    let idx = rd::<u32>(rdi + 0x8) as usize;
    let rbx = rd::<usize>(rdi).wrapping_add(idx * 0xd0);
    core::ptr::write_bytes(rbx as *mut u8, 0, 0xd0);
    wr::<u64>(rbx + 0xc8, 0);
    let new_len = (idx as u32).wrapping_add(1);
    wr::<u32>(rdi + 0x8, new_len);
    rd::<usize>(rdi).wrapping_add((new_len as usize) * 0xd0).wrapping_sub(0xd0)
}

/// `0x14223dbf0` — per-worker command/instance queue push
pub unsafe extern "win64" fn fn_14223dbf0(rbx_in: usize, rbp: usize) {
    use crate::exe::{call_iat, call_va, rd, wr};
    let idx = call_va!(0x142407c80, () -> u32) as usize;
    let slot_ptr = rd::<usize>(exe_va(0x143c4aa00) + idx * 8);
    let worker_id = if slot_ptr != 0 { rd::<u32>(slot_ptr) as usize } else { 0 };
    let rdi = rbx_in.wrapping_add(worker_id * 0xd8);
    let lock = rdi.wrapping_add(0xa8);
    let tid = call_iat!(0x142645790, () -> u32);
    if rd::<u32>(lock + 0x8) != tid {
        call_iat!(0x1426455a8, (lock) -> ());
        wr::<u32>(lock + 0x8, tid);
    }
    wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));
    let dst = fn_1412eda20(rdi + 0x78);
    wr::<u32>(dst, 1);
    let data = rd::<[u8; 64]>(rbp);
    wr::<[u8; 64]>(dst + 8, data);
    let new_cnt = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, new_cnt);
    if new_cnt == 0 {
        atomic_xchg_u32((lock + 0x8) as u64, u32::MAX);
        call_iat!(0x142645598, (lock) -> ());
    }
}

/// `0x1420ceff0` — thread-safe bit/flag table query
pub unsafe extern "win64" fn fn_1420ceff0(r15: usize, r14: usize, r8: u64, r9: u64) -> u32 {
    use crate::exe::{call_iat, rd, wr};
    let esi = (r9 as u8) as i8;
    let edi = (r8 as u16) as usize;
    let tid = call_iat!(0x142645790, () -> u32);
    if rd::<u32>(r14 + 0x10) != tid {
        call_iat!(0x1426455a8, (r14 + 0x8) -> ());
        wr::<u32>(r14 + 0x10, tid);
    }
    wr::<u32>(r14 + 0x14, rd::<u32>(r14 + 0x14).wrapping_add(1));
    let base_r8 = r14.wrapping_add(0x38);
    let addr = if esi < 0 {
        if rd::<u8>(r15 + 0x4fd8) != 0 {
            base_r8.wrapping_add(edi).wrapping_add(0x183)
        } else {
            let v = rd::<u64>(base_r8 + 0x208).wrapping_add(2);
            let rem = (v % 3) as usize;
            base_r8.wrapping_add(edi).wrapping_add(rem * 0x81)
        }
    } else {
        let v = rd::<u64>(base_r8 + 0x208);
        let rem = (v % 3) as usize;
        base_r8.wrapping_add(edi).wrapping_add(rem * 0x81)
    };
    let bit = (rd::<u8>(addr - 1) & 1) as u32;
    let new_cnt = rd::<u32>(r14 + 0x14).wrapping_sub(1);
    wr::<u32>(r14 + 0x14, new_cnt);
    if new_cnt == 0 {
        atomic_xchg_u32((r14 + 0x10) as u64, u32::MAX);
        call_iat!(0x142645598, (r14 + 0x8) -> ());
    }
    bit
}

/// `0x142206de0` — per-frame mesh/texture LOD streaming bucket update
pub unsafe extern "win64" fn fn_142206de0(rsi: usize, rbx: usize) {
    use crate::exe::{call_va, rd, wr};
    let xmm = take_xmm_args();
    let xmm8 = f32::from_bits(xmm[2] as u32);
    let xmm7 = f32::from_bits(xmm[3] as u32);
    let r12d = rd::<u8>(rbx + 0xbd) as i32;
    let arr_ptr = rd::<usize>(rsi + 0x2e0);
    let arr_len = rd::<u32>(rsi + 0x2e8) as u64;
    let idx = rd::<u32>(rbx + 0xc4) as u64;
    let xmm4 = if idx < arr_len {
        rd::<f32>(arr_ptr + (idx as usize) * 4)
    } else {
        rd::<f32>(exe_va(0x143426d10))
    };
    let ba = rd::<u8>(rbx + 0xba) as i32;
    let xmm1 = xmm4 - (ba as f32);
    let xmm3 = if 0.0f32 <= xmm1 { xmm1 } else { 0.0f32 };
    let c_58 = rd::<f32>(exe_va(0x143867c58));
    let xmm9 = (xmm3 * c_58) / (xmm3 + c_58) + (xmm4 - xmm3);
    let c0 = rd::<f32>(rbx + 0xc0);
    let d2 = xmm8 - c0;
    let xmm3_max = if 0.0f32 > d2 { 0.0f32 } else { d2 };
    let d3 = xmm7 - c0;
    let xmm6_max = if 0.0f32 > d3 { 0.0f32 } else { d3 };

    let bb = rd::<u8>(rbx + 0xbb) as i32;
    let (edi, r14d): (i32, i32) = if (xmm9 > 0.0f32 && rd::<u8>(exe_va(0x143867d90)) != 0)
        || rd::<i32>(rbx + 0xd0) > 0
    {
        (0, 0)
    } else if rd::<u8>(rsi + 0x39) != 0 || rd::<u8>(exe_va(0x143867d18)) != 0 {
        let v = bb - 1;
        (v, v)
    } else {
        let c_hi = rd::<f32>(exe_va(0x143867cb8));
        let c_lo = rd::<f32>(exe_va(0x143867d70));
        let v1 = xmm9 - xmm3_max;
        let max1 = if v1 > c_lo { v1 } else { c_lo };
        let min1 = if max1 < c_hi { max1 } else { c_hi };
        let ecx = min1.ceil() as i32;
        let v2 = xmm9 - xmm6_max;
        let max2 = if v2 > c_lo { v2 } else { c_lo };
        let min2 = if max2 < c_hi { max2 } else { c_hi };
        let r11d = min2.ceil() as i32;
        let r9d = bb - 1;
        let mut edx = ba - ecx;
        if edx < 0 {
            edx = 0;
        } else if edx > r9d {
            edx = r9d;
        }
        let edi = (edx as u8) as i32;
        let mut eax = ba - r11d;
        if eax < 0 {
            eax = 0;
        } else if eax > r9d {
            eax = r9d;
        }
        let r14d = (eax as u8) as i32;
        (edi, r14d)
    };

    let bb_m1 = (rd::<u8>(rbx + 0xbb) as i32) - 1;
    let edx_add = if edi < bb_m1 { rd::<u32>(rbx + 0x100) } else { 0 };
    let c1 = rd::<u32>((rbx as isize + (edi as isize) * 4 + 0xd8) as usize).wrapping_add(edx_add) as u64;
    wr::<u64>(rsi + 0x40, rd::<u64>(rsi + 0x40).wrapping_add(c1));

    let bb_m1_2 = (rd::<u8>(rbx + 0xbb) as i32) - 1;
    let r8d_add = if r14d < bb_m1_2 { rd::<u32>(rbx + 0x100) } else { 0 };
    let c2 = rd::<u32>((rbx as isize + (r14d as isize) * 4 + 0xd8) as usize).wrapping_add(r8d_add) as u64;
    wr::<u64>(rsi + 0x48, rd::<u64>(rsi + 0x48).wrapping_add(c2));

    if edi != r12d && rd::<u8>(rbx + 0xb9) == 0 {
        let rbp = (rsi as isize + (((edi - r12d) as isize + 0xe) << 4)) as usize;
        let old = atomic_xadd_u32((rbx + 0x24) as u64, 1);
        if old.wrapping_add(1) == 1 {
            let rcx = rd::<usize>(rbx + 0x28);
            if rcx != 0 {
                call_va!(0x14241d6f0, (rcx, rbx) -> ());
            }
        }
        let edx = rd::<u32>(rbp + 0x8) as u64;
        let r8d = rd::<u32>(rbp + 0xc) as u64;
        if (edx << 4) == (r8d << 4) {
            let new_cap = (r8d + (r8d >> 1)).max(edx + 1);
            call_va!(0x1412df620, (rbp, new_cap) -> ());
        }
        let idx = rd::<u32>(rbp + 0x8) as usize;
        let elem = rd::<usize>(rbp).wrapping_add(idx << 4);
        wr::<usize>(elem, rbx);
        wr::<i32>(elem + 8, edi);
        wr::<u32>(rbp + 8, (idx as u32).wrapping_add(1));
    }

    if r14d != r12d && rd::<u8>(rbx + 0xb9) == 0 {
        let rdi = (rsi as isize + (((r14d - r12d) as isize + 0x1f) << 4)) as usize;
        let old = atomic_xadd_u32((rbx + 0x24) as u64, 1);
        if old == 0 {
            let rcx = rd::<usize>(rbx + 0x28);
            if rcx != 0 {
                call_va!(0x14241d6f0, (rcx, rbx) -> ());
            }
        }
        let edx = rd::<u32>(rdi + 0x8) as u64;
        let r8d = rd::<u32>(rdi + 0xc) as u64;
        if (edx << 4) == (r8d << 4) {
            let new_cap = (r8d + (r8d >> 1)).max(edx + 1);
            call_va!(0x1412df620, (rdi, new_cap) -> ());
        }
        let idx = rd::<u32>(rdi + 0x8) as usize;
        let elem = rd::<usize>(rdi).wrapping_add(idx << 4);
        wr::<usize>(elem, rbx);
        wr::<i32>(elem + 8, r14d);
        wr::<u32>(rdi + 8, (idx as u32).wrapping_add(1));
    }
}

/// `0x1423ef2b0` — `r::WString::reserve`
pub unsafe extern "win64" fn fn_1423ef2b0(rdi: usize, rdx_in: u64) {
    use crate::exe::{call_iat, call_va, rd, wr};
    let rdx = rdx_in.wrapping_add(1);
    let rcx_18 = rd::<usize>(rdi + 0x18);
    let (r14, esi): (usize, u32);
    if rcx_18 != 0 && rdx <= (rd::<u32>(rdi + 0x8) as u64) {
        let cur_ptr = rd::<usize>(rdi);
        if cur_ptr == rcx_18 {
            let cap = rd::<u32>(rdi + 0xc);
            if cap <= rd::<u32>(rdi + 0x10) {
                let new_len = cap.wrapping_sub(1);
                wr::<u32>(rdi + 0x10, new_len);
                wr::<u16>(cur_ptr + (new_len as usize) * 2, 0);
            }
            return;
        }
        let inline_cap = rd::<u32>(rdi + 0x8);
        call_iat!(0x1426469c8, (rcx_18, inline_cap, cur_ptr, (inline_cap as u64).wrapping_sub(1)) -> ());
        let old_ptr = rd::<usize>(rdi);
        if old_ptr != 0 {
            call_va!(0x142442de0, (old_ptr) -> ());
        }
        r14 = rd::<usize>(rdi + 0x18);
        esi = rd::<u32>(rdi + 0x8);
    } else {
        let cur_cap = rd::<u32>(rdi + 0xc) as u64;
        if rdx <= cur_cap {
            return;
        }
        let aligned = (rdx as u32) & !0xfu32;
        let mut new_cap = if (rdx & 0xf) != 0 { aligned.wrapping_add(0x10) } else { aligned };
        if rdx > 0x100 {
            let grown = (cur_cap as u32).wrapping_add((cur_cap as u32) >> 1);
            if grown > new_cap {
                new_cap = grown;
            }
        }
        let alloc_ptr = call_va!(0x1423ef3d0, (rcx_18, rdx, cur_cap, new_cap) -> usize);
        let old_ptr = rd::<usize>(rdi);
        call_iat!(0x1426469c8, (alloc_ptr, new_cap, old_ptr, new_cap.wrapping_sub(1)) -> ());
        let old_ptr2 = rd::<usize>(rdi);
        if old_ptr2 != rd::<usize>(rdi + 0x18) && old_ptr2 != 0 {
            call_va!(0x142442de0, (old_ptr2) -> ());
        }
        r14 = alloc_ptr;
        esi = new_cap;
    }
    wr::<u32>(rdi + 0xc, esi);
    wr::<usize>(rdi, r14);
    if esi <= rd::<u32>(rdi + 0x10) {
        let new_len = esi.wrapping_sub(1);
        wr::<u32>(rdi + 0x10, new_len);
        wr::<u16>(r14 + (new_len as usize) * 2, 0);
    }
}

/// `0x1423f0370` — `r::WString::operator=(WString&&)`
pub unsafe extern "win64" fn fn_1423f0370(rbx: usize, rdi: usize) -> usize {
    use crate::exe::{call_iat, call_va, rd, wr};
    if rbx == rdi {
        return rbx;
    }
    let src_len = rd::<u32>(rdi + 0x10);
    let dst_inline_cap = rd::<u32>(rbx + 0x8);
    let src_ptr = rd::<usize>(rdi);
    if src_len >= dst_inline_cap && rd::<usize>(rdi + 0x18) != src_ptr {
        let dst_ptr = rd::<usize>(rbx);
        if dst_ptr != rd::<usize>(rbx + 0x18) && dst_ptr != 0 {
            call_va!(0x142442de0, (dst_ptr) -> ());
        }
        wr::<usize>(rbx, rd::<usize>(rdi));
        wr::<u32>(rbx + 0xc, rd::<u32>(rdi + 0xc));
        wr::<u32>(rbx + 0x10, rd::<u32>(rdi + 0x10));
        let src_inline_cap = rd::<u32>(rdi + 0x8);
        let src_inline_buf = rd::<usize>(rdi + 0x18);
        wr::<u32>(rdi + 0xc, src_inline_cap);
        wr::<u32>(rdi + 0x10, 0);
        wr::<usize>(rdi, src_inline_buf);
        wr::<u16>(src_inline_buf, 0);
        return rbx;
    }
    let dst_len = rd::<u32>(rbx + 0x10);
    let req = if src_len >= dst_len { src_len } else { dst_len };
    fn_1423ef2b0(rbx, req as u64);
    call_iat!(0x1426469d0, (rd::<usize>(rbx), rd::<u32>(rbx + 0xc), rd::<usize>(rdi)) -> ());
    wr::<u32>(rbx + 0x10, rd::<u32>(rdi + 0x10));
    rbx
}

/// `0x1401ed730` — `r::ObjectStream` reader context constructor
pub unsafe extern "win64" fn fn_1401ed730(
    r14: usize,
    rdi: usize,
    rbx: usize,
    r9: u64,
    s5: u64,
) -> usize {
    use crate::exe::{call_va, rd, wr};
    core::ptr::write_bytes(r14 as *mut u8, 0, 0x310);
    wr::<usize>(r14, r14 + 0x10);
    wr::<u32>(r14 + 0x8, 0);
    wr::<u32>(r14 + 0xc, 0x10);
    wr::<u64>(r14 + 0x310, 0);
    wr::<u32>(r14 + 0x318, 0);
    wr::<u32>(r14 + 0x328, 0xffff_ffff);
    wr::<usize>(r14 + 0x330, rbx);
    wr::<usize>(r14 + 0x338, rdi);
    wr::<usize>(r14 + 0x340, rdi);
    let rax_348 = r14 + 0x348;
    wr::<usize>(rax_348, exe_va(0x14307b558));
    wr::<usize>(rax_348 + 0x8, rdi);
    let rcx_358 = r14 + 0x358;
    wr::<usize>(rcx_358, exe_va(0x14340e9e0));
    wr::<usize>(rcx_358 + 0x8, rax_348);
    wr::<u8>(r14 + 0x368, r9 as u8);
    wr::<u8>(r14 + 0x369, s5 as u8);
    wr::<usize>(r14 + 0x320, rcx_358);

    let edx = rd::<u32>(r14 + 0x8) as u64;
    let r8d = rd::<u32>(r14 + 0xc) as u64;
    if edx * 48 == r8d * 48 {
        let new_cap = (r8d + (r8d >> 1)).max(edx + 1);
        call_va!(0x1413aa3e0, (r14, new_cap) -> ());
    }
    let idx = rd::<u32>(r14 + 0x8) as usize;
    let slot0 = rd::<usize>(r14).wrapping_add(idx * 48);
    core::ptr::write_bytes(slot0 as *mut u8, 0, 48);
    let new_len = (idx as u32).wrapping_add(1);
    wr::<u32>(r14 + 0x8, new_len);
    let end_slot = rd::<usize>(r14).wrapping_add((new_len as usize) * 48);
    wr::<u64>(end_slot - 0x30, 0);
    wr::<usize>(end_slot - 0x28, exe_va(0x14309f824));
    wr::<u64>(end_slot - 0x20, 0);
    wr::<u64>(end_slot - 0x18, 0);
    wr::<u64>(end_slot - 0x10, 0);
    wr::<u32>(end_slot - 0x8, 0xffff_ffff);
    let p320 = rd::<usize>(r14 + 0x320);
    if p320 != 0 {
        let inner = rd::<usize>(p320 + 0x8);
        if inner != 0 {
            let vtbl = rd::<usize>(inner);
            let fn_10 = rd::<u64>(vtbl + 0x10);
            let pos = call_va!(fn_10, (inner) -> u64);
            wr::<u64>(end_slot - 0x18, pos);
        }
    }
    r14
}

/// `0x1420dcac0` — recursive node index resolution with SRWLock
pub unsafe extern "win64" fn fn_1420dcac0(r15: usize, r14: usize, r8: u64) -> usize {
    use crate::exe::{call_iat, rd, wr};
    let mut bx = r8 as u16;
    if r14 == 0 {
        wr::<u16>(r15, 0);
        return r15;
    }
    if (bx as i16) >= 0 {
        wr::<u16>(r15, bx);
        return r15;
    }
    bx &= 0x7fff;
    let rdi = r14.wrapping_add(0x5a90);
    let tid = call_iat!(0x142645790, () -> u32);
    if rd::<u32>(rdi + 0x8) != tid {
        call_iat!(0x1426455a8, (rdi) -> ());
        wr::<u32>(rdi + 0x8, tid);
    }
    wr::<u32>(rdi + 0xc, rd::<u32>(rdi + 0xc).wrapping_add(1));
    loop {
        let rdx = rd::<usize>(r14 + 0x5870);
        let idx = bx.wrapping_sub(0xce) as usize;
        let len = rd::<u32>(r14 + 0x5878) as usize;
        if idx >= len {
            wr::<u16>(r15, 0);
            let new_cnt = rd::<u32>(rdi + 0xc).wrapping_sub(1);
            wr::<u32>(rdi + 0xc, new_cnt);
            if new_cnt == 0 {
                atomic_xchg_u32((rdi + 0x8) as u64, u32::MAX);
                call_iat!(0x142645598, (rdi) -> ());
            }
            return r15;
        }
        let entry = rdx.wrapping_add(idx * 0x38);
        bx = rd::<u16>(entry + 0x28);
        let kind = rd::<u8>(entry + 0x30);
        if kind == 1 {
            let mut tmp: u16 = 0;
            fn_1420dcac0((&mut tmp as *mut u16) as usize, r14, bx as u64);
            bx = tmp;
            if bx == 0x95 || bx == 0x98 {
                bx = bx.wrapping_add(1);
            }
        } else if kind == 2 {
            let mut tmp: u16 = 0;
            fn_1420dcac0((&mut tmp as *mut u16) as usize, r14, bx as u64);
            bx = tmp;
            if bx == 0x95 || bx == 0x98 {
                bx = bx.wrapping_add(2);
            }
        }
        if (bx as i16) >= 0 {
            break;
        }
        bx &= 0x7fff;
    }
    let new_cnt = rd::<u32>(rdi + 0xc).wrapping_sub(1);
    wr::<u32>(rdi + 0xc, new_cnt);
    if new_cnt == 0 {
        atomic_xchg_u32((rdi + 0x8) as u64, u32::MAX);
        call_iat!(0x142645598, (rdi) -> ());
    }
    wr::<u16>(r15, bx);
    r15
}

/// `0x14243b000` — `r::ObjectStream` type hash match check
pub unsafe extern "win64" fn fn_14243b000(_rcx: usize, edx: u32, r8d: u32) -> u8 {
    use crate::exe::{call_va, rd};
    if edx == r8d {
        return 1;
    }
    let rbx = call_va!(0x1424b14c0, (edx) -> usize);
    if rbx == 0 {
        return 0;
    }
    let vtbl1 = rd::<usize>(rbx);
    let base1 = call_va!(rd::<u64>(vtbl1 + 0x88), (rbx) -> usize);
    if base1 == 0 {
        return 0;
    }
    let vtbl2 = rd::<usize>(rbx);
    let base2 = call_va!(rd::<u64>(vtbl2 + 0x88), (rbx) -> usize);
    let vtbl3 = rd::<usize>(base2);
    let base_hash = call_va!(rd::<u64>(vtbl3 + 0x8), (base2) -> u32);
    (base_hash == r8d) as u8
}

/// `0x1401e9a20` — `r::ObjectStream` scope end (`END_FN`)
pub unsafe extern "win64" fn fn_1401e9a20(rbx: usize) -> u64 {
    use crate::exe::{call_va, rd, wr};
    if rd::<u8>(rbx + 0x368) == 0 {
        let edx = rd::<u32>(rbx + 0x8) as usize;
        let rsi = rd::<usize>(rbx).wrapping_add(edx * 48);
        let rcx_338 = rd::<usize>(rbx + 0x338);
        let rax_120 = rd::<usize>(rcx_338 + 0x120);
        let edi = rd::<u32>(rax_120);
        wr::<usize>(rcx_338 + 0x120, rax_120 + 4);
        let rax_320 = rd::<usize>(rbx + 0x320);
        let inner = rd::<usize>(rax_320 + 0x8);
        let vtbl = rd::<usize>(inner);
        let fn_10 = rd::<u64>(vtbl + 0x10);
        let pos = call_va!(fn_10, (inner) -> u64);
        let diff = pos.wrapping_sub(rd::<u64>(rsi - 0x18));
        let expected = rd::<u64>(rsi - 0x10);
        if diff != expected || (edi != 0xdead_beef && edi != 0xd34d_b33f) {
            wr::<usize>(rcx_338 + 0x120, rax_120);
            return run_guest_va::<0x1401e9a20, 2>(
                rbx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            );
        }
    }
    let rax = rd::<usize>(rbx);
    let edx = rd::<u32>(rbx + 0x8) as usize;
    let prev = rd::<u32>(rax + edx * 48 - 8);
    wr::<u32>(rbx + 0x328, prev);
    let new_depth = (edx as u32).wrapping_sub(1);
    wr::<u32>(rbx + 0x8, new_depth);
    new_depth as u64
}

/// `0x1401e9b60` — `r::ObjectStream` scope begin (`BEGIN_FN`)
pub unsafe extern "win64" fn fn_1401e9b60(
    rbx: usize,
    rdx_in: usize,
    rsi: usize,
    r13d: u32,
    edi: i32,
    ebp: i32,
) {
    use crate::exe::{call_va, rd, wr};
    let old_depth = rd::<u32>(rbx + 0x8);
    let rcx_338 = rd::<usize>(rbx + 0x338);
    let old_cursor = rd::<usize>(rcx_338 + 0x120);
    call_va!(0x14243b0b0, (rbx, rdx_in, rsi, r13d, edi as u32, ebp as u32) -> ());
    let rax_120 = rd::<usize>(rcx_338 + 0x120);
    if rd::<u8>(rbx + 0x368) != 0 {
        let r8d = rd::<i32>(rax_120);
        wr::<usize>(rcx_338 + 0x120, rax_120 + 4);
        if r8d >= ebp && r8d <= edi {
            let rax = rd::<usize>(rbx);
            let ecx = rd::<u32>(rbx + 0x8) as usize;
            let prev = rd::<u32>(rbx + 0x328);
            wr::<u32>(rax + ecx * 48 - 8, prev);
            wr::<i32>(rbx + 0x328, r8d);
            return;
        }
    } else {
        let magic = rd::<u32>(rax_120);
        wr::<usize>(rcx_338 + 0x120, rax_120 + 4);
        if magic == 0xdead_beef || magic == 0xd34d_b33f {
            let mut r12d = 0u32;
            if magic == 0xd34d_b33f {
                let c = rd::<usize>(rbx + 0x338);
                let p = rd::<usize>(c + 0x120);
                r12d = rd::<u32>(p);
                wr::<usize>(c + 0x120, p + 4);
            }
            let c1 = rd::<usize>(rbx + 0x338);
            let p1 = rd::<usize>(c1 + 0x120);
            let size_val = rd::<u32>(p1) as u64;
            wr::<usize>(c1 + 0x120, p1 + 4);
            let depth = rd::<u32>(rbx + 0x8) as usize;
            let arr = rd::<usize>(rbx);
            wr::<u64>(arr + depth * 48 - 0x10, size_val);

            let c2 = rd::<usize>(rbx + 0x338);
            let p2 = rd::<usize>(c2 + 0x120);
            let r15d = rd::<u32>(p2);
            wr::<usize>(c2 + 0x120, p2 + 4);

            let c3 = rd::<usize>(rbx + 0x338);
            let p3 = rd::<usize>(c3 + 0x120);
            let r14d = rd::<i32>(p3);
            wr::<usize>(c3 + 0x120, p3 + 4);

            if r14d >= ebp && r14d <= edi {
                let ok = fn_14243b000(c3, r15d, r13d);
                if ok != 0 {
                    let rax = rd::<usize>(rbx);
                    let ecx = rd::<u32>(rbx + 0x8) as usize;
                    let prev = rd::<u32>(rbx + 0x328);
                    wr::<u32>(rax + ecx * 48 - 8, prev);
                    wr::<i32>(rbx + 0x328, r14d);
                    if r12d == 1 {
                        let c4 = rd::<usize>(rbx + 0x338);
                        wr::<usize>(c4 + 0x120, rd::<usize>(c4 + 0x120) + 4);
                    }
                    return;
                }
            }
        }
    }
    wr::<u32>(rbx + 0x8, old_depth);
    wr::<usize>(rcx_338 + 0x120, old_cursor);
    let _ = run_guest_va::<0x1401e9b60, 2>(
        rbx as u64,
        rdx_in as u64,
        rsi as u64,
        r13d as u64,
        edi as u32 as u64,
        ebp as u32 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    );
}

#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub unsafe extern "win64" fn aot_va<const VA: u64, const EH: u8>() -> u64 {
    core::arch::asm!("movz x16, #0xa07a", options(nomem, nostack, preserves_flags));
    0
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(never)]
pub unsafe extern "win64" fn aot_va<const VA: u64, const EH: u8>(
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
    arg10: u64,
    arg11: u64,
    arg12: u64,
    arg13: u64,
    arg14: u64,
    arg15: u64,
    arg16: u64,
) -> u64 {
    run_guest_va::<VA, EH>(
        rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15,
        arg16,
    )
}

#[inline]
pub unsafe fn run_guest_va<const VA: u64, const EH: u8>(
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
    arg10: u64,
    arg11: u64,
    arg12: u64,
    arg13: u64,
    arg14: u64,
    arg15: u64,
    arg16: u64,
) -> u64 {
    let target = crate::exe::lookup_trampoline_or_mapped(VA);
    #[cfg(target_arch = "x86_64")]
    if EH == 2 {
        return match crate::exe::try_call(
            target,
            &[
                rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
                arg15, arg16,
            ],
            &[],
        ) {
            Ok(v) => v.rax,
            Err(e) => e.rethrow(),
        };
    }
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> u64 = core::mem::transmute(target);
        f(
            rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
            arg15, arg16,
        )
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let xmm = take_xmm_args();
        let (r_gpr, r_xmm) = crate::emu::emu_run_full_rax(
            (target & !(1usize << 46)) as u64,
            [rcx, rdx, r8, r9, arg5, arg6, arg7, arg8],
            xmm,
            [arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16],
            0,
        );
        set_xmm_ret(r_xmm);
        r_gpr
    }
}

#[cfg(test)]
mod tests {
    use super::vpermd;

    fn pack_u32x4(a: u32, b: u32, c: u32, d: u32) -> u128 {
        (a as u128) | ((b as u128) << 32) | ((c as u128) << 64) | ((d as u128) << 96)
    }

    #[test]
    fn vpermd_cross_lane_and_high_index_masking() {
        let src_lo = pack_u32x4(0x1010_0000, 0x2020_0001, 0x3030_0002, 0x4040_0003);
        let src_hi = pack_u32x4(0x5050_0004, 0x6060_0005, 0x7070_0006, 0x8080_0007);

        // Reverse all 8 lanes while setting high garbage bits in control dwords (only bits [2:0] are used).
        let idx_lo = pack_u32x4(7 | 0xFFFF_FF00, 6 | 0x8000_0008, 5 | 0x1234_5678, 4);
        let idx_hi = pack_u32x4(3 | 0xDEAD_BEE8, 2, 1 | 0x0000_00F8, 0 | 0x7FFF_FFF8);
        let (got_lo, got_hi) = vpermd(idx_lo, idx_hi, src_lo, src_hi);
        assert_eq!(got_lo, pack_u32x4(0x8080_0007, 0x7070_0006, 0x6060_0005, 0x5050_0004));
        assert_eq!(got_hi, pack_u32x4(0x4040_0003, 0x3030_0002, 0x2020_0001, 0x1010_0000));

        // Broadcast lane 5 across all 8 lanes.
        let bc = pack_u32x4(5, 5, 5, 5);
        let (b_lo, b_hi) = vpermd(bc, bc, src_lo, src_hi);
        assert_eq!(b_lo, pack_u32x4(0x6060_0005, 0x6060_0005, 0x6060_0005, 0x6060_0005));
        assert_eq!(b_hi, b_lo);
    }
}
