//! GfxGraph command execution as run by the render thread's `rend::OrderSolver` pass loop.
//!
//! `rend::GfxGraphCommand::declare(OrderSolver&)` (0x142377590, vfunc_1 shared by the command
//! classes) registers each command as one OrderSolver pass and stores
//! `<lambda_1>` = `[this](OrderSolverPassData* d) { this->execute(d); }` in the pass's second
//! `std::function` (pass entry `+0x50`, impl pointer at `+0x88`). The pass loop
//! (0x1422fe4e0 -> 0x1422fec00, see `culling::worker::propagate_render_pass_masks`) invokes it
//! through `_Func_impl_no_alloc<..>::_Do_call` (0x14236d960) once the pass is started and its
//! pass data counter is 0. GfxGraph passes have no start/finish callbacks, so their counter
//! stays 0 and `execute` runs exactly once per pass per frame.
//!
//! Resource handles (u32: low 16 bits slot index, high 16 bits generation) resolve through the
//! global resource registry `[0x143a001e0]` (`+0x78` slot table of 0x78-byte slots, `+0x80`
//! slot count). Slot: `+0x68` kind byte, `+0x70` u16 generation; per kind a "fixed" flag with
//! its object pointer, or a key that is searched in the pass data's 64-entry key array
//! (`pass+0x208`), whose matching `pass+8+i*8` holds a pointer to the object pointer.

use crate::exe::{self, rd, wr};
use std::cell::RefCell;

/// Global resource registry pointer.
const RESOURCE_REGISTRY_VA: u64 = 0x1_43a0_01e0;
/// `lookup_render_texture(pass, handle)` (kind 1).
const LOOKUP_RENDER_TEXTURE_VA: u64 = 0x1_4237_8130;
/// `lookup_depth_buffer(pass, handle)` (kind 2).
const LOOKUP_DEPTH_BUFFER_VA: u64 = 0x1_4237_8090;
/// Render-target setup `(desc*, mip)` (0x118-byte descriptor, see [`render_to_texture_execute`]).
const SET_RENDER_TARGETS_VA: u64 = 0x1_4205_49b0;
/// Viewport/scissor setup `(width, height)`.
const SET_VIEWPORT_VA: u64 = 0x1_422e_6e80;
/// `rend::GfxGraphRenderToTextureCommand::vfunc_0` (execute).
const RTT_EXECUTE_VA: u64 = 0x1_4237_6a30;
/// `_Func_impl_no_alloc<GfxGraphCommand::declare::<lambda_1>, void, OrderSolverPassData*>::_Do_call`.
pub const DECLARE_LAMBDA_DO_CALL_VA: u64 = 0x1_4236_d960;

/// Registry slot for `handle` (low 16 bits non-zero, checked by the caller) or 0: index below
/// the slot count (u32 compare) and generation match (u16 compare at `+0x70`).
#[inline(always)]
unsafe fn registry_slot(reg: usize, handle: u32) -> usize {
    let idx = (handle & 0xffff) as usize;
    if idx as u32 >= rd::<u32>(reg + 0x80) {
        return 0;
    }
    let slot = rd::<usize>(reg + 0x78).wrapping_add(idx * 0x78);
    if rd::<u16>(slot + 0x70) != (handle >> 16) as u16 {
        return 0;
    }
    slot
}

/// The pass data's pointer for `key`: first of the 64 entries of `pass+0x208` that is non-zero
/// and equals `key` -> `[pass + 8 + i*8]`; 0 when none matches.
#[inline(always)]
unsafe fn pass_resource(pass: usize, key: u64) -> usize {
    for i in 0..0x40usize {
        let k = rd::<u64>(pass + 0x208 + i * 8);
        if k != 0 && k == key {
            return rd::<usize>(pass + 8 + i * 8);
        }
    }
    0
}

/// Shared body of the two typed lookups: `kind` byte, offsets of the "fixed" u32 flag, its
/// object pointer and the pass key.
#[inline(always)]
unsafe fn lookup_typed(pass: usize, handle: u32, kind: u8, fixed: usize, fixed_ptr: usize, key: usize) -> u64 {
    if handle as u16 == 0 {
        return 0;
    }
    let reg = rd::<usize>(exe::ptr::<u8>(RESOURCE_REGISTRY_VA) as usize);
    let slot = registry_slot(reg, handle);
    if slot == 0 || rd::<u8>(slot + 0x68) != kind {
        return 0;
    }
    let p = if rd::<u32>(slot + fixed) == 1 { rd::<usize>(slot + fixed_ptr) } else { pass_resource(pass, rd::<u64>(slot + key)) };
    if p == 0 {
        0
    } else {
        rd::<u64>(p)
    }
}

/// Resolve a render-texture handle (registry kind 1) for a pass.
///
/// Original: 0x142378130 (146 bytes, leaf, Throws::No). Kind 1 slots: `+0x18 == 1` -> object
/// pointer at `+0x58`, else the pass key at `+0x60`. Returns `*ptr` or 0 (null handle, index out
/// of range, generation or kind mismatch, key not found, null entry).
pub unsafe extern "win64" fn lookup_render_texture(pass: usize, handle: u32) -> u64 {
    lookup_typed(pass, handle, 1, 0x18, 0x58, 0x60)
}

/// Resolve a depth-buffer handle (registry kind 2) for a pass.
///
/// Original: 0x142378090 (146 bytes, leaf, Throws::No). Kind 2 slots: `+0x08 == 1` -> object
/// pointer at `+0x28`, else the pass key at `+0x30`.
pub unsafe extern "win64" fn lookup_depth_buffer(pass: usize, handle: u32) -> u64 {
    lookup_typed(pass, handle, 2, 0x08, 0x28, 0x30)
}

/// Inlined handle resolution of [`render_to_texture_execute`] for kinds 4 and 5: key at
/// `slot+8`. `None` = the kind-4 slot's `+0x40` count is 0 (the original returns at once).
#[inline(always)]
unsafe fn lookup_inline(reg: usize, pass: usize, handle: u32, kind: u8) -> Option<u64> {
    if handle as u16 == 0 {
        return Some(0);
    }
    let slot = registry_slot(reg, handle);
    if slot == 0 || rd::<u8>(slot + 0x68) != kind {
        return Some(0);
    }
    if kind == 4 && rd::<u32>(slot + 0x40) == 0 {
        return None;
    }
    let p = pass_resource(pass, rd::<u64>(slot + 8));
    Some(if p == 0 { 0 } else { rd::<u64>(p) })
}

/// `rend::GfxGraphRenderToTextureCommand::execute(OrderSolverPassData*)` — bind the command's
/// color/depth target, set the viewport to its size and execute the child commands.
///
/// Original: 0x142376a30 (719 bytes, vftable 0x143401878 slot 0; no FH4 handler;
/// Throws::Discard: the child calls can throw). Color target: handle `+0x290` through
/// 0x142378130, else handle `+0x294` resolved inline as registry kind 4 (whose `+0x40 == 0`
/// returns at once). Depth target: `+0x298` through 0x142378090, else `+0x29c` inline as kind 5.
/// The registry pointer is read once on entry. Nothing happens when both are null, when both
/// are set and their first two u32s (width, height) differ, or when the child count `+0x2a0` is
/// 0. Otherwise builds the 0x118-byte target descriptor on the stack (`+0x00` u32 1,
/// `+0x08..+0x118` zero except `+0xc8` = color and `+0x108` = depth; the original leaves
/// `+0x04` uninitialized, zero here), calls 0x1420549b0(desc, 0), then 0x1422e6e80(w, h) from
/// the color target (else depth), and calls `child->vfunc_3(child, &ctx)` for every child
/// pointer at `+0x90 + i*8` (count re-read per iteration) with `ctx = {pass, 0 x5}`.
pub unsafe extern "win64" fn render_to_texture_execute(this: usize, pass: usize) {
    let reg = rd::<usize>(exe::ptr::<u8>(RESOURCE_REGISTRY_VA) as usize);
    let color: u64 = if rd::<u16>(this + 0x290) != 0 {
        let h = rd::<u32>(this + 0x290);
        exe::call_native!(LOOKUP_RENDER_TEXTURE_VA, lookup_render_texture, (pass, h) -> u64)
    } else {
        match lookup_inline(reg, pass, rd::<u32>(this + 0x294), 4) {
            Some(v) => v,
            None => return,
        }
    };
    let depth: u64 = if rd::<u16>(this + 0x298) != 0 {
        let h = rd::<u32>(this + 0x298);
        exe::call_native!(LOOKUP_DEPTH_BUFFER_VA, lookup_depth_buffer, (pass, h) -> u64)
    } else {
        // Kind 5 has no `+0x40` early-out.
        lookup_inline(reg, pass, rd::<u32>(this + 0x29c), 5).unwrap_or(0)
    };
    let (color, depth) = (color as usize, depth as usize);
    if color == 0 {
        if depth == 0 {
            return;
        }
    } else if depth != 0 && (rd::<u32>(color) != rd::<u32>(depth) || rd::<u32>(color + 4) != rd::<u32>(depth + 4)) {
        return;
    }
    if rd::<u32>(this + 0x2a0) == 0 {
        return;
    }
    let mut desc = [0u64; 0x118 / 8];
    let d = desc.as_mut_ptr() as usize;
    wr::<u32>(d, 1);
    wr::<usize>(d + 0xc8, color);
    wr::<usize>(d + 0x108, depth);
    exe::call_va!(SET_RENDER_TARGETS_VA, (d, 0u32));
    let size_src = if color != 0 { color } else { depth };
    exe::call_va!(SET_VIEWPORT_VA, (rd::<u32>(size_src), rd::<u32>(size_src + 4)));
    let mut ctx = [0u64; 6];
    ctx[0] = pass as u64;
    let c = ctx.as_mut_ptr() as usize;
    let mut i = 0u32;
    while i < rd::<u32>(this + 0x2a0) {
        let child = rd::<usize>(this + 0x90 + i as usize * 8);
        exe::call_va!(rd::<u64>(rd::<usize>(child) + 0x18), (child, c));
        i += 1;
    }
}

/// `std::_Func_impl_no_alloc<GfxGraphCommand::declare::<lambda_1>, void,
/// OrderSolverPassData*>::_Do_call(this, OrderSolverPassData** arg)` — the OrderSolver pass
/// callback of every GfxGraph command: `cmd = this->lambda.cmd` (`+8`), tail call
/// `cmd->vfunc_0(cmd, *arg)` (the command's execute).
///
/// Original: 0x14236d960 (13 bytes; Throws::No for direct calls, the command call is
/// indirect). The RenderToTexture execute is called directly when its replacement is active.
pub unsafe extern "win64" fn declare_lambda_do_call(this: usize, arg: usize) {
    let cmd = rd::<usize>(this + 8);
    let pass = rd::<usize>(arg);
    let f = rd::<u64>(rd::<usize>(cmd));
    if f == exe::ptr::<u8>(RTT_EXECUTE_VA) as u64 {
        exe::call_native!(RTT_EXECUTE_VA, render_to_texture_execute, (cmd, pass) -> ());
    } else {
        exe::call_va!(f, (cmd, pass));
    }
}

/// Shader-variant table lookup `(cmd + 0x18) -> table*`.
const COMPUTE_VARIANTS_VA: u64 = 0x1_4237_7ae0;
/// Bind the command's parameters for a variant `(cmd + 8, pass, variant) -> bool`.
const BIND_COMPUTE_PARAMS_VA: u64 = 0x1_4236_cf00;
/// `(cmd list, u32, u32)` with the globals 0x143a0014c / 0x143a00150.
const SET_COMPUTE_STATE_VA: u64 = 0x1_421f_0110;
/// `(variant)`.
const SET_COMPUTE_SHADER_VA: u64 = 0x1_4224_ded0;
/// `(tls + 0x467e0, &{u64, u32} dims)`.
const DISPATCH_COMPUTE_VA: u64 = 0x1_4206_ee70;
/// `(texture)` for every UAV-class binding the variant references.
const MARK_UAV_WRITTEN_VA: u64 = 0x1_4206_7c10;
const DYN_TLS_ON_DEMAND_INIT_VA: u64 = 0x1_4183_49c0;
const COMPUTE_STATE_ARG1_VA: u64 = 0x1_43a0_014c;
const COMPUTE_STATE_ARG2_VA: u64 = 0x1_43a0_0150;
/// i32 per binding id: class (5 = the class handled after the dispatch).
const BINDING_CLASS_TABLE_VA: u64 = 0x1_4385_8590;

/// Whether `id` appears in one of the variant's six resource layouts (`variant+0x60..+0x90`):
/// first the layout's `{.., ptr @+0x18, u32 count @+0x20}` groups (0x28 bytes each, array at
/// `+0x88`, u32 count `+0x90`) with 0x18-byte entries, then its eight `{ptr, u32 count}` lists at
/// `+0x08` (0x10 apart) with 0x1c-byte entries; entry `+0` is the id.
#[inline(always)]
unsafe fn variant_references(variant: usize, id: i32) -> bool {
    for l in 0..6usize {
        let lay = rd::<usize>(variant + 0x60 + l * 8);
        if lay == 0 {
            continue;
        }
        let ngroups = rd::<u32>(lay + 0x90) as usize;
        let mut g = rd::<usize>(lay + 0x88).wrapping_add(0x18);
        for _ in 0..ngroups {
            let n = rd::<u32>(g + 8) as usize;
            let mut e = rd::<usize>(g);
            for _ in 0..n {
                if rd::<i32>(e) == id {
                    return true;
                }
                e = e.wrapping_add(0x18);
            }
            g = g.wrapping_add(0x28);
        }
        for k in 0..8usize {
            let list = lay + 8 + k * 0x10;
            let n = rd::<u32>(list + 8) as usize;
            let mut e = rd::<usize>(list);
            for _ in 0..n {
                if rd::<i32>(e) == id {
                    return true;
                }
                e = e.wrapping_add(0x1c);
            }
        }
    }
    false
}

/// Dispatch helper of `rend::GfxGraphDispatchComputeShaderCommand::execute` (its only caller,
/// 0x142375b70): select the shader variant, bind, dispatch, then mark the written resources.
///
/// Original: 0x142375c70 (661 bytes; no FH4 handler; Throws::Discard like the shard draft).
/// `table = 0x142377ae0(cmd + 0x18)`; none -> return. Binary search (signed bounds, 32-bit
/// midpoint `(lo + hi) >> 1` arithmetic) of the table's sorted variant pointers (`[table]`,
/// u32 count `+8`) for the key `[table+0xb8] & ~[table+0xbc]` compared unsigned with
/// `variant+0x90`; returns when not found or `variant+0x40` is null, and when
/// `0x14236cf00(cmd + 8, pass, variant)` returns false (`al`). Then, with the exe's static TLS
/// block (`[gs:[0x58]]`, `__dyn_tls_on_demand_init` when its `+0x70` byte is 0):
/// `0x1421f0110([tls+0x6308], [0x143a0014c], [0x143a00150])`, `0x14224ded0(variant)`, and
/// `0x14206ee70(tls + 0x467e0, &{[cmd+0x28] (u64), [cmd+0x30] (u32)})` (dims read after the
/// two calls; the TLS byte re-checked before the third). Finally, for each 16-byte binding of
/// the command (`[cmd+8]`, count `[cmd+0x10]`; `+0` i32 id, `+4` u8 type, `+8` pointer to the
/// u32 handle) whose class `[0x143858590 + id*4]` is 5 and that the variant references (see
/// [`variant_references`]): type 0xd resolves the handle through 0x142378130, type 0xf through
/// 0x142378090 (other types skip), and a non-null result goes to 0x142067c10.
pub unsafe extern "win64" fn dispatch_compute_shader(cmd: usize, pass: usize) {
    let table = exe::call_va!(COMPUTE_VARIANTS_VA, (cmd + 0x18) -> usize);
    if table == 0 {
        return;
    }
    let key = !rd::<u32>(table + 0xbc) & rd::<u32>(table + 0xb8);
    let mut hi = rd::<u32>(table + 8).wrapping_sub(1) as i32;
    let mut lo = 0i32;
    let mut variant = 0usize;
    if hi >= 0 {
        let arr = rd::<usize>(table);
        while lo <= hi {
            let mid = (hi.wrapping_add(lo)) >> 1;
            let v = rd::<usize>(arr.wrapping_add((mid as isize as usize).wrapping_mul(8)));
            let vk = rd::<u32>(v + 0x90);
            if key < vk {
                hi = mid.wrapping_sub(1);
            } else if key == vk {
                variant = v;
                break;
            } else {
                lo = mid.wrapping_add(1);
            }
        }
    }
    if variant == 0 || rd::<u64>(variant + 0x40) == 0 {
        return;
    }
    if !exe::call_va!(BIND_COMPUTE_PARAMS_VA, (cmd + 8, pass, variant) -> bool) {
        return;
    }
    let tls = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls + 0x70) == 0 {
        exe::call_va!(DYN_TLS_ON_DEMAND_INIT_VA, () -> ());
    }
    let a1 = rd::<u32>(exe::ptr::<u8>(COMPUTE_STATE_ARG1_VA) as usize);
    let a2 = rd::<u32>(exe::ptr::<u8>(COMPUTE_STATE_ARG2_VA) as usize);
    exe::call_va!(SET_COMPUTE_STATE_VA, (rd::<u64>(tls + 0x6308), a1, a2));
    exe::call_va!(SET_COMPUTE_SHADER_VA, (variant));
    let dims_xy = rd::<u64>(cmd + 0x28);
    let dims_z = rd::<u32>(cmd + 0x30);
    if rd::<u8>(tls + 0x70) == 0 {
        exe::call_va!(DYN_TLS_ON_DEMAND_INIT_VA, () -> ());
    }
    let mut dims = [0u64; 2];
    let dp = dims.as_mut_ptr() as usize;
    wr::<u64>(dp, dims_xy);
    wr::<u32>(dp + 8, dims_z);
    exe::call_va!(DISPATCH_COMPUTE_VA, (tls + 0x467e0, dp));
    let first = rd::<usize>(cmd + 8);
    let end = first.wrapping_add(rd::<u64>(cmd + 0x10).wrapping_shl(4) as usize);
    let classes = exe::ptr::<u8>(BINDING_CLASS_TABLE_VA) as usize;
    let mut b = first;
    while b != end {
        let id = rd::<i32>(b);
        if rd::<i32>(classes.wrapping_add((id as isize as usize).wrapping_mul(4))) == 5 && variant_references(variant, id) {
            let ty = rd::<u8>(b + 4);
            let res = match ty {
                0xd => {
                    let h = rd::<u32>(rd::<usize>(b + 8));
                    Some(exe::call_native!(LOOKUP_RENDER_TEXTURE_VA, lookup_render_texture, (pass, h) -> u64))
                }
                0xf => {
                    let h = rd::<u32>(rd::<usize>(b + 8));
                    Some(exe::call_native!(LOOKUP_DEPTH_BUFFER_VA, lookup_depth_buffer, (pass, h) -> u64))
                }
                _ => None,
            };
            if let Some(r) = res {
                if r != 0 {
                    exe::call_va!(MARK_UAV_WRITTEN_VA, (r));
                }
            }
        }
        b = b.wrapping_add(0x10);
    }
}

// ---------------------------------------------------------------------------------------------
// Difftests
// ---------------------------------------------------------------------------------------------

type LogEntry = [u64; 6];

thread_local! {
    static LOG: RefCell<Vec<LogEntry>> = const { RefCell::new(Vec::new()) };
}

fn log(e: LogEntry) {
    LOG.with(|l| l.borrow_mut().push(e));
}

fn take_log() -> Vec<LogEntry> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// Zeroed byte arena at a chosen misalignment.
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
}

// World layout: registry header, slot table, pass data, object cells, objects, command.
const A_REG: usize = 0x0000; // 0x100
const A_SLOTS: usize = 0x0100; // N_SLOTS x 0x78
const N_SLOTS: usize = 24;
const A_PASS: usize = 0x0d00; // 0x608
const A_CELLS: usize = 0x1400; // 64 x 8: pointers to objects
const A_OBJS: usize = 0x1600; // 16 x 0x10: {w, h, tag}
const A_CMD: usize = 0x1800; // 0x2b0
const A_CHILDREN: usize = 0x1b00; // 8 x 0x10: {vtable ptr, id}
const A_VTBL: usize = 0x1c00; // 4 slots
const A_SIZE: usize = 0x1c40;

/// Restores a snapshot of exe globals when dropped (registry pointer, class table).
struct RegistryGuard(crate::difftest::Snapshot);

impl RegistryGuard {
    unsafe fn install(reg: usize) -> RegistryGuard {
        let g = exe::ptr::<u8>(RESOURCE_REGISTRY_VA) as *mut u8;
        let s = crate::difftest::Snapshot::take(g, 8);
        wr::<usize>(g as usize, reg);
        RegistryGuard(s)
    }
}

impl Drop for RegistryGuard {
    fn drop(&mut self) {
        unsafe { self.0.restore() }
    }
}

/// Random registry + pass data. Returns handles that hit valid slots of each kind (plus
/// random noise handles are generated by the callers).
unsafe fn build_world(a: &Arena, rng: &mut crate::difftest::Rng) -> Vec<(u8, u32)> {
    let n_valid = 1 + rng.below(N_SLOTS as u64) as u32;
    wr::<usize>(a.at(A_REG + 0x78), a.at(A_SLOTS));
    wr::<u32>(a.at(A_REG + 0x80), n_valid);
    // Objects: sizes from a small set so width/height matches are common.
    for i in 0..16usize {
        let o = a.at(A_OBJS + i * 0x10);
        wr::<u32>(o, [640u32, 1280, 640, 320][rng.below(4) as usize]);
        wr::<u32>(o + 4, [360u32, 720, 360, 180][rng.below(4) as usize]);
        wr::<u32>(o + 8, 0x0b0b_0000 | i as u32);
    }
    // Cells: pointer to an object or null.
    for i in 0..64usize {
        let v = if rng.below(6) == 0 { 0 } else { a.at(A_OBJS + rng.below(16) as usize * 0x10) };
        wr::<usize>(a.at(A_CELLS + i * 8), v);
    }
    // Pass data: 64 keys (some zero, some repeated) and pointers to cells (some null).
    let keys: Vec<u64> = (0..12).map(|_| rng.next_u64() | 1).collect();
    for i in 0..64usize {
        let k = match rng.below(5) {
            0 => 0,
            1 | 2 => keys[rng.below(keys.len() as u64) as usize],
            _ => rng.next_u64(),
        };
        wr::<u64>(a.at(A_PASS + 0x208 + i * 8), k);
        let p = if rng.below(8) == 0 { 0 } else { a.at(A_CELLS + rng.below(64) as usize * 8) };
        wr::<usize>(a.at(A_PASS + 8 + i * 8), p);
    }
    wr::<u32>(a.at(A_PASS), rng.next_u32());
    let mut handles = Vec::new();
    for s in 0..N_SLOTS {
        let slot = a.at(A_SLOTS + s * 0x78);
        for q in 0..0x78usize / 8 {
            wr::<u64>(slot + q * 8, rng.next_u64());
        }
        let kind = [1u8, 2, 4, 5, 1, 2, 4, 5, 3, 0][rng.below(10) as usize];
        wr::<u8>(slot + 0x68, kind);
        let gen = rng.next_u32() as u16;
        wr::<u16>(slot + 0x70, gen);
        let key = |rng: &mut crate::difftest::Rng| {
            if rng.below(3) == 0 { rng.next_u64() } else { keys[rng.below(keys.len() as u64) as usize] }
        };
        let fixed = |rng: &mut crate::difftest::Rng| if rng.below(3) == 0 { 1u32 } else { rng.below(3) as u32 * 2 };
        let cell = |rng: &mut crate::difftest::Rng, a: &Arena| {
            if rng.below(5) == 0 { 0 } else { a.at(A_CELLS + rng.below(64) as usize * 8) }
        };
        // Kind 1: +0x18 flag, +0x58 ptr, +0x60 key. Kind 2: +0x08 flag, +0x28 ptr, +0x30 key.
        // Kinds 4/5: +0x08 key, +0x40 count.
        wr::<u32>(slot + 0x18, fixed(rng));
        wr::<usize>(slot + 0x58, cell(rng, a));
        wr::<u64>(slot + 0x60, key(rng));
        if kind == 2 {
            wr::<u32>(slot + 0x08, fixed(rng));
        } else {
            wr::<u64>(slot + 0x08, key(rng));
        }
        wr::<usize>(slot + 0x28, cell(rng, a));
        wr::<u64>(slot + 0x30, key(rng));
        wr::<u32>(slot + 0x40, if rng.below(4) == 0 { 0 } else { rng.next_u32() });
        let h = ((gen as u32) << 16) | s as u32;
        if s != 0 {
            handles.push((kind, h));
        }
    }
    handles
}

/// A handle for a lookup test: valid ones, slot 0 (null index), stale generation, index past
/// the count, random.
fn pick_handle(rng: &mut crate::difftest::Rng, handles: &[(u8, u32)], want: u8) -> u32 {
    let of_kind: Vec<u32> = handles.iter().filter(|(k, _)| *k == want).map(|(_, h)| *h).collect();
    match rng.below(10) {
        0 => rng.next_u32() & 0xffff_0000,
        1 => rng.next_u32(),
        2 => (rng.next_u32() & 0xffff_0000) | (N_SLOTS as u32 + rng.below(4) as u32),
        3 if !handles.is_empty() => handles[rng.below(handles.len() as u64) as usize].1 ^ 0x0001_0000,
        4 if !handles.is_empty() => handles[rng.below(handles.len() as u64) as usize].1,
        _ if !of_kind.is_empty() => of_kind[rng.below(of_kind.len() as u64) as usize],
        _ => rng.next_u32(),
    }
}

unsafe fn diff_lookup(orig: usize, new: usize, kind: u8, seed: u64) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u64;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(seed);
    let mut cases = 0u32;
    for round in 0..32u32 {
        for mis in [0usize, 4] {
            let a = Arena::new(A_SIZE, mis);
            let handles = build_world(&a, &mut rng);
            let _g = RegistryGuard::install(a.at(A_REG));
            let before = a.bytes();
            for _ in 0..24 {
                let h = pick_handle(&mut rng, &handles, kind);
                let pass = a.at(A_PASS);
                let ro = o(pass, h);
                let rn = n(pass, h);
                crate::diff_eq!(ro, rn, "kind {kind} round {round} mis {mis} handle {h:#x}");
                cases += 1;
            }
            if a.bytes() != before {
                return Err(format!("lookup wrote to the world (round {round} mis {mis})"));
            }
        }
    }
    Ok(cases)
}

/// 0x142378130: random registries (all slot kinds, fixed/keyed, stale generations, indices at
/// and past the count, null cells and entries, zero and repeated pass keys), +0/+4 arenas.
pub unsafe fn diff_lookup_render_texture(orig: usize, new: usize) -> Result<u32, String> {
    diff_lookup(orig, new, 1, 0x1423_7813)
}

/// 0x142378090: as [`diff_lookup_render_texture`] for kind 2.
pub unsafe fn diff_lookup_depth_buffer(orig: usize, new: usize) -> Result<u32, String> {
    diff_lookup(orig, new, 2, 0x1423_7809)
}

/// FNV-1a over the target descriptor, skipping `+0x04..+0x08` (uninitialized in the original).
unsafe fn desc_hash(d: usize) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for i in (0..0x118usize).filter(|i| !(4..8).contains(i)) {
        h = (h ^ rd::<u8>(d + i) as u64).wrapping_mul(0x100_0000_01b3);
    }
    h
}

unsafe extern "win64" fn m_set_render_targets(d: usize, mip: u32) {
    log([SET_RENDER_TARGETS_VA, desc_hash(d), rd::<u32>(d) as u64, rd::<u64>(d + 0xc8), rd::<u64>(d + 0x108), mip as u64]);
    // The real one writes the mip level and a zero into the descriptor.
    wr::<u32>(d + 0x110, mip);
    wr::<u32>(d + 0x114, 0);
}

unsafe extern "win64" fn m_set_viewport(w: u32, h: u32) {
    log([SET_VIEWPORT_VA, w as u64, h as u64, 0, 0, 0]);
}

unsafe extern "win64" fn m_child_exec(child: usize, ctx: usize) {
    log([0xc41d, child as u64, rd::<u64>(ctx), rd::<u64>(ctx + 8) | rd::<u64>(ctx + 0x10), rd::<u64>(ctx + 0x18) | rd::<u64>(ctx + 0x20), rd::<u64>(ctx + 0x28)]);
    // Children may grow or shrink the list the loop re-reads (count at cmd+0x2a0).
    let cmd = rd::<usize>(child + 8);
    if cmd != 0 {
        let c = rd::<u32>(cmd + 0x2a0);
        if c > 1 {
            wr::<u32>(cmd + 0x2a0, c - 1);
        }
    }
}

unsafe extern "win64" fn m_child_log(child: usize, ctx: usize) {
    log([0xc41e, child as u64, rd::<u64>(ctx), rd::<u64>(ctx + 8) | rd::<u64>(ctx + 0x10), rd::<u64>(ctx + 0x18) | rd::<u64>(ctx + 0x20), rd::<u64>(ctx + 0x28)]);
}

/// 0x142376a30: real lookups 0x142378130/0x142378090 on random registries; 0x1420549b0 and
/// 0x1422e6e80 mocked (recorders: descriptor hash/contents, viewport size), children with a
/// logging vfunc_3 (one variant shrinks the child count the loop re-reads). Handles cover every
/// path (direct/inline color and depth, kind-4 `+0x40 == 0` early return, size mismatch, both
/// null, child count 0); compares the call log and the whole world.
pub unsafe fn diff_render_to_texture_execute(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(SET_RENDER_TARGETS_VA, m_set_render_targets as *const ())?;
    let _m2 = crate::difftest::Mock::new(SET_VIEWPORT_VA, m_set_viewport as *const ())?;
    let mut rng = crate::difftest::Rng::new(0x1423_76a3);
    let mut cases = 0u32;
    let mut ran_children = 0u32;
    for round in 0..96u32 {
        for mis in [0usize, 4] {
            let a = Arena::new(A_SIZE, mis);
            let handles = build_world(&a, &mut rng);
            let _g = RegistryGuard::install(a.at(A_REG));
            let cmd = a.at(A_CMD);
            for q in 0..0x2b0usize / 8 {
                wr::<u64>(cmd + q * 8, rng.next_u64());
            }
            let pick = |rng: &mut crate::difftest::Rng, kind: u8| -> u32 {
                if rng.below(3) == 0 { rng.next_u32() & 0xffff_0000 } else { pick_handle(rng, &handles, kind) }
            };
            wr::<u32>(cmd + 0x290, if round % 4 < 2 { pick(&mut rng, 1) } else { 0 });
            wr::<u32>(cmd + 0x294, pick(&mut rng, 4));
            wr::<u32>(cmd + 0x298, if round % 2 == 0 { pick(&mut rng, 2) } else { 0 });
            wr::<u32>(cmd + 0x29c, pick(&mut rng, 5));
            let nchild = [0u32, 1, 3, 8][rng.below(4) as usize];
            wr::<u32>(cmd + 0x2a0, nchild);
            let vt = a.at(A_VTBL);
            let shrink = round % 5 == 4;
            wr::<usize>(vt + 0x18, if shrink { m_child_exec as *const () as usize } else { m_child_log as *const () as usize });
            for i in 0..8usize {
                let ch = a.at(A_CHILDREN + i * 0x10);
                wr::<usize>(ch, vt);
                wr::<usize>(ch + 8, cmd);
                wr::<usize>(cmd + 0x90 + i * 8, ch);
            }
            let init = a.bytes();
            let mut out = Vec::new();
            for f in [o, n] {
                std::ptr::copy_nonoverlapping(init.as_ptr(), a.base as *mut u8, a.len);
                take_log();
                f(cmd, a.at(A_PASS));
                out.push((a.bytes(), take_log()));
            }
            let what = format!("round {round} mis {mis} handles {:#x}/{:#x}/{:#x}/{:#x} children {nchild}", rd::<u32>(cmd + 0x290), rd::<u32>(cmd + 0x294), rd::<u32>(cmd + 0x298), rd::<u32>(cmd + 0x29c));
            crate::diff_eq!(out[0].1, out[1].1, "call log, {what}");
            if let Some(i) = out[0].0.iter().zip(out[1].0.iter()).position(|(x, y)| x != y) {
                return Err(format!("memory mismatch at world+{i:#x} for {what}"));
            }
            if out[0].1.iter().any(|e| e[0] == 0xc41d || e[0] == 0xc41e) {
                ran_children += 1;
            }
            cases += 1;
        }
    }
    if ran_children < 8 {
        return Err(format!("only {ran_children} cases reached the child loop"));
    }
    Ok(cases)
}

unsafe extern "win64" fn m_cmd_exec(cmd: usize, pass: usize) {
    log([0xe8ec, cmd as u64, pass as u64, 0, 0, 0]);
}

/// 0x14236d960: lambda objects at +0/+4 addresses whose command's vfunc_0 is a recorder; the
/// argument cell holds random pass pointers.
pub unsafe fn diff_declare_lambda_do_call(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1423_6d96);
    let mut cases = 0u32;
    for mis in [0usize, 4] {
        let a = Arena::new(0x100, mis);
        let (lam, cmd, vt, cell) = (a.at(0), a.at(0x40), a.at(0x80), a.at(0xc0));
        wr::<usize>(vt, m_cmd_exec as *const () as usize);
        wr::<usize>(cmd, vt);
        wr::<u64>(lam, rng.next_u64());
        wr::<usize>(lam + 8, cmd);
        for _ in 0..8 {
            wr::<u64>(cell, rng.next_u64());
            take_log();
            o(lam, cell);
            let lo = take_log();
            n(lam, cell);
            let ln = take_log();
            crate::diff_eq!(lo, ln, "mis {mis} pass {:#x}", rd::<u64>(cell));
            cases += 1;
        }
    }
    Ok(cases)
}

thread_local! {
    static TABLE_RET: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static BIND_RET: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static TLS_BASE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

unsafe extern "win64" fn m_variants(p: usize) -> usize {
    log([COMPUTE_VARIANTS_VA, p as u64, 0, 0, 0, 0]);
    TABLE_RET.with(|t| t.get())
}

unsafe extern "win64" fn m_bind(a: usize, pass: usize, v: usize) -> u64 {
    log([BIND_COMPUTE_PARAMS_VA, a as u64, pass as u64, v as u64, 0, 0]);
    BIND_RET.with(|b| b.get())
}

unsafe extern "win64" fn m_state(cl: u64, a: u32, b: u32) {
    log([SET_COMPUTE_STATE_VA, cl, a as u64, b as u64, 0, 0]);
}

unsafe extern "win64" fn m_shader(v: usize) {
    log([SET_COMPUTE_SHADER_VA, v as u64, 0, 0, 0, 0]);
}

unsafe extern "win64" fn m_dispatch(ctx: usize, dims: usize) {
    let off = ctx.wrapping_sub(TLS_BASE.with(|t| t.get()));
    log([DISPATCH_COMPUTE_VA, off as u64, rd::<u64>(dims), rd::<u32>(dims + 8) as u64, 0, 0]);
}

unsafe extern "win64" fn m_mark(t: u64) {
    log([MARK_UAV_WRITTEN_VA, t, 0, 0, 0, 0]);
}

// Compute world layout.
const C_CMD: usize = 0x000;
const C_BIND: usize = 0x100; // 16 x 0x10
const C_HCELLS: usize = 0x200; // 16 x 4
const C_TABLE: usize = 0x300;
const C_VARR: usize = 0x400; // 8 pointers
const C_VARS: usize = 0x500; // 8 x 0xa0
const C_LAYS: usize = 0xa00; // 8 x 0xa0
const C_GROUPS: usize = 0xf00; // 8 x 3 x 0x28
const C_ENTRIES: usize = 0x1300; // entry pool
const C_SIZE: usize = 0x3000;
/// Static TLS block size covering `+0x467e0` and `+0x6308`.
const FAKE_TLS_SIZE: usize = 0x47400;

/// Puts a fake block into the exe's static TLS slot 0 for the guard's lifetime.
struct Tls0Guard {
    arr: usize,
    saved: usize,
}

impl Drop for Tls0Guard {
    fn drop(&mut self) {
        unsafe { wr::<usize>(self.arr, self.saved) }
    }
}

/// 0x142375c70: callees 0x142377ae0 / 0x14236cf00 / 0x1421f0110 / 0x14224ded0 / 0x14206ee70 /
/// 0x142067c10 mocked (recorders; table and bind result chosen per case), real lookups on a
/// random registry, exe static TLS slot 0 pointed at a fake block. Random sorted variant tables
/// (0..8 variants, keys hit/miss, null `+0x40`), layouts with groups and lists referencing
/// class-5 and other binding ids (test classes written into the 0x143858590 table under a
/// snapshot: it is filled at runtime and zero under --diff-test), bindings of types
/// 0xd/0xf/other; +0/+4 arenas. Compares the call log and both worlds.
pub unsafe fn diff_dispatch_compute_shader(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let arr = exe::tls_pointer();
    if arr == 0 {
        return Err(format!("{} no static TLS array on this thread", crate::difftest::SKIP));
    }
    // Binding classes: the table at 0x143858590 is filled at runtime (zero under --diff-test);
    // give ids 0..256 test classes for the duration (restored by the snapshot guard).
    let classes = exe::ptr::<u8>(BINDING_CLASS_TABLE_VA) as usize;
    let _class_snap = RegistryGuard(crate::difftest::Snapshot::take(classes as *mut u8, 256 * 4));
    let mut ids5 = Vec::new();
    let mut ids_other = Vec::new();
    let mut crng = crate::difftest::Rng::new(0x1438_5859);
    for i in 0..256i32 {
        let c = if crng.below(3) == 0 { 5 } else { [0i32, 1, 4, 6, -1][crng.below(5) as usize] };
        wr::<i32>(classes + i as usize * 4, c);
        if c == 5 {
            ids5.push(i);
        } else {
            ids_other.push(i);
        }
    }
    if ids5.is_empty() || ids_other.is_empty() {
        return Err(format!("binding class table has no class-5/other ids in 0..256 ({} / {})", ids5.len(), ids_other.len()));
    }
    let _mocks = [
        crate::difftest::Mock::new(COMPUTE_VARIANTS_VA, m_variants as *const ())?,
        crate::difftest::Mock::new(BIND_COMPUTE_PARAMS_VA, m_bind as *const ())?,
        crate::difftest::Mock::new(SET_COMPUTE_STATE_VA, m_state as *const ())?,
        crate::difftest::Mock::new(SET_COMPUTE_SHADER_VA, m_shader as *const ())?,
        crate::difftest::Mock::new(DISPATCH_COMPUTE_VA, m_dispatch as *const ())?,
        crate::difftest::Mock::new(MARK_UAV_WRITTEN_VA, m_mark as *const ())?,
    ];
    let tls_buf = Arena::new(FAKE_TLS_SIZE, 0);
    wr::<u8>(tls_buf.at(0x70), 1);
    let _tg = Tls0Guard { arr, saved: rd::<usize>(arr) };
    wr::<usize>(arr, tls_buf.at(0));
    TLS_BASE.with(|t| t.set(tls_buf.at(0)));

    let mut rng = crate::difftest::Rng::new(0x1423_75c7);
    let mut cases = 0u32;
    let mut marked = 0u32;
    let mut focus_marked = [0u32; 8];
    for round in 0..512u32 {
        for mis in [0usize, 4] {
            let lw = Arena::new(A_SIZE, mis);
            let handles = build_world(&lw, &mut rng);
            let _g = RegistryGuard::install(lw.at(A_REG));
            let cw = Arena::new(C_SIZE, mis);
            wr::<u64>(tls_buf.at(0x6308), rng.next_u64());
            // Every other round populates a single list (cycling through all eight) and no
            // groups, and is steered to reach the marking loop (variant hit, bind succeeds,
            // mostly hot class-5 ids); every fourth+1 round only groups: each position must
            // count on its own.
            let focus_list = if round % 2 == 0 { Some((round / 2) as usize % 8) } else { None };
            let focus = focus_list.is_some();
            let groups_only = round % 4 == 1;
            let pick_id = |rng: &mut crate::difftest::Rng| -> i32 {
                match rng.below(if focus { 8 } else { 4 }) {
                    0 => ids_other[rng.below(ids_other.len() as u64) as usize],
                    1 => rng.below(256) as i32,
                    _ => ids5[rng.below(ids5.len().min(6) as u64) as usize],
                }
            };
            // Entry pool.
            let mut ep = cw.at(C_ENTRIES);
            let mut alloc_entries = |rng: &mut crate::difftest::Rng, n: usize, stride: usize| -> usize {
                let at = ep;
                for i in 0..n {
                    wr::<i32>(at + i * stride, pick_id(rng));
                    wr::<u32>(at + i * stride + 4, rng.next_u32());
                }
                ep += n * stride + 8;
                at
            };
            // Layouts.
            for l in 0..8usize {
                let lay = cw.at(C_LAYS + l * 0xa0);
                let ng = if focus_list.is_some() { 0 } else { rng.below(4) as usize };
                let groups = cw.at(C_GROUPS + l * 3 * 0x28);
                for g in 0..ng {
                    let gp = groups + g * 0x28;
                    let cnt = rng.below(3) as usize;
                    wr::<usize>(gp + 0x18, alloc_entries(&mut rng, cnt, 0x18));
                    wr::<u32>(gp + 0x20, cnt as u32);
                }
                wr::<usize>(lay + 0x88, groups);
                wr::<u32>(lay + 0x90, ng as u32);
                for k in 0..8usize {
                    let cnt = match focus_list {
                        Some(fk) if fk == k => 1 + rng.below(3) as usize,
                        Some(_) => 0,
                        None if groups_only => 0,
                        None => if rng.below(3) == 0 { rng.below(3) as usize } else { 0 },
                    };
                    wr::<usize>(lay + 8 + k * 0x10, alloc_entries(&mut rng, cnt, 0x1c));
                    wr::<u32>(lay + 8 + k * 0x10 + 8, cnt as u32);
                }
            }
            // Variants, sorted by key.
            let nv = if focus { 1 + rng.below(8) as usize } else { rng.below(9) as usize };
            let mut keys: Vec<u32> = (0..nv).map(|_| rng.below(64) as u32 * 3).collect();
            keys.sort_unstable();
            keys.dedup();
            for (i, &k) in keys.iter().enumerate() {
                let v = cw.at(C_VARS + i * 0xa0);
                wr::<u32>(v + 0x90, k);
                wr::<u64>(v + 0x40, if !focus && rng.below(6) == 0 { 0 } else { rng.next_u64() | 1 });
                for l in 0..6usize {
                    let lay = if rng.below(if focus { 6 } else { 3 }) == 0 { 0 } else { cw.at(C_LAYS + rng.below(8) as usize * 0xa0) };
                    wr::<usize>(v + 0x60 + l * 8, lay);
                }
                wr::<usize>(cw.at(C_VARR + i * 8), v);
            }
            let t = cw.at(C_TABLE);
            wr::<usize>(t, cw.at(C_VARR));
            wr::<u32>(t + 8, keys.len() as u32);
            let want = if !keys.is_empty() && (focus || rng.below(5) != 0) { keys[rng.below(keys.len() as u64) as usize] } else { rng.below(200) as u32 };
            let mask = if !focus && rng.below(3) == 0 { rng.next_u32() & 0xffff_0000 } else { 0 };
            wr::<u32>(t + 0xb8, want | mask);
            wr::<u32>(t + 0xbc, mask);
            // Command + bindings.
            let cmd = cw.at(C_CMD);
            let nb = if focus { 4 + rng.below(8) as usize } else { rng.below(12) as usize };
            for i in 0..nb {
                let b = cw.at(C_BIND + i * 0x10);
                wr::<i32>(b, pick_id(&mut rng));
                let ty = [0xdu8, 0xf, 0xd, 0xf, 0x3][rng.below(5) as usize];
                wr::<u8>(b + 4, ty);
                let cell = cw.at(C_HCELLS + i * 4);
                // Mostly the registry kind the type resolves through (0xd: 1, 0xf: 2).
                let matching = if ty == 0xf { 2 } else { 1 };
                let kind = if rng.below(5) == 0 { 3 - matching } else { matching };
                wr::<u32>(cell, pick_handle(&mut rng, &handles, kind));
                wr::<usize>(b + 8, cell);
            }
            wr::<usize>(cmd + 8, cw.at(C_BIND));
            wr::<u64>(cmd + 0x10, nb as u64);
            wr::<u64>(cmd + 0x28, rng.next_u64());
            wr::<u32>(cmd + 0x30, rng.next_u32());
            let table_ret = if !focus && round % 9 == 8 { 0 } else { t };
            let bind_ret = match if focus { 0 } else { round % 7 } {
                6 => 0x1234_5600u64,
                5 => 0x0000_0001u64 | (rng.next_u64() & !0xff),
                _ => 1,
            };
            let init = (lw.bytes(), cw.bytes());
            let mut out = Vec::new();
            for f in [o, n] {
                std::ptr::copy_nonoverlapping(init.0.as_ptr(), lw.base as *mut u8, lw.len);
                std::ptr::copy_nonoverlapping(init.1.as_ptr(), cw.base as *mut u8, cw.len);
                TABLE_RET.with(|c| c.set(table_ret));
                BIND_RET.with(|c| c.set(bind_ret));
                take_log();
                f(cmd, lw.at(A_PASS));
                out.push((lw.bytes(), cw.bytes(), take_log()));
            }
            let what = format!("round {round} mis {mis} variants {} want {want:#x} bindings {nb} bind {bind_ret:#x}", keys.len());
            crate::diff_eq!(out[0].2, out[1].2, "call log, {what}");
            if out[0].0 != out[1].0 || out[0].1 != out[1].1 {
                return Err(format!("memory mismatch for {what}"));
            }
            if out[0].2.iter().any(|e| e[0] == MARK_UAV_WRITTEN_VA) {
                marked += 1;
                if let Some(fk) = focus_list {
                    focus_marked[fk] += 1;
                }
            }
            cases += 1;
        }
    }
    if marked < 8 || focus_marked.iter().any(|&c| c < 8) {
        return Err(format!("too few cases reached 0x142067c10: {marked} total, per focused list {focus_marked:?}"));
    }
    Ok(cases)
}
