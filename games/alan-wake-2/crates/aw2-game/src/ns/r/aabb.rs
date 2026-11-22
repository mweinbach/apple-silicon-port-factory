//! Axis-aligned bounding box helpers (`{ float3 min; float3 max; }`, 0x18 bytes).

use crate::exe::{self, rd, wr};

/// Half extents of an AABB: `out = (max - min) * 0.5`, or zero when the box is
/// inverted on all three axes (the "empty" box, min > max everywhere).
///
/// Original: 0x14024ae10 (176 bytes, leaf). Evidence from the disassembly:
/// three ordered `vcomiss min.i, max.i` + `jbe` (NaN takes the extent path),
/// `vsubss` per axis into a stack float3, then each component scaled by the
/// .rdata constant 0x143426780 (0.5f) in place in `out`. Returns `out`.
/// Called ~130k/s on the render thread (callprof cip1).
pub unsafe extern "win64" fn aabb_half_extents(aabb: usize, out: usize) -> usize {
    let min_x = rd::<f32>(aabb);
    let min_y = rd::<f32>(aabb + 0x04);
    let min_z = rd::<f32>(aabb + 0x08);
    let max_x = rd::<f32>(aabb + 0x0c);
    let max_y = rd::<f32>(aabb + 0x10);
    let max_z = rd::<f32>(aabb + 0x14);
    let (ex, ey, ez) = if min_x > max_x && min_y > max_y && min_z > max_z {
        (0.0f32, 0.0f32, 0.0f32)
    } else {
        (max_x - min_x, max_y - min_y, max_z - min_z)
    };
    let half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    wr::<f32>(out, half * ex);
    wr::<f32>(out + 0x04, half * ey);
    wr::<f32>(out + 0x08, half * ez);
    out
}

pub unsafe fn diff_aabb_half_extents(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let nan = f32::from_bits(0x7fc0_1234);
    let fixed: [[f32; 6]; 9] = [
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [-1.0, -2.0, -3.0, 4.0, 5.5, 6.25],
        // Inverted on all axes: zero.
        [1.0, 2.0, 3.0, -1.0, -2.0, -3.0],
        // Inverted on two axes only: extents (negative).
        [1.0, 2.0, -3.0, -1.0, -2.0, 3.0],
        [1.0, -2.0, 3.0, -1.0, 2.0, -3.0],
        [-1.0, 2.0, 3.0, 1.0, -2.0, -3.0],
        // Equal on one axis: not empty.
        [1.0, 2.0, 3.0, 1.0, -2.0, -3.0],
        // NaN: unordered compare takes the extent path.
        [nan, 2.0, 3.0, -1.0, -2.0, -3.0],
        [f32::MAX, f32::MAX, 3.0, -f32::MAX, -f32::MAX, 3.0],
    ];
    let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut cases = 0u32;
    for i in 0..64usize {
        let vals: [f32; 6] = if i < fixed.len() {
            fixed[i]
        } else {
            core::array::from_fn(|_| ((rnd() >> 40) as i32 - (1 << 23)) as f32 / 4096.0)
        };
        // 4-byte misaligned buffers on odd cases.
        let mis = if i % 2 == 1 { 4 } else { 0 };
        let mut box_buf = [0u8; 0x20];
        let mut out_o = [0xaau8; 0x18];
        let mut out_n = [0xaau8; 0x18];
        let b = box_buf.as_mut_ptr() as usize + mis;
        for (k, v) in vals.iter().enumerate() {
            wr::<f32>(b + k * 4, *v);
        }
        let ro = o(b, out_o.as_mut_ptr() as usize + mis);
        let rn = n(b, out_n.as_mut_ptr() as usize + mis);
        crate::diff_eq!(ro - out_o.as_ptr() as usize, rn - out_n.as_ptr() as usize, "ret case {i}");
        crate::diff_eq!(out_o, out_n, "out case {i} {vals:?}");
        cases += 1;
    }
    // Output aliasing the input box.
    let mut a_o = [0u8; 0x18];
    let mut a_n = [0u8; 0x18];
    for (k, v) in [-1.0f32, -2.0, -3.0, 4.0, 5.0, 6.0].iter().enumerate() {
        wr::<f32>(a_o.as_mut_ptr() as usize + k * 4, *v);
        wr::<f32>(a_n.as_mut_ptr() as usize + k * 4, *v);
    }
    o(a_o.as_mut_ptr() as usize, a_o.as_mut_ptr() as usize + 4);
    n(a_n.as_mut_ptr() as usize, a_n.as_mut_ptr() as usize + 4);
    crate::diff_eq!(a_o, a_n, "aliased out");
    Ok(cases + 1)
}
