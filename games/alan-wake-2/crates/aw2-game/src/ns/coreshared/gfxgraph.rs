//! Per-node update of `coregame::gfxgraph::updateActiveInstances` (system s796), the serial
//! main-thread hot path: wrapper 0x1410c2c40 -> 0x141f444f0 -> 0x141f450f0 -> 0x1420474e0.
//!
//! Layouts used here (recovered from the disassembly of the functions below; names inferred):
//! - update context `ctx` (built on 0x141f450f0's stack, at its `rbp+0x2a0`): `+0x10` and
//!   `+0x358` are the two binding sources passed on to 0x142046590 / 0x1420460d0; `+0x360`
//!   points at the shared event queue.
//! - event queue `{u64 head; u64 tail; u64 keys[64]}`: pop reads `keys[tail & 63]` and sets
//!   `tail = min(tail + 1, head)` (unsigned).
//! - node: `+0x00` graph definition, `+0x08` object whose `+0x10` is the handler table
//!   (`fn(ctx, node, &u32 id)`, indexed by `id & 0xffff`, null = no handler), `+0x10` value
//!   slots (0x50 bytes each, variant tag at `+0x40`), `+0x60..+0x90` a Swiss-table key set
//!   (`ctrl`, `slots`, `size`, `capacity mask`, -, `growth_left`), `+0x90` the node's own event
//!   queue.
//! - definition: `+0x70/+0x78` event entries `{u64 key; u32* ids; u32 count}` (0x18 bytes);
//!   `+0x90/+0x98` and `+0xa0/+0xa8` binding lists `{u64 key; u64 slot; u8 dirty; u8 notify}`
//!   (0x18 bytes) that use the `ctx+0x358` and `ctx+0x10` sources respectively.

use crate::exe::{self, rd, wr};
use std::cell::{Cell, RefCell};

const UPDATE_NODE_VA: u64 = 0x1_4204_74e0;
const DRAIN_NODE_EVENTS_VA: u64 = 0x1_4204_5ad0;
const DISPATCH_ENTRY_IDS_VA: u64 = 0x1_4204_5be0;
const NOTIFY_CHANGED_BINDINGS_VA: u64 = 0x1_4204_5c60;
const APPLY_DIRTY_BINDINGS_VA: u64 = 0x1_4204_5ff0;
const APPLY_BINDING_VALUE_VA: u64 = 0x1_4204_6590;

/// `notify(3, node, source, binding)` for a binding whose key is in the node's changed set.
const NOTIFY_BINDING_VA: u64 = 0x1_4204_60d0;
/// Swiss-table reset for capacities above 0x7f (tail call with `&node.set`).
const KEYSET_RESET_VA: u64 = 0x1_401d_e4e0;
/// `j_memset` import thunk.
const J_MEMSET_VA: u64 = 0x1_4183_acc2;
/// `lookup(source, key) -> value*` (0x48-byte variant, tag at `+0x40`).
const VALUE_LOOKUP_VA: u64 = 0x1_4202_bd50;
/// Variant destroy visitor `(tag + 1, &functor, slot)`.
const SLOT_DESTROY_VA: u64 = 0x1_410c_38f0;
/// Assign for value tag 0xb when the slot already holds tag 0x37.
const SLOT_ASSIGN_REF_VA: u64 = 0x1_403b_3290;
/// `on_slot_changed(node, slot index)` after a tag-0xb assignment.
const SLOT_CHANGED_VA: u64 = 0x1_4204_6d60;
/// Ref-count manager callback on the 0 -> 1 transition.
const REF_FIRST_VA: u64 = 0x1_4241_d6f0;
/// Destructor of the temporary (null) ref holder.
const REF_HOLDER_DTOR_VA: u64 = 0x1_4028_0170;

/// Event key whose entry is dispatched once after the queues are drained.
const POST_EVENT_KEY: u64 = (-0x2043_bf57_e5b1_22f4i64) as u64;

/// Value tags 1..=9 -> slot assign function (`assign(slot, value)`).
const ASSIGN_BY_TAG: [(u8, u64); 9] = [
    (0x1, 0x1_411c_aea0),
    (0x2, 0x1_411c_af00),
    (0x4, 0x1_411c_afc0),
    (0x6, 0x1_411c_b080),
    (0x8, 0x1_411c_b160),
    (0x3, 0x1_411c_af60),
    (0x5, 0x1_411c_b020),
    (0x7, 0x1_411c_b0f0),
    (0x9, 0x1_411c_b1d0),
];

/// Remaining tags: `get_if<T>(value)` (returns `value` when its tag is 0xc..=0x14) -> assign.
const ASSIGN_BY_GETTER: [(u64, u64); 9] = [
    (0x1_411c_b640, 0x1_411c_b660),
    (0x1_4106_8ce0, 0x1_411c_2a20),
    (0x1_411c_b6f0, 0x1_411c_b710),
    (0x1_411c_b770, 0x1_411c_b790),
    (0x1_411c_b7f0, 0x1_411c_b810),
    (0x1_411c_b870, 0x1_411c_b890),
    (0x1_411c_b8f0, 0x1_411c_b910),
    (0x1_411b_9190, 0x1_411c_b970),
    (0x1_411b_91b0, 0x1_411c_b9e0),
];

/// First event entry of the node's definition with `key` (linear scan of `def+0x70`).
#[inline(always)]
unsafe fn find_event_entry(node: usize, key: u64) -> Option<usize> {
    let def = rd::<usize>(node);
    let mut e = rd::<usize>(def + 0x70);
    let end = e.wrapping_add(rd::<u32>(def + 0x78) as usize * 0x18);
    while e != end {
        if rd::<u64>(e) == key {
            return Some(e);
        }
        e = e.wrapping_add(0x18);
    }
    None
}

/// Call the handler of every id of an event entry (the loop shared by 0x1420474e0,
/// 0x142045ad0 and 0x142045be0). The id is copied to a local and passed by pointer.
#[inline(always)]
unsafe fn dispatch_ids(ctx: usize, node: usize, entry: usize) {
    let mut p = rd::<usize>(entry + 0x08);
    let end = p.wrapping_add(rd::<u32>(entry + 0x10) as usize * 4);
    while p != end {
        let mut id = rd::<u32>(p);
        let table = rd::<usize>(rd::<usize>(node + 0x08) + 0x10);
        let h = rd::<usize>(table.wrapping_add((id & 0xffff) as usize * 8));
        if h != 0 {
            exe::call_va!(h, (ctx, node, &mut id as *mut u32 as usize));
        }
        p = p.wrapping_add(4);
    }
}

/// Pop one key from an event queue at `q`.
#[inline(always)]
unsafe fn queue_pop(q: usize, head: u64, tail: u64) -> u64 {
    let key = rd::<u64>(q + 0x10 + (tail & 0x3f) as usize * 8);
    let next = tail.wrapping_add(1);
    wr::<u64>(q + 0x08, if next < head { next } else { head });
    key
}

/// Update one gfxgraph node: apply dirty bindings, drain the shared event queue, dispatch the
/// post-drain event, notify changed bindings.
///
/// Original: 0x1420474e0 (347 bytes; no FH4 handler; Throws::Discard: 0x142045ff0 can throw).
/// Called from 0x141f450f0 with its stack context. Calls 0x142045ff0(ctx, node); then while
/// the queue at `[ctx+0x360]` is non-empty pops a key and, if the definition has an event
/// entry for it, calls the entry's handlers and 0x142045ad0(ctx, node) (drain the node's own
/// queue). Then, if an entry with key 0xdfbc40a81a4edd0c exists, 0x142045be0(ctx, node,
/// entry) and 0x142045ad0. Ends with a tail call to 0x142045c60(ctx, node).
pub unsafe extern "win64" fn update_node(ctx: usize, node: usize) {
    exe::call_va!(APPLY_DIRTY_BINDINGS_VA, (ctx, node));
    loop {
        let q = rd::<usize>(ctx + 0x360);
        let head = rd::<u64>(q);
        let tail = rd::<u64>(q + 0x08);
        if head == tail {
            break;
        }
        let key = queue_pop(q, head, tail);
        if let Some(e) = find_event_entry(node, key) {
            dispatch_ids(ctx, node, e);
            exe::call_native!(DRAIN_NODE_EVENTS_VA, drain_node_events, (ctx, node) -> ());
        }
    }
    if let Some(e) = find_event_entry(node, POST_EVENT_KEY) {
        exe::call_native!(DISPATCH_ENTRY_IDS_VA, dispatch_entry_ids, (ctx, node, e) -> ());
        exe::call_native!(DRAIN_NODE_EVENTS_VA, drain_node_events, (ctx, node) -> ());
    }
    exe::call_native!(NOTIFY_CHANGED_BINDINGS_VA, notify_changed_bindings, (ctx, node) -> ());
}

/// Drain the node's own event queue (`node+0x90`), dispatching each key's handlers.
///
/// Original: 0x142045ad0 (263 bytes; leaf apart from the indirect handler calls;
/// Throws::No). After each popped key the iteration counter is checked: once it was already
/// above 100 (i.e. after the 102nd key) the queue is reset to `head = tail = 0x40` and the
/// function returns.
pub unsafe extern "win64" fn drain_node_events(ctx: usize, node: usize) {
    let q = node + 0x90;
    let mut head = rd::<u64>(q);
    let mut tail = rd::<u64>(q + 0x08);
    if head == tail {
        return;
    }
    let mut n: u64 = 0;
    loop {
        let key = queue_pop(q, head, tail);
        if let Some(e) = find_event_entry(node, key) {
            dispatch_ids(ctx, node, e);
        }
        let old = n;
        n += 1;
        if old > 100 {
            wr::<u64>(q, 0x40);
            wr::<u64>(q + 0x08, 0x40);
            return;
        }
        head = rd::<u64>(q);
        tail = rd::<u64>(q + 0x08);
        if head == tail {
            return;
        }
    }
}

/// Call the handlers of one event entry.
///
/// Original: 0x142045be0 (117 bytes; Throws::No: only the indirect handler calls).
pub unsafe extern "win64" fn dispatch_entry_ids(ctx: usize, node: usize, entry: usize) {
    dispatch_ids(ctx, node, entry);
}

/// Apply every binding flagged dirty (`+0x10 != 0`) of both binding lists of the node's
/// definition via 0x142046590(node, source, binding).
///
/// Original: 0x142045ff0 (168 bytes; Throws::Discard: 0x142046590 can throw). The first list
/// (`def+0x90`) uses source `[ctx+0x358]`, the second (`def+0xa0`, definition re-read from
/// `node`) uses `[ctx+0x10]`; the source is re-read for every call, the list bounds once.
pub unsafe extern "win64" fn apply_dirty_bindings(ctx: usize, node: usize) {
    for (list, src) in [(0x90usize, 0x358usize), (0xa0, 0x10)] {
        let def = rd::<usize>(node);
        let mut b = rd::<usize>(def + list);
        let end = b.wrapping_add(rd::<u32>(def + list + 8) as usize * 0x18);
        while b != end {
            if rd::<u8>(b + 0x10) != 0 {
                exe::call_va!(APPLY_BINDING_VALUE_VA, (node, rd::<u64>(ctx + src), b));
            }
            b = b.wrapping_add(0x18);
        }
    }
}

/// 64-byte copy in four 16-byte steps (the original's `vmovups` pairs).
#[inline(always)]
unsafe fn copy_value64(dst: usize, src: usize) {
    for i in 0..4 {
        wr::<u128>(dst + i * 16, rd::<u128>(src + i * 16));
    }
}

/// Destroy the slot's current variant alternative through 0x1410c38f0.
#[inline(always)]
unsafe fn destroy_slot(slot: usize) {
    let mut functor = 0u64;
    let idx = (rd::<i8>(slot + 0x40) as i64).wrapping_add(1);
    exe::call_va!(SLOT_DESTROY_VA, (idx, &mut functor as *mut u64 as usize, slot));
}

/// Look up a binding's source value and assign it to the node's value slot.
///
/// Original: 0x142046590 (772 bytes; no FH4 handler; Throws::Discard: callees can throw).
/// `slot = [node+0x10] + binding.slot * 0x50` (computed before the lookup);
/// `value = 0x14202bd50(source, binding.key)`; nothing when null. Dispatch on `value.tag`
/// (`+0x40`, u8): tags 1..=9 call their assign function `(slot, value)`; tag 0xa copies the
/// 64-byte payload into the slot as tag 0x19 (destroying a different alternative first and
/// marking the slot 0xff meanwhile); tag 0xb assigns a ref-counted pointer as tag 0x37
/// (0x1403b3290 when already 0x37, else `lock xadd [obj+0x24], 1` with 0x14241d6f0(mgr, obj)
/// on 0 -> 1, destroy, store, destroy a null holder via 0x140280170) and then calls
/// 0x142046d60(node, binding.slot). Any other tag tries the `get_if` chain in the original's
/// order (the getters are called with `rcx = value`, which the original leaves live) and calls
/// the first matching assign.
pub unsafe extern "win64" fn apply_binding_value(node: usize, source: u64, binding: usize) {
    let slot = rd::<u64>(binding + 0x08).wrapping_mul(0x50).wrapping_add(rd::<u64>(node + 0x10)) as usize;
    let value = exe::call_va!(VALUE_LOOKUP_VA, (source, rd::<u64>(binding)) -> usize);
    if value == 0 {
        return;
    }
    let tag = rd::<u8>(value + 0x40);
    if let Some(&(_, f)) = ASSIGN_BY_TAG.iter().find(|&&(t, _)| t == tag) {
        exe::call_va!(f, (slot, value));
        return;
    }
    match tag {
        0xa => {
            if rd::<u8>(slot + 0x40) == 0x19 {
                copy_value64(slot, value);
            } else {
                destroy_slot(slot);
                wr::<u8>(slot + 0x40, 0xff);
                copy_value64(slot, value);
                wr::<u8>(slot + 0x40, 0x19);
            }
        }
        0xb => {
            if rd::<u8>(slot + 0x40) == 0x37 {
                exe::call_va!(SLOT_ASSIGN_REF_VA, (slot, value));
            } else {
                let obj = rd::<usize>(value);
                if obj != 0 {
                    let old = crate::simd_ops::atomic_xadd_u32((obj + 0x24) as u64, 1);
                    if old.wrapping_add(1) == 1 {
                        let mgr = rd::<usize>(obj + 0x28);
                        if mgr != 0 {
                            exe::call_va!(REF_FIRST_VA, (mgr, obj));
                        }
                    }
                }
                destroy_slot(slot);
                wr::<usize>(slot, obj);
                let mut holder = 0u64;
                wr::<u8>(slot + 0x40, 0x37);
                exe::call_va!(REF_HOLDER_DTOR_VA, (&mut holder as *mut u64 as usize));
            }
            exe::call_va!(SLOT_CHANGED_VA, (node, rd::<u64>(binding + 0x08)));
        }
        _ => {
            for &(get, set) in ASSIGN_BY_GETTER.iter() {
                let v = exe::call_va!(get, (value) -> usize);
                if v != 0 {
                    exe::call_va!(set, (slot, v));
                    return;
                }
            }
        }
    }
}

/// FNV-1a over the 8 key bytes, then the 128-bit multiply-fold by 0xde5fb9d2630458e9.
#[inline(always)]
fn keyset_hash(key: u64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in key.to_le_bytes() {
        h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
    }
    let p = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u128);
    (p as u64).wrapping_add((p >> 64) as u64)
}

/// Whether the node's changed-key set (`node+0x60`) contains `key`: Swiss-table probe with
/// 16-byte groups; "not found" is the end position `ctrl + mask`, compared like the original.
#[inline(always)]
unsafe fn keyset_contains(node: usize, key: u64) -> bool {
    let mask = rd::<u64>(node + 0x78);
    let ctrl = rd::<usize>(node + 0x60);
    let h = keyset_hash(key);
    let h2 = (h as u8) & 0x7f;
    let mut pos = (h >> 7) & mask;
    let mut stride = 0u64;
    let found = 'probe: loop {
        let g = rd::<[u8; 16]>(ctrl.wrapping_add(pos as usize));
        let mut m = 0u32;
        let mut empty = false;
        for (i, &c) in g.iter().enumerate() {
            m |= ((c == h2) as u32) << i;
            empty |= c == 0x80;
        }
        if m != 0 {
            let slots = rd::<usize>(node + 0x68);
            while m != 0 {
                let idx = (m.trailing_zeros() as u64).wrapping_add(pos) & mask;
                if rd::<u64>(slots.wrapping_add(idx as usize * 8)) == key {
                    break 'probe ctrl.wrapping_add(idx as usize);
                }
                m &= m - 1;
            }
        }
        if empty {
            break ctrl.wrapping_add(mask as usize);
        }
        stride = stride.wrapping_add(0x10);
        pos = pos.wrapping_add(stride) & mask;
    };
    found != ctrl.wrapping_add(mask as usize)
}

/// Notify every binding (flag `+0x11`) whose key is in the node's changed set via
/// 0x1420460d0(3, node, source, binding), then clear the set.
///
/// Original: 0x142045c60 (903 bytes; Throws::No: no throwing direct callee). Lists and
/// sources as in [`apply_dirty_bindings`]; the set header is re-read per binding. Clearing
/// when `size != 0`: capacity > 0x7f tail-calls 0x1401de4e0(node + 0x60); otherwise (capacity
/// != 0) `size = 0`, `memset(ctrl, 0x80, cap + 16)` through `j_memset`, `ctrl[cap] = 0xff`,
/// `growth_left = cap - cap / 8 - size`.
pub unsafe extern "win64" fn notify_changed_bindings(ctx: usize, node: usize) {
    for (list, src) in [(0x90usize, 0x358usize), (0xa0, 0x10)] {
        let def = rd::<usize>(node);
        let mut b = rd::<usize>(def + list);
        let end = b.wrapping_add(rd::<u32>(def + list + 8) as usize * 0x18);
        while b != end {
            if rd::<u8>(b + 0x11) != 0 && keyset_contains(node, rd::<u64>(b)) {
                exe::call_va!(NOTIFY_BINDING_VA, (3u32, node, rd::<u64>(ctx + src), b));
            }
            b = b.wrapping_add(0x18);
        }
    }
    if rd::<u64>(node + 0x70) == 0 {
        return;
    }
    let cap = rd::<u64>(node + 0x78);
    if cap > 0x7f {
        exe::call_va!(KEYSET_RESET_VA, (node + 0x60));
        return;
    }
    if cap == 0 {
        return;
    }
    let ctrl = rd::<usize>(node + 0x60);
    wr::<u64>(node + 0x70, 0);
    exe::call_va!(J_MEMSET_VA, (ctrl, 0xffff_ff80u32, cap.wrapping_add(0x10)));
    wr::<u8>(rd::<usize>(node + 0x60).wrapping_add(cap as usize), 0xff);
    let cap = rd::<u64>(node + 0x78);
    wr::<u64>(node + 0x88, cap.wrapping_sub(cap >> 3).wrapping_sub(rd::<u64>(node + 0x70)));
}

/// `m::View` copy constructor `(dst, src) -> dst` (0x2f0-byte object).
const VIEW_COPY_VA: u64 = 0x1_4242_da50;
/// `m::View::vftable` (stored by the inlined destructor of the temporary copy).
const VIEW_VFTABLE_VA: u64 = 0x1_4330_bca8;
/// Global u16 index into `world+0x580e0`.
const WORLD_SLOT_INDEX_VA: u64 = 0x1_43cd_b540;
const NODE_PREPARE_VA: u64 = 0x1_4204_7650;
const NODE_ACTIVATE_VA: u64 = 0x1_4204_73d0;

/// Build the update context for one active gfxgraph instance and run the node passes.
///
/// Original: 0x141f450f0 (821 bytes; FH4 with a one-entry unwind map; Throws::Caught by the
/// lint rule). `inst` is the per-instance column set (`+0x00` 0x28-byte rows, `+0x08` 0x10-byte
/// rows whose `+8` is the node, `+0x10` 0x70-byte rows, `+0x18` u32 column, `+0x30` row index),
/// `view` a `{u64 x3; pad; m::View (0x2f0); trailer}` block. The context (0x380 bytes, the
/// original's `rbp+0x2a0`) gets the view header (0x18 bytes), a copy of the view made through a
/// temporary (two calls of 0x14242da50), the trailer fields `+0x310..+0x348` with their original
/// widths, and `inst`, `row28[0]`, `row28[0x18]`, `&row70`, `row28[8]`, `row28[0x10]` at
/// `+0x350..+0x380`. Bytes the original leaves uninitialized (`+0x18`, `+0x32c`, `+0x348`) are
/// zero here. The temporary's vptr is reset to `m::View::vftable` (inlined destructor).
/// Then, with `active = row28.byte[0x20] && u32col[i] == 0` and
/// `flag = [[world+0x580e0] + [0x143cdb540]*8].byte[0x1c]`, calls 0x142047650 / 0x1420473d0 /
/// 0x1420474e0 `(ctx, node)` depending on `node.byte[0x2f8]` (re-read before each test), in
/// the original's branch order (including the second 0x1420473d0 call when `flag && active`
/// and the byte is still clear).
/// The only unwind action (state 0, around the node calls) is 0x1410bddf0(ctx), which stores
/// the vftable into the frame-local context: nothing observable outside the dying frame, so a
/// throw needs no extra handling here (the frame holds only plain byte buffers).
pub unsafe extern "win64" fn update_instance(inst: usize, world: usize, view: usize) {
    let i = rd::<u64>(inst + 0x30) as usize;
    let row28 = rd::<usize>(inst).wrapping_add(i.wrapping_mul(0x28));
    let row10 = rd::<usize>(inst + 0x08).wrapping_add(i.wrapping_mul(0x10));
    let active = rd::<u8>(row28 + 0x20) != 0 && rd::<u32>(rd::<usize>(inst + 0x18).wrapping_add(i.wrapping_mul(4))) == 0;
    let row70 = i.wrapping_mul(0x70).wrapping_add(rd::<usize>(inst + 0x10));
    let hdr = [rd::<u64>(view), rd::<u64>(view + 0x08), rd::<u64>(view + 0x10)];
    let mut tmp = [0u64; 0x2f0 / 8];
    let mut ctx = [0u64; 0x380 / 8];
    let t = tmp.as_mut_ptr() as usize;
    let c = ctx.as_mut_ptr() as usize;
    exe::call_va!(VIEW_COPY_VA, (t, view + 0x20));
    let tr_310 = rd::<u64>(view + 0x310);
    let tr_318 = rd::<u64>(view + 0x318);
    let tr_320 = rd::<u64>(view + 0x320);
    let tr_328 = rd::<u32>(view + 0x328);
    let tr_330 = rd::<u64>(view + 0x330);
    let tr_338 = rd::<u32>(view + 0x338);
    let tr_33c = rd::<u32>(view + 0x33c);
    let tr_340 = rd::<u32>(view + 0x340);
    let tr_344 = rd::<u32>(view + 0x344);
    let r_00 = rd::<u64>(row28);
    let r_18 = rd::<u64>(row28 + 0x18);
    let r_08 = rd::<u64>(row28 + 0x08);
    let r_10 = rd::<u64>(row28 + 0x10);
    wr::<u64>(c, hdr[0]);
    wr::<u64>(c + 0x08, hdr[1]);
    wr::<u64>(c + 0x10, hdr[2]);
    exe::call_va!(VIEW_COPY_VA, (c + 0x20, t));
    wr::<u64>(c + 0x310, tr_310);
    wr::<u64>(c + 0x318, tr_318);
    wr::<u64>(c + 0x320, tr_320);
    wr::<u32>(c + 0x328, tr_328);
    wr::<u64>(c + 0x330, tr_330);
    wr::<u32>(c + 0x338, tr_338);
    wr::<u32>(c + 0x33c, tr_33c);
    wr::<u32>(c + 0x340, tr_340);
    wr::<u32>(c + 0x344, tr_344);
    wr::<usize>(c + 0x350, inst);
    wr::<u64>(c + 0x358, r_00);
    wr::<u64>(c + 0x360, r_18);
    wr::<usize>(c + 0x368, row70);
    wr::<u64>(c + 0x370, r_08);
    wr::<u64>(c + 0x378, r_10);
    wr::<usize>(t, exe::ptr::<u8>(VIEW_VFTABLE_VA) as usize);
    let slot = rd::<u16>(exe::ptr::<u16>(WORLD_SLOT_INDEX_VA) as usize) as usize;
    let flag = rd::<u8>(rd::<usize>(rd::<usize>(world + 0x580e0) + slot * 8) + 0x1c) != 0;
    let node = || rd::<usize>(row10 + 0x08);
    let pending = |n: usize| rd::<u8>(n + 0x2f8) != 0;
    // Labels of the original: L_b1 (0x141f453b1), L_cf (0x141f453cf), L_eb (0x141f453eb).
    let to_b1 = if flag {
        let n = node();
        if pending(n) {
            exe::call_va!(NODE_PREPARE_VA, (c, n));
        }
        if active {
            exe::call_va!(NODE_ACTIVATE_VA, (c, node()));
        }
        active
    } else {
        active
    };
    let n = node();
    if to_b1 {
        if !pending(n) {
            exe::call_va!(NODE_ACTIVATE_VA, (c, n));
        }
    } else if pending(n) {
        exe::call_va!(NODE_PREPARE_VA, (c, n));
    }
    let n = node();
    if pending(n) {
        exe::call_va!(UPDATE_NODE_VA, (c, n));
    }
}

// ---------------------------------------------------------------------------------------------
// Difftests
//
// Every test builds its world in one arena, runs the original, restores the arena's initial
// bytes and runs the replacement on the same addresses; it compares the whole arena and the
// ordered log of callee/handler calls. Arenas are placed at +0 and +4 (under-aligned) offsets.

type LogEntry = [u64; 5];

thread_local! {
    static LOG: RefCell<Vec<LogEntry>> = const { RefCell::new(Vec::new()) };
    /// Remaining pushes the queue-pushing handlers/mocks may do in this run.
    static BUDGET: Cell<u32> = const { Cell::new(0) };
    /// Keys the pushing handlers choose from.
    static KEYS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    /// Value returned by the lookup mock (0 for the null key).
    static LOOKUP_RET: Cell<usize> = const { Cell::new(0) };
    /// Extra behavior switch for mocks (per test).
    static MODE: Cell<u32> = const { Cell::new(0) };
    /// (ctx, alternate definition) for the 0x142046590 mock.
    static APPLY_ENV: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
}

fn log(e: LogEntry) {
    LOG.with(|l| l.borrow_mut().push(e));
}

fn take_log() -> Vec<LogEntry> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

struct Arena {
    _buf: Vec<u64>,
    base: usize,
    len: usize,
}

impl Arena {
    fn new(len: usize, mis: usize) -> Arena {
        let mut buf = vec![0u64; len / 8 + 2];
        let base = buf.as_mut_ptr() as usize + mis;
        Arena { _buf: buf, base, len }
    }
    fn at(&self, off: usize) -> usize {
        self.base + off
    }
    unsafe fn bytes(&self) -> Vec<u8> {
        std::slice::from_raw_parts(self.base as *const u8, self.len).to_vec()
    }
    unsafe fn restore(&self, b: &[u8]) {
        std::ptr::copy_nonoverlapping(b.as_ptr(), self.base as *mut u8, self.len);
    }
}

/// Run `call` on the original and the replacement from the same arena state and compare the
/// arena bytes and the call logs. `reset` re-arms per-run thread-local state.
unsafe fn run_pair(arena: &Arena, reset: &dyn Fn(), call: &dyn Fn(bool), what: &str) -> Result<(), String> {
    let init = arena.bytes();
    let mut out = Vec::new();
    for orig in [true, false] {
        arena.restore(&init);
        reset();
        take_log();
        call(orig);
        out.push((arena.bytes(), take_log()));
    }
    let (o, n) = (&out[0], &out[1]);
    if o.1 != n.1 {
        let i = o.1.iter().zip(n.1.iter()).position(|(a, b)| a != b).unwrap_or(o.1.len().min(n.1.len()));
        return Err(format!(
            "call log mismatch at #{i} (orig {} calls, new {}): orig={:x?} new={:x?} for {what}",
            o.1.len(),
            n.1.len(),
            o.1.get(i),
            n.1.get(i)
        ));
    }
    if let Some(i) = o.0.iter().zip(n.0.iter()).position(|(a, b)| a != b) {
        return Err(format!("memory mismatch at arena+0x{i:x}: orig={:#x} new={:#x} for {what}", o.0[i], n.0[i]));
    }
    Ok(())
}

unsafe fn queue_push(q: usize, key: u64) {
    let head = rd::<u64>(q);
    wr::<u64>(q + 0x10 + (head & 0x3f) as usize * 8, key);
    wr::<u64>(q, head.wrapping_add(1));
}

fn take_budget() -> bool {
    BUDGET.with(|b| {
        let v = b.get();
        if v == 0 {
            false
        } else {
            b.set(v - 1);
            true
        }
    })
}

fn pick_key(id: u32) -> u64 {
    KEYS.with(|k| {
        let k = k.borrow();
        if k.is_empty() { 0 } else { k[(id >> 16) as usize % k.len()] }
    })
}

unsafe extern "win64" fn h_log(ctx: usize, node: usize, idp: usize) {
    log([1, ctx as u64, node as u64, rd::<u32>(idp) as u64, 0]);
}

unsafe extern "win64" fn h_push_ctx(ctx: usize, node: usize, idp: usize) {
    let id = rd::<u32>(idp);
    log([2, ctx as u64, node as u64, id as u64, 0]);
    if take_budget() {
        queue_push(rd::<usize>(ctx + 0x360), pick_key(id));
    }
}

unsafe extern "win64" fn h_push_node(ctx: usize, node: usize, idp: usize) {
    let id = rd::<u32>(idp);
    log([3, ctx as u64, node as u64, id as u64, 0]);
    if take_budget() {
        queue_push(node + 0x90, pick_key(id));
    }
}

// Arena layout of the event-dispatch world (0x1420474e0 / 0x142045ad0 / 0x142045be0).
const W_CTX: usize = 0x0000; // 0x370 bytes
const W_CTXQ: usize = 0x0400; // queue, 0x210 bytes
const W_NODE: usize = 0x0800; // 0x2a0 bytes (own queue at +0x90)
const W_DEF: usize = 0x0b00;
const W_ENTRIES: usize = 0x0c00; // up to 16 x 0x18
const W_IDS: usize = 0x0e00; // 16 x 8 ids
const W_HOLDER: usize = 0x1000;
const W_TABLE: usize = 0x1100; // 64 handlers
const W_SIZE: usize = 0x1400;
const W_HANDLERS: usize = 64;

/// Build the event-dispatch world. Returns the entry addresses.
unsafe fn build_event_world(a: &Arena, rng: &mut crate::difftest::Rng, with_post: bool, ctx_events: u32, node_events: u32) -> Vec<usize> {
    let keys: Vec<u64> = (0..6).map(|_| rng.next_u64()).collect();
    KEYS.with(|k| *k.borrow_mut() = keys.clone());
    wr::<u64>(a.at(W_CTX + 0x10), 0x1111_0000_0000_0010);
    wr::<u64>(a.at(W_CTX + 0x358), 0x1111_0000_0000_0358);
    wr::<usize>(a.at(W_CTX + 0x360), a.at(W_CTXQ));
    wr::<usize>(a.at(W_NODE), a.at(W_DEF));
    wr::<usize>(a.at(W_NODE + 0x08), a.at(W_HOLDER));
    wr::<usize>(a.at(W_HOLDER + 0x10), a.at(W_TABLE));
    for i in 0..W_HANDLERS {
        let h = match rng.below(20) {
            0..=4 => 0usize,
            5..=14 => h_log as *const () as usize,
            15..=17 => h_push_ctx as *const () as usize,
            _ => h_push_node as *const () as usize,
        };
        wr::<usize>(a.at(W_TABLE + i * 8), h);
    }
    // Entries: keys[0..4] (one may repeat), one unknown key, optional post-drain entry.
    let mut ekeys: Vec<u64> = keys[..4].to_vec();
    ekeys.push(rng.next_u64());
    if rng.below(3) == 0 {
        ekeys.push(keys[1]);
    }
    if with_post {
        let at = rng.below(ekeys.len() as u64 + 1) as usize;
        ekeys.insert(at, POST_EVENT_KEY);
    }
    let mut entries = Vec::new();
    for (i, &k) in ekeys.iter().enumerate() {
        let e = a.at(W_ENTRIES + i * 0x18);
        let ids = a.at(W_IDS + i * 32);
        let n = rng.below(6) as u32;
        for j in 0..n as usize {
            let id = ((rng.next_u32() & 0xffff) << 16) | rng.below(W_HANDLERS as u64) as u32;
            wr::<u32>(ids + j * 4, id);
        }
        wr::<u64>(e, k);
        wr::<usize>(e + 8, ids);
        wr::<u32>(e + 0x10, n);
        wr::<u32>(e + 0x14, rng.next_u32());
        entries.push(e);
    }
    wr::<usize>(a.at(W_DEF + 0x70), a.at(W_ENTRIES));
    wr::<u32>(a.at(W_DEF + 0x78), ekeys.len() as u32);
    // Queues: counters near a 64 boundary so the ring index wraps.
    for (q, n) in [(a.at(W_CTXQ), ctx_events), (a.at(W_NODE + 0x90), node_events)] {
        let tail = (rng.next_u64() & !0x3f) | (0x38 + rng.below(8));
        wr::<u64>(q + 8, tail);
        wr::<u64>(q, tail);
        for i in 0..64 {
            wr::<u64>(q + 0x10 + i * 8, rng.next_u64());
        }
        for _ in 0..n {
            let k = if rng.below(4) == 0 { rng.next_u64() } else { keys[rng.below(6) as usize] };
            queue_push(q, k);
        }
    }
    entries
}

unsafe extern "win64" fn m_apply_dirty(ctx: usize, node: usize) {
    log([0x10, ctx as u64, node as u64, 0, 0]);
    if MODE.with(|m| m.get()) & 1 != 0 {
        queue_push(rd::<usize>(ctx + 0x360), pick_key(0));
    }
}

unsafe extern "win64" fn m_drain(ctx: usize, node: usize) {
    let q = node + 0x90;
    log([0x11, ctx as u64, node as u64, rd::<u64>(q), rd::<u64>(q + 8)]);
}

unsafe extern "win64" fn m_dispatch_entry(ctx: usize, node: usize, e: usize) {
    log([0x12, ctx as u64, node as u64, e as u64, 0]);
}

unsafe extern "win64" fn m_notify_changed(ctx: usize, node: usize) {
    log([0x13, ctx as u64, node as u64, 0, 0]);
}

/// 0x1420474e0: callees 0x142045ff0/0x142045ad0/0x142045be0/0x142045c60 mocked (recorders;
/// the 0x142045ff0 mock optionally pushes a key first), real handler table with logging and
/// queue-pushing handlers.
pub unsafe fn diff_update_node(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(APPLY_DIRTY_BINDINGS_VA, m_apply_dirty as *const ())?;
    let _m2 = crate::difftest::Mock::new(DRAIN_NODE_EVENTS_VA, m_drain as *const ())?;
    let _m3 = crate::difftest::Mock::new(DISPATCH_ENTRY_IDS_VA, m_dispatch_entry as *const ())?;
    let _m4 = crate::difftest::Mock::new(NOTIFY_CHANGED_BINDINGS_VA, m_notify_changed as *const ())?;
    let mut rng = crate::difftest::Rng::new(0x1420_474e);
    let mut cases = 0u32;
    for round in 0..48u32 {
        for mis in [0usize, 4] {
            let a = Arena::new(W_SIZE, mis);
            let with_post = round % 3 != 1;
            let events = [0u32, 1, 3, 8][round as usize % 4];
            let node_events = rng.below(4) as u32;
            build_event_world(&a, &mut rng, with_post, events, node_events);
            let budget = [0u32, 3, 10][round as usize % 3];
            let mode = (round / 4) % 2;
            let (ctx, node) = (a.at(W_CTX), a.at(W_NODE));
            run_pair(
                &a,
                &|| {
                    BUDGET.with(|b| b.set(budget));
                    MODE.with(|m| m.set(mode));
                },
                &|orig| if orig { o(ctx, node) } else { n(ctx, node) },
                &format!("round {round} mis {mis} post {with_post} events {events} budget {budget} mode {mode}"),
            )?;
            cases += 1;
        }
    }
    MODE.with(|m| m.set(0));
    Ok(cases)
}

/// 0x142045ad0: no mocks; node queue with 0..~140 keys, handlers that push more (to reach the
/// 102-key cutoff), counters across the ring wrap, and a `head < tail` queue.
pub unsafe fn diff_drain_node_events(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1420_45ad);
    let mut cases = 0u32;
    for round in 0..40u32 {
        for mis in [0usize, 4] {
            let a = Arena::new(W_SIZE, mis);
            let events = [0u32, 1, 2, 5, 20, 60, 63, 64][round as usize % 8];
            let ctx_events = rng.below(3) as u32;
            build_event_world(&a, &mut rng, round % 2 == 0, ctx_events, events);
            let q = a.at(W_NODE + 0x90);
            if round % 10 == 9 {
                // head below tail: one pop, then tail = head.
                let t = rd::<u64>(q + 8);
                wr::<u64>(q, t.wrapping_sub(3));
            }
            let budget = [0u32, 5, 200, 40][round as usize % 4];
            let (ctx, node) = (a.at(W_CTX), a.at(W_NODE));
            run_pair(
                &a,
                &|| BUDGET.with(|b| b.set(budget)),
                &|orig| if orig { o(ctx, node) } else { n(ctx, node) },
                &format!("round {round} mis {mis} events {events} budget {budget}"),
            )?;
            cases += 1;
        }
    }
    Ok(cases)
}

/// 0x142045be0: no mocks; every entry of random worlds (0..5 ids, null/logging/pushing
/// handlers, ids with random upper halves).
pub unsafe fn diff_dispatch_entry_ids(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1420_45be);
    let mut cases = 0u32;
    for round in 0..8u32 {
        for mis in [0usize, 4] {
            let a = Arena::new(W_SIZE, mis);
            let entries = build_event_world(&a, &mut rng, true, 2, 2);
            let (ctx, node) = (a.at(W_CTX), a.at(W_NODE));
            for (ei, &e) in entries.iter().enumerate() {
                run_pair(
                    &a,
                    &|| BUDGET.with(|b| b.set(2)),
                    &|orig| if orig { o(ctx, node, e) } else { n(ctx, node, e) },
                    &format!("round {round} mis {mis} entry {ei}"),
                )?;
                cases += 1;
            }
        }
    }
    Ok(cases)
}

unsafe extern "win64" fn m_apply_binding(node: usize, src: u64, b: usize) {
    log([0x20, node as u64, src, b as u64, 0]);
    let mode = MODE.with(|m| m.get());
    let (ctx, alt_def) = APPLY_ENV.with(|e| e.get());
    if mode & 1 != 0 {
        // Sources are re-read per call.
        wr::<u64>(ctx + 0x358, rd::<u64>(ctx + 0x358).wrapping_add(1));
        wr::<u64>(ctx + 0x10, rd::<u64>(ctx + 0x10).wrapping_add(1));
    }
    if mode & 2 != 0 {
        // The second list is read from the definition the node points at after the first.
        wr::<usize>(node, alt_def);
    }
}

/// Fill a binding list of `n` 0x18-byte entries at `at`.
unsafe fn fill_bindings(at: usize, n: usize, rng: &mut crate::difftest::Rng, flag_off: usize) {
    for i in 0..n {
        let b = at + i * 0x18;
        wr::<u64>(b, rng.next_u64());
        wr::<u64>(b + 8, rng.below(8));
        wr::<u64>(b + 0x10, rng.next_u64());
        wr::<u8>(b + flag_off, if rng.below(3) == 0 { 0 } else { rng.next_u32() as u8 | 1 });
    }
}

/// 0x142045ff0: 0x142046590 mocked (recorder; optionally bumps the sources and swaps the
/// node's definition after its first call to check the re-reads).
pub unsafe fn diff_apply_dirty_bindings(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m = crate::difftest::Mock::new(APPLY_BINDING_VALUE_VA, m_apply_binding as *const ())?;
    let mut rng = crate::difftest::Rng::new(0x1420_45ff);
    let mut cases = 0u32;
    let result = (|| -> Result<(), String> {
        for round in 0..24u32 {
            for mis in [0usize, 4] {
                let a = Arena::new(W_SIZE, mis);
                let (ctx, node, def) = (a.at(W_CTX), a.at(W_NODE), a.at(W_DEF));
                wr::<u64>(ctx + 0x10, 0x2222_0000_0000_0010);
                wr::<u64>(ctx + 0x358, 0x2222_0000_0000_0358);
                wr::<usize>(node, def);
                let n1 = [0usize, 1, 4, 7][round as usize % 4];
                let n2 = [0usize, 3, 1, 6, 2][round as usize % 5];
                fill_bindings(a.at(W_ENTRIES), n1, &mut rng, 0x10);
                fill_bindings(a.at(W_ENTRIES + 0x100), n2, &mut rng, 0x10);
                fill_bindings(a.at(W_IDS), 3, &mut rng, 0x10);
                wr::<usize>(def + 0x90, a.at(W_ENTRIES));
                wr::<u32>(def + 0x98, n1 as u32);
                wr::<usize>(def + 0xa0, a.at(W_ENTRIES + 0x100));
                wr::<u32>(def + 0xa8, n2 as u32);
                // Alternate definition (at W_HOLDER): its second list is W_IDS.
                let alt = a.at(W_HOLDER);
                wr::<usize>(alt + 0x90, a.at(W_ENTRIES));
                wr::<u32>(alt + 0x98, 0);
                wr::<usize>(alt + 0xa0, a.at(W_IDS));
                wr::<u32>(alt + 0xa8, 3);
                let mode = round % 4;
                APPLY_ENV.with(|e| e.set((ctx, alt)));
                run_pair(
                    &a,
                    &|| MODE.with(|m| m.set(mode)),
                    &|orig| if orig { o(ctx, node) } else { n(ctx, node) },
                    &format!("round {round} mis {mis} n1 {n1} n2 {n2} mode {mode}"),
                )?;
                cases += 1;
            }
        }
        Ok(())
    })();
    MODE.with(|m| m.set(0));
    result.map(|_| cases)
}

unsafe extern "win64" fn m_lookup(src: u64, key: u64) -> usize {
    log([VALUE_LOOKUP_VA, src, key, 0, 0]);
    LOOKUP_RET.with(|r| r.get())
}

unsafe extern "win64" fn m_destroy(idx: i64, functor: usize, slot: usize) {
    log([SLOT_DESTROY_VA, idx as u64, rd::<u8>(functor) as u64, slot as u64, rd::<u64>(slot)]);
    for i in 0..0x40 {
        wr::<u8>(slot + i, 0xee);
    }
    wr::<u8>(slot + 0x40, 0x42);
}

unsafe extern "win64" fn m_assign_ref(slot: usize, v: usize) {
    log([SLOT_ASSIGN_REF_VA, slot as u64, v as u64, rd::<u8>(slot + 0x40) as u64, 0]);
}

unsafe extern "win64" fn m_slot_changed(node: usize, idx: u64) {
    log([SLOT_CHANGED_VA, node as u64, idx, 0, 0]);
}

unsafe extern "win64" fn m_ref_first(mgr: usize, obj: usize) {
    log([REF_FIRST_VA, mgr as u64, obj as u64, rd::<u32>(obj + 0x24) as u64, 0]);
}

unsafe extern "win64" fn m_holder_dtor(p: usize) {
    log([REF_HOLDER_DTOR_VA, rd::<u64>(p), 0, 0, 0]);
}

unsafe extern "win64" fn m_assign<const VA: u64>(slot: usize, v: usize) {
    log([VA, slot as u64, v as u64, rd::<u8>(slot + 0x40) as u64, rd::<u8>(v + 0x40) as u64]);
}

const ASSIGN_MOCKS: [(u64, *const ()); 18] = [
    (0x1_411c_aea0, m_assign::<0x1_411c_aea0> as *const ()),
    (0x1_411c_af00, m_assign::<0x1_411c_af00> as *const ()),
    (0x1_411c_afc0, m_assign::<0x1_411c_afc0> as *const ()),
    (0x1_411c_b080, m_assign::<0x1_411c_b080> as *const ()),
    (0x1_411c_b160, m_assign::<0x1_411c_b160> as *const ()),
    (0x1_411c_af60, m_assign::<0x1_411c_af60> as *const ()),
    (0x1_411c_b020, m_assign::<0x1_411c_b020> as *const ()),
    (0x1_411c_b0f0, m_assign::<0x1_411c_b0f0> as *const ()),
    (0x1_411c_b1d0, m_assign::<0x1_411c_b1d0> as *const ()),
    (0x1_411c_b660, m_assign::<0x1_411c_b660> as *const ()),
    (0x1_411c_2a20, m_assign::<0x1_411c_2a20> as *const ()),
    (0x1_411c_b710, m_assign::<0x1_411c_b710> as *const ()),
    (0x1_411c_b790, m_assign::<0x1_411c_b790> as *const ()),
    (0x1_411c_b810, m_assign::<0x1_411c_b810> as *const ()),
    (0x1_411c_b890, m_assign::<0x1_411c_b890> as *const ()),
    (0x1_411c_b910, m_assign::<0x1_411c_b910> as *const ()),
    (0x1_411c_b970, m_assign::<0x1_411c_b970> as *const ()),
    (0x1_411c_b9e0, m_assign::<0x1_411c_b9e0> as *const ()),
];

/// 0x142046590: lookup, all assign functions, destroy visitor, ref callbacks and the
/// slot-changed notification mocked (recorders; the destroy mock scribbles the slot). The
/// `get_if` getters run for real. Every value tag 0..=0x16 plus 0x7f/0x80/0xff against slot
/// tags 0x19/0x37/0x03/0xff/0x80 (negative tag index), ref objects null / first ref with and
/// without manager / already referenced, and a null lookup.
pub unsafe fn diff_apply_binding_value(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut mocks = vec![
        crate::difftest::Mock::new(VALUE_LOOKUP_VA, m_lookup as *const ())?,
        crate::difftest::Mock::new(SLOT_DESTROY_VA, m_destroy as *const ())?,
        crate::difftest::Mock::new(SLOT_ASSIGN_REF_VA, m_assign_ref as *const ())?,
        crate::difftest::Mock::new(SLOT_CHANGED_VA, m_slot_changed as *const ())?,
        crate::difftest::Mock::new(REF_FIRST_VA, m_ref_first as *const ())?,
        crate::difftest::Mock::new(REF_HOLDER_DTOR_VA, m_holder_dtor as *const ())?,
    ];
    for &(va, f) in ASSIGN_MOCKS.iter() {
        mocks.push(crate::difftest::Mock::new(va, f)?);
    }
    const NODE: usize = 0x0;
    const SLOTS: usize = 0x100; // 8 x 0x50
    const BINDING: usize = 0x400;
    const VALUE: usize = 0x500;
    const OBJ: usize = 0x600;
    let mut rng = crate::difftest::Rng::new(0x1420_4659);
    let mut cases = 0u32;
    let mut vtags: Vec<u8> = (0..=0x16).collect();
    vtags.extend([0x7f, 0x80, 0xff]);
    let result = (|| -> Result<(), String> {
        for &vt in &vtags {
            for &st in &[0x19u8, 0x37, 0x03, 0xff, 0x80] {
                // (has obj, refcount, mgr) variants matter only for tag 0xb.
                let objs: &[(bool, u32, u64)] =
                    if vt == 0xb { &[(false, 0, 0), (true, 0, 0x7000), (true, 0, 0), (true, 4, 0x7000)] } else { &[(true, 0, 0)] };
                for &(has_obj, rc, mgr) in objs {
                    for mis in [0usize, 4] {
                        let a = Arena::new(0x700, mis);
                        for i in 0..0x700 {
                            wr::<u8>(a.at(i), rng.next_u32() as u8);
                        }
                        wr::<usize>(a.at(NODE + 0x10), a.at(SLOTS));
                        let idx = rng.below(8);
                        wr::<u64>(a.at(BINDING), rng.next_u64());
                        wr::<u64>(a.at(BINDING + 8), idx);
                        wr::<u8>(a.at(SLOTS + idx as usize * 0x50 + 0x40), st);
                        wr::<u8>(a.at(VALUE + 0x40), vt);
                        wr::<usize>(a.at(VALUE), if has_obj { a.at(OBJ) } else { 0 });
                        wr::<u32>(a.at(OBJ + 0x24), rc);
                        wr::<u64>(a.at(OBJ + 0x28), mgr);
                        let (node, b) = (a.at(NODE), a.at(BINDING));
                        let src = rng.next_u64();
                        let ret = a.at(VALUE);
                        run_pair(
                            &a,
                            &|| LOOKUP_RET.with(|r| r.set(ret)),
                            &|orig| if orig { o(node, src, b) } else { n(node, src, b) },
                            &format!("vtag {vt:#x} stag {st:#x} obj {has_obj} rc {rc} mgr {mgr:#x} mis {mis}"),
                        )?;
                        cases += 1;
                    }
                }
            }
        }
        // Null lookup.
        for mis in [0usize, 4] {
            let a = Arena::new(0x700, mis);
            wr::<usize>(a.at(NODE + 0x10), a.at(SLOTS));
            wr::<u64>(a.at(BINDING), 0x1234);
            wr::<u64>(a.at(BINDING + 8), 3);
            let (node, b) = (a.at(NODE), a.at(BINDING));
            run_pair(
                &a,
                &|| LOOKUP_RET.with(|r| r.set(0)),
                &|orig| if orig { o(node, 0x55, b) } else { n(node, 0x55, b) },
                &format!("null lookup mis {mis}"),
            )?;
            cases += 1;
        }
        Ok(())
    })();
    drop(mocks);
    result.map(|_| cases)
}

unsafe extern "win64" fn m_notify_binding(kind: u32, node: usize, src: u64, b: usize) {
    log([NOTIFY_BINDING_VA, kind as u64, node as u64, src, b as u64]);
}

unsafe extern "win64" fn m_keyset_reset(set: usize) {
    log([KEYSET_RESET_VA, set as u64, rd::<u64>(set + 0x10), rd::<u64>(set + 0x18), 0]);
}

/// 0x142045c60: 0x1420460d0 and 0x1401de4e0 mocked (recorders), real `j_memset`. Real
/// Swiss-table layouts (sentinel at `ctrl[cap]`, 15 cloned control bytes) with capacities
/// 0/15/31/127/255, keys hit in the first or a later probe group, absent keys, deleted
/// markers, and sizes 0 / non-zero (both clear paths).
pub unsafe fn diff_notify_changed_bindings(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(NOTIFY_BINDING_VA, m_notify_binding as *const ())?;
    let _m2 = crate::difftest::Mock::new(KEYSET_RESET_VA, m_keyset_reset as *const ())?;
    const NODE: usize = 0x0;
    const DEF: usize = 0x100;
    const LIST1: usize = 0x200;
    const LIST2: usize = 0x400;
    const CTX: usize = 0x600;
    const CTRL: usize = 0x1000;
    const SLOTS: usize = 0x1200;
    let mut rng = crate::difftest::Rng::new(0x1420_45c6);
    let mut cases = 0u32;
    for round in 0..60u32 {
        for mis in [0usize, 4] {
            let cap: u64 = [15u64, 31, 127, 255, 0, 15, 127][round as usize % 7];
            let a = Arena::new(0x2000, mis);
            let (node, ctx) = (a.at(NODE), a.at(CTX));
            wr::<usize>(node, a.at(DEF));
            wr::<u64>(ctx + 0x10, 0x3333_0000_0000_0010);
            wr::<u64>(ctx + 0x358, 0x3333_0000_0000_0358);
            // Control bytes: empty / deleted / random h2; sentinel; clones.
            let slots = cap as usize + 1;
            let mut ctrl = vec![0x80u8; slots + 15];
            for c in ctrl.iter_mut().take(slots) {
                *c = match rng.below(10) {
                    0..=3 => 0x80,
                    4 => 0xfe,
                    _ => rng.next_u32() as u8 & 0x7f,
                };
            }
            let mut slot_keys: Vec<u64> = (0..slots).map(|_| rng.next_u64()).collect();
            // Bindings: some present (first group or a later group), some absent.
            let n1 = rng.below(9) as usize;
            let n2 = rng.below(9) as usize;
            fill_bindings(a.at(LIST1), n1, &mut rng, 0x11);
            fill_bindings(a.at(LIST2), n2, &mut rng, 0x11);
            if cap != 0 {
                for i in 0..n1 + n2 {
                    let b = if i < n1 { a.at(LIST1 + i * 0x18) } else { a.at(LIST2 + (i - n1) * 0x18) };
                    let key = rd::<u64>(b);
                    let h = keyset_hash(key);
                    let pos = (h >> 7) & cap;
                    match rng.below(4) {
                        0 => {}
                        1 | 2 => {
                            let idx = ((pos + rng.below(16)) & cap) as usize;
                            if idx as u64 != cap {
                                ctrl[idx] = h as u8 & 0x7f;
                                slot_keys[idx] = key;
                            }
                        }
                        _ => {
                            // Second probe group; make the first group free of empties.
                            if cap >= 31 {
                                for k in 0..16 {
                                    let j = ((pos + k) & cap) as usize;
                                    if j as u64 != cap && ctrl[j] == 0x80 {
                                        ctrl[j] = 0xfe;
                                    }
                                }
                                let idx = ((pos + 16 + rng.below(16)) & cap) as usize;
                                if idx as u64 != cap {
                                    ctrl[idx] = h as u8 & 0x7f;
                                    slot_keys[idx] = key;
                                }
                            }
                        }
                    }
                }
            }
            // Keep at least one empty so absent keys terminate.
            if cap != 0 && !ctrl[..cap as usize].contains(&0x80) {
                ctrl[0] = 0x80;
            }
            ctrl[cap as usize] = 0xff;
            for i in 0..15 {
                // Clones of ctrl[0..15]; a zero-capacity table is {sentinel, 15 x empty}.
                ctrl[slots + i] = if cap == 0 { 0x80 } else { ctrl[i % slots] };
            }
            for (i, &c) in ctrl.iter().enumerate() {
                wr::<u8>(a.at(CTRL + i), c);
            }
            for (i, &k) in slot_keys.iter().enumerate() {
                wr::<u64>(a.at(SLOTS + i * 8), k);
            }
            wr::<usize>(node + 0x60, a.at(CTRL));
            wr::<usize>(node + 0x68, a.at(SLOTS));
            wr::<u64>(node + 0x70, if round % 5 == 4 { 0 } else { 1 + rng.below(cap.max(1)) });
            wr::<u64>(node + 0x78, cap);
            wr::<u64>(node + 0x80, rng.next_u64());
            wr::<u64>(node + 0x88, rng.next_u64());
            wr::<usize>(a.at(DEF + 0x90), a.at(LIST1));
            wr::<u32>(a.at(DEF + 0x98), n1 as u32);
            wr::<usize>(a.at(DEF + 0xa0), a.at(LIST2));
            wr::<u32>(a.at(DEF + 0xa8), n2 as u32);
            run_pair(
                &a,
                &|| {},
                &|orig| if orig { o(ctx, node) } else { n(ctx, node) },
                &format!("round {round} mis {mis} cap {cap} n1 {n1} n2 {n2}"),
            )?;
            cases += 1;
        }
    }
    Ok(cases)
}

unsafe extern "win64" fn m_view_copy(dst: usize, src: usize) -> usize {
    let (lo, hi) = APPLY_ENV.with(|e| e.get());
    let src_tag = if (lo..hi).contains(&src) { src as u64 } else { 0x5ac };
    log([VIEW_COPY_VA, src_tag, rd::<u64>(src), rd::<u64>(src + 0x2e8), 0]);
    for i in 0..0x2f0 / 8 {
        wr::<u64>(dst + i * 8, rd::<u64>(src + i * 8) ^ 0x9e37_79b9);
    }
    wr::<u64>(dst, 0x1430_ee038);
    dst
}

/// Log a node pass call: node, FNV of the context bytes (the original's uninitialized gaps
/// masked), and toggle `node.byte[0x2f8]` per MODE bit.
unsafe fn log_pass(tag: u64, ctx: usize, node: usize, bit: u32) {
    let mut b = std::slice::from_raw_parts(ctx as *const u8, 0x380).to_vec();
    for r in [0x18..0x20, 0x32c..0x330, 0x348..0x350] {
        b[r].fill(0);
    }
    let h = b.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &x| (h ^ x as u64).wrapping_mul(0x100_0000_01b3));
    log([tag, node as u64, h, rd::<u64>(ctx + 0x358), rd::<u8>(node + 0x2f8) as u64]);
    if MODE.with(|m| m.get()) & bit != 0 {
        wr::<u8>(node + 0x2f8, rd::<u8>(node + 0x2f8) ^ 1);
    }
}

unsafe extern "win64" fn m_node_prepare(ctx: usize, node: usize) {
    log_pass(NODE_PREPARE_VA, ctx, node, 1);
}

unsafe extern "win64" fn m_node_activate(ctx: usize, node: usize) {
    log_pass(NODE_ACTIVATE_VA, ctx, node, 2);
}

unsafe extern "win64" fn m_update_node(ctx: usize, node: usize) {
    log_pass(UPDATE_NODE_VA, ctx, node, 4);
}

/// 0x141f450f0: View copy constructor and the three node passes mocked (the copy mock writes
/// a deterministic transform of its source; the pass mocks hash the whole context and can
/// flip `node.byte[0x2f8]` to exercise the re-reads); the world-slot global is pinned to 3.
/// All combinations of row active flag, u32 column, world flag, node byte and mock modes over
/// four rows.
pub unsafe fn diff_update_instance(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(VIEW_COPY_VA, m_view_copy as *const ())?;
    let _m2 = crate::difftest::Mock::new(NODE_PREPARE_VA, m_node_prepare as *const ())?;
    let _m3 = crate::difftest::Mock::new(NODE_ACTIVATE_VA, m_node_activate as *const ())?;
    let _m4 = crate::difftest::Mock::new(UPDATE_NODE_VA, m_update_node as *const ())?;
    let g = exe::ptr::<u8>(WORLD_SLOT_INDEX_VA) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g, 2);
    wr::<u16>(g as usize, 3);
    const INST: usize = 0x0;
    const ROW28: usize = 0x100;
    const ROW10: usize = 0x200;
    const ROW70: usize = 0x300;
    const COL32: usize = 0x500;
    const NODE: usize = 0x600;
    const VIEW: usize = 0x1000;
    const FLAGS: usize = 0x2000;
    const WORLD: usize = 0x3000;
    const SIZE: usize = WORLD + 0x580e0 + 0x20;
    let mut rng = crate::difftest::Rng::new(0x141f_450f);
    let mut cases = 0u32;
    let result = (|| -> Result<(), String> {
        for combo in 0..64u32 {
            for mis in [0usize, 4] {
                let a = Arena::new(SIZE, mis);
                for i in 0..0x2000 {
                    wr::<u8>(a.at(i), rng.next_u32() as u8);
                }
                let row = rng.below(4) as usize;
                wr::<usize>(a.at(INST), a.at(ROW28));
                wr::<usize>(a.at(INST + 0x08), a.at(ROW10));
                wr::<usize>(a.at(INST + 0x10), a.at(ROW70));
                wr::<usize>(a.at(INST + 0x18), a.at(COL32));
                wr::<u64>(a.at(INST + 0x30), row as u64);
                wr::<u8>(a.at(ROW28 + row * 0x28 + 0x20), (combo & 1) as u8 * (rng.next_u32() as u8 | 1));
                wr::<u32>(a.at(COL32 + row * 4), if combo & 2 != 0 { 0 } else { 1 + rng.below(5) as u32 });
                wr::<usize>(a.at(ROW10 + row * 0x10 + 8), a.at(NODE));
                wr::<u8>(a.at(NODE + 0x2f8), ((combo >> 3) & 1) as u8);
                wr::<usize>(a.at(WORLD + 0x580e0), a.at(FLAGS));
                wr::<usize>(a.at(FLAGS + 3 * 8), a.at(FLAGS + 0x100));
                wr::<u8>(a.at(FLAGS + 0x100 + 0x1c), ((combo >> 2) & 1) as u8 * 0x80);
                let mode = combo >> 4;
                let (inst, world, view) = (a.at(INST), a.at(WORLD), a.at(VIEW));
                let span = (a.base, a.base + a.len);
                run_pair(
                    &a,
                    &|| {
                        MODE.with(|m| m.set(mode));
                        APPLY_ENV.with(|e| e.set(span));
                    },
                    &|orig| if orig { o(inst, world, view, 0) } else { n(inst, world, view, 0) },
                    &format!("combo {combo:#x} row {row} mis {mis}"),
                )?;
                cases += 1;
            }
        }
        Ok(())
    })();
    snap.restore();
    MODE.with(|m| m.set(0));
    result.map(|_| cases)
}
