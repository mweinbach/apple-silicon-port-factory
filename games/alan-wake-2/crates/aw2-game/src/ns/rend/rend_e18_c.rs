//! Decompiled `rend` particle, render-command, and render-graph helper functions (`0x1422f6ff0..0x1423002b0`).

use crate::exe::{self, rd, wr};

/// `rend::rend_1422f6ff0`
/// Original: 0x1422f6ff0 (61 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f6ff0(this: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0xac, 0);
    wr::<u64>(this + 0xb4, 0);
    this
}

/// `rend::rend_1422f7040` (`RenderInstancesCommand` ctor)
/// Original: 0x1422f7040 (83 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f7040(this: usize) -> usize {
    wr::<u32>(this + 0x8, 0xffff_ffff);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_a5b8) as u64);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);
    wr::<u64>(this + 0x60, 0);
    wr::<u64>(this + 0xc4, 0);
    wr::<u64>(this + 0xcc, 0);
    this
}

/// `rend::rend_1422f70a0` (`RenderInstancesFromGfxGraphCommand` ctor)
/// Original: 0x1422f70a0 (112 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f70a0(this: usize) -> usize {
    wr::<u32>(this + 0x8, 0xffff_ffff);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_a588) as u64);
    wr::<u64>(this + 0x20, exe::ptr::<u8>(0x1_433f_a5b8) as u64);
    wr::<u64>(this + 0x18, 0);
    wr::<u32>(this + 0x28, 0xffff_ffff);
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);
    wr::<u64>(this + 0x60, 0);
    wr::<u64>(this + 0x68, 0);
    wr::<u64>(this + 0x70, 0);
    wr::<u64>(this + 0x78, 0);
    wr::<u64>(this + 0x80, 0);
    wr::<u64>(this + 0xe4, 0);
    wr::<u64>(this + 0xec, 0);
    this
}

/// `rend::rend_1422f7120`
/// Original: 0x1422f7120 (160 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f7120(this: usize) -> usize {
    let c = rd::<[u32; 4]>(exe::ptr::<u8>(0x1_4342_7660) as usize);
    wr::<u64>(this, 0x3f80_0000);
    wr::<u32>(this + 0x8, 0);
    wr::<u64>(this + 0xc, (c[0] as u64) << 32);
    wr::<u64>(this + 0x24, (c[1] as u64) | ((c[2] as u64) << 32));
    wr::<u64>(this + 0x18, (c[2] as u64) | ((c[3] as u64) << 32));
    wr::<u32>(this + 0x2c, c[3]);
    wr::<u32>(this + 0x14, c[1]);
    wr::<u32>(this + 0x20, c[0]);
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);
    wr::<u64>(this + 0x60, 0);
    wr::<u64>(this + 0x68, 0);
    wr::<u64>(this + 0x70, 0);
    wr::<u64>(this + 0x78, 0);
    wr::<u64>(this + 0xd0, 0);
    wr::<u64>(this + 0xd8, 0);
    this
}

/// `rend::rend_1422f71d0` (`RenderParticleCommand` ctor)
/// Original: 0x1422f71d0 (194 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f71d0(this: usize) -> usize {
    let c = rd::<[u32; 4]>(exe::ptr::<u8>(0x1_4342_7660) as usize);
    wr::<u32>(this + 0x8, 0xffff_ffff);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_a5a0) as u64);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x20, 0x3f80_0000);
    wr::<u32>(this + 0x28, 0);
    wr::<u64>(this + 0x2c, (c[0] as u64) << 32);
    wr::<u64>(this + 0x44, (c[1] as u64) | ((c[2] as u64) << 32));
    wr::<u64>(this + 0x38, (c[2] as u64) | ((c[3] as u64) << 32));
    wr::<u32>(this + 0x4c, c[3]);
    wr::<u32>(this + 0x34, c[1]);
    wr::<u32>(this + 0x40, c[0]);
    wr::<u64>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);
    wr::<u64>(this + 0x60, 0);
    wr::<u64>(this + 0x68, 0);
    wr::<u64>(this + 0x70, 0);
    wr::<u64>(this + 0x78, 0);
    wr::<u64>(this + 0x80, 0);
    wr::<u64>(this + 0x88, 0);
    wr::<u64>(this + 0x90, 0);
    wr::<u64>(this + 0x98, 0);
    wr::<u64>(this + 0xf0, 0);
    wr::<u64>(this + 0xf8, 0);
    this
}

/// `rend::rend_1422f72a0` (`RenderParticleFromGfxGraphCommand` ctor)
/// Original: 0x1422f72a0 (51 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f72a0(this: usize) -> usize {
    wr::<u32>(this + 0x8, 0xffff_ffff);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_a558) as u64);
    let _ = exe::call_va!(0x1_422f_71d0, (this.wrapping_add(0x20)) -> usize);
    this
}

/// `rend::rend_1422f7340`
/// Original: 0x1422f7340 (95 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f7340(this: usize, other: usize) -> usize {
    if this != other {
        let count_slot = other.wrapping_add(0x48);
        wr::<u64>(this + 0x48, 0);
        let count = rd::<u64>(count_slot) as usize;
        let end = other.wrapping_add(count.wrapping_mul(0x18));
        let mut src = other;
        let mut dst = this;
        while src != end {
            let b0 = rd::<[u8; 16]>(src);
            wr::<[u8; 16]>(dst, b0);
            let q2 = rd::<u64>(src + 0x10);
            src = src.wrapping_add(0x18);
            wr::<u64>(dst + 0x10, q2);
            dst = dst.wrapping_add(0x18);
        }
        let final_count = rd::<u64>(count_slot);
        wr::<u64>(count_slot, 0);
        wr::<u64>(this + 0x48, final_count);
    }
    this
}

/// `rend::rend_1422f73f0`
/// Original: 0x1422f73f0 (95 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f73f0(this: usize, other: usize) -> usize {
    wr::<u64>(this + 0x48, 0);
    if this != other {
        let count_slot = other.wrapping_add(0x48);
        let count = rd::<u64>(count_slot) as usize;
        let end = other.wrapping_add(count.wrapping_mul(0x18));
        let mut src = other;
        let mut dst = this;
        while src != end {
            let b0 = rd::<[u8; 16]>(src);
            wr::<[u8; 16]>(dst, b0);
            let q2 = rd::<u64>(src + 0x10);
            src = src.wrapping_add(0x18);
            wr::<u64>(dst + 0x10, q2);
            dst = dst.wrapping_add(0x18);
        }
        let final_count = rd::<u64>(count_slot);
        wr::<u64>(count_slot, 0);
        wr::<u64>(this + 0x48, final_count);
    }
    this
}

/// `rend::ParticleEngine::vfunc_1` (`rend_1422f8f70`)
/// Original: 0x1422f8f70 (167 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f8f70(this: usize) {
    let mut cur = rd::<usize>(this + 0x60);
    let len = rd::<u32>(this + 0x68) as usize;
    let end = cur.wrapping_add(len.wrapping_mul(8));
    if cur != end {
        let neg_inf = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
        while cur != end {
            let item = rd::<usize>(cur);
            exe::call_va!(0x1_422f_6c40, (item) -> ());
            let pos_inf_pair = rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize);
            wr::<u64>(item + 0x1c, pos_inf_pair);
            let pos_inf = rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize);
            cur = cur.wrapping_add(8);
            wr::<u32>(item + 0x24, pos_inf);
            wr::<u32>(item + 0x28, neg_inf);
            wr::<u32>(item + 0x2c, neg_inf);
            wr::<u32>(item + 0x30, neg_inf);
        }
        wr::<u32>(this + 0x68, 0);
        return;
    }
    wr::<u32>(this + 0x68, 0);
}

/// `rend::rend_1422f9020` (`rend::ParticleEngine` destructor)
/// Original: 0x1422f9020 (221 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f9020(this: usize) {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_a540) as u64);
    exe::call_va!(0x1_4243_b310, (rd::<usize>(this + 0x58)) -> ());
    let p58 = rd::<usize>(this + 0x58);
    if p58 != 0 {
        exe::call_va!(0x1_4243_b540, (p58) -> ());
        exe::call_va!(0x1_4244_2de0, (p58) -> ());
    }
    wr::<u64>(this + 0x58, 0);
    wr::<u64>(exe::ptr::<u64>(0x1_4397_ff80) as usize, 0);
    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x60)) -> ());
    let p28 = rd::<usize>(this + 0x28);
    if p28 != 0 {
        exe::call_va!(0x1_4233_8630, (p28) -> ());
        exe::call_va!(0x1_4244_2de0, (p28) -> ());
    }
    exe::call_va!(0x1_4130_fd20, (this.wrapping_add(0x20)) -> ());
    let p18 = rd::<usize>(this + 0x18);
    if p18 != 0 {
        exe::call_va!(0x1_4209_d670, (p18) -> ());
        exe::call_va!(0x1_411e_7fc0, (p18.wrapping_add(0x58)) -> ());
        exe::call_va!(0x1_4244_2de0, (p18) -> ());
    }
    let p10 = rd::<usize>(this + 0x10);
    if p10 != 0 {
        exe::call_va!(0x1_4204_db80, (p10) -> ());
        exe::call_va!(0x1_4244_2de0, (p10) -> ());
    }
    let p08 = rd::<usize>(this + 0x8);
    if p08 != 0 {
        exe::call_va!(0x1_4204_db80, (p08) -> ());
        exe::call_va!(0x1_4244_2de0, (p08) -> ());
    }
    exe::call_va!(0x1_4205_6af0, (this) -> ());
}

/// `rend::rend_1422f9110`
/// Original: 0x1422f9110 (27 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f9110(val: u8) -> u32 {
    match val {
        0 => 0,
        1 => 1,
        _ => 2,
    }
}

/// `rend::rend_1422f9140`
/// Original: 0x1422f9140 (150 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422f9140(
    a0: u32,
    a1: u8,
    a2: u8,
    a3: f32,
    a4_bits: u32,
    a5: u8,
) {
    rend_1422f9140_impl(a0, a1, a2, a3.to_bits(), a4_bits, a5);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422f9140(
    a0: u32,
    a1: u8,
    a2: u8,
    _r9: usize,
    a4_bits: u32,
    a5: u8,
    _xmm0: f32,
    _xmm1: f32,
    _xmm2: f32,
    a3: f32,
) {
    rend_1422f9140_impl(a0, a1, a2, a3.to_bits(), a4_bits, a5);
}

#[inline(always)]
unsafe fn rend_1422f9140_impl(
    a0: u32,
    a1: u8,
    a2: u8,
    a3_bits: u32,
    a4_bits: u32,
    a5: u8,
) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let ctx = rd::<usize>(tls0 + 0x6338);
    exe::call_va!(
        0x1_4205_5020,
        (ctx, a1, a0, a2, a3_bits, a4_bits, a5) -> ()
    );
}

/// `rend::rend_1422f91e0`
/// Original: 0x1422f91e0 (106 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f91e0(a: i32, b: i32, c: i32) -> u64 {
    if c == 0 {
        return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd10) as usize);
    }
    if c == 9 {
        return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd18) as usize);
    }
    if c == 0xe {
        if b == 0 {
            return rd::<u64>(exe::ptr::<u64>(0x1_4397_fce8) as usize);
        }
    } else if c == 0x13 {
        return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd00) as usize);
    }
    match a {
        1 => rd::<u64>(exe::ptr::<u64>(0x1_4397_fce8) as usize),
        2 => {
            if c == 0x14 {
                rd::<u64>(exe::ptr::<u64>(0x1_4397_fd10) as usize)
            } else {
                rd::<u64>(exe::ptr::<u64>(0x1_4397_fce8) as usize)
            }
        }
        3 => rd::<u64>(exe::ptr::<u64>(0x1_4397_fcc8) as usize),
        _ => rd::<u64>(exe::ptr::<u64>(0x1_4397_fd00) as usize),
    }
}

/// `rend::rend_1422f9250`
/// Original: 0x1422f9250 (121 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422f9250(a: i32, b: i32, c: i32) -> u64 {
    if c != 0 && c != 9 {
        if c == 0xe {
            if b == 0 {
                return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd48) as usize);
            }
        } else {
            if c == 0x13 {
                return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd48) as usize);
            }
            if (c as u32).wrapping_sub(1) <= 2 {
                return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd68) as usize);
            }
            if (c as u32).wrapping_sub(4) <= 2 {
                return match a {
                    1 => rd::<u64>(exe::ptr::<u64>(0x1_4397_fda0) as usize),
                    2 => rd::<u64>(exe::ptr::<u64>(0x1_4397_fd90) as usize),
                    _ => rd::<u64>(exe::ptr::<u64>(0x1_4397_fd30) as usize),
                };
            }
        }
        match a {
            1 => return rd::<u64>(exe::ptr::<u64>(0x1_4397_fe48) as usize),
            2 => return rd::<u64>(exe::ptr::<u64>(0x1_4397_fd48) as usize),
            _ => {}
        }
    }
    rd::<u64>(exe::ptr::<u64>(0x1_4397_fd30) as usize)
}

/// `rend::rend_1422f92d0`
/// Original: 0x1422f92d0 (132 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422f92d0(mode: u32, dist: f32, bias: i32) -> u32 {
    rend_1422f92d0_impl(mode, dist, bias)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422f92d0(
    mode: u32,
    _rdx: usize,
    bias: i32,
    _xmm0: f32,
    dist: f32,
) -> u32 {
    rend_1422f92d0_impl(mode, dist, bias)
}

#[inline(always)]
unsafe fn rend_1422f92d0_impl(mode: u32, dist: f32, bias: i32) -> u32 {
    if mode > 9 {
        return 0;
    }
    let scale = rd::<f32>(exe::ptr::<f32>(0x1_4342_6f94) as usize);
    let denom = rd::<f32>(exe::ptr::<f32>(0x1_43a0_0148) as usize);
    let scaled = (dist * scale) / denom;
    #[cfg(target_arch = "x86_64")]
    let int_part = core::arch::x86_64::_mm_cvttss_si32(core::arch::x86_64::_mm_set_ss(scaled));
    #[cfg(not(target_arch = "x86_64"))]
    let int_part = crate::simd_ops::cvt_f32_i32(scaled) as i32;
    let mut val = 0xffffi32.wrapping_sub(int_part).wrapping_sub(bias);
    if val < 0 {
        0xc000_0000
    } else {
        if val > 0xffff {
            val = 0xffff;
        }
        (val as u32) | 0xc000_0000
    }
}

/// `rend::rend_1422f9640`
/// Original: 0x1422f9640 (128 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422f9640(val: f32, mode: i32) -> u8 {
    rend_1422f9640_impl(val, mode)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422f9640(_rcx: usize, mode: i32, val: f32) -> u8 {
    rend_1422f9640_impl(val, mode)
}

#[inline(always)]
unsafe fn rend_1422f9640_impl(val: f32, mode: i32) -> u8 {
    match mode {
        0 => {
            let t158 = rd::<f32>(exe::ptr::<f32>(0x1_4386_c158) as usize);
            if t158 >= val && t158 >= 0.0 {
                return 6;
            }
            let t138 = rd::<f32>(exe::ptr::<f32>(0x1_4386_c138) as usize);
            if t138 >= val {
                u8::from(t138 >= 0.0).wrapping_add(4)
            } else {
                4
            }
        }
        2 => {
            let t138 = rd::<f32>(exe::ptr::<f32>(0x1_4386_c138) as usize);
            if t138 >= 0.0 { 5 } else { 4 }
        }
        3 => {
            let t158 = rd::<f32>(exe::ptr::<f32>(0x1_4386_c158) as usize);
            if t158 >= 0.0 { 6 } else { 4 }
        }
        _ => 4,
    }
}

/// `rend::rend_1422fc110`
/// Original: 0x1422fc110 (79 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fc110(this: usize, target: u64) {
    let mut cur = rd::<usize>(this + 0x60);
    let mut len = rd::<u32>(this + 0x68) as usize;
    if cur != cur.wrapping_add(len.wrapping_mul(8)) {
        let mut base = cur;
        loop {
            if rd::<u64>(cur) == target {
                let last = base.wrapping_add(len.wrapping_sub(1).wrapping_mul(8));
                if cur != last {
                    wr::<u64>(cur, rd::<u64>(last));
                }
                let new_len = rd::<u32>(this + 0x68).wrapping_sub(1);
                wr::<u32>(this + 0x68, new_len);
                len = new_len as usize;
            } else {
                cur = cur.wrapping_add(8);
            }
            base = rd::<usize>(this + 0x60);
            if cur == base.wrapping_add(len.wrapping_mul(8)) {
                break;
            }
        }
    }
}

/// `rend::rend_1422fc1d0`
/// Original: 0x1422fc1d0 (136 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fc1d0(_this: usize, state: usize) {
    let count = rd::<u8>(state + 0x192);
    let mut i: usize = 0;
    if rd::<u8>(state + 0x193) != 0 {
        if count != 0 {
            loop {
                exe::call_va!(0x1_4205_da30, () -> ());
                i = i.wrapping_add(1);
                if i >= rd::<u8>(state + 0x192) as usize {
                    break;
                }
            }
        }
    } else if count != 0 {
        loop {
            exe::call_va!(0x1_4205_f070, () -> ());
            i = i.wrapping_add(1);
            if i >= rd::<u8>(state + 0x192) as usize {
                break;
            }
        }
    }
    wr::<u8>(state + 0x194, 1);
}

/// `rend::rend_1422fcae0`
/// Original: 0x1422fcae0 (238 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fcae0(this: usize) -> usize {
    for off in (0..0x40).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<u8>(this + 0x40, 0);
    wr::<[u8; 32]>(this + 0x48, [0u8; 32]);

    const ELEMS: [(u32, u64); 8] = [
        (0x0102_0302, 0x10),
        (0x0102_0302, 0x10),
        (0x0102_0302, 0x10),
        (0x0102_0f02, 0x4),
        (0x0102_0f02, 0x4),
        (0x0102_0302, 0x10),
        (0x0102_0302, 0x10),
        (0x0102_0302, 0x10),
    ];
    for (desc, sz) in ELEMS {
        let idx = rd::<u8>(this + 0x40) as usize;
        wr::<u32>(this + idx * 4, desc);
        wr::<u8>(this + 0x40, rd::<u8>(this + 0x40).wrapping_add(1));
        wr::<u64>(this + 0x58, rd::<u64>(this + 0x58).wrapping_add(sz));
    }
    this
}

/// `rend::rend_1422fd680`
/// Original: 0x1422fd680 (159 bytes, Throws::No, FLOAT_RETURN)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422fd680(this: usize, dist: f32, camera: usize) -> f32 {
    rend_1422fd680_impl(this, dist, camera)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422fd680(
    this: usize,
    _rdx: usize,
    camera: usize,
    _xmm0: f32,
    dist: f32,
) -> f32 {
    rend_1422fd680_impl(this, dist, camera)
}

#[inline(always)]
unsafe fn rend_1422fd680_impl(this: usize, dist: f32, camera: usize) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let mut mat_50 = [0.0f32; 16];
    let mut mat_20 = [0.0f32; 12];
    let _ = exe::call_va!(
        0x1_403b_3de0,
        (mat_50.as_mut_ptr() as usize, camera.wrapping_add(0xd0)) -> usize
    );
    let _ = exe::call_va!(
        0x1_40f6_c9e0,
        (mat_20.as_mut_ptr() as usize, camera.wrapping_add(0x10)) -> usize
    );
    let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_647c) as usize);
    let ratio = if eps < dist {
        (mat_50[0] * rd::<f32>(this + 0x3a4)) / dist
    } else {
        rd::<f32>(exe::ptr::<f32>(0x1_4342_68fc) as usize)
    };
    let sub = ratio - rd::<f32>(exe::ptr::<f32>(0x1_4342_6564) as usize);
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let clamped_low = if sub > 0.0f32 { sub } else { 0.0f32 };
    let clamped = if clamped_low < one { clamped_low } else { one };
    ((one - clamped) * rd::<f32>(this + 0x3ac)) + clamped
}

/// `rend::rend_1422fd730`
/// Original: 0x1422fd730 (33 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fd730(this: usize) -> u64 {
    let num: u64 = if rd::<u8>(this + 0x193) != 0 {
        0x80000
    } else {
        0x100000
    };
    let denom = rd::<u16>(this + 0x190) as u64;
    num / denom
}

/// `rend::rend_1422fd760`
/// Original: 0x1422fd760 (17 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fd760(this: usize, idx: u8) -> u64 {
    let off = (idx as usize).wrapping_mul(0x60).wrapping_add(0x50);
    rd::<u64>(this.wrapping_add(off))
}

#[inline]
unsafe fn atomic_dec_i32_at(addr: usize) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        let mut prev: i32 = -1;
        core::arch::asm!(
            "lock xadd dword ptr [{p}], {v:e}",
            p = in(reg) addr,
            v = inout(reg) prev,
            options(nostack),
        );
        prev
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        if (addr & 3) == 0 {
            (*(addr as *const core::sync::atomic::AtomicI32))
                .fetch_sub(1, core::sync::atomic::Ordering::SeqCst)
        } else {
            let prev = rd::<i32>(addr);
            wr::<i32>(addr, prev.wrapping_sub(1));
            prev
        }
    }
}

#[inline]
unsafe fn release_particle_ref_slot(obj: usize) {
    if obj != 0 {
        let prev = atomic_dec_i32_at(obj.wrapping_add(0x24));
        if prev == 1 {
            let inner = rd::<usize>(obj + 0x28);
            if inner != 0 {
                exe::call_va!(0x1_4241_d6f0, (inner) -> ());
            }
        }
    }
}

/// `rend::rend_1422fd780`
/// Original: 0x1422fd780 (208 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fd780(this: usize, item: usize) {
    if item != 0 {
        let pool = rd::<usize>(this + 0x58);
        release_particle_ref_slot(rd::<usize>(item + 0x1e0));
        wr::<u64>(item + 0x1e0, 0);
        release_particle_ref_slot(rd::<usize>(item + 0x1d8));
        wr::<u64>(item + 0x1d8, 0);
        release_particle_ref_slot(rd::<usize>(item + 0x1d0));
        wr::<u64>(item + 0x1d0, 0);
        exe::call_va!(0x1_4243_b3e0, (pool, item) -> ());
    }
}

/// `rend::rend_1422fde70`
/// Original: 0x1422fde70 (159 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_1422fde70(this: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize);
    let mut i: u32 = 0;
    if rd::<u32>(this + 0x20) != 0 {
        let mut idx: usize = 0;
        loop {
            let off = idx.wrapping_mul(0x70);
            let base = rd::<usize>(this + 0x18);
            let elem = base.wrapping_add(off);
            if (rd::<u8>(elem + 0x58) & 2) == 0 {
                let kind = rd::<u32>(elem + 0x54);
                if kind == 0 {
                    let h = rd::<u64>(elem + 0x60);
                    exe::call_va!(0x1_4225_8080, (mgr, h) -> ());
                } else if kind == 1 {
                    let h = rd::<u64>(elem + 0x60);
                    exe::call_va!(0x1_4225_7af0, (mgr, h) -> ());
                }
            }
            i = i.wrapping_add(1);
            idx = (i as i32) as usize;
            if idx >= (rd::<u32>(this + 0x20) as usize) {
                break;
            }
        }
        wr::<u32>(this + 0x20, 0);
        return;
    }
    wr::<u32>(this + 0x20, 0);
}

/// `rend::rend_1422fe070`
/// Original: 0x1422fe070 (65 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe070(this: usize, name_hash: u64, arg2: u64) {
    let entry = exe::call_va!(0x1_4131_00d0, (this.wrapping_add(0x8)) -> usize);
    wr::<u64>(entry, name_hash);
    wr::<u64>(entry + 0x8, arg2);
    wr::<u64>(entry + 0x54, 2);
    wr::<u64>(entry + 0x60, 0);
    wr::<u64>(entry + 0x68, u64::MAX);
}

/// `rend::rend_1422fe0c0`
/// Original: 0x1422fe0c0 (93 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe0c0(
    this: usize,
    name_hash: u64,
    desc: usize,
    flags: u32,
) {
    let entry = exe::call_va!(0x1_4131_00d0, (this.wrapping_add(0x8)) -> usize);
    let b16 = rd::<[u8; 16]>(desc);
    let d10 = rd::<u32>(desc + 0x10);
    wr::<u64>(entry, name_hash);
    wr::<[u8; 16]>(entry + 0x40, b16);
    wr::<u32>(entry + 0x50, d10);
    wr::<u32>(entry + 0x54, 1);
    wr::<u32>(entry + 0x58, flags);
    wr::<u64>(entry + 0x60, 0);
    wr::<u64>(entry + 0x68, u64::MAX);
}

/// `rend::rend_1422fe130`
/// Original: 0x1422fe130 (94 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe130(
    this: usize,
    name_hash: u64,
    desc: usize,
    flags: u32,
) {
    let entry = exe::call_va!(0x1_4131_00d0, (this.wrapping_add(0x8)) -> usize);
    let b32 = rd::<[u8; 32]>(desc);
    let b16 = rd::<[u8; 16]>(desc + 0x20);
    wr::<[u8; 32]>(entry + 0x10, b32);
    wr::<[u8; 16]>(entry + 0x30, b16);
    wr::<u64>(entry, name_hash);
    wr::<u32>(entry + 0x54, 0);
    wr::<u32>(entry + 0x58, flags);
    wr::<u64>(entry + 0x60, 0);
    wr::<u64>(entry + 0x68, u64::MAX);
}

/// `rend::rend_1422fe1a0`
/// Original: 0x1422fe1a0 (43 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe1a0(this: usize, key: u64) -> u32 {
    let count = rd::<u32>(this + 0x10) as usize;
    let mut i: u32 = 0;
    if count != 0 {
        let base = rd::<usize>(this + 0x8);
        let mut idx: usize = 0;
        loop {
            let elem = base.wrapping_add(idx.wrapping_mul(0x70));
            if rd::<u64>(elem) == key {
                return i;
            }
            i = i.wrapping_add(1);
            idx = (i as i32) as usize;
            if idx >= count {
                break;
            }
        }
    }
    0xffff_ffff
}

/// `rend::rend_1422fe1e0`
/// Original: 0x1422fe1e0 (134 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe1e0(this: usize, handle: u64) {
    wr::<u8>(this, rd::<u8>(exe::ptr::<u8>(0x1_4386_c190) as usize));
    wr::<u32>(this + 0x10, 0);
    let entry = exe::call_va!(0x1_4131_00d0, (this.wrapping_add(0x8)) -> usize);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for off in 0..11usize {
        let b = (rd::<i8>((exe::ptr::<u8>(0x1_433f_a5c8) as usize) + off) as i64) as u64;
        hash = (hash ^ b).wrapping_mul(0x0100_0000_01b3);
    }
    wr::<u64>(entry, hash);
    wr::<u32>(entry + 0x54, 0);
    wr::<u32>(entry + 0x58, 4);
    wr::<u64>(entry + 0x60, handle);
    wr::<u64>(entry + 0x68, u64::MAX);
}

/// `rend::rend_1422fe270`
/// Original: 0x1422fe270 (118 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe270(this: usize, handle: u64) {
    let entry = exe::call_va!(0x1_4131_00d0, (this.wrapping_add(0x8)) -> usize);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for off in 0..11usize {
        let b = (rd::<i8>((exe::ptr::<u8>(0x1_433f_a5c8) as usize) + off) as i64) as u64;
        hash = (hash ^ b).wrapping_mul(0x0100_0000_01b3);
    }
    wr::<u64>(entry, hash);
    wr::<u32>(entry + 0x54, 0);
    wr::<u32>(entry + 0x58, 4);
    wr::<u64>(entry + 0x60, handle);
    wr::<u64>(entry + 0x68, u64::MAX);
}

/// `rend::rend_1422fe2f0`
/// Original: 0x1422fe2f0 (25 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fe2f0(this: usize) -> usize {
    wr::<u8>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    this
}

/// `rend::rend_1422ff4f0`
/// Original: 0x1422ff4f0 (190 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422ff4f0(_this: usize, key: u64) -> u32 {
    let mut i: u32 = 0;
    let p_a0 = exe::ptr::<usize>(0x1_43bb_d1a0) as usize;
    let p_a8 = exe::ptr::<u32>(0x1_43bb_d1a8) as usize;
    let p_ac = exe::ptr::<u32>(0x1_43bb_d1ac) as usize;
    let len0 = rd::<u32>(p_a8);
    let mut base = rd::<usize>(p_a0);
    if len0 != 0 {
        let mut idx: usize = 0;
        loop {
            if rd::<u64>(base.wrapping_add(idx.wrapping_mul(16))) == key {
                if i != 0xffff_ffff {
                    return i;
                }
                break;
            }
            i = i.wrapping_add(1);
            idx = (i as i32) as usize;
            if idx >= (len0 as usize) {
                break;
            }
        }
    }
    let cap = rd::<u32>(p_ac);
    if ((len0 as u64) << 4) == ((cap as u64) << 4) {
        exe::call_va!(
            0x1_401f_c320,
            (
                p_a0,
                (len0 as u64).wrapping_add(1)
            ) -> ()
        );
    }
    let cur_len = rd::<u32>(p_a8) as usize;
    base = rd::<usize>(p_a0);
    wr::<[u8; 16]>(base.wrapping_add(cur_len.wrapping_mul(16)), [0u8; 16]);
    let next_len = rd::<u32>(p_a8).wrapping_add(1);
    wr::<u32>(p_a8, next_len);
    let end_ptr = rd::<usize>(p_a0).wrapping_add((next_len as usize).wrapping_mul(16));
    wr::<u64>(end_ptr.wrapping_sub(0x10), key);
    wr::<u32>(end_ptr.wrapping_sub(0x8), 0xffff_ffff);
    rd::<u32>(p_a8).wrapping_sub(1)
}

/// `rend::rend_1422fffa0`
/// Original: 0x1422fffa0 (102 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422fffa0() {
    let p_170 = exe::ptr::<usize>(0x1_43bb_d170) as usize;
    let p_178 = exe::ptr::<u64>(0x1_43bb_d178) as usize;
    let p_180 = exe::ptr::<usize>(0x1_43bb_d180) as usize;
    let mut r10 = rd::<u64>(p_178);
    let mut i: u32 = 0;
    let mut prev_id: i32 = -1;
    if (r10 as u32) != 0 {
        let mut idx: usize = 0;
        loop {
            let base_170 = rd::<usize>(p_170);
            let cur_id = rd::<i32>(base_170.wrapping_add(idx.wrapping_mul(0x30)));
            if cur_id != prev_id {
                prev_id = cur_id;
                if cur_id != -1 {
                    let dst = rd::<usize>(p_180)
                        .wrapping_add(((cur_id as isize) as usize).wrapping_mul(0x2e0));
                    wr::<u32>(dst + 0x4, i);
                    r10 = rd::<u64>(p_178);
                }
            }
            i = i.wrapping_add(1);
            idx = (i as i32) as usize;
            if idx >= ((r10 as u32) as usize) {
                break;
            }
        }
    }
}

/// `rend::rend_142300010`
/// Original: 0x142300010 (23 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142300010(this: usize, val: u64) {
    let idx = (rd::<i32>(this + 0x28) as isize) as usize;
    let base = rd::<usize>(this + 0x40);
    let dst = base.wrapping_add(idx.wrapping_mul(0x2e0)).wrapping_add(0x2d0);
    wr::<u64>(dst, val);
}

/// `rend::rend_142300030`
/// Original: 0x142300030 (202 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142300030(this: usize, val: u32) {
    let old_len = rd::<u32>(this + 0x58);
    let cap = rd::<u32>(this + 0x5c);
    if ((old_len as u64) << 3) == ((cap as u64) << 3) {
        exe::call_va!(0x1_4020_bcb0, (this.wrapping_add(0x50), (old_len as u64).wrapping_add(1)) -> ());
    }
    let cur_len = rd::<u32>(this + 0x58) as usize;
    let base = rd::<usize>(this + 0x50);
    wr::<u64>(base.wrapping_add(cur_len.wrapping_mul(8)), 0);
    let new_len = rd::<u32>(this + 0x58).wrapping_add(1);
    wr::<u32>(this + 0x58, new_len);
    let new_len_usize = rd::<u32>(this + 0x58) as usize;
    let base_after = rd::<usize>(this + 0x50);
    let slot = base_after.wrapping_add(new_len_usize.wrapping_mul(8));
    wr::<u32>(slot.wrapping_sub(8), val);
    wr::<u32>(slot.wrapping_sub(4), rd::<u32>(this + 0x28));
    if rd::<u32>(this + 0x58) < 0x800 {
        let idx = (rd::<i32>(this + 0x28) as isize) as usize;
        let entry = rd::<usize>(this + 0x40).wrapping_add(idx.wrapping_mul(0x2e0));
        let bit_pos = ((old_len as i32) as i64) as u64;
        if bit_pos >= 0x800 {
            exe::call_va!(0x1_4131_03a0, () -> ());
            return;
        }
        let word_off = ((bit_pos >> 6) as usize).wrapping_mul(8);
        let word_addr = entry.wrapping_add(word_off).wrapping_add(0x1d0);
        let mask = 1u64 << (bit_pos & 0x3f);
        wr::<u64>(word_addr, rd::<u64>(word_addr) | mask);
    } else if rd::<u8>(this + 0x74) != 0 {
        wr::<u8>(this + 0x74, 0);
    }
}

/// `rend::rend_142300100`
/// Original: 0x142300100 (214 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142300100(this: usize) {
    let inner = rd::<usize>(this + 0x8);
    let old_len = rd::<u32>(inner + 0x58);
    let cap = rd::<u32>(inner + 0x5c);
    if ((old_len as u64) << 3) == ((cap as u64) << 3) {
        exe::call_va!(0x1_4020_bcb0, (inner.wrapping_add(0x50), (old_len as u64).wrapping_add(1)) -> ());
    }
    let cur_len = rd::<u32>(inner + 0x58) as usize;
    let base = rd::<usize>(inner + 0x50);
    wr::<u64>(base.wrapping_add(cur_len.wrapping_mul(8)), 0);
    let new_len = rd::<u32>(inner + 0x58).wrapping_add(1);
    wr::<u32>(inner + 0x58, new_len);
    let new_len_usize = rd::<u32>(inner + 0x58) as usize;
    let base_after = rd::<usize>(inner + 0x50);
    let slot = base_after.wrapping_add(new_len_usize.wrapping_mul(8));
    wr::<u32>(slot.wrapping_sub(8), 0x42);
    wr::<u32>(slot.wrapping_sub(4), rd::<u32>(this));
    let inner2 = rd::<usize>(this + 0x8);
    if rd::<u32>(inner2 + 0x58) < 0x800 {
        let idx = (rd::<i32>(this) as isize) as usize;
        let entry = rd::<usize>(inner2 + 0x40).wrapping_add(idx.wrapping_mul(0x2e0));
        let bit_pos = ((old_len as i32) as i64) as u64;
        if bit_pos >= 0x800 {
            exe::call_va!(0x1_4131_03a0, () -> ());
            return;
        }
        let word_off = ((bit_pos >> 6) as usize).wrapping_mul(8);
        let word_addr = entry.wrapping_add(word_off).wrapping_add(0x1d0);
        let mask = 1u64 << (bit_pos & 0x3f);
        wr::<u64>(word_addr, rd::<u64>(word_addr) | mask);
    } else if rd::<u8>(inner2 + 0x74) != 0 {
        wr::<u8>(inner2 + 0x74, 0);
    }
}

/// `rend::rend_1423001e0`
/// Original: 0x1423001e0 (16 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1423001e0(this: usize) -> u32 {
    let old = rd::<u32>(this + 0xa0);
    wr::<u32>(this + 0xa0, old.wrapping_add(1));
    old
}

/// `rend::rend_142300210`
/// Original: 0x142300210 (46 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142300210(this: usize) {
    let inner = rd::<usize>(this + 0x8);
    let slot = exe::call_va!(0x1_4131_03c0, (inner.wrapping_add(0x30)) -> usize);
    wr::<u32>(slot, rd::<u32>(this));
    wr::<u32>(slot + 0x4, 1);
    wr::<u32>(slot + 0x8, 0x56);
}

/// `rend::rend_142300250`
/// Original: 0x142300250 (82 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142300250(_this: usize, fn_obj: usize) {
    let idx = (rd::<i32>(exe::ptr::<i32>(0x1_43bb_d168) as usize) as isize) as usize;
    let dst = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d180) as usize)
        .wrapping_add(idx.wrapping_mul(0x2e0))
        .wrapping_add(0x90);
    let _ = exe::call_va!(0x1_4128_cba0, (dst, fn_obj) -> usize);
    let target = rd::<usize>(fn_obj + 0x38);
    if target != 0 {
        let vtbl = rd::<usize>(target);
        let is_heap = u8::from(target != fn_obj);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (target, is_heap) -> ());
        wr::<u64>(fn_obj + 0x38, 0);
    }
}

/// `rend::rend_1423002b0`
/// Original: 0x1423002b0 (76 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1423002b0(this: usize, fn_obj: usize) {
    let idx = (rd::<i32>(this + 0x28) as isize) as usize;
    let dst = rd::<usize>(this + 0x40)
        .wrapping_add(idx.wrapping_mul(0x2e0))
        .wrapping_add(0x50);
    let _ = exe::call_va!(0x1_4128_cba0, (dst, fn_obj) -> usize);
    let target = rd::<usize>(fn_obj + 0x38);
    if target != 0 {
        let vtbl = rd::<usize>(target);
        let is_heap = u8::from(target != fn_obj);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (target, is_heap) -> ());
        wr::<u64>(fn_obj + 0x38, 0);
    }
}

/// `rend::rend_14226c9b0`
/// Original: 0x14226c9b0 (773 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14226c9b0(
    this: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mode = rd::<u8>(param_2 + 0x08);
    exe::call_va!(0x1_4226_cd10, (mode as u64));

    let srw_lock = exe::ptr::<u64>(0x1_4385_83b8) as u64;
    let _ = exe::call_iat!(0x1_4264_55a8, (srw_lock) -> u64);
    let g_cnt_ptr = exe::ptr::<u32>(0x1_4385_83b0) as usize;
    let cnt = rd::<u32>(g_cnt_ptr);
    if cnt != 0 {
        let cap = rd::<u32>(exe::ptr::<u32>(0x1_4385_83b4) as usize);
        let head_ptr = exe::ptr::<u32>(0x1_4385_83a8) as usize;
        let head = rd::<u32>(head_ptr);
        let slot = if cap != 0 { head % cap } else { 0 };
        let ring_base = rd::<usize>(exe::ptr::<usize>(0x1_4385_83a0) as usize);
        let val = rd::<u64>(ring_base.wrapping_add((slot as usize) * 8));
        wr::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize, val);
        let next_head = head.wrapping_add(1);
        wr::<u32>(head_ptr, if next_head == cap { 0 } else { next_head });
        wr::<u32>(g_cnt_ptr, cnt.wrapping_sub(1));
    }
    let _ = exe::call_iat!(0x1_4264_5598, (srw_lock) -> u64);

    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) != 0 {
        let tok = rd::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize);
        let fn_slot = exe::ptr::<u64>(0x1_43c8_c608) as usize;
        let mut fn_ptr = rd::<u64>(fn_slot);
        let mut ok = fn_ptr != 0;
        if !ok {
            let name_ptr = exe::ptr::<u8>(0x1_433d_d168) as u64;
            let rc = exe::call_iat!(0x1_4264_7308, (4u64, name_ptr, fn_slot as u64) -> u32);
            if rc == 0 {
                fn_ptr = rd::<u64>(fn_slot);
                ok = true;
            }
        }
        if ok && fn_ptr != 0 {
            exe::call_va!(fn_ptr, (2u64, tok));
        }
    }

    let flag_ce = rd::<u8>(exe::ptr::<u8>(0x1_4385_83ce) as usize);
    let flag_2c0 = rd::<u8>(exe::ptr::<u8>(0x1_4385_82c0) as usize);
    wr::<u8>(
        exe::ptr::<u8>(0x1_4385_83cf) as usize,
        u8::from(flag_ce != 0 && flag_2c0 != 0),
    );

    if rd::<u8>(param_2 + 0x08) != 0 {
        let idx = rd::<u32>(param_2);
        let arg3 = if idx != 0xffff_ffff {
            let tbl = exe::ptr::<u64>(0x1_43bf_6108) as usize;
            rd::<u64>(tbl.wrapping_add((idx as usize) * 8))
        } else {
            0
        };
        exe::call_va!(0x1_4229_86e0, (this, param_3, arg3, param_4));
    } else {
        let p48 = rd::<usize>(this + 0x48);
        let f_50 = rd::<u32>(p48 + 0x50) as u128;
        let f_glob = rd::<u32>(exe::ptr::<u32>(0x1_4384_1638) as usize) as u128;
        let b_4c = rd::<u8>(p48 + 0x4c) as u64;
        let p_f8 = rd::<u64>(this + 0xf8);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4220_2be0,
            p_f8,
            b_4c,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, f_50, f_glob],
        );

        let p450 = rd::<usize>(this + 0x450);
        wr::<u64>(this + 0x450, 0);
        if p450 != 0 {
            exe::call_va!(0x1_4209_d670, (p450));
            exe::call_va!(0x1_411e_7fc0, (p450 + 0x58));
            exe::call_va!(0x1_4244_2de0, (p450));
        }
        let p458 = rd::<usize>(this + 0x458);
        wr::<u64>(this + 0x458, 0);
        if p458 != 0 {
            exe::call_va!(0x1_4130_2d60, (0u64, p458));
        }

        let p1a8 = rd::<usize>(this + 0x1a8);
        exe::call_va!(0x1_4226_1520, (p1a8));
        exe::call_va!(0x1_4226_11b0, (rd::<usize>(this + 0x1a8)));
        exe::call_va!(0x1_412f_7fb0, (rd::<usize>(this + 0x1a8) + 0x38));
        exe::call_va!(0x1_4226_2860, (rd::<usize>(this + 0x1a8)));
        if rd::<u8>(exe::ptr::<u8>(0x1_4397_d8a7) as usize) == 0 {
            let v20 = rd::<u32>(rd::<usize>(this + 0x1a8) + 0x20);
            if v20 != 0 {
                exe::call_va!(0x1_4225_9da0, (0u64, v20 as u64));
                exe::call_va!(0x1_4225_9370, (0u64, v20 as u64));
            }
        }
        exe::call_va!(0x1_4226_26d0, (rd::<usize>(this + 0x1a8)));
        exe::call_va!(0x1_4225_91f0, (rd::<usize>(this + 0x1a8)));
        exe::call_va!(0x1_4230_be30, (rd::<usize>(this + 0x1e8)));
        let cur_1a8 = rd::<usize>(this + 0x1a8);
        let flag_8ca = rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize) as u64;
        exe::call_va!(
            0x1_4222_acb0,
            (rd::<u64>(exe::ptr::<u64>(0x1_4397_fed0) as usize))
        );
        exe::call_va!(0x1_4225_c0f0, (cur_1a8, flag_8ca));
        exe::call_va!(0x1_4226_2310, (rd::<usize>(this + 0x1a8)));
        exe::call_va!(0x1_4226_1cf0, (rd::<usize>(this + 0x1a8)));
    }

    if rd::<u8>(param_2 + 0x09) != 0 {
        exe::call_va!(0x1_4226_bb40, (this));
    }
    exe::call_va!(0x1_4226_a820, (this));

    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) != 0 {
        let tok = rd::<u64>(exe::ptr::<u64>(0x1_4385_8398) as usize);
        let fn_slot = exe::ptr::<u64>(0x1_43c8_c608) as usize;
        let mut fn_ptr = rd::<u64>(fn_slot);
        let mut ok = fn_ptr != 0;
        if !ok {
            let name_ptr = exe::ptr::<u8>(0x1_433d_d168) as u64;
            let rc = exe::call_iat!(0x1_4264_7308, (4u64, name_ptr, fn_slot as u64) -> u32);
            if rc == 0 {
                fn_ptr = rd::<u64>(fn_slot);
                ok = true;
            }
        }
        if ok && fn_ptr != 0 {
            exe::call_va!(fn_ptr, (3u64, tok));
        }
    }

    let g_counter_ptr = exe::ptr::<i32>(0x1_43a2_b19c) as usize;
    let g_counter = rd::<i32>(g_counter_ptr);
    if g_counter > 0 && rd::<u8>(param_2 + 0x08) == 1 {
        wr::<i32>(g_counter_ptr, g_counter.wrapping_sub(1));
        let mut sp = [0u64; 2];
        let ret_ptr = exe::call_va!(0x1_4207_34e0, (sp.as_mut_ptr() as usize, 0u64) -> usize);
        let ctrl = rd::<u64>(ret_ptr + 8);
        if ctrl != 0 {
            exe::call_va!(0x1_4207_02c0, (ctrl, rd::<u64>(ret_ptr)));
        }
    }
}

/// `rend::rend_1422fd9c0`
/// Original: 0x1422fd9c0 (350 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_1422fd9c0(this: usize, pass_idx: i32) {
    let mgr = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize);
    let mut i = 0u32;
    while (i as usize) < (rd::<u32>(this + 0x10) as usize) {
        let base = rd::<usize>(this + 0x08);
        let off = (i as usize).wrapping_mul(0x70);
        let entry = base.wrapping_add(off);
        let flags = rd::<u32>(entry + 0x58);
        let handle = rd::<u64>(entry + 0x60);
        let bit4 = ((flags >> 4) & 1) != 0;
        if handle == 0 {
            let first_pass = rd::<i32>(entry + 0x68);
            if first_pass != -1
                && pass_idx == first_pass
                && (flags & 4) == 0
                && (!bit4 || ((flags >> 3) & 1) == 0)
            {
                let kind = rd::<i32>(entry + 0x54);
                let tag_ptr = exe::ptr::<u8>(0x1_426c_dcb0) as u64;
                if kind == 0 {
                    let mut out_h = 0u64;
                    exe::call_va!(
                        0x1_4225_81e0,
                        (mgr, (entry + 0x10) as u64, tag_ptr, (&mut out_h as *mut u64) as u64)
                    );
                    let cur_base = rd::<usize>(this + 0x08);
                    wr::<u64>(cur_base.wrapping_add(off).wrapping_add(0x60), out_h);
                } else if kind == 1 {
                    let h = exe::call_va!(
                        0x1_4225_7c60,
                        (mgr, (entry + 0x40) as u64, tag_ptr) -> u64
                    );
                    let cur_base = rd::<usize>(this + 0x08);
                    wr::<u64>(cur_base.wrapping_add(off).wrapping_add(0x60), h);
                }
            }
        } else {
            let last_pass = rd::<i32>(entry + 0x6c);
            if last_pass != -1 && pass_idx > last_pass && (flags & 4) == 0 && !bit4 {
                let kind = rd::<i32>(entry + 0x54);
                if kind == 0 {
                    exe::call_va!(0x1_4225_8080, (mgr, handle));
                } else if kind == 1 {
                    exe::call_va!(0x1_4225_7af0, (mgr, handle));
                }
                let cur_base = rd::<usize>(this + 0x08);
                wr::<u64>(cur_base.wrapping_add(off).wrapping_add(0x60), 0);
            }
        }
        i = i.wrapping_add(1);
    }
}

/// `rend::rend_1422ff5c0`
/// Original: 0x1422ff5c0 (1634 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1422ff5c0() {
    let g_order_ptr = exe::ptr::<u64>(0x1_43bb_d148) as usize;
    let g_passes_ptr = exe::ptr::<u64>(0x1_43bb_d180) as usize;
    let g_target_pass = exe::ptr::<i32>(0x1_43bb_d1b0) as usize;
    let g_valid_flag = exe::ptr::<u8>(0x1_43bb_d1b4) as usize;

    wr::<u32>(g_order_ptr + 8, 0);
    if rd::<u32>(g_passes_ptr + 8) != 0 {
        let mut i = 0u32;
        loop {
            let base = rd::<usize>(g_passes_ptr);
            wr::<u32>(base.wrapping_add((i as usize).wrapping_mul(0x2e0)).wrapping_add(8), 0);
            i = i.wrapping_add(1);
            let cnt = rd::<u32>(g_passes_ptr + 8);
            if (i as usize) >= (cnt as usize) {
                break;
            }
        }
    }

    let mut accum_or = [0u64; 32];
    let mut satisfied = [0u64; 32];

    loop {
        let cnt = rd::<u32>(g_passes_ptr + 8) as usize;
        if cnt == 0 {
            wr::<u8>(g_valid_flag, 0);
            return;
        }
        let base = rd::<usize>(g_passes_ptr);
        let mut found: Option<usize> = None;
        for idx in 0..cnt {
            let entry = base.wrapping_add(idx.wrapping_mul(0x2e0));
            if rd::<u32>(entry + 8) == 0 {
                let mut ok = true;
                for w in 0..32 {
                    let req = rd::<u64>(entry + 0xd0 + w * 8);
                    if (satisfied[w] & req) != req {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    found = Some(idx);
                    break;
                }
            }
        }
        let Some(idx) = found else {
            wr::<u8>(g_valid_flag, 0);
            return;
        };

        let cur_len = rd::<u32>(g_order_ptr + 8) as u64;
        let cur_cap = rd::<u32>(g_order_ptr + 12) as u64;
        if cur_len == cur_cap {
            let next_len = cur_len + 1;
            let mut new_cap = cur_cap + (cur_cap >> 1);
            if new_cap < next_len {
                new_cap = next_len;
            }
            exe::call_va!(0x1_401d_eb40, (g_order_ptr, new_cap));
        }
        let order_buf = rd::<usize>(g_order_ptr);
        let len_now = rd::<u32>(g_order_ptr + 8);
        wr::<u32>(order_buf.wrapping_add((len_now as usize) * 4), idx as u32);
        wr::<u32>(g_order_ptr + 8, len_now.wrapping_add(1));

        let entry = base.wrapping_add(idx.wrapping_mul(0x2e0));
        for w in 0..32 {
            accum_or[w] |= rd::<u64>(entry + 0x1d0 + w * 8);
        }
        wr::<u32>(entry + 8, 2);
        if (idx as i32) == rd::<i32>(g_target_pass) {
            return;
        }
        satisfied = accum_or;
    }
}

/// `rend::rend_142300560`
/// Original: 0x142300560 (383 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142300560(this: usize, pass_id: i32) {
    wr::<i32>(this + 0x28, -1);
    let count = rd::<u32>(this + 0x48);
    if count != 0 {
        let base = rd::<usize>(this + 0x40);
        for i in 0..count {
            if rd::<i32>(base.wrapping_add((i as usize).wrapping_mul(0x2e0))) == pass_id {
                wr::<u32>(this + 0x28, i);
                break;
            }
        }
    }
    if rd::<i32>(this + 0x28) == -1 {
        wr::<u32>(this + 0x28, count);
        let new_entry = exe::call_va!(0x1_4131_0450, (this + 0x40) -> usize);
        wr::<i32>(new_entry, pass_id);
        core::ptr::write_bytes((new_entry + 0xd0) as *mut u8, 0, 0x210);
    }
}

/// `rend::rend_142300ab0`
/// Original: 0x142300ab0 (694 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_142300ab0() -> u8 {
    let g_edges_ptr = exe::ptr::<u64>(0x1_43bb_d170) as usize;
    let g_passes_ptr = exe::ptr::<u64>(0x1_43bb_d180) as usize;
    let g_res_ptr = exe::ptr::<u64>(0x1_43bb_d190) as usize;
    let g_valid_flag = exe::ptr::<u8>(0x1_43bb_d1b4) as usize;

    let mut begin_ptr = rd::<u64>(g_edges_ptr);
    let init_cnt = rd::<u32>(g_edges_ptr + 8) as u64;
    let byte_span = init_cnt.wrapping_mul(0x30);
    let mut end_ptr = begin_ptr.wrapping_add(byte_span);
    let r9_byte = (end_ptr & 0xff) as u64;
    exe::call_va!(
        0x1_4131_0580,
        (
            (&mut begin_ptr as *mut u64) as usize,
            (&mut end_ptr as *mut u64) as usize,
            init_cnt,
            r9_byte
        )
    );

    if rd::<u32>(g_edges_ptr + 8) != 0 {
        let mut prev_pass: i32 = -1;
        let mut i = 0u32;
        while (i as usize) < (rd::<u32>(g_edges_ptr + 8) as usize) {
            let edges_base = rd::<usize>(g_edges_ptr);
            let pass_id = rd::<i32>(edges_base.wrapping_add((i as usize).wrapping_mul(0x30)));
            if pass_id != prev_pass {
                prev_pass = pass_id;
                if pass_id != -1 {
                    let passes_base = rd::<usize>(g_passes_ptr);
                    wr::<u32>(
                        passes_base
                            .wrapping_add((pass_id as isize as usize).wrapping_mul(0x2e0))
                            .wrapping_add(4),
                        i,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    }

    if rd::<u32>(g_passes_ptr + 8) != 0 {
        let mut pass_idx = 0u32;
        while (pass_idx as usize) < (rd::<u32>(g_passes_ptr + 8) as usize) {
            let passes_base = rd::<usize>(g_passes_ptr);
            let pass_entry = passes_base.wrapping_add((pass_idx as usize).wrapping_mul(0x2e0));
            core::ptr::write_bytes((pass_entry + 0xd0) as *mut u8, 0, 0x100);

            let mut edge_idx = rd::<i32>(pass_entry + 4) as isize;
            let mut edge_cnt = rd::<u32>(g_edges_ptr + 8) as usize;
            let mut pass_ok = 1u8;
            if (edge_idx as usize) < edge_cnt {
                loop {
                    let edge = rd::<usize>(g_edges_ptr)
                        .wrapping_add((edge_idx as usize).wrapping_mul(0x30));
                    if rd::<i32>(edge) != (pass_idx as i32) {
                        break;
                    }
                    if rd::<i32>(edge + 4) == 1 {
                        let mut matched = false;
                        if rd::<u32>(g_res_ptr + 8) != 0 {
                            let mut r_idx = 0u32;
                            while (r_idx as usize) < (rd::<u32>(g_res_ptr + 8) as usize) {
                                let res_base = rd::<usize>(g_res_ptr);
                                let want_id = rd::<i32>(edge + 8);
                                if rd::<i32>(res_base.wrapping_add((r_idx as usize) * 8)) == want_id
                                    && (r_idx as usize) < 0x800
                                {
                                    matched = true;
                                    let word_addr = pass_entry + 0xd0 + ((r_idx as usize) >> 6) * 8;
                                    wr::<u64>(
                                        word_addr,
                                        rd::<u64>(word_addr) | (1u64 << (r_idx & 0x3f)),
                                    );
                                }
                                r_idx = r_idx.wrapping_add(1);
                            }
                        }
                        if !matched {
                            pass_ok = 0;
                            break;
                        }
                        edge_cnt = rd::<u32>(g_edges_ptr + 8) as usize;
                    }
                    edge_idx = (edge_idx as i32).wrapping_add(1) as isize;
                    if (edge_idx as usize) >= edge_cnt {
                        break;
                    }
                }
            }
            wr::<u8>(g_valid_flag, rd::<u8>(g_valid_flag) & pass_ok);
            pass_idx = pass_idx.wrapping_add(1);
        }
    }

    exe::call_va!(0x1_422f_f5c0, () -> ());
    if rd::<u8>(g_valid_flag) != 0 {
        exe::call_va!(0x1_422f_e4e0, () -> ());
        return 1;
    }
    0
}

/// `rend::rend_14235d320`
/// Original: 0x14235d320 (844 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14235d320(param_1: usize) {
    let cam_ctx = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01d0) as usize);
    if cam_ctx == 0 {
        return;
    }
    let stack_obj = rd::<usize>(param_1);
    let z_f = rd::<f64>(cam_ctx + 0xc8) as f32;
    let y_f = rd::<f64>(cam_ctx + 0xc0) as f32;
    let x_f = rd::<f64>(cam_ctx + 0xb8) as f32;

    let px = (y_f * rd::<f32>(stack_obj + 0x13c) + x_f * rd::<f32>(stack_obj + 0x130))
        + z_f * rd::<f32>(stack_obj + 0x148)
        + rd::<f32>(stack_obj + 0x154);
    let py = (y_f * rd::<f32>(stack_obj + 0x144) + x_f * rd::<f32>(stack_obj + 0x138))
        + z_f * rd::<f32>(stack_obj + 0x150)
        + rd::<f32>(stack_obj + 0x15c);
    let packed_xy = (px.to_bits() as u64) | ((py.to_bits() as u64) << 32);

    exe::call_va!(0x1_4235_0200, (rd::<u64>(stack_obj + 0x48), packed_xy));
    let s1 = rd::<usize>(param_1);
    exe::call_va!(0x1_4234_ed30, (rd::<u64>(s1 + 0x48), 1u64));

    let s2 = rd::<usize>(param_1);
    let sub_188 = rd::<usize>(s2 + 0x48) + 0x188;
    let vtbl = rd::<usize>(sub_188);
    exe::call_va!(rd::<usize>(vtbl + 0x38), (sub_188));

    let s3 = rd::<usize>(param_1);
    let tree = rd::<usize>(s3 + 0x48);
    let parents = rd::<usize>(tree + 0x98);
    let weights = rd::<usize>(tree + 0x78);
    let num_nodes = rd::<u32>(tree + 0x18) as u64;
    let mut zero_f32 = 0.0f32;
    let scratch_buf = if num_nodes != 0 {
        exe::call_va!(0x1_4241_c0a0, (num_nodes * 4, 4u64) -> usize)
    } else {
        0
    };
    exe::call_va!(
        0x1_4123_e200,
        (scratch_buf, num_nodes, (&mut zero_f32 as *mut f32) as usize)
    );

    let s4 = rd::<usize>(param_1);
    let tree4 = rd::<usize>(s4 + 0x48);
    let num_bits = rd::<u64>(tree4 + 0x30);
    let bit_words = rd::<usize>(tree4 + 0x20);
    let word_cnt = num_bits.wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let mut word = rd::<u64>(bit_words.wrapping_add((w_idx as usize) * 8));
        let base_bit = w_idx * 64;
        while word != 0 {
            let tz = word.trailing_zeros() as u64;
            let mut node = base_bit + tz;
            word &= !(1u64 << tz);
            let w_val = rd::<f32>(weights.wrapping_add((node as usize) * 4));
            let mut scale = 1.0f32;
            while node >= num_nodes {
                node = rd::<u32>(parents.wrapping_add((node as usize) * 4)) as u64;
                scale += scale;
            }
            let cand = scale * w_val;
            let slot_addr = scratch_buf.wrapping_add((node as usize) * 4);
            let prev_val = rd::<f32>(slot_addr);
            let next_val = if prev_val > cand { prev_val } else { cand };
            wr::<f32>(slot_addr, next_val);
        }
    }

    let scale_div = rd::<f32>(exe::ptr::<f32>(0x1_4386_ca58) as usize);
    let items_begin = rd::<usize>(s4);
    let items_cnt = rd::<u32>(s4 + 8) as usize;
    let items_end = items_begin.wrapping_add(items_cnt * 8);
    let log2f_fn: unsafe extern "win64" fn(f32) -> f32 = exe::func(0x1_4254_76d0);

    let mut cur_item = items_begin;
    while cur_item != items_end {
        let item = rd::<usize>(cur_item);
        let mut max_w = 0.0f32;
        let idx_begin = rd::<usize>(item + 0x10f8);
        let idx_cnt = rd::<u32>(item + 0x1100) as usize;
        let idx_end = idx_begin.wrapping_add(idx_cnt * 4);
        let mut p_idx = idx_begin;
        while p_idx != idx_end {
            let n_idx = rd::<u32>(p_idx) as usize;
            let v = rd::<f32>(scratch_buf.wrapping_add(n_idx * 4));
            if !(max_w > v) {
                max_w = v;
            }
            p_idx = p_idx.wrapping_add(4);
        }
        let lg = log2f_fn((max_w * max_w) / scale_div);
        wr::<f32>(item + 0x30, lg * 0.5f32 + 1.0f32);

        let mut i = 0i32;
        if rd::<i32>(item + 0x1048) > 0 {
            while i < rd::<i32>(item + 0x1048) {
                let base_30 = rd::<f32>(item + 0x30);
                let off_c48 = rd::<f32>(item + 0xc48 + (i as usize) * 4);
                let dst_idx = (rd::<u16>(item + 0x842) as usize).wrapping_add(i as usize);
                let dst_base = rd::<usize>(item + 0x838);
                wr::<f32>(dst_base.wrapping_add(dst_idx * 4), base_30 + off_c48);
                i += 1;
            }
        }
        let mut j = 0i32;
        if rd::<i32>(item + 0x1048) > 0 {
            while j < rd::<i32>(item + 0x1048) {
                let sub_ptr = rd::<usize>(item + 0x38 + (j as usize) * 8);
                let diff = rd::<u8>(sub_ptr + 0xba).wrapping_sub(rd::<u8>(sub_ptr + 0xbc)) as u32;
                let slot_848 = item + 0x848 + (j as usize) * 4;
                if diff != rd::<u32>(slot_848) {
                    wr::<u8>(item + 0x10e4, 1);
                }
                wr::<u32>(slot_848, diff);
                j += 1;
            }
        }
        cur_item = cur_item.wrapping_add(8);
    }

    if scratch_buf != 0 {
        exe::call_va!(0x1_4241_c190, (scratch_buf));
    }
}

/// `d3d_1411d5b00`
/// Original: 0x1411d5b00 (225 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_1411d5b00(out_pair: usize) -> usize {
    let tls_arr = exe::tls_pointer();
    let tls0 = rd::<usize>(tls_arr);
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let lock_ptr = if rd::<u64>(tls0 + 0x6300) != 0 {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        let p = rd::<usize>(tls0 + 0x6318);
        wr::<usize>(out_pair, p);
        p
    } else {
        let p = exe::ptr::<u8>(0x1_4385_7f30) as usize;
        wr::<usize>(out_pair, p);
        p
    };
    if lock_ptr != 0 {
        let tid = exe::call_iat_native!(0x1_4264_5790, win32_rt::kernel32::threads::GetCurrentThreadId, () -> u32);
        if rd::<u32>(lock_ptr + 0x08) != tid {
            exe::call_iat_native!(0x1_4264_55a8, win32_rt::sync::AcquireSRWLockExclusive, (lock_ptr as *mut core::ffi::c_void) -> ());
            wr::<u32>(lock_ptr + 0x08, tid);
        }
        wr::<u32>(lock_ptr + 0x0c, rd::<u32>(lock_ptr + 0x0c).wrapping_add(1));
    }
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let ctx_obj = if rd::<u64>(tls0 + 0x6300) != 0 {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        rd::<usize>(tls0 + 0x6300)
    } else {
        rd::<usize>(exe::ptr::<usize>(0x1_4397_fc68) as usize)
    };
    wr::<u64>(ctx_obj + 0x18, rd::<u64>(ctx_obj + 0x18).wrapping_add(1));
    wr::<u64>(out_pair + 0x08, rd::<u64>(ctx_obj + 0x28));
    out_pair
}

/// `std_1412defc0`
/// Original: 0x1412defc0 (450 bytes, Throws::No)
pub unsafe extern "win64" fn std_1412defc0(this: usize, param_2: usize) {
    let lock_base = this + 0x140;
    let tid = exe::call_iat_native!(0x1_4264_5790, win32_rt::kernel32::threads::GetCurrentThreadId, () -> u32);
    if rd::<u32>(lock_base + 0x08) != tid {
        exe::call_iat_native!(0x1_4264_55a8, win32_rt::sync::AcquireSRWLockExclusive, (lock_base as *mut core::ffi::c_void) -> ());
        wr::<u32>(lock_base + 0x08, tid);
    }
    wr::<u32>(lock_base + 0x0c, rd::<u32>(lock_base + 0x0c).wrapping_add(1));

    let total = (rd::<i32>(this + 0x68).wrapping_add(rd::<i32>(this + 0x28))) as isize;
    if total != 0 {
        let sentinel = (this + 0x80) as u64;
        for i in 0..(total as usize) {
            let arr = rd::<usize>(this + 0xc8);
            let item = rd::<usize>(arr.wrapping_add(i * 8));
            if rd::<u64>(item + 0x58) != sentinel && rd::<i32>(item + 0x80) == 5 {
                if rd::<u8>(item + 0xb9) != 0 {
                    let target = rd::<usize>(param_2);
                    let atomic_ref = (item + 0x24) as *const core::sync::atomic::AtomicI32;
                    let prev = (*atomic_ref).fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                    if prev == 0 {
                        let cb_owner = rd::<usize>(item + 0x28);
                        if cb_owner != 0 {
                            exe::call_va!(0x1_4241_d6f0, (cb_owner, item));
                        }
                    }
                    let len = rd::<u32>(target + 0x58) as u64;
                    let cap = rd::<u32>(target + 0x5c) as u64;
                    if len == cap {
                        let next_len = len + 1;
                        let mut new_cap = cap + (cap >> 1);
                        if new_cap < next_len {
                            new_cap = next_len;
                        }
                        exe::call_va!(0x1_411c_a040, (target + 0x50, new_cap));
                    }
                    let cur_len = rd::<u32>(target + 0x58) as usize;
                    let buf = rd::<usize>(target + 0x50);
                    wr::<usize>(buf.wrapping_add(cur_len * 8), item);
                    wr::<u32>(target + 0x58, (cur_len as u32).wrapping_add(1));
                }

                let target = rd::<usize>(param_2);
                if rd::<u8>(item + 0xbb) <= 1 {
                    let add_bytes = if rd::<u8>(item + 0xb8) == 0 {
                        rd::<u64>(item + 0x168).wrapping_add(rd::<u64>(item + 0x160))
                    } else {
                        let sub_1d0 = rd::<usize>(item + 0x1d0);
                        if sub_1d0 == 0 {
                            0
                        } else {
                            let obj_60 = rd::<usize>(sub_1d0 + 0x60);
                            let vtbl = rd::<usize>(obj_60);
                            let v_sz = exe::call_va!(rd::<usize>(vtbl + 0x20), (obj_60) -> u64);
                            let p10 = rd::<usize>(sub_1d0 + 0x10);
                            let s38 = (rd::<i32>(sub_1d0 + 0x38) as i64) as u64;
                            rd::<u64>(p10 + 0xe0).wrapping_add(s38).wrapping_add(v_sz)
                        }
                    };
                    wr::<u64>(target + 0x40, rd::<u64>(target + 0x40).wrapping_add(add_bytes));
                    wr::<u64>(target + 0x48, rd::<u64>(target + 0x48).wrapping_add(add_bytes));
                } else {
                    let f2 = rd::<u32>(param_2 + 0x08) as u128;
                    let f3 = rd::<u32>(param_2 + 0x0c) as u128;
                    let direct = crate::simd_ops::fn_142206de0 as *const () as usize;
                    if crate::emu::resolve_dispatch(0x1_4220_6de0).0 == direct {
                        // Same TLS xmm hand-off as the bridge (callee reads it via take_xmm_args).
                        crate::simd_ops::set_tls_xmm_in([0, 0, f2, f3]);
                        crate::simd_ops::fn_142206de0(target, item);
                        let c = crate::callctx::ctx();
                        c.xmm_in_set.set(false);
                        c.xmm_out.set(None);
                    } else {
                        let _ = crate::simd_ops::call_va8_xmm(
                            0x1_4220_6de0,
                            target as u64,
                            item as u64,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            [0, 0, f2, f3],
                        );
                    }
                }
            }
        }
    }

    let next_rc = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_base + 0x0c, next_rc);
    if next_rc == 0 {
        let owner_atomic = (lock_base + 0x08) as *const core::sync::atomic::AtomicU32;
        (*owner_atomic).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
        exe::call_iat_native!(0x1_4264_5598, win32_rt::sync::ReleaseSRWLockExclusive, (lock_base as *mut core::ffi::c_void) -> ());
    }
}

/// `rend::SparseUploadBuffer<unsigned int>::vfunc_1`
/// Original: 0x1412eb690 (810 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_1412eb690(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let num_bits = rd::<u64>(this + 0xb8);
    let word_cnt = num_bits.wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32) * 64 + tz;
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len * 12 == cur_cap * 12 {
                    let next_len = cur_len + 1;
                    let mut new_cap = cur_cap + (cur_cap >> 1);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_412e_c2c0, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let data_ptr = rd::<usize>(vec_ptr);
                let slot = data_ptr.wrapping_add(idx_now * 12);
                wr::<u64>(slot, 0);
                wr::<u32>(slot + 8, 0);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let data_ptr2 = rd::<usize>(vec_ptr);
                let slot2 = data_ptr2.wrapping_add(new_len * 12 - 12);
                wr::<u32>(slot2, elem_idx);
                let src_vals = rd::<usize>(this + 0x98);
                wr::<u32>(slot2 + 8, rd::<u32>(src_vals.wrapping_add((elem_idx as usize) * 4)));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    exe::call_va!(
        0x1_412e_c110,
        (this + 0x08, rd::<u32>(this + 0xa0) as u64, this + 0x58)
    );
    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        let needed = (count as u64).wrapping_mul(rd::<u64>(this + 0x78));
        let mut alloc_sz = rd::<u64>(this + 0x80);
        if needed >= alloc_sz {
            let sum = alloc_sz.wrapping_add(needed).wrapping_sub(1);
            alloc_sz = if alloc_sz != 0 {
                sum.wrapping_sub(sum % alloc_sz)
            } else {
                needed
            };
        }
        let time_fn: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
        let now_sec = time_fn();
        let buf_obj = rd::<usize>(this + 0x10);
        let cur_sz = rd::<u64>(buf_obj + 0x10);
        let shrink_ok = rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0
            && cur_sz > alloc_sz
            && (now_sec - rd::<f64>(this + 0x88)) > 1.0;
        if cur_sz < alloc_sz || shrink_ok {
            exe::call_va!(0x1_4209_d670, (buf_obj));
            exe::call_va!(
                0x1_4207_5af0,
                (alloc_sz, 0u64, 0u64, 0x11u64, 0u64, 0u64, buf_obj as u64)
            );
            wr::<u32>(buf_obj + 0x50, u32::from(alloc_sz > 0x40_0000) + 2);
            wr::<f64>(this + 0x88, now_sec);
        }
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<u64>(this + 0x10), 0u64, (count as u64) * 12, 2u64) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = (rd::<usize>(this + 0x38)).wrapping_add((prev_off as usize) * 12);
        let src_ptr = rd::<usize>(this + 0xc0);
        core::ptr::copy_nonoverlapping(
            src_ptr as *const u8,
            dst_ptr as *mut u8,
            (count as usize) * 12,
        );
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<u64>(this + 0x10), 0u64));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4222_b6e0,
            (
                rd::<u64>(this + 0x08),
                rd::<u64>(this + 0x10),
                pending as u64,
                0u64,
                0u64
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let len_bytes = (rd::<u32>(this + 0xc8) as u64) * 12;
        let cap_bytes = (rd::<u32>(this + 0xcc) as u64) * 12;
        if len_bytes > 0x10_0000 && len_bytes != cap_bytes {
            exe::call_va!(0x1_412e_c2c0, (this + 0xc0, rd::<u32>(this + 0xc8) as u64));
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `rend_1412ed4a0`
/// Original: 0x1412ed4a0 (576 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ed4a0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u64,
) {
    let total = rd::<u64>(param_3);
    if total == 0 {
        return;
    }
    let split = rd::<u64>(param_5);
    let batch = total.min(split);
    let rem = total - batch;
    if rem != 0 {
        let p1_0 = rd::<u64>(param_1);
        let p1_1 = rd::<u64>(param_1 + 8);
        let p4_0 = rd::<u64>(param_4);
        let p4_1 = rd::<u64>(param_4 + 8);
        let sched_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        let inner_sched = rd::<usize>(sched_mgr + 0x10);
        let wid = crate::ns::r::thread::worker_index() as u32 as usize;
        let prio_off = ((param_2 & 0xff) as usize) << 6;
        let pool = rd::<usize>(inner_sched.wrapping_add(wid << 6).wrapping_add(0x48))
            .wrapping_add(prio_off);
        let mut job_handle = [0u64; 2];
        crate::ns::r::thread::job_node_alloc(
            pool,
            job_handle.as_mut_ptr() as usize,
        );
        let job_ptr = job_handle[0] as usize;
        let atomic_ref = job_ptr as *const core::sync::atomic::AtomicU32;
        (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
        wr::<u64>(job_ptr + 0x08, 0);
        wr::<u64>(job_ptr + 0x10, exe::ptr::<u8>(0x1_433f_1280) as u64);
        wr::<u64>(job_ptr + 0x18, p1_0);
        wr::<u64>(job_ptr + 0x20, p1_1);
        wr::<u16>(job_ptr + 0x28, param_2 as u16);
        wr::<u64>(job_ptr + 0x30, rem);
        wr::<u64>(job_ptr + 0x38, rd::<u64>(param_3 + 8).wrapping_add(batch << 8));
        wr::<u64>(job_ptr + 0x40, p4_0);
        wr::<u64>(job_ptr + 0x48, p4_1);
        wr::<u64>(job_ptr + 0x50, split);
        wr::<u64>(job_ptr + 0x58, param_6.wrapping_add(batch));

        let parent_cnt = p1_0 as *const core::sync::atomic::AtomicU32;
        (*parent_cnt).fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        wr::<u64>(job_ptr + 0x08, p1_0);
        if rd::<u32>(job_ptr) == 1 {
            let inner2 = rd::<usize>(sched_mgr + 0x10);
            crate::ns::r::thread::scheduler_enqueue_job(inner2, job_ptr);
            let sem_owner = rd::<usize>(inner2);
            let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(sem_owner + 0x1008), 1u64, 0u64) -> u64);
        }
    }

    let base_cascade = rd::<usize>(param_3 + 8);
    let end_cascade = base_cascade.wrapping_add((batch as usize) << 8);
    let mut cur = base_cascade;
    while cur != end_cascade {
        let p4_base = rd::<usize>(param_4);
        let p4_stride = rd::<usize>(param_4 + 8);
        let dx = rd::<i32>(cur + 0x18);
        let dy = rd::<i32>(cur + 0x1c);
        let dz = rd::<i32>(cur + 0x20);
        let total_bytes = (dx * dy * dz) as i64 as u64;
        if (rd::<u32>(cur + 0x10) as u64) != total_bytes {
            exe::call_va!(0x1_4028_e490, (cur + 0x08, total_bytes));
        }
        let row_bytes = dx as isize as usize;
        let slice_off = (dx as i64).wrapping_mul(rd::<i64>(cur)) as usize;
        let dst_base = rd::<usize>(cur + 0x08);
        if dz > 0 {
            for iz in 0..dz {
                let ny = rd::<i32>(cur + 0x1c);
                if ny > 0 {
                    for iy in 0..ny {
                        let row_idx = (iz * rd::<i32>(cur + 0x1c) + iy) as isize as usize;
                        let r_bytes = rd::<i32>(cur + 0x18) as isize as usize;
                        let dst = dst_base.wrapping_add(row_idx.wrapping_mul(r_bytes));
                        let src = p4_base
                            .wrapping_add(slice_off)
                            .wrapping_add(row_idx.wrapping_mul(p4_stride));
                        core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, r_bytes);
                        let _ = row_bytes;
                    }
                }
            }
        }
        cur = cur.wrapping_add(0x100);
    }
}

/// `unknown_141e69330`
/// Original: 0x141e69330 (675 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e69330(
    param_1: usize,
    param_2: usize,
    _param_3: u64,
    _param_4: u64,
) {
    let idx = rd::<usize>(param_1 + 0x50);
    let p_00 = rd::<usize>(param_1).wrapping_add(idx.wrapping_mul(0x20));
    let p_08 = rd::<usize>(param_1 + 0x08).wrapping_add(idx.wrapping_mul(0x18));
    let p_10 = rd::<usize>(param_1 + 0x10);
    let p_18 = rd::<usize>(param_1 + 0x18).wrapping_add(idx.wrapping_mul(0x148));
    let p_20 = rd::<usize>(param_1 + 0x20).wrapping_add(idx.wrapping_mul(0x40));
    let p_28 = rd::<usize>(param_1 + 0x28);
    let off_4 = idx.wrapping_mul(4);

    let p_48 = rd::<usize>(param_1 + 0x48);
    let slot_idx = rd::<u32>(p_48.wrapping_add(idx * 8).wrapping_add(0x10)) as usize;
    let tbl_30 = rd::<usize>(param_2 + 0x30);
    let entry_val = rd::<u64>(tbl_30.wrapping_add(slot_idx * 8));
    if (entry_val as u16) == 0xffff {
        let _ = exe::call_iat!(0x1_4264_67e8, () -> u64);
        return;
    }
    let span_off = ((entry_val & 0xffff) as usize) << 5;
    let span_ptr = rd::<u64>(param_2.wrapping_add(span_off).wrapping_add(0x18090));
    let span_len = rd::<u32>(param_2.wrapping_add(span_off).wrapping_add(0x180a8));
    let want_u16 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_c8e4) as usize);
    let found_idx = exe::call_va!(0x1_4248_9ee0, (span_ptr, span_len as u64, want_u16 as u64) -> i32);

    let p_38 = rd::<usize>(param_1 + 0x38);
    let has_flag = (p_38 != 0 && rd::<u8>(p_38.wrapping_add(idx)) != 0)
        || rd::<u8>(p_18 + 0x14) != 0;

    if rd::<u32>(p_28 + off_4) == 0 {
        if rd::<u32>(p_10 + off_4) != 0 {
            if has_flag && found_idx == -1 {
                exe::call_va!(0x1_41e6_a370, (p_00, p_18, p_08));
            }
            wr::<u32>(p_10 + off_4, rd::<u32>(p_28 + off_4));
        }
    } else if rd::<u32>(p_10 + off_4) == 0 {
        if rd::<u8>(p_08 + 0x10) != 0 {
            exe::call_va!(0x1_41e6_a280, (p_00, p_18, p_08));
        }
        wr::<u32>(p_10 + off_4, rd::<u32>(p_28 + off_4));
        return;
    }

    if found_idx != -1 {
        let r = rd::<f32>(p_18 + 0x1c);
        let r_sq = r * r;
        let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
        let slot_c414 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_c414) as usize) as usize;
        let cam_obj = rd::<usize>(tbl_580e0.wrapping_add(slot_c414 * 8));
        let cx = rd::<f32>(cam_obj + 0x7c);
        let cy = rd::<f32>(cam_obj + 0x80);
        let cz = rd::<f32>(cam_obj + 0x84);

        let px = rd::<f32>(p_20 + 0x10);
        let py = rd::<f32>(p_20 + 0x14);
        let pz = rd::<f32>(p_20 + 0x18);
        let dx = cx - px;
        let dy = cy - py;
        let dz = cz - pz;
        let mut in_range = ((dx * dx + dy * dy) + dz * dz) < r_sq;

        let p_40 = rd::<usize>(param_1 + 0x40);
        if p_40 != 0 {
            let idx2 = rd::<usize>(param_1 + 0x50);
            let sub_40 = p_40.wrapping_add(idx2.wrapping_mul(0x18));
            if rd::<u8>(sub_40) != 0 {
                in_range = rd::<u8>(sub_40 + 0x02) != 0 && in_range;
                let pts_base = rd::<usize>(sub_40 + 0x08);
                let pts_cnt = rd::<u32>(sub_40 + 0x10) as usize;
                let pts_end = pts_base.wrapping_add(pts_cnt.wrapping_mul(12));
                let mut pt = pts_base;
                while pt != pts_end {
                    if !in_range {
                        let qx = cx - rd::<f32>(pt);
                        let qy = cy - rd::<f32>(pt + 4);
                        let qz = cz - rd::<f32>(pt + 8);
                        if ((qx * qx + qy * qy) + qz * qz) < r_sq {
                            in_range = true;
                        }
                    }
                    pt = pt.wrapping_add(12);
                }
            }
        }

        if in_range {
            if rd::<u8>(p_08 + 0x10) == 0 && has_flag {
                exe::call_va!(0x1_41e6_a370, (p_00, p_18, p_08));
            }
        } else if rd::<u8>(p_08 + 0x10) != 0 {
            exe::call_va!(0x1_41e6_a280, (p_00, p_18, p_08));
        }
    }
}

/// `unknown_141eec2e0`
/// Original: 0x141eec2e0 (1447 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141eec2e0(param_1: usize, param_2: usize) {
    let tbl_580e0 = rd::<usize>(param_1 + 0x580e0);
    let slot_bac = rd::<u16>(exe::ptr::<u16>(0x1_43cd_6bac) as usize) as usize;
    let slot_e88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
    let out_vecs = rd::<usize>(tbl_580e0.wrapping_add(slot_bac * 8));
    let hash_tbl = rd::<usize>(tbl_580e0.wrapping_add(slot_e88 * 8));

    wr::<u32>(out_vecs + 0x08, 0);
    exe::call_va!(0x1_4028_dee0, (out_vecs, 0x2000u64));
    let out_vecs2 = out_vecs + 0x10;
    wr::<u32>(out_vecs2 + 0x08, 0);
    exe::call_va!(0x1_4028_dee0, (out_vecs2, 0x2000u64));

    let mut scratch_ptr = exe::call_va!(0x1_4241_c0a0, (0x1_0000u64, 8u64) -> usize);
    let mut scratch_len = 0usize;
    let mut scratch_cap = 0x2000usize;

    let r11 = rd::<u64>(param_2).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_2 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );

    let mut outer = [0u64; 9];
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

    exe::call_va!(0x1_4106_f2b0, (outer.as_mut_ptr() as usize));
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let s0 = outer[7] as usize;
            let s1 = outer[8] as usize;
            if rd::<u8>(s0.wrapping_add(idx)) != 0 {
                let comp = s1.wrapping_add(idx.wrapping_mul(0x30));
                let mut prev_tag = 0xffff_ffffu32;
                let start_vec1 = rd::<u32>(out_vecs + 0x08) as usize;
                let mut tags_ptr = rd::<usize>(comp);
                let mut entries_ptr = rd::<usize>(comp + 0x20);
                let entries_cnt = rd::<u32>(comp + 0x28) as usize;
                let entries_end = entries_ptr.wrapping_add(entries_cnt.wrapping_mul(0x38));

                while entries_ptr != entries_end {
                    let bit_id = rd::<u16>(tags_ptr) as usize;
                    tags_ptr = tags_ptr.wrapping_add(2);
                    let word = rd::<u64>(hash_tbl + 0x118 + (bit_id >> 6) * 8);
                    if ((word >> (bit_id & 0x3f)) & 1) != 0
                        && (rd::<u64>(entries_ptr) != 0 || rd::<u64>(entries_ptr + 8) != 0)
                    {
                        let slot_idx = if rd::<u64>(hash_tbl + 0x40) == rd::<u64>(hash_tbl + 0x38) {
                            rd::<u64>(hash_tbl + 0x48)
                        } else {
                            let mut probe_out = 0i64;
                            crate::ns::r::scheduler::open_hash_probe_40(
                                hash_tbl as u64,
                                (&mut probe_out as *mut i64) as u64,
                                entries_ptr as u64,
                            );
                            if probe_out == -1 {
                                rd::<u64>(hash_tbl + 0x48)
                            } else {
                                probe_out as u64
                            }
                        };
                        let end_slot = rd::<u64>(hash_tbl + 0x48);
                        let bucket = (rd::<usize>(hash_tbl + 0x78))
                            .wrapping_add((slot_idx as usize).wrapping_mul(0x28));
                        let end_bucket = (rd::<usize>(hash_tbl + 0x78))
                            .wrapping_add((end_slot as usize).wrapping_mul(0x28));
                        if bucket != end_bucket {
                            let val_ptr = bucket + 0x10;
                            if val_ptr != 0 && (rd::<u8>(val_ptr + 0x03) & 1) != 0 {
                                let eq = exe::call_va!(0x1_403b_2dc0, (val_ptr, entries_ptr + 0x10) -> u8);
                                if eq == 0 {
                                    let cur_tag = rd::<u32>(entries_ptr + 0x30);
                                    if cur_tag != prev_tag {
                                        let mut item_24 = [0u64; 3];
                                        item_24[0] = rd::<u64>(entries_ptr + 0x28);
                                        item_24[1] = cur_tag as u64;
                                        let v_len = rd::<u32>(out_vecs + 0x08);
                                        let v_cap = rd::<u32>(out_vecs + 0x0c) & 0x3fff_ffff;
                                        let mut end_p = (rd::<usize>(out_vecs))
                                            .wrapping_add((v_len as usize).wrapping_mul(0x18));
                                        if v_len == v_cap {
                                            exe::call_va!(
                                                0x1_4030_5fe0,
                                                (
                                                    out_vecs,
                                                    (&mut end_p as *mut usize) as usize,
                                                    item_24.as_ptr() as usize
                                                )
                                            );
                                        } else {
                                            wr::<u32>(out_vecs + 0x08, v_len.wrapping_add(1));
                                            wr::<[u64; 3]>(end_p, item_24);
                                        }
                                        prev_tag = rd::<u32>(entries_ptr + 0x30);
                                    }
                                    wr::<[u64; 3]>(entries_ptr + 0x10, rd::<[u64; 3]>(val_ptr));
                                }
                                if scratch_len == scratch_cap {
                                    let next_len = scratch_len + 1;
                                    let mut new_cap = scratch_cap + (scratch_cap >> 1);
                                    if new_cap < next_len {
                                        new_cap = next_len;
                                    }
                                    let new_ptr = exe::call_va!(0x1_4241_c0a0, ((new_cap * 8) as u64, 8u64) -> usize);
                                    core::ptr::copy_nonoverlapping(
                                        scratch_ptr as *const u8,
                                        new_ptr as *mut u8,
                                        scratch_len * 8,
                                    );
                                    if scratch_ptr != 0 {
                                        exe::call_va!(0x1_4241_c190, (scratch_ptr));
                                    }
                                    scratch_ptr = new_ptr;
                                    scratch_cap = new_cap;
                                }
                                wr::<usize>(scratch_ptr.wrapping_add(scratch_len * 8), val_ptr);
                                scratch_len += 1;
                            }
                        }
                    }
                    entries_ptr = entries_ptr.wrapping_add(0x38);
                }

                let mut scan_idx = 0usize;
                let mut v_idx = start_vec1;
                while v_idx < (rd::<u32>(out_vecs + 0x08) as usize) {
                    let vec_item = (rd::<usize>(out_vecs)).wrapping_add(v_idx.wrapping_mul(0x18));
                    let want_tag = rd::<u32>(vec_item + 0x08);
                    let e_base = rd::<usize>(comp + 0x20);
                    while rd::<u32>(e_base.wrapping_add(scan_idx.wrapping_mul(0x38)).wrapping_add(0x30)) != want_tag {
                        scan_idx += 1;
                    }
                    wr::<u32>(vec_item + 0x0c, rd::<u32>(out_vecs2 + 0x08));
                    let total_e = rd::<i32>(comp + 0x28) as isize as usize;
                    while scan_idx < total_e {
                        let e_cur = (rd::<usize>(comp + 0x20)).wrapping_add(scan_idx.wrapping_mul(0x38));
                        if rd::<u32>(e_cur + 0x30) != want_tag {
                            break;
                        }
                        let src_24 = e_cur + 0x10;
                        let v2_len = rd::<u32>(out_vecs2 + 0x08);
                        let v2_cap = rd::<u32>(out_vecs2 + 0x0c) & 0x3fff_ffff;
                        let mut end2_p = (rd::<usize>(out_vecs2))
                            .wrapping_add((v2_len as usize).wrapping_mul(0x18));
                        if v2_len == v2_cap {
                            exe::call_va!(
                                0x1_4030_5fe0,
                                (
                                    out_vecs2,
                                    (&mut end2_p as *mut usize) as usize,
                                    src_24
                                )
                            );
                        } else {
                            wr::<u32>(out_vecs2 + 0x08, v2_len.wrapping_add(1));
                            wr::<[u64; 3]>(end2_p, rd::<[u64; 3]>(src_24));
                        }
                        wr::<u32>(vec_item + 0x10, rd::<u32>(vec_item + 0x10).wrapping_add(1));
                        scan_idx += 1;
                    }
                    v_idx += 1;
                }
            }
            outer[6] = outer[6].wrapping_add(1);
        }

        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_e0 = outer[1] as usize;
            if rd::<u64>(base_e0.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_e0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                let base_e8 = outer[0] as usize;
                let cp = rd::<u64>(base_e8.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_e8.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
                break;
            }
        }
        exe::call_va!(0x1_4106_f2b0, (outer.as_mut_ptr() as usize));
    }

    for i in 0..scratch_len {
        let p = rd::<usize>(scratch_ptr.wrapping_add(i * 8));
        wr::<u8>(p + 0x03, rd::<u8>(p + 0x03) & !1);
    }
    core::ptr::write_bytes((hash_tbl + 0x118) as *mut u8, 0, 0x1000);
    if scratch_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (scratch_ptr));
    }
}

/// `unknown_14209b0c0`
/// Original: 0x14209b0c0 (606 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14209b0c0(this: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_1620) as usize) == 0 {
        return;
    }
    let count = rd::<u32>(this + 0x90) as usize;
    if count == 0 {
        return;
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    let tbl_base = exe::ptr::<i32>(0x1_43a6_37b0) as usize;
    for i in 0..count {
        let off = i.wrapping_mul(0x28);
        let arr = rd::<usize>(this + 0x88);
        let entry = arr.wrapping_add(off);
        let src_slot = rd::<i32>(entry + 0x08) as isize;
        let dst_slot = rd::<i32>(entry + 0x10) as isize;
        let new_ver = rd::<i32>((tls0 + 0x463e0).wrapping_add((src_slot * 4) as usize));
        let dst_ver_addr = (tls0 + 0x47460).wrapping_add((dst_slot * 4) as usize);
        let old_ver = rd::<i32>(dst_ver_addr);
        wr::<i32>(dst_ver_addr, new_ver);
        let force_upload = rd::<u8>(exe::ptr::<u8>(0x1_4386_15a0) as usize) == 0;

        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        let cb_slot_addr = (tls0 + 0x5b460).wrapping_add((dst_slot * 0x28) as usize);
        if rd::<u64>(cb_slot_addr + 0x18) == 0 || new_ver != old_ver || force_upload {
            if rd::<u8>(tls0 + 0x70) == 0 {
                exe::call_va!(0x1_4183_49c0, () -> ());
            }
            let cb_size = rd::<i32>(entry + 0x04) as i64 as u64;
            let alloc_ctx = exe::call_va!(0x1_411d_9920, (rd::<u64>(tls0 + 0x6360)) -> usize);
            let mut tmp_40 = [0u64; 5];
            let res_ptr = exe::call_va!(
                0x1_411d_9690,
                (alloc_ctx, tmp_40.as_mut_ptr() as usize, cb_size) -> usize
            );
            wr::<[u64; 5]>(cb_slot_addr, rd::<[u64; 5]>(res_ptr));
            let dst_buf = rd::<usize>(cb_slot_addr + 0x18);
            let num_fields = rd::<u32>(entry + 0x20) as usize;
            for f_idx in 0..num_fields {
                let fields_base = rd::<usize>(entry + 0x18);
                let f_entry = fields_base.wrapping_add(f_idx.wrapping_mul(0x18));
                let cb_total = rd::<i32>(entry + 0x04);
                let field_len = rd::<i32>(f_entry + 0x0c);
                let dst_off = rd::<i32>(f_entry + 0x08);
                let rem = cb_total.wrapping_sub(dst_off);
                let copy_len = if rem < field_len { rem } else { field_len };
                if copy_len > 0 {
                    let param_id = rd::<i32>(f_entry) as isize;
                    let tls_off = rd::<i32>(tbl_base.wrapping_add((param_id * 16) as usize)) as isize;
                    let src_ptr = (tls0 + 0x63d0).wrapping_add(tls_off as usize);
                    let dst_ptr = dst_buf.wrapping_add(dst_off as isize as usize);
                    core::ptr::copy_nonoverlapping(
                        src_ptr as *const u8,
                        dst_ptr as *mut u8,
                        copy_len as usize,
                    );
                }
            }
            if rd::<u8>(tls0 + 0x70) == 0 {
                exe::call_va!(0x1_4183_49c0, () -> ());
            }
            let alloc_ctx2 = exe::call_va!(0x1_411d_9920, (rd::<u64>(tls0 + 0x6360)) -> usize);
            wr::<u64>(alloc_ctx2 + 0x28, rd::<u64>(alloc_ctx2 + 0x28).wrapping_sub(1));
        }
    }
}

/// `physics_142142870`
/// Original: 0x142142870 (508 bytes, Throws::No)
pub unsafe extern "win64" fn physics_142142870(this: usize) {
    let g_sched = exe::ptr::<u64>(0x1_43a0_82f8) as usize;
    while rd::<u8>(this + 0xc8) == 0 {
        crate::ns::r::thread::scheduler_pump_one(rd::<usize>(g_sched));
    }

    let lock_d0 = (this + 0xd0) as u64;
    let _ = exe::call_iat!(0x1_4264_55a8, (lock_d0) -> u64);
    wr::<u8>(this + 0xd8, 1);
    let flag_atomic = (this + 0xc8) as *const core::sync::atomic::AtomicU8;
    (*flag_atomic).swap(0, core::sync::atomic::Ordering::SeqCst);

    let scene = rd::<usize>(this + 0x20);
    let scene_vtbl = rd::<usize>(scene);
    exe::call_va!(rd::<usize>(scene_vtbl + 0x1c8), (scene, this + 0xa0));
    let _ = exe::call_iat!(0x1_4264_5598, (lock_d0) -> u64);

    while rd::<u8>(this + 0xc8) == 0 {
        crate::ns::r::thread::scheduler_pump_one(rd::<usize>(g_sched));
    }

    let list1_begin = rd::<usize>(this + 0x50);
    let list1_cnt = rd::<u32>(this + 0x58) as usize;
    let list1_end = list1_begin.wrapping_add(list1_cnt * 8);
    let mut p1 = list1_begin;
    while p1 != list1_end {
        let actor = rd::<usize>(p1);
        let user_data = rd::<usize>(actor + 0x10);
        if user_data != 0 && rd::<u64>(user_data) != 0 && rd::<u16>(actor + 0x08) == 6 {
            let vtbl = rd::<usize>(actor);
            let mut flags_out = 0u64;
            let flags_ptr = exe::call_va!(
                rd::<usize>(vtbl + 0x178),
                (actor, (&mut flags_out as *mut u64) as usize) -> usize
            );
            if (rd::<u8>(flags_ptr) & 4) == 0 {
                let idx_8 = rd::<i32>(user_data + 0x08) as isize;
                let owner = rd::<usize>(user_data);
                let arr_b8 = rd::<usize>(owner + 0xb8);
                let entry_byte = rd::<u8>(arr_b8.wrapping_add((idx_8 * 20) as usize).wrapping_add(0x10));
                if (entry_byte & 2) != 0 {
                    let vtbl2 = rd::<usize>(actor);
                    exe::call_va!(rd::<usize>(vtbl2 + 0x168), (actor, 0x20u64, 1u64));
                }
            }
        }
        p1 = p1.wrapping_add(8);
    }
    wr::<u32>(this + 0x58, 0);

    let list2_begin = rd::<usize>(this + 0x60);
    let list2_cnt = rd::<u32>(this + 0x68) as usize;
    let list2_end = list2_begin.wrapping_add(list2_cnt * 8);
    let px_rigid_str = exe::ptr::<u8>(0x1_433e_2638) as u64;
    let mut p2 = list2_begin;
    while p2 != list2_end {
        let actor = rd::<usize>(p2);
        let user_data = rd::<usize>(actor + 0x10);
        if user_data != 0 && rd::<u64>(user_data) != 0 {
            if rd::<u16>(actor + 0x08) == 6 {
                let vtbl = rd::<usize>(actor);
                exe::call_va!(rd::<usize>(vtbl + 0x168), (actor, 0x20u64, 0u64));
            }
            let owner = rd::<usize>(user_data);
            let vtbl = rd::<usize>(actor);
            let is_rigid = exe::call_va!(rd::<usize>(vtbl + 0x20), (actor, px_rigid_str) -> u8);
            let match_actor = if is_rigid != 0 { actor } else { 0 };
            let sub_20 = rd::<usize>(owner + 0x20);
            let n_actors = rd::<u16>(sub_20 + 0x90) as usize;
            let mut found = 0xffff_ffffu32;
            if n_actors != 0 {
                let actors_arr = rd::<usize>(owner + 0xc0);
                for k in 0..n_actors {
                    if rd::<usize>(actors_arr.wrapping_add(k * 8)) == match_actor {
                        found = k as u32;
                        break;
                    }
                }
            }
            exe::call_va!(0x1_4216_3f20, (owner, found as u64));
        }
        p2 = p2.wrapping_add(8);
    }
    wr::<u32>(this + 0x68, 0);
}

/// `unknown_142330af0`
/// Original: 0x142330af0 (491 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_142330af0() {
    let lock1 = exe::ptr::<u64>(0x1_4386_c5e8) as usize;
    let tid1 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock1 + 0x08) != tid1 {
        let _ = exe::call_iat!(0x1_4264_55a8, (lock1 as u64) -> u64);
        wr::<u32>(lock1 + 0x08, tid1);
    }
    wr::<u32>(lock1 + 0x0c, rd::<u32>(lock1 + 0x0c).wrapping_add(1));

    let step = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as usize;
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);

    let lock2 = exe::ptr::<u64>(0x1_4385_8478) as usize;
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock2 + 0x08) != tid2 {
        let _ = exe::call_iat!(0x1_4264_55a8, (lock2 as u64) -> u64);
        wr::<u32>(lock2 + 0x08, tid2);
    }
    wr::<u32>(lock2 + 0x0c, rd::<u32>(lock2 + 0x0c).wrapping_add(1));

    let lock3 = mgr + 0x20;
    let tid3 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock3 + 0x08) != tid3 {
        let _ = exe::call_iat!(0x1_4264_55a8, (lock3 as u64) -> u64);
        wr::<u32>(lock3 + 0x08, tid3);
    }
    wr::<u32>(lock3 + 0x0c, rd::<u32>(lock3 + 0x0c).wrapping_add(1));

    let g_ca8 = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let mut ca8 = rd::<usize>(g_ca8);
    if ca8 == 0 {
        ca8 = exe::call_va!(0x1_4209_2770, (mgr) -> usize);
        wr::<usize>(g_ca8, ca8);
    }
    let idx_cb0 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as usize;
    let mut dst2 = (rd::<usize>(ca8 + 0x08)).wrapping_add(idx_cb0.wrapping_mul(0x40400));
    let mut dst1 = rd::<usize>(mgr + 0x2a0);

    let rc3 = rd::<u32>(lock3 + 0x0c).wrapping_sub(1);
    wr::<u32>(lock3 + 0x0c, rc3);
    if rc3 == 0 {
        let a3 = (lock3 + 0x08) as *const core::sync::atomic::AtomicU32;
        (*a3).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
        let _ = exe::call_iat!(0x1_4264_5598, (lock3 as u64) -> u64);
    }

    let rc2 = rd::<u32>(lock2 + 0x0c).wrapping_sub(1);
    wr::<u32>(lock2 + 0x0c, rc2);
    if rc2 == 0 {
        let a2 = (lock2 + 0x08) as *const core::sync::atomic::AtomicU32;
        (*a2).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
        let _ = exe::call_iat!(0x1_4264_5598, (lock2 as u64) -> u64);
    }

    let count = rd::<u32>(exe::ptr::<u32>(0x1_43bb_d870) as usize) as usize;
    let g_arr = exe::ptr::<usize>(0x1_43bb_d868) as usize;
    // ID3D12Device::CopyDescriptorsSimple (vtable +0xc0): call our builtin directly.
    let copy_va = if count != 0 { rd::<usize>(rd::<usize>(dev) + 0xc0) as u64 } else { 0 };
    let native_copy = count != 0
        && crate::emu::resolve_dispatch(copy_va).0
            == win32_rt::d3d12::descheap::copy_descriptors_simple as *const () as usize;
    for i in 0..count {
        let arr = rd::<usize>(g_arr);
        let src1 = rd::<u64>(arr.wrapping_add(i * 8));
        if native_copy {
            use win32_rt::d3d12::descheap::{copy_descriptors_simple, copy_one_resource_descriptor_inline};
            if !copy_one_resource_descriptor_inline(dst1, src1 as usize) {
                copy_descriptors_simple(dev as _, 1, dst1, src1 as usize, 0);
            }
            let src2 = rd::<u64>(rd::<usize>(g_arr).wrapping_add(i * 8));
            if !copy_one_resource_descriptor_inline(dst2, src2 as usize) {
                copy_descriptors_simple(dev as _, 1, dst2, src2 as usize, 0);
            }
            dst1 = dst1.wrapping_add(step);
            dst2 = dst2.wrapping_add(step);
            continue;
        }
        let vtbl1 = rd::<usize>(dev);
        exe::call_va!(rd::<usize>(vtbl1 + 0xc0), (dev, 1u64, dst1, src1, 0u64));

        let arr2 = rd::<usize>(g_arr);
        let src2 = rd::<u64>(arr2.wrapping_add(i * 8));
        let vtbl2 = rd::<usize>(dev);
        exe::call_va!(rd::<usize>(vtbl2 + 0xc0), (dev, 1u64, dst2, src2, 0u64));

        dst1 = dst1.wrapping_add(step);
        dst2 = dst2.wrapping_add(step);
    }

    let mgr_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    exe::call_va!(0x1_4209_3200, (mgr_now));

    let rc1 = rd::<u32>(lock1 + 0x0c).wrapping_sub(1);
    wr::<u32>(lock1 + 0x0c, rc1);
    if rc1 == 0 {
        let a1 = (lock1 + 0x08) as *const core::sync::atomic::AtomicU32;
        (*a1).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
        let _ = exe::call_iat!(0x1_4264_5598, (lock1 as u64) -> u64);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_1422f6ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0xc0];
    let mut buf_n = [0x5au8; 0xc0];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f6ff0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f6ff0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f6ff0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f7040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0xd8];
    let mut buf_n = [0x5au8; 0xd8];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f7040 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f7040 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f7040 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f70a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0xf8];
    let mut buf_n = [0x5au8; 0xf8];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f70a0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f70a0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f70a0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f7120(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0xe0];
    let mut buf_n = [0x5au8; 0xe0];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f7120 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f7120 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f7120 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f71d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0x100];
    let mut buf_n = [0x5au8; 0x100];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f71d0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f71d0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f71d0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f72a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0x120];
    let mut buf_n = [0x5au8; 0x120];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422f72a0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422f72a0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422f72a0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422f7340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut dst_o = [0u64; 10];
    let mut dst_n = [0u64; 10];
    let mut src_o: [u64; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 2];
    let mut src_n = src_o;
    f_o(dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize);
    f_n(dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize);
    crate::diff_eq!(dst_o, dst_n, "1422f7340 move dst");
    crate::diff_eq!(src_o, src_n, "1422f7340 move src");

    let p_o = dst_o.as_mut_ptr() as usize;
    let p_n = dst_n.as_mut_ptr() as usize;
    f_o(p_o, p_o);
    f_n(p_n, p_n);
    crate::diff_eq!(dst_o, dst_n, "1422f7340 self");
    Ok(2)
}

pub unsafe fn diff_rend_1422f73f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut dst_o = [0u64; 10];
    let mut dst_n = [0u64; 10];
    let mut src_o: [u64; 10] = [11, 22, 33, 44, 55, 66, 77, 88, 99, 2];
    let mut src_n = src_o;
    f_o(dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize);
    f_n(dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize);
    crate::diff_eq!(dst_o, dst_n, "1422f73f0 move dst");
    crate::diff_eq!(src_o, src_n, "1422f73f0 move src");

    let p_o = dst_o.as_mut_ptr() as usize;
    let p_n = dst_n.as_mut_ptr() as usize;
    f_o(p_o, p_o);
    f_n(p_n, p_n);
    crate::diff_eq!(dst_o, dst_n, "1422f73f0 self");
    Ok(2)
}

pub unsafe fn diff_rend_1422f8f70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut item_o = [0u8; 0x40];
    let mut item_n = [0u8; 0x40];
    let ptrs_o = [item_o.as_mut_ptr() as u64];
    let ptrs_n = [item_n.as_mut_ptr() as u64];
    let mut eng_o = [0u8; 0x70];
    let mut eng_n = [0u8; 0x70];
    wr::<u64>((eng_o.as_mut_ptr() as usize) + 0x60, ptrs_o.as_ptr() as u64);
    wr::<u32>((eng_o.as_mut_ptr() as usize) + 0x68, 1);
    wr::<u64>((eng_n.as_mut_ptr() as usize) + 0x60, ptrs_n.as_ptr() as u64);
    wr::<u32>((eng_n.as_mut_ptr() as usize) + 0x68, 1);
    f_o(eng_o.as_mut_ptr() as usize);
    f_n(eng_n.as_mut_ptr() as usize);
    crate::diff_eq!(item_o, item_n, "1422f8f70 item");
    crate::diff_eq!(
        rd::<u32>((eng_o.as_ptr() as usize) + 0x68),
        rd::<u32>((eng_n.as_ptr() as usize) + 0x68),
        "1422f8f70 len"
    );
    Ok(1)
}

pub unsafe fn diff_rend_1422f9020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BASE_ADDR: usize = 0;
    static mut TRACE_HASH: u64 = 0;
    unsafe fn record(tag: u64, ptr: usize) {
        let rel = if ptr == 0 {
            0u64
        } else {
            ptr.wrapping_sub(unsafe { BASE_ADDR }) as u64
        };
        unsafe {
            TRACE_HASH = TRACE_HASH
                .wrapping_mul(0x1000_0000_01b3)
                .wrapping_add(tag ^ (rel << 8));
        }
    }
    unsafe extern "win64" fn m_b310(p: usize) {
        unsafe { record(1, p) };
    }
    unsafe extern "win64" fn m_b540(p: usize) {
        unsafe { record(2, p) };
    }
    unsafe extern "win64" fn m_2de0(p: usize) {
        unsafe { record(3, p) };
    }
    unsafe extern "win64" fn m_e830(p: usize) {
        unsafe { record(4, p) };
    }
    unsafe extern "win64" fn m_8630(p: usize) {
        unsafe { record(5, p) };
    }
    unsafe extern "win64" fn m_fd20(p: usize) {
        unsafe { record(6, p) };
    }
    unsafe extern "win64" fn m_d670(p: usize) {
        unsafe { record(7, p) };
    }
    unsafe extern "win64" fn m_7fc0(p: usize) {
        unsafe { record(8, p) };
    }
    unsafe extern "win64" fn m_db80(p: usize) {
        unsafe { record(9, p) };
    }
    unsafe extern "win64" fn m_6af0(p: usize) {
        unsafe { record(10, p) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_b310, m_b310 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b540, m_b540 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, m_2de0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, m_e830 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4233_8630, m_8630 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4130_fd20, m_fd20 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_d670, m_d670 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_411e_7fc0, m_7fc0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4204_db80, m_db80 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4205_6af0, m_6af0 as *const ())?;

    let g_addr = exe::ptr::<u64>(0x1_4397_ff80) as usize;
    let saved_g = rd::<u64>(g_addr);

    for with_ptrs in [false, true] {
        let mut buf_o = [0u8; 0x200];
        let mut buf_n = [0u8; 0x200];
        let po = buf_o.as_mut_ptr() as usize;
        let pn = buf_n.as_mut_ptr() as usize;
        if with_ptrs {
            wr::<usize>(po + 0x08, po + 0x80);
            wr::<usize>(po + 0x10, po + 0x90);
            wr::<usize>(po + 0x18, po + 0xa0);
            wr::<usize>(po + 0x28, po + 0x120);
            wr::<usize>(po + 0x58, po + 0x140);

            wr::<usize>(pn + 0x08, pn + 0x80);
            wr::<usize>(pn + 0x10, pn + 0x90);
            wr::<usize>(pn + 0x18, pn + 0xa0);
            wr::<usize>(pn + 0x28, pn + 0x120);
            wr::<usize>(pn + 0x58, pn + 0x140);
        }

        BASE_ADDR = po;
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        wr::<u64>(g_addr, 0xdead_beef);
        f_o(po);
        let trace_o = TRACE_HASH;
        let g_o = rd::<u64>(g_addr);

        BASE_ADDR = pn;
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        wr::<u64>(g_addr, 0xdead_beef);
        f_n(pn);
        let trace_n = TRACE_HASH;
        let g_n = rd::<u64>(g_addr);

        wr::<u64>(g_addr, saved_g);

        crate::diff_eq!(trace_o, trace_n, "1422f9020 trace with_ptrs={with_ptrs}");
        crate::diff_eq!(g_o, g_n, "1422f9020 global with_ptrs={with_ptrs}");
        crate::diff_eq!(rd::<u64>(po), rd::<u64>(pn), "1422f9020 vtbl");
        crate::diff_eq!(rd::<u64>(po + 0x58), rd::<u64>(pn + 0x58), "1422f9020 p58 zeroed");
    }
    Ok(2)
}

pub unsafe fn diff_rend_1422f9110(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    for v in [0u8, 1, 2, 3, 255] {
        crate::diff_eq!(f_o(v), f_n(v), "1422f9110");
    }
    Ok(5)
}

pub unsafe fn diff_rend_1422f9140(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn mock_55020(
        ctx: usize,
        a1: u8,
        a0: u32,
        a2: u8,
        a3_bits: u32,
        a4_bits: u32,
        a5: u8,
    ) {
        wr::<u32>(ctx, a0);
        wr::<u8>(ctx + 0x4, a1);
        wr::<u8>(ctx + 0x5, a2);
        wr::<u8>(ctx + 0x6, a5);
        wr::<u32>(ctx + 0x8, a3_bits);
        wr::<u32>(ctx + 0xc, a4_bits);
    }
    let _m = crate::difftest::Mock::new(0x1_4205_5020, mock_55020 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0u8; 0x6350];
    let mut tls_n = vec![0u8; 0x6350];
    let mut ctx_o = [0u8; 16];
    let mut ctx_n = [0u8; 16];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;
    wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6338, ctx_o.as_mut_ptr() as usize);
    wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6338, ctx_n.as_mut_ptr() as usize);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    let _ = crate::simd_ops::call_va8_xmm(
        orig as u64,
        0x1234_5678,
        3,
        7,
        0,
        (-2.5f32).to_bits() as u64,
        9,
        0,
        0,
        [0, 0, 0, 1.25f32.to_bits() as u128],
    );

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    let _ = crate::simd_ops::call_va8_xmm(
        new as u64,
        0x1234_5678,
        3,
        7,
        0,
        (-2.5f32).to_bits() as u64,
        9,
        0,
        0,
        [0, 0, 0, 1.25f32.to_bits() as u128],
    );

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(ctx_o, ctx_n, "1422f9140 forwarded args");
    Ok(1)
}

pub unsafe fn diff_rend_1422f91e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, i32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    const ADDRS: [usize; 5] = [
        0x1_4397_fcc8,
        0x1_4397_fce8,
        0x1_4397_fd00,
        0x1_4397_fd10,
        0x1_4397_fd18,
    ];
    let saved = [
        rd::<u64>(ADDRS[0]),
        rd::<u64>(ADDRS[1]),
        rd::<u64>(ADDRS[2]),
        rd::<u64>(ADDRS[3]),
        rd::<u64>(ADDRS[4]),
    ];
    for (i, &addr) in ADDRS.iter().enumerate() {
        wr::<u64>(addr, 0x1000 + (i as u64) * 0x111);
    }
    let mut res = Ok(0u32);
    'outer: for a in [0i32, 1, 2, 3, 4] {
        for b in [0i32, 1] {
            for c in [0i32, 1, 9, 0xe, 0x13, 0x14, 0x15] {
                let r_o = f_o(a, b, c);
                let r_n = f_n(a, b, c);
                if r_o != r_n {
                    res = Err(format!("1422f91e0 mismatch at ({a},{b},{c}): {r_o:#x} != {r_n:#x}"));
                    break 'outer;
                }
                if let Ok(cnt) = &mut res {
                    *cnt += 1;
                }
            }
        }
    }
    for (i, &addr) in ADDRS.iter().enumerate() {
        wr::<u64>(addr, saved[i]);
    }
    res
}

pub unsafe fn diff_rend_1422f9250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, i32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    const ADDRS: [usize; 6] = [
        0x1_4397_fd30,
        0x1_4397_fd48,
        0x1_4397_fd68,
        0x1_4397_fd90,
        0x1_4397_fda0,
        0x1_4397_fe48,
    ];
    let saved = [
        rd::<u64>(ADDRS[0]),
        rd::<u64>(ADDRS[1]),
        rd::<u64>(ADDRS[2]),
        rd::<u64>(ADDRS[3]),
        rd::<u64>(ADDRS[4]),
        rd::<u64>(ADDRS[5]),
    ];
    for (i, &addr) in ADDRS.iter().enumerate() {
        wr::<u64>(addr, 0x2000 + (i as u64) * 0x222);
    }
    let mut res = Ok(0u32);
    'outer: for a in [0i32, 1, 2, 3] {
        for b in [0i32, 1] {
            for c in [0i32, 1, 2, 4, 5, 7, 9, 0xe, 0x13] {
                let r_o = f_o(a, b, c);
                let r_n = f_n(a, b, c);
                if r_o != r_n {
                    res = Err(format!("1422f9250 mismatch at ({a},{b},{c}): {r_o:#x} != {r_n:#x}"));
                    break 'outer;
                }
                if let Ok(cnt) = &mut res {
                    *cnt += 1;
                }
            }
        }
    }
    for (i, &addr) in ADDRS.iter().enumerate() {
        wr::<u64>(addr, saved[i]);
    }
    res
}

pub unsafe fn diff_rend_1422f92d0(orig: usize, new: usize) -> Result<u32, String> {
    let saved_denom = rd::<u32>(0x1_43a0_0148);
    wr::<f32>(0x1_43a0_0148, 1000.0);
    let mut res = Ok(0u32);
    'outer: for mode in [0u32, 4, 9, 10, 99] {
        for dist in [0.0f32, 10.0, 500.0, 2000.0] {
            for bias in [0i32, 100, -50, 70000] {
                let r_o = crate::simd_ops::call_va8_xmm(
                    orig as u64,
                    mode as u64,
                    0,
                    bias as i64 as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, dist.to_bits() as u128, 0, 0],
                )
                .0 as u32;
                let r_n = crate::simd_ops::call_va8_xmm(
                    new as u64,
                    mode as u64,
                    0,
                    bias as i64 as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, dist.to_bits() as u128, 0, 0],
                )
                .0 as u32;
                if r_o != r_n {
                    res = Err(format!("1422f92d0 mismatch: {r_o:#x} != {r_n:#x}"));
                    break 'outer;
                }
                if let Ok(cnt) = &mut res {
                    *cnt += 1;
                }
            }
        }
    }
    wr::<u32>(0x1_43a0_0148, saved_denom);
    res
}

pub unsafe fn diff_rend_1422f9640(orig: usize, new: usize) -> Result<u32, String> {
    let saved_138 = rd::<u32>(0x1_4386_c138);
    let saved_158 = rd::<u32>(0x1_4386_c158);
    let mut res = Ok(0u32);
    'outer: for (t138, t158) in [(10.0f32, 5.0f32), (-1.0, -1.0), (10.0, -1.0)] {
        wr::<f32>(0x1_4386_c138, t138);
        wr::<f32>(0x1_4386_c158, t158);
        for mode in [0i32, 1, 2, 3, 4] {
            for val in [2.0f32, 7.0, 15.0] {
                let r_o = crate::simd_ops::call_va8_xmm(
                    orig as u64,
                    0,
                    mode as i64 as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [val.to_bits() as u128, 0, 0, 0],
                )
                .0 as u8;
                let r_n = crate::simd_ops::call_va8_xmm(
                    new as u64,
                    0,
                    mode as i64 as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [val.to_bits() as u128, 0, 0, 0],
                )
                .0 as u8;
                if r_o != r_n {
                    res = Err(format!("1422f9640 mismatch: {r_o} != {r_n}"));
                    break 'outer;
                }
                if let Ok(cnt) = &mut res {
                    *cnt += 1;
                }
            }
        }
    }
    wr::<u32>(0x1_4386_c138, saved_138);
    wr::<u32>(0x1_4386_c158, saved_158);
    res
}

pub unsafe fn diff_rend_1422fc110(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut arr_o: [u64; 4] = [10, 20, 30, 20];
    let mut arr_n = arr_o;
    let mut obj_o = [0u8; 0x70];
    let mut obj_n = [0u8; 0x70];
    wr::<u64>((obj_o.as_mut_ptr() as usize) + 0x60, arr_o.as_mut_ptr() as u64);
    wr::<u32>((obj_o.as_mut_ptr() as usize) + 0x68, 4);
    wr::<u64>((obj_n.as_mut_ptr() as usize) + 0x60, arr_n.as_mut_ptr() as u64);
    wr::<u32>((obj_n.as_mut_ptr() as usize) + 0x68, 4);
    f_o(obj_o.as_mut_ptr() as usize, 20);
    f_n(obj_n.as_mut_ptr() as usize, 20);
    let len_o = rd::<u32>((obj_o.as_ptr() as usize) + 0x68) as usize;
    let len_n = rd::<u32>((obj_n.as_ptr() as usize) + 0x68) as usize;
    crate::diff_eq!(len_o, len_n, "1422fc110 len");
    crate::diff_eq!(arr_o[..len_o], arr_n[..len_n], "1422fc110 items");
    Ok(1)
}

pub unsafe fn diff_rend_1422fc1d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    for flag193 in [0u8, 1u8] {
        let mut st_o = [0u8; 0x1a0];
        let mut st_n = [0u8; 0x1a0];
        st_o[0x193] = flag193;
        st_n[0x193] = flag193;
        f_o(0, st_o.as_mut_ptr() as usize);
        f_n(0, st_n.as_mut_ptr() as usize);
        crate::diff_eq!(st_o, st_n, "1422fc1d0 empty loop");
    }
    Ok(2)
}

pub unsafe fn diff_rend_1422fcae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0x70];
    let mut buf_n = [0x5au8; 0x70];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422fcae0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422fcae0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422fcae0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422fd680(orig: usize, new: usize) -> Result<u32, String> {
    let mut this_buf = [0u8; 0x3b8];
    wr::<f32>((this_buf.as_mut_ptr() as usize) + 0x3a4, 1.5);
    wr::<f32>((this_buf.as_mut_ptr() as usize) + 0x3ac, 0.25);
    let mut cam_buf = [0u8; 0x160];
    for i in 0..16 {
        wr::<f64>((cam_buf.as_mut_ptr() as usize) + 0xd0 + i * 8, 2.0 + (i as f64) * 0.5);
    }
    for i in 0..12 {
        wr::<f64>((cam_buf.as_mut_ptr() as usize) + 0x10 + i * 8, 1.0 + (i as f64) * 0.25);
    }
    for dist in [0.0f32, 0.00005, 0.5, 2.0, 10.0, 100.0] {
        let r_o = crate::simd_ops::call_va8_xmm(
            orig as u64,
            this_buf.as_ptr() as usize as u64,
            0,
            cam_buf.as_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dist.to_bits() as u128, 0, 0],
        )
        .1 as u32;
        let r_n = crate::simd_ops::call_va8_xmm(
            new as u64,
            this_buf.as_ptr() as usize as u64,
            0,
            cam_buf.as_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dist.to_bits() as u128, 0, 0],
        )
        .1 as u32;
        crate::diff_eq!(r_o, r_n, "1422fd680");
    }
    Ok(6)
}

pub unsafe fn diff_rend_1422fd730(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    for flag in [0u8, 1u8] {
        for denom in [1u16, 16, 64, 256, 1024] {
            let mut buf = [0u8; 0x1a0];
            wr::<u16>((buf.as_mut_ptr() as usize) + 0x190, denom);
            wr::<u8>((buf.as_mut_ptr() as usize) + 0x193, flag);
            crate::diff_eq!(
                f_o(buf.as_ptr() as usize),
                f_n(buf.as_ptr() as usize),
                "1422fd730"
            );
        }
    }
    Ok(10)
}

pub unsafe fn diff_rend_1422fd760(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf = [0u8; 0x50 + 4 * 0x60];
    for i in 0..4usize {
        wr::<u64>((buf.as_mut_ptr() as usize) + 0x50 + i * 0x60, 0xdead_0000 + i as u64);
    }
    for idx in 0..4u8 {
        crate::diff_eq!(
            f_o(buf.as_ptr() as usize, idx),
            f_n(buf.as_ptr() as usize, idx),
            "1422fd760"
        );
    }
    Ok(4)
}

pub unsafe fn diff_rend_1422fd780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let pool = [0u64; 8];
    let mut this_buf = [0u8; 0x60];
    wr::<u64>((this_buf.as_mut_ptr() as usize) + 0x58, pool.as_ptr() as u64);

    f_o(this_buf.as_ptr() as usize, 0);
    f_n(this_buf.as_ptr() as usize, 0);

    let mut rc_o = [[0u8; 0x30]; 3];
    let mut rc_n = [[0u8; 0x30]; 3];
    for i in 0..3 {
        wr::<i32>((rc_o[i].as_mut_ptr() as usize) + 0x24, (i as i32) + 1);
        wr::<i32>((rc_n[i].as_mut_ptr() as usize) + 0x24, (i as i32) + 1);
    }
    let mut item_o = [0u8; 0x1f0];
    let mut item_n = [0u8; 0x1f0];
    wr::<u64>((item_o.as_mut_ptr() as usize) + 0x1d0, rc_o[0].as_mut_ptr() as u64);
    wr::<u64>((item_o.as_mut_ptr() as usize) + 0x1d8, rc_o[1].as_mut_ptr() as u64);
    wr::<u64>((item_o.as_mut_ptr() as usize) + 0x1e0, rc_o[2].as_mut_ptr() as u64);
    wr::<u64>((item_n.as_mut_ptr() as usize) + 0x1d0, rc_n[0].as_mut_ptr() as u64);
    wr::<u64>((item_n.as_mut_ptr() as usize) + 0x1d8, rc_n[1].as_mut_ptr() as u64);
    wr::<u64>((item_n.as_mut_ptr() as usize) + 0x1e0, rc_n[2].as_mut_ptr() as u64);

    f_o(this_buf.as_ptr() as usize, item_o.as_mut_ptr() as usize);
    f_n(this_buf.as_ptr() as usize, item_n.as_mut_ptr() as usize);
    crate::diff_eq!(item_o, item_n, "1422fd780 item");
    crate::diff_eq!(rc_o, rc_n, "1422fd780 refcounts");
    Ok(2)
}

pub unsafe fn diff_rend_1422fde70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut elems = [0u8; 2 * 0x70];
    wr::<u8>((elems.as_mut_ptr() as usize) + 0x58, 2);
    wr::<u32>((elems.as_mut_ptr() as usize) + 0x70 + 0x54, 2);
    let mut this_o = [0u8; 0x30];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x18, elems.as_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x20, 2);
    let mut this_n = this_o;
    f_o(this_o.as_mut_ptr() as usize);
    f_n(this_n.as_mut_ptr() as usize);
    crate::diff_eq!(this_o, this_n, "1422fde70");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 0x70];
    let mut storage_n = [0x5au8; 0x70];
    let mut this_o = [0u8; 0x20];
    let mut this_n = [0u8; 0x20];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x8, storage_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x14, 2);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x8, storage_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x14, 2);
    f_o(this_o.as_mut_ptr() as usize, 0x1122_3344, 0x5566_7788);
    f_n(this_n.as_mut_ptr() as usize, 0x1122_3344, 0x5566_7788);
    crate::diff_eq!(storage_o, storage_n, "1422fe070 entry");
    crate::diff_eq!(this_o[0x10..0x18], this_n[0x10..0x18], "1422fe070 vec");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe0c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 0x70];
    let mut storage_n = [0x5au8; 0x70];
    let mut this_o = [0u8; 0x20];
    let mut this_n = [0u8; 0x20];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x8, storage_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x14, 2);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x8, storage_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x14, 2);
    let desc = [1u32, 2, 3, 4, 5, 6];
    f_o(this_o.as_mut_ptr() as usize, 0x9988_7766, desc.as_ptr() as usize, 7);
    f_n(this_n.as_mut_ptr() as usize, 0x9988_7766, desc.as_ptr() as usize, 7);
    crate::diff_eq!(storage_o, storage_n, "1422fe0c0 entry");
    crate::diff_eq!(this_o[0x10..0x18], this_n[0x10..0x18], "1422fe0c0 vec");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe130(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 0x70];
    let mut storage_n = [0x5au8; 0x70];
    let mut this_o = [0u8; 0x20];
    let mut this_n = [0u8; 0x20];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x8, storage_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x14, 2);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x8, storage_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x14, 2);
    let desc = [10u32, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120];
    f_o(this_o.as_mut_ptr() as usize, 0xaabb_ccdd, desc.as_ptr() as usize, 3);
    f_n(this_n.as_mut_ptr() as usize, 0xaabb_ccdd, desc.as_ptr() as usize, 3);
    crate::diff_eq!(storage_o, storage_n, "1422fe130 entry");
    crate::diff_eq!(this_o[0x10..0x18], this_n[0x10..0x18], "1422fe130 vec");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe1a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage = [0u8; 3 * 0x70];
    wr::<u64>(storage.as_mut_ptr() as usize, 111);
    wr::<u64>((storage.as_mut_ptr() as usize) + 0x70, 222);
    wr::<u64>((storage.as_mut_ptr() as usize) + 0xe0, 333);
    let mut this_buf = [0u8; 0x20];
    wr::<u64>((this_buf.as_mut_ptr() as usize) + 0x8, storage.as_ptr() as u64);
    wr::<u32>((this_buf.as_mut_ptr() as usize) + 0x10, 3);
    for k in [111u64, 222, 333, 444] {
        crate::diff_eq!(
            f_o(this_buf.as_ptr() as usize, k),
            f_n(this_buf.as_ptr() as usize, k),
            "1422fe1a0"
        );
    }
    Ok(4)
}

pub unsafe fn diff_rend_1422fe1e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 0x70];
    let mut storage_n = [0x5au8; 0x70];
    let mut this_o = [0x5au8; 0x20];
    let mut this_n = [0x5au8; 0x20];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x8, storage_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x14, 2);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x8, storage_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x14, 2);
    f_o(this_o.as_mut_ptr() as usize, 0x1234_5678);
    f_n(this_n.as_mut_ptr() as usize, 0x1234_5678);
    crate::diff_eq!(storage_o, storage_n, "1422fe1e0 entry");
    crate::diff_eq!(this_o[0], this_n[0], "1422fe1e0 b0");
    crate::diff_eq!(this_o[0x10..0x18], this_n[0x10..0x18], "1422fe1e0 vec");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 0x70];
    let mut storage_n = [0x5au8; 0x70];
    let mut this_o = [0u8; 0x20];
    let mut this_n = [0u8; 0x20];
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x8, storage_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x14, 2);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x8, storage_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x14, 2);
    f_o(this_o.as_mut_ptr() as usize, 0x8765_4321);
    f_n(this_n.as_mut_ptr() as usize, 0x8765_4321);
    crate::diff_eq!(storage_o, storage_n, "1422fe270 entry");
    crate::diff_eq!(this_o[0x10..0x18], this_n[0x10..0x18], "1422fe270 vec");
    Ok(1)
}

pub unsafe fn diff_rend_1422fe2f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0x28];
    let mut buf_n = [0x5au8; 0x28];
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, buf_o.as_mut_ptr() as usize, "1422fe2f0 ret_o");
    crate::diff_eq!(r_n, buf_n.as_mut_ptr() as usize, "1422fe2f0 ret_n");
    crate::diff_eq!(buf_o, buf_n, "1422fe2f0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_1422ff4f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let saved_ptr = rd::<u64>(0x1_43bb_d1a0);
    let saved_len = rd::<u32>(0x1_43bb_d1a8);
    let saved_cap = rd::<u32>(0x1_43bb_d1ac);

    let mut buf_o = [0u64; 8];
    buf_o[0] = 0x1111;
    let mut buf_n = buf_o;

    wr::<u64>(0x1_43bb_d1a0, buf_o.as_mut_ptr() as u64);
    wr::<u32>(0x1_43bb_d1a8, 1);
    wr::<u32>(0x1_43bb_d1ac, 4);
    let r_hit_o = f_o(0, 0x1111);
    let r_new_o = f_o(0, 0x2222);
    let len_o = rd::<u32>(0x1_43bb_d1a8);

    wr::<u64>(0x1_43bb_d1a0, buf_n.as_mut_ptr() as u64);
    wr::<u32>(0x1_43bb_d1a8, 1);
    wr::<u32>(0x1_43bb_d1ac, 4);
    let r_hit_n = f_n(0, 0x1111);
    let r_new_n = f_n(0, 0x2222);
    let len_n = rd::<u32>(0x1_43bb_d1a8);

    wr::<u64>(0x1_43bb_d1a0, saved_ptr);
    wr::<u32>(0x1_43bb_d1a8, saved_len);
    wr::<u32>(0x1_43bb_d1ac, saved_cap);

    crate::diff_eq!(r_hit_o, r_hit_n, "1422ff4f0 hit");
    crate::diff_eq!(r_new_o, r_new_n, "1422ff4f0 append ret");
    crate::diff_eq!(len_o, len_n, "1422ff4f0 len");
    crate::diff_eq!(buf_o, buf_n, "1422ff4f0 buf");
    Ok(2)
}

pub unsafe fn diff_rend_1422fffa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let saved_170 = rd::<u64>(0x1_43bb_d170);
    let saved_178 = rd::<u64>(0x1_43bb_d178);
    let saved_180 = rd::<u64>(0x1_43bb_d180);

    let mut src = [0u8; 4 * 0x30];
    wr::<i32>(src.as_mut_ptr() as usize, 0);
    wr::<i32>((src.as_mut_ptr() as usize) + 0x30, 0);
    wr::<i32>((src.as_mut_ptr() as usize) + 0x60, -1);
    wr::<i32>((src.as_mut_ptr() as usize) + 0x90, 1);

    let mut dst_o = [0u8; 2 * 0x2e0];
    let mut dst_n = [0u8; 2 * 0x2e0];

    wr::<u64>(0x1_43bb_d170, src.as_ptr() as u64);
    wr::<u64>(0x1_43bb_d178, 4);
    wr::<u64>(0x1_43bb_d180, dst_o.as_mut_ptr() as u64);
    f_o();

    wr::<u64>(0x1_43bb_d180, dst_n.as_mut_ptr() as u64);
    f_n();

    wr::<u64>(0x1_43bb_d170, saved_170);
    wr::<u64>(0x1_43bb_d178, saved_178);
    wr::<u64>(0x1_43bb_d180, saved_180);

    crate::diff_eq!(dst_o, dst_n, "1422fffa0");
    Ok(1)
}

pub unsafe fn diff_rend_142300010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut entries_o = [0u8; 2 * 0x2e0];
    let mut entries_n = [0u8; 2 * 0x2e0];
    let mut this_o = [0u8; 0x50];
    let mut this_n = [0u8; 0x50];
    wr::<i32>((this_o.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x40, entries_o.as_mut_ptr() as u64);
    wr::<i32>((this_n.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x40, entries_n.as_mut_ptr() as u64);
    f_o(this_o.as_ptr() as usize, 0xcafe_babe_1234_5678);
    f_n(this_n.as_ptr() as usize, 0xcafe_babe_1234_5678);
    crate::diff_eq!(entries_o, entries_n, "142300010");
    Ok(1)
}

pub unsafe fn diff_rend_142300030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut entries_o = [0u8; 2 * 0x2e0];
    let mut entries_n = [0u8; 2 * 0x2e0];
    let mut vec_o = [0u64; 4];
    let mut vec_n = [0u64; 4];
    let mut this_o = [0u8; 0x80];
    let mut this_n = [0u8; 0x80];
    wr::<i32>((this_o.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x40, entries_o.as_mut_ptr() as u64);
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x50, vec_o.as_mut_ptr() as u64);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x58, 1);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x5c, 4);

    wr::<i32>((this_n.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x40, entries_n.as_mut_ptr() as u64);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x50, vec_n.as_mut_ptr() as u64);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x58, 1);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x5c, 4);

    f_o(this_o.as_mut_ptr() as usize, 0x77);
    f_n(this_n.as_mut_ptr() as usize, 0x77);
    crate::diff_eq!(vec_o, vec_n, "142300030 vec");
    crate::diff_eq!(entries_o, entries_n, "142300030 entries");
    Ok(1)
}

pub unsafe fn diff_rend_142300100(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut entries_o = [0u8; 2 * 0x2e0];
    let mut entries_n = [0u8; 2 * 0x2e0];
    let mut vec_o = [0u64; 4];
    let mut vec_n = [0u64; 4];
    let mut inner_o = [0u8; 0x80];
    let mut inner_n = [0u8; 0x80];
    wr::<u64>((inner_o.as_mut_ptr() as usize) + 0x40, entries_o.as_mut_ptr() as u64);
    wr::<u64>((inner_o.as_mut_ptr() as usize) + 0x50, vec_o.as_mut_ptr() as u64);
    wr::<u32>((inner_o.as_mut_ptr() as usize) + 0x58, 1);
    wr::<u32>((inner_o.as_mut_ptr() as usize) + 0x5c, 4);

    wr::<u64>((inner_n.as_mut_ptr() as usize) + 0x40, entries_n.as_mut_ptr() as u64);
    wr::<u64>((inner_n.as_mut_ptr() as usize) + 0x50, vec_n.as_mut_ptr() as u64);
    wr::<u32>((inner_n.as_mut_ptr() as usize) + 0x58, 1);
    wr::<u32>((inner_n.as_mut_ptr() as usize) + 0x5c, 4);

    let wrap_o = [1u64, inner_o.as_mut_ptr() as u64];
    let wrap_n = [1u64, inner_n.as_mut_ptr() as u64];
    f_o(wrap_o.as_ptr() as usize);
    f_n(wrap_n.as_ptr() as usize);
    crate::diff_eq!(vec_o, vec_n, "142300100 vec");
    crate::diff_eq!(entries_o, entries_n, "142300100 entries");
    Ok(1)
}

pub unsafe fn diff_rend_1423001e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf_o = [0u8; 0xb0];
    let mut buf_n = [0u8; 0xb0];
    wr::<u32>((buf_o.as_mut_ptr() as usize) + 0xa0, 41);
    wr::<u32>((buf_n.as_mut_ptr() as usize) + 0xa0, 41);
    let r_o = f_o(buf_o.as_mut_ptr() as usize);
    let r_n = f_n(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "1423001e0 ret");
    crate::diff_eq!(buf_o, buf_n, "1423001e0 buf");
    Ok(1)
}

pub unsafe fn diff_rend_142300210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut storage_o = [0x5au8; 2 * 0x30];
    let mut storage_n = [0x5au8; 2 * 0x30];
    let mut inner_o = [0u8; 0x50];
    let mut inner_n = [0u8; 0x50];
    wr::<u64>((inner_o.as_mut_ptr() as usize) + 0x30, storage_o.as_mut_ptr() as u64);
    wr::<u32>((inner_o.as_mut_ptr() as usize) + 0x3c, 2);
    wr::<u64>((inner_n.as_mut_ptr() as usize) + 0x30, storage_n.as_mut_ptr() as u64);
    wr::<u32>((inner_n.as_mut_ptr() as usize) + 0x3c, 2);
    let wrap_o = [7u64, inner_o.as_mut_ptr() as u64];
    let wrap_n = [7u64, inner_n.as_mut_ptr() as u64];
    f_o(wrap_o.as_ptr() as usize);
    f_n(wrap_n.as_ptr() as usize);
    crate::diff_eq!(storage_o, storage_n, "142300210 storage");
    crate::diff_eq!(inner_o[0x38..0x40], inner_n[0x38..0x40], "142300210 vec");
    Ok(1)
}

pub unsafe fn diff_rend_142300250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let saved_168 = rd::<i32>(0x1_43bb_d168);
    let saved_180 = rd::<u64>(0x1_43bb_d180);

    let mut entries_o = [0u8; 2 * 0x2e0];
    let mut entries_n = [0u8; 2 * 0x2e0];
    let mut fn_o = [0u64; 8];
    fn_o[0] = 0x1_433f_3eb0;
    fn_o[1] = 0xdead_beef_cafe_babe;
    fn_o[7] = fn_o.as_ptr() as u64;
    let mut fn_n = fn_o;
    fn_n[7] = fn_n.as_ptr() as u64;

    wr::<i32>(0x1_43bb_d168, 1);
    wr::<u64>(0x1_43bb_d180, entries_o.as_mut_ptr() as u64);
    f_o(0, fn_o.as_mut_ptr() as usize);

    wr::<u64>(0x1_43bb_d180, entries_n.as_mut_ptr() as u64);
    f_n(0, fn_n.as_mut_ptr() as usize);

    wr::<i32>(0x1_43bb_d168, saved_168);
    wr::<u64>(0x1_43bb_d180, saved_180);

    crate::diff_eq!(fn_o[7], fn_n[7], "142300250 fn cleared");
    crate::diff_eq!(
        entries_o[0x2e0 + 0x90..0x2e0 + 0xa0],
        entries_n[0x2e0 + 0x90..0x2e0 + 0xa0],
        "142300250 moved sbo"
    );
    Ok(1)
}

pub unsafe fn diff_rend_1423002b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut entries_o = [0u8; 2 * 0x2e0];
    let mut entries_n = [0u8; 2 * 0x2e0];
    let mut this_o = [0u8; 0x50];
    let mut this_n = [0u8; 0x50];
    wr::<i32>((this_o.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_o.as_mut_ptr() as usize) + 0x40, entries_o.as_mut_ptr() as u64);
    wr::<i32>((this_n.as_mut_ptr() as usize) + 0x28, 1);
    wr::<u64>((this_n.as_mut_ptr() as usize) + 0x40, entries_n.as_mut_ptr() as u64);

    let mut fn_o = [0u64; 8];
    fn_o[0] = 0x1_433f_3eb0;
    fn_o[1] = 0x1234_5678_9abc_def0;
    fn_o[7] = fn_o.as_ptr() as u64;
    let mut fn_n = fn_o;
    fn_n[7] = fn_n.as_ptr() as u64;

    f_o(this_o.as_ptr() as usize, fn_o.as_mut_ptr() as usize);
    f_n(this_n.as_ptr() as usize, fn_n.as_mut_ptr() as usize);

    crate::diff_eq!(fn_o[7], fn_n[7], "1423002b0 fn cleared");
    crate::diff_eq!(
        entries_o[0x2e0 + 0x50..0x2e0 + 0x60],
        entries_n[0x2e0 + 0x50..0x2e0 + 0x60],
        "1423002b0 moved sbo"
    );
    Ok(1)
}

pub unsafe fn diff_rend_14226c9b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop1(_a: u64) -> u64 {
        0
    }
    unsafe extern "win64" fn mock_2986e0(this: usize, p3: usize, a3: u64, p4: usize) {
        wr::<u64>(this, (p3 as u64) ^ a3 ^ (p4 as u64));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4226_cd10, noop1 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4229_86e0, mock_2986e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4226_bb40, noop1 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4226_a820, noop1 as *const ())?;

    let g_cnt = exe::ptr::<u32>(0x1_4385_83b0) as usize;
    let g_ca = exe::ptr::<u8>(0x1_4385_83ca) as usize;
    let g_ctr = exe::ptr::<i32>(0x1_43a2_b19c) as usize;
    let s_cnt = rd::<u32>(g_cnt);
    let s_ca = rd::<u8>(g_ca);
    let s_ctr = rd::<i32>(g_ctr);
    wr::<u32>(g_cnt, 0);
    wr::<u8>(g_ca, 0);
    wr::<i32>(g_ctr, 0);

    let mut this_o = [0u64; 4];
    let mut this_n = [0u64; 4];
    let mut p2 = [0u8; 16];
    wr::<u32>(p2.as_mut_ptr() as usize, 0xffff_ffff);
    wr::<u8>(p2.as_mut_ptr() as usize + 8, 1);
    wr::<u8>(p2.as_mut_ptr() as usize + 9, 1);

    f_o(this_o.as_mut_ptr() as usize, p2.as_ptr() as usize, 0x1111, 0x2222);
    f_n(this_n.as_mut_ptr() as usize, p2.as_ptr() as usize, 0x1111, 0x2222);

    wr::<u32>(g_cnt, s_cnt);
    wr::<u8>(g_ca, s_ca);
    wr::<i32>(g_ctr, s_ctr);

    crate::diff_eq!(this_o, this_n, "14226c9b0 this");
    Ok(1)
}

pub unsafe fn diff_rend_1422fd9c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_57c60(mgr: u64, _desc: u64, _tag: u64) -> u64 {
        mgr ^ 0x7777
    }
    unsafe extern "win64" fn mock_58080(_mgr: u64, _h: u64) -> u64 {
        0
    }
    let _m1 = crate::difftest::Mock::new(0x1_4225_7c60, mock_57c60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_8080, mock_58080 as *const ())?;

    let g_mgr = exe::ptr::<u64>(0x1_4397_ff18) as usize;
    let s_mgr = rd::<u64>(g_mgr);
    wr::<u64>(g_mgr, 0x1000);

    let run = |f: F| -> [u64; 2] {
        let mut entries = [0u8; 0xe0];
        let ep = entries.as_mut_ptr() as usize;
        // Entry 0: handle == 0, first_pass == 2, kind == 1
        wr::<i32>(ep + 0x54, 1);
        wr::<u32>(ep + 0x58, 0);
        wr::<u64>(ep + 0x60, 0);
        wr::<i32>(ep + 0x68, 2);
        // Entry 1: handle != 0, last_pass == 1, kind == 0
        wr::<i32>(ep + 0x70 + 0x54, 0);
        wr::<u32>(ep + 0x70 + 0x58, 0);
        wr::<u64>(ep + 0x70 + 0x60, 0x9999);
        wr::<i32>(ep + 0x70 + 0x6c, 1);

        let mut this_buf = [0u8; 0x20];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, ep);
        wr::<u32>(tp + 0x10, 2);

        f(tp, 2);
        [rd::<u64>(ep + 0x60), rd::<u64>(ep + 0x70 + 0x60)]
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<u64>(g_mgr, s_mgr);
    crate::diff_eq!(o, n, "1422fd9c0 handles");
    Ok(1)
}

pub unsafe fn diff_rend_1422ff5c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_order_ptr = exe::ptr::<u64>(0x1_43bb_d148) as usize;
    let g_passes_ptr = exe::ptr::<u64>(0x1_43bb_d180) as usize;
    let g_target_pass = exe::ptr::<i32>(0x1_43bb_d1b0) as usize;
    let g_valid_flag = exe::ptr::<u8>(0x1_43bb_d1b4) as usize;

    let s_order = rd::<[u64; 2]>(g_order_ptr);
    let s_passes = rd::<[u64; 2]>(g_passes_ptr);
    let s_target = rd::<i32>(g_target_pass);
    let s_valid = rd::<u8>(g_valid_flag);

    let run = |f: F| -> ([u32; 2], u32, u8) {
        let mut order_buf = [0u32; 4];
        let mut passes_buf = [0u8; 0x2e0 * 2];
        let pb = passes_buf.as_mut_ptr() as usize;
        // Pass 0 produces bit 0 (at +0x1d0), Pass 1 requires bit 0 (at +0xd0)
        wr::<u64>(pb + 0x1d0, 1);
        wr::<u64>(pb + 0x2e0 + 0xd0, 1);

        wr::<usize>(g_order_ptr, order_buf.as_mut_ptr() as usize);
        wr::<u32>(g_order_ptr + 8, 0);
        wr::<u32>(g_order_ptr + 12, 4);
        wr::<usize>(g_passes_ptr, pb);
        wr::<u32>(g_passes_ptr + 8, 2);
        wr::<i32>(g_target_pass, 1);
        wr::<u8>(g_valid_flag, 1);

        f();
        (
            [order_buf[0], order_buf[1]],
            rd::<u32>(g_order_ptr + 8),
            rd::<u8>(g_valid_flag),
        )
    };
    let o = run(f_o);
    let n = run(f_n);

    wr::<[u64; 2]>(g_order_ptr, s_order);
    wr::<[u64; 2]>(g_passes_ptr, s_passes);
    wr::<i32>(g_target_pass, s_target);
    wr::<u8>(g_valid_flag, s_valid);

    crate::diff_eq!(o, n, "1422ff5c0 order");
    Ok(1)
}

pub unsafe fn diff_rend_142300560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> i32 {
        let mut passes = [0u8; 0x2e0 * 2];
        let pb = passes.as_mut_ptr() as usize;
        wr::<i32>(pb, 10);
        wr::<i32>(pb + 0x2e0, 25);

        let mut this_buf = [0u8; 0x60];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x40, pb);
        wr::<u32>(tp + 0x48, 2);

        f(tp, 25);
        rd::<i32>(tp + 0x28)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142300560 found idx");
    Ok(1)
}

pub unsafe fn diff_rend_142300ab0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop_sort(_a: usize, _b: usize, _c: u64, _d: u64) {}
    unsafe extern "win64" fn noop0() {}
    let _m1 = crate::difftest::Mock::new(0x1_4131_0580, noop_sort as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422f_f5c0, noop0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_422f_e4e0, noop0 as *const ())?;

    let g_edges_ptr = exe::ptr::<u64>(0x1_43bb_d170) as usize;
    let g_passes_ptr = exe::ptr::<u64>(0x1_43bb_d180) as usize;
    let g_res_ptr = exe::ptr::<u64>(0x1_43bb_d190) as usize;
    let g_valid_flag = exe::ptr::<u8>(0x1_43bb_d1b4) as usize;

    let s_edges = rd::<[u64; 2]>(g_edges_ptr);
    let s_passes = rd::<[u64; 2]>(g_passes_ptr);
    let s_res = rd::<[u64; 2]>(g_res_ptr);
    let s_valid = rd::<u8>(g_valid_flag);

    let run = |f: F| -> (u8, u64, u32) {
        let mut edges = [0u8; 0x30];
        let ep = edges.as_mut_ptr() as usize;
        wr::<i32>(ep, 0);
        wr::<i32>(ep + 4, 1);
        wr::<i32>(ep + 8, 42);

        let mut passes = [0xaa_u8; 0x2e0];
        let pb = passes.as_mut_ptr() as usize;
        let mut res = [0u64; 2];
        wr::<i32>(res.as_mut_ptr() as usize + 8, 42);

        wr::<usize>(g_edges_ptr, ep);
        wr::<u32>(g_edges_ptr + 8, 1);
        wr::<usize>(g_passes_ptr, pb);
        wr::<u32>(g_passes_ptr + 8, 1);
        wr::<usize>(g_res_ptr, res.as_mut_ptr() as usize);
        wr::<u32>(g_res_ptr + 8, 2);
        wr::<u8>(g_valid_flag, 1);

        let ret = f();
        (ret, rd::<u64>(pb + 0xd0), rd::<u32>(pb + 4))
    };
    let o = run(f_o);
    let n = run(f_n);

    wr::<[u64; 2]>(g_edges_ptr, s_edges);
    wr::<[u64; 2]>(g_passes_ptr, s_passes);
    wr::<[u64; 2]>(g_res_ptr, s_res);
    wr::<u8>(g_valid_flag, s_valid);

    crate::diff_eq!(o, n, "142300ab0 out");
    Ok(1)
}

pub unsafe fn diff_rend_14235d320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop2(_a: u64, _b: u64) {}
    unsafe extern "win64" fn noop1(_a: u64) {}
    unsafe extern "win64" fn mock_fill(buf: usize, cnt: u64, val_ptr: usize) {
        let v = rd::<u32>(val_ptr);
        for i in 0..cnt as usize {
            wr::<u32>(buf + i * 4, v);
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4235_0200, noop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4234_ed30, noop2 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4123_e200, mock_fill as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f6_7380, noop1 as *const ())?;

    let g_cam = exe::ptr::<usize>(0x1_43a0_01d0) as usize;
    let s_cam = rd::<usize>(g_cam);

    let mut cam = [0u8; 0x100];
    let cp = cam.as_mut_ptr() as usize;
    wr::<f64>(cp + 0xb8, 1.0);
    wr::<f64>(cp + 0xc0, 2.0);
    wr::<f64>(cp + 0xc8, 3.0);
    wr::<usize>(g_cam, cp);

    let run = |f: F| -> (u32, u32, u8) {
        let mut vtbl = [0u64; 16];
        vtbl[7] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
        let mut tree = [0u8; 0x200];
        let tp = tree.as_mut_ptr() as usize;
        wr::<u32>(tp + 0x18, 0);
        wr::<u64>(tp + 0x30, 0);
        wr::<usize>(tp + 0x188, vtbl.as_ptr() as usize);

        let mut sub_entry = [0u8; 0x100];
        let sep = sub_entry.as_mut_ptr() as usize;
        wr::<u8>(sep + 0xba, 5);
        wr::<u8>(sep + 0xbc, 2);

        let mut dst_floats = [0.0f32; 4];
        let mut item = [0u8; 0x1120];
        let ip = item.as_mut_ptr() as usize;
        wr::<usize>(ip + 0x38, sep);
        wr::<usize>(ip + 0x838, dst_floats.as_mut_ptr() as usize);
        wr::<u16>(ip + 0x842, 1);
        wr::<u32>(ip + 0x848, 0);
        wr::<f32>(ip + 0xc48, 2.5);
        wr::<i32>(ip + 0x1048, 1);
        wr::<u32>(ip + 0x1100, 0);

        let mut item_ptr = ip;
        let mut stack_obj = [0u8; 0x180];
        let sp = stack_obj.as_mut_ptr() as usize;
        wr::<usize>(sp, (&mut item_ptr as *mut usize) as usize);
        wr::<u32>(sp + 0x08, 1);
        wr::<usize>(sp + 0x48, tp);

        let param_1 = sp;
        f((&param_1 as *const usize) as usize);
        (dst_floats[1].to_bits(), rd::<u32>(ip + 0x848), rd::<u8>(ip + 0x10e4))
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_cam, s_cam);

    crate::diff_eq!(o, n, "14235d320 out");
    Ok(1)
}

pub unsafe fn diff_d3d_1411d5b00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let tls_arr = exe::tls_pointer();
    let orig_tls0 = rd::<usize>(tls_arr);

    let run = |f: F| -> (u64, u64, u32) {
        let mut tls0 = vec![0u8; 0x6400];
        let tp = tls0.as_mut_ptr() as usize;
        let mut lock = [0u8; 0x20];
        let lp = lock.as_mut_ptr() as usize;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        wr::<u32>(lp + 0x08, tid);
        wr::<u32>(lp + 0x0c, 1);

        let mut ctx = [0u8; 0x40];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x18, 7);
        wr::<u64>(cp + 0x28, 0x55aa_66bb);

        wr::<u8>(tp + 0x70, 1);
        wr::<usize>(tp + 0x6300, cp);
        wr::<usize>(tp + 0x6318, lp);
        wr::<usize>(tls_arr, tp);

        let mut out_pair = [0u64; 2];
        f(out_pair.as_mut_ptr() as usize);
        (out_pair[1], rd::<u64>(cp + 0x18), rd::<u32>(lp + 0x0c))
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(tls_arr, orig_tls0);
    crate::diff_eq!(o, n, "1411d5b00 out");
    Ok(1)
}

pub unsafe fn diff_std_1412defc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> (u64, u64, u32) {
        let mut target = [0u8; 0x80];
        let tp = target.as_mut_ptr() as usize;
        let mut vec_buf = [0usize; 4];
        wr::<usize>(tp + 0x50, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(tp + 0x58, 0);
        wr::<u32>(tp + 0x5c, 4);

        let mut item = [0u8; 0x200];
        let ip = item.as_mut_ptr() as usize;
        wr::<i32>(ip + 0x80, 5);
        wr::<u8>(ip + 0xb8, 0);
        wr::<u8>(ip + 0xb9, 1);
        wr::<u8>(ip + 0xbb, 1);
        wr::<u64>(ip + 0x160, 100);
        wr::<u64>(ip + 0x168, 250);

        let items = [ip];
        let mut this_buf = [0u8; 0x180];
        let this_p = this_buf.as_mut_ptr() as usize;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        wr::<u32>(this_p + 0x148, tid);
        wr::<u32>(this_p + 0x14c, 1);
        wr::<i32>(this_p + 0x28, 1);
        wr::<i32>(this_p + 0x68, 0);
        wr::<usize>(this_p + 0xc8, items.as_ptr() as usize);

        let mut p2 = [0u8; 16];
        wr::<usize>(p2.as_mut_ptr() as usize, tp);
        f(this_p, p2.as_ptr() as usize);
        (rd::<u64>(tp + 0x40), rd::<u64>(tp + 0x48), rd::<u32>(tp + 0x58))
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412defc0 out");
    Ok(1)
}

pub unsafe fn diff_rend_1412eb690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop3(_a: usize, _b: u64, _c: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_412e_c110, noop3 as *const ())?;

    let run = |f: F| -> (u64, u32) {
        let mut word = 1u64;
        let src_vals = [0x1234_5678u32; 4];
        let mut vec_data = [0u32; 12];
        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<u64>(tp + 0x90, 0);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 4);

        f(tp);
        (word, rd::<u32>(tp + 0xc8))
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412eb690 out");
    Ok(1)
}

pub unsafe fn diff_rend_1412ed4a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> [u8; 8] {
        let src_data = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let mut dst_data = [0u8; 8];
        let mut cascade = [0u8; 0x100];
        let cp = cascade.as_mut_ptr() as usize;
        wr::<i64>(cp, 0);
        wr::<usize>(cp + 0x08, dst_data.as_mut_ptr() as usize);
        wr::<u32>(cp + 0x10, 8);
        wr::<i32>(cp + 0x18, 4);
        wr::<i32>(cp + 0x1c, 2);
        wr::<i32>(cp + 0x20, 1);

        let p1 = [0u64, 0u64];
        let p3 = [1u64, cp as u64];
        let p4 = [src_data.as_ptr() as u64, 4u64];
        let p5 = [4u64];
        f(
            p1.as_ptr() as usize,
            0,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            0,
        );
        dst_data
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412ed4a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e69330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(_p: u64, _n: u64, _w: u64) -> i32 {
        -1
    }
    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;

    let run = |f: F| -> u32 {
        let mut b00 = [0u8; 0x40];
        let mut b08 = [0u8; 0x40];
        let mut b10 = [5u32; 4];
        let mut b18 = [0u8; 0x180];
        let mut b20 = [0u8; 0x80];
        let mut b28 = [0u32; 4];
        let mut b48 = [0u8; 0x40];
        wr::<u32>(b48.as_mut_ptr() as usize + 0x10, 0);

        let tbl_30 = [0u64; 2];
        let mut p2 = vec![0u8; 0x18100];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x30, tbl_30.as_ptr() as usize);

        let p1: [usize; 11] = [
            b00.as_mut_ptr() as usize,
            b08.as_mut_ptr() as usize,
            b10.as_mut_ptr() as usize,
            b18.as_mut_ptr() as usize,
            b20.as_mut_ptr() as usize,
            b28.as_mut_ptr() as usize,
            0,
            0,
            0,
            b48.as_mut_ptr() as usize,
            0,
        ];
        f(p1.as_ptr() as usize, p2p, 0, 0);
        b10[0]
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e69330 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eec2e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop2(_a: usize, _b: u64) {}
    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, _span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 8, 0x1000);
        wr::<u64>(out + 16, 0x1000);
        out
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn mock_alloc(_sz: u64, _align: u64) -> usize {
        0x4000
    }
    let _m1 = crate::difftest::Mock::new(0x1_4028_dee0, noop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4106_f2b0, noop1 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_c190, noop1 as *const ())?;

    let g_bac = exe::ptr::<u16>(0x1_43cd_6bac) as usize;
    let g_e88 = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let s_bac = rd::<u16>(g_bac);
    let s_e88 = rd::<u16>(g_e88);
    wr::<u16>(g_bac, 0);
    wr::<u16>(g_e88, 1);

    let run = |f: F| -> (u32, u32, u64) {
        let mut out_vecs = [0u8; 0x40];
        let ovp = out_vecs.as_mut_ptr() as usize;
        wr::<u32>(ovp + 0x08, 7);
        wr::<u32>(ovp + 0x18, 9);

        let mut hash_tbl = vec![0xffu8; 0x1200];
        let htp = hash_tbl.as_mut_ptr() as usize;

        let tbl_580e0 = [ovp, htp];
        let mut p1 = vec![0u8; 0x58100];
        wr::<usize>(p1.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);
        let p2 = [0x2000u64, 0u64, 0u64];

        f(p1.as_mut_ptr() as usize, p2.as_ptr() as usize);
        (rd::<u32>(ovp + 0x08), rd::<u32>(ovp + 0x18), rd::<u64>(htp + 0x118))
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_bac, s_bac);
    wr::<u16>(g_e88, s_e88);
    crate::diff_eq!(o, n, "141eec2e0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14209b0c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_620 = exe::ptr::<u8>(0x1_4386_1620) as usize;
    let s_620 = rd::<u8>(g_620);
    wr::<u8>(g_620, 1);

    let mut this_o = [0u8; 0xa0];
    let mut this_n = [0u8; 0xa0];
    f_o(this_o.as_mut_ptr() as usize);
    f_n(this_n.as_mut_ptr() as usize);
    wr::<u8>(g_620, s_620);
    crate::diff_eq!(this_o, this_n, "14209b0c0 out");
    Ok(1)
}

pub unsafe fn diff_physics_142142870(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_scene_step(_scene: usize, a0_ptr: usize) {
        let this_ptr = a0_ptr - 0xa0;
        wr::<u8>(this_ptr + 0xc8, 1);
    }
    let _m1 = crate::difftest::Mock::new(0x1_40f6_7380, mock_scene_step as *const ())?;

    let run = |f: F| -> (u8, u32, u32) {
        let mut vtbl = [0u64; 64];
        vtbl[0x1c8 / 8] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
        let scene = [vtbl.as_ptr() as u64];
        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x20, scene.as_ptr() as usize);
        wr::<u32>(tp + 0x58, 0);
        wr::<u32>(tp + 0x68, 0);
        wr::<u8>(tp + 0xc8, 1);

        f(tp);
        (rd::<u8>(tp + 0xd8), rd::<u32>(tp + 0x58), rd::<u32>(tp + 0x68))
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142142870 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142330af0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop1(_a: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_4209_3200, noop1 as *const ())?;

    let g_cb0 = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let g_c80 = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let g_cb8 = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let g_ca8 = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let g_870 = exe::ptr::<u32>(0x1_43bb_d870) as usize;

    let s_cb0 = rd::<u32>(g_cb0);
    let s_c80 = rd::<usize>(g_c80);
    let s_cb8 = rd::<usize>(g_cb8);
    let s_ca8 = rd::<usize>(g_ca8);
    let s_870 = rd::<u32>(g_870);

    let run = |f: F| -> u32 {
        let mut mgr = [0u8; 0x300];
        let mp = mgr.as_mut_ptr() as usize;
        let mut ca8 = [0u64; 4];
        wr::<u32>(g_cb0, 24);
        wr::<usize>(g_c80, 0);
        wr::<usize>(g_cb8, mp);
        wr::<usize>(g_ca8, ca8.as_mut_ptr() as usize);
        wr::<u32>(g_870, 0);

        f();
        rd::<u32>(mp + 0x2c)
    };
    let o = run(f_o);
    let n = run(f_n);

    wr::<u32>(g_cb0, s_cb0);
    wr::<usize>(g_c80, s_c80);
    wr::<usize>(g_cb8, s_cb8);
    wr::<usize>(g_ca8, s_ca8);
    wr::<u32>(g_870, s_870);

    crate::diff_eq!(o, n, "142330af0 out");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: 0x1_422f_d680 (`rend_1422fd680` returns `f32` in `xmm0`)
//
// Replacement::verified(0x1_422f_6ff0, "rend::rend_1422f6ff0", rend::rend_e18_c::rend_1422f6ff0 as *const (), rend::rend_e18_c::diff_rend_1422f6ff0),
// Replacement::verified(0x1_422f_7040, "rend::rend_1422f7040", rend::rend_e18_c::rend_1422f7040 as *const (), rend::rend_e18_c::diff_rend_1422f7040),
// Replacement::verified(0x1_422f_70a0, "rend::rend_1422f70a0", rend::rend_e18_c::rend_1422f70a0 as *const (), rend::rend_e18_c::diff_rend_1422f70a0),
// Replacement::verified(0x1_422f_7120, "rend::rend_1422f7120", rend::rend_e18_c::rend_1422f7120 as *const (), rend::rend_e18_c::diff_rend_1422f7120),
// Replacement::verified(0x1_422f_71d0, "rend::rend_1422f71d0", rend::rend_e18_c::rend_1422f71d0 as *const (), rend::rend_e18_c::diff_rend_1422f71d0),
// Replacement::verified(0x1_422f_72a0, "rend::rend_1422f72a0", rend::rend_e18_c::rend_1422f72a0 as *const (), rend::rend_e18_c::diff_rend_1422f72a0),
// Replacement::verified(0x1_422f_7340, "rend::rend_1422f7340", rend::rend_e18_c::rend_1422f7340 as *const (), rend::rend_e18_c::diff_rend_1422f7340),
// Replacement::verified(0x1_422f_73f0, "rend::rend_1422f73f0", rend::rend_e18_c::rend_1422f73f0 as *const (), rend::rend_e18_c::diff_rend_1422f73f0),
// Replacement::verified(0x1_422f_8f70, "rend::ParticleEngine::vfunc_1", rend::rend_e18_c::rend_1422f8f70 as *const (), rend::rend_e18_c::diff_rend_1422f8f70),
// Replacement::verified(0x1_422f_9020, "rend::rend_1422f9020", rend::rend_e18_c::rend_1422f9020 as *const (), rend::rend_e18_c::diff_rend_1422f9020),
// Replacement::verified(0x1_422f_9110, "rend::rend_1422f9110", rend::rend_e18_c::rend_1422f9110 as *const (), rend::rend_e18_c::diff_rend_1422f9110),
// Replacement::verified(0x1_422f_9140, "rend::rend_1422f9140", rend::rend_e18_c::rend_1422f9140 as *const (), rend::rend_e18_c::diff_rend_1422f9140),
// Replacement::verified(0x1_422f_91e0, "rend::rend_1422f91e0", rend::rend_e18_c::rend_1422f91e0 as *const (), rend::rend_e18_c::diff_rend_1422f91e0),
// Replacement::verified(0x1_422f_9250, "rend::rend_1422f9250", rend::rend_e18_c::rend_1422f9250 as *const (), rend::rend_e18_c::diff_rend_1422f9250),
// Replacement::verified(0x1_422f_92d0, "rend::rend_1422f92d0", rend::rend_e18_c::rend_1422f92d0 as *const (), rend::rend_e18_c::diff_rend_1422f92d0),
// Replacement::verified(0x1_422f_9640, "rend::rend_1422f9640", rend::rend_e18_c::rend_1422f9640 as *const (), rend::rend_e18_c::diff_rend_1422f9640),
// Replacement::verified(0x1_422f_c110, "rend::rend_1422fc110", rend::rend_e18_c::rend_1422fc110 as *const (), rend::rend_e18_c::diff_rend_1422fc110),
// Replacement::verified(0x1_422f_c1d0, "rend::rend_1422fc1d0", rend::rend_e18_c::rend_1422fc1d0 as *const (), rend::rend_e18_c::diff_rend_1422fc1d0),
// Replacement::verified(0x1_422f_cae0, "rend::rend_1422fcae0", rend::rend_e18_c::rend_1422fcae0 as *const (), rend::rend_e18_c::diff_rend_1422fcae0),
// Replacement::verified(0x1_422f_d680, "rend::rend_1422fd680", rend::rend_e18_c::rend_1422fd680 as *const (), rend::rend_e18_c::diff_rend_1422fd680),
// Replacement::verified(0x1_422f_d730, "rend::rend_1422fd730", rend::rend_e18_c::rend_1422fd730 as *const (), rend::rend_e18_c::diff_rend_1422fd730),
// Replacement::verified(0x1_422f_d760, "rend::rend_1422fd760", rend::rend_e18_c::rend_1422fd760 as *const (), rend::rend_e18_c::diff_rend_1422fd760),
// Replacement::verified(0x1_422f_d780, "rend::rend_1422fd780", rend::rend_e18_c::rend_1422fd780 as *const (), rend::rend_e18_c::diff_rend_1422fd780),
// Replacement::verified_throws(0x1_422f_de70, "rend::rend_1422fde70", rend::rend_e18_c::rend_1422fde70 as *const (), Throws::Discard, rend::rend_e18_c::diff_rend_1422fde70),
// Replacement::verified(0x1_422f_e070, "rend::rend_1422fe070", rend::rend_e18_c::rend_1422fe070 as *const (), rend::rend_e18_c::diff_rend_1422fe070),
// Replacement::verified(0x1_422f_e0c0, "rend::rend_1422fe0c0", rend::rend_e18_c::rend_1422fe0c0 as *const (), rend::rend_e18_c::diff_rend_1422fe0c0),
// Replacement::verified(0x1_422f_e130, "rend::rend_1422fe130", rend::rend_e18_c::rend_1422fe130 as *const (), rend::rend_e18_c::diff_rend_1422fe130),
// Replacement::verified(0x1_422f_e1a0, "rend::rend_1422fe1a0", rend::rend_e18_c::rend_1422fe1a0 as *const (), rend::rend_e18_c::diff_rend_1422fe1a0),
// Replacement::verified(0x1_422f_e1e0, "rend::rend_1422fe1e0", rend::rend_e18_c::rend_1422fe1e0 as *const (), rend::rend_e18_c::diff_rend_1422fe1e0),
// Replacement::verified(0x1_422f_e270, "rend::rend_1422fe270", rend::rend_e18_c::rend_1422fe270 as *const (), rend::rend_e18_c::diff_rend_1422fe270),
// Replacement::verified(0x1_422f_e2f0, "rend::rend_1422fe2f0", rend::rend_e18_c::rend_1422fe2f0 as *const (), rend::rend_e18_c::diff_rend_1422fe2f0),
// Replacement::verified(0x1_422f_f4f0, "rend::rend_1422ff4f0", rend::rend_e18_c::rend_1422ff4f0 as *const (), rend::rend_e18_c::diff_rend_1422ff4f0),
// Replacement::verified(0x1_422f_ffa0, "rend::rend_1422fffa0", rend::rend_e18_c::rend_1422fffa0 as *const (), rend::rend_e18_c::diff_rend_1422fffa0),
// Replacement::verified(0x1_4230_0010, "rend::rend_142300010", rend::rend_e18_c::rend_142300010 as *const (), rend::rend_e18_c::diff_rend_142300010),
// Replacement::verified_throws(0x1_4230_0030, "rend::rend_142300030", rend::rend_e18_c::rend_142300030 as *const (), Throws::Caught, rend::rend_e18_c::diff_rend_142300030),
// Replacement::verified_throws(0x1_4230_0100, "rend::rend_142300100", rend::rend_e18_c::rend_142300100 as *const (), Throws::Caught, rend::rend_e18_c::diff_rend_142300100),
// Replacement::verified(0x1_4230_01e0, "rend::rend_1423001e0", rend::rend_e18_c::rend_1423001e0 as *const (), rend::rend_e18_c::diff_rend_1423001e0),
// Replacement::verified(0x1_4230_0210, "rend::rend_142300210", rend::rend_e18_c::rend_142300210 as *const (), rend::rend_e18_c::diff_rend_142300210),
// Replacement::verified(0x1_4230_0250, "rend::rend_142300250", rend::rend_e18_c::rend_142300250 as *const (), rend::rend_e18_c::diff_rend_142300250),
// Replacement::verified(0x1_4230_02b0, "rend::rend_1423002b0", rend::rend_e18_c::rend_1423002b0 as *const (), rend::rend_e18_c::diff_rend_1423002b0),

