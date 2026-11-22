//! Replay: walks a trace in order, keeps an object table and per-command-list state, flags
//! API misuse (unknown or released ids, wrong object kinds, commands on closed lists, draws
//! without a pipeline...), and hands every call to a [`Backend`].
//!
//! [`NullBackend`] only validates. A Metal backend can be plugged in later on top of
//! `d3d12_metal::metal` once that API settles. It receives the same records plus the object
//! table, so it only has to map ids to its own objects.

use std::collections::{BTreeMap, HashMap, HashSet};

use d3d12_metal::trace::{Call, Reader, Record};

use crate::pod::{self, DescriptorHeapDesc, ResourceDesc};

/// Object kinds the trace creates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Device,
    Queue,
    Allocator,
    CommandList,
    DescriptorHeap,
    Heap,
    Resource,
    RootSignature,
    GraphicsPso,
    MeshPso,
    ComputePso,
    StateObject,
    CommandSignature,
    Fence,
    QueryHeap,
    SwapChain,
}

/// Extra per-object data the validation (and backends) use.
#[derive(Debug, Clone, PartialEq)]
pub enum Info {
    None,
    /// `None` when the recorded desc didn't decode.
    Resource { desc: Option<ResourceDesc>, gpu_va: u64 },
    DescriptorHeap { desc: Option<DescriptorHeapDesc>, cpu_start: u64, gpu_start: u64 },
    /// Swap-chain back buffer (`GetSwapChainBuffer`).
    SwapChainBuffer { swapchain: u64, index: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub kind: Kind,
    /// Record index of the creation.
    pub created: u64,
    pub info: Info,
}

/// Command-list recording state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListState {
    pub open: bool,
    pub pso: u64,
    pub graphics_root_signature: u64,
    pub compute_root_signature: u64,
    pub commands: u64,
}

/// Live objects keyed by id (the recorded COM pointer).
#[derive(Debug, Default)]
pub struct Objects {
    live: HashMap<u64, Object>,
    /// Ids whose final release we've seen and that weren't re-created since.
    released: HashSet<u64>,
    lists: HashMap<u64, ListState>,
    pub created_total: BTreeMap<Kind, u64>,
    pub released_total: u64,
}

impl Objects {
    pub fn get(&self, id: u64) -> Option<&Object> {
        self.live.get(&id)
    }
    pub fn list(&self, id: u64) -> Option<&ListState> {
        self.lists.get(&id)
    }
    pub fn live(&self) -> impl Iterator<Item = (&u64, &Object)> {
        self.live.iter()
    }
    pub fn live_by_kind(&self) -> BTreeMap<Kind, u64> {
        let mut m = BTreeMap::new();
        for o in self.live.values() {
            *m.entry(o.kind).or_insert(0) += 1;
        }
        m
    }
}

/// How a backend dealt with a call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handled {
    /// Executed (or deliberately consumed).
    Done,
    /// Not implemented by this backend yet.
    Skipped,
}

/// A replay target. `call` sees each record after the object table has registered any object
/// the record creates and before a `Release` removes one.
pub trait Backend {
    fn name(&self) -> &'static str;
    fn call(&mut self, rec: &Record, objs: &Objects) -> Result<Handled, String>;
    /// Called once after the last record (wait for the GPU, flush captures, ...).
    fn finish(&mut self) -> Result<(), String> {
        Ok(())
    }
}

/// Validation only: accepts every call.
#[derive(Debug, Default)]
pub struct NullBackend;

impl Backend for NullBackend {
    fn name(&self) -> &'static str {
        "null"
    }
    fn call(&mut self, _rec: &Record, _objs: &Objects) -> Result<Handled, String> {
        Ok(Handled::Done)
    }
}

/// Keeps at most `cap` messages but counts all of them.
#[derive(Debug, Clone, Default)]
pub struct Capped {
    pub items: Vec<(u64, String)>,
    pub total: u64,
    pub cap: usize,
}

impl Capped {
    pub fn new(cap: usize) -> Self {
        Self { items: Vec::new(), total: 0, cap }
    }
    pub fn push(&mut self, index: u64, msg: String) {
        self.total += 1;
        if self.items.len() < self.cap {
            self.items.push((index, msg));
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReplayReport {
    pub backend: &'static str,
    pub records: u64,
    pub frames: u64,
    pub done: u64,
    pub skipped: BTreeMap<&'static str, u64>,
    pub failed: Capped,
    pub warnings: Capped,
    pub decode_errors: Capped,
    pub truncated: bool,
    pub created: BTreeMap<Kind, u64>,
    pub released: u64,
    pub live_at_end: BTreeMap<Kind, u64>,
}

impl std::fmt::Display for ReplayReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "replay ({} backend): {} records, {} frames{}", self.backend, self.records, self.frames, if self.truncated { ", trace truncated" } else { "" })?;
        writeln!(f, "  handled {}, skipped {}, failed {}", self.done, self.skipped.values().sum::<u64>(), self.failed.total)?;
        for (op, n) in &self.skipped {
            writeln!(f, "    skipped {op}: {n}")?;
        }
        writeln!(f, "  objects: created {}, released {}", self.created.values().sum::<u64>(), self.released)?;
        for (k, n) in &self.live_at_end {
            writeln!(f, "    live at end {k:?}: {n}")?;
        }
        for (title, c) in [("decode errors", &self.decode_errors), ("failures", &self.failed), ("warnings", &self.warnings)] {
            if c.total > 0 {
                writeln!(f, "  {title}: {}", c.total)?;
                for (i, m) in &c.items {
                    writeln!(f, "    #{i}: {m}")?;
                }
                if c.total as usize > c.items.len() {
                    writeln!(f, "    ... {} more", c.total as usize - c.items.len())?;
                }
            }
        }
        Ok(())
    }
}

/// Replays `data` (a whole trace) into `backend`, stopping after record `until` if given.
pub fn replay(data: &[u8], backend: &mut dyn Backend, until: Option<u64>, max_messages: usize) -> anyhow::Result<ReplayReport> {
    let mut reader = Reader::new(data).map_err(|e| anyhow::anyhow!("{}", e.0))?;
    let mut t = Tracker { objs: Objects::default(), warn: Capped::new(max_messages) };
    let mut rep = ReplayReport { backend: backend.name(), failed: Capped::new(max_messages), decode_errors: Capped::new(max_messages), ..Default::default() };
    let mut index = 0u64;
    for r in reader.by_ref() {
        let rec = match r {
            Ok(rec) => rec,
            Err(e) => {
                rep.decode_errors.push(index, e.0);
                index += 1;
                continue;
            }
        };
        index = rec.index + 1;
        if until.is_some_and(|u| rec.index > u) {
            break;
        }
        rep.records += 1;
        if matches!(rec.call, Call::Present { .. }) {
            rep.frames += 1;
        }
        t.before(&rec);
        match backend.call(&rec, &t.objs) {
            Ok(Handled::Done) => rep.done += 1,
            Ok(Handled::Skipped) => *rep.skipped.entry(rec.call.name()).or_insert(0) += 1,
            Err(e) => rep.failed.push(rec.index, format!("{}: {e}", rec.call.name())),
        }
        t.after(&rec);
    }
    if let Err(e) = backend.finish() {
        rep.failed.push(index, format!("finish: {e}"));
    }
    rep.truncated = reader.truncated();
    rep.warnings = t.warn;
    rep.created = t.objs.created_total.clone();
    rep.released = t.objs.released_total;
    rep.live_at_end = t.objs.live_by_kind();
    Ok(rep)
}

/// Object-table maintenance and validation.
struct Tracker {
    objs: Objects,
    warn: Capped,
}

impl Tracker {
    fn create(&mut self, i: u64, id: u64, kind: Kind, info: Info) {
        if id == 0 {
            self.warn.push(i, format!("{kind:?} created with a null id"));
            return;
        }
        if let Some(old) = self.objs.live.get(&id) {
            // Swap-chain buffers are handed out repeatedly with the same pointer.
            let same_buffer = matches!((&old.info, &info), (Info::SwapChainBuffer { .. }, Info::SwapChainBuffer { .. }));
            if same_buffer {
                return;
            }
            self.warn.push(i, format!("{kind:?} {id:#x} created while {:?} {id:#x} (from #{}) is still live", old.kind, old.created));
        }
        self.objs.released.remove(&id);
        *self.objs.created_total.entry(kind).or_insert(0) += 1;
        if kind == Kind::CommandList {
            // Command lists are created in the recording state.
            self.objs.lists.insert(id, ListState { open: true, ..Default::default() });
        }
        self.objs.live.insert(id, Object { kind, created: i, info });
    }

    /// Checks that `id` names a live object of one of `kinds`. `optional` allows null.
    fn expect(&mut self, i: u64, what: &str, id: u64, kinds: &[Kind], optional: bool) {
        if id == 0 {
            if !optional {
                self.warn.push(i, format!("{what}: null"));
            }
            return;
        }
        match self.objs.live.get(&id) {
            Some(o) if kinds.contains(&o.kind) => {}
            Some(o) => self.warn.push(i, format!("{what}: {id:#x} is a {:?} (from #{}), expected {kinds:?}", o.kind, o.created)),
            None if self.objs.released.contains(&id) => self.warn.push(i, format!("{what}: {id:#x} was released")),
            None => self.warn.push(i, format!("{what}: unknown id {id:#x}")),
        }
    }

    fn resource(&mut self, i: u64, what: &str, id: u64, optional: bool) {
        self.expect(i, what, id, &[Kind::Resource], optional);
    }

    /// A command recorded into `list`: it must exist and be open.
    fn command(&mut self, i: u64, name: &str, list: u64) -> Option<&mut ListState> {
        self.expect(i, name, list, &[Kind::CommandList], false);
        let st = self.objs.lists.get_mut(&list)?;
        st.commands += 1;
        if !st.open {
            self.warn.push(i, format!("{name}: command list {list:#x} is closed"));
        }
        self.objs.lists.get_mut(&list)
    }

    /// `want`: the pipeline kind the call needs (compute PSOs and raytracing state objects use the compute root signature).
    fn draw(&mut self, i: u64, name: &str, list: u64, want: Kind) {
        let Some(st) = self.command(i, name, list) else { return };
        let compute = matches!(want, Kind::ComputePso | Kind::StateObject);
        let (pso, rs) = (st.pso, if compute { st.compute_root_signature } else { st.graphics_root_signature });
        match self.objs.live.get(&pso) {
            Some(o) if o.kind == want => {}
            Some(o) => self.warn.push(i, format!("{name}: bound pipeline {pso:#x} is a {:?}", o.kind)),
            None if pso == 0 => self.warn.push(i, format!("{name}: no pipeline state set")),
            None => self.warn.push(i, format!("{name}: bound pipeline {pso:#x} is gone")),
        }
        if rs == 0 {
            self.warn.push(i, format!("{name}: no {} root signature set", if compute { "compute" } else { "graphics" }));
        }
    }

    fn before(&mut self, rec: &Record) {
        use Kind as K;
        let i = rec.index;
        match &rec.call {
            Call::CreateDevice { device, .. } => self.create(i, *device, K::Device, Info::None),
            Call::CreateCommandQueue { device, queue, .. } => {
                self.expect(i, "CreateCommandQueue", *device, &[K::Device], false);
                self.create(i, *queue, K::Queue, Info::None)
            }
            Call::CreateCommandAllocator { device, allocator, .. } => {
                self.expect(i, "CreateCommandAllocator", *device, &[K::Device], false);
                self.create(i, *allocator, K::Allocator, Info::None)
            }
            Call::CreateCommandList { device, list, allocator, initial_pso, .. } => {
                self.expect(i, "CreateCommandList", *device, &[K::Device], false);
                self.expect(i, "CreateCommandList allocator", *allocator, &[K::Allocator], false);
                self.expect(i, "CreateCommandList initial PSO", *initial_pso, &[K::GraphicsPso, K::ComputePso, K::MeshPso], true);
                self.create(i, *list, K::CommandList, Info::None);
                if let Some(st) = self.objs.lists.get_mut(list) {
                    st.pso = *initial_pso;
                }
            }
            Call::CreateDescriptorHeap { device, heap, desc, cpu_start, gpu_start } => {
                self.expect(i, "CreateDescriptorHeap", *device, &[K::Device], false);
                let d = pod::read::<DescriptorHeapDesc>(desc).map_err(|e| self.warn.push(i, format!("CreateDescriptorHeap: {e}"))).ok();
                self.create(i, *heap, K::DescriptorHeap, Info::DescriptorHeap { desc: d, cpu_start: *cpu_start, gpu_start: *gpu_start })
            }
            Call::CreateHeap { device, heap, .. } => {
                self.expect(i, "CreateHeap", *device, &[K::Device], false);
                self.create(i, *heap, K::Heap, Info::None)
            }
            Call::CreateCommittedResource { device, resource, desc, gpu_va, .. }
            | Call::CreateReservedResource { device, resource, desc, gpu_va, .. } => {
                self.expect(i, rec.call.name(), *device, &[K::Device], false);
                let d = pod::read_prefix::<ResourceDesc>(desc).map_err(|e| self.warn.push(i, format!("{}: {e}", rec.call.name()))).ok();
                self.create(i, *resource, K::Resource, Info::Resource { desc: d, gpu_va: *gpu_va })
            }
            Call::CreatePlacedResource { device, resource, heap, desc, gpu_va, .. } => {
                self.expect(i, "CreatePlacedResource", *device, &[K::Device], false);
                self.expect(i, "CreatePlacedResource heap", *heap, &[K::Heap], false);
                let d = pod::read_prefix::<ResourceDesc>(desc).map_err(|e| self.warn.push(i, format!("CreatePlacedResource: {e}"))).ok();
                self.create(i, *resource, K::Resource, Info::Resource { desc: d, gpu_va: *gpu_va })
            }
            Call::CreateRootSignature { device, root_signature, .. } => {
                self.expect(i, "CreateRootSignature", *device, &[K::Device], false);
                self.create(i, *root_signature, K::RootSignature, Info::None)
            }
            Call::CreateGraphicsPipelineState { device, pso, desc } => {
                self.expect(i, "CreateGraphicsPipelineState", *device, &[K::Device], false);
                // Null = root signature embedded in the shaders.
                self.expect(i, "CreateGraphicsPipelineState root signature", desc.root_signature, &[K::RootSignature], true);
                self.create(i, *pso, K::GraphicsPso, Info::None)
            }
            Call::CreateMeshPipelineState { device, pso, desc, .. } => {
                self.expect(i, "CreateMeshPipelineState", *device, &[K::Device], false);
                self.expect(i, "CreateMeshPipelineState root signature", desc.root_signature, &[K::RootSignature], true);
                self.create(i, *pso, K::MeshPso, Info::None)
            }
            Call::CreateComputePipelineState { device, pso, root_signature, .. } => {
                self.expect(i, "CreateComputePipelineState", *device, &[K::Device], false);
                self.expect(i, "CreateComputePipelineState root signature", *root_signature, &[K::RootSignature], true);
                self.create(i, *pso, K::ComputePso, Info::None)
            }
            Call::CreateStateObject { device, state_object, .. } => {
                self.expect(i, "CreateStateObject", *device, &[K::Device], false);
                self.create(i, *state_object, K::StateObject, Info::None)
            }
            Call::AddToStateObject { device, state_object, base, .. } => {
                self.expect(i, "AddToStateObject", *device, &[K::Device], false);
                self.expect(i, "AddToStateObject base", *base, &[K::StateObject], false);
                self.create(i, *state_object, K::StateObject, Info::None)
            }
            Call::CreateCommandSignature { device, signature, root_signature, .. } => {
                self.expect(i, "CreateCommandSignature", *device, &[K::Device], false);
                self.expect(i, "CreateCommandSignature root signature", *root_signature, &[K::RootSignature], true);
                self.create(i, *signature, K::CommandSignature, Info::None)
            }
            Call::CreateFence { device, fence, .. } => {
                self.expect(i, "CreateFence", *device, &[K::Device], false);
                self.create(i, *fence, K::Fence, Info::None)
            }
            Call::CreateQueryHeap { device, heap, .. } => {
                self.expect(i, "CreateQueryHeap", *device, &[K::Device], false);
                self.create(i, *heap, K::QueryHeap, Info::None)
            }
            Call::Release { object } => {
                if !self.objs.live.contains_key(object) {
                    let why = if self.objs.released.contains(object) { "released twice" } else { "unknown id" };
                    self.warn.push(i, format!("Release {object:#x}: {why}"));
                }
            }
            Call::CreateShaderResourceView { resource, .. } | Call::CreateRenderTargetView { resource, .. } | Call::CreateDepthStencilView { resource, .. } => {
                self.resource(i, rec.call.name(), *resource, true)
            }
            Call::CreateUnorderedAccessView { resource, counter, .. } => {
                self.resource(i, "CreateUnorderedAccessView", *resource, true);
                self.resource(i, "CreateUnorderedAccessView counter", *counter, true);
            }
            Call::CreateConstantBufferView { .. } | Call::CreateSampler { .. } | Call::CopyDescriptorsSimple { .. } | Call::CopyDescriptors { .. } => {}
            Call::WriteResource { resource, .. } => self.resource(i, "WriteResource", *resource, false),
            Call::Reset { list, allocator, pso } => {
                self.expect(i, "Reset", *list, &[K::CommandList], false);
                self.expect(i, "Reset allocator", *allocator, &[K::Allocator], false);
                self.expect(i, "Reset PSO", *pso, &[K::GraphicsPso, K::ComputePso, K::MeshPso], true);
                if let Some(st) = self.objs.lists.get_mut(list) {
                    if st.open {
                        self.warn.push(i, format!("Reset: command list {list:#x} is still open"));
                    }
                    *st = ListState { open: true, pso: *pso, ..Default::default() };
                }
            }
            Call::Close { list } => {
                self.expect(i, "Close", *list, &[K::CommandList], false);
                if let Some(st) = self.objs.lists.get_mut(list) {
                    if !st.open {
                        self.warn.push(i, format!("Close: command list {list:#x} is already closed"));
                    }
                    st.open = false;
                }
            }
            Call::ClearState { list, pso } => {
                self.expect(i, "ClearState PSO", *pso, &[K::GraphicsPso, K::ComputePso, K::MeshPso], true);
                if let Some(st) = self.command(i, "ClearState", *list) {
                    st.pso = *pso;
                    st.graphics_root_signature = 0;
                    st.compute_root_signature = 0;
                }
            }
            Call::SetPredication { list, buffer, aligned_offset, operation } => {
                self.command(i, "SetPredication", *list);
                self.resource(i, "SetPredication buffer", *buffer, true);
                if *buffer != 0 && !aligned_offset.is_multiple_of(8) {
                    self.warn.push(i, format!("SetPredication: offset {aligned_offset:#x} is not 8-byte aligned"));
                }
                if *operation > 1 {
                    self.warn.push(i, format!("SetPredication: unknown operation {operation}"));
                }
            }
            Call::SetPipelineState { list, pso } => {
                self.expect(i, "SetPipelineState PSO", *pso, &[K::GraphicsPso, K::ComputePso, K::MeshPso], false);
                if let Some(st) = self.command(i, "SetPipelineState", *list) {
                    st.pso = *pso;
                }
            }
            Call::SetPipelineState1 { list, state_object } => {
                self.expect(i, "SetPipelineState1 state_object", *state_object, &[K::StateObject], false);
                if let Some(st) = self.command(i, "SetPipelineState1", *list) {
                    st.pso = *state_object;
                }
            }
            Call::SetGraphicsRootSignature { list, root_signature } | Call::SetComputeRootSignature { list, root_signature } => {
                let compute = matches!(rec.call, Call::SetComputeRootSignature { .. });
                self.expect(i, rec.call.name(), *root_signature, &[K::RootSignature], false);
                if let Some(st) = self.command(i, rec.call.name(), *list) {
                    if compute {
                        st.compute_root_signature = *root_signature;
                    } else {
                        st.graphics_root_signature = *root_signature;
                    }
                }
            }
            Call::SetDescriptorHeaps { list, heaps } => {
                self.command(i, "SetDescriptorHeaps", *list);
                for h in heaps.iter() {
                    self.expect(i, "SetDescriptorHeaps heap", h, &[K::DescriptorHeap], false);
                }
            }
            Call::DrawInstanced { list, .. } | Call::DrawIndexedInstanced { list, .. } => self.draw(i, rec.call.name(), *list, K::GraphicsPso),
            Call::Dispatch { list, .. } => self.draw(i, "Dispatch", *list, K::ComputePso),
            Call::DispatchMesh { list, .. } => self.draw(i, "DispatchMesh", *list, K::MeshPso),
            Call::DispatchRays { list, desc, .. } => {
                self.draw(i, "DispatchRays", *list, K::StateObject);
                if desc.len() != std::mem::size_of::<d3d12_metal::rt::pipeline::DispatchRaysDesc>() {
                    self.warn.push(i, format!("DispatchRays: {}-byte desc", desc.len()));
                }
            }
            Call::ExecuteIndirect { list, signature, args, count, .. } => {
                self.command(i, "ExecuteIndirect", *list);
                self.expect(i, "ExecuteIndirect signature", *signature, &[K::CommandSignature], false);
                self.resource(i, "ExecuteIndirect arguments", *args, false);
                self.resource(i, "ExecuteIndirect count buffer", *count, true);
            }
            Call::ResourceBarrier { list, .. }
            | Call::SetRootDescriptorTable { list, .. }
            | Call::SetRoot32BitConstants { list, .. }
            | Call::SetRootView { list, .. }
            | Call::IASetPrimitiveTopology { list, .. }
            | Call::IASetVertexBuffers { list, .. }
            | Call::IASetIndexBuffer { list, .. }
            | Call::RSSetViewports { list, .. }
            | Call::RSSetScissorRects { list, .. }
            | Call::OMSetRenderTargets { list, .. }
            | Call::OMSetBlendFactor { list, .. }
            | Call::OMSetStencilRef { list, .. }
            | Call::OMSetFrontAndBackStencilRef { list, .. }
            | Call::OMSetDepthBounds { list, .. }
            | Call::Marker { list, .. } => {
                self.command(i, rec.call.name(), *list);
            }
            Call::ClearRenderTargetView { list, rects, .. } | Call::ClearDepthStencilView { list, rects, .. } => {
                let n = rec.call.name();
                self.command(i, n, *list);
                if !rects.len().is_multiple_of(std::mem::size_of::<d3d12_metal::types::Rect>()) {
                    self.warn.push(i, format!("{n}: {}-byte rects blob", rects.len()));
                }
            }
            Call::BuildRaytracingAccelerationStructure { list, dest, inputs_type, num_descs, geometries, postbuild, .. } => {
                self.command(i, "BuildRaytracingAccelerationStructure", *list);
                if *dest == 0 {
                    self.warn.push(i, "BuildRaytracingAccelerationStructure: null dest".into());
                }
                if *inputs_type == d3d12_metal::rt::d3d::TYPE_BOTTOM_LEVEL && geometries.len() != *num_descs as usize * std::mem::size_of::<d3d12_metal::rt::d3d::GeometryDesc>() {
                    self.warn.push(i, format!("BuildRaytracingAccelerationStructure: {} geometry bytes for {} descs", geometries.len(), num_descs));
                }
                if !postbuild.len().is_multiple_of(std::mem::size_of::<d3d12_metal::rt::d3d::PostbuildInfoDesc>()) {
                    self.warn.push(i, format!("BuildRaytracingAccelerationStructure: {}-byte postbuild blob", postbuild.len()));
                }
            }
            Call::EmitRaytracingAccelerationStructurePostbuildInfo { list, dest_buffer, sources, .. } => {
                self.command(i, "EmitRaytracingAccelerationStructurePostbuildInfo", *list);
                if *dest_buffer == 0 && !sources.is_empty() {
                    self.warn.push(i, "EmitRaytracingAccelerationStructurePostbuildInfo: null dest_buffer".into());
                }
            }
            Call::CopyRaytracingAccelerationStructure { list, dest, source, .. } => {
                self.command(i, "CopyRaytracingAccelerationStructure", *list);
                if *dest == 0 || *source == 0 {
                    self.warn.push(i, "CopyRaytracingAccelerationStructure: null dest or source".into());
                }
            }
            Call::ClearUnorderedAccessViewUint { list, resource, .. } | Call::ClearUnorderedAccessViewFloat { list, resource, .. } | Call::DiscardResource { list, resource, .. } => {
                self.command(i, rec.call.name(), *list);
                self.resource(i, rec.call.name(), *resource, false);
            }
            Call::CopyBufferRegion { list, dst, src, .. } | Call::CopyResource { list, dst, src } | Call::ResolveSubresource { list, dst, src, .. } => {
                let n = rec.call.name();
                self.command(i, n, *list);
                self.resource(i, &format!("{n} dst"), *dst, false);
                self.resource(i, &format!("{n} src"), *src, false);
            }
            Call::CopyTextureRegion { list, dst, src, .. } => {
                self.command(i, "CopyTextureRegion", *list);
                for (what, loc) in [("CopyTextureRegion dst", dst), ("CopyTextureRegion src", src)] {
                    match loc.get(..8) {
                        Some(b) => self.resource(i, what, u64::from_le_bytes(b.try_into().unwrap()), false),
                        None => self.warn.push(i, format!("{what}: {}-byte copy location", loc.len())),
                    }
                }
            }
            Call::BeginQuery { list, heap, .. } | Call::EndQuery { list, heap, .. } => {
                self.command(i, rec.call.name(), *list);
                self.expect(i, rec.call.name(), *heap, &[K::QueryHeap], false);
            }
            Call::ResolveQueryData { list, heap, dst, .. } => {
                self.command(i, "ResolveQueryData", *list);
                self.expect(i, "ResolveQueryData heap", *heap, &[K::QueryHeap], false);
                self.resource(i, "ResolveQueryData dst", *dst, false);
            }
            Call::ExecuteCommandLists { queue, lists } => {
                self.expect(i, "ExecuteCommandLists", *queue, &[K::Queue], false);
                for l in lists.iter() {
                    self.expect(i, "ExecuteCommandLists list", l, &[K::CommandList], false);
                    if self.objs.lists.get(&l).is_some_and(|s| s.open) {
                        self.warn.push(i, format!("ExecuteCommandLists: command list {l:#x} was not closed"));
                    }
                }
            }
            Call::QueueSignal { queue, fence, .. } | Call::QueueWait { queue, fence, .. } => {
                self.expect(i, rec.call.name(), *queue, &[K::Queue], false);
                self.expect(i, rec.call.name(), *fence, &[K::Fence], false);
            }
            Call::FenceSignal { fence, .. } => self.expect(i, "FenceSignal", *fence, &[K::Fence], false),
            Call::CreateSwapChain { queue, swapchain, .. } => {
                self.expect(i, "CreateSwapChain", *queue, &[K::Queue], false);
                self.create(i, *swapchain, K::SwapChain, Info::None)
            }
            Call::GetSwapChainBuffer { swapchain, index, resource } => {
                self.expect(i, "GetSwapChainBuffer", *swapchain, &[K::SwapChain], false);
                self.create(i, *resource, K::Resource, Info::SwapChainBuffer { swapchain: *swapchain, index: *index })
            }
            Call::ResizeBuffers { swapchain, .. } | Call::Present { swapchain, .. } => self.expect(i, rec.call.name(), *swapchain, &[K::SwapChain], false),
            Call::FrameEnd { .. } => {}
        }
    }

    fn after(&mut self, rec: &Record) {
        if let Call::Release { object } = rec.call {
            if self.objs.live.remove(&object).is_some() {
                self.objs.released.insert(object);
                self.objs.lists.remove(&object);
                self.objs.released_total += 1;
            }
        }
    }
}
