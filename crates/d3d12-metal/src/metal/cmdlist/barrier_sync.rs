//! Cross-encoder stage-sync tracking, intra-compute tight barrier analysis,
//! deferred pre-raster soft-sync partitioning, and open-render-pass kept-write tracking.

use super::opts::*;
use super::state::*;
use super::CommandList;
use crate::metal::encstats as stats;
use crate::metal::pipeline::{GraphicsPipeline, READS_BUF, READS_TEX};
use crate::metal::resource::{Resource, SubRange, HK_ALL, HK_CBV_BUF};
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

const KNOWN_ALL_STAGES: MTLStages = MTLStages::Vertex
    .union(MTLStages::Object)
    .union(MTLStages::Mesh)
    .union(MTLStages::Fragment)
    .union(MTLStages::Dispatch)
    .union(MTLStages::Blit)
    .union(MTLStages::AccelerationStructure);

impl CommandList {
    #[inline]
    pub(super) fn has_any_soft_pre_obl(&self) -> bool {
        self.bar.vtx_defer != NO_VTX
            || self.bar.vtx_carry != NO_VTX
            || self.bar.split_pend.soft_after != [MTLStages::empty(); 2]
            || self.bar.clear_tex_gfx_soft != [MTLStages::empty(); 2]
    }

    #[inline]
    pub(super) fn unsatisfied_sync_after(
        qa: MTLStages,
        sync: &StageSync,
        req: &[u32; 7],
    ) -> MTLStages {
        if qa.is_empty() || !sync.pre_list_all {
            return qa;
        }
        let known_all = KNOWN_ALL_STAGES;
        let check_all = qa == MTLStages::All;
        let mut out = if check_all {
            MTLStages::empty()
        } else {
            qa - known_all
        };
        for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
            if (check_all || qa.contains(pbit)) && sync.gen[pi] < req[pi] {
                out |= pbit;
            }
        }
        out
    }

    #[inline]
    pub(super) fn reset_cs_soft_disp(&mut self) {
        self.bar.cs_soft_disp_items.clear();
        self.bar.cs_soft_disp_hard_after = MTLStages::empty();
        self.bar.cs_soft_disp_hard_req = [0; 7];
        self.bar.cs_soft_disp_poisoned = false;
    }

    pub(super) fn record_cs_soft_disp(&mut self, r: usize, after: MTLStages, req: [u32; 7]) {
        if !self.ps_to_cs_active() || after.is_empty() || self.bar.cs_soft_disp_poisoned {
            return;
        }
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        let v2 = self.ps_to_cs_v2_active();
        let clean_r = r != 0
            && unsafe {
                let res = &*(r as *const Resource);
                let non_alias = if v2 {
                    !res.may_alias_placed()
                } else {
                    res.placed_in.is_none()
                };
                non_alias && (res.desc.Flags & SIMUL == 0)
            };
        if !clean_r && !v2 {
            self.bar.cs_soft_disp_poisoned = true;
            self.bar.cs_soft_disp_items.clear();
            return;
        }
        let eff_after = if after == MTLStages::All {
            if self.bar.stage_gen[STAGE_DISPATCH] > 0
                && self.bar.stage_sync[STAGE_DISPATCH].pre_list_all
            {
                let u = Self::unsatisfied_sync_after(
                    MTLStages::All,
                    &self.bar.stage_sync[STAGE_DISPATCH],
                    &req,
                );
                if u.is_empty() {
                    return;
                }
                u
            } else {
                self.bar.cs_soft_disp_hard_after |= MTLStages::All;
                merge_req_for_after(&mut self.bar.cs_soft_disp_hard_req, MTLStages::All, &req);
                return;
            }
        } else {
            after
        };
        if !clean_r {
            self.bar.cs_soft_disp_hard_after |= eff_after;
            merge_req_for_after(&mut self.bar.cs_soft_disp_hard_req, eff_after, &req);
            return;
        }
        let mut r_req = [0u32; 7];
        merge_req_for_after(&mut r_req, eff_after, &req);
        if let Some(ex) = self.bar.cs_soft_disp_items.iter_mut().find(|it| it.r == r) {
            ex.after |= eff_after;
            merge_req_for_after(&mut ex.req, eff_after, &r_req);
        } else if self.bar.cs_soft_disp_items.len() < MAX_CS_SOFT_DISP {
            self.bar.cs_soft_disp_items.push(CsSoftDispObl {
                r,
                after: eff_after,
                req: r_req,
            });
        } else if v2 {
            self.bar.cs_soft_disp_hard_after |= eff_after;
            merge_req_for_after(&mut self.bar.cs_soft_disp_hard_req, eff_after, &r_req);
        } else {
            self.bar.cs_soft_disp_poisoned = true;
            self.bar.cs_soft_disp_items.clear();
        }
    }

    pub(super) fn record_cs_disp_passthrough(&mut self, r: usize, after: MTLStages, req: [u32; 7]) {
        let s = after - MTLStages::Dispatch;
        if r == 0 || s.is_empty() {
            return;
        }
        let mut r_req = [0u32; 7];
        merge_req_for_after(&mut r_req, s, &req);
        if let Some(ex) = self.bar.cs_disp_passthrough.iter_mut().find(|e| e.0 == r) {
            ex.1 |= s;
            merge_req_for_after(&mut ex.2, s, &r_req);
        } else {
            self.bar.cs_disp_passthrough.push((r, s, r_req));
        }
    }

    #[inline]
    pub(super) fn cs_disp_passthrough_for(&self, r: usize) -> MTLStages {
        let mut out = MTLStages::empty();
        for &(rk, st, _) in &self.bar.cs_disp_passthrough {
            if r == 0 || rk == 0 || rk == r {
                out |= st;
            }
        }
        out
    }

    pub(super) fn flush_enc_cs_soft_disp_all(&mut self) {
        if self.bar.enc_cs_soft_disp.is_empty() {
            return;
        }
        let deferred = std::mem::take(&mut self.bar.enc_cs_soft_disp);
        if let Encoder::Compute(e) = &self.enc {
            let e = e.clone();
            let mid_on = self.mid_qb_stagesync_active();
            let mut qa = MTLStages::empty();
            let mut qreq = [0u32; 7];
            for it in deferred {
                let u = self.unsatisfied_enc_after_for_req(
                    it.after,
                    MTLStages::Dispatch,
                    &it.req,
                    mid_on,
                );
                if !u.is_empty() {
                    qa |= u;
                    merge_req_for_after(&mut qreq, u, &it.req);
                }
            }
            if !qa.is_empty() {
                let emit_qa = if self.split_compute_exact_active() {
                    self.fold_dominated_after(qa, &qreq)
                } else {
                    qa
                };
                if !emit_qa.is_empty() {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        emit_qa,
                        MTLStages::Dispatch,
                        MTL4VisibilityOptions::Device,
                    );
                    let rec_qa = if self.opts.opt_rp_keep_write_exact {
                        emit_qa
                    } else {
                        qa
                    };
                    self.record_enc_queue_sync(rec_qa, MTLStages::Dispatch);
                    stats::inc(&stats::c::cs_mid_qb);
                    self.note_enc_sync_pair(rec_qa, MTLStages::Dispatch);
                }
            }
        } else {
            let ds = self.bar.stage_sync[STAGE_DISPATCH];
            let dgen_ran = self.bar.stage_gen[STAGE_DISPATCH] > 0;
            for it in deferred {
                let u = if dgen_ran {
                    Self::unsatisfied_sync_after(it.after, &ds, &it.req)
                } else {
                    it.after
                };
                if !u.is_empty() {
                    self.bar.pend_after |= u;
                    self.bar.pend_before |= MTLStages::Dispatch;
                    self.bar.split_pend.disp_after |= u;
                    merge_req_for_after(&mut self.bar.split_pend.disp_req, u, &it.req);
                    self.record_cs_soft_disp(it.r, u, it.req);
                }
            }
            self.sync_legacy_from_split();
        }
    }

    pub(super) fn cs_soft_disp_invariant_ok(&self) -> bool {
        if self.bar.cs_soft_disp_poisoned {
            return false;
        }
        let disp_check = if self.bar.split_pend.disp_after == MTLStages::All
            && self.bar.stage_gen[STAGE_DISPATCH] > 0
            && self.bar.stage_sync[STAGE_DISPATCH].pre_list_all
        {
            Self::unsatisfied_sync_after(
                MTLStages::All,
                &self.bar.stage_sync[STAGE_DISPATCH],
                &self.bar.split_pend.disp_req,
            )
        } else {
            self.bar.split_pend.disp_after
        };
        if disp_check.is_empty() {
            return true;
        }
        let mut tracked_union = self.bar.cs_soft_disp_hard_after;
        let mut tracked_req = [0u32; 7];
        merge_req_for_after(
            &mut tracked_req,
            self.bar.cs_soft_disp_hard_after,
            &self.bar.cs_soft_disp_hard_req,
        );
        for it in &self.bar.cs_soft_disp_items {
            if it.r == 0 {
                return false;
            }
            tracked_union |= it.after;
            merge_req_for_after(&mut tracked_req, it.after, &it.req);
        }
        if tracked_union != MTLStages::All && (tracked_union & disp_check) != disp_check {
            return false;
        }
        STAGE_BITS.iter().enumerate().all(|(pi, &b)| {
            !disp_check.contains(b) || tracked_req[pi] >= self.bar.split_pend.disp_req[pi]
        })
    }

    pub(super) fn partition_cs_soft_disp_for_open(
        &mut self,
        op_stage: MTLStages,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) {
        if !self.ps_to_cs_active()
            || !self.render_split_sync_active()
            || !self.ib_tight_active()
            || !self.mid_qb_stagesync_active()
            || self.is_copy_queue_light()
            || self.bar.needs_barrier
        {
            if !self.bar.enc_cs_soft_disp.is_empty() {
                self.flush_enc_cs_soft_disp_all();
            }
            return;
        }
        if op_stage != MTLStages::Dispatch {
            if !self.bar.enc_cs_soft_disp.is_empty() {
                self.flush_enc_cs_soft_disp_all();
            }
            return;
        }
        if self.bar.split_pend.disp_after.is_empty() {
            self.reset_cs_soft_disp();
            return;
        }
        if !self.cs_soft_disp_invariant_ok() {
            return;
        }
        let orig_disp = self.bar.split_pend.disp_after;
        let mut hit_after = self.bar.cs_soft_disp_hard_after;
        let mut hit_req = [0u32; 7];
        merge_req_for_after(
            &mut hit_req,
            self.bar.cs_soft_disp_hard_after,
            &self.bar.cs_soft_disp_hard_req,
        );
        let mut items = std::mem::take(&mut self.bar.cs_soft_disp_items);
        self.reset_cs_soft_disp();
        for it in items.drain(..) {
            if self.cs_op_may_touch_res(it.r, MTLStages::Dispatch, op_touch, exact) {
                hit_after |= it.after;
                merge_req_for_after(&mut hit_req, it.after, &it.req);
            } else {
                self.record_cs_disp_passthrough(it.r, it.after, it.req);
                if let Some(ex) = self.bar.enc_cs_soft_disp.iter_mut().find(|e| e.r == it.r) {
                    ex.after |= it.after;
                    merge_req_for_after(&mut ex.req, it.after, &it.req);
                } else {
                    self.bar.enc_cs_soft_disp.push(it);
                }
            }
        }
        self.bar.cs_soft_disp_items = items;
        self.bar.split_pend.disp_after = hit_after;
        self.bar.split_pend.disp_req = hit_req;
        self.sync_legacy_from_split();
        if !self.bar.enc_cs_soft_disp.is_empty() && hit_after != orig_disp {
            if self.opts.opt_ps_to_cs_hk_overlap {
                stats::inc(&stats::c::ps_to_cs_hk_overlap);
            } else if self.opts.opt_ps_to_cs_overlap_v2 {
                stats::inc(&stats::c::ps_to_cs_overlap_v2);
            } else {
                stats::inc(&stats::c::ps_to_cs_overlap);
            }
        }
    }

    pub(super) fn partition_cs_soft_disp_for_op(
        &mut self,
        op_stage: MTLStages,
        full: bool,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) {
        if full
            || !self.ps_to_cs_active()
            || !self.render_split_sync_active()
            || !self.ib_tight_active()
            || !self.mid_qb_stagesync_active()
            || self.is_copy_queue_light()
        {
            self.flush_enc_cs_soft_disp_all();
            return;
        }
        let cs_mask =
            if self.split_compute_exact_active() && !(op_stage & COMPUTE_STAGES).is_empty() {
                op_stage & COMPUTE_STAGES
            } else {
                COMPUTE_STAGES
            };
        if !cs_mask.contains(MTLStages::Dispatch) {
            return;
        }
        if op_stage != MTLStages::Dispatch {
            self.flush_enc_cs_soft_disp_all();
            return;
        }
        if self.bar.enc_cs_soft_disp.is_empty() && self.bar.split_pend.disp_after.is_empty() {
            self.reset_cs_soft_disp();
            return;
        }
        let mid_on = true;
        if !self.bar.split_pend.disp_after.is_empty() && !self.cs_soft_disp_invariant_ok() {
            if !self.bar.enc_cs_soft_disp.is_empty() {
                let mut prev = std::mem::take(&mut self.bar.enc_cs_soft_disp);
                let mut kept = std::mem::take(&mut self.bar.cs_soft_disp_scratch);
                kept.clear();
                for mut it in prev.drain(..) {
                    it.after = self.unsatisfied_enc_after_for_req(
                        it.after,
                        MTLStages::Dispatch,
                        &it.req,
                        mid_on,
                    );
                    if it.after.is_empty() {
                        continue;
                    }
                    if self.cs_op_may_touch_res(it.r, MTLStages::Dispatch, op_touch, exact) {
                        self.bar.split_pend.disp_after |= it.after;
                        merge_req_for_after(&mut self.bar.split_pend.disp_req, it.after, &it.req);
                        if !self.bar.pend_cs_poisoned[0] && it.r != 0 {
                            Self::push_cs_mid_obl_into(
                                &mut self.bar.pend_cs_items,
                                it.r,
                                it.after,
                                MTLStages::empty(),
                                MTLStages::empty(),
                                it.req,
                            );
                        } else {
                            self.bar.pend_cs_poisoned[0] = true;
                        }
                    } else {
                        kept.push(it);
                    }
                }
                self.bar.enc_cs_soft_disp = kept;
                self.bar.cs_soft_disp_scratch = prev;
                self.sync_legacy_from_split();
            }
            return;
        }
        let orig_disp = self.bar.split_pend.disp_after;
        let hard_u = self.unsatisfied_enc_after_for_req(
            self.bar.cs_soft_disp_hard_after,
            MTLStages::Dispatch,
            &self.bar.cs_soft_disp_hard_req,
            mid_on,
        );
        let mut hit_after = hard_u;
        let mut hit_req = [0u32; 7];
        merge_req_for_after(&mut hit_req, hard_u, &self.bar.cs_soft_disp_hard_req);
        let mut prev_enc = std::mem::take(&mut self.bar.enc_cs_soft_disp);
        let mut new_items = std::mem::take(&mut self.bar.cs_soft_disp_items);
        self.reset_cs_soft_disp();
        let mut deferred_any_new = false;
        let mut kept_enc = std::mem::take(&mut self.bar.cs_soft_disp_scratch);
        kept_enc.clear();
        let mut process_item = |this: &mut Self, it: CsSoftDispObl, is_new: bool| {
            let u = this.unsatisfied_enc_after_for_req(
                it.after,
                MTLStages::Dispatch,
                &it.req,
                mid_on,
            );
            if u.is_empty() {
                return;
            }
            if this.cs_op_may_touch_res(it.r, MTLStages::Dispatch, op_touch, exact) {
                hit_after |= u;
                merge_req_for_after(&mut hit_req, u, &it.req);
            } else {
                if is_new {
                    deferred_any_new = true;
                    this.record_cs_disp_passthrough(it.r, u, it.req);
                }
                if let Some(ex) = kept_enc.iter_mut().find(|e| e.r == it.r) {
                    ex.after |= u;
                    merge_req_for_after(&mut ex.req, u, &it.req);
                } else {
                    let mut ureq = [0u32; 7];
                    merge_req_for_after(&mut ureq, u, &it.req);
                    kept_enc.push(CsSoftDispObl {
                        r: it.r,
                        after: u,
                        req: ureq,
                    });
                }
            }
        };
        for it in prev_enc.drain(..) {
            process_item(self, it, false);
        }
        for it in new_items.drain(..) {
            process_item(self, it, true);
        }
        self.bar.enc_cs_soft_disp = kept_enc;
        self.bar.cs_soft_disp_scratch = prev_enc;
        self.bar.cs_soft_disp_items = new_items;
        self.bar.split_pend.disp_after = hit_after;
        self.bar.split_pend.disp_req = hit_req;
        self.bar.pend_cs_items.retain_mut(|item| {
            item.disp_after = MTLStages::empty();
            !(item.blit_after | item.as_after).is_empty()
        });
        self.bar.cs_mid_v3_items.retain_mut(|item| {
            item.disp_after = MTLStages::empty();
            !(item.blit_after | item.as_after).is_empty()
        });
        self.sync_legacy_from_split();
        if deferred_any_new && hit_after != orig_disp {
            if self.opts.opt_ps_to_cs_hk_overlap {
                stats::inc(&stats::c::ps_to_cs_hk_overlap);
            } else if self.opts.opt_ps_to_cs_overlap_v2 {
                stats::inc(&stats::c::ps_to_cs_overlap_v2);
            } else {
                stats::inc(&stats::c::ps_to_cs_overlap);
            }
        }
    }

    #[inline]
    pub(super) fn need_pre_open_cur_draw_exact(&self) -> bool {
        if !self.opts.opt_gfx_desc_table_exact {
            return false;
        }
        if self.opts.opt_cs_to_ps_overlap {
            return true;
        }
        if (self.opts.opt_cs_to_ps_overlap_v2
            || self.gfx_to_ps_overlap_active()
            || self.list_rsplit_v2_active())
            && self.has_any_soft_pre_obl()
        {
            return true;
        }
        (self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
            && self.bar.needs_barrier
            && self.bar.full_why == FullWhy::ListStart
            && !matches!(self.enc, Encoder::Render(_))
    }

    #[inline]
    pub(super) fn reset_frag_rt_pend(&mut self) {
        self.bar.frag_rt_pend.clear();
        self.bar.frag_non_rt_after = MTLStages::empty();
        self.bar.frag_non_rt_req = [0; 7];
        self.bar.frag_rt_poisoned = false;
    }

    #[inline]
    pub(super) fn push_frag_rt_pend(&mut self, r: usize, ia: MTLStages, req: &[u32; 7]) {
        if r == 0 || ia.is_empty() {
            return;
        }
        if let Some(ex) = self.bar.frag_rt_pend.iter_mut().find(|e| e.0 == r) {
            ex.1 |= ia;
            merge_req_for_after(&mut ex.2, ia, req);
        } else {
            let mut r_req = [0u32; 7];
            merge_req_for_after(&mut r_req, ia, req);
            self.bar.frag_rt_pend.push((r, ia, r_req));
        }
    }

    pub(super) fn consume_frag_rt_pend_for_clear(&mut self, targets: &[usize], cap_frag: MTLStages) {
        if !self.clear_isolate_active()
            || self.bar.frag_rt_poisoned
            || targets.is_empty()
            || self.bar.frag_rt_pend.is_empty()
        {
            return;
        }
        let mut check_union = self.bar.frag_non_rt_after;
        for &(_, ia, _) in &self.bar.frag_rt_pend {
            check_union |= ia;
        }
        if check_union != self.bar.split_pend.frag_after {
            return;
        }
        let mut consumed = false;
        self.bar.frag_rt_pend.retain(|&(r, ia, _)| {
            if targets.contains(&r) && (cap_frag == MTLStages::All || cap_frag.contains(ia)) {
                consumed = true;
                false
            } else {
                true
            }
        });
        if consumed {
            let mut new_after = self.bar.frag_non_rt_after;
            let mut new_req = [0u32; 7];
            merge_req_for_after(
                &mut new_req,
                self.bar.frag_non_rt_after,
                &self.bar.frag_non_rt_req,
            );
            for &(_, ia, ref ireq) in &self.bar.frag_rt_pend {
                new_after |= ia;
                merge_req_for_after(&mut new_req, ia, ireq);
            }
            self.bar.split_pend.frag_after = new_after;
            self.bar.split_pend.frag_req = new_req;
            self.sync_legacy_from_split();
        }
    }

    #[inline]
    pub(super) fn finish_post_draw0_list_frag(&mut self) {
        if self.bar.arm_post_draw0_list_frag == [false; 2] {
            return;
        }
        let arm = std::mem::replace(&mut self.bar.arm_post_draw0_list_frag, [false; 2]);
        for (i, &on) in arm.iter().enumerate() {
            if !on {
                continue;
            }
            self.bar.vtx_defer[i] = merge_vtx(self.bar.vtx_defer[i], Some((MTLStages::All, PRE_RASTER)));
            merge_req_for_after(
                &mut self.bar.vtx_defer_req[i],
                MTLStages::All,
                &self.bar.stage_gen,
            );
            if self.list_rsplit_v2_active() && !self.bar.vtx_defer_poisoned[i] {
                let mut req0 = [0u32; 7];
                merge_req_for_after(&mut req0, MTLStages::All, &self.bar.stage_gen);
                let sentinel = if i == 0 {
                    SOFT_RES_PRE_LIST_FRAG_TEX
                } else {
                    SOFT_RES_PRE_LIST_FRAG_BUF
                };
                Self::push_soft_obl_into(
                    &mut self.bar.vtx_defer_items[i],
                    sentinel,
                    MTLStages::All,
                    PRE_RASTER,
                    req0,
                );
            } else {
                self.bar.vtx_defer_poisoned[i] = true;
                self.bar.vtx_defer_items[i].clear();
            }
            self.bar.vtx_defer_list_frag = true;
        }
    }

    #[inline]
    pub(super) fn push_cs_mid_obl_into(
        list: &mut Vec<CsMidResObl>,
        r: usize,
        disp_after: MTLStages,
        blit_after: MTLStages,
        as_after: MTLStages,
        req: [u32; 7],
    ) {
        if r == 0 || (disp_after | blit_after | as_after).is_empty() {
            return;
        }
        if let Some(existing) = list.iter_mut().find(|e| e.r == r) {
            existing.disp_after |= disp_after;
            existing.blit_after |= blit_after;
            existing.as_after |= as_after;
            merge_req_for_after(&mut existing.req, disp_after | blit_after | as_after, &req);
        } else {
            let mut er = [0; 7];
            merge_req_for_after(&mut er, disp_after | blit_after | as_after, &req);
            list.push(CsMidResObl {
                r,
                disp_after,
                blit_after,
                as_after,
                req: er,
            });
        }
    }

    #[inline]
    fn strip_cs_mid_items(items: &mut Vec<CsMidResObl>, cs_mask: MTLStages) {
        items.retain_mut(|item| {
            if cs_mask.contains(MTLStages::Dispatch) {
                item.disp_after = MTLStages::empty();
            }
            if cs_mask.contains(MTLStages::Blit) {
                item.blit_after = MTLStages::empty();
            }
            if cs_mask.contains(MTLStages::AccelerationStructure) {
                item.as_after = MTLStages::empty();
            }
            !(item.disp_after | item.blit_after | item.as_after).is_empty()
        });
    }

    #[inline]
    pub(super) fn clear_cs_mid_v3_for(&mut self, cs_mask: MTLStages) {
        Self::strip_cs_mid_items(&mut self.bar.cs_mid_v3_items, cs_mask);
        if self.opts.opt_cs_mid_qb_unpoison_v4 {
            for &(ci, cbit) in &CS_SUB_BITS {
                if cs_mask.contains(cbit) {
                    self.bar.cs_mid_v3_poisoned[ci] = false;
                }
            }
        } else if !self.bar.split_pend.has_compute() && self.bar.cs_mid_v3_items.is_empty() {
            self.bar.cs_mid_v3_poisoned = [false; 3];
        }
    }

    #[inline]
    pub(super) fn clear_pend_cs_for(&mut self, cs_mask: MTLStages) {
        Self::strip_cs_mid_items(&mut self.bar.pend_cs_items, cs_mask);
        for &(ci, cbit) in &CS_SUB_BITS {
            if cs_mask.contains(cbit) {
                self.bar.pend_cs_poisoned[ci] = false;
            }
        }
    }

    #[inline]
    pub(super) fn cs_op_may_touch_res(
        &self,
        r: usize,
        op_stage: MTLStages,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) -> bool {
        if r == 0 {
            return true;
        }
        let rc = res_class(r);
        let ro_exact = self.opts.opt_cs_mid_qb_exact_v2
            || self.opts.opt_sm66_cs_stage_exact
            || self.hk_v2_active();
        let mut op_classes = READS_BUF | READS_TEX;
        if op_stage == MTLStages::Dispatch {
            if let Some(ex) = exact {
                if !ex.touches_textures {
                    op_classes &= !READS_TEX;
                }
                if ro_exact {
                    op_classes &= ex.uav_classes | ex.exact_read_only_classes;
                } else if self.msc_class_active() {
                    op_classes &= ex.uav_classes | ex.read_classes;
                }
            }
        }
        if (op_classes & rc) == 0 {
            return false;
        }
        if let Some(touch) = op_touch {
            return touch.contains(&r);
        }
        if op_stage == MTLStages::Dispatch {
            if let Some(ex) = exact {
                let uav_hit =
                    (ex.uav_classes & rc) != 0 && (!ex.uav_exact || ex.uavs.contains(&r));
                let ro_classes = if ro_exact {
                    ex.exact_read_only_classes
                } else {
                    ex.read_classes
                };
                let read_exact_for_rc = ex.full_read_exact
                    || (ex.bounded_read_exact && (ex.unbounded_srv_classes & rc) == 0);
                let read_hit = (ro_classes & rc) != 0
                    && (!read_exact_for_rc || ex.bounded_reads.contains(&r));
                let hit = uav_hit || read_hit;
                if hit && self.hk_any_active() && r != 0 {
                    const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                    let res = unsafe { &*(r as *const Resource) };
                    if res.desc.Flags & SIMUL == 0 {
                        let rhk = res.handle_kind_mask();
                        let ro_hk = if self.hk_v2_active()
                            && ex.bounded_read_exact
                            && !ex.bounded_reads.contains(&r)
                        {
                            ex.hk.unbounded_srv_kinds
                                | if (ex.unbounded_srv_classes & READS_BUF) != 0 {
                                    ex.hk.read_kinds & HK_CBV_BUF
                                } else {
                                    0
                                }
                        } else if ro_exact {
                            ex.hk.ro_srv_kinds
                                | if (ro_classes & READS_BUF) != 0 {
                                    ex.hk.read_kinds & HK_CBV_BUF
                                } else {
                                    0
                                }
                        } else {
                            ex.hk.read_kinds
                        };
                        let hk_uav = uav_hit && (ex.hk.uav_srv_kinds & rhk) != 0;
                        let hk_read = read_hit && (ro_hk & rhk) != 0;
                        if !hk_uav && !hk_read {
                            self.bump_hk_stat();
                            return false;
                        }
                    }
                }
                return hit;
            }
        }
        true
    }

    /// Return the subset of `ran` stages of the closing render pass that could legally have
    /// accessed `kw.r` before/across `kw` (`AW2_RP_KEEP_WRITE_EXACT=1`).
    #[inline]
    pub(super) fn rp_kept_item_touch_stages(
        &self,
        kw: &KeptWriteItem,
        ran: MTLStages,
        had_uav_gfx: bool,
    ) -> MTLStages {
        let mut cur = if !self.opts.opt_ps_srv_class_exact {
            self.rp_kept_item_touch_stages_with(kw, ran, had_uav_gfx, false)
        } else if !stats::active() {
            self.rp_kept_item_touch_stages_with(kw, ran, had_uav_gfx, true)
        } else {
            let legacy = self.rp_kept_item_touch_stages_with(kw, ran, had_uav_gfx, false);
            let exact = self.rp_kept_item_touch_stages_with(kw, ran, had_uav_gfx, true);
            if exact != legacy {
                stats::inc(&stats::c::ps_srv_class_exact);
            }
            exact
        };
        if self.opts.opt_gfx_desc_table_exact
            && kw.r != 0
            && !cur.is_empty()
            && !self.rp_sync.enc_sync_targets.contains(&kw.r)
        {
            const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
            if unsafe { (*(kw.r as *const Resource)).desc.Flags & SIMUL == 0 } {
                let rc = res_class(kw.r);
                let mut narrowed = cur;
                if narrowed.intersects(PRE_RASTER)
                    && (self.rp_sync.enc_pre_raster_unbounded_classes & rc) == 0
                    && !self.rp_sync.enc_pre_raster_exact_res.contains(&kw.r)
                {
                    narrowed -= PRE_RASTER;
                }
                if narrowed.contains(MTLStages::Fragment)
                    && (self.rp_sync.enc_ps_unbounded_classes & rc) == 0
                    && !self.rp_sync.enc_ps_exact_res.contains(&kw.r)
                {
                    narrowed -= MTLStages::Fragment;
                }
                if narrowed != cur {
                    stats::inc(&stats::c::gfx_desc_table_exact);
                    cur = narrowed;
                }
            }
        }
        if self.hk_any_active()
            && kw.r != 0
            && !cur.is_empty()
            && !self.rp_sync.enc_sync_targets.contains(&kw.r)
        {
            const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
            let res = unsafe { &*(kw.r as *const Resource) };
            if res.desc.Flags & SIMUL == 0 {
                let rhk = res.handle_kind_mask();
                let rc = res_class(kw.r);
                let no_common_buf_promo = !(kw.eff_s1 == 0
                    && rc == READS_BUF
                    && self.rp_sync.rp_kept_later_draw);
                let eff_pre_hk = if self.hk_v2_active()
                    && self.opts.opt_gfx_desc_table_exact
                    && !self.rp_sync.enc_pre_raster_exact_res.contains(&kw.r)
                {
                    self.rp_sync.enc_pre_raster_unb_hk
                } else {
                    self.rp_sync.enc_pre_raster_hk
                };
                let eff_ps_hk = if self.hk_v2_active()
                    && self.opts.opt_gfx_desc_table_exact
                    && !self.rp_sync.enc_ps_exact_res.contains(&kw.r)
                {
                    self.rp_sync.enc_ps_unb_hk
                } else {
                    self.rp_sync.enc_ps_hk
                };
                let mut narrowed = cur;
                if narrowed.intersects(PRE_RASTER)
                    && (kw.is_uav_barrier
                        || (kw.s0 != 0
                            && no_common_buf_promo
                            && (kw.eff_s0 & !(0x8 | 0x40 | 0x80)) == 0))
                    && (eff_pre_hk & rhk) == 0
                {
                    narrowed -= PRE_RASTER;
                }
                if narrowed.contains(MTLStages::Fragment)
                    && (kw.is_uav_barrier
                        || (kw.s0 != 0 && (kw.eff_s0 & !(0x1 | 0x8 | 0x40 | 0x80)) == 0))
                    && (eff_ps_hk & rhk) == 0
                {
                    narrowed -= MTLStages::Fragment;
                }
                if narrowed != cur {
                    self.bump_hk_stat();
                    cur = narrowed;
                }
            }
        }
        cur
    }

    #[inline]
    fn rp_kept_item_touch_stages_with(
        &self,
        kw: &KeptWriteItem,
        ran: MTLStages,
        had_uav_gfx: bool,
        srv_uav_exact: bool,
    ) -> MTLStages {
        let uav_active = had_uav_gfx || self.rp_sync.enc_uav_used;
        let render_ran = ran & (PRE_RASTER | MTLStages::Fragment);
        if render_ran.is_empty() {
            return MTLStages::empty();
        }
        if kw.is_uav_barrier {
            if !uav_active {
                return MTLStages::empty();
            }
            if kw.r == 0 {
                return render_ran;
            }
            let rc = res_class(kw.r);
            let pre_m = if srv_uav_exact {
                self.rp_sync.enc_pre_raster_srv_uav_reads
            } else {
                self.rp_sync.enc_pre_raster_reads
            };
            let ps_m = if srv_uav_exact {
                self.rp_sync.enc_ps_srv_uav_reads
            } else {
                self.rp_sync.enc_ps_reads
            };
            let mut touch = MTLStages::empty();
            if (pre_m & rc) != 0 {
                touch |= render_ran & PRE_RASTER;
            }
            if (ps_m & rc) != 0 {
                touch |= render_ran & MTLStages::Fragment;
            }
            return touch;
        }
        if kw.r == 0 {
            return render_ran;
        }
        let rc = res_class(kw.r);
        let attached = self.rp_sync.enc_sync_targets.contains(&kw.r);
        if kw.s0 == 0 {
            let mut touch = MTLStages::empty();
            if (self.rp_sync.enc_pre_raster_reads & rc) != 0 {
                touch |= render_ran & PRE_RASTER;
            }
            if attached || (self.rp_sync.enc_ps_reads & rc) != 0 {
                touch |= render_ran & MTLStages::Fragment;
            }
            return touch;
        }
        let pre_m = if srv_uav_exact && (kw.eff_s0 & (0x1 | 0x2 | 0x200)) == 0 {
            self.rp_sync.enc_pre_raster_srv_uav_reads
        } else {
            self.rp_sync.enc_pre_raster_reads
        };
        let ps_m = if srv_uav_exact && (kw.eff_s0 & 0x1) == 0 {
            self.rp_sync.enc_ps_srv_uav_reads
        } else {
            self.rp_sync.enc_ps_reads
        };
        let s0_st = state_stages(kw.eff_s0);
        let mut touch = MTLStages::empty();
        if s0_st.intersects(PRE_RASTER)
            && (pre_m & rc) != 0
            && ((kw.eff_s0 & !0x8) != 0 || uav_active)
        {
            touch |= render_ran & PRE_RASTER;
        }
        if s0_st.contains(MTLStages::Fragment) {
            let frag_shader_state = (kw.eff_s0 & (0x1 | 0x8 | 0x80)) != 0;
            let frag_hit = attached
                || (frag_shader_state
                    && (ps_m & rc) != 0
                    && ((kw.eff_s0 & !0x8) != 0 || uav_active));
            if frag_hit {
                touch |= render_ran & MTLStages::Fragment;
            }
        }
        if srv_uav_exact
            && kw.eff_s1 == 0
            && rc == READS_BUF
            && self.rp_sync.rp_kept_later_draw
        {
            if (self.rp_sync.enc_pre_raster_reads & rc) != 0 {
                touch |= render_ran & PRE_RASTER;
            }
            if (self.rp_sync.enc_ps_reads & rc) != 0 {
                touch |= render_ran & MTLStages::Fragment;
            }
        }
        if uav_active && (kw.eff_s1 == 0 || (kw.eff_s1 & 0x8) != 0) {
            let uav_pre = if srv_uav_exact {
                self.rp_sync.enc_pre_raster_srv_uav_reads
            } else {
                self.rp_sync.enc_pre_raster_reads
            };
            let uav_ps = if srv_uav_exact {
                self.rp_sync.enc_ps_srv_uav_reads
            } else {
                self.rp_sync.enc_ps_reads
            };
            if (uav_pre & rc) != 0 {
                touch |= render_ran & PRE_RASTER;
            }
            if (uav_ps & rc) != 0 {
                touch |= render_ran & MTLStages::Fragment;
            }
        }
        touch
    }

    #[inline]
    pub(super) fn gfx_pso_writes_uav(&self, p: &GraphicsPipeline) -> bool {
        if self.opts.opt_sm66_gfx_stage_exact {
            p.sm66_writes_uav
        } else {
            p.writes_uav
        }
    }

    #[inline]
    pub(super) fn mesh_pso_writes_uav(&self, p: &crate::mesh::MeshPipeline) -> bool {
        if self.opts.opt_sm66_gfx_stage_exact {
            p.sm66_writes_uav
        } else {
            p.writes_uav
        }
    }

    #[inline]
    pub(super) fn gfx_pso_srv_uav_pre_raster_reads(&self, p: &GraphicsPipeline) -> u8 {
        if self.opts.opt_sm66_gfx_stage_exact {
            p.sm66_pre_raster_reads
        } else {
            p.pre_raster_reads
        }
    }

    #[inline]
    pub(super) fn effective_gfx_pre_raster_reads(
        &self,
        p: &GraphicsPipeline,
        indexed: bool,
        indirect: bool,
    ) -> u8 {
        let (base_reads, vs_cbv_or_ia) = if self.opts.opt_sm66_gfx_stage_exact {
            if p.sm66_narrowed {
                stats::inc(&stats::c::sm66_gfx_exact);
            }
            (p.sm66_pre_raster_reads, p.sm66_vs_reads_cbv_or_ia)
        } else {
            (p.pre_raster_reads, p.vs_reads_cbv_or_ia)
        };
        if self.vs_buf_defer_exact_active()
            && (vs_cbv_or_ia
                || indexed
                || indirect
                || ((self.opts.opt_cs_to_ps_overlap
                    || self.opts.opt_cs_to_ps_overlap_v2
                    || self.gfx_to_ps_overlap_active())
                    && self.predication.is_some()))
        {
            base_reads | READS_BUF
        } else {
            base_reads
        }
    }

    #[inline]
    pub(super) fn flush_cs_end_after(&mut self) {
        if self.bar.cs_end_after == [MTLStages::empty(); 3] && self.bar.cs_end_res.is_empty() {
            return;
        }
        let ea = std::mem::replace(&mut self.bar.cs_end_after, [MTLStages::empty(); 3]);
        let req = self.cur_producer_gen();
        for &(ci, cbit) in &CS_SUB_BITS {
            if !ea[ci].is_empty() {
                self.bar
                    .split_pend
                    .add_with_req(ea[ci], cbit, [MTLStages::empty(); 2], req);
                if self.ps_to_cs_active() && cbit == MTLStages::Dispatch {
                    self.record_cs_soft_disp(0, ea[ci], req);
                }
                self.bar.pend_cs_poisoned[ci] = true;
            }
        }
        for (r, da, ba, aa) in std::mem::take(&mut self.bar.cs_end_res) {
            if !da.is_empty() {
                self.bar
                    .split_pend
                    .add_with_req(da, MTLStages::Dispatch, [MTLStages::empty(); 2], req);
                if self.ps_to_cs_active() {
                    self.record_cs_soft_disp(r, da, req);
                }
            }
            if !ba.is_empty() {
                self.bar
                    .split_pend
                    .add_with_req(ba, MTLStages::Blit, [MTLStages::empty(); 2], req);
            }
            if !aa.is_empty() {
                self.bar.split_pend.add_with_req(
                    aa,
                    MTLStages::AccelerationStructure,
                    [MTLStages::empty(); 2],
                    req,
                );
            }
            if r != 0 {
                Self::push_cs_mid_obl_into(&mut self.bar.pend_cs_items, r, da, ba, aa, req);
            } else {
                if !da.is_empty() {
                    self.bar.pend_cs_poisoned[0] = true;
                }
                if !ba.is_empty() {
                    self.bar.pend_cs_poisoned[1] = true;
                }
                if !aa.is_empty() {
                    self.bar.pend_cs_poisoned[2] = true;
                }
            }
        }
        self.sync_legacy_from_split();
    }

    #[inline]
    pub(super) fn push_cs_mid_res_obl(
        &mut self,
        r: usize,
        disp_after: MTLStages,
        blit_after: MTLStages,
        as_after: MTLStages,
        req: [u32; 7],
    ) {
        Self::push_cs_mid_obl_into(
            &mut self.bar.cs_mid_res_pend,
            r,
            disp_after,
            blit_after,
            as_after,
            req,
        );
    }

    #[inline]
    pub(super) fn flush_cs_mid_res_all(&mut self) {
        if self.bar.cs_mid_res_pend.is_empty() {
            return;
        }
        for e in std::mem::take(&mut self.bar.cs_mid_res_pend) {
            if !e.disp_after.is_empty() {
                self.bar.split_pend.add_with_req(
                    e.disp_after,
                    MTLStages::Dispatch,
                    [MTLStages::empty(); 2],
                    e.req,
                );
                if self.ps_to_cs_active() {
                    self.record_cs_soft_disp(e.r, e.disp_after, e.req);
                }
            }
            if !e.blit_after.is_empty() {
                self.bar.split_pend.add_with_req(
                    e.blit_after,
                    MTLStages::Blit,
                    [MTLStages::empty(); 2],
                    e.req,
                );
            }
            if !e.as_after.is_empty() {
                self.bar.split_pend.add_with_req(
                    e.as_after,
                    MTLStages::AccelerationStructure,
                    [MTLStages::empty(); 2],
                    e.req,
                );
            }
            if e.r != 0 {
                Self::push_cs_mid_obl_into(
                    &mut self.bar.pend_cs_items,
                    e.r,
                    e.disp_after,
                    e.blit_after,
                    e.as_after,
                    e.req,
                );
            } else {
                if !e.disp_after.is_empty() {
                    self.bar.pend_cs_poisoned[0] = true;
                }
                if !e.blit_after.is_empty() {
                    self.bar.pend_cs_poisoned[1] = true;
                }
                if !e.as_after.is_empty() {
                    self.bar.pend_cs_poisoned[2] = true;
                }
            }
        }
        self.sync_legacy_from_split();
    }

    /// Flush `cs_mid_res_pend` obligations matching `op_touch` / `exact` into `self.bar.split_pend`
    /// right before `compute_mid_queue_sync`, incrementing `cs_mid_qb_defer` when an
    /// unsatisfied obligation for `op_stage` stays deferred.
    pub(super) fn flush_cs_mid_res_for_op(
        &mut self,
        op_stage: MTLStages,
        full: bool,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) {
        if self.bar.cs_mid_res_pend.is_empty() {
            return;
        }
        if full {
            self.flush_cs_mid_res_all();
            return;
        }
        let split_cs_exact = self.split_compute_exact_active();
        let cs_mask = if split_cs_exact && !(op_stage & COMPUTE_STAGES).is_empty() {
            op_stage & COMPUTE_STAGES
        } else {
            COMPUTE_STAGES
        };
        let mut op_classes = READS_BUF | READS_TEX;
        if op_stage == MTLStages::Dispatch {
            if let Some(ex) = exact {
                if !ex.touches_textures {
                    op_classes &= !READS_TEX;
                }
                if self.opts.opt_cs_mid_qb_exact_v2 {
                    op_classes &= ex.uav_classes | ex.exact_read_only_classes;
                } else if self.msc_class_active() {
                    op_classes &= ex.uav_classes | ex.read_classes;
                }
            }
        }
        let mut flushed_any = false;
        let mut kept_unsatisfied = false;
        let mid_on = self.mid_qb_stagesync_active();
        for i in (0..self.bar.cs_mid_res_pend.len()).rev() {
            let e_copy = self.bar.cs_mid_res_pend[i];
            let mut ea = MTLStages::empty();
            let mut eb = MTLStages::empty();
            if cs_mask.contains(MTLStages::Dispatch) && !e_copy.disp_after.is_empty() {
                ea |= e_copy.disp_after;
                eb |= MTLStages::Dispatch;
            }
            if cs_mask.contains(MTLStages::Blit) && !e_copy.blit_after.is_empty() {
                ea |= e_copy.blit_after;
                eb |= MTLStages::Blit;
            }
            if cs_mask.contains(MTLStages::AccelerationStructure) && !e_copy.as_after.is_empty() {
                ea |= e_copy.as_after;
                eb |= MTLStages::AccelerationStructure;
            }
            if ea.is_empty() {
                continue;
            }
            let may_touch = if (op_classes & res_class(e_copy.r)) == 0 {
                false
            } else if let Some(touch) = op_touch {
                touch.contains(&e_copy.r)
            } else if self.opts.opt_cs_mid_qb_exact_v2 && op_stage == MTLStages::Dispatch {
                if let Some(ex) = exact {
                    let rc = res_class(e_copy.r);
                    let uav_hit = (ex.uav_classes & rc) != 0
                        && (!ex.uav_exact || ex.uavs.contains(&e_copy.r));
                    let read_exact_rc = ex.full_read_exact
                        || ((self.opts.opt_desc_table_res_exact
                            || self.opts.opt_desc_table_cs_fast
                            || self.cs_intra_safe_active()
                            || self.hk_v2_active())
                            && ex.bounded_read_exact
                            && (ex.unbounded_srv_classes & rc) == 0);
                    let read_hit = (ex.exact_read_only_classes & rc) != 0
                        && (!read_exact_rc || ex.bounded_reads.contains(&e_copy.r));
                    if !uav_hit
                        && !read_hit
                        && !ex.full_read_exact
                        && (ex.exact_read_only_classes & rc) != 0
                    {
                        if self.cs_intra_safe_active() {
                            self.bump_cs_intra_stat();
                        } else if self.hk_v2_active() {
                            self.bump_hk_stat();
                        } else if self.opts.opt_desc_table_res_exact {
                            stats::inc(&stats::c::desc_table_cs_exact);
                        } else {
                            stats::inc(&stats::c::desc_table_cs_fast);
                        }
                    }
                    let mut hit = uav_hit || read_hit;
                    if hit && self.hk_any_active() && e_copy.r != 0 {
                        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                        let res = unsafe { &*(e_copy.r as *const Resource) };
                        if res.desc.Flags & SIMUL == 0 {
                            let rhk = res.handle_kind_mask();
                            let ro_hk = if self.hk_v2_active()
                                && ex.bounded_read_exact
                                && !ex.bounded_reads.contains(&e_copy.r)
                            {
                                ex.hk.unbounded_srv_kinds
                                    | if (ex.unbounded_srv_classes & READS_BUF) != 0 {
                                        ex.hk.read_kinds & HK_CBV_BUF
                                    } else {
                                        0
                                    }
                            } else {
                                ex.hk.ro_srv_kinds
                                    | if (ex.exact_read_only_classes & READS_BUF) != 0 {
                                        ex.hk.read_kinds & HK_CBV_BUF
                                    } else {
                                        0
                                    }
                            };
                            let hk_uav = uav_hit && (ex.hk.uav_srv_kinds & rhk) != 0;
                            let hk_read = read_hit && (ro_hk & rhk) != 0;
                            if !hk_uav && !hk_read {
                                self.bump_hk_stat();
                                hit = false;
                            }
                        }
                    }
                    hit
                } else {
                    true
                }
            } else {
                true
            };
            if may_touch {
                let mut f_da = MTLStages::empty();
                let mut f_ba = MTLStages::empty();
                let mut f_aa = MTLStages::empty();
                let f_req;
                let remove_entry;
                {
                    let entry = &mut self.bar.cs_mid_res_pend[i];
                    f_req = entry.req;
                    if cs_mask.contains(MTLStages::Dispatch) && !entry.disp_after.is_empty() {
                        f_da = std::mem::replace(&mut entry.disp_after, MTLStages::empty());
                        self.bar.split_pend.add_with_req(
                            f_da,
                            MTLStages::Dispatch,
                            [MTLStages::empty(); 2],
                            f_req,
                        );
                        flushed_any = true;
                    }
                    if cs_mask.contains(MTLStages::Blit) && !entry.blit_after.is_empty() {
                        f_ba = std::mem::replace(&mut entry.blit_after, MTLStages::empty());
                        self.bar.split_pend.add_with_req(
                            f_ba,
                            MTLStages::Blit,
                            [MTLStages::empty(); 2],
                            f_req,
                        );
                        flushed_any = true;
                    }
                    if cs_mask.contains(MTLStages::AccelerationStructure)
                        && !entry.as_after.is_empty()
                    {
                        f_aa = std::mem::replace(&mut entry.as_after, MTLStages::empty());
                        self.bar.split_pend.add_with_req(
                            f_aa,
                            MTLStages::AccelerationStructure,
                            [MTLStages::empty(); 2],
                            f_req,
                        );
                        flushed_any = true;
                    }
                    remove_entry = (entry.disp_after | entry.blit_after | entry.as_after).is_empty();
                }
                if remove_entry {
                    self.bar.cs_mid_res_pend.swap_remove(i);
                }
                if self.ps_to_cs_active() && !f_da.is_empty() {
                    self.record_cs_soft_disp(e_copy.r, f_da, f_req);
                }
                if e_copy.r != 0 {
                    Self::push_cs_mid_obl_into(
                        &mut self.bar.pend_cs_items,
                        e_copy.r,
                        f_da,
                        f_ba,
                        f_aa,
                        f_req,
                    );
                } else {
                    if !f_da.is_empty() {
                        self.bar.pend_cs_poisoned[0] = true;
                    }
                    if !f_ba.is_empty() {
                        self.bar.pend_cs_poisoned[1] = true;
                    }
                    if !f_aa.is_empty() {
                        self.bar.pend_cs_poisoned[2] = true;
                    }
                }
            } else if !kept_unsatisfied
                && !self
                    .unsatisfied_enc_after_for_req(ea, eb, &e_copy.req, mid_on)
                    .is_empty()
            {
                kept_unsatisfied = true;
            }
        }
        if flushed_any {
            self.sync_legacy_from_split();
        }
        if kept_unsatisfied && !self.bar.split_pend.has_compute_for(cs_mask) {
            stats::inc(&stats::c::cs_mid_qb_defer);
            if self.opts.opt_cs_mid_qb_exact_v2 {
                stats::inc(&stats::c::cs_mid_qb_v2);
            }
        }
    }

    #[inline]
    pub(super) fn cur_producer_gen(&self) -> [u32; 7] {
        let mut g = self.bar.stage_gen;
        for (pi, &bit) in STAGE_BITS.iter().enumerate() {
            if self.bar.enc_ran_stages.contains(bit) {
                g[pi] = g[pi].wrapping_add(1);
            }
        }
        g
    }

    #[inline]
    pub(super) fn in_list_pre_list_all_anchor(&self) -> Option<MTLStages> {
        // Pick the closed in-list stage that already waited on `pre_list_all` with the
        // smallest generation count (tie-broken to prefer lightweight stages `Blit`, `AS`,
        // `Vertex` over heavy `Dispatch`/`Fragment` stages so independent encoders overlap).
        const ORDER: [(usize, MTLStages); 7] = [
            (5, MTLStages::Blit),
            (STAGE_ACCEL, MTLStages::AccelerationStructure),
            (0, MTLStages::Vertex),
            (1, MTLStages::Object),
            (2, MTLStages::Mesh),
            (STAGE_DISPATCH, MTLStages::Dispatch),
            (STAGE_FRAGMENT, MTLStages::Fragment),
        ];
        let mut best: Option<(u32, MTLStages)> = None;
        for &(i, bit) in &ORDER {
            let g = self.bar.stage_gen[i];
            if g > 0
                && self.bar.stage_sync[i].pre_list_all
                && best.is_none_or(|(bg, _)| g < bg)
            {
                best = Some((g, bit));
            }
        }
        best.map(|(_, bit)| bit)
    }

    #[inline]
    pub(super) fn mask_has_in_list_pre_list_all(&self, m: MTLStages) -> bool {
        m == MTLStages::All
            || (0..7).any(|i| {
                m.contains(STAGE_BITS[i])
                    && self.bar.stage_gen[i] > 0
                    && self.bar.stage_sync[i].pre_list_all
            })
    }

    #[inline]
    pub(super) fn can_narrow_uav_to_cs(&self) -> bool {
        self.opts.opt_cs_uav_producer_exact
            && !self.is_copy_queue_light()
            && !legacy_barriers()
            && barrier_masks()
            && !self.bar.needs_barrier
            && !self.bar.list_unwaited_cs
            && (!self.bar.list_unwaited_gfx
                || (self.opts.opt_list_start_substage
                    && !self.bar.enc_blit_only_sync
                    && !self.bar.list_cs_unwaited_frag))
            && self.bar.gfx_uav_gen == 0
            && !self.bar.uav_gfx_dirty
            && !self.gfx.pass_uav
            && !self.bar.enc_had_uav_gfx
            && ((self.bar.stage_gen[STAGE_DISPATCH] > 0
                && self.bar.stage_sync[STAGE_DISPATCH].pre_list_all)
                || (self.open_compute_disp_uav_synced()
                    && self.bar.enc_ran_stages.contains(MTLStages::Dispatch)))
    }

    #[inline]
    pub(super) fn pt_for_transition(pt: &[(usize, MTLStages)], r: usize) -> MTLStages {
        let mut out = MTLStages::empty();
        for &(rk, st) in pt {
            if r == 0 || rk == 0 || rk == r {
                out |= st;
            }
        }
        out
    }

    #[inline]
    pub(super) fn pt_for_uav(
        pt: &[(usize, MTLStages)],
        clear_tex_cs: &[usize],
        r: usize,
    ) -> MTLStages {
        let mut out = MTLStages::empty();
        for &(rk, st) in pt {
            if rk == 0 || (r != 0 && rk == r && !clear_tex_cs.contains(&r)) {
                out |= st;
            }
        }
        out
    }

    #[inline]
    pub(super) fn pt_insert(pt: &mut Vec<(usize, MTLStages)>, r: usize, stages: MTLStages) {
        let s = stages - MTLStages::Dispatch;
        if s.is_empty() {
            return;
        }
        if let Some(entry) = pt.iter_mut().find(|(rk, _)| *rk == r) {
            entry.1 |= s;
        } else {
            pt.push((r, s));
        }
    }

    #[inline]
    pub(super) fn pt_remove(pt: &mut Vec<(usize, MTLStages)>, r: usize) {
        if r != 0 {
            pt.retain(|&(rk, _)| rk != r);
        }
    }

    #[cfg(test)]
    #[inline]
    pub(super) fn uav_passthrough_for_transition(&self, r: usize) -> MTLStages {
        Self::pt_for_transition(&self.bar.uav_passthrough, r)
    }

    #[inline]
    pub(super) fn uav_passthrough_for_uav(&self, r: usize) -> MTLStages {
        Self::pt_for_uav(&self.bar.uav_passthrough, &self.bar.clear_tex_cs, r)
    }

    #[inline]
    pub(super) fn cs_uav_base_stages(&self) -> MTLStages {
        MTLStages::Dispatch
            | if self.bar.cs_as_gen > 0
                || self
                    .bar
                    .enc_ran_stages
                    .contains(MTLStages::AccelerationStructure)
            {
                MTLStages::AccelerationStructure
            } else {
                MTLStages::empty()
            }
    }

    #[inline]
    pub(super) fn source_stages_for_transition_with(
        &self,
        r: usize,
        eff_s0: u32,
        s0: u32,
        count_stat: bool,
        pt: &[(usize, MTLStages)],
    ) -> MTLStages {
        use MTLStages as S;
        if s0 == 0 {
            return S::All;
        }
        if eff_s0 == 0 {
            return S::empty();
        }
        let mut full_st = strip_nonbuf_accel(r, state_stages(eff_s0));
        let disp_pt = if self.ps_to_cs_active()
            && !self.bar.cs_disp_passthrough.is_empty()
            && full_st.contains(S::Dispatch)
        {
            strip_nonbuf_accel(r, self.cs_disp_passthrough_for(r))
        } else {
            S::empty()
        };
        full_st |= disp_pt;
        if (eff_s0 & 0x8) != 0
            && self.can_narrow_uav_to_cs()
            && (r == 0 || !self.clears.pending_clears.iter().any(|p| p.view.resource == r))
        {
            let non_uav = eff_s0 & !0x8;
            let clear_frag = if r == 0 {
                if self.bar.clear_tex_cs.is_empty() {
                    S::empty()
                } else {
                    S::Fragment
                }
            } else if self.bar.clear_tex_cs.contains(&r) {
                S::Fragment
            } else {
                S::empty()
            };
            let narrowed = strip_nonbuf_accel(
                r,
                (if non_uav != 0 {
                    state_stages(non_uav)
                } else {
                    S::empty()
                }) | self.cs_uav_base_stages()
                    | clear_frag
                    | Self::pt_for_transition(pt, r)
                    | disp_pt,
            );
            if count_stat
                && (full_st & (S::Fragment | PRE_RASTER))
                    != (narrowed & (S::Fragment | PRE_RASTER))
            {
                stats::inc(&stats::c::cs_uav_prod_exact);
            }
            return narrowed;
        }
        full_st
    }

    #[cfg(test)]
    #[inline]
    pub(super) fn source_stages_for_transition(
        &self,
        r: usize,
        eff_s0: u32,
        s0: u32,
        count_stat: bool,
    ) -> MTLStages {
        self.source_stages_for_transition_with(r, eff_s0, s0, count_stat, &self.bar.uav_passthrough)
    }

    #[inline]
    pub(super) fn source_stages_for_uav_with(
        &self,
        r: usize,
        shader: MTLStages,
        count_stat: bool,
        pt: &[(usize, MTLStages)],
    ) -> MTLStages {
        use MTLStages as S;
        let full_shader = strip_nonbuf_accel(r, shader);
        if self.can_narrow_uav_to_cs()
            && (r == 0 || !self.clears.pending_clears.iter().any(|p| p.view.resource == r))
        {
            let disp_pt = if self.ps_to_cs_active() && !self.bar.cs_disp_passthrough.is_empty() {
                strip_nonbuf_accel(r, self.cs_disp_passthrough_for(r))
            } else {
                S::empty()
            };
            let narrowed = strip_nonbuf_accel(
                r,
                self.cs_uav_base_stages()
                    | Self::pt_for_uav(pt, &self.bar.clear_tex_cs, r)
                    | disp_pt,
            );
            if count_stat
                && (full_shader & (S::Fragment | PRE_RASTER))
                    != (narrowed & (S::Fragment | PRE_RASTER))
            {
                stats::inc(&stats::c::cs_uav_prod_exact);
            }
            return narrowed;
        }
        full_shader
    }

    #[inline]
    pub(super) fn add_pend_with_soft(
        &mut self,
        after: MTLStages,
        before: MTLStages,
        soft: [MTLStages; 2],
    ) {
        if after.is_empty() || (before | soft[0] | soft[1]).is_empty() {
            return;
        }
        if before.intersects(COMPUTE_STAGES) {
            if matches!(self.enc, Encoder::Compute(_)) {
                let mid_on = self.mid_qb_stagesync_active();
                for &(ci, cbit) in &CS_SUB_BITS {
                    if before.contains(cbit)
                        && !self
                            .unsatisfied_enc_after_for(after, cbit, mid_on)
                            .is_empty()
                    {
                        self.bar.pend_cs_poisoned[ci] = true;
                        if self.opts.opt_cs_mid_qb_unpoison_v4 {
                            self.bar.cs_mid_v3_poisoned[ci] = true;
                        }
                    }
                }
                if !self.opts.opt_cs_mid_qb_unpoison_v4 {
                    self.bar.cs_mid_v3_poisoned = [true; 3];
                }
            } else {
                for &(ci, cbit) in &CS_SUB_BITS {
                    if before.contains(cbit) {
                        self.bar.pend_cs_poisoned[ci] = true;
                    }
                }
            }
        }
        for i in 0..2 {
            if !soft[i].is_empty() {
                self.bar.soft_pend_poisoned[i] = true;
                self.bar.soft_pend_items[i].clear();
            }
        }
        self.bar.pend_after |= after;
        self.bar.pend_before |= before;
        self.bar.pend_before_soft[0] |= soft[0];
        self.bar.pend_before_soft[1] |= soft[1];
        let req = self.cur_producer_gen();
        if self.clear_isolate_active() && before.contains(MTLStages::Fragment) {
            if self.opts.opt_clear_zero_qb_coalesce {
                self.bar.frag_rt_poisoned = true;
                self.bar.frag_rt_pend.clear();
            } else {
                self.bar.frag_non_rt_after |= after;
                merge_req_for_after(&mut self.bar.frag_non_rt_req, after, &req);
            }
        }
        if self.ps_to_cs_active() && before.contains(MTLStages::Dispatch) {
            self.record_cs_soft_disp(0, after, req);
        }
        self.bar.split_pend.add_with_req(after, before, soft, req);
    }

    #[inline]
    pub(super) fn add_pend(&mut self, after: MTLStages, before: MTLStages) {
        self.add_pend_with_soft(after, before, [MTLStages::empty(); 2]);
    }

    pub(super) fn add_pend_with_soft_res(
        &mut self,
        after: MTLStages,
        hard_before: MTLStages,
        soft_res: &[usize],
    ) {
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        if after.is_empty() {
            return;
        }
        if (after == MTLStages::All
            && !(self.opts.opt_cs_to_ps_overlap_v2 || self.gfx_to_ps_overlap_active()))
            || soft_res.is_empty()
            || !self.vs_buf_defer_exact_active()
            || self.is_copy_queue_light()
            || legacy_barriers()
            || !barrier_masks()
            || soft_res
                .iter()
                .any(|&r| r == 0 || unsafe { (*(r as *const Resource)).desc.Flags & SIMUL != 0 })
        {
            self.add_pend(after, hard_before | PRE_RASTER);
            return;
        }
        let mut soft = [MTLStages::empty(); 2];
        for &r in soft_res {
            let c = res_class(r);
            for (i, s) in soft.iter_mut().enumerate() {
                if c & (1 << i) != 0 {
                    *s |= PRE_RASTER;
                }
            }
        }
        if (hard_before | soft[0] | soft[1]).is_empty() {
            return;
        }
        if hard_before.intersects(COMPUTE_STAGES) {
            if matches!(self.enc, Encoder::Compute(_)) {
                let mid_on = self.mid_qb_stagesync_active();
                for &(ci, cbit) in &CS_SUB_BITS {
                    if hard_before.contains(cbit)
                        && !self
                            .unsatisfied_enc_after_for(after, cbit, mid_on)
                            .is_empty()
                    {
                        self.bar.pend_cs_poisoned[ci] = true;
                        if self.opts.opt_cs_mid_qb_unpoison_v4 {
                            self.bar.cs_mid_v3_poisoned[ci] = true;
                        }
                    }
                }
                if !self.opts.opt_cs_mid_qb_unpoison_v4 {
                    self.bar.cs_mid_v3_poisoned = [true; 3];
                }
            } else {
                for &(ci, cbit) in &CS_SUB_BITS {
                    if hard_before.contains(cbit) {
                        self.bar.pend_cs_poisoned[ci] = true;
                    }
                }
            }
        }
        let req = self.cur_producer_gen();
        for &r in soft_res {
            let c = res_class(r);
            for i in 0..2 {
                if c & (1 << i) != 0 && !self.bar.soft_pend_poisoned[i] {
                    Self::push_soft_obl_into(
                        &mut self.bar.soft_pend_items[i],
                        r,
                        after,
                        PRE_RASTER,
                        req,
                    );
                }
            }
        }
        self.bar.pend_after |= after;
        self.bar.pend_before |= hard_before;
        self.bar.pend_before_soft[0] |= soft[0];
        self.bar.pend_before_soft[1] |= soft[1];
        if self.clear_isolate_active() && hard_before.contains(MTLStages::Fragment) {
            if self.opts.opt_clear_zero_qb_coalesce {
                self.bar.frag_rt_poisoned = true;
                self.bar.frag_rt_pend.clear();
            } else {
                self.bar.frag_non_rt_after |= after;
                merge_req_for_after(&mut self.bar.frag_non_rt_req, after, &req);
            }
        }
        if self.ps_to_cs_active() && hard_before.contains(MTLStages::Dispatch) {
            self.record_cs_soft_disp(0, after, req);
        }
        self.bar
            .split_pend
            .add_with_req(after, hard_before, soft, req);
    }

    #[inline]
    pub(super) fn clear_all_pend(&mut self) {
        self.bar.pend_after = MTLStages::empty();
        self.bar.pend_before = MTLStages::empty();
        self.bar.pend_before_soft = [MTLStages::empty(); 2];
        self.bar.split_pend = SplitPend::default();
        self.bar.soft_pend_items[0].clear();
        self.bar.soft_pend_items[1].clear();
        self.bar.soft_pend_poisoned = [false; 2];
        self.bar.cs_mid_res_pend.clear();
        self.bar.cs_mid_v3_items.clear();
        self.bar.cs_mid_v3_poisoned = [false; 3];
        self.bar.pend_cs_items.clear();
        self.bar.pend_cs_poisoned = [false; 3];
        self.reset_frag_rt_pend();
        self.reset_cs_soft_disp();
    }

    #[inline]
    pub(super) fn sync_legacy_from_split(&mut self) {
        let (a, b, s) = self.bar.split_pend.to_legacy();
        self.bar.pend_after = a;
        self.bar.pend_before = b;
        self.bar.pend_before_soft = s;
    }

    #[inline]
    pub(super) fn sync_covers_gfx_uav(&self, s: &StageSync) -> bool {
        self.bar.gfx_uav_gen > 0
            && self.bar.gfx_uav_stage_gen != [0; 4]
            && (0..4).all(|si| s.gen[si] >= self.bar.gfx_uav_stage_gen[si])
    }

    #[inline]
    pub(super) fn sync_from_after(&self, after: MTLStages) -> StageSync {
        if after == MTLStages::All {
            return StageSync {
                pre_list_all: true,
                pre_list_stages: MTLStages::All,
                gen: self.bar.stage_gen,
                disp_gen: self.bar.cs_disp_gen,
                as_gen: self.bar.cs_as_gen,
                gfx_uav_gen: self.bar.gfx_uav_gen,
            };
        }
        let mut out = StageSync::default();
        if after.contains(MTLStages::Dispatch) {
            out.disp_gen = self.bar.cs_disp_gen;
        }
        if after.contains(MTLStages::AccelerationStructure) {
            out.as_gen = self.bar.cs_as_gen;
        }
        if after.contains(MTLStages::Fragment | PRE_RASTER) {
            out.gfx_uav_gen = self.bar.gfx_uav_gen;
        }
        for (i, &bit) in STAGE_BITS.iter().enumerate() {
            if after.contains(bit) {
                out.gen[i] = self.bar.stage_gen[i];
                out.merge(self.bar.stage_sync[i]);
            }
        }
        if self.sync_covers_gfx_uav(&out) {
            out.gfx_uav_gen = self.bar.gfx_uav_gen;
        }
        out
    }

    #[inline]
    fn unsatisfied_enc_after_impl(
        &self,
        qa: MTLStages,
        qb: MTLStages,
        req: Option<&[u32; 7]>,
        enabled: bool,
    ) -> MTLStages {
        if !enabled || qa.is_empty() || qb.is_empty() {
            return qa;
        }
        let mut cons = [0usize; 7];
        let mut n_cons = 0usize;
        let mut all_pre_list = true;
        for (ci, &cbit) in STAGE_BITS.iter().enumerate() {
            if qb.contains(cbit) {
                cons[n_cons] = ci;
                n_cons += 1;
                if !self.bar.enc_cur_sync[ci].pre_list_all {
                    all_pre_list = false;
                }
            }
        }
        if n_cons == 0 {
            return qa;
        }
        if !all_pre_list && (!self.opts.opt_list_boundary_overlap || qa == MTLStages::All) {
            return qa;
        }
        let check_all = qa == MTLStages::All;
        let mut out = if check_all {
            MTLStages::empty()
        } else {
            qa - KNOWN_ALL_STAGES
        };
        if n_cons == 1 {
            let sync = &self.bar.enc_cur_sync[cons[0]];
            let check_pre = !all_pre_list && !sync.pre_list_all;
            for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
                if !check_all && !qa.contains(pbit) {
                    continue;
                }
                if !all_pre_list && pbit == MTLStages::Fragment {
                    out |= pbit;
                    continue;
                }
                let gen_unsat = match req {
                    Some(r) => {
                        let target_gen = if check_all {
                            self.bar.stage_gen[pi]
                        } else {
                            r[pi].min(self.bar.stage_gen[pi])
                        };
                        sync.gen[pi] < target_gen
                    }
                    None => sync.gen[pi] != self.bar.stage_gen[pi],
                };
                if gen_unsat || (check_pre && !sync.pre_list_stages.contains(pbit)) {
                    out |= pbit;
                }
            }
        } else {
            let active_cons = &cons[..n_cons];
            for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
                if !check_all && !qa.contains(pbit) {
                    continue;
                }
                if !all_pre_list && pbit == MTLStages::Fragment {
                    out |= pbit;
                    continue;
                }
                let target_gen = match req {
                    Some(r) if !check_all => r[pi].min(self.bar.stage_gen[pi]),
                    _ => self.bar.stage_gen[pi],
                };
                let mut synced = true;
                for &ci in active_cons {
                    let sync = &self.bar.enc_cur_sync[ci];
                    let gen_unsat = if req.is_some() {
                        sync.gen[pi] < target_gen
                    } else {
                        sync.gen[pi] != target_gen
                    };
                    if gen_unsat
                        || (!all_pre_list
                            && !sync.pre_list_all
                            && !sync.pre_list_stages.contains(pbit))
                    {
                        synced = false;
                        break;
                    }
                }
                if !synced {
                    out |= pbit;
                }
            }
        }
        if !all_pre_list && out != qa {
            stats::inc(&stats::c::list_boundary_overlap);
        }
        out
    }

    /// Strip producer stages from `qa` (`!full`) that are already transitively satisfied
    /// for all consumer stages in `qb` on the open encoder.
    #[inline]
    pub(super) fn unsatisfied_enc_after_for(
        &self,
        qa: MTLStages,
        qb: MTLStages,
        enabled: bool,
    ) -> MTLStages {
        self.unsatisfied_enc_after_impl(qa, qb, None, enabled)
    }

    /// Per-obligation required-generation variant of `unsatisfied_enc_after_for`:
    /// compares `enc_cur_sync[ci].gen[pi] < req[pi].min(self.bar.stage_gen[pi])` so an
    /// obligation stamped at `req[pi] = G` stays satisfied even if stage `pi` ran
    /// again in a later encoder (`stage_gen[pi] > G`) for an unrelated resource.
    #[inline]
    pub(super) fn unsatisfied_enc_after_for_req(
        &self,
        qa: MTLStages,
        qb: MTLStages,
        req: &[u32; 7],
        enabled: bool,
    ) -> MTLStages {
        self.unsatisfied_enc_after_impl(qa, qb, Some(req), enabled)
    }

    #[inline]
    pub(super) fn unsatisfied_mid_qb_after(&self, qa: MTLStages, qb: MTLStages) -> MTLStages {
        self.unsatisfied_enc_after_for(qa, qb, self.mid_qb_stagesync_active())
    }

    /// Fold dominated producer stage bits in `!full` `after` using per-obligation
    /// required generations `req` (`AW2_SPLIT_COMPUTE_EXACT=1` / `AW2_RENDER_DOM_EXACT=1`).
    /// Never narrows `MTLStages::All` so `pre_list_all` semantics remain untouched.
    #[inline]
    pub(super) fn fold_dominated_after(&self, after: MTLStages, req: &[u32; 7]) -> MTLStages {
        if after.is_empty() || after == MTLStages::All {
            return after;
        }
        let mut known_all = MTLStages::empty();
        for &bit in &STAGE_BITS {
            known_all |= bit;
        }
        let mut kept = after;
        let extra = kept - known_all;
        for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
            if !kept.contains(pbit) {
                continue;
            }
            let target_gen = req[pi].min(self.bar.stage_gen[pi]);
            let dom_full = (0..7).any(|ki| {
                ki != pi
                    && kept.contains(STAGE_BITS[ki])
                    && self.bar.stage_gen[ki] > 0
                    && self.bar.stage_sync[ki].pre_list_all
                    && self.bar.stage_sync[ki].gen[pi] >= target_gen
            });
            if dom_full {
                kept -= pbit;
            } else if self.opts.opt_list_boundary_overlap && pbit != MTLStages::Fragment {
                let dom_part = (0..7).any(|ki| {
                    ki != pi
                        && kept.contains(STAGE_BITS[ki])
                        && self.bar.stage_gen[ki] > 0
                        && self.bar.stage_sync[ki].pre_list_stages.contains(pbit)
                        && self.bar.stage_sync[ki].gen[pi] >= target_gen
                });
                if dom_part {
                    kept -= pbit;
                    stats::inc(&stats::c::list_boundary_overlap);
                }
            }
        }
        kept | extra
    }

    /// Prune producer stage bits from an encoder-opening `!full` `after` mask when
    /// another kept stage `k` (`stage_gen[k] > 0 && stage_sync[k].pre_list_all`) in `after`
    /// already transitively waited on `stage_gen[p]` (`AW2_STAGESYNC_BARRIER_V2=1`).
    #[inline]
    pub(super) fn prune_dominated_after(&self, after: MTLStages) -> MTLStages {
        if !self.opts.opt_stagesync_barrier_v2 || after.is_empty() {
            return after;
        }
        let mut known_all = MTLStages::empty();
        for &bit in &STAGE_BITS {
            known_all |= bit;
        }
        let mut kept = if after == MTLStages::All {
            // Only narrow `MTLStages::All` (e.g. `s0 == 0` `COMMON` transition when `!full`)
            // when at least one closed in-list stage already anchored `pre_list_all`.
            let Some(anchor_k) =
                (0..7).find(|&k| self.bar.stage_gen[k] > 0 && self.bar.stage_sync[k].pre_list_all)
            else {
                return after;
            };
            let mut m = STAGE_BITS[anchor_k];
            for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
                if self.bar.stage_gen[pi] != self.bar.stage_sync[anchor_k].gen[pi] {
                    m |= pbit;
                }
            }
            m
        } else {
            after
        };
        let extra = kept - known_all;
        for (pi, &pbit) in STAGE_BITS.iter().enumerate() {
            if !kept.contains(pbit) {
                continue;
            }
            let dom_full = (0..7).any(|ki| {
                ki != pi
                    && kept.contains(STAGE_BITS[ki])
                    && self.bar.stage_gen[ki] > 0
                    && self.bar.stage_sync[ki].pre_list_all
                    && self.bar.stage_sync[ki].gen[pi] == self.bar.stage_gen[pi]
            });
            if dom_full {
                kept -= pbit;
            } else if after != MTLStages::All
                && self.opts.opt_list_boundary_overlap
                && pbit != MTLStages::Fragment
            {
                let dom_part = (0..7).any(|ki| {
                    ki != pi
                        && kept.contains(STAGE_BITS[ki])
                        && self.bar.stage_gen[ki] > 0
                        && self.bar.stage_sync[ki].pre_list_stages.contains(pbit)
                        && self.bar.stage_sync[ki].gen[pi] == self.bar.stage_gen[pi]
                });
                if dom_part {
                    kept -= pbit;
                    stats::inc(&stats::c::list_boundary_overlap);
                }
            }
        }
        kept | extra
    }

    #[inline]
    pub(super) fn record_enc_queue_sync(&mut self, after: MTLStages, before: MTLStages) {
        if after.is_empty() || before.is_empty() {
            return;
        }
        if (after.contains(MTLStages::Fragment) || after == MTLStages::All)
            && before.intersects(PRE_RASTER)
        {
            self.rp_sync.gfx_ps_overlap_chain = 0;
            self.rp_sync.enc_deferred_frag_ps = false;
        }
        let s = self.sync_from_after(after);
        for (i, &bit) in STAGE_BITS.iter().enumerate() {
            if before.contains(bit) {
                self.bar.enc_cur_sync[i].merge(s);
                if self.sync_covers_gfx_uav(&self.bar.enc_cur_sync[i]) {
                    self.bar.enc_cur_sync[i].gfx_uav_gen = self.bar.gfx_uav_gen;
                }
            }
        }
        if self.bar.enc_cur_sync[0].pre_list_all {
            self.bar.list_unwaited_vtx_frag = false;
            self.bar.vtx_defer_list_frag = false;
            self.bar.vtx_carry_list_frag = false;
        }
        if after.contains(MTLStages::Fragment) && before.contains(MTLStages::Dispatch) {
            self.bar.clear_tex_cs.clear();
        }
        if after.contains(MTLStages::Fragment)
            && before.contains(PRE_RASTER | MTLStages::Fragment | COMPUTE_STAGES)
        {
            self.bar.clear_rp_res_ids.clear();
            self.bar.clear_tex_cs.clear();
            self.bar.clear_tex_gfx_before = MTLStages::empty();
            self.bar.clear_tex_gfx_soft = [MTLStages::empty(); 2];
            self.bar.clear_tex_gfx_soft_res.clear();
            self.bar.clear_tex_gfx_soft_poisoned = false;
        }
    }

    #[inline]
    pub(super) fn note_stage_ran(&mut self, stages: MTLStages) {
        self.bar.enc_ran_stages |= stages;
        for (i, &bit) in STAGE_BITS.iter().enumerate() {
            if stages.contains(bit) {
                self.bar.enc_completed_sync[i].merge(self.bar.enc_cur_sync[i]);
            }
        }
        if stages.contains(MTLStages::Dispatch) && !self.bar.uav_passthrough.is_empty() {
            let ib_tight = self.ib_tight_active();
            let legacy_blit_obl =
                !ib_tight && (self.ib.ib_obl_all || !self.ib.ib_obl.is_empty());
            let mut updated = Vec::with_capacity(self.bar.uav_passthrough.len());
            for &(rk, st) in &self.bar.uav_passthrough {
                let mut rem = self.unsatisfied_enc_after_for(st, MTLStages::Dispatch, true);
                if st.contains(MTLStages::Blit) {
                    let has_unfired_blit = if ib_tight {
                        self.ib.ib_obl_tight.iter().any(|o| {
                            o.after.contains(MTLStages::Blit)
                                && o.before.intersects(MTLStages::Dispatch)
                                && (rk == 0 || o.r == 0 || o.r == rk)
                        })
                    } else {
                        legacy_blit_obl
                            && (rk == 0 || self.ib.ib_obl_all || self.ib.ib_obl.contains(&rk))
                    };
                    if has_unfired_blit {
                        rem |= MTLStages::Blit;
                    }
                }
                if !rem.is_empty() {
                    updated.push((rk, rem));
                }
            }
            self.bar.uav_passthrough = updated;
        }
    }

    #[inline]
    pub(super) fn pre_list_anchor_stages(&self) -> MTLStages {
        let mut out = MTLStages::empty();
        // Prefer compute stages (`Dispatch`/`Blit`/`AS`) as anchors so subsequent compute
        // encoders do not need to wait on `Fragment`/`PRE_RASTER` when `CLEAR` passes are in flight.
        for &(i, bit) in &[
            (STAGE_DISPATCH, MTLStages::Dispatch),
            (5, MTLStages::Blit),
            (STAGE_ACCEL, MTLStages::AccelerationStructure),
        ] {
            if self.bar.stage_sync[i].pre_list_all {
                out |= bit;
            }
        }
        out
    }

    #[inline]
    pub(super) fn open_compute_disp_uav_synced(&self) -> bool {
        if !matches!(self.enc, Encoder::Compute(_)) {
            return false;
        }
        let d = self.bar.enc_cur_sync[STAGE_DISPATCH];
        d.pre_list_all
            && d.disp_gen == self.bar.cs_disp_gen
            && d.as_gen == self.bar.cs_as_gen
            && d.gfx_uav_gen == self.bar.gfx_uav_gen
            && self.bar.enc_cs_soft_disp.is_empty()
    }

    #[inline]
    pub(super) fn flush_cs_uav_deferred(&mut self) {
        if !self.bar.cs_uav_after.is_empty() {
            let a = std::mem::replace(&mut self.bar.cs_uav_after, MTLStages::empty());
            let b = std::mem::replace(&mut self.bar.cs_uav_before, MTLStages::empty());
            let s = [
                std::mem::replace(&mut self.bar.cs_uav_soft[0], MTLStages::empty()),
                std::mem::replace(&mut self.bar.cs_uav_soft[1], MTLStages::empty()),
            ];
            let items = [
                std::mem::take(&mut self.bar.cs_uav_soft_items[0]),
                std::mem::take(&mut self.bar.cs_uav_soft_items[1]),
            ];
            let poisoned = std::mem::replace(&mut self.bar.cs_uav_soft_poisoned, [false; 2]);
            if (self.opts.opt_cs_to_ps_overlap_v2 || self.gfx_to_ps_overlap_active())
                && !self.is_copy_queue_light()
                && !legacy_barriers()
                && barrier_masks()
            {
                if b.intersects(COMPUTE_STAGES) {
                    for &(ci, cbit) in &CS_SUB_BITS {
                        if b.contains(cbit) {
                            self.bar.pend_cs_poisoned[ci] = true;
                        }
                    }
                }
                let req = self.cur_producer_gen();
                for i in 0..2 {
                    let sb = s[i] & PRE_RASTER;
                    if !sb.is_empty() {
                        if poisoned[i] || items[i].is_empty() {
                            self.bar.soft_pend_poisoned[i] = true;
                            self.bar.soft_pend_items[i].clear();
                        } else if !self.bar.soft_pend_poisoned[i] {
                            for &r in &items[i] {
                                Self::push_soft_obl_into(
                                    &mut self.bar.soft_pend_items[i],
                                    r,
                                    a,
                                    sb,
                                    req,
                                );
                            }
                        }
                    }
                }
                self.bar.pend_after |= a;
                self.bar.pend_before |= b;
                self.bar.pend_before_soft[0] |= s[0];
                self.bar.pend_before_soft[1] |= s[1];
                if self.clear_isolate_active() && b.contains(MTLStages::Fragment) {
                    if self.opts.opt_clear_zero_qb_coalesce {
                        self.bar.frag_rt_poisoned = true;
                        self.bar.frag_rt_pend.clear();
                    } else {
                        self.bar.frag_non_rt_after |= a;
                        merge_req_for_after(&mut self.bar.frag_non_rt_req, a, &req);
                    }
                }
                if self.ps_to_cs_active() && b.contains(MTLStages::Dispatch) {
                    self.record_cs_soft_disp(0, a, req);
                }
                self.bar.split_pend.add_with_req(a, b, s, req);
            } else {
                self.add_pend_with_soft(a, b, s);
            }
        } else {
            self.bar.cs_uav_soft_items[0].clear();
            self.bar.cs_uav_soft_items[1].clear();
            self.bar.cs_uav_soft_poisoned = [false; 2];
        }
    }

    #[inline]
    pub(super) fn flush_clear_tex_cs_all(&mut self) {
        if !self.bar.clear_tex_cs.is_empty() {
            self.bar.clear_tex_cs.clear();
            self.add_pend(MTLStages::Fragment, COMPUTE_STAGES);
        }
    }

    #[inline]
    pub(super) fn flush_clear_tex_cs_touch(&mut self, touch: &[usize]) {
        if !self.bar.clear_tex_cs.is_empty()
            && touch.iter().any(|r| self.bar.clear_tex_cs.contains(r))
        {
            self.bar.clear_tex_cs.retain(|r| !touch.contains(r));
            self.add_pend(MTLStages::Fragment, COMPUTE_STAGES);
        }
    }

    /// Work just encoded in `after` stages must be visible to later `before`
    /// stages (internal producers: expansion kernels, copies).
    pub(super) fn produced(&mut self, after: MTLStages, before: MTLStages) {
        if legacy_barriers() || !barrier_masks() {
            self.need_full(FullWhy::Internal);
        } else {
            let (after, before) = if self.is_copy_queue_light() {
                (MTLStages::Blit, MTLStages::Blit)
            } else {
                (after, before)
            };
            self.add_pend(after, before);
            if before.intersects(COMPUTE_STAGES) {
                if self.ib_tight_active() {
                    self.ib_add_obl_tight(0, after & COMPUTE_STAGES, before & COMPUTE_STAGES);
                } else {
                    self.ib_note(0);
                }
            }
        }
    }

    pub(super) fn ib_reset_tight(&mut self) {
        self.ib.ib_stages = MTLStages::empty();
        self.ib.ib_disp_uav.clear_exact();
        self.ib.ib_disp_read.clear_exact();
        self.ib.ib_disp_uav_cls[0].clear_exact();
        self.ib.ib_disp_uav_cls[1].clear_exact();
        self.ib.ib_disp_read_cls[0].clear_exact();
        self.ib.ib_disp_read_cls[1].clear_exact();
        self.ib.ib_disp_uav_spans.clear_exact();
        self.ib.ib_disp_read_spans.clear_exact();
        self.ib.ib_unbounded_read_classes = 0;
        self.ib.ib_disp_uav_classes = 0;
        self.ib.ib_disp_read_classes = 0;
        self.ib.ib_disp_uav_srv_classes = 0;
        self.ib.ib_disp_uav_hk = 0;
        self.ib.ib_disp_ro_hk = 0;
        self.ib.ib_disp_read_hk = 0;
        self.ib.ib_disp_uav_srv_hk = 0;
        self.ib.ib_blit_touch.clear_exact();
        self.ib.ib_blit_dst.clear();
        self.ib.ib_as_touch.clear_exact();
        self.ib.ib_cs_cleared.clear();
        self.ib.ib_obl_tight.clear();
        for s in &mut self.ib.ib_sub {
            s.reset();
        }
    }

    pub(super) fn ib_clear_satisfied(
        &mut self,
        need_after: MTLStages,
        before_enc: MTLStages,
        op_stage: MTLStages,
    ) {
        if before_enc == COMPUTE_STAGES {
            self.ib.ib_obl_tight.retain_mut(|obl| {
                obl.after -= need_after;
                if obl.after != MTLStages::Dispatch {
                    obl.spans = None;
                }
                !obl.after.is_empty()
            });
            for s in &mut self.ib.ib_sub {
                s.clear_after(need_after);
            }
            self.ib.ib_stages -= need_after;
            if need_after.contains(MTLStages::Dispatch) {
                self.ib.ib_disp_uav.clear_exact();
                self.ib.ib_disp_read.clear_exact();
                self.ib.ib_disp_uav_cls[0].clear_exact();
                self.ib.ib_disp_uav_cls[1].clear_exact();
                self.ib.ib_disp_read_cls[0].clear_exact();
                self.ib.ib_disp_read_cls[1].clear_exact();
                self.ib.ib_disp_uav_spans.clear_exact();
                self.ib.ib_disp_read_spans.clear_exact();
                self.ib.ib_unbounded_read_classes = 0;
                self.ib.ib_disp_uav_classes = 0;
                self.ib.ib_disp_read_classes = 0;
                self.ib.ib_disp_uav_srv_classes = 0;
                self.ib.ib_disp_uav_hk = 0;
                self.ib.ib_disp_ro_hk = 0;
                self.ib.ib_disp_read_hk = 0;
                self.ib.ib_disp_uav_srv_hk = 0;
                self.ib.ib_cs_cleared.clear();
            }
            if need_after.contains(MTLStages::Blit) {
                self.ib.ib_blit_touch.clear_exact();
                if op_stage.contains(MTLStages::Blit) {
                    self.ib.ib_blit_dst.clear();
                }
            }
            if need_after.contains(MTLStages::AccelerationStructure) {
                self.ib.ib_as_touch.clear_exact();
            }
            return;
        }
        let mut split_add: Vec<IbObl> = Vec::new();
        self.ib.ib_obl_tight.retain_mut(|obl| {
            if !obl.before.intersects(before_enc) || !obl.after.intersects(need_after) {
                return true;
            }
            if before_enc.contains(obl.before) {
                obl.after -= need_after;
                if obl.after != MTLStages::Dispatch {
                    obl.spans = None;
                }
                !obl.after.is_empty()
            } else {
                let rem_after = obl.after - need_after;
                let sat_before = obl.before & before_enc;
                obl.before -= before_enc;
                if !rem_after.is_empty() {
                    split_add.push(IbObl {
                        r: obl.r,
                        after: rem_after,
                        before: sat_before,
                        kind: obl.kind,
                        spans: if rem_after == MTLStages::Dispatch {
                            obl.spans.clone()
                        } else {
                            None
                        },
                    });
                }
                true
            }
        });
        for obl in split_add {
            self.ib.ib_obl_tight.push(obl);
        }
        for &(ci, cbit) in &CS_SUB_BITS {
            if before_enc.contains(cbit) {
                self.ib.ib_sub[ci].clear_after(need_after);
            }
        }
        self.ib.ib_stages =
            self.ib.ib_sub[0].stages | self.ib.ib_sub[1].stages | self.ib.ib_sub[2].stages;
        if !(self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light())
            && before_enc.contains(MTLStages::Dispatch)
            && need_after.contains(MTLStages::Dispatch)
        {
            self.ib.ib_disp_uav.copy_from(&self.ib.ib_sub[0].disp_uav);
            self.ib.ib_disp_read.copy_from(&self.ib.ib_sub[0].disp_read);
            if self.cs_intra_safe_active() {
                self.ib.ib_disp_uav_cls[0].copy_from(&self.ib.ib_sub[0].disp_uav_cls[0]);
                self.ib.ib_disp_uav_cls[1].copy_from(&self.ib.ib_sub[0].disp_uav_cls[1]);
                self.ib.ib_disp_read_cls[0].copy_from(&self.ib.ib_sub[0].disp_read_cls[0]);
                self.ib.ib_disp_read_cls[1].copy_from(&self.ib.ib_sub[0].disp_read_cls[1]);
            }
            self.ib
                .ib_disp_uav_spans
                .copy_from(&self.ib.ib_sub[0].disp_uav_spans);
            self.ib
                .ib_disp_read_spans
                .copy_from(&self.ib.ib_sub[0].disp_read_spans);
            self.ib.ib_unbounded_read_classes = self.ib.ib_sub[0].unbounded_read_classes;
            self.ib.ib_disp_uav_classes = self.ib.ib_sub[0].disp_uav_classes;
            self.ib.ib_disp_read_classes = self.ib.ib_sub[0].disp_read_classes;
            self.ib.ib_disp_uav_srv_classes = self.ib.ib_sub[0].disp_uav_srv_classes;
            self.ib.ib_disp_uav_hk = self.ib.ib_sub[0].disp_uav_hk;
            self.ib.ib_disp_ro_hk = self.ib.ib_sub[0].disp_ro_hk;
            self.ib.ib_disp_read_hk = self.ib.ib_sub[0].disp_read_hk;
            self.ib.ib_disp_uav_srv_hk = self.ib.ib_sub[0].disp_uav_srv_hk;
            self.ib.ib_cs_cleared.clear();
            self.ib
                .ib_cs_cleared
                .extend_from_slice(&self.ib.ib_sub[0].cs_cleared);
        }
        if before_enc.contains(MTLStages::Blit)
            && need_after.contains(MTLStages::Blit)
            && op_stage.contains(MTLStages::Blit)
        {
            self.ib.ib_blit_dst.clear();
        }
    }

    pub(super) fn ib_merge_set(dst: &mut TrackedSet<usize>, src: Option<&[usize]>) {
        match src {
            Some(s) => {
                if dst.is_exact() {
                    for &r in s {
                        if r != 0 {
                            dst.insert_res(r);
                        }
                    }
                }
            }
            None => dst.invalidate(),
        }
    }

    pub(super) fn ib_merge_spans_whole(dst: &mut TrackedSet<SubRange>, src: Option<&[usize]>) {
        match src {
            Some(s) => {
                if dst.is_exact() {
                    for &r in s {
                        if r != 0 {
                            dst.insert_span(SubRange::whole(r));
                        }
                    }
                }
            }
            None => dst.invalidate(),
        }
    }

    pub(super) fn ib_merge_spans_exact(dst: &mut TrackedSet<SubRange>, src: &[SubRange]) {
        if dst.is_exact() {
            for &s in src {
                dst.insert_span(s);
            }
        }
    }

    pub(super) fn ib_add_obl_tight(&mut self, r: usize, after: MTLStages, before: MTLStages) {
        self.ib_add_obl_spans_tight(r, after, before, OblKind::Any, None);
    }

    pub(super) fn ib_add_obl_kind_tight(
        &mut self,
        r: usize,
        after: MTLStages,
        before: MTLStages,
        kind: OblKind,
    ) {
        self.ib_add_obl_spans_tight(r, after, before, kind, None);
    }

    pub(super) fn ib_add_obl_spans_tight(
        &mut self,
        r: usize,
        after: MTLStages,
        before: MTLStages,
        kind: OblKind,
        spans: Option<SmallSpans>,
    ) {
        let before = before & COMPUTE_STAGES;
        if before.is_empty() {
            return;
        }
        if self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light() {
            for &(ci, cbit) in &CS_SUB_BITS {
                if !before.contains(cbit) {
                    continue;
                }
                let sub_after = after & self.ib.ib_sub[ci].stages;
                if sub_after.is_empty() {
                    continue;
                }
                let sub_spans = if sub_after == MTLStages::Dispatch {
                    spans.clone()
                } else {
                    None
                };
                if let Some(existing) = self
                    .ib
                    .ib_obl_tight
                    .iter_mut()
                    .find(|o| o.r == r && o.before == cbit)
                {
                    existing.after |= sub_after;
                    existing.kind = existing.kind.join(kind);
                    if existing.after != MTLStages::Dispatch {
                        existing.spans = None;
                    } else {
                        match (&mut existing.spans, sub_spans) {
                            (Some(dst), Some(src)) => dst.extend_coalesced(&src),
                            (dst, _) => *dst = None,
                        }
                    }
                } else {
                    self.ib.ib_obl_tight.push(IbObl {
                        r,
                        after: sub_after,
                        before: cbit,
                        kind,
                        spans: sub_spans,
                    });
                }
            }
            return;
        }
        let after = after & self.ib.ib_stages;
        if after.is_empty() {
            return;
        }
        let spans = if after == MTLStages::Dispatch {
            spans
        } else {
            None
        };
        if let Some(existing) = self.ib.ib_obl_tight.iter_mut().find(|o| o.r == r) {
            existing.after |= after;
            existing.before |= before;
            existing.kind = existing.kind.join(kind);
            if existing.after != MTLStages::Dispatch {
                existing.spans = None;
            } else {
                match (&mut existing.spans, spans) {
                    (Some(dst), Some(src)) => dst.extend_coalesced(&src),
                    (dst, _) => *dst = None,
                }
            }
        } else {
            self.ib.ib_obl_tight.push(IbObl {
                r,
                after,
                before,
                kind,
                spans,
            });
        }
    }

    /// Record an intra-compute obligation on `r` (`0` = all) before `before` stages
    /// for whichever stages in the open compute encoder have actually touched `r`.
    pub(super) fn ib_note_touch_before_tight(&mut self, r: usize, before: MTLStages) {
        use MTLStages as S;
        if self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light() {
            let cs_before = before & COMPUTE_STAGES;
            for &(ci, cbit) in &CS_SUB_BITS {
                if !cs_before.contains(cbit) {
                    continue;
                }
                let mut sub_after = S::empty();
                let sub = &self.ib.ib_sub[ci];
                if sub.stages.contains(S::Dispatch) {
                    let uav_hit = sub.disp_uav.hits_res(r);
                    let read_hit = !uav_hit && sub.disp_read.hits_res(r);
                    let cs_hit = !uav_hit
                        && !read_hit
                        && if r == 0 {
                            !sub.cs_cleared.is_empty()
                        } else {
                            sub.cs_cleared.contains(&r)
                        };
                    if uav_hit || read_hit || cs_hit {
                        sub_after |= S::Dispatch;
                    }
                }
                if sub.stages.contains(S::Blit) && sub.blit_touch.hits_res(r) {
                    sub_after |= S::Blit;
                }
                if sub.stages.contains(S::AccelerationStructure) && sub.as_touch.hits_res(r) {
                    sub_after |= S::AccelerationStructure;
                }
                self.ib_add_obl_tight(r, sub_after, cbit);
            }
            return;
        }
        let mut after = S::empty();
        if self.ib.ib_stages.contains(S::Dispatch) {
            let uav_hit = self.ib.ib_disp_uav.hits_res(r);
            let read_hit = !uav_hit && self.ib.ib_disp_read.hits_res(r);
            let cs_hit = !uav_hit
                && !read_hit
                && if r == 0 {
                    !self.ib.ib_cs_cleared.is_empty()
                } else {
                    self.ib.ib_cs_cleared.contains(&r)
                };
            if uav_hit || read_hit || cs_hit {
                after |= S::Dispatch;
            }
        }
        if self.ib.ib_stages.contains(S::Blit) && self.ib.ib_blit_touch.hits_res(r) {
            after |= S::Blit;
        }
        if self.ib.ib_stages.contains(S::AccelerationStructure) && self.ib.ib_as_touch.hits_res(r) {
            after |= S::AccelerationStructure;
        }
        self.ib_add_obl_tight(r, after, before);
    }

    #[inline]
    pub(super) fn push_soft_obl_into(
        list: &mut SoftOblVec,
        r: usize,
        ia: MTLStages,
        ib: MTLStages,
        req: [u32; 7],
    ) {
        let ib = ib & PRE_RASTER;
        if r == 0 || ia.is_empty() || ib.is_empty() {
            return;
        }
        let bit = res_bloom_bit(r);
        if (list.bloom & bit) == 0 {
            list.bloom |= bit;
            let mut item_req = [0u32; 7];
            merge_req_for_after(&mut item_req, ia, &req);
            list.items.push(SoftPreResObl {
                r,
                after: ia,
                before: ib,
                req: item_req,
            });
            return;
        }
        if let Some(ex) = list.items.iter_mut().find(|o| o.r == r) {
            ex.after |= ia;
            ex.before |= ib;
            merge_req_for_after(&mut ex.req, ia, &req);
        } else {
            let mut item_req = [0u32; 7];
            merge_req_for_after(&mut item_req, ia, &req);
            list.items.push(SoftPreResObl {
                r,
                after: ia,
                before: ib,
                req: item_req,
            });
        }
    }

    #[inline]
    pub(super) fn extend_soft_items(dst: &mut SoftOblVec, src: SoftOblVec) {
        if dst.items.is_empty() {
            *dst = src;
            return;
        }
        for it in src.items {
            Self::push_soft_obl_into(dst, it.r, it.after, it.before, it.req);
        }
    }

    #[inline]
    pub(super) fn strip_soft_items_after(items: &mut SoftOblVec, strip: MTLStages) {
        if strip.is_empty() {
            return;
        }
        if strip == MTLStages::All {
            items.clear();
            return;
        }
        items.retain_mut(|it| {
            if it.after == MTLStages::All {
                return true;
            }
            it.after -= strip;
            !it.after.is_empty() && !it.before.is_empty()
        });
    }

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub(super) fn partition_soft_items_for_cur_draw(
        &mut self,
        class_idx: usize,
        items: &mut SoftOblVec,
        poisoned: bool,
        total_after: MTLStages,
        total_before: MTLStages,
        _total_req: &[u32; 7],
        count_stat: bool,
    ) -> Option<(
        MTLStages,
        MTLStages,
        [u32; 7],
        Option<(MTLStages, MTLStages, [u32; 7])>,
    )> {
        if !(self.opts.opt_cs_to_ps_overlap
            || self.opts.opt_cs_to_ps_overlap_v2
            || self.gfx_to_ps_overlap_active()
            || self.list_rsplit_v2_active())
            || !self.opts.opt_gfx_desc_table_exact
            || !self.rp_sync.cur_draw_exact_valid
            || poisoned
            || (self.rp_sync.cur_draw_pre_unb & (1 << class_idx)) != 0
            || total_after.is_empty()
            || total_before.is_empty()
            || (total_after == MTLStages::All
                && !(self.opts.opt_cs_to_ps_overlap_v2
                    || self.gfx_to_ps_overlap_active()
                    || self.list_rsplit_v2_active()))
            || items.is_empty()
        {
            return None;
        }
        let mut union_a = MTLStages::empty();
        for it in items.iter() {
            if it.r == 0
                || (it.after == MTLStages::All
                    && !(self.opts.opt_cs_to_ps_overlap_v2
                        || self.gfx_to_ps_overlap_active()
                        || self.list_rsplit_v2_active()))
            {
                return None;
            }
            union_a |= it.after;
        }
        if union_a != total_after {
            return None;
        }
        const FRAG_TEX_WRITE_FLAGS: u32 = crate::types::resource_flags::ALLOW_RENDER_TARGET
            | crate::types::resource_flags::ALLOW_DEPTH_STENCIL
            | crate::types::resource_flags::ALLOW_UNORDERED_ACCESS
            | crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        let defer_cs = self.opts.opt_cs_to_ps_overlap || self.opts.opt_cs_to_ps_overlap_v2;
        let defer_gfx = self.gfx_to_ps_overlap_active();
        let dom_prune = !count_stat
            && (self.opts.opt_cs_to_ps_overlap_v2 || self.list_rsplit_v2_active())
            && self.render_dom_exact_active();
        let mid_on = self.mid_qb_stagesync_active();
        let mut pruned_dom_any = false;
        let mut hit_a = MTLStages::empty();
        let mut hit_b = MTLStages::empty();
        let mut hit_r = [0u32; 7];
        let mut rem_a = MTLStages::empty();
        let mut rem_b = MTLStages::empty();
        let mut rem_r = [0u32; 7];
        let mut deferred_fragment = false;
        items.retain_mut(|it| {
            let ib = it.before & PRE_RASTER;
            let eff_ia = if dom_prune {
                let u = self.unsatisfied_enc_after_for_req(it.after, ib, &it.req, mid_on);
                if u != it.after {
                    pruned_dom_any = true;
                }
                u
            } else {
                it.after
            };
            if eff_ia.is_empty() || ib.is_empty() {
                return false;
            }
            let is_sentinel =
                it.r == SOFT_RES_PRE_LIST_FRAG_TEX || it.r == SOFT_RES_PRE_LIST_FRAG_BUF;
            let item_hit = if it.r == SOFT_RES_PRE_LIST_FRAG_TEX {
                !self.list_rsplit_v2_active()
                    || self.rp_sync.cur_draw_pre_exact.iter().any(|&cr| {
                        cr == 0
                            || (res_class(cr) == READS_TEX
                                && (self.bar.written.contains(&cr)
                                    || self.bar.pass_targets.contains(&cr)
                                    || is_frag_resolve_dst(cr)
                                    || unsafe {
                                        let res = &*(cr as *const Resource);
                                        let placed_bad = if self.opts.opt_gfx_overlap_capped
                                            || self.opts.opt_gfx_overlap_frag_cap
                                        {
                                            res.may_alias_placed()
                                        } else {
                                            res.placed_in.is_some()
                                        };
                                        placed_bad || (res.desc.Flags & FRAG_TEX_WRITE_FLAGS != 0)
                                    }))
                    })
            } else if it.r == SOFT_RES_PRE_LIST_FRAG_BUF {
                !self.list_rsplit_v2_active()
                    || self.rp_sync.cur_draw_pre_exact.iter().any(|&cr| {
                        cr == 0
                            || (res_class(cr) == READS_BUF
                                && (self.bar.written.contains(&cr)
                                    || unsafe {
                                        let res = &*(cr as *const Resource);
                                        let placed_bad = if self.opts.opt_gfx_overlap_capped
                                            || self.opts.opt_gfx_overlap_frag_cap
                                        {
                                            res.may_alias_placed()
                                        } else {
                                            res.placed_in.is_some()
                                        };
                                        placed_bad || (res.desc.Flags & FRAG_TEX_WRITE_FLAGS != 0)
                                    }))
                    })
            } else {
                self.rp_sync.cur_draw_pre_exact.contains(&it.r)
            };
            if item_hit {
                hit_a |= eff_ia;
                hit_b |= ib;
                merge_req_for_after(&mut hit_r, eff_ia, &it.req);
                false
            } else {
                let item_defer_gfx = defer_gfx || (is_sentinel && self.list_rsplit_v2_active());
                let (imm_part, def_part) = if defer_cs && item_defer_gfx {
                    (MTLStages::empty(), eff_ia)
                } else if defer_cs {
                    if eff_ia == MTLStages::All {
                        (MTLStages::All, MTLStages::empty())
                    } else {
                        (eff_ia - COMPUTE_STAGES, eff_ia & COMPUTE_STAGES)
                    }
                } else if item_defer_gfx {
                    if eff_ia == MTLStages::All {
                        (MTLStages::empty(), MTLStages::All)
                    } else {
                        (eff_ia & COMPUTE_STAGES, eff_ia - COMPUTE_STAGES)
                    }
                } else {
                    (eff_ia, MTLStages::empty())
                };
                if !imm_part.is_empty() {
                    hit_a |= imm_part;
                    hit_b |= ib;
                    merge_req_for_after(&mut hit_r, imm_part, &it.req);
                }
                if !def_part.is_empty() {
                    deferred_fragment |= !is_sentinel
                        && def_part != MTLStages::All
                        && def_part.contains(MTLStages::Fragment);
                    it.after = def_part;
                    let mut nr = [0u32; 7];
                    merge_req_for_after(&mut nr, def_part, &it.req);
                    it.req = nr;
                    rem_a |= def_part;
                    rem_b |= ib;
                    merge_req_for_after(&mut rem_r, def_part, &it.req);
                    true
                } else {
                    false
                }
            }
        });
        let saved = sub_stages_preserving_all(total_after, hit_a);
        if self.opts.opt_gfx_overlap_frag_cap
            && saved != MTLStages::All
            && saved.contains(MTLStages::Fragment)
            && deferred_fragment
        {
            self.rp_sync.enc_deferred_frag_ps = true;
        }
        if (!saved.is_empty() || pruned_dom_any)
            && (count_stat || !rem_a.is_empty() || pruned_dom_any)
        {
            if self.opts.opt_cs_to_ps_overlap && !saved.is_empty() {
                stats::inc(&stats::c::cs_to_ps_overlap);
            }
            if self.opts.opt_cs_to_ps_overlap_v2
                && ((saved != MTLStages::All && saved.intersects(COMPUTE_STAGES)) || pruned_dom_any)
            {
                stats::inc(&stats::c::cs_to_ps_overlap_v2);
            }
            if self.opts.opt_gfx_overlap_frag_cap
                && (saved == MTLStages::All
                    || saved.intersects(MTLStages::Fragment | PRE_RASTER)
                    || (!self.opts.opt_cs_to_ps_overlap_v2 && pruned_dom_any))
            {
                stats::inc(&stats::c::gfx_overlap_frag_cap);
            } else if self.opts.opt_gfx_overlap_capped
                && (saved == MTLStages::All
                    || saved.intersects(MTLStages::Fragment | PRE_RASTER)
                    || (!self.opts.opt_cs_to_ps_overlap_v2 && pruned_dom_any))
            {
                stats::inc(&stats::c::gfx_overlap_capped);
            } else if self.opts.opt_gfx_to_ps_overlap_v2
                && (saved == MTLStages::All
                    || saved.intersects(MTLStages::Fragment | PRE_RASTER)
                    || (!self.opts.opt_cs_to_ps_overlap_v2 && pruned_dom_any))
            {
                stats::inc(&stats::c::gfx_to_ps_overlap_v2);
            } else if self.opts.opt_gfx_to_ps_overlap
                && (saved == MTLStages::All || saved.intersects(MTLStages::Fragment | PRE_RASTER))
            {
                stats::inc(&stats::c::gfx_to_ps_overlap);
            }
        }
        let rem_opt = (!rem_a.is_empty() && !rem_b.is_empty()).then_some((rem_a, rem_b, rem_r));
        Some((hit_a, hit_b, hit_r, rem_opt))
    }

    pub(super) fn prune_satisfied_vtx_defer(&mut self, mid_on: bool) {
        let mut pruned = false;
        for i in 0..2 {
            let Some((da, db)) = self.bar.vtx_defer[i] else {
                continue;
            };
            let u =
                self.unsatisfied_enc_after_for_req(da, db, &self.bar.vtx_defer_req[i], mid_on);
            if u.is_empty() {
                self.bar.vtx_defer[i] = None;
                self.bar.vtx_defer_req[i] = [0; 7];
                self.bar.vtx_defer_items[i].clear();
                self.bar.vtx_defer_poisoned[i] = false;
                pruned = true;
            } else if u != da {
                self.bar.vtx_defer[i] = Some((u, db));
                let mut nr = [0u32; 7];
                merge_req_for_after(&mut nr, u, &self.bar.vtx_defer_req[i]);
                self.bar.vtx_defer_req[i] = nr;
                if !self.bar.vtx_defer_poisoned[i] {
                    let mut items = std::mem::take(&mut self.bar.vtx_defer_items[i]);
                    items.retain_mut(|it| {
                        let iu = self.unsatisfied_enc_after_for_req(
                            it.after,
                            it.before & PRE_RASTER,
                            &it.req,
                            mid_on,
                        );
                        it.after = iu;
                        !it.after.is_empty() && !it.before.is_empty()
                    });
                    self.bar.vtx_defer_items[i] = items;
                }
                pruned = true;
            }
        }
        if pruned {
            if self.bar.vtx_defer == NO_VTX {
                self.bar.vtx_defer_list_frag = false;
            }
            if self.opts.opt_cs_to_ps_overlap_v2 {
                stats::inc(&stats::c::cs_to_ps_overlap_v2);
            }
            if self.opts.opt_gfx_overlap_frag_cap {
                stats::inc(&stats::c::gfx_overlap_frag_cap);
            } else if self.opts.opt_gfx_overlap_capped {
                stats::inc(&stats::c::gfx_overlap_capped);
            } else if self.opts.opt_gfx_to_ps_overlap_v2 {
                stats::inc(&stats::c::gfx_to_ps_overlap_v2);
            }
        }
    }

    /// Move pending SRV/UAV pre-raster obligations to the next render pass
    /// (the barrier being emitted does not order vertex/object/mesh work).
    pub(super) fn carry_soft(&mut self) {
        let split_on = self.render_split_sync_active();
        for i in 0..2 {
            let soft = std::mem::replace(&mut self.bar.pend_before_soft[i], MTLStages::empty());
            let sa =
                std::mem::replace(&mut self.bar.split_pend.soft_after[i], MTLStages::empty());
            let sb =
                std::mem::replace(&mut self.bar.split_pend.soft_before[i], MTLStages::empty());
            let sr = std::mem::take(&mut self.bar.split_pend.soft_req[i]);
            let sp_items = std::mem::take(&mut self.bar.soft_pend_items[i]);
            let sp_poison = std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
            let pair = if split_on {
                (!sa.is_empty() && !sb.is_empty()).then_some((sa, sb))
            } else {
                (!soft.is_empty() && !self.bar.pend_after.is_empty())
                    .then_some((self.bar.pend_after, soft))
            };
            if let Some(p) = pair {
                if split_on {
                    merge_req_for_after(&mut self.bar.vtx_carry_req[i], p.0, &sr);
                    self.bar.vtx_carry_poisoned[i] |= sp_poison;
                    if !self.bar.vtx_carry_poisoned[i] {
                        Self::extend_soft_items(&mut self.bar.vtx_carry_items[i], sp_items);
                    } else {
                        self.bar.vtx_carry_items[i].clear();
                    }
                } else {
                    merge_req_for_after(&mut self.bar.vtx_carry_req[i], p.0, &self.bar.stage_gen);
                    self.bar.vtx_carry_poisoned[i] = true;
                    self.bar.vtx_carry_items[i].clear();
                }
                self.bar.vtx_carry[i] = merge_vtx(self.bar.vtx_carry[i], Some(p));
            }
        }
    }

    #[inline]
    pub(super) fn cur_pre_raster_stages(&self) -> MTLStages {
        if legacy_barriers() || !barrier_masks() {
            return PRE_RASTER;
        }
        let st = self.bar.opening_pre_raster_stages & PRE_RASTER;
        if st.is_empty() { PRE_RASTER } else { st }
    }

    pub(super) fn ensure_pre_raster_stage_synced(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        pre_b: MTLStages,
    ) {
        let need_st = self.bar.enc_pre_unwaited_stages & pre_b;
        if need_st.is_empty() || self.bar.enc_pre_unwaited_after.is_empty() {
            return;
        }
        let after = self.bar.enc_pre_unwaited_after;
        let req = self.bar.enc_pre_unwaited_req;
        let mid_on = self.mid_qb_stagesync_active();
        let eff_after = if self.render_dom_exact_active() {
            let u = self.unsatisfied_enc_after_for_req(after, need_st, &req, mid_on);
            self.fold_dominated_after(u, &req)
        } else {
            self.unsatisfied_mid_qb_after(after, need_st)
        };
        if !eff_after.is_empty() {
            e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                eff_after,
                need_st,
                MTL4VisibilityOptions::Device,
            );
            let rec_after = if self.opts.opt_rp_keep_write_exact {
                eff_after
            } else {
                after
            };
            self.record_enc_queue_sync(rec_after, need_st);
            stats::inc(&stats::c::rb_mid_qb);
        }
        self.bar.enc_pre_unwaited_stages -= need_st;
        if self.bar.enc_pre_unwaited_stages.is_empty() {
            self.bar.enc_pre_unwaited_after = MTLStages::empty();
            self.bar.enc_pre_unwaited_req = [0; 7];
        }
    }

    pub(super) fn collect_pre_raster_stage_sync(&mut self, pre_b: MTLStages) {
        let need_st = self.bar.enc_pre_unwaited_stages & pre_b;
        if need_st.is_empty() || self.bar.enc_pre_unwaited_after.is_empty() {
            return;
        }
        let after = self.bar.enc_pre_unwaited_after;
        let req = self.bar.enc_pre_unwaited_req;
        let mid_on = self.mid_qb_stagesync_active();
        let eff_after = if self.render_dom_exact_active() {
            let u = self.unsatisfied_enc_after_for_req(after, need_st, &req, mid_on);
            self.fold_dominated_after(u, &req)
        } else {
            self.unsatisfied_mid_qb_after(after, need_st)
        };
        if !eff_after.is_empty() {
            self.push_batch_qbarrier(eff_after, need_st);
            if mid_on {
                let rec_after = if self.opts.opt_rp_keep_write_exact {
                    eff_after
                } else {
                    after
                };
                self.record_enc_queue_sync(rec_after, need_st);
            }
            stats::inc(&stats::c::rb_mid_qb);
        }
        self.bar.enc_pre_unwaited_stages -= need_st;
        if self.bar.enc_pre_unwaited_stages.is_empty() {
            self.bar.enc_pre_unwaited_after = MTLStages::empty();
            self.bar.enc_pre_unwaited_req = [0; 7];
        }
    }

    /// A draw is about to be encoded: the pass's deferred pre-raster
    /// barriers for the resource classes its pre-raster stages may read
    /// (`reads`, [`crate::metal::pipeline::stage_reads_resources`]) land here.
    pub(super) fn sync_pre_raster(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        reads: u8,
        pre_b: MTLStages,
    ) {
        let pre_b = if legacy_barriers() || !barrier_masks() {
            PRE_RASTER
        } else {
            let st = pre_b & PRE_RASTER;
            if st.is_empty() { PRE_RASTER } else { st }
        };
        if reads == 0 || self.bar.vtx_defer == NO_VTX {
            self.finish_post_draw0_list_frag();
            return;
        }
        let mut sync = None;
        let mut sync_req = [0u32; 7];
        for i in 0..2 {
            if reads & (1 << i) != 0 {
                if let Some((da, db)) = self.bar.vtx_defer[i].take() {
                    let dr = std::mem::take(&mut self.bar.vtx_defer_req[i]);
                    let mut d_items = std::mem::take(&mut self.bar.vtx_defer_items[i]);
                    let d_poison = std::mem::replace(&mut self.bar.vtx_defer_poisoned[i], false);
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
            let eff_after = if self.render_dom_exact_active() {
                let u = self.unsatisfied_enc_after_for_req(after, emit_before, &sync_req, mid_on);
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
                self.finish_post_draw0_list_frag();
                return;
            }
            if eff_after != after {
                stats::inc(&stats::c::stagesync_narrow);
            }
            e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                eff_after,
                emit_before,
                MTL4VisibilityOptions::Device,
            );
            let rec_after = if self.opts.opt_rp_keep_write_exact {
                eff_after
            } else {
                after
            };
            self.record_enc_queue_sync(rec_after, emit_before);
            if (self.opts.opt_cs_to_ps_overlap_v2 || self.list_rsplit_v2_active())
                && self.render_dom_exact_active()
            {
                self.prune_satisfied_vtx_defer(mid_on);
            }
            stats::inc(&stats::c::dv_hit);
        }
        self.finish_post_draw0_list_frag();
    }

    #[inline]
    pub(super) fn note_enc_sync_pair(&mut self, qa: MTLStages, qb: MTLStages) {
        if qa.is_empty() || qb.is_empty() {
            return;
        }
        if self.bar.enc_sync_after.is_empty() || self.bar.enc_sync_before.is_empty() {
            self.bar.enc_sync_after = qa;
            self.bar.enc_sync_before = qb;
        } else if self.bar.enc_sync_after.contains(qa) && self.bar.enc_sync_before.contains(qb) {
            return;
        } else if qa.contains(self.bar.enc_sync_after) && qb.contains(self.bar.enc_sync_before) {
            self.bar.enc_sync_after = qa;
            self.bar.enc_sync_before = qb;
        } else if self.bar.enc_sync_before == qb {
            self.bar.enc_sync_after |= qa;
        } else if self.bar.enc_sync_after == qa {
            self.bar.enc_sync_before |= qb;
        } else if self.bar.enc_sync_before.contains(qb) {
            self.bar.enc_sync_after |= qa;
            self.bar.enc_sync_before = qb;
        } else if !self.opts.opt_split_compute_exact {
            self.bar.enc_sync_after |= qa;
            self.bar.enc_sync_before &= qb;
        } else {
            let inter_a = self.bar.enc_sync_after & qa;
            if !inter_a.is_empty() {
                self.bar.enc_sync_after = inter_a;
                self.bar.enc_sync_before |= qb;
            }
        }
    }

    pub(super) fn ib_record_op_tight_exact(
        &mut self,
        op_stage: MTLStages,
        disp_uav: Option<&[usize]>,
        disp_read_or_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) {
        const BOTH_CLASSES: u8 = READS_TEX | READS_BUF;
        if let Some(uavs) = disp_uav {
            for &r in uavs {
                if r != 0 {
                    unsafe { (*(r as *const Resource)).mark_gpu_used() };
                }
            }
        }
        if let Some(rt) = disp_read_or_touch {
            for &r in rt {
                if r != 0 {
                    unsafe { (*(r as *const Resource)).mark_gpu_used() };
                }
            }
        }
        let subrange_on = self.opts.opt_subrange_uav_barrier;
        let use_cls_bounded = self.opts.opt_desc_table_res_exact
            || self.opts.opt_desc_table_cs_fast
            || self.opts.opt_cs_intra_barrier_exact
            || self.opts.opt_cs_intra_barrier_v2
            || self.cs_intra_safe_active()
            || self.hk_v2_active();
        let merge_cls_on = self.opts.opt_cs_intra_barrier_v2 || self.cs_intra_safe_active();
        let merge_cls =
            |uav_cls: &mut [TrackedSet<usize>; 2], read_cls: &mut [TrackedSet<usize>; 2]| {
                let push_filtered = |slot: &mut TrackedSet<usize>, src: &[usize], rc: u8| {
                    if !slot.is_exact() {
                        return;
                    }
                    for &r in src {
                        if r == 0 {
                            slot.invalidate();
                            return;
                        }
                        if (res_class(r) & rc) != 0 {
                            slot.insert_res(r);
                        }
                    }
                };
                for ci_cls in 0..2 {
                    let rc = 1u8 << ci_cls;
                    if let Some(ex) = exact {
                        if (ex.uav_classes & rc) != 0 {
                            if ex.uav_exact {
                                push_filtered(&mut uav_cls[ci_cls], ex.uavs, rc);
                            } else {
                                uav_cls[ci_cls].invalidate();
                            }
                        }
                        if (ex.read_classes & rc) != 0 {
                            if ex.full_read_exact {
                                if let Some(rt) = disp_read_or_touch {
                                    push_filtered(&mut read_cls[ci_cls], rt, rc);
                                } else {
                                    read_cls[ci_cls].invalidate();
                                }
                            } else if use_cls_bounded
                                && ex.bounded_read_exact
                                && (ex.unbounded_srv_classes & rc) == 0
                            {
                                push_filtered(&mut read_cls[ci_cls], ex.bounded_reads, rc);
                                if (ex.uav_classes & rc) != 0 && ex.uav_exact {
                                    push_filtered(&mut read_cls[ci_cls], ex.uavs, rc);
                                }
                            } else {
                                read_cls[ci_cls].invalidate();
                            }
                        }
                    } else {
                        if let Some(uavs) = disp_uav {
                            push_filtered(&mut uav_cls[ci_cls], uavs, rc);
                        } else {
                            uav_cls[ci_cls].invalidate();
                        }
                        if let Some(rt) = disp_read_or_touch {
                            push_filtered(&mut read_cls[ci_cls], rt, rc);
                        } else {
                            read_cls[ci_cls].invalidate();
                        }
                    }
                }
            };
        let util_exact_cls_hk = if exact.is_none() && self.hk_v2_active() {
            match (disp_uav, disp_read_or_touch) {
                (Some(uavs), Some(rts))
                    if uavs.iter().all(|&r| r != 0) && rts.iter().all(|&r| r != 0) =>
                {
                    const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                    let mut u_cls = 0u8;
                    let mut r_cls = 0u8;
                    let mut u_hk = 0u32;
                    let mut ro_hk = 0u32;
                    for &r in uavs {
                        u_cls |= res_class(r);
                        let res = unsafe { &*(r as *const Resource) };
                        u_hk |= if res.desc.Flags & SIMUL != 0 {
                            HK_ALL
                        } else {
                            res.handle_kind_mask()
                        };
                    }
                    for &r in rts {
                        r_cls |= res_class(r);
                        let res = unsafe { &*(r as *const Resource) };
                        ro_hk |= if res.desc.Flags & SIMUL != 0 {
                            HK_ALL
                        } else {
                            res.handle_kind_mask()
                        };
                    }
                    Some((u_cls, u_cls | r_cls, u_hk, ro_hk))
                }
                _ => None,
            }
        } else {
            None
        };
        if let Some((_, _, u_hk, ro_hk)) = util_exact_cls_hk {
            if op_stage.contains(MTLStages::Dispatch) && (u_hk | ro_hk) != HK_ALL {
                self.bump_hk_stat();
            }
        }
        let (add_uav_cls, add_read_cls, add_uav_srv_cls, add_uav_hk, add_ro_hk, add_read_hk, add_uav_srv_hk) =
            if let Some(ex) = exact {
                let ex_ro_hk = ex.hk.ro_srv_kinds
                    | if (ex.exact_read_only_classes & READS_BUF) != 0 {
                        ex.hk.read_kinds & HK_CBV_BUF
                    } else {
                        0
                    };
                (
                    ex.uav_classes,
                    ex.read_classes,
                    ex.uav_srv_classes,
                    ex.hk.uav_write_kinds,
                    ex_ro_hk,
                    ex.hk.read_kinds,
                    ex.hk.uav_srv_kinds,
                )
            } else if let Some((u_cls, rw_cls, u_hk, ro_hk)) = util_exact_cls_hk {
                (u_cls, rw_cls, rw_cls, u_hk, ro_hk, u_hk | ro_hk, u_hk | ro_hk)
            } else {
                (
                    BOTH_CLASSES,
                    BOTH_CLASSES,
                    BOTH_CLASSES,
                    HK_ALL,
                    HK_ALL,
                    HK_ALL,
                    HK_ALL,
                )
            };
        let add_unb_read_cls = if subrange_on {
            if let Some(ex) = exact {
                if ex.bounded_read_spans.is_some() && !ex.full_read_exact {
                    ex.unbounded_srv_classes
                } else if ex.bounded_read_spans.is_none() && disp_read_or_touch.is_none() {
                    BOTH_CLASSES
                } else {
                    0
                }
            } else if disp_read_or_touch.is_none() {
                BOTH_CLASSES
            } else {
                0
            }
        } else {
            0
        };
        let record_spans = |uav_spans: &mut TrackedSet<SubRange>,
                                read_spans: &mut TrackedSet<SubRange>,
                                unb_cls: &mut u8| {
            if !subrange_on {
                return;
            }
            if let Some(us) = exact.and_then(|ex| ex.uav_spans) {
                Self::ib_merge_spans_exact(uav_spans, us);
            } else {
                Self::ib_merge_spans_whole(uav_spans, disp_uav);
            }
            if let Some(rs) = exact.and_then(|ex| ex.bounded_read_spans) {
                Self::ib_merge_spans_exact(read_spans, rs);
            } else {
                Self::ib_merge_spans_whole(read_spans, disp_read_or_touch);
            }
            *unb_cls |= add_unb_read_cls;
        };
        self.ib.ib_stages |= op_stage & COMPUTE_STAGES;
        let sub_narrow = self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light();
        if !sub_narrow {
            if op_stage.contains(MTLStages::Dispatch) {
                Self::ib_merge_set(&mut self.ib.ib_disp_uav, disp_uav);
                Self::ib_merge_set(&mut self.ib.ib_disp_read, disp_read_or_touch);
                if merge_cls_on {
                    merge_cls(&mut self.ib.ib_disp_uav_cls, &mut self.ib.ib_disp_read_cls);
                }
                self.ib.ib_disp_uav_classes |= add_uav_cls;
                self.ib.ib_disp_read_classes |= add_read_cls;
                self.ib.ib_disp_uav_srv_classes |= add_uav_srv_cls;
                self.ib.ib_disp_uav_hk |= add_uav_hk;
                self.ib.ib_disp_ro_hk |= add_ro_hk;
                self.ib.ib_disp_read_hk |= add_read_hk;
                self.ib.ib_disp_uav_srv_hk |= add_uav_srv_hk;
                record_spans(
                    &mut self.ib.ib_disp_uav_spans,
                    &mut self.ib.ib_disp_read_spans,
                    &mut self.ib.ib_unbounded_read_classes,
                );
            }
            if op_stage.contains(MTLStages::Blit) {
                Self::ib_merge_set(&mut self.ib.ib_blit_touch, disp_read_or_touch);
            }
            if op_stage.contains(MTLStages::AccelerationStructure) {
                Self::ib_merge_set(&mut self.ib.ib_as_touch, disp_read_or_touch);
            }
        }
        if self.opts.opt_ib_substage_narrow || self.is_copy_queue_light() {
            for s in &mut self.ib.ib_sub {
                s.stages |= op_stage & COMPUTE_STAGES;
                if op_stage.contains(MTLStages::Dispatch) {
                    Self::ib_merge_set(&mut s.disp_uav, disp_uav);
                    Self::ib_merge_set(&mut s.disp_read, disp_read_or_touch);
                    if merge_cls_on {
                        merge_cls(&mut s.disp_uav_cls, &mut s.disp_read_cls);
                    }
                    s.disp_uav_classes |= add_uav_cls;
                    s.disp_read_classes |= add_read_cls;
                    s.disp_uav_srv_classes |= add_uav_srv_cls;
                    s.disp_uav_hk |= add_uav_hk;
                    s.disp_ro_hk |= add_ro_hk;
                    s.disp_read_hk |= add_read_hk;
                    s.disp_uav_srv_hk |= add_uav_srv_hk;
                    record_spans(
                        &mut s.disp_uav_spans,
                        &mut s.disp_read_spans,
                        &mut s.unbounded_read_classes,
                    );
                }
                if op_stage.contains(MTLStages::Blit) {
                    Self::ib_merge_set(&mut s.blit_touch, disp_read_or_touch);
                }
                if op_stage.contains(MTLStages::AccelerationStructure) {
                    Self::ib_merge_set(&mut s.as_touch, disp_read_or_touch);
                }
            }
        }
    }

    fn collect_buf_uav_spans_from(
        subrange_on: bool,
        disp_uav_spans: &TrackedSet<SubRange>,
        rk: usize,
    ) -> Option<SmallSpans> {
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        if !subrange_on || rk == 0 {
            return None;
        }
        let us = disp_uav_spans.as_slice()?;
        if !disp_uav_spans.maybe_contains_res(rk) {
            return None;
        }
        let res = unsafe { &*(rk as *const Resource) };
        if !res.desc.is_buffer() || (res.desc.Flags & SIMUL) != 0 {
            return None;
        }
        let mut out = SmallSpans::new();
        for &s in us {
            if s.r == rk {
                out.push_coalesced(s);
            }
        }
        (!out.is_empty()).then_some(out)
    }

    fn collect_buf_trans_spans_from(
        subrange_on: bool,
        cs_cleared: &[usize],
        disp_uav_spans: &TrackedSet<SubRange>,
        disp_read_spans: &TrackedSet<SubRange>,
        unbounded_read_classes: u8,
        rk: usize,
        include_read: bool,
    ) -> Option<SmallSpans> {
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        if !subrange_on || rk == 0 || cs_cleared.contains(&rk) {
            return None;
        }
        let res = unsafe { &*(rk as *const Resource) };
        if !res.desc.is_buffer() || (res.desc.Flags & SIMUL) != 0 {
            return None;
        }
        let us = disp_uav_spans.as_slice()?;
        let mut out = SmallSpans::new();
        if disp_uav_spans.maybe_contains_res(rk) {
            for &s in us {
                if s.r == rk {
                    out.push_coalesced(s);
                }
            }
        }
        if include_read {
            if (unbounded_read_classes & READS_BUF) != 0 {
                return None;
            }
            let rs = disp_read_spans.as_slice()?;
            if disp_read_spans.maybe_contains_res(rk) {
                for &s in rs {
                    if s.r == rk {
                        out.push_coalesced(s);
                    }
                }
            }
        }
        (!out.is_empty()).then_some(out)
    }

    pub(super) fn note_compute_barriers_tight(
        &mut self,
        items: &[Barrier],
        elide_r2r: bool,
        full: bool,
    ) {
        use MTLStages as S;
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        if full {
            self.ib_add_obl_tight(0, COMPUTE_STAGES, COMPUTE_STAGES);
            return;
        }
        let exact_obl = self.opts.opt_ib_exact_obl;
        let msc_class = self.msc_class_active();
        let sub_narrow = self.opts.opt_ib_substage_narrow && !self.is_copy_queue_light();
        let subrange_on = self.opts.opt_subrange_uav_barrier;
        let mut pending_adds = std::mem::take(&mut self.bar.bar_scratch_ib_adds);
        let mut pending_uav_adds = std::mem::take(&mut self.bar.bar_scratch_ib_uav_adds);
        for b in items {
            match *b {
                Barrier::SplitBegin(..) => {}
                Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1) => {
                    if elide_r2r
                        && (s0 == s1
                            || (is_read_to_read_transition(s0, s1)
                                && !self
                                    .clears
                                    .pending_clears
                                    .iter()
                                    .any(|p| p.view.resource == r)))
                    {
                        continue;
                    }
                    let (eff_s0, eff_s1) =
                        if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                            (filter_res_state_bits(r, s0), filter_res_state_bits(r, s1))
                        } else {
                            (s0, s1)
                        };
                    let cs_before = if s1 == 0 {
                        COMPUTE_STAGES
                    } else if eff_s1 == 0 {
                        S::empty()
                    } else {
                        state_stages(eff_s1) & COMPUTE_STAGES
                    };
                    if cs_before.is_empty() {
                        continue;
                    }
                    let s0_stages = if s0 == 0 {
                        S::All
                    } else if eff_s0 == 0 {
                        S::empty()
                    } else {
                        state_stages(eff_s0)
                    };
                    let non_simul = r != 0
                        && unsafe {
                            let res = &*(r as *const Resource);
                            let not_aliased = if self.cs_intra_safe_active() {
                                !res.may_alias_placed()
                            } else {
                                res.placed_in.is_none()
                            };
                            not_aliased && (res.desc.Flags & SIMUL == 0)
                        };
                    let cls_idx = usize::from(r != 0 && res_class(r) == READS_BUF);
                    let s0_uav_disp = s0 == 0
                        || (eff_s0 & 0x8) != 0
                        || (r != 0 && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL != 0 });
                    let s0_non_uav_disp = s0 == 0
                        || ((eff_s0 & !0x8) != 0
                            && state_stages(eff_s0 & !0x8).contains(S::Dispatch));
                    if sub_narrow {
                        let allow_generic_read =
                            (self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2)
                                && (eff_s1 & 0x40_0000) == 0;
                        let dst_read_only = s1 != 0
                            && eff_s1 != 0
                            && (eff_s1 & D3D12_WRITE_STATES) == 0
                            && (eff_s1 & !D3D12_READ_ONLY_MASK) == 0;
                        let mut any_cand = false;
                        let mut any_after = false;
                        let mut any_class_filtered = false;
                        let mut any_v2_filtered = false;
                        pending_adds.clear();
                        for &(ci, cbit) in &CS_SUB_BITS {
                            if !cs_before.contains(cbit) {
                                continue;
                            }
                            let sub = &self.ib.ib_sub[ci];
                            let cand_stages = (s0_stages & COMPUTE_STAGES)
                                | if eff_s0 & 0x4 != 0 && !sub.cs_cleared.is_empty() {
                                    S::Dispatch
                                } else {
                                    S::empty()
                                };
                            let active_cand = cand_stages & sub.stages;
                            if active_cand.is_empty() {
                                continue;
                            }
                            any_cand = true;
                            let mut item_after = S::empty();
                            let mut sub_uav_hit = false;
                            let cs_cleared_r =
                                eff_s0 & 0x4 != 0 && (r == 0 || sub.cs_cleared.contains(&r));
                            if active_cand.contains(S::Dispatch) {
                                let mut disp_hit = false;
                                if s0_stages.contains(S::Dispatch) {
                                    let base_uav_hit = sub.disp_uav.hits_res(r);
                                    let base_read_hit = s0_non_uav_disp && sub.disp_read.hits_res(r);
                                    let (uav_hit, read_hit) = if self.opts.opt_cs_intra_barrier_v2
                                        || self.cs_intra_safe_active()
                                    {
                                        let uh = s0_uav_disp
                                            && if non_simul {
                                                sub.disp_uav_cls[cls_idx].hits_res(r)
                                            } else {
                                                base_uav_hit
                                            };
                                        let rh = s0_non_uav_disp
                                            && if non_simul {
                                                sub.disp_read_cls[cls_idx].hits_res(r)
                                            } else {
                                                base_read_hit
                                            };
                                        (uh, rh)
                                    } else {
                                        (base_uav_hit, base_read_hit)
                                    };
                                    sub_uav_hit = uav_hit;
                                    let mut base_raw = base_uav_hit || base_read_hit;
                                    let mut raw_hit = uav_hit || read_hit;
                                    if (raw_hit || base_raw) && msc_class {
                                        let no_hazard = if r == 0 {
                                            sub.disp_read_classes == 0
                                                || (dst_read_only && sub.disp_uav_classes == 0)
                                        } else if unsafe {
                                            (*(r as *const Resource)).desc.Flags & SIMUL == 0
                                        } {
                                            let rc = res_class(r);
                                            (sub.disp_read_classes & rc) == 0
                                                || (dst_read_only
                                                    && (sub.disp_uav_classes & rc) == 0)
                                        } else {
                                            false
                                        };
                                        if no_hazard {
                                            base_raw = false;
                                            if raw_hit {
                                                raw_hit = false;
                                                any_class_filtered = true;
                                            }
                                        }
                                    }
                                    if raw_hit && self.hk_any_active() && r != 0 {
                                        let res = unsafe { &*(r as *const Resource) };
                                        if res.desc.Flags & SIMUL == 0 {
                                            let rhk = res.handle_kind_mask();
                                            let eff_read_hk = if self.hk_v2_active()
                                                && !s0_uav_disp
                                                && s0_non_uav_disp
                                            {
                                                sub.disp_ro_hk
                                            } else {
                                                sub.disp_read_hk
                                            };
                                            if (eff_read_hk & rhk) == 0
                                                || (dst_read_only && (sub.disp_uav_hk & rhk) == 0)
                                            {
                                                raw_hit = false;
                                                base_raw = false;
                                                any_class_filtered = true;
                                                self.bump_hk_stat();
                                            }
                                        }
                                    }
                                    if base_raw && !raw_hit {
                                        any_v2_filtered = true;
                                    }
                                    disp_hit |= raw_hit;
                                }
                                if cs_cleared_r {
                                    disp_hit = true;
                                }
                                if disp_hit {
                                    item_after |= S::Dispatch;
                                }
                            }
                            if active_cand.contains(S::Blit) && sub.blit_touch.hits_res(r) {
                                item_after |= S::Blit;
                            }
                            if active_cand.contains(S::AccelerationStructure)
                                && sub.as_touch.hits_res(r)
                            {
                                item_after |= S::AccelerationStructure;
                            }
                            if !item_after.is_empty() {
                                any_after = true;
                                let pure_war_uav = self.opts.opt_cs_intra_barrier_v2
                                    && exact_obl
                                    && non_simul
                                    && s0 != 0
                                    && (eff_s0 & D3D12_WRITE_STATES) == 0
                                    && !sub_uav_hit
                                    && !cs_cleared_r
                                    && s1 != 0
                                    && eff_s1 != 0
                                    && (eff_s1 & 0x8) != 0
                                    && (eff_s1 & !(0x8 | 0x1 | 0x40 | 0x80 | 0x400 | 0x800)) == 0
                                    && cbit == S::Dispatch;
                                let kind = if pure_war_uav {
                                    stats::inc(&stats::c::cs_intra_barrier_v2);
                                    OblKind::UavWriteOnly
                                } else if exact_obl
                                    && r != 0
                                    && dst_read_only
                                    && (allow_generic_read
                                        || ((eff_s1 & 0x200) == 0 && cbit == S::Dispatch))
                                    && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                                {
                                    if eff_s1 & 0x40 == 0 {
                                        OblKind::UavOrCbv
                                    } else {
                                        OblKind::ClassSrv(res_class(r))
                                    }
                                } else if (self.opts.opt_desc_table_res_exact
                                    || self.opts.opt_desc_table_cs_fast
                                    || self.opts.opt_cs_intra_barrier_exact
                                    || self.opts.opt_cs_intra_barrier_v2
                                    || self.cs_intra_safe_active())
                                    && exact_obl
                                    && r != 0
                                    && s1 != 0
                                    && eff_s1 != 0
                                    && cbit == S::Dispatch
                                    && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                                {
                                    let bump = || {
                                        if self.cs_intra_safe_active() {
                                            self.bump_cs_intra_stat();
                                        } else if self.opts.opt_desc_table_res_exact {
                                            stats::inc(&stats::c::desc_table_cs_exact);
                                        } else if self.opts.opt_desc_table_cs_fast {
                                            stats::inc(&stats::c::desc_table_cs_fast);
                                        } else if self.opts.opt_cs_intra_barrier_exact {
                                            stats::inc(&stats::c::cs_intra_barrier_exact);
                                        } else {
                                            stats::inc(&stats::c::cs_intra_barrier_v2);
                                        }
                                    };
                                    if (eff_s1 & !(0x8 | 0x400 | 0x800)) == 0 {
                                        bump();
                                        OblKind::UavOnly
                                    } else if (eff_s1 & !(0x8 | 0x1 | 0x400 | 0x800)) == 0 {
                                        bump();
                                        OblKind::UavOrCbv
                                    } else if (eff_s1
                                        & !(0x8 | 0x1 | 0x40 | 0x80 | 0x400 | 0x800))
                                        == 0
                                    {
                                        bump();
                                        OblKind::ClassSrv(res_class(r))
                                    } else {
                                        OblKind::Any
                                    }
                                } else {
                                    OblKind::Any
                                };
                                let spans = if item_after == S::Dispatch {
                                    Self::collect_buf_trans_spans_from(
                                        subrange_on,
                                        &sub.cs_cleared,
                                        &sub.disp_uav_spans,
                                        &sub.disp_read_spans,
                                        sub.unbounded_read_classes,
                                        r,
                                        s0_non_uav_disp,
                                    )
                                } else {
                                    None
                                };
                                pending_adds.push((item_after, cbit, kind, spans));
                            }
                        }
                        if any_v2_filtered {
                            if self.cs_intra_safe_active() {
                                self.bump_cs_intra_stat();
                            } else {
                                stats::inc(&stats::c::cs_intra_barrier_v2);
                            }
                        }
                        if !any_cand {
                            stats::inc(&stats::c::ib_skip_stage);
                        } else if !any_after {
                            if any_class_filtered {
                                stats::inc(&stats::c::ib_class_skip);
                            }
                            stats::inc(&stats::c::ib_skip_touch);
                        } else {
                            for (ia, cb, kd, sp) in pending_adds.drain(..) {
                                self.ib_add_obl_spans_tight(r, ia, cb, kd, sp);
                            }
                        }
                        continue;
                    }
                    let cand_stages = (s0_stages & COMPUTE_STAGES)
                        | if eff_s0 & 0x4 != 0 && !self.ib.ib_cs_cleared.is_empty() {
                            S::Dispatch
                        } else {
                            S::empty()
                        };
                    let active_cand = cand_stages & self.ib.ib_stages;
                    if active_cand.is_empty() {
                        stats::inc(&stats::c::ib_skip_stage);
                        continue;
                    }
                    let mut item_after = S::empty();
                    let mut class_filtered = false;
                    let mut v2_filtered = false;
                    let mut top_uav_hit = false;
                    let cs_cleared_r =
                        eff_s0 & 0x4 != 0 && (r == 0 || self.ib.ib_cs_cleared.contains(&r));
                    if active_cand.contains(S::Dispatch) {
                        let mut disp_hit = false;
                        if s0_stages.contains(S::Dispatch) {
                            let base_uav_hit = self.ib.ib_disp_uav.as_slice().is_none_or(|v| {
                                if r == 0 {
                                    !v.is_empty()
                                } else {
                                    v.contains(&r)
                                }
                            });
                            let base_read_hit = s0_non_uav_disp
                                && self.ib.ib_disp_read.as_slice().is_none_or(|v| {
                                    if r == 0 {
                                        !v.is_empty()
                                    } else {
                                        v.contains(&r)
                                    }
                                });
                            let (uav_hit, read_hit) = if self.opts.opt_cs_intra_barrier_v2
                                || self.cs_intra_safe_active()
                            {
                                let uh = s0_uav_disp
                                    && if non_simul {
                                        self.ib.ib_disp_uav_cls[cls_idx]
                                            .as_slice()
                                            .is_none_or(|v| v.contains(&r))
                                    } else {
                                        base_uav_hit
                                    };
                                let rh = s0_non_uav_disp
                                    && if non_simul {
                                        self.ib.ib_disp_read_cls[cls_idx]
                                            .as_slice()
                                            .is_none_or(|v| v.contains(&r))
                                    } else {
                                        base_read_hit
                                    };
                                (uh, rh)
                            } else {
                                (base_uav_hit, base_read_hit)
                            };
                            top_uav_hit = uav_hit;
                            let mut base_raw = base_uav_hit || base_read_hit;
                            let mut raw_hit = uav_hit || read_hit;
                            let dst_read_only = s1 != 0
                                && eff_s1 != 0
                                && (eff_s1 & D3D12_WRITE_STATES) == 0
                                && (eff_s1 & !D3D12_READ_ONLY_MASK) == 0;
                            if (raw_hit || base_raw) && msc_class {
                                let no_hazard = if r == 0 {
                                    self.ib.ib_disp_read_classes == 0
                                        || (dst_read_only && self.ib.ib_disp_uav_classes == 0)
                                } else if unsafe {
                                    (*(r as *const Resource)).desc.Flags & SIMUL == 0
                                } {
                                    let rc = res_class(r);
                                    (self.ib.ib_disp_read_classes & rc) == 0
                                        || (dst_read_only && (self.ib.ib_disp_uav_classes & rc) == 0)
                                } else {
                                    false
                                };
                                if no_hazard {
                                    base_raw = false;
                                    if raw_hit {
                                        raw_hit = false;
                                        class_filtered = true;
                                    }
                                }
                            }
                            if raw_hit && self.hk_any_active() && r != 0 {
                                let res = unsafe { &*(r as *const Resource) };
                                if res.desc.Flags & SIMUL == 0 {
                                    let rhk = res.handle_kind_mask();
                                    let eff_read_hk =
                                        if self.hk_v2_active() && !s0_uav_disp && s0_non_uav_disp {
                                            self.ib.ib_disp_ro_hk
                                        } else {
                                            self.ib.ib_disp_read_hk
                                        };
                                    if (eff_read_hk & rhk) == 0
                                        || (dst_read_only && (self.ib.ib_disp_uav_hk & rhk) == 0)
                                    {
                                        raw_hit = false;
                                        base_raw = false;
                                        class_filtered = true;
                                        self.bump_hk_stat();
                                    }
                                }
                            }
                            if base_raw && !raw_hit {
                                v2_filtered = true;
                            }
                            disp_hit |= raw_hit;
                        }
                        if cs_cleared_r {
                            disp_hit = true;
                        }
                        if disp_hit {
                            item_after |= S::Dispatch;
                        }
                    }
                    if active_cand.contains(S::Blit)
                        && self.ib.ib_blit_touch.as_slice().is_none_or(|v| {
                            if r == 0 {
                                !v.is_empty()
                            } else {
                                v.contains(&r)
                            }
                        })
                    {
                        item_after |= S::Blit;
                    }
                    if active_cand.contains(S::AccelerationStructure)
                        && self.ib.ib_as_touch.as_slice().is_none_or(|v| {
                            if r == 0 {
                                !v.is_empty()
                            } else {
                                v.contains(&r)
                            }
                        })
                    {
                        item_after |= S::AccelerationStructure;
                    }
                    if v2_filtered {
                        if self.cs_intra_safe_active() {
                            self.bump_cs_intra_stat();
                        } else {
                            stats::inc(&stats::c::cs_intra_barrier_v2);
                        }
                    }
                    if item_after.is_empty() {
                        if class_filtered {
                            stats::inc(&stats::c::ib_class_skip);
                        }
                        stats::inc(&stats::c::ib_skip_touch);
                    } else {
                        let allow_generic_read =
                            (self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2)
                                && (eff_s1 & 0x40_0000) == 0;
                        let pure_war_uav = self.opts.opt_cs_intra_barrier_v2
                            && exact_obl
                            && non_simul
                            && s0 != 0
                            && (eff_s0 & D3D12_WRITE_STATES) == 0
                            && !top_uav_hit
                            && !cs_cleared_r
                            && s1 != 0
                            && eff_s1 != 0
                            && (eff_s1 & 0x8) != 0
                            && (eff_s1 & !(0x8 | 0x1 | 0x40 | 0x80)) == 0
                            && (cs_before - S::Dispatch).is_empty();
                        let kind = if pure_war_uav {
                            stats::inc(&stats::c::cs_intra_barrier_v2);
                            OblKind::UavWriteOnly
                        } else if exact_obl
                            && r != 0
                            && s1 != 0
                            && eff_s1 != 0
                            && (eff_s1 & D3D12_WRITE_STATES) == 0
                            && (eff_s1 & !D3D12_READ_ONLY_MASK) == 0
                            && (allow_generic_read
                                || ((eff_s1 & 0x200) == 0 && (cs_before - S::Dispatch).is_empty()))
                            && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                        {
                            if eff_s1 & 0x40 == 0 {
                                OblKind::UavOrCbv
                            } else {
                                OblKind::ClassSrv(res_class(r))
                            }
                        } else if (self.opts.opt_desc_table_res_exact
                            || self.opts.opt_desc_table_cs_fast
                            || self.opts.opt_cs_intra_barrier_exact
                            || self.opts.opt_cs_intra_barrier_v2
                            || self.cs_intra_safe_active())
                            && exact_obl
                            && r != 0
                            && s1 != 0
                            && eff_s1 != 0
                            && (cs_before - S::Dispatch).is_empty()
                            && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                        {
                            let bump = || {
                                if self.cs_intra_safe_active() {
                                    self.bump_cs_intra_stat();
                                } else if self.opts.opt_desc_table_res_exact {
                                    stats::inc(&stats::c::desc_table_cs_exact);
                                } else if self.opts.opt_desc_table_cs_fast {
                                    stats::inc(&stats::c::desc_table_cs_fast);
                                } else if self.opts.opt_cs_intra_barrier_exact {
                                    stats::inc(&stats::c::cs_intra_barrier_exact);
                                } else {
                                    stats::inc(&stats::c::cs_intra_barrier_v2);
                                }
                            };
                            if (eff_s1 & !0x8) == 0 {
                                bump();
                                OblKind::UavOnly
                            } else if (eff_s1 & !(0x8 | 0x1)) == 0 {
                                bump();
                                OblKind::UavOrCbv
                            } else if (eff_s1 & !(0x8 | 0x1 | 0x40 | 0x80)) == 0 {
                                bump();
                                OblKind::ClassSrv(res_class(r))
                            } else {
                                OblKind::Any
                            }
                        } else {
                            OblKind::Any
                        };
                        let spans = if item_after == S::Dispatch {
                            Self::collect_buf_trans_spans_from(
                                subrange_on,
                                &self.ib.ib_cs_cleared,
                                &self.ib.ib_disp_uav_spans,
                                &self.ib.ib_disp_read_spans,
                                self.ib.ib_unbounded_read_classes,
                                r,
                                s0_non_uav_disp,
                            )
                        } else {
                            None
                        };
                        self.ib_add_obl_spans_tight(r, item_after, cs_before, kind, spans);
                    }
                }
                Barrier::Uav(r) => {
                    const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                    let non_simul = r != 0
                        && unsafe {
                            let res = &*(r as *const Resource);
                            let not_aliased = if self.cs_intra_safe_active() {
                                !res.may_alias_placed()
                            } else {
                                res.placed_in.is_none()
                            };
                            not_aliased && (res.desc.Flags & SIMUL == 0)
                        };
                    let cls_idx = usize::from(r != 0 && res_class(r) == READS_BUF);
                    let uav_unb_exact = self.opts.opt_ib_uav0_unbounded_exact
                        || self.opts.opt_cs_intra_barrier_exact
                        || self.opts.opt_cs_intra_barrier_v2
                        || self.cs_intra_safe_active();
                    if sub_narrow {
                        let mut any_cand = false;
                        let mut any_after = false;
                        let mut any_class_filtered = false;
                        let mut any_v2 = false;
                        let uav_cap = if uav_unb_exact { 512 } else { 32 };
                        let mut expanded_v44 = false;
                        pending_uav_adds.clear();
                        for &(ci, cbit) in &[(0usize, S::Dispatch), (2usize, S::AccelerationStructure)]
                        {
                            let sub = &self.ib.ib_sub[ci];
                            let cand = (S::Dispatch | S::AccelerationStructure) & sub.stages;
                            if cand.is_empty() {
                                continue;
                            }
                            any_cand = true;
                            let mut item_after = S::empty();
                            if cand.contains(S::Dispatch) {
                                let base_hit = sub.disp_uav.as_slice().is_none_or(|v| {
                                    if r == 0 {
                                        !v.is_empty()
                                    } else {
                                        v.contains(&r)
                                    }
                                });
                                let hit = if (self.opts.opt_cs_intra_barrier_v2
                                    || self.cs_intra_safe_active())
                                    && non_simul
                                {
                                    sub.disp_uav_cls[cls_idx]
                                        .as_slice()
                                        .is_none_or(|v| v.contains(&r))
                                } else {
                                    base_hit
                                };
                                let no_touch = msc_class
                                    && if r == 0 {
                                        sub.disp_uav_srv_classes == 0
                                    } else if unsafe {
                                        (*(r as *const Resource)).desc.Flags & SIMUL == 0
                                    } {
                                        (sub.disp_uav_srv_classes & res_class(r)) == 0
                                    } else {
                                        false
                                    };
                                let hk_no_touch = self.hk_any_active()
                                    && r != 0
                                    && unsafe {
                                        let res = &*(r as *const Resource);
                                        (res.desc.Flags & SIMUL == 0)
                                            && (sub.disp_uav_srv_hk & res.handle_kind_mask()) == 0
                                    };
                                if base_hit && !hit && !(no_touch || hk_no_touch) {
                                    any_v2 = true;
                                }
                                if hit {
                                    if no_touch || hk_no_touch {
                                        if !no_touch && hk_no_touch {
                                            self.bump_hk_stat();
                                        }
                                        any_class_filtered = true;
                                    } else {
                                        item_after |= S::Dispatch;
                                    }
                                }
                            }
                            if cand.contains(S::AccelerationStructure)
                                && sub.as_touch.as_slice().is_none_or(|v| {
                                    if r == 0 {
                                        !v.is_empty()
                                    } else {
                                        v.contains(&r)
                                    }
                                })
                            {
                                item_after |= S::AccelerationStructure;
                            }
                            if !item_after.is_empty() {
                                any_after = true;
                                if exact_obl
                                    && r == 0
                                    && item_after.contains(S::Dispatch)
                                    && (uav_unb_exact || item_after == S::Dispatch)
                                {
                                    let expand_uavs =
                                        match (sub.disp_uav.as_slice(), sub.as_touch.as_slice()) {
                                            (Some(uavs), Some(as_v))
                                                if (uav_unb_exact || as_v.is_empty())
                                                    && !uavs.is_empty()
                                                    && uavs.len() <= uav_cap
                                                    && uavs.iter().all(|&rk| {
                                                        rk != 0
                                                            && unsafe {
                                                                (*(rk as *const Resource))
                                                                    .desc
                                                                    .Flags
                                                                    & SIMUL
                                                                    == 0
                                                            }
                                                    }) =>
                                            {
                                                Some((uavs, !as_v.is_empty()))
                                            }
                                            (Some(uavs), None)
                                                if uav_unb_exact
                                                    && !uavs.is_empty()
                                                    && uavs.len() <= uav_cap
                                                    && uavs.iter().all(|&rk| {
                                                        rk != 0
                                                            && unsafe {
                                                                (*(rk as *const Resource))
                                                                    .desc
                                                                    .Flags
                                                                    & SIMUL
                                                                    == 0
                                                            }
                                                    }) =>
                                            {
                                                Some((uavs, true))
                                            }
                                            _ => None,
                                        };
                                    if let Some((uavs, had_as_touch)) = expand_uavs {
                                        if uav_unb_exact
                                            && (uavs.len() > 32
                                                || item_after != S::Dispatch
                                                || had_as_touch)
                                        {
                                            expanded_v44 = true;
                                        }
                                        for &rk in uavs {
                                            let spans = Self::collect_buf_uav_spans_from(
                                                subrange_on,
                                                &sub.disp_uav_spans,
                                                rk,
                                            );
                                            pending_uav_adds.push((
                                                rk,
                                                S::Dispatch,
                                                cbit,
                                                OblKind::UavOnly,
                                                spans,
                                            ));
                                        }
                                        if item_after.contains(S::AccelerationStructure) {
                                            pending_uav_adds.push((
                                                0,
                                                S::AccelerationStructure,
                                                cbit,
                                                OblKind::Any,
                                                None,
                                            ));
                                        }
                                    } else if self.opts.opt_cs_intra_barrier_v2
                                        && !sub.disp_uav.is_exact()
                                        && sub.cs_cleared.is_empty()
                                    {
                                        let mut emitted_disp = false;
                                        for ci_cls in 0..2 {
                                            let rc = 1u8 << ci_cls;
                                            if (sub.disp_uav_classes & rc) == 0 {
                                                continue;
                                            }
                                            emitted_disp = true;
                                            if let Some(uavs) = sub.disp_uav_cls[ci_cls]
                                                .as_slice()
                                                .filter(|uavs| {
                                                    uavs.len() <= uav_cap
                                                        && uavs.iter().all(|&rk| {
                                                            rk != 0
                                                                && unsafe {
                                                                    (*(rk as *const Resource))
                                                                        .desc
                                                                        .Flags
                                                                        & SIMUL
                                                                        == 0
                                                                }
                                                        })
                                                })
                                            {
                                                any_v2 = true;
                                                for &rk in uavs {
                                                    let spans = Self::collect_buf_uav_spans_from(
                                                        subrange_on,
                                                        &sub.disp_uav_spans,
                                                        rk,
                                                    );
                                                    pending_uav_adds.push((
                                                        rk,
                                                        S::Dispatch,
                                                        cbit,
                                                        OblKind::UavOnly,
                                                        spans,
                                                    ));
                                                }
                                            } else if sub.disp_uav_classes
                                                != (READS_TEX | READS_BUF)
                                            {
                                                any_v2 = true;
                                                pending_uav_adds.push((
                                                    0,
                                                    S::Dispatch,
                                                    cbit,
                                                    OblKind::ClassUavOnly(rc),
                                                    None,
                                                ));
                                            } else {
                                                pending_uav_adds.push((
                                                    0,
                                                    S::Dispatch,
                                                    cbit,
                                                    OblKind::UavOnly,
                                                    None,
                                                ));
                                            }
                                        }
                                        if !emitted_disp {
                                            any_v2 = true;
                                        }
                                        if item_after.contains(S::AccelerationStructure) {
                                            pending_uav_adds.push((
                                                0,
                                                S::AccelerationStructure,
                                                cbit,
                                                OblKind::Any,
                                                None,
                                            ));
                                        }
                                    } else {
                                        pending_uav_adds.push((
                                            0,
                                            item_after,
                                            cbit,
                                            OblKind::UavOnly,
                                            None,
                                        ));
                                    }
                                } else {
                                    let kind = if exact_obl
                                        && (r == 0
                                            || unsafe {
                                                (*(r as *const Resource)).desc.Flags & SIMUL == 0
                                            })
                                    {
                                        OblKind::UavOnly
                                    } else {
                                        OblKind::Any
                                    };
                                    let spans = if item_after == S::Dispatch {
                                        Self::collect_buf_uav_spans_from(
                                            subrange_on,
                                            &sub.disp_uav_spans,
                                            r,
                                        )
                                    } else {
                                        None
                                    };
                                    pending_uav_adds.push((r, item_after, cbit, kind, spans));
                                }
                            }
                        }
                        if any_v2 {
                            if self.cs_intra_safe_active() {
                                self.bump_cs_intra_stat();
                            } else {
                                stats::inc(&stats::c::cs_intra_barrier_v2);
                            }
                        }
                        if expanded_v44 {
                            if self.cs_intra_safe_active() {
                                self.bump_cs_intra_stat();
                            } else if self.opts.opt_ib_uav0_unbounded_exact {
                                stats::inc(&stats::c::ib_uav0_exact);
                            } else if self.opts.opt_cs_intra_barrier_exact {
                                stats::inc(&stats::c::cs_intra_barrier_exact);
                            } else {
                                stats::inc(&stats::c::cs_intra_barrier_v2);
                            }
                        }
                        if !any_cand {
                            stats::inc(&stats::c::ib_skip_stage);
                        } else if !any_after {
                            if any_class_filtered {
                                stats::inc(&stats::c::ib_class_skip);
                            }
                            stats::inc(&stats::c::ib_skip_touch);
                        } else {
                            for (rk, ia, cb, kd, sp) in pending_uav_adds.drain(..) {
                                self.ib_add_obl_spans_tight(rk, ia, cb, kd, sp);
                            }
                        }
                        continue;
                    }
                    let uav_cap = if uav_unb_exact { 512 } else { 32 };
                    let cand = (S::Dispatch | S::AccelerationStructure) & self.ib.ib_stages;
                    if cand.is_empty() {
                        stats::inc(&stats::c::ib_skip_stage);
                        continue;
                    }
                    let mut item_after = S::empty();
                    let mut class_filtered = false;
                    if cand.contains(S::Dispatch) {
                        let base_hit = self.ib.ib_disp_uav.as_slice().is_none_or(|v| {
                            if r == 0 {
                                !v.is_empty()
                            } else {
                                v.contains(&r)
                            }
                        });
                        let hit = if (self.opts.opt_cs_intra_barrier_v2
                            || self.cs_intra_safe_active())
                            && non_simul
                        {
                            self.ib.ib_disp_uav_cls[cls_idx]
                                .as_slice()
                                .is_none_or(|v| v.contains(&r))
                        } else {
                            base_hit
                        };
                        let no_touch = msc_class
                            && if r == 0 {
                                self.ib.ib_disp_uav_srv_classes == 0
                            } else if unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 } {
                                (self.ib.ib_disp_uav_srv_classes & res_class(r)) == 0
                            } else {
                                false
                            };
                        let hk_no_touch = self.hk_any_active()
                            && r != 0
                            && unsafe {
                                let res = &*(r as *const Resource);
                                (res.desc.Flags & SIMUL == 0)
                                    && (self.ib.ib_disp_uav_srv_hk & res.handle_kind_mask()) == 0
                            };
                        if base_hit && !hit && !(no_touch || hk_no_touch) {
                            if self.cs_intra_safe_active() {
                                self.bump_cs_intra_stat();
                            } else {
                                stats::inc(&stats::c::cs_intra_barrier_v2);
                            }
                        }
                        if hit {
                            if no_touch || hk_no_touch {
                                if !no_touch && hk_no_touch {
                                    self.bump_hk_stat();
                                }
                                class_filtered = true;
                            } else {
                                item_after |= S::Dispatch;
                            }
                        }
                    }
                    if cand.contains(S::AccelerationStructure)
                        && self.ib.ib_as_touch.as_slice().is_none_or(|v| {
                            if r == 0 {
                                !v.is_empty()
                            } else {
                                v.contains(&r)
                            }
                        })
                    {
                        item_after |= S::AccelerationStructure;
                    }
                    if item_after.is_empty() {
                        if class_filtered {
                            stats::inc(&stats::c::ib_class_skip);
                        }
                        stats::inc(&stats::c::ib_skip_touch);
                    } else if exact_obl
                        && r == 0
                        && item_after.contains(S::Dispatch)
                        && (uav_unb_exact || item_after == S::Dispatch)
                    {
                        let expand_uavs =
                            match (self.ib.ib_disp_uav.as_slice(), self.ib.ib_as_touch.as_slice()) {
                                (Some(uavs), Some(as_v))
                                    if (uav_unb_exact || as_v.is_empty())
                                        && !uavs.is_empty()
                                        && uavs.len() <= uav_cap
                                        && uavs.iter().all(|&rk| {
                                            rk != 0
                                                && unsafe {
                                                    (*(rk as *const Resource)).desc.Flags & SIMUL
                                                        == 0
                                                }
                                        }) =>
                                {
                                    Some((uavs.to_vec(), !as_v.is_empty()))
                                }
                                (Some(uavs), None)
                                    if uav_unb_exact
                                        && !uavs.is_empty()
                                        && uavs.len() <= uav_cap
                                        && uavs.iter().all(|&rk| {
                                            rk != 0
                                                && unsafe {
                                                    (*(rk as *const Resource)).desc.Flags & SIMUL
                                                        == 0
                                                }
                                        }) =>
                                {
                                    Some((uavs.to_vec(), true))
                                }
                                _ => None,
                            };
                        if let Some((uavs, had_as_touch)) = expand_uavs {
                            if uav_unb_exact
                                && (uavs.len() > 32 || item_after != S::Dispatch || had_as_touch)
                            {
                                if self.cs_intra_safe_active() {
                                    self.bump_cs_intra_stat();
                                } else if self.opts.opt_ib_uav0_unbounded_exact {
                                    stats::inc(&stats::c::ib_uav0_exact);
                                } else if self.opts.opt_cs_intra_barrier_exact {
                                    stats::inc(&stats::c::cs_intra_barrier_exact);
                                } else {
                                    stats::inc(&stats::c::cs_intra_barrier_v2);
                                }
                            }
                            for rk in uavs {
                                let spans = Self::collect_buf_uav_spans_from(
                                    subrange_on,
                                    &self.ib.ib_disp_uav_spans,
                                    rk,
                                );
                                self.ib_add_obl_spans_tight(
                                    rk,
                                    S::Dispatch,
                                    S::Dispatch | S::AccelerationStructure,
                                    OblKind::UavOnly,
                                    spans,
                                );
                            }
                            if item_after.contains(S::AccelerationStructure) {
                                self.ib_add_obl_kind_tight(
                                    0,
                                    S::AccelerationStructure,
                                    S::Dispatch | S::AccelerationStructure,
                                    OblKind::Any,
                                );
                            }
                        } else if self.opts.opt_cs_intra_barrier_v2
                            && !self.ib.ib_disp_uav.is_exact()
                            && self.ib.ib_cs_cleared.is_empty()
                        {
                            let mut any_v2 = false;
                            for ci_cls in 0..2 {
                                let rc = 1u8 << ci_cls;
                                if (self.ib.ib_disp_uav_classes & rc) == 0 {
                                    continue;
                                }
                                let uav_cls_opt = self.ib.ib_disp_uav_cls[ci_cls]
                                    .as_slice()
                                    .filter(|uavs| {
                                        uavs.len() <= uav_cap
                                            && uavs.iter().all(|&rk| {
                                                rk != 0
                                                    && unsafe {
                                                        (*(rk as *const Resource)).desc.Flags
                                                            & SIMUL
                                                            == 0
                                                    }
                                            })
                                    })
                                    .map(|uavs| uavs.to_vec());
                                if let Some(uavs) = uav_cls_opt {
                                    any_v2 = true;
                                    for rk in uavs {
                                        let spans = Self::collect_buf_uav_spans_from(
                                            subrange_on,
                                            &self.ib.ib_disp_uav_spans,
                                            rk,
                                        );
                                        self.ib_add_obl_spans_tight(
                                            rk,
                                            S::Dispatch,
                                            S::Dispatch | S::AccelerationStructure,
                                            OblKind::UavOnly,
                                            spans,
                                        );
                                    }
                                } else if self.ib.ib_disp_uav_classes != (READS_TEX | READS_BUF) {
                                    any_v2 = true;
                                    self.ib_add_obl_kind_tight(
                                        0,
                                        S::Dispatch,
                                        S::Dispatch | S::AccelerationStructure,
                                        OblKind::ClassUavOnly(rc),
                                    );
                                } else {
                                    self.ib_add_obl_kind_tight(
                                        0,
                                        S::Dispatch,
                                        S::Dispatch | S::AccelerationStructure,
                                        OblKind::UavOnly,
                                    );
                                }
                            }
                            if any_v2 {
                                stats::inc(&stats::c::cs_intra_barrier_v2);
                            }
                            if item_after.contains(S::AccelerationStructure) {
                                self.ib_add_obl_kind_tight(
                                    0,
                                    S::AccelerationStructure,
                                    S::Dispatch | S::AccelerationStructure,
                                    OblKind::Any,
                                );
                            }
                        } else {
                            self.ib_add_obl_kind_tight(
                                0,
                                item_after,
                                S::Dispatch | S::AccelerationStructure,
                                OblKind::UavOnly,
                            );
                        }
                    } else {
                        let kind = if exact_obl
                            && (r == 0
                                || unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 })
                        {
                            OblKind::UavOnly
                        } else {
                            OblKind::Any
                        };
                        let spans = if item_after == S::Dispatch {
                            Self::collect_buf_uav_spans_from(
                                subrange_on,
                                &self.ib.ib_disp_uav_spans,
                                r,
                            )
                        } else {
                            None
                        };
                        self.ib_add_obl_spans_tight(
                            r,
                            item_after,
                            S::Dispatch | S::AccelerationStructure,
                            kind,
                            spans,
                        );
                    }
                }
                Barrier::Full => {
                    self.ib_add_obl_tight(0, COMPUTE_STAGES, COMPUTE_STAGES);
                }
            }
        }
        pending_adds.clear();
        self.bar.bar_scratch_ib_adds = pending_adds;
        pending_uav_adds.clear();
        self.bar.bar_scratch_ib_uav_adds = pending_uav_adds;
    }

    /// Whether `items` can be ordered inside the open draw pass (see
    /// `barrier_list`).
    pub(super) fn render_barrier_keeps(&self, items: &[Barrier], elide_r2r: bool) -> bool {
        const UAV: u32 = 0x8;
        const STREAM_OUT: u32 = 0x100;
        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
        let open_rtvs = self.active_rtvs();
        let open_dsv = self.active_dsv();
        self.gfx.draw_pass
            && items.iter().all(|b| match *b {
                Barrier::SplitBegin(r, _, s0, s1) => {
                    if elide_r2r
                        && (s0 == s1
                            || (is_read_to_read_transition(s0, s1)
                                && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                                && (!self.ps_to_cs_active()
                                    || self.cs_disp_passthrough_for(r).is_empty())))
                    {
                        return true;
                    }
                    r != 0
                        && !self.bar.pass_targets.contains(&r)
                        && !open_rtvs.iter().any(|v| v.resource == r)
                        && !open_dsv.is_some_and(|v| v.resource == r)
                        && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                }
                Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1) => {
                    if elide_r2r
                        && (s0 == s1
                            || (is_read_to_read_transition(s0, s1)
                                && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                                && (!self.ps_to_cs_active()
                                    || self.cs_disp_passthrough_for(r).is_empty())))
                    {
                        return true;
                    }
                    let uav_written_here = self.rp_sync.enc_uav_write_unbounded
                        || self.rp_sync.enc_uav_write_exact_res.contains(&r);
                    let common_tex_keep = s1 == 0
                        && r != 0
                        && (self.opts.opt_barrier_tight_v2
                            || self.opts.opt_icb_enc_barrier_exact
                            || self.gfx_to_ps_overlap_active())
                        && self.opts.opt_rp_keep_write
                        && unsafe {
                            let res = &*(r as *const Resource);
                            !res.desc.is_buffer() && (res.desc.Flags & SIMUL == 0)
                        };
                    let common_ro_buf_keep = s1 == 0
                        && s0 != 0
                        && r != 0
                        && self.opts.opt_ps_srv_class_exact
                        && self.opts.opt_rp_keep_write
                        && unsafe {
                            let res = &*(r as *const Resource);
                            res.desc.is_buffer()
                                && (res.desc.Flags & SIMUL == 0)
                                && (filter_res_state_bits(r, s0) & D3D12_WRITE_STATES) == 0
                        };
                    let common_buf_write_keep = s1 == 0
                        && s0 != 0
                        && r != 0
                        && self.opts.opt_rp_split_exact_keep
                        && self.opts.opt_rp_keep_write
                        && (s0 & STREAM_OUT == 0)
                        && unsafe {
                            let res = &*(r as *const Resource);
                            res.desc.is_buffer() && (res.desc.Flags & SIMUL == 0)
                        }
                        && !((s0 & UAV != 0) && uav_written_here);
                    let common_keep =
                        common_tex_keep || common_ro_buf_keep || common_buf_write_keep;
                    let write_ok = if s1 == 0 {
                        common_keep
                    } else if s1 & D3D12_WRITE_STATES == 0 {
                        true
                    } else {
                        self.opts.opt_rp_keep_write && (s1 & STREAM_OUT == 0)
                    };
                    let uav_s0_conflict = (s0 & UAV != 0)
                        && self.gfx.pass_uav
                        && (!self.opts.opt_rp_split_exact_keep
                            || r == 0
                            || uav_written_here
                            || unsafe { (*(r as *const Resource)).desc.Flags & SIMUL != 0 });
                    r != 0
                        && (s1 != 0 || common_keep)
                        && write_ok
                        && !uav_s0_conflict
                        && !self.bar.pass_targets.contains(&r)
                        && !open_rtvs.iter().any(|v| v.resource == r)
                        && !open_dsv.is_some_and(|v| v.resource == r)
                        && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                }
                Barrier::Uav(r) => {
                    self.opts.opt_rp_keep_write
                        && (!self.gfx.pass_uav
                            || (self.opts.opt_rp_split_exact_keep
                                && r != 0
                                && !self.rp_sync.enc_uav_write_unbounded
                                && !self.rp_sync.enc_uav_write_exact_res.contains(&r)
                                && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }))
                }
                Barrier::Full => false,
            })
    }

    /// Record barrier obligations for kept write transitions or `Barrier::Uav`
    /// items so they land cleanly when the open draw pass closes (`AW2_RP_KEEP_WRITE=1`).
    pub(super) fn note_render_kept_writes(
        &mut self,
        items: &[Barrier],
        elide_r2r: bool,
        shader: MTLStages,
    ) -> bool {
        use MTLStages as S;
        const UAV: u32 = 0x8;
        const SRV_UAV: u32 = 0x8 | 0x40;
        const VS_BUF_SOFT: u32 = 0x1 | 0x2 | 0x200;
        let vs_buf_defer = self.vs_buf_defer_exact_active();
        let add_rp_soft = |rp_soft: &mut [S; 2], r: usize, st: S| {
            let c = res_class(r);
            for (i, s) in rp_soft.iter_mut().enumerate() {
                if c & (1 << i) != 0 {
                    *s |= st;
                }
            }
        };
        let mut rp_pt = std::mem::take(&mut self.bar.bar_scratch_rp_pt);
        rp_pt.clear();
        rp_pt.extend_from_slice(&self.bar.uav_passthrough);
        let mut any = false;
        for b in items {
            match *b {
                Barrier::SplitBegin(..) => {}
                Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1) => {
                    if elide_r2r
                        && (s0 == s1
                            || (is_read_to_read_transition(s0, s1)
                                && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                                && (!self.ps_to_cs_active()
                                    || self.cs_disp_passthrough_for(r).is_empty())))
                    {
                        continue;
                    }
                    let is_ro_buf_common = s1 == 0
                        && self.opts.opt_ps_srv_class_exact
                        && r != 0
                        && unsafe {
                            let res = &*(r as *const Resource);
                            res.desc.is_buffer()
                                && (filter_res_state_bits(r, s0) & D3D12_WRITE_STATES) == 0
                        };
                    let is_buf_write_common_exact = s1 == 0
                        && self.opts.opt_rp_split_exact_keep
                        && !is_ro_buf_common
                        && r != 0
                        && unsafe { (*(r as *const Resource)).desc.is_buffer() };
                    let is_tex_common_exact = s1 == 0
                        && (self.opts.opt_icb_enc_barrier_exact || self.gfx_to_ps_overlap_active())
                        && !self.opts.opt_barrier_tight_v2
                        && r != 0
                        && unsafe { !(*(r as *const Resource)).desc.is_buffer() };
                    if s1 == 0
                        && (self.opts.opt_barrier_tight_v2
                            || is_ro_buf_common
                            || is_buf_write_common_exact
                            || is_tex_common_exact)
                    {
                        any = true;
                        if is_ro_buf_common {
                            stats::inc(&stats::c::ps_srv_class_exact);
                        }
                        if is_buf_write_common_exact
                            || (self.opts.opt_rp_split_exact_keep
                                && (s0 & UAV != 0)
                                && self.gfx.pass_uav)
                        {
                            stats::inc(&stats::c::rp_split_exact_keep);
                        }
                        if is_tex_common_exact && self.opts.opt_icb_enc_barrier_exact {
                            stats::inc(&stats::c::icb_enc_barrier_exact);
                        }
                        let eff_s0 = filter_res_state_bits(r, s0);
                        let ia =
                            self.source_stages_for_transition_with(r, eff_s0, s0, false, &rp_pt);
                        if (eff_s0 & UAV) != 0 {
                            Self::pt_remove(&mut rp_pt, r);
                        }
                        let ib = S::All - PRE_RASTER;
                        let mut isoft = [S::empty(); 2];
                        add_rp_soft(&mut isoft, r, PRE_RASTER);
                        self.rp_sync.rp_write_after |= ia;
                        self.rp_sync.rp_write_before |= ib;
                        self.rp_sync.rp_write_soft[0] |= isoft[0];
                        self.rp_sync.rp_write_soft[1] |= isoft[1];
                        self.rp_sync
                            .rp_write_split
                            .add_with_req(ia, ib, isoft, self.bar.stage_gen);
                        if self.opts.opt_rp_keep_write_exact {
                            let mut kw_split = SplitPend::default();
                            kw_split.add_with_req(ia, ib, isoft, self.bar.stage_gen);
                            self.rp_sync.rp_kept_writes.push(KeptWriteItem {
                                r,
                                s0,
                                eff_s0,
                                eff_s1: 0,
                                is_uav_barrier: false,
                                split: kw_split,
                            });
                        }
                    } else if s1 & D3D12_WRITE_STATES != 0 {
                        any = true;
                        if self.opts.opt_rp_split_exact_keep
                            && (s0 & UAV != 0)
                            && self.gfx.pass_uav
                        {
                            stats::inc(&stats::c::rp_split_exact_keep);
                        }
                        if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                            let eff_s0 = filter_res_state_bits(r, s0);
                            let eff_s1 = filter_res_state_bits(r, s1);
                            let ia = self.source_stages_for_transition_with(
                                r, eff_s0, s0, false, &rp_pt,
                            );
                            if (eff_s0 & UAV) != 0 && (eff_s1 & UAV) == 0 {
                                Self::pt_remove(&mut rp_pt, r);
                            }
                            if (eff_s1 & UAV) != 0 {
                                Self::pt_insert(&mut rp_pt, r, ia);
                            }
                            let is_buf = r != 0 && res_class(r) == READS_BUF;
                            let soft_mask = if vs_buf_defer && is_buf {
                                SRV_UAV | VS_BUF_SOFT
                            } else {
                                SRV_UAV
                            };
                            let mut ib = S::empty();
                            let mut isoft = [S::empty(); 2];
                            let hard = eff_s1 & !soft_mask;
                            if eff_s1 & soft_mask != 0 {
                                let st = state_stages(eff_s1 & soft_mask);
                                add_rp_soft(&mut isoft, r, st & PRE_RASTER);
                                ib |= st - PRE_RASTER;
                            }
                            if hard != 0 {
                                ib |= state_stages(hard);
                            }
                            if vs_buf_defer
                                && is_buf
                                && (eff_s1 & VS_BUF_SOFT) != 0
                                && !ib.intersects(PRE_RASTER)
                                && !ia.is_empty()
                            {
                                stats::inc(&stats::c::vs_buf_defer);
                            }
                            if self.opts.opt_split_res_soft_sync
                                && !(isoft[0] | isoft[1]).is_empty()
                            {
                                stats::inc(&stats::c::res_soft_sync);
                            }
                            self.rp_sync.rp_write_after |= ia;
                            self.rp_sync.rp_write_before |= ib;
                            self.rp_sync.rp_write_soft[0] |= isoft[0];
                            self.rp_sync.rp_write_soft[1] |= isoft[1];
                            self.rp_sync.rp_write_split.add_with_req(
                                ia,
                                ib,
                                isoft,
                                self.bar.stage_gen,
                            );
                            if self.opts.opt_rp_keep_write_exact {
                                let mut kw_split = SplitPend::default();
                                kw_split.add_with_req(ia, ib, isoft, self.bar.stage_gen);
                                self.rp_sync.rp_kept_writes.push(KeptWriteItem {
                                    r,
                                    s0,
                                    eff_s0,
                                    eff_s1,
                                    is_uav_barrier: false,
                                    split: kw_split,
                                });
                            }
                        } else {
                            let ia =
                                self.source_stages_for_transition_with(r, s0, s0, false, &rp_pt);
                            if (s0 & UAV) != 0 && (s1 & UAV) == 0 {
                                Self::pt_remove(&mut rp_pt, r);
                            }
                            if (s1 & UAV) != 0 {
                                Self::pt_insert(&mut rp_pt, r, ia);
                            }
                            let ib = state_stages(s1);
                            self.rp_sync.rp_write_after |= ia;
                            self.rp_sync.rp_write_before |= ib;
                            self.rp_sync.rp_write_split.add_with_req(
                                ia,
                                ib,
                                [S::empty(); 2],
                                self.bar.stage_gen,
                            );
                            if self.opts.opt_rp_keep_write_exact {
                                let mut kw_split = SplitPend::default();
                                kw_split.add_with_req(ia, ib, [S::empty(); 2], self.bar.stage_gen);
                                self.rp_sync.rp_kept_writes.push(KeptWriteItem {
                                    r,
                                    s0,
                                    eff_s0: s0,
                                    eff_s1: s1,
                                    is_uav_barrier: false,
                                    split: kw_split,
                                });
                            }
                        }
                        if s1 & UAV != 0 {
                            self.gfx.pass_uav_hazard = true;
                            if r == 0 {
                                self.rp_sync.pass_uav_hazard_all = true;
                            } else if !self.rp_sync.pass_uav_hazard_res.contains(&r) {
                                self.rp_sync.pass_uav_hazard_res.push(r);
                            }
                        }
                    } else {
                        if self.opts.opt_rp_split_exact_keep
                            && (s0 & UAV != 0)
                            && self.gfx.pass_uav
                        {
                            stats::inc(&stats::c::rp_split_exact_keep);
                        }
                        let (eff_s0, eff_s1) =
                            if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                                (filter_res_state_bits(r, s0), filter_res_state_bits(r, s1))
                            } else {
                                (s0, s1)
                            };
                        if (eff_s0 & UAV) != 0 && (eff_s1 & UAV) == 0 {
                            Self::pt_remove(&mut rp_pt, r);
                        }
                    }
                }
                Barrier::Uav(r) => {
                    any = true;
                    if self.opts.opt_rp_split_exact_keep && self.gfx.pass_uav {
                        stats::inc(&stats::c::rp_split_exact_keep);
                    }
                    let uav_after = self.source_stages_for_uav_with(r, shader, false, &rp_pt);
                    self.rp_sync.rp_write_after |= uav_after;
                    if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
                        let ib = shader - PRE_RASTER;
                        let mut isoft = [S::empty(); 2];
                        add_rp_soft(&mut isoft, r, PRE_RASTER);
                        if self.opts.opt_split_res_soft_sync && !(isoft[0] | isoft[1]).is_empty() {
                            stats::inc(&stats::c::res_soft_sync);
                        }
                        self.rp_sync.rp_write_before |= ib;
                        self.rp_sync.rp_write_soft[0] |= isoft[0];
                        self.rp_sync.rp_write_soft[1] |= isoft[1];
                        self.rp_sync.rp_write_split.add_with_req(
                            uav_after,
                            ib,
                            isoft,
                            self.bar.stage_gen,
                        );
                        if self.opts.opt_rp_keep_write_exact {
                            let mut kw_split = SplitPend::default();
                            kw_split.add_with_req(uav_after, ib, isoft, self.bar.stage_gen);
                            self.rp_sync.rp_kept_writes.push(KeptWriteItem {
                                r,
                                s0: UAV,
                                eff_s0: UAV,
                                eff_s1: UAV,
                                is_uav_barrier: true,
                                split: kw_split,
                            });
                        }
                    } else {
                        self.rp_sync.rp_write_before |= shader;
                        self.rp_sync.rp_write_split.add_with_req(
                            uav_after,
                            shader,
                            [S::empty(); 2],
                            self.bar.stage_gen,
                        );
                        if self.opts.opt_rp_keep_write_exact {
                            let mut kw_split = SplitPend::default();
                            kw_split.add_with_req(
                                uav_after,
                                shader,
                                [S::empty(); 2],
                                self.bar.stage_gen,
                            );
                            self.rp_sync.rp_kept_writes.push(KeptWriteItem {
                                r,
                                s0: UAV,
                                eff_s0: UAV,
                                eff_s1: UAV,
                                is_uav_barrier: true,
                                split: kw_split,
                            });
                        }
                    }
                    self.gfx.pass_uav_hazard = true;
                    if r == 0 {
                        self.rp_sync.pass_uav_hazard_all = true;
                    } else if !self.rp_sync.pass_uav_hazard_res.contains(&r) {
                        self.rp_sync.pass_uav_hazard_res.push(r);
                    }
                }
                Barrier::Full => {}
            }
        }
        rp_pt.clear();
        self.bar.bar_scratch_rp_pt = rp_pt;
        any
    }
}
