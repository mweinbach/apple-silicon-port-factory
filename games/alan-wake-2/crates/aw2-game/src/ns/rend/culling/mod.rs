//! `rend::` — Renderer threading, command ring-buffer, culling, and world-render jobs.
#![allow(static_mut_refs)]

use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

use crate::exe::{self, rd, wr};

/// Plain (non-TSO) guest load. `exe::rd` lowers to `ldapr` on aarch64; the
/// per-instance cull loops only touch arrays that are private to the running
/// job, and their results are published by the release points that end the
/// job (scheduler push and the SeqCst pass-counter `fetch_sub`), so ordinary
/// loads/stores are sufficient there (same reasoning as a32b3a7).
#[inline(always)]
pub(super) unsafe fn pl<T: Copy>(a: usize) -> T {
    std::ptr::read_unaligned(a as *const T)
}

/// Plain (non-TSO) guest store; see [`pl`].
#[inline(always)]
pub(super) unsafe fn ps<T: Copy>(a: usize, v: T) {
    std::ptr::write_unaligned(a as *mut T, v)
}

// ============================================================================
// 1. Fill u32 array helper (`0x1412f8e00`)
// ============================================================================

/// Fill `count` 32-bit words at `dst` with `*val_ptr`, returning `dst + count * 4`.
///
/// Original: `0x1412f8e00` (168 bytes, `Throws::No`).
/// Fast-paths `*val_ptr == 0` via `memset(dst, 0, count * 4)` (`0x14183acc2`).
pub unsafe extern "win64" fn fill_u32_array(dst: usize, count: usize, val_ptr: usize) -> usize {
    if count == 0 {
        return dst;
    }
    let v = rd::<u32>(val_ptr);
    if v == 0 {
        let byte_len = count * 4;
        let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, byte_len) -> usize);
        return dst.wrapping_add(byte_len);
    }
    let mut p = dst;
    let mut rem = count;
    while rem >= 4 {
        let v0 = rd::<u32>(val_ptr);
        wr::<u32>(p, v0);
        let v1 = rd::<u32>(val_ptr);
        wr::<u32>(p + 4, v1);
        let v2 = rd::<u32>(val_ptr);
        wr::<u32>(p + 8, v2);
        let v3 = rd::<u32>(val_ptr);
        wr::<u32>(p + 0xc, v3);
        p = p.wrapping_add(0x10);
        rem -= 4;
    }
    while rem != 0 {
        let v0 = rd::<u32>(val_ptr);
        wr::<u32>(p, v0);
        p = p.wrapping_add(4);
        rem -= 1;
    }
    p
}

// ============================================================================
// 2. Renderer command ring-buffer (`0x1423084b0`, `0x142308440`, `0x1422a36e0`, `0x1422a38a0`)
// ============================================================================

const RING_STOP_SENTINEL: u64 = 0x0000_69fa_fafa_faff;
const RING_CONSUMED_SENTINEL: u64 = 0x0000_69ff_ffff_ffff;
const RING_WRAP_SENTINEL: u64 = 0x0000_00ff_ffff_ffff;

#[inline(always)]
unsafe fn rd_acq_u32(addr: usize) -> u32 {
    if (addr & 3) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU32)).load(Ordering::Acquire)
    } else {
        let v = core::ptr::read_unaligned(addr as *const u32);
        core::sync::atomic::fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn rd_acq_u64(addr: usize) -> u64 {
    if (addr & 7) == 0 {
        (*(addr as *const AtomicU64)).load(Ordering::Acquire)
    } else {
        let v = core::ptr::read_unaligned(addr as *const u64);
        core::sync::atomic::fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn wr_rel_u64(addr: usize, val: u64) {
    if (addr & 7) == 0 {
        (*(addr as *const AtomicU64)).store(val, Ordering::Release);
    } else {
        core::sync::atomic::fence(Ordering::Release);
        core::ptr::write_unaligned(addr as *mut u64, val);
    }
}

/// Peek the next command packet from the renderer command ring buffer `ring`.
/// Writes the packet byte size to `*out_size` and returns a pointer to the payload
/// (or `0` with `*out_size = 0` if no command is ready).
///
/// Original: `0x1423084b0` (203 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_peek(ring: usize, out_size: usize) -> usize {
    let load_u64 = |addr: usize| unsafe { rd_acq_u64(addr) };
    let load_u32 = |addr: usize| unsafe { rd_acq_u32(addr) };
    let read_ptr = load_u64(ring + 0x20) as usize;
    if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
        wr::<u64>(out_size, 0);
        return 0;
    }
    let cur_ptr = load_u64(ring + 0x20) as usize;
    if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
        wr::<u64>(out_size, 0);
        return 0;
    }
    if load_u64(cur_ptr) == RING_STOP_SENTINEL {
        wr::<u64>(out_size, 0);
        return 0;
    }
    let old_depth = rd::<u32>(ring + 0x38);
    wr::<u64>(out_size, RING_WRAP_SENTINEL);
    wr::<u32>(ring + 0x38, old_depth.wrapping_add(2));

    let end_minus_8 = rd::<usize>(ring + 0x10).wrapping_sub(8);
    if read_ptr <= end_minus_8 {
        let hdr = load_u64(read_ptr);
        wr::<u64>(out_size, hdr);
        if hdr != RING_WRAP_SENTINEL {
            wr::<u32>(ring + 0x38, old_depth);
            if hdr == RING_CONSUMED_SENTINEL {
                wr::<u64>(out_size, 0);
                return 0;
            }
            return read_ptr.wrapping_add(8);
        }
    }

    let base_ptr = rd::<usize>(ring + 0x8);
    let can_wrap = if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
        false
    } else {
        load_u64(base_ptr) != RING_STOP_SENTINEL
    };

    if can_wrap {
        let base = rd::<usize>(ring + 0x8);
        let hdr = load_u64(base);
        wr::<u32>(ring + 0x38, rd::<u32>(ring + 0x38).wrapping_sub(2));
        wr::<u64>(out_size, hdr);
        return base.wrapping_add(8);
    }

    wr::<u32>(ring + 0x38, rd::<u32>(ring + 0x38).wrapping_sub(2));
    wr::<u64>(out_size, 0);
    0
}

/// Advance the renderer command ring buffer `ring` past the current command packet.
///
/// Original: `0x142308440` (105 bytes, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_consume(ring: usize) {
    wr::<u32>(ring + 0x38, rd::<u32>(ring + 0x38).wrapping_add(2));
    let mut pkt_size: u64 = 0;
    let payload = cmd_ring_peek(ring, &mut pkt_size as *mut u64 as usize);
    if payload != 0 {
        wr_rel_u64(payload.wrapping_sub(8), RING_CONSUMED_SENTINEL);
        let aligned_sz = (pkt_size.wrapping_add(7) & !7) as usize;
        let mut next_ptr = payload.wrapping_add(aligned_sz);
        let end_ptr = rd::<usize>(ring + 0x10);
        if end_ptr.wrapping_sub(next_ptr) < 8 {
            next_ptr = rd::<usize>(ring + 0x8);
        }
        crate::simd_ops::atomic_xchg_u64((ring + 0x20) as u64, next_ptr as u64);
        crate::simd_ops::atomic_add_u32((ring + 0x34) as u64, 1);
    }
    wr::<u32>(ring + 0x38, rd::<u32>(ring + 0x38).wrapping_sub(2));
}

const CMD_DISPATCH_TABLE_VA: u64 = 0x1_43c4_9520;

/// Pop, time, and execute one renderer command from `this->ring` (`+0x168`),
/// accumulating elapsed time in `this + 0x170` (`f64`) and count in `this + 0x178` (`u32`).
///
/// Original: `0x1422a36e0` (321 bytes, `Throws::No`).
pub unsafe extern "win64" fn renderer_threading_exec_one(this: usize) {
    let ring = rd::<usize>(this + 0x168);
    let load_u32 = |addr: usize| unsafe { rd_acq_u32(addr) };
    let load_u64 = |addr: usize| unsafe { rd_acq_u64(addr) };
    if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
        return;
    }
    let read_ptr = load_u64(ring + 0x20) as usize;
    if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
        return;
    }
    if load_u64(read_ptr) == RING_STOP_SENTINEL {
        return;
    }

    let mut pkt_size: u64 = 0;
    let payload = exe::call_native!(
        0x1_4230_84b0u64,
        cmd_ring_peek,
        (rd::<usize>(this + 0x168), &mut pkt_size as *mut u64 as usize) -> usize
    );
    if payload == 0 {
        return;
    }

    let t0 = exe::func::<unsafe extern "win64" fn() -> f64>(0x1_4240_69a0)();
    let header = rd::<u32>(payload);
    let opcode = (header & 0x1ff) as usize;
    let arg_edx = (header >> 9) as u64;
    let table_base = exe::ptr::<usize>(CMD_DISPATCH_TABLE_VA) as usize;
    let handler_va = rd::<usize>(table_base + opcode * 8) as u64;
    let jp = crate::jobprof::begin();
    exe::call_va!(handler_va, (payload.wrapping_add(4), arg_edx) -> ());
    crate::jobprof::end(jp, crate::jobprof::K_MARK, handler_va, opcode as u16, 0);
    let t1 = exe::func::<unsafe extern "win64" fn() -> f64>(0x1_4240_69a0)();
    let dt = t1 - t0;

    let acc_atom = AtomicU64::from_ptr((this + 0x170) as *mut u64);
    let mut cur_bits = rd::<u64>(this + 0x170);
    loop {
        let cur_val = f64::from_bits(cur_bits);
        let new_bits = (cur_val + dt).to_bits();
        match acc_atom.compare_exchange(cur_bits, new_bits, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => break,
            Err(actual) => cur_bits = actual,
        }
    }
    AtomicI32::from_ptr((this + 0x178) as *mut i32).fetch_add(1, Ordering::SeqCst);

    let ring2 = rd::<usize>(this + 0x168);
    let mut pkt_size2: u64 = 0;
    wr::<u32>(ring2 + 0x38, rd::<u32>(ring2 + 0x38).wrapping_add(2));
    let payload2 = exe::call_native!(
        0x1_4230_84b0u64,
        cmd_ring_peek,
        (ring2, &mut pkt_size2 as *mut u64 as usize) -> usize
    );
    if payload2 != 0 {
        wr_rel_u64(payload2.wrapping_sub(8), RING_CONSUMED_SENTINEL);
        let aligned_sz = (pkt_size2.wrapping_add(7) & !7) as usize;
        let mut next_ptr = payload2.wrapping_add(aligned_sz);
        let end_ptr = rd::<usize>(ring2 + 0x10);
        if end_ptr.wrapping_sub(next_ptr) < 8 {
            next_ptr = rd::<usize>(ring2 + 0x8);
        }
        crate::simd_ops::atomic_xchg_u64((ring2 + 0x20) as u64, next_ptr as u64);
        crate::simd_ops::atomic_add_u32((ring2 + 0x34) as u64, 1);
    }
    wr::<u32>(ring2 + 0x38, rd::<u32>(ring2 + 0x38).wrapping_sub(2));
}

const IAT_WAIT_FOR_SINGLE_OBJECT: u64 = 0x1_4264_5748;
const THREAD_MGR_PTR_VA: u64 = 0x1_43a0_82f8;
const MEM_MGR_PTR_VA: u64 = 0x1_43a0_82f0;
const IDLE_THRESHOLD_SEC_VA: u64 = 0x1_4342_68f0;

/// `rend::RendererThreading::vfunc_1` — main render thread command loop.
///
/// Original: `0x1422a38a0` (530 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn renderer_threading_main(this: usize) -> u32 {
    let mgr_holder = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    let mgr = rd::<usize>(mgr_holder + 0x8);
    exe::call_va!(0x1_4240_6a80, (mgr, 2u32, u32::MAX) -> ());

    let qpc_now = crate::ns::r::thread::qpc_iat();
    let _freq = exe::call_va!(0x1_4240_6920, () -> i64);
    let mut baseline_qpc: i64 = qpc_now;

    if rd::<u8>(this + 0x160) != 0 {
        return 0;
    }
    let idle_threshold = rd::<f64>(exe::ptr::<f64>(IDLE_THRESHOLD_SEC_VA) as usize);

    loop {
        if rd::<u8>(this + 0x198) != 0 {
            let sem = rd::<usize>(this + 0x1a0);
            let mut tries: i32 = 1;
            loop {
                let prev = tries;
                tries -= 1;
                if prev <= 0 {
                    break;
                }
                let wait_res = exe::call_iat!(IAT_WAIT_FOR_SINGLE_OBJECT, (sem, 10u32) -> u32);
                if wait_res == u32::MAX {
                    break;
                }
            }
        }

        let mut mode = rd::<u32>(this + 0x17c);
        if mode != rd::<u32>(this + 0x180) {
            if mode == 1 {
                exe::call_va!(0x1_401e_65c0, (this + 0x140) -> ());
            }
            mode = rd::<u32>(this + 0x17c);
            wr::<u32>(this + 0x180, mode);
        }

        if mode != 1 {
            // The idle poll: 32% of all lifted-call dispatches in gameplay (cvP,
            // `AW2_CALLVA_PROF`), ~96% of them finding the ring empty. Direct call while
            // the VA still dispatches to the native function (difftest mocks fall back).
            exe::call_native!(0x1_422a_36e0u64, renderer_threading_exec_one, (this) -> ());
            let qpc_after = crate::ns::r::thread::qpc_iat();
            let delta = (qpc_after.wrapping_sub(baseline_qpc)) as f64;
            let freq = exe::call_native!(
                0x1_4240_6920u64,
                crate::ns::r::thread::perf_frequency,
                () -> i64
            ) as f64;
            if (delta / freq) >= idle_threshold {
                let mem_mgr = rd::<usize>(exe::ptr::<usize>(MEM_MGR_PTR_VA) as usize);
                if mem_mgr != 0 {
                    let w = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                    let tls_alloc = rd::<usize>(mem_mgr + 0x2d8 + w * 8);
                    exe::call_va!(0x1_4247_5db0, (tls_alloc, u32::MAX) -> ());
                }
                let mut cb = rd::<usize>(this + 0x120);
                let cb_end = cb.wrapping_add((rd::<u32>(this + 0x128) as usize) << 4);
                while cb != cb_end {
                    let fn_va = rd::<usize>(cb + 8) as u64;
                    let arg = rd::<usize>(cb);
                    exe::call_va!(fn_va, (arg) -> ());
                    cb = cb.wrapping_add(0x10);
                }
                let qpc_reset = crate::ns::r::thread::qpc_iat();
                let _freq2 = exe::call_va!(0x1_4240_6920, () -> i64);
                baseline_qpc = qpc_reset;
            }
        }

        if rd::<u8>(this + 0x160) != 0 {
            break;
        }
    }
    0
}

// ============================================================================
// 3. Bounding-volume / LOD / Culling helpers (`0x14225b620`, `0x1422599d0`,
//    `0x142259950`, `0x142259830`, `0x1422599f0`, `0x142259370`)
// ============================================================================

#[inline(always)]
fn dot3_f32(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Plain (non-TSO) 16-byte load: the culling helpers read mesh/bone/transform
/// data that the job scheduler publishes to the job before it runs.
#[inline(always)]
unsafe fn read_f32x4(addr: usize) -> [f32; 4] {
    std::ptr::read_unaligned(addr as *const [f32; 4])
}

#[inline(always)]
unsafe fn write_f32x4(addr: usize, v: [f32; 4]) {
    wr::<f32>(addr, v[0]);
    wr::<f32>(addr + 4, v[1]);
    wr::<f32>(addr + 8, v[2]);
    wr::<f32>(addr + 12, v[3]);
}

/// x86 `vminps` / `vminss` semantics: `if a < b { a } else { b }` (second operand on NaN/equal).
#[inline(always)]
fn x86_min_f32(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// x86 `vmaxps` / `vmaxss` semantics: `if a > b { a } else { b }` (second operand on NaN/equal).
#[inline(always)]
fn x86_max_f32(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

const AABB_INIT_MIN_VA: u64 = 0x1_4342_86e0;
const AABB_INIT_MAX_VA: u64 = 0x1_4342_89c0;
const HALF_VEC_VA: u64 = 0x1_4342_8120;
const MAT_ROW3_DEFAULT_VA: u64 = 0x1_4342_8240;

/// Compute transformed world-space AABB (`out_aabb`: 32 bytes `[min.xyzw, max.xyzw]`)
/// and bounding sphere (`out_sphere`: 16 bytes `[center.xyz, radius]`) for a mesh/skeleton
/// instance `mesh` (`r8`) with world transform `world_mat` (`r9`, 4x4f) and optional
/// bone 3x4 matrices `bone_mats` (`stack + 0x28`).
///
/// Original: `0x14225b620` (635 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn compute_transformed_bounds(
    out_aabb: usize,
    out_sphere: usize,
    mesh: usize,
    world_mat: usize,
    bone_mats: usize,
) {
    let w_row0 = read_f32x4(world_mat);
    let w_row1 = read_f32x4(world_mat + 0x10);
    let w_row2 = read_f32x4(world_mat + 0x20);
    let w_row3 = read_f32x4(world_mat + 0x30);

    let d1 = dot3_f32(w_row1, w_row1);
    let d2 = dot3_f32(w_row2, w_row2);
    let d0 = dot3_f32(w_row0, w_row0);
    let max_scale_sq = x86_max_f32(x86_max_f32(d1, d2), d0);
    let max_scale = max_scale_sq.sqrt();

    let mut vmin = read_f32x4(exe::ptr::<f32>(AABB_INIT_MIN_VA) as usize);
    let mut vmax = read_f32x4(exe::ptr::<f32>(AABB_INIT_MAX_VA) as usize);

    let elem_offset = (rd::<u32>(mesh + 0x1e8) as usize) << 4;
    let mut elem_ptr = rd::<usize>(mesh + 0x1a0).wrapping_add(elem_offset);
    let count = rd::<u32>(mesh + 0x1c8) as usize;

    if bone_mats != 0 {
        if count != 0 {
            let def_row3 = rd::<f32>(exe::ptr::<f32>(MAT_ROW3_DEFAULT_VA) as usize);
            let mut b_ptr = bone_mats.wrapping_add(0x20);
            let mut idx: i32 = 0;
            while (idx as usize) < count {
                let s = read_f32x4(elem_ptr);
                let radius = s[3];
                if !(0.0 >= radius) {
                    let r0 = read_f32x4(b_ptr - 0x20);
                    let r1 = read_f32x4(b_ptr - 0x10);
                    let r2 = read_f32x4(b_ptr);

                    // 4x3 packed float3 columns (12 floats = 48 bytes per bone):
                    // col0 (xmm6) = [r0[0], r0[1], r0[2], 0.0]
                    // col1 (xmm4) = [r0[3], r1[0], r1[1], 0.0]
                    // col2 (xmm3) = [r1[2], r1[3], r2[0], 0.0]
                    // col3 (xmm5) = [r2[1], r2[2], r2[3], def_row3]
                    let col0 = [r0[0], r0[1], r0[2], 0.0];
                    let col1 = [r0[3], r1[0], r1[1], 0.0];
                    let col2 = [r1[2], r1[3], r2[0], 0.0];
                    let mut p = [r2[1], r2[2], r2[3], def_row3];

                    for k in 0..4 {
                        p[k] = s[2].mul_add(col2[k], p[k]);
                        p[k] = s[1].mul_add(col1[k], p[k]);
                        p[k] = s[0].mul_add(col0[k], p[k]);
                    }

                    let mut wp = [0.0f32; 4];
                    for k in 0..4 {
                        let mut acc = w_row2[k].mul_add(p[2], w_row3[k]);
                        acc = p[1].mul_add(w_row1[k], acc);
                        acc = p[0].mul_add(w_row0[k], acc);
                        wp[k] = acc;
                    }

                    let scaled_r = radius * max_scale;
                    for k in 0..4 {
                        let lo = wp[k] - scaled_r;
                        let hi = wp[k] + scaled_r;
                        vmax[k] = x86_max_f32(vmax[k], hi);
                        vmin[k] = x86_min_f32(vmin[k], lo);
                    }
                }
                idx += 1;
                elem_ptr = elem_ptr.wrapping_add(0x10);
                b_ptr = b_ptr.wrapping_add(0x30);
            }
        }
    } else if count != 0 {
        let mut idx: i32 = 0;
        while (idx as usize) < count {
            let s = read_f32x4(elem_ptr);
            let radius = s[3];
            if !(0.0 >= radius) {
                let mut wp = [0.0f32; 4];
                for k in 0..4 {
                    let mut acc = w_row2[k].mul_add(s[2], w_row3[k]);
                    acc = s[1].mul_add(w_row1[k], acc);
                    acc = s[0].mul_add(w_row0[k], acc);
                    wp[k] = acc;
                }
                let scaled_r = radius * max_scale;
                for k in 0..4 {
                    let lo = wp[k] - scaled_r;
                    let hi = wp[k] + scaled_r;
                    vmax[k] = x86_max_f32(vmax[k], hi);
                    vmin[k] = x86_min_f32(vmin[k], lo);
                }
            }
            idx += 1;
            elem_ptr = elem_ptr.wrapping_add(0x10);
        }
    }

    let half = read_f32x4(exe::ptr::<f32>(HALF_VEC_VA) as usize);
    let mut extent = [0.0f32; 4];
    for k in 0..4 {
        extent[k] = (vmax[k] - vmin[k]) * half[k];
    }

    write_f32x4(out_aabb, vmin);
    write_f32x4(out_aabb + 0x10, vmax);

    let radius_sq = dot3_f32(extent, extent);
    let sphere_r = radius_sq.sqrt();
    let sphere = [
        vmin[0] + extent[0],
        vmin[1] + extent[1],
        vmin[2] + extent[2],
        sphere_r,
    ];
    write_f32x4(out_sphere, sphere);
}

/// Find first index in `thresholds[0..8]` where `val < thresholds[i]`, or `32` if none.
///
/// Original: `0x1422599d0` (26 bytes, leaf, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn lod_find_threshold_avx(thresholds: usize, val: f32) -> u32 {
    lod_find_threshold_avx_impl(thresholds, val)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn lod_find_threshold_avx(
    thresholds: usize,
    _xmm0: f32,
    val: f32,
) -> u32 {
    lod_find_threshold_avx_impl(thresholds, val)
}

#[inline(always)]
unsafe fn lod_find_threshold_avx_impl(thresholds: usize, val: f32) -> u32 {
    let mut mask: u32 = 0;
    for i in 0..8 {
        let t = rd::<f32>(thresholds + i * 4);
        if val < t {
            mask |= 1 << i;
        }
    }
    mask.trailing_zeros()
}

/// Hysteresis LOD level selection across `thresholds` (`rcx`), given distance `dist` (`xmm1`),
/// hysteresis margins `hyst_down` (`xmm2`) and `hyst_up` (`xmm3`), `min_lod` (`stack+0x28`),
/// `max_lod` (`stack+0x30`), and `cur_lod` (`stack+0x38`).
///
/// Original: `0x142259950` (116 bytes, leaf, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn lod_select_with_hysteresis(
    thresholds: usize,
    dist: f32,
    hyst_down: f32,
    hyst_up: f32,
    min_lod: i32,
    max_lod: i32,
    cur_lod: i32,
) -> i32 {
    lod_select_with_hysteresis_impl(thresholds, dist, hyst_down, hyst_up, min_lod, max_lod, cur_lod)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn lod_select_with_hysteresis(
    thresholds: usize,
    _rdx: usize,
    _r8: usize,
    _r9: usize,
    _xmm0: f32,
    dist: f32,
    hyst_down: f32,
    hyst_up: f32,
    min_lod: i32,
    max_lod: i32,
    cur_lod: i32,
) -> i32 {
    lod_select_with_hysteresis_impl(thresholds, dist, hyst_down, hyst_up, min_lod, max_lod, cur_lod)
}

#[inline(always)]
unsafe fn lod_select_with_hysteresis_impl(
    thresholds: usize,
    dist: f32,
    hyst_down: f32,
    hyst_up: f32,
    min_lod: i32,
    max_lod: i32,
    cur_lod: i32,
) -> i32 {
    let limit0 = if max_lod < cur_lod { max_lod } else { cur_lod };
    let adj_down = dist - hyst_down;
    let mut i = min_lod as isize;
    let mut i_i32 = min_lod;
    let limit0_isize = limit0 as isize;
    while i < limit0_isize {
        let t = rd::<f32>(thresholds + (i as usize) * 4);
        if t > adj_down {
            return i_i32;
        }
        i_i32 = i_i32.wrapping_add(1);
        i += 1;
    }

    let adj_up = dist - hyst_up;
    let mut j = cur_lod;
    if cur_lod < max_lod {
        let mut p = thresholds.wrapping_add((cur_lod as isize as usize).wrapping_mul(4));
        while j < max_lod {
            let t = rd::<f32>(p);
            if t > adj_up {
                return j;
            }
            j = j.wrapping_add(1);
            p = p.wrapping_add(4);
        }
    }
    max_lod
}

const CULL_MGR_PTR_VA: u64 = 0x1_4397_ff48;
const HYST_DOWN_VA: u64 = 0x1_4386_c2b0;
const HYST_UP_VA: u64 = 0x1_4386_c2d0;

/// Update LOD indices for all instances in the global culling manager (`0x14397ff48`).
///
/// Original: `0x142259830` (275 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn update_instance_lod_levels() {
    let mgr = rd::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);
    let hyst_down = rd::<f32>(exe::ptr::<f32>(HYST_DOWN_VA) as usize);
    let hyst_up = rd::<f32>(exe::ptr::<f32>(HYST_UP_VA) as usize);
    let count = rd::<u32>(mgr + 0x248) as usize;
    if count == 0 {
        return;
    }

    let mut desc_ptr = rd::<usize>(mgr + 0x240).wrapping_add(1);
    let mut dist_ptr = rd::<usize>(mgr + 0x260);
    let clamp_arr = rd::<usize>(mgr + 0x280);
    let mut out_ptr = rd::<usize>(mgr + 0x2a0);
    let clamp_delta = clamp_arr.wrapping_sub(out_ptr);

    for _ in 0..count {
        let cur_lod = rd::<u8>(desc_ptr + 1) as u32;
        let max_lod = rd::<u8>(desc_ptr) as u32;
        let min_lod = rd::<u8>(desc_ptr - 1) as u32;
        let limit0 = if max_lod < cur_lod { max_lod } else { cur_lod };
        let dist = rd::<f32>(dist_ptr);

        let mut chosen: u32 = min_lod;
        let adj_down = dist - hyst_down;
        let mut found = false;
        if (min_lod as usize) < (limit0 as usize) {
            let mut idx = min_lod as usize;
            while idx < (limit0 as usize) {
                let t = rd::<f32>(desc_ptr + 3 + idx * 4);
                if t > adj_down {
                    found = true;
                    break;
                }
                chosen = chosen.wrapping_add(1);
                idx += 1;
            }
        }

        if !found {
            chosen = cur_lod;
            let adj_up = dist - hyst_up;
            if cur_lod < max_lod {
                let mut p = desc_ptr.wrapping_add(3 + (cur_lod as usize) * 4);
                while chosen < max_lod {
                    let t = rd::<f32>(p);
                    if t > adj_up {
                        found = true;
                        break;
                    }
                    chosen = chosen.wrapping_add(1);
                    p = p.wrapping_add(4);
                }
            }
            if !found {
                chosen = max_lod;
            }
        }

        let cap = rd::<u8>(out_ptr.wrapping_add(clamp_delta));
        let chosen_u8 = chosen as u8;
        let final_lod = if cap < chosen_u8 { cap } else { chosen_u8 };
        wr::<u8>(out_ptr, final_lod);

        desc_ptr = desc_ptr.wrapping_add(0x24);
        dist_ptr = dist_ptr.wrapping_add(4);
        out_ptr = out_ptr.wrapping_add(1);
    }
}

const DEFAULT_MIN_DIST_VA: u64 = 0x1_4342_6f98;

/// Aggregate per-submesh visibility/LOD min-max distances and bitmasks into parent instances.
///
/// Original: `0x1422599f0` (924 bytes, `Throws::No`).
pub unsafe extern "win64" fn aggregate_instance_cull_results() {
    let mgr = rd::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);
    let init_min_dist = rd::<f32>(exe::ptr::<f32>(DEFAULT_MIN_DIST_VA) as usize);

    let sub_min_arr = rd::<usize>(mgr + 0x1c8);
    let sub_max_arr = rd::<usize>(mgr + 0x1d8);
    let sub_lod_arr = rd::<usize>(mgr + 0x208);
    let sub_bit_arr = rd::<usize>(mgr + 0x218);
    let sub_inst_arr = rd::<usize>(mgr + 0x198);
    let sub_count = rd::<u32>(mgr + 0x1a0) as usize;

    let inst_min_arr = rd::<usize>(mgr + 0x260);
    let inst_count = rd::<u32>(mgr + 0x268) as usize;
    let inst_max_arr = rd::<usize>(mgr + 0x270);
    let inst_lod_arr = rd::<usize>(mgr + 0x280);
    let inst_mask_arr = rd::<usize>(mgr + 0x2c0);

    let mut fill_min = init_min_dist;
    let _ = exe::call_va!(
        0x1_412f_8e00,
        (inst_min_arr, inst_count, &mut fill_min as *mut f32 as usize) -> usize
    );
    let mut fill_zero: f32 = 0.0;
    let _ = exe::call_va!(
        0x1_412f_8e00,
        (inst_max_arr, inst_count, &mut fill_zero as *mut f32 as usize) -> usize
    );

    if inst_count != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (inst_mask_arr, 0u32, inst_count * 4) -> usize);
        let _ = exe::call_va!(0x1_4183_acc2, (inst_lod_arr, 0xffu32, inst_count) -> usize);
    }

    for i in 0..sub_count {
        let inst_idx = rd::<u32>(sub_inst_arr + i * 4);
        if inst_idx != u32::MAX {
            let idx = inst_idx as usize;
            let cur_min = rd::<f32>(inst_min_arr + idx * 4);
            let sub_min = rd::<f32>(sub_min_arr + i * 4);
            wr::<f32>(inst_min_arr + idx * 4, x86_min_f32(cur_min, sub_min));

            let cur_max = rd::<f32>(inst_max_arr + idx * 4);
            let sub_max = rd::<f32>(sub_max_arr + i * 4);
            wr::<f32>(inst_max_arr + idx * 4, x86_max_f32(cur_max, sub_max));

            let sub_lod = rd::<u8>(sub_lod_arr + 1 + i * 2);
            let cur_lod = rd::<u8>(inst_lod_arr + idx);
            let bit_idx = rd::<u8>(sub_bit_arr + i) as u32;
            let bit_mask = 1u32.wrapping_shl(bit_idx & 31);
            let cur_mask = rd::<u32>(inst_mask_arr + idx * 4);
            wr::<u32>(inst_mask_arr + idx * 4, cur_mask | bit_mask);

            let new_lod = if cur_lod < sub_lod { cur_lod } else { sub_lod };
            wr::<u8>(inst_lod_arr + idx, new_lod);
        }
    }
}

const CULL_DISABLED_FLAG_VA: u64 = 0x1_4397_d8a7;

/// Aggregate submesh cull results (`0x1422599f0`) and update instance LOD levels
/// (`0x142259830` inline logic) when `this->count` (`+0x20`) is non-zero and culling is active.
///
/// Original: `0x1422626d0` (312 bytes, `Throws::No`).
pub unsafe extern "win64" fn update_cull_and_instance_lods(this: usize) {
    if rd::<u8>(exe::ptr::<u8>(CULL_DISABLED_FLAG_VA) as usize) != 0 {
        return;
    }
    if rd::<u32>(this + 0x20) == 0 {
        return;
    }
    exe::call_va!(0x1_4225_99f0, () -> ());
    update_instance_lod_levels();
}

const CAM_STATE_PTR_VA: u64 = 0x1_43a0_01d0;
const EPSILON_BASE_VA: u64 = 0x1_4386_9810;
const SIGN_FLIP_MASK_VA: u64 = 0x1_4342_8760;

/// Evaluate per-submesh LOD selection, view-cone test, occlusion/distance culling,
/// and dirty bitmask updates across `count` (`rdx`) items.
///
/// Original: `0x142259370` (1209 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn evaluate_submesh_lod_and_culling(_unused_rcx: usize, count: usize) {
    let mgr = pl::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);

    let cam_ptr = pl::<usize>(exe::ptr::<usize>(CAM_STATE_PTR_VA) as usize);
    let cam_dir = if cam_ptr != 0 {
        [
            pl::<f64>(cam_ptr + 0xa0) as f32,
            pl::<f64>(cam_ptr + 0xa8) as f32,
            pl::<f64>(cam_ptr + 0xb0) as f32,
            0.0f32,
        ]
    } else {
        [0.0f32; 4]
    };

    let cam_pos = if cam_ptr != 0 {
        [
            pl::<f64>(cam_ptr + 0xb8) as f32,
            pl::<f64>(cam_ptr + 0xc0) as f32,
            pl::<f64>(cam_ptr + 0xc8) as f32,
            0.0f32,
        ]
    } else {
        [0.0f32; 4]
    };

    let eps_bits = pl::<u32>(exe::ptr::<u32>(EPSILON_BASE_VA) as usize);
    let flip_bits = pl::<u32>(exe::ptr::<u32>(SIGN_FLIP_MASK_VA) as usize);

    // The original re-reads the manager pointer before phase 2; phase 1 writes only the
    // per-submesh LOD bytes, so it is the same manager.
    let c = SubmeshCullCtx {
        desc_arr: pl::<usize>(mgr + 0x240),
        clamp_pairs: pl::<usize>(mgr + 0x208),
        inst_idx_arr: pl::<usize>(mgr + 0x198),
        dist_arr: pl::<usize>(mgr + 0x1c8),
        out_lod_arr: pl::<usize>(mgr + 0x1a8),
        sphere_arr: pl::<usize>(mgr + 0xe8),
        node_stream: pl::<usize>(mgr + 0x28),
        dist_a_arr: pl::<usize>(mgr + 0x1e8),
        dist_b_arr: pl::<usize>(mgr + 0x1f8),
        prev_lod_arr: pl::<usize>(mgr + 0x1b8),
        dirty_mask_arr: pl::<usize>(mgr + 0x180),
        active_mask_arr: pl::<usize>(mgr),
        cam_dir,
        cam_pos,
        neg_eps: f32::from_bits(eps_bits ^ flip_bits),
        count,
    };
    if count >= worker::LOD_PAR_MIN && par::lanes() >= 2 {
        let chunks = count.div_ceil(worker::LOD_CHUNK);
        par::for_chunks(chunks, &c as *const SubmeshCullCtx as *const (), submesh_cull_chunk);
        return;
    }
    submesh_cull_range(&c, 0, count);
}

struct SubmeshCullCtx {
    desc_arr: usize,
    clamp_pairs: usize,
    inst_idx_arr: usize,
    dist_arr: usize,
    out_lod_arr: usize,
    sphere_arr: usize,
    node_stream: usize,
    dist_a_arr: usize,
    dist_b_arr: usize,
    prev_lod_arr: usize,
    dirty_mask_arr: usize,
    active_mask_arr: usize,
    cam_dir: [f32; 4],
    cam_pos: [f32; 4],
    neg_eps: f32,
    count: usize,
}

unsafe fn submesh_cull_chunk(ctx: *const (), chunk: usize) {
    let c = &*(ctx as *const SubmeshCullCtx);
    let s = chunk * worker::LOD_CHUNK;
    submesh_cull_range(c, s, (s + worker::LOD_CHUNK).min(c.count));
}

/// Phases 1-3 of [`evaluate_submesh_lod_and_culling`] for submeshes `s..e`. `s` is a multiple of
/// 64 (and `e` too unless it is `count`): element `i` writes only `out_lod[i]` and node byte
/// `i*12+3`, the 32-wide blocks write only their own dirty u32, and the scalar tail (`count &
/// !31 .. count`) read-modify-writes whole u64 dirty words, which all lie in the last range
/// together with the one block u32 the tail's first word can share; so ranges are independent
/// and each range keeps the original phase order.
#[inline(always)]
unsafe fn submesh_cull_range(c: &SubmeshCullCtx, s: usize, e: usize) {
    let (cam_dir, cam_pos, neg_eps) = (c.cam_dir, c.cam_pos, c.neg_eps);
    let out_lod_arr = c.out_lod_arr;
    let count = c.count;

    // Phase 1: Per-submesh LOD selection + camera plane test
    for i in s..e {
        let inst_idx = pl::<u32>(c.inst_idx_arr + i * 4);
        if inst_idx != u32::MAX {
            let d = pl::<f32>(c.dist_arr + i * 4);
            let sphere = read_f32x4(c.sphere_arr + i * 0x10);
            let desc = c.desc_arr.wrapping_add((inst_idx as usize) * 0x24);

            let mut mask: u32 = 0;
            for bit in 0..8 {
                let thresh = pl::<f32>(desc + 4 + bit * 4);
                if d < thresh {
                    mask |= 1 << bit;
                }
            }
            let tz = (mask.trailing_zeros() & 0xff) as u8;

            let mut min_lod = pl::<u8>(desc + 1);
            let mut max_lod = pl::<u8>(desc + 2);
            let pair = pl::<u16>(c.clamp_pairs + i * 2);
            let pair_lo = (pair & 0xff) as u8;
            let pair_hi = (pair >> 8) as u8;

            if pair_hi < min_lod {
                min_lod = pair_hi;
            }
            if pair_lo > max_lod {
                max_lod = pair_lo;
            }

            let radius = sphere[3];
            let p_minus_cam = [
                sphere[0].mul_add(1.0, cam_dir[0] * radius) - cam_pos[0],
                sphere[1].mul_add(1.0, cam_dir[1] * radius) - cam_pos[1],
                sphere[2].mul_add(1.0, cam_dir[2] * radius) - cam_pos[2],
                0.0,
            ];
            let dot = dot3_f32(p_minus_cam, cam_dir);

            let mut cand = tz;
            if dot < neg_eps {
                cand = min_lod.wrapping_sub(1);
            }
            let mut chosen = min_lod;
            if cand < min_lod {
                chosen = cand;
            }
            let mut final_lod = max_lod;
            if chosen > max_lod {
                final_lod = chosen;
            }
            ps::<u8>(out_lod_arr + i, final_lod);
        }
    }

    // Phase 2: Compare `cull_dist_a[i]` (`+0x1e8`) vs `cull_dist_b[i]` (`+0x1f8`);
    // 8-wide AVX loop keeps `lod` only where `a > b` (else `0xff`), and scalar tail sets `0xff` when `b > a`.
    let vec8_end = (count & !7).clamp(s, e);
    for i in s..vec8_end {
        let a = pl::<f32>(c.dist_a_arr + i * 4);
        let b = pl::<f32>(c.dist_b_arr + i * 4);
        let mut lod = pl::<u8>(out_lod_arr + i);
        if !(a > b) {
            lod = 0xff;
        }
        ps::<u8>(out_lod_arr + i, lod);
        ps::<u8>(c.node_stream + i * 12 + 3, lod);
    }
    for i in vec8_end..e {
        let a = pl::<f32>(c.dist_a_arr + i * 4);
        let b = pl::<f32>(c.dist_b_arr + i * 4);
        let mut lod = pl::<u8>(out_lod_arr + i);
        if b > a {
            lod = 0xff;
        }
        ps::<u8>(out_lod_arr + i, lod);
        ps::<u8>(c.node_stream + i * 12 + 3, lod);
    }

    // Phase 3: Compare `out_lod_arr[i]` against `prev_lod_arr[i]` (`+0x1b8`),
    // masking with `active_mask_words` (`*(mgr2 + 0x0)`) and writing `dirty_mask_words` (`+0x180`).
    let (prev_lod_arr, dirty_mask_arr, active_mask_arr) = (c.prev_lod_arr, c.dirty_mask_arr, c.active_mask_arr);
    let full_end = count & !0x1f;
    let vec_end = full_end.clamp(s, e);
    let mut base = s;
    while base < vec_end {
        let mut eq_mask: u32 = 0;
        for bit in 0..32 {
            let a = pl::<u8>(prev_lod_arr + base + bit);
            let b = pl::<u8>(out_lod_arr + base + bit);
            if a == b {
                eq_mask |= 1u32 << bit;
            }
        }
        let word_idx = base >> 5;
        let active = pl::<u32>(active_mask_arr + word_idx * 4);
        let dirty = (!eq_mask) & active;
        ps::<u32>(dirty_mask_arr + word_idx * 4, dirty);
        base += 32;
    }

    if e != count {
        return;
    }
    for i in full_end..count {
        let word32 = pl::<u32>(active_mask_arr + (i >> 5) * 4);
        let is_active = ((word32 >> (i & 31)) & 1) != 0;
        let is_diff = if is_active {
            pl::<u8>(out_lod_arr + i) != pl::<u8>(prev_lod_arr + i)
        } else {
            false
        };
        let qword_idx = i >> 6;
        let bit64 = (i & 63) as u32;
        let cur_qword = pl::<u64>(dirty_mask_arr + qword_idx * 8);
        let cleared = cur_qword & !(1u64 << bit64);
        let updated = if is_diff {
            cleared | (1u64 << bit64)
        } else {
            cleared
        };
        ps::<u64>(dirty_mask_arr + qword_idx * 8, updated);
    }
}


// ============================================================================
// 5. Additional renderer command ring, culling, and instance-management helpers
// ============================================================================

/// Update submesh visibility masks (`0x142330950`) across all non-null instances in `this`.
///
/// Original: `0x142259300` (103 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn update_all_instance_vis_masks(this: usize) {
    let count = rd::<u32>(this + 0x298) as usize;
    let masks = rd::<usize>(this + 0x2c0);
    for i in 0..count {
        let obj_arr = rd::<usize>(this + 0x290);
        let obj = rd::<usize>(obj_arr + i * 8);
        if obj != 0 {
            let mask = rd::<u32>(masks + i * 4);
            exe::call_native!(0x1_4233_0950, update_render_instance_submesh_mask, (obj, mask) -> ());
        }
    }
}

const VIS_POST_JOB_VTBL_VA: u64 = 0x1_433f_3360;
const VIS_POST_JOB_HANDLE_VA: u64 = 0x1_4397_ff38;

/// Render-world job body: update submesh visibility masks (`0x142330950`) for every non-null
/// instance in `this` (same loop as [`update_all_instance_vis_masks`]), then allocate the next
/// job (vtable `0x1433f3360`) from the current worker's pool, publish its handle to
/// `0x14397ff38` and push it to the scheduler (waking a worker) if it is still pending.
///
/// The original copies one byte from its caller's `rcx` home slot (`[rsp+0x40]`) into the
/// job's empty-lambda storage (`job + 0x18`); that byte is never read, so 0 is stored here.
///
/// Original: `0x1422591f0` (254 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn update_instance_vis_masks_post_job(this: usize) {
    let count = rd::<u32>(this + 0x298) as usize;
    let masks = rd::<usize>(this + 0x2c0);
    for i in 0..count {
        let obj = rd::<usize>(rd::<usize>(this + 0x290) + i * 8);
        if obj != 0 {
            let mask = rd::<u32>(masks + i * 4);
            exe::call_native!(0x1_4233_0950, update_render_instance_submesh_mask, (obj, mask) -> ());
        }
    }

    let holder = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    let sched = rd::<usize>(holder + 0x10);
    let w = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool = rd::<usize>(sched + 0x48 + (w << 6)).wrapping_add(0x40);
    let mut handle = [0u64; 2];
    let _ = exe::call_va!(0x1_4248_5aa0, (pool, handle.as_mut_ptr() as usize) -> usize);
    let job = handle[0] as usize;
    AtomicI32::from_ptr(job as *mut i32).swap(1, Ordering::SeqCst);
    wr::<u64>(job + 0x8, 0);
    wr::<usize>(job + 0x10, exe::ptr::<u8>(VIS_POST_JOB_VTBL_VA) as usize);
    wr::<u8>(job + 0x18, 0);
    let g_handle = exe::ptr::<u64>(VIS_POST_JOB_HANDLE_VA) as usize;
    wr::<u64>(g_handle, handle[0]);
    wr::<u64>(g_handle + 8, handle[1]);

    if rd::<u32>(job) == 1 {
        let sched2 = rd::<usize>(holder + 0x10);
        let job_arg = rd::<usize>(g_handle);
        exe::call_va!(0x1_4248_6460, (sched2, job_arg) -> ());
        let sem = rd::<usize>(rd::<usize>(sched2) + 0x1008);
        let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1u32, 0usize) -> i32);
    }
}

/// Run `0x142259da0(0, count)` and `0x142259370(0, count)` when culling is enabled
/// (`*0x14397d8a7 == 0`) and `this->count` (`+0x20`) is non-zero.
///
/// Original: `0x142262810` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn evaluate_culling_for_manager(this: usize) {
    if rd::<u8>(exe::ptr::<u8>(CULL_DISABLED_FLAG_VA) as usize) != 0 {
        return;
    }
    let count = rd::<u32>(this + 0x20) as usize;
    if count != 0 {
        exe::call_va!(0x1_4225_9da0, (0usize, count) -> ());
        exe::call_va!(0x1_4225_9370, (0usize, count) -> ());
    }
}

const LOD_SCALE_VA: u64 = 0x1_4386_c680;
const LOD_MODE_VA: u64 = 0x1_4386_c660;
const LOD_FOV_FACTOR_VA: u64 = 0x1_4384_170c;
const CACHED_LOD_MODE_VA: u64 = 0x1_4387_9600;
const CACHED_LOD_SCALE_VA: u64 = 0x1_4387_9604;
const CACHED_LOD_FOV_VA: u64 = 0x1_4387_9608;
const ABS_MASK_VA: u64 = 0x1_4342_8720;
const LOD_EPS_VA: u64 = 0x1_4342_6460;
const FORCE_REBUILD_A_VA: u64 = 0x1_4386_9848;
const FORCE_REBUILD_B_VA: u64 = 0x1_4386_c648;

/// Recompute per-instance LOD distance thresholds (`0x142330d80`) either incrementally
/// from `this->dirty_words` (`+0x228`, bit count `+0x238`) or across all `this->inst_count` (`+0x248`)
/// when global LOD parameters change.
///
/// Original: `0x142262860` (478 bytes, `Throws::No`).
pub unsafe extern "win64" fn update_cull_lod_thresholds(this: usize) {
    if rd::<u8>(exe::ptr::<u8>(CULL_DISABLED_FLAG_VA) as usize) != 0 {
        return;
    }
    let lod_scale = rd::<f32>(exe::ptr::<f32>(LOD_SCALE_VA) as usize);
    let lod_mode = rd::<u32>(exe::ptr::<u32>(LOD_MODE_VA) as usize);
    let fov_factor = rd::<f32>(exe::ptr::<f32>(LOD_FOV_FACTOR_VA) as usize);
    let abs_mask = rd::<u32>(exe::ptr::<u32>(ABS_MASK_VA) as usize);
    let eps = rd::<f32>(exe::ptr::<f32>(LOD_EPS_VA) as usize);

    let cached_mode = rd::<u32>(exe::ptr::<u32>(CACHED_LOD_MODE_VA) as usize);
    let cached_scale = rd::<f32>(exe::ptr::<f32>(CACHED_LOD_SCALE_VA) as usize);
    let cached_fov = rd::<f32>(exe::ptr::<f32>(CACHED_LOD_FOV_VA) as usize);

    let diff_scale = f32::from_bits((cached_scale - lod_scale).to_bits() & abs_mask);
    let diff_fov = f32::from_bits((cached_fov - fov_factor).to_bits() & abs_mask);

    let can_incremental = cached_mode == lod_mode
        && !(diff_scale > eps)
        && !(diff_fov > eps)
        && rd::<u8>(exe::ptr::<u8>(FORCE_REBUILD_A_VA) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(FORCE_REBUILD_B_VA) as usize) == 0;

    #[cfg(not(target_arch = "aarch64"))]
    let compute_fn =
        exe::func::<unsafe extern "win64" fn(usize, usize, f32, f32)>(0x1_4233_0d80);

    if can_incremental {
        let word_count = (rd::<u64>(this + 0x238).wrapping_add(0x3f) >> 6) as usize;
        let dirty_words = rd::<usize>(this + 0x228);
        for w in 0..word_count {
            let mut bits = rd::<u64>(dirty_words + w * 8);
            wr::<u64>(dirty_words + w * 8, 0);
            while bits != 0 {
                let b = bits.trailing_zeros() as usize;
                let idx = w * 64 + b;
                let obj_arr = rd::<usize>(this + 0x290);
                let obj = rd::<usize>(obj_arr + idx * 8);
                let desc_arr = rd::<usize>(this + 0x240);
                let out_thresh = desc_arr.wrapping_add((idx * 9 + 1) * 4);
                let obj_scale = rd::<f32>(obj + 0x280);
                #[cfg(not(target_arch = "aarch64"))]
                compute_fn(obj, out_thresh, obj_scale, fov_factor);
                #[cfg(target_arch = "aarch64")]
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_4233_0d80,
                    obj as u64,
                    out_thresh as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, 0, obj_scale.to_bits() as u128, fov_factor.to_bits() as u128],
                );
                bits &= !(1u64 << b);
            }
        }
    } else {
        let count = rd::<u32>(this + 0x248) as usize;
        for i in 0..count {
            let obj_arr = rd::<usize>(this + 0x290);
            let obj = rd::<usize>(obj_arr + i * 8);
            if obj != 0 {
                let desc_arr = rd::<usize>(this + 0x240);
                let out_thresh = desc_arr.wrapping_add(i * 0x24 + 4);
                let obj_scale = rd::<f32>(obj + 0x280);
                #[cfg(not(target_arch = "aarch64"))]
                compute_fn(obj, out_thresh, obj_scale, fov_factor);
                #[cfg(target_arch = "aarch64")]
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_4233_0d80,
                    obj as u64,
                    out_thresh as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, 0, obj_scale.to_bits() as u128, fov_factor.to_bits() as u128],
                );
            }
        }
        let final_scale = if count != 0 {
            rd::<f32>(exe::ptr::<f32>(LOD_SCALE_VA) as usize)
        } else {
            lod_scale
        };
        wr::<f32>(exe::ptr::<f32>(CACHED_LOD_SCALE_VA) as usize, final_scale);
        wr::<f32>(exe::ptr::<f32>(CACHED_LOD_FOV_VA) as usize, fov_factor);
        wr::<u32>(exe::ptr::<u32>(CACHED_LOD_MODE_VA) as usize, lod_mode);
    }
}

/// Forward free-range request `(start, count)` to `this + 0x590` (`0x142229c90`).
///
/// Original: `0x142262a50` (12 bytes, `Throws::No`).
pub unsafe extern "win64" fn free_submesh_range_at_590(this: usize, start: u32, count: u32) {
    exe::call_va!(0x1_4222_9c90, (this.wrapping_add(0x590), start, count) -> ());
}

/// Allocate `count` contiguous slots from `this + 0x590` (`0x142229a00`), growing
/// the backing vectors at `+0x5a8` and `+0x5b8` in 4096-aligned chunks on miss.
///
/// Original: `0x142262a70` (156 bytes, `Throws::No`).
pub unsafe extern "win64" fn alloc_submesh_range_at_590(this: usize, count: u32) -> u32 {
    let found = exe::call_va!(0x1_4222_9a00, (this.wrapping_add(0x590), count) -> u32);
    if found != u32::MAX {
        return found;
    }
    let old_total = rd::<u32>(this + 0x590);
    let aligned = count.wrapping_add(0xfff) & !0xfff;
    wr::<u32>(this + 0x590, old_total.wrapping_add(aligned));
    let mut dummy: u64 = 0;
    exe::call_va!(
        0x1_412f_8ca0,
        (this.wrapping_add(0x5a8), &mut dummy as *mut u64 as usize, aligned) -> ()
    );
    exe::call_va!(
        0x1_412f_8ca0,
        (this.wrapping_add(0x5b8), &mut dummy as *mut u64 as usize, aligned) -> ()
    );
    let rem = aligned.wrapping_sub(count);
    if rem != 0 {
        let slot = exe::call_va!(0x1_412f_8d90, (this.wrapping_add(0x598)) -> usize);
        wr::<u32>(slot, old_total.wrapping_add(count));
        wr::<u32>(slot + 4, rem);
    }
    old_total
}

const IAT_RELEASE_SEMAPHORE: u64 = 0x1_4264_5580;
const GLOBAL_RENDER_JOB_HANDLE_VA: u64 = 0x1_43c8_c840;
const POST_CULL_JOB_VTBL_VA: u64 = 0x1_433f_3710;

unsafe fn run_render_world_cull_job(this: usize, outer_offset: usize) {
    let outer0 = rd::<usize>(this + outer_offset);
    let cull_mgr0 = rd::<usize>(outer0 + 0x1a8);
    exe::call_va!(0x1_4226_2860, (cull_mgr0) -> ());

    let outer1 = rd::<usize>(this + outer_offset);
    let cull_mgr1 = rd::<usize>(outer1 + 0x1a8);
    if rd::<u8>(exe::ptr::<u8>(CULL_DISABLED_FLAG_VA) as usize) == 0 {
        let count = rd::<u32>(cull_mgr1 + 0x20) as usize;
        if count != 0 {
            exe::call_va!(0x1_4225_9da0, (0usize, count) -> ());
            exe::call_va!(0x1_4225_9370, (cull_mgr1, count) -> ());
        }
    }

    let outer2 = rd::<usize>(this + outer_offset);
    let sched_holder = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    let sched = rd::<usize>(sched_holder + 0x10);
    let w = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool = rd::<usize>(sched + 0x48 + (w << 6)).wrapping_add(0x40);
    let mut handle = [0u64; 2];
    let _ = exe::call_va!(0x1_4248_5aa0, (pool, handle.as_mut_ptr() as usize) -> usize);
    let job = handle[0] as usize;
    AtomicI32::from_ptr(job as *mut i32).swap(1, Ordering::SeqCst);
    wr::<u64>(job + 0x8, 0);
    wr::<usize>(job + 0x10, exe::ptr::<u8>(POST_CULL_JOB_VTBL_VA) as usize);
    wr::<usize>(job + 0x18, outer2);

    let g_handle = exe::ptr::<u64>(GLOBAL_RENDER_JOB_HANDLE_VA) as usize;
    wr::<u64>(g_handle, handle[0]);
    wr::<u64>(g_handle + 8, handle[1]);

    let sched_holder2 = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    if rd::<u32>(job) == 1 {
        let sched2 = rd::<usize>(sched_holder2 + 0x10);
        let job_arg = rd::<usize>(g_handle);
        exe::call_va!(0x1_4248_6460, (sched2, job_arg) -> ());
        let q_base = rd::<usize>(sched2);
        let sem = rd::<usize>(q_base + 0x1008);
        let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1u32, 0usize) -> i32);
    }

    let counter_ptr = rd::<usize>(this + outer_offset - 8);
    AtomicI32::from_ptr(counter_ptr as *mut i32).fetch_sub(1, Ordering::SeqCst);
}

/// `r::JobCallable<rend::RendererInterface::renderWorldInternal::<lambda_3>::operator()::<lambda_1>>::vfunc_1`
/// (reads renderer pointer from `this + 0x10`).
///
/// Original: `0x1422640a0` (262 bytes, `Throws::No`).
pub unsafe extern "win64" fn render_world_cull_job_10(this: usize) {
    run_render_world_cull_job(this, 0x10);
}

/// Duplicate specialization of `render_world_cull_job_10` reading renderer pointer from `this + 0x8`.
///
/// Original: `0x14228ae00` (261 bytes, `Throws::No`).
pub unsafe extern "win64" fn render_world_cull_job_08(this: usize) {
    run_render_world_cull_job(this, 0x8);
}

const SCENE_INST_MGR_PTR_VA: u64 = 0x1_4397_fed0;

/// Return `true` if submesh `sub` (`rcx`) has a valid instance slot (`+0x50 != -1`),
/// is mapped in `SCENE_INST_MGR_PTR_VA`, and its clamped LOD has positive submesh count (`+0x1c8`).
///
/// Original: `0x142308140` (97 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn is_submesh_lod_visible(sub: usize) -> bool {
    let inst_id = rd::<u32>(sub + 0x50);
    if inst_id == u32::MAX {
        return false;
    }
    let scene_mgr = rd::<usize>(exe::ptr::<usize>(SCENE_INST_MGR_PTR_VA) as usize);
    let id_map = rd::<usize>(scene_mgr + 0x100);
    if rd::<u32>(id_map + (inst_id as usize) * 4) == u32::MAX {
        return false;
    }
    let mesh = rd::<usize>(sub + 0x8);
    let sub_idx = rd::<u32>(sub + 0x48) as usize;
    let cull_mgr = rd::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);
    let lod_count = rd::<u8>(mesh + 0x272) as i32;
    let lod_ptr = rd::<usize>(cull_mgr + 0x1a8).wrapping_add(sub_idx);
    if lod_count == 0 {
        return false;
    }
    let cur_lod = rd::<u8>(lod_ptr) as i32;
    let max_idx = lod_count - 1;
    let clamped = if max_idx < cur_lod { max_idx } else { cur_lod } as isize;
    rd::<i32>((mesh as isize + clamped * 4 + 0x1c8) as usize) > 0
}

/// Resolve the 48-byte submesh LOD entry in `cull_mgr + 0x5a8` for `sub` (`rcx`), or `0` if culled.
///
/// Original: `0x1423081b0` (157 bytes, `Throws::No`).
pub unsafe extern "win64" fn get_submesh_lod_entry(sub: usize) -> usize {
    get_submesh_lod_entry_with(sub, |entry, lod| exe::call_va!(0x1_4222_f750, (entry, lod) -> u32))
}

/// [`get_submesh_lod_entry`] with the `0x14222f750(inst_entry, raw_lod)` slot lookup supplied
/// by the caller (the parallel cull helpers call the native replacement directly).
#[inline(always)]
pub(super) unsafe fn get_submesh_lod_entry_with(sub: usize, slot_of: impl Fn(usize, u32) -> u32) -> usize {
    let inst_id = rd::<u32>(sub + 0x50);
    if inst_id == u32::MAX {
        return 0;
    }
    let sub_idx = rd::<u32>(sub + 0x48) as usize;
    let mesh = rd::<usize>(sub + 0x8);
    let cull_mgr = rd::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);
    let lod_count = rd::<u8>(mesh + 0x272) as i32;
    let lod_ptr = rd::<usize>(cull_mgr + 0x1a8).wrapping_add(sub_idx);
    if lod_count == 0 {
        return 0;
    }
    let raw_lod = rd::<u8>(lod_ptr) as u32;
    let max_idx = lod_count - 1;
    let clamped = if max_idx < (raw_lod as i32) {
        max_idx
    } else {
        raw_lod as i32
    } as isize;
    if rd::<i32>((mesh as isize + clamped * 4 + 0x1c8) as usize) <= 0 {
        return 0;
    }
    let scene_mgr = rd::<usize>(exe::ptr::<usize>(SCENE_INST_MGR_PTR_VA) as usize);
    let inst_table = rd::<usize>(scene_mgr + 0x120);
    let id_map = rd::<usize>(scene_mgr + 0x100);
    if rd::<u32>(id_map + (inst_id as usize) * 4) == u32::MAX {
        return 0;
    }
    let inst_entry = inst_table.wrapping_add((inst_id as usize) * 0xa0);
    let slot = slot_of(inst_entry, raw_lod) as usize;
    rd::<usize>(cull_mgr + 0x5a8).wrapping_add(slot * 0x30)
}

/// Scalar search across 8 float thresholds at `thresholds` (`rcx`) for the first index
/// where `thresholds[i] > val` (`xmm1`), returning `8` if none.
///
/// Original: `0x142308260` (35 bytes, leaf, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn lod_find_threshold_scalar(thresholds: usize, val: f32) -> u32 {
    lod_find_threshold_scalar_impl(thresholds, val)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn lod_find_threshold_scalar(
    thresholds: usize,
    _xmm0: f32,
    val: f32,
) -> u32 {
    lod_find_threshold_scalar_impl(thresholds, val)
}

#[inline(always)]
unsafe fn lod_find_threshold_scalar_impl(thresholds: usize, val: f32) -> u32 {
    for i in 0..8 {
        let t = rd::<f32>(thresholds + i * 4);
        if t > val {
            return i as u32;
        }
    }
    8
}

/// Return `true` when `ring` (`+0x30 != +0x34`) is non-empty and `*ptr != RING_STOP_SENTINEL`.
///
/// Original: `0x1423082a0` (34 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_has_packet_at(ring: usize, ptr: usize) -> bool {
    if rd::<u32>(ring + 0x30) == rd::<u32>(ring + 0x34) {
        return false;
    }
    rd::<u64>(ptr) != RING_STOP_SENTINEL
}

/// Return `true` when `ring` has no ready command packet (`+0x30 == +0x34` or stop sentinel at read_ptr).
///
/// Original: `0x1423083e0` (54 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_is_empty(ring: usize) -> bool {
    if rd::<u32>(ring + 0x30) == rd::<u32>(ring + 0x34) {
        return true;
    }
    let read_ptr = rd::<usize>(ring + 0x20);
    if rd::<u32>(ring + 0x30) == rd::<u32>(ring + 0x34) {
        return true;
    }
    rd::<u64>(read_ptr) == RING_STOP_SENTINEL
}

/// Return the signed difference `(ring->write_count - ring->read_count)` (`+0x30` minus `+0x34`).
///
/// Original: `0x142308420` (14 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_pending_count(ring: usize) -> i64 {
    let w = rd::<u32>(ring + 0x30);
    let r = rd::<u32>(ring + 0x34);
    (w.wrapping_sub(r) as i32) as i64
}

/// Write `2` to `*ring` (`+0x0`).
///
/// Original: `0x142308590` (7 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_set_mode_2(ring: usize) {
    wr::<u32>(ring, 2);
}

/// Initialize a `CmdRing` at `ring` (`rcx`) with buffer byte size `size` (`rdx`),
/// allocating the backing buffer via `0x1423082d0`.
///
/// Original: `0x1423085d0` (97 bytes, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_init(ring: usize, size: usize) -> usize {
    wr::<u32>(ring + 0x38, 2);
    wr::<usize>(ring + 0x8, 0);
    wr::<usize>(ring + 0x10, 0);
    wr::<usize>(ring + 0x18, 0);
    wr::<usize>(ring + 0x20, 0);
    wr::<usize>(ring + 0x28, 0);
    wr::<u64>(ring + 0x30, 0);
    wr::<u32>(ring, 0);
    let buf = exe::call_va!(0x1_4230_82d0, (ring, size, 0usize, size) -> usize);
    wr::<usize>(ring + 0x8, buf);
    wr::<usize>(ring + 0x10, buf.wrapping_add(size));
    AtomicU64::from_ptr((ring + 0x20) as *mut u64).swap(buf as u64, Ordering::SeqCst);
    let base = rd::<usize>(ring + 0x8);
    AtomicU64::from_ptr((ring + 0x28) as *mut u64).swap(base as u64, Ordering::SeqCst);
    ring
}

const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;

/// Allocate a command packet of `size` (`rdx`) bytes in `ring` (`rcx`), spinning if `wait != 0`
/// (`r8b`) when the ring is full, or returning `0` if `size == 0` or full with `wait == 0`.
///
/// Original: `0x142308640` (270 bytes, `Throws::No`).
pub unsafe extern "win64" fn cmd_ring_alloc(ring: usize, size: usize, wait: u8) -> usize {
    if size == 0 {
        return 0;
    }
    let lock = ring.wrapping_add(0x18);
    exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());

    let load_u64 = |addr: usize| unsafe { rd_acq_u64(addr) };
    let load_u32 = |addr: usize| unsafe { rd_acq_u32(addr) };
    let mut read_ptr = load_u64(ring + 0x20) as usize;
    let aligned_sz = (size.wrapping_add(7)) & !7;
    let mut write_ptr = load_u64(ring + 0x28) as usize;
    let mut next_ptr = write_ptr.wrapping_add(8).wrapping_add(aligned_sz);

    if next_ptr > rd::<usize>(ring + 0x10) {
        if write_ptr <= read_ptr {
            while next_ptr > read_ptr {
                if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
                    break;
                }
                if wait == 0 {
                    exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
                    return 0;
                }
                core::hint::spin_loop();
                read_ptr = load_u64(ring + 0x20) as usize;
                if write_ptr > read_ptr {
                    break;
                }
            }
        }
        let end_minus_8 = rd::<usize>(ring + 0x10).wrapping_sub(8);
        if write_ptr <= end_minus_8 {
            wr_rel_u64(write_ptr, RING_WRAP_SENTINEL);
        }
        write_ptr = rd::<usize>(ring + 0x8);
        next_ptr = write_ptr.wrapping_add(8).wrapping_add(aligned_sz);
    }

    if write_ptr <= read_ptr {
        while next_ptr > read_ptr {
            if load_u32(ring + 0x30) == load_u32(ring + 0x34) {
                break;
            }
            if wait == 0 {
                exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
                return 0;
            }
            core::hint::spin_loop();
            read_ptr = load_u64(ring + 0x20) as usize;
            if write_ptr > read_ptr {
                break;
            }
        }
    }

    wr_rel_u64(write_ptr, RING_STOP_SENTINEL);
    AtomicU64::from_ptr((ring + 0x28) as *mut u64).swap(next_ptr as u64, Ordering::SeqCst);
    let payload = write_ptr.wrapping_add(8);
    exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
    payload
}

/// Search `count = *table` u32 keys at `table + 0xa8 + i * 4` for `id` (`edx`),
/// returning `table + 8 + i * 40` on hit or `0` on miss.
///
/// Original: `0x142308980` (50 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn find_entry_by_id_40(table: usize, id: u32) -> usize {
    let count = rd::<usize>(table);
    for i in 0..count {
        if rd::<u32>(table + 0xa8 + i * 4) == id {
            return table.wrapping_add((i * 5 + 1) * 8);
        }
    }
    0
}

/// Wait on the job handle at `this + 0x60` via `r::JobSystem::scheduler_wait_handle_any` (`0x142485780`).
///
/// Original: `0x14230be10` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn wait_post_cull_job_60(this: usize) {
    let sched = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    exe::call_va!(0x1_4248_5780, (sched, this.wrapping_add(0x60)) -> ());
}

const JOB1_VTBL_VA: u64 = 0x1_433f_a700;
const JOB2_VTBL_VA: u64 = 0x1_433f_a6b0;

/// Schedule the two post-culling jobs (`this + 0x60` and `this + 0x70`) when neither is currently active.
///
/// Original: `0x14230be30` (376 bytes, `Throws::No`).
pub unsafe extern "win64" fn schedule_post_cull_jobs(this: usize) {
    let j60 = rd::<usize>(this + 0x60);
    if j60 != 0 {
        let gen = rd::<u16>(j60 + 0x6) >> 3;
        if rd::<u16>(this + 0x68) == gen && rd::<u32>(j60) != 0 {
            return;
        }
    }
    let j70 = rd::<usize>(this + 0x70);
    if j70 != 0 {
        let gen = rd::<u16>(j70 + 0x6) >> 3;
        if rd::<u16>(this + 0x78) == gen && rd::<u32>(j70) != 0 {
            return;
        }
    }

    let sched_holder = rd::<usize>(exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize);
    let seq = rd::<u64>(this + 0xe0);
    wr::<u64>(this + 0xe0, seq.wrapping_add(1));

    let sched1 = rd::<usize>(sched_holder + 0x10);
    let w1 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool1 = rd::<usize>(sched1 + 0x48 + (w1 << 6)).wrapping_add(0x40);
    let mut h1 = [0u64; 2];
    let _ = exe::call_va!(0x1_4248_5aa0, (pool1, h1.as_mut_ptr() as usize) -> usize);
    let job1 = h1[0] as usize;
    AtomicI32::from_ptr(job1 as *mut i32).swap(1, Ordering::SeqCst);
    wr::<u64>(job1 + 0x8, 0);
    wr::<usize>(job1 + 0x10, exe::ptr::<u8>(JOB1_VTBL_VA) as usize);
    wr::<usize>(job1 + 0x18, this);
    wr::<u64>(job1 + 0x20, seq);
    wr::<u64>(this + 0x60, h1[0]);
    wr::<u64>(this + 0x68, h1[1]);

    let sched2 = rd::<usize>(sched_holder + 0x10);
    let w2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool2 = rd::<usize>(sched2 + 0x48 + (w2 << 6)).wrapping_add(0x40);
    let mut h2 = [0u64; 2];
    let _ = exe::call_va!(0x1_4248_5aa0, (pool2, h2.as_mut_ptr() as usize) -> usize);
    let job2 = h2[0] as usize;
    AtomicI32::from_ptr(job2 as *mut i32).swap(1, Ordering::SeqCst);
    wr::<u64>(job2 + 0x8, 0);
    wr::<usize>(job2 + 0x10, exe::ptr::<u8>(JOB2_VTBL_VA) as usize);
    wr::<usize>(job2 + 0x18, this);
    wr::<u64>(this + 0x70, h2[0]);
    wr::<u64>(this + 0x78, h2[1]);

    let j1_ptr = rd::<usize>(this + 0x60);
    let j2_ptr = rd::<usize>(this + 0x70);
    AtomicI32::from_ptr(j2_ptr as *mut i32).fetch_add(1, Ordering::SeqCst);
    wr::<usize>(j1_ptr + 0x8, j2_ptr);
    exe::call_va!(0x1_4248_5930, (sched_holder, this.wrapping_add(0x60)) -> ());
    exe::call_va!(0x1_4248_5930, (sched_holder, this.wrapping_add(0x70)) -> ());
}

/// Compare two small-buffer-optimized pair arrays (`a`, `b`), where `count <= 1` stores
/// elements inline at `+0x8` and `+0x10`, and `count > 1` stores heap pointers there.
///
/// Original: `0x142330640` (150 bytes, `Throws::No`).
pub unsafe extern "win64" fn render_pair_array_eq(a: usize, b: usize) -> bool {
    let count = rd::<u32>(a + 0x4);
    if rd::<u32>(a) != rd::<u32>(b) || count != rd::<u32>(b + 0x4) {
        return false;
    }
    let byte_len = (count as usize) << 3;
    let b_p0 = if rd::<u32>(b + 0x4) > 1 {
        rd::<usize>(b + 0x8)
    } else {
        b + 0x8
    };
    let a_p0 = if count > 1 {
        rd::<usize>(a + 0x8)
    } else {
        a + 0x8
    };
    if exe::call_va!(0x1_4183_acda, (a_p0, b_p0, byte_len) -> i32) != 0 {
        return false;
    }
    let b_p1 = if rd::<u32>(b + 0x4) > 1 {
        rd::<usize>(b + 0x10)
    } else {
        b + 0x10
    };
    let a_p1 = if count > 1 {
        rd::<usize>(a + 0x10)
    } else {
        a + 0x10
    };
    exe::call_va!(0x1_4183_acda, (a_p1, b_p1, byte_len) -> i32) == 0
}

/// Destroy heap-allocated arrays at `this + 0x8` and `this + 0x10` when `this->count > 1`.
///
/// Original: `0x1423306e0` (50 bytes, `Throws::No`).
pub unsafe extern "win64" fn render_pair_array_dtor(this: usize) {
    if rd::<u32>(this + 0x4) > 1 {
        let p0 = rd::<usize>(this + 0x8);
        if p0 != 0 {
            exe::call_va!(0x1_4244_2de0, (p0) -> ());
        }
        let p1 = rd::<usize>(this + 0x10);
        if p1 != 0 {
            exe::call_va!(0x1_4244_2de0, (p1) -> ());
        }
    }
}

/// Copy-construct a small-buffer-optimized pair array `this` (`rcx`) from `src` (`rdx`).
///
/// Original: `0x142330720` (168 bytes, `Throws::No`).
pub unsafe extern "win64" fn render_pair_array_copy_ctor(this: usize, src: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u32>(this, rd::<u32>(src));
    let count = rd::<u32>(src + 0x4);
    wr::<u32>(this + 0x4, count);
    if count <= 1 {
        wr::<u64>(this + 0x8, rd::<u64>(src + 0x8));
        wr::<u64>(this + 0x10, rd::<u64>(src + 0x10));
        return this;
    }
    let p0 = exe::call_va!(0x1_4232_a430, (this, src, 0usize, count) -> usize);
    wr::<usize>(this + 0x8, p0);
    let count2 = rd::<u32>(src + 0x4);
    let p1 = exe::call_va!(0x1_4232_a310, (this, src, 0usize, count2) -> usize);
    wr::<usize>(this + 0x10, p1);

    let c0 = rd::<u32>(src + 0x4) as usize;
    let src0 = if c0 > 1 { rd::<usize>(src + 0x8) } else { src + 0x8 };
    let _ = exe::call_va!(0x1_4183_acbc, (rd::<usize>(this + 0x8), src0, c0 << 3) -> usize);

    let c1 = rd::<u32>(src + 0x4) as usize;
    let src1 = if c1 > 1 { rd::<usize>(src + 0x10) } else { src + 0x10 };
    let _ = exe::call_va!(0x1_4183_acbc, (rd::<usize>(this + 0x10), src1, c1 << 3) -> usize);
    this
}

/// Return `this->buckets + idx * 0x338` (`+0x118`) when `idx < this->count` (`+0x120`),
/// else `this->default_bucket` (`+0x128`).
///
/// Original: `0x1423307d0` (37 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn get_render_bucket_or_default_338(this: usize, idx: u32) -> usize {
    let count = rd::<u32>(this + 0x120);
    if idx < count {
        rd::<usize>(this + 0x118).wrapping_add((idx as usize) * 0x338)
    } else {
        rd::<usize>(this + 0x128)
    }
}

/// Return the pointer at `+0xc8` of the sub-entry selected by `(idx, group)`.
///
/// Original: `0x142330800` (73 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn get_render_sub_entry_c8(this: usize, idx: i32, group: i32) -> usize {
    let idx_off = (idx as isize as usize).wrapping_mul(8);
    let arr = if group != 0 && rd::<u32>(this + 0x178) != 0 {
        let g = (group as isize as usize).wrapping_mul(24);
        let groups = rd::<usize>(this + 0x170);
        rd::<usize>(groups.wrapping_add(g).wrapping_sub(0x10))
    } else {
        rd::<usize>(this + 0x160)
    };
    let elem = rd::<usize>(arr.wrapping_add(idx_off));
    rd::<usize>(elem + 0xc8)
}

/// Return `this->grid[row * this->cols + col]` (`cols` u8 at `+0x270`, `grid` ptr at `+0x380`).
///
/// Original: `0x142330850` (28 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn get_render_grid_entry_380(this: usize, row: i32, col: i32) -> usize {
    let cols = rd::<u8>(this + 0x270) as i32;
    let flat = cols.wrapping_mul(row).wrapping_add(col) as isize as usize;
    let grid = rd::<usize>(this + 0x380);
    rd::<usize>(grid.wrapping_add(flat.wrapping_mul(8)))
}

/// Update the active submesh bitmask (`+0x458`) on render instance `this` (`rcx`),
/// acquiring/releasing submesh references and updating the slot table at `*(this + 0x428)`.
///
/// Original: `0x142330950` (402 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn update_render_instance_submesh_mask(this: usize, mask: u32) {
    if rd::<u32>(this + 0x458) == mask {
        return;
    }
    wr::<u32>(this + 0x458, mask);
    if rd::<u32>(this + 0x450) == 0 {
        return;
    }
    let mut i = 0usize;
    let mut off = 0usize;
    loop {
        let entry = rd::<usize>(this + 0x448).wrapping_add(off);
        let matched = (mask & rd::<u32>(entry + 0xc)) != 0;
        let matched_u8 = u8::from(matched);
        if matched_u8 != rd::<u8>(entry + 0x14) {
            wr::<u8>(entry + 0x14, matched_u8);
            if matched {
                let sub = rd::<usize>(entry);
                let prev = AtomicI32::from_ptr((sub + 0x24) as *mut i32)
                    .fetch_add(1, Ordering::SeqCst);
                if prev + 1 == 1 {
                    let cb = rd::<usize>(sub + 0x28);
                    if cb != 0 {
                        exe::call_va!(0x1_4241_d6f0, (cb) -> ());
                    }
                }
                let sub2 = rd::<usize>(entry);
                let h_val = if sub2 != 0 {
                    exe::call_va!(0x1_4220_ad50, (sub2) -> ());
                    rd::<u32>(sub2 + 0xc4)
                } else {
                    u32::MAX
                };
                let slot1 = (rd::<u16>(this + 0x432) as usize) + i;
                let tbl1 = rd::<usize>(this + 0x428);
                wr::<u32>(tbl1 + slot1 * 4 + 0x1000, h_val);

                let slot2 = (rd::<u16>(this + 0x432) as usize) + i;
                let tbl2 = rd::<usize>(this + 0x428);
                let old_ptr = rd::<usize>(tbl2 + slot2 * 8 + 0x2000);
                wr::<usize>(tbl2 + slot2 * 8 + 0x2000, sub2);
                if old_ptr != 0 {
                    exe::call_va!(0x1_4220_ac20, (old_ptr) -> ());
                }
            } else {
                let slot1 = (rd::<u16>(this + 0x432) as usize) + i;
                let tbl1 = rd::<usize>(this + 0x428);
                wr::<u32>(tbl1 + slot1 * 4 + 0x1000, u32::MAX);

                let slot2 = (rd::<u16>(this + 0x432) as usize) + i;
                let tbl2 = rd::<usize>(this + 0x428);
                let old_ptr = rd::<usize>(tbl2 + slot2 * 8 + 0x2000);
                wr::<usize>(tbl2 + slot2 * 8 + 0x2000, 0);
                if old_ptr != 0 {
                    exe::call_va!(0x1_4220_ac20, (old_ptr) -> ());
                }
                let sub = rd::<usize>(entry);
                let prev = AtomicI32::from_ptr((sub + 0x24) as *mut i32)
                    .fetch_sub(1, Ordering::SeqCst);
                if prev == 1 {
                    let cb = rd::<usize>(sub + 0x28);
                    if cb != 0 {
                        exe::call_va!(0x1_4241_d6f0, (cb) -> ());
                    }
                }
            }
        }
        let total = rd::<u32>(this + 0x450) as usize;
        i += 1;
        off += 0x18;
        if i >= total {
            break;
        }
    }
}

/// Return the pointer at `+0xc0` of the sub-entry selected by `(idx, group)`.
///
/// Original: `0x142330cf0` (73 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn get_render_sub_entry_c0(this: usize, idx: i32, group: i32) -> usize {
    let idx_off = (idx as isize as usize).wrapping_mul(8);
    let arr = if group != 0 && rd::<u32>(this + 0x178) != 0 {
        let g = (group as isize as usize).wrapping_mul(24);
        let groups = rd::<usize>(this + 0x170);
        rd::<usize>(groups.wrapping_add(g).wrapping_sub(0x10))
    } else {
        rd::<usize>(this + 0x160)
    };
    let elem = rd::<usize>(arr.wrapping_add(idx_off));
    rd::<usize>(elem + 0xc0)
}

/// Search `this->ids` (`+0x190`, offset `+0x1e8`, count `+0x1c8`) for `id` (`edx`) via `0x142489d30`,
/// or return `u32::MAX` if `this->ids == 0`.
///
/// Original: `0x142330d40` (44 bytes, `Throws::No`).
pub unsafe extern "win64" fn lookup_mesh_bone_or_submesh_id(this: usize, id: u32) -> u32 {
    let base = rd::<usize>(this + 0x190);
    if base == 0 {
        return u32::MAX;
    }
    let off = rd::<u32>(this + 0x1e8) as usize;
    let count = rd::<u32>(this + 0x1c8);
    exe::call_va!(0x1_4248_9d30, (base.wrapping_add(off * 4), count, id) -> u32)
}

/// Return the 1-based index of `tag` (`edx`) in `this->tags` (`+0x170`, count `+0x178`, stride 24),
/// or `0` if `tag == 0`, `tag == 0x933b5bde`, or not found.
///
/// Original: `0x142330fe0` (71 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn find_render_tag_1based(this: usize, tag: u32) -> u32 {
    if tag == 0x933b_5bde || tag == 0 {
        return 0;
    }
    let start = rd::<usize>(this + 0x170);
    let count = rd::<u32>(this + 0x178) as usize;
    let end = start.wrapping_add(count * 24);
    let mut cur = start;
    let mut idx: u32 = 1;
    while cur != end {
        if rd::<u32>(cur) == tag {
            return idx;
        }
        idx = idx.wrapping_add(1);
        cur = cur.wrapping_add(24);
    }
    0
}


pub mod difftests;
pub use difftests::*;
pub mod worker;
pub use worker::*;
mod par;
mod par_tests;
