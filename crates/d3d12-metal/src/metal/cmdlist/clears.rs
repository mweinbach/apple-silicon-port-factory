//! Deferred/coalesced RTV & DSV clears, compute-clear batching, UAV clears,
//! and shared utility compute dispatches.

use super::opts::{barrier_masks, barrier_tight_stages, legacy_barriers};
use super::state::{
    layered, merge_req_for_after, ClearKind, Encoder, FullWhy, OblKind, PendingClear, TargetView,
    COMPUTE_STAGES, PRE_RASTER, STAGE_DISPATCH,
};
use super::CommandList;
use crate::metal::clear::{self, Class, ClearValue, FillParams, RectClearParams, TexParams};
use crate::metal::descriptor::{resolve_uav_for_clear, UavClearLookup};
use crate::metal::encstats as stats;
use crate::metal::passtiming as pt;
use crate::metal::pipeline::READS_TEX;
use crate::metal::resource::{MtlTexture, Resource};
use crate::types::Rect;
use objc2::rc::Retained;
use objc2::Message;
use objc2_metal::{
    MTL4ArgumentTable, MTL4ComputeCommandEncoder, MTL4RenderCommandEncoder,
    MTL4RenderPassDescriptor, MTLBlitOption, MTLBuffer, MTLClearColor, MTLLoadAction, MTLOrigin,
    MTLPixelFormat, MTLPrimitiveType, MTLRenderStages, MTLScissorRect, MTLSize, MTLStages,
    MTLStoreAction, MTLTexture, MTLTextureType, MTLTextureUsage, MTLViewport,
};

/// Clip D3D12 clear scissor rects to a mip level's `(mw, mh)` dimensions.
fn clip_scissor_rects(rects: &[Rect], mw: usize, mh: usize) -> Vec<MTLScissorRect> {
    rects
        .iter()
        .filter_map(|rc| {
            let x0 = (rc.left.max(0) as usize).min(mw);
            let y0 = (rc.top.max(0) as usize).min(mh);
            let x1 = (rc.right.max(0) as usize).min(mw);
            let y1 = (rc.bottom.max(0) as usize).min(mh);
            (x1 > x0 && y1 > y0).then_some(MTLScissorRect {
                x: x0,
                y: y0,
                width: x1 - x0,
                height: y1 - y0,
            })
        })
        .collect()
}

impl CommandList {
    #[inline]
    fn bump_clear_coalesce_stat(&self, count: u64, include_fold_v2: bool) {
        if include_fold_v2 && self.opts.opt_clear_pass_fold_v2 {
            stats::add(&stats::c::clear_pass_fold_v2, count);
        } else if !self.opts.opt_clear_pass_fold_v2 {
            if self.opts.opt_clear_isolate_coalesce {
                stats::add(&stats::c::clear_isolate_coalesce, count);
            } else if self.opts.opt_clear_zero_qb_coalesce {
                stats::add(&stats::c::clear_zero_qb_coalesce, count);
            }
        }
    }

    /// Whether a deferred clear `p` can be satisfied by `compute_clear_item`
    /// without opening a standalone render pass.
    pub(super) fn can_compute_clear(p: &PendingClear) -> bool {
        match p.kind {
            ClearKind::Color(c) => unsafe { Self::compute_clear_item(&p.view, c, 1).is_some() },
            ClearKind::DepthStencil { .. } => false,
        }
    }

    /// Lands the pending (deferred) clears of `rs` before work outside a
    /// render pass (copies, UAV clears, upscales) reads or overwrites them:
    /// a deferred clear otherwise lands at the next barrier/draw/close,
    /// after that work.
    pub(super) fn flush_clears_for(&mut self, rs: &[usize]) -> Result<(), String> {
        if self.clears.pending_clears.is_empty() {
            return Ok(());
        }
        self.flush_clears(&stats::c::cf_copy, |p| rs.contains(&p.view.resource))
    }

    /// Encode (and remove) the pending clears matching `pred`, in order;
    /// `why` counts the standalone clear passes this flush forces.
    pub(super) fn flush_clears(
        &mut self,
        why: &stats::Counter,
        pred: impl Fn(&PendingClear) -> bool,
    ) -> Result<(), String> {
        if self.clears.pending_clears.is_empty() {
            return Ok(());
        }
        let mut todo = Vec::new();
        self.clears.pending_clears.retain(|p| {
            if pred(p) {
                todo.push(*p);
                false
            } else {
                true
            }
        });
        let mut res = Ok(());
        // Standalone clear passes cost ~50 µs each plus inter-pass idle;
        // shader-writable colour targets are cleared by compute instead, all
        // in one encoder, first (it can join an open compute encoder).
        let mut batch = Vec::new();
        todo.retain(|p| {
            if let ClearKind::Color(c) = p.kind {
                if let Some(item) = unsafe { Self::compute_clear_item(&p.view, c, 1) } {
                    batch.push(item);
                    return false;
                }
            }
            true
        });
        if !batch.is_empty() {
            res = self.compute_clear_batch(batch);
        }
        if stats::enabled() {
            for p in &todo {
                stats::inc(why);
                let writable = || unsafe {
                    (*(p.view.resource as *const Resource))
                        .texture()
                        .is_some_and(|t| t.usage().contains(MTLTextureUsage::ShaderWrite))
                };
                stats::inc(match p.kind {
                    ClearKind::DepthStencil { .. } => &stats::c::cp_dsv,
                    ClearKind::Color(_) if writable() => &stats::c::cp_rtv_fmt,
                    ClearKind::Color(_) => &stats::c::cp_rtv_nows,
                });
            }
        }
        if (self.opts.opt_uav0_clear_overlap
            || self.opts.opt_clear_pass_fold_v2
            || self.clear_isolate_active())
            && todo.len() > 1
        {
            let r = self.encode_coalesced_clear_passes(todo);
            if r.is_err() && res.is_ok() {
                res = r;
            }
            return res;
        }
        for p in todo {
            if self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active() {
                self.bar.opening_clear_frag_cap = Some((p.defer_frag_after, p.defer_frag_req));
            }
            let r = unsafe {
                match p.kind {
                    ClearKind::Color(c) => self.encode_rtv_clear_pass(&p.view, c),
                    ClearKind::DepthStencil { depth, stencil } => {
                        self.encode_dsv_clear_pass(&p.view, depth, stencil)
                    }
                }
            };
            self.bar.opening_clear_frag_cap = None;
            if r.is_err() && res.is_ok() {
                res = r;
            }
        }
        res
    }

    fn same_clear_kind(a: &ClearKind, b: &ClearKind) -> bool {
        match (*a, *b) {
            (ClearKind::Color(x), ClearKind::Color(y)) => {
                x.iter().zip(&y).all(|(u, v)| u.to_bits() == v.to_bits())
            }
            (
                ClearKind::DepthStencil { depth: d0, stencil: s0 },
                ClearKind::DepthStencil { depth: d1, stencil: s1 },
            ) => d0.map(f32::to_bits) == d1.map(f32::to_bits) && s0 == s1,
            _ => false,
        }
    }

    /// Coalesce deferred non-compute clears (`AW2_UAV0_CLEAR_OVERLAP=1` / `AW2_CLEAR_PASS_FOLD_V2=1`):
    /// 1. Contiguous slices of the same layered RTV/DSV texture/mip/format with
    ///    identical clear values merge into a single layered `renderTargetArrayLength` pass.
    /// 2. Remaining single-slice RTV (up to 8) + DSV (up to 1) clears on distinct
    ///    resources sharing identical `(mw, mh, sample_count)` merge into a single
    ///    multi-attachment `MTL4RenderPassDescriptor`.
    fn encode_coalesced_clear_passes(&mut self, todo: Vec<PendingClear>) -> Result<(), String> {
        struct ResolvedClear {
            view: TargetView,
            kind: ClearKind,
            defer_frag_after: MTLStages,
            defer_frag_req: [u32; 7],
            tex: Retained<MtlTexture>,
            level: usize,
            slice: usize,
            mw: usize,
            mh: usize,
            samples: usize,
        }
        let mut items = Vec::with_capacity(todo.len());
        let mut first_err: Result<(), String> = Ok(());
        for p in todo {
            match unsafe { Self::target_texture(&p.view) } {
                Some((tex, level, slice)) => {
                    let mw = (tex.width() >> level).max(1);
                    let mh = (tex.height() >> level).max(1);
                    let samples = tex.sampleCount();
                    items.push(ResolvedClear {
                        view: p.view,
                        kind: p.kind,
                        defer_frag_after: p.defer_frag_after,
                        defer_frag_req: p.defer_frag_req,
                        tex,
                        level,
                        slice,
                        mw,
                        mh,
                        samples,
                    });
                }
                None => {
                    if first_err.is_ok() {
                        first_err = Err("clear view does not reference a texture".into());
                    }
                }
            }
        }
        let n = items.len();
        let mut done = vec![false; n];

        // Pass 1: coalesce contiguous slices on layered textures (`Type2DArray`, `TypeCube`, `TypeCubeArray`, ...).
        for i in 0..n {
            if done[i] || !layered(&items[i].tex) {
                continue;
            }
            let mut matching: Vec<(usize, usize)> = Vec::new();
            for j in i..n {
                if !done[j]
                    && items[j].view.resource == items[i].view.resource
                    && items[j].view.format == items[i].view.format
                    && items[j].view.mip == items[i].view.mip
                    && Self::same_clear_kind(&items[j].kind, &items[i].kind)
                {
                    matching.push((items[j].slice, j));
                }
            }
            if matching.len() < 2 {
                continue;
            }
            matching.sort_unstable_by_key(|&(s, _)| s);
            matching.dedup_by_key(|&mut (s, _)| s);
            let mut run_start = 0;
            while run_start < matching.len() {
                let mut run_end = run_start + 1;
                while run_end < matching.len() && matching[run_end].0 == matching[run_end - 1].0 + 1 {
                    run_end += 1;
                }
                let layers = run_end - run_start;
                if layers >= 2 {
                    let base_slice = matching[run_start].0;
                    let idx0 = matching[run_start].1;
                    let mut cap_a = MTLStages::empty();
                    let mut cap_r = [0u32; 7];
                    for &(_, idx) in &matching[run_start..run_end] {
                        done[idx] = true;
                        cap_a |= items[idx].defer_frag_after;
                        merge_req_for_after(
                            &mut cap_r,
                            items[idx].defer_frag_after,
                            &items[idx].defer_frag_req,
                        );
                    }
                    if self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active() {
                        self.bar.opening_clear_frag_cap = Some((cap_a, cap_r));
                    }
                    let it = &items[idx0];
                    let r = match it.kind {
                        ClearKind::Color(c) => self.rtv_clear_slice(
                            &it.tex,
                            it.level,
                            base_slice,
                            layers,
                            c,
                            it.view.resource,
                        ),
                        ClearKind::DepthStencil { depth, stencil } => self.dsv_clear_slice(
                            &it.tex,
                            it.level,
                            base_slice,
                            layers,
                            depth,
                            stencil,
                            it.view.resource,
                        ),
                    };
                    self.bar.opening_clear_frag_cap = None;
                    if r.is_ok() {
                        stats::add(&stats::c::clear_pass_merged, (layers - 1) as u64);
                        self.bump_clear_coalesce_stat((layers - 1) as u64, false);
                    } else if first_err.is_ok() {
                        first_err = r;
                    }
                }
                run_start = run_end;
            }
        }

        // Pass 2: coalesce remaining single-slice RTV (<= 8) + DSV (<= 1) clears on distinct
        // resources that share identical `(mw, mh, sample_count)`.
        for i in 0..n {
            if done[i] {
                continue;
            }
            done[i] = true;
            let (mw, mh, samples) = (items[i].mw, items[i].mh, items[i].samples);
            let mut colors: Vec<usize> = Vec::new();
            let mut dsv_idx: Option<usize> = None;
            let mut targets: Vec<usize> = vec![items[i].view.resource];
            match items[i].kind {
                ClearKind::Color(_) => colors.push(i),
                ClearKind::DepthStencil { .. } => dsv_idx = Some(i),
            }
            for j in (i + 1)..n {
                if done[j]
                    || items[j].mw != mw
                    || items[j].mh != mh
                    || items[j].samples != samples
                    || targets.contains(&items[j].view.resource)
                {
                    continue;
                }
                match items[j].kind {
                    ClearKind::Color(_) if colors.len() < 8 => {
                        colors.push(j);
                        targets.push(items[j].view.resource);
                        done[j] = true;
                    }
                    ClearKind::DepthStencil { .. } if dsv_idx.is_none() => {
                        dsv_idx = Some(j);
                        targets.push(items[j].view.resource);
                        done[j] = true;
                    }
                    _ => {}
                }
            }
            let merged_count = colors.len() + usize::from(dsv_idx.is_some());
            if merged_count == 1 {
                let it = &items[i];
                if self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active() {
                    self.bar.opening_clear_frag_cap = Some((it.defer_frag_after, it.defer_frag_req));
                }
                let r = match it.kind {
                    ClearKind::Color(c) => {
                        self.rtv_clear_slice(&it.tex, it.level, it.slice, 1, c, it.view.resource)
                    }
                    ClearKind::DepthStencil { depth, stencil } => self.dsv_clear_slice(
                        &it.tex,
                        it.level,
                        it.slice,
                        1,
                        depth,
                        stencil,
                        it.view.resource,
                    ),
                };
                self.bar.opening_clear_frag_cap = None;
                if r.is_err() && first_err.is_ok() {
                    first_err = r;
                }
                continue;
            }
            let mut cap_a = MTLStages::empty();
            let mut cap_r = [0u32; 7];
            for &ci in &colors {
                cap_a |= items[ci].defer_frag_after;
                merge_req_for_after(&mut cap_r, items[ci].defer_frag_after, &items[ci].defer_frag_req);
            }
            if let Some(di) = dsv_idx {
                cap_a |= items[di].defer_frag_after;
                merge_req_for_after(&mut cap_r, items[di].defer_frag_after, &items[di].defer_frag_req);
            }
            let rp = MTL4RenderPassDescriptor::new();
            for (slot, &ci) in colors.iter().enumerate() {
                let it = &items[ci];
                let ClearKind::Color(color) = it.kind else { unreachable!() };
                let ca = unsafe { rp.colorAttachments().objectAtIndexedSubscript(slot) };
                ca.setTexture(Some(&it.tex));
                ca.setLevel(it.level);
                if it.tex.textureType() == MTLTextureType::Type3D {
                    ca.setDepthPlane(it.slice);
                } else {
                    ca.setSlice(it.slice);
                }
                ca.setLoadAction(MTLLoadAction::Clear);
                ca.setStoreAction(MTLStoreAction::Store);
                ca.setClearColor(MTLClearColor {
                    red: color[0] as f64,
                    green: color[1] as f64,
                    blue: color[2] as f64,
                    alpha: color[3] as f64,
                });
            }
            if let Some(di) = dsv_idx {
                let it = &items[di];
                let ClearKind::DepthStencil { depth, stencil } = it.kind else { unreachable!() };
                let pf = it.tex.pixelFormat();
                let has_depth =
                    pf != MTLPixelFormat::X32_Stencil8 && pf != MTLPixelFormat::Stencil8;
                let has_stencil = matches!(
                    pf,
                    MTLPixelFormat::Depth32Float_Stencil8
                        | MTLPixelFormat::X32_Stencil8
                        | MTLPixelFormat::Stencil8
                );
                if has_depth {
                    let da = rp.depthAttachment();
                    da.setTexture(Some(&it.tex));
                    da.setLevel(it.level);
                    da.setSlice(it.slice);
                    da.setLoadAction(if depth.is_some() {
                        MTLLoadAction::Clear
                    } else {
                        MTLLoadAction::Load
                    });
                    da.setStoreAction(MTLStoreAction::Store);
                    da.setClearDepth(depth.unwrap_or(0.0) as f64);
                }
                if has_stencil {
                    let sa = rp.stencilAttachment();
                    sa.setTexture(Some(&it.tex));
                    sa.setLevel(it.level);
                    sa.setSlice(it.slice);
                    sa.setLoadAction(if stencil.is_some() {
                        MTLLoadAction::Clear
                    } else {
                        MTLLoadAction::Load
                    });
                    sa.setStoreAction(MTLStoreAction::Store);
                    sa.setClearStencil(stencil.unwrap_or(0) as u32);
                }
            }
            stats::inc(&stats::c::clear_pass);
            stats::add(&stats::c::clear_pass_merged, (merged_count - 1) as u64);
            self.bump_clear_coalesce_stat((merged_count - 1) as u64, false);
            if self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active() {
                self.bar.opening_clear_frag_cap = Some((cap_a, cap_r));
            }
            let opened = self.util_render_pass(&rp, &targets, pt::kind::CLEAR);
            self.bar.opening_clear_frag_cap = None;
            match opened {
                Some(_) => {
                    if let Some(m) = self.pt_cur() {
                        (m.w, m.h) = (mw as u32, mh as u32);
                        m.shader = pt::synth::CLEAR_PASS;
                    }
                    self.end_encoder();
                }
                None => {
                    if first_err.is_ok() {
                        first_err = Err("render encoder".into());
                    }
                }
            }
        }
        first_err
    }

    /// A whole-subresource colour clear as a compute write: `slices` slices
    /// (3D: W planes) from the view's first, of a shader-writable,
    /// single-sample 2D / 2D-array / 3D texture in a typed colour format
    /// (sRGB: the encoded value through the linear view; None: use a clear
    /// render pass).
    unsafe fn compute_clear_item(
        v: &TargetView,
        color: [f32; 4],
        slices: u32,
    ) -> Option<(usize, Retained<MtlTexture>, TexParams, &'static str)> {
        use crate::format::{self, Num};
        let r = &*(v.resource as *const Resource);
        let t = r.texture()?;
        if !t.usage().contains(MTLTextureUsage::ShaderWrite) || t.sampleCount() > 1 {
            return None;
        }
        let fits = match t.textureType() {
            MTLTextureType::Type2D => slices == 1,
            MTLTextureType::Type2DArray => {
                v.first_slice as usize + slices as usize <= t.arrayLength()
            }
            MTLTextureType::Type3D => true,
            _ => false,
        };
        if !fits {
            return None;
        }
        let vfmt = match MTLPixelFormat(v.format as usize) {
            MTLPixelFormat::Invalid => r.mtl_format,
            f => f,
        };
        let (wfmt, srgb) = match vfmt {
            MTLPixelFormat::RGBA8Unorm_sRGB => (MTLPixelFormat::RGBA8Unorm, true),
            MTLPixelFormat::BGRA8Unorm_sRGB => (MTLPixelFormat::BGRA8Unorm, true),
            f => (f, false),
        };
        let res_fi = format::info(r.desc.Format);
        let fi = if wfmt == r.mtl_format && !matches!(res_fi.num, Num::Typeless) {
            res_fi
        } else {
            format::typed_for_mtl(wfmt)?
        };
        if !matches!(fi.num, Num::Float | Num::Unorm | Num::Snorm | Num::Uint | Num::Sint) {
            stats::inc(&stats::c::cx_num);
            return None;
        }
        let mut color = color;
        if srgb {
            for c in &mut color[..3] {
                let x = c.clamp(0.0, 1.0);
                *c = if x <= 0.003_130_8 {
                    12.92 * x
                } else {
                    1.055 * x.powf(1.0 / 2.4) - 0.055
                };
            }
        }
        let (class, u, f) = clear::texture_value(fi.dxgi, ClearValue::Float(color)).ok()?;
        let mip = v.mip as u32;
        let (w, h) = ((t.width() >> mip).max(1) as u32, (t.height() >> mip).max(1) as u32);
        let (view, z0, d, dim) = if t.textureType() == MTLTextureType::Type3D {
            // RTV slices of a 3D texture are W planes of the mip level.
            let dm = (t.depth() >> mip).max(1) as u32;
            let z0 = v.first_slice.min(dm);
            let d = slices.min(dm - z0);
            if d == 0 {
                return None;
            }
            (r.texture_view(wfmt, MTLTextureType::Type3D, (mip, 1), (0, 1))?, z0, d, "3d")
        } else if slices > 1 {
            (
                r.texture_view(
                    wfmt,
                    MTLTextureType::Type2DArray,
                    (mip, 1),
                    (v.first_slice, slices),
                )?,
                0,
                slices,
                "2da",
            )
        } else {
            (
                r.texture_view(wfmt, MTLTextureType::Type2D, (mip, 1), (v.first_slice, 1))?,
                0,
                1,
                "2d",
            )
        };
        let p = TexParams { u, f, x0: 0, y0: 0, z0, w, h, d, _pad: [0; 2] };
        let name = match (dim, class) {
            ("3d", Class::Float) => "port_clear_3d_f",
            ("3d", Class::Uint) => "port_clear_3d_u",
            ("3d", Class::Sint) => "port_clear_3d_i",
            ("2da", Class::Float) => "port_clear_2da_f",
            ("2da", Class::Uint) => "port_clear_2da_u",
            ("2da", Class::Sint) => "port_clear_2da_i",
            (_, Class::Float) => "port_clear_2d_f",
            (_, Class::Uint) => "port_clear_2d_u",
            (_, Class::Sint) => "port_clear_2d_i",
        };
        Some((v.resource, view, p, name))
    }

    /// Encode compute clears back to back in one compute encoder (distinct
    /// subresources: no barriers between them), ordered after earlier
    /// fragment work and before later work by stage-masked barriers.
    fn compute_clear_batch(
        &mut self,
        mut items: Vec<(usize, Retained<MtlTexture>, TexParams, &'static str)>,
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        if self.opts.opt_clear_cs_tile4 {
            for (_rk, _view, p, name) in &mut items {
                if p.w >= 4 && p.h >= 4 {
                    if let Some(t4) = match *name {
                        "port_clear_2d_f" => Some("port_clear_2d_f_t4"),
                        "port_clear_2d_u" => Some("port_clear_2d_u_t4"),
                        "port_clear_2d_i" => Some("port_clear_2d_i_t4"),
                        "port_clear_2da_f" => Some("port_clear_2da_f_t4"),
                        "port_clear_2da_u" => Some("port_clear_2da_u_t4"),
                        "port_clear_2da_i" => Some("port_clear_2da_i_t4"),
                        _ => None,
                    } {
                        *name = t4;
                    }
                }
            }
            if items.len() > 1 {
                items.sort_by_key(|it| it.3);
            }
        }
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let t = self.ensure_util_table()?;
        let n_items = items.len() as u32;
        let ib_tight = self.ib_tight_active();
        let mut clear_keys: Vec<usize> = Vec::with_capacity(items.len());
        for &(rk, _, _, _) in &items {
            if rk != 0 && !clear_keys.contains(&rk) {
                unsafe { (*(rk as *const Resource)).mark_gpu_used() };
                clear_keys.push(rk);
            }
        }
        let open_cs_pre_list_synced = matches!(self.enc, Encoder::Compute(_))
            && self.bar.enc_cur_sync[STAGE_DISPATCH].pre_list_all;
        let closed_disp_pre_list_synced = self.bar.stage_gen[STAGE_DISPATCH] > 0
            && self.bar.stage_sync[STAGE_DISPATCH].pre_list_all;
        let exact_pre_list_sync = self.opts.opt_clear_cs_exact_sync
            && !self.is_copy_queue_light()
            && !legacy_barriers()
            && barrier_masks()
            && !self.bar.needs_barrier
            && !self.bar.in_list_gfx_non_clear
            && self.bar.gfx_uav_gen == 0
            && !self.bar.uav_gfx_dirty
            && !clear_keys.is_empty()
            && !clear_keys.iter().any(|r| {
                self.bar.written.contains(r)
                    || self.bar.clear_rp_res_ids.contains(r)
                    || self.bar.pass_targets.contains(r)
            })
            && (open_cs_pre_list_synced || closed_disp_pre_list_synced);
        let v5_res_pre_sync = self.opts.opt_cs_mid_qb_res_defer_v5
            && ib_tight
            && self.render_split_sync_active()
            && !self.is_copy_queue_light()
            && !legacy_barriers()
            && barrier_masks()
            && !self.bar.needs_barrier
            && !clear_keys.is_empty()
            && clear_keys.iter().all(|&r| r != 0);
        let in_open_cs = matches!(self.enc, Encoder::Compute(_));
        if in_open_cs && !self.opts.opt_cs_mid_qb_unpoison_v4 {
            self.bar.cs_mid_v3_poisoned = [true; 3];
        }
        if v5_res_pre_sync {
            let v5_exact = self.opts.opt_clear_cs_exact_sync
                && self.bar.gfx_uav_gen == 0
                && !self.bar.uav_gfx_dirty
                && !clear_keys.iter().any(|r| {
                    self.bar.written.contains(r)
                        || self.bar.in_list_rt_res_ids.contains(r)
                        || self.bar.clear_rp_res_ids.contains(r)
                        || self.bar.pass_targets.contains(r)
                })
                && (open_cs_pre_list_synced || closed_disp_pre_list_synced);
            if v5_exact {
                let mut need_a = MTLStages::empty();
                let mut need_req = [0u32; 7];
                if !open_cs_pre_list_synced {
                    need_a |= MTLStages::Dispatch;
                    let pg = self.cur_producer_gen();
                    merge_req_for_after(&mut need_req, MTLStages::Dispatch, &pg);
                }
                let unsynced_to_rt = self.unsatisfied_enc_after_for_req(
                    self.bar.split_pend.frag_after,
                    MTLStages::Dispatch,
                    &self.bar.split_pend.frag_req,
                    self.mid_qb_stagesync_active(),
                );
                if !unsynced_to_rt.is_empty() {
                    need_a |= unsynced_to_rt;
                    merge_req_for_after(
                        &mut need_req,
                        unsynced_to_rt,
                        &self.bar.split_pend.frag_req,
                    );
                }
                if !need_a.is_empty() {
                    for &rk in &clear_keys {
                        Self::push_cs_mid_obl_into(
                            &mut self.bar.pend_cs_items,
                            rk,
                            need_a,
                            MTLStages::empty(),
                            MTLStages::empty(),
                            need_req,
                        );
                        self.record_cs_soft_disp(rk, need_a, need_req);
                    }
                    self.bar.split_pend.add_with_req(
                        need_a,
                        MTLStages::Dispatch,
                        [MTLStages::empty(); 2],
                        need_req,
                    );
                    self.sync_legacy_from_split();
                }
                stats::inc(&stats::c::clear_cs_exact_sync);
            } else {
                let fa = self.bar.split_pend.frag_after | MTLStages::Fragment;
                let mut fr = self.bar.split_pend.frag_req;
                let pg = self.cur_producer_gen();
                merge_req_for_after(&mut fr, MTLStages::Fragment, &pg);
                if in_open_cs
                    && !self
                        .unsatisfied_enc_after_for_req(
                            fa,
                            MTLStages::Dispatch,
                            &fr,
                            self.mid_qb_stagesync_active(),
                        )
                        .is_empty()
                {
                    self.bar.cs_mid_v3_poisoned[0] = true;
                }
                for &rk in &clear_keys {
                    Self::push_cs_mid_obl_into(
                        &mut self.bar.pend_cs_items,
                        rk,
                        fa,
                        MTLStages::empty(),
                        MTLStages::empty(),
                        fr,
                    );
                    self.record_cs_soft_disp(rk, fa, fr);
                }
                self.bar.split_pend.add_with_req(
                    fa,
                    MTLStages::Dispatch,
                    [MTLStages::empty(); 2],
                    fr,
                );
                self.sync_legacy_from_split();
            }
        } else if exact_pre_list_sync {
            if !open_cs_pre_list_synced {
                self.add_pend(MTLStages::Dispatch, MTLStages::Dispatch);
            } else if self.render_split_sync_active() {
                let unsynced_cs_to_rt = self.unsatisfied_enc_after_for(
                    self.bar.split_pend.frag_after & COMPUTE_STAGES,
                    MTLStages::Dispatch,
                    true,
                );
                if !unsynced_cs_to_rt.is_empty() {
                    let fr = self.bar.split_pend.frag_req;
                    self.bar.split_pend.disp_after |= unsynced_cs_to_rt;
                    merge_req_for_after(&mut self.bar.split_pend.disp_req, unsynced_cs_to_rt, &fr);
                    self.bar.pend_cs_poisoned[0] = true;
                    self.record_cs_soft_disp(0, unsynced_cs_to_rt, fr);
                    self.sync_legacy_from_split();
                    if in_open_cs {
                        self.bar.cs_mid_v3_poisoned[0] = true;
                    }
                }
            } else if !self.bar.pend_after.is_empty() {
                self.bar.pend_before |= MTLStages::Dispatch;
                self.bar.pend_cs_poisoned[0] = true;
                if in_open_cs {
                    self.bar.cs_mid_v3_poisoned[0] = true;
                }
            }
            stats::inc(&stats::c::clear_cs_exact_sync);
        } else {
            // The targets are in RENDER_TARGET state: earlier same-state
            // attachment writes are fragment work; other prior accesses were
            // ordered by the transition into it (pending stages).
            if self.render_split_sync_active() {
                let fa = self.bar.split_pend.frag_after;
                let fr = self.bar.split_pend.frag_req;
                if in_open_cs
                    && !self
                        .unsatisfied_enc_after_for_req(
                            fa,
                            MTLStages::Dispatch,
                            &fr,
                            self.mid_qb_stagesync_active(),
                        )
                        .is_empty()
                {
                    self.bar.cs_mid_v3_poisoned[0] = true;
                }
                self.bar.split_pend.disp_after |= fa;
                merge_req_for_after(&mut self.bar.split_pend.disp_req, fa, &fr);
                if !fa.is_empty() {
                    self.bar.pend_cs_poisoned[0] = true;
                    self.record_cs_soft_disp(0, fa, fr);
                }
            }
            self.add_pend(MTLStages::Fragment, MTLStages::Dispatch);
        }
        let e = if ib_tight {
            for &rk in &clear_keys {
                self.ib_note_touch_before_tight(rk, MTLStages::Dispatch);
            }
            let e = self
                .compute_op_tight(
                    MTLStages::Dispatch,
                    Some(&clear_keys),
                    Some(&[]),
                    Some(&clear_keys),
                )
                .ok_or("compute encoder")?;
            for &rk in &clear_keys {
                if !self.ib.ib_cs_cleared.contains(&rk) {
                    self.ib.ib_cs_cleared.push(rk);
                }
                if self.opts.opt_ib_substage_narrow || self.is_copy_queue_light() {
                    for s in &mut self.ib.ib_sub {
                        if !s.cs_cleared.contains(&rk) {
                            s.cs_cleared.push(rk);
                        }
                    }
                }
            }
            e
        } else {
            self.ib_note(0);
            let e = self.compute().ok_or("compute encoder")?;
            self.ib.ib_cs_clear = true;
            e
        };
        self.pt_note_util(pt::synth::CLEAR_CS, n_items);
        if self.opts.opt_fast_bind_pso {
            e.setArgumentTable(Some(&t));
        }
        let mut stack_vas = [0u64; 32];
        let mut heap_vas: Vec<u64> = Vec::new();
        let item_vas: &[u64] = if items.len() <= stack_vas.len() {
            {
                let mut r = ring.lock().unwrap();
                for (dst, (_, _, p, _)) in stack_vas.iter_mut().zip(&items) {
                    *dst = r.upload(clear::bytes_of(p), 256)?;
                }
            }
            &stack_vas[..items.len()]
        } else {
            heap_vas.reserve(items.len());
            {
                let mut r = ring.lock().unwrap();
                for (_, _, p, _) in &items {
                    heap_vas.push(r.upload(clear::bytes_of(p), 256)?);
                }
            }
            &heap_vas
        };
        let mut last_pso_ptr: usize = 0;
        for ((_rk, view, p, name), &pa) in items.into_iter().zip(item_vas.iter()) {
            let (pso, tg) = clear::kernel_for_dispatch(name, self.opts.opt_fast_bind_pso)?;
            unsafe {
                t.setAddress_atIndex(pa, 0);
                t.setTexture_atIndex(view.gpuResourceID(), 0);
            }
            if !self.opts.opt_fast_bind_pso {
                e.setArgumentTable(Some(&t));
            }
            let pso_ptr = &*pso as *const _ as usize;
            if !self.opts.opt_fast_bind_pso || pso_ptr != last_pso_ptr {
                e.setComputePipelineState(&pso);
                last_pso_ptr = pso_ptr;
            }
            let is_t4 = name.ends_with("_t4");
            let (gw, gh) = if is_t4 {
                ((p.w as usize).div_ceil(2), (p.h as usize).div_ceil(2))
            } else {
                (p.w as usize, p.h as usize)
            };
            e.dispatchThreadgroups_threadsPerThreadgroup(
                MTLSize {
                    width: gw.div_ceil(tg[0] as usize),
                    height: gh.div_ceil(tg[1] as usize),
                    depth: (p.d as usize).div_ceil(tg[2] as usize),
                },
                MTLSize { width: tg[0] as usize, height: tg[1] as usize, depth: tg[2] as usize },
            );
            if is_t4 {
                stats::inc(&stats::c::clear_cs_tile4);
            }
            stats::inc(&stats::c::clear_cs);
        }
        if n_items > 0 {
            self.note_stage_ran(MTLStages::Dispatch);
        }
        e.setArgumentTable(Some(&self.table));
        self.compute_pso_bound = false;
        if ib_tight {
            const SIMUL: u32 = crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS;
            for &rk in &clear_keys {
                let kind = if self.opts.opt_ib_exact_obl
                    && rk != 0
                    && unsafe { (*(rk as *const Resource)).desc.Flags & SIMUL == 0 }
                {
                    OblKind::ClassSrv(READS_TEX)
                } else {
                    OblKind::Any
                };
                self.ib_add_obl_kind_tight(rk, MTLStages::Dispatch, COMPUTE_STAGES, kind);
            }
            if self.opts.opt_cs_mid_qb_res_defer_v5
                && !clear_keys.is_empty()
                && clear_keys.iter().all(|&rk| rk != 0)
            {
                if self.opts.opt_cs_to_ps_overlap_v2 {
                    self.add_pend_with_soft_res(
                        MTLStages::Dispatch,
                        MTLStages::Fragment,
                        &clear_keys,
                    );
                } else {
                    self.add_pend(MTLStages::Dispatch, MTLStages::Fragment | PRE_RASTER);
                }
                for &rk in &clear_keys {
                    self.bar.cs_end_res.push((
                        rk,
                        MTLStages::Dispatch,
                        MTLStages::Dispatch,
                        MTLStages::empty(),
                    ));
                }
                stats::inc(&stats::c::cs_mid_qb_v5);
            } else if self.opts.opt_cs_to_ps_overlap || self.opts.opt_cs_to_ps_overlap_v2 {
                self.add_pend_with_soft_res(
                    MTLStages::Dispatch,
                    MTLStages::Fragment | MTLStages::Dispatch | MTLStages::Blit,
                    &clear_keys,
                );
            } else {
                self.add_pend(
                    MTLStages::Dispatch,
                    MTLStages::Fragment | PRE_RASTER | MTLStages::Dispatch | MTLStages::Blit,
                );
            }
        } else {
            self.produced(
                MTLStages::Dispatch,
                MTLStages::Fragment | PRE_RASTER | MTLStages::Dispatch | MTLStages::Blit,
            );
        }
        Ok(())
    }

    /// Record a whole-subresource clear for folding into a later pass.
    fn defer_clear(&mut self, view: TargetView, kind: ClearKind) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        let dsv = matches!(kind, ClearKind::DepthStencil { .. });
        // An open pass already attaching the resource must not miss the clear.
        let r = view.resource;
        let attached = self.gfx.draw_pass
            && (self.active_rtvs().iter().any(|v| v.resource == r)
                || self.active_dsv().is_some_and(|d| d.resource == r));
        if attached || self.bar.pass_targets.contains(&r) {
            self.end_encoder();
        }
        // The same subresource cleared through a differently-formatted view
        // must land first.
        self.flush_clears(&stats::c::cf_defer, |p| {
            p.view.resource == view.resource
                && p.view.mip == view.mip
                && p.view.first_slice == view.first_slice
                && !p.same_key(&view, dsv)
        })?;
        let (mut defer_frag_after, mut defer_frag_req) = if self.render_split_sync_active() {
            if self.opts.opt_clear_zero_qb_coalesce
                && !self.bar.frag_rt_poisoned
                && r != 0
                && unsafe {
                    let res = &*(r as *const Resource);
                    res.placed_in.is_none()
                        && (res.desc.Flags & crate::types::resource_flags::ALLOW_SIMULTANEOUS_ACCESS
                            == 0)
                }
            {
                let mut check_union = self.bar.frag_non_rt_after;
                for &(_, ia, _) in &self.bar.frag_rt_pend {
                    check_union |= ia;
                }
                if check_union == self.bar.split_pend.frag_after {
                    let (iso_a, iso_r) = self
                        .bar
                        .frag_rt_pend
                        .iter()
                        .find(|e| e.0 == r)
                        .map_or((MTLStages::empty(), [0u32; 7]), |&(_, ia, ir)| (ia, ir));
                    if iso_a != self.bar.split_pend.frag_after {
                        stats::inc(&stats::c::clear_zero_qb_coalesce);
                    }
                    (iso_a, iso_r)
                } else {
                    (self.bar.split_pend.frag_after, self.bar.split_pend.frag_req)
                }
            } else {
                (self.bar.split_pend.frag_after, self.bar.split_pend.frag_req)
            }
        } else {
            let mut rq = [0u32; 7];
            merge_req_for_after(&mut rq, self.bar.pend_after, &self.bar.stage_gen);
            (self.bar.pend_after, rq)
        };
        let kind = match (
            kind,
            self.clears.pending_clears.iter().position(|p| p.same_key(&view, dsv)),
        ) {
            (ClearKind::DepthStencil { depth, stencil }, Some(i)) => {
                let old = self.clears.pending_clears.remove(i);
                defer_frag_after |= old.defer_frag_after;
                merge_req_for_after(
                    &mut defer_frag_req,
                    old.defer_frag_after,
                    &old.defer_frag_req,
                );
                let ClearKind::DepthStencil { depth: d0, stencil: s0 } = old.kind else {
                    unreachable!()
                };
                ClearKind::DepthStencil { depth: depth.or(d0), stencil: stencil.or(s0) }
            }
            (k, Some(i)) => {
                let old = self.clears.pending_clears.remove(i);
                defer_frag_after |= old.defer_frag_after;
                merge_req_for_after(
                    &mut defer_frag_req,
                    old.defer_frag_after,
                    &old.defer_frag_req,
                );
                k
            }
            (k, None) => k,
        };
        stats::inc(&stats::c::clear_deferred);
        self.clears.pending_clears.push(PendingClear {
            view,
            kind,
            defer_frag_after,
            defer_frag_req,
        });
        Ok(())
    }

    /// One render pass clearing `v`'s first slice to `color`.
    pub(super) unsafe fn encode_rtv_clear_pass(&mut self, v: &TargetView, color: [f32; 4]) -> Result<(), String> {
        let (tex, level, slice) =
            Self::target_texture(v).ok_or("RTV does not reference a texture")?;
        self.rtv_clear_slice(&tex, level, slice, 1, color, v.resource)
    }

    /// Clear `layers` slices starting at `slice` in one (layered) render pass.
    fn rtv_clear_slice(
        &mut self,
        tex: &MtlTexture,
        level: usize,
        slice: usize,
        layers: usize,
        color: [f32; 4],
        resource: usize,
    ) -> Result<(), String> {
        let rp = MTL4RenderPassDescriptor::new();
        if layers > 1 {
            rp.setRenderTargetArrayLength(layers);
        }
        let ca = unsafe { rp.colorAttachments().objectAtIndexedSubscript(0) };
        ca.setTexture(Some(tex));
        ca.setLevel(level);
        if tex.textureType() == MTLTextureType::Type3D {
            ca.setDepthPlane(slice);
        } else {
            ca.setSlice(slice);
        }
        ca.setLoadAction(MTLLoadAction::Clear);
        ca.setStoreAction(MTLStoreAction::Store);
        ca.setClearColor(MTLClearColor {
            red: color[0] as f64,
            green: color[1] as f64,
            blue: color[2] as f64,
            alpha: color[3] as f64,
        });
        stats::inc(&stats::c::clear_pass);
        self.util_render_pass(&rp, &[resource], pt::kind::CLEAR)
            .ok_or("render encoder")?;
        if let Some(m) = self.pt_cur() {
            (m.w, m.h) = ((tex.width() >> level).max(1) as u32, (tex.height() >> level).max(1) as u32);
            m.shader = pt::synth::CLEAR_PASS;
        }
        self.end_encoder();
        Ok(())
    }

    /// One render pass clearing `v`'s first slice (depth and/or stencil).
    pub(super) unsafe fn encode_dsv_clear_pass(
        &mut self,
        v: &TargetView,
        depth: Option<f32>,
        stencil: Option<u8>,
    ) -> Result<(), String> {
        let (tex, level, slice) =
            Self::target_texture(v).ok_or("DSV does not reference a texture")?;
        self.dsv_clear_slice(&tex, level, slice, 1, depth, stencil, v.resource)
    }

    /// Clear `layers` slices starting at `slice` in one (layered) render pass.
    #[allow(clippy::too_many_arguments)]
    fn dsv_clear_slice(
        &mut self,
        tex: &MtlTexture,
        level: usize,
        slice: usize,
        layers: usize,
        depth: Option<f32>,
        stencil: Option<u8>,
        resource: usize,
    ) -> Result<(), String> {
        let pf = tex.pixelFormat();
        let has_depth = pf != MTLPixelFormat::X32_Stencil8 && pf != MTLPixelFormat::Stencil8;
        let has_stencil = matches!(
            pf,
            MTLPixelFormat::Depth32Float_Stencil8
                | MTLPixelFormat::X32_Stencil8
                | MTLPixelFormat::Stencil8
        );
        let rp = MTL4RenderPassDescriptor::new();
        if layers > 1 {
            rp.setRenderTargetArrayLength(layers);
        }
        if has_depth {
            let da = rp.depthAttachment();
            da.setTexture(Some(tex));
            da.setLevel(level);
            da.setSlice(slice);
            da.setLoadAction(if depth.is_some() {
                MTLLoadAction::Clear
            } else {
                MTLLoadAction::Load
            });
            da.setStoreAction(MTLStoreAction::Store);
            da.setClearDepth(depth.unwrap_or(0.0) as f64);
        }
        if has_stencil {
            let sa = rp.stencilAttachment();
            sa.setTexture(Some(tex));
            sa.setLevel(level);
            sa.setSlice(slice);
            sa.setLoadAction(if stencil.is_some() {
                MTLLoadAction::Clear
            } else {
                MTLLoadAction::Load
            });
            sa.setStoreAction(MTLStoreAction::Store);
            sa.setClearStencil(stencil.unwrap_or(0) as u32);
        }
        stats::inc(&stats::c::clear_pass);
        self.util_render_pass(&rp, &[resource], pt::kind::CLEAR)
            .ok_or("render encoder")?;
        if let Some(m) = self.pt_cur() {
            (m.w, m.h) = ((tex.width() >> level).max(1) as u32, (tex.height() >> level).max(1) as u32);
            m.shader = pt::synth::CLEAR_PASS;
        }
        self.end_encoder();
        Ok(())
    }

    /// Resolve an RTV/DSV record to (texture, level, slice).
    ///
    /// # Safety
    /// `v.resource` must point at a live [`Resource`].
    pub unsafe fn target_texture(v: &TargetView) -> Option<(Retained<MtlTexture>, usize, usize)> {
        let r = &*(v.resource as *const Resource);
        let fmt = MTLPixelFormat(v.format as usize);
        let t = r.texture()?;
        if fmt == r.mtl_format || fmt == MTLPixelFormat::Invalid {
            Some((t.retain(), v.mip as usize, v.first_slice as usize))
        } else {
            let view = r.texture_view(
                fmt,
                t.textureType(),
                (0, t.mipmapLevelCount() as u32),
                (0, (t.arrayLength() as u32).max(1)),
            )?;
            Some((view, v.mip as usize, v.first_slice as usize))
        }
    }

    /// `ClearRenderTargetView` (supports multi-slice views and scissor rect clears).
    ///
    /// # Safety
    /// See [`Self::target_texture`].
    pub unsafe fn clear_rtv(
        &mut self,
        v: &TargetView,
        color: [f32; 4],
        rects: &[Rect],
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        self.disc_unmark(v.resource);
        let (tex, level, first_slice) =
            Self::target_texture(v).ok_or("RTV does not reference a texture")?;
        let slices = v.slices.max(1) as usize;
        if rects.is_empty() && slices == 1 && !legacy_barriers() {
            return self.defer_clear(*v, ClearKind::Color(color));
        }
        if rects.is_empty() && !legacy_barriers() {
            if let Some(item) = Self::compute_clear_item(v, color, slices as u32) {
                let res = v.resource;
                self.flush_clears(&stats::c::cf_rect, |p| p.view.resource == res)?;
                return self.compute_clear_batch(vec![item]);
            }
            if (self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active())
                && layered(&tex)
                && slices <= 64
            {
                for s in 0..slices {
                    let mut sv = *v;
                    sv.first_slice = (first_slice + s) as u32;
                    sv.slices = 1;
                    self.defer_clear(sv, ClearKind::Color(color))?;
                }
                self.bump_clear_coalesce_stat(1, true);
                return Ok(());
            }
        }
        let res = v.resource;
        self.flush_clears(&stats::c::cf_rect, |p| p.view.resource == res)?;
        if rects.is_empty() {
            if layered(&tex) {
                self.rtv_clear_slice(&tex, level, first_slice, slices, color, res)?;
            } else {
                for s in 0..slices {
                    self.rtv_clear_slice(&tex, level, first_slice + s, 1, color, res)?;
                }
            }
            return Ok(());
        }
        let mw = (tex.width() >> level).max(1);
        let mh = (tex.height() >> level).max(1);
        let sc_rects = clip_scissor_rects(rects, mw, mh);
        if sc_rects.is_empty() {
            return Ok(());
        }
        let pso = clear::rtv_rect_pipeline(tex.pixelFormat(), tex.sampleCount())?;
        let params = RectClearParams { color, depth: 0.0, _pad: [0; 3] };
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let pa = ring.lock().unwrap().upload(clear::bytes_of(&params), 256)?;
        let table = self.ensure_util_table()?;
        table.setAddress_atIndex(pa, 0);
        let vp = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: mw as f64,
            height: mh as f64,
            znear: 0.0,
            zfar: 1.0,
        };
        for s in 0..slices {
            let slice = first_slice + s;
            let rp = MTL4RenderPassDescriptor::new();
            let ca = rp.colorAttachments().objectAtIndexedSubscript(0);
            ca.setTexture(Some(&tex));
            ca.setLevel(level);
            if tex.textureType() == MTLTextureType::Type3D {
                ca.setDepthPlane(slice);
            } else {
                ca.setSlice(slice);
            }
            ca.setLoadAction(MTLLoadAction::Load);
            ca.setStoreAction(MTLStoreAction::Store);
            stats::inc(&stats::c::util_pass);
            let e = self
                .util_render_pass(&rp, &[res], pt::kind::UTIL)
                .ok_or("render encoder")?;
            if let Some(m) = self.pt_cur() {
                (m.w, m.h) = (mw as u32, mh as u32);
                m.draws += sc_rects.len() as u32;
                m.shader = pt::synth::CLEAR_PASS;
            }
            e.setArgumentTable_atStages(
                &table,
                MTLRenderStages::Vertex | MTLRenderStages::Fragment,
            );
            e.setRenderPipelineState(&pso);
            e.setViewport(vp);
            for &sc in &sc_rects {
                e.setScissorRect(sc);
                e.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                    MTLPrimitiveType::Triangle,
                    0,
                    3,
                    1,
                    0,
                );
            }
            self.note_stage_ran(MTLStages::Vertex | MTLStages::Fragment);
            self.end_encoder();
        }
        Ok(())
    }

    /// `ClearDepthStencilView` (supports multi-slice views and scissor rect clears).
    ///
    /// # Safety
    /// See [`Self::target_texture`].
    pub unsafe fn clear_dsv(
        &mut self,
        v: &TargetView,
        clear_depth: bool,
        depth: f32,
        clear_stencil: bool,
        stencil: u8,
        rects: &[Rect],
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        self.disc_unmark(v.resource);
        let (tex, level, first_slice) =
            Self::target_texture(v).ok_or("DSV does not reference a texture")?;
        let pf = tex.pixelFormat();
        let has_depth = pf != MTLPixelFormat::X32_Stencil8 && pf != MTLPixelFormat::Stencil8;
        let has_stencil = matches!(
            pf,
            MTLPixelFormat::Depth32Float_Stencil8
                | MTLPixelFormat::X32_Stencil8
                | MTLPixelFormat::Stencil8
        );
        let clear_d = clear_depth && has_depth;
        let clear_s = clear_stencil && has_stencil;
        if !clear_d && !clear_s {
            return Ok(());
        }
        let slices = v.slices.max(1) as usize;
        let (d_val, s_val) = (clear_d.then_some(depth), clear_s.then_some(stencil));
        if rects.is_empty() && slices == 1 && !legacy_barriers() {
            return self.defer_clear(
                *v,
                ClearKind::DepthStencil { depth: d_val, stencil: s_val },
            );
        }
        if rects.is_empty()
            && !legacy_barriers()
            && (self.opts.opt_clear_pass_fold_v2 || self.clear_isolate_active())
            && layered(&tex)
            && slices <= 64
        {
            for s in 0..slices {
                let mut sv = *v;
                sv.first_slice = (first_slice + s) as u32;
                sv.slices = 1;
                self.defer_clear(
                    sv,
                    ClearKind::DepthStencil { depth: d_val, stencil: s_val },
                )?;
            }
            self.bump_clear_coalesce_stat(1, true);
            return Ok(());
        }
        let res = v.resource;
        self.flush_clears(&stats::c::cf_rect, |p| p.view.resource == res)?;
        if rects.is_empty() {
            if layered(&tex) {
                self.dsv_clear_slice(&tex, level, first_slice, slices, d_val, s_val, res)?;
            } else {
                for s in 0..slices {
                    self.dsv_clear_slice(&tex, level, first_slice + s, 1, d_val, s_val, res)?;
                }
            }
            return Ok(());
        }
        let mw = (tex.width() >> level).max(1);
        let mh = (tex.height() >> level).max(1);
        let sc_rects = clip_scissor_rects(rects, mw, mh);
        if sc_rects.is_empty() {
            return Ok(());
        }
        let (pso, dss) = clear::dsv_rect_pipeline(pf, tex.sampleCount(), clear_d, clear_s)?;
        let params = RectClearParams { color: [0.0; 4], depth: depth.clamp(0.0, 1.0), _pad: [0; 3] };
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let pa = ring.lock().unwrap().upload(clear::bytes_of(&params), 256)?;
        let table = self.ensure_util_table()?;
        table.setAddress_atIndex(pa, 0);
        let vp = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: mw as f64,
            height: mh as f64,
            znear: 0.0,
            zfar: 1.0,
        };
        for s in 0..slices {
            let slice = first_slice + s;
            let rp = MTL4RenderPassDescriptor::new();
            if has_depth {
                let da = rp.depthAttachment();
                da.setTexture(Some(&tex));
                da.setLevel(level);
                da.setSlice(slice);
                da.setLoadAction(MTLLoadAction::Load);
                da.setStoreAction(MTLStoreAction::Store);
            }
            if has_stencil {
                let sa = rp.stencilAttachment();
                sa.setTexture(Some(&tex));
                sa.setLevel(level);
                sa.setSlice(slice);
                sa.setLoadAction(MTLLoadAction::Load);
                sa.setStoreAction(MTLStoreAction::Store);
            }
            stats::inc(&stats::c::util_pass);
            let e = self
                .util_render_pass(&rp, &[res], pt::kind::UTIL)
                .ok_or("render encoder")?;
            if let Some(m) = self.pt_cur() {
                (m.w, m.h) = (mw as u32, mh as u32);
                m.draws += sc_rects.len() as u32;
                m.shader = pt::synth::CLEAR_PASS;
            }
            e.setArgumentTable_atStages(&table, MTLRenderStages::Vertex);
            e.setRenderPipelineState(&pso);
            e.setDepthStencilState(Some(&dss));
            if clear_s {
                e.setStencilReferenceValue(stencil as u32);
            }
            e.setViewport(vp);
            for &sc in &sc_rects {
                e.setScissorRect(sc);
                e.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                    MTLPrimitiveType::Triangle,
                    0,
                    3,
                    1,
                    0,
                );
            }
            self.note_stage_ran(MTLStages::Vertex | MTLStages::Fragment);
            self.end_encoder();
        }
        Ok(())
    }

    /// `ClearUnorderedAccessView{Uint,Float}` of the UAV `desc` (the 40-byte
    /// `D3D12_UNORDERED_ACCESS_VIEW_DESC` the descriptor was created with,
    /// `None` = whole resource). `rects` empty = whole view; for buffers a
    /// rect's left/right select an element range.
    pub fn clear_uav(
        &mut self,
        r: &Resource,
        desc: Option<&[u8; 40]>,
        v: ClearValue,
        rects: &[Rect],
    ) -> Result<(), String> {
        self.clear_uav_with(r, desc, v, rects, barrier_tight_stages())
    }

    /// Resolve a tracked CPU UAV descriptor handle via [`resolve_uav_for_clear`]
    /// and encode [`Self::clear_uav`].
    pub fn clear_uav_handle(
        &mut self,
        cpu: usize,
        r: &Resource,
        v: ClearValue,
        rects: &[Rect],
    ) -> Result<(), String> {
        match resolve_uav_for_clear(cpu, r, "ClearUnorderedAccessView") {
            UavClearLookup::Found(desc) => self.clear_uav(r, desc.as_ref(), v, rects),
            UavClearLookup::MissingBufferView => Err(format!(
                "CPU handle {cpu:#x} holds no UAV created by CreateUnorderedAccessView"
            )),
        }
    }

    pub(super) fn clear_uav_with(
        &mut self,
        r: &Resource,
        desc: Option<&[u8; 40]>,
        v: ClearValue,
        rects: &[Rect],
        tight_stages: bool,
    ) -> Result<(), String> {
        self.flush_icb_prepass_if_pending();
        r.register_ptr();
        r.mark_gpu_used();
        self.disc_unmark(r as *const Resource as usize);
        self.flush_clears_for(&[r as *const Resource as usize])?;
        let u32s = |d: &[u8], i: usize| u32::from_le_bytes(d[i * 4..i * 4 + 4].try_into().unwrap());
        let (dxgi, dimension) = match desc {
            Some(d) => (u32s(d, 0), u32s(d, 1)),
            None if r.desc.is_buffer() => return Err("buffer UAV clear without a view desc".into()),
            None => (0, 0),
        };
        if r.desc.is_buffer() {
            if dimension != 1 {
                return Err(format!("UAV clear: buffer with view dimension {dimension}"));
            }
            let u = &desc.unwrap()[8..];
            let first = u64::from_le_bytes(u[0..8].try_into().unwrap());
            let (count, stride, flags) = (u32s(u, 2) as u64, u32s(u, 3), u32s(u, 6));
            // (element bytes, dword pattern, period)
            let (bpe, (pattern, period)) = if flags & 1 != 0 || stride != 0 {
                // Raw / structured: the first value goes to every dword.
                let ClearValue::Uint(x) = v else {
                    return Err("float clear of a raw/structured buffer UAV".into());
                };
                (if stride != 0 { stride as u64 } else { 4 }, ([x[0]; 4], 1))
            } else {
                let fi = crate::format::info(dxgi);
                let e = clear::encode_element(dxgi, v)
                    .ok_or_else(|| format!("UAV clear of typed buffer format {}", fi.name))?;
                (
                    e.len() as u64,
                    clear::dword_pattern(&e)
                        .ok_or_else(|| format!("UAV clear of typed buffer format {}", fi.name))?,
                )
            };
            let ranges: Vec<(u64, u64)> = if rects.is_empty() {
                vec![(0, count)]
            } else {
                rects
                    .iter()
                    .map(|rc| (rc.left.max(0) as u64, (rc.right.max(0) as u64).min(count)))
                    .filter(|(a, b)| a < b)
                    .collect()
            };
            let buf_len = r.desc.Width;
            for (a, b) in ranges {
                let (off, len) = ((first + a) * bpe, (b - a) * bpe);
                if off % 4 != 0 || len % 4 != 0 || off + len > buf_len {
                    return Err(format!(
                        "UAV clear of bytes {off}+{len} of a {buf_len}-byte buffer (needs dword alignment, in bounds)"
                    ));
                }
                let words = (len / 4) as u32;
                if words == 0 {
                    continue;
                }
                let dst_va = r.gpu_va + off;
                let p = FillParams { dst: dst_va, words, period, pattern };
                if self.opts.opt_clear_cs_tile4 && dst_va % 16 == 0 && words >= 64 {
                    stats::inc(&stats::c::clear_cs_tile4);
                    self.util_dispatch_with(
                        "port_fill_v4",
                        None,
                        None,
                        clear::bytes_of(&p),
                        [words.div_ceil(4), 1, 1],
                        &[r as *const Resource as usize],
                        tight_stages,
                    )?;
                } else {
                    self.util_dispatch_with(
                        "port_fill",
                        None,
                        None,
                        clear::bytes_of(&p),
                        [words, 1, 1],
                        &[r as *const Resource as usize],
                        tight_stages,
                    )?;
                }
            }
            return Ok(());
        }
        let t = r.texture().ok_or("UAV clear: resource has no texture")?;
        use MTLTextureType as T;
        let full = (r.desc.Width as u32, r.desc.Height.max(1));
        // (view type, mip, slices, z range within the view)
        let (ty, mip, slices, z) = match (desc, dimension) {
            (None, _) => (t.textureType(), 0, (0, r.desc.array_size()), None),
            (Some(d), _) => {
                let u = &d[8..];
                match dimension {
                    2 => (T::Type1D, u32s(u, 0), (0, 1), None),
                    3 => (T::Type1DArray, u32s(u, 0), (u32s(u, 1), u32s(u, 2)), None),
                    4 => (T::Type2D, u32s(u, 0), (0, 1), None),
                    5 => (T::Type2DArray, u32s(u, 0), (u32s(u, 1), u32s(u, 2)), None),
                    // Metal views cannot select a W range: whole mip, z range in the kernel.
                    8 => (T::Type3D, u32s(u, 0), (0, 1), Some((u32s(u, 1), u32s(u, 2)))),
                    _ => return Err(format!("UAV clear: view dimension {dimension}")),
                }
            }
        };
        let view_dxgi = if dxgi != 0 { dxgi } else { r.desc.Format };
        let fmt = crate::format::view_format(view_dxgi, r.mtl_format);
        let kind = clear::texture_kind(ty).ok_or_else(|| format!("UAV clear of texture type {ty:?}"))?;
        if let Some(elem) = clear::raw_texture_uint_clear_element(view_dxgi, v) {
            return self.clear_uav_texture_raw_bytes(r, t, ty, mip, slices, z, rects, &elem, tight_stages);
        }
        let (class, uv, fv) = clear::texture_value(view_dxgi, v)?;
        let view = r.texture_view(fmt, ty, (mip, 1), slices).ok_or("UAV clear: texture view")?;
        let (w, h) = (
            (full.0 >> mip).max(1),
            if ty == T::Type1D || ty == T::Type1DArray { 1 } else { (full.1 >> mip).max(1) },
        );
        let (z0, d) = match z {
            Some((first, size)) => {
                let depth = (r.desc.depth() >> mip).max(1);
                let first = first.min(depth);
                (first, if size == u32::MAX { depth - first } else { size.min(depth - first) })
            }
            None if ty == T::Type3D => (0, (r.desc.depth() >> mip).max(1)),
            None => (0, view.arrayLength() as u32),
        };
        let whole = [Rect { left: 0, top: 0, right: w as i32, bottom: h as i32 }];
        let name = format!("port_clear_{kind}_{}", clear::class_suffix(class));
        let name_t4 = format!("{name}_t4");
        for rc in if rects.is_empty() { &whole[..] } else { rects } {
            let (x0, y0) = (rc.left.clamp(0, w as i32) as u32, rc.top.clamp(0, h as i32) as u32);
            let (x1, y1) = (rc.right.clamp(0, w as i32) as u32, rc.bottom.clamp(0, h as i32) as u32);
            if x1 <= x0 || y1 <= y0 || d == 0 {
                continue;
            }
            let p = TexParams { u: uv, f: fv, x0, y0, z0, w: x1 - x0, h: y1 - y0, d, _pad: [0; 2] };
            if self.opts.opt_clear_cs_tile4 && matches!(kind, "2d" | "2da") && p.w >= 4 && p.h >= 4 {
                stats::inc(&stats::c::clear_cs_tile4);
                self.util_dispatch_with(
                    &name_t4,
                    Some(&view),
                    None,
                    clear::bytes_of(&p),
                    [p.w.div_ceil(2), p.h.div_ceil(2), p.d],
                    &[r as *const Resource as usize],
                    tight_stages,
                )?;
            } else {
                self.util_dispatch_with(
                    &name,
                    Some(&view),
                    None,
                    clear::bytes_of(&p),
                    [p.w, p.h, p.d],
                    &[r as *const Resource as usize],
                    tight_stages,
                )?;
            }
        }
        Ok(())
    }

    /// Bit-preserving texture UAV clear via `copyFromBuffer` from the command
    /// allocator's upload ring. Used when `ClearUnorderedAccessViewUint` on a
    /// floating-point texture carries a NaN channel payload that GPU float
    /// texture-write instructions would canonicalize.
    #[allow(clippy::too_many_arguments)]
    fn clear_uav_texture_raw_bytes(
        &mut self,
        r: &Resource,
        t: &MtlTexture,
        ty: MTLTextureType,
        mip: u32,
        slices: (u32, u32),
        z: Option<(u32, u32)>,
        rects: &[Rect],
        elem: &[u8],
        tight_stages: bool,
    ) -> Result<(), String> {
        use MTLTextureType as T;
        let bpp = elem.len();
        let base_bpp = crate::format::info(r.desc.Format).bytes as usize;
        if bpp == 0 || (base_bpp != 0 && base_bpp != bpp) {
            return Err(format!(
                "UAV raw clear: element size {bpp} does not match texture format {}",
                crate::format::name(r.desc.Format)
            ));
        }
        if mip >= t.mipmapLevelCount() as u32 {
            return Err(format!("UAV clear: mip {mip} out of bounds"));
        }
        let base_ty = t.textureType();
        let ty_ok = match ty {
            T::Type1D | T::Type1DArray => matches!(base_ty, T::Type1D | T::Type1DArray),
            T::Type2D | T::Type2DArray => matches!(base_ty, T::Type2D | T::Type2DArray),
            T::Type3D => base_ty == T::Type3D,
            _ => false,
        };
        if !ty_ok {
            return Err(format!("UAV clear: incompatible view type {ty:?} for {base_ty:?}"));
        }
        let is_1d = matches!(base_ty, T::Type1D | T::Type1DArray);
        let full = (r.desc.Width as u32, r.desc.Height.max(1));
        let (w, h) = ((full.0 >> mip).max(1), if is_1d { 1 } else { (full.1 >> mip).max(1) });
        let arr_len = (t.arrayLength() as u32).max(1);
        let (first_slice, slice_count, z0, depth) = if ty == T::Type3D {
            let td = (r.desc.depth() >> mip).max(1);
            let (z0, d) = match z {
                Some((first, size)) => {
                    let first = first.min(td);
                    (first, if size == u32::MAX { td - first } else { size.min(td - first) })
                }
                None => (0, td),
            };
            (0u32, 1u32, z0, d)
        } else {
            if slices.0 >= arr_len {
                return Err(format!("UAV clear: first slice {} >= array length {arr_len}", slices.0));
            }
            let max_rem = arr_len - slices.0;
            let count = if slices.1 == u32::MAX {
                max_rem
            } else if slices.1 > max_rem {
                return Err(format!(
                    "UAV clear: slice range {}..{} exceeds array length {arr_len}",
                    slices.0,
                    slices.0.saturating_add(slices.1)
                ));
            } else {
                slices.1
            };
            (slices.0, count, 0u32, 1u32)
        };
        if slice_count == 0 || depth == 0 {
            return Ok(());
        }
        let whole = [Rect { left: 0, top: 0, right: w as i32, bottom: h as i32 }];
        let mut clipped: Vec<(u32, u32, u32, u32)> = Vec::new();
        for rc in if rects.is_empty() { &whole[..] } else { rects } {
            let x0 = rc.left.clamp(0, w as i32) as u32;
            let y0 = if is_1d { 0 } else { rc.top.clamp(0, h as i32) as u32 };
            let x1 = rc.right.clamp(0, w as i32) as u32;
            let y1 = if is_1d { 1 } else { rc.bottom.clamp(0, h as i32) as u32 };
            if x1 > x0 && y1 > y0 {
                clipped.push((x0, y0, x1 - x0, y1 - y0));
            }
        }
        if clipped.is_empty() {
            return Ok(());
        }

        let ring = self.ring.clone().ok_or("command list is not open")?;
        let res_key = r as *const Resource as usize;
        let touch = [res_key];
        let masked = tight_stages && !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        if masked {
            const UTIL_AFTER: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            self.add_pend(UTIL_AFTER, MTLStages::Blit);
            if ib_tight {
                self.ib_note_touch_before_tight(res_key, MTLStages::Blit);
            } else {
                self.ib_note(res_key);
            }
            stats::inc(&stats::c::qb_upscale_masked);
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        let e = if ib_tight && masked {
            self.compute_op_tight(MTLStages::Blit, Some(&touch), Some(&[]), Some(&touch))
        } else {
            self.compute_touch(if masked { Some(&touch) } else { None })
        }
        .ok_or("compute encoder")?;
        self.pt_note_util(pt::synth::CLEAR_UAV, 0);

        const STRIP_BUDGET: usize = 64 * 1024;
        let row_align = if 256 % bpp == 0 { 256 } else { bpp };
        for (x0, y0, rw, rh) in clipped {
            let row_payload = rw as usize * bpp;
            let row_bytes = if is_1d { row_payload } else { row_payload.next_multiple_of(row_align) };
            let strip_h = if is_1d {
                1
            } else {
                (STRIP_BUDGET / row_bytes.max(1)).clamp(1, rh as usize)
            };
            let strip_bytes = row_bytes * strip_h;
            let (src_buf, src_off) = ring.lock().unwrap().alloc_buffer(strip_bytes, 256)?;
            unsafe {
                let base = (src_buf.contents().as_ptr() as *mut u8).add(src_off);
                for px in 0..rw as usize {
                    std::ptr::copy_nonoverlapping(elem.as_ptr(), base.add(px * bpp), bpp);
                }
                for row in 1..strip_h {
                    std::ptr::copy_nonoverlapping(base, base.add(row * row_bytes), row_payload);
                }
            }
            for s in 0..slice_count {
                for dz in 0..depth {
                    let mut ry = 0usize;
                    while ry < rh as usize {
                        let ch = strip_h.min(rh as usize - ry);
                        let (src_bpr, src_bpi) = if is_1d { (0, 0) } else { (row_bytes, row_bytes * ch) };
                        unsafe {
                            e.copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
                                &src_buf,
                                src_off,
                                src_bpr,
                                src_bpi,
                                MTLSize { width: rw as usize, height: ch, depth: 1 },
                                t,
                                (first_slice + s) as usize,
                                mip as usize,
                                MTLOrigin {
                                    x: x0 as usize,
                                    y: if is_1d { 0 } else { y0 as usize + ry },
                                    z: (z0 + dz) as usize,
                                },
                                MTLBlitOption::None,
                            );
                        }
                        ry += ch;
                    }
                }
            }
        }

        self.note_stage_ran(MTLStages::Blit);
        if masked {
            const UTIL_BEFORE: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            self.add_pend(MTLStages::Blit, UTIL_BEFORE);
            if ib_tight {
                self.ib_add_obl_tight(res_key, MTLStages::Blit, COMPUTE_STAGES);
            } else {
                self.ib_note(res_key);
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        Ok(())
    }

    /// One dispatch of a utility kernel with its own argument table (params
    /// at buffer 0, optional texture 0 and texture 1); the list's table is restored after.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn util_dispatch_with(
        &mut self,
        kernel: &str,
        tex0: Option<&MtlTexture>,
        tex1: Option<&MtlTexture>,
        params: &[u8],
        threads: [u32; 3],
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<(), String> {
        self.util_dispatch_3tex_with(kernel, tex0, tex1, None, params, threads, touch, tight_stages)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn util_dispatch_3tex_with(
        &mut self,
        kernel: &str,
        tex0: Option<&MtlTexture>,
        tex1: Option<&MtlTexture>,
        tex2: Option<&MtlTexture>,
        params: &[u8],
        threads: [u32; 3],
        touch: &[usize],
        tight_stages: bool,
    ) -> Result<(), String> {
        let (pso, tg) = clear::kernel_for_dispatch(kernel, self.opts.opt_fast_bind_pso)?;
        let ring = self.ring.clone().ok_or("command list is not open")?;
        let pa = ring.lock().unwrap().upload(params, 256)?;
        self.ensure_util_table()?;
        let masked = tight_stages && !legacy_barriers() && barrier_masks();
        let ib_tight = self.ib_tight_active();
        let synth_key = if kernel.starts_with("port_upscale") {
            pt::synth::UPSCALE
        } else {
            pt::synth::CLEAR_UAV
        };
        if masked {
            // Prior same-state UAV/attachment writes to the target/source must
            // complete before this utility dispatch runs, without escalating
            // to an All -> All queue barrier (`qb_clear_uav`).
            const UTIL_AFTER: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            self.add_pend(UTIL_AFTER, MTLStages::Dispatch);
            if ib_tight {
                for &r in touch {
                    self.ib_note_touch_before_tight(r, MTLStages::Dispatch);
                }
            } else {
                for &r in touch {
                    self.ib_note(r);
                }
            }
            stats::inc(&stats::c::qb_upscale_masked);
        } else {
            // Clears are ordered against earlier and later work like any UAV write.
            self.need_full(FullWhy::ClearUav);
        }
        let e = if ib_tight && masked {
            let uav_slice = &touch[..1.min(touch.len())];
            let read_slice = if touch.len() > 1 { &touch[1..] } else { &[] };
            self.compute_op_tight(
                MTLStages::Dispatch,
                Some(uav_slice),
                Some(read_slice),
                Some(touch),
            )
        } else {
            self.compute_touch(if masked { Some(touch) } else { None })
        }
        .ok_or("compute encoder")?;
        self.pt_note_util(synth_key, 1);
        let t = self.util_table.as_ref().unwrap();
        unsafe {
            t.setAddress_atIndex(pa, 0);
            if let Some(tex) = tex0 {
                t.setTexture_atIndex(tex.gpuResourceID(), 0);
            }
            if let Some(tex) = tex1 {
                t.setTexture_atIndex(tex.gpuResourceID(), 1);
            }
            if let Some(tex) = tex2 {
                t.setTexture_atIndex(tex.gpuResourceID(), 2);
            }
        }
        e.setArgumentTable(Some(t));
        e.setComputePipelineState(&pso);
        e.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize {
                width: (threads[0] as usize).div_ceil(tg[0] as usize),
                height: (threads[1] as usize).div_ceil(tg[1] as usize),
                depth: (threads[2] as usize).div_ceil(tg[2] as usize),
            },
            MTLSize { width: tg[0] as usize, height: tg[1] as usize, depth: tg[2] as usize },
        );
        self.note_stage_ran(MTLStages::Dispatch);
        e.setArgumentTable(Some(&self.table));
        self.compute_pso_bound = false;
        if masked {
            // Order subsequent UAV/copy/attachment work on the touched resources
            // (even without an intervening ResourceBarrier for same-state UAVs).
            const UTIL_BEFORE: MTLStages = PRE_RASTER
                .union(MTLStages::Fragment)
                .union(MTLStages::Dispatch)
                .union(MTLStages::Blit)
                .union(MTLStages::AccelerationStructure);
            if ib_tight
                && self.opts.opt_cs_mid_qb_res_defer_v5
                && !touch.is_empty()
                && touch.iter().all(|&r| r != 0)
            {
                if self.opts.opt_cs_to_ps_overlap_v2 && !kernel.starts_with("port_upscale") {
                    self.add_pend_with_soft_res(
                        MTLStages::Dispatch,
                        MTLStages::Fragment,
                        &touch[..1],
                    );
                } else {
                    self.add_pend(MTLStages::Dispatch, PRE_RASTER | MTLStages::Fragment);
                }
                for &r in touch {
                    self.ib_add_obl_tight(r, MTLStages::Dispatch, COMPUTE_STAGES);
                    self.bar.cs_end_res.push((
                        r,
                        MTLStages::Dispatch,
                        MTLStages::Dispatch,
                        MTLStages::Dispatch,
                    ));
                }
                stats::inc(&stats::c::cs_mid_qb_v5);
            } else {
                if (self.opts.opt_cs_to_ps_overlap || self.opts.opt_cs_to_ps_overlap_v2)
                    && !kernel.starts_with("port_upscale")
                    && !touch.is_empty()
                {
                    self.add_pend_with_soft_res(
                        MTLStages::Dispatch,
                        UTIL_BEFORE - PRE_RASTER,
                        &touch[..1],
                    );
                } else {
                    self.add_pend(MTLStages::Dispatch, UTIL_BEFORE);
                }
                if ib_tight {
                    for &r in touch {
                        self.ib_add_obl_tight(r, MTLStages::Dispatch, COMPUTE_STAGES);
                    }
                } else {
                    for &r in touch {
                        self.ib_note(r);
                    }
                }
            }
        } else {
            self.need_full(FullWhy::ClearUav);
        }
        Ok(())
    }
}
