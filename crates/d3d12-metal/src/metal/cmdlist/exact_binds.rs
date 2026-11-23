//! Exact descriptor-table / root-argument bind resolution and caching for
//! compute, graphics, and mesh pipelines.

use super::state::{res_class, DrawSegExact};
use super::CommandList;
use crate::metal::binding::RootState;
use crate::metal::pipeline::{
    resolve_pso_hk_profile, ComputeAccess, ComputeBind, GraphicsPipeline, HandleKindProfile,
    MAX_VERTEX_SLOTS, READS_BUF, READS_TEX,
};
use crate::metal::resource::{
    push_subrange, res_registry_gen, with_buf_va_cache, with_resource_registry, ResRegistry,
    Resource, SubRange, HK_ALL, HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TYPED_BUF,
};

const ALL_BUF_HK: u32 = HK_RAW_STRUCT_BUF | HK_TYPED_BUF | HK_CBV_BUF;

/// Fallback unbounded class mask + slot-failed flag when a pipeline's pixel
/// shader has no parsed `ComputeAccess`.
#[inline]
fn ps_none_unbounded(ps_read_classes: u8, ps_writes_uav: bool) -> (u8, bool) {
    if ps_read_classes != 0 || ps_writes_uav {
        let unb = ps_read_classes | if ps_writes_uav { READS_BUF | READS_TEX } else { 0 };
        (unb, true)
    } else {
        (0, false)
    }
}

/// Mark every non-zero resource in `src` as GPU-used and append unique IDs to `dst`.
#[inline]
pub(super) fn mark_and_extend_unique(src: &[usize], dst: &mut Vec<usize>) {
    for &r in src {
        if r != 0 {
            unsafe { (*(r as *const Resource)).mark_gpu_used() };
        }
        if !dst.contains(&r) {
            dst.push(r);
        }
    }
}

impl CommandList {
    pub(super) fn resolve_stage_access_with_reg(
        reg: &ResRegistry,
        access: &ComputeAccess,
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        patch_root_invalid: bool,
        scratch: &mut Vec<usize>,
        dst: &mut Vec<usize>,
    ) -> (u8, bool) {
        if access.heap_indexed_or_unknown {
            return (READS_BUF | READS_TEX, true);
        }
        if patch_root_invalid {
            return (access.uav_classes | access.read_classes, true);
        }
        let mut unbounded = access.unbounded_srv_classes;
        let mut slot_failed = false;
        match &access.uav_slots {
            Some(slots) => {
                if !slots.is_empty() {
                    if Self::resolve_slots_with_reg(reg, slots, root, heap0, scratch, None) {
                        for &r in scratch.iter() {
                            if r != 0 && !dst.contains(&r) {
                                dst.push(r);
                            }
                        }
                    } else {
                        unbounded |= access.uav_classes;
                        slot_failed = true;
                    }
                }
            }
            None => {
                unbounded |= access.uav_classes;
            }
        }
        if access.read_slots.is_none()
            && access.unbounded_srv_classes == 0
            && access.bounded_read_slots.is_empty()
            && access.read_classes != 0
        {
            unbounded |= access.read_classes;
        } else if !access.bounded_read_slots.is_empty() {
            if Self::resolve_slots_with_reg(
                reg,
                &access.bounded_read_slots,
                root,
                heap0,
                scratch,
                None,
            ) {
                for &r in scratch.iter() {
                    if r != 0 && !dst.contains(&r) {
                        dst.push(r);
                    }
                }
            } else {
                unbounded |= access.read_classes;
                slot_failed = true;
            }
        }
        (unbounded, slot_failed)
    }

    pub(super) fn resolve_uav_write_with_reg(
        reg: &ResRegistry,
        access: &ComputeAccess,
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        scratch: &mut Vec<usize>,
        dst: &mut Vec<usize>,
    ) -> bool {
        if access.heap_indexed_or_unknown {
            return true;
        }
        match &access.uav_slots {
            Some(slots) => {
                if slots.is_empty() {
                    false
                } else if Self::resolve_slots_with_reg(reg, slots, root, heap0, scratch, None) {
                    for &r in scratch.iter() {
                        if r != 0 && !dst.contains(&r) {
                            dst.push(r);
                        }
                    }
                    false
                } else {
                    true
                }
            }
            None => true,
        }
    }

    pub(super) fn stage_access_overlaps_hazard(
        reg: &ResRegistry,
        access: &ComputeAccess,
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        hazard_res: &[usize],
        hazard_classes: u8,
        scratch: &mut Vec<usize>,
    ) -> bool {
        if access.heap_indexed_or_unknown {
            return true;
        }
        if (access.unbounded_srv_classes & hazard_classes) != 0 {
            return true;
        }
        match &access.uav_slots {
            Some(slots) => {
                if !slots.is_empty() {
                    if Self::resolve_slots_with_reg(reg, slots, root, heap0, scratch, None) {
                        if scratch.iter().any(|r| *r != 0 && hazard_res.contains(r)) {
                            return true;
                        }
                    } else if (access.uav_classes & hazard_classes) != 0 {
                        return true;
                    }
                }
            }
            None => {
                if (access.uav_classes & hazard_classes) != 0 {
                    return true;
                }
            }
        }
        if access.read_slots.is_none()
            && access.unbounded_srv_classes == 0
            && access.bounded_read_slots.is_empty()
            && (access.read_classes & hazard_classes) != 0
        {
            return true;
        } else if !access.bounded_read_slots.is_empty() {
            if Self::resolve_slots_with_reg(
                reg,
                &access.bounded_read_slots,
                root,
                heap0,
                scratch,
                None,
            ) {
                if scratch.iter().any(|r| *r != 0 && hazard_res.contains(r)) {
                    return true;
                }
            } else if (access.read_classes & hazard_classes) != 0 {
                return true;
            }
        }
        false
    }

    fn extract_stage_bind_tokens(
        access: &ComputeAccess,
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        include_reads: bool,
        tokens: &mut Vec<u64>,
    ) -> bool {
        if access.heap_indexed_or_unknown {
            return true;
        }
        if let Some(slots) = &access.uav_slots {
            if !slots.is_empty() && !Self::extract_compute_slot_tokens(slots, root, heap0, tokens) {
                return false;
            }
        }
        if include_reads
            && !access.bounded_read_slots.is_empty()
            && !Self::extract_compute_slot_tokens(&access.bounded_read_slots, root, heap0, tokens)
        {
            return false;
        }
        true
    }

    /// Compute the effective `(pre_unb_hk, ps_unb_hk)` masks from `self.binds.gfx_bind_cache`
    /// and pipeline stage handle-kind profiles (shared across draw-setup, encoder commit,
    /// and pre-pass segment snapshotting).
    #[inline]
    fn gfx_cached_unb_hk(&self, p: &GraphicsPipeline) -> (u32, u32, u32, u32) {
        let (pre_hk, ps_hk, mut pre_unb_hk, mut ps_unb_hk) = self.gfx_pso_stage_hk(p);
        let cache = &self.binds.gfx_bind_cache;
        if cache.pre_slot_failed {
            pre_unb_hk |= pre_hk;
        }
        if cache.pre_va_failed {
            pre_unb_hk |= ALL_BUF_HK;
        }
        if cache.ps_slot_failed {
            ps_unb_hk |= ps_hk;
        }
        (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk)
    }

    /// Resolve a graphics draw's pre-raster, pixel, and UAV-write bindings into
    /// `self.binds.gfx_bind_cache`, taking ZERO `res_registry` locks on a token-cache hit
    /// and at most ONE `res_registry` read lock on a cache miss.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_gfx_binds_cached(
        &mut self,
        p: &GraphicsPipeline,
        indexed: bool,
        indirect: bool,
        patch_root_invalid: bool,
        patch_vb: bool,
        patch_ib: bool,
        indirect_vas: Option<&[u64]>,
        do_gfx: bool,
        do_uav: bool,
    ) {
        let pso_ptr = p as *const GraphicsPipeline as usize;
        let do_uav_eff = do_uav && !patch_root_invalid;
        let flags: u8 = (indexed as u8)
            | ((indirect as u8) << 1)
            | ((patch_root_invalid as u8) << 2)
            | ((patch_vb as u8) << 3)
            | ((patch_ib as u8) << 4)
            | ((do_gfx as u8) << 5)
            | ((do_uav_eff as u8) << 6);
        let reg_gen = res_registry_gen();
        let heap0_base = self.heaps[0];

        let mut tokens = std::mem::take(&mut self.binds.gfx_bind_tokens);
        tokens.clear();
        let mut tokens_ok = true;
        if !patch_root_invalid {
            if do_gfx || (do_uav_eff && p.vs_writes_uav) {
                tokens_ok &= Self::extract_stage_bind_tokens(
                    &p.vs_access,
                    &self.gfx_root,
                    self.heap0_cpu,
                    do_gfx,
                    &mut tokens,
                );
            }
            if tokens_ok {
                if let Some(ps_acc) = &p.ps_access {
                    if do_gfx || (do_uav_eff && p.ps_writes_uav) {
                        tokens_ok &= Self::extract_stage_bind_tokens(
                            ps_acc,
                            &self.gfx_root,
                            self.heap0_cpu,
                            do_gfx,
                            &mut tokens,
                        );
                    }
                }
            }
        }
        if tokens_ok && do_gfx {
            if p.vertex_slots != 0 && !patch_vb {
                let mut vm = p.vertex_slots;
                while vm != 0 {
                    let slot = vm.trailing_zeros() as usize;
                    vm &= vm - 1;
                    if slot < MAX_VERTEX_SLOTS as usize {
                        tokens.push(self.gfx.vbs[slot].0);
                    }
                }
            }
            if indexed && !patch_ib {
                tokens.push(self.gfx.ib.map_or(0, |(va, _, _)| va));
            }
            if indirect {
                match indirect_vas {
                    Some(vas) => {
                        tokens.push(vas.len() as u64);
                        tokens.extend_from_slice(vas);
                    }
                    None => tokens.push(u64::MAX),
                }
            }
            tokens.push(self.predication.map_or(0, |pr| pr.gpu_va));
        }

        if tokens_ok
            && self
                .binds
                .gfx_bind_cache
                .matches(pso_ptr, flags, reg_gen, heap0_base, &tokens)
        {
            self.binds.gfx_bind_tokens = tokens;
            return;
        }

        let mut scratch = std::mem::take(&mut self.ib.ib_scratch_op);
        let gfx_root = &self.gfx_root;
        let heap0 = self.heap0_cpu;
        let vbs = self.gfx.vbs;
        let ib = self.gfx.ib;
        let pred = self.predication;
        let cache = &mut self.binds.gfx_bind_cache;
        cache.pre_exact.clear();
        cache.ps_exact.clear();
        cache.uav_exact.clear();
        let mut pre_unb = 0u8;
        let mut ps_unb = 0u8;
        let mut uav_unb = patch_root_invalid;
        let mut pre_slot_failed = false;
        let mut pre_va_failed = false;
        let mut ps_slot_failed = false;

        let has_slots = !patch_root_invalid
            && ((do_gfx
                && (p.vs_access.uav_slots.as_ref().is_some_and(|s| !s.is_empty())
                    || !p.vs_access.bounded_read_slots.is_empty()
                    || p.ps_access.as_ref().is_some_and(|a| {
                        a.uav_slots.as_ref().is_some_and(|s| !s.is_empty())
                            || !a.bounded_read_slots.is_empty()
                    })))
                || (do_uav_eff
                    && ((p.vs_writes_uav
                        && p.vs_access.uav_slots.as_ref().is_some_and(|s| !s.is_empty()))
                        || (p.ps_writes_uav
                            && p.ps_access.as_ref().is_some_and(|a| {
                                a.uav_slots.as_ref().is_some_and(|s| !s.is_empty())
                            })))));
        let has_vas = do_gfx
            && ((p.vertex_slots != 0 && !patch_vb && {
                let mut vm = p.vertex_slots;
                let mut any = false;
                while vm != 0 {
                    let slot = vm.trailing_zeros() as usize;
                    vm &= vm - 1;
                    if slot < MAX_VERTEX_SLOTS as usize && vbs[slot].0 != 0 {
                        any = true;
                        break;
                    }
                }
                any
            }) || (indexed && !patch_ib && ib.is_some_and(|(va, _, _)| va != 0))
                || (indirect && indirect_vas.is_some_and(|vas| vas.iter().any(|&va| va != 0)))
                || pred.is_some_and(|pr| pr.gpu_va != 0));

        let resolved_gen = if has_slots || has_vas {
            with_resource_registry(|reg| {
                if do_gfx {
                    let (pu, pf) = Self::resolve_stage_access_with_reg(
                        reg,
                        &p.vs_access,
                        gfx_root,
                        heap0,
                        patch_root_invalid,
                        &mut scratch,
                        &mut cache.pre_exact,
                    );
                    pre_unb = pu;
                    pre_slot_failed = pf;
                    if p.vertex_slots != 0 {
                        if patch_vb {
                            pre_unb |= READS_BUF;
                            pre_va_failed = true;
                        } else {
                            let mut vm = p.vertex_slots;
                            while vm != 0 {
                                let slot = vm.trailing_zeros() as usize;
                                vm &= vm - 1;
                                if slot < MAX_VERTEX_SLOTS as usize {
                                    let (va, _, _) = vbs[slot];
                                    if va != 0 && !reg.push_buffer_va(va, &mut cache.pre_exact) {
                                        pre_unb |= READS_BUF;
                                        pre_va_failed = true;
                                    }
                                }
                            }
                        }
                    }
                    if indexed {
                        if patch_ib {
                            pre_unb |= READS_BUF;
                            pre_va_failed = true;
                        } else if let Some((va, _, _)) = ib {
                            if va != 0 && !reg.push_buffer_va(va, &mut cache.pre_exact) {
                                pre_unb |= READS_BUF;
                                pre_va_failed = true;
                            }
                        }
                    }
                    if indirect {
                        if let Some(vas) = indirect_vas {
                            for &va in vas {
                                if va != 0 && !reg.push_buffer_va(va, &mut cache.pre_exact) {
                                    pre_unb |= READS_BUF;
                                    pre_va_failed = true;
                                }
                            }
                        } else {
                            pre_unb |= READS_BUF;
                            pre_va_failed = true;
                        }
                    }
                    if let Some(pr) = pred {
                        if pr.gpu_va != 0 && !reg.push_buffer_va(pr.gpu_va, &mut cache.pre_exact) {
                            pre_unb |= READS_BUF;
                            pre_va_failed = true;
                        }
                    }
                    match &p.ps_access {
                        Some(ps_acc) => {
                            let (psu, psf) = Self::resolve_stage_access_with_reg(
                                reg,
                                ps_acc,
                                gfx_root,
                                heap0,
                                patch_root_invalid,
                                &mut scratch,
                                &mut cache.ps_exact,
                            );
                            ps_unb = psu;
                            ps_slot_failed = psf;
                        }
                        None => (ps_unb, ps_slot_failed) = ps_none_unbounded(p.ps_read_classes, p.ps_writes_uav),
                    }
                } else {
                    pre_slot_failed = true;
                    ps_slot_failed = true;
                }
                if do_uav_eff {
                    if p.vs_writes_uav
                        && Self::resolve_uav_write_with_reg(
                            reg,
                            &p.vs_access,
                            gfx_root,
                            heap0,
                            &mut scratch,
                            &mut cache.uav_exact,
                        )
                    {
                        uav_unb = true;
                    }
                    if p.ps_writes_uav {
                        match &p.ps_access {
                            Some(ps_acc) => {
                                if Self::resolve_uav_write_with_reg(
                                    reg,
                                    ps_acc,
                                    gfx_root,
                                    heap0,
                                    &mut scratch,
                                    &mut cache.uav_exact,
                                ) {
                                    uav_unb = true;
                                }
                            }
                            None => uav_unb = true,
                        }
                    }
                }
                res_registry_gen()
            })
        } else {
            // Fast path: no root/table slots or buffer VAs require registry lookup.
            if do_gfx {
                let stage_no_slot_unb = |acc: &ComputeAccess| -> (u8, bool) {
                    if acc.heap_indexed_or_unknown {
                        return (READS_BUF | READS_TEX, true);
                    }
                    if patch_root_invalid {
                        return (acc.uav_classes | acc.read_classes, true);
                    }
                    let mut u = acc.unbounded_srv_classes;
                    if acc.uav_slots.is_none() {
                        u |= acc.uav_classes;
                    }
                    if acc.read_slots.is_none()
                        && acc.unbounded_srv_classes == 0
                        && acc.bounded_read_slots.is_empty()
                        && acc.read_classes != 0
                    {
                        u |= acc.read_classes;
                    }
                    (u, false)
                };
                let (pu, pf) = stage_no_slot_unb(&p.vs_access);
                pre_unb = pu;
                pre_slot_failed = pf;
                if (p.vertex_slots != 0 && patch_vb)
                    || (indexed && patch_ib)
                    || (indirect && indirect_vas.is_none())
                {
                    pre_unb |= READS_BUF;
                    pre_va_failed = true;
                }
                match &p.ps_access {
                    Some(ps_acc) => {
                        let (psu, psf) = stage_no_slot_unb(ps_acc);
                        ps_unb = psu;
                        ps_slot_failed = psf;
                    }
                    None => (ps_unb, ps_slot_failed) = ps_none_unbounded(p.ps_read_classes, p.ps_writes_uav),
                }
            } else {
                pre_slot_failed = true;
                ps_slot_failed = true;
            }
            if do_uav_eff {
                let stage_uav_unb = |acc: &ComputeAccess| -> bool {
                    acc.heap_indexed_or_unknown || acc.uav_slots.is_none()
                };
                if p.vs_writes_uav && stage_uav_unb(&p.vs_access) {
                    uav_unb = true;
                }
                if p.ps_writes_uav {
                    match &p.ps_access {
                        Some(ps_acc) => {
                            if stage_uav_unb(ps_acc) {
                                uav_unb = true;
                            }
                        }
                        None => uav_unb = true,
                    }
                }
            }
            reg_gen
        };

        cache.pre_unb = pre_unb;
        cache.ps_unb = ps_unb;
        cache.uav_unb = uav_unb;
        cache.pre_slot_failed = pre_slot_failed;
        cache.pre_va_failed = pre_va_failed;
        cache.ps_slot_failed = ps_slot_failed;
        if tokens_ok {
            cache.valid = true;
            cache.pso_ptr = pso_ptr;
            cache.flags = flags;
            cache.reg_gen = resolved_gen;
            cache.heap0_base = heap0_base;
            std::mem::swap(&mut cache.tokens, &mut tokens);
        } else {
            cache.valid = false;
        }
        self.binds.gfx_bind_tokens = tokens;
        self.ib.ib_scratch_op = scratch;
    }

    #[inline]
    pub(super) fn gfx_cached_overlaps_pass_hazard(&self, patch_root_invalid: bool) -> bool {
        if self.rp_sync.pass_uav_hazard_all || patch_root_invalid {
            return true;
        }
        if self.rp_sync.pass_uav_hazard_res.is_empty() {
            return false;
        }
        let mut hazard_classes = 0u8;
        for &r in &self.rp_sync.pass_uav_hazard_res {
            hazard_classes |= res_class(r);
        }
        let cache = &self.binds.gfx_bind_cache;
        if ((cache.pre_unb | cache.ps_unb) & hazard_classes) != 0 {
            return true;
        }
        cache
            .pre_exact
            .iter()
            .chain(cache.ps_exact.iter())
            .any(|r| self.rp_sync.pass_uav_hazard_res.contains(r))
    }

    #[inline]
    pub(super) fn apply_gfx_bind_cache_to_cur_draw(&mut self, p: &GraphicsPipeline) {
        let (_, _, pre_unb_hk, ps_unb_hk) = self.gfx_cached_unb_hk(p);
        let cache = &self.binds.gfx_bind_cache;
        self.rp_sync.cur_draw_pre_exact.clear();
        self.rp_sync
            .cur_draw_pre_exact
            .extend_from_slice(&cache.pre_exact);
        self.rp_sync.cur_draw_ps_exact.clear();
        self.rp_sync
            .cur_draw_ps_exact
            .extend_from_slice(&cache.ps_exact);
        self.rp_sync.cur_draw_pre_unb = cache.pre_unb;
        self.rp_sync.cur_draw_pre_unb_hk = pre_unb_hk;
        self.rp_sync.cur_draw_ps_unb = cache.ps_unb;
        self.rp_sync.cur_draw_ps_unb_hk = ps_unb_hk;
        self.rp_sync.cur_draw_exact_valid = true;
    }

    #[inline]
    pub(super) fn commit_gfx_bind_cache_to_enc(&mut self, p: &GraphicsPipeline) {
        let (_, _, pre_unb_hk, ps_unb_hk) = self.gfx_cached_unb_hk(p);
        let cache = &self.binds.gfx_bind_cache;
        self.rp_sync.enc_pre_raster_unbounded_classes |= cache.pre_unb;
        self.rp_sync.enc_pre_raster_unb_hk |= pre_unb_hk;
        mark_and_extend_unique(&cache.pre_exact, &mut self.rp_sync.enc_pre_raster_exact_res);
        self.rp_sync.enc_ps_unbounded_classes |= cache.ps_unb;
        self.rp_sync.enc_ps_unb_hk |= ps_unb_hk;
        mark_and_extend_unique(&cache.ps_exact, &mut self.rp_sync.enc_ps_exact_res);
    }

    #[inline]
    pub(super) fn commit_gfx_bind_cache_uav_writes(&mut self, patch_root_invalid: bool) {
        if patch_root_invalid
            || self.rp_sync.enc_uav_write_unbounded
            || self.binds.gfx_bind_cache.uav_unb
        {
            self.rp_sync.enc_uav_write_unbounded = true;
            return;
        }
        for &r in &self.binds.gfx_bind_cache.uav_exact {
            if r != 0 && !self.rp_sync.enc_uav_write_exact_res.contains(&r) {
                self.rp_sync.enc_uav_write_exact_res.push(r);
            }
        }
    }

    pub(super) fn mesh_uav_overlaps_pass_hazard(
        &mut self,
        p: &crate::mesh::MeshPipeline,
        indirect: bool,
    ) -> bool {
        if self.rp_sync.pass_uav_hazard_all || indirect {
            return true;
        }
        if self.rp_sync.pass_uav_hazard_res.is_empty() {
            return false;
        }
        let mut hazard_classes = 0u8;
        for &r in &self.rp_sync.pass_uav_hazard_res {
            hazard_classes |= res_class(r);
        }
        let mut scratch = std::mem::take(&mut self.ib.ib_scratch_op);
        let gfx_root = &self.gfx_root;
        let heap0 = self.heap0_cpu;
        let hazard_res = &self.rp_sync.pass_uav_hazard_res;
        let hit = with_resource_registry(|reg| {
            if Self::stage_access_overlaps_hazard(
                reg,
                &p.ms_access,
                gfx_root,
                heap0,
                hazard_res,
                hazard_classes,
                &mut scratch,
            ) {
                return true;
            }
            if let Some(as_acc) = &p.as_access {
                if Self::stage_access_overlaps_hazard(
                    reg,
                    as_acc,
                    gfx_root,
                    heap0,
                    hazard_res,
                    hazard_classes,
                    &mut scratch,
                ) {
                    return true;
                }
            }
            match &p.ps_access {
                Some(ps_acc) => {
                    if Self::stage_access_overlaps_hazard(
                        reg,
                        ps_acc,
                        gfx_root,
                        heap0,
                        hazard_res,
                        hazard_classes,
                        &mut scratch,
                    ) {
                        return true;
                    }
                }
                None => {
                    if p.ps_writes_uav || (p.ps_read_classes & hazard_classes) != 0 {
                        return true;
                    }
                }
            }
            false
        });
        self.ib.ib_scratch_op = scratch;
        hit
    }

    pub(super) fn record_mesh_uav_write_exact(
        &mut self,
        p: &crate::mesh::MeshPipeline,
        indirect: bool,
    ) {
        if indirect || self.rp_sync.enc_uav_write_unbounded {
            self.rp_sync.enc_uav_write_unbounded = true;
            return;
        }
        let mut scratch = std::mem::take(&mut self.ib.ib_scratch_op);
        let gfx_root = &self.gfx_root;
        let heap0 = self.heap0_cpu;
        let dst = &mut self.rp_sync.enc_uav_write_exact_res;
        let unb = with_resource_registry(|reg| {
            let mut u = false;
            if p.ms_as_writes_uav {
                if Self::resolve_uav_write_with_reg(
                    reg,
                    &p.ms_access,
                    gfx_root,
                    heap0,
                    &mut scratch,
                    dst,
                ) {
                    u = true;
                }
                if let Some(as_acc) = &p.as_access {
                    if Self::resolve_uav_write_with_reg(
                        reg,
                        as_acc,
                        gfx_root,
                        heap0,
                        &mut scratch,
                        dst,
                    ) {
                        u = true;
                    }
                }
            }
            if p.ps_writes_uav {
                match &p.ps_access {
                    Some(ps_acc) => {
                        if Self::resolve_uav_write_with_reg(
                            reg,
                            ps_acc,
                            gfx_root,
                            heap0,
                            &mut scratch,
                            dst,
                        ) {
                            u = true;
                        }
                    }
                    None => u = true,
                }
            }
            u
        });
        if unb {
            self.rp_sync.enc_uav_write_unbounded = true;
        }
        self.ib.ib_scratch_op = scratch;
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn snapshot_gfx_seg_exact(
        &mut self,
        p: &GraphicsPipeline,
        indexed: bool,
        indirect: bool,
        patch_root_invalid: bool,
        patch_vb: bool,
        patch_ib: bool,
        indirect_vas: Option<&[u64]>,
    ) -> DrawSegExact {
        let writes_uav = self.gfx_pso_writes_uav(p);
        let eff_reads = self.effective_gfx_pre_raster_reads(p, indexed, indirect);
        let srv_uav_pre_reads = self.gfx_pso_srv_uav_pre_raster_reads(p);
        let do_gfx = self.opts.opt_gfx_desc_table_exact;
        let do_uav = writes_uav && self.opts.opt_rp_split_exact_keep;
        let (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk, pre_unb, pre_exact, ps_unb, ps_exact, uav_unb, uav_exact) =
            if do_gfx || do_uav {
                self.resolve_gfx_binds_cached(
                    p,
                    indexed,
                    indirect,
                    patch_root_invalid,
                    patch_vb,
                    patch_ib,
                    indirect_vas,
                    do_gfx,
                    do_uav,
                );
                let (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk) = self.gfx_cached_unb_hk(p);
                let cache = &self.binds.gfx_bind_cache;
                (
                    pre_hk,
                    ps_hk,
                    pre_unb_hk,
                    ps_unb_hk,
                    cache.pre_unb,
                    cache.pre_exact.clone(),
                    cache.ps_unb,
                    cache.ps_exact.clone(),
                    cache.uav_unb,
                    cache.uav_exact.clone(),
                )
            } else {
                let (pre_hk, ps_hk, _, _) = self.gfx_pso_stage_hk(p);
                (
                    pre_hk,
                    ps_hk,
                    pre_hk,
                    ps_hk,
                    0,
                    Vec::new(),
                    0,
                    Vec::new(),
                    patch_root_invalid,
                    Vec::new(),
                )
            };
        DrawSegExact {
            eff_reads,
            srv_uav_pre_reads,
            ps_reads: p.ps_read_classes,
            ps_srv_uav_reads: p.ps_srv_uav_classes,
            writes_uav,
            vs_writes_uav: p.vs_writes_uav,
            ps_writes_uav: p.ps_writes_uav,
            uses_uav: p.uses_uav,
            pre_unb,
            pre_exact,
            ps_unb,
            ps_exact,
            uav_unb,
            uav_exact,
            pre_hk,
            ps_hk,
            pre_unb_hk,
            ps_unb_hk,
        }
    }

    #[inline]
    fn stage_profile_hk_and_unb(
        embedded: HandleKindProfile,
        sh: &dxil_metal::CompiledShader,
        acc: &ComputeAccess,
    ) -> (u32, u32) {
        let p = resolve_pso_hk_profile(embedded, sh, acc);
        let full = p.read_kinds | p.uav_srv_kinds | p.uav_write_kinds;
        let unb = if acc.heap_indexed_or_unknown {
            full
        } else {
            let mut u = p.unbounded_srv_kinds
                | if (acc.unbounded_srv_classes & READS_BUF) != 0 {
                    p.read_kinds & HK_CBV_BUF
                } else {
                    0
                };
            if acc.uav_slots.is_none() {
                u |= p.uav_srv_kinds | p.uav_write_kinds;
            }
            if acc.read_slots.is_none()
                && acc.unbounded_srv_classes == 0
                && acc.bounded_read_slots.is_empty()
                && acc.read_classes != 0
            {
                u |= p.read_kinds;
            }
            u
        };
        (full, unb)
    }

    #[inline]
    pub(super) fn gfx_pso_stage_hk(&self, p: &GraphicsPipeline) -> (u32, u32, u32, u32) {
        if !self.hk_any_active() {
            return (0, 0, 0, 0);
        }
        let (pre_hk, pre_unb_hk) =
            Self::stage_profile_hk_and_unb(p.vs_hk_profile, &p.vs, &p.vs_access);
        let (ps_hk, ps_unb_hk) = match (&p.ps, &p.ps_access) {
            (Some(ps), Some(acc)) => {
                let emb = p
                    .ps_hk_profile
                    .unwrap_or_else(|| HandleKindProfile::OPAQUE.intersect_access(acc));
                Self::stage_profile_hk_and_unb(emb, ps, acc)
            }
            (Some(_), None) => (HK_ALL, HK_ALL),
            (None, _) => (0, 0),
        };
        (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk)
    }

    #[inline]
    pub(super) fn mesh_pso_stage_hk(&self, p: &crate::mesh::MeshPipeline) -> (u32, u32, u32, u32) {
        if !self.hk_any_active() {
            return (0, 0, 0, 0);
        }
        let (mut pre_hk, mut pre_unb_hk) =
            Self::stage_profile_hk_and_unb(p.ms_hk_profile, &p.mesh, &p.ms_access);
        if let Some(as_sh) = &p.amplification {
            let (ah, auh) = match &p.as_access {
                Some(acc) => {
                    let emb = p
                        .as_hk_profile
                        .unwrap_or_else(|| HandleKindProfile::OPAQUE.intersect_access(acc));
                    Self::stage_profile_hk_and_unb(emb, as_sh, acc)
                }
                None => (HK_ALL, HK_ALL),
            };
            pre_hk |= ah;
            pre_unb_hk |= auh;
        }
        let (ps_hk, ps_unb_hk) = match (&p.pixel, &p.ps_access) {
            (Some(ps), Some(acc)) => {
                let emb = p
                    .ps_hk_profile
                    .unwrap_or_else(|| HandleKindProfile::OPAQUE.intersect_access(acc));
                Self::stage_profile_hk_and_unb(emb, ps, acc)
            }
            (Some(_), None) => (HK_ALL, HK_ALL),
            (None, _) => (0, 0),
        };
        (pre_hk, ps_hk, pre_unb_hk, ps_unb_hk)
    }

    /// Shared mesh-pipeline stage bind resolver (deduplicates `resolve_cur_draw_mesh_exact`
    /// and `record_mesh_stage_exact_binds`).
    #[allow(clippy::too_many_arguments)]
    fn resolve_mesh_stage_exact_into(
        p: &crate::mesh::MeshPipeline,
        indirect: bool,
        stage_hk: (u32, u32, u32, u32),
        gfx_root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        pred: Option<super::state::Predication>,
        scratch: &mut Vec<usize>,
        exact_pre: &mut Vec<usize>,
        exact_ps: &mut Vec<usize>,
    ) -> (u8, u32, u8, u32) {
        let (pre_hk, ps_hk, mut pre_unb_hk, mut ps_unb_hk) = stage_hk;
        let mut pre_slot_failed = false;
        let mut pre_va_failed = indirect;
        let mut ps_slot_failed = false;
        let (pre_unb, ps_unb) = with_resource_registry(|reg| {
            let (mut pre_u, mf) = Self::resolve_stage_access_with_reg(
                reg,
                &p.ms_access,
                gfx_root,
                heap0,
                false,
                scratch,
                exact_pre,
            );
            pre_slot_failed |= mf;
            if let Some(as_acc) = &p.as_access {
                let (au, af) = Self::resolve_stage_access_with_reg(
                    reg,
                    as_acc,
                    gfx_root,
                    heap0,
                    false,
                    scratch,
                    exact_pre,
                );
                pre_u |= au;
                pre_slot_failed |= af;
            }
            if indirect {
                pre_u |= READS_BUF;
            }
            if let Some(pr) = pred {
                if pr.gpu_va != 0 && !reg.push_buffer_va(pr.gpu_va, exact_pre) {
                    pre_u |= READS_BUF;
                    pre_va_failed = true;
                }
            }
            let ps_u = match &p.ps_access {
                Some(ps_acc) => {
                    let (psu, psf) = Self::resolve_stage_access_with_reg(
                        reg,
                        ps_acc,
                        gfx_root,
                        heap0,
                        false,
                        scratch,
                        exact_ps,
                    );
                    ps_slot_failed = psf;
                    psu
                }
                None => {
                    let (u, failed) = ps_none_unbounded(p.ps_read_classes, p.ps_writes_uav);
                    ps_slot_failed = failed;
                    u
                }
            };
            (pre_u, ps_u)
        });
        if pre_slot_failed {
            pre_unb_hk |= pre_hk;
        }
        if pre_va_failed {
            pre_unb_hk |= ALL_BUF_HK;
        }
        if ps_slot_failed {
            ps_unb_hk |= ps_hk;
        }
        (pre_unb, pre_unb_hk, ps_unb, ps_unb_hk)
    }

    pub(super) fn resolve_cur_draw_mesh_exact(
        &mut self,
        p: &crate::mesh::MeshPipeline,
        indirect: bool,
    ) {
        let stage_hk = self.mesh_pso_stage_hk(p);
        self.rp_sync.cur_draw_pre_exact.clear();
        self.rp_sync.cur_draw_ps_exact.clear();
        let mut scratch = std::mem::take(&mut self.ib.ib_scratch_op);
        let (pre_unb, pre_unb_hk, ps_unb, ps_unb_hk) = Self::resolve_mesh_stage_exact_into(
            p,
            indirect,
            stage_hk,
            &self.gfx_root,
            self.heap0_cpu,
            self.predication,
            &mut scratch,
            &mut self.rp_sync.cur_draw_pre_exact,
            &mut self.rp_sync.cur_draw_ps_exact,
        );
        self.rp_sync.cur_draw_pre_unb = pre_unb;
        self.rp_sync.cur_draw_pre_unb_hk = pre_unb_hk;
        self.rp_sync.cur_draw_ps_unb = ps_unb;
        self.rp_sync.cur_draw_ps_unb_hk = ps_unb_hk;
        self.rp_sync.cur_draw_exact_valid = true;
        self.ib.ib_scratch_op = scratch;
    }

    #[inline]
    pub(super) fn commit_cur_draw_exact_to_enc(&mut self) {
        self.rp_sync.enc_pre_raster_unbounded_classes |= self.rp_sync.cur_draw_pre_unb;
        self.rp_sync.enc_pre_raster_unb_hk |= self.rp_sync.cur_draw_pre_unb_hk;
        mark_and_extend_unique(
            &self.rp_sync.cur_draw_pre_exact,
            &mut self.rp_sync.enc_pre_raster_exact_res,
        );
        self.rp_sync.enc_ps_unbounded_classes |= self.rp_sync.cur_draw_ps_unb;
        self.rp_sync.enc_ps_unb_hk |= self.rp_sync.cur_draw_ps_unb_hk;
        mark_and_extend_unique(
            &self.rp_sync.cur_draw_ps_exact,
            &mut self.rp_sync.enc_ps_exact_res,
        );
    }

    pub(super) fn record_mesh_stage_exact_binds(
        &mut self,
        p: &crate::mesh::MeshPipeline,
        indirect: bool,
    ) {
        let stage_hk = self.mesh_pso_stage_hk(p);
        let mut scratch = std::mem::take(&mut self.ib.ib_scratch_op);
        let pre_start = self.rp_sync.enc_pre_raster_exact_res.len();
        let ps_start = self.rp_sync.enc_ps_exact_res.len();
        let (pre_unb, pre_unb_hk, ps_unb, ps_unb_hk) = Self::resolve_mesh_stage_exact_into(
            p,
            indirect,
            stage_hk,
            &self.gfx_root,
            self.heap0_cpu,
            self.predication,
            &mut scratch,
            &mut self.rp_sync.enc_pre_raster_exact_res,
            &mut self.rp_sync.enc_ps_exact_res,
        );
        for &r in &self.rp_sync.enc_pre_raster_exact_res[pre_start..] {
            if r != 0 {
                unsafe { (*(r as *const Resource)).mark_gpu_used() };
            }
        }
        for &r in &self.rp_sync.enc_ps_exact_res[ps_start..] {
            if r != 0 {
                unsafe { (*(r as *const Resource)).mark_gpu_used() };
            }
        }
        self.rp_sync.enc_pre_raster_unbounded_classes |= pre_unb;
        self.rp_sync.enc_pre_raster_unb_hk |= pre_unb_hk;
        self.rp_sync.enc_ps_unbounded_classes |= ps_unb;
        self.rp_sync.enc_ps_unb_hk |= ps_unb_hk;
        self.ib.ib_scratch_op = scratch;
    }

    #[allow(dead_code)]
    pub(super) fn resolve_compute_binds(
        slots: &[ComputeBind],
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        out: &mut Vec<usize>,
    ) -> bool {
        Self::resolve_compute_binds_spans(slots, root, heap0, out, None)
    }

    pub(super) fn resolve_slots_with_reg(
        reg: &ResRegistry,
        slots: &[ComputeBind],
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        out: &mut Vec<usize>,
        mut spans: Option<&mut Vec<SubRange>>,
    ) -> bool {
        out.clear();
        if let Some(sp) = spans.as_deref_mut() {
            sp.clear();
        }
        if slots.is_empty() {
            return true;
        }
        with_buf_va_cache(|cache| {
            for &slot in slots {
                match slot {
                    ComputeBind::RootVa { param } => {
                        let Some(va) = root.param_u64(param as usize) else {
                            return false;
                        };
                        if va != 0 {
                            let ok = match spans.as_deref_mut() {
                                Some(sp) => reg.push_buffer_span_cached(cache, va, 0, out, sp),
                                None => reg.push_buffer_va_cached(cache, va, out),
                            };
                            if !ok {
                                return false;
                            }
                        }
                    }
                    ComputeBind::TableEntry { param, byte_offset } => {
                        let Some(base_gpu) = root.param_u64(param as usize) else {
                            return false;
                        };
                        if base_gpu == 0 {
                            continue;
                        }
                        let Some((hbase, hlen, hcpu)) = heap0 else {
                            return false;
                        };
                        let Some(entry_gpu) = base_gpu.checked_add(byte_offset) else {
                            return false;
                        };
                        if entry_gpu < hbase || entry_gpu.saturating_add(24) > hbase.saturating_add(hlen) {
                            return false;
                        }
                        let entry_off = (entry_gpu - hbase) as usize;
                        let (gpu_va, tex_id, meta) = unsafe {
                            let p = (hcpu + entry_off) as *const u64;
                            (
                                p.read_unaligned(),
                                p.add(1).read_unaligned(),
                                p.add(2).read_unaligned(),
                            )
                        };
                        if gpu_va != 0 {
                            let found = match spans.as_deref_mut() {
                                Some(sp) => reg.push_buffer_span_cached(
                                    cache,
                                    gpu_va,
                                    meta & 0xffff_ffff,
                                    out,
                                    sp,
                                ),
                                None => reg.push_buffer_va_cached(cache, gpu_va, out),
                            };
                            if !found {
                                return false;
                            }
                        } else if tex_id != 0 {
                            let Some(rk) = reg.lookup_texture_id(tex_id) else {
                                return false;
                            };
                            if !out.contains(&rk) {
                                out.push(rk);
                            }
                            if let Some(sp) = spans.as_deref_mut() {
                                push_subrange(sp, SubRange::whole(rk));
                            }
                        }
                    }
                }
            }
            true
        })
    }

    pub(super) fn extract_compute_slot_tokens(
        slots: &[ComputeBind],
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        tokens: &mut Vec<u64>,
    ) -> bool {
        for &slot in slots {
            match slot {
                ComputeBind::RootVa { param } => {
                    let Some(va) = root.param_u64(param as usize) else {
                        return false;
                    };
                    tokens.push(va);
                }
                ComputeBind::TableEntry { param, byte_offset } => {
                    let Some(base_gpu) = root.param_u64(param as usize) else {
                        return false;
                    };
                    if base_gpu == 0 {
                        tokens.extend_from_slice(&[0, 0, 0]);
                        continue;
                    }
                    let Some((hbase, hlen, hcpu)) = heap0 else {
                        return false;
                    };
                    let Some(entry_gpu) = base_gpu.checked_add(byte_offset) else {
                        return false;
                    };
                    if entry_gpu < hbase || entry_gpu.saturating_add(24) > hbase.saturating_add(hlen) {
                        return false;
                    }
                    let entry_off = (entry_gpu - hbase) as usize;
                    let (gpu_va, tex_id, meta) = unsafe {
                        let p = (hcpu + entry_off) as *const u64;
                        (
                            p.read_unaligned(),
                            p.add(1).read_unaligned(),
                            p.add(2).read_unaligned(),
                        )
                    };
                    tokens.extend_from_slice(&[gpu_va, tex_id, meta & 0xffff_ffff]);
                }
            }
        }
        true
    }

    pub(super) fn resolve_compute_binds_spans(
        slots: &[ComputeBind],
        root: &RootState,
        heap0: Option<(u64, u64, usize)>,
        out: &mut Vec<usize>,
        mut spans: Option<&mut Vec<SubRange>>,
    ) -> bool {
        if slots.is_empty() {
            out.clear();
            if let Some(sp) = spans.as_deref_mut() {
                sp.clear();
            }
            return true;
        }
        with_resource_registry(|reg| {
            Self::resolve_slots_with_reg(reg, slots, root, heap0, out, spans)
        })
    }
}
