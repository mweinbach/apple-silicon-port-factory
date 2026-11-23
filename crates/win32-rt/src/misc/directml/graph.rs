//! Safe D3D12 buffer I/O (via `crate::d3d12` + `d3d12_metal`, zero `objc2-metal`),
//! DirectML execution plans, graph topological compilation, and dispatch execution.

use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::cmdlist::{
    CommandAllocator as MtlCommandAllocator, CommandList as MtlCommandList,
};
use d3d12_metal::metal::queue::{ListType, Queue as MtlQueue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use super::abi::*;
use super::ops::{execute_op, OpSpec};
use crate::com::{com_call_slot, com_call_slot_raw};
use crate::strings::rd;
use crate::types::{E_INVALIDARG, HRESULT};

// ============================================================================
// Safe D3D12 Buffer Memory Access (via crate::d3d12 + d3d12_metal, zero objc2-metal)
// ============================================================================

struct BufferCopier {
    queue: MtlQueue,
    alloc: MtlCommandAllocator,
    cl: MtlCommandList,
}

impl BufferCopier {
    fn new() -> Option<Self> {
        let queue = MtlQueue::new(ListType::Copy, "DirectML copy")?;
        let alloc = MtlCommandAllocator::new(ListType::Copy)?;
        let cl = MtlCommandList::new(ListType::Copy)?;
        Some(Self { queue, alloc, cl })
    }

    fn alloc_staging(size: u64, is_readback: bool) -> Option<Arc<MtlResource>> {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: size.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties {
            Type: if is_readback { heap_type::READBACK } else { heap_type::UPLOAD },
            ..Default::default()
        };
        let r = MtlResource::new_committed_default(&desc, &hp, "directml staging").ok()?;
        let arc = Arc::new(r);
        arc.register_ptr();
        Some(arc)
    }

    fn read_buffer(&mut self, src: &Arc<MtlResource>, offset: u64, len: usize) -> Option<Vec<u8>> {
        let rb = Self::alloc_staging(len as u64, true)?;
        self.alloc.reset();
        self.cl.begin(&self.alloc).ok()?;
        self.cl.barrier();
        self.cl.copy_buffer(&rb, 0, src, offset, len as u64).ok()?;
        self.cl.barrier();
        self.cl.close().ok()?;
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        let p = rb.cpu_ptr()?;
        let mut out = vec![0u8; len];
        unsafe { std::ptr::copy_nonoverlapping(p, out.as_mut_ptr(), len) };
        Some(out)
    }

    fn write_buffer(&mut self, dst: &Arc<MtlResource>, offset: u64, data: &[u8]) -> bool {
        if data.is_empty() {
            return true;
        }
        let Some(up) = Self::alloc_staging(data.len() as u64, false) else {
            return false;
        };
        let Some(p) = up.cpu_ptr() else {
            return false;
        };
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), p, data.len()) };
        self.alloc.reset();
        if self.cl.begin(&self.alloc).is_err() {
            return false;
        }
        self.cl.barrier();
        if self.cl.copy_buffer(dst, offset, &up, 0, data.len() as u64).is_err() {
            let _ = self.cl.close();
            return false;
        }
        self.cl.barrier();
        if self.cl.close().is_err() {
            return false;
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        true
    }
}

fn copier_lock() -> &'static Mutex<Option<BufferCopier>> {
    static C: OnceLock<Mutex<Option<BufferCopier>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

#[derive(Clone, Debug)]
pub(super) struct BufferBinding {
    pub(super) resource_ptr: usize,
    pub(super) offset: u64,
    pub(super) size: u64,
}

#[derive(Clone, Debug)]
pub(super) enum BindingSlot {
    None,
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
}

impl BindingSlot {
    pub(super) fn first_buffer(&self) -> Option<&BufferBinding> {
        match self {
            BindingSlot::None => None,
            BindingSlot::Buffer(b) => Some(b),
            BindingSlot::BufferArray(v) => v.first(),
        }
    }
}

pub(super) struct BindingTableState {
    pub(super) target_ptr: usize,
    pub(super) cpu_handle: usize,
    pub(super) gpu_handle: u64,
    pub(super) size_in_descriptors: u32,
    pub(super) inputs: Vec<BindingSlot>,
    pub(super) outputs: Vec<BindingSlot>,
    pub(super) temp: BindingSlot,
    pub(super) persistent: BindingSlot,
}

/// Read `len` bytes from a D3D12 buffer binding (fast-path on `cpu_ptr` / `shared_slice`,
/// GPU staging readback fallback via `BufferCopier`, or foreign COM `Map`/`Unmap`).
pub(super) unsafe fn read_d3d12_buffer(binding: &BufferBinding, min_len: usize) -> Option<Vec<u8>> {
    if binding.resource_ptr == 0 {
        return None;
    }
    let len = (binding.size as usize).max(min_len);
    if len == 0 {
        return Some(Vec::new());
    }
    if let Some(res) = crate::d3d12::resource::from_ptr(binding.resource_ptr as *mut c_void) {
        let width = res.r.desc.Width;
        if binding.offset >= width {
            return Some(vec![0u8; len]);
        }
        let avail = (width - binding.offset).min(len as u64) as usize;
        if let Some(p) = res.r.cpu_ptr() {
            let mut out = vec![0u8; len];
            std::ptr::copy_nonoverlapping(p.add(binding.offset as usize), out.as_mut_ptr(), avail);
            return Some(out);
        }
        if let Some(s) = res.r.shared_slice(binding.offset, avail) {
            let mut out = vec![0u8; len];
            out[..s.len()].copy_from_slice(s);
            return Some(out);
        }
        let mut cg = copier_lock().lock().unwrap();
        if cg.is_none() {
            *cg = BufferCopier::new();
        }
        if let Some(copier) = cg.as_mut() {
            if let Some(mut v) = copier.read_buffer(&res.r, binding.offset, avail) {
                v.resize(len, 0);
                return Some(v);
            }
        }
        return None;
    }
    // Foreign ID3D12Resource COM fallback (Map slot 8, Unmap slot 9)
    let p_res = binding.resource_ptr as *mut c_void;
    let mut mapped: *mut c_void = std::ptr::null_mut();
    if com_call_slot(p_res, 8, 0, 0, (&mut mapped as *mut *mut c_void) as u64) < 0
        || mapped.is_null()
    {
        return None;
    }
    let mut out = vec![0u8; len];
    std::ptr::copy_nonoverlapping(
        (mapped as *const u8).add(binding.offset as usize),
        out.as_mut_ptr(),
        len,
    );
    let _ = com_call_slot_raw(p_res, 9, &[0, 0]);
    Some(out)
}

/// Write `data` into a D3D12 buffer binding (`cpu_ptr` direct write, GPU upload via
/// `BufferCopier` for DEFAULT heaps, or foreign COM `Map`/`Unmap`).
pub(super) unsafe fn write_d3d12_buffer(binding: &BufferBinding, data: &[u8]) {
    if binding.resource_ptr == 0 || data.is_empty() {
        return;
    }
    let max_write = if binding.size > 0 {
        data.len().min(binding.size as usize)
    } else {
        data.len()
    };
    let slice = &data[..max_write];
    if let Some(res) = crate::d3d12::resource::from_ptr(binding.resource_ptr as *mut c_void) {
        let width = res.r.desc.Width;
        if binding.offset >= width {
            return;
        }
        let clamped = slice.len().min((width - binding.offset) as usize);
        if let Some(p) = res.r.cpu_ptr() {
            std::ptr::copy_nonoverlapping(slice.as_ptr(), p.add(binding.offset as usize), clamped);
            return;
        }
        let mut cg = copier_lock().lock().unwrap();
        if cg.is_none() {
            *cg = BufferCopier::new();
        }
        if let Some(copier) = cg.as_mut() {
            let _ = copier.write_buffer(&res.r, binding.offset, &slice[..clamped]);
        }
        return;
    }
    let p_res = binding.resource_ptr as *mut c_void;
    let mut mapped: *mut c_void = std::ptr::null_mut();
    if com_call_slot(p_res, 8, 0, 0, (&mut mapped as *mut *mut c_void) as u64) >= 0
        && !mapped.is_null()
    {
        std::ptr::copy_nonoverlapping(
            slice.as_ptr(),
            (mapped as *mut u8).add(binding.offset as usize),
            slice.len(),
        );
        let _ = com_call_slot_raw(p_res, 9, &[0, 0]);
    }
}

// ============================================================================
// Graph Execution Plan (for IDMLDevice1::CompileGraph)
// ============================================================================

#[derive(Clone)]
pub(super) struct GraphPlan {
    pub(super) input_count: usize,
    pub(super) output_count: usize,
    pub(super) nodes: Vec<OpSpec>,
    pub(super) order: Vec<usize>,
    /// `(graph_input_idx, to_node, to_node_input)`
    pub(super) input_edges: Vec<(usize, usize, usize)>,
    /// `(from_node, from_node_output, graph_output_idx)`
    pub(super) output_edges: Vec<(usize, usize, usize)>,
    /// `(from_node, from_node_output, to_node, to_node_input)`
    pub(super) inter_edges: Vec<(usize, usize, usize, usize)>,
}

#[derive(Clone)]
pub(super) enum ExecutionPlan {
    Single(OpSpec),
    Graph(GraphPlan),
}

pub(super) struct CompiledOpState {
    pub(super) plan: ExecutionPlan,
    pub(super) binding_props: DML_BINDING_PROPERTIES,
    /// Persistent-resource byte spans for `OWNED_BY_DML` inputs (single-op path).
    pub(super) persistent_inputs: Vec<(usize, u64, u64)>,
}

/// Parse and topologically sort a `DML_GRAPH_DESC`, returning the compiled
/// [`GraphPlan`] and required descriptor count.
pub(super) unsafe fn parse_graph_plan(
    desc_ptr: *const DML_GRAPH_DESC,
    resolve_op: impl Fn(*mut c_void) -> Option<OpSpec>,
) -> Result<(GraphPlan, u32), HRESULT> {
    if desc_ptr.is_null() {
        return Err(E_INVALIDARG);
    }
    let gd = rd(desc_ptr);
    if gd.NodeCount == 0 || gd.Nodes.is_null() {
        return Err(E_INVALIDARG);
    }
    let n_nodes = gd.NodeCount as usize;
    let mut nodes = Vec::with_capacity(n_nodes);
    for i in 0..n_nodes {
        let nd = rd(gd.Nodes.add(i));
        if nd.Type != DML_GRAPH_NODE_TYPE_OPERATOR || nd.Desc.is_null() {
            return Err(E_INVALIDARG);
        }
        let ond = rd(nd.Desc as *const DML_OPERATOR_GRAPH_NODE_DESC);
        let Some(op_spec) = resolve_op(ond.Operator) else {
            return Err(E_INVALIDARG);
        };
        nodes.push(op_spec);
    }

    let mut input_edges = Vec::with_capacity(gd.InputEdgeCount as usize);
    if gd.InputEdgeCount > 0 {
        if gd.InputEdges.is_null() {
            return Err(E_INVALIDARG);
        }
        for i in 0..gd.InputEdgeCount as usize {
            let e = rd(gd.InputEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_INPUT || e.Desc.is_null() {
                return Err(E_INVALIDARG);
            }
            let ie = rd(e.Desc as *const DML_INPUT_GRAPH_EDGE_DESC);
            if ie.GraphInputIndex >= gd.InputCount || ie.ToNodeIndex >= gd.NodeCount {
                return Err(E_INVALIDARG);
            }
            input_edges.push((
                ie.GraphInputIndex as usize,
                ie.ToNodeIndex as usize,
                ie.ToNodeInputIndex as usize,
            ));
        }
    }

    let mut output_edges = Vec::with_capacity(gd.OutputEdgeCount as usize);
    if gd.OutputEdgeCount > 0 {
        if gd.OutputEdges.is_null() {
            return Err(E_INVALIDARG);
        }
        for i in 0..gd.OutputEdgeCount as usize {
            let e = rd(gd.OutputEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_OUTPUT || e.Desc.is_null() {
                return Err(E_INVALIDARG);
            }
            let oe = rd(e.Desc as *const DML_OUTPUT_GRAPH_EDGE_DESC);
            if oe.FromNodeIndex >= gd.NodeCount || oe.GraphOutputIndex >= gd.OutputCount {
                return Err(E_INVALIDARG);
            }
            output_edges.push((
                oe.FromNodeIndex as usize,
                oe.FromNodeOutputIndex as usize,
                oe.GraphOutputIndex as usize,
            ));
        }
    }

    let mut inter_edges = Vec::with_capacity(gd.IntermediateEdgeCount as usize);
    let mut in_deg = vec![0usize; n_nodes];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_nodes];
    if gd.IntermediateEdgeCount > 0 {
        if gd.IntermediateEdges.is_null() {
            return Err(E_INVALIDARG);
        }
        for i in 0..gd.IntermediateEdgeCount as usize {
            let e = rd(gd.IntermediateEdges.add(i));
            if e.Type != DML_GRAPH_EDGE_TYPE_INTERMEDIATE || e.Desc.is_null() {
                return Err(E_INVALIDARG);
            }
            let me = rd(e.Desc as *const DML_INTERMEDIATE_GRAPH_EDGE_DESC);
            if me.FromNodeIndex >= gd.NodeCount || me.ToNodeIndex >= gd.NodeCount {
                return Err(E_INVALIDARG);
            }
            let u = me.FromNodeIndex as usize;
            let v = me.ToNodeIndex as usize;
            inter_edges.push((
                u,
                me.FromNodeOutputIndex as usize,
                v,
                me.ToNodeInputIndex as usize,
            ));
            adj[u].push(v);
            in_deg[v] += 1;
        }
    }

    // Kahn's topological sort
    let mut queue: VecDeque<usize> = (0..n_nodes).filter(|&i| in_deg[i] == 0).collect();
    let mut order = Vec::with_capacity(n_nodes);
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if order.len() != n_nodes {
        return Err(E_INVALIDARG); // Cycle detected
    }

    let req_descs = (gd.InputCount + gd.OutputCount).max(1);
    Ok((
        GraphPlan {
            input_count: gd.InputCount as usize,
            output_count: gd.OutputCount as usize,
            nodes,
            order,
            input_edges,
            output_edges,
            inter_edges,
        },
        req_descs,
    ))
}

/// Execute `IDMLOperatorInitializer` initialization: copy any `DML_TENSOR_FLAG_OWNED_BY_DML`
/// input tensors from the initializer's bound inputs into each operator's persistent output buffer.
pub(super) unsafe fn run_initialize(ops: &[Arc<CompiledOpState>], bt: &BindingTableState) {
    crate::d3d12::cs::flush();
    for (op_idx, compiled) in ops.iter().enumerate() {
        if compiled.persistent_inputs.is_empty() {
            continue;
        }
        let Some(persist_dst) = bt.outputs.get(op_idx).and_then(|s| s.first_buffer()) else {
            continue;
        };
        let input_bindings: Vec<Option<&BufferBinding>> = match bt.inputs.get(op_idx) {
            Some(BindingSlot::BufferArray(arr)) => arr.iter().map(Some).collect(),
            Some(BindingSlot::Buffer(b)) => vec![Some(b)],
            _ => continue,
        };
        for &(in_slot, dst_rel_off, byte_len) in &compiled.persistent_inputs {
            if let Some(Some(src_b)) = input_bindings.get(in_slot) {
                if let Some(bytes) = read_d3d12_buffer(src_b, byte_len as usize) {
                    let sub_dst = BufferBinding {
                        resource_ptr: persist_dst.resource_ptr,
                        offset: persist_dst.offset.saturating_add(dst_rel_off),
                        size: byte_len,
                    };
                    write_d3d12_buffer(&sub_dst, &bytes[..byte_len as usize]);
                }
            }
        }
    }
}

/// Execute one compiled single operator against D3D12 buffer bindings.
pub(super) unsafe fn run_single_op(
    compiled: &CompiledOpState,
    spec: &OpSpec,
    bt: &BindingTableState,
) {
    crate::d3d12::cs::flush();
    let in_descs = spec.input_descs();
    let out_descs = spec.output_descs();
    let persist_buf = bt.persistent.first_buffer();

    // Flatten bindings: allow either N separate Buffer slots OR a single BufferArray (e.g. Join).
    let flat_inputs: Vec<Option<BufferBinding>> =
        if bt.inputs.len() == 1 && in_descs.len() > 1 {
            match &bt.inputs[0] {
                BindingSlot::BufferArray(arr) => arr.iter().cloned().map(Some).collect(),
                _ => bt.inputs.iter().map(|s| s.first_buffer().cloned()).collect(),
            }
        } else {
            bt.inputs.iter().map(|s| s.first_buffer().cloned()).collect()
        };

    let mut owned_inputs: Vec<Option<Vec<u8>>> = Vec::with_capacity(in_descs.len());
    for (i, td_opt) in in_descs.iter().enumerate() {
        let Some(td) = td_opt else {
            owned_inputs.push(None);
            continue;
        };
        if let Some(Some(bb)) = flat_inputs.get(i) {
            if bb.resource_ptr != 0 {
                owned_inputs.push(read_d3d12_buffer(bb, td.total_bytes as usize));
                continue;
            }
        }
        // Fallback for OWNED_BY_DML tensor stored in persistent buffer during Initialize.
        if let Some(&(_, p_off, p_len)) = compiled.persistent_inputs.iter().find(|&&(idx, _, _)| idx == i) {
            if let Some(pb) = persist_buf {
                let sub = BufferBinding {
                    resource_ptr: pb.resource_ptr,
                    offset: pb.offset.saturating_add(p_off),
                    size: p_len,
                };
                owned_inputs.push(read_d3d12_buffer(&sub, td.total_bytes as usize));
                continue;
            }
        }
        owned_inputs.push(None);
    }

    let mut owned_outputs: Vec<Option<Vec<u8>>> = out_descs
        .iter()
        .map(|td_opt| td_opt.map(|td| vec![0u8; td.total_bytes as usize]))
        .collect();

    {
        let in_slices: Vec<Option<&[u8]>> = owned_inputs.iter().map(|v| v.as_deref()).collect();
        let mut out_slices: Vec<Option<&mut [u8]>> =
            owned_outputs.iter_mut().map(|v| v.as_deref_mut()).collect();
        execute_op(spec, &in_slices, &mut out_slices);
    }

    let flat_outputs: Vec<Option<BufferBinding>> =
        if bt.outputs.len() == 1 && out_descs.len() > 1 {
            match &bt.outputs[0] {
                BindingSlot::BufferArray(arr) => arr.iter().cloned().map(Some).collect(),
                _ => bt.outputs.iter().map(|s| s.first_buffer().cloned()).collect(),
            }
        } else {
            bt.outputs.iter().map(|s| s.first_buffer().cloned()).collect()
        };

    for (i, out_data) in owned_outputs.into_iter().enumerate() {
        if let (Some(bytes), Some(Some(bb))) = (out_data, flat_outputs.get(i)) {
            write_d3d12_buffer(bb, &bytes);
        }
    }
}

/// Execute a compiled multi-node DirectML graph in topological order.
pub(super) unsafe fn run_graph(graph: &GraphPlan, bt: &BindingTableState) {
    crate::d3d12::cs::flush();
    // Read all graph inputs once.
    let mut graph_in_bufs: Vec<Option<Vec<u8>>> = vec![None; graph.input_count];
    for &(g_in, to_node, to_slot) in &graph.input_edges {
        if g_in < graph.input_count && graph_in_bufs[g_in].is_none() {
            let min_len = graph
                .nodes
                .get(to_node)
                .and_then(|n| n.input_descs().get(to_slot).copied().flatten())
                .map_or(0, |t| t.total_bytes as usize);
            if let Some(bb) = bt.inputs.get(g_in).and_then(|s| s.first_buffer()) {
                graph_in_bufs[g_in] = read_d3d12_buffer(bb, min_len);
            }
        }
    }

    let mut node_outputs: Vec<Vec<Option<Vec<u8>>>> = graph
        .nodes
        .iter()
        .map(|n| vec![None; n.output_descs().len()])
        .collect();

    for &node_idx in &graph.order {
        let spec = &graph.nodes[node_idx];
        let in_descs = spec.input_descs();
        let out_descs = spec.output_descs();

        let mut in_slices: Vec<Option<&[u8]>> = vec![None; in_descs.len()];
        for &(g_in, to_n, to_slot) in &graph.input_edges {
            if to_n == node_idx && to_slot < in_slices.len() {
                in_slices[to_slot] = graph_in_bufs.get(g_in).and_then(|v| v.as_deref());
            }
        }
        for &(from_n, from_slot, to_n, to_slot) in &graph.inter_edges {
            if to_n == node_idx && to_slot < in_slices.len() {
                in_slices[to_slot] = node_outputs
                    .get(from_n)
                    .and_then(|outs| outs.get(from_slot))
                    .and_then(|v| v.as_deref());
            }
        }

        let mut cur_outs: Vec<Option<Vec<u8>>> = out_descs
            .iter()
            .map(|td_opt| td_opt.map(|td| vec![0u8; td.total_bytes as usize]))
            .collect();
        {
            let mut out_slices: Vec<Option<&mut [u8]>> =
                cur_outs.iter_mut().map(|v| v.as_deref_mut()).collect();
            execute_op(spec, &in_slices, &mut out_slices);
        }
        node_outputs[node_idx] = cur_outs;
    }

    for &(from_n, from_slot, g_out) in &graph.output_edges {
        if g_out >= graph.output_count {
            continue;
        }
        if let (Some(bytes), Some(bb)) = (
            node_outputs
                .get(from_n)
                .and_then(|outs| outs.get(from_slot))
                .and_then(|v| v.as_ref()),
            bt.outputs.get(g_out).and_then(|s| s.first_buffer()),
        ) {
            write_d3d12_buffer(bb, bytes);
        }
    }
}
