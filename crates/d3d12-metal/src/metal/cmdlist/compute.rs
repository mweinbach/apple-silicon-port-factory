//! Compute pipeline/heap/root binding and `Dispatch`.

use super::state::DispatchExact;
use super::CommandList;
use dxil_metal::runtime::{
    ARGUMENT_BUFFER_BIND_POINT, DESCRIPTOR_HEAP_BIND_POINT, SAMPLER_HEAP_BIND_POINT,
};
use crate::metal::descriptor;
use crate::metal::encstats as stats;
use crate::metal::gpunan;
use crate::metal::indirect::{IndirectLayout, Work};
use crate::metal::pipeline::{
    resolve_pso_hk_profile, ComputeBind, ComputePipeline, HandleKindProfile, READS_TEX,
};
use crate::metal::resource;
use objc2_metal::{MTL4ComputeCommandEncoder, MTLSize, MTLStages};
use std::sync::Arc;

impl CommandList {
    /// `SetDescriptorHeaps`: GPU addresses of the shader-visible heaps
    /// (0 = none of that type).
    pub fn set_descriptor_heaps(&mut self, resource: u64, sampler: u64) {
        self.heaps = [resource, sampler];
        self.heap0_cpu = (resource != 0)
            .then(|| descriptor::resolve_shader_heap(resource))
            .flatten();
        self.binds.cs_bind_cache.valid = false;
        self.binds.gfx_bind_cache.valid = false;
    }

    /// `SetPipelineState` with a compute PSO.
    pub fn set_compute_pipeline(&mut self, p: Arc<ComputePipeline>) {
        if self.hk_any_active() {
            self.rp_sync.cs_hk_profile = resolve_pso_hk_profile(p.hk_profile, &p.shader, &p.access);
        }
        if gpunan::is_capturing() {
            self.diag_last_kind = "cs";
            self.diag_last_key = p.shader.key as u64;
            self.diag_last_vs_key = 0;
        }
        if self.compute_pso.as_ref().is_some_and(|c| Arc::ptr_eq(c, &p)) {
            return;
        }
        self.binds.cs_bind_cache.valid = false;
        self.compute_pso = Some(p);
        self.compute_pso_bound = false;
    }

    /// Bind heaps and the current top-level argument buffer in the table.
    pub(super) fn bind_table(&mut self, root: u64) {
        if self.heaps[0] != 0 {
            self.set_table_addr(self.heaps[0], DESCRIPTOR_HEAP_BIND_POINT as usize);
        }
        if self.heaps[1] != 0 {
            self.set_table_addr(self.heaps[1], SAMPLER_HEAP_BIND_POINT as usize);
        }
        if root != 0 {
            self.set_table_addr(root, ARGUMENT_BUFFER_BIND_POINT as usize);
        }
    }

    #[inline]
    pub(super) fn flush_root(&mut self, compute: bool) -> Result<u64, String> {
        let root = if compute { &mut self.compute_root } else { &mut self.gfx_root };
        if root.rs.is_none() {
            return Ok(0);
        }
        if !root.is_dirty() {
            return Ok(root.bound);
        }
        let ring = self.ring.as_ref().ok_or("command list is not open")?;
        let mut ring = ring.lock().unwrap();
        root.flush(&mut ring)
    }

    /// `Dispatch(x, y, z)`.
    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) -> Result<(), String> {
        self.disc_unmark(0);
        let pso = self.compute_pso.clone().ok_or("Dispatch without a compute PSO")?;
        if x == 0 || y == 0 || z == 0 {
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
                work: Work::Dispatch,
                work_offset: 0,
                patches: Vec::new(),
                vb_offsets: Vec::new(),
                ib_offset: None,
            };
            return self.execute_indirect(&layout, 1, args, None, None);
        }
        self.flush_icb_prepass_if_pending();
        self.pt_split();
        let root = self.flush_root(true)?;
        let e = if self.ib_tight_active() {
            let mut scratch_uav = std::mem::take(&mut self.ib.ib_scratch_uav);
            let mut scratch_read = std::mem::take(&mut self.ib.ib_scratch_read);
            let mut scratch_op = std::mem::take(&mut self.ib.ib_scratch_op);
            let mut scratch_uav_spans = std::mem::take(&mut self.ib.ib_scratch_uav_spans);
            let mut scratch_read_spans = std::mem::take(&mut self.ib.ib_scratch_read_spans);
            let want_spans = self.opts.opt_subrange_uav_barrier;
            let opt_cache_mode = self.opts.opt_desc_table_cs_fast
                || self.opts.opt_cs_intra_barrier_exact
                || self.ps_to_cs_active()
                || self.opts.opt_cs_intra_barrier_v2
                || self.opts.opt_cs_bind_intra_safe
                || self.opts.opt_cs_fast_bind_exact
                || self.hk_v2_active();
            let want_bounded = (opt_cache_mode
                || self.opts.opt_ib_bindless_war
                || self.opts.opt_ib_exact_obl
                || self.opts.opt_subrange_uav_barrier
                || self.opts.opt_desc_table_res_exact
                || self.cs_intra_safe_active())
                && !pso.access.heap_indexed_or_unknown;
            let uav_slots = pso.access.uav_slots.as_deref();
            let read_slots = pso.access.read_slots.as_deref();
            let has_any_slots = uav_slots.is_some_and(|s| !s.is_empty())
                || read_slots.is_some_and(|s| !s.is_empty())
                || (want_bounded && !pso.access.bounded_read_slots.is_empty());
            let use_bind_cache = opt_cache_mode || self.opts.opt_cs_bind_cache;
            // Tier-1 generation check requires `bump_desc_heap_gen()` on every descriptor copy;
            // keep it gated to `opt_cs_bind_intra_safe` so default `opt_cs_bind_cache` uses
            // atomic-free Tier-2 token matching with zero global cache-line contention in
            // `CopyDescriptorsSimple`.
            let want_gen_cache = self.opts.opt_cs_bind_intra_safe;
            let (uav_exact, full_read_exact, bounded_read_exact) = if (use_bind_cache
                || has_any_slots)
                && !pso.access.heap_indexed_or_unknown
            {
                let pso_ptr = Arc::as_ptr(&pso) as usize;
                let reg_gen = resource::res_registry_gen();
                let desc_gen = if want_gen_cache { descriptor::desc_heap_gen() } else { 0 };
                let root_gen = self.compute_root.bind_gen;
                let heap0_base = self.heaps[0];
                if want_gen_cache
                    && self.binds.cs_bind_cache.matches_gen(
                        pso_ptr,
                        want_spans,
                        reg_gen,
                        desc_gen,
                        root_gen,
                        heap0_base,
                    )
                {
                    scratch_uav.clear();
                    scratch_uav.extend_from_slice(&self.binds.cs_bind_cache.uavs);
                    scratch_read.clear();
                    scratch_read.extend_from_slice(&self.binds.cs_bind_cache.reads);
                    if want_spans {
                        scratch_uav_spans.clear();
                        scratch_uav_spans.extend_from_slice(&self.binds.cs_bind_cache.uav_spans);
                        scratch_read_spans.clear();
                        scratch_read_spans.extend_from_slice(&self.binds.cs_bind_cache.read_spans);
                    }
                    if self.opts.opt_cs_bind_intra_safe {
                        stats::inc(&stats::c::cs_bind_intra_safe);
                    } else if self.opts.opt_cs_fast_bind_exact {
                        stats::inc(&stats::c::cs_fast_bind_exact);
                    } else if self.opts.opt_desc_table_cs_fast {
                        stats::inc(&stats::c::desc_table_cs_fast);
                    } else if self.opts.opt_cs_intra_barrier_exact {
                        stats::inc(&stats::c::cs_intra_barrier_exact);
                    } else if self.opts.opt_cs_intra_barrier_v2 {
                        stats::inc(&stats::c::cs_intra_barrier_v2);
                    }
                    (
                        self.binds.cs_bind_cache.uav_exact,
                        self.binds.cs_bind_cache.full_read_exact,
                        self.binds.cs_bind_cache.bounded_read_exact,
                    )
                } else {
                    let mut tokens = std::mem::take(&mut self.binds.cs_bind_tokens);
                    tokens.clear();
                    let u_ok = uav_slots.is_none_or(|s| {
                        Self::extract_compute_slot_tokens(
                            s,
                            &self.compute_root,
                            self.heap0_cpu,
                            &mut tokens,
                        )
                    });
                    let r_slots_opt: &[ComputeBind] = if let Some(s) = read_slots {
                        s
                    } else if want_bounded {
                        pso.access.bounded_read_slots.as_slice()
                    } else {
                        &[]
                    };
                    let r_ok = u_ok
                        && Self::extract_compute_slot_tokens(
                            r_slots_opt,
                            &self.compute_root,
                            self.heap0_cpu,
                            &mut tokens,
                        );
                    let res = if r_ok
                        && self.binds.cs_bind_cache.matches_tokens(
                            pso_ptr,
                            want_spans,
                            want_bounded,
                            reg_gen,
                            heap0_base,
                            &tokens,
                        )
                    {
                        scratch_uav.clear();
                        scratch_uav.extend_from_slice(&self.binds.cs_bind_cache.uavs);
                        scratch_read.clear();
                        scratch_read.extend_from_slice(&self.binds.cs_bind_cache.reads);
                        if want_spans {
                            scratch_uav_spans.clear();
                            scratch_uav_spans
                                .extend_from_slice(&self.binds.cs_bind_cache.uav_spans);
                            scratch_read_spans.clear();
                            scratch_read_spans
                                .extend_from_slice(&self.binds.cs_bind_cache.read_spans);
                        }
                        if want_gen_cache {
                            self.binds.cs_bind_cache.desc_gen = desc_gen;
                            self.binds.cs_bind_cache.root_gen = root_gen;
                            self.binds.cs_bind_cache.heap0_base = heap0_base;
                        }
                        if self.opts.opt_cs_bind_intra_safe {
                            stats::inc(&stats::c::cs_bind_intra_safe);
                        } else if self.opts.opt_cs_fast_bind_exact {
                            stats::inc(&stats::c::cs_fast_bind_exact);
                        } else if self.opts.opt_desc_table_cs_fast {
                            stats::inc(&stats::c::desc_table_cs_fast);
                        } else if self.opts.opt_cs_intra_barrier_exact {
                            stats::inc(&stats::c::cs_intra_barrier_exact);
                        } else if self.opts.opt_cs_intra_barrier_v2 {
                            stats::inc(&stats::c::cs_intra_barrier_v2);
                        }
                        (
                            self.binds.cs_bind_cache.uav_exact,
                            self.binds.cs_bind_cache.full_read_exact,
                            self.binds.cs_bind_cache.bounded_read_exact,
                        )
                    } else {
                        let (ue, fre, bre, resolved_gen) = if has_any_slots {
                            resource::with_resource_registry(|reg| {
                                let ue = uav_slots.is_some_and(|s| {
                                    Self::resolve_slots_with_reg(
                                        reg,
                                        s,
                                        &self.compute_root,
                                        self.heap0_cpu,
                                        &mut scratch_uav,
                                        want_spans.then_some(&mut scratch_uav_spans),
                                    )
                                });
                                let fre = read_slots.is_some_and(|s| {
                                    Self::resolve_slots_with_reg(
                                        reg,
                                        s,
                                        &self.compute_root,
                                        self.heap0_cpu,
                                        &mut scratch_read,
                                        want_spans.then_some(&mut scratch_read_spans),
                                    )
                                });
                                let bre = if fre {
                                    true
                                } else if want_bounded {
                                    Self::resolve_slots_with_reg(
                                        reg,
                                        &pso.access.bounded_read_slots,
                                        &self.compute_root,
                                        self.heap0_cpu,
                                        &mut scratch_read,
                                        want_spans.then_some(&mut scratch_read_spans),
                                    )
                                } else {
                                    false
                                };
                                (ue, fre, bre, resource::res_registry_gen())
                            })
                        } else {
                            let ue = uav_slots.is_some();
                            if ue {
                                scratch_uav.clear();
                                if want_spans {
                                    scratch_uav_spans.clear();
                                }
                            }
                            let fre = read_slots.is_some();
                            if fre || want_bounded {
                                scratch_read.clear();
                                if want_spans {
                                    scratch_read_spans.clear();
                                }
                            }
                            (ue, fre, fre || want_bounded, reg_gen)
                        };
                        if self.cs_intra_safe_active() && (ue || fre || bre) {
                            self.bump_cs_intra_stat();
                        }
                        if r_ok {
                            self.binds.cs_bind_cache.store(
                                pso_ptr,
                                want_spans,
                                want_bounded,
                                resolved_gen,
                                desc_gen,
                                root_gen,
                                heap0_base,
                                &tokens,
                                ue,
                                fre,
                                bre,
                                &scratch_uav,
                                &scratch_read,
                                &scratch_uav_spans,
                                &scratch_read_spans,
                            );
                        } else {
                            self.binds.cs_bind_cache.valid = false;
                        }
                        (ue, fre, bre)
                    };
                    self.binds.cs_bind_tokens = tokens;
                    res
                }
            } else {
                let need_reg = uav_slots.is_some_and(|s| !s.is_empty())
                    || read_slots.is_some_and(|s| !s.is_empty())
                    || (want_bounded && !pso.access.bounded_read_slots.is_empty());
                if need_reg {
                    resource::with_resource_registry(|reg| {
                        let ue = uav_slots.is_some_and(|s| {
                            Self::resolve_slots_with_reg(
                                reg,
                                s,
                                &self.compute_root,
                                self.heap0_cpu,
                                &mut scratch_uav,
                                want_spans.then_some(&mut scratch_uav_spans),
                            )
                        });
                        let fre = read_slots.is_some_and(|s| {
                            Self::resolve_slots_with_reg(
                                reg,
                                s,
                                &self.compute_root,
                                self.heap0_cpu,
                                &mut scratch_read,
                                want_spans.then_some(&mut scratch_read_spans),
                            )
                        });
                        let bre = if fre {
                            true
                        } else if want_bounded {
                            Self::resolve_slots_with_reg(
                                reg,
                                &pso.access.bounded_read_slots,
                                &self.compute_root,
                                self.heap0_cpu,
                                &mut scratch_read,
                                want_spans.then_some(&mut scratch_read_spans),
                            )
                        } else {
                            false
                        };
                        (ue, fre, bre)
                    })
                } else {
                    let ue = if uav_slots.is_some() {
                        scratch_uav.clear();
                        if want_spans {
                            scratch_uav_spans.clear();
                        }
                        true
                    } else {
                        false
                    };
                    let fre = if read_slots.is_some() {
                        scratch_read.clear();
                        if want_spans {
                            scratch_read_spans.clear();
                        }
                        true
                    } else {
                        false
                    };
                    let bre = if fre {
                        true
                    } else if want_bounded {
                        scratch_read.clear();
                        if want_spans {
                            scratch_read_spans.clear();
                        }
                        true
                    } else {
                        false
                    };
                    (ue, fre, bre)
                }
            };
            let msc_class = self.msc_class_active();
            if msc_class && pso.access.msc_refined {
                stats::inc(&stats::c::msc_access_exact);
            }
            if !self.bar.clear_tex_cs.is_empty() {
                if (self.opts.opt_stagesync_barrier_v2 || msc_class) && !pso.access.touches_textures
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
                scratch_op.clear();
                scratch_op.extend_from_slice(&scratch_uav);
                for &r in &scratch_read {
                    if !scratch_op.contains(&r) {
                        scratch_op.push(r);
                    }
                }
            }
            let use_exact_cs_ro = self.opts.opt_cs_mid_qb_exact_v2
                || self.opts.opt_sm66_cs_stage_exact
                || self.hk_v2_active();
            if self.opts.opt_sm66_cs_stage_exact
                && (pso.access.msc_refined
                    || pso.access.msc_exact_classes != pso.access.read_classes)
            {
                stats::inc(&stats::c::sm66_cs_exact);
            }
            let exact_read_only_classes = if use_exact_cs_ro {
                pso.access.msc_exact_classes
            } else {
                pso.access.read_classes
            };
            let read_classes = if use_exact_cs_ro {
                pso.access.uav_classes | exact_read_only_classes
            } else {
                pso.access.read_classes
            };
            let uav_srv_classes = if (self.opts.opt_ib_uav0_unbounded_exact
                || self.opts.opt_cs_intra_barrier_exact
                || self.opts.opt_cs_intra_barrier_v2
                || self.cs_intra_safe_active()
                || self.hk_v2_active())
                && use_exact_cs_ro
            {
                let n = pso.access.uav_classes
                    | (pso.access.uav_srv_classes
                        & (pso.access.uav_classes | exact_read_only_classes));
                if n != pso.access.uav_srv_classes {
                    if self.cs_intra_safe_active() {
                        self.bump_cs_intra_stat();
                    } else if self.hk_v2_active() {
                        self.bump_hk_stat();
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
            let hk = if self.hk_any_active() {
                self.rp_sync.cs_hk_profile
            } else {
                HandleKindProfile::OPAQUE
            };
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
                || msc_class)
                .then_some(DispatchExact {
                    uav_exact,
                    full_read_exact,
                    bounded_read_exact,
                    touches_textures: pso.access.touches_textures,
                    unbounded_srv_classes: if use_exact_cs_ro {
                        pso.access.unbounded_srv_classes & exact_read_only_classes
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
                    uav_spans: (want_spans && uav_exact).then_some(scratch_uav_spans.as_slice()),
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
            m.dispatches += 1;
            m.shader = pso.shader.key as u64;
        }
        if !self.compute_pso_bound {
            e.setComputePipelineState(&pso.pso);
            self.compute_pso_bound = true;
        }
        self.bind_table(root);
        let tg = pso.threadgroup;
        e.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize { width: x as usize, height: y as usize, depth: z as usize },
            MTLSize { width: tg[0] as usize, height: tg[1] as usize, depth: tg[2] as usize },
        );
        self.note_stage_ran(MTLStages::Dispatch);
        Ok(())
    }
}
