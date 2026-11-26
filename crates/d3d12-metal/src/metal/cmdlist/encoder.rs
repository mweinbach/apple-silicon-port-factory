//! Metal 4 encoder lifecycle: opening, reusing, and closing compute/render encoders,
//! pass timing timestamps, argument-table cache helpers, and close-time producer barriers.

use super::opts::*;
use super::state::*;
use super::CommandList;
use crate::metal::encstats as stats;
use crate::metal::passtiming as pt;
use crate::metal::pipeline::{vertex_buffer_index, MAX_VERTEX_SLOTS, READS_BUF};
use crate::metal::resource::{
    any_placed_alias, widen_uav0_alias_families, Resource, SubRange, HK_CBV_BUF,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

impl CommandList {
    #[inline]
    pub(super) fn flush_icb_prepass_if_pending(&mut self) {
        if self.icb.icb_prepass_pending && !self.icb.icb_prepass_flushing {
            self.flush_draw_batch(true);
        }
    }

    #[inline]
    pub(super) fn active_rtvs(&self) -> &[TargetView] {
        if self.opts.opt_rp_split_exact_keep && self.gfx.draw_pass {
            &self.gfx.open_rtvs
        } else {
            &self.gfx.rtvs
        }
    }

    #[inline]
    pub(super) fn active_dsv(&self) -> Option<TargetView> {
        if self.opts.opt_rp_split_exact_keep && self.gfx.draw_pass {
            self.gfx.open_dsv
        } else {
            self.gfx.dsv
        }
    }

    #[inline]
    pub(super) fn same_targets(
        a_rtvs: &[TargetView],
        a_dsv: Option<TargetView>,
        b_rtvs: &[TargetView],
        b_dsv: Option<TargetView>,
    ) -> bool {
        fn trim(s: &[TargetView]) -> &[TargetView] {
            let mut n = s.len();
            while n > 0 && s[n - 1].resource == 0 {
                n -= 1;
            }
            &s[..n]
        }
        let ar = trim(a_rtvs);
        let br = trim(b_rtvs);
        if ar.len() != br.len() {
            return false;
        }
        for (x, y) in ar.iter().zip(br.iter()) {
            if x.resource != y.resource
                || (x.resource != 0
                    && (x.format != y.format
                        || x.mip != y.mip
                        || x.first_slice != y.first_slice
                        || x.slices != y.slices))
            {
                return false;
            }
        }
        let norm_d = |d: Option<TargetView>| {
            d.filter(|v| v.resource != 0)
                .map(|v| (v.resource, v.format, v.mip, v.first_slice, v.slices, v.flags))
        };
        norm_d(a_dsv) == norm_d(b_dsv)
    }

    #[inline]
    pub(crate) fn invalidate_table_cache(&mut self) {
        self.binds.bound_table_addrs = [u64::MAX; 7];
        self.binds.bound_vbs = [(u64::MAX, usize::MAX); MAX_VERTEX_SLOTS as usize];
    }

    #[inline]
    pub(crate) fn set_table_addr(&mut self, addr: u64, slot: usize) {
        if self.opts.opt_fast_bind_pso && slot < self.binds.bound_table_addrs.len() {
            if self.binds.bound_table_addrs[slot] == addr {
                stats::inc(&stats::c::bind_skip);
                return;
            }
            self.binds.bound_table_addrs[slot] = addr;
        }
        unsafe { self.table.setAddress_atIndex(addr, slot) };
    }

    #[inline]
    pub(crate) fn set_table_vb(&mut self, addr: u64, stride: usize, slot: u32) {
        let s = slot as usize;
        if self.opts.opt_fast_bind_pso && s < self.binds.bound_vbs.len() {
            if self.binds.bound_vbs[s] == (addr, stride) {
                stats::inc(&stats::c::bind_skip);
                return;
            }
            self.binds.bound_vbs[s] = (addr, stride);
        }
        unsafe {
            self.table
                .setAddress_attributeStride_atIndex(addr, stride, vertex_buffer_index(slot))
        };
    }

    /// Request a full (all -> all) barrier before the next work.
    pub(super) fn need_full(&mut self, why: FullWhy) {
        if !self.bar.needs_barrier || why != FullWhy::Internal {
            self.bar.full_why = why;
        }
        self.bar.needs_barrier = true;
    }

    pub(super) fn end_encoder(&mut self) {
        if !self.icb.icb_segs.is_empty() {
            let saved_valid = self.rp_sync.cur_draw_exact_valid;
            let saved_pre_unb = self.rp_sync.cur_draw_pre_unb;
            let saved_pre_unb_hk = self.rp_sync.cur_draw_pre_unb_hk;
            let saved_ps_unb = self.rp_sync.cur_draw_ps_unb;
            let saved_ps_unb_hk = self.rp_sync.cur_draw_ps_unb_hk;
            let saved_pre = if saved_valid {
                std::mem::take(&mut self.rp_sync.cur_draw_pre_exact)
            } else {
                Vec::new()
            };
            let saved_ps = if saved_valid {
                std::mem::take(&mut self.rp_sync.cur_draw_ps_exact)
            } else {
                Vec::new()
            };
            self.flush_draw_batch(false);
            self.rp_sync.cur_draw_exact_valid = saved_valid;
            if saved_valid {
                self.rp_sync.cur_draw_pre_unb = saved_pre_unb;
                self.rp_sync.cur_draw_pre_unb_hk = saved_pre_unb_hk;
                self.rp_sync.cur_draw_ps_unb = saved_ps_unb;
                self.rp_sync.cur_draw_ps_unb_hk = saved_ps_unb_hk;
                self.rp_sync.cur_draw_pre_exact = saved_pre;
                self.rp_sync.cur_draw_ps_exact = saved_ps;
            }
        } else {
            self.icb.icb_prepass_pending = false;
        }
        self.compute_pso_bound = false;
        self.bar.enc_blit_only_sync = false;
        self.rp_sync.enc_render_cache.reset();
        self.finish_post_draw0_list_frag();
        if self.bar.vtx_defer != NO_VTX {
            // No draw in the pass needed it: the next render pass waits.
            for i in 0..2 {
                let d = self.bar.vtx_defer[i].take();
                let d_items = std::mem::take(&mut self.bar.vtx_defer_items[i]);
                let d_poison = std::mem::replace(&mut self.bar.vtx_defer_poisoned[i], false);
                if d.is_some() {
                    if self.bar.vtx_defer_list_frag
                        && (i == 0
                            || self.list_rsplit_v2_active()
                            || self.opts.opt_queue_start_stage_exact)
                    {
                        self.bar.vtx_carry_list_frag = true;
                    }
                    merge_req_all(&mut self.bar.vtx_carry_req[i], &self.bar.vtx_defer_req[i]);
                    self.bar.vtx_carry_poisoned[i] |=
                        d_poison || (self.bar.vtx_defer_list_frag && !self.list_rsplit_v2_active());
                    if !self.bar.vtx_carry_poisoned[i] {
                        Self::extend_soft_items(&mut self.bar.vtx_carry_items[i], d_items);
                    } else {
                        self.bar.vtx_carry_items[i].clear();
                    }
                }
                self.bar.vtx_defer_req[i] = [0; 7];
                self.bar.vtx_carry[i] = merge_vtx(self.bar.vtx_carry[i], d);
            }
            self.bar.vtx_defer_list_frag = false;
            stats::inc(&stats::c::dv_carry);
        } else {
            for i in 0..2 {
                self.bar.vtx_defer_items[i].clear();
                self.bar.vtx_defer_poisoned[i] = false;
            }
            self.bar.vtx_defer_list_frag = false;
        }
        if matches!(self.enc, Encoder::Compute(_)) && !self.bar.enc_cs_soft_disp.is_empty() {
            let mid_on = self.mid_qb_stagesync_active();
            let mut deferred = std::mem::take(&mut self.bar.enc_cs_soft_disp);
            deferred.retain_mut(|it| {
                it.after = self.unsatisfied_enc_after_for_req(
                    it.after,
                    MTLStages::Dispatch,
                    &it.req,
                    mid_on,
                );
                !it.after.is_empty()
            });
            self.bar.enc_cs_soft_disp = deferred;
        }
        let ran = std::mem::replace(&mut self.bar.enc_ran_stages, MTLStages::empty());
        let uav_write_stages =
            std::mem::replace(&mut self.rp_sync.enc_uav_write_stages, MTLStages::empty());
        let completed_sync =
            std::mem::replace(&mut self.bar.enc_completed_sync, [StageSync::default(); 7]);
        self.bar.enc_cur_sync = [StageSync::default(); 7];
        let had_uav_gfx = std::mem::replace(&mut self.bar.enc_had_uav_gfx, false) || self.gfx.pass_uav;
        let was_draw_pass = std::mem::replace(&mut self.gfx.draw_pass, false);
        if was_draw_pass && !had_uav_gfx {
            stats::inc(&stats::c::draw_pass_no_uav);
        }
        self.gfx.pass_uav = false;
        self.gfx.pass_uav_hazard = false;
        self.rp_sync.pass_uav_hazard_all = false;
        self.rp_sync.pass_uav_hazard_res.clear();
        self.gfx.open_rtvs.clear();
        self.gfx.open_dsv = None;
        self.gfx.had_target_rebind = false;
        if let Some(i) = self.pt.as_mut().and_then(|p| p.close()) {
            self.pt_stamp(i, true);
        }
        self.icb.enc_icb_war_done = false;
        match std::mem::replace(&mut self.enc, Encoder::None) {
            Encoder::None => return,
            Encoder::Render(e) => {
                e.endEncoding();
                self.icb.enc_icb_cmds = 0;
                if std::mem::replace(&mut self.icb.enc_used_icb, false) && icb_reuse_enabled() {
                    if let Some(icbs) = &self.icb.icbs {
                        icbs.lock().unwrap().rewind();
                    }
                    if let Some(scratch) = &self.icb.icb_scratch {
                        scratch.lock().unwrap().rewind();
                    }
                }
                if std::mem::replace(&mut self.rp_sync.enc_deferred_frag_ps, false)
                    && self.opts.opt_gfx_overlap_frag_cap
                {
                    self.rp_sync.gfx_ps_overlap_chain =
                        self.rp_sync.gfx_ps_overlap_chain.saturating_add(1);
                }
                for (i, &bit) in STAGE_BITS.iter().enumerate() {
                    if ran.contains(bit) {
                        self.bar.stage_sync[i].merge(completed_sync[i]);
                    }
                }
                for (i, &bit) in STAGE_BITS.iter().enumerate() {
                    if ran.contains(bit) {
                        self.bar.stage_gen[i] = self.bar.stage_gen[i].wrapping_add(1);
                    }
                }
                if had_uav_gfx {
                    self.bar.gfx_uav_gen = self.bar.gfx_uav_gen.wrapping_add(1);
                    let mut uav_st = uav_write_stages & ran & (PRE_RASTER | MTLStages::Fragment);
                    if uav_st.is_empty() {
                        uav_st = ran & (PRE_RASTER | MTLStages::Fragment);
                    }
                    for si in 0..4 {
                        if uav_st.contains(STAGE_BITS[si]) {
                            self.bar.gfx_uav_stage_gen[si] = self.bar.stage_gen[si];
                        }
                    }
                }
                if self.bar.stage_gen[0] > 0 && self.bar.stage_sync[0].pre_list_all {
                    self.bar.list_unwaited_vtx_frag = false;
                    self.bar.vtx_carry_list_frag = false;
                } else if self.bar.vtx_carry_list_frag
                    && self.bar.stage_gen[STAGE_FRAGMENT] > 0
                    && self.bar.stage_sync[STAGE_FRAGMENT].pre_list_all
                {
                    let classes = if self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active() {
                        2
                    } else {
                        1
                    };
                    for ci in 0..classes {
                        if let Some((ca, cb)) = self.bar.vtx_carry[ci] {
                            if ca == MTLStages::All {
                                self.bar.vtx_carry[ci] = Some((MTLStages::Fragment, cb));
                                self.bar.vtx_carry_req[ci] = [0; 7];
                                self.bar.vtx_carry_req[ci][STAGE_FRAGMENT] =
                                    self.bar.stage_gen[STAGE_FRAGMENT];
                                if self.list_rsplit_v2_active() && !self.bar.vtx_carry_poisoned[ci] {
                                    let fg = self.bar.stage_gen[STAGE_FRAGMENT];
                                    for it in &mut self.bar.vtx_carry_items[ci] {
                                        if it.after == MTLStages::All {
                                            it.after = MTLStages::Fragment;
                                            it.req = [0; 7];
                                            it.req[STAGE_FRAGMENT] = fg;
                                        }
                                    }
                                    let mut union_a = MTLStages::empty();
                                    let mut nreq = [0u32; 7];
                                    for it in &self.bar.vtx_carry_items[ci] {
                                        union_a |= it.after;
                                        merge_req_for_after(&mut nreq, it.after, &it.req);
                                    }
                                    if !union_a.is_empty() {
                                        self.bar.vtx_carry[ci] = Some((union_a, cb));
                                        self.bar.vtx_carry_req[ci] = nreq;
                                    }
                                } else {
                                    self.bar.vtx_carry_poisoned[ci] = true;
                                    self.bar.vtx_carry_items[ci].clear();
                                }
                            }
                        }
                    }
                    self.bar.vtx_carry_list_frag = false;
                }
                if !self.rp_sync.rp_write_after.is_empty() {
                    if self.opts.opt_rp_keep_write_exact && self.render_split_sync_active() {
                        let _ = std::mem::replace(&mut self.rp_sync.rp_write_after, MTLStages::empty());
                        let _ = std::mem::replace(&mut self.rp_sync.rp_write_before, MTLStages::empty());
                        self.rp_sync.rp_write_soft = [MTLStages::empty(); 2];
                        self.rp_sync.rp_write_split = SplitPend::default();
                        let render_ran = ran & (PRE_RASTER | MTLStages::Fragment);
                        let mut kept = std::mem::take(&mut self.rp_sync.rp_kept_writes);
                        for mut kw in kept.drain(..) {
                            let touch = self.rp_kept_item_touch_stages(&kw, ran, had_uav_gfx);
                            if touch != render_ran {
                                stats::inc(&stats::c::rp_keep_exact);
                            }
                            kw.split.bump_for_ran(touch, &self.bar.stage_gen);
                            if self.clear_isolate_active() && !kw.split.frag_after.is_empty() {
                                if self.opts.opt_clear_zero_qb_coalesce {
                                    const SIMUL: u32 =
                                        crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                                    let is_clean_res = kw.r != 0
                                        && unsafe {
                                            let res = &*(kw.r as *const Resource);
                                            res.placed_in.is_none()
                                                && (res.desc.Flags & SIMUL == 0)
                                        };
                                    if !is_clean_res {
                                        self.bar.frag_rt_poisoned = true;
                                        self.bar.frag_rt_pend.clear();
                                    } else if kw.is_uav_barrier
                                        || kw.eff_s1 == 0
                                        || (kw.eff_s1 & (0x4 | 0x10 | 0x20 | 0x1000)) != 0
                                    {
                                        if !self.bar.frag_rt_poisoned {
                                            self.push_frag_rt_pend(
                                                kw.r,
                                                kw.split.frag_after,
                                                &kw.split.frag_req,
                                            );
                                        }
                                    } else {
                                        self.bar.frag_non_rt_after |= kw.split.frag_after;
                                        merge_req_for_after(
                                            &mut self.bar.frag_non_rt_req,
                                            kw.split.frag_after,
                                            &kw.split.frag_req,
                                        );
                                    }
                                } else {
                                    self.bar.frag_non_rt_after |= kw.split.frag_after;
                                    merge_req_for_after(
                                        &mut self.bar.frag_non_rt_req,
                                        kw.split.frag_after,
                                        &kw.split.frag_req,
                                    );
                                }
                            }
                            self.bar.split_pend.merge(kw.split);
                            for i in 0..2 {
                                let sa = kw.split.soft_after[i];
                                let sb = kw.split.soft_before[i] & PRE_RASTER;
                                if !sa.is_empty() && !sb.is_empty() {
                                    let allow_soft_kw = (((self.opts.opt_cs_to_ps_overlap
                                        || self.opts.opt_cs_to_ps_overlap_v2)
                                        && sa != MTLStages::All)
                                        || self.gfx_to_ps_overlap_active())
                                        && kw.r != 0
                                        && unsafe {
                                            (*(kw.r as *const Resource)).desc.Flags
                                                & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS
                                                == 0
                                        };
                                    if allow_soft_kw {
                                        if !self.bar.soft_pend_poisoned[i] {
                                            Self::push_soft_obl_into(
                                                &mut self.bar.soft_pend_items[i],
                                                kw.r,
                                                sa,
                                                sb,
                                                kw.split.soft_req[i],
                                            );
                                        }
                                    } else {
                                        self.bar.soft_pend_poisoned[i] = true;
                                        self.bar.soft_pend_items[i].clear();
                                    }
                                }
                            }
                            if self.ps_to_cs_active() && !kw.split.disp_after.is_empty() {
                                self.record_cs_soft_disp(
                                    kw.r,
                                    kw.split.disp_after,
                                    kw.split.disp_req,
                                );
                            }
                            if kw.r != 0 {
                                let mut kw_req = [0u32; 7];
                                merge_req_for_after(
                                    &mut kw_req,
                                    kw.split.disp_after,
                                    &kw.split.disp_req,
                                );
                                merge_req_for_after(
                                    &mut kw_req,
                                    kw.split.blit_after,
                                    &kw.split.blit_req,
                                );
                                merge_req_for_after(&mut kw_req, kw.split.as_after, &kw.split.as_req);
                                Self::push_cs_mid_obl_into(
                                    &mut self.bar.pend_cs_items,
                                    kw.r,
                                    kw.split.disp_after,
                                    kw.split.blit_after,
                                    kw.split.as_after,
                                    kw_req,
                                );
                            } else {
                                if !kw.split.disp_after.is_empty() {
                                    self.bar.pend_cs_poisoned[0] = true;
                                }
                                if !kw.split.blit_after.is_empty() {
                                    self.bar.pend_cs_poisoned[1] = true;
                                }
                                if !kw.split.as_after.is_empty() {
                                    self.bar.pend_cs_poisoned[2] = true;
                                }
                            }
                        }
                        self.rp_sync.rp_kept_writes = kept;
                        self.sync_legacy_from_split();
                    } else {
                        self.rp_sync.rp_kept_writes.clear();
                        self.bar.pend_after |=
                            std::mem::replace(&mut self.rp_sync.rp_write_after, MTLStages::empty());
                        self.bar.pend_before |=
                            std::mem::replace(&mut self.rp_sync.rp_write_before, MTLStages::empty());
                        self.bar.pend_before_soft[0] |=
                            std::mem::replace(&mut self.rp_sync.rp_write_soft[0], MTLStages::empty());
                        self.bar.pend_before_soft[1] |=
                            std::mem::replace(&mut self.rp_sync.rp_write_soft[1], MTLStages::empty());
                        let mut rps = std::mem::take(&mut self.rp_sync.rp_write_split);
                        rps.bump_for_ran(ran, &self.bar.stage_gen);
                        if self.clear_isolate_active() && !rps.frag_after.is_empty() {
                            if self.opts.opt_clear_zero_qb_coalesce {
                                self.bar.frag_rt_poisoned = true;
                                self.bar.frag_rt_pend.clear();
                            } else {
                                self.bar.frag_non_rt_after |= rps.frag_after;
                                merge_req_for_after(
                                    &mut self.bar.frag_non_rt_req,
                                    rps.frag_after,
                                    &rps.frag_req,
                                );
                            }
                        }
                        if self.ps_to_cs_active() && !rps.disp_after.is_empty() {
                            self.record_cs_soft_disp(0, rps.disp_after, rps.disp_req);
                        }
                        for i in 0..2 {
                            if !rps.soft_after[i].is_empty() {
                                self.bar.soft_pend_poisoned[i] = true;
                                self.bar.soft_pend_items[i].clear();
                            }
                        }
                        if !rps.disp_after.is_empty() {
                            self.bar.pend_cs_poisoned[0] = true;
                        }
                        if !rps.blit_after.is_empty() {
                            self.bar.pend_cs_poisoned[1] = true;
                        }
                        if !rps.as_after.is_empty() {
                            self.bar.pend_cs_poisoned[2] = true;
                        }
                        self.bar.split_pend.merge(rps);
                    }
                } else {
                    self.rp_sync.rp_write_split = SplitPend::default();
                    self.rp_sync.rp_kept_writes.clear();
                }
                self.rp_sync.rp_kept_later_draw = false;
                self.rp_sync.reset_for_render_pass(&[], 0);
                for r in std::mem::take(&mut self.bar.pass_targets) {
                    if !self.bar.written.contains(&r) {
                        self.bar.written.push(r);
                    }
                }
                if legacy_barriers() {
                    self.need_full(FullWhy::Internal);
                }
            }
            Encoder::Compute(e) => {
                e.endEncoding();
                for (i, &bit) in STAGE_BITS.iter().enumerate() {
                    if ran.contains(bit) {
                        self.bar.stage_sync[i].merge(completed_sync[i]);
                    }
                }
                for (i, &bit) in STAGE_BITS.iter().enumerate() {
                    if ran.contains(bit) {
                        self.bar.stage_gen[i] = self.bar.stage_gen[i].wrapping_add(1);
                    }
                }
                if ran.contains(MTLStages::Dispatch) {
                    self.bar.cs_disp_gen = self.bar.cs_disp_gen.wrapping_add(1);
                }
                if ran.contains(MTLStages::AccelerationStructure) {
                    self.bar.cs_as_gen = self.bar.cs_as_gen.wrapping_add(1);
                }
                self.bar.cs_mid_v3_items.clear();
                self.bar.cs_mid_v3_poisoned = [false; 3];
                let had_cs_uav = !self.bar.cs_uav_after.is_empty();
                self.flush_cs_uav_deferred();
                if had_cs_uav {
                    self.bar.pend_cs_poisoned[0] = true;
                    self.bar.pend_cs_poisoned[2] = true;
                }
                self.flush_cs_end_after();
                self.flush_cs_mid_res_all();
                if !self.bar.cs_disp_passthrough.is_empty() && self.bar.stage_gen[STAGE_DISPATCH] > 0 {
                    let ds = self.bar.stage_sync[STAGE_DISPATCH];
                    self.bar.cs_disp_passthrough.retain_mut(|(_, st, req)| {
                        *st = Self::unsatisfied_sync_after(*st, &ds, req);
                        !st.is_empty()
                    });
                }
                if !self.bar.enc_cs_soft_disp.is_empty() {
                    let ds = self.bar.stage_sync[STAGE_DISPATCH];
                    let dgen_ran = self.bar.stage_gen[STAGE_DISPATCH] > 0;
                    let v5 = self.opts.opt_cs_mid_qb_res_defer_v5 && self.ib_tight_active();
                    let deferred = std::mem::take(&mut self.bar.enc_cs_soft_disp);
                    for it in deferred {
                        let unsat = if dgen_ran {
                            Self::unsatisfied_sync_after(it.after, &ds, &it.req)
                        } else {
                            it.after
                        };
                        if unsat.is_empty() {
                            continue;
                        }
                        self.bar.pend_after |= unsat;
                        self.bar.pend_before |= MTLStages::Dispatch;
                        self.bar.split_pend.disp_after |= unsat;
                        merge_req_for_after(&mut self.bar.split_pend.disp_req, unsat, &it.req);
                        self.record_cs_soft_disp(it.r, unsat, it.req);
                        if v5 {
                            if it.r != 0 && !self.bar.pend_cs_poisoned[0] {
                                Self::push_cs_mid_obl_into(
                                    &mut self.bar.pend_cs_items,
                                    it.r,
                                    unsat,
                                    MTLStages::empty(),
                                    MTLStages::empty(),
                                    it.req,
                                );
                            } else {
                                self.bar.pend_cs_poisoned[0] = true;
                                self.bar.pend_cs_items.clear();
                            }
                        }
                    }
                    self.sync_legacy_from_split();
                }
                if ran.contains(MTLStages::Blit) {
                    for &r in &self.ib.ib_blit_dst {
                        if r != 0 && !self.bar.blit_dst_unwaited_cross.contains(&r) {
                            self.bar.blit_dst_unwaited_cross.push(r);
                        }
                    }
                }
                self.ib.ib_blit_dst.clear();
                if legacy_barriers() {
                    self.need_full(FullWhy::Internal);
                }
            }
        }
    }

    /// Pass timing: describe the encoder just opened (set as `self.enc`)
    /// and stamp its start at `MTLRenderStages::Vertex` (`end_encoder` stamps
    /// `MTLRenderStages::Fragment`). On Apple Silicon Metal 4, 0-draw `CLEAR`/`UTIL`
    /// passes (both DSV-only and color) resolve `Vertex` while leaving `Fragment` at `0`,
    /// which `passtiming::zero_work_coalesce` coalesces (`(s, 0) -> (s, s)`) with `bad == 0`.
    pub(super) fn pt_open(&mut self, kind: u8) {
        if self.pt.is_none() && self.pt_armed {
            self.pt = pt::PassLog::acquire();
            self.pt_armed = false;
        }
        let a = self.bar.enc_sync_after;
        let sync = if a.is_empty() {
            0
        } else if a == MTLStages::All {
            2
        } else {
            1
        };
        if let Some(i) = self.pt.as_mut().and_then(|p| p.open(kind, sync)) {
            self.pt_stamp(i, false);
        }
    }

    /// Pass timing: relaxed timestamp in the open encoder (snaps to the
    /// encoder boundary without splitting it).
    pub(super) fn pt_stamp(&self, index: usize, use_frag: bool) {
        let Some(p) = &self.pt else { return };
        let stage = if use_frag {
            MTLRenderStages::Fragment
        } else {
            MTLRenderStages::Vertex
        };
        unsafe {
            match &self.enc {
                Encoder::Render(e) => e.writeTimestampWithGranularity_afterStage_intoHeap_atIndex(
                    MTL4TimestampGranularity::Relaxed,
                    stage,
                    p.heap(),
                    index,
                ),
                Encoder::Compute(e) => e.writeTimestampWithGranularity_intoHeap_atIndex(
                    MTL4TimestampGranularity::Relaxed,
                    p.heap(),
                    index,
                ),
                Encoder::None => {}
            }
        }
    }

    /// Pass timing: attribute utility/internal work in the open encoder when no
    /// game shader has been bound in it yet.
    #[inline]
    pub(super) fn pt_note_util(&mut self, synth_key: u64, add_dispatches: u32) {
        if let Some(m) = self.pt_cur() {
            m.dispatches += add_dispatches;
            if m.shader == 0 {
                m.shader = synth_key;
            }
        }
    }

    /// Pass timing: the current encoder description (when timing).
    pub(super) fn pt_cur(&mut self) -> Option<&mut pt::PassMeta> {
        self.pt.as_mut().map(|p| p.cur())
    }

    /// Pass timing split mode (`PORT_PASS_TIMING=2`): close the open encoder
    /// before the next dispatch/draw when it already holds one, and fence the
    /// next one behind it (independent encoders overlap on the GPU, which
    /// would put other work inside each interval).
    pub(super) fn pt_split(&mut self) {
        if pt::split()
            && !matches!(self.enc, Encoder::None)
            && self.pt.as_ref().is_some_and(|p| p.has_work())
        {
            self.end_encoder();
            self.need_full(FullWhy::Internal);
        }
    }

    /// The pass timing log of the closed recording (moved to the submission).
    pub fn take_pass_log(&mut self) -> Option<pt::PassLog> {
        if self.open {
            None
        } else {
            self.pt.take()
        }
    }

    /// Emit Metal 4 producer barriers (`barrierAfterStages_beforeQueueStages`)
    /// at the end of a recording when `AW2_LIST_BARRIER_TIGHT=1` is active so
    /// subsequent command lists' encoders order against trailing transitions and
    /// un-synced `Fragment` attachment writes without requiring `FullWhy::ListStart`.
    pub(super) fn emit_close_producer_barriers(&mut self) {
        self.flush_draw_batch(true);
        self.flush_enc_cs_soft_disp_all();
        self.flush_cs_uav_deferred();
        self.flush_cs_end_after();
        self.flush_cs_mid_res_all();
        self.flush_clear_tex_cs_all();
        if !self.bar.clear_tex_gfx_before.is_empty()
            || self.bar.clear_tex_gfx_soft != [MTLStages::empty(); 2]
        {
            let b = std::mem::replace(&mut self.bar.clear_tex_gfx_before, MTLStages::empty());
            let s = std::mem::replace(&mut self.bar.clear_tex_gfx_soft, [MTLStages::empty(); 2]);
            self.add_pend_with_soft(MTLStages::Fragment, b, s);
        }
        let (mut trail_after, mut trail_before) = (MTLStages::empty(), MTLStages::empty());
        if self.bar.needs_barrier {
            trail_after = MTLStages::All;
            trail_before = MTLStages::All;
        } else {
            let pend_soft = self.bar.pend_before_soft[0] | self.bar.pend_before_soft[1];
            if !self.bar.pend_after.is_empty() && !(self.bar.pend_before | pend_soft).is_empty() {
                trail_after |= self.bar.pend_after;
                trail_before |= self.bar.pend_before | pend_soft;
            }
            let rp_soft = self.rp_sync.rp_write_soft[0] | self.rp_sync.rp_write_soft[1];
            if !self.rp_sync.rp_write_after.is_empty()
                && !(self.rp_sync.rp_write_before | rp_soft).is_empty()
            {
                trail_after |= self.rp_sync.rp_write_after;
                trail_before |= self.rp_sync.rp_write_before | rp_soft;
            }
            for c in self.bar.vtx_defer.into_iter().flatten() {
                trail_after |= c.0;
                trail_before |= c.1;
            }
            for c in self.bar.vtx_carry.into_iter().flatten() {
                trail_after |= c.0;
                trail_before |= c.1;
            }
        }
        let had_frag_writes = !self.bar.written.is_empty() || !self.bar.pass_targets.is_empty();
        let need_trail = !trail_after.is_empty() && !trail_before.is_empty();
        let need_frag_waw = had_frag_writes
            && !(need_trail
                && trail_after.contains(MTLStages::Fragment)
                && trail_before.contains(MTLStages::Fragment));
        if !need_trail && !need_frag_waw {
            return;
        }
        let emit_on = |enc: &ProtocolObject<dyn MTL4CommandEncoder>| {
            if need_trail {
                enc.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    trail_after,
                    trail_before,
                    MTL4VisibilityOptions::Device,
                );
            }
            if need_frag_waw {
                enc.barrierAfterStages_beforeQueueStages_visibilityOptions(
                    MTLStages::Fragment,
                    MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
            }
        };
        match &self.enc {
            Encoder::Render(e) => {
                emit_on(ProtocolObject::from_ref(&**e));
                stats::inc(&stats::c::qb_list_trail);
            }
            Encoder::Compute(e) => {
                emit_on(ProtocolObject::from_ref(&**e));
                stats::inc(&stats::c::qb_list_trail);
            }
            Encoder::None => {
                if let Some(e) = self.cb.computeCommandEncoder() {
                    self.bar.has_gpu_work = true;
                    emit_on(ProtocolObject::from_ref(&*e));
                    e.endEncoding();
                    stats::inc(&stats::c::qb_list_trail);
                }
            }
        }
        self.bar.needs_barrier = false;
        self.clear_all_pend();
        self.rp_sync.rp_write_after = MTLStages::empty();
        self.rp_sync.rp_write_before = MTLStages::empty();
        self.rp_sync.rp_write_soft = [MTLStages::empty(); 2];
        self.rp_sync.rp_write_split = SplitPend::default();
        self.rp_sync.rp_kept_writes.clear();
        self.bar.vtx_defer = NO_VTX;
        self.bar.vtx_defer_req = [[0; 7]; 2];
        self.bar.vtx_carry = NO_VTX;
        self.bar.vtx_carry_req = [[0; 7]; 2];
        self.bar.vtx_defer_list_frag = false;
        self.bar.vtx_carry_list_frag = false;
        self.bar.list_unwaited_vtx_frag = false;
        self.bar.written.clear();
    }

    #[inline]
    pub(super) fn is_blit_touch_nonfrag_safe(&self, touch: &[usize]) -> bool {
        !touch.is_empty()
            && touch.iter().all(|&r| {
                if r == 0 || self.bar.is_blit_tainted(r) {
                    return false;
                }
                if self.bar.is_blit_safe(r) {
                    return true;
                }
                let res = unsafe { &*(r as *const Resource) };
                if self.opts.opt_queue_start_stage_exact
                    && (!self.bar.has_gpu_work || self.bar.list_cs_unwaited_frag)
                    && res.is_fresh_nonfrag_blit_safe()
                {
                    return true;
                }
                res.desc.is_buffer()
                    && (res.heap.Type == crate::types::heap_type::UPLOAD
                        || res.heap.Type == crate::types::heap_type::READBACK)
            })
    }

    #[inline]
    pub(super) fn upgrade_blit_only_compute_sync(
        &mut self,
        e: &ProtocolObject<dyn MTL4ComputeCommandEncoder>,
    ) {
        if self.bar.enc_blit_only_sync || self.bar.list_cs_unwaited_frag {
            let (up_after, up_before, narrowed) = if (self.opts.opt_list_start_render_split
                || self.opts.opt_list_boundary_overlap
                || self.opts.opt_queue_start_stage_exact)
                && !self.bar.list_cs_unwaited_frag
                && self.bar.enc_cur_sync[5].pre_list_all
            {
                let anchor = if self.opts.opt_list_start_substage {
                    self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All)
                } else {
                    MTLStages::All
                };
                let a = if self.bar.enc_sync_after == MTLStages::All || anchor == MTLStages::All {
                    MTLStages::All
                } else {
                    self.bar.enc_sync_after | anchor
                };
                (a, COMPUTE_STAGES - MTLStages::Blit, true)
            } else {
                (MTLStages::All, COMPUTE_STAGES, false)
            };
            e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                up_after,
                up_before,
                MTL4VisibilityOptions::Device,
            );
            self.record_enc_queue_sync(up_after, up_before);
            stats::inc(&stats::c::cs_mid_qb);
            if narrowed {
                if self.opts.opt_list_start_render_split {
                    stats::inc(&stats::c::list_start_rsplit);
                } else if self.opts.opt_list_boundary_overlap {
                    stats::inc(&stats::c::list_boundary_overlap);
                } else {
                    stats::inc(&stats::c::queue_start_stage_exact);
                }
            }
            self.bar.enc_sync_after = up_after;
            self.bar.enc_sync_before = COMPUTE_STAGES;
            self.bar.enc_blit_only_sync = false;
            self.bar.list_cs_unwaited_frag = false;
            self.bar.list_unwaited_cs = false;
            if up_after.contains(MTLStages::Fragment) {
                self.bar.written.clear();
            }
            if !self.gfx.pass_uav && up_after.contains(MTLStages::Fragment | PRE_RASTER) {
                self.bar.uav_gfx_dirty = false;
            }
        }
    }

    /// `AW2_IB_TIGHT`: open or reuse the compute encoder for an operation in
    /// `op_stage` (`Dispatch`, `Blit`, `AccelerationStructure`, or `COMPUTE_STAGES`),
    /// checking per-resource obligations against `op_touch` (`None` = opaque) and
    /// recording stage-specific touches.
    pub(super) fn compute_op_tight(
        &mut self,
        op_stage: MTLStages,
        disp_uav: Option<&[usize]>,
        disp_read_or_touch: Option<&[usize]>,
        op_touch: Option<&[usize]>,
    ) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>> {
        self.compute_op_tight_exact(op_stage, disp_uav, disp_read_or_touch, op_touch, None)
    }

    pub(super) fn compute_op_tight_exact(
        &mut self,
        op_stage: MTLStages,
        disp_uav: Option<&[usize]>,
        disp_read_or_touch: Option<&[usize]>,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>> {
        self.flush_icb_prepass_if_pending();
        let copy_light = self.is_copy_queue_light();
        if !copy_light {
            if let Some(t) = op_touch {
                let already = if self.render_split_sync_active() {
                    self.bar
                        .split_pend
                        .after_for_compute_stage(op_stage)
                        .contains(MTLStages::Fragment)
                } else {
                    self.bar.pend_after.contains(MTLStages::Fragment)
                };
                if !already && t.iter().any(|r| *r != 0 && self.bar.written.contains(r)) {
                    if self.ps_to_cs_active()
                        && op_stage == MTLStages::Dispatch
                        && self.render_split_sync_active()
                        && !t.contains(&0)
                    {
                        if matches!(self.enc, Encoder::Compute(_)) {
                            let mid_on = self.mid_qb_stagesync_active();
                            if !self
                                .unsatisfied_enc_after_for(
                                    MTLStages::Fragment,
                                    MTLStages::Dispatch,
                                    mid_on,
                                )
                                .is_empty()
                                && self.opts.opt_cs_mid_qb_unpoison_v4
                            {
                                self.bar.cs_mid_v3_poisoned[0] = true;
                            }
                            if !self.opts.opt_cs_mid_qb_unpoison_v4 {
                                self.bar.cs_mid_v3_poisoned = [true; 3];
                            }
                        }
                        self.bar.pend_cs_poisoned[0] = true;
                        self.bar.pend_after |= MTLStages::Fragment;
                        self.bar.pend_before |= MTLStages::Dispatch;
                        let req = self.cur_producer_gen();
                        self.bar.split_pend.add_with_req(
                            MTLStages::Fragment,
                            MTLStages::Dispatch,
                            [MTLStages::empty(); 2],
                            req,
                        );
                        for &r in t {
                            if r != 0 && self.bar.written.contains(&r) {
                                self.record_cs_soft_disp(r, MTLStages::Fragment, req);
                            }
                        }
                    } else {
                        self.add_pend(MTLStages::Fragment, op_stage);
                    }
                }
            }
        }
        if let Encoder::Compute(e) = &self.enc {
            let e = e.clone();
            if op_stage != MTLStages::Blit && !copy_light {
                self.upgrade_blit_only_compute_sync(&e);
            }
            let full = self.bar.needs_barrier;
            self.flush_cs_mid_res_for_op(op_stage, full, op_touch, exact);
            self.partition_cs_soft_disp_for_op(op_stage, full, op_touch, exact);
            self.compute_mid_queue_sync_with(&e, full, op_stage, op_touch, exact);
            if full {
                let st = if copy_light {
                    MTLStages::Blit
                } else {
                    COMPUTE_STAGES
                };
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    st,
                    st,
                    MTL4VisibilityOptions::Device,
                );
                stats::inc(&stats::c::enc_barrier);
                self.ib_reset_tight();
            } else if !self.ib.ib_obl_tight.is_empty() {
                let msc_class = self.msc_class_active();
                let sub_ci = (self.opts.opt_ib_substage_narrow && !copy_light)
                    .then(|| cs_sub_idx(op_stage))
                    .flatten();
                let (
                    cs_cleared,
                    disp_uav_srv_classes,
                    disp_uav_classes,
                    disp_uav_hk,
                    disp_uav_srv_hk,
                ) = if let Some(ci) = sub_ci {
                    (
                        &self.ib.ib_sub[ci].cs_cleared,
                        self.ib.ib_sub[ci].disp_uav_srv_classes,
                        self.ib.ib_sub[ci].disp_uav_classes,
                        self.ib.ib_sub[ci].disp_uav_hk,
                        self.ib.ib_sub[ci].disp_uav_srv_hk,
                    )
                } else {
                    (
                        &self.ib.ib_cs_cleared,
                        self.ib.ib_disp_uav_srv_classes,
                        self.ib.ib_disp_uav_classes,
                        self.ib.ib_disp_uav_hk,
                        self.ib.ib_disp_uav_srv_hk,
                    )
                };
                let mut need_after = MTLStages::empty();
                let mut stage_match = false;
                let mut exact_skipped = false;
                let mut subrange_skipped = false;
                let mut class_skipped = false;
                for obl in &self.ib.ib_obl_tight {
                    if obl.before.intersects(op_stage) {
                        stage_match = true;
                        let base_hit = obl.r == 0 || op_touch.is_none_or(|t| t.contains(&obl.r));
                        if !base_hit {
                            continue;
                        }
                        let mut hit = if let Some(ex) = exact {
                            match obl.kind {
                                OblKind::UavWriteOnly if obl.r != 0 && ex.uav_exact => {
                                    (ex.uav_classes & res_class(obl.r)) != 0
                                        && ex.uavs.contains(&obl.r)
                                }
                                OblKind::ClassUavOnly(c) if ex.uav_exact => {
                                    if obl.r == 0 {
                                        ex.uavs
                                            .iter()
                                            .any(|&ur| ur == 0 || (res_class(ur) & c) != 0)
                                    } else {
                                        ex.uavs.contains(&obl.r)
                                    }
                                }
                                OblKind::UavOnly if ex.uav_exact => {
                                    if obl.r == 0 {
                                        !ex.uavs.is_empty()
                                    } else {
                                        ex.uavs.contains(&obl.r)
                                    }
                                }
                                OblKind::UavOrCbv
                                    if obl.r != 0 && ex.uav_exact && ex.bounded_read_exact =>
                                {
                                    ex.uavs.contains(&obl.r) || ex.bounded_reads.contains(&obl.r)
                                }
                                OblKind::ClassSrv(c)
                                    if obl.r != 0
                                        && ex.uav_exact
                                        && ((c & ex.exact_read_only_classes) == 0
                                            || (ex.bounded_read_exact
                                                && (c & ex.unbounded_srv_classes) == 0)) =>
                                {
                                    ex.uavs.contains(&obl.r)
                                        || ((c & ex.exact_read_only_classes) != 0
                                            && ex.bounded_reads.contains(&obl.r))
                                }
                                OblKind::Any
                                    if (self.opts.opt_desc_table_res_exact
                                        || self.opts.opt_desc_table_cs_fast
                                        || self.opts.opt_cs_intra_barrier_exact
                                        || self.opts.opt_cs_intra_barrier_v2
                                        || self.cs_intra_safe_active()
                                        || self.hk_v2_active())
                                        && op_stage == MTLStages::Dispatch
                                        && obl.r != 0
                                        && ex.uav_exact
                                        && ex.bounded_read_exact
                                        && (ex.unbounded_srv_classes & res_class(obl.r)) == 0
                                        && unsafe {
                                            (*(obl.r as *const Resource)).desc.Flags
                                                & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS
                                                == 0
                                        } =>
                                {
                                    let h = ex.uavs.contains(&obl.r)
                                        || ex.bounded_reads.contains(&obl.r);
                                    if !h {
                                        if self.cs_intra_safe_active() {
                                            self.bump_cs_intra_stat();
                                        } else if self.hk_v2_active() {
                                            self.bump_hk_stat();
                                        } else if self.opts.opt_desc_table_res_exact {
                                            stats::inc(&stats::c::desc_table_cs_exact);
                                        } else if self.opts.opt_desc_table_cs_fast {
                                            stats::inc(&stats::c::desc_table_cs_fast);
                                        } else if self.opts.opt_cs_intra_barrier_exact {
                                            stats::inc(&stats::c::cs_intra_barrier_exact);
                                        } else {
                                            stats::inc(&stats::c::cs_intra_barrier_v2);
                                        }
                                    }
                                    h
                                }
                                _ => true,
                            }
                        } else {
                            true
                        };
                        if hit
                            && msc_class
                            && op_stage == MTLStages::Dispatch
                            && obl.after == MTLStages::Dispatch
                        {
                            if let Some(ex) = exact {
                                const SIMUL: u32 =
                                    crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                                let class_disjoint = if obl.r == 0 {
                                    match obl.kind {
                                        OblKind::UavOnly => {
                                            cs_cleared.is_empty()
                                                && (disp_uav_srv_classes
                                                    & ex.uav_srv_classes
                                                    & (disp_uav_classes | ex.uav_classes))
                                                    == 0
                                        }
                                        OblKind::ClassUavOnly(c) => {
                                            let d = cs_cleared.is_empty()
                                                && ((c & ex.uav_srv_classes) == 0
                                                    || (c & (disp_uav_classes | ex.uav_classes))
                                                        == 0);
                                            if d {
                                                stats::inc(&stats::c::cs_intra_barrier_v2);
                                            }
                                            d
                                        }
                                        _ => false,
                                    }
                                } else if unsafe {
                                    (*(obl.r as *const Resource)).desc.Flags & SIMUL == 0
                                } {
                                    let rc = res_class(obl.r);
                                    if obl.kind == OblKind::UavWriteOnly {
                                        let d = (ex.uav_classes & rc) == 0;
                                        if d {
                                            stats::inc(&stats::c::cs_intra_barrier_v2);
                                        }
                                        d
                                    } else {
                                        let cons_classes = if matches!(
                                            obl.kind,
                                            OblKind::UavOnly | OblKind::ClassUavOnly(_)
                                        ) {
                                            ex.uav_srv_classes
                                        } else {
                                            ex.read_classes
                                        };
                                        (cons_classes & rc) == 0
                                            || (!cs_cleared.contains(&obl.r)
                                                && ((disp_uav_classes | ex.uav_classes) & rc) == 0)
                                    }
                                } else {
                                    false
                                };
                                if class_disjoint {
                                    hit = false;
                                    class_skipped = true;
                                }
                            }
                        }
                        if hit
                            && self.hk_any_active()
                            && op_stage == MTLStages::Dispatch
                            && obl.after == MTLStages::Dispatch
                        {
                            if let Some(ex) = exact {
                                const SIMUL: u32 =
                                    crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                                let hk_disjoint = if obl.r == 0 {
                                    match obl.kind {
                                        OblKind::UavOnly => {
                                            if cs_cleared.is_empty() && !any_placed_alias() {
                                                let prod_srv =
                                                    widen_uav0_alias_families(disp_uav_srv_hk);
                                                let prod_uav =
                                                    widen_uav0_alias_families(disp_uav_hk);
                                                let cons_srv =
                                                    widen_uav0_alias_families(ex.hk.uav_srv_kinds);
                                                let cons_uav = widen_uav0_alias_families(
                                                    ex.hk.uav_write_kinds,
                                                );
                                                (prod_srv & cons_srv & (prod_uav | cons_uav)) == 0
                                            } else {
                                                false
                                            }
                                        }
                                        _ => false,
                                    }
                                } else {
                                    let res = unsafe { &*(obl.r as *const Resource) };
                                    if res.desc.Flags & SIMUL == 0 {
                                        let rhk = res.handle_kind_mask();
                                        let unb_only = self.hk_v2_active()
                                            && ex.uav_exact
                                            && !ex.uavs.contains(&obl.r)
                                            && ex.bounded_read_exact
                                            && !ex.bounded_reads.contains(&obl.r);
                                        let unb_ro_hk = ex.hk.unbounded_srv_kinds
                                            | if (ex.unbounded_srv_classes & READS_BUF) != 0 {
                                                ex.hk.read_kinds & HK_CBV_BUF
                                            } else {
                                                0
                                            };
                                        let cons_hk = if matches!(obl.kind, OblKind::UavWriteOnly) {
                                            if self.hk_v2_active()
                                                && ex.uav_exact
                                                && !ex.uavs.contains(&obl.r)
                                            {
                                                0
                                            } else {
                                                ex.hk.uav_write_kinds
                                            }
                                        } else if matches!(
                                            obl.kind,
                                            OblKind::UavOnly | OblKind::ClassUavOnly(_)
                                        ) {
                                            if unb_only {
                                                ex.hk.unbounded_srv_kinds
                                            } else {
                                                ex.hk.uav_srv_kinds
                                            }
                                        } else if unb_only {
                                            unb_ro_hk
                                        } else {
                                            ex.hk.read_kinds
                                        };
                                        let ex_uav_hk_for_r = if self.hk_v2_active()
                                            && ex.uav_exact
                                            && !ex.uavs.contains(&obl.r)
                                        {
                                            0
                                        } else {
                                            ex.hk.uav_write_kinds
                                        };
                                        (cons_hk & rhk) == 0
                                            || (!cs_cleared.contains(&obl.r)
                                                && ((disp_uav_hk | ex_uav_hk_for_r) & rhk) == 0)
                                    } else {
                                        false
                                    }
                                };
                                if hk_disjoint {
                                    hit = false;
                                    class_skipped = true;
                                    self.bump_hk_stat();
                                }
                            }
                        }
                        if hit
                            && self.opts.opt_subrange_uav_barrier
                            && op_stage == MTLStages::Dispatch
                            && obl.r != 0
                        {
                            if let (Some(obl_spans), Some(ex)) = (obl.spans.as_deref(), exact) {
                                let any_overlap = |spans: &[SubRange]| {
                                    spans
                                        .iter()
                                        .any(|s| obl_spans.iter().any(|os| os.overlaps(*s)))
                                };
                                let disjoint = match obl.kind {
                                    OblKind::UavOnly
                                    | OblKind::UavWriteOnly
                                    | OblKind::ClassUavOnly(_)
                                        if ex.uav_exact =>
                                    {
                                        ex.uav_spans.is_some_and(|us| !any_overlap(us))
                                    }
                                    OblKind::UavOrCbv if ex.uav_exact && ex.bounded_read_exact => {
                                        ex.uav_spans.is_some_and(|us| !any_overlap(us))
                                            && ex
                                                .bounded_read_spans
                                                .is_some_and(|rs| !any_overlap(rs))
                                    }
                                    OblKind::ClassSrv(c)
                                        if ex.uav_exact
                                            && ex.bounded_read_exact
                                            && (c & ex.unbounded_srv_classes) == 0 =>
                                    {
                                        ex.uav_spans.is_some_and(|us| !any_overlap(us))
                                            && ex
                                                .bounded_read_spans
                                                .is_some_and(|rs| !any_overlap(rs))
                                    }
                                    OblKind::Any
                                        if ex.uav_exact
                                            && (ex.full_read_exact
                                                || ((self.opts.opt_desc_table_res_exact
                                                    || self.opts.opt_desc_table_cs_fast
                                                    || self.opts.opt_cs_intra_barrier_exact
                                                    || self.opts.opt_cs_intra_barrier_v2
                                                    || self.cs_intra_safe_active()
                                                    || self.hk_v2_active())
                                                    && ex.bounded_read_exact
                                                    && (ex.unbounded_srv_classes
                                                        & res_class(obl.r))
                                                        == 0
                                                    && unsafe {
                                                        (*(obl.r as *const Resource)).desc.Flags
                                                            & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS
                                                            == 0
                                                    })) =>
                                    {
                                        let d = ex.uav_spans.is_some_and(|us| !any_overlap(us))
                                            && ex
                                                .bounded_read_spans
                                                .is_some_and(|rs| !any_overlap(rs));
                                        if d && !ex.full_read_exact {
                                            if self.cs_intra_safe_active() {
                                                self.bump_cs_intra_stat();
                                            } else if self.hk_v2_active() {
                                                self.bump_hk_stat();
                                            } else if self.opts.opt_desc_table_res_exact {
                                                stats::inc(&stats::c::desc_table_cs_exact);
                                            } else if self.opts.opt_desc_table_cs_fast {
                                                stats::inc(&stats::c::desc_table_cs_fast);
                                            } else if self.opts.opt_cs_intra_barrier_exact {
                                                stats::inc(&stats::c::cs_intra_barrier_exact);
                                            } else {
                                                stats::inc(&stats::c::cs_intra_barrier_v2);
                                            }
                                        }
                                        d
                                    }
                                    _ => false,
                                };
                                if disjoint {
                                    hit = false;
                                    subrange_skipped = true;
                                }
                            }
                        }
                        if hit {
                            need_after |= obl.after;
                        } else {
                            exact_skipped = true;
                        }
                    }
                }
                if let Some(ci) = sub_ci {
                    need_after &= self.ib.ib_sub[ci].stages;
                } else {
                    need_after &= self.ib.ib_stages;
                }
                if !need_after.is_empty() {
                    let before_enc = if copy_light {
                        MTLStages::Blit
                    } else if sub_ci.is_some() {
                        stats::inc(&stats::c::ib_substage_narrow);
                        op_stage & COMPUTE_STAGES
                    } else {
                        COMPUTE_STAGES
                    };
                    e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                        need_after,
                        before_enc,
                        MTL4VisibilityOptions::Device,
                    );
                    stats::inc(&stats::c::enc_barrier);
                    if need_after != COMPUTE_STAGES || before_enc != COMPUTE_STAGES {
                        stats::inc(&stats::c::ib_masked);
                    }
                    self.ib_clear_satisfied(need_after, before_enc, op_stage);
                } else if stage_match {
                    if subrange_skipped {
                        stats::inc(&stats::c::subrange_uav_skip);
                        stats::inc(&stats::c::ib_skip_exact);
                    } else if class_skipped {
                        stats::inc(&stats::c::ib_class_skip);
                        stats::inc(&stats::c::ib_skip_exact);
                    } else if exact_skipped {
                        stats::inc(&stats::c::ib_skip_exact);
                    } else {
                        stats::inc(&stats::c::ib_skip_touch);
                    }
                } else {
                    stats::inc(&stats::c::ib_skip_stage);
                }
            }
            self.ib_record_op_tight_exact(op_stage, disp_uav, disp_read_or_touch, exact);
            return Some(e);
        }
        self.end_encoder();
        let e = self.cb.computeCommandEncoder()?;
        self.bar.has_gpu_work = true;
        stats::inc(&stats::c::compute_enc);
        let prev_opening = self.bar.opening_compute_stage;
        if !(op_stage & COMPUTE_STAGES).is_empty() {
            self.bar.opening_compute_stage = op_stage & COMPUTE_STAGES;
        }
        self.partition_cs_soft_disp_for_open(op_stage, op_touch, exact);
        self.begin_barrier(ProtocolObject::from_ref(&*e), &[], false, 0);
        self.bar.opening_compute_stage = prev_opening;
        e.setArgumentTable(Some(&self.table));
        self.enc = Encoder::Compute(e.clone());
        self.bar.cs_mid_v3_items.clear();
        self.bar.cs_mid_v3_poisoned = if self.render_split_sync_active() {
            if self.opts.opt_cs_mid_qb_unpoison_v4 {
                [
                    !self.bar.split_pend.disp_after.is_empty(),
                    !self.bar.split_pend.blit_after.is_empty(),
                    !self.bar.split_pend.as_after.is_empty(),
                ]
            } else {
                [self.bar.split_pend.has_compute(); 3]
            }
        } else {
            [!self.bar.pend_after.is_empty() && self.bar.pend_before.intersects(COMPUTE_STAGES); 3]
        };
        self.ib.ib_obl.clear();
        self.ib.ib_obl_all = false;
        self.ib.ib_cs_clear = false;
        self.ib.ib_touch.clear_exact();
        self.ib_reset_tight();
        self.ib_record_op_tight_exact(op_stage, disp_uav, disp_read_or_touch, exact);
        self.pt_open(pt::kind::COMPUTE);
        Some(e)
    }

    /// The current compute/blit encoder (opening one if needed), for work
    /// touching unknown resources (dispatches, internal kernels).
    pub fn compute(&mut self) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>> {
        self.compute_touch(None)
    }

    /// Open/reuse the compute encoder for a `Blit` copy touching `touch`.
    pub(super) fn compute_blit_touch(
        &mut self,
        touch: &[usize],
    ) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>> {
        self.flush_icb_prepass_if_pending();
        self.flush_clear_tex_cs_touch(touch);
        if self.list_barrier_tight_active()
            && !self.bar.list_sync_compute.is_empty()
            && !self.is_copy_queue_light()
        {
            // D3D12 allows implicit `COMMON -> COPY_DEST` promotion on buffers
            // without a prior `ResourceBarrier`, so a list starting with a Blit
            // copy (`L3#0`) must wait for prior lists' `Fragment` reads too.
            self.bar.list_sync_compute = MTLStages::All;
        }
        let nonfrag_safe = (self.opts.opt_empty_enc_elide
            || self.opts.opt_blit_list_narrow_v2
            || self.opts.opt_list_boundary_overlap
            || self.opts.opt_queue_start_stage_exact)
            && self.is_blit_touch_nonfrag_safe(touch);
        if self.opts.opt_queue_start_stage_exact
            && (!self.bar.has_gpu_work || self.bar.list_cs_unwaited_frag)
            && nonfrag_safe
        {
            for &r in touch {
                if r != 0 {
                    self.bar.mark_blit_safe(r);
                }
            }
        }
        if let Encoder::Compute(e) = &self.enc {
            if self.bar.list_cs_unwaited_frag && !nonfrag_safe {
                let e = e.clone();
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::Fragment,
                    MTLStages::Blit,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(MTLStages::Fragment, MTLStages::Blit);
                self.bar.enc_cur_sync[5].pre_list_all = true;
                self.bar.enc_cur_sync[5].pre_list_stages = MTLStages::All;
                self.bar.enc_completed_sync[5].pre_list_all = true;
                self.bar.enc_completed_sync[5].pre_list_stages = MTLStages::All;
                stats::inc(&stats::c::cs_mid_qb);
                self.bar.list_cs_unwaited_frag = false;
                self.bar.enc_sync_after |= MTLStages::Fragment;
                self.bar.enc_sync_before |= MTLStages::Blit;
                self.bar.written.clear();
            }
        }
        self.bar.opening_compute_stage = MTLStages::Blit;
        self.bar.opening_blit_nonfrag_safe = nonfrag_safe;
        let out = if self.ib_tight_active() {
            self.compute_op_tight(MTLStages::Blit, None, Some(touch), Some(touch))
        } else {
            self.compute_touch(Some(touch))
        };
        self.bar.opening_compute_stage = COMPUTE_STAGES;
        self.bar.opening_blit_nonfrag_safe = false;
        if let Some(e) = &out {
            for &r in touch {
                if r != 0 {
                    unsafe { (*(r as *const Resource)).mark_gpu_used() };
                }
            }
            if !self.bar.clear_rp_unwaited_blit.is_empty()
                && touch
                    .iter()
                    .any(|r| *r != 0 && self.bar.clear_rp_unwaited_blit.contains(r))
            {
                if self.bar.enc_sync_after != MTLStages::All {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        MTLStages::All,
                        MTLStages::Blit,
                        MTL4VisibilityOptions::Device,
                    );
                }
                self.bar.clear_rp_unwaited_blit.clear();
            }
            if !self.bar.blit_dst_unwaited_cross.is_empty()
                && touch
                    .iter()
                    .any(|r| *r != 0 && self.bar.blit_dst_unwaited_cross.contains(r))
            {
                if !self.bar.enc_sync_after.contains(MTLStages::Blit) {
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        MTLStages::Blit,
                        MTLStages::Blit,
                        MTL4VisibilityOptions::Device,
                    );
                }
                self.bar.blit_dst_unwaited_cross.clear();
            }
            if touch.iter().any(|r| *r != 0 && self.ib.ib_blit_dst.contains(r)) {
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    MTLStages::Blit,
                    MTLStages::Blit,
                    MTL4VisibilityOptions::Device,
                );
                stats::inc(&stats::c::enc_barrier);
                stats::inc(&stats::c::ib_masked);
                self.ib.ib_blit_dst.clear();
            }
        }
        out
    }

    /// The current compute/blit encoder for work touching `touch` (resource
    /// pointers; `None`: unknown).
    pub(super) fn compute_touch(
        &mut self,
        touch: Option<&[usize]>,
    ) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>> {
        self.flush_icb_prepass_if_pending();
        if let Some(t) = touch {
            self.flush_clear_tex_cs_touch(t);
        } else {
            self.flush_clear_tex_cs_all();
        }
        if self.ib_tight_active() {
            return self.compute_op_tight(self.bar.opening_compute_stage, touch, touch, touch);
        }
        self.flush_enc_cs_soft_disp_all();
        if let Some(t) = touch {
            for &r in t {
                if r != 0 {
                    unsafe { (*(r as *const Resource)).mark_gpu_used() };
                }
            }
        }
        if !self.is_copy_queue_light() {
            if let Some(t) = touch {
                let stage = self.bar.opening_compute_stage;
                let already = if self.render_split_sync_active() {
                    self.bar
                        .split_pend
                        .after_for_compute_stage(stage)
                        .contains(MTLStages::Fragment)
                } else {
                    self.bar.pend_after.contains(MTLStages::Fragment)
                };
                if !already && t.iter().any(|r| *r != 0 && self.bar.written.contains(r)) {
                    self.add_pend(MTLStages::Fragment, stage);
                }
            }
        }
        if let Encoder::Compute(e) = &self.enc {
            let e = e.clone();
            if self.bar.opening_compute_stage != MTLStages::Blit && !self.is_copy_queue_light() {
                self.upgrade_blit_only_compute_sync(&e);
            }
            let full = self.bar.needs_barrier;
            self.flush_cs_mid_res_all();
            self.compute_mid_queue_sync_with(&e, full, self.bar.opening_compute_stage, touch, None);
            // This encoder's own earlier work: an intra-encoder barrier when
            // the work about to be encoded touches a resource it must wait for.
            let hit = self.ib.ib_obl_all
                || (!self.ib.ib_obl.is_empty()
                    && touch.is_none_or(|t| t.iter().any(|r| self.ib.ib_obl.contains(r))));
            if full || hit {
                let st = if self.is_copy_queue_light() {
                    MTLStages::Blit
                } else {
                    COMPUTE_STAGES
                };
                e.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                    st,
                    st,
                    MTL4VisibilityOptions::Device,
                );
                stats::inc(&stats::c::enc_barrier);
                self.ib.ib_obl.clear();
                self.ib.ib_obl_all = false;
                self.ib.ib_cs_clear = false;
                self.ib.ib_touch.clear_exact();
            }
            self.ib_add_touch(touch);
            return Some(e);
        }
        self.end_encoder();
        let e = self.cb.computeCommandEncoder()?;
        self.bar.has_gpu_work = true;
        stats::inc(&stats::c::compute_enc);
        self.begin_barrier(ProtocolObject::from_ref(&*e), &[], false, 0);
        e.setArgumentTable(Some(&self.table));
        self.enc = Encoder::Compute(e.clone());
        self.bar.cs_mid_v3_items.clear();
        self.bar.cs_mid_v3_poisoned = if self.render_split_sync_active() {
            if self.opts.opt_cs_mid_qb_unpoison_v4 {
                [
                    !self.bar.split_pend.disp_after.is_empty(),
                    !self.bar.split_pend.blit_after.is_empty(),
                    !self.bar.split_pend.as_after.is_empty(),
                ]
            } else {
                [self.bar.split_pend.has_compute(); 3]
            }
        } else {
            [!self.bar.pend_after.is_empty() && self.bar.pend_before.intersects(COMPUTE_STAGES); 3]
        };
        self.ib.ib_obl.clear();
        self.ib.ib_obl_all = false;
        self.ib.ib_cs_clear = false;
        self.ib.ib_touch.clear_exact();
        self.ib_add_touch(touch);
        self.pt_open(pt::kind::COMPUTE);
        Some(e)
    }

    pub(super) fn ib_add_touch(&mut self, touch: Option<&[usize]>) {
        match touch {
            Some(t) => {
                if let Some(v) = self.ib.ib_touch.as_mut_vec() {
                    for &r in t {
                        if !v.contains(&r) {
                            v.push(r);
                        }
                    }
                }
            }
            None => self.ib.ib_touch.invalidate(),
        }
    }

    /// A `ResourceBarrier` naming `r` (0: every resource) inside the open
    /// compute encoder: later work touching `r` waits for this encoder's
    /// earlier work if that touched `r`.
    pub(super) fn ib_note(&mut self, r: usize) {
        match self.ib.ib_touch.as_slice() {
            Some(v) if r != 0 && !v.contains(&r) => {}
            Some(v) if v.is_empty() => {}
            _ if r == 0 => self.ib.ib_obl_all = true,
            _ => {
                if !self.ib.ib_obl.contains(&r) {
                    self.ib.ib_obl.push(r);
                }
            }
        }
    }

    /// Open a render pass writing `targets` (resource pointers), ending
    /// whatever encoder is open.
    pub fn render_pass(
        &mut self,
        rp: &MTL4RenderPassDescriptor,
        targets: &[usize],
    ) -> Option<Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>> {
        self.render_pass_with_sync(rp, targets, targets, 0)
    }

    pub(super) fn render_pass_with_sync(
        &mut self,
        rp: &MTL4RenderPassDescriptor,
        sync_targets: &[usize],
        write_targets: &[usize],
        opening_pre_raster_reads: u8,
    ) -> Option<Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>> {
        self.end_encoder();
        self.bar.in_list_gfx_non_clear = true;
        for &r in sync_targets {
            if r != 0 {
                unsafe { (*(r as *const Resource)).mark_gpu_used() };
            }
        }
        for &r in write_targets {
            if r != 0 {
                unsafe { (*(r as *const Resource)).mark_gpu_used() };
                if !self.bar.in_list_rt_res_ids.contains(&r) {
                    self.bar.in_list_rt_res_ids.push(r);
                }
            }
        }
        if let Some(o) = &self.occlusion {
            unsafe { rp.setVisibilityResultBuffer(Some(&o.buffer)) };
            rp.setVisibilityResultType(MTLVisibilityResultType::Accumulate);
        }
        let e = self.cb.renderCommandEncoderWithDescriptor(rp)?;
        self.bar.has_gpu_work = true;
        self.rp_sync
            .reset_for_render_pass(sync_targets, opening_pre_raster_reads);
        self.begin_barrier(
            ProtocolObject::from_ref(&*e),
            sync_targets,
            true,
            opening_pre_raster_reads,
        );
        self.bar.pass_targets.clear();
        self.bar.pass_targets.extend_from_slice(write_targets);
        if let Some(o) = &self.occlusion {
            let mode = if o.binary {
                MTLVisibilityResultMode::Boolean
            } else {
                MTLVisibilityResultMode::Counting
            };
            unsafe { e.setVisibilityResultMode_offset(mode, o.index as usize * 8) };
        }
        self.enc = Encoder::Render(e.clone());
        if !sync_targets.is_empty() || !write_targets.is_empty() {
            self.note_stage_ran(MTLStages::Fragment);
        }
        self.pt_open(pt::kind::DRAW);
        Some(e)
    }

    /// Open a utility render pass (clears, resolves) writing `targets`
    /// without touching active occlusion queries.
    pub(super) fn util_render_pass(
        &mut self,
        rp: &MTL4RenderPassDescriptor,
        targets: &[usize],
        kind: u8,
    ) -> Option<Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>> {
        self.bar.opening_render_split_ok = false;
        self.end_encoder();
        for &r in targets {
            if r != 0 {
                unsafe { (*(r as *const Resource)).mark_gpu_used() };
            }
        }
        if kind == pt::kind::CLEAR {
            for &r in targets {
                if r != 0 && !self.bar.clear_rp_res_ids.contains(&r) {
                    self.bar.clear_rp_res_ids.push(r);
                }
                if r != 0 && !self.bar.clear_rp_unwaited_blit.contains(&r) {
                    self.bar.clear_rp_unwaited_blit.push(r);
                }
            }
        } else {
            self.bar.in_list_gfx_non_clear = true;
            for &r in targets {
                if r != 0 && !self.bar.in_list_rt_res_ids.contains(&r) {
                    self.bar.in_list_rt_res_ids.push(r);
                }
            }
        }
        let e = self.cb.renderCommandEncoderWithDescriptor(rp)?;
        self.bar.has_gpu_work = true;
        self.rp_sync.reset_for_render_pass(targets, 0);
        self.begin_barrier(ProtocolObject::from_ref(&*e), targets, true, 0);
        self.bar.pass_targets.clear();
        self.bar.pass_targets.extend_from_slice(targets);
        self.enc = Encoder::Render(e.clone());
        self.note_stage_ran(MTLStages::Fragment);
        self.pt_open(kind);
        Some(e)
    }
}
