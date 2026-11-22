//! Decompiled hot runtime functions (`cauldron_hot_a`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn inplace_str_ptr_len(s: usize) -> (usize, usize) {
    let b3 = rd::<u8>(s + 0x03) as u32;
    let inline_cap = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap = (rd::<u32>(s) & 0x00ff_ffff) << 3;
    let ptr = if cap > inline_cap {
        rd::<usize>(s + 0x08)
    } else {
        s + 0x08
    };
    let len = rd::<u32>(s + 0x04) as usize;
    (ptr, len)
}

#[inline(always)]
unsafe fn inplace_str_eq_nocase(s1: usize, s2: usize) -> bool {
    let (p2, len2) = inplace_str_ptr_len(s2);
    inplace_str_eq_nocase_bytes(s1, p2, len2)
}

/// [`inplace_str_eq_nocase`] against a raw byte range `p2[..len2]` instead of a second
/// inplace string (same fold table `0x1426d01e0`).
pub(crate) unsafe fn inplace_str_eq_nocase_bytes(s1: usize, p2: usize, len2: usize) -> bool {
    let (p1, len1) = inplace_str_ptr_len(s1);
    if len1 != len2 {
        return false;
    }
    if len1 != 0 {
        let fold = exe::ptr::<u8>(0x1_426d_01e0) as usize;
        for i in 0..len1 {
            let c1 = rd::<u8>(p1.wrapping_add(i)) as usize;
            let c2 = rd::<u8>(p2.wrapping_add(i)) as usize;
            if rd::<i8>(fold + c1) != rd::<i8>(fold + c2) {
                return false;
            }
        }
    }
    true
}

/// `unknown_1401e6ff0`
/// Original: 0x1401e6ff0 (183 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401e6ff0(param_1: usize, param_2: usize) -> u8 {
    if inplace_str_eq_nocase(param_1, param_2) {
        1
    } else {
        0
    }
}

/// `unknown_1411f4260`
/// Original: 0x1411f4260 (208 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411f4260(param_1: usize, param_2: u16, param_3: u8) -> u8 {
    let c1 = exe::call_va!(0x1_420e_3160, (param_1, param_2 as u32) -> u8);
    if c1 != 0 {
        return 1;
    }
    let p0 = rd::<usize>(param_1);
    if p0 == 0 {
        return 0;
    }
    let fn_960 = exe::ptr::<u8>(0x1_420e_0960) as usize;
    if rd::<u64>(param_1 + 0x08) != rd::<u64>(p0 + 0x4fc0)
        && rd::<usize>(param_1 + 0x70) != fn_960
    {
        return 0;
    }
    let fn_70 = rd::<usize>(param_1 + 0x70);
    if fn_70 != 0 {
        let arg_68 = rd::<u64>(param_1 + 0x68);
        let ok = exe::call_va!(fn_70, (arg_68) -> u8);
        if ok == 0 {
            return 0;
        }
    }
    let has_data = if rd::<u64>(param_1 + 0x48) != 0 || rd::<u64>(param_1 + 0x50) != 0 {
        true
    } else {
        let p60 = rd::<usize>(param_1 + 0x60);
        p60 != 0 && rd::<u8>(p60 + 0x18) != 0
    };
    if !has_data {
        return 0;
    }
    let p0_fresh = rd::<u64>(param_1);
    let mut pair = [param_1 as u64, p0_fresh];
    let mut out_tmp = 0u64;
    let res_ptr = exe::call_va!(
        0x1_420d_cac0,
        ((&mut out_tmp as *mut u64) as usize, p0_fresh, param_2 as u32) -> usize
    );
    let w = rd::<u16>(res_ptr);
    let c2 = exe::call_va!(
        0x1_420d_9330,
        (pair.as_mut_ptr() as usize, w as u32, param_3 as u32) -> u8
    );
    if c2 != 0 {
        1
    } else {
        0
    }
}

/// `r_1413abbb0`
/// Original: 0x1413abbb0 (455 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1413abbb0(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    _param_4: usize,
    param_5: usize,
) -> usize {
    wr::<u64>(param_2, 0);
    wr::<u64>(param_2 + 0x08, 0);
    let init_cap = rd::<u32>(param_1 + 0x08) as u64;
    if init_cap != 0 {
        exe::call_va!(0x1_413a_cb60, (param_2, init_cap));
    }
    let mut cur = rd::<usize>(param_1);
    let len = rd::<u32>(param_1 + 0x08) as usize;
    let end = cur.wrapping_add(len * 8);
    while cur != end {
        let item = rd::<usize>(cur);
        let vtbl = rd::<usize>(item);
        let vfn = rd::<usize>(vtbl + 0x08);
        let id = exe::call_va!(vfn, (item) -> u32);
        let p5_cnt = rd::<usize>(param_5);
        let mut p5_cur = rd::<usize>(param_5 + 0x08);
        let p5_end = p5_cur.wrapping_add(p5_cnt * 8);
        while p5_cur != p5_end {
            if id == rd::<u32>(p5_cur) {
                let entry_u64 = rd::<u64>(p5_cur);
                let mut parsed: usize = 0;
                exe::call_va!(
                    0x1_4242_df50,
                    (param_5, (&mut parsed as *mut usize) as usize, entry_u64, item) -> usize
                );
                if parsed != 0 {
                    let v_len = rd::<u32>(param_2 + 0x08) as u64;
                    let v_cap = rd::<u32>(param_2 + 0x0c) as u64;
                    if v_len * 8 == v_cap * 8 {
                        exe::call_va!(0x1_413a_cb60, (param_2, v_len + 1));
                    }
                    let idx = rd::<u32>(param_2 + 0x08) as usize;
                    let data = rd::<usize>(param_2);
                    wr::<usize>(data.wrapping_add(idx * 8), parsed);
                    wr::<u32>(param_2 + 0x08, (idx as u32).wrapping_add(1));
                }
                break;
            }
            p5_cur = p5_cur.wrapping_add(8);
        }
        cur = cur.wrapping_add(8);
    }
    param_2
}

/// `unknown_141dfaaa0`
/// Original: 0x141dfaaa0 (1001 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141dfaaa0(param_1: usize, param_2: usize) -> u8 {
    if rd::<u8>(param_1 + 0x118) != 0 {
        return 1;
    }
    let base = rd::<usize>(param_2).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_2 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            base as u64,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );
    let iter_base = iter_init[0] as usize;
    let mut cur_arch = iter_init[1] as usize;
    let end_arch = iter_init[2] as usize;
    let mut count: u64 = 0;
    let mut idx: u64 = 0;
    let mut chunk_ptr: usize = 0;
    if cur_arch != end_arch {
        let arch_id = rd::<u16>(cur_arch) as usize;
        chunk_ptr = rd::<usize>(base.wrapping_add(arch_id * 8).wrapping_add(0x48));
        if chunk_ptr != 0 {
            count = rd::<u32>(base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64;
        }
    }
    let mut comp_base: usize = 0;
    if cur_arch != end_arch {
        let arch_id = rd::<u16>(cur_arch) as usize;
        let off_32 = arch_id * 32;
        let arg1 = rd::<u64>(base.wrapping_add(off_32).wrapping_add(0x18058));
        let arg2 = rd::<u32>(base.wrapping_add(off_32).wrapping_add(0x1806c));
        let arg3 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize);
        let found = exe::call_va!(0x1_4248_9ee0, (arg1, arg2 as u64, arg3 as u32) -> i32);
        if found != -1 {
            let off_arr = rd::<usize>(base.wrapping_add((arch_id + 0xc03) * 32));
            let rel_off =
                rd::<u32>((off_arr as isize + (found as isize) * 4) as usize) as usize;
            if rel_off != 0 {
                comp_base = chunk_ptr.wrapping_add(rel_off);
            }
        }
    }
    if count == 0 {
        return 0;
    }

    let mut local_str = [0u64; 8];
    let local_str_ptr = local_str.as_mut_ptr() as usize;

    while idx < count {
        while idx < count {
            let elem = comp_base.wrapping_add((idx as usize).wrapping_mul(0x140));
            let eq1 = exe::call_va!(0x1_401e_6ff0, (elem, param_1) -> u8);
            let mut has_local_str = false;
            let mut matched = false;
            if eq1 == 0 {
                let sub_idx = rd::<i32>(elem + 0x11c) as isize;
                let sub_len = rd::<u32>(elem + 0x128) as usize;
                if sub_idx > -1 && (sub_idx as usize) < sub_len {
                    let sub_arr = rd::<usize>(elem + 0x120);
                    let sub_elem = sub_arr.wrapping_add((sub_idx as usize).wrapping_mul(0x28));
                    wr::<u64>(local_str_ptr, 0x0400_0004);
                    wr::<u8>(local_str_ptr + 0x08, 0);
                    if local_str_ptr != sub_elem {
                        let (s_ptr, s_len) = inplace_str_ptr_len(sub_elem);
                        let view = [s_ptr as u64, s_len as u64];
                        exe::call_va!(
                            0x1_4240_9b50,
                            (local_str_ptr, view.as_ptr() as usize, 0u64, s_len as u64)
                        );
                    }
                } else {
                    wr::<u64>(local_str_ptr, 0x0400_0004);
                    wr::<u8>(local_str_ptr + 0x08, 0);
                    let empty_view = [exe::ptr::<u8>(0x1_426c_dcb0) as u64, 0u64];
                    exe::call_va!(
                        0x1_4240_9b50,
                        (local_str_ptr, empty_view.as_ptr() as usize, 0u64, 0u64)
                    );
                }
                has_local_str = true;
                if inplace_str_eq_nocase(local_str_ptr, param_1) {
                    let rec = exe::call_va!(0x1_41df_aaa0, (elem, param_2) -> u8);
                    if rec != 0 {
                        matched = true;
                    }
                }
            }
            if has_local_str {
                exe::call_va!(0x1_4240_ab10, (local_str_ptr));
            }
            if matched {
                return 1;
            }
            idx += 1;
        }
        cur_arch = cur_arch.wrapping_add(2);
        if cur_arch != end_arch {
            loop {
                let a_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_base.wrapping_add(a_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
                if cur_arch == end_arch {
                    break;
                }
            }
            if cur_arch != end_arch {
                let a_id = rd::<u16>(cur_arch) as usize;
                let ch_ptr = rd::<usize>(base.wrapping_add(a_id * 8).wrapping_add(0x48));
                count = if ch_ptr != 0 {
                    rd::<u32>(base.wrapping_add(a_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                idx = 0;
                let off_32 = a_id * 32;
                let arg1 = rd::<u64>(base.wrapping_add(off_32).wrapping_add(0x18058));
                let arg2 = rd::<u32>(base.wrapping_add(off_32).wrapping_add(0x1806c));
                let arg3 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize);
                let found =
                    exe::call_va!(0x1_4248_9ee0, (arg1, arg2 as u64, arg3 as u32) -> i32);
                comp_base = if found == -1 {
                    0
                } else {
                    let off_arr = rd::<usize>(base.wrapping_add((a_id + 0xc03) * 32));
                    let rel_off =
                        rd::<u32>((off_arr as isize + (found as isize) * 4) as usize) as usize;
                    if rel_off != 0 {
                        ch_ptr.wrapping_add(rel_off)
                    } else {
                        0
                    }
                };
            }
        }
    }
    0
}

/// `unknown_141e56ff0`
/// Original: 0x141e56ff0 (425 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e56ff0(
    param_1: usize,
    _param_2: usize,
    param_3: usize,
) {
    let r11 = rd::<u64>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );
    let mut outer = [0u64; 11];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_40fa_e290, (outer.as_mut_ptr() as usize));
    let mut rdi = outer[6] as usize;
    let mut rsi = outer[5] as usize;
    while rdi < rsi {
        while rdi < rsi {
            let p_38 = outer[7] as usize;
            let p_40 = outer[8] as usize;
            let p_48 = outer[9] as usize;
            if rd::<u8>(p_40.wrapping_add(rdi)) != 0 {
                if rd::<u8>(p_48.wrapping_add(rdi * 2)) != 0
                    && rd::<u8>(p_48.wrapping_add(rdi * 2 + 1)) == 0
                {
                    let cnt = rd::<u8>(p_38.wrapping_add(rdi));
                    let fire = if cnt == 0 {
                        true
                    } else {
                        let next_cnt = cnt.wrapping_sub(1);
                        wr::<u8>(p_38.wrapping_add(rdi), next_cnt);
                        next_cnt == 0
                    };
                    if fire {
                        let cp = outer[4] as usize;
                        let val = rd::<u64>(cp.wrapping_add(rdi * 8).wrapping_add(0x10));
                        exe::call_va!(0x1_40fb_0db0, (param_3, val));
                    }
                }
                wr::<u8>(p_40.wrapping_add(rdi), 0);
            }
            rdi += 1;
            outer[6] = rdi as u64;
        }
        let mut cur_arch = (outer[2] as usize).wrapping_add(2);
        outer[2] = cur_arch as u64;
        let end_arch = outer[3] as usize;
        if cur_arch != end_arch {
            let iter_base = outer[1] as usize;
            loop {
                let arch_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_base.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
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
                let arch_id = rd::<u16>(cur_arch) as usize;
                let base_0 = outer[0] as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_40fa_e290, (outer.as_mut_ptr() as usize));
        rdi = outer[6] as usize;
        rsi = outer[5] as usize;
    }
}

/// `unknown_141f3aa30`
/// Original: 0x141f3aa30 (137 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f3aa30(param_1: usize, param_2: usize) {
    let g_flag = exe::ptr::<u8>(0x1_4385_3f40) as usize;
    if rd::<u8>(g_flag) == 0 {
        return;
    }
    let p1_0 = rd::<usize>(param_1);
    let p1_1 = rd::<usize>(param_1 + 0x08);
    let p1_3 = rd::<usize>(param_1 + 0x18);
    let flag_ptr = p1_1.wrapping_add(p1_3);
    let vec_ptr = p1_0.wrapping_add(p1_3.wrapping_mul(0x20)).wrapping_add(0x10);
    let arg2 = rd::<[u64; 2]>(vec_ptr);
    let arg1 = rd::<[u64; 3]>(param_2);
    let res = exe::call_va!(
        0x1_41f3_aae0,
        (arg1.as_ptr() as usize, arg2.as_ptr() as usize) -> u8
    );
    let b = rd::<u8>(flag_ptr);
    let new_b = if res != 0 { b | 2 } else { b & !2 };
    wr::<u8>(flag_ptr, new_b);
}

/// `unknown_1423e0900`
/// Original: 0x1423e0900 (91 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1423e0900(param_1: usize, param_2: u64) {
    if param_2 == 0 {
        return;
    }
    let tbl_base = exe::ptr::<usize>(0x1_43be_60a8) as usize;
    let mut cur = param_1;
    let mut rem = param_2;
    while rem != 0 {
        let v = rd::<u32>(cur);
        let lo = (v & 0xffff) as usize;
        let hi = (v >> 16) as usize;
        let base_ptr = rd::<usize>(tbl_base.wrapping_add(hi * 8));
        let target = base_ptr.wrapping_add(lo.wrapping_mul(0x98));
        exe::call_va!(0x1_4230_4820, (target, cur.wrapping_add(0x10)));
        cur = cur.wrapping_add(0x40);
        rem -= 1;
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_unknown_1401e6ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let make_inline = |buf: &mut [u8; 40], s: &[u8]| -> usize {
        let p = buf.as_mut_ptr() as usize;
        wr::<u32>(p, 0x0400_0004);
        wr::<u32>(p + 4, s.len() as u32);
        for (i, &b) in s.iter().enumerate() {
            wr::<u8>(p + 8 + i, b);
        }
        wr::<u8>(p + 8 + s.len(), 0);
        p
    };

    let run = |f: F| -> (u8, u8, u8) {
        let mut b1 = [0u8; 40];
        let mut b2 = [0u8; 40];
        let mut b3 = [0u8; 40];
        let mut b4 = [0u8; 40];
        let heap_bytes = *b"HELLO\0";
        let s1 = make_inline(&mut b1, b"Hello");
        let s2 = make_inline(&mut b2, b"hELLO");
        let s3 = make_inline(&mut b3, b"World");
        let s4 = b4.as_mut_ptr() as usize;
        wr::<u32>(s4, 0x0400_0008);
        wr::<u32>(s4 + 4, 5);
        wr::<usize>(s4 + 8, heap_bytes.as_ptr() as usize);
        (f(s1, s2), f(s1, s3), f(s1, s4))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1401e6ff0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1411f4260(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_3160(_p1: usize, _p2: u16) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_cac0(out: usize, _p0: u64, p2: u16) -> usize {
        wr::<u16>(out, p2.wrapping_add(10));
        out
    }
    unsafe extern "win64" fn mock_9330(pair_ptr: usize, w: u16, p3: u8) -> u8 {
        if rd::<usize>(pair_ptr) != 0 && w == 17 && p3 == 3 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_cb(arg: u64) -> u8 {
        (arg == 0x99) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_3160, mock_3160 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_cac0, mock_cac0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_9330, mock_9330 as *const ())?;

    let run = |f: F| -> (u8, u8) {
        let mut p0 = vec![0u8; 0x5000];
        let p0p = p0.as_mut_ptr() as usize;
        wr::<u64>(p0p + 0x4fc0, 0x1234_5678);

        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, p0p);
        wr::<u64>(p1p + 0x08, 0x1234_5678);
        wr::<u64>(p1p + 0x48, 1);
        wr::<u64>(p1p + 0x68, 0x99);
        wr::<usize>(p1p + 0x70, mock_cb as *const () as usize);

        (f(p1p, 7, 3), f(p1p, 8, 3))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1411f4260 out");
    Ok(1)
}

pub unsafe fn diff_r_1413abbb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_reserve(vec_ptr: usize, new_cap: u64) {
        let buf = rd::<usize>(vec_ptr + 0x10);
        wr::<usize>(vec_ptr, buf);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_parse(
        _p5: usize,
        out_ptr: usize,
        entry_val: u64,
        _item_ptr: usize,
    ) -> usize {
        wr::<u64>(out_ptr, entry_val.wrapping_add(0x1000));
        out_ptr
    }
    unsafe extern "win64" fn mock_get_id(item: usize) -> u32 {
        rd::<u32>(item + 0x08)
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_cb60, mock_reserve as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_df50, mock_parse as *const ())?;

    let run = |f: F| -> (bool, u32, u32, u64) {
        let vtbl = [0usize, mock_get_id as *const () as usize];
        let item0 = [vtbl.as_ptr() as u64, 42u64];
        let items = [item0.as_ptr() as usize];
        let p1 = [items.as_ptr() as u64, 1u64];

        let mut backing = [0u64; 4];
        let mut p2 = [0u64, 0u64, backing.as_mut_ptr() as u64, 0u64];
        let p2p = p2.as_mut_ptr() as usize;

        let p5_entries = [0x0005_0000_0000_002au64];
        let p5 = [1u64, p5_entries.as_ptr() as u64];

        let ret = f(p1.as_ptr() as usize, p2p, 0, 0, p5.as_ptr() as usize);
        (
            ret == p2p,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x0c),
            backing[0],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1413abbb0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141dfaaa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_find(_p: u64, _n: u64, _w: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_eq_nocase(_a: usize, _b: usize) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_assign(dst: usize, view: usize, _off: u64, count: u64) {
        let src = rd::<usize>(view);
        let n = (count as usize).min(24);
        wr::<u32>(dst + 0x04, n as u32);
        for i in 0..n {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + i));
        }
        wr::<u8>(dst + 0x08 + n, 0);
    }
    unsafe extern "win64" fn noop1(_a: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_eq_nocase as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_assign as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, noop1 as *const ())?;

    let run = |f: F, sub_name: &[u8; 3]| -> u8 {
        let mut p1 = [0u8; 0x140];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p, 0x0400_0004);
        wr::<u32>(p1p + 0x04, 3);
        wr::<[u8; 3]>(p1p + 0x08, *b"abc");
        wr::<u8>(p1p + 0x0b, 0);
        wr::<u8>(p1p + 0x118, 0);

        let mut sub_elem = [0u8; 0x28];
        let sep = sub_elem.as_mut_ptr() as usize;
        wr::<u32>(sep, 0x0400_0004);
        wr::<u32>(sep + 0x04, 3);
        wr::<[u8; 3]>(sep + 0x08, *sub_name);
        wr::<u8>(sep + 0x0b, 0);

        let mut chunk = [0u8; 0x180];
        let cp = chunk.as_mut_ptr() as usize;
        let elem = cp + 0x40;
        wr::<u32>(elem, 0x0400_0004);
        wr::<u32>(elem + 0x04, 0);
        wr::<u8>(elem + 0x118, 1);
        wr::<i32>(elem + 0x11c, 0);
        wr::<usize>(elem + 0x120, sep);
        wr::<u32>(elem + 0x128, 1);

        let off_arr = [0x40u32];
        let mut world = vec![0u8; 0x18200];
        let wp = world.as_mut_ptr() as usize;
        let base = wp + 0x40;
        wr::<usize>(base + 0x48, cp);
        wr::<u32>(base + 0x10048, 1);
        wr::<usize>(base + 0xc03 * 32, off_arr.as_ptr() as usize);

        let arch_list = [0u16];
        let p2 = [
            wp as u64,
            arch_list.as_ptr() as u64,
            arch_list.as_ptr().add(1) as u64,
        ];
        f(p1p, p2.as_ptr() as usize)
    };

    let o = (run(f_o, b"ABC"), run(f_o, b"XYZ"));
    let n = (run(f_n, b"ABC"), run(f_n, b"XYZ"));
    crate::diff_eq!(o, n, "141dfaaa0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e56ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_fae290(outer: usize) {
        let cp = rd::<usize>(outer + 0x20);
        if cp != 0 {
            wr::<u64>(outer + 0x38, rd::<u64>(cp + 0x20));
            wr::<u64>(outer + 0x40, rd::<u64>(cp + 0x28));
            wr::<u64>(outer + 0x48, rd::<u64>(cp + 0x30));
        }
    }
    unsafe extern "win64" fn mock_fb0db0(p3: usize, val: u64) {
        wr::<u64>(p3, rd::<u64>(p3).wrapping_add(val));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fa_e290, mock_fae290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40fb_0db0, mock_fb0db0 as *const ())?;

    let run = |f: F| -> (u64, [u8; 2], [u8; 2]) {
        let mut p_38 = [1u8, 2u8];
        let mut p_40 = [1u8, 1u8];
        let p_48 = [1u8, 0u8, 1u8, 0u8];
        let mut chunk = [0u64; 8];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x10, 0x1111);
        wr::<u64>(cp + 0x18, 0x2222);
        wr::<usize>(cp + 0x20, p_38.as_mut_ptr() as usize);
        wr::<usize>(cp + 0x28, p_40.as_mut_ptr() as usize);
        wr::<usize>(cp + 0x30, p_48.as_ptr() as usize);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        let base = wp + 0x40;
        wr::<usize>(base + 0x48, cp);
        wr::<u32>(base + 0x10048, 2);

        let arch_list = [0u16];
        let p1 = [
            wp as u64,
            arch_list.as_ptr() as u64,
            arch_list.as_ptr().add(1) as u64,
        ];
        let mut out_sum = 0u64;
        f(p1.as_ptr() as usize, 0, (&mut out_sum as *mut u64) as usize);
        (out_sum, p_38, p_40)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e56ff0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f3aa30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_aae0(a1: usize, a2: usize) -> u8 {
        if rd::<u64>(a1) == 0x11 && rd::<u64>(a1 + 0x10) == 0x33 && rd::<u64>(a2) == 0x44 {
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f3_aae0, mock_aae0 as *const ())?;

    let g_flag = exe::ptr::<u8>(0x1_4385_3f40) as usize;
    let s_flag = rd::<u8>(g_flag);
    wr::<u8>(g_flag, 1);

    let run = |f: F| -> u8 {
        let mut arr = [0u8; 0x40];
        let ap = arr.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x20 + 0x10, 0x44);
        wr::<u64>(ap + 0x20 + 0x18, 0x55);

        let mut flags = [0x01u8, 0x01u8];
        let p1 = [ap as u64, flags.as_mut_ptr() as u64, 0u64, 1u64];
        let p2 = [0x11u64, 0x22u64, 0x33u64];

        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        flags[1]
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u8>(g_flag, s_flag);
    crate::diff_eq!(o, n, "141f3aa30 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1423e0900(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_4820(target: usize, src: usize) {
        wr::<u32>(target, rd::<u32>(src));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4230_4820, mock_4820 as *const ())?;

    let g_tbl = exe::ptr::<[usize; 2]>(0x1_43be_60a8) as usize;
    let s_tbl = rd::<[usize; 2]>(g_tbl);

    let run = |f: F| -> (u32, u32) {
        let mut targets0 = [0u8; 0x98 * 2];
        let mut targets1 = [0u8; 0x98 * 2];
        let t0 = targets0.as_mut_ptr() as usize;
        let t1 = targets1.as_mut_ptr() as usize;
        wr::<[usize; 2]>(g_tbl, [t0, t1]);

        let mut items = [0u8; 0x80];
        let ip = items.as_mut_ptr() as usize;
        wr::<u32>(ip, 0x0000_0001);
        wr::<u32>(ip + 0x10, 0xdead_beef);
        wr::<u32>(ip + 0x40, 0x0001_0000);
        wr::<u32>(ip + 0x50, 0xcafe_babe);

        f(ip, 2);
        (rd::<u32>(t0 + 0x98), rd::<u32>(t1))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<[usize; 2]>(g_tbl, s_tbl);
    crate::diff_eq!(o, n, "1423e0900 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_401e_6ff0, "unknown_1401e6ff0", rend::cauldron_hot_a::unknown_1401e6ff0 as *const (), rend::cauldron_hot_a::diff_unknown_1401e6ff0),
// Replacement::verified_throws(0x1_411f_4260, "unknown_1411f4260", rend::cauldron_hot_a::unknown_1411f4260 as *const (), Throws::Discard, rend::cauldron_hot_a::diff_unknown_1411f4260),
// Replacement::verified_throws(0x1_413a_bbb0, "r_1413abbb0", rend::cauldron_hot_a::r_1413abbb0 as *const (), Throws::Discard, rend::cauldron_hot_a::diff_r_1413abbb0),
// Replacement::verified(0x1_41df_aaa0, "unknown_141dfaaa0", rend::cauldron_hot_a::unknown_141dfaaa0 as *const (), rend::cauldron_hot_a::diff_unknown_141dfaaa0),
// Replacement::verified(0x1_41e5_6ff0, "unknown_141e56ff0", rend::cauldron_hot_a::unknown_141e56ff0 as *const (), rend::cauldron_hot_a::diff_unknown_141e56ff0),
// Replacement::verified(0x1_41f3_aa30, "unknown_141f3aa30", rend::cauldron_hot_a::unknown_141f3aa30 as *const (), rend::cauldron_hot_a::diff_unknown_141f3aa30),
// Replacement::verified(0x1_423e_0900, "unknown_1423e0900", rend::cauldron_hot_a::unknown_1423e0900 as *const (), rend::cauldron_hot_a::diff_unknown_1423e0900),
