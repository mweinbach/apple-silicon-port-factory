//! Differential tests for `rend::culling`.
#![allow(static_mut_refs)]

use super::*;

// ============================================================================
// 4. Differential tests
// ============================================================================

pub unsafe fn diff_fill_u32_array(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for count in [0usize, 1, 3, 4, 7, 8, 13] {
        for val in [0u32, 0x1234_5678, 0xffff_ffff] {
            let mut buf_a = vec![0xaa_u8; 128];
            let mut buf_b = vec![0xaa_u8; 128];
            let v = val;
            let ra = o(buf_a.as_mut_ptr() as usize, count, &v as *const u32 as usize)
                - buf_a.as_ptr() as usize;
            let rb = n(buf_b.as_mut_ptr() as usize, count, &v as *const u32 as usize)
                - buf_b.as_ptr() as usize;
            crate::diff_eq!(ra, rb, "ret offset count={count} val={val:#x}");
            crate::diff_eq!(buf_a, buf_b, "buffer count={count} val={val:#x}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_peek(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;

    // 1) Empty ring (`+0x30 == +0x34`), stop sentinel, normal packet, consumed packet, wrap packet
    for scenario in 0..5 {
        let mut mem_a = vec![0u8; 128];
        let mut mem_b = vec![0u8; 128];
        let mut ring_a = [0u8; 0x40];
        let mut ring_b = [0u8; 0x40];

        for (m, r) in [
            (mem_a.as_mut_ptr() as usize, ring_a.as_mut_ptr() as usize),
            (mem_b.as_mut_ptr() as usize, ring_b.as_mut_ptr() as usize),
        ] {
            wr::<usize>(r + 0x8, m);
            wr::<usize>(r + 0x10, m + 64);
            wr::<usize>(r + 0x20, m + 16);
            wr::<u32>(r + 0x30, 5);
            wr::<u32>(r + 0x34, if scenario == 0 { 5 } else { 3 });
            wr::<u32>(r + 0x38, 10);

            match scenario {
                1 => wr::<u64>(m + 16, RING_STOP_SENTINEL),
                2 => wr::<u64>(m + 16, 24),
                3 => wr::<u64>(m + 16, RING_CONSUMED_SENTINEL),
                4 => {
                    wr::<u64>(m + 16, RING_WRAP_SENTINEL);
                    wr::<u64>(m, 32);
                }
                _ => {}
            }
        }

        let mut sz_a: u64 = 0xdead;
        let mut sz_b: u64 = 0xdead;
        let ra = o(ring_a.as_mut_ptr() as usize, &mut sz_a as *mut u64 as usize);
        let rb = n(ring_b.as_mut_ptr() as usize, &mut sz_b as *mut u64 as usize);

        let off_a = if ra == 0 { 0 } else { ra - mem_a.as_ptr() as usize };
        let off_b = if rb == 0 { 0 } else { rb - mem_b.as_ptr() as usize };
        crate::diff_eq!(off_a, off_b, "ret offset scenario={scenario}");
        crate::diff_eq!(sz_a, sz_b, "out_size scenario={scenario}");
        crate::diff_eq!(
            rd::<u32>(ring_a.as_ptr() as usize + 0x38),
            rd::<u32>(ring_b.as_ptr() as usize + 0x38),
            "depth scenario={scenario}"
        );
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_consume(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;

    for pkt_sz in [0u64, 16, 40] {
        let mut mem_a = vec![0u8; 128];
        let mut mem_b = vec![0u8; 128];
        let mut ring_a = [0u8; 0x40];
        let mut ring_b = [0u8; 0x40];

        for (m, r) in [
            (mem_a.as_mut_ptr() as usize, ring_a.as_mut_ptr() as usize),
            (mem_b.as_mut_ptr() as usize, ring_b.as_mut_ptr() as usize),
        ] {
            wr::<usize>(r + 0x8, m);
            wr::<usize>(r + 0x10, m + 64);
            wr::<usize>(r + 0x20, m + 16);
            wr::<u32>(r + 0x30, 5);
            wr::<u32>(r + 0x34, if pkt_sz == 0 { 5 } else { 2 });
            wr::<u32>(r + 0x38, 4);
            wr::<u64>(m + 16, pkt_sz);
        }

        o(ring_a.as_mut_ptr() as usize);
        n(ring_b.as_mut_ptr() as usize);

        let rp_a = rd::<usize>(ring_a.as_ptr() as usize + 0x20) - mem_a.as_ptr() as usize;
        let rp_b = rd::<usize>(ring_b.as_ptr() as usize + 0x20) - mem_b.as_ptr() as usize;
        crate::diff_eq!(rp_a, rp_b, "read_ptr offset pkt_sz={pkt_sz}");
        crate::diff_eq!(
            rd::<u32>(ring_a.as_ptr() as usize + 0x34),
            rd::<u32>(ring_b.as_ptr() as usize + 0x34),
            "tail count pkt_sz={pkt_sz}"
        );
        crate::diff_eq!(
            rd::<u32>(ring_a.as_ptr() as usize + 0x38),
            rd::<u32>(ring_b.as_ptr() as usize + 0x38),
            "depth pkt_sz={pkt_sz}"
        );
        crate::diff_eq!(mem_a, mem_b, "mem pkt_sz={pkt_sz}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_renderer_threading_exec_one(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut HANDLER_CALLS: Vec<(usize, u64)> = Vec::new();
    unsafe extern "win64" fn mock_cmd_handler(payload_plus_4: usize, edx_arg: u64) {
        unsafe {
            HANDLER_CALLS.push((rd::<u32>(payload_plus_4) as usize, edx_arg));
        }
    }
    unsafe extern "win64" fn mock_elapsed() -> f64 {
        1.5
    }
    let _m = crate::difftest::Mock::new(0x1_4240_69a0, mock_elapsed as *const ())?;

    let table_slot = exe::ptr::<usize>(CMD_DISPATCH_TABLE_VA) as usize + 3 * 8;
    let orig_handler = rd::<usize>(table_slot);
    wr::<usize>(table_slot, mock_cmd_handler as *const () as usize);

    let mut cases = 0;
    for has_cmd in [false, true] {
        let mut mem_a = vec![0u8; 128];
        let mut mem_b = vec![0u8; 128];
        let mut ring_a = [0u8; 0x40];
        let mut ring_b = [0u8; 0x40];
        let mut this_a = vec![0u8; 0x190];
        let mut this_b = vec![0u8; 0x190];

        for (m, r, t) in [
            (
                mem_a.as_mut_ptr() as usize,
                ring_a.as_mut_ptr() as usize,
                this_a.as_mut_ptr() as usize,
            ),
            (
                mem_b.as_mut_ptr() as usize,
                ring_b.as_mut_ptr() as usize,
                this_b.as_mut_ptr() as usize,
            ),
        ] {
            wr::<usize>(r + 0x8, m);
            wr::<usize>(r + 0x10, m + 96);
            wr::<usize>(r + 0x20, m + 16);
            wr::<u32>(r + 0x30, 4);
            wr::<u32>(r + 0x34, if has_cmd { 3 } else { 4 });
            wr::<u64>(m + 16, 16);
            wr::<u32>(m + 24, (7 << 9) | 3); // opcode 3, arg 7
            wr::<u32>(m + 28, 0x1234);
            wr::<usize>(t + 0x168, r);
            wr::<f64>(t + 0x170, 10.0);
            wr::<u32>(t + 0x178, 5);
        }

        HANDLER_CALLS.clear();
        o(this_a.as_mut_ptr() as usize);
        let ca = HANDLER_CALLS.clone();

        HANDLER_CALLS.clear();
        n(this_b.as_mut_ptr() as usize);
        let cb = HANDLER_CALLS.clone();

        crate::diff_eq!(ca, cb, "handler calls has_cmd={has_cmd}");
        crate::diff_eq!(
            rd::<u32>(this_a.as_ptr() as usize + 0x178),
            rd::<u32>(this_b.as_ptr() as usize + 0x178),
            "cmd count has_cmd={has_cmd}"
        );
        crate::diff_eq!(mem_a, mem_b, "mem has_cmd={has_cmd}");
        cases += 1;
    }

    wr::<usize>(table_slot, orig_handler);
    Ok(cases)
}

pub unsafe fn diff_renderer_threading_main(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut INIT_CALLED: u32 = 0;
    unsafe extern "win64" fn mock_init(_mgr: usize, _kind: u32, _idx: u32) {
        unsafe {
            INIT_CALLED += 1;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4240_6a80, mock_init as *const ())?;

    let holder_va = exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize;
    let orig_holder = rd::<usize>(holder_va);
    let mut fake_holder = [0usize; 4];
    wr::<usize>(holder_va, fake_holder.as_mut_ptr() as usize);

    let mut this_a = vec![0u8; 0x1b0];
    let mut this_b = vec![0u8; 0x1b0];
    wr::<u8>(this_a.as_mut_ptr() as usize + 0x160, 1);
    wr::<u8>(this_b.as_mut_ptr() as usize + 0x160, 1);

    INIT_CALLED = 0;
    let ra = o(this_a.as_mut_ptr() as usize);
    let ia = INIT_CALLED;

    INIT_CALLED = 0;
    let rb = n(this_b.as_mut_ptr() as usize);
    let ib = INIT_CALLED;

    wr::<usize>(holder_va, orig_holder);
    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(ia, ib, "init called");
    Ok(1)
}

pub unsafe fn diff_compute_transformed_bounds(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut world = [0.0f32; 16];
    world[0] = 1.5;
    world[5] = 2.0;
    world[10] = 0.5;
    world[12] = 10.0;
    world[13] = -5.0;
    world[14] = 3.0;
    world[15] = 1.0;

    let elems: [[f32; 4]; 2] = [[1.0, 2.0, 3.0, 4.0], [-2.0, 0.5, 1.5, 2.5]];
    let bones: [[f32; 12]; 2] = [
        [1.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.5],
        [0.5, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 3.0, 0.0, 0.0, 1.5, -2.0],
    ];

    let mut mesh = vec![0u8; 0x200];
    wr::<usize>(mesh.as_mut_ptr() as usize + 0x1a0, elems.as_ptr() as usize);
    wr::<u32>(mesh.as_mut_ptr() as usize + 0x1c8, 2);
    wr::<u32>(mesh.as_mut_ptr() as usize + 0x1e8, 0);

    let mut cases = 0;
    for use_bones in [false, true] {
        let mut aabb_a = [0u8; 32];
        let mut aabb_b = [0u8; 32];
        let mut sph_a = [0u8; 16];
        let mut sph_b = [0u8; 16];
        let bm = if use_bones { bones.as_ptr() as usize } else { 0 };

        o(
            aabb_a.as_mut_ptr() as usize,
            sph_a.as_mut_ptr() as usize,
            mesh.as_ptr() as usize,
            world.as_ptr() as usize,
            bm,
        );
        n(
            aabb_b.as_mut_ptr() as usize,
            sph_b.as_mut_ptr() as usize,
            mesh.as_ptr() as usize,
            world.as_ptr() as usize,
            bm,
        );

        crate::diff_eq!(aabb_a, aabb_b, "aabb use_bones={use_bones}");
        crate::diff_eq!(sph_a, sph_b, "sphere use_bones={use_bones}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_lod_find_threshold_avx(orig: usize, new: usize) -> Result<u32, String> {
    #[cfg(not(target_arch = "aarch64"))]
    type F = unsafe extern "win64" fn(usize, f32) -> u32;
    #[cfg(target_arch = "aarch64")]
    type F = unsafe extern "win64" fn(usize, f32, f32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let arr: [f32; 8] = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let mut cases = 0;
    for v in [0.0f32, 10.0, 15.0, 40.0, 75.0, 90.0] {
        #[cfg(not(target_arch = "aarch64"))]
        let (ra, rb) = (o(arr.as_ptr() as usize, v), n(arr.as_ptr() as usize, v));
        #[cfg(target_arch = "aarch64")]
        let (ra, rb) = (
            o(arr.as_ptr() as usize, 0.0, v),
            n(arr.as_ptr() as usize, 0.0, v),
        );
        crate::diff_eq!(ra, rb, "val={v}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_lod_select_with_hysteresis(orig: usize, new: usize) -> Result<u32, String> {
    #[cfg(not(target_arch = "aarch64"))]
    type F = unsafe extern "win64" fn(usize, f32, f32, f32, i32, i32, i32) -> i32;
    #[cfg(target_arch = "aarch64")]
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, f32, f32, f32, f32, i32, i32, i32) -> i32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let arr: [f32; 8] = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let mut cases = 0;
    for dist in [5.0f32, 18.0, 25.0, 45.0, 95.0] {
        for cur in [0i32, 2, 4] {
            #[cfg(not(target_arch = "aarch64"))]
            let (ra, rb) = (
                o(arr.as_ptr() as usize, dist, 2.0, -2.0, 0, 5, cur),
                n(arr.as_ptr() as usize, dist, 2.0, -2.0, 0, 5, cur),
            );
            #[cfg(target_arch = "aarch64")]
            let (ra, rb) = (
                o(arr.as_ptr() as usize, 0, 0, 0, 0.0, dist, 2.0, -2.0, 0, 5, cur),
                n(arr.as_ptr() as usize, 0, 0, 0, 0.0, dist, 2.0, -2.0, 0, 5, cur),
            );
            crate::diff_eq!(ra, rb, "dist={dist} cur={cur}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_update_instance_lod_levels(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mgr_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let orig_mgr = rd::<usize>(mgr_va);

    let mut descs = [0u8; 0x48];
    // Instance 0: min=0, max=3, cur=1, thresholds=[10, 20, 30, 40]
    descs[0] = 0;
    descs[1] = 3;
    descs[2] = 1;
    wr::<f32>(descs.as_mut_ptr() as usize + 4, 10.0);
    wr::<f32>(descs.as_mut_ptr() as usize + 8, 20.0);
    wr::<f32>(descs.as_mut_ptr() as usize + 12, 30.0);
    wr::<f32>(descs.as_mut_ptr() as usize + 16, 40.0);

    // Instance 1: min=1, max=4, cur=2, thresholds=[15, 25, 35, 45, 55]
    let d1 = descs.as_mut_ptr() as usize + 0x24;
    wr::<u8>(d1, 1);
    wr::<u8>(d1 + 1, 4);
    wr::<u8>(d1 + 2, 2);
    wr::<f32>(d1 + 4, 15.0);
    wr::<f32>(d1 + 8, 25.0);
    wr::<f32>(d1 + 12, 35.0);
    wr::<f32>(d1 + 16, 45.0);
    wr::<f32>(d1 + 20, 55.0);

    let dists: [f32; 2] = [22.0, 5.0];
    let clamps: [u8; 2] = [2, 5];
    let mut out_a = [0u8; 2];
    let mut out_b = [0u8; 2];

    let mut mgr = vec![0u8; 0x300];
    let m = mgr.as_mut_ptr() as usize;
    wr::<usize>(m + 0x240, descs.as_ptr() as usize);
    wr::<u32>(m + 0x248, 2);
    wr::<usize>(m + 0x260, dists.as_ptr() as usize);
    wr::<usize>(m + 0x280, clamps.as_ptr() as usize);

    wr::<usize>(mgr_va, m);
    wr::<usize>(m + 0x2a0, out_a.as_mut_ptr() as usize);
    o();

    wr::<usize>(m + 0x2a0, out_b.as_mut_ptr() as usize);
    n();

    wr::<usize>(mgr_va, orig_mgr);
    crate::diff_eq!(out_a, out_b, "out lods");
    Ok(1)
}

pub unsafe fn diff_aggregate_instance_cull_results(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mgr_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let orig_mgr = rd::<usize>(mgr_va);

    let sub_inst: [u32; 5] = [0, 1, u32::MAX, 0, 1];
    let sub_min: [f32; 5] = [12.0, 8.0, 1.0, 5.0, 20.0];
    let sub_max: [f32; 5] = [15.0, 18.0, 99.0, 30.0, 10.0];
    let sub_lod: [u8; 10] = [0, 3, 0, 2, 0, 0, 0, 1, 0, 4];
    let sub_bit: [u8; 5] = [0, 2, 5, 3, 1];

    let mut min_a = [0.0f32; 2];
    let mut min_b = [0.0f32; 2];
    let mut max_a = [0.0f32; 2];
    let mut max_b = [0.0f32; 2];
    let mut lod_a = [0u8; 2];
    let mut lod_b = [0u8; 2];
    let mut mask_a = [0u32; 2];
    let mut mask_b = [0u32; 2];

    let mut mgr = vec![0u8; 0x300];
    let m = mgr.as_mut_ptr() as usize;
    wr::<usize>(m + 0x198, sub_inst.as_ptr() as usize);
    wr::<u32>(m + 0x1a0, 5);
    wr::<usize>(m + 0x1c8, sub_min.as_ptr() as usize);
    wr::<usize>(m + 0x1d8, sub_max.as_ptr() as usize);
    wr::<usize>(m + 0x208, sub_lod.as_ptr() as usize);
    wr::<usize>(m + 0x218, sub_bit.as_ptr() as usize);
    wr::<u32>(m + 0x268, 2);

    wr::<usize>(mgr_va, m);

    wr::<usize>(m + 0x260, min_a.as_mut_ptr() as usize);
    wr::<usize>(m + 0x270, max_a.as_mut_ptr() as usize);
    wr::<usize>(m + 0x280, lod_a.as_mut_ptr() as usize);
    wr::<usize>(m + 0x2c0, mask_a.as_mut_ptr() as usize);
    o();

    wr::<usize>(m + 0x260, min_b.as_mut_ptr() as usize);
    wr::<usize>(m + 0x270, max_b.as_mut_ptr() as usize);
    wr::<usize>(m + 0x280, lod_b.as_mut_ptr() as usize);
    wr::<usize>(m + 0x2c0, mask_b.as_mut_ptr() as usize);
    n();

    wr::<usize>(mgr_va, orig_mgr);
    crate::diff_eq!(min_a, min_b, "inst min");
    crate::diff_eq!(max_a, max_b, "inst max");
    crate::diff_eq!(lod_a, lod_b, "inst lod");
    crate::diff_eq!(mask_a, mask_b, "inst mask");
    Ok(1)
}

pub unsafe fn diff_update_cull_and_instance_lods(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut AGG_CALLS: u32 = 0;
    unsafe extern "win64" fn mock_agg() {
        unsafe {
            AGG_CALLS += 1;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4225_99f0, mock_agg as *const ())?;

    let mgr_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let orig_mgr = rd::<usize>(mgr_va);
    let mut mgr = vec![0u8; 0x300];
    wr::<usize>(mgr_va, mgr.as_mut_ptr() as usize);

    let mut this_obj = [0u8; 0x30];
    wr::<u32>(this_obj.as_mut_ptr() as usize + 0x20, 1);

    AGG_CALLS = 0;
    o(this_obj.as_ptr() as usize);
    let ca = AGG_CALLS;

    AGG_CALLS = 0;
    n(this_obj.as_ptr() as usize);
    let cb = AGG_CALLS;

    wr::<usize>(mgr_va, orig_mgr);
    crate::diff_eq!(ca, cb, "agg calls");
    Ok(1)
}

pub unsafe fn diff_evaluate_submesh_lod_and_culling(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mgr_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let orig_mgr = rd::<usize>(mgr_va);

    let mut desc = [0u8; 0x24];
    desc[1] = 1; // min_lod
    desc[2] = 4; // max_lod
    for bit in 0..8 {
        wr::<f32>(desc.as_mut_ptr() as usize + 4 + bit * 4, (bit as f32 + 1.0) * 10.0);
    }

    let inst_idx: [u32; 5] = [0, 0, u32::MAX, 0, 0];
    let dists: [f32; 5] = [5.0, 25.0, 10.0, 45.0, 90.0];
    let clamps: [u16; 5] = [0x0104, 0x0005, 0x0000, 0x0203, 0x0104];
    let spheres: [[f32; 4]; 5] = [
        [0.0, 0.0, 10.0, 2.0],
        [0.0, 0.0, -10.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
        [5.0, 5.0, 5.0, 3.0],
        [1.0, 2.0, 3.0, 0.5],
    ];
    let dist_a: [f32; 5] = [1.0, 10.0, 2.0, 5.0, 8.0];
    let dist_b: [f32; 5] = [2.0, 5.0, 3.0, 5.0, 9.0];
    let prev_lod: [u8; 5] = [1, 0xff, 0, 3, 2];
    let active_mask: [u32; 2] = [0b11111, 0];

    let mut out_lod_a = [0u8; 5];
    let mut out_lod_b = [0u8; 5];
    let mut stream_a = [0u8; 60];
    let mut stream_b = [0u8; 60];
    let mut dirty_a = [0u64; 1];
    let mut dirty_b = [0u64; 1];

    let mut mgr = vec![0u8; 0x300];
    let m = mgr.as_mut_ptr() as usize;
    wr::<usize>(m, active_mask.as_ptr() as usize);
    wr::<usize>(m + 0xe8, spheres.as_ptr() as usize);
    wr::<usize>(m + 0x198, inst_idx.as_ptr() as usize);
    wr::<usize>(m + 0x1b8, prev_lod.as_ptr() as usize);
    wr::<usize>(m + 0x1c8, dists.as_ptr() as usize);
    wr::<usize>(m + 0x1e8, dist_a.as_ptr() as usize);
    wr::<usize>(m + 0x1f8, dist_b.as_ptr() as usize);
    wr::<usize>(m + 0x208, clamps.as_ptr() as usize);
    wr::<usize>(m + 0x240, desc.as_ptr() as usize);

    wr::<usize>(mgr_va, m);

    wr::<usize>(m + 0x28, stream_a.as_mut_ptr() as usize);
    wr::<usize>(m + 0x180, dirty_a.as_mut_ptr() as usize);
    wr::<usize>(m + 0x1a8, out_lod_a.as_mut_ptr() as usize);
    o(0, 5);

    wr::<usize>(m + 0x28, stream_b.as_mut_ptr() as usize);
    wr::<usize>(m + 0x180, dirty_b.as_mut_ptr() as usize);
    wr::<usize>(m + 0x1a8, out_lod_b.as_mut_ptr() as usize);
    n(0, 5);

    wr::<usize>(mgr_va, orig_mgr);
    crate::diff_eq!(out_lod_a, out_lod_b, "out_lod");
    crate::diff_eq!(stream_a, stream_b, "node_stream");
    crate::diff_eq!(dirty_a, dirty_b, "dirty_mask");
    Ok(1 + super::par_tests::big_evaluate_submesh_lod_and_culling(orig, new)?)
}


// ============================================================================
// 6. Differential tests for additional rend::culling functions
// ============================================================================

pub unsafe fn diff_update_all_instance_vis_masks(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut VIS_CALLS: Vec<(usize, u32)> = Vec::new();
    unsafe extern "win64" fn mock_vis(obj: usize, mask: u32) {
        unsafe {
            VIS_CALLS.push((obj, mask));
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4233_0950, mock_vis as *const ())?;

    let objs: [usize; 3] = [0x1000, 0, 0x2000];
    let masks: [u32; 3] = [0x11, 0x22, 0x33];
    let mut mgr = vec![0u8; 0x2d0];
    let m = mgr.as_mut_ptr() as usize;
    wr::<usize>(m + 0x290, objs.as_ptr() as usize);
    wr::<u32>(m + 0x298, 3);
    wr::<usize>(m + 0x2c0, masks.as_ptr() as usize);

    VIS_CALLS.clear();
    o(m);
    let ca = VIS_CALLS.clone();

    VIS_CALLS.clear();
    n(m);
    let cb = VIS_CALLS.clone();

    crate::diff_eq!(ca, cb, "vis calls");
    Ok(1)
}

pub unsafe fn diff_update_instance_vis_masks_post_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut CALLS: Vec<(u32, usize, usize)> = Vec::new();
    unsafe extern "win64" fn mock_vis(obj: usize, mask: usize) {
        unsafe { CALLS.push((1, obj, mask & 0xffff_ffff)) }
    }
    unsafe extern "win64" fn mock_worker() -> u32 {
        unsafe { CALLS.push((2, 0, 0)) };
        3
    }
    static mut FAKE_JOB: [u64; 8] = [0; 8];
    unsafe extern "win64" fn mock_alloc(pool: usize, out: usize) -> usize {
        unsafe {
            CALLS.push((3, pool, 0));
            FAKE_JOB = [0xaaaa_aaaa_0000_0000; 8];
            wr::<usize>(out, FAKE_JOB.as_mut_ptr() as usize);
            wr::<u64>(out + 8, 0x55);
            out
        }
    }
    unsafe extern "win64" fn mock_push(sched: usize, job: usize) {
        unsafe { CALLS.push((4, sched, job)) }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4233_0950, mock_vis as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6460, mock_push as *const ())?;

    let holder_va = exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize;
    let handle_va = exe::ptr::<u64>(0x1_4397_ff38) as usize;
    let orig_holder = rd::<usize>(holder_va);
    let orig_handle = (rd::<u64>(handle_va), rd::<u64>(handle_va + 8));
    let mut fake_q = vec![0u8; 0x1020];
    let mut fake_sched = vec![0u8; 0x200];
    let fs = fake_sched.as_mut_ptr() as usize;
    wr::<usize>(fs, fake_q.as_mut_ptr() as usize);
    wr::<usize>(fs + 0x48 + (3 << 6), 0x7000); // worker 3's pool base
    let mut fake_holder = [0usize; 4];
    fake_holder[2] = fs; // +0x10
    wr::<usize>(holder_va, fake_holder.as_mut_ptr() as usize);

    let objs: [usize; 5] = [0x1000, 0, 0x2000, 0x3000, 0];
    let masks: [u32; 5] = [0x11, 0x22, 0xffff_ffff, 0, 0x55];
    let mut cases = 0;
    let mut res = Ok(());
    'outer: for count in [0u32, 1, 5] {
        // The job's state word is 0 before the xchg and nothing completes it concurrently,
        // so both sides always see 1 afterwards and push it.
        for misalign in [0usize, 4] {
            let mut mgr = vec![0u8; 0x2d8];
            let m = mgr.as_mut_ptr() as usize + misalign;
            wr::<usize>(m + 0x290, objs.as_ptr() as usize);
            wr::<u32>(m + 0x298, count);
            wr::<usize>(m + 0x2c0, masks.as_ptr() as usize);

            let run = |f: F| {
                CALLS.clear();
                wr::<u64>(handle_va, 0);
                wr::<u64>(handle_va + 8, 0);
                f(m);
                let mut job = FAKE_JOB;
                job[3] &= !0xff; // +0x18: uninitialized empty-lambda byte in the original
                (CALLS.clone(), job, rd::<u64>(handle_va), rd::<u64>(handle_va + 8))
            };
            let a = run(o);
            let b = run(n);
            if a != b {
                res = Err(format!("count={count} misalign={misalign}: orig {a:x?} new {b:x?}"));
                break 'outer;
            }
            cases += 1;
        }
    }

    wr::<usize>(holder_va, orig_holder);
    wr::<u64>(handle_va, orig_handle.0);
    wr::<u64>(handle_va + 8, orig_handle.1);
    res?;
    Ok(cases)
}

pub unsafe fn diff_evaluate_culling_for_manager(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut SEQ: Vec<(u32, usize)> = Vec::new();
    unsafe extern "win64" fn m1(_rcx: usize, count: usize) {
        unsafe {
            SEQ.push((1, count));
        }
    }
    unsafe extern "win64" fn m2(_rcx: usize, count: usize) {
        unsafe {
            SEQ.push((2, count));
        }
    }
    let _g1 = crate::difftest::Mock::new(0x1_4225_9da0, m1 as *const ())?;
    let _g2 = crate::difftest::Mock::new(0x1_4225_9370, m2 as *const ())?;

    let mut this_obj = [0u8; 0x30];
    wr::<u32>(this_obj.as_mut_ptr() as usize + 0x20, 7);

    SEQ.clear();
    o(this_obj.as_ptr() as usize);
    let sa = SEQ.clone();

    SEQ.clear();
    n(this_obj.as_ptr() as usize);
    let sb = SEQ.clone();

    crate::diff_eq!(sa, sb, "cull sequence");
    Ok(1)
}

pub unsafe fn diff_update_cull_lod_thresholds(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut THRESH_CALLS: Vec<(usize, usize, u32, u32)> = Vec::new();
    #[cfg(not(target_arch = "aarch64"))]
    unsafe extern "win64" fn mock_thresh(obj: usize, out: usize, s: f32, fov: f32) {
        unsafe {
            THRESH_CALLS.push((obj, out, s.to_bits(), fov.to_bits()));
        }
    }
    #[cfg(target_arch = "aarch64")]
    unsafe extern "win64" fn mock_thresh(
        obj: usize,
        out: usize,
        _xmm0: f32,
        _xmm1: f32,
        s: f32,
        fov: f32,
    ) {
        unsafe {
            THRESH_CALLS.push((obj, out, s.to_bits(), fov.to_bits()));
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4233_0d80, mock_thresh as *const ())?;

    let mut obj0 = vec![0u8; 0x290];
    wr::<f32>(obj0.as_mut_ptr() as usize + 0x280, 2.5);
    let objs: [usize; 2] = [obj0.as_ptr() as usize, 0];
    let descs = [0u8; 0x48];
    let mut dirty_a: [u64; 1] = [1];
    let mut dirty_b: [u64; 1] = [1];

    let mut mgr = vec![0u8; 0x2b0];
    let m = mgr.as_mut_ptr() as usize;
    wr::<u64>(m + 0x238, 2);
    wr::<usize>(m + 0x240, descs.as_ptr() as usize);
    wr::<u32>(m + 0x248, 2);
    wr::<usize>(m + 0x290, objs.as_ptr() as usize);

    // 1) Full rebuild path (mismatch cached mode)
    let cm_va = exe::ptr::<u32>(CACHED_LOD_MODE_VA) as usize;
    let orig_cm = rd::<u32>(cm_va);

    wr::<u32>(cm_va, 0xdead_beef);
    wr::<usize>(m + 0x228, dirty_a.as_mut_ptr() as usize);
    THRESH_CALLS.clear();
    o(m);
    let ca_full = THRESH_CALLS.clone();

    wr::<u32>(cm_va, 0xdead_beef);
    wr::<usize>(m + 0x228, dirty_b.as_mut_ptr() as usize);
    THRESH_CALLS.clear();
    n(m);
    let cb_full = THRESH_CALLS.clone();
    crate::diff_eq!(ca_full, cb_full, "full rebuild calls");

    // 2) Incremental path (cached state matches)
    dirty_a[0] = 1;
    dirty_b[0] = 1;
    wr::<usize>(m + 0x228, dirty_a.as_mut_ptr() as usize);
    THRESH_CALLS.clear();
    o(m);
    let ca_inc = THRESH_CALLS.clone();

    wr::<usize>(m + 0x228, dirty_b.as_mut_ptr() as usize);
    THRESH_CALLS.clear();
    n(m);
    let cb_inc = THRESH_CALLS.clone();

    wr::<u32>(cm_va, orig_cm);
    crate::diff_eq!(ca_inc, cb_inc, "incremental calls");
    crate::diff_eq!(dirty_a, dirty_b, "dirty cleared");
    Ok(2)
}

pub unsafe fn diff_free_submesh_range_at_590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ARGS: (usize, u32, u32) = (0, 0, 0);
    unsafe extern "win64" fn mock_free(p: usize, s: u32, c: u32) {
        unsafe {
            ARGS = (p, s, c);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4222_9c90, mock_free as *const ())?;

    o(0x1000, 4, 8);
    let a = ARGS;
    n(0x1000, 4, 8);
    let b = ARGS;
    crate::diff_eq!(a, b, "forwarded args");
    Ok(1)
}

pub unsafe fn diff_alloc_submesh_range_at_590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut MOCK_FOUND: u32 = 0;
    static mut FREE_SLOT: [u32; 2] = [0, 0];
    unsafe extern "win64" fn mock_find(_p: usize, _c: u32) -> u32 {
        unsafe { MOCK_FOUND }
    }
    unsafe extern "win64" fn mock_grow(_vec: usize, _val: usize, _cnt: u32) {}
    unsafe extern "win64" fn mock_push(_vec: usize) -> usize {
        unsafe { FREE_SLOT.as_mut_ptr() as usize }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4222_9a00, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412f_8ca0, mock_grow as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_8d90, mock_push as *const ())?;

    let mut cases = 0;
    for found in [12u32, u32::MAX] {
        MOCK_FOUND = found;
        let mut a = vec![0u8; 0x600];
        let mut b = vec![0u8; 0x600];
        wr::<u32>(a.as_mut_ptr() as usize + 0x590, 100);
        wr::<u32>(b.as_mut_ptr() as usize + 0x590, 100);

        FREE_SLOT = [0, 0];
        let ra = o(a.as_mut_ptr() as usize, 50);
        let fa = FREE_SLOT;

        FREE_SLOT = [0, 0];
        let rb = n(b.as_mut_ptr() as usize, 50);
        let fb = FREE_SLOT;

        crate::diff_eq!(ra, rb, "ret found={found}");
        crate::diff_eq!(fa, fb, "free_slot found={found}");
        crate::diff_eq!(a, b, "state found={found}");
        cases += 1;
    }
    Ok(cases)
}

unsafe fn run_cull_job_difftest(orig: usize, new: usize, outer_offset: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    unsafe extern "win64" fn mock_nop1(_a: usize) {}
    unsafe extern "win64" fn mock_nop2(_a: usize, _b: usize) {}
    unsafe extern "win64" fn mock_worker() -> u32 {
        0
    }
    static mut FAKE_JOB: [u64; 8] = [0; 8];
    unsafe extern "win64" fn mock_alloc(_pool: usize, out: usize) -> usize {
        unsafe {
            wr::<usize>(out, FAKE_JOB.as_mut_ptr() as usize);
            wr::<u64>(out + 8, 0x55);
            out
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4226_2860, mock_nop1 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_9da0, mock_nop2 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4225_9370, mock_nop2 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4248_6460, mock_nop2 as *const ())?;

    let holder_va = exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize;
    let orig_holder = rd::<usize>(holder_va);
    let mut fake_q = vec![0u8; 0x1020];
    let mut fake_sched = vec![0u8; 0x100];
    wr::<usize>(fake_sched.as_mut_ptr() as usize, fake_q.as_mut_ptr() as usize);
    let mut fake_holder = [0usize; 4];
    fake_holder[2] = fake_sched.as_mut_ptr() as usize; // +0x10
    wr::<usize>(holder_va, fake_holder.as_mut_ptr() as usize);

    let mut cull_mgr = vec![0u8; 0x40];
    wr::<u32>(cull_mgr.as_mut_ptr() as usize + 0x20, 3);
    let mut outer = vec![0u8; 0x200];
    wr::<usize>(outer.as_mut_ptr() as usize + 0x1a8, cull_mgr.as_mut_ptr() as usize);

    let mut cnt_a: i32 = 5;
    let mut cnt_b: i32 = 5;
    let mut job_this_a = [0u8; 0x20];
    let mut job_this_b = [0u8; 0x20];
    wr::<usize>(
        job_this_a.as_mut_ptr() as usize + outer_offset - 8,
        &mut cnt_a as *mut i32 as usize,
    );
    wr::<usize>(
        job_this_b.as_mut_ptr() as usize + outer_offset - 8,
        &mut cnt_b as *mut i32 as usize,
    );
    wr::<usize>(
        job_this_a.as_mut_ptr() as usize + outer_offset,
        outer.as_mut_ptr() as usize,
    );
    wr::<usize>(
        job_this_b.as_mut_ptr() as usize + outer_offset,
        outer.as_mut_ptr() as usize,
    );

    FAKE_JOB = [0; 8];
    o(job_this_a.as_mut_ptr() as usize);
    let job_a = FAKE_JOB;

    FAKE_JOB = [0; 8];
    n(job_this_b.as_mut_ptr() as usize);
    let job_b = FAKE_JOB;

    wr::<usize>(holder_va, orig_holder);
    crate::diff_eq!(cnt_a, cnt_b, "counter");
    crate::diff_eq!(job_a, job_b, "job struct");
    Ok(1)
}

pub unsafe fn diff_render_world_cull_job_10(orig: usize, new: usize) -> Result<u32, String> {
    run_cull_job_difftest(orig, new, 0x10)
}

pub unsafe fn diff_render_world_cull_job_08(orig: usize, new: usize) -> Result<u32, String> {
    run_cull_job_difftest(orig, new, 0x8)
}

pub unsafe fn diff_is_submesh_lod_visible(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let scene_va = exe::ptr::<usize>(SCENE_INST_MGR_PTR_VA) as usize;
    let cull_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let (orig_scene, orig_cull) = (rd::<usize>(scene_va), rd::<usize>(cull_va));

    let id_map: [u32; 2] = [0, u32::MAX];
    let mut scene_mgr = vec![0u8; 0x140];
    wr::<usize>(scene_mgr.as_mut_ptr() as usize + 0x100, id_map.as_ptr() as usize);

    let lods: [u8; 2] = [1, 0];
    let mut cull_mgr = vec![0u8; 0x200];
    wr::<usize>(cull_mgr.as_mut_ptr() as usize + 0x1a8, lods.as_ptr() as usize);

    let mut mesh = vec![0u8; 0x280];
    wr::<u8>(mesh.as_mut_ptr() as usize + 0x272, 2);
    wr::<i32>(mesh.as_mut_ptr() as usize + 0x1c8, 0);
    wr::<i32>(mesh.as_mut_ptr() as usize + 0x1cc, 4);

    wr::<usize>(scene_va, scene_mgr.as_ptr() as usize);
    wr::<usize>(cull_va, cull_mgr.as_ptr() as usize);

    let mut cases = 0;
    for (inst_id, sub_idx) in [(u32::MAX, 0u32), (1, 0), (0, 0), (0, 1)] {
        let mut sub = [0u8; 0x60];
        wr::<usize>(sub.as_mut_ptr() as usize + 0x8, mesh.as_ptr() as usize);
        wr::<u32>(sub.as_mut_ptr() as usize + 0x48, sub_idx);
        wr::<u32>(sub.as_mut_ptr() as usize + 0x50, inst_id);

        let ra = o(sub.as_ptr() as usize) & 1;
        let rb = n(sub.as_ptr() as usize) & 1;
        crate::diff_eq!(ra, rb, "inst={inst_id} sub={sub_idx}");
        cases += 1;
    }

    wr::<usize>(scene_va, orig_scene);
    wr::<usize>(cull_va, orig_cull);
    Ok(cases)
}

pub unsafe fn diff_get_submesh_lod_entry(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    unsafe extern "win64" fn mock_slot(_entry: usize, lod: u32) -> u32 {
        lod + 2
    }
    let _m = crate::difftest::Mock::new(0x1_4222_f750, mock_slot as *const ())?;

    let scene_va = exe::ptr::<usize>(SCENE_INST_MGR_PTR_VA) as usize;
    let cull_va = exe::ptr::<usize>(CULL_MGR_PTR_VA) as usize;
    let (orig_scene, orig_cull) = (rd::<usize>(scene_va), rd::<usize>(cull_va));

    let id_map: [u32; 2] = [0, u32::MAX];
    let mut scene_mgr = vec![0u8; 0x140];
    wr::<usize>(scene_mgr.as_mut_ptr() as usize + 0x100, id_map.as_ptr() as usize);
    wr::<usize>(scene_mgr.as_mut_ptr() as usize + 0x120, 0x5000_0000);

    let lods: [u8; 2] = [1, 0];
    let mut cull_mgr = vec![0u8; 0x600];
    wr::<usize>(cull_mgr.as_mut_ptr() as usize + 0x1a8, lods.as_ptr() as usize);
    wr::<usize>(cull_mgr.as_mut_ptr() as usize + 0x5a8, 0x8000_0000);

    let mut mesh = vec![0u8; 0x280];
    wr::<u8>(mesh.as_mut_ptr() as usize + 0x272, 2);
    wr::<i32>(mesh.as_mut_ptr() as usize + 0x1c8, 0);
    wr::<i32>(mesh.as_mut_ptr() as usize + 0x1cc, 3);

    wr::<usize>(scene_va, scene_mgr.as_ptr() as usize);
    wr::<usize>(cull_va, cull_mgr.as_ptr() as usize);

    let mut cases = 0;
    for (inst_id, sub_idx) in [(u32::MAX, 0u32), (1, 0), (0, 0), (0, 1)] {
        let mut sub = [0u8; 0x60];
        wr::<usize>(sub.as_mut_ptr() as usize + 0x8, mesh.as_ptr() as usize);
        wr::<u32>(sub.as_mut_ptr() as usize + 0x48, sub_idx);
        wr::<u32>(sub.as_mut_ptr() as usize + 0x50, inst_id);

        let ra = o(sub.as_ptr() as usize);
        let rb = n(sub.as_ptr() as usize);
        crate::diff_eq!(ra, rb, "inst={inst_id} sub={sub_idx}");
        cases += 1;
    }

    wr::<usize>(scene_va, orig_scene);
    wr::<usize>(cull_va, orig_cull);
    Ok(cases)
}

pub unsafe fn diff_lod_find_threshold_scalar(orig: usize, new: usize) -> Result<u32, String> {
    #[cfg(not(target_arch = "aarch64"))]
    type F = unsafe extern "win64" fn(usize, f32) -> u32;
    #[cfg(target_arch = "aarch64")]
    type F = unsafe extern "win64" fn(usize, f32, f32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let arr: [f32; 8] = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let mut cases = 0;
    for v in [0.0f32, 10.0, 15.0, 40.0, 75.0, 90.0] {
        #[cfg(not(target_arch = "aarch64"))]
        let (ra, rb) = (o(arr.as_ptr() as usize, v), n(arr.as_ptr() as usize, v));
        #[cfg(target_arch = "aarch64")]
        let (ra, rb) = (
            o(arr.as_ptr() as usize, 0.0, v),
            n(arr.as_ptr() as usize, 0.0, v),
        );
        crate::diff_eq!(ra, rb, "val={v}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_has_packet_at(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for (w, r, hdr) in [(3u32, 3u32, 16u64), (4, 3, RING_STOP_SENTINEL), (4, 3, 24)] {
        let mut ring = [0u8; 0x40];
        wr::<u32>(ring.as_mut_ptr() as usize + 0x30, w);
        wr::<u32>(ring.as_mut_ptr() as usize + 0x34, r);
        let pkt = hdr;
        let ra = o(ring.as_ptr() as usize, &pkt as *const u64 as usize) & 1;
        let rb = n(ring.as_ptr() as usize, &pkt as *const u64 as usize) & 1;
        crate::diff_eq!(ra, rb, "w={w} r={r} hdr={hdr:#x}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_is_empty(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for (w, r, hdr) in [(3u32, 3u32, 16u64), (4, 3, RING_STOP_SENTINEL), (4, 3, 24)] {
        let pkt = hdr;
        let mut ring = [0u8; 0x40];
        wr::<usize>(ring.as_mut_ptr() as usize + 0x20, &pkt as *const u64 as usize);
        wr::<u32>(ring.as_mut_ptr() as usize + 0x30, w);
        wr::<u32>(ring.as_mut_ptr() as usize + 0x34, r);
        let ra = o(ring.as_ptr() as usize) & 1;
        let rb = n(ring.as_ptr() as usize) & 1;
        crate::diff_eq!(ra, rb, "w={w} r={r} hdr={hdr:#x}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_pending_count(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for (w, r) in [(5u32, 5u32), (10, 3), (2, 7)] {
        let mut ring = [0u8; 0x40];
        wr::<u32>(ring.as_mut_ptr() as usize + 0x30, w);
        wr::<u32>(ring.as_mut_ptr() as usize + 0x34, r);
        let ra = o(ring.as_ptr() as usize);
        let rb = n(ring.as_ptr() as usize);
        crate::diff_eq!(ra, rb, "w={w} r={r}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_ring_set_mode_2(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut a = [0u32; 2];
    let mut b = [0u32; 2];
    o(a.as_mut_ptr() as usize);
    n(b.as_mut_ptr() as usize);
    crate::diff_eq!(a, b, "mode");
    Ok(1)
}

pub unsafe fn diff_cmd_ring_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    unsafe extern "win64" fn mock_alloc(_r: usize, _s: usize, _a: usize, _s2: usize) -> usize {
        0x7777_0000
    }
    let _m = crate::difftest::Mock::new(0x1_4230_82d0, mock_alloc as *const ())?;

    let mut a = [0xffu8; 0x40];
    let mut b = [0xffu8; 0x40];
    let ra = o(a.as_mut_ptr() as usize, 0x1000) - a.as_ptr() as usize;
    let rb = n(b.as_mut_ptr() as usize, 0x1000) - b.as_ptr() as usize;
    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(a, b, "ring state");
    Ok(1)
}

pub unsafe fn diff_cmd_ring_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;

    // Test size=0, normal alloc, wrap alloc, and full non-waiting alloc
    for scenario in 0..4 {
        let mut mem_a = vec![0u8; 128];
        let mut mem_b = vec![0u8; 128];
        let mut ring_a = [0u8; 0x40];
        let mut ring_b = [0u8; 0x40];

        for (m, r) in [
            (mem_a.as_mut_ptr() as usize, ring_a.as_mut_ptr() as usize),
            (mem_b.as_mut_ptr() as usize, ring_b.as_mut_ptr() as usize),
        ] {
            wr::<usize>(r + 0x8, m);
            wr::<usize>(r + 0x10, m + 64);
            match scenario {
                0 | 1 => {
                    wr::<usize>(r + 0x20, m);
                    wr::<usize>(r + 0x28, m + 16);
                    wr::<u32>(r + 0x30, 1);
                    wr::<u32>(r + 0x34, 0);
                }
                2 => {
                    // Wrap around: write_ptr near end, read_ptr at +32
                    wr::<usize>(r + 0x20, m + 32);
                    wr::<usize>(r + 0x28, m + 48);
                    wr::<u32>(r + 0x30, 1);
                    wr::<u32>(r + 0x34, 0);
                }
                _ => {
                    // Full ring: write_ptr <= read_ptr and next_ptr > read_ptr
                    wr::<usize>(r + 0x20, m + 16);
                    wr::<usize>(r + 0x28, m + 8);
                    wr::<u32>(r + 0x30, 2);
                    wr::<u32>(r + 0x34, 1);
                }
            }
        }

        let sz = if scenario == 0 { 0 } else { 16 };
        let ra = o(ring_a.as_mut_ptr() as usize, sz, 0);
        let rb = n(ring_b.as_mut_ptr() as usize, sz, 0);

        let off_a = if ra == 0 { 0 } else { ra - mem_a.as_ptr() as usize };
        let off_b = if rb == 0 { 0 } else { rb - mem_b.as_ptr() as usize };
        crate::diff_eq!(off_a, off_b, "ret scenario={scenario}");
        let wp_a = rd::<usize>(ring_a.as_ptr() as usize + 0x28) - mem_a.as_ptr() as usize;
        let wp_b = rd::<usize>(ring_b.as_ptr() as usize + 0x28) - mem_b.as_ptr() as usize;
        crate::diff_eq!(wp_a, wp_b, "write_ptr scenario={scenario}");
        crate::diff_eq!(mem_a, mem_b, "mem scenario={scenario}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_find_entry_by_id_40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut table = vec![0u8; 0x100];
    let t = table.as_mut_ptr() as usize;
    wr::<usize>(t, 3);
    wr::<u32>(t + 0xa8, 10);
    wr::<u32>(t + 0xac, 20);
    wr::<u32>(t + 0xb0, 30);

    let mut cases = 0;
    for q in [10u32, 20, 30, 99] {
        let ra = o(t, q);
        let rb = n(t, q);
        crate::diff_eq!(ra, rb, "query={q}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_wait_post_cull_job_60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut WAIT_ARGS: (usize, usize) = (0, 0);
    unsafe extern "win64" fn mock_wait(sched: usize, h: usize) {
        unsafe {
            WAIT_ARGS = (sched, h);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4248_5780, mock_wait as *const ())?;

    WAIT_ARGS = (0, 0);
    o(0x2000);
    let a = WAIT_ARGS;

    WAIT_ARGS = (0, 0);
    n(0x2000);
    let b = WAIT_ARGS;
    crate::diff_eq!(a, b, "wait args");
    Ok(1)
}

pub unsafe fn diff_schedule_post_cull_jobs(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut JOB_BUF1: [u64; 8] = [0; 8];
    static mut JOB_BUF2: [u64; 8] = [0; 8];
    static mut ALLOC_IDX: u32 = 0;
    static mut DISPATCHED: usize = 0;

    unsafe extern "win64" fn mock_worker() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc(_pool: usize, out: usize) -> usize {
        unsafe {
            ALLOC_IDX += 1;
            let p = if ALLOC_IDX == 1 {
                JOB_BUF1.as_mut_ptr() as usize
            } else {
                JOB_BUF2.as_mut_ptr() as usize
            };
            wr::<usize>(out, p);
            wr::<u64>(out + 8, 0x10 + ALLOC_IDX as u64);
            out
        }
    }
    unsafe extern "win64" fn mock_dispatch(_sched: usize, _h: usize) {
        unsafe {
            DISPATCHED += 1;
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5930, mock_dispatch as *const ())?;

    let holder_va = exe::ptr::<usize>(THREAD_MGR_PTR_VA) as usize;
    let orig_holder = rd::<usize>(holder_va);
    let mut fake_sched = vec![0u8; 0x100];
    let mut fake_holder = [0usize; 4];
    fake_holder[2] = fake_sched.as_mut_ptr() as usize;
    wr::<usize>(holder_va, fake_holder.as_mut_ptr() as usize);

    let mut this_a = vec![0u8; 0x100];
    wr::<u64>(this_a.as_mut_ptr() as usize + 0xe0, 42);

    JOB_BUF1 = [0; 8];
    JOB_BUF2 = [0; 8];
    ALLOC_IDX = 0;
    DISPATCHED = 0;
    o(this_a.as_mut_ptr() as usize);
    let (da, j1a, j2a) = (DISPATCHED, JOB_BUF1, JOB_BUF2);
    let state_a = this_a.clone();

    this_a.fill(0);
    wr::<u64>(this_a.as_mut_ptr() as usize + 0xe0, 42);
    JOB_BUF1 = [0; 8];
    JOB_BUF2 = [0; 8];
    ALLOC_IDX = 0;
    DISPATCHED = 0;
    n(this_a.as_mut_ptr() as usize);
    let (db, j1b, j2b) = (DISPATCHED, JOB_BUF1, JOB_BUF2);

    wr::<usize>(holder_va, orig_holder);
    crate::diff_eq!(da, db, "dispatched count");
    crate::diff_eq!(j1a, j1b, "job1");
    crate::diff_eq!(j2a, j2b, "job2");
    crate::diff_eq!(state_a, this_a, "this state");
    Ok(1)
}

pub unsafe fn diff_render_pair_array_eq(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut a = [0u8; 0x18];
    let mut b = [0u8; 0x18];
    wr::<u32>(a.as_mut_ptr() as usize, 7);
    wr::<u32>(a.as_mut_ptr() as usize + 4, 1);
    wr::<u64>(a.as_mut_ptr() as usize + 8, 0x1111);
    wr::<u64>(a.as_mut_ptr() as usize + 16, 0x2222);
    b.copy_from_slice(&a);

    let ra1 = o(a.as_ptr() as usize, b.as_ptr() as usize) & 1;
    let rb1 = n(a.as_ptr() as usize, b.as_ptr() as usize) & 1;
    crate::diff_eq!(ra1, rb1, "equal inline");

    wr::<u64>(b.as_mut_ptr() as usize + 16, 0x3333);
    let ra2 = o(a.as_ptr() as usize, b.as_ptr() as usize) & 1;
    let rb2 = n(a.as_ptr() as usize, b.as_ptr() as usize) & 1;
    crate::diff_eq!(ra2, rb2, "unequal inline");
    Ok(2)
}

pub unsafe fn diff_render_pair_array_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut FREED: Vec<usize> = Vec::new();
    unsafe extern "win64" fn mock_free(p: usize) {
        unsafe {
            FREED.push(p);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4244_2de0, mock_free as *const ())?;

    let mut cases = 0;
    for cnt in [1u32, 3] {
        let mut obj = [0u8; 0x18];
        wr::<u32>(obj.as_mut_ptr() as usize + 4, cnt);
        wr::<usize>(obj.as_mut_ptr() as usize + 8, 0x1000);
        wr::<usize>(obj.as_mut_ptr() as usize + 16, 0x2000);

        FREED.clear();
        o(obj.as_mut_ptr() as usize);
        let fa = FREED.clone();

        FREED.clear();
        n(obj.as_mut_ptr() as usize);
        let fb = FREED.clone();
        crate::diff_eq!(fa, fb, "freed cnt={cnt}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_render_pair_array_copy_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut src = [0u8; 0x18];
    wr::<u32>(src.as_mut_ptr() as usize, 9);
    wr::<u32>(src.as_mut_ptr() as usize + 4, 1);
    wr::<u64>(src.as_mut_ptr() as usize + 8, 0xaaaa_bbbb);
    wr::<u64>(src.as_mut_ptr() as usize + 16, 0xcccc_dddd);

    let mut dst_a = [0xffu8; 0x18];
    let mut dst_b = [0xffu8; 0x18];
    let ra = o(dst_a.as_mut_ptr() as usize, src.as_ptr() as usize) - dst_a.as_ptr() as usize;
    let rb = n(dst_b.as_mut_ptr() as usize, src.as_ptr() as usize) - dst_b.as_ptr() as usize;
    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(dst_a, dst_b, "inline copy");
    Ok(1)
}

pub unsafe fn diff_get_render_bucket_or_default_338(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut obj = vec![0u8; 0x140];
    let p = obj.as_mut_ptr() as usize;
    wr::<usize>(p + 0x118, 0x1000_0000);
    wr::<u32>(p + 0x120, 4);
    wr::<usize>(p + 0x128, 0x9999_0000);

    let mut cases = 0;
    for idx in [0u32, 2, 4, 9] {
        crate::diff_eq!(o(p, idx), n(p, idx), "idx={idx}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_get_render_sub_entry_c8(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut elem0 = vec![0u8; 0xd0];
    let mut elem1 = vec![0u8; 0xd0];
    wr::<usize>(elem0.as_mut_ptr() as usize + 0xc8, 0x1111_2222);
    wr::<usize>(elem1.as_mut_ptr() as usize + 0xc8, 0x3333_4444);

    let arr0: [usize; 1] = [elem0.as_ptr() as usize];
    let arr1: [usize; 1] = [elem1.as_ptr() as usize];
    let mut grp = [0usize; 3];
    grp[1] = arr1.as_ptr() as usize; // group=1 -> +24 - 16 = +8

    let mut obj = vec![0u8; 0x180];
    let p = obj.as_mut_ptr() as usize;
    wr::<usize>(p + 0x160, arr0.as_ptr() as usize);
    wr::<usize>(p + 0x170, grp.as_ptr() as usize);
    wr::<u32>(p + 0x178, 1);

    let mut cases = 0;
    for g in [0i32, 1] {
        crate::diff_eq!(o(p, 0, g), n(p, 0, g), "group={g}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_get_render_grid_entry_380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let grid: [usize; 6] = [10, 20, 30, 40, 50, 60];
    let mut obj = vec![0u8; 0x390];
    let p = obj.as_mut_ptr() as usize;
    wr::<u8>(p + 0x270, 3);
    wr::<usize>(p + 0x380, grid.as_ptr() as usize);
    crate::diff_eq!(o(p, 1, 2), n(p, 1, 2), "row=1 col=2");
    Ok(1)
}

pub unsafe fn diff_update_render_instance_submesh_mask(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    unsafe extern "win64" fn mock_nop(_p: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_nop as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_ad50, mock_nop as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4220_ac20, mock_nop as *const ())?;

    let mut sub_a = vec![0u8; 0xd0];
    let mut sub_b = vec![0u8; 0xd0];
    wr::<i32>(sub_a.as_mut_ptr() as usize + 0x24, 0);
    wr::<u32>(sub_a.as_mut_ptr() as usize + 0xc4, 77);
    sub_b.copy_from_slice(&sub_a);

    let mut ent_a = [0u8; 0x18];
    let mut ent_b = [0u8; 0x18];
    wr::<usize>(ent_a.as_mut_ptr() as usize, sub_a.as_mut_ptr() as usize);
    wr::<u32>(ent_a.as_mut_ptr() as usize + 0xc, 1);
    wr::<usize>(ent_b.as_mut_ptr() as usize, sub_b.as_mut_ptr() as usize);
    wr::<u32>(ent_b.as_mut_ptr() as usize + 0xc, 1);

    let mut tbl_a = vec![0u8; 0x2100];
    let mut tbl_b = vec![0u8; 0x2100];
    let mut inst_a = vec![0u8; 0x460];
    let mut inst_b = vec![0u8; 0x460];

    for (inst, tbl, ent) in [
        (
            inst_a.as_mut_ptr() as usize,
            tbl_a.as_mut_ptr() as usize,
            ent_a.as_mut_ptr() as usize,
        ),
        (
            inst_b.as_mut_ptr() as usize,
            tbl_b.as_mut_ptr() as usize,
            ent_b.as_mut_ptr() as usize,
        ),
    ] {
        wr::<usize>(inst + 0x428, tbl);
        wr::<u16>(inst + 0x432, 2);
        wr::<usize>(inst + 0x448, ent);
        wr::<u32>(inst + 0x450, 1);
        wr::<u32>(inst + 0x458, 0);
    }

    o(inst_a.as_mut_ptr() as usize, 1);
    n(inst_b.as_mut_ptr() as usize, 1);
    crate::diff_eq!(sub_a, sub_b, "sub after enable");
    crate::diff_eq!(&ent_a[8..], &ent_b[8..], "entry after enable");
    crate::diff_eq!(
        rd::<u32>(tbl_a.as_ptr() as usize + 0x1008),
        rd::<u32>(tbl_b.as_ptr() as usize + 0x1008),
        "table handle after enable"
    );

    o(inst_a.as_mut_ptr() as usize, 0);
    n(inst_b.as_mut_ptr() as usize, 0);
    crate::diff_eq!(sub_a, sub_b, "sub after disable");
    crate::diff_eq!(&ent_a[8..], &ent_b[8..], "entry after disable");
    crate::diff_eq!(tbl_a, tbl_b, "table after disable");
    Ok(2)
}

pub unsafe fn diff_get_render_sub_entry_c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut elem0 = vec![0u8; 0xd0];
    wr::<usize>(elem0.as_mut_ptr() as usize + 0xc0, 0x5555_6666);
    let arr0: [usize; 1] = [elem0.as_ptr() as usize];
    let mut obj = vec![0u8; 0x180];
    let p = obj.as_mut_ptr() as usize;
    wr::<usize>(p + 0x160, arr0.as_ptr() as usize);
    crate::diff_eq!(o(p, 0, 0), n(p, 0, 0), "group=0");
    Ok(1)
}

pub unsafe fn diff_lookup_mesh_bone_or_submesh_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    unsafe extern "win64" fn mock_bsearch(_arr: usize, cnt: u32, id: u32) -> u32 {
        cnt + id
    }
    let _m = crate::difftest::Mock::new(0x1_4248_9d30, mock_bsearch as *const ())?;

    let mut obj = vec![0u8; 0x200];
    let p = obj.as_mut_ptr() as usize;
    crate::diff_eq!(o(p, 5), n(p, 5), "null base");

    wr::<usize>(p + 0x190, 0x1000);
    wr::<u32>(p + 0x1c8, 10);
    wr::<u32>(p + 0x1e8, 2);
    crate::diff_eq!(o(p, 5), n(p, 5), "non-null base");
    Ok(2)
}

pub unsafe fn diff_find_render_tag_1based(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut buf = [0u8; 48];
    wr::<u32>(buf.as_mut_ptr() as usize, 111);
    wr::<u32>(buf.as_mut_ptr() as usize + 24, 222);

    let mut obj = vec![0u8; 0x180];
    let p = obj.as_mut_ptr() as usize;
    wr::<usize>(p + 0x170, buf.as_ptr() as usize);
    wr::<u32>(p + 0x178, 2);

    let mut cases = 0;
    for tag in [0u32, 0x933b_5bde, 111, 222, 333] {
        crate::diff_eq!(o(p, tag), n(p, tag), "tag={tag:#x}");
        cases += 1;
    }
    Ok(cases)
}
