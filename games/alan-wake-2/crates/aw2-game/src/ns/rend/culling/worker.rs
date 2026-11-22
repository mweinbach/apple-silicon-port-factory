//! Renderer worker hot-path functions (`0x14133ca40`, `0x142304820`, `0x1422fec00`,
//! `0x142261520`, `0x142259da0`) and differential tests.

use super::{compute_transformed_bounds, get_submesh_lod_entry, pl, ps, CULL_MGR_PTR_VA};
use crate::difftest::{self, Rng};
use crate::exe;

const SHADER_RES_CTRL_VA: u64 = 0x1_4386_c5b8;
const SHADER_RES_SLOTS_VA: u64 = 0x1_4386_c5c0;
const SHADER_RES_MASK_VA: u64 = 0x1_4386_c5d0;

const RENDER_PASS_BYPASS_FILTER_VA: u64 = 0x1_4386_c1d8;
const RENDER_PASS_ARRAY_VA: u64 = 0x1_43bb_d180;
const RENDER_PASS_COUNT_VA: u64 = 0x1_43bb_d188;

const SKINNING_MGR_PTR_VA: u64 = 0x1_4397_fed0;

const CAM_VIEWPOINTS_PTR_VA: u64 = 0x1_43bb_ddd8;
const CAM_VIEWPOINTS_COUNT_VA: u64 = 0x1_43bb_dde0;
const CAM_FOV_RAD_VA: u64 = 0x1_4384_170c;
const CAM_RES_SCALE_A_VA: u64 = 0x1_43a3_395c;
const CAM_RES_SCALE_B_VA: u64 = 0x1_4386_c660;
const J_TANF_VA: u64 = 0x1_4183_ad8e;

#[inline(always)]
fn rsqrt_f32(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0f32 / x.sqrt()
    }
}

#[inline(always)]
fn dot3_f32(a: [f32; 3], b: [f32; 3]) -> f32 {
    (a[0] * b[0] + a[1] * b[1]) + a[2] * b[2]
}

// @ 0x14133ca40 verified — SwissTable 16-way control-byte lookup by 8-byte key
pub unsafe extern "win64" fn lookup_shader_resource_slot(
    _this: usize,
    out: *mut u64,
    key_ptr: *const u8,
) -> *mut u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let prime: u64 = 0x0000_0100_0000_01b3;
    let k0 = exe::rd::<u8>(key_ptr as usize) as u64;
    h = (h ^ k0).wrapping_mul(prime);
    for i in 1..8 {
        let b = exe::rd::<u8>(key_ptr as usize + i) as u64;
        h = (h ^ b).wrapping_mul(prime);
    }
    let prod = (h as u128) * (0xde5f_b9d2_6304_58e9u64 as u128);
    let mixed = (prod as u64).wrapping_add((prod >> 64) as u64);

    let mask = exe::rd::<u64>(SHADER_RES_MASK_VA as usize);
    let ctrl = exe::rd::<usize>(SHADER_RES_CTRL_VA as usize);
    let slots = exe::rd::<usize>(SHADER_RES_SLOTS_VA as usize);

    let mut pos = (mixed >> 7) & mask;
    let h2 = (mixed as u8) & 0x7f;
    let mut step = 0u64;

    loop {
        let mut match_mask = 0u32;
        let mut empty_mask = 0u32;
        for i in 0..16usize {
            let cb = exe::rd::<u8>(ctrl.wrapping_add(pos as usize).wrapping_add(i));
            if cb == h2 {
                match_mask |= 1u32 << i;
            }
            if cb == 0x80 {
                empty_mask |= 1u32 << i;
            }
        }
        if match_mask != 0 {
            let key_u64 = exe::rd::<u64>(key_ptr as usize);
            let mut m = match_mask;
            while m != 0 {
                let bit = m.trailing_zeros() as u64;
                let idx = pos.wrapping_add(bit) & mask;
                let slot_addr = slots.wrapping_add((idx as usize) * 24);
                if exe::rd::<u64>(slot_addr) == key_u64 {
                    exe::wr::<u64>(out as usize + 8, slot_addr as u64);
                    exe::wr::<u64>(out as usize, ctrl.wrapping_add(idx as usize) as u64);
                    return out;
                }
                m &= m - 1;
            }
        }
        if empty_mask != 0 {
            exe::wr::<u64>(out as usize, ctrl.wrapping_add(mask as usize) as u64);
            return out;
        }
        step = step.wrapping_add(16);
        pos = pos.wrapping_add(step) & mask;
    }
}

// @ 0x142304820 verified — copy 48-byte instance transform and mark dirty bits
pub unsafe extern "win64" fn set_cull_instance_transform(this: *const u8, transform: *const u8) {
    let idx = exe::rd::<u32>(this as usize + 0x48) as usize;
    let mgr = exe::rd::<usize>(CULL_MGR_PTR_VA as usize);
    let word_idx = idx >> 6;
    let bit_mask = 1u64 << (idx & 0x3f);

    let dirty_a = exe::rd::<usize>(mgr + 0x638).wrapping_add(word_idx * 8);
    exe::wr::<u64>(dirty_a, exe::rd::<u64>(dirty_a) | bit_mask);

    let dst = exe::rd::<usize>(mgr + 0x5c8).wrapping_add(idx * 48);
    let src = transform as usize;
    for off in [0usize, 0x0c, 0x18, 0x24] {
        exe::wr::<u64>(dst + off, exe::rd::<u64>(src + off));
        exe::wr::<u32>(dst + off + 8, exe::rd::<u32>(src + off + 8));
    }

    let idx2 = exe::rd::<u32>(this as usize + 0x48) as usize;
    let dirty_b = exe::rd::<usize>(mgr + 0x38).wrapping_add((idx2 >> 6) * 8);
    exe::wr::<u64>(dirty_b, exe::rd::<u64>(dirty_b) | (1u64 << (idx2 & 0x3f)));
}

// @ 0x1422fec00 verified — fixed-point render pass bitmask propagation
pub unsafe extern "win64" fn propagate_render_pass_masks(
    _this: usize,
    dst_mask: *mut u8,
    filter_mask: *const u8,
    pass_ctx_base: *mut u8,
) {
    let matches_filter = |dst: usize, flt: usize| -> bool {
        for i in 0..32usize {
            let d = unsafe { exe::rd::<u64>(dst + i * 8) };
            let f = unsafe { exe::rd::<u64>(flt + i * 8) };
            if (d & f) != f {
                return false;
            }
        }
        true
    };

    use crate::jobprof;
    let prof_t0 = jobprof::begin();
    let mut prof_rounds: u32 = 0;
    let mut prof_poll_ns: u64 = 0;
    let prof_done = |rounds: u32, poll_ns: u64| {
        jobprof::end(prof_t0, jobprof::K_PROPAGATE, poll_ns, rounds.min(0xffff) as u16, 0);
    };

    loop {
        if exe::rd::<u8>(RENDER_PASS_BYPASS_FILTER_VA as usize) == 0
            && matches_filter(dst_mask as usize, filter_mask as usize)
        {
            prof_done(prof_rounds, prof_poll_ns);
            return;
        }
        prof_rounds = prof_rounds.wrapping_add(1);

        let mut i = 0u32;
        if exe::rd::<u32>(RENDER_PASS_COUNT_VA as usize) > 0 {
            loop {
                let idx = i as usize;
                let entry = exe::rd::<usize>(RENDER_PASS_ARRAY_VA as usize).wrapping_add(idx * 0x2e0);
                let pass_ctx = (pass_ctx_base as usize).wrapping_add(idx * 0x608);
                if exe::rd::<u32>(entry + 0x8) == 1 {
                    let flag = exe::rd::<u32>(pass_ctx);
                    let cb1 = exe::rd::<usize>(entry + 0x88);
                    if cb1 != 0 {
                        let mut ctx_arg = pass_ctx;
                        let vtbl = exe::rd::<usize>(cb1);
                        let pt = jobprof::begin();
                        let cb1_fn = exe::rd::<u64>(vtbl + 0x10);
                        if cb1_fn == exe::ptr::<u8>(crate::ns::rend::gfxgraph_exec::DECLARE_LAMBDA_DO_CALL_VA) as u64 {
                            // GfxGraph command pass: `cmd->execute(pass data)`.
                            let arg = &mut ctx_arg as *mut usize as usize;
                            exe::call_native!(
                                crate::ns::rend::gfxgraph_exec::DECLARE_LAMBDA_DO_CALL_VA,
                                crate::ns::rend::gfxgraph_exec::declare_lambda_do_call,
                                (cb1, arg) -> ()
                            );
                        } else {
                            let _ = exe::call_va!(cb1_fn, (cb1, &mut ctx_arg) -> ());
                        }
                        if let (Some(a), Some(b)) = (pt, jobprof::begin()) {
                            prof_poll_ns += b.saturating_sub(a);
                            let id = if cb1_fn
                                == exe::ptr::<u8>(crate::ns::rend::gfxgraph_exec::DECLARE_LAMBDA_DO_CALL_VA) as u64
                            {
                                exe::rd::<u64>(exe::rd::<usize>(exe::rd::<usize>(cb1 + 8)))
                            } else {
                                cb1_fn
                            };
                            jobprof::end(pt, jobprof::K_PASS_CB1, id, idx as u16, 0);
                        }
                    }
                    if flag == 0 {
                        let cb2 = exe::rd::<usize>(entry + 0xc8);
                        let cb2_fn = if prof_t0.is_some() && cb2 != 0 {
                            exe::rd::<u64>(exe::rd::<usize>(cb2) + 0x10)
                        } else {
                            0
                        };
                        jobprof::end(prof_t0, jobprof::K_PASS_READY, cb2_fn, idx as u16, 0);
                        let et = jobprof::begin();
                        if cb2 != 0 {
                            let mut ctx_arg = pass_ctx;
                            let vtbl = exe::rd::<usize>(cb2);
                            let _ = exe::call_va!(exe::rd::<u64>(vtbl + 0x10), (cb2, &mut ctx_arg) -> ());
                        }
                        for q in 0..32usize {
                            let off = q * 8;
                            let v = exe::rd::<u64>(entry + 0x1d0 + off);
                            let cur = exe::rd::<u64>(dst_mask as usize + off);
                            exe::wr::<u64>(dst_mask as usize + off, cur | v);
                        }
                        exe::wr::<u32>(entry + 0x8, 2);
                        jobprof::end(et, jobprof::K_PASS_EXEC, cb2_fn, idx as u16, 0);
                    }
                }
                i = i.wrapping_add(1);
                let count = exe::rd::<u32>(RENDER_PASS_COUNT_VA as usize) as u64;
                if (i as u64) >= count {
                    break;
                }
            }
        }

        if matches_filter(dst_mask as usize, filter_mask as usize) {
            prof_done(prof_rounds, prof_poll_ns);
            return;
        }
    }
}

// @ 0x142261520 verified — update world-space spheres and packed AABBs for dirty instances
//
// Every dirty index writes only its own sphere (`+0xe8`, 16 B), packed AABB (`+0x120`, 24 B)
// and its bit of the visibility word (`+0xf8`); everything else is read-only, so whole dirty
// words are independent and are split across the `AW2_PAR_CULL` pool (see [`super::par`]).
pub unsafe extern "win64" fn update_dirty_instance_bounds(this: *mut u8) {
    let mgr = this as usize;
    let num_words = (pl::<u64>(mgr + 0x48).wrapping_add(0x3f)) >> 6;
    if num_words == 0 {
        return;
    }
    let c = BoundsCtx {
        inst_ptrs: pl::<usize>(mgr + 0x18),
        transforms: pl::<usize>(mgr + 0x5c8),
        mesh_ptrs: pl::<usize>(mgr + 0x290),
        aabb_stream: pl::<usize>(mgr + 0x120),
        mesh_indices: pl::<usize>(mgr + 0x198),
        dirty_words: pl::<usize>(mgr + 0x38),
        vis_words: pl::<usize>(mgr + 0xf8),
        spheres: pl::<usize>(mgr + 0xe8),
        num_words: num_words as usize,
    };
    if super::par::lanes() >= 2 && bounds_par_ok(&c) {
        let chunks = c.num_words.div_ceil(BOUNDS_CHUNK_WORDS);
        super::par::for_chunks(chunks, &c as *const BoundsCtx as *const (), bounds_chunk_direct);
        return;
    }
    bounds_words::<false>(&c, 0, c.num_words);
}

/// Dirty words per pool chunk.
const BOUNDS_CHUNK_WORDS: usize = 8;
/// Below this many dirty instances the pool round trip is not worth it.
const BOUNDS_PAR_MIN_DIRTY: u32 = 512;

struct BoundsCtx {
    inst_ptrs: usize,
    transforms: usize,
    mesh_ptrs: usize,
    aabb_stream: usize,
    mesh_indices: usize,
    dirty_words: usize,
    vis_words: usize,
    spheres: usize,
    num_words: usize,
}

/// Parallel only when the one emulator-reachable callee on the per-instance path (the
/// submesh-LOD slot helper 0x14222f750 under [`get_submesh_lod_entry`]) dispatches to its native
/// replacement (helpers call it directly), and there is enough dirty work.
unsafe fn bounds_par_ok(c: &BoundsCtx) -> bool {
    if crate::emu::resolve_dispatch(0x1_4222_f750).0 != crate::ns::rend::cauldron_hot_t1::std_14222f750 as *const () as usize {
        return false;
    }
    let mut dirty = 0u32;
    for w in 0..c.num_words {
        dirty += pl::<u64>(c.dirty_words + w * 8).count_ones();
        if dirty >= BOUNDS_PAR_MIN_DIRTY {
            return true;
        }
    }
    false
}

unsafe fn bounds_chunk_direct(ctx: *const (), chunk: usize) {
    let c = &*(ctx as *const BoundsCtx);
    let w0 = chunk * BOUNDS_CHUNK_WORDS;
    bounds_words::<true>(c, w0, (w0 + BOUNDS_CHUNK_WORDS).min(c.num_words));
}

/// [`get_submesh_lod_entry`] with the 0x14222f750 slot lookup called natively (`DIRECT`, for the
/// pool helpers, which have no emulator context) or through `call_va!` (the original path).
#[inline(always)]
unsafe fn submesh_lod_entry<const DIRECT: bool>(sub: usize) -> usize {
    if !DIRECT {
        return get_submesh_lod_entry(sub);
    }
    super::get_submesh_lod_entry_with(sub, |entry, lod| {
        crate::ns::rend::cauldron_hot_t1::std_14222f750(entry, lod as i32)
    })
}

#[inline(always)]
unsafe fn bounds_words<const DIRECT: bool>(c: &BoundsCtx, w_begin: usize, w_end: usize) {
    for word_i in w_begin..w_end {
        let mut word = pl::<u64>(c.dirty_words + word_i * 8);
        if word == 0 {
            continue;
        }
        let base_idx = word_i * 64;
        while word != 0 {
            let bit = word.trailing_zeros() as usize;
            word ^= 1u64 << bit;
            let idx = base_idx + bit;
            let mesh_idx = pl::<i32>(c.mesh_indices + idx * 4);
            if mesh_idx == -1 {
                continue;
            }
            let inst = pl::<usize>(c.inst_ptrs + idx * 8);
            let mesh = pl::<usize>(c.mesh_ptrs + (mesh_idx as u32 as usize) * 8);
            let tf_addr = c.transforms + idx * 48;
            let r0 = pl::<[f32; 4]>(tf_addr);
            let r1 = pl::<[f32; 4]>(tf_addr + 0x10);
            let r2 = pl::<[f32; 4]>(tf_addr + 0x20);
            let col0 = [r0[0], r0[1], r0[2], 0.0f32];
            let col1 = [r0[3], r1[0], r1[1], 0.0f32];
            let col2 = [r1[2], r1[3], r2[0], 0.0f32];
            let col3 = [r2[1], r2[2], r2[3], 1.0f32];

            let mut use_custom = false;
            let skin_id = pl::<i32>(inst + 0x50);
            if skin_id != -1 {
                let skin_mgr = pl::<usize>(exe::ptr::<usize>(SKINNING_MGR_PTR_VA) as usize);
                let skin_table = pl::<usize>(skin_mgr + 0x100);
                if pl::<i32>(skin_table + (skin_id as u32 as usize) * 4) != -1 {
                    let lod_info = pl::<usize>(inst + 0x8);
                    let global_cull = pl::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize);
                    let inst_slot = pl::<u32>(inst + 0x48) as usize;
                    let max_lod = pl::<u8>(lod_info + 0x272);
                    if max_lod != 0 {
                        let lod_arr = pl::<usize>(global_cull + 0x1a8);
                        let cur_lod = pl::<u8>(lod_arr + inst_slot) as i32;
                        let lim = (max_lod as i32) - 1;
                        let clamped = if lim < cur_lod { lim } else { cur_lod };
                        if pl::<i32>(lod_info + 0x1c8 + (clamped as isize as usize) * 4) > 0 {
                            use_custom = true;
                        }
                    }
                }
            }
            if !use_custom {
                let vtbl = pl::<usize>(inst);
                if pl::<u64>(vtbl + 0xc8) != 0 {
                    use_custom = true;
                }
            }

            let transform_sphere_and_aabb =
                |sphere_src: usize, aabb_src: usize| -> ([f32; 4], [f32; 4], [f32; 4]) {
                    let s = unsafe { pl::<[f32; 4]>(sphere_src) };
                    let d0 = dot3_f32([col0[0], col0[1], col0[2]], [col0[0], col0[1], col0[2]]);
                    let d1 = dot3_f32([col1[0], col1[1], col1[2]], [col1[0], col1[1], col1[2]]);
                    let d2 = dot3_f32([col2[0], col2[1], col2[2]], [col2[0], col2[1], col2[2]]);
                    let max_d = d0.max(d1).max(d2).sqrt();
                    let radius = max_d * s[3];
                    let mut center = col3;
                    for lane in 0..4 {
                        center[lane] = s[2].mul_add(col2[lane], center[lane]);
                        center[lane] = s[1].mul_add(col1[lane], center[lane]);
                        center[lane] = s[0].mul_add(col0[lane], center[lane]);
                    }
                    let sphere_out = [center[0], center[1], center[2], radius];

                    let bmin_raw = unsafe { pl::<[f32; 4]>(aabb_src) };
                    let bmax_hi = unsafe { pl::<[f32; 4]>(aabb_src + 8) };
                    let bmax_raw = [bmax_hi[1], bmax_hi[2], bmax_hi[3], bmax_hi[0]];
                    if bmax_raw[0] < bmin_raw[0]
                        && bmax_raw[1] < bmin_raw[1]
                        && bmax_raw[2] < bmin_raw[2]
                    {
                        return (sphere_out, bmin_raw, bmax_raw);
                    }
                    let mut out_min = [0.0f32; 4];
                    let mut out_max = [0.0f32; 4];
                    for lane in 0..4 {
                        let a0 = bmin_raw[0] * col0[lane];
                        let b0 = bmax_raw[0] * col0[lane];
                        let a1 = bmin_raw[1] * col1[lane];
                        let b1 = bmax_raw[1] * col1[lane];
                        let a2 = bmin_raw[2] * col2[lane];
                        let b2 = bmax_raw[2] * col2[lane];
                        let min01 = a1.min(b1) + a0.min(b0);
                        out_min[lane] = (min01 + a2.min(b2)) + col3[lane];
                        let max01 = a1.max(b1) + a0.max(b0);
                        out_max[lane] = (max01 + a2.max(b2)) + col3[lane];
                    }
                    (sphere_out, out_min, out_max)
                };

            let (sphere_out, aabb_min, aabb_max) = if !use_custom {
                transform_sphere_and_aabb(mesh + 0x274, mesh + 0x284)
            } else if (pl::<u32>(inst + 0x4c) & 0x4000) != 0 {
                transform_sphere_and_aabb(inst + 0x6c, inst + 0x7c)
            } else {
                let submesh_data = submesh_lod_entry::<DIRECT>(inst);
                let mut aabb_pair = [[0.0f32; 4]; 2];
                let mut sph = [0.0f32; 4];
                let mat_cols = [col0, col1, col2, col3];
                compute_transformed_bounds(
                    aabb_pair.as_mut_ptr() as usize,
                    sph.as_mut_ptr() as usize,
                    mesh,
                    mat_cols.as_ptr() as usize,
                    submesh_data,
                );
                (sph, aabb_pair[0], aabb_pair[1])
            };

            let vw_addr = c.vis_words + (idx >> 6) * 8;
            ps::<u64>(vw_addr, pl::<u64>(vw_addr) | (1u64 << (idx & 0x3f)));

            ps::<[f32; 4]>(c.spheres + idx * 16, sphere_out);

            let aabb_dst = c.aabb_stream + idx * 24;
            ps::<[f32; 4]>(aabb_dst, aabb_min);
            let tail = [aabb_min[2], aabb_max[0], aabb_max[1], aabb_max[2]];
            ps::<[f32; 4]>(aabb_dst + 8, tail);
        }
    }
}

// @ 0x142259da0 verified — compute per-instance camera distance and screen-space LOD factors
//
// Element `i` reads only sphere `i` and writes only `dist[i]`, `rad[i]`, `lod[i]` (the second
// pass reads back the first pass's `dist[i]`/`rad[i]`), so index ranges are independent; the
// 8-wide/tail formula split stays keyed on the global `count & !7`.
pub unsafe extern "win64" fn compute_submesh_camera_lod_factors(_this: usize, count: usize) {
    let num_cams = pl::<u32>(CAM_VIEWPOINTS_COUNT_VA as usize) as usize;
    let cams_ptr = pl::<usize>(CAM_VIEWPOINTS_PTR_VA as usize);
    let mgr = pl::<usize>(CULL_MGR_PTR_VA as usize);

    const MAX_F32: f32 = f32::from_bits(0x7f7f_ffff);
    let mut max_dist_sq = [MAX_F32; 4];
    let mut min_dist_sq = [0.0f32; 4];
    let mut cams = [[0.0f32; 3]; 4];
    for c in 0..4usize {
        if c < num_cams {
            let d = pl::<f32>(cams_ptr + c * 0x14 + 0x0c);
            let m = pl::<f32>(cams_ptr + c * 0x14 + 0x10);
            max_dist_sq[c] = d * d;
            min_dist_sq[c] = m * m;
            cams[c] = [
                pl::<f32>(cams_ptr + c * 0x14),
                pl::<f32>(cams_ptr + c * 0x14 + 4),
                pl::<f32>(cams_ptr + c * 0x14 + 8),
            ];
        }
    }

    let half_fov = pl::<f32>(CAM_FOV_RAD_VA as usize) * 0.5f32;
    let _ = J_TANF_VA;
    let tan_half_fov = half_fov.tan();
    let c = LodFactorsCtx {
        num_cams,
        cams_ptr,
        cams,
        max_dist_sq,
        min_dist_sq,
        spheres: pl::<usize>(mgr + 0xe8),
        dist: pl::<usize>(mgr + 0x1c8),
        rad: pl::<usize>(mgr + 0x1e8),
        lod_out: pl::<usize>(mgr + 0x1d8),
        scale_a: (pl::<i32>(CAM_RES_SCALE_A_VA as usize) as f32) / tan_half_fov,
        scale_b: (pl::<i32>(CAM_RES_SCALE_B_VA as usize) as f32) / tan_half_fov,
        simd_end: count & !7usize,
        count,
    };
    if count >= LOD_PAR_MIN && super::par::lanes() >= 2 {
        let chunks = count.div_ceil(LOD_CHUNK);
        super::par::for_chunks(chunks, &c as *const LodFactorsCtx as *const (), lod_factors_chunk);
        return;
    }
    lod_factors_range(&c, 0, count);
}

/// Elements per pool chunk (multiple of 64) and the minimum count worth splitting, shared by
/// the camera-LOD and submesh-LOD/culling loops.
pub(super) const LOD_CHUNK: usize = 2048;
pub(super) const LOD_PAR_MIN: usize = 4 * LOD_CHUNK;

struct LodFactorsCtx {
    num_cams: usize,
    cams_ptr: usize,
    cams: [[f32; 3]; 4],
    max_dist_sq: [f32; 4],
    min_dist_sq: [f32; 4],
    spheres: usize,
    dist: usize,
    rad: usize,
    lod_out: usize,
    scale_a: f32,
    scale_b: f32,
    simd_end: usize,
    count: usize,
}

unsafe fn lod_factors_chunk(ctx: *const (), chunk: usize) {
    let c = &*(ctx as *const LodFactorsCtx);
    let s = chunk * LOD_CHUNK;
    lod_factors_range(c, s, (s + LOD_CHUNK).min(c.count));
}

#[inline(always)]
unsafe fn lod_factors_range(c: &LodFactorsCtx, s: usize, e: usize) {
    const MAX_F32: f32 = f32::from_bits(0x7f7f_ffff);
    let (spheres_ptr, dist_ptr, rad_ptr) = (c.spheres, c.dist, c.rad);
    let (max_dist_sq, min_dist_sq) = (c.max_dist_sq, c.min_dist_sq);
    if c.num_cams == 1 {
        let cam0 = c.cams[0];
        for i in s..e {
            let sph = pl::<[f32; 4]>(spheres_ptr + i * 16);
            let diff = [cam0[0] - sph[0], cam0[1] - sph[1], cam0[2] - sph[2]];
            let d2 = dot3_f32(diff, diff);
            let clamped = if max_dist_sq[0] < d2 { MAX_F32 } else { d2 };
            let res = clamped.max(min_dist_sq[0]);
            ps::<f32>(dist_ptr + i * 4, res);
            ps::<f32>(rad_ptr + i * 4, sph[3]);
        }
    } else if c.num_cams == 2 {
        let cam0 = c.cams[0];
        for i in s..e {
            let sph = pl::<[f32; 4]>(spheres_ptr + i * 16);
            // Note: original binary uses `vblendps xmm2, xmm1, xmm0, 0xc` (placing cam0 in
            // lanes 0..1 and cam1 in lanes 2..3) followed by `vpermilps xmm0, xmm3, 0x5`
            // (reducing lanes 0 and 1, both of which hold cam0).
            let diff = [cam0[0] - sph[0], cam0[1] - sph[1], cam0[2] - sph[2]];
            let d2 = dot3_f32(diff, diff);
            let v = if d2 < max_dist_sq[0] { d2 } else { MAX_F32 };
            let best = v.max(min_dist_sq[0]);
            ps::<f32>(dist_ptr + i * 4, best);
            ps::<f32>(rad_ptr + i * 4, sph[3]);
        }
    } else if c.num_cams == 3 || c.num_cams == 4 {
        let nc = c.num_cams;
        for i in s..e {
            let sph = pl::<[f32; 4]>(spheres_ptr + i * 16);
            let mut best = MAX_F32;
            for k in 0..nc {
                let cam = c.cams[k];
                let diff = [cam[0] - sph[0], cam[1] - sph[1], cam[2] - sph[2]];
                let d2 = dot3_f32(diff, diff);
                let v = if d2 < max_dist_sq[k] { d2 } else { MAX_F32 };
                let v = v.max(min_dist_sq[k]);
                best = best.min(v);
            }
            ps::<f32>(dist_ptr + i * 4, best);
            ps::<f32>(rad_ptr + i * 4, sph[3]);
        }
    } else if c.count != 0 {
        let cams_ptr = c.cams_ptr;
        for i in s..e {
            let sph = pl::<[f32; 4]>(spheres_ptr + i * 16);
            let mut best = MAX_F32;
            for k in 0..c.num_cams {
                let base = cams_ptr + k * 0x14;
                let cam = [
                    pl::<f32>(base),
                    pl::<f32>(base + 4),
                    pl::<f32>(base + 8),
                ];
                let diff = [sph[0] - cam[0], sph[1] - cam[1], sph[2] - cam[2]];
                let d2 = dot3_f32(diff, diff);
                let max_d = pl::<f32>(base + 0x0c);
                let min_d = pl::<f32>(base + 0x10);
                let v = if max_d < d2 { MAX_F32 } else { d2 };
                let v = min_d.max(v);
                best = best.min(v);
            }
            ps::<f32>(dist_ptr + i * 4, best);
            ps::<f32>(rad_ptr + i * 4, sph[3]);
        }
    }

    let lod_out_ptr = c.lod_out;
    let (scale_a, scale_b) = (c.scale_a, c.scale_b);

    const EPS: f32 = f32::from_bits(0x2834_24dc);
    const MIN_DENOM: f32 = f32::from_bits(0x3dcc_cccd);
    const ONE: f32 = 1.0f32;
    const C4: f32 = f32::from_bits(0x3d74_552f);
    const C3: f32 = f32::from_bits(0xbeee_7397);
    const C2: f32 = f32::from_bits(0x3fbd_96dd);
    const C1: f32 = f32::from_bits(0xc021_53f6);
    const C0: f32 = f32::from_bits(0x4038_d96c);

    let simd_end = c.simd_end.clamp(s, e);
    for i in s..simd_end {
        let r = pl::<f32>(rad_ptr + i * 4);
        let d2 = pl::<f32>(dist_ptr + i * 4);
        let rsq = rsqrt_f32(d2 + EPS);
        let d = rsq * d2;
        let denom = MIN_DENOM.max(d - r);
        let ratio = (r * scale_a) / denom;
        let twice = ratio + ratio;
        let bits = twice.to_bits();
        let m = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
        let p1 = m.mul_add(C4, C3);
        let p2 = m.mul_add(p1, C2);
        let p3 = m.mul_add(p2, C1);
        let p4 = m.mul_add(p3, C0);
        let poly = p4 * (m - ONE);
        let exp = (((bits & 0x7f80_0000) >> 23) as i32 - 0x7f) as f32;
        let log_val = poly + exp;
        let out_lod = log_val + log_val;
        let out_rad = (r * scale_b) / d;
        ps::<f32>(lod_out_ptr + i * 4, out_lod);
        ps::<f32>(dist_ptr + i * 4, d);
        ps::<f32>(rad_ptr + i * 4, out_rad);
    }

    for i in simd_end..e {
        let r = pl::<f32>(rad_ptr + i * 4);
        let d2 = pl::<f32>(dist_ptr + i * 4);
        let rsq = rsqrt_f32(d2 + EPS);
        let d = rsq * d2;
        let denom = MIN_DENOM.max(d - r);
        let out_rad = (r * scale_b) / d;
        ps::<f32>(rad_ptr + i * 4, out_rad);
        let ratio = (r * scale_a) / denom;
        let twice = ratio + ratio;
        ps::<f32>(dist_ptr + i * 4, d);
        let bits = twice.to_bits();
        let m = f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000);
        let p1 = m * C4 + C3;
        let p2 = p1 * m + C2;
        let p3 = p2 * m + C1;
        let p4 = p3 * m + C0;
        let poly = p4 * (m - ONE);
        let exp = (((bits & 0x7f80_0000) >> 23) as i32 - 0x7f) as f32;
        let log_val = poly + exp;
        let out_lod = log_val + log_val;
        ps::<f32>(lod_out_ptr + i * 4, out_lod);
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_lookup_shader_resource_slot(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, *mut u64, *const u8) -> *mut u64;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1413_3ca4);
    let mut cases = 0u32;

    let saved_ctrl = exe::rd::<usize>(SHADER_RES_CTRL_VA as usize);
    let saved_slots = exe::rd::<usize>(SHADER_RES_SLOTS_VA as usize);
    let saved_mask = exe::rd::<u64>(SHADER_RES_MASK_VA as usize);

    let cap = 64usize;
    let mask = (cap - 1) as u64;
    let mut ctrl = vec![0x80u8; cap + 16];
    let mut slots = vec![0u64; cap * 3];

    let mut keys = Vec::new();
    for i in 0..24usize {
        let key = rng.next_u64() | 1;
        keys.push(key);
        let kb = key.to_le_bytes();
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for &b in &kb {
            h = (h ^ (b as u64)).wrapping_mul(0x100_0000_01b3);
        }
        let prod = (h as u128) * (0xde5f_b9d2_6304_58e9u64 as u128);
        let mixed = (prod as u64).wrapping_add((prod >> 64) as u64);
        let h2 = (mixed as u8) & 0x7f;
        let base_pos = ((mixed >> 7) & mask) as usize;
        let idx = (base_pos + (i % 5)) & (mask as usize);
        ctrl[idx] = h2;
        if idx < 16 {
            ctrl[cap + idx] = h2;
        }
        slots[idx * 3] = key;
        slots[idx * 3 + 1] = (i as u64) + 100;
        slots[idx * 3 + 2] = (i as u64) + 200;
    }

    exe::wr::<usize>(SHADER_RES_CTRL_VA as usize, ctrl.as_mut_ptr() as usize);
    exe::wr::<usize>(SHADER_RES_SLOTS_VA as usize, slots.as_mut_ptr() as usize);
    exe::wr::<u64>(SHADER_RES_MASK_VA as usize, mask);

    for k in keys.iter().copied().chain((0..16).map(|_| rng.next_u64())) {
        let kb = k.to_le_bytes();
        let mut out_a = [0xdead_beef_u64; 2];
        let mut out_b = [0xdead_beef_u64; 2];
        let ra = o(0, out_a.as_mut_ptr(), kb.as_ptr());
        let rb = n(0, out_b.as_mut_ptr(), kb.as_ptr());
        difftest::eq("lookup ret", ra == out_a.as_mut_ptr(), rb == out_b.as_mut_ptr())?;
        difftest::eq("lookup out", out_a, out_b)?;
        cases += 1;
    }

    exe::wr::<usize>(SHADER_RES_CTRL_VA as usize, saved_ctrl);
    exe::wr::<usize>(SHADER_RES_SLOTS_VA as usize, saved_slots);
    exe::wr::<u64>(SHADER_RES_MASK_VA as usize, saved_mask);
    Ok(cases)
}

pub unsafe fn diff_set_cull_instance_transform(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *const u8);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1423_0482);
    let mut cases = 0u32;

    let saved_mgr = exe::rd::<usize>(CULL_MGR_PTR_VA as usize);
    for _ in 0..32 {
        let idx = (rng.next_u32() % 128) as usize;
        let mut inst = [0u8; 0x60];
        exe::wr::<u32>(inst.as_mut_ptr() as usize + 0x48, idx as u32);
        let mut tf = [0u8; 48];
        for b in &mut tf {
            *b = rng.next_u32() as u8;
        }

        let mut tf_a = vec![0u8; 128 * 48];
        let mut tf_b = tf_a.clone();
        let mut d1_a = [0u64; 4];
        let mut d1_b = [0u64; 4];
        let mut d2_a = [0u64; 4];
        let mut d2_b = [0u64; 4];

        let mut mgr_a = vec![0u8; 0x700];
        let mut mgr_b = vec![0u8; 0x700];
        exe::wr::<usize>(mgr_a.as_mut_ptr() as usize + 0x38, d2_a.as_mut_ptr() as usize);
        exe::wr::<usize>(mgr_a.as_mut_ptr() as usize + 0x5c8, tf_a.as_mut_ptr() as usize);
        exe::wr::<usize>(mgr_a.as_mut_ptr() as usize + 0x638, d1_a.as_mut_ptr() as usize);

        exe::wr::<usize>(mgr_b.as_mut_ptr() as usize + 0x38, d2_b.as_mut_ptr() as usize);
        exe::wr::<usize>(mgr_b.as_mut_ptr() as usize + 0x5c8, tf_b.as_mut_ptr() as usize);
        exe::wr::<usize>(mgr_b.as_mut_ptr() as usize + 0x638, d1_b.as_mut_ptr() as usize);

        exe::wr::<usize>(CULL_MGR_PTR_VA as usize, mgr_a.as_mut_ptr() as usize);
        o(inst.as_ptr(), tf.as_ptr());

        exe::wr::<usize>(CULL_MGR_PTR_VA as usize, mgr_b.as_mut_ptr() as usize);
        n(inst.as_ptr(), tf.as_ptr());

        difftest::eq("dirty_a", d1_a, d1_b)?;
        difftest::eq("dirty_b", d2_a, d2_b)?;
        difftest::eq("transforms", &tf_a[..], &tf_b[..])?;
        cases += 1;
    }
    exe::wr::<usize>(CULL_MGR_PTR_VA as usize, saved_mgr);
    Ok(cases)
}

pub unsafe fn diff_propagate_render_pass_masks(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, *mut u8, *const u8, *mut u8);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1422_fec0);
    let mut cases = 0u32;

    let saved_bypass = exe::rd::<u8>(RENDER_PASS_BYPASS_FILTER_VA as usize);
    let saved_arr = exe::rd::<usize>(RENDER_PASS_ARRAY_VA as usize);
    let saved_cnt = exe::rd::<u32>(RENDER_PASS_COUNT_VA as usize);

    for bypass in [0u8, 1u8] {
        for cnt in [0u32, 1, 3] {
            exe::wr::<u8>(RENDER_PASS_BYPASS_FILTER_VA as usize, bypass);
            exe::wr::<u32>(RENDER_PASS_COUNT_VA as usize, cnt);

            let mut entries_a = vec![0u8; (cnt as usize).max(1) * 0x2e0];
            let mut ctx = vec![0u8; (cnt as usize).max(1) * 0x608];
            let mut filter = [0u8; 0x100];
            let mut dst_init = [0u8; 0x100];
            for i in 0..0x100 {
                let f = (rng.next_u32() & 0xff) as u8;
                filter[i] = f;
                dst_init[i] = f;
            }
            for idx in 0..(cnt as usize) {
                let e = entries_a.as_mut_ptr() as usize + idx * 0x2e0;
                exe::wr::<u32>(e + 0x8, if idx % 2 == 0 { 1 } else { 0 });
                for q in 0..32usize {
                    exe::wr::<u64>(e + 0x1d0 + q * 8, rng.next_u64() | 0xffff_ffff_ffff_ffff);
                }
                let c = ctx.as_mut_ptr() as usize + idx * 0x608;
                exe::wr::<u32>(c, if idx == 1 { 1 } else { 0 });
            }
            let mut entries_b = entries_a.clone();
            let mut dst_a = dst_init;
            let mut dst_b = dst_init;

            exe::wr::<usize>(RENDER_PASS_ARRAY_VA as usize, entries_a.as_mut_ptr() as usize);
            o(0, dst_a.as_mut_ptr(), filter.as_ptr(), ctx.as_mut_ptr());

            exe::wr::<usize>(RENDER_PASS_ARRAY_VA as usize, entries_b.as_mut_ptr() as usize);
            n(0, dst_b.as_mut_ptr(), filter.as_ptr(), ctx.as_mut_ptr());

            difftest::eq("dst_mask", &dst_a[..], &dst_b[..])?;
            difftest::eq("entries", &entries_a[..], &entries_b[..])?;
            cases += 1;
        }
    }

    exe::wr::<u8>(RENDER_PASS_BYPASS_FILTER_VA as usize, saved_bypass);
    exe::wr::<usize>(RENDER_PASS_ARRAY_VA as usize, saved_arr);
    exe::wr::<u32>(RENDER_PASS_COUNT_VA as usize, saved_cnt);
    Ok(cases)
}

pub unsafe fn diff_update_dirty_instance_bounds(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1422_6152);
    let mut cases = 0u32;

    for custom_flag in [false, true] {
        let num_inst = 8usize;
        let dirty_word: u64 = 0b1011_0101;
        let mesh_indices: [i32; 8] = [0, -1, 0, 0, 0, 0, -1, 0];

        let mut vtbl = [0u64; 32];
        if custom_flag {
            vtbl[0xc8 / 8] = 1;
        }
        let mut inst = [0u8; 0x100];
        exe::wr::<usize>(inst.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        exe::wr::<i32>(inst.as_mut_ptr() as usize + 0x50, -1);
        exe::wr::<u32>(inst.as_mut_ptr() as usize + 0x4c, if custom_flag { 0x4000 } else { 0 });
        exe::wr::<[f32; 4]>(inst.as_mut_ptr() as usize + 0x6c, [1.0, 2.0, 3.0, 4.5]);
        exe::wr::<[f32; 4]>(inst.as_mut_ptr() as usize + 0x7c, [-2.0, -3.0, -4.0, 0.0]);
        exe::wr::<[f32; 4]>(inst.as_mut_ptr() as usize + 0x84, [0.0, 5.0, 6.0, 7.0]);

        let inst_ptrs = [inst.as_ptr() as usize; 8];

        let mut mesh = [0u8; 0x300];
        exe::wr::<[f32; 4]>(mesh.as_mut_ptr() as usize + 0x274, [0.5, -1.5, 2.5, 3.0]);
        exe::wr::<[f32; 4]>(mesh.as_mut_ptr() as usize + 0x284, [-1.0, -2.0, -3.0, 0.0]);
        exe::wr::<[f32; 4]>(mesh.as_mut_ptr() as usize + 0x28c, [0.0, 2.0, 3.0, 4.0]);
        let mesh_ptrs = [mesh.as_ptr() as usize; 1];

        let mut transforms = [0.0f32; 8 * 12];
        for v in &mut transforms {
            *v = ((rng.next_u32() % 200) as f32 - 100.0) * 0.05;
        }

        let mut vis_a = [0u64; 1];
        let mut vis_b = [0u64; 1];
        let mut sph_a = [0.0f32; 8 * 4];
        let mut sph_b = [0.0f32; 8 * 4];
        let mut aabb_a = [0.0f32; 8 * 6];
        let mut aabb_b = [0.0f32; 8 * 6];

        let mut mgr_a = vec![0u8; 0x600];
        let mut mgr_b = vec![0u8; 0x600];
        for (mgr, vis, sph, aabb) in [
            (&mut mgr_a, &mut vis_a, &mut sph_a, &mut aabb_a),
            (&mut mgr_b, &mut vis_b, &mut sph_b, &mut aabb_b),
        ] {
            let m = mgr.as_mut_ptr() as usize;
            exe::wr::<usize>(m + 0x18, inst_ptrs.as_ptr() as usize);
            exe::wr::<usize>(m + 0x38, (&dirty_word as *const u64) as usize);
            exe::wr::<u64>(m + 0x48, num_inst as u64);
            exe::wr::<usize>(m + 0xe8, sph.as_mut_ptr() as usize);
            exe::wr::<usize>(m + 0xf8, vis.as_mut_ptr() as usize);
            exe::wr::<usize>(m + 0x120, aabb.as_mut_ptr() as usize);
            exe::wr::<usize>(m + 0x198, mesh_indices.as_ptr() as usize);
            exe::wr::<usize>(m + 0x290, mesh_ptrs.as_ptr() as usize);
            exe::wr::<usize>(m + 0x5c8, transforms.as_ptr() as usize);
        }

        o(mgr_a.as_mut_ptr());
        n(mgr_b.as_mut_ptr());

        difftest::eq("vis", vis_a, vis_b)?;
        difftest::eq("spheres", sph_a, sph_b)?;
        difftest::eq("aabbs", aabb_a, aabb_b)?;
        cases += 1;
    }
    cases += super::par_tests::big_update_dirty_instance_bounds(orig, new)?;
    Ok(cases)
}

pub unsafe fn diff_compute_submesh_camera_lod_factors(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    let saved_mgr = exe::rd::<usize>(CULL_MGR_PTR_VA as usize);
    let saved_cams = exe::rd::<usize>(CAM_VIEWPOINTS_PTR_VA as usize);
    let saved_ncams = exe::rd::<u32>(CAM_VIEWPOINTS_COUNT_VA as usize);
    let saved_fov = exe::rd::<f32>(CAM_FOV_RAD_VA as usize);
    let saved_sa = exe::rd::<i32>(CAM_RES_SCALE_A_VA as usize);
    let saved_sb = exe::rd::<i32>(CAM_RES_SCALE_B_VA as usize);

    let cams: [[f32; 5]; 5] = [
        [1.0, 2.0, 3.0, 500.0, 0.5],
        [-4.0, 5.0, -6.0, 600.0, 1.0],
        [10.0, 0.0, 2.0, 400.0, 0.25],
        [0.0, -8.0, 4.0, 800.0, 0.75],
        [3.0, 3.0, 3.0, 1000.0, 0.1],
    ];
    let spheres: [[f32; 4]; 11] = [
        [2.0, 3.0, 4.0, 1.5],
        [-1.0, 4.0, 2.0, 0.75],
        [8.0, -2.0, 5.0, 2.25],
        [0.5, 1.5, -3.5, 1.0],
        [12.0, 6.0, -1.0, 3.5],
        [-5.0, -5.0, -5.0, 0.5],
        [4.0, 0.0, 9.0, 1.25],
        [7.0, 7.0, 1.0, 2.0],
        [-3.0, 2.0, 8.0, 1.75],
        [6.0, -4.0, 2.0, 0.9],
        [1.0, 1.0, 1.0, 0.4],
    ];

    exe::wr::<usize>(CAM_VIEWPOINTS_PTR_VA as usize, cams.as_ptr() as usize);
    exe::wr::<f32>(CAM_FOV_RAD_VA as usize, 1.0471976);
    exe::wr::<i32>(CAM_RES_SCALE_A_VA as usize, 1080);
    exe::wr::<i32>(CAM_RES_SCALE_B_VA as usize, 1920);

    for num_cams in [1u32, 2, 3, 4, 5] {
        exe::wr::<u32>(CAM_VIEWPOINTS_COUNT_VA as usize, num_cams);
        let count = spheres.len();
        let mut dist_a = vec![0.0f32; count];
        let mut dist_b = vec![0.0f32; count];
        let mut rad_a = vec![0.0f32; count];
        let mut rad_b = vec![0.0f32; count];
        let mut lod_a = vec![0.0f32; count];
        let mut lod_b = vec![0.0f32; count];

        let mut mgr_a = vec![0u8; 0x240];
        let mut mgr_b = vec![0u8; 0x240];
        for (mgr, dist, rad, lod) in [
            (&mut mgr_a, &mut dist_a, &mut rad_a, &mut lod_a),
            (&mut mgr_b, &mut dist_b, &mut rad_b, &mut lod_b),
        ] {
            let m = mgr.as_mut_ptr() as usize;
            exe::wr::<usize>(m + 0xe8, spheres.as_ptr() as usize);
            exe::wr::<usize>(m + 0x1c8, dist.as_mut_ptr() as usize);
            exe::wr::<usize>(m + 0x1d8, lod.as_mut_ptr() as usize);
            exe::wr::<usize>(m + 0x1e8, rad.as_mut_ptr() as usize);
        }

        exe::wr::<usize>(CULL_MGR_PTR_VA as usize, mgr_a.as_mut_ptr() as usize);
        o(0, count);

        exe::wr::<usize>(CULL_MGR_PTR_VA as usize, mgr_b.as_mut_ptr() as usize);
        n(0, count);

        for i in 0..count {
            difftest::eq(
                &format!("dist nc={num_cams} i={i}"),
                dist_a[i].to_bits(),
                dist_b[i].to_bits(),
            )?;
            difftest::eq(
                &format!("rad nc={num_cams} i={i}"),
                rad_a[i].to_bits(),
                rad_b[i].to_bits(),
            )?;
            difftest::eq(
                &format!("lod nc={num_cams} i={i}"),
                lod_a[i].to_bits(),
                lod_b[i].to_bits(),
            )?;
        }
        cases += 1;
    }

    exe::wr::<usize>(CULL_MGR_PTR_VA as usize, saved_mgr);
    exe::wr::<usize>(CAM_VIEWPOINTS_PTR_VA as usize, saved_cams);
    exe::wr::<u32>(CAM_VIEWPOINTS_COUNT_VA as usize, saved_ncams);
    exe::wr::<f32>(CAM_FOV_RAD_VA as usize, saved_fov);
    exe::wr::<i32>(CAM_RES_SCALE_A_VA as usize, saved_sa);
    exe::wr::<i32>(CAM_RES_SCALE_B_VA as usize, saved_sb);
    cases += super::par_tests::big_compute_submesh_camera_lod_factors(orig, new)?;
    Ok(cases)
}
