//! `ID3D11DeviceContext` (115 base slots) / `ID3D11DeviceContext1` (134 total slots):
//! immediate Metal 4 execution and deferred command-list recording/replay.

use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::clear::ClearValue;
use d3d12_metal::metal::cmdlist::{Barrier, CommandAllocator, CommandList, CopyBox, CopyLoc, TargetView};
use d3d12_metal::metal::descriptor::{bump_desc_heap_gen, heap_kind, DescriptorHeap, RESOURCE_DESCRIPTOR_SIZE};
use d3d12_metal::metal::indirect::{IndirectLayout, Work};
use d3d12_metal::metal::pipeline::{GraphicsDesc, GraphicsPipeline, MAX_VERTEX_SLOTS};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::metal::views;
use d3d12_metal::types::{dim, heap_type, HeapProperties, Rect, ResourceDesc};

use crate::types::*;

use super::com::*;
use super::device::{DeviceCore, DynamicBufPool};
use super::resources::*;
use super::state::*;
use super::types::*;

/// Exact Windows x64 `ID3D11DeviceContext` base vtable length (slots 0..114).
pub const ID3D11_CONTEXT_BASE_SLOTS: usize = 115;
/// Exact Windows x64 `ID3D11DeviceContext1` total vtable length (slots 0..133).
pub const ID3D11_CONTEXT1_TOTAL_SLOTS: usize = 134;

// ---------------------------------------------------------------------------
// RAII COM pointer holder for ContextState
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct ComRef(pub usize);

impl ComRef {
    pub unsafe fn from_borrowed(p: *mut c_void) -> Self {
        if !p.is_null() {
            com_add_ref(p);
        }
        Self(p as usize)
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.0 as *mut c_void
    }

    pub unsafe fn copy_out(&self, out: *mut *mut c_void) {
        if !out.is_null() {
            let p = self.as_ptr();
            if !p.is_null() {
                com_add_ref(p);
            }
            std::ptr::write_unaligned(out, p);
        }
    }
}

impl Clone for ComRef {
    fn clone(&self) -> Self {
        if self.0 != 0 {
            unsafe { com_add_ref(self.0 as *mut c_void) };
        }
        Self(self.0)
    }
}

impl Drop for ComRef {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe { com_release(self.0 as *mut c_void) };
        }
    }
}

// ---------------------------------------------------------------------------
// Viewport bridge helper (no `objc2_metal` import needed)
// ---------------------------------------------------------------------------

fn push_mtl_viewport<V>(out: &mut Vec<V>, vp: &D3D11_VIEWPORT) {
    #[repr(C)]
    struct RawMtlViewport {
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        znear: f64,
        zfar: f64,
    }
    debug_assert_eq!(std::mem::size_of::<V>(), std::mem::size_of::<RawMtlViewport>());
    debug_assert_eq!(std::mem::align_of::<V>(), std::mem::align_of::<RawMtlViewport>());
    let raw = RawMtlViewport {
        origin_x: vp.TopLeftX as f64,
        origin_y: vp.TopLeftY as f64,
        width: vp.Width as f64,
        height: vp.Height as f64,
        znear: vp.MinDepth as f64,
        zfar: vp.MaxDepth as f64,
    };
    // SAFETY: `MTLViewport` is `#[repr(C)]` with 6 `f64` fields in identical order.
    out.push(unsafe { std::mem::transmute_copy(&raw) });
}

fn d3d11_topology_to_type(topo: u32) -> u32 {
    use d3d12_metal::topology::{d3d, topology_type};
    match topo {
        d3d::POINTLIST => topology_type::POINT,
        d3d::LINELIST | d3d::LINESTRIP | d3d::LINELIST_ADJ | d3d::LINESTRIP_ADJ => topology_type::LINE,
        d3d::PATCHLIST_1..=d3d::PATCHLIST_32 => topology_type::PATCH,
        _ => topology_type::TRIANGLE,
    }
}

fn d3d11_primitives_per_instance(topo: u32, count: u64) -> u64 {
    use d3d12_metal::topology::d3d;
    match topo {
        d3d::POINTLIST => count,
        d3d::LINELIST => count / 2,
        d3d::LINESTRIP => count.saturating_sub(1),
        d3d::TRIANGLELIST => count / 3,
        d3d::TRIANGLESTRIP | 6 /* TRIANGLEFAN */ => count.saturating_sub(2),
        d3d::LINELIST_ADJ => count / 4,
        d3d::LINESTRIP_ADJ => count.saturating_sub(3),
        d3d::TRIANGLELIST_ADJ => count / 6,
        d3d::TRIANGLESTRIP_ADJ => count.saturating_sub(4) / 2,
        d3d::PATCHLIST_1..=d3d::PATCHLIST_32 => count / (topo - d3d::PATCHLIST_1 + 1) as u64,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Immediate Metal 4 execution backend (owned by DeviceCore)
// ---------------------------------------------------------------------------

const ALLOC_RING: usize = 4;
const HEAP_RING: usize = 4;
const RES_HEAP_CAP: u32 = if cfg!(target_os = "ios") { 65_536 } else { 262_144 };
const SAMP_HEAP_CAP: u32 = if cfg!(target_os = "ios") { 8_192 } else { 16_384 };
const UPLOAD_CHUNK_SIZE: usize = if cfg!(target_os = "ios") { 1 << 20 } else { 4 << 20 };
const MAX_READY_UPLOAD_CHUNKS: usize = if cfg!(target_os = "ios") { 2 } else { 6 };

/// Per-frame immediate-context counters snapshotted on `Present`.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameStats {
    pub draws: u64,
    pub flushes: u64,
    pub mid_frame_flushes: u64,
    pub mid_frame_stalls: u64,
    pub res_slots: u64,
    pub samp_slots: u64,
}

#[derive(Clone, Copy, Default)]
struct StageResCache {
    cmdbuf_epoch: u64,
    heap_epoch: u64,
    bind_gen: u64,
    dev_ver: u64,
    slots_sig: u64,
    cbv_mask: u16,
    srv_mask: u128,
    uav_mask: u8,
    gpu_va: u64,
}

#[derive(Clone, Copy, Default)]
struct StageSampCache {
    cmdbuf_epoch: u64,
    heap_epoch: u64,
    bind_gen: u64,
    samp_mask: u16,
    gpu_va: u64,
}

const STAGE_VS: usize = 0;
const STAGE_PS: usize = 1;
const STAGE_GS: usize = 2;
const STAGE_HS: usize = 3;
const STAGE_DS: usize = 4;
const STAGE_CS: usize = 5;

pub struct ImmediateExec {
    queue: Arc<Queue>,
    allocs: Vec<CommandAllocator>,
    ring_vals: [u64; ALLOC_RING],
    ring_idx: usize,
    pub(super) cl: CommandList,
    open: bool,
    pub(super) has_work: bool,
    res_heaps: Vec<DescriptorHeap>,
    res_heap_idx: usize,
    res_heap_vals: Vec<u64>,
    res_cursor: u32,
    res_heap_epoch: u64,
    samp_heaps: Vec<DescriptorHeap>,
    samp_heap_idx: usize,
    samp_heap_vals: Vec<u64>,
    samp_cursor: u32,
    samp_heap_epoch: u64,
    cmdbuf_epoch: u64,
    /// Dedicated keep-alive vectors avoid per-resource `Box<dyn Send>` heap allocations.
    keep_mtls: Vec<Arc<MtlResource>>,
    keep_psos: Vec<Arc<GraphicsPipeline>>,
    keep_alive: Vec<Box<dyn Send>>,
    /// Resident upload ring for `Map(WRITE_DISCARD)` small dynamic buffers and `UpdateSubresource`.
    cur_upload: Option<(Arc<MtlResource>, usize, usize)>,
    active_upload_chunks: Vec<Arc<MtlResource>>,
    retired_upload_chunks: VecDeque<(u64, Arc<MtlResource>)>,
    ready_upload_chunks: Vec<Arc<MtlResource>>,
    /// Dedicated dynamic-pool buffers discarded while the current command buffer had work.
    dynamic_pool: Arc<Mutex<DynamicBufPool>>,
    pending_pool_returns: Vec<(u64, Arc<MtlResource>)>,
    /// Per-stage descriptor table caches (indexed by `STAGE_*`).
    stage_res_cache: [StageResCache; 6],
    stage_samp_cache: [StageSampCache; 6],
    /// Last bound graphics/compute state on the current open command buffer.
    gfx_heaps_bound: bool,
    compute_heaps_bound: bool,
    bound_gfx_tables: [u64; 10],
    bound_cs_tables: [u64; 2],
    cached_gfx_pso: Option<Arc<CachedGfxPso>>,
    cached_gfx_indexed: bool,
    rt_synced: bool,
    rt_backings_sig: u64,
    dyn_synced: bool,
    bound_vbs_mask: u32,
    /// Scratch buffers reused across draws to avoid per-draw `Vec` allocations.
    scratch_barriers: Vec<Barrier>,
    scratch_attached: Vec<usize>,
    pending_queries: Vec<Arc<QueryInner>>,
    active_pipeline_stats: Vec<Arc<QueryInner>>,
    /// Per-frame telemetry counters.
    frame_stats: FrameStats,
}
unsafe impl Send for ImmediateExec {}

impl ImmediateExec {
    pub fn new(queue: Arc<Queue>, dynamic_pool: Arc<Mutex<DynamicBufPool>>) -> Result<Self, String> {
        let mut allocs = Vec::with_capacity(ALLOC_RING);
        for _ in 0..ALLOC_RING {
            allocs.push(CommandAllocator::new(ListType::Direct).ok_or("CommandAllocator::new failed")?);
        }
        let cl = CommandList::new(ListType::Direct).ok_or("CommandList::new failed")?;
        let res_heaps = vec![
            DescriptorHeap::new(heap_kind::CBV_SRV_UAV, RES_HEAP_CAP, true, "d3d11 res heap 0")?,
            DescriptorHeap::new(heap_kind::CBV_SRV_UAV, RES_HEAP_CAP, true, "d3d11 res heap 1")?,
        ];
        let samp_heaps = vec![
            DescriptorHeap::new(heap_kind::SAMPLER, SAMP_HEAP_CAP, true, "d3d11 samp heap 0")?,
            DescriptorHeap::new(heap_kind::SAMPLER, SAMP_HEAP_CAP, true, "d3d11 samp heap 1")?,
        ];
        Ok(Self {
            queue,
            allocs,
            ring_vals: [0; ALLOC_RING],
            ring_idx: 0,
            cl,
            open: false,
            has_work: false,
            res_heaps,
            res_heap_idx: 0,
            res_heap_vals: vec![0; 2],
            res_cursor: 1, // Slot 0 reserved as zeroed dummy descriptor.
            res_heap_epoch: 1,
            samp_heaps,
            samp_heap_idx: 0,
            samp_heap_vals: vec![0; 2],
            samp_cursor: 1, // Slot 0 reserved as zeroed dummy descriptor.
            samp_heap_epoch: 1,
            cmdbuf_epoch: 1,
            keep_mtls: Vec::with_capacity(64),
            keep_psos: Vec::with_capacity(16),
            keep_alive: Vec::new(),
            cur_upload: None,
            active_upload_chunks: Vec::new(),
            retired_upload_chunks: VecDeque::new(),
            ready_upload_chunks: Vec::new(),
            dynamic_pool,
            pending_pool_returns: Vec::new(),
            stage_res_cache: [StageResCache::default(); 6],
            stage_samp_cache: [StageSampCache::default(); 6],
            gfx_heaps_bound: false,
            compute_heaps_bound: false,
            bound_gfx_tables: [u64::MAX; 10],
            bound_cs_tables: [u64::MAX; 2],
            cached_gfx_pso: None,
            cached_gfx_indexed: false,
            rt_synced: false,
            rt_backings_sig: 0,
            dyn_synced: false,
            bound_vbs_mask: 0,
            scratch_barriers: Vec::with_capacity(16),
            scratch_attached: Vec::with_capacity(10),
            pending_queries: Vec::new(),
            active_pipeline_stats: Vec::new(),
            frame_stats: FrameStats::default(),
        })
    }

    #[inline]
    pub fn invalidate_live_caches(&mut self) {
        self.cached_gfx_pso = None;
        self.rt_synced = false;
        self.dyn_synced = false;
        self.stage_res_cache = [StageResCache::default(); 6];
        self.stage_samp_cache = [StageSampCache::default(); 6];
        self.bound_gfx_tables = [u64::MAX; 10];
        self.bound_cs_tables = [u64::MAX; 2];
    }

    pub fn take_frame_stats(&mut self) -> FrameStats {
        std::mem::take(&mut self.frame_stats)
    }

    pub(super) fn ensure_open(&mut self) -> Result<(), String> {
        if self.open {
            return Ok(());
        }
        let i = self.ring_idx;
        if self.ring_vals[i] != 0 && self.queue.timeline.completed() < self.ring_vals[i] {
            self.queue.timeline.wait_cpu(self.ring_vals[i], u64::MAX);
        }
        self.allocs[i].reset();
        self.cl.begin(&self.allocs[i])?;
        self.open = true;
        self.gfx_heaps_bound = false;
        self.compute_heaps_bound = false;
        self.bound_gfx_tables = [u64::MAX; 10];
        self.bound_cs_tables = [u64::MAX; 2];
        self.rt_synced = false;
        self.dyn_synced = false;
        self.bound_vbs_mask = 0;
        Ok(())
    }

    pub fn keep<T: Send + 'static>(&mut self, obj: T) {
        self.keep_alive.push(Box::new(obj));
    }

    #[inline]
    pub fn keep_mtl(&mut self, mtl: &Arc<MtlResource>) {
        if let Some((cur, _, _)) = &self.cur_upload {
            if Arc::ptr_eq(cur, mtl) {
                if !self.active_upload_chunks.iter().any(|c| Arc::ptr_eq(c, cur)) {
                    self.active_upload_chunks.push(cur.clone());
                }
                return;
            }
        }
        if self.keep_mtls.last().is_some_and(|last| Arc::ptr_eq(last, mtl)) {
            return;
        }
        self.keep_mtls.push(mtl.clone());
    }

    #[inline]
    fn keep_pso(&mut self, pso: &Arc<GraphicsPipeline>) {
        if self.keep_psos.last().is_some_and(|last| Arc::ptr_eq(last, pso)) {
            return;
        }
        self.keep_psos.push(pso.clone());
    }

    /// Sub-allocate an `align`-aligned slice from the resident upload ring (recycled
    /// via `Queue::timeline` once the GPU finishes and no live `ResourceCore` references
    /// the chunk).
    pub(super) fn alloc_upload_slice(
        &mut self,
        len: usize,
        align: usize,
    ) -> Result<(Arc<MtlResource>, u64, *mut u8), String> {
        let align = align.max(16);
        if let Some((ref chunk, ref mut cur, cap)) = self.cur_upload {
            let off = cur.next_multiple_of(align);
            if off.saturating_add(len) <= cap {
                *cur = off + len;
                let cpu = chunk.cpu_ptr().ok_or("upload ring chunk has no cpu_ptr")?;
                if !self.active_upload_chunks.iter().any(|c| Arc::ptr_eq(c, chunk)) {
                    self.active_upload_chunks.push(chunk.clone());
                }
                return Ok((chunk.clone(), off as u64, unsafe { cpu.add(off) }));
            }
        }

        // Reclaim completed chunks no longer referenced by any live ResourceCore.
        let completed = self.queue.timeline.completed();
        let mut remaining = VecDeque::new();
        while let Some((tv, chunk)) = self.retired_upload_chunks.pop_front() {
            if tv <= completed {
                if Arc::strong_count(&chunk) == 1 && self.ready_upload_chunks.len() < MAX_READY_UPLOAD_CHUNKS {
                    self.ready_upload_chunks.push(chunk);
                }
            } else {
                remaining.push_back((tv, chunk));
            }
        }
        self.retired_upload_chunks = remaining;

        let cap = len.max(UPLOAD_CHUNK_SIZE);
        let chunk = if cap == UPLOAD_CHUNK_SIZE {
            if let Some(reused) = self.ready_upload_chunks.pop() {
                reused
            } else {
                let rdesc = ResourceDesc {
                    Dimension: dim::BUFFER,
                    Width: UPLOAD_CHUNK_SIZE as u64,
                    Height: 1,
                    DepthOrArraySize: 1,
                    MipLevels: 1,
                    SampleCount: 1,
                    Layout: 1,
                    ..Default::default()
                };
                let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
                let r = Arc::new(
                    new_committed_resource(&rdesc, &hp, "d3d11 upload ring")
                        .map_err(|e| format!("{e:?}"))?,
                );
                r.register_ptr();
                r
            }
        } else {
            let rdesc = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: cap as u64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
            let r = Arc::new(
                new_committed_resource(&rdesc, &hp, "d3d11 upload large")
                    .map_err(|e| format!("{e:?}"))?,
            );
            r.register_ptr();
            r
        };

        let cpu = chunk.cpu_ptr().ok_or("upload ring chunk has no cpu_ptr")?;
        if cap == UPLOAD_CHUNK_SIZE {
            // Move previous cur_upload (if any) into active_upload_chunks so it gets
            // retired with the exact timeline value on the next flush.
            if let Some((prev, _, _)) = self.cur_upload.replace((chunk.clone(), len, cap)) {
                if !self.active_upload_chunks.iter().any(|c| Arc::ptr_eq(c, &prev)) {
                    self.active_upload_chunks.push(prev);
                }
            }
            self.active_upload_chunks.push(chunk.clone());
        } else {
            self.keep_mtl(&chunk);
        }
        Ok((chunk, 0, cpu))
    }

    pub(super) fn defer_pool_return(&mut self, class_size: u64, buf: Arc<MtlResource>) {
        if self.has_work {
            self.pending_pool_returns.push((class_size, buf));
        } else {
            let tv = self.queue.last_submitted();
            self.dynamic_pool.lock().unwrap().retire(tv, class_size, buf);
        }
    }

    pub fn flush(&mut self) -> u64 {
        if !self.open {
            return self.queue.last_submitted();
        }
        if !self.has_work {
            if let Err(e) = self.cl.close() {
                log::warn!(target: "d3d", "d3d11 empty close: {e}");
            }
            self.open = false;
            return self.queue.last_submitted();
        }
        if let Err(e) = self.cl.close() {
            log::warn!(target: "d3d", "d3d11 flush close: {e}");
            self.open = false;
            self.has_work = false;
            return self.queue.last_submitted();
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        self.frame_stats.flushes += 1;
        self.cmdbuf_epoch = self.cmdbuf_epoch.wrapping_add(1);
        let i = self.ring_idx;
        self.ring_vals[i] = v;
        self.ring_idx = (i + 1) % self.allocs.len();
        self.open = false;
        self.has_work = false;
        self.gfx_heaps_bound = false;
        self.compute_heaps_bound = false;
        self.bound_gfx_tables = [u64::MAX; 10];
        self.bound_cs_tables = [u64::MAX; 2];
        self.rt_synced = false;
        self.dyn_synced = false;

        let kept_mtls = std::mem::take(&mut self.keep_mtls);
        let kept_psos = std::mem::take(&mut self.keep_psos);
        let kept_any = std::mem::take(&mut self.keep_alive);
        if !kept_mtls.is_empty() || !kept_psos.is_empty() || !kept_any.is_empty() {
            self.queue.defer_release(Box::new((kept_mtls, kept_psos, kept_any)));
        }

        // Tag active upload chunks with this submission's timeline value.
        for chunk in self.active_upload_chunks.drain(..) {
            let is_cur = self.cur_upload.as_ref().is_some_and(|(c, _, _)| Arc::ptr_eq(c, &chunk));
            if is_cur {
                // Keep chunk alive on the queue while the current chunk continues bump-allocating
                // disjoint tail offsets in subsequent frames.
                self.queue.defer_release(Box::new(chunk));
            } else {
                self.retired_upload_chunks.push_back((v, chunk));
            }
        }
        if !self.pending_pool_returns.is_empty() {
            let mut pool = self.dynamic_pool.lock().unwrap();
            for (class_size, buf) in self.pending_pool_returns.drain(..) {
                pool.retire(v, class_size, buf);
            }
        }

        // Proactively rotate half-full descriptor heaps at frame/flush boundaries onto a
        // completed heap (zero wait_cpu) so the next frame starts cleanly at cursor 1.
        let completed = self.queue.timeline.completed();
        if self.res_cursor > RES_HEAP_CAP / 2 {
            self.res_heap_vals[self.res_heap_idx] = v;
            let next = (self.res_heap_idx + 1) % self.res_heaps.len();
            if self.res_heap_vals[next] <= completed {
                self.res_heap_idx = next;
                self.res_cursor = 1;
                self.res_heap_epoch = self.res_heap_epoch.wrapping_add(1);
            }
        }
        if self.samp_cursor > SAMP_HEAP_CAP / 2 {
            self.samp_heap_vals[self.samp_heap_idx] = v;
            let next = (self.samp_heap_idx + 1) % self.samp_heaps.len();
            if self.samp_heap_vals[next] <= completed {
                self.samp_heap_idx = next;
                self.samp_cursor = 1;
                self.samp_heap_epoch = self.samp_heap_epoch.wrapping_add(1);
            }
        }

        for q in std::mem::take(&mut self.pending_queries) {
            let mut rt = q.rt.lock().unwrap();
            rt.submitted = true;
            rt.timeline_value = v;
        }
        v
    }

    #[inline]
    fn dummy_res_gpu(&self) -> u64 {
        self.res_heaps[self.res_heap_idx].gpu_base
    }

    #[inline]
    fn dummy_samp_gpu(&self) -> u64 {
        self.samp_heaps[self.samp_heap_idx].gpu_base
    }

    fn ensure_res_space(&mut self, needed: u32) -> Result<(), String> {
        if self.res_cursor + needed <= RES_HEAP_CAP {
            return Ok(());
        }
        self.frame_stats.mid_frame_flushes += 1;
        let v = self.flush();
        self.res_heap_vals[self.res_heap_idx] = v;
        let completed = self.queue.timeline.completed();
        let next = (self.res_heap_idx + 1) % self.res_heaps.len();
        if self.res_heap_vals[next] > completed && self.res_heaps.len() < HEAP_RING {
            let idx = self.res_heaps.len();
            let h = DescriptorHeap::new(
                heap_kind::CBV_SRV_UAV,
                RES_HEAP_CAP,
                true,
                &format!("d3d11 res heap {idx}"),
            )?;
            self.res_heaps.push(h);
            self.res_heap_vals.push(0);
            self.res_heap_idx = idx;
        } else {
            self.res_heap_idx = next;
            if self.res_heap_vals[next] > completed {
                self.frame_stats.mid_frame_stalls += 1;
                self.queue.timeline.wait_cpu(self.res_heap_vals[next], u64::MAX);
            }
        }
        self.res_cursor = 1;
        self.res_heap_epoch = self.res_heap_epoch.wrapping_add(1);
        self.ensure_open()
    }

    fn ensure_samp_space(&mut self, needed: u32) -> Result<(), String> {
        if self.samp_cursor + needed <= SAMP_HEAP_CAP {
            return Ok(());
        }
        self.frame_stats.mid_frame_flushes += 1;
        let v = self.flush();
        self.samp_heap_vals[self.samp_heap_idx] = v;
        let completed = self.queue.timeline.completed();
        let next = (self.samp_heap_idx + 1) % self.samp_heaps.len();
        if self.samp_heap_vals[next] > completed && self.samp_heaps.len() < HEAP_RING {
            let idx = self.samp_heaps.len();
            let h = DescriptorHeap::new(
                heap_kind::SAMPLER,
                SAMP_HEAP_CAP,
                true,
                &format!("d3d11 samp heap {idx}"),
            )?;
            self.samp_heaps.push(h);
            self.samp_heap_vals.push(0);
            self.samp_heap_idx = idx;
        } else {
            self.samp_heap_idx = next;
            if self.samp_heap_vals[next] > completed {
                self.frame_stats.mid_frame_stalls += 1;
                self.queue.timeline.wait_cpu(self.samp_heap_vals[next], u64::MAX);
            }
        }
        self.samp_cursor = 1;
        self.samp_heap_epoch = self.samp_heap_epoch.wrapping_add(1);
        self.ensure_open()
    }

    #[inline]
    fn alloc_res_slots_unchecked(&mut self, count: u32) -> (usize, u64) {
        debug_assert!(self.res_cursor + count <= RES_HEAP_CAP);
        let h = &self.res_heaps[self.res_heap_idx];
        let off = self.res_cursor as usize * RESOURCE_DESCRIPTOR_SIZE as usize;
        self.res_cursor += count;
        self.frame_stats.res_slots += count as u64;
        (h.cpu_base + off, h.gpu_base + off as u64)
    }

    #[inline]
    fn alloc_samp_slots_unchecked(&mut self, count: u32) -> (usize, u64) {
        debug_assert!(self.samp_cursor + count <= SAMP_HEAP_CAP);
        let h = &self.samp_heaps[self.samp_heap_idx];
        let off = self.samp_cursor as usize * RESOURCE_DESCRIPTOR_SIZE as usize;
        self.samp_cursor += count;
        self.frame_stats.samp_slots += count as u64;
        (h.cpu_base + off, h.gpu_base + off as u64)
    }

    fn bind_active_heaps(&mut self) {
        let r_gpu = self.res_heaps[self.res_heap_idx].gpu_base;
        let s_gpu = self.samp_heaps[self.samp_heap_idx].gpu_base;
        self.cl.set_descriptor_heaps(r_gpu, s_gpu);
    }
}

// ---------------------------------------------------------------------------
// Pipeline state tracked per context (immediate or deferred)
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
pub struct CbSlot {
    pub com: ComRef,
    pub res: Option<Arc<ResourceCore>>,
    pub first_constant: u32,
    pub num_constants: u32,
}

#[derive(Clone, Default)]
pub struct SrvSlot {
    pub com: ComRef,
    pub srv: Option<Arc<SrvInner>>,
}

#[derive(Clone, Default)]
pub struct UavSlot {
    pub com: ComRef,
    pub uav: Option<Arc<UavInner>>,
}

#[derive(Clone, Default)]
pub struct SampSlot {
    pub com: ComRef,
    pub samp: Option<Arc<SamplerStateInner>>,
}

#[derive(Clone, Default)]
pub struct VbSlot {
    pub com: ComRef,
    pub res: Option<Arc<ResourceCore>>,
    pub stride: u32,
    pub offset: u32,
}

#[derive(Clone, Default)]
pub struct RtvSlot {
    pub com: ComRef,
    pub rtv: Option<Arc<RtvInner>>,
}

#[derive(Clone)]
pub struct StageBindings {
    pub cbs: Vec<CbSlot>,
    pub srvs: Vec<SrvSlot>,
    pub samplers: Vec<SampSlot>,
    pub res_gen: u64,
    pub samp_gen: u64,
}

impl StageBindings {
    /// Clone only the inner `Arc`s and binding metadata without calling `com_add_ref`,
    /// so deferred draw snapshots (and command-list teardown) never re-enter guest COM callbacks.
    fn snapshot(&self) -> Self {
        Self {
            cbs: self
                .cbs
                .iter()
                .map(|c| CbSlot {
                    com: ComRef(0),
                    res: c.res.clone(),
                    first_constant: c.first_constant,
                    num_constants: c.num_constants,
                })
                .collect(),
            srvs: self.srvs.iter().map(|s| SrvSlot { com: ComRef(0), srv: s.srv.clone() }).collect(),
            samplers: self
                .samplers
                .iter()
                .map(|s| SampSlot { com: ComRef(0), samp: s.samp.clone() })
                .collect(),
            res_gen: self.res_gen,
            samp_gen: self.samp_gen,
        }
    }
}

impl Default for StageBindings {
    fn default() -> Self {
        Self {
            cbs: vec![CbSlot::default(); D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT],
            srvs: vec![SrvSlot::default(); D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT],
            samplers: vec![SampSlot::default(); D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT],
            res_gen: 1,
            samp_gen: 1,
        }
    }
}

#[derive(Clone)]
pub struct ContextState {
    pub vs_com: ComRef,
    pub vs: Option<Arc<ShaderBlob>>,
    pub hs_com: ComRef,
    pub hs: Option<Arc<ShaderBlob>>,
    pub ds_com: ComRef,
    pub ds: Option<Arc<ShaderBlob>>,
    pub gs_com: ComRef,
    pub gs: Option<Arc<ShaderBlob>>,
    pub ps_com: ComRef,
    pub ps: Option<Arc<ShaderBlob>>,
    pub cs_com: ComRef,
    pub cs: Option<Arc<ComputeShaderInner>>,

    pub vs_bind: StageBindings,
    pub ps_bind: StageBindings,
    pub gs_bind: StageBindings,
    pub hs_bind: StageBindings,
    pub ds_bind: StageBindings,
    pub cs_bind: StageBindings,
    pub cs_uavs: Vec<UavSlot>,
    pub cs_uavs_gen: u64,
    pub om_uavs: Vec<UavSlot>,
    pub om_uavs_gen: u64,

    pub input_layout_com: ComRef,
    pub input_layout: Option<Arc<InputLayoutInner>>,
    pub vbs: Vec<VbSlot>,
    pub ib_com: ComRef,
    pub ib: Option<Arc<ResourceCore>>,
    pub ib_format: u32,
    pub ib_offset: u32,
    pub topology: u32,

    pub rtvs: Vec<RtvSlot>,
    pub dsv_com: ComRef,
    pub dsv: Option<Arc<DsvInner>>,

    pub blend_com: ComRef,
    pub blend: Option<Arc<BlendStateInner>>,
    pub blend_factor: [f32; 4],
    pub sample_mask: u32,

    pub ds_state_com: ComRef,
    pub ds_state: Option<Arc<DepthStencilStateInner>>,
    pub stencil_ref: u32,

    pub rast_com: ComRef,
    pub rast: Option<Arc<RasterizerStateInner>>,
    pub viewports: Vec<D3D11_VIEWPORT>,
    pub scissors: Vec<Rect>,

    pub so_targets: Vec<ComRef>,
    pub predicate_com: ComRef,
    pub predicate_val: BOOL,

    // Dirty flags for immediate-context fast-path state diffing.
    pub pso_dirty: bool,
    pub rt_dirty: bool,
    pub vp_dirty: bool,
    pub scissor_dirty: bool,
    pub blend_factor_dirty: bool,
    pub stencil_ref_dirty: bool,
}

impl Default for ContextState {
    fn default() -> Self {
        Self {
            vs_com: ComRef::default(),
            vs: None,
            hs_com: ComRef::default(),
            hs: None,
            ds_com: ComRef::default(),
            ds: None,
            gs_com: ComRef::default(),
            gs: None,
            ps_com: ComRef::default(),
            ps: None,
            cs_com: ComRef::default(),
            cs: None,
            vs_bind: StageBindings::default(),
            ps_bind: StageBindings::default(),
            gs_bind: StageBindings::default(),
            hs_bind: StageBindings::default(),
            ds_bind: StageBindings::default(),
            cs_bind: StageBindings::default(),
            cs_uavs: vec![UavSlot::default(); D3D11_PS_CS_UAV_REGISTER_COUNT],
            cs_uavs_gen: 1,
            om_uavs: vec![UavSlot::default(); D3D11_PS_CS_UAV_REGISTER_COUNT],
            om_uavs_gen: 1,
            input_layout_com: ComRef::default(),
            input_layout: None,
            vbs: vec![VbSlot::default(); D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT],
            ib_com: ComRef::default(),
            ib: None,
            ib_format: 0,
            ib_offset: 0,
            topology: topology::UNDEFINED,
            rtvs: vec![RtvSlot::default(); D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT],
            dsv_com: ComRef::default(),
            dsv: None,
            blend_com: ComRef::default(),
            blend: None,
            blend_factor: [1.0; 4],
            sample_mask: u32::MAX,
            ds_state_com: ComRef::default(),
            ds_state: None,
            stencil_ref: 0,
            rast_com: ComRef::default(),
            rast: None,
            viewports: Vec::new(),
            scissors: Vec::new(),
            so_targets: vec![ComRef::default(); D3D11_SO_BUFFER_SLOT_COUNT],
            predicate_com: ComRef::default(),
            predicate_val: 0,
            pso_dirty: true,
            rt_dirty: true,
            vp_dirty: true,
            scissor_dirty: true,
            blend_factor_dirty: true,
            stencil_ref_dirty: true,
        }
    }
}

impl ContextState {
    #[inline]
    fn clear_gfx_dirty(&mut self) {
        self.pso_dirty = false;
        self.rt_dirty = false;
        self.vp_dirty = false;
        self.scissor_dirty = false;
        self.blend_factor_dirty = false;
        self.stencil_ref_dirty = false;
    }

    /// Snapshot execution-only state for deferred `Draw*`/`Dispatch*` recording without
    /// touching COM refcounts (`ComRef` stays null), so recording and dropping command lists
    /// never invoke guest `IUnknown` callbacks (`SetPrivateDataInterface`).
    fn snapshot_for_draw(&self) -> Self {
        Self {
            vs_com: ComRef(0),
            vs: self.vs.clone(),
            hs_com: ComRef(0),
            hs: self.hs.clone(),
            ds_com: ComRef(0),
            ds: self.ds.clone(),
            gs_com: ComRef(0),
            gs: self.gs.clone(),
            ps_com: ComRef(0),
            ps: self.ps.clone(),
            cs_com: ComRef(0),
            cs: self.cs.clone(),
            vs_bind: self.vs_bind.snapshot(),
            ps_bind: self.ps_bind.snapshot(),
            gs_bind: self.gs_bind.snapshot(),
            hs_bind: self.hs_bind.snapshot(),
            ds_bind: self.ds_bind.snapshot(),
            cs_bind: self.cs_bind.snapshot(),
            cs_uavs: self.cs_uavs.iter().map(|u| UavSlot { com: ComRef(0), uav: u.uav.clone() }).collect(),
            cs_uavs_gen: self.cs_uavs_gen,
            om_uavs: self.om_uavs.iter().map(|u| UavSlot { com: ComRef(0), uav: u.uav.clone() }).collect(),
            om_uavs_gen: self.om_uavs_gen,
            input_layout_com: ComRef(0),
            input_layout: self.input_layout.clone(),
            vbs: self
                .vbs
                .iter()
                .map(|v| VbSlot { com: ComRef(0), res: v.res.clone(), stride: v.stride, offset: v.offset })
                .collect(),
            ib_com: ComRef(0),
            ib: self.ib.clone(),
            ib_format: self.ib_format,
            ib_offset: self.ib_offset,
            topology: self.topology,
            rtvs: self.rtvs.iter().map(|r| RtvSlot { com: ComRef(0), rtv: r.rtv.clone() }).collect(),
            dsv_com: ComRef(0),
            dsv: self.dsv.clone(),
            blend_com: ComRef(0),
            blend: self.blend.clone(),
            blend_factor: self.blend_factor,
            sample_mask: self.sample_mask,
            ds_state_com: ComRef(0),
            ds_state: self.ds_state.clone(),
            stencil_ref: self.stencil_ref,
            rast_com: ComRef(0),
            rast: self.rast.clone(),
            viewports: self.viewports.clone(),
            scissors: self.scissors.clone(),
            so_targets: Vec::new(),
            predicate_com: ComRef(0),
            predicate_val: self.predicate_val,
            pso_dirty: true,
            rt_dirty: true,
            vp_dirty: true,
            scissor_dirty: true,
            blend_factor_dirty: true,
            stencil_ref_dirty: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Recorded commands for Deferred Contexts / ID3D11CommandList
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum RecordedCmd {
    ClearRtv { rtv: Arc<RtvInner>, color: [f32; 4], rects: Vec<Rect> },
    ClearDsv { dsv: Arc<DsvInner>, flags: u32, depth: f32, stencil: u8, rects: Vec<Rect> },
    ClearUavUint { uav: Arc<UavInner>, values: [u32; 4] },
    ClearUavFloat { uav: Arc<UavInner>, values: [f32; 4] },
    CopyResource { dst: Arc<ResourceCore>, src: Arc<ResourceCore> },
    CopySubresourceRegion {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        dst_xyz: [u32; 3],
        src: Arc<ResourceCore>,
        src_sub: u32,
        src_box: Option<D3D11_BOX>,
    },
    UpdateSubresource {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        dst_box: Option<D3D11_BOX>,
        data: Vec<u8>,
        src_row_pitch: u32,
        src_depth_pitch: u32,
    },
    ResolveSubresource {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        src: Arc<ResourceCore>,
        src_sub: u32,
        format: u32,
    },
    Draw { state: Box<ContextState>, vertex_count: u32, instance_count: u32, start_vertex: u32, start_instance: u32 },
    DrawIndexed {
        state: Box<ContextState>,
        index_count: u32,
        instance_count: u32,
        start_index: u32,
        base_vertex: i32,
        start_instance: u32,
    },
    DrawInstancedIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    DrawIndexedInstancedIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    Dispatch { state: Box<ContextState>, x: u32, y: u32, z: u32 },
    DispatchIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    CopyStructureCount { dst: Arc<ResourceCore>, dst_offset: u32, uav: Arc<UavInner> },
    SetUavCounter { uav: Arc<UavInner>, value: u32 },
    GenerateMips { srv: Arc<SrvInner> },
    BeginQuery(Arc<QueryInner>),
    EndQuery(Arc<QueryInner>),
    DiscardResource(Arc<ResourceCore>),
}

// ---------------------------------------------------------------------------
// Immediate execution helpers
// ---------------------------------------------------------------------------

#[inline]
unsafe fn write_desc_entry(cpu_base: usize, idx: usize, entry: views::Entry) {
    let dst = (cpu_base + idx * RESOURCE_DESCRIPTOR_SIZE as usize) as *mut u8;
    std::ptr::copy_nonoverlapping(entry.as_ptr(), dst, entry.len());
}

#[inline]
fn mix_sig(h: &mut u64, v: u64) {
    *h ^= v;
    *h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(27);
}

fn compute_stage_slots_sig(
    bind: &StageBindings,
    uavs: Option<&[UavSlot]>,
    usage: StageUsage,
) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mut m = usage.cbv_mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        m &= m - 1;
        let cb = &bind.cbs[i];
        let v = cb.res.as_ref().map_or(0, |r| {
            ((Arc::as_ptr(r) as u64) << 16) ^ r.version.load(Ordering::Acquire)
        });
        mix_sig(&mut h, (i as u64) | (v << 4) ^ ((cb.first_constant as u64) << 32) ^ (cb.num_constants as u64));
    }
    let mut sm = usage.srv_mask;
    while sm != 0 {
        let i = sm.trailing_zeros() as usize;
        sm &= sm - 1;
        let v = bind.srvs[i].srv.as_ref().map_or(0, |s| {
            ((Arc::as_ptr(s) as u64) << 16) ^ s.resource.version.load(Ordering::Acquire)
        });
        mix_sig(&mut h, ((i + 14) as u64) ^ v);
    }
    if let Some(uavs) = uavs {
        let mut um = usage.uav_mask;
        while um != 0 {
            let i = um.trailing_zeros() as usize;
            um &= um - 1;
            let v = uavs[i].uav.as_ref().map_or(0, |u| {
                ((Arc::as_ptr(u) as u64) << 16) ^ u.resource.version.load(Ordering::Acquire)
            });
            mix_sig(&mut h, ((i + 142) as u64) ^ v);
        }
    }
    h
}

unsafe fn populate_stage_table_masked(
    exec: &mut ImmediateExec,
    cpu_base: usize,
    bind: &StageBindings,
    uavs: Option<&[UavSlot]>,
    usage: StageUsage,
) {
    let active_count =
        usage.cbv_mask.count_ones() + usage.srv_mask.count_ones() + usage.uav_mask.count_ones();
    if usage.res_table_len > active_count {
        std::ptr::write_bytes(
            cpu_base as *mut u8,
            0,
            usage.res_table_len as usize * RESOURCE_DESCRIPTOR_SIZE as usize,
        );
    }
    let mut m = usage.cbv_mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        m &= m - 1;
        let cb = &bind.cbs[i];
        let entry = if let Some(res) = &cb.res {
            let b = res.current_backing();
            let off = (cb.first_constant as u64) * 16;
            let total = (res.kind.buffer_width() as u64).min(b.mtl.desc.Width.saturating_sub(b.offset));
            let len = if cb.num_constants == 0 {
                total.saturating_sub(off)
            } else {
                ((cb.num_constants as u64) * 16).min(total.saturating_sub(off))
            };
            let e = views::cbv_entry(b.mtl.gpu_va + b.offset + off, len.min(u32::MAX as u64) as u32);
            exec.keep_mtl(&b.mtl);
            e
        } else {
            [0u8; 24]
        };
        write_desc_entry(cpu_base, i, entry);
    }
    let srv_off = RS_CBV_COUNT as usize;
    let mut sm = usage.srv_mask;
    while sm != 0 {
        let i = sm.trailing_zeros() as usize;
        sm &= sm - 1;
        let slot = &bind.srvs[i];
        let entry = if let Some(srv) = &slot.srv {
            match srv.entry() {
                Ok((mtl, e)) => {
                    exec.keep_mtl(&mtl);
                    e
                }
                Err(e) => {
                    log::warn!(target: "d3d", "d3d11 SRV slot {i}: {e}");
                    [0u8; 24]
                }
            }
        } else {
            [0u8; 24]
        };
        write_desc_entry(cpu_base, srv_off + i, entry);
    }
    if let Some(uavs) = uavs {
        let uav_off = (RS_CBV_COUNT + RS_SRV_COUNT) as usize;
        let mut um = usage.uav_mask;
        while um != 0 {
            let i = um.trailing_zeros() as usize;
            um &= um - 1;
            let slot = &uavs[i];
            let entry = if let Some(uav) = &slot.uav {
                match uav.entry() {
                    Ok((mtl, e)) => {
                        exec.keep_mtl(&mtl);
                        if let Some(counter) = &uav.counter {
                            exec.keep_mtl(counter);
                        }
                        e
                    }
                    Err(e) => {
                        log::warn!(target: "d3d", "d3d11 UAV slot {i}: {e}");
                        [0u8; 24]
                    }
                }
            } else {
                [0u8; 24]
            };
            write_desc_entry(cpu_base, uav_off + i, entry);
        }
    }
}

unsafe fn populate_sampler_table_masked(cpu_base: usize, bind: &StageBindings, usage: StageUsage) {
    if usage.samp_table_len > usage.samp_mask.count_ones() {
        std::ptr::write_bytes(
            cpu_base as *mut u8,
            0,
            usage.samp_table_len as usize * RESOURCE_DESCRIPTOR_SIZE as usize,
        );
    }
    let mut m = usage.samp_mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        m &= m - 1;
        let entry = bind.samplers[i].samp.as_ref().map_or([0u8; 24], |s| s.entry);
        write_desc_entry(cpu_base, i, entry);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn resolve_stage_res_table(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    stage_idx: usize,
    bind: &StageBindings,
    uavs: Option<&[UavSlot]>,
    uav_gen: u64,
    usage: StageUsage,
    allow_cache: bool,
) -> (u64, bool) {
    if usage.res_table_len == 0 {
        return (exec.dummy_res_gpu(), false);
    }
    let combined_gen = bind.res_gen ^ uav_gen.rotate_left(17);
    let dev_ver = device.desc_resource_version();
    if allow_cache {
        let c = exec.stage_res_cache[stage_idx];
        if c.cmdbuf_epoch == exec.cmdbuf_epoch
            && c.heap_epoch == exec.res_heap_epoch
            && c.bind_gen == combined_gen
            && (usage.cbv_mask & !c.cbv_mask) == 0
            && (usage.srv_mask & !c.srv_mask) == 0
            && (usage.uav_mask & !c.uav_mask) == 0
        {
            if c.dev_ver == dev_ver {
                return (c.gpu_va, false);
            }
            let sig = compute_stage_slots_sig(bind, uavs, usage);
            if sig == c.slots_sig {
                exec.stage_res_cache[stage_idx].dev_ver = dev_ver;
                return (c.gpu_va, false);
            }
        }
    }
    let (cpu, gpu) = exec.alloc_res_slots_unchecked(usage.res_table_len);
    populate_stage_table_masked(exec, cpu, bind, uavs, usage);
    if allow_cache {
        let sig = compute_stage_slots_sig(bind, uavs, usage);
        exec.stage_res_cache[stage_idx] = StageResCache {
            cmdbuf_epoch: exec.cmdbuf_epoch,
            heap_epoch: exec.res_heap_epoch,
            bind_gen: combined_gen,
            dev_ver,
            slots_sig: sig,
            cbv_mask: usage.cbv_mask,
            srv_mask: usage.srv_mask,
            uav_mask: usage.uav_mask,
            gpu_va: gpu,
        };
    }
    (gpu, true)
}

unsafe fn resolve_stage_samp_table(
    exec: &mut ImmediateExec,
    stage_idx: usize,
    bind: &StageBindings,
    usage: StageUsage,
    allow_cache: bool,
) -> u64 {
    if usage.samp_table_len == 0 {
        return exec.dummy_samp_gpu();
    }
    if allow_cache {
        let c = exec.stage_samp_cache[stage_idx];
        if c.cmdbuf_epoch == exec.cmdbuf_epoch
            && c.heap_epoch == exec.samp_heap_epoch
            && c.bind_gen == bind.samp_gen
            && (usage.samp_mask & !c.samp_mask) == 0
        {
            return c.gpu_va;
        }
    }
    let (cpu, gpu) = exec.alloc_samp_slots_unchecked(usage.samp_table_len);
    populate_sampler_table_masked(cpu, bind, usage);
    if allow_cache {
        exec.stage_samp_cache[stage_idx] = StageSampCache {
            cmdbuf_epoch: exec.cmdbuf_epoch,
            heap_epoch: exec.samp_heap_epoch,
            bind_gen: bind.samp_gen,
            samp_mask: usage.samp_mask,
            gpu_va: gpu,
        };
    }
    gpu
}

fn get_or_build_gfx_pso(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    indexed: bool,
    live_state: bool,
) -> Result<Arc<CachedGfxPso>, String> {
    if live_state && !st.pso_dirty && exec.cached_gfx_indexed == indexed {
        if let Some(ref cached) = exec.cached_gfx_pso {
            return Ok(cached.clone());
        }
    }
    let vs = st.vs.as_ref().ok_or("no vertex shader bound")?;
    let hs = st.hs.as_ref();
    let ds = st.ds.as_ref();
    let gs = st.gs.as_ref();
    let ps = st.ps.as_ref();

    let mut rtv_formats = [0u16; 8];
    let mut num_rtvs = 0u8;
    let mut sample_count = 1u8;
    for (i, slot) in st.rtvs.iter().enumerate() {
        if let Some(rtv) = &slot.rtv {
            rtv_formats[i] = rtv.format() as u16;
            num_rtvs = (i + 1) as u8;
            sample_count = rtv.resource.kind.sample_count().clamp(1, 255) as u8;
        }
    }
    let dsv_format = match &st.dsv {
        Some(dsv) => {
            if num_rtvs == 0 {
                sample_count = dsv.resource.kind.sample_count().clamp(1, 255) as u8;
            }
            dsv.format() as u16
        }
        None => 0,
    };

    let topology_type = d3d11_topology_to_type(st.topology) as u8;
    let strip_cut = if indexed {
        match st.ib_format {
            57 => 1, // CUT_0XFFFF
            42 => 2, // CUT_0XFFFFFFFF
            _ => 0,
        }
    } else {
        0
    };

    let key = GraphicsPsoKey {
        vs_id: vs.id,
        hs_id: hs.map_or(0, |h| h.id),
        ds_id: ds.map_or(0, |d| d.id),
        gs_id: gs.map_or(0, |g| g.id),
        ps_id: ps.map_or(0, |p| p.id),
        il_id: st.input_layout.as_ref().map_or(0, |il| il.id),
        blend_id: st.blend.as_ref().map_or(0, |b| b.id),
        sample_mask: st.sample_mask,
        raster_id: st.rast.as_ref().map_or(0, |r| r.id),
        ds_state_id: if dsv_format == 0 { 0 } else { st.ds_state.as_ref().map_or(0, |d| d.id) },
        rtv_formats,
        dsv_format,
        topology_type,
        strip_cut,
        num_rtvs,
        sample_count,
    };

    if let Some(p) = device.gfx_psos.lock().unwrap().get(&key).cloned() {
        if live_state {
            exec.cached_gfx_pso = Some(p.clone());
            exec.cached_gfx_indexed = indexed;
        }
        return Ok(p);
    }

    let blend_d12 = st
        .blend
        .as_ref()
        .map(|b| b.d3d12)
        .unwrap_or_else(|| D3D11_BLEND_DESC::default_d3d11().to_d3d12());
    let rast_mtl = match &st.rast {
        Some(r) => r.mtl,
        None => d3d12_metal::state::raster_state(&D3D11_RASTERIZER_DESC::default_d3d11().to_desc1().to_d3d12()),
    };
    let ds_mtl = match &st.ds_state {
        Some(d) => d.mtl,
        None => d3d12_metal::state::depth_stencil_state(&D3D11_DEPTH_STENCIL_DESC::default_d3d11().to_d3d12()),
    };
    let rtv_formats_u32: Vec<u32> = rtv_formats[..num_rtvs as usize].iter().map(|&f| f as u32).collect();
    let gdesc = GraphicsDesc {
        vs: &vs.bytecode,
        ps: ps.map(|p| p.bytecode.as_slice()),
        blend: blend_d12,
        sample_mask: st.sample_mask,
        raster: rast_mtl,
        depth_stencil: ds_mtl,
        input_layout: st.input_layout.as_ref().map_or_else(Vec::new, |il| il.elements.clone()),
        strip_cut: strip_cut as u32,
        topology_type: topology_type as u32,
        rtv_formats: rtv_formats_u32,
        dsv_format: dsv_format as u32,
        sample_count: sample_count as u32,
    };
    let label = format!("d3d11_gfx_{:016x}", vs.hash as u64);
    let pso = Arc::new(match (hs, ds, gs) {
        (Some(h), Some(d), g) => GraphicsPipeline::new_with_tessellation(
            device.gfx_rs.clone(),
            &gdesc,
            &h.bytecode,
            &d.bytecode,
            g.map(|gs| gs.bytecode.as_slice()),
            &label,
        )?,
        (Some(_), None, _) | (None, Some(_), _) => {
            return Err("D3D11 tessellation requires both Hull and Domain shaders to be bound".into());
        }
        (None, None, Some(g)) => {
            GraphicsPipeline::new_with_geometry(device.gfx_rs.clone(), &gdesc, &g.bytecode, &label)?
        }
        (None, None, None) => GraphicsPipeline::new(device.gfx_rs.clone(), &gdesc, &label)?,
    });
    let usage = GfxPsoUsage::from_pipeline(&pso);
    let cached = Arc::new(CachedGfxPso { pso, usage });
    device.gfx_psos.lock().unwrap().insert(key, cached.clone());
    if live_state {
        exec.cached_gfx_pso = Some(cached.clone());
        exec.cached_gfx_indexed = indexed;
    }
    Ok(cached)
}

fn sync_stage_shared(device: &DeviceCore, bind: &StageBindings) {
    for cb in &bind.cbs {
        if let Some(r) = &cb.res {
            r.sync_other_devices(device);
        }
    }
    for s in &bind.srvs {
        if let Some(srv) = &s.srv {
            srv.resource.sync_other_devices(device);
        }
    }
}

fn sync_state_shared(device: &DeviceCore, st: &ContextState) {
    if !device.has_shared_resources.load(Ordering::Acquire) {
        return;
    }
    sync_stage_shared(device, &st.vs_bind);
    sync_stage_shared(device, &st.hs_bind);
    sync_stage_shared(device, &st.ds_bind);
    sync_stage_shared(device, &st.gs_bind);
    sync_stage_shared(device, &st.ps_bind);
    sync_stage_shared(device, &st.cs_bind);
    for r in &st.rtvs {
        if let Some(rtv) = &r.rtv {
            rtv.resource.sync_other_devices(device);
        }
    }
    if let Some(dsv) = &st.dsv {
        dsv.resource.sync_other_devices(device);
    }
    for u in &st.om_uavs {
        if let Some(uav) = &u.uav {
            uav.resource.sync_other_devices(device);
        }
    }
    for u in &st.cs_uavs {
        if let Some(uav) = &u.uav {
            uav.resource.sync_other_devices(device);
        }
    }
    for vb in &st.vbs {
        if let Some(r) = &vb.res {
            r.sync_other_devices(device);
        }
    }
    if let Some(ib) = &st.ib {
        ib.sync_other_devices(device);
    }
}

fn sync_cmd_shared_resources(device: &DeviceCore, cmd: &RecordedCmd) {
    match cmd {
        RecordedCmd::ClearRtv { rtv, .. } => rtv.resource.sync_other_devices(device),
        RecordedCmd::ClearDsv { dsv, .. } => dsv.resource.sync_other_devices(device),
        RecordedCmd::ClearUavUint { uav, .. } | RecordedCmd::ClearUavFloat { uav, .. } => {
            uav.resource.sync_other_devices(device);
        }
        RecordedCmd::CopyResource { dst, src }
        | RecordedCmd::CopySubresourceRegion { dst, src, .. }
        | RecordedCmd::ResolveSubresource { dst, src, .. } => {
            dst.sync_other_devices(device);
            src.sync_other_devices(device);
        }
        RecordedCmd::UpdateSubresource { dst, .. } => dst.sync_other_devices(device),
        RecordedCmd::Draw { state, .. }
        | RecordedCmd::DrawIndexed { state, .. }
        | RecordedCmd::Dispatch { state, .. } => sync_state_shared(device, state),
        RecordedCmd::DrawInstancedIndirect { state, args, .. }
        | RecordedCmd::DrawIndexedInstancedIndirect { state, args, .. }
        | RecordedCmd::DispatchIndirect { state, args, .. } => {
            sync_state_shared(device, state);
            args.sync_other_devices(device);
        }
        RecordedCmd::CopyStructureCount { dst, uav, .. } => {
            dst.sync_other_devices(device);
            uav.resource.sync_other_devices(device);
        }
        RecordedCmd::SetUavCounter { uav, .. } => uav.resource.sync_other_devices(device),
        RecordedCmd::GenerateMips { srv } => srv.resource.sync_other_devices(device),
        RecordedCmd::DiscardResource(r) => r.sync_other_devices(device),
        RecordedCmd::BeginQuery(_) | RecordedCmd::EndQuery(_) => {}
    }
}

unsafe fn exec_recorded_cmd(device: &DeviceCore, cmd: &RecordedCmd) {
    sync_cmd_shared_resources(device, cmd);
    let mut exec = device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 ensure_open failed: {e}");
        return;
    }
    match cmd {
        RecordedCmd::ClearRtv { rtv, color, rects } => {
            match rtv.target_view() {
                Ok((mtl, tv)) => {
                    exec.keep_mtl(&mtl);
                    if let Err(e) = exec.cl.clear_rtv(&tv, *color, rects) {
                        log::warn!(target: "d3d", "d3d11 ClearRenderTargetView: {e}");
                    } else {
                        rtv.resource.last_gpu_state.store(gpu_state::RENDER_TARGET, Ordering::Release);
                        exec.has_work = true;
                    }
                }
                Err(e) => log::warn!(target: "d3d", "d3d11 ClearRenderTargetView target_view: {e}"),
            }
        }
        RecordedCmd::ClearDsv { dsv, flags, depth, stencil, rects } => {
            match dsv.target_view() {
                Ok((mtl, tv)) => {
                    exec.keep_mtl(&mtl);
                    if let Err(e) = exec.cl.clear_dsv(&tv, *flags & clear_flag::DEPTH != 0, *depth, *flags & clear_flag::STENCIL != 0, *stencil, rects) {
                        log::warn!(target: "d3d", "d3d11 ClearDepthStencilView: {e}");
                    } else {
                        dsv.resource.last_gpu_state.store(gpu_state::DEPTH_WRITE, Ordering::Release);
                        exec.has_work = true;
                    }
                }
                Err(e) => log::warn!(target: "d3d", "d3d11 ClearDepthStencilView target_view: {e}"),
            }
        }
        RecordedCmd::ClearUavUint { uav, values } => {
            let mtl = uav.resource.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.clear_uav(&mtl, Some(&uav.d3d12_desc), ClearValue::Uint(*values), &[]) {
                log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewUint: {e}");
            } else {
                exec.cl.barrier();
                uav.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
                exec.invalidate_live_caches();
                exec.has_work = true;
            }
            exec.keep_mtl(&mtl);
        }
        RecordedCmd::ClearUavFloat { uav, values } => {
            let mtl = uav.resource.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.clear_uav(&mtl, Some(&uav.d3d12_desc), ClearValue::Float(*values), &[]) {
                log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewFloat: {e}");
            } else {
                exec.cl.barrier();
                uav.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
                exec.invalidate_live_caches();
                exec.has_work = true;
            }
            exec.keep_mtl(&mtl);
        }
        RecordedCmd::CopyResource { dst, src } => {
            exec.cl.barrier();
            if dst.is_buffer() && src.is_buffer() {
                let db = dst.current_backing();
                let sb = src.current_backing();
                let len = (dst.kind.buffer_width() as u64).min(src.kind.buffer_width() as u64);
                if let Err(e) = exec.cl.copy_buffer(&db.mtl, db.offset, &sb.mtl, sb.offset, len) {
                    log::warn!(target: "d3d", "d3d11 CopyResource: {e}");
                } else {
                    exec.cl.barrier();
                    dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                    exec.has_work = true;
                }
                exec.keep_mtl(&db.mtl);
                exec.keep_mtl(&sb.mtl);
            } else {
                let d = dst.current_mtl();
                let s = src.current_mtl();
                if let Err(e) = exec.cl.copy_resource(&d, &s) {
                    log::warn!(target: "d3d", "d3d11 CopyResource: {e}");
                } else {
                    exec.cl.barrier();
                    dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                    exec.has_work = true;
                }
                exec.keep_mtl(&d);
                exec.keep_mtl(&s);
            }
        }
        RecordedCmd::CopySubresourceRegion { dst, dst_sub, dst_xyz, src, src_sub, src_box } => {
            exec.cl.barrier();
            if dst.is_buffer() && src.is_buffer() {
                let db = dst.current_backing();
                let sb = src.current_backing();
                let src_w = src.kind.buffer_width() as u64;
                let dst_w = dst.kind.buffer_width() as u64;
                let (src_off, len) = match src_box {
                    Some(b) => (b.left as u64, b.right.saturating_sub(b.left) as u64),
                    None => (0, src_w),
                };
                let dst_off = dst_xyz[0] as u64;
                let max_len = len
                    .min(src_w.saturating_sub(src_off))
                    .min(dst_w.saturating_sub(dst_off));
                if let Err(e) = exec.cl.copy_buffer(&db.mtl, db.offset + dst_off, &sb.mtl, sb.offset + src_off, max_len) {
                    log::warn!(target: "d3d", "d3d11 CopySubresourceRegion: {e}");
                } else {
                    exec.cl.barrier();
                    dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                    exec.has_work = true;
                }
                exec.keep_mtl(&db.mtl);
                exec.keep_mtl(&sb.mtl);
            } else {
                let d = dst.current_mtl();
                let s = src.current_mtl();
                let cbox = src_box.map(|b| CopyBox {
                    left: b.left,
                    top: b.top,
                    front: b.front,
                    right: b.right,
                    bottom: b.bottom,
                    back: b.back,
                });
                if let Err(e) = exec.cl.copy_texture_region(
                    CopyLoc::Subresource(&d, *dst_sub),
                    *dst_xyz,
                    CopyLoc::Subresource(&s, *src_sub),
                    cbox,
                ) {
                    log::warn!(target: "d3d", "d3d11 CopySubresourceRegion: {e}");
                } else {
                    exec.cl.barrier();
                    dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                    exec.has_work = true;
                }
                exec.keep_mtl(&d);
                exec.keep_mtl(&s);
            }
        }
        RecordedCmd::UpdateSubresource { dst, dst_sub, dst_box, data, src_row_pitch, src_depth_pitch } => {
            if let Err(e) = exec_update_subresource_raw(
                &mut exec,
                dst,
                *dst_sub,
                *dst_box,
                data.as_ptr(),
                Some(data.len()),
                *src_row_pitch,
                *src_depth_pitch,
            ) {
                log::warn!(target: "d3d", "d3d11 UpdateSubresource: {e}");
            }
        }
        RecordedCmd::ResolveSubresource { dst, dst_sub, src, src_sub, format } => {
            let d = dst.current_mtl();
            let s = src.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.resolve_subresource(&d, *dst_sub, &s, *src_sub, *format) {
                log::warn!(target: "d3d", "d3d11 ResolveSubresource: {e}");
            } else {
                exec.cl.barrier();
                dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                exec.has_work = true;
            }
            exec.keep_mtl(&d);
            exec.keep_mtl(&s);
        }
        RecordedCmd::Draw { state, vertex_count, instance_count, start_vertex, start_instance } => {
            if let Err(e) = exec_draw(device, &mut exec, state, false, None, *vertex_count, *instance_count, *start_vertex, 0, *start_instance) {
                log::warn!(target: "d3d", "d3d11 Draw({vertex_count}, {instance_count}): {e}");
            }
        }
        RecordedCmd::DrawIndexed { state, index_count, instance_count, start_index, base_vertex, start_instance } => {
            if let Err(e) = exec_draw(device, &mut exec, state, false, Some(*start_index), *index_count, *instance_count, 0, *base_vertex, *start_instance) {
                log::warn!(target: "d3d", "d3d11 DrawIndexed({index_count}, {instance_count}): {e}");
            }
        }
        RecordedCmd::DrawInstancedIndirect { state, args, offset } => {
            if let Err(e) = exec_draw_indirect(device, &mut exec, state, false, args, *offset, false) {
                log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::DrawIndexedInstancedIndirect { state, args, offset } => {
            if let Err(e) = exec_draw_indirect(device, &mut exec, state, false, args, *offset, true) {
                log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::Dispatch { state, x, y, z } => {
            if let Err(e) = exec_dispatch(device, &mut exec, state, false, *x, *y, *z) {
                log::warn!(target: "d3d", "d3d11 Dispatch({x}, {y}, {z}): {e}");
            }
        }
        RecordedCmd::DispatchIndirect { state, args, offset } => {
            if let Err(e) = exec_dispatch_indirect(device, &mut exec, state, false, args, *offset) {
                log::warn!(target: "d3d", "d3d11 DispatchIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::CopyStructureCount { dst, dst_offset, uav } => {
            if let Some(counter) = &uav.counter {
                let db = dst.current_backing();
                let dst_w = dst.kind.buffer_width() as u64;
                let off = *dst_offset as u64;
                if off & 3 == 0 && off.saturating_add(4) <= dst_w {
                    exec.cl.barrier();
                    if let Err(e) = exec.cl.copy_buffer(&db.mtl, db.offset + off, counter, 0, 4) {
                        log::warn!(target: "d3d", "d3d11 CopyStructureCount: {e}");
                    } else {
                        exec.cl.barrier();
                        dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                        exec.has_work = true;
                    }
                    exec.keep_mtl(&db.mtl);
                    exec.keep_mtl(counter);
                } else {
                    log::warn!(
                        target: "d3d",
                        "d3d11 CopyStructureCount: misaligned or out-of-bounds offset {off} for {dst_w}B dst buffer"
                    );
                }
            } else {
                log::warn!(target: "d3d", "d3d11 CopyStructureCount: source UAV has no APPEND/COUNTER flag");
            }
        }
        RecordedCmd::SetUavCounter { uav, value } => {
            if let Some(counter) = &uav.counter {
                exec.cl.barrier();
                if let Err(e) = exec.cl.write_buffer_u32(counter, 0, *value) {
                    log::warn!(target: "d3d", "d3d11 SetUavCounter({value}): {e}");
                } else {
                    exec.cl.barrier();
                    exec.has_work = true;
                }
                exec.keep_mtl(counter);
            }
        }
        RecordedCmd::GenerateMips { srv } => {
            if let Err(e) = exec_generate_mips(&mut exec, srv) {
                log::warn!(target: "d3d", "d3d11 GenerateMips: {e}");
            }
        }
        RecordedCmd::BeginQuery(q) => {
            let mut rt = q.rt.lock().unwrap();
            rt.ended = false;
            rt.submitted = false;
            if q.desc.Query == query_kind::PIPELINE_STATISTICS {
                rt.pipeline_stats = D3D11_QUERY_DATA_PIPELINE_STATISTICS::default();
                drop(rt);
                if !exec.active_pipeline_stats.iter().any(|existing| Arc::ptr_eq(existing, q)) {
                    exec.active_pipeline_stats.push(q.clone());
                }
            } else if matches!(q.desc.Query, query_kind::OCCLUSION | query_kind::OCCLUSION_PREDICATE) {
                if let Some(heap) = rt.heap.clone() {
                    let binary = q.desc.Query == query_kind::OCCLUSION_PREDICATE;
                    if let Err(e) = exec.cl.begin_occlusion(&heap, 0, binary) {
                        log::warn!(target: "d3d", "d3d11 BeginQuery(occlusion): {e}");
                    } else {
                        exec.keep(heap);
                        exec.has_work = true;
                    }
                }
            }
        }
        RecordedCmd::EndQuery(q) => {
            let mut rt = q.rt.lock().unwrap();
            rt.ended = true;
            rt.submitted = false;
            match q.desc.Query {
                query_kind::PIPELINE_STATISTICS => {
                    exec.active_pipeline_stats.retain(|existing| !Arc::ptr_eq(existing, q));
                }
                query_kind::TIMESTAMP => {
                    if let (Some(heap), Some(rb)) = (rt.heap.clone(), rt.readback.clone()) {
                        let slot = rt.slot;
                        let _ = exec.cl.write_timestamp(&heap, slot);
                        let _ = exec.cl.resolve_timestamps(&heap, slot, 1, &rb, 0);
                        exec.keep((heap, rb));
                        exec.has_work = true;
                    }
                }
                query_kind::OCCLUSION | query_kind::OCCLUSION_PREDICATE => {
                    if let (Some(heap), Some(rb)) = (rt.heap.clone(), rt.readback.clone()) {
                        let _ = exec.cl.end_occlusion(&heap, 0);
                        let _ = exec.cl.resolve_occlusion(&heap, 0, 1, &rb, 0);
                        exec.keep((heap, rb));
                        exec.has_work = true;
                    }
                }
                _ => {}
            }
            drop(rt);
            exec.pending_queries.push(q.clone());
        }
        RecordedCmd::DiscardResource(res) => {
            let mtl = res.current_mtl();
            exec.cl.discard_resource(&mtl, None);
            res.last_gpu_state.store(gpu_state::COMMON, Ordering::Release);
            exec.keep_mtl(&mtl);
        }
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn exec_update_subresource_raw(
    exec: &mut ImmediateExec,
    dst: &ResourceCore,
    dst_sub: u32,
    dst_box: Option<D3D11_BOX>,
    src_ptr: *const u8,
    src_len: Option<usize>,
    src_row_pitch: u32,
    src_depth_pitch: u32,
) -> Result<(), String> {
    if dst.is_buffer() {
        let db = dst.current_backing();
        let dst_w = dst.kind.buffer_width() as u64;
        let (dst_off, len) = match dst_box {
            Some(b) => (b.left as u64, b.right.saturating_sub(b.left) as u64),
            None => (0, dst_w),
        };
        let mut copy_len = (len as usize).min(dst_w.saturating_sub(dst_off) as usize);
        if let Some(slen) = src_len {
            copy_len = copy_len.min(slen);
        }
        if copy_len == 0 {
            return Ok(());
        }
        let (up, up_off, up_cpu) = exec.alloc_upload_slice(copy_len, 256)?;
        std::ptr::copy_nonoverlapping(src_ptr, up_cpu, copy_len);
        exec.cl.barrier();
        exec.cl.copy_buffer(&db.mtl, db.offset + dst_off, &up, up_off, copy_len as u64)?;
        exec.cl.barrier();
        dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
        exec.has_work = true;
        exec.keep_mtl(&db.mtl);
        return Ok(());
    }

    let dst_mtl = dst.current_mtl();
    let (layouts, _) = d3d12_metal::layout::copyable_footprints(&dst_mtl.desc, dst_sub, 1, 0)
        .ok_or_else(|| format!("copyable_footprints failed for subresource {dst_sub}"))?;
    let lay = layouts[0];
    let full_w = lay.footprint.width;
    let full_h = lay.footprint.height;
    let full_d = lay.footprint.depth.max(1);
    let fi = d3d12_metal::format::info(dst_mtl.desc.Format);
    let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
    let bpe = fi.bytes.max(1) as usize;

    let (x0, y0, z0, w, h, d) = match dst_box {
        Some(b) => {
            if b.right <= b.left || b.bottom <= b.top || b.back <= b.front {
                return Ok(());
            }
            (b.left, b.top, b.front, b.right - b.left, b.bottom - b.top, b.back - b.front)
        }
        None => (0, 0, 0, full_w, full_h, full_d),
    };
    let row_bytes = (w.div_ceil(bw) as usize) * bpe;
    let num_rows = h.div_ceil(bh) as usize;
    let dst_row_pitch = row_bytes.next_multiple_of(256).max(256);
    let dst_slice_pitch = dst_row_pitch * num_rows;
    let total = dst_slice_pitch * d as usize;
    let (up, up_off, dst_base) = exec.alloc_upload_slice(total.max(16), 512)?;
    let srp = if src_row_pitch == 0 { row_bytes } else { src_row_pitch as usize };
    let sdp = if src_depth_pitch == 0 { srp * num_rows } else { src_depth_pitch as usize };
    for z in 0..d as usize {
        for y in 0..num_rows {
            let soff = z * sdp + y * srp;
            let doff = z * dst_slice_pitch + y * dst_row_pitch;
            let n = match src_len {
                Some(slen) => {
                    if soff >= slen {
                        0
                    } else {
                        row_bytes.min(slen - soff)
                    }
                }
                None => row_bytes,
            };
            if n > 0 {
                std::ptr::copy_nonoverlapping(src_ptr.add(soff), dst_base.add(doff), n);
            }
        }
    }
    let fp = PlacedFootprint {
        offset: up_off,
        format: lay.footprint.format,
        width: w.next_multiple_of(bw),
        height: h.next_multiple_of(bh),
        depth: d,
        row_pitch: dst_row_pitch as u32,
        _pad: 0,
    };
    exec.cl.barrier();
    exec.cl.copy_texture_region(
        CopyLoc::Subresource(&dst_mtl, dst_sub),
        [x0, y0, z0],
        CopyLoc::Footprint(&up, fp),
        None,
    )?;
    exec.cl.barrier();
    dst.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
    exec.has_work = true;
    exec.keep_mtl(&dst_mtl);
    Ok(())
}

unsafe fn prepare_gfx_draw(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
    indexed: bool,
    indirect_arg: Option<(&ResourceCore, &Arc<MtlResource>)>,
) -> Result<(), String> {
    let cached = get_or_build_gfx_pso(device, exec, st, indexed, live_state)?;
    let pso = &cached.pso;
    let usage = cached.usage;

    // Pre-reserve worst-case descriptor space BEFORE allocating any stage tables so
    // a mid-draw heap rotation cannot invalidate an earlier stage's table pointer.
    let need_res = usage.vs.res_table_len
        + usage.ps.res_table_len
        + if st.gs.is_some() { usage.gs.res_table_len } else { 0 }
        + if st.hs.is_some() { usage.hs.res_table_len } else { 0 }
        + if st.ds.is_some() { usage.ds.res_table_len } else { 0 };
    let need_samp = usage.vs.samp_table_len
        + usage.ps.samp_table_len
        + if st.gs.is_some() { usage.gs.samp_table_len } else { 0 }
        + if st.hs.is_some() { usage.hs.samp_table_len } else { 0 }
        + if st.ds.is_some() { usage.ds.samp_table_len } else { 0 };
    exec.ensure_res_space(need_res)?;
    exec.ensure_samp_space(need_samp)?;

    // Resolve per-stage resource & sampler descriptor tables (reusing cached tables when unchanged).
    let (vs_res_gpu, mut wrote_res) =
        resolve_stage_res_table(device, exec, STAGE_VS, &st.vs_bind, None, 0, usage.vs, live_state);
    let (ps_res_gpu, w) = resolve_stage_res_table(
        device,
        exec,
        STAGE_PS,
        &st.ps_bind,
        Some(&st.om_uavs),
        st.om_uavs_gen,
        usage.ps,
        live_state,
    );
    wrote_res |= w;
    let vs_samp_gpu = resolve_stage_samp_table(exec, STAGE_VS, &st.vs_bind, usage.vs, live_state);
    let ps_samp_gpu = resolve_stage_samp_table(exec, STAGE_PS, &st.ps_bind, usage.ps, live_state);
    let gs_tables = if st.gs.is_some() {
        let (r, w) = resolve_stage_res_table(device, exec, STAGE_GS, &st.gs_bind, None, 0, usage.gs, live_state);
        wrote_res |= w;
        let s = resolve_stage_samp_table(exec, STAGE_GS, &st.gs_bind, usage.gs, live_state);
        Some((r, s))
    } else {
        None
    };
    let hs_tables = if st.hs.is_some() {
        let (r, w) = resolve_stage_res_table(device, exec, STAGE_HS, &st.hs_bind, None, 0, usage.hs, live_state);
        wrote_res |= w;
        let s = resolve_stage_samp_table(exec, STAGE_HS, &st.hs_bind, usage.hs, live_state);
        Some((r, s))
    } else {
        None
    };
    let ds_tables = if st.ds.is_some() {
        let (r, w) = resolve_stage_res_table(device, exec, STAGE_DS, &st.ds_bind, None, 0, usage.ds, live_state);
        wrote_res |= w;
        let s = resolve_stage_samp_table(exec, STAGE_DS, &st.ds_bind, usage.ds, live_state);
        Some((r, s))
    } else {
        None
    };
    if wrote_res {
        bump_desc_heap_gen();
    }

    // Render targets & DSV (skip rebuilding TargetViews when unchanged on this open command buffer).
    let mut rt_sig = 0x517c_c1b7_2722_0a95u64;
    let mut max_rt = 0usize;
    for (i, slot) in st.rtvs.iter().enumerate() {
        if let Some(rtv) = &slot.rtv {
            max_rt = i + 1;
            let v = ((Arc::as_ptr(rtv) as u64) << 16) ^ rtv.resource.version.load(Ordering::Acquire);
            mix_sig(&mut rt_sig, (i as u64) | (v << 4));
        }
    }
    if let Some(dsv) = &st.dsv {
        let v = ((Arc::as_ptr(dsv) as u64) << 16) ^ dsv.resource.version.load(Ordering::Acquire);
        mix_sig(&mut rt_sig, 0x8000 | (v << 4));
    }
    let need_rt = !live_state || st.rt_dirty || !exec.rt_synced || rt_sig != exec.rt_backings_sig;
    if need_rt {
        exec.scratch_attached.clear();
        let mut rtv_views = Vec::with_capacity(max_rt);
        for slot in &st.rtvs[..max_rt] {
            if let Some(rtv) = &slot.rtv {
                let (mtl, tv) = rtv.target_view()?;
                exec.scratch_attached.push(Arc::as_ptr(&mtl) as usize);
                exec.keep_mtl(&mtl);
                rtv_views.push(tv);
            } else {
                rtv_views.push(TargetView::NULL);
            }
        }
        let dsv_view = match &st.dsv {
            Some(dsv) => {
                let (mtl, tv) = dsv.target_view()?;
                exec.scratch_attached.push(Arc::as_ptr(&mtl) as usize);
                exec.keep_mtl(&mtl);
                Some(tv)
            }
            None => None,
        };
        exec.cl.set_render_targets(rtv_views, dsv_view);
        if live_state {
            exec.rt_synced = true;
            exec.rt_backings_sig = rt_sig;
        } else {
            exec.rt_synced = false;
        }
    }

    // Per-resource last-state hazard tracking: only emit SRV Transition barriers when a
    // bound SRV actually has a pending write state (prevents splitting Metal render passes
    // on steady-state read-only textures).
    exec.scratch_barriers.clear();
    let check_srv_stage = |bind: &StageBindings,
                           mask: u128,
                           attached: &[usize],
                           barriers: &mut Vec<Barrier>,
                           keep_mtls: &mut Vec<Arc<MtlResource>>| {
        let mut sm = mask;
        while sm != 0 {
            let i = sm.trailing_zeros() as usize;
            sm &= sm - 1;
            if let Some(srv) = &bind.srvs[i].srv {
                let prev = srv.resource.last_gpu_state.load(Ordering::Acquire);
                if (prev & gpu_state::WRITE_MASK) != 0 {
                    let mtl = srv.resource.current_mtl();
                    let ptr = Arc::as_ptr(&mtl) as usize;
                    if !attached.contains(&ptr) {
                        barriers.push(Barrier::Transition(
                            ptr,
                            prev & gpu_state::WRITE_MASK,
                            gpu_state::SRV_ALL,
                        ));
                        srv.resource.last_gpu_state.store(gpu_state::SRV_ALL, Ordering::Release);
                    }
                    if keep_mtls.last().is_none_or(|last| !Arc::ptr_eq(last, &mtl)) {
                        keep_mtls.push(mtl);
                    }
                }
            }
        }
    };
    check_srv_stage(&st.vs_bind, usage.vs.srv_mask, &exec.scratch_attached, &mut exec.scratch_barriers, &mut exec.keep_mtls);
    check_srv_stage(&st.ps_bind, usage.ps.srv_mask, &exec.scratch_attached, &mut exec.scratch_barriers, &mut exec.keep_mtls);
    if st.gs.is_some() {
        check_srv_stage(&st.gs_bind, usage.gs.srv_mask, &exec.scratch_attached, &mut exec.scratch_barriers, &mut exec.keep_mtls);
    }
    if st.hs.is_some() {
        check_srv_stage(&st.hs_bind, usage.hs.srv_mask, &exec.scratch_attached, &mut exec.scratch_barriers, &mut exec.keep_mtls);
    }
    if st.ds.is_some() {
        check_srv_stage(&st.ds_bind, usage.ds.srv_mask, &exec.scratch_attached, &mut exec.scratch_barriers, &mut exec.keep_mtls);
    }
    let mut om_um = usage.ps.uav_mask;
    while om_um != 0 {
        let i = om_um.trailing_zeros() as usize;
        om_um &= om_um - 1;
        if let Some(uav) = &st.om_uavs[i].uav {
            let mtl = uav.resource.current_mtl();
            exec.scratch_barriers.push(Barrier::Uav(Arc::as_ptr(&mtl) as usize));
            uav.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
            exec.keep_mtl(&mtl);
            if let Some(counter) = &uav.counter {
                exec.scratch_barriers.push(Barrier::Uav(Arc::as_ptr(counter) as usize));
                exec.keep_mtl(counter);
            }
        }
    }
    if let Some((arg_res, arg_mtl)) = indirect_arg {
        let prev = arg_res.last_gpu_state.load(Ordering::Acquire);
        if prev != gpu_state::INDIRECT_ARGUMENT {
            exec.scratch_barriers.push(Barrier::Transition(
                Arc::as_ptr(arg_mtl) as usize,
                prev & gpu_state::WRITE_MASK,
                gpu_state::INDIRECT_ARGUMENT,
            ));
            arg_res.last_gpu_state.store(gpu_state::INDIRECT_ARGUMENT, Ordering::Release);
        }
    }
    if !exec.scratch_barriers.is_empty() {
        exec.cl.barrier_list(&exec.scratch_barriers);
    }

    // Mark bound render targets / writable DSV as written for future SRV reads.
    for slot in &st.rtvs[..max_rt] {
        if let Some(rtv) = &slot.rtv {
            if rtv.resource.last_gpu_state.load(Ordering::Acquire) != gpu_state::RENDER_TARGET {
                rtv.resource.last_gpu_state.store(gpu_state::RENDER_TARGET, Ordering::Release);
            }
        }
    }
    if let Some(dsv) = &st.dsv {
        let read_only_both = (dsv.desc.Flags & 0x3) == 0x3;
        if !read_only_both && dsv.resource.last_gpu_state.load(Ordering::Acquire) != gpu_state::DEPTH_WRITE {
            dsv.resource.last_gpu_state.store(gpu_state::DEPTH_WRITE, Ordering::Release);
        }
    }

    if !exec.gfx_heaps_bound {
        exec.bind_active_heaps();
        exec.cl.gfx_root.set_root_signature(device.gfx_rs.clone())?;
        exec.gfx_heaps_bound = true;
        exec.bound_gfx_tables = [u64::MAX; 10];
    }
    if exec.bound_gfx_tables[0] != vs_res_gpu {
        exec.cl.gfx_root.set_table(0, vs_res_gpu)?;
        exec.bound_gfx_tables[0] = vs_res_gpu;
    }
    if exec.bound_gfx_tables[1] != vs_samp_gpu {
        exec.cl.gfx_root.set_table(1, vs_samp_gpu)?;
        exec.bound_gfx_tables[1] = vs_samp_gpu;
    }
    if exec.bound_gfx_tables[2] != ps_res_gpu {
        exec.cl.gfx_root.set_table(2, ps_res_gpu)?;
        exec.bound_gfx_tables[2] = ps_res_gpu;
    }
    if exec.bound_gfx_tables[3] != ps_samp_gpu {
        exec.cl.gfx_root.set_table(3, ps_samp_gpu)?;
        exec.bound_gfx_tables[3] = ps_samp_gpu;
    }
    if let Some((gs_res_gpu, gs_samp_gpu)) = gs_tables {
        if exec.bound_gfx_tables[4] != gs_res_gpu {
            exec.cl.gfx_root.set_table(4, gs_res_gpu)?;
            exec.bound_gfx_tables[4] = gs_res_gpu;
        }
        if exec.bound_gfx_tables[5] != gs_samp_gpu {
            exec.cl.gfx_root.set_table(5, gs_samp_gpu)?;
            exec.bound_gfx_tables[5] = gs_samp_gpu;
        }
    }
    if let Some((hs_res_gpu, hs_samp_gpu)) = hs_tables {
        if exec.bound_gfx_tables[6] != hs_res_gpu {
            exec.cl.gfx_root.set_table(6, hs_res_gpu)?;
            exec.bound_gfx_tables[6] = hs_res_gpu;
        }
        if exec.bound_gfx_tables[7] != hs_samp_gpu {
            exec.cl.gfx_root.set_table(7, hs_samp_gpu)?;
            exec.bound_gfx_tables[7] = hs_samp_gpu;
        }
    }
    if let Some((ds_res_gpu, ds_samp_gpu)) = ds_tables {
        if exec.bound_gfx_tables[8] != ds_res_gpu {
            exec.cl.gfx_root.set_table(8, ds_res_gpu)?;
            exec.bound_gfx_tables[8] = ds_res_gpu;
        }
        if exec.bound_gfx_tables[9] != ds_samp_gpu {
            exec.cl.gfx_root.set_table(9, ds_samp_gpu)?;
            exec.bound_gfx_tables[9] = ds_samp_gpu;
        }
    }

    if exec.cl.gfx.pso.as_ref().is_none_or(|cur| !Arc::ptr_eq(cur, pso)) {
        exec.cl.set_graphics_pipeline(pso.clone());
        exec.keep_pso(pso);
    }

    // Viewports, scissors, blend factor, stencil ref (only push when dirty or on first draw of cmdbuf).
    let sync_all_dyn = !live_state || !exec.dyn_synced;
    if sync_all_dyn || st.vp_dirty {
        let mut mtl_vps = Vec::with_capacity(st.viewports.len());
        for vp in &st.viewports {
            push_mtl_viewport(&mut mtl_vps, vp);
        }
        exec.cl.set_viewports(mtl_vps);
    }
    if sync_all_dyn || st.scissor_dirty {
        let scissor_on = st.rast.as_ref().is_some_and(|r| r.scissor_enable);
        exec.cl.set_scissors(if scissor_on { st.scissors.clone() } else { Vec::new() });
    }
    if sync_all_dyn || st.blend_factor_dirty {
        exec.cl.set_blend_factor(st.blend_factor);
    }
    if sync_all_dyn || st.stencil_ref_dirty {
        exec.cl.set_stencil_ref(st.stencil_ref);
    }
    exec.dyn_synced = live_state;
    if !live_state {
        exec.cached_gfx_pso = None;
    }

    // Vertex buffers (only slots declared by the pipeline's InputLayout) & index buffer.
    exec.cl.gfx.topology = st.topology;
    let new_vbs_mask = pso.vertex_slots;
    let mut vm = new_vbs_mask;
    while vm != 0 {
        let i = vm.trailing_zeros() as usize;
        vm &= vm - 1;
        if i >= MAX_VERTEX_SLOTS as usize {
            break;
        }
        let vb = &st.vbs[i];
        if let Some(res) = &vb.res {
            let b = res.current_backing();
            let buf_w = res.kind.buffer_width() as u64;
            let off = vb.offset as u64;
            let avail = buf_w.saturating_sub(off).min(u32::MAX as u64) as u32;
            exec.cl.gfx.vbs[i] = (b.mtl.gpu_va + b.offset + off, avail, vb.stride);
            exec.keep_mtl(&b.mtl);
        } else {
            exec.cl.gfx.vbs[i] = (0, 0, 0);
        }
    }
    let mut stale = exec.bound_vbs_mask & !new_vbs_mask;
    while stale != 0 {
        let i = stale.trailing_zeros() as usize;
        stale &= stale - 1;
        exec.cl.gfx.vbs[i] = (0, 0, 0);
    }
    exec.bound_vbs_mask = new_vbs_mask;

    if indexed {
        if let Some(ib) = &st.ib {
            let b = ib.current_backing();
            let buf_w = ib.kind.buffer_width() as u64;
            let off = st.ib_offset as u64;
            let avail = buf_w.saturating_sub(off).min(u32::MAX as u64) as u32;
            exec.cl.gfx.ib = Some((b.mtl.gpu_va + b.offset + off, avail, st.ib_format));
            exec.keep_mtl(&b.mtl);
        } else {
            return Err("indexed draw with no index buffer bound".into());
        }
    } else {
        exec.cl.gfx.ib = None;
    }
    Ok(())
}

pub(super) fn accumulate_draw_pipeline_stats(
    stats: &mut D3D11_QUERY_DATA_PIPELINE_STATISTICS,
    topology: u32,
    count: u32,
    instances: u32,
    indexed: bool,
    gs_instances: Option<u32>,
    has_hs: bool,
) {
    use d3d12_metal::topology::d3d;
    // Indexed strip topologies can contain primitive-restart cut indices (0xFFFF / 0xFFFFFFFF)
    // in GPU-resident index buffers, so IA vertex/primitive counts cannot be derived on the CPU.
    if indexed
        && matches!(
            topology,
            d3d::LINESTRIP | d3d::TRIANGLESTRIP | d3d::LINESTRIP_ADJ | d3d::TRIANGLESTRIP_ADJ
        )
    {
        return;
    }
    let verts = (count as u64).saturating_mul(instances as u64);
    let prims = d3d11_primitives_per_instance(topology, count as u64)
        .saturating_mul(instances as u64);
    stats.IAVertices = stats.IAVertices.saturating_add(verts);
    stats.IAPrimitives = stats.IAPrimitives.saturating_add(prims);
    // Non-indexed draws invoke VS once per sequential vertex. Indexed draws can reuse cached
    // vertex-shader outputs for repeated indices, so leave VSInvocations unmeasured for them.
    if !indexed {
        stats.VSInvocations = stats.VSInvocations.saturating_add(verts);
    }
    // Without tessellation, each IA primitive invokes the GS `gs_instance_count` times
    // (1 unless `[instance(N)]` is declared). When tessellation is active, the number of
    // primitives reaching GS depends on GPU tessellation factors, so do not fabricate
    // GSInvocations from the input-assembler patch count.
    if let Some(gs_inst) = gs_instances {
        if !has_hs {
            stats.GSInvocations = stats
                .GSInvocations
                .saturating_add(prims.saturating_mul(gs_inst.max(1) as u64));
        }
    }
    if has_hs {
        stats.HSInvocations = stats.HSInvocations.saturating_add(prims);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn exec_draw(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
    indexed_start: Option<u32>,
    count: u32,
    instances: u32,
    start_vertex: u32,
    base_vertex: i32,
    start_instance: u32,
) -> Result<bool, String> {
    if count == 0 || instances == 0 || st.vs.is_none() {
        return Ok(false);
    }
    prepare_gfx_draw(device, exec, st, live_state, indexed_start.is_some(), None)?;
    if let Some(start_index) = indexed_start {
        exec.cl.draw_indexed(count, instances, start_index, base_vertex, start_instance)?;
    } else {
        exec.cl.draw(count, instances, start_vertex, start_instance)?;
    }
    if !exec.active_pipeline_stats.is_empty() {
        let gs_instances = exec
            .cl
            .gfx
            .pso
            .as_ref()
            .and_then(|p| p.gs_emu.as_ref())
            .map(|g| g.instance_count.max(1));
        for q in &exec.active_pipeline_stats {
            let mut rt = q.rt.lock().unwrap();
            accumulate_draw_pipeline_stats(
                &mut rt.pipeline_stats,
                st.topology,
                count,
                instances,
                indexed_start.is_some(),
                gs_instances,
                st.hs.is_some(),
            );
        }
    }
    exec.frame_stats.draws += 1;
    exec.has_work = true;
    Ok(true)
}

unsafe fn exec_draw_indirect(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
    args: &ResourceCore,
    offset: u32,
    indexed: bool,
) -> Result<bool, String> {
    if st.vs.is_none() {
        return Ok(false);
    }
    let arg_size: u64 = if indexed { 20 } else { 16 };
    let arg_b = args.current_backing();
    let arg_w = args.kind.buffer_width() as u64;
    let off = offset as u64;
    if off & 3 != 0 || off.saturating_add(arg_size) > arg_w {
        return Err(format!(
            "indirect draw args offset {off}+{arg_size} out of bounds or misaligned ({arg_w}B buffer)"
        ));
    }
    arg_b.mtl.register_ptr();
    prepare_gfx_draw(device, exec, st, live_state, indexed, Some((args, &arg_b.mtl)))?;
    let layout = IndirectLayout {
        stride: arg_size as u32,
        work: if indexed { Work::DrawIndexed } else { Work::Draw },
        work_offset: 0,
        patches: Vec::new(),
        vb_offsets: Vec::new(),
        ib_offset: None,
    };
    exec.cl.execute_indirect(&layout, 1, arg_b.mtl.gpu_va + arg_b.offset + off, None, None)?;
    exec.keep_mtl(&arg_b.mtl);
    exec.frame_stats.draws += 1;
    exec.has_work = true;
    Ok(true)
}

unsafe fn prepare_compute_dispatch(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
) -> Result<Arc<CachedComputePso>, String> {
    let Some(cs) = &st.cs else {
        return Err("no compute shader bound".into());
    };
    let cached = cs.pipeline()?;
    let usage = cached.usage;

    exec.ensure_res_space(usage.res_table_len)?;
    exec.ensure_samp_space(usage.samp_table_len)?;

    let (res_gpu, wrote_res) = resolve_stage_res_table(
        device,
        exec,
        STAGE_CS,
        &st.cs_bind,
        Some(&st.cs_uavs),
        st.cs_uavs_gen,
        usage,
        live_state,
    );
    let samp_gpu = resolve_stage_samp_table(exec, STAGE_CS, &st.cs_bind, usage, live_state);
    if wrote_res {
        bump_desc_heap_gen();
    }

    if !exec.compute_heaps_bound {
        exec.bind_active_heaps();
        exec.cl.compute_root.set_root_signature(device.compute_rs.clone())?;
        exec.compute_heaps_bound = true;
        exec.bound_cs_tables = [u64::MAX; 2];
    }
    if exec.bound_cs_tables[0] != res_gpu {
        exec.cl.compute_root.set_table(0, res_gpu)?;
        exec.bound_cs_tables[0] = res_gpu;
    }
    if exec.bound_cs_tables[1] != samp_gpu {
        exec.cl.compute_root.set_table(1, samp_gpu)?;
        exec.bound_cs_tables[1] = samp_gpu;
    }
    exec.cl.set_compute_pipeline(cached.pso.clone());
    exec.keep(cached.pso.clone());
    Ok(cached)
}

unsafe fn exec_dispatch(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
    x: u32,
    y: u32,
    z: u32,
) -> Result<(), String> {
    if x == 0 || y == 0 || z == 0 {
        return Ok(());
    }
    let cached = prepare_compute_dispatch(device, exec, st, live_state)?;
    exec.cl.barrier();
    exec.cl.dispatch(x, y, z)?;
    exec.cl.barrier();
    let mut um = cached.usage.uav_mask;
    while um != 0 {
        let i = um.trailing_zeros() as usize;
        um &= um - 1;
        if let Some(uav) = &st.cs_uavs[i].uav {
            uav.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
        }
    }
    if !exec.active_pipeline_stats.is_empty() {
        let tg = cached.pso.threadgroup;
        let threads = (x as u64)
            .saturating_mul(y as u64)
            .saturating_mul(z as u64)
            .saturating_mul(tg[0].max(1) as u64)
            .saturating_mul(tg[1].max(1) as u64)
            .saturating_mul(tg[2].max(1) as u64);
        for q in &exec.active_pipeline_stats {
            let mut rt = q.rt.lock().unwrap();
            rt.pipeline_stats.CSInvocations = rt.pipeline_stats.CSInvocations.saturating_add(threads);
        }
    }
    exec.has_work = true;
    Ok(())
}

unsafe fn exec_dispatch_indirect(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    live_state: bool,
    args: &ResourceCore,
    offset: u32,
) -> Result<(), String> {
    let arg_b = args.current_backing();
    let arg_w = args.kind.buffer_width() as u64;
    let off = offset as u64;
    if off & 3 != 0 || off.saturating_add(12) > arg_w {
        return Err(format!(
            "DispatchIndirect args offset {off}+12 out of bounds or misaligned ({arg_w}B buffer)"
        ));
    }
    arg_b.mtl.register_ptr();
    let cached = prepare_compute_dispatch(device, exec, st, live_state)?;
    let layout = IndirectLayout {
        stride: 12,
        work: Work::Dispatch,
        work_offset: 0,
        patches: Vec::new(),
        vb_offsets: Vec::new(),
        ib_offset: None,
    };
    exec.cl.barrier();
    exec.cl.execute_indirect(&layout, 1, arg_b.mtl.gpu_va + arg_b.offset + off, None, None)?;
    exec.cl.barrier();
    let mut um = cached.usage.uav_mask;
    while um != 0 {
        let i = um.trailing_zeros() as usize;
        um &= um - 1;
        if let Some(uav) = &st.cs_uavs[i].uav {
            uav.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
        }
    }
    exec.keep_mtl(&arg_b.mtl);
    exec.has_work = true;
    Ok(())
}

unsafe fn exec_generate_mips(exec: &mut ImmediateExec, srv: &SrvInner) -> Result<(), String> {
    let misc = srv.resource.kind.misc_flags();
    let bind = srv.resource.kind.bind_flags();
    if misc & misc_flag::GENERATE_MIPS == 0 {
        log::debug!(
            target: "d3d",
            "d3d11 GenerateMips: ignored because resource lacks D3D11_RESOURCE_MISC_GENERATE_MIPS (misc={misc:#x}, bind={bind:#x})"
        );
        return Ok(());
    }
    let u = srv.desc.u;
    let (most_detailed_mip, mip_levels, first_slice, slice_count) = match srv.desc.ViewDimension {
        srv_dim::TEXTURE1D | srv_dim::TEXTURE2D | srv_dim::TEXTURE3D => (u[0], u[1], 0, 1),
        srv_dim::TEXTURE1DARRAY | srv_dim::TEXTURE2DARRAY => (u[0], u[1], u[2], u[3]),
        srv_dim::TEXTURECUBE => (u[0], u[1], 0, 6),
        srv_dim::TEXTURECUBEARRAY => (u[0], u[1], u[2], u[3].saturating_mul(6)),
        _ => return Ok(()),
    };
    let mtl = srv.resource.current_mtl();
    exec.cl.barrier();
    exec.cl.generate_mips(
        &mtl,
        srv.desc.Format,
        most_detailed_mip,
        mip_levels,
        first_slice,
        slice_count,
    )?;
    exec.cl.barrier();
    srv.resource.last_gpu_state.store(gpu_state::UNORDERED_ACCESS, Ordering::Release);
    exec.invalidate_live_caches();
    exec.keep_mtl(&mtl);
    exec.has_work = true;
    Ok(())
}

// ---------------------------------------------------------------------------
// D3d11Context COM object (both immediate and deferred)
// ---------------------------------------------------------------------------

pub struct D3d11Context {
    pub device: Arc<DeviceCore>,
    pub deferred: bool,
    pub flags: u32,
    pub state: Mutex<ContextState>,
    pub recorded: Mutex<Vec<RecordedCmd>>,
    pub active_context_state: Mutex<Option<ComRef>>,
}

impl D3d11Context {
    pub fn new_immediate(device: Arc<DeviceCore>) -> *mut Obj<Self> {
        Obj::new(Self {
            device,
            deferred: false,
            flags: 0,
            state: Mutex::new(ContextState::default()),
            recorded: Mutex::new(Vec::new()),
            active_context_state: Mutex::new(None),
        })
    }

    pub fn new_deferred(device: Arc<DeviceCore>, flags: u32) -> *mut Obj<Self> {
        Obj::new(Self {
            device,
            deferred: true,
            flags,
            state: Mutex::new(ContextState::default()),
            recorded: Mutex::new(Vec::new()),
            active_context_state: Mutex::new(None),
        })
    }

    fn emit(&self, cmd: RecordedCmd) {
        if self.deferred {
            self.recorded.lock().unwrap().push(cmd);
        } else {
            unsafe { exec_recorded_cmd(&self.device, &cmd) };
        }
    }
}

impl HasDeviceCore for D3d11Context {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}

static CTX_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11Context {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&CTX_CLASS, || {
            let mut s = child_slots::<D3d11Context>();
            s.extend([
                // [7..114]: ID3D11DeviceContext
                ("VSSetConstantBuffers", Some(vs_set_constant_buffers as *const ())),
                ("PSSetShaderResources", Some(ps_set_shader_resources as *const ())),
                ("PSSetShader", Some(ps_set_shader as *const ())),
                ("PSSetSamplers", Some(ps_set_samplers as *const ())),
                ("VSSetShader", Some(vs_set_shader as *const ())),
                ("DrawIndexed", Some(draw_indexed as *const ())),
                ("Draw", Some(draw as *const ())),
                ("Map", Some(map as *const ())),
                ("Unmap", Some(unmap as *const ())),
                ("PSSetConstantBuffers", Some(ps_set_constant_buffers as *const ())),
                ("IASetInputLayout", Some(ia_set_input_layout as *const ())),
                ("IASetVertexBuffers", Some(ia_set_vertex_buffers as *const ())),
                ("IASetIndexBuffer", Some(ia_set_index_buffer as *const ())),
                ("DrawIndexedInstanced", Some(draw_indexed_instanced as *const ())),
                ("DrawInstanced", Some(draw_instanced as *const ())),
                ("GSSetConstantBuffers", Some(gs_set_constant_buffers as *const ())),
                ("GSSetShader", Some(gs_set_shader as *const ())),
                ("IASetPrimitiveTopology", Some(ia_set_primitive_topology as *const ())),
                ("VSSetShaderResources", Some(vs_set_shader_resources as *const ())),
                ("VSSetSamplers", Some(vs_set_samplers as *const ())),
                ("Begin", Some(begin_async as *const ())),
                ("End", Some(end_async as *const ())),
                ("GetData", Some(get_data as *const ())),
                ("SetPredication", Some(set_predication as *const ())),
                ("GSSetShaderResources", Some(gs_set_shader_resources as *const ())),
                ("GSSetSamplers", Some(gs_set_samplers as *const ())),
                ("OMSetRenderTargets", Some(om_set_render_targets as *const ())),
                ("OMSetRenderTargetsAndUnorderedAccessViews", Some(om_set_render_targets_and_uavs as *const ())),
                ("OMSetBlendState", Some(om_set_blend_state as *const ())),
                ("OMSetDepthStencilState", Some(om_set_depth_stencil_state as *const ())),
                ("SOSetTargets", Some(so_set_targets as *const ())),
                ("DrawAuto", Some(draw_auto as *const ())),
                ("DrawIndexedInstancedIndirect", Some(draw_indexed_instanced_indirect as *const ())),
                ("DrawInstancedIndirect", Some(draw_instanced_indirect as *const ())),
                ("Dispatch", Some(dispatch as *const ())),
                ("DispatchIndirect", Some(dispatch_indirect as *const ())),
                ("RSSetState", Some(rs_set_state as *const ())),
                ("RSSetViewports", Some(rs_set_viewports as *const ())),
                ("RSSetScissorRects", Some(rs_set_scissor_rects as *const ())),
                ("CopySubresourceRegion", Some(copy_subresource_region as *const ())),
                ("CopyResource", Some(copy_resource as *const ())),
                ("UpdateSubresource", Some(update_subresource as *const ())),
                ("CopyStructureCount", Some(copy_structure_count as *const ())),
                ("ClearRenderTargetView", Some(clear_render_target_view as *const ())),
                ("ClearUnorderedAccessViewUint", Some(clear_uav_uint as *const ())),
                ("ClearUnorderedAccessViewFloat", Some(clear_uav_float as *const ())),
                ("ClearDepthStencilView", Some(crate::bridge_entry!(clear_depth_stencil_view))),
                ("GenerateMips", Some(generate_mips as *const ())),
                ("SetResourceMinLOD", Some(crate::bridge_entry!(set_resource_min_lod))),
                ("GetResourceMinLOD", Some(get_resource_min_lod as *const ())),
                ("ResolveSubresource", Some(resolve_subresource as *const ())),
                ("ExecuteCommandList", Some(execute_command_list as *const ())),
                ("HSSetShaderResources", Some(hs_set_shader_resources as *const ())),
                ("HSSetShader", Some(hs_set_shader as *const ())),
                ("HSSetSamplers", Some(hs_set_samplers as *const ())),
                ("HSSetConstantBuffers", Some(hs_set_constant_buffers as *const ())),
                ("DSSetShaderResources", Some(ds_set_shader_resources as *const ())),
                ("DSSetShader", Some(ds_set_shader as *const ())),
                ("DSSetSamplers", Some(ds_set_samplers as *const ())),
                ("DSSetConstantBuffers", Some(ds_set_constant_buffers as *const ())),
                ("CSSetShaderResources", Some(cs_set_shader_resources as *const ())),
                ("CSSetUnorderedAccessViews", Some(cs_set_uavs as *const ())),
                ("CSSetShader", Some(cs_set_shader as *const ())),
                ("CSSetSamplers", Some(cs_set_samplers as *const ())),
                ("CSSetConstantBuffers", Some(cs_set_constant_buffers as *const ())),
                ("VSGetConstantBuffers", Some(vs_get_constant_buffers as *const ())),
                ("PSGetShaderResources", Some(ps_get_shader_resources as *const ())),
                ("PSGetShader", Some(ps_get_shader as *const ())),
                ("PSGetSamplers", Some(ps_get_samplers as *const ())),
                ("VSGetShader", Some(vs_get_shader as *const ())),
                ("PSGetConstantBuffers", Some(ps_get_constant_buffers as *const ())),
                ("IAGetInputLayout", Some(ia_get_input_layout as *const ())),
                ("IAGetVertexBuffers", Some(ia_get_vertex_buffers as *const ())),
                ("IAGetIndexBuffer", Some(ia_get_index_buffer as *const ())),
                ("GSGetConstantBuffers", Some(gs_get_constant_buffers as *const ())),
                ("GSGetShader", Some(gs_get_shader as *const ())),
                ("IAGetPrimitiveTopology", Some(ia_get_primitive_topology as *const ())),
                ("VSGetShaderResources", Some(vs_get_shader_resources as *const ())),
                ("VSGetSamplers", Some(vs_get_samplers as *const ())),
                ("GetPredication", Some(get_predication as *const ())),
                ("GSGetShaderResources", Some(gs_get_shader_resources as *const ())),
                ("GSGetSamplers", Some(gs_get_samplers as *const ())),
                ("OMGetRenderTargets", Some(om_get_render_targets as *const ())),
                ("OMGetRenderTargetsAndUnorderedAccessViews", Some(om_get_render_targets_and_uavs as *const ())),
                ("OMGetBlendState", Some(om_get_blend_state as *const ())),
                ("OMGetDepthStencilState", Some(om_get_depth_stencil_state as *const ())),
                ("SOGetTargets", Some(so_get_targets as *const ())),
                ("RSGetState", Some(rs_get_state as *const ())),
                ("RSGetViewports", Some(rs_get_viewports as *const ())),
                ("RSGetScissorRects", Some(rs_get_scissor_rects as *const ())),
                ("HSGetShaderResources", Some(hs_get_shader_resources as *const ())),
                ("HSGetShader", Some(hs_get_shader as *const ())),
                ("HSGetSamplers", Some(hs_get_samplers as *const ())),
                ("HSGetConstantBuffers", Some(hs_get_constant_buffers as *const ())),
                ("DSGetShaderResources", Some(ds_get_shader_resources as *const ())),
                ("DSGetShader", Some(ds_get_shader as *const ())),
                ("DSGetSamplers", Some(ds_get_samplers as *const ())),
                ("DSGetConstantBuffers", Some(ds_get_constant_buffers as *const ())),
                ("CSGetShaderResources", Some(cs_get_shader_resources as *const ())),
                ("CSGetUnorderedAccessViews", Some(cs_get_uavs as *const ())),
                ("CSGetShader", Some(cs_get_shader as *const ())),
                ("CSGetSamplers", Some(cs_get_samplers as *const ())),
                ("CSGetConstantBuffers", Some(cs_get_constant_buffers as *const ())),
                ("ClearState", Some(clear_state as *const ())),
                ("Flush", Some(flush as *const ())),
                ("GetType", Some(get_type as *const ())),
                ("GetContextFlags", Some(get_context_flags as *const ())),
                ("FinishCommandList", Some(finish_command_list as *const ())),
                // [115..133]: ID3D11DeviceContext1
                ("CopySubresourceRegion1", Some(copy_subresource_region1 as *const ())),
                ("UpdateSubresource1", Some(update_subresource1 as *const ())),
                ("DiscardResource", Some(discard_resource as *const ())),
                ("DiscardView", Some(discard_view as *const ())),
                ("VSSetConstantBuffers1", Some(vs_set_constant_buffers1 as *const ())),
                ("HSSetConstantBuffers1", Some(hs_set_constant_buffers1 as *const ())),
                ("DSSetConstantBuffers1", Some(ds_set_constant_buffers1 as *const ())),
                ("GSSetConstantBuffers1", Some(gs_set_constant_buffers1 as *const ())),
                ("PSSetConstantBuffers1", Some(ps_set_constant_buffers1 as *const ())),
                ("CSSetConstantBuffers1", Some(cs_set_constant_buffers1 as *const ())),
                ("VSGetConstantBuffers1", Some(vs_get_constant_buffers1 as *const ())),
                ("HSGetConstantBuffers1", Some(hs_get_constant_buffers1 as *const ())),
                ("DSGetConstantBuffers1", Some(ds_get_constant_buffers1 as *const ())),
                ("GSGetConstantBuffers1", Some(gs_get_constant_buffers1 as *const ())),
                ("PSGetConstantBuffers1", Some(ps_get_constant_buffers1 as *const ())),
                ("CSGetConstantBuffers1", Some(cs_get_constant_buffers1 as *const ())),
                ("SwapDeviceContextState", Some(swap_device_context_state as *const ())),
                ("ClearView", Some(clear_view as *const ())),
                ("DiscardView1", Some(discard_view1 as *const ())),
            ]);
            debug_assert_eq!(s.len(), ID3D11_CONTEXT1_TOTAL_SLOTS);
            Class {
                name: "ID3D11DeviceContext1",
                vtbl: build_vtable("ID3D11DeviceContext1", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11DeviceContext, IID_ID3D11DeviceContext1],
            }
        })
    }

    fn final_release(&self) {
        if !self.deferred {
            self.device.flush_immediate();
        }
    }
}

pub type PCtx = *mut Obj<D3d11Context>;

// ---------------------------------------------------------------------------
// Binding helpers
// ---------------------------------------------------------------------------

unsafe fn set_cbs(
    bind: &mut StageBindings,
    start: u32,
    n: u32,
    bufs: *const *mut c_void,
    firsts: *const u32,
    nums: *const u32,
) {
    let mut changed = false;
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= bind.cbs.len() {
            break;
        }
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        let fc = if firsts.is_null() { 0 } else { std::ptr::read_unaligned(firsts.add(i)) };
        let nc = if nums.is_null() { 0 } else { std::ptr::read_unaligned(nums.add(i)) };
        let cur = &bind.cbs[idx];
        if cur.com.as_ptr() == p && cur.first_constant == fc && cur.num_constants == nc {
            continue;
        }
        bind.cbs[idx] = CbSlot { com: ComRef::from_borrowed(p), res: buffer_from_ptr(p), first_constant: fc, num_constants: nc };
        changed = true;
    }
    if changed {
        bind.res_gen = bind.res_gen.wrapping_add(1);
    }
}

unsafe fn get_cbs(
    slots: &[CbSlot],
    start: u32,
    n: u32,
    bufs: *mut *mut c_void,
    firsts: *mut u32,
    nums: *mut u32,
) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if let Some(s) = slots.get(idx) {
            if !bufs.is_null() {
                s.com.copy_out(bufs.add(i));
            }
            if !firsts.is_null() {
                std::ptr::write_unaligned(firsts.add(i), s.first_constant);
            }
            if !nums.is_null() {
                std::ptr::write_unaligned(nums.add(i), s.num_constants);
            }
        } else {
            if !bufs.is_null() { std::ptr::write_unaligned(bufs.add(i), std::ptr::null_mut()); }
            if !firsts.is_null() { std::ptr::write_unaligned(firsts.add(i), 0); }
            if !nums.is_null() { std::ptr::write_unaligned(nums.add(i), 0); }
        }
    }
}

unsafe fn set_srvs(bind: &mut StageBindings, start: u32, n: u32, views: *const *mut c_void) {
    let mut changed = false;
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= bind.srvs.len() { break; }
        let p = if views.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(views.add(i)) };
        if bind.srvs[idx].com.as_ptr() == p {
            continue;
        }
        let srv = Obj::<D3d11Srv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
        bind.srvs[idx] = SrvSlot { com: ComRef::from_borrowed(p), srv };
        changed = true;
    }
    if changed {
        bind.res_gen = bind.res_gen.wrapping_add(1);
    }
}

unsafe fn get_srvs(slots: &[SrvSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe fn set_samplers(bind: &mut StageBindings, start: u32, n: u32, samps: *const *mut c_void) {
    let mut changed = false;
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= bind.samplers.len() { break; }
        let p = if samps.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(samps.add(i)) };
        if bind.samplers[idx].com.as_ptr() == p {
            continue;
        }
        bind.samplers[idx] = SampSlot { com: ComRef::from_borrowed(p), samp: samp_from_ptr(p) };
        changed = true;
    }
    if changed {
        bind.samp_gen = bind.samp_gen.wrapping_add(1);
    }
}

unsafe fn get_samplers(slots: &[SampSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe fn set_uavs(
    slots: &mut [UavSlot],
    gen: &mut u64,
    start: u32,
    n: u32,
    views: *const *mut c_void,
    init_counts: *const u32,
    counter_updates: &mut Vec<(Arc<UavInner>, u32)>,
) {
    let mut changed = false;
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= slots.len() { break; }
        let p = if views.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(views.add(i)) };
        let same = slots[idx].com.as_ptr() == p;
        let uav = if same {
            slots[idx].uav.clone()
        } else {
            Obj::<D3d11Uav>::from_iface(p).map(|o| Obj::get(o).inner.clone())
        };
        if !init_counts.is_null() {
            let init = std::ptr::read_unaligned(init_counts.add(i));
            if init != D3D11_UAV_COUNTER_KEEP {
                if let Some(u) = &uav {
                    if u.counter.is_some() {
                        counter_updates.push((u.clone(), init));
                    }
                }
            }
        }
        if !same {
            slots[idx] = UavSlot { com: ComRef::from_borrowed(p), uav };
            changed = true;
        }
    }
    if changed {
        *gen = gen.wrapping_add(1);
    }
}

unsafe fn get_uavs(slots: &[UavSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

// ---------------------------------------------------------------------------
// Shader & resource setter / getter slots
// ---------------------------------------------------------------------------

unsafe extern "win64" fn vs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().vs_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn ps_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ps_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn gs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().gs_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn hs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().hs_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn ds_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ds_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn cs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().cs_bind, start, n, bufs, std::ptr::null(), std::ptr::null());
}

unsafe extern "win64" fn vs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().vs_bind, start, n, bufs, f, c);
}
unsafe extern "win64" fn ps_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ps_bind, start, n, bufs, f, c);
}
unsafe extern "win64" fn gs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().gs_bind, start, n, bufs, f, c);
}
unsafe extern "win64" fn hs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().hs_bind, start, n, bufs, f, c);
}
unsafe extern "win64" fn ds_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ds_bind, start, n, bufs, f, c);
}
unsafe extern "win64" fn cs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().cs_bind, start, n, bufs, f, c);
}

unsafe extern "win64" fn vs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn ps_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn gs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn hs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn ds_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn cs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}

unsafe extern "win64" fn vs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ps_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn gs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn hs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ds_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn cs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, f, c);
}

unsafe extern "win64" fn vs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().vs_bind, start, n, v);
}
unsafe extern "win64" fn ps_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().ps_bind, start, n, v);
}
unsafe extern "win64" fn gs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().gs_bind, start, n, v);
}
unsafe extern "win64" fn hs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().hs_bind, start, n, v);
}
unsafe extern "win64" fn ds_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().ds_bind, start, n, v);
}
unsafe extern "win64" fn cs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().cs_bind, start, n, v);
}

unsafe extern "win64" fn vs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().vs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ps_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().ps_bind.srvs, start, n, v);
}
unsafe extern "win64" fn gs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().gs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn hs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().hs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ds_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().ds_bind.srvs, start, n, v);
}
unsafe extern "win64" fn cs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().cs_bind.srvs, start, n, v);
}

unsafe extern "win64" fn vs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().vs_bind, start, n, s);
}
unsafe extern "win64" fn ps_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().ps_bind, start, n, s);
}
unsafe extern "win64" fn gs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().gs_bind, start, n, s);
}
unsafe extern "win64" fn hs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().hs_bind, start, n, s);
}
unsafe extern "win64" fn ds_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().ds_bind, start, n, s);
}
unsafe extern "win64" fn cs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().cs_bind, start, n, s);
}

unsafe extern "win64" fn vs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().vs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ps_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().ps_bind.samplers, start, n, s);
}
unsafe extern "win64" fn gs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().gs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn hs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().hs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ds_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().ds_bind.samplers, start, n, s);
}
unsafe extern "win64" fn cs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().cs_bind.samplers, start, n, s);
}

unsafe extern "win64" fn cs_set_uavs(this: PCtx, start: u32, n: u32, views: *const *mut c_void, init_counts: *const u32) {
    let ctx = Obj::get(this);
    let mut counter_updates = Vec::new();
    {
        let mut st = ctx.state.lock().unwrap();
        let st = &mut *st;
        set_uavs(&mut st.cs_uavs, &mut st.cs_uavs_gen, start, n, views, init_counts, &mut counter_updates);
    }
    for (uav, value) in counter_updates {
        ctx.emit(RecordedCmd::SetUavCounter { uav, value });
    }
}
unsafe extern "win64" fn cs_get_uavs(this: PCtx, start: u32, n: u32, views: *mut *mut c_void) {
    get_uavs(&Obj::get(this).state.lock().unwrap().cs_uavs, start, n, views);
}

unsafe extern "win64" fn vs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.vs_com.as_ptr() == shader {
        return;
    }
    st.vs_com = ComRef::from_borrowed(shader);
    st.vs = vs_from_ptr(shader);
    st.pso_dirty = true;
}
unsafe extern "win64" fn ps_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.ps_com.as_ptr() == shader {
        return;
    }
    st.ps_com = ComRef::from_borrowed(shader);
    st.ps = ps_from_ptr(shader);
    st.pso_dirty = true;
}
unsafe extern "win64" fn cs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.cs_com.as_ptr() == shader {
        return;
    }
    st.cs_com = ComRef::from_borrowed(shader);
    st.cs = cs_from_ptr(shader);
}
unsafe extern "win64" fn gs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.gs_com.as_ptr() == shader {
        return;
    }
    st.gs_com = ComRef::from_borrowed(shader);
    st.gs = gs_from_ptr(shader);
    st.pso_dirty = true;
}
unsafe extern "win64" fn hs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.hs_com.as_ptr() == shader {
        return;
    }
    st.hs_com = ComRef::from_borrowed(shader);
    st.hs = hs_from_ptr(shader);
    st.pso_dirty = true;
}
unsafe extern "win64" fn ds_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.ds_com.as_ptr() == shader {
        return;
    }
    st.ds_com = ComRef::from_borrowed(shader);
    st.ds = domain_shader_from_ptr(shader);
    st.pso_dirty = true;
}

unsafe extern "win64" fn vs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().vs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn ps_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().ps_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn cs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().cs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn gs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().gs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn hs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().hs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn ds_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().ds_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}

// ---------------------------------------------------------------------------
// Input Assembler, Output Merger, Rasterizer state
// ---------------------------------------------------------------------------

unsafe extern "win64" fn ia_set_input_layout(this: PCtx, il: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.input_layout_com.as_ptr() == il {
        return;
    }
    st.input_layout_com = ComRef::from_borrowed(il);
    st.input_layout = il_from_ptr(il);
    st.pso_dirty = true;
}
unsafe extern "win64" fn ia_get_input_layout(this: PCtx, out: *mut *mut c_void) {
    Obj::get(this).state.lock().unwrap().input_layout_com.copy_out(out);
}

unsafe extern "win64" fn ia_set_vertex_buffers(
    this: PCtx,
    start: u32,
    n: u32,
    bufs: *const *mut c_void,
    strides: *const u32,
    offsets: *const u32,
) {
    let mut st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= st.vbs.len() { break; }
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        let stride = if strides.is_null() { 0 } else { std::ptr::read_unaligned(strides.add(i)) };
        let offset = if offsets.is_null() { 0 } else { std::ptr::read_unaligned(offsets.add(i)) };
        let cur = &st.vbs[idx];
        if cur.com.as_ptr() == p && cur.stride == stride && cur.offset == offset {
            continue;
        }
        st.vbs[idx] = VbSlot { com: ComRef::from_borrowed(p), res: buffer_from_ptr(p), stride, offset };
    }
}
unsafe extern "win64" fn ia_get_vertex_buffers(
    this: PCtx,
    start: u32,
    n: u32,
    bufs: *mut *mut c_void,
    strides: *mut u32,
    offsets: *mut u32,
) {
    let st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        if let Some(vb) = st.vbs.get(start as usize + i) {
            if !bufs.is_null() { vb.com.copy_out(bufs.add(i)); }
            if !strides.is_null() { std::ptr::write_unaligned(strides.add(i), vb.stride); }
            if !offsets.is_null() { std::ptr::write_unaligned(offsets.add(i), vb.offset); }
        } else {
            if !bufs.is_null() { std::ptr::write_unaligned(bufs.add(i), std::ptr::null_mut()); }
            if !strides.is_null() { std::ptr::write_unaligned(strides.add(i), 0); }
            if !offsets.is_null() { std::ptr::write_unaligned(offsets.add(i), 0); }
        }
    }
}

unsafe extern "win64" fn ia_set_index_buffer(this: PCtx, buf: *mut c_void, fmt: u32, offset: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.ib_format != fmt {
        st.ib_format = fmt;
        st.pso_dirty = true;
    }
    st.ib_offset = offset;
    if st.ib_com.as_ptr() != buf {
        st.ib_com = ComRef::from_borrowed(buf);
        st.ib = buffer_from_ptr(buf);
    }
}
unsafe extern "win64" fn ia_get_index_buffer(this: PCtx, out: *mut *mut c_void, fmt: *mut u32, offset: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.ib_com.copy_out(out);
    if !fmt.is_null() { std::ptr::write_unaligned(fmt, st.ib_format); }
    if !offset.is_null() { std::ptr::write_unaligned(offset, st.ib_offset); }
}

unsafe extern "win64" fn ia_set_primitive_topology(this: PCtx, topo: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    let old = st.topology;
    if old != topo {
        st.topology = topo;
        if d3d11_topology_to_type(old) != d3d11_topology_to_type(topo) {
            st.pso_dirty = true;
        }
    }
}
unsafe extern "win64" fn ia_get_primitive_topology(this: PCtx, out: *mut u32) {
    if !out.is_null() { std::ptr::write_unaligned(out, Obj::get(this).state.lock().unwrap().topology); }
}

unsafe extern "win64" fn om_set_render_targets(this: PCtx, n: u32, rtvs: *const *mut c_void, dsv: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    let count = (n as usize).min(st.rtvs.len());
    let mut changed = false;
    for i in 0..st.rtvs.len() {
        let p = if i < count && !rtvs.is_null() {
            std::ptr::read_unaligned(rtvs.add(i))
        } else {
            std::ptr::null_mut()
        };
        if st.rtvs[i].com.as_ptr() != p {
            let rtv = Obj::<D3d11Rtv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
            st.rtvs[i] = RtvSlot { com: ComRef::from_borrowed(p), rtv };
            changed = true;
        }
    }
    if st.dsv_com.as_ptr() != dsv {
        st.dsv_com = ComRef::from_borrowed(dsv);
        st.dsv = Obj::<D3d11Dsv>::from_iface(dsv).map(|o| Obj::get(o).inner.clone());
        changed = true;
    }
    if changed {
        st.rt_dirty = true;
        st.pso_dirty = true;
    }
}

unsafe extern "win64" fn om_set_render_targets_and_uavs(
    this: PCtx,
    num_rtvs: u32,
    rtvs: *const *mut c_void,
    dsv: *mut c_void,
    uav_start: u32,
    num_uavs: u32,
    uavs: *const *mut c_void,
    init_counts: *const u32,
) {
    let ctx = Obj::get(this);
    let mut counter_updates = Vec::new();
    {
        let mut st = ctx.state.lock().unwrap();
        let st = &mut *st;
        // D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL = 0xFFFFFFFF
        if num_rtvs != u32::MAX {
            let count = (num_rtvs as usize).min(st.rtvs.len());
            let mut changed = false;
            for i in 0..st.rtvs.len() {
                let p = if i < count && !rtvs.is_null() {
                    std::ptr::read_unaligned(rtvs.add(i))
                } else {
                    std::ptr::null_mut()
                };
                if st.rtvs[i].com.as_ptr() != p {
                    let rtv = Obj::<D3d11Rtv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
                    st.rtvs[i] = RtvSlot { com: ComRef::from_borrowed(p), rtv };
                    changed = true;
                }
            }
            if st.dsv_com.as_ptr() != dsv {
                st.dsv_com = ComRef::from_borrowed(dsv);
                st.dsv = Obj::<D3d11Dsv>::from_iface(dsv).map(|o| Obj::get(o).inner.clone());
                changed = true;
            }
            if changed {
                st.rt_dirty = true;
                st.pso_dirty = true;
            }
        }
        // D3D11_KEEP_UNORDERED_ACCESS_VIEWS = 0xFFFFFFFF
        if num_uavs != u32::MAX {
            let mut cleared_any = false;
            for (i, slot) in st.om_uavs.iter_mut().enumerate() {
                let in_range = i >= uav_start as usize && i < (uav_start as usize).saturating_add(num_uavs as usize);
                if !in_range && !slot.com.as_ptr().is_null() {
                    *slot = UavSlot::default();
                    cleared_any = true;
                }
            }
            if cleared_any {
                st.om_uavs_gen = st.om_uavs_gen.wrapping_add(1);
            }
            set_uavs(&mut st.om_uavs, &mut st.om_uavs_gen, uav_start, num_uavs, uavs, init_counts, &mut counter_updates);
        }
    }
    for (uav, value) in counter_updates {
        ctx.emit(RecordedCmd::SetUavCounter { uav, value });
    }
}

unsafe extern "win64" fn om_get_render_targets(this: PCtx, n: u32, rtvs: *mut *mut c_void, dsv: *mut *mut c_void) {
    let st = Obj::get(this).state.lock().unwrap();
    if !rtvs.is_null() {
        for i in 0..n as usize {
            if let Some(s) = st.rtvs.get(i) {
                s.com.copy_out(rtvs.add(i));
            } else {
                std::ptr::write_unaligned(rtvs.add(i), std::ptr::null_mut());
            }
        }
    }
    st.dsv_com.copy_out(dsv);
}

unsafe extern "win64" fn om_get_render_targets_and_uavs(
    this: PCtx,
    n_rtvs: u32,
    rtvs: *mut *mut c_void,
    dsv: *mut *mut c_void,
    uav_start: u32,
    n_uavs: u32,
    uavs: *mut *mut c_void,
) {
    om_get_render_targets(this, n_rtvs, rtvs, dsv);
    get_uavs(&Obj::get(this).state.lock().unwrap().om_uavs, uav_start, n_uavs, uavs);
}

unsafe extern "win64" fn om_set_blend_state(this: PCtx, state: *mut c_void, factor: *const [f32; 4], sample_mask: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.blend_com.as_ptr() != state {
        let old_id = st.blend.as_ref().map_or(0, |b| b.id);
        let next = blend_from_ptr(state);
        let new_id = next.as_ref().map_or(0, |b| b.id);
        st.blend_com = ComRef::from_borrowed(state);
        st.blend = next;
        if old_id != new_id {
            st.pso_dirty = true;
        }
    }
    let next_factor = if factor.is_null() { [1.0; 4] } else { std::ptr::read_unaligned(factor) };
    if st.blend_factor != next_factor {
        st.blend_factor = next_factor;
        st.blend_factor_dirty = true;
    }
    if st.sample_mask != sample_mask {
        st.sample_mask = sample_mask;
        st.pso_dirty = true;
    }
}
unsafe extern "win64" fn om_get_blend_state(this: PCtx, out: *mut *mut c_void, factor: *mut [f32; 4], mask: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.blend_com.copy_out(out);
    if !factor.is_null() { std::ptr::write_unaligned(factor, st.blend_factor); }
    if !mask.is_null() { std::ptr::write_unaligned(mask, st.sample_mask); }
}

unsafe extern "win64" fn om_set_depth_stencil_state(this: PCtx, state: *mut c_void, stencil_ref: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.ds_state_com.as_ptr() != state {
        let old_id = st.ds_state.as_ref().map_or(0, |d| d.id);
        let next = ds_from_ptr(state);
        let new_id = next.as_ref().map_or(0, |d| d.id);
        st.ds_state_com = ComRef::from_borrowed(state);
        st.ds_state = next;
        if old_id != new_id {
            st.pso_dirty = true;
        }
    }
    if st.stencil_ref != stencil_ref {
        st.stencil_ref = stencil_ref;
        st.stencil_ref_dirty = true;
    }
}
unsafe extern "win64" fn om_get_depth_stencil_state(this: PCtx, out: *mut *mut c_void, stencil_ref: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.ds_state_com.copy_out(out);
    if !stencil_ref.is_null() { std::ptr::write_unaligned(stencil_ref, st.stencil_ref); }
}

unsafe extern "win64" fn rs_set_state(this: PCtx, state: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    if st.rast_com.as_ptr() == state {
        return;
    }
    let old_id = st.rast.as_ref().map_or(0, |r| r.id);
    let old_scissor = st.rast.as_ref().is_some_and(|r| r.scissor_enable);
    let next = rast_from_ptr(state);
    let new_id = next.as_ref().map_or(0, |r| r.id);
    let new_scissor = next.as_ref().is_some_and(|r| r.scissor_enable);
    st.rast_com = ComRef::from_borrowed(state);
    st.rast = next;
    if old_id != new_id {
        st.pso_dirty = true;
    }
    if old_scissor != new_scissor {
        st.scissor_dirty = true;
    }
}
unsafe extern "win64" fn rs_get_state(this: PCtx, out: *mut *mut c_void) {
    Obj::get(this).state.lock().unwrap().rast_com.copy_out(out);
}

unsafe extern "win64" fn rs_set_viewports(this: PCtx, n: u32, vps: *const D3D11_VIEWPORT) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.viewports.clear();
    if n > 0 && !vps.is_null() {
        let k = (n as usize).min(D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE);
        for i in 0..k {
            st.viewports.push(std::ptr::read_unaligned(vps.add(i)));
        }
    }
    st.vp_dirty = true;
}
unsafe extern "win64" fn rs_get_viewports(this: PCtx, n: *mut u32, out: *mut D3D11_VIEWPORT) {
    if n.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    if out.is_null() {
        std::ptr::write_unaligned(n, st.viewports.len() as u32);
        return;
    }
    let req = std::ptr::read_unaligned(n) as usize;
    let copy = req.min(st.viewports.len());
    for i in 0..copy {
        std::ptr::write_unaligned(out.add(i), st.viewports[i]);
    }
    for i in copy..req {
        std::ptr::write_unaligned(out.add(i), D3D11_VIEWPORT::default());
    }
    std::ptr::write_unaligned(n, copy as u32);
}

unsafe extern "win64" fn rs_set_scissor_rects(this: PCtx, n: u32, rects: *const Rect) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.scissors.clear();
    if n > 0 && !rects.is_null() {
        let k = (n as usize).min(D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE);
        for i in 0..k {
            st.scissors.push(std::ptr::read_unaligned(rects.add(i)));
        }
    }
    st.scissor_dirty = true;
}
unsafe extern "win64" fn rs_get_scissor_rects(this: PCtx, n: *mut u32, out: *mut Rect) {
    if n.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    if out.is_null() {
        std::ptr::write_unaligned(n, st.scissors.len() as u32);
        return;
    }
    let req = std::ptr::read_unaligned(n) as usize;
    let copy = req.min(st.scissors.len());
    for i in 0..copy {
        std::ptr::write_unaligned(out.add(i), st.scissors[i]);
    }
    for i in copy..req {
        std::ptr::write_unaligned(out.add(i), Rect::default());
    }
    std::ptr::write_unaligned(n, copy as u32);
}

unsafe extern "win64" fn so_set_targets(this: PCtx, n: u32, bufs: *const *mut c_void, _offsets: *const u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    for s in &mut st.so_targets { *s = ComRef::default(); }
    for i in 0..(n as usize).min(st.so_targets.len()) {
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        st.so_targets[i] = ComRef::from_borrowed(p);
    }
}
unsafe extern "win64" fn so_get_targets(this: PCtx, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        if let Some(s) = st.so_targets.get(i) {
            s.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe extern "win64" fn set_predication(this: PCtx, pred: *mut c_void, val: BOOL) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.predicate_com = ComRef::from_borrowed(pred);
    st.predicate_val = val;
}
unsafe extern "win64" fn get_predication(this: PCtx, out: *mut *mut c_void, val: *mut BOOL) {
    let st = Obj::get(this).state.lock().unwrap();
    st.predicate_com.copy_out(out);
    if !val.is_null() { std::ptr::write_unaligned(val, st.predicate_val); }
}

// ---------------------------------------------------------------------------
// Map / Unmap / UpdateSubresource / CopyResource / CopySubresourceRegion
// ---------------------------------------------------------------------------

unsafe extern "win64" fn map(
    this: PCtx,
    res_ptr: *mut c_void,
    sub: u32,
    mtype: u32,
    _flags: u32,
    out: *mut D3D11_MAPPED_SUBRESOURCE,
) -> HRESULT {
    let Some(res) = resource_from_ptr(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "ID3D11DeviceContext::Map: null/foreign resource {res_ptr:?}");
    };
    let ctx = Obj::get(this);
    let writable = mtype != map_type::READ;

    if ctx.deferred {
        if !matches!(mtype, map_type::WRITE_DISCARD | map_type::WRITE_NO_OVERWRITE) {
            return d11_failed!(E_INVALIDARG, "DeferredContext::Map: only WRITE_DISCARD / WRITE_NO_OVERWRITE allowed");
        }
        let (size, row_pitch, depth_pitch) = if res.is_buffer() {
            let w = res.kind.buffer_width() as usize;
            (w.max(1), w as u32, w as u32)
        } else {
            let mtl = res.current_mtl();
            let Some((lays, _)) = d3d12_metal::layout::copyable_footprints(&mtl.desc, sub, 1, 0) else {
                return d11_failed!(E_INVALIDARG, "DeferredContext::Map: invalid subresource {sub}");
            };
            let lay = lays[0];
            let dp = lay.footprint.row_pitch * lay.num_rows;
            ((dp * lay.footprint.depth.max(1)) as usize, lay.footprint.row_pitch, dp)
        };
        let mut buf = vec![0u8; size];
        let ptr = buf.as_mut_ptr() as *mut c_void;
        res.mapped.lock().unwrap().insert(sub, MapState {
            map_type: mtype,
            writable: true,
            deferred_scratch: Some(buf),
            row_pitch,
            depth_pitch,
        });
        if !out.is_null() {
            std::ptr::write_unaligned(
                out,
                D3D11_MAPPED_SUBRESOURCE { pData: ptr, RowPitch: row_pitch, DepthPitch: depth_pitch },
            );
        }
        return S_OK;
    }

    // Immediate context Map.
    res.sync_other_devices(&ctx.device);
    if res.is_buffer() {
        let byte_width = res.kind.buffer_width();
        let ptr: *mut u8 = if mtype == map_type::WRITE_DISCARD {
            if let Some(bw) = res.ring_dynamic_byte_width() {
                let (mtl, offset, cpu_ptr) = {
                    let mut exec = ctx.device.imm_exec.lock().unwrap();
                    match exec.alloc_upload_slice(bw as usize, 256) {
                        Ok(slice) => slice,
                        Err(e) => return d11_failed!(E_OUTOFMEMORY, "Map(WRITE_DISCARD) ring: {e}"),
                    }
                };
                let _old = res.replace_backing(ResourceBacking { mtl, offset });
                // `_old` is either a ring chunk already retained by `imm_exec`/`queue` or the initial
                // buffer; if a command buffer references it, `keep_mtl` already holds an `Arc`.
                cpu_ptr
            } else if let Some(class_size) = res.pool_dynamic_class_size() {
                let completed = ctx.device.queue.timeline.completed();
                let fresh = match ctx.device.dynamic_pool.lock().unwrap().acquire(class_size, completed) {
                    Ok(buf) => buf,
                    Err(e) => return d11_failed!(E_OUTOFMEMORY, "Map(WRITE_DISCARD) pool: {e}"),
                };
                let Some(cpu_ptr) = fresh.cpu_ptr() else {
                    return d11_failed!(E_INVALIDARG, "Map: pooled buffer has no cpu_ptr");
                };
                let old = res.replace_backing(ResourceBacking { mtl: fresh, offset: 0 });
                if old.offset == 0 && old.mtl.desc.Width == class_size {
                    let mut exec = ctx.device.imm_exec.lock().unwrap();
                    exec.defer_pool_return(class_size, old.mtl);
                } else {
                    ctx.device.queue.defer_release(Box::new(old.mtl));
                }
                cpu_ptr
            } else {
                let fresh = match res.alloc_fresh_buffer_mtl() {
                    Ok(f) => f,
                    Err(e) => return d11_failed!(E_OUTOFMEMORY, "Map(WRITE_DISCARD): {e}"),
                };
                let Some(cpu_ptr) = fresh.cpu_ptr() else {
                    return d11_failed!(E_INVALIDARG, "Map: buffer {:?} is not CPU-accessible (usage={})", res.kind, res.kind.usage());
                };
                let old = res.replace_backing(ResourceBacking { mtl: fresh, offset: 0 });
                ctx.device.queue.defer_release(Box::new(old.mtl));
                cpu_ptr
            }
        } else {
            if mtype != map_type::WRITE_NO_OVERWRITE {
                ctx.device.flush_and_wait_idle();
            }
            let b = res.current_backing();
            let Some(base) = b.mtl.cpu_ptr() else {
                return d11_failed!(E_INVALIDARG, "Map: buffer {:?} is not CPU-accessible (usage={})", res.kind, res.kind.usage());
            };
            base.add(b.offset as usize)
        };
        res.mapped.lock().unwrap().insert(sub, MapState {
            map_type: mtype,
            writable,
            deferred_scratch: None,
            row_pitch: byte_width,
            depth_pitch: byte_width,
        });
        if !out.is_null() {
            std::ptr::write_unaligned(
                out,
                D3D11_MAPPED_SUBRESOURCE {
                    pData: ptr as *mut c_void,
                    RowPitch: byte_width,
                    DepthPitch: byte_width,
                },
            );
        }
        return S_OK;
    }

    // Texture Map via staging buffer.
    let had_staging = res.tex_staging.lock().unwrap().is_some();
    let (staging_buf, layouts) = match res.ensure_tex_staging() {
        Ok(x) => x,
        Err(e) => return d11_failed!(E_FAIL, "Map texture staging: {e}"),
    };
    let Some(lay) = layouts.get(sub as usize).copied() else {
        return d11_failed!(E_INVALIDARG, "Map texture: subresource {sub} out of range ({})", layouts.len());
    };
    if matches!(mtype, map_type::READ | map_type::READ_WRITE) {
        let mtl = res.current_mtl();
        {
            let mut exec = ctx.device.imm_exec.lock().unwrap();
            if let Err(e) = exec.ensure_open() {
                return d11_failed!(E_FAIL, "Map texture readback ensure_open: {e}");
            }
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_texture_region(
                CopyLoc::Footprint(&staging_buf, lay.footprint),
                [0, 0, 0],
                CopyLoc::Subresource(&mtl, sub),
                None,
            ) {
                return d11_failed!(E_FAIL, "Map texture readback copy: {e}");
            }
            exec.has_work = true;
            exec.keep((mtl, staging_buf.clone()));
        }
        ctx.device.flush_and_wait_idle();
    } else if had_staging && mtype != map_type::WRITE_NO_OVERWRITE {
        // Ensure any earlier GPU upload from `staging_buf` has completed before CPU overwrites it.
        ctx.device.flush_and_wait_idle();
    }
    let Some(base_ptr) = staging_buf.cpu_ptr() else {
        return d11_failed!(E_FAIL, "Map texture: staging buffer has no cpu_ptr");
    };
    let ptr = base_ptr.add(lay.footprint.offset as usize) as *mut c_void;
    let row_pitch = lay.footprint.row_pitch;
    let depth_pitch = lay.footprint.row_pitch * lay.num_rows;
    res.mapped.lock().unwrap().insert(sub, MapState {
        map_type: mtype,
        writable,
        deferred_scratch: None,
        row_pitch,
        depth_pitch,
    });
    if !out.is_null() {
        std::ptr::write_unaligned(
            out,
            D3D11_MAPPED_SUBRESOURCE { pData: ptr, RowPitch: row_pitch, DepthPitch: depth_pitch },
        );
    }
    S_OK
}

unsafe extern "win64" fn unmap(this: PCtx, res_ptr: *mut c_void, sub: u32) {
    let Some(res) = resource_from_ptr(res_ptr) else {
        return;
    };
    let Some(ms) = res.mapped.lock().unwrap().remove(&sub) else {
        return;
    };
    let ctx = Obj::get(this);
    if let Some(data) = ms.deferred_scratch {
        ctx.emit(RecordedCmd::UpdateSubresource {
            dst: res,
            dst_sub: sub,
            dst_box: None,
            data,
            src_row_pitch: ms.row_pitch,
            src_depth_pitch: ms.depth_pitch,
        });
        return;
    }
    if !res.is_buffer() && ms.writable {
        if let Ok((staging_buf, layouts)) = res.ensure_tex_staging() {
            if let Some(lay) = layouts.get(sub as usize).copied() {
                let mtl = res.current_mtl();
                let mut exec = ctx.device.imm_exec.lock().unwrap();
                if exec.ensure_open().is_ok() {
                    exec.cl.barrier();
                    if let Err(e) = exec.cl.copy_texture_region(
                        CopyLoc::Subresource(&mtl, sub),
                        [0, 0, 0],
                        CopyLoc::Footprint(&staging_buf, lay.footprint),
                        None,
                    ) {
                        log::warn!(target: "d3d", "d3d11 Unmap texture upload: {e}");
                    } else {
                        exec.cl.barrier();
                        res.last_gpu_state.store(gpu_state::COPY_DEST, Ordering::Release);
                        exec.has_work = true;
                    }
                    exec.keep_mtl(&mtl);
                    exec.keep_mtl(&staging_buf);
                }
            }
        }
    }
}

unsafe extern "win64" fn copy_resource(this: PCtx, dst: *mut c_void, src: *mut c_void) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        log::warn!(target: "d3d", "d3d11 CopyResource: null/foreign resource ({dst:?}, {src:?})");
        return;
    };
    Obj::get(this).emit(RecordedCmd::CopyResource { dst: d, src: s });
}

unsafe extern "win64" fn copy_subresource_region(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_x: u32,
    dst_y: u32,
    dst_z: u32,
    src: *mut c_void,
    src_sub: u32,
    src_box: *const D3D11_BOX,
) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        log::warn!(target: "d3d", "d3d11 CopySubresourceRegion: null/foreign resource");
        return;
    };
    let bx = if src_box.is_null() { None } else { Some(std::ptr::read_unaligned(src_box)) };
    Obj::get(this).emit(RecordedCmd::CopySubresourceRegion {
        dst: d,
        dst_sub,
        dst_xyz: [dst_x, dst_y, dst_z],
        src: s,
        src_sub,
        src_box: bx,
    });
}

unsafe extern "win64" fn copy_subresource_region1(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_x: u32,
    dst_y: u32,
    dst_z: u32,
    src: *mut c_void,
    src_sub: u32,
    src_box: *const D3D11_BOX,
    _copy_flags: u32,
) {
    copy_subresource_region(this, dst, dst_sub, dst_x, dst_y, dst_z, src, src_sub, src_box);
}

unsafe extern "win64" fn update_subresource(
    this: PCtx,
    dst_ptr: *mut c_void,
    dst_sub: u32,
    dst_box: *const D3D11_BOX,
    src_data: *const c_void,
    src_row_pitch: u32,
    src_depth_pitch: u32,
) {
    if src_data.is_null() {
        return;
    }
    let Some(dst) = resource_from_ptr(dst_ptr) else {
        log::warn!(target: "d3d", "d3d11 UpdateSubresource: null/foreign dst {dst_ptr:?}");
        return;
    };
    let bx = if dst_box.is_null() { None } else { Some(std::ptr::read_unaligned(dst_box)) };
    let ctx = Obj::get(this);
    if !ctx.deferred {
        // Immediate fast path: single copy directly into the resident upload ring (zero Vec<u8> alloc).
        dst.sync_other_devices(&ctx.device);
        let mut exec = ctx.device.imm_exec.lock().unwrap();
        if let Err(e) = exec.ensure_open() {
            log::warn!(target: "d3d", "d3d11 UpdateSubresource ensure_open: {e}");
            return;
        }
        if let Err(e) = exec_update_subresource_raw(
            &mut exec,
            &dst,
            dst_sub,
            bx,
            src_data as *const u8,
            None,
            src_row_pitch,
            src_depth_pitch,
        ) {
            log::warn!(target: "d3d", "d3d11 UpdateSubresource: {e}");
        }
        return;
    }
    let data = if dst.is_buffer() {
        let len = match bx {
            Some(b) => b.right.saturating_sub(b.left) as usize,
            None => dst.kind.buffer_width() as usize,
        };
        std::slice::from_raw_parts(src_data as *const u8, len).to_vec()
    } else {
        let mtl = dst.current_mtl();
        let Some((lays, _)) = d3d12_metal::layout::copyable_footprints(&mtl.desc, dst_sub, 1, 0) else {
            return;
        };
        let lay = lays[0];
        let fi = d3d12_metal::format::info(mtl.desc.Format);
        let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
        let bpe = fi.bytes.max(1) as usize;
        let (w, h, d) = match bx {
            Some(b) => (b.right.saturating_sub(b.left), b.bottom.saturating_sub(b.top), b.back.saturating_sub(b.front)),
            None => (lay.footprint.width, lay.footprint.height, lay.footprint.depth.max(1)),
        };
        if w == 0 || h == 0 || d == 0 {
            return;
        }
        let row_bytes = (w.div_ceil(bw) as usize) * bpe;
        let rows = h.div_ceil(bh) as usize;
        let srp = if src_row_pitch == 0 { row_bytes } else { src_row_pitch as usize };
        let sdp = if src_depth_pitch == 0 { srp * rows } else { src_depth_pitch as usize };
        let total = (d as usize - 1) * sdp + (rows - 1) * srp + row_bytes;
        std::slice::from_raw_parts(src_data as *const u8, total).to_vec()
    };
    ctx.emit(RecordedCmd::UpdateSubresource {
        dst,
        dst_sub,
        dst_box: bx,
        data,
        src_row_pitch,
        src_depth_pitch,
    });
}

unsafe extern "win64" fn update_subresource1(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_box: *const D3D11_BOX,
    src_data: *const c_void,
    src_row_pitch: u32,
    src_depth_pitch: u32,
    _copy_flags: u32,
) {
    update_subresource(this, dst, dst_sub, dst_box, src_data, src_row_pitch, src_depth_pitch);
}

// ---------------------------------------------------------------------------
// Clears, Draws, Dispatches, Queries, Command Lists
// ---------------------------------------------------------------------------

unsafe extern "win64" fn clear_render_target_view(this: PCtx, rtv_ptr: *mut c_void, color: *const [f32; 4]) {
    let Some(o) = Obj::<D3d11Rtv>::from_iface(rtv_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearRenderTargetView: null/foreign RTV {rtv_ptr:?}");
        return;
    };
    let c = if color.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(color) };
    Obj::get(this).emit(RecordedCmd::ClearRtv {
        rtv: Obj::get(o).inner.clone(),
        color: c,
        rects: Vec::new(),
    });
}

pub unsafe extern "win64" fn clear_depth_stencil_view(
    this: PCtx,
    dsv_ptr: *mut c_void,
    flags: u32,
    depth: f32,
    stencil: u8,
) {
    let Some(o) = Obj::<D3d11Dsv>::from_iface(dsv_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearDepthStencilView: null/foreign DSV {dsv_ptr:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::ClearDsv {
        dsv: Obj::get(o).inner.clone(),
        flags,
        depth,
        stencil,
        rects: Vec::new(),
    });
}
crate::win64_bridge!(fn clear_depth_stencil_view(PCtx, *mut c_void, u32, f32, u8) -> ());

unsafe extern "win64" fn clear_uav_uint(this: PCtx, uav_ptr: *mut c_void, vals: *const [u32; 4]) {
    let Some(o) = Obj::<D3d11Uav>::from_iface(uav_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewUint: null/foreign UAV {uav_ptr:?}");
        return;
    };
    let values = if vals.is_null() { [0; 4] } else { std::ptr::read_unaligned(vals) };
    Obj::get(this).emit(RecordedCmd::ClearUavUint { uav: Obj::get(o).inner.clone(), values });
}

unsafe extern "win64" fn clear_uav_float(this: PCtx, uav_ptr: *mut c_void, vals: *const [f32; 4]) {
    let Some(o) = Obj::<D3d11Uav>::from_iface(uav_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewFloat: null/foreign UAV {uav_ptr:?}");
        return;
    };
    let values = if vals.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(vals) };
    Obj::get(this).emit(RecordedCmd::ClearUavFloat { uav: Obj::get(o).inner.clone(), values });
}

unsafe extern "win64" fn clear_view(
    this: PCtx,
    view: *mut c_void,
    color: *const [f32; 4],
    rects: *const Rect,
    num_rects: u32,
) {
    let c = if color.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(color) };
    let mut rs = Vec::new();
    if num_rects > 0 && !rects.is_null() {
        rs.reserve(num_rects as usize);
        for i in 0..num_rects as usize {
            rs.push(std::ptr::read_unaligned(rects.add(i)));
        }
    }
    if let Some(o) = Obj::<D3d11Rtv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearRtv { rtv: Obj::get(o).inner.clone(), color: c, rects: rs });
    } else if let Some(o) = Obj::<D3d11Dsv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearDsv {
            dsv: Obj::get(o).inner.clone(),
            flags: clear_flag::DEPTH,
            depth: c[0],
            stencil: 0,
            rects: rs,
        });
    } else if let Some(o) = Obj::<D3d11Uav>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearUavFloat { uav: Obj::get(o).inner.clone(), values: c });
    }
}

unsafe extern "win64" fn draw(this: PCtx, vertex_count: u32, start_vertex: u32) {
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::Draw { state: st, vertex_count, instance_count: 1, start_vertex, start_instance: 0 });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 Draw ensure_open: {e}");
        return;
    }
    match exec_draw(&ctx.device, &mut exec, &st, true, None, vertex_count, 1, start_vertex, 0, 0) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 Draw({vertex_count}, 1): {e}"),
    }
}

unsafe extern "win64" fn draw_instanced(
    this: PCtx,
    vertex_count: u32,
    instance_count: u32,
    start_vertex: u32,
    start_instance: u32,
) {
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::Draw { state: st, vertex_count, instance_count, start_vertex, start_instance });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DrawInstanced ensure_open: {e}");
        return;
    }
    match exec_draw(&ctx.device, &mut exec, &st, true, None, vertex_count, instance_count, start_vertex, 0, start_instance) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 Draw({vertex_count}, {instance_count}): {e}"),
    }
}

unsafe extern "win64" fn draw_indexed(this: PCtx, index_count: u32, start_index: u32, base_vertex: i32) {
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::DrawIndexed {
            state: st,
            index_count,
            instance_count: 1,
            start_index,
            base_vertex,
            start_instance: 0,
        });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DrawIndexed ensure_open: {e}");
        return;
    }
    match exec_draw(&ctx.device, &mut exec, &st, true, Some(start_index), index_count, 1, 0, base_vertex, 0) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 DrawIndexed({index_count}, 1): {e}"),
    }
}

unsafe extern "win64" fn draw_indexed_instanced(
    this: PCtx,
    index_count: u32,
    instance_count: u32,
    start_index: u32,
    base_vertex: i32,
    start_instance: u32,
) {
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::DrawIndexed {
            state: st,
            index_count,
            instance_count,
            start_index,
            base_vertex,
            start_instance,
        });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DrawIndexedInstanced ensure_open: {e}");
        return;
    }
    match exec_draw(&ctx.device, &mut exec, &st, true, Some(start_index), index_count, instance_count, 0, base_vertex, start_instance) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 DrawIndexed({index_count}, {instance_count}): {e}"),
    }
}

unsafe extern "win64" fn dispatch(this: PCtx, x: u32, y: u32, z: u32) {
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::Dispatch { state: st, x, y, z });
        return;
    }
    let st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 Dispatch ensure_open: {e}");
        return;
    }
    if let Err(e) = exec_dispatch(&ctx.device, &mut exec, &st, true, x, y, z) {
        log::warn!(target: "d3d", "d3d11 Dispatch({x}, {y}, {z}): {e}");
    }
}

unsafe extern "win64" fn draw_auto(_this: PCtx) {
    stub!("ID3D11DeviceContext::DrawAuto not implemented");
}
unsafe extern "win64" fn draw_instanced_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::DrawInstancedIndirect { state: st, args, offset: off });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    args.sync_other_devices(&ctx.device);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect ensure_open: {e}");
        return;
    }
    match exec_draw_indirect(&ctx.device, &mut exec, &st, true, &args, off, false) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect(off={off}): {e}"),
    }
}
unsafe extern "win64" fn draw_indexed_instanced_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::DrawIndexedInstancedIndirect { state: st, args, offset: off });
        return;
    }
    let mut st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    args.sync_other_devices(&ctx.device);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect ensure_open: {e}");
        return;
    }
    match exec_draw_indirect(&ctx.device, &mut exec, &st, true, &args, off, true) {
        Ok(true) => st.clear_gfx_dirty(),
        Ok(false) => {}
        Err(e) => log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect(off={off}): {e}"),
    }
}
unsafe extern "win64" fn dispatch_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DispatchIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    if ctx.deferred {
        let st = Box::new(ctx.state.lock().unwrap().snapshot_for_draw());
        ctx.emit(RecordedCmd::DispatchIndirect { state: st, args, offset: off });
        return;
    }
    let st = ctx.state.lock().unwrap();
    sync_state_shared(&ctx.device, &st);
    args.sync_other_devices(&ctx.device);
    let mut exec = ctx.device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 DispatchIndirect ensure_open: {e}");
        return;
    }
    if let Err(e) = exec_dispatch_indirect(&ctx.device, &mut exec, &st, true, &args, off) {
        log::warn!(target: "d3d", "d3d11 DispatchIndirect(off={off}): {e}");
    }
}
unsafe extern "win64" fn copy_structure_count(this: PCtx, dst: *mut c_void, off: u32, uav: *mut c_void) {
    let Some(dst_buf) = buffer_from_ptr(dst) else {
        log::warn!(target: "d3d", "d3d11 CopyStructureCount: null/foreign dst buffer {dst:?}");
        return;
    };
    let Some(uav_obj) = Obj::<D3d11Uav>::from_iface(uav) else {
        log::warn!(target: "d3d", "d3d11 CopyStructureCount: null/foreign UAV {uav:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::CopyStructureCount {
        dst: dst_buf,
        dst_offset: off,
        uav: Obj::get(uav_obj).inner.clone(),
    });
}
unsafe extern "win64" fn generate_mips(this: PCtx, srv: *mut c_void) {
    let Some(srv_obj) = Obj::<D3d11Srv>::from_iface(srv) else {
        log::warn!(target: "d3d", "d3d11 GenerateMips: null/foreign SRV {srv:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::GenerateMips { srv: Obj::get(srv_obj).inner.clone() });
}

pub unsafe extern "win64" fn set_resource_min_lod(_this: PCtx, res: *mut c_void, min_lod: f32) {
    if let Some(r) = resource_from_ptr(res) {
        let bits = min_lod.to_bits();
        if r.min_lod_bits.swap(bits, Ordering::AcqRel) != bits {
            r.version.fetch_add(1, Ordering::Release);
            r.notify_desc_resource_changed();
        }
    }
}
crate::win64_bridge!(fn set_resource_min_lod(PCtx, *mut c_void, f32) -> ());

unsafe extern "win64" fn get_resource_min_lod(_this: PCtx, res: *mut c_void) -> f32 {
    resource_from_ptr(res)
        .map(|r| f32::from_bits(r.min_lod_bits.load(std::sync::atomic::Ordering::Relaxed)))
        .unwrap_or(0.0)
}

unsafe extern "win64" fn resolve_subresource(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    src: *mut c_void,
    src_sub: u32,
    format: u32,
) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        return;
    };
    Obj::get(this).emit(RecordedCmd::ResolveSubresource { dst: d, dst_sub, src: s, src_sub, format });
}

unsafe extern "win64" fn begin_async(this: PCtx, async_ptr: *mut c_void) {
    let Some(q) = query_from_ptr(async_ptr) else { return };
    Obj::get(this).emit(RecordedCmd::BeginQuery(q));
}

unsafe extern "win64" fn end_async(this: PCtx, async_ptr: *mut c_void) {
    let Some(q) = query_from_ptr(async_ptr) else { return };
    Obj::get(this).emit(RecordedCmd::EndQuery(q));
}

unsafe extern "win64" fn get_data(
    this: PCtx,
    async_ptr: *mut c_void,
    data: *mut c_void,
    size: u32,
    flags: u32,
) -> HRESULT {
    let ctx = Obj::get(this);
    if ctx.deferred {
        return DXGI_ERROR_INVALID_CALL;
    }
    let Some(q) = query_from_ptr(async_ptr) else {
        return E_INVALIDARG;
    };
    let need = q.data_size();
    if size != 0 && (data.is_null() || size < need) {
        return E_INVALIDARG;
    }
    let (ended, submitted, tv) = {
        let rt = q.rt.lock().unwrap();
        (rt.ended, rt.submitted, rt.timeline_value)
    };
    if !ended {
        return DXGI_ERROR_INVALID_CALL;
    }
    let target_tv = if !submitted {
        if flags & async_getdata_flag::DO_NOT_FLUSH != 0 {
            return S_FALSE;
        }
        ctx.device.flush_immediate()
    } else {
        tv
    };
    if ctx.device.queue.timeline.completed() < target_tv {
        if flags & async_getdata_flag::DO_NOT_FLUSH != 0 {
            return S_FALSE;
        }
        ctx.device.queue.timeline.wait_cpu(target_tv, u64::MAX);
    }
    if size > 0 && !data.is_null() {
        match q.desc.Query {
            query_kind::EVENT => {
                std::ptr::write_unaligned(data as *mut BOOL, TRUE);
            }
            query_kind::TIMESTAMP => {
                let ticks = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut u64, ticks);
            }
            query_kind::TIMESTAMP_DISJOINT => {
                std::ptr::write_unaligned(
                    data as *mut D3D11_QUERY_DATA_TIMESTAMP_DISJOINT,
                    D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
                        Frequency: d3d12_metal::metal::query::timestamp_frequency(),
                        Disjoint: FALSE,
                    },
                );
            }
            query_kind::OCCLUSION => {
                let samples = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut u64, samples);
            }
            query_kind::OCCLUSION_PREDICATE => {
                let samples = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut BOOL, if samples != 0 { TRUE } else { FALSE });
            }
            query_kind::PIPELINE_STATISTICS => {
                // CPU-submitted counters (IAVertices, IAPrimitives, VS/GS/HS/CSInvocations)
                // are accumulated during Begin..End, while GPU-side counters (CInvocations,
                // CPrimitives, PSInvocations, GSPrimitives, DSInvocations, and indirect draw
                // counts) cannot be sampled on Apple Silicon because Metal 4's
                // MTL4CounterHeapType only provides timestamps (no MTLCommonCounterSetStatistic).
                stub!("GetData(PIPELINE_STATISTICS): GPU-side counters (PS/Clipper/GSPrimitives/DS/indirect) unavailable on Apple Silicon");
                let stats = q.rt.lock().unwrap().pipeline_stats;
                std::ptr::write_unaligned(data as *mut D3D11_QUERY_DATA_PIPELINE_STATISTICS, stats);
            }
            _ => {
                std::ptr::write_bytes(data as *mut u8, 0, need as usize);
            }
        }
    }
    S_OK
}

unsafe extern "win64" fn clear_state(this: PCtx) {
    let ctx = Obj::get(this);
    *ctx.state.lock().unwrap() = ContextState::default();
    if !ctx.deferred {
        ctx.device.imm_exec.lock().unwrap().invalidate_live_caches();
    }
}

unsafe extern "win64" fn flush(this: PCtx) {
    let ctx = Obj::get(this);
    if !ctx.deferred {
        ctx.device.flush_immediate();
    }
}

unsafe extern "win64" fn get_type(this: PCtx) -> u32 {
    if Obj::get(this).deferred { context_type::DEFERRED } else { context_type::IMMEDIATE }
}

unsafe extern "win64" fn get_context_flags(this: PCtx) -> u32 {
    Obj::get(this).flags
}

unsafe extern "win64" fn finish_command_list(
    this: PCtx,
    restore_deferred_state: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let ctx = Obj::get(this);
    if !ctx.deferred {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "FinishCommandList called on immediate context");
    }
    if out.is_null() {
        return E_INVALIDARG;
    }
    let cmds = std::mem::take(&mut *ctx.recorded.lock().unwrap());
    if restore_deferred_state == 0 {
        *ctx.state.lock().unwrap() = ContextState::default();
    }
    let inner = Arc::new(CommandListInner {
        device: ctx.device.clone(),
        context_flags: ctx.flags,
        cmds,
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11CommandList { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn execute_command_list(
    this: PCtx,
    cl_ptr: *mut c_void,
    restore_context_state: BOOL,
) {
    let Some(cl) = cmdlist_from_ptr(cl_ptr) else {
        log::warn!(target: "d3d", "d3d11 ExecuteCommandList: null/foreign command list {cl_ptr:?}");
        return;
    };
    let ctx = Obj::get(this);
    let saved = if restore_context_state != 0 {
        Some(ctx.state.lock().unwrap().clone())
    } else {
        None
    };
    if !ctx.deferred {
        ctx.device.imm_exec.lock().unwrap().invalidate_live_caches();
    }
    for cmd in &cl.cmds {
        ctx.emit(cmd.clone());
    }
    let mut next_state = saved.unwrap_or_default();
    next_state.pso_dirty = true;
    next_state.rt_dirty = true;
    next_state.vp_dirty = true;
    next_state.scissor_dirty = true;
    next_state.blend_factor_dirty = true;
    next_state.stencil_ref_dirty = true;
    *ctx.state.lock().unwrap() = next_state;
    if !ctx.deferred {
        ctx.device.imm_exec.lock().unwrap().invalidate_live_caches();
    }
}

unsafe extern "win64" fn discard_resource(this: PCtx, res: *mut c_void) {
    if let Some(r) = resource_from_ptr(res) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(r));
    }
}

unsafe extern "win64" fn discard_view(this: PCtx, view: *mut c_void) {
    if let Some(o) = Obj::<D3d11Rtv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Dsv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Uav>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Srv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    }
}

unsafe extern "win64" fn discard_view1(this: PCtx, view: *mut c_void, _rects: *const Rect, num_rects: u32) {
    if num_rects == 0 {
        discard_view(this, view);
    }
}

unsafe extern "win64" fn swap_device_context_state(
    this: PCtx,
    state: *mut c_void,
    prev: *mut *mut c_void,
) {
    if !prev.is_null() {
        std::ptr::write_unaligned(prev, std::ptr::null_mut());
    }
    let ctx = Obj::get(this);
    if ctx.deferred || state.is_null() {
        return;
    }
    let Some(target) = Obj::<D3d11DeviceContextState>::from_iface(state) else {
        return;
    };

    // Determine or lazily materialize the previously active ID3DDeviceContextState object.
    let mut active_guard = ctx.active_context_state.lock().unwrap();
    let prev_obj = match active_guard.as_ref() {
        Some(cr) => cr.clone(),
        None => {
            let initial = D3d11DeviceContextState::new_com(
                ctx.device.clone(),
                ctx.device.feature_level,
                ContextState::default(),
            );
            // Ownership transfers into ComRef (no extra AddRef).
            ComRef(initial as usize)
        }
    };

    // Swapping in the already-active state is a no-op (still returns it in `ppPreviousState`).
    if prev_obj.as_ptr() == state {
        prev_obj.copy_out(prev);
        *active_guard = Some(prev_obj);
        return;
    }

    // Save current context state into the previous state object and load target's state.
    let current_state = std::mem::take(&mut *ctx.state.lock().unwrap());
    if let Some(prev_state_obj) = Obj::<D3d11DeviceContextState>::from_iface(prev_obj.as_ptr()) {
        *Obj::get(prev_state_obj).state.lock().unwrap() = current_state;
    }
    let mut next_state = std::mem::take(&mut *Obj::get(target).state.lock().unwrap());
    next_state.pso_dirty = true;
    next_state.rt_dirty = true;
    next_state.vp_dirty = true;
    next_state.scissor_dirty = true;
    next_state.blend_factor_dirty = true;
    next_state.stencil_ref_dirty = true;
    *ctx.state.lock().unwrap() = next_state;
    ctx.device.imm_exec.lock().unwrap().invalidate_live_caches();

    prev_obj.copy_out(prev);
    *active_guard = Some(ComRef::from_borrowed(state));
}

/// Audited `ID3D11DeviceContext` / `ID3D11DeviceContext1` methods safe for the
/// ARM64 JIT `DF_PLAIN` direct-call fast path:
/// - `<= 8` integer/pointer arguments (including `this`), integer/void return
/// - no `f32`/`f64`/XMM arguments or returns
/// - strictly callback-free: NEVER invokes `com_release`/`com_add_ref` on `ComRef`s
///   (dropping a bound COM object's `PrivateData` can re-enter guest `IUnknown::Release`
///   stored via `SetPrivateDataInterface`, which requires the generic bridge to publish
///   the live guest stack pointer)
/// - no SEH unwind
///
/// Explicitly excluded:
/// - Float/XMM-arg or float-return methods: `ClearDepthStencilView`, `SetResourceMinLOD`,
///   `GetResourceMinLOD`, `ClearRenderTargetView`, `ClearUnorderedAccessViewFloat`, `ClearView`
/// - `> 8` argument methods: `CopySubresourceRegion` (9 args), `CopySubresourceRegion1` (10 args)
/// - COM-refcount-mutating methods (`ClearState`, `FinishCommandList`, `ExecuteCommandList`,
///   `SwapDeviceContextState`, and all `*_Set*` / `*_Get*` COM object binding slots)
pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Vec<usize>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            draw_indexed as *const () as usize,
            draw as *const () as usize,
            map as *const () as usize,
            unmap as *const () as usize,
            draw_indexed_instanced as *const () as usize,
            draw_instanced as *const () as usize,
            ia_set_primitive_topology as *const () as usize,
            ia_get_primitive_topology as *const () as usize,
            begin_async as *const () as usize,
            end_async as *const () as usize,
            get_data as *const () as usize,
            draw_indexed_instanced_indirect as *const () as usize,
            draw_instanced_indirect as *const () as usize,
            dispatch as *const () as usize,
            dispatch_indirect as *const () as usize,
            rs_set_viewports as *const () as usize,
            rs_get_viewports as *const () as usize,
            rs_set_scissor_rects as *const () as usize,
            rs_get_scissor_rects as *const () as usize,
            copy_resource as *const () as usize,
            update_subresource as *const () as usize,
            update_subresource1 as *const () as usize,
            copy_structure_count as *const () as usize,
            clear_uav_uint as *const () as usize,
            generate_mips as *const () as usize,
            resolve_subresource as *const () as usize,
            flush as *const () as usize,
            get_type as *const () as usize,
            get_context_flags as *const () as usize,
            discard_resource as *const () as usize,
            discard_view as *const () as usize,
            discard_view1 as *const () as usize,
        ]
    })
}
