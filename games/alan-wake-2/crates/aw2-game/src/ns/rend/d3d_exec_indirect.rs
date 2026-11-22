//! The d3d context's `ExecuteIndirect` wrapper (`0x14206d7e0`).

use crate::exe::{self, rd, wr};

/// `--count`; at 0 drop ownership (`xchg owner, -1`) and
/// `ReleaseSRWLockExclusive` (recursive lock taken by `0x1411d5b00`).
#[inline]
unsafe fn unlock_recursive_srw(lock: usize) {
    let next = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, next);
    if next == 0 {
        wr::<u32>(lock + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

/// `ID3D12GraphicsCommandList::ExecuteIndirect` (vtable +0x1d8) with the
/// wrapper's argument mapping: the D3D command signature at `sig+8`, the
/// D3D resources at `buf+0x40`, offsets zero-extended.
#[inline]
unsafe fn execute(cl: usize, sig: usize, max_count: u32, argbuf: usize, argoff: u32, countbuf: usize, countoff: u32) {
    let f = rd::<usize>(rd::<usize>(cl) + 0x1d8);
    let count_res = if countbuf != 0 { rd::<usize>(countbuf + 0x40) } else { 0 };
    exe::call_va!(
        f,
        (cl, rd::<usize>(sig + 8), max_count, rd::<usize>(argbuf + 0x40), argoff as u64, count_res, countoff as u64) -> ()
    );
}

/// Original: `0x14206d7e0` (576 bytes, `Throws::Discard`: the native
/// callees' contract throws). Main caller: `0x142324af0` ("Draw {} pass").
///
/// `kind` 0 = compute: with a compute PSO (`ctx+0xc58`) set it
/// (`0x1420604d0` + `SetPipelineState`), bind the compute root
/// (`0x1420711e0`) and execute. Otherwise, with a graphics PSO
/// (`ctx+0xc60`), make sure the pipeline is bound (`ctx+0xa80` or
/// `0x142070fb0`) and bind the root signature `g_dev+0x28`.
/// `kind` != 0 = graphics topology: lazily allocate `ctx+0x48`
/// (`0x1411dce60`), switch `IASetPrimitiveTopology` when `ctx+0xc40`
/// differs (clearing the pipeline-bound flag), flush vertex buffers
/// (`0x14206f290`), bind the pipeline, root signature `g_dev+0x20`. Both
/// graphics paths bind through `0x142071360` before executing.
pub unsafe extern "win64" fn d3d_execute_indirect(
    ctx: usize,
    kind: u32,
    sig: usize,
    max_count: u32,
    argbuf: usize,
    argoff: u32,
    countbuf: usize,
    countoff: u32,
) {
    let mut pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (pair.as_mut_ptr() as usize) -> ());
    'body: {
        let g_dev = exe::ptr::<usize>(0x1_4397_fc58) as usize;
        let root_sig;
        if kind == 0 {
            let cpso = rd::<usize>(ctx + 0xc58);
            if cpso != 0 {
                let cl = pair[1];
                let set_pso = rd::<usize>(rd::<usize>(cl) + 0xc8);
                let pso = exe::call_va!(0x1_4206_04d0, (cpso) -> usize);
                exe::call_va!(set_pso, (cl, pso) -> ());
                // rcx is dead here in the original (left over from the vcall).
                exe::call_va!(0x1_4207_11e0, (cl, ctx) -> ());
                execute(cl, sig, max_count, argbuf, argoff, countbuf, countoff);
                break 'body;
            }
            if rd::<usize>(ctx + 0xc60) == 0 {
                break 'body;
            }
            // rcx = ctx+0xc58 (0) in the original.
            if rd::<u8>(ctx + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (0usize, ctx) -> u8) == 0 {
                break 'body;
            }
            root_sig = rd::<usize>(rd::<usize>(g_dev) + 0x28);
        } else {
            if rd::<usize>(ctx + 0x48) == 0 {
                let sub = rd::<usize>(ctx + 0xc48);
                if sub == 0 {
                    break 'body;
                }
                let p_idx = rd::<usize>(ctx + 0xb18);
                let slot = exe::call_va!(0x1_411d_ce60, (sub, rd::<u32>(p_idx).wrapping_add(2), p_idx) -> usize);
                wr::<usize>(ctx + 0x48, slot);
                if slot == 0 {
                    break 'body;
                }
            }
            if rd::<u32>(ctx + 0xc40) != kind {
                let mut pair2 = [0usize; 2];
                exe::call_va!(0x1_411d_5b00, (pair2.as_mut_ptr() as usize) -> ());
                let cl2 = pair2[1];
                let set_topology = rd::<usize>(rd::<usize>(cl2) + 0xa0);
                exe::call_va!(set_topology, (cl2, kind) -> ());
                if pair2[0] != 0 {
                    unlock_recursive_srw(pair2[0]);
                }
                wr::<u32>(ctx + 0xc40, kind);
                wr::<u8>(ctx + 0xa80, 0);
            }
            exe::call_va!(0x1_4206_f290, (ctx) -> ());
            if rd::<u8>(ctx + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (ctx, ctx) -> u8) == 0 {
                break 'body;
            }
            root_sig = rd::<usize>(rd::<usize>(g_dev) + 0x20);
        }
        exe::call_va!(0x1_4207_1360, (ctx, root_sig) -> ());
        execute(pair[1], sig, max_count, argbuf, argoff, countbuf, countoff);
    }
    if pair[0] != 0 {
        unlock_recursive_srw(pair[0]);
    }
}

// ---------------------------------------------------------------------------
// Difftest
// ---------------------------------------------------------------------------

/// Call log shared by the mocks: (tag, args...) records, compared between
/// the original and the replacement.
static mut LOG: [u64; 256] = [0; 256];
static mut LOG_N: usize = 0;
/// Recursive-lock fixture handed out by the `0x1411d5b00` mock (0 = none):
/// SRWLOCK at +0, owner tid at +8, count at +0xc.
static mut FIX_LOCK: usize = 0;
static mut FIX_CL: usize = 0;
/// `0x1411dce60` mock result (0 = allocation failure).
static mut FIX_SLOT: usize = 0;

unsafe fn log(rec: &[u64]) {
    let base = core::ptr::addr_of_mut!(LOG) as *mut u64;
    for &v in rec {
        if LOG_N < 256 {
            base.add(LOG_N).write(v);
            LOG_N += 1;
        }
    }
}

unsafe fn take_log() -> Vec<u64> {
    let base = core::ptr::addr_of!(LOG) as *const u64;
    let v = (0..LOG_N).map(|i| base.add(i).read()).collect();
    LOG_N = 0;
    v
}

/// Same contract as the engine's acquire: recursive SRW lock, `{lock, cl}`.
unsafe extern "win64" fn mock_1411d5b00(out_pair: usize) {
    let lock = FIX_LOCK;
    if lock != 0 {
        let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(lock + 0x8) != tid {
            exe::call_iat!(0x1_4264_55a8, (lock) -> ());
            wr::<u32>(lock + 0x8, tid);
        }
        wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));
    }
    let cnt = if lock != 0 { rd::<u32>(lock + 0xc) as u64 } else { 0 };
    log(&[0x5b00, lock as u64, cnt]);
    wr::<usize>(out_pair, lock);
    wr::<usize>(out_pair + 8, FIX_CL);
}

unsafe extern "win64" fn mock_1420604d0(cpso: usize) -> usize {
    log(&[0x04d0, cpso as u64]);
    cpso ^ 0x5555
}

unsafe extern "win64" fn mock_1420711e0(_rcx: usize, ctx: usize) {
    log(&[0x11e0, ctx as u64]);
}

/// Pipeline bind: result from `ctx+0xa81` (test knob); rcx is dead.
unsafe extern "win64" fn mock_142070fb0(_rcx: usize, ctx: usize) -> u8 {
    log(&[0x0fb0, ctx as u64, rd::<u8>(ctx + 0xa80) as u64, rd::<u32>(ctx + 0xc40) as u64]);
    let ok = rd::<u8>(ctx + 0xa81);
    if ok != 0 {
        wr::<u8>(ctx + 0xa80, 1);
    }
    ok
}

unsafe extern "win64" fn mock_142071360(ctx: usize, rs: usize) {
    log(&[0x1360, ctx as u64, rs as u64]);
}

unsafe extern "win64" fn mock_14206f290(ctx: usize) {
    log(&[0xf290, ctx as u64, rd::<u32>(ctx + 0xc40) as u64, rd::<u8>(ctx + 0xa80) as u64]);
}

unsafe extern "win64" fn mock_1411dce60(sub: usize, n: u32, p_idx: usize) -> usize {
    log(&[0xce60, sub as u64, n as u64, p_idx as u64]);
    FIX_SLOT
}

unsafe extern "win64" fn mock_cl_set_pso(cl: usize, pso: usize) {
    log(&[0xc8, cl as u64, pso as u64]);
}

unsafe extern "win64" fn mock_cl_set_topology(cl: usize, topo: u32) {
    // Lock state at the call: the second acquire must be recursive.
    let lock = FIX_LOCK;
    let cnt = if lock != 0 { rd::<u32>(lock + 0xc) as u64 } else { 0 };
    log(&[0xa0, cl as u64, topo as u64, cnt]);
}

unsafe extern "win64" fn mock_cl_execute_indirect(cl: usize, sig: usize, max: u32, arg: usize, argoff: u64, cnt: usize, cntoff: u64) {
    let lock = FIX_LOCK;
    let lcnt = if lock != 0 { rd::<u32>(lock + 0xc) as u64 } else { 0 };
    log(&[0x1d8, cl as u64, sig as u64, max as u64, arg as u64, argoff, cnt as u64, cntoff, lcnt]);
}

pub unsafe fn diff_d3d_execute_indirect(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Rng, SKIP};
    // Adopt the thread (TEB) so the IAT lock calls work.
    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if tid == 0 {
        return Err(format!("{SKIP} no thread id"));
    }
    let _m0 = Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    let _m1 = Mock::new(0x1_4206_04d0, mock_1420604d0 as *const ())?;
    let _m2 = Mock::new(0x1_4207_11e0, mock_1420711e0 as *const ())?;
    let _m3 = Mock::new(0x1_4207_0fb0, mock_142070fb0 as *const ())?;
    let _m4 = Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = Mock::new(0x1_4206_f290, mock_14206f290 as *const ())?;
    let _m6 = Mock::new(0x1_411d_ce60, mock_1411dce60 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u32, usize, u32, usize, u32, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0x40];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x2020_2020_aaaa);
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x28, 0x2828_2828_bbbb);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    let mut vtbl = vec![0usize; 0x200 / 8];
    vtbl[0xa0 / 8] = mock_cl_set_topology as *const () as usize;
    vtbl[0xc8 / 8] = mock_cl_set_pso as *const () as usize;
    vtbl[0x1d8 / 8] = mock_cl_execute_indirect as *const () as usize;

    let mut rng = Rng::new(0x1420_6d7e_0000_0001);
    let mut cases = 0u32;
    let mut res = Ok(());
    'cases: for case in 0..600u32 {
        // Misaligned (+4 / +1) placements for the context and the inputs.
        let mis = if case % 3 == 0 { 4 } else { 0 };
        let kind: u32 = match rng.below(6) {
            0 | 1 => 0,
            2 => 4,
            3 => 5,
            4 => 1 + rng.below(9) as u32,
            _ => rng.next_u32(),
        };
        let idx_val = rng.next_u32();
        let mut ctx_init = rng.bytes(0xc70);
        let put = |b: &mut Vec<u8>, off: usize, v: u64, n: usize| b[off..off + n].copy_from_slice(&v.to_le_bytes()[..n]);
        put(&mut ctx_init, 0xc58, if rng.below(2) == 0 { 0 } else { 0xc58c_0000 | rng.below(0x1000) }, 8);
        put(&mut ctx_init, 0xc60, if rng.below(3) == 0 { 0 } else { 0xc60c_0000 }, 8);
        put(&mut ctx_init, 0xa80, rng.below(2), 1);
        put(&mut ctx_init, 0xa81, (rng.below(4) != 0) as u64, 1);
        put(&mut ctx_init, 0x48, if rng.below(2) == 0 { 0 } else { 0x4848_0000 }, 8);
        put(&mut ctx_init, 0xc48, if rng.below(4) == 0 { 0 } else { 0xc48c_0000 }, 8);
        put(&mut ctx_init, 0xc40, if rng.below(2) == 0 { kind as u64 } else { rng.next_u32() as u64 }, 4);
        let slot = if rng.below(4) == 0 { 0 } else { 0x5107_0000 | rng.below(0x100) as usize };
        let with_lock = rng.below(4) != 0;
        let lock_held = rng.below(2) == 0;
        let countbuf_null = rng.below(3) == 0;
        let (max_count, argoff, countoff) = (rng.next_u32(), rng.next_u32(), rng.next_u32());

        let mut outs: Vec<(Vec<u64>, Vec<u8>, [u32; 2])> = Vec::new();
        for f in [f_orig, f_new] {
            let mut ctx_buf = vec![0u8; 0xc70 + 8];
            ctx_buf[mis..mis + 0xc70].copy_from_slice(&ctx_init);
            let ctx = ctx_buf.as_mut_ptr() as usize + mis;
            let mut idx_buf = [0u8; 12];
            let p_idx = idx_buf.as_mut_ptr() as usize + 1;
            wr::<u32>(p_idx, idx_val);
            wr::<usize>(ctx + 0xb18, p_idx);
            let mut sig = [0u8; 0x20];
            wr::<usize>(sig.as_mut_ptr() as usize + 4 + 8, 0x5150_0000 + case as usize);
            let mut argb = [0u8; 0x50];
            wr::<usize>(argb.as_mut_ptr() as usize + 4 + 0x40, 0xa4a4_0000 + case as usize);
            let mut cntb = [0u8; 0x50];
            wr::<usize>(cntb.as_mut_ptr() as usize + 4 + 0x40, 0xc4c4_0000 + case as usize);
            let mut cl = [0usize; 4];
            cl[0] = vtbl.as_ptr() as usize;
            // Lock fixture: a real SRWLOCK (+0), owner (+8), count (+0xc).
            let mut lock = [0u64; 2];
            let lp = lock.as_mut_ptr() as usize;
            if lock_held {
                // Pretend an outer holder (owner = us, count 2): no SRW calls.
                wr::<u32>(lp + 0x8, tid);
                wr::<u32>(lp + 0xc, 2);
            } else {
                wr::<u32>(lp + 0x8, u32::MAX);
            }
            FIX_LOCK = if with_lock { lp } else { 0 };
            FIX_CL = cl.as_mut_ptr() as usize;
            FIX_SLOT = slot;
            LOG_N = 0;
            f(
                ctx,
                kind,
                sig.as_ptr() as usize + 4,
                max_count,
                argb.as_ptr() as usize + 4,
                argoff,
                if countbuf_null { 0 } else { cntb.as_ptr() as usize + 4 },
                countoff,
            );
            // Normalise per-run addresses to offsets for the comparison.
            let mut lg = take_log();
            let ranges = [
                (ctx, 0xc70usize, 0x1_0000_0000u64),
                (cl.as_ptr() as usize, 0x20, 0x2_0000_0000),
                (lp, 0x10, 0x3_0000_0000),
                (p_idx, 4, 0x4_0000_0000),
            ];
            for v in lg.iter_mut() {
                for &(b, n, tag) in &ranges {
                    if (*v as usize) >= b && (*v as usize) < b + n {
                        *v = tag + (*v - b as u64);
                    }
                }
            }
            let state = ctx_buf[mis..mis + 0xc70].to_vec();
            let ls = [rd::<u32>(lp + 0x8), rd::<u32>(lp + 0xc)];
            // A fixture lock left held would deadlock later runs.
            if with_lock && !lock_held && ls[1] != 0 {
                res = Err(format!("case {case}: lock left held (count {})", ls[1]));
                break 'cases;
            }
            outs.push((lg, state, ls));
        }
        let (o, n) = (&outs[0], &outs[1]);
        if o.0 != n.0 {
            res = Err(format!("case {case} kind {kind}: call log orig={:x?} new={:x?}", o.0, n.0));
            break;
        }
        if o.1 != n.1 {
            let i = o.1.iter().zip(&n.1).position(|(a, b)| a != b).unwrap_or(0);
            res = Err(format!("case {case} kind {kind}: ctx byte {i:#x} orig={:#x} new={:#x}", o.1[i], n.1[i]));
            break;
        }
        if o.2 != n.2 {
            res = Err(format!("case {case}: lock state orig={:?} new={:?}", o.2, n.2));
            break;
        }
        cases += 1;
    }
    FIX_LOCK = 0;
    FIX_CL = 0;
    wr::<usize>(g_dev_addr, saved_g_dev);
    res.map(|_| cases)
}
