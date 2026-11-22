//! Utility compute kernels for DXR acceleration structures (MSL, compiled once at first use).
//!
//! * `aw2_rt_instances`: D3D12 instance descs → `MTLIndirectAccelerationStructureInstanceDescriptor`,
//!   plus the TLAS header and its instance-contributions array (docs/design/DXR.md §3.3).
//! * `aw2_rt_write`: write up to 8 qwords at a VA (BLAS headers, postbuild info).
//! * `aw2_rt_copy`: copy dwords between VAs (contributions arrays on clone/compact).
//!
//! All parameters are passed as one constant struct at buffer index 0, holding raw GPU VAs.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::metal::binding::MtlArgumentTable;
use crate::metal::binding::UploadRing;

pub type MtlComputePso = ProtocolObject<dyn MTLComputePipelineState>;

const SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

// IRRaytracingAccelerationStructureGPUHeader (ir_raytracing.h).
struct Header { ulong asid; ulong contributions; ulong pad[6]; };
// D3D12_RAYTRACING_INSTANCE_DESC.
struct D3DInstance { float m[12]; uint id_mask; uint contribution_flags; ulong blas; };
// MTLIndirectAccelerationStructureInstanceDescriptor (transform row-major, as D3D's).
struct MtlInstance { float m[12]; uint options; uint mask; uint ift; uint user_id; ulong asid; };

struct ConvertParams {
    ulong src;        // D3D instance descs, or VAs of them when `pointers`
    ulong dst;        // MtlInstance[count]
    ulong header;     // TLAS header VA; contributions follow at +64
    ulong tlas_asid;
    ulong null_asid;  // resident dummy BLAS for inactive instances
    uint count;
    uint pointers;
};

kernel void aw2_rt_instances(constant ConvertParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    device Header* h = reinterpret_cast<device Header*>(p.header);
    device uint* contributions = reinterpret_cast<device uint*>(p.header + 64);
    if (i == 0) {
        h->asid = p.tlas_asid;
        h->contributions = p.header + 64;
        for (uint k = 0; k < 6; k++) h->pad[k] = 0;
    }
    if (i >= p.count) return;
    device MtlInstance& d = reinterpret_cast<device MtlInstance*>(p.dst)[i];
    ulong sa = p.pointers != 0 ? reinterpret_cast<device const ulong*>(p.src)[i] : p.src + ulong(i) * 64;
    ulong blas = 0;
    uint id_mask = 0, contribution_flags = 0;
    if (sa != 0) {
        device const D3DInstance& s = *reinterpret_cast<device const D3DInstance*>(sa);
        for (uint k = 0; k < 12; k++) d.m[k] = s.m[k];
        blas = s.blas;
        id_mask = s.id_mask;
        contribution_flags = s.contribution_flags;
    } else {
        for (uint k = 0; k < 12; k++) d.m[k] = 0.0f;
    }
    ulong asid = blas != 0 ? reinterpret_cast<device const Header*>(blas)->asid : 0;
    // Inactive (null BLAS) instances stay in place (InstanceIndex and contributions are array
    // indices) with mask 0 and a valid dummy BLAS.
    bool active = asid != 0;
    d.options = (contribution_flags >> 24) & 0xFu;
    d.mask = active ? (id_mask >> 24) : 0u;
    d.ift = 0;
    d.user_id = id_mask & 0xFFFFFFu;
    d.asid = active ? asid : p.null_asid;
    contributions[i] = active ? (contribution_flags & 0xFFFFFFu) : 0u;
}

struct WriteParams { ulong dst; uint count; uint pad; ulong values[8]; };
kernel void aw2_rt_write(constant WriteParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    if (i < p.count) reinterpret_cast<device ulong*>(p.dst)[i] = p.values[i];
}

struct CopyParams { ulong dst; ulong src; uint words; uint pad; };
kernel void aw2_rt_copy(constant CopyParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    if (i < p.words) reinterpret_cast<device uint*>(p.dst)[i] = reinterpret_cast<device const uint*>(p.src)[i];
}
"#;

const NAMES: [&str; 3] = ["aw2_rt_instances", "aw2_rt_write", "aw2_rt_copy"];

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ConvertParams {
    pub src: u64,
    pub dst: u64,
    pub header: u64,
    pub tlas_asid: u64,
    pub null_asid: u64,
    pub count: u32,
    pub pointers: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WriteParams {
    pub dst: u64,
    pub count: u32,
    pub pad: u32,
    pub values: [u64; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CopyParams {
    pub dst: u64,
    pub src: u64,
    pub words: u32,
    pub pad: u32,
}

const _: () = {
    assert!(std::mem::size_of::<ConvertParams>() == 48);
    assert!(std::mem::size_of::<WriteParams>() == 80);
    assert!(std::mem::size_of::<CopyParams>() == 24);
};

struct Kernels(Result<HashMap<&'static str, Retained<MtlComputePso>>, String>);
unsafe impl Send for Kernels {}
unsafe impl Sync for Kernels {}

/// A kernel by name (`aw2_rt_instances`, `aw2_rt_write`, `aw2_rt_copy`).
pub fn kernel(name: &str) -> Result<&'static MtlComputePso, String> {
    static K: OnceLock<Kernels> = OnceLock::new();
    let k = K.get_or_init(|| {
        Kernels((|| {
            let lib = crate::metal::pipeline::library_from_source("rt kernels", SOURCE)?;
            let mut m = HashMap::new();
            for n in NAMES {
                let p = crate::metal::pipeline::compute_pipeline_from_function(&lib, n)?;
                m.insert(n, p);
            }
            Ok(m)
        })())
    });
    match &k.0 {
        Ok(m) => m.get(name).map(|p| &**p).ok_or_else(|| format!("no rt kernel {name}")),
        Err(e) => Err(e.clone()),
    }
}

fn bytes_of<T: Copy>(t: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()) }
}

/// Dispatch `name` over `threads` threads with `params` (uploaded to `ring`) at buffer 0 of
/// `table`, which is left bound on `enc` (the caller restores its own table and PSO).
pub fn dispatch<P: Copy>(enc: &ProtocolObject<dyn MTL4ComputeCommandEncoder>, table: &MtlArgumentTable, ring: &mut UploadRing, name: &str, params: &P, threads: u32) -> Result<(), String> {
    let pso = kernel(name)?;
    let va = ring.upload(bytes_of(params), 256)?;
    unsafe { table.setAddress_atIndex(va, 0) };
    enc.setArgumentTable(Some(table));
    enc.setComputePipelineState(pso);
    enc.dispatchThreads_threadsPerThreadgroup(MTLSize { width: threads.max(1) as usize, height: 1, depth: 1 }, MTLSize { width: 64, height: 1, depth: 1 });
    Ok(())
}

/// `aw2_rt_write`: `values` (at most 8 qwords) to `dst` (8-byte aligned).
pub fn write_qwords(enc: &ProtocolObject<dyn MTL4ComputeCommandEncoder>, table: &MtlArgumentTable, ring: &mut UploadRing, dst: u64, values: &[u64]) -> Result<(), String> {
    if values.len() > 8 || !dst.is_multiple_of(8) {
        return Err(format!("rt write of {} qwords at {dst:#x}", values.len()));
    }
    let mut p = WriteParams { dst, count: values.len() as u32, ..Default::default() };
    p.values[..values.len()].copy_from_slice(values);
    dispatch(enc, table, ring, "aw2_rt_write", &p, p.count)
}

/// `aw2_rt_copy`: `words` dwords from `src` to `dst`.
pub fn copy_dwords(enc: &ProtocolObject<dyn MTL4ComputeCommandEncoder>, table: &MtlArgumentTable, ring: &mut UploadRing, dst: u64, src: u64, words: u32) -> Result<(), String> {
    if words == 0 {
        return Ok(());
    }
    dispatch(enc, table, ring, "aw2_rt_copy", &CopyParams { dst, src, words, pad: 0 }, words)
}
