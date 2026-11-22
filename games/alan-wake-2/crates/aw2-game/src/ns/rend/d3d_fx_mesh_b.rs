use crate::exe::{self, rd, wr};

/// Original: `0x1420a3740` (29 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3740(
    rcx: usize,
    rdx: usize,
    r8: u16,
) -> usize {
    wr::<usize>(rcx, rdx);
    wr::<u16>(rcx + 0x8, r8);
    wr::<u8>(rcx + 0xc, 0);
    exe::call_va!(0x1_420a_3310, (rcx) -> ());
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a3740(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let nodes = [0u8; 128];
    let mut rdx1 = [0u8; 32];
    let mut r8_arr = [0u16; 8];
    wr::<u32>(rdx1.as_mut_ptr() as usize + 8 + 4, 2);
    wr::<u16>(rdx1.as_mut_ptr() as usize + 8 + 2, 1);

    let rdx0 = [nodes.as_ptr() as usize, 0usize, rdx1.as_ptr() as usize];
    let rdx2 = [r8_arr.as_mut_ptr() as usize];
    let mut r8_ctx = [0u8; 32];
    wr::<usize>(r8_ctx.as_mut_ptr() as usize, rdx0.as_ptr() as usize);
    wr::<usize>(r8_ctx.as_mut_ptr() as usize + 0x18, rdx2.as_ptr() as usize);

    let mut container = [0u8; 24];
    wr::<u16>(container.as_mut_ptr() as usize + 0x2, 2);
    wr::<usize>(container.as_mut_ptr() as usize + 0x8, r8_ctx.as_ptr() as usize);
    wr::<u16>(container.as_mut_ptr() as usize + 0x10, 1);

    let mut iter_o = [0u8; 16];
    let mut iter_n = [0u8; 16];
    let r_o = orig_fn(iter_o.as_mut_ptr() as usize, container.as_ptr() as usize, 0);
    let r_n = new_fn(iter_n.as_mut_ptr() as usize, container.as_ptr() as usize, 0);
    crate::diff_eq!(r_o == iter_o.as_ptr() as usize, r_n == iter_n.as_ptr() as usize, "1420a3740 ret");
    crate::diff_eq!(iter_o, iter_n, "1420a3740 advance");
    Ok(1)
}

/// Original: `0x1420a3770` (9 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3770(rcx: usize) {
    let cur = rd::<u16>(rcx + 0x8);
    wr::<u16>(rcx + 0x8, cur.wrapping_add(1));
    exe::call_va!(0x1_420a_3310, (rcx) -> ());
}

pub unsafe fn diff_d3d_unknown_1420a3770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let nodes = [0u8; 128];
    let mut rdx1 = [0u8; 32];
    let mut r8_arr = [0u16; 8];
    wr::<u32>(rdx1.as_mut_ptr() as usize + 16 + 4, 2);
    wr::<u16>(rdx1.as_mut_ptr() as usize + 16 + 2, 1);

    let rdx0 = [nodes.as_ptr() as usize, 0usize, rdx1.as_ptr() as usize];
    let rdx2 = [r8_arr.as_mut_ptr() as usize];
    let mut r8_ctx = [0u8; 32];
    wr::<usize>(r8_ctx.as_mut_ptr() as usize, rdx0.as_ptr() as usize);
    wr::<usize>(r8_ctx.as_mut_ptr() as usize + 0x18, rdx2.as_ptr() as usize);

    let mut container = [0u8; 24];
    wr::<u16>(container.as_mut_ptr() as usize + 0x2, 3);
    wr::<usize>(container.as_mut_ptr() as usize + 0x8, r8_ctx.as_ptr() as usize);
    wr::<u16>(container.as_mut_ptr() as usize + 0x10, 1);

    let mut iter_o = [0u8; 16];
    wr::<usize>(iter_o.as_mut_ptr() as usize, container.as_ptr() as usize);
    wr::<u16>(iter_o.as_mut_ptr() as usize + 0x8, 0);
    let mut iter_n = iter_o;

    orig_fn(iter_o.as_mut_ptr() as usize);
    new_fn(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "1420a3770 next");
    Ok(1)
}

/// Original: `0x1420a3780` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3780(rcx: usize) -> u8 {
    let rax = rd::<usize>(rcx);
    let dx = rd::<u16>(rax + 0x2);
    (rd::<u16>(rcx + 0x8) != dx) as u8
}

pub unsafe fn diff_d3d_unknown_1420a3780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut target = [0u8; 16];
    let mut state = [0u8; 32];
    let t_ptr = target.as_mut_ptr() as usize;
    let s_ptr = state.as_mut_ptr() as usize;
    wr::<usize>(s_ptr, t_ptr);
    wr::<u16>(t_ptr + 0x2, 5);
    wr::<u16>(s_ptr + 0x8, 5);
    crate::diff_eq!(orig_fn(s_ptr), new_fn(s_ptr), "eq case");
    wr::<u16>(s_ptr + 0x8, 7);
    crate::diff_eq!(orig_fn(s_ptr), new_fn(s_ptr), "ne case");
    Ok(2)
}

/// Original: `0x1420a37e0` (34 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a37e0(
    rcx: usize,
    rdx: usize,
    r8: usize,
) -> usize {
    wr::<usize>(rcx, rdx);
    let rax = rd::<u64>(r8);
    wr::<u64>(rcx + 0x8, rax);
    wr::<u8>(rcx + 0x18, 0);
    exe::call_va!(0x1_420a_33f0, (rcx) -> ());
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a37e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut nodes = [0u8; 128];
    let mut items = [0u8; 96];
    wr::<u16>(nodes.as_mut_ptr() as usize + 0x22, 0);
    wr::<u16>(nodes.as_mut_ptr() as usize + 0x40 + 0x22, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x20, 0);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x21, 0);

    let inner = [nodes.as_ptr() as usize, 2usize];
    let rax = [0usize, 0usize, items.as_ptr() as usize];
    let c = [inner.as_ptr() as usize, rax.as_ptr() as usize];
    let r8 = [nodes.as_ptr() as usize];

    let mut iter_o = [0u8; 32];
    let mut iter_n = [0u8; 32];
    let r_o = orig_fn(iter_o.as_mut_ptr() as usize, c.as_ptr() as usize, r8.as_ptr() as usize);
    let r_n = new_fn(iter_n.as_mut_ptr() as usize, c.as_ptr() as usize, r8.as_ptr() as usize);
    crate::diff_eq!(r_o == iter_o.as_ptr() as usize, r_n == iter_n.as_ptr() as usize, "1420a37e0 ret");
    crate::diff_eq!(iter_o, iter_n, "1420a37e0 init");
    Ok(1)
}

/// Original: `0x1420a3820` (10 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3820(rcx: usize) {
    let cur = rd::<u64>(rcx + 0x8);
    wr::<u64>(rcx + 0x8, cur.wrapping_add(0x40));
    exe::call_va!(0x1_420a_33f0, (rcx) -> ());
}

pub unsafe fn diff_d3d_unknown_1420a3820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut nodes = [0u8; 192];
    let mut items = [0u8; 96];
    wr::<u16>(nodes.as_mut_ptr() as usize + 0x40 + 0x22, 0);
    wr::<u16>(nodes.as_mut_ptr() as usize + 0x80 + 0x22, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x20, 0);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, 1);
    wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x21, 0);

    let inner = [nodes.as_ptr() as usize, 3usize];
    let rax = [0usize, 0usize, items.as_ptr() as usize];
    let c = [inner.as_ptr() as usize, rax.as_ptr() as usize];

    let mut iter_o = [0u8; 32];
    wr::<usize>(iter_o.as_mut_ptr() as usize, c.as_ptr() as usize);
    wr::<usize>(iter_o.as_mut_ptr() as usize + 0x8, nodes.as_ptr() as usize);
    let mut iter_n = iter_o;

    orig_fn(iter_o.as_mut_ptr() as usize);
    new_fn(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "1420a3820 next");
    Ok(1)
}

/// Original: `0x1420a3830` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3830(rcx: usize) -> u8 {
    let rax = rd::<usize>(rcx);
    let rdx = rd::<usize>(rax);
    let count = rd::<u32>(rdx + 0x8) as u64;
    let end = rd::<u64>(rdx).wrapping_add(count << 6);
    (rd::<u64>(rcx + 0x8) != end) as u8
}

pub unsafe fn diff_d3d_unknown_1420a3830(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut inner = [0u8; 32];
    let mut mid = [0u8; 16];
    let mut outer = [0u8; 32];
    let i_ptr = inner.as_mut_ptr() as usize;
    let m_ptr = mid.as_mut_ptr() as usize;
    let o_ptr = outer.as_mut_ptr() as usize;
    wr::<u64>(i_ptr, 0x1000);
    wr::<u32>(i_ptr + 0x8, 3);
    wr::<usize>(m_ptr, i_ptr);
    wr::<usize>(o_ptr, m_ptr);
    wr::<u64>(o_ptr + 0x8, 0x1000 + 3 * 0x40);
    crate::diff_eq!(orig_fn(o_ptr), new_fn(o_ptr), "at end");
    wr::<u64>(o_ptr + 0x8, 0x1040);
    crate::diff_eq!(orig_fn(o_ptr), new_fn(o_ptr), "not at end");
    Ok(2)
}

/// Original: `0x1420a3880` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3880(
    rcx: usize,
    rdx: usize,
    r8: usize,
) -> usize {
    wr::<usize>(rcx, rdx);
    let v0 = rd::<u64>(r8);
    wr::<u64>(rcx + 0x8, v0);
    let v1 = rd::<u64>(r8 + 0x8);
    wr::<u64>(rcx + 0x10, v1);
    wr::<u8>(rcx + 0x28, 0);
    exe::call_va!(0x1_420a_34a0, (rcx) -> ());
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a3880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let entries: [[u16; 4]; 2] = [[1, 2, 0, 0], [3, 0xffff, 0, 0]];
    let inner = [entries.as_ptr() as usize, 2usize];
    let c = [inner.as_ptr() as usize];
    let r8 = [entries.as_ptr() as usize, 5usize];

    let mut iter_o = [0u8; 48];
    let mut iter_n = [0u8; 48];
    let r_o = orig_fn(iter_o.as_mut_ptr() as usize, c.as_ptr() as usize, r8.as_ptr() as usize);
    let r_n = new_fn(iter_n.as_mut_ptr() as usize, c.as_ptr() as usize, r8.as_ptr() as usize);
    crate::diff_eq!(r_o == iter_o.as_ptr() as usize, r_n == iter_n.as_ptr() as usize, "1420a3880 ret");
    crate::diff_eq!(iter_o, iter_n, "1420a3880 init");
    Ok(1)
}

/// Original: `0x1420a38d0` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a38d0(rcx: usize) {
    wr::<u64>(rcx + 0x10, rd::<u64>(rcx + 0x10).wrapping_add(1));
    wr::<u64>(rcx + 0x8, rd::<u64>(rcx + 0x8).wrapping_add(0x8));
    exe::call_va!(0x1_420a_34a0, (rcx) -> ());
}

pub unsafe fn diff_d3d_unknown_1420a38d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let entries: [[u16; 4]; 3] = [[1, 0xffff, 0, 0], [2, 3, 0, 0], [4, 0xffff, 0, 0]];
    let inner = [entries.as_ptr() as usize, 3usize];
    let c = [inner.as_ptr() as usize];

    let mut iter_o = [0u8; 48];
    wr::<usize>(iter_o.as_mut_ptr() as usize, c.as_ptr() as usize);
    wr::<usize>(iter_o.as_mut_ptr() as usize + 0x8, entries.as_ptr() as usize);
    wr::<u64>(iter_o.as_mut_ptr() as usize + 0x10, 0);
    let mut iter_n = iter_o;

    orig_fn(iter_o.as_mut_ptr() as usize);
    new_fn(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "1420a38d0 next");
    Ok(1)
}

/// Original: `0x1420a3d50` (5 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3d50(rcx: usize, rdx: u16) {
    wr::<u16>(rcx + 0x2, rdx);
}

pub unsafe fn diff_d3d_unknown_1420a3d50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut b1 = [0u8; 16];
    let mut b2 = [0u8; 16];
    orig_fn(b1.as_mut_ptr() as usize, 0x1234);
    new_fn(b2.as_mut_ptr() as usize, 0x1234);
    crate::diff_eq!(b1, b2, "write u16 at +2");
    Ok(1)
}

/// Original: `0x1420a3d70` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3d70(rcx: usize, rdx: usize) -> u8 {
    let ax = rd::<u16>(rcx);
    (rd::<u16>(rdx + 0x22) == ax) as u8
}

pub unsafe fn diff_d3d_unknown_1420a3d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut a = [0u8; 8];
    let mut b = [0u8; 64];
    let pa = a.as_mut_ptr() as usize;
    let pb = b.as_mut_ptr() as usize;
    wr::<u16>(pa, 42);
    wr::<u16>(pb + 0x22, 42);
    crate::diff_eq!(orig_fn(pa, pb), new_fn(pa, pb), "match");
    wr::<u16>(pb + 0x22, 99);
    crate::diff_eq!(orig_fn(pa, pb), new_fn(pa, pb), "mismatch");
    Ok(2)
}

/// Original: `0x1420a3d90` (85 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a3d90(
    rcx: usize,
    rdx: usize,
    r8: u16,
) -> usize {
    let count = rd::<u64>(rdx);
    let base = rd::<usize>(rdx + 0x8);
    let mut rax = base.wrapping_add((count << 6) as usize);
    if rax == base {
        wr::<u8>(rcx + 0x2, 0);
        return rcx;
    }
    loop {
        let next = rax.wrapping_sub(0x40);
        if rd::<u16>(rax - 0x1e) == r8 {
            break;
        }
        rax = next;
        if next == base {
            break;
        }
    }
    if rax == base {
        wr::<u8>(rcx + 0x2, 0);
        return rcx;
    }
    let val = rd::<u16>(rax - 0x20);
    wr::<u16>(rcx, val);
    wr::<u8>(rcx + 0x2, 1);
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a3d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut entries = [0u8; 128];
    let mut desc = [0u8; 16];
    let mut out1 = [0u8; 8];
    let mut out2 = [0u8; 8];
    let e_ptr = entries.as_mut_ptr() as usize;
    let d_ptr = desc.as_mut_ptr() as usize;
    wr::<u16>(e_ptr + 0x20, 0x1111);
    wr::<u16>(e_ptr + 0x22, 10);
    wr::<u16>(e_ptr + 0x60, 0x2222);
    wr::<u16>(e_ptr + 0x62, 20);
    wr::<u64>(d_ptr, 2);
    wr::<usize>(d_ptr + 0x8, e_ptr);
    orig_fn(out1.as_mut_ptr() as usize, d_ptr, 10);
    new_fn(out2.as_mut_ptr() as usize, d_ptr, 10);
    crate::diff_eq!(out1, out2, "found first");
    orig_fn(out1.as_mut_ptr() as usize, d_ptr, 30);
    new_fn(out2.as_mut_ptr() as usize, d_ptr, 30);
    crate::diff_eq!(out1, out2, "not found");
    Ok(2)
}

/// Original: `0x1420a43e0` (77 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_unknown_1420a43e0(rcx: usize) {
    let p50 = rd::<usize>(rcx + 0x50);
    if p50 != 0 {
        exe::call_va!(0x1_4241_c190, (p50) -> ());
    }
    wr::<u64>(rcx + 0x50, 0);
    wr::<u64>(rcx + 0x58, 0);
    let p40 = rd::<usize>(rcx + 0x40);
    if p40 != 0 {
        exe::call_va!(0x1_4241_c190, (p40) -> ());
    }
    wr::<u64>(rcx + 0x40, 0);
    wr::<u64>(rcx + 0x48, 0);
    exe::call_va!(0x1_4065_ffa0, (rcx) -> ());
}

pub unsafe fn diff_d3d_unknown_1420a43e0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_c190(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0xc190));
    }
    unsafe extern "win64" fn fake_ffa0(rcx: usize) {
        wr::<u64>(rcx, 0xfeed_ffa0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4241_c190, fake_c190 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4065_ffa0, fake_ffa0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut b40_o = [0u8; 8];
    let mut b40_n = [0u8; 8];
    let mut b50_o = [0u8; 8];
    let mut b50_n = [0u8; 8];
    let mut s_o = [0xffu8; 0x60];
    let mut s_n = [0xffu8; 0x60];
    wr::<usize>(s_o.as_mut_ptr() as usize + 0x40, b40_o.as_mut_ptr() as usize);
    wr::<usize>(s_o.as_mut_ptr() as usize + 0x50, b50_o.as_mut_ptr() as usize);
    wr::<usize>(s_n.as_mut_ptr() as usize + 0x40, b40_n.as_mut_ptr() as usize);
    wr::<usize>(s_n.as_mut_ptr() as usize + 0x50, b50_n.as_mut_ptr() as usize);

    orig_fn(s_o.as_mut_ptr() as usize);
    new_fn(s_n.as_mut_ptr() as usize);
    crate::diff_eq!(s_o, s_n, "1420a43e0 non-null");
    crate::diff_eq!(b40_o, b40_n, "1420a43e0 b40");
    crate::diff_eq!(b50_o, b50_n, "1420a43e0 b50");

    orig_fn(s_o.as_mut_ptr() as usize);
    new_fn(s_n.as_mut_ptr() as usize);
    crate::diff_eq!(s_o, s_n, "1420a43e0 null");
    Ok(2)
}

/// Original: `0x1420a4460` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a4460(rcx: usize, rdx: usize) -> u8 {
    let idx = rd::<u16>(rdx + 0x22) as usize;
    let rax = rd::<usize>(rcx);
    let base = rd::<usize>(rax + 0x10);
    let entry = base + idx * 0x30;
    if rd::<u8>(entry + 0x20) != 0 && rd::<u8>(entry + 0x21) == 0 {
        1
    } else {
        0
    }
}

pub unsafe fn diff_d3d_unknown_1420a4460(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut table = [0u8; 96];
    let mut inner = [0u8; 32];
    let mut outer = [0u8; 16];
    let mut arg = [0u8; 64];
    let t_ptr = table.as_mut_ptr() as usize;
    let i_ptr = inner.as_mut_ptr() as usize;
    let o_ptr = outer.as_mut_ptr() as usize;
    let a_ptr = arg.as_mut_ptr() as usize;
    wr::<usize>(i_ptr + 0x10, t_ptr);
    wr::<usize>(o_ptr, i_ptr);
    wr::<u16>(a_ptr + 0x22, 1);
    wr::<u8>(t_ptr + 0x30 + 0x20, 1);
    wr::<u8>(t_ptr + 0x30 + 0x21, 0);
    crate::diff_eq!(orig_fn(o_ptr, a_ptr), new_fn(o_ptr, a_ptr), "active");
    wr::<u8>(t_ptr + 0x30 + 0x21, 1);
    crate::diff_eq!(orig_fn(o_ptr, a_ptr), new_fn(o_ptr, a_ptr), "masked");
    Ok(2)
}

/// Original: `0x1420a45f0` (81 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a45f0(
    rcx: usize,
    rdx: u16,
    r8: u16,
) -> u8 {
    let r9 = rdx as usize;
    let rdx_ptr = rd::<usize>(rcx);
    let rcx_18 = rd::<usize>(rcx + 0x18);
    let rax = rd::<usize>(rdx_ptr).wrapping_add((r8 as usize) << 6);
    let table10 = rd::<usize>(rdx_ptr + 0x10);
    let r10 = table10 + 8 * r9;
    let table_rcx = rd::<usize>(rcx_18);
    if rd::<u32>(r10 + 0x4) != 2 {
        return 0;
    }
    let cx = rd::<u16>(r10 + 0x2);
    if cx == r8 {
        return 1;
    }
    if cx != 0xffff {
        return 0;
    }
    (rd::<u16>(table_rcx + 2 * r9) == rd::<u16>(rax + 0x22)) as u8
}

pub unsafe fn diff_d3d_unknown_1420a45f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u16) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut nodes = [0u8; 128];
    let mut tbl10 = [0u8; 32];
    let mut tbl18 = [0u8; 16];
    let mut sub0 = [0u8; 32];
    let mut sub18 = [0u8; 16];
    let mut ctx = [0u8; 32];
    let n_ptr = nodes.as_mut_ptr() as usize;
    let t10_ptr = tbl10.as_mut_ptr() as usize;
    let t18_ptr = tbl18.as_mut_ptr() as usize;
    let s0_ptr = sub0.as_mut_ptr() as usize;
    let s18_ptr = sub18.as_mut_ptr() as usize;
    let c_ptr = ctx.as_mut_ptr() as usize;
    wr::<usize>(s0_ptr, n_ptr);
    wr::<usize>(s0_ptr + 0x10, t10_ptr);
    wr::<usize>(s18_ptr, t18_ptr);
    wr::<usize>(c_ptr, s0_ptr);
    wr::<usize>(c_ptr + 0x18, s18_ptr);
    wr::<u32>(t10_ptr + 8 + 4, 2);
    wr::<u16>(t10_ptr + 8 + 2, 0xffff);
    wr::<u16>(n_ptr + 0x40 + 0x22, 77);
    wr::<u16>(t18_ptr + 2, 77);
    crate::diff_eq!(orig_fn(c_ptr, 1, 1), new_fn(c_ptr, 1, 1), "wildcard match");
    wr::<u16>(t18_ptr + 2, 78);
    crate::diff_eq!(orig_fn(c_ptr, 1, 1), new_fn(c_ptr, 1, 1), "wildcard mismatch");
    Ok(2)
}

/// Original: `0x1420a4690` (88 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a4690(rcx: usize, rdx: u16) -> u8 {
    let r10w = rd::<u16>(rcx + 0x8);
    let r9 = rd::<usize>(rcx);
    let r8 = rdx as usize;
    let rcx0 = rd::<usize>(r9);
    let rax = rd::<usize>(rcx0).wrapping_add((r10w as usize) << 6);
    let table10 = rd::<usize>(rcx0 + 0x10);
    let r11 = table10 + 8 * r8;
    let rcx18 = rd::<usize>(r9 + 0x18);
    let table_rdx = rd::<usize>(rcx18);
    if rd::<u32>(r11 + 0x4) != 2 {
        return 0;
    }
    let cx = rd::<u16>(r11 + 0x2);
    if cx == r10w {
        return 1;
    }
    if cx != 0xffff {
        return 0;
    }
    (rd::<u16>(table_rdx + 2 * r8) == rd::<u16>(rax + 0x22)) as u8
}

pub unsafe fn diff_d3d_unknown_1420a4690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut nodes = [0u8; 128];
    let mut tbl10 = [0u8; 32];
    let mut tbl18 = [0u8; 16];
    let mut sub0 = [0u8; 32];
    let mut sub18 = [0u8; 16];
    let mut ctx = [0u8; 32];
    let mut iter = [0u8; 16];
    let n_ptr = nodes.as_mut_ptr() as usize;
    let t10_ptr = tbl10.as_mut_ptr() as usize;
    let t18_ptr = tbl18.as_mut_ptr() as usize;
    let s0_ptr = sub0.as_mut_ptr() as usize;
    let s18_ptr = sub18.as_mut_ptr() as usize;
    let c_ptr = ctx.as_mut_ptr() as usize;
    let it_ptr = iter.as_mut_ptr() as usize;
    wr::<usize>(s0_ptr, n_ptr);
    wr::<usize>(s0_ptr + 0x10, t10_ptr);
    wr::<usize>(s18_ptr, t18_ptr);
    wr::<usize>(c_ptr, s0_ptr);
    wr::<usize>(c_ptr + 0x18, s18_ptr);
    wr::<usize>(it_ptr, c_ptr);
    wr::<u16>(it_ptr + 0x8, 1);
    wr::<u32>(t10_ptr + 8 + 4, 2);
    wr::<u16>(t10_ptr + 8 + 2, 1);
    crate::diff_eq!(orig_fn(it_ptr, 1), new_fn(it_ptr, 1), "direct match");
    wr::<u32>(t10_ptr + 8 + 4, 0);
    crate::diff_eq!(orig_fn(it_ptr, 1), new_fn(it_ptr, 1), "type mismatch");
    Ok(2)
}

/// Original: `0x1420a4ae0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a4ae0(rcx: usize, rdx: u16) -> u8 {
    let r8 = rd::<usize>(rcx);
    let rcx0 = rd::<usize>(r8);
    let rax = rd::<usize>(rcx0).wrapping_add((rdx as usize) << 6);
    let idx = rd::<u16>(rax + 0x22) as usize;
    let rcx10 = rd::<usize>(r8 + 0x10);
    let base = rd::<usize>(rcx10 + 0x10);
    rd::<u8>(base + idx * 0x30 + 0x20)
}

pub unsafe fn diff_d3d_unknown_1420a4ae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut nodes = [0u8; 128];
    let mut items = [0u8; 128];
    let mut s0 = [0u8; 16];
    let mut s10 = [0u8; 32];
    let mut mid = [0u8; 32];
    let mut top = [0u8; 16];
    let n_ptr = nodes.as_mut_ptr() as usize;
    let i_ptr = items.as_mut_ptr() as usize;
    let s0_ptr = s0.as_mut_ptr() as usize;
    let s10_ptr = s10.as_mut_ptr() as usize;
    let m_ptr = mid.as_mut_ptr() as usize;
    let t_ptr = top.as_mut_ptr() as usize;
    wr::<usize>(s0_ptr, n_ptr);
    wr::<usize>(s10_ptr + 0x10, i_ptr);
    wr::<usize>(m_ptr, s0_ptr);
    wr::<usize>(m_ptr + 0x10, s10_ptr);
    wr::<usize>(t_ptr, m_ptr);
    wr::<u16>(n_ptr + 0x40 + 0x22, 1);
    wr::<u8>(i_ptr + 0x30 + 0x20, 0x5a);
    crate::diff_eq!(orig_fn(t_ptr, 1), new_fn(t_ptr, 1), "lookup byte");
    Ok(1)
}

/// Original: `0x1420a4ed0` (7 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a4ed0(_rcx: usize, rdx: u32) -> u16 {
    (rdx >> 16) as u16
}

pub unsafe fn diff_d3d_unknown_1420a4ed0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u16;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    crate::diff_eq!(orig_fn(0, 0xdead_beef), new_fn(0, 0xdead_beef), "hi16");
    Ok(1)
}

/// Original: `0x1420a5070` (35 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a5070(rcx: usize, rdx: usize) -> usize {
    let r8d = rd::<u32>(rcx + 0x20);
    let rax = rd::<usize>(rcx + 0x18);
    let val = rd::<u32>(rax + (r8d as usize) * 4 - 4);
    wr::<u32>(rcx + 0x20, r8d.wrapping_sub(1));
    wr::<u16>(rdx + 0x2, val as u16);
    wr::<u8>(rdx, (val >> 16) as u8);
    rdx
}

pub unsafe fn diff_d3d_unknown_1420a5070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut arr = [0u32; 4];
    arr[1] = 0x00ab_cdef;
    let mut s1 = [0u8; 40];
    let mut s2 = [0u8; 40];
    let mut o1 = [0u8; 8];
    let mut o2 = [0u8; 8];
    let a_ptr = arr.as_mut_ptr() as usize;
    wr::<usize>(s1.as_mut_ptr() as usize + 0x18, a_ptr);
    wr::<u32>(s1.as_mut_ptr() as usize + 0x20, 2);
    wr::<usize>(s2.as_mut_ptr() as usize + 0x18, a_ptr);
    wr::<u32>(s2.as_mut_ptr() as usize + 0x20, 2);
    orig_fn(s1.as_mut_ptr() as usize, o1.as_mut_ptr() as usize);
    new_fn(s2.as_mut_ptr() as usize, o2.as_mut_ptr() as usize);
    crate::diff_eq!(s1, s2, "state after pop");
    crate::diff_eq!(o1, o2, "out after pop");
    Ok(2)
}

/// Original: `0x1420a50e0` (99 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_unknown_1420a50e0(rcx: usize, rdx: usize) -> usize {
    wr::<usize>(rcx, rdx);
    let edi = rd::<u32>(rdx + 0x58);
    wr::<u64>(rcx + 0x8, 0);
    wr::<u64>(rcx + 0x10, 0);
    if edi != 0 {
        let buf = exe::call_va!(0x1_4241_c0a0, (edi as usize, 1usize) -> usize);
        wr::<usize>(rcx + 0x8, buf);
        wr::<u32>(rcx + 0x14, edi);
        exe::call_va!(0x1_4183_acc2, (buf, 0u32, edi as usize) -> usize);
    }
    wr::<u32>(rcx + 0x10, edi);
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x20, 0);
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a50e0(orig: usize, new: usize) -> Result<u32, String> {
    static mut SCRATCH_50E0: [u8; 32] = [0; 32];
    unsafe extern "win64" fn fake_c0a0(_sz: usize, _align: usize) -> usize {
        core::ptr::addr_of_mut!(SCRATCH_50E0) as usize
    }
    let _m = crate::difftest::Mock::new(0x1_4241_c0a0, fake_c0a0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for count in [0u32, 16u32] {
        let mut rdx_buf = [0u8; 0x60];
        wr::<u32>(rdx_buf.as_mut_ptr() as usize + 0x58, count);
        let mut s_o = [0xffu8; 0x28];
        let mut s_n = [0xffu8; 0x28];

        let scratch_ptr = core::ptr::addr_of_mut!(SCRATCH_50E0) as *mut u8;
        core::ptr::write_bytes(scratch_ptr, 0xaa, 32);
        let r_o = orig_fn(s_o.as_mut_ptr() as usize, rdx_buf.as_ptr() as usize);
        let snap_o = core::slice::from_raw_parts(scratch_ptr as *const u8, 32).to_vec();

        core::ptr::write_bytes(scratch_ptr, 0xaa, 32);
        let r_n = new_fn(s_n.as_mut_ptr() as usize, rdx_buf.as_ptr() as usize);
        let snap_n = core::slice::from_raw_parts(scratch_ptr as *const u8, 32).to_vec();

        crate::diff_eq!(r_o == s_o.as_ptr() as usize, r_n == s_n.as_ptr() as usize, "1420a50e0 ret");
        crate::diff_eq!(s_o, s_n, "1420a50e0 state");
        crate::diff_eq!(snap_o, snap_n, "1420a50e0 memset");
    }
    Ok(2)
}

/// Original: `0x1420a5340` (57 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_unknown_1420a5340(rcx: usize) {
    let p30 = rd::<usize>(rcx + 0x30);
    if p30 != 0 {
        exe::call_va!(0x1_4241_c190, (p30) -> ());
    }
    wr::<u64>(rcx + 0x30, 0);
    wr::<u64>(rcx + 0x38, 0);
    exe::call_va!(0x1_401d_e830, (rcx + 0x20) -> ());
    exe::call_va!(0x1_401e_59a0, (rcx + 0x10) -> ());
}

pub unsafe fn diff_d3d_unknown_1420a5340(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_c190(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0xc190));
    }
    unsafe extern "win64" fn fake_e830(p: usize) {
        wr::<u64>(p, 0x1111_2222);
    }
    unsafe extern "win64" fn fake_59a0(p: usize) {
        wr::<u64>(p, 0x3333_4444);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4241_c190, fake_c190 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, fake_e830 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_59a0, fake_59a0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut b30_o = [0u8; 8];
    let mut b30_n = [0u8; 8];
    let mut s_o = [0xffu8; 0x40];
    let mut s_n = [0xffu8; 0x40];
    wr::<usize>(s_o.as_mut_ptr() as usize + 0x30, b30_o.as_mut_ptr() as usize);
    wr::<usize>(s_n.as_mut_ptr() as usize + 0x30, b30_n.as_mut_ptr() as usize);

    orig_fn(s_o.as_mut_ptr() as usize);
    new_fn(s_n.as_mut_ptr() as usize);
    crate::diff_eq!(s_o, s_n, "1420a5340 non-null");
    crate::diff_eq!(b30_o, b30_n, "1420a5340 b30");

    orig_fn(s_o.as_mut_ptr() as usize);
    new_fn(s_n.as_mut_ptr() as usize);
    crate::diff_eq!(s_o, s_n, "1420a5340 null");
    Ok(2)
}

/// Original: `0x1420a59e0` (152 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_unknown_1420a59e0(
    rcx: usize,
    rdx: usize,
    r8: u16,
    r9: u16,
) {
    let rax8 = rd::<usize>(rcx + 0x8);
    let r10 = rd::<usize>(rax8);
    let r11 = (r8 as usize) << 6;
    let pack_r9 = (r9 as u32) | 0x0001_0000;
    wr::<u32>(r10 + r11 + 0x36, pack_r9);
    let rdx8 = rd::<usize>(rcx + 0x8);
    let pack_r8 = (r8 as u32) | 0x0001_0000;
    let target = rd::<usize>(rdx8).wrapping_add((r9 as usize) << 6);
    wr::<u32>(target + 0x32, pack_r8);
    let idx = rd::<u16>(r10 + r11 + 0x22) as usize;
    let rcx0 = rd::<usize>(rcx);
    let item = rd::<usize>(rcx0 + 0x10) + idx * 0x30;
    if rd::<u8>(item + 0x20) != 0 {
        let arg5 = rd::<u16>(r10 + r11 + 0x22);
        exe::call_va!(
            0x1_420a_58b0,
            (rcx, rdx, r8, r9, arg5, 1u32) -> ()
        );
    }
}

pub unsafe fn diff_d3d_unknown_1420a59e0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_58b0(
        _rcx: usize,
        rdx: usize,
        r8: u16,
        r9: u16,
        arg5: u16,
        arg6: u32,
    ) {
        wr::<u16>(rdx, r8);
        wr::<u16>(rdx + 2, r9);
        wr::<u16>(rdx + 4, arg5);
        wr::<u32>(rdx + 8, arg6);
    }
    let _m = crate::difftest::Mock::new(0x1_420a_58b0, fake_58b0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize, u16, u16);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for active in [0u8, 1u8] {
        let mut nodes_o = [0u8; 128];
        let mut nodes_n = [0u8; 128];
        wr::<u16>(nodes_o.as_mut_ptr() as usize + 0x22, 1);
        wr::<u16>(nodes_n.as_mut_ptr() as usize + 0x22, 1);

        let mut items = [0u8; 96];
        wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, active);

        let rcx0 = [0usize, 0usize, items.as_ptr() as usize];
        let rax8_o = [nodes_o.as_mut_ptr() as usize];
        let rax8_n = [nodes_n.as_mut_ptr() as usize];
        let rcx_o = [rcx0.as_ptr() as usize, rax8_o.as_ptr() as usize];
        let rcx_n = [rcx0.as_ptr() as usize, rax8_n.as_ptr() as usize];

        let mut log_o = [0u8; 16];
        let mut log_n = [0u8; 16];
        orig_fn(rcx_o.as_ptr() as usize, log_o.as_mut_ptr() as usize, 0, 1);
        new_fn(rcx_n.as_ptr() as usize, log_n.as_mut_ptr() as usize, 0, 1);

        nodes_o[0x39] = 0;
        nodes_n[0x39] = 0;
        nodes_o[0x40 + 0x35] = 0;
        nodes_n[0x40 + 0x35] = 0;
        crate::diff_eq!(nodes_o, nodes_n, "1420a59e0 nodes");
        crate::diff_eq!(log_o, log_n, "1420a59e0 log");
    }
    Ok(2)
}

/// Original: `0x1420a5a80` (55 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a5a80(
    rcx: usize,
    rdx: usize,
    r8: u16,
) -> usize {
    let r9 = rd::<usize>(rcx + 0x8);
    let rax = rd::<usize>(r9).wrapping_add((r8 as usize) << 6);
    let idx = rd::<u16>(rax + 0x22) as usize;
    let base10 = rd::<usize>(rcx + 0x10);
    let slot = base10 + 4 * idx;
    let old = rd::<u32>(slot);
    wr::<u32>(rdx, old);
    wr::<u32>(slot, (r8 as u32) | 0x0001_0000);
    rdx
}

pub unsafe fn diff_d3d_unknown_1420a5a80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut nodes = [0u8; 128];
    wr::<u16>(nodes.as_mut_ptr() as usize + 0x40 + 0x22, 2);
    let r9 = [nodes.as_ptr() as usize];

    let mut slots_o = [0u32, 0, 0x0001_0077, 0];
    let mut slots_n = slots_o;
    let rcx_o = [0usize, r9.as_ptr() as usize, slots_o.as_mut_ptr() as usize];
    let rcx_n = [0usize, r9.as_ptr() as usize, slots_n.as_mut_ptr() as usize];
    let mut out_o = 0u32;
    let mut out_n = 0u32;

    let r_o = orig_fn(rcx_o.as_ptr() as usize, (&mut out_o as *mut u32) as usize, 1);
    let r_n = new_fn(rcx_n.as_ptr() as usize, (&mut out_n as *mut u32) as usize, 1);

    slots_o[2] &= 0x00ff_ffff;
    slots_n[2] &= 0x00ff_ffff;
    crate::diff_eq!(r_o == (&out_o as *const u32) as usize, r_n == (&out_n as *const u32) as usize, "1420a5a80 ret");
    crate::diff_eq!(out_o, out_n, "1420a5a80 out");
    crate::diff_eq!(slots_o, slots_n, "1420a5a80 slots");
    Ok(1)
}

/// Original: `0x1420a5ac0` (216 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_unknown_1420a5ac0(rcx: usize, rdx: u16) {
    let edi = rdx;
    let r11 = exe::call_va!(0x1_420a_5ba0, (rcx, edi) -> usize);
    let r8_ptr = rd::<usize>(rcx + 0x8);
    let rcx10 = rd::<usize>(rcx + 0x10);
    let r10 = (edi as usize) << 6;
    let r9_base = rd::<usize>(r8_ptr);
    let pack_di = (edi as u32) | 0x0001_0000;
    let idx = rd::<u16>(r9_base + r10 + 0x22) as usize;
    let prev = rd::<u32>(rcx10 + 4 * idx);
    wr::<u32>(rcx10 + 4 * idx, pack_di);
    if ((prev >> 16) as u8) != 0 {
        let prev_lo = prev as u16;
        let rax8 = rd::<usize>(rcx + 0x8);
        let rdx_base = rd::<usize>(rax8);
        let pack_prev = (prev_lo as u32) | 0x0001_0000;
        wr::<u32>(rdx_base + r10 + 0x36, pack_prev);
        let rcx8 = rd::<usize>(rcx + 0x8);
        let prev_node = rd::<usize>(rcx8).wrapping_add((prev_lo as usize) << 6);
        wr::<u32>(prev_node + 0x32, pack_di);
        let item_idx = rd::<u16>(rdx_base + r10 + 0x22) as usize;
        let rcx0 = rd::<usize>(rcx);
        let item = rd::<usize>(rcx0 + 0x10) + item_idx * 0x30;
        if rd::<u8>(item + 0x20) != 0 {
            let arg5 = rd::<u16>(rdx_base + r10 + 0x22);
            exe::call_va!(
                0x1_420a_58b0,
                (rcx, r11, edi, prev_lo, arg5, 1u32) -> ()
            );
        }
    }
}

pub unsafe fn diff_d3d_unknown_1420a5ac0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5ba0(rcx: usize, edi: u16) -> usize {
        let log = rd::<usize>(rcx + 0x18);
        wr::<u16>(log, edi);
        log
    }
    unsafe extern "win64" fn fake_58b0(
        _rcx: usize,
        rdx: usize,
        r8: u16,
        r9: u16,
        arg5: u16,
        arg6: u32,
    ) {
        wr::<u16>(rdx + 2, r8);
        wr::<u16>(rdx + 4, r9);
        wr::<u16>(rdx + 6, arg5);
        wr::<u32>(rdx + 8, arg6);
    }
    let _m1 = crate::difftest::Mock::new(0x1_420a_5ba0, fake_5ba0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420a_58b0, fake_58b0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, u16);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for prev_val in [0u32, 0x0001_0000u32] {
        let mut nodes_o = [0u8; 128];
        let mut nodes_n = [0u8; 128];
        wr::<u16>(nodes_o.as_mut_ptr() as usize + 0x40 + 0x22, 1);
        wr::<u16>(nodes_n.as_mut_ptr() as usize + 0x40 + 0x22, 1);

        let mut items = [0u8; 96];
        wr::<u8>(items.as_mut_ptr() as usize + 0x30 + 0x20, 1);
        let rcx0 = [0usize, 0usize, items.as_ptr() as usize];
        let r8_o = [nodes_o.as_mut_ptr() as usize];
        let r8_n = [nodes_n.as_mut_ptr() as usize];
        let mut slots_o = [0u32, prev_val];
        let mut slots_n = [0u32, prev_val];
        let mut log_o = [0u8; 16];
        let mut log_n = [0u8; 16];

        let rcx_o = [
            rcx0.as_ptr() as usize,
            r8_o.as_ptr() as usize,
            slots_o.as_mut_ptr() as usize,
            log_o.as_mut_ptr() as usize,
        ];
        let rcx_n = [
            rcx0.as_ptr() as usize,
            r8_n.as_ptr() as usize,
            slots_n.as_mut_ptr() as usize,
            log_n.as_mut_ptr() as usize,
        ];

        orig_fn(rcx_o.as_ptr() as usize, 1);
        new_fn(rcx_n.as_ptr() as usize, 1);

        slots_o[1] &= 0x00ff_ffff;
        slots_n[1] &= 0x00ff_ffff;
        nodes_o[0x35] = 0;
        nodes_n[0x35] = 0;
        nodes_o[0x40 + 0x39] = 0;
        nodes_n[0x40 + 0x39] = 0;

        crate::diff_eq!(slots_o, slots_n, "1420a5ac0 slots");
        crate::diff_eq!(nodes_o, nodes_n, "1420a5ac0 nodes");
        crate::diff_eq!(log_o, log_n, "1420a5ac0 log");
    }
    Ok(2)
}

/// Original: `0x1420a6020` (21 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6020(rcx: usize, rdx: u16) -> u8 {
    let rax = rd::<usize>(rcx);
    let base = rd::<usize>(rax + 0x10);
    (rd::<u8>(base + 4 * (rdx as usize) + 0x2) == 0) as u8
}

pub unsafe fn diff_d3d_unknown_1420a6020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut arr = [0u8; 32];
    let mut inner = [0u8; 32];
    let mut outer = [0u8; 16];
    let a_ptr = arr.as_mut_ptr() as usize;
    let i_ptr = inner.as_mut_ptr() as usize;
    let o_ptr = outer.as_mut_ptr() as usize;
    wr::<usize>(i_ptr + 0x10, a_ptr);
    wr::<usize>(o_ptr, i_ptr);
    wr::<u8>(a_ptr + 8 + 2, 0);
    crate::diff_eq!(orig_fn(o_ptr, 2), new_fn(o_ptr, 2), "is zero");
    wr::<u8>(a_ptr + 8 + 2, 1);
    crate::diff_eq!(orig_fn(o_ptr, 2), new_fn(o_ptr, 2), "non zero");
    Ok(2)
}

/// Original: `0x1420a6310` (143 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6310(
    rcx: usize,
    rdx: usize,
    r8: usize,
) -> usize {
    wr::<usize>(rcx, rdx);
    wr::<usize>(rcx + 0x8, r8);
    let mut proto = [0u8; 8];
    let count0 = rd::<u32>(rdx + 0x18);
    exe::call_va!(
        0x1_411e_be80,
        (rcx + 0x10, count0, proto.as_mut_ptr() as usize) -> ()
    );
    let rax0 = rd::<usize>(rcx);
    let esi = rd::<u32>(rax0 + 0x18);
    wr::<u64>(rcx + 0x20, 0);
    wr::<u64>(rcx + 0x28, 0);
    if esi != 0 {
        let buf = exe::call_va!(0x1_4028_e1c0, (rcx + 0x20, esi) -> usize);
        wr::<usize>(rcx + 0x20, buf);
        wr::<u32>(rcx + 0x28, 0);
        wr::<u32>(rcx + 0x2c, esi);
        exe::call_va!(0x1_4183_acc2, (buf, 0u32, (esi as usize) * 2) -> usize);
    }
    wr::<u32>(rcx + 0x28, esi);
    wr::<u64>(rcx + 0x30, 0);
    wr::<u64>(rcx + 0x38, 0);
    wr::<u64>(rcx + 0x40, 0);
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a6310(orig: usize, new: usize) -> Result<u32, String> {
    static mut SCRATCH_6310: [u8; 32] = [0; 32];
    unsafe extern "win64" fn fake_be80(rcx_10: usize, count: u32, proto_ptr: usize) {
        wr::<u32>(rcx_10, count);
        wr::<u32>(rcx_10 + 4, rd::<u8>(proto_ptr + 2) as u32);
        wr::<u64>(rcx_10 + 8, 0x1122_3344);
    }
    unsafe extern "win64" fn fake_e1c0(_rcx: usize, _count: u32) -> usize {
        core::ptr::addr_of_mut!(SCRATCH_6310) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411e_be80, fake_be80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4028_e1c0, fake_e1c0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for count in [0u32, 6u32] {
        let mut rdx_buf = [0u8; 32];
        wr::<u32>(rdx_buf.as_mut_ptr() as usize + 0x18, count);
        let mut s_o = [0xffu8; 0x48];
        let mut s_n = [0xffu8; 0x48];
        let scratch_ptr = core::ptr::addr_of_mut!(SCRATCH_6310) as *mut u8;

        core::ptr::write_bytes(scratch_ptr, 0xbb, 32);
        let r_o = orig_fn(s_o.as_mut_ptr() as usize, rdx_buf.as_ptr() as usize, 0x5566_7788);
        let snap_o = core::slice::from_raw_parts(scratch_ptr as *const u8, 32).to_vec();

        core::ptr::write_bytes(scratch_ptr, 0xbb, 32);
        let r_n = new_fn(s_n.as_mut_ptr() as usize, rdx_buf.as_ptr() as usize, 0x5566_7788);
        let snap_n = core::slice::from_raw_parts(scratch_ptr as *const u8, 32).to_vec();

        crate::diff_eq!(r_o == s_o.as_ptr() as usize, r_n == s_n.as_ptr() as usize, "1420a6310 ret");
        crate::diff_eq!(s_o, s_n, "1420a6310 state");
        crate::diff_eq!(snap_o, snap_n, "1420a6310 memset");
    }
    Ok(2)
}

/// Original: `0x1420a63a0` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a63a0(rcx: usize, rdx: u16) -> usize {
    rd::<usize>(rcx + 0x10).wrapping_add((rdx as usize) * 0x30)
}

pub unsafe fn diff_d3d_unknown_1420a63a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut s = [0u8; 32];
    let s_ptr = s.as_mut_ptr() as usize;
    wr::<usize>(s_ptr + 0x10, 0x2000_0000);
    crate::diff_eq!(orig_fn(s_ptr, 5), new_fn(s_ptr, 5), "index 5");
    Ok(1)
}

/// Original: `0x1420a63c0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a63c0(
    rcx: usize,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: usize,
    arg6: usize,
) -> usize {
    wr::<u64>(rcx, rdx);
    wr::<u64>(rcx + 0x8, r8);
    wr::<u64>(rcx + 0x10, r9);
    let v0 = rd::<u128>(arg5);
    wr::<u128>(rcx + 0x18, v0);
    let v1 = rd::<u64>(arg6);
    wr::<u64>(rcx + 0x28, v1);
    rcx
}

pub unsafe fn diff_d3d_unknown_1420a63c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64, usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut o1 = [0u8; 48];
    let mut o2 = [0u8; 48];
    let a5: u128 = 0x11223344_55667788_99aabbcc_ddeeff00;
    let a6: u64 = 0xfedcba98_76543210;
    orig_fn(
        o1.as_mut_ptr() as usize,
        1,
        2,
        3,
        (&a5 as *const u128) as usize,
        (&a6 as *const u64) as usize,
    );
    new_fn(
        o2.as_mut_ptr() as usize,
        1,
        2,
        3,
        (&a5 as *const u128) as usize,
        (&a6 as *const u64) as usize,
    );
    crate::diff_eq!(o1, o2, "init 48B struct");
    Ok(1)
}

/// Original: `0x1420a6510` (63 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6510(
    rcx: usize,
    rdx: usize,
    r8: u16,
) -> usize {
    let base = rd::<usize>(rcx + 0x10);
    let entry = base + (r8 as usize) * 0x30;
    let ecx = rd::<u32>(entry + 0x26);
    wr::<usize>(rdx, rcx);
    wr::<u32>(rdx + 0x8, ecx);
    wr::<u32>(rdx + 0xc, 0);
    wr::<u64>(rdx + 0x10, 0);
    wr::<u64>(rdx + 0x18, 0);
    rdx
}

pub unsafe fn diff_d3d_unknown_1420a6510(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut entries = [0u8; 96];
    wr::<u32>(entries.as_mut_ptr() as usize + 0x30 + 0x26, 0xabcd_ef12);
    let mut ctx = [0u8; 32];
    wr::<usize>(ctx.as_mut_ptr() as usize + 0x10, entries.as_ptr() as usize);

    let mut o1 = [0xffu8; 32];
    let mut o2 = [0xffu8; 32];
    let r_o = orig_fn(ctx.as_ptr() as usize, o1.as_mut_ptr() as usize, 1);
    let r_n = new_fn(ctx.as_ptr() as usize, o2.as_mut_ptr() as usize, 1);

    crate::diff_eq!(r_o == o1.as_ptr() as usize, r_n == o2.as_ptr() as usize, "1420a6510 ret");
    crate::diff_eq!(&o1[0..0xc], &o2[0..0xc], "1420a6510 [0..12]");
    crate::diff_eq!(&o1[0x10..0x20], &o2[0x10..0x20], "1420a6510 [16..32]");
    Ok(1)
}

/// Original: `0x1420a65a0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a65a0(rcx: usize, rdx: usize) -> u8 {
    let mask = rd::<u64>(rcx + 0x10);
    let req = rd::<u64>(rdx);
    if (mask & req) != req {
        return 0;
    }
    let excl = rd::<u64>(rdx + 0x8);
    ((excl & mask) == 0) as u8
}

pub unsafe fn diff_d3d_unknown_1420a65a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut a = [0u8; 24];
    let mut b = [0u8; 16];
    let pa = a.as_mut_ptr() as usize;
    let pb = b.as_mut_ptr() as usize;
    wr::<u64>(pa + 0x10, 0b1011);
    wr::<u64>(pb, 0b0011);
    wr::<u64>(pb + 0x8, 0b0100);
    crate::diff_eq!(orig_fn(pa, pb), new_fn(pa, pb), "passes mask");
    wr::<u64>(pb + 0x8, 0b1000);
    crate::diff_eq!(orig_fn(pa, pb), new_fn(pa, pb), "fails excl");
    Ok(2)
}

/// Original: `0x1420a65d0` (112 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a65d0(
    rcx: usize,
    rdx: usize,
    mut r8: u64,
) {
    let mut rax = rd::<usize>(rcx + 0x10);
    let count = rd::<u32>(rcx + 0x18) as usize;
    let end = rax + count * 0x30;
    while rax != end {
        let cur = rd::<u64>(rax + 0x18);
        if cur >= r8 {
            wr::<u64>(rax + 0x18, cur.wrapping_add(rd::<u64>(rdx)));
        }
        rax += 0x30;
    }
    let mut p = rd::<usize>(rdx + 0x8);
    let n = rd::<u64>(rdx) as usize;
    let p_end = p + 2 * n;
    while p != p_end {
        let idx = rd::<u16>(p) as usize;
        p += 2;
        let entry = rd::<usize>(rcx + 0x10) + idx * 0x30;
        wr::<u64>(entry + 0x18, r8);
        r8 = r8.wrapping_add(1);
    }
}

pub unsafe fn diff_d3d_unknown_1420a65d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut items1 = [0u8; 96];
    let mut items2 = [0u8; 96];
    let mut ctx1 = [0u8; 32];
    let mut ctx2 = [0u8; 32];
    let indices: [u16; 1] = [1];
    let mut span = [0u8; 16];
    wr::<u64>(items1.as_mut_ptr() as usize + 0x18, 5);
    wr::<u64>(items1.as_mut_ptr() as usize + 0x30 + 0x18, 2);
    items2.copy_from_slice(&items1);
    wr::<usize>(ctx1.as_mut_ptr() as usize + 0x10, items1.as_mut_ptr() as usize);
    wr::<u32>(ctx1.as_mut_ptr() as usize + 0x18, 2);
    wr::<usize>(ctx2.as_mut_ptr() as usize + 0x10, items2.as_mut_ptr() as usize);
    wr::<u32>(ctx2.as_mut_ptr() as usize + 0x18, 2);
    wr::<u64>(span.as_mut_ptr() as usize, 1);
    wr::<usize>(span.as_mut_ptr() as usize + 0x8, indices.as_ptr() as usize);
    orig_fn(ctx1.as_mut_ptr() as usize, span.as_mut_ptr() as usize, 3);
    new_fn(ctx2.as_mut_ptr() as usize, span.as_mut_ptr() as usize, 3);
    crate::diff_eq!(items1, items2, "renumber items");
    Ok(1)
}

/// Original: `0x1420a6650` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6650(
    rcx: usize,
    rdx: usize,
    r8: u16,
) -> usize {
    let entry = rd::<usize>(rcx + 0x10) + (r8 as usize) * 0x30;
    let val = rd::<u64>(entry + 0x10);
    wr::<u64>(rdx, val);
    rdx
}

pub unsafe fn diff_d3d_unknown_1420a6650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut items = [0u8; 96];
    let mut ctx = [0u8; 32];
    let mut o1 = 0u64;
    let mut o2 = 0u64;
    wr::<u64>(items.as_mut_ptr() as usize + 0x30 + 0x10, 0x1234_5678_9abc_def0);
    wr::<usize>(ctx.as_mut_ptr() as usize + 0x10, items.as_mut_ptr() as usize);
    orig_fn(ctx.as_mut_ptr() as usize, (&mut o1 as *mut u64) as usize, 1);
    new_fn(ctx.as_mut_ptr() as usize, (&mut o2 as *mut u64) as usize, 1);
    crate::diff_eq!(o1, o2, "read +0x10 into out");
    Ok(1)
}

/// Original: `0x1420a6b30` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6b30(rcx: usize, rdx: u16) -> u64 {
    let entry = rd::<usize>(rcx + 0x10) + (rdx as usize) * 0x30;
    rd::<u64>(entry + 0x18)
}

pub unsafe fn diff_d3d_unknown_1420a6b30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u64;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut items = [0u8; 96];
    let mut ctx = [0u8; 32];
    wr::<u64>(items.as_mut_ptr() as usize + 0x30 + 0x18, 0xfeed_face_cafe_babe);
    wr::<usize>(ctx.as_mut_ptr() as usize + 0x10, items.as_mut_ptr() as usize);
    let c_ptr = ctx.as_mut_ptr() as usize;
    crate::diff_eq!(orig_fn(c_ptr, 1), new_fn(c_ptr, 1), "read +0x18");
    Ok(1)
}

/// Original: `0x1420a6b50` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6b50(rcx: usize, rdx: u16, r8: u64) {
    let entry = rd::<usize>(rcx + 0x10) + (rdx as usize) * 0x30;
    let prev = rd::<u64>(entry + 0x10);
    wr::<u64>(entry + 0x10, prev | r8);
}

pub unsafe fn diff_d3d_unknown_1420a6b50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u64);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut i1 = [0u8; 96];
    let mut i2 = [0u8; 96];
    let mut c1 = [0u8; 32];
    let mut c2 = [0u8; 32];
    wr::<u64>(i1.as_mut_ptr() as usize + 0x30 + 0x10, 0x0f0f);
    wr::<u64>(i2.as_mut_ptr() as usize + 0x30 + 0x10, 0x0f0f);
    wr::<usize>(c1.as_mut_ptr() as usize + 0x10, i1.as_mut_ptr() as usize);
    wr::<usize>(c2.as_mut_ptr() as usize + 0x10, i2.as_mut_ptr() as usize);
    orig_fn(c1.as_mut_ptr() as usize, 1, 0xf000);
    new_fn(c2.as_mut_ptr() as usize, 1, 0xf000);
    crate::diff_eq!(i1, i2, "or +0x10");
    Ok(1)
}

/// Original: `0x1420a6c80` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6c80(rcx: usize, rdx: u16, r8: u64) {
    let entry = rd::<usize>(rcx + 0x10) + (rdx as usize) * 0x30;
    wr::<u64>(entry + 0x10, r8);
}

pub unsafe fn diff_d3d_unknown_1420a6c80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u64);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);
    let mut i1 = [0u8; 96];
    let mut i2 = [0u8; 96];
    let mut c1 = [0u8; 32];
    let mut c2 = [0u8; 32];
    wr::<usize>(c1.as_mut_ptr() as usize + 0x10, i1.as_mut_ptr() as usize);
    wr::<usize>(c2.as_mut_ptr() as usize + 0x10, i2.as_mut_ptr() as usize);
    orig_fn(c1.as_mut_ptr() as usize, 1, 0x1234_5678);
    new_fn(c2.as_mut_ptr() as usize, 1, 0x1234_5678);
    crate::diff_eq!(i1, i2, "set +0x10");
    Ok(1)
}

/// Original: `0x1420a6ca0` (225 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420a6ca0(
    rcx: usize,
    rdx: usize,
    r8: usize,
    r9: u64,
    arg5: u16,
    arg6: u16,
) -> usize {
    let rdi = rcx + 0x10;
    let count = rd::<u32>(rdi + 0x8) as usize;
    let cap = rd::<u32>(rdi + 0xc) as usize;
    if count * 0x30 == cap * 0x30 {
        exe::call_va!(0x1_4020_afa0, (rdi, count + 1) -> ());
    }
    let cur = rd::<u32>(rdi + 0x8) as usize;
    let elem = rd::<usize>(rdi) + cur * 0x30;
    wr::<u128>(elem, 0);
    wr::<u128>(elem + 0x10, 0);
    wr::<u128>(elem + 0x20, 0);
    let new_count = (cur as u32).wrapping_add(1);
    wr::<u32>(rdi + 0x8, new_count);
    let end = rd::<usize>(rdi) + (new_count as usize) * 0x30;
    let v0 = rd::<u128>(r8);
    wr::<u128>(end - 0x30, v0);
    wr::<u16>(end - 0xe, arg5);
    wr::<u16>(end - 0xc, arg6);
    wr::<u64>(end - 0x20, r9);
    wr::<u64>(end - 0x18, count as u64);
    wr::<u16>(rdx, count as u16);
    rdx
}

pub unsafe fn diff_d3d_unknown_1420a6ca0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_afa0(rdi: usize, new_cap: usize) {
        wr::<u32>(rdi + 0xc, new_cap as u32);
    }
    let _m = crate::difftest::Mock::new(0x1_4020_afa0, fake_afa0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize, usize, u64, u16, u16) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let v0: u128 = 0x01020304_05060708_090a0b0c_0d0e0f10;
    for (init_count, init_cap) in [(1u32, 3u32), (1u32, 1u32)] {
        let mut elems_o = [0xffu8; 144];
        let mut elems_n = [0xffu8; 144];
        let mut ctx_o = [0u8; 32];
        let mut ctx_n = [0u8; 32];
        wr::<usize>(ctx_o.as_mut_ptr() as usize + 0x10, elems_o.as_mut_ptr() as usize);
        wr::<u32>(ctx_o.as_mut_ptr() as usize + 0x18, init_count);
        wr::<u32>(ctx_o.as_mut_ptr() as usize + 0x1c, init_cap);
        wr::<usize>(ctx_n.as_mut_ptr() as usize + 0x10, elems_n.as_mut_ptr() as usize);
        wr::<u32>(ctx_n.as_mut_ptr() as usize + 0x18, init_count);
        wr::<u32>(ctx_n.as_mut_ptr() as usize + 0x1c, init_cap);

        let mut out_o = 0u16;
        let mut out_n = 0u16;
        let r_o = orig_fn(
            ctx_o.as_mut_ptr() as usize,
            (&mut out_o as *mut u16) as usize,
            (&v0 as *const u128) as usize,
            0xdead_beef_cafe_babe,
            0x1111,
            0x2222,
        );
        let r_n = new_fn(
            ctx_n.as_mut_ptr() as usize,
            (&mut out_n as *mut u16) as usize,
            (&v0 as *const u128) as usize,
            0xdead_beef_cafe_babe,
            0x1111,
            0x2222,
        );

        crate::diff_eq!(r_o == (&out_o as *const u16) as usize, r_n == (&out_n as *const u16) as usize, "1420a6ca0 ret");
        crate::diff_eq!(out_o, out_n, "1420a6ca0 out");
        crate::diff_eq!(&ctx_o[0x18..0x20], &ctx_n[0x18..0x20], "1420a6ca0 vec hdr");
        crate::diff_eq!(elems_o, elems_n, "1420a6ca0 elems");
    }
    Ok(2)
}

// REGISTRY: Replacement::verified(0x1_420a_3740, "d3d::d3d_unknown_1420a3740", rend::d3d_fx_mesh_b::d3d_unknown_1420a3740 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a3740),
// REGISTRY: Replacement::verified(0x1_420a_3770, "d3d::d3d_unknown_1420a3770", rend::d3d_fx_mesh_b::d3d_unknown_1420a3770 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a3770),
// REGISTRY: Replacement::verified(0x1_420a_37e0, "d3d::d3d_unknown_1420a37e0", rend::d3d_fx_mesh_b::d3d_unknown_1420a37e0 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a37e0),
// REGISTRY: Replacement::verified(0x1_420a_3820, "d3d::d3d_unknown_1420a3820", rend::d3d_fx_mesh_b::d3d_unknown_1420a3820 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a3820),
// REGISTRY: Replacement::verified(0x1_420a_3880, "d3d::d3d_unknown_1420a3880", rend::d3d_fx_mesh_b::d3d_unknown_1420a3880 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a3880),
// REGISTRY: Replacement::verified(0x1_420a_38d0, "d3d::d3d_unknown_1420a38d0", rend::d3d_fx_mesh_b::d3d_unknown_1420a38d0 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a38d0),
// REGISTRY: Replacement::verified_throws(0x1_420a_43e0, "d3d::d3d_unknown_1420a43e0", rend::d3d_fx_mesh_b::d3d_unknown_1420a43e0 as *const (), Throws::Discard, rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a43e0),
// REGISTRY: Replacement::verified_throws(0x1_420a_50e0, "d3d::d3d_unknown_1420a50e0", rend::d3d_fx_mesh_b::d3d_unknown_1420a50e0 as *const (), Throws::Discard, rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a50e0),
// REGISTRY: Replacement::verified_throws(0x1_420a_5340, "d3d::d3d_unknown_1420a5340", rend::d3d_fx_mesh_b::d3d_unknown_1420a5340 as *const (), Throws::Discard, rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a5340),
// REGISTRY: Replacement::verified_throws(0x1_420a_59e0, "d3d::d3d_unknown_1420a59e0", rend::d3d_fx_mesh_b::d3d_unknown_1420a59e0 as *const (), Throws::Discard, rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a59e0),
// REGISTRY: Replacement::verified(0x1_420a_5a80, "d3d::d3d_unknown_1420a5a80", rend::d3d_fx_mesh_b::d3d_unknown_1420a5a80 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a5a80),
// REGISTRY: Replacement::verified_throws(0x1_420a_5ac0, "d3d::d3d_unknown_1420a5ac0", rend::d3d_fx_mesh_b::d3d_unknown_1420a5ac0 as *const (), Throws::Discard, rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a5ac0),
// REGISTRY: Replacement::verified(0x1_420a_6310, "d3d::d3d_unknown_1420a6310", rend::d3d_fx_mesh_b::d3d_unknown_1420a6310 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a6310),
// REGISTRY: Replacement::verified(0x1_420a_6510, "d3d::d3d_unknown_1420a6510", rend::d3d_fx_mesh_b::d3d_unknown_1420a6510 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a6510),
// REGISTRY: Replacement::verified(0x1_420a_6ca0, "d3d::d3d_unknown_1420a6ca0", rend::d3d_fx_mesh_b::d3d_unknown_1420a6ca0 as *const (), rend::d3d_fx_mesh_b::diff_d3d_unknown_1420a6ca0),
