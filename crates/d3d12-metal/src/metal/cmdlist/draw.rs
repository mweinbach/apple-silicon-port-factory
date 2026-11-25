//! Graphics & mesh state setters, lazy `OMSetRenderTargets` render-pass open,
//! direct draws, geometry-shader emulation, and `DispatchMesh`.

use super::opts::rp_discard;
use super::state::{
    view_flags, ClearKind, DirectBind, DrawOp, DrawSeg, DrawSegExact, DynSnapshot, Encoder,
    PendingClear, TargetView, GFX_DIRTY_ALL, GFX_DIRTY_BLEND, GFX_DIRTY_DEPTH_BOUNDS,
    GFX_DIRTY_PSO, GFX_DIRTY_SCISSORS, GFX_DIRTY_SHARED, GFX_DIRTY_STENCIL, GFX_DIRTY_VIEWPORTS,
};
use super::CommandList;
use dxil_metal::runtime::{
    ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT, ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT,
    ARGUMENT_BUFFER_UNIFORMS_BIND_POINT, NON_INDEXED_DRAW, VERTEX_BUFFER_BIND_POINT,
};
use crate::metal::binding;
use crate::metal::encstats as stats;
use crate::metal::gpunan;
use crate::metal::indirect::{self, IndirectLayout, KernelParams, Work};
use crate::metal::passtiming as pt;
use crate::metal::pipeline::{GraphicsPipeline, READS_BUF, READS_TEX};
use crate::metal::resource::{MtlTexture, Resource};
use crate::types::Rect;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{
    MTL4ComputeCommandEncoder, MTL4RenderCommandEncoder, MTL4RenderPassDescriptor, MTLClearColor,
    MTLLoadAction, MTLPixelFormat, MTLPrimitiveType, MTLRenderStages, MTLScissorRect, MTLSize,
    MTLStages, MTLStoreAction, MTLTexture, MTLTextureType, MTLViewport,
};
use std::sync::Arc;

impl CommandList {
    /// Record per-draw stage read/HK/exact-bind metadata on the open render pass
    /// (called before `render_mid_barrier` and UAV dirty flagging).
    pub(super) fn record_gfx_draw_pre_barrier(
        &mut self,
        p: &GraphicsPipeline,
        eff_reads: u8,
        writes_uav: bool,
        uav_indirect: bool,
    ) {
        self.rp_sync.enc_pre_raster_reads |= eff_reads;
        self.rp_sync.enc_pre_raster_srv_uav_reads |= self.gfx_pso_srv_uav_pre_raster_reads(p);
        self.rp_sync.enc_ps_reads |= p.ps_read_classes;
        self.rp_sync.enc_ps_srv_uav_reads |= p.ps_srv_uav_classes;
        if self.hk_any_active() {
            let (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk) = self.gfx_pso_stage_hk(p);
            self.rp_sync.enc_pre_raster_hk |= pre_hk;
            self.rp_sync.enc_ps_hk |= ps_hk;
            if !self.opts.opt_gfx_desc_table_exact {
                self.rp_sync.enc_pre_raster_unb_hk |= pre_unb_hk | pre_hk;
                self.rp_sync.enc_ps_unb_hk |= ps_unb_hk | ps_hk;
            }
        }
        self.rp_sync.enc_uav_used |= writes_uav || p.uses_uav;
        if self.opts.opt_gfx_desc_table_exact {
            if self.rp_sync.cur_draw_exact_valid {
                self.commit_cur_draw_exact_to_enc();
            } else {
                self.commit_gfx_bind_cache_to_enc(p);
            }
        }
        if writes_uav && self.opts.opt_rp_split_exact_keep {
            self.commit_gfx_bind_cache_uav_writes(uav_indirect);
        }
    }

    /// Update open render-pass UAV write state after a graphics/mesh/emulated draw.
    #[inline]
    pub(super) fn record_gfx_draw_uav_writes(
        &mut self,
        writes_uav: bool,
        pre_writes_uav: bool,
        ps_writes_uav: bool,
        pre_stages: MTLStages,
    ) {
        self.gfx.pass_uav |= writes_uav;
        if writes_uav {
            self.bar.uav_gfx_dirty = true;
            self.bar.enc_had_uav_gfx = true;
            if pre_writes_uav {
                self.rp_sync.enc_uav_write_stages |= pre_stages;
            }
            if ps_writes_uav {
                self.rp_sync.enc_uav_write_stages |= MTLStages::Fragment;
            }
            self.disc_unmark(0);
        }
    }

    /// Check whether a direct draw in an active ICB batch must split for a pass UAV hazard.
    fn check_batched_draw_uav_hazard(&mut self, indexed: bool) -> bool {
        self.gfx.pass_uav_hazard
            && self.gfx.pso.clone().is_some_and(|p| {
                let w = self.gfx_pso_writes_uav(&p);
                if self.opts.opt_rp_split_exact_keep {
                    if !(w || p.uses_uav) {
                        false
                    } else {
                        self.resolve_gfx_binds_cached(
                            &p,
                            indexed,
                            false,
                            false,
                            false,
                            false,
                            Some(&[]),
                            true,
                            w && self.opts.opt_rp_split_exact_keep,
                        );
                        self.gfx_cached_overlaps_pass_hazard(false)
                    }
                } else {
                    w
                }
            })
    }

    /// `SetPipelineState` with a graphics PSO.
    pub fn set_graphics_pipeline(&mut self, p: Arc<GraphicsPipeline>) {
        if gpunan::is_capturing() {
            self.diag_last_kind = "gfx";
            self.diag_last_key = p.ps.as_ref().map_or(p.vs.key as u64, |ps| ps.key as u64);
            self.diag_last_vs_key = p.vs.key as u64;
        }
        if self.gfx.pso.as_ref().is_some_and(|c| Arc::ptr_eq(c, &p)) {
            return;
        }
        self.binds.gfx_bind_cache.valid = false;
        self.gfx.pso = Some(p);
        self.gfx.mesh = None;
        self.gfx.dirty |= GFX_DIRTY_PSO;
    }

    /// `SetPipelineState` with a mesh-shader PSO.
    pub fn set_mesh_pipeline(&mut self, p: Arc<crate::mesh::MeshPipeline>) {
        if gpunan::is_capturing() {
            self.diag_last_kind = "mesh";
            self.diag_last_key = 0;
            self.diag_last_vs_key = 0;
        }
        if self.gfx.mesh.as_ref().is_some_and(|c| Arc::ptr_eq(c, &p)) {
            return;
        }
        self.binds.gfx_bind_cache.valid = false;
        self.gfx.mesh = Some(p);
        self.gfx.pso = None;
        self.gfx.dirty |= GFX_DIRTY_PSO;
    }

    /// `OMSetRenderTargets`: a draw after this opens a new render pass.
    pub fn set_render_targets(&mut self, rtvs: Vec<TargetView>, dsv: Option<TargetView>) {
        let same = Self::same_targets(&self.gfx.rtvs, self.gfx.dsv, &rtvs, dsv);
        if !same && !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        self.gfx.rtvs = rtvs;
        self.gfx.dsv = dsv;
        if !same && self.gfx.draw_pass {
            if self.opts.opt_rp_split_exact_keep {
                self.gfx.had_target_rebind = true;
            } else {
                self.end_encoder();
            }
        }
    }

    pub fn set_viewports(&mut self, v: Vec<MTLViewport>) {
        self.gfx.viewports = v;
        self.gfx.dirty |= GFX_DIRTY_VIEWPORTS;
    }

    /// Set D3D12 viewports without exposing `objc2_metal::MTLViewport` to callers.
    pub fn set_viewports_d3d(&mut self, viewports: &[crate::types::Viewport]) {
        let m = viewports
            .iter()
            .map(|v| MTLViewport {
                originX: v.x as f64,
                originY: v.y as f64,
                width: v.w as f64,
                height: v.h as f64,
                znear: v.min_depth as f64,
                zfar: v.max_depth as f64,
            })
            .collect();
        self.set_viewports(m);
    }

    /// Set viewports from raw `(x, y, width, height, min_depth, max_depth)` tuples.
    pub fn set_viewports_raw(&mut self, viewports: &[(f32, f32, f32, f32, f32, f32)]) {
        let m = viewports
            .iter()
            .map(|&(x, y, w, h, znear, zfar)| MTLViewport {
                originX: x as f64,
                originY: y as f64,
                width: w as f64,
                height: h as f64,
                znear: znear as f64,
                zfar: zfar as f64,
            })
            .collect();
        self.set_viewports(m);
    }

    pub fn set_scissors(&mut self, r: Vec<Rect>) {
        self.gfx.scissors = r;
        self.gfx.dirty |= GFX_DIRTY_SCISSORS;
    }

    pub fn set_blend_factor(&mut self, f: [f32; 4]) {
        self.gfx.blend_factor = f;
        self.gfx.dirty |= GFX_DIRTY_BLEND;
    }

    pub fn set_stencil_ref(&mut self, r: u32) {
        self.gfx.stencil_ref = r;
        self.gfx.stencil_back_ref = r;
        self.gfx.dirty |= GFX_DIRTY_STENCIL;
    }

    pub fn set_front_and_back_stencil_ref(&mut self, front: u32, back: u32) {
        self.gfx.stencil_ref = front;
        self.gfx.stencil_back_ref = back;
        self.gfx.dirty |= GFX_DIRTY_STENCIL;
    }

    pub fn set_depth_bounds(&mut self, min: f32, max: f32) {
        self.gfx.depth_bounds = (min, max);
        self.gfx.dirty |= GFX_DIRTY_DEPTH_BOUNDS;
    }

    /// `DiscardResource`: the contents become undefined, so the next render
    /// pass attaching the resource may skip loading it (LoadAction DontCare,
    /// with `AW2_RP_DISCARD=1`; the counters run either way). Only
    /// whole-resource discards of textures are marked (`region` = (NumRects,
    /// FirstSubresource, NumSubresources)). A mark is dropped by any later
    /// write to the resource (clears, copies, resolves, upscales, UAV writes
    /// by dispatches or UAV-writing draws) and consumed by the next pass
    /// attaching it.
    pub fn discard_resource(&mut self, r: &Resource, region: Option<(u32, u32, u32)>) {
        self.flush_icb_prepass_if_pending();
        stats::inc(&stats::c::disc_call);
        if r.texture().is_none() {
            return;
        }
        let whole = match region {
            None => true,
            Some((rects, first, count)) => {
                rects == 0 && first == 0 && count >= r.desc.subresource_count()
            }
        };
        if !whole {
            return;
        }
        let key = r as *const Resource as usize;
        // Attached to the open pass: draws after the discard keep their output.
        let attached = self.gfx.draw_pass
            && (self.active_rtvs().iter().any(|v| v.resource == key)
                || self.active_dsv().is_some_and(|v| v.resource == key));
        if attached || self.bar.pass_targets.contains(&key) {
            return;
        }
        stats::inc(&stats::c::disc_whole);
        if !self.clears.discards.contains(&key) {
            self.clears.discards.push(key);
        }
    }

    /// Drop the discard mark of resource `r` (0: all marks, for writes to
    /// unknown resources).
    #[inline]
    pub(super) fn disc_unmark(&mut self, r: usize) {
        if self.clears.discards.is_empty() {
            return;
        }
        if r == 0 {
            self.clears.discards.clear();
        } else {
            self.clears.discards.retain(|&d| d != r);
        }
    }

    /// Whether an attachment load of discarded resource `r` becomes
    /// DontCare (counts `disc_hit` / `disc_dc` per attachment plane).
    fn disc_load(&self, r: usize) -> bool {
        if !self.clears.discards.contains(&r) {
            return false;
        }
        stats::inc(&stats::c::disc_hit);
        let on = rp_discard();
        if on {
            stats::inc(&stats::c::disc_dc);
        }
        on
    }

    /// Take the pending clear for attachment `v`, if any.
    fn take_clear(&mut self, v: &TargetView, dsv: bool) -> Option<ClearKind> {
        let i = self.clears.pending_clears.iter().position(|p| p.same_key(v, dsv))?;
        stats::inc(&stats::c::clear_folded);
        Some(self.clears.pending_clears.remove(i).kind)
    }

    #[cfg(test)]
    pub(super) unsafe fn draw_encoder(
        &mut self,
    ) -> Result<(Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>, (usize, usize)), String> {
        self.draw_encoder_for(0)
    }

    /// Open a render pass for the current targets (loading and storing
    /// their contents, or clearing them when a deferred clear is pending).
    /// Returns the encoder and the render target size.
    pub(super) unsafe fn draw_encoder_for(
        &mut self,
        opening_pre_raster_reads: u8,
    ) -> Result<(Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>, (usize, usize)), String> {
        if let (Encoder::Render(e), true) = (&self.enc, self.gfx.draw_pass) {
            if self.opts.opt_rp_split_exact_keep {
                let dsv_norm = self.gfx.dsv.filter(|v| v.resource != 0);
                let same_open = Self::same_targets(
                    &self.gfx.open_rtvs,
                    self.gfx.open_dsv,
                    &self.gfx.rtvs,
                    dsv_norm,
                );
                let pending_hit = !self.clears.pending_clears.is_empty()
                    && self.clears.pending_clears.iter().any(|p| {
                        self.gfx
                            .rtvs
                            .iter()
                            .chain(dsv_norm.iter())
                            .any(|v| v.resource != 0 && v.resource == p.view.resource)
                    });
                let disc_hit = !self.clears.discards.is_empty()
                    && self.clears.discards.iter().any(|&d| {
                        self.gfx
                            .rtvs
                            .iter()
                            .chain(dsv_norm.iter())
                            .any(|v| v.resource == d)
                    });
                if !same_open || pending_hit || disc_hit {
                    self.end_encoder();
                } else {
                    if std::mem::replace(&mut self.gfx.had_target_rebind, false) {
                        stats::inc(&stats::c::rp_split_exact_keep);
                    }
                    if self.rp_sync.rp_kept_writes.iter().any(|kw| kw.eff_s1 == 0) {
                        self.rp_sync.rp_kept_later_draw = true;
                    }
                    return Ok((e.clone(), self.gfx.pass_size));
                }
            } else {
                if self.rp_sync.rp_kept_writes.iter().any(|kw| kw.eff_s1 == 0) {
                    self.rp_sync.rp_kept_later_draw = true;
                }
                return Ok((e.clone(), self.gfx.pass_size));
            }
        }
        let rtvs = self.gfx.rtvs.clone();
        let dsv = self.gfx.dsv.filter(|v| v.resource != 0);
        if !self.clears.pending_clears.is_empty() {
            // A pending clear of an attached subresource through another
            // view format cannot be folded: it must land first.
            let conflicts = |p: &PendingClear| {
                rtvs.iter().chain(dsv.iter()).any(|v| {
                    v.resource != 0
                        && v.resource == p.view.resource
                        && v.mip == p.view.mip
                        && v.first_slice == p.view.first_slice
                }) && !rtvs.iter().any(|v| p.same_key(v, false))
                    && !dsv.is_some_and(|v| p.same_key(&v, true))
            };
            self.flush_clears(&stats::c::cf_view, conflicts)?;
        }
        let mut size = (usize::MAX, usize::MAX);
        let mut fit = |t: &MtlTexture, level: usize| {
            size.0 = size.0.min((t.width() >> level).max(1));
            size.1 = size.1.min((t.height() >> level).max(1));
        };
        let rp = MTL4RenderPassDescriptor::new();
        let mut sync_targets: Vec<usize> = Vec::with_capacity(rtvs.len() + 1);
        let mut write_targets: Vec<usize> = Vec::with_capacity(rtvs.len() + 1);
        for (i, v) in rtvs.iter().enumerate() {
            if v.resource == 0 {
                continue;
            }
            let (tex, level, slice) =
                Self::target_texture(v).ok_or("RTV does not reference a texture")?;
            fit(&tex, level);
            sync_targets.push(v.resource);
            write_targets.push(v.resource);
            let ca = rp.colorAttachments().objectAtIndexedSubscript(i);
            ca.setTexture(Some(&tex));
            ca.setLevel(level);
            if tex.textureType() == MTLTextureType::Type3D {
                ca.setDepthPlane(slice);
            } else {
                ca.setSlice(slice);
            }
            match self.take_clear(v, false) {
                Some(ClearKind::Color(c)) => {
                    ca.setLoadAction(MTLLoadAction::Clear);
                    ca.setClearColor(MTLClearColor {
                        red: c[0] as f64,
                        green: c[1] as f64,
                        blue: c[2] as f64,
                        alpha: c[3] as f64,
                    });
                }
                _ if self.disc_load(v.resource) => ca.setLoadAction(MTLLoadAction::DontCare),
                _ => ca.setLoadAction(MTLLoadAction::Load),
            }
            ca.setStoreAction(MTLStoreAction::Store);
        }
        if let Some(v) = dsv {
            let (tex, level, slice) =
                Self::target_texture(&v).ok_or("DSV does not reference a texture")?;
            fit(&tex, level);
            let (cd, cs) = match self.take_clear(&v, true) {
                Some(ClearKind::DepthStencil { depth, stencil }) => (depth, stencil),
                _ => (None, None),
            };
            let pf = tex.pixelFormat();
            let has_depth = pf != MTLPixelFormat::X32_Stencil8 && pf != MTLPixelFormat::Stencil8;
            let has_stencil = matches!(
                pf,
                MTLPixelFormat::Depth32Float_Stencil8
                    | MTLPixelFormat::X32_Stencil8
                    | MTLPixelFormat::Stencil8
            );
            let mut depth_ro_dc = false;
            let mut stencil_ro_dc = false;
            if has_depth {
                let da = rp.depthAttachment();
                da.setTexture(Some(&tex));
                da.setLevel(level);
                da.setSlice(slice);
                let disc_d = if let Some(d) = cd {
                    da.setLoadAction(MTLLoadAction::Clear);
                    da.setClearDepth(d as f64);
                    false
                } else if self.disc_load(v.resource) {
                    da.setLoadAction(MTLLoadAction::DontCare);
                    true
                } else {
                    da.setLoadAction(MTLLoadAction::Load);
                    false
                };
                depth_ro_dc = self.opts.opt_ro_dsv_dontcare
                    && (v.flags & view_flags::DSV_READ_ONLY_DEPTH != 0)
                    && cd.is_none()
                    && !disc_d;
                if depth_ro_dc {
                    da.setStoreAction(MTLStoreAction::DontCare);
                    stats::inc(&stats::c::ro_dsv_dontcare);
                } else {
                    da.setStoreAction(MTLStoreAction::Store);
                }
            }
            if has_stencil {
                let sa = rp.stencilAttachment();
                sa.setTexture(Some(&tex));
                sa.setLevel(level);
                sa.setSlice(slice);
                let disc_s = if let Some(s) = cs {
                    sa.setLoadAction(MTLLoadAction::Clear);
                    sa.setClearStencil(s as u32);
                    false
                } else if self.disc_load(v.resource) {
                    sa.setLoadAction(MTLLoadAction::DontCare);
                    true
                } else {
                    sa.setLoadAction(MTLLoadAction::Load);
                    false
                };
                stencil_ro_dc = self.opts.opt_ro_dsv_dontcare
                    && (v.flags & view_flags::DSV_READ_ONLY_STENCIL != 0)
                    && cs.is_none()
                    && !disc_s;
                if stencil_ro_dc {
                    sa.setStoreAction(MTLStoreAction::DontCare);
                    stats::inc(&stats::c::ro_dsv_dontcare);
                } else {
                    sa.setStoreAction(MTLStoreAction::Store);
                }
            }
            if self.opts.opt_ro_dsv_dontcare {
                sync_targets.push(v.resource);
                let writes_any_plane =
                    (has_depth && !depth_ro_dc) || (has_stencil && !stencil_ro_dc);
                if writes_any_plane {
                    write_targets.push(v.resource);
                }
            } else {
                let ro = view_flags::DSV_READ_ONLY_DEPTH | view_flags::DSV_READ_ONLY_STENCIL;
                if v.flags & ro != ro {
                    sync_targets.push(v.resource);
                    write_targets.push(v.resource);
                }
            }
        }
        // The pass consumes the discard marks of its attachments (later
        // passes load what this one stores).
        if !self.clears.discards.is_empty() {
            let dr = dsv.map(|v| v.resource);
            self.clears
                .discards
                .retain(|&d| !rtvs.iter().any(|v| v.resource == d) && dr != Some(d));
        }
        if size.0 == usize::MAX {
            // Attachment-less pass (UAV-only rendering): size from the viewport.
            let vp = self
                .gfx
                .viewports
                .first()
                .ok_or("draw without render targets or viewport")?;
            size = (
                ((vp.originX + vp.width).ceil() as usize).max(1),
                ((vp.originY + vp.height).ceil() as usize).max(1),
            );
            rp.setRenderTargetWidth(size.0);
            rp.setRenderTargetHeight(size.1);
            if let Some(n) = self
                .gfx
                .pso
                .as_ref()
                .map(|p| p.sample_count)
                .or(self.gfx.mesh.as_ref().map(|m| m.sample_count))
            {
                rp.setDefaultRasterSampleCount(n as usize);
            }
        }
        stats::inc(&stats::c::draw_pass);
        let e = self
            .render_pass_with_sync(&rp, &sync_targets, &write_targets, opening_pre_raster_reads)
            .ok_or("render encoder")?;
        e.setArgumentTable_atStages(&self.table, MTLRenderStages::Vertex | MTLRenderStages::Fragment);
        self.gfx.draw_pass = true;
        self.gfx.open_rtvs = rtvs;
        self.gfx.open_dsv = dsv;
        self.gfx.had_target_rebind = false;
        self.gfx.dirty = GFX_DIRTY_ALL;
        self.gfx.pass_size = size;
        if let Some(m) = self.pt_cur() {
            (m.w, m.h) = (size.0 as u32, size.1 as u32);
        }
        Ok((e, size))
    }

    fn apply_dynamic(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        p: &GraphicsPipeline,
        size: (usize, usize),
    ) {
        let dirty = std::mem::replace(&mut self.gfx.dirty, 0);
        if dirty == 0 {
            return;
        }
        let fast = self.opts.opt_fast_enc_state;
        if (dirty & GFX_DIRTY_PSO) != 0 {
            self.rp_sync.enc_render_cache.apply_pso(e, &p.pso, fast);
            self.rp_sync.enc_render_cache.apply_raster(e, &p.raster, fast);
            self.rp_sync.enc_render_cache.apply_ds(e, &p.depth_stencil, fast);
        } else if fast && stats::active() {
            stats::add(&stats::c::enc_state_skip, 3);
        }
        if (dirty & GFX_DIRTY_SHARED) != 0 {
            self.rp_sync.enc_render_cache.apply_shared_dirty(
                e,
                size,
                dirty,
                &self.gfx.viewports,
                &self.gfx.scissors,
                self.gfx.blend_factor,
                self.gfx.stencil_ref,
                self.gfx.stencil_back_ref,
                fast,
            );
        }
        if fast && stats::active() {
            let clean_shared = (GFX_DIRTY_SHARED & !dirty).count_ones() as u64;
            if clean_shared > 0 {
                stats::add(&stats::c::enc_state_skip, clean_shared);
            }
        }
    }

    pub(super) fn apply_shared_dynamic_snap(
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        d: &DynSnapshot,
        size: (usize, usize),
    ) {
        Self::apply_shared_dynamic_raw(
            e,
            size,
            &d.viewports,
            &d.scissors,
            d.blend_factor,
            d.stencil_ref,
            d.stencil_back_ref,
        );
    }

    fn apply_shared_dynamic_raw(
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        size: (usize, usize),
        viewports: &[MTLViewport],
        scissors: &[Rect],
        blend_factor: [f32; 4],
        stencil_ref: u32,
        stencil_back_ref: u32,
    ) {
        if stencil_ref == stencil_back_ref {
            e.setStencilReferenceValue(stencil_ref);
        } else {
            e.setStencilFrontReferenceValue_backReferenceValue(stencil_ref, stencil_back_ref);
        }
        e.setBlendColorRed_green_blue_alpha(
            blend_factor[0],
            blend_factor[1],
            blend_factor[2],
            blend_factor[3],
        );
        // Depth bounds: reported unsupported (D3D12_OPTIONS2); the AGX G15
        // driver traps on setDepthTestMinBound, so never call it.
        let mut full = [MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: size.0 as f64,
            height: size.1 as f64,
            znear: 0.0,
            zfar: 1.0,
        }];
        // Scissors are mandatory in D3D12; Metal requires them inside the target.
        let mut sc_buf = [MTLScissorRect { x: 0, y: 0, width: size.0, height: size.1 }; 16];
        let sc_len = if scissors.is_empty() {
            1
        } else {
            let n = scissors.len().min(sc_buf.len());
            for (dst, r) in sc_buf[..n].iter_mut().zip(&scissors[..n]) {
                let x = (r.left.max(0) as usize).min(size.0);
                let y = (r.top.max(0) as usize).min(size.1);
                let r_ = (r.right.max(0) as usize).min(size.0);
                let b = (r.bottom.max(0) as usize).min(size.1);
                *dst = MTLScissorRect {
                    x,
                    y,
                    width: r_.saturating_sub(x),
                    height: b.saturating_sub(y),
                };
            }
            n
        };
        unsafe {
            if viewports.is_empty() {
                e.setViewports_count(std::ptr::NonNull::new(full.as_mut_ptr()).unwrap(), 1);
            } else {
                e.setViewports_count(
                    std::ptr::NonNull::new(viewports.as_ptr() as *mut MTLViewport).unwrap(),
                    viewports.len(),
                );
            }
            e.setScissorRects_count(std::ptr::NonNull::new(sc_buf.as_mut_ptr()).unwrap(), sc_len);
        }
    }

    /// Flush dirty graphics root arguments and/or upload `RuntimeDrawParams` under a
    /// single `UploadRing` lock (zero locks when neither is dirty).
    #[inline]
    pub(super) fn flush_gfx_root_and_draw_params(
        &mut self,
        needs_draw_params: bool,
        params: &[u8],
    ) -> Result<(u64, u64), String> {
        let need_root = self.gfx_root.rs.is_some() && self.gfx_root.is_dirty();
        let (need_params, cached_pa) = if !needs_draw_params {
            (false, 0)
        } else if self.opts.opt_fast_enc_state
            && self.rp_sync.last_draw_params_va != 0
            && params.len() <= 20
            && self.rp_sync.last_draw_params_len as usize == params.len()
            && self.rp_sync.last_draw_params[..params.len()] == *params
        {
            stats::inc(&stats::c::enc_state_skip);
            (false, self.rp_sync.last_draw_params_va)
        } else {
            (true, 0)
        };
        if !need_root && !need_params {
            let root = if self.gfx_root.rs.is_some() { self.gfx_root.bound } else { 0 };
            return Ok((root, cached_pa));
        }
        let ring = self.ring.as_ref().ok_or("command list is not open")?;
        let mut r = ring.lock().unwrap();
        let root = if need_root {
            self.gfx_root.flush(&mut r)?
        } else if self.gfx_root.rs.is_some() {
            self.gfx_root.bound
        } else {
            0
        };
        let pa = if need_params {
            let va = r.upload(params, 16)?;
            if params.len() <= 20 {
                self.rp_sync.last_draw_params[..params.len()].copy_from_slice(params);
                self.rp_sync.last_draw_params_len = params.len() as u8;
                self.rp_sync.last_draw_params_va = va;
            }
            va
        } else {
            cached_pa
        };
        Ok((root, pa))
    }

    #[cfg(test)]
    pub(super) fn upload_draw_params(&mut self, params: &[u8]) -> Result<u64, String> {
        if self.opts.opt_fast_enc_state
            && self.rp_sync.last_draw_params_va != 0
            && params.len() <= 20
            && self.rp_sync.last_draw_params_len as usize == params.len()
            && self.rp_sync.last_draw_params[..params.len()] == *params
        {
            stats::inc(&stats::c::enc_state_skip);
            return Ok(self.rp_sync.last_draw_params_va);
        }
        let ring = self.ring.as_ref().ok_or("command list is not open")?;
        let va = ring.lock().unwrap().upload(params, 16)?;
        if params.len() <= 20 {
            self.rp_sync.last_draw_params[..params.len()].copy_from_slice(params);
            self.rp_sync.last_draw_params_len = params.len() as u8;
            self.rp_sync.last_draw_params_va = va;
        }
        Ok(va)
    }

    fn begin_draw_batched(
        &mut self,
        indexed: Option<u16>,
        params: &[u8],
    ) -> Result<
        (Arc<GraphicsPipeline>, MTLPrimitiveType, u32, DirectBind, Option<Box<DrawSegExact>>),
        String,
    > {
        let p = self.gfx.pso.clone().ok_or("draw without a graphics PSO")?;
        let t = crate::topology::topology(self.gfx.topology)
            .ok_or_else(|| format!("primitive topology {}", self.gfx.topology))?;
        let prim = match t.primitive {
            Some(pr) if t.is_native() => pr,
            _ => {
                return Err(format!(
                    "primitive topology {} needs emulation (adjacency/fan/patch), not implemented",
                    self.gfx.topology
                ))
            }
        };
        let (root, pa) = self.flush_gfx_root_and_draw_params(p.needs_draw_params, params)?;
        let ia = binding::static_index_type_va(indexed.unwrap_or(NON_INDEXED_DRAW))?;
        if self.icb.icb_prepass_pending {
            let meta = self.snapshot_gfx_seg_exact(
                &p,
                indexed.is_some(),
                false,
                false,
                false,
                false,
                Some(&[]),
            );
            let dyn_idx = self.record_batch_dyn_idx();
            let bind = self.snapshot_direct_bind(&p, root, pa, ia);
            return Ok((p, prim, dyn_idx, bind, Some(Box::new(meta))));
        }
        let writes_uav = self.gfx_pso_writes_uav(&p);
        let mut eff_reads = self.effective_gfx_pre_raster_reads(&p, indexed.is_some(), false);
        if p.vs_writes_uav && self.bar.list_unwaited_vtx_frag {
            eff_reads |= READS_TEX;
        }
        let need_pre_open = self.need_pre_open_cur_draw_exact();
        let do_gfx = need_pre_open || self.opts.opt_gfx_desc_table_exact;
        let do_uav = writes_uav && self.opts.opt_rp_split_exact_keep;
        if do_gfx || do_uav {
            self.resolve_gfx_binds_cached(
                &p,
                indexed.is_some(),
                false,
                false,
                false,
                false,
                Some(&[]),
                do_gfx,
                do_uav,
            );
        }
        if need_pre_open {
            self.apply_gfx_bind_cache_to_cur_draw(&p);
        } else {
            self.rp_sync.cur_draw_exact_valid = false;
        }
        self.bar.opening_render_split_ok = !p.vs_writes_uav
            && (!self.has_frag_buf_write()
                || ((self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
                    && self.occlusion.is_none()));
        let enc_res = unsafe { self.draw_encoder_for(eff_reads) };
        self.bar.opening_render_split_ok = false;
        let (_e, _size) = match enc_res {
            Ok(v) => v,
            Err(err) => {
                self.rp_sync.cur_draw_exact_valid = false;
                return Err(err);
            }
        };
        self.record_gfx_draw_pre_barrier(&p, eff_reads, writes_uav, false);
        self.record_gfx_draw_uav_writes(writes_uav, p.vs_writes_uav, p.ps_writes_uav, MTLStages::Vertex);
        self.collect_batch_qbarriers(eff_reads);
        self.rp_sync.cur_draw_exact_valid = false;
        if let Some(m) = self.pt_cur() {
            m.draws += 1;
            m.shader = p.vs.key as u64 ^ p.ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1));
        }
        let dyn_idx = self.record_batch_dyn_idx();
        let bind = self.snapshot_direct_bind(&p, root, pa, ia);
        Ok((p, prim, dyn_idx, bind, None))
    }

    /// Common draw setup; returns the encoder and pipeline.
    #[inline]
    fn begin_draw(
        &mut self,
        indexed: Option<u16>,
        params: &[u8],
    ) -> Result<
        (
            Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>,
            Arc<GraphicsPipeline>,
            MTLPrimitiveType,
        ),
        String,
    > {
        self.begin_draw_ext(indexed, params, false)
    }

    pub(super) fn begin_draw_ext(
        &mut self,
        indexed: Option<u16>,
        params: &[u8],
        indirect: bool,
    ) -> Result<
        (
            Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>,
            Arc<GraphicsPipeline>,
            MTLPrimitiveType,
        ),
        String,
    > {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self.gfx.pso.clone().ok_or("draw without a graphics PSO")?;
        let t = crate::topology::topology(self.gfx.topology)
            .ok_or_else(|| format!("primitive topology {}", self.gfx.topology))?;
        let prim = match t.primitive {
            Some(pr) if t.is_native() => pr,
            _ => {
                return Err(format!(
                    "primitive topology {} needs emulation (adjacency/fan/patch), not implemented",
                    self.gfx.topology
                ))
            }
        };
        self.pt_split();
        let (root, pa) = self.flush_gfx_root_and_draw_params(p.needs_draw_params, params)?;
        let ia = binding::static_index_type_va(indexed.unwrap_or(NON_INDEXED_DRAW))?;
        let writes_uav = self.gfx_pso_writes_uav(&p);
        let check_hazard =
            self.gfx.pass_uav_hazard && self.opts.opt_rp_split_exact_keep && (writes_uav || p.uses_uav);
        let need_pre_open = self.need_pre_open_cur_draw_exact();
        let do_gfx = check_hazard || need_pre_open || self.opts.opt_gfx_desc_table_exact;
        let do_uav = writes_uav && self.opts.opt_rp_split_exact_keep;
        if do_gfx || do_uav {
            self.resolve_gfx_binds_cached(
                &p,
                indexed.is_some(),
                indirect,
                indirect,
                indirect,
                indirect,
                if indirect { None } else { Some(&[]) },
                do_gfx,
                do_uav,
            );
        }
        if self.gfx.pass_uav_hazard {
            if self.opts.opt_rp_split_exact_keep {
                if (writes_uav || p.uses_uav) && self.gfx_cached_overlaps_pass_hazard(indirect) {
                    stats::inc(&stats::c::rb_uav_split);
                    self.end_encoder();
                } else if writes_uav {
                    stats::inc(&stats::c::rp_split_exact_keep);
                }
            } else if writes_uav {
                stats::inc(&stats::c::rb_uav_split);
                self.end_encoder();
            }
        }
        let mut eff_reads = self.effective_gfx_pre_raster_reads(&p, indexed.is_some(), indirect);
        if p.vs_writes_uav && self.bar.list_unwaited_vtx_frag {
            eff_reads |= READS_TEX;
        }
        if need_pre_open {
            self.apply_gfx_bind_cache_to_cur_draw(&p);
        } else {
            self.rp_sync.cur_draw_exact_valid = false;
        }
        self.bar.opening_render_split_ok = !p.vs_writes_uav
            && (!self.has_frag_buf_write()
                || ((self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
                    && self.occlusion.is_none()));
        let enc_res = unsafe { self.draw_encoder_for(eff_reads) };
        self.bar.opening_render_split_ok = false;
        let (e, size) = match enc_res {
            Ok(v) => v,
            Err(err) => {
                self.rp_sync.cur_draw_exact_valid = false;
                return Err(err);
            }
        };
        self.record_gfx_draw_pre_barrier(&p, eff_reads, writes_uav, indirect);
        self.render_mid_barrier(&e);
        self.record_gfx_draw_uav_writes(writes_uav, p.vs_writes_uav, p.ps_writes_uav, MTLStages::Vertex);
        self.sync_pre_raster(&e, eff_reads);
        self.rp_sync.cur_draw_exact_valid = false;
        self.apply_dynamic(&e, &p, size);
        if let Some(m) = self.pt_cur() {
            m.draws += 1;
            m.shader = p.vs.key as u64 ^ p.ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1));
        }
        self.bind_table(root);
        if pa != 0 {
            self.set_table_addr(pa, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
        }
        self.set_table_addr(ia, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
        let mut slots = p.vertex_slots;
        while slots != 0 {
            let s = slots.trailing_zeros();
            slots &= slots - 1;
            let (addr, _size, stride) = self.gfx.vbs[s as usize];
            if addr != 0 {
                self.set_table_vb(addr, stride as usize, s);
            } else if let Ok(zero_va) = binding::static_index_type_va(0) {
                self.set_table_vb(zero_va, 0, s);
            }
        }
        let mut ran = MTLStages::Vertex;
        if !self.bar.pass_targets.is_empty()
            || self.active_dsv().is_some_and(|v| v.resource != 0)
            || p.ps.is_some()
        {
            ran |= MTLStages::Fragment;
        }
        self.note_stage_ran(ran);
        Ok((e, p, prim))
    }

    /// Evaluates active GPU predication (`self.predication`) for an emulated object+mesh draw
    /// (`draw_geometry_emulated` / `draw_tessellation_emulated`). When predication is active,
    /// runs the indirect expansion compute kernel to zero `{x, y, z}` when suppressed and returns
    /// the GPU VA of the 12-byte `MTLDispatchThreadgroupsIndirectArguments` buffer.
    fn prepare_predicated_mesh_grid(
        &mut self,
        x: u32,
        y: u32,
        z: u32,
    ) -> Result<Option<u64>, String> {
        let Some(pred) = self.predication else {
            return Ok(None);
        };
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let mut grid_bytes = [0u8; 12];
        grid_bytes[0..4].copy_from_slice(&x.to_le_bytes());
        grid_bytes[4..8].copy_from_slice(&y.to_le_bytes());
        grid_bytes[8..12].copy_from_slice(&z.to_le_bytes());
        let (args_out, params_va) = {
            let mut r = ring.lock().unwrap();
            let args_in = r.upload(&grid_bytes, 16)?;
            let (out, _) = r.alloc(12, 16)?;
            let (zero_word, zero_n) = Work::DispatchMesh.zero();
            let p = KernelParams {
                args: args_in,
                count: 0,
                args_out: out,
                tlab_out: 0,
                tlab_in: 0,
                base_ib_va: 0,
                ib_offset: 0,
                stride: 12,
                max_count: 1,
                work_offset: 0,
                work_size: 12,
                out_stride: 12,
                zero_word,
                zero_n,
                tlab_size: 0,
                tlab_stride: 0,
                npatch: 0,
                pred: pred.gpu_va,
                pred_op: pred.op,
                ..Default::default()
            };
            (out, r.upload(p.bytes(), 256)?)
        };
        let k = indirect::kernel()?;
        let e = self.compute().ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::EI_EXPAND, 1);
        e.setComputePipelineState(k);
        self.compute_pso_bound = false;
        self.set_table_addr(params_va, 0);
        e.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: 1, height: 1, depth: 1 },
            MTLSize { width: 64, height: 1, depth: 1 },
        );
        self.note_stage_ran(MTLStages::Dispatch);
        stats::inc(&stats::c::ei_expand);
        self.produced(
            MTLStages::Dispatch,
            MTLStages::Vertex
                | MTLStages::Object
                | MTLStages::Mesh
                | MTLStages::Fragment
                | MTLStages::Dispatch,
        );
        Ok(Some(args_out))
    }

    /// Encodes a draw through an MSC geometry-emulation pipeline (Metal 4 object + mesh +
    /// fragment), mirroring Apple's `IRRuntime4Draw[Indexed]PrimitivesGeometryEmulation`.
    fn draw_geometry_emulated(
        &mut self,
        indexed: Option<(u16, u64)>,
        count: u32,
        instances: u32,
        draw_params: &[u8; 20],
    ) -> Result<(), String> {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self.gfx.pso.clone().ok_or("geometry draw without a graphics PSO")?;
        let emu = p.gs_emu.as_ref().ok_or("not a geometry-emulation PSO")?;
        let ir_prim = crate::topology::ir_primitive_type(self.gfx.topology)
            .ok_or_else(|| format!("unsupported geometry-shader topology {}", self.gfx.topology))?;
        let prim_verts: u32 = match ir_prim {
            0 => 1,     // Point
            1 | 2 => 2, // Line, LineStrip
            3 | 4 => 3, // Triangle, TriangleStrip
            5 | 7 => 4, // LineWithAdj, LineStripWithAdj
            6 => 6,     // TriangleWithAdj
            _ => return Err(format!("invalid IRRuntimePrimitiveType {ir_prim}")),
        };
        let overlap: u32 = match ir_prim {
            2 => 1, // LineStrip
            4 => 2, // TriangleStrip
            7 => 3, // LineStripWithAdj
            _ => 0,
        };
        if count <= overlap {
            return Ok(());
        }

        const TOTAL_PAYLOAD_BYTES: u32 = 16384;
        const PAYLOAD_METADATA_BYTES: u32 = 32;
        const MAX_THREADS_PER_TG: u32 = 256;
        let payload_vtx_bytes = TOTAL_PAYLOAD_BYTES - PAYLOAD_METADATA_BYTES;
        let vtx_bytes = emu.vertex_output_size_in_bytes.max(16);
        let max_vtx_by_payload = ((payload_vtx_bytes / vtx_bytes) / prim_verts) * prim_verts;
        let max_input_prims = emu.max_input_primitives_per_mesh_threadgroup.max(1);
        let max_prim_by_amp =
            crate::mesh::MAX_MESH_GRID_THREADGROUPS.saturating_mul(max_input_prims);
        let max_prims_per_obj_tg = (max_vtx_by_payload / prim_verts)
            .min(max_prim_by_amp)
            .min(MAX_THREADS_PER_TG / prim_verts)
            .max(1);
        let obj_tg_vtx_stride = max_prims_per_obj_tg * prim_verts;
        let obj_tg_count_x = (count - overlap).div_ceil(obj_tg_vtx_stride);
        if obj_tg_count_x == 0 {
            return Ok(());
        }
        let obj_tg_size = obj_tg_vtx_stride + overlap;
        let mesh_tg_size = max_input_prims;

        let (index_type, index_buffer) = match indexed {
            Some((ir_idx, ib_va)) => (ir_idx, ib_va),
            None => (NON_INDEXED_DRAW, 0),
        };
        let draw_info = dxil_metal::runtime::RuntimeDrawInfo {
            index_type,
            primitive_topology: ir_prim,
            threads_per_patch: prim_verts as u8,
            max_input_primitives_per_mesh_threadgroup: max_input_prims as u16,
            object_threadgroup_vertex_stride: obj_tg_vtx_stride as u16,
            mesh_threadgroup_primitive_stride: max_input_prims as u16,
            gs_instance_count: instances.min(u16::MAX as u32) as u16,
            patches_per_object_threadgroup: max_prims_per_obj_tg as u16,
            input_control_points_per_patch: prim_verts as u16,
            index_buffer,
        };

        self.pt_split();
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let (root, params_va, info_va, vbs_va) = {
            let mut r = ring.lock().unwrap();
            let root = if self.gfx_root.rs.is_some() {
                if self.gfx_root.is_dirty() {
                    self.gfx_root.flush(&mut r)?
                } else {
                    self.gfx_root.bound
                }
            } else {
                0
            };
            let pa = r.upload(draw_params, 16)?;
            let ia = r.upload(&draw_info.to_bytes(), 16)?;
            let mut vb_bytes = [0u8; 31 * 16];
            for s in 0..31usize {
                let (addr, size, stride) =
                    if s < self.gfx.vbs.len() { self.gfx.vbs[s] } else { (0, 0, 0) };
                let rv = dxil_metal::runtime::RuntimeVertexBuffer { addr, length: size, stride };
                vb_bytes[s * 16..(s + 1) * 16].copy_from_slice(&rv.to_bytes());
            }
            let va = r.upload(&vb_bytes, 16)?;
            (root, pa, ia, va)
        };

        let pred_grid_va = self.prepare_predicated_mesh_grid(obj_tg_count_x, instances, 1)?;
        let writes_uav = self.gfx_pso_writes_uav(&p);
        if self.gfx.pass_uav_hazard && writes_uav {
            self.end_encoder();
        }
        let eff_reads = p.pre_raster_reads | READS_BUF;
        let (e, size) = unsafe { self.draw_encoder_for(eff_reads)? };
        self.rp_sync.enc_pre_raster_reads |= eff_reads;
        self.rp_sync.enc_ps_reads |= p.ps_read_classes;
        self.rp_sync.enc_uav_used |= writes_uav || p.uses_uav;
        self.render_mid_barrier(&e);
        self.record_gfx_draw_uav_writes(
            writes_uav,
            p.vs_writes_uav,
            p.ps_writes_uav,
            MTLStages::Object | MTLStages::Mesh,
        );
        self.sync_pre_raster(&e, eff_reads);
        self.apply_dynamic(&e, &p, size);
        let fast = self.opts.opt_fast_enc_state;
        if !(fast && self.rp_sync.enc_render_cache.mesh_stages_bound) {
            e.setArgumentTable_atStages(&self.table, crate::mesh::STAGES);
            self.rp_sync.enc_render_cache.mesh_stages_bound = true;
        }
        self.bind_table(root);
        self.set_table_addr(params_va, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
        self.set_table_addr(info_va, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
        self.set_table_addr(vbs_va, VERTEX_BUFFER_BIND_POINT as usize);

        let mut ran = MTLStages::Object | MTLStages::Mesh;
        if !self.bar.pass_targets.is_empty()
            || self.active_dsv().is_some_and(|v| v.resource != 0)
            || p.ps.is_some()
        {
            ran |= MTLStages::Fragment;
        }
        self.note_stage_ran(ran);
        if let Some(grid_va) = pred_grid_va {
            e.drawMeshThreadgroupsWithIndirectBuffer_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                grid_va,
                MTLSize { width: obj_tg_size as usize, height: 1, depth: 1 },
                MTLSize { width: mesh_tg_size as usize, height: 1, depth: 1 },
            );
        } else {
            e.drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                MTLSize { width: obj_tg_count_x as usize, height: instances as usize, depth: 1 },
                MTLSize { width: obj_tg_size as usize, height: 1, depth: 1 },
                MTLSize { width: mesh_tg_size as usize, height: 1, depth: 1 },
            );
        }
        Ok(())
    }

    /// Encodes a draw through an MSC tessellation-emulation pipeline (Metal 4 object + mesh +
    /// fragment), mirroring Apple's `IRRuntime4Draw[Indexed]PatchesTessellationEmulation`.
    fn draw_tessellation_emulated(
        &mut self,
        indexed: Option<(u16, u64)>,
        count: u32,
        instances: u32,
        draw_params: &[u8; 20],
    ) -> Result<(), String> {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self.gfx.pso.clone().ok_or("tessellation draw without a graphics PSO")?;
        let emu = p.tess_emu.as_ref().ok_or("not a tessellation-emulation PSO")?;
        let ir_prim = crate::topology::ir_primitive_type(self.gfx.topology)
            .ok_or_else(|| format!("unsupported tessellation topology {}", self.gfx.topology))?;

        let mesh_overlap: u16 = match emu.output_primitive {
            1 => 0, // Point
            2 => 1, // Line
            _ => 2, // TriangleCW / TriangleCCW
        };
        let max_input_prims = emu.gs_max_input_primitives_per_mesh_threadgroup.max(1);
        let patches_per_obj_tg = emu.hs_max_patches_per_object_threadgroup.max(1);
        let input_cps = emu.hs_input_control_point_count.max(1);
        let threads_per_patch = emu.hs_max_object_threads_per_patch.max(1);
        let obj_tg_vtx_stride = patches_per_obj_tg.saturating_mul(input_cps).max(1);
        let obj_tg_count_x = count.div_ceil(obj_tg_vtx_stride);
        if obj_tg_count_x == 0 {
            return Ok(());
        }
        let obj_tg_size = patches_per_obj_tg.saturating_mul(threads_per_patch).max(1);
        let mesh_tg_size = max_input_prims;

        let (index_type, index_buffer) = match indexed {
            Some((ir_idx, ib_va)) => (ir_idx, ib_va),
            None => (NON_INDEXED_DRAW, 0),
        };
        let draw_info = dxil_metal::runtime::RuntimeDrawInfo {
            index_type,
            primitive_topology: ir_prim,
            threads_per_patch: threads_per_patch.min(u8::MAX as u32) as u8,
            max_input_primitives_per_mesh_threadgroup: max_input_prims as u16,
            object_threadgroup_vertex_stride: obj_tg_vtx_stride as u16,
            mesh_threadgroup_primitive_stride: (max_input_prims as u16).saturating_sub(mesh_overlap),
            gs_instance_count: emu.gs_instance_count.min(u16::MAX as u32) as u16,
            patches_per_object_threadgroup: patches_per_obj_tg as u16,
            input_control_points_per_patch: input_cps as u16,
            index_buffer,
        };

        self.pt_split();
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let (root, params_va, info_va, vbs_va) = {
            let mut r = ring.lock().unwrap();
            let root = if self.gfx_root.rs.is_some() {
                if self.gfx_root.is_dirty() {
                    self.gfx_root.flush(&mut r)?
                } else {
                    self.gfx_root.bound
                }
            } else {
                0
            };
            let pa = r.upload(draw_params, 16)?;
            let ia = r.upload(&draw_info.to_bytes(), 16)?;
            let mut vb_bytes = [0u8; 31 * 16];
            for s in 0..31usize {
                let (addr, size, stride) =
                    if s < self.gfx.vbs.len() { self.gfx.vbs[s] } else { (0, 0, 0) };
                let rv = dxil_metal::runtime::RuntimeVertexBuffer { addr, length: size, stride };
                vb_bytes[s * 16..(s + 1) * 16].copy_from_slice(&rv.to_bytes());
            }
            let va = r.upload(&vb_bytes, 16)?;
            (root, pa, ia, va)
        };

        let pred_grid_va = self.prepare_predicated_mesh_grid(obj_tg_count_x, instances, 1)?;
        let writes_uav = self.gfx_pso_writes_uav(&p);
        if self.gfx.pass_uav_hazard && writes_uav {
            self.end_encoder();
        }
        let eff_reads = p.pre_raster_reads | READS_BUF;
        let (e, size) = unsafe { self.draw_encoder_for(eff_reads)? };
        self.rp_sync.enc_pre_raster_reads |= eff_reads;
        self.rp_sync.enc_ps_reads |= p.ps_read_classes;
        self.rp_sync.enc_uav_used |= writes_uav || p.uses_uav;
        self.render_mid_barrier(&e);
        self.record_gfx_draw_uav_writes(
            writes_uav,
            p.vs_writes_uav,
            p.ps_writes_uav,
            MTLStages::Object | MTLStages::Mesh,
        );
        self.sync_pre_raster(&e, eff_reads);
        self.apply_dynamic(&e, &p, size);
        let fast = self.opts.opt_fast_enc_state;
        if !(fast && self.rp_sync.enc_render_cache.mesh_stages_bound) {
            e.setArgumentTable_atStages(&self.table, crate::mesh::STAGES);
            self.rp_sync.enc_render_cache.mesh_stages_bound = true;
        }
        self.bind_table(root);
        // HS (object-linked) and DS (mesh-linked) read the top-level root signature from bind point 3,
        // and the MSC tessellator reads its lookup tables from bind point 7.
        if root != 0 {
            self.set_table_addr(root, ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT as usize);
        }
        let tess_tables_va = binding::tessellator_tables_va()?;
        self.set_table_addr(tess_tables_va, dxil_metal::runtime::TESSELLATOR_TABLES_BIND_POINT as usize);
        self.set_table_addr(params_va, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
        self.set_table_addr(info_va, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
        self.set_table_addr(vbs_va, VERTEX_BUFFER_BIND_POINT as usize);

        let mut ran = MTLStages::Object | MTLStages::Mesh;
        if !self.bar.pass_targets.is_empty()
            || self.active_dsv().is_some_and(|v| v.resource != 0)
            || p.ps.is_some()
        {
            ran |= MTLStages::Fragment;
        }
        self.note_stage_ran(ran);
        unsafe {
            e.setObjectThreadgroupMemoryLength_atIndex(15360, 0);
        }
        if let Some(grid_va) = pred_grid_va {
            e.drawMeshThreadgroupsWithIndirectBuffer_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                grid_va,
                MTLSize { width: obj_tg_size as usize, height: 1, depth: 1 },
                MTLSize { width: mesh_tg_size as usize, height: 1, depth: 1 },
            );
        } else {
            e.drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                MTLSize { width: obj_tg_count_x as usize, height: instances as usize, depth: 1 },
                MTLSize { width: obj_tg_size as usize, height: 1, depth: 1 },
                MTLSize { width: mesh_tg_size as usize, height: 1, depth: 1 },
            );
        }
        Ok(())
    }

    /// `DrawInstanced`.
    pub fn draw(
        &mut self,
        vertices: u32,
        instances: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Result<(), String> {
        if vertices == 0 || instances == 0 {
            return Ok(());
        }
        let args = dxil_metal::runtime::RuntimeDrawParams {
            draw: dxil_metal::runtime::RuntimeDrawArgument {
                vertex_count_per_instance: vertices,
                instance_count: instances,
                start_vertex_location: first_vertex,
                start_instance_location: first_instance,
            },
        };
        if self.gfx.pso.as_ref().is_some_and(|p| p.tess_emu.is_some()) {
            return self.draw_tessellation_emulated(None, vertices, instances, &args.to_bytes());
        }
        if self.gfx.pso.as_ref().is_some_and(|p| p.gs_emu.is_some()) {
            return self.draw_geometry_emulated(None, vertices, instances, &args.to_bytes());
        }
        if self.predication.is_some() {
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let a = ring.lock().unwrap().upload(&args.to_bytes()[..16], 16)?;
            let layout = IndirectLayout {
                stride: 16,
                work: Work::Draw,
                work_offset: 0,
                patches: Vec::new(),
                vb_offsets: Vec::new(),
                ib_offset: None,
            };
            return self.execute_indirect(&layout, 1, a, None, None);
        }
        if !self.icb.icb_segs.is_empty() && !self.check_batched_draw_uav_hazard(false) {
            let (pso, prim, dyn_idx, bind, deferred_meta) =
                self.begin_draw_batched(None, &args.to_bytes())?;
            self.icb.icb_segs.push(DrawSeg {
                pso,
                dyn_idx,
                deferred_meta,
                op: DrawOp::Draw {
                    bind,
                    prim,
                    first_vertex: first_vertex as usize,
                    vertices: vertices as usize,
                    instances: instances as usize,
                    first_instance: first_instance as usize,
                },
            });
            return Ok(());
        }
        let (e, _p, prim) = self.begin_draw(None, &args.to_bytes())?;
        unsafe {
            e.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                prim,
                first_vertex as usize,
                vertices as usize,
                instances as usize,
                first_instance as usize,
            )
        };
        Ok(())
    }

    /// `DrawIndexedInstanced`.
    pub fn draw_indexed(
        &mut self,
        indices: u32,
        instances: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    ) -> Result<(), String> {
        if indices == 0 || instances == 0 {
            return Ok(());
        }
        let (addr, size, fmt) = self.gfx.ib.ok_or("indexed draw without an index buffer")?;
        let f = crate::topology::index_format(fmt)
            .ok_or_else(|| format!("index format {fmt}"))?;
        let off = first_index as u64 * f.size as u64;
        if off + indices as u64 * f.size as u64 > size as u64 {
            return Err(format!(
                "indices {first_index}+{indices} exceed the index buffer ({size} bytes)"
            ));
        }
        let args = dxil_metal::runtime::RuntimeDrawParams {
            draw_indexed: dxil_metal::runtime::RuntimeDrawIndexedArgument {
                index_count_per_instance: indices,
                instance_count: instances,
                start_index_location: first_index,
                base_vertex_location: base_vertex,
                start_instance_location: first_instance,
            },
        };
        if self.gfx.pso.as_ref().is_some_and(|p| p.tess_emu.is_some()) {
            return self.draw_tessellation_emulated(
                Some((f.ir, addr)),
                indices,
                instances,
                &args.to_bytes(),
            );
        }
        if self.gfx.pso.as_ref().is_some_and(|p| p.gs_emu.is_some()) {
            return self.draw_geometry_emulated(
                Some((f.ir, addr)),
                indices,
                instances,
                &args.to_bytes(),
            );
        }
        if self.predication.is_some() {
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let a = ring.lock().unwrap().upload(&args.to_bytes(), 16)?;
            let layout = IndirectLayout {
                stride: 20,
                work: Work::DrawIndexed,
                work_offset: 0,
                patches: Vec::new(),
                vb_offsets: Vec::new(),
                ib_offset: None,
            };
            return self.execute_indirect(&layout, 1, a, None, None);
        }
        if !self.icb.icb_segs.is_empty() && !self.check_batched_draw_uav_hazard(true) {
            let (pso, prim, dyn_idx, bind, deferred_meta) =
                self.begin_draw_batched(Some(f.ir), &args.to_bytes())?;
            if !crate::topology::strip_cut_matches(pso.strip_cut, fmt, self.gfx.topology) {
                log::debug!(
                    target: "d3d",
                    "DrawIndexedInstanced: strip-cut value {} differs from Metal's restart rule (topology {}, format {fmt})",
                    pso.strip_cut,
                    self.gfx.topology
                );
            }
            self.icb.icb_segs.push(DrawSeg {
                pso,
                dyn_idx,
                deferred_meta,
                op: DrawOp::DrawIndexed {
                    bind,
                    prim,
                    indices: indices as usize,
                    index_type: f.mtl,
                    ib_va: addr + off,
                    ib_len: (size as u64 - off) as usize,
                    instances: instances as usize,
                    base_vertex: base_vertex as isize,
                    first_instance: first_instance as usize,
                },
            });
            return Ok(());
        }
        let (e, p, prim) = self.begin_draw(Some(f.ir), &args.to_bytes())?;
        if !crate::topology::strip_cut_matches(p.strip_cut, fmt, self.gfx.topology) {
            log::debug!(
                target: "d3d",
                "DrawIndexedInstanced: strip-cut value {} differs from Metal's restart rule (topology {}, format {fmt})",
                p.strip_cut,
                self.gfx.topology
            );
        }
        unsafe {
            e.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_baseVertex_baseInstance(
                prim,
                indices as usize,
                f.mtl,
                addr + off,
                (size as u64 - off) as usize,
                instances as usize,
                base_vertex as isize,
                first_instance as usize,
            )
        };
        Ok(())
    }

    /// `DispatchMesh(x, y, z)` with the bound mesh PSO (graphics root
    /// state). No draw params, index uniforms or vertex buffers.
    pub fn dispatch_mesh(&mut self, x: u32, y: u32, z: u32) -> Result<(), String> {
        if !crate::mesh::check_dispatch(x, y, z)? {
            return Ok(());
        }
        if self.predication.is_some() {
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let mut bytes = [0u8; 12];
            bytes[0..4].copy_from_slice(&x.to_le_bytes());
            bytes[4..8].copy_from_slice(&y.to_le_bytes());
            bytes[8..12].copy_from_slice(&z.to_le_bytes());
            let args = ring.lock().unwrap().upload(&bytes, 16)?;
            let layout = IndirectLayout {
                stride: 12,
                work: Work::DispatchMesh,
                work_offset: 0,
                patches: Vec::new(),
                vb_offsets: Vec::new(),
                ib_offset: None,
            };
            return self.execute_indirect(&layout, 1, args, None, None);
        }
        let (e, p) = self.begin_mesh(false)?;
        crate::mesh::encode_dispatch(&e, &p, x, y, z)
    }

    /// Mesh draw setup: render pass, table at the mesh stages, pipeline and
    /// dynamic state when dirty, graphics root arguments.
    pub(super) fn begin_mesh(
        &mut self,
        indirect: bool,
    ) -> Result<
        (
            Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>,
            Arc<crate::mesh::MeshPipeline>,
        ),
        String,
    > {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self.gfx.mesh.clone().ok_or("DispatchMesh without a mesh PSO")?;
        self.pt_split();
        let root = self.flush_root(false)?;
        let writes_uav = self.mesh_pso_writes_uav(&p);
        if self.gfx.pass_uav_hazard {
            if self.opts.opt_rp_split_exact_keep {
                if (writes_uav || p.uses_uav) && self.mesh_uav_overlaps_pass_hazard(&p, indirect) {
                    stats::inc(&stats::c::rb_uav_split);
                    self.end_encoder();
                } else if writes_uav {
                    stats::inc(&stats::c::rp_split_exact_keep);
                }
            } else if writes_uav {
                stats::inc(&stats::c::rb_uav_split);
                self.end_encoder();
            }
        }
        let base_reads = if self.opts.opt_sm66_gfx_stage_exact {
            if p.sm66_narrowed {
                stats::inc(&stats::c::sm66_gfx_exact);
            }
            p.sm66_pre_raster_reads
        } else {
            p.pre_raster_reads
        };
        let need_vs_buf = self.vs_buf_defer_exact_active()
            && (!self.opts.opt_sm66_gfx_stage_exact
                || p.sm66_ms_as_reads_cbv
                || indirect
                || ((self.opts.opt_cs_to_ps_overlap
                    || self.opts.opt_cs_to_ps_overlap_v2
                    || self.gfx_to_ps_overlap_active())
                    && self.predication.is_some()));
        let mut eff_reads = base_reads | if need_vs_buf { READS_BUF } else { 0 };
        if p.ms_as_writes_uav && self.bar.list_unwaited_vtx_frag {
            eff_reads |= READS_TEX;
        }
        if self.need_pre_open_cur_draw_exact() {
            self.resolve_cur_draw_mesh_exact(&p, indirect);
        } else {
            self.rp_sync.cur_draw_exact_valid = false;
        }
        self.bar.opening_render_split_ok = !p.ms_as_writes_uav
            && (!self.has_frag_buf_write()
                || ((self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
                    && self.occlusion.is_none()));
        let enc_res = unsafe { self.draw_encoder_for(eff_reads) };
        self.bar.opening_render_split_ok = false;
        let (e, size) = match enc_res {
            Ok(v) => v,
            Err(err) => {
                self.rp_sync.cur_draw_exact_valid = false;
                return Err(err);
            }
        };
        self.rp_sync.enc_pre_raster_reads |= eff_reads;
        self.rp_sync.enc_pre_raster_srv_uav_reads |= base_reads;
        self.rp_sync.enc_ps_reads |= p.ps_read_classes;
        self.rp_sync.enc_ps_srv_uav_reads |= p.ps_srv_uav_classes;
        if self.hk_any_active() {
            let (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk) = self.mesh_pso_stage_hk(&p);
            self.rp_sync.enc_pre_raster_hk |= pre_hk;
            self.rp_sync.enc_ps_hk |= ps_hk;
            if !self.opts.opt_gfx_desc_table_exact {
                self.rp_sync.enc_pre_raster_unb_hk |= pre_unb_hk | pre_hk;
                self.rp_sync.enc_ps_unb_hk |= ps_unb_hk | ps_hk;
            }
        }
        self.rp_sync.enc_uav_used |= writes_uav || p.uses_uav;
        if self.opts.opt_gfx_desc_table_exact {
            if self.rp_sync.cur_draw_exact_valid {
                self.commit_cur_draw_exact_to_enc();
            } else {
                self.record_mesh_stage_exact_binds(&p, indirect);
            }
        }
        if writes_uav && self.opts.opt_rp_split_exact_keep {
            self.record_mesh_uav_write_exact(&p, indirect);
        }
        self.render_mid_barrier(&e);
        self.record_gfx_draw_uav_writes(
            writes_uav,
            p.ms_as_writes_uav,
            p.ps_writes_uav,
            MTLStages::Object | MTLStages::Mesh,
        );
        self.sync_pre_raster(&e, eff_reads);
        self.rp_sync.cur_draw_exact_valid = false;
        if let Some(m) = self.pt_cur() {
            m.draws += 1;
            m.shader =
                p.mesh.key as u64 ^ p.pixel.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1));
        }
        // The table must be visible to the object and mesh stages too
        // (Vertex stays bound for ordinary draws in the same pass).
        let fast = self.opts.opt_fast_enc_state;
        if fast && self.rp_sync.enc_render_cache.mesh_stages_bound {
            stats::inc(&stats::c::enc_state_skip);
        } else {
            e.setArgumentTable_atStages(&self.table, crate::mesh::STAGES);
            self.rp_sync.enc_render_cache.mesh_stages_bound = true;
        }
        let dirty = std::mem::replace(&mut self.gfx.dirty, 0);
        if dirty != 0 {
            if (dirty & GFX_DIRTY_PSO) != 0 {
                self.rp_sync.enc_render_cache.apply_pso(&e, &p.pso, fast);
                self.rp_sync.enc_render_cache.apply_raster(&e, &p.raster, fast);
                self.rp_sync.enc_render_cache.apply_ds(&e, &p.depth_stencil, fast);
            }
            if (dirty & GFX_DIRTY_SHARED) != 0 {
                self.rp_sync.enc_render_cache.apply_shared_dirty(
                    &e,
                    size,
                    dirty,
                    &self.gfx.viewports,
                    &self.gfx.scissors,
                    self.gfx.blend_factor,
                    self.gfx.stencil_ref,
                    self.gfx.stencil_back_ref,
                    fast,
                );
            }
        }
        self.bind_table(root);
        let mut ran = MTLStages::Mesh;
        if p.object_threads.is_some() {
            ran |= MTLStages::Object;
        }
        if !self.bar.pass_targets.is_empty()
            || self.gfx.dsv.is_some_and(|v| v.resource != 0)
            || p.pixel.is_some()
        {
            ran |= MTLStages::Fragment;
        }
        self.note_stage_ran(ran);
        Ok((e, p))
    }
}
