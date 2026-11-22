use crate::exe::{self, rd, wr};

#[inline]
unsafe fn read_teb() -> usize {
    exe::teb_self()
}

/// Original: `0x14204eef0` (209 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204eef0(rcx: usize) {
    let r14 = rcx;
    let eax = rd::<u32>(rcx + 0x14).wrapping_mul(rd::<u32>(rcx + 0x10));
    let edi = eax.wrapping_mul(6);
    let rsi = (edi as i32 as isize) as usize;
    let ebp = rd::<u32>(rcx + 0x28) as usize;
    let cap = rd::<u32>(r14 + 0x2c) as usize;

    if rsi > cap {
        let mut rdx = cap.wrapping_add(cap >> 1);
        if rdx < rsi {
            rdx = rsi;
        }
        exe::call_va!(0x1_40fb_1b60, (r14 + 0x20, rdx) -> ());
        let cur_len = rd::<u32>(r14 + 0x28) as usize;
        let dst = rd::<usize>(r14 + 0x20).wrapping_add(cur_len << 5);
        let r8 = rsi.wrapping_sub(ebp);
        if r8 != 0 {
            exe::call_va!(0x1_4183_acc2, (dst, 0usize, r8 << 5) -> ());
        }
        wr::<u32>(r14 + 0x28, edi);
    } else if rsi > ebp {
        let dst = rd::<usize>(r14 + 0x20).wrapping_add(ebp << 5);
        let r8 = rsi.wrapping_sub(ebp);
        exe::call_va!(0x1_4183_acc2, (dst, 0usize, r8 << 5) -> ());
        wr::<u32>(r14 + 0x28, edi);
    } else if rsi != ebp {
        wr::<u32>(r14 + 0x28, edi);
    }

    exe::call_va!(0x1_411d_2ea0, (r14 + 0x30, rsi) -> ());
    exe::call_va!(0x1_4183_acc2, (rd::<usize>(r14 + 0x20), 0usize, rsi << 5) -> ());
    exe::call_va!(0x1_4183_acc2, (rd::<usize>(r14 + 0x30), 0usize, rsi << 3) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_14204eef0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_140fb1b60(vec_ptr: usize, new_cap: usize) {
        wr::<u32>(vec_ptr + 0xc, new_cap as u32);
    }
    unsafe extern "win64" fn fake_1411d2ea0(vec_ptr: usize, new_len: usize) {
        wr::<u32>(vec_ptr + 0x8, new_len as u32);
    }
    let _m1 = crate::difftest::Mock::new(0x1_40fb_1b60, fake_140fb1b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_2ea0, fake_1411d2ea0 as *const ())?;

    let mut cases = 0u32;
    for (cur_len, cap) in [(2u32, 4u32), (2u32, 8u32), (8u32, 8u32)] {
        let mut obj_o = [0u8; 0x48];
        let mut obj_n = [0u8; 0x48];
        let mut buf20_o = [0xaa_u8; 256];
        let mut buf20_n = [0xaa_u8; 256];
        let mut buf30_o = [0xbb_u8; 64];
        let mut buf30_n = [0xbb_u8; 64];

        for (obj, b20, b30) in [
            (&mut obj_o, &mut buf20_o, &mut buf30_o),
            (&mut obj_n, &mut buf20_n, &mut buf30_n),
        ] {
            let p = obj.as_mut_ptr() as usize;
            wr::<u32>(p + 0x10, 1);
            wr::<u32>(p + 0x14, 1); // rsi = 6
            wr::<usize>(p + 0x20, b20.as_mut_ptr() as usize);
            wr::<u32>(p + 0x28, cur_len);
            wr::<u32>(p + 0x2c, cap);
            wr::<usize>(p + 0x30, b30.as_mut_ptr() as usize);
        }

        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x20, 0);
        wr::<usize>(obj_n.as_mut_ptr() as usize + 0x20, 0);
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x30, 0);
        wr::<usize>(obj_n.as_mut_ptr() as usize + 0x30, 0);
        crate::diff_eq!(obj_o, obj_n, "14204eef0 obj");
        crate::diff_eq!(buf20_o, buf20_n, "14204eef0 buf20");
        crate::diff_eq!(buf30_o, buf30_n, "14204eef0 buf30");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x14204efd0` (148 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204efd0(rcx: usize, edx: u32, r8d: u32) -> usize {
    let r11d = edx;
    let eax = r8d.wrapping_mul(rd::<u32>(rcx + 0x14)).wrapping_mul(3);
    let r9d = edx.wrapping_add(eax.wrapping_mul(2));
    let r10 = (r9d as i32 as isize) as usize;
    let arr = rd::<usize>(rcx + 0x30);
    let rbx = arr.wrapping_add(r10.wrapping_mul(8));
    if rd::<usize>(rbx) == 0 {
        let rdx_ptr = rd::<usize>(rcx);
        let flag = rd::<u8>(rdx_ptr + 0x18) as usize;
        let mut out_ptr: usize = 0;
        let out_addr = (&mut out_ptr as *mut usize) as usize;
        exe::call_va!(0x1_4206_73e0, (out_addr, rdx_ptr, r8d as usize, r11d as usize, flag) -> ());
        let rax = rd::<usize>(out_addr);
        wr::<usize>(out_addr, 0);
        let old = rd::<usize>(rbx);
        wr::<usize>(rbx, rax);
        if old != 0 {
            exe::call_va!(0x1_4206_b020, (old) -> ());
            exe::call_va!(0x1_4244_2de0, (old) -> ());
            let rem = rd::<usize>(out_addr);
            if rem != 0 {
                exe::call_va!(0x1_4206_b020, (rem) -> ());
                exe::call_va!(0x1_4244_2de0, (rem) -> ());
            }
        }
    }
    rd::<usize>(rbx)
}

pub unsafe fn diff_d3d_tls_ctx_14204efd0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1420673e0(
        out: usize,
        rdx_ptr: usize,
        r8d: u32,
        r9d: u32,
        flag: u8,
    ) {
        let tag = (rd::<u32>(rdx_ptr) as usize)
            ^ ((r8d as usize) << 16)
            ^ ((r9d as usize) << 8)
            ^ (flag as usize);
        wr::<usize>(out, tag | 0x4000_0000);
    }
    let _m = crate::difftest::Mock::new(0x1_4206_73e0, fake_1420673e0 as *const ())?;

    let mut sub = [0u8; 0x20];
    wr::<u32>(sub.as_mut_ptr() as usize, 0x1234);
    wr::<u8>(sub.as_mut_ptr() as usize + 0x18, 1);

    let mut arr_o = [0usize; 32];
    let mut arr_n = [0usize; 32];
    let mut obj_o = [0u8; 0x40];
    let mut obj_n = [0u8; 0x40];
    for (obj, arr) in [(&mut obj_o, &mut arr_o), (&mut obj_n, &mut arr_n)] {
        let p = obj.as_mut_ptr() as usize;
        wr::<usize>(p, sub.as_ptr() as usize);
        wr::<u32>(p + 0x14, 2);
        wr::<usize>(p + 0x30, arr.as_mut_ptr() as usize);
    }

    // 1. Lazy creation when slot is 0
    let ro1 = exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 1u32, 1u32) -> usize);
    let rn1 = exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 1u32, 1u32) -> usize);
    crate::diff_eq!(ro1, rn1, "14204efd0 lazy create");
    crate::diff_eq!(arr_o, arr_n, "14204efd0 arr after create");

    // 2. Cached hit when slot is non-zero
    let ro2 = exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 1u32, 1u32) -> usize);
    let rn2 = exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 1u32, 1u32) -> usize);
    crate::diff_eq!(ro2, rn2, "14204efd0 cached hit");
    Ok(2)
}

/// Original: `0x14204f090` (60 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204f090(rcx: usize) {
    let rbx = rcx;
    exe::call_va!(0x1_411d_3020, (rbx + 0x30) -> ());
    exe::call_va!(0x1_401d_e830, (rbx + 0x20) -> ());
    let p0 = rd::<usize>(rbx);
    if p0 != 0 {
        exe::call_va!(0x1_4206_b020, (p0) -> ());
        exe::call_va!(0x1_4244_2de0, (p0) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_14204f090(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_3020(rcx: usize) {
        wr::<u32>(rcx, 0x3020);
    }
    unsafe extern "win64" fn fake_e830(rcx: usize) {
        wr::<u32>(rcx, 0xe830);
    }
    unsafe extern "win64" fn fake_b020(rcx: usize) {
        wr::<u32>(rcx, rd::<u32>(rcx).wrapping_add(0xb020));
    }
    unsafe extern "win64" fn fake_2de0(rcx: usize) {
        wr::<u32>(rcx + 0x4, rd::<u32>(rcx + 0x4).wrapping_add(0x2de0));
    }
    let _m1 = crate::difftest::Mock::new(0x1_411d_3020, fake_3020 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, fake_e830 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4206_b020, fake_b020 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4244_2de0, fake_2de0 as *const ())?;

    let mut sub_o = [0u8; 16];
    let mut sub_n = [0u8; 16];
    let mut obj_o = [0u8; 0x40];
    let mut obj_n = [0u8; 0x40];
    wr::<usize>(obj_o.as_mut_ptr() as usize, sub_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize, sub_n.as_mut_ptr() as usize);

    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
    wr::<usize>(obj_o.as_mut_ptr() as usize, 0);
    wr::<usize>(obj_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(obj_o, obj_n, "14204f090 obj");
    crate::diff_eq!(sub_o, sub_n, "14204f090 sub");
    Ok(1)
}

/// Original: `0x14204f2f0` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204f2f0(rcx: usize, _rdx: usize, r8: usize) {
    let rbx = rcx;
    let a2c = rd::<u32>(rcx + 0x2c) as usize;
    let r9 = rd::<usize>(rcx + 0x20);
    let a28 = rd::<u32>(rcx + 0x28) as usize;
    let p0 = rd::<usize>(rcx);
    exe::call_va!(0x1_4206_a360, (p0, 0usize, r8, r9, a28, a2c) -> ());
    let p20 = rd::<usize>(rbx + 0x20);
    if p20 != 0 {
        exe::call_va!(0x1_4244_2de0, (p20) -> ());
    }
    wr::<usize>(rbx + 0x20, 0);
}

pub unsafe fn diff_d3d_tls_ctx_14204f2f0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_a360(
        p0: usize,
        rdx: u32,
        r8: usize,
        r9: usize,
        a28: u32,
        a2c: u32,
    ) {
        wr::<u32>(p0, rdx);
        wr::<usize>(p0 + 0x8, r8);
        wr::<u32>(p0 + 0x10, rd::<u32>(r9));
        wr::<u32>(p0 + 0x14, a28);
        wr::<u32>(p0 + 0x18, a2c);
    }
    unsafe extern "win64" fn fake_2de0(p20: usize) {
        wr::<u32>(p20 + 0x4, 0x2de0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4206_a360, fake_a360 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_2de0 as *const ())?;

    let mut p0_o = [0u8; 0x20];
    let mut p0_n = [0u8; 0x20];
    let mut p20_o = [0x11_u8; 0x10];
    let mut p20_n = [0x11_u8; 0x10];
    let mut obj_o = [0u8; 0x38];
    let mut obj_n = [0u8; 0x38];
    for (obj, p0, p20) in [
        (&mut obj_o, &mut p0_o, &mut p20_o),
        (&mut obj_n, &mut p0_n, &mut p20_n),
    ] {
        let p = obj.as_mut_ptr() as usize;
        wr::<usize>(p, p0.as_mut_ptr() as usize);
        wr::<usize>(p + 0x20, p20.as_mut_ptr() as usize);
        wr::<u32>(p + 0x28, 0x1234);
        wr::<u32>(p + 0x2c, 0x5678);
    }

    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 99usize, 0xdead_beefusize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 99usize, 0xdead_beefusize) -> ());
    wr::<usize>(obj_o.as_mut_ptr() as usize, 0);
    wr::<usize>(obj_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(obj_o, obj_n, "14204f2f0 obj");
    crate::diff_eq!(p0_o, p0_n, "14204f2f0 p0");
    crate::diff_eq!(p20_o, p20_n, "14204f2f0 p20");
    Ok(1)
}

/// Original: `0x14204f450` (124 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204f450(rcx: usize) -> usize {
    let rsi = rcx;
    let r8d = rd::<u8>(rcx + 0x1c) as u32;
    let edi = rd::<i32>(rcx + 0xc).max(1) as u32;
    let edx = rd::<i32>(rcx + 0x10).max(1) as u32;
    let eax1 = exe::call_va!(0x1_4204_e7f0, (edi, edx, r8d) -> u32);
    let r11d = (rd::<i32>(rsi + 0x14).max(1) as u32).wrapping_mul(eax1);
    wr::<u32>(rsi + 0x2c, eax1);
    let r8d_2 = rd::<u8>(rsi + 0x1c) as u32;
    let eax2 = exe::call_va!(0x1_4204_e7f0, (edi, 1u32, r8d_2) -> u32);
    let r9 = (r11d as i32 as isize) as usize;
    wr::<u32>(rsi + 0x28, eax2);
    let ptr = exe::call_va!(0x1_4204_f340, (edi as usize, 1usize, r8d_2 as usize, r9) -> usize);
    wr::<usize>(rsi + 0x20, ptr);
    ptr
}

pub unsafe fn diff_d3d_tls_ctx_14204f450(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_f340(ecx: u32, edx: u32, r8d: u32, r9: usize) -> usize {
        ((ecx as usize) << 32) ^ ((edx as usize) << 24) ^ ((r8d as usize) << 16) ^ r9
    }
    let _m = crate::difftest::Mock::new(0x1_4204_f340, fake_f340 as *const ())?;

    let mut cases = 0u32;
    for (w, h, d, fmt) in [(0i32, 0i32, 0i32, 1u8), (64i32, 32i32, 4i32, 28u8)] {
        let mut obj_o = [0u8; 0x38];
        let mut obj_n = [0u8; 0x38];
        for obj in [&mut obj_o, &mut obj_n] {
            let p = obj.as_mut_ptr() as usize;
            wr::<i32>(p + 0xc, w);
            wr::<i32>(p + 0x10, h);
            wr::<i32>(p + 0x14, d);
            wr::<u8>(p + 0x1c, fmt);
        }
        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(obj_o, obj_n, "14204f450 obj");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x14204f860` (179 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_14204f860(rcx: usize) {
    let rbx_in = rcx;
    if rd::<u8>(rcx + 0x1c) != 0 {
        wr::<usize>(rcx, 0);
    }
    exe::call_va!(0x1_411d_3020, (rcx + 0x20) -> ());
    let rbx = rd::<usize>(rbx_in);
    if rbx != 0 {
        exe::call_va!(0x1_4206_b020, (rbx) -> ());
        let r10 = rbx.wrapping_sub(1) & !0x1ff_ffffusize;
        let teb = read_teb();
        let dl = teb == rd::<usize>(r10 + 0x100);
        let idx = (rbx.wrapping_sub(r10) >> 16).wrapping_mul(5) << 4;
        let mut chunk = r10.wrapping_add(0x108).wrapping_add(idx);
        let off = rd::<u32>(chunk + 0x4) as usize;
        chunk = chunk.wrapping_sub(off);
        let r8: usize = if dl {
            if rd::<u8>(chunk + 0xe) == 0 {
                let next = rd::<usize>(chunk + 0x20);
                wr::<usize>(rbx, next);
                wr::<usize>(chunk + 0x20, rbx);
                let new_cnt = rd::<u32>(chunk + 0x18).wrapping_sub(1);
                wr::<u32>(chunk + 0x18, new_cnt);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (chunk) -> ());
                }
                return;
            }
            1
        } else {
            0
        };
        exe::call_va!(0x1_4253_3200, (r10, chunk, r8, rbx) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_14204f860(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_3020(rcx: usize) {
        wr::<u32>(rcx, 0x3020);
    }
    let _m = crate::difftest::Mock::new(0x1_411d_3020, fake_3020 as *const ())?;

    let mut cases = 0u32;
    for (p0, flag) in [(0x1234_5678usize, 1u8), (0usize, 0u8)] {
        let mut obj_o = [0u8; 0x38];
        let mut obj_n = [0u8; 0x38];
        for obj in [&mut obj_o, &mut obj_n] {
            let p = obj.as_mut_ptr() as usize;
            wr::<usize>(p, p0);
            wr::<u8>(p + 0x1c, flag);
        }
        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(obj_o, obj_n, "14204f860 obj");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142050070` (252 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050070(rcx: usize, rdx: usize) -> usize {
    let rbx = rcx;
    let rsi = rdx;
    wr::<usize>(rcx, 0);
    wr::<usize>(rcx + 0x20, 0);
    wr::<usize>(rcx + 0x28, 0);
    wr::<u32>(rcx + 0x38, 0xffff_ffff);
    wr::<u32>(rcx + 0x3c, 0);
    wr::<usize>(rcx + 0x30, 0);
    let rdi = rd::<usize>(rcx);
    wr::<usize>(rcx, rdx);
    if rdi != 0 {
        exe::call_va!(0x1_4206_b020, (rdi) -> ());
        let r10 = rdi.wrapping_sub(1) & !0x1ff_ffffusize;
        let teb = read_teb();
        let dl = teb == rd::<usize>(r10 + 0x100);
        let idx = (rdi.wrapping_sub(r10) >> 16).wrapping_mul(5) << 4;
        let mut chunk = r10.wrapping_add(0x108).wrapping_add(idx);
        let off = rd::<u32>(chunk + 0x4) as usize;
        chunk = chunk.wrapping_sub(off);
        let mut call_slow = true;
        let mut r8: usize = 0;
        if dl {
            if rd::<u8>(chunk + 0xe) == 0 {
                let next = rd::<usize>(chunk + 0x20);
                wr::<usize>(rdi, next);
                wr::<usize>(chunk + 0x20, rdi);
                let new_cnt = rd::<u32>(chunk + 0x18).wrapping_sub(1);
                wr::<u32>(chunk + 0x18, new_cnt);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (chunk) -> ());
                }
                call_slow = false;
            } else {
                r8 = 1;
            }
        }
        if call_slow {
            exe::call_va!(0x1_4253_3200, (r10, chunk, r8, rdi) -> ());
        }
    }
    wr::<u32>(rbx + 0x8, rd::<u32>(rsi));
    wr::<u32>(rbx + 0xc, rd::<u32>(rsi + 0x4));
    wr::<u32>(rbx + 0x10, rd::<u32>(rsi + 0xc));
    wr::<u8>(rbx + 0x14, rd::<u8>(rsi + 0x18));
    wr::<u32>(rbx + 0x18, 0);
    wr::<u8>(rbx + 0x1c, 1);
    rbx
}

pub unsafe fn diff_d3d_tls_ctx_142050070(orig: usize, new: usize) -> Result<u32, String> {
    let mut src = [0u8; 0x20];
    let sp = src.as_mut_ptr() as usize;
    wr::<u32>(sp, 1920);
    wr::<u32>(sp + 0x4, 1080);
    wr::<u32>(sp + 0xc, 1);
    wr::<u8>(sp + 0x18, 28);

    let mut dst_o = [0xaa_u8; 0x40];
    let mut dst_n = [0xaa_u8; 0x40];
    let ro = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, sp) -> usize);
    let rn = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, sp) -> usize);
    crate::diff_eq!(ro, dst_o.as_ptr() as usize, "142050070 ret orig");
    crate::diff_eq!(rn, dst_n.as_ptr() as usize, "142050070 ret new");
    crate::diff_eq!(dst_o, dst_n, "142050070 dst");
    Ok(1)
}

/// Original: `0x142050180` (37 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050180(rcx: usize) -> i64 {
    let a10 = rd::<u8>(rcx + 0x10) as usize;
    let r9d = rd::<u32>(rcx + 0x8);
    let r8d = rd::<u32>(rcx + 0xc);
    let edx = rd::<u32>(rcx + 0x4);
    let ecx = rd::<u32>(rcx);
    let res = exe::call_va!(0x1_4204_e5e0, (ecx, edx, r8d, r9d, a10) -> i32);
    res as i64
}

pub unsafe fn diff_d3d_tls_ctx_142050180(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf = [0u8; 32];
    let ptr = buf.as_mut_ptr() as usize;
    wr::<u32>(ptr, 64);
    wr::<u32>(ptr + 0x4, 64);
    wr::<u32>(ptr + 0x8, 1);
    wr::<u32>(ptr + 0xc, 28);
    wr::<u8>(ptr + 0x10, 0);
    let r_orig = exe::call_va!(orig, (ptr) -> i64);
    let r_new = exe::call_va!(new, (ptr) -> i64);
    crate::diff_eq!(r_orig, r_new, "142050180 case 1");

    wr::<u32>(ptr, 128);
    wr::<u32>(ptr + 0x4, 256);
    wr::<u32>(ptr + 0x8, 4);
    wr::<u32>(ptr + 0xc, 10);
    wr::<u8>(ptr + 0x10, 1);
    let r_orig2 = exe::call_va!(orig, (ptr) -> i64);
    let r_new2 = exe::call_va!(new, (ptr) -> i64);
    crate::diff_eq!(r_orig2, r_new2, "142050180 case 2");
    Ok(2)
}

/// Original: `0x1420501f0` (55 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420501f0(rcx: usize, rdx: usize) {
    let rdi = rcx;
    let rbx = rdx;
    exe::call_va!(0x1_4207_5a20, (rdx) -> ());
    if rbx != 0 {
        exe::call_va!(0x1_4243_b3e0, (rdi + 0xf0, rbx) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_1420501f0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5a20(rdx: usize) {
        if rdx != 0 {
            wr::<u32>(rdx, 0x5a20);
        }
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, node: usize) {
        wr::<u32>(pool, rd::<u32>(node));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut obj_o = [0u8; 0x100];
    let mut obj_n = [0u8; 0x100];
    let mut node_o = [0u8; 16];
    let mut node_n = [0u8; 16];
    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, node_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize, node_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(obj_o, obj_n, "1420501f0 non-null");
    crate::diff_eq!(node_o, node_n, "1420501f0 node");

    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 0usize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 0usize) -> ());
    crate::diff_eq!(obj_o, obj_n, "1420501f0 null");
    Ok(2)
}

/// Original: `0x142050230` (158 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050230(rcx: usize, rdx: usize) -> usize {
    let rsi = rdx;
    let rbx = exe::call_va!(0x1_4243_b450, (rcx + 0xf0, rdx) -> usize);
    if rbx != 0 {
        for off in (0..=0x60).step_by(8) {
            wr::<usize>(rbx + off, 0);
        }
    }
    exe::call_va!(0x1_4207_5af0, (rsi, 0usize, 0usize, 0usize, 0x10usize, 0usize, rbx) -> ());
    wr::<usize>(rbx + 0x60, rsi);
    wr::<usize>(rbx + 0x50, 0);
    wr::<usize>(rbx + 0x58, 0);
    rbx
}

pub unsafe fn diff_d3d_tls_ctx_142050230(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_b450(pool: usize) -> usize {
        rd::<usize>(pool)
    }
    unsafe extern "win64" fn fake_5af0(
        size: usize,
        stride: usize,
        flag: u8,
        usage: u32,
        extra: u32,
        _unused6: usize,
        out_buf: usize,
    ) {
        wr::<usize>(out_buf, size ^ 0x1111);
        wr::<usize>(out_buf + 0x8, stride);
        wr::<u32>(out_buf + 0x48, (flag as u32) | (usage << 8) | (extra << 16));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4243_b450, fake_b450 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5af0, fake_5af0 as *const ())?;

    let mut obj_o = [0u8; 0x100];
    let mut obj_n = [0u8; 0x100];
    let mut node_o = [0xff_u8; 0x68];
    let mut node_n = [0xff_u8; 0x68];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0xf0, node_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0xf0, node_n.as_mut_ptr() as usize);

    let ro = exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 4096usize) -> usize);
    let rn = exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 4096usize) -> usize);
    crate::diff_eq!(ro, node_o.as_ptr() as usize, "142050230 ret orig");
    crate::diff_eq!(rn, node_n.as_ptr() as usize, "142050230 ret new");
    crate::diff_eq!(node_o, node_n, "142050230 node");
    Ok(1)
}

/// Original: `0x1420502e0` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420502e0(rcx: usize) {
    let rbx = rcx;
    let r8 = rd::<usize>(rcx + 0x18);
    let rdx = rd::<usize>(rcx + 0x10);
    let arg1 = rd::<usize>(rcx + 0x8);
    exe::call_va!(0x1_411d_3820, (arg1, rdx, r8) -> ());
    let pool = rd::<usize>(rbx);
    exe::call_va!(0x1_4243_b3e0, (pool, rbx) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_1420502e0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_3820(a1: usize, a2: usize, a3: usize) {
        wr::<usize>(a1, a2 ^ a3);
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, node: usize) {
        wr::<u32>(pool, rd::<u32>(node + 0x10));
    }
    let _m1 = crate::difftest::Mock::new(0x1_411d_3820, fake_3820 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut pool_o = [0u8; 8];
    let mut pool_n = [0u8; 8];
    let mut a1_o = [0u8; 8];
    let mut a1_n = [0u8; 8];
    let mut node_o = [0usize; 4];
    let mut node_n = [0usize; 4];
    node_o[0] = pool_o.as_mut_ptr() as usize;
    node_o[1] = a1_o.as_mut_ptr() as usize;
    node_o[2] = 0x1234_5678;
    node_o[3] = 0x9abc_def0;
    node_n[0] = pool_n.as_mut_ptr() as usize;
    node_n[1] = a1_n.as_mut_ptr() as usize;
    node_n[2] = 0x1234_5678;
    node_n[3] = 0x9abc_def0;

    exe::call_va!(orig, (node_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (node_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(pool_o, pool_n, "1420502e0 pool");
    crate::diff_eq!(a1_o, a1_n, "1420502e0 a1");
    Ok(1)
}

/// Original: `0x142050310` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050310(_rcx: usize, rdx: usize) {
    let rbx = rdx;
    let r8 = rd::<usize>(rdx + 0x18);
    let arg2 = rd::<usize>(rdx + 0x10);
    let arg1 = rd::<usize>(rbx + 0x8);
    exe::call_va!(0x1_411d_3820, (arg1, arg2, r8) -> ());
    let pool = rd::<usize>(rbx);
    exe::call_va!(0x1_4243_b3e0, (pool, rbx) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_142050310(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_3820(a1: usize, a2: usize, a3: usize) {
        wr::<usize>(a1, a2.wrapping_add(a3));
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, node: usize) {
        wr::<u32>(pool, rd::<u32>(node + 0x18));
    }
    let _m1 = crate::difftest::Mock::new(0x1_411d_3820, fake_3820 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut pool_o = [0u8; 8];
    let mut pool_n = [0u8; 8];
    let mut a1_o = [0u8; 8];
    let mut a1_n = [0u8; 8];
    let mut node_o = [0usize; 4];
    let mut node_n = [0usize; 4];
    node_o[0] = pool_o.as_mut_ptr() as usize;
    node_o[1] = a1_o.as_mut_ptr() as usize;
    node_o[2] = 0x1111_2222;
    node_o[3] = 0x3333_4444;
    node_n[0] = pool_n.as_mut_ptr() as usize;
    node_n[1] = a1_n.as_mut_ptr() as usize;
    node_n[2] = 0x1111_2222;
    node_n[3] = 0x3333_4444;

    exe::call_va!(orig, (0usize, node_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (0usize, node_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(pool_o, pool_n, "142050310 pool");
    crate::diff_eq!(a1_o, a1_n, "142050310 a1");
    Ok(1)
}

/// Original: `0x142050340` (106 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050340(
    rcx: usize,
    rdx: usize,
    r8: usize,
    r9: usize,
    arg5: usize,
) {
    let rbx = rcx + 0x70;
    let rdi = rdx;
    let rsi = r8;
    let rbp = r9;
    let node = exe::call_va!(0x1_4243_b450, (rbx) -> usize);
    wr::<usize>(node, rbx);
    wr::<usize>(node + 0x8, rdi);
    wr::<usize>(node + 0x10, rsi);
    wr::<usize>(node + 0x18, rbp);
    wr::<usize>(node + 0x20, arg5);
    let cb = exe::ptr::<u8>(0x1_4205_02e0) as usize;
    exe::call_va!(0x1_4209_a110, (node, cb, 1u32) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_142050340(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_b450(pool: usize) -> usize {
        rd::<usize>(pool)
    }
    unsafe extern "win64" fn fake_a110(node: usize, cb: usize, flag: u32) {
        wr::<usize>(node + 0x28, cb);
        wr::<u32>(node + 0x30, flag);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4243_b450, fake_b450 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_a110, fake_a110 as *const ())?;

    let mut obj_o = [0u8; 0x80];
    let mut obj_n = [0u8; 0x80];
    let mut node_o = [0usize; 7];
    let mut node_n = [0usize; 7];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x70, node_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x70, node_n.as_mut_ptr() as usize);

    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 10usize, 20usize, 30usize, 40usize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 10usize, 20usize, 30usize, 40usize) -> ());
    node_o[0] -= obj_o.as_ptr() as usize;
    node_n[0] -= obj_n.as_ptr() as usize;
    crate::diff_eq!(node_o, node_n, "142050340 node");
    Ok(1)
}

/// Original: `0x1420503b0` (178 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420503b0(rcx: usize, rdx: usize) {
    let rdi = rcx;
    let rsi = rdx;
    if rd::<usize>(rdx + 0x10) > 0x10_0000 {
        let rbx = rd::<usize>(rdx);
        exe::call_va!(0x1_4207_5a20, (rbx) -> ());
        if rbx != 0 {
            exe::call_va!(0x1_4243_b3e0, (rdi + 0xf0, rbx) -> ());
        }
    } else {
        let rbx = rcx + 0x130;
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        let rbp = rd::<usize>(rsi);
        let cnt = rd::<usize>(rbp + 0x50).wrapping_sub(1);
        wr::<usize>(rbp + 0x50, cnt);
        if cnt == 0 && rbp != rd::<usize>(rdi + 0x138) {
            exe::call_va!(0x1_4207_5a20, (rbp) -> ());
            exe::call_va!(0x1_4243_b3e0, (rdi + 0xf0, rbp) -> ());
        }
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
    exe::call_va!(0x1_4243_b3e0, (rdi + 0xb0, rsi) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_1420503b0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5a20(rbp: usize) {
        wr::<u32>(rbp + 0x4, rd::<u32>(rbp + 0x4).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, _node: usize) {
        wr::<u32>(pool, rd::<u32>(pool).wrapping_add(1));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut cases = 0u32;
    for (sz, refcnt) in [(0x20_0000usize, 1usize), (0x1000usize, 1usize), (0x1000usize, 2usize)] {
        let mut obj_o = [0u8; 0x148];
        let mut obj_n = [0u8; 0x148];
        let mut rbp_o = [0u8; 0x60];
        let mut rbp_n = [0u8; 0x60];
        let mut rsi_o = [0usize; 4];
        let mut rsi_n = [0usize; 4];
        wr::<usize>(rbp_o.as_mut_ptr() as usize + 0x50, refcnt);
        wr::<usize>(rbp_n.as_mut_ptr() as usize + 0x50, refcnt);
        rsi_o[0] = rbp_o.as_mut_ptr() as usize;
        rsi_o[2] = sz;
        rsi_n[0] = rbp_n.as_mut_ptr() as usize;
        rsi_n[2] = sz;

        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, rsi_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize, rsi_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(obj_o, obj_n, "1420503b0 obj");
        crate::diff_eq!(rbp_o, rbp_n, "1420503b0 rbp");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142050590` (247 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050590(rcx: usize, rdx: usize) {
    let r11 = rcx;
    let rdi = rdx;
    let rbx = rd::<usize>(rdx + 0x10);
    let r8 = rd::<usize>(rdx + 0x8);
    if r8 == 0 {
        let rax = rd::<usize>(r11 + 0x10);
        let r9 = rd::<usize>(rdi);
        let arg2 = rd::<usize>(rax);
        exe::call_va!(0x1_4205_0340, (r11, arg2, rbx, r9, r8) -> ());
        return;
    }
    let rax_m1 = r8.wrapping_sub(1);
    let r10_init = rax_m1 >> 4;
    if r10_init < 8 {
        let rax = rd::<usize>(r11 + 0x10);
        let r9 = rd::<usize>(rdi);
        let arg2 = rd::<usize>(rax.wrapping_add(r10_init * 8));
        exe::call_va!(0x1_4205_0340, (r11, arg2, rbx, r9, r8) -> ());
        return;
    }
    let r10: u32 = if rax_m1 == 0 {
        0
    } else {
        63 - (rax_m1 as u64).leading_zeros()
    };
    let pow2 = 1usize.wrapping_shl(r10);
    let r9_div = pow2 >> 2;
    let q = r8.wrapping_sub(pow2).wrapping_sub(1) / r9_div;
    let bucket = (r10 as usize).wrapping_sub(7).wrapping_mul(4).wrapping_add(q);
    if bucket < 0x27 {
        let rax = rd::<usize>(r11 + 0x20);
        let r9 = rd::<usize>(rdi);
        let arg2 = rd::<usize>(rax.wrapping_add(bucket * 8));
        exe::call_va!(0x1_4205_0340, (r11, arg2, rbx, r9, r8) -> ());
        return;
    }
    let pool = r11 + 0x30;
    if rbx != 0 {
        if rd::<usize>(rbx + 0xd8) == 0 {
            exe::call_va!(0x1_4207_5a20, (rbx + 0x10) -> ());
            if rd::<usize>(rbx + 0xa0) != 0 {
                exe::call_va!(0x1_4207_5a20, (rbx + 0x60) -> ());
            }
        }
        exe::call_va!(0x1_401d_e830, (rbx + 0xc0) -> ());
        exe::call_va!(0x1_4243_b3e0, (pool, rbx) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_142050590(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_0340(rcx: usize, a2: usize, _a3: usize, a4: usize, a5: usize) {
        wr::<usize>(rcx + 0x40, a2 ^ a4 ^ a5);
    }
    unsafe extern "win64" fn fake_5a20(p: usize) {
        wr::<u32>(p, 0x5a20);
    }
    unsafe extern "win64" fn fake_e830(p: usize) {
        wr::<u32>(p, 0xe830);
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, _node: usize) {
        wr::<u32>(pool, 0xb3e0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4205_0340, fake_0340 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_e830, fake_e830 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut tbl10 = [0usize; 8];
    let mut tbl20 = [0usize; 40];
    for (i, slot) in tbl10.iter_mut().enumerate() {
        *slot = (i + 1) * 0x100;
    }
    for (i, slot) in tbl20.iter_mut().enumerate() {
        *slot = (i + 1) * 0x1000;
    }

    let mut cases = 0u32;
    for r8 in [0usize, 32usize, 512usize, 1usize << 20] {
        let mut obj_o = [0u8; 0x50];
        let mut obj_n = [0u8; 0x50];
        let mut rbx_o = [0u8; 0xe8];
        let mut rbx_n = [0u8; 0xe8];
        wr::<usize>(rbx_o.as_mut_ptr() as usize + 0xa0, 1);
        wr::<usize>(rbx_n.as_mut_ptr() as usize + 0xa0, 1);
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x10, tbl10.as_ptr() as usize);
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x20, tbl20.as_ptr() as usize);
        wr::<usize>(obj_n.as_mut_ptr() as usize + 0x10, tbl10.as_ptr() as usize);
        wr::<usize>(obj_n.as_mut_ptr() as usize + 0x20, tbl20.as_ptr() as usize);
        let rdx_o = [0x77usize, r8, rbx_o.as_mut_ptr() as usize];
        let rdx_n = [0x77usize, r8, rbx_n.as_mut_ptr() as usize];

        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, rdx_o.as_ptr() as usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize, rdx_n.as_ptr() as usize) -> ());
        crate::diff_eq!(obj_o, obj_n, "142050590 obj");
        crate::diff_eq!(rbx_o, rbx_n, "142050590 rbx");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x1420507c0` (171 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420507c0(rcx: usize) {
    let rbx = rcx;
    let rdi = rd::<usize>(rcx + 0x138);
    if rdi != 0 {
        exe::call_va!(0x1_4207_5a20, (rdi) -> ());
        exe::call_va!(0x1_4243_b3e0, (rbx + 0xf0, rdi) -> ());
    }
    let mut edx = rd::<i32>(rbx + 0x70);
    let p_a8 = rd::<usize>(rbx + 0xa8);
    if p_a8 != 0 {
        let add_v = exe::call_va!(0x1_4243_b2e0, (p_a8) -> i32);
        edx = edx.wrapping_add(add_v);
    }
    if edx > 0 {
        exe::call_va!(0x1_4209_9c20, (1u32, 1u32) -> ());
    }
    exe::call_va!(0x1_4243_b540, (rbx + 0xf0) -> ());
    exe::call_va!(0x1_4243_b540, (rbx + 0xb0) -> ());
    exe::call_va!(0x1_4243_b540, (rbx + 0x70) -> ());
    exe::call_va!(0x1_4243_b540, (rbx + 0x30) -> ());
    exe::call_va!(0x1_411d_3de0, (rbx + 0x20) -> ());
    exe::call_va!(0x1_411d_3de0, (rbx + 0x10) -> ());
    exe::call_va!(0x1_411d_3de0, (rbx) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_1420507c0(orig: usize, new: usize) -> Result<u32, String> {
    static mut CALL_9C20: u32 = 0;
    unsafe extern "win64" fn fake_5a20(rdi: usize) {
        wr::<u32>(rdi, 0x5a20);
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, _rdi: usize) {
        wr::<u32>(pool + 0x4, 0xb3e0);
    }
    unsafe extern "win64" fn fake_b2e0(_p: usize) -> i32 {
        3
    }
    unsafe extern "win64" fn fake_9c20(a: u32, b: u32) {
        CALL_9C20 = (a << 8) | b;
    }
    unsafe extern "win64" fn fake_b540(p: usize) {
        wr::<u32>(p + 0x8, 0xb540);
    }
    unsafe extern "win64" fn fake_3de0(p: usize) {
        wr::<u32>(p + 0x4, 0x3de0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_b2e0, fake_b2e0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_9c20, fake_9c20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4243_b540, fake_b540 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411d_3de0, fake_3de0 as *const ())?;

    let mut sub_o = [0u8; 8];
    let mut sub_n = [0u8; 8];
    let add_src = 3i32;
    let mut obj_o = [0u8; 0x140];
    let mut obj_n = [0u8; 0x140];
    for (obj, sub) in [(&mut obj_o, &mut sub_o), (&mut obj_n, &mut sub_n)] {
        let p = obj.as_mut_ptr() as usize;
        wr::<i32>(p + 0x70, 2);
        wr::<usize>(p + 0xa8, (&add_src as *const i32) as usize);
        wr::<usize>(p + 0x138, sub.as_mut_ptr() as usize);
    }

    CALL_9C20 = 0;
    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
    let c_o = CALL_9C20;
    CALL_9C20 = 0;
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
    let c_n = CALL_9C20;
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x138, 0);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x138, 0);
    crate::diff_eq!(c_o, c_n, "1420507c0 9c20");
    crate::diff_eq!(obj_o, obj_n, "1420507c0 obj");
    crate::diff_eq!(sub_o, sub_n, "1420507c0 sub");
    Ok(1)
}

/// Original: `0x142050890` (216 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050890(rcx: usize, rdx: usize, r8: usize) -> usize {
    let rbx = rdx;
    let mut arg_r8: u64 = r8 as u64;
    let mut arg_r9: u64;
    let scaled = (r8 as u64).wrapping_mul(8);
    if scaled > 1 {
        let dec = scaled.wrapping_sub(1);
        let bsr = (63 - dec.leading_zeros()).wrapping_add(1).max(0xc);
        let idx = ((bsr - 0xb) >> 1) as usize;
        let shift = (idx * 2 + 0xc) as u32;
        arg_r9 = 1u64.wrapping_shl(shift);
        let rax_ptr = rd::<usize>(rcx);
        let rdx_arg = rax_ptr + 0x30;
        let mut slot_val: usize = if idx >= 2 {
            0
        } else {
            let table = rd::<usize>(rax_ptr);
            rd::<usize>(table + idx * 8)
        };
        let slot_addr = (&mut slot_val as *mut usize) as usize;
        let r8_addr = (&mut arg_r8 as *mut u64) as usize;
        let r9_addr = (&mut arg_r9 as *mut u64) as usize;
        exe::call_va!(
            0x1_411d_3d00,
            (rbx, rdx_arg, r8_addr, r9_addr, rdx_arg, slot_addr) -> ()
        );
        return rbx;
    }
    let rax_ptr = rd::<usize>(rcx);
    arg_r9 = 0x1000;
    let rdx_arg = rax_ptr + 0x30;
    let table = rd::<usize>(rax_ptr);
    let mut slot_val: usize = rd::<usize>(table);
    let slot_addr = (&mut slot_val as *mut usize) as usize;
    let r8_addr = (&mut arg_r8 as *mut u64) as usize;
    let r9_addr = (&mut arg_r9 as *mut u64) as usize;
    exe::call_va!(
        0x1_411d_3d00,
        (rbx, rdx_arg, r8_addr, r9_addr, rdx_arg, slot_addr) -> ()
    );
    rbx
}

pub unsafe fn diff_d3d_tls_ctx_142050890(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_3d00(
        out: usize,
        a2: usize,
        p_r8: usize,
        p_r9: usize,
        a5: usize,
        p_slot: usize,
    ) {
        wr::<usize>(out, a2);
        wr::<u64>(out + 0x8, rd::<u64>(p_r8));
        wr::<u64>(out + 0x10, rd::<u64>(p_r9));
        wr::<usize>(out + 0x18, a5);
        wr::<usize>(out + 0x20, rd::<usize>(p_slot));
    }
    let _m = crate::difftest::Mock::new(0x1_411d_3d00, fake_3d00 as *const ())?;

    let table: [usize; 2] = [0xaaaa_1111, 0xbbbb_2222];
    let mut inner = [0u8; 0x40];
    wr::<usize>(inner.as_mut_ptr() as usize, table.as_ptr() as usize);
    let outer: usize = inner.as_ptr() as usize;
    let rcx = (&outer as *const usize) as usize;

    let mut cases = 0u32;
    for r8 in [0usize, 100usize, 512usize, 4096usize] {
        let mut out_o = [0u64; 5];
        let mut out_n = [0u64; 5];
        let ro = exe::call_va!(orig, (rcx, out_o.as_mut_ptr() as usize, r8) -> usize);
        let rn = exe::call_va!(new, (rcx, out_n.as_mut_ptr() as usize, r8) -> usize);
        crate::diff_eq!(ro, out_o.as_ptr() as usize, "142050890 ret orig");
        crate::diff_eq!(rn, out_n.as_ptr() as usize, "142050890 ret new");
        crate::diff_eq!(out_o, out_n, "142050890 out");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142050df0` (46 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050df0(rcx: u64) -> u64 {
    let scaled = rcx.wrapping_shl(3);
    let mut rax: u64 = 0;
    if scaled > 1 {
        let dec = scaled.wrapping_sub(1);
        rax = (63 - dec.leading_zeros() + 1) as u64;
    }
    if rax < 0xc {
        rax = 0xc;
    }
    (rax - 0xb) >> 1
}

pub unsafe fn diff_d3d_tls_ctx_142050df0(orig: usize, new: usize) -> Result<u32, String> {
    for &v in &[0u64, 1, 2, 15, 16, 128, 511, 512, 513, 4096, 65536] {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050df0");
    }
    Ok(11)
}

/// Original: `0x142050e30` (19 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050e30(rcx: u64) -> u64 {
    let shift = rcx.wrapping_mul(2).wrapping_add(0xc);
    1u64.wrapping_shl(shift as u32)
}

pub unsafe fn diff_d3d_tls_ctx_142050e30(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u64..8 {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050e30");
    }
    Ok(8)
}

/// Original: `0x142050e50` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050e50(rcx: u64) -> u64 {
    let shift = (rcx >> 2).wrapping_add(7);
    let rem = (rcx & 3).wrapping_add(1);
    let rdx = 1u64.wrapping_shl(shift as u32);
    let rax = (rdx >> 2).wrapping_mul(rem);
    rax.wrapping_add(rdx)
}

pub unsafe fn diff_d3d_tls_ctx_142050e50(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u64..16 {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050e50");
    }
    Ok(16)
}

/// Original: `0x142050e80` (54 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050e80(rcx: u64) -> u64 {
    let rax_m1 = rcx.wrapping_sub(1);
    let r8: u32 = if rax_m1 == 0 {
        0
    } else {
        63 - rax_m1.leading_zeros()
    };
    let pow2 = 1u64.wrapping_shl(r8);
    let r9 = rcx.wrapping_sub(pow2);
    let div_by = pow2 >> 2;
    let q = r9.wrapping_sub(1) / div_by;
    q.wrapping_add((r8 as u64).wrapping_mul(4))
        .wrapping_sub(0x1c)
}

pub unsafe fn diff_d3d_tls_ctx_142050e80(orig: usize, new: usize) -> Result<u32, String> {
    for &v in &[129u64, 160, 255, 256, 257, 512, 1024, 4096, 65535] {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050e80");
    }
    Ok(9)
}

/// Original: `0x142050ec0` (9 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050ec0(rcx: u64) -> u64 {
    rcx.wrapping_add(1).wrapping_shl(4)
}

pub unsafe fn diff_d3d_tls_ctx_142050ec0(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u64..8 {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050ec0");
    }
    Ok(8)
}

/// Original: `0x142050ed0` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142050ed0(rcx: u64) -> u64 {
    if rcx == 0 {
        0
    } else {
        rcx.wrapping_sub(1) >> 4
    }
}

pub unsafe fn diff_d3d_tls_ctx_142050ed0(orig: usize, new: usize) -> Result<u32, String> {
    for &v in &[0u64, 1, 15, 16, 17, 32, 33, 128] {
        let r_orig = exe::call_va!(orig, (v) -> u64);
        let r_new = exe::call_va!(new, (v) -> u64);
        crate::diff_eq!(r_orig, r_new, "142050ed0");
    }
    Ok(8)
}

/// Original: `0x142051040` (121 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142051040(rcx: usize, rdx: usize, r8: usize) {
    let rsi = rcx;
    let rdi = rdx;
    let rbx = r8;
    exe::call_va!(0x1_4205_0ef0, (rcx) -> ());
    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize);
    let mut tmp = [0u64; 4];
    let tmp_addr = tmp.as_mut_ptr() as usize;
    let rax = exe::call_va!(0x1_4205_0690, (g, tmp_addr, rdi) -> usize);
    wr::<u64>(rsi + 0x40, rd::<u64>(rax));
    wr::<u64>(rsi + 0x48, rd::<u64>(rax + 0x8));
    wr::<u64>(rsi + 0x50, rd::<u64>(rax + 0x10));
    let p50 = rd::<usize>(rsi + 0x50);
    let r8_arg = rd::<usize>(p50 + 0x8).wrapping_add(rd::<usize>(rsi + 0x40));
    exe::call_va!(0x1_4207_58f0, (rsi, p50 + 0x10, r8_arg, rdi, rbx, 0usize) -> ());
}

unsafe fn run_diff_51040_family(
    orig: usize,
    new: usize,
    is_two_arg: bool,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_0ef0(rcx: usize) {
        wr::<u32>(rcx + 0x38, 0x0ef0);
    }
    unsafe extern "win64" fn fake_0690(g: usize, tmp: usize, size: usize) -> usize {
        wr::<u64>(tmp, (size as u64) + 0x100);
        wr::<u64>(tmp + 0x8, 0x2222_3333);
        wr::<usize>(tmp + 0x10, g);
        tmp
    }
    unsafe extern "win64" fn fake_58f0(
        this: usize,
        src: usize,
        offset: usize,
        size: usize,
        stride: usize,
        flag: u8,
    ) {
        wr::<usize>(this, src ^ offset);
        wr::<usize>(this + 0x8, size);
        wr::<usize>(this + 0x10, stride);
        wr::<u8>(this + 0x18, flag);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4205_0ef0, fake_0ef0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_0690, fake_0690 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_58f0, fake_58f0 as *const ())?;

    let g_addr = exe::ptr::<u8>(0x1_4397_fb88) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_addr, 8);
    let mut fake_p50 = [0u64; 4];
    fake_p50[1] = 0x4000;
    wr::<usize>(g_addr as usize, fake_p50.as_mut_ptr() as usize);

    let mut obj_o = [0u8; 0x60];
    let mut obj_n = [0u8; 0x60];
    if is_two_arg {
        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 256usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 256usize) -> ());
    } else {
        exe::call_va!(orig, (obj_o.as_mut_ptr() as usize, 16usize, 32usize) -> ());
        exe::call_va!(new, (obj_n.as_mut_ptr() as usize, 16usize, 32usize) -> ());
    }
    snap.restore();
    crate::diff_eq!(obj_o, obj_n, "51040 family obj");
    Ok(1)
}

pub unsafe fn diff_d3d_tls_ctx_142051040(orig: usize, new: usize) -> Result<u32, String> {
    run_diff_51040_family(orig, new, false)
}

/// Original: `0x1420510c0` (125 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420510c0(rcx: usize, rdx: usize, r8: usize) {
    let rsi = rcx;
    let rbx = r8;
    let rdi = rdx.wrapping_mul(r8);
    exe::call_va!(0x1_4205_0ef0, (rcx) -> ());
    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize);
    let mut tmp = [0u64; 4];
    let tmp_addr = tmp.as_mut_ptr() as usize;
    let rax = exe::call_va!(0x1_4205_0690, (g, tmp_addr, rdi) -> usize);
    wr::<u64>(rsi + 0x40, rd::<u64>(rax));
    wr::<u64>(rsi + 0x48, rd::<u64>(rax + 0x8));
    wr::<u64>(rsi + 0x50, rd::<u64>(rax + 0x10));
    let p50 = rd::<usize>(rsi + 0x50);
    let r8_arg = rd::<usize>(p50 + 0x8).wrapping_add(rd::<usize>(rsi + 0x40));
    exe::call_va!(0x1_4207_58f0, (rsi, p50 + 0x10, r8_arg, rdi, rbx, 0usize) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_1420510c0(orig: usize, new: usize) -> Result<u32, String> {
    run_diff_51040_family(orig, new, false)
}

/// Original: `0x142051150` (112 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142051150(rcx: usize, rdx: usize) {
    let rdi = rcx;
    let rbx = rdx;
    exe::call_va!(0x1_4205_0ef0, (rcx) -> ());
    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize);
    let mut tmp = [0u64; 4];
    let tmp_addr = tmp.as_mut_ptr() as usize;
    let rax = exe::call_va!(0x1_4205_0690, (g, tmp_addr, rbx) -> usize);
    wr::<u64>(rdi + 0x40, rd::<u64>(rax));
    wr::<u64>(rdi + 0x48, rd::<u64>(rax + 0x8));
    wr::<u64>(rdi + 0x50, rd::<u64>(rax + 0x10));
    let p50 = rd::<usize>(rdi + 0x50);
    let r8_arg = rd::<usize>(p50 + 0x8).wrapping_add(rd::<usize>(rdi + 0x40));
    exe::call_va!(0x1_4207_58f0, (rdi, p50 + 0x10, r8_arg, rbx, 1usize, 0usize) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_142051150(orig: usize, new: usize) -> Result<u32, String> {
    run_diff_51040_family(orig, new, true)
}

/// Original: `0x1420511d0` (168 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420511d0(rcx: usize, rdx: usize) -> usize {
    let rdi = rcx;
    let rbx = rdx;
    exe::call_va!(0x1_4205_0ef0, (rcx) -> ());
    for off in (0..0x60).step_by(8) {
        let val = rd::<u64>(rbx + off);
        wr::<u64>(rbx + off, 0);
        wr::<u64>(rdi + off, val);
    }
    rdi
}

pub unsafe fn diff_d3d_tls_ctx_1420511d0(orig: usize, new: usize) -> Result<u32, String> {
    static mut RELEASED_CNT: u32 = 0;
    unsafe extern "win64" fn fake_0ef0(_rcx: usize) {
        RELEASED_CNT += 1;
    }
    let _m = crate::difftest::Mock::new(0x1_4205_0ef0, fake_0ef0 as *const ())?;

    let mut dst_o = [0u64; 12];
    let mut src_o = [0u64; 12];
    let mut dst_n = [0u64; 12];
    let mut src_n = [0u64; 12];
    for i in 0..12 {
        let v = (i as u64 + 1) * 0x1234_5678_9abc_def0;
        src_o[i] = v;
        src_n[i] = v;
    }
    RELEASED_CNT = 0;
    let ro = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> usize);
    let cnt_o = RELEASED_CNT;
    RELEASED_CNT = 0;
    let rn = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> usize);
    let cnt_n = RELEASED_CNT;
    crate::diff_eq!(ro, dst_o.as_ptr() as usize, "1420511d0 ret orig");
    crate::diff_eq!(rn, dst_n.as_ptr() as usize, "1420511d0 ret new");
    crate::diff_eq!(cnt_o, cnt_n, "1420511d0 0ef0 count");
    crate::diff_eq!(dst_o, dst_n, "1420511d0 dst");
    crate::diff_eq!(src_o, src_n, "1420511d0 src zeroed");
    Ok(1)
}

/// Original: `0x1420512a0` (137 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420512a0(rcx: usize, rdx: usize) -> usize {
    for off in (0..0x60).step_by(8) {
        let val = rd::<u64>(rdx + off);
        wr::<u64>(rdx + off, 0);
        wr::<u64>(rcx + off, val);
    }
    rcx
}

pub unsafe fn diff_d3d_tls_ctx_1420512a0(orig: usize, new: usize) -> Result<u32, String> {
    let mut dst_o = [0u64; 12];
    let mut src_o = [0u64; 12];
    let mut dst_n = [0u64; 12];
    let mut src_n = [0u64; 12];
    for i in 0..12 {
        let v = (i as u64 + 1) * 0x1111_2222_3333_4444;
        src_o[i] = v;
        src_n[i] = v;
    }
    let ro = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(ro, dst_o.as_ptr() as usize, "1420512a0 ret orig");
    crate::diff_eq!(rn, dst_n.as_ptr() as usize, "1420512a0 ret new");
    crate::diff_eq!(dst_o, dst_n, "1420512a0 dst");
    crate::diff_eq!(src_o, src_n, "1420512a0 src zeroed");
    Ok(1)
}

/// Original: `0x142051620` (180 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142051620(rcx: usize) {
    let r14 = rcx;
    let rbp = exe::call_va!(0x1_4205_1a20, (rd::<usize>(rcx)) -> usize);
    let lock = rd::<usize>(r14) + 0xd0;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(lock + 0x8, tid);
    }
    wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));

    let rdi = rd::<usize>(r14);
    let count = rd::<u32>(rdi + 0x30);
    if count == rd::<u32>(rdi + 0x34) {
        exe::call_va!(0x1_4034_97a0, (rdi + 0x20, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(rdi + 0x2c) as usize;
    let buf = rd::<usize>(rdi + 0x20);
    wr::<usize>(buf + tail * 8, rbp);
    let next_tail = rd::<u32>(rdi + 0x2c).wrapping_add(1);
    if next_tail == rd::<u32>(rdi + 0x34) {
        wr::<u32>(rdi + 0x2c, 0);
    } else {
        wr::<u32>(rdi + 0x2c, next_tail);
    }
    wr::<u32>(rdi + 0x30, rd::<u32>(rdi + 0x30).wrapping_add(1));

    let depth = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, depth);
    if depth == 0 {
        wr::<u32>(lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_142051620(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1a20(_rdi: usize) -> usize {
        0x7777_8888
    }
    unsafe extern "win64" fn fake_97a0(vec_ptr: usize, new_cap: u32) {
        wr::<u32>(vec_ptr + 0x14, new_cap);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4205_1a20, fake_1a20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4034_97a0, fake_97a0 as *const ())?;

    let mut cases = 0u32;
    for (tail, count, cap) in [(1u32, 1u32, 4u32), (3u32, 2u32, 4u32), (2u32, 4u32, 4u32)] {
        let mut ring_o = [0usize; 8];
        let mut ring_n = [0usize; 8];
        let mut rdi_o = [0u8; 0xf0];
        let mut rdi_n = [0u8; 0xf0];
        for (rdi, ring) in [(&mut rdi_o, &mut ring_o), (&mut rdi_n, &mut ring_n)] {
            let p = rdi.as_mut_ptr() as usize;
            wr::<usize>(p + 0x20, ring.as_mut_ptr() as usize);
            wr::<u32>(p + 0x2c, tail);
            wr::<u32>(p + 0x30, count);
            wr::<u32>(p + 0x34, cap);
            wr::<u32>(p + 0xd8, 0xffff_ffff);
        }
        let r14_o: usize = rdi_o.as_mut_ptr() as usize;
        let r14_n: usize = rdi_n.as_mut_ptr() as usize;
        exe::call_va!(orig, ((&r14_o as *const usize) as usize) -> ());
        exe::call_va!(new, ((&r14_n as *const usize) as usize) -> ());
        wr::<usize>(rdi_o.as_mut_ptr() as usize + 0x20, 0);
        wr::<usize>(rdi_n.as_mut_ptr() as usize + 0x20, 0);
        crate::diff_eq!(rdi_o, rdi_n, "142051620 rdi");
        crate::diff_eq!(ring_o, ring_n, "142051620 ring");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142051d00` (158 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142051d00(rcx: usize, rdx: usize) {
    let rdi = rcx;
    let r14 = rdx;
    let lock = rcx + 0xd0;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(lock + 0x8, tid);
    }
    wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));

    let owner = rd::<usize>(r14);
    let refs = rd::<u32>(owner + 0x14).wrapping_sub(1);
    wr::<u32>(owner + 0x14, refs);
    if refs == 0 {
        exe::call_va!(0x1_4205_1450, (rdi, owner) -> ());
    }
    let sz = rd::<u32>(r14 + 0xc) as u64;
    wr::<u64>(rdi + 0xf8, rd::<u64>(rdi + 0xf8).wrapping_sub(sz));
    exe::call_va!(0x1_4243_b3e0, (rdi + 0x78, r14) -> ());

    let depth = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, depth);
    if depth == 0 {
        wr::<u32>(lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_142051d00(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1450(_rdi: usize, owner: usize) {
        wr::<u32>(owner + 0x18, 0x1450);
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, _node: usize) {
        wr::<u32>(pool, rd::<u32>(pool).wrapping_add(1));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4205_1450, fake_1450 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut cases = 0u32;
    for init_refs in [1u32, 2u32] {
        let mut rdi_o = [0u8; 0x108];
        let mut rdi_n = [0u8; 0x108];
        let mut owner_o = [0u8; 0x20];
        let mut owner_n = [0u8; 0x20];
        let mut node_o = [0u8; 0x18];
        let mut node_n = [0u8; 0x18];
        for (rdi, owner, node) in [
            (&mut rdi_o, &mut owner_o, &mut node_o),
            (&mut rdi_n, &mut owner_n, &mut node_n),
        ] {
            wr::<u32>(rdi.as_mut_ptr() as usize + 0xd8, 0xffff_ffff);
            wr::<u64>(rdi.as_mut_ptr() as usize + 0xf8, 0x5000);
            wr::<u32>(owner.as_mut_ptr() as usize + 0x14, init_refs);
            wr::<usize>(node.as_mut_ptr() as usize, owner.as_mut_ptr() as usize);
            wr::<u32>(node.as_mut_ptr() as usize + 0xc, 0x1200);
        }
        exe::call_va!(orig, (rdi_o.as_mut_ptr() as usize, node_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (rdi_n.as_mut_ptr() as usize, node_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(rdi_o, rdi_n, "142051d00 rdi");
        crate::diff_eq!(owner_o, owner_n, "142051d00 owner");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142052700` (180 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052700(rcx: usize, rdx: usize) {
    let rdi = rcx;
    let rsi = rdx;
    let lock = rcx + 0xc0;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(lock + 0x8, tid);
    }
    wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));

    let owner = rd::<usize>(rsi);
    let mask = !rd::<u64>(rsi + 0x18);
    let new_mask = rd::<u64>(owner + 0x18) & mask;
    wr::<u64>(owner + 0x18, new_mask);
    let amt = rd::<u64>(rsi + 0x10);
    wr::<u64>(owner + 0x10, rd::<u64>(owner + 0x10).wrapping_add(amt));
    if new_mask == 0 {
        exe::call_va!(0x1_4205_2100, (rdi, owner) -> ());
    }
    let amt2 = rd::<u64>(rsi + 0x10);
    wr::<u64>(rdi + 0xd8, rd::<u64>(rdi + 0xd8).wrapping_sub(amt2));
    exe::call_va!(0x1_4243_b3e0, (rdi + 0x68, rsi) -> ());

    let depth = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, depth);
    if depth == 0 {
        wr::<u32>(lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_142052700(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_2100(_rdi: usize, owner: usize) {
        wr::<u32>(owner + 0x20, 0x2100);
    }
    unsafe extern "win64" fn fake_b3e0(pool: usize, _node: usize) {
        wr::<u32>(pool, rd::<u32>(pool).wrapping_add(1));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4205_2100, fake_2100 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b3e0, fake_b3e0 as *const ())?;

    let mut cases = 0u32;
    for init_mask in [0x0f_u64, 0xff_u64] {
        let mut rdi_o = [0u8; 0xe8];
        let mut rdi_n = [0u8; 0xe8];
        let mut owner_o = [0u8; 0x28];
        let mut owner_n = [0u8; 0x28];
        let mut rsi_o = [0u8; 0x20];
        let mut rsi_n = [0u8; 0x20];
        for (rdi, owner, rsi) in [
            (&mut rdi_o, &mut owner_o, &mut rsi_o),
            (&mut rdi_n, &mut owner_n, &mut rsi_n),
        ] {
            wr::<u32>(rdi.as_mut_ptr() as usize + 0xc8, 0xffff_ffff);
            wr::<u64>(rdi.as_mut_ptr() as usize + 0xd8, 0x8000);
            wr::<u64>(owner.as_mut_ptr() as usize + 0x10, 0x100);
            wr::<u64>(owner.as_mut_ptr() as usize + 0x18, init_mask);
            wr::<usize>(rsi.as_mut_ptr() as usize, owner.as_mut_ptr() as usize);
            wr::<u64>(rsi.as_mut_ptr() as usize + 0x10, 0x400);
            wr::<u64>(rsi.as_mut_ptr() as usize + 0x18, 0x0f);
        }
        exe::call_va!(orig, (rdi_o.as_mut_ptr() as usize, rsi_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (rdi_n.as_mut_ptr() as usize, rsi_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(rdi_o, rdi_n, "142052700 rdi");
        crate::diff_eq!(owner_o, owner_n, "142052700 owner");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142052a70` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052a70(ecx: u32) -> u32 {
    match ecx {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => 0xffff_ffff,
    }
}

pub unsafe fn diff_d3d_tls_ctx_142052a70(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u32..6 {
        let r_orig = exe::call_va!(orig, (v) -> u32);
        let r_new = exe::call_va!(new, (v) -> u32);
        crate::diff_eq!(r_orig, r_new, "142052a70");
    }
    Ok(6)
}

/// Original: `0x142052aa0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052aa0(ecx: u32) -> usize {
    if ecx < 3 {
        let base = exe::ptr::<usize>(0x1_4297_a220) as usize;
        rd::<usize>(base + (ecx as usize) * 8)
    } else {
        exe::ptr::<u8>(0x1_4310_71d0) as usize
    }
}

pub unsafe fn diff_d3d_tls_ctx_142052aa0(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u32..6 {
        let r_orig = exe::call_va!(orig, (v) -> usize);
        let r_new = exe::call_va!(new, (v) -> usize);
        crate::diff_eq!(r_orig, r_new, "142052aa0");
    }
    Ok(6)
}

/// Original: `0x142052ae0` (120 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052ae0(rcx: usize) {
    let rsi = rcx;
    let head = rd::<usize>(rcx + 0x28);
    let mut rbx = rd::<usize>(head + 0x8);
    while rd::<u8>(rbx + 0x19) == 0 {
        let r8 = rd::<usize>(rbx + 0x10);
        exe::call_va!(0x1_4024_6bb0, (rsi + 0x28, rsi + 0x28, r8) -> ());
        let node = rbx;
        rbx = rd::<usize>(rbx);
        exe::call_va!(0x1_401e_66f0, (rsi + 0x28, node) -> ());
    }
    exe::call_va!(0x1_401e_66f0, (rsi + 0x28, rd::<usize>(rsi + 0x28)) -> ());
    let p18 = rd::<usize>(rsi + 0x18);
    let r8 = rd::<usize>(p18 + 0x8);
    exe::call_va!(0x1_411d_40c0, (rsi + 0x18, rsi + 0x18, r8) -> ());
    exe::call_va!(0x1_401e_66f0, (rsi + 0x18, rd::<usize>(rsi + 0x18)) -> ());
}

pub unsafe fn diff_d3d_tls_ctx_142052ae0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_6bb0(_rcx: usize, _rdx: usize, r8: usize) {
        wr::<u32>(r8, 0x6bb0);
    }
    unsafe extern "win64" fn fake_66f0(_rcx: usize, node: usize) {
        wr::<u32>(node + 0x20, 0x66f0);
    }
    unsafe extern "win64" fn fake_40c0(_rcx: usize, _rdx: usize, r8: usize) {
        wr::<u32>(r8, 0x40c0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4024_6bb0, fake_6bb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_66f0, fake_66f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_40c0, fake_40c0 as *const ())?;

    let mut head_o = [0u8; 0x28];
    let mut head_n = [0u8; 0x28];
    let mut node1_o = [0u8; 0x28];
    let mut node1_n = [0u8; 0x28];
    let mut sub1_o = [0u8; 8];
    let mut sub1_n = [0u8; 8];
    let mut p18_o = [0u8; 0x28];
    let mut p18_n = [0u8; 0x28];
    let mut sub2_o = [0u8; 8];
    let mut sub2_n = [0u8; 8];
    let mut obj_o = [0u8; 0x38];
    let mut obj_n = [0u8; 0x38];

    for (obj, head, n1, s1, p18, s2) in [
        (&mut obj_o, &mut head_o, &mut node1_o, &mut sub1_o, &mut p18_o, &mut sub2_o),
        (&mut obj_n, &mut head_n, &mut node1_n, &mut sub1_n, &mut p18_n, &mut sub2_n),
    ] {
        wr::<usize>(head.as_mut_ptr() as usize + 0x8, n1.as_mut_ptr() as usize);
        wr::<u8>(head.as_mut_ptr() as usize + 0x19, 1);
        wr::<usize>(n1.as_mut_ptr() as usize, head.as_mut_ptr() as usize);
        wr::<usize>(n1.as_mut_ptr() as usize + 0x10, s1.as_mut_ptr() as usize);
        wr::<u8>(n1.as_mut_ptr() as usize + 0x19, 0);
        wr::<usize>(p18.as_mut_ptr() as usize + 0x8, s2.as_mut_ptr() as usize);
        wr::<usize>(obj.as_mut_ptr() as usize + 0x18, p18.as_mut_ptr() as usize);
        wr::<usize>(obj.as_mut_ptr() as usize + 0x28, head.as_mut_ptr() as usize);
    }

    exe::call_va!(orig, (obj_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (obj_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(sub1_o, sub1_n, "142052ae0 sub1");
    crate::diff_eq!(sub2_o, sub2_n, "142052ae0 sub2");
    crate::diff_eq!(head_o[0x20..], head_n[0x20..], "142052ae0 head freed");
    crate::diff_eq!(node1_o[0x20..], node1_n[0x20..], "142052ae0 node1 freed");
    crate::diff_eq!(p18_o[0x20..], p18_n[0x20..], "142052ae0 p18 freed");
    Ok(1)
}

/// Original: `0x142052b60` (149 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052b60(ecx: u32) {
    let rdi = ecx as usize;
    let ref_addr = (exe::ptr::<u32>(0x1_43c4_8a48) as usize) + rdi * 4;
    let new_ref = rd::<u32>(ref_addr).wrapping_sub(1);
    wr::<u32>(ref_addr, new_ref);
    if new_ref == 0 {
        let obj_slot = (exe::ptr::<usize>(0x1_43a6_3130) as usize) + rdi * 8;
        let rbx = rd::<usize>(obj_slot);
        wr::<usize>(obj_slot, 0);
        if rbx != 0 {
            let vtbl = rd::<usize>(rbx);
            let dtor = rd::<usize>(vtbl);
            exe::call_va!(dtor, (rbx, 0u32) -> ());
            exe::call_va!(0x1_4244_2de0, (rbx) -> ());
        }
        let str_obj = (exe::ptr::<u8>(0x1_43a6_3150) as usize) + rdi * 40;
        let lit = exe::ptr::<u8>(0x1_433d_c2d8) as usize;
        let mut view: [usize; 2] = [lit, 0xf];
        let view_addr = view.as_mut_ptr() as usize;
        exe::call_va!(0x1_4240_9b50, (str_obj, view_addr, 0u32, 0xfu32) -> ());
    }
}

pub unsafe fn diff_d3d_tls_ctx_142052b60(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dtor(this: usize, flag: u32) {
        wr::<u32>(this + 0x8, 0xd700 | flag);
    }
    unsafe extern "win64" fn fake_2de0(this: usize) {
        wr::<u32>(this + 0xc, 0x2de0);
    }
    unsafe extern "win64" fn fake_9b50(str_obj: usize, view: usize, r8d: u32, r9d: u32) {
        wr::<usize>(str_obj, rd::<usize>(view));
        wr::<usize>(str_obj + 0x8, rd::<usize>(view + 0x8));
        wr::<u32>(str_obj + 0x10, r8d);
        wr::<u32>(str_obj + 0x14, r9d);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4244_2de0, fake_2de0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_9b50, fake_9b50 as *const ())?;

    let g_ref = exe::ptr::<u8>(0x1_43c4_8a48) as *mut u8;
    let g_obj = exe::ptr::<u8>(0x1_43a6_3130) as *mut u8;
    let g_str = exe::ptr::<u8>(0x1_43a6_3150) as *mut u8;
    let s_ref = crate::difftest::Snapshot::take(g_ref, 8);
    let s_obj = crate::difftest::Snapshot::take(g_obj, 16);
    let s_str = crate::difftest::Snapshot::take(g_str, 80);

    let vtbl: [usize; 1] = [fake_dtor as *const () as usize];
    let mut rbx_o = [0u8; 16];
    let mut rbx_n = [0u8; 16];
    wr::<usize>(rbx_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(rbx_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    wr::<u32>(g_ref as usize + 4, 1);
    wr::<usize>(g_obj as usize + 8, rbx_o.as_mut_ptr() as usize);
    std::ptr::write_bytes((g_str as usize + 40) as *mut u8, 0, 40);
    exe::call_va!(orig, (1u32) -> ());
    let ref_o = rd::<u32>(g_ref as usize + 4);
    let slot_o = rd::<usize>(g_obj as usize + 8);
    let str_o = std::slice::from_raw_parts((g_str as usize + 40) as *const u8, 24).to_vec();

    wr::<u32>(g_ref as usize + 4, 1);
    wr::<usize>(g_obj as usize + 8, rbx_n.as_mut_ptr() as usize);
    std::ptr::write_bytes((g_str as usize + 40) as *mut u8, 0, 40);
    exe::call_va!(new, (1u32) -> ());
    let ref_n = rd::<u32>(g_ref as usize + 4);
    let slot_n = rd::<usize>(g_obj as usize + 8);
    let str_n = std::slice::from_raw_parts((g_str as usize + 40) as *const u8, 24).to_vec();

    s_ref.restore();
    s_obj.restore();
    s_str.restore();

    crate::diff_eq!(ref_o, ref_n, "142052b60 ref");
    crate::diff_eq!(slot_o, slot_n, "142052b60 slot");
    crate::diff_eq!(rbx_o[8..], rbx_n[8..], "142052b60 rbx dtor/free");
    crate::diff_eq!(str_o, str_n, "142052b60 str_obj");
    Ok(1)
}

/// Original: `0x142052e50` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142052e50(rcx: usize, rdx: usize) -> usize {
    let rax = rd::<usize>(rcx);
    let val = rd::<usize>(rax);
    wr::<usize>(rax, 0);
    wr::<usize>(rdx, val);
    rdx
}

pub unsafe fn diff_d3d_tls_ctx_142052e50(orig: usize, new: usize) -> Result<u32, String> {
    let mut inner_o: usize = 0xdead_beef_cafe_babe;
    let mut outer_o: usize = (&mut inner_o as *mut usize) as usize;
    let mut out_o: usize = 0;

    let mut inner_n: usize = 0xdead_beef_cafe_babe;
    let mut outer_n: usize = (&mut inner_n as *mut usize) as usize;
    let mut out_n: usize = 0;

    let ro = exe::call_va!(
        orig,
        ((&mut outer_o as *mut usize) as usize, (&mut out_o as *mut usize) as usize) -> usize
    );
    let rn = exe::call_va!(
        new,
        ((&mut outer_n as *mut usize) as usize, (&mut out_n as *mut usize) as usize) -> usize
    );
    crate::diff_eq!(ro, (&out_o as *const usize) as usize, "142052e50 ret orig");
    crate::diff_eq!(rn, (&out_n as *const usize) as usize, "142052e50 ret new");
    crate::diff_eq!(inner_o, inner_n, "142052e50 inner cleared");
    crate::diff_eq!(out_o, out_n, "142052e50 out written");
    Ok(1)
}

/// Original: `0x142053380` (7 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142053380(ecx: u32) -> u8 {
    (ecx == 1) as u8
}

pub unsafe fn diff_d3d_tls_ctx_142053380(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u32..5 {
        let r_orig = exe::call_va!(orig, (v) -> u8);
        let r_new = exe::call_va!(new, (v) -> u8);
        crate::diff_eq!(r_orig, r_new, "142053380");
    }
    Ok(5)
}

/// Original: `0x142053390` (10 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_142053390(ecx: u32) -> u8 {
    ((ecx & 0xffff_fffd) == 0) as u8
}

pub unsafe fn diff_d3d_tls_ctx_142053390(orig: usize, new: usize) -> Result<u32, String> {
    for v in 0u32..6 {
        let r_orig = exe::call_va!(orig, (v) -> u8);
        let r_new = exe::call_va!(new, (v) -> u8);
        crate::diff_eq!(r_orig, r_new, "142053390");
    }
    Ok(6)
}

/// Original: `0x1420533b0` (191 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_1420533b0(val: f32) {
    let globals = [
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_4d58) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_4a98) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_57b8) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_5498) as usize),
        rd::<usize>(exe::ptr::<usize>(0x1_43a3_5268) as usize),
    ];
    let mut stack_pkt = [0u8; 0x38];
    let pkt_addr = stack_pkt.as_mut_ptr() as usize;
    for &r8 in &globals {
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        for off in (0..0x30).step_by(8) {
            wr::<u64>(pkt_addr + off, rd::<u64>(r8 + 0x8 + off));
        }
        wr::<u32>(pkt_addr + 0x30, rd::<u32>(r8 + 0x38));
        wr::<f32>(pkt_addr + 0x10, val);
        let arg3 = rd::<usize>(rd::<usize>(r8));
        let vtbl = rd::<usize>(ctx);
        let vfn = rd::<usize>(vtbl + 0xb0);
        exe::call_va!(vfn, (ctx, pkt_addr, arg3) -> ());
    }
    wr::<f32>(exe::ptr::<f32>(0x1_4397_db78) as usize, val);
}

pub unsafe fn diff_d3d_tls_ctx_1420533b0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_vfn_b0(ctx: usize, pkt: usize, arg3: usize) {
        let idx = rd::<u32>(ctx + 0x8) as usize;
        let dst = ctx + 0x10 + idx * 0x40;
        std::ptr::copy_nonoverlapping(pkt as *const u8, dst as *mut u8, 0x34);
        wr::<usize>(dst + 0x38, arg3);
        wr::<u32>(ctx + 0x8, (idx + 1) as u32);
    }

    type F = unsafe extern "win64" fn(f32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_vas = [
        0x1_43a3_4d58u64,
        0x1_43a3_4a98,
        0x1_43a3_57b8,
        0x1_43a3_5498,
        0x1_43a3_5268,
    ];
    let snaps: Vec<_> = g_vas
        .iter()
        .map(|&va| crate::difftest::Snapshot::take(exe::ptr::<u8>(va) as *mut u8, 8))
        .collect();
    let s_ctx = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fc80) as *mut u8, 8);
    let s_flt = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_db78) as *mut u8, 4);

    let mut arg3_targets = [0usize; 5];
    let mut items = [[0u8; 0x40]; 5];
    for i in 0..5 {
        arg3_targets[i] = (i + 1) * 0x1111;
        let p = items[i].as_mut_ptr() as usize;
        wr::<usize>(p, (&arg3_targets[i] as *const usize) as usize);
        for b in 0..0x34usize {
            wr::<u8>(p + 0x8 + b, ((i * 17 + b) & 0xff) as u8);
        }
        wr::<usize>(exe::ptr::<usize>(g_vas[i]) as usize, p);
    }

    let mut vtbl = [0usize; 24];
    vtbl[0xb0 / 8] = fake_vfn_b0 as *const () as usize;
    let mut ctx_o = [0u8; 0x160];
    let mut ctx_n = [0u8; 0x160];
    wr::<usize>(ctx_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(ctx_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize, ctx_o.as_mut_ptr() as usize);
    f_orig(2.5f32);
    let flt_o = rd::<u32>(exe::ptr::<u32>(0x1_4397_db78) as usize);

    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize, ctx_n.as_mut_ptr() as usize);
    f_new(2.5f32);
    let flt_n = rd::<u32>(exe::ptr::<u32>(0x1_4397_db78) as usize);

    for s in &snaps {
        s.restore();
    }
    s_ctx.restore();
    s_flt.restore();

    crate::diff_eq!(flt_o, flt_n, "1420533b0 flt");
    crate::diff_eq!(ctx_o[8..], ctx_n[8..], "1420533b0 ctx");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x14204eef0, "d3d::d3d_tls_ctx_14204eef0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204eef0 as *const (), Throws::Caught, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204eef0),
// REGISTRY: Replacement::verified_throws(0x14204efd0, "d3d::d3d_tls_ctx_14204efd0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204efd0 as *const (), Throws::Caught, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204efd0),
// REGISTRY: Replacement::verified_throws(0x14204f090, "d3d::d3d_tls_ctx_14204f090", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204f090 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204f090),
// REGISTRY: Replacement::verified_throws(0x14204f2f0, "d3d::d3d_tls_ctx_14204f2f0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204f2f0 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204f2f0),
// REGISTRY: Replacement::verified(0x14204f450, "d3d::d3d_tls_ctx_14204f450", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204f450 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204f450),
// REGISTRY: Replacement::verified_throws(0x14204f860, "d3d::d3d_tls_ctx_14204f860", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_14204f860 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_14204f860),
// REGISTRY: Replacement::verified_throws(0x142050070, "d3d::d3d_tls_ctx_142050070", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050070 as *const (), Throws::Caught, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050070),
// REGISTRY: Replacement::verified(0x1420501f0, "d3d::d3d_tls_ctx_1420501f0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420501f0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420501f0),
// REGISTRY: Replacement::verified_throws(0x142050230, "d3d::d3d_tls_ctx_142050230", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050230 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050230),
// REGISTRY: Replacement::verified(0x1420502e0, "d3d::d3d_tls_ctx_1420502e0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420502e0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420502e0),
// REGISTRY: Replacement::verified(0x142050310, "d3d::d3d_tls_ctx_142050310", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050310 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050310),
// REGISTRY: Replacement::verified(0x142050340, "d3d::d3d_tls_ctx_142050340", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050340 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050340),
// REGISTRY: Replacement::verified(0x1420503b0, "d3d::d3d_tls_ctx_1420503b0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420503b0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420503b0),
// REGISTRY: Replacement::verified(0x142050590, "d3d::d3d_tls_ctx_142050590", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050590 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050590),
// REGISTRY: Replacement::verified_throws(0x1420507c0, "d3d::d3d_tls_ctx_1420507c0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420507c0 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420507c0),
// REGISTRY: Replacement::verified(0x142050890, "d3d::d3d_tls_ctx_142050890", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142050890 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142050890),
// REGISTRY: Replacement::verified_throws(0x142051040, "d3d::d3d_tls_ctx_142051040", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142051040 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142051040),
// REGISTRY: Replacement::verified_throws(0x1420510c0, "d3d::d3d_tls_ctx_1420510c0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420510c0 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420510c0),
// REGISTRY: Replacement::verified_throws(0x142051150, "d3d::d3d_tls_ctx_142051150", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142051150 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142051150),
// REGISTRY: Replacement::verified(0x1420511d0, "d3d::d3d_tls_ctx_1420511d0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420511d0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420511d0),
// REGISTRY: Replacement::verified_throws(0x142051620, "d3d::d3d_tls_ctx_142051620", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142051620 as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142051620),
// REGISTRY: Replacement::verified(0x142051d00, "d3d::d3d_tls_ctx_142051d00", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142051d00 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142051d00),
// REGISTRY: Replacement::verified(0x142052700, "d3d::d3d_tls_ctx_142052700", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142052700 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142052700),
// REGISTRY: Replacement::verified(0x142052ae0, "d3d::d3d_tls_ctx_142052ae0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142052ae0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142052ae0),
// REGISTRY: Replacement::verified(0x142052b60, "d3d::d3d_tls_ctx_142052b60", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_142052b60 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_142052b60),
// REGISTRY: Replacement::verified(0x1420533b0, "d3d::d3d_tls_ctx_1420533b0", crate::ns::rend::d3d_tls_ctx_tail::d3d_tls_ctx_1420533b0 as *const (), crate::ns::rend::d3d_tls_ctx_tail::diff_d3d_tls_ctx_1420533b0),
