//! Command allocators and command lists: D3D12 commands are encoded into an
//! `MTL4CommandBuffer` at record time.
//!
//! Barriers (memo §3.8, pitfall 7): the first encoder of a list and the first
//! encoder after any `ResourceBarrier` (or internal producer such as a copy or
//! ExecuteIndirect expansion) start with a queue barrier all-stages →
//! all-stages; a barrier inside a render pass ends the pass. Encoder
//! boundaries alone do not serialize: D3D12 requires an explicit barrier for
//! every cross-state dependency, except render-target/depth writes that stay
//! in the same state across passes, which are tracked per resource and get a
//! fragment → fragment barrier when a later pass attaches the same resource.
//! Whole-subresource clears are deferred and folded into the next render
//! pass's load action (`AW2_LEGACY_BARRIERS=1` restores the old model).

mod barrier_app;
mod barrier_sync;
mod clears;
mod compute;
mod copy;
mod draw;
mod encoder;
mod exact_binds;
mod indirect;
mod metalfx_rt;
mod opts;
mod state;
#[cfg(test)]
mod tests;

pub use copy::{split_subresource, CopyBox, CopyLoc};
pub use opts::{copy_queue_light, take_ios_icb_pass_splits};
#[allow(unused_imports)]
pub(crate) use opts::{
    opt_barrier_tight_v2_env, opt_blit_list_narrow_v2_env, opt_clear_cs_exact_sync_env,
    opt_clear_cs_tile4_env, opt_clear_isolate_coalesce_env, opt_clear_keep_dsv_env,
    opt_clear_keep_smart_env, opt_clear_overlap_exact_env, opt_clear_pass_fold_v2_env,
    opt_clear_prelist_overlap_env, opt_clear_zero_qb_coalesce_env, opt_copy_queue_light_env,
    opt_cs_bind_cache_env, opt_cs_bind_intra_safe_env, opt_cs_fast_bind_exact_env,
    opt_cs_intra_barrier_exact_env, opt_cs_intra_barrier_v2_env, opt_cs_mid_qb_defer_v3_env,
    opt_cs_mid_qb_res_defer_v5_env, opt_cs_mid_qb_touch_defer_env, opt_cs_mid_qb_unpoison_v4_env,
    opt_cs_to_ps_overlap_env, opt_cs_to_ps_overlap_v2_env, opt_cs_uav_producer_exact_env,
    opt_desc_table_cs_fast_env, opt_desc_table_res_exact_env, opt_ei_dispatch_exact_env,
    opt_empty_enc_elide_env, opt_fast_bind_pso_env, opt_fast_enc_state_env,
    opt_gfx_desc_table_exact_env, opt_gfx_overlap_capped_env, opt_gfx_overlap_frag_cap_env,
    opt_gfx_to_ps_overlap_env, opt_gfx_to_ps_overlap_v2_env, opt_ib_bindless_war_env,
    opt_ib_exact_obl_env, opt_ib_substage_narrow_env, opt_ib_tight_env,
    opt_ib_uav0_unbounded_exact_env, opt_icb_batch_env, opt_icb_enc_barrier_exact_env,
    opt_icb_prepass_compute_env, opt_list_barrier_tight_env, opt_list_boundary_overlap_env,
    opt_list_start_render_split_env, opt_list_start_substage_env, opt_mid_qb_stagesync_env,
    opt_ps_srv_class_exact_env, opt_ps_to_cs_hk_overlap_env, opt_ps_to_cs_overlap_env,
    opt_ps_to_cs_overlap_v2_env, opt_queue_start_stage_exact_env, opt_render_dom_exact_env,
    opt_render_split_stage_sync_env, opt_ro_dsv_dontcare_env, opt_rp_keep_write_env,
    opt_rp_keep_write_exact_env, opt_rp_split_exact_keep_env, opt_sm66_handle_kind_exact_env,
    opt_sm66_handle_kind_v2_env, opt_split_compute_exact_env, opt_split_res_soft_sync_env,
    opt_stagesync_barrier_v2_env, opt_subrange_uav_barrier_env, opt_uav0_clear_overlap_env,
    opt_vs_buf_defer_exact_env,
};
pub use state::{
    filter_res_state_bits, is_read_to_read_transition, state_stages, state_stages_for_res,
    view_flags, Barrier, GfxState, Predication, TargetView,
};

use self::opts::CmdListOpts;
use self::state::{
    BarrierState, BindTableCache, ClearState, Encoder, IcbBatchState, IntraComputeState,
    RenderPassSyncState,
};
#[cfg(test)]
use self::state::GFX_DIRTY_ALL;
use super::binding::{new_argument_table, MtlArgumentTable, RootState, UploadRing};
use super::encstats as stats;
use super::passtiming as pt;
use super::pipeline::ComputePipeline;
use super::queue::{ListType, MtlCommandBuffer};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::sync::{Arc, Mutex};

pub type MtlAllocator = ProtocolObject<dyn MTL4CommandAllocator>;

pub struct CommandAllocator {
    pub a: Retained<MtlAllocator>,
    pub kind: ListType,
    /// Transient GPU memory (CPU-written root argument images & params) of lists recorded here.
    pub ring: Arc<Mutex<UploadRing>>,
    /// GPU-written, GPU-read transient buffers for `ExecuteIndirect` ICB expansion
    /// (`range_out`, `tlab_out`, `args_out`), rewindable between GPU-synchronized batches.
    pub icb_scratch: Arc<Mutex<UploadRing>>,
    /// Retired acceleration structures replaced while recording on this allocator;
    /// kept alive until [`Self::reset`] (when the app guarantees GPU completion).
    pub rt_keep: Arc<Mutex<Vec<crate::rt::accel::Retired>>>,
    /// GPU-encoded `ExecuteIndirect` commands of lists recorded here.
    pub icbs: Arc<Mutex<super::indirect_icb::IcbRing>>,
}
unsafe impl Send for CommandAllocator {}
unsafe impl Sync for CommandAllocator {}

impl CommandAllocator {
    pub fn new(kind: ListType) -> Option<CommandAllocator> {
        let d = super::device()?;
        Some(CommandAllocator {
            a: d.dev.newCommandAllocator()?,
            kind,
            ring: Arc::default(),
            icb_scratch: Arc::new(Mutex::new(UploadRing::new_gpu_scratch())),
            rt_keep: Arc::default(),
            icbs: Arc::default(),
        })
    }

    /// `ID3D12CommandAllocator::Reset` (the app guarantees GPU completion).
    pub fn reset(&self) {
        self.a.reset();
        self.ring.lock().unwrap().reset();
        self.icb_scratch.lock().unwrap().reset();
        self.rt_keep.lock().unwrap().clear();
        self.icbs.lock().unwrap().reset();
    }
}

pub struct CommandList {
    pub kind: ListType,
    pub cb: Retained<MtlCommandBuffer>,
    pub label: String,
    pub compute_root: RootState,
    pub gfx_root: RootState,
    pub gfx: GfxState,
    /// Timeline optimization toggles (initialized from env vars, overridable in tests).
    pub(crate) opts: CmdListOpts,
    /// Cross-encoder / list-level barrier tracking, stage generations, and scratch vectors.
    bar: BarrierState,
    /// Intra-compute-encoder touch sets, obligations, substage buckets, and scratch vectors.
    ib: IntraComputeState,
    /// Open render-pass kept-write tracking, per-stage exact bindings, UAV hazards, and state cache.
    rp_sync: RenderPassSyncState,
    /// Deferred whole-subresource clears and `DiscardResource` marks.
    clears: ClearState,
    /// `ExecuteIndirect` ICB batching, ring handles, and pass-split counters.
    icb: IcbBatchState,
    /// Dirty-tracked `MTL4ArgumentTable` slots plus token-validated compute & graphics bind caches.
    binds: BindTableCache,
    open: bool,
    /// Begun at least once (an executable command buffer exists).
    recorded: bool,
    enc: Encoder,
    ring: Option<Arc<Mutex<UploadRing>>>,
    table: Retained<MtlArgumentTable>,
    /// GPU addresses of the bound CBV/SRV/UAV and sampler heaps.
    heaps: [u64; 2],
    /// Resolved `(base_gpu_va, byte_len, base_cpu_ptr)` of `heaps[0]`.
    heap0_cpu: Option<(u64, u64, usize)>,
    compute_pso: Option<Arc<ComputePipeline>>,
    /// The compute PSO is set on the current encoder.
    compute_pso_bound: bool,
    rt_pso: Option<Arc<crate::rt::pipeline::StateObject>>,
    occlusion: Option<super::query::ActiveOcclusion>,
    /// True once a render pass has opened/enabled the current `occlusion` query slot.
    occl_drew: bool,
    /// `MtlBuffer` pointer attached as `visibilityResultBuffer` on the open render pass.
    enc_vis_buf: Option<usize>,
    /// True if the open render pass uses `MTLVisibilityResultType::Reset`.
    enc_vis_reset: bool,
    /// Query slot indices already enabled in the open `Reset` render pass.
    enc_vis_used_indices: Vec<u32>,
    predication: Option<Predication>,
    /// Argument table of utility kernels (ClearUAV, RT), created on first use.
    util_table: Option<Retained<MtlArgumentTable>>,
    /// Lazily allocated fence updated before opaque MetalFX passes (`MTL4FX*Scaler::setFence:`).
    mfx_fence: Option<Retained<ProtocolObject<dyn MTLFence>>>,
    /// Deferred post-MetalFX fence wait folded into the next encoder's `begin_barrier`
    /// (or flushed in `close()` if MetalFX was the last operation on the command buffer).
    pending_mfx_wait_fence: Option<Retained<ProtocolObject<dyn MTLFence>>>,
    rt_keep: Option<Arc<Mutex<Vec<crate::rt::accel::Retired>>>>,
    /// Per-encoder GPU timestamps of this recording (`AW2_PASS_TIMING`), lazily acquired on first `pt_open`.
    pt: Option<pt::PassLog>,
    pt_armed: bool,
    /// Last shader recorded on this list (for `PORT_DIAG_GPU_NAN` pass attribution; zero overhead when off).
    diag_last_kind: &'static str,
    diag_last_key: u64,
    diag_last_vs_key: u64,
}
unsafe impl Send for CommandList {}

impl CommandList {
    pub fn new(kind: ListType) -> Option<CommandList> {
        let d = super::device()?;
        let opts = CmdListOpts::from_env();
        let mut compute_root = RootState::default();
        let mut gfx_root = RootState::default();
        compute_root.dedup = opts.opt_fast_enc_state;
        gfx_root.dedup = opts.opt_fast_enc_state;
        Some(CommandList {
            kind,
            cb: d.dev.newCommandBuffer()?,
            label: String::new(),
            compute_root,
            gfx_root,
            gfx: GfxState::default(),
            opts,
            bar: BarrierState::new(),
            ib: IntraComputeState::new(),
            rp_sync: RenderPassSyncState::new(),
            clears: ClearState::default(),
            icb: IcbBatchState::default(),
            binds: BindTableCache::new(),
            open: false,
            recorded: false,
            enc: Encoder::None,
            ring: None,
            table: new_argument_table("aw2 command list")?,
            heaps: [0; 2],
            heap0_cpu: None,
            compute_pso: None,
            compute_pso_bound: false,
            rt_pso: None,
            occlusion: None,
            occl_drew: false,
            enc_vis_buf: None,
            enc_vis_reset: false,
            enc_vis_used_indices: Vec::new(),
            predication: None,
            util_table: None,
            mfx_fence: None,
            pending_mfx_wait_fence: None,
            rt_keep: None,
            pt: None,
            pt_armed: false,
            diag_last_kind: "none",
            diag_last_key: 0,
            diag_last_vs_key: 0,
        })
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn set_fast_enc_state(&mut self, on: bool) {
        self.opts.opt_fast_enc_state = on;
        self.compute_root.dedup = on;
        self.gfx_root.dedup = on;
        self.rp_sync.enc_render_cache.reset();
        self.gfx.dirty = GFX_DIRTY_ALL;
        self.rp_sync.last_draw_params_len = 0;
        self.rp_sync.last_draw_params_va = 0;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Closed with a recorded command buffer (safe to commit).
    pub fn has_commands(&self) -> bool {
        self.recorded && !self.open
    }

    /// Whether at least one GPU encoder, timestamp query, or counter resolve was
    /// encoded during the current/most-recent `begin..close` recording.
    #[inline]
    pub fn has_gpu_work_recorded(&self) -> bool {
        self.bar.has_gpu_work
    }

    /// `Reset(allocator, pso)` / creation: begin recording (all list state
    /// is reset, D3D12 semantics).
    pub fn begin(&mut self, alloc: &CommandAllocator) -> Result<(), String> {
        if self.open {
            return Err("command list is already open".into());
        }
        self.cb.beginCommandBufferWithAllocator(&alloc.a);
        if !self.label.is_empty() {
            self.cb.setLabel(Some(&objc2_foundation::NSString::from_str(&self.label)));
        }
        self.pt = None;
        self.pt_armed = pt::enabled();
        stats::inc(&stats::c::lists);
        self.open = true;
        self.recorded = true;
        let list_tight = self.list_barrier_tight_active();
        self.bar.reset_for_begin(list_tight);
        self.ib.ib_obl.clear();
        self.ib.ib_obl_all = false;
        self.ib.ib_cs_clear = false;
        self.ib.ib_touch.clear_exact();
        self.ib_reset_tight();
        self.rp_sync.reset_for_begin();
        self.invalidate_table_cache();
        self.clears.pending_clears.clear();
        self.clears.discards.clear();
        self.enc = Encoder::None;
        self.ring = Some(alloc.ring.clone());
        self.rt_keep = Some(alloc.rt_keep.clone());
        self.icb.reset_for_begin(alloc.icb_scratch.clone(), alloc.icbs.clone());
        self.binds.cs_bind_cache.valid = false;
        self.binds.gfx_bind_cache.valid = false;
        self.pending_mfx_wait_fence = None;
        self.compute_root = RootState::default();
        self.gfx_root = RootState::default();
        self.compute_root.dedup = self.opts.opt_fast_enc_state;
        self.gfx_root.dedup = self.opts.opt_fast_enc_state;
        self.heaps = [0; 2];
        self.heap0_cpu = None;
        self.compute_pso = None;
        self.compute_pso_bound = false;
        self.rt_pso = None;
        self.occlusion = None;
        self.occl_drew = false;
        self.enc_vis_buf = None;
        self.enc_vis_reset = false;
        self.enc_vis_used_indices.clear();
        self.predication = None;
        self.gfx = GfxState::default();
        Ok(())
    }

    /// `ClearState(initial_pso)`: resets pipeline, root argument, IA, RS, and OM
    /// bindings while preserving descriptor heaps and predication (D3D12 semantics).
    pub fn clear_state(&mut self) {
        self.end_encoder();
        self.rp_sync.cur_draw_exact_valid = false;
        self.binds.cs_bind_cache.valid = false;
        self.binds.gfx_bind_cache.valid = false;
        self.rp_sync.cs_hk_profile = super::pipeline::HandleKindProfile::OPAQUE;
        self.compute_root = RootState::default();
        self.gfx_root = RootState::default();
        self.compute_root.dedup = self.opts.opt_fast_enc_state;
        self.gfx_root.dedup = self.opts.opt_fast_enc_state;
        self.compute_pso = None;
        self.compute_pso_bound = false;
        self.rt_pso = None;
        self.gfx = GfxState::default();
        self.rp_sync.last_draw_params_len = 0;
        self.rp_sync.last_draw_params_va = 0;
    }

    /// `SetPredication(buffer_gpu_va, op)` (`None` disables predication).
    pub fn set_predication(&mut self, pred: Option<(u64, u32)>) {
        self.predication = pred.map(|(gpu_va, op)| Predication { gpu_va, op });
    }

    /// `Close`.
    pub fn close(&mut self) -> Result<(), String> {
        if !self.open {
            return Err("command list is not open".into());
        }
        let flushed = self.flush_clears(&stats::c::cf_close, |_| true);
        self.flush_pending_mfx_wait_fence();
        if self.list_barrier_tight_active() {
            self.emit_close_producer_barriers();
        } else if !opts::legacy_barriers()
            && opts::barrier_masks()
            && !self.is_copy_queue_light()
        {
            self.emit_close_trailing_producer_barriers();
        }
        self.end_encoder();
        self.cb.endCommandBuffer();
        self.open = false;
        self.pt_armed = false;
        if self.opts.opt_empty_enc_elide && !self.bar.has_gpu_work {
            self.recorded = false;
            self.pt = None;
            stats::inc(&stats::c::empty_list_elided);
        }
        flushed
    }
}
