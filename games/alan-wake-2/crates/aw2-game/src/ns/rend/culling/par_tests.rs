//! Large randomized difftest cases for the cull loops that have an `AW2_PAR_CULL` parallel
//! path: original vs native serial vs native parallel (pool forced on; once with the caller
//! idle so the helper threads run every chunk, once with the caller participating).

use std::sync::atomic::Ordering;

use super::par::{ForceLanes, HELPER_CHUNKS, PAR_TASKS};
use super::CULL_MGR_PTR_VA;
use crate::difftest::{Rng, Snapshot};
use crate::exe::{self, wr};

const SKINNING_MGR_PTR_VA: u64 = 0x1_4397_fed0;
const CAM_VIEWPOINTS_PTR_VA: u64 = 0x1_43bb_ddd8;
const CAM_VIEWPOINTS_COUNT_VA: u64 = 0x1_43bb_dde0;
const CAM_FOV_RAD_VA: u64 = 0x1_4384_170c;
const CAM_RES_SCALE_A_VA: u64 = 0x1_43a3_395c;
const CAM_RES_SCALE_B_VA: u64 = 0x1_4386_c660;
const CAM_STATE_PTR_VA: u64 = 0x1_43a0_01d0;

/// Restores exe globals when dropped.
struct Restore(Vec<Snapshot>);
impl Restore {
    unsafe fn globals(list: &[(u64, usize)]) -> Restore {
        Restore(list.iter().map(|&(va, len)| Snapshot::take(exe::ptr::<u8>(va) as *mut u8, len)).collect())
    }
}
impl Drop for Restore {
    fn drop(&mut self) {
        for s in &self.0 {
            unsafe { s.restore() };
        }
    }
}

fn f32r(rng: &mut Rng, lo: f32, hi: f32) -> f32 {
    lo + (hi - lo) * (rng.below(1 << 20) as f32 / (1u32 << 20) as f32)
}

fn fill_f32(rng: &mut Rng, buf: &mut [u8], lo: f32, hi: f32) {
    for ch in buf.chunks_exact_mut(4) {
        ch.copy_from_slice(&f32r(rng, lo, hi).to_le_bytes());
    }
}

fn first_diff(a: &[u8], b: &[u8]) -> Option<usize> {
    if a.len() != b.len() {
        return Some(a.len().min(b.len()));
    }
    a.iter().zip(b).position(|(x, y)| x != y)
}

/// `run(f)` resets the inputs/outputs, calls `f` and returns the outputs. Compares the
/// original with the native function run serially and through the forced pool.
unsafe fn compare3(name: &str, o: usize, n: usize, mut run: impl FnMut(usize) -> Vec<u8>) -> Result<u32, String> {
    let a = run(o);
    let b = run(n);
    if let Some(at) = first_diff(&a, &b) {
        return Err(format!("{name}: orig vs native serial differ at output byte {at:#x}"));
    }
    let (t0, h0) = (PAR_TASKS.load(Ordering::SeqCst), HELPER_CHUNKS.load(Ordering::SeqCst));
    let _ = super::par::take_stall();
    let c = {
        let _f = ForceLanes::new(4, true);
        run(n)
    };
    if let Some(s) = super::par::take_stall() {
        return Err(format!("{name}: parallel (helpers only): {s}"));
    }
    let (t1, h1) = (PAR_TASKS.load(Ordering::SeqCst), HELPER_CHUNKS.load(Ordering::SeqCst));
    if t1 == t0 || h1 == h0 {
        return Err(format!("{name}: parallel path not taken (pool tasks {t0}->{t1}, helper chunks {h0}->{h1})"));
    }
    if let Some(at) = first_diff(&a, &c) {
        return Err(format!("{name}: orig vs native parallel (helpers only) differ at output byte {at:#x}"));
    }
    let d = {
        let _f = ForceLanes::new(4, false);
        run(n)
    };
    if let Some(at) = first_diff(&a, &d) {
        return Err(format!("{name}: orig vs native parallel (caller + helpers) differ at output byte {at:#x}"));
    }
    Ok(1)
}

/// `0x142261520` on a few thousand instances covering every bounds path (mesh, `+0x4000`
/// instance-local, skinned/custom via the submesh LOD entry with and without bone matrices).
pub(super) unsafe fn big_update_dirty_instance_bounds(o: usize, n: usize) -> Result<u32, String> {
    let _r = Restore::globals(&[(CULL_MGR_PTR_VA, 8), (SKINNING_MGR_PTR_VA, 8)]);
    let mut rng = Rng::new(0x1422_6152_0bb0);
    let mut cases = 0;
    for round in 0..3u32 {
        let n_inst = 3000 + rng.below(200) as usize + round as usize * 1111;
        let nw = n_inst.div_ceil(64);

        // Meshes (also used as LOD info / std_14222f750 sub objects).
        let mut meshes: Vec<Vec<u8>> = (0..8).map(|_| vec![0u8; 0x300]).collect();
        let mut elems: Vec<Vec<u8>> = (0..8).map(|_| vec![0u8; 16 * 16]).collect();
        for (m, el) in meshes.iter_mut().zip(elems.iter_mut()) {
            fill_f32(&mut rng, el, -3.0, 3.0);
            for k in 0..16 {
                let r = f32r(&mut rng, -1.0, 3.0);
                el[k * 16 + 12..k * 16 + 16].copy_from_slice(&r.to_le_bytes());
            }
            let mp = m.as_mut_ptr() as usize;
            wr::<usize>(mp + 0x1a0, el.as_ptr() as usize);
            wr::<u32>(mp + 0x1e8, rng.below(5) as u32);
            for l in 0..4 {
                wr::<i32>(mp + 0x1c8 + l * 4, if rng.below(4) == 0 { 0 } else { rng.below(5) as i32 });
            }
            wr::<u32>(mp + 0x34, rng.below(2) as u32);
            wr::<u8>(mp + 0x270, rng.below(5) as u8);
            wr::<u8>(mp + 0x272, rng.below(5) as u8);
            fill_f32(&mut rng, &mut m[0x274..0x29c], -4.0, 4.0);
            if rng.below(6) == 0 {
                // Inverted box (max < min on all three axes): passed through untransformed.
                for k in 0..3 {
                    let lo = f32r(&mut rng, 1.0, 2.0);
                    wr::<f32>(mp + 0x284 + k * 4, lo);
                    wr::<f32>(mp + 0x290 + k * 4, -lo);
                }
            }
        }
        let mesh_ptrs: Vec<usize> = meshes.iter().map(|m| m.as_ptr() as usize).collect();

        // Instances.
        let vt_plain = vec![0u64; 32];
        let mut vt_custom = vec![0u64; 32];
        vt_custom[0xc8 / 8] = 0x1234;
        let mut insts: Vec<Vec<u8>> = (0..16).map(|_| vec![0u8; 0x100]).collect();
        for it in insts.iter_mut() {
            let ip = it.as_mut_ptr() as usize;
            fill_f32(&mut rng, it, -4.0, 4.0);
            wr::<usize>(ip, (if rng.below(3) == 0 { vt_custom.as_ptr() } else { vt_plain.as_ptr() }) as usize);
            wr::<usize>(ip + 0x8, mesh_ptrs[rng.below(8) as usize]);
            wr::<u32>(ip + 0x48, rng.below(64) as u32);
            wr::<u32>(ip + 0x4c, (if rng.below(3) == 0 { 0x4000u32 } else { 0 }) | rng.below(0x100) as u32);
            wr::<i32>(ip + 0x50, if rng.below(4) == 0 { -1 } else { rng.below(16) as i32 });
        }

        // Scene / skinning manager (0x14397fed0): id map +0x100, instance table +0x120, +0xd0.
        let mut scene = vec![0u8; 0x200];
        let id_map: Vec<i32> = (0..16).map(|i| if rng.below(4) == 0 { -1 } else { i }).collect();
        let mut inst_table = vec![0u8; 16 * 0xa0];
        for e in 0..16 {
            wr::<u32>(inst_table.as_mut_ptr() as usize + e * 0xa0 + 0x98, rng.below(8) as u32);
            wr::<usize>(inst_table.as_mut_ptr() as usize + e * 0xa0 + 0x90, mesh_ptrs[rng.below(8) as usize]);
        }
        let arr_d0: Vec<u32> = (0..8).map(|_| rng.below(9) as u32).collect();
        let sp = scene.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x100, id_map.as_ptr() as usize);
        wr::<usize>(sp + 0x120, inst_table.as_ptr() as usize);
        wr::<usize>(sp + 0xd0, arr_d0.as_ptr() as usize);

        // Per-instance streams.
        let inst_ptrs: Vec<usize> = (0..n_inst).map(|_| insts[rng.below(16) as usize].as_ptr() as usize).collect();
        let mesh_idx: Vec<i32> = (0..n_inst).map(|_| if rng.below(10) == 0 { -1 } else { rng.below(8) as i32 }).collect();
        let mut transforms = vec![0u8; n_inst * 48];
        fill_f32(&mut rng, &mut transforms, -2.0, 2.0);
        let mut dirty: Vec<u64> = (0..nw)
            .map(|w| match w % 7 {
                0 => 0,
                1 => u64::MAX,
                _ => rng.next_u64() & rng.next_u64().rotate_left(7) | rng.next_u64() & 0xf0f0,
            })
            .collect();
        // No dirty bits past the instance count (they would index past every stream).
        if n_inst % 64 != 0 {
            dirty[nw - 1] &= (1u64 << (n_inst % 64)) - 1;
        }
        let lod_arr: Vec<u8> = (0..64).map(|_| rng.below(6) as u8).collect();
        let mut bone_mats = vec![0u8; 64 * 0x30];
        fill_f32(&mut rng, &mut bone_mats, -2.0, 2.0);

        // Outputs (initial contents random).
        let mut out_init = vec![0u8; nw * 8 + n_inst * 16 + n_inst * 24 + 8];
        for b in out_init.iter_mut() {
            *b = rng.next_u32() as u8;
        }
        let mut out = out_init.clone();
        let vis = out.as_mut_ptr() as usize;
        let sph = vis + nw * 8;
        let aabb = sph + n_inst * 16;

        // `this` doubles as the global cull manager (+0x1a8 LOD bytes, +0x5a8 bone entries).
        let mut mgr = vec![0u8; 0x600];
        let m = mgr.as_mut_ptr() as usize;
        wr::<usize>(m + 0x18, inst_ptrs.as_ptr() as usize);
        wr::<usize>(m + 0x38, dirty.as_ptr() as usize);
        wr::<u64>(m + 0x48, n_inst as u64);
        wr::<usize>(m + 0xe8, sph);
        wr::<usize>(m + 0xf8, vis);
        wr::<usize>(m + 0x120, aabb);
        wr::<usize>(m + 0x198, mesh_idx.as_ptr() as usize);
        wr::<usize>(m + 0x1a8, lod_arr.as_ptr() as usize);
        wr::<usize>(m + 0x290, mesh_ptrs.as_ptr() as usize);
        wr::<usize>(m + 0x5a8, bone_mats.as_ptr() as usize);
        wr::<usize>(m + 0x5c8, transforms.as_ptr() as usize);
        wr::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize, m);
        wr::<usize>(exe::ptr::<usize>(SKINNING_MGR_PTR_VA) as usize, sp);

        let outp = out.as_mut_ptr();
        let len = out.len();
        cases += compare3(&format!("update_dirty_instance_bounds round {round} n={n_inst}"), o, n, |f| {
            std::ptr::copy_nonoverlapping(out_init.as_ptr(), outp, len);
            let f: unsafe extern "win64" fn(*mut u8) = std::mem::transmute(f);
            f(m as *mut u8);
            std::slice::from_raw_parts(outp, len).to_vec()
        })?;
        drop(out);
    }
    Ok(cases)
}

/// `0x142259da0` on ~20k spheres for 0..5 cameras.
pub(super) unsafe fn big_compute_submesh_camera_lod_factors(o: usize, n: usize) -> Result<u32, String> {
    let _r = Restore::globals(&[
        (CULL_MGR_PTR_VA, 8),
        (CAM_VIEWPOINTS_PTR_VA, 8),
        (CAM_VIEWPOINTS_COUNT_VA, 4),
        (CAM_FOV_RAD_VA, 4),
        (CAM_RES_SCALE_A_VA, 4),
        (CAM_RES_SCALE_B_VA, 4),
    ]);
    let mut rng = Rng::new(0x1422_59da_0bb0);
    let mut cases = 0;
    let mut cams = vec![0u8; 5 * 0x14];
    for c in 0..5 {
        let cp = cams.as_mut_ptr() as usize + c * 0x14;
        for k in 0..3 {
            wr::<f32>(cp + k * 4, f32r(&mut rng, -50.0, 50.0));
        }
        wr::<f32>(cp + 0xc, f32r(&mut rng, 100.0, 400.0));
        wr::<f32>(cp + 0x10, f32r(&mut rng, 0.1, 2.0));
    }
    wr::<usize>(exe::ptr::<usize>(CAM_VIEWPOINTS_PTR_VA) as usize, cams.as_ptr() as usize);
    wr::<f32>(exe::ptr::<f32>(CAM_FOV_RAD_VA) as usize, 1.0471976);
    wr::<i32>(exe::ptr::<i32>(CAM_RES_SCALE_A_VA) as usize, 1080);
    wr::<i32>(exe::ptr::<i32>(CAM_RES_SCALE_B_VA) as usize, 1920);
    for num_cams in [1u32, 2, 3, 4, 5, 0] {
        wr::<u32>(exe::ptr::<u32>(CAM_VIEWPOINTS_COUNT_VA) as usize, num_cams);
        let count = 20_000 + rng.below(64) as usize;
        let mut spheres = vec![0u8; count * 16];
        fill_f32(&mut rng, &mut spheres, -300.0, 300.0);
        for i in 0..count {
            let r = f32r(&mut rng, 0.05, 30.0);
            spheres[i * 16 + 12..i * 16 + 16].copy_from_slice(&r.to_le_bytes());
        }
        let mut out_init = vec![0u8; count * 12];
        fill_f32(&mut rng, &mut out_init, -1.0, 1.0);
        let mut out = out_init.clone();
        let dist = out.as_mut_ptr() as usize;
        let lod = dist + count * 4;
        let rad = lod + count * 4;
        let mut mgr = vec![0u8; 0x240];
        let m = mgr.as_mut_ptr() as usize;
        wr::<usize>(m + 0xe8, spheres.as_ptr() as usize);
        wr::<usize>(m + 0x1c8, dist);
        wr::<usize>(m + 0x1d8, lod);
        wr::<usize>(m + 0x1e8, rad);
        wr::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize, m);
        let outp = out.as_mut_ptr();
        let len = out.len();
        cases += compare3(&format!("compute_submesh_camera_lod_factors cams={num_cams} n={count}"), o, n, |f| {
            std::ptr::copy_nonoverlapping(out_init.as_ptr(), outp, len);
            let f: unsafe extern "win64" fn(usize, usize) = std::mem::transmute(f);
            f(0, count);
            std::slice::from_raw_parts(outp, len).to_vec()
        })?;
        drop(out);
    }
    Ok(cases)
}

/// `0x142259370` on ~20k submeshes (counts with and without a partial 32/64 tail), with and
/// without a camera.
pub(super) unsafe fn big_evaluate_submesh_lod_and_culling(o: usize, n: usize) -> Result<u32, String> {
    let _r = Restore::globals(&[(CULL_MGR_PTR_VA, 8), (CAM_STATE_PTR_VA, 8)]);
    let mut rng = Rng::new(0x1422_5937_0bb0);
    let mut cases = 0;
    let mut cam = vec![0u8; 0xd0];
    let cp = cam.as_mut_ptr() as usize;
    let dir = [0.48f64, -0.6, 0.64];
    for k in 0..3 {
        wr::<f64>(cp + 0xa0 + k * 8, dir[k]);
        wr::<f64>(cp + 0xb8 + k * 8, (rng.below(2000) as f64 - 1000.0) * 0.05);
    }
    let mut descs = vec![0u8; 64 * 0x24];
    for d in 0..64 {
        let dp = descs.as_mut_ptr() as usize + d * 0x24;
        wr::<u8>(dp + 1, rng.below(4) as u8);
        wr::<u8>(dp + 2, 2 + rng.below(5) as u8);
        let mut t = 0.0f32;
        for b in 0..8 {
            t += f32r(&mut rng, 0.0, 60.0);
            wr::<f32>(dp + 4 + b * 4, t);
        }
    }
    for (round, count) in [20_000usize + 37, 16_384, 20_000 + 96 + 5, 24_576 + 32].into_iter().enumerate() {
        wr::<usize>(exe::ptr::<usize>(CAM_STATE_PTR_VA) as usize, if round == 2 { 0 } else { cp });
        let inst_idx: Vec<u32> = (0..count).map(|_| if rng.below(10) == 0 { u32::MAX } else { rng.below(64) as u32 }).collect();
        let mut dists = vec![0u8; count * 4];
        fill_f32(&mut rng, &mut dists, 0.0, 400.0);
        let clamps: Vec<u16> = (0..count).map(|_| (rng.below(8) as u16) | (rng.below(8) as u16) << 8).collect();
        let mut spheres = vec![0u8; count * 16];
        fill_f32(&mut rng, &mut spheres, -60.0, 60.0);
        let mut da = vec![0u8; count * 4];
        let mut db = vec![0u8; count * 4];
        fill_f32(&mut rng, &mut da, 0.0, 10.0);
        fill_f32(&mut rng, &mut db, 0.0, 10.0);
        for i in (0..count).step_by(9) {
            db[i * 4..i * 4 + 4].copy_from_slice(&da[i * 4..i * 4 + 4].to_vec());
        }
        let prev_lod: Vec<u8> = (0..count).map(|_| if rng.below(5) == 0 { 0xff } else { rng.below(8) as u8 }).collect();
        let active: Vec<u32> = (0..count / 32 + 2).map(|w| if w % 5 == 0 { u32::MAX } else { rng.next_u32() }).collect();

        let nq = count / 64 + 2;
        let mut out_init = vec![0u8; count + count * 12 + nq * 8];
        for b in out_init.iter_mut() {
            *b = rng.next_u32() as u8;
        }
        let mut out = out_init.clone();
        let out_lod = out.as_mut_ptr() as usize;
        let stream = out_lod + count;
        let dirty = stream + count * 12;

        let mut mgr = vec![0u8; 0x300];
        let m = mgr.as_mut_ptr() as usize;
        wr::<usize>(m, active.as_ptr() as usize);
        wr::<usize>(m + 0x28, stream);
        wr::<usize>(m + 0xe8, spheres.as_ptr() as usize);
        wr::<usize>(m + 0x180, dirty);
        wr::<usize>(m + 0x198, inst_idx.as_ptr() as usize);
        wr::<usize>(m + 0x1a8, out_lod);
        wr::<usize>(m + 0x1b8, prev_lod.as_ptr() as usize);
        wr::<usize>(m + 0x1c8, dists.as_ptr() as usize);
        wr::<usize>(m + 0x1e8, da.as_ptr() as usize);
        wr::<usize>(m + 0x1f8, db.as_ptr() as usize);
        wr::<usize>(m + 0x208, clamps.as_ptr() as usize);
        wr::<usize>(m + 0x240, descs.as_ptr() as usize);
        wr::<usize>(exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize, m);
        let outp = out.as_mut_ptr();
        let len = out.len();
        cases += compare3(&format!("evaluate_submesh_lod_and_culling round {round} n={count}"), o, n, |f| {
            std::ptr::copy_nonoverlapping(out_init.as_ptr(), outp, len);
            let f: unsafe extern "win64" fn(usize, usize) = std::mem::transmute(f);
            f(0, count);
            std::slice::from_raw_parts(outp, len).to_vec()
        })?;
        drop(out);
    }
    Ok(cases)
}
