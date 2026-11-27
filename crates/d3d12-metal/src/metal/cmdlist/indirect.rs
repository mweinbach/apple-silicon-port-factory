//! `ExecuteIndirect`, GPU ICB batching & expansion, and ICB scratch/param allocation.

use super::exact_binds::mark_and_extend_unique;
use super::opts::{
    barrier_masks, icb_reuse_enabled, ios_icb_pass_cap, legacy_barriers, IOS_ICB_PASS_SPLITS,
};
use super::state::{
    merge_req_all, merge_req_for_after, merge_vtx, sub_stages_preserving_all, DirectBind,
    DispatchExact, DrawOp, DrawSeg, DynSnapshot, Encoder, FullWhy, GfxState, COMPUTE_STAGES,
    GFX_DIRTY_ALL, NO_VTX, PRE_RASTER, STAGE_FRAGMENT,
};
use super::CommandList;
use crate::metal::binding::{self, MtlArgumentTable};
use crate::metal::clear;
use crate::metal::encstats as stats;
use crate::metal::indirect::{self, IndirectLayout, KernelParams, TessGridParams, Work};
use crate::metal::indirect_icb::{self, IcbParams, MtlIcb, PrepBatch, PrepItem, MAX_PREP_BATCH};
use crate::metal::passtiming as pt;
use crate::metal::pipeline::{
    resolve_pso_hk_profile, vertex_buffer_index, GraphicsPipeline, HandleKindProfile,
    MtlRenderPipeline, MAX_VERTEX_SLOTS, READS_BUF, READS_TEX,
};
use crate::metal::resource::{self, SubRange, HK_CBV_BUF, HK_RAW_STRUCT_BUF};
use dxil_metal::runtime::{
    ARGUMENT_BUFFER_BIND_POINT, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT,
    ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT,
    DESCRIPTOR_HEAP_BIND_POINT, NON_INDEXED_DRAW, SAMPLER_HEAP_BIND_POINT,
    TESSELLATOR_TABLES_BIND_POINT, VERTEX_BUFFER_BIND_POINT,
};
use objc2::rc::Retained;
use objc2_metal::{
    MTL4ArgumentTable, MTL4CommandEncoder, MTL4ComputeCommandEncoder, MTL4RenderCommandEncoder,
    MTL4VisibilityOptions, MTLIndirectCommandBuffer, MTLPrimitiveType, MTLRenderStages,
    MTLResourceID, MTLSize, MTLStages, MTLVisibilityResultMode,
};
use std::sync::Arc;

impl CommandList {
    fn same_dyn(d: &DynSnapshot, g: &GfxState) -> bool {
        d.stencil_ref == g.stencil_ref
            && d.stencil_back_ref == g.stencil_back_ref
            && d.blend_factor
                .iter()
                .zip(&g.blend_factor)
                .all(|(a, b)| a.to_bits() == b.to_bits())
            && d.viewports.len() == g.viewports.len()
            && d.viewports.iter().zip(&g.viewports).all(|(a, b)| {
                a.originX.to_bits() == b.originX.to_bits()
                    && a.originY.to_bits() == b.originY.to_bits()
                    && a.width.to_bits() == b.width.to_bits()
                    && a.height.to_bits() == b.height.to_bits()
                    && a.znear.to_bits() == b.znear.to_bits()
                    && a.zfar.to_bits() == b.zfar.to_bits()
            })
            && d.scissors.len() == g.scissors.len()
            && d.scissors.iter().zip(&g.scissors).all(|(a, b)| {
                a.left == b.left && a.top == b.top && a.right == b.right && a.bottom == b.bottom
            })
    }

    pub(super) fn record_batch_dyn_idx(&mut self) -> u32 {
        if let Some((idx, last)) = self.icb.icb_dyn.iter().enumerate().next_back() {
            if Self::same_dyn(last, &self.gfx) {
                return idx as u32;
            }
        }
        let idx = self.icb.icb_dyn.len() as u32;
        self.icb.icb_dyn.push(DynSnapshot {
            viewports: self.gfx.viewports.clone(),
            scissors: self.gfx.scissors.clone(),
            blend_factor: self.gfx.blend_factor,
            stencil_ref: self.gfx.stencil_ref,
            stencil_back_ref: self.gfx.stencil_back_ref,
        });
        idx
    }

    pub(super) fn push_batch_qbarrier(&mut self, after: MTLStages, before: MTLStages) {
        if after.is_empty() || before.is_empty() {
            return;
        }
        if self.mid_qb_stagesync_active() {
            self.record_enc_queue_sync(after, before);
        }
        for existing in &mut self.icb.icb_qbarriers {
            if existing.0 == after {
                existing.1 |= before;
                return;
            }
            if existing.1 == before {
                existing.0 |= after;
                return;
            }
        }
        self.icb.icb_qbarriers.push((after, before));
    }

    pub(super) fn collect_batch_qbarriers(&mut self, reads: u8, pre_b: MTLStages) {
        let pre_b = if legacy_barriers() || !barrier_masks() {
            PRE_RASTER
        } else {
            let st = pre_b & PRE_RASTER;
            if st.is_empty() { PRE_RASTER } else { st }
        };
        let omitted_pre = PRE_RASTER - pre_b;
        self.collect_pre_raster_stage_sync(pre_b);
        let dom_exact = self.render_dom_exact_active();
        if self.render_split_sync_active() {
            if !self.bar.split_pend.is_empty() {
                self.reset_frag_rt_pend();
                let mut orig_vtx =
                    std::mem::replace(&mut self.bar.split_pend.vtx_after, MTLStages::empty());
                let mut vtx_req = std::mem::take(&mut self.bar.split_pend.vtx_req);
                let orig_frag =
                    std::mem::replace(&mut self.bar.split_pend.frag_after, MTLStages::empty());
                let frag_req = std::mem::take(&mut self.bar.split_pend.frag_req);
                if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                    let mut deferred = false;
                    let mut deferred_inlist_frag = false;
                    for i in 0..2 {
                        let sa = std::mem::replace(
                            &mut self.bar.split_pend.soft_after[i],
                            MTLStages::empty(),
                        );
                        let sb = std::mem::replace(
                            &mut self.bar.split_pend.soft_before[i],
                            MTLStages::empty(),
                        ) & PRE_RASTER;
                        let sr = std::mem::take(&mut self.bar.split_pend.soft_req[i]);
                        let mut sp_items = std::mem::take(&mut self.bar.soft_pend_items[i]);
                        let sp_poison =
                            std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
                        let mut rem_sa = sub_stages_preserving_all(sa, orig_vtx);
                        if self.opts.opt_gfx_overlap_capped
                            && !self.gfx_to_ps_overlap_active()
                            && rem_sa != MTLStages::All
                            && rem_sa.contains(MTLStages::Fragment)
                        {
                            orig_vtx |= MTLStages::Fragment;
                            vtx_req[STAGE_FRAGMENT] =
                                vtx_req[STAGE_FRAGMENT].max(sr[STAGE_FRAGMENT]);
                            rem_sa = sub_stages_preserving_all(rem_sa, MTLStages::Fragment);
                        }
                        if !rem_sa.is_empty() && !sb.is_empty() {
                            if orig_vtx != MTLStages::empty() {
                                Self::strip_soft_items_after(&mut sp_items, orig_vtx);
                            }
                            if rem_sa != MTLStages::All && rem_sa.contains(MTLStages::Fragment) {
                                deferred_inlist_frag = true;
                            }
                            self.bar.vtx_defer[i] =
                                merge_vtx(self.bar.vtx_defer[i], Some((rem_sa, sb)));
                            merge_req_for_after(&mut self.bar.vtx_defer_req[i], rem_sa, &sr);
                            self.bar.vtx_defer_poisoned[i] |= sp_poison;
                            if !self.bar.vtx_defer_poisoned[i] {
                                Self::extend_soft_items(&mut self.bar.vtx_defer_items[i], sp_items);
                            } else {
                                self.bar.vtx_defer_items[i].clear();
                            }
                            deferred = true;
                        }
                    }
                    if deferred_inlist_frag && self.opts.opt_gfx_overlap_capped {
                        self.rp_sync.gfx_ps_overlap_chain =
                            self.rp_sync.gfx_ps_overlap_chain.saturating_add(1);
                        stats::inc(&stats::c::gfx_overlap_capped);
                    }
                    if deferred {
                        stats::inc(&stats::c::dv_defer);
                        if self.opts.opt_split_res_soft_sync {
                            stats::inc(&stats::c::res_soft_sync);
                        }
                    }
                } else {
                    for i in 0..2 {
                        let sa = std::mem::replace(
                            &mut self.bar.split_pend.soft_after[i],
                            MTLStages::empty(),
                        );
                        let sb = std::mem::replace(
                            &mut self.bar.split_pend.soft_before[i],
                            MTLStages::empty(),
                        );
                        let sr = std::mem::take(&mut self.bar.split_pend.soft_req[i]);
                        self.bar.soft_pend_items[i].clear();
                        self.bar.soft_pend_poisoned[i] = false;
                        if !sa.is_empty() && sb.intersects(PRE_RASTER) {
                            orig_vtx |= sa;
                            merge_req_for_after(&mut vtx_req, sa, &sr);
                        }
                    }
                }
                self.sync_legacy_from_split();
                if !(orig_vtx | orig_frag).is_empty() {
                    if !orig_vtx.is_empty() && !omitted_pre.is_empty() {
                        self.bar.enc_pre_unwaited_stages |= omitted_pre;
                        self.bar.enc_pre_unwaited_after |= orig_vtx;
                        merge_req_for_after(&mut self.bar.enc_pre_unwaited_req, orig_vtx, &vtx_req);
                    }
                    if !(orig_frag - orig_vtx).is_empty() {
                        stats::inc(&stats::c::render_split_sync);
                    }
                    let mid_on = self.mid_qb_stagesync_active();
                    let (eff_vtx, eff_frag) = if dom_exact {
                        let ev = if orig_vtx.is_empty() {
                            MTLStages::empty()
                        } else {
                            let u = self.unsatisfied_enc_after_for_req(
                                orig_vtx,
                                pre_b,
                                &vtx_req,
                                mid_on,
                            );
                            self.fold_dominated_after(u, &vtx_req)
                        };
                        let ef = if orig_frag.is_empty() {
                            MTLStages::empty()
                        } else {
                            let u = self.unsatisfied_enc_after_for_req(
                                orig_frag,
                                MTLStages::Fragment,
                                &frag_req,
                                mid_on,
                            );
                            self.fold_dominated_after(u, &frag_req)
                        };
                        if stats::active() {
                            let base_vtx = if orig_vtx.is_empty() {
                                MTLStages::empty()
                            } else {
                                self.unsatisfied_mid_qb_after(orig_vtx, pre_b)
                            };
                            let base_frag = if orig_frag.is_empty() {
                                MTLStages::empty()
                            } else {
                                self.unsatisfied_mid_qb_after(orig_frag, MTLStages::Fragment)
                            };
                            if ev != base_vtx || ef != base_frag {
                                stats::inc(&stats::c::render_dom_fold);
                            }
                        }
                        (ev, ef)
                    } else {
                        let base_vtx = if orig_vtx.is_empty() {
                            MTLStages::empty()
                        } else {
                            self.unsatisfied_mid_qb_after(orig_vtx, pre_b)
                        };
                        let base_frag = if orig_frag.is_empty() {
                            MTLStages::empty()
                        } else {
                            self.unsatisfied_mid_qb_after(orig_frag, MTLStages::Fragment)
                        };
                        (base_vtx, base_frag)
                    };
                    if (eff_vtx | eff_frag).is_empty() {
                        stats::inc(&stats::c::stagesync_skip);
                    } else {
                        if eff_vtx != orig_vtx || eff_frag != orig_frag {
                            stats::inc(&stats::c::stagesync_narrow);
                        }
                        let both = eff_vtx & eff_frag;
                        let vtx_only = eff_vtx - both;
                        let frag_only = eff_frag - both;
                        if !both.is_empty() {
                            self.push_batch_qbarrier(both, pre_b | MTLStages::Fragment);
                        }
                        if !vtx_only.is_empty() {
                            self.push_batch_qbarrier(vtx_only, pre_b);
                        }
                        if !frag_only.is_empty() {
                            self.push_batch_qbarrier(frag_only, MTLStages::Fragment);
                        }
                        if mid_on {
                            let rec_vtx =
                                if self.opts.opt_rp_keep_write_exact { eff_vtx } else { orig_vtx };
                            let rec_frag =
                                if self.opts.opt_rp_keep_write_exact { eff_frag } else { orig_frag };
                            if !rec_vtx.is_empty() {
                                self.record_enc_queue_sync(rec_vtx, pre_b);
                                if rec_vtx.contains(MTLStages::Fragment)
                                    || rec_vtx == MTLStages::All
                                {
                                    self.rp_sync.gfx_ps_overlap_chain = 0;
                                }
                                if (self.opts.opt_cs_to_ps_overlap_v2
                                    || self.list_rsplit_v2_active())
                                    && dom_exact
                                {
                                    self.prune_satisfied_vtx_defer(mid_on);
                                }
                            }
                            if !rec_frag.is_empty() {
                                self.record_enc_queue_sync(rec_frag, MTLStages::Fragment);
                            }
                        } else if eff_vtx.contains(MTLStages::Fragment)
                            || eff_vtx == MTLStages::All
                        {
                            self.rp_sync.gfx_ps_overlap_chain = 0;
                        }
                        stats::inc(&stats::c::rb_mid_qb);
                        let clear_written = if self.opts.opt_rp_keep_write_exact {
                            eff_frag.contains(MTLStages::Fragment)
                                || (orig_frag.contains(MTLStages::Fragment)
                                    && self.bar.enc_cur_sync[STAGE_FRAGMENT].gen[STAGE_FRAGMENT]
                                        == self.bar.stage_gen[STAGE_FRAGMENT])
                        } else {
                            orig_frag.contains(MTLStages::Fragment)
                        };
                        if clear_written {
                            self.bar.written.clear();
                        }
                    }
                }
            }
        } else if !self.bar.pend_after.is_empty() {
            self.reset_frag_rt_pend();
            const RENDER: MTLStages = PRE_RASTER.union(MTLStages::Fragment);
            if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                let hard_before = self.bar.pend_before & RENDER;
                let mut deferred = false;
                for i in 0..2 {
                    self.bar.soft_pend_items[i].clear();
                    self.bar.soft_pend_poisoned[i] = false;
                    let s = std::mem::replace(
                        &mut self.bar.pend_before_soft[i],
                        MTLStages::empty(),
                    ) - hard_before;
                    if !s.is_empty() {
                        if s.intersects(PRE_RASTER)
                            && self.bar.pend_after != MTLStages::All
                            && self.bar.pend_after.contains(MTLStages::Fragment)
                            && self.opts.opt_gfx_overlap_capped
                        {
                            self.rp_sync.gfx_ps_overlap_chain =
                                self.rp_sync.gfx_ps_overlap_chain.saturating_add(1);
                        }
                        self.bar.vtx_defer[i] =
                            merge_vtx(self.bar.vtx_defer[i], Some((self.bar.pend_after, s)));
                        merge_req_for_after(
                            &mut self.bar.vtx_defer_req[i],
                            self.bar.pend_after,
                            &self.bar.stage_gen,
                        );
                        self.bar.vtx_defer_poisoned[i] = true;
                        self.bar.vtx_defer_items[i].clear();
                        deferred = true;
                    }
                }
                if deferred {
                    stats::inc(&stats::c::dv_defer);
                    if self.opts.opt_split_res_soft_sync {
                        stats::inc(&stats::c::res_soft_sync);
                    }
                }
                if !hard_before.is_empty() {
                    let orig_after = self.bar.pend_after;
                    let eff_after = self.unsatisfied_mid_qb_after(orig_after, hard_before);
                    if eff_after.is_empty() {
                        stats::inc(&stats::c::stagesync_skip);
                    } else {
                        if eff_after != orig_after {
                            stats::inc(&stats::c::stagesync_narrow);
                        }
                        self.push_batch_qbarrier(eff_after, hard_before);
                        if hard_before.intersects(PRE_RASTER)
                            && (eff_after.contains(MTLStages::Fragment)
                                || eff_after == MTLStages::All)
                        {
                            self.rp_sync.gfx_ps_overlap_chain = 0;
                        }
                        if self.mid_qb_stagesync_active() && eff_after != orig_after {
                            self.record_enc_queue_sync(orig_after, hard_before);
                        }
                        stats::inc(&stats::c::rb_mid_qb);
                        if eff_after.contains(MTLStages::Fragment)
                            && hard_before.contains(MTLStages::Fragment)
                        {
                            self.bar.written.clear();
                        }
                    }
                    self.bar.pend_before -= RENDER;
                }
                if self.bar.pend_before.is_empty() {
                    self.bar.pend_after = MTLStages::empty();
                }
            } else {
                let soft = self.bar.pend_before_soft[0] | self.bar.pend_before_soft[1];
                let before = (self.bar.pend_before | soft) & RENDER;
                if !before.is_empty() {
                    let orig_after = self.bar.pend_after;
                    let eff_after = self.unsatisfied_mid_qb_after(orig_after, before);
                    if eff_after.is_empty() {
                        stats::inc(&stats::c::stagesync_skip);
                    } else {
                        if eff_after != orig_after {
                            stats::inc(&stats::c::stagesync_narrow);
                        }
                        self.push_batch_qbarrier(eff_after, before);
                        if before.intersects(PRE_RASTER)
                            && (eff_after.contains(MTLStages::Fragment)
                                || eff_after == MTLStages::All)
                        {
                            self.rp_sync.gfx_ps_overlap_chain = 0;
                        }
                        if self.mid_qb_stagesync_active() && eff_after != orig_after {
                            self.record_enc_queue_sync(orig_after, before);
                        }
                        stats::inc(&stats::c::rb_mid_qb);
                        if eff_after.contains(MTLStages::Fragment)
                            && before.contains(MTLStages::Fragment)
                        {
                            self.bar.written.clear();
                        }
                    }
                    self.bar.pend_before_soft = [MTLStages::empty(); 2];
                    for i in 0..2 {
                        self.bar.soft_pend_items[i].clear();
                        self.bar.soft_pend_poisoned[i] = false;
                    }
                    self.bar.pend_before -= RENDER;
                    if self.bar.pend_before.is_empty() {
                        self.bar.pend_after = MTLStages::empty();
                    }
                }
            }
        }
        if reads != 0 && self.bar.vtx_defer != NO_VTX {
            let mut sync = None;
            let mut sync_req = [0u32; 7];
            for i in 0..2 {
                if reads & (1 << i) != 0 {
                    if let Some((da, db)) = self.bar.vtx_defer[i].take() {
                        let dr = std::mem::take(&mut self.bar.vtx_defer_req[i]);
                        let mut d_items = std::mem::take(&mut self.bar.vtx_defer_items[i]);
                        let d_poison =
                            std::mem::replace(&mut self.bar.vtx_defer_poisoned[i], false);
                        if let Some((hit_a, hit_b, hit_r, rem_opt)) = self
                            .partition_soft_items_for_cur_draw(
                                i,
                                &mut d_items,
                                d_poison,
                                da,
                                db,
                                &dr,
                                false,
                            )
                        {
                            if !hit_a.is_empty() && !hit_b.is_empty() {
                                merge_req_for_after(&mut sync_req, hit_a, &hit_r);
                                sync = merge_vtx(sync, Some((hit_a, hit_b)));
                            }
                            if let Some((rem_a, rem_b, rem_r)) = rem_opt {
                                self.bar.vtx_defer[i] = Some((rem_a, rem_b));
                                self.bar.vtx_defer_req[i] = rem_r;
                                self.bar.vtx_defer_items[i] = d_items;
                            }
                        } else {
                            merge_req_all(&mut sync_req, &dr);
                            sync = merge_vtx(sync, Some((da, db)));
                        }
                    } else {
                        self.bar.vtx_defer_req[i] = [0; 7];
                        self.bar.vtx_defer_items[i].clear();
                        self.bar.vtx_defer_poisoned[i] = false;
                    }
                }
            }
            if let Some((after, before)) = sync {
                let emit_before = {
                    let narrowed = before & pre_b;
                    if narrowed.is_empty() { before } else { narrowed }
                };
                let omitted = before - emit_before;
                if !omitted.is_empty() && !after.is_empty() {
                    self.bar.enc_pre_unwaited_stages |= omitted;
                    self.bar.enc_pre_unwaited_after |= after;
                    merge_req_for_after(&mut self.bar.enc_pre_unwaited_req, after, &sync_req);
                }
                let mid_on = self.mid_qb_stagesync_active();
                let eff_after = if dom_exact {
                    let u =
                        self.unsatisfied_enc_after_for_req(after, emit_before, &sync_req, mid_on);
                    let f = self.fold_dominated_after(u, &sync_req);
                    if stats::active() && f != self.unsatisfied_mid_qb_after(after, emit_before) {
                        stats::inc(&stats::c::render_dom_fold);
                    }
                    f
                } else {
                    self.unsatisfied_mid_qb_after(after, emit_before)
                };
                if eff_after.is_empty() {
                    stats::inc(&stats::c::stagesync_skip);
                } else {
                    if eff_after != after {
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                    self.push_batch_qbarrier(eff_after, emit_before);
                    if eff_after.contains(MTLStages::Fragment) || eff_after == MTLStages::All {
                        self.rp_sync.gfx_ps_overlap_chain = 0;
                    }
                    if mid_on {
                        if self.opts.opt_rp_keep_write_exact {
                            self.record_enc_queue_sync(eff_after, emit_before);
                        } else if eff_after != after {
                            self.record_enc_queue_sync(after, emit_before);
                        }
                    }
                    stats::inc(&stats::c::dv_hit);
                }
            }
        }
        self.finish_post_draw0_list_frag();
    }

    pub(super) fn snapshot_direct_bind(
        &self,
        p: &GraphicsPipeline,
        root: u64,
        pa: u64,
        ia: u64,
    ) -> DirectBind {
        let mut vbs = Vec::with_capacity(p.vertex_slots.count_ones() as usize);
        let zero_va = binding::static_index_type_va(0).unwrap_or(0);
        let mut slots = p.vertex_slots;
        while slots != 0 {
            let s = slots.trailing_zeros();
            slots &= slots - 1;
            let (addr, _size, stride) = self.gfx.vbs[s as usize];
            if addr != 0 {
                vbs.push((s, addr, stride as usize));
            } else if zero_va != 0 {
                vbs.push((s, zero_va, 0));
            }
        }
        DirectBind { heaps: self.heaps, root, pa, ia, vbs }
    }

    fn apply_direct_bind(&mut self, bind: &DirectBind) {
        if bind.heaps[0] != 0 {
            self.set_table_addr(bind.heaps[0], DESCRIPTOR_HEAP_BIND_POINT as usize);
        }
        if bind.heaps[1] != 0 {
            self.set_table_addr(bind.heaps[1], SAMPLER_HEAP_BIND_POINT as usize);
        }
        if bind.root != 0 {
            self.set_table_addr(bind.root, ARGUMENT_BUFFER_BIND_POINT as usize);
        }
        if bind.pa != 0 {
            self.set_table_addr(bind.pa, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
        }
        self.set_table_addr(bind.ia, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
        for &(slot, addr, stride) in &bind.vbs {
            self.set_table_vb(addr, stride, slot);
        }
    }

    /// Flush buffered `AW2_ICB_BATCH=1` segments into the open render encoder:
    /// 1. Emit all harvested mid-pass/deferred queue barriers.
    /// 2. Hoist all `aw2_ei_icb_vs_v2` `Point` vertex-stage expansions into a
    ///    single burst followed by **1** `Vertex -> Vertex|Object|Mesh|Fragment`
    ///    intra-encoder barrier (`icb_hoist_saved += K - 1`).
    /// 3. Replay all indirect ICB executions and interleaved direct draws in
    ///    exact recorded order.
    pub(super) fn flush_draw_batch(&mut self, keep_open: bool) {
        if self.icb.icb_segs.is_empty() {
            self.icb.icb_prepass_pending = false;
            return;
        }
        let segs = std::mem::take(&mut self.icb.icb_segs);
        let dyns = std::mem::take(&mut self.icb.icb_dyn);
        let mut qbs = std::mem::take(&mut self.icb.icb_qbarriers);
        let was_prepass = std::mem::replace(&mut self.icb.icb_prepass_pending, false);
        let icb_count = segs.iter().filter(|s| matches!(s.op, DrawOp::Icb { .. })).count();

        if was_prepass {
            self.icb.icb_prepass_flushing = true;
            if icb_count > 0 {
                let want_spans = self.opts.opt_subrange_uav_barrier;
                let mut exp_reads: Vec<usize> = Vec::with_capacity(icb_count * 2);
                let mut exp_spans: Vec<SubRange> = Vec::new();
                let va_cache = &mut *self.binds.buf_va_cache;
                let exp_ok = resource::with_resource_registry(|reg| {
                    let mut ok = true;
                    for seg in &segs {
                        if let DrawOp::Icb { args_va, args_bytes, count_va, pred_va, .. } = &seg.op
                        {
                            if *args_va != 0 {
                                let aok = if want_spans {
                                    reg.push_buffer_span_cached(
                                        va_cache,
                                        *args_va,
                                        *args_bytes,
                                        &mut exp_reads,
                                        &mut exp_spans,
                                    )
                                } else {
                                    reg.push_buffer_va_cached(va_cache, *args_va, &mut exp_reads)
                                };
                                if !aok {
                                    ok = false;
                                }
                            }
                            if *count_va != 0 {
                                let cok = if want_spans {
                                    reg.push_buffer_span_cached(
                                        va_cache,
                                        *count_va,
                                        4,
                                        &mut exp_reads,
                                        &mut exp_spans,
                                    )
                                } else {
                                    reg.push_buffer_va_cached(va_cache, *count_va, &mut exp_reads)
                                };
                                if !cok {
                                    ok = false;
                                }
                            }
                            if *pred_va != 0 {
                                let pok = if want_spans {
                                    reg.push_buffer_span_cached(
                                        va_cache,
                                        *pred_va,
                                        8,
                                        &mut exp_reads,
                                        &mut exp_spans,
                                    )
                                } else {
                                    reg.push_buffer_va_cached(va_cache, *pred_va, &mut exp_reads)
                                };
                                if !pok {
                                    ok = false;
                                }
                            }
                        }
                    }
                    ok
                });
                let ce_opt = if self.ib_tight_active() {
                    let exact = (self.opts.opt_ib_exact_obl
                        || self.opts.opt_subrange_uav_barrier
                        || self.opts.opt_desc_table_res_exact
                        || self.opts.opt_desc_table_cs_fast
                        || self.opts.opt_cs_intra_barrier_exact
                        || self.opts.opt_cs_intra_barrier_v2
                        || self.cs_intra_safe_active()
                        || self.ps_to_cs_active()
                        || self.opts.opt_cs_mid_qb_touch_defer
                        || self.opts.opt_cs_mid_qb_exact_v2
                        || self.opts.opt_cs_mid_qb_defer_v3
                        || self.opts.opt_cs_mid_qb_unpoison_v4
                        || self.opts.opt_cs_mid_qb_res_defer_v5
                        || self.hk_any_active()
                        || self.msc_class_active())
                        .then_some(DispatchExact {
                            uav_exact: true,
                            full_read_exact: exp_ok,
                            bounded_read_exact: exp_ok,
                            touches_textures: false,
                            unbounded_srv_classes: 0,
                            uav_classes: 0,
                            read_classes: READS_BUF,
                            exact_read_only_classes: READS_BUF,
                            uav_srv_classes: 0,
                            hk: HandleKindProfile {
                                uav_write_kinds: 0,
                                uav_srv_kinds: HK_RAW_STRUCT_BUF,
                                ro_srv_kinds: HK_RAW_STRUCT_BUF,
                                read_kinds: HK_RAW_STRUCT_BUF | HK_CBV_BUF,
                                unbounded_srv_kinds: 0,
                            },
                            uavs: &[],
                            bounded_reads: if exp_ok { exp_reads.as_slice() } else { &[] },
                            uav_spans: (want_spans && exp_ok).then_some(exp_spans.as_slice()),
                            bounded_read_spans: (want_spans && exp_ok)
                                .then_some(exp_spans.as_slice()),
                        });
                    self.compute_op_tight_exact(
                        MTLStages::Dispatch,
                        Some(&[]),
                        exp_ok.then_some(exp_reads.as_slice()),
                        exp_ok.then_some(exp_reads.as_slice()),
                        exact,
                    )
                } else {
                    self.compute()
                };
                if let (Some(ce), Some(ring_arc)) = (ce_opt, self.ring.clone()) {
                    let scratch_arc =
                        self.icb.icb_scratch.clone().unwrap_or_else(|| ring_arc.clone());
                    let mut clamped_jobs: Vec<(u64, u64)> = Vec::new();
                    let mut prep_items: Vec<PrepItem> = Vec::new();
                    let mut direct_jobs: Vec<(u64, u32)> = Vec::new();
                    {
                        let mut scratch = scratch_arc.lock().unwrap();
                        for seg in &segs {
                            if let DrawOp::Icb {
                                params_va,
                                max_count,
                                location,
                                range_out,
                                count_va,
                                pred_va,
                                pred_op,
                                ..
                            } = &seg.op
                            {
                                if (*count_va != 0 || *pred_va != 0) && *max_count > 64 {
                                    if let Ok((ind_out, _)) = scratch.alloc(16, 16) {
                                        prep_items.push(PrepItem {
                                            count: *count_va,
                                            pred: *pred_va,
                                            range_out: *range_out,
                                            ind_out,
                                            max_count: *max_count,
                                            location: *location,
                                            pred_op: *pred_op,
                                            _pad: 0,
                                        });
                                        clamped_jobs.push((*params_va, ind_out));
                                    } else {
                                        direct_jobs.push((*params_va, *max_count));
                                    }
                                } else {
                                    direct_jobs.push((*params_va, *max_count));
                                }
                            }
                        }
                    }
                    let mut n_dispatches: u32 = 0;
                    if icb_reuse_enabled() {
                        // Ensure any prior encoder's reads/writes of the reused ICB / `icb_scratch`
                        // complete across all stages before we overwrite them.
                        ce.barrierAfterQueueStages_beforeStages_visibilityOptions(
                            MTLStages::All,
                            MTLStages::Dispatch,
                            MTL4VisibilityOptions::Device,
                        );
                    }
                    if !prep_items.is_empty() {
                        if let (Ok(pk), Ok(ck)) =
                            (indirect_icb::prep_batch_kernel(), indirect_icb::kernel_v3_clamped())
                        {
                            ce.setComputePipelineState(pk);
                            let n_chunks = prep_items.len().div_ceil(MAX_PREP_BATCH);
                            let mut stack_vas = [0u64; 8];
                            let mut heap_vas: Vec<u64> = Vec::new();
                            let chunk_vas: &[u64] = if n_chunks <= stack_vas.len() {
                                {
                                    let mut r = ring_arc.lock().unwrap();
                                    for (dst, chunk) in stack_vas
                                        .iter_mut()
                                        .zip(prep_items.chunks(MAX_PREP_BATCH))
                                    {
                                        let mut batch = PrepBatch {
                                            n_items: chunk.len() as u32,
                                            ..Default::default()
                                        };
                                        batch.items[..chunk.len()].copy_from_slice(chunk);
                                        let used =
                                            16 + chunk.len() * std::mem::size_of::<PrepItem>();
                                        let bytes = &clear::bytes_of(&batch)[..used];
                                        *dst = r.upload(bytes, 256).unwrap_or(0);
                                    }
                                }
                                &stack_vas[..n_chunks]
                            } else {
                                heap_vas.reserve(n_chunks);
                                {
                                    let mut r = ring_arc.lock().unwrap();
                                    for chunk in prep_items.chunks(MAX_PREP_BATCH) {
                                        let mut batch = PrepBatch {
                                            n_items: chunk.len() as u32,
                                            ..Default::default()
                                        };
                                        batch.items[..chunk.len()].copy_from_slice(chunk);
                                        let used =
                                            16 + chunk.len() * std::mem::size_of::<PrepItem>();
                                        let bytes = &clear::bytes_of(&batch)[..used];
                                        heap_vas.push(r.upload(bytes, 256).unwrap_or(0));
                                    }
                                }
                                &heap_vas
                            };
                            for (chunk, &b_va) in
                                prep_items.chunks(MAX_PREP_BATCH).zip(chunk_vas.iter())
                            {
                                if b_va != 0 {
                                    self.set_table_addr(b_va, 0);
                                    ce.dispatchThreadgroups_threadsPerThreadgroup(
                                        MTLSize {
                                            width: chunk.len().div_ceil(64),
                                            height: 1,
                                            depth: 1,
                                        },
                                        MTLSize { width: 64, height: 1, depth: 1 },
                                    );
                                    n_dispatches += 1;
                                }
                            }
                            let before_enc = if self.opts.opt_ib_substage_narrow
                                && !self.is_copy_queue_light()
                            {
                                stats::inc(&stats::c::ib_substage_narrow);
                                MTLStages::Dispatch
                            } else {
                                COMPUTE_STAGES
                            };
                            ce.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                                MTLStages::Dispatch,
                                before_enc,
                                MTL4VisibilityOptions::Device,
                            );
                            stats::inc(&stats::c::enc_barrier);
                            stats::inc(&stats::c::ib_masked);
                            self.ib_clear_satisfied(
                                MTLStages::Dispatch,
                                before_enc,
                                MTLStages::Dispatch,
                            );

                            ce.setComputePipelineState(ck);
                            for &(p_va, ind_out) in &clamped_jobs {
                                self.set_table_addr(p_va, 0);
                                unsafe {
                                    ce.dispatchThreadgroupsWithIndirectBuffer_threadsPerThreadgroup(
                                        ind_out,
                                        MTLSize { width: 64, height: 1, depth: 1 },
                                    );
                                }
                                n_dispatches += 1;
                            }
                        } else {
                            for (item, &(p_va, _)) in prep_items.iter().zip(&clamped_jobs) {
                                direct_jobs.push((p_va, item.max_count));
                            }
                        }
                    }
                    if !direct_jobs.is_empty() {
                        if let Ok(dk) = indirect_icb::kernel_v3() {
                            ce.setComputePipelineState(dk);
                            for &(p_va, mc) in &direct_jobs {
                                self.set_table_addr(p_va, 0);
                                ce.dispatchThreadgroups_threadsPerThreadgroup(
                                    MTLSize {
                                        width: (mc as usize).div_ceil(64),
                                        height: 1,
                                        depth: 1,
                                    },
                                    MTLSize { width: 64, height: 1, depth: 1 },
                                );
                                n_dispatches += 1;
                            }
                        }
                    }
                    self.set_table_addr(self.heaps[0], DESCRIPTOR_HEAP_BIND_POINT as usize);
                    self.compute_pso_bound = false;
                    self.pt_note_util(pt::synth::EI_EXPAND, n_dispatches);
                    self.note_stage_ran(MTLStages::Dispatch);
                    stats::add(&stats::c::icb_prepass_cs, icb_count as u64);
                    let any_ps = segs
                        .iter()
                        .any(|s| matches!(s.op, DrawOp::Icb { .. }) && s.pso.ps.is_some());
                    let before_icb = if any_ps {
                        MTLStages::Vertex | MTLStages::Fragment
                    } else {
                        MTLStages::Vertex
                    };
                    self.add_pend(MTLStages::Dispatch, before_icb);
                }
            }
            let first_meta = segs.first().and_then(|s| s.deferred_meta.as_deref());
            let mut first_eff_reads = first_meta.map_or(0, |m| m.eff_reads);
            let first_vs_uav = first_meta.is_some_and(|m| m.vs_writes_uav);
            if first_vs_uav && self.bar.list_unwaited_vtx_frag {
                first_eff_reads |= READS_TEX;
            }
            if self.need_pre_open_cur_draw_exact() {
                if let Some(m) = first_meta {
                    self.rp_sync.cur_draw_pre_unb = m.pre_unb;
                    self.rp_sync.cur_draw_pre_unb_hk = m.pre_unb_hk;
                    self.rp_sync.cur_draw_pre_exact.clear();
                    self.rp_sync.cur_draw_pre_exact.extend_from_slice(&m.pre_exact);
                    self.rp_sync.cur_draw_ps_unb = m.ps_unb;
                    self.rp_sync.cur_draw_ps_unb_hk = m.ps_unb_hk;
                    self.rp_sync.cur_draw_ps_exact.clear();
                    self.rp_sync.cur_draw_ps_exact.extend_from_slice(&m.ps_exact);
                    self.rp_sync.cur_draw_exact_valid = true;
                } else {
                    self.rp_sync.cur_draw_exact_valid = false;
                }
            } else {
                self.rp_sync.cur_draw_exact_valid = false;
            }
            self.bar.opening_render_split_ok = !first_vs_uav
                && (!self.has_frag_buf_write()
                    || ((self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
                        && self.occlusion.is_none()));
            self.bar.opening_pre_raster_stages = MTLStages::Vertex;
            // Open the deferred batch's render pass using its captured segment PSO's RTV
            // mask rather than `self.gfx.pso`, which may already hold a narrower next PSO.
            let batch_rtv_fmts = segs.first().map(|s| s.pso.rtv_formats.as_slice());
            let enc_res =
                unsafe { self.draw_encoder_for_rtv_formats(first_eff_reads, batch_rtv_fmts) };
            self.bar.opening_pre_raster_stages = PRE_RASTER;
            self.bar.opening_render_split_ok = false;
            self.icb.icb_prepass_flushing = false;
            self.rp_sync.cur_draw_exact_valid = false;
            if enc_res.is_err() {
                return;
            }
            for seg in &segs {
                if let Some(meta) = seg.deferred_meta.as_deref() {
                    let mut eff_reads = meta.eff_reads;
                    if meta.vs_writes_uav && self.bar.list_unwaited_vtx_frag {
                        eff_reads |= READS_TEX;
                    }
                    self.rp_sync.enc_pre_raster_reads |= eff_reads;
                    self.rp_sync.enc_pre_raster_srv_uav_reads |= meta.srv_uav_pre_reads;
                    self.rp_sync.enc_ps_reads |= meta.ps_reads;
                    self.rp_sync.enc_ps_srv_uav_reads |= meta.ps_srv_uav_reads;
                    if self.hk_any_active() {
                        self.rp_sync.enc_pre_raster_hk |= meta.pre_hk;
                        self.rp_sync.enc_ps_hk |= meta.ps_hk;
                        if !self.opts.opt_gfx_desc_table_exact {
                            self.rp_sync.enc_pre_raster_unb_hk |= meta.pre_unb_hk | meta.pre_hk;
                            self.rp_sync.enc_ps_unb_hk |= meta.ps_unb_hk | meta.ps_hk;
                        }
                    }
                    self.rp_sync.enc_uav_used |= meta.writes_uav || meta.uses_uav;
                    if self.opts.opt_gfx_desc_table_exact {
                        self.rp_sync.enc_pre_raster_unbounded_classes |= meta.pre_unb;
                        self.rp_sync.enc_pre_raster_unb_hk |= meta.pre_unb_hk;
                        mark_and_extend_unique(
                            &meta.pre_exact,
                            &mut self.rp_sync.enc_pre_raster_exact_res,
                        );
                        self.rp_sync.enc_ps_unbounded_classes |= meta.ps_unb;
                        self.rp_sync.enc_ps_unb_hk |= meta.ps_unb_hk;
                        mark_and_extend_unique(&meta.ps_exact, &mut self.rp_sync.enc_ps_exact_res);
                    }
                    if meta.writes_uav && self.opts.opt_rp_split_exact_keep {
                        if meta.uav_unb {
                            self.rp_sync.enc_uav_write_unbounded = true;
                        } else {
                            for &r in &meta.uav_exact {
                                if !self.rp_sync.enc_uav_write_exact_res.contains(&r) {
                                    self.rp_sync.enc_uav_write_exact_res.push(r);
                                }
                            }
                        }
                    }
                    self.record_gfx_draw_uav_writes(
                        meta.writes_uav,
                        meta.vs_writes_uav,
                        meta.ps_writes_uav,
                        MTLStages::Vertex,
                    );
                    if (self.opts.opt_cs_to_ps_overlap
                        || ((self.opts.opt_cs_to_ps_overlap_v2
                            || self.gfx_to_ps_overlap_active()
                            || self.list_rsplit_v2_active())
                            && self.has_any_soft_pre_obl()))
                        && self.opts.opt_gfx_desc_table_exact
                    {
                        self.rp_sync.cur_draw_pre_unb = meta.pre_unb;
                        self.rp_sync.cur_draw_pre_exact.clear();
                        self.rp_sync.cur_draw_pre_exact.extend_from_slice(&meta.pre_exact);
                        self.rp_sync.cur_draw_ps_unb = meta.ps_unb;
                        self.rp_sync.cur_draw_ps_exact.clear();
                        self.rp_sync.cur_draw_ps_exact.extend_from_slice(&meta.ps_exact);
                        self.rp_sync.cur_draw_exact_valid = true;
                    }
                    self.collect_batch_qbarriers(eff_reads, MTLStages::Vertex);
                    self.rp_sync.cur_draw_exact_valid = false;
                }
                if let Some(m) = self.pt_cur() {
                    m.draws += 1;
                    if matches!(seg.op, DrawOp::Icb { .. }) {
                        m.icb += 1;
                    }
                    m.shader = seg.pso.vs.key as u64
                        ^ seg.pso.ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1));
                }
            }
            qbs.extend(std::mem::take(&mut self.icb.icb_qbarriers));
        }

        let Encoder::Render(e) = &self.enc else {
            return;
        };
        let e = e.clone();
        let size = self.gfx.pass_size;
        let fast = self.opts.opt_fast_enc_state;
        for (after, before) in qbs {
            e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                after,
                before,
                MTL4VisibilityOptions::Device,
            );
            self.record_enc_queue_sync(after, before);
        }
        let mut applied_dyn: Option<u32> = None;
        if icb_count > 0 && !was_prepass {
            if self.occlusion.is_some() {
                unsafe { e.setVisibilityResultMode_offset(MTLVisibilityResultMode::Disabled, 0) };
            }
            if let Some(d0) = dyns.first() {
                if fast {
                    self.rp_sync.enc_render_cache.apply_shared(
                        &e,
                        size,
                        &d0.viewports,
                        &d0.scissors,
                        d0.blend_factor,
                        d0.stencil_ref,
                        d0.stencil_back_ref,
                        true,
                    );
                } else {
                    Self::apply_shared_dynamic_snap(&e, d0, size);
                }
                applied_dyn = Some(0);
            }
            if icb_reuse_enabled() && !self.icb.enc_icb_war_done {
                // Wait for any previous encoder's reads/writes of reused ICB / `icb_scratch`
                // slots before this render pass's Vertex expander writes them.
                // (Never emit an intra-encoder Fragment -> Vertex barrier on Apple TBDR.)
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Vertex,
                    MTL4VisibilityOptions::Device,
                );
                self.icb.enc_icb_war_done = true;
            }
            let mut last_ex_ptr: usize = 0;
            for seg in &segs {
                if let DrawOp::Icb { ex: Some(ex), params_va, max_count, .. } = &seg.op {
                    if fast {
                        self.rp_sync.enc_render_cache.apply_pso(&e, ex, true);
                    } else {
                        let ex_ptr = &**ex as *const _ as usize;
                        if ex_ptr != last_ex_ptr {
                            e.setRenderPipelineState(ex);
                            last_ex_ptr = ex_ptr;
                        }
                    }
                    self.set_table_addr(*params_va, 0);
                    unsafe {
                        e.drawPrimitives_vertexStart_vertexCount(
                            MTLPrimitiveType::Point,
                            0,
                            *max_count as usize,
                        );
                    }
                }
            }
            self.note_stage_ran(MTLStages::Vertex);
            self.set_table_addr(self.heaps[0], DESCRIPTOR_HEAP_BIND_POINT as usize);
            let before_icb = if self.opts.opt_icb_fast_v3 || self.opts.opt_icb_enc_barrier_exact {
                if !self.opts.opt_icb_fast_v3 && self.opts.opt_icb_enc_barrier_exact {
                    stats::inc(&stats::c::icb_enc_barrier_exact);
                }
                let any_ps = segs
                    .iter()
                    .any(|s| matches!(s.op, DrawOp::Icb { .. }) && s.pso.ps.is_some());
                if any_ps {
                    MTLStages::Vertex | MTLStages::Fragment
                } else {
                    MTLStages::Vertex
                }
            } else {
                MTLStages::Vertex | MTLStages::Object | MTLStages::Mesh | MTLStages::Fragment
            };
            e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                MTLStages::Vertex,
                before_icb,
                MTL4VisibilityOptions::Device,
            );
            if icb_count > 1 {
                stats::add(&stats::c::icb_hoist_saved, (icb_count - 1) as u64);
            }
            if let Some(o) = &self.occlusion {
                let mode = if o.binary {
                    MTLVisibilityResultMode::Boolean
                } else {
                    MTLVisibilityResultMode::Counting
                };
                unsafe { e.setVisibilityResultMode_offset(mode, o.index as usize * 8) };
            }
        }
        let mut applied_pso_arc: usize = 0;
        let mut applied_mtl_pso: usize = 0;
        let mut table_clobbered = false;
        let mut batch_ran = MTLStages::empty();
        let has_targets =
            !self.bar.pass_targets.is_empty() || self.active_dsv().is_some_and(|v| v.resource != 0);
        for seg in &segs {
            batch_ran |= MTLStages::Vertex;
            if has_targets || seg.pso.ps.is_some() {
                batch_ran |= MTLStages::Fragment;
            }
            let mtl_pso: &MtlRenderPipeline = match &seg.op {
                DrawOp::Icb { .. } => match seg.pso.icb_pso() {
                    Ok(p) => p,
                    Err(err) => {
                        log::warn!(target: "d3d", "flush_draw_batch: icb_pso failed ({err}); using primary pso");
                        &seg.pso.pso
                    }
                },
                DrawOp::Draw { .. } | DrawOp::DrawIndexed { .. } => &seg.pso.pso,
            };
            if fast {
                self.rp_sync.enc_render_cache.apply_pso(&e, mtl_pso, true);
                self.rp_sync.enc_render_cache.apply_raster(&e, &seg.pso.raster, true);
                self.rp_sync.enc_render_cache.apply_ds(&e, &seg.pso.depth_stencil, true);
            } else {
                let mtl_ptr = mtl_pso as *const _ as usize;
                if mtl_ptr != applied_mtl_pso {
                    e.setRenderPipelineState(mtl_pso);
                    applied_mtl_pso = mtl_ptr;
                }
                let arc_ptr = Arc::as_ptr(&seg.pso) as usize;
                if arc_ptr != applied_pso_arc {
                    seg.pso.raster.apply(&e);
                    e.setDepthStencilState(Some(&seg.pso.depth_stencil));
                    applied_pso_arc = arc_ptr;
                }
            }
            if applied_dyn != Some(seg.dyn_idx) {
                if let Some(d) = dyns.get(seg.dyn_idx as usize) {
                    if fast {
                        self.rp_sync.enc_render_cache.apply_shared(
                            &e,
                            size,
                            &d.viewports,
                            &d.scissors,
                            d.blend_factor,
                            d.stencil_ref,
                            d.stencil_back_ref,
                            true,
                        );
                    } else {
                        Self::apply_shared_dynamic_snap(&e, d, size);
                    }
                }
                applied_dyn = Some(seg.dyn_idx);
            }
            match &seg.op {
                DrawOp::Icb { icb, range_out, .. } => {
                    unsafe { e.executeCommandsInBuffer_indirectBuffer(icb, *range_out) };
                    table_clobbered = true;
                    self.rp_sync.enc_render_cache.mesh_stages_bound = false;
                }
                DrawOp::Draw {
                    bind,
                    prim,
                    first_vertex,
                    vertices,
                    instances,
                    first_instance,
                } => {
                    if table_clobbered {
                        e.setArgumentTable_atStages(
                            &self.table,
                            MTLRenderStages::Vertex | MTLRenderStages::Fragment,
                        );
                        self.invalidate_table_cache();
                        table_clobbered = false;
                    }
                    self.apply_direct_bind(bind);
                    unsafe {
                        e.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                            *prim,
                            *first_vertex,
                            *vertices,
                            *instances,
                            *first_instance,
                        );
                    }
                }
                DrawOp::DrawIndexed {
                    bind,
                    prim,
                    indices,
                    index_type,
                    ib_va,
                    ib_len,
                    instances,
                    base_vertex,
                    first_instance,
                } => {
                    if table_clobbered {
                        e.setArgumentTable_atStages(
                            &self.table,
                            MTLRenderStages::Vertex | MTLRenderStages::Fragment,
                        );
                        self.invalidate_table_cache();
                        table_clobbered = false;
                    }
                    self.apply_direct_bind(bind);
                    unsafe {
                        e.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_baseVertex_baseInstance(
                            *prim,
                            *indices,
                            *index_type,
                            *ib_va,
                            *ib_len,
                            *instances,
                            *base_vertex,
                            *first_instance,
                        );
                    }
                }
            }
        }
        if !batch_ran.is_empty() {
            self.note_stage_ran(batch_ran);
        }
        if icb_count > 0 {
            self.icb.enc_used_icb = true;
        }
        if keep_open {
            if table_clobbered {
                e.setArgumentTable_atStages(
                    &self.table,
                    MTLRenderStages::Vertex | MTLRenderStages::Fragment,
                );
                self.invalidate_table_cache();
            }
            if !fast {
                self.rp_sync.enc_render_cache.reset();
            }
            self.gfx.dirty = GFX_DIRTY_ALL;
        }
    }

    /// `ExecuteIndirect(signature, max_count, args, args_cpu, count)`. `args` and
    /// `count` are GPU addresses (resource VA + offset); `args_cpu` is the host
    /// slice of the argument buffer when it is CPU-readable (e.g. UPLOAD heap or
    /// mirrored from an upload `CopyBufferRegion`). See [`crate::metal::indirect`] for
    /// the expansion scheme.
    pub fn execute_indirect(
        &mut self,
        l: &IndirectLayout,
        max_count: u32,
        args: u64,
        args_cpu: Option<&[u8]>,
        count: Option<u64>,
    ) -> Result<(), String> {
        if !matches!(l.work, Work::Draw | Work::DrawIndexed | Work::DispatchMesh) {
            self.disc_unmark(0);
        }
        if max_count == 0 {
            return Ok(());
        }
        let compute = l.work == Work::Dispatch;
        stats::inc(&stats::c::ei_calls);
        stats::add(&stats::c::ei_max_count, max_count as u64);
        // Validate the work state before encoding anything.
        let index = if l.work == Work::DrawIndexed && l.ib_offset.is_none() {
            let (addr, size, fmt) =
                self.gfx.ib.ok_or("indirect indexed draw without an index buffer")?;
            Some((
                addr,
                size,
                crate::topology::index_format(fmt)
                    .ok_or_else(|| format!("index format {fmt}"))?,
            ))
        } else {
            None
        };
        if compute && self.compute_pso.is_none() {
            return Err("ExecuteIndirect dispatch without a compute PSO".into());
        }
        let mesh = l.work == Work::DispatchMesh;
        if mesh && self.gfx.mesh.is_none() {
            return Err("ExecuteIndirect DispatchMesh without a mesh PSO".into());
        }
        if matches!(l.work, Work::Draw | Work::DrawIndexed)
            && self.gfx.pso.as_ref().is_some_and(|p| p.tess_emu.is_some())
        {
            return self.execute_indirect_tessellation_emulated(l, max_count, args, args_cpu, count);
        }
        let ws = l.work.size();
        let pred = self.predication;
        if mesh
            && (l.needs_expansion(count.is_some()) || pred.is_some() || max_count > 1)
            && indirect_icb::enabled()
        {
            return self.execute_indirect_mesh_icb(l, max_count, args, count);
        }
        if !compute
            && !mesh
            && (l.needs_expansion(count.is_some()) || pred.is_some())
            && l.vb_offsets.len() <= indirect_icb::MAX_VBO
            && indirect_icb::enabled()
        {
            return self.execute_indirect_icb(l, max_count, args, count);
        }
        // Inspect CPU-visible indirect commands (either CPU-uploaded or GPU-written in a prior frame)
        // when the signature carries an embedded D3D12_INDEX_BUFFER_VIEW.
        let mut cpu_valid_ibs = 0u32;
        if let (Some(ib_off), Some(cb)) = (l.ib_offset, args_cpu) {
            let stride = l.stride as usize;
            let off = ib_off as usize;
            for i in 0..max_count as usize {
                let base = i.saturating_mul(stride);
                let Some(ib) = cb.get(base + off..base + off + 16) else { break };
                let iaddr = u64::from_le_bytes(ib[0..8].try_into().unwrap());
                let isize = u32::from_le_bytes(ib[8..12].try_into().unwrap());
                let ifmt = u32::from_le_bytes(ib[12..16].try_into().unwrap());
                if iaddr != 0 && isize != 0 && (ifmt == 42 || ifmt == 57) {
                    cpu_valid_ibs += 1;
                }
            }
        }
        // Use GPU-side D3D12_INDEX_BUFFER_VIEW translation in `aw2_execute_indirect` only when
        // no CPU-visible IB views are present yet (`cpu_valid_ibs == 0`).
        let gpu_ib = if let Some(ib_off) = l.ib_offset {
            if cpu_valid_ibs == 0 {
                resource::buffer_va_range().map(|(lo, hi)| {
                    (lo, hi.saturating_sub(lo).clamp(4, 0x7FFF_FFF0) as usize, ib_off)
                })
            } else {
                None
            }
        } else {
            None
        };
        let expanded = l.needs_expansion(count.is_some()) || pred.is_some();
        let ei_cs_exact = compute && self.opts.opt_ei_dispatch_exact && self.ib_tight_active();
        let (work_addr, work_stride, tlabs) = if expanded {
            let (tlab_size, patches) = if l.patches.is_empty() {
                (0, Vec::new())
            } else {
                let root = if compute { &self.compute_root } else { &self.gfx_root };
                let rs = root.rs.as_ref().ok_or(
                    "ExecuteIndirect with root argument changes but no root signature",
                )?;
                (rs.layout().size, l.kernel_patches(rs.layout())?)
            };
            let ts = indirect::tlab_stride(tlab_size);
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let out_slots = if gpu_ib.is_some() { 2usize } else { 1usize };
            let (out, tout, params) = {
                let mut r = ring.lock().unwrap();
                let root_state = if compute { &mut self.compute_root } else { &mut self.gfx_root };
                let template = if root_state.rs.is_some() {
                    if root_state.is_dirty() {
                        root_state.flush(&mut r)?
                    } else {
                        root_state.bound
                    }
                } else {
                    0
                };
                let (out, _) = r.alloc(ws as usize * max_count as usize * out_slots, 16)?;
                let tout = if patches.is_empty() {
                    0
                } else {
                    r.alloc(ts as usize * max_count as usize, 256)?.0
                };
                let (zero_word, zero_n) = l.work.zero();
                let mut p = KernelParams {
                    args,
                    count: count.unwrap_or(0),
                    args_out: out,
                    tlab_out: tout,
                    tlab_in: template,
                    base_ib_va: gpu_ib.map_or(0, |(lo, _, _)| lo),
                    ib_offset: gpu_ib.map_or(0, |(_, _, off)| off),
                    stride: l.stride,
                    max_count,
                    work_offset: l.work_offset,
                    work_size: ws,
                    out_stride: ws,
                    zero_word,
                    zero_n,
                    tlab_size,
                    tlab_stride: ts,
                    npatch: patches.len() as u32,
                    pred: pred.map_or(0, |pr| pr.gpu_va),
                    pred_op: pred.map_or(0, |pr| pr.op),
                    ..Default::default()
                };
                p.patches[..patches.len()].copy_from_slice(&patches);
                (out, tout, r.upload(p.bytes(), 256)?)
            };
            let k = indirect::kernel()?;
            let e = if ei_cs_exact {
                let want_spans = self.opts.opt_subrange_uav_barrier;
                let mut exp_reads: Vec<usize> = Vec::with_capacity(3);
                let mut exp_spans: Vec<SubRange> = Vec::new();
                let arg_bytes = (max_count as u64).saturating_mul(l.stride as u64);
                let va_cache = &mut *self.binds.buf_va_cache;
                let exp_ok = resource::with_resource_registry(|reg| {
                    let mut ok = if args != 0 {
                        if want_spans {
                            reg.push_buffer_span_cached(
                                va_cache,
                                args,
                                arg_bytes,
                                &mut exp_reads,
                                &mut exp_spans,
                            )
                        } else {
                            reg.push_buffer_va_cached(va_cache, args, &mut exp_reads)
                        }
                    } else {
                        true
                    };
                    if ok {
                        if let Some(c_va) = count.filter(|&v| v != 0) {
                            ok = if want_spans {
                                reg.push_buffer_span_cached(
                                    va_cache,
                                    c_va,
                                    4,
                                    &mut exp_reads,
                                    &mut exp_spans,
                                )
                            } else {
                                reg.push_buffer_va_cached(va_cache, c_va, &mut exp_reads)
                            };
                        }
                    }
                    if ok {
                        if let Some(p_va) = pred.map(|pr| pr.gpu_va).filter(|&v| v != 0) {
                            ok = if want_spans {
                                reg.push_buffer_span_cached(
                                    va_cache,
                                    p_va,
                                    8,
                                    &mut exp_reads,
                                    &mut exp_spans,
                                )
                            } else {
                                reg.push_buffer_va_cached(va_cache, p_va, &mut exp_reads)
                            };
                        }
                    }
                    ok
                });
                let ei_hk = HandleKindProfile {
                    uav_write_kinds: 0,
                    uav_srv_kinds: HK_RAW_STRUCT_BUF,
                    ro_srv_kinds: HK_RAW_STRUCT_BUF,
                    read_kinds: HK_RAW_STRUCT_BUF | HK_CBV_BUF,
                    unbounded_srv_kinds: 0,
                };
                if exp_ok {
                    stats::inc(&stats::c::ei_cs_exact);
                    let exact = (self.opts.opt_ib_exact_obl
                        || self.opts.opt_subrange_uav_barrier
                        || self.opts.opt_cs_mid_qb_touch_defer
                        || self.opts.opt_cs_mid_qb_exact_v2
                        || self.opts.opt_cs_mid_qb_defer_v3
                        || self.ps_to_cs_active()
                        || self.opts.opt_cs_intra_barrier_v2
                        || self.cs_intra_safe_active()
                        || self.hk_any_active()
                        || self.msc_class_active())
                        .then_some(DispatchExact {
                            uav_exact: true,
                            full_read_exact: true,
                            bounded_read_exact: true,
                            touches_textures: false,
                            unbounded_srv_classes: 0,
                            uav_classes: 0,
                            read_classes: READS_BUF,
                            exact_read_only_classes: READS_BUF,
                            uav_srv_classes: 0,
                            hk: ei_hk,
                            uavs: &[],
                            bounded_reads: exp_reads.as_slice(),
                            uav_spans: want_spans.then_some(&[]),
                            bounded_read_spans: want_spans.then_some(exp_spans.as_slice()),
                        });
                    self.compute_op_tight_exact(
                        MTLStages::Dispatch,
                        Some(&[]),
                        Some(exp_reads.as_slice()),
                        Some(exp_reads.as_slice()),
                        exact,
                    )
                    .ok_or("compute encoder")?
                } else {
                    let exact = (self.opts.opt_cs_mid_qb_touch_defer
                        || self.opts.opt_cs_mid_qb_exact_v2
                        || self.opts.opt_cs_mid_qb_defer_v3
                        || self.ps_to_cs_active()
                        || self.opts.opt_cs_intra_barrier_v2
                        || self.cs_intra_safe_active()
                        || self.hk_any_active()
                        || self.msc_class_active())
                        .then_some(DispatchExact {
                            uav_exact: true,
                            full_read_exact: false,
                            bounded_read_exact: false,
                            touches_textures: false,
                            unbounded_srv_classes: 0,
                            uav_classes: 0,
                            read_classes: READS_BUF,
                            exact_read_only_classes: READS_BUF,
                            uav_srv_classes: 0,
                            hk: ei_hk,
                            uavs: &[],
                            bounded_reads: &[],
                            uav_spans: None,
                            bounded_read_spans: None,
                        });
                    self.compute_op_tight_exact(MTLStages::Dispatch, Some(&[]), None, None, exact)
                        .ok_or("compute encoder")?
                }
            } else {
                self.compute().ok_or("compute encoder")?
            };
            self.pt_note_util(pt::synth::EI_EXPAND, 1);
            e.setComputePipelineState(k);
            self.compute_pso_bound = false;
            // Index 0 is the descriptor heap slot; bind_table restores it.
            self.set_table_addr(params, 0);
            e.dispatchThreadgroups_threadsPerThreadgroup(
                MTLSize { width: (max_count as usize).div_ceil(64), height: 1, depth: 1 },
                MTLSize { width: 64, height: 1, depth: 1 },
            );
            self.note_stage_ran(MTLStages::Dispatch);
            // The expanded arguments are read by the following work.
            stats::inc(&stats::c::ei_expand);
            if ei_cs_exact {
                // `out` / `tout` are transient ring buffers read ONLY by the indirect
                // compute dispatches right below in this same encoder — emit an exact
                // intra-encoder `Dispatch -> Dispatch` (when `AW2_IB_SUBSTAGE_NARROW=1`)
                // or `Dispatch -> COMPUTE_STAGES` barrier without poisoning cross-encoder
                // `split_pend` (`vtx_after` / `frag_after` / `disp_after`) or wildcarding
                // `ib_obl_tight(0)`.
                let before_enc =
                    if self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light() {
                        stats::inc(&stats::c::ib_substage_narrow);
                        MTLStages::Dispatch
                    } else {
                        COMPUTE_STAGES
                    };
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    MTLStages::Dispatch,
                    before_enc,
                    MTL4VisibilityOptions::Device,
                );
                stats::inc(&stats::c::enc_barrier);
                stats::inc(&stats::c::ib_masked);
                self.ib_clear_satisfied(MTLStages::Dispatch, before_enc, MTLStages::Dispatch);
            } else {
                self.produced(
                    MTLStages::Dispatch,
                    MTLStages::Vertex
                        | MTLStages::Object
                        | MTLStages::Mesh
                        | MTLStages::Fragment
                        | MTLStages::Dispatch,
                );
            }
            (out, ws as u64, (tout != 0).then_some((tout, ts as u64)))
        } else {
            (args + l.work_offset as u64, l.stride as u64, None)
        };
        let bind_root = |table: &MtlArgumentTable, i: u64| {
            if let Some((t, ts)) = tlabs {
                unsafe {
                    table.setAddress_atIndex(t + i * ts, ARGUMENT_BUFFER_BIND_POINT as usize)
                };
            }
        };
        if compute {
            let pso = self
                .compute_pso
                .clone()
                .ok_or("ExecuteIndirect dispatch without a compute PSO")?;
            let root = self.flush_root(true)?;
            let e = if ei_cs_exact {
                let mut scratch_uav = std::mem::take(&mut self.ib.ib_scratch_uav);
                let mut scratch_read = std::mem::take(&mut self.ib.ib_scratch_read);
                let mut scratch_op = std::mem::take(&mut self.ib.ib_scratch_op);
                let mut scratch_uav_spans = std::mem::take(&mut self.ib.ib_scratch_uav_spans);
                let mut scratch_read_spans = std::mem::take(&mut self.ib.ib_scratch_read_spans);
                scratch_uav.clear();
                scratch_read.clear();
                scratch_op.clear();
                scratch_uav_spans.clear();
                scratch_read_spans.clear();
                let want_spans = self.opts.opt_subrange_uav_barrier;
                let non_const_patch = l.patches.iter().any(|pt| {
                    self.compute_root.rs.as_ref().is_none_or(|rs| {
                        rs.layout().parameters.get(pt.param as usize).is_none_or(|pl| {
                            !matches!(
                                pl.kind,
                                dxil_metal::root_signature::RootParameterLayoutKind::Constants { .. }
                            )
                        })
                    })
                });
                let want_bounded_ind = (self.opts.opt_ib_bindless_war
                    || self.opts.opt_ib_exact_obl
                    || self.opts.opt_subrange_uav_barrier
                    || self.opts.opt_desc_table_res_exact
                    || self.opts.opt_desc_table_cs_fast
                    || self.opts.opt_cs_intra_barrier_exact
                    || self.ps_to_cs_active()
                    || self.opts.opt_cs_intra_barrier_v2
                    || self.opts.opt_cs_fast_bind_exact)
                    && !pso.access.heap_indexed_or_unknown;
                let va_cache = &mut *self.binds.buf_va_cache;
                let compute_root = &self.compute_root;
                let heap0 = self.heap0_cpu;
                let (uav_exact, full_read_exact, bounded_read_exact) = if !non_const_patch {
                    resource::with_resource_registry(|reg| {
                        let ue = pso.access.uav_slots.as_deref().is_some_and(|s| {
                            Self::resolve_slots_with_reg(
                                reg,
                                va_cache,
                                s,
                                compute_root,
                                heap0,
                                &mut scratch_uav,
                                want_spans.then_some(&mut scratch_uav_spans),
                            )
                        });
                        let fre = pso.access.read_slots.as_deref().is_some_and(|s| {
                            Self::resolve_slots_with_reg(
                                reg,
                                va_cache,
                                s,
                                compute_root,
                                heap0,
                                &mut scratch_read,
                                want_spans.then_some(&mut scratch_read_spans),
                            )
                        });
                        let bre = if fre {
                            true
                        } else if want_bounded_ind {
                            Self::resolve_slots_with_reg(
                                reg,
                                va_cache,
                                &pso.access.bounded_read_slots,
                                compute_root,
                                heap0,
                                &mut scratch_read,
                                want_spans.then_some(&mut scratch_read_spans),
                            )
                        } else {
                            false
                        };
                        let (mut fre_out, mut bre_out) = (fre, bre);
                        if !expanded && (fre_out || bre_out) && work_addr != 0 {
                            let work_bytes = (max_count as u64).saturating_mul(work_stride);
                            let arg_ok = if want_spans {
                                reg.push_buffer_span_cached(
                                    va_cache,
                                    work_addr,
                                    work_bytes,
                                    &mut scratch_read,
                                    &mut scratch_read_spans,
                                )
                            } else {
                                reg.push_buffer_va_cached(va_cache, work_addr, &mut scratch_read)
                            };
                            if !arg_ok {
                                fre_out = false;
                                bre_out = false;
                            }
                        }
                        (ue, fre_out, bre_out)
                    })
                } else {
                    (false, false, false)
                };
                let msc_class = self.msc_class_active();
                if msc_class && pso.access.msc_refined {
                    stats::inc(&stats::c::msc_access_exact);
                }
                if !self.bar.clear_tex_cs.is_empty() {
                    if (self.opts.opt_stagesync_barrier_v2 || msc_class)
                        && !pso.access.touches_textures
                    {
                        stats::inc(&stats::c::clear_tex_cs_keep);
                    } else {
                        let exact_tex = full_read_exact
                            || (bounded_read_exact
                                && !pso.access.heap_indexed_or_unknown
                                && (pso.access.unbounded_srv_classes & READS_TEX) == 0);
                        if !exact_tex || !uav_exact {
                            self.flush_clear_tex_cs_all();
                        } else {
                            let mut hit = false;
                            self.bar.clear_tex_cs.retain(|r| {
                                if scratch_read.contains(r) || scratch_uav.contains(r) {
                                    hit = true;
                                    false
                                } else {
                                    true
                                }
                            });
                            if hit {
                                self.add_pend(MTLStages::Fragment, MTLStages::Dispatch);
                            }
                        }
                    }
                }
                let war_read_exact =
                    full_read_exact || (self.opts.opt_ib_bindless_war && bounded_read_exact);
                let op_exact = uav_exact && full_read_exact;
                if op_exact {
                    scratch_op.extend_from_slice(&scratch_uav);
                    for &r in &scratch_read {
                        if !scratch_op.contains(&r) {
                            scratch_op.push(r);
                        }
                    }
                }
                if uav_exact || war_read_exact || !pso.access.touches_textures {
                    stats::inc(&stats::c::ei_cs_exact);
                }
                let use_exact_cs_ro =
                    self.opts.opt_cs_mid_qb_exact_v2 || self.opts.opt_sm66_cs_stage_exact;
                if self.opts.opt_sm66_cs_stage_exact
                    && (pso.access.msc_refined
                        || pso.access.msc_exact_classes != pso.access.read_classes)
                {
                    stats::inc(&stats::c::sm66_cs_exact);
                }
                let ro_rc = if use_exact_cs_ro {
                    pso.access.msc_exact_classes
                } else {
                    pso.access.read_classes
                };
                let exact_read_only_classes = if !expanded { ro_rc | READS_BUF } else { ro_rc };
                let read_classes = if use_exact_cs_ro {
                    pso.access.uav_classes | exact_read_only_classes
                } else if !expanded {
                    pso.access.read_classes | READS_BUF
                } else {
                    pso.access.read_classes
                };
                let uav_srv_classes = if (self.opts.opt_ib_uav0_unbounded_exact
                    || self.opts.opt_cs_intra_barrier_exact
                    || self.opts.opt_cs_intra_barrier_v2
                    || self.opts.opt_cs_fast_bind_exact)
                    && use_exact_cs_ro
                {
                    let n = pso.access.uav_classes
                        | (pso.access.uav_srv_classes & (pso.access.uav_classes | ro_rc));
                    if n != pso.access.uav_srv_classes {
                        if self.opts.opt_cs_fast_bind_exact {
                            stats::inc(&stats::c::cs_fast_bind_exact);
                        } else if self.opts.opt_ib_uav0_unbounded_exact {
                            stats::inc(&stats::c::ib_uav0_exact);
                        } else if self.opts.opt_cs_intra_barrier_exact {
                            stats::inc(&stats::c::cs_intra_barrier_exact);
                        } else {
                            stats::inc(&stats::c::cs_intra_barrier_v2);
                        }
                    }
                    n
                } else {
                    pso.access.uav_srv_classes
                };
                let mut hk = if self.opts.opt_sm66_handle_kind_exact {
                    resolve_pso_hk_profile(pso.hk_profile, &pso.shader, &pso.access)
                } else {
                    HandleKindProfile::OPAQUE
                };
                if !expanded {
                    hk.ro_srv_kinds |= HK_RAW_STRUCT_BUF | HK_CBV_BUF;
                    hk.read_kinds |= HK_RAW_STRUCT_BUF | HK_CBV_BUF;
                }
                let exact = (self.opts.opt_ib_exact_obl
                    || self.opts.opt_subrange_uav_barrier
                    || self.opts.opt_desc_table_res_exact
                    || self.opts.opt_desc_table_cs_fast
                    || self.opts.opt_cs_intra_barrier_exact
                    || self.opts.opt_cs_mid_qb_touch_defer
                    || self.opts.opt_cs_mid_qb_exact_v2
                    || self.opts.opt_cs_mid_qb_defer_v3
                    || self.opts.opt_cs_mid_qb_unpoison_v4
                    || self.opts.opt_cs_mid_qb_res_defer_v5
                    || self.ps_to_cs_active()
                    || self.opts.opt_cs_intra_barrier_v2
                    || self.opts.opt_cs_fast_bind_exact
                    || self.opts.opt_sm66_handle_kind_exact
                    || msc_class)
                    .then_some(DispatchExact {
                        uav_exact,
                        full_read_exact,
                        bounded_read_exact,
                        touches_textures: pso.access.touches_textures,
                        unbounded_srv_classes: if use_exact_cs_ro {
                            pso.access.unbounded_srv_classes & ro_rc
                        } else {
                            pso.access.unbounded_srv_classes
                        },
                        uav_classes: pso.access.uav_classes,
                        read_classes,
                        exact_read_only_classes,
                        uav_srv_classes,
                        hk,
                        uavs: scratch_uav.as_slice(),
                        bounded_reads: scratch_read.as_slice(),
                        uav_spans: (want_spans && uav_exact)
                            .then_some(scratch_uav_spans.as_slice()),
                        bounded_read_spans: (want_spans && bounded_read_exact)
                            .then_some(scratch_read_spans.as_slice()),
                    });
                let enc = self.compute_op_tight_exact(
                    MTLStages::Dispatch,
                    uav_exact.then_some(scratch_uav.as_slice()),
                    war_read_exact.then_some(scratch_read.as_slice()),
                    op_exact.then_some(scratch_op.as_slice()),
                    exact,
                );
                self.ib.ib_scratch_uav = scratch_uav;
                self.ib.ib_scratch_read = scratch_read;
                self.ib.ib_scratch_op = scratch_op;
                self.ib.ib_scratch_uav_spans = scratch_uav_spans;
                self.ib.ib_scratch_read_spans = scratch_read_spans;
                enc.ok_or("compute encoder")?
            } else {
                self.compute().ok_or("compute encoder")?
            };
            if let Some(m) = self.pt_cur() {
                m.dispatches += max_count;
                m.shader = pso.shader.key as u64;
            }
            e.setComputePipelineState(&pso.pso);
            self.compute_pso_bound = true;
            self.bind_table(root);
            let tg = MTLSize {
                width: pso.threadgroup[0] as usize,
                height: pso.threadgroup[1] as usize,
                depth: pso.threadgroup[2] as usize,
            };
            for i in 0..max_count as u64 {
                bind_root(&self.table, i);
                unsafe {
                    e.dispatchThreadgroupsWithIndirectBuffer_threadsPerThreadgroup(
                        work_addr + i * work_stride,
                        tg,
                    )
                };
            }
            self.note_stage_ran(MTLStages::Dispatch);
            if tlabs.is_some() {
                self.invalidate_table_cache();
            }
            return Ok(());
        }
        if mesh {
            // D3D12_DISPATCH_MESH_ARGUMENTS are read in place by Metal.
            let (e, p) = self.begin_mesh(!expanded)?;
            for i in 0..max_count as u64 {
                bind_root(&self.table, i);
                crate::mesh::encode_dispatch_indirect(&e, &p, work_addr + i * work_stride)?;
            }
            if tlabs.is_some() {
                self.invalidate_table_cache();
            }
            return Ok(());
        }
        let f16 = crate::topology::index_format(57).unwrap();
        let f32 = crate::topology::index_format(42).unwrap();
        let default_ir = index
            .map(|(_, _, f)| f.ir)
            .or_else(|| gpu_ib.map(|_| f16.ir))
            .or_else(|| {
                (l.work == Work::DrawIndexed)
                    .then(|| {
                        self.gfx
                            .ib
                            .and_then(|(_, _, fmt)| crate::topology::index_format(fmt).map(|f| f.ir))
                    })
                    .flatten()
            });
        let (e, p, prim) = self.begin_draw_ext(default_ir, &[0u8; 20], true)?;
        let ia16 = binding::static_index_type_va(f16.ir).unwrap_or(0);
        let ia32 = binding::static_index_type_va(f32.ir).unwrap_or(0);
        stats::add(&stats::c::ei_draws, max_count as u64 * if gpu_ib.is_some() { 2 } else { 1 });
        for i in 0..max_count as u64 {
            let cmd_bytes = args_cpu.and_then(|b| b.get((i as usize) * (l.stride as usize)..));
            let cmd_ib = if gpu_ib.is_some() {
                None
            } else if let Some(ib_off) = l.ib_offset {
                let from_cmd = cmd_bytes
                    .and_then(|cb| cb.get(ib_off as usize..ib_off as usize + 16))
                    .and_then(|ib| {
                        let iaddr = u64::from_le_bytes(ib[0..8].try_into().unwrap());
                        let isize = u32::from_le_bytes(ib[8..12].try_into().unwrap());
                        let ifmt = u32::from_le_bytes(ib[12..16].try_into().unwrap());
                        if iaddr == 0 || isize == 0 {
                            None
                        } else {
                            let f = crate::topology::index_format(ifmt)?;
                            self.gfx.ib = Some((iaddr, isize, ifmt));
                            Some((iaddr, isize, f))
                        }
                    });
                match from_cmd {
                    Some(x) => Some(x),
                    None if cpu_valid_ibs > 0 => continue,
                    None => match self.gfx.ib {
                        Some((iaddr, isize, ifmt)) if iaddr != 0 && isize != 0 => {
                            crate::topology::index_format(ifmt).map(|f| (iaddr, isize, f))
                        }
                        _ => continue,
                    },
                }
            } else {
                index
            };
            bind_root(&self.table, i);
            if !l.vb_offsets.is_empty() {
                if let Some(cb) = cmd_bytes {
                    for &(slot, voff) in &l.vb_offsets {
                        if slot < MAX_VERTEX_SLOTS {
                            if let Some(vb) = cb.get(voff as usize..voff as usize + 16) {
                                let vaddr = u64::from_le_bytes(vb[0..8].try_into().unwrap());
                                let vsize = u32::from_le_bytes(vb[8..12].try_into().unwrap());
                                let vstride = u32::from_le_bytes(vb[12..16].try_into().unwrap());
                                self.gfx.vbs[slot as usize] = (vaddr, vsize, vstride);
                                if vaddr != 0 && (p.vertex_slots & (1 << slot)) != 0 {
                                    unsafe {
                                        self.table.setAddress_attributeStride_atIndex(
                                            vaddr,
                                            vstride as usize,
                                            vertex_buffer_index(slot),
                                        )
                                    };
                                }
                            }
                        }
                    }
                }
            }
            if let Some((base_va, span_size, _)) = gpu_ib {
                let a16 = work_addr + (2 * i) * work_stride;
                let a32 = work_addr + (2 * i + 1) * work_stride;
                unsafe {
                    if p.needs_draw_params {
                        self.table.setAddress_atIndex(
                            a16,
                            ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize,
                        );
                    }
                    if ia16 != 0 {
                        self.table
                            .setAddress_atIndex(ia16, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
                    }
                    e.drawIndexedPrimitives_indexType_indexBuffer_indexBufferLength_indirectBuffer(
                        prim, f16.mtl, base_va, span_size, a16,
                    );
                    if p.needs_draw_params {
                        self.table.setAddress_atIndex(
                            a32,
                            ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize,
                        );
                    }
                    if ia32 != 0 {
                        self.table
                            .setAddress_atIndex(ia32, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
                    }
                    e.drawIndexedPrimitives_indexType_indexBuffer_indexBufferLength_indirectBuffer(
                        prim, f32.mtl, base_va, span_size, a32,
                    );
                }
                continue;
            }
            let a = work_addr + i * work_stride;
            unsafe {
                if p.needs_draw_params {
                    self.table
                        .setAddress_atIndex(a, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
                }
                match cmd_ib {
                    Some((addr, size, f)) => {
                        if let Ok(ia) = binding::static_index_type_va(f.ir) {
                            self.table.setAddress_atIndex(
                                ia,
                                ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize,
                            );
                        }
                        e.drawIndexedPrimitives_indexType_indexBuffer_indexBufferLength_indirectBuffer(
                            prim,
                            f.mtl,
                            addr,
                            size as usize,
                            a,
                        );
                    }
                    None => e.drawPrimitives_indirectBuffer(prim, a),
                }
            }
        }
        self.invalidate_table_cache();
        Ok(())
    }

    /// Routes `ExecuteIndirect` (`Work::Draw` / `Work::DrawIndexed`) for tessellation-emulation
    /// graphics PSOs through a GPU compute expansion into Metal 4 indirect mesh-threadgroup
    /// dispatches, honoring optional count buffers, predication, and per-command root patches.
    fn execute_indirect_tessellation_emulated(
        &mut self,
        l: &IndirectLayout,
        max_count: u32,
        args: u64,
        args_cpu: Option<&[u8]>,
        count: Option<u64>,
    ) -> Result<(), String> {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self.gfx.pso.clone().ok_or("indirect tessellation draw without a graphics PSO")?;
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
        let obj_tg_size = patches_per_obj_tg.saturating_mul(threads_per_patch).max(1);
        let mesh_tg_size = max_input_prims;

        let default_index = if l.work == Work::DrawIndexed && l.ib_offset.is_none() {
            let (addr, _size, fmt) =
                self.gfx.ib.ok_or("indirect indexed tessellation draw without an index buffer")?;
            let f = crate::topology::index_format(fmt)
                .ok_or_else(|| format!("index format {fmt}"))?;
            Some((f.ir, addr))
        } else {
            None
        };
        if l.work == Work::DrawIndexed && l.ib_offset.is_some() && args_cpu.is_none() && self.gfx.ib.is_none() {
            return Err("indirect indexed tessellation draw with GPU-only IB view requires a bound index buffer".into());
        }

        let (tlab_size, patches) = if l.patches.is_empty() {
            (0, Vec::new())
        } else {
            let rs = self.gfx_root.rs.as_ref().ok_or(
                "ExecuteIndirect with root argument changes but no root signature",
            )?;
            (rs.layout().size, l.kernel_patches(rs.layout())?)
        };
        let ts = indirect::tlab_stride(tlab_size);
        let pred = self.predication;

        let make_draw_info = |idx: Option<(u16, u64)>| -> dxil_metal::runtime::RuntimeDrawInfo {
            let (index_type, index_buffer) = match idx {
                Some((ir_idx, ib_va)) => (ir_idx, ib_va),
                None => (NON_INDEXED_DRAW, 0),
            };
            dxil_metal::runtime::RuntimeDrawInfo {
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
            }
        };

        self.pt_split();
        let ring = self.ring.clone().ok_or("command list is not open")?;
        const ARGS_OUT_STRIDE: u32 = 32;
        const GRID_OUT_STRIDE: u32 = 16;
        let (root, args_out, grid_out, tlabs, kernel_params_va, default_info_va, default_vbs_va) = {
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
            let (args_out, _) = r.alloc(ARGS_OUT_STRIDE as usize * max_count as usize, 16)?;
            let (grid_out, _) = r.alloc(GRID_OUT_STRIDE as usize * max_count as usize, 16)?;
            let tout = if patches.is_empty() {
                0
            } else {
                r.alloc(ts as usize * max_count as usize, 256)?.0
            };
            let mut tp = TessGridParams {
                args,
                count: count.unwrap_or(0),
                args_out,
                grid_out,
                tlab_out: tout,
                tlab_in: root,
                pred: pred.map_or(0, |pr| pr.gpu_va),
                stride: l.stride,
                max_count,
                work_offset: l.work_offset,
                work_size: l.work.size(),
                args_out_stride: ARGS_OUT_STRIDE,
                grid_out_stride: GRID_OUT_STRIDE,
                obj_tg_vtx_stride,
                tlab_size,
                tlab_stride: ts,
                npatch: patches.len() as u32,
                pred_op: pred.map_or(0, |pr| pr.op),
                _pad: 0,
                patches: [Default::default(); indirect::MAX_PATCHES],
            };
            tp.patches[..patches.len()].copy_from_slice(&patches);
            let kernel_params_va = r.upload(tp.bytes(), 256)?;
            let default_info_va = r.upload(&make_draw_info(default_index).to_bytes(), 16)?;
            let mut vb_bytes = [0u8; 31 * 16];
            for s in 0..31usize {
                let (addr, size, stride) =
                    if s < self.gfx.vbs.len() { self.gfx.vbs[s] } else { (0, 0, 0) };
                let rv = dxil_metal::runtime::RuntimeVertexBuffer { addr, length: size, stride };
                vb_bytes[s * 16..(s + 1) * 16].copy_from_slice(&rv.to_bytes());
            }
            let default_vbs_va = r.upload(&vb_bytes, 16)?;
            (
                root,
                args_out,
                grid_out,
                (tout != 0).then_some((tout, ts as u64)),
                kernel_params_va,
                default_info_va,
                default_vbs_va,
            )
        };

        // Expand D3D indirect draw args into Metal indirect mesh-threadgroup grid + draw params.
        let k = indirect::tess_grid_kernel()?;
        let ce = self.compute().ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::EI_EXPAND, 1);
        ce.setComputePipelineState(k);
        self.compute_pso_bound = false;
        self.set_table_addr(kernel_params_va, 0);
        ce.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: (max_count as usize).div_ceil(64), height: 1, depth: 1 },
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

        // Pre-upload any per-command CPU-visible IB/VB overrides before opening the render encoder.
        let mut per_cmd_overrides: Option<Vec<(Option<u64>, Option<u64>)>> = None;
        if (l.ib_offset.is_some() || !l.vb_offsets.is_empty()) && args_cpu.is_some() {
            let cb_all = args_cpu.unwrap();
            let mut overrides = Vec::with_capacity(max_count as usize);
            let mut r = ring.lock().unwrap();
            for i in 0..max_count as usize {
                let cmd_bytes = cb_all.get(i * (l.stride as usize)..);
                let cmd_info_va = if let Some(ib_off) = l.ib_offset {
                    let ib = cmd_bytes
                        .and_then(|cb| cb.get(ib_off as usize..ib_off as usize + 16))
                        .and_then(|ib| {
                            let iaddr = u64::from_le_bytes(ib[0..8].try_into().unwrap());
                            let isize = u32::from_le_bytes(ib[8..12].try_into().unwrap());
                            let ifmt = u32::from_le_bytes(ib[12..16].try_into().unwrap());
                            if iaddr == 0 || isize == 0 {
                                None
                            } else {
                                let f = crate::topology::index_format(ifmt)?;
                                self.gfx.ib = Some((iaddr, isize, ifmt));
                                Some((f.ir, iaddr))
                            }
                        })
                        .or_else(|| {
                            self.gfx.ib.and_then(|(iaddr, isize, ifmt)| {
                                (iaddr != 0 && isize != 0)
                                    .then(|| crate::topology::index_format(ifmt).map(|f| (f.ir, iaddr)))
                                    .flatten()
                            })
                        });
                    Some(r.upload(&make_draw_info(ib).to_bytes(), 16)?)
                } else {
                    None
                };
                let cmd_vbs_va = if !l.vb_offsets.is_empty() {
                    if let Some(cb) = cmd_bytes {
                        for &(slot, voff) in &l.vb_offsets {
                            if slot < MAX_VERTEX_SLOTS {
                                if let Some(vb) = cb.get(voff as usize..voff as usize + 16) {
                                    let vaddr = u64::from_le_bytes(vb[0..8].try_into().unwrap());
                                    let vsize = u32::from_le_bytes(vb[8..12].try_into().unwrap());
                                    let vstride = u32::from_le_bytes(vb[12..16].try_into().unwrap());
                                    self.gfx.vbs[slot as usize] = (vaddr, vsize, vstride);
                                }
                            }
                        }
                    }
                    let mut vb_bytes = [0u8; 31 * 16];
                    for s in 0..31usize {
                        let (addr, size, stride) =
                            if s < self.gfx.vbs.len() { self.gfx.vbs[s] } else { (0, 0, 0) };
                        let rv = dxil_metal::runtime::RuntimeVertexBuffer { addr, length: size, stride };
                        vb_bytes[s * 16..(s + 1) * 16].copy_from_slice(&rv.to_bytes());
                    }
                    Some(r.upload(&vb_bytes, 16)?)
                } else {
                    None
                };
                overrides.push((cmd_info_va, cmd_vbs_va));
            }
            per_cmd_overrides = Some(overrides);
        }

        let writes_uav = self.gfx_pso_writes_uav(&p);
        if self.gfx.pass_uav_hazard && writes_uav {
            self.end_encoder();
        }
        let eff_reads = p.pre_raster_reads | READS_BUF;
        let emu_pre = MTLStages::Object | MTLStages::Mesh;
        self.bar.opening_pre_raster_stages = emu_pre;
        let enc_res = unsafe { self.draw_encoder_for(eff_reads) };
        self.bar.opening_pre_raster_stages = PRE_RASTER;
        let (e, size) = enc_res?;
        self.rp_sync.enc_pre_raster_reads |= eff_reads;
        self.rp_sync.enc_ps_reads |= p.ps_read_classes;
        self.rp_sync.enc_uav_used |= writes_uav || p.uses_uav;
        self.render_mid_barrier(&e, emu_pre);
        self.record_gfx_draw_uav_writes(
            writes_uav,
            p.vs_writes_uav,
            p.ps_writes_uav,
            emu_pre,
        );
        self.sync_pre_raster(&e, eff_reads, emu_pre);
        self.apply_dynamic(&e, &p, size);
        let fast = self.opts.opt_fast_enc_state;
        if !(fast && self.rp_sync.enc_render_cache.mesh_stages_bound) {
            e.setArgumentTable_atStages(&self.table, crate::mesh::STAGES);
            self.rp_sync.enc_render_cache.mesh_stages_bound = true;
        }
        self.bind_table(root);
        if root != 0 {
            self.set_table_addr(root, ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT as usize);
        }
        let tess_tables_va = binding::tessellator_tables_va()?;
        self.set_table_addr(tess_tables_va, TESSELLATOR_TABLES_BIND_POINT as usize);
        self.set_table_addr(default_info_va, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
        self.set_table_addr(default_vbs_va, VERTEX_BUFFER_BIND_POINT as usize);

        let mut ran = MTLStages::Object | MTLStages::Mesh;
        if !self.bar.pass_targets.is_empty()
            || self.active_dsv().is_some_and(|v| v.resource != 0)
            || p.ps.is_some()
        {
            ran |= MTLStages::Fragment;
        }
        self.note_stage_ran(ran);
        stats::add(&stats::c::ei_draws, max_count as u64);
        unsafe {
            e.setObjectThreadgroupMemoryLength_atIndex(15360, 0);
        }
        for i in 0..max_count as u64 {
            if let Some((t, tstride)) = tlabs {
                let tva = t + i * tstride;
                unsafe {
                    self.table.setAddress_atIndex(tva, ARGUMENT_BUFFER_BIND_POINT as usize);
                    self.table.setAddress_atIndex(tva, ARGUMENT_BUFFER_HULL_DOMAIN_BIND_POINT as usize);
                }
            }
            if let Some(ref overrides) = per_cmd_overrides {
                let (cmd_info_va, cmd_vbs_va) = overrides[i as usize];
                if let Some(ia) = cmd_info_va {
                    unsafe {
                        self.table.setAddress_atIndex(ia, ARGUMENT_BUFFER_UNIFORMS_BIND_POINT as usize);
                    }
                }
                if let Some(va) = cmd_vbs_va {
                    unsafe {
                        self.table.setAddress_atIndex(va, VERTEX_BUFFER_BIND_POINT as usize);
                    }
                }
            }
            let a = args_out + i * ARGS_OUT_STRIDE as u64;
            let g = grid_out + i * GRID_OUT_STRIDE as u64;
            unsafe {
                self.table.setAddress_atIndex(a, ARGUMENT_BUFFER_DRAW_ARGUMENTS_BIND_POINT as usize);
            }
            e.drawMeshThreadgroupsWithIndirectBuffer_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
                g,
                MTLSize { width: obj_tg_size as usize, height: 1, depth: 1 },
                MTLSize { width: mesh_tg_size as usize, height: 1, depth: 1 },
            );
        }
        self.invalidate_table_cache();
        Ok(())
    }

    /// Allocate the ICB slice, GPU-written `icb_scratch` outputs (`range_out`, `tlab_out`,
    /// `args_out`), and upload `IcbParams` to the append-only CPU ring.
    ///
    /// MUST be called strictly AFTER any `end_encoder()` / `draw_encoder_for()` /
    /// `begin_draw_ext()` / `compute()` that could close the previous render pass and rewind
    /// `self.icb.icbs` + `self.icb.icb_scratch`; otherwise that rewind would reset the cursors below
    /// this draw's freshly allocated slots.
    fn alloc_icb_params(
        &mut self,
        q: &mut IcbParams,
        max_count: u32,
        need_vb_binds: usize,
        has_patches: bool,
        tlab_stride: u32,
        needs_draw_params: bool,
    ) -> Result<(Retained<MtlIcb>, u64, u64), String> {
        let (icb, location) = self
            .icb
            .icbs
            .as_ref()
            .ok_or("command list is not open")?
            .lock()
            .unwrap()
            .alloc_for(max_count, need_vb_binds)?;
        q.icb = unsafe { std::mem::transmute::<MTLResourceID, u64>(icb.gpuResourceID()) };
        q.location = location;
        // GPU-written/GPU-read scratch lives in `icb_scratch` (rewindable between closed
        // render passes); CPU-written `IcbParams` stays in append-only `ring` until allocator reset.
        let scratch = self.icb.icb_scratch.clone().ok_or("command list is not open")?;
        {
            let mut sr = scratch.lock().unwrap();
            q.range_out = sr.alloc(8, 16)?.0;
            if has_patches {
                q.tlab_out = sr.alloc(tlab_stride as usize * max_count as usize, 256)?.0;
            }
            if needs_draw_params {
                q.args_out = sr.alloc(32 * max_count as usize, 32)?.0;
            }
        }
        // Coalesce graphics root flush + `IcbParams` upload under a single `ring` lock.
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let (params, root) = {
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
            q.tlab_in = root;
            q.tlab_static = root;
            (r.upload(q.bytes(), 256)?, root)
        };
        self.icb.enc_icb_cmds = self.icb.enc_icb_cmds.saturating_add(max_count);
        Ok((icb, params, root))
    }

    /// `ExecuteIndirect` draws encoded on the GPU into an indirect command
    /// buffer (see [`crate::metal::indirect_icb`]): the GPU front end only sees the
    /// commands below the count buffer's value.
    fn execute_indirect_icb(
        &mut self,
        l: &IndirectLayout,
        max_count: u32,
        args: u64,
        count: Option<u64>,
    ) -> Result<(), String> {
        if icb_reuse_enabled()
            && self.icb.enc_icb_cmds > 0
            && self.icb.enc_icb_cmds.saturating_add(max_count) > ios_icb_pass_cap()
            && self.occlusion.is_none()
            && (matches!(self.enc, Encoder::Render(_)) || self.icb.icb_prepass_pending)
        {
            // Close the current render pass (`endEncoding()` + `StoreAction::Store`), rewind
            // `IcbRing` + `icb_scratch` to offset 0, and reopen via `draw_encoder_for()`
            // (`LoadAction::Load`) with a TBDR-safe cross-encoder WAR barrier.
            IOS_ICB_PASS_SPLITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.end_encoder();
        }
        if self.icb.icb_prepass_pending && !self.batch_rtvs_compatible() {
            self.flush_draw_batch(false);
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
        let (tlab_size, patches) = if l.patches.is_empty() {
            (0, Vec::new())
        } else {
            let rs = self.gfx_root.rs.as_ref().ok_or(
                "ExecuteIndirect with root argument changes but no root signature",
            )?;
            (rs.layout().size, l.kernel_patches(rs.layout())?)
        };
        let ts = indirect::tlab_stride(tlab_size);
        let zero_va = binding::static_index_type_va(0)?;
        let mut q = IcbParams {
            args,
            count: count.unwrap_or(0),
            tlab_in: 0,
            tlab_static: 0,
            pred: self.predication.map_or(0, |pr| pr.gpu_va),
            pred_op: self.predication.map_or(0, |pr| pr.op),
            heap: self.heaps[0],
            sampler_heap: self.heaps[1],
            ia16: binding::static_index_type_va(crate::topology::index_format(57).unwrap().ir)
                .unwrap_or(0),
            ia32: binding::static_index_type_va(crate::topology::index_format(42).unwrap().ir)
                .unwrap_or(0),
            ia_none: binding::static_index_type_va(NON_INDEXED_DRAW)?,
            stride: l.stride,
            max_count,
            work_offset: l.work_offset,
            work_size: l.work.size(),
            tlab_size,
            tlab_stride: ts,
            npatch: patches.len() as u32,
            ib_offset: l.ib_offset.unwrap_or(u32::MAX),
            ib_format: 57,
            indexed: (l.work == Work::DrawIndexed) as u32,
            prim: prim.0 as u32,
            vb_mask: p.vertex_slots,
            nvbo: l.vb_offsets.len() as u32,
            draw_params: if self.opts.opt_icb_fast_v3 || self.opts.opt_icb_prepass_compute {
                (u32::from(p.needs_draw_params) * indirect_icb::DRAW_PARAMS_NEEDED)
                    | (u32::from(p.ps.is_some()) * indirect_icb::DRAW_PARAMS_HAS_PS)
            } else {
                p.needs_draw_params as u32
            },
            ..Default::default()
        };
        q.patches[..patches.len()].copy_from_slice(&patches);
        if q.indexed != 0 && l.ib_offset.is_none() {
            let (addr, _size, fmt) =
                self.gfx.ib.ok_or("indirect indexed draw without an index buffer")?;
            q.ib = addr;
            q.ib_format = fmt;
        }
        let use_v3 = self.opts.opt_icb_fast_v3 || self.opts.opt_icb_prepass_compute;
        let use_v2 = self.opts.opt_icb_batch || use_v3;
        if use_v2 {
            let mut slots = p.vertex_slots;
            while slots != 0 {
                let s = slots.trailing_zeros() as usize;
                slots &= slots - 1;
                let (addr, _size, stride) = self.gfx.vbs[s];
                (q.vb[s], q.vb_stride[s]) = if addr != 0 { (addr, stride) } else { (zero_va, 0) };
            }
        } else {
            for s in 0..MAX_VERTEX_SLOTS as usize {
                let (addr, _size, stride) = self.gfx.vbs[s];
                (q.vb[s], q.vb_stride[s]) = if addr != 0 { (addr, stride) } else { (zero_va, 0) };
            }
        }
        for (k, &(slot, off)) in l.vb_offsets.iter().enumerate() {
            q.vbo_slot[k] = slot;
            q.vbo_off[k] = off;
        }
        let need_vb_binds = indirect_icb::required_vb_bind_count(p.vertex_slots, &l.vb_offsets);
        let has_patches = !patches.is_empty();
        stats::inc(&stats::c::ei_icb);
        if self.opts.opt_icb_fast_v3 {
            stats::inc(&stats::c::icb_v3_fast);
        }
        if self.opts.opt_icb_batch && indirect_icb::in_pass() && !pt::split() {
            let _ = p.icb_pso()?;
            let writes_uav = self.gfx_pso_writes_uav(&p);
            let non_const_patch = !l.patches.is_empty()
                && self.gfx_root.rs.as_ref().is_none_or(|rs| {
                    let rl = rs.layout();
                    l.patches.iter().any(|rp| {
                        !matches!(
                            rl.parameters.get(rp.param as usize).map(|pl| &pl.kind),
                            Some(
                                dxil_metal::root_signature::RootParameterLayoutKind::Constants { .. }
                            )
                        )
                    })
                });
            let mid_on = self.mid_qb_stagesync_active();
            let no_gfx_to_cs_hazard = (!self.bar.needs_barrier
                || self.bar.full_why == FullWhy::ListStart)
                && !self.bar.list_unwaited_cs
                && self
                    .unsatisfied_enc_after_for_req(
                        self.bar.split_pend.disp_after & (PRE_RASTER | MTLStages::Fragment),
                        MTLStages::Dispatch,
                        &self.bar.split_pend.disp_req,
                        mid_on,
                    )
                    .is_empty()
                && self.bar.enc_cs_soft_disp.iter().all(|it| {
                    self.unsatisfied_enc_after_for_req(
                        it.after & (PRE_RASTER | MTLStages::Fragment),
                        MTLStages::Dispatch,
                        &it.req,
                        mid_on,
                    )
                    .is_empty()
                });
            let can_prepass = self.opts.opt_icb_prepass_compute
                && (self.icb.icb_prepass_pending
                    || (!self.gfx.draw_pass
                        && !matches!(self.enc, Encoder::Render(_))
                        && no_gfx_to_cs_hazard
                        && (matches!(self.enc, Encoder::Compute(_))
                            || ((q.count != 0 || q.pred != 0) && max_count > 64))));
            let args_bytes = (max_count as u64).saturating_mul(l.stride as u64);
            if can_prepass {
                let (icb, params, _root) = self.alloc_icb_params(
                    &mut q,
                    max_count,
                    need_vb_binds,
                    has_patches,
                    ts,
                    p.needs_draw_params,
                )?;
                let meta = self.snapshot_gfx_seg_exact(
                    &p,
                    q.indexed != 0,
                    true,
                    non_const_patch,
                    !l.vb_offsets.is_empty(),
                    l.ib_offset.is_some(),
                    Some(&[args, q.count, q.pred]),
                );
                self.icb.icb_prepass_pending = true;
                let dyn_idx = self.record_batch_dyn_idx();
                self.icb.icb_segs.push(DrawSeg {
                    pso: p,
                    dyn_idx,
                    deferred_meta: Some(Box::new(meta)),
                    op: DrawOp::Icb {
                        ex: None,
                        params_va: params,
                        max_count,
                        location: q.location,
                        icb,
                        range_out: q.range_out,
                        args_va: args,
                        args_bytes,
                        count_va: q.count,
                        pred_va: q.pred,
                        pred_op: q.pred_op,
                    },
                });
                return Ok(());
            }
            let ex = if use_v3 {
                indirect_icb::expander_v3(&p.rtv_formats, p.sample_count)?
            } else {
                indirect_icb::expander_with(&p.rtv_formats, p.sample_count, true)?
            };
            let check_hazard = self.gfx.pass_uav_hazard
                && self.opts.opt_rp_split_exact_keep
                && (writes_uav || p.uses_uav);
            let need_pre_open = self.need_pre_open_cur_draw_exact();
            let do_gfx = check_hazard || need_pre_open || self.opts.opt_gfx_desc_table_exact;
            let do_uav = writes_uav && self.opts.opt_rp_split_exact_keep;
            let ind_vas = [args, q.count, q.pred];
            if do_gfx || do_uav {
                self.resolve_gfx_binds_cached(
                    &p,
                    q.indexed != 0,
                    true,
                    non_const_patch,
                    !l.vb_offsets.is_empty(),
                    l.ib_offset.is_some(),
                    Some(&ind_vas),
                    do_gfx,
                    do_uav,
                );
            }
            if self.gfx.pass_uav_hazard {
                if self.opts.opt_rp_split_exact_keep {
                    if (writes_uav || p.uses_uav)
                        && self.gfx_cached_overlaps_pass_hazard(non_const_patch)
                    {
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
            let mut eff_reads = self.effective_gfx_pre_raster_reads(&p, q.indexed != 0, true);
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
            self.bar.opening_pre_raster_stages = MTLStages::Vertex;
            let enc_res = unsafe { self.draw_encoder_for(eff_reads) };
            self.bar.opening_pre_raster_stages = PRE_RASTER;
            self.bar.opening_render_split_ok = false;
            let (_e, _size) = match enc_res {
                Ok(v) => v,
                Err(err) => {
                    self.rp_sync.cur_draw_exact_valid = false;
                    return Err(err);
                }
            };
            let (icb, params, _root) = self.alloc_icb_params(
                &mut q,
                max_count,
                need_vb_binds,
                has_patches,
                ts,
                p.needs_draw_params,
            )?;
            self.record_gfx_draw_pre_barrier(&p, eff_reads, writes_uav, non_const_patch);
            self.record_gfx_draw_uav_writes(
                writes_uav,
                p.vs_writes_uav,
                p.ps_writes_uav,
                MTLStages::Vertex,
            );
            self.collect_batch_qbarriers(eff_reads, MTLStages::Vertex);
            self.rp_sync.cur_draw_exact_valid = false;
            if let Some(m) = self.pt_cur() {
                m.draws += 1;
                m.icb += 1;
                m.shader =
                    p.vs.key as u64 ^ p.ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1));
            }
            let dyn_idx = self.record_batch_dyn_idx();
            self.icb.icb_segs.push(DrawSeg {
                pso: p,
                dyn_idx,
                deferred_meta: None,
                op: DrawOp::Icb {
                    ex: Some(ex),
                    params_va: params,
                    max_count,
                    location: q.location,
                    icb,
                    range_out: q.range_out,
                    args_va: args,
                    args_bytes,
                    count_va: q.count,
                    pred_va: q.pred,
                    pred_op: q.pred_op,
                },
            });
            return Ok(());
        }
        let icb_pso = p.icb_pso()?;
        let before_icb = if use_v3 || self.opts.opt_icb_enc_barrier_exact {
            if !use_v3 && self.opts.opt_icb_enc_barrier_exact && indirect_icb::in_pass() {
                stats::inc(&stats::c::icb_enc_barrier_exact);
            }
            if p.ps.is_some() {
                MTLStages::Vertex | MTLStages::Fragment
            } else {
                MTLStages::Vertex
            }
        } else {
            MTLStages::Vertex | MTLStages::Object | MTLStages::Mesh | MTLStages::Fragment
        };
        let (e, icb) = if indirect_icb::in_pass() {
            let ex = if use_v3 {
                indirect_icb::expander_v3(&p.rtv_formats, p.sample_count)?
            } else {
                indirect_icb::expander_with(&p.rtv_formats, p.sample_count, use_v2)?
            };
            let (e, _p, _prim) = self.begin_draw_ext(None, &[0u8; 20], true)?;
            let (icb, params, root) = self.alloc_icb_params(
                &mut q,
                max_count,
                need_vb_binds,
                has_patches,
                ts,
                p.needs_draw_params,
            )?;
            if icb_reuse_enabled() && !self.icb.enc_icb_war_done {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Vertex,
                    MTL4VisibilityOptions::Device,
                );
                self.icb.enc_icb_war_done = true;
            }
            self.rp_sync
                .enc_render_cache
                .apply_pso(&e, &ex, self.opts.opt_fast_enc_state);
            // Index 0 is the descriptor heap slot; bind_table restores it.
            self.set_table_addr(params, 0);
            unsafe {
                e.drawPrimitives_vertexStart_vertexCount(
                    MTLPrimitiveType::Point,
                    0,
                    max_count as usize,
                );
            }
            e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                MTLStages::Vertex,
                before_icb,
                MTL4VisibilityOptions::Device,
            );
            self.bind_table(root);
            (e, icb)
        } else {
            let k = if use_v3 {
                indirect_icb::kernel_v3()?
            } else {
                indirect_icb::kernel_with(use_v2)?
            };
            let e = self.compute().ok_or("compute encoder")?;
            let (icb, params, _root) = self.alloc_icb_params(
                &mut q,
                max_count,
                need_vb_binds,
                has_patches,
                ts,
                p.needs_draw_params,
            )?;
            if icb_reuse_enabled() && !self.icb.enc_icb_war_done {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Dispatch,
                    MTL4VisibilityOptions::Device,
                );
                self.icb.enc_icb_war_done = true;
            }
            self.pt_note_util(pt::synth::EI_EXPAND, 1);
            e.setComputePipelineState(k);
            self.compute_pso_bound = false;
            // Index 0 is the descriptor heap slot; bind_table restores it.
            self.set_table_addr(params, 0);
            e.dispatchThreadgroups_threadsPerThreadgroup(
                MTLSize { width: (max_count as usize).div_ceil(64), height: 1, depth: 1 },
                MTLSize { width: 64, height: 1, depth: 1 },
            );
            self.note_stage_ran(MTLStages::Dispatch);
            // The ICB commands and range are read by the render pass.
            self.produced(MTLStages::Dispatch, before_icb);
            (self.begin_draw_ext(None, &[0u8; 20], true)?.0, icb)
        };
        self.rp_sync
            .enc_render_cache
            .apply_pso(&e, icb_pso, self.opts.opt_fast_enc_state);
        if let Some(m) = self.pt_cur() {
            m.icb += 1;
        }
        unsafe { e.executeCommandsInBuffer_indirectBuffer(&icb, q.range_out) };
        self.icb.enc_used_icb = true;
        // ICB commands bind their own buffers: restore the encoder's table
        // and state for later direct draws.
        e.setArgumentTable_atStages(&self.table, MTLRenderStages::Vertex | MTLRenderStages::Fragment);
        self.rp_sync.enc_render_cache.mesh_stages_bound = false;
        self.invalidate_table_cache();
        self.gfx.dirty = GFX_DIRTY_ALL;
        Ok(())
    }

    /// GPU-encoded `ExecuteIndirect(DispatchMesh)` using `MTLIndirectCommandTypeDrawMeshThreadgroups`
    /// and an in-pass `Point` vertex expander (`aw2_ei_mesh_icb_vs`), staying inside the open
    /// TBDR render pass without splitting into a compute encoder, wiping discard marks, or
    /// issuing a `0..max_count` CPU loop.
    fn execute_indirect_mesh_icb(
        &mut self,
        l: &IndirectLayout,
        max_count: u32,
        args: u64,
        count: Option<u64>,
    ) -> Result<(), String> {
        if !self.icb.icb_segs.is_empty() {
            self.flush_draw_batch(true);
        }
        let p = self
            .gfx
            .mesh
            .clone()
            .ok_or("ExecuteIndirect DispatchMesh without a mesh PSO")?;
        let icb_pso = p.icb_pso()?;
        let (tlab_size, patches) = if l.patches.is_empty() {
            (0, Vec::new())
        } else {
            let rs = self.gfx_root.rs.as_ref().ok_or(
                "ExecuteIndirect with root argument changes but no root signature",
            )?;
            (rs.layout().size, l.kernel_patches(rs.layout())?)
        };
        let ts = indirect::tlab_stride(tlab_size);
        let has_patches = !patches.is_empty();
        let has_object = p.object_threads.is_some();
        let has_ps = p.pixel.is_some();
        let mut before_icb = MTLStages::Mesh;
        if has_object {
            before_icb |= MTLStages::Object;
        }
        if has_ps {
            before_icb |= MTLStages::Fragment;
        }

        let mut q = crate::mesh::MeshIcbParams {
            args,
            count: count.unwrap_or(0),
            pred: self.predication.map_or(0, |pr| pr.gpu_va),
            pred_op: self.predication.map_or(0, |pr| pr.op),
            heap: self.heaps[0],
            sampler_heap: self.heaps[1],
            stride: l.stride,
            max_count,
            work_offset: l.work_offset,
            tlab_size,
            tlab_stride: ts,
            npatch: patches.len() as u32,
            obj_threads: p.object_threads.unwrap_or([1, 1, 1]),
            mesh_threads: p.mesh_threads,
            has_object: u32::from(has_object),
            has_ps: u32::from(has_ps),
            ..Default::default()
        };
        q.patches[..patches.len()].copy_from_slice(&patches);

        stats::inc(&stats::c::ei_icb);
        let cb_ptr = Retained::as_ptr(&self.cb) as usize;
        let alloc_key = self.alloc_key;

        let (e, icb) = if indirect_icb::in_pass() {
            // Preserve `pass_uav_hazard` ordering, render-pass barriers (including
            // `MTLStages::Vertex` + `READS_BUF` for the vertex expander's `args`/`count`/`pred`
            // inputs), dynamic state, and masked RTV attachments via `begin_mesh_for(true, true)`.
            let (e, _p) = self.begin_mesh_for(true, true)?;
            let eff_fmts = self.open_pass_rtv_formats(&p.rtv_formats);
            let ex = crate::mesh::mesh_icb_expander(&eff_fmts, p.sample_count)?;
            let (icb, location) =
                crate::mesh::alloc_mesh_icb(cb_ptr, alloc_key, max_count, has_object)?;
            q.icb = unsafe { std::mem::transmute::<MTLResourceID, u64>(icb.gpuResourceID()) };
            q.location = location;
            let scratch = self.icb.icb_scratch.clone().ok_or("command list is not open")?;
            {
                let mut sr = scratch.lock().unwrap();
                q.range_out = sr.alloc(8, 16)?.0;
                if has_patches {
                    q.tlab_out = sr.alloc(ts as usize * max_count as usize, 256)?.0;
                }
            }
            let root = if self.gfx_root.rs.is_some() { self.gfx_root.bound } else { 0 };
            q.tlab_in = root;
            q.tlab_static = root;
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let params_va = ring.lock().unwrap().upload(q.bytes(), 256)?;
            self.icb.enc_icb_cmds = self.icb.enc_icb_cmds.saturating_add(max_count);

            if icb_reuse_enabled() && !self.icb.enc_icb_war_done {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Vertex,
                    MTL4VisibilityOptions::Device,
                );
                self.icb.enc_icb_war_done = true;
            }
            if self.occlusion.is_some() {
                unsafe { e.setVisibilityResultMode_offset(MTLVisibilityResultMode::Disabled, 0) };
            }
            self.rp_sync
                .enc_render_cache
                .apply_pso(&e, &ex, self.opts.opt_fast_enc_state);
            // Index 0 is the descriptor heap slot; `bind_table` restores it after expansion.
            self.set_table_addr(params_va, 0);
            unsafe {
                e.drawPrimitives_vertexStart_vertexCount(
                    MTLPrimitiveType::Point,
                    0,
                    max_count as usize,
                );
            }
            self.note_stage_ran(MTLStages::Vertex);
            e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                MTLStages::Vertex,
                before_icb,
                MTL4VisibilityOptions::Device,
            );
            if let Some(o) = &self.occlusion {
                let mode = if o.binary {
                    MTLVisibilityResultMode::Boolean
                } else {
                    MTLVisibilityResultMode::Counting
                };
                unsafe { e.setVisibilityResultMode_offset(mode, o.index as usize * 8) };
            }
            self.bind_table(root);
            (e, icb)
        } else {
            let k = crate::mesh::mesh_icb_kernel()?;
            let root = self.flush_root(false)?;
            let ce = self.compute().ok_or("compute encoder")?;
            let (icb, location) =
                crate::mesh::alloc_mesh_icb(cb_ptr, alloc_key, max_count, has_object)?;
            q.icb = unsafe { std::mem::transmute::<MTLResourceID, u64>(icb.gpuResourceID()) };
            q.location = location;
            let scratch = self.icb.icb_scratch.clone().ok_or("command list is not open")?;
            {
                let mut sr = scratch.lock().unwrap();
                q.range_out = sr.alloc(8, 16)?.0;
                if has_patches {
                    q.tlab_out = sr.alloc(ts as usize * max_count as usize, 256)?.0;
                }
            }
            q.tlab_in = root;
            q.tlab_static = root;
            let ring = self.ring.clone().ok_or("command list is not open")?;
            let params_va = ring.lock().unwrap().upload(q.bytes(), 256)?;
            self.icb.enc_icb_cmds = self.icb.enc_icb_cmds.saturating_add(max_count);

            if icb_reuse_enabled() && !self.icb.enc_icb_war_done {
                ce.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::All,
                    MTLStages::Dispatch,
                    MTL4VisibilityOptions::Device,
                );
                self.icb.enc_icb_war_done = true;
            }
            self.pt_note_util(pt::synth::EI_EXPAND, 1);
            ce.setComputePipelineState(k);
            self.compute_pso_bound = false;
            self.set_table_addr(params_va, 0);
            ce.dispatchThreadgroups_threadsPerThreadgroup(
                MTLSize { width: (max_count as usize).div_ceil(64), height: 1, depth: 1 },
                MTLSize { width: 64, height: 1, depth: 1 },
            );
            self.note_stage_ran(MTLStages::Dispatch);
            self.produced(MTLStages::Dispatch, before_icb);
            (self.begin_mesh(true)?.0, icb)
        };

        self.rp_sync
            .enc_render_cache
            .apply_pso(&e, icb_pso, self.opts.opt_fast_enc_state);
        if let Some(m) = self.pt_cur() {
            m.icb += 1;
        }
        unsafe { e.executeCommandsInBuffer_indirectBuffer(&icb, q.range_out) };
        self.icb.enc_used_icb = true;
        // Restore the encoder's argument table and dirty flags after ICB execution.
        e.setArgumentTable_atStages(&self.table, MTLRenderStages::Vertex | MTLRenderStages::Fragment);
        self.rp_sync.enc_render_cache.mesh_stages_bound = false;
        self.invalidate_table_cache();
        self.gfx.dirty = GFX_DIRTY_ALL;
        Ok(())
    }
}
