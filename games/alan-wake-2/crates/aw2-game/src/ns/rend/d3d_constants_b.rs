use crate::exe::{self, rd, wr};

/// Original: `0x142097880` (12 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097880(rcx: usize) -> u32 {
    let ptr = rd::<usize>(rcx);
    if ptr != 0 {
        rd::<u32>(ptr)
    } else {
        0
    }
}

pub unsafe fn diff_d3d_142097880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut val = [0x1234_5678u32, 0];
    let mut holder_a = [val.as_mut_ptr() as usize];
    let mut holder_b = [val.as_mut_ptr() as usize];
    let r1 = orig_fn(holder_a.as_mut_ptr() as usize);
    let r2 = new_fn(holder_b.as_mut_ptr() as usize);
    crate::diff_eq!(r1, r2, "non-null deref");

    let mut null_a = [0usize];
    let mut null_b = [0usize];
    let r3 = orig_fn(null_a.as_mut_ptr() as usize);
    let r4 = new_fn(null_b.as_mut_ptr() as usize);
    crate::diff_eq!(r3, r4, "null deref");
    Ok(2)
}

/// Original: `0x142097be0` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097be0(rcx: usize) -> i64 {
    let arg5 = rd::<u8>(rcx + 0x8);
    let edx = rd::<u32>(rcx + 0x4);
    let ecx = rd::<u32>(rcx);
    let res = exe::call_va!(0x1_4204_e5e0, (ecx, edx, 1u32, 1u32, arg5) -> i32);
    res as i64
}

/// Original: `0x142097c40` (173 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097c40(rcx: usize, rdx: usize) {
    let rdi = rdx;
    let rbx = rcx;
    let list1 = rd::<usize>(rdi + 0x18);
    if list1 != 0 && list1 != rdi {
        let prev = rd::<usize>(rdi + 0x8);
        let next = rd::<usize>(rdi + 0x10);
        wr::<usize>(prev + 0x10, next);
        let next2 = rd::<usize>(rdi + 0x10);
        let prev2 = rd::<usize>(rdi + 0x8);
        wr::<usize>(next2 + 0x8, prev2);
        wr::<usize>(rdi + 0x8, rdi);
        wr::<usize>(rdi + 0x10, rdi);
        wr::<usize>(rdi + 0x18, 0);
        wr::<u32>(list1 + 0x28, rd::<u32>(list1 + 0x28).wrapping_sub(1));
    }

    let list2 = rd::<usize>(rdi + 0x38);
    let node2 = rdi + 0x20;
    if list2 != 0 && list2 != node2 {
        let prev = rd::<usize>(node2 + 0x8);
        let next = rd::<usize>(node2 + 0x10);
        wr::<usize>(prev + 0x10, next);
        let next2 = rd::<usize>(node2 + 0x10);
        let prev2 = rd::<usize>(node2 + 0x8);
        wr::<usize>(next2 + 0x8, prev2);
        wr::<usize>(node2 + 0x8, node2);
        wr::<usize>(node2 + 0x10, node2);
        wr::<usize>(node2 + 0x18, 0);
        wr::<u32>(list2 + 0x28, rd::<u32>(list2 + 0x28).wrapping_sub(1));
    }

    wr::<usize>(rdi + 0x58, 0);
    wr::<usize>(rdi + 0x60, 0);
    exe::call_va!(0x1_411e_32b0, (node2) -> ());
    exe::call_va!(0x1_411e_32b0, (rdi) -> ());
    exe::call_va!(0x1_4243_b3e0, (rbx + 0x20, rdi) -> ());
}

/// Original: `0x142097d00` (144 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097d00(rcx: usize) {
    let rdi = rcx;
    let rbx = rcx + 0x60;
    while rd::<u32>(rdi + 0x88) != 0 {
        let owner = rd::<usize>(rbx + 0x18);
        let entry_ptr = if owner != 0 && owner != rbx {
            exe::call_va!(0x1_411e_3360, (owner) -> usize)
        } else {
            rd::<usize>(rbx + 0x10)
        };
        let node = rd::<usize>(entry_ptr);
        exe::call_va!(0x1_4209_7c40, (rdi, node) -> ());
    }
    exe::call_va!(0x1_4022_7190, (rdi + 0x460) -> ());
    let dtor = exe::ptr::<u8>(0x1_411e_3350) as usize;
    exe::call_va!(0x1_4183_4ee8, (rdi + 0xa0, 0x40usize, 0xfusize, dtor) -> ());
    exe::call_va!(0x1_411e_32b0, (rbx) -> ());
    exe::call_va!(0x1_4243_b540, (rdi + 0x20) -> ());
}

/// Original: `0x142097d90` (37 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097d90(rcx: usize) {
    let rbx = rcx + 0x8;
    exe::call_va!(0x1_4209_89b0, (rbx) -> ());
    wr::<u64>(rbx, 0);
    wr::<u64>(rbx + 0x8, 0);
    wr::<u64>(rbx + 0x10, 0);
}

/// Original: `0x1420981e0` (31 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420981e0(rcx: usize) -> usize {
    let count_minus_1 = rd::<u32>(rcx + 0x10).wrapping_sub(1) as u64;
    let head = rd::<u32>(rcx + 0x8) as u64;
    let cap = rd::<u32>(rcx + 0x14) as u64;
    let rem = count_minus_1.wrapping_add(head) % cap;
    rd::<usize>(rcx).wrapping_add((rem << 4) as usize)
}

pub unsafe fn diff_d3d_1420981e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        len: u32,
        cap: u32,
    }

    let mut a = Ring { base: 0x1000_0000, head: 3, tail: 0, len: 5, cap: 6 };
    let mut b = a;
    let r1 = orig_fn(&mut a as *mut Ring as usize);
    let r2 = new_fn(&mut b as *mut Ring as usize);
    crate::diff_eq!(r1, r2, "ring back ptr");
    Ok(1)
}

/// Original: `0x142098750` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142098750(rcx: usize) -> usize {
    exe::call_va!(0x1_4209_89b0, (rcx) -> ());
    wr::<u64>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    wr::<u64>(rcx + 0x10, 0);
    rcx
}

/// Original: `0x1420988e0` (45 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420988e0(rcx: usize) {
    if rcx != 0 {
        exe::call_va!(0x1_4209_89b0, (rcx) -> ());
        wr::<u64>(rcx, 0);
        wr::<u64>(rcx + 0x8, 0);
        wr::<u64>(rcx + 0x10, 0);
        exe::call_va!(0x1_4244_2de0, (rcx) -> ());
    }
}

/// Original: `0x142098930` (83 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142098930(rcx: usize, rdx: usize) -> usize {
    if rcx != rdx {
        exe::call_va!(0x1_4209_89b0, (rcx) -> ());
        wr::<u64>(rcx, rd::<u64>(rdx));
        wr::<u32>(rcx + 0x8, rd::<u32>(rdx + 0x8));
        wr::<u32>(rcx + 0xc, rd::<u32>(rdx + 0xc));
        wr::<u32>(rcx + 0x10, rd::<u32>(rdx + 0x10));
        wr::<u32>(rcx + 0x14, rd::<u32>(rdx + 0x14));
        wr::<u64>(rdx, 0);
        wr::<u64>(rdx + 0x8, 0);
        wr::<u64>(rdx + 0x10, 0);
    }
    rcx
}

/// Original: `0x142098990` (26 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142098990(rcx: usize, rdx: u64) -> usize {
    let head = rd::<u32>(rcx + 0x8) as u64;
    let cap = rd::<u32>(rcx + 0x14) as u64;
    let rem = head.wrapping_add(rdx) % cap;
    rd::<usize>(rcx).wrapping_add((rem << 4) as usize)
}

pub unsafe fn diff_d3d_142098990(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        len: u32,
        cap: u32,
    }

    let mut a = Ring { base: 0x2000_0000, head: 4, tail: 0, len: 5, cap: 8 };
    let mut b = a;
    let r1 = orig_fn(&mut a as *mut Ring as usize, 7);
    let r2 = new_fn(&mut b as *mut Ring as usize, 7);
    crate::diff_eq!(r1, r2, "ring index ptr");
    Ok(1)
}

/// Original: `0x142098a90` (190 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142098a90(rcx: usize) {
    let rbx = rcx;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        wr::<u32>(rbx + 0x8, tid);
    }
    wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

    if rd::<u32>(rbx + 0x20) != 0 {
        loop {
            let buf = rd::<usize>(rbx + 0x10);
            let head = rd::<u32>(rbx + 0x18) as u64;
            let cap = rd::<u32>(rbx + 0x24) as u64;
            let rem = (head % cap) as usize;
            let rdi = rd::<usize>(buf + rem * 8);
            if rdi != 0 {
                exe::call_va!(0x1_4209_89b0, (rdi) -> ());
                wr::<u64>(rdi, 0);
                wr::<u64>(rdi + 0x8, 0);
                wr::<u64>(rdi + 0x10, 0);
                exe::call_va!(0x1_4244_2de0, (rdi) -> ());
            }
            let mut count = rd::<u32>(rbx + 0x20);
            if count != 0 {
                let next_head = rd::<u32>(rbx + 0x18).wrapping_add(1);
                wr::<u32>(rbx + 0x18, next_head);
                if next_head == rd::<u32>(rbx + 0x24) {
                    wr::<u32>(rbx + 0x18, 0);
                }
                count = count.wrapping_sub(1);
                wr::<u32>(rbx + 0x20, count);
            }
            wr::<u64>(rbx + 0x28, rd::<u64>(rbx + 0x28).wrapping_sub(1));
            if count == 0 {
                break;
            }
        }
    }

    let depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
    wr::<u32>(rbx + 0xc, depth);
    if depth == 0 {
        wr::<u32>(rbx + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
}

/// Original: `0x142098f10` (203 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142098f10(rcx: usize) {
    let rbx = rcx;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        wr::<u32>(rbx + 0x8, tid);
    }
    wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

    if rd::<u32>(rbx + 0x20) != 0 {
        loop {
            let buf = rd::<usize>(rbx + 0x10);
            let head = rd::<u32>(rbx + 0x18) as u64;
            let cap = rd::<u32>(rbx + 0x24) as u64;
            let rem = (head % cap) as usize;
            let rdi = rd::<usize>(buf + rem * 8);
            if rdi != 0 {
                exe::call_va!(0x1_4209_89b0, (rdi) -> ());
                wr::<u64>(rdi, 0);
                wr::<u64>(rdi + 0x8, 0);
                wr::<u64>(rdi + 0x10, 0);
                exe::call_va!(0x1_4244_2de0, (rdi) -> ());
            }
            let mut count = rd::<u32>(rbx + 0x20);
            if count != 0 {
                let next_head = rd::<u32>(rbx + 0x18).wrapping_add(1);
                wr::<u32>(rbx + 0x18, next_head);
                if next_head == rd::<u32>(rbx + 0x24) {
                    wr::<u32>(rbx + 0x18, 0);
                }
                count = count.wrapping_sub(1);
                wr::<u32>(rbx + 0x20, count);
            }
            wr::<u64>(rbx + 0x28, rd::<u64>(rbx + 0x28).wrapping_sub(1));
            if count == 0 {
                break;
            }
        }
    }

    let depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
    wr::<u32>(rbx + 0xc, depth);
    if depth == 0 {
        wr::<u32>(rbx + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
    exe::call_va!(0x1_4209_91d0, (rbx + 0x10) -> ());
}

/// Original: `0x142099350` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099350(rcx: usize, rdx: usize) -> usize {
    wr::<u64>(rcx, rd::<u64>(rdx));
    wr::<u32>(rcx + 0x8, rd::<u32>(rdx + 0x8));
    rcx
}

pub unsafe fn diff_d3d_142099350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let src = [0x1122_3344_5566_7788u64, 0x99aa_bbcc_ddeeff00u64];
    let mut dst_a = [0u64; 2];
    let mut dst_b = [0u64; 2];
    let r1 = orig_fn(dst_a.as_mut_ptr() as usize, src.as_ptr() as usize);
    let r2 = new_fn(dst_b.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(r1, dst_a.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r2, dst_b.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(dst_a, dst_b, "copied 12 bytes");
    Ok(1)
}

/// Original: `0x142099410` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099410(rcx: usize, rdx: usize) -> usize {
    wr::<u32>(rcx + 0x8, rd::<u32>(rcx + 0x8).wrapping_add(1));
    wr::<u128>(rdx, rd::<u128>(rcx));
    rdx
}

pub unsafe fn diff_d3d_142099410(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut src_a = [0x1122_3344_5566_7788u64, 0x0000_0005_0000_0009u64];
    let mut src_b = src_a;
    let mut dst_a = [0u64; 2];
    let mut dst_b = [0u64; 2];
    let r1 = orig_fn(src_a.as_mut_ptr() as usize, dst_a.as_mut_ptr() as usize);
    let r2 = new_fn(src_b.as_mut_ptr() as usize, dst_b.as_mut_ptr() as usize);
    crate::diff_eq!(r1, dst_a.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r2, dst_b.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(src_a, src_b, "mutated src");
    crate::diff_eq!(dst_a, dst_b, "copied 16 bytes");
    Ok(1)
}

/// Original: `0x142099430` (45 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099430(rcx: usize, rdx: usize) -> u8 {
    let a0 = rd::<u64>(rcx);
    let b0 = rd::<u64>(rdx);
    if a0 != b0 {
        return 1;
    }
    let a2 = rd::<u32>(rcx + 0x8);
    let b2 = rd::<u32>(rdx + 0x8);
    if a2 != b2 {
        1
    } else {
        0
    }
}

pub unsafe fn diff_d3d_142099430(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let a = [0x1111_2222_3333_4444u64, 0xaaaa_bbbb_0000_0007u64];
    let b = [0x1111_2222_3333_4444u64, 0xcccc_dddd_0000_0007u64];
    let c = [0x1111_2222_3333_4444u64, 0xcccc_dddd_0000_0008u64];

    let r1 = orig_fn(a.as_ptr() as usize, b.as_ptr() as usize);
    let r2 = new_fn(a.as_ptr() as usize, b.as_ptr() as usize);
    crate::diff_eq!(r1, r2, "equal first 12 bytes");

    let r3 = orig_fn(a.as_ptr() as usize, c.as_ptr() as usize);
    let r4 = new_fn(a.as_ptr() as usize, c.as_ptr() as usize);
    crate::diff_eq!(r3, r4, "differ at +8");
    Ok(2)
}

/// Original: `0x1420994f0` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420994f0(rcx: usize) {
    let len = rd::<u32>(rcx + 0x10);
    let mut i = 0u32;
    while i != len {
        let cap = rd::<u32>(rcx + 0x14) as u64;
        let head = rd::<u32>(rcx + 0x8) as u64;
        let rem = head.wrapping_add(i as u64) % cap;
        let entry = rd::<usize>(rcx).wrapping_add((rem << 4) as usize);
        let arg = rd::<usize>(entry);
        let func = rd::<usize>(entry + 0x8);
        exe::call_va!(func, (arg) -> ());
        i = i.wrapping_add(1);
    }
}

/// Original: `0x142099560` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099560(rcx: usize) -> u8 {
    let inner = rd::<usize>(rcx);
    let fence = rd::<usize>(inner + 0x8);
    let target = rd::<u64>(inner);
    if fence == 0 {
        return 0;
    }
    let vtbl = rd::<usize>(fence);
    let get_completed = rd::<usize>(vtbl + 0x40);
    let completed = exe::call_va!(get_completed, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (fence, 0x887a_0005u32) -> ());
    }
    (completed < target) as u8
}

/// Original: `0x1420996d0` (22 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420996d0(ecx: u32) -> usize {
    if ecx == 1 {
        exe::ptr::<u8>(0x1_4385_8500) as usize
    } else {
        exe::ptr::<u8>(0x1_4385_8490) as usize
    }
}

pub unsafe fn diff_d3d_1420996d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for v in [0u32, 1, 2, u32::MAX] {
        crate::diff_eq!(orig_fn(v), new_fn(v), "select global queue");
    }
    Ok(4)
}

/// Original: `0x142099780` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099780(rcx: usize) -> usize {
    wr::<u32>(rcx + 0x8, u32::MAX);
    wr::<u32>(rcx + 0xc, 0);
    wr::<u64>(rcx, 0);
    wr::<u32>(rcx + 0x24, 0);
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x28, 0);
    wr::<u64>(rcx + 0x30, 0);
    wr::<u64>(rcx + 0x38, 0);
    wr::<u64>(rcx + 0x40, 0);
    wr::<u64>(rcx + 0x48, 0);
    wr::<u64>(rcx + 0x50, 0);
    wr::<u64>(rcx + 0x58, 0);
    wr::<u64>(rcx + 0x60, 0);
    wr::<u32>(rcx + 0x68, 0);
    wr::<u8>(rcx + 0x6c, 0);
    wr::<u32>(rcx + 0x20, u32::MAX);
    rcx
}

pub unsafe fn diff_d3d_142099780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut a = [0xaa_u8; 0x70];
    let mut b = [0xaa_u8; 0x70];
    let r1 = orig_fn(a.as_mut_ptr() as usize);
    let r2 = new_fn(b.as_mut_ptr() as usize);
    crate::diff_eq!(r1, a.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(r2, b.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(a, b, "init 0x70 struct");
    Ok(1)
}

/// Original: `0x1420997e0` (79 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420997e0(rcx: usize) -> u8 {
    let target = rd::<u64>(rcx + 0x18);
    let fence = rd::<usize>(rcx + 0x20);
    if fence == 0 {
        return 0;
    }
    let vtbl = rd::<usize>(fence);
    let get_completed = rd::<usize>(vtbl + 0x40);
    let completed = exe::call_va!(get_completed, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (fence, 0x887a_0005u32) -> ());
    }
    (completed < target) as u8
}

/// Original: `0x142099a80` (49 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099a80(cl: u8) -> u32 {
    let a = exe::call_va!(0x1_4209_9c20, (cl, 1u32) -> u32);
    let b = exe::call_va!(0x1_4209_9c20, (cl, 0u32) -> u32);
    a.wrapping_add(b)
}

/// Original: `0x142099ac0` (25 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099ac0() {
    exe::call_va!(0x1_4209_95c0, (0u32) -> ());
    exe::call_va!(0x1_4209_95c0, (1u32) -> ());
}

/// Original: `0x14209a110` (187 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a110(rcx: usize, rdx: usize, r8d: u32) {
    let rbx = if r8d == 1 {
        exe::ptr::<u8>(0x1_4385_8500) as usize
    } else {
        exe::ptr::<u8>(0x1_4385_8490) as usize
    };
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        wr::<u32>(rbx + 0x8, tid);
    }
    wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

    let rdi = rd::<usize>(rbx + 0x10);
    let len = rd::<u32>(rdi + 0x10);
    if len == rd::<u32>(rdi + 0x14) {
        exe::call_va!(0x1_4209_8270, (rdi, len.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(rdi + 0xc) as usize;
    let slot = rd::<usize>(rdi).wrapping_add(tail << 4);
    wr::<usize>(slot, rcx);
    wr::<usize>(slot + 0x8, rdx);
    let next_tail = rd::<u32>(rdi + 0xc).wrapping_add(1);
    wr::<u32>(rdi + 0xc, next_tail);
    if next_tail == rd::<u32>(rdi + 0x14) {
        wr::<u32>(rdi + 0xc, 0);
    }
    wr::<u32>(rdi + 0x10, rd::<u32>(rdi + 0x10).wrapping_add(1));

    let depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
    wr::<u32>(rbx + 0xc, depth);
    if depth == 0 {
        wr::<u32>(rbx + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
}

/// Original: `0x14209a1e0` (180 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a1e0(rcx: usize, edx: u32) {
    let rbx = if edx == 1 {
        exe::ptr::<u8>(0x1_4385_8500) as usize
    } else {
        exe::ptr::<u8>(0x1_4385_8490) as usize
    };
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        wr::<u32>(rbx + 0x8, tid);
    }
    wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

    let rdi = rd::<usize>(rbx + 0x10);
    let len = rd::<u32>(rdi + 0x10);
    if len == rd::<u32>(rdi + 0x14) {
        exe::call_va!(0x1_4209_8270, (rdi, len.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(rdi + 0xc) as usize;
    let slot = rd::<usize>(rdi).wrapping_add(tail << 4);
    wr::<usize>(slot, rcx);
    let cb = exe::ptr::<u8>(0x1_4209_97d0) as usize;
    wr::<usize>(slot + 0x8, cb);
    let next_tail = rd::<u32>(rdi + 0xc).wrapping_add(1);
    wr::<u32>(rdi + 0xc, next_tail);
    if next_tail == rd::<u32>(rdi + 0x14) {
        wr::<u32>(rdi + 0xc, 0);
    }
    wr::<u32>(rdi + 0x10, rd::<u32>(rdi + 0x10).wrapping_add(1));

    let depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
    wr::<u32>(rbx + 0xc, depth);
    if depth == 0 {
        wr::<u32>(rbx + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
}

/// Original: `0x14209a2a0` (116 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a2a0(_rcx: usize) {
    let flag = rd::<u8>(exe::ptr::<u8>(0x1_4385_8580) as usize);
    if flag != 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x463e0, 0i32, 0x400usize) -> usize);
        let len = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x63d0, 0i32, len) -> usize);
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x47460, -1i32, 0x4000usize) -> usize);
    }
}

/// Original: `0x14209a430` (158 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a430(ecx: u32, rdx: usize) -> u8 {
    let head = rd::<usize>(exe::ptr::<usize>(0x1_43a8_77b0) as usize);
    let mut curr = rd::<usize>(head);
    while curr != head {
        if rd::<u32>(curr + 0xa4) == ecx {
            exe::call_va!(0x1_4240_aa00, (rdx, curr + 0x1c) -> ());
            return 1;
        }
        let right = rd::<usize>(curr + 0x10);
        if rd::<u8>(right + 0x19) != 0 {
            let mut parent = rd::<usize>(curr + 0x8);
            while rd::<u8>(parent + 0x19) == 0 && curr == rd::<usize>(parent + 0x10) {
                curr = parent;
                parent = rd::<usize>(parent + 0x8);
            }
            curr = parent;
        } else {
            let mut next = right;
            let mut left = rd::<usize>(next);
            while rd::<u8>(left + 0x19) == 0 {
                next = left;
                left = rd::<usize>(left);
            }
            curr = next;
        }
    }
    0
}

/// Original: `0x14209a500` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a500(ecx: i32) -> u32 {
    let base = exe::ptr::<u8>(0x1_43a6_37bc) as usize;
    let idx = (ecx as isize) * 16;
    rd::<u32>(base.wrapping_add(idx as usize))
}

pub unsafe fn diff_d3d_14209a500(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32) -> u32;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    for idx in [0i32, 1, 2, 7] {
        crate::diff_eq!(orig_fn(idx), new_fn(idx), "table lookup +0xc");
    }
    Ok(4)
}

/// Original: `0x14209a6e0` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a6e0(ecx: i32, edx: i32) -> u8 {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let src_addr = (tls0 + 0x463e0).wrapping_add(((ecx as isize) * 4) as usize);
    let dst_addr = (tls0 + 0x47460).wrapping_add(((edx as isize) * 4) as usize);
    let val = rd::<u32>(src_addr);
    let prev = rd::<u32>(dst_addr);
    wr::<u32>(dst_addr, val);
    (val != prev) as u8
}

/// Original: `0x14209a720` (63 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a720(ecx: i32, edx: u32, r8: u64) {
    let base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry = base.wrapping_add(((ecx as isize) * 16) as usize);
    let cur_type = rd::<i32>(entry + 0x4);
    if cur_type != -1 {
        let cur_size = rd::<i32>(entry + 0x8) as i64 as u64;
        if cur_size >= r8 {
            return;
        }
    }
    let total_ptr = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let total = rd::<u32>(total_ptr);
    wr::<u32>(entry, total);
    wr::<u32>(total_ptr, total.wrapping_add(r8 as u32));
    wr::<u32>(entry + 0x4, edx);
    wr::<u32>(entry + 0x8, r8 as u32);
}

/// Original: `0x14209a770` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a770(ecx: i32, edx: u32) {
    let base = exe::ptr::<u8>(0x1_4385_8590) as usize;
    let addr = base.wrapping_add(((ecx as isize) * 4) as usize);
    wr::<u32>(addr, edx);
}

pub unsafe fn diff_d3d_14209a770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, u32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let slot = exe::ptr::<u32>(0x1_4385_8590) as usize;
    let saved = rd::<u32>(slot);
    orig_fn(0, 0x1357_2468);
    let after_orig = rd::<u32>(slot);
    wr::<u32>(slot, saved);
    new_fn(0, 0x1357_2468);
    let after_new = rd::<u32>(slot);
    wr::<u32>(slot, saved);
    crate::diff_eq!(after_orig, after_new, "write global slot 0");
    Ok(1)
}

/// Original: `0x14209a790` (88 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a790(ecx: i32, edx: i32) {
    let base = exe::ptr::<u8>(0x1_43a6_37bc) as usize;
    let slot_ptr = base.wrapping_add(((ecx as isize) * 16) as usize);
    let cur = rd::<i32>(slot_ptr);
    if cur == -1 || cur == edx {
        wr::<i32>(slot_ptr, edx);
        return;
    }
    let mut ecx_slot = ecx;
    let mut edx_slot = edx;
    let mut msg_buf = [0u8; 32];
    let fmt_ptr = exe::call_va!(
        0x1_411e_3700,
        (
            msg_buf.as_mut_ptr() as usize,
            &mut ecx_slot as *mut i32 as usize,
            slot_ptr,
            &mut edx_slot as *mut i32 as usize
        ) -> usize
    );
    let mut err_buf = [0u8; 32];
    let err_ptr = exe::call_va!(0x1_4205_cde0, (err_buf.as_mut_ptr() as usize, fmt_ptr) -> usize);
    exe::call_va!(0x1_411e_3720, (err_ptr) -> ());
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("int3", options(noreturn));
    #[cfg(not(target_arch = "x86_64"))]
    std::process::abort();
}

/// Original: `0x14209a7f0` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a7f0(ecx: i32) -> usize {
    let base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let off = rd::<i32>(base.wrapping_add(((ecx as isize) * 16) as usize)) as isize;
    let tls0 = rd::<usize>(exe::tls_pointer());
    tls0.wrapping_add(0x63d0).wrapping_add(off as usize)
}

/// Original: `0x14209a820` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a820(ecx: i32) {
    if ecx != -1 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let addr = (tls0 + 0x463e0).wrapping_add(((ecx as isize) * 4) as usize);
        let next = rd::<u32>(addr).wrapping_add(1);
        wr::<u32>(addr, next);
        if next == u32::MAX {
            wr::<u32>(addr, 0);
        }
    }
}

/// Original: `0x14209a870` (75 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a870(ecx: i32) {
    let base = exe::ptr::<u8>(0x1_43a6_37bc) as usize;
    let slot = rd::<i32>(base.wrapping_add(((ecx as isize) * 16) as usize));
    if slot != -1 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let addr = (tls0 + 0x463e0).wrapping_add(((slot as isize) * 4) as usize);
        let next = rd::<u32>(addr).wrapping_add(1);
        wr::<u32>(addr, next);
        if next == u32::MAX {
            wr::<u32>(addr, 0);
        }
    }
}

/// Original: `0x14209a8d0` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a8d0(ecx: i32, rdx: usize) {
    let base = exe::ptr::<u8>(0x1_43a6_37b8) as usize;
    let size = rd::<i32>(base.wrapping_add(((ecx as isize) * 16) as usize)) as isize as usize;
    exe::call_va!(0x1_4209_a900, (ecx, 0usize, size, rdx) -> ());
}

/// Original: `0x14209a900` (134 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a900(ecx: i32, rdx: usize, r8: usize, r9: usize) {
    let base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry = base.wrapping_add(((ecx as isize) * 16) as usize);
    let off = rd::<i32>(entry);
    if off >= 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let dst = tls0
            .wrapping_add(0x63d0)
            .wrapping_add(off as usize)
            .wrapping_add(rdx);
        exe::call_va!(0x1_4183_acbc, (dst, r9, r8) -> usize);
        let slot = rd::<i32>(entry + 0xc);
        if slot != -1 {
            let addr = (tls0 + 0x463e0).wrapping_add(((slot as isize) * 4) as usize);
            let next = rd::<u32>(addr).wrapping_add(1);
            wr::<u32>(addr, next);
            if next == u32::MAX {
                wr::<u32>(addr, 0);
            }
        }
    }
}

/// Original: `0x14209a990` (182 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209a990(ecx: i32, rdx: usize) {
    let base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry = base.wrapping_add(((ecx as isize) * 16) as usize);
    let off = rd::<i32>(entry);
    if off >= 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let size = rd::<i32>(entry + 0x8) as isize as usize;
        let buf = tls0.wrapping_add(0x63d0).wrapping_add(off as usize);
        let cmp = exe::call_va!(0x1_4183_acda, (buf, rdx, size) -> i32);
        if cmp == 0 {
            exe::call_va!(0x1_4183_acc2, (buf, 0i32, size) -> usize);
        }
        let slot = rd::<i32>(entry + 0xc);
        if slot != -1 {
            let addr = (tls0 + 0x463e0).wrapping_add(((slot as isize) * 4) as usize);
            let next = rd::<u32>(addr).wrapping_add(1);
            wr::<u32>(addr, next);
            if next == u32::MAX {
                wr::<u32>(addr, 0);
            }
        }
    }
}

/// Original: `0x14209aa70` (132 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209aa70(rcx: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    exe::call_va!(0x1_4183_acc2, (tls0 + 0x463e0, 0i32, 0x400usize) -> usize);
    let len = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
    let dst = tls0 + 0x63d0;
    if rcx != 0 {
        exe::call_va!(0x1_4183_acbc, (dst, rcx + 0x400, len) -> usize);
    } else {
        exe::call_va!(0x1_4183_acc2, (dst, 0i32, len) -> usize);
    }
    exe::call_va!(0x1_4183_acc2, (tls0 + 0x47460, -1i32, 0x4000usize) -> usize);
}

/// Original: `0x14209ab00` (207 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209ab00(rcx: usize) {
    let total_ptr = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let mut r8 = rd::<u32>(total_ptr) as usize;
    let cap = rd::<u32>(rcx + 0x8) as usize;
    let needed = r8 + 0x400;
    if cap < needed {
        exe::call_va!(0x1_4028_e490, (rcx, needed) -> ());
        r8 = rd::<u32>(total_ptr) as usize;
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    let mut dst = rd::<usize>(rcx);
    let mut src = tls0 + 0x463e0;
    for _ in 0..8 {
        wr::<u128>(dst, rd::<u128>(src));
        wr::<u128>(dst + 0x10, rd::<u128>(src + 0x10));
        wr::<u128>(dst + 0x20, rd::<u128>(src + 0x20));
        wr::<u128>(dst + 0x30, rd::<u128>(src + 0x30));
        wr::<u128>(dst + 0x40, rd::<u128>(src + 0x40));
        wr::<u128>(dst + 0x50, rd::<u128>(src + 0x50));
        wr::<u128>(dst + 0x60, rd::<u128>(src + 0x60));
        wr::<u128>(dst + 0x70, rd::<u128>(src + 0x70));
        dst += 0x80;
        src += 0x80;
    }
    let dst_data = rd::<usize>(rcx) + 0x400;
    let src_data = tls0 + 0x63d0;
    exe::call_va!(0x1_4183_acbc, (dst_data, src_data, r8) -> usize);
}

/// Original: `0x14209abe0` (63 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209abe0(rcx: usize) -> usize {
    exe::call_va!(0x1_4205_6ed0, (rcx) -> ());
    let vtbl = exe::ptr::<u8>(0x1_433d_d578) as usize;
    wr::<usize>(rcx, vtbl);
    let tls0 = rd::<usize>(exe::tls_pointer());
    exe::call_va!(0x1_4183_acc2, (tls0 + 0x63d0, 0i32, 0x40000usize) -> usize);
    rcx
}

pub unsafe fn diff_d3d_142097be0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> i64;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut cases = 0u32;
    for (w, h, fmt) in [(64u32, 64u32, 0u8), (128, 256, 1), (16, 16, 2)] {
        let mut desc = [0u8; 16];
        wr::<u32>(desc.as_mut_ptr() as usize, w);
        wr::<u32>(desc.as_mut_ptr() as usize + 4, h);
        wr::<u8>(desc.as_mut_ptr() as usize + 8, fmt);
        let o = orig_fn(desc.as_ptr() as usize);
        let n = new_fn(desc.as_ptr() as usize);
        crate::diff_eq!(o, n, "d3d_142097be0");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_142098930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut ring_o = [0x11u8; 0x18];
    let mut ring_n = [0x11u8; 0x18];
    let po = ring_o.as_mut_ptr() as usize;
    let pn = ring_n.as_mut_ptr() as usize;
    let ro = orig_fn(po, po);
    let rn = new_fn(pn, pn);
    crate::diff_eq!(ro, po, "ret orig");
    crate::diff_eq!(rn, pn, "ret new");
    crate::diff_eq!(ring_o, ring_n, "self assign");
    Ok(1)
}

pub unsafe fn diff_d3d_142098a90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let mut obj_o = [0u8; 0x30];
    let mut obj_n = [0u8; 0x30];
    for obj in [&mut obj_o, &mut obj_n] {
        let b = obj.as_mut_ptr() as usize;
        wr::<u32>(b + 0x08, tid);
        wr::<u32>(b + 0x0c, 1);
        wr::<u32>(b + 0x20, 0);
    }
    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj_o, obj_n, "d3d_142098a90 empty ring");
    Ok(1)
}

pub unsafe fn diff_d3d_1420994f0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_ring_cb(arg: usize) {
        wr::<u64>(arg, rd::<u64>(arg).wrapping_add(0x101));
    }

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut acc_o = [10u64, 20u64];
    let mut acc_n = [10u64, 20u64];

    let entries_o: [[usize; 2]; 4] = [
        [0, 0],
        [
            core::ptr::addr_of_mut!(acc_o[0]) as usize,
            fake_ring_cb as *const () as usize,
        ],
        [
            core::ptr::addr_of_mut!(acc_o[1]) as usize,
            fake_ring_cb as *const () as usize,
        ],
        [0, 0],
    ];
    let entries_n: [[usize; 2]; 4] = [
        [0, 0],
        [
            core::ptr::addr_of_mut!(acc_n[0]) as usize,
            fake_ring_cb as *const () as usize,
        ],
        [
            core::ptr::addr_of_mut!(acc_n[1]) as usize,
            fake_ring_cb as *const () as usize,
        ],
        [0, 0],
    ];

    let mut ring_o = [0u8; 0x18];
    let mut ring_n = [0u8; 0x18];
    for (ring, entries) in [(&mut ring_o, &entries_o), (&mut ring_n, &entries_n)] {
        let b = ring.as_mut_ptr() as usize;
        wr::<usize>(b, entries.as_ptr() as usize);
        wr::<u32>(b + 0x08, 1); // head
        wr::<u32>(b + 0x10, 2); // len
        wr::<u32>(b + 0x14, 4); // cap
    }

    orig_fn(ring_o.as_mut_ptr() as usize);
    new_fn(ring_n.as_mut_ptr() as usize);
    crate::diff_eq!(acc_o, acc_n, "d3d_1420994f0 callbacks");
    Ok(1)
}

unsafe extern "win64" fn fake_fence_get_completed(fence: usize) -> u64 {
    rd::<u64>(fence + 8)
}

pub unsafe fn diff_d3d_142099560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 10];
    vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;
    let mut fence_obj = [vtbl.as_ptr() as u64, 0u64];

    let mut inner = [0u64; 2];
    let holder = [inner.as_mut_ptr() as usize];
    let mut cases = 0u32;

    // 1. Null fence
    let o0 = orig_fn(holder.as_ptr() as usize);
    let n0 = new_fn(holder.as_ptr() as usize);
    crate::diff_eq!(o0, n0, "null fence");
    cases += 1;

    // 2. Non-null fence
    inner[1] = fence_obj.as_mut_ptr() as u64;
    for (target, completed) in [(10u64, 5u64), (10, 10), (10, 15)] {
        inner[0] = target;
        fence_obj[1] = completed;
        let o = orig_fn(holder.as_ptr() as usize);
        let n = new_fn(holder.as_ptr() as usize);
        crate::diff_eq!(o, n, "fence compare");
        cases += 1;
    }

    Ok(cases)
}

pub unsafe fn diff_d3d_1420997e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 10];
    vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;
    let mut fence_obj = [vtbl.as_ptr() as u64, 0u64];

    let mut obj = [0u8; 0x30];
    let base = obj.as_mut_ptr() as usize;
    let mut cases = 0u32;

    // 1. Null fence
    let o0 = orig_fn(base);
    let n0 = new_fn(base);
    crate::diff_eq!(o0, n0, "null fence");
    cases += 1;

    // 2. Non-null fence
    wr::<usize>(base + 0x20, fence_obj.as_mut_ptr() as usize);
    for (target, completed) in [(10u64, 5u64), (10, 10), (10, 15)] {
        wr::<u64>(base + 0x18, target);
        fence_obj[1] = completed;
        let o = orig_fn(base);
        let n = new_fn(base);
        crate::diff_eq!(o, n, "fence compare");
        cases += 1;
    }

    Ok(cases)
}

pub unsafe fn diff_d3d_14209a110(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let mut cases = 0u32;

    for r8d in [0u32, 1u32] {
        let rbx = if r8d == 1 {
            exe::ptr::<u8>(0x1_4385_8500) as usize
        } else {
            exe::ptr::<u8>(0x1_4385_8490) as usize
        };
        let mut saved_rbx = [0u8; 0x18];
        for i in 0..0x18 {
            saved_rbx[i] = rd::<u8>(rbx + i);
        }

        let mut buf_o = [[0usize; 2]; 4];
        let mut buf_n = [[0usize; 2]; 4];
        let mut ring_o = [0u8; 0x18];
        let mut ring_n = [0u8; 0x18];
        for (ring, buf) in [(&mut ring_o, &mut buf_o), (&mut ring_n, &mut buf_n)] {
            let b = ring.as_mut_ptr() as usize;
            wr::<usize>(b, buf.as_mut_ptr() as usize);
            wr::<u32>(b + 0x0c, 3); // tail (wraps to 0 when cap=4)
            wr::<u32>(b + 0x10, 1); // len
            wr::<u32>(b + 0x14, 4); // cap
        }

        wr::<u32>(rbx + 0x08, tid);
        wr::<u32>(rbx + 0x0c, 1);
        wr::<usize>(rbx + 0x10, ring_o.as_mut_ptr() as usize);
        orig_fn(0x1111_2222, 0x3333_4444, r8d);

        wr::<u32>(rbx + 0x08, tid);
        wr::<u32>(rbx + 0x0c, 1);
        wr::<usize>(rbx + 0x10, ring_n.as_mut_ptr() as usize);
        new_fn(0x1111_2222, 0x3333_4444, r8d);

        for i in 0..0x18 {
            wr::<u8>(rbx + i, saved_rbx[i]);
        }

        crate::diff_eq!(buf_o, buf_n, "d3d_14209a110 buf");
        crate::diff_eq!(&ring_o[0x08..0x18], &ring_n[0x08..0x18], "d3d_14209a110 ring");
        cases += 1;
    }

    Ok(cases)
}

pub unsafe fn diff_d3d_14209a1e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let mut cases = 0u32;

    for edx in [0u32, 1u32] {
        let rbx = if edx == 1 {
            exe::ptr::<u8>(0x1_4385_8500) as usize
        } else {
            exe::ptr::<u8>(0x1_4385_8490) as usize
        };
        let mut saved_rbx = [0u8; 0x18];
        for i in 0..0x18 {
            saved_rbx[i] = rd::<u8>(rbx + i);
        }

        let mut buf_o = [[0usize; 2]; 4];
        let mut buf_n = [[0usize; 2]; 4];
        let mut ring_o = [0u8; 0x18];
        let mut ring_n = [0u8; 0x18];
        for (ring, buf) in [(&mut ring_o, &mut buf_o), (&mut ring_n, &mut buf_n)] {
            let b = ring.as_mut_ptr() as usize;
            wr::<usize>(b, buf.as_mut_ptr() as usize);
            wr::<u32>(b + 0x0c, 2);
            wr::<u32>(b + 0x10, 1);
            wr::<u32>(b + 0x14, 4);
        }

        wr::<u32>(rbx + 0x08, tid);
        wr::<u32>(rbx + 0x0c, 1);
        wr::<usize>(rbx + 0x10, ring_o.as_mut_ptr() as usize);
        orig_fn(0xaaaa_bbbb, edx);

        wr::<u32>(rbx + 0x08, tid);
        wr::<u32>(rbx + 0x0c, 1);
        wr::<usize>(rbx + 0x10, ring_n.as_mut_ptr() as usize);
        new_fn(0xaaaa_bbbb, edx);

        for i in 0..0x18 {
            wr::<u8>(rbx + i, saved_rbx[i]);
        }

        crate::diff_eq!(buf_o, buf_n, "d3d_14209a1e0 buf");
        crate::diff_eq!(&ring_o[0x08..0x18], &ring_n[0x08..0x18], "d3d_14209a1e0 ring");
        cases += 1;
    }

    Ok(cases)
}

pub unsafe fn diff_d3d_14209a2a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let g_flag = exe::ptr::<u8>(0x1_4385_8580) as usize;
    let g_len = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let s_flag = rd::<u8>(g_flag);
    let s_len = rd::<u32>(g_len);
    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0x5au8; 0x4b500];
    let mut tls_n = vec![0x5au8; 0x4b500];

    // 1. flag == 0
    wr::<u8>(g_flag, 0);
    wr::<u32>(g_len, 0x80);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    // 2. flag == 1
    wr::<u8>(g_flag, 1);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    wr::<usize>(tls_slot, s_tls0);
    wr::<u8>(g_flag, s_flag);
    wr::<u32>(g_len, s_len);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6460],
        &tls_n[0x63d0..0x6460],
        "tls +0x63d0"
    );
    crate::diff_eq!(
        &tls_o[0x463e0..0x467e0],
        &tls_n[0x463e0..0x467e0],
        "tls +0x463e0"
    );
    crate::diff_eq!(
        &tls_o[0x47460..0x4b460],
        &tls_n[0x47460..0x4b460],
        "tls +0x47460"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209a430(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, usize) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut head = [0u8; 0xb0];
    let mut node = [0u8; 0xb0];
    let head_ptr = head.as_mut_ptr() as usize;
    let node_ptr = node.as_mut_ptr() as usize;

    // Sentinel head: is_nil (+0x19) = 1, leftmost (+0x0) = node_ptr
    wr::<usize>(head_ptr, node_ptr);
    wr::<usize>(head_ptr + 0x08, node_ptr);
    wr::<usize>(head_ptr + 0x10, node_ptr);
    wr::<u8>(head_ptr + 0x19, 1);

    // Single node: left/parent/right = head_ptr, is_nil = 0, key (+0xa4) = 42, C-string at +0x1c = "test\0"
    wr::<usize>(node_ptr, head_ptr);
    wr::<usize>(node_ptr + 0x08, head_ptr);
    wr::<usize>(node_ptr + 0x10, head_ptr);
    wr::<u8>(node_ptr + 0x19, 0);
    node[0x1c..0x21].copy_from_slice(b"test\0");
    wr::<u32>(node_ptr + 0xa4, 42);

    let g_head = exe::ptr::<usize>(0x1_43a8_77b0) as usize;
    let s_head = rd::<usize>(g_head);
    wr::<usize>(g_head, head_ptr);

    // 1. Key not found -> returns 0 without touching rdx
    let ro0 = orig_fn(99, 0);
    let rn0 = new_fn(99, 0);
    wr::<usize>(g_head, s_head);
    crate::diff_eq!(ro0, rn0, "not found");
    Ok(1)
}

pub unsafe fn diff_d3d_14209a6e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32) -> u8;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x47500];
    let mut tls_n = vec![0u8; 0x47500];
    for tls in [&mut tls_o, &mut tls_n] {
        wr::<u32>(tls.as_mut_ptr() as usize + 0x463e0 + 12, 0x1234_5678);
        wr::<u32>(tls.as_mut_ptr() as usize + 0x47460 + 20, 0x9999_9999);
    }

    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    // 1. Different values -> returns 1 and updates dst
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    let ro1 = orig_fn(3, 5);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    let rn1 = new_fn(3, 5);

    // 2. Equal values -> returns 0
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    let ro2 = orig_fn(3, 5);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    let rn2 = new_fn(3, 5);

    wr::<usize>(tls_slot, s_tls0);

    crate::diff_eq!((ro1, ro2), (rn1, rn2), "ret values");
    crate::diff_eq!(
        &tls_o[0x47460..0x47480],
        &tls_n[0x47460..0x47480],
        "dst slot"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209a720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, u32, u64);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let entry_addr = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let total_addr = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let saved_entry = rd::<u128>(entry_addr);
    let saved_total = rd::<u32>(total_addr);

    // Test 1: cur_type == -1 -> allocates
    wr::<i32>(entry_addr + 4, -1);
    wr::<u32>(total_addr, 0x100);
    orig_fn(0, 7, 0x40);
    let e_o1 = rd::<u128>(entry_addr);
    let t_o1 = rd::<u32>(total_addr);

    wr::<u128>(entry_addr, saved_entry);
    wr::<i32>(entry_addr + 4, -1);
    wr::<u32>(total_addr, 0x100);
    new_fn(0, 7, 0x40);
    let e_n1 = rd::<u128>(entry_addr);
    let t_n1 = rd::<u32>(total_addr);

    // Test 2: cur_type != -1 and cur_size >= r8 -> no-op
    orig_fn(0, 9, 0x20);
    let e_o2 = rd::<u128>(entry_addr);
    let t_o2 = rd::<u32>(total_addr);

    wr::<u128>(entry_addr, e_n1);
    wr::<u32>(total_addr, t_n1);
    new_fn(0, 9, 0x20);
    let e_n2 = rd::<u128>(entry_addr);
    let t_n2 = rd::<u32>(total_addr);

    wr::<u128>(entry_addr, saved_entry);
    wr::<u32>(total_addr, saved_total);

    crate::diff_eq!((e_o1, t_o1), (e_n1, t_n1), "alloc entry");
    crate::diff_eq!((e_o2, t_o2), (e_n2, t_n2), "no-op when size sufficient");
    Ok(2)
}

pub unsafe fn diff_d3d_14209a790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let slot_ptr = exe::ptr::<u8>(0x1_43a6_37bc) as usize;
    let saved = rd::<i32>(slot_ptr);

    // 1. cur == -1 -> sets to edx
    wr::<i32>(slot_ptr, -1);
    orig_fn(0, 12);
    let o1 = rd::<i32>(slot_ptr);

    wr::<i32>(slot_ptr, -1);
    new_fn(0, 12);
    let n1 = rd::<i32>(slot_ptr);

    // 2. cur == edx -> keeps edx
    orig_fn(0, 12);
    let o2 = rd::<i32>(slot_ptr);
    new_fn(0, 12);
    let n2 = rd::<i32>(slot_ptr);

    wr::<i32>(slot_ptr, saved);
    crate::diff_eq!((o1, o2), (n1, n2), "d3d_14209a790");
    Ok(2)
}

pub unsafe fn diff_d3d_14209a7f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let entry_addr = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let saved_off = rd::<i32>(entry_addr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<i32>(entry_addr, 0x240);
    wr::<usize>(tls_slot, 0x5000_0000);
    let ro = orig_fn(0);
    let rn = new_fn(0);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<i32>(entry_addr, saved_off);

    crate::diff_eq!(ro, rn, "d3d_14209a7f0");
    Ok(1)
}

pub unsafe fn diff_d3d_14209a820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x46500];
    let mut tls_n = vec![0u8; 0x46500];
    for tls in [&mut tls_o, &mut tls_n] {
        wr::<u32>(tls.as_mut_ptr() as usize + 0x463e0 + 8, 5);
        wr::<u32>(tls.as_mut_ptr() as usize + 0x463e0 + 12, u32::MAX - 1);
    }

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(-1);
    orig_fn(2);
    orig_fn(3);

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(-1);
    new_fn(2);
    new_fn(3);

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(
        &tls_o[0x463e0..0x46400],
        &tls_n[0x463e0..0x46400],
        "d3d_14209a820"
    );
    Ok(3)
}

pub unsafe fn diff_d3d_14209a870(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x46500];
    let mut tls_n = vec![0u8; 0x46500];
    for tls in [&mut tls_o, &mut tls_n] {
        wr::<u32>(tls.as_mut_ptr() as usize + 0x463e0 + 16, 99);
    }

    let slot_ptr = exe::ptr::<u8>(0x1_43a6_37bc) as usize;
    let saved_slot = rd::<i32>(slot_ptr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    // 1. slot == -1 -> no-op
    wr::<i32>(slot_ptr, -1);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    // 2. slot == 4 -> increments tls0 + 0x463e0 + 16
    wr::<i32>(slot_ptr, 4);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<i32>(slot_ptr, saved_slot);

    crate::diff_eq!(
        &tls_o[0x463e0..0x46400],
        &tls_n[0x463e0..0x46400],
        "d3d_14209a870"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209a8d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x46500];
    let mut tls_n = vec![0u8; 0x46500];
    let src = [0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];

    let entry_addr = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let saved_entry = rd::<u128>(entry_addr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<i32>(entry_addr, 0x20); // off
    wr::<i32>(entry_addr + 8, 8); // size
    wr::<i32>(entry_addr + 12, 3); // slot

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, src.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, src.as_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u128>(entry_addr, saved_entry);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6410],
        &tls_n[0x63d0..0x6410],
        "copied bytes"
    );
    crate::diff_eq!(
        &tls_o[0x463e0..0x46400],
        &tls_n[0x463e0..0x46400],
        "version counter"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_14209a900(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, usize, usize, usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x46500];
    let mut tls_n = vec![0u8; 0x46500];
    let src = [0xa1u8, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x07, 0x18];

    let entry_addr = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let saved_entry = rd::<u128>(entry_addr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    // 1. off < 0 -> no-op
    wr::<i32>(entry_addr, -1);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, 4, 8, src.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, 4, 8, src.as_ptr() as usize);

    // 2. off >= 0 -> copies and bumps slot
    wr::<i32>(entry_addr, 0x10);
    wr::<i32>(entry_addr + 12, 2);
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, 4, 8, src.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, 4, 8, src.as_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u128>(entry_addr, saved_entry);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6400],
        &tls_n[0x63d0..0x6400],
        "copied bytes"
    );
    crate::diff_eq!(
        &tls_o[0x463e0..0x46400],
        &tls_n[0x463e0..0x46400],
        "version counter"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209a990(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x46500];
    let mut tls_n = vec![0u8; 0x46500];
    let pat = [0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let other = [0x99u8; 8];
    for tls in [&mut tls_o, &mut tls_n] {
        tls[0x63d0 + 0x10..0x63d0 + 0x18].copy_from_slice(&pat);
    }

    let entry_addr = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let saved_entry = rd::<u128>(entry_addr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<i32>(entry_addr, 0x10); // off
    wr::<i32>(entry_addr + 8, 8); // size
    wr::<i32>(entry_addr + 12, 1); // slot

    // 1. Non-matching buffer -> does not zero, increments slot
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, other.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, other.as_ptr() as usize);

    // 2. Matching buffer -> zeroes buffer and increments slot
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, pat.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, pat.as_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u128>(entry_addr, saved_entry);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6400],
        &tls_n[0x63d0..0x6400],
        "zeroed buffer"
    );
    crate::diff_eq!(
        &tls_o[0x463e0..0x46400],
        &tls_n[0x463e0..0x46400],
        "version counter"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209aa70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let g_len = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let saved_len = rd::<u32>(g_len);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0x5au8; 0x4b500];
    let mut tls_n = vec![0x5au8; 0x4b500];
    let mut src_blob = vec![0u8; 0x480];
    for (i, b) in src_blob.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(7);
    }

    wr::<u32>(g_len, 0x40);

    // 1. rcx == 0 -> zeroes tls0 + 0x63d0..+0x6410
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6420],
        &tls_n[0x63d0..0x6420],
        "rcx==0 +0x63d0"
    );

    // 2. rcx != 0 -> copies from rcx + 0x400
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(src_blob.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(src_blob.as_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u32>(g_len, saved_len);

    crate::diff_eq!(
        &tls_o[0x63d0..0x6420],
        &tls_n[0x63d0..0x6420],
        "rcx!=0 +0x63d0"
    );
    crate::diff_eq!(
        &tls_o[0x463e0..0x467e0],
        &tls_n[0x463e0..0x467e0],
        "+0x463e0"
    );
    crate::diff_eq!(
        &tls_o[0x47460..0x4b460],
        &tls_n[0x47460..0x4b460],
        "+0x47460"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_14209ab00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let g_len = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let saved_len = rd::<u32>(g_len);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut fake_tls = vec![0u8; 0x47000];
    for (i, b) in fake_tls[0x463e0..0x467e0].iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(17).wrapping_add(3);
    }
    for (i, b) in fake_tls[0x63d0..0x6450].iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(29).wrapping_add(11);
    }

    let mut dst_o = vec![0u8; 0x500];
    let mut dst_n = vec![0u8; 0x500];
    let mut hdr_o = [0u8; 0x10];
    let mut hdr_n = [0u8; 0x10];
    wr::<usize>(hdr_o.as_mut_ptr() as usize, dst_o.as_mut_ptr() as usize);
    wr::<u32>(hdr_o.as_mut_ptr() as usize + 0x08, 0x500);
    wr::<usize>(hdr_n.as_mut_ptr() as usize, dst_n.as_mut_ptr() as usize);
    wr::<u32>(hdr_n.as_mut_ptr() as usize + 0x08, 0x500);

    wr::<u32>(g_len, 0x80);
    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
    orig_fn(hdr_o.as_mut_ptr() as usize);
    new_fn(hdr_n.as_mut_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u32>(g_len, saved_len);

    crate::diff_eq!(dst_o, dst_n, "d3d_14209ab00 snapshot");
    Ok(1)
}

pub unsafe fn diff_d3d_142097c40(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_14243b3e0(pool: usize, node: usize) {
        wr::<usize>(pool, node);
        wr::<u32>(pool + 8, rd::<u32>(pool + 8).wrapping_add(1));
    }

    let _m = crate::difftest::Mock::new(0x1_4243_b3e0, fake_14243b3e0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut rbx_o = [0u8; 0x30];
    let mut rbx_n = [0u8; 0x30];
    let mut list1_o = [0u8; 0x30];
    let mut list1_n = [0u8; 0x30];
    let mut list2_o = [0u8; 0x30];
    let mut list2_n = [0u8; 0x30];
    let mut rdi_o = [0u8; 0x68];
    let mut rdi_n = [0u8; 0x68];

    for (l1, l2, rdi) in [
        (&mut list1_o, &mut list2_o, &mut rdi_o),
        (&mut list1_n, &mut list2_n, &mut rdi_n),
    ] {
        let l1p = l1.as_mut_ptr() as usize;
        let l2p = l2.as_mut_ptr() as usize;
        let rp = rdi.as_mut_ptr() as usize;
        let n2p = rp + 0x20;

        wr::<usize>(l1p + 0x08, rp);
        wr::<usize>(l1p + 0x10, rp);
        wr::<usize>(l1p + 0x18, l1p);
        wr::<u32>(l1p + 0x28, 3);

        wr::<usize>(rp + 0x08, l1p);
        wr::<usize>(rp + 0x10, l1p);
        wr::<usize>(rp + 0x18, l1p);

        wr::<usize>(l2p + 0x08, n2p);
        wr::<usize>(l2p + 0x10, n2p);
        wr::<usize>(l2p + 0x18, l2p);
        wr::<u32>(l2p + 0x28, 5);

        wr::<usize>(n2p + 0x08, l2p);
        wr::<usize>(n2p + 0x10, l2p);
        wr::<usize>(n2p + 0x18, l2p);

        wr::<usize>(rp + 0x58, 0xaaaa_bbbb);
        wr::<usize>(rp + 0x60, 0xcccc_dddd);
    }

    orig_fn(rbx_o.as_mut_ptr() as usize, rdi_o.as_mut_ptr() as usize);
    new_fn(rbx_n.as_mut_ptr() as usize, rdi_n.as_mut_ptr() as usize);

    crate::diff_eq!( rd::<u32>(list1_o.as_ptr() as usize + 0x28), rd::<u32>(list1_n.as_ptr() as usize + 0x28), "list1 count");
    crate::diff_eq!( rd::<u32>(list2_o.as_ptr() as usize + 0x28), rd::<u32>(list2_n.as_ptr() as usize + 0x28), "list2 count");
    crate::diff_eq!(&rdi_o[0x18..0x20], &rdi_n[0x18..0x20], "rdi+0x18");
    crate::diff_eq!(&rdi_o[0x38..0x68], &rdi_n[0x38..0x68], "rdi+0x38..0x68");
    crate::diff_eq!(rd::<u32>(rbx_o.as_ptr() as usize + 0x28), rd::<u32>(rbx_n.as_ptr() as usize + 0x28), "pool calls");
    Ok(1)
}

pub unsafe fn diff_d3d_142097d00(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_142097c40(rdi: usize, node: usize) {
        let rem = rd::<u32>(rdi + 0x88).wrapping_sub(1);
        wr::<u32>(rdi + 0x88, rem);
        wr::<usize>(rdi + 0x90, rd::<usize>(rdi + 0x90).wrapping_add(node));
    }
    unsafe extern "win64" fn fake_140227190(p460: usize) {
        wr::<u64>(p460, 0x1122_3344_5566_7788);
    }
    unsafe extern "win64" fn fake_14243b540(p20: usize) {
        wr::<u64>(p20, 0x99aa_bbcc_ddee_ff00);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_7c40, fake_142097c40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4022_7190, fake_140227190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_b540, fake_14243b540 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let entry = [0x1234usize];
    let mut obj_o = vec![0u8; 0x470];
    let mut obj_n = vec![0u8; 0x470];
    for obj in [&mut obj_o, &mut obj_n] {
        let b = obj.as_mut_ptr() as usize;
        wr::<usize>(b + 0x70, entry.as_ptr() as usize);
        wr::<u32>(b + 0x88, 2);
    }

    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(obj_o, obj_n, "d3d_142097d00 state");
    Ok(1)
}

unsafe extern "win64" fn fake_1420989b0_rec(p: usize) {
    wr::<u64>(p + 0x18, rd::<u64>(p).wrapping_add(0x7777));
}

pub unsafe fn diff_d3d_142097d90(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4209_89b0, fake_1420989b0_rec as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut obj_o = [0x33u8; 0x28];
    let mut obj_n = [0x33u8; 0x28];
    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(obj_o, obj_n, "d3d_142097d90");
    Ok(1)
}

pub unsafe fn diff_d3d_142098750(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4209_89b0, fake_1420989b0_rec as *const ())?;

    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut obj_o = [0x44u8; 0x20];
    let mut obj_n = [0x44u8; 0x20];
    let ro = orig_fn(obj_o.as_mut_ptr() as usize);
    let rn = new_fn(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(ro, obj_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, obj_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(obj_o, obj_n, "d3d_142098750");
    Ok(1)
}

pub unsafe fn diff_d3d_1420988e0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_free_142442de0(p: usize) {
        let sum = rd::<u64>(p) | rd::<u64>(p + 8) | rd::<u64>(p + 0x10);
        wr::<u64>(p + 0x20, sum ^ 0xdead_0001);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_89b0, fake_1420989b0_rec as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free_142442de0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    // 1. Null pointer -> no-op
    orig_fn(0);
    new_fn(0);

    // 2. Non-null pointer
    let mut obj_o = [0x55u8; 0x28];
    let mut obj_n = [0x55u8; 0x28];
    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(obj_o, obj_n, "d3d_1420988e0");
    Ok(2)
}

pub unsafe fn diff_d3d_142098f10(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_free_item(p: usize) {
        wr::<u64>(p + 0x20, 0xcafe_0002);
    }
    unsafe extern "win64" fn fake_dtor_ring(ring_ptr: usize) {
        wr::<u64>(ring_ptr, 0);
        wr::<u64>(ring_ptr + 8, 0);
        wr::<u64>(ring_ptr + 0x10, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_89b0, fake_1420989b0_rec as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free_item as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_91d0, fake_dtor_ring as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);

    let mut item_o = [0x66u8; 0x28];
    let mut item_n = [0x66u8; 0x28];
    let ring_buf_o = [0usize, item_o.as_mut_ptr() as usize, 0, 0];
    let ring_buf_n = [0usize, item_n.as_mut_ptr() as usize, 0, 0];

    let mut obj_o = [0u8; 0x30];
    let mut obj_n = [0u8; 0x30];
    for (obj, rbuf) in [(&mut obj_o, &ring_buf_o), (&mut obj_n, &ring_buf_n)] {
        let b = obj.as_mut_ptr() as usize;
        wr::<u32>(b + 0x08, tid);
        wr::<u32>(b + 0x0c, 1);
        wr::<usize>(b + 0x10, rbuf.as_ptr() as usize);
        wr::<u32>(b + 0x18, 1);
        wr::<u32>(b + 0x20, 2);
        wr::<u32>(b + 0x24, 4);
        wr::<u64>(b + 0x28, 10);
    }

    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(item_o, item_n, "d3d_142098f10 item");
    crate::diff_eq!(obj_o, obj_n, "d3d_142098f10 obj");
    Ok(1)
}

pub unsafe fn diff_d3d_142099a80(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_142099c20(cl: u8, edx: u32) -> u32 {
        (cl as u32)
            .wrapping_mul(100)
            .wrapping_add(edx.wrapping_mul(17))
            .wrapping_add(3)
    }

    let _m = crate::difftest::Mock::new(0x1_4209_9c20, fake_142099c20 as *const ())?;

    type F = unsafe extern "win64" fn(u8) -> u32;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut ran = 0u32;
    for cl in [0u8, 1, 5, 0xff] {
        crate::diff_eq!(orig_fn(cl), new_fn(cl), "d3d_142099a80");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_142099ac0(orig: usize, new: usize) -> Result<u32, String> {
    static REC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    unsafe extern "win64" fn fake_1420995c0(arg: u32) {
        let prev = REC.load(std::sync::atomic::Ordering::SeqCst);
        REC.store(
            prev.wrapping_mul(31).wrapping_add(arg as u64 + 10),
            std::sync::atomic::Ordering::SeqCst,
        );
    }

    let _m = crate::difftest::Mock::new(0x1_4209_95c0, fake_1420995c0 as *const ())?;

    type F = unsafe extern "win64" fn();
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    REC.store(0, std::sync::atomic::Ordering::SeqCst);
    orig_fn();
    let ro = REC.swap(0, std::sync::atomic::Ordering::SeqCst);

    new_fn();
    let rn = REC.swap(0, std::sync::atomic::Ordering::SeqCst);

    crate::diff_eq!(ro, rn, "d3d_142099ac0 call sequence");
    Ok(1)
}

pub unsafe fn diff_d3d_14209abe0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_142056ed0(rcx: usize) {
        wr::<usize>(rcx, 0x1111_2222);
        wr::<u64>(rcx + 8, 0x3333_4444_5555_6666);
    }

    let _m = crate::difftest::Mock::new(0x1_4205_6ed0, fake_142056ed0 as *const ())?;

    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0x5au8; 0x46400];
    let mut tls_n = vec![0x5au8; 0x46400];
    let mut obj_o = [0u8; 0x18];
    let mut obj_n = [0u8; 0x18];

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    let ro = orig_fn(obj_o.as_mut_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    let rn = new_fn(obj_n.as_mut_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(ro, obj_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, obj_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(obj_o, obj_n, "d3d_14209abe0 obj");
    crate::diff_eq!(&tls_o[0x63c0..0x6400], &tls_n[0x63c0..0x6400], "tls start");
    crate::diff_eq!(&tls_o[0x463c0..0x463e0], &tls_n[0x463c0..0x463e0], "tls end");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x142097880, "d3d::d3d_142097880", crate::ns::rend::d3d_constants_b::d3d_142097880 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142097880),
// Replacement::verified(0x142097be0, "d3d::d3d_142097be0", crate::ns::rend::d3d_constants_b::d3d_142097be0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142097be0),
// Replacement::verified(0x142097c40, "d3d::d3d_142097c40", crate::ns::rend::d3d_constants_b::d3d_142097c40 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142097c40),
// Replacement::verified(0x142097d00, "d3d::d3d_142097d00", crate::ns::rend::d3d_constants_b::d3d_142097d00 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142097d00),
// Replacement::verified(0x142097d90, "d3d::d3d_142097d90", crate::ns::rend::d3d_constants_b::d3d_142097d90 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142097d90),
// Replacement::verified(0x1420981e0, "d3d::d3d_1420981e0", crate::ns::rend::d3d_constants_b::d3d_1420981e0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_1420981e0),
// Replacement::verified(0x142098750, "d3d::d3d_142098750", crate::ns::rend::d3d_constants_b::d3d_142098750 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142098750),
// Replacement::verified(0x1420988e0, "d3d::d3d_1420988e0", crate::ns::rend::d3d_constants_b::d3d_1420988e0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_1420988e0),
// Replacement::verified(0x142098930, "d3d::d3d_142098930", crate::ns::rend::d3d_constants_b::d3d_142098930 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142098930),
// Replacement::verified(0x142098990, "d3d::d3d_142098990", crate::ns::rend::d3d_constants_b::d3d_142098990 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142098990),
// Replacement::verified(0x142098a90, "d3d::d3d_142098a90", crate::ns::rend::d3d_constants_b::d3d_142098a90 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142098a90),
// Replacement::verified(0x142098f10, "d3d::d3d_142098f10", crate::ns::rend::d3d_constants_b::d3d_142098f10 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142098f10),
// Replacement::verified(0x142099350, "d3d::d3d_142099350", crate::ns::rend::d3d_constants_b::d3d_142099350 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142099350),
// Replacement::verified(0x142099410, "d3d::d3d_142099410", crate::ns::rend::d3d_constants_b::d3d_142099410 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142099410),
// Replacement::verified(0x142099430, "d3d::d3d_142099430", crate::ns::rend::d3d_constants_b::d3d_142099430 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142099430),
// Replacement::verified(0x1420994f0, "d3d::d3d_1420994f0", crate::ns::rend::d3d_constants_b::d3d_1420994f0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_1420994f0),
// Replacement::verified_throws(0x142099560, "d3d::d3d_142099560", crate::ns::rend::d3d_constants_b::d3d_142099560 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_b::diff_d3d_142099560),
// Replacement::verified(0x1420996d0, "d3d::d3d_1420996d0", crate::ns::rend::d3d_constants_b::d3d_1420996d0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_1420996d0),
// Replacement::verified(0x142099780, "d3d::d3d_142099780", crate::ns::rend::d3d_constants_b::d3d_142099780 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_142099780),
// Replacement::verified_throws(0x1420997e0, "d3d::d3d_1420997e0", crate::ns::rend::d3d_constants_b::d3d_1420997e0 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_b::diff_d3d_1420997e0),
// Replacement::verified_throws(0x142099a80, "d3d::d3d_142099a80", crate::ns::rend::d3d_constants_b::d3d_142099a80 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_b::diff_d3d_142099a80),
// Replacement::verified_throws(0x142099ac0, "d3d::d3d_142099ac0", crate::ns::rend::d3d_constants_b::d3d_142099ac0 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_b::diff_d3d_142099ac0),
// Replacement::verified(0x14209a110, "d3d::d3d_14209a110", crate::ns::rend::d3d_constants_b::d3d_14209a110 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a110),
// Replacement::verified(0x14209a1e0, "d3d::d3d_14209a1e0", crate::ns::rend::d3d_constants_b::d3d_14209a1e0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a1e0),
// Replacement::verified(0x14209a2a0, "d3d::d3d_14209a2a0", crate::ns::rend::d3d_constants_b::d3d_14209a2a0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a2a0),
// Replacement::verified(0x14209a430, "d3d::d3d_14209a430", crate::ns::rend::d3d_constants_b::d3d_14209a430 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a430),
// Replacement::verified(0x14209a500, "d3d::d3d_14209a500", crate::ns::rend::d3d_constants_b::d3d_14209a500 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a500),
// Replacement::verified(0x14209a6e0, "d3d::d3d_14209a6e0", crate::ns::rend::d3d_constants_b::d3d_14209a6e0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a6e0),
// Replacement::verified(0x14209a720, "d3d::d3d_14209a720", crate::ns::rend::d3d_constants_b::d3d_14209a720 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a720),
// Replacement::verified(0x14209a770, "d3d::d3d_14209a770", crate::ns::rend::d3d_constants_b::d3d_14209a770 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a770),
// Replacement::verified_throws(0x14209a790, "d3d::d3d_14209a790", crate::ns::rend::d3d_constants_b::d3d_14209a790 as *const (), Throws::Caught, crate::ns::rend::d3d_constants_b::diff_d3d_14209a790),
// Replacement::verified(0x14209a7f0, "d3d::d3d_14209a7f0", crate::ns::rend::d3d_constants_b::d3d_14209a7f0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a7f0),
// Replacement::verified(0x14209a820, "d3d::d3d_14209a820", crate::ns::rend::d3d_constants_b::d3d_14209a820 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a820),
// Replacement::verified(0x14209a870, "d3d::d3d_14209a870", crate::ns::rend::d3d_constants_b::d3d_14209a870 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a870),
// Replacement::verified(0x14209a8d0, "d3d::d3d_14209a8d0", crate::ns::rend::d3d_constants_b::d3d_14209a8d0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a8d0),
// Replacement::verified(0x14209a900, "d3d::d3d_14209a900", crate::ns::rend::d3d_constants_b::d3d_14209a900 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a900),
// Replacement::verified(0x14209a990, "d3d::d3d_14209a990", crate::ns::rend::d3d_constants_b::d3d_14209a990 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209a990),
// Replacement::verified(0x14209aa70, "d3d::d3d_14209aa70", crate::ns::rend::d3d_constants_b::d3d_14209aa70 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209aa70),
// Replacement::verified(0x14209ab00, "d3d::d3d_14209ab00", crate::ns::rend::d3d_constants_b::d3d_14209ab00 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209ab00),
// Replacement::verified(0x14209abe0, "d3d::d3d_14209abe0", crate::ns::rend::d3d_constants_b::d3d_14209abe0 as *const (), crate::ns::rend::d3d_constants_b::diff_d3d_14209abe0),

