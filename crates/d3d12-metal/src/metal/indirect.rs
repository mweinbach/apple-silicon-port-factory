//! `ExecuteIndirect` support: command-signature layouts and the expansion
//! kernel.
//!
//! Metal indirect draws/dispatches read the same argument structs as D3D12
//! (`D3D12_DRAW_ARGUMENTS` = `MTLDrawPrimitivesIndirectArguments`, etc.),
//! but have no count buffer and no per-command root argument changes. When a
//! signature needs either, a utility kernel (our own MSL, not a game shader)
//! expands the game's argument buffer on the GPU into
//! * one packed draw/dispatch argument struct per command, with the
//!   instance count (draws) or thread-group counts (dispatches) zeroed for
//!   commands at or beyond the count buffer's value, and
//! * one patched copy of the current top-level argument buffer per command
//!   (root constants / root descriptors / incrementing constants).
//!
//! The CPU then issues `max_count` indirect draws/dispatches, binding the
//! command's argument buffer for each. Vertex/index buffer view arguments
//! would need CPU-side binding and are rejected (`LayoutError`).

use dxil_metal::root_signature::{RootParameterLayoutKind, RootSignatureLayout};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::sync::OnceLock;

/// `D3D12_INDIRECT_ARGUMENT_TYPE`.
pub mod arg {
    pub const DRAW: u32 = 0;
    pub const DRAW_INDEXED: u32 = 1;
    pub const DISPATCH: u32 = 2;
    pub const VERTEX_BUFFER_VIEW: u32 = 3;
    pub const INDEX_BUFFER_VIEW: u32 = 4;
    pub const CONSTANT: u32 = 5;
    pub const CONSTANT_BUFFER_VIEW: u32 = 6;
    pub const SHADER_RESOURCE_VIEW: u32 = 7;
    pub const UNORDERED_ACCESS_VIEW: u32 = 8;
    pub const DISPATCH_RAYS: u32 = 9;
    pub const DISPATCH_MESH: u32 = 10;
    pub const INCREMENTING_CONSTANT: u32 = 11;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Work {
    Draw,
    DrawIndexed,
    Dispatch,
    /// `D3D12_DISPATCH_MESH_ARGUMENTS` (= Metal's threadgroups-indirect layout).
    DispatchMesh,
}

impl Work {
    /// Size of the D3D12 (= Metal) argument struct in bytes.
    pub fn size(self) -> u32 {
        match self {
            Work::Draw => 16,
            Work::DrawIndexed => 20,
            Work::Dispatch | Work::DispatchMesh => 12,
        }
    }
    /// Dwords zeroed to make a command a no-op: (first, count).
    pub fn zero(self) -> (u32, u32) {
        match self {
            Work::Draw | Work::DrawIndexed => (1, 1), // InstanceCount
            Work::Dispatch | Work::DispatchMesh => (0, 3), // ThreadGroupCountX/Y/Z
        }
    }
}

/// A root-argument change carried by each command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RootPatch {
    pub param: u32,
    /// Destination dword within the parameter (constants).
    pub dest_dword: u32,
    /// Dwords written.
    pub dwords: u32,
    /// Byte offset of the source data in the command; `None` = the command
    /// index (INCREMENTING_CONSTANT).
    pub src: Option<u32>,
}

/// A parsed `D3D12_COMMAND_SIGNATURE_DESC`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndirectLayout {
    pub stride: u32,
    pub work: Work,
    /// Byte offset of the draw/dispatch arguments in a command.
    pub work_offset: u32,
    pub patches: Vec<RootPatch>,
    /// Per-command vertex buffer view changes: `(slot, byte_offset_in_command)` of
    /// `D3D12_VERTEX_BUFFER_VIEW` (16 bytes: `BufferLocation: u64, SizeInBytes: u32, StrideInBytes: u32`).
    pub vb_offsets: Vec<(u32, u32)>,
    /// Per-command index buffer view change: byte offset of `D3D12_INDEX_BUFFER_VIEW`
    /// (16 bytes: `BufferLocation: u64, SizeInBytes: u32, Format: u32`).
    pub ib_offset: Option<u32>,
}

/// `D3D12_INDIRECT_ARGUMENT_DESC`: (type, union dwords).
pub type ArgumentDesc = (u32, [u32; 3]);

impl IndirectLayout {
    pub fn new(stride: u32, args: &[ArgumentDesc]) -> Result<IndirectLayout, String> {
        let mut off = 0u32;
        let mut patches = Vec::new();
        let mut vb_offsets = Vec::new();
        let mut ib_offset = None;
        let mut work = None;
        for (i, &(ty, u)) in args.iter().enumerate() {
            if work.is_some() {
                return Err(format!("argument {i} (type {ty}) follows the draw/dispatch argument"));
            }
            match ty {
                arg::DRAW | arg::DRAW_INDEXED | arg::DISPATCH | arg::DISPATCH_MESH => {
                    let w = match ty {
                        arg::DRAW => Work::Draw,
                        arg::DRAW_INDEXED => Work::DrawIndexed,
                        arg::DISPATCH_MESH => Work::DispatchMesh,
                        _ => Work::Dispatch,
                    };
                    work = Some((w, off));
                    off += w.size();
                }
                arg::CONSTANT => {
                    patches.push(RootPatch { param: u[0], dest_dword: u[1], dwords: u[2], src: Some(off) });
                    off += 4 * u[2];
                }
                arg::CONSTANT_BUFFER_VIEW | arg::SHADER_RESOURCE_VIEW | arg::UNORDERED_ACCESS_VIEW => {
                    patches.push(RootPatch { param: u[0], dest_dword: 0, dwords: 2, src: Some(off) });
                    off += 8;
                }
                arg::INCREMENTING_CONSTANT => patches.push(RootPatch { param: u[0], dest_dword: u[1], dwords: 1, src: None }),
                arg::VERTEX_BUFFER_VIEW => {
                    vb_offsets.push((u[0], off));
                    off += 16;
                }
                arg::INDEX_BUFFER_VIEW => {
                    ib_offset = Some(off);
                    off += 16;
                }
                arg::DISPATCH_RAYS => return Err("indirect DispatchRays is not implemented".into()),
                _ => return Err(format!("indirect argument type {ty}")),
            }
        }
        let (work, work_offset) = work.ok_or("command signature without a draw/dispatch argument")?;
        if off > stride {
            return Err(format!("arguments ({off} bytes) exceed the stride ({stride})"));
        }
        if patches.len() > MAX_PATCHES {
            return Err(format!("{} root argument changes per command (max {MAX_PATCHES})", patches.len()));
        }
        Ok(IndirectLayout { stride, work, work_offset, patches, vb_offsets, ib_offset })
    }

    /// Whether `ExecuteIndirect` needs the expansion kernel.
    pub fn needs_expansion(&self, has_count: bool) -> bool {
        has_count || !self.patches.is_empty() || self.ib_offset.is_some()
    }
}

pub const MAX_PATCHES: usize = 16;

/// Kernel patch op (matches the MSL `Patch`).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct KernelPatch {
    /// Source byte offset in the command.
    pub src: u32,
    /// Destination byte offset in the top-level argument buffer.
    pub dst: u32,
    pub dwords: u32,
    /// 0 = copy from the command, 1 = command index.
    pub kind: u32,
}

/// Kernel parameters (matches the MSL `Params`; device addresses as u64).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct KernelParams {
    pub args: u64,
    pub count: u64,
    pub args_out: u64,
    pub tlab_out: u64,
    pub tlab_in: u64,
    pub pred: u64,
    pub base_ib_va: u64,
    pub stride: u32,
    pub max_count: u32,
    pub work_offset: u32,
    pub work_size: u32,
    pub out_stride: u32,
    pub zero_word: u32,
    pub zero_n: u32,
    pub tlab_size: u32,
    pub tlab_stride: u32,
    pub npatch: u32,
    pub pred_op: u32,
    pub ib_offset: u32,
    pub patches: [KernelPatch; MAX_PATCHES],
}

impl KernelParams {
    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()) }
    }
}

/// Bytes between consecutive expanded top-level argument buffers.
pub fn tlab_stride(size: u32) -> u32 {
    size.max(8).next_multiple_of(64)
}

impl IndirectLayout {
    /// Kernel ops against `layout` (the bound root signature).
    pub fn kernel_patches(&self, layout: &RootSignatureLayout) -> Result<Vec<KernelPatch>, String> {
        self.patches
            .iter()
            .map(|p| {
                let pl = layout.parameters.get(p.param as usize).ok_or_else(|| format!("indirect root parameter {} out of range", p.param))?;
                match (&pl.kind, p.src) {
                    (RootParameterLayoutKind::Constants { num_32bit_values, .. }, _) if p.dest_dword + p.dwords <= *num_32bit_values => {}
                    (RootParameterLayoutKind::Descriptor { .. }, Some(_)) if p.dwords == 2 => {}
                    (k, _) => return Err(format!("indirect argument {p:?} does not match root parameter kind {k:?}")),
                }
                Ok(KernelPatch { src: p.src.unwrap_or(0), dst: pl.offset + 4 * p.dest_dword, dwords: p.dwords, kind: p.src.is_none() as u32 })
            })
            .collect()
    }
}

const SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Patch { uint src; uint dst; uint dwords; uint kind; };
struct Params {
    device const uchar* args;
    device const uint* count;
    device uchar* args_out;
    device uchar* tlab_out;
    device const uchar* tlab_in;
    device const ulong* pred;
    ulong base_ib_va;
    uint stride;
    uint max_count;
    uint work_offset;
    uint work_size;
    uint out_stride;
    uint zero_word;
    uint zero_n;
    uint tlab_size;
    uint tlab_stride;
    uint npatch;
    uint pred_op;
    uint ib_offset;
    Patch patches[16];
};

kernel void aw2_execute_indirect(constant Params& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    if (i >= p.max_count) return;
    uint n = p.count != nullptr ? min(p.count[0], p.max_count) : p.max_count;
    device const uchar* cmd = p.args + (ulong)i * p.stride;
    bool skip = i >= n;
    if (p.pred != nullptr) {
        ulong pv = p.pred[0];
        if ((p.pred_op == 0 && pv == 0) || (p.pred_op != 0 && pv != 0)) {
            skip = true;
        }
    }
    if (p.base_ib_va != 0ul) {
        device uint* o16 = (device uint*)(p.args_out + (ulong)(2u * i) * p.out_stride);
        device uint* o32 = (device uint*)(p.args_out + (ulong)(2u * i + 1u) * p.out_stride);
        for (uint w = 0; w < p.work_size / 4; ++w) {
            uint v = *(device const uint*)(cmd + p.work_offset + 4 * w);
            o16[w] = v;
            o32[w] = v;
        }
        ulong ib_va = *(device const ulong*)(cmd + p.ib_offset);
        uint ib_size = *(device const uint*)(cmd + p.ib_offset + 8);
        uint ib_fmt = *(device const uint*)(cmd + p.ib_offset + 12);
        if (skip || ib_va < p.base_ib_va || ib_size == 0u || o16[0] == 0u) {
            o16[1] = 0u;
            o32[1] = 0u;
        } else if (ib_fmt == 57u) {
            ulong idx_off = ((ib_va - p.base_ib_va) >> 1u) + (ulong)o16[2];
            if (idx_off > 0xFFFFFFFFul) {
                o16[1] = 0u;
            } else {
                o16[2] = (uint)idx_off;
            }
            o32[1] = 0u;
        } else if (ib_fmt == 42u) {
            o16[1] = 0u;
            ulong idx_off = ((ib_va - p.base_ib_va) >> 2u) + (ulong)o32[2];
            if (idx_off > 0xFFFFFFFFul) {
                o32[1] = 0u;
            } else {
                o32[2] = (uint)idx_off;
            }
        } else {
            o16[1] = 0u;
            o32[1] = 0u;
        }
    } else {
        device uint* o = (device uint*)(p.args_out + (ulong)i * p.out_stride);
        for (uint w = 0; w < p.work_size / 4; ++w) o[w] = *(device const uint*)(cmd + p.work_offset + 4 * w);
        if (skip) {
            for (uint w = 0; w < p.zero_n; ++w) o[p.zero_word + w] = 0;
        }
    }
    if (p.tlab_out != nullptr) {
        device uchar* t = p.tlab_out + (ulong)i * p.tlab_stride;
        for (uint b = 0; b < p.tlab_size; b += 4) *(device uint*)(t + b) = *(device const uint*)(p.tlab_in + b);
        for (uint k = 0; k < p.npatch; ++k) {
            Patch q = p.patches[k];
            for (uint w = 0; w < q.dwords; ++w) {
                *(device uint*)(t + q.dst + 4 * w) = q.kind == 1 ? i : *(device const uint*)(cmd + q.src + 4 * w);
            }
        }
    }
}
"#;

pub type MtlComputePso = ProtocolObject<dyn MTLComputePipelineState>;

struct Kernel(Result<Retained<MtlComputePso>, String>);
unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}

/// The expansion kernel (compiled once from source via `MTL4Compiler` targeting Metal 4.1).
pub fn kernel() -> Result<&'static MtlComputePso, String> {
    static K: OnceLock<Kernel> = OnceLock::new();
    let k = K.get_or_init(|| {
        Kernel((|| {
            let lib = super::pipeline::library_from_source("indirect kernel", SRC)?;
            let tg = super::pipeline::fast_bind_pso().then_some([64, 1, 1]);
            super::pipeline::compute_pipeline_from_function_tg(&lib, "aw2_execute_indirect", tg)
        })())
    });
    k.0.as_deref().map_err(|e| e.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layouts() {
        let l = IndirectLayout::new(16, &[(arg::DRAW, [0; 3])]).unwrap();
        assert_eq!((l.work, l.work_offset, l.patches.len()), (Work::Draw, 0, 0));
        assert!(!l.needs_expansion(false));
        // NRI-style draw-parameter emulation: 2 root constants then a draw.
        let l = IndirectLayout::new(28, &[(arg::CONSTANT, [3, 0, 2]), (arg::DRAW_INDEXED, [0; 3])]).unwrap();
        assert_eq!(l.work_offset, 8);
        assert_eq!(l.patches, vec![RootPatch { param: 3, dest_dword: 0, dwords: 2, src: Some(0) }]);
        assert!(l.needs_expansion(false));
        // IB view (16 bytes) + 4 root constants (16 bytes) + DrawIndexed (20 bytes) = 52 bytes.
        let l = IndirectLayout::new(52, &[(arg::INDEX_BUFFER_VIEW, [0; 3]), (arg::CONSTANT, [0, 0, 4]), (arg::DRAW_INDEXED, [0; 3])]).unwrap();
        assert_eq!((l.ib_offset, l.work_offset), (Some(0), 32));
        assert!(IndirectLayout::new(16, &[(arg::VERTEX_BUFFER_VIEW, [0; 3]), (arg::DRAW, [0; 3])]).is_err());
        assert!(IndirectLayout::new(8, &[(arg::DRAW, [0; 3])]).is_err());
        let l = IndirectLayout::new(12, &[(arg::DISPATCH_MESH, [0; 3])]).unwrap();
        assert_eq!((l.work, l.work_offset, l.work.zero()), (Work::DispatchMesh, 0, (0, 3)));
        assert!(IndirectLayout::new(32, &[(arg::DRAW, [0; 3]), (arg::CONSTANT, [0, 0, 1])]).is_err());
        let l = IndirectLayout::new(20, &[(arg::INCREMENTING_CONSTANT, [1, 2, 0]), (arg::UNORDERED_ACCESS_VIEW, [0, 0, 0]), (arg::DISPATCH, [0; 3])]).unwrap();
        assert_eq!(l.work_offset, 8);
        assert_eq!(l.patches[0].src, None);
    }

    #[test]
    fn params_layout() {
        // 7 u64 + 12 u32 + 16 patches of 16 bytes.
        assert_eq!(std::mem::size_of::<KernelParams>(), 56 + 48 + 256);
    }

    /// Compiles the MSL and runs it on the GPU (arm64 hosts with Metal).
    #[test]
    fn kernel_expands_with_count() {
        let Some(d) = super::super::device() else { return };
        let k = match kernel() {
            Ok(k) => k,
            Err(e) => panic!("{e}"),
        };
        let opts = MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked;
        // 3 commands: [const c, DrawArgs(4 dwords)], stride 20; count = 2.
        let mut args = Vec::<u32>::new();
        for i in 0..3u32 {
            args.extend_from_slice(&[100 + i, 3, 7, 0, 0]);
        }
        let ab = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(args.as_ptr() as *mut _).unwrap(), args.len() * 4, opts) }.unwrap();
        let cb_ = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new([2u32].as_ptr() as *mut _).unwrap(), 4, opts) }.unwrap();
        let out = d.dev.newBufferWithLength_options(3 * 16, opts).unwrap();
        let tin: Vec<u32> = vec![0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD];
        let tinb = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(tin.as_ptr() as *mut _).unwrap(), 16, opts) }.unwrap();
        let tout = d.dev.newBufferWithLength_options(3 * 64, opts).unwrap();
        let predb = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new([0u64, 5u64].as_ptr() as *mut _).unwrap(), 16, opts) }.unwrap();
        let mut p = KernelParams {
            args: ab.gpuAddress(),
            count: cb_.gpuAddress(),
            args_out: out.gpuAddress(),
            tlab_out: tout.gpuAddress(),
            tlab_in: tinb.gpuAddress(),
            stride: 20,
            max_count: 3,
            work_offset: 4,
            work_size: 16,
            out_stride: 16,
            zero_word: 1,
            zero_n: 1,
            tlab_size: 16,
            tlab_stride: 64,
            npatch: 2,
            ..Default::default()
        };
        p.patches[0] = KernelPatch { src: 0, dst: 4, dwords: 1, kind: 0 };
        p.patches[1] = KernelPatch { src: 0, dst: 12, dwords: 1, kind: 1 };
        let pb = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(p.bytes().as_ptr() as *mut _).unwrap(), p.bytes().len(), opts) }.unwrap();

        let rsd = MTLResidencySetDescriptor::new();
        let rs = d.dev.newResidencySetWithDescriptor_error(&rsd).unwrap();
        for b in [&ab, &cb_, &out, &tinb, &tout, &predb, &pb] {
            rs.addAllocation(ProtocolObject::from_ref(&**b));
        }
        rs.commit();

        let atd = MTL4ArgumentTableDescriptor::new();
        atd.setMaxBufferBindCount(1);
        let table = d.dev.newArgumentTableWithDescriptor_error(&atd).unwrap();

        let q = d.dev.newMTL4CommandQueue().unwrap();
        q.addResidencySet(&rs);
        let alloc = d.dev.newCommandAllocator().unwrap();
        let ev = d.dev.newSharedEvent().unwrap();

        let run_once = |param_va: u64, seq: u64| {
            let cmd = d.dev.newCommandBuffer().unwrap();
            cmd.beginCommandBufferWithAllocator(&alloc);
            let e = cmd.computeCommandEncoder().unwrap();
            e.setComputePipelineState(k);
            unsafe { table.setAddress_atIndex(param_va, 0) };
            e.setArgumentTable(Some(&table));
            e.dispatchThreads_threadsPerThreadgroup(MTLSize { width: 3, height: 1, depth: 1 }, MTLSize { width: 64, height: 1, depth: 1 });
            e.endEncoding();
            cmd.endCommandBuffer();
            let mut cbs = [std::ptr::NonNull::from(&*cmd)];
            unsafe { q.commit_count(std::ptr::NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
            q.signalEvent_value(ProtocolObject::from_ref(&*ev), seq);
            assert!(ev.waitUntilSignaledValue_timeoutMS(seq, 5000));
        };

        run_once(pb.gpuAddress(), 1);
        let o = unsafe { std::slice::from_raw_parts(out.contents().as_ptr() as *const u32, 12) };
        assert_eq!(o, &[3, 7, 0, 0, 3, 7, 0, 0, 3, 0, 0, 0]);
        let t = unsafe { std::slice::from_raw_parts(tout.contents().as_ptr() as *const u32, 48) };
        for i in 0..3usize {
            assert_eq!(&t[i * 16..i * 16 + 4], &[0xAAAA, 100 + i as u32, 0xCCCC, i as u32]);
        }

        // Predicated off with EQUAL_ZERO (pred_op == 0) when pred[0] == 0.
        p.pred = predb.gpuAddress();
        p.pred_op = 0;
        let pb2 = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(p.bytes().as_ptr() as *mut _).unwrap(), p.bytes().len(), opts) }.unwrap();
        rs.addAllocation(ProtocolObject::from_ref(&*pb2));
        rs.commit();
        run_once(pb2.gpuAddress(), 2);
        let o2 = unsafe { std::slice::from_raw_parts(out.contents().as_ptr() as *const u32, 12) };
        assert_eq!(o2, &[3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0]);

        // Dual-slot D3D12_INDEX_BUFFER_VIEW expansion:
        // cmd 0: ib_fmt = 57 (R16_UINT), ib_va = base + 0x100, start_idx = 10 -> o16[2] = 0x80 + 10 = 138, o32[1] = 0
        // cmd 1: ib_fmt = 42 (R32_UINT), ib_va = base + 0x200, start_idx = 5  -> o16[1] = 0, o32[2] = 0x80 + 5 = 133
        // cmd 2: beyond count (count = 2) -> both o16[1] = 0 and o32[1] = 0
        let base_va = 0x100_0000_0000u64;
        let mut ib_cmds = Vec::<u32>::new();
        for (ib_off_bytes, fmt, start_idx) in [(0x100u64, 57u32, 10u32), (0x200u64, 42u32, 5u32), (0x300u64, 57u32, 1u32)] {
            let va = base_va + ib_off_bytes;
            ib_cmds.extend_from_slice(&[va as u32, (va >> 32) as u32, 64, fmt]);
            ib_cmds.extend_from_slice(&[36, 1, start_idx, 0, 0]);
        }
        let ab3 = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(ib_cmds.as_ptr() as *mut _).unwrap(), ib_cmds.len() * 4, opts) }.unwrap();
        let out3 = d.dev.newBufferWithLength_options(3 * 2 * 20, opts).unwrap();
        let p3 = KernelParams {
            args: ab3.gpuAddress(),
            count: cb_.gpuAddress(),
            args_out: out3.gpuAddress(),
            base_ib_va: base_va,
            ib_offset: 0,
            stride: 36,
            max_count: 3,
            work_offset: 16,
            work_size: 20,
            out_stride: 20,
            zero_word: 1,
            zero_n: 1,
            ..Default::default()
        };
        let pb3 = unsafe { d.dev.newBufferWithBytes_length_options(std::ptr::NonNull::new(p3.bytes().as_ptr() as *mut _).unwrap(), p3.bytes().len(), opts) }.unwrap();
        for b in [&ab3, &out3, &pb3] {
            rs.addAllocation(ProtocolObject::from_ref(&**b));
        }
        rs.commit();
        run_once(pb3.gpuAddress(), 3);
        let o3 = unsafe { std::slice::from_raw_parts(out3.contents().as_ptr() as *const u32, 30) };
        // cmd 0: o16 active (start_idx = 138), o32 zeroed
        assert_eq!(&o3[0..5], &[36, 1, 138, 0, 0]);
        assert_eq!(&o3[5..10], &[36, 0, 10, 0, 0]);
        // cmd 1: o16 zeroed, o32 active (start_idx = 133)
        assert_eq!(&o3[10..15], &[36, 0, 5, 0, 0]);
        assert_eq!(&o3[15..20], &[36, 1, 133, 0, 0]);
        // cmd 2: both zeroed
        assert_eq!(&o3[20..25], &[36, 0, 1, 0, 0]);
        assert_eq!(&o3[25..30], &[36, 0, 1, 0, 0]);
    }
}
