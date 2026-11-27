//! Application `ResourceBarrier` intake (`barrier`, `barrier_list`, `barrier_list_with`),
//! encoder-opening barriers (`begin_barrier`), and mid-encoder queue sync
//! (`compute_mid_queue_sync_with`, `render_mid_barrier`).

use super::opts::*;
use super::state::*;
use super::CommandList;
use crate::metal::encstats as stats;
use crate::metal::pipeline::{READS_BUF, READS_TEX};
use crate::metal::queue::ListType;
use crate::metal::resource::{MtlTexture, Resource};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

impl CommandList {
    /// Barrier at the start of a new encoder: all -> all after an internal
    /// producer or unparsed barrier; the stage masks of parsed
    /// `ResourceBarrier`s; fragment -> fragment when the encoder attaches a
    /// resource an earlier render pass wrote without an intervening barrier
    /// (D3D12 orders same-state render-target/depth writes implicitly).
    pub(super) fn begin_barrier(
        &mut self,
        enc: &ProtocolObject<dyn MTL4CommandEncoder>,
        targets: &[usize],
        render: bool,
        opening_pre_raster_reads: u8,
    ) {
        if self.pending_mfx_wait_fence.is_some() {
            self.consume_pending_mfx_fence_on_enc_open(
                enc,
                render,
                self.bar.needs_barrier,
                opening_pre_raster_reads,
            );
        }
        let clear_cap = if render {
            self.bar.opening_clear_frag_cap.take()
        } else {
            self.bar.opening_clear_frag_cap = None;
            None
        };
        if render && clear_cap.is_none() {
            let res = std::mem::take(&mut self.bar.clear_tex_gfx_soft_res);
            let poison = std::mem::replace(&mut self.bar.clear_tex_gfx_soft_poisoned, false);
            if !self.bar.clear_tex_gfx_before.is_empty()
                || self.bar.clear_tex_gfx_soft != [MTLStages::empty(); 2]
            {
                let b = std::mem::replace(&mut self.bar.clear_tex_gfx_before, MTLStages::empty());
                let s =
                    std::mem::replace(&mut self.bar.clear_tex_gfx_soft, [MTLStages::empty(); 2]);
                if (self.opts.opt_cs_to_ps_overlap_v2 || self.gfx_to_ps_overlap_active())
                    && self.vs_buf_defer_exact_active()
                    && !poison
                    && !res.is_empty()
                    && s != [MTLStages::empty(); 2]
                {
                    self.add_pend_with_soft_res(MTLStages::Fragment, b, &res);
                } else {
                    self.add_pend_with_soft(MTLStages::Fragment, b, s);
                }
            }
        }
        let full = self.bar.needs_barrier;
        if !render && self.is_copy_queue_light() {
            let has_pend = !self.bar.pend_after.is_empty()
                && !(self.bar.pend_before
                    | self.bar.pend_before_soft[0]
                    | self.bar.pend_before_soft[1])
                    .is_empty();
            self.bar.needs_barrier = false;
            self.bar.list_sync_compute = MTLStages::empty();
            self.clear_all_pend();
            self.bar.vtx_carry = NO_VTX;
            self.bar.vtx_carry_req = [[0; 7]; 2];
            self.bar.vtx_carry_items[0].clear();
            self.bar.vtx_carry_items[1].clear();
            self.bar.vtx_carry_poisoned = [false; 2];
            if !full && !has_pend {
                self.bar.enc_sync_after = MTLStages::empty();
                self.bar.enc_sync_before = MTLStages::empty();
                return;
            }
            self.bar.enc_sync_after = MTLStages::Blit;
            self.bar.enc_sync_before = MTLStages::Blit;
            stats::inc(&stats::c::copy_q_light);
            stats::inc(if full && self.bar.full_why == FullWhy::ListStart {
                &stats::c::qb_list
            } else {
                &stats::c::qb_masked_other
            });
            enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                MTLStages::Blit,
                MTLStages::Blit,
                MTL4VisibilityOptions::Device,
            );
            self.record_enc_queue_sync(MTLStages::Blit, MTLStages::Blit);
            stats::inc(&stats::c::q_barrier_frag);
            return;
        }
        if self.list_barrier_tight_active() && !full {
            if !render {
                let cs_sync = std::mem::replace(&mut self.bar.list_sync_compute, MTLStages::empty());
                let _ = self.bar.split_pend.take_compute();
                self.reset_cs_soft_disp();
                let has_cs_pend = !self.bar.pend_after.is_empty()
                    && self.bar.pend_before.intersects(COMPUTE_STAGES);
                let after = cs_sync
                    | if has_cs_pend {
                        self.bar.pend_after
                    } else {
                        MTLStages::empty()
                    };
                let before = if !cs_sync.is_empty() {
                    COMPUTE_STAGES
                } else {
                    self.bar.pend_before & COMPUTE_STAGES
                };
                if has_cs_pend {
                    let rest = self.bar.pend_before - COMPUTE_STAGES;
                    if rest.is_empty() {
                        self.carry_soft();
                        self.clear_all_pend();
                    } else {
                        self.bar.pend_before = rest;
                        self.clear_pend_cs_for(COMPUTE_STAGES);
                        stats::inc(&stats::c::cs_keep_gfx);
                    }
                } else if !self.bar.pend_after.is_empty() {
                    stats::inc(&stats::c::cs_defer_gfx);
                }
                self.bar.needs_barrier = false;
                self.bar.enc_sync_after = after;
                self.bar.enc_sync_before = if after.is_empty() {
                    MTLStages::empty()
                } else {
                    before
                };
                if after.is_empty() {
                    return;
                }
                stats::inc(if !cs_sync.is_empty() {
                    &stats::c::qb_list_tight
                } else {
                    &stats::c::qb_masked_other
                });
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    after,
                    before,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(after, before);
                stats::inc(if after == MTLStages::All {
                    &stats::c::q_barrier_all
                } else {
                    &stats::c::q_barrier_frag
                });
                if !self.gfx.pass_uav && after.contains(MTLStages::Fragment | PRE_RASTER) {
                    self.bar.uav_gfx_dirty = false;
                }
                return;
            }

            let mut vtx_sync = std::mem::replace(&mut self.bar.list_sync_vtx, MTLStages::empty());
            let frag_sync = std::mem::replace(&mut self.bar.list_sync_frag, MTLStages::empty());
            let uav_draw = self.gfx.draw_pass
                && (self
                    .gfx
                    .pso
                    .as_ref()
                    .is_some_and(|p| self.gfx_pso_writes_uav(p))
                    || self
                        .gfx
                        .mesh
                        .as_ref()
                        .is_some_and(|m| self.mesh_pso_writes_uav(m)));
            if uav_draw {
                vtx_sync |= frag_sync;
            }
            let mut deferred = false;
            for i in 0..2 {
                let raw_soft =
                    std::mem::replace(&mut self.bar.pend_before_soft[i], MTLStages::empty());
                let carried = self.bar.vtx_carry[i].take();
                let cr = std::mem::take(&mut self.bar.vtx_carry_req[i]);
                self.bar.vtx_carry_items[i].clear();
                self.bar.vtx_carry_poisoned[i] = false;
                let Some((soft_after, soft)) =
                    merge_vtx(Some((self.bar.pend_after, raw_soft)), carried)
                else {
                    continue;
                };
                if soft_after.is_empty() || (soft - self.bar.pend_before).is_empty() {
                    continue;
                }
                let mut comb_req = [0u32; 7];
                if carried.is_some() {
                    merge_req_all(&mut comb_req, &cr);
                }
                if !raw_soft.is_empty() {
                    merge_req_for_after(&mut comb_req, self.bar.pend_after, &self.bar.stage_gen);
                }
                self.bar.vtx_defer[i] = Some((soft_after, soft - self.bar.pend_before));
                self.bar.vtx_defer_req[i] = comb_req;
                self.bar.vtx_defer_poisoned[i] = true;
                self.bar.vtx_defer_items[i].clear();
                deferred = true;
            }
            if deferred {
                stats::inc(&stats::c::dv_defer);
            }
            let mut pend_after = self.bar.pend_after;
            let mut pend_before = self.bar.pend_before;
            let mut by_written = false;
            if !pend_after.contains(MTLStages::Fragment)
                && !frag_sync.contains(MTLStages::Fragment)
                && targets.iter().any(|t| self.bar.written.contains(t))
            {
                by_written = pend_after.is_empty();
                pend_after |= MTLStages::Fragment;
                pend_before |= MTLStages::Fragment;
            }
            if deferred && pend_after.contains(MTLStages::Fragment) {
                pend_before |= MTLStages::Fragment;
            }
            let vtx_after = vtx_sync
                | if pend_before.intersects(PRE_RASTER) {
                    pend_after
                } else {
                    MTLStages::empty()
                };
            let frag_after = frag_sync
                | if pend_before.contains(MTLStages::Fragment) {
                    pend_after
                } else {
                    MTLStages::empty()
                };
            self.bar.needs_barrier = false;
            self.clear_all_pend();
            let narrow_vtx_only =
                self.gfx.draw_pass && !frag_sync.is_empty() && vtx_after != MTLStages::All;
            self.bar.enc_sync_after = if narrow_vtx_only {
                vtx_after
            } else {
                vtx_after | frag_after
            };
            self.bar.enc_sync_before = if self.bar.enc_sync_after.is_empty() {
                MTLStages::empty()
            } else if narrow_vtx_only {
                PRE_RASTER
            } else {
                (if !vtx_after.is_empty() {
                    PRE_RASTER
                } else {
                    MTLStages::empty()
                }) | (if !frag_after.is_empty() {
                    MTLStages::Fragment
                } else {
                    MTLStages::empty()
                })
            };
            let union_after = vtx_after | frag_after;
            if union_after.is_empty() {
                return;
            }
            let both = vtx_after & frag_after;
            let vtx_only = vtx_after - both;
            let frag_only = frag_after - both;
            stats::inc(if !(vtx_sync | frag_sync).is_empty() {
                &stats::c::qb_list_tight
            } else if by_written {
                &stats::c::qb_written
            } else if vtx_after.contains(MTLStages::Fragment) {
                &stats::c::qb_frag_to_vtx
            } else {
                &stats::c::qb_masked_other
            });
            if !both.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    both,
                    PRE_RASTER | MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(both, PRE_RASTER | MTLStages::Fragment);
            }
            if !vtx_only.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    vtx_only,
                    PRE_RASTER,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(vtx_only, PRE_RASTER);
            }
            if !frag_only.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    frag_only,
                    MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(frag_only, MTLStages::Fragment);
            }
            stats::inc(if both == MTLStages::All {
                &stats::c::q_barrier_all
            } else {
                &stats::c::q_barrier_frag
            });
            if frag_after.contains(MTLStages::Fragment) {
                self.bar.written.clear();
            }
            if !self.gfx.pass_uav && both.contains(MTLStages::Fragment | PRE_RASTER) {
                self.bar.uav_gfx_dirty = false;
            }
            return;
        }
        if self.render_split_sync_active() && !full {
            let orig_pend_after = self.bar.pend_after;
            let orig_pend_before = self.bar.pend_before;
            if !render {
                let mut stamp_blit_pre_stages = false;
                let split_cs_exact = self.split_compute_exact_active();
                let cs_mask = if split_cs_exact
                    && !(self.bar.opening_compute_stage & COMPUTE_STAGES).is_empty()
                {
                    self.bar.opening_compute_stage & COMPUTE_STAGES
                } else {
                    COMPUTE_STAGES
                };
                let had_any_cs_pend = self.bar.split_pend.has_compute();
                let (mut after, mut before, mut cs_req) =
                    self.bar.split_pend.take_compute_for(cs_mask);
                if cs_mask.contains(MTLStages::Dispatch) {
                    self.reset_cs_soft_disp();
                }
                self.clear_pend_cs_for(cs_mask);
                let had_cs_pend = !after.is_empty();
                if split_cs_exact
                    && had_any_cs_pend
                    && (!had_cs_pend || self.bar.split_pend.has_compute())
                {
                    stats::inc(&stats::c::split_cs_substage);
                }
                if self.bar.list_unwaited_cs {
                    if self.bar.opening_compute_stage == MTLStages::Blit
                        && (self.opts.opt_empty_enc_elide
                            || self.opts.opt_blit_list_narrow_v2
                            || self.opts.opt_list_boundary_overlap
                            || (self.opts.opt_queue_start_stage_exact
                                && self.bar.opening_blit_nonfrag_safe))
                    {
                        if !self.opts.opt_empty_enc_elide && !self.opts.opt_blit_list_narrow_v2 {
                            if self.opts.opt_list_boundary_overlap {
                                stats::inc(&stats::c::list_boundary_overlap);
                            } else if self.opts.opt_queue_start_stage_exact {
                                stats::inc(&stats::c::queue_start_stage_exact);
                            }
                        }
                        let a = if self.bar.opening_blit_nonfrag_safe {
                            stamp_blit_pre_stages = true;
                            COMPUTE_STAGES | PRE_RASTER
                        } else if self.opts.opt_list_start_substage {
                            self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All)
                        } else {
                            MTLStages::All
                        };
                        after |= a;
                        before |= MTLStages::Blit;
                        merge_req_for_after(&mut cs_req, a, &self.bar.stage_gen);
                        self.bar.enc_blit_only_sync = true;
                        self.bar.list_cs_unwaited_frag = self.bar.opening_blit_nonfrag_safe;
                    } else if self.opts.opt_list_start_substage {
                        let anchor = self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All);
                        if !self.mask_has_in_list_pre_list_all(after) {
                            after |= anchor;
                            merge_req_for_after(&mut cs_req, anchor, &self.bar.stage_gen);
                        }
                        before |= COMPUTE_STAGES;
                        self.bar.enc_blit_only_sync = false;
                        self.bar.list_cs_unwaited_frag = false;
                        self.bar.list_unwaited_cs = false;
                    } else if self.bar.opening_compute_stage == MTLStages::Blit {
                        after |= MTLStages::All;
                        before |= MTLStages::Blit;
                        merge_req_for_after(&mut cs_req, MTLStages::All, &self.bar.stage_gen);
                        self.bar.enc_blit_only_sync = true;
                        self.bar.list_cs_unwaited_frag = false;
                    } else {
                        after |= MTLStages::All;
                        before |= COMPUTE_STAGES;
                        merge_req_for_after(&mut cs_req, MTLStages::All, &self.bar.stage_gen);
                        self.bar.enc_blit_only_sync = false;
                        self.bar.list_cs_unwaited_frag = false;
                        self.bar.list_unwaited_cs = false;
                    }
                }
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
                    let sp_items = std::mem::take(&mut self.bar.soft_pend_items[i]);
                    let sp_poison = std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
                    let carried = self.bar.vtx_carry[i].take();
                    let cr = std::mem::take(&mut self.bar.vtx_carry_req[i]);
                    let c_items = std::mem::take(&mut self.bar.vtx_carry_items[i]);
                    let c_poison = std::mem::replace(&mut self.bar.vtx_carry_poisoned[i], false);
                    let mut comb_req = [0u32; 7];
                    let mut comb_items = SoftOblVec::new();
                    let mut comb_poison = false;
                    if !sa.is_empty() && !sb.is_empty() {
                        merge_req_for_after(&mut comb_req, sa, &sr);
                        comb_poison |= sp_poison;
                        if !comb_poison {
                            Self::extend_soft_items(&mut comb_items, sp_items);
                        }
                    }
                    if carried.is_some() {
                        merge_req_all(&mut comb_req, &cr);
                        comb_poison |= c_poison;
                        if !comb_poison {
                            Self::extend_soft_items(&mut comb_items, c_items);
                        } else {
                            comb_items.clear();
                        }
                    }
                    let merged = if !sa.is_empty() && !sb.is_empty() {
                        merge_vtx(Some((sa, sb)), carried)
                    } else {
                        carried
                    };
                    if let Some((ma, mb)) = merged {
                        if !ma.is_empty() && !mb.is_empty() {
                            self.bar.vtx_carry[i] = Some((ma, mb));
                            self.bar.vtx_carry_req[i] = comb_req;
                            self.bar.vtx_carry_poisoned[i] = comb_poison;
                            if !comb_poison {
                                self.bar.vtx_carry_items[i] = comb_items;
                            }
                        }
                    }
                }
                if had_cs_pend {
                    if !(self.bar.split_pend.vtx_after | self.bar.split_pend.frag_after).is_empty()
                    {
                        stats::inc(&stats::c::cs_keep_gfx);
                    }
                } else if !(self.bar.split_pend.vtx_after | self.bar.split_pend.frag_after)
                    .is_empty()
                {
                    stats::inc(&stats::c::cs_defer_gfx);
                }
                if after != orig_pend_after || before != orig_pend_before {
                    stats::inc(&stats::c::render_split_sync);
                }
                self.bar.needs_barrier = false;
                self.sync_legacy_from_split();
                let orig_after = after;
                if split_cs_exact && !orig_after.is_empty() {
                    after = self.fold_dominated_after(orig_after, &cs_req);
                    if after != orig_after {
                        stats::inc(&stats::c::split_cs_dom_fold);
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                } else if self.opts.opt_stagesync_barrier_v2 && !orig_after.is_empty() {
                    after = self.prune_dominated_after(orig_after);
                    if after != orig_after {
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                }
                let rec_after = if self.opts.opt_rp_keep_write_exact {
                    after
                } else {
                    orig_after
                };
                self.bar.enc_sync_after = rec_after;
                self.bar.enc_sync_before = if rec_after.is_empty() {
                    MTLStages::empty()
                } else {
                    before
                };
                if after.is_empty() {
                    return;
                }
                stats::inc(&stats::c::qb_masked_other);
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    after,
                    before,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(rec_after, before);
                if stamp_blit_pre_stages {
                    self.bar.enc_cur_sync[STAGE_BLIT].pre_list_stages |=
                        COMPUTE_STAGES | PRE_RASTER;
                }
                stats::inc(if after == MTLStages::All {
                    &stats::c::q_barrier_all
                } else {
                    &stats::c::q_barrier_frag
                });
                return;
            }

            let is_clear_cap = clear_cap.is_some();
            let was_carry_list_frag = if is_clear_cap {
                false
            } else {
                std::mem::replace(&mut self.bar.vtx_carry_list_frag, false)
            };
            let mut vtx_after;
            let mut vtx_req;
            let mut frag_after;
            let mut frag_req;
            let mut deferred = false;
            let mut merged_soft: [Option<(MTLStages, MTLStages)>; 2] = NO_VTX;
            let mut merged_soft_req: [[u32; 7]; 2] = [[0; 7]; 2];
            let mut merged_soft_items: [SoftOblVec; 2] = [SoftOblVec::new(), SoftOblVec::new()];
            let mut merged_soft_poison = [false; 2];
            if let Some((cap_frag, cap_req)) = clear_cap {
                if !self.bar.split_pend.vtx_after.is_empty()
                    || self.bar.split_pend.frag_after != cap_frag
                    || self.bar.split_pend.soft_after != [MTLStages::empty(); 2]
                    || self.bar.vtx_carry != NO_VTX
                {
                    if self.opts.opt_clear_pass_fold_v2 {
                        stats::inc(&stats::c::clear_pass_fold_v2);
                    } else if self.opts.opt_clear_isolate_coalesce {
                        stats::inc(&stats::c::clear_isolate_coalesce);
                    } else if self.opts.opt_clear_zero_qb_coalesce {
                        stats::inc(&stats::c::clear_zero_qb_coalesce);
                    }
                }
                self.consume_frag_rt_pend_for_clear(targets, cap_frag);
                vtx_after = MTLStages::empty();
                vtx_req = [0; 7];
                frag_after = cap_frag;
                frag_req = cap_req;
            } else {
                self.reset_frag_rt_pend();
                vtx_after =
                    std::mem::replace(&mut self.bar.split_pend.vtx_after, MTLStages::empty());
                vtx_req = std::mem::take(&mut self.bar.split_pend.vtx_req);
                frag_after =
                    std::mem::replace(&mut self.bar.split_pend.frag_after, MTLStages::empty());
                frag_req = std::mem::take(&mut self.bar.split_pend.frag_req);
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
                    let sp_items = std::mem::take(&mut self.bar.soft_pend_items[i]);
                    let sp_poison = std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
                    let carried = self.bar.vtx_carry[i].take();
                    let cr = std::mem::take(&mut self.bar.vtx_carry_req[i]);
                    let c_items = std::mem::take(&mut self.bar.vtx_carry_items[i]);
                    let c_poison = std::mem::replace(&mut self.bar.vtx_carry_poisoned[i], false);
                    let mut comb_req = [0u32; 7];
                    let mut comb_items = SoftOblVec::new();
                    let mut comb_poison = false;
                    if !sa.is_empty() && !sb.is_empty() {
                        merge_req_for_after(&mut comb_req, sa, &sr);
                        comb_poison |= sp_poison;
                        if !comb_poison {
                            Self::extend_soft_items(&mut comb_items, sp_items);
                        }
                    }
                    if carried.is_some() {
                        merge_req_all(&mut comb_req, &cr);
                        comb_poison |= c_poison;
                        if !comb_poison {
                            Self::extend_soft_items(&mut comb_items, c_items);
                        } else {
                            comb_items.clear();
                        }
                    }
                    let merged = if !sa.is_empty() && !sb.is_empty() {
                        merge_vtx(Some((sa, sb)), carried)
                    } else {
                        carried
                    };
                    let Some((soft_after, soft_before)) = merged else {
                        continue;
                    };
                    if soft_after.is_empty() || soft_before.is_empty() {
                        continue;
                    }
                    if (opening_pre_raster_reads & (1 << i)) != 0 {
                        if let Some((hit_a, _, hit_r, rem_opt)) = self
                            .partition_soft_items_for_cur_draw(
                                i,
                                &mut comb_items,
                                comb_poison,
                                soft_after,
                                soft_before,
                                &comb_req,
                                true,
                            )
                        {
                            if !hit_a.is_empty() {
                                vtx_after |= hit_a;
                                merge_req_for_after(&mut vtx_req, hit_a, &hit_r);
                            }
                            if let Some((rem_a, rem_b, rem_r)) = rem_opt {
                                merged_soft[i] = Some((rem_a, rem_b));
                                merged_soft_req[i] = rem_r;
                                merged_soft_items[i] = comb_items;
                            }
                        } else {
                            vtx_after |= soft_after;
                            merge_req_for_after(&mut vtx_req, soft_after, &comb_req);
                        }
                    } else if self.opts.opt_gfx_overlap_capped && !self.gfx_to_ps_overlap_active() {
                        if comb_poison {
                            let mut rem_a = soft_after;
                            if rem_a != MTLStages::All && rem_a.contains(MTLStages::Fragment) {
                                vtx_after |= MTLStages::Fragment;
                                vtx_req[STAGE_FRAGMENT] =
                                    vtx_req[STAGE_FRAGMENT].max(comb_req[STAGE_FRAGMENT]);
                                rem_a = sub_stages_preserving_all(rem_a, MTLStages::Fragment);
                            }
                            if !rem_a.is_empty() && !soft_before.is_empty() {
                                merged_soft[i] = Some((rem_a, soft_before));
                                let mut rem_r = [0u32; 7];
                                merge_req_for_after(&mut rem_r, rem_a, &comb_req);
                                merged_soft_req[i] = rem_r;
                                merged_soft_poison[i] = true;
                            }
                        } else {
                            let mut rem_a = MTLStages::empty();
                            let mut rem_b = MTLStages::empty();
                            let mut rem_r = [0u32; 7];
                            comb_items.retain_mut(|it| {
                                let is_sentinel = it.r == SOFT_RES_PRE_LIST_FRAG_TEX
                                    || it.r == SOFT_RES_PRE_LIST_FRAG_BUF;
                                if !is_sentinel
                                    && it.after != MTLStages::All
                                    && it.after.contains(MTLStages::Fragment)
                                {
                                    vtx_after |= MTLStages::Fragment;
                                    vtx_req[STAGE_FRAGMENT] =
                                        vtx_req[STAGE_FRAGMENT].max(it.req[STAGE_FRAGMENT]);
                                    it.after =
                                        sub_stages_preserving_all(it.after, MTLStages::Fragment);
                                    it.req[STAGE_FRAGMENT] = 0;
                                }
                                let ib = it.before & PRE_RASTER;
                                if it.after.is_empty() || ib.is_empty() {
                                    false
                                } else {
                                    rem_a |= it.after;
                                    rem_b |= ib;
                                    merge_req_for_after(&mut rem_r, it.after, &it.req);
                                    true
                                }
                            });
                            if !rem_a.is_empty() && !rem_b.is_empty() {
                                merged_soft[i] = Some((rem_a, rem_b));
                                merged_soft_req[i] = rem_r;
                                merged_soft_items[i] = comb_items;
                            }
                        }
                    } else {
                        merged_soft[i] = Some((soft_after, soft_before));
                        merged_soft_req[i] = comb_req;
                        merged_soft_items[i] = comb_items;
                        merged_soft_poison[i] = comb_poison;
                    }
                }
            }
            let mut by_written = false;
            if !frag_after.contains(MTLStages::Fragment)
                && targets.iter().any(|t| self.bar.written.contains(t))
            {
                by_written = !vtx_after.contains(MTLStages::Fragment);
                frag_after |= MTLStages::Fragment;
                frag_req[STAGE_FRAGMENT] =
                    frag_req[STAGE_FRAGMENT].max(self.bar.stage_gen[STAGE_FRAGMENT]);
            }
            if self.bar.list_unwaited_gfx {
                if self.opts.opt_list_start_substage {
                    let anchor = self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All);
                    if !self.mask_has_in_list_pre_list_all(vtx_after) {
                        vtx_after |= anchor;
                        merge_req_for_after(&mut vtx_req, anchor, &self.bar.stage_gen);
                    }
                    if !self.mask_has_in_list_pre_list_all(frag_after) {
                        frag_after |= anchor;
                        merge_req_for_after(&mut frag_req, anchor, &self.bar.stage_gen);
                    }
                } else {
                    vtx_after |= MTLStages::All;
                    frag_after |= MTLStages::All;
                    merge_req_for_after(&mut vtx_req, MTLStages::All, &self.bar.stage_gen);
                    merge_req_for_after(&mut frag_req, MTLStages::All, &self.bar.stage_gen);
                }
                if vtx_after.contains(MTLStages::Fragment) {
                    by_written = false;
                }
                self.bar.list_unwaited_gfx = false;
            }
            let mut deferred_inlist_frag = false;
            for (i, item) in merged_soft.into_iter().enumerate() {
                let Some((soft_after, soft_before)) = item else {
                    continue;
                };
                let rem_after = sub_stages_preserving_all(soft_after, vtx_after);
                if rem_after.is_empty() || soft_before.is_empty() {
                    continue;
                }
                self.bar.vtx_defer[i] = Some((rem_after, soft_before));
                let mut m_items = std::mem::take(&mut merged_soft_items[i]);
                let mut m_poison = merged_soft_poison[i];
                if was_carry_list_frag && rem_after == MTLStages::All {
                    self.bar.vtx_defer_list_frag = true;
                    if !self.list_rsplit_v2_active() {
                        m_poison = true;
                    }
                }
                if !vtx_after.is_empty() {
                    Self::strip_soft_items_after(&mut m_items, vtx_after);
                }
                let has_non_sentinel_inlist_frag = if !m_poison {
                    m_items.iter().any(|it| {
                        it.r != SOFT_RES_PRE_LIST_FRAG_TEX
                            && it.r != SOFT_RES_PRE_LIST_FRAG_BUF
                            && it.after != MTLStages::All
                            && it.after.contains(MTLStages::Fragment)
                    })
                } else {
                    !was_carry_list_frag
                        && rem_after != MTLStages::All
                        && rem_after.contains(MTLStages::Fragment)
                };
                if has_non_sentinel_inlist_frag {
                    deferred_inlist_frag = true;
                }
                self.bar.vtx_defer_poisoned[i] = m_poison;
                if !m_poison {
                    self.bar.vtx_defer_items[i] = m_items;
                } else {
                    self.bar.vtx_defer_items[i].clear();
                }
                let mut dr = [0u32; 7];
                merge_req_for_after(&mut dr, rem_after, &merged_soft_req[i]);
                self.bar.vtx_defer_req[i] = dr;
                deferred = true;
            }
            if deferred_inlist_frag && self.opts.opt_gfx_overlap_capped {
                self.rp_sync.gfx_ps_overlap_chain =
                    self.rp_sync.gfx_ps_overlap_chain.saturating_add(1);
                stats::inc(&stats::c::gfx_overlap_capped);
            }
            if deferred {
                stats::inc(&stats::c::dv_defer);
            }
            self.bar.needs_barrier = false;
            if !is_clear_cap {
                self.sync_legacy_from_split();
            }
            let orig_vtx = vtx_after;
            let orig_frag = frag_after;
            if !is_clear_cap
                && (!(orig_frag - orig_vtx).is_empty()
                    || ((orig_vtx | orig_frag) != orig_pend_after && !orig_pend_after.is_empty()))
            {
                stats::inc(&stats::c::render_split_sync);
            }
            if self.render_dom_exact_active() {
                if !orig_vtx.is_empty() {
                    vtx_after = self.fold_dominated_after(orig_vtx, &vtx_req);
                }
                if !orig_frag.is_empty() {
                    frag_after = self.fold_dominated_after(orig_frag, &frag_req);
                }
                if vtx_after != orig_vtx || frag_after != orig_frag {
                    stats::inc(&stats::c::render_dom_fold);
                    stats::inc(&stats::c::stagesync_narrow);
                }
            } else if self.opts.opt_stagesync_barrier_v2 {
                if !orig_vtx.is_empty() {
                    vtx_after = self.prune_dominated_after(orig_vtx);
                }
                if !orig_frag.is_empty() {
                    frag_after = self.prune_dominated_after(orig_frag);
                }
                if vtx_after != orig_vtx || frag_after != orig_frag {
                    stats::inc(&stats::c::stagesync_narrow);
                }
            }
            let pre_b = self.cur_pre_raster_stages();
            let omitted_pre = PRE_RASTER - pre_b;
            let rec_vtx = if self.opts.opt_rp_keep_write_exact {
                vtx_after
            } else {
                orig_vtx
            };
            let rec_frag = if self.opts.opt_rp_keep_write_exact {
                frag_after
            } else {
                orig_frag
            };
            self.bar.enc_sync_after = rec_vtx | rec_frag;
            self.bar.enc_sync_before = (if !rec_vtx.is_empty() {
                pre_b
            } else {
                MTLStages::empty()
            }) | (if !rec_frag.is_empty() {
                MTLStages::Fragment
            } else {
                MTLStages::empty()
            });
            if !orig_vtx.is_empty() && !omitted_pre.is_empty() {
                self.bar.enc_pre_unwaited_stages |= omitted_pre;
                self.bar.enc_pre_unwaited_after |= orig_vtx;
                merge_req_for_after(&mut self.bar.enc_pre_unwaited_req, orig_vtx, &vtx_req);
            }
            if (vtx_after | frag_after).is_empty() {
                if !is_clear_cap
                    && !self.gfx.pass_uav
                    && (0..4).all(|si| self.sync_covers_gfx_uav(&self.bar.enc_cur_sync[si]))
                {
                    self.bar.uav_gfx_dirty = false;
                }
                return;
            }
            let both = vtx_after & frag_after;
            let vtx_only = vtx_after - both;
            let frag_only = frag_after - both;
            stats::inc(if by_written {
                &stats::c::qb_written
            } else if vtx_after.contains(MTLStages::Fragment) {
                &stats::c::qb_frag_to_vtx
            } else {
                &stats::c::qb_masked_other
            });
            if !both.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    both,
                    pre_b | MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
            }
            if !vtx_only.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    vtx_only,
                    pre_b,
                    MTL4VisibilityOptions::Device,
                );
            }
            if !frag_only.is_empty() {
                enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    frag_only,
                    MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
            }
            if !rec_vtx.is_empty() {
                self.record_enc_queue_sync(rec_vtx, pre_b);
                if (self.opts.opt_cs_to_ps_overlap_v2 || self.list_rsplit_v2_active())
                    && self.render_dom_exact_active()
                {
                    let mid_on = self.mid_qb_stagesync_active();
                    for i in 0..2 {
                        if let Some((da, db)) = self.bar.vtx_defer[i] {
                            let unsat = self.unsatisfied_enc_after_for_req(
                                da,
                                db,
                                &self.bar.vtx_defer_req[i],
                                mid_on,
                            );
                            if unsat.is_empty() {
                                self.bar.vtx_defer[i] = None;
                                self.bar.vtx_defer_req[i] = [0; 7];
                                self.bar.vtx_defer_items[i].clear();
                                self.bar.vtx_defer_poisoned[i] = false;
                            } else if unsat != da {
                                let stripped = sub_stages_preserving_all(da, unsat);
                                self.bar.vtx_defer[i] = Some((unsat, db));
                                if !self.bar.vtx_defer_poisoned[i] {
                                    Self::strip_soft_items_after(
                                        &mut self.bar.vtx_defer_items[i],
                                        stripped,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            if !rec_frag.is_empty() {
                self.record_enc_queue_sync(rec_frag, MTLStages::Fragment);
            }
            stats::inc(if both == MTLStages::All {
                &stats::c::q_barrier_all
            } else {
                &stats::c::q_barrier_frag
            });
            if !self.opts.opt_rp_keep_write_exact {
                if orig_frag.contains(MTLStages::Fragment) {
                    self.bar.written.clear();
                }
            } else if frag_after.contains(MTLStages::Fragment)
                || (orig_frag.contains(MTLStages::Fragment)
                    && self.bar.enc_cur_sync[STAGE_FRAGMENT].gen[STAGE_FRAGMENT]
                        == self.bar.stage_gen[STAGE_FRAGMENT])
            {
                self.bar.written.clear();
            }
            if !is_clear_cap
                && !self.gfx.pass_uav
                && ((pre_b == PRE_RASTER
                    && (rec_vtx & rec_frag).contains(MTLStages::Fragment | PRE_RASTER))
                    || (0..4).all(|si| self.sync_covers_gfx_uav(&self.bar.enc_cur_sync[si])))
            {
                self.bar.uav_gfx_dirty = false;
            }
            return;
        }
        if full {
            if render {
                self.bar.list_sync_vtx = MTLStages::empty();
                self.bar.list_sync_frag = MTLStages::empty();
            } else {
                self.bar.list_sync_compute = MTLStages::empty();
            }
        }
        let blit_narrow = !render
            && full
            && self.bar.full_why == FullWhy::ListStart
            && (self.opts.opt_empty_enc_elide
                || self.opts.opt_blit_list_narrow_v2
                || self.opts.opt_list_boundary_overlap
                || (self.opts.opt_queue_start_stage_exact && self.bar.opening_blit_nonfrag_safe)
                || self.bar.opening_blit_explicit_after.is_some())
            && !legacy_barriers()
            && barrier_masks()
            && !self.list_barrier_tight_active()
            && self.bar.opening_compute_stage == MTLStages::Blit;
        let list_substage = full
            && self.bar.full_why == FullWhy::ListStart
            && self.opts.opt_list_start_substage
            && !blit_narrow
            && !legacy_barriers()
            && barrier_masks()
            && !self.is_copy_queue_light()
            && !self.list_barrier_tight_active();
        const FRAG_WRITE_FLAGS: u32 = crate::types::resource_flags::ALLOW_RENDER_TARGET
            | crate::types::resource_flags::ALLOW_DEPTH_STENCIL
            | crate::types::resource_flags::ALLOW_UNORDERED_ACCESS;
        let legacy_rsplit_ok = (self.opts.opt_list_start_render_split
            || self.opts.opt_list_boundary_overlap
            || self.list_rsplit_v2_active())
            && !self.has_frag_buf_write()
            && (opening_pre_raster_reads & READS_TEX) == 0
            && !self.bar.split_pend.vtx_after.contains(MTLStages::Fragment)
            && self.bar.split_pend.vtx_after != MTLStages::All
            && !((opening_pre_raster_reads & READS_BUF) != 0
                && (self.bar.split_pend.soft_after[1].contains(MTLStages::Fragment)
                    || self.bar.split_pend.soft_after[1] == MTLStages::All));
        let exact_rsplit_ok = (self.opts.opt_queue_start_stage_exact || self.list_rsplit_v2_active())
            && self.occlusion.is_none()
            && self.rp_sync.cur_draw_exact_valid
            && self.rp_sync.cur_draw_pre_unb == 0
            && !self.bar.split_pend.vtx_after.contains(MTLStages::Fragment)
            && self.bar.split_pend.vtx_after != MTLStages::All
            && self.rp_sync.cur_draw_pre_exact.iter().all(|&r| {
                r != 0
                    && !self.bar.written.contains(&r)
                    && !is_frag_resolve_dst(r)
                    && unsafe {
                        let res = &*(r as *const Resource);
                        let not_aliased = if self.opts.opt_gfx_overlap_capped
                            || self.opts.opt_gfx_overlap_frag_cap
                        {
                            !res.may_alias_placed()
                        } else {
                            res.placed_in.is_none()
                        };
                        not_aliased && (res.desc.Flags & FRAG_WRITE_FLAGS == 0)
                    }
            });
        if list_substage
            && render
            && self.render_split_sync_active()
            && self.bar.opening_render_split_ok
            && (legacy_rsplit_ok || exact_rsplit_ok)
        {
            let orig_sa1 = self.bar.split_pend.soft_after[1];
            let sb1 = self.bar.split_pend.soft_before[1] & PRE_RASTER;
            let sr1 = self.bar.split_pend.soft_req[1];
            let frag_buf_possible = !legacy_rsplit_ok && self.has_frag_buf_write();
            self.bar.enc_blit_only_sync = false;
            self.bar.list_cs_unwaited_frag = true;
            self.bar.list_unwaited_cs = true;
            self.bar.list_unwaited_gfx = false;
            self.bar.needs_barrier = false;
            self.clear_all_pend();
            self.bar.vtx_carry = NO_VTX;
            self.bar.vtx_carry_req = [[0; 7]; 2];
            self.bar.vtx_carry_items[0].clear();
            self.bar.vtx_carry_items[1].clear();
            self.bar.vtx_carry_poisoned = [false; 2];
            self.bar.vtx_carry_list_frag = false;
            self.bar.vtx_defer = NO_VTX;
            self.bar.vtx_defer_req = [[0; 7]; 2];
            self.bar.vtx_defer_items[0].clear();
            self.bar.vtx_defer_items[1].clear();
            self.bar.vtx_defer_poisoned = [false; 2];
            self.bar.arm_post_draw0_list_frag = [false; 2];
            if (opening_pre_raster_reads & READS_TEX) == 0 {
                self.bar.vtx_defer[0] = Some((MTLStages::All, PRE_RASTER));
                merge_req_for_after(
                    &mut self.bar.vtx_defer_req[0],
                    MTLStages::All,
                    &self.bar.stage_gen,
                );
                if self.list_rsplit_v2_active() {
                    self.bar.vtx_defer_poisoned[0] = false;
                    self.bar.vtx_defer_items[0].push(SoftPreResObl {
                        r: SOFT_RES_PRE_LIST_FRAG_TEX,
                        after: MTLStages::All,
                        before: PRE_RASTER,
                        req: self.bar.vtx_defer_req[0],
                    });
                } else {
                    self.bar.vtx_defer_poisoned[0] = true;
                }
                self.bar.vtx_defer_list_frag = true;
            } else {
                self.bar.arm_post_draw0_list_frag[0] = true;
            }
            if frag_buf_possible {
                if (opening_pre_raster_reads & READS_BUF) == 0 {
                    self.bar.vtx_defer[1] = Some((MTLStages::All, PRE_RASTER));
                    merge_req_for_after(
                        &mut self.bar.vtx_defer_req[1],
                        MTLStages::All,
                        &self.bar.stage_gen,
                    );
                    if self.list_rsplit_v2_active() {
                        self.bar.vtx_defer_poisoned[1] = false;
                        self.bar.vtx_defer_items[1].push(SoftPreResObl {
                            r: SOFT_RES_PRE_LIST_FRAG_BUF,
                            after: MTLStages::All,
                            before: PRE_RASTER,
                            req: self.bar.vtx_defer_req[1],
                        });
                    } else {
                        self.bar.vtx_defer_poisoned[1] = true;
                    }
                    self.bar.vtx_defer_list_frag = true;
                } else {
                    self.bar.arm_post_draw0_list_frag[1] = true;
                }
            } else if !orig_sa1.is_empty() && !sb1.is_empty() {
                let rem1 = sub_stages_preserving_all(orig_sa1, COMPUTE_STAGES | PRE_RASTER);
                if !rem1.is_empty() {
                    if (opening_pre_raster_reads & READS_BUF) == 0 {
                        self.bar.vtx_defer[1] = Some((rem1, sb1));
                        merge_req_for_after(&mut self.bar.vtx_defer_req[1], rem1, &sr1);
                        self.bar.vtx_defer_poisoned[1] = true;
                    } else {
                        self.bar.arm_post_draw0_list_frag[1] = true;
                    }
                }
            }
            self.bar.list_unwaited_vtx_frag = true;
            let pre_b = self.cur_pre_raster_stages();
            let omitted_pre = PRE_RASTER - pre_b;
            let vtx_open = COMPUTE_STAGES | PRE_RASTER;
            self.bar.enc_sync_after = vtx_open;
            self.bar.enc_sync_before = pre_b | MTLStages::Fragment;
            enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                vtx_open,
                pre_b,
                MTL4VisibilityOptions::Device,
            );
            enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
                MTLStages::All,
                MTLStages::Fragment,
                MTL4VisibilityOptions::Device,
            );
            self.record_enc_queue_sync(vtx_open, pre_b);
            for si in 0..3 {
                if pre_b.contains(STAGE_BITS[si]) {
                    self.bar.enc_cur_sync[si].pre_list_stages |= vtx_open;
                }
            }
            if !omitted_pre.is_empty() {
                self.bar.enc_pre_unwaited_stages |= omitted_pre;
                self.bar.enc_pre_unwaited_after |= vtx_open;
                merge_req_for_after(&mut self.bar.enc_pre_unwaited_req, vtx_open, &self.bar.stage_gen);
            }
            self.record_enc_queue_sync(MTLStages::All, MTLStages::Fragment);
            self.bar.written.clear();
            stats::inc(&stats::c::list_start_substage);
            if self.opts.opt_gfx_overlap_frag_cap {
                stats::inc(&stats::c::gfx_overlap_frag_cap);
            } else if self.opts.opt_gfx_overlap_capped {
                stats::inc(&stats::c::gfx_overlap_capped);
            } else if self.opts.opt_gfx_to_ps_overlap_v2 {
                stats::inc(&stats::c::gfx_to_ps_overlap_v2);
            } else if self.opts.opt_list_start_render_split {
                stats::inc(&stats::c::list_start_rsplit);
            } else if self.opts.opt_list_boundary_overlap {
                stats::inc(&stats::c::list_boundary_overlap);
            } else {
                stats::inc(&stats::c::queue_start_stage_exact);
            }
            stats::inc(&stats::c::dv_defer);
            stats::inc(&stats::c::qb_list);
            stats::inc(&stats::c::q_barrier_frag);
            return;
        }
        let mut stamp_blit_pre_stages: Option<MTLStages> = None;
        let (mut after, mut before) = if blit_narrow {
            self.bar.enc_blit_only_sync = true;
            self.bar.list_unwaited_cs = true;
            self.bar.list_unwaited_gfx = true;
            stats::inc(&stats::c::blit_list_narrow);
            if self.opts.opt_blit_list_narrow_v2 {
                stats::inc(&stats::c::blit_list_v2);
            } else if self.opts.opt_list_boundary_overlap && !self.opts.opt_empty_enc_elide {
                stats::inc(&stats::c::list_boundary_overlap);
            } else if self.opts.opt_queue_start_stage_exact && !self.opts.opt_empty_enc_elide {
                stats::inc(&stats::c::queue_start_stage_exact);
            }
            const ALL_KNOWN: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(COMPUTE_STAGES);
            let a = if self.bar.opening_blit_nonfrag_safe
                && self
                    .bar
                    .opening_blit_explicit_after
                    .is_none_or(|st| st.contains(MTLStages::Dispatch))
            {
                self.bar.opening_blit_explicit_after = None;
                stamp_blit_pre_stages = Some(COMPUTE_STAGES | PRE_RASTER);
                self.bar.list_blit_unwaited_stages = MTLStages::empty();
                self.bar.list_cs_unwaited_frag = true;
                COMPUTE_STAGES | PRE_RASTER
            } else if let Some(exp_a) = self.bar.opening_blit_explicit_after.take() {
                let eff = if self.bar.opening_blit_nonfrag_safe {
                    exp_a & (COMPUTE_STAGES | PRE_RASTER)
                } else {
                    exp_a
                };
                if eff != MTLStages::All {
                    stamp_blit_pre_stages = Some(eff);
                    self.bar.list_blit_unwaited_stages = ALL_KNOWN - eff;
                    self.bar.list_cs_unwaited_frag = !eff.contains(MTLStages::Fragment);
                } else {
                    self.bar.list_blit_unwaited_stages = MTLStages::empty();
                    self.bar.list_cs_unwaited_frag = false;
                }
                eff
            } else {
                self.bar.list_blit_unwaited_stages = MTLStages::empty();
                self.bar.list_cs_unwaited_frag = false;
                MTLStages::All
            };
            (a, MTLStages::Blit)
        } else if list_substage {
            stats::inc(&stats::c::list_start_substage);
            self.bar.enc_blit_only_sync = false;
            self.bar.list_cs_unwaited_frag = false;
            self.bar.list_blit_unwaited_stages = MTLStages::empty();
            if render {
                self.bar.list_unwaited_cs = true;
                self.bar.list_unwaited_gfx = false;
                let pre_b = self.cur_pre_raster_stages();
                let omitted_pre = PRE_RASTER - pre_b;
                if !omitted_pre.is_empty() {
                    self.bar.enc_pre_unwaited_stages |= omitted_pre;
                    self.bar.enc_pre_unwaited_after = MTLStages::All;
                    merge_req_for_after(
                        &mut self.bar.enc_pre_unwaited_req,
                        MTLStages::All,
                        &self.bar.stage_gen,
                    );
                }
                (MTLStages::All, pre_b | MTLStages::Fragment)
            } else if self.opts.opt_clear_prelist_overlap && !self.clears.pending_clears.is_empty()
            {
                self.bar.list_unwaited_cs = false;
                self.bar.list_unwaited_gfx = false;
                stats::inc(&stats::c::clear_prelist_overlap);
                (MTLStages::All, MTLStages::All)
            } else {
                self.bar.list_unwaited_cs = false;
                self.bar.list_unwaited_gfx = true;
                (MTLStages::All, COMPUTE_STAGES)
            }
        } else if full {
            self.bar.enc_blit_only_sync = false;
            self.bar.list_cs_unwaited_frag = false;
            self.bar.list_blit_unwaited_stages = MTLStages::empty();
            self.bar.list_unwaited_cs = false;
            self.bar.list_unwaited_gfx = false;
            self.bar.enc_pre_unwaited_stages = MTLStages::empty();
            self.bar.enc_pre_unwaited_after = MTLStages::empty();
            self.bar.enc_pre_unwaited_req = [0; 7];
            (MTLStages::All, MTLStages::All)
        } else {
            let (mut a, mut b) = (self.bar.pend_after, self.bar.pend_before);
            if render && self.bar.list_unwaited_gfx {
                if self.opts.opt_list_start_substage {
                    let anchor = self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All);
                    if !self.mask_has_in_list_pre_list_all(a) {
                        a |= anchor;
                    }
                } else {
                    a |= MTLStages::All;
                }
                b |= PRE_RASTER | MTLStages::Fragment;
                self.bar.list_unwaited_gfx = false;
            } else if !render && self.bar.list_unwaited_cs {
                if self.bar.opening_compute_stage == MTLStages::Blit
                    && (self.opts.opt_empty_enc_elide
                        || self.opts.opt_blit_list_narrow_v2
                        || self.opts.opt_list_boundary_overlap
                        || (self.opts.opt_queue_start_stage_exact
                            && self.bar.opening_blit_nonfrag_safe))
                {
                    if !self.opts.opt_empty_enc_elide && !self.opts.opt_blit_list_narrow_v2 {
                        if self.opts.opt_list_boundary_overlap {
                            stats::inc(&stats::c::list_boundary_overlap);
                        } else if self.opts.opt_queue_start_stage_exact {
                            stats::inc(&stats::c::queue_start_stage_exact);
                        }
                    }
                    a |= if self.bar.opening_blit_nonfrag_safe {
                        stamp_blit_pre_stages = Some(COMPUTE_STAGES | PRE_RASTER);
                        COMPUTE_STAGES | PRE_RASTER
                    } else if self.opts.opt_list_start_substage {
                        self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All)
                    } else {
                        MTLStages::All
                    };
                    b |= MTLStages::Blit;
                    self.bar.enc_blit_only_sync = true;
                    self.bar.list_cs_unwaited_frag = self.bar.opening_blit_nonfrag_safe;
                } else if self.opts.opt_list_start_substage {
                    let anchor = self.in_list_pre_list_all_anchor().unwrap_or(MTLStages::All);
                    if !self.mask_has_in_list_pre_list_all(a) {
                        a |= anchor;
                    }
                    b |= COMPUTE_STAGES;
                    self.bar.enc_blit_only_sync = false;
                    self.bar.list_cs_unwaited_frag = false;
                    self.bar.list_unwaited_cs = false;
                } else if self.bar.opening_compute_stage == MTLStages::Blit {
                    a |= MTLStages::All;
                    b |= MTLStages::Blit;
                    self.bar.enc_blit_only_sync = true;
                    self.bar.list_cs_unwaited_frag = false;
                } else {
                    a |= MTLStages::All;
                    b |= COMPUTE_STAGES;
                    self.bar.enc_blit_only_sync = false;
                    self.bar.list_cs_unwaited_frag = false;
                    self.bar.list_unwaited_cs = false;
                }
            }
            (a, b)
        };
        // Pre-raster reads of SRV/UAV-state resources (and obligations
        // carried from earlier passes), per resource class.
        let mut deferred = false;
        for i in 0..2 {
            let raw_soft = std::mem::replace(&mut self.bar.pend_before_soft[i], MTLStages::empty());
            let carried = self.bar.vtx_carry[i].take();
            let cr = std::mem::take(&mut self.bar.vtx_carry_req[i]);
            self.bar.vtx_carry_items[i].clear();
            self.bar.vtx_carry_poisoned[i] = false;
            let Some((soft_after, soft)) =
                merge_vtx(Some((self.bar.pend_after, raw_soft)), carried)
            else {
                continue;
            };
            if (full && !blit_narrow) || soft_after.is_empty() || (soft - before).is_empty() {
                continue;
            }
            let mut comb_req = [0u32; 7];
            if carried.is_some() {
                merge_req_all(&mut comb_req, &cr);
            }
            if !raw_soft.is_empty() {
                merge_req_for_after(&mut comb_req, self.bar.pend_after, &self.bar.stage_gen);
            }
            let soft = soft - before;
            if render {
                self.bar.vtx_defer[i] = Some((soft_after, soft));
                self.bar.vtx_defer_req[i] = comb_req;
                self.bar.vtx_defer_poisoned[i] = true;
                self.bar.vtx_defer_items[i].clear();
                deferred = true;
            } else {
                self.bar.vtx_carry[i] = Some((soft_after, soft));
                self.bar.vtx_carry_req[i] = comb_req;
                self.bar.vtx_carry_poisoned[i] = true;
                self.bar.vtx_carry_items[i].clear();
            }
        }
        if deferred {
            stats::inc(&stats::c::dv_defer);
        }
        let mut by_written = false;
        if !after.contains(MTLStages::Fragment)
            && targets.iter().any(|t| self.bar.written.contains(t))
        {
            by_written = after.is_empty();
            after |= MTLStages::Fragment;
            before |= MTLStages::Fragment;
        }
        // Attachment writes stay ordered behind the fragment work waited for.
        if deferred && after.contains(MTLStages::Fragment) {
            before |= MTLStages::Fragment;
        }
        self.bar.needs_barrier = false;
        self.clear_all_pend();
        let orig_after = after;
        if self.opts.opt_stagesync_barrier_v2 && !full && !blit_narrow {
            after = self.prune_dominated_after(orig_after);
            if after != orig_after {
                stats::inc(&stats::c::stagesync_narrow);
            }
        }
        self.bar.enc_sync_after = orig_after;
        self.bar.enc_sync_before = if orig_after.is_empty() {
            MTLStages::empty()
        } else {
            before
        };
        if after.is_empty() {
            return;
        }
        stats::inc(match (full && !blit_narrow, self.bar.full_why) {
            (true, FullWhy::ListStart) => &stats::c::qb_list,
            (true, FullWhy::App) => &stats::c::qb_app_full,
            (true, FullWhy::ClearUav) => &stats::c::qb_clear_uav,
            (true, FullWhy::Internal) => &stats::c::qb_internal,
            _ if by_written => &stats::c::qb_written,
            _ if after.contains(MTLStages::Fragment) && before.intersects(PRE_RASTER) => {
                &stats::c::qb_frag_to_vtx
            }
            _ => &stats::c::qb_masked_other,
        });
        enc.barrierAfterQueueStages_beforeStages_visibilityOptions(
            after,
            before,
            MTL4VisibilityOptions::Device,
        );
        self.record_enc_queue_sync(orig_after, before);
        if let Some(st) = stamp_blit_pre_stages {
            self.bar.enc_cur_sync[STAGE_BLIT].pre_list_stages |= st;
        }
        stats::inc(if after == MTLStages::All && before == MTLStages::All {
            &stats::c::q_barrier_all
        } else {
            &stats::c::q_barrier_frag
        });
        if orig_after.contains(MTLStages::Fragment) {
            self.bar.written.clear();
        }
        if !self.gfx.pass_uav
            && (orig_after.contains(MTLStages::Fragment | PRE_RASTER)
                || (render && (0..4).all(|si| self.sync_covers_gfx_uav(&self.bar.enc_cur_sync[si]))))
        {
            self.bar.uav_gfx_dirty = false;
        }
    }

    /// Sync pending cross-encoder barriers on an already-open compute encoder.
    pub(super) fn compute_mid_queue_sync_with(
        &mut self,
        e: &ProtocolObject<dyn MTL4ComputeCommandEncoder>,
        full: bool,
        op_stage: MTLStages,
        op_touch: Option<&[usize]>,
        exact: Option<DispatchExact<'_>>,
    ) {
        if self.pending_mfx_wait_fence.is_some() {
            let mfx_cs = if !(op_stage & COMPUTE_STAGES).is_empty() {
                op_stage & COMPUTE_STAGES
            } else {
                COMPUTE_STAGES
            };
            self.wait_pending_mfx_fence(ProtocolObject::from_ref(e), mfx_cs);
        }
        if full {
            self.bar.cs_mid_v3_items.clear();
            self.bar.cs_mid_v3_poisoned = [false; 3];
            self.bar.pend_cs_items.clear();
            self.bar.pend_cs_poisoned = [false; 3];
            self.reset_cs_soft_disp();
        }
        if full && self.is_copy_queue_light() {
            if !(self.bar.enc_sync_after.contains(MTLStages::Blit)
                && self.bar.enc_sync_before.contains(MTLStages::Blit))
            {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    MTLStages::Blit,
                    MTLStages::Blit,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(MTLStages::Blit, MTLStages::Blit);
                stats::inc(&stats::c::cs_mid_qb);
                stats::inc(&stats::c::copy_q_light);
                self.note_enc_sync_pair(MTLStages::Blit, MTLStages::Blit);
            }
            self.bar.needs_barrier = false;
            self.clear_all_pend();
            return;
        }
        if !full && self.render_split_sync_active() {
            let split_cs_exact = self.split_compute_exact_active();
            let cs_mask = if split_cs_exact && !(op_stage & COMPUTE_STAGES).is_empty() {
                op_stage & COMPUTE_STAGES
            } else {
                COMPUTE_STAGES
            };
            if !self.bar.split_pend.has_compute_for(cs_mask) {
                if split_cs_exact && self.bar.split_pend.has_compute() {
                    stats::inc(&stats::c::split_cs_substage);
                } else if !self.bar.split_pend.is_empty() {
                    stats::inc(&stats::c::cs_defer_gfx);
                }
                return;
            }
            let orig_pend_after = self.bar.pend_after;
            let orig_pend_cs_before = self.bar.pend_before & COMPUTE_STAGES;
            let (qa, eff_qb, cs_req) = self.bar.split_pend.take_compute_for(cs_mask);
            if cs_mask.contains(MTLStages::Dispatch) {
                self.reset_cs_soft_disp();
            }
            if split_cs_exact && self.bar.split_pend.has_compute() {
                stats::inc(&stats::c::split_cs_substage);
            }
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
                let sp_items = std::mem::take(&mut self.bar.soft_pend_items[i]);
                let sp_poison = std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
                if !sa.is_empty() && !sb.is_empty() {
                    merge_req_for_after(&mut self.bar.vtx_carry_req[i], sa, &sr);
                    if self.opts.opt_cs_to_ps_overlap_v2 || self.gfx_to_ps_overlap_active() {
                        self.bar.vtx_carry_poisoned[i] |= sp_poison;
                        if !self.bar.vtx_carry_poisoned[i] {
                            Self::extend_soft_items(&mut self.bar.vtx_carry_items[i], sp_items);
                        } else {
                            self.bar.vtx_carry_items[i].clear();
                        }
                    } else {
                        self.bar.vtx_carry_poisoned[i] = true;
                        self.bar.vtx_carry_items[i].clear();
                    }
                    self.bar.vtx_carry[i] = merge_vtx(self.bar.vtx_carry[i], Some((sa, sb)));
                }
            }
            if !(self.bar.split_pend.vtx_after | self.bar.split_pend.frag_after).is_empty() {
                stats::inc(&stats::c::cs_keep_gfx);
            }
            if qa != orig_pend_after || eff_qb != orig_pend_cs_before {
                stats::inc(&stats::c::render_split_sync);
            }
            self.sync_legacy_from_split();
            let in_list_clean = qa != MTLStages::All
                && STAGE_BITS
                    .iter()
                    .enumerate()
                    .all(|(pi, &b)| !qa.contains(b) || self.bar.stage_gen[pi] == 0);
            let already_synced = self.bar.enc_sync_after.contains(qa)
                && self.bar.enc_sync_before.contains(eff_qb)
                && (!self.bar.enc_blit_only_sync || in_list_clean);
            let mid_on = self.mid_qb_stagesync_active();
            let try_v5_partition = |this: &mut Self, full_eff_qa: MTLStages| -> bool {
                if !this.opts.opt_cs_mid_qb_res_defer_v5
                    || !this.ib_tight_active()
                    || (0..3)
                        .any(|ci| cs_mask.contains(CS_SUB_BITS[ci].1) && this.bar.pend_cs_poisoned[ci])
                    || this.bar.pend_cs_items.is_empty()
                {
                    return false;
                }
                let items = std::mem::take(&mut this.bar.pend_cs_items);
                let mut kept = Vec::with_capacity(items.len());
                let mut hit_qa = MTLStages::empty();
                let mut hit_req = [0u32; 7];
                let mut any_unsat = false;
                let mut kept_cs_any = false;
                for mut item in items {
                    let uda = if cs_mask.contains(MTLStages::Dispatch) && !item.disp_after.is_empty()
                    {
                        this.unsatisfied_enc_after_for_req(
                            item.disp_after,
                            MTLStages::Dispatch,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    let uba = if cs_mask.contains(MTLStages::Blit) && !item.blit_after.is_empty() {
                        this.unsatisfied_enc_after_for_req(
                            item.blit_after,
                            MTLStages::Blit,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    let uaa = if cs_mask.contains(MTLStages::AccelerationStructure)
                        && !item.as_after.is_empty()
                    {
                        this.unsatisfied_enc_after_for_req(
                            item.as_after,
                            MTLStages::AccelerationStructure,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    let u_any = uda | uba | uaa;
                    if u_any.is_empty() {
                        if cs_mask.contains(MTLStages::Dispatch) {
                            item.disp_after = MTLStages::empty();
                        }
                        if cs_mask.contains(MTLStages::Blit) {
                            item.blit_after = MTLStages::empty();
                        }
                        if cs_mask.contains(MTLStages::AccelerationStructure) {
                            item.as_after = MTLStages::empty();
                        }
                        if !(item.disp_after | item.blit_after | item.as_after).is_empty() {
                            kept.push(item);
                        }
                        continue;
                    }
                    any_unsat = true;
                    if this.cs_op_may_touch_res(item.r, op_stage, op_touch, exact) {
                        hit_qa |= u_any;
                        merge_req_for_after(&mut hit_req, u_any, &item.req);
                        if cs_mask.contains(MTLStages::Dispatch) {
                            item.disp_after = MTLStages::empty();
                        }
                        if cs_mask.contains(MTLStages::Blit) {
                            item.blit_after = MTLStages::empty();
                        }
                        if cs_mask.contains(MTLStages::AccelerationStructure) {
                            item.as_after = MTLStages::empty();
                        }
                        if !(item.disp_after | item.blit_after | item.as_after).is_empty() {
                            kept.push(item);
                        }
                    } else {
                        kept_cs_any = true;
                        if cs_mask.contains(MTLStages::Dispatch) {
                            item.disp_after = uda;
                            if !uda.is_empty() {
                                this.bar.split_pend.disp_after |= uda;
                                merge_req_for_after(
                                    &mut this.bar.split_pend.disp_req,
                                    uda,
                                    &item.req,
                                );
                                this.record_cs_soft_disp(item.r, uda, item.req);
                            }
                        }
                        if cs_mask.contains(MTLStages::Blit) {
                            item.blit_after = uba;
                            if !uba.is_empty() {
                                this.bar.split_pend.blit_after |= uba;
                                merge_req_for_after(
                                    &mut this.bar.split_pend.blit_req,
                                    uba,
                                    &item.req,
                                );
                            }
                        }
                        if cs_mask.contains(MTLStages::AccelerationStructure) {
                            item.as_after = uaa;
                            if !uaa.is_empty() {
                                this.bar.split_pend.as_after |= uaa;
                                merge_req_for_after(
                                    &mut this.bar.split_pend.as_req,
                                    uaa,
                                    &item.req,
                                );
                            }
                        }
                        if !(item.disp_after | item.blit_after | item.as_after).is_empty() {
                            kept.push(item);
                        }
                    }
                }
                this.bar.pend_cs_items = kept;
                if !any_unsat {
                    return false;
                }
                if hit_qa.is_empty() {
                    this.sync_legacy_from_split();
                    this.bar.needs_barrier = false;
                    stats::inc(&stats::c::cs_mid_qb_defer);
                    stats::inc(&stats::c::cs_mid_qb_v5);
                    return true;
                }
                let emit_qa = if split_cs_exact {
                    let f = this.fold_dominated_after(hit_qa, &hit_req);
                    if f != hit_qa {
                        stats::inc(&stats::c::split_cs_dom_fold);
                    }
                    f
                } else {
                    hit_qa
                };
                if !emit_qa.is_empty() {
                    if emit_qa != qa {
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        emit_qa,
                        eff_qb,
                        MTL4VisibilityOptions::Device,
                    );
                    let rec_qa = if this.opts.opt_rp_keep_write_exact {
                        emit_qa
                    } else {
                        hit_qa
                    };
                    this.record_enc_queue_sync(rec_qa, eff_qb);
                    stats::inc(&stats::c::cs_mid_qb);
                    if kept_cs_any || emit_qa != full_eff_qa {
                        stats::inc(&stats::c::cs_mid_qb_v5);
                    }
                    this.note_enc_sync_pair(rec_qa, eff_qb);
                }
                if !kept_cs_any {
                    this.clear_cs_mid_v3_for(cs_mask);
                }
                this.sync_legacy_from_split();
                this.bar.needs_barrier = false;
                true
            };
            let try_v3_defer = |this: &mut Self| -> bool {
                if !(this.opts.opt_cs_mid_qb_defer_v3
                    || this.opts.opt_cs_mid_qb_unpoison_v4
                    || this.opts.opt_desc_table_cs_fast
                    || this.opts.opt_cs_fast_bind_exact)
                    || !this.ib_tight_active()
                    || (0..3).any(|ci| {
                        cs_mask.contains(CS_SUB_BITS[ci].1) && this.bar.cs_mid_v3_poisoned[ci]
                    })
                    || this.bar.cs_mid_v3_items.is_empty()
                {
                    return false;
                }
                let mut any_unsat = false;
                for item in &this.bar.cs_mid_v3_items {
                    let uda = if cs_mask.contains(MTLStages::Dispatch) && !item.disp_after.is_empty()
                    {
                        this.unsatisfied_enc_after_for_req(
                            item.disp_after,
                            MTLStages::Dispatch,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    let uba = if cs_mask.contains(MTLStages::Blit) && !item.blit_after.is_empty() {
                        this.unsatisfied_enc_after_for_req(
                            item.blit_after,
                            MTLStages::Blit,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    let uaa = if cs_mask.contains(MTLStages::AccelerationStructure)
                        && !item.as_after.is_empty()
                    {
                        this.unsatisfied_enc_after_for_req(
                            item.as_after,
                            MTLStages::AccelerationStructure,
                            &item.req,
                            mid_on,
                        )
                    } else {
                        MTLStages::empty()
                    };
                    if !(uda | uba | uaa).is_empty() {
                        any_unsat = true;
                        if this.cs_op_may_touch_res(item.r, op_stage, op_touch, exact) {
                            return false;
                        }
                    }
                }
                if !any_unsat {
                    return false;
                }
                let items = std::mem::take(&mut this.bar.cs_mid_v3_items);
                let mut kept = Vec::with_capacity(items.len());
                for mut item in items {
                    if cs_mask.contains(MTLStages::Dispatch) && !item.disp_after.is_empty() {
                        let uda = this.unsatisfied_enc_after_for_req(
                            item.disp_after,
                            MTLStages::Dispatch,
                            &item.req,
                            mid_on,
                        );
                        item.disp_after = uda;
                        if !uda.is_empty() {
                            this.bar.split_pend.disp_after |= uda;
                            merge_req_for_after(&mut this.bar.split_pend.disp_req, uda, &item.req);
                            this.record_cs_soft_disp(item.r, uda, item.req);
                        }
                    }
                    if cs_mask.contains(MTLStages::Blit) && !item.blit_after.is_empty() {
                        let uba = this.unsatisfied_enc_after_for_req(
                            item.blit_after,
                            MTLStages::Blit,
                            &item.req,
                            mid_on,
                        );
                        item.blit_after = uba;
                        if !uba.is_empty() {
                            this.bar.split_pend.blit_after |= uba;
                            merge_req_for_after(&mut this.bar.split_pend.blit_req, uba, &item.req);
                        }
                    }
                    if cs_mask.contains(MTLStages::AccelerationStructure)
                        && !item.as_after.is_empty()
                    {
                        let uaa = this.unsatisfied_enc_after_for_req(
                            item.as_after,
                            MTLStages::AccelerationStructure,
                            &item.req,
                            mid_on,
                        );
                        item.as_after = uaa;
                        if !uaa.is_empty() {
                            this.bar.split_pend.as_after |= uaa;
                            merge_req_for_after(&mut this.bar.split_pend.as_req, uaa, &item.req);
                        }
                    }
                    if !(item.disp_after | item.blit_after | item.as_after).is_empty() {
                        kept.push(item);
                    }
                }
                this.bar.cs_mid_v3_items = kept;
                this.sync_legacy_from_split();
                this.bar.needs_barrier = false;
                stats::inc(&stats::c::cs_mid_qb_defer);
                if this.opts.opt_cs_mid_qb_defer_v3 {
                    stats::inc(&stats::c::cs_mid_qb_v3);
                }
                if this.opts.opt_cs_mid_qb_unpoison_v4 {
                    stats::inc(&stats::c::cs_mid_qb_v4);
                }
                if this.opts.opt_cs_fast_bind_exact {
                    stats::inc(&stats::c::cs_fast_bind_exact);
                } else if this.opts.opt_desc_table_cs_fast {
                    stats::inc(&stats::c::desc_table_cs_fast);
                }
                true
            };
            if mid_on {
                let has_all_pre_list = STAGE_BITS
                    .iter()
                    .enumerate()
                    .filter(|&(_, &b)| eff_qb.contains(b))
                    .all(|(ci, _)| self.bar.enc_cur_sync[ci].pre_list_all);
                let base_qa = if !has_all_pre_list && already_synced {
                    MTLStages::empty()
                } else {
                    self.unsatisfied_mid_qb_after(qa, eff_qb)
                };
                let eff_qa = if split_cs_exact {
                    let u = if !has_all_pre_list && already_synced {
                        MTLStages::empty()
                    } else {
                        self.unsatisfied_enc_after_for_req(qa, eff_qb, &cs_req, true)
                    };
                    let f = self.fold_dominated_after(u, &cs_req);
                    if f != base_qa {
                        stats::inc(&stats::c::split_cs_dom_fold);
                    }
                    f
                } else {
                    base_qa
                };
                if eff_qa.is_empty() {
                    self.clear_cs_mid_v3_for(cs_mask);
                    self.clear_pend_cs_for(cs_mask);
                    if !already_synced {
                        stats::inc(&stats::c::stagesync_skip);
                        if !self.opts.opt_rp_keep_write_exact {
                            self.note_enc_sync_pair(qa, eff_qb);
                        }
                    }
                } else {
                    if try_v5_partition(self, eff_qa) {
                        return;
                    }
                    if try_v3_defer(self) {
                        return;
                    }
                    if eff_qa != qa {
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        eff_qa,
                        eff_qb,
                        MTL4VisibilityOptions::Device,
                    );
                    self.clear_cs_mid_v3_for(cs_mask);
                    self.clear_pend_cs_for(cs_mask);
                    let rec_qa = if self.opts.opt_rp_keep_write_exact {
                        eff_qa
                    } else {
                        qa
                    };
                    self.record_enc_queue_sync(rec_qa, eff_qb);
                    stats::inc(&stats::c::cs_mid_qb);
                    self.note_enc_sync_pair(rec_qa, eff_qb);
                }
            } else if already_synced {
                self.clear_cs_mid_v3_for(cs_mask);
                self.clear_pend_cs_for(cs_mask);
            } else {
                let eff_qa = if split_cs_exact {
                    self.fold_dominated_after(qa, &cs_req)
                } else {
                    qa
                };
                if try_v5_partition(self, eff_qa) {
                    return;
                }
                if try_v3_defer(self) {
                    return;
                }
                if eff_qa != qa {
                    stats::inc(&stats::c::split_cs_dom_fold);
                    stats::inc(&stats::c::stagesync_narrow);
                }
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    eff_qa,
                    eff_qb,
                    MTL4VisibilityOptions::Device,
                );
                self.clear_cs_mid_v3_for(cs_mask);
                self.clear_pend_cs_for(cs_mask);
                let rec_qa = if self.opts.opt_rp_keep_write_exact {
                    eff_qa
                } else {
                    qa
                };
                self.record_enc_queue_sync(rec_qa, eff_qb);
                stats::inc(&stats::c::cs_mid_qb);
                self.note_enc_sync_pair(rec_qa, eff_qb);
            }
            if qa.contains(MTLStages::Fragment) && eff_qb.contains(MTLStages::Fragment) {
                self.bar.written.clear();
            }
            self.bar.needs_barrier = false;
            return;
        }
        if full
            || (!self.bar.pend_after.is_empty() && self.bar.pend_before.intersects(COMPUTE_STAGES))
        {
            let qa = if full {
                MTLStages::All
            } else {
                self.bar.pend_after
            };
            let qb = if full {
                MTLStages::All
            } else {
                self.bar.pend_before
            };
            if !full && self.mid_qb_stagesync_active() {
                let eff_qb = qb & COMPUTE_STAGES;
                let in_list_clean = qa != MTLStages::All
                    && STAGE_BITS
                        .iter()
                        .enumerate()
                        .all(|(pi, &b)| !qa.contains(b) || self.bar.stage_gen[pi] == 0);
                let already_synced = self.bar.enc_sync_after.contains(qa)
                    && self.bar.enc_sync_before.contains(eff_qb)
                    && (!self.bar.enc_blit_only_sync || in_list_clean);
                let has_all_pre_list = STAGE_BITS
                    .iter()
                    .enumerate()
                    .filter(|&(_, &b)| eff_qb.contains(b))
                    .all(|(ci, _)| self.bar.enc_cur_sync[ci].pre_list_all);
                let eff_qa = if !has_all_pre_list && already_synced {
                    MTLStages::empty()
                } else {
                    self.unsatisfied_mid_qb_after(qa, eff_qb)
                };
                if eff_qa.is_empty() {
                    if !already_synced {
                        stats::inc(&stats::c::stagesync_skip);
                        self.note_enc_sync_pair(qa, eff_qb);
                    }
                } else {
                    if eff_qa != qa {
                        stats::inc(&stats::c::stagesync_narrow);
                    }
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        eff_qa,
                        eff_qb,
                        MTL4VisibilityOptions::Device,
                    );
                    let rec_qa = if self.opts.opt_rp_keep_write_exact {
                        eff_qa
                    } else {
                        qa
                    };
                    self.record_enc_queue_sync(rec_qa, eff_qb);
                    stats::inc(&stats::c::cs_mid_qb);
                    self.note_enc_sync_pair(rec_qa, eff_qb);
                    if eff_qa.contains(MTLStages::Fragment) {
                        self.bar.written.clear();
                    }
                }
                if !self.gfx.pass_uav
                    && self
                        .bar
                        .enc_sync_after
                        .contains(MTLStages::Fragment | PRE_RASTER)
                {
                    self.bar.uav_gfx_dirty = false;
                }
            } else if !(self.bar.enc_sync_after.contains(qa)
                && self.bar.enc_sync_before.contains(qb & COMPUTE_STAGES))
                || (full && self.bar.enc_blit_only_sync)
            {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    qa,
                    qb,
                    MTL4VisibilityOptions::Device,
                );
                self.record_enc_queue_sync(qa, qb);
                stats::inc(&stats::c::cs_mid_qb);
                self.note_enc_sync_pair(qa, qb);
                if full {
                    self.bar.enc_blit_only_sync = false;
                    self.bar.list_cs_unwaited_frag = false;
                    self.bar.list_unwaited_cs = false;
                }
                if qa.contains(MTLStages::Fragment) {
                    self.bar.written.clear();
                }
                if !self.gfx.pass_uav
                    && self
                        .bar
                        .enc_sync_after
                        .contains(MTLStages::Fragment | PRE_RASTER)
                {
                    self.bar.uav_gfx_dirty = false;
                }
            }
            self.bar.needs_barrier = false;
            let rest = if full {
                MTLStages::empty()
            } else {
                self.bar.pend_before - COMPUTE_STAGES
            };
            if rest.is_empty() {
                self.carry_soft();
                self.clear_all_pend();
            } else {
                self.bar.pend_before = rest;
                stats::inc(&stats::c::cs_keep_gfx);
            }
        } else if !self.bar.pend_after.is_empty() {
            stats::inc(&stats::c::cs_defer_gfx);
        }
    }

    /// `ResourceBarrier` (unparsed): pending clears execute first, then a
    /// full barrier precedes the next work.
    pub fn barrier(&mut self) {
        self.barrier_list(&[Barrier::Full]);
    }

    /// `ResourceBarrier` with parsed items: pending clears of the involved
    /// resources execute first; the next work waits for the stages of the
    /// "before" states and blocks the stages of the "after" states.
    pub fn barrier_list(&mut self, items: &[Barrier]) {
        self.barrier_list_with(items, barrier_r2r_elide(), barrier_uav0_keep());
    }

    fn sanitize_f16_barriers(&mut self, items: &[Barrier]) {
        let mut targets: Vec<(usize, Retained<MtlTexture>, u32, u32, u64)> = Vec::new();
        for item in items {
            // Only sanitize when transitioning from a write state (RT/UAV) to a read/copy
            // state (SRV/CopySource), or on explicit UAV barriers — skip RT<->UAV ping-pong
            // transitions where a subsequent RT/UAV->SRV transition will sanitize once.
            let r = match *item {
                Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1)
                    if r != 0 && (s0 & (0x4 | 0x8)) != 0 && (s1 & (0x40 | 0x80 | 0x800)) != 0 =>
                {
                    r
                }
                Barrier::Uav(r) if r != 0 => r,
                _ => 0,
            };
            if r == 0 || targets.iter().any(|t| t.0 == r) {
                continue;
            }
            let res = unsafe { &*(r as *const Resource) };
            if let Some((tex, w, h, maxv_va)) = crate::metal::gpunan::sanitize_f16_target(res) {
                targets.push((r, tex, w, h, maxv_va));
            }
        }
        if targets.is_empty() {
            return;
        }
        let Ok((pso, tg)) =
            crate::metal::clear::kernel_for_dispatch("port_clamp_f16_inf_2d", self.opts.opt_fast_bind_pso)
        else {
            return;
        };
        if self.ensure_util_table().is_err() {
            return;
        }
        let touch: Vec<usize> = targets.iter().map(|t| t.0).collect();
        let masked = !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        if masked {
            const UTIL_AFTER: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            self.add_pend(UTIL_AFTER, MTLStages::Dispatch);
            if ib_tight {
                for &r in &touch {
                    self.ib_note_touch_before_tight(r, MTLStages::Dispatch);
                }
            } else {
                for &r in &touch {
                    self.ib_note(r);
                }
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        let e = if ib_tight && masked {
            self.compute_op_tight(MTLStages::Dispatch, Some(&touch), Some(&touch), Some(&touch))
        } else {
            self.compute_touch(if masked { Some(&touch) } else { None })
        };
        let Some(e) = e else {
            return;
        };
        let t = self.util_table.as_ref().unwrap();
        e.setComputePipelineState(&pso);
        for (_, tex, w, h, maxv_va) in &targets {
            unsafe {
                t.setAddress_atIndex(*maxv_va, 0);
                t.setTexture_atIndex(tex.gpuResourceID(), 0);
            }
            e.setArgumentTable(Some(t));
            let tx = (*w as usize).div_ceil(2);
            let ty = (*h as usize).div_ceil(2);
            e.dispatchThreadgroups_threadsPerThreadgroup(
                MTLSize {
                    width: tx.div_ceil(tg[0] as usize),
                    height: ty.div_ceil(tg[1] as usize),
                    depth: 1,
                },
                MTLSize {
                    width: tg[0] as usize,
                    height: tg[1] as usize,
                    depth: 1,
                },
            );
        }
        self.note_stage_ran(MTLStages::Dispatch);
        e.setArgumentTable(Some(&self.table));
        self.compute_pso_bound = false;
        if masked {
            const UTIL_BEFORE: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            if ib_tight && self.opts.opt_cs_mid_qb_res_defer_v5 {
                if self.opts.opt_cs_to_ps_overlap_v2 {
                    self.add_pend_with_soft_res(MTLStages::Dispatch, MTLStages::Fragment, &touch);
                } else {
                    self.add_pend(MTLStages::Dispatch, PRE_RASTER | MTLStages::Fragment);
                }
                for &r in &touch {
                    self.ib_add_obl_tight(r, MTLStages::Dispatch, COMPUTE_STAGES);
                    self.bar.cs_end_res.push((
                        r,
                        MTLStages::Dispatch,
                        MTLStages::Dispatch,
                        MTLStages::Dispatch,
                    ));
                }
            } else {
                self.add_pend(MTLStages::Dispatch, UTIL_BEFORE);
                if ib_tight {
                    for &r in &touch {
                        self.ib_add_obl_tight(r, MTLStages::Dispatch, COMPUTE_STAGES);
                    }
                } else {
                    for &r in &touch {
                        self.ib_note(r);
                    }
                }
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
    }

    pub(super) fn barrier_list_with(&mut self, items: &[Barrier], elide_r2r: bool, uav0_keep: bool) {
        use MTLStages as S;
        self.flush_icb_prepass_if_pending();
        if crate::metal::gpunan::sanitize_f16_inf_active()
            && self.open
            && self.kind != ListType::Copy
        {
            self.sanitize_f16_barriers(items);
        }
        if crate::metal::gpunan::is_capturing() && self.open && self.kind != ListType::Copy {
            let mut scanned_any = false;
            for item in items {
                match *item {
                    Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1)
                        if r != 0 && (s0 & (0x4 | 0x8)) != 0 =>
                    {
                        let res = unsafe { &*(r as *const Resource) };
                        if crate::metal::gpunan::can_scan_resource(res) {
                            self.end_encoder();
                            let tag = if s0 & 0x4 != 0 { "bar_rt" } else { "bar_uav" };
                            if crate::metal::gpunan::encode_scan_resource(
                                &self.cb,
                                res,
                                tag,
                                self.diag_last_kind,
                                self.diag_last_key,
                                self.diag_last_vs_key,
                                s0,
                                s1,
                            ) {
                                scanned_any = true;
                            }
                        }
                    }
                    Barrier::Uav(r) if r != 0 => {
                        let res = unsafe { &*(r as *const Resource) };
                        if crate::metal::gpunan::can_scan_resource(res) {
                            self.end_encoder();
                            if crate::metal::gpunan::encode_scan_resource(
                                &self.cb,
                                res,
                                "bar_uav",
                                self.diag_last_kind,
                                self.diag_last_key,
                                self.diag_last_vs_key,
                                0x8,
                                0x8,
                            ) {
                                scanned_any = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
            if scanned_any {
                self.need_full(FullWhy::ClearUav);
            }
        }
        stats::inc(&stats::c::app_barrier);
        let masked_mode = !legacy_barriers() && barrier_masks();
        let elide_r2r = elide_r2r && masked_mode;
        let uav0_keep = uav0_keep && masked_mode;
        let smart_uav0 = self.opts.opt_clear_keep_smart && masked_mode && !uav0_keep;
        let dsv_uav0 = self.opts.opt_clear_keep_dsv && masked_mode && !uav0_keep;
        let non_uav_uav0 = self.opts.opt_clear_pass_fold_v2 && masked_mode && !uav0_keep;
        let ib_tight = self.opts.opt_ib_tight && masked_mode;
        let copy_light = self.is_copy_queue_light();
        let shader =
            S::Vertex | S::Object | S::Mesh | S::Fragment | S::Dispatch | S::AccelerationStructure;
        let track_read_only =
            matches!(self.enc, Encoder::Render(_)) && self.opts.opt_rp_keep_write;
        let (mut after, mut before, mut all_res) = (S::empty(), S::empty(), false);
        let mut soft = [S::empty(); 2];
        let mut local_split = SplitMasks::default();
        let (mut read_after, mut read_before) = (S::empty(), S::empty());
        let mut read_soft = [S::empty(); 2];
        let mut read_split = SplitMasks::default();
        let add_soft_cls = |soft: &mut [S; 2], cls: u8, st: S| {
            if cls & 1 != 0 {
                soft[0] |= st;
            }
            if cls & 2 != 0 {
                soft[1] |= st;
            }
        };
        let mut uav_all = false;
        let mut had_uav_item = false;
        let mut only_uav_items = true;
        let mut res = std::mem::take(&mut self.bar.bar_scratch_res);
        res.clear();
        let vs_buf_defer = self.vs_buf_defer_exact_active();
        let defer_cs_mid_v2 = self.cs_mid_qb_exact_v2_active();
        let track_cs_mid_v3 = (self.opts.opt_cs_mid_qb_defer_v3
            || self.opts.opt_cs_mid_qb_unpoison_v4
            || self.opts.opt_desc_table_cs_fast
            || self.opts.opt_cs_fast_bind_exact)
            && self.render_split_sync_active()
            && self.ib_tight_active()
            && !copy_light
            && matches!(self.enc, Encoder::Compute(_));
        let defer_cs_mid =
            self.cs_mid_qb_touch_defer_active() || defer_cs_mid_v2 || track_cs_mid_v3;
        let collect_pend_cs = self.opts.opt_cs_mid_qb_res_defer_v5
            && masked_mode
            && !copy_light
            && self.render_split_sync_active()
            && self.ib_tight_active();
        let collect_cs_soft = collect_pend_cs
            || (self.ps_to_cs_active()
                && masked_mode
                && !copy_light
                && self.render_split_sync_active());
        let allow_all_cs_mid = self.opts.opt_cs_mid_qb_unpoison_v4
            || self.opts.opt_cs_mid_qb_res_defer_v5
            || self.opts.opt_desc_table_cs_fast
            || self.opts.opt_cs_fast_bind_exact
            || self.ps_to_cs_active();
        let track_blit_safe = self.opts.opt_empty_enc_elide
            || self.opts.opt_blit_list_narrow_v2
            || self.opts.opt_list_boundary_overlap
            || self.opts.opt_queue_start_stage_exact;
        let track_soft_items = (self.opts.opt_cs_to_ps_overlap
            || self.opts.opt_cs_to_ps_overlap_v2
            || self.gfx_to_ps_overlap_active()
            || self.list_rsplit_v2_active())
            && masked_mode
            && !copy_light;
        let track_frag_rt = self.clear_isolate_active() && masked_mode && !copy_light;
        let mut local_rt_frag = if track_frag_rt {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_rt_frag);
            v.clear();
            v
        } else {
            Vec::new()
        };
        let mut local_non_rt_frag = S::empty();
        let mut local_rt_poison = false;
        let mut local_soft_items = if track_soft_items {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_soft);
            v[0].clear();
            v[1].clear();
            v
        } else {
            [Vec::new(), Vec::new()]
        };
        let mut local_soft_poison = [false; 2];
        let mut read_soft_items = if track_read_only && track_soft_items {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_read_soft);
            v[0].clear();
            v[1].clear();
            v
        } else {
            [Vec::new(), Vec::new()]
        };
        let mut read_soft_poison = [false; 2];
        let allow_all_soft = self.opts.opt_cs_to_ps_overlap_v2
            || self.gfx_to_ps_overlap_active()
            || self.list_rsplit_v2_active();
        let record_soft_item = |items_out: &mut [Vec<(usize, S, S)>; 2],
                                poison_out: &mut [bool; 2],
                                r: usize,
                                ia: S,
                                isoft: [S; 2]| {
            if !track_soft_items || ia.is_empty() || (isoft[0] | isoft[1]).is_empty() {
                return;
            }
            const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
            let simul = r != 0 && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL != 0 };
            for i in 0..2 {
                let sb = isoft[i] & PRE_RASTER;
                if sb.is_empty() {
                    continue;
                }
                if r == 0 || (ia == S::All && !allow_all_soft) || simul {
                    poison_out[i] = true;
                } else {
                    items_out[i].push((r, ia, sb));
                }
            }
        };
        let need_cs_mid = defer_cs_mid || collect_cs_soft;
        let mut cs_mid_items = if need_cs_mid {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_cs_mid);
            v.clear();
            v
        } else {
            Vec::new()
        };
        let mut read_cs_mid_items = if track_read_only && collect_cs_soft {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_read_cs_mid);
            v.clear();
            v
        } else {
            Vec::new()
        };
        let mut uav0_disp_after = S::empty();
        let track_exact_clear = self.opts.opt_clear_overlap_exact
            && masked_mode
            && !copy_light
            && !self.bar.in_list_gfx_non_clear;
        let mut active_items = if track_exact_clear {
            let mut v = std::mem::take(&mut self.bar.bar_scratch_active);
            v.clear();
            v
        } else {
            Vec::new()
        };
        let mut batch_pt = std::mem::take(&mut self.bar.bar_scratch_pt);
        batch_pt.clear();
        if !self.bar.uav_passthrough.is_empty() {
            batch_pt.extend_from_slice(&self.bar.uav_passthrough);
        }
        let mut split_begin_res = std::mem::take(&mut self.bar.bar_scratch_split_begin_res);
        split_begin_res.clear();
        let mut split_begin_all = false;
        let mut split_begin_keys = std::mem::take(&mut self.bar.bar_scratch_split_begin_keys);
        split_begin_keys.clear();
        let mut local_narrow_trail = std::mem::take(&mut self.bar.bar_scratch_narrow_trail);
        local_narrow_trail.clear();
        let mut all_matched_split_end = true;
        let mut any_matched_split_end = false;
        let mut matched_split_req = [0u32; 7];
        let track_list_start_blit = !self.bar.has_gpu_work
            && self.bar.needs_barrier
            && self.bar.full_why == FullWhy::ListStart
            && masked_mode
            && !copy_light;
        const SRV_UAV: u32 = 0x8 | 0x40;
        const VS_BUF_SOFT: u32 = 0x1 | 0x2 | 0x200;
        const BUF_ONLY_BITS: u32 = 0x1 | 0x2 | 0x100 | 0x200 | 0x40_0000;
        const TEX_ONLY_BITS: u32 = 0x4 | 0x10 | 0x20 | 0x1000 | 0x2000;
        let filter_bits = self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2;
        for b in items {
            match *b {
                Barrier::SplitBegin(r, sub, s0, s1) => {
                    if ib_tight && r != 0 {
                        unsafe { (*(r as *const Resource)).register_ptr() };
                    }
                    if !masked_mode {
                        only_uav_items = false;
                        all_matched_split_end = false;
                        after = S::All;
                        before = S::All;
                        all_res = true;
                        local_split.add(S::All, S::All, [S::empty(); 2]);
                        local_soft_poison = [true; 2];
                        local_rt_poison = true;
                        continue;
                    }
                    if s0 == s1 {
                        stats::inc(&stats::c::rb_noop);
                        if elide_r2r {
                            continue;
                        }
                    } else if is_read_to_read_transition(s0, s1) {
                        stats::inc(&stats::c::rb_r2r);
                        if elide_r2r
                            && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                            && (!self.ps_to_cs_active()
                                || self.cs_disp_passthrough_for(r).is_empty())
                        {
                            stats::inc(&stats::c::rb_r2r_elided);
                            continue;
                        }
                    }
                    if r == 0 {
                        split_begin_all = true;
                    } else if !split_begin_res.contains(&r) {
                        split_begin_res.push(r);
                    }
                    if matches!(self.enc, Encoder::Render(_)) {
                        let attached = r == 0
                            || self.bar.pass_targets.contains(&r)
                            || self.active_rtvs().iter().any(|v| v.resource == r)
                            || self.active_dsv().is_some_and(|v| v.resource == r);
                        if attached {
                            stats::inc(&stats::c::app_barrier_split);
                            self.end_encoder();
                        }
                    }
                    let begin_req = self.cur_producer_gen();
                    self.bar.split_barriers.push((r, sub, s0, s1, begin_req));
                    split_begin_keys.push((r, sub, s0, s1));
                    continue;
                }
                Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1) => {
                    if let Barrier::SplitEnd(_, sub, _, _) = *b {
                        let mut match_idx: Option<usize> = None;
                        let mut ambiguous = false;
                        for (i, &(sr, ssub, ss0, ss1, _)) in
                            self.bar.split_barriers.iter().enumerate()
                        {
                            if sr == r && ssub == sub && ss0 == s0 && ss1 == s1 {
                                if match_idx.is_some() {
                                    ambiguous = true;
                                } else {
                                    match_idx = Some(i);
                                }
                            }
                        }
                        if let Some(idx) = match_idx {
                            let (_, _, _, _, breq) = self.bar.split_barriers.remove(idx);
                            if ambiguous {
                                all_matched_split_end = false;
                            } else {
                                any_matched_split_end = true;
                                merge_req_all(&mut matched_split_req, &breq);
                            }
                        } else {
                            all_matched_split_end = false;
                        }
                    } else {
                        all_matched_split_end = false;
                    }
                    let (is_buf, cls) = if r == 0 {
                        (false, READS_TEX | READS_BUF)
                    } else {
                        let res_ref = unsafe { &*(r as *const Resource) };
                        if ib_tight {
                            res_ref.register_ptr();
                        }
                        if res_ref.desc.Dimension == crate::types::dim::BUFFER {
                            (true, READS_BUF)
                        } else {
                            (false, READS_TEX)
                        }
                    };
                    let filter_state = |s: u32| -> u32 {
                        if r == 0 || s == 0 {
                            return s;
                        }
                        let f = if is_buf { s & !TEX_ONLY_BITS } else { s & !BUF_ONLY_BITS };
                        if f != 0 { f } else { s }
                    };
                    let (filt_s0, filt_s1) = if track_blit_safe || filter_bits {
                        (filter_state(s0), filter_state(s1))
                    } else {
                        (s0, s1)
                    };
                    if track_list_start_blit {
                        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                        if r == 0 {
                            if !self.bar.list_start_blit_tainted.contains(&0) {
                                self.bar.list_start_blit_tainted.push(0);
                            }
                        } else {
                            let res_ref = unsafe { &*(r as *const Resource) };
                            let st0 = if s0 == 0 {
                                S::All
                            } else {
                                state_stages_for_res(r, s0)
                            };
                            if s0 == 0
                                || st0 == S::All
                                || (res_ref.desc.Flags & SIMUL) != 0
                                || res_ref.may_alias_placed()
                            {
                                if !self.bar.list_start_blit_tainted.contains(&r) {
                                    self.bar.list_start_blit_tainted.push(r);
                                }
                                self.bar.list_start_blit_explicit.retain(|e| e.0 != r);
                            } else if !self.bar.list_start_blit_tainted.contains(&r) {
                                if let Some(ex) = self
                                    .bar
                                    .list_start_blit_explicit
                                    .iter_mut()
                                    .find(|e| e.0 == r)
                                {
                                    ex.1 |= st0;
                                } else {
                                    self.bar.list_start_blit_explicit.push((r, st0));
                                }
                            }
                        }
                    }
                    if track_blit_safe && r != 0 {
                        let eff_s0_b = filt_s0;
                        let eff_s1_b = filt_s1;
                        if s0 == 0
                            && self.opts.opt_queue_start_stage_exact
                            && (!self.bar.has_gpu_work || self.bar.list_cs_unwaited_frag)
                            && !self.bar.is_blit_tainted(r)
                            && unsafe { (*(r as *const Resource)).is_fresh_nonfrag_blit_safe() }
                        {
                            if eff_s1_b != 0 && (eff_s1_b & !(0x400 | 0x800)) == 0 {
                                self.bar.mark_blit_safe(r);
                            }
                        } else if s0 == 0 || state_stages(eff_s0_b).contains(S::Fragment) {
                            self.bar.mark_blit_tainted(r);
                            self.bar.remove_blit_safe(r);
                        } else if (!self.bar.has_gpu_work || self.bar.list_cs_unwaited_frag)
                            && eff_s1_b != 0
                            && (eff_s1_b & !(0x400 | 0x800)) == 0
                            && !self.bar.is_blit_tainted(r)
                        {
                            self.bar.mark_blit_safe(r);
                        }
                    }
                    if s0 == s1 {
                        stats::inc(&stats::c::rb_noop);
                        if elide_r2r {
                            continue;
                        }
                    } else if is_read_to_read_transition(s0, s1) {
                        stats::inc(&stats::c::rb_r2r);
                        if elide_r2r
                            && !self.clears.pending_clears.iter().any(|p| p.view.resource == r)
                            && (!self.ps_to_cs_active()
                                || self.cs_disp_passthrough_for(r).is_empty())
                        {
                            stats::inc(&stats::c::rb_r2r_elided);
                            continue;
                        }
                    }
                    only_uav_items = false;
                    if track_exact_clear {
                        active_items.push(*b);
                    }
                    let (eff_s0, eff_s1) = if filter_bits {
                        if self.opts.opt_split_res_soft_sync && (filt_s0 != s0 || filt_s1 != s1) {
                            stats::inc(&stats::c::res_soft_sync);
                        }
                        (filt_s0, filt_s1)
                    } else {
                        (s0, s1)
                    };
                    let st_after = if s0 == 0
                        && self.opts.opt_queue_start_stage_exact
                        && r != 0
                        && s1 != 0
                        && eff_s1 != 0
                        && (eff_s1 & !(0x400 | 0x800)) == 0
                        && !self.bar.in_list_gfx_non_clear
                        && self.bar.cs_disp_gen == 0
                        && self.bar.cs_as_gen == 0
                        && !self.bar.is_blit_tainted(r)
                        && unsafe { (*(r as *const Resource)).is_fresh_nonfrag_blit_safe() }
                        && (!self.bar.has_gpu_work
                            || self.bar.enc_cur_sync[STAGE_BLIT]
                                .pre_list_stages
                                .contains(COMPUTE_STAGES | PRE_RASTER))
                    {
                        stats::inc(&stats::c::queue_start_stage_exact);
                        S::empty()
                    } else {
                        self.source_stages_for_transition_with(r, eff_s0, s0, true, &batch_pt)
                    };
                    if (eff_s0 & 0x8) != 0 && (eff_s1 & 0x8) == 0 {
                        Self::pt_remove(&mut batch_pt, r);
                    }
                    if (eff_s1 & 0x8) != 0 {
                        Self::pt_insert(&mut batch_pt, r, st_after);
                    }
                    let is_read_dst =
                        track_read_only && s1 != 0 && (eff_s1 & D3D12_WRITE_STATES) == 0;
                    // Vertex/object/mesh reads through SRV/UAV states (and buffer
                    // VB_CB/IB/INDIRECT_ARG states under `AW2_VS_BUF_DEFER_EXACT=1`)
                    // can be ordered at the first draw that reads in those stages.
                    let soft_mask = if vs_buf_defer && is_buf {
                        SRV_UAV | VS_BUF_SOFT
                    } else {
                        SRV_UAV
                    };
                    let hard = eff_s1 & !soft_mask;
                    let mut item_before = S::empty();
                    let mut item_soft = [S::empty(); 2];
                    if eff_s1 & soft_mask != 0 {
                        let st = state_stages(eff_s1 & soft_mask);
                        add_soft_cls(&mut item_soft, cls, st & PRE_RASTER);
                        item_before |= st - PRE_RASTER;
                    }
                    if hard != 0 || s1 == 0 {
                        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                        if s1 == 0
                            && is_buf
                            && self.opts.opt_ps_srv_class_exact
                            && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                        {
                            add_soft_cls(&mut item_soft, cls, PRE_RASTER);
                            item_before |= S::All - PRE_RASTER;
                        } else if s1 == 0
                            && !is_buf
                            && r != 0
                            && (self.opts.opt_icb_enc_barrier_exact
                                || self.gfx_to_ps_overlap_active())
                            && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL == 0 }
                        {
                            add_soft_cls(&mut item_soft, cls, PRE_RASTER);
                            item_before |= S::All - PRE_RASTER;
                            if self.opts.opt_icb_enc_barrier_exact {
                                stats::inc(&stats::c::icb_enc_barrier_exact);
                            }
                        } else {
                            let st_hard = if s1 == 0 { S::All } else { state_stages(hard) };
                            item_before |= st_hard;
                        }
                    }
                    if r != 0 && !is_buf && item_before != S::All {
                        item_before -= S::AccelerationStructure;
                    }
                    if item_before == S::All {
                        item_soft = [S::empty(); 2];
                    } else if vs_buf_defer
                        && is_buf
                        && s1 != 0
                        && (eff_s1 & VS_BUF_SOFT) != 0
                        && !item_before.intersects(PRE_RASTER)
                        && !st_after.is_empty()
                    {
                        stats::inc(&stats::c::vs_buf_defer);
                    }
                    if track_frag_rt && !st_after.is_empty() {
                        const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                        if self.opts.opt_clear_zero_qb_coalesce {
                            let is_clean_res = r != 0
                                && unsafe {
                                    let res = &*(r as *const Resource);
                                    res.placed_in.is_none() && (res.desc.Flags & SIMUL == 0)
                                };
                            if item_before == S::All || item_before.contains(S::Fragment) {
                                if !is_clean_res {
                                    local_rt_poison = true;
                                } else if s1 == 0 || (eff_s1 & (0x4 | 0x10 | 0x20 | 0x1000)) != 0 {
                                    local_rt_frag.push((r, st_after));
                                } else {
                                    local_non_rt_frag |= st_after;
                                }
                            }
                        } else if r != 0
                            && s1 != 0
                            && eff_s1 != 0
                            && (eff_s1 & !(0x4 | 0x10)) == 0
                            && item_before == S::Fragment
                            && item_soft == [S::empty(); 2]
                        {
                            local_rt_frag.push((r, st_after));
                        } else if item_before == S::All || item_before.contains(S::Fragment) {
                            if r == 0 {
                                local_rt_poison = true;
                            } else {
                                local_non_rt_frag |= st_after;
                            }
                        }
                    }
                    after |= st_after;
                    before |= item_before;
                    soft[0] |= item_soft[0];
                    soft[1] |= item_soft[1];
                    local_split.add(st_after, item_before, item_soft);
                    record_soft_item(
                        &mut local_soft_items,
                        &mut local_soft_poison,
                        r,
                        st_after,
                        item_soft,
                    );
                    if need_cs_mid
                        && r != 0
                        && (item_before != S::All || allow_all_cs_mid)
                    {
                        let da = if item_before.contains(S::Dispatch) {
                            st_after
                        } else {
                            S::empty()
                        };
                        let ba = if item_before.contains(S::Blit) {
                            st_after
                        } else {
                            S::empty()
                        };
                        let aa = if item_before.contains(S::AccelerationStructure) {
                            st_after
                        } else {
                            S::empty()
                        };
                        if !(da | ba | aa).is_empty() {
                            cs_mid_items.push((r, da, ba, aa));
                            if collect_cs_soft && is_read_dst {
                                read_cs_mid_items.push((r, da, ba, aa));
                            }
                        }
                    }
                    if is_read_dst {
                        read_after |= st_after;
                        read_before |= item_before;
                        read_soft[0] |= item_soft[0];
                        read_soft[1] |= item_soft[1];
                        read_split.add(st_after, item_before, item_soft);
                        record_soft_item(
                            &mut read_soft_items,
                            &mut read_soft_poison,
                            r,
                            st_after,
                            item_soft,
                        );
                    }
                    if masked_mode && !copy_light {
                        let s1_full = if s1 == 0 { S::All } else { state_stages_for_res(r, eff_s1) };
                        let narrow = sub_stages_preserving_all(st_after, s1_full);
                        if !narrow.is_empty() {
                            local_narrow_trail.push((r, narrow, s1_full));
                        }
                    }
                    if r == 0 {
                        all_res = true;
                    } else {
                        res.push(r);
                    }
                }
                Barrier::Uav(r) => {
                    all_matched_split_end = false;
                    if track_list_start_blit {
                        if !self.bar.list_start_blit_tainted.contains(&r) {
                            self.bar.list_start_blit_tainted.push(r);
                        }
                        if r != 0 {
                            self.bar.list_start_blit_explicit.retain(|e| e.0 != r);
                        }
                    }
                    had_uav_item = true;
                    let cls = if r == 0 {
                        READS_TEX | READS_BUF
                    } else {
                        let res_ref = unsafe { &*(r as *const Resource) };
                        if ib_tight {
                            res_ref.register_ptr();
                        }
                        if res_ref.desc.Dimension == crate::types::dim::BUFFER {
                            READS_BUF
                        } else {
                            READS_TEX
                        }
                    };
                    if track_exact_clear {
                        active_items.push(*b);
                    }
                    let uav_after = self.source_stages_for_uav_with(r, shader, true, &batch_pt);
                    let mut item_soft = [S::empty(); 2];
                    add_soft_cls(&mut item_soft, cls, PRE_RASTER);
                    let uav_hard = if r != 0 && cls == READS_TEX {
                        (shader - PRE_RASTER) - S::AccelerationStructure
                    } else {
                        shader - PRE_RASTER
                    };
                    if track_frag_rt
                        && !uav_after.is_empty()
                        && (uav_hard == S::All || uav_hard.contains(S::Fragment))
                    {
                        if self.opts.opt_clear_zero_qb_coalesce {
                            const SIMUL: u32 =
                                crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                            let is_clean_res = r != 0
                                && unsafe {
                                    let res = &*(r as *const Resource);
                                    res.placed_in.is_none() && (res.desc.Flags & SIMUL == 0)
                                };
                            if is_clean_res {
                                local_rt_frag.push((r, uav_after));
                            } else {
                                local_rt_poison = true;
                            }
                        } else if r == 0 {
                            local_rt_poison = true;
                        } else {
                            local_non_rt_frag |= uav_after;
                        }
                    }
                    after |= uav_after;
                    before |= uav_hard;
                    soft[0] |= item_soft[0];
                    soft[1] |= item_soft[1];
                    local_split.add(uav_after, uav_hard, item_soft);
                    record_soft_item(
                        &mut local_soft_items,
                        &mut local_soft_poison,
                        r,
                        uav_after,
                        item_soft,
                    );
                    if need_cs_mid && r != 0 {
                        let da = if uav_hard.contains(S::Dispatch) {
                            uav_after
                        } else {
                            S::empty()
                        };
                        let ba = if uav_hard.contains(S::Blit) {
                            uav_after
                        } else {
                            S::empty()
                        };
                        let aa = if uav_hard.contains(S::AccelerationStructure) {
                            uav_after
                        } else {
                            S::empty()
                        };
                        if !(da | ba | aa).is_empty() {
                            cs_mid_items.push((r, da, ba, aa));
                        }
                    }
                    if r == 0 {
                        uav_all = true;
                        if uav_hard.contains(S::Dispatch) {
                            uav0_disp_after |= uav_after;
                        }
                    } else {
                        res.push(r);
                    }
                }
                Barrier::Full => {
                    all_matched_split_end = false;
                    only_uav_items = false;
                    after = S::All;
                    before = S::All;
                    all_res = true;
                    local_split.add(S::All, S::All, [S::empty(); 2]);
                    local_soft_poison = [true; 2];
                    local_rt_poison = true;
                }
            }
        }
        if !masked_mode {
            after = S::All;
            before = S::All;
            all_res = true;
            local_split = SplitMasks::default();
            local_split.add(S::All, S::All, [S::empty(); 2]);
            local_soft_poison = [true; 2];
            local_rt_poison = true;
        }
        if before == S::All {
            soft = [S::empty(); 2];
        }
        let flush_all_why = if all_res {
            Some(&stats::c::cf_barrier_full)
        } else if uav_all {
            if uav0_keep {
                if !self.clears.pending_clears.is_empty() {
                    stats::inc(&stats::c::cf_uav0_kept);
                }
                None
            } else if smart_uav0 || dsv_uav0 || non_uav_uav0 {
                None
            } else {
                Some(&stats::c::cf_barrier_uav0)
            }
        } else {
            None
        };
        if after.is_empty() || (before | soft[0] | soft[1]).is_empty() {
            if split_begin_all || !split_begin_res.is_empty() {
                let pre_len = self.clears.pending_clears.len();
                if let Err(e) = self.flush_clears(&stats::c::cf_barrier_res, |p| {
                    split_begin_all || split_begin_res.contains(&p.view.resource)
                }) {
                    log::warn!(target: "d3d", "ResourceBarrier: deferred clear: {e}");
                }
                if self.clears.pending_clears.len() != pre_len {
                    let post_req = self.cur_producer_gen();
                    for entry in &mut self.bar.split_barriers {
                        if split_begin_keys.contains(&(entry.0, entry.1, entry.2, entry.3)) {
                            entry.4 = post_req;
                        }
                    }
                }
            }
            self.restore_bar_scratch(
                res,
                local_rt_frag,
                local_soft_items,
                read_soft_items,
                cs_mid_items,
                read_cs_mid_items,
                active_items,
                batch_pt,
                split_begin_res,
                split_begin_keys,
                local_narrow_trail,
            );
            return;
        }
        let mut kept_render = false;
        let mut had_write_or_uav = false;
        match self.enc {
            // Read-only transitions of resources the open draw pass neither
            // attaches nor may have written: only earlier encoders produced
            // them, so a mid-pass queue barrier before the next draw orders
            // it (`render_mid_barrier`). With `opt_rp_keep_write`, unattached
            // write transitions and `!pass_uav` UAV barriers also keep the pass
            // open while recording WAR stages for `end_encoder()`.
            Encoder::Render(_)
                if flush_all_why.is_none()
                    && !(after == S::All && before == S::All)
                    && self.render_barrier_keeps(items, elide_r2r) =>
            {
                kept_render = true;
                had_write_or_uav = self.opts.opt_rp_keep_write
                    && self.note_render_kept_writes(items, elide_r2r, shader);
                if had_write_or_uav {
                    stats::inc(&stats::c::rb_keep_write);
                } else {
                    stats::inc(&stats::c::rb_keep);
                }
            }
            Encoder::Render(_) => {
                stats::inc(&stats::c::app_barrier_split);
                self.end_encoder();
            }
            // An intra-encoder barrier only orders this encoder's own work:
            // earlier encoders' `after` stages must have been waited for when
            // this encoder began: a full barrier needs a new encoder; masked
            // ones become a mid-encoder queue barrier (see `compute`).
            Encoder::Compute(_)
                if !(self.bar.enc_sync_after.contains(after)
                    && self.bar.enc_sync_before.contains(COMPUTE_STAGES))
                    && !copy_light =>
            {
                if after == S::All && before == S::All {
                    stats::inc(&stats::c::cs_split);
                    self.end_encoder();
                }
            }
            _ => {}
        }
        if let Encoder::Compute(_) = self.enc {
            if ib_tight {
                self.note_compute_barriers_tight(
                    items,
                    elide_r2r,
                    !copy_light && after == S::All && before == S::All,
                );
            } else {
                if !copy_light && after == S::All && before == S::All {
                    self.ib_note(0);
                }
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
                            let before_cs = state_stages(s0).intersects(COMPUTE_STAGES)
                                || (self.ib.ib_cs_clear && s0 & 0x4 != 0);
                            if before_cs && state_stages(s1).intersects(COMPUTE_STAGES) {
                                self.ib_note(r);
                            }
                        }
                        Barrier::Uav(r) => self.ib_note(r),
                        Barrier::Full => self.ib_note(0),
                    }
                }
            }
        }
        let cs_pend_after = if self.render_split_sync_active() && self.split_compute_exact_active()
        {
            self.bar.split_pend.disp_after
        } else {
            self.bar.pend_after
        };
        let cs_disp_synced_all =
            self.open_compute_disp_uav_synced() && self.bar.uav_passthrough.is_empty();
        let cs_pend_extra = cs_pend_after | (self.bar.cs_end_after[0] - COMPUTE_STAGES);
        let cs_pend_unsat = if cs_disp_synced_all {
            self.unsatisfied_enc_after_for(
                cs_pend_extra,
                S::Dispatch,
                self.mid_qb_stagesync_active(),
            )
        } else {
            cs_pend_extra - self.bar.enc_sync_after
        };
        let pre_uav_overlap_cs = if self.opts.opt_uav0_clear_overlap
            && masked_mode
            && only_uav_items
            && !self.bar.uav_gfx_dirty
            && !self.bar.needs_barrier
            && self.bar.cs_mid_res_pend.iter().all(|o| o.disp_after.is_empty())
            && cs_pend_unsat.is_empty()
            && matches!(self.enc, Encoder::Compute(_))
            && ((self.bar.enc_sync_after.contains(shader)
                && self.bar.enc_sync_before.contains(S::Dispatch))
                || cs_disp_synced_all)
        {
            let cs_end_res_disp = if self.bar.cs_end_res.iter().any(|&(_, da, _, _)| !da.is_empty())
            {
                S::Dispatch
            } else {
                S::empty()
            };
            let cs = (self.ib.ib_stages & (S::Dispatch | S::AccelerationStructure))
                | (self.bar.cs_end_after[0] & COMPUTE_STAGES)
                | cs_end_res_disp;
            Some(if cs.is_empty() { S::Dispatch } else { cs })
        } else {
            None
        };
        let is_non_uav_clear = |p: &PendingClear| {
            p.is_dsv()
                || (p.view.resource != 0
                    && unsafe {
                        (*(p.view.resource as *const Resource)).desc.Flags
                            & crate::types::resource_flags::ALLOW_UNORDERED_ACCESS
                            == 0
                    })
        };
        let pre_clear_len = self.clears.pending_clears.len();
        let flushed = if let Some(why) = flush_all_why {
            self.flush_clears(why, |_| true)
        } else if (smart_uav0 || dsv_uav0 || non_uav_uav0) && uav_all && !kept_render {
            let kept_smart = smart_uav0
                && self
                    .clears
                    .pending_clears
                    .iter()
                    .any(|p| !res.contains(&p.view.resource) && Self::can_compute_clear(p));
            let kept_dsv = dsv_uav0
                && self
                    .clears
                    .pending_clears
                    .iter()
                    .any(|p| !res.contains(&p.view.resource) && !Self::can_compute_clear(p));
            let kept_non_uav = non_uav_uav0
                && self.clears.pending_clears.iter().any(|p| {
                    !res.contains(&p.view.resource)
                        && is_non_uav_clear(p)
                        && !(smart_uav0 && Self::can_compute_clear(p))
                        && !(dsv_uav0 && !Self::can_compute_clear(p))
                });
            if kept_smart {
                stats::inc(&stats::c::cf_smart_keep);
            }
            if kept_dsv {
                stats::inc(&stats::c::cf_dsv_keep);
            }
            if kept_non_uav {
                stats::inc(&stats::c::clear_pass_fold_v2);
            }
            self.flush_clears(&stats::c::cf_barrier_uav0, |p| {
                if res.contains(&p.view.resource)
                    || split_begin_all
                    || split_begin_res.contains(&p.view.resource)
                {
                    return true;
                }
                if non_uav_uav0 && is_non_uav_clear(p) {
                    return false;
                }
                if Self::can_compute_clear(p) {
                    !smart_uav0
                } else {
                    !dsv_uav0
                }
            })
        } else {
            if uav_all && kept_render && !self.clears.pending_clears.is_empty() {
                if smart_uav0
                    && self
                        .clears
                        .pending_clears
                        .iter()
                        .any(|p| Self::can_compute_clear(p))
                {
                    stats::inc(&stats::c::cf_smart_keep);
                }
                if dsv_uav0
                    && self
                        .clears
                        .pending_clears
                        .iter()
                        .any(|p| !Self::can_compute_clear(p))
                {
                    stats::inc(&stats::c::cf_dsv_keep);
                }
                if non_uav_uav0
                    && self.clears.pending_clears.iter().any(|p| {
                        is_non_uav_clear(p)
                            && !(smart_uav0 && Self::can_compute_clear(p))
                            && !(dsv_uav0 && !Self::can_compute_clear(p))
                    })
                {
                    stats::inc(&stats::c::clear_pass_fold_v2);
                }
            }
            self.flush_clears(&stats::c::cf_barrier_res, |p| {
                res.contains(&p.view.resource)
                    || split_begin_all
                    || split_begin_res.contains(&p.view.resource)
            })
        };
        if let Err(e) = flushed {
            log::warn!(target: "d3d", "ResourceBarrier: deferred clear: {e}");
        }
        let had_flushed_clears = self.clears.pending_clears.len() != pre_clear_len;
        if had_flushed_clears && !split_begin_keys.is_empty() {
            let post_req = self.cur_producer_gen();
            for entry in &mut self.bar.split_barriers {
                if split_begin_keys.contains(&(entry.0, entry.1, entry.2, entry.3)) {
                    entry.4 = post_req;
                }
            }
        }
        if copy_light {
            after = S::Blit;
            before = S::Blit;
            soft = [S::empty(); 2];
            local_split = SplitMasks::default();
            local_split.add(S::Blit, S::Blit, [S::empty(); 2]);
            stats::inc(&stats::c::copy_q_light);
        } else {
            if let Some(cs_after) = pre_uav_overlap_cs {
                if matches!(self.enc, Encoder::None) {
                    after = cs_after;
                    local_split = SplitMasks::default();
                    local_split.add(cs_after, before, soft);
                    local_rt_poison = true;
                    for v in &mut local_soft_items {
                        for (_, ia, _) in v {
                            if !ia.is_empty() {
                                *ia = cs_after;
                            }
                        }
                    }
                    if cs_after == S::All && !allow_all_soft {
                        local_soft_poison = [true; 2];
                    }
                    for (_, da, ba, aa) in &mut cs_mid_items {
                        if !da.is_empty() {
                            *da = cs_after;
                        }
                        if !ba.is_empty() {
                            *ba = cs_after;
                        }
                        if !aa.is_empty() {
                            *aa = cs_after;
                        }
                    }
                    stats::inc(&stats::c::uav0_clear_overlap);
                }
            }
            if track_exact_clear
                && !self.bar.clear_rp_res_ids.is_empty()
                && !kept_render
                && !all_res
                && !self.bar.in_list_gfx_non_clear
                && self.bar.gfx_uav_gen == 0
                && !self.bar.uav_gfx_dirty
            {
                let in_synced_cs = self.open_compute_disp_uav_synced();
                let anchor = self.pre_list_anchor_stages();
                if in_synced_cs || !anchor.is_empty() {
                    const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                    let mut na = S::empty();
                    let mut nb = S::empty();
                    let mut nsoft = [S::empty(); 2];
                    let mut nsplit = SplitMasks::default();
                    let mut n_soft_items = std::mem::take(&mut self.bar.bar_scratch_n_soft);
                    n_soft_items[0].clear();
                    n_soft_items[1].clear();
                    let mut n_soft_poison = [false; 2];
                    let mut n_cs_mid_items = std::mem::take(&mut self.bar.bar_scratch_n_cs_mid);
                    n_cs_mid_items.clear();
                    let mut n_uav0_disp_after = S::empty();
                    let mut exact_pt = std::mem::take(&mut self.bar.bar_scratch_exact_pt);
                    exact_pt.clear();
                    exact_pt.extend_from_slice(&self.bar.uav_passthrough);
                    let mut deferred_any = false;
                    let allow_all_cs_mid = self.opts.opt_cs_mid_qb_unpoison_v4
                        || self.opts.opt_cs_mid_qb_res_defer_v5
                        || self.opts.opt_desc_table_cs_fast
                        || self.opts.opt_cs_fast_bind_exact
                        || self.ps_to_cs_active();
                    let push_cs_mid = |out: &mut Vec<(usize, S, S, S)>, r: usize, ia: S, ib: S| {
                        if !(defer_cs_mid || collect_cs_soft)
                            || r == 0
                            || (ib == S::All && !allow_all_cs_mid)
                            || ia.is_empty()
                        {
                            return;
                        }
                        let da = if ib.contains(S::Dispatch) {
                            ia
                        } else {
                            S::empty()
                        };
                        let ba = if ib.contains(S::Blit) {
                            ia
                        } else {
                            S::empty()
                        };
                        let aa = if ib.contains(S::AccelerationStructure) {
                            ia
                        } else {
                            S::empty()
                        };
                        if !(da | ba | aa).is_empty() {
                            out.push((r, da, ba, aa));
                        }
                    };
                    for b in &active_items {
                        match *b {
                            Barrier::Uav(r) => {
                                let as_part = if self.bar.cs_as_gen > 0
                                    || self.bar.enc_ran_stages.contains(S::AccelerationStructure)
                                {
                                    S::AccelerationStructure
                                } else {
                                    S::empty()
                                };
                                let ia = strip_nonbuf_accel(
                                    r,
                                    S::Dispatch
                                        | as_part
                                        | Self::pt_for_uav(&exact_pt, &self.bar.clear_tex_cs, r)
                                        | if in_synced_cs { S::empty() } else { anchor },
                                );
                                let mut isoft = [S::empty(); 2];
                                add_soft_cls(&mut isoft, res_class(r), PRE_RASTER);
                                let uav_hard = strip_nonbuf_accel(r, shader - PRE_RASTER);
                                na |= ia;
                                nb |= uav_hard;
                                nsoft[0] |= isoft[0];
                                nsoft[1] |= isoft[1];
                                nsplit.add(ia, uav_hard, isoft);
                                record_soft_item(
                                    &mut n_soft_items,
                                    &mut n_soft_poison,
                                    r,
                                    ia,
                                    isoft,
                                );
                                push_cs_mid(&mut n_cs_mid_items, r, ia, uav_hard);
                                if r == 0 && uav_hard.contains(S::Dispatch) {
                                    n_uav0_disp_after |= ia;
                                }
                            }
                            Barrier::Transition(r, s0, s1) | Barrier::SplitEnd(r, _, s0, s1) => {
                                let (eff_s0, eff_s1) = if self.opts.opt_split_res_soft_sync
                                    || self.opts.opt_barrier_tight_v2
                                {
                                    (filter_res_state_bits(r, s0), filter_res_state_bits(r, s1))
                                } else {
                                    (s0, s1)
                                };
                                let st_after = self.source_stages_for_transition_with(
                                    r, eff_s0, s0, false, &exact_pt,
                                );
                                if (eff_s0 & 0x8) != 0 && (eff_s1 & 0x8) == 0 {
                                    Self::pt_remove(&mut exact_pt, r);
                                }
                                let mut st_before = S::empty();
                                let mut st_soft = [S::empty(); 2];
                                let soft_mask =
                                    if vs_buf_defer && r != 0 && res_class(r) == READS_BUF {
                                        SRV_UAV | VS_BUF_SOFT
                                    } else {
                                        SRV_UAV
                                    };
                                let hard = eff_s1 & !soft_mask;
                                if eff_s1 & soft_mask != 0 {
                                    let st = state_stages(eff_s1 & soft_mask);
                                    add_soft_cls(&mut st_soft, res_class(r), st & PRE_RASTER);
                                    st_before |= st - PRE_RASTER;
                                }
                                if hard != 0 || s1 == 0 {
                                    st_before |= if s1 == 0 { S::All } else { state_stages(hard) };
                                }
                                st_before = strip_nonbuf_accel(r, st_before);
                                if st_before == S::All {
                                    st_soft = [S::empty(); 2];
                                }
                                let has_pending =
                                    self.clears.pending_clears.iter().any(|p| p.view.resource == r);
                                if !self.bar.clear_rp_res_ids.contains(&r) && !has_pending {
                                    let cs_part = if st_after == S::All {
                                        COMPUTE_STAGES
                                    } else {
                                        st_after - (S::Fragment | PRE_RASTER)
                                    };
                                    let had_gfx = st_after == S::All
                                        || st_after.intersects(S::Fragment | PRE_RASTER);
                                    let dst_has_gfx = st_before == S::All
                                        || st_before.intersects(S::Fragment | PRE_RASTER)
                                        || !st_soft[0].is_empty()
                                        || !st_soft[1].is_empty();
                                    let gfx_anchor = if !had_gfx {
                                        S::empty()
                                    } else if dst_has_gfx {
                                        if !anchor.is_empty() {
                                            anchor
                                        } else {
                                            st_after & (S::Fragment | PRE_RASTER)
                                        }
                                    } else if !in_synced_cs {
                                        anchor
                                    } else {
                                        S::empty()
                                    };
                                    let ia = cs_part | gfx_anchor;
                                    if (eff_s1 & 0x8) != 0 {
                                        Self::pt_insert(&mut exact_pt, r, ia);
                                    }
                                    na |= ia;
                                    nb |= st_before;
                                    nsoft[0] |= st_soft[0];
                                    nsoft[1] |= st_soft[1];
                                    nsplit.add(ia, st_before, st_soft);
                                    record_soft_item(
                                        &mut n_soft_items,
                                        &mut n_soft_poison,
                                        r,
                                        ia,
                                        st_soft,
                                    );
                                    push_cs_mid(&mut n_cs_mid_items, r, ia, st_before);
                                } else if self.bar.clear_rp_res_ids.contains(&r)
                                    && !has_pending
                                    && s1 != 0
                                    && (st_before & S::Blit).is_empty()
                                    && (self.opts.opt_stagesync_barrier_v2
                                        || unsafe {
                                            (*(r as *const Resource)).desc.Flags & SIMUL == 0
                                        })
                                {
                                    let is_simul = unsafe {
                                        (*(r as *const Resource)).desc.Flags & SIMUL != 0
                                    };
                                    let cs_part = if st_after == S::All {
                                        S::empty()
                                    } else {
                                        st_after - (S::Fragment | PRE_RASTER)
                                    };
                                    if (eff_s1 & 0x8) != 0 {
                                        Self::pt_insert(&mut exact_pt, r, cs_part);
                                    }
                                    na |= cs_part;
                                    if !cs_part.is_empty() {
                                        nb |= st_before;
                                        nsoft[0] |= st_soft[0];
                                        nsoft[1] |= st_soft[1];
                                        nsplit.add(cs_part, st_before, st_soft);
                                        record_soft_item(
                                            &mut n_soft_items,
                                            &mut n_soft_poison,
                                            r,
                                            cs_part,
                                            st_soft,
                                        );
                                        push_cs_mid(&mut n_cs_mid_items, r, cs_part, st_before);
                                    }
                                    self.bar.clear_tex_gfx_before |=
                                        st_before & (S::Fragment | PRE_RASTER);
                                    self.bar.clear_tex_gfx_soft[0] |= st_soft[0];
                                    self.bar.clear_tex_gfx_soft[1] |= st_soft[1];
                                    if !(st_soft[0] | st_soft[1]).is_empty() {
                                        if r == 0 || is_simul {
                                            self.bar.clear_tex_gfx_soft_poisoned = true;
                                            self.bar.clear_tex_gfx_soft_res.clear();
                                        } else if !self.bar.clear_tex_gfx_soft_poisoned
                                            && !self.bar.clear_tex_gfx_soft_res.contains(&r)
                                        {
                                            self.bar.clear_tex_gfx_soft_res.push(r);
                                        }
                                    }
                                    if (st_before.intersects(COMPUTE_STAGES) || is_simul)
                                        && !self.bar.clear_tex_cs.contains(&r)
                                    {
                                        self.bar.clear_tex_cs.push(r);
                                    }
                                    deferred_any = true;
                                } else {
                                    self.bar.clear_tex_cs.retain(|&x| x != r);
                                    if (eff_s1 & 0x8) != 0 {
                                        Self::pt_insert(&mut exact_pt, r, st_after);
                                    }
                                    na |= st_after;
                                    nb |= st_before;
                                    nsoft[0] |= st_soft[0];
                                    nsoft[1] |= st_soft[1];
                                    nsplit.add(st_after, st_before, st_soft);
                                    record_soft_item(
                                        &mut n_soft_items,
                                        &mut n_soft_poison,
                                        r,
                                        st_after,
                                        st_soft,
                                    );
                                    push_cs_mid(&mut n_cs_mid_items, r, st_after, st_before);
                                }
                            }
                            Barrier::Full | Barrier::SplitBegin(..) => {}
                        }
                    }
                    if ((after == S::All || after.intersects(S::Fragment | PRE_RASTER))
                        && na != S::All
                        && !na.intersects(S::Fragment | PRE_RASTER))
                        || deferred_any
                    {
                        stats::inc(&stats::c::clear_overlap_exact);
                    }
                    std::mem::swap(&mut batch_pt, &mut exact_pt);
                    exact_pt.clear();
                    self.bar.bar_scratch_exact_pt = exact_pt;
                    after = na;
                    before = nb;
                    soft = nsoft;
                    local_split = nsplit;
                    std::mem::swap(&mut local_soft_items, &mut n_soft_items);
                    n_soft_items[0].clear();
                    n_soft_items[1].clear();
                    self.bar.bar_scratch_n_soft = n_soft_items;
                    local_soft_poison = n_soft_poison;
                    std::mem::swap(&mut cs_mid_items, &mut n_cs_mid_items);
                    n_cs_mid_items.clear();
                    self.bar.bar_scratch_n_cs_mid = n_cs_mid_items;
                    uav0_disp_after = n_uav0_disp_after;
                    local_rt_poison = true;
                    if after.is_empty() || (before | soft[0] | soft[1]).is_empty() {
                        std::mem::swap(&mut self.bar.uav_passthrough, &mut batch_pt);
                        self.restore_bar_scratch(
                            res,
                            local_rt_frag,
                            local_soft_items,
                            read_soft_items,
                            cs_mid_items,
                            read_cs_mid_items,
                            active_items,
                            batch_pt,
                            split_begin_res,
                            split_begin_keys,
                            local_narrow_trail,
                        );
                        return;
                    }
                }
            }
            std::mem::swap(&mut self.bar.uav_passthrough, &mut batch_pt);
            if self.opts.opt_subrange_uav_barrier
                && masked_mode
                && only_uav_items
                && self.open_compute_disp_uav_synced()
                && !self.bar.needs_barrier
            {
                let mut item_pt = S::empty();
                for b in items {
                    if let Barrier::Uav(r) = *b {
                        item_pt |= self.uav_passthrough_for_uav(r);
                    }
                }
                let prod = self.bar.enc_ran_stages & (S::Dispatch | S::AccelerationStructure);
                let def_after = if !prod.is_empty() {
                    prod | item_pt
                } else if self.bar.gfx_uav_gen > 0 {
                    S::Dispatch | S::AccelerationStructure | S::Fragment | PRE_RASTER
                } else {
                    S::Dispatch
                        | if self.bar.cs_as_gen > 0 {
                            S::AccelerationStructure
                        } else {
                            S::empty()
                        }
                        | item_pt
                        | self.pre_list_anchor_stages()
                };
                self.bar.cs_uav_after |= def_after;
                self.bar.cs_uav_before |= before;
                self.bar.cs_uav_soft[0] |= soft[0];
                self.bar.cs_uav_soft[1] |= soft[1];
                if self.opts.opt_cs_to_ps_overlap_v2
                    || self.gfx_to_ps_overlap_active()
                    || self.list_rsplit_v2_active()
                {
                    const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
                    for b in items {
                        if let Barrier::Uav(r) = *b {
                            let simul = r != 0
                                && unsafe { (*(r as *const Resource)).desc.Flags & SIMUL != 0 };
                            let rc = if r != 0 {
                                res_class(r)
                            } else {
                                READS_BUF | READS_TEX
                            };
                            for i in 0..2 {
                                if rc & (1 << i) == 0 || (soft[i] & PRE_RASTER).is_empty() {
                                    continue;
                                }
                                if r == 0 || simul || (def_after == S::All && !allow_all_soft) {
                                    self.bar.cs_uav_soft_poisoned[i] = true;
                                    self.bar.cs_uav_soft_items[i].clear();
                                } else if !self.bar.cs_uav_soft_poisoned[i]
                                    && !self.bar.cs_uav_soft_items[i].contains(&r)
                                {
                                    self.bar.cs_uav_soft_items[i].push(r);
                                }
                            }
                        }
                    }
                }
                stats::inc(&stats::c::cs_uav_mid_skip);
                self.restore_bar_scratch(
                    res,
                    local_rt_frag,
                    local_soft_items,
                    read_soft_items,
                    cs_mid_items,
                    read_cs_mid_items,
                    active_items,
                    batch_pt,
                    split_begin_res,
                    split_begin_keys,
                    local_narrow_trail,
                );
                return;
            }
        }
        if !local_narrow_trail.is_empty() && masked_mode && !copy_light {
            let trail_req = if kept_render && self.opts.opt_rp_keep_write_exact {
                self.bar.stage_gen
            } else if all_matched_split_end && any_matched_split_end && !had_flushed_clears {
                matched_split_req
            } else {
                self.cur_producer_gen()
            };
            for (nr, na, nb) in local_narrow_trail.drain(..) {
                self.bar.trail_narrow_items.push((nr, na, nb, trail_req));
            }
        }
        if after == S::All && before == S::All {
            if track_frag_rt {
                self.bar.frag_rt_poisoned = true;
                self.bar.frag_rt_pend.clear();
            }
            self.need_full(FullWhy::App);
        } else if kept_render && had_write_or_uav {
            // Kept write/UAV items are recorded in `rp_write_after`/`rp_write_before`
            // (and `rp_write_soft`) and land when this render pass ends; only read-only
            // transitions need `render_mid_barrier` inside the open render pass.
            if !read_after.is_empty() && !(read_before | read_soft[0] | read_soft[1]).is_empty() {
                self.bar.pend_after |= read_after;
                self.bar.pend_before |= read_before;
                self.bar.pend_before_soft[0] |= read_soft[0];
                self.bar.pend_before_soft[1] |= read_soft[1];
                let req = if self.opts.opt_rp_keep_write_exact {
                    if !self.bar.enc_ran_stages.is_empty() {
                        stats::inc(&stats::c::rp_keep_exact);
                    }
                    self.bar.stage_gen
                } else {
                    self.cur_producer_gen()
                };
                if track_frag_rt && (read_before == S::All || read_before.contains(S::Fragment)) {
                    if all_res {
                        self.bar.frag_rt_poisoned = true;
                        self.bar.frag_rt_pend.clear();
                    } else {
                        self.bar.frag_non_rt_after |= read_after;
                        merge_req_for_after(&mut self.bar.frag_non_rt_req, read_after, &req);
                    }
                }
                self.bar.split_pend.merge_masks(&read_split, &req);
                if track_soft_items {
                    for i in 0..2 {
                        if read_soft_poison[i] {
                            self.bar.soft_pend_poisoned[i] = true;
                            self.bar.soft_pend_items[i].clear();
                        } else if !self.bar.soft_pend_poisoned[i] {
                            for &(r, ia, ib) in &read_soft_items[i] {
                                Self::push_soft_obl_into(
                                    &mut self.bar.soft_pend_items[i],
                                    r,
                                    ia,
                                    ib,
                                    req,
                                );
                            }
                        }
                    }
                }
                if self.ps_to_cs_active() && self.render_split_sync_active() {
                    if self.bar.needs_barrier || all_res || (uav_all && !self.ps_to_cs_v2_active())
                    {
                        if !read_split.disp_after.is_empty() {
                            self.record_cs_soft_disp(0, read_split.disp_after, req);
                        }
                    } else {
                        for &(r, da, _, _) in &read_cs_mid_items {
                            self.record_cs_soft_disp(r, da, req);
                        }
                    }
                }
                if collect_pend_cs {
                    for &(r, da, ba, aa) in &read_cs_mid_items {
                        Self::push_cs_mid_obl_into(
                            &mut self.bar.pend_cs_items,
                            r,
                            da,
                            ba,
                            aa,
                            req,
                        );
                    }
                }
            }
        } else if defer_cs_mid_v2
            && !copy_light
            && matches!(self.enc, Encoder::Compute(_))
            && !self.bar.needs_barrier
            && !had_uav_item
            && !all_res
            && !uav_all
            && !had_flushed_clears
            && before != S::All
        {
            let req = self.cur_producer_gen();
            local_split.disp_after = S::empty();
            local_split.blit_after = S::empty();
            local_split.as_after = S::empty();
            self.bar.split_pend.merge_masks(&local_split, &req);
            self.sync_legacy_from_split();
            if track_frag_rt {
                if local_rt_poison {
                    self.bar.frag_rt_poisoned = true;
                    self.bar.frag_rt_pend.clear();
                } else if !self.bar.frag_rt_poisoned {
                    if !local_non_rt_frag.is_empty() {
                        self.bar.frag_non_rt_after |= local_non_rt_frag;
                        merge_req_for_after(&mut self.bar.frag_non_rt_req, local_non_rt_frag, &req);
                    }
                    for &(r, ia) in &local_rt_frag {
                        self.push_frag_rt_pend(r, ia, &req);
                    }
                }
            }
            if track_soft_items {
                for i in 0..2 {
                    if local_soft_poison[i] {
                        self.bar.soft_pend_poisoned[i] = true;
                        self.bar.soft_pend_items[i].clear();
                    } else if !self.bar.soft_pend_poisoned[i] {
                        for &(r, ia, ib) in &local_soft_items[i] {
                            Self::push_soft_obl_into(
                                &mut self.bar.soft_pend_items[i],
                                r,
                                ia,
                                ib,
                                req,
                            );
                        }
                    }
                }
            }
            let mid_on = self.mid_qb_stagesync_active();
            let sg = self.bar.stage_gen;
            for &(r, da, ba, aa) in &cs_mid_items {
                let mut uda = S::empty();
                let mut uba = S::empty();
                let mut uaa = S::empty();
                for (ci, cbit, ia, out_u) in [
                    (0usize, S::Dispatch, da, &mut uda),
                    (1usize, S::Blit, ba, &mut uba),
                    (2usize, S::AccelerationStructure, aa, &mut uaa),
                ] {
                    if !ia.is_empty() {
                        let unsat = self.unsatisfied_enc_after_for_req(ia, cbit, &sg, mid_on);
                        *out_u = unsat;
                        self.bar.cs_end_after[ci] |=
                            (ia - unsat) | (ia & self.bar.enc_ran_stages);
                    }
                }
                if !(uda | uba | uaa).is_empty() {
                    self.push_cs_mid_res_obl(r, uda, uba, uaa, sg);
                }
            }
        } else if self.cs_mid_qb_touch_defer_active()
            && !copy_light
            && matches!(self.enc, Encoder::Compute(_))
            && !self.bar.needs_barrier
            && !all_res
            && !uav_all
            && !had_flushed_clears
            && before != S::All
        {
            let req = self.cur_producer_gen();
            local_split.disp_after = S::empty();
            local_split.blit_after = S::empty();
            local_split.as_after = S::empty();
            self.bar.split_pend.merge_masks(&local_split, &req);
            self.sync_legacy_from_split();
            if track_frag_rt {
                if local_rt_poison {
                    self.bar.frag_rt_poisoned = true;
                    self.bar.frag_rt_pend.clear();
                } else if !self.bar.frag_rt_poisoned {
                    if !local_non_rt_frag.is_empty() {
                        self.bar.frag_non_rt_after |= local_non_rt_frag;
                        merge_req_for_after(&mut self.bar.frag_non_rt_req, local_non_rt_frag, &req);
                    }
                    for &(r, ia) in &local_rt_frag {
                        self.push_frag_rt_pend(r, ia, &req);
                    }
                }
            }
            if track_soft_items {
                for i in 0..2 {
                    if local_soft_poison[i] {
                        self.bar.soft_pend_poisoned[i] = true;
                        self.bar.soft_pend_items[i].clear();
                    } else if !self.bar.soft_pend_poisoned[i] {
                        for &(r, ia, ib) in &local_soft_items[i] {
                            Self::push_soft_obl_into(
                                &mut self.bar.soft_pend_items[i],
                                r,
                                ia,
                                ib,
                                req,
                            );
                        }
                    }
                }
            }
            for &(r, da, ba, aa) in &cs_mid_items {
                self.push_cs_mid_res_obl(r, da, ba, aa, req);
            }
        } else {
            self.bar.pend_after |= after;
            self.bar.pend_before |= before;
            self.bar.pend_before_soft[0] |= soft[0];
            self.bar.pend_before_soft[1] |= soft[1];
            let req = if kept_render && self.opts.opt_rp_keep_write_exact {
                if !self.bar.enc_ran_stages.is_empty() {
                    stats::inc(&stats::c::rp_keep_exact);
                }
                self.bar.stage_gen
            } else if all_matched_split_end && any_matched_split_end && !had_flushed_clears {
                matched_split_req
            } else {
                self.cur_producer_gen()
            };
            self.bar.split_pend.merge_masks(&local_split, &req);
            if track_frag_rt {
                if self.bar.needs_barrier || all_res || local_rt_poison {
                    self.bar.frag_rt_poisoned = true;
                    self.bar.frag_rt_pend.clear();
                } else if !self.bar.frag_rt_poisoned {
                    if !local_non_rt_frag.is_empty() {
                        self.bar.frag_non_rt_after |= local_non_rt_frag;
                        merge_req_for_after(&mut self.bar.frag_non_rt_req, local_non_rt_frag, &req);
                    }
                    for &(r, ia) in &local_rt_frag {
                        self.push_frag_rt_pend(r, ia, &req);
                    }
                }
            }
            if track_soft_items {
                if self.bar.needs_barrier || all_res {
                    self.bar.soft_pend_poisoned = [true; 2];
                    self.bar.soft_pend_items[0].clear();
                    self.bar.soft_pend_items[1].clear();
                } else {
                    for i in 0..2 {
                        if local_soft_poison[i] {
                            self.bar.soft_pend_poisoned[i] = true;
                            self.bar.soft_pend_items[i].clear();
                        } else if !self.bar.soft_pend_poisoned[i] {
                            for &(r, ia, ib) in &local_soft_items[i] {
                                Self::push_soft_obl_into(
                                    &mut self.bar.soft_pend_items[i],
                                    r,
                                    ia,
                                    ib,
                                    req,
                                );
                            }
                        }
                    }
                }
            }
            if self.ps_to_cs_active() && self.render_split_sync_active() {
                if self.bar.needs_barrier || all_res || (uav_all && !self.ps_to_cs_v2_active()) {
                    if !local_split.disp_after.is_empty() {
                        self.record_cs_soft_disp(0, local_split.disp_after, req);
                    }
                } else {
                    for &(r, da, _, _) in &cs_mid_items {
                        self.record_cs_soft_disp(r, da, req);
                    }
                    if uav_all && !uav0_disp_after.is_empty() {
                        self.record_cs_soft_disp(0, uav0_disp_after, req);
                    }
                }
            }
            if collect_pend_cs {
                if self.bar.needs_barrier || all_res {
                    self.bar.pend_cs_poisoned = [true; 3];
                } else {
                    for &(r, da, ba, aa) in &cs_mid_items {
                        Self::push_cs_mid_obl_into(
                            &mut self.bar.pend_cs_items,
                            r,
                            da,
                            ba,
                            aa,
                            req,
                        );
                    }
                    if uav_all {
                        if matches!(self.enc, Encoder::Compute(_)) {
                            let uav0_after = self.source_stages_for_uav_with(
                                0,
                                shader,
                                false,
                                &self.bar.uav_passthrough,
                            );
                            let mid_on = self.mid_qb_stagesync_active();
                            let pt0 = self.uav_passthrough_for_uav(0);
                            let disp_uav0_sat = (self.open_compute_disp_uav_synced()
                                && pt0.is_empty())
                                || self
                                    .unsatisfied_enc_after_for_req(
                                        uav0_after,
                                        S::Dispatch,
                                        &self.bar.stage_gen,
                                        mid_on,
                                    )
                                    .is_empty();
                            if !disp_uav0_sat {
                                self.bar.pend_cs_poisoned[0] = true;
                            }
                            if !self
                                .unsatisfied_enc_after_for_req(
                                    uav0_after,
                                    S::AccelerationStructure,
                                    &self.bar.stage_gen,
                                    mid_on,
                                )
                                .is_empty()
                            {
                                self.bar.pend_cs_poisoned[2] = true;
                            }
                            if !track_cs_mid_v3 || !self.opts.opt_cs_mid_qb_unpoison_v4 {
                                let prod = self.bar.enc_ran_stages
                                    & (S::Dispatch | S::AccelerationStructure);
                                let def_after = if !prod.is_empty() {
                                    prod | pt0
                                } else if self.bar.gfx_uav_gen > 0 {
                                    S::Dispatch
                                        | S::AccelerationStructure
                                        | S::Fragment
                                        | PRE_RASTER
                                } else {
                                    S::Dispatch
                                        | if self.bar.cs_as_gen > 0 {
                                            S::AccelerationStructure
                                        } else {
                                            S::empty()
                                        }
                                        | pt0
                                        | self.pre_list_anchor_stages()
                                };
                                self.bar.cs_uav_after |= def_after;
                                self.bar.cs_uav_before |= shader - PRE_RASTER;
                                self.bar.cs_uav_soft[0] |= shader & PRE_RASTER;
                                self.bar.cs_uav_soft[1] |= shader & PRE_RASTER;
                                self.bar.cs_uav_soft_poisoned = [true; 2];
                                self.bar.cs_uav_soft_items[0].clear();
                                self.bar.cs_uav_soft_items[1].clear();
                            }
                        } else {
                            self.bar.pend_cs_poisoned[0] = true;
                            self.bar.pend_cs_poisoned[2] = true;
                        }
                    }
                }
            } else if copy_light {
                self.bar.pend_cs_poisoned[1] = true;
            }
            if track_cs_mid_v3 && matches!(self.enc, Encoder::Compute(_)) {
                if !self.opts.opt_cs_mid_qb_unpoison_v4 {
                    if self.bar.needs_barrier
                        || all_res
                        || uav_all
                        || had_flushed_clears
                        || before == S::All
                    {
                        self.bar.cs_mid_v3_poisoned = [true; 3];
                    } else {
                        for &(r, da, ba, aa) in &cs_mid_items {
                            Self::push_cs_mid_obl_into(
                                &mut self.bar.cs_mid_v3_items,
                                r,
                                da,
                                ba,
                                aa,
                                req,
                            );
                        }
                    }
                } else if self.bar.needs_barrier || all_res || had_flushed_clears {
                    self.bar.cs_mid_v3_poisoned = [true; 3];
                } else {
                    for &(r, da, ba, aa) in &cs_mid_items {
                        Self::push_cs_mid_obl_into(
                            &mut self.bar.cs_mid_v3_items,
                            r,
                            da,
                            ba,
                            aa,
                            req,
                        );
                    }
                    if uav_all {
                        let uav0_after = self.source_stages_for_uav_with(
                            0,
                            shader,
                            false,
                            &self.bar.uav_passthrough,
                        );
                        let mid_on = self.mid_qb_stagesync_active();
                        let pt0 = self.uav_passthrough_for_uav(0);
                        let disp_uav0_sat = (self.open_compute_disp_uav_synced() && pt0.is_empty())
                            || self
                                .unsatisfied_enc_after_for_req(
                                    uav0_after,
                                    S::Dispatch,
                                    &self.bar.stage_gen,
                                    mid_on,
                                )
                                .is_empty();
                        if !disp_uav0_sat {
                            self.bar.cs_mid_v3_poisoned[0] = true;
                        }
                        if !self
                            .unsatisfied_enc_after_for_req(
                                uav0_after,
                                S::AccelerationStructure,
                                &self.bar.stage_gen,
                                mid_on,
                            )
                            .is_empty()
                        {
                            self.bar.cs_mid_v3_poisoned[2] = true;
                        }
                        let prod =
                            self.bar.enc_ran_stages & (S::Dispatch | S::AccelerationStructure);
                        let def_after = if !prod.is_empty() {
                            prod | pt0
                        } else if self.bar.gfx_uav_gen > 0 {
                            S::Dispatch | S::AccelerationStructure | S::Fragment | PRE_RASTER
                        } else {
                            S::Dispatch
                                | if self.bar.cs_as_gen > 0 {
                                    S::AccelerationStructure
                                } else {
                                    S::empty()
                                }
                                | pt0
                                | self.pre_list_anchor_stages()
                        };
                        self.bar.cs_uav_after |= def_after;
                        self.bar.cs_uav_before |= shader - PRE_RASTER;
                        self.bar.cs_uav_soft[0] |= shader & PRE_RASTER;
                        self.bar.cs_uav_soft[1] |= shader & PRE_RASTER;
                        self.bar.cs_uav_soft_poisoned = [true; 2];
                        self.bar.cs_uav_soft_items[0].clear();
                        self.bar.cs_uav_soft_items[1].clear();
                    }
                }
            }
        }
        self.restore_bar_scratch(
            res,
            local_rt_frag,
            local_soft_items,
            read_soft_items,
            cs_mid_items,
            read_cs_mid_items,
            active_items,
            batch_pt,
            split_begin_res,
            split_begin_keys,
            local_narrow_trail,
        );
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn restore_bar_scratch(
        &mut self,
        mut res: Vec<usize>,
        mut local_rt_frag: Vec<(usize, MTLStages)>,
        mut local_soft_items: [Vec<(usize, MTLStages, MTLStages)>; 2],
        mut read_soft_items: [Vec<(usize, MTLStages, MTLStages)>; 2],
        mut cs_mid_items: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
        mut read_cs_mid_items: Vec<(usize, MTLStages, MTLStages, MTLStages)>,
        mut active_items: Vec<Barrier>,
        mut batch_pt: Vec<(usize, MTLStages)>,
        mut split_begin_res: Vec<usize>,
        mut split_begin_keys: Vec<(usize, u32, u32, u32)>,
        mut local_narrow_trail: Vec<(usize, MTLStages, MTLStages)>,
    ) {
        res.clear();
        self.bar.bar_scratch_res = res;
        if local_rt_frag.capacity() > 0 {
            local_rt_frag.clear();
            self.bar.bar_scratch_rt_frag = local_rt_frag;
        }
        if local_soft_items[0].capacity() > 0 || local_soft_items[1].capacity() > 0 {
            local_soft_items[0].clear();
            local_soft_items[1].clear();
            self.bar.bar_scratch_soft = local_soft_items;
        }
        if read_soft_items[0].capacity() > 0 || read_soft_items[1].capacity() > 0 {
            read_soft_items[0].clear();
            read_soft_items[1].clear();
            self.bar.bar_scratch_read_soft = read_soft_items;
        }
        if cs_mid_items.capacity() > 0 {
            cs_mid_items.clear();
            self.bar.bar_scratch_cs_mid = cs_mid_items;
        }
        if read_cs_mid_items.capacity() > 0 {
            read_cs_mid_items.clear();
            self.bar.bar_scratch_read_cs_mid = read_cs_mid_items;
        }
        if active_items.capacity() > 0 {
            active_items.clear();
            self.bar.bar_scratch_active = active_items;
        }
        batch_pt.clear();
        self.bar.bar_scratch_pt = batch_pt;
        if split_begin_res.capacity() > 0 {
            split_begin_res.clear();
            self.bar.bar_scratch_split_begin_res = split_begin_res;
        }
        if split_begin_keys.capacity() > 0 {
            split_begin_keys.clear();
            self.bar.bar_scratch_split_begin_keys = split_begin_keys;
        }
        if local_narrow_trail.capacity() > 0 {
            local_narrow_trail.clear();
            self.bar.bar_scratch_narrow_trail = local_narrow_trail;
        }
    }

    /// Before a draw in an already open draw pass: pending barrier stages
    /// for render stages (kept by `barrier_list`) as a mid-pass queue
    /// barrier; compute consumers stay pending for the next encoder.
    pub(super) fn render_mid_barrier(
        &mut self,
        e: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
        pre_b: MTLStages,
    ) {
        let pre_b = if legacy_barriers() || !barrier_masks() {
            PRE_RASTER
        } else {
            let st = pre_b & PRE_RASTER;
            if st.is_empty() { PRE_RASTER } else { st }
        };
        let omitted_pre = PRE_RASTER - pre_b;
        self.ensure_pre_raster_stage_synced(e, pre_b);
        let dom_exact = self.render_dom_exact_active();
        if self.render_split_sync_active() {
            if self.bar.split_pend.is_empty() {
                return;
            }
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
                    let sp_poison = std::mem::replace(&mut self.bar.soft_pend_poisoned[i], false);
                    let mut rem_sa = sub_stages_preserving_all(sa, orig_vtx);
                    if self.opts.opt_gfx_overlap_capped
                        && !self.gfx_to_ps_overlap_active()
                        && rem_sa != MTLStages::All
                        && rem_sa.contains(MTLStages::Fragment)
                    {
                        orig_vtx |= MTLStages::Fragment;
                        vtx_req[STAGE_FRAGMENT] = vtx_req[STAGE_FRAGMENT].max(sr[STAGE_FRAGMENT]);
                        rem_sa = sub_stages_preserving_all(rem_sa, MTLStages::Fragment);
                    }
                    if !rem_sa.is_empty() && !sb.is_empty() {
                        if orig_vtx != MTLStages::empty() {
                            Self::strip_soft_items_after(&mut sp_items, orig_vtx);
                        }
                        if rem_sa != MTLStages::All && rem_sa.contains(MTLStages::Fragment) {
                            deferred_inlist_frag = true;
                        }
                        self.bar.vtx_defer[i] = merge_vtx(self.bar.vtx_defer[i], Some((rem_sa, sb)));
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
            if (orig_vtx | orig_frag).is_empty() {
                return;
            }
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
                    let u =
                        self.unsatisfied_enc_after_for_req(orig_vtx, pre_b, &vtx_req, mid_on);
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
                return;
            }
            if eff_vtx != orig_vtx || eff_frag != orig_frag {
                stats::inc(&stats::c::stagesync_narrow);
            }
            let both = eff_vtx & eff_frag;
            let vtx_only = eff_vtx - both;
            let frag_only = eff_frag - both;
            if !both.is_empty() {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    both,
                    pre_b | MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
            }
            if !vtx_only.is_empty() {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    vtx_only,
                    pre_b,
                    MTL4VisibilityOptions::Device,
                );
            }
            if !frag_only.is_empty() {
                e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                    frag_only,
                    MTLStages::Fragment,
                    MTL4VisibilityOptions::Device,
                );
            }
            let rec_vtx = if self.opts.opt_rp_keep_write_exact {
                eff_vtx
            } else {
                orig_vtx
            };
            let rec_frag = if self.opts.opt_rp_keep_write_exact {
                eff_frag
            } else {
                orig_frag
            };
            if !rec_vtx.is_empty() {
                self.record_enc_queue_sync(rec_vtx, pre_b);
                if rec_vtx.contains(MTLStages::Fragment) || rec_vtx == MTLStages::All {
                    self.rp_sync.gfx_ps_overlap_chain = 0;
                }
                if (self.opts.opt_cs_to_ps_overlap_v2 || self.list_rsplit_v2_active()) && dom_exact {
                    self.prune_satisfied_vtx_defer(mid_on);
                }
            }
            if !rec_frag.is_empty() {
                self.record_enc_queue_sync(rec_frag, MTLStages::Fragment);
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
            return;
        }
        if self.bar.pend_after.is_empty() {
            return;
        }
        self.reset_frag_rt_pend();
        const RENDER: MTLStages = PRE_RASTER.union(MTLStages::Fragment);
        if self.opts.opt_split_res_soft_sync || self.opts.opt_barrier_tight_v2 {
            let hard_before = self.bar.pend_before & RENDER;
            let mut deferred = false;
            for i in 0..2 {
                self.bar.soft_pend_items[i].clear();
                self.bar.soft_pend_poisoned[i] = false;
                let s = std::mem::replace(&mut self.bar.pend_before_soft[i], MTLStages::empty())
                    - hard_before;
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
                    e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                        eff_after,
                        hard_before,
                        MTL4VisibilityOptions::Device,
                    );
                    if hard_before.intersects(PRE_RASTER)
                        && (orig_after.contains(MTLStages::Fragment)
                            || orig_after == MTLStages::All)
                    {
                        self.rp_sync.gfx_ps_overlap_chain = 0;
                    }
                    self.record_enc_queue_sync(orig_after, hard_before);
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
            return;
        }
        let soft = self.bar.pend_before_soft[0] | self.bar.pend_before_soft[1];
        let before = (self.bar.pend_before | soft) & RENDER;
        if before.is_empty() {
            return;
        }
        let orig_after = self.bar.pend_after;
        let eff_after = self.unsatisfied_mid_qb_after(orig_after, before);
        if eff_after.is_empty() {
            stats::inc(&stats::c::stagesync_skip);
        } else {
            if eff_after != orig_after {
                stats::inc(&stats::c::stagesync_narrow);
            }
            e.barrierAfterQueueStages_beforeStages_visibilityOptions(
                eff_after,
                before,
                MTL4VisibilityOptions::Device,
            );
            if before.intersects(PRE_RASTER)
                && (orig_after.contains(MTLStages::Fragment) || orig_after == MTLStages::All)
            {
                self.rp_sync.gfx_ps_overlap_chain = 0;
            }
            self.record_enc_queue_sync(orig_after, before);
            stats::inc(&stats::c::rb_mid_qb);
            if eff_after.contains(MTLStages::Fragment) && before.contains(MTLStages::Fragment) {
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
