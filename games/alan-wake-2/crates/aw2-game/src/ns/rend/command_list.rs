//! Render-command lists executed by the renderWorldInternal pass callbacks.
//!
//! A command list (`rend::RendererInterface` holds them, e.g. `+0x388 + i*8` "Moment Commands"
//! in the MBOIT transparency pass `renderWorldInternal::<lambda_44>` 0x142272de0) is
//! `+0x00` instance-ID batch state (see [`PREPARE_VA`]), `+0x08` singly linked node list
//! (next at node `+0x10`), `+0x10` node count (i32), `+0x60/+0x68` `(node, u32 id)` pairs.
//! A node is `+0x00` vtable (slot 1 = `execute(node, ctx)`), `+0x08` u32 sort key, `+0x10`
//! next, `+0x18/+0x1c` its range in the uploaded instance-ID buffer. The per-draw work is in the
//! node `execute` implementations; [`run_command_list`] records one jobprof span per node
//! (`K_MARK`, `id` = the node's `execute` VA, `sys` = [`JOBPROF_NODE_SYS`]) so the hot node
//! classes can be identified.

use crate::exe::{self, rd, wr};
use crate::jobprof;

/// `rend_1423c3050(list)`: sorts the list's `(node, id)` pairs, uploads the ids and stores each
/// node's `[offset, count]` range (draft).
const PREPARE_VA: u64 = 0x1_423c_3050;
/// `r::mem` aligned allocate `(size, align)`.
const ALLOC_VA: u64 = 0x1_4241_c0a0;
/// `j_memset` import thunk.
const MEMSET_VA: u64 = 0x1_4183_acc2;
/// MSVC `std::sort` instantiation `(&first, &last, count, pred)` (STL, called through the VA).
const SORT_VA: u64 = 0x1_411d_45c0;
/// Sort predicate `a->key (+8) < b->key` (u32), passed by address.
const KEY_LESS_VA: u64 = 0x1_423c_3030;
/// `r::mem` free.
const FREE_VA: u64 = 0x1_4241_c190;

/// jobprof `sys` tag of the per-node spans recorded by [`run_command_list`].
pub const JOBPROF_NODE_SYS: u16 = 0xfffe;

/// Execute a render-command list: prepare it ([`PREPARE_VA`]), copy the node pointers into a
/// temporary array, sort it by the nodes' u32 key (MSVC `std::sort`, not stable), call each
/// node's `vfunc_1(node, ctx)` in that order and free the array.
///
/// Original: 0x1423c3e30 (276 bytes; FH4 unwind map frees the array if a node throws;
/// `Throws::Discard` like the lifted draft: on an exception the array leaks instead). The
/// count is read as i32 (`movsxd`): the allocation is `n * 8` sign-extended, the array end
/// `(u32)n * 8`, the sort count `(end - begin) >> 3` (arithmetic). An empty list allocates
/// nothing and still calls the sort with a null range.
pub unsafe extern "win64" fn run_command_list(list: usize, ctx: usize) {
    exe::call_va!(PREPARE_VA, (list, ctx));
    let n = rd::<i32>(list + 0x10);
    let mut node = rd::<usize>(list + 0x8);
    let mut buf = 0usize;
    if n != 0 {
        let bytes = (n as i64 as u64).wrapping_mul(8);
        buf = exe::call_va!(ALLOC_VA, (bytes, 8u32) -> usize);
        exe::call_va!(MEMSET_VA, (buf, 0u32, bytes));
    }
    let mut p = buf;
    while node != 0 {
        wr::<usize>(p, node);
        node = rd::<usize>(node + 0x10);
        p = p.wrapping_add(8);
    }
    let end = buf.wrapping_add((n as u32 as usize).wrapping_mul(8));
    let count = (end.wrapping_sub(buf) as i64 >> 3) as u64;
    let mut first = buf;
    let mut last = end;
    exe::call_va!(
        SORT_VA,
        (
            &mut first as *mut usize as usize,
            &mut last as *mut usize as usize,
            count,
            exe::ptr::<u8>(KEY_LESS_VA) as usize
        )
    );
    let mut i = 0u64;
    while i < count {
        let nd = rd::<usize>(buf.wrapping_add(i as usize * 8));
        let f = rd::<usize>(rd::<usize>(nd) + 8);
        let t = jobprof::begin();
        exe::call_va!(f as u64, (nd, ctx));
        if t.is_some() {
            let va = (f as u64).wrapping_sub(exe::ptr::<u8>(0) as u64);
            jobprof::end(t, jobprof::K_MARK, va, JOBPROF_NODE_SYS, 0);
        }
        i += 1;
    }
    if buf != 0 {
        exe::call_va!(FREE_VA, (buf));
    }
}

// ---------------------------------------------------------------------------
// Differential test
// ---------------------------------------------------------------------------

use std::cell::RefCell;

thread_local! {
    static LOG: RefCell<Vec<[u64; 4]>> = const { RefCell::new(Vec::new()) };
    static HEAP: RefCell<(usize, usize)> = const { RefCell::new((0, 0)) };
}

fn log(e: [u64; 4]) {
    LOG.with(|l| l.borrow_mut().push(e));
}

unsafe extern "win64" fn m_prepare(list: usize, _ctx: usize) {
    // Mirrors what the real prepare may touch: the instance batch fields.
    log([1, list as u64, rd::<u64>(list), 0]);
}

unsafe extern "win64" fn m_alloc(size: u64, align: u32) -> usize {
    let p = HEAP.with(|h| {
        let mut h = h.borrow_mut();
        let p = (h.0 + h.1 + 15) & !15;
        h.1 = p + size as usize - h.0;
        p
    });
    // Garbage the caller must clear (memset is the real import).
    for k in 0..size as usize {
        wr::<u8>(p + k, 0xa5);
    }
    log([2, size, align as u64, p as u64]);
    p
}

unsafe extern "win64" fn m_free(p: usize) {
    // The array as sorted (the nodes in call order were logged separately).
    let n = HEAP.with(|h| {
        let h = h.borrow();
        (h.0 + h.1 - p) / 8
    });
    let mut sum = 0u64;
    for k in 0..n {
        sum = sum.wrapping_mul(0x100_0000_01b3).wrapping_add(rd::<u64>(p + k * 8));
    }
    log([3, p as u64, sum, 0]);
}

unsafe extern "win64" fn m_exec_a(node: usize, ctx: usize) {
    log([4, node as u64, ctx as u64, rd::<u32>(node + 8) as u64]);
}

unsafe extern "win64" fn m_exec_b(node: usize, ctx: usize) {
    log([5, node as u64, ctx as u64, rd::<u32>(node + 8) as u64]);
    // Node bodies may write their own state.
    wr::<u32>(node + 0x1c, rd::<u32>(node + 0x1c).wrapping_add(1));
}

/// Lists of 0..64 nodes (random keys with many ties, two node classes), the real `std::sort` and
/// `memset`, mocked prepare/allocator/free; compares the call log (prepare, alloc size/align,
/// node calls in order with their ctx, the freed array's sorted contents) and the node memory.
pub unsafe fn diff_run_command_list(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m = [
        crate::difftest::Mock::new(PREPARE_VA, m_prepare as *const ())?,
        crate::difftest::Mock::new(ALLOC_VA, m_alloc as *const ())?,
        crate::difftest::Mock::new(FREE_VA, m_free as *const ())?,
    ];
    let vt_a = [0usize, m_exec_a as *const () as usize, 0];
    let vt_b = [0usize, m_exec_b as *const () as usize, 0];
    let mut heap = vec![0u64; 0x1000];
    let heap_base = heap.as_mut_ptr() as usize;
    let mut rng = crate::difftest::Rng::new(0x1423_c3e3);
    let mut cases = 0u32;
    for round in 0..96u32 {
        let count = match round {
            0 => 0,
            1 => 1,
            2 => 2,
            r => 1 + (r as usize * 7 + rng.below(13) as usize) % 64,
        };
        let key_range = [1u64, 3, 17, 1 << 32][round as usize % 4];
        let mut nodes = vec![0u64; count * 4 + 1];
        let base = nodes.as_mut_ptr() as usize;
        let mut list = [0u64; 16];
        let lp = list.as_mut_ptr() as usize;
        wr::<u64>(lp, if round % 3 == 0 { 0 } else { rng.next_u64() | 1 });
        let mut head = 0usize;
        for k in (0..count).rev() {
            let nd = base + k * 0x20;
            wr::<usize>(nd, (if rng.below(2) == 0 { vt_a.as_ptr() } else { vt_b.as_ptr() }) as usize);
            wr::<u32>(nd + 8, rng.below(key_range) as u32);
            wr::<usize>(nd + 0x10, head);
            wr::<u32>(nd + 0x18, rng.next_u32());
            wr::<u32>(nd + 0x1c, rng.next_u32());
            head = nd;
        }
        wr::<usize>(lp + 8, head);
        wr::<i32>(lp + 0x10, count as i32);
        let ctx = rng.next_u64() as usize;
        let init = (nodes.clone(), list);
        let mut out = Vec::new();
        for f in [o, n] {
            nodes.copy_from_slice(&init.0);
            list = init.1;
            HEAP.with(|h| *h.borrow_mut() = (heap_base, 0));
            LOG.with(|l| l.borrow_mut().clear());
            f(lp, ctx);
            out.push((LOG.with(|l| std::mem::take(&mut *l.borrow_mut())), nodes.clone(), list));
        }
        crate::diff_eq!(out[0].0, out[1].0, "call log, round {round} count {count}");
        crate::diff_eq!(out[0].1, out[1].1, "nodes, round {round} count {count}");
        crate::diff_eq!(out[0].2, out[1].2, "list, round {round} count {count}");
        let calls = out[0].0.iter().filter(|e| e[0] == 4 || e[0] == 5).count();
        if calls != count {
            return Err(format!("round {round}: {calls} node calls for {count} nodes"));
        }
        cases += 1;
    }
    drop(heap);
    Ok(cases)
}
