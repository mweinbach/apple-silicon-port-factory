//! Real Metal 4 execution backend for `d3d12-replay`, backed by `d3d12-metal`.
//!
//! Maps recorded D3D12 object IDs, GPU virtual addresses, and CPU/GPU descriptor
//! handles onto live `d3d12_metal::metal` objects and executes every recorded
//! command on Apple Silicon Metal 4.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::mesh::{MeshDesc, MeshPipeline};
use d3d12_metal::metal::clear::ClearValue;
use d3d12_metal::metal::cmdlist::{Barrier, CommandAllocator, CommandList, CopyBox, CopyLoc, TargetView};
use d3d12_metal::metal::descriptor::{self, heap_kind, DescriptorHeap};
use d3d12_metal::metal::fence::Fence;
use d3d12_metal::metal::heap::Heap;
use d3d12_metal::metal::indirect::{arg as indirect_arg, ArgumentDesc, IndirectLayout};
use d3d12_metal::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
use d3d12_metal::metal::query::{self, QueryHeap};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource;
use d3d12_metal::metal::swapchain::Swapchain;
use d3d12_metal::metal::views::{self, SamplerDesc};
use d3d12_metal::state::{
    self, BlendDesc, DepthStencilDesc, DepthStencilDesc1, DepthStencilDesc2, RasterizerDesc,
    RasterizerDesc1, RasterizerDesc2,
};
use d3d12_metal::trace::{Call, GraphicsPipeline as TraceGfxPso, Record};
use d3d12_metal::types::{
    CommandQueueDesc, DescriptorHeapDesc, HeapDesc, HeapProperties, Rect, ResourceBarrier,
    ResourceDesc, Viewport,
};
use dxil_metal::root_signature::RootSignature;
use dxil_metal::InputElement;
use objc2_metal::{MTLBuffer, MTLTextureUsage};

use crate::golden::{self, FrameImage};
use crate::pod::{self, IndexBufferView, VertexBufferView};
use crate::replay::{Backend, Handled, Objects};

/// Maximum entries allowed by Metal 4's `MTL4CounterHeapDescriptor.count` (4096).
pub(crate) const DEFAULT_MAX_TIMESTAMPS: u32 = 4096;

/// Live descriptor heap paired with its recorded address space for safe handle remapping.
struct LiveDescHeap {
    heap: Arc<DescriptorHeap>,
    rec_cpu_start: u64,
    rec_gpu_start: u64,
    rec_stride: u64,
    /// Owned `TargetView` slots for RTV/DSV heaps (indexed by descriptor slot).
    targets: Vec<TargetView>,
}

impl LiveDescHeap {
    fn cpu_index(&self, rec_cpu: u64) -> Option<usize> {
        if self.rec_cpu_start == 0 || rec_cpu < self.rec_cpu_start {
            return None;
        }
        let delta = rec_cpu - self.rec_cpu_start;
        if self.rec_stride == 0 || !delta.is_multiple_of(self.rec_stride) {
            return None;
        }
        let idx = (delta / self.rec_stride) as usize;
        (idx < self.heap.count as usize).then_some(idx)
    }

    fn gpu_index(&self, rec_gpu: u64) -> Option<u64> {
        if !self.heap.shader_visible || self.rec_gpu_start == 0 || rec_gpu < self.rec_gpu_start {
            return None;
        }
        let delta = rec_gpu - self.rec_gpu_start;
        if self.rec_stride == 0 || !delta.is_multiple_of(self.rec_stride) {
            return None;
        }
        let idx = delta / self.rec_stride;
        (idx < self.heap.count as u64).then_some(idx)
    }

    fn live_cpu_addr(&self, idx: usize) -> usize {
        self.heap.cpu_base + idx * self.heap.descriptor_size() as usize
    }

    fn live_gpu_addr(&self, idx: u64) -> u64 {
        self.heap.gpu_base + idx * self.heap.descriptor_size() as u64
    }

    /// Write a 24-byte `IRDescriptorTableEntry` into slot `idx` with bounds checks.
    fn write_entry(&mut self, idx: usize, entry: &[u8; 24]) -> Result<(), String> {
        if idx >= self.heap.count as usize {
            return Err(format!("descriptor slot {idx} out of range ({})", self.heap.count));
        }
        if self.heap.descriptor_size() != 24 {
            return Err("attempted to write 24-byte descriptor into RTV/DSV heap".into());
        }
        let dst = self.live_cpu_addr(idx) as *mut u8;
        unsafe { std::ptr::copy_nonoverlapping(entry.as_ptr(), dst, 24) };
        Ok(())
    }

    fn read_entry(&self, idx: usize) -> Result<[u8; 24], String> {
        if idx >= self.heap.count as usize {
            return Err(format!("descriptor slot {idx} out of range ({})", self.heap.count));
        }
        if self.heap.descriptor_size() != 24 {
            return Err("attempted to read 24-byte descriptor from RTV/DSV heap".into());
        }
        let mut out = [0u8; 24];
        let src = self.live_cpu_addr(idx) as *const u8;
        unsafe { std::ptr::copy_nonoverlapping(src, out.as_mut_ptr(), 24) };
        Ok(out)
    }
}

#[derive(Clone)]
enum LivePso {
    Graphics(Arc<GraphicsPipeline>),
    Mesh(Arc<MeshPipeline>),
    Compute(Arc<ComputePipeline>),
}

struct PendingIndirectReloc {
    sig: LiveCmdSig,
    max_count: u32,
    src_res_id: u64,
    src_offset: u64,
    args_len: usize,
    scratch: Arc<Resource>,
}

struct LiveCommandList {
    cl: CommandList,
    alloc_id: u64,
    initial_pso: u64,
    open_ts_slot: Option<u32>,
    /// Closed `(start_slot, end_slot)` timestamp pair awaiting actual submission via
    /// `ExecuteCommandLists`; discarded recordings (`Reset` without submit) are never summed.
    closed_ts_pair: Option<(u32, u32)>,
    /// True once this closed command list has been submitted via `ExecuteCommandLists`
    /// since its last `begin`/`Reset`, so timed benchmarks reject re-executing the same
    /// baked timestamp writes into identical counter-heap slots.
    submitted_since_reset: bool,
    /// Indirect argument buffers with embedded GPU VAs that must be refreshed from their
    /// source resource right before `ExecuteCommandLists` so submission-time `WriteResource`
    /// uploads (`tr::sync_uploads`) are reflected in the relocated scratch buffer.
    pending_indirect_relocs: Vec<PendingIndirectReloc>,
    /// Strong references to every `Resource`, `DescriptorHeap`, and `QueryHeap` referenced by
    /// commands recorded on this list (including non-owning `TargetView` and barrier raw pointers,
    /// relocated indirect scratch buffers, and bound heaps), kept alive until `Reset` and moved to
    /// `retired` across `Reset` so `Release` + queue drain can never cause a host or GPU UAF.
    retained_resources: Vec<Arc<Resource>>,
    retained_desc_heaps: Vec<Arc<DescriptorHeap>>,
    retained_query_heaps: Vec<Arc<QueryHeap>>,
}

#[derive(Clone)]
struct LiveCmdSig {
    layout: IndirectLayout,
    /// Byte offsets within each indirect command stride that hold a 64-bit D3D12 GPU virtual
    /// address (VB/IB views and root CBV/SRV/UAV arguments) requiring capture->live remapping.
    va_offsets: Vec<u32>,
}

/// Hardware GPU timestamp state (`MTL4CounterHeap`) for `--bench` `gpu_busy` timing.
struct GpuTimer {
    heap: QueryHeap,
    readback: Resource,
    alloc: CommandAllocator,
    cl: CommandList,
    capacity: u32,
    cursor: u32,
    overflowed: bool,
    pairs: Vec<(u32, u32)>,
}

impl GpuTimer {
    fn with_capacity(capacity: u32) -> Result<Self, String> {
        let cap = capacity.clamp(1, DEFAULT_MAX_TIMESTAMPS);
        let heap = QueryHeap::new(query::heap_type::TIMESTAMP, cap)?;
        let rb_desc = ResourceDesc {
            Dimension: d3d12_metal::types::dim::BUFFER,
            Width: cap as u64 * 8,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: d3d12_metal::types::heap_type::READBACK, ..Default::default() };
        let readback = Resource::new_committed(&rb_desc, &hp, MTLTextureUsage::empty(), "replay ts readback")
            .map_err(|e| format!("timestamp readback buffer: {e:?}"))?;
        let alloc = CommandAllocator::new(ListType::Direct).ok_or("timestamp resolve allocator")?;
        let cl = CommandList::new(ListType::Direct).ok_or("timestamp resolve command list")?;
        Ok(Self {
            heap,
            readback,
            alloc,
            cl,
            capacity: cap,
            cursor: 0,
            overflowed: false,
            pairs: Vec::new(),
        })
    }

    fn reset(&mut self) {
        self.cursor = 0;
        self.overflowed = false;
        self.pairs.clear();
    }

    fn remaining(&self) -> u32 {
        self.capacity.saturating_sub(self.cursor)
    }

    fn alloc_slot(&mut self) -> Result<u32, String> {
        if self.cursor < self.capacity {
            let s = self.cursor;
            self.cursor += 1;
            Ok(s)
        } else {
            self.overflowed = true;
            Err(format!(
                "benchmark timestamp heap capacity exhausted ({} slots = {} timed command lists)",
                self.capacity,
                self.capacity / 2
            ))
        }
    }

    fn resolve_all_iterations_ms(&mut self, queue: &Queue, ranges: &[(usize, usize)]) -> Result<Vec<f64>, String> {
        if self.overflowed {
            return Err(format!(
                "benchmark timestamp heap overflowed ({} slots); timing results would be incomplete",
                self.capacity
            ));
        }
        if self.cursor == 0 || ranges.is_empty() {
            return Ok(vec![0.0; ranges.len()]);
        }
        let count = self.cursor;
        self.alloc.reset();
        self.cl.begin(&self.alloc)?;
        self.cl.resolve_timestamps(&self.heap, 0, count, &self.readback, 0)?;
        self.cl.close()?;
        queue.execute_command_lists(&mut [&mut self.cl]);
        queue.wait_idle();

        let buf = self.readback.buffer().ok_or("timestamp readback missing buffer")?;
        let ticks = unsafe { std::slice::from_raw_parts(buf.contents().as_ptr() as *const u64, count as usize) };
        let freq = query::timestamp_frequency() as f64;
        let mut out = Vec::with_capacity(ranges.len());
        for &(lo, hi) in ranges {
            let mut total_ticks = 0u64;
            for &(s, e) in &self.pairs[lo..hi] {
                let t0 = ticks[s as usize];
                let t1 = ticks[e as usize];
                if t1 >= t0 {
                    total_ticks = total_ticks.saturating_add(t1 - t0);
                }
            }
            out.push((total_ticks as f64 / freq) * 1000.0);
        }
        Ok(out)
    }
}

/// Real Metal 4 execution backend implementing [`Backend`].
pub struct MetalBackend {
    queues: HashMap<u64, Arc<Queue>>,
    primary_queue: Option<u64>,
    allocators: HashMap<u64, CommandAllocator>,
    lists: HashMap<u64, LiveCommandList>,
    desc_heaps: HashMap<u64, LiveDescHeap>,
    heaps: HashMap<u64, Arc<Heap>>,
    resources: HashMap<u64, Arc<Resource>>,
    /// Recorded buffer GPU VA -> `(size_bytes, live_gpu_va)`.
    va_map: BTreeMap<u64, (u64, u64)>,
    root_sigs: HashMap<u64, Arc<RootSignature>>,
    psos: HashMap<u64, LivePso>,
    cmd_sigs: HashMap<u64, LiveCmdSig>,
    fences: HashMap<u64, Arc<Fence>>,
    query_heaps: HashMap<u64, Arc<QueryHeap>>,
    swapchains: HashMap<u64, Swapchain>,
    /// Swapchain ID -> owning command queue ID (for tracking pending GPU work on `Present`).
    swapchain_queues: HashMap<u64, u64>,
    /// Objects released during replay, kept alive until `wait_all_idle()` so GPU command buffers
    /// and non-owning `TargetView` pointers never dangle.
    retired: Vec<Box<dyn Send>>,
    /// Most recently presented swapchain back buffer (primary target for `--golden`).
    last_presented: Option<Arc<Resource>>,
    /// Most recently bound or cleared RTV resource (fallback target for `--golden` without swapchain).
    last_rtv: Option<Arc<Resource>>,
    /// Optional GPU timestamp timer used by `--bench`.
    gpu_timer: Option<GpuTimer>,
    timing_armed: bool,
    /// When true, `Present` forces `sync_interval = 0` so headless replay never sleeps 16.6 ms.
    disable_vsync: bool,
    /// Queues that have submitted GPU work (`ExecuteCommandLists`/`Present`) since their last drain.
    queue_busy: HashSet<u64>,
    /// Pending cross-queue `(fence_id, value)` waits enqueued on each queue since its last drain.
    queue_waits: HashMap<u64, Vec<(u64, u64)>>,
    /// Signals recorded on each fence: `(value, producer_queue_id)` where `None` is CPU/initial.
    fence_signals: HashMap<u64, Vec<(u64, Option<u64>)>>,
}

impl MetalBackend {
    pub fn new() -> Result<Self, String> {
        if d3d12_metal::metal::device().is_none() {
            return Err("no Metal 4 device available".into());
        }
        Ok(Self {
            queues: HashMap::new(),
            primary_queue: None,
            allocators: HashMap::new(),
            lists: HashMap::new(),
            desc_heaps: HashMap::new(),
            heaps: HashMap::new(),
            resources: HashMap::new(),
            va_map: BTreeMap::new(),
            root_sigs: HashMap::new(),
            psos: HashMap::new(),
            cmd_sigs: HashMap::new(),
            fences: HashMap::new(),
            query_heaps: HashMap::new(),
            swapchains: HashMap::new(),
            swapchain_queues: HashMap::new(),
            retired: Vec::new(),
            last_presented: None,
            last_rtv: None,
            gpu_timer: None,
            timing_armed: false,
            disable_vsync: true,
            queue_busy: HashSet::new(),
            queue_waits: HashMap::new(),
            fence_signals: HashMap::new(),
        })
    }

    pub fn enable_gpu_timing(&mut self, enable: bool) -> Result<(), String> {
        self.enable_gpu_timing_with_capacity(enable, DEFAULT_MAX_TIMESTAMPS)
    }

    pub(crate) fn enable_gpu_timing_with_capacity(&mut self, enable: bool, capacity: u32) -> Result<(), String> {
        if enable {
            let cap = capacity.clamp(1, DEFAULT_MAX_TIMESTAMPS);
            let need_new = self.gpu_timer.as_ref().is_none_or(|t| t.capacity != cap);
            if need_new {
                self.gpu_timer = Some(GpuTimer::with_capacity(cap)?);
            }
            self.timing_armed = true;
        } else {
            self.timing_armed = false;
        }
        Ok(())
    }

    pub fn wait_all_idle(&mut self) {
        for q in self.queues.values() {
            q.wait_idle();
        }
        self.queue_busy.clear();
        self.queue_waits.clear();
        self.retired.clear();
    }

    /// Return IDs of any busy queues currently blocked on fence waits that have not been
    /// transitively satisfied by prior signals (draining them would deadlock).
    pub fn blocked_queues(&self) -> Vec<u64> {
        let mut blocked: HashSet<u64> = HashSet::new();
        loop {
            let mut changed = false;
            for (&qid, waits) in &self.queue_waits {
                if blocked.contains(&qid) {
                    continue;
                }
                let is_blocked = waits.iter().any(|&(fid, val)| {
                    let satisfied_by_unblocked = self.fence_signals.get(&fid).is_some_and(|sigs| {
                        sigs.iter()
                            .any(|&(sv, prod)| sv >= val && prod.is_none_or(|pq| !blocked.contains(&pq)))
                    });
                    !satisfied_by_unblocked
                });
                if is_blocked {
                    blocked.insert(qid);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut out: Vec<u64> = self
            .queue_busy
            .iter()
            .copied()
            .filter(|qid| blocked.contains(qid))
            .collect();
        out.sort_unstable();
        out
    }

    /// Drain every busy queue whose enqueued fence waits are already transitively satisfied
    /// by prior signals, avoiding deadlocks on queues waiting for a producer that appears
    /// later in the trace.
    fn drain_unblocked_queues(&mut self) {
        if self.queue_busy.is_empty() {
            return;
        }
        let blocked: HashSet<u64> = self.blocked_queues().into_iter().collect();
        let drainable: Vec<u64> = self
            .queue_busy
            .iter()
            .copied()
            .filter(|qid| !blocked.contains(qid))
            .collect();
        for qid in drainable {
            if let Some(q) = self.queues.get(&qid) {
                q.wait_idle();
            }
            self.queue_busy.remove(&qid);
            self.queue_waits.remove(&qid);
        }
        if self.queue_busy.is_empty() {
            self.retired.clear();
        }
    }

    /// Primary Direct queue (or any queue if no Direct queue was created).
    pub fn primary_queue(&self) -> Option<Arc<Queue>> {
        self.primary_queue
            .and_then(|id| self.queues.get(&id).cloned())
            .or_else(|| self.queues.values().next().cloned())
    }

    /// Reset GPU timing counters before the measured phase of a benchmark.
    pub fn reset_gpu_timer(&mut self) {
        if let Some(t) = self.gpu_timer.as_mut() {
            t.reset();
        }
    }

    /// Number of completed command-buffer timestamp pairs recorded so far.
    pub fn gpu_timer_pair_count(&self) -> usize {
        self.gpu_timer.as_ref().map_or(0, |t| t.pairs.len())
    }

    /// Number of timestamp slots currently consumed in the active counter heap.
    pub fn gpu_timer_used_slots(&self) -> u32 {
        self.gpu_timer.as_ref().map_or(0, |t| t.cursor)
    }

    /// Number of unused timestamp slots remaining in the active counter heap.
    pub fn gpu_timer_remaining_slots(&self) -> u32 {
        self.gpu_timer.as_ref().map_or(0, |t| t.remaining())
    }

    /// Snapshot each live swapchain's current back-buffer index so repeated benchmark
    /// iterations always target the exact buffer recorded in the looped frame slice.
    pub fn snapshot_swapchain_indices(&self) -> Vec<(u64, usize)> {
        self.swapchains.iter().map(|(&id, sc)| (id, sc.current)).collect()
    }

    /// Snapshot each live fence's completed timeline value so repeated benchmark iterations
    /// restore cross-queue fence timelines instead of reusing already-completed values.
    pub fn snapshot_fence_values(&self) -> Vec<(u64, u64)> {
        self.fences.iter().map(|(&id, f)| (id, f.completed())).collect()
    }

    /// Wait idle, reset allocators, restore swapchain buffer indices and fence timelines,
    /// and optionally reopen lists that were open at the start of a repeated benchmark frame.
    pub fn prepare_bench_iteration(
        &mut self,
        reopen_lists: &[u64],
        swapchain_indices: &[(u64, usize)],
        fence_values: &[(u64, u64)],
    ) -> Result<(), String> {
        self.wait_all_idle();
        for &(id, idx) in swapchain_indices {
            if let Some(sc) = self.swapchains.get_mut(&id) {
                if idx < sc.buffers.len() {
                    sc.current = idx;
                }
            }
        }
        for &(id, val) in fence_values {
            if self.fences.contains_key(&id) {
                let fresh = Fence::new(val).ok_or_else(|| format!("recreate fence {id:#x} at value {val}"))?;
                self.fences.insert(id, Arc::new(fresh));
                self.fence_signals.insert(id, vec![(val, None)]);
            }
        }
        let open_allocs: HashSet<u64> = self
            .lists
            .values()
            .filter(|l| l.cl.is_open())
            .map(|l| l.alloc_id)
            .collect();
        for (&alloc_id, alloc) in &self.allocators {
            if !open_allocs.contains(&alloc_id) {
                alloc.reset();
            }
        }
        for &list_id in reopen_lists {
            let (already_open, alloc_id, init_pso) = {
                let Some(l) = self.lists.get(&list_id) else { continue };
                (l.cl.is_open(), l.alloc_id, l.initial_pso)
            };
            if already_open {
                // Setup left this list open while timing was disarmed (e.g. warmup == 0):
                // arm its opening timestamp now so `Close` records a complete GPU duration pair.
                let l = self.lists.get_mut(&list_id).unwrap();
                if self.timing_armed
                    && l.open_ts_slot.is_none()
                    && matches!(l.cl.kind, ListType::Direct | ListType::Compute | ListType::Copy)
                {
                    if let Some(timer) = self.gpu_timer.as_mut() {
                        let slot = timer.alloc_slot()?;
                        l.cl.write_timestamp(&timer.heap, slot)?;
                        l.open_ts_slot = Some(slot);
                    }
                }
                continue;
            }
            let alloc = self
                .allocators
                .get(&alloc_id)
                .ok_or_else(|| format!("unknown allocator {alloc_id:#x} for list {list_id:#x}"))?;
            let pso = if init_pso != 0 { self.psos.get(&init_pso).cloned() } else { None };
            let mut ts_slot = None;
            let l = self.lists.get_mut(&list_id).unwrap();
            l.cl.begin(alloc)?;
            if self.timing_armed && matches!(l.cl.kind, ListType::Direct | ListType::Compute | ListType::Copy) {
                if let Some(timer) = self.gpu_timer.as_mut() {
                    let slot = timer.alloc_slot()?;
                    l.cl.write_timestamp(&timer.heap, slot)?;
                    ts_slot = Some(slot);
                }
            }
            l.open_ts_slot = ts_slot;
            l.closed_ts_pair = None;
            l.submitted_since_reset = false;
            l.pending_indirect_relocs.clear();
            l.retained_resources.clear();
            l.retained_desc_heaps.clear();
            l.retained_query_heaps.clear();
            if let Some(p) = pso {
                bind_pso(&mut l.cl, p);
            }
        }
        Ok(())
    }

    /// Resolve accumulated hardware `gpu_busy` durations (ms) for each `(pair_lo, pair_hi)` slice.
    pub fn resolve_bench_gpu_busy_ms(&mut self, ranges: &[(usize, usize)]) -> Result<Vec<f64>, String> {
        self.wait_all_idle();
        let Some(q) = self.primary_queue() else {
            return Ok(vec![0.0; ranges.len()]);
        };
        let Some(timer) = self.gpu_timer.as_mut() else {
            return Ok(vec![0.0; ranges.len()]);
        };
        timer.resolve_all_iterations_ms(&q, ranges)
    }

    /// Read back the last presented swapchain buffer (or last bound/cleared RTV) as RGB8.
    pub fn capture_frame(&mut self) -> Result<FrameImage, String> {
        let blocked = self.blocked_queues();
        if !blocked.is_empty() {
            return Err(format!(
                "cannot capture frame: queue(s) {blocked:?} remain blocked on unsignaled fence waits"
            ));
        }
        self.wait_all_idle();
        let queue = self.primary_queue().ok_or("no command queue available for frame readback")?;
        let target = self
            .last_presented
            .clone()
            .or_else(|| self.last_rtv.clone())
            .ok_or("no swapchain back buffer or render target was recorded")?;
        golden::readback_texture(&queue, &target)
    }

    /// Look up a live resource by its recorded ID (useful in tests and diagnostics).
    pub fn resource(&self, id: u64) -> Option<&Arc<Resource>> {
        self.resources.get(&id)
    }

    fn register_va(&mut self, rec_va: u64, res: &Resource) {
        if rec_va != 0 && res.gpu_va != 0 && res.desc.is_buffer() {
            self.va_map.insert(rec_va, (res.desc.Width.max(1), res.gpu_va));
        }
    }

    fn translate_gpu_va(&self, rec_va: u64) -> u64 {
        if rec_va == 0 {
            return 0;
        }
        if let Some((&base, &(size, live_base))) = self.va_map.range(..=rec_va).next_back() {
            let off = rec_va - base;
            if off <= size {
                return live_base + off;
            }
        }
        rec_va
    }

    /// Copy `src_bytes` into a fresh `UPLOAD` buffer and relocate every 64-bit GPU virtual
    /// address embedded in each indirect command stride so both GPU expansion kernels and
    /// CPU-side VB/IB indirect binding observe live Metal buffer addresses.
    fn build_relocated_indirect_args(
        &self,
        sig: &LiveCmdSig,
        max_count: u32,
        src_bytes: &[u8],
    ) -> Result<Arc<Resource>, String> {
        let len = src_bytes.len();
        let desc = ResourceDesc {
            Dimension: d3d12_metal::types::dim::BUFFER,
            Width: (len as u64).max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: d3d12_metal::types::heap_type::UPLOAD, ..Default::default() };
        let scratch = Arc::new(
            Resource::new_committed(&desc, &hp, MTLTextureUsage::empty(), "replay relocated indirect args")
                .map_err(|e| format!("relocated indirect args buffer: {e:?}"))?,
        );
        self.write_relocated_indirect_args(&scratch, sig, max_count, src_bytes)?;
        Ok(scratch)
    }

    fn write_relocated_indirect_args(
        &self,
        scratch: &Resource,
        sig: &LiveCmdSig,
        max_count: u32,
        src_bytes: &[u8],
    ) -> Result<(), String> {
        let ptr = scratch
            .cpu_ptr()
            .ok_or("relocated indirect args upload buffer missing CPU pointer")?;
        let mut buf = src_bytes.to_vec();
        let stride = sig.layout.stride as usize;
        for cmd in 0..max_count as usize {
            let base = cmd.saturating_mul(stride);
            for &va_off in &sig.va_offsets {
                let pos = base.saturating_add(va_off as usize);
                if pos + 8 <= buf.len() {
                    let rec_va = u64::from_le_bytes(buf[pos..pos + 8].try_into().unwrap());
                    let live_va = self.translate_gpu_va(rec_va);
                    buf[pos..pos + 8].copy_from_slice(&live_va.to_le_bytes());
                }
            }
        }
        unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), ptr, buf.len()) };
        Ok(())
    }

    fn find_cpu_slot(&self, rec_cpu: u64) -> Option<(u64, usize)> {
        if rec_cpu == 0 {
            return None;
        }
        for (&id, h) in &self.desc_heaps {
            if let Some(idx) = h.cpu_index(rec_cpu) {
                return Some((id, idx));
            }
        }
        None
    }

    fn translate_gpu_handle(&self, rec_gpu: u64) -> Result<u64, String> {
        if rec_gpu == 0 {
            return Ok(0);
        }
        for h in self.desc_heaps.values() {
            if let Some(idx) = h.gpu_index(rec_gpu) {
                return Ok(h.live_gpu_addr(idx));
            }
        }
        Err(format!("unknown GPU descriptor handle {rec_gpu:#x}"))
    }

    fn resolve_target_view(&self, rec_cpu: u64) -> Result<(TargetView, Option<Arc<Resource>>), String> {
        let (heap_id, idx) = self
            .find_cpu_slot(rec_cpu)
            .ok_or_else(|| format!("unknown RTV/DSV CPU handle {rec_cpu:#x}"))?;
        let h = &self.desc_heaps[&heap_id];
        let tv = *h
            .targets
            .get(idx)
            .ok_or_else(|| format!("target view slot {idx} out of range"))?;
        if tv.resource == 0 {
            return Ok((tv, None));
        }
        let res = self
            .resources
            .values()
            .find(|r| Arc::as_ptr(r) as usize == tv.resource)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "RTV/DSV handle {rec_cpu:#x} references resource pointer {:#x} which is no longer live",
                    tv.resource
                )
            })?;
        Ok((tv, Some(res)))
    }

    fn resolve_root_sig(&self, id: u64, embedded_shader: &[u8]) -> Result<Arc<RootSignature>, String> {
        if id != 0 {
            return self
                .root_sigs
                .get(&id)
                .cloned()
                .ok_or_else(|| format!("unknown root signature {id:#x}"));
        }
        RootSignature::from_blob(embedded_shader)
            .map_err(|e| format!("embedded root signature: {e}"))
    }

    fn copy_single_descriptor(&mut self, dst_cpu: u64, src_cpu: u64, heap_type: u32) -> Result<(), String> {
        let (dst_id, dst_idx) = self
            .find_cpu_slot(dst_cpu)
            .ok_or_else(|| format!("CopyDescriptors dest handle {dst_cpu:#x} not in any descriptor heap"))?;
        let (src_id, src_idx) = self
            .find_cpu_slot(src_cpu)
            .ok_or_else(|| format!("CopyDescriptors src handle {src_cpu:#x} not in any descriptor heap"))?;
        if matches!(heap_type, heap_kind::RTV | heap_kind::DSV) {
            let tv = *self.desc_heaps[&src_id]
                .targets
                .get(src_idx)
                .ok_or("src target view out of range")?;
            let dst_h = self.desc_heaps.get_mut(&dst_id).unwrap();
            let slot = dst_h.targets.get_mut(dst_idx).ok_or("dst target view out of range")?;
            *slot = tv;
            return Ok(());
        }
        let entry = self.desc_heaps[&src_id].read_entry(src_idx)?;
        let src_live_cpu = self.desc_heaps[&src_id].live_cpu_addr(src_idx);
        let dst_live_cpu = {
            let dst_h = self.desc_heaps.get_mut(&dst_id).unwrap();
            dst_h.write_entry(dst_idx, &entry)?;
            dst_h.live_cpu_addr(dst_idx)
        };
        if heap_type == heap_kind::CBV_SRV_UAV {
            if let Some((res_ptr, desc)) = descriptor::uav_at(src_live_cpu) {
                let res = self.resources.values().find(|r| Arc::as_ptr(r) as usize == res_ptr);
                descriptor::record_uav(dst_live_cpu, res.map(|r| r.as_ref()), desc.as_ref());
            }
            descriptor::bump_desc_heap_gen();
        }
        Ok(())
    }

    fn decode_copy_loc_owned(
        resources: &HashMap<u64, Arc<Resource>>,
        blob: &[u8],
    ) -> Result<(Arc<Resource>, Result<u32, PlacedFootprint>), String> {
        if blob.len() < 48 {
            return Err(format!("D3D12_TEXTURE_COPY_LOCATION too short ({} bytes)", blob.len()));
        }
        let res_id = u64::from_le_bytes(blob[0..8].try_into().unwrap());
        let loc_type = u32::from_le_bytes(blob[8..12].try_into().unwrap());
        let res = resources
            .get(&res_id)
            .cloned()
            .ok_or_else(|| format!("CopyTextureRegion unknown resource {res_id:#x}"))?;
        if loc_type == 0 {
            let sub = u32::from_le_bytes(blob[16..20].try_into().unwrap());
            Ok((res, Ok(sub)))
        } else {
            let fp = read_copy::<PlacedFootprint>(&blob[16..48])?;
            Ok((res, Err(fp)))
        }
    }
}

impl Backend for MetalBackend {
    fn name(&self) -> &'static str {
        "metal"
    }

    fn call(&mut self, rec: &Record, _objs: &Objects) -> Result<Handled, String> {
        match &rec.call {
            Call::CreateDevice { .. } => {
                if d3d12_metal::metal::device().is_none() {
                    return Err("no Metal 4 device available".into());
                }
                Ok(Handled::Done)
            }
            Call::CreateCommandQueue { queue, desc, .. } => {
                let qd = if desc.len() >= std::mem::size_of::<CommandQueueDesc>() {
                    read_copy::<CommandQueueDesc>(&desc[..std::mem::size_of::<CommandQueueDesc>()])?
                } else {
                    CommandQueueDesc::default()
                };
                let kind = ListType::from_raw(qd.Type).unwrap_or(ListType::Direct);
                let q = Arc::new(
                    Queue::new(kind, &format!("replay queue {queue:#x}"))
                        .ok_or("Queue::new failed")?,
                );
                if kind == ListType::Direct && self.primary_queue.is_none() {
                    self.primary_queue = Some(*queue);
                }
                self.queues.insert(*queue, q);
                Ok(Handled::Done)
            }
            Call::CreateCommandAllocator { allocator, list_type, .. } => {
                let kind = ListType::from_raw(*list_type).unwrap_or(ListType::Direct);
                let a = CommandAllocator::new(kind).ok_or("CommandAllocator::new failed")?;
                self.allocators.insert(*allocator, a);
                Ok(Handled::Done)
            }
            Call::CreateCommandList { list, list_type, allocator, initial_pso, .. } => {
                let kind = ListType::from_raw(*list_type).unwrap_or(ListType::Direct);
                let mut cl = CommandList::new(kind).ok_or("CommandList::new failed")?;
                let mut open_ts_slot = None;
                // `ID3D12Device4::CreateCommandList1` captures `allocator == 0` and creates a
                // closed command list that will be opened later by `Reset`.
                if *allocator != 0 {
                    let alloc = self
                        .allocators
                        .get(allocator)
                        .ok_or_else(|| format!("unknown allocator {allocator:#x}"))?;
                    cl.begin(alloc)?;
                    if self.timing_armed && matches!(kind, ListType::Direct | ListType::Compute | ListType::Copy) {
                        if let Some(timer) = self.gpu_timer.as_mut() {
                            let slot = timer.alloc_slot()?;
                            cl.write_timestamp(&timer.heap, slot)?;
                            open_ts_slot = Some(slot);
                        }
                    }
                    if *initial_pso != 0 {
                        if let Some(p) = self.psos.get(initial_pso).cloned() {
                            bind_pso(&mut cl, p);
                        }
                    }
                }
                self.lists.insert(
                    *list,
                    LiveCommandList {
                        cl,
                        alloc_id: *allocator,
                        initial_pso: *initial_pso,
                        open_ts_slot,
                        closed_ts_pair: None,
                        submitted_since_reset: false,
                        pending_indirect_relocs: Vec::new(),
                        retained_resources: Vec::new(),
                        retained_desc_heaps: Vec::new(),
                        retained_query_heaps: Vec::new(),
                    },
                );
                Ok(Handled::Done)
            }
            Call::CreateDescriptorHeap { heap, desc, cpu_start, gpu_start, .. } => {
                if self.desc_heaps.contains_key(heap) {
                    return Err(format!("CreateDescriptorHeap: duplicate still-live descriptor heap id {heap:#x}"));
                }
                let d = read_copy::<DescriptorHeapDesc>(desc)?;
                let shader_visible = (d.Flags & 1) != 0;
                let dh = DescriptorHeap::new(
                    d.Type,
                    d.NumDescriptors,
                    shader_visible,
                    &format!("replay desc heap {heap:#x}"),
                )?;
                let stride = dh.descriptor_size() as u64;
                let targets = if matches!(d.Type, heap_kind::RTV | heap_kind::DSV) {
                    vec![TargetView::NULL; d.NumDescriptors.max(1) as usize]
                } else {
                    Vec::new()
                };
                self.desc_heaps.insert(
                    *heap,
                    LiveDescHeap {
                        heap: Arc::new(dh),
                        rec_cpu_start: *cpu_start,
                        rec_gpu_start: *gpu_start,
                        rec_stride: stride,
                        targets,
                    },
                );
                Ok(Handled::Done)
            }
            Call::CreateHeap { heap, desc, .. } => {
                if self.heaps.contains_key(heap) {
                    return Err(format!("CreateHeap: duplicate still-live heap id {heap:#x}"));
                }
                let d = read_copy::<HeapDesc>(desc)?;
                let h = Heap::new(&d, &format!("replay heap {heap:#x}"))
                    .map_err(|e| format!("Heap::new: {e:?}"))?;
                self.heaps.insert(*heap, Arc::new(h));
                Ok(Handled::Done)
            }
            Call::CreateCommittedResource { resource, heap_properties, desc, gpu_va, .. } => {
                if self.resources.contains_key(resource) {
                    return Err(format!("CreateCommittedResource: duplicate still-live resource id {resource:#x}"));
                }
                let hp = if heap_properties.len() >= std::mem::size_of::<HeapProperties>() {
                    read_copy::<HeapProperties>(&heap_properties[..std::mem::size_of::<HeapProperties>()])?
                } else if heap_properties.len() >= 4 {
                    HeapProperties {
                        Type: u32::from_le_bytes(heap_properties[0..4].try_into().unwrap()),
                        ..Default::default()
                    }
                } else {
                    HeapProperties { Type: d3d12_metal::types::heap_type::DEFAULT, ..Default::default() }
                };
                let rd = read_copy_prefix::<ResourceDesc>(desc)?;
                let res = Arc::new(
                    Resource::new_committed_default(&rd, &hp, &format!("replay res {resource:#x}"))
                        .map_err(|e| format!("Resource::new_committed: {e:?}"))?,
                );
                self.register_va(*gpu_va, &res);
                self.resources.insert(*resource, res);
                Ok(Handled::Done)
            }
            Call::CreatePlacedResource { resource, heap, offset, desc, gpu_va, .. } => {
                if self.resources.contains_key(resource) {
                    return Err(format!("CreatePlacedResource: duplicate still-live resource id {resource:#x}"));
                }
                let h = self
                    .heaps
                    .get(heap)
                    .ok_or_else(|| format!("unknown heap {heap:#x}"))?;
                let rd = read_copy_prefix::<ResourceDesc>(desc)?;
                let res = Arc::new(
                    Resource::new_placed_default(h, *offset, &rd, &format!("replay placed {resource:#x}"))
                        .map_err(|e| format!("Resource::new_placed: {e:?}"))?,
                );
                self.register_va(*gpu_va, &res);
                self.resources.insert(*resource, res);
                Ok(Handled::Done)
            }
            Call::CreateReservedResource { .. } => Ok(Handled::Skipped),
            Call::CreateRootSignature { root_signature, blob, .. } => {
                let rs = RootSignature::from_blob(blob).map_err(|e| format!("RootSignature: {e}"))?;
                self.root_sigs.insert(*root_signature, rs);
                Ok(Handled::Done)
            }
            Call::CreateGraphicsPipelineState { pso, desc, .. } => {
                if !desc.hs.is_empty() || !desc.ds.is_empty() {
                    return Ok(Handled::Skipped);
                }
                let rs = self.resolve_root_sig(desc.root_signature, desc.vs)?;
                let gdesc = decode_graphics_desc(desc)?;
                let gp = if !desc.gs.is_empty() {
                    GraphicsPipeline::new_with_geometry(rs, &gdesc, desc.gs, &format!("replay gs pso {pso:#x}"))?
                } else {
                    GraphicsPipeline::new(rs, &gdesc, &format!("replay gfx pso {pso:#x}"))?
                };
                self.psos.insert(*pso, LivePso::Graphics(Arc::new(gp)));
                Ok(Handled::Done)
            }
            Call::CreateMeshPipelineState { pso, amplification, mesh, desc, .. } => {
                let rs = self.resolve_root_sig(desc.root_signature, if !amplification.is_empty() { amplification } else { mesh })?;
                let mdesc = decode_mesh_desc(amplification, mesh, desc)?;
                let mp = MeshPipeline::new(rs, &mdesc, &format!("replay mesh pso {pso:#x}"))?;
                self.psos.insert(*pso, LivePso::Mesh(Arc::new(mp)));
                Ok(Handled::Done)
            }
            Call::CreateComputePipelineState { pso, root_signature, cs, .. } => {
                let rs = self.resolve_root_sig(*root_signature, cs)?;
                let cp = ComputePipeline::new(rs, cs, &format!("replay cs pso {pso:#x}"))?;
                self.psos.insert(*pso, LivePso::Compute(Arc::new(cp)));
                Ok(Handled::Done)
            }
            Call::CreateStateObject { .. } | Call::AddToStateObject { .. } => Ok(Handled::Skipped),
            Call::CreateCommandSignature { signature, byte_stride, arguments, .. } => {
                if !arguments.len().is_multiple_of(16) {
                    return Err(format!("CreateCommandSignature arguments {} bytes not multiple of 16", arguments.len()));
                }
                let args: Vec<ArgumentDesc> = arguments
                    .chunks_exact(16)
                    .map(|c| {
                        let ty = u32::from_le_bytes(c[0..4].try_into().unwrap());
                        let u0 = u32::from_le_bytes(c[4..8].try_into().unwrap());
                        let u1 = u32::from_le_bytes(c[8..12].try_into().unwrap());
                        let u2 = u32::from_le_bytes(c[12..16].try_into().unwrap());
                        (ty, [u0, u1, u2])
                    })
                    .collect();
                let layout = IndirectLayout::new(*byte_stride, &args)?;
                let va_offsets = indirect_va_offsets(&args);
                self.cmd_sigs.insert(*signature, LiveCmdSig { layout, va_offsets });
                Ok(Handled::Done)
            }
            Call::CreateFence { fence, initial_value, .. } => {
                let f = Fence::new(*initial_value).ok_or("Fence::new failed")?;
                self.fences.insert(*fence, Arc::new(f));
                self.fence_signals.insert(*fence, vec![(*initial_value, None)]);
                Ok(Handled::Done)
            }
            Call::CreateQueryHeap { heap, desc, .. } => {
                if desc.len() < 8 {
                    return Err(format!("CreateQueryHeap desc too short ({} bytes)", desc.len()));
                }
                let ty = u32::from_le_bytes(desc[0..4].try_into().unwrap());
                let count = u32::from_le_bytes(desc[4..8].try_into().unwrap());
                let qh = QueryHeap::new(ty, count)?;
                self.query_heaps.insert(*heap, Arc::new(qh));
                Ok(Handled::Done)
            }
            Call::Release { object } => {
                if let Some(r) = self.resources.remove(object) {
                    self.retired.push(Box::new(r));
                } else if let Some(h) = self.heaps.remove(object) {
                    self.retired.push(Box::new(h));
                } else if let Some(dh) = self.desc_heaps.remove(object) {
                    self.retired.push(Box::new(dh));
                } else if let Some(p) = self.psos.remove(object) {
                    self.retired.push(Box::new(p));
                } else if let Some(rs) = self.root_sigs.remove(object) {
                    self.retired.push(Box::new(rs));
                } else if let Some(l) = self.lists.remove(object) {
                    self.retired.push(Box::new(l));
                } else if let Some(a) = self.allocators.remove(object) {
                    self.retired.push(Box::new(a));
                } else if let Some(q) = self.queues.remove(object) {
                    self.retired.push(Box::new(q));
                } else if let Some(f) = self.fences.remove(object) {
                    self.retired.push(Box::new(f));
                } else if let Some(qh) = self.query_heaps.remove(object) {
                    self.retired.push(Box::new(qh));
                } else if let Some(sc) = self.swapchains.remove(object) {
                    self.retired.push(Box::new(sc));
                } else {
                    self.cmd_sigs.remove(object);
                }
                Ok(Handled::Done)
            }
            Call::CreateShaderResourceView { resource, desc, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateShaderResourceView unknown CPU handle {dest:#x}"))?;
                let res = if *resource != 0 {
                    Some(
                        self.resources
                            .get(resource)
                            .ok_or_else(|| format!("CreateShaderResourceView unknown resource {resource:#x}"))?
                            .as_ref(),
                    )
                } else {
                    None
                };
                let mut desc_arr = [0u8; 40];
                let d_opt = match desc {
                    Some(b) => {
                        if b.len() < 40 {
                            return Err(format!("SRV desc too short ({} bytes)", b.len()));
                        }
                        desc_arr.copy_from_slice(&b[..40]);
                        // Acceleration structure SRV (dimension == 11) stores GPU VA at offset 16.
                        if u32::from_le_bytes(desc_arr[4..8].try_into().unwrap()) == 11 {
                            let va = u64::from_le_bytes(desc_arr[16..24].try_into().unwrap());
                            let live = self.translate_gpu_va(va);
                            desc_arr[16..24].copy_from_slice(&live.to_le_bytes());
                        }
                        Some(&desc_arr)
                    }
                    None => None,
                };
                let entry = views::srv_entry(res, d_opt)?;
                self.desc_heaps.get_mut(&heap_id).unwrap().write_entry(idx, &entry)?;
                descriptor::bump_desc_heap_gen();
                Ok(Handled::Done)
            }
            Call::CreateUnorderedAccessView { resource, counter, desc, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateUnorderedAccessView unknown CPU handle {dest:#x}"))?;
                let res = if *resource != 0 {
                    Some(
                        self.resources
                            .get(resource)
                            .ok_or_else(|| format!("CreateUnorderedAccessView unknown resource {resource:#x}"))?
                            .as_ref(),
                    )
                } else {
                    None
                };
                let ctr = if *counter != 0 {
                    Some((
                        self.resources
                            .get(counter)
                            .ok_or_else(|| format!("CreateUnorderedAccessView unknown counter {counter:#x}"))?
                            .as_ref(),
                        0u64,
                    ))
                } else {
                    None
                };
                let mut desc_arr = [0u8; 40];
                let d_opt = match desc {
                    Some(b) => {
                        if b.len() < 40 {
                            return Err(format!("UAV desc too short ({} bytes)", b.len()));
                        }
                        desc_arr.copy_from_slice(&b[..40]);
                        Some(&desc_arr)
                    }
                    None => None,
                };
                let entry = views::uav_entry_with_counter(res, ctr, d_opt)?;
                let live_cpu = {
                    let h = self.desc_heaps.get_mut(&heap_id).unwrap();
                    h.write_entry(idx, &entry)?;
                    h.live_cpu_addr(idx)
                };
                descriptor::record_uav(live_cpu, res, d_opt);
                descriptor::bump_desc_heap_gen();
                Ok(Handled::Done)
            }
            Call::CreateRenderTargetView { resource, desc, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateRenderTargetView unknown CPU handle {dest:#x}"))?;
                let tv = if *resource == 0 {
                    TargetView::NULL
                } else {
                    let res = self
                        .resources
                        .get(resource)
                        .ok_or_else(|| format!("CreateRenderTargetView unknown resource {resource:#x}"))?;
                    let words = decode_target_words(*desc)?;
                    TargetView::rtv(res, words.as_ref())?
                };
                let h = self.desc_heaps.get_mut(&heap_id).unwrap();
                let slot = h.targets.get_mut(idx).ok_or("RTV slot out of range")?;
                *slot = tv;
                Ok(Handled::Done)
            }
            Call::CreateDepthStencilView { resource, desc, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateDepthStencilView unknown CPU handle {dest:#x}"))?;
                let tv = if *resource == 0 {
                    TargetView::NULL
                } else {
                    let res = self
                        .resources
                        .get(resource)
                        .ok_or_else(|| format!("CreateDepthStencilView unknown resource {resource:#x}"))?;
                    let words = decode_target_words(*desc)?;
                    TargetView::dsv(res, words.as_ref())?
                };
                let h = self.desc_heaps.get_mut(&heap_id).unwrap();
                let slot = h.targets.get_mut(idx).ok_or("DSV slot out of range")?;
                *slot = tv;
                Ok(Handled::Done)
            }
            Call::CreateConstantBufferView { buffer_location, size, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateConstantBufferView unknown CPU handle {dest:#x}"))?;
                let live_va = self.translate_gpu_va(*buffer_location);
                let entry = views::cbv_entry(live_va, *size);
                self.desc_heaps.get_mut(&heap_id).unwrap().write_entry(idx, &entry)?;
                descriptor::bump_desc_heap_gen();
                Ok(Handled::Done)
            }
            Call::CreateSampler { desc, dest } => {
                let (heap_id, idx) = self
                    .find_cpu_slot(*dest)
                    .ok_or_else(|| format!("CreateSampler unknown CPU handle {dest:#x}"))?;
                let sd = read_copy::<SamplerDesc>(desc)?;
                let entry = views::sampler_entry(&sd)?;
                self.desc_heaps.get_mut(&heap_id).unwrap().write_entry(idx, &entry)?;
                Ok(Handled::Done)
            }
            Call::CopyDescriptorsSimple { count, dest, src, heap_type } => {
                let stride = descriptor::descriptor_size(*heap_type)
                    .ok_or_else(|| format!("invalid heap_type {heap_type}"))? as u64;
                for i in 0..(*count as u64) {
                    self.copy_single_descriptor(dest + i * stride, src + i * stride, *heap_type)?;
                }
                Ok(Handled::Done)
            }
            Call::CopyDescriptors { dest_ranges, src_ranges, heap_type } => {
                let stride = descriptor::descriptor_size(*heap_type)
                    .ok_or_else(|| format!("invalid heap_type {heap_type}"))? as u64;
                let expand = |ranges: &d3d12_metal::trace::U64s| -> Result<Vec<u64>, String> {
                    if !ranges.len().is_multiple_of(2) {
                        return Err("CopyDescriptors range list must have even length".into());
                    }
                    let mut out = Vec::new();
                    let raw = ranges.to_vec();
                    for pair in raw.chunks_exact(2) {
                        for k in 0..pair[1] {
                            out.push(pair[0] + k * stride);
                        }
                    }
                    Ok(out)
                };
                let dsts = expand(dest_ranges)?;
                let srcs = expand(src_ranges)?;
                for (d, s) in dsts.into_iter().zip(srcs) {
                    self.copy_single_descriptor(d, s, *heap_type)?;
                }
                Ok(Handled::Done)
            }
            Call::WriteResource { resource, offset, data, .. } => {
                // Captured traces do not record CPU fence waits (`SetEventOnCompletion`), while
                // `Queue::execute_command_lists` submits asynchronously. Drain unblocked queues
                // before overwriting shared upload memory so an earlier submission never reads
                // data written for a later submission, without deadlocking queues waiting on a
                // producer that appears later in the trace.
                self.drain_unblocked_queues();
                let res = self
                    .resources
                    .get(resource)
                    .ok_or_else(|| format!("WriteResource unknown resource {resource:#x}"))?;
                let ptr = res
                    .cpu_ptr()
                    .ok_or_else(|| format!("WriteResource on non-CPU-accessible resource {resource:#x}"))?;
                let end = offset
                    .checked_add(data.len() as u64)
                    .ok_or("WriteResource offset overflow")?;
                if end > res.desc.Width {
                    return Err(format!("WriteResource {offset}+{} exceeds resource width {}", data.len(), res.desc.Width));
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr(), ptr.add(*offset as usize), data.len());
                }
                res.record_cpu_copy(*offset, data);
                Ok(Handled::Done)
            }
            Call::Reset { list, allocator, pso } => {
                let alloc = self
                    .allocators
                    .get(allocator)
                    .ok_or_else(|| format!("Reset unknown allocator {allocator:#x}"))?;
                let live_pso = if *pso != 0 { self.psos.get(pso).cloned() } else { None };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("Reset unknown command list {list:#x}"))?;
                l.cl.begin(alloc)?;
                l.alloc_id = *allocator;
                l.initial_pso = *pso;
                let mut open_ts_slot = None;
                if self.timing_armed && matches!(l.cl.kind, ListType::Direct | ListType::Compute | ListType::Copy) {
                    if let Some(timer) = self.gpu_timer.as_mut() {
                        let slot = timer.alloc_slot()?;
                        l.cl.write_timestamp(&timer.heap, slot)?;
                        open_ts_slot = Some(slot);
                    }
                }
                l.open_ts_slot = open_ts_slot;
                l.closed_ts_pair = None;
                l.submitted_since_reset = false;
                l.pending_indirect_relocs.clear();
                let old_retained = std::mem::take(&mut l.retained_resources);
                let old_desc_heaps = std::mem::take(&mut l.retained_desc_heaps);
                let old_query_heaps = std::mem::take(&mut l.retained_query_heaps);
                if let Some(p) = live_pso {
                    bind_pso(&mut l.cl, p);
                }
                if !old_retained.is_empty() {
                    self.retired.push(Box::new(old_retained));
                }
                if !old_desc_heaps.is_empty() {
                    self.retired.push(Box::new(old_desc_heaps));
                }
                if !old_query_heaps.is_empty() {
                    self.retired.push(Box::new(old_query_heaps));
                }
                Ok(Handled::Done)
            }
            Call::Close { list } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("Close unknown command list {list:#x}"))?;
                l.closed_ts_pair = None;
                if let Some(start_slot) = l.open_ts_slot.take() {
                    if self.timing_armed {
                        if let Some(timer) = self.gpu_timer.as_mut() {
                            let end_slot = timer.alloc_slot()?;
                            l.cl.write_timestamp(&timer.heap, end_slot)?;
                            l.closed_ts_pair = Some((start_slot, end_slot));
                        }
                    }
                }
                l.cl.close()?;
                Ok(Handled::Done)
            }
            Call::ClearState { list, pso } => {
                let live_pso = if *pso != 0 { self.psos.get(pso).cloned() } else { None };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("ClearState unknown command list {list:#x}"))?;
                l.cl.clear_state();
                if let Some(p) = live_pso {
                    bind_pso(&mut l.cl, p);
                }
                Ok(Handled::Done)
            }
            Call::SetPredication { list, buffer, aligned_offset, operation } => {
                let (pred, pred_res) = if *buffer == 0 {
                    (None, None)
                } else {
                    let res = self
                        .resources
                        .get(buffer)
                        .cloned()
                        .ok_or_else(|| format!("SetPredication unknown buffer {buffer:#x}"))?;
                    (Some((res.gpu_va + aligned_offset, *operation)), Some(res))
                };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("SetPredication unknown command list {list:#x}"))?;
                if let Some(r) = pred_res {
                    l.retained_resources.push(r);
                }
                l.cl.set_predication(pred);
                Ok(Handled::Done)
            }
            Call::ResourceBarrier { list, barriers } => {
                let raw_barriers = read_copy_array::<ResourceBarrier>(barriers)?;
                let mut items = Vec::with_capacity(raw_barriers.len());
                let mut barrier_res = Vec::with_capacity(raw_barriers.len());
                for b in &raw_barriers {
                    match b.Type {
                        0 => {
                            let res_id = b.u[0];
                            let ptr = if let Some(r) = self.resources.get(&res_id) {
                                barrier_res.push(Arc::clone(r));
                                Arc::as_ptr(r) as usize
                            } else {
                                0
                            };
                            let sub = b.u[1] as u32;
                            let before = (b.u[1] >> 32) as u32;
                            let after = b.u[2] as u32;
                            match b.Flags {
                                1 => items.push(Barrier::SplitBegin(ptr, sub, before, after)),
                                2 => items.push(Barrier::SplitEnd(ptr, sub, before, after)),
                                _ => items.push(Barrier::Transition(ptr, before, after)),
                            }
                        }
                        1 => items.push(Barrier::Full),
                        2 => {
                            let res_id = b.u[0];
                            let ptr = if let Some(r) = self.resources.get(&res_id) {
                                barrier_res.push(Arc::clone(r));
                                Arc::as_ptr(r) as usize
                            } else {
                                0
                            };
                            items.push(Barrier::Uav(ptr));
                        }
                        _ => items.push(Barrier::Full),
                    }
                }
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("ResourceBarrier unknown command list {list:#x}"))?;
                l.retained_resources.extend(barrier_res);
                l.cl.barrier_list(&items);
                Ok(Handled::Done)
            }
            Call::SetPipelineState { list, pso } => {
                let live_pso = self
                    .psos
                    .get(pso)
                    .cloned()
                    .ok_or_else(|| format!("SetPipelineState unknown PSO {pso:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("SetPipelineState unknown command list {list:#x}"))?;
                bind_pso(&mut l.cl, live_pso);
                Ok(Handled::Done)
            }
            Call::SetGraphicsRootSignature { list, root_signature } => {
                let rs = self
                    .root_sigs
                    .get(root_signature)
                    .cloned()
                    .ok_or_else(|| format!("SetGraphicsRootSignature unknown RS {root_signature:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.gfx_root.set_root_signature(rs)?;
                Ok(Handled::Done)
            }
            Call::SetComputeRootSignature { list, root_signature } => {
                let rs = self
                    .root_sigs
                    .get(root_signature)
                    .cloned()
                    .ok_or_else(|| format!("SetComputeRootSignature unknown RS {root_signature:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.compute_root.set_root_signature(rs)?;
                Ok(Handled::Done)
            }
            Call::SetDescriptorHeaps { list, heaps } => {
                let (mut res_gpu, mut samp_gpu) = (0u64, 0u64);
                let mut bound_heaps = Vec::with_capacity(heaps.len());
                for hid in heaps.iter() {
                    let h = self
                        .desc_heaps
                        .get(&hid)
                        .ok_or_else(|| format!("SetDescriptorHeaps unknown heap {hid:#x}"))?;
                    bound_heaps.push(Arc::clone(&h.heap));
                    match h.heap.kind {
                        heap_kind::CBV_SRV_UAV => res_gpu = h.heap.gpu_base,
                        heap_kind::SAMPLER => samp_gpu = h.heap.gpu_base,
                        _ => {}
                    }
                }
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.retained_desc_heaps.extend(bound_heaps);
                l.cl.set_descriptor_heaps(res_gpu, samp_gpu);
                Ok(Handled::Done)
            }
            Call::SetRootDescriptorTable { list, compute, param, gpu_handle } => {
                let live_gpu = self.translate_gpu_handle(*gpu_handle)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                if *compute {
                    l.cl.compute_root.set_table(*param, live_gpu)?;
                } else {
                    l.cl.gfx_root.set_table(*param, live_gpu)?;
                }
                Ok(Handled::Done)
            }
            Call::SetRoot32BitConstants { list, compute, param, dest_offset, values } => {
                let vals = values.to_vec();
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                if *compute {
                    l.cl.compute_root.set_constants(*param, *dest_offset, &vals)?;
                } else {
                    l.cl.gfx_root.set_constants(*param, *dest_offset, &vals)?;
                }
                Ok(Handled::Done)
            }
            Call::SetRootView { list, compute, param, gpu_va, .. } => {
                let live_va = self.translate_gpu_va(*gpu_va);
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                if *compute {
                    l.cl.compute_root.set_descriptor(*param, live_va)?;
                } else {
                    l.cl.gfx_root.set_descriptor(*param, live_va)?;
                }
                Ok(Handled::Done)
            }
            Call::IASetPrimitiveTopology { list, topology } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.gfx.topology = *topology;
                Ok(Handled::Done)
            }
            Call::IASetVertexBuffers { list, start_slot, views } => {
                let vbs = pod::read_array::<VertexBufferView>(views)?;
                let translated: Vec<(usize, (u64, u32, u32))> = vbs
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let slot = *start_slot as usize + i;
                        (slot, (self.translate_gpu_va(v.BufferLocation), v.SizeInBytes, v.StrideInBytes))
                    })
                    .collect();
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                for (slot, entry) in translated {
                    if slot < l.cl.gfx.vbs.len() {
                        l.cl.gfx.vbs[slot] = entry;
                    }
                }
                Ok(Handled::Done)
            }
            Call::IASetIndexBuffer { list, view } => {
                let ib = match view {
                    Some(b) => {
                        let v = pod::read::<IndexBufferView>(b)?;
                        Some((self.translate_gpu_va(v.BufferLocation), v.SizeInBytes, v.Format))
                    }
                    None => None,
                };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.gfx.ib = ib;
                Ok(Handled::Done)
            }
            Call::RSSetViewports { list, viewports } => {
                let vps = read_copy_array::<Viewport>(viewports)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_viewports_d3d(&vps);
                Ok(Handled::Done)
            }
            Call::RSSetScissorRects { list, rects } => {
                let rs = read_copy_array::<Rect>(rects)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_scissors(rs);
                Ok(Handled::Done)
            }
            Call::OMSetRenderTargets { list, rtvs, dsv } => {
                let mut live_rtvs = Vec::with_capacity(rtvs.len());
                let mut bound_res = Vec::with_capacity(rtvs.len() + 1);
                let mut first_rtv_res = None;
                for h in rtvs.iter() {
                    if h == 0 {
                        live_rtvs.push(TargetView::NULL);
                    } else {
                        let (tv, res) = self.resolve_target_view(h)?;
                        if let Some(r) = res {
                            if first_rtv_res.is_none() {
                                first_rtv_res = Some(Arc::clone(&r));
                            }
                            bound_res.push(r);
                        }
                        live_rtvs.push(tv);
                    }
                }
                if first_rtv_res.is_some() {
                    self.last_rtv = first_rtv_res;
                }
                let live_dsv = if *dsv != 0 {
                    let (tv, res) = self.resolve_target_view(*dsv)?;
                    if let Some(r) = res {
                        bound_res.push(r);
                    }
                    Some(tv)
                } else {
                    None
                };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.retained_resources.extend(bound_res);
                l.cl.set_render_targets(live_rtvs, live_dsv);
                Ok(Handled::Done)
            }
            Call::OMSetBlendFactor { list, factor } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_blend_factor(*factor);
                Ok(Handled::Done)
            }
            Call::OMSetStencilRef { list, stencil_ref } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_stencil_ref(*stencil_ref);
                Ok(Handled::Done)
            }
            Call::OMSetFrontAndBackStencilRef { list, front, back } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_front_and_back_stencil_ref(*front, *back);
                Ok(Handled::Done)
            }
            Call::OMSetDepthBounds { list, min, max } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.set_depth_bounds(*min, *max);
                Ok(Handled::Done)
            }
            Call::ClearRenderTargetView { list, rtv, color, rects } => {
                let (tv, res) = self.resolve_target_view(*rtv)?;
                if let Some(ref r) = res {
                    self.last_rtv = Some(Arc::clone(r));
                }
                let rs = read_copy_array::<Rect>(rects)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                if let Some(r) = res {
                    l.retained_resources.push(r);
                }
                unsafe { l.cl.clear_rtv(&tv, *color, &rs)? };
                Ok(Handled::Done)
            }
            Call::ClearDepthStencilView { list, dsv, flags, depth, stencil, rects } => {
                let (tv, res) = self.resolve_target_view(*dsv)?;
                let rs = read_copy_array::<Rect>(rects)?;
                let clear_d = (*flags & 1) != 0;
                let clear_s = (*flags & 2) != 0;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                if let Some(r) = res {
                    l.retained_resources.push(r);
                }
                unsafe { l.cl.clear_dsv(&tv, clear_d, *depth, clear_s, *stencil, &rs)? };
                Ok(Handled::Done)
            }
            Call::ClearUnorderedAccessViewUint { list, cpu_handle, resource, values, rects, .. } => {
                let res = self
                    .resources
                    .get(resource)
                    .cloned()
                    .ok_or_else(|| format!("ClearUAVUint unknown resource {resource:#x}"))?;
                let live_cpu = self
                    .find_cpu_slot(*cpu_handle)
                    .map(|(hid, idx)| self.desc_heaps[&hid].live_cpu_addr(idx))
                    .unwrap_or(0);
                let mut v = [0u32; 4];
                for (i, x) in values.iter().take(4).enumerate() {
                    v[i] = x;
                }
                let rs = read_copy_array::<Rect>(rects)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.clear_uav_handle(live_cpu, &res, ClearValue::Uint(v), &rs)?;
                l.retained_resources.push(res);
                Ok(Handled::Done)
            }
            Call::ClearUnorderedAccessViewFloat { list, cpu_handle, resource, values, rects, .. } => {
                let res = self
                    .resources
                    .get(resource)
                    .cloned()
                    .ok_or_else(|| format!("ClearUAVFloat unknown resource {resource:#x}"))?;
                let live_cpu = self
                    .find_cpu_slot(*cpu_handle)
                    .map(|(hid, idx)| self.desc_heaps[&hid].live_cpu_addr(idx))
                    .unwrap_or(0);
                let rs = read_copy_array::<Rect>(rects)?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.clear_uav_handle(live_cpu, &res, ClearValue::Float(*values), &rs)?;
                l.retained_resources.push(res);
                Ok(Handled::Done)
            }
            Call::DiscardResource { list, resource, region } => {
                let res = self
                    .resources
                    .get(resource)
                    .cloned()
                    .ok_or_else(|| format!("DiscardResource unknown resource {resource:#x}"))?;
                let reg = region.and_then(|b| {
                    if b.len() >= 24 {
                        let rects = u32::from_le_bytes(b[0..4].try_into().unwrap());
                        let first = u32::from_le_bytes(b[16..20].try_into().unwrap());
                        let count = u32::from_le_bytes(b[20..24].try_into().unwrap());
                        Some((rects, first, count))
                    } else {
                        None
                    }
                });
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.discard_resource(&res, reg);
                l.retained_resources.push(res);
                Ok(Handled::Done)
            }
            Call::DrawInstanced { list, vertex_count, instance_count, start_vertex, start_instance } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.draw(*vertex_count, *instance_count, *start_vertex, *start_instance)?;
                Ok(Handled::Done)
            }
            Call::DrawIndexedInstanced { list, index_count, instance_count, start_index, base_vertex, start_instance } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.draw_indexed(*index_count, *instance_count, *start_index, *base_vertex, *start_instance)?;
                Ok(Handled::Done)
            }
            Call::Dispatch { list, x, y, z } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.dispatch(*x, *y, *z)?;
                Ok(Handled::Done)
            }
            Call::DispatchMesh { list, x, y, z } => {
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.dispatch_mesh(*x, *y, *z)?;
                Ok(Handled::Done)
            }
            Call::ExecuteIndirect { list, signature, max_count, args, args_offset, count, count_offset } => {
                let sig = self
                    .cmd_sigs
                    .get(signature)
                    .cloned()
                    .ok_or_else(|| format!("ExecuteIndirect unknown signature {signature:#x}"))?;
                let args_res = self
                    .resources
                    .get(args)
                    .cloned()
                    .ok_or_else(|| format!("ExecuteIndirect unknown args buffer {args:#x}"))?;
                let (count_va, count_res) = if *count != 0 {
                    let c_res = self
                        .resources
                        .get(count)
                        .cloned()
                        .ok_or_else(|| format!("ExecuteIndirect unknown count buffer {count:#x}"))?;
                    (Some(c_res.gpu_va + count_offset), Some(c_res))
                } else {
                    (None, None)
                };
                let args_len = (*max_count as usize)
                    .checked_mul(sig.layout.stride as usize)
                    .ok_or("ExecuteIndirect args byte length overflow")?;
                let relocated_scratch = if !sig.va_offsets.is_empty() && args_len > 0 {
                    let src_bytes = args_res.shared_slice(*args_offset, args_len).ok_or_else(|| {
                        format!(
                            "ExecuteIndirect signature {signature:#x} has {} embedded GPU VA field(s), \
                             but argument buffer {args:#x} has no CPU-accessible or CPU-shadowed copy to relocate",
                            sig.va_offsets.len()
                        )
                    })?;
                    let scratch = self.build_relocated_indirect_args(&sig, *max_count, src_bytes)?;
                    Some(scratch)
                } else {
                    None
                };
                let (live_args_va, args_cpu) = match relocated_scratch.as_ref() {
                    Some(s) => (s.gpu_va, s.shared_slice(0, args_len)),
                    None => (args_res.gpu_va + args_offset, args_res.shared_slice(*args_offset, args_len)),
                };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.execute_indirect(&sig.layout, *max_count, live_args_va, args_cpu, count_va)?;
                l.retained_resources.push(args_res);
                if let Some(cr) = count_res {
                    l.retained_resources.push(cr);
                }
                if let Some(scratch) = relocated_scratch {
                    l.retained_resources.push(Arc::clone(&scratch));
                    l.pending_indirect_relocs.push(PendingIndirectReloc {
                        sig,
                        max_count: *max_count,
                        src_res_id: *args,
                        src_offset: *args_offset,
                        args_len,
                        scratch,
                    });
                }
                Ok(Handled::Done)
            }
            Call::CopyBufferRegion { list, dst, dst_offset, src, src_offset, size } => {
                let dr = self
                    .resources
                    .get(dst)
                    .cloned()
                    .ok_or_else(|| format!("CopyBufferRegion unknown dst {dst:#x}"))?;
                let sr = self
                    .resources
                    .get(src)
                    .cloned()
                    .ok_or_else(|| format!("CopyBufferRegion unknown src {src:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.copy_buffer(&dr, *dst_offset, &sr, *src_offset, *size)?;
                l.retained_resources.push(dr);
                l.retained_resources.push(sr);
                Ok(Handled::Done)
            }
            Call::CopyTextureRegion { list, dst, dst_x, dst_y, dst_z, src, src_box } => {
                let (d_res, d_kind) = Self::decode_copy_loc_owned(&self.resources, dst)?;
                let (s_res, s_kind) = Self::decode_copy_loc_owned(&self.resources, src)?;
                let sbox = match src_box {
                    Some(b) => Some(read_copy::<CopyBox>(b)?),
                    None => None,
                };
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                let dloc = match d_kind {
                    Ok(sub) => CopyLoc::Subresource(d_res.as_ref(), sub),
                    Err(fp) => CopyLoc::Footprint(d_res.as_ref(), fp),
                };
                let sloc = match s_kind {
                    Ok(sub) => CopyLoc::Subresource(s_res.as_ref(), sub),
                    Err(fp) => CopyLoc::Footprint(s_res.as_ref(), fp),
                };
                l.cl.copy_texture_region(dloc, [*dst_x, *dst_y, *dst_z], sloc, sbox)?;
                l.retained_resources.push(d_res);
                l.retained_resources.push(s_res);
                Ok(Handled::Done)
            }
            Call::CopyResource { list, dst, src } => {
                let dr = self
                    .resources
                    .get(dst)
                    .cloned()
                    .ok_or_else(|| format!("CopyResource unknown dst {dst:#x}"))?;
                let sr = self
                    .resources
                    .get(src)
                    .cloned()
                    .ok_or_else(|| format!("CopyResource unknown src {src:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.copy_resource(&dr, &sr)?;
                l.retained_resources.push(dr);
                l.retained_resources.push(sr);
                Ok(Handled::Done)
            }
            Call::ResolveSubresource { list, dst, dst_sub, src, src_sub, format } => {
                let dr = self
                    .resources
                    .get(dst)
                    .cloned()
                    .ok_or_else(|| format!("ResolveSubresource unknown dst {dst:#x}"))?;
                let sr = self
                    .resources
                    .get(src)
                    .cloned()
                    .ok_or_else(|| format!("ResolveSubresource unknown src {src:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                l.cl.resolve_subresource(&dr, *dst_sub, &sr, *src_sub, *format)?;
                l.retained_resources.push(dr);
                l.retained_resources.push(sr);
                Ok(Handled::Done)
            }
            Call::BeginQuery { list, heap, query_type, index } => {
                let qh = self
                    .query_heaps
                    .get(heap)
                    .cloned()
                    .ok_or_else(|| format!("BeginQuery unknown heap {heap:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                match *query_type {
                    query::query_type::OCCLUSION => l.cl.begin_occlusion(&qh, *index, false)?,
                    query::query_type::BINARY_OCCLUSION => l.cl.begin_occlusion(&qh, *index, true)?,
                    _ => return Ok(Handled::Skipped),
                }
                l.retained_query_heaps.push(qh);
                Ok(Handled::Done)
            }
            Call::EndQuery { list, heap, query_type, index } => {
                let qh = self
                    .query_heaps
                    .get(heap)
                    .cloned()
                    .ok_or_else(|| format!("EndQuery unknown heap {heap:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                match *query_type {
                    query::query_type::TIMESTAMP => l.cl.write_timestamp(&qh, *index)?,
                    query::query_type::OCCLUSION | query::query_type::BINARY_OCCLUSION => {
                        l.cl.end_occlusion(&qh, *index)?;
                    }
                    _ => return Ok(Handled::Skipped),
                }
                l.retained_query_heaps.push(qh);
                Ok(Handled::Done)
            }
            Call::ResolveQueryData { list, heap, query_type, start, count, dst, dst_offset } => {
                let qh = self
                    .query_heaps
                    .get(heap)
                    .cloned()
                    .ok_or_else(|| format!("ResolveQueryData unknown heap {heap:#x}"))?;
                let dr = self
                    .resources
                    .get(dst)
                    .cloned()
                    .ok_or_else(|| format!("ResolveQueryData unknown dst {dst:#x}"))?;
                let l = self
                    .lists
                    .get_mut(list)
                    .ok_or_else(|| format!("unknown command list {list:#x}"))?;
                match *query_type {
                    query::query_type::TIMESTAMP => l.cl.resolve_timestamps(&qh, *start, *count, &dr, *dst_offset)?,
                    query::query_type::OCCLUSION | query::query_type::BINARY_OCCLUSION => {
                        l.cl.resolve_occlusion(&qh, *start, *count, &dr, *dst_offset)?;
                    }
                    _ => return Ok(Handled::Skipped),
                }
                l.retained_query_heaps.push(qh);
                l.retained_resources.push(dr);
                Ok(Handled::Done)
            }
            Call::BuildRaytracingAccelerationStructure { .. }
            | Call::EmitRaytracingAccelerationStructurePostbuildInfo { .. }
            | Call::CopyRaytracingAccelerationStructure { .. }
            | Call::SetPipelineState1 { .. }
            | Call::DispatchRays { .. } => Ok(Handled::Skipped),
            Call::Marker { .. } | Call::FrameEnd { .. } => Ok(Handled::Done),
            Call::ExecuteCommandLists { queue, lists } => {
                let q = self
                    .queues
                    .get(queue)
                    .cloned()
                    .ok_or_else(|| format!("ExecuteCommandLists unknown queue {queue:#x}"))?;
                let ids = lists.to_vec();
                // Validate list existence, uniqueness within this batch, and repeated-submission
                // timing soundness before removing any list from `self.lists`.
                let mut batch_seen: HashSet<u64> = HashSet::with_capacity(ids.len());
                for &id in &ids {
                    if !batch_seen.insert(id) {
                        return Err(format!("ExecuteCommandLists duplicate list {id:#x} in single batch"));
                    }
                    let l = self
                        .lists
                        .get(&id)
                        .ok_or_else(|| format!("ExecuteCommandLists unknown list {id:#x}"))?;
                    if self.timing_armed
                        && matches!(l.cl.kind, ListType::Direct | ListType::Compute | ListType::Copy)
                    {
                        if l.submitted_since_reset {
                            return Err(format!(
                                "cannot benchmark repeated submission of closed command list {id:#x} \
                                 without Reset (re-executing baked timestamp writes would overwrite slots \
                                 and understate gpu_busy_ms)"
                            ));
                        }
                        if l.closed_ts_pair.is_none() {
                            return Err(format!(
                                "cannot benchmark command list {id:#x} recorded while GPU timing was disarmed"
                            ));
                        }
                    }
                }
                // Extract command lists in submission order, execute as one batch, then restore.
                let mut taken: Vec<(u64, LiveCommandList)> = Vec::with_capacity(ids.len());
                for id in ids {
                    let l = self.lists.remove(&id).unwrap();
                    taken.push((id, l));
                }
                // Refresh any relocated indirect argument scratch buffers from their source
                // upload buffers right before submission so captured `WriteResource` updates
                // emitted via `tr::sync_uploads()` after `ExecuteIndirect` are reflected.
                for (_, l) in &taken {
                    for reloc in &l.pending_indirect_relocs {
                        if let Some(src_res) = self.resources.get(&reloc.src_res_id) {
                            if let Some(src_bytes) = src_res.shared_slice(reloc.src_offset, reloc.args_len) {
                                self.write_relocated_indirect_args(
                                    &reloc.scratch,
                                    &reloc.sig,
                                    reloc.max_count,
                                    src_bytes,
                                )?;
                            }
                        }
                    }
                }
                {
                    let mut refs: Vec<&mut CommandList> = taken.iter_mut().map(|(_, l)| &mut l.cl).collect();
                    q.execute_command_lists(&mut refs);
                }
                self.queue_busy.insert(*queue);
                for (id, mut l) in taken {
                    if self.timing_armed {
                        if let Some(pair) = l.closed_ts_pair.take() {
                            if let Some(timer) = self.gpu_timer.as_mut() {
                                timer.pairs.push(pair);
                            }
                        }
                    }
                    l.submitted_since_reset = true;
                    self.lists.insert(id, l);
                }
                Ok(Handled::Done)
            }
            Call::QueueSignal { queue, fence, value } => {
                let q = self
                    .queues
                    .get(queue)
                    .ok_or_else(|| format!("QueueSignal unknown queue {queue:#x}"))?;
                let f = self
                    .fences
                    .get(fence)
                    .ok_or_else(|| format!("QueueSignal unknown fence {fence:#x}"))?;
                q.signal(f, *value);
                self.fence_signals.entry(*fence).or_default().push((*value, Some(*queue)));
                Ok(Handled::Done)
            }
            Call::QueueWait { queue, fence, value } => {
                let q = self
                    .queues
                    .get(queue)
                    .ok_or_else(|| format!("QueueWait unknown queue {queue:#x}"))?;
                let f = self
                    .fences
                    .get(fence)
                    .ok_or_else(|| format!("QueueWait unknown fence {fence:#x}"))?;
                q.wait(f, *value);
                self.queue_waits.entry(*queue).or_default().push((*fence, *value));
                Ok(Handled::Done)
            }
            Call::FenceSignal { fence, value } => {
                let f = self
                    .fences
                    .get(fence)
                    .ok_or_else(|| format!("FenceSignal unknown fence {fence:#x}"))?;
                f.signal_cpu(*value);
                self.fence_signals.entry(*fence).or_default().push((*value, None));
                Ok(Handled::Done)
            }
            Call::CreateSwapChain { queue, swapchain, desc } => {
                let q = self
                    .queues
                    .get(queue)
                    .cloned()
                    .ok_or_else(|| format!("CreateSwapChain unknown queue {queue:#x}"))?;
                let (w, h, fmt, count) = if desc.len() >= 32 {
                    let w = u32::from_le_bytes(desc[0..4].try_into().unwrap());
                    let h = u32::from_le_bytes(desc[4..8].try_into().unwrap());
                    let fmt = u32::from_le_bytes(desc[8..12].try_into().unwrap());
                    let count = u32::from_le_bytes(desc[28..32].try_into().unwrap());
                    (w.max(1), h.max(1), if fmt != 0 { fmt } else { 28 }, count.max(2))
                } else {
                    (256, 256, 28, 2)
                };
                let sc = unsafe {
                    Swapchain::new(std::ptr::null_mut(), q, w, h, fmt, count, &|f| f())?
                };
                self.swapchains.insert(*swapchain, sc);
                self.swapchain_queues.insert(*swapchain, *queue);
                Ok(Handled::Done)
            }
            Call::GetSwapChainBuffer { swapchain, index, resource } => {
                let sc = self
                    .swapchains
                    .get(swapchain)
                    .ok_or_else(|| format!("GetSwapChainBuffer unknown swapchain {swapchain:#x}"))?;
                let buf = sc
                    .buffers
                    .get(*index as usize)
                    .cloned()
                    .ok_or_else(|| format!("swapchain buffer index {index} out of range ({})", sc.buffers.len()))?;
                if let Some(existing) = self.resources.get(resource) {
                    if !Arc::ptr_eq(existing, &buf) {
                        return Err(format!(
                            "GetSwapChainBuffer: duplicate still-live resource id {resource:#x}"
                        ));
                    }
                } else {
                    self.resources.insert(*resource, buf);
                }
                Ok(Handled::Done)
            }
            Call::ResizeBuffers { swapchain, count, width, height, format, .. } => {
                let sc = self
                    .swapchains
                    .get_mut(swapchain)
                    .ok_or_else(|| format!("ResizeBuffers unknown swapchain {swapchain:#x}"))?;
                let new_count = if *count != 0 { *count } else { sc.buffers.len() as u32 };
                let new_w = if *width != 0 { *width } else { sc.width };
                let new_h = if *height != 0 { *height } else { sc.height };
                let new_fmt = if *format != 0 { *format } else { sc.format };
                sc.resize(new_w, new_h, new_fmt, new_count, &|f| f())?;
                Ok(Handled::Done)
            }
            Call::Present { swapchain, sync_interval, .. } => {
                let sc = self
                    .swapchains
                    .get_mut(swapchain)
                    .ok_or_else(|| format!("Present unknown swapchain {swapchain:#x}"))?;
                if let Some(bb) = sc.buffers.get(sc.current).cloned() {
                    self.last_presented = Some(bb);
                }
                let si = if self.disable_vsync { 0 } else { *sync_interval };
                sc.present(si)?;
                if let Some(&qid) = self.swapchain_queues.get(swapchain) {
                    self.queue_busy.insert(qid);
                }
                Ok(Handled::Done)
            }
        }
    }

    fn finish(&mut self) -> Result<(), String> {
        let blocked = self.blocked_queues();
        if !blocked.is_empty() {
            self.drain_unblocked_queues();
            return Err(format!(
                "cannot finish GPU replay: queue(s) {blocked:?} remain blocked on unsignaled fence waits"
            ));
        }
        self.wait_all_idle();
        Ok(())
    }
}

impl Drop for MetalBackend {
    fn drop(&mut self) {
        // Release any blocked cross-queue waits via CPU signal before draining so dropping
        // a rejected backend after an early error never hangs waiting on an unsignaled fence.
        if !self.blocked_queues().is_empty() {
            for waits in self.queue_waits.values() {
                for &(fid, val) in waits {
                    if let Some(f) = self.fences.get(&fid) {
                        if f.completed() < val {
                            f.signal_cpu(val);
                        }
                    }
                }
            }
        }
        self.wait_all_idle();
    }
}

fn bind_pso(cl: &mut CommandList, pso: LivePso) {
    match pso {
        LivePso::Graphics(p) => cl.set_graphics_pipeline(p),
        LivePso::Mesh(p) => cl.set_mesh_pipeline(p),
        LivePso::Compute(p) => cl.set_compute_pipeline(p),
    }
}

/// Compute the byte offset within each indirect command stride of every 64-bit GPU virtual
/// address field (`VERTEX_BUFFER_VIEW`, `INDEX_BUFFER_VIEW`, `CONSTANT_BUFFER_VIEW`,
/// `SHADER_RESOURCE_VIEW`, `UNORDERED_ACCESS_VIEW`).
pub(crate) fn indirect_va_offsets(args: &[ArgumentDesc]) -> Vec<u32> {
    let mut off = 0u32;
    let mut out = Vec::new();
    for &(ty, u) in args {
        match ty {
            indirect_arg::DRAW => off = off.saturating_add(16),
            indirect_arg::DRAW_INDEXED => off = off.saturating_add(20),
            indirect_arg::DISPATCH | indirect_arg::DISPATCH_MESH => off = off.saturating_add(12),
            indirect_arg::CONSTANT => off = off.saturating_add(4u32.saturating_mul(u[2])),
            indirect_arg::CONSTANT_BUFFER_VIEW
            | indirect_arg::SHADER_RESOURCE_VIEW
            | indirect_arg::UNORDERED_ACCESS_VIEW => {
                out.push(off);
                off = off.saturating_add(8);
            }
            indirect_arg::INCREMENTING_CONSTANT => {}
            indirect_arg::VERTEX_BUFFER_VIEW | indirect_arg::INDEX_BUFFER_VIEW => {
                out.push(off);
                off = off.saturating_add(16);
            }
            _ => {}
        }
    }
    out
}

fn decode_target_words(desc: Option<&[u8]>) -> Result<Option<[u32; 6]>, String> {
    let Some(b) = desc else { return Ok(None) };
    if b.is_empty() {
        return Ok(None);
    }
    let mut words = [0u32; 6];
    for i in 0..6 {
        if (i + 1) * 4 <= b.len() {
            words[i] = u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
        }
    }
    Ok(Some(words))
}

fn decode_fixed_state(d: &TraceGfxPso) -> Result<(BlendDesc, state::RasterState, state::DepthStencilState), String> {
    let blend = pod::read::<BlendDesc>(d.blend)?;
    let raster = match d.raster_version {
        0 => state::raster_state(&pod::read::<RasterizerDesc>(d.rasterizer)?),
        1 => state::raster_state1(&pod::read::<RasterizerDesc1>(d.rasterizer)?),
        2 => state::raster_state2(&pod::read::<RasterizerDesc2>(d.rasterizer)?),
        v => return Err(format!("unknown rasterizer desc version {v}")),
    };
    let depth_stencil = match d.depth_stencil_version {
        0 => state::depth_stencil_state(&pod::read::<DepthStencilDesc>(d.depth_stencil)?),
        1 => state::depth_stencil_state1(&pod::read::<DepthStencilDesc1>(d.depth_stencil)?),
        2 => state::depth_stencil_state2(&pod::read::<DepthStencilDesc2>(d.depth_stencil)?),
        v => return Err(format!("unknown depth-stencil desc version {v}")),
    };
    Ok((blend, raster, depth_stencil))
}

fn decode_graphics_desc<'a>(d: &'a TraceGfxPso<'a>) -> Result<GraphicsDesc<'a>, String> {
    let (blend, raster, depth_stencil) = decode_fixed_state(d)?;
    let input_layout = if d.input_layout.is_empty() {
        Vec::new()
    } else {
        d3d12_metal::trace::decode_input_layout(d.input_layout)
            .map_err(|e| e.0)?
            .into_iter()
            .map(|e| InputElement {
                semantic_name: e.semantic_name,
                semantic_index: e.semantic_index,
                format: e.format,
                input_slot: e.input_slot,
                aligned_byte_offset: e.aligned_byte_offset,
                input_slot_class: e.input_slot_class,
                instance_data_step_rate: e.instance_data_step_rate,
            })
            .collect()
    };
    Ok(GraphicsDesc {
        vs: d.vs,
        ps: (!d.ps.is_empty()).then_some(d.ps),
        blend,
        sample_mask: d.sample_mask,
        raster,
        depth_stencil,
        input_layout,
        strip_cut: d.ib_strip_cut_value,
        topology_type: d.primitive_topology_type,
        rtv_formats: d.rtv_formats.to_vec(),
        dsv_format: d.dsv_format,
        sample_count: d.sample_count.max(1),
    })
}

fn decode_mesh_desc<'a>(amp: &'a [u8], mesh: &'a [u8], d: &'a TraceGfxPso<'a>) -> Result<MeshDesc<'a>, String> {
    let (blend, raster, depth_stencil) = decode_fixed_state(d)?;
    Ok(MeshDesc {
        amplification: (!amp.is_empty()).then_some(amp),
        mesh,
        pixel: (!d.ps.is_empty()).then_some(d.ps),
        blend,
        sample_mask: d.sample_mask,
        raster,
        depth_stencil,
        rtv_formats: d.rtv_formats.to_vec(),
        dsv_format: d.dsv_format,
        sample_count: d.sample_count.max(1),
    })
}

fn read_copy<T: Copy>(b: &[u8]) -> Result<T, String> {
    let want = std::mem::size_of::<T>();
    if b.len() != want {
        return Err(format!("{}: {} bytes, expected {want}", std::any::type_name::<T>(), b.len()));
    }
    Ok(unsafe { std::ptr::read_unaligned(b.as_ptr().cast::<T>()) })
}

fn read_copy_prefix<T: Copy>(b: &[u8]) -> Result<T, String> {
    let want = std::mem::size_of::<T>();
    let head = b
        .get(..want)
        .ok_or_else(|| format!("{}: {} bytes, expected at least {want}", std::any::type_name::<T>(), b.len()))?;
    read_copy(head)
}

fn read_copy_array<T: Copy>(b: &[u8]) -> Result<Vec<T>, String> {
    let n = std::mem::size_of::<T>();
    if !b.len().is_multiple_of(n) {
        return Err(format!("{}[]: {} bytes is not a multiple of {n}", std::any::type_name::<T>(), b.len()));
    }
    b.chunks_exact(n).map(read_copy).collect()
}
