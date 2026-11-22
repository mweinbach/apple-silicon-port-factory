//! GPU-encoded `ExecuteIndirect` draws: an expansion kernel writes one
//! indirect-command-buffer (ICB) command per executed D3D12 command plus the
//! execution range `{location, count}`, and the render pass runs
//! `executeCommandsInBuffer:indirectBuffer:`. Only the commands below the
//! count buffer's value reach the GPU front end (the fallback path issues
//! `max_count` indirect draws and zeroes the surplus ones).
//!
//! Each ICB command binds everything the IR-converter runtime expects for a
//! draw (heaps, top-level argument buffer, draw arguments, index-type
//! uniforms, vertex buffers with dynamic strides) for the vertex and fragment
//! stages; pipeline and fixed-function state are inherited from the encoder.
//! Per-command root argument patches, index buffer views and vertex buffer
//! views are applied on the GPU. `AW2_EI_ICB=0` disables the path.

use super::indirect::{KernelPatch, MAX_PATCHES};
use super::pipeline::MAX_VERTEX_SLOTS;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub type MtlIcb = ProtocolObject<dyn MTLIndirectCommandBuffer>;

/// Per-command vertex buffer view overrides supported.
pub const MAX_VBO: usize = 8;
const VB: usize = MAX_VERTEX_SLOTS as usize;

/// Kernel parameters (matches the MSL `IcbParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IcbParams {
    /// `MTLResourceID` of the ICB.
    pub icb: u64,
    pub args: u64,
    pub count: u64,
    /// Per-command draw-argument copies (32-byte stride) when the pipeline
    /// reads draw parameters; 0 otherwise.
    pub args_out: u64,
    pub tlab_out: u64,
    pub tlab_in: u64,
    pub pred: u64,
    /// `MTLIndirectCommandBufferExecutionRange` written by thread 0.
    pub range_out: u64,
    pub heap: u64,
    pub sampler_heap: u64,
    /// Top-level argument buffer when no per-command patches exist.
    pub tlab_static: u64,
    pub ia16: u64,
    pub ia32: u64,
    pub ia_none: u64,
    /// Static index buffer (GPU address of index 0) when the signature has no IB view.
    pub ib: u64,
    pub vb: [u64; VB],
    pub vb_stride: [u32; VB],
    pub stride: u32,
    pub max_count: u32,
    pub work_offset: u32,
    pub work_size: u32,
    pub location: u32,
    pub tlab_size: u32,
    pub tlab_stride: u32,
    pub npatch: u32,
    pub pred_op: u32,
    /// Byte offset of a `D3D12_INDEX_BUFFER_VIEW` in each command, `u32::MAX` = none.
    pub ib_offset: u32,
    /// Static index format: 57 = R16_UINT, 42 = R32_UINT.
    pub ib_format: u32,
    pub indexed: u32,
    /// `MTLPrimitiveType` raw value.
    pub prim: u32,
    pub vb_mask: u32,
    pub nvbo: u32,
    /// Bit `0` (`DRAW_PARAMS_NEEDED`): write `args_out` and bind at slot 4;
    /// Bit `1` (`DRAW_PARAMS_HAS_PS`, used by `v3`): bind fragment argument buffers `0..5`.
    pub draw_params: u32,
    pub vbo_slot: [u32; MAX_VBO],
    pub vbo_off: [u32; MAX_VBO],
    pub patches: [KernelPatch; MAX_PATCHES],
}
const _: () = assert!(std::mem::size_of::<IcbParams>() == 15 * 8 + VB * 8 + VB * 4 + 16 * 4 + MAX_VBO * 8 + MAX_PATCHES * 16);

pub const DRAW_PARAMS_NEEDED: u32 = 1;
pub const DRAW_PARAMS_HAS_PS: u32 = 2;

impl Default for IcbParams {
    fn default() -> Self {
        // SAFETY: plain-old-data.
        unsafe { std::mem::zeroed() }
    }
}

impl IcbParams {
    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()) }
    }
}

const SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

#define VB 24
#define MAX_VBO 8
struct Patch { uint src; uint dst; uint dwords; uint kind; };
struct IcbParams {
    command_buffer icb;
    ulong args, count, args_out, tlab_out, tlab_in, pred, range_out;
    ulong heap, sampler_heap, tlab_static, ia16, ia32, ia_none, ib;
    ulong vb[VB];
    uint vb_stride[VB];
    uint stride, max_count, work_offset, work_size, location, tlab_size, tlab_stride, npatch, pred_op;
    uint ib_offset, ib_format, indexed, prim, vb_mask, nvbo, draw_params;
    uint vbo_slot[MAX_VBO];
    uint vbo_off[MAX_VBO];
    Patch patches[16];
};

template <typename T> static inline device T* ptr(ulong va) { return reinterpret_cast<device T*>(va); }
static inline uint rd(device const uchar* p, uint off) { return *(device const uint*)(p + off); }
static inline ulong rd64(device const uchar* p, uint off) { return (ulong)rd(p, off) | ((ulong)rd(p, off + 4) << 32); }

static inline void bind(thread render_command& rc, ulong va, uint index) {
    if (va != 0ul) {
        rc.set_vertex_buffer(ptr<uchar>(va), index);
        rc.set_fragment_buffer(ptr<uchar>(va), index);
    }
}

static void encode(constant IcbParams& p, uint i) {
    uint n = p.count != 0ul ? min(ptr<uint>(p.count)[0], p.max_count) : p.max_count;
    if (p.pred != 0ul) {
        ulong pv = ptr<ulong>(p.pred)[0];
        if ((p.pred_op == 0 && pv == 0ul) || (p.pred_op != 0 && pv != 0ul)) n = 0;
    }
    if (i == 0) {
        device uint* r = ptr<uint>(p.range_out);
        r[0] = p.location;
        r[1] = n;
    }
    if (i >= n) return;
    device const uchar* cmd = ptr<uchar>(p.args) + (ulong)i * p.stride;
    uint w0 = rd(cmd, p.work_offset), w1 = rd(cmd, p.work_offset + 4), w2 = rd(cmd, p.work_offset + 8), w3 = rd(cmd, p.work_offset + 12);
    uint w4 = p.work_size > 16 ? rd(cmd, p.work_offset + 16) : 0u;

    ulong tl = p.tlab_static;
    if (p.tlab_out != 0ul) {
        device uchar* t = ptr<uchar>(p.tlab_out) + (ulong)i * p.tlab_stride;
        device const uchar* tin = ptr<uchar>(p.tlab_in);
        for (uint b = 0; b < p.tlab_size; b += 4) *(device uint*)(t + b) = *(device const uint*)(tin + b);
        for (uint k = 0; k < p.npatch; ++k) {
            Patch q = p.patches[k];
            for (uint w = 0; w < q.dwords; ++w) *(device uint*)(t + q.dst + 4 * w) = q.kind == 1 ? i : rd(cmd, q.src + 4 * w);
        }
        tl = p.tlab_out + (ulong)i * p.tlab_stride;
    }

    render_command rc(p.icb, p.location + i);
    bind(rc, p.heap, 0);
    bind(rc, p.sampler_heap, 1);
    bind(rc, tl, 2);
    if (p.draw_params != 0) {
        device uint* da = ptr<uint>(p.args_out) + (ulong)i * 8;
        da[0] = w0; da[1] = w1; da[2] = w2; da[3] = w3; da[4] = w4;
        bind(rc, p.args_out + (ulong)i * 32, 4);
    }

    // Vertex buffers: static bindings, then per-command views.
    ulong vbs[VB];
    uint strides[VB];
    for (uint s = 0; s < VB; ++s) { vbs[s] = p.vb[s]; strides[s] = p.vb_stride[s]; }
    for (uint k = 0; k < p.nvbo; ++k) {
        uint s = p.vbo_slot[k];
        if (s < VB) {
            ulong va = rd64(cmd, p.vbo_off[k]);
            if (va != 0ul) { vbs[s] = va; strides[s] = rd(cmd, p.vbo_off[k] + 12); }
        }
    }
    for (uint s = 0; s < VB; ++s) {
        if ((p.vb_mask >> s) & 1u) {
            if (vbs[s] != 0ul) rc.set_vertex_buffer(ptr<uchar>(vbs[s]), (size_t)strides[s], 6 + s);
        }
    }

    primitive_type prim = (primitive_type)p.prim;
    if (p.indexed == 0) {
        bind(rc, p.ia_none, 5);
        // D3D12_DRAW_ARGUMENTS: VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation
        rc.draw_primitives(prim, w2, w0, w1, w3);
        return;
    }
    ulong ib = p.ib;
    uint fmt = p.ib_format;
    if (p.ib_offset != 0xFFFFFFFFu) {
        ib = rd64(cmd, p.ib_offset);
        if (rd(cmd, p.ib_offset + 8) == 0u) ib = 0ul;
        fmt = rd(cmd, p.ib_offset + 12);
    }
    // D3D12_DRAW_INDEXED_ARGUMENTS: IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation
    if (ib == 0ul || w0 == 0u || w1 == 0u) {
        rc.reset();
        return;
    }
    if (fmt == 42u) {
        bind(rc, p.ia32, 5);
        rc.draw_indexed_primitives(prim, w0, ptr<uint>(ib) + w2, w1, (int)w3, w4);
    } else {
        bind(rc, p.ia16, 5);
        rc.draw_indexed_primitives(prim, w0, ptr<ushort>(ib) + w2, w1, (int)w3, w4);
    }
}

kernel void aw2_ei_icb(constant IcbParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) { encode(p, i); }
vertex void aw2_ei_icb_vs(constant IcbParams& p [[buffer(0)]], uint i [[vertex_id]]) { encode(p, i); }

static void encode_v2(constant IcbParams& p, uint i) {
    uint n = p.count != 0ul ? min(ptr<uint>(p.count)[0], p.max_count) : p.max_count;
    if (p.pred != 0ul) {
        ulong pv = ptr<ulong>(p.pred)[0];
        if ((p.pred_op == 0 && pv == 0ul) || (p.pred_op != 0 && pv != 0ul)) n = 0;
    }
    if (i == 0) {
        device uint* r = ptr<uint>(p.range_out);
        r[0] = p.location;
        r[1] = n;
    }
    if (i >= n) return;
    device const uchar* cmd = ptr<uchar>(p.args) + (ulong)i * p.stride;
    uint w0 = rd(cmd, p.work_offset), w1 = rd(cmd, p.work_offset + 4);

    // Recycled StorageModePrivate ICB slots retain prior commands across IcbRing::reset(),
    // so culled/empty draws must reset their slot first, then exit before TLAB/buffer work.
    render_command rc(p.icb, p.location + i);
    if (w0 == 0u || w1 == 0u) {
        rc.reset();
        return;
    }
    ulong ib = p.ib;
    uint fmt = p.ib_format;
    if (p.indexed != 0) {
        if (p.ib_offset != 0xFFFFFFFFu) {
            ib = rd64(cmd, p.ib_offset);
            if (rd(cmd, p.ib_offset + 8) == 0u) ib = 0ul;
            fmt = rd(cmd, p.ib_offset + 12);
        }
        if (ib == 0ul) {
            rc.reset();
            return;
        }
    }

    uint w2 = rd(cmd, p.work_offset + 8), w3 = rd(cmd, p.work_offset + 12);
    uint w4 = p.work_size > 16 ? rd(cmd, p.work_offset + 16) : 0u;

    ulong tl = p.tlab_static;
    if (p.tlab_out != 0ul) {
        device uchar* t = ptr<uchar>(p.tlab_out) + (ulong)i * p.tlab_stride;
        device const uchar* tin = ptr<uchar>(p.tlab_in);
        uint b16 = p.tlab_size & ~15u;
        for (uint b = 0; b < b16; b += 16) *(device uint4*)(t + b) = *(device const uint4*)(tin + b);
        for (uint b = b16; b < p.tlab_size; b += 4) *(device uint*)(t + b) = *(device const uint*)(tin + b);
        for (uint k = 0; k < p.npatch; ++k) {
            Patch q = p.patches[k];
            for (uint w = 0; w < q.dwords; ++w) *(device uint*)(t + q.dst + 4 * w) = q.kind == 1 ? i : rd(cmd, q.src + 4 * w);
        }
        tl = p.tlab_out + (ulong)i * p.tlab_stride;
    }

    bind(rc, p.heap, 0);
    bind(rc, p.sampler_heap, 1);
    bind(rc, tl, 2);
    if (p.draw_params != 0) {
        device uint* da = ptr<uint>(p.args_out) + (ulong)i * 8;
        da[0] = w0; da[1] = w1; da[2] = w2; da[3] = w3; da[4] = w4;
        bind(rc, p.args_out + (ulong)i * 32, 4);
    }

    // Zero-spill vertex buffer binding: iterate only set bits in p.vb_mask without
    // materializing thread-local vbs[24] / strides[24] stack arrays.
    uint mask = p.vb_mask;
    if (p.nvbo == 0u) {
        while (mask != 0u) {
            uint s = ctz(mask);
            mask &= mask - 1u;
            ulong va = p.vb[s];
            if (va != 0ul) rc.set_vertex_buffer(ptr<uchar>(va), (size_t)p.vb_stride[s], 6 + s);
        }
    } else {
        while (mask != 0u) {
            uint s = ctz(mask);
            mask &= mask - 1u;
            ulong va = p.vb[s];
            uint st = p.vb_stride[s];
            for (uint k = 0; k < p.nvbo; ++k) {
                if (p.vbo_slot[k] == s) {
                    ulong ova = rd64(cmd, p.vbo_off[k]);
                    if (ova != 0ul) { va = ova; st = rd(cmd, p.vbo_off[k] + 12); }
                }
            }
            if (va != 0ul) rc.set_vertex_buffer(ptr<uchar>(va), (size_t)st, 6 + s);
        }
    }

    primitive_type prim = (primitive_type)p.prim;
    if (p.indexed == 0) {
        bind(rc, p.ia_none, 5);
        rc.draw_primitives(prim, w2, w0, w1, w3);
        return;
    }
    if (fmt == 42u) {
        bind(rc, p.ia32, 5);
        rc.draw_indexed_primitives(prim, w0, ptr<uint>(ib) + w2, w1, (int)w3, w4);
    } else {
        bind(rc, p.ia16, 5);
        rc.draw_indexed_primitives(prim, w0, ptr<ushort>(ib) + w2, w1, (int)w3, w4);
    }
}

kernel void aw2_ei_icb_v2(constant IcbParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) { encode_v2(p, i); }
vertex void aw2_ei_icb_vs_v2(constant IcbParams& p [[buffer(0)]], uint i [[vertex_id]]) { encode_v2(p, i); }

static inline void bind_v3(thread render_command& rc, ulong va, uint index, bool has_ps) {
    if (va != 0ul) {
        rc.set_vertex_buffer(ptr<uchar>(va), index);
        if (has_ps) {
            rc.set_fragment_buffer(ptr<uchar>(va), index);
        }
    }
}

static void encode_v3_inner(constant IcbParams& p, uint i, bool write_range) {
    uint n = p.count != 0ul ? min(ptr<uint>(p.count)[0], p.max_count) : p.max_count;
    if (p.pred != 0ul) {
        ulong pv = ptr<ulong>(p.pred)[0];
        if ((p.pred_op == 0 && pv == 0ul) || (p.pred_op != 0 && pv != 0ul)) n = 0;
    }
    if (write_range && i == 0) {
        device uint* r = ptr<uint>(p.range_out);
        r[0] = p.location;
        r[1] = n;
    }
    if (i >= n) return;
    device const uchar* cmd = ptr<uchar>(p.args) + (ulong)i * p.stride;
    uint w0 = rd(cmd, p.work_offset), w1 = rd(cmd, p.work_offset + 4);

    render_command rc(p.icb, p.location + i);
    if (w0 == 0u || w1 == 0u) {
        rc.reset();
        return;
    }
    ulong ib = p.ib;
    uint fmt = p.ib_format;
    if (p.indexed != 0) {
        if (p.ib_offset != 0xFFFFFFFFu) {
            ib = rd64(cmd, p.ib_offset);
            if (rd(cmd, p.ib_offset + 8) == 0u) ib = 0ul;
            fmt = rd(cmd, p.ib_offset + 12);
        }
        if (ib == 0ul) {
            rc.reset();
            return;
        }
    }

    uint w2 = rd(cmd, p.work_offset + 8), w3 = rd(cmd, p.work_offset + 12);
    uint w4 = p.work_size > 16 ? rd(cmd, p.work_offset + 16) : 0u;

    ulong tl = p.tlab_static;
    if (p.tlab_out != 0ul) {
        device uchar* t = ptr<uchar>(p.tlab_out) + (ulong)i * p.tlab_stride;
        device const uchar* tin = ptr<uchar>(p.tlab_in);
        uint b16 = p.tlab_size & ~15u;
        for (uint b = 0; b < b16; b += 16) *(device uint4*)(t + b) = *(device const uint4*)(tin + b);
        for (uint b = b16; b < p.tlab_size; b += 4) *(device uint*)(t + b) = *(device const uint*)(tin + b);
        for (uint k = 0; k < p.npatch; ++k) {
            Patch q = p.patches[k];
            for (uint w = 0; w < q.dwords; ++w) *(device uint*)(t + q.dst + 4 * w) = q.kind == 1 ? i : rd(cmd, q.src + 4 * w);
        }
        tl = p.tlab_out + (ulong)i * p.tlab_stride;
    }

    bool has_ps = (p.draw_params & 2u) != 0u;
    bind_v3(rc, p.heap, 0, has_ps);
    bind_v3(rc, p.sampler_heap, 1, has_ps);
    bind_v3(rc, tl, 2, has_ps);
    if ((p.draw_params & 1u) != 0u) {
        ulong da_va = p.args_out + (ulong)i * 32ul;
        *(device uint4*)ptr<uchar>(da_va) = uint4(w0, w1, w2, w3);
        *(device uint*)(ptr<uchar>(da_va) + 16ul) = w4;
        bind_v3(rc, da_va, 4, has_ps);
    }

    uint mask = p.vb_mask;
    if (p.nvbo == 0u) {
        while (mask != 0u) {
            uint s = ctz(mask);
            mask &= mask - 1u;
            ulong va = p.vb[s];
            if (va != 0ul) rc.set_vertex_buffer(ptr<uchar>(va), (size_t)p.vb_stride[s], 6 + s);
        }
    } else {
        while (mask != 0u) {
            uint s = ctz(mask);
            mask &= mask - 1u;
            ulong va = p.vb[s];
            uint st = p.vb_stride[s];
            for (uint k = 0; k < p.nvbo; ++k) {
                if (p.vbo_slot[k] == s) {
                    ulong ova = rd64(cmd, p.vbo_off[k]);
                    if (ova != 0ul) { va = ova; st = rd(cmd, p.vbo_off[k] + 12); }
                }
            }
            if (va != 0ul) rc.set_vertex_buffer(ptr<uchar>(va), (size_t)st, 6 + s);
        }
    }

    primitive_type prim = (primitive_type)p.prim;
    if (p.indexed == 0) {
        bind_v3(rc, p.ia_none, 5, has_ps);
        rc.draw_primitives(prim, w2, w0, w1, w3);
        return;
    }
    if (fmt == 42u) {
        bind_v3(rc, p.ia32, 5, has_ps);
        rc.draw_indexed_primitives(prim, w0, ptr<uint>(ib) + w2, w1, (int)w3, w4);
    } else {
        bind_v3(rc, p.ia16, 5, has_ps);
        rc.draw_indexed_primitives(prim, w0, ptr<ushort>(ib) + w2, w1, (int)w3, w4);
    }
}

struct PrepItem {
    ulong count;
    ulong pred;
    ulong range_out;
    ulong ind_out;
    uint max_count;
    uint location;
    uint pred_op;
    uint pad0;
};
struct PrepBatch {
    uint n_items;
    uint pad0;
    uint pad1;
    uint pad2;
    PrepItem items[64];
};

kernel void aw2_ei_icb_prep_batch(constant PrepBatch& b [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    if (i >= b.n_items) return;
    PrepItem it = b.items[i];
    uint n = it.count != 0ul ? min(ptr<uint>(it.count)[0], it.max_count) : it.max_count;
    if (it.pred != 0ul) {
        ulong pv = ptr<ulong>(it.pred)[0];
        if ((it.pred_op == 0u && pv == 0ul) || (it.pred_op != 0u && pv != 0ul)) n = 0u;
    }
    if (it.range_out != 0ul) {
        device uint* r = ptr<uint>(it.range_out);
        r[0] = it.location;
        r[1] = n;
    }
    if (it.ind_out != 0ul) {
        device uint* ind = ptr<uint>(it.ind_out);
        ind[0] = (n + 63u) >> 6u;
        ind[1] = 1u;
        ind[2] = 1u;
    }
}

kernel void aw2_ei_icb_v3(constant IcbParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) { encode_v3_inner(p, i, true); }
kernel void aw2_ei_icb_v3_clamped(constant IcbParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) { encode_v3_inner(p, i, false); }
vertex void aw2_ei_icb_vs_v3(constant IcbParams& p [[buffer(0)]], uint i [[vertex_id]]) { encode_v3_inner(p, i, true); }
"#;

/// Maximum items in a single `aw2_ei_icb_prep_batch` dispatch.
pub const MAX_PREP_BATCH: usize = 64;

/// Per-segment indirect threadgroup & ICB range preparation item (matches MSL `PrepItem`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PrepItem {
    pub count: u64,
    pub pred: u64,
    pub range_out: u64,
    pub ind_out: u64,
    pub max_count: u32,
    pub location: u32,
    pub pred_op: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<PrepItem>() == 48);

/// Batched indirect threadgroup & range preparation buffer (matches MSL `PrepBatch`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PrepBatch {
    pub n_items: u32,
    pub _pad: [u32; 3],
    pub items: [PrepItem; MAX_PREP_BATCH],
}
const _: () = assert!(std::mem::size_of::<PrepBatch>() == 16 + MAX_PREP_BATCH * 48);

impl Default for PrepBatch {
    fn default() -> Self {
        // SAFETY: plain-old-data.
        unsafe { std::mem::zeroed() }
    }
}

pub type MtlComputePso = ProtocolObject<dyn MTLComputePipelineState>;
pub type MtlRenderPso = ProtocolObject<dyn MTLRenderPipelineState>;

struct Lib(Result<Retained<super::pipeline::MtlLibrary>, String>);
unsafe impl Send for Lib {}
unsafe impl Sync for Lib {}

fn library() -> Result<&'static super::pipeline::MtlLibrary, String> {
    static L: OnceLock<Lib> = OnceLock::new();
    L.get_or_init(|| Lib(super::pipeline::library_from_source("indirect icb kernel", SRC))).0.as_deref().map_err(|e| e.clone())
}

struct Kernel(Result<Retained<MtlComputePso>, String>);
unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}

/// The ICB expansion kernel (compiled once).
pub fn kernel() -> Result<&'static MtlComputePso, String> {
    kernel_with(false)
}

/// The ICB expansion kernel (`v2 = true` selects zero-spill `aw2_ei_icb_v2`).
pub fn kernel_with(v2: bool) -> Result<&'static MtlComputePso, String> {
    let tg = super::pipeline::fast_bind_pso().then_some([64, 1, 1]);
    if v2 {
        static K2: OnceLock<Kernel> = OnceLock::new();
        let k = K2.get_or_init(|| Kernel(library().and_then(|lib| super::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_icb_v2", tg))));
        k.0.as_deref().map_err(|e| e.clone())
    } else {
        static K1: OnceLock<Kernel> = OnceLock::new();
        let k = K1.get_or_init(|| Kernel(library().and_then(|lib| super::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_icb", tg))));
        k.0.as_deref().map_err(|e| e.clone())
    }
}

/// Fragment-aware zero-spill v3 ICB compute expansion kernel (`AW2_ICB_FAST_V3=1`).
pub fn kernel_v3() -> Result<&'static MtlComputePso, String> {
    let tg = super::pipeline::fast_bind_pso().then_some([64, 1, 1]);
    static K3: OnceLock<Kernel> = OnceLock::new();
    let k = K3.get_or_init(|| Kernel(library().and_then(|lib| super::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_icb_v3", tg))));
    k.0.as_deref().map_err(|e| e.clone())
}

/// Indirect-threadgroup-clamped v3 ICB compute expansion kernel (`AW2_ICB_PREPASS_COMPUTE=1`),
/// where `aw2_ei_icb_prep_batch` already wrote `range_out` and `MTLDispatchThreadgroupsIndirectArguments`.
pub fn kernel_v3_clamped() -> Result<&'static MtlComputePso, String> {
    let tg = super::pipeline::fast_bind_pso().then_some([64, 1, 1]);
    static K3C: OnceLock<Kernel> = OnceLock::new();
    let k = K3C.get_or_init(|| Kernel(library().and_then(|lib| super::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_icb_v3_clamped", Some([64, 1, 1])))));
    let _ = tg;
    k.0.as_deref().map_err(|e| e.clone())
}

/// Batched indirect count -> `(range_out, indirect_tg_args)` preparation kernel (`AW2_ICB_PREPASS_COMPUTE=1`).
pub fn prep_batch_kernel() -> Result<&'static MtlComputePso, String> {
    let tg = super::pipeline::fast_bind_pso().then_some([64, 1, 1]);
    static KP: OnceLock<Kernel> = OnceLock::new();
    let k = KP.get_or_init(|| Kernel(library().and_then(|lib| super::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_icb_prep_batch", tg))));
    k.0.as_deref().map_err(|e| e.clone())
}

struct Expanders(Mutex<HashMap<(Vec<u32>, u32, u8), Retained<MtlRenderPso>>>);
unsafe impl Send for Expanders {}
unsafe impl Sync for Expanders {}

/// The vertex-stage expander for a render pass with `rtv_formats` (DXGI)
/// and `sample_count`: `drawPrimitives(point, 0, max_count)` with the
/// parameters at buffer 0 encodes the commands without leaving the pass.
pub fn expander(rtv_formats: &[u32], sample_count: u32) -> Result<Retained<MtlRenderPso>, String> {
    expander_with(rtv_formats, sample_count, false)
}

/// Vertex-stage ICB expander (`v2 = true` selects zero-spill `aw2_ei_icb_vs_v2`
/// under a distinct `PipelineCache` archive key).
pub fn expander_with(rtv_formats: &[u32], sample_count: u32, v2: bool) -> Result<Retained<MtlRenderPso>, String> {
    expander_ver(rtv_formats, sample_count, u8::from(v2))
}

/// Fragment-aware zero-spill v3 vertex-stage ICB expander (`AW2_ICB_FAST_V3=1`).
pub fn expander_v3(rtv_formats: &[u32], sample_count: u32) -> Result<Retained<MtlRenderPso>, String> {
    expander_ver(rtv_formats, sample_count, 2)
}

fn expander_ver(rtv_formats: &[u32], sample_count: u32, ver: u8) -> Result<Retained<MtlRenderPso>, String> {
    static E: OnceLock<Expanders> = OnceLock::new();
    let e = E.get_or_init(|| Expanders(Mutex::new(HashMap::new())));
    let key = (rtv_formats.to_vec(), sample_count.max(1), ver);
    if let Some(p) = e.0.lock().unwrap().get(&key) {
        return Ok(p.clone());
    }
    let lib = library()?;
    let (fn_name, label, salt) = match ver {
        2 => ("aw2_ei_icb_vs_v3", "aw2 ExecuteIndirect expander v3", "d3d12-metal ei expander v3"),
        1 => ("aw2_ei_icb_vs_v2", "aw2 ExecuteIndirect expander v2", "d3d12-metal ei expander v2"),
        _ => ("aw2_ei_icb_vs", "aw2 ExecuteIndirect expander", "d3d12-metal ei expander v1"),
    };
    let desc = MTL4RenderPipelineDescriptor::new();
    desc.setVertexFunctionDescriptor(Some(&super::pipeline::function_descriptor(lib, fn_name)));
    desc.setRasterizationEnabled(false);
    desc.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
    let ca = desc.colorAttachments();
    for (i, f) in rtv_formats.iter().enumerate().take(8) {
        let a = unsafe { ca.objectAtIndexedSubscript(i) };
        a.setPixelFormat(crate::format::info(*f).mtl);
    }
    unsafe { desc.setRasterSampleCount(key.1 as usize) };
    desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Point);
    let mut h = dxil_metal::hash::KeyHasher::new(salt);
    h.bytes(format!("{:?}|{}", key.0, key.1).as_bytes());
    let pso = super::pipeline::pso_cache()?.render_pipeline(Some(h.finish()), &desc).map_err(|e| format!("ExecuteIndirect expander: {e}"))?;
    e.0.lock().unwrap().insert(key, pso.clone());
    Ok(pso)
}

/// `AW2_ICB_FAST_V3=1` (default off): fragment-aware `aw2_ei_icb_vs_v3` ICB
/// expansion (skipping dead `rc.set_fragment_buffer` calls on depth/shadow
/// VS-only PSOs and vectorizing `da[0..3]` via `uint4`) plus tight
/// `Vertex` / `Vertex | Fragment` hoisted intra-pass encoder barrier.
pub fn fast_v3() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_ICB_FAST_V3").is_some_and(|v| v == "1"))
}

/// `AW2_EI_INPASS` (default on): encode in the render pass (vertex stage)
/// instead of a compute dispatch that splits the pass.
pub fn in_pass() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_EI_INPASS").is_none_or(|v| v != "0"))
}

/// `AW2_EI_ICB` (default on).
pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_EI_ICB").is_none_or(|v| v != "0"))
}

/// Minimum vertex buffer bind count needed by an `ExecuteIndirect` draw (`6` IR runtime
/// buffers plus the highest vertex buffer slot actually referenced). On iOS this selects
/// between 3 compact ICB tiers (`6`, `14`, or `30` vertex buffer slots) so vertex-pulling
/// and <=8-stream shaders allocate 2–4× smaller `MTLIndirectCommandBuffer`s.
#[inline]
pub fn required_vb_bind_count(vb_mask: u32, vb_offsets: &[(u32, u32)]) -> usize {
    if !cfg!(target_os = "ios") {
        return 6 + VB;
    }
    let mut max_slot_plus_1 = if vb_mask == 0 {
        0
    } else {
        (32 - vb_mask.leading_zeros()) as usize
    };
    for &(slot, _) in vb_offsets {
        max_slot_plus_1 = max_slot_plus_1.max(slot as usize + 1);
    }
    if max_slot_plus_1 == 0 {
        6
    } else if max_slot_plus_1 <= 8 {
        6 + 8
    } else {
        6 + VB
    }
}

struct Chunk {
    icb: Retained<MtlIcb>,
    cap: u32,
    off: u32,
    vb_binds: usize,
    alloc_bytes: u64,
}
unsafe impl Send for Chunk {}

impl Drop for Chunk {
    fn drop(&mut self) {
        super::device::note_icb_free(self.alloc_bytes);
        if let Some(d) = super::device() {
            d.remove_resident(ProtocolObject::from_ref(&*self.icb));
        }
    }
}

/// Chunks released by dropped (or on iOS, reset) rings, reused before creating new ICBs.
/// Creating one is expensive (the driver builds per-command state programs),
/// and allocators come and go, so chunks are recycled process-wide.
static POOL: std::sync::Mutex<Vec<Chunk>> = std::sync::Mutex::new(Vec::new());

/// Total `newIndirectCommandBufferWithDescriptor` allocations since last perf sample.
static NEW_ICB_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Return and reset the count of `newIndirectCommandBufferWithDescriptor` allocations
/// since the previous call.
pub fn take_new_icb_count() -> u64 {
    NEW_ICB_COUNT.swap(0, std::sync::atomic::Ordering::Relaxed)
}

/// Compact vs large iOS ICB pool bucket thresholds.
/// Compact chunks (`cap <= 16384`, ~6–12 MiB each) serve steady-state per-allocator draws
/// across frames in flight; large chunks (`cap > 16384`, ~40–46 MiB each) serve heavy
/// `ExecuteIndirect` passes (up to 2 passes/frame × 2 frames in flight = 4 chunks) and
/// must NEVER evict compact chunks or be stolen by small draws. `trim_pool()` still
/// releases 100% of idle pooled ICBs on iOS memory warnings / Jetsam pressure.
const IOS_COMPACT_CAP_MAX: u32 = 16384;
const IOS_COMPACT_STEAL_CEIL: u32 = 24576;
const IOS_MAX_POOLED_COMPACT: usize = 24;
const IOS_MAX_POOLED_COMPACT_BYTES: u64 = 160 << 20;
const IOS_MAX_POOLED_LARGE: usize = 4;

/// Pure iOS chunk-matching rule: compact requests (`n <= 16384`) never steal giant
/// (`cap > 24576`) pooled chunks, preventing duplicate 43–46 MiB allocations.
#[inline]
fn ios_chunk_fits(n: u32, need_vb_binds: usize, chunk_cap: u32, chunk_vb_binds: usize) -> bool {
    if chunk_vb_binds < need_vb_binds || chunk_cap < n {
        return false;
    }
    if n <= IOS_COMPACT_CAP_MAX && chunk_cap > IOS_COMPACT_STEAL_CEIL {
        return false;
    }
    true
}

/// Pure iOS 2-bucket pool trim selector: returns the indices to EVICT from `items`
/// `(cap, vb_binds, alloc_bytes)` so up to `IOS_MAX_POOLED_COMPACT` (and `<= 80 MiB`)
/// compact chunks AND up to `IOS_MAX_POOLED_LARGE` giant chunks stay warm.
fn ios_trim_pool_evictions(items: &[(u32, usize, u64)]) -> Vec<usize> {
    let mut compact: Vec<usize> = Vec::new();
    let mut large: Vec<usize> = Vec::new();
    for (i, &(cap, _, _)) in items.iter().enumerate() {
        if cap <= IOS_COMPACT_CAP_MAX {
            compact.push(i);
        } else {
            large.push(i);
        }
    }
    // Prefer larger capacity / wider VB count within each bucket so retained chunks
    // satisfy the broadest range of requests in their bucket.
    compact.sort_by(|&a, &b| items[b].0.cmp(&items[a].0).then_with(|| items[b].1.cmp(&items[a].1)));
    large.sort_by(|&a, &b| items[b].0.cmp(&items[a].0).then_with(|| items[b].1.cmp(&items[a].1)));

    let mut evict = Vec::new();
    let mut kept_compact = 0usize;
    let mut kept_compact_bytes = 0u64;
    for idx in compact {
        let bytes = items[idx].2;
        if kept_compact < IOS_MAX_POOLED_COMPACT
            && (kept_compact == 0 || kept_compact_bytes.saturating_add(bytes) <= IOS_MAX_POOLED_COMPACT_BYTES)
        {
            kept_compact += 1;
            kept_compact_bytes = kept_compact_bytes.saturating_add(bytes);
        } else {
            evict.push(idx);
        }
    }
    for idx in large.into_iter().skip(IOS_MAX_POOLED_LARGE) {
        evict.push(idx);
    }
    // Sort descending so callers can safely `swap_remove` without invalidating earlier indices.
    evict.sort_unstable_by(|a, b| b.cmp(a));
    evict
}

fn return_chunks_to_pool(chunks: &mut Vec<Chunk>) {
    if chunks.is_empty() {
        return;
    }
    for c in chunks.iter_mut() {
        c.off = 0;
    }
    let mut dropped = Vec::new();
    if let Ok(mut p) = POOL.lock() {
        p.append(chunks);
        if cfg!(target_os = "ios") && p.len() > IOS_MAX_POOLED_LARGE {
            let meta: Vec<(u32, usize, u64)> = p.iter().map(|c| (c.cap, c.vb_binds, c.alloc_bytes)).collect();
            for idx in ios_trim_pool_evictions(&meta) {
                dropped.push(p.swap_remove(idx));
            }
        }
    }
    if !dropped.is_empty() {
        // Defer residency removal and deallocation until all live Metal queues pass their
        // current timeline value so no in-flight GPU work can fault.
        super::queue::defer_release_all(dropped);
    }
}

/// Release all idle pooled `MTLIndirectCommandBuffer`s back to Metal/XNU once active queues
/// drain (called on iOS memory warnings).
pub fn trim_pool() -> usize {
    let dropped = if let Ok(mut p) = POOL.lock() {
        std::mem::take(&mut *p)
    } else {
        Vec::new()
    };
    let n = dropped.len();
    if !dropped.is_empty() {
        super::queue::defer_release_all(dropped);
    }
    n
}

/// Linear sub-allocator of ICB commands, reset with its command allocator
/// (after the GPU finished every list recorded on it).
#[derive(Default)]
pub struct IcbRing {
    chunks: Vec<Chunk>,
}
unsafe impl Send for IcbRing {}

impl Drop for IcbRing {
    fn drop(&mut self) {
        // D3D12 requires the allocator to outlive GPU execution of its lists.
        return_chunks_to_pool(&mut self.chunks);
    }
}

/// Minimum ICB chunk command count. On macOS, 32,768 commands (~40–50 MiB) minimizes chunk
/// splits; on iOS (6 GiB unified Jetsam limit), 4,096 commands (~4–6 MiB) avoids allocating
/// multi-hundred-MiB ICBs while scaling in 4,096-command steps for larger `max_count`.
#[cfg(target_os = "ios")]
const CHUNK_COMMANDS: u32 = 1 << 12;
#[cfg(not(target_os = "ios"))]
const CHUNK_COMMANDS: u32 = 1 << 15;

impl IcbRing {
    /// Rewind all chunk cursors to 0 while keeping chunks resident (safe ONLY after the
    /// render encoder consuming these ICBs has ended via `endEncoding()`, paired with a
    /// cross-encoder `barrierAfterQueueStages(Vertex|Fragment, Vertex|Dispatch)`).
    pub fn rewind(&mut self) {
        for c in &mut self.chunks {
            c.off = 0;
        }
    }

    pub fn reset(&mut self) {
        for c in &mut self.chunks {
            c.off = 0;
        }
        // On iOS, return this completed allocator's ICB chunks to the shared global pool
        // so the next in-flight frame allocator reuses them instead of allocating duplicate
        // multi-hundred-MiB ICB sets per frame-in-flight.
        #[cfg(target_os = "ios")]
        return_chunks_to_pool(&mut self.chunks);
    }

    /// Reserve `n` consecutive commands with full vertex buffer bind capacity.
    #[allow(dead_code)]
    pub fn alloc(&mut self, n: u32) -> Result<(Retained<MtlIcb>, u32), String> {
        self.alloc_for(n, 6 + VB)
    }

    /// Reserve `n` consecutive commands with at least `need_vb_binds` vertex buffer slots:
    /// returns `(icb, first command index)`.
    pub fn alloc_for(&mut self, n: u32, need_vb_binds: usize) -> Result<(Retained<MtlIcb>, u32), String> {
        // Prefer the narrowest existing chunk that satisfies both bind count and capacity.
        let mut best_idx: Option<usize> = None;
        for (i, c) in self.chunks.iter().enumerate() {
            if c.vb_binds >= need_vb_binds && c.cap - c.off >= n {
                match best_idx {
                    None => best_idx = Some(i),
                    Some(bi) if c.vb_binds < self.chunks[bi].vb_binds => best_idx = Some(i),
                    _ => {}
                }
            }
        }
        if let Some(i) = best_idx {
            let c = &mut self.chunks[i];
            let at = c.off;
            c.off += n;
            return Ok((c.icb.clone(), at));
        }

        let pooled = {
            let mut p = POOL.lock().unwrap();
            let mut best: Option<usize> = None;
            for (i, c) in p.iter().enumerate() {
                let fits = if cfg!(target_os = "ios") {
                    ios_chunk_fits(n, need_vb_binds, c.cap, c.vb_binds)
                } else {
                    c.vb_binds >= need_vb_binds && c.cap >= n
                };
                if fits {
                    match best {
                        None => best = Some(i),
                        Some(bi)
                            if (c.vb_binds, c.cap) < (p[bi].vb_binds, p[bi].cap) =>
                        {
                            best = Some(i);
                        }
                        _ => {}
                    }
                }
            }
            best.map(|i| p.swap_remove(i))
        };
        let mut chunk = match pooled {
            Some(mut c) => {
                c.off = 0;
                c
            }
            None => {
                let cap = if cfg!(target_os = "ios") {
                    CHUNK_COMMANDS.max(n.next_multiple_of(CHUNK_COMMANDS))
                } else {
                    CHUNK_COMMANDS.max(n.next_power_of_two())
                };
                super::encstats::inc(&super::encstats::c::ei_icb_new);
                let (icb, alloc_bytes) = new_icb(cap, need_vb_binds)?;
                Chunk { icb, cap, off: 0, vb_binds: need_vb_binds, alloc_bytes }
            }
        };
        let at = chunk.off;
        chunk.off += n;
        let icb = chunk.icb.clone();
        self.chunks.push(chunk);
        Ok((icb, at))
    }
}

fn new_icb(cap: u32, max_vb_binds: usize) -> Result<(Retained<MtlIcb>, u64), String> {
    let d = super::device().ok_or("no device")?;
    let desc = MTLIndirectCommandBufferDescriptor::new();
    desc.setCommandTypes(MTLIndirectCommandType::Draw | MTLIndirectCommandType::DrawIndexed);
    desc.setInheritPipelineState(true);
    desc.setInheritBuffers(false);
    desc.setInheritDepthStencilState(true);
    desc.setInheritDepthBias(true);
    desc.setInheritDepthClipMode(true);
    desc.setInheritCullMode(true);
    desc.setInheritFrontFacingWinding(true);
    desc.setInheritTriangleFillMode(true);
    desc.setMaxVertexBufferBindCount(max_vb_binds);
    desc.setMaxFragmentBufferBindCount(6);
    desc.setSupportDynamicAttributeStride(max_vb_binds > 6);
    let opts = MTLResourceOptions::StorageModePrivate | MTLResourceOptions::HazardTrackingModeUntracked;
    let icb = unsafe { d.dev.newIndirectCommandBufferWithDescriptor_maxCommandCount_options(&desc, cap as usize, opts) }.ok_or_else(|| format!("newIndirectCommandBuffer({cap}, vb={max_vb_binds})"))?;
    icb.setLabel(Some(&objc2_foundation::NSString::from_str("aw2 ExecuteIndirect ICB")));
    let alloc_bytes = MTLResource::allocatedSize(&*icb) as u64;
    NEW_ICB_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    super::device::note_icb_alloc(alloc_bytes);
    if cfg!(target_os = "ios") && alloc_bytes >= (4 << 20) {
        log::info!(
            target: "d3d",
            "icb alloc: {:.1} MiB cap={cap} vb_binds={max_vb_binds}",
            alloc_bytes as f64 / 1048576.0
        );
    }
    d.add_resident(ProtocolObject::from_ref(&*icb));
    Ok((icb, alloc_bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn kernel_compiles() {
        if std::env::var_os("AW2_ICB_FAST_V3").is_none() {
            assert!(!super::fast_v3());
        }
        if super::super::device().is_none() {
            return;
        }
        if let Err(e) = super::kernel() {
            panic!("{e}");
        }
        if let Err(e) = super::kernel_with(true) {
            panic!("v2 kernel: {e}");
        }
        if let Err(e) = super::kernel_v3() {
            panic!("v3 kernel: {e}");
        }
    }

    #[test]
    fn expander_builds() {
        if super::super::device().is_none() {
            return;
        }
        // One R8G8B8A8_UNORM target, no MSAA (v1, v2, and v3 + depth-only v3).
        if let Err(e) = super::expander(&[28], 1) {
            panic!("{e}");
        }
        if let Err(e) = super::expander_with(&[28], 1, true) {
            panic!("v2 expander: {e}");
        }
        if let Err(e) = super::expander_v3(&[28], 1) {
            panic!("v3 color expander: {e}");
        }
        if let Err(e) = super::expander_v3(&[], 1) {
            panic!("v3 depth-only expander: {e}");
        }
    }

    #[test]
    fn icb_ring_rewind_reuses_same_buffer_and_trim_pool_releases() {
        if super::super::device().is_none() {
            return;
        }
        let _ = super::trim_pool();
        let mut ring = super::IcbRing::default();
        // Verify compact 6-slot (0-VB) and 14-slot (<=8-VB) tiers allocate and stay distinct.
        let (icb_compact, off_c) = ring.alloc_for(64, 6).expect("alloc compact 6-slot");
        assert_eq!(off_c, 0);
        let (icb_mid, off_m) = ring.alloc_for(64, 14).expect("alloc mid 14-slot");
        assert_eq!(off_m, 0);
        assert_ne!(objc2::rc::Retained::as_ptr(&icb_compact), objc2::rc::Retained::as_ptr(&icb_mid));
        // Compact request prefers the narrowest fitting chunk (6-slot) over the wider 14-slot chunk.
        let (icb_compact2, off_c2) = ring.alloc_for(64, 6).expect("alloc compact 6-slot #2");
        assert_eq!(off_c2, 64);
        assert_eq!(objc2::rc::Retained::as_ptr(&icb_compact), objc2::rc::Retained::as_ptr(&icb_compact2));
        // Rewind resets all chunk cursors to 0.
        ring.rewind();
        let (icb_compact3, off_c3) = ring.alloc_for(64, 6).expect("alloc compact after rewind");
        assert_eq!(off_c3, 0);
        assert_eq!(objc2::rc::Retained::as_ptr(&icb_compact), objc2::rc::Retained::as_ptr(&icb_compact3));
        drop(icb_compact);
        drop(icb_compact2);
        drop(icb_compact3);
        drop(icb_mid);
        drop(ring);
        let freed = super::trim_pool();
        assert!(freed >= 2, "expected trim_pool to release at least 2 pooled ICB chunks");
    }

    #[test]
    fn ios_pool_policy_keeps_compact_chunks_and_caps_giants() {
        // Small requests must never steal giant (> 24576) chunks from the pool.
        assert!(!super::ios_chunk_fits(8192, 6, 61440, 6));
        assert!(!super::ios_chunk_fits(4096, 6, 57344, 6));
        assert!(super::ios_chunk_fits(4096, 6, 8192, 6));
        assert!(super::ios_chunk_fits(8192, 6, 12288, 6));
        assert!(super::ios_chunk_fits(57344, 6, 61440, 6));
        assert!(!super::ios_chunk_fits(8192, 14, 8192, 6));

        // 5 giants (40-46 MiB) + 6 compact (6.2 MiB) -> keep 4 largest giants and ALL 6 compact chunks!
        let mib = 1u64 << 20;
        let items = vec![
            (53248, 6, 40 * mib), // 0: giant #1 (smallest -> evicted)
            (57344, 6, 43 * mib), // 1: giant #2
            (57344, 6, 43 * mib), // 2: giant #3
            (61440, 6, 46 * mib), // 3: giant #4
            (61440, 6, 46 * mib), // 4: giant #5
            (8192, 6, 6 * mib),   // 5..10: compact chunks
            (8192, 6, 6 * mib),
            (8192, 6, 6 * mib),
            (12288, 6, 9 * mib),
            (8192, 6, 6 * mib),
            (8192, 6, 6 * mib),
        ];
        let evict = super::ios_trim_pool_evictions(&items);
        assert_eq!(evict, vec![0], "only the 5th smallest giant chunk should be evicted, got {evict:?}");
    }
}
