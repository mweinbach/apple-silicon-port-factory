//! Decompiled `d3d` render-graph pass dependency and iterator functions (`0x1420a6e20..0x1420a8cf0`).

use crate::exe::{self, rd, wr};

/// `d3d::renderGraphIterAtEnd_1420a6e20`
/// Original: 0x1420a6e20 (25 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_iter_at_end(this: usize) -> u8 {
    let inner = rd::<u64>(this + 0x8) as usize;
    let vec = rd::<u64>(inner) as usize;
    let len = rd::<u32>(vec + 0x8) as u64;
    let base = rd::<u64>(vec);
    let end = base.wrapping_add(len.wrapping_mul(8));
    u8::from(rd::<u64>(this + 0x10) == end)
}

/// `d3d::renderGraphIterNext_1420a6e50`
/// Original: 0x1420a6e50 (29 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_iter_next(this: usize) -> u64 {
    wr::<u64>(this + 0x10, rd::<u64>(this + 0x10).wrapping_add(8));
    exe::call_va!(0x1420a76b0, (this + 0x8) -> ());
    this as u64
}

/// `d3d::renderGraphIterEqualsChecked_1420a6e90`
/// Original: 0x1420a6e90 (51 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_iter_equals_checked(
    this: usize,
    other: usize,
) -> u8 {
    if rd::<u8>(this + 0x28) != 0 || rd::<u8>(other + 0x28) != 1 {
        exe::call_va!(0x140200070, () -> ());
        return 0;
    }
    let inner = rd::<u64>(this + 0x8) as usize;
    let vec = rd::<u64>(inner) as usize;
    let len = rd::<u32>(vec + 0x8) as u64;
    let base = rd::<u64>(vec);
    let end = base.wrapping_add(len.wrapping_mul(8));
    u8::from(rd::<u64>(this + 0x10) == end)
}

/// `d3d::renderGraphIterDeref_1420a6ed0`
/// Original: 0x1420a6ed0 (29 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_iter_deref(this: usize, out: usize) -> u64 {
    let idx_ptr = rd::<u64>(this + 0x18) as usize;
    let ctx = rd::<u64>(this) as usize;
    let idx = rd::<u64>(idx_ptr);
    let vec_ptr = rd::<u64>(ctx + 0x18) as usize;
    let elem = rd::<u64>(vec_ptr).wrapping_add(idx.wrapping_mul(0x38)) as usize;
    wr::<u16>(out, rd::<u16>(elem + 0x20));
    out as u64
}

/// `d3d::renderGraphIterNextChecked_1420a6f00`
/// Original: 0x1420a6f00 (41 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_iter_next_checked(this: usize) -> u64 {
    if rd::<u8>(this + 0x28) != 0 {
        exe::call_va!(0x140200070, () -> ());
        return this as u64;
    }
    wr::<u64>(this + 0x10, rd::<u64>(this + 0x10).wrapping_add(8));
    exe::call_va!(0x1420a76b0, (this + 0x8) -> ());
    this as u64
}

/// `d3d::renderGraphIterNotEqualsChecked_1420a6f30`
/// Original: 0x1420a6f30 (51 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_iter_not_equals_checked(
    this: usize,
    other: usize,
) -> u8 {
    if rd::<u8>(this + 0x28) != 0 || rd::<u8>(other + 0x28) != 1 {
        exe::call_va!(0x140200070, () -> ());
        return 0;
    }
    let inner = rd::<u64>(this + 0x8) as usize;
    let vec = rd::<u64>(inner) as usize;
    let len = rd::<u32>(vec + 0x8) as u64;
    let base = rd::<u64>(vec);
    let end = base.wrapping_add(len.wrapping_mul(8));
    u8::from(rd::<u64>(this + 0x10) != end)
}

/// `d3d::renderGraphIterDerefChecked_1420a6fa0`
/// Original: 0x1420a6fa0 (49 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_iter_deref_checked(this: usize, out: usize) -> u64 {
    if rd::<u8>(this + 0x28) != 0 {
        exe::call_va!(0x140200070, () -> ());
        return out as u64;
    }
    let idx_ptr = rd::<u64>(this + 0x18) as usize;
    let ctx = rd::<u64>(this) as usize;
    let idx = rd::<u64>(idx_ptr);
    let vec_ptr = rd::<u64>(ctx + 0x18) as usize;
    let elem = rd::<u64>(vec_ptr).wrapping_add(idx.wrapping_mul(0x38)) as usize;
    wr::<u16>(out, rd::<u16>(elem + 0x20));
    out as u64
}

/// `d3d::spanIterAtEnd_1420a7520`
/// Original: 0x1420a7520 (28 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_span_iter_at_end(this: usize) -> u8 {
    let span = rd::<u64>(this) as usize;
    let count = rd::<u64>(span);
    let base = rd::<u64>(span + 0x8);
    let end = base.wrapping_add(count.wrapping_mul(8));
    u8::from(rd::<u64>(this + 0x8) == end)
}

/// `d3d::spanIterSkipAboveOne_1420a7550`
/// Original: 0x1420a7550 (85 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_span_iter_skip_above_one(this: usize) {
    let mut span = rd::<u64>(this) as usize;
    let mut cur = rd::<u64>(this + 0x8);
    let mut end = rd::<u64>(span + 0x8).wrapping_add(rd::<u64>(span).wrapping_mul(8));
    while cur != end {
        wr::<u64>(this + 0x10, cur);
        wr::<u8>(this + 0x18, 1);
        if rd::<u32>((cur as usize) + 0x4) <= 1 {
            break;
        }
        cur = rd::<u64>(this + 0x8).wrapping_add(8);
        wr::<u64>(this + 0x8, cur);
        span = rd::<u64>(this) as usize;
        end = rd::<u64>(span + 0x8).wrapping_add(rd::<u64>(span).wrapping_mul(8));
    }
}

/// `d3d::copyU16FromThirdArg_1420a75b0`
/// Original: 0x1420a75b0 (11 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_copy_u16_from_third_arg(
    out: usize,
    _unused: usize,
    src: usize,
) -> u64 {
    wr::<u16>(out, rd::<u16>(src));
    out as u64
}

/// `d3d::vecIterAtEnd_1420a75d0`
/// Original: 0x1420a75d0 (24 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_vec_iter_at_end(this: usize) -> u8 {
    let ctx = rd::<u64>(this) as usize;
    let vec = rd::<u64>(ctx) as usize;
    let len = rd::<u32>(vec + 0x8) as u64;
    let base = rd::<u64>(vec);
    u8::from(rd::<u64>(this + 0x8) == base.wrapping_add(len.wrapping_mul(8)))
}

/// `d3d::vecIterFindTypeTwoMatch_1420a75f0`
/// Original: 0x1420a75f0 (95 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_vec_iter_find_type_two_match(this: usize) {
    let mut ctx = rd::<u64>(this) as usize;
    let mut cur = rd::<u64>(this + 0x8);
    let mut vec = rd::<u64>(ctx) as usize;
    let mut end = rd::<u64>(vec).wrapping_add((rd::<u32>(vec + 0x8) as u64).wrapping_mul(8));
    while cur != end {
        wr::<u64>(this + 0x10, cur);
        wr::<u8>(this + 0x18, 1);
        if rd::<u32>((cur as usize) + 0x4) == 2 {
            ctx = rd::<u64>(this) as usize;
            if rd::<u16>((cur as usize) + 0x2) == rd::<u16>(ctx + 0x8) {
                break;
            }
        }
        cur = rd::<u64>(this + 0x8).wrapping_add(8);
        wr::<u64>(this + 0x8, cur);
        ctx = rd::<u64>(this) as usize;
        vec = rd::<u64>(ctx) as usize;
        end = rd::<u64>(vec).wrapping_add((rd::<u32>(vec + 0x8) as u64).wrapping_mul(8));
    }
}

/// `d3d::renderGraphIterAdvanceToActive_1420a76b0`
/// Original: 0x1420a76b0 (91 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_iter_advance_to_active(this: usize) {
    let mut ctx = rd::<u64>(this) as usize;
    let mut cur = rd::<u64>(this + 0x8);
    let mut vec = rd::<u64>(ctx) as usize;
    let mut end = rd::<u64>(vec).wrapping_add((rd::<u32>(vec + 0x8) as u64).wrapping_mul(8));
    while cur != end {
        wr::<u64>(this + 0x10, cur);
        wr::<u8>(this + 0x18, 1);
        let idx = rd::<u64>(cur as usize);
        ctx = rd::<u64>(this) as usize;
        let nodes_vec = rd::<u64>(ctx + 0x8) as usize;
        let elem = rd::<u64>(nodes_vec).wrapping_add(idx.wrapping_mul(0x38)) as usize;
        if rd::<u8>(elem + 0x22) == 0 {
            break;
        }
        cur = rd::<u64>(this + 0x8).wrapping_add(8);
        wr::<u64>(this + 0x8, cur);
        vec = rd::<u64>(ctx) as usize;
        end = rd::<u64>(vec).wrapping_add((rd::<u32>(vec + 0x8) as u64).wrapping_mul(8));
    }
}

/// `d3d::spanIterNotAtEnd_1420a77a0`
/// Original: 0x1420a77a0 (28 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_span_iter_not_at_end(this: usize) -> u8 {
    let span = rd::<u64>(this) as usize;
    let count = rd::<u64>(span);
    let base = rd::<u64>(span + 0x8);
    let end = base.wrapping_add(count.wrapping_mul(8));
    u8::from(rd::<u64>(this + 0x8) != end)
}

/// `d3d::initFilterPred_1420a77f0`
/// Original: 0x1420a77f0 (22 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_init_filter_pred(this: usize, ptr_src: usize, u16_src: usize) -> u64 {
    wr::<u64>(this, rd::<u64>(ptr_src));
    wr::<u16>(this + 0x8, rd::<u16>(u16_src));
    wr::<u8>(this + 0xa, 1);
    this as u64
}

/// `d3d::vecIterNotAtEnd_1420a7860`
/// Original: 0x1420a7860 (24 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_vec_iter_not_at_end(this: usize) -> u8 {
    let ctx = rd::<u64>(this) as usize;
    let vec = rd::<u64>(ctx) as usize;
    let len = rd::<u32>(vec + 0x8) as u64;
    let base = rd::<u64>(vec);
    u8::from(rd::<u64>(this + 0x8) != base.wrapping_add(len.wrapping_mul(8)))
}

/// `d3d::renderGraphSubIterInit_1420a78e0`
/// Original: 0x1420a78e0 (31 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_sub_iter_init(
    this: usize,
    ctx: u64,
    start_ptr_ref: usize,
) -> u64 {
    wr::<u64>(this, ctx);
    wr::<u64>(this + 0x8, rd::<u64>(start_ptr_ref));
    wr::<u8>(this + 0x18, 0);
    exe::call_va!(0x1420a76b0, (this) -> ());
    this as u64
}

/// `d3d::renderGraphBeginIter_1420a7980`
/// Original: 0x1420a7980 (82 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_begin_iter(this: usize, out: usize) -> u64 {
    let vec = rd::<u64>(this) as usize;
    let mut tmp = [0u64; 4];
    let tmp_addr = tmp.as_mut_ptr() as usize;
    wr::<u64>(tmp_addr, this as u64);
    wr::<u64>(tmp_addr + 0x8, rd::<u64>(vec));
    wr::<u8>(tmp_addr + 0x18, 0);
    exe::call_va!(0x1420a76b0, (tmp_addr) -> ());
    wr::<u64>(out, this as u64);
    wr::<u64>(out + 0x8, rd::<u64>(tmp_addr));
    wr::<u64>(out + 0x10, rd::<u64>(tmp_addr + 0x8));
    wr::<u64>(out + 0x18, rd::<u64>(tmp_addr + 0x10));
    wr::<u64>(out + 0x20, rd::<u64>(tmp_addr + 0x18));
    out as u64
}

/// `d3d::renderGraphNodeGetId_1420a7a00`
/// Original: 0x1420a7a00 (21 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_node_get_id(
    this: usize,
    out: usize,
    idx: u64,
) -> u64 {
    let vec = rd::<u64>(this) as usize;
    let elem = rd::<u64>(vec).wrapping_add(idx.wrapping_mul(0x38)) as usize;
    wr::<u16>(out, rd::<u16>(elem + 0x20));
    out as u64
}

/// `d3d::renderGraphNodeIsActive_1420a7a30`
/// Original: 0x1420a7a30 (18 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_node_is_active(this: usize, idx: u64) -> u8 {
    let vec = rd::<u64>(this) as usize;
    let elem = rd::<u64>(vec).wrapping_add(idx.wrapping_mul(0x38)) as usize;
    u8::from(rd::<u8>(elem + 0x22) == 0)
}

/// `d3d::renderGraphNodeCompare_1420a7a70`
/// Original: 0x1420a7a70 (155 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_node_compare(
    this: usize,
    idx_a: u64,
    idx_b: u64,
) -> u8 {
    let solver = rd::<u64>(this) as usize;
    let da = exe::call_va!(0x1420a7db0, (solver, idx_a) -> u16);
    let db = exe::call_va!(0x1420a7db0, (solver, idx_b) -> u16);
    if da != db {
        let solver2 = rd::<u64>(this) as usize;
        let db2 = exe::call_va!(0x1420a7db0, (solver2, idx_b) -> u16);
        let da2 = exe::call_va!(0x1420a7db0, (solver2, idx_a) -> u16);
        return u8::from(da2 < db2);
    }
    let vec = rd::<u64>(this + 0x8) as usize;
    let base = rd::<u64>(vec);
    let elem_a = base.wrapping_add(idx_a.wrapping_mul(0x38)) as usize;
    let elem_b = base.wrapping_add(idx_b.wrapping_mul(0x38)) as usize;
    u8::from(rd::<u64>(elem_a + 0x28) < rd::<u64>(elem_b + 0x28))
}

/// `d3d::renderGraphComputeMaxChildDepth_1420a7cf0`
/// Original: 0x1420a7cf0 (177 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_compute_max_child_depth(
    this: usize,
    idx: u64,
) -> u16 {
    let vec = rd::<u64>(this) as usize;
    let elem = rd::<u64>(vec).wrapping_add(idx.wrapping_mul(0x38)) as usize;
    let mut edge = rd::<u64>(elem + 0x30) as usize;
    let mut max_depth: u16 = 0;
    while edge != 0 {
        let cur_vec = rd::<u64>(this) as usize;
        let diff = rd::<u64>(edge).wrapping_sub(rd::<u64>(cur_vec)) as i64;
        let target_idx = (diff / 0x38) as u64;
        let child_depth = exe::call_va!(0x1420a7db0, (this, target_idx) -> u16);
        let cand = child_depth.wrapping_add(rd::<u8>(edge + 0x10) as u16);
        if max_depth < cand {
            max_depth = cand;
        }
        edge = rd::<u64>(edge + 0x8) as usize;
    }
    max_depth
}

/// `d3d::renderGraphEvalNodeDepth_1420a7db0`
/// Original: 0x1420a7db0 (296 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_eval_node_depth(this: usize, idx: u64) -> u16 {
    let memo = rd::<u64>(this + 0x18) as usize;
    if rd::<u16>(memo + (idx as usize) * 2) == 0xffff {
        let len = rd::<u32>(this + 0x10) as u64;
        let cap = rd::<u32>(this + 0x14) as u64;
        if len.wrapping_mul(8) == cap.wrapping_mul(8) {
            let min_cap = len.wrapping_add(1);
            let grow = cap.wrapping_add(cap >> 1);
            let new_cap = if grow < min_cap { min_cap } else { grow };
            exe::call_va!(0x140469cd0, (this + 0x8, new_cap) -> ());
        }
        let cur_len = rd::<u32>(this + 0x10) as usize;
        let stack = rd::<u64>(this + 0x8) as usize;
        wr::<u64>(stack + cur_len * 8, idx);
        wr::<u32>(this + 0x10, (cur_len as u32).wrapping_add(1));

        let mut max_depth: u16 = 0;
        let vec = rd::<u64>(this) as usize;
        let elem = rd::<u64>(vec).wrapping_add(idx.wrapping_mul(0x38)) as usize;
        let mut edge = rd::<u64>(elem + 0x30) as usize;
        while edge != 0 {
            let cur_vec = rd::<u64>(this) as usize;
            let diff = rd::<u64>(edge).wrapping_sub(rd::<u64>(cur_vec)) as i64;
            let target_idx = (diff / 0x38) as u64;
            let child_depth = exe::call_va!(0x1420a7db0, (this, target_idx) -> u16);
            let cand = child_depth.wrapping_add(rd::<u8>(edge + 0x10) as u16);
            if max_depth < cand {
                max_depth = cand;
            }
            edge = rd::<u64>(edge + 0x8) as usize;
        }
        let memo_after = rd::<u64>(this + 0x18) as usize;
        wr::<u16>(memo_after + (idx as usize) * 2, max_depth);
        wr::<u32>(this + 0x10, rd::<u32>(this + 0x10).wrapping_sub(1));
    }
    let memo_final = rd::<u64>(this + 0x18) as usize;
    rd::<u16>(memo_final + (idx as usize) * 2)
}

/// `d3d::renderGraphEvalNodeDepthPtr_1420a7ee0`
/// Original: 0x1420a7ee0 (38 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_eval_node_depth_ptr(
    this: usize,
    node_ptr: u64,
) -> u16 {
    let vec = rd::<u64>(this) as usize;
    let diff = node_ptr.wrapping_sub(rd::<u64>(vec)) as i64;
    let idx = (diff / 0x38) as u64;
    exe::call_va!(0x1420a7db0, (this, idx) -> u16)
}

/// `d3d::renderGraphDepthSolverInit_1420a7f10`
/// Original: 0x1420a7f10 (250 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_depth_solver_init(
    this: usize,
    graph_vec: usize,
) -> u64 {
    wr::<u64>(this, graph_vec as u64);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    let count = rd::<u32>(graph_vec + 0x8);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    if count != 0 {
        let bytes = (count as u64).wrapping_mul(2);
        let buf = exe::call_va!(0x14241c0a0, (bytes, 2u32) -> u64);
        wr::<u64>(this + 0x18, buf);
        wr::<u32>(this + 0x20, 0);
        wr::<u32>(this + 0x24, count);
        for i in 0..(count as usize) {
            wr::<u16>((buf as usize) + i * 2, 0xffff);
        }
    }
    wr::<u32>(this + 0x20, count);
    let g_vec = rd::<u64>(this) as usize;
    let needed = rd::<u32>(g_vec + 0x8) as u64;
    let cur_cap = rd::<u32>(this + 0x14) as u64;
    if needed > cur_cap {
        let grow = cur_cap.wrapping_add(cur_cap >> 1);
        let new_cap = if grow < needed { needed } else { grow };
        let old_len = rd::<u32>(this + 0x10);
        let new_buf = exe::call_va!(0x14241c0a0, (new_cap.wrapping_mul(8), 8u32) -> u64);
        let old_buf = rd::<u64>(this + 0x8);
        exe::call_va!(0x14183acc8, (new_buf, old_buf, (old_len as u64).wrapping_mul(8)) -> u64);
        if old_buf != 0 {
            exe::call_va!(0x14241c190, (old_buf) -> ());
        }
        wr::<u64>(this + 0x8, new_buf);
        wr::<u32>(this + 0x10, old_len);
        wr::<u32>(this + 0x14, new_cap as u32);
    }
    this as u64
}

/// `d3d::writeU16_1420a8230`
/// Original: 0x1420a8230 (7 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_write_u16(this: usize, val: u16) -> u64 {
    wr::<u16>(this, val);
    this as u64
}

/// `d3d::matchTypeTwoU16_1420a8240`
/// Original: 0x1420a8240 (21 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_match_type_two_u16(this: usize, other: usize) -> u8 {
    if rd::<u32>(other + 0x4) == 2 && rd::<u16>(other + 0x2) == rd::<u16>(this) {
        1
    } else {
        0
    }
}

/// `d3d::isTypeAtMostOne_1420a84c0`
/// Original: 0x1420a84c0 (8 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_is_type_at_most_one(_this: usize, entry: usize) -> u8 {
    u8::from(rd::<u32>(entry + 0x4) <= 1)
}

/// `d3d::renderGraphGetNodeSpan_1420a8710`
/// Original: 0x1420a8710 (57 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_get_node_span(
    this: usize,
    out: usize,
    span_in: usize,
    node: usize,
) -> u64 {
    let table = rd::<u64>(this + 0x10) as usize;
    let id = rd::<u16>(node + 0x20) as usize;
    let base_ptr = rd::<u64>(span_in + 0x8);
    let packed = rd::<u32>(table + id * 4);
    let start = (packed as u16) as u64;
    let ptr = base_ptr.wrapping_add(start.wrapping_mul(0x38));
    let count = (((packed >> 16) as u16).wrapping_sub(packed as u16)) as u64;
    wr::<u64>(out + 0x8, ptr);
    wr::<u64>(out, count);
    out as u64
}

/// `d3d::renderGraphInitNodeEntry_1420a88c0`
/// Original: 0x1420a88c0 (55 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_render_graph_init_node_entry(
    _this: usize,
    out: usize,
    src: usize,
) -> u64 {
    wr::<u64>(out, 0);
    wr::<u64>(out + 0x8, 0);
    wr::<u64>(out + 0x10, 0);
    wr::<u64>(out + 0x18, 0xf);
    wr::<u16>(out + 0x20, rd::<u16>(src + 0x22));
    wr::<u8>(out + 0x22, rd::<u8>(src + 0x30));
    wr::<u64>(out + 0x28, rd::<u64>(src + 0x28));
    wr::<u64>(out + 0x30, 0);
    out as u64
}

/// `d3d::renderGraphBuilderInit_1420a8af0`
/// Original: 0x1420a8af0 (132 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_builder_init(
    this: usize,
    ctx: u64,
    vec: usize,
) -> u64 {
    wr::<u64>(this, ctx);
    wr::<u64>(this + 0x8, vec as u64);
    let fill_val = rd::<u32>(0x14297fdd4);
    let count = rd::<u32>(vec + 0x8);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    if count != 0 {
        let buf = exe::call_va!(0x14241c0a0, ((count as u64).wrapping_mul(4), 2u32) -> u64);
        wr::<u64>(this + 0x10, buf);
        wr::<u32>(this + 0x18, 0);
        wr::<u32>(this + 0x1c, count);
        for i in 0..(count as usize) {
            wr::<u32>((buf as usize) + i * 4, fill_val);
        }
    }
    wr::<u32>(this + 0x18, count);
    this as u64
}

/// `d3d::renderGraphPushEdge_1420a8b80`
/// Original: 0x1420a8b80 (143 bytes, Throws::Discard)
pub unsafe extern "win64" fn d3d_render_graph_push_edge(
    this: usize,
    node: usize,
    target_node: u64,
    weight: u8,
) {
    let prev_edge = rd::<u64>(node + 0x30);
    let len = rd::<u32>(this + 0x8) as u64;
    let cap = rd::<u32>(this + 0xc) as u64;
    if len.wrapping_mul(24) == cap.wrapping_mul(24) {
        exe::call_va!(0x1411ec570, (this, len.wrapping_add(1)) -> ());
    }
    let idx = rd::<u32>(this + 0x8) as usize;
    let base = rd::<u64>(this) as usize;
    let slot = base + idx * 24;
    wr::<u64>(slot, target_node);
    wr::<u64>(slot + 0x8, prev_edge);
    wr::<u64>(slot + 0x10, weight as u64);
    let new_len = (idx as u32).wrapping_add(1);
    wr::<u32>(this + 0x8, new_len);
    let new_base = rd::<u64>(this) as usize;
    let edge_ptr = (new_base + ((new_len as usize) - 1) * 24) as u64;
    wr::<u64>(node + 0x30, edge_ptr);
}

/// `d3d::findCaseInsensitiveStringSlice_1420a8c10`
/// Original: 0x1420a8c10 (218 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_find_case_insensitive_string_slice(
    this: usize,
    out: usize,
    needle: usize,
) -> u64 {
    let base = rd::<u64>(this) as usize;
    let count = rd::<u32>(this + 0x8) as usize;
    let end = base + count * 16;
    let mut cur = base;
    if cur != end {
        let needle_ptr = rd::<u64>(needle) as usize;
        let needle_len = rd::<u64>(needle + 0x8) as usize;
        let fold_table: usize = 0x1426d01e0;
        while cur != end {
            let cand_ptr = rd::<u64>(cur) as usize;
            let cand_len = rd::<u64>(cur + 0x8) as usize;
            if cand_len == needle_len {
                let mut matched = true;
                for i in 0..cand_len {
                    let b1 = rd::<u8>(cand_ptr + i) as usize;
                    let b2 = rd::<u8>(needle_ptr + i) as usize;
                    if rd::<u8>(fold_table + b1) != rd::<u8>(fold_table + b2) {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    break;
                }
            }
            cur += 16;
        }
    }
    if cur == end {
        wr::<u8>(out + 0x8, 0);
    } else {
        wr::<u8>(out + 0x8, 1);
        wr::<u64>(out, ((cur - base) >> 4) as u64);
    }
    out as u64
}

/// `d3d::destroyAndReturnCount_1420a8cf0`
/// Original: 0x1420a8cf0 (22 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_destroy_and_return_count(this: usize, item: usize) -> u32 {
    let count = rd::<u32>(this + 0x8);
    exe::call_va!(0x140600350, (this, item) -> ());
    count
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_d3d_render_graph_iter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut vec = [0x1000u64, 3u64];
    let mut inner = [vec.as_mut_ptr() as u64];
    let mut iter = [0u64, inner.as_mut_ptr() as u64, 0x1018u64];
    let r1 = f_orig(iter.as_mut_ptr() as usize);
    let r2 = f_new(iter.as_mut_ptr() as usize);
    crate::diff_eq!(r1, r2, "at_end_true");
    iter[2] = 0x1008;
    let r3 = f_orig(iter.as_mut_ptr() as usize);
    let r4 = f_new(iter.as_mut_ptr() as usize);
    crate::diff_eq!(r3, r4, "at_end_false");
    Ok(2)
}

pub unsafe fn diff_d3d_render_graph_iter_next(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let indices = [0u64, 1u64];
    let mut node_buf = [0u8; 0x70];
    node_buf[0x22] = 1; // node 0 inactive
    node_buf[0x38 + 0x22] = 0; // node 1 active
    let mut vec = [indices.as_ptr() as u64, 2u64];
    let mut nodes_vec = [node_buf.as_ptr() as u64, 2u64];
    let mut ctx = [vec.as_mut_ptr() as u64, nodes_vec.as_mut_ptr() as u64];
    let mut iter_o = [0u64, ctx.as_mut_ptr() as u64, indices.as_ptr() as u64, 0u64, 0u64, 0u64];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "iter_next");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_equals_checked(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut vec = [0x2000u64, 2u64];
    let mut inner = [vec.as_mut_ptr() as u64];
    let mut this = [0u64; 6];
    this[1] = inner.as_mut_ptr() as u64;
    this[2] = 0x2010;
    let mut other = [0u64; 6];
    wr::<u8>((other.as_mut_ptr() as usize) + 0x28, 1);
    let r1 = f_orig(this.as_mut_ptr() as usize, other.as_mut_ptr() as usize);
    let r2 = f_new(this.as_mut_ptr() as usize, other.as_mut_ptr() as usize);
    crate::diff_eq!(r1, r2, "eq_checked");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_deref(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut nodes = [0u8; 0x70];
    wr::<u16>((nodes.as_mut_ptr() as usize) + 0x38 + 0x20, 0x4242);
    let mut nodes_vec = [nodes.as_ptr() as u64];
    let mut ctx = [0u64, 0u64, 0u64, nodes_vec.as_mut_ptr() as u64];
    let idx = 1u64;
    let iter = [ctx.as_mut_ptr() as u64, 0u64, 0u64, (&idx as *const u64) as u64];
    let mut out_o = 0u16;
    let mut out_n = 0u16;
    f_orig(iter.as_ptr() as usize, (&mut out_o as *mut u16) as usize);
    f_new(iter.as_ptr() as usize, (&mut out_n as *mut u16) as usize);
    crate::diff_eq!(out_o, out_n, "deref");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_next_checked(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let indices = [0u64, 1u64];
    let mut node_buf = [0u8; 0x70];
    node_buf[0x38 + 0x22] = 0;
    let mut vec = [indices.as_ptr() as u64, 2u64];
    let mut nodes_vec = [node_buf.as_ptr() as u64, 2u64];
    let mut ctx = [vec.as_mut_ptr() as u64, nodes_vec.as_mut_ptr() as u64];
    let mut iter_o = [0u64, ctx.as_mut_ptr() as u64, indices.as_ptr() as u64, 0u64, 0u64, 0u64];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "next_checked");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_not_equals_checked(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut vec = [0x2000u64, 2u64];
    let mut inner = [vec.as_mut_ptr() as u64];
    let mut this = [0u64; 6];
    this[1] = inner.as_mut_ptr() as u64;
    this[2] = 0x2008;
    let mut other = [0u64; 6];
    wr::<u8>((other.as_mut_ptr() as usize) + 0x28, 1);
    let r1 = f_orig(this.as_mut_ptr() as usize, other.as_mut_ptr() as usize);
    let r2 = f_new(this.as_mut_ptr() as usize, other.as_mut_ptr() as usize);
    crate::diff_eq!(r1, r2, "neq_checked");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_deref_checked(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut nodes = [0u8; 0x70];
    wr::<u16>((nodes.as_mut_ptr() as usize) + 0x20, 0x1337);
    let mut nodes_vec = [nodes.as_ptr() as u64];
    let mut ctx = [0u64, 0u64, 0u64, nodes_vec.as_mut_ptr() as u64];
    let idx = 0u64;
    let iter = [
        ctx.as_mut_ptr() as u64,
        0u64,
        0u64,
        (&idx as *const u64) as u64,
        0u64,
        0u64,
    ];
    let mut out_o = 0u16;
    let mut out_n = 0u16;
    f_orig(iter.as_ptr() as usize, (&mut out_o as *mut u16) as usize);
    f_new(iter.as_ptr() as usize, (&mut out_n as *mut u16) as usize);
    crate::diff_eq!(out_o, out_n, "deref_checked");
    Ok(1)
}

pub unsafe fn diff_d3d_span_iter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let span = [4u64, 0x5000u64];
    let iter = [span.as_ptr() as u64, 0x5020u64];
    let r1 = f_orig(iter.as_ptr() as usize);
    let r2 = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r1, r2, "span_at_end");
    Ok(1)
}

pub unsafe fn diff_d3d_span_iter_skip_above_one(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let items: [u64; 2] = [(3u64 << 32) | 10, (1u64 << 32) | 20];
    let span = [2u64, items.as_ptr() as u64];
    let mut iter_o = [span.as_ptr() as u64, items.as_ptr() as u64, 0u64, 0u64];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "skip_above_one");
    Ok(1)
}

pub unsafe fn diff_d3d_copy_u16_from_third_arg(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let src = 0x9988u16;
    let mut out_o = 0u16;
    let mut out_n = 0u16;
    f_orig((&mut out_o as *mut u16) as usize, 0, (&src as *const u16) as usize);
    f_new((&mut out_n as *mut u16) as usize, 0, (&src as *const u16) as usize);
    crate::diff_eq!(out_o, out_n, "copy_u16");
    Ok(1)
}

pub unsafe fn diff_d3d_vec_iter_at_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let vec = [0x4000u64, 2u64];
    let ctx = [vec.as_ptr() as u64];
    let iter = [ctx.as_ptr() as u64, 0x4010u64];
    let r1 = f_orig(iter.as_ptr() as usize);
    let r2 = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r1, r2, "vec_iter_at_end");
    Ok(1)
}

pub unsafe fn diff_d3d_vec_iter_find_type_two_match(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let items: [u64; 2] = [(1u64 << 32) | (7u64 << 16), (2u64 << 32) | (7u64 << 16)];
    let vec = [items.as_ptr() as u64, 2u64];
    let ctx = [vec.as_ptr() as u64, 7u64];
    let mut iter_o = [ctx.as_ptr() as u64, items.as_ptr() as u64, 0u64, 0u64];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "find_type_two");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_iter_advance_to_active(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let indices = [0u64, 1u64];
    let mut node_buf = [0u8; 0x70];
    node_buf[0x22] = 1;
    node_buf[0x38 + 0x22] = 0;
    let vec = [indices.as_ptr() as u64, 2u64];
    let nodes_vec = [node_buf.as_ptr() as u64, 2u64];
    let ctx = [vec.as_ptr() as u64, nodes_vec.as_ptr() as u64];
    let mut iter_o = [ctx.as_ptr() as u64, indices.as_ptr() as u64, 0u64, 0u64];
    let mut iter_n = iter_o;
    f_orig(iter_o.as_mut_ptr() as usize);
    f_new(iter_n.as_mut_ptr() as usize);
    crate::diff_eq!(iter_o, iter_n, "advance_to_active");
    Ok(1)
}

pub unsafe fn diff_d3d_span_iter_not_at_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let span = [4u64, 0x5000u64];
    let iter = [span.as_ptr() as u64, 0x5008u64];
    let r1 = f_orig(iter.as_ptr() as usize);
    let r2 = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r1, r2, "span_not_at_end");
    Ok(1)
}

pub unsafe fn diff_d3d_init_filter_pred(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let ptr_val = 0x12345678u64;
    let u16_val = 0x55u16;
    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    f_orig(
        out_o.as_mut_ptr() as usize,
        (&ptr_val as *const u64) as usize,
        (&u16_val as *const u16) as usize,
    );
    f_new(
        out_n.as_mut_ptr() as usize,
        (&ptr_val as *const u64) as usize,
        (&u16_val as *const u16) as usize,
    );
    crate::diff_eq!(out_o, out_n, "init_filter_pred");
    Ok(1)
}

pub unsafe fn diff_d3d_vec_iter_not_at_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let vec = [0x4000u64, 2u64];
    let ctx = [vec.as_ptr() as u64];
    let iter = [ctx.as_ptr() as u64, 0x4008u64];
    let r1 = f_orig(iter.as_ptr() as usize);
    let r2 = f_new(iter.as_ptr() as usize);
    crate::diff_eq!(r1, r2, "vec_iter_not_at_end");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_sub_iter_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let indices = [0u64];
    let node_buf = [0u8; 0x38];
    let vec = [indices.as_ptr() as u64, 1u64];
    let nodes_vec = [node_buf.as_ptr() as u64, 1u64];
    let ctx = [vec.as_ptr() as u64, nodes_vec.as_ptr() as u64];
    let start_ptr = indices.as_ptr() as u64;
    let mut iter_o = [0u64; 4];
    let mut iter_n = [0u64; 4];
    f_orig(
        iter_o.as_mut_ptr() as usize,
        ctx.as_ptr() as u64,
        (&start_ptr as *const u64) as usize,
    );
    f_new(
        iter_n.as_mut_ptr() as usize,
        ctx.as_ptr() as u64,
        (&start_ptr as *const u64) as usize,
    );
    crate::diff_eq!(iter_o, iter_n, "sub_iter_init");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_begin_iter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let indices = [0u64];
    let node_buf = [0u8; 0x38];
    let vec = [indices.as_ptr() as u64, 1u64];
    let nodes_vec = [node_buf.as_ptr() as u64, 1u64];
    let ctx = [vec.as_ptr() as u64, nodes_vec.as_ptr() as u64];
    let mut out_o = [0u64; 5];
    let mut out_n = [0u64; 5];
    f_orig(ctx.as_ptr() as usize, out_o.as_mut_ptr() as usize);
    f_new(ctx.as_ptr() as usize, out_n.as_mut_ptr() as usize);
    crate::diff_eq!(out_o[0..4], out_n[0..4], "begin_iter_qwords");
    crate::diff_eq!(out_o[4] as u8, out_n[4] as u8, "begin_iter_flag");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_node_get_id(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut nodes = [0u8; 0x70];
    wr::<u16>((nodes.as_mut_ptr() as usize) + 0x38 + 0x20, 0xabcd);
    let vec = [nodes.as_ptr() as u64];
    let this = [vec.as_ptr() as u64];
    let mut out_o = 0u16;
    let mut out_n = 0u16;
    f_orig(this.as_ptr() as usize, (&mut out_o as *mut u16) as usize, 1);
    f_new(this.as_ptr() as usize, (&mut out_n as *mut u16) as usize, 1);
    crate::diff_eq!(out_o, out_n, "node_get_id");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_node_is_active(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut nodes = [0u8; 0x70];
    nodes[0x22] = 0;
    nodes[0x38 + 0x22] = 1;
    let vec = [nodes.as_ptr() as u64];
    let this = [vec.as_ptr() as u64];
    let r1 = f_orig(this.as_ptr() as usize, 0);
    let r2 = f_new(this.as_ptr() as usize, 0);
    crate::diff_eq!(r1, r2, "is_active_0");
    let r3 = f_orig(this.as_ptr() as usize, 1);
    let r4 = f_new(this.as_ptr() as usize, 1);
    crate::diff_eq!(r3, r4, "is_active_1");
    Ok(2)
}

pub unsafe fn diff_d3d_render_graph_eval_node_depth(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut nodes = [0u8; 0x70];
    let nodes_base = nodes.as_mut_ptr() as usize;
    let edge = [nodes_base as u64 + 0x38, 0u64, 3u64];
    wr::<u64>(nodes_base + 0x30, edge.as_ptr() as u64);
    let vec = [nodes_base as u64, 2u64];
    let mut stack_o = [0u64; 4];
    let mut memo_o = [0xffffu16, 0xffffu16];
    let mut solver_o = [
        vec.as_ptr() as u64,
        stack_o.as_mut_ptr() as u64,
        4u64 << 32,
        memo_o.as_mut_ptr() as u64,
        2u64,
    ];
    let mut stack_n = [0u64; 4];
    let mut memo_n = [0xffffu16, 0xffffu16];
    let mut solver_n = [
        vec.as_ptr() as u64,
        stack_n.as_mut_ptr() as u64,
        4u64 << 32,
        memo_n.as_mut_ptr() as u64,
        2u64,
    ];
    let r1 = f_orig(solver_o.as_mut_ptr() as usize, 0);
    let r2 = f_new(solver_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(r1, r2, "eval_depth_ret");
    crate::diff_eq!(memo_o, memo_n, "eval_depth_memo");
    Ok(2)
}

pub unsafe fn diff_d3d_write_u16(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut o = 0u16;
    let mut n = 0u16;
    f_orig((&mut o as *mut u16) as usize, 0x7766);
    f_new((&mut n as *mut u16) as usize, 0x7766);
    crate::diff_eq!(o, n, "write_u16");
    Ok(1)
}

pub unsafe fn diff_d3d_match_type_two_u16(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let this = 5u16;
    let other_match = (2u64 << 32) | (5u64 << 16);
    let other_miss = (1u64 << 32) | (5u64 << 16);
    let r1 = f_orig((&this as *const u16) as usize, (&other_match as *const u64) as usize);
    let r2 = f_new((&this as *const u16) as usize, (&other_match as *const u64) as usize);
    crate::diff_eq!(r1, r2, "match_hit");
    let r3 = f_orig((&this as *const u16) as usize, (&other_miss as *const u64) as usize);
    let r4 = f_new((&this as *const u16) as usize, (&other_miss as *const u64) as usize);
    crate::diff_eq!(r3, r4, "match_miss");
    Ok(2)
}

pub unsafe fn diff_d3d_is_type_at_most_one(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let e1 = 1u64 << 32;
    let e2 = 2u64 << 32;
    let r1 = f_orig(0, (&e1 as *const u64) as usize);
    let r2 = f_new(0, (&e1 as *const u64) as usize);
    crate::diff_eq!(r1, r2, "le_one_true");
    let r3 = f_orig(0, (&e2 as *const u64) as usize);
    let r4 = f_new(0, (&e2 as *const u64) as usize);
    crate::diff_eq!(r3, r4, "le_one_false");
    Ok(2)
}

pub unsafe fn diff_d3d_render_graph_get_node_span(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let table: [u32; 2] = [0, (5u32 << 16) | 2u32];
    let this = [0u64, 0u64, table.as_ptr() as u64];
    let span_in = [10u64, 0x8000u64];
    let mut node = [0u8; 0x28];
    wr::<u16>((node.as_mut_ptr() as usize) + 0x20, 1);
    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    f_orig(
        this.as_ptr() as usize,
        out_o.as_mut_ptr() as usize,
        span_in.as_ptr() as usize,
        node.as_ptr() as usize,
    );
    f_new(
        this.as_ptr() as usize,
        out_n.as_mut_ptr() as usize,
        span_in.as_ptr() as usize,
        node.as_ptr() as usize,
    );
    crate::diff_eq!(out_o, out_n, "node_span");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_init_node_entry(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut src = [0u8; 0x40];
    let src_addr = src.as_mut_ptr() as usize;
    wr::<u16>(src_addr + 0x22, 0x3344);
    wr::<u8>(src_addr + 0x30, 1);
    wr::<u64>(src_addr + 0x28, 0xdeadbeef);
    let mut out_o = [0u8; 0x38];
    let mut out_n = [0u8; 0x38];
    f_orig(0, out_o.as_mut_ptr() as usize, src_addr);
    f_new(0, out_n.as_mut_ptr() as usize, src_addr);
    crate::diff_eq!(out_o, out_n, "init_node_entry");
    Ok(1)
}

pub unsafe fn diff_d3d_render_graph_push_edge(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0u64; 6];
    let mut buf_n = [0u64; 6];
    let mut vec_o = [buf_o.as_mut_ptr() as u64, 2u64 << 32];
    let mut vec_n = [buf_n.as_mut_ptr() as u64, 2u64 << 32];
    let mut node_o = [0u8; 0x38];
    let mut node_n = [0u8; 0x38];
    f_orig(vec_o.as_mut_ptr() as usize, node_o.as_mut_ptr() as usize, 0x9000, 1);
    f_new(vec_n.as_mut_ptr() as usize, node_n.as_mut_ptr() as usize, 0x9000, 1);
    crate::diff_eq!(buf_o[0..2], buf_n[0..2], "push_edge_buf_ptrs");
    crate::diff_eq!(buf_o[2] as u8, buf_n[2] as u8, "push_edge_buf_flag");
    Ok(1)
}

pub unsafe fn diff_d3d_find_case_insensitive_string_slice(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let s0 = b"Alpha";
    let s1 = b"Bravo";
    let entries: [u64; 4] = [s0.as_ptr() as u64, 5, s1.as_ptr() as u64, 5];
    let this = [entries.as_ptr() as u64, 2];
    let needle = [b"bRAVo".as_ptr() as u64, 5];
    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    f_orig(
        this.as_ptr() as usize,
        out_o.as_mut_ptr() as usize,
        needle.as_ptr() as usize,
    );
    f_new(
        this.as_ptr() as usize,
        out_n.as_mut_ptr() as usize,
        needle.as_ptr() as usize,
    );
    crate::diff_eq!(out_o, out_n, "find_ci_hit");
    Ok(1)
}

static mut TAIL_A8_ARENA: [u64; 64] = [0; 64];
static mut TAIL_A8_ARENA_OFF: usize = 0;

unsafe extern "win64" fn fake_alloc_tail_a8(size: u64, _align: u32) -> u64 {
    let aligned = ((size as usize) + 7) & !7;
    let base = core::ptr::addr_of_mut!(TAIL_A8_ARENA) as *mut u8;
    let ptr = base.add(TAIL_A8_ARENA_OFF) as u64;
    TAIL_A8_ARENA_OFF += aligned;
    ptr
}

pub unsafe fn diff_d3d_render_graph_node_compare(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes = [0u8; 0x38 * 3];
    let nodes_base = nodes.as_mut_ptr() as usize;
    wr::<u64>(nodes_base + 0x28, 100);
    wr::<u64>(nodes_base + 0x38 + 0x28, 200);
    wr::<u64>(nodes_base + 0x70 + 0x28, 300);

    let vec = [nodes_base as u64, 3u64];
    let memo = [10u16, 20u16, 10u16];
    let solver = [
        vec.as_ptr() as u64,
        0u64,
        0u64,
        memo.as_ptr() as u64,
        3u64,
    ];
    let this = [solver.as_ptr() as u64, vec.as_ptr() as u64];

    let mut cases = 0u32;
    for (a, b) in [(0u64, 1u64), (1, 0), (0, 2), (2, 0), (0, 0)] {
        let ro = f_orig(this.as_ptr() as usize, a, b);
        let rn = f_new(this.as_ptr() as usize, a, b);
        crate::diff_eq!(ro, rn, "render_graph_node_compare");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_render_graph_compute_max_child_depth(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes = [0u8; 0x38 * 3];
    let nodes_base = nodes.as_mut_ptr() as usize;
    let edge2 = [nodes_base as u64 + 0x70, 0u64, 3u64];
    let edge1 = [nodes_base as u64 + 0x38, edge2.as_ptr() as u64, 2u64];
    wr::<u64>(nodes_base + 0x30, edge1.as_ptr() as u64);

    let vec = [nodes_base as u64, 3u64];
    let memo = [0u16, 5u16, 10u16];
    let solver = [
        vec.as_ptr() as u64,
        0u64,
        0u64,
        memo.as_ptr() as u64,
        3u64,
    ];

    let mut cases = 0u32;
    for idx in [0u64, 1u64] {
        let ro = f_orig(solver.as_ptr() as usize, idx);
        let rn = f_new(solver.as_ptr() as usize, idx);
        crate::diff_eq!(ro, rn, "compute_max_child_depth");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_render_graph_eval_node_depth_ptr(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes = [0u8; 0x70];
    let nodes_base = nodes.as_mut_ptr() as usize;
    let edge = [nodes_base as u64 + 0x38, 0u64, 4u64];
    wr::<u64>(nodes_base + 0x30, edge.as_ptr() as u64);
    let vec = [nodes_base as u64, 2u64];

    let mut stack_o = [0u64; 4];
    let mut memo_o = [0xffffu16, 7u16];
    let mut solver_o = [
        vec.as_ptr() as u64,
        stack_o.as_mut_ptr() as u64,
        4u64 << 32,
        memo_o.as_mut_ptr() as u64,
        2u64,
    ];
    let mut stack_n = [0u64; 4];
    let mut memo_n = [0xffffu16, 7u16];
    let mut solver_n = [
        vec.as_ptr() as u64,
        stack_n.as_mut_ptr() as u64,
        4u64 << 32,
        memo_n.as_mut_ptr() as u64,
        2u64,
    ];

    let ro = f_orig(solver_o.as_mut_ptr() as usize, nodes_base as u64);
    let rn = f_new(solver_n.as_mut_ptr() as usize, nodes_base as u64);
    crate::diff_eq!(ro, rn, "eval_node_depth_ptr_ret");
    crate::diff_eq!(memo_o, memo_n, "eval_node_depth_ptr_memo");
    Ok(2)
}

pub unsafe fn diff_d3d_render_graph_depth_solver_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x14241c0a0, fake_alloc_tail_a8 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut cases = 0u32;
    for count in [0u64, 3u64] {
        let graph_vec = [0x1000u64, count];
        let mut solver_o = [0xaa55_aa55_aa55_aa55u64; 5];
        let mut solver_n = [0xaa55_aa55_aa55_aa55u64; 5];

        TAIL_A8_ARENA = [0; 64];
        TAIL_A8_ARENA_OFF = 0;
        let ro = f_orig(solver_o.as_mut_ptr() as usize, graph_vec.as_ptr() as usize);
        let arena_o = TAIL_A8_ARENA;

        TAIL_A8_ARENA = [0; 64];
        TAIL_A8_ARENA_OFF = 0;
        let rn = f_new(solver_n.as_mut_ptr() as usize, graph_vec.as_ptr() as usize);
        let arena_n = TAIL_A8_ARENA;

        crate::diff_eq!(ro, solver_o.as_mut_ptr() as u64, "depth_solver_init_ret_o");
        crate::diff_eq!(rn, solver_n.as_mut_ptr() as u64, "depth_solver_init_ret_n");
        crate::diff_eq!(solver_o, solver_n, "depth_solver_init_struct");
        crate::diff_eq!(arena_o, arena_n, "depth_solver_init_arena");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_render_graph_builder_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x14241c0a0, fake_alloc_tail_a8 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u64, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut cases = 0u32;
    for count in [0u64, 4u64] {
        let vec = [0x2000u64, count];
        let mut builder_o = [0xaa55_aa55_aa55_aa55u64; 4];
        let mut builder_n = [0xaa55_aa55_aa55_aa55u64; 4];

        TAIL_A8_ARENA = [0; 64];
        TAIL_A8_ARENA_OFF = 0;
        let ro = f_orig(builder_o.as_mut_ptr() as usize, 0x9999, vec.as_ptr() as usize);
        let arena_o = TAIL_A8_ARENA;

        TAIL_A8_ARENA = [0; 64];
        TAIL_A8_ARENA_OFF = 0;
        let rn = f_new(builder_n.as_mut_ptr() as usize, 0x9999, vec.as_ptr() as usize);
        let arena_n = TAIL_A8_ARENA;

        crate::diff_eq!(ro, builder_o.as_mut_ptr() as u64, "builder_init_ret_o");
        crate::diff_eq!(rn, builder_n.as_mut_ptr() as u64, "builder_init_ret_n");
        crate::diff_eq!(builder_o, builder_n, "builder_init_struct");
        crate::diff_eq!(arena_o, arena_n, "builder_init_arena");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_destroy_and_return_count(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut backing_o = [0u64; 8];
    let mut backing_n = [0u64; 8];
    let mut vec_o = [backing_o.as_mut_ptr() as u64, (4u64 << 32) | 1u64];
    let mut vec_n = [backing_n.as_mut_ptr() as u64, (4u64 << 32) | 1u64];
    let item = [0x1122_3344_5566_7788u64, 0x99aa_bbcc_ddee_ff00u64];

    let ro = f_orig(vec_o.as_mut_ptr() as usize, item.as_ptr() as usize);
    let rn = f_new(vec_n.as_mut_ptr() as usize, item.as_ptr() as usize);
    crate::diff_eq!(ro, rn, "destroy_and_return_count_ret");
    crate::diff_eq!(vec_o[1], vec_n[1], "destroy_and_return_count_len");
    crate::diff_eq!(backing_o, backing_n, "destroy_and_return_count_backing");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x1420a7a70, "d3d::renderGraphNodeCompare_1420a7a70", crate::ns::rend::d3d_tail_a8::d3d_render_graph_node_compare as *const (), Throws::Discard, crate::ns::rend::d3d_tail_a8::diff_d3d_render_graph_node_compare),
// REGISTRY: Replacement::verified_throws(0x1420a7cf0, "d3d::renderGraphComputeMaxChildDepth_1420a7cf0", crate::ns::rend::d3d_tail_a8::d3d_render_graph_compute_max_child_depth as *const (), Throws::Discard, crate::ns::rend::d3d_tail_a8::diff_d3d_render_graph_compute_max_child_depth),
// REGISTRY: Replacement::verified_throws(0x1420a7ee0, "d3d::renderGraphEvalNodeDepthPtr_1420a7ee0", crate::ns::rend::d3d_tail_a8::d3d_render_graph_eval_node_depth_ptr as *const (), Throws::Discard, crate::ns::rend::d3d_tail_a8::diff_d3d_render_graph_eval_node_depth_ptr),
// REGISTRY: Replacement::verified_throws(0x1420a7f10, "d3d::renderGraphDepthSolverInit_1420a7f10", crate::ns::rend::d3d_tail_a8::d3d_render_graph_depth_solver_init as *const (), Throws::Discard, crate::ns::rend::d3d_tail_a8::diff_d3d_render_graph_depth_solver_init),
// REGISTRY: Replacement::verified_throws(0x1420a8af0, "d3d::renderGraphBuilderInit_1420a8af0", crate::ns::rend::d3d_tail_a8::d3d_render_graph_builder_init as *const (), Throws::Discard, crate::ns::rend::d3d_tail_a8::diff_d3d_render_graph_builder_init),
// REGISTRY: Replacement::verified(0x1420a8cf0, "d3d::destroyAndReturnCount_1420a8cf0", crate::ns::rend::d3d_tail_a8::d3d_destroy_and_return_count as *const (), crate::ns::rend::d3d_tail_a8::diff_d3d_destroy_and_return_count),

